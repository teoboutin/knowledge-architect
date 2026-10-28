//! `klarch init` and `klarch new-component`, run as a caller runs them, in scratch directories
//! outside this repository, per `design@core@init-command` and `design@core@component-command`.

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

/// The claim: each row of the table of `design@core@init-runs-git-init`, outside a worktree and
/// inside one.
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

/// `init` then `check` passes, at the root: a document the check comes to require that `init` does
/// not write fails it, per `design@core@init-command`.
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

/// The claim: under `--vcs none` outside every worktree, the documents and the agent files are
/// written, no index and no check run, and the run exits 0 saying what comes next.
#[test]
fn outside_a_worktree_the_run_stops_before_the_indexes() {
    let scratch = Scratch::new("no-vcs");
    let (out, err, code) = klarch(&scratch.dir, &["init", "p", "--vcs", "none"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("git init"), "{out}");
    assert!(scratch.path("p/docs/goals.md").is_file());
    assert!(scratch
        .path("p/.claude/knowledge-architect/PRIMER.md")
        .is_file());
    assert!(!scratch.path("p/docs/open-issues/index.md").exists());
}

/// The claim: `init` over a repository whose index holds staged changes, and no manifest,
/// writes the indexes and passes the check: a commit of that index is no project, so `--fix`
/// has no staged tree to compare with.
#[test]
fn init_over_staged_changes_passes() {
    let scratch = Scratch::new("staged");
    let dir = scratch.path("p");
    std::fs::create_dir(&dir).unwrap();
    git(&dir, &["init", "-q"]);
    std::fs::write(dir.join("notes.txt"), "kept\n").unwrap();
    git(&dir, &["add", "notes.txt"]);
    let (out, err, code) = klarch(&dir, &["init"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(dir.join("docs/open-issues/index.md").is_file());
}

/// The comment lines of a manifest, in order.
fn comment_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.find('#').map(|at| l[at..].to_string()))
        .collect()
}

/// A manifest's comments survive `new-component` byte for byte, in their order, per
/// `design@core@component-manifest-edit`.
#[test]
fn new_component_keeps_every_comment_of_the_manifest() {
    let scratch = Scratch::new("ac1");
    assert_eq!(klarch(&scratch.dir, &["init", "p"]).2, 0);
    let path = scratch.path("p/knowledge-architect.toml");
    let text = std::fs::read_to_string(&path).unwrap();
    let text = format!("# above the project table\n{text}")
        .replace(
            "components = []",
            "components = [\n    # inside the array\n]  # after the array",
        )
        .replace("skip-dirs = []", "skip-dirs = []  # in the walk table");
    std::fs::write(&path, &text).unwrap();
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "svc"]);
    assert_eq!(code, 0, "{out}{err}");
    let after = std::fs::read_to_string(&path).unwrap();
    assert_eq!(comment_lines(&after), comment_lines(&text), "{after}");
    let value: toml::Table = toml::from_str(&after).unwrap();
    assert_eq!(
        value["project"]["components"].as_array().unwrap().len(),
        1,
        "{after}"
    );

    // A reason beside an element stays on that element's line, with or without its comma, and
    // the new element takes a line of its own.
    for (case, listed) in [
        "components = [\n    \"a\", # reason for a\n    \"b\", # reason for b\n]",
        "components = [\n    \"a\", # reason for a\n    \"b\" # reason for b\n]",
    ]
    .into_iter()
    .enumerate()
    {
        let project = format!("q{case}");
        let args = ["init", &project, "--component", "a", "--component", "b"];
        assert_eq!(klarch(&scratch.dir, &args).2, 0);
        let path = scratch.path(&format!("{project}/knowledge-architect.toml"));
        let written = std::fs::read_to_string(&path).unwrap();
        let line = written
            .lines()
            .find(|l| l.starts_with("components = "))
            .unwrap();
        std::fs::write(&path, written.replace(line, listed)).unwrap();
        let (out, err, code) = klarch(&scratch.path(&project), &["new-component", "svc"]);
        assert_eq!(code, 0, "{out}{err}");
        let after = std::fs::read_to_string(&path).unwrap();
        for element in ["a", "b"] {
            let line = format!("    \"{element}\", # reason for {element}");
            assert!(
                after.lines().any(|l| l == line),
                "{line:?} not in:\n{after}"
            );
        }
        assert!(
            after
                .lines()
                .any(|l| l.trim_end_matches(',') == "    \"svc\""),
            "{after}"
        );
    }
}

/// `new-component` then `check` passes, and `init --component` lists the
/// Component once and passes too.
#[test]
fn a_component_added_either_way_passes_the_check() {
    let scratch = Scratch::new("ac3-component");
    assert_eq!(klarch(&scratch.dir, &["init", "p"]).2, 0);
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "crates/core"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(scratch.path("p/crates/core/docs/goals.md").is_file());
    assert_eq!(klarch(&scratch.path("p"), &["check"]).2, 0);

    let (out, err, code) = klarch(&scratch.dir, &["init", "q", "--component", "svc"]);
    assert_eq!(code, 0, "{out}{err}");
    let manifest = std::fs::read_to_string(scratch.path("q/knowledge-architect.toml")).unwrap();
    assert_eq!(manifest.matches("\"svc\"").count(), 1, "{manifest}");
    assert!(scratch.path("q/svc/README.md").is_file());
}

/// The claims: over a tree phases 1 to 3 find incomplete, `new-component` refuses, exit 2,
/// writing nothing; a DIR named like the project refuses with the manifest's complaint, writing
/// nothing; so does an `index.md` already in the new Component.
#[test]
fn new_component_refuses_before_any_write() {
    let scratch = Scratch::new("nc-refuse");
    assert_eq!(klarch(&scratch.dir, &["init", "p"]).2, 0);
    let manifest = scratch.path("p/knowledge-architect.toml");
    let before = std::fs::read_to_string(&manifest).unwrap();

    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "p"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(err.contains("names 2 anchors"), "{err}");
    assert_eq!(std::fs::read_to_string(&manifest).unwrap(), before);
    assert!(!scratch.path("p/p").exists());

    std::fs::remove_file(scratch.path("p/docs/design.md")).unwrap();
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "svc"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(err.contains("incomplete"), "{err}");
    assert_eq!(std::fs::read_to_string(&manifest).unwrap(), before);
    assert!(!scratch.path("p/svc").exists());

    // An index this tool did not write, in the new Component, refuses it.
    scratch.write("p/docs/design.md", "# Design\n\nNo entry yet.\n");
    scratch.write("p/own/docs/open-issues/index.md", "# Our own table\n");
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "own"]);
    assert_eq!(code, 2, "{out}{err}");
    assert_eq!(std::fs::read_to_string(&manifest).unwrap(), before);
}

/// `init --rust` then `check` passes, the manifest declares the command and
/// the maintenance crate, and the files carry this version and the project's checkout variable.
#[test]
fn init_rust_then_check_passes() {
    let scratch = Scratch::new("ac3-rust");
    let (out, err, code) = klarch(&scratch.dir, &["init", "my-proj", "--rust"]);
    assert_eq!(code, 0, "{out}{err}");
    assert_eq!(klarch(&scratch.path("my-proj"), &["check"]).2, 0);
    let read = |rel: &str| std::fs::read_to_string(scratch.path("my-proj").join(rel)).unwrap();
    let manifest: toml::Table = toml::from_str(&read("knowledge-architect.toml")).unwrap();
    assert_eq!(
        manifest["project"]["command"].as_str(),
        Some("cargo klarch")
    );
    assert_eq!(
        manifest["project"]["components"].as_array().unwrap().len(),
        1
    );
    assert!(read("xtask/Cargo.toml").contains(&format!(
        "knowledge-architect = \"={}\"",
        knowledge_architect::CHECKER_VERSION
    )));
    assert!(read(".cargo/config.toml").contains("\nMY_PROJ_CHECKOUT = {"));
    assert!(read("xtask/src/main.rs").contains("option_env!(\"MY_PROJ_CHECKOUT\")"));
    assert_eq!(read(".gitignore"), "/target\n");
}

/// The claims: an existing `.gitignore` keeps every line and gains `/target` once; one that
/// already ignores it is untouched; a PATH holding a `Cargo.toml` refuses `--rust`, exit 2,
/// writing nothing.
#[test]
fn rust_keeps_the_gitignore_and_refuses_an_existing_cargo_toml() {
    let scratch = Scratch::new("rust-ignore");
    scratch.write("a/.gitignore", "mine");
    let (out, err, code) = klarch(&scratch.dir, &["init", "a", "--rust", "--vcs", "git"]);
    assert_eq!(code, 0, "{out}{err}");
    assert_eq!(
        std::fs::read_to_string(scratch.path("a/.gitignore")).unwrap(),
        "mine\n\n# Added by klarch\n/target\n"
    );

    scratch.write("b/.gitignore", "target/\n");
    let (out, err, code) = klarch(&scratch.dir, &["init", "b", "--rust", "--vcs", "git"]);
    assert_eq!(code, 0, "{out}{err}");
    assert_eq!(
        std::fs::read_to_string(scratch.path("b/.gitignore")).unwrap(),
        "target/\n"
    );

    scratch.write("c/Cargo.toml", "[package]\nname = \"c\"\n");
    let before = listing(&scratch.path("c"));
    let (out, err, code) = klarch(&scratch.dir, &["init", "c", "--rust", "--vcs", "git"]);
    assert_eq!(code, 2, "{out}{err}");
    assert_eq!(listing(&scratch.path("c")), before);
}

/// Stdout, stderr and the exit code of one run in `dir` with environment variables set.
fn klarch_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, i32) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_klarch"));
    command
        .args(args)
        .current_dir(dir)
        .stdin(std::process::Stdio::null());
    for (key, value) in env {
        command.env(key, value);
    }
    let out = command.output().expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// The claims of the git rule's edges: a new PATH under a worktree gets no repository; `--vcs git`
/// over an existing directory inside a worktree creates one at PATH; over a repository's root it
/// creates none and says nothing of one; a git failure other than "not a git repository", here a
/// repository git refuses as another user's, refuses the run instead of reading as outside every
/// repository; a variable that points git at another repository's files refuses the run.
#[test]
fn the_git_rule_holds_at_its_edges() {
    let scratch = Scratch::new("git-edges");
    let repo = |rel: &str| scratch.path(rel).join(".git").exists();
    std::fs::create_dir_all(scratch.path("host")).unwrap();
    git(&scratch.path("host"), &["init", "-q"]);

    assert_eq!(klarch(&scratch.path("host"), &["init", "new/deep"]).2, 0);
    assert!(
        !repo("host/new/deep"),
        "a new PATH under a worktree gets no repository"
    );

    scratch.write("host/existing/notes.txt", "mine");
    let (out, err, code) = klarch(&scratch.path("host"), &["init", "existing", "--vcs", "git"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        repo("host/existing"),
        "--vcs git creates one at an existing PATH"
    );

    std::fs::create_dir_all(scratch.path("root")).unwrap();
    git(&scratch.path("root"), &["init", "-q"]);
    let (out, err, code) = klarch(&scratch.dir, &["init", "root", "--vcs", "git"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains("created  a git repository"), "{out}");

    // A machine whose configuration sets `safe.directory = *` makes git trust every repository, so
    // the refusal is reached only with the machine's configuration kept out,
    // per `issue@core@the-tests-read-the-machine-s-git-configuration`.
    scratch.write("empty.gitconfig", "");
    let empty_config = scratch.path("empty.gitconfig");
    let (out, err, code) = klarch_env(
        &scratch.path("host"),
        &["init", "owned"],
        &[
            ("GIT_TEST_ASSUME_DIFFERENT_OWNER", "1"),
            ("GIT_CONFIG_NOSYSTEM", "1"),
            ("GIT_CONFIG_GLOBAL", empty_config.to_str().unwrap()),
        ],
    );
    assert_eq!(code, 2, "{out}{err}");
    assert!(!scratch.path("host/owned").exists());

    for (variable, value) in [
        ("GIT_DIR", "host/.git"),
        ("GIT_WORK_TREE", "host"),
        ("GIT_INDEX_FILE", "host/.git/index"),
        ("GIT_COMMON_DIR", "host/.git"),
        ("GIT_OBJECT_DIRECTORY", "host/.git/objects"),
    ] {
        let (out, err, code) = klarch_env(
            &scratch.dir,
            &["init", "hooked"],
            &[(variable, scratch.path(value).to_str().unwrap())],
        );
        assert_eq!(code, 2, "{variable}: {out}{err}");
        assert!(!scratch.path("hooked").exists(), "{variable}");
    }
}

/// The claims of `--rust` against what exists: a Cargo.toml above PATH refuses it, as one at
/// PATH does; an existing file of the four it writes refuses it, a symlink included; a
/// `.gitignore` that is a symlink or a directory refuses it; each refusal writes nothing.
#[test]
fn rust_refuses_what_it_would_overwrite_or_nest_in() {
    let scratch = Scratch::new("rust-refuse");
    git(&scratch.dir, &["init", "-q"]);
    let refused = |rel: &str| {
        let before = listing(&scratch.dir);
        let (out, err, code) = klarch(&scratch.dir, &["init", rel, "--rust"]);
        assert_eq!(code, 2, "{rel}: {out}{err}");
        assert_eq!(
            listing(&scratch.dir),
            before,
            "{rel}: a refused run writes nothing"
        );
    };
    scratch.write("ws/Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
    refused("ws/crates/kp");
    scratch.write("cc/.cargo/config.toml", "[build]\njobs = 1\n");
    refused("cc");
    scratch.write("xf/xtask", "a file where the crate's directory is due");
    refused("xf");
    scratch.write("elsewhere.txt", "not this project's");
    std::fs::create_dir_all(scratch.path("sl")).unwrap();
    std::os::unix::fs::symlink(scratch.path("elsewhere.txt"), scratch.path("sl/.gitignore"))
        .unwrap();
    refused("sl");
    std::fs::create_dir_all(scratch.path("dg/.gitignore")).unwrap();
    refused("dg");
    assert_eq!(
        std::fs::read_to_string(scratch.path("elsewhere.txt")).unwrap(),
        "not this project's"
    );
}

/// The claims of the `.gitignore` rule: inside a repository, git decides whether `/target` is
/// ignored, so each usual spelling is kept and a negated or indented one is not taken for it;
/// outside every repository, a line holding one of the four spellings is; an empty file gains
/// the rule; inside an existing worktree git decides too.
#[test]
fn the_gitignore_rule_follows_what_git_ignores() {
    let scratch = Scratch::new("ignore");
    let after = |rel: &str, given: &str, vcs: &str| -> String {
        scratch.write(&format!("{rel}/.gitignore"), given);
        let (out, err, code) = klarch(&scratch.dir, &["init", rel, "--rust", "--vcs", vcs]);
        assert_eq!(code, 0, "{rel}: {out}{err}");
        std::fs::read_to_string(scratch.path(rel).join(".gitignore")).unwrap()
    };
    for (i, spelling) in ["/target", "/target/", "target", "target/"]
        .iter()
        .enumerate()
    {
        let given = format!("{spelling}\n");
        assert_eq!(after(&format!("kept{i}"), &given, "git"), given);
        assert_eq!(after(&format!("bare{i}"), &given, "none"), given);
    }
    let appended = "\n# Added by klarch\n/target\n";
    assert_eq!(
        after("negated", "target/\n!target/\n", "git"),
        format!("target/\n!target/\n{appended}")
    );
    assert_eq!(
        after("indented", "  target\n", "git"),
        format!("  target\n{appended}")
    );
    assert_eq!(after("empty", "", "git"), appended);

    // Inside an existing worktree, where no repository is created, git decides as well.
    std::fs::create_dir_all(scratch.path("host")).unwrap();
    git(&scratch.path("host"), &["init", "-q"]);
    scratch.write("host/inner/.gitignore", "/target*\n");
    let (out, err, code) = klarch(&scratch.path("host"), &["init", "inner", "--rust"]);
    assert_eq!(code, 0, "{out}{err}");
    assert_eq!(
        std::fs::read_to_string(scratch.path("host/inner/.gitignore")).unwrap(),
        "/target*\n"
    );
}

/// The claims: a file where the agent configuration's directory is due refuses the run before
/// its first write; a failure only the write itself meets, here a read-only `docs/`, comes after
/// the manifest is written, and makes the run exit 1, never 2.
#[test]
fn a_failure_after_the_first_write_exits_one() {
    use std::os::unix::fs::PermissionsExt;
    let scratch = Scratch::new("late-failure");
    git(&scratch.dir, &["init", "-q"]);
    scratch.write("a/.claude", "a file where a directory is due");
    let (out, err, code) = klarch(&scratch.dir, &["init", "a"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(!scratch.path("a/knowledge-architect.toml").exists());

    // The cases below make a write fail with a read-only directory, the one failure `init`'s
    // refusals before the first write cannot see. A process that writes into a read-only
    // directory anyway, as one running as root does, cannot build them, so they are skipped
    // there, and the skip is printed so that the run does not read as a pass of them.
    std::fs::create_dir_all(scratch.path("probe")).unwrap();
    std::fs::set_permissions(
        scratch.path("probe"),
        std::fs::Permissions::from_mode(0o555),
    )
    .unwrap();
    let writes_through = std::fs::write(scratch.path("probe/file"), "").is_ok();
    std::fs::set_permissions(
        scratch.path("probe"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    if writes_through {
        eprintln!(
            "skipped: this process writes into a read-only directory, as root does, so a write \
             failure after the first write cannot be built"
        );
        return;
    }

    std::fs::create_dir_all(scratch.path("b/docs")).unwrap();
    std::fs::set_permissions(
        scratch.path("b/docs"),
        std::fs::Permissions::from_mode(0o555),
    )
    .unwrap();
    let (out, err, code) = klarch(&scratch.dir, &["init", "b"]);
    std::fs::set_permissions(
        scratch.path("b/docs"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert_eq!(code, 1, "{out}{err}");
    assert!(scratch.path("b/knowledge-architect.toml").is_file());

    // The install, run by the code `check --fix` runs, fails having written nothing of its own,
    // which `--fix` reports as 2; this run had written, so it exits 1.
    std::fs::create_dir_all(scratch.path("c/.claude")).unwrap();
    std::fs::set_permissions(
        scratch.path("c/.claude"),
        std::fs::Permissions::from_mode(0o555),
    )
    .unwrap();
    let (out, err, code) = klarch(&scratch.dir, &["init", "c"]);
    std::fs::set_permissions(
        scratch.path("c/.claude"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert_eq!(code, 1, "{out}{err}");

    // `new-component` writes the manifest, then fails on a read-only Component directory.
    assert_eq!(klarch(&scratch.dir, &["init", "d"]).2, 0);
    std::fs::create_dir_all(scratch.path("d/svc")).unwrap();
    std::fs::set_permissions(
        scratch.path("d/svc"),
        std::fs::Permissions::from_mode(0o555),
    )
    .unwrap();
    let (out, err, code) = klarch(&scratch.path("d"), &["new-component", "svc"]);
    std::fs::set_permissions(
        scratch.path("d/svc"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert_eq!(code, 1, "{out}{err}");
}

/// The claims of the destination check, each a refusal before the first write, exit 2: a
/// dangling symlink at the manifest's path; an `index.md` this tool did not write at a generated
/// path; a FIFO at a destination; a file where a directory is due; a `--component` that overlaps
/// a file the run writes; a Component in a `.git` directory or in the installer's namespace; a
/// git that cannot run; the index of a directory register; a symlink where `--rust` writes; a
/// PATH that is a file, lies under one, is a dangling symlink or lies in `.git`; a PATH or a
/// Component git ignores. And: a home kept in its other shape is named as it stands.
#[test]
fn init_refuses_every_destination_it_cannot_write_safely() {
    let scratch = Scratch::new("destinations");
    git(&scratch.dir, &["init", "-q"]);
    let refused = |args: &[&str], dir: &str| {
        let before = listing(&scratch.dir);
        let (out, err, code) = klarch(&scratch.dir, args);
        assert_eq!(code, 2, "{args:?}: {out}{err}");
        assert_eq!(listing(&scratch.dir), before, "{args:?}: nothing written");
        let _ = dir;
    };
    std::fs::create_dir_all(scratch.path("a1")).unwrap();
    std::os::unix::fs::symlink(
        scratch.path("outside.toml"),
        scratch.path("a1/knowledge-architect.toml"),
    )
    .unwrap();
    refused(&["init", "a1"], "a1");
    assert!(!scratch.path("outside.toml").exists());

    scratch.write("a6/docs/open-issues/index.md", "# Our own table\n");
    refused(&["init", "a6"], "a6");

    std::fs::create_dir_all(scratch.path("a8")).unwrap();
    assert!(Command::new("mkfifo")
        .arg(scratch.path("a8/README.md"))
        .status()
        .unwrap()
        .success());
    refused(&["init", "a8"], "a8");

    scratch.write("fd/docs", "a file where a directory is due");
    refused(&["init", "fd"], "fd");

    refused(&["init", "ov", "--component", "README.md"], "ov");
    refused(&["init", "g1", "--component", ".git/c"], "g1");
    refused(
        &[
            "init",
            "ns",
            "--component",
            ".claude/skills/knowledge-architect-x",
        ],
        "ns",
    );

    let nogit = scratch.path("no-git-bin");
    std::fs::create_dir_all(&nogit).unwrap();
    let (out, err, code) = klarch_env(
        &scratch.dir,
        &["init", "ng", "--vcs", "none"],
        &[("PATH", nogit.to_str().unwrap())],
    );
    assert_eq!(code, 2, "{out}{err}");
    assert!(!scratch.path("ng").exists());

    scratch.write("a7/docs/plans/milestones/index.md", "# Our milestones\n");
    refused(&["init", "a7"], "a7");

    std::fs::create_dir_all(scratch.path("a9")).unwrap();
    std::os::unix::fs::symlink(scratch.path("elsewhere"), scratch.path("a9/.cargo")).unwrap();
    refused(&["init", "a9", "--rust"], "a9");
    assert!(!scratch.path("elsewhere").exists());

    scratch.write("plain", "a file at PATH");
    refused(&["init", "plain"], "plain");
    refused(&["init", "plain/p"], "plain/p");
    std::os::unix::fs::symlink(scratch.path("nowhere"), scratch.path("dangling")).unwrap();
    refused(&["init", "dangling"], "dangling");
    refused(&["init", ".git/proj"], ".git/proj");

    scratch.write(".gitignore", "ign/\nbuild/\n");
    refused(&["init", "ign/p"], "ign/p");
    refused(&["init", "cb", "--component", "build"], "cb");

    scratch.write("ks/docs/design/README.md", "# Our design\n");
    let (out, err, code) = klarch(&scratch.dir, &["init", "ks"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("kept     docs/design/README.md"), "{out}");
    assert!(!scratch.path("ks/docs/design.md").exists());
}

/// The claim: `init` runs the build-origin refusal over PATH, so a directory holding a copy of
/// this tool's crate, at the trailing path it was compiled at, refuses it.
#[test]
fn init_refuses_a_foreign_build_over_path() {
    let scratch = Scratch::new("foreign");
    git(&scratch.dir, &["init", "-q"]);
    scratch.write(
        "copy/crates/core/Cargo.toml",
        "[package]\nname = \"knowledge-architect\"\n",
    );
    let (out, err, code) = klarch(&scratch.dir, &["init", "copy", "--name", "copy"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(!scratch.path("copy/knowledge-architect.toml").exists());
}

/// The claims of `new-component`'s edge: a destination it cannot write refuses it with the
/// manifest untouched; a manifest written with CRLF line breaks keeps them, and one with a
/// byte-order mark or mixed line endings keeps every byte outside the array.
#[test]
fn new_component_keeps_the_manifest_whole() {
    let scratch = Scratch::new("nc-whole");
    assert_eq!(klarch(&scratch.dir, &["init", "p"]).2, 0);
    let manifest = scratch.path("p/knowledge-architect.toml");
    let before = std::fs::read_to_string(&manifest).unwrap();
    std::fs::create_dir_all(scratch.path("p/svc/README.md")).unwrap();
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "svc"]);
    assert_eq!(code, 2, "{out}{err}");
    assert_eq!(std::fs::read_to_string(&manifest).unwrap(), before);

    std::fs::write(&manifest, before.replace('\n', "\r\n")).unwrap();
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "lib"]);
    assert_eq!(code, 0, "{out}{err}");
    let after = std::fs::read_to_string(&manifest).unwrap();
    assert_eq!(
        after.matches('\n').count(),
        after.matches("\r\n").count(),
        "{after:?}"
    );

    // Every byte outside the array is kept: a byte-order mark, and line endings that mix; the
    // array's own `\n` line breaks stay `\n`.
    let mixed = format!("\u{feff}{}", before.replacen('\n', "\r\n", 1))
        .replace("components = []", "components = [\n    \"lib\",\n]");
    std::fs::write(&manifest, &mixed).unwrap();
    let (out, err, code) = klarch(&scratch.path("p"), &["new-component", "app"]);
    assert!(code < 2, "{out}{err}");
    assert!(out.contains("wrote    app/README.md"), "{out}");
    assert_eq!(
        std::fs::read_to_string(&manifest).unwrap(),
        mixed.replace("    \"lib\",\n]", "    \"lib\",\n    \"app\",\n]")
    );
}

/// The claim: a stdout that cannot be written ends `init` with 2, as every command, only once
/// every document is written, so `check --fix` completes the project; and `new-component` the
/// same.
#[cfg(target_os = "linux")]
#[test]
fn a_closed_stdout_leaves_every_document_written() {
    let scratch = Scratch::new("full");
    let full = || {
        std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/full")
            .unwrap()
    };
    let status = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .args(["init", "p"])
        .current_dir(&scratch.dir)
        .stdin(std::process::Stdio::null())
        .stdout(full())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(2));
    let (out, err, code) = klarch(&scratch.path("p"), &["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");

    git(&scratch.path("p"), &["add", "-A"]);
    git(
        &scratch.path("p"),
        &[
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "-qm",
            "p",
        ],
    );
    let status = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .args(["new-component", "svc"])
        .current_dir(scratch.path("p"))
        .stdin(std::process::Stdio::null())
        .stdout(full())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(2));
    let (out, err, code) = klarch(&scratch.path("p"), &["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
}
