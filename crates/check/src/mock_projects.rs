//! The core run against whole projects, not against strings.
//!
//! A mock project under `path@check@tests/projects/` is a complete project: it carries its
//! own `knowledge-architect.toml` and its own documents, and it declares no table the core does not own.
//! That is what makes these tests worth more than the in-memory ones — a fixture written as a
//! string cannot exercise the walk, the exclusions, or a layout different from this repository's.
//!
//! The directory stays excluded by this repository's own manifest: each mock project is a
//! complete foreign project, and its planted defects must be reported by the test that runs the
//! tool over it, never as this repository's own.

use std::path::PathBuf;

use crate::{Manifest, Model};

fn mock(name: &str) -> Manifest {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name);
    Manifest::load(&root).expect("the mock project's manifest")
}

fn model(name: &str) -> Model {
    Model::build(&mock(name), &[]).expect("a model of the mock project")
}

/// Every `register.toml` beside an instance, as the binary reads them for a check.
fn configs(manifest: &Manifest, model: &Model) -> std::collections::HashMap<PathBuf, String> {
    let mut out = std::collections::HashMap::new();
    for (_, _, home) in crate::model::table::Anchors::of(manifest, model.listing()).instances() {
        if let Ok(text) = std::fs::read_to_string(manifest.root().join(&home.config)) {
            out.insert(home.config.clone(), text);
        }
    }
    out
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
    let anchors = crate::model::table::Anchors::of(manifest, model.listing());
    let queries = crate::model::resolve::ignore_queries(model, &anchors);
    let ignored = crate::git::ignored(manifest.root(), &queries).expect("git answers the batch");
    let tracked =
        crate::git::tracked_and_ignored(manifest.root()).expect("git answers the listing");
    (ignored, tracked)
}

/// A project that carries what it declares reports nothing, over every core check.
#[test]
fn the_conformant_mocks_report_nothing_over_every_core_check() {
    use crate::check::{foundation, run, Inputs};
    use std::collections::HashMap;
    for name in ["minimal", "dirhome", "core"] {
        let manifest = mock(name);
        let model = model(name);
        let survey =
            crate::model::listing::survey(&manifest, &model).expect("a survey of the mock");
        // The committed generated files, read from the tree rather than regenerated, so a
        // committed index that has gone stale fails here.
        let mut committed = HashMap::new();
        for rel in crate::model::generated::generated_paths(&manifest, model.listing()) {
            if let Ok(text) = std::fs::read_to_string(manifest.root().join(&rel)) {
                committed.insert(rel, text);
            }
        }
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            committed: &committed,
            configs: &configs(&manifest, &model),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
            installed: &survey.installed,
            shipped: &[],
        };
        assert!(
            foundation(&model, &manifest, &inputs).is_ok(),
            "{name}: phases 1 to 3"
        );
        let found: Vec<String> = run(&model, &manifest, &inputs, &[])
            .findings
            .iter()
            .map(|f| f.to_string())
            .collect();
        assert!(found.is_empty(), "{name}: {found:#?}");
    }
}

mod planted {
    use super::*;
    use crate::check::{run, Inputs, CHECKS};
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// The file-register indexes this project would have if they were current.
    fn current_indexes(manifest: &Manifest, model: &Model) -> HashMap<PathBuf, String> {
        let survey = crate::model::listing::survey(manifest, model).expect("a survey of the mock");
        crate::model::generated::file_register_indexes(
            model,
            manifest,
            &survey.present,
            &survey.directories,
        )
        .into_iter()
        .collect()
    }

    /// The findings of the core's last phase, with the committed files as stated.
    fn findings_with(
        committed: impl Fn(&Manifest, &Model) -> HashMap<PathBuf, String>,
    ) -> Vec<String> {
        let manifest = mock("planted");
        let model = Model::build(&manifest, &[]).expect("a model");
        let committed = committed(&manifest, &model);
        let survey =
            crate::model::listing::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            committed: &committed,
            configs: &configs(&manifest, &model),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
            installed: &survey.installed,
            shipped: &[],
        };
        run(&model, &manifest, &inputs, &[])
            .findings
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    fn findings() -> Vec<String> {
        findings_with(current_indexes)
    }

    /// Every planted defect this project carries, by the core check that reports it: the check,
    /// how many findings it produces, and a fragment of one. `every_check_has_a_row` reads
    /// `CHECKS` and demands a row for each, so a check with no planted defect fails there.
    ///
    /// The definition-site findings are not here: whether a home EXISTS is phase 2, and a slug
    /// or an entry id where none may sit is phase 3, and both are `unsound`'s.
    const PLANTED: [(&str, usize, &str); 3] = [
        // One reference of each shape the resolver tells apart — dangling, unknown anchor,
        // two and four segments, an anchor and a reserved anchor in kind position, the two
        // retired slug shapes — and the path shapes: a dangling one, the unanchored bare form,
        // a wrong kind claim, an escape that resolves here, a root pointer reaching inside the
        // component, a generic pointer nothing carries, a dangling tripwire reference, a link
        // outside a navigation home, and the retired `@` escape with its empty head. Then the
        // five refusals of the plans layout, in notes/plans.md: a spec, a milestone directory,
        // a slice spec cited by path, a milestone named as a path's anchor, and a pointer from
        // the root reaching inside `plans`.
        // And an item of a plan cited from outside it, which the scoped resolution refuses, and
        // a `rejected` entry no design entry cites, which the orphan check reports. Then, in
        // notes/ids.md, a defined id written outside its reference form, once per lint. And, in
        // docs/goals.md, a design head cited from a goals home, which resolves and is refused
        // there, per `design@check@goals-home-cites-upward-only`.
        ("references", 28, "is referenced"),
        // One defect per shape assertion over what is there: a missing index, an undeclared
        // kind, a missing owed subsection, an undeclared group, a file of another suffix, and
        // frontmatter that does not parse.
        ("registers", 6, "issue register"),
        // Six register indexes: three issue homes, the two plans homes and the one milestone.
        // The run is handed an empty committed set, so each is reported missing.
        ("generated", 6, "the generated file is missing"),
    ];

    #[test]
    fn every_check_has_a_row_and_the_run_is_their_sum() {
        // Read off `CHECKS` rather than off `PLANTED`, so a check added to the library with no
        // planted defect fails here instead of being absent from both. And nothing is invented
        // by running them together: the whole run is the sum of the rows.
        for name in CHECKS {
            assert!(
                PLANTED.iter().any(|(n, _, _)| *n == name),
                "{name} has no row: every check owes a planted defect"
            );
        }
        let whole = findings_with(|_, _| HashMap::new());
        let planted: usize = PLANTED.iter().map(|(_, n, _)| n).sum();
        assert_eq!(whole.len(), planted, "{whole:#?}");
        for (name, n, fragment) in PLANTED {
            let hits = whole.iter().filter(|f| f.contains(fragment)).count();
            assert!(
                (1..=n).contains(&hits),
                "{name}: {hits} of {fragment:?}, expected 1 to {n}, in {whole:#?}"
            );
        }
    }

    /// The committed indexes of this project that are meant to be current: each is what the
    /// generator writes. The project's generated files are written by hand, so this list is
    /// the one place that tells a planted stale file from a drifted one. The other two
    /// register indexes are `ABSENT`, the registers check's planted missing index, and
    /// `STALE`, which lists no entry, beside the entry whose title says it was never
    /// regenerated.
    const CURRENT: [&str; 4] = [
        "agent-config/open-issues/index.md",
        "docs/plans/milestones/index.md",
        "docs/plans/milestones/m-one/index.md",
        "docs/plans/specs/index.md",
    ];

    /// The register index whose absence is planted, and the one whose staleness is.
    const ABSENT: &str = "docs/open-issues/index.md";
    const STALE: &str = "parts/widget/docs/open-issues/index.md";

    /// Every register index of the project is current, absent or stale as the lists above
    /// say, so a planted defect that drifts to current, or an index dropped from `CURRENT`,
    /// fails here.
    #[test]
    fn every_committed_index_meant_to_be_current_is_what_the_generator_writes() {
        let manifest = mock("planted");
        let model = Model::build(&manifest, &[]).expect("a model");
        let want = current_indexes(&manifest, &model);
        let mut listed: Vec<PathBuf> = CURRENT.iter().map(PathBuf::from).collect();
        listed.push(PathBuf::from(ABSENT));
        listed.push(PathBuf::from(STALE));
        listed.sort();
        let mut generated: Vec<PathBuf> = want.keys().cloned().collect();
        generated.sort();
        assert_eq!(listed, generated);
        for rel in CURRENT {
            let have =
                std::fs::read_to_string(manifest.root().join(rel)).expect("a committed index");
            assert_eq!(Some(&have), want.get(&PathBuf::from(rel)), "{rel}");
        }
        assert!(!manifest.root().join(ABSENT).exists(), "{ABSENT}");
        let stale = std::fs::read_to_string(manifest.root().join(STALE)).expect("committed");
        assert_ne!(Some(&stale), want.get(&PathBuf::from(STALE)), "{STALE}");
    }

    /// The comparison is exact to the byte: an index that differs from its regeneration by
    /// one trailing line break alone is out of date.
    #[test]
    fn an_index_that_differs_by_a_trailing_line_break_is_out_of_date() {
        let path = PathBuf::from(CURRENT[0]);
        let edited = findings_with(|m, model| {
            let mut c = current_indexes(m, model);
            let text = format!("{}\n", c[&path]);
            c.insert(path.clone(), text);
            c
        });
        let hits: Vec<&String> = edited
            .iter()
            .filter(|f| f.contains("out of date"))
            .collect();
        assert_eq!(hits.len(), 1, "{edited:#?}");
        assert!(hits[0].starts_with(CURRENT[0]), "{hits:#?}");
    }

    #[test]
    fn a_generated_file_that_is_absent_is_reported_as_missing() {
        let gone = findings_with(|_, _| HashMap::new());
        assert_eq!(
            gone.iter().filter(|f| f.contains("is missing")).count(),
            6,
            "the six register indexes: {gone:#?}"
        );
    }

    /// `design@model@plan-document-kinds`: a plan document is cited by its kind, so each `path`
    /// citation of one is refused with the kind form that resolves, from any anchor; and a
    /// pointer from the root at a file of `plans` that is no plan document reaches inside it.
    #[test]
    fn a_plan_document_cited_by_its_path_is_refused_with_the_form_that_resolves() {
        let manifest = mock("planted");
        let model = Model::build(&manifest, &[]).expect("a model");
        let committed = current_indexes(&manifest, &model);
        let survey =
            crate::model::listing::survey(&manifest, &model).expect("a survey of the mock");
        let git = git_answers(&manifest, &model);
        let inputs = Inputs {
            committed: &committed,
            configs: &configs(&manifest, &model),
            present: &survey.present,
            directories: &survey.directories,
            outside: &survey.outside,
            ignored: &git.0,
            tracked_and_ignored: &git.1,
            refused: &survey.refused,
            links: &survey.links,
            installed: &survey.installed,
            shipped: &[],
        };
        let here: Vec<(u32, String, String)> = run(&model, &manifest, &inputs, &[])
            .findings
            .into_iter()
            .filter(|f| f.file == std::path::Path::new("notes/plans.md"))
            .map(|f| (f.line.unwrap_or(0), f.what, f.action))
            .collect();
        let at = |line: u32| -> (String, String) {
            let hit = here.iter().find(|(l, _, _)| *l == line);
            let (_, what, action) = hit.unwrap_or_else(|| panic!("line {line}: {here:#?}"));
            (what.clone(), action.clone())
        };
        let (what, action) = at(6);
        assert!(what.contains("cites a plan document by its path"), "{what}");
        assert!(action.contains("`spec@plans@a-spec`"), "{action}");
        let (what, action) = at(7);
        assert!(what.contains("cites a plan document by its path"), "{what}");
        assert!(action.contains("`milestone@plans@m-one`"), "{action}");
        let (what, action) = at(8);
        assert!(what.contains("cites a plan document by its path"), "{what}");
        assert!(action.contains("`spec@m-one@a-step`"), "{action}");
        let (what, action) = at(9);
        assert!(
            what.contains("names the milestone `m-one`, which carries no path kind"),
            "{what}"
        );
        assert!(action.contains("`spec@m-one@<slice>`"), "{action}");
        let (what, _) = at(10);
        assert!(what.contains("reaches inside the anchor `plans`"), "{what}");
        // An item of a plan cited from outside it is refused, and the repair cites the plan whole.
        let (what, action) = at(12);
        assert!(
            what.contains("cites an item of the plan `m-one` from outside it"),
            "{what}"
        );
        assert!(action.contains("`milestone@plans@m-one`"), "{action}");
        assert_eq!(here.len(), 6, "{here:#?}");
    }

    fn one(needle: &str) -> String {
        let all = findings();
        let hits: Vec<&String> = all.iter().filter(|f| f.contains(needle)).collect();
        assert_eq!(hits.len(), 1, "expected exactly one {needle:?} in {all:#?}");
        hits[0].clone()
    }

    /// A file-register index is judged the same way, and by its own bytes.
    ///
    /// The hand edit is in the rows rather than in the banner, because the banner is the one
    /// line a generator that produced nothing at all would still get right.
    #[test]
    fn a_hand_edited_file_register_index_is_reported_at_the_line_that_was_edited() {
        let path = PathBuf::from("parts/widget/docs/open-issues/index.md");
        let edited = findings_with(|m, model| {
            let mut c = current_indexes(m, model);
            let text = c[&path].replace("| todo |", "| defect |");
            c.insert(path.clone(), text);
            c
        });
        let hits: Vec<&String> = edited
            .iter()
            .filter(|f| f.contains("out of date"))
            .collect();
        assert_eq!(hits.len(), 1, "{edited:#?}");
        // The banner, the blank line, the count, the blank line, the header, the rule, the
        // row: the edit is in the row and the finding names it.
        assert!(
            hits[0].starts_with("parts/widget/docs/open-issues/index.md:7"),
            "{}",
            hits[0]
        );
    }

    #[test]
    fn a_reference_whose_id_the_anchor_does_not_define_is_reported() {
        let f = one("`design@planted@dangling-anchor` is referenced");
        assert!(f.starts_with("notes/structure.md:5"), "{f}");
        assert!(f.contains("defines no design `dangling-anchor`"), "{f}");
    }

    #[test]
    fn a_reference_naming_an_anchor_that_is_not_declared_is_reported() {
        // A different repair from the one above, and the finding says which: the id exists,
        // and the anchor segment is what nothing resolves.
        let f = one("`design@nowhere@twice-defined` names `nowhere`");
        assert!(f.starts_with("notes/structure.md:7"), "{f}");
    }

    #[test]
    fn a_malformed_reference_is_reported_naming_the_segment_count() {
        let two = one("`design@planted` is malformed");
        assert!(two.starts_with("notes/structure.md:9"), "{two}");
        let four = one("`design@planted@twice@defined` is malformed");
        assert!(four.starts_with("notes/structure.md:11"), "{four}");
    }

    #[test]
    fn an_anchor_in_kind_position_is_reported_for_a_component_and_a_reserved_word() {
        let old = one("`planted@notes/p.md` opens with `planted`");
        assert!(old.starts_with("notes/structure.md:13"), "{old}");
        let reserved = one("`*@notes/p.md` opens with `*`");
        assert!(reserved.starts_with("notes/structure.md:15"), "{reserved}");
    }

    #[test]
    fn the_retired_slug_form_is_reported_as_what_it_was_and_a_bare_entry_number_is_not() {
        let qualified = one("`planted#twice-defined` is the retired slug reference form");
        assert!(
            qualified.starts_with("notes/structure.md:17"),
            "{qualified}"
        );
        let bare = one("`#twice-defined` is the retired slug reference form");
        assert!(bare.starts_with("notes/structure.md:19"), "{bare}");
        let all = findings();
        assert!(
            !all.iter().any(|f| f.starts_with("notes/structure.md:21")),
            "{all:#?}"
        );
    }

    /// The claim: each lint on a defined id reports its planted defect, a reference-shaped
    /// token outside backticks, an id in backticks alone and an id as a plain word, each naming
    /// the reference it is written for.
    #[test]
    fn each_lint_on_a_defined_id_reports_its_planted_defect() {
        let token = one("`goal@planted@planted-goal` is a reference written without backticks");
        assert!(token.starts_with("notes/ids.md:5"), "{token}");
        let bare = one("`planted-goal` is written in backticks without its kind");
        assert!(bare.starts_with("notes/ids.md:7"), "{bare}");
        assert!(bare.contains("names `goal@planted@planted-goal`"), "{bare}");
        let plain = one("`planted-goal` is written as a plain word");
        assert!(plain.starts_with("notes/ids.md:9"), "{plain}");
    }

    #[test]
    fn a_reference_across_a_component_boundary_resolves() {
        // Line 23 of that document points at the decision the component below the root
        // records, and the component's own design document points back. Neither is a
        // finding, and the count in `PLANTED` is what asserts that they produce none.
        let all = findings();
        assert!(
            !all.iter().any(|f| f.contains("widget-decision")),
            "a qualified reference that resolves must be silent: {all:#?}"
        );
    }

    #[test]
    fn an_empty_head_in_front_of_a_path_shape_is_malformed_rather_than_silent() {
        let f = one("`@notes/p.md` is malformed: the kind segment is empty");
        assert!(f.starts_with("notes/structure.md:43"), "{f}");
    }

    #[test]
    fn a_path_that_does_not_resolve_is_reported() {
        // Named rather than matched on "does not exist": a declared tracker that is not there
        // says the same words, and a needle matching both would pass while checking neither.
        assert!(one("`path@planted@notes/missing.md` does not exist")
            .starts_with("notes/structure.md:25"));
    }

    #[test]
    fn each_shape_of_the_path_kind_is_enforced_over_a_real_walk() {
        // One planted defect per path check, end to end; the unit tests carry the shapes,
        // and this asserts the walk delivers each to its check.
        assert!(one("names no anchor").contains("notes/missing.md"));
        assert!(one("claims a file and names a directory").contains("path@planted@notes"));
        assert!(one("resolves in this tree").contains("path@elsewhere@notes/p.md"));
        assert!(one("reaches inside the anchor `widget`").starts_with("notes/structure.md:33"));
        assert!(one("resolves in no component").contains("path@*@notes/void.md"));
        assert!(one("not a navigation home").contains("p.md"));
    }

    #[test]
    fn a_reference_to_another_register_is_resolved_in_that_register() {
        let f = one("`tripwire@planted@nothing` is referenced");
        assert!(f.starts_with("notes/structure.md:37"), "{f}");
        assert!(f.contains("defines no tripwire `nothing`"), "{f}");
    }

    #[test]
    fn every_shape_a_file_register_asserts_has_a_planted_defect() {
        // One per shape assertion, over a real walk: the missing index, a kind the closed
        // list does not hold, a missing owed subsection, a subdirectory that is no declared
        // group, and a file of another suffix inside the instance. The retired file shape
        // and an id no reference can spell are `unsound`'s.
        assert!(one("has no index.md").starts_with("docs/open-issues/index.md"));
        assert!(one("no accepted value").starts_with("docs/open-issues/a-wrong-kind.md"));
        assert!(one("no level-three subsection `What would close it`")
            .starts_with("docs/open-issues/a-missing-subsection.md"));
        assert!(one("is no declared group").contains("an-undeclared-group"));
        assert!(one("is not an entry of the issue register").contains("nonsense.txt"));
        assert!(one("does not parse: line 3 declares `kind` a second time")
            .starts_with("docs/open-issues/two-kinds.md"));
    }

    #[test]
    fn a_well_formed_entry_produces_no_finding() {
        // The negative half. Without it the entry checks could fire on everything and every
        // planted assertion above would still pass.
        let all = findings();
        assert!(
            !all.iter().any(|f| f.contains("a-well-formed-one.md")),
            "{all:#?}"
        );
    }
}
