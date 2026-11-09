//! `worklist`: the work list of a search of the whole record, written as one directory per
//! agent, and the check of the reports the agents fill there.
//!
//! The clusters and the packing are `path@query@src/graph.rs`'s; this module writes them out and
//! reads them back. The files are for the searcher and for `--verify`, not a contract with any
//! other reader, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

use crate::graph::{self, Graph, Role};
use crate::model::table::{Anchors, Candidate, Entities};
use crate::output::outln;
use crate::Manifest;

/// The file listing every agent's entries, one per line.
const AGENTS_FILE: &str = "agents.txt";
/// The file holding an agent's entries whole.
const ENTRIES_FILE: &str = "entries.md";
/// The skeleton an agent fills.
const REPORT_FILE: &str = "report.md";
/// The show text of every entry of the record, for `--verify` to assemble from; no agent is
/// given it, so an agent's load is its entries file alone.
const SHOWN_FILE: &str = "shown.md";
/// The work of the search and its subjects, for the searchers and `--verify`: a line
/// `work <path>` per file of the work, its absolute path, then a line `subject <id>` per subject.
const WORK_FILE: &str = "work.txt";
/// The one subject of a work for which no subject is named.
const WHOLE_WORK: &str = "whole-work";
/// The assembly a passing `--verify` writes: every entry whose highest grade is `bears`.
const BEARS_FILE: &str = "bears.md";
/// The assembly a passing `--verify` writes: every entry whose highest grade is `may-bear`.
const MAY_BEAR_FILE: &str = "may-bear.md";
/// The section every report ends with.
const MET_OUTSIDE: &str = "## Met outside the task";
/// A report line that names an entry graded under an earlier cluster of the same agent.
const GRADED_ABOVE: &str = "graded above";
/// The mark of a seed that is itself an entry of the agent, and of an entry that cites a seed.
const SEED_MARK: &str = "seed";
const CITES_SEED_MARK: &str = "cites-seed";

/// What `worklist` does. One value, so that writing a work list and checking the reports cannot be
/// asked together, and neither can a bound or a seed be given to the check.
#[derive(Clone, Debug)]
pub enum WorklistMode {
    /// Write the work list into `dir`, a new or empty directory outside the project.
    Write {
        /// Where the work list goes.
        dir: std::path::PathBuf,
        /// The words one agent is given at most, the work's words included; a group above what
        /// the work leaves goes to an agent alone.
        bound: usize,
        /// The references the work names.
        seeds: Vec<String>,
        /// The files of the work, which every agent reads beside its entries.
        work: Vec<PathBuf>,
        /// The subjects of the work, each an id in the grammar of an entry id; none names the one
        /// subject `whole-work`.
        subjects: Vec<String>,
    },
    /// Check the reports the agents filled in `dir`, a directory `Write` wrote.
    Verify {
        /// The directory the work list was written into.
        dir: std::path::PathBuf,
    },
}

/// Write the work list of a search of the whole record, or check the reports its agents filled, per
/// `design@query@worklist-writes-and-verifies-the-reports`. `Err` is a refusal or a failure: under
/// `WorklistMode::Write`, an incomplete model, a directory inside the project or not empty, a seed
/// naming no entry, or a file of the work it cannot read, nothing written, and a write that failed, the files before it left in place.
/// Exit 1 is a finding on a report.
pub fn worklist(
    manifest: &Manifest,
    mode: &WorklistMode,
    checker: &[&Path],
) -> Result<ExitCode, String> {
    match mode {
        WorklistMode::Write {
            dir,
            bound,
            seeds,
            work,
            subjects,
        } => write(manifest, dir, *bound, seeds, work, subjects, checker),
        WorklistMode::Verify { dir } => verify(dir),
    }
}

/// Write the work list into `dir`.
///
/// Every refusal comes before the first write, so a refused run leaves `dir` as it found it: an
/// incomplete model, per `design@check@phases-gate-the-report`, a `dir` the walk would read or that
/// holds anything, a seed that names no entry of the record, and a file of the work it cannot read.
fn write(
    manifest: &Manifest,
    dir: &Path,
    bound: usize,
    named: &[String],
    work: &[PathBuf],
    subjects: &[String],
    checker: &[&Path],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    crate::model::foundation::complete_working_tree(manifest, &model)?;
    let dir = absolute(dir)?;
    let root = manifest
        .root()
        .canonicalize()
        .unwrap_or_else(|_| manifest.root().to_path_buf());
    if dir.starts_with(&root) || dir.starts_with(manifest.root()) {
        return Err(format!(
            "{} is inside the project, whose walk would read the files written there: give a \
             directory outside {}. Nothing was written.",
            dir.display(),
            root.display()
        ));
    }
    if dir.exists() {
        let empty = std::fs::read_dir(&dir)
            .map_err(|e| format!("{}: {e}. Nothing was written.", dir.display()))?
            .next()
            .is_none();
        if !empty {
            return Err(format!(
                "{} exists and is not empty: give a new or empty directory. Nothing was written.",
                dir.display()
            ));
        }
    }

    let anchors = Anchors::of(manifest, model.listing());
    let entities = Entities::build(&model, &anchors);
    let mut graph = graph::graph(&model, &anchors, &entities);
    let index: BTreeMap<String, usize> = graph
        .entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.reference(), i))
        .collect();
    let mut seeds: BTreeSet<usize> = BTreeSet::new();
    for seed in named {
        let reference = seed.trim_matches('`');
        let found = match crate::model::table::candidate(reference, &anchors) {
            Candidate::Reference { .. } => index.get(reference),
            _ => None,
        };
        match found {
            Some(&i) => {
                seeds.insert(i);
            }
            None => {
                return Err(format!(
                    "the seed {reference} names no entry of the record: give the reference of a \
                     goal, a design entry or another entry `{} show` prints. Nothing was written.",
                    manifest.command()
                ))
            }
        }
    }

    // Every agent reads the work beside its entries, so its words come out of every agent's
    // bound. A work longer than the bound leaves none, and every group then goes to an agent alone.
    let mut work_paths: Vec<PathBuf> = Vec::new();
    let mut work_words = 0;
    for path in work {
        let text = std::fs::read_to_string(path).map_err(|e| {
            format!(
                "the file of the work {}: {e}. Nothing was written.",
                path.display()
            )
        })?;
        work_words += text.split_whitespace().count();
        work_paths.push(absolute(path)?);
    }
    let bound_left = bound.saturating_sub(work_words);

    let plan = graph::plan(&mut graph, bound_left);
    let agent_groups: Vec<Vec<usize>> = plan.agents.iter().map(|a| a.groups.clone()).collect();
    let mut files = render(&graph, &agent_groups, &seeds);
    files.push((
        PathBuf::from(SHOWN_FILE),
        snapshot(&graph, &anchors, &model),
    ));
    files.push((PathBuf::from(WORK_FILE), work_text(&work_paths, subjects)));

    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("{}: {e}. Nothing was written.", dir.display()))?;
    for (rel, text) in &files {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    }

    if work_words > 0 {
        outln!("the work holds {work_words} words, which leaves {bound_left} of the {bound}-word bound");
    }
    for (n, agent) in plan.agents.iter().enumerate() {
        // Each item carries the load it was packed by, a search unit packed whole or one group of
        // a split one, and their sum is the agent's summed load, which the packing keeps under the
        // bound; the agent's own load counts an entry or a title line two of its groups share once.
        let names: Vec<String> = agent
            .items
            .iter()
            .map(|(name, load)| format!("{name} ({load} words)"))
            .collect();
        outln!(
            "agent-{}  {} words  {}",
            n + 1,
            graph.load(&agent.groups),
            names.join("; ")
        );
    }
    // A group alone above the bound is named where its own entries exceed it, which a split of its
    // subject, or of its standing entries, reduces; one above it by its context entries and title
    // lines alone has nothing of its own to split.
    for &g in &plan.above {
        let group = &graph.groups[g];
        let own = graph.own_words(g);
        if own <= bound_left {
            continue;
        }
        match group.kind {
            graph::GroupKind::Cluster => outln!(
                "notice: the cluster \"{}\" holds {own} words, above the {bound_left}-word bound: \
                 split its subject into subjects read apart",
                group.name,
            ),
            graph::GroupKind::Remainder => outln!(
                "notice: the remainder of {} holds {own} words, above the {bound_left}-word bound: \
                 give its issues group subdirectories, or have them cite the heads they concern",
                group.unit,
            ),
            graph::GroupKind::Goals => outln!(
                "notice: the goals of \"{}\" hold {own} words, above the {bound_left}-word bound: \
                 split the goals home into subdocuments",
                group.name,
            ),
        }
    }
    outln!(
        "{} agents in {}; run `{} worklist --verify {}` on their reports",
        plan.agents.len(),
        dir.display(),
        manifest.command(),
        dir.display()
    );
    Ok(ExitCode::SUCCESS)
}

/// The directory as an absolute path, its `.` and `..` segments resolved without touching the
/// filesystem, since it may not exist yet.
fn absolute(dir: &Path) -> Result<PathBuf, String> {
    let joined = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| format!("cannot read the working directory: {e}"))?
            .join(dir)
    };
    let mut out = PathBuf::new();
    for part in joined.components() {
        match part {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    // The deepest existing ancestor is compared by its real path, so a link into the project, the
    // directory itself or any parent of it, is seen; the segments below it do not exist yet.
    let mut existing = out.clone();
    let mut rest: Vec<std::ffi::OsString> = Vec::new();
    while !existing.exists() {
        match (existing.file_name(), existing.parent()) {
            (Some(name), Some(parent)) => {
                rest.push(name.to_os_string());
                existing = parent.to_path_buf();
            }
            _ => break,
        }
    }
    let mut real = existing.canonicalize().unwrap_or(existing);
    for name in rest.into_iter().rev() {
        real.push(name);
    }
    Ok(real)
}

/// `work.txt`: a line `work <path>` per file of the work, then a line `subject <id>` per subject,
/// each once, in the order given, `whole-work` where none is given. The shape is this function's,
/// read by `--verify`, the searcher and the session alone, per
/// `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
fn work_text(work: &[PathBuf], subjects: &[String]) -> String {
    let mut out = String::new();
    for path in work {
        out.push_str(&format!("work {}\n", path.display()));
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let named: Vec<&str> = subjects
        .iter()
        .map(String::as_str)
        .filter(|s| seen.insert(s))
        .collect();
    for subject in if named.is_empty() {
        vec![WHOLE_WORK]
    } else {
        named
    } {
        out.push_str(&format!("subject {subject}\n"));
    }
    out
}

/// One entry whole, as `entries.md` gives it: a line `=== <reference>  <site>`, then its body in
/// a fence longer than any run of backticks it holds, so neither a line of it nor a heading of it
/// reads as the start of another entry or group.
fn entry_block(entry: &graph::Entry) -> String {
    let body = entry.record.body.trim_end();
    let fence = "`".repeat(longest_backtick_run(body).max(2) + 1);
    format!(
        "\n=== {}  {}\n\n{fence}markdown\n{body}\n{fence}\n",
        entry.reference(),
        entry.record.site
    )
}

/// Every file of the work list, by path under its directory.
///
/// Each agent's `entries.md` holds, group by group, the entries it grades; a goals file's group
/// opens with the file's prose above its first goal. A cluster agent's file then holds its context
/// entries whole, under `## Context entries`, and its title lines, under `## Goal titles`, each the
/// goal's reference and its title: both are read only, and its report names neither.
fn render(graph: &Graph, agents: &[Vec<usize>], seeds: &BTreeSet<usize>) -> Vec<(PathBuf, String)> {
    let mut list = String::new();
    let mut files = Vec::new();
    for (n, groups) in agents.iter().enumerate() {
        let agent = format!("agent-{}", n + 1);
        let mut entries = format!("# The entries of {agent}\n");
        let mut report = format!(
            "# The report of {agent}\n\n\
             Grade each entry: `bears` with a relation, `may-bear`, or `unrelated`. A reason is owed\n\
             for `bears` and `may-bear`.\n"
        );
        let mut given: BTreeSet<usize> = BTreeSet::new();
        for &g in groups {
            let group = &graph.groups[g];
            let heading = match group.kind {
                graph::GroupKind::Goals => "Goals",
                _ => "Cluster",
            };
            entries.push_str(&format!("\n## {heading}: {}\n", group.name));
            report.push_str(&format!("\n## {heading}: {}\n\n", group.name));
            let prose = group.prose.trim();
            if !prose.is_empty() {
                let fence = "`".repeat(longest_backtick_run(prose).max(2) + 1);
                entries.push_str(&format!(
                    "\nThe goals file above its first goal:\n\n{fence}markdown\n{prose}\n{fence}\n"
                ));
            }
            for &i in &group.entries {
                let entry = &graph.entries[i];
                let reference = entry.reference();
                if !given.insert(i) {
                    entries.push_str(&format!("\n=== {reference}  {GRADED_ABOVE}\n"));
                    report.push_str(&format!("- {reference} | {GRADED_ABOVE}\n"));
                    continue;
                }
                let cited: Vec<String> = graph
                    .cites
                    .get(&i)
                    .into_iter()
                    .flatten()
                    .filter(|c| seeds.contains(c))
                    .map(|&c| graph.entries[c].reference())
                    .collect();
                let mark = if seeds.contains(&i) {
                    format!(" {SEED_MARK}")
                } else if !cited.is_empty() {
                    format!(" {CITES_SEED_MARK}")
                } else {
                    String::new()
                };
                list.push_str(&format!(
                    "{agent} {} {reference}{mark}\n",
                    entry.role.word()
                ));
                entries.push_str(&entry_block(entry));
                let seed_field = if seeds.contains(&i) {
                    " | is a seed".to_string()
                } else if cited.is_empty() {
                    String::new()
                } else {
                    format!(" | cites seed: {}", cited.join(", "))
                };
                report.push_str(&format!(
                    "- {reference}{seed_field} | grade: | relation: | reason:\n"
                ));
            }
        }
        let context = graph.context(groups);
        if !context.is_empty() {
            entries.push_str(
                "\n## Context entries\n\nCited by an entry above, graded by another agent: read only, \
                 and named in no line of the report.\n",
            );
            for &i in &context {
                entries.push_str(&entry_block(&graph.entries[i]));
            }
        }
        let titles = graph.title_lines(groups);
        if !titles.is_empty() {
            entries.push_str(
                "\n## Goal titles\n\nGraded by a goal agent: read only, and named in no line of the \
                 report.\n\n",
            );
            for &i in &titles {
                let goal = &graph.entries[i];
                entries.push_str(&format!("- {}  {}\n", goal.reference(), goal.record.title));
            }
        }
        report.push_str(&format!("\n{MET_OUTSIDE}\n\n"));
        files.push((PathBuf::from(&agent).join(ENTRIES_FILE), entries));
        files.push((PathBuf::from(&agent).join(REPORT_FILE), report));
    }
    files.insert(0, (PathBuf::from(AGENTS_FILE), list));
    files
}

/// The snapshot: every entry of the record, in the graph's order, as a line `=== <reference>`, a
/// blank line, and the entry's show text in a fence longer than any run of backticks it holds.
///
/// The text is `show`'s own, from [`crate::show::record_text`], [`crate::show::cited_at`] and
/// [`crate::show::inbound_text`], over one [`crate::model::table::Inbound`] built for every entry at once.
/// `--verify` finds an entry by the reference on its `===` line, and reads its text up to the
/// line that is the fence alone, which no line of the text can be, since the fence is longer
/// than any run of backticks the text holds.
fn snapshot(graph: &Graph, anchors: &Anchors, model: &crate::Model) -> String {
    let index = crate::model::table::Inbound::build(model, anchors);
    let mut out = String::new();
    for entry in &graph.entries {
        let reference = entry.reference();
        let (kind, anchor, id) = &entry.key;
        let text = format!(
            "{}\n\n{}",
            crate::show::record_text(&reference, &entry.record),
            crate::show::inbound_text(&crate::show::cited_at(&index, anchors, kind, anchor, id))
        );
        let fence = "`".repeat(longest_backtick_run(&text).max(2) + 1);
        out.push_str(&format!(
            "=== {reference}\n\n{fence}text\n{text}\n{fence}\n\n"
        ));
    }
    out
}

/// The longest run of backticks in `text`.
fn longest_backtick_run(text: &str) -> usize {
    let (mut best, mut run) = (0, 0);
    for c in text.chars() {
        run = if c == '`' { run + 1 } else { 0 };
        best = best.max(run);
    }
    best
}

/// One line of `agents.txt`.
struct Listed {
    role: Role,
    seed: bool,
    cites_seed: bool,
}

/// The relations a `bears` grade takes for an entry of a role.
fn relations(role: Role) -> &'static [&'static str] {
    match role {
        Role::Unit | Role::Goal => &["constrains", "may change"],
        Role::Rejected => &["may bring back"],
        Role::Issue => &["touches", "closes", "worsens", "depends on", "trigger met"],
        Role::Tripwire => &["fires"],
    }
}

/// Check every report of the directory against `agents.txt`, and, when every report passes,
/// assemble the graded entries from the snapshot.
///
/// It reads no model: what each agent was given is what `agents.txt` says, and each entry's text is
/// what `--out` wrote into the snapshot, so the check and the assembly hold whatever the tree has
/// become since. A previous run's assembly is removed before anything is judged, so a run that
/// fails, or stops on a file it cannot read, leaves none a session could take for this one's.
fn verify(dir: &Path) -> Result<ExitCode, String> {
    for name in [BEARS_FILE, MAY_BEAR_FILE] {
        let path = dir.join(name);
        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    let list_path = dir.join(AGENTS_FILE);
    let text =
        std::fs::read_to_string(&list_path).map_err(|e| format!("{}: {e}", list_path.display()))?;
    let mut agents: BTreeMap<String, BTreeMap<String, Listed>> = BTreeMap::new();
    // The order of `agents.txt`, which the assembly keeps: the map above sorts `agent-10`
    // before `agent-2`.
    let mut order: Vec<(String, String)> = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        let role = parts.get(1).and_then(|w| role_of(w));
        let (Some(agent), Some(role), Some(reference)) = (parts.first(), role, parts.get(2)) else {
            return Err(format!(
                "{}:{}: not a line `<agent> <role> <reference>`",
                list_path.display(),
                n + 1
            ));
        };
        let mark = parts.get(3).copied();
        if parts.len() > 4 || !matches!(mark, None | Some(SEED_MARK) | Some(CITES_SEED_MARK)) {
            return Err(format!(
                "{}:{}: not a line `<agent> <role> <reference>`",
                list_path.display(),
                n + 1
            ));
        }
        order.push((agent.to_string(), reference.to_string()));
        agents.entry(agent.to_string()).or_default().insert(
            reference.to_string(),
            Listed {
                role,
                seed: mark == Some(SEED_MARK),
                cites_seed: mark == Some(CITES_SEED_MARK),
            },
        );
    }

    let mut findings: Vec<String> = Vec::new();
    let mut grades: BTreeMap<(String, String), Fields> = BTreeMap::new();
    for (agent, listed) in &agents {
        let rel = PathBuf::from(agent).join(REPORT_FILE);
        let path = dir.join(&rel);
        let report =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut graded: BTreeMap<String, usize> = BTreeMap::new();
        let mut met_outside = false;
        for (n, line) in report.lines().enumerate() {
            let at = format!("{}:{}", rel.display(), n + 1);
            if line.trim_end() == MET_OUTSIDE {
                met_outside = true;
                break;
            }
            let Some(rest) = line.strip_prefix("- ") else {
                continue;
            };
            let (reference, fields) = rest.split_once(" | ").unwrap_or((rest, ""));
            if fields == GRADED_ABOVE {
                if !listed.contains_key(reference) {
                    findings.push(format!("{at}  {reference} is not an entry of {agent}"));
                }
                continue;
            }
            let Some(parsed) = parse_fields(fields) else {
                return Err(format!(
                    "{}/{at}: not a line `- <reference> | grade: <grade> | relation: <relation> \
                     | reason: <reason>`",
                    dir.display()
                ));
            };
            *graded.entry(reference.to_string()).or_default() += 1;
            let Some(entry) = listed.get(reference) else {
                findings.push(format!("{at}  {reference} is not an entry of {agent}"));
                continue;
            };
            findings.extend(judge(&at, reference, entry, &parsed));
            grades.insert((agent.clone(), reference.to_string()), parsed);
        }
        if !met_outside {
            findings.push(format!(
                "{}  no section `{MET_OUTSIDE}`: end the report with it, empty where nothing was met",
                rel.display()
            ));
        }
        for reference in listed.keys() {
            match graded.get(reference) {
                None => findings.push(format!(
                    "{}  {reference} is given to {agent} and graded nowhere",
                    rel.display()
                )),
                Some(&count) if count > 1 => findings.push(format!(
                    "{}  {reference} is graded {count} times",
                    rel.display()
                )),
                _ => {}
            }
        }
    }
    for finding in &findings {
        outln!("{finding}");
    }
    if findings.is_empty() {
        let (bears, may_bear) = assemble(dir, &order, &agents, &grades)?;
        outln!(
            "assembly: {bears} entries in {}, {may_bear} in {}",
            dir.join(BEARS_FILE).display(),
            dir.join(MAY_BEAR_FILE).display()
        );
        outln!("PASSED: every report grades every entry it was given");
        Ok(ExitCode::SUCCESS)
    } else {
        outln!("FAILED: {} findings above", findings.len());
        Ok(ExitCode::FAILURE)
    }
}

/// Write the assembly from passing reports, and return how many entries each file holds.
///
/// Each entry goes under its highest grade over every report that grades it, `bears` above
/// `may-bear`, once: an entry many agents grade is a goal, which joins every cluster that reaches
/// it. Above its text, every agent's grade line at that grade or the other bearing one, identical
/// lines merged into one naming every agent that gave it, then its text, fenced. A goal is its
/// grade lines and its site alone: the design grounding reads every goals home whole, and the
/// other readers read a goal with `show`, per `design@query@graded-entries-assembled`. `may-bear.md` opens with a table of
/// contents, each entry with its reasons, from which the session picks the entries a proposal
/// touches. A snapshot that cannot be read, or that lacks an entry `agents.txt` lists, writes
/// nothing: the run could not do its whole job, so it stops with exit 2, per
/// `design@cli@exit-code-ladder`.
fn assemble(
    dir: &Path,
    order: &[(String, String)],
    agents: &BTreeMap<String, BTreeMap<String, Listed>>,
    grades: &BTreeMap<(String, String), Fields>,
) -> Result<(usize, usize), String> {
    let snapshot = read_snapshot(dir)?;
    let mut references: Vec<&str> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (_, reference) in order {
        if seen.insert(reference) {
            references.push(reference);
        }
    }
    let missing: Vec<&str> = references
        .iter()
        .copied()
        .filter(|r| !snapshot.contains_key(*r))
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "the reports pass, but {} lacks {}: run `--out` again into a new directory. No \
             assembly was written.",
            dir.join(SHOWN_FILE).display(),
            missing.join(", ")
        ));
    }

    let mut bears = "# The entries graded `bears`\n".to_string();
    let mut may_bear_contents = String::new();
    let mut may_bear_entries = String::new();
    let (mut n_bears, mut n_may_bear) = (0, 0);
    for reference in references {
        // Every agent's bearing grade of the entry, in the order of `agents.txt`.
        let mut lines: Vec<(String, &Fields)> = Vec::new();
        let mut goal = false;
        for (agent, r) in order.iter().filter(|(_, r)| r == reference) {
            goal |= agents
                .get(agent)
                .and_then(|l| l.get(r))
                .is_some_and(|l| l.role == Role::Goal);
            if let Some(fields) = grades.get(&(agent.clone(), r.clone())) {
                if matches!(fields.grade.as_str(), "bears" | "may-bear") {
                    lines.push((agent.clone(), fields));
                }
            }
        }
        if lines.is_empty() {
            continue;
        }
        let highest_bears = lines.iter().any(|(_, f)| f.grade == "bears");
        let mut merged: Vec<(String, Vec<String>)> = Vec::new();
        for (agent, f) in &lines {
            // The shape of the report's own line, an empty relation written `relation: |`.
            let relation = if f.relation.is_empty() {
                String::new()
            } else {
                format!(" {}", f.relation)
            };
            let line = format!(
                "grade: {} | relation:{relation} | reason: {}",
                f.grade, f.reason
            );
            match merged.iter_mut().find(|(l, _)| *l == line) {
                Some((_, who)) => who.push(agent.clone()),
                None => merged.push((line, vec![agent.clone()])),
            }
        }
        let text = &snapshot[reference];
        let shown = if goal {
            text.lines().next().unwrap_or_default().to_string()
        } else {
            text.clone()
        };
        let mut block = format!("\n=== {reference}\n");
        for (line, who) in &merged {
            block.push_str(&format!("{}: {line}\n", who.join(", ")));
        }
        // Fenced as the snapshot fences it, so an entry's own headings and any `=== ` line of its
        // body read as its text, not as the file's sections or as another entry.
        let fence = "`".repeat(longest_backtick_run(&shown).max(2) + 1);
        block.push_str(&format!("\n{fence}text\n{shown}\n{fence}\n"));
        if highest_bears {
            bears.push_str(&block);
            n_bears += 1;
        } else {
            let reasons: Vec<&str> = lines.iter().map(|(_, f)| f.reason.as_str()).collect();
            may_bear_contents.push_str(&format!("- {reference}: {}\n", reasons.join(" / ")));
            may_bear_entries.push_str(&block);
            n_may_bear += 1;
        }
    }
    let may_bear = format!(
        "# The entries graded `may-bear`\n\n## Contents\n\n{may_bear_contents}\n## Entries\n\
         {may_bear_entries}"
    );
    // Both files or neither: a write that fails removes the other, so a session never reads half an
    // assembly as the whole.
    let written = [(BEARS_FILE, bears), (MAY_BEAR_FILE, may_bear)]
        .into_iter()
        .try_for_each(|(name, text)| {
            let path = dir.join(name);
            std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
        });
    if let Err(e) = written {
        for name in [BEARS_FILE, MAY_BEAR_FILE] {
            let _ = std::fs::remove_file(dir.join(name));
        }
        return Err(format!("{e}. No assembly was written."));
    }
    Ok((n_bears, n_may_bear))
}

/// The snapshot `--out` wrote, as each reference with its show text.
///
/// An entry is a line `=== <reference>`, a blank line, a fence opening with `text`, and the
/// show text up to the line that is that fence alone, as `snapshot` writes it.
fn read_snapshot(dir: &Path) -> Result<BTreeMap<String, String>, String> {
    let path = dir.join(SHOWN_FILE);
    let text = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "{}: {e}: the reports pass, but the snapshot `--out` writes cannot be read: run \
             `--out` again into a new directory. No assembly was written.",
            path.display()
        )
    })?;
    let mut out = BTreeMap::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let Some(reference) = line.strip_prefix("=== ") else {
            continue;
        };
        let fence = lines
            .find(|l| !l.is_empty())
            .and_then(|l| l.strip_suffix("text"))
            .filter(|f| f.len() >= 3 && f.chars().all(|c| c == '`'))
            .ok_or_else(|| {
                format!(
                    "{}: the entry {reference} opens no fence: run `--out` again into a new \
                     directory. No assembly was written.",
                    path.display()
                )
            })?;
        let mut body: Vec<&str> = Vec::new();
        let mut closed = false;
        for line in lines.by_ref() {
            if line == fence {
                closed = true;
                break;
            }
            body.push(line);
        }
        // A fence never closed is a snapshot cut short, whose last entry would be assembled
        // truncated.
        if !closed {
            return Err(format!(
                "{}: the entry {reference} is cut short, its fence never closed: run `--out` \
                 again into a new directory. No assembly was written.",
                path.display()
            ));
        }
        out.insert(reference.to_string(), body.join("\n"));
    }
    Ok(out)
}

fn role_of(word: &str) -> Option<Role> {
    [
        Role::Unit,
        Role::Goal,
        Role::Rejected,
        Role::Issue,
        Role::Tripwire,
    ]
    .into_iter()
    .find(|r| r.word() == word)
}

/// The fields of a graded line, after its reference.
#[derive(Clone)]
struct Fields {
    grade: String,
    relation: String,
    reason: String,
}

/// The fields after a line's reference: an optional seed field, then `grade:`, `relation:` and
/// `reason:`, in that order. The reason is the rest of the line, so it may hold ` | `.
fn parse_fields(fields: &str) -> Option<Fields> {
    let mut rest = fields;
    if rest.starts_with("cites seed: ") || rest.starts_with("is a seed") {
        rest = rest.split_once(" | ")?.1;
    }
    let (grade, rest) = rest.strip_prefix("grade:")?.split_once(" | ")?;
    let (relation, rest) = rest.strip_prefix("relation:")?.split_once(" | ")?;
    let reason = rest.strip_prefix("reason:")?;
    Some(Fields {
        grade: grade.trim().to_string(),
        relation: relation.trim().to_string(),
        reason: reason.trim().to_string(),
    })
}

/// Every finding on one graded line.
fn judge(at: &str, reference: &str, entry: &Listed, fields: &Fields) -> Vec<String> {
    let mut out = Vec::new();
    let allowed = relations(entry.role);
    match fields.grade.as_str() {
        "bears" => {
            let given: Vec<&str> = fields
                .relation
                .split(',')
                .map(str::trim)
                .filter(|r| !r.is_empty())
                .collect();
            if given.is_empty() {
                out.push(format!(
                    "{at}  {reference} bears with no relation: give one of {}",
                    allowed.join(", ")
                ));
            }
            for relation in given.iter().filter(|r| !allowed.contains(r)) {
                out.push(format!(
                    "{at}  {reference}: `{relation}` is no relation for the role `{}`: give one of {}",
                    entry.role.word(),
                    allowed.join(", ")
                ));
            }
        }
        "may-bear" | "unrelated" => {
            if !fields.relation.is_empty() {
                out.push(format!(
                    "{at}  {reference} is graded `{}` and gives a relation: a relation goes with \
                     `bears` alone",
                    fields.grade
                ));
            }
        }
        "" => out.push(format!(
            "{at}  {reference} has no grade: give `bears`, `may-bear` or `unrelated`"
        )),
        other => out.push(format!(
            "{at}  {reference}: `{other}` is no grade: give `bears`, `may-bear` or `unrelated`"
        )),
    }
    if matches!(fields.grade.as_str(), "bears" | "may-bear") && fields.reason.is_empty() {
        out.push(format!(
            "{at}  {reference} is graded `{}` with no reason",
            fields.grade
        ));
    }
    if fields.grade == "unrelated" && (entry.seed || entry.cites_seed) {
        let what = if entry.seed {
            "is a seed"
        } else {
            "cites a seed"
        };
        out.push(format!(
            "{at}  {reference} {what}, so it bears or may bear: grade it `bears` or `may-bear`"
        ));
    }
    out
}
