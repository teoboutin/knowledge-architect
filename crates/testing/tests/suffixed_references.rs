//! A reference whose id, or whose path after its anchor, carries a line suffix or a fragment,
//! checked through `check`'s public API over a whole project: each resolution path names the
//! repair of the suffix where the entry or the file exists and only the suffix fails, and a
//! written form that resolves passes, an ignored path among them. The resolver's own tests hold
//! each cause; this one holds what a finding prints, and the answers of the ignore batch the run
//! gathers from git.

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

/// The claim: for each resolution path, a register kind, a `register` home, a harness kind with
/// each of its three repairs, `path` under a named anchor and under `*`, and `planned`, a
/// reference whose entry or file exists and whose suffix alone fails is told to drop the suffix,
/// naming the form without it; and one whose rest fails too keeps the repair of what fails, the
/// sentence on the suffix after it. The project's design entry is `one-decision`. Mutation
/// checked, and caught: the call to `located` in `path` of `path@model@src/resolver.rs` made
/// `resolve(path)`, which gives `README.md` the `Missing` cause.
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

/// The claim: a suffixed reference whose written form resolves passes, as it would with no
/// suffix: a path under a directory the ignore rules cover, whose written spelling git reports
/// ignored, and a planned path in a plan document whose target does not exist. A planned path
/// pointing into a file that exists beside them is reported, so the plan document is read. Mutation
/// checked: in `located` of `path@model@src/resolver.rs`, `written.is_ok() || ` removed, which
/// tells the ignored path to drop its suffix.
#[test]
fn a_suffixed_reference_whose_written_form_resolves_passes() {
    let plans = "# Plans\n\nIt writes `planned@core-mock@new.md#a-head`.\n\
                 It edits `planned@core-mock@README.md:3`.\n";
    let m = project(
        "resolves",
        &[
            ("docs/plans/README.md", plans),
            (
                "notes.md",
                "# Notes\n\nSee `path@core-mock@gen/out.md:3`.\n",
            ),
        ],
    );
    assert_eq!(findings_in(&m, "notes.md"), []);
    let found = findings_in(&m, "docs/plans/README.md");
    let actions: Vec<&str> = found.iter().map(|(_, a)| a.as_str()).collect();
    assert_eq!(
        actions,
        ["write `path@core-mock@README.md`, and name the function or the heading in prose"],
        "{found:#?}"
    );
}

/// The claim: a suffixed path whose spelling without the suffix climbs out of the root, as `..#a`,
/// is a finding of the run, which gathers its ignore batch from git without an error. Mutation
/// checked: in
/// `ignore_queries` of `path@model@src/resolver.rs`, `&& refused(bare).is_none()` removed, which
/// makes the gathering fail with git's `outside repository`.
#[test]
fn a_suffixed_upward_segment_is_a_finding_and_no_failed_run() {
    let m = project(
        "upward",
        &[(
            "notes.md",
            "# Notes\n\nSee `path@core-mock@..#a`.\nSee `path@*@..:3`.\n",
        )],
    );
    let found = findings_in(&m, "notes.md");
    assert_eq!(found.len(), 2, "{found:#?}");
    for (what, _) in &found {
        assert!(what.contains("is refused"), "{found:#?}");
    }
}
