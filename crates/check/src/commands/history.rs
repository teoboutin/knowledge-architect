//! Commit messages under the regime, judged by `commits` over a range, and the snapshot of a tree
//! read from git objects that judging them needs, which `check --staged`, `index --staged` and the
//! comparison of `check --fix` read too.
//!
//! **A commit message is a document.** It is parsed as one markdown document — the subject
//! line, the blank line and the body — and every rule of the regime runs over it: every
//! reference resolves through the entity table, and each registered extension judges it against
//! the tree it prepared for that commit.
//! The decision, and why a message is judged against a tree read from git objects rather than
//! against the working tree, is `design@check@a-commit-message-is-a-document`.
//!
//! `commits <range>` lives here: it walks the range and judges each commit's message and tree
//! against that commit's own tree. So do `check --staged` and `index --staged`, which assemble the
//! tree git's index would commit the same way, per `design@check@staged-tree-source`.

use super::output::outln;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::check::{self, Inputs};
use crate::extension::{Extension, Prepared, Purpose, Snapshot, Tree};
use crate::git::Source;
use crate::model::listing::Survey;
use crate::model::table::{Anchors, Entities};
use crate::model::MANIFEST_NAME;
use crate::{Finding, Manifest, Model};

// ---------------------------------------------------------------------------------------
// The message as a document
// ---------------------------------------------------------------------------------------

/// The rel path a message is parsed under.
///
/// The suffix is what selects the markdown grammar, per `design@model@grammars-not-prefixes`, so
/// a message needs one; the findings are relabelled with the commit or the file before they
/// are printed, and no reader ever sees this name.
fn message_rel() -> PathBuf {
    PathBuf::from("commit-message.md")
}

/// Everything one snapshot contributes to judging a message written against it, or to judging
/// and generating over the tree git's index would commit.
///
/// Held together because `Inputs` borrows every field of it: a check is a pure function over
/// the model, so whatever it needs about the tree is gathered once and handed in.
struct Assembly {
    manifest: Manifest,
    model: Model,
    /// Each extension prepared for this tree, with its check names; empty for a tree assembled
    /// for its references alone.
    prepared: Vec<(&'static [&'static str], Box<dyn Prepared>)>,
    committed: HashMap<PathBuf, String>,
    configs: HashMap<PathBuf, String>,
    /// The `register.toml` files whose bytes are not UTF-8.
    configs_not_text: HashSet<PathBuf>,
    survey: Survey,
    ignored: HashSet<String>,
    tracked_and_ignored: Vec<PathBuf>,
    /// What is wrong with the tree itself: every check's findings at `Depth::Judged`, the
    /// findings of phases 1 to 3 at `Depth::Gated`, nothing at `Depth::References`.
    trouble: Vec<Finding>,
    /// The phase the tree's run stopped in, where it stopped before the last: its entity
    /// table is then incomplete, and a message judged against it is judged against nothing.
    stopped: Option<check::Phase>,
    /// Whether the installed files are compared against `shipped`: under `check --staged`,
    /// never under `commits`.
    compare_installed: bool,
    /// The agent files the running binary ships; empty where nothing is compared.
    shipped: Vec<(PathBuf, String)>,
    /// The tree assembled: the index's under `check --staged` and `index --staged`, a commit's
    /// under `commits`.
    judged: check::Judged,
    /// The whole report of a judged tree, or of its stop, for a caller that prints it as `check`
    /// does; `None` for a tree assembled for its references alone.
    report: Option<check::Report>,
    /// The snapshot the extensions were prepared over, kept for a caller that asks them for
    /// their generated files; `None` where no extension was prepared.
    snapshot: Option<Snapshot>,
}

impl Assembly {
    fn anchors(&self) -> Anchors {
        Anchors::of(&self.manifest, &self.survey.present)
    }

    fn inputs(&self) -> Inputs<'_> {
        Inputs {
            committed: &self.committed,
            configs: &self.configs,
            configs_not_text: &self.configs_not_text,
            present: &self.survey.present,
            directories: &self.survey.directories,
            outside: &self.survey.outside,
            ignored: &self.ignored,
            tracked_and_ignored: &self.tracked_and_ignored,
            refused: &self.survey.refused,
            links: &self.survey.links,
            // A commit's installed files are not compared: the running binary ships its own
            // version's text, and an older commit's installed set would fail against it with no
            // repair a commit in history can take. `check` on the working tree enforces them,
            // per `design@check@only-check-compares-installed-files`, and so does `check --staged`, whose
            // snapshot is the next commit, judged as a clean checkout of it would be.
            installed: if self.compare_installed {
                &self.survey.installed
            } else {
                &[]
            },
            shipped: &self.shipped,
            judged: self.judged,
        }
    }
}

/// The tree a snapshot of `source` is, as a check names its repairs for it.
fn judged_of(source: Source) -> check::Judged {
    match source {
        Source::Index => check::Judged::Index,
        Source::Commit(_) => check::Judged::Commit,
    }
}

/// Judge one message against a tree, and optionally against a second one.
///
/// **A reference resolves against either tree, a lint stands where either tree holds what it names,
/// and every other rule is judged against the first**, per
/// `design@check@message-references-resolve-in-either-tree`. A commit that closes an issue deletes
/// the entry file and names it in the message; against its own tree alone every such message would
/// dangle, and the same message naming the file by a bare path points at what its commit deletes.
/// An extension's rules have no such asymmetry: what a message says about its subject is judged
/// against what the commit's own tree holds, and nothing about the parent bears on it.
fn judge_message(
    text: &str,
    primary: &Assembly,
    primary_entities: &Entities,
    parent: Option<(&Assembly, &Entities)>,
    message_ignored: &HashSet<String>,
) -> Vec<Finding> {
    use check::references::{judge_ids, judge_part, IdTable, Part};
    let model = Model::from_documents(vec![(message_rel(), text.to_string())]);
    let doc = &model.documents()[0];
    let mut out = Vec::new();

    for (_, prepared) in &primary.prepared {
        out.extend(prepared.check_message(doc));
    }

    let anchors = primary.anchors();
    // **A message's path spans are asked of git too.** Each tree's ignore answers cover the spans
    // of its own documents, so a span only the message holds would be asserted where an ignore
    // rule exempts it, as `check` never asserts one in a file, per
    // `design@model@ignored-targets-are-not-asserted`.
    let primary_ignored: HashSet<String> =
        primary.ignored.union(message_ignored).cloned().collect();
    let mut primary_inputs = primary.inputs();
    primary_inputs.ignored = &primary_ignored;
    let (mine, _) = judge_part(
        model.documents(),
        primary_entities,
        &anchors,
        &primary_inputs,
        Part::References,
    );
    let (mut lints, _) = judge_part(
        model.documents(),
        primary_entities,
        &anchors,
        &primary_inputs,
        Part::Lints,
    );
    let references = match parent {
        None => mine,
        Some((other, other_entities)) => {
            let other_anchors = other.anchors();
            let other_ignored: HashSet<String> =
                other.ignored.union(message_ignored).cloned().collect();
            let mut other_inputs = other.inputs();
            other_inputs.ignored = &other_ignored;
            let (theirs, _) = judge_part(
                model.documents(),
                other_entities,
                &other_anchors,
                &other_inputs,
                Part::References,
            );
            let (their_lints, _) = judge_part(
                model.documents(),
                other_entities,
                &other_anchors,
                &other_inputs,
                Part::Lints,
            );
            // **One finding per occurrence, as one tree reports it.** Each tree's lints hold one
            // finding per occurrence of a span, two equal findings for a span repeated on a line.
            // Both trees judge the same text, so a finding equal to one of the commit's own tree
            // comes as many times from each: the parent's copies are dropped whole, and a lint
            // only the parent raises keeps all its occurrences. Inserting the parent's lints into
            // the set as they were pushed collapsed those to one. A set, not a search of `lints`:
            // a message holds as many lints as it has lines, and a search per lint made judging
            // one quadratic in its length.
            let own_lints: HashSet<Finding> = lints.iter().cloned().collect();
            lints.extend(their_lints.into_iter().filter(|f| !own_lints.contains(f)));
            // **A finding survives where both trees refuse the same span on the same line,
            // whatever each says about it.** Two trees can refuse one reference for different
            // reasons — one because the anchor is unknown, the other because the id is not
            // defined there — and comparing the findings whole would then intersect to
            // nothing and let a reference that resolves in NEITHER tree pass. The site is the
            // line and the span the finding opens with, which is what both arms name.
            let refused: HashSet<(Option<u32>, String)> = theirs.iter().map(site).collect();
            mine.into_iter()
                .filter(|f| refused.contains(&site(f)))
                .collect()
        }
    };
    out.extend(references);
    out.extend(lints);
    // The lints on ids read both trees in one pass, so an id the two define under different
    // kinds or anchors gets one finding listing every candidate, not one per tree.
    let parent_anchors = parent.map(|(other, _)| other.anchors());
    let mut tables = vec![IdTable::new(primary_entities, &anchors)];
    if let (Some((_, other_entities)), Some(other_anchors)) = (parent, &parent_anchors) {
        tables.push(IdTable::new(other_entities, other_anchors));
    }
    out.extend(judge_ids(model.documents(), &tables));
    out.sort_by_key(|f| f.line);
    out
}

/// git's ignore answers for the path spans of a message, against the anchors of its commit's
/// tree and of its first parent's: the trees' own batches ask only of their documents' spans.
/// Asked of the working tree's rules, as every ignore question of `commits` is.
fn message_ignored(
    root: &Path,
    text: &str,
    primary: &Assembly,
    parent: Option<&Assembly>,
) -> Result<HashSet<String>, String> {
    let model = Model::from_documents(vec![(message_rel(), text.to_string())]);
    let mut queries = crate::model::resolve::ignore_queries(&model, &primary.anchors());
    if let Some(other) = parent {
        queries.extend(crate::model::resolve::ignore_queries(
            &model,
            &other.anchors(),
        ));
    }
    queries.sort();
    queries.dedup();
    crate::git::ignored(root, &queries).map_err(|e| e.to_string())
}

/// The repair of an ignore batch git refuses, for a span of a document or of a message alike.
///
/// git reads each question as a pathspec, and refuses the whole batch over one path that goes
/// through a symbolic link or into a submodule of the working tree, or that opens with pathspec
/// magic, per `issue@model@a-path-git-reads-as-a-pathspec-stops-the-run-or-misreads`: the span
/// that names it is what can change, since the working tree is the tip's. Any other refusal is
/// git's own, and its reason names it.
const IGNORE_REPAIR: &str = "where git's reason names a path beyond a symbolic link or in a \
     submodule, or a pathspec magic, reword the span that names that path, so that git reads it as \
     the path it is; otherwise repair what git's reason names, and run again";

/// What a reference finding is about: the line, and the span it opens with.
///
/// Every finding the reference family writes begins with the backticked span it judged, and
/// that span plus the line is what identifies the site across two trees that phrase their
/// refusals differently. A finding opening with no span is its own site, spelled whole.
fn site(finding: &Finding) -> (Option<u32>, String) {
    let what = &finding.what;
    let span = what
        .strip_prefix('`')
        .and_then(|rest| rest.split_once('`'))
        .map(|(span, _)| span.to_string())
        .unwrap_or_else(|| what.clone());
    (finding.line, span)
}

/// The same findings, named by the commit or the file the message came from.
fn relabelled(findings: Vec<Finding>, label: &str) -> Vec<Finding> {
    findings
        .into_iter()
        .map(|mut f| {
            f.file = PathBuf::from(label);
            f
        })
        .collect()
}

// ---------------------------------------------------------------------------------------
// One commit's tree
// ---------------------------------------------------------------------------------------

/// Why a commit's tree could not be assembled at all, and the repair that fits that cause.
///
/// Distinct from a tree that assembled and failed its checks: the first is reported as
/// `its tree does not load`, which is what a commit whose manifest the tip checker refuses
/// produces, and the second names its finding count. **The repair is chosen where the cause is
/// known**, since the causes need different ones, per `design@check@finding-names-the-repair`: a
/// manifest the index lacks is staged, one a commit lacks is a history edit, and a git read
/// that fails is repaired as git's reason says.
struct Unloadable {
    why: String,
    repair: &'static str,
}

/// Why a listed file has no bytes where its blob was asked for and git answered nothing: git's
/// object store lacks the blob, as in a repository an object was lost from, and the repair,
/// `NO_BLOB_REPAIR`, is git's, whatever the walk would make of the file.
const NO_BLOB: &str = "git's object store holds no blob for this tree entry";

/// The repair of [`NO_BLOB`]. A file outside the walk carries the statement alone, and `tree`
/// names this repair beside the one of a file the working tree refuses.
const NO_BLOB_REPAIR: &str = "restore the missing object, which `git fsck` names, from a remote \
     or another clone that holds it";

/// The repair of a git read that fails: git's reason names what failed, a missing object, a
/// lock, a corrupt index or a git too old, and nothing in the project is the cause.
const GIT_REPAIR: &str = "repair what git's reason names, and run again";

/// The repair of a refused staging of the generated files, whose reason is git's or names the
/// change to the index that lifts it.
const STAGE_REPAIR: &str = "repair what the reason names, and run again";

impl Unloadable {
    fn git(why: String) -> Self {
        Unloadable {
            why,
            repair: GIT_REPAIR,
        }
    }

    /// The error of a command that cannot run over the staged tree: the statement on the
    /// first line, the repair under it, indented under the `error: ` its caller prints.
    fn refusal(&self, lead: &str) -> String {
        format!("{lead}: {}\n       {}", self.why, self.repair)
    }
}

/// The repair of a manifest a snapshot lacks, or holds in a form the tip checker refuses.
///
/// The index's is staged or corrected. A commit's is a history edit, per
/// `design@check@every-commit-judged-by-the-tip-checker`, unless the range reaches back before the
/// commit that gave the manifest its present form, where the range is what is wrong.
fn manifest_repair(source: Source, absent: bool) -> &'static str {
    match (source, absent) {
        (Source::Index, true) => "stage knowledge-architect.toml",
        (Source::Index, false) => {
            "correct knowledge-architect.toml as the reason says, and stage it"
        }
        (Source::Commit(_), true) => {
            "where the range reaches back before the commit that adds the manifest, start it \
             after that commit; otherwise put the manifest in the branch's first commit, or \
             squash the branch"
        }
        (Source::Commit(_), false) => {
            "where the range reaches back before the commit that gives the manifest a form the \
             tip checker reads, start it after that commit; otherwise put that change in the \
             branch's first commit, or squash the branch"
        }
    }
}

/// Everything read from one snapshot, before the checks run over it.
struct FromTree {
    manifest: Manifest,
    /// Every path of the tree, the symlink and gitlink entries included.
    listing: Vec<PathBuf>,
    /// The symlink and gitlink entries, which are read as no document.
    links: Vec<crate::git::Entry>,
    blobs: std::collections::BTreeMap<PathBuf, String>,
    /// The wanted blobs whose bytes are not UTF-8, as the survey reads them: binary, or text
    /// decoded lossily, as `check` reads the same file on disk. Read by the survey alone; a
    /// walked path here is still a document with no text.
    not_text: std::collections::BTreeMap<PathBuf, crate::model::listing::Outside>,
}

/// Read one snapshot, a commit's tree or the one git's index would commit: its manifest,
/// configured against the extensions, its listing, and
/// the text of everything the walk or the inverse assertion reads.
///
/// Deliberately not the whole tree. A path under a skipped directory, an excluded path and a
/// skipped file are read by no family, and the corpus alone is two megabytes at every commit
/// in the range. **A manifest carrying a phase-1 complaint reads nothing further**: the tree
/// stops at phase 1, and nothing it holds is judged or read as a parent.
fn read_tree(
    root: &Path,
    source: Source,
    generated_extra: &[PathBuf],
    extensions: &mut [Box<dyn Extension>],
) -> Result<FromTree, Unloadable> {
    // **An unmerged index has no tree to commit**: a conflicted merge or rebase leaves it, and
    // git refuses `commit` there. Asked before the listing, which would meet the same paths as
    // `U` rows and name only the first.
    if source == Source::Index {
        let unmerged = crate::git::unmerged(root)
            .map_err(|e| Unloadable::git(format!("its unmerged paths could not be listed: {e}")))?;
        if !unmerged.is_empty() {
            let names: Vec<String> = unmerged
                .iter()
                .map(|p| crate::model::escaped(&p.display().to_string()))
                .collect();
            return Err(Unloadable {
                why: format!("the index holds unmerged paths: {}", names.join(", ")),
                repair: "resolve each conflict and stage the result, or abort the merge or the \
                         rebase that left them",
            });
        }
    }
    let entries: Vec<crate::git::Entry> = crate::git::snapshot_entries(root, source)
        .map_err(|e| Unloadable::git(format!("its tree could not be listed: {e}")))?
        .iter()
        .map(crate::git::SnapshotEntry::entry)
        .collect();
    let listing: Vec<PathBuf> = entries.iter().map(|e| e.rel.clone()).collect();
    let links: Vec<crate::git::Entry> = entries
        .iter()
        .filter(|e| e.kind != crate::git::EntryKind::File)
        .cloned()
        .collect();
    let files: Vec<PathBuf> = entries
        .into_iter()
        .filter(|e| e.kind == crate::git::EntryKind::File)
        .map(|e| e.rel)
        .collect();
    let manifest_rel = PathBuf::from(MANIFEST_NAME);
    // Asked of the listing first: git names a blob for a path the snapshot does not hold, the
    // empty blob for an intent-to-add entry, and a manifest read from it would be an empty one.
    if !files.contains(&manifest_rel) {
        return Err(Unloadable {
            why: format!("its tree holds no {MANIFEST_NAME}"),
            repair: manifest_repair(source, true),
        });
    }
    let declaration = crate::git::blobs(root, source, std::slice::from_ref(&manifest_rel))
        .map_err(|e| Unloadable::git(format!("its {MANIFEST_NAME} could not be read: {e}")))?;
    let Some(text) = declaration.get(&manifest_rel) else {
        return Err(Unloadable {
            why: format!("its tree holds no {MANIFEST_NAME}"),
            repair: manifest_repair(source, true),
        });
    };
    let mut manifest = Manifest::parse(root, text).map_err(|e| Unloadable {
        why: format!("its {MANIFEST_NAME} does not parse: {e}"),
        repair: manifest_repair(source, false),
    })?;
    // Configured against this commit's own manifest, so the files its extensions generate
    // are left out of its walk and read as committed, and a table it refuses is its phase 1.
    crate::extension::configure(&mut manifest, extensions);
    if !manifest.complaints().is_empty() {
        return Ok(FromTree {
            manifest,
            listing,
            links,
            blobs: Default::default(),
            not_text: Default::default(),
        });
    }
    let walk = manifest.walk();
    let mut wanted: Vec<PathBuf> = files
        .iter()
        .filter(|rel| !crate::model::listing::skipped(rel, walk) && !walk.skip_files.contains(rel))
        .cloned()
        .collect();
    for rel in generated_extra {
        if listing.contains(rel) {
            wanted.push(rel.clone());
        }
    }
    // The installed copies define the installed entities whatever the walk rows say, as
    // `Model::build` reads them off the disk, so a skip over the installer's namespace leaves
    // `check` and `commits` judging one tree alike. The index's snapshot compares every
    // installed file's bytes, as `check` does on disk, so it reads every one, skipped or not.
    wanted.extend(
        files
            .iter()
            .filter(|rel| {
                manifest.owned(rel)
                    && (source == Source::Index || rel.extension().is_some_and(|e| e == "md"))
            })
            .cloned(),
    );
    wanted.sort();
    wanted.dedup();
    let mut blobs = std::collections::BTreeMap::new();
    let mut not_text = std::collections::BTreeMap::new();
    for (rel, bytes) in crate::git::blob_bytes(root, source, &wanted)
        .map_err(|e| Unloadable::git(format!("its blobs could not be read: {e}")))?
    {
        match String::from_utf8(bytes) {
            Ok(text) => {
                blobs.insert(rel, text);
            }
            Err(e) => {
                not_text.insert(
                    rel,
                    crate::model::listing::Outside::from_bytes(e.as_bytes()),
                );
            }
        }
    }
    Ok(FromTree {
        manifest,
        listing,
        links,
        blobs,
        not_text,
    })
}

/// How much of a tree the caller needs.
///
/// A tree used only to resolve a message's references needs its entity table and the facts a
/// path reference asks about, and nothing else: no release parsed, no family run. That is
/// every parent tree, and parsing the corpus is the single largest cost in the run.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Depth {
    /// The entity table and the path facts. Nothing is judged.
    References,
    /// Everything: every check of `check`, and each extension prepared for the snapshot with the
    /// caller's purpose.
    Judged,
    /// Phases 1 to 3, the gate a writer passes before it writes, and the snapshot kept for the
    /// writer. No extension is prepared: the writer prepares them for what it generates.
    Gated,
}

/// Assemble one tree into a model, and judge it as far as `depth` asks.
///
/// **An extension is prepared for a snapshot, not for a checkout**, with `purpose`. A check
/// whose subject is filesystem state has no subject in a snapshot, and one that would read the
/// whole archive at every step of the range decides nothing about whether a message's
/// references resolve, which is the question a commit's assembly exists to answer; the
/// extension leaves both out, per `design@check@every-commit-judged-by-the-tip-checker`.
///
/// **The index's snapshot is judged as `check` would judge a clean checkout of it**: its
/// installed files are compared against the running binary's shipped set, and the files git
/// both tracks and ignores are reported, since the index is what that question is about.
fn commit_tree(
    root: &Path,
    source: Source,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
    depth: Depth,
    purpose: Purpose,
) -> Result<Assembly, Unloadable> {
    let read = read_tree(root, source, &[], extensions)?;
    let manifest = read.manifest;
    if !manifest.complaints().is_empty() {
        // Phase 1 failed, so no blob was read and nothing is assembled: the tree is judged no
        // further, and it serves as no parent, per
        // `design@check@message-references-resolve-in-either-tree`; the range walk keeps no parent
        // for it. The early return in `read_tree` changes no verdict and is what saves reading the
        // blobs.
        let model = Model::from_documents(Vec::new());
        let survey = crate::model::listing::from_listing(&manifest, &model, &[], &[], |_| {
            crate::model::listing::Outside::Missing
        });
        let trouble = if depth != Depth::References {
            manifest.complaints().to_vec()
        } else {
            Vec::new()
        };
        let report = (depth != Depth::References).then(|| {
            check::Report::stopped(
                check::Stop {
                    phase: check::Phase::Resolution,
                    findings: trouble.clone(),
                },
                &model,
            )
        });
        return Ok(Assembly {
            manifest,
            model,
            prepared: Vec::new(),
            committed: HashMap::new(),
            configs: HashMap::new(),
            configs_not_text: HashSet::new(),
            survey,
            ignored: HashSet::new(),
            tracked_and_ignored: Vec::new(),
            trouble,
            stopped: Some(check::Phase::Resolution),
            compare_installed: false,
            shipped: Vec::new(),
            judged: judged_of(source),
            report,
            snapshot: None,
        });
    }
    // The commit's own listing places its entry anchors, so the register options read below
    // and the ignore questions asked of this tree follow the homes of the tree it commits rather
    // than the working tree's.
    let anchors = Anchors::of(&manifest, &read.listing);
    let generated = crate::model::generated::generated_paths(&manifest, &read.listing);
    // The walk reads through no symlink and no gitlink, so the files alone are walked.
    let files: Vec<PathBuf> = read
        .listing
        .iter()
        .filter(|rel| !read.links.iter().any(|e| e.rel == **rel))
        .cloned()
        .collect();
    // An installed file leaves the walk as a generated one does, per
    // `design@check@installed-files-outside-the-walk`. A commit's installed files are not compared, but
    // `read_tree` reads their blobs with every other unskipped file, and the survey keeps their
    // text.
    let installed: Vec<PathBuf> = files
        .iter()
        .filter(|f| manifest.owned(f))
        .cloned()
        .collect();
    let mut excluded = generated.clone();
    excluded.extend(installed.iter().cloned());
    let walked =
        crate::model::listing::live_files(Path::new(""), manifest.walk(), &files, &excluded);
    let mut blobs = read.blobs;
    // The generated indexes and the per-instance options sit outside the walk and are read by
    // the caller, exactly as `check` reads them off the filesystem.
    let mut extra: Vec<PathBuf> = generated.iter().cloned().collect();
    for (_, _, home) in anchors.instances() {
        extra.push(home.config.clone());
    }
    let mut not_text = read.not_text;
    extra.retain(|rel| {
        read.listing.contains(rel) && !blobs.contains_key(rel) && !not_text.contains_key(rel)
    });
    // Read as bytes, so a file whose bytes are not UTF-8 is kept as such, as `read_tree` keeps
    // one, rather than read as absent: a register's options read as absent would declare no
    // group.
    for (rel, bytes) in crate::git::blob_bytes(root, source, &extra)
        .map_err(|e| Unloadable::git(format!("its generated files could not be read: {e}")))?
    {
        match String::from_utf8(bytes) {
            Ok(text) => {
                blobs.insert(rel, text);
            }
            Err(e) => {
                not_text.insert(
                    rel,
                    crate::model::listing::Outside::from_bytes(e.as_bytes()),
                );
            }
        }
    }

    let mut docs = Vec::new();
    let mut missing = Vec::new();
    for rel in &walked {
        match blobs.get(rel) {
            Some(text) => docs.push((rel.clone(), text.clone())),
            // A tree entry the batch answered nothing for is a blob whose bytes are not UTF-8,
            // or one git's object store does not hold. It is kept as an empty document carrying
            // the reason, the way `Model::build` keeps an unreadable file, so it is reported
            // rather than silently absent.
            None => missing.push(rel.clone()),
        }
    }
    let mut model = Model::from_documents_under(docs, checker);
    // The commit's own installed copies define its installed entities, so a commit is judged
    // against the set it holds rather than the one the running binary ships.
    model.set_installed(
        installed
            .iter()
            .filter(|rel| rel.extension().is_some_and(|e| e == "md"))
            .filter_map(|rel| blobs.get(rel).map(|text| (rel.clone(), text.clone())))
            .collect(),
    );
    // The two causes need two repairs, per `design@check@finding-names-the-repair`: bytes that
    // are not UTF-8 are a file to re-encode or to keep out of the walk, and an absent object is
    // the repository's to restore.
    for rel in missing {
        if not_text.contains_key(&rel) {
            model.push_not_utf8(rel);
        } else {
            model.push_unreadable(rel, NO_BLOB.to_string(), NO_BLOB_REPAIR.to_string());
        }
    }

    let judging = depth != Depth::References;

    let mut committed = HashMap::new();
    for rel in extra_generated(&generated) {
        if let Some(text) = blobs.get(&rel) {
            committed.insert(rel, text.clone());
        }
    }
    let mut configs = HashMap::new();
    let mut configs_not_text = HashSet::new();
    for (_, _, home) in anchors.instances() {
        if let Some(text) = blobs.get(&home.config) {
            configs.insert(home.config.clone(), text.clone());
        } else if not_text.contains_key(&home.config) {
            configs_not_text.insert(home.config.clone());
        }
    }
    let survey =
        crate::model::listing::from_listing(&manifest, &model, &read.listing, &read.links, |rel| {
            match blobs.get(rel) {
                // The same binary test `check` applies on disk: a NUL is valid UTF-8, so a blob
                // that decoded may still be binary.
                Some(text) => crate::model::listing::Outside::from_bytes(text.as_bytes()),
                None => not_text.get(rel).cloned().unwrap_or_else(|| {
                    crate::model::listing::Outside::Unreadable(NO_BLOB.to_string())
                }),
            }
        });
    // **The ignore rules are the working tree's.** `git check-ignore` reads the `.gitignore`
    // files that are on disk and has no form that asks a historical tree, so a commit whose
    // ignore rules differ from today's is judged against today's. What that can cost is a
    // path reference asserted where the commit's own rules would have exempted it — a finding
    // rather than a silence, which fails the range; a branch that changes its ignore rules
    // orders its commits for it, per `design@check@a-commit-message-is-a-document`.
    let queries = crate::model::resolve::ignore_queries(&model, &anchors);
    let ignored = crate::git::ignored(root, &queries).map_err(|e| Unloadable {
        why: format!("its ignore rules could not be asked: {e}"),
        repair: IGNORE_REPAIR,
    })?;
    let staged = source == Source::Index;
    // A commit holds no index, so nothing is both tracked and ignored in a tree; the index's
    // snapshot is the index, and the question is asked of it as `check` asks it.
    let tracked_and_ignored = if staged {
        crate::git::tracked_and_ignored(root).map_err(|e| {
            Unloadable::git(format!(
                "its tracked and ignored files could not be listed: {e}"
            ))
        })?
    } else {
        Vec::new()
    };
    let shipped = if staged {
        crate::agents::shipped(&manifest)
    } else {
        Vec::new()
    };

    let mut assembly = Assembly {
        manifest,
        model,
        prepared: Vec::new(),
        committed,
        configs,
        configs_not_text,
        survey,
        ignored,
        tracked_and_ignored,
        trouble: Vec::new(),
        stopped: None,
        compare_installed: staged,
        shipped,
        judged: judged_of(source),
        report: None,
        snapshot: None,
    };
    if !judging {
        return Ok(assembly);
    }
    // The phases, as `check` runs them: a tree that stops in one of the first three carries
    // that phase's findings as its trouble and is judged no further.
    let report = match check::foundation(&assembly.model, &assembly.manifest, &assembly.inputs()) {
        Err(stop) => {
            assembly.stopped = Some(stop.phase);
            check::Report::stopped(stop, &assembly.model)
        }
        Ok(()) if depth == Depth::Gated => {
            assembly.snapshot = Some(Snapshot::new(root, source, read.listing, blobs));
            return Ok(assembly);
        }
        Ok(()) => {
            // The foundation passed, so each extension reads what it needs from this
            // snapshot's git objects now, and a tree that stopped earlier read nothing.
            let snapshot = Snapshot::new(root, source, read.listing, blobs);
            for extension in extensions.iter_mut() {
                let prepared = extension
                    .prepare(
                        &assembly.manifest,
                        &assembly.model,
                        Tree::Snapshot(&snapshot),
                        purpose,
                    )
                    .map_err(|why| Unloadable {
                        why,
                        repair: super::EXTENSION_REPAIR,
                    })?;
                assembly.prepared.push((extension.checks(), prepared));
            }
            assembly.snapshot = Some(snapshot);
            let with: Vec<(&[&'static str], &dyn Prepared)> = assembly
                .prepared
                .iter()
                .map(|(c, p)| (*c, p.as_ref()))
                .collect();
            check::run_with(
                &assembly.model,
                &assembly.manifest,
                &assembly.inputs(),
                &with,
            )
        }
    };
    assembly.trouble = report.findings.clone();
    assembly.report = Some(report);
    Ok(assembly)
}

/// `check --staged`: every check, over the tree git's index would commit, printed as `check`
/// prints, with a `tree:` line saying which tree was walked.
///
/// A staged tree that cannot be assembled, an unmerged index among them, is a could-not-run.
pub(super) fn check_staged(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let root = manifest.root();
    let (inside, named) = crate::model::listing::snapshot_checker_dirs(root, checker)
        .map_err(|e| format!("cannot read the project root: {e}"))?;
    let inside: Vec<&Path> = inside.iter().map(PathBuf::as_path).collect();
    let assembly = commit_tree(
        root,
        Source::Index,
        &inside,
        extensions,
        Depth::Judged,
        Purpose::Check,
    )
    .map_err(|e| e.refusal("the staged tree cannot be judged"))?;
    let mut report = assembly
        .report
        .expect("a tree assembled at `Depth::Judged` carries its report");
    report.structure.checker_sources = named;
    // A session that runs this before staging anything judges the previous commit's tree,
    // and is told so rather than reading a pass as a verdict on its edits.
    let tree = if crate::git::index_differs_from_head(root).map_err(|e| e.to_string())? {
        "staged"
    } else {
        "staged (nothing staged: the tree of HEAD)"
    };
    super::print_report(&report, &assembly.model, Some(tree));
    Ok(if report.failed() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// `index --staged`: write each generated file the tree git's index would commit needs into the
/// index, and touch no working-tree file.
///
/// **A writer refuses over an incomplete model**, as `index` does over the working tree: the
/// staged tree is gated on phases 1 to 3, and a stop writes nothing. Every destination is checked
/// against the staged listing before any is written: a generated path the index holds as a
/// symlink or a gitlink, or whose directory the staged tree does not hold, is refused. Every entry
/// is then staged in one locked write, so a refusal of git's lock leaves the index as it was.
/// Each of those refusals is exit 2, which promises an index as the caller left it.
pub(super) fn index_staged(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let root = manifest.root();
    let (inside, _) = crate::model::listing::snapshot_checker_dirs(root, checker)
        .map_err(|e| format!("cannot read the project root: {e}"))?;
    let inside: Vec<&Path> = inside.iter().map(PathBuf::as_path).collect();
    let assembly = commit_tree(
        root,
        Source::Index,
        &inside,
        extensions,
        Depth::Gated,
        Purpose::Index,
    )
    .map_err(|e| e.refusal("the staged tree cannot be read"))?;
    if let Some(report) = assembly
        .report
        .as_ref()
        .filter(|_| assembly.stopped.is_some())
    {
        return Err(format!(
            "the staged model is incomplete: {}. `{} check --staged` reports them. Nothing \
             was written.",
            report.phase.stop_line(report.findings.len()),
            manifest.command()
        ));
    }
    let snapshot = assembly
        .snapshot
        .as_ref()
        .expect("a gated tree that passed the gate keeps its snapshot");
    let generated = super::generated_list(
        &assembly.manifest,
        &assembly.model,
        &assembly.inputs(),
        Tree::Snapshot(snapshot),
        extensions,
    )
    .map_err(|why| {
        Unloadable {
            why,
            repair: super::EXTENSION_REPAIR,
        }
        .refusal("the staged tree's generated files cannot be computed")
    })?;
    // A generated path the index holds as a symlink or a gitlink needs no refusal here: phase 2
    // reports every such entry no walk row keeps, so the gate above has stopped, and one a row
    // keeps is refused by `git::stage_generated`, which asks the index itself.
    for (rel, _) in &generated {
        let dir = rel.parent().unwrap_or(Path::new(""));
        if !dir.as_os_str().is_empty() && !assembly.survey.directories.contains(dir) {
            return Err(format!(
                "{}: the directory this file is generated into is not in the staged tree. \
                 Nothing was written.\n       stage a file under that directory, or configure \
                 the extension that generates this file to write it into a directory the staged \
                 tree holds",
                dir.display()
            ));
        }
    }
    let mut stale = Vec::new();
    for (rel, text) in generated {
        if assembly.committed.get(&rel) == Some(&text) {
            outln!("{:<40} already current", rel.display());
        } else {
            stale.push((rel, text));
        }
    }
    // One repair for every cause: a refusal of an entry's kind names the change to the index
    // that lifts it, and a failure of git carries git's reason, which names its own, as for a
    // held index lock. Git's reason may end with a period of its own.
    crate::git::stage_generated(root, &stale).map_err(|e| {
        format!(
            "{}. Nothing was staged.\n       {STAGE_REPAIR}",
            e.to_string().trim_end().trim_end_matches('.')
        )
    })?;
    for (rel, _) in &stale {
        outln!("{:<40} staged", rel.display());
    }
    // **A working copy out of date for the working tree is named.** No working-tree file is
    // written, per `design@check@index-staged-write`, so after an entry removed with `git rm` the
    // working copy still lists it, and a session that edits the register next works from it. The
    // copy is compared with what `index` would write for the working tree, not with what was
    // staged: in the partial commit this command serves, the working copy rightly differs from the
    // staged file. Where the working tree's model is incomplete, `index` itself would refuse, so
    // nothing is named. A copy that cannot be read is out of date.
    for rel in super::working_generated(manifest, checker, extensions)
        .unwrap_or_default()
        .into_iter()
        .filter(|(rel, text)| std::fs::read_to_string(root.join(rel)).ok().as_ref() != Some(text))
        .map(|(rel, _)| rel)
    {
        outln!(
            "{:<40} the working copy is out of date for the working tree: `{} index` rewrites it",
            rel.display(),
            manifest.command()
        );
    }
    Ok(ExitCode::SUCCESS)
}

/// The generated files the tree git's index would commit needs, for `check --fix`, which compares
/// them with the working tree's.
///
/// `Ok(None)` in the three states where no staged tree is there to commit: an index holding an
/// unmerged path, a staged tree holding no manifest, and one that stops in phases 1 to 3. **Every other failure is an
/// error**, a git read or an extension that cannot prepare over the snapshot among them: the
/// comparison guards a partial commit against the generated files of the wrong tree, and a
/// comparison skipped in silence lets that commit through.
pub(super) fn staged_generated(
    root: &Path,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<Option<Vec<(PathBuf, String)>>, String> {
    let unmerged = crate::git::unmerged(root).map_err(|e| {
        Unloadable::git(format!("its unmerged paths could not be listed: {e}"))
            .refusal("the staged tree cannot be read")
    })?;
    if !unmerged.is_empty() {
        return Ok(None);
    }
    // A staged tree holding no manifest is no project: phase 1 has nothing to read, and a commit
    // of it carries no generated file to get wrong. `init` over a repository with staged changes
    // meets it, as does a manifest written and not yet staged.
    let listed = crate::git::snapshot_entries(root, crate::git::Source::Index).map_err(|e| {
        Unloadable::git(format!("its tree could not be listed: {e}"))
            .refusal("the staged tree cannot be read")
    })?;
    if !listed.iter().any(|e| {
        let entry = e.entry();
        entry.kind == crate::git::EntryKind::File && entry.rel == Path::new(MANIFEST_NAME)
    }) {
        return Ok(None);
    }
    let (inside, _) = crate::model::listing::snapshot_checker_dirs(root, checker)
        .map_err(|e| format!("cannot read the project root: {e}"))?;
    let inside: Vec<&Path> = inside.iter().map(PathBuf::as_path).collect();
    let assembly = commit_tree(
        root,
        Source::Index,
        &inside,
        extensions,
        Depth::Gated,
        Purpose::Index,
    )
    .map_err(|e| e.refusal("the staged tree cannot be read"))?;
    if assembly.stopped.is_some() {
        return Ok(None);
    }
    let snapshot = assembly
        .snapshot
        .as_ref()
        .expect("a gated tree that passed the gate keeps its snapshot");
    super::generated_list(
        &assembly.manifest,
        &assembly.model,
        &assembly.inputs(),
        Tree::Snapshot(snapshot),
        extensions,
    )
    .map(Some)
    .map_err(|why| {
        Unloadable {
            why,
            repair: super::EXTENSION_REPAIR,
        }
        .refusal("the staged tree's generated files cannot be computed")
    })
}

/// Every generated file whose committed bytes a check compares against: each file an
/// extension generates, and one index per file-register instance.
fn extra_generated(generated: &HashSet<PathBuf>) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = generated.iter().cloned().collect();
    out.sort();
    out.dedup();
    out
}

// ---------------------------------------------------------------------------------------
// The commands
// ---------------------------------------------------------------------------------------

/// One commit's place in the run, as the summary block names it.
enum Outcome {
    /// Nothing of the run's findings belongs to it: its tree, its message and its citations
    /// all passed.
    Passed,
    /// At least one finding of the run belongs to it. `why` names every source, joined by
    /// `; `: its tree does not load, its tree carries findings, its message carries findings,
    /// or it cites the range by SHA. Where its tree stopped before the last phase, `why` says
    /// its message was judged against nothing.
    Failed { why: String },
}

impl Outcome {
    /// Passed when nothing failed, and failed with every cause otherwise.
    fn of(causes: Vec<String>) -> Self {
        if causes.is_empty() {
            Outcome::Passed
        } else {
            Outcome::Failed {
                why: causes.join("; "),
            }
        }
    }
}

/// Judge every commit of a range, its tree and its message, alike.
///
/// **No commit of the range is judged apart**, the last one included: a tree that fails is a
/// finding naming the commit, whichever commit it is, per
/// `design@check@every-commit-judged-by-the-tip-checker`. Exit 1 on a finding, `Err` on a range
/// that does not resolve.
pub fn commits(
    manifest: &Manifest,
    range: &str,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let root = manifest.root();
    let shas = crate::git::rev_list(root, range).map_err(|e| {
        format!(
            "{range} does not resolve to a range of commits: {e}\n       fetch each revision \
             the range names, or correct the name git's reason gives"
        )
    })?;
    if shas.is_empty() {
        outln!("\ncommits: no commit is in range {range}");
        outln!("{}", verdict(0));
        return Ok(ExitCode::SUCCESS);
    }
    // The checker's own directories, project-relative, so a per-commit model reads the tool's
    // own fixtures as data the way `check` does.
    let checker_rel: Vec<PathBuf> = checker
        .iter()
        .filter_map(|c| c.strip_prefix(root).ok().map(Path::to_path_buf))
        .collect();
    let checker_rel: Vec<&Path> = checker_rel.iter().map(PathBuf::as_path).collect();

    let mut summary: Vec<(String, Outcome)> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut previous: Option<(String, Assembly, Entities)> = None;
    // Each generated file a commit's tree fails on, with the commits that fail on it.
    let mut generated: BTreeMap<PathBuf, usize> = BTreeMap::new();

    for sha in &shas {
        let short = &sha[..7.min(sha.len())];
        let assembled = commit_tree(
            root,
            Source::Commit(sha),
            &checker_rel,
            extensions,
            Depth::Judged,
            Purpose::Commit,
        );
        let tree = match assembled {
            Ok(tree) => tree,
            Err(Unloadable { why, repair }) => {
                // **Every commit of the range must load under the tip checker.** A branch that
                // changes the manifest format puts that change in its first commit, or is squashed,
                // per `design@check@every-commit-judged-by-the-tip-checker`; a git read that failed
                // is no fault of the branch, and its repair is git's.
                findings.push(Finding::in_file(
                    format!("commit {short}"),
                    format!("this commit's tree does not load under the tip checker: {why}"),
                    repair,
                ));
                summary.push((
                    short.to_string(),
                    Outcome::Failed {
                        why: format!("its tree does not load: {why}"),
                    },
                ));
                previous = None;
                continue;
            }
        };
        let trouble = tree.trouble.len();
        let mut causes = Vec::new();
        if trouble > 0 {
            for path in tree
                .trouble
                .iter()
                .filter(|f| crate::check::generated::is_generated_finding(f))
                .map(|f| f.file.clone())
                .collect::<BTreeSet<_>>()
            {
                *generated.entry(path).or_default() += 1;
            }
            // **Every commit of the range must pass under the tip checker**, so the tree's own
            // findings are the run's, each named by the commit and by the file inside it.
            findings.extend(tree.trouble.iter().cloned().map(|mut f| {
                f.file = PathBuf::from(format!("commit {short}: {}", f.file.display()));
                f
            }));
            if let Some(phase) = tree.stopped {
                let mut why = format!(
                    "its tree stops at phase {} with {trouble} finding(s); its message was \
                     judged against nothing",
                    phase.number()
                );
                // The citation scan needs no entity table and no verdict: the manifest, the
                // message, the documents and the range's SHAs. A tree stopped at phase 2 or 3
                // has all four, so its citations are reported in this run rather than in the
                // one after its tree is repaired. A tree stopped at phase 1 read no document,
                // and its manifest did not resolve, so the option is not read from it, per
                // `design@check@branch-shas-are-refused`.
                if phase != check::Phase::Resolution && tree.manifest.refuses_branch_shas() {
                    let message =
                        crate::git::commit_message(root, sha).map_err(|e| e.to_string())?;
                    let cited = branch_sha_findings(short, &message, &tree, &shas, range);
                    if !cited.is_empty() {
                        why.push_str(&format!(
                            "; it cites the range by SHA {} time(s)",
                            cited.len()
                        ));
                    }
                    findings.extend(cited);
                }
                summary.push((short.to_string(), Outcome::Failed { why }));
                if phase == check::Phase::Resolution {
                    // Phase 1 failed and nothing of the tree was read, so it serves as no
                    // parent. An empty tree kept as a parent would not be the same: a finding
                    // is kept only when the parent refuses the reference too, and an empty
                    // tree accepts some a real one refuses, such as an escape naming a path
                    // the tree holds.
                    previous = None;
                } else {
                    // An incomplete table would refuse a reference into what the walk could
                    // not read, so the message is judged by nothing. The tree still serves as
                    // the next commit's parent: its entities are read, not its verdict.
                    let entities = Entities::build(&tree.model, &tree.anchors());
                    previous = Some((sha.clone(), tree, entities));
                }
                continue;
            }
            // The tree reached the last phase, so its message is judged below and its own
            // findings join this cause.
            causes.push(format!("its tree fails {trouble} finding(s)"));
        }

        // The parent model is the previous commit's where the walk followed the parent chain,
        // a failed commit's included, and is built once otherwise — at the range's first
        // commit, and wherever the previous commit is not this one's first parent.
        let first_parent = crate::git::rev_parse(root, &format!("{sha}^"));
        let parent_owned = match (&previous, &first_parent) {
            (Some((seen, _, _)), Some(parent)) if seen == parent => None,
            (_, Some(parent)) => {
                commit_tree(
                    root,
                    Source::Commit(parent),
                    &checker_rel,
                    extensions,
                    Depth::References,
                    Purpose::Commit,
                )
                .ok()
                // A parent whose phase 1 fails was not read, and serves as no parent.
                .filter(|a| a.stopped != Some(check::Phase::Resolution))
                .map(|a| {
                    let e = Entities::build(&a.model, &a.anchors());
                    (a, e)
                })
            }
            (_, None) => None,
        };
        let parent = match (&previous, &first_parent, &parent_owned) {
            (Some((seen, a, e)), Some(p), _) if seen == p => Some((a, e)),
            (_, _, Some((a, e))) => Some((a, e)),
            _ => None,
        };

        // **The message as the commit holds it, cleaned of nothing.** Git applied its own
        // cleanup before the commit existed, so a `#` line here is a line the author wrote and
        // a second pass over it would take bytes of a commit out of the regime.
        let message = crate::git::commit_message(root, sha).map_err(|e| e.to_string())?;
        let entities = Entities::build(&tree.model, &tree.anchors());
        // A batch git refuses, as a span beyond a symbolic link makes it, fails this commit
        // alone, with git's reason, and its message is judged with no ignore answer: one span
        // must not stop the judgement of every commit of the range, as
        // `issue@model@a-path-git-reads-as-a-pathspec-stops-the-run-or-misreads` records of a
        // tree's own batch. **The refusal is a finding**, as every other cause of a failed
        // commit is: the verdict and the exit code count findings alone, so a cause with none
        // would print a failed commit above a pass.
        let ignored = match message_ignored(root, &message, &tree, parent.map(|(a, _)| a)) {
            Ok(ignored) => ignored,
            Err(why) => {
                causes.push("git refused the ignore question of its message".to_string());
                findings.push(Finding::in_file(
                    format!("commit {short}"),
                    format!(
                        "git refused to say whether the paths this commit's message names are \
                         ignored, so its message was judged with no ignore answer: {why}"
                    ),
                    IGNORE_REPAIR,
                ));
                HashSet::new()
            }
        };
        let found = judge_message(&message, &tree, &entities, parent, &ignored);
        if !found.is_empty() {
            causes.push(format!("its message carries {} finding(s)", found.len()));
        }
        findings.extend(relabelled(found, &format!("commit {short}")));
        // **Read from this commit's own manifest**, as everything a commit is judged against
        // is, per `design@check@a-commit-message-is-a-document`.
        let cited = if tree.manifest.refuses_branch_shas() {
            branch_sha_findings(short, &message, &tree, &shas, range)
        } else {
            Vec::new()
        };
        if !cited.is_empty() {
            causes.push(format!("it cites the range by SHA {} time(s)", cited.len()));
        }
        findings.extend(cited);
        summary.push((short.to_string(), Outcome::of(causes)));
        previous = Some((sha.clone(), tree, entities));
    }

    let passed = summary
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Passed))
        .count();
    let failed = summary.len() - passed;
    let total = match summary.len() {
        1 => "1 commit".to_string(),
        n => format!("{n} commits"),
    };
    outln!("\ncommits in {range}: {total}, {passed} passed, {failed} failed");
    for (short, outcome) in &summary {
        match outcome {
            Outcome::Passed => outln!("  {short} passed"),
            Outcome::Failed { why } => outln!("  {short} failed: {why}"),
        }
    }
    // **A generated file several commits fail on is one repair**, which a finding per commit does
    // not show: `index` at the tip rewrites the last commit's file alone. The line names the repair
    // and no cause, per `design@check@finding-names-the-repair`: a branch that changes the
    // generator and an early commit that left the file stale need the same one.
    for (path, count) in generated.iter().filter(|(_, n)| **n > 1) {
        outln!(
            "  {}: {count} commits fail on this generated file; regenerate it at each of them and \
             fold each regeneration into its commit",
            path.display()
        );
    }
    if !findings.is_empty() {
        outln!();
        for finding in &findings {
            outln!("{finding}");
        }
    }
    outln!("{}", verdict(findings.len()));
    Ok(if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Every citation of a commit of the range in `text`: a run of 7 to 64 lowercase hex digits,
/// with no ASCII letter, digit or underscore on either side, that is a prefix of one of `shas`.
/// Each is the line, counted from one, the run as written, and the full SHA it names.
///
/// **Only the range's commits are refused**, per `design@check@branch-shas-are-refused`: they are
/// the ones a rebase merge gives a new SHA. A SHA on the main branch, or of another project, is
/// never a prefix of one of them, except by a collision of its first 7 digits, so a citation of
/// either passes.
fn branch_sha_citations<'a>(text: &str, shas: &'a [String]) -> Vec<(u32, String, &'a str)> {
    fn is_word(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
    let mut out = Vec::new();
    for (i, line) in text.split('\n').enumerate() {
        let bytes = line.as_bytes();
        let mut start = 0;
        while start < bytes.len() {
            if !is_word(bytes[start]) {
                start += 1;
                continue;
            }
            // A word ends at the first byte that is not ASCII alphanumeric or `_`, so both
            // ends of the slice sit on ASCII bytes and the slice is valid UTF-8.
            let mut end = start;
            while end < bytes.len() && is_word(bytes[end]) {
                end += 1;
            }
            let word = &line[start..end];
            let hex = word.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
            // 64 digits is a SHA under SHA-256.
            if hex && (7..=64).contains(&word.len()) {
                if let Some(sha) = shas.iter().find(|s| s.starts_with(word)) {
                    out.push((i as u32 + 1, word.to_string(), sha.as_str()));
                }
            }
            start = end;
        }
    }
    out
}

/// The findings for every citation of a commit of the range in one commit's message and in its
/// tree's documents, each named by the commit and by the site.
fn branch_sha_findings(
    short: &str,
    message: &str,
    tree: &Assembly,
    shas: &[String],
    range: &str,
) -> Vec<Finding> {
    let finding = |file: String, (line, word, sha): (u32, String, &str)| {
        Finding::at(
            file,
            line,
            format!(
                "`{word}` cites commit {}, a commit of the range {range}, by its SHA; a rebase \
                 merge gives that commit a new SHA, and the citation then names nothing",
                &sha[..7.min(sha.len())]
            ),
            "name that commit by its subject; where the word cites a commit outside the \
             range whose first digits are the same, write enough of its digits that no commit \
             of the range starts with them",
        )
    };
    let mut out: Vec<Finding> = branch_sha_citations(message, shas)
        .into_iter()
        .map(|c| finding(format!("commit {short}"), c))
        .collect();
    for doc in tree.model.documents() {
        out.extend(
            branch_sha_citations(&doc.text, shas)
                .into_iter()
                .map(|c| finding(format!("commit {short}: {}", doc.rel.display()), c)),
        );
    }
    out
}

fn verdict(n: usize) -> String {
    match n {
        0 => "PASSED: no findings".to_string(),
        1 => "FAILED: 1 finding above".to_string(),
        n => format!("FAILED: {n} findings above"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A repository under the temporary directory, configured to commit, holding a manifest
    /// committed and, as intent-to-add, a file nothing commits.
    fn repository_with_an_intent_to_add_entry(tag: &str) -> PathBuf {
        let repo = std::env::temp_dir().join(format!(
            "knowledge-history-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).expect("a repository directory");
        let run = |args: &[&str]| {
            crate::git::git(&repo).args(args).output().expect("git");
        };
        run(&["init", "-q"]);
        run(&["config", "user.name", "fixture"]);
        run(&["config", "user.email", "fixture@example.invalid"]);
        std::fs::write(repo.join(MANIFEST_NAME), "[project]\n").expect("a manifest");
        std::fs::write(repo.join("ita.md"), "# Intent to add\n").expect("a file");
        run(&["add", MANIFEST_NAME]);
        run(&["commit", "-qm", "base"]);
        run(&["add", "-N", "ita.md"]);
        repo
    }

    /// The claim: a manifest the index holds only as intent-to-add is no manifest of the staged
    /// tree, since a commit would record none, and the refusal says so rather than reading the
    /// empty blob git names for it, naming the staging as its repair.
    #[test]
    fn an_intent_to_add_manifest_is_no_manifest_of_the_staged_tree() {
        let repo = repository_with_an_intent_to_add_entry("ita-manifest");
        let run = |args: &[&str]| {
            crate::git::git(&repo).args(args).output().expect("git");
        };
        run(&["rm", "-q", "--cached", MANIFEST_NAME]);
        run(&["add", "-N", MANIFEST_NAME]);
        let refused = read_tree(&repo, Source::Index, &[], &mut [])
            .err()
            .expect("no manifest is staged");
        assert_eq!(refused.why, format!("its tree holds no {MANIFEST_NAME}"));
        assert_eq!(refused.repair, "stage knowledge-architect.toml");
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// The claim: a snapshot names a blob only for a path it holds; an intent-to-add entry, which
    /// git names with the empty blob, is not one.
    #[test]
    fn a_snapshot_names_no_blob_for_a_path_it_does_not_hold() {
        let repo = repository_with_an_intent_to_add_entry("ita-object");
        let listing: Vec<PathBuf> = crate::git::snapshot_entries(&repo, Source::Index)
            .expect("the snapshot")
            .into_iter()
            .map(|e| e.entry().rel)
            .collect();
        let snapshot = Snapshot::new(&repo, Source::Index, listing, Default::default());
        assert!(snapshot.object_id(Path::new(MANIFEST_NAME)).is_some());
        assert!(!snapshot.holds(Path::new("ita.md")));
        assert_eq!(snapshot.object_id(Path::new("ita.md")), None);
        let _ = std::fs::remove_dir_all(&repo);
    }

    /// The staged assembly refuses an index holding an unmerged path, naming each path once,
    /// before it reads anything else: a conflicted merge has no tree to commit.
    #[test]
    fn an_unmerged_index_is_refused_naming_each_path_once() {
        let repo = std::env::temp_dir().join(format!(
            "knowledge-history-unmerged-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).expect("a repository directory");
        let run = |args: &[&str]| {
            let _ = crate::git::git(&repo).args(args).accept(1).output();
        };
        run(&["init", "-q"]);
        run(&["config", "user.name", "fixture"]);
        run(&["config", "user.email", "fixture@example.invalid"]);
        for (branch, text) in [(None, "base\n"), (Some("side"), "side\n"), (None, "main\n")] {
            match branch {
                Some(name) => run(&["checkout", "-qb", name]),
                None if text == "main\n" => run(&["checkout", "-q", "-"]),
                None => {}
            }
            std::fs::write(repo.join("c.md"), text).expect("a file");
            std::fs::write(repo.join("d.md"), text).expect("a file");
            run(&["add", "-A"]);
            run(&["commit", "-qm", text.trim()]);
        }
        run(&["merge", "-q", "side"]);
        let refused = read_tree(&repo, Source::Index, &[], &mut [])
            .err()
            .expect("an unmerged index is refused");
        assert_eq!(refused.why, "the index holds unmerged paths: c.md, d.md");
        assert!(refused.repair.starts_with("resolve each conflict"));
        let _ = std::fs::remove_dir_all(&repo);
    }

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@check@checker-source-literals-are-data`.

    /// The claim: the line a finding names is the line of the message, counted from one.
    ///
    /// The message is parsed as a markdown document under a name no reader sees, and the
    /// relabelling replaces the file and nothing else.
    #[test]
    fn relabelling_names_the_commit_and_keeps_the_line() {
        let found = vec![Finding::at("commit-message.md", 4, "what", "action")];
        let out = relabelled(found, "commit abc1234");
        assert_eq!(out[0].location(), "commit abc1234:4");
        assert_eq!(out[0].what, "what");
    }

    /// The claim: a run of 7 to 64 lowercase hex digits bounded by non-word bytes is a citation
    /// exactly when it prefixes a SHA of the range. Mutations: dropping the left or the right
    /// boundary fails the embedded cases, lowering the minimum to 6 fails the six-digit case,
    /// and matching any hex run fails the outside-the-range case.
    #[test]
    fn a_citation_is_a_bounded_hex_prefix_of_a_sha_of_the_range() {
        let shas = vec![
            "0123456789abcdef0123456789abcdef01234567".to_string(),
            "fedcba9876543210fedcba9876543210fedcba98".to_string(),
        ];
        let cited = |text: &str| -> Vec<(u32, String)> {
            branch_sha_citations(text, &shas)
                .into_iter()
                .map(|(l, w, _)| (l, w))
                .collect()
        };
        assert_eq!(cited("see 0123456 here"), vec![(1, "0123456".to_string())]);
        assert_eq!(
            cited("x\n(fedcba98765)."),
            vec![(2, "fedcba98765".to_string())]
        );
        assert_eq!(
            cited("0123456789abcdef0123456789abcdef01234567"),
            vec![(1, "0123456789abcdef0123456789abcdef01234567".to_string())]
        );
        assert_eq!(
            cited("`0123456`, é 0123456-x"),
            vec![(1, "0123456".to_string()), (1, "0123456".to_string())]
        );
        // Six digits, or a run inside a longer word, cite nothing.
        assert!(cited("012345 is short").is_empty());
        assert!(cited("x0123456 and 0123456y and 0123456_z").is_empty());
        // Upper case is not how git prints a SHA.
        assert!(cited("0123456ABC").is_empty());
        // A hex run that prefixes no SHA of the range: main's, or another project's.
        assert!(cited("d2f0ce4 and e98e296").is_empty());
        // A run that shares the first 7 digits and then differs is no prefix.
        assert!(cited("0123456ff").is_empty());
        // A SHA-256 SHA, 64 digits, is a citation of itself.
        let long = vec!["ab".repeat(32)];
        assert_eq!(branch_sha_citations(&"ab".repeat(32), &long).len(), 1);
    }

    #[test]
    fn the_verdict_line_counts_what_was_found() {
        assert_eq!(verdict(0), "PASSED: no findings");
        assert_eq!(verdict(1), "FAILED: 1 finding above");
        assert_eq!(verdict(3), "FAILED: 3 findings above");
    }
}
