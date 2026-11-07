//! `check` driven through its public API over whole projects, not over strings.
//!
//! A mock project under `path@testing@tests/projects/` is a complete project: it carries its
//! own `knowledge-architect.toml` and its own documents, and it declares no table `check` does not own.
//! That is what makes these tests worth more than the in-memory ones: a fixture written as a
//! string cannot exercise the walk, the exclusions, or a layout different from this repository's.
//!
//! **A test states what the tree holds, in a copy.** A run reads its committed generated files
//! from the tree, and no caller assembles what it reads, per `design@check@inputs-are-gathered`, so a
//! test about a generated file that is absent, current or edited makes that state in a copy of
//! the mock, a git repository of its own, and runs over it.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use knowledge_architect_check::commands::Gathered;
use knowledge_architect_check::{Finding, Manifest, Model};
use knowledge_architect_model as km;
use knowledge_architect_testing::{foundation, run_with, CHECKS};

fn source(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

fn git(dir: &Path, args: &[&str]) {
    // The variables that would point it elsewhere are removed: a suite run from a commit hook inherits them.
    let out = Command::new("git")
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .current_dir(dir)
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the copy's directory");
    for entry in std::fs::read_dir(from).expect("a readable mock") {
        let entry = entry.expect("a directory entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a file type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copied file");
        }
    }
}

/// A copy of a mock in the target directory, under this process's id so two runs do not meet, whose
/// generated files `state` writes before git lists them.
fn copy(tag: &str, name: &str, state: impl FnOnce(&Path)) -> Manifest {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("checks-over-mocks-{}", std::process::id()))
        .join(tag);
    copy_tree(&source(name), &dir);
    git(&dir, &["init", "-q"]);
    git(&dir, &["-c", "core.excludesFile=/dev/null", "add", "-A"]);
    state(&dir);
    git(&dir, &["-c", "core.excludesFile=/dev/null", "add", "-A"]);
    Manifest::load(&dir).expect("the copy's manifest")
}

fn findings(manifest: &Manifest) -> Vec<Finding> {
    let model = Model::build(manifest, &[]).expect("a model");
    let gathered = Gathered::over(manifest, &model).expect("what a run reads");
    run_with(&model, manifest, &gathered.inputs(), &[]).findings
}

/// A project that carries what it declares reports nothing, over every built-in check.
#[test]
fn the_conformant_mocks_report_nothing_over_every_built_in_check() {
    for name in ["minimal", "dirhome", "core"] {
        // In place, so a committed index that has gone stale fails here.
        let manifest = Manifest::load(&source(name)).expect("the mock's manifest");
        let model = Model::build(&manifest, &[]).expect("a model");
        let gathered = Gathered::over(&manifest, &model).expect("what a run reads");
        let inputs = gathered.inputs();
        assert!(
            foundation(&model, &manifest, &inputs).is_ok(),
            "{name}: phases 1 to 3"
        );
        let found: Vec<String> = run_with(&model, &manifest, &inputs, &[])
            .findings
            .iter()
            .map(|f| f.to_string())
            .collect();
        assert!(found.is_empty(), "{name}: {found:#?}");
    }
}

mod planted {
    use super::*;

    /// The file-register indexes this project would have if they were current.
    fn current_indexes(manifest: &Manifest) -> HashMap<PathBuf, String> {
        let model = Model::build(manifest, &[]).expect("a model");
        let survey = km::listing::survey(manifest, &model).expect("a survey of the mock");
        km::generated::file_register_indexes(&model, manifest, &survey.present, &survey.directories)
            .into_iter()
            .collect()
    }

    /// The findings of `check`'s last phase over a copy of the project.
    fn as_lines(manifest: &Manifest) -> Vec<String> {
        super::findings(manifest)
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    /// Writes each file of a generated set into a copy.
    fn write_all(dir: &Path, files: &HashMap<PathBuf, String>) {
        for (rel, text) in files {
            let path = dir.join(rel);
            std::fs::create_dir_all(path.parent().expect("a parent")).expect("a parent directory");
            std::fs::write(path, text).expect("a generated file");
        }
    }

    /// A copy with every generated file current but `ABSENT`, whose absence is the registers check's
    /// planted defect, shared by the tests that only read it.
    fn current() -> &'static Manifest {
        static COPY: OnceLock<Manifest> = OnceLock::new();
        COPY.get_or_init(|| {
            copy("current", "planted", |dir| {
                let mut want = current_indexes(&Manifest::load(dir).expect("the manifest"));
                want.remove(Path::new(ABSENT));
                write_all(dir, &want);
            })
        })
    }

    /// A copy with every generated file current but for what `edit` does to the set.
    fn edited(tag: &str, edit: impl FnOnce(&mut HashMap<PathBuf, String>)) -> Vec<String> {
        as_lines(&copy(tag, "planted", |dir| {
            let mut want = current_indexes(&Manifest::load(dir).expect("the manifest"));
            edit(&mut want);
            write_all(dir, &want);
        }))
    }

    /// A copy whose tree holds no generated file.
    fn none() -> Vec<String> {
        static LINES: OnceLock<Vec<String>> = OnceLock::new();
        LINES
            .get_or_init(|| {
                as_lines(&copy("none", "planted", |dir| {
                    let manifest = Manifest::load(dir).expect("the manifest");
                    for rel in current_indexes(&manifest).keys() {
                        let _ = std::fs::remove_file(dir.join(rel));
                    }
                }))
            })
            .clone()
    }

    fn findings() -> Vec<String> {
        static LINES: OnceLock<Vec<String>> = OnceLock::new();
        LINES.get_or_init(|| as_lines(current())).clone()
    }

    /// Every planted defect this project carries, by the check that reports it: the check,
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
        // there, per `design@check@goals-home-cites-upward-only`. And, in this copy that holds no
        // generated file, the three READMEs' links to an index that is not there.
        ("references", 31, "is referenced"),
        // One defect per shape assertion over what is there: an undeclared kind, a missing owed
        // subsection, an undeclared group, a file of another suffix, and frontmatter that does not
        // parse; and, in this copy that holds no generated file, a missing index for each of the
        // six homes.
        ("registers", 11, "issue register"),
        // Six register indexes: three issue homes, the two plans homes and the one milestone.
        // This copy holds none of them, so each is reported missing.
        ("generated", 6, "the generated file is missing"),
    ];

    #[test]
    fn every_check_has_a_row_and_the_run_is_their_sum() {
        // Read off `CHECKS` rather than off `PLANTED`, so a check added to the library with no
        // planted defect fails here instead of being absent from both. And nothing is invented
        // by running them together: the whole run is the sum of the rows.
        for name in CHECKS {
            // Every mock but `unsound` declares no harness, so `agents` finds nothing here: its
            // defects are built by its unit tests and by the binary tests in a copy, since a binary
            // run over a mock that served the harness would list every file of the shipped set.
            if name == "agents" {
                continue;
            }
            assert!(
                PLANTED.iter().any(|(n, _, _)| *n == name),
                "{name} has no row: every check owes a planted defect"
            );
        }
        let whole = none();
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
        let manifest = Manifest::load(&source("planted")).expect("the mock's manifest");
        let want = current_indexes(&manifest);
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
        let edited = edited("trailing-break", |c| {
            let text = format!("{}\n", c[&path]);
            c.insert(path.clone(), text);
        });
        let hits: Vec<&String> = edited
            .iter()
            .filter(|f| f.contains("out of date"))
            .collect();
        assert_eq!(hits.len(), 1, "{edited:#?}");
        assert!(hits[0].starts_with(CURRENT[0]), "{hits:#?}");
    }

    /// An edit that keeps the file's length is out of date too: the comparison reads the bytes.
    #[test]
    fn an_index_edited_to_the_same_length_is_out_of_date() {
        let path = PathBuf::from(CURRENT[0]);
        let edited = edited("same-length", |c| {
            let text = c[&path].replacen("index", "indeX", 1);
            assert_ne!(text, c[&path], "the edit changes a byte");
            c.insert(path.clone(), text);
        });
        assert!(
            edited
                .iter()
                .any(|f| f.starts_with(CURRENT[0]) && f.contains("out of date")),
            "{edited:#?}"
        );
    }

    #[test]
    fn a_generated_file_that_is_absent_is_reported_as_missing() {
        let gone = none();
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
        let here: Vec<(u32, String, String)> = super::findings(current())
            .into_iter()
            .filter(|f| f.file == Path::new("notes/plans.md"))
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
        let edited = edited("hand-edited", |c| {
            let text = c[&path].replace("| todo |", "| defect |");
            c.insert(path.clone(), text);
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
