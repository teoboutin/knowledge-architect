//! `klarch init`, run as a caller runs it, in scratch directories outside this repository, per
//! the claims of `spec@plans@init-and-new-component`.

use std::path::{Path, PathBuf};
use std::process::Command;

/// A scratch directory under the system's temporary directory, outside every git worktree,
/// removed when dropped.
struct Scratch {
    dir: PathBuf,
}

impl Scratch {
    fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("klarch-init-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Scratch { dir }
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.dir.join(rel)
    }

    fn write(&self, rel: &str, text: &str) {
        let path = self.path(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("a parent directory");
        std::fs::write(path, text).expect("a written file");
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// Stdout, stderr and the exit code of one run in `dir`, stdin closed.
fn klarch(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .args(args)
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Every path under `dir`, relative to it, `.git` left out.
fn listing(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(at) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&at) else {
            continue;
        };
        for entry in entries {
            let path = entry.expect("an entry").path();
            if path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            out.push(path.strip_prefix(dir).expect("under dir").to_path_buf());
            if path.is_dir() {
                stack.push(path);
            }
        }
    }
    out.sort();
    out
}

/// The claim: the manifest `init` writes pins the version of the binary that wrote it, and
/// names the project after PATH's basename. Shown to discriminate by a mutation that writes
/// another version.
#[test]
fn the_manifest_is_named_after_path_and_pinned_to_this_version() {
    let scratch = Scratch::new("pin");
    let (out, err, code) = klarch(&scratch.dir, &["init", "probe"]);
    assert_eq!(code, 0, "{out}{err}");
    let manifest =
        std::fs::read_to_string(scratch.path("probe/knowledge-architect.toml")).expect("written");
    let value: toml::Table = toml::from_str(&manifest).expect("the manifest parses");
    let project = value["project"].as_table().expect("[project]");
    assert_eq!(project["name"].as_str(), Some("probe"));
    assert_eq!(
        project["checker-version"].as_str(),
        Some(knowledge_architect::CHECKER_VERSION)
    );
    assert!(project.get("command").is_none(), "{manifest}");
}

/// The claim: `--command` is written as `[project] command`, and `--name` overrides the
/// basename.
#[test]
fn name_and_command_are_written_as_given() {
    let scratch = Scratch::new("named");
    let (out, err, code) = klarch(
        &scratch.dir,
        &[
            "init",
            "dir",
            "--name",
            "other-name",
            "--command",
            "tools/bin/klarch",
        ],
    );
    assert_eq!(code, 0, "{out}{err}");
    let manifest =
        std::fs::read_to_string(scratch.path("dir/knowledge-architect.toml")).expect("written");
    let value: toml::Table = toml::from_str(&manifest).expect("the manifest parses");
    assert_eq!(value["project"]["name"].as_str(), Some("other-name"));
    assert_eq!(
        value["project"]["command"].as_str(),
        Some("tools/bin/klarch")
    );
}

/// The claim: `init` refuses a PATH that holds a manifest, exit 2, writing nothing; a manifest
/// above PATH does not refuse it, since a project nested in another is legal.
#[test]
fn a_manifest_at_path_refuses_and_one_above_does_not() {
    let scratch = Scratch::new("existing");
    let (_, _, code) = klarch(&scratch.dir, &["init", "outer"]);
    assert_eq!(code, 0);
    let before = listing(&scratch.dir);
    let (out, err, code) = klarch(&scratch.dir, &["init", "outer"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        err.contains("already holds knowledge-architect.toml"),
        "{err}"
    );
    assert_eq!(
        listing(&scratch.dir),
        before,
        "a refused run writes nothing"
    );

    let (out, err, code) = klarch(&scratch.path("outer"), &["init", "inner"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(scratch
        .path("outer/inner/knowledge-architect.toml")
        .is_file());
}

/// The claim: a name the manifest refuses, or one inside the installer's namespace, refuses the
/// run, exit 2, and PATH is not even created. The installer's namespace without its hyphen is
/// accepted.
#[test]
fn a_refused_name_writes_nothing() {
    let scratch = Scratch::new("names");
    for name in ["design", "plans", "elsewhere", "knowledge-architect-x"] {
        let (out, err, code) = klarch(&scratch.dir, &["init", "p", "--name", name]);
        assert_eq!(code, 2, "{name}: {out}{err}");
        assert!(!scratch.path("p").exists(), "{name}: PATH was created");
    }
    let (out, err, code) = klarch(
        &scratch.dir,
        &["init", "p", "--name", "knowledge-architect"],
    );
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: each row of the spec's git table, outside a worktree and inside one.
#[test]
fn the_git_rule_holds_in_every_case() {
    let scratch = Scratch::new("git");
    let repo = |rel: &str| scratch.path(rel).join(".git").exists();

    // Outside every worktree.
    assert_eq!(klarch(&scratch.dir, &["init", "new"]).2, 0);
    assert!(repo("new"), "a new directory gets a repository");
    std::fs::create_dir_all(scratch.path("empty")).unwrap();
    assert_eq!(klarch(&scratch.dir, &["init", "empty"]).2, 0);
    assert!(repo("empty"), "an empty directory gets a repository");
    scratch.write("full/notes.txt", "mine");
    let (out, err, code) = klarch(&scratch.dir, &["init", "full"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(err.contains("--vcs git"), "{err}");
    assert!(!scratch.path("full/knowledge-architect.toml").exists());
    assert!(!repo("full"));
    assert_eq!(klarch(&scratch.dir, &["init", "full", "--vcs", "git"]).2, 0);
    assert!(repo("full"));
    scratch.write("bare/notes.txt", "mine");
    assert_eq!(
        klarch(&scratch.dir, &["init", "bare", "--vcs", "none"]).2,
        0
    );
    assert!(!repo("bare") && scratch.path("bare/knowledge-architect.toml").is_file());

    // Inside a worktree.
    std::fs::create_dir_all(scratch.path("host")).unwrap();
    git(&scratch.path("host"), &["init", "-q"]);
    scratch.write("host/sub/notes.txt", "mine");
    assert_eq!(klarch(&scratch.path("host"), &["init", "sub"]).2, 0);
    assert!(
        !repo("host/sub"),
        "inside a worktree no repository is created"
    );
    assert_eq!(
        klarch(&scratch.path("host"), &["init", "nested", "--vcs", "git"]).2,
        0
    );
    assert!(repo("host/nested"), "--vcs git creates one anyway");
}

/// AC3, root-only shape: `init` then `check` passes. Guards #init-file-contents of
/// `spec@plans@init-and-new-component`: a document the check comes to require that `init` does
/// not write fires it.
#[test]
fn init_then_check_passes_at_the_root() {
    let scratch = Scratch::new("ac3-root");
    let (out, err, code) = klarch(&scratch.dir, &["init", "probe"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("PASSED: no findings"), "{out}");
    let (out, err, code) = klarch(&scratch.path("probe"), &["check"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("PASSED: no findings"), "{out}");
    let goals = std::fs::read_to_string(scratch.path("probe/docs/goals.md")).expect("written");
    assert!(!goals.contains("\n## "), "no goal is written: {goals}");
}

/// The claim: an existing README.md and CLAUDE.md are kept byte for byte and listed as kept;
/// a kept CLAUDE.md that lacks the primer's import makes the run exit 1 with the check's
/// finding, since files were written.
#[test]
fn existing_documents_are_kept() {
    let scratch = Scratch::new("kept");
    git(&scratch.dir, &["init", "-q"]);
    scratch.write("README.md", "# Mine\n\nMy own words.\n");
    scratch.write("CLAUDE.md", "# Mine\n\nMy own instructions.\n");
    let (out, err, code) = klarch(&scratch.dir, &["init", "--name", "mine"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("kept     README.md"), "{out}");
    assert!(out.contains("kept     CLAUDE.md"), "{out}");
    assert!(
        out.contains("the root CLAUDE.md does not import the installed primer"),
        "{out}"
    );
    assert_eq!(
        std::fs::read_to_string(scratch.path("README.md")).unwrap(),
        "# Mine\n\nMy own words.\n"
    );
    assert_eq!(
        std::fs::read_to_string(scratch.path("CLAUDE.md")).unwrap(),
        "# Mine\n\nMy own instructions.\n"
    );
    assert!(
        scratch.path("docs/goals.md").is_file(),
        "the missing documents are written"
    );
}

/// The claim: a directory where a file is due, or a symlink at a destination, refuses the run
/// with exit 2 and nothing written, the manifest included.
#[test]
fn an_unwritable_destination_refuses_before_any_write() {
    let scratch = Scratch::new("unwritable");
    git(&scratch.dir, &["init", "-q"]);
    std::fs::create_dir_all(scratch.path("a/README.md")).unwrap();
    let before = listing(&scratch.path("a"));
    let (out, err, code) = klarch(&scratch.dir, &["init", "a"]);
    assert_eq!(code, 2, "{out}{err}");
    assert_eq!(listing(&scratch.path("a")), before);

    scratch.write("elsewhere.md", "not this project's");
    std::fs::create_dir_all(scratch.path("b/docs")).unwrap();
    std::os::unix::fs::symlink(
        scratch.path("elsewhere.md"),
        scratch.path("b/docs/goals.md"),
    )
    .unwrap();
    let (out, err, code) = klarch(&scratch.dir, &["init", "b"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(!scratch.path("b/knowledge-architect.toml").exists());
    assert_eq!(
        std::fs::read_to_string(scratch.path("elsewhere.md")).unwrap(),
        "not this project's"
    );
}

/// The claim: under `--vcs none` outside every worktree, the documents are written, no index
/// and no check run, and the run exits 0 saying what comes next.
#[test]
fn outside_a_worktree_the_run_stops_before_the_indexes() {
    let scratch = Scratch::new("no-vcs");
    let (out, err, code) = klarch(&scratch.dir, &["init", "p", "--vcs", "none"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("git init"), "{out}");
    assert!(scratch.path("p/docs/goals.md").is_file());
    assert!(!scratch.path("p/docs/open-issues/index.md").exists());
}
