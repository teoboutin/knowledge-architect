//! The checks. Each is a pure function from the model to findings.
//!
//! No check reads a file, spawns a process, or knows how the walk works. That is what makes
//! the single walk a property of the design rather than of anyone's care, and it is what lets
//! a check be run against a model assembled in memory.

pub(crate) mod agents;
pub(crate) mod generated;
pub(crate) mod references;
pub(crate) mod registers;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::extension::{Generated, Prepared};
use crate::model::foundation::TreeInputs;
pub use crate::model::foundation::{Phase, Stop};
use crate::model::{Finding, Manifest, Model};

/// What the structural checks looked at, so that finding nothing is distinguishable from
/// looking at nothing.
#[derive(Debug, Default)]
pub struct Structure {
    pub components: usize,
    pub locations: usize,
    /// Anchor-and-register pairs whose home was asserted, and the file-register entries
    /// whose shape was.
    pub instances: usize,
    pub entries: usize,
    /// Distinct entities the table holds, reference occurrences judged against it, and the
    /// relative markdown links resolved.
    pub entities: usize,
    pub references: usize,
    pub links: usize,
    /// How many files the walk read. Not a family's count: it describes the walk every family
    /// read, so it is set whatever was asked for. It is printed because git is the walk, per
    /// `design@model@git-supplies-the-walk`, and two machines answering differently has to be
    /// visible in the output rather than inferred from a finding list.
    pub walked: usize,
    /// The checker's own directory as the model was told it, and how many walked Rust files
    /// sit under it with their string literals read as data. Not a family's count: it describes
    /// the walk every family read, so it is set whatever was asked for.
    pub checker_sources: Vec<std::path::PathBuf>,
    pub checker_files: usize,
}

/// Phases 1 to 3, over everything a check reads.
///
/// `model` runs them, over what they read; this is what a caller holding [`Inputs`] calls.
pub fn foundation(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Result<(), Stop> {
    crate::model::foundation::foundation(model, manifest, &inputs.tree())
}

/// What a whole run found and counted.
pub struct Report {
    /// The phase the findings belong to: `Content` for a run that judged everything, and
    /// the phase that stopped it otherwise, when nothing later was judged.
    pub phase: Phase,
    pub findings: Vec<Finding>,
    pub structure: Structure,
    /// Every check the last phase performed or was asked to: `check`'s, then each
    /// extension's, in the order the `checked:` line prints them.
    pub checks: Vec<&'static str>,
    /// The checks the tree gave no input to, by name. Each is printed as not run rather
    /// than counted, because a count nobody took would read as "nothing found" for a check
    /// that never happened.
    pub not_run: Vec<&'static str>,
    /// Each extension's block of the summary, in the order the extensions were registered.
    pub summaries: Vec<String>,
}

impl Report {
    pub fn failed(&self) -> bool {
        !self.findings.is_empty()
    }

    /// The report of a run that stopped: the stop's findings and nothing later, with what
    /// the walk read still counted, since every phase read it.
    pub(crate) fn stopped(stop: Stop, model: &Model) -> Self {
        let structure = Structure {
            walked: model.documents().len(),
            checker_sources: model.checker_sources().to_vec(),
            checker_files: model.checker_files(),
            ..Structure::default()
        };
        Report {
            phase: stop.phase,
            findings: stop.findings,
            structure,
            checks: Vec::new(),
            not_run: Vec::new(),
            summaries: Vec::new(),
        }
    }
}

/// Everything a check needs that it may not fetch for itself.
///
/// A check is a pure function from the model to findings, so anything requiring the
/// filesystem or the network is resolved by the caller and handed in. Resolving a release may
/// read the archive; reading a generated file reaches outside the walk, which excludes those
/// files by name.
///
/// Non-exhaustive: a caller outside this crate gets one from [`crate::commands::Gathered`], so a new
/// input is not a breaking change for it.
// Non-exhaustive per `design@check@ne-minimal`.
#[non_exhaustive]
pub struct Inputs<'a> {
    /// The generated files as committed, keyed by their project-relative path.
    pub committed: &'a HashMap<PathBuf, String>,
    /// Every `register.toml` beside a register instance, keyed by its project-relative path.
    ///
    /// Read by the caller like the generated files, and for the same reason: a check may not
    /// touch the filesystem, and this one is outside the walk because it is not markdown.
    pub configs: &'a HashMap<PathBuf, String>,
    /// The `register.toml` files beside a register instance whose bytes are not UTF-8, which
    /// `configs` holds no text for.
    pub(crate) configs_not_text: &'a HashSet<PathBuf>,
    /// Every path that exists in the project, files and directories, project-relative. One
    /// listing by the caller answers every question a check has about what is there. It is the
    /// listing of the tree being judged: the working tree under `check`, and a snapshot's
    /// listing under `check --staged` and `commits`.
    pub present: &'a HashSet<PathBuf>,
    /// The subset of `present` that is directories, so a check can assert a path's kind —
    /// a required document is a file, and a reference's trailing slash claims a directory.
    pub directories: &'a HashSet<PathBuf>,
    /// Files the walk does not cover, with their text — excluding the paths the manifest
    /// excludes, which are other projects rather than unchecked files of this one. A state
    /// other than text or binary is a finding.
    pub outside: &'a [(PathBuf, crate::model::listing::Outside)],
    /// The path spellings the ignore rules cover, out of every spelling a path reference in
    /// this run could ask about.
    ///
    /// One `git check-ignore` batch by the caller, keyed by `git::ignore_query`'s spelling, so
    /// a check answers the question with a lookup and spawns nothing. Asking the RULES rather
    /// than what is on disk is what makes the verdict identical on a fresh clone and a built
    /// tree, per `design@model@ignored-targets-are-not-asserted`.
    pub ignored: &'a HashSet<String>,
    /// The files git both tracks and ignores, each of which is a finding.
    pub tracked_and_ignored: &'a [PathBuf],
    /// The files the walk refuses by name, by the name, as the walk refuses it, each of which is a finding.
    pub refused: &'a [PathBuf],
    /// The listing's symlink and gitlink entries no walk row keeps, each of which is a finding.
    pub links: &'a [crate::git::Entry],
    /// The files of the installer's namespace, with what reading each gave, per
    /// `design@check@installed-files-compared-by-bytes`.
    pub installed: &'a [(PathBuf, crate::model::listing::Outside)],
    /// What the installed set must be: every shipped file at its install path, rendered with
    /// the project's command. Empty when the project serves no agent harness.
    pub shipped: &'a [(PathBuf, String)],
    /// The tree this run judges, which decides the command that repairs a generated file.
    pub(crate) judged: Judged,
}

/// The tree a run judges.
///
/// Within one run it fixes the repair of a generated file, so a finding names that one repair,
/// per `design@check@finding-names-the-repair`: the working tree's file is written by `index`,
/// the index's by `index --staged`, and a commit's by a history edit that runs `index`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Judged {
    /// `check`, `check --fix` and every command that reads the working tree.
    WorkingTree,
    /// `check --staged`: the tree git's index would commit.
    Index,
    /// One commit's tree, under `commits`.
    Commit,
}

impl<'a> Inputs<'a> {
    /// What phases 1 to 3 and the resolver read: every input but those phase 4 alone reads.
    pub(crate) fn tree(&self) -> TreeInputs<'a> {
        TreeInputs {
            present: self.present,
            directories: self.directories,
            outside: self.outside,
            ignored: self.ignored,
            tracked_and_ignored: self.tracked_and_ignored,
            refused: self.refused,
            links: self.links,
        }
    }
}

/// `check`'s checks of the last phase, by the name the `checked:` line prints.
///
/// One per module under `check/` that runs in the last phase. Every one runs on every run that
/// reaches the last phase. An extension adds its own, per
/// `design@check@an-extension-plugs-in-through-phased-hooks`.
pub const CHECKS: [&str; 4] = ["agents", "generated", "registers", "references"];

/// Run `check`'s checks of the last phase, over a model `foundation` found complete, with
/// the files each extension generates compared by the `generated` check.
pub(crate) fn run(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    extension_files: &[Generated],
) -> Report {
    let mut findings = Vec::new();
    let mut structure = Structure::default();
    // First, since a stale install explains a reference finding below it: a missing shipped file's
    // entities are unresolved, and the install is the repair of both.
    findings.extend(agents::check(model, manifest, inputs));
    findings.extend(generated::check(model, manifest, inputs, extension_files));
    {
        let (found, counts) = registers::check(model, manifest, inputs);
        findings.extend(found);
        structure.components = counts.components;
        structure.locations = counts.locations;
        structure.instances = counts.instances;
        structure.entries = counts.entries;
    }
    {
        let (found, c) = references::check(model, manifest, inputs);
        findings.extend(found);
        structure.entities = c.entities;
        structure.references = c.references;
        structure.links = c.links;
    }
    structure.walked = model.documents().len();
    structure.checker_sources = model.checker_sources().to_vec();
    structure.checker_files = model.checker_files();

    Report {
        phase: Phase::Content,
        findings,
        structure,
        checks: CHECKS.to_vec(),
        // A project that serves no harness gives the family nothing to read.
        not_run: if manifest.serves_claude() {
            Vec::new()
        } else {
            vec!["agents"]
        },
        summaries: Vec::new(),
    }
}

/// The last phase over `check` and every prepared extension: `check`'s checks, then each
/// extension's, merged into one report.
pub fn run_with(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    extensions: &[(&[&'static str], &dyn Prepared)],
) -> Report {
    let files: Vec<Generated> = extensions
        .iter()
        .flat_map(|(_, p)| p.generated(model, manifest))
        .collect();
    let mut report = run(model, manifest, inputs, &files);
    for (checks, prepared) in extensions {
        let theirs = prepared.check(model, manifest, inputs);
        report.checks.extend(checks.iter().copied());
        report.findings.extend(theirs.findings);
        report.not_run.extend(theirs.not_run);
        report.summaries.push(theirs.summary);
    }
    report
}

#[cfg(test)]
pub(crate) mod testing {
    use std::collections::HashSet;
    use std::path::PathBuf;

    /// The plans layout every project owes at its root, with no plan document in it: the plans
    /// directory, its README, and its two homes with their README and index.
    pub(crate) const PLANS_TREE: [&str; 8] = [
        "docs/plans",
        "docs/plans/README.md",
        "docs/plans/specs",
        "docs/plans/specs/README.md",
        "docs/plans/specs/index.md",
        "docs/plans/milestones",
        "docs/plans/milestones/README.md",
        "docs/plans/milestones/index.md",
    ];

    /// The directories a flat listing implies: every path with an entry beneath it.
    ///
    /// A test states its tree as a list of paths, the way a checkout lists one, and the kinds
    /// follow from the shape. A test that needs a childless directory inserts it directly.
    pub(crate) fn implied_directories(present: &HashSet<PathBuf>) -> HashSet<PathBuf> {
        present
            .iter()
            .filter(|p| present.iter().any(|q| *q != **p && q.starts_with(p)))
            .cloned()
            .collect()
    }
}
