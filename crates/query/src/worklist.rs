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

/// The file listing every agent's entries, one per line: `<agent> <role> <reference>`, then the
/// seed mark where one applies. `--verify` also reads a changed-since mark, after the seed mark,
/// and carries it into the verdict files.
const AGENTS_FILE: &str = "agents.txt";
/// The file holding an agent's entries whole.
const ENTRIES_FILE: &str = "entries.md";
/// The skeleton an agent fills.
const REPORT_FILE: &str = "report.md";
/// The text of every entry of the record, its own text and its inbound list as two fenced blocks,
/// for `--verify` to check the quotations against and to write its outputs from; no agent is
/// given it, so an agent's load is its entries file alone.
const SHOWN_FILE: &str = "shown.md";
/// The work of the search and its subjects, for the searchers, the session that dispatches them
/// and `--verify`, which takes the subjects from it, as [`work_text`] writes it.
const WORK_FILE: &str = "work.txt";
/// The one subject of a work for which no subject is named.
const WHOLE_WORK: &str = "whole-work";
/// A passing `--verify` writes one verdict file per subject of the work,
/// `verdicts-<subject>.md`: every verdict but `unrelated` against that subject, with its entry's
/// text.
const VERDICTS_PREFIX: &str = "verdicts-";
const VERDICTS_SUFFIX: &str = ".md";
/// A passing `--verify` writes the lines a commit message owes, one per entry given a verdict
/// other than `unrelated`: its reference, its verdict and its action.
const GENERATED_LINES_FILE: &str = "generated-lines.md";
/// The two sections every report closes with, in this order, per `primer@decided-section`; the
/// verdict lines stop at the first of them.
const DECIDED: &str = "## Decided";
const MET_OUTSIDE: &str = "## Met outside the task";
/// A report line that names an entry graded under an earlier cluster of the same agent.
const GRADED_ABOVE: &str = "graded above";
/// The mark of a seed that is itself an entry of the agent, and of an entry that cites a seed.
const SEED_MARK: &str = "seed";
const CITES_SEED_MARK: &str = "cites-seed";
/// The mark of an entry whose file changed since a commit the run names.
const CHANGED_MARK: &str = "changed-since";
/// The words a quotation holds at least, or the whole of an entry's own body where it is shorter.
const QUOTE_WORDS: usize = 8;

/// What `worklist` does. One value, so that writing a work list and checking the reports cannot be
/// asked together, and neither can a bound, a seed, a file of the work or a subject be given to the
/// check.
#[derive(Clone, Debug)]
pub enum WorklistMode {
    /// Write the work list into `dir`, a new or empty directory outside the project.
    Write {
        /// Where the work list goes.
        dir: std::path::PathBuf,
        /// The words one agent is given at most, the work's words included; a group above what
        /// the work leaves goes to an agent alone. A work longer than the bound leaves 0, and every
        /// group then goes to an agent alone.
        bound: usize,
        /// The references the work names.
        seeds: Vec<String>,
        /// The files of the work, which `work.txt` lists for the agents to read beside their
        /// entries; no agent's directory holds them. A relative path is read from the working
        /// directory. Each file is counted and listed once, by its real path, however many times
        /// and in whatever spellings it is given. One that cannot be read refuses the run.
        work: Vec<PathBuf>,
        /// The subjects of the work, which `work.txt` lists, each once, in the order given; none
        /// gives the one subject `whole-work`. Nothing here checks their form: the caller does,
        /// as `cli::run` refuses a subject that is no id in the grammar of an entry id.
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
/// naming no entry, or a file of the work it cannot read, nothing written, and a write that
/// failed, the files before it left in place. Exit 1 is a finding on a report.
///
/// It checks neither the form of a subject nor that a file of the work holds anything: a subject
/// is listed as given, each once, and an empty file of the work counts no word.
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
                     design entry or of another entry `{} show` prints, a goal at an intake. \
                     Nothing was written.",
                    manifest.command()
                ))
            }
        }
    }

    // Every agent reads the work beside its entries, so its words come out of every agent's
    // bound. A work longer than the bound leaves none, and every group then goes to an agent alone.
    // A file given twice, or by a relative and an absolute spelling, is one file of the work: it is
    // counted and listed once, by the real path `absolute` gives an existing file.
    let mut work_paths: Vec<PathBuf> = Vec::new();
    let mut work_words = 0;
    for path in work {
        let text = std::fs::read_to_string(path).map_err(|e| {
            format!(
                "the file of the work {}: {e}. Nothing was written.",
                path.display()
            )
        })?;
        let real = absolute(path)?;
        if work_paths.contains(&real) {
            continue;
        }
        work_words += text.split_whitespace().count();
        work_paths.push(real);
    }
    let bound_left = bound.saturating_sub(work_words);

    let plan = graph::plan(&mut graph, bound_left);
    let agent_groups: Vec<Vec<usize>> = plan.agents.iter().map(|a| a.groups.clone()).collect();
    let mut files = render(&graph, &agent_groups, &seeds, &work_subjects(subjects));
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

    if !work_paths.is_empty() {
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
    for &g in &plan.above {
        if let Some(line) = notice(&graph.groups[g], graph.own_words(g), bound_left) {
            outln!("{line}");
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

/// The notice for a group alone on its agent above the bound, whose own entries hold `own` words,
/// or `None`.
///
/// A group is named where its own entries exceed the bound, which a split of its subject, of its
/// standing entries or of its goals home reduces; one above it by its context entries and title
/// lines alone has nothing of its own to split, and is named by nothing but its agent's load.
fn notice(group: &graph::Group, own: usize, bound: usize) -> Option<String> {
    if own <= bound {
        return None;
    }
    Some(match group.kind {
        graph::GroupKind::Cluster => format!(
            "notice: the cluster \"{}\" holds {own} words, above the {bound}-word bound: split its \
             subject into subjects read apart",
            group.name,
        ),
        graph::GroupKind::Remainder => format!(
            "notice: the remainder of {} holds {own} words, above the {bound}-word bound: give its \
             issues group subdirectories, or have them cite the heads they concern",
            group.unit,
        ),
        graph::GroupKind::Goals => format!(
            "notice: the goals of \"{}\" hold {own} words, above the {bound}-word bound: split the \
             goals home into subdocuments",
            group.name,
        ),
    })
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

/// `work.txt`: a line `work <path>` per file of the work, as `write` lists them, then a line
/// `subject <id>` per subject, each once, in the order given, `whole-work` where none is given.
/// The shape is this function's, read by the searcher and the session alone, per
/// `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
fn work_text(work: &[PathBuf], subjects: &[String]) -> String {
    let mut out = String::new();
    for path in work {
        out.push_str(&format!("work {}\n", path.display()));
    }
    for subject in work_subjects(subjects) {
        out.push_str(&format!("subject {subject}\n"));
    }
    out
}

/// The subjects of the work: each one given once, in the order given, or `whole-work` alone.
fn work_subjects(subjects: &[String]) -> Vec<&str> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let named: Vec<&str> = subjects
        .iter()
        .map(String::as_str)
        .filter(|s| seen.insert(s))
        .collect();
    if named.is_empty() {
        vec![WHOLE_WORK]
    } else {
        named
    }
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
///
/// Each agent's `report.md` names the subjects of the work, then gives one line per entry it
/// grades, whose `verdict:` field lists the verdicts the entry's register admits, for the searcher
/// to replace by one, and closes with `## Decided` and `## Met outside the task`.
fn render(
    graph: &Graph,
    agents: &[Vec<usize>],
    seeds: &BTreeSet<usize>,
    subjects: &[&str],
) -> Vec<(PathBuf, String)> {
    let mut list = String::new();
    let mut files = Vec::new();
    let named: Vec<String> = subjects.iter().map(|s| format!("`{s}`")).collect();
    for (n, groups) in agents.iter().enumerate() {
        let agent = format!("agent-{}", n + 1);
        let mut entries = format!("# The entries of {agent}\n");
        let mut report = format!(
            "# The report of {agent}\n\n\
             The subjects of the work: {}.\n\n\
             On each line, replace the verdicts after `verdict:` by one of them. After `against:`,\n\
             the subjects the verdict is against, comma-separated, or `-`; after `action:`, one\n\
             action the verdict admits; after `owner-question:`, what only the owner can answer,\n\
             or nothing; after `quote:`, at least {QUOTE_WORDS} words copied verbatim from the entry's\n\
             own text, or the whole of a shorter one; after `reason:`, one line. Under\n\
             `{DECIDED}`, the decisions the owner could have made, and under\n\
             `{MET_OUTSIDE}`, what was met outside the task, each empty where\n\
             there is none.\n",
            named.join(", ")
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
                    "- {reference}{seed_field} | verdict: {} | against: | action: | \
                     owner-question: | quote: | reason:\n",
                    verdicts(entry.role).join(", ")
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
        report.push_str(&format!("\n{DECIDED}\n\n{MET_OUTSIDE}\n\n"));
        files.push((PathBuf::from(&agent).join(ENTRIES_FILE), entries));
        files.push((PathBuf::from(&agent).join(REPORT_FILE), report));
    }
    files.insert(0, (PathBuf::from(AGENTS_FILE), list));
    files
}

/// The snapshot: every entry of the record, in the graph's order, as a line `=== <reference>`, a
/// blank line, the entry's own text in a fence, a blank line, and its inbound list in a fence,
/// each fence longer than any run of backticks its block holds.
///
/// The two blocks are `show`'s own text, from [`crate::show::record_text`], and its inbound list,
/// from [`crate::show::cited_at`] and [`crate::show::inbound_text`], over one
/// [`crate::model::table::Inbound`] built for every entry at once: `show` prints the first, a
/// blank line, then the second. They are apart so that `--verify` checks a quotation against the
/// entry's own body alone, never against the sites that cite it. `--verify` finds an entry by the
/// reference on its `===` line, and reads each block up to the line that is its fence alone,
/// which no line of the block can be.
fn snapshot(graph: &Graph, anchors: &Anchors, model: &crate::Model) -> String {
    let index = crate::model::table::Inbound::build(model, anchors);
    let mut out = String::new();
    for entry in &graph.entries {
        let reference = entry.reference();
        let (kind, anchor, id) = &entry.key;
        let own = crate::show::record_text(&reference, &entry.record);
        let inbound =
            crate::show::inbound_text(&crate::show::cited_at(&index, anchors, kind, anchor, id));
        out.push_str(&format!(
            "=== {reference}\n\n{}\n{}\n",
            fenced(&own),
            fenced(&inbound)
        ));
    }
    out
}

/// `text` in a fence opening with `text`, longer than any run of backticks `text` holds, and a
/// newline after the closing fence.
fn fenced(text: &str) -> String {
    let fence = "`".repeat(longest_backtick_run(text).max(2) + 1);
    format!("{fence}text\n{text}\n{fence}\n")
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
    changed: bool,
}

/// The verdicts an entry of a role admits, which are those of its register, a head and an entry
/// of a register the project declares sharing the role `unit`. The skeleton lists them on each
/// line, and `judge` refuses any other.
fn verdicts(role: Role) -> &'static [&'static str] {
    match role {
        Role::Unit | Role::Goal => &["constrains", "conflicts", "unrelated"],
        Role::Rejected => &["brings-back", "constrains", "unrelated"],
        Role::Issue => &[
            "closes",
            "touches",
            "worsens",
            "depends-on",
            "trigger-met",
            "constrains",
            "unrelated",
        ],
        Role::Tripwire => &["fires", "constrains", "unrelated"],
    }
}

/// The actions a verdict admits on an entry of a role: `conflicts` on a goal goes to goal-setting,
/// on any other entry to the reversal check. A word that is no verdict admits none.
fn actions(role: Role, verdict: &str) -> &'static [&'static str] {
    match verdict {
        "constrains" | "touches" | "worsens" => &["respect", "none"],
        "conflicts" if role == Role::Goal => &["goal-setting"],
        "conflicts" => &["check-reversal"],
        "brings-back" => &["new-argument-needed"],
        "closes" => &["close-with-the-work"],
        "depends-on" | "fires" | "trigger-met" => &["rule-before-work"],
        "unrelated" => &["none"],
        _ => &[],
    }
}

/// Check every report of the directory against `agents.txt`, `work.txt` and the snapshot, and,
/// when every report passes, write the verdict files and the generated lines.
///
/// It reads no model: what each agent was given is what `agents.txt` says, the subjects are what
/// `work.txt` lists, and each entry's text is what `--out` wrote into the snapshot, so the check
/// and the outputs hold whatever the tree has become since. Every output of a previous run is
/// removed before anything is judged, so a run that fails, or stops on a file it cannot read,
/// leaves none a session could take for this one's.
fn verify(dir: &Path) -> Result<ExitCode, String> {
    remove_outputs(dir)?;
    let list_path = dir.join(AGENTS_FILE);
    let text =
        std::fs::read_to_string(&list_path).map_err(|e| format!("{}: {e}", list_path.display()))?;
    let mut agents: BTreeMap<String, BTreeMap<String, Listed>> = BTreeMap::new();
    // The order of `agents.txt`, which the outputs keep: the map above sorts `agent-10`
    // before `agent-2`.
    let mut order: Vec<(String, String)> = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let parts: Vec<&str> = line.split(' ').collect();
        let role = parts.get(1).and_then(|w| role_of(w));
        let marks = parts.get(3..).unwrap_or_default();
        let (Some(agent), Some(reference), Some(listed)) = (
            parts.first(),
            parts.get(2),
            role.and_then(|r| listed(r, marks)),
        ) else {
            return Err(format!(
                "{}:{}: not a line `<agent> <role> <reference>`, then `{SEED_MARK}` or \
                 `{CITES_SEED_MARK}`, then `{CHANGED_MARK}`, each where it applies",
                list_path.display(),
                n + 1
            ));
        };
        order.push((agent.to_string(), reference.to_string()));
        agents
            .entry(agent.to_string())
            .or_default()
            .insert(reference.to_string(), listed);
    }
    let subjects = read_subjects(dir)?;
    let snapshot = read_snapshot(dir)?;
    let mut missing: Vec<&str> = order
        .iter()
        .map(|(_, r)| r.as_str())
        .filter(|r| !snapshot.contains_key(*r))
        .collect();
    missing.dedup();
    if !missing.is_empty() {
        return Err(format!(
            "{} lacks {}, which {AGENTS_FILE} lists: run `--out` again into a new directory. \
             Nothing was judged.",
            dir.join(SHOWN_FILE).display(),
            missing.join(", ")
        ));
    }

    let mut findings: Vec<String> = Vec::new();
    let mut given: BTreeMap<(String, String), Verdict> = BTreeMap::new();
    for (agent, listed) in &agents {
        let rel = PathBuf::from(agent).join(REPORT_FILE);
        let path = dir.join(&rel);
        let report =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut graded: BTreeMap<String, usize> = BTreeMap::new();
        for (n, line) in report.lines().enumerate() {
            let at = format!("{}:{}", rel.display(), n + 1);
            // The verdict lines end at the report's closing sections, whose own lines may be
            // list items too.
            if matches!(line.trim_end(), DECIDED | MET_OUTSIDE) {
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
                    "{}/{at}: not a line `- <reference> | verdict: <verdict> | against: <subjects, \
                     or -> | action: <action> | owner-question: <text, or nothing> | quote: <text> \
                     | reason: <text>`, its fields in this order, the owner-question holding no \
                     ` | quote:` and the quotation no ` | reason:`",
                    dir.display()
                ));
            };
            *graded.entry(reference.to_string()).or_default() += 1;
            let Some(entry) = listed.get(reference) else {
                findings.push(format!("{at}  {reference} is not an entry of {agent}"));
                continue;
            };
            findings.extend(judge(
                &at,
                reference,
                entry,
                &parsed,
                &subjects,
                own_body(&snapshot[reference].own),
            ));
            given.insert((agent.clone(), reference.to_string()), parsed);
        }
        for section in [DECIDED, MET_OUTSIDE] {
            if !report.lines().any(|l| l.trim_end() == section) {
                findings.push(format!(
                    "{}  no section `{section}`: close the report with `{DECIDED}` then \
                     `{MET_OUTSIDE}`, each empty where there is nothing",
                    rel.display()
                ));
            }
        }
        for reference in listed.keys() {
            match graded.get(reference) {
                None => findings.push(format!(
                    "{}  {reference} is given to {agent} and has no verdict",
                    rel.display()
                )),
                Some(&count) if count > 1 => findings.push(format!(
                    "{}  {reference} is given {count} verdicts",
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
        let (per_subject, lines) = assemble(dir, &order, &agents, &given, &snapshot, &subjects)?;
        for (subject, count) in per_subject {
            outln!(
                "{count} verdicts against {subject} in {}",
                dir.join(verdicts_file(&subject)).display()
            );
        }
        outln!(
            "{lines} lines in {}",
            dir.join(GENERATED_LINES_FILE).display()
        );
        outln!("PASSED: every report gives every entry it was given one verdict");
        Ok(ExitCode::SUCCESS)
    } else {
        outln!("FAILED: {} findings above", findings.len());
        Ok(ExitCode::FAILURE)
    }
}

/// The marks of a line of `agents.txt` after its reference, as the entry they describe: a seed
/// mark, then a changed-since mark, each where it applies, or `None` for any other words.
fn listed(role: Role, marks: &[&str]) -> Option<Listed> {
    let (seed_mark, changed) = match marks {
        [] => (None, false),
        [CHANGED_MARK] => (None, true),
        [mark] => (Some(*mark), false),
        [mark, CHANGED_MARK] => (Some(*mark), true),
        _ => return None,
    };
    if !matches!(seed_mark, None | Some(SEED_MARK) | Some(CITES_SEED_MARK)) {
        return None;
    }
    Some(Listed {
        role,
        seed: seed_mark == Some(SEED_MARK),
        cites_seed: seed_mark == Some(CITES_SEED_MARK),
        changed,
    })
}

/// The name of the verdict file of `subject`.
fn verdicts_file(subject: &str) -> String {
    format!("{VERDICTS_PREFIX}{subject}{VERDICTS_SUFFIX}")
}

/// Remove every output a previous `--verify` wrote into `dir`: each verdict file and the
/// generated lines.
fn remove_outputs(dir: &Path) -> Result<(), String> {
    let listing = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in listing {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let output = name == GENERATED_LINES_FILE
            || (name.starts_with(VERDICTS_PREFIX) && name.ends_with(VERDICTS_SUFFIX));
        if output {
            let path = entry.path();
            std::fs::remove_file(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(())
}

/// The subjects `work.txt` lists, in its order.
///
/// A line is `work <path>`, which `--verify` does not read, or `subject <id>`; any other line, and
/// a subject that is no id in the grammar of an entry id, refuse the run, since a subject names
/// a file `--verify` writes.
fn read_subjects(dir: &Path) -> Result<Vec<String>, String> {
    let path = dir.join(WORK_FILE);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut subjects = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.starts_with("work ") {
            continue;
        }
        match line.strip_prefix("subject ") {
            Some(s) if crate::model::table::is_entry_id(s) => subjects.push(s.to_string()),
            _ => {
                return Err(format!(
                    "{}:{}: not a line `work <path>` or `subject <id>`",
                    path.display(),
                    n + 1
                ))
            }
        }
    }
    Ok(subjects)
}

/// The words of `text`, each run of whitespace made one space.
fn collapsed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// An entry's own body: its own text in the snapshot without its first line, the
/// `<reference>  <site>` line `show` prints above the body.
fn own_body(own: &str) -> &str {
    own.split_once('\n').map(|(_, body)| body).unwrap_or("")
}

/// Write the outputs of passing reports, and return how many verdicts each subject's file holds,
/// in the order of `work.txt`, and how many generated lines there are.
///
/// Each subject's verdict file, `verdicts-<subject>.md`, holds every verdict but `unrelated` whose
/// `against` names the subject, in the order of `agents.txt`: its entry's `===` line, carrying
/// the changed-since mark where `agents.txt` gives it, the agent's verdict line, then the entry's
/// own text and its inbound list from the snapshot, each fenced as the snapshot fences it, so that
/// a heading of the text reads as no section of the file. A verdict `against: -` is in no verdict
/// file. A subject no verdict names still gets its file, which then holds its heading alone, so a
/// session tells a subject nothing bears on from one whose file is missing.
///
/// `generated-lines.md` holds one line per entry given a verdict other than `unrelated`, in the
/// same order, `` - `<reference>`: <verdict>, <action>. ``. Its reference
/// is backticked so that `cargo klarch commits` judges it in the message it is pasted into; it
/// carries no quotation and no reason, since a quotation's references would read as live there and
/// its marks could end the span early.
///
/// Every file or none: a write that fails removes those written before it, so a session never
/// reads part of the outputs as the whole.
fn assemble(
    dir: &Path,
    order: &[(String, String)],
    agents: &BTreeMap<String, BTreeMap<String, Listed>>,
    given: &BTreeMap<(String, String), Verdict>,
    snapshot: &BTreeMap<String, Shown>,
    subjects: &[String],
) -> Result<(Vec<(String, usize)>, usize), String> {
    let mut files: Vec<(String, String, usize)> = subjects
        .iter()
        .map(|s| (s.clone(), format!("# The verdicts against `{s}`\n"), 0))
        .collect();
    let mut lines: Vec<String> = Vec::new();
    for (agent, reference) in order {
        let Some(verdict) = given.get(&(agent.clone(), reference.clone())) else {
            continue;
        };
        if verdict.verdict == "unrelated" {
            continue;
        }
        let line = format!("- `{reference}`: {}, {}.", verdict.verdict, verdict.action);
        lines.push(line);
        let changed = agents
            .get(agent)
            .and_then(|l| l.get(reference))
            .is_some_and(|l| l.changed);
        let shown = &snapshot[reference];
        let block = format!(
            "\n=== {reference}{}\n{agent}: {}\n\n{}\n{}",
            if changed {
                format!("  {CHANGED_MARK}")
            } else {
                String::new()
            },
            verdict.line(),
            fenced(&shown.own),
            fenced(&shown.inbound)
        );
        let mut named: Vec<&str> = Vec::new();
        for subject in verdict.against.iter().flatten() {
            if named.contains(&subject.as_str()) {
                continue;
            }
            named.push(subject);
            if let Some((_, text, count)) = files.iter_mut().find(|(s, _, _)| s == subject) {
                text.push_str(&block);
                *count += 1;
            }
        }
    }
    let mut writes: Vec<(String, String)> = files
        .iter()
        .map(|(s, text, _)| (verdicts_file(s), text.clone()))
        .collect();
    let generated: String = lines.iter().map(|l| format!("{l}\n")).collect();
    writes.push((GENERATED_LINES_FILE.to_string(), generated));
    for (n, (name, text)) in writes.iter().enumerate() {
        let path = dir.join(name);
        if let Err(e) = std::fs::write(&path, text) {
            for (written, _) in &writes[..n] {
                let _ = std::fs::remove_file(dir.join(written));
            }
            return Err(format!(
                "{}: {e}. No verdict file and no generated line was written.",
                path.display()
            ));
        }
    }
    Ok((
        files.into_iter().map(|(s, _, count)| (s, count)).collect(),
        lines.len(),
    ))
}

/// One entry of the snapshot: its own text, which `show` prints first, and its inbound list.
struct Shown {
    own: String,
    inbound: String,
}

/// The snapshot `--out` wrote, as each reference with its two blocks.
///
/// An entry is a line `=== <reference>`, a blank line, a fence opening with `text`, the entry's own
/// text up to the line that is that fence alone, a blank line, and its inbound list in a fence of
/// its own, as `snapshot` writes it.
fn read_snapshot(dir: &Path) -> Result<BTreeMap<String, Shown>, String> {
    let path = dir.join(SHOWN_FILE);
    let text = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "{}: {e}: the snapshot `--out` writes cannot be read: run `--out` again into a new \
             directory. Nothing was judged.",
            path.display()
        )
    })?;
    let mut out = BTreeMap::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let Some(reference) = line.strip_prefix("=== ") else {
            continue;
        };
        let mut blocks: Vec<String> = Vec::new();
        for _ in 0..2 {
            let fence = lines
                .find(|l| !l.is_empty())
                .and_then(|l| l.strip_suffix("text"))
                .filter(|f| f.len() >= 3 && f.chars().all(|c| c == '`'))
                .ok_or_else(|| {
                    format!(
                        "{}: the entry {reference} does not open its two fenced blocks: run \
                         `--out` again into a new directory. Nothing was judged.",
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
            // A fence never closed is a snapshot cut short, whose last entry would be read
            // truncated.
            if !closed {
                return Err(format!(
                    "{}: the entry {reference} is cut short, its fence never closed: run `--out` \
                     again into a new directory. Nothing was judged.",
                    path.display()
                ));
            }
            blocks.push(body.join("\n"));
        }
        let inbound = blocks.pop().unwrap_or_default();
        let own = blocks.pop().unwrap_or_default();
        out.insert(reference.to_string(), Shown { own, inbound });
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

/// The fields of a verdict line, after its reference and its seed field.
#[derive(Clone, Debug, PartialEq)]
struct Verdict {
    verdict: String,
    /// The subjects the verdict is against, as written, or `None` for `-`.
    against: Option<Vec<String>>,
    action: String,
    owner_question: String,
    quote: String,
    reason: String,
}

impl Verdict {
    /// The line's fields, as the report writes them, an empty field as its name and a colon.
    fn line(&self) -> String {
        let against = match &self.against {
            None => "-".to_string(),
            Some(subjects) => subjects.join(", "),
        };
        [
            ("verdict", self.verdict.as_str()),
            ("against", &against),
            ("action", &self.action),
            ("owner-question", &self.owner_question),
            ("quote", &self.quote),
            ("reason", &self.reason),
        ]
        .iter()
        .map(|(name, value)| {
            if value.is_empty() {
                format!("{name}:")
            } else {
                format!("{name}: {value}")
            }
        })
        .collect::<Vec<_>>()
        .join(" | ")
    }
}

/// The fields after a line's reference: an optional seed field, then `verdict:`, `against:`,
/// `action:`, `owner-question:`, `quote:` and `reason:`, in that order, or `None`.
///
/// The first three hold no ` | `. The reason runs from the last ` | reason:` of the line to its
/// end, and the quotation from the last ` | quote:` before it, so both may hold ` | `, as a
/// quoted table row does. A line whose owner-question holds ` | quote:`, or whose quotation holds
/// ` | reason:`, is `None`: where either field ends could not be told.
fn parse_fields(fields: &str) -> Option<Verdict> {
    let mut rest = fields;
    if rest.starts_with("cites seed: ") || rest.starts_with("is a seed") {
        rest = rest.split_once(" | ")?.1;
    }
    let (verdict, rest) = rest.strip_prefix("verdict:")?.split_once(" | ")?;
    let (against, rest) = rest.strip_prefix("against:")?.split_once(" | ")?;
    let (action, rest) = rest.strip_prefix("action:")?.split_once(" | ")?;
    let rest = rest.strip_prefix("owner-question:")?;
    let (rest, reason) = rest.rsplit_once(" | reason:")?;
    let (owner_question, quote) = rest.rsplit_once(" | quote:")?;
    if owner_question.contains(" | quote:") || quote.contains(" | reason:") {
        return None;
    }
    let against = match against.trim() {
        "-" => None,
        list => Some(
            list.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect(),
        ),
    };
    Some(Verdict {
        verdict: verdict.trim().to_string(),
        against,
        action: action.trim().to_string(),
        owner_question: owner_question.trim().to_string(),
        quote: quote.trim().to_string(),
        reason: reason.trim().to_string(),
    })
}

/// Every finding on one verdict line of an entry whose own body is `body`, the subjects of the
/// work being `subjects`.
///
/// The verdict and the action are judged by the tables of [`verdicts`] and [`actions`]; `against`
/// names subjects of `work.txt`, or is `-` on `unrelated` and on `constrains` with the action
/// `none`; the quotation is found in the body once each run of whitespace is one space, and holds
/// at least eight words, or the whole body where it holds fewer, since no quotation of a shorter
/// body could hold eight; the reason is not empty; and a seed or an entry citing one is never
/// `unrelated`, its verdict where nothing in the work meets it being `constrains`, `none`, `-`.
fn judge(
    at: &str,
    reference: &str,
    entry: &Listed,
    fields: &Verdict,
    subjects: &[String],
    body: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    let allowed = verdicts(entry.role);
    let verdict = fields.verdict.as_str();
    if verdict.is_empty() {
        out.push(format!(
            "{at}  {reference} has no verdict: give one of {}",
            allowed.join(", ")
        ));
    } else if !allowed.contains(&verdict) {
        out.push(format!(
            "{at}  {reference}: `{verdict}` is no verdict for the role `{}`: give one of {}",
            entry.role.word(),
            allowed.join(", ")
        ));
    } else {
        let admitted = actions(entry.role, verdict);
        if !admitted.contains(&fields.action.as_str()) {
            out.push(format!(
                "{at}  {reference}: `{}` is no action for the verdict `{verdict}`: give one of {}",
                fields.action,
                admitted.join(", ")
            ));
        }
    }
    match &fields.against {
        None => {
            if !(verdict == "unrelated" || (verdict == "constrains" && fields.action == "none")) {
                out.push(format!(
                    "{at}  {reference}: `against: -` goes with `unrelated`, or with `constrains` \
                     and the action `none`: name the subjects the verdict is against, of {}",
                    subjects.join(", ")
                ));
            }
        }
        Some(named) if named.is_empty() => out.push(format!(
            "{at}  {reference} names no subject after `against:`: give one or more of {}, or `-`",
            subjects.join(", ")
        )),
        Some(named) => {
            for subject in named.iter().filter(|s| !subjects.contains(s)) {
                out.push(format!(
                    "{at}  {reference}: `{subject}` is no subject of the work: give one of {}",
                    subjects.join(", ")
                ));
            }
        }
    }
    let quote = collapsed(&fields.quote);
    let body = collapsed(body);
    let words = quote.split_whitespace().count();
    let owed = QUOTE_WORDS.min(body.split_whitespace().count());
    if words == 0 {
        out.push(format!(
            "{at}  {reference} has no quotation: copy at least {QUOTE_WORDS} words verbatim from \
             the entry's own text"
        ));
    } else if !body.contains(&quote) {
        out.push(format!(
            "{at}  {reference}: the quotation is not in the entry's own text: copy it verbatim \
             from the entry's body, never from its first line or from the sites that cite it"
        ));
    } else if words < owed {
        out.push(format!(
            "{at}  {reference}: the quotation holds {words} words: quote at least {QUOTE_WORDS} \
             words of the entry's own text, or the whole of a shorter one"
        ));
    }
    if fields.reason.is_empty() {
        out.push(format!("{at}  {reference} has no reason"));
    }
    if verdict == "unrelated" && (entry.seed || entry.cites_seed) {
        let what = if entry.seed {
            "is a seed"
        } else {
            "cites a seed"
        };
        out.push(format!(
            "{at}  {reference} {what}, so it is never `unrelated`: where nothing in the work \
             meets it, give `constrains`, the action `none` and `against: -`"
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(kind: graph::GroupKind) -> graph::Group {
        graph::Group {
            name: "a-unit :: A subject".to_string(),
            key: (String::new(), String::new(), String::new()),
            unit: "a-unit".to_string(),
            kind,
            entries: Vec::new(),
            prose: String::new(),
        }
    }

    /// The claim: a group is named only where its own words exceed the bound, each kind by its own
    /// notice. Mutation checked: `if own <= bound {` made `if false {`, which names a group at the
    /// bound; caught.
    #[test]
    fn a_notice_names_a_group_whose_own_words_exceed_the_bound() {
        assert_eq!(notice(&group(graph::GroupKind::Cluster), 10, 10), None);
        assert_eq!(
            notice(&group(graph::GroupKind::Cluster), 11, 10).as_deref(),
            Some(
                "notice: the cluster \"a-unit :: A subject\" holds 11 words, above the 10-word \
                 bound: split its subject into subjects read apart"
            )
        );
        assert!(notice(&group(graph::GroupKind::Remainder), 11, 10)
            .is_some_and(|n| n.starts_with("notice: the remainder of a-unit holds 11 words")));
        assert!(notice(&group(graph::GroupKind::Goals), 11, 10).is_some_and(
            |n| n.starts_with("notice: the goals of \"a-unit :: A subject\" hold 11 words")
        ));
    }

    /// The claim: `work.txt` lists each file of the work, then each subject once, in the order
    /// given, and `whole-work` where no subject is given. Mutations checked: in `work_subjects`, the
    /// filter `.filter(|s| seen.insert(s))` made `.filter(|_| true)`, which lists a subject twice;
    /// and `if named.is_empty() {` made `if false {`, which lists no subject; each caught.
    #[test]
    fn the_work_file_lists_the_work_then_each_subject_once() {
        let work = [PathBuf::from("/a/draft.md")];
        let subjects = ["second-thread", "first-thread", "second-thread"].map(String::from);
        assert_eq!(
            work_text(&work, &subjects),
            "work /a/draft.md\nsubject second-thread\nsubject first-thread\n"
        );
        assert_eq!(work_text(&[], &[]), "subject whole-work\n");
    }
    fn listed(role: Role) -> Listed {
        Listed {
            role,
            seed: false,
            cites_seed: false,
            changed: false,
        }
    }

    /// A body of nine words, and a quotation of its last eight.
    const BODY: &str = "### A head `##a-head`\n\nIt serves the work\nof the project.";
    const QUOTE: &str = "A head `##a-head` It serves the work of";

    fn line(verdict: &str, against: &str, action: &str, quote: &str, reason: &str) -> Verdict {
        parse_fields(&format!(
            "verdict: {verdict} | against: {against} | action: {action} | owner-question: | \
             quote: {quote} | reason: {reason}"
        ))
        .expect("a verdict line")
    }

    fn findings(role: Role, fields: &Verdict) -> Vec<String> {
        judge(
            "r:1",
            "x@a@b",
            &listed(role),
            fields,
            &["one-thread".to_string()],
            BODY,
        )
    }

    /// The claim: a verdict line parses its six fields in their order, after an optional seed
    /// field, with a quotation and a reason that hold ` | `, and `-` as no subject; a line with a
    /// field missing or out of order, an owner-question holding ` | quote:` or a quotation holding
    /// ` | reason:` does not parse. Mutations checked: in `parse_fields`, the test
    /// `if owner_question.contains(" | quote:") || quote.contains(" | reason:") {` made
    /// `if false {`; `rest.rsplit_once(" | reason:")?` made `rest.split_once(" | reason:")?`;
    /// `"-" => None,` made `"-!" => None,`; each caught.
    #[test]
    fn a_verdict_line_parses_its_fields_in_order_and_refuses_an_ambiguous_one() {
        let full = "is a seed | verdict: constrains | against: one-thread, two-thread | action: \
                    respect | owner-question: which one? | quote: | a | b | table row | reason: it \
                    limits a | b";
        assert_eq!(
            parse_fields(full),
            Some(Verdict {
                verdict: "constrains".into(),
                against: Some(vec!["one-thread".into(), "two-thread".into()]),
                action: "respect".into(),
                owner_question: "which one?".into(),
                quote: "| a | b | table row".into(),
                reason: "it limits a | b".into(),
            })
        );
        let empty = parse_fields(
            "verdict: unrelated | against: - | action: none | owner-question: | quote: q | reason:",
        )
        .expect("a line with empty fields");
        assert_eq!((empty.against, empty.owner_question.as_str()), (None, ""));
        for refused in [
            // The reason missing.
            "verdict: unrelated | against: - | action: none | owner-question: | quote: q",
            // The quotation before the owner-question.
            "verdict: unrelated | against: - | action: none | quote: q | owner-question: | reason: r",
            // The action before `against`.
            "verdict: unrelated | action: none | against: - | owner-question: | quote: q | reason: r",
            "verdict: unrelated | against: - | action: none | owner-question: a | quote: b | \
             quote: c | reason: r",
            "verdict: unrelated | against: - | action: none | owner-question: | quote: a | \
             reason: b | reason: r",
        ] {
            assert_eq!(parse_fields(refused), None, "{refused}");
        }
    }

    /// The claim: `judge` refuses a verdict outside its register's set and an action outside its
    /// verdict's, `conflicts` taking `goal-setting` on a goal and `check-reversal` elsewhere.
    /// Mutations checked: in `verdicts`, `Role::Rejected => &["brings-back", ...` made to admit
    /// `conflicts` too; in `actions`, `"conflicts" if role == Role::Goal` made
    /// `"conflicts" if false`; and in `judge`, `if !admitted.contains(&fields.action.as_str()) {`
    /// made `if false {`; each caught.
    #[test]
    fn judge_refuses_a_verdict_outside_its_register_and_an_action_outside_its_verdict() {
        let ok = |role, verdict, action| {
            findings(role, &line(verdict, "one-thread", action, QUOTE, "r")).is_empty()
        };
        assert!(ok(Role::Unit, "conflicts", "check-reversal"));
        assert!(ok(Role::Goal, "conflicts", "goal-setting"));
        assert!(!ok(Role::Goal, "conflicts", "check-reversal"));
        assert!(!ok(Role::Unit, "conflicts", "goal-setting"));
        assert!(ok(Role::Rejected, "brings-back", "new-argument-needed"));
        assert!(!ok(Role::Rejected, "conflicts", "check-reversal"));
        assert!(!ok(Role::Unit, "fires", "rule-before-work"));
        assert!(ok(Role::Tripwire, "fires", "rule-before-work"));
        assert!(!ok(Role::Tripwire, "fires", "respect"));
        assert!(ok(Role::Issue, "trigger-met", "rule-before-work"));
        assert!(ok(Role::Issue, "touches", "none"));
        assert!(!ok(Role::Issue, "closes", "respect"));
        let refused = findings(Role::Unit, &line("fires", "one-thread", "none", QUOTE, "r"));
        assert!(
            refused[0].contains("`fires` is no verdict for the role `unit`"),
            "{refused:?}"
        );
        let refused = findings(
            Role::Issue,
            &line("closes", "one-thread", "none", QUOTE, "r"),
        );
        assert!(
            refused[0].contains("`none` is no action for the verdict `closes`"),
            "{refused:?}"
        );
    }

    /// The claim: `against` names subjects of the work, or is `-` on `unrelated` and on
    /// `constrains` with `none` alone. Mutations checked: in `judge`, the subject test
    /// `.filter(|s| !subjects.contains(s))` made `.filter(|_| false)`; in the test of `-`,
    /// `(verdict == "constrains" && fields.action == "none")` made `(verdict == "constrains")`,
    /// which admits `-` on `constrains` with `respect`; and the guard of the empty arm,
    /// `Some(named) if named.is_empty()`, made `Some(named) if false`; each caught.
    #[test]
    fn judge_holds_against_to_the_subjects_of_the_work() {
        let at = |verdict, against, action| {
            findings(Role::Unit, &line(verdict, against, action, QUOTE, "r"))
        };
        assert!(at("constrains", "one-thread", "respect").is_empty());
        assert!(at("constrains", "-", "none").is_empty());
        assert!(at("unrelated", "-", "none").is_empty());
        assert!(at("constrains", "-", "respect")[0].contains("`against: -` goes with"));
        assert!(at("conflicts", "-", "check-reversal")[0].contains("`against: -` goes with"));
        assert!(at("constrains", "two-thread", "respect")[0]
            .contains("`two-thread` is no subject of the work: give one of one-thread"));
        assert!(at("constrains", "", "respect")[0].contains("names no subject"));
    }

    /// The claim: the quotation is owed on every line, `unrelated` included, holds at least eight
    /// words or the whole of a shorter body, and is found in the body once whitespace is
    /// collapsed; the reason is owed on every line. Mutations checked: in `judge`,
    /// `} else if !body.contains(&quote) {` made `} else if false {`; `if words == 0 {` made
    /// `if false {`, with the quotation's other tests then passing an empty one; `words < owed`
    /// made `words < 1`; `QUOTE_WORDS.min(` made `QUOTE_WORDS.max(`, which refuses the whole of a
    /// short body; and `if fields.reason.is_empty() {` made `if false {`; each caught.
    #[test]
    fn judge_owes_a_quotation_of_the_body_and_a_reason_on_every_line() {
        let with =
            |quote, reason| findings(Role::Unit, &line("unrelated", "-", "none", quote, reason));
        assert!(with(QUOTE, "r").is_empty());
        // Across the body's line break, by its collapsed whitespace.
        assert!(with("`##a-head` It serves the work of the   project.", "r").is_empty());
        assert!(with("", "r")[0].contains("has no quotation"));
        assert!(
            with("A head `##a-head` It serves the work of the world", "r")[0]
                .contains("the quotation is not in the entry's own text")
        );
        assert!(with("A head `##a-head` It serves the work", "r")[0]
            .contains("the quotation holds 7 words"));
        assert!(with(QUOTE, "")[0].contains("has no reason"));
        let short = judge(
            "r:1",
            "x@a@b",
            &listed(Role::Unit),
            &line("unrelated", "-", "none", "### A head `##a-head`", "r"),
            &["one-thread".to_string()],
            "### A head `##a-head`\n",
        );
        assert!(short.is_empty(), "{short:?}");
    }

    /// The claim: a seed, and an entry citing one, are never `unrelated`. Mutation checked: in
    /// `judge`, `if verdict == "unrelated" && (entry.seed || entry.cites_seed) {` made
    /// `if verdict == "unrelated" && entry.seed {`, which passes the citer; caught.
    #[test]
    fn judge_refuses_a_seed_or_its_citer_given_unrelated() {
        for (seed, cites_seed, what) in [(true, false, "is a seed"), (false, true, "cites a seed")]
        {
            let entry = Listed {
                role: Role::Issue,
                seed,
                cites_seed,
                changed: false,
            };
            let at = |verdict, action| {
                judge(
                    "r:1",
                    "x@a@b",
                    &entry,
                    &line(verdict, "-", action, QUOTE, "r"),
                    &["one-thread".to_string()],
                    BODY,
                )
            };
            assert!(at("unrelated", "none")[0].contains(&format!("x@a@b {what}, so it is never")));
            assert!(at("constrains", "none").is_empty());
        }
    }

    /// The claim: a line of `agents.txt` takes a seed mark, then a changed-since mark, each where it
    /// applies, and no other word. Mutation checked: in `listed`, the arm
    /// `[mark, CHANGED_MARK] => (Some(*mark), true),` removed, which refuses a seed that changed;
    /// caught.
    #[test]
    fn a_listed_entry_takes_a_seed_mark_then_a_changed_since_mark() {
        let marks = |m: &[&str]| listed_of(m);
        assert_eq!(marks(&[]), Some((false, false, false)));
        assert_eq!(marks(&["seed"]), Some((true, false, false)));
        assert_eq!(
            marks(&["cites-seed", "changed-since"]),
            Some((false, true, true))
        );
        assert_eq!(marks(&["changed-since"]), Some((false, false, true)));
        assert_eq!(marks(&["changed-since", "seed"]), None);
        assert_eq!(marks(&["seed", "cites-seed"]), None);
        assert_eq!(marks(&["other"]), None);
    }

    fn listed_of(marks: &[&str]) -> Option<(bool, bool, bool)> {
        super::listed(Role::Unit, marks).map(|l| (l.seed, l.cites_seed, l.changed))
    }
}
