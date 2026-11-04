//! Phases 1 to 3: whether the model is complete, so that what is computed over it can be trusted.
//!
//! Each of the three builds one input of the next, and the first that produces anything ends the
//! run, per `design@core@phases-gate-the-report`. They say whether the model can be trusted, not
//! whether the content conforms. A tool that needs the model and no check asks
//! [`complete_working_tree`].

use std::collections::HashSet;
use std::path::PathBuf;

use crate::finding::Finding;
use crate::manifest::Manifest;
use crate::model::Model;
use crate::survey::Survey;

/// What phases 1 to 3 and the resolver read, read from the tree being judged.
pub struct TreeInputs<'a> {
    /// Every path that exists in the tree, files and directories, project-relative.
    pub present: &'a HashSet<PathBuf>,
    /// The subset of `present` that is directories.
    pub directories: &'a HashSet<PathBuf>,
    /// Files the walk does not cover, with what reading each gave.
    pub outside: &'a [(PathBuf, crate::survey::Outside)],
    /// The spellings the ignore rules cover, keyed by `git::ignore_query`, over
    /// [`ignore_queries`](crate::resolver::ignore_queries). A target left out reads as not ignored,
    /// so the resolver reports it as missing.
    pub ignored: &'a HashSet<String>,
    /// The files git both tracks and ignores.
    pub tracked_and_ignored: &'a [PathBuf],
    /// The files the walk refuses by name.
    pub refused: &'a [PathBuf],
    /// The listing's symlink and gitlink entries no walk row keeps.
    pub links: &'a [crate::git::Entry],
    /// The files of the installer's namespace, with what reading each gave.
    pub installed: &'a [(PathBuf, crate::survey::Outside)],
    /// Every shipped file at its install path, rendered. Empty when the project serves no harness.
    pub shipped: &'a [(PathBuf, String)],
}

/// Where in the pipeline a finding is produced, and therefore what it undermines.
///
/// Each of the first three phases builds one input of the next — the manifest resolves into
/// a configuration, the walk and the parse into a model, the model into the entity table —
/// and every check of the last phase consumes the table and produces nothing another check
/// reads. So a finding raised in one phase makes every finding of a later phase unreliable
/// in both directions, missing and false, and a finding raised in the last undermines
/// nothing. A finding is classified by the place it is produced, never by a label at its
/// site: one added to a building step is gated because of where it is raised.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    /// The manifest's declarations, resolved: every complaint `Manifest::parse` holds.
    Resolution = 1,
    /// The walk and the parse against the tree: `tree`.
    Tree = 2,
    /// The entity table's own findings: a definition where none may sit, an id nothing can
    /// spell, one id defined twice.
    Definitions = 3,
    /// Everything computed over a complete model.
    Content = 4,
}

impl Phase {
    pub fn number(self) -> u8 {
        self as u8
    }

    /// The summary line a stop prints: what was reported, and what was not judged and why.
    pub fn stop_line(self, findings: usize) -> String {
        let later: Vec<String> = [Phase::Tree, Phase::Definitions, Phase::Content]
            .into_iter()
            .filter(|p| *p > self)
            .map(|p| p.number().to_string())
            .collect();
        let later = match later.as_slice() {
            [one] => format!("phase {one} was"),
            [head @ .., last] => format!("phases {} and {last} were", head.join(", ")),
            [] => String::new(),
        };
        format!(
            "phase {}: {findings} finding(s); {later} not judged, because their findings would \
             be computed over a model these leave incomplete",
            self.number()
        )
    }
}

/// A run that ended before the last phase: the phase that produced something, and what.
#[derive(Debug)]
pub struct Stop {
    pub phase: Phase,
    pub findings: Vec<Finding>,
}

/// Phases 1 to 3, in order. The first that produces anything ends the run.
///
/// A check is a pure function over the model, so the releases the last phase needs are not
/// needed here and a caller resolves them only once this returns `Ok`: a run that stops
/// earlier fetches nothing.
pub fn foundation(model: &Model, manifest: &Manifest, inputs: &TreeInputs) -> Result<(), Stop> {
    let complaints = manifest.complaints();
    if !complaints.is_empty() {
        return Err(Stop {
            phase: Phase::Resolution,
            findings: complaints.to_vec(),
        });
    }
    let mut findings = crate::tree::check(model, manifest, inputs);
    findings.extend(crate::shipped::check(model, manifest, inputs));
    if !findings.is_empty() {
        return Err(Stop {
            phase: Phase::Tree,
            findings,
        });
    }
    let anchors = crate::entity::Anchors::of(manifest, inputs.present);
    let findings = crate::entity::Entities::build(model, &anchors)
        .definition_findings()
        .to_vec();
    if !findings.is_empty() {
        return Err(Stop {
            phase: Phase::Definitions,
            findings,
        });
    }
    Ok(())
}

/// What a run reads from the working tree for phases 1 to 3 and the resolver, gathered once for
/// every caller, per `design@model@tree-inputs-gathered-here`.
///
/// A phase and the resolver are pure over the model, so the files and the git answers they read
/// are fetched here by the caller, per `design@core@model-then-checks`.
pub struct Gathered {
    survey: Survey,
    ignored: HashSet<String>,
    tracked_and_ignored: Vec<PathBuf>,
    shipped: Vec<(PathBuf, String)>,
}

impl Gathered {
    /// Read the working tree under the manifest's root: one survey, the two git batches, and the
    /// agent files this version ships.
    ///
    /// `Err` is could-not-run: git or the filesystem refused a read the run cannot do without.
    pub fn over(manifest: &Manifest, model: &Model) -> Result<Gathered, String> {
        // One listing answers every question a phase has about what is there.
        let survey = crate::survey::survey(manifest, model).map_err(|e| e.to_string())?;
        // Whether the ignore rules cover a path target is git's answer, taken in ONE batch over
        // every spelling a reference in this run could ask about: a process per reference
        // would be a process per pointer in the tree.
        let anchors = crate::entity::Anchors::of(manifest, model.listing());
        let queries = crate::resolver::ignore_queries(model, &anchors);
        let ignored = crate::git::ignored(manifest.root(), &queries).map_err(|e| e.to_string())?;
        let tracked_and_ignored =
            crate::git::tracked_and_ignored(manifest.root()).map_err(|e| e.to_string())?;
        let shipped = crate::shipped::shipped(manifest);
        Ok(Gathered {
            survey,
            ignored,
            tracked_and_ignored,
            shipped,
        })
    }

    /// What phases 1 to 3 and the resolver read, borrowed from what was gathered.
    pub fn inputs(&self) -> TreeInputs<'_> {
        TreeInputs {
            present: &self.survey.present,
            directories: &self.survey.directories,
            outside: &self.survey.outside,
            ignored: &self.ignored,
            tracked_and_ignored: &self.tracked_and_ignored,
            refused: &self.survey.refused,
            links: &self.survey.links,
            installed: &self.survey.installed,
            shipped: &self.shipped,
        }
    }
}

/// Gather the working tree and refuse a model the foundation stops, or return what was gathered.
///
/// The gate of a tool that writes files over the model, or that answers from it: such a tool runs
/// no check, and its answer over an incomplete model could not be trusted.
///
/// `Err` is could-not-run: git or the filesystem refused a read, or the foundation stopped, which the
/// text says by the phase and the command that reports what it found. Over a project that serves the
/// `claude` harness, it compares the installed agent files with what this version of the crate ships.
pub fn complete_working_tree(manifest: &Manifest, model: &Model) -> Result<Gathered, String> {
    let gathered = Gathered::over(manifest, model)?;
    match foundation(model, manifest, &gathered.inputs()) {
        Ok(()) => Ok(gathered),
        Err(stop) => Err(format!(
            "the model is incomplete: {}. `{} check` reports them. Nothing was \
             written.",
            stop.phase.stop_line(stop.findings.len()),
            manifest.command()
        )),
    }
}

#[cfg(test)]
mod phase_tests {
    use super::*;

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n\n\
             "
        );
        Manifest::parse(std::path::Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The tree every component owes, in the file-shaped heading homes.
    /// Directories are listed too: what exists is asked of `present`, and a checkout's
    /// listing holds a directory wherever a file sits under it.
    fn complete_tree() -> HashSet<PathBuf> {
        [
            "docs",
            "docs/open-issues",
            "README.md",
            "CLAUDE.md",
            "docs/design.md",
            "docs/goals.md",
            "docs/tripwires.md",
            "docs/rejected.md",
            "docs/open-issues/README.md",
            "docs/open-issues/index.md",
        ]
        .iter()
        .chain(testing::PLANS_TREE.iter())
        .map(PathBuf::from)
        .collect()
    }

    fn foundation_over(
        manifest: &Manifest,
        present: HashSet<PathBuf>,
        model: &Model,
    ) -> Result<(), Stop> {
        let directories = testing::implied_directories(&present);
        let inputs = TreeInputs {
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
        };
        foundation(model, manifest, &inputs)
    }

    #[test]
    fn a_complaint_stops_the_run_at_phase_one_before_the_tree_is_read() {
        // Both a refused row and a missing home: the complaint is what prints, alone, and
        // phase 2's missing home is not judged.
        let m = manifest("[locations.up]\npath = \"docs/../docs\"\nregisters = [\"issue\"]\n");
        let mut present = complete_tree();
        present.remove(&PathBuf::from("docs/goals.md"));
        let stop = foundation_over(&m, present, &Model::from_documents(Vec::new()))
            .expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Resolution);
        assert_eq!(stop.findings.len(), 1, "{:#?}", stop.findings);
        assert!(stop.findings[0].what.contains("spells a `..` segment"));
    }

    #[test]
    fn every_complaint_prints_at_phase_one() {
        let m = manifest(
            "[locations.up]\npath = \"docs/../docs\"\nregisters = [\"issue\"]\n\n\
             [locations.abs]\npath = \"/abs\"\nregisters = [\"issue\"]\n",
        );
        let stop = foundation_over(&m, complete_tree(), &Model::from_documents(Vec::new()))
            .expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Resolution);
        assert_eq!(stop.findings.len(), 2, "{:#?}", stop.findings);
    }

    #[test]
    fn a_location_holding_the_root_s_homes_is_the_one_finding_and_owns_no_slug() {
        // The shape the cascade entry was about: a location at the root's `docs/`, refused for
        // holding the five homes, and a slug in the design home that it would have owned. The
        // refusal is the one finding; nothing reports the slug misplaced against the location.
        let m = manifest("[locations.papers]\npath = \"docs\"\nregisters = [\"issue\"]\n");
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n### A decision `##decision-one`\n".to_string(),
        )]);
        let stop = foundation_over(&m, complete_tree(), &model).expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Resolution, "{:#?}", stop.findings);
        assert_eq!(stop.findings.len(), 1, "{:#?}", stop.findings);
        assert!(
            stop.findings[0]
                .what
                .contains("holds the design, goal, tripwire, rejected and issue homes"),
            "{:#?}",
            stop.findings
        );
    }

    #[test]
    fn a_definition_defect_stops_the_run_at_phase_three_and_a_complete_tree_passes() {
        let m = manifest("");
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n#### Too deep `##too-deep`\n".to_string(),
        )]);
        let stop = foundation_over(&m, complete_tree(), &model).expect_err("the run stops");
        assert_eq!(stop.phase, Phase::Definitions, "{:#?}", stop.findings);
        assert!(
            stop.findings[0].what.contains("`##too-deep`"),
            "{:#?}",
            stop.findings
        );
        let clean = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n### A decision `##decision-one`\n".to_string(),
        )]);
        assert!(foundation_over(&m, complete_tree(), &clean).is_ok());
    }

    #[test]
    fn the_stop_line_names_the_phase_and_what_was_not_judged() {
        assert_eq!(
            Phase::Resolution.stop_line(2),
            "phase 1: 2 finding(s); phases 2, 3 and 4 were not judged, because their findings \
             would be computed over a model these leave incomplete"
        );
        assert!(Phase::Tree
            .stop_line(1)
            .starts_with("phase 2: 1 finding(s); phases 3 and 4 were"));
        assert!(Phase::Definitions
            .stop_line(1)
            .starts_with("phase 3: 1 finding(s); phase 4 was"));
    }
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
