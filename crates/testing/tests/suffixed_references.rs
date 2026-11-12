//! A reference whose id, or whose path after its anchor, carries a line suffix or a fragment,
//! checked through `check`'s public API over a whole project: each resolution path names the
//! repair of the suffix, where the entry or the file exists and only the suffix fails, and where
//! the rest fails too. The resolver's own tests hold each cause; this one holds what a finding
//! prints, the ignore batch the run gathers included.

use std::path::{Path, PathBuf};
use std::process::Command;

use knowledge_architect_check::commands::Gathered;
use knowledge_architect_check::{Finding, Manifest, Model};
use knowledge_architect_testing::run_with;

fn git(dir: &Path, args: &[&str]) {
    // The variables that would point it elsewhere are removed: a suite run from a commit hook
    // inherits them.
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

fn write(dir: &Path, rel: &str, text: &str) {
    let path = dir.join(rel);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("a directory");
    std::fs::write(path, text).expect("a written file");
}

/// A copy of `core` that serves the default harness, with a skill holding one section, a section
/// of the root CLAUDE.md, an ignored file, and the documents given. The installed set is not
/// there, so the run reports it missing; the tests read the findings of their own documents.
fn project(tag: &str, documents: &[(&str, &str)]) -> Manifest {
    let dir: PathBuf = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("suffixed-references-{}", std::process::id()))
        .join(tag);
    copy_tree(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/projects/core"),
        &dir,
    );
    let manifest = std::fs::read_to_string(dir.join("knowledge-architect.toml")).expect("read");
    let served = manifest
        .split_once("[agents]")
        .map_or(manifest.as_str(), |(head, _)| head);
    write(&dir, "knowledge-architect.toml", served);
    write(
        &dir,
        ".claude/skills/demo-x/SKILL.md",
        "---\nname: demo-x\ndescription: A demo.\n---\n\n# Demo\n\n## A part `##a-part`\n\nText.\n",
    );
    let claude = std::fs::read_to_string(dir.join("CLAUDE.md")).expect("read");
    write(
        &dir,
        "CLAUDE.md",
        &format!("{claude}\n## Some rules `##some-rules`\n\nText.\n"),
    );
    write(&dir, ".gitignore", "gen/\n");
    write(&dir, "gen/out.md", "Built.\n");
    for (rel, text) in documents {
        write(&dir, rel, text);
    }
    git(&dir, &["init", "-q"]);
    git(&dir, &["-c", "core.excludesFile=/dev/null", "add", "-A"]);
    Manifest::load(&dir).expect("the copy's manifest")
}

/// The findings of a run over `manifest` on the document `rel`, each as its statement and its
/// repair.
fn findings_in(manifest: &Manifest, rel: &str) -> Vec<(String, String)> {
    let model = Model::build(manifest, &[]).expect("a model");
    let gathered = Gathered::over(manifest, &model).expect("what a run reads");
    run_with(&model, manifest, &gathered.inputs(), &[])
        .findings
        .into_iter()
        .filter(|f: &Finding| f.file == Path::new(rel))
        .map(|f| (f.what, f.action))
        .collect()
}

const DROP: &str = "A reference names its target whole: drop the line number or the fragment, \
                    and name the point inside it in prose";

/// The claim: for each resolution path, a register kind, a `register` home, a harness kind with
/// each of its three repairs, `path` under a named anchor and under `*`, and `planned`, a
/// reference whose entry or file exists and whose suffix alone fails is told to drop the suffix,
/// naming the form without it; and one whose rest fails too keeps the repair of what fails, the
/// sentence on the suffix after it. The project's design entry is `one-decision`. Mutation
/// checked, and caught: the call to `located` in `path` of `path@model@src/resolver.rs` made
/// `resolve(path)`, which leaves the file under `gen/` with no finding and gives `README.md`
/// the `Missing` cause.
#[test]
fn each_resolution_path_names_the_repair_of_a_suffix() {
    let spans = [
        (
            "design@core-mock@one-decision#top",
            "design@core-mock@one-decision",
        ),
        (
            "design@core-mock@one-decision:12",
            "design@core-mock@one-decision",
        ),
        ("register@core-mock@design:3", "register@core-mock@design"),
        ("skill@demo-x@a-part:3", "skill@demo-x@a-part"),
        ("skill@demo-x#top", "skill@demo-x"),
        ("instructions@some-rules:3-4", "instructions@some-rules"),
        ("path@core-mock@README.md:3", "path@core-mock@README.md"),
        ("path@core-mock@README.md#top", "path@core-mock@README.md"),
        ("path@core-mock@gen/out.md:3", "path@core-mock@gen/out.md"),
        ("path@*@README.md:3", "path@*@README.md"),
    ];
    let text: String = spans
        .iter()
        .map(|(span, _)| format!("See `{span}`.\n"))
        .collect();
    let m = project("located", &[("notes.md", &format!("# Notes\n\n{text}"))]);
    let found = findings_in(&m, "notes.md");
    assert_eq!(found.len(), spans.len(), "{found:#?}");
    for ((what, action), (span, form)) in found.iter().zip(spans) {
        assert_eq!(what, &format!("`{span}` points inside `{form}`"));
        assert_eq!(
            action,
            &format!(
                "drop the line number or the fragment, as `{form}`, and name the point inside \
                 it in prose"
            )
        );
    }
}

/// The claim: where the rest of a suffixed reference fails too, the finding is the one its form
/// without the suffix meets, naming the id a writer can define, and its repair ends with the
/// sentence on the suffix.
#[test]
fn a_suffixed_reference_whose_rest_fails_keeps_its_repair_and_adds_the_suffix_s() {
    let text = "# Notes\n\nSee `design@core-mock@no-decision#top`.\n\
                See `skill@demo-x@no-part#x`.\n\
                See `skill@no-skill:3`.\n\
                See `instructions@no-rules:3`.\n\
                See `path@core-mock@void.md#x`.\n\
                See `path@*@void.md:3`.\n";
    let m = project("suffixed", &[("notes.md", text)]);
    let found = findings_in(&m, "notes.md");
    let expected = [
        (
            "`design@core-mock@no-decision#top` is referenced and `core-mock` defines no design \
             `no-decision`",
            "define it in the design home of `core-mock`, or repair the reference",
        ),
        (
            "`skill@demo-x@no-part#x` is referenced and the skill `demo-x` defines no section \
             `no-part`",
            "end a level-2 heading of the skill `demo-x` with that slug, or repair the reference",
        ),
        (
            "`skill@no-skill:3` is referenced and no skill is named `no-skill`",
            "name an existing skill, or repair the reference",
        ),
        (
            "`instructions@no-rules:3` is referenced and the root CLAUDE.md defines no section \
             `no-rules`",
            "end a level-2 heading of the root CLAUDE.md with that slug, or repair the reference",
        ),
        (
            "`path@core-mock@void.md#x` does not exist, at `void.md`",
            "repair the pointer, or delete it",
        ),
        (
            "`path@*@void.md:3` resolves in no component",
            "the generic form names a required document, or a path at least one component \
             carries; repair the path, or anchor at one component",
        ),
    ];
    let expected: Vec<(String, String)> = expected
        .iter()
        .map(|(what, action)| (what.to_string(), format!("{action}. {DROP}")))
        .collect();
    assert_eq!(found, expected);
}

/// The claim: a suffixed planned path in a plan document names the `path` form of a file that
/// exists, keeping a directory's slash, the planned form of a file that does not, and a slice
/// spec by its kind; outside the plans directory, a planned path to a file that exists names
/// its `path` form.
#[test]
fn a_suffixed_planned_path_names_the_form_without_its_suffix() {
    let plans = "# Plans\n\nIt edits `planned@core-mock@README.md:3`.\n\
                 It edits `planned@core-mock@docs:3`.\n\
                 It writes `planned@core-mock@new.md#a-head`.\n\
                 It edits `planned@core-mock@docs/plans/milestones/m-one/a-slice.md:3`.\n";
    let m = project(
        "planned",
        &[
            ("docs/plans/README.md", plans),
            (
                "notes.md",
                "# Notes\n\nSee `planned@core-mock@README.md`.\n",
            ),
        ],
    );
    let found = findings_in(&m, "docs/plans/README.md");
    let actions: Vec<&str> = found.iter().map(|(_, a)| a.as_str()).collect();
    assert_eq!(
        actions,
        [
            "write `path@core-mock@README.md`, and name the function or the heading in prose",
            "write `path@core-mock@docs/`, and name the function or the heading in prose",
            "drop the line number or the fragment, as `planned@core-mock@new.md`, and name the \
             point inside it in prose",
            &format!(
                "cite it as `spec@m-one@a-slice`; a plan document has one name, so `show` finds \
                 every citation of it. {DROP}"
            ),
        ],
        "{found:#?}"
    );
    let found = findings_in(&m, "notes.md");
    assert_eq!(
        found,
        [(
            "`planned@core-mock@README.md` is a planned path outside the plans directory"
                .to_string(),
            "write `path@core-mock@README.md`, since the target exists; a planned path is cited \
             from a plan document only"
                .to_string()
        )]
    );
}
