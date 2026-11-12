//! The repair of a slug written where it defines nothing, over a whole project: it names the
//! reference form of what the slug resolves to, since its writer may have meant a pointer, per
//! `design@check@finding-names-the-repair`, and keeps the move or the deletion for a slug that
//! names nothing. The two reproductions of
//! `issue@check@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to` are the first
//! two tests.
//!
//! The project is a copy of the mock `core`, written in the target directory with the files each
//! test plants and served the `claude` harness, so the root CLAUDE.md is a section home; the
//! entity table is called directly, as phase 3 builds it.

use std::path::{Path, PathBuf};
use std::process::Command;

use km::table::{Anchors, Entities};
use km::{Finding, Manifest, Model};
use knowledge_architect_model as km;

fn source(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory in the copy");
    for entry in std::fs::read_dir(from).expect("a mock directory") {
        let entry = entry.expect("an entry of the mock");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("its type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copied file");
        }
    }
}

fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .current_dir(dir)
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?} failed in {}", dir.display());
}

/// A manifest of `core`'s, serving the default harness: the root CLAUDE.md defines its sections.
const MANIFEST: &str = "[project]\nchecker-version = \"fixture\"\nname = \"core-mock\"\n\
                        components = []\n\n[walk]\nskip-dirs = []\nskip-files = []\n";

/// The findings of the entity table over a copy of `core` holding `files`, written over the
/// copy's own.
fn definition_findings(tag: &str, files: &[(&str, &str)]) -> Vec<Finding> {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("misplaced-slugs-{}", std::process::id()))
        .join(tag);
    let _ = std::fs::remove_dir_all(&dir);
    copy_tree(&source("core"), &dir);
    std::fs::write(dir.join("knowledge-architect.toml"), MANIFEST).expect("the manifest");
    for (rel, text) in files {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("its directory");
        std::fs::write(path, text).expect("a planted file");
    }
    git(&dir, &["init", "-q"]);
    git(&dir, &["-c", "core.excludesFile=/dev/null", "add", "-A"]);
    let manifest = Manifest::load(&dir).expect("the copy's manifest");
    let model = Model::build(&manifest, &[]).expect("a model");
    let anchors = Anchors::of(&manifest, model.listing());
    Entities::build(&model, &anchors)
        .definition_findings()
        .to_vec()
}

fn one(found: &[Finding], what: &str) -> Finding {
    let hits: Vec<&Finding> = found.iter().filter(|f| f.what.contains(what)).collect();
    assert_eq!(hits.len(), 1, "expected exactly one {what:?} in {found:#?}");
    hits[0].clone()
}

const CLAUDE_MD: &str =
    "# The project\n\n## Git `##git-workflow`\n\nAll work happens on a branch.\n";

/// The claim: a `##<slug>` in a plan document's prose naming a section of the root CLAUDE.md is
/// told the section's reference form beside the move. Mutation checked: no form added to the
/// repair.
#[test]
fn a_slug_in_a_plan_naming_a_section_of_the_root_instructions_names_its_reference() {
    let spec = "# A spec\n\nThe branch follows `##git-workflow` of the root CLAUDE.md.\n";
    let found = definition_findings(
        "instructions",
        &[
            ("CLAUDE.md", CLAUDE_MD),
            ("docs/plans/specs/a-spec.md", spec),
        ],
    );
    let f = one(&found, "`##git-workflow` is written");
    assert_eq!(f.file, Path::new("docs/plans/specs/a-spec.md"));
    assert!(
        f.action.ends_with(
            "move it there, or delete it, or, to point at it, write `instructions@git-workflow`"
        ),
        "{f:#?}"
    );
}

/// The claim: an item cited in prose in its definition form, inside an item section of its own
/// plan, is told where it stands, in the middle of a line rather than outside the item sections,
/// and is told the item's reference form. Mutation checked: the site read as outside the item
/// sections whatever it is; no form added to the repair; the sort in line order leaving the
/// recorded index behind.
#[test]
fn an_item_cited_in_its_definition_form_names_the_item_s_reference() {
    let spec = "# A spec\n\n## Arguments\n\n### The argument `##the-argument`\n\nBody.\n\n\
                ## Threads\n\n### A thread `##a-thread`\n\nIt rests on `##the-argument`.\n\n\
                ### A thread with no slug\n";
    let found = definition_findings("item", &[("docs/plans/specs/a-spec.md", spec)]);
    // The heading's finding is found first and put after the slug's in line order, so the form
    // follows the slug's finding through that sort.
    let heading = one(&found, "\"A thread with no slug\"");
    assert!(!heading.action.contains("to point at"), "{heading:#?}");
    let f = one(&found, "`##the-argument` is written");
    assert_eq!(
        f.what,
        "`##the-argument` is written in the middle of a line of the plan `a-spec` and defines \
         nothing"
    );
    assert!(
        f.action
            .ends_with("or, to point at it, write `argument@a-spec@the-argument`"),
        "{f:#?}"
    );
}

/// The claim: an item of another plan is not offered, since an item is cited only from inside
/// its own plan, and a slug that names nothing keeps its repair as it was. Mutation checked: the
/// plan filter dropped, which offers `argument@a-spec@the-argument` in the other plan.
#[test]
fn an_item_of_another_plan_and_a_slug_naming_nothing_keep_the_move_alone() {
    let spec = "# A spec\n\n## Arguments\n\n### The argument `##the-argument`\n";
    let other = "# Another spec\n\nIt reads `##the-argument` and `##names-nothing`.\n";
    let found = definition_findings(
        "other-plan",
        &[
            ("docs/plans/specs/a-spec.md", spec),
            ("docs/plans/specs/another-spec.md", other),
        ],
    );
    for slug in ["`##the-argument`", "`##names-nothing`"] {
        let f = one(&found, &format!("{slug} is written"));
        assert_eq!(f.file, Path::new("docs/plans/specs/another-spec.md"));
        assert!(f.action.ends_with("move it there, or delete it"), "{f:#?}");
    }
}

/// The claim: outside a plan, a slug naming entries of several registers names every form, and a
/// slug in a section home of the harness names a section's. Mutation checked: the forms limited
/// to the first entity found; the misplaced slug of a section home left unrecorded.
#[test]
fn a_slug_naming_several_entities_names_each_form() {
    let design = "# Design\n\n### A decision `##one-subject`\n\nSee `##one-subject` below.\n";
    let issue = "---\nkind: defect\n---\n# The subject\n";
    let claude = "# The project\n\n## Git `##git-workflow`\n\nAs `##git-workflow` says.\n";
    let found = definition_findings(
        "several",
        &[
            ("CLAUDE.md", claude),
            ("docs/design.md", design),
            ("docs/open-issues/one-subject.md", issue),
        ],
    );
    let f = one(&found, "`##one-subject` is written");
    assert!(
        f.action.ends_with(
            "or, to point at the entity it names, write one of `design@core-mock@one-subject`, \
             `issue@core-mock@one-subject`"
        ),
        "{f:#?}"
    );
    let f = one(&found, "`##git-workflow` is written in the root CLAUDE.md");
    assert!(
        f.action
            .ends_with("move the slug to one, or delete it, or, to point at it, write `instructions@git-workflow`"),
        "{f:#?}"
    );
}
