//! `check` run against whole projects, not against strings.
//!
//! A mock project under `path@check@tests/projects/` is a complete project: it carries its
//! own `knowledge-architect.toml` and its own documents, and it declares no table `check` does not own.
//! That is what makes these tests worth more than the in-memory ones — a fixture written as a
//! string cannot exercise the walk, the exclusions, or a layout different from this repository's.
//!
//! The directory stays excluded by this repository's own manifest: each mock project is a
//! complete foreign project, and its planted defects must be reported by the test that runs the
//! tool over it, never as this repository's own.

use std::path::PathBuf;

use crate::scan::{Observation, RetiredForm};
use crate::{Manifest, Model};

fn mock(name: &str) -> Manifest {
    // A crate reaching another's fixtures, as `cli`'s tests do, until the shared test Component holds them both.
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../check/tests/projects")
        .join(name)
        .canonicalize()
        .expect("the mock project exists");
    Manifest::load(&root).expect("the mock project's manifest")
}

fn model(name: &str) -> Model {
    Model::build(&mock(name), &[]).expect("a model of the mock project")
}

/// What the binary asks git for, taken over a mock project in place.
///
/// These tests build their models under this repository's own worktree, so git answers about
/// the mock's files out of this repository's own listing. That listing is
/// `--cached --others --exclude-standard`, so a new fixture file is in it whether or not it is
/// staged; an ignore rule is what takes one out, and a deletion has to be staged or the path
/// stays in the listing with no bytes behind it.
fn git_answers(
    manifest: &crate::Manifest,
    model: &Model,
) -> (std::collections::HashSet<String>, Vec<PathBuf>) {
    let anchors = crate::entity::Anchors::of(manifest, model.listing());
    let queries = crate::resolver::ignore_queries(model, &anchors);
    let ignored = crate::git::ignored(manifest.root(), &queries).expect("git answers the batch");
    let tracked =
        crate::git::tracked_and_ignored(manifest.root()).expect("git answers the listing");
    (ignored, tracked)
}

fn walked(model: &Model) -> Vec<String> {
    let mut names: Vec<String> = model
        .documents()
        .iter()
        .map(|d| d.rel.display().to_string())
        .collect();
    names.sort();
    names
}

#[test]
fn the_walk_obeys_the_project_that_declares_it() {
    let names = walked(&model("minimal"));
    assert_eq!(
        names,
        vec![
            "CLAUDE.md".to_string(),
            "README.md".to_string(),
            "code/lib.rs".to_string(),
            "docs/design.md".to_string(),
            "docs/goals.md".to_string(),
            "docs/open-issues/README.md".to_string(),
            "docs/open-issues/the-mock-has-one-issue.md".to_string(),
            "docs/plans/README.md".to_string(),
            "docs/plans/milestones/README.md".to_string(),
            "docs/plans/specs/README.md".to_string(),
            "docs/rejected.md".to_string(),
            "docs/tripwires.md".to_string(),
            "notes/a.md".to_string(),
            "notes/b.md".to_string(),
            "notes/decisions.md".to_string(),
            "notes/open-issues/README.md".to_string(),
            "notes/open-issues/the-notes-are-not-a-component.md".to_string(),
            "notes/readings/README.md".to_string(),
            "notes/readings/one-concern/a-reading-the-mock-records.md".to_string(),
        ],
        "the walk should hold every markdown and Rust file, minus every exclusion and \
         every generated index"
    );
}

/// The indexes of the plans homes and of each milestone leave the walk too: the walk asks for
/// the generated paths with git's listing, which places the milestones. Mutation checked:
/// `Model::build` asking with an empty listing, which walks the milestone's index.
#[test]
fn the_plans_indexes_are_outside_the_walk() {
    let names = walked(&model("dirhome"));
    let plans: Vec<&String> = names
        .iter()
        .filter(|n| n.starts_with("docs/plans/"))
        .collect();
    assert_eq!(
        plans,
        vec![
            "docs/plans/README.md",
            "docs/plans/milestones/README.md",
            "docs/plans/milestones/a-milestone/README.md",
            "docs/plans/milestones/a-milestone/a-step.md",
            "docs/plans/specs/README.md",
            "docs/plans/specs/a-spec.md",
        ]
    );
}

/// A generated index leaves the walk because the tool derives the set from the register
/// instances, and no manifest row names one.
///
/// Both halves matter. Inside the walk, the rows of a listing would be read as this project's
/// own claims; inside the inverse assertion of `uncovered`, they would be reported as a file no
/// checker reads. The mock declares one `skip-files` row and it names neither index.
#[test]
fn a_generated_index_is_outside_the_walk_and_outside_the_inverse_assertion() {
    let manifest = mock("minimal");
    let model = model("minimal");
    let generated = crate::index::generated_index_paths(&manifest, model.listing());
    // The issue home of the root and of the location, the readings home, and the two plans
    // homes, which every project carries.
    assert_eq!(generated.len(), 5, "{generated:?}");
    for rel in &generated {
        assert!(
            !manifest.walk().skip_files.contains(rel),
            "{} is declared, not derived",
            rel.display()
        );
    }
    let walked = walked(&model);
    let survey = crate::survey::survey(&manifest, &model).expect("a survey of the mock");
    for rel in &generated {
        let name = rel.display().to_string();
        assert!(
            std::path::Path::new(&manifest.root().join(rel)).exists(),
            "{name} should be a file the mock carries"
        );
        assert!(!walked.contains(&name), "{name} should not be walked");
        assert!(
            !survey.outside.iter().any(|(p, _)| p == rel),
            "{name} should not be reported as a file no checker reads"
        );
        assert!(survey.present.contains(rel), "{name} still exists");
    }
}

#[test]
fn each_of_the_three_exclusion_kinds_removes_its_file() {
    let names = walked(&model("minimal"));
    // A directory name, a path, and a filename — one document each, and each planted with a
    // slug definition that would collide if it were walked.
    assert!(!names.iter().any(|n| n.starts_with("build/")), "skip-dirs");
    assert!(!names.iter().any(|n| n.starts_with("vendor/")), "exclude");
    assert!(
        !names.iter().any(|n| n.ends_with("generated.md")),
        "skip-files"
    );
}

#[test]
fn a_suffix_the_tool_cannot_parse_is_not_walked() {
    // The walk is markdown and Rust, compiled in rather than declared, so the mock's own TOML
    // manifest and its rules text are both outside it whatever the manifest says.
    let names = walked(&model("minimal"));
    assert!(!names.iter().any(|n| n.ends_with(".txt")));
    assert!(!names.iter().any(|n| n.ends_with(".tsv")));
    assert!(!names.iter().any(|n| n.ends_with(".toml")));
}

#[test]
fn the_survey_records_which_paths_are_directories() {
    // The kind is a fact of the listing rather than an inference from entries beneath a
    // path, which could not tell an empty directory from a file.
    let manifest = mock("minimal");
    let model = Model::build(&manifest, &[]).expect("a model");
    let survey = crate::survey::survey(&manifest, &model).expect("a survey");
    assert!(survey.directories.contains(&PathBuf::from("docs")));
    assert!(survey.present.contains(&PathBuf::from("docs")));
    assert!(!survey.directories.contains(&PathBuf::from("README.md")));
    assert!(survey.present.contains(&PathBuf::from("README.md")));
}

#[test]
fn a_location_outside_every_component_is_read_by_the_listings() {
    // What a location is for: a directory carrying a subset of the registers and nothing else
    // a component carries. Undeclared, this entry is in no listing and nobody finds it.
    use crate::entity::{Anchors, Entities, Kind};
    use crate::manifest::ISSUE_REGISTER;

    let manifest = mock("minimal");
    let model = model("minimal");
    let anchors = Anchors::of(&manifest, model.listing());
    let entities = Entities::build(&model, &anchors);
    let rows = crate::records::records(&model, &anchors, &entities, &Kind::new(ISSUE_REGISTER));
    let here: Vec<_> = rows.iter().filter(|r| r.anchor == "notes").collect();
    assert_eq!(here.len(), 1, "{rows:#?}");
    // The kind comes out of the entry's own frontmatter, which is what the register declares,
    // and the anchor is the location rather than the component above it.
    assert_eq!(here[0].metadata.as_deref(), Some("observation"));
    assert_eq!(here[0].title, "The notes are not a component");
    assert_eq!(here[0].id, "the-notes-are-not-a-component");
    assert_eq!(
        here[0].site.file,
        PathBuf::from("notes/open-issues/the-notes-are-not-a-component.md")
    );
    // The component's own instance is a second anchor, so the two are not one listing.
    assert!(rows.iter().any(|r| r.anchor == "minimal"), "{rows:#?}");
}

#[test]
fn a_fenced_illustration_is_neither_a_definition_nor_a_reference_in_a_real_file() {
    // The fenced block in that document holds a heading-shaped definition and a placeholder
    // reference, which is what a document explaining the convention holds. Neither is an
    // observation; the real definitions and pointers elsewhere in the project are, and
    // nothing in this project is written in a retired form.
    let model = model("minimal");
    let observed = |f: fn(&Observation) -> Option<String>| -> Vec<String> {
        let mut out: Vec<String> = model
            .documents()
            .iter()
            .flat_map(|d| d.observations.iter())
            .filter_map(|l| f(&l.what))
            .collect();
        out.sort();
        out
    };
    let defs = observed(|o| match o {
        Observation::SlugDef { id, .. } => Some(id.clone()),
        _ => None,
    });
    assert_eq!(
        defs,
        vec![
            "a-note-the-location-keeps".to_string(),
            "mock-anchor".to_string(),
            "mock-goal".to_string(),
            "mock-tripwire".to_string()
        ]
    );
    let spans = observed(|o| match o {
        Observation::Span(s) => Some(s.clone()),
        _ => None,
    });
    assert_eq!(
        spans,
        vec![
            "design@minimal@mock-anchor".to_string(),
            "design@minimal@mock-anchor".to_string(),
            "design@minimal@mock-anchor".to_string(),
            "goal@minimal@mock-goal".to_string(),
            "note@notes@a-note-the-location-keeps".to_string(),
            "path@notes@b.md".to_string(),
            "register@notes@note".to_string(),
            "tripwire@minimal@mock-tripwire".to_string(),
        ]
    );
    let retired = observed(|o| match o {
        Observation::Retired(RetiredForm::SlugRef(s)) => Some(s.clone()),
        _ => None,
    });
    assert_eq!(retired, Vec::<String>::new());
}

/// Every committed file-register index is what the generator writes, in every mock of `check`.
#[test]
fn every_committed_index_is_what_the_generator_writes() {
    for name in ["minimal", "dirhome", "core"] {
        let manifest = mock(name);
        let model = model(name);
        let survey = crate::survey::survey(&manifest, &model).expect("a survey of the mock");
        for (rel, want) in crate::index::file_register_indexes(
            &model,
            &manifest,
            &survey.present,
            &survey.directories,
        ) {
            let have = std::fs::read_to_string(manifest.root().join(&rel)).expect("committed");
            assert_eq!(have, want, "{name}: {}", rel.display());
        }
    }
}

/// `check`'s rows carry the line of the file they sit on.
///
/// The whole value is asserted, and the trailing newline is load-bearing: a `contains` over a
/// field that is not terminated asserts a prefix, and passes against a value with more after it.
#[test]
fn observations_come_out_of_a_real_walk_with_real_line_numbers() {
    let dump = model("minimal").canonical();
    assert!(
        dump.contains("docs/design.md\t5\tslug-def\tmock-anchor heading-3\n"),
        "{dump}"
    );
    assert!(
        dump.contains("docs/design.md\t7\tspan\tpath@notes@b.md\n"),
        "{dump}"
    );
    assert!(
        dump.contains("notes/a.md\t5\tspan\tgoal@minimal@mock-goal\n"),
        "{dump}"
    );
    assert!(
        dump.contains("code/lib.rs\t1\tspan\tdesign@minimal@mock-anchor\n"),
        "{dump}"
    );
}

mod unsound {
    use super::*;
    use crate::entity::{Anchors, Entities};
    use crate::phases::{foundation, Phase, TreeInputs};
    use crate::tree;

    /// The findings of `tree::check`, phase 2, over the project.
    fn phase_two() -> Vec<String> {
        let manifest = mock("unsound");
        let model = Model::build(&manifest, &[]).expect("a model");
        let survey = crate::survey::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = TreeInputs {
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
        };
        assert!(
            manifest.complaints().is_empty(),
            "{:#?}",
            manifest.complaints()
        );
        let stopped = foundation(&model, &manifest, &inputs).expect_err("the run stops");
        assert_eq!(stopped.phase, Phase::Tree);
        let found: Vec<String> = tree::check(&model, &manifest, &inputs)
            .iter()
            .map(|f| format!("{}  {}  → {}", f.location(), f.what, f.action))
            .collect();
        let printed: Vec<String> = stopped
            .findings
            .iter()
            .map(|f| format!("{}  {}  → {}", f.location(), f.what, f.action))
            .collect();
        assert_eq!(
            printed, found,
            "the stop carries exactly phase 2's findings"
        );
        found
    }

    /// The findings of the entity table, phase 3, over the project.
    fn phase_three() -> Vec<String> {
        let manifest = mock("unsound");
        let model = Model::build(&manifest, &[]).expect("a model");
        let anchors = Anchors::of(&manifest, model.listing());
        Entities::build(&model, &anchors)
            .definition_findings()
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    fn one_of(all: &[String], needle: &str) -> String {
        let hits: Vec<&String> = all.iter().filter(|f| f.contains(needle)).collect();
        assert_eq!(hits.len(), 1, "expected exactly one {needle:?} in {all:#?}");
        hits[0].clone()
    }

    #[test]
    fn every_phase_two_assertion_has_a_planted_defect_and_nothing_else_is_reported() {
        let found = phase_two();
        assert!(one_of(&found, "could not be read as text").starts_with("notes/latin1.md:1"));
        assert!(one_of(
            &found,
            "is declared in [walk] skip-files and does not exist"
        )
        .contains("notes/gone.md"));
        assert!(one_of(&found, "retired file shape").starts_with("docs/open-issues.md"));
        let home = one_of(&found, "carries no tripwire home");
        assert!(
            home.starts_with("parts/the-widget/docs/tripwires.md"),
            "{home}"
        );
        assert!(home.contains("`the-widget`"), "{home}");
        // A location carries the registers it declares and nothing else, so a missing home
        // is a finding against the location.
        assert!(one_of(
            &found,
            "the anchor `agent-config` carries no issue directory"
        )
        .starts_with("agent-config/open-issues"));
        // The plans layout, each phase 2 because its anchors and entries are read off the
        // tree: one file outside both homes, per `design@model@plans-split-dirs`; a directory
        // with no README, a stray file and two options files in milestones/; and four names
        // `design@model@a-placed-name-reads-as-nothing-else` refuses.
        assert!(one_of(&found, "sits in the plans directory").starts_with("docs/plans/stray.md"));
        assert!(one_of(&found, "holds no README.md, so it is no milestone")
            .starts_with("docs/plans/milestones/no-readme"));
        assert!(one_of(
            &found,
            "`docs/plans/milestones/stray.md` sits in milestones/"
        )
        .starts_with("docs/plans/milestones/stray.md"));
        assert!(one_of(
            &found,
            "`docs/plans/milestones/register.toml` sits in milestones/"
        )
        .starts_with("docs/plans/milestones/register.toml"));
        assert!(
            one_of(&found, "a milestone directory holds no register.toml")
                .starts_with("docs/plans/milestones/held-twice/register.toml")
                && one_of(&found, "a milestone directory holds no register.toml")
                    .contains("a milestone's slice specs sit beside its README, ungrouped")
        );
        assert!(one_of(&found, "the milestone `the-widget` is no anchor")
            .contains("is the name of a component"));
        assert!(one_of(&found, "the milestone `Not_An_Id` is no anchor")
            .contains("its slices `spec@<id>@<slice>`"));
        assert!(one_of(&found, "the milestone `Not_An_Id` is no anchor")
            .contains("is not two or more lower-case words"));
        assert!(one_of(&found, "the spec `agent-config`").contains("is the name of a location"));
        assert!(one_of(&found, "the spec `held-twice`").contains("is also the name of a milestone"));
        // A file beside a directory that is no milestone is no retired single file of any home,
        // so the layout check names it.
        assert!(one_of(
            &found,
            "`docs/plans/milestones/the-widget.md` sits in milestones/"
        )
        .starts_with("docs/plans/milestones/the-widget.md"));
        // A setext heading, which markdown reads and the scanner does not, per
        // `design@model@headings-open-with-hash-marks`.
        assert!(one_of(&found, "markdown reads a heading here").starts_with("notes/structure.md:7"));
        assert_eq!(found.len(), 16, "{found:#?}");
    }

    #[test]
    fn a_slug_defined_twice_in_one_instance_is_reported_at_both_sites() {
        // A rename that left one behind. The reader who opens either copy is told about the
        // other.
        let all = phase_three();
        let dup: Vec<&String> = all
            .iter()
            .filter(|f| f.contains("`design@unsound@twice-defined` is also defined at"))
            .collect();
        assert_eq!(dup.len(), 2, "{all:#?}");
        assert!(
            dup.iter().any(|f| f.starts_with("docs/design/planted.md:3")
                && f.contains("docs/design/planted.md:7")),
            "{dup:#?}"
        );
        assert!(
            dup.iter().any(|f| f.starts_with("docs/design/planted.md:7")
                && f.contains("docs/design/planted.md:3")),
            "{dup:#?}"
        );
    }

    #[test]
    fn a_misplaced_definition_is_reported_where_it_stands_and_defines_nothing() {
        // Four shapes over a real walk: a level-four heading, a line head and a table cell
        // inside the design home, and a level-three heading in a file that is no register
        // home.
        let all = phase_three();
        let deep = one_of(&all, "`##too-deep` is written at a level-4 heading");
        assert!(deep.starts_with("docs/design/planted.md:9"), "{deep}");
        let head = one_of(&all, "`##line-head` is written at the head of a plain line");
        assert!(head.starts_with("docs/design/planted.md:11"), "{head}");
        let cell = one_of(&all, "`##in-a-cell` is written in a table cell");
        assert!(cell.starts_with("docs/design/planted.md:17"), "{cell}");
        let stray = one_of(&all, "`##stray-anchor` is written in `notes/structure.md`");
        assert!(stray.starts_with("notes/structure.md:3"), "{stray}");
        assert!(
            stray.contains("no heading register home of `unsound`"),
            "{stray}"
        );
        let inline = one_of(&all, "`##twice-defined` is written in the middle of a line");
        assert!(inline.starts_with("notes/structure.md:5"), "{inline}");
        assert!(one_of(&all, "cannot be an entry id").starts_with("docs/open-issues/Not_An_Id.md"));
        // A plan's items: a slug outside its item sections, and one id defined in a milestone's
        // README and in its slice spec, which share one namespace.
        assert!(
            one_of(&all, "`##not-an-item` is written outside the item sections")
                .starts_with("docs/plans/milestones/a-defective-plan/README.md")
        );
        let twice: Vec<&String> = all
            .iter()
            .filter(|f| f.contains("`thread@a-defective-plan@defined-twice` is also defined at"))
            .collect();
        assert_eq!(twice.len(), 2, "{all:#?}");
        // An id of one segment, reported where it is defined, per
        // `design@model@two-segment-ids`.
        let lonely = one_of(&all, "`##lonely` is an id of one segment");
        assert!(lonely.starts_with("docs/design/planted.md:"), "{lonely}");
        assert_eq!(all.len(), 13, "{all:#?}");
    }

    #[test]
    fn a_heading_at_the_register_level_with_no_slug_is_reported_and_defines_nothing() {
        // A tripwire or a decision written without its slug reads as an entry, and nothing
        // lists it. The design register's level is three; the level-one title and the
        // level-four heading beside it owe nothing.
        let all = phase_three();
        let lost = one_of(&all, "carries no slug");
        assert!(lost.starts_with("docs/design/planted.md:13"), "{lost}");
        assert!(lost.contains("level-3 heading"), "{lost}");
        assert!(lost.contains("design home"), "{lost}");
    }

    /// The negative half of the gate: the projects that reach phase 4 have nothing in the
    /// first three, so a stop is never mistaken for a pass and a pass never hides a stop.
    #[test]
    fn planted_and_dirhome_are_complete_through_phase_three() {
        for name in ["planted", "dirhome"] {
            let manifest = mock(name);
            let model = Model::build(&manifest, &[]).expect("a model");
            let survey = crate::survey::survey(&manifest, &model).expect("a survey of the mock");
            let git = git_answers(&manifest, &model);
            let inputs = TreeInputs {
                present: &survey.present,
                directories: &survey.directories,
                outside: &survey.outside,
                ignored: &git.0,
                tracked_and_ignored: &git.1,
                refused: &survey.refused,
                links: &survey.links,
            };
            let outcome = foundation(&model, &manifest, &inputs);
            assert!(outcome.is_ok(), "{name}: {:#?}", outcome.err());
        }
    }
}

mod checker {
    use super::*;

    /// The root is canonicalised before the prefix test, so a checkout reached through a
    /// symlink still finds the checker's directory inside it. Without the canonicalisation
    /// the joined path starts with the link and the real directory starts with its target,
    /// and nothing is exempt.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_root_still_finds_the_checker_directory_inside_it() {
        let real = mock("planted").root().to_path_buf();
        let link =
            std::env::temp_dir().join(format!("knowledge-symlinked-root-{}", std::process::id()));
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&real, &link).expect("a symlink to the mock project");
        let manifest = Manifest::load(&link).expect("the manifest through the symlink");
        let model = Model::build(&manifest, &[real.join("code").as_path()]).expect("a model");
        let _ = std::fs::remove_file(&link);
        assert!(model.checker_files() >= 1, "{:?}", model.checker_sources());
    }
}
