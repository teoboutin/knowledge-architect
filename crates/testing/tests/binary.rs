//! `klarch`, run as a caller runs it, with no extension registered, per
//! `design@cli@the-commands-are-a-library-module`.
//!
//! The commands' mock projects sit under `path@testing@tests/projects/`, and a test runs in a copy of
//! one, since `"fixture"` is refused here: `klarch` links nothing of this crate, per
//! `design@check@checked-sentinel-values`.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, OnceLock};

/// The real binary, built from this checkout once per run of this test file.
///
/// `CARGO_BIN_EXE_klarch` is set in `klarch`'s own package alone, and a dev-dependency on it does not
/// build its binary, so this test asks the cargo that runs it, per
/// `design@testing@nested-cargo-builds-the-binary`. The path is the one cargo reports for the build, so a
/// `CARGO_BUILD_TARGET` or any other setting that moves the output cannot leave a test judging an older
/// binary. A binary that does not build fails each test that asks for it: a mutation run owes a
/// `--build` probe that builds `klarch`, or a mutant that does not compile reads as caught.
fn klarch() -> &'static Path {
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    BUILT.get_or_init(|| {
        // <target>/<profile>/deps/<this test>, or <target>/<triple>/<profile>/deps/<this test>.
        let exe = std::env::current_exe().expect("this test's path");
        let profile_dir = exe
            .parent()
            .and_then(Path::parent)
            .expect("a test sits under <profile>/deps");
        let profile = profile_dir
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a profile directory");
        let cargo = std::env::var_os("CARGO").expect("cargo sets CARGO for a test it runs");
        let mut build = Command::new(cargo);
        build
            .args([
                "build",
                "-q",
                "-p",
                "knowledge-architect",
                "--bin",
                "klarch",
            ])
            .arg("--message-format=json-render-diagnostics")
            .current_dir(env!("CARGO_MANIFEST_DIR"));
        match profile {
            "debug" => {}
            "release" => {
                build.arg("--release");
            }
            other => {
                build.args(["--profile", other]);
            }
        }
        let out = build.output().expect("cargo runs");
        assert!(
            out.status.success(),
            "klarch does not build: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let stdout = String::from_utf8_lossy(&out.stdout);
        built_executable(&stdout).expect("cargo reports the binary it built")
    })
}

/// The path of `klarch` in cargo's JSON messages: the artifact line whose target is named `klarch`.
fn built_executable(messages: &str) -> Option<PathBuf> {
    const KEY: &str = "\"executable\":\"";
    let line = messages
        .lines()
        .rfind(|l| l.contains("\"name\":\"klarch\"") && l.contains(KEY))?;
    let rest = &line[line.find(KEY)? + KEY.len()..];
    Some(PathBuf::from(&rest[..rest.find('"')?]))
}

/// The directory that holds the transcripts' fixtures: this crate's.
fn transcript_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// One mock project as it sits in this crate, the source of every copy.
fn source(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

/// A copy of a mock project pinned to this binary's version, a git repository of its own, shared by
/// the tests that only read it, so each mock is copied once per run.
///
/// It sits in the target directory under this process's id, so two runs over one target directory do
/// not meet, and nothing is left outside the target directory. A copy that fails fails the tests over
/// that mock alone.
fn project(name: &str) -> PathBuf {
    static COPIES: OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, Arc<OnceLock<PathBuf>>>>,
    > = OnceLock::new();
    let slot = COPIES
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .entry(name.to_string())
        .or_default()
        .clone();
    slot.get_or_init(|| {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("mock-copies-{}", std::process::id()))
            .join(name);
        copy_dir(&source(name), &dir);
        git(&dir, &["init", "-q"]);
        git(&dir, &["-c", "core.excludesFile=/dev/null", "add", "-A"]);
        dir
    })
    .clone()
}

/// Stdout, stderr and the exit code of one run in a directory.
fn run_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(klarch())
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// Stdout, stderr and the exit code of one run in one mock project.
fn run(name: &str, args: &[&str]) -> (String, String, i32) {
    run_in(&project(name), args)
}

/// The same, with `PATH` replaced, which is how the missing-`git` case is stated.
fn run_with_path(dir: &Path, args: &[&str], path: &Path) -> (String, String, i32) {
    run_with_env(dir, args, &[("PATH", path)])
}

/// The same, with environment variables replaced.
fn run_with_env(dir: &Path, args: &[&str], env: &[(&str, &Path)]) -> (String, String, i32) {
    let mut command = Command::new(klarch());
    command.args(args).current_dir(dir);
    for (name, value) in env {
        command.env(name, value);
    }
    let out = command.output().expect("the binary runs");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// A throwaway copy of a mock project, for the tests whose command writes.
///
/// A test that only reads shares the copy `project` gives, which works only while a run leaves the
/// tree alone. `index` writes, so it gets a copy of its own: writing into a shared one would leave the
/// next run comparing against the previous run's output rather than against the fixture.
///
/// **The copy is a git repository of its own.** The walk is `git ls-files` from the project
/// root, so a copy outside any worktree walks nothing and every test over it passes for the
/// wrong reason. `git init` and `git add -A` are what make the copied files live, and anything
/// a test wants staged — a `.gitignore` above all — is written by `seeded` BEFORE the add.
struct Sandbox {
    dir: PathBuf,
}

impl Sandbox {
    fn new(tag: &str, from: &str) -> Self {
        Self::seeded(tag, from, &[])
    }

    /// The same, with extra files written into the copy before it is staged.
    fn seeded(tag: &str, from: &str, files: &[(&str, &str)]) -> Self {
        let dir = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        copy_dir(&source(from), &dir);
        let sandbox = Sandbox { dir };
        for (rel, text) in files {
            sandbox.write(rel, text);
        }
        sandbox.git(&["init", "-q"]);
        // `add` needs no identity, so none is configured: a fixture that wrote one would be
        // recording a name in a temporary repository nobody reads.
        sandbox.stage();
        sandbox
    }

    /// A copy that is NOT a git repository, for the tests about a project outside a worktree.
    fn without_git(tag: &str, from: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        copy_dir(&source(from), &dir);
        Sandbox { dir }
    }

    /// Stage whatever the copy holds now.
    ///
    /// A test that deletes a fixture file has to call this: the walk reads git's listing, and
    /// a deleted file the index still holds is listed, read, and reported as unreadable.
    ///
    /// **The per-user ignore file is pinned away for this invocation and for no other.** `add`
    /// honours it, so a developer's global rule would otherwise decide which fixture files are
    /// tracked. Nothing is written to the copy's configuration: the tool's own pin is what a
    /// test of that pin has to be able to see.
    fn stage(&self) {
        self.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
    }

    /// One git command in the copy, not yet run, for a test that judges its exit itself.
    ///
    /// **It carries the sandbox's identity, and no other**: `user.name` and `user.email` on its
    /// command line, and the environment variables that would override them removed. A command
    /// that writes a commit asks for an identity, a merge before it starts, and a test that passed
    /// on a machine with an identity would otherwise fail on a runner with none; git also exports
    /// `GIT_AUTHOR_NAME` and its kin to a commit hook, so a suite run from one would otherwise
    /// commit under the hook's identity. The identity is written to no configuration file: a
    /// command line reaches only the command it is given to.
    fn git_command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("git");
        command
            .args([
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.invalid",
            ])
            .args(args)
            .env_remove("GIT_AUTHOR_NAME")
            .env_remove("GIT_AUTHOR_EMAIL")
            .env_remove("GIT_COMMITTER_NAME")
            .env_remove("GIT_COMMITTER_EMAIL")
            .env_remove("GIT_DIR")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("GIT_WORK_TREE")
            .current_dir(&self.dir);
        command
    }

    /// One git command in the copy, which must succeed, with the identity `git_command` gives.
    fn git(&self, args: &[&str]) {
        let out = self.git_command(args).output().expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?} in the sandbox: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.dir.join(rel)
    }

    /// Write a file into the copy; a manifest is written pinned to this binary's version.
    fn write(&self, rel: &str, text: &str) {
        let path = self.path(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
        let text = if rel == "knowledge-architect.toml" {
            pinned(text)
        } else {
            text.to_string()
        };
        std::fs::write(&path, text).expect("a written fixture file");
    }

    fn run(&self, args: &[&str]) -> (String, String, i32) {
        run_in(&self.dir, args)
    }

    /// Make the copy serve the `claude` harness, as a project that adopted the workflow does: the
    /// manifest declares the harness, and the root CLAUDE.md imports the shipped primer. Then
    /// stage it. The mocks declare `harness = []`, because the shipped set changes with every
    /// version of the skills; a test about the installed set declares the harness in its own copy.
    fn serve_claude(&self) {
        let path = self.path("knowledge-architect.toml");
        let manifest = std::fs::read_to_string(&path).expect("the manifest");
        assert_eq!(manifest.matches("harness = []").count(), 1, "{manifest}");
        std::fs::write(
            &path,
            manifest.replace("harness = []", "harness = [\"claude\"]"),
        )
        .expect("the manifest is rewritten");
        let claude = std::fs::read_to_string(self.path("CLAUDE.md")).expect("the root CLAUDE.md");
        self.write(
            "CLAUDE.md",
            &format!("@.claude/knowledge-architect/PRIMER.md\n\n{claude}"),
        );
        self.stage();
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// Copy a mock project, with its manifest pinned to this binary's version.
///
/// A mock says `checker-version = "fixture"`, which the binary confirms only inside a library it
/// links, per `design@check@checked-sentinel-values`, and this crate is none, so a copy carries the
/// version.
fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the sandbox directory");
    for entry in std::fs::read_dir(from).expect("a readable fixture") {
        let entry = entry.expect("a directory entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a file type").is_dir() {
            copy_dir(&entry.path(), &target);
        } else if entry.file_name() == "knowledge-architect.toml" {
            let text = std::fs::read_to_string(entry.path()).expect("a mock manifest");
            std::fs::write(&target, pinned(&text)).expect("a copied manifest");
        } else {
            std::fs::copy(entry.path(), &target).expect("a copied file");
        }
    }
}

/// A manifest's text with `[project] checker-version` set to this binary's version: the key's
/// line replaced where `[project]` holds one, and inserted after the `[project]` line where it
/// does not. A text with no `[project]` table is returned as it is.
fn pinned(text: &str) -> String {
    let key = format!("checker-version = \"{}\"", env!("CARGO_PKG_VERSION"));
    let mut out = Vec::new();
    let mut in_project = false;
    let mut done = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            if in_project && !done {
                out.push(key.clone());
                done = true;
            }
            in_project = trimmed == "[project]";
            out.push(line.to_string());
            if in_project && !done {
                out.push(key.clone());
                done = true;
            }
            continue;
        }
        if in_project && trimmed.starts_with("checker-version") {
            continue;
        }
        out.push(line.to_string());
    }
    let mut joined = out.join("\n");
    if text.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

/// The smallest conformant mock, which declares no table `check` does not own.
fn mock() -> PathBuf {
    project("core")
}

fn check(dir: &Path) -> (i32, String) {
    let out = Command::new(klarch())
        .arg("check")
        .current_dir(dir)
        .output()
        .expect("the knowledge binary runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_string(),
    )
}

/// One git command in `dir`, with the variables that would point it elsewhere removed: a suite run
/// from a commit hook inherits them, and would otherwise write into the outer repository.
fn git(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .current_dir(dir)
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?}");
}

#[test]
fn the_binary_passes_over_a_project_that_declares_only_what_it_reads() {
    let (code, out) = check(&mock());
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("checked: generated, registers, references\n"),
        "{out}"
    );
    assert!(out.ends_with("PASSED: no findings\n"), "{out}");
}

/// `klarch` compiles in the directory of each Component whose library it links and whose
/// source is the checker's, so their fixtures are read as data,
/// per `design@check@checker-source-literals-are-data`. Over this checkout the summary names each
/// directory and counts their walked Rust files, whatever the run finds.
#[test]
fn the_binary_names_the_checker_s_directories_and_counts_the_files_under_them() {
    let checkout = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/testing sits two levels below the root");
    let (code, out) = check(checkout);
    assert_ne!(code, 2, "the check ran: {out}");
    let line = out
        .lines()
        .find(|l| l.starts_with("checker source: "))
        .unwrap_or_else(|| panic!("no checker-source line in {out}"));
    let listing = Command::new("git")
        .args(["ls-files", "--cached", "--others", "--exclude-standard"])
        .current_dir(checkout)
        .output()
        .expect("git lists the checkout");
    let listing = String::from_utf8_lossy(&listing.stdout).into_owned();
    let count = |dir: &str| {
        listing
            .lines()
            .filter(|l| l.starts_with(dir) && l.ends_with(".rs") && !l.contains("/tests/projects/"))
            .count()
    };
    let dirs = [
        "crates/cli",
        "crates/check",
        "crates/git",
        "crates/model",
        "crates/create",
        "crates/query",
        "crates/transcript",
    ];
    let counts = dirs.map(|dir| count(&format!("{dir}/")));
    assert!(counts.iter().all(|n| *n > 0), "{counts:?}");
    assert_eq!(
        line,
        format!(
            "checker source: {}, {} file(s) with string literals read as data",
            dirs.join(", "),
            counts.iter().sum::<usize>()
        )
    );
}

#[test]
fn the_binary_refuses_a_table_no_extension_of_it_claims() {
    // A copy, so the table is added to nothing another test reads. The walk is git's listing,
    // so the copy is its own repository.
    let dir = std::env::temp_dir().join(format!("knowledge-check-claims-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    copy_dir(&mock(), &dir);
    let manifest = dir.join("knowledge-architect.toml");
    let mut text = std::fs::read_to_string(&manifest).expect("the mock manifest");
    text.push_str(
        "\n[rules]\ndir = \"r\"\ntext = \"t\"\nbody-starts-at = 0\nversion = \"v\"\n\
         past = \"p\"\nmanifest = \"m\"\n",
    );
    std::fs::write(&manifest, text).expect("the manifest written");
    git(&dir, &["init", "-q"]);
    git(&dir, &["add", "-A"]);
    let (code, out) = check(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("phase 1: 1 finding(s)"), "{out}");
    assert!(
        out.contains("[rules] is a table no extension of this binary reads"),
        "{out}"
    );
}

const FIRST_ISSUE: &str = "\
---
kind: defect
---
# The first thing outstanding

## Summary

A body, so the entry is an entry.
";

const SECOND_ISSUE: &str = "\
---
kind: todo
---
# The second thing outstanding

## Summary

Another body, and a reference nothing defines: `design@minimal@no-such-decision`.
";

/// PLANTED: one entry declaring `kind` twice, so the block is refused and the row has no kind.
const TWO_KINDS: &str = "\
---
kind: defect
kind: todo
---
# The entry that declares two kinds

## Summary

Two values for one thing, and whichever reader looks first decides.
";

const ONE_TRIPWIRE: &str = "\
# Tripwires

## Guarding the fixture's own decision `##a-fixture-tripwire`

**Fires when:** a condition the fixture names is met, guarding `design@minimal@mock-anchor`.
**Response:** reopen it.
";

/// The `walk: n file(s)` count out of a summary block.
fn walked_count(stdout: &str) -> usize {
    let line = stdout
        .lines()
        .find(|l| l.starts_with("walk: "))
        .unwrap_or_else(|| panic!("the summary block names the walked-file count: {stdout}"));
    line.trim_start_matches("walk: ")
        .split(' ')
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("a number: {line}"))
}

/// A reference to a decision no home defines, for a document to carry.
const DANGLING: &str = "# Another mock document\n\nIt points at `design@minimal@no-such-thing`.\n";

// ---------------------------------------------------------------------------------------
// Commit messages: `commits`
// ---------------------------------------------------------------------------------------

/// A throwaway repository holding a project the checks find nothing wrong with.
///
/// **The mock projects under `path@testing@tests/projects/` are not used here.** A commit whose
/// tree fails fails the range, and `dirhome` is the mock over which every family
/// runs and finds nothing — so a copy of it could serve as a base. The project below is written out anyway, because each test
/// below states the exact findings its commits carry, and a mock's contents are shared with
/// every other test over it: a document added here for one commit's sake would move another
/// test's counts.
struct History {
    /// The project's own directory, which is where every command is run.
    dir: PathBuf,
    /// The repository's root, which is `dir` unless the project is vendored under it.
    repo: PathBuf,
}

impl History {
    fn new(tag: &str) -> Self {
        Self::at(tag, "")
    }

    /// The same, with the project `under` a subdirectory of its repository.
    ///
    /// A project vendored that way is what the manifest's `find` already supports, and every
    /// git question the per-commit read asks has to be answered project-relative rather than
    /// repository-relative.
    fn at(tag: &str, under: &str) -> Self {
        let repo = std::env::temp_dir().join(format!("knowledge-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        let dir = if under.is_empty() {
            repo.clone()
        } else {
            repo.join(under)
        };
        std::fs::create_dir_all(&dir).expect("a temporary directory");
        let history = History { dir, repo };
        history.git(&["init", "-q"]);
        // An identity in the repository's own configuration, not the machine's: a fixture
        // that read the developer's would fail wherever none is set, which is every runner.
        history.git(&["config", "user.name", "fixture"]);
        history.git(&["config", "user.email", "fixture@example.invalid"]);
        history
    }

    /// One git command at the REPOSITORY root, which is where `init`, `add` and `commit`
    /// belong: `add -A` run inside a subdirectory stages that subdirectory alone.
    fn git(&self, args: &[&str]) {
        let out = Command::new("git")
            .args(args)
            .current_dir(&self.repo)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Write a file into the project; a manifest is written pinned to this binary's version.
    fn write(&self, rel: &str, text: &str) {
        if rel == "knowledge-architect.toml" {
            self.write_bytes(rel, pinned(text).as_bytes());
        } else {
            self.write_bytes(rel, text.as_bytes());
        }
    }

    /// The same, for bytes that are not text: a blob the walk reads as a document.
    fn write_bytes(&self, rel: &str, bytes: &[u8]) {
        let path = self.dir.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
        std::fs::write(&path, bytes).expect("a written fixture file");
    }

    fn remove(&self, rel: &str) {
        std::fs::remove_file(self.dir.join(rel)).expect("a fixture file to delete");
    }

    fn run(&self, args: &[&str]) -> (String, String, i32) {
        run_in(&self.dir, args)
    }

    /// Regenerate the indexes, stage everything, and commit with this message.
    ///
    /// The indexes are regenerated first because the `generated` family compares bytes: a
    /// commit that adds or deletes a register entry and leaves the index alone has a tree
    /// that fails, and every such commit would fail the range for its tree rather than for
    /// what the test planted.
    fn commit(&self, message: &str) -> String {
        // **Staged, then regenerated, then staged again.** The walk is git's listing, so a
        // deleted file the index still holds is walked and its register entry still counted:
        // regenerating before the deletion is staged writes the listing the tree no longer
        // has. A tree whose manifest this tool refuses to load regenerates no index, and neither
        // does a tree that stops at phase 2, such as one holding a symlink; tests commit both
        // shapes on purpose, so the run's own code is not asserted.
        self.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
        self.run(&["index"]);
        self.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
        let file = self.repo.join("message.txt");
        std::fs::write(&file, message).expect("a message file");
        let path = file.to_string_lossy().into_owned();
        // `--allow-empty`, because a test commits a message over a tree it did not change:
        // the subject here is the message, and a tree edit per commit would be noise the
        // reader has to discount.
        self.git(&[
            "commit",
            "-q",
            "--allow-empty",
            "--cleanup=verbatim",
            "-F",
            &path,
        ]);
        std::fs::remove_file(&file).expect("the message file leaves the tree");
        let out = Command::new("git")
            .args(["rev-parse", "--short=7", "HEAD"])
            .current_dir(&self.repo)
            .output()
            .expect("git runs");
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }
}

impl Drop for History {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.repo);
    }
}

/// A plan document holding every section a spec or a milestone's README owes, each empty but
/// those `sections` fills, as `(title, body)` pairs.
fn plan_document(title: &str, intro: &str, sections: &[(&str, &str)]) -> String {
    const OWED: [&str; 20] = [
        "Status and audience",
        "How the work is done",
        "Names",
        "What the work is",
        "What is already decided",
        "Criteria",
        "Threads",
        "Arguments",
        "New names, in one place",
        "Decided design",
        "Mapping tables",
        "Losing alternatives",
        "Readings",
        "Premortem",
        "Acceptance criteria",
        "Implementation sequence",
        "Order rationale",
        "Defaults awaiting the owner",
        "Harvest",
        "Later consequences",
    ];
    let mut out = format!("# {title}\n\n{intro}\n");
    for owed in OWED {
        let body = sections
            .iter()
            .find(|(t, _)| *t == owed)
            .map(|(_, b)| *b)
            .unwrap_or("None.");
        out.push_str(&format!("\n## {owed}\n\n{body}\n"));
    }
    out
}

/// A slice spec holding every section a slice owes, each empty.
fn slice_document(title: &str) -> String {
    let mut out = format!("# {title}\n");
    for owed in [
        "Builds",
        "Claims",
        "Audit subjects",
        "Fails alone on",
        "Premises that expire",
    ] {
        out.push_str(&format!("\n## {owed}\n\nNone.\n"));
    }
    out
}

/// Everything a project owes, written out, with one design slug and one issue entry.
///
/// `unloadable` writes a `[project]` key the grammar does not know, so the manifest does not
/// load and the commit fails the range.
fn tiny_project(history: &History, unloadable: bool) {
    let unknown = if unloadable { "no-such-key = []\n" } else { "" };
    history.write(
        "knowledge-architect.toml",
        &format!(
            "[project]\nname = \"tiny\"\ncomponents = []\n{unknown}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n\n\
             [agents]\nharness = []\n"
        ),
    );
    history.write(
        "README.md",
        "# tiny\n\nA project a test builds so a commit has a tree with nothing wrong in it.\n",
    );
    history.write(
        "CLAUDE.md",
        "# tiny\n\nNothing here holds of any code: this project has none.\n",
    );
    history.write(
        "docs/design.md",
        "# tiny — design\n\n### A decision this project records `##tiny-anchor`\n\n\
         It exists so that a message has something to name.\n",
    );
    history.write(
        "docs/goals.md",
        "# Goals — tiny\n\n## Be committed, so a message has a tree `##tiny-goal`\n\n\
         The one goal of this project.\n",
    );
    history.write(
        "docs/tripwires.md",
        "# Tripwires — tiny\n\n## Guarding `design@tiny@tiny-anchor` `##tiny-tripwire`\n\n\
         **Fires when:** the project stops being committed.\n**Response:** reopen it.\n",
    );
    history.write(
        "docs/rejected.md",
        "# tiny — rejected designs\n\nNothing has lost yet.\n",
    );
    history.write(
        "docs/open-issues/README.md",
        "# Open issues — tiny\n\nOne file per entry; the listing beside this file is generated.\n",
    );
    history.write(
        "docs/open-issues/a-closable-issue.md",
        "---\nkind: observation\n---\n# A closable issue\n\n## Summary\n\n\
         An entry a commit can delete.\n\n## Details\n\n### What\n\n\
         This entry exists to be closed.\n\n### Why it matters\n\n\
         A message naming a deleted entry resolves against the parent tree or against nothing.\n\n\
         ### What would close it\n\nThe commit that deletes this file.\n",
    );
    // The plans layout every project carries, holding no plan document; `commit` generates
    // the two indexes.
    history.write(
        "docs/plans/README.md",
        "# Plans — tiny\n\nNo plan document is open.\n",
    );
    history.write(
        "docs/plans/specs/README.md",
        "# Specs — tiny\n\nOne file per spec; the listing beside this file is generated.\n",
    );
    history.write(
        "docs/plans/milestones/README.md",
        "# Milestones — tiny\n\nOne directory per milestone; the listing beside this file is \
         generated.\n",
    );
}

/// The claim: a definition-site defect alone stops the run at phase 3, and the references
/// that would dangle because of it are not judged.
#[test]
fn a_definition_site_defect_stops_the_run_at_phase_three() {
    let sandbox = Sandbox::seeded(
        "phase-three",
        "dirhome",
        &[(
            "docs/design/misplaced.md",
            "# Misplaced\n\n#### Too deep `##too-deep`\n\nIt points at `design@dirhome@too-deep`.\n",
        )],
    );
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("phase 3:") && stdout.contains("phase 4 was not judged"),
        "{stdout}"
    );
    assert!(
        stdout.contains("`##too-deep` is written at a level-4 heading"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("is referenced") && !stdout.contains("not linked from"),
        "no phase-4 finding: {stdout}"
    );
}

/// The claim: a symlink at a walked name is read as no document and is a phase-2 finding,
/// `check` and `commits` agree on it by the mode git records, and a `skip-files` row keeps it.
#[test]
fn a_symlink_is_a_phase_two_finding_in_both_readers_and_a_skip_row_keeps_it() {
    let sandbox = Sandbox::new("symlink", "minimal");
    std::os::unix::fs::symlink("a.md", sandbox.path("notes/link.md")).expect("a symlink");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("phase 2: 1 finding(s)") && stdout.contains("`notes/link.md` is a symlink"),
        "{stdout}"
    );
    // Untracked, the same: the kind is read off the working tree where the index has none.
    let untracked = Sandbox::new("symlink-untracked", "minimal");
    std::os::unix::fs::symlink("a.md", untracked.path("notes/link.md")).expect("a symlink");
    let (stdout, _, code) = untracked.run(&["check"]);
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.contains("`notes/link.md` is a symlink"), "{stdout}");

    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            "skip-files = [\"notes/link.md\", \"notes/generated.md\"]",
        ),
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(!stdout.contains("link.md"), "{stdout}");
}

/// The claim: a submodule's gitlink is a phase-2 finding naming it, and an `exclude` row is
/// the declared silence. The entry is written straight into the index, since a submodule
/// needs no checkout to be one.
#[test]
fn a_gitlink_is_a_phase_two_finding_and_an_exclude_row_declares_the_silence() {
    let history = History::new("gitlink");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // A gitlink names a commit, which need not be reachable; the empty directory is what an
    // uninitialised submodule looks like, and what keeps the staging step from removing it.
    std::fs::create_dir_all(history.dir.join("vendor/sub")).expect("the submodule's place");
    history.git(&[
        "update-index",
        "--add",
        "--cacheinfo",
        "160000,e69de29bb2d1d6434b8b29ae775ad8c2e48c5391,vendor/sub",
    ]);
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("phase 2: 1 finding(s)") && stdout.contains("`vendor/sub` is a submodule"),
        "{stdout}"
    );
    let sha = history.commit("A submodule is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s)"
        )),
        "the two readers agree: {stdout}"
    );

    let manifest = std::fs::read_to_string(history.dir.join("knowledge-architect.toml"))
        .expect("the manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest.replace("exclude = []", "exclude = [\"vendor/sub\"]"),
    );
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

/// The claim: a committed symlink at a walked name is one phase-2 finding in `commits` too,
/// read off the tree's mode, and is walked as no document there either.
#[test]
fn a_committed_symlink_is_one_phase_two_finding_in_commits() {
    let history = History::new("commit-symlink");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    std::os::unix::fs::symlink("design.md", history.dir.join("docs/link.md")).expect("a symlink");
    let sha = history.commit("A symlink is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s)"
        )),
        "one finding, the symlink, and not a second for a document nothing could read: {stdout}"
    );
}

#[test]
fn check_takes_no_selection() {
    // Every check runs on every run, in the phases; a selection would cut across them.
    let (stdout, stderr, code) = run("planted", &["check", "--only", "references"]);
    assert!(stderr.contains("--only"), "{stderr}");
    assert!(stdout.is_empty(), "nothing was checked: {stdout}");
    assert_eq!(code, 2, "{stderr}");
}

#[test]
fn no_subcommand_is_not_success() {
    // A tool whose default is to exit 0 having done nothing is the silent false negative
    // every check here exists to prevent.
    let (_, stderr, code) = run("planted", &[]);
    assert_eq!(code, 2, "{stderr}");
}

/// The verdict counts what was printed, and the count is the exit code's own predicate.
///
/// A count tracked beside the findings could disagree with the list and with the exit code.
/// This reads both off the same run and compares them, so a second source of truth for
/// "did it fail" cannot be introduced without failing here.
#[test]
fn the_verdict_counts_the_findings_it_printed() {
    let (stdout, _, code) = run("planted", &["check"]);
    let last = stdout.lines().rfind(|l| !l.is_empty()).unwrap();
    let claimed: usize = last
        .trim_start_matches("FAILED: ")
        .trim_end_matches(" findings above")
        .trim_end_matches(" finding above")
        .parse()
        .unwrap_or_else(|_| panic!("the verdict names a count: {last:?}"));

    // A finding is two lines: the finding, then its indented `→` action. Counting the actions
    // counts the findings without parsing the finding lines themselves.
    let printed = stdout
        .lines()
        .filter(|l| l.trim_start().starts_with('→'))
        .count();
    assert_eq!(
        claimed, printed,
        "the verdict counts what was printed: {stdout}"
    );
    assert!(
        claimed > 0 && code == 1,
        "a nonzero count means failure: {stdout}"
    );
}

/// The claim: `issues` prints one row per issue entry with the five columns in order, and each
/// filter keeps only what it names.
///
/// Recorded mutation, `cargo mutate run` over `path@check@src/commands/mod.rs`: turning the `--kind`
/// filter's `is_none_or` into `is_some_and` empties every unfiltered listing, and turning the
/// row's `metadata` cell into a constant makes every kind read alike. Both are caught here.
#[test]
fn issues_lists_one_row_per_entry_and_each_filter_keeps_what_it_names() {
    let sandbox = Sandbox::new("issues", "minimal");
    // The fixture's own entries are replaced, so the rows below are this test's.
    std::fs::remove_file(sandbox.path("docs/open-issues/the-mock-has-one-issue.md"))
        .expect("the fixture's own entry");
    std::fs::remove_file(sandbox.path("notes/open-issues/the-notes-are-not-a-component.md"))
        .expect("the fixture's own entry");
    sandbox.write("docs/open-issues/the-first-thing.md", FIRST_ISSUE);
    sandbox.write("docs/open-issues/the-second-thing.md", SECOND_ISSUE);
    sandbox.write("notes/open-issues/two-kinds.md", TWO_KINDS);
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);
    sandbox.stage();

    let (all, stderr, code) = sandbox.run(&["issues"]);
    assert_eq!(code, 0, "{stderr}");
    let rows: Vec<&str> = all.lines().collect();
    assert_eq!(
        rows[0].split_whitespace().collect::<Vec<_>>(),
        vec!["kind", "anchor", "id", "title", "last", "change"],
        "{all}"
    );
    // Sorted by kind then id, so `-` (the refused block) comes before `defect` before `todo`.
    assert_eq!(rows.len(), 4, "{all}");
    assert!(rows[1].starts_with("-  "), "{all}");
    assert!(rows[1].contains("two-kinds"), "{all}");
    assert!(rows[2].starts_with("defect"), "{all}");
    assert!(rows[3].starts_with("todo"), "{all}");
    // The anchor column tells the two instances apart.
    assert!(rows[1].contains("notes"), "{all}");
    assert!(rows[2].contains("minimal"), "{all}");
    // No tripwire is an issue.
    assert!(!all.contains("Guarding the fixture"), "{all}");

    let (one_kind, _, code) = sandbox.run(&["issues", "--kind", "todo"]);
    assert_eq!(code, 0, "{one_kind}");
    assert_eq!(one_kind.lines().count(), 2, "{one_kind}");
    assert!(one_kind.contains("the-second-thing"), "{one_kind}");

    let (one_anchor, _, code) = sandbox.run(&["issues", "notes"]);
    assert_eq!(code, 0, "{one_anchor}");
    assert_eq!(one_anchor.lines().count(), 2, "{one_anchor}");
    assert!(one_anchor.contains("two-kinds"), "{one_anchor}");

    let (searched, _, code) = sandbox.run(&["issues", "second"]);
    assert_eq!(code, 0, "{searched}");
    assert_eq!(searched.lines().count(), 2, "{searched}");
    assert!(searched.contains("the-second-thing"), "{searched}");

    // No row is a negative answer, and the header still says what was looked for.
    let (none, _, code) = sandbox.run(&["issues", "nothing-matches-this"]);
    assert_eq!(code, 1, "{none}");
    assert!(
        none.contains("kind") && none.contains("(no entry)"),
        "{none}"
    );
}

/// `anchors` prints every anchor a project has, of every kind, each register a declared anchor
/// carries with its home, and exits 0 when every home it prints exists. `planted` is clean
/// through phase 2 and holds every kind: two Components, a location, the plans directory, a
/// milestone and a spec.
#[test]
fn anchors_lists_every_anchor_kind_with_its_homes() {
    let (out, stderr, code) = run("planted", &["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    for (name, kind) in [
        ("planted", "component"),
        ("widget", "component"),
        ("agent-config", "location"),
        ("plans", "plans"),
        ("m-one", "milestone"),
        ("a-spec", "spec"),
    ] {
        assert!(
            out.lines()
                .any(|l| l.split_whitespace().take(2).eq([name, kind])),
            "no line for the {kind} `{name}`:\n{out}"
        );
    }
    // A register line names its home as the tree holds it.
    assert!(
        out.lines().any(|l| l
            .split_whitespace()
            .eq(["issue", "agent-config/open-issues/"])),
        "{out}"
    );
    assert!(!out.contains("missing"), "{out}");
    // The root Component's path is `.`; `plans` lists its two homes; a milestone and a spec
    // list no register, since their items live in the plan's own documents.
    assert!(out.lines().any(|l| l == "planted  component  ."), "{out}");
    let after = |head: &str| {
        out.split('\n')
            .skip_while(|l| !l.starts_with(head))
            .skip(1)
            .take_while(|l| l.starts_with("  "))
            .map(str::trim)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        after("plans "),
        [
            "spec  docs/plans/specs/",
            "milestone  docs/plans/milestones/"
        ],
        "{out}"
    );
    assert!(after("m-one ").is_empty(), "{out}");
    assert!(after("a-spec ").is_empty(), "{out}");
}

/// A heading register's home in its directory shape is printed as the directory, and a
/// conformant project exits 0. `dirhome` keeps its design and goals homes as directories.
#[test]
fn anchors_prints_a_heading_register_s_directory_home() {
    let (out, stderr, code) = run("dirhome", &["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    for (register, home) in [("design", "docs/design/"), ("goal", "docs/goals/")] {
        assert!(
            out.lines()
                .any(|l| l.split_whitespace().eq([register, home])),
            "{out}"
        );
    }
}

/// A heading register's directory with no README is still its home, as `check` judges it: the
/// absent README is a finding of its own, not a missing home.
#[test]
fn anchors_prints_a_directory_home_without_its_readme() {
    let sandbox = Sandbox::new("anchors-no-readme", "dirhome");
    std::fs::remove_file(sandbox.path("docs/design/README.md")).expect("the README is there");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l.split_whitespace().eq(["design", "docs/design/"])),
        "{out}"
    );
}

/// A file register's directory is its home when it holds an entry and nothing else, the entry in
/// a group: the directory is what `check` judges, not its README, its index, its register.toml or
/// a top-level file, and the absent README is a finding of its own.
#[test]
fn anchors_prints_a_file_register_s_home_holding_only_a_grouped_entry() {
    let sandbox = Sandbox::new("anchors-file-grouped-only", "dirhome");
    for file in [
        "README.md",
        "index.md",
        "register.toml",
        "an-ungrouped-entry.md",
    ] {
        std::fs::remove_file(sandbox.path(&format!("docs/open-issues/{file}")))
            .expect("the file is there");
    }
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l.split_whitespace().eq(["issue", "docs/open-issues/"])),
        "{out}"
    );
}

/// A Directory register's directory with no README and no index is still its home, as for a
/// file register's.
#[test]
fn anchors_prints_a_directory_register_s_home_without_its_readme() {
    let sandbox = Sandbox::new("anchors-milestones-no-readme", "dirhome");
    for file in ["README.md", "index.md"] {
        std::fs::remove_file(sandbox.path(&format!("docs/plans/milestones/{file}")))
            .expect("the file is there");
    }
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines().any(|l| l
            .split_whitespace()
            .eq(["milestone", "docs/plans/milestones/"])),
        "{out}"
    );
}

/// A home directory that holds its README alone is held: the README is a file under it.
#[test]
fn anchors_prints_a_home_holding_only_its_readme() {
    let sandbox = Sandbox::new("anchors-readme-only", "dirhome");
    for file in [
        "index.md",
        "register.toml",
        "an-ungrouped-entry.md",
        "housekeeping/a-grouped-entry.md",
    ] {
        std::fs::remove_file(sandbox.path(&format!("docs/open-issues/{file}")))
            .expect("the file is there");
    }
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l.split_whitespace().eq(["issue", "docs/open-issues/"])),
        "{out}"
    );
}

/// A sibling directory whose name only begins with the home's name is not the home: the issue
/// directory renamed with a suffix leaves the home missing.
#[test]
fn anchors_does_not_take_a_sibling_with_the_home_s_prefix_for_the_home() {
    let sandbox = Sandbox::new("anchors-prefix-sibling", "dirhome");
    std::fs::rename(
        sandbox.path("docs/open-issues"),
        sandbox.path("docs/open-issues-old"),
    )
    .expect("the directory is there");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l.split_whitespace().eq(["issue", "missing"])),
        "{out}"
    );
}

/// A heading register held in both shapes prints both, as `check` reports the pair.
#[test]
fn anchors_prints_both_shapes_of_a_heading_home() {
    let sandbox = Sandbox::new("anchors-both", "dirhome");
    sandbox.write("docs/design.md", "# Design\n");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines().any(|l| l
            .split_whitespace()
            .eq(["design", "docs/design.md", "docs/design/"])),
        "{out}"
    );
}

/// A home the tree does not hold is printed `missing`, and the run is a negative answer, exit
/// 1, per `design@cli@exit-code-ladder`. `unsound` plants a location whose issue directory is
/// not there.
#[test]
fn anchors_marks_a_missing_home_and_exits_one() {
    let (out, stderr, code) = run("unsound", &["anchors"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let location = out
        .split("\n")
        .skip_while(|l| !l.starts_with("agent-config "))
        .nth(1)
        .unwrap_or_default();
    assert_eq!(
        location.split_whitespace().collect::<Vec<_>>(),
        ["issue", "missing"],
        "{out}"
    );
    // A heading register's missing home: the widget's tripwire home is not there.
    assert!(
        out.lines()
            .any(|l| l.split_whitespace().eq(["tripwire", "missing"])),
        "{out}"
    );
}

/// The claim: `--group` keeps the rows whose entry sits in the group it names, and drops the
/// ungrouped ones.
///
/// The only grouped ISSUE instance in any mock is `dirhome`'s, and it is read in the shared
/// copy: the command writes nothing. Both directions are asserted, because a filter that
/// dropped every row would satisfy a test that only checked the grouped entry was gone.
#[test]
fn issues_group_filter_keeps_the_entries_of_the_group_it_names() {
    let (all, stderr, code) = run("dirhome", &["issues"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(all.contains("a-grouped-entry"), "{all}");
    assert!(all.contains("an-ungrouped-entry"), "{all}");

    let (grouped, _, code) = run("dirhome", &["issues", "--group", "housekeeping"]);
    assert_eq!(code, 0, "{grouped}");
    assert!(grouped.contains("a-grouped-entry"), "{grouped}");
    assert!(!grouped.contains("an-ungrouped-entry"), "{grouped}");

    // A group the instance does not declare keeps nothing, and no row is exit 1.
    let (none, _, code) = run("dirhome", &["issues", "--group", "no-such-group"]);
    assert_eq!(code, 1, "{none}");
    assert!(!none.contains("a-grouped-entry"), "{none}");
}

/// The claim: `tripwires` prints the references each entry carries, `--guarding` keeps the rows
/// carrying one, and no issue appears among them.
#[test]
fn tripwires_lists_the_decisions_each_entry_guards() {
    let sandbox = Sandbox::new("tripwires", "minimal");
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);

    let (all, stderr, code) = sandbox.run(&["tripwires"]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(
        all.lines()
            .next()
            .map(|l| l.split_whitespace().collect::<Vec<_>>()),
        Some(vec!["anchor", "id", "title", "guarding"]),
        "{all}"
    );
    let row = all.lines().nth(1).expect("one tripwire");
    // The title loses the slug that defines the entry, and the guarded decision is a column.
    assert!(row.contains("a-fixture-tripwire"), "{row}");
    assert!(row.contains("Guarding the fixture's own decision"), "{row}");
    assert!(!row.contains("##a-fixture-tripwire"), "{row}");
    assert!(
        row.trim_end().ends_with("design@minimal@mock-anchor"),
        "{row}"
    );
    assert!(!all.contains("The mock has one issue"), "{all}");

    let (kept, _, code) = sandbox.run(&["tripwires", "--guarding", "design@minimal@mock-anchor"]);
    assert_eq!(code, 0, "{kept}");
    assert_eq!(kept.lines().count(), 2, "{kept}");

    let (dropped, _, code) =
        sandbox.run(&["tripwires", "--guarding", "design@minimal@mock-anchor-2"]);
    assert_eq!(code, 1, "{dropped}");
    assert!(dropped.contains("(no entry)"), "{dropped}");
}

/// The claim: `show` prints a file entry whole and a heading entry's section, lists every inbound
/// reference as `file:line`, exits 1 on a reference that resolves to nothing and 2 on an argument
/// that is not reference-shaped.
#[test]
fn show_prints_the_entry_and_what_points_at_it() {
    let sandbox = Sandbox::new("show", "minimal");
    sandbox.write("docs/open-issues/the-second-thing.md", SECOND_ISSUE);
    sandbox.write("docs/tripwires.md", ONE_TRIPWIRE);

    // A file entry: the file whole, frontmatter included.
    let (entry, stderr, code) = sandbox.run(&["show", "issue@minimal@the-second-thing"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(entry.contains("kind: todo"), "{entry}");
    assert!(entry.contains("## Summary"), "{entry}");
    assert!(
        entry.contains("docs/open-issues/the-second-thing.md:1"),
        "{entry}"
    );

    // A heading entry: that heading's section and not the whole file.
    let (heading, _, code) = sandbox.run(&["show", "tripwire@minimal@a-fixture-tripwire"]);
    assert_eq!(code, 0, "{heading}");
    assert!(heading.contains("**Fires when:**"), "{heading}");
    assert!(
        !heading.contains("# Tripwires\n"),
        "the level-one head is another section: {heading}"
    );

    // The inbound half: the entry above names a decision, and that decision's `show` finds it.
    let (inbound, _, code) = sandbox.run(&["show", "design@minimal@mock-anchor"]);
    assert_eq!(code, 0, "{inbound}");
    assert!(inbound.contains("referenced at:"), "{inbound}");
    assert!(inbound.contains("docs/tripwires.md:"), "{inbound}");

    // An entry nothing points at says so, rather than printing an empty list.
    let (alone, _, code) = sandbox.run(&["show", "issue@minimal@the-second-thing"]);
    assert_eq!(code, 0, "{alone}");
    assert!(alone.contains("referenced by nothing"), "{alone}");

    // The two failures are different questions and different codes.
    let (gone, _, code) = sandbox.run(&["show", "issue@minimal@no-such-entry"]);
    assert_eq!(code, 1, "{gone}");
    assert!(gone.contains("resolves to nothing"), "{gone}");
    for shape in ["not-a-reference", "design@minimal", "design@minimal@a@b"] {
        let (_, err, code) = sandbox.run(&["show", shape]);
        assert_eq!(code, 2, "{shape}: {err}");
    }
}

/// The claim, per `design@model@register-home-reference`: `show` on a `register` reference prints
/// the home the tree holds, a file for a heading home in one file and a directory for one split
/// into a directory, with the register's shape, then every site that cites it; one the check
/// refuses resolves to nothing with the check's finding. Mutation checked, and caught: the
/// `listed(&file)` branch disabled.
#[test]
fn show_of_a_register_reference_prints_the_home_the_tree_holds() {
    let sandbox = Sandbox::new("show-register", "dirhome");
    sandbox.write(
        "docs/rejected.md",
        "# Rejected alternatives\n\nNone yet; the tripwires sit in `register@dirhome@tripwire`.\n\n\
         Every component's sit in `register@*@tripwire`.\n",
    );

    let (file, stderr, code) = sandbox.run(&["show", "register@dirhome@tripwire"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(file.contains("dirhome: docs/tripwires.md"), "{file}");
    assert!(file.contains("Heading"), "{file}");
    assert!(file.contains("docs/rejected.md:3"), "{file}");
    // A citation through `*` names this component's home too.
    assert!(file.contains("docs/rejected.md:5"), "{file}");
    // One line per home of the anchor named, and under `*` one per component: never a
    // location's or a plan anchor's.
    assert!(!file.contains("plans:"), "{file}");
    let (every, stderr, code) = sandbox.run(&["show", "register@*@design"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(every.contains("dirhome: docs/design/"), "{every}");
    assert!(!every.contains("plans:"), "{every}");

    let (dir, stderr, code) = sandbox.run(&["show", "register@dirhome@design"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(dir.contains("dirhome: docs/design/"), "{dir}");
    assert!(dir.contains("referenced by nothing"), "{dir}");

    let (refused, _, code) = sandbox.run(&["show", "register@dirhome@spec"]);
    assert_eq!(code, 1, "{refused}");
    assert!(refused.contains("resolves to nothing"), "{refused}");
    assert!(refused.contains("carries no spec register"), "{refused}");
}

/// The claim, per `design@model@planned-path-form`: `show` on a planned reference resolves whether
/// or not the target exists, says which, and lists the plans that cite it, which are the sites a
/// conversion edits.
#[test]
fn show_of_a_planned_path_lists_its_citations_and_says_whether_it_exists() {
    let sandbox = Sandbox::new("show-planned", "minimal");
    sandbox.write(
        "docs/plans/README.md",
        "# Plan documents\n\nThe mock's plans directory: specs under specs/, milestones under \
         milestones/.\n\nIt creates `planned@minimal@code/new.rs` beside \
         `planned@minimal@code/lib.rs`.\n\nThe same target, slashed: `planned@minimal@code/new.rs/`.\n",
    );
    let (absent, stderr, code) = sandbox.run(&["show", "planned@minimal@code/new.rs"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(absent.contains("does not hold the target"), "{absent}");
    assert!(absent.contains("docs/plans/README.md:5"), "{absent}");
    // The slash is a kind claim, not another target: both citations are listed.
    assert!(absent.contains("docs/plans/README.md:7"), "{absent}");

    // The path the planned form converts to lists the planned citations: the sites to edit.
    let (path, _, code) = sandbox.run(&["show", "path@minimal@code/lib.rs"]);
    assert_eq!(code, 0, "{path}");
    assert!(path.contains("docs/plans/README.md:5"), "{path}");

    // A reference the check refuses resolves to nothing here too.
    let (refused, _, code) = sandbox.run(&["show", "planned@minimal@../x.rs"]);
    assert_eq!(code, 1, "{refused}");
    assert!(refused.contains("resolves to nothing"), "{refused}");

    let (present, stderr, code) = sandbox.run(&["show", "planned@minimal@code/lib.rs"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(present.contains("now exists"), "{present}");
    assert!(
        !present.contains("A doc comment pointing"),
        "no body is printed: {present}"
    );
    assert!(present.contains("docs/plans/README.md:5"), "{present}");
}

/// The claim: a tracked document a `.gitignore` line covers is still walked, and the pair is a
/// finding naming the file.
///
/// The recipe is the one the tracker entry this closed carried, and the entry is gone with the
/// defect. Under the hand-rolled matcher the second run printed nothing:
/// the ignore line pruned the document from the walk AND from the inverse assertion, so the
/// dangling reference in it was read by no check and the run exited 0. Git's tracked listing is
/// unaffected by the ignore rules, so the reference is still found, and the contradiction
/// between the index and the ignore rules is reported instead of being silent.
#[test]
fn a_tracked_document_an_ignore_line_covers_is_walked_and_reported() {
    let sandbox = Sandbox::new("tracked-ignored", "minimal");
    sandbox.write("notes/b.md", DANGLING);
    sandbox.stage();

    let (before, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(
        code, 1,
        "the dangling reference is a finding: {before}{stderr}"
    );
    assert!(before.contains("no-such-thing"), "{before}");

    // The recipe: one root gitignore line holding the document's bare filename. The document
    // is already tracked, so `git add -A` leaves it tracked.
    sandbox.write(".gitignore", "b.md\n");
    sandbox.stage();
    let (after, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{after}{stderr}");
    // The contradiction is a phase-2 fact, so the run stops there and the dangling reference
    // is not judged; that the document is still walked is what the walk count says.
    assert_eq!(
        walked_count(&after),
        walked_count(&before),
        "the document must still be walked: {after}"
    );
    assert!(after.contains("phase 2:"), "{after}");
    assert!(
        !after.contains("no-such-thing"),
        "nothing later is judged over an incomplete model: {after}"
    );
    assert!(
        after.contains("notes/b.md") && after.contains("git tracks this file"),
        "the tracked-and-ignored pair must be named: {after}"
    );
    // The summary says how much was walked, on a failing run as readily as a passing one.
    assert!(walked_count(&after) > 0, "{after}");
}

/// The claim: a file outside the walk that cannot be read stops the run at phase 2, naming it.
///
/// The read is made to fail by a loop, not by a mode: the tracked file is replaced on disk by a
/// symlink to itself, which every user fails to open, root included, where root reads through a
/// mode of 000. The index still records a regular file, so the listing does not report a
/// symlink and the file reaches the read.
#[test]
fn an_unreadable_file_outside_the_walk_is_a_phase_two_finding_naming_it() {
    let sandbox = Sandbox::seeded("unreadable-outside", "minimal", &[("notes/old.txt", "x\n")]);
    let path = sandbox.path("notes/old.txt");
    std::fs::remove_file(&path).expect("the file");
    std::os::unix::fs::symlink("old.txt", &path).expect("a symlink to itself");
    assert!(
        std::fs::read(&path).is_err(),
        "the loop must fail every read, or this test asserts nothing"
    );
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("phase 2:"), "{out}");
    let about: Vec<&str> = out
        .lines()
        .filter(|l| l.contains("notes/old.txt"))
        .collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(about[0].contains("could not be read"), "{out}");

    // Readable again, it is an ordinary file outside the walk, and the run passes.
    std::fs::remove_file(&path).expect("the symlink");
    std::fs::write(&path, "x\n").expect("the file");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// The claim: a file outside the walk that git lists and the disk does not hold, and an untracked
/// nested repository, are each a phase-2 finding with a statement of its own, not the one for a
/// file that could not be read.
#[test]
fn a_missing_file_and_a_nested_repository_outside_the_walk_are_named_for_what_they_are() {
    let sandbox = Sandbox::seeded("missing-outside", "minimal", &[("notes/old.txt", "x\n")]);
    std::fs::remove_file(sandbox.path("notes/old.txt")).expect("the file");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out
        .lines()
        .filter(|l| l.contains("notes/old.txt"))
        .collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(
        about[0].contains("the working tree does not hold it"),
        "{out}"
    );
    assert!(out.contains("stage the deletion"), "{out}");

    let nested = Sandbox::new("nested-outside", "minimal");
    nested.write("sub/a.txt", "x\n");
    git(&nested.path("sub"), &["init", "-q"]);
    let (out, stderr, code) = nested.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out.lines().filter(|l| l.starts_with("sub")).collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(
        about[0].contains("a repository nested in this one"),
        "{out}"
    );
}

/// The claim: a symlink whose name the walk refuses is one finding, the link's, and not a second
/// for its name.
#[test]
fn a_symlink_with_a_refused_name_is_one_finding() {
    let sandbox = Sandbox::new("symlink-refused", "minimal");
    std::os::unix::fs::symlink("README.md", sandbox.path("a:b")).expect("a symlink");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out.lines().filter(|l| l.starts_with("a:b")).collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(about[0].contains("symlink"), "{out}");
}

/// The claim: a tree holding `check`'s own crate at another path than the binary was built
/// from is a second checkout of the tool, and no command runs over it: the binary would judge it
/// with the other checkout's code.
///
/// The copy stands for the second checkout: a crate declaring `check`'s package name at the
/// path relative to the tree `check` has in this repository. The reverse arrangement, two
/// checkouts building into one target directory, is what produces this state in practice.
#[test]
fn a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused() {
    let sandbox = Sandbox::seeded(
        "foreign-build",
        "minimal",
        &[(
            "crates/cli/Cargo.toml",
            "[package]\nname = \"knowledge-architect\"\n",
        )],
    );
    for command in [&["check"][..], &["issues"][..]] {
        let (out, stderr, code) = sandbox.run(command);
        assert_eq!(code, 2, "{command:?}: {out}{stderr}");
        assert!(stderr.contains("built from another checkout"), "{stderr}");
        assert!(
            // The profile of the clean is the binary's, which is this test's.
            stderr.contains(&format!(
                "cargo clean{} -p knowledge-architect -p knowledge-architect-check -p knowledge-architect-git -p knowledge-architect-model -p knowledge-architect-create -p knowledge-architect-query -p knowledge-architect-transcript -p knowledge-architect-agent-skills`",
                if cfg!(debug_assertions) {
                    ""
                } else {
                    " --release"
                }
            )),
            "{stderr}"
        );
        assert!(out.is_empty(), "nothing ran: {out}");
    }
}

/// The claim: the refusal covers every library the binary links, the git crate's included, so a
/// tree holding the git crate at another path than the binary was built from is refused too.
#[test]
fn a_tree_holding_the_git_crate_at_another_path_than_the_binarys_is_refused() {
    let sandbox = Sandbox::seeded(
        "foreign-git",
        "minimal",
        &[(
            "crates/git/Cargo.toml",
            "[package]\nname = \"knowledge-architect-git\"\n",
        )],
    );
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains("its `knowledge-architect-git` was compiled at"),
        "{stderr}"
    );
    assert!(out.is_empty(), "nothing ran: {out}");
}

/// The claim: the refusal covers each crate that holds a command's logic, so a tree holding one at
/// another path than the binary was built from is refused.
#[test]
fn a_tree_holding_a_command_s_crate_at_another_path_than_the_binarys_is_refused() {
    for name in ["create", "query", "transcript"] {
        let package = format!("knowledge-architect-{name}");
        let manifest = format!("[package]\nname = \"{package}\"\n");
        let path = format!("crates/{name}/Cargo.toml");
        let sandbox = Sandbox::seeded(
            &format!("foreign-{name}"),
            "minimal",
            &[(path.as_str(), manifest.as_str())],
        );
        let (out, stderr, code) = sandbox.run(&["check"]);
        assert_eq!(code, 2, "{name}: {out}{stderr}");
        assert!(
            stderr.contains(&format!("its `{package}` was compiled at")),
            "{stderr}"
        );
        assert!(out.is_empty(), "nothing ran: {out}");
    }
}

/// The claim: the refusal covers the model crate too, so a tree holding it at another path than the
/// binary was built from is refused.
#[test]
fn a_tree_holding_the_model_crate_at_another_path_than_the_binarys_is_refused() {
    let sandbox = Sandbox::seeded(
        "foreign-model",
        "minimal",
        &[(
            "crates/model/Cargo.toml",
            "[package]\nname = \"knowledge-architect-model\"\n",
        )],
    );
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains("its `knowledge-architect-model` was compiled at"),
        "{stderr}"
    );
    assert!(out.is_empty(), "nothing ran: {out}");
}

/// The claim: a name holding a line break is one finding, on one line, and the file is read by
/// nothing; a `skip-files` row keeps it.
#[test]
fn a_name_holding_a_newline_is_one_finding_on_one_line_and_its_contents_are_read_by_nothing() {
    // The reproduction the issue named: a markdown file whose name holds a newline, holding a
    // reference that would be a finding if the file were read. The refusal is the one finding,
    // on one line, with the newline escaped; the reference inside is reported by nothing.
    let sandbox = Sandbox::seeded("newline-name", "minimal", &[("notes/a\nb.md", DANGLING)]);
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let about: Vec<&str> = out.lines().filter(|l| l.contains("notes/a")).collect();
    assert_eq!(about.len(), 1, "{out}");
    assert!(
        about[0].starts_with("notes/a\\nb.md  ") && about[0].contains("holds a line break"),
        "{out}"
    );
    assert!(!out.contains("no-such-thing"), "read by nothing: {out}");

    // The declared way to keep such a file: a `skip-files` row, which a TOML string spells
    // with an escape. The row takes the file out of the walk like any other, so the run
    // passes and the file is still read by nothing.
    let kept = Sandbox::seeded("newline-kept", "minimal", &[("notes/a\nb.md", DANGLING)]);
    let manifest =
        std::fs::read_to_string(kept.path("knowledge-architect.toml")).expect("the manifest");
    kept.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            "skip-files = [\"notes/a\\nb.md\", \"notes/generated.md\"]",
        ),
    );
    kept.stage();
    let (out, stderr, code) = kept.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(!out.contains("notes/a"), "{out}");
}

/// The claim: a `.gitignore` below the root decides the walk too.
///
/// The matcher this replaced read the root file alone, so a document under a nested ignore was
/// walked and its planted defect reported. Both halves are asserted, because a test that only
/// showed the silence would pass over a walk that had stopped reading anything at all.
#[test]
fn a_nested_gitignore_is_honoured() {
    let ignored = Sandbox::seeded(
        "nested-ignored",
        "minimal",
        &[
            ("notes/.gitignore", "scratch.md\n"),
            ("notes/scratch.md", DANGLING),
        ],
    );
    let (out, stderr, code) = ignored.run(&["check"]);
    assert!(
        !out.contains("no-such-thing"),
        "the nested ignore rule covers it: {out}{stderr}"
    );
    assert_eq!(code, 0, "{out}{stderr}");

    let walked = Sandbox::seeded(
        "nested-walked",
        "minimal",
        &[("notes/scratch.md", DANGLING)],
    );
    let (walked_out, stderr, code) = walked.run(&["check"]);
    assert_eq!(
        code, 1,
        "without the rule the file is live: {walked_out}{stderr}"
    );
    assert!(walked_out.contains("no-such-thing"), "{walked_out}");
    // Exactly one file separates the two copies, and the count says so. A count that did not
    // follow the walk would read alike here.
    assert_eq!(walked_count(&walked_out), walked_count(&out) + 1);
}

/// The claim: no `git` on the path is exit 2 naming git, never an empty walk.
///
/// An empty walk is the dangerous answer: a project with no document is reported as one with
/// nothing wrong, which is the failure class this tool exists to prevent.
#[test]
fn a_run_with_no_git_on_the_path_is_exit_two_naming_git() {
    let empty = std::env::temp_dir().join(format!("knowledge-no-git-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&empty);
    std::fs::create_dir_all(&empty).expect("a directory holding no git");
    let (stdout, stderr, code) = run_with_path(&project("minimal"), &["check"], &empty);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("git is not on the PATH"), "{stderr}");
    assert!(!stdout.contains("PASSED"), "{stdout}");
    let _ = std::fs::remove_dir_all(&empty);
}

/// The claim: a project directory outside any git worktree is exit 2 carrying git's own reason.
#[test]
fn a_project_outside_a_worktree_is_exit_two_carrying_gits_reason() {
    let sandbox = Sandbox::without_git("no-worktree", "minimal");
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(
        stderr.to_lowercase().contains("not a git repository"),
        "{stderr}"
    );
    assert!(!stdout.contains("PASSED"), "{stdout}");
}

/// The claim: a path reference whose target the ignore rules cover is exempt from the existence
/// assertion, and the exemption is decided by the rules rather than by what is on disk.
///
/// The target below is on no disk at all: it is a generated path, which is the case the
/// exemption exists for. A run that asked the filesystem, or that asked git nothing, reports it
/// as dangling — so the second half of the test is the same tree without the ignore line.
#[test]
fn a_reference_to_an_ignored_target_is_exempt_and_the_rules_decide_it() {
    let pointer = "# A mock document\n\nIt points at `path@minimal@build-output/out.txt` and at \
                   `path@minimal@build-output/`.\n";
    let exempt = Sandbox::seeded(
        "ignored-target",
        "minimal",
        &[(".gitignore", "build-output/\n"), ("notes/a.md", pointer)],
    );
    let (out, stderr, code) = exempt.run(&["check"]);
    assert_eq!(code, 0, "the ignore rules cover the target: {out}{stderr}");
    assert!(!out.contains("build-output"), "{out}");

    let asserted = Sandbox::seeded("unignored-target", "minimal", &[("notes/a.md", pointer)]);
    let (out, stderr, code) = asserted.run(&["check"]);
    assert_eq!(
        code, 1,
        "without the rule the target is asserted: {out}{stderr}"
    );
    assert_eq!(
        out.lines().filter(|l| l.contains("does not exist")).count(),
        2,
        "both the file claim and the directory claim: {out}"
    );
}

/// The claim: a tracked file the working tree does not hold is reported, not dropped.
///
/// An unstaged deletion leaves a path in git's listing with no bytes behind it. Dropping it
/// would take a live document out of every check on the strength of a working-tree state, which
/// is the shape `design@model@a-failed-parse-is-loud` refuses. The finding names the deletion
/// rather than reporting an encoding failure, because the two need different repairs.
#[test]
fn a_tracked_file_the_working_tree_does_not_hold_is_reported() {
    let sandbox = Sandbox::new("deleted-tracked", "minimal");
    let (before, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(
        code, 0,
        "the fixture starts clean under this family: {before}{stderr}"
    );

    std::fs::remove_file(sandbox.path("notes/b.md")).expect("a staged fixture file");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.contains("notes/b.md") && out.contains("the working tree does not hold it"),
        "{out}"
    );
    assert_eq!(
        walked_count(&out),
        walked_count(&before),
        "the path stays in the walk while the index holds it"
    );

    // Staging the deletion is the repair, and it takes the path out of the listing. What is
    // then reported is the fixture's own pointer at the file, dangling, which is the tree's
    // fact and not the walk's.
    sandbox.stage();
    let (after, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{after}{stderr}");
    assert!(
        !after.contains("the working tree does not hold it"),
        "{after}"
    );
    assert!(
        after.contains("`path@notes@b.md` does not exist"),
        "{after}"
    );
    assert_eq!(walked_count(&after), walked_count(&before) - 1);
}

/// The claim: the developer's own global ignore file does not decide what is walked.
///
/// `core.excludesFile` lives in the home directory and is no part of any project, so a line in
/// it would take an untracked live document out of every check on one clone and not on another.
/// The run is pinned against it, and this states the pin by giving the run a home directory
/// whose git configuration ignores one of the fixture's files.
#[test]
fn a_per_user_ignore_file_does_not_decide_the_walk() {
    // The document is left UNTRACKED on purpose: the ignore rules act on untracked files
    // alone, so a staged one would be listed whatever any ignore file said and the test would
    // pass over the pin without touching it.
    let sandbox = Sandbox::new("global-ignore", "minimal");
    sandbox.write("notes/scratch.md", DANGLING);
    let home = std::env::temp_dir().join(format!("knowledge-home-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(home.join(".config/git")).expect("a fake home");
    std::fs::write(home.join(".config/git/ignore"), "scratch.md\n").expect("a global rule");
    std::fs::write(
        home.join(".gitconfig"),
        "[core]\n\texcludesFile = ~/.config/git/ignore\n",
    )
    .expect("a global configuration");

    let config = home.join(".config");
    let env: Vec<(&str, &Path)> = vec![("HOME", home.as_path()), ("XDG_CONFIG_HOME", &config)];
    let (out, stderr, code) = run_with_env(&sandbox.dir, &["check"], &env);
    assert_eq!(code, 1, "the document stays live: {out}{stderr}");
    assert!(out.contains("no-such-thing"), "{out}");

    // The same run without the fake home, so the count is the same either way.
    let (plain, _, _) = sandbox.run(&["check"]);
    assert_eq!(walked_count(&out), walked_count(&plain));
    let _ = std::fs::remove_dir_all(&home);
}

/// The claim: a project written this way has nothing wrong with it, so a commit over it is
/// judged and contributes no finding of its tree.
///
/// Every test below reads a planted finding out of a run whose other findings are none. With
/// a tree that failed, every commit would carry its tree's findings and each test's counts
/// would be counts of the fixture.
#[test]
fn the_fixture_project_is_one_the_checks_find_nothing_wrong_with() {
    let history = History::new("commit-clean");
    tiny_project(&history, false);
    history.commit("The project is created\n");
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("PASSED: no findings"), "{stdout}");
}

/// The claim: where two or more commits of the range fail on one generated file, `commits` prints
/// one line naming the file and the repair that reaches every one of them, regenerating it at
/// each and folding each regeneration into its commit, since `index` at the tip repairs the last
/// alone; a file one commit fails on gets no such line. Per `design@check@finding-names-the-repair`,
/// the line names no cause.
#[test]
fn commits_names_the_repair_of_a_generated_file_several_commits_fail_on() {
    let history = History::new("commits-generated");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // Committed by hand, so no index is regenerated: every commit holds the stale listing.
    let raw = |subject: &str| {
        history.git(&["-c", "core.excludesFile=/dev/null", "add", "-A"]);
        history.git(&["commit", "-q", "-m", subject]);
    };
    let index = "docs/open-issues/index.md";
    let listing = std::fs::read_to_string(history.dir.join(index)).expect("the index");
    history.write(
        index,
        &format!("{listing}| todo | A row no entry backs |\n"),
    );
    raw("The index gains a row");
    history.write("notes.txt", "one\n");
    raw("A note");
    history.write("notes.txt", "two\n");
    raw("The note again");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    let line = stdout
        .lines()
        .find(|l| l.contains(index) && l.contains("3 commits"))
        .unwrap_or_else(|| panic!("a line naming the file and its commits: {stdout}"));
    assert!(line.contains("fold"), "{line}");
    let (stdout, stderr, code) = history.run(&["commits", "HEAD~1..HEAD"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(!stdout.contains("fold"), "one failing commit: {stdout}");

    // Two commits failing on a hand-written file get no such line: `index` is no repair there.
    history.write(index, &listing);
    let fixed = history.commit("The index is regenerated\n");
    let readme = std::fs::read_to_string(history.dir.join("README.md")).expect("the README");
    history.write(
        "README.md",
        &format!("{readme}\nIt leans on `design@tiny@no-such-decision`.\n"),
    );
    raw("The README leans on nothing");
    history.write("notes.txt", "three\n");
    raw("The note a third time");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{fixed}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(!stdout.contains("fold"), "a hand-written file: {stdout}");
}

#[test]
fn a_message_naming_nothing_that_exists_fails_and_names_the_commit_and_the_line() {
    let history = History::new("commit-dangling");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let sha = history.commit(
        "A subject line\n\nA body naming `design@tiny@no-such-decision`, which is nowhere.\n",
    );
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{sha} failed: its message carries 1 finding(s)")),
        "{stdout}"
    );
    // The commit, then the line inside the message: the body sits on line three.
    assert!(
        stdout.contains(&format!("commit {sha}:3")),
        "the finding names the commit and the line: {stdout}"
    );
    assert!(stdout.contains("no-such-decision"), "{stdout}");
}

/// `tiny_project`, with `[commits] refuse-branch-shas = true` in its manifest.
fn tiny_project_refusing_branch_shas(history: &History) {
    tiny_project(history, false);
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        format!("{text}\n[commits]\nrefuse-branch-shas = true\n"),
    )
    .expect("the manifest is rewritten");
}

/// The claim: under `refuse-branch-shas`, a message citing a commit of the range by its SHA
/// fails, naming the commit and the line, and the citing commit is counted as failed, its line
/// naming the citations and how many. Mutation checked: the count off by one.
#[test]
fn a_message_citing_a_commit_of_the_range_by_sha_fails_under_refuse_branch_shas() {
    let history = History::new("commit-branch-sha");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    let second = history.commit(&format!(
        "A second change\n\nIt follows {first}.\nIt names {first} again.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {second}:3")), "{stdout}");
    assert!(stdout.contains(&format!("commit {second}:4")), "{stdout}");
    // The cited commit is named by its 7-digit abbreviation, followed by the rest of the sentence.
    assert!(
        stdout.contains(&format!(
            "`{first}` cites commit {first}, a commit of the range"
        )),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!(
            "{second} failed: it cites the range by SHA 2 time(s)\n"
        )),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{first} passed")), "{stdout}");
}

/// The claim: whether a commit is judged for branch SHAs is read from that commit's own
/// manifest, not from the working tree's. Mutation: reading the working tree's manifest, which
/// the last commit turns the option off in, lets the citing commit pass.
#[test]
fn a_commit_is_judged_for_branch_shas_under_its_own_manifest() {
    let history = History::new("commit-branch-sha-own-manifest");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    let second = history.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        text.replace("refuse-branch-shas = true", "refuse-branch-shas = false"),
    )
    .expect("the manifest is rewritten");
    let third = history.commit("The option is turned off\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{second} failed")), "{stdout}");
    assert!(stdout.contains(&format!("{third} passed")), "{stdout}");
}

/// The claim: a commit whose tree carries a finding of its own is still judged for branch SHAs,
/// and both are reported. Mutation: judging citations only where the tree is clean drops them.
#[test]
fn a_citation_in_a_commit_whose_tree_fails_is_reported_beside_the_tree_finding() {
    let history = History::new("commit-branch-sha-failing-tree");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write(
        "README.md",
        "# tiny\n\nA project a test builds. It names `design@tiny@no-such-decision`.\n",
    );
    let second = history.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("no-such-decision"), "{stdout}");
    assert!(stdout.contains(&format!("commit {second}:3")), "{stdout}");
}

/// The claim: a commit whose tree stops at phase 2 is still scanned for citations of the range,
/// and its line names both causes with the count, per `design@check@branch-shas-are-refused`;
/// without the option, the same commit is not scanned.
#[test]
fn a_citation_in_a_commit_whose_tree_stops_at_phase_two_is_reported() {
    let history = History::new("commit-branch-sha-stopped-tree");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write_bytes(
        "docs/latin1.md",
        b"# A note\n\nOne byte of Windows-1252: caf\xe9.\n",
    );
    let second = history.commit(&format!(
        "A second change\n\nIt follows {first}.\nIt names {first} again.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {second}:3")), "{stdout}");
    assert!(stdout.contains(&format!("commit {second}:4")), "{stdout}");
    assert!(
        stdout.contains(&format!(
            "{second} failed: its tree stops at phase 2 with 1 finding(s); its message was \
             judged against nothing; it cites the range by SHA 2 time(s)\n"
        )),
        "{stdout}"
    );

    let plain = History::new("commit-branch-sha-stopped-tree-off");
    tiny_project(&plain, false);
    let base = plain.commit("The project is created\n");
    let first = plain.commit("A first change\n");
    plain.write_bytes(
        "docs/latin1.md",
        b"# A note\n\nOne byte of Windows-1252: caf\xe9.\n",
    );
    plain.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let (stdout, stderr, code) = plain.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("its tree stops at phase 2"), "{stdout}");
    assert!(!stdout.contains("cites the range"), "{stdout}");
}

/// The claim: a commit whose tree stops at phase 1 is not scanned for citations, since its
/// manifest did not resolve, per `design@check@branch-shas-are-refused`.
#[test]
fn a_commit_whose_tree_stops_at_phase_one_is_not_scanned_for_citations() {
    let history = History::new("commit-branch-sha-phase-one");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    let manifest =
        std::fs::read_to_string(history.dir.join("knowledge-architect.toml")).expect("manifest");
    history.write("knowledge-architect.toml", &format!("{manifest}\n[lint]\n"));
    let second = history.commit(&format!(
        "A table no extension claims\n\nIt follows {first}.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{second} failed: its tree stops at phase 1")),
        "{stdout}"
    );
    assert!(!stdout.contains("cites the range"), "{stdout}");
    assert!(!stdout.contains(&format!("commit {second}:3")), "{stdout}");
}

/// The claim: a Rust document is read whole, its code included. Mutation: reading Markdown
/// documents alone misses it.
#[test]
fn a_rust_document_citing_a_commit_of_the_range_fails() {
    let history = History::new("commit-branch-sha-rust");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write(
        "src/lib.rs",
        &format!("const FOLLOWS: &str = \"{first}\";\n"),
    );
    let second = history.commit("A source file names a change\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {second}: src/lib.rs:1")),
        "{stdout}"
    );
}

/// The claim: a document of a commit's tree citing a commit of the range fails too, named by
/// the commit and the file.
#[test]
fn a_document_citing_a_commit_of_the_range_by_sha_fails_under_refuse_branch_shas() {
    let history = History::new("commit-branch-sha-doc");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    let first = history.commit("A first change\n");
    history.write(
        "README.md",
        &format!("# tiny\n\nA project a test builds. It changed in {first}.\n"),
    );
    let second = history.commit("The readme names a change\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {second}: README.md:3")),
        "{stdout}"
    );
}

/// The claim: a SHA outside the range passes under the option, and a citation of the range
/// passes without it. Mutations: matching any hex run fails the first half, and ignoring the
/// manifest fails the second.
#[test]
fn a_sha_outside_the_range_or_a_project_without_the_option_passes() {
    let history = History::new("commit-main-sha");
    tiny_project_refusing_branch_shas(&history);
    let base = history.commit("The project is created\n");
    history.commit(&format!(
        "A change\n\nIt builds on {base}, already on the main branch.\n"
    ));
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");

    let plain = History::new("commit-branch-sha-off");
    tiny_project(&plain, false);
    let base = plain.commit("The project is created\n");
    let first = plain.commit("A first change\n");
    plain.commit(&format!("A second change\n\nIt follows {first}.\n"));
    let (stdout, stderr, code) = plain.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

#[test]
fn a_message_naming_the_entry_its_commit_deletes_resolves_against_the_parent() {
    let history = History::new("commit-parent");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.remove("docs/open-issues/a-closable-issue.md");
    let sha = history
        .commit("The closable issue is closed\n\nIt closed `issue@tiny@a-closable-issue`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(
        code, 0,
        "the parent tree still defines it: {stdout}{stderr}"
    );
    assert!(stdout.contains(&format!("{sha} passed")), "{stdout}");
}

/// The claim: `--help` opens with the binary's own description, not with the doc comment the
/// library writes for a binary's author about the command enum.
#[test]
fn help_opens_with_the_binarys_description() {
    let out = Command::new(klarch())
        .arg("--help")
        .output()
        .expect("the binary runs");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}");
    assert_eq!(
        stdout.lines().next(),
        Some("What a project's documents hold, and whether it still holds."),
        "{stdout}"
    );
    assert!(!stdout.contains("flattens this enum"), "{stdout}");
}

/// The claim: in a message, an unanchored path whose first segment only the parent tree holds
/// is still a finding, since the message points at what its commit deletes; a path-shaped span
/// that names nothing in either tree is silent.
#[test]
fn a_message_naming_the_file_its_commit_deletes_by_a_bare_path_is_a_finding() {
    let history = History::new("commit-bare-path");
    tiny_project(&history, false);
    history.write(
        "scratch/notes.md",
        "# Notes\n\nA file a later commit deletes.\n",
    );
    let base = history.commit("The project is created\n");
    let silent = history.commit("A subject line\n\nIt answers in `application/json`.\n");
    history.remove("scratch/notes.md");
    let sha = history.commit("The notes leave\n\nIt deletes `scratch/notes.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`scratch/notes.md` is shaped like a path"),
        "{stdout}"
    );
    assert!(!stdout.contains(&format!("commit {silent}:")), "{stdout}");
}

/// The claim: a message's lints on ids read its first parent's tree too, so a message that names
/// in words the decision its commit deletes is reported, and one that names it in the commit
/// before is reported too, against its own tree. Mutation checked: judging the message's lints
/// against its own tree alone leaves the deleting commit's message silent.
#[test]
fn a_message_naming_in_words_the_entry_its_commit_deletes_is_a_finding() {
    let history = History::new("commit-plain-id");
    tiny_project(&history, false);
    let design = "# tiny — design\n\n### A decision this project records `##tiny-anchor`\n\n\
                  It exists so that a message has something to name.\n";
    history.write(
        "docs/design.md",
        &format!("{design}\n### A decision a later commit removes `##gone-anchor`\n"),
    );
    let base = history.commit("The project is created\n");
    let named = history.commit("A subject line\n\nIt keeps the gone-anchor decision.\n");
    history.write("docs/design.md", design);
    let sha = history.commit("The decision leaves\n\nIt removes the gone-anchor decision.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for commit in [named, sha] {
        assert!(
            stdout.contains(&format!("commit {commit}:3"))
                && stdout.contains("`gone-anchor` is written as a plain word"),
            "{stdout}"
        );
    }
}

/// The claim: an id that a commit's tree and its parent's define under different kinds is one
/// finding in its message, listing both references, not one finding per tree. Mutation checked:
/// judging the message's lints on ids once per tree gives two findings on the line.
#[test]
fn a_message_naming_an_id_two_trees_define_apart_lists_both() {
    let history = History::new("commit-id-two-trees");
    tiny_project(&history, false);
    let design = "# tiny — design\n\n### A decision this project records `##tiny-anchor`\n\n\
                  It exists so that a message has something to name.\n";
    history.write(
        "docs/design.md",
        &format!("{design}\n### A second decision `##second-way`\n"),
    );
    let base = history.commit("The project is created\n");
    history.write(
        "docs/design.md",
        &format!("{design}It rejected `rejected@tiny@second-way`.\n"),
    );
    history.write(
        "docs/rejected.md",
        "# tiny — rejected designs\n\n### A second decision `##second-way`\n\n`live`. It lost.\n",
    );
    let sha = history.commit("The decision is reversed\n\nThe second-way decision is rejected.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    let lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.starts_with(&format!("commit {sha}:3")))
        .collect();
    assert_eq!(lines.len(), 1, "{stdout}");
    assert!(lines[0].contains("2 entries define it"), "{stdout}");
}

/// The claim: a commit whose message alone carries findings is counted failed, and its line
/// says so, while the commit before it passes. Mutation checked: the message's findings left
/// out of the causes.
#[test]
fn a_commit_failing_on_its_message_alone_is_counted_failed() {
    let history = History::new("commit-message-fails");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let clean = history.commit("A subject line\n\nIt argues from `design@tiny@tiny-anchor`.\n");
    let sha = history
        .commit("A subject line\n\nIt names `design@tiny@nothing` and `design@tiny@none`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{clean} passed")), "{stdout}");
    assert!(
        stdout.contains(&format!("{sha} failed: its message carries 2 finding(s)")),
        "{stdout}"
    );
    assert!(stdout.contains("2 commits, 1 passed, 1 failed"), "{stdout}");
    let (stdout, stderr, _) = history.run(&["commits", &format!("{clean}..HEAD")]);
    assert!(
        stdout.contains("1 commit, 0 passed, 1 failed"),
        "{stdout}{stderr}"
    );
}

/// The claim: a lint both trees raise on one line is reported once, and a lint on another line
/// is reported again. Mutation checked: the union of the two trees' lints made a plain extend.
#[test]
fn a_lint_both_trees_raise_is_reported_once() {
    let history = History::new("commit-lint-once");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let sha =
        history.commit("A subject line\n\nIt reads `docs/design.md`.\nAnd `docs/design.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for line in [3, 4] {
        let site = format!("commit {sha}:{line} ");
        assert_eq!(
            stdout.lines().filter(|l| l.contains(&site)).count(),
            1,
            "{stdout}"
        );
    }
}

/// The claim: a message's `path` reference to a target an ignore rule covers is exempt, as the
/// same reference in a document is, though no document of either tree cites it. Mutation checked:
/// the message's own ignore answers left empty reports the target as missing.
#[test]
fn a_message_citing_an_ignored_target_no_document_cites_passes() {
    let history = History::new("commit-message-ignored");
    tiny_project(&history, false);
    // No file is written under `build/`: an ignored target is exempt whether or not it exists,
    // and a file on disk would not tell the two trees' listings from the ignore answer.
    history.write(".gitignore", "build/\n");
    let base = history.commit("The project is created\n");
    history.commit("A subject line\n\nIt reads `path@tiny@build/out.txt`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

/// The claim: each tree judges a message's span with the message's own ignore answers, asked
/// against its own anchors: in a commit that renames the project, a span to an ignored path at
/// the old root's name, which only the parent's anchors hold, and one at the new name, which only
/// the commit's own hold, both pass. Mutations checked: the commit's tree, or the parent's, judged
/// with its own answers alone, and the parent's anchors left out of the message's batch, each
/// reports a span.
#[test]
fn a_message_s_ignored_spans_are_judged_in_each_tree_with_its_own_anchors() {
    let history = History::new("commit-message-ignored-rename");
    tiny_project(&history, false);
    history.write(".gitignore", "build/\n");
    let base = history.commit("The project is created\n");
    for rel in ["knowledge-architect.toml", "docs/tripwires.md"] {
        let text = std::fs::read_to_string(history.dir.join(rel)).expect("a file of the project");
        history.write(
            rel,
            &text
                .replace("name = \"tiny\"", "name = \"small\"")
                .replace("design@tiny@", "design@small@"),
        );
    }
    // The old name alone is the case where only the parent's anchors give the question: both
    // names are the root's, so a span at the new name asks git the same path.
    for message in [
        "The project is renamed\n\nIts old build output was `path@tiny@build/out.txt`.\n\
         Its new one is `path@small@build/out.txt`.\n",
        "The project is renamed\n\nIts old build output was `path@tiny@build/out.txt`.\n",
    ] {
        history.commit(message);
        let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
        assert_eq!(code, 0, "{message}: {stdout}{stderr}");
        history.git(&["reset", "-q", "--hard", &base]);
        for rel in ["knowledge-architect.toml", "docs/tripwires.md"] {
            let text =
                std::fs::read_to_string(history.dir.join(rel)).expect("a file of the project");
            history.write(
                rel,
                &text
                    .replace("name = \"tiny\"", "name = \"small\"")
                    .replace("design@tiny@", "design@small@"),
            );
        }
    }
}

/// The claim: a message's span that git refuses to be asked about, one beyond a symbolic link,
/// fails its commit alone, naming git's reason, and the run judges the range rather than stopping.
/// Mutation checked: the refusal propagated with `?` again exits 2 with no commit judged.
#[cfg(unix)]
#[test]
fn a_message_span_git_refuses_to_ask_about_fails_its_commit_alone() {
    let history = History::new("commit-message-beyond-link");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.commit("A subject line\n\nIt is not `path@tiny@link/design.md`.\n");
    std::os::unix::fs::symlink("docs", history.dir.join("link")).expect("a symbolic link");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("1 commit, 0 passed, 1 failed"), "{stdout}");
    assert!(
        stdout.contains("the ignore rules could not be asked about its message"),
        "{stdout}"
    );
}

/// The claim: a span repeated on one line of a message is one finding per occurrence, whichever
/// tree holds what it names: the commit's own, both, or the parent's alone, as the commit that
/// deletes the file has it. Mutation checked: the union of the two trees' lints made a set again
/// collapses the parent's two occurrences into one.
#[test]
fn a_lint_repeated_on_a_line_is_one_finding_per_occurrence_in_either_tree() {
    let history = History::new("commit-lint-repeated");
    tiny_project(&history, false);
    history.write(
        "scratch/notes.md",
        "# Notes\n\nA file the next commit deletes.\n",
    );
    let base = history.commit("The project is created\n");
    history.remove("scratch/notes.md");
    let sha = history.commit(
        "The notes leave\n\nIt deletes `scratch/notes.md` and `scratch/notes.md`.\n\
         It keeps `docs/design.md` and `docs/design.md`.\n",
    );
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for line in [3, 4] {
        let site = format!("commit {sha}:{line} ");
        assert_eq!(
            stdout.lines().filter(|l| l.contains(&site)).count(),
            2,
            "line {line}: {stdout}"
        );
    }
}

/// The claim: in a message, a retired slug reference is a finding when its id is an entry of
/// the project, and silent when it names nothing here.
#[test]
fn a_message_holding_the_retired_slug_form_is_judged_by_what_it_names() {
    let history = History::new("commit-retired");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let silent = history.commit("A subject line\n\nIt closes `#123` and keeps `#include`.\n");
    let sha = history.commit("A subject line\n\nIt argues from `#tiny-anchor`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`#tiny-anchor` is the retired slug reference form"),
        "{stdout}"
    );
    assert!(!stdout.contains(&format!("commit {silent}:")), "{stdout}");
}

/// The claim: in a message, an unanchored path whose first segment only the commit's own tree
/// holds is a finding, as one only the parent holds is.
#[test]
fn a_message_naming_the_file_its_commit_adds_by_a_bare_path_is_a_finding() {
    let history = History::new("commit-bare-added");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write("scratch/notes.md", "# Notes\n\nA file this commit adds.\n");
    let sha = history.commit("The notes arrive\n\nIt adds `scratch/notes.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`scratch/notes.md` is shaped like a path"),
        "{stdout}"
    );
}

/// The claim: in a message, a bare path is read from the parent's anchors in the parent's tree, so
/// a path relative to a Component the commit removes is still a finding.
#[test]
fn a_message_naming_a_path_of_a_component_its_commit_removes_is_a_finding() {
    let history = History::new("commit-bare-component");
    tiny_project(&history, false);
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        text.replace("components = []", "components = [\"part\"]"),
    )
    .expect("the manifest is rewritten");
    let part = [
        (
            "part/README.md",
            "# part\n\nA Component a later commit removes.\n",
        ),
        ("part/docs/design.md", "# part — design\n\nNo entry.\n"),
        (
            "part/docs/tripwires.md",
            "# Tripwires — part\n\nNo entry.\n",
        ),
        ("part/docs/rejected.md", "# part — rejected\n\nNothing.\n"),
        (
            "part/docs/open-issues/README.md",
            "# Open issues — part\n\nOne file per entry.\n",
        ),
        ("part/src/lib.rs", "pub fn f() {}\n"),
    ];
    for (rel, body) in part {
        history.write(rel, body);
    }
    let base = history.commit("The project is created\n");
    std::fs::write(&manifest, text).expect("the manifest is restored");
    std::fs::remove_dir_all(history.dir.join("part")).expect("the Component is removed");
    let sha = history.commit("The part leaves\n\nIt drops `src/lib.rs`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("commit {sha}:3"))
            && stdout.contains("`src/lib.rs` is shaped like a path"),
        "{stdout}"
    );
}

/// The claim: an unanchored path through a tracked symlink is judged like any other, in a
/// document and in a message, and asks git nothing that a symlink makes it refuse.
#[test]
fn a_bare_path_through_a_tracked_symlink_is_a_finding_and_not_an_error() {
    let history = History::new("bare-symlink");
    tiny_project(&history, false);
    let manifest = history.dir.join("knowledge-architect.toml");
    let text = std::fs::read_to_string(&manifest).expect("the manifest");
    std::fs::write(
        &manifest,
        text.replace("skip-files = []", "skip-files = [\"lnk\"]"),
    )
    .expect("the manifest is rewritten");
    std::os::unix::fs::symlink("docs", history.dir.join("lnk")).expect("a symlink");
    let base = history.commit("The project is created\n");
    history.commit("A subject line\n\nIt reads `lnk/design.md`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("`lnk/design.md` is shaped like a path"),
        "{stdout}"
    );
    history.write("notes.md", "# Notes\n\nSee `lnk/design.md`.\n");
    let (stdout, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("`lnk/design.md` is shaped like a path"),
        "{stdout}"
    );
}

/// The claim: a commit message citing a plan document in the commit that deletes it resolves
/// against the parent, for a spec of `specs/` and for a slice spec, and also when the whole
/// milestone leaves, so its anchor is gone from the commit's tree. The deletion that ends a plan
/// document's life is written this way, since a `path` citation of one is refused.
#[test]
fn a_message_naming_the_plan_document_its_commit_deletes_resolves_against_the_parent() {
    let history = History::new("commit-plan-parent");
    tiny_project(&history, false);
    history.write(
        "docs/plans/specs/a-spec.md",
        &plan_document(
            "A spec that lands",
            "It is the plan of one pull request.",
            &[],
        ),
    );
    history.write(
        "docs/plans/milestones/a-milestone/README.md",
        &plan_document(
            "A milestone that lands",
            "Its steps: [the first](first-step.md), [the last](last-step.md); [the index](index.md).",
            &[],
        ),
    );
    history.write(
        "docs/plans/milestones/a-milestone/first-step.md",
        &slice_document("Its first step"),
    );
    history.write(
        "docs/plans/milestones/a-milestone/last-step.md",
        &slice_document("Its last step"),
    );
    let base = history.commit("The plans are written\n");
    history.remove("docs/plans/specs/a-spec.md");
    history.remove("docs/plans/milestones/a-milestone/first-step.md");
    history.write(
        "docs/plans/milestones/a-milestone/README.md",
        &plan_document(
            "A milestone that lands",
            "Its step: [the last](last-step.md); [the index](index.md).",
            &[],
        ),
    );
    let spec = history.commit(
        "The spec and the first step land\n\nThey leave: `spec@plans@a-spec` and \
         `spec@a-milestone@first-step`.\n",
    );
    for rel in ["README.md", "last-step.md", "index.md"] {
        history.remove(&format!("docs/plans/milestones/a-milestone/{rel}"));
    }
    let milestone = history.commit(
        "The milestone lands\n\nIt leaves: `milestone@plans@a-milestone` and \
         `spec@a-milestone@last-step`.\n",
    );
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(
        code, 0,
        "the parent tree still defines each: {stdout}{stderr}"
    );
    assert!(stdout.contains(&format!("{spec} passed")), "{stdout}");
    assert!(stdout.contains(&format!("{milestone} passed")), "{stdout}");
}

/// The claim: a commit message cites a plan document whole, never an item. It is one document
/// at the root, inside no plan, so an item citation in it is the out-of-plan refusal, while the
/// whole document resolves. Mutation checked: `judge_message` resolving with no citing file,
/// which treats the message as inside every plan.
#[test]
fn a_message_cites_a_plan_whole_and_never_an_item() {
    let history = History::new("commit-plan-item");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/plans/specs/a-spec.md",
        &plan_document(
            "A spec with one thread",
            "It is the plan of one pull request.",
            &[(
                "Threads",
                "### It holds one thread `##one-thread`\n\nApproved.",
            )],
        ),
    );
    let whole = history.commit("A spec is planned\n\nIt is `spec@plans@a-spec`.\n");
    let item = history.commit("Its thread is named\n\nIt is `thread@a-spec@one-thread`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{whole} passed")), "{stdout}");
    assert!(
        stdout.contains(&format!("commit {item}:3"))
            && stdout.contains("cites an item of the plan `a-spec` from outside it"),
        "{stdout}"
    );
}

/// The claim: the anchors a message is judged against are its commit's, milestone anchors
/// included, and not the working tree's. The milestone's deletion is staged and not committed
/// before the range is judged, so git's listing of the working tree holds no anchor
/// `a-milestone`, and a run that read the anchors off it would find none in either tree.
#[test]
fn a_commit_s_milestone_anchors_are_read_off_its_own_tree() {
    let history = History::new("commit-milestone-anchor");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/plans/milestones/a-milestone/README.md",
        &plan_document(
            "A milestone",
            "Its step: [the step](a-step.md); [the index](index.md).",
            &[],
        ),
    );
    history.write(
        "docs/plans/milestones/a-milestone/a-step.md",
        &slice_document("Its step"),
    );
    let sha = history.commit("A milestone is planned\n\nIts step is `spec@a-milestone@a-step`.\n");
    for rel in ["README.md", "a-step.md", "index.md"] {
        history.remove(&format!("docs/plans/milestones/a-milestone/{rel}"));
    }
    history.git(&["add", "-A"]);
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(
        code, 0,
        "the commit's own tree defines it: {stdout}{stderr}"
    );
    assert!(stdout.contains(&format!("{sha} passed")), "{stdout}");
}

/// The claim: a reference that resolves in NEITHER tree is reported, however differently the
/// two trees phrase their refusal.
///
/// A commit that adds a component makes the two arms disagree about the words: the parent has
/// no such anchor at all, the commit has the anchor and not the id. Intersecting the findings
/// whole then produces the empty set, and a reference nothing defines anywhere passes.
#[test]
fn a_reference_no_tree_defines_is_reported_though_the_two_refuse_it_differently() {
    let history = History::new("commit-both-refuse");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");

    // A second component, which the parent tree does not know as an anchor at all.
    for (rel, text) in [
        ("other/README.md", "# other\n\nA second component.\n"),
        ("other/CLAUDE.md", "# other\n\nNothing holds of it.\n"),
        (
            "other/docs/design.md",
            "# other — design\n\n### It records one decision `##other-anchor`\n\nIt exists.\n",
        ),
        (
            "other/docs/tripwires.md",
            "# Tripwires — other\n\nNothing yet.\n",
        ),
        (
            "other/docs/rejected.md",
            "# other — rejected designs\n\nNo design rejected yet.\n",
        ),
        (
            "other/docs/open-issues/README.md",
            "# Open issues — other\n\nOne file per entry.\n",
        ),
    ] {
        history.write(rel, text);
    }
    let manifest =
        std::fs::read_to_string(history.dir.join("knowledge-architect.toml")).expect("a manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest.replace("components = []", "components = [\"other\"]"),
    );
    let sha = history.commit(
        "A second component arrives\n\nIt names `design@other@no-such-slug`, which nothing defines.\n",
    );

    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "neither tree defines it: {stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {sha}:3")), "{stdout}");
    assert!(stdout.contains("no-such-slug"), "{stdout}");
}

/// The claim: a commit whose manifest does not load fails the run and is
/// named, and the commit after it is still judged.
///
/// Its message cannot be judged, since no table exists to resolve it against, so the tree's
/// failure is the finding. A run that passed over it would take the message out of the regime
/// with exit 0.
#[test]
fn a_commit_whose_manifest_does_not_load_fails_the_run_and_the_next_one_is_judged() {
    let history = History::new("commit-premigration");
    tiny_project(&history, true);
    let base = history.commit("The project is created\n");
    let old = history.commit("A commit from before the manifest was migrated\n");
    tiny_project(&history, false);
    let new = history.commit("The manifest is migrated\n\nIt records `design@tiny@tiny-anchor`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{old} failed: its tree does not load")),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{new} passed")), "{stdout}");
    assert!(stdout.contains("2 commits, 1 passed, 1 failed"), "{stdout}");
    assert!(
        stdout.contains(&format!("commit {old}  this commit's tree does not load")),
        "the finding names the commit: {stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: a commit whose tree carries a finding fails the run with that
/// finding, named by the commit and the file, and its message is still judged.
///
/// The tip checker judges every commit of the branch, so a tree that fails at any commit is a
/// finding of the branch. Its message is judged too, since the tree reached the last phase and
/// its table is complete: a dangling reference in it is reported beside the tree's own.
#[test]
fn a_failing_tree_is_a_finding_and_its_message_is_still_judged() {
    let history = History::new("commit-middle-fails");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/note.md",
        "# A note\n\nIt names `design@tiny@no-such-decision`.\n",
    );
    let mid = history.commit("A note is added\n\nIt names `design@tiny@no-such-message-ref`.\n");
    history.remove("docs/note.md");
    let fixed = history.commit("The note leaves\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{mid} failed: its tree fails 1 finding(s); its message carries 1 finding(s)"
        )),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{fixed} passed")), "{stdout}");
    assert!(stdout.contains("2 commits, 1 passed, 1 failed"), "{stdout}");
    // The tree's finding, named by the commit and by the file inside it.
    assert!(
        stdout.contains(&format!("commit {mid}: docs/note.md:3")),
        "the tree's finding names the commit and the file: {stdout}"
    );
    // The message's finding, on line three of the message.
    assert!(stdout.contains(&format!("commit {mid}:3")), "{stdout}");
    assert!(stdout.contains("no-such-message-ref"), "{stdout}");
}

/// The claim: a commit whose tree stops before the last phase fails the run,
/// and its message is judged against nothing, which the run says.
///
/// Its entity table is incomplete, so a finding against the message would be computed over
/// what the walk could not read. The tree's own findings are what is reported.
#[test]
fn a_tree_that_stops_early_fails_and_its_message_is_not_judged() {
    let history = History::new("commit-middle-stops");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write_bytes("docs/latin1.md", b"# A note\n\ncaf\xe9\n");
    let mid =
        history.commit("A note that is not text\n\nIt names `design@tiny@no-such-message-ref`.\n");
    history.remove("docs/latin1.md");
    history.commit("The note leaves\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{mid} failed: its tree stops at phase 2")),
        "{stdout}"
    );
    assert!(stdout.contains("latin1.md"), "{stdout}");
    assert!(
        !stdout.contains("no-such-message-ref"),
        "no finding computed over the incomplete table: {stdout}"
    );
}

/// The claim: a tree entry the walk reads as a document and whose blob is not text is a
/// finding of that tree, so the commit fails for it rather than being judged clean over a
/// document nothing read.
#[test]
fn a_commit_holding_a_blob_that_is_not_text_fails_its_tree() {
    let history = History::new("commit-blob");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write_bytes(
        "docs/latin1.md",
        b"# A note\n\nOne byte of Windows-1252: caf\xe9.\n",
    );
    let sha = history.commit("A note that is not text is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s)"
        )),
        "{stdout}"
    );
}

/// The claim: a last commit whose tree stops before the last phase fails the run like any other
/// commit, and has its message judged against nothing rather than against the incomplete entity
/// table.
#[test]
fn a_last_commit_whose_tree_stops_before_the_last_phase_fails_and_its_message_is_unjudged() {
    let history = History::new("commit-stopped-last");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // The new decision is defined in a file the walk cannot read, so against the table the
    // reference would dangle; the message is right, and the tree is what is wrong.
    history.write_bytes(
        "docs/new.md",
        b"# New\n\n### A new decision `##new-one`\n\ncaf\xe9\n",
    );
    let sha = history.commit("Adds `design@tiny@new-one`\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "{sha} failed: its tree stops at phase 2 with 1 finding(s); its message was judged \
             against nothing"
        )),
        "{stdout}"
    );
    assert!(
        !stdout.contains("is referenced and"),
        "no finding computed over the incomplete table: {stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: the range's last commit is judged like every other: its failing tree is a
/// finding naming the commit, and exit 1.
#[test]
fn a_last_commit_whose_tree_fails_is_a_finding_of_the_run() {
    let history = History::new("commit-head-fails");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // A live document naming a decision nothing defines: one `references` finding, in the
    // tree rather than in the message.
    history.write(
        "docs/note.md",
        "# A note\n\nIt names `design@tiny@no-such-decision`.\n",
    );
    let sha = history.commit("A note is added\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{sha} failed: its tree fails 1 finding(s)")),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {sha}: docs/note.md"))),
        "{stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: `commits` reads every checker directory it is given as data in each commit's
/// tree, per `design@check@checker-source-literals-are-data`, and not only the first.
///
/// The binary compiles its own directories in, and they are not inside a temporary history,
/// so this calls the library with directories inside it. Two tool directories each hold a
/// Rust source whose unbound string literal names a decision nothing defines, in a commit whose
/// successor removes them, so only that commit's tree can fail.
#[test]
fn commits_reads_every_checker_directory_as_data_in_each_commits_tree() {
    let history = History::new("commit-checker-dirs");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    for tool in ["tool-a", "tool-b"] {
        history.write(
            &format!("{tool}/src/lib.rs"),
            "pub fn f() {\n    println!(\"{}\", \"`design@tiny@no-such-decision`\");\n}\n",
        );
    }
    history.commit("Two tools are added\n");
    history.remove("tool-a/src/lib.rs");
    history.remove("tool-b/src/lib.rs");
    history.commit("The tools are removed\n");
    let manifest = knowledge_architect::Manifest::find(&history.dir).expect("the manifest");
    let range = format!("{base}..HEAD");
    let (a, b) = (history.dir.join("tool-a"), history.dir.join("tool-b"));
    let judge = |dirs: &[&Path]| {
        let command =
            knowledge_architect::cli::Command::Commits(knowledge_architect::cli::CommitsArgs {
                range: range.clone(),
            });
        knowledge_architect::cli::run(command, &manifest, dirs, &mut []).expect("the run completes")
    };
    // The literals are live where no directory covers them, so the fixture can fail.
    assert_eq!(judge(&[]), std::process::ExitCode::FAILURE);
    assert_eq!(judge(&[&a]), std::process::ExitCode::FAILURE);
    assert_eq!(judge(&[&a, &b]), std::process::ExitCode::SUCCESS);
}

/// The claim: a commit whose manifest fails phase 1 is read no further and serves as no
/// parent, per `design@check@a-commit-message-is-a-document`. The next commit's message then
/// resolves against its own tree alone, so naming an entry that only the failed tree held
/// dangles.
#[test]
fn a_tree_whose_phase_one_fails_serves_as_no_parent() {
    let history = History::new("commit-phase-one-parent");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let manifest =
        std::fs::read_to_string(history.dir.join("knowledge-architect.toml")).expect("manifest");
    history.write("knowledge-architect.toml", &format!("{manifest}\n[lint]\n"));
    let failed = history.commit("A table no extension claims\n");
    history.write("knowledge-architect.toml", &manifest);
    history.remove("docs/open-issues/a-closable-issue.md");
    // The escape names a path this tree holds, so a real parent refuses it as the next tree
    // does and the finding stands; an empty tree kept as a parent would accept it and drop it.
    let next =
        history.commit("Closes `issue@tiny@a-closable-issue`. See `path@elsewhere@README.md`.\n");
    history.commit("The last commit\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("{failed} failed: its tree stops at phase 1")),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {next}:"))
                && l.contains("issue@tiny@a-closable-issue")),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {next}:"))
                && l.contains("path@elsewhere@README.md")),
        "{stdout}"
    );
}

/// The claim: where HEAD sits changes nothing. A HEAD checked out inside the range is judged
/// like any other commit: its failing tree is a finding naming it, and exit 1.
#[test]
fn a_head_inside_the_range_is_judged_like_any_other_commit() {
    let history = History::new("commit-head-inside");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/note.md",
        "# A note\n\nIt names `design@tiny@no-such-decision`.\n",
    );
    let planted = history.commit("A note is added\n");
    history.remove("docs/note.md");
    let last = history.commit("The note is removed\n");
    history.git(&["checkout", "-q", "--detach", &planted]);
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..{last}")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with(&format!("commit {planted}: docs/note.md"))),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("{last} passed")), "{stdout}");
    let verdict = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(verdict.starts_with("FAILED"), "{verdict}");
}

/// The claim: a last commit whose tree cannot be assembled at all is a finding of the run, like
/// any other commit's, and the commits before it are still printed.
#[test]
fn a_last_commit_whose_manifest_does_not_load_is_a_finding_of_the_run() {
    let history = History::new("commit-last-unloadable");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let good = history.commit("A subject line\n\nIt records `design@tiny@tiny-anchor`.\n");
    tiny_project(&history, true);
    let bad = history.commit("The manifest goes back to a shape this tool refuses\n");
    // The working tree gets a manifest again: the binary locates its project by reading one,
    // so a tree whose manifest does not load refuses before any range is walked. What is under
    // test is the last commit's tree, which keeps it.
    tiny_project(&history, false);

    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("{good} passed")), "{stdout}");
    assert!(
        stdout.contains(&format!("{bad} failed: its tree does not load")),
        "{stdout}"
    );
    let last = stdout.trim_end().lines().last().expect("a verdict line");
    assert!(last.starts_with("FAILED"), "{last}");
}

/// The claim: a `#` line a commit actually holds is judged like any other line.
///
/// Git's default cleanup for `-m` is whitespace-only, so such a line reaches the commit.
/// `commits` reads the message the commit holds and cleans nothing, or a `CR:` marker on such a
/// line would leave the regime in silence.
#[test]
fn a_hash_line_a_commit_holds_is_judged_and_not_cleaned_away() {
    let history = History::new("commit-hash-line");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let sha = history.commit("A subject line\n\n# A line naming `design@tiny@no-such-thing`.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains(&format!("commit {sha}:3")), "{stdout}");
    assert!(stdout.contains("no-such-thing"), "{stdout}");
}

/// The claim: a project vendored under its repository is judged from its own paths.
///
/// Every git question a per-commit read asks is answered relative to the working directory —
/// `ls-tree` is scoped to it and names entries relative to it, and a blob is asked for as
/// `<sha>:./<path>`. Asked repository-relative instead, the listing carries a prefix the
/// manifest never declares, so every document falls out of the walk and every reference in a
/// message dangles while the working tree's own `check` passes.
#[test]
fn a_project_vendored_under_its_repository_is_judged_from_its_own_paths() {
    let history = History::at("commit-nested", "vendored");
    tiny_project(&history, false);
    // A file at the repository root, outside the project: nothing about it may reach the walk.
    std::fs::write(history.repo.join("outside.md"), "# Outside\n").expect("a file outside");
    let base = history.commit("The project is created\n");
    let sha = history.commit("A subject line\n\nIt records `design@tiny@tiny-anchor`.\n");

    let (clean, stderr, code) = history.run(&["check"]);
    assert_eq!(code, 0, "the working tree is clean: {clean}{stderr}");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "and so is the commit's tree: {stdout}{stderr}");
    assert!(stdout.contains(&format!("{sha} passed")), "{stdout}");
    assert!(
        !stdout.contains("its own tree fails"),
        "the listing carried no repository prefix: {stdout}"
    );
}

#[test]
fn an_empty_range_is_a_pass_that_says_it_judged_nothing() {
    let history = History::new("commit-empty");
    tiny_project(&history, false);
    history.commit("The project is created\n");
    let (stdout, stderr, code) = history.run(&["commits", "HEAD..HEAD"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("no commit is in range"), "{stdout}");
    assert!(stdout.contains("PASSED: no findings"), "{stdout}");
}

#[test]
fn a_range_that_does_not_resolve_could_not_run() {
    let history = History::new("commit-bad-range");
    tiny_project(&history, false);
    history.commit("The project is created\n");
    let (stdout, stderr, code) = history.run(&["commits", "origin/nowhere..HEAD"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("does not resolve"), "{stderr}");
}

/// A reader that closes the pipe early ends the run quietly with exit 2, never with a panic
/// on stderr: `cargo klarch model | head` is how a citation is located, and the panic the
/// standard print macros raise on a broken pipe made that idiom print a backtrace hint.
#[test]
fn a_closed_stdout_ends_the_run_quietly_with_exit_2() {
    let (reader, writer) = std::io::pipe().expect("a pipe");
    drop(reader);
    let out = Command::new(klarch())
        .args(["model"])
        .current_dir(project("planted"))
        .stdout(writer)
        .output()
        .expect("the binary runs");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "stderr: {stderr}");
    assert!(
        !stderr.contains("panicked") && !stderr.contains("Broken pipe"),
        "the closed pipe must not be reported: {stderr}"
    );
}

/// The claim: a command of `query`, which holds a copy of the writer, ends the same way, per
/// `design@cli@exit-code-ladder`. Mutation checked: exit 0 in that copy.
#[test]
fn a_closed_stdout_ends_a_command_over_the_record_with_exit_2() {
    let (reader, writer) = std::io::pipe().expect("a pipe");
    drop(reader);
    let out = Command::new(klarch())
        .args(["anchors"])
        .current_dir(project("planted"))
        .stdout(writer)
        .output()
        .expect("the binary runs");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "stderr: {stderr}");
    assert!(
        stderr.is_empty(),
        "the closed pipe is not reported: {stderr}"
    );
}

/// The claim: `transcript`, whose crate holds a copy of the writer, ends the same way, per
/// `design@cli@exit-code-ladder`. Mutation checked: exit 0 in that copy.
#[test]
fn a_closed_stdout_ends_transcript_with_exit_2() {
    let config = transcript_case("kinds", "closed-stdout");
    let (reader, writer) = std::io::pipe().expect("a pipe");
    drop(reader);
    let out = Command::new(klarch())
        .args(["transcript", "list", "--all-projects"])
        .current_dir(project("core"))
        .env("CLAUDE_CONFIG_DIR", &config)
        .stdout(writer)
        .output()
        .expect("the binary runs");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(2), "stderr: {stderr}");
    assert!(
        stderr.is_empty(),
        "the closed pipe is not reported: {stderr}"
    );
}

/// The claim: a file in the installer's namespace that this version does not ship fails the check
/// in the last phase, and is not walked as a document; `install-agent-skills` removes it and leaves a
/// project's own skill, and the check then passes. Per `design@check@owned-namespace-check` and
/// `design@check@installed-files-compared-by-bytes`.
/// Mutations: dropping the namespace from the walk's exclusions walks the stray file as a
/// document; dropping the unshipped branch of the installed-file check lets the first run pass.
#[test]
fn a_stray_file_in_the_installers_namespace_fails_the_check_and_the_install_removes_it() {
    let sandbox = Sandbox::seeded(
        "stray-installed",
        "minimal",
        &[
            (
                ".claude/agents/knowledge-architect-stray.md",
                "a reference `design@nowhere@nothing` that is read by nothing\n",
            ),
            (".claude/skills/project-own/SKILL.md", "# Project skill\n"),
        ],
    );
    sandbox.serve_claude();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    // The stray file, and each shipped file the copy does not hold yet.
    let expected = format!(
        "FAILED: {} findings above",
        knowledge_architect_agent_skills::FILES.len() + 1
    );
    assert!(out.contains(&expected), "{out}");
    assert!(!out.contains("phase 2:"), "{out}");
    assert!(
        out.contains(".claude/agents/knowledge-architect-stray.md  this file sits in the installer's namespace"),
        "{out}"
    );
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.contains("deleted  .claude/agents/knowledge-architect-stray.md"),
        "{out}"
    );
    assert!(!sandbox
        .path(".claude/agents/knowledge-architect-stray.md")
        .exists());
    assert!(sandbox.path(".claude/skills/project-own/SKILL.md").exists());
    // git still lists the deleted file until the deletion is staged, and the check says so.
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("the deletion is not staged"), "{out}");
    assert!(out.contains("→ stage the deletion"), "{out}");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// The claim: `commits` does not compare a commit's installed files, so a commit holding a file of
/// the installer's namespace that the running version does not ship is judged on its message and
/// its documents alone, while `check` on the working tree still reports it. Per
/// `design@check@only-check-compares-installed-files`. Mutation: giving the per-commit inputs the installed set
/// fails the range.
#[test]
fn commits_does_not_compare_installed_files_and_check_does() {
    let history = History::new("commit-installed");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    // The harness is declared after the base commit: the range is judged with nothing of the
    // shipped set installed, which `check` reports and `commits` does not.
    let manifest = std::fs::read_to_string(history.dir.join("knowledge-architect.toml"))
        .expect("the manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest.replace("harness = []", "harness = [\"claude\"]"),
    );
    history.write(
        ".claude/agents/knowledge-architect-older.md",
        "an installed file of another version\n",
    );
    history.commit("An installed file of another version is committed\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let (stdout, _, code) = history.run(&["check"]);
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.contains("does not ship it"), "{stdout}");
}

/// The claim: under `harness = []` a component owes no CLAUDE.md, the namespace is walked like any
/// other path, and the install writes and deletes nothing. Mutations: requiring the compiled
/// document set regardless of the harness, or running the install without its harness guard,
/// each fail a case.
#[test]
fn no_harness_owes_no_claude_md_and_installs_nothing() {
    let sandbox = Sandbox::seeded(
        "no-harness",
        "minimal",
        &[(
            ".claude/agents/knowledge-architect-kept.md",
            "# A file the project keeps\n",
        )],
    );
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    assert!(manifest.contains("[agents]\nharness = []\n"), "{manifest}");
    std::fs::remove_file(sandbox.path("CLAUDE.md")).expect("the root CLAUDE.md is removed");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(out.contains("nothing was installed"), "{out}");
    assert!(sandbox
        .path(".claude/agents/knowledge-architect-kept.md")
        .exists());
}

/// The claim: the binary hands the check and the install the files this version ships. Before
/// the install, the check reports each one missing; the install writes each, rendered with the
/// project's command, here the default; then the check passes. Per
/// `design@check@installed-files-compared-by-bytes`. Rendering with a declared command is a unit test of
/// `agents`. Mutations: handing `Gathered` an empty set instead of
/// `agents::shipped` lets the first check pass; handing the install an empty set writes nothing.
#[test]
fn the_check_and_the_install_receive_the_shipped_set() {
    let files = knowledge_architect_agent_skills::FILES;
    assert!(
        !files.is_empty(),
        "this version ships no file, so the test proves nothing"
    );
    let sandbox = Sandbox::new("shipped-set", "minimal");
    sandbox.serve_claude();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.contains(&format!("FAILED: {} findings above", files.len())),
        "{out}"
    );
    for (rel, _) in files {
        assert!(out.contains(rel), "{rel} is not reported missing:\n{out}");
    }
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    for (rel, template) in files {
        assert!(out.contains(&format!("wrote    {rel}")), "{out}");
        let installed = std::fs::read_to_string(sandbox.path(rel)).expect("an installed file");
        assert_eq!(
            installed,
            template
                .replace("{{command}}", "klarch")
                .replace("\r\n", "\n"),
            "{rel}"
        );
    }
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// Every file under `dir` with its bytes, `.git` left out.
fn tree_bytes(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    let mut out = std::collections::BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).expect("a readable directory") {
            let path = entry.expect("a directory entry").path();
            if path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
            } else {
                let bytes = std::fs::read(&path).expect("a readable file");
                out.insert(
                    path.strip_prefix(dir)
                        .expect("under the copy")
                        .to_path_buf(),
                    bytes,
                );
            }
        }
    }
    out
}

/// The claim: a writer refuses over an incomplete model. `index` over `unsound`, which stops at
/// phase 2, exits 2, says why, and writes nothing.
#[test]
fn index_over_an_incomplete_model_refuses_and_writes_nothing() {
    let sandbox = Sandbox::new("index-refuses", "unsound");
    let before = tree_bytes(&sandbox.dir);
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 2, "stdout:\n{out}\nstderr:\n{err}");
    assert!(err.contains("the model is incomplete"), "{err}");
    assert!(err.contains("Nothing was written."), "{err}");
    assert!(tree_bytes(&sandbox.dir) == before, "index changed the tree");
}

/// The claim: `--version` answers from any directory, project or not, with the version of the
/// package the binary was built from, so a session can ask which checker ran rather than read the
/// project's pin. Shown to fail before the flag existed: clap refused the argument and exited 2.
#[test]
fn the_binary_reports_its_version_from_anywhere() {
    let (out, err, code) = run_in(&std::env::temp_dir(), &["--version"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(
        out.trim(),
        format!("klarch {}", env!("CARGO_PKG_VERSION")),
        "{out}"
    );
}

/// The issue the `--fix` tests retitle, and the index that lists it.
const FIX_ISSUE: &str = "docs/open-issues/the-mock-has-one-issue.md";
const FIX_INDEX: &str = "docs/open-issues/index.md";

/// A copy of `minimal` whose issue is retitled, so its index is stale.
fn stale_index(tag: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "minimal");
    let issue = std::fs::read_to_string(sandbox.path(FIX_ISSUE)).expect("the mock's issue");
    sandbox.write(
        FIX_ISSUE,
        &issue.replace(
            "# The mock has one issue so the register is not empty",
            "# A retitled issue",
        ),
    );
    sandbox.stage();
    sandbox
}

/// The claim: `check --fix` rewrites a stale generated index, lists it, and the run then passes,
/// so an edit cycle is one command. Mutation checked: skipping the write of the generated list
/// leaves the index stale, and the run exits 1.
///
/// The claim: the stale index's finding names the repair and no cause, per
/// `design@check@finding-names-the-repair`. The action line is asserted whole, its line
/// break included, so a cause appended to it fails the test.
#[test]
fn check_fix_rewrites_a_stale_index_and_the_run_passes() {
    let sandbox = stale_index("fix-stale");
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("the generated file is out of date\n    → run `klarch index`\n"),
        "{out}"
    );
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: wrote {FIX_INDEX} (regenerated)")),
        "{out}"
    );
    assert!(out.trim_end().ends_with("PASSED: no findings"), "{out}");
    let index = std::fs::read_to_string(sandbox.path(FIX_INDEX)).expect("the index");
    assert!(index.contains("A retitled issue"), "{index}");
}

/// The claim: the stale index's action names the command the project declares, not the
/// default one, so a project that runs the checker through its own command is told that one.
#[test]
fn the_stale_index_action_names_the_declared_command() {
    let sandbox = stale_index("stale-command");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "name = \"minimal\"\n",
            "name = \"minimal\"\ncommand = \"probe klarch\"\n",
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains("the generated file is out of date\n    → run `probe klarch index`\n"),
        "{out}"
    );
}

/// The claim: a finding of phases 1 to 3 stops a `--fix` run before any generated file is
/// written, so no writer writes over an incomplete model. Mutation checked: removing the gate's
/// stop writes the index over the incomplete model.
#[test]
fn check_fix_writes_no_generated_file_over_an_incomplete_model() {
    let sandbox = stale_index("fix-incomplete");
    std::fs::remove_file(sandbox.path("docs/rejected.md")).expect("a register home is removed");
    sandbox.stage();
    let before = std::fs::read(sandbox.path(FIX_INDEX)).expect("the index");
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("phase 2:"), "{out}");
    assert!(!out.contains("fixed:"), "{out}");
    assert_eq!(
        std::fs::read(sandbox.path(FIX_INDEX)).expect("the index"),
        before
    );
}

/// The claim: with nothing to fix, `check --fix` prints exactly what `check` prints and exits as
/// it does. Mutation checked: an install run whenever the project serves a harness is still
/// silent here, but printing a header before the report fails the comparison.
#[test]
fn check_fix_with_nothing_to_fix_is_a_plain_check() {
    let sandbox = Sandbox::new("fix-nothing", "minimal");
    let plain = sandbox.run(&["check"]);
    let fixed = sandbox.run(&["check", "--fix"]);
    assert_eq!(plain.2, 0, "{}{}", plain.0, plain.1);
    assert_eq!(fixed, plain);
}

/// The claim: `check --fix` installs an installed file that differs from the shipped text, lists
/// it, and the run passes. Mutation checked: skipping the install when it is needed leaves the
/// finding, and the run exits 1.
#[test]
fn check_fix_installs_a_differing_installed_file() {
    let sandbox = Sandbox::new("fix-installed", "minimal");
    sandbox.serve_claude();
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    let (rel, _) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    sandbox.write(rel, "an edit the install overwrites\n");
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: wrote {rel} (installed)")),
        "{out}"
    );
}

/// The claim: `check --fix` repairs the installed files before the gate of phases 1 to 3, per
/// `design@check@fix-before-the-checks`: a tree that stops at phase 2 still has them repaired, since
/// their bytes do not depend on the model, and the run reports the stop. Mutation checked: running
/// the repair after the gate leaves the edit in place.
#[test]
fn check_fix_repairs_the_installed_files_before_the_gate() {
    let sandbox = harnessed("fix-before-the-gate");
    let (rel, shipped) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    sandbox.write(rel, "an edit\n");
    std::fs::remove_file(sandbox.path("README.md")).expect("a required document");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("phase 2:"), "{out}");
    assert!(
        out.contains(&format!("fixed: wrote {rel} (installed)")),
        "{out}"
    );
    assert_eq!(
        std::fs::read_to_string(sandbox.path(rel)).expect("the installed file"),
        shipped
            .replace("{{command}}", "klarch")
            .replace("\r\n", "\n")
    );
}

/// A note citing what no entry defines, so a tree has a finding of the reference check.
const UNRESOLVED_NOTE: &str = "A reference to `issue@minimal@no-such-entry`.\n";

/// The claim: an installed file that differs is a finding of the last phase, listed with a
/// reference finding of the same tree. Mutation checked: `foundation` of the checks stopping on the
/// family's findings lists the install finding alone, in phase 2.
#[test]
fn a_differing_installed_file_is_listed_beside_a_reference_finding() {
    let sandbox = harnessed("installed-differs");
    let (rel, _) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    sandbox.write(rel, "an edit\n");
    sandbox.write("docs/note.md", UNRESOLVED_NOTE);
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(!out.contains("phase 2:"), "{out}");
    assert!(
        out.contains(&format!("{rel}  this installed file differs")),
        "{out}"
    );
    assert!(out.contains("no-such-entry"), "{out}");
    assert!(out.contains("FAILED: 2 findings above"), "{out}");
    assert!(
        out.find("this installed file differs") < out.find("no-such-entry"),
        "the install finding is listed first: {out}"
    );
}

/// The claim: a shipped file deleted with the deletion staged is a finding of the last phase, and a
/// reference to the skill it defined is reported beside it as unresolved, since the model is built
/// over the tree as it is, and names the install as its repair. Mutations checked: `foundation` of
/// the checks stopping on the family's findings lists the install finding alone, in phase 2; the
/// repair text with no case for the namespace fails the last assertion.
#[test]
fn a_missing_shipped_file_is_listed_beside_the_references_to_what_it_defined() {
    let sandbox = harnessed("installed-missing");
    let (rel, name) = knowledge_architect_agent_skills::FILES
        .iter()
        .find_map(|(path, _)| {
            let name = path
                .strip_prefix(".claude/skills/")?
                .strip_suffix("/SKILL.md")?;
            Some((*path, name))
        })
        .expect("this version ships a skill");
    sandbox.write("docs/note.md", &format!("See `skill@{name}`.\n"));
    std::fs::remove_file(sandbox.path(rel)).expect("the installed skill");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(!out.contains("phase 2:"), "{out}");
    assert!(
        out.contains(&format!("{rel}  ")) && out.contains("no installed copy"),
        "{out}"
    );
    assert!(
        out.lines()
            .any(|l| l.starts_with("docs/note.md") && l.contains(name)),
        "{out}"
    );
    assert!(
        out.contains("run `install-agent-skills` if the installed skill is missing"),
        "{out}"
    );
}

/// The claim: `commits` reports a commit's tree whose row keeps the root CLAUDE.md out under the
/// `claude` harness, as `check` does, in phase 2. Mutation checked: removing the finding from
/// `tree::check` passes the range.
#[test]
fn commits_reports_a_tree_whose_row_keeps_the_root_claude_md_out() {
    let history = History::new("commit-row-claude");
    tiny_project(&history, false);
    let base = history.commit("The project is created\n");
    let manifest = std::fs::read_to_string(history.dir.join("knowledge-architect.toml"))
        .expect("the manifest");
    history.write(
        "knowledge-architect.toml",
        &manifest
            .replace("harness = []", "harness = [\"claude\"]")
            .replace("skip-files = []", "skip-files = [\"CLAUDE.md\"]"),
    );
    history.write("CLAUDE.md", "# A project\n");
    history.commit("A row keeps the root document out\n");
    let (stdout, _, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.contains("its tree stops at phase 2"), "{stdout}");
}

/// The claim: `index` writes over a stale install, since the family is no part of the gate of phases
/// 1 to 3. Mutation checked: the foundation of the gate stopping on the family exits 2 and writes
/// nothing.
#[test]
fn index_writes_over_a_stale_install() {
    let sandbox = harnessed("index-stale-install");
    let (rel, _) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    sandbox.write(rel, "an edit\n");
    let issue = std::fs::read_to_string(sandbox.path(FIX_ISSUE)).expect("the mock's issue");
    sandbox.write(
        FIX_ISSUE,
        &issue.replace(
            "# The mock has one issue so the register is not empty",
            "# A retitled issue",
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{err}");
    let index = std::fs::read_to_string(sandbox.path("docs/open-issues/index.md"))
        .expect("the register's index");
    assert!(index.contains("A retitled issue"), "{index}");
}

/// The claim: `index --staged` writes into git's index over a stale install staged. Mutation
/// checked: the foundation of `commit_tree` stopping on the family exits 2 and stages nothing.
#[test]
fn index_staged_writes_over_a_stale_install() {
    let sandbox = harnessed("index-staged-stale-install");
    let (rel, _) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    sandbox.write(rel, "an edit\n");
    let issue = std::fs::read_to_string(sandbox.path(FIX_ISSUE)).expect("the mock's issue");
    sandbox.write(
        FIX_ISSUE,
        &issue.replace(
            "# The mock has one issue so the register is not empty",
            "# A retitled issue",
        ),
    );
    sandbox.stage();
    let before = index_lines(&sandbox);
    let (out, err, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{err}");
    let changed: Vec<String> = changed_lines(&before, &index_lines(&sandbox))
        .into_iter()
        .map(|l| l.split('\t').nth(1).unwrap_or_default().to_string())
        .collect();
    assert_eq!(changed, vec!["docs/open-issues/index.md".to_string()]);
}

/// The claim: `--fix` belongs to `check` alone; `commits` judges history and refuses it, which
/// clap answers with exit 2.
#[test]
fn commits_takes_no_fix() {
    let (_, err, code) = run("minimal", &["commits", "--fix", "HEAD"]);
    assert_eq!(code, 2, "{err}");
}

/// A copy of `minimal` serving the `claude` harness, its shipped set installed and staged: a tree
/// the check passes.
fn harnessed(tag: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "minimal");
    sandbox.serve_claude();
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    sandbox.stage();
    sandbox
}

/// The claim: on a harnessed tree the check passes, `check --fix` touches nothing and prints what
/// `check` prints. Mutation checked: putting every listed shipped file in the repairs, whatever
/// its bytes, prints `fixed:` lines and fails the comparison.
#[test]
fn check_fix_on_a_clean_harnessed_tree_is_a_plain_check() {
    let sandbox = harnessed("fix-harnessed");
    let before = tree_bytes(&sandbox.path(".claude"));
    let plain = sandbox.run(&["check"]);
    assert_eq!(plain.2, 0, "{}{}", plain.0, plain.1);
    assert_eq!(sandbox.run(&["check", "--fix"]), plain);
    assert_eq!(tree_bytes(&sandbox.path(".claude")), before);
}

/// The claim: `check --fix` never deletes a file of the installer's namespace that git does not
/// list, such as an ignored editor swap file the check never reports. Mutation checked: judging
/// the namespace from the filesystem, as the install does, deletes it.
#[test]
fn check_fix_never_deletes_an_ignored_file_in_the_namespace() {
    let sandbox = harnessed("fix-ignored");
    sandbox.write(".gitignore", "*.swp\n");
    let swap = ".claude/skills/knowledge-architect-design/.SKILL.md.swp";
    sandbox.write(swap, "my unsaved edits\n");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains("fixed:"), "{out}");
    assert!(sandbox.path(swap).exists());
}

/// The claim: `check --fix` deletes an unshipped file git lists in the namespace and lists the
/// removal; the deletion is unstaged, so the check reports it in the last phase, which the run
/// reaches, so the stale generated file is written in that same run; after `git add` a second run
/// passes. Mutations checked: dropping the removal line fails it; stopping `foundation` on the
/// family's findings leaves the index unwritten. Not counting a removal as a write survives here:
/// it changes the exit code only when a later write fails, which this case does not reach.
#[test]
fn check_fix_removes_an_unshipped_file_and_writes_the_generated_files_in_that_run() {
    let sandbox = harnessed("fix-removed");
    let stray = ".claude/agents/knowledge-architect-retired.md";
    sandbox.write(stray, "a file an older version shipped\n");
    let issue = std::fs::read_to_string(sandbox.path(FIX_ISSUE)).expect("the mock's issue");
    sandbox.write(
        FIX_ISSUE,
        &issue.replace(
            "# The mock has one issue so the register is not empty",
            "# A retitled issue",
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: removed {stray} (installed)")),
        "{out}"
    );
    assert!(out.contains("the deletion is not staged"), "{out}");
    assert!(out.contains("(regenerated)"), "{out}");
    assert!(!out.contains("phase 2:"), "{out}");
    assert!(!sandbox.path(stray).exists());
    let index = std::fs::read_to_string(sandbox.path("docs/open-issues/index.md"))
        .expect("the register's index");
    assert!(index.contains("A retitled issue"), "{index}");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: when a write fails after another file was written, `check --fix` lists what it
/// wrote and exits 1, since 2 would promise an untouched tree. The write fails because a file
/// stands where the last shipped file's directory goes. Mutations checked: exiting 2 whatever was
/// written, or dropping the error, each fail it.
#[test]
fn check_fix_exits_one_when_a_write_fails_after_a_write() {
    let sandbox = Sandbox::new("fix-partway", "minimal");
    sandbox.serve_claude();
    let (last, _) = knowledge_architect_agent_skills::FILES
        .last()
        .expect("this version ships a file");
    let parent = Path::new(last)
        .parent()
        .and_then(Path::to_str)
        .expect("the last shipped file sits in a directory");
    sandbox.write(parent, "a file where a directory goes\n");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("fixed: wrote"), "{out}");
    assert!(err.contains(parent), "{err}");
}

/// The claim: a directory at the path of a shipped file is left alone by `check --fix` and named
/// with its repair, whether git lists the path or not: the run exits 1, writes nothing over it, and
/// the finding says a directory stands there. Mutations checked: the unlisted arm of `repairs`
/// writing over a directory exits 2 with an OS error; the listed-directory arm of the check
/// reporting the file as differing loses the directory from the message.
#[test]
fn check_fix_leaves_a_directory_at_a_shipped_path_and_names_it() {
    let (rel, _) = knowledge_architect_agent_skills::FILES
        .first()
        .expect("this version ships a file");
    for listed in [false, true] {
        let sandbox = harnessed(if listed {
            "fix-dir-listed"
        } else {
            "fix-dir-unlisted"
        });
        if listed {
            std::fs::remove_file(sandbox.path(rel)).expect("the installed file");
        } else {
            sandbox.git(&["rm", "-q", "--cached", rel]);
            std::fs::remove_file(sandbox.path(rel)).expect("the installed file");
        }
        std::fs::create_dir_all(sandbox.path(rel)).expect("a directory in the file's place");
        let (out, err, code) = sandbox.run(&["check", "--fix"]);
        assert_eq!(code, 1, "listed {listed}: {out}{err}");
        assert!(!out.contains(&format!("fixed: wrote {rel}")), "{out}");
        assert!(out.contains("directory"), "listed {listed}: {out}");
        assert!(sandbox.path(rel).is_dir());
    }
}

/// The claim: a generated destination that is a symlink is refused before anything is written,
/// and with nothing written the run exits 2. Mutations checked: dropping the symlink refusal
/// writes through the link, outside the project; exiting 1 whatever was written fails the code.
#[test]
fn check_fix_refuses_a_symlinked_destination_and_writes_nothing() {
    let sandbox = stale_index("fix-symlink");
    let outside = std::env::temp_dir().join(format!("ka-fix-outside-{}", std::process::id()));
    std::fs::write(&outside, "outside the project\n").expect("a file outside the project");
    std::fs::remove_file(sandbox.path(FIX_INDEX)).expect("the index is removed");
    std::os::unix::fs::symlink(&outside, sandbox.path(FIX_INDEX)).expect("a symlinked index");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            &format!("skip-files = [\"notes/generated.md\", \"{FIX_INDEX}\"]"),
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(err.contains("symlink"), "{err}");
    assert_eq!(
        std::fs::read_to_string(&outside).expect("the outside file"),
        "outside the project\n"
    );
    let _ = std::fs::remove_file(&outside);
}

/// The claim: a manifest holding a refused declaration makes `check --fix` write nothing, not even
/// the agent files, since the shipped text is rendered with the declared command. Mutation
/// checked: removing the early return installs the files.
#[test]
fn check_fix_over_a_refused_manifest_writes_nothing() {
    let sandbox = Sandbox::new("fix-refused", "minimal");
    sandbox.serve_claude();
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &format!("{manifest}\n[bogus]\nkey = 1\n"),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(!out.contains("fixed:"), "{out}");
    assert!(!sandbox
        .path(".claude/knowledge-architect/PRIMER.md")
        .exists());
}

/// The claim: a finding in a file git does not track is followed by one note naming the file
/// as untracked, above the verdict; a finding in a tracked file brings no note.
#[test]
fn a_finding_in_an_untracked_file_is_followed_by_a_note_naming_it() {
    // Each finding is an unanchored path into `docs/`, a directory the mock holds.
    let sandbox = Sandbox::seeded("untracked-note", "core", &[("tracked.md", "`docs/a`\n")]);
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("tracked.md:1"), "{out}");
    assert!(!out.contains("untracked:"), "{out}");

    // Written after the copy was staged, so git does not track it.
    sandbox.write("scratch.md", "`docs/c`\n");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("scratch.md:1"), "{out}");
    let lines: Vec<&str> = out.lines().collect();
    let note = lines
        .iter()
        .position(|l| l.contains("untracked:"))
        .expect("a note naming the untracked file");
    assert_eq!(
        lines[note], "1 file above is untracked: scratch.md",
        "{out}"
    );
    assert!(
        lines[note + 1].contains("commit a file that belongs to the project"),
        "{out}"
    );
    assert!(
        lines.last().expect("a verdict").starts_with("FAILED:"),
        "{out}"
    );

    // Two findings in one file name it once, and a second file makes the count plural.
    sandbox.write("scratch.md", "`docs/c`\n`docs/e`\n");
    sandbox.write("other.md", "`docs/g`\n");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l == "2 files above are untracked: other.md, scratch.md"),
        "{out}"
    );
}

/// The claim: a finding with no line, from a run stopped at phase 2, is noted like any other,
/// and a line break in the untracked name is escaped, so the note stays one line.
#[test]
fn the_untracked_note_covers_a_phase_two_stop_and_escapes_the_name() {
    let sandbox = Sandbox::new("untracked-note-phase-two", "core");
    sandbox.write("nl\nname.md", "text\n");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("phase 2:"), "{out}");
    assert!(
        out.lines()
            .any(|l| l == "1 file above is untracked: nl\\nname.md"),
        "{out}"
    );
}

/// The claim: an untracked manifest brings no note, since it is read whatever git says of it
/// and neither moving it out nor ignoring it is a way out.
#[test]
fn an_untracked_manifest_brings_no_untracked_note() {
    let sandbox = Sandbox::new("untracked-note-manifest", "core");
    sandbox.git(&["rm", "-q", "--cached", "knowledge-architect.toml"]);
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &format!("{manifest}\n[bogus]\n"),
    );
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(out.contains("knowledge-architect.toml"), "{out}");
    assert!(!out.contains("untracked:"), "{out}");
}

/// Write a copy's manifest with `[project] checker-version` set to `value` exactly, bypassing
/// the pinning every other write of a manifest goes through.
fn pin_copy_to(sandbox: &Sandbox, value: &str) {
    let path = sandbox.path("knowledge-architect.toml");
    let text = std::fs::read_to_string(&path).expect("the manifest");
    let line = format!("checker-version = \"{}\"", env!("CARGO_PKG_VERSION"));
    assert_eq!(text.matches(&line).count(), 1, "{text}");
    let replaced = if value.is_empty() {
        text.replace(&format!("{line}\n"), "")
    } else {
        text.replace(&line, &format!("checker-version = \"{value}\""))
    };
    std::fs::write(&path, replaced).expect("the manifest is rewritten");
}

/// The claim: over a copy of a mock, a pin the binary does not satisfy refuses the run with exit
/// 2 and a message opening with the case: the binary older or newer than the pin, `"fixture"`
/// outside a library, `"self"` where the checker is not built; the pin equal to the binary's
/// version passes. Per `design@check@installed-binary-version-check`.
#[test]
fn a_copy_whose_pin_this_binary_does_not_satisfy_refuses_the_run() {
    let version = env!("CARGO_PKG_VERSION");
    let sandbox = Sandbox::new("pin-refusal", "minimal");
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "the pinned copy passes: {out}{err}");
    for (value, opening) in [
        (
            "99.0.0".to_string(),
            format!("this binary runs knowledge-architect {version}, older than the 99.0.0 the manifest pins"),
        ),
        (
            "0.0.1".to_string(),
            format!("this binary runs knowledge-architect {version}, newer than the 0.0.1 the manifest pins"),
        ),
        (
            "fixture".to_string(),
            "\"fixture\" is only valid for a mock project inside a library this binary links".to_string(),
        ),
        (
            "self".to_string(),
            "\"self\" is only valid where the checker is built from this tree".to_string(),
        ),
    ] {
        let copy = Sandbox::new(&format!("pin-refusal-{value}"), "minimal");
        pin_copy_to(&copy, &value);
        let (out, err, code) = copy.run(&["check"]);
        assert_eq!(code, 2, "{value}: {out}{err}");
        assert!(err.contains(&opening), "{value}: {err}");
    }
}

/// The claim: with no key, every command refuses with the key's complaint, a command that reads
/// nothing included.
#[test]
fn a_manifest_without_the_key_refuses_every_command() {
    let sandbox = Sandbox::new("pin-absent", "minimal");
    pin_copy_to(&sandbox, "");
    for args in [
        &["check"][..],
        &["show", "design@minimal@nothing"],
        &["model"],
    ] {
        let (out, err, code) = sandbox.run(args);
        assert_eq!(code, 2, "{args:?}: {out}{err}");
        assert!(
            err.contains("[project] checker-version is absent"),
            "{args:?}: {err}"
        );
    }
}

/// The claim: the install over a project pinning another version refuses, and writes nothing.
#[test]
fn the_install_refuses_another_version_and_writes_nothing() {
    let sandbox = Sandbox::new("pin-install", "minimal");
    sandbox.serve_claude();
    pin_copy_to(&sandbox, "0.0.1");
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 2, "{out}{err}");
    assert!(
        err.contains("newer than the 0.0.1 the manifest pins"),
        "{err}"
    );
    assert!(!sandbox.path(".claude").exists(), "nothing was installed");
}

/// The claim: `commits` judges a range whose earlier commit pins another version without
/// comparing that historical pin, so moving the pin fails no earlier commit.
#[test]
fn commits_compares_no_historical_pin() {
    let history = History::new("pin-history");
    tiny_project(&history, false);
    // The base is pinned to this binary, so `commit` regenerates its indexes; the next commit
    // changes the pin alone, which `commit` cannot regenerate over, and needs not.
    let base = history.commit("The project is created\n");
    let manifest = history.dir.join("knowledge-architect.toml");
    let pinned_text = std::fs::read_to_string(&manifest).expect("the manifest");
    let line = format!("checker-version = \"{}\"", env!("CARGO_PKG_VERSION"));
    std::fs::write(
        &manifest,
        pinned_text.replace(&line, "checker-version = \"0.0.1\""),
    )
    .expect("an older pin");
    history.commit("A subject line\n\nThe project pins an older checker.\n");
    std::fs::write(&manifest, &pinned_text).expect("the pin moved");
    history.commit("The pin moves\n\nThe project now pins this checker.\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("2 commits, 2 passed, 0 failed"), "{stdout}");
}

/// The claim: the foreign-build refusal comes before the pin's, so over a second checkout whose
/// manifest pins another version, the message names the checkout, not the version.
#[test]
fn a_foreign_build_is_refused_before_its_pin_is_read() {
    let sandbox = Sandbox::seeded(
        "foreign-before-pin",
        "minimal",
        &[(
            "crates/cli/Cargo.toml",
            "[package]\nname = \"knowledge-architect\"\n",
        )],
    );
    pin_copy_to(&sandbox, "0.0.1");
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(stderr.contains("built from another checkout"), "{stderr}");
    assert!(!stderr.contains("the manifest pins"), "{stderr}");
}

/// `tiny_project` serving the `claude` harness: the manifest declares it, the root CLAUDE.md
/// imports the primer, and the shipped set is installed.
fn tiny_project_serving_claude(history: &History) {
    tiny_project(history, false);
    history.write(
        "knowledge-architect.toml",
        "[project]\nname = \"tiny\"\ncomponents = []\n\n\
         [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n\n\
         [agents]\nharness = [\"claude\"]\n",
    );
    history.write(
        "CLAUDE.md",
        "@.claude/knowledge-architect/PRIMER.md\n\n# tiny\n\n\
         Nothing here holds of any code: this project has none.\n",
    );
    let (out, err, code) = history.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim, per `design@check@import-on-first-line`: `install-agent-skills` says the primer is
/// not imported exactly when the root CLAUDE.md's first line, read as it is on disk, is not the
/// import line; not over a file that cannot be read as text, which `check` reports with its own
/// cause. Mutations: the condition made `if false` fails the second and third cases; the text
/// trimmed at its start fails the third; a failed read taken as an empty file fails the last.
#[test]
fn the_install_names_an_import_line_off_the_first_line_and_nothing_else() {
    let history = History::new("install-import-line");
    tiny_project_serving_claude(&history);
    let said = "does not import the primer on its first line";
    let (out, err, code) = history.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains(said), "{out}");
    for text in [
        "# tiny\n\n@.claude/knowledge-architect/PRIMER.md\n",
        "    @.claude/knowledge-architect/PRIMER.md\n\n# tiny\n",
    ] {
        history.write("CLAUDE.md", text);
        let (out, err, code) = history.run(&["install-agent-skills"]);
        assert_eq!(code, 0, "{out}{err}");
        assert!(out.contains(said), "{text:?}: {out}");
    }
    history.write_bytes(
        "CLAUDE.md",
        b"@.claude/knowledge-architect/PRIMER.md\n\n# tiny\n\ncaf\xe9\n",
    );
    let (out, err, code) = history.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!out.contains(said), "{out}");
}

/// The claim, per `design@model@installed-entities-from-the-tree`: commit 1 cites a
/// section of an installed skill; commit 2 renames the section in the installed copy and repairs
/// the citation. `commits` judges each commit against its own installed copies, so both pass.
/// Mutation checked: `commit_tree` not handing the commit's installed blobs to the model makes
/// both commits' citations dangle.
#[test]
fn a_branch_that_renames_an_installed_section_passes_commits_on_every_commit() {
    let history = History::new("commits-own-copies");
    tiny_project_serving_claude(&history);
    let base = history.commit("The project is created\n");
    history.write(
        "docs/rejected.md",
        "# tiny — rejected designs\n\n\
         Nothing has lost yet; see `skill@knowledge-architect-review@review-axes`.\n",
    );
    history.commit("Cite a section of an installed skill\n");
    let skill = ".claude/skills/knowledge-architect-review/SKILL.md";
    let text = std::fs::read_to_string(history.dir.join(skill)).expect("the installed skill");
    assert_eq!(text.matches("`##review-axes`").count(), 1, "{text}");
    history.write(skill, &text.replace("`##review-axes`", "`##the-axes`"));
    history.write(
        "docs/rejected.md",
        "# tiny — rejected designs\n\n\
         Nothing has lost yet; see `skill@knowledge-architect-review@the-axes`.\n",
    );
    history.commit("Rename the section and repair its citation\n");
    let (out, err, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: `show` prints a skill whole and a section of it, and lists what cites each.
/// Mutation checked: the harness arm of `show` removed makes both resolve to nothing.
#[test]
fn show_prints_a_skill_and_one_of_its_sections() {
    let sandbox = harnessed("show-skill");
    sandbox.write(
        "README.md",
        &format!(
            "{}\nSee `skill@knowledge-architect-review@review-axes`.\n",
            std::fs::read_to_string(sandbox.path("README.md")).expect("the README")
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["show", "skill@knowledge-architect-review"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("name: knowledge-architect-review"), "{out}");
    let (out, err, code) = sandbox.run(&["show", "skill@knowledge-architect-review@review-axes"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("`##review-axes`"), "{out}");
    assert!(!out.contains("## 2."), "a section stops at the next: {out}");
    assert!(out.contains("README.md:"), "{out}");
}

/// The claim, per `design@model@section-homes-carry-slugs`: a project skill
/// with a level-two heading that carries no slug fails the check, and the finding names the
/// repair. Mutation checked: the finding's action replaced by an empty one fails the assertion.
#[test]
fn an_unslugged_heading_of_a_project_skill_is_reported_with_its_repair() {
    let sandbox = harnessed("unslugged-skill");
    sandbox.write(
        ".claude/skills/tiny-own/SKILL.md",
        "---\nname: tiny-own\ndescription: a project skill\n---\n# Own\n\n## A section\n",
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("the level-2 heading \"A section\""), "{out}");
    assert!(out.contains("end it with its slug"), "{out}");
}

/// The claim: a walk row that skips the installer's namespace leaves `check` and `commits` judging
/// one tree alike, both reading the installed copies for definitions. Mutation checked: `read_tree`
/// fetching no installed blob a walk row skips makes `commits` report the primer's section missing.
#[test]
fn a_skipped_installed_namespace_defines_alike_under_check_and_commits() {
    let history = History::new("skipped-installed");
    tiny_project_serving_claude(&history);
    history.write(
        "knowledge-architect.toml",
        "[project]\nname = \"tiny\"\ncomponents = []\n\n\
         [walk]\nskip-dirs = [\".claude/knowledge-architect\"]\nskip-files = []\nexclude = []\n\n\
         [agents]\nharness = [\"claude\"]\n",
    );
    let base = history.commit("The project is created\n");
    history.write(
        "docs/rejected.md",
        "# tiny — rejected designs\n\nNothing has lost yet; see `primer@room-to-judge`.\n",
    );
    history.commit("Cite a section of the primer\n");
    let (out, err, code) = history.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
    let (out, err, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: `show` prints a section of the primer, not the whole primer, and lists the
/// references to that section only, not to a slug of the same id under another owner. Mutation
/// checked: printing whole every entity with no owner prints the primer's next section; listing
/// inbound references by kind and id alone lists the other skill's citation.
#[test]
fn show_prints_one_primer_section_and_its_own_references() {
    let sandbox = harnessed("show-primer");
    sandbox.write(
        ".claude/skills/tiny-a/SKILL.md",
        "---\nname: tiny-a\n---\n# A\n\n## Room to judge `##room-to-judge`\n",
    );
    sandbox.write(
        ".claude/skills/tiny-b/SKILL.md",
        "---\nname: tiny-b\n---\n# B\n\n## Room to judge `##room-to-judge`\n",
    );
    sandbox.write(
        "README.md",
        &format!(
            "{}\nSee `primer@room-to-judge` and `skill@tiny-a@room-to-judge`.\n\n\
             And `skill@tiny-b@room-to-judge`.\n",
            std::fs::read_to_string(sandbox.path("README.md")).expect("the README")
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["show", "primer@room-to-judge"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("`##room-to-judge`"), "{out}");
    assert!(
        !out.contains("`##intent-and-claims`"),
        "a section stops at the next: {out}"
    );
    let (out, err, code) = sandbox.run(&["show", "skill@tiny-a@room-to-judge"]);
    assert_eq!(code, 0, "{out}{err}");
    let cited: Vec<&str> = out.lines().filter(|l| l.contains("README.md:")).collect();
    assert_eq!(cited.len(), 1, "{out}");
}

/// The claim: a commit message naming a skill by its bare name is a finding, judged against the
/// commit's own tree and its parent's, so a project skill the commit adds is reported in its
/// message, and so is one it deletes; the message citing the skill as a reference passes.
/// Mutations checked: the bare-name arm left out of the lints part makes the planted message pass;
/// the parent tree's lints dropped makes the deleting commit's message pass.
#[test]
fn a_message_naming_a_skill_its_commit_adds_by_its_bare_name_is_a_finding() {
    let history = History::new("commit-bare-skill");
    tiny_project_serving_claude(&history);
    let base = history.commit("The project is created\n");
    history.write(
        ".claude/skills/tiny-own/SKILL.md",
        "---\nname: tiny-own\ndescription: a project skill\n---\n# Own\n",
    );
    let sha = history.commit("A skill arrives\n\nIt adds `tiny-own`.\n");
    let cited = history.commit("A subject line\n\nIt cites `skill@tiny-own`.\n");
    history.remove(".claude/skills/tiny-own/SKILL.md");
    let gone = history.commit("A skill leaves\n\nIt deletes `tiny-own`.\n");
    let (out, err, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains(&format!("commit {gone}:3")), "{out}");
    assert!(
        out.contains(&format!("commit {sha}:3"))
            && out.contains(
                "`tiny-own` is written in backticks without its kind, and names `skill@tiny-own`"
            )
            && out.contains("write `skill@tiny-own`"),
        "{out}"
    );
    assert!(!out.contains(&format!("commit {cited}:")), "{out}");
}

/// The claim: the bare-name lint reports under the `claude` harness and nothing under
/// `harness = []`, where no skill is an entity. Mutation checked: the skills' definitions taken
/// whatever the harness makes the unharnessed copy report the span.
#[test]
fn a_bare_skill_name_is_silent_under_no_harness() {
    let plant = |sandbox: &Sandbox| {
        sandbox.write(
            ".claude/skills/tiny-own/SKILL.md",
            "---\nname: tiny-own\ndescription: a project skill\n---\n# Own\n",
        );
        sandbox.write(
            "README.md",
            &format!(
                "{}\nRun `tiny-own`.\n",
                std::fs::read_to_string(sandbox.path("README.md")).expect("the README")
            ),
        );
        sandbox.stage();
    };
    let harnessed = harnessed("bare-skill-harnessed");
    plant(&harnessed);
    let (out, err, code) = harnessed.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains(
            "`tiny-own` is written in backticks without its kind, and names `skill@tiny-own`"
        ),
        "{out}"
    );
    let bare = Sandbox::new("bare-skill-unharnessed", "minimal");
    plant(&bare);
    let (out, err, code) = bare.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: a commit made through `Sandbox::git` carries the sandbox's identity, not the
/// machine's, so no test's git command depends on the identity of the machine it runs on.
#[test]
fn a_sandbox_commit_carries_the_sandbox_identity() {
    let sandbox = Sandbox::new("sandbox-identity", "core");
    sandbox.git(&["commit", "-qm", "identity"]);
    let out = Command::new("git")
        .args(["log", "-1", "--format=%an <%ae> %cn <%ce>"])
        .current_dir(&sandbox.dir)
        .output()
        .expect("git runs");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        "fixture <fixture@example.invalid> fixture <fixture@example.invalid>"
    );
}

/// Commit what the sandbox holds, with the identity `Sandbox::git_command` gives.
fn commit_all(sandbox: &Sandbox, subject: &str) {
    sandbox.stage();
    sandbox.git(&["commit", "-qm", subject]);
}

/// The line of a report that opens with `prefix`, or `None`.
fn line_of<'a>(out: &'a str, prefix: &str) -> Option<&'a str> {
    out.lines().find(|l| l.starts_with(prefix))
}

/// The claim: `check --staged` judges the tree git's index would commit and nothing of the
/// working tree, in both directions, and plain `check` judges the working tree and prints no
/// `tree:` line. With nothing staged it says it judged HEAD's tree. Per
/// `design@check@staged-tree-source`.
#[test]
fn check_staged_judges_the_index_and_not_the_working_tree() {
    let sandbox = Sandbox::new("staged-verdict", "dirhome");
    commit_all(&sandbox, "the base");
    let readme = std::fs::read_to_string(sandbox.path("README.md")).expect("the README");
    let dangling = format!("{readme}\nIt leans on `design@dirhome@no-such-entry` too.\n");

    // A dangling reference staged, and the working tree restored: the index fails alone.
    sandbox.write("README.md", &dangling);
    sandbox.git(&["add", "README.md"]);
    sandbox.write("README.md", &readme);
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "the working tree passes: {out}{stderr}");
    assert_eq!(
        line_of(&out, "tree:"),
        None,
        "plain `check` names no tree: {out}"
    );
    let (out, stderr, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 1, "the staged tree fails: {out}{stderr}");
    assert!(out.contains("no-such-entry"), "{out}");
    assert_eq!(line_of(&out, "tree:"), Some("tree: staged"), "{out}");

    // The reverse: the index holds HEAD's README, the working tree the dangling one.
    sandbox.git(&["add", "README.md"]);
    sandbox.write("README.md", &dangling);
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "the working tree fails: {out}");
    let (out, stderr, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 0, "the staged tree passes: {out}{stderr}");
    assert_eq!(
        line_of(&out, "tree:"),
        Some("tree: staged (nothing staged: the tree of HEAD)"),
        "nothing is staged, and the run says so: {out}"
    );
}

/// The claim: `check --staged` run as a pre-commit hook judges the index the commit records, which
/// git names by `GIT_INDEX_FILE`: `.git/index` for a commit of what is staged, a lock file of its
/// own for `commit -a` and for `commit <paths>`, which leave `.git/index` holding what was staged
/// before; and the same in a linked worktree, whose hook git also gives `GIT_DIR`. Per
/// `design@git@index-file-variable-kept`; the forms of `git commit` a hook is covered for are those
/// this test runs.
#[test]
fn check_staged_as_a_pre_commit_hook_judges_the_index_the_commit_records() {
    let sandbox = Sandbox::new("staged-hook", "dirhome");
    commit_all(&sandbox, "the base");
    let hooks = sandbox.path("hooks");
    std::fs::create_dir_all(&hooks).expect("the hooks directory");
    let hook = hooks.join("pre-commit");
    std::fs::write(
        &hook,
        format!("#!/bin/sh\nexec '{}' check --staged\n", klarch().display()),
    )
    .expect("the hook");
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("executable");
    // The hooks directory is named on the command line, so a machine's own `core.hooksPath`
    // decides nothing here. It lies outside the walk only once ignored; it is ignored before
    // the hook runs, as an untracked file the walk would read.
    std::fs::write(sandbox.path(".git/info/exclude"), "hooks/\n").expect("the exclude file");
    let hooks_path = format!("core.hooksPath={}", hooks.display());
    let commit_in = |dir: &Path, args: &[&str]| {
        let mut all = vec!["-c", hooks_path.as_str(), "commit", "-q"];
        all.extend_from_slice(args);
        let out = sandbox
            .git_command(&all)
            .current_dir(dir)
            .output()
            .expect("git runs");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        (out.status.success(), text)
    };
    let commit = |args: &[&str]| commit_in(&sandbox.dir, args);
    let readme = std::fs::read_to_string(sandbox.path("README.md")).expect("the README");
    let dangling = format!("{readme}\nIt leans on `design@dirhome@no-such-entry` too.\n");

    // A plain commit of a staged dangling reference: the hook reads `.git/index`.
    sandbox.write("README.md", &dangling);
    sandbox.git(&["add", "README.md"]);
    let (passed, out) = commit(&["-m", "staged"]);
    assert!(!passed, "the hook refuses the staged tree: {out}");
    assert!(out.contains("no-such-entry"), "{out}");

    // `commit -a` of a sound working tree over that index: the hook reads the lock file, which
    // holds the working tree's README, so the commit is made.
    sandbox.write("README.md", &format!("{readme}\nOne more sentence.\n"));
    let (passed, out) = commit(&["-a", "-m", "sound"]);
    assert!(
        passed,
        "the hook passes the tree `commit -a` records: {out}"
    );

    // `commit -a` of a dangling reference over a sound index: `.git/index` passes, and the hook
    // must refuse the tree the commit records all the same.
    sandbox.write("README.md", &dangling);
    let (passed, out) = commit(&["-a", "-m", "unstaged"]);
    assert!(
        !passed,
        "the hook refuses the tree `commit -a` records: {out}"
    );
    assert!(out.contains("no-such-entry"), "{out}");

    // `commit <paths>` reads a lock file of its own too: the dangling README named, refused;
    // a sound one named, over the same index, passed.
    let (passed, out) = commit(&["-m", "named", "README.md"]);
    assert!(
        !passed,
        "the hook refuses the tree `commit <paths>` records: {out}"
    );
    assert!(out.contains("no-such-entry"), "{out}");
    sandbox.write("README.md", &format!("{readme}\nA named sentence.\n"));
    let (passed, out) = commit(&["-m", "named", "README.md"]);
    assert!(
        passed,
        "the hook passes the tree `commit <paths>` records: {out}"
    );

    // A plain commit of a sound staged README passes.
    sandbox.write("README.md", &format!("{readme}\nA staged sentence.\n"));
    sandbox.git(&["add", "README.md"]);
    let (passed, out) = commit(&["-m", "staged, sound"]);
    assert!(passed, "the hook passes the staged tree: {out}");

    // The variants the goal admits beside `-m` and `-q`: a message from `-F`, one from the
    // editor, a run without `-q`, and `--` before the paths, each refused over a failing tree
    // and passed over a sound one.
    let message = hooks.join("message.txt");
    std::fs::write(&message, "from a file\n").expect("the message file");
    let editor = hooks.join("editor.sh");
    std::fs::write(&editor, "#!/bin/sh\necho 'from the editor' > \"$1\"\n").expect("the editor");
    std::fs::set_permissions(&editor, std::fs::Permissions::from_mode(0o755)).expect("executable");
    let loud = |args: &[&str]| {
        let mut all = vec!["-c", hooks_path.as_str(), "commit"];
        all.extend_from_slice(args);
        let out = sandbox
            .git_command(&all)
            .env("GIT_EDITOR", &editor)
            .output()
            .expect("git runs");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        (out.status.success(), text)
    };
    let message_path = message.to_string_lossy().into_owned();
    let variants: [&[&str]; 4] = [
        &["-F", &message_path],
        &[],
        &["-m", "loud"],
        &["-m", "separated", "--", "README.md"],
    ];
    for (n, variant) in variants.iter().enumerate() {
        let named = variant.contains(&"--");
        sandbox.write("README.md", &dangling);
        if !named {
            sandbox.git(&["add", "README.md"]);
        }
        let (passed, out) = loud(variant);
        assert!(!passed, "{variant:?} refuses a failing tree: {out}");
        assert!(out.contains("no-such-entry"), "{out}");
        sandbox.write("README.md", &format!("{readme}\nVariant sentence {n}.\n"));
        if !named {
            sandbox.git(&["add", "README.md"]);
        }
        let (passed, out) = loud(variant);
        assert!(passed, "{variant:?} passes a sound tree: {out}");
    }

    // A linked worktree, whose hook git also gives `GIT_DIR`: each form refuses a failing tree
    // and passes a sound one. The guard removes the worktree whatever the assertions do.
    struct Linked<'a>(&'a Sandbox, PathBuf);
    impl Drop for Linked<'_> {
        fn drop(&mut self) {
            let path = self.1.to_string_lossy().into_owned();
            let _ = self
                .0
                .git_command(&["worktree", "remove", "--force", &path])
                .output();
            let _ = std::fs::remove_dir_all(&self.1);
        }
    }
    let linked = Linked(
        &sandbox,
        std::env::temp_dir().join(format!("knowledge-staged-hook-wt-{}", std::process::id())),
    );
    let _ = std::fs::remove_dir_all(&linked.1);
    sandbox.git(&[
        "worktree",
        "add",
        "-q",
        "--detach",
        &linked.1.to_string_lossy(),
    ]);
    let readme_at = linked.1.join("README.md");
    let stage = |dir: &Path| {
        let out = sandbox
            .git_command(&["add", "README.md"])
            .current_dir(dir)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    for (n, form) in [&["-a"][..], &["--", "README.md"][..], &[][..]]
        .iter()
        .enumerate()
    {
        let run = |subject: &str| {
            let mut args = vec!["-m", subject];
            args.extend_from_slice(form);
            commit_in(&linked.1, &args)
        };
        std::fs::write(&readme_at, &dangling).expect("the linked README");
        if form.is_empty() {
            stage(&linked.1);
        }
        let (passed, out) = run("linked, failing");
        assert!(!passed, "{form:?} in a linked worktree refuses: {out}");
        assert!(out.contains("no-such-entry"), "{out}");
        std::fs::write(&readme_at, format!("{readme}\nLinked sentence {n}.\n"))
            .expect("the linked README");
        if form.is_empty() {
            stage(&linked.1);
        }
        let (passed, out) = run("linked, sound");
        assert!(passed, "{form:?} in a linked worktree passes: {out}");
    }
}

/// The claim: a `GIT_DIR` naming another repository in the caller's environment changes nothing:
/// the checker asks git about the repository the project's directory lies in. Per
/// `design@git@repository-variables-dropped`.
#[test]
fn a_git_dir_in_the_environment_does_not_redirect_the_walk() {
    let sandbox = Sandbox::new("git-dir-env", "dirhome");
    let other =
        std::env::temp_dir().join(format!("knowledge-git-dir-other-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&other);
    std::fs::create_dir_all(&other).expect("the other directory");
    for args in [&["init", "-q"][..], &["add", "f"][..]] {
        if args[0] == "add" {
            std::fs::write(other.join("f"), "x\n").expect("a file of the other repository");
        }
        let out = Command::new("git")
            .args(args)
            .current_dir(&other)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "the project passes: {out}{stderr}");
    for variable in ["GIT_DIR", "GIT_COMMON_DIR"] {
        let (out, stderr, code) =
            run_with_env(&sandbox.dir, &["check"], &[(variable, &other.join(".git"))]);
        assert_eq!(
            code, 0,
            "{variable} names another repository: {out}{stderr}"
        );
    }
    let (out, stderr, code) = run_with_env(&sandbox.dir, &["check"], &[("GIT_WORK_TREE", &other)]);
    assert_eq!(code, 0, "GIT_WORK_TREE names another tree: {out}{stderr}");
    let _ = std::fs::remove_dir_all(&other);
}

/// The claim: a required document missing at the root Component names the one repair that
/// applies, creating it: the root is no member of `[project] components`, so the repair a listed
/// Component's finding names, to stop listing it, cannot be carried out there.
#[test]
fn a_root_missing_a_required_document_is_told_only_to_create_it() {
    let sandbox = Sandbox::new("root-readme", "dirhome");
    std::fs::remove_file(sandbox.path("README.md")).expect("the root README");
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    let at = out
        .lines()
        .position(|l| l.starts_with("README.md  "))
        .unwrap_or_else(|| panic!("a finding on README.md: {out}"));
    let repair = out.lines().nth(at + 1).expect("the repair line");
    assert!(repair.contains("create it"), "{repair}");
    assert!(!repair.contains("stop listing"), "{repair}");
}

/// The claim, per `design@model@escape-anchor-elsewhere`: an escape whose target is a plan document is told
/// the plan document's own form, never a `path` the next run refuses; and a path opening with `..`
/// is read from its document first, so the form names the file its writer meant.
#[test]
fn an_escape_resolving_here_names_a_form_the_next_run_accepts() {
    let sandbox = Sandbox::new("escape-forms", "dirhome");
    let cases = [
        (
            "docs/plans/milestones/a-milestone/",
            "`milestone@plans@a-milestone`",
        ),
        (
            "docs/plans/milestones/a-milestone/README.md",
            "`milestone@plans@a-milestone`",
        ),
        (
            "docs/plans/milestones/a-milestone/a-step.md",
            "`spec@a-milestone@a-step`",
        ),
        ("docs/plans/specs/a-spec.md", "`spec@plans@a-spec`"),
        ("../README.md", "`path@dirhome@README.md`"),
    ];
    let text: String = cases
        .iter()
        .map(|(path, _)| format!("See `path@elsewhere@{path}`.\n\n"))
        .collect();
    sandbox.write("docs/probe.md", &format!("# Probe\n\n{text}"));
    sandbox.stage();
    let (out, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{stderr}");
    for (path, form) in cases {
        let line = out
            .lines()
            .find(|l| l.contains(&format!("`path@elsewhere@{path}`")))
            .unwrap_or_else(|| panic!("a finding on {path}: {out}"));
        assert!(line.contains(form), "{path}: {line}");
    }
}

/// The claim: `check --staged` compares the staged installed files against the shipped set, as
/// `check` does on disk, and prints the `checker source:` line `check` prints.
#[test]
fn check_staged_compares_the_staged_installed_files_and_names_the_checker_source() {
    let sandbox = Sandbox::new("staged-installed", "dirhome");
    sandbox.serve_claude();
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    commit_all(&sandbox, "the base, installed");
    let primer = ".claude/knowledge-architect/PRIMER.md";
    let shipped = std::fs::read_to_string(sandbox.path(primer)).expect("the primer");
    sandbox.write(
        primer,
        &format!("{shipped}\nA line the version does not ship.\n"),
    );
    sandbox.git(&["add", primer]);
    sandbox.write(primer, &shipped);

    let (plain, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(
        code, 0,
        "the working copy is the shipped one: {plain}{stderr}"
    );
    let (staged, stderr, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 1, "the staged copy is not: {staged}{stderr}");
    // The one finding is the staged primer's: every other installed file is staged as shipped.
    assert!(staged.contains("FAILED: 1 finding above"), "{staged}");
    assert!(!staged.contains("phase 2:"), "{staged}");
    assert!(
        staged.contains(&format!("{primer}  this installed file differs")),
        "{staged}"
    );
    let checker = line_of(&plain, "checker source:");
    assert!(checker.is_some(), "{plain}");
    assert_eq!(line_of(&staged, "checker source:"), checker, "{staged}");
}

/// The claim, per `design@check@staged-check-semantics`: `check --staged` compares every staged
/// installed file, a workflow script that a walk row skips included, as `check` compares it on
/// disk. Mutation checked: `read_tree` reading under the index only the installed files whose
/// name ends in `.md` reports each shipped workflow as differing.
#[test]
fn check_staged_compares_an_installed_file_a_walk_row_skips() {
    let sandbox = harnessed("staged-skipped-installed");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    // Asserted, since a row missed by the replacement leaves this test the existing staged one,
    // which the mutation survives.
    assert_eq!(
        manifest.matches("skip-dirs = [\"build\"]").count(),
        1,
        "{manifest}"
    );
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-dirs = [\"build\"]",
            "skip-dirs = [\"build\", \".claude/workflows\"]",
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 0, "{out}{err}");
    let (workflow, shipped) = knowledge_architect_agent_skills::FILES
        .iter()
        .find(|(path, _)| path.starts_with(".claude/workflows/"))
        .expect("this version ships a workflow");
    sandbox.write(workflow, &format!("{shipped}\n// an edit\n"));
    sandbox.git(&["add", workflow]);
    let (out, err, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("FAILED: 1 finding above"), "{out}");
    assert!(
        out.contains(&format!("{workflow}  this installed file differs")),
        "{out}"
    );
}

/// The claim: a symbolic link in the installer's namespace that a `skip-files` row keeps is
/// still judged by the installed-file check, which reads the namespace whatever the walk rows
/// say: the row silences the link's own finding and no other. Ownership is decided by the name,
/// per `design@check@owned-namespace-check`, and the installed files are outside the walk, so no
/// walk row governs them, per `design@check@installed-files-outside-the-walk`. Mutation checked: `from_listing`
/// leaving a link out of `installed` at the owned test makes the stray pass.
#[test]
fn a_link_in_the_installers_namespace_a_skip_row_keeps_is_still_judged() {
    let sandbox = harnessed("owned-link");
    let link = ".claude/agents/knowledge-architect-linked.md";
    std::os::unix::fs::symlink("../../notes/a.md", sandbox.path(link)).expect("a symlink");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace(
            "skip-files = [\"notes/generated.md\"]",
            &format!("skip-files = [\"{link}\", \"notes/generated.md\"]"),
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(!out.contains("is a symlink"), "{out}");
    assert!(out.contains("FAILED: 1 finding above"), "{out}");
    assert!(
        out.contains(&format!(
            "{link}  this file sits in the installer's namespace"
        )),
        "{out}"
    );
}

/// The claim: `check --fix` removes the skill directory its deletion of an unshipped skill left
/// empty, as the install does. Mutation checked: `apply` not calling `remove_empty_owned_dirs`
/// leaves the directory.
#[test]
fn check_fix_removes_the_skill_directory_its_deletion_empties() {
    let sandbox = harnessed("fix-empty-skill");
    let stray = ".claude/skills/knowledge-architect-retired/SKILL.md";
    sandbox.write(stray, "a skill an older version shipped\n");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains(&format!("fixed: removed {stray} (installed)")),
        "{out}"
    );
    assert!(!sandbox
        .path(".claude/skills/knowledge-architect-retired")
        .exists());
}

/// The claim: `check --staged` reads a staged file outside the walk whose bytes are not UTF-8 as
/// `check` reads it on disk, binary, and so judges it no differently. Mutation checked: `read_tree`
/// keeping no state for a blob that does not decode reports the file as unreadable.
#[test]
fn check_staged_reads_a_staged_binary_file_as_check_does() {
    let sandbox = Sandbox::new("staged-binary", "minimal");
    std::fs::write(
        sandbox.path("notes/img.png"),
        b"\x89PNG\r\n\x1a\n\0\0\xff\xfe",
    )
    .expect("a binary file");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
    let (out, err, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: an index holding an unmerged path is refused with exit 2, naming the path once.
#[test]
fn check_staged_refuses_an_unmerged_index() {
    let sandbox = Sandbox::new("staged-unmerged", "dirhome");
    commit_all(&sandbox, "the base");
    let readme = std::fs::read_to_string(sandbox.path("README.md")).expect("the README");
    let base = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(sandbox.path(""))
        .output()
        .expect("git runs");
    let base = String::from_utf8_lossy(&base.stdout).trim().to_string();
    sandbox.git(&["checkout", "-qb", "side"]);
    sandbox.write("README.md", &format!("{readme}\nSide.\n"));
    commit_all(&sandbox, "side");
    sandbox.git(&["checkout", "-q", &base]);
    sandbox.write("README.md", &format!("{readme}\nMain.\n"));
    commit_all(&sandbox, "main");
    // The merge conflicts and exits 1; the index it leaves is the subject. It runs through
    // `git_command`, which gives it the sandbox's identity, and its output is kept for the
    // assertion below.
    let merge = sandbox
        .git_command(&["merge", "-q", "side"])
        .output()
        .expect("git runs");
    let unmerged = Command::new("git")
        .args(["ls-files", "-u"])
        .current_dir(sandbox.path(""))
        .output()
        .expect("git runs");
    assert!(
        !unmerged.stdout.is_empty(),
        "the merge left no unmerged path: exit {:?}, {}{}",
        merge.status.code(),
        String::from_utf8_lossy(&merge.stdout),
        String::from_utf8_lossy(&merge.stderr)
    );
    let (out, stderr, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains("the index holds unmerged paths: README.md\n")
            || stderr
                .trim_end()
                .ends_with("the index holds unmerged paths: README.md"),
        "{stderr}"
    );
}

/// The claim: `check --staged` reports a file git both tracks and ignores, as `check` does,
/// since the question is about the index.
#[test]
fn check_staged_reports_a_tracked_file_the_ignore_rules_cover() {
    let sandbox = Sandbox::new("staged-tracked-ignored", "dirhome");
    // Staged first, then covered: the add finds the file already in the index.
    sandbox.write("notes.txt", "kept\n");
    sandbox.stage();
    sandbox.write(".gitignore", "notes.txt\n");
    sandbox.git(&["add", ".gitignore"]);
    let (plain, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{plain}");
    let (staged, _, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 1, "{staged}");
    assert!(staged.contains("notes.txt"), "{staged}");
    let finding = |out: &str| {
        out.lines()
            .find(|l| l.starts_with("notes.txt"))
            .map(str::to_string)
    };
    assert_eq!(finding(&staged), finding(&plain), "{plain}\n{staged}");
}

/// Every file of the sandbox but git's own, with its bytes.
fn working_files(sandbox: &Sandbox) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    fn walk(dir: &Path, root: &Path, out: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>) {
        for entry in std::fs::read_dir(dir).expect("a directory") {
            let path = entry.expect("an entry").path();
            if path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            if path.is_dir() {
                walk(&path, root, out);
            } else {
                let rel = path
                    .strip_prefix(root)
                    .expect("under the root")
                    .to_path_buf();
                out.insert(rel, std::fs::read(&path).expect("a file"));
            }
        }
    }
    let mut out = std::collections::BTreeMap::new();
    walk(&sandbox.path(""), &sandbox.path(""), &mut out);
    out
}

/// What `git ls-files -s` lists in the sandbox, one entry per line.
fn index_lines(sandbox: &Sandbox) -> Vec<String> {
    let out = Command::new("git")
        .args(["ls-files", "-s"])
        .current_dir(sandbox.path(""))
        .output()
        .expect("git runs");
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

/// The lines of `after` that `before` does not hold.
fn changed_lines(before: &[String], after: &[String]) -> Vec<String> {
    after
        .iter()
        .filter(|l| !before.contains(l))
        .cloned()
        .collect()
}

const STAGED_ENTRY: &str = "---\nkind: todo\n---\n# An entry staged for this commit\n\n## Summary\n\nStaged.\n\n## Details\n\n### What\n\nStaged.\n\n### Why it matters\n\nIt is.\n\n### What would close it\n\nA commit.\n";
const WORKING_ENTRY: &str = "---\nkind: todo\n---\n# An entry left for a later commit\n\n## Summary\n\nUnstaged.\n\n## Details\n\n### What\n\nUnstaged.\n\n### Why it matters\n\nIt is.\n\n### What would close it\n\nA later commit.\n";

/// The claim: `index --staged` names each generated file whose working copy is out of date for
/// the working tree, with the command that rewrites it, and names no other. The case named is an
/// entry removed with `git rm`, after which the working copy of the register's index still lists
/// it; the case not named is the partial commit the command serves, where the working copy holds
/// the working tree's rows and the staged file the staged tree's.
#[test]
fn index_staged_names_a_working_copy_out_of_date_for_the_working_tree() {
    let sandbox = Sandbox::new("index-staged-stale", "dirhome");
    commit_all(&sandbox, "the base");
    sandbox.git(&["rm", "-q", "docs/open-issues/an-ungrouped-entry.md"]);
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{stderr}");
    let line = out
        .lines()
        .find(|l| l.starts_with("docs/open-issues/index.md") && l.contains("working copy"))
        .unwrap_or_else(|| panic!("a line on the working copy: {out}"));
    assert!(line.contains("index` rewrites it"), "{line}");
    assert_eq!(
        out.lines().filter(|l| l.contains("working copy")).count(),
        1,
        "no other file is named: {out}"
    );
    // After `index`, the working copy matches, and a second run names nothing.
    let (out, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{stderr}");
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(!out.contains("working copy"), "{out}");

    // A partial commit: one new entry staged, another left in the working tree, whose index
    // lists both. The working copy is current for the working tree, so nothing is named.
    commit_all(&sandbox, "the entry leaves");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.write("docs/open-issues/working-entry.md", WORKING_ENTRY);
    let (out, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{stderr}");
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(out.lines().any(|l| l.ends_with(" staged")), "{out}");
    assert!(!out.contains("working copy"), "a partial commit: {out}");

    // A working copy that cannot be read is out of date as well.
    std::fs::remove_file(sandbox.path("docs/plans/specs/index.md")).expect("the specs index");
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l.starts_with("docs/plans/specs/index.md") && l.contains("working copy")),
        "a deleted copy: {out}"
    );
}

/// The claim of `design@check@index-staged-write`: `index --staged` changes exactly the staged entries of
/// the generated files whose staged bytes differ, a missing one included, and no working-tree
/// file. After it the staged tree passes, while the working tree still needs its own index.
#[test]
fn index_staged_stages_the_generated_files_and_touches_no_working_file() {
    let sandbox = Sandbox::new("index-staged", "dirhome");
    commit_all(&sandbox, "the base");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);
    sandbox.write("docs/open-issues/working-entry.md", WORKING_ENTRY);
    // The specs index leaves the index, so it is a missing generated file of the staged tree.
    sandbox.git(&["rm", "-q", "--cached", "docs/plans/specs/index.md"]);
    let (out, _, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 1, "the staged tree needs its index: {out}");

    let files_before = working_files(&sandbox);
    let index_before = index_lines(&sandbox);
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(out.contains("docs/open-issues/index.md"), "{out}");
    assert!(out.lines().any(|l| l.ends_with(" staged")), "{out}");
    let changed: Vec<String> = changed_lines(&index_before, &index_lines(&sandbox))
        .into_iter()
        .map(|l| l.split('\t').nth(1).unwrap_or_default().to_string())
        .collect();
    assert_eq!(
        changed,
        vec![
            "docs/open-issues/index.md".to_string(),
            "docs/plans/specs/index.md".to_string()
        ],
        "exactly the generated files whose staged bytes differed"
    );
    assert_eq!(
        working_files(&sandbox),
        files_before,
        "no working-tree file changes"
    );
    let (out, stderr, code) = sandbox.run(&["check", "--staged"]);
    assert_eq!(code, 0, "the staged tree passes: {out}{stderr}");
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        !out.lines().any(|l| l.ends_with(" staged")),
        "a second run stages nothing: {out}"
    );
}

/// `design@check@index-staged-write`: `index --staged` writes nothing, exit 2, when phases 1 to 3 of the staged tree find
/// anything, and when git's index lock is held.
#[test]
fn index_staged_refuses_with_the_index_unchanged() {
    let sandbox = Sandbox::new("index-staged-refused", "dirhome");
    commit_all(&sandbox, "the base");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);

    let lock = sandbox.path(".git/index.lock");
    std::fs::write(&lock, "").expect("a held lock");
    let before = index_lines(&sandbox);
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(stderr.contains("Nothing was staged."), "{stderr}");
    std::fs::remove_file(&lock).expect("the lock released");
    assert_eq!(index_lines(&sandbox), before, "the index is as it was");

    // A gitlink staged in the index stops the staged tree in phase 2, per
    // `design@model@links-are-no-documents`.
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(sandbox.path(""))
        .output()
        .expect("git runs");
    let head = String::from_utf8_lossy(&head.stdout).trim().to_string();
    sandbox.git(&[
        "update-index",
        "--add",
        "--cacheinfo",
        &format!("160000,{head},docs/sub"),
    ]);
    let before = index_lines(&sandbox);
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains("the staged model is incomplete"),
        "{stderr}"
    );
    assert_eq!(index_lines(&sandbox), before, "the index is as it was");
}

/// The claim: `check --fix` runs where the staged tree and the working tree need the
/// same generated files, though the index differs from HEAD.
#[test]
fn fix_runs_where_both_trees_need_the_same_generated_files() {
    let sandbox = Sandbox::new("fix-same", "dirhome");
    commit_all(&sandbox, "the base");
    let readme = std::fs::read_to_string(sandbox.path("README.md")).expect("the README");
    sandbox.write("README.md", &format!("{readme}\nStaged.\n"));
    sandbox.git(&["add", "README.md"]);
    sandbox.write("README.md", &format!("{readme}\nStaged.\n\nNot staged.\n"));
    let (out, stderr, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{stderr}");
}

/// The claim: `check --fix` refuses, exit 2 and nothing written, where the index differs from HEAD
/// and the two trees need different generated files, naming each and both repairs.
#[test]
fn fix_refuses_where_the_two_trees_need_different_generated_files() {
    let sandbox = Sandbox::new("fix-differs", "dirhome");
    commit_all(&sandbox, "the base");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);
    sandbox.write("docs/open-issues/working-entry.md", WORKING_ENTRY);
    let before = working_files(&sandbox);
    let (out, stderr, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains(
            "these generated files would differ between the working tree and the staged tree: \
             docs/open-issues/index.md"
        ),
        "{stderr}"
    );
    assert!(stderr.contains("index --staged, then"), "{stderr}");
    assert!(
        stderr.contains("to fix the working tree as a whole"),
        "{stderr}"
    );
    assert_eq!(working_files(&sandbox), before, "nothing is written");
}

/// The claim: with the index equal to HEAD, `check --fix` compares nothing and writes the working
/// tree's generated files as before.
#[test]
fn fix_writes_the_working_trees_files_where_nothing_is_staged() {
    let sandbox = Sandbox::new("fix-unstaged", "dirhome");
    commit_all(&sandbox, "the base");
    sandbox.write("docs/open-issues/working-entry.md", WORKING_ENTRY);
    let (out, stderr, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.contains("fixed: wrote docs/open-issues/index.md"),
        "{out}"
    );
}

/// The claim: the refusal comes after the installed repairs, per
/// `design@check@fix-before-the-checks`: a run that repaired an installed file and then refuses
/// exits 1 and writes no generated file.
#[test]
fn fix_refuses_after_the_installed_repairs_with_exit_one() {
    let sandbox = Sandbox::new("fix-differs-installed", "dirhome");
    sandbox.serve_claude();
    let (out, stderr, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{stderr}");
    commit_all(&sandbox, "the base, installed");
    let primer = ".claude/knowledge-architect/PRIMER.md";
    sandbox.write(primer, "A primer the version does not ship.\n");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);
    sandbox.write("docs/open-issues/working-entry.md", WORKING_ENTRY);
    let index = std::fs::read(sandbox.path("docs/open-issues/index.md")).expect("the index");
    let (out, stderr, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}{stderr}");
    assert!(
        out.contains(&format!("fixed: wrote {primer} (installed)")),
        "{out}"
    );
    assert!(stderr.contains("would differ"), "{stderr}");
    assert_eq!(
        std::fs::read(sandbox.path("docs/open-issues/index.md")).expect("the index"),
        index,
        "no generated file is written"
    );
}

/// The claim: `--fix` with `--staged` is refused with exit 2 before anything is read: run where no
/// project is, it is clap that answers.
#[test]
fn fix_with_staged_is_refused_before_anything_is_read() {
    let nowhere = std::env::temp_dir();
    let (out, stderr, code) = run_in(&nowhere, &["check", "--fix", "--staged"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(stderr.contains("cannot be used with"), "{stderr}");
}

/// The claim: `index --staged` refuses, exit 2 with the index unchanged, a generated path the
/// index holds as a gitlink, a symlink or a directory, though a walk row hides the path from the
/// survey: it never writes a generated file's text under another kind of entry, nor replaces the
/// entries under a directory.
#[test]
fn index_staged_refuses_a_destination_staged_as_another_kind_of_entry() {
    let sandbox = Sandbox::new("index-staged-kinds", "dirhome");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replacen(
            "skip-files = []",
            "skip-files = [\"docs/open-issues/index.md\"]",
            1,
        ),
    );
    commit_all(&sandbox, "the base");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(sandbox.path(""))
        .output()
        .expect("git runs");
    let head = String::from_utf8_lossy(&head.stdout).trim().to_string();
    for mode in ["160000", "120000"] {
        sandbox.git(&[
            "update-index",
            "--cacheinfo",
            &format!("{mode},{head},docs/open-issues/index.md"),
        ]);
        let before = index_lines(&sandbox);
        let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
        assert_eq!(code, 2, "mode {mode}: {out}{stderr}");
        assert!(
            stderr.contains("is staged as a symlink or a gitlink"),
            "{stderr}"
        );
        assert_eq!(
            index_lines(&sandbox),
            before,
            "mode {mode}: the index is as it was"
        );
    }

    // A directory in the index at the generated path.
    sandbox.git(&["rm", "-q", "-f", "--cached", "docs/open-issues/index.md"]);
    let blob = Command::new("git")
        .args(["hash-object", "-w", "--stdin"])
        .current_dir(sandbox.path(""))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .take()
                .expect("stdin")
                .write_all(b"a staged note\n")?;
            child.wait_with_output()
        })
        .expect("a blob");
    let blob = String::from_utf8_lossy(&blob.stdout).trim().to_string();
    sandbox.git(&[
        "update-index",
        "--add",
        "--cacheinfo",
        &format!("100644,{blob},docs/open-issues/index.md/note.txt"),
    ]);
    let before = index_lines(&sandbox);
    let (out, stderr, code) = sandbox.run(&["index", "--staged"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(stderr.contains("is a directory in the index"), "{stderr}");
    assert_eq!(index_lines(&sandbox), before, "the index is as it was");
}

/// The claim: `check --fix` refuses only for a generated file it would write. With the working
/// tree's index already current, a partial stage makes it write nothing, and the repair the refusal
/// names, `index`, leaves nothing to refuse: `--fix` runs as `check` does.
#[test]
fn fix_refuses_nothing_where_it_would_write_nothing() {
    let sandbox = Sandbox::new("fix-writes-nothing", "dirhome");
    commit_all(&sandbox, "the base");
    sandbox.write("docs/open-issues/staged-entry.md", STAGED_ENTRY);
    sandbox.git(&["add", "docs/open-issues/staged-entry.md"]);
    sandbox.write("docs/open-issues/working-entry.md", WORKING_ENTRY);
    let (out, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{stderr}");
    let (out, stderr, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(!out.contains("fixed:"), "{out}");
}

/// The manifest of a project that declares a Directory register at a location's own path, each
/// entry carrying one heading register, per `design@model@declarable-directory-register` and
/// `design@model@home-at-the-carrier-path`: the layout a game project keeps its game design in, one
/// subject per directory.
const GAME_MANIFEST: &str = "[project]\nchecker-version = \"fixture\"\nname = \"core-mock\"\n\
    components = []\n\n\
    [locations.gdd]\npath = \"game-design\"\nregisters = [\"subject\"]\n\n\
    [registers.subject]\nscope = \"opt-in\"\nshape = \"directory\"\ndir = \".\"\n\
    carries = [\"game-design\"]\n\n\
    [registers.game-design]\nscope = \"opt-in\"\nshape = \"heading\"\nlevel = 3\n\n\
    [walk]\nskip-dirs = []\nskip-files = []\n\n\
    [agents]\nharness = []\n";

/// The documents of that project's game design: two subjects, each a README linking its files,
/// whose entries cite each other across subjects.
const GAME_FILES: [(&str, &str); 7] = [
    ("knowledge-architect.toml", GAME_MANIFEST),
    (
        "game-design/README.md",
        "# Game design\n\nOne directory per subject; the listing beside this file is generated.\n",
    ),
    (
        "game-design/close-combat/README.md",
        "# Combat\n\n- [Melee](melee.md)\n- [Ranged](ranged.md)\n",
    ),
    (
        "game-design/close-combat/melee.md",
        "# Melee\n\n### A sword swing takes 400 ms `##sword-swing`\n\n\
         It costs stamina, per `game-design@war-economy@stamina-is-a-currency`.\n",
    ),
    (
        "game-design/close-combat/ranged.md",
        "# Ranged\n\n### Arrows drop with gravity `##arrow-drop`\n\n\
         Slower than `game-design@close-combat@sword-swing`, in the subject `subject@gdd@close-combat`, \
         beside `path@close-combat@melee.md`.\n",
    ),
    (
        "game-design/war-economy/README.md",
        "# Economy\n\n- [Resources](resources.md)\n",
    ),
    (
        "game-design/war-economy/resources.md",
        "# Resources\n\n### Stamina is spent like a currency `##stamina-is-a-currency`\n\n\
         See `game-design@close-combat@arrow-drop`, and every subject in `register@gdd@subject`.\n",
    ),
];

/// A copy of the `core` mock holding that game design, its indexes generated, staged.
fn game_sandbox(tag: &str) -> Sandbox {
    let sandbox = Sandbox::seeded(tag, "core", &GAME_FILES);
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    sandbox
}

/// The claim: a declared Directory register places one anchor per entry directory, named by its
/// basename and carrying what the register's `carries` names at its own directory, so an entry of
/// another subject, the subject as a whole, the home and a file beside the README all resolve,
/// and the home's index lists one row per subject.
#[test]
fn a_declared_directory_register_places_one_anchor_per_entry_carrying_its_registers() {
    let sandbox = game_sandbox("directory-register");
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.ends_with("PASSED: no findings\n"), "{stdout}");

    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    for line in [
        "gdd  location  game-design",
        "  subject  game-design/",
        "close-combat  subject  game-design/close-combat",
        "  game-design  game-design/close-combat/",
        "war-economy  subject  game-design/war-economy",
    ] {
        assert!(out.lines().any(|l| l == line), "no line `{line}`:\n{out}");
    }

    let (out, stderr, code) =
        sandbox.run(&["show", "game-design@war-economy@stamina-is-a-currency"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(out.contains("game-design/close-combat/melee.md:5"), "{out}");
    let (out, stderr, code) = sandbox.run(&["show", "subject@gdd@close-combat"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.contains("game-design/close-combat/ranged.md:5"),
        "{out}"
    );

    let index =
        std::fs::read_to_string(sandbox.path("game-design/index.md")).expect("the subjects' index");
    assert!(index.contains("\n2 entries\n"), "{index}");
    assert!(
        index.contains("| close-combat | [Combat](close-combat/README.md) |")
            && index.contains("| war-economy | [Economy](war-economy/README.md) |"),
        "{index}"
    );
}

/// The claim: a declared Directory home holds its README, its index and one directory per entry,
/// each holding a README and no subdirectory or register.toml, as the milestones home does, and
/// each departure is a phase-2 finding naming the register's entry.
#[test]
fn a_declared_directory_home_holds_its_entries_and_nothing_else() {
    let sandbox = game_sandbox("directory-layout");
    sandbox.write("game-design/lore/notes.md", "# Notes\n");
    sandbox.write("game-design/stray.md", "# Stray\n");
    sandbox.write("game-design/close-combat/extra/deep.md", "# Deep\n");
    sandbox.write("game-design/war-economy/register.toml", "groups = []\n");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("phase 2: "), "{stdout}");
    for what in [
        "`game-design/lore` holds no README.md, so it is no subject entry",
        "`game-design/stray.md` sits in game-design/, which holds its README.md, its index.md and \
         one directory per subject entry",
        "`game-design/close-combat/extra` is a subdirectory of a subject entry",
        "a subject entry directory holds no register.toml",
    ] {
        assert!(stdout.contains(what), "no finding `{what}`:\n{stdout}");
    }
}

/// The claim: an entry's name is refused when it reads as another anchor, and of two entries of
/// one name the anchor is decided by the order `design@model@a-placed-name-reads-as-nothing-else`
/// states, a milestone first, then the Directory registers by name, whatever the order of the
/// declarations: AC3 of the discussion that made the shape declarable.
#[test]
fn an_entry_named_like_another_anchor_is_refused_in_a_fixed_order() {
    let sandbox = game_sandbox("directory-names");
    // A second Directory register, declared after `subject` and named before it.
    let manifest = GAME_MANIFEST.replace(
        "[walk]",
        "[locations.world]\npath = \"world\"\nregisters = [\"area\"]\n\n\
         [registers.area]\nscope = \"opt-in\"\nshape = \"directory\"\ncarries = [\"game-design\"]\n\n\
         [walk]",
    );
    sandbox.write("knowledge-architect.toml", &manifest);
    sandbox.write("world/README.md", "# World\n");
    sandbox.write("world/area/README.md", "# Areas\n");
    sandbox.write("world/area/close-combat/README.md", "# Combat areas\n");
    sandbox.write("world/area/war-economy/README.md", "# Market\n");
    sandbox.write(
        "docs/plans/milestones/war-economy/README.md",
        "# A war-economy milestone\n",
    );
    sandbox.write(
        "game-design/game-design/README.md",
        "# A subject named like a kind\n",
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for what in [
        // `area` sorts before `subject`, so its `close-combat` is the anchor.
        "the subject entry `close-combat` is no anchor: its name is also the name of an entry of `area`",
        // A milestone comes first of all.
        "the area entry `war-economy` is no anchor: its name is also the name of a milestone",
        "the subject entry `game-design` is no anchor: its name is the name of a kind",
    ] {
        assert!(stdout.contains(what), "no finding `{what}`:\n{stdout}");
    }
    assert!(
        !stdout.contains("the area entry `close-combat`")
            && !stdout.contains("the milestone `war-economy`"),
        "{stdout}"
    );
}

/// The claim: an entry's README is the head of what it carries, so a slug there defines nothing
/// and is reported, and the slug moved into a file beside it defines the entry: AC4 of the
/// discussion that made the shape declarable.
#[test]
fn a_slug_in_an_entry_s_readme_defines_nothing() {
    let sandbox = game_sandbox("directory-readme-slug");
    sandbox.write(
        "game-design/close-combat/README.md",
        "# Combat\n\n- [Melee](melee.md)\n- [Ranged](ranged.md)\n\n### Parry `##parry`\n",
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("game-design/close-combat/README.md:6") && stdout.contains("`##parry`"),
        "{stdout}"
    );
}

/// The claim: what an entry carries is named by the entry's own reference, so a `register` or a
/// `path` reference to an entry's directory or README is refused with that reference as the
/// repair, and a path to a file beside the README resolves.
#[test]
fn an_entry_s_directory_is_cited_by_the_entry_and_by_nothing_else() {
    let sandbox = game_sandbox("directory-forms");
    sandbox.write(
        "README.md",
        "# core-mock\n\nSee `register@close-combat@game-design`, `path@gdd@close-combat/` and \
         `path@close-combat@README.md`.\n",
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("`register@close-combat@game-design` names the entry `close-combat`")
            && stdout.contains("`path@gdd@close-combat/` cites an entry by its path")
            && stdout.contains("`path@close-combat@README.md` cites an entry by its path"),
        "{stdout}"
    );
    assert_eq!(
        stdout
            .matches("cite the entry as `subject@gdd@close-combat`")
            .count()
            + stdout
                .matches("cite it as `subject@gdd@close-combat`")
                .count(),
        3,
        "{stdout}"
    );
}

/// The claim: `commits` judges a message against the entry anchors of its own commit's tree, or
/// of its first parent's: a message naming an entry the commit adds resolves, one naming it from
/// the commit that deletes it resolves against the parent, and one naming it after resolves to
/// nothing: AC5 of the discussion that made the shape declarable.
#[test]
fn commits_reads_the_entry_anchors_of_each_commit_s_own_tree() {
    let history = History::new("directory-commits");
    tiny_project(&history, false);
    let manifest = GAME_MANIFEST.replace("name = \"core-mock\"", "name = \"tiny\"");
    history.write("knowledge-architect.toml", &manifest);
    for (rel, text) in &GAME_FILES[1..] {
        history.write(rel, text);
    }
    let base = history.commit("The project is created\n");
    history.write(
        "game-design/high-magic/README.md",
        "# Magic\n\n- [Spells](spells.md)\n",
    );
    history.write(
        "game-design/high-magic/spells.md",
        "# Spells\n\n### A fireball costs mana `##fireball-cost`\n",
    );
    history.commit(
        "The subject `subject@gdd@high-magic` holds `game-design@high-magic@fireball-cost`\n",
    );
    for rel in [
        "game-design/high-magic/README.md",
        "game-design/high-magic/spells.md",
    ] {
        history.remove(rel);
    }
    history.commit("The subject `subject@gdd@high-magic` leaves, with `game-design@high-magic@fireball-cost`\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let late = history.commit("Nothing names `game-design@high-magic@fireball-cost` any more\n");
    let (stdout, stderr, code) = history.run(&["commits", &format!("{base}..HEAD")]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains(&late) && stdout.contains("`game-design@high-magic@fireball-cost`"),
        "{stdout}"
    );
}

/// The claim: an `opt-in` register the project attaches to a Component is carried by that
/// Component at its `docs/<dir>/`, so the same game design sits under the root's docs/ with no
/// location, its subjects cited as entries of the root, and a Component that names it and lacks
/// its home is reported, per `design@model@a-component-carries-what-a-project-attaches`.
#[test]
fn a_component_carries_the_register_a_project_attaches_to_it() {
    let manifest = GAME_MANIFEST
        .replace(
            "[locations.gdd]\npath = \"game-design\"\nregisters = [\"subject\"]\n\n",
            "",
        )
        .replace(
            "dir = \".\"\ncarries = [\"game-design\"]\n",
            "dir = \"game-design\"\ncarries = [\"game-design\"]\ncomponents = [\"core-mock\"]\n",
        );
    assert!(
        manifest.contains("components = [\"core-mock\"]"),
        "{manifest}"
    );
    let files: Vec<(String, String)> = GAME_FILES[1..]
        .iter()
        .map(|(rel, text)| {
            (
                format!("docs/{rel}"),
                text.replace("subject@gdd@", "subject@core-mock@")
                    .replace("register@gdd@", "register@core-mock@"),
            )
        })
        .chain(std::iter::once((
            "knowledge-architect.toml".to_string(),
            manifest,
        )))
        .collect();
    let seeded: Vec<(&str, &str)> = files
        .iter()
        .map(|(rel, text)| (rel.as_str(), text.as_str()))
        .collect();
    let sandbox = Sandbox::seeded("component-attached", "core", &seeded);
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    for line in [
        "  subject  docs/game-design/",
        "close-combat  subject  docs/game-design/close-combat",
    ] {
        assert!(out.lines().any(|l| l == line), "no line `{line}`:\n{out}");
    }
    let (out, stderr, code) = sandbox.run(&["show", "subject@core-mock@close-combat"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.contains("docs/game-design/close-combat/ranged.md:5"),
        "{out}"
    );

    for rel in [
        "README.md",
        "index.md",
        "close-combat/README.md",
        "close-combat/melee.md",
        "close-combat/ranged.md",
        "war-economy/README.md",
        "war-economy/resources.md",
    ] {
        std::fs::remove_file(sandbox.path(&format!("docs/game-design/{rel}"))).expect("a file");
    }
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("the anchor `core-mock` carries no subject directory"),
        "{stdout}"
    );
}

/// The claim: a register at its carrier's own path has no single-file shape, so a file beside the
/// location named like it is an ordinary document of the anchor that holds it, never a second
/// home, for a heading register and for a file register alike, per
/// `design@model@home-at-the-carrier-path`.
#[test]
fn a_register_at_its_carrier_s_path_claims_no_file_beside_it() {
    let manifest = "[project]\nchecker-version = \"fixture\"\nname = \"core-mock\"\n\
        components = []\n\n\
        [locations.rules]\npath = \"rules\"\nregisters = [\"rule\"]\n\n\
        [locations.notes]\npath = \"notes\"\nregisters = [\"note\"]\n\n\
        [registers.rule]\nscope = \"opt-in\"\nshape = \"heading\"\nlevel = 3\ndir = \".\"\n\n\
        [registers.note]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \".\"\n\n\
        [walk]\nskip-dirs = []\nskip-files = []\n\n[agents]\nharness = []\n";
    let sandbox = Sandbox::seeded(
        "carrier-path-no-file",
        "core",
        &[
            ("knowledge-architect.toml", manifest),
            ("rules/README.md", "# Rules\n\n- [Core](core.md)\n"),
            ("rules/core.md", "# Core\n\n### A rule `##a-rule`\n"),
            ("notes/README.md", "# Notes\n"),
            (
                "rules.md",
                "# About the rules\n\nSee `rule@rules@a-rule`.\n",
            ),
            ("notes.md", "# About the notes\n"),
        ],
    );
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

/// The claim: what has no `register` form is refused with the form that names it: a register only
/// entries carry is named by the entry, and a register attached to Components by one of them,
/// under `register` and under a `path` through `*`; and a register at its carrier's own path is
/// named by the repair of an undefined entry as that carrier's own directory.
#[test]
fn a_home_with_no_register_form_is_refused_with_the_form_that_names_it() {
    let manifest = GAME_MANIFEST.replace(
        "[walk]",
        "[registers.lore]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"lore\"\n\
         components = [\"core-mock\"]\n\n[walk]",
    );
    let sandbox = game_sandbox("no-register-form");
    sandbox.write("knowledge-architect.toml", &manifest);
    sandbox.write("docs/lore/README.md", "# Lore\n");
    sandbox.write(
        "README.md",
        "# core-mock\n\nSee `register@gdd@game-design`, `register@*@lore`, `path@*@docs/lore/` \
         and `subject@gdd@nowhere`.\n",
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for what in [
        "`register@gdd@game-design` names the game-design register, whose homes are the entries \
         of the subject register",
        "cite the entry whose home it is, as `subject@<anchor>@<entry>`",
        "`register@*@lore` names `*`, which names a component register's homes, and the lore \
         register is attached to core-mock",
        "`path@*@docs/lore/` names the homes of the lore register by their path",
        "write `register@<component>@lore` for one of core-mock",
        "define it in the home at its own directory of `gdd`",
    ] {
        assert!(stdout.contains(what), "no finding `{what}`:\n{stdout}");
    }
}

/// The game manifest with a second location, `zz` at aa-lore/, carrying the same Directory
/// register at its own path, so one register keeps two homes.
fn two_homes_manifest() -> String {
    GAME_MANIFEST.replace(
        "[registers.subject]",
        "[locations.zz]\npath = \"aa-lore\"\nregisters = [\"subject\"]\n\n[registers.subject]",
    )
}

/// The claim: the order of `design@model@a-placed-name-reads-as-nothing-else` decides every
/// clash: a spec before any declared entry, even where a refused entry of another register holds
/// the name first; among one register's entries, the first by path, whichever home holds it; and
/// an entry named `index` is refused, since it is no id of two segments and would read as its own
/// home's listing.
#[test]
fn an_entry_s_name_gives_way_to_a_spec_and_to_an_earlier_path() {
    let sandbox = game_sandbox("entry-name-order");
    let manifest = two_homes_manifest().replace(
        "[walk]",
        "[locations.world]\npath = \"world\"\nregisters = [\"area\"]\n\n\
         [registers.area]\nscope = \"opt-in\"\nshape = \"directory\"\ndir = \".\"\n\
         carries = [\"game-design\"]\n\n[walk]",
    );
    sandbox.write("knowledge-architect.toml", &manifest);
    sandbox.write("aa-lore/README.md", "# Lore\n");
    sandbox.write("aa-lore/close-combat/README.md", "# Combat lore\n");
    sandbox.write("world/README.md", "# World\n");
    sandbox.write("world/war-economy/README.md", "# A market\n");
    sandbox.write(
        "docs/plans/specs/war-economy.md",
        "# A spec named war-economy\n",
    );
    sandbox.write("game-design/index/README.md", "# An entry named index\n");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    for what in [
        // aa-lore/close-combat sorts before game-design/close-combat.
        "the subject entry `close-combat` is no anchor: its name is also the name of an entry of \
         `subject`",
        // The area entry is refused for the spec, and the subject entry too, for the spec
        // rather than for the refused area entry.
        "the area entry `war-economy` is no anchor: its name is also the name of a spec",
        "the subject entry `war-economy` is no anchor: its name is also the name of a spec",
        "the subject entry `index` is no anchor: its name is not two or more lower-case words \
         joined by hyphens",
    ] {
        assert!(stdout.contains(what), "no finding `{what}`:\n{stdout}");
    }
    assert_eq!(
        stdout.matches("entry `close-combat` is no anchor").count(),
        1,
        "{stdout}"
    );
    assert!(
        stdout.contains("game-design/close-combat  the subject entry `close-combat`"),
        "the later path is the one refused:\n{stdout}"
    );
}

/// The claim: a Directory register kept by two homes lists in each home's index the entries of
/// that home alone, and an entry of a component-scoped Directory register is placed under every
/// component's `docs/<dir>/`.
#[test]
fn each_home_of_a_directory_register_lists_its_own_entries() {
    let sandbox = game_sandbox("directory-two-homes");
    sandbox.write("knowledge-architect.toml", &two_homes_manifest());
    sandbox.write("aa-lore/README.md", "# Lore\n");
    sandbox.write("aa-lore/high-magic/README.md", "# Magic\n");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let here = std::fs::read_to_string(sandbox.path("game-design/index.md")).expect("an index");
    let there = std::fs::read_to_string(sandbox.path("aa-lore/index.md")).expect("an index");
    assert!(
        here.contains("\n2 entries\n") && !here.contains("high-magic"),
        "{here}"
    );
    assert!(
        there.contains("\n1 entry\n") && there.contains("| high-magic |"),
        "{there}"
    );

    let manifest = GAME_MANIFEST.replace(
        "[walk]",
        "[registers.area]\nscope = \"component\"\nshape = \"directory\"\ndir = \"areas\"\n\
         carries = [\"game-design\"]\n\n[walk]",
    );
    sandbox.write("knowledge-architect.toml", &manifest);
    sandbox.write("docs/areas/README.md", "# Areas\n");
    sandbox.write(
        "docs/areas/far-north/README.md",
        "# North\n\n- [Paths](paths.md)\n",
    );
    sandbox.write(
        "docs/areas/far-north/paths.md",
        "# Paths\n\n### A path `##a-path`\n",
    );
    std::fs::remove_dir_all(sandbox.path("aa-lore")).expect("the second home");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let (out, stderr, code) = sandbox.run(&["anchors"]);
    assert_eq!(code, 0, "{out}{stderr}");
    assert!(
        out.lines()
            .any(|l| l == "far-north  area  docs/areas/far-north"),
        "{out}"
    );
}

/// The claim: an entry's name is no anchor word, so a span headed by it is silent, and a path of
/// either path kind to an entry's directory or README, through the escape anchor or as a
/// `planned` path the tree now holds, is refused with the entry's reference as the repair.
#[test]
fn every_path_to_an_entry_s_directory_names_the_entry() {
    let sandbox = game_sandbox("entry-path-forms");
    sandbox.write(
        "README.md",
        "# core-mock\n\nSee `close-combat@north` and `path@elsewhere@game-design/close-combat/README.md`.\n",
    );
    sandbox.write(
        "docs/plans/specs/a-plan.md",
        "# A plan\n\nIt creates `planned@gdd@close-combat/`.\n",
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(!stdout.contains("close-combat@north"), "{stdout}");
    for span in [
        "`path@elsewhere@game-design/close-combat/README.md` cites an entry by its path",
        "`planned@gdd@close-combat/` cites an entry by its path",
    ] {
        assert!(stdout.contains(span), "no finding `{span}`:\n{stdout}");
    }
    assert!(
        stdout
            .matches("cite it as `subject@gdd@close-combat`")
            .count()
            >= 2,
        "{stdout}"
    );
}

// ---- `worklist`, per `design@query@worklist-writes-and-verifies-the-reports` and
// `design@query@worklist-check-judges-one-report` ----

/// The manifest of the worklist project: the root `core-mock`, a Component `widget` whose design
/// home is a directory, and a location `notes` carrying a declared file register with groups and a
/// declared Directory register, `topic`, whose entry anchors the tool constructs, each carrying a
/// declared heading register `rule`.
const WORKLIST_MANIFEST: &str = "[project]\nchecker-version = \"fixture\"\nname = \"core-mock\"\n\
    components = [\"widget\"]\n\n\
    [locations.notes]\npath = \"notes\"\nregisters = [\"reading\", \"topic\"]\n\n\
    [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\
    sections = [\"Reading\"]\n\n\
    [registers.topic]\nscope = \"opt-in\"\nshape = \"directory\"\ndir = \"topics\"\n\
    carries = [\"rule\"]\n\n\
    [registers.rule]\nscope = \"opt-in\"\nshape = \"heading\"\ndir = \"rules\"\nlevel = 2\n\n\
    [walk]\nskip-dirs = []\nskip-files = []\n\n[agents]\nharness = []\n";

/// The root's design home, one file: a head before any subject, then two subjects. The second
/// subject's head is long, so that a small bound puts its cluster above it, and the root's search
/// unit above half of a bound the others fit under.
const WORKLIST_DESIGN: &str = "# Design — core-mock\n\n\
    ### One decision `##one-decision`\n\nA decision under no subject. The design it did not keep is \
    `rejected@core-mock@other-way`.\n\n\
    ## First subject\n\n\
    ### A first head `##first-head`\n\nIt serves `goal@fine-goal`.\n\n\
    ## Second subject\n\n\
    ### A long head `##long-head`\n\n\
    one two three four five six seven eight nine ten one two three four five six seven eight nine \
    ten one two three four five six seven eight nine ten one two three four five six seven eight \
    nine ten one two three four five six seven eight nine ten one two three four five six seven \
    eight nine ten one two three four five six seven eight nine ten one two three four five six.\n";

const WORKLIST_GOALS: &str = "# Goals — core-mock\n\nThe root's prose.\n\n\
    ## A fine goal `##fine-goal`\n\nIt refines `goal@middle-goal`.\n\n\
    ## A middle goal `##middle-goal`\n\nIt refines `goal@top-goal`.\n\n\
    ## A top goal `##top-goal`\n\nThe top.\n";

const WORKLIST_TRIPWIRES: &str = "# Tripwires — core-mock\n\n\
    ## Guarding `design@core-mock@first-head`: a premise `##first-premise`\n\n\
    **Fires when:** x.\n";

fn worklist_issue(cites: &str) -> String {
    format!(
        "---\nkind: todo\n---\n# An issue\n\n## Summary\n\nIt names {cites}.\n\n## Details\n\n\
         ### What\n\nx\n\n### Why it matters\n\nx\n\n### What would close it\n\nx\n"
    )
}

/// The worklist project, and a directory outside it for the work list, which does not exist yet.
///
/// Beside the entries above: an issue of the root citing heads of both its subjects, one of
/// `widget` citing a head of the root, and a topic of `notes`, `alpha-topic`,
/// whose rule is an entry of an anchor the tool constructs.
fn worklist_project(tag: &str) -> (Sandbox, PathBuf) {
    let on_goal = worklist_issue("`goal@top-goal`");
    let on_head = worklist_issue("`design@core-mock@first-head`");
    let on_both = worklist_issue("`design@core-mock@first-head` and `design@core-mock@long-head`");
    let on_the_root = worklist_issue("`design@core-mock@first-head`");
    let sandbox = Sandbox::seeded(
        tag,
        "core",
        &[
            ("knowledge-architect.toml", WORKLIST_MANIFEST),
            ("docs/design.md", WORKLIST_DESIGN),
            ("docs/goals.md", WORKLIST_GOALS),
            ("docs/tripwires.md", WORKLIST_TRIPWIRES),
            ("docs/open-issues/on-the-goal.md", &on_goal),
            ("docs/open-issues/on-the-head.md", &on_head),
            ("docs/open-issues/on-both-subjects.md", &on_both),
            ("widget/README.md", "# widget\n\nA Component.\n"),
            ("widget/docs/open-issues/on-the-root.md", &on_the_root),
            ("widget/docs/rejected.md", "# Rejected — widget\n"),
            ("widget/docs/tripwires.md", "# Tripwires — widget\n"),
            (
                "widget/docs/open-issues/README.md",
                "# Open issues — widget\n",
            ),
            (
                "widget/docs/design/README.md",
                "# Design — widget\n\n- [Parts](parts.md)\n",
            ),
            (
                "widget/docs/design/parts.md",
                "# Parts\n\n## The gears\n\n### Gears turn `##gears-turn`\n\nThey do.\n\n\
                 ## The springs\n\n### Springs coil `##springs-coil`\n\nThey do.\n",
            ),
            ("notes/readings/README.md", "# Readings\n"),
            (
                "notes/readings/top-reading.md",
                "# Top\n\n## Reading\n\nx\n",
            ),
            (
                "notes/readings/loops/loop-reading.md",
                "# Loop\n\n## Reading\n\nx\n",
            ),
            ("notes/topics/README.md", "# Topics\n"),
            (
                "notes/topics/alpha-topic/README.md",
                "# Alpha\n\n- [Rules](rules.md)\n",
            ),
            (
                "notes/topics/alpha-topic/rules.md",
                "# Rules — alpha\n\n## An alpha rule `##alpha-rule`\n\nx\n",
            ),
        ],
    );
    // The generated indexes of the issue homes and of the topics, written as the tree holds them.
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    let out = std::env::temp_dir().join(format!("knowledge-{tag}-out-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    (sandbox, out)
}

/// Every report of a work list, as group name → the references listed under it, a cluster or a
/// remainder under `## Cluster: `, a goals file under `## Goals: `.
fn worklist_clusters(out: &Path) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut clusters = std::collections::BTreeMap::new();
    let mut agents: Vec<_> = std::fs::read_dir(out)
        .expect("the work list")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    agents.sort_by_key(|e| e.file_name());
    for agent in agents {
        let report = std::fs::read_to_string(agent.path().join("report.md")).expect("a report");
        let mut current: Option<String> = None;
        for line in report.lines() {
            if let Some(name) = line
                .strip_prefix("## Cluster: ")
                .or_else(|| line.strip_prefix("## Goals: "))
            {
                current = Some(name.to_string());
                clusters.entry(name.to_string()).or_insert_with(Vec::new);
            } else if let (Some(name), Some(rest)) = (&current, line.strip_prefix("- ")) {
                let reference = rest.split(" | ").next().expect("a reference");
                clusters
                    .get_mut(name)
                    .expect("a cluster")
                    .push(reference.to_string());
            }
        }
    }
    clusters
}

/// The agents' lists together hold every entry of the record, in both shapes of a design home,
/// under no subject, in a declared file register's groups; a cluster holds no goal; the goals of a
/// search unit are graded in the group of their goals file; and an issue citing only a goal is in
/// its search unit's remainder. Mutation checked: in `graph` of `path@query@src/graph.rs`, the
/// test `!held.contains(&i)` of the remainder made `false`, which leaves the issue on the goal in
/// no group; caught.
#[test]
fn worklist_lists_every_entry_of_the_record_in_its_search_unit_s_groups() {
    let (sandbox, out) = worklist_project("worklist-every-entry");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let listed: std::collections::BTreeSet<String> =
        std::fs::read_to_string(out.join("agents.txt"))
            .expect("agents.txt")
            .lines()
            .map(|l| l.split(' ').nth(2).expect("a reference").to_string())
            .collect();
    let every: std::collections::BTreeSet<String> =
        WORKLIST_ENTRIES.iter().map(|s| s.to_string()).collect();
    assert_eq!(listed, every);
    let clusters = worklist_clusters(&out);
    assert_eq!(
        clusters["core-mock :: First subject"],
        [
            "design@core-mock@first-head",
            "issue@core-mock@on-both-subjects",
            "issue@core-mock@on-the-head",
            "tripwire@core-mock@first-premise",
        ]
    );
    assert_eq!(
        clusters["core-mock :: docs/design.md"],
        [
            "design@core-mock@one-decision",
            "rejected@core-mock@other-way"
        ]
    );
    assert_eq!(
        clusters["widget :: The gears"],
        ["design@widget@gears-turn"]
    );
    assert_eq!(
        clusters["notes :: notes/readings/loops"],
        ["reading@notes@loop-reading"]
    );
    assert_eq!(
        clusters["core-mock :: (remainder)"],
        ["issue@core-mock@on-the-goal"]
    );
    assert_eq!(
        clusters["core-mock :: docs/goals.md"],
        ["goal@fine-goal", "goal@middle-goal", "goal@top-goal",]
    );
}

/// Every entry of the worklist project's record.
const WORKLIST_ENTRIES: &[&str] = &[
    "design@core-mock@one-decision",
    "design@core-mock@first-head",
    "design@core-mock@long-head",
    "design@widget@gears-turn",
    "design@widget@springs-coil",
    "goal@fine-goal",
    "goal@middle-goal",
    "goal@top-goal",
    "rejected@core-mock@other-way",
    "tripwire@core-mock@first-premise",
    "issue@core-mock@on-the-goal",
    "issue@core-mock@on-the-head",
    "issue@core-mock@on-both-subjects",
    "issue@widget@on-the-root",
    "reading@notes@top-reading",
    "reading@notes@loop-reading",
    "topic@notes@alpha-topic",
    "rule@alpha-topic@alpha-rule",
];

/// A work list written by `--out` with `args`, its stdout, and its directory; `tag` names it.
fn worklist_out(tag: &str, args: &[&str]) -> (Sandbox, PathBuf, String) {
    let (sandbox, out) = worklist_project(tag);
    let out_text = out.to_string_lossy().into_owned();
    let mut all = vec!["worklist", "--out", out_text.as_str()];
    all.extend_from_slice(args);
    let (stdout, stderr, code) = sandbox.run(&all);
    assert_eq!(code, 0, "{stdout}{stderr}");
    (sandbox, out, stdout)
}

/// `agents.txt` of a work list, as each agent with the references it grades.
fn worklist_agents(out: &Path) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut agents: std::collections::BTreeMap<String, Vec<String>> = Default::default();
    for line in std::fs::read_to_string(out.join("agents.txt"))
        .expect("agents.txt")
        .lines()
    {
        let mut parts = line.split(' ');
        let agent = parts.next().expect("an agent").to_string();
        let reference = parts.nth(1).expect("a reference").to_string();
        agents.entry(agent).or_default().push(reference);
    }
    agents
}

/// The search unit of a reference of the worklist project: its anchor's, `alpha-topic` being an
/// entry anchor of `notes`.
fn worklist_unit(reference: &str) -> &str {
    match reference.split('@').nth(1).expect("an anchor") {
        "alpha-topic" => "notes",
        anchor => anchor,
    }
}

/// One agent line of a work list's stdout: its agent, its own load, and its items with the load
/// each was packed by.
type AgentLine = (String, usize, Vec<(String, usize)>);

/// Each agent line of a work list's stdout.
fn worklist_lines(stdout: &str) -> Vec<AgentLine> {
    stdout
        .lines()
        .filter(|l| l.starts_with("agent-"))
        .map(|line| {
            let mut parts = line.splitn(3, "  ");
            let agent = parts.next().expect("an agent").to_string();
            let load = parts
                .next()
                .and_then(|w| w.strip_suffix(" words"))
                .and_then(|w| w.parse().ok())
                .unwrap_or_else(|| panic!("an agent's load: {line}"));
            let items = parts
                .next()
                .expect("the items")
                .split("; ")
                .map(|item| {
                    let (name, words) = item
                        .rsplit_once(" (")
                        .unwrap_or_else(|| panic!("an item with no load: {line}"));
                    let words = words
                        .strip_suffix(" words)")
                        .and_then(|w| w.parse().ok())
                        .unwrap_or_else(|| panic!("an item with no load: {line}"));
                    (name.to_string(), words)
                })
                .collect();
            (agent, load, items)
        })
        .collect()
}

/// The claim: every entry is graded in exactly one agent, a split search unit's entry that cites
/// heads of two of its clusters included. At a bound of 120 the root's search unit is split, and
/// `on-both-subjects` cites a head of each of its subjects. Mutation checked: in `assign` of
/// `path@query@src/graph.rs`, `holders.into_iter().filter(|(_, h)| h.len() > 1)` made
/// `holders.into_iter().filter(|(_, h)| h.len() > 1 && false)`, which leaves the issue in both
/// clusters, so two agents list it; caught.
#[test]
fn worklist_grades_every_entry_in_exactly_one_agent() {
    let (_sandbox, out, stdout) = worklist_out("worklist-one-agent", &["--bound", "120"]);
    assert!(
        stdout.contains("core-mock :: First subject (")
            && stdout.contains("core-mock :: Second subject ("),
        "the root's search unit is not split: {stdout}"
    );
    let mut count: std::collections::BTreeMap<String, usize> = Default::default();
    for references in worklist_agents(&out).values() {
        for reference in references {
            *count.entry(reference.clone()).or_default() += 1;
        }
    }
    for reference in WORKLIST_ENTRIES {
        assert_eq!(count.get(*reference), Some(&1), "{reference}: {count:?}");
    }
    assert_eq!(count.len(), WORKLIST_ENTRIES.len(), "{count:?}");
}

/// The claim: no agent's summed load, the sum of the loads of the items it was packed as, exceeds
/// the bound left unless it holds one item alone; and an agent's own load, which counts an entry or
/// a title line two of its groups share once, is at most that sum. Mutation checked: in `pack` of
/// `path@query@src/graph.rs`, the first-fit test `load + loads[g].0 <= bound` made `true`, which
/// puts the root's split groups together at 120; caught.
#[test]
fn worklist_keeps_every_agent_s_summed_load_under_the_bound() {
    for bound in [60, 120, 300, 400] {
        let bound_text = bound.to_string();
        let (_sandbox, _out, stdout) = worklist_out(
            &format!("worklist-summed-{bound}"),
            &["--bound", &bound_text],
        );
        for (agent, load, items) in worklist_lines(&stdout) {
            let summed: usize = items.iter().map(|(_, w)| w).sum();
            assert!(load <= summed, "{agent} at {bound}: {stdout}");
            if items.len() > 1 {
                assert!(summed <= bound, "{agent} at {bound}: {stdout}");
            }
        }
    }
}

/// The claim: a search unit whose load exceeds half the bound left shares no agent. At a bound of
/// 400 the root's search unit, of a load above 200 and under 400, fits one agent with `widget` and
/// `notes`. Mutation checked: in `plan` of `path@query@src/graph.rs`, `load * 2 > bound` made
/// `load > bound`, which packs the root with the small units; caught.
#[test]
fn worklist_gives_a_search_unit_above_half_the_bound_agents_of_its_own() {
    let (_sandbox, out, stdout) = worklist_out("worklist-half", &["--bound", "400"]);
    let root = worklist_lines(&stdout)
        .into_iter()
        .find(|(_, _, items)| items.iter().any(|(n, _)| n == "unit core-mock"))
        .unwrap_or_else(|| panic!("no agent holds the root's search unit: {stdout}"));
    assert!(
        root.2[0].1 * 2 > 400 && root.2[0].1 <= 400,
        "the root's load is not between half the bound and the bound: {stdout}"
    );
    let mut shared = 0;
    for (agent, references) in worklist_agents(&out) {
        let units: std::collections::BTreeSet<&str> = references
            .iter()
            .filter(|r| !r.starts_with("goal@"))
            .map(|r| worklist_unit(r))
            .collect();
        if units.contains("core-mock") {
            assert_eq!(units.len(), 1, "{agent} shares the root's agent: {units:?}");
        }
        shared += usize::from(units.len() > 1);
    }
    assert!(shared > 0, "no agent holds two search units: {stdout}");
}

/// The claim: an entry of an anchor the tool constructs, the rule of the topic `alpha-topic`, is
/// packed with the search unit that holds its home, `notes`, and no search unit of its own is
/// packed. Mutation checked: `search_unit` in `path@query@src/graph.rs` returning the anchor's own
/// name, which packs `unit alpha-topic` beside `unit notes`; caught.
#[test]
fn worklist_packs_a_constructed_anchor_s_entry_with_its_home_s_search_unit() {
    let (_sandbox, out, stdout) = worklist_out("worklist-constructed", &["--bound", "60"]);
    assert!(!stdout.contains("unit alpha-topic"), "{stdout}");
    let agents = worklist_agents(&out);
    let (agent, references) = agents
        .iter()
        .find(|(_, r)| r.iter().any(|r| r == "rule@alpha-topic@alpha-rule"))
        .expect("the agent of the rule");
    assert!(
        references.iter().any(|r| r == "reading@notes@top-reading"),
        "{agent}: {references:?}"
    );
    let line = stdout
        .lines()
        .find(|l| l.starts_with(&format!("{agent}  ")))
        .expect("its line");
    assert!(line.contains("unit notes ("), "{line}");
}

/// The entries file of the agent that grades `reference`, and that agent's references.
fn worklist_entries_of(out: &Path, reference: &str) -> (String, Vec<String>) {
    let (agent, references) = worklist_agents(out)
        .into_iter()
        .find(|(_, r)| r.iter().any(|r| r == reference))
        .unwrap_or_else(|| panic!("no agent grades {reference}"));
    let entries = std::fs::read_to_string(out.join(&agent).join("entries.md")).expect("entries");
    (entries, references)
}

/// The text of `entries.md` from the line `heading` to the next level-two heading.
fn worklist_section<'a>(entries: &'a str, heading: &str) -> &'a str {
    let Some(start) = entries.find(&format!("\n{heading}\n")) else {
        return "";
    };
    let rest = &entries[start + 1..];
    let end = rest[1..].find("\n## ").map_or(rest.len(), |i| i + 1);
    &rest[..end]
}

/// The claim: an issue citing a head another agent grades is given that head whole as a context
/// entry, which its agent's report and its list do not name. At a bound of 120 `widget`'s issue on
/// the root's first head is graded apart from the root's split search unit. Mutation checked: in
/// `context` of `path@query@src/graph.rs`, the filter made to keep nothing, which leaves the agent
/// no context entry; caught.
#[test]
fn worklist_gives_a_head_another_agent_grades_as_a_context_entry() {
    let (_sandbox, out, _) = worklist_out("worklist-context", &["--bound", "120"]);
    let (entries, references) = worklist_entries_of(&out, "issue@widget@on-the-root");
    assert!(
        !references
            .iter()
            .any(|r| r == "design@core-mock@first-head"),
        "the head is graded beside the issue: {references:?}"
    );
    let context = worklist_section(&entries, "## Context entries");
    assert!(
        context.contains("=== design@core-mock@first-head  docs/design.md:")
            && context.contains("It serves `goal@fine-goal`."),
        "{entries}"
    );
    // The split root's own context: the first subject's issue cites the long head, graded apart.
    let (entries, _) = worklist_entries_of(&out, "issue@core-mock@on-both-subjects");
    assert!(
        worklist_section(&entries, "## Context entries")
            .contains("=== design@core-mock@long-head  "),
        "{entries}"
    );
}

/// The claim: every cluster agent holds the title lines of the root's goals, the root alone
/// carrying the register `goal`; a goal agent grades every goal, and no entry of another register.
/// Mutation checked: in `graph` of `path@query@src/graph.rs`, the root's goals filtered by a unit
/// no anchor names, which leaves `widget`'s agent with no title line; caught.
#[test]
fn worklist_gives_cluster_agents_title_lines_and_goal_agents_the_goals() {
    let (_sandbox, out, _) = worklist_out("worklist-titles", &["--bound", "120"]);
    let root_titles = [
        "- goal@fine-goal  A fine goal",
        "- goal@middle-goal  A middle goal",
        "- goal@top-goal  A top goal",
    ];
    let (widget, _) = worklist_entries_of(&out, "design@widget@gears-turn");
    let titles = worklist_section(&widget, "## Goal titles");
    for line in root_titles {
        assert!(titles.contains(line), "{line}: {widget}");
    }
    let (root, references) = worklist_entries_of(&out, "design@core-mock@long-head");
    assert!(
        references.iter().all(|r| worklist_unit(r) == "core-mock"),
        "{references:?}"
    );
    let titles = worklist_section(&root, "## Goal titles");
    for line in root_titles {
        assert!(titles.contains(line), "{line}: {root}");
    }
    let mut goals: Vec<String> = Vec::new();
    for (agent, references) in worklist_agents(&out) {
        if references.iter().any(|r| r.starts_with("goal@")) {
            assert!(
                references.iter().all(|r| r.starts_with("goal@")),
                "{agent} grades goals and other entries: {references:?}"
            );
            goals.extend(references);
        }
    }
    goals.sort();
    assert_eq!(
        goals,
        ["goal@fine-goal", "goal@middle-goal", "goal@top-goal",]
    );
}

/// The claim: the words of the work come out of the bound, so a bound of 400 with a work of 300
/// words packs as a bound of 100 does; `work.txt` lists the work's file and the subjects, and the
/// one subject `whole-work` where none is named; a file of the work that cannot be read is refused
/// with nothing written; and the line of the bound the work leaves is printed. Mutations checked,
/// in `path@query@src/worklist.rs`: in `work_load`, `(paths, words, bound.saturating_sub(words))`
/// made `(paths, words, bound + 0 * words)`, which packs as 400; in `write`, the read of a file of
/// the work made to yield an empty text on an error, `std::fs::read_to_string(path)` followed by
/// `.or_else(|_| Ok::<String, std::io::Error>(String::new()))`, which writes a work list over a
/// file that does not exist; and `if !work_paths.is_empty() {` made `if false {`, which prints no
/// line of the bound; each caught.
#[test]
fn worklist_takes_the_work_s_words_out_of_the_bound() {
    let (sandbox, out) = worklist_project("worklist-work");
    let work = out.with_extension("work.md");
    std::fs::write(&work, "word ".repeat(300)).expect("the work");
    let work_text = work.to_string_lossy().into_owned();
    let lines = |dir: &Path, args: &[&str]| -> Vec<String> {
        let dir_text = dir.to_string_lossy().into_owned();
        let mut all = vec!["worklist", "--out", dir_text.as_str()];
        all.extend_from_slice(args);
        let (stdout, stderr, code) = sandbox.run(&all);
        assert_eq!(code, 0, "{stdout}{stderr}");
        stdout
            .lines()
            .filter(|l| l.starts_with("agent-") || l.starts_with("notice:"))
            .map(String::from)
            .collect()
    };
    let with_work = lines(
        &out.join("with"),
        &[
            "--bound",
            "400",
            "--work",
            &work_text,
            "--subject",
            "first-thread",
            "--subject",
            "second-thread",
        ],
    );
    let at_100 = lines(&out.join("at-100"), &["--bound", "100"]);
    let at_400 = lines(&out.join("at-400"), &["--bound", "400"]);
    assert_ne!(at_100, at_400, "the two bounds pack alike");
    assert_eq!(with_work, at_100);
    let (stdout, _, _) = sandbox.run(&[
        "worklist",
        "--out",
        &out.join("line").to_string_lossy(),
        "--bound",
        "400",
        "--work",
        &work_text,
    ]);
    assert!(
        stdout.contains("the work holds 300 words, which leaves 100 of the 400-word bound"),
        "{stdout}"
    );
    let listed = std::fs::read_to_string(out.join("with").join("work.txt")).expect("work.txt");
    let real = work.canonicalize().expect("the work's path");
    assert_eq!(
        listed,
        format!(
            "work {}\nsubject first-thread\nsubject second-thread\n",
            real.display()
        )
    );
    let listed = std::fs::read_to_string(out.join("at-100").join("work.txt")).expect("work.txt");
    assert_eq!(listed, "subject whole-work\n");

    let missing = out.with_extension("no-such-work.md");
    let missing_text = missing.to_string_lossy().into_owned();
    let refused = out.join("refused");
    let refused_text = refused.to_string_lossy().into_owned();
    let (stdout, stderr, code) =
        sandbox.run(&["worklist", "--out", &refused_text, "--work", &missing_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("Nothing was written"), "{stderr}");
    assert!(!refused.exists());
    // A subject that is no id is refused while parsing.
    let (stdout, stderr, code) =
        sandbox.run(&["worklist", "--out", &refused_text, "--subject", "Not An Id"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(!refused.exists());
    let _ = std::fs::remove_file(&work);
}

/// The claim: the default bound is 40,000 words: a cluster whose own entries hold more is named by
/// the notice of a 40000-word bound. Mutation checked: `DEFAULT_WORKLIST_BOUND` in
/// `path@cli@src/cli/mod.rs` made 40001, and made 26000; each caught.
#[test]
fn worklist_s_default_bound_is_40000_words() {
    let (sandbox, out) = worklist_project("worklist-default-bound");
    sandbox.write(
        "docs/design.md",
        &format!(
            "{WORKLIST_DESIGN}\n## Third subject\n\n### A huge head `##huge-head`\n\n{}\n",
            "word ".repeat(40_000)
        ),
    );
    sandbox.stage();
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains(
            "notice: the cluster \"core-mock :: Third subject\" holds 40005 words, above the \
             40000-word bound"
        ),
        "{stdout}"
    );
}

/// AC3: a cluster above the bound goes to an agent alone, and the output names it with its
/// notice. Mutation checked: the partition of groups above the bound removed from `pack`, which
/// packs the long head's cluster with the others.
#[test]
fn worklist_sends_a_cluster_above_the_bound_alone_with_a_notice() {
    let (sandbox, out) = worklist_project("worklist-above");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text, "--bound", "60"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains("notice: the cluster \"core-mock :: Second subject\" holds"),
        "{stdout}"
    );
    let line = stdout
        .lines()
        .find(|l| l.contains("Second subject") && l.starts_with("agent-"))
        .expect("the agent of the long cluster");
    assert!(!line.contains(';'), "it shares its agent: {line}");
}

/// The claim: each agent's line names every item it was packed as with that item's load, a search
/// unit packed whole as `unit <name>` and a group of a split one by its name; a group alone on its
/// agent carries the agent's load. Mutation checked: in `plan` of `path@query@src/graph.rs`,
/// `.map(|&u| (format!("unit {}", small[u].0), small[u].2))` made
/// `.map(|&u| (small[u].0.clone(), small[u].2))`, which drops the word `unit`; caught.
#[test]
fn worklist_prints_each_item_with_its_load() {
    let (_sandbox, _out, stdout) = worklist_out("worklist-loads", &["--bound", "60"]);
    let lines = worklist_lines(&stdout);
    let (_, load, items) = lines
        .iter()
        .find(|(_, _, items)| {
            items
                .iter()
                .any(|(n, _)| n == "core-mock :: Second subject")
        })
        .unwrap_or_else(|| panic!("no agent of the long cluster: {stdout}"));
    assert_eq!(items, &[("core-mock :: Second subject".to_string(), *load)]);
    assert!(
        lines
            .iter()
            .any(|(_, _, items)| items.iter().any(|(n, _)| n == "unit notes")),
        "{stdout}"
    );
}

/// AC4: `worklist --out` over a tree that stops before the last phase exits 2 and writes
/// nothing; the test plants a phase-2 finding, a missing home. Mutation checked: the call of
/// `complete_working_tree` removed from `write`, which writes the work list.
#[test]
fn worklist_refuses_an_incomplete_model_and_writes_nothing() {
    let (sandbox, out) = worklist_project("worklist-incomplete");
    std::fs::remove_file(sandbox.path("widget/docs/tripwires.md")).expect("a home to remove");
    sandbox.stage();
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("the model is incomplete"), "{stderr}");
    assert!(!out.exists());
}

/// The refusals before the first write: a directory inside the project, a directory that holds
/// a file, and a seed that names no entry. Mutation checked: the inside-the-project test removed
/// from `write`, which writes into the project.
#[test]
fn worklist_refuses_a_directory_it_must_not_write_and_an_unknown_seed() {
    let (sandbox, out) = worklist_project("worklist-refusals");
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", "inside"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("inside the project"), "{stderr}");
    assert!(!sandbox.path("inside").exists());

    std::fs::create_dir_all(&out).expect("the directory");
    std::fs::write(out.join("left.txt"), "x").expect("a file in it");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("not empty"), "{stderr}");
    std::fs::remove_dir_all(&out).expect("cleared");

    let (stdout, stderr, code) = sandbox.run(&[
        "worklist",
        "--out",
        &out_text,
        "--seed",
        "design@core-mock@no-such-head",
    ]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("names no entry"), "{stderr}");
    assert!(!out.exists());
}

/// The snapshot of a work list, as each reference with its own text and its inbound list, in the
/// file's order.
fn worklist_snapshot(out: &Path) -> Vec<(String, String, String)> {
    let text = std::fs::read_to_string(out.join("shown.md")).expect("shown.md");
    let mut entries = Vec::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let Some(reference) = line.strip_prefix("=== ") else {
            continue;
        };
        let mut blocks = Vec::new();
        for _ in 0..2 {
            assert_eq!(
                lines.next(),
                Some(""),
                "{reference}: a blank line before each block"
            );
            let open = lines.next().expect("a fence");
            let fence = open.strip_suffix("text").expect("a fence opening `text`");
            let mut block = Vec::new();
            for line in lines.by_ref() {
                if line == fence {
                    break;
                }
                block.push(line);
            }
            blocks.push(block.join("\n"));
        }
        let inbound = blocks.pop().expect("an inbound list");
        let own = blocks.pop().expect("an own text");
        entries.push((reference.to_string(), own, inbound));
    }
    entries
}

/// The first eight words of `text`, its whitespace collapsed, or all of them where it has fewer.
fn first_words(text: &str) -> String {
    text.split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
}

/// A quotation `--verify` accepts for each entry of a work list: the first eight words of its own
/// body, below the `<reference>  <site>` line of its own text, or the whole of a shorter body.
fn worklist_quotes(out: &Path) -> std::collections::BTreeMap<String, String> {
    worklist_snapshot(out)
        .into_iter()
        .map(|(reference, own, _)| {
            let body = own.split_once('\n').map(|(_, b)| b).unwrap_or("");
            (reference, first_words(body))
        })
        .collect()
}

/// Fill every verdict line of every report of a work list as `fields(reference, quotation)`
/// says, the reference and its seed field kept.
fn fill_reports(out: &Path, fields: impl Fn(&str, &str) -> String) {
    let quotes = worklist_quotes(out);
    for entry in std::fs::read_dir(out).expect("the work list").flatten() {
        let report = entry.path().join("report.md");
        let Ok(text) = std::fs::read_to_string(&report) else {
            continue;
        };
        let mut filled = String::new();
        for line in text.lines() {
            match line.split_once(" | verdict: ") {
                Some((head, _)) if line.starts_with("- ") => {
                    let reference = head[2..].split(" | ").next().expect("a reference");
                    filled.push_str(&format!(
                        "{head} | {}\n",
                        fields(reference, &quotes[reference])
                    ));
                }
                _ => filled.push_str(&format!("{line}\n")),
            }
        }
        std::fs::write(&report, filled).expect("a filled report");
    }
}

/// Fill every report of a work list with `unrelated`, `-`, `none`, a quotation of each entry's own
/// body and a reason.
fn fill_unrelated(out: &Path) {
    fill_reports(out, |_, quote| {
        format!(
            "verdict: unrelated | against: - | action: none | owner-question: | quote: {quote} | \
             reason: none"
        )
    });
}

/// The report holding `reference`, and its text.
fn report_with(out: &Path, reference: &str) -> (PathBuf, String) {
    for entry in std::fs::read_dir(out).expect("the work list").flatten() {
        let report = entry.path().join("report.md");
        if let Ok(text) = std::fs::read_to_string(&report) {
            if text.contains(&format!("- {reference} |")) {
                return (report, text);
            }
        }
    }
    panic!("no report lists {reference}");
}

/// The line of `reference` in `text`, a report.
fn report_line(text: &str, reference: &str) -> String {
    text.lines()
        .find(|l| l.starts_with(&format!("- {reference} |")))
        .expect("its line")
        .to_string()
}

/// `worklist --verify` exits 0 on complete reports; 1 naming the entry for an entry with no
/// verdict, one given two, a verdict outside its register's set and an entry not in the agent's
/// list, and 1 for a report missing `## Decided` or `## Met outside the task`; and 2 on a line it
/// cannot parse, its fields out of order among them. The skeleton closes each report with both
/// sections, in that order, and lists on each line the verdicts its register admits. Mutations
/// checked, in `path@query@src/worklist.rs`: in `judge_report`,
/// `for section in [DECIDED, MET_OUTSIDE] {`
/// made `for section in [MET_OUTSIDE] {`, which passes a report missing `## Decided`; the line
/// `let Some(parsed) = parse_fields(fields) else {` made
/// `let Some(parsed) = parse_fields(fields) else { continue; }; let Some(()) = Some(()) else {`,
/// which skips a line out of order and reads it as no verdict, exit 1; in `render`,
/// `verdicts(entry.role).join(", ")` made `verdicts(Role::Issue).join(", ")`, and the closing
/// `"\n{DECIDED}\n\n{MET_OUTSIDE}\n\n"` made `"\n{MET_OUTSIDE}\n\n"`; each caught.
#[test]
fn worklist_verify_names_each_gap_of_a_report() {
    let (sandbox, out) = worklist_project("worklist-verify");
    let out_text = out.to_string_lossy().into_owned();
    let (_, _, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0);
    let target = "design@core-mock@first-head";
    let (_, skeleton) = report_with(&out, target);
    assert!(
        skeleton.contains(&format!(
            "- {target} | verdict: constrains, conflicts, unrelated | against: | action: | \
             owner-question: | quote: | reason:\n"
        )) && skeleton.ends_with("\n## Decided\n\n## Met outside the task\n\n"),
        "{skeleton}"
    );
    fill_unrelated(&out);
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");

    let (path, text) = report_with(&out, target);
    let line = report_line(&text, target);
    let fields = line.split_once(" | ").expect("fields").1;
    let cases: Vec<(String, &str, i32)> = vec![
        (text.replace(&format!("{line}\n"), ""), "has no verdict", 1),
        (
            text.replace(&line, &format!("{line}\n{line}")),
            "is given 2 verdicts",
            1,
        ),
        (
            text.replace(&line, &line.replace("verdict: unrelated", "verdict: maybe")),
            "is no verdict",
            1,
        ),
        (
            text.replace(
                &line,
                &format!("{line}\n- design@core-mock@not-given | {fields}"),
            ),
            "is not an entry of",
            1,
        ),
        (
            text.replace("## Met outside the task", "## Something else"),
            "no section `## Met outside the task`",
            1,
        ),
        (
            text.replace("## Decided", "## Something else"),
            "no section `## Decided`",
            1,
        ),
        (
            text.replace(&line, &format!("- {target} | nonsense")),
            "not a line",
            2,
        ),
        (
            text.replace(
                &line,
                &line.replace(
                    " | against: - | action: none",
                    " | action: none | against: -",
                ),
            ),
            "not a line",
            2,
        ),
    ];
    for (edited, expected, want) in cases {
        std::fs::write(&path, &edited).expect("an edited report");
        let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
        assert_eq!(code, want, "{expected}: {stdout}{stderr}");
        assert!(
            stdout.contains(expected) || stderr.contains(expected),
            "{expected}: {stdout}{stderr}"
        );
        // A finding names the entry it is about, so the reader of the verdict finds its line.
        if want == 1 && !expected.starts_with("no section") && expected != "is not an entry of" {
            assert!(
                stdout
                    .lines()
                    .any(|l| l.contains(expected) && l.contains(target)),
                "{expected} does not name {target}: {stdout}"
            );
        }
    }
}

/// Every case of a verdict line `--verify` judges over a work list `--out` wrote with two
/// subjects: a quotation running from the entry's first line into its body, or from its body into
/// its inbound list, one absent from its body, or none; a verdict outside its register's set; an
/// action outside its verdict's; an `against` that `work.txt` does not list; and a missing
/// reason, each refused, and a line that meets every rule passed. Then the refusals of a
/// `work.txt` listing a subject twice and of a list it cannot read. Mutations checked, in
/// `path@query@src/worklist.rs`: `own_body` made to return the whole own text, `own` for
/// `own.split_once('\n').map(|(_, body)| body).unwrap_or("")`, which finds the quotation of the
/// first line; in `parse_snapshot`, the own text read with the inbound list after it,
/// `Shown { own, inbound }` made `Shown { own: format!("{own}\n\n{inbound}"), inbound }`, which
/// finds the quotation of the inbound list; in `judge`,
/// `} else if !body.windows(words).any(|run| run == quote.as_slice()) {` made
/// `} else if false {`; `.filter(|s| !subjects.contains(s))` made `.filter(|_| false)`; and
/// `if fields.reason.is_empty() {` made `if false {`; in `render`, the subjects of the
/// skeleton's header, `named.join(", ")`, made `String::new()`; and in `parse_work`,
/// `Some(s) if work.subjects.iter().any(|t| t == s) => {` made `Some(s) if false => {`; each
/// caught.
#[test]
fn worklist_verify_judges_every_case_of_a_verdict_line() {
    let (sandbox, out) = worklist_project("worklist-cases");
    let out_text = out.to_string_lossy().into_owned();
    let (_, _, code) = sandbox.run(&[
        "worklist",
        "--out",
        &out_text,
        "--subject",
        "one-thread",
        "--subject",
        "two-thread",
    ]);
    assert_eq!(code, 0);
    let target = "design@core-mock@first-head";
    let (_, skeleton) = report_with(&out, target);
    assert!(
        skeleton.contains("The subjects of the work: `one-thread`, `two-thread`."),
        "{skeleton}"
    );
    fill_unrelated(&out);
    let (_, own, inbound) = worklist_snapshot(&out)
        .into_iter()
        .find(|(r, _, _)| r == target)
        .expect("the target's snapshot");
    let quote = worklist_quotes(&out)[target].clone();
    // Eight words that begin on the first line, `<reference>  <site>`, and run into the body.
    let from_first_line = first_words(&own);
    // Eight words that end the body and run into the inbound list, as `show` prints both.
    let body_words: Vec<&str> = own.split_whitespace().collect();
    let from_inbound = first_words(&format!(
        "{} {inbound}",
        body_words[body_words.len() - 2..].join(" ")
    ));
    assert_eq!(from_inbound.split(' ').count(), 8, "{inbound}");
    let (path, text) = report_with(&out, target);
    let line = report_line(&text, target);
    let with = |verdict: &str, against: &str, action: &str, quote: &str, reason: &str| {
        text.replace(
            &line,
            &format!(
                "- {target} | verdict: {verdict} | against: {against} | action: {action} | \
                 owner-question: | quote: {quote} | reason: {reason}"
            ),
        )
    };
    let not_in = "the quotation is not in the entry's own text";
    let cases: Vec<(String, Option<&str>)> = vec![
        (
            with(
                "constrains",
                "one-thread, two-thread",
                "respect",
                &quote,
                "r",
            ),
            None,
        ),
        (
            with("unrelated", "-", "none", &from_first_line, "r"),
            Some(not_in),
        ),
        (
            with("unrelated", "-", "none", &from_inbound, "r"),
            Some(not_in),
        ),
        (
            with(
                "unrelated",
                "-",
                "none",
                "these eight words are in no entry at all",
                "r",
            ),
            Some(not_in),
        ),
        (
            with("unrelated", "-", "none", "", "r"),
            Some("has no quotation"),
        ),
        (
            with("fires", "one-thread", "rule-before-work", &quote, "r"),
            Some("`fires` is no verdict for the role `unit`"),
        ),
        (
            with("constrains", "one-thread", "check-reversal", &quote, "r"),
            Some("`check-reversal` is no action for the verdict `constrains`"),
        ),
        (
            with("constrains", "three-thread", "respect", &quote, "r"),
            Some("`three-thread` is no subject of the work"),
        ),
        (
            with("unrelated", "-", "none", &quote, ""),
            Some("has no reason"),
        ),
    ];
    for (edited, expected) in cases {
        std::fs::write(&path, &edited).expect("an edited report");
        let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
        match expected {
            None => assert_eq!(code, 0, "{stdout}{stderr}"),
            Some(expected) => {
                assert_eq!(code, 1, "{expected}: {stdout}{stderr}");
                assert!(
                    stdout
                        .lines()
                        .any(|l| l.contains(expected) && l.contains(target)),
                    "{expected}: {stdout}"
                );
            }
        }
    }
    std::fs::write(&path, &text).expect("the report restored");
    // A `work.txt` listing a subject twice, which would write that subject's file twice.
    let work = out.join("work.txt");
    let listed_work = std::fs::read_to_string(&work).expect("work.txt");
    std::fs::write(&work, format!("{listed_work}subject one-thread\n"))
        .expect("a repeated subject");
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(
        stderr.contains("work.txt:3: the subject one-thread is listed twice"),
        "{stderr}"
    );
    std::fs::write(&work, &listed_work).expect("work.txt restored");
    let list = out.join("agents.txt");
    let listed = std::fs::read_to_string(&list).expect("agents.txt");
    std::fs::write(&list, format!("{listed}a line of one word\n")).expect("a broken list");
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("agents.txt"), "{stderr}");
    std::fs::remove_file(&list).expect("the list removed");
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("agents.txt"), "{stderr}");
}

/// A seed and the entries that cite it are never given `unrelated`, and pass as `constrains`,
/// `none`, `-`. Mutation checked: in `judge`, `(entry.seed || entry.cites_seed)` made
/// `entry.seed`, which passes the issue citing the seed given `unrelated`; caught.
#[test]
fn worklist_verify_holds_seeds_and_their_citers() {
    let (sandbox, out) = worklist_project("worklist-seeds");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&[
        "worklist",
        "--out",
        &out_text,
        "--seed",
        "design@core-mock@first-head",
    ]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let (_, text) = report_with(&out, "issue@core-mock@on-the-head");
    assert!(
        text.contains("- issue@core-mock@on-the-head | cites seed: design@core-mock@first-head |"),
        "{text}"
    );
    fill_unrelated(&out);
    let (stdout, _, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 1, "{stdout}");
    assert!(
        stdout.contains("issue@core-mock@on-the-head cites a seed, so it is never `unrelated`"),
        "{stdout}"
    );
    assert!(
        stdout.contains("design@core-mock@first-head is a seed, so it is never `unrelated`"),
        "{stdout}"
    );
    fill_reports(&out, |_, quote| {
        format!(
            "verdict: constrains | against: - | action: none | owner-question: | quote: {quote} \
             | reason: the seed"
        )
    });
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
}

/// A directory reached through a link to the project, one that does not exist yet, is still
/// inside the project, and is refused. Mutation checked: the walk up to the deepest existing
/// ancestor removed from `absolute`, which compares the new path by its spelling and writes into
/// the project.
#[cfg(unix)]
#[test]
fn worklist_refuses_a_new_directory_under_a_link_into_the_project() {
    let (sandbox, out) = worklist_project("worklist-link");
    let link = out.with_extension("link");
    let _ = std::fs::remove_file(&link);
    std::os::unix::fs::symlink(sandbox.path(""), &link).expect("a link to the project");
    let target = link.join("new-list");
    let target_text = target.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &target_text]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("inside the project"), "{stderr}");
    assert!(!sandbox.path("new-list").exists());
    std::fs::remove_file(&link).expect("the link removed");
}

/// Each entry's body is fenced in `entries.md`, so a line of it that reads as an entry's opening
/// stays inside the fence; and a `graded above` line naming a reference no agent was given is a
/// finding. Mutations checked: the fence dropped from `render`, which leaves the body's `=== ` line
/// at the start of a line outside any fence; and the list check of `graded above` lines removed,
/// which passes the stray line.
#[test]
fn worklist_fences_each_body_and_judges_a_graded_above_line() {
    let forged = worklist_issue("`goal@top-goal`").replace(
        "x\n\n### Why",
        "=== design@core-mock@long-head  docs/design.md:1\n\n### Why",
    );
    let (sandbox, out) = worklist_project("worklist-fence");
    sandbox.write("docs/open-issues/on-the-goal.md", &forged);
    sandbox.stage();
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let (report, _) = report_with(&out, "issue@core-mock@on-the-goal");
    let entries = std::fs::read_to_string(report.with_file_name("entries.md")).expect("entries");
    let mut fenced = false;
    for line in entries.lines() {
        if line.starts_with("```") {
            fenced = !fenced;
        } else if line == "=== design@core-mock@long-head  docs/design.md:1" {
            assert!(fenced, "the body's line is outside every fence:\n{entries}");
        }
    }
    fill_unrelated(&out);
    let text = std::fs::read_to_string(&report).expect("a report");
    std::fs::write(
        &report,
        text.replace(
            "\n## Decided\n",
            "- design@core-mock@not-given | graded above\n\n## Decided\n",
        ),
    )
    .expect("an edited report");
    let (stdout, _, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 1, "{stdout}");
    assert!(
        stdout.contains("design@core-mock@not-given is not an entry of"),
        "{stdout}"
    );
}

/// The snapshot holds, for every entry of the record, the text `show` prints for it, its own text
/// and its inbound list as two fenced blocks, over the same tree. Mutation checked: in `snapshot`
/// of `path@query@src/worklist.rs`, the inbound list written as the own text,
/// `fenced("text", &inbound)` made `fenced("text", &own)`; caught.
#[test]
fn worklist_snapshot_holds_the_show_text_of_every_entry() {
    let (sandbox, out) = worklist_project("worklist-snapshot");
    // An entry holding a fence of three backticks, so the snapshot's own fence must be longer.
    sandbox.write(
        "docs/open-issues/with-a-fence.md",
        &worklist_issue("`goal@top-goal`, run as\n\n```sh\nklarch check\n```\n"),
    );
    sandbox.stage();
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let snapshot = worklist_snapshot(&out);
    let listed: std::collections::BTreeSet<String> =
        std::fs::read_to_string(out.join("agents.txt"))
            .expect("agents.txt")
            .lines()
            .map(|l| l.split(' ').nth(2).expect("a reference").to_string())
            .collect();
    let held: std::collections::BTreeSet<String> =
        snapshot.iter().map(|(r, _, _)| r.clone()).collect();
    assert_eq!(
        held, listed,
        "the snapshot holds every entry the work list lists, once"
    );
    assert_eq!(held.len(), snapshot.len(), "no entry twice");
    for (reference, own, inbound) in &snapshot {
        let (shown, stderr, code) = sandbox.run(&["show", reference]);
        assert_eq!(code, 0, "{reference}: {shown}{stderr}");
        assert_eq!(&format!("{own}\n\n{inbound}\n"), &shown, "{reference}");
        assert!(inbound.starts_with("referenced "), "{reference}: {inbound}");
    }
    assert!(
        snapshot
            .iter()
            .any(|(r, own, _)| r.ends_with("with-a-fence") && own.contains("```sh")),
        "the entry holding a fence is whole"
    );
    // The inbound lists are compared, not only the bodies: one entry here is cited.
    assert!(
        snapshot
            .iter()
            .any(|(_, _, inbound)| inbound.contains("referenced at:")),
        "no entry of the snapshot is cited"
    );
}

/// A failing run, and a run stopped on a line it cannot parse, leave none of the outputs of a
/// previous run: no verdict file, a subject's that this run's `work.txt` no longer lists included,
/// and no generated lines. Mutations checked, in `path@query@src/worklist.rs`: the call
/// `remove_outputs(dir)?;` deleted from `verify`; in `remove_outputs`,
/// `(name.starts_with(VERDICTS_PREFIX) && name.ends_with(VERDICTS_SUFFIX))` made `false`; and
/// `[GENERATED_LINES_FILE, GAPS_FILE, FIT_LINES_FILE]` made `[GAPS_FILE, FIT_LINES_FILE]`; each
/// caught.
#[test]
fn worklist_verify_leaves_no_output_of_a_previous_run_when_it_fails() {
    let (sandbox, out) = worklist_project("worklist-verify-removes");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    fill_unrelated(&out);
    let head = "design@core-mock@first-head";
    let (report, passing) = report_with(&out, head);
    let line = report_line(&passing, head);
    let outputs = || {
        let mut names: Vec<String> = std::fs::read_dir(&out)
            .expect("the work list")
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with("verdicts-") || n == "generated-lines.md")
            .collect();
        names.sort();
        names
    };
    for (failing, want) in [
        (passing.replace(&format!("{line}\n"), ""), 1),
        (passing.replace(&line, &format!("- {head} | nonsense")), 2),
    ] {
        std::fs::write(&report, &passing).expect("a passing report");
        let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
        assert_eq!(code, 0, "{stdout}{stderr}");
        assert_eq!(outputs(), ["generated-lines.md", "verdicts-whole-work.md"]);
        std::fs::write(out.join("verdicts-old-thread.md"), "# an earlier subject\n")
            .expect("an earlier subject's file");
        std::fs::write(&report, &failing).expect("a failing report");
        let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
        assert_eq!(code, want, "{stdout}{stderr}");
        assert!(outputs().is_empty(), "{:?}", outputs());
    }
}

/// Every file under `dir` with its bytes, by its path relative to `dir`.
fn tree_of(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    let mut files = std::collections::BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(at) = stack.pop() {
        for entry in std::fs::read_dir(&at).expect("a directory").flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path.strip_prefix(dir).expect("under dir").to_path_buf();
                files.insert(rel, std::fs::read(&path).expect("a file"));
            }
        }
    }
    files
}

/// `worklist --check <agent-dir>` judges one agent's report by `--verify`'s rules: on a passing
/// report it exits 0 with its `PASSED` line, and on a failing one 1, printing exactly the findings
/// `--verify` prints for that report and none of another agent's, though that one fails too. In
/// both cases it writes and removes no file of the work list, the outputs of an earlier `--verify`
/// included. An agent `agents.txt` does not name is refused, exit 2. Mutations checked, in
/// `path@query@src/worklist.rs`: in `judge_reports`, `if only.is_some_and(|o| o != agent) {` made
/// `if false {`, which prints the other agent's findings; and in `check`,
/// `if !list.agents.contains_key(&agent) {` made `if false {`, which passes an unknown agent; each
/// caught.
#[test]
fn worklist_check_judges_one_report_and_writes_nothing() {
    let (sandbox, out, _) = worklist_out("worklist-check", &["--bound", "120"]);
    let out_text = out.to_string_lossy().into_owned();
    fill_unrelated(&out);
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");

    let target = "design@core-mock@first-head";
    let (report, passing) = report_with(&out, target);
    let agent_dir = report.parent().expect("its agent directory").to_path_buf();
    let agent = agent_dir
        .file_name()
        .expect("a name")
        .to_string_lossy()
        .into_owned();
    let agent_text = agent_dir.to_string_lossy().into_owned();
    let before = tree_of(&out);
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--check", &agent_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!("PASSED: the report of {agent}")),
        "{stdout}"
    );
    assert_eq!(
        tree_of(&out),
        before,
        "a passing check changed the work list"
    );

    // The target's line removed, and a line of another agent's report removed too.
    let line = report_line(&passing, target);
    std::fs::write(&report, passing.replace(&format!("{line}\n"), "")).expect("a failing report");
    let other = std::fs::read_dir(&out)
        .expect("the work list")
        .flatten()
        .map(|e| e.path().join("report.md"))
        .find(|r| r.exists() && r != &report)
        .expect("a second agent");
    let other_text = std::fs::read_to_string(&other).expect("its report");
    let other_line = other_text
        .lines()
        .find(|l| l.starts_with("- ") && l.contains(" | verdict: "))
        .expect("a verdict line")
        .to_string();
    std::fs::write(&other, other_text.replace(&format!("{other_line}\n"), ""))
        .expect("a second failing report");
    let before = tree_of(&out);
    let (checked, stderr, code) = sandbox.run(&["worklist", "--check", &agent_text]);
    assert_eq!(code, 1, "{checked}{stderr}");
    assert_eq!(
        tree_of(&out),
        before,
        "a failing check changed the work list"
    );
    let (verified, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 1, "{verified}{stderr}");
    let findings = |text: &str, prefix: &str| -> Vec<String> {
        text.lines()
            .filter(|l| l.starts_with(prefix))
            .map(str::to_string)
            .collect()
    };
    let own = format!("{agent}/");
    assert!(!findings(&verified, &own).is_empty(), "{verified}");
    assert_eq!(findings(&checked, &own), findings(&verified, &own));
    assert_eq!(
        findings(&checked, "agent-").len(),
        findings(&checked, &own).len(),
        "the check printed another agent's findings: {checked}"
    );
    assert!(
        findings(&verified, "agent-").len() > findings(&verified, &own).len(),
        "the second agent's report does not fail: {verified}"
    );

    let unknown = out.join("agent-999").to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--check", &unknown]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(stderr.contains("names no agent `agent-999`"), "{stderr}");
    let _ = std::fs::remove_dir_all(&out);
}

/// The outputs are the snapshot's text, not the tree's: an entry edited after `--out` is written
/// as `--out` saw it, since `--verify` reads no model; and a passing run prints the count of each
/// subject's verdicts and of the generated lines. A snapshot that lacks an entry `agents.txt`
/// lists, as one an older `--out` wrote, stops the run before any report is judged, with exit 2,
/// no `PASSED` line, and none of the outputs of the passing run before it; one cut short, or one
/// whose entry lacks its second block, as an older `--out` wrote it, is refused the same way.
/// Mutations checked, in `verify` of `path@query@src/worklist.rs`: `if !missing.is_empty() {` made
/// `if false {`, which reaches the lookup of the missing entry; and the printed counts,
/// `for (subject, count) in &out.counts {` made to print 0 for each; each caught.
#[test]
fn worklist_verify_writes_from_the_snapshot_and_refuses_one_that_lacks_an_entry() {
    let (sandbox, out) = worklist_project("worklist-snapshot-gap");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    fill_unrelated(&out);
    let head = "design@core-mock@first-head";
    let (report, text) = report_with(&out, head);
    let quote = worklist_quotes(&out)[head].clone();
    std::fs::write(
        &report,
        text.replace(
            &report_line(&text, head),
            &format!(
                "- {head} | verdict: constrains | against: whole-work | action: respect | \
                 owner-question: | quote: {quote} | reason: it bears"
            ),
        ),
    )
    .expect("a report on the head");
    sandbox.write(
        "docs/design.md",
        &WORKLIST_DESIGN.replace("It serves", "EDITED AFTER THE WORK LIST. It serves"),
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains(&format!(
            "1 verdicts against whole-work in {}",
            out.join("verdicts-whole-work.md").display()
        )) && stdout.contains(&format!(
            "1 lines in {}",
            out.join("generated-lines.md").display()
        )),
        "{stdout}"
    );
    let verdicts = std::fs::read_to_string(out.join("verdicts-whole-work.md")).expect("verdicts");
    assert!(
        verdicts.contains("It serves `goal@fine-goal`"),
        "{verdicts}"
    );
    assert!(
        !verdicts.contains("EDITED AFTER THE WORK LIST"),
        "{verdicts}"
    );

    let shown = std::fs::read_to_string(out.join("shown.md")).expect("shown.md");
    let start = shown
        .find(&format!("=== {head}\n"))
        .expect("the head's entry");
    let end = shown[start + 4..]
        .find("\n=== ")
        .map(|i| start + 4 + i + 1)
        .unwrap_or(shown.len());
    let lacking = format!("{}{}", &shown[..start], &shown[end..]);
    // Cut short: its last fence never closed.
    let cut = shown[..shown.trim_end().rfind('\n').expect("a last line")].to_string();
    // One block per entry, as an older `--out` wrote it.
    let one_block: String = worklist_snapshot(&out)
        .iter()
        .map(|(r, own, inbound)| format!("=== {r}\n\n````text\n{own}\n\n{inbound}\n````\n\n"))
        .collect();
    for (snapshot, expected) in [
        (lacking, format!("lacks {head}")),
        (cut, "cut short".to_string()),
        (one_block, "does not open its two fenced blocks".to_string()),
    ] {
        // Each refusal follows a passing run, whose outputs it must not leave.
        std::fs::write(out.join("shown.md"), &shown).expect("the snapshot restored");
        let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
        assert_eq!(code, 0, "{stdout}{stderr}");
        std::fs::write(out.join("shown.md"), &snapshot).expect("a refused snapshot");
        let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
        assert_eq!(code, 2, "{expected}: {stdout}{stderr}");
        assert!(!stdout.contains("PASSED"), "{stdout}");
        assert!(
            stderr.contains(&expected) && stderr.contains("--out"),
            "{expected}: {stderr}"
        );
        assert!(
            !out.join("verdicts-whole-work.md").exists()
                && !out.join("generated-lines.md").exists(),
            "{expected}"
        );
    }
}

/// AC4 of the spec of the graded entries' assembly: `id` answers a free id with exit 0, lists
/// every entity defining an id with exit 1, reports an argument outside the grammar with exit 1,
/// and refuses an incomplete model with exit 2. Mutation checked: the call of
/// `complete_working_tree` in `id` deleted, which answers `free` over a tree missing a home.
#[test]
fn id_answers_free_defined_and_malformed_ids_and_refuses_an_incomplete_model() {
    let (sandbox, _) = worklist_project("id-lookup");
    // The design entry `first-head` of the root, and an issue of the same id in `widget`.
    sandbox.write(
        "widget/docs/open-issues/first-head.md",
        &worklist_issue("nothing"),
    );
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();

    let (stdout, stderr, code) = sandbox.run(&["id", "no-such-thing"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert_eq!(stdout, "no-such-thing  free\n");

    let (stdout, stderr, code) = sandbox.run(&["id", "no-such-thing", "first-head", "two"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(stdout.contains("no-such-thing  free"), "{stdout}");
    assert!(
        stdout.contains("design@core-mock@first-head")
            && stdout.contains("issue@widget@first-head"),
        "{stdout}"
    );
    assert!(
        stdout.contains("two            not an id: an id matches [a-z0-9]+(-[a-z0-9]+)+"),
        "{stdout}"
    );

    // Each answer alone gives its own exit: a defined id, and a malformed one.
    let (stdout, _, code) = sandbox.run(&["id", "first-head"]);
    assert_eq!(code, 1, "{stdout}");
    let (stdout, _, code) = sandbox.run(&["id", "two"]);
    assert_eq!(code, 1, "{stdout}");

    // A phase-2 finding: a home every Component carries, deleted and staged.
    std::fs::remove_file(sandbox.path("widget/docs/tripwires.md")).expect("a home removed");
    sandbox.stage();
    let (stdout, stderr, code) = sandbox.run(&["id", "no-such-thing"]);
    assert_eq!(code, 2, "{stdout}{stderr}");
    assert!(!stdout.contains("free"), "{stdout}");
    assert!(stderr.contains("the model is incomplete"), "{stderr}");
    assert!(stderr.contains("No id is answered over it"), "{stderr}");
    assert!(!stderr.contains("Nothing was written"), "{stderr}");
}

/// `id` prints a harness kind's entity in the form `show` takes, with no anchor for a skill and
/// with its owner for a section. Mutation checked: in `id`, the empty-anchor branch made never to
/// match, which prints `skill@@<name>`; caught.
#[test]
fn id_prints_a_skill_and_a_section_in_their_reference_forms() {
    let sandbox = harnessed("id-harness");
    let (stdout, stderr, code) = sandbox.run(&["id", "knowledge-architect-review", "review-axes"]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(
        stdout.contains("  skill@knowledge-architect-review\n"),
        "{stdout}"
    );
    assert!(
        stdout.contains("skill@knowledge-architect-review@review-axes"),
        "{stdout}"
    );
    assert!(!stdout.contains("@@"), "{stdout}");
}

/// Every `entries.md` of a work list, each under a line naming its agent, in agent order.
fn worklist_entries_files(out: &Path) -> String {
    let mut agents: Vec<PathBuf> = std::fs::read_dir(out)
        .expect("the work list")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("entries.md").exists())
        .collect();
    agents.sort_by_key(|p| {
        let name = p
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        name.trim_start_matches("agent-")
            .parse::<usize>()
            .expect("agent-<n>")
    });
    let mut all = String::new();
    for agent in agents {
        let name = agent
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .into_owned();
        all.push_str(&format!("==== {name}/entries.md\n"));
        all.push_str(&std::fs::read_to_string(agent.join("entries.md")).expect("entries"));
    }
    all
}

/// The claim, a regression guard: the files the searchers read hold no part of the snapshot, the
/// inbound lists `show` prints, so the snapshot adds nothing to an agent's load. Mutation checked:
/// in `entry_block` of `path@query@src/worklist.rs`, the line `referenced by nothing` appended
/// after each entry's closing fence; caught.
#[test]
fn worklist_entries_files_hold_no_inbound_list() {
    let (_sandbox, out, _) = worklist_out(
        "worklist-entries-no-inbound",
        &["--seed", "design@core-mock@first-head", "--bound", "60"],
    );
    let entries = worklist_entries_files(&out);
    assert!(entries.contains("==== agent-1/entries.md"), "{entries}");
    assert!(
        !entries.contains("referenced at:") && !entries.contains("referenced by nothing"),
        "{entries}"
    );
    let shown = std::fs::read_to_string(out.join("shown.md")).expect("shown.md");
    assert!(shown.contains("referenced at:"), "{shown}");
}

/// The claim: a file of the work given twice, by a relative and by an absolute path, is counted
/// and listed once. Mutation checked: in `work_load` of `path@query@src/worklist.rs`,
/// `if paths.contains(real) {` made `if false {`, which counts 600 words and lists two lines;
/// caught.
#[test]
fn worklist_counts_and_lists_a_file_of_the_work_once() {
    let (sandbox, out) = worklist_project("worklist-work-once");
    let work = out.with_extension("work.md");
    std::fs::write(&work, "word ".repeat(300)).expect("the work");
    let absolute = work.to_string_lossy().into_owned();
    // The sandbox and the work sit in one temporary directory, the sandbox's root the working
    // directory of the run.
    let relative = format!("../{}", work.file_name().expect("a name").to_string_lossy());
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&[
        "worklist", "--out", &out_text, "--bound", "400", "--work", &absolute, "--work", &relative,
    ]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains("the work holds 300 words, which leaves 100 of the 400-word bound"),
        "{stdout}"
    );
    let listed = std::fs::read_to_string(out.join("work.txt")).expect("work.txt");
    let real = work.canonicalize().expect("the work's path");
    assert_eq!(
        listed,
        format!("work {}\nsubject whole-work\n", real.display())
    );
    let _ = std::fs::remove_file(&work);
}

/// The claim: a group alone above the bound is named by a notice only where its own entries
/// exceed the bound, a cluster, a remainder and a goals file each by its own notice. At 120 the
/// root's first subject is above the bound by its context entry and title lines alone, and gets
/// no notice; at 20 the root's remainder and its goals file hold more words of their own than the
/// bound. Mutation checked: in `notice` of `path@query@src/worklist.rs`, `if own <= bound {` made
/// `if false {`, which names the first subject at 120; caught.
#[test]
fn worklist_names_a_group_above_the_bound_only_by_its_own_entries() {
    let (_sandbox, _out, stdout) = worklist_out("worklist-notices-120", &["--bound", "120"]);
    let first = worklist_lines(&stdout)
        .into_iter()
        .find(|(_, _, items)| items.iter().any(|(n, _)| n == "core-mock :: First subject"))
        .unwrap_or_else(|| panic!("no agent of the first subject: {stdout}"));
    assert!(
        first.1 > 120,
        "the first subject is within the bound: {stdout}"
    );
    assert!(!stdout.contains("First subject\" holds"), "{stdout}");
    let (_sandbox, _out, stdout) = worklist_out("worklist-notices-20", &["--bound", "20"]);
    assert!(
        stdout.contains("notice: the remainder of core-mock holds ")
            && stdout.contains("notice: the goals of \"core-mock :: docs/goals.md\" hold "),
        "{stdout}"
    );
}

/// The claim: a goal agent's entries file gives each goals file's prose above its first goal,
/// before the goals. Mutation checked: in `render` of `path@query@src/worklist.rs`,
/// `if !prose.is_empty() {` made `if false {`, which leaves the prose out; caught.
#[test]
fn worklist_gives_a_goal_agent_the_goals_file_s_prose() {
    let (_sandbox, out, _) = worklist_out("worklist-goals-prose", &[]);
    let (entries, _) = worklist_entries_of(&out, "goal@fine-goal");
    let section = worklist_section(&entries, "## Goals: core-mock :: docs/goals.md");
    let prose = section
        .find("The goals file above its first goal:")
        .unwrap_or_else(|| panic!("no prose: {entries}"));
    let heading = section
        .find("# Goals — core-mock")
        .expect("the file's heading");
    let goal = section.find("=== goal@fine-goal  ").expect("the goal");
    assert!(prose < heading && heading < goal, "{section}");
}

/// Stage everything the worklist project's copy holds and commit it as `subject`, with the
/// sandbox's identity.
fn worklist_commit(sandbox: &Sandbox, subject: &str) {
    sandbox.stage();
    sandbox.git(&["commit", "-q", "-m", subject]);
}

/// The marks of each line of `agents.txt`, by reference: the words after the reference.
fn worklist_marks(out: &Path) -> std::collections::BTreeMap<String, Vec<String>> {
    std::fs::read_to_string(out.join("agents.txt"))
        .expect("agents.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split(' ').skip(2);
            let reference = parts.next().expect("a reference").to_string();
            (reference, parts.map(String::from).collect())
        })
        .collect()
}

/// The references `agents.txt` marks `changed-since`.
fn worklist_changed(out: &Path) -> Vec<String> {
    worklist_marks(out)
        .into_iter()
        .filter(|(_, marks)| marks.iter().any(|m| m == "changed-since"))
        .map(|(reference, _)| reference)
        .collect()
}

/// The claim: `--since` marks exactly the entries whose file differs between the commit it names
/// and the work tree: a file a later commit changed, one changed and not staged, an untracked one
/// and the new path of a rename; no other entry. The mark is in `agents.txt`, after the seed mark,
/// and on the entry's `===` line in its entries file, and the run names the commit by its subject.
/// Over `HEAD`, only the changes of the work tree are marked. A branch named `docs`, like a
/// directory at the project's root, gives the marks and the subject of the commit it names.
/// Mutation checked for the last: in `write` of `path@query@src/worklist.rs`,
/// `commit_message(manifest.root(), &sha)` made `commit_message(manifest.root(), commit)`, which
/// git refuses as ambiguous; caught. Mutations checked earlier: in
/// `paths_changed_since` of `path@git@src/git.rs`, the `ls-files --others` extension removed,
/// which misses the untracked file; `.args([sha.as_str(), "--"])` made `.args(["HEAD", "--"])`,
/// which misses the later commit's change; in `write` of `path@query@src/worklist.rs`,
/// `paths.contains(&e.record.site.file)` made `!paths.contains(&e.record.site.file)`; and in
/// `render`, the push of the changed-since mark in `agents.txt` removed; each caught.
#[test]
fn worklist_since_marks_exactly_the_entries_of_the_files_changed_since_the_commit() {
    let (sandbox, out) = worklist_project("worklist-since");
    worklist_commit(&sandbox, "The base of the audit");
    // A later commit changes the tripwires home.
    sandbox.write(
        "docs/tripwires.md",
        &format!("{WORKLIST_TRIPWIRES}\nA later line.\n"),
    );
    worklist_commit(&sandbox, "A later change");
    // In the work tree: a design file changed and not staged, a rename staged, and an untracked
    // issue, with the issue index regenerated.
    sandbox.write(
        "widget/docs/design/parts.md",
        "# Parts\n\n## The gears\n\n### Gears turn `##gears-turn`\n\nThey do, now.\n\n\
         ## The springs\n\n### Springs coil `##springs-coil`\n\nThey do.\n",
    );
    sandbox.git(&[
        "mv",
        "widget/docs/open-issues/on-the-root.md",
        "widget/docs/open-issues/on-the-root-moved.md",
    ]);
    sandbox.write(
        "docs/open-issues/an-untracked-one.md",
        &worklist_issue("`goal@top-goal`"),
    );
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let base = String::from_utf8(
        sandbox
            .git_command(&["rev-parse", "HEAD~1"])
            .output()
            .expect("git runs")
            .stdout,
    )
    .expect("a sha")
    .trim()
    .to_string();
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&[
        "worklist",
        "--out",
        &out_text,
        "--since",
        &base,
        "--seed",
        "design@widget@gears-turn",
    ]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert_eq!(
        worklist_changed(&out),
        [
            "design@widget@gears-turn",
            "design@widget@springs-coil",
            "issue@core-mock@an-untracked-one",
            "issue@widget@on-the-root-moved",
            "tripwire@core-mock@first-premise",
        ]
    );
    assert_eq!(
        worklist_marks(&out)["design@widget@gears-turn"],
        ["seed", "changed-since"]
    );
    assert!(
        stdout.contains("5 entries changed since the commit \"The base of the audit\""),
        "{stdout}"
    );
    let (entries, _) = worklist_entries_of(&out, "design@widget@gears-turn");
    assert!(
        entries
            .lines()
            .any(|l| l.starts_with("=== design@widget@gears-turn  ")
                && l.ends_with("  changed-since")),
        "{entries}"
    );
    assert!(
        entries
            .lines()
            .filter(|l| l.starts_with("=== design@core-mock@"))
            .all(|l| !l.ends_with("changed-since")),
        "{entries}"
    );
    let head = std::env::temp_dir().join(format!(
        "knowledge-worklist-since-head-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&head);
    let head_text = head.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &head_text, "--since", "HEAD"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert_eq!(
        worklist_changed(&head),
        [
            "design@widget@gears-turn",
            "design@widget@springs-coil",
            "issue@core-mock@an-untracked-one",
            "issue@widget@on-the-root-moved",
        ]
    );
    let _ = std::fs::remove_dir_all(&head);
    // A branch named like a directory at the project's root: the revision is resolved once, and
    // every later question names the commit by its full object name, which git never reads as a
    // path.
    sandbox.git(&["branch", "docs", &base]);
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--out", &head_text, "--since", "docs"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert_eq!(worklist_changed(&head), worklist_changed(&out));
    assert!(
        stdout.contains("5 entries changed since the commit \"The base of the audit\""),
        "{stdout}"
    );
    let _ = std::fs::remove_dir_all(&head);
    let _ = std::fs::remove_dir_all(&out);
}

/// The claim: a `--since` git cannot resolve to one commit, a name of none or a range, stops the
/// run with exit 2, naming the git command that failed and git's own reason, and writes nothing.
/// Mutation checked, in `commit_of` of `path@git@src/git.rs`: `let object = ask(revision)?;` made
/// `let object = ask(revision).unwrap_or_else(|_| "HEAD".to_string());`, which resolves every
/// revision; caught.
#[test]
fn worklist_since_a_revision_git_cannot_resolve_stops_with_git_s_reason() {
    let (sandbox, out) = worklist_project("worklist-since-refused");
    worklist_commit(&sandbox, "The base");
    let out_text = out.to_string_lossy().into_owned();
    for revision in ["nowhere-at-all", "HEAD..HEAD"] {
        let (stdout, stderr, code) =
            sandbox.run(&["worklist", "--out", &out_text, "--since", revision]);
        assert_eq!(code, 2, "{stdout}{stderr}");
        assert!(
            stderr.contains(&format!(
                "--since {revision}: `git rev-parse --verify --end-of-options {revision}` failed"
            )),
            "{stderr}"
        );
        assert!(stderr.contains("Needed a single revision"), "{stderr}");
        assert!(!out.exists(), "nothing written");
    }
}

/// The text of a spec of the worklist project, whose section "What is already decided" cites a
/// head, a rule of a declared register, a goal, an issue and a `rejected` entry, and whose other
/// sections cite a head too.
const WORKLIST_SPEC: &str = "# An audited spec\n\n## Status and audience\n\nIt names \
    `design@core-mock@long-head`.\n\n## What is already decided\n\n\
    - `design@core-mock@first-head`\n- `goal@top-goal`\n\
    - `issue@core-mock@on-the-head`\n- `rejected@core-mock@other-way`\n\n\
    ### Of the notes\n\n- `rule@alpha-topic@alpha-rule`\n\n\
    ## Criteria\n\nIt names `design@widget@springs-coil`.\n";

/// The worklist project with the specs `an-audit` and `no-section`, the second holding no section
/// "What is already decided", its plans index regenerated and staged.
fn worklist_audit_project(tag: &str) -> (Sandbox, PathBuf) {
    let (sandbox, out) = worklist_project(tag);
    sandbox.write("docs/plans/specs/an-audit.md", WORKLIST_SPEC);
    sandbox.write(
        "docs/plans/specs/no-section.md",
        "# A spec with no decided section\n\n## Status and audience\n\nIt names \
         `design@core-mock@first-head`.\n",
    );
    let (stdout, stderr, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    sandbox.stage();
    (sandbox, out)
}

/// The claim: `--seeds-from` adds as seeds exactly the design entries and the declared-register
/// entries the section "What is already decided" of the plan document cites, its subsection
/// included, beside a `--seed`: no goal, issue or `rejected` entry it cites there, and nothing it
/// cites in another section; `work.txt` names the plan document by its citation and its path; and
/// a path that is no plan document is refused with exit 2, nothing written. The count of seeds
/// from the section counts one a `--seed` also names, `design@core-mock@first-head`, as a
/// regression guard: it counted only the seeds `--seed` had not named before `decided_seeds` took
/// the selection out of `write`, which no mutation of the present code brings back. A plan
/// document with no such section gives no seed, and the run says so. Mutations checked, in
/// `write` of `path@query@src/worklist.rs`: `seeds.extend(&taken);` made `let _ = &taken;`, which
/// takes no seed; the refusal `let Some((citation, rel)) = found else {` given the citation
/// `spec@plans@x` for any path; and `if *has_section {` made `if true {`; each caught.
#[test]
fn worklist_seeds_from_takes_the_decided_section_s_design_and_declared_entries() {
    let (sandbox, out) = worklist_audit_project("worklist-seeds-from");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&[
        "worklist",
        "--out",
        &out_text,
        "--seeds-from",
        "docs/plans/specs/an-audit.md",
        "--seed",
        "design@widget@gears-turn",
        "--seed",
        "design@core-mock@first-head",
    ]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let seeds: Vec<String> = worklist_marks(&out)
        .into_iter()
        .filter(|(_, marks)| marks.first().is_some_and(|m| m == "seed"))
        .map(|(reference, _)| reference)
        .collect();
    assert_eq!(
        seeds,
        [
            "design@core-mock@first-head",
            "design@widget@gears-turn",
            "rule@alpha-topic@alpha-rule",
        ]
    );
    assert!(
        stdout
            .contains("spec@plans@an-audit: 2 seeds from its section \"What is already decided\""),
        "{stdout}"
    );
    let work = std::fs::read_to_string(out.join("work.txt")).expect("work.txt");
    let spec_line = work
        .lines()
        .find(|l| l.starts_with("spec "))
        .unwrap_or_else(|| panic!("no spec line: {work}"));
    assert!(
        spec_line.starts_with("spec spec@plans@an-audit /")
            && spec_line.ends_with("docs/plans/specs/an-audit.md"),
        "{spec_line}"
    );
    let refused = std::env::temp_dir().join(format!(
        "knowledge-worklist-seeds-refused-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&refused);
    let refused_text = refused.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&[
        "worklist",
        "--out",
        &refused_text,
        "--seeds-from",
        "docs/plans/specs/no-section.md",
    ]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains(
            "spec@plans@no-section: no section \"What is already decided\", so no seed from it"
        ),
        "{stdout}"
    );
    assert!(
        !worklist_marks(&refused)
            .values()
            .any(|marks| marks.first().is_some_and(|m| m == "seed")),
        "no seed"
    );
    let _ = std::fs::remove_dir_all(&refused);
    for path in ["docs/design.md", "docs/plans/specs/missing.md"] {
        let (stdout, stderr, code) =
            sandbox.run(&["worklist", "--out", &refused_text, "--seeds-from", path]);
        assert_eq!(code, 2, "{stdout}{stderr}");
        assert!(
            stderr.contains("no plan document of the project"),
            "{stderr}"
        );
        assert!(!refused.exists(), "nothing written");
    }
    let _ = std::fs::remove_dir_all(&out);
}

/// The claim: `--out` refuses, with exit 2 and nothing written, a file of the work and a plan
/// document whose path holds a line break, which `work.txt`, one item per line, cannot list. The
/// project is a copy whose own directory's name holds one. Mutations checked, in
/// `path@query@src/worklist.rs`: in `write`, `refuse_a_line_break("the file of the work", &real)?;`
/// removed, and `refuse_a_line_break("--seeds-from", &real)?;` removed; in `refuse_a_line_break`,
/// `.contains(['\n', '\r'])` made `.contains(['\r'])`; each caught.
#[test]
fn worklist_refuses_a_path_of_the_work_holding_a_line_break() {
    let (sandbox, out) = worklist_audit_project("worklist-line\nbreak");
    let out_text = out.to_string_lossy().into_owned();
    for option in ["--work", "--seeds-from"] {
        let (stdout, stderr, code) = sandbox.run(&[
            "worklist",
            "--out",
            &out_text,
            option,
            "docs/plans/specs/an-audit.md",
        ]);
        assert_eq!(code, 2, "{option}: {stdout}{stderr}");
        assert!(stderr.contains("its path holds a line break"), "{stderr}");
        assert!(!out.exists(), "nothing written");
    }
}

/// The claim: at a design audit, `--verify` writes `gaps.md`, every verdict but `constrains` and
/// `unrelated` with its entry's text, and `fit-lines.md`, the line of each entry given a verdict
/// other than `unrelated` under the spec cited whole and the `--since` commit named by its
/// subject, in place of `generated-lines.md`, and prints the count of each; and a later run that
/// fails leaves neither. Mutations checked, in `path@query@src/worklist.rs`: in `outputs`,
/// `match &work.spec {` made to take the arm of a run outside an audit; in `parse_work`,
/// `work.spec = Some(citation.to_string());` made `work.spec = None;`; in `remove_outputs`,
/// `[GENERATED_LINES_FILE, GAPS_FILE, FIT_LINES_FILE]` made `[GENERATED_LINES_FILE]`; and in
/// `verify`, the printed count of gaps made 0; each caught.
#[test]
fn worklist_verify_at_an_audit_writes_the_gaps_and_the_fit_lines() {
    let (sandbox, out) = worklist_audit_project("worklist-audit");
    worklist_commit(&sandbox, "[plans] The spec of the audit");
    let out_text = out.to_string_lossy().into_owned();
    let (stdout, stderr, code) = sandbox.run(&[
        "worklist",
        "--out",
        &out_text,
        "--work",
        "docs/plans/specs/an-audit.md",
        "--subject",
        "spec-audit",
        "--seeds-from",
        "docs/plans/specs/an-audit.md",
        "--since",
        "HEAD",
    ]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let marks = worklist_marks(&out);
    fill_reports(&out, |reference, quote| {
        let (verdict, against, action) = match reference {
            "design@core-mock@long-head" => ("conflicts", "spec-audit", "check-reversal"),
            "issue@core-mock@on-the-goal" => ("touches", "spec-audit", "none"),
            _ if !marks[reference].is_empty() => ("constrains", "spec-audit", "respect"),
            _ => ("unrelated", "-", "none"),
        };
        format!(
            "verdict: {verdict} | against: {against} | action: {action} | owner-question: | \
             quote: {quote} | reason: r"
        )
    });
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        !out.join("generated-lines.md").exists(),
        "no generated lines at an audit"
    );
    let gaps = std::fs::read_to_string(out.join("gaps.md")).expect("gaps.md");
    let headed: Vec<&str> = gaps
        .lines()
        .filter_map(|l| l.strip_prefix("=== "))
        .collect();
    assert_eq!(
        headed,
        ["design@core-mock@long-head", "issue@core-mock@on-the-goal"],
        "{gaps}"
    );
    let fit = std::fs::read_to_string(out.join("fit-lines.md")).expect("fit-lines.md");
    assert!(
        fit.starts_with(
            "The fit of `spec@plans@an-audit` to the record, since the commit \
             \"[plans] The spec of the audit\":\n\n"
        ),
        "{fit}"
    );
    assert!(
        fit.contains("- `design@core-mock@first-head`: constrains, respect.\n"),
        "{fit}"
    );
    assert!(
        fit.contains("- `design@core-mock@long-head`: conflicts, check-reversal.\n"),
        "{fit}"
    );
    assert!(stdout.contains("2 gaps in "), "{stdout}");
    assert!(
        stdout.contains(" lines in ") && stdout.contains("fit-lines.md"),
        "{stdout}"
    );
    let (report, passing) = report_with(&out, "design@core-mock@long-head");
    let line = report_line(&passing, "design@core-mock@long-head");
    std::fs::write(&report, passing.replace(&format!("{line}\n"), "")).expect("a failing report");
    let (stdout, stderr, code) = sandbox.run(&["worklist", "--verify", &out_text]);
    assert_eq!(code, 1, "{stdout}{stderr}");
    assert!(!out.join("gaps.md").exists() && !out.join("fit-lines.md").exists());
    let _ = std::fs::remove_dir_all(&out);
}

// ---- `transcript` ------------------------------------------------------------------------------
//
// The fixtures under `path@testing@tests/transcripts/` are Claude Code transcript directories, one per
// case, whose `cwd` fields hold `{{root}}`. Each test writes its case into a directory of its own,
// with `{{root}}` replaced by the mock `core`'s root, and runs the binary there with
// `CLAUDE_CONFIG_DIR` pointed at it, so no test reads the machine's own transcripts. The mock gains
// no file: `transcript` reads nothing of the project but its root.

/// The case `case` written for the test `tag`, its `{{root}}` replaced; the directory to hand as
/// `CLAUDE_CONFIG_DIR`.
fn transcript_case(case: &str, tag: &str) -> PathBuf {
    let root = project("core").canonicalize().expect("the mock exists");
    let dir = std::env::temp_dir().join(format!("klarch-transcript-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let from = transcript_dir().join("tests/transcripts").join(case);
    copy_rendered(&from, &dir, &root.display().to_string());
    dir
}

fn copy_rendered(from: &Path, to: &Path, root: &str) {
    std::fs::create_dir_all(to).expect("a fixture directory");
    for entry in std::fs::read_dir(from).expect("a fixture directory") {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_rendered(&entry.path(), &target, root);
        } else {
            // Bytes, not text: a fixture may hold a line that is no UTF-8 on purpose.
            let bytes = std::fs::read(entry.path()).expect("a fixture file");
            let mut out = Vec::with_capacity(bytes.len());
            let mut rest = bytes.as_slice();
            while let Some(at) = rest.windows(8).position(|w| w == b"{{root}}") {
                out.extend_from_slice(&rest[..at]);
                out.extend_from_slice(root.as_bytes());
                rest = &rest[at + 8..];
            }
            out.extend_from_slice(rest);
            std::fs::write(&target, out).expect("a fixture copy");
        }
    }
}

/// One `transcript` run over a case, in the mock `core`.
fn transcript(case: &str, tag: &str, args: &[&str]) -> (String, String, i32) {
    let config = transcript_case(case, tag);
    let mut all = vec!["transcript"];
    all.extend_from_slice(args);
    run_with_env(&project("core"), &all, &[("CLAUDE_CONFIG_DIR", &config)])
}

/// A session opens where the `sessionId` changes and at a root that is no compaction; a compaction
/// and a resumed process stay inside their session.
#[test]
fn transcript_sessions_are_cut_at_a_new_id_and_a_root_that_is_no_compaction() {
    let (stdout, stderr, code) = transcript("split", "split-list", &["list"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let rows: Vec<&str> = stdout.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(rows.len(), 3, "{stdout}");
    assert!(
        rows[0].starts_with("aaaaaaaa ") && rows[0].contains("compactions 1"),
        "{stdout}"
    );
    assert!(rows[1].starts_with("bbbbbbbb "), "{stdout}");
    assert!(
        rows[2].starts_with("bbbbbbbb#2 ") && rows[2].contains("feature"),
        "{stdout}"
    );

    let (stdout, _, code) = transcript("split", "split-show", &["show", "aaaaaaaa"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("first question"), "{stdout}");
    assert!(stdout.contains("injected compact-boundary"), "{stdout}");
    assert!(stdout.contains("injected compact-summary"), "{stdout}");
    assert!(stdout.contains("after the resume"), "{stdout}");
    assert!(!stdout.contains("another id"), "{stdout}");

    let (stdout, _, code) = transcript("split", "split-branch", &["list", "--branch", "feature"]);
    assert_eq!(code, 0, "{stdout}");
    let rows: Vec<&str> = stdout.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(rows.len(), 1, "{stdout}");
    assert!(rows[0].starts_with("bbbbbbbb#2 "), "{stdout}");
}

/// A cleared session is listed apart, found by its first owner message, and its `/clear` is the
/// owner's command, per `design@transcript@transcript-session-split`.
#[test]
fn transcript_a_cleared_session_is_listed_apart_and_found_by_its_first_owner_message() {
    let (stdout, _, code) = transcript("cleared", "cleared-list", &["list"]);
    assert_eq!(code, 0, "{stdout}");
    assert_eq!(
        stdout.lines().filter(|l| !l.starts_with('#')).count(),
        2,
        "{stdout}"
    );

    let (stdout, _, code) = transcript(
        "cleared",
        "cleared-containing",
        &["list", "--containing", "where is my transcript, after"],
    );
    assert_eq!(code, 0, "{stdout}");
    let rows: Vec<&str> = stdout.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(rows.len(), 1, "{stdout}");
    assert!(rows[0].starts_with("dddddddd "), "{stdout}");

    let (stdout, _, code) = transcript("cleared", "cleared-show", &["show", "dddddddd"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("owner-command"), "{stdout}");
    assert!(stdout.contains("\n/clear\n"), "{stdout}");
    assert!(!stdout.contains("<command-name>"), "{stdout}");
    assert!(stdout.contains("injected meta"), "{stdout}");
}

/// The rows of the mapping table "Line to label", AC1 (every unknown value is named in the header,
/// and its line kept) and AC3 (an owner's message that opens with a harness tag stays the owner's).
#[test]
fn transcript_each_line_takes_the_label_of_its_fields_and_the_unknown_is_reported() {
    let (stdout, stderr, code) = transcript(
        "labels",
        "labels-show",
        &["show", "eeeeeeee", "--tools", "--subagents"],
    );
    assert_eq!(code, 0, "{stdout}{stderr}");
    let has = |needle: &str| assert!(stdout.contains(needle), "missing {needle:?} in:\n{stdout}");
    // AC3.
    has("[L1 owner 2026-01-03 10:00]\n<system-reminder>pasted by the owner</system-reminder>");
    // AC1: the header names each unknown value, with its count.
    has("# unrecognised type \"mystery\": 1");
    has("# unrecognised attachment.type \"new_thing\": 1");
    has("# unrecognised system subtype \"new_sub\": 1");
    has("# unrecognised assistant content item \"new_item\": 1");
    has("# unrecognised origin.kind \"robot\": 1");
    has("# unrecognised attachment.origin.kind \"robot\": 1");
    has("injected unknown 2026-01-03 10:00]\nfrom a robot");
    has("injected unknown 2026-01-03 10:00]\nfrom a queued robot");
    // The other rows.
    has("agent 2026-01-03 10:00]\nthe agent speaks");
    has("tool Bash 2026-01-03 10:00]\nBash {\"command\":\"ls\"}");
    has("tool Bash 2026-01-03 10:00]\nfile listing");
    has("owner-answer");
    has("answer: B");
    has("injected skill");
    has("lines omitted; --full prints them");
    has("injected meta 2026-01-03 10:00]\na meta notice");
    has("injected peer 2026-01-03 10:00]\na peer report");
    has("injected task-notification");
    // An interrupt's text placed only whole: a line that holds more stays unplaced.
    has("unlabelled 2026-01-03 10:00]\n[Request interrupted by user] and words after it");
    // A shell output's tag places a line only as the child of a shell command's line.
    has("unlabelled 2026-01-03 10:00]\n<bash-stdout>pasted with no command before it");
    has("owner-queued 2026-01-03 10:00]\ntyped while busy");
    has("injected peer 2026-01-03 10:00]\na queued report");
    has("# unlabelled lines: 2");
    has("# lines skipped for a repeated uuid: 1");
    has("# not JSON: -proj/eeeeeeee-0000-0000-0000-000000000005.jsonl:");
    has("is a gap of this command");
    // The subagents: a field-less line of a subagent file is its dispatcher's.
    has("# subagent eeeeeeee/f00dcafe: general-purpose");
    has("dispatch 2026-01-03 10:00]\nthe brief of the reviewer");
    has("the reviewer reports");
    has("dispatch 2026-01-03 10:00]\n[Workflow harness - computed task] the task");
    assert!(
        !stdout.contains("hidden"),
        "a thinking item is dropped:\n{stdout}"
    );
    assert_eq!(stdout.matches("the last owner word").count(), 1, "{stdout}");

    let (stdout, _, _) = transcript("labels", "labels-no-tools", &["show", "eeeeeeee"]);
    assert!(!stdout.contains("tool Bash"), "{stdout}");

    // AC3, its second half: the owner's words are found.
    let (stdout, _, code) = transcript(
        "labels",
        "labels-containing",
        &["list", "--containing", "and my words"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("eeeeeeee "), "{stdout}");
}

/// A project's sessions are its by the `cwd` of their first line, a worktree under the root
/// included; a log with no field is placed by no text, so its lines are unlabelled and reported.
#[test]
fn transcript_a_project_s_sessions_are_found_by_their_cwd() {
    let (stdout, _, code) = transcript("places", "places-list", &["list"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.contains("ffffffff "),
        "a worktree under the root:\n{stdout}"
    );
    assert!(stdout.contains("88888888 "), "{stdout}");
    assert!(!stdout.contains("99999999 "), "another project:\n{stdout}");

    let (stdout, _, _) = transcript("places", "places-all", &["list", "--all-projects"]);
    assert!(stdout.contains("99999999 "), "{stdout}");

    let (stdout, _, code) = transcript("places", "places-show", &["show", "88888888"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.contains("unlabelled 2026-01-04 12:00]\nan owner's words"),
        "{stdout}"
    );
    assert!(
        stdout.contains("unlabelled 2026-01-04 12:00]\n<task-notification>"),
        "{stdout}"
    );
    assert!(stdout.contains("# unlabelled lines: 2"), "{stdout}");
    assert!(!stdout.contains("no line carries a field"), "{stdout}");
}

/// The exit codes: a negative answer is 1, an input that cannot be used is 2.
#[test]
fn transcript_exit_codes_follow_the_ladder() {
    let code = |case: &str, tag: &str, args: &[&str]| transcript(case, tag, args).2;
    assert_eq!(
        code(
            "cleared",
            "exit-from",
            &["show", "dddddddd", "--from", "nowhere"]
        ),
        1
    );
    assert_eq!(code("cleared", "exit-handle", &["show", "zzzzzzzz"]), 1);
    assert_eq!(code("split", "exit-ambiguous", &["show", "bbbb"]), 1);
    assert_eq!(code("split", "exit-handle-2", &["show", "bbbbbbbb#2"]), 0);
    assert_eq!(code("labels", "exit-regex", &["search", "("]), 2);
    assert_eq!(
        code(
            "labels",
            "exit-nomatch",
            &["search", "nothing matches this"]
        ),
        1
    );
    assert_eq!(
        code("labels", "exit-since", &["list", "--since", "yesterday"]),
        2
    );
    assert_eq!(
        code(
            "split",
            "exit-since-none",
            &["list", "--since", "2027-01-01"]
        ),
        1
    );
    let (_, stderr, code) = run_with_env(
        &project("core"),
        &["transcript", "list"],
        &[("CLAUDE_CONFIG_DIR", Path::new("/nonexistent/klarch-config"))],
    );
    assert_eq!(code, 2, "{stderr}");
}

/// `search` keeps lines by role and by tool, and a tool's results with its calls.
#[test]
fn transcript_search_selects_by_role_and_by_tool() {
    let (stdout, _, code) =
        transcript("labels", "search-role", &["search", "--role", "owner", "."]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("eeeeeeee L1 owner"), "{stdout}");
    assert!(!stdout.contains(" agent "), "{stdout}");

    let (stdout, _, code) = transcript("labels", "search-tool", &["search", "--tool", "Bash", "."]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("tool Bash"), "{stdout}");
    assert!(
        stdout.contains("file listing"),
        "the call's result:\n{stdout}"
    );
    assert!(!stdout.contains("Agent"), "{stdout}");

    let (stdout, _, _) = transcript("labels", "search-plain", &["search", "file listing"]);
    assert!(
        !stdout.contains("file listing"),
        "tool lines are searched under --tools:\n{stdout}"
    );
}

/// The cuts: a line no row placed, the agent's text and `--full` print whole; an injected or a tool
/// text is cut beyond 40 lines and not at 40.
#[test]
fn transcript_an_injected_or_tool_text_is_cut_beyond_forty_lines_and_nothing_else_is() {
    let (stdout, _, code) = transcript("extras", "extras-cuts", &["show", "12345678"]);
    assert_eq!(code, 0, "{stdout}");
    let has = |needle: &str| assert!(stdout.contains(needle), "missing {needle:?} in:\n{stdout}");
    has("unplaced 40\n");
    has("agentline 44\n");
    has("metalong 4\n(36 lines omitted; --full prints them)");
    has("metaforty 39\n");
    assert!(!stdout.contains("metalong 40"), "{stdout}");
    let (stdout, _, _) = transcript("extras", "extras-toolcut", &["show", "12345678", "--tools"]);
    assert!(stdout.contains("toolline 4\n(36 lines omitted"), "{stdout}");

    let (stdout, _, _) = transcript("labels", "labels-full", &["show", "eeeeeeee", "--full"]);
    assert!(stdout.contains("body line 59"), "{stdout}");
    let (stdout, _, _) = transcript("labels", "labels-cut", &["show", "eeeeeeee"]);
    assert!(!stdout.contains("body line 59"), "{stdout}");
}

/// The rows the first fixtures left untested.
#[test]
fn transcript_the_remaining_rows_of_the_mapping_hold() {
    let (stdout, _, code) = transcript("extras", "extras-rows", &["show", "12345678", "--tools"]);
    assert_eq!(code, 0, "{stdout}");
    let has = |needle: &str| assert!(stdout.contains(needle), "missing {needle:?} in:\n{stdout}");
    // A line holding a tool result keeps its own text.
    has("owner 2026-02-01 10:00]\nwords beside a result");
    // Questions with no answers are a tool result, not the owner's answer.
    assert_eq!(stdout.matches("owner-answer").count(), 2, "{stdout}");
    has("options: A (the first) | B (the second)");
    has("answer: B");
    has("annotations:");
    has("my note");
    has("injected meta 2026-02-01 10:00]\na queued meta text");
    has("injected unknown 2026-02-01 10:00]\nan origin with no kind");
    has("# unrecognised attachment.origin.kind \"(none)\": 1");
    // A `local_command` line with no `commandRun` consumes nothing, and is reported.
    has("unlabelled 2026-02-01 10:00]\n<command-name>/foo</command-name>");
    has("# unrecognised system local_command shape \"no commandRun\": 1");
    // A `local_command` whose parent is the agent's line drops no line.
    has("agent 2026-02-01 10:00]\nthe agent before a model switch");
    has("owner-command 2026-02-01 10:00]\n/model opus");
    has("# unrecognised user content item \"image2\": 1");
    has("injected meta 2026-02-01 10:00]\nnotes: Base directory for this skill: inside");
    has("# unrecognised attachment.type \"dup_thing\": 2, first at -proj/12345678-0000-0000-0000-000000000009.jsonl:17");
    has("# files read: 1; harness versions: 2.1.310");
    assert_eq!(
        stdout.matches("# not JSON").count(),
        1,
        "a blank line is no line, and `[1, 2]` is no JSON object:\n{stdout}"
    );

    // A line no field places in a file whose one field is `isCompactSummary` is unlabelled; a line
    // with no parent key opens a session.
    let (stdout, _, _) = transcript("extras", "extras-nofallback", &["show", "23456789"]);
    assert!(
        stdout.contains("unlabelled 2026-02-02 10:00]\nplain words"),
        "{stdout}"
    );
    assert!(!stdout.contains("no parent key"), "{stdout}");
    let (stdout, _, _) = transcript("extras", "extras-list", &["list"]);
    assert!(stdout.contains("23456789#2 "), "{stdout}");

    // A message to a subagent from its dispatcher, through the harness, is injected.
    let (stdout, _, _) = transcript(
        "labels",
        "labels-followup",
        &["show", "eeeeeeee", "--subagents"],
    );
    assert!(
        stdout.contains("injected peer 2026-01-03 10:00]\na follow-up from the dispatcher"),
        "{stdout}"
    );

    // A line no row places is a gap, alone.
    let (stdout, _, _) = transcript("split", "split-gap", &["show", "bbbbbbbb"]);
    assert!(stdout.contains("# unlabelled lines: 1"), "{stdout}");
    assert!(stdout.contains("is a gap of this command"), "{stdout}");

    // A line of bytes that are no UTF-8 is a line that is no JSON, in either mode.
    let (stdout, _, code) = transcript("places", "places-utf8", &["list"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("# not JSON: -wt/ffffffff"), "{stdout}");
}

/// One `sessionId` in two files gives two handles; `list` is oldest first; `--since` keeps a session
/// that ends on the date; subagents go to the session whose call dispatched them, or to the last.
#[test]
fn transcript_handles_order_and_placement_hold_across_files() {
    let (stdout, _, code) = transcript("twofiles", "two-list", &["list"]);
    assert_eq!(code, 0, "{stdout}");
    let rows: Vec<&str> = stdout.lines().filter(|l| !l.starts_with('#')).collect();
    let handles: Vec<&str> = rows.iter().map(|r| r.split(' ').next().unwrap()).collect();
    assert_eq!(
        handles,
        [
            "b2000000#2",
            "a1000000",
            "a1000000#2",
            "b2000000",
            "c3000000"
        ],
        "{stdout}"
    );
    assert!(
        rows[0].ends_with(
            "b in its own file, with a first message long enough to be cut at eighty characte…"
        ),
        "{stdout}"
    );
    // Agent x by its call, and agent z by the call agent x made.
    assert!(rows[1].contains("subagents 2"), "{stdout}");
    assert!(rows[2].contains("subagents 1"), "{stdout}");
    assert!(rows[3].contains("zeta, alpha"), "{stdout}");
    assert!(rows[3].contains("owner 1"), "{stdout}");

    let (stdout, _, code) = transcript("twofiles", "two-show2", &["show", "b2000000#2"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("b in its own file"), "{stdout}");

    let (stdout, _, _) = transcript("twofiles", "two-sub", &["show", "a1000000", "--subagents"]);
    assert!(stdout.contains("brief of agent x"), "{stdout}");
    assert!(!stdout.contains("brief of agent y"), "{stdout}");
    let (stdout, _, _) = transcript(
        "twofiles",
        "two-sub2",
        &["show", "a1000000#2", "--subagents"],
    );
    assert!(stdout.contains("brief of agent y"), "{stdout}");

    let (stdout, _, code) = transcript("twofiles", "two-since", &["list", "--since", "2026-03-04"]);
    assert_eq!(code, 0, "{stdout}");
    let rows: Vec<&str> = stdout.lines().filter(|l| !l.starts_with('#')).collect();
    assert_eq!(rows.len(), 2, "{stdout}");
    assert!(rows[0].starts_with("b2000000 "), "{stdout}");

    // `--containing` reads what the owner typed, not the agent's lines.
    let code = transcript(
        "twofiles",
        "two-cont",
        &["list", "--containing", "an agent line only"],
    )
    .2;
    assert_eq!(code, 1);
    // A session with no kept line prints nothing, a negative answer.
    assert_eq!(
        transcript("twofiles", "two-empty", &["show", "c3000000"]).2,
        1
    );
}

/// `--from` starts at the owner's line it names, that line included; the owner count holds the
/// lines typed mid-turn; `search` reads subagents under `--subagents`, and `--tool` keeps tool lines
/// alone.
#[test]
fn transcript_from_counts_and_search_scopes_hold() {
    let (stdout, _, code) = transcript(
        "labels",
        "from-incl",
        &["show", "eeeeeeee", "--from", "and my words"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("[L1 owner"), "{stdout}");
    let (stdout, _, _) = transcript(
        "labels",
        "from-late",
        &["show", "eeeeeeee", "--from", "the last owner word"],
    );
    assert!(!stdout.contains("[L1 owner"), "{stdout}");
    assert!(stdout.contains("the last owner word"), "{stdout}");

    let (stdout, _, _) = transcript("labels", "count", &["list"]);
    assert!(
        stdout.contains("owner 3"),
        "two owner lines and one queued:\n{stdout}"
    );

    let (stdout, _, code) = transcript(
        "labels",
        "search-sub",
        &["search", "--subagents", "reviewer reports"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("eeeeeeee/f00dcafe L2 agent"), "{stdout}");
    assert_eq!(
        transcript("labels", "search-nosub", &["search", "reviewer reports"]).2,
        1
    );

    // A pattern reads a line's whole text, across a line break, and prints the line it starts on.
    let (stdout, _, code) = transcript(
        "labels",
        "search-span",
        &["search", "skill: /x\\nbody line 0"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.contains("eeeeeeee L5 injected skill  Base directory for this skill: /x"),
        "{stdout}"
    );

    // A subagent's line carries its agent's id in `show`.
    let (stdout, _, _) = transcript(
        "labels",
        "show-agent-id",
        &["show", "eeeeeeee", "--subagents"],
    );
    assert!(stdout.contains("[f00dcafe L1 dispatch"), "{stdout}");

    let (stdout, _, _) = transcript(
        "labels",
        "search-tool-only",
        &["search", "--tool", "Bash", "."],
    );
    assert!(
        !stdout.contains(" owner ") && !stdout.contains(" agent "),
        "{stdout}"
    );
}

/// What the second adversarial review found untested: the shapes counted, the rows it mutated, the
/// cuts and excerpts, the header's gap and scope, `search`'s scopes and `--json`.
#[test]
fn transcript_shapes_rows_and_output_forms_hold() {
    let (stdout, _, code) = transcript("extras", "adv-show", &["show", "12345678", "--tools"]);
    assert_eq!(code, 0, "{stdout}");
    let has = |needle: &str| assert!(stdout.contains(needle), "missing {needle:?} in:\n{stdout}");
    // A command's parent that carries a field is no command line, and prints.
    has("owner 2026-02-01 10:00]\nthe owner speaks before a command");
    has("injected meta 2026-02-01 10:00]\na meta line before a command");
    has("injected unknown 2026-02-01 10:00]\na meta line from a robot");
    has("long question line 44");
    // A tool result is never a command's own line, whatever its parent link says.
    has("a result before a command");
    // The shapes the labels cannot read are counted.
    has("# unrecognised assistant content shape \"neither a list nor a string the line takes\": 1");
    has("# unrecognised assistant content item shape \"text with no text\": 1");
    has("# unrecognised queued_command shape \"no prompt\": 1");
    has("# unrecognised origin \"not an object: \\\"human\\\"\": 1");
    has("# unrecognised type \"(none)\": 1");
    has("# not JSON: -proj/12345678-0000-0000-0000-000000000009.jsonl:");

    let (stdout, _, _) = transcript("extras", "adv-list", &["list"]);
    assert!(
        stdout.contains("first owner word"),
        "the first non-empty line:\n{stdout}"
    );

    let (stdout, _, code) = transcript("extras", "adv-search-cut", &["search", "owner w+"]);
    assert_eq!(code, 0, "{stdout}");
    let hit = stdout
        .lines()
        .find(|l| l.contains("owner www"))
        .expect("a hit");
    assert!(hit.ends_with('…') && hit.chars().count() < 250, "{hit}");

    // A long brief is never cut, and agent x's own subagent is placed by its call.
    let (stdout, _, _) = transcript(
        "twofiles",
        "adv-brief",
        &["show", "a1000000", "--subagents"],
    );
    assert!(stdout.contains("briefline 44"), "{stdout}");
    assert!(stdout.contains("brief of nested agent z"), "{stdout}");

    // A subagent's `isMeta` line is injected, not its dispatcher's.
    let (stdout, _, _) = transcript(
        "labels",
        "adv-submeta",
        &["show", "eeeeeeee", "--subagents"],
    );
    assert!(
        stdout.contains("injected meta 2026-01-03 10:00]\na skill loaded in the subagent"),
        "{stdout}"
    );
    // A compaction's line names its trigger.
    let (stdout, _, _) = transcript("split", "adv-trigger", &["show", "aaaaaaaa"]);
    assert!(stdout.contains("compaction, trigger auto"), "{stdout}");

    // The header: the gap on an unknown value alone and on a line that is no JSON alone; `show`
    // reads no subagent file without `--subagents`; a `--from` miss prints the header.
    let (stdout, _, _) = transcript("twofiles", "adv-gap-type", &["list"]);
    assert!(
        stdout.contains("# unrecognised type \"brand-new-type\"")
            && stdout.contains("is a gap of this command"),
        "{stdout}"
    );
    let (stdout, _, _) = transcript("places", "adv-gap-json", &["show", "ffffffff"]);
    assert!(
        stdout.contains("# not JSON") && stdout.contains("is a gap of this command"),
        "{stdout}"
    );
    let (stdout, _, _) = transcript("labels", "adv-files", &["show", "eeeeeeee"]);
    assert!(stdout.contains("# files read: 1;"), "{stdout}");
    let (stdout, _, _) = transcript(
        "labels",
        "adv-from-miss",
        &["show", "eeeeeeee", "--from", "nowhere"],
    );
    assert!(stdout.contains("# files read:"), "{stdout}");

    // A project's file is the one whose first chain line ran in its root.
    let (stdout, _, _) = transcript("places", "adv-mixed", &["list"]);
    assert!(!stdout.contains("77777777 "), "{stdout}");

    // `search`: a later line of a text, `--tools`, `--branch`, and an unknown role refused.
    let (stdout, _, code) = transcript("labels", "adv-s2", &["search", "(?m)body line 3$"]);
    assert_eq!(code, 0, "{stdout}");
    let (stdout, _, code) = transcript("labels", "adv-s3", &["search", "--tools", "file listing"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("tool Bash"), "{stdout}");
    let (stdout, _, _) = transcript("split", "adv-s4", &["search", "--branch", "feature", "."]);
    assert!(
        stdout.contains("bbbbbbbb#2 ") && !stdout.contains("aaaaaaaa "),
        "{stdout}"
    );
    assert_eq!(
        transcript(
            "labels",
            "adv-role",
            &["search", "--role", "owner-anwser", "."]
        )
        .2,
        2
    );

    // `--json`: the header first, then one object per row or line.
    let (stdout, _, code) = transcript("split", "adv-json-list", &["list", "--json"]);
    assert_eq!(code, 0, "{stdout}");
    let first = stdout.lines().next().expect("a line");
    assert!(
        first.starts_with("{\"files\"") || first.contains("\"header\":\"read\""),
        "{stdout}"
    );
    assert!(
        stdout
            .lines()
            .any(|l| l.contains("\"session\":\"aaaaaaaa\"")),
        "{stdout}"
    );
    let (stdout, _, _) = transcript("split", "adv-json-show", &["show", "aaaaaaaa", "--json"]);
    assert!(stdout.contains("\"text\":\"first question\""), "{stdout}");
    assert!(!stdout.lines().any(|l| l.starts_with('[')), "{stdout}");
}

/// From a linked worktree of the project, as every reviewer runs, the project's sessions are those
/// that ran in the main checkout, at the same path.
#[test]
fn transcript_a_linked_worktree_reads_the_sessions_of_its_main_checkout() {
    let sandbox = Sandbox::new("transcript-worktree", "core");
    sandbox.git(&["commit", "-q", "-m", "the mock"]);
    let worktree = sandbox.path("linked");
    sandbox.git(&["worktree", "add", "-q", "--detach", "linked"]);
    let root = sandbox.dir.canonicalize().expect("the sandbox exists");
    let config =
        std::env::temp_dir().join(format!("klarch-transcript-worktree-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&config);
    copy_rendered(
        &transcript_dir().join("tests/transcripts/places"),
        &config,
        &root.display().to_string(),
    );
    let (stdout, stderr, code) = run_with_env(
        &worktree,
        &["transcript", "list"],
        &[("CLAUDE_CONFIG_DIR", &config)],
    );
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains("88888888 "),
        "a session of the main checkout:\n{stdout}"
    );
    assert!(!stdout.contains("99999999 "), "another project:\n{stdout}");
}

/// The claim: from a subdirectory, the sessions are those of the project, not of the working directory:
/// `cli::run` hands `transcript` the project's root, per `design@transcript@transcript-directory-resolution`.
/// Mutation checked: `Path::new(".")` handed in for `manifest.root()`.
#[test]
fn transcript_from_a_subdirectory_reads_the_sessions_of_the_project() {
    let sandbox = Sandbox::new("transcript-subdirectory", "core");
    let sub = sandbox.path("sub");
    std::fs::create_dir(&sub).expect("a subdirectory");
    let root = sandbox.dir.canonicalize().expect("the sandbox exists");
    let config = std::env::temp_dir().join(format!("klarch-transcript-sub-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&config);
    copy_rendered(
        &transcript_dir().join("tests/transcripts/places"),
        &config,
        &root.display().to_string(),
    );
    let (stdout, stderr, code) = run_with_env(
        &sub,
        &["transcript", "list"],
        &[("CLAUDE_CONFIG_DIR", &config)],
    );
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(
        stdout.contains("88888888 "),
        "the project's session:\n{stdout}"
    );
}

/// The kinds of line harness versions 2.1.310 to 2.1.316 added, each classed: an interrupt placed by
/// its `interruptedMessageId`, a refusal shown, the `/remote-control` echo and the metadata, status
/// and reminder lines ignored, so the header names none of them and asks for no report. The
/// fixture's last line is of the metadata type `session-owner`. Mutation checked: that type
/// removed from `KNOWN_METADATA_TYPES` names it in the header.
#[test]
fn transcript_the_kinds_of_later_harness_versions_are_classed() {
    let (stdout, stderr, code) =
        transcript("kinds", "kinds-show", &["show", "abcdef01", "--tools"]);
    assert_eq!(code, 0, "{stdout}{stderr}");
    let has = |needle: &str| assert!(stdout.contains(needle), "missing {needle:?} in:\n{stdout}");
    has("owner-interrupt 2026-04-01 10:00]\n[Request interrupted by user]");
    has("injected model-refusal 2026-04-01 10:00]\nmodel refusal, category some_category");
    has("owner-command 2026-04-01 10:00]\n/remote-control\n");
    // The field places an interrupt only on a line no other field places.
    has("injected meta 2026-04-01 10:00]\na meta line that names an interrupt");
    has("injected model-refusal 2026-04-01 10:00]\nmodel refusal, category unknown");
    assert_eq!(stdout.matches("remote-control").count(), 1, "{stdout}");
    assert!(!stdout.contains("# unrecognised"), "{stdout}");
    assert!(!stdout.contains("# unlabelled"), "{stdout}");
    assert!(!stdout.contains("gap"), "{stdout}");

    // An interrupt with no id, from harness 2.1.316, is placed by its whole text, on a line no
    // field places.
    has("[L16 owner-interrupt 2026-04-01 10:00]\n[Request interrupted by user for tool use]");
    // A command the owner ran with `!`, and its output; a typed message that opens with the tag
    // stays the owner's message, since its field places it first.
    has("[L17 owner-shell 2026-04-01 10:00]\n<bash-input>git status</bash-input>");
    has("[L18 injected shell-output 2026-04-01 10:00]\n<bash-stdout>nothing to commit");
    has("[L19 owner 2026-04-01 10:00]\n<bash-input>pasted by the owner");
    // A shell command is the owner's act, so it is never cut, however long.
    has("shell line 45</bash-input>");

    // `--role` takes the shell command's role.
    let (stdout, _, code) = transcript(
        "kinds",
        "kinds-shell",
        &["search", "--role", "owner-shell", "."],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("abcdef01 L17 owner-shell"), "{stdout}");
    assert!(!stdout.contains("shell-output"), "{stdout}");

    // An interrupt and a shell command are the owner's acts, not lines the owner typed: the count
    // holds four messages.
    let (stdout, _, _) = transcript("kinds", "kinds-list", &["list"]);
    assert!(stdout.contains("owner 4 "), "{stdout}");
    let (stdout, _, code) = transcript(
        "kinds",
        "kinds-role",
        &["search", "--role", "owner-interrupt", "."],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("abcdef01 L5 owner-interrupt"), "{stdout}");

    // In a subagent file, the field places an interrupt before the row of the dispatcher's text.
    let (stdout, _, _) = transcript("kinds", "kinds-sub", &["show", "abcdef01", "--subagents"]);
    assert!(stdout.contains("[0000000k L1 dispatch"), "{stdout}");
    assert!(
        stdout.contains("[0000000k L2 owner-interrupt 2026-04-01 10:01]"),
        "{stdout}"
    );
    // A null id is no id: the line stays the dispatcher's.
    assert!(stdout.contains("[0000000k L3 dispatch"), "{stdout}");
    // An interrupt with no id is placed by its whole text before the dispatcher's row; a shell tag
    // in a subagent file places nothing.
    assert!(stdout.contains("[0000000k L4 owner-interrupt"), "{stdout}");
    assert!(stdout.contains("[0000000k L5 dispatch"), "{stdout}");
    // A fork's first line names the parent's line up to which it inherits its context, and is no
    // unrecognised type.
    // It is the fork's first entry, timed by the fork's first chain line, and the fork's copy of the
    // parent's call that dispatched it, with that call's result, is not shown again.
    let fork = stdout
        .split("# subagent abcdef01/0000000f")
        .nth(1)
        .and_then(|rest| rest.split_once('\n'))
        .map(|(_, body)| body)
        .expect("the fork's section");
    assert!(
        fork.starts_with(
            "[0000000f L1 injected fork-context 2026-04-01 10:02]\na fork: it inherits its \
             parent's context up to L12 (contextLength 12)\n"
        ),
        "{stdout}"
    );
    let (calls, _, _) = transcript(
        "kinds",
        "kinds-fork-calls",
        &["search", "--subagents", "--tool", "Agent", "."],
    );
    assert!(!calls.contains("0000000f"), "{calls}");
    assert!(
        stdout.contains("[0000000f L3 dispatch 2026-04-01 10:02]\nthe brief of fork f"),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "[0000000g L1 injected fork-context 2026-04-01 10:03]\na fork: it inherits its \
             parent's context up to the line \"no-such-uuid\", which the parent's file does not \
             hold (contextLength 12)"
        ),
        "{stdout}"
    );
    assert!(!stdout.contains("# lines skipped"), "{stdout}");
    assert!(!stdout.contains("# unrecognised"), "{stdout}");
    assert!(!stdout.contains("# unlabelled"), "{stdout}");

    // A `local_command` line with no `commandRun` is an echo only where a line that carries one
    // names it as its parent: two in a row are both reported.
    let (stdout, _, _) = transcript("kinds", "kinds-echoes", &["show", "abcdef02"]);
    assert!(
        stdout.contains("# unrecognised system local_command shape \"no commandRun\": 2"),
        "{stdout}"
    );
}

/// `list` previews the words inside a pasted-content tag, never the tag.
#[test]
fn transcript_the_list_previews_the_words_inside_a_pasted_content_tag() {
    let (stdout, _, code) = transcript("kinds", "kinds-preview", &["list"]);
    assert_eq!(code, 0, "{stdout}");
    let row = stdout
        .lines()
        .find(|l| l.starts_with("abcdef01 "))
        .expect("the session's row");
    assert!(
        row.ends_with("please run\u{a0}the pasted plan now"),
        "{row}"
    );
    assert!(!row.contains("pasted_content"), "{row}");
}

/// `--containing` and `--from` match a non-breaking space in the owner's words with a typed space,
/// and the other way round.
#[test]
fn transcript_the_owner_s_words_match_across_a_non_breaking_space() {
    let (stdout, _, code) = transcript(
        "kinds",
        "kinds-nbsp-list",
        &["list", "--containing", "please run the pasted"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains("abcdef01 "), "{stdout}");
    let (stdout, _, code) = transcript(
        "kinds",
        "kinds-nbsp-from",
        &["show", "abcdef01", "--from", "a second message"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(!stdout.contains("[L1 owner"), "{stdout}");
    let code = transcript(
        "labels",
        "labels-nbsp-typed",
        &["list", "--containing", "and\u{a0}my words"],
    )
    .2;
    assert_eq!(code, 0);
    // A line break is no space.
    let code = transcript(
        "kinds",
        "kinds-nbsp-break",
        &["list", "--containing", "one line"],
    )
    .2;
    assert_eq!(code, 1);
}

/// The request to report a gap names what it covers, and a log with no field, whose lines no row
/// places, asks for it.
#[test]
fn transcript_the_gap_request_names_what_it_covers() {
    let (stdout, _, _) = transcript("labels", "gap-words", &["show", "eeeeeeee"]);
    assert!(
        stdout.contains(
            "# each unrecognised value, unlabelled line and line that is no JSON above is a gap"
        ),
        "{stdout}"
    );
    let (stdout, _, _) = transcript("places", "gap-nofield", &["show", "88888888"]);
    assert!(stdout.contains("is a gap of this command"), "{stdout}");
}

/// The session of the case `gaps` that holds the cut texts, named by a prefix of its `sessionId`:
/// its handle is one the case's other session carries too.
const GAPS_SESSION: &str = "9a9a9a9a-0000-0000-0000-000000000020";

/// The claim: each text the harness cut, in a line its fields label injected or tool, is named in
/// the header with its line, its count and its pointer, or with no file named where the marker
/// names none; a marker in the agent's text, and a quotation of one, are no cut; and a cut is no
/// gap of the command.
#[test]
fn transcript_a_text_the_harness_cut_is_named_in_the_header() {
    let (stdout, _, code) = transcript("gaps", "gaps-cuts", &["show", GAPS_SESSION]);
    assert_eq!(code, 0, "{stdout}");
    let file = "-proj/9a9a9a9a-0000-0000-0000-000000000020.jsonl";
    assert!(
        stdout.contains(&format!(
            "# cut by the harness: {file}:3, 31934 chars not in the transcript, the whole in \
             /cache/tasks/w1.output\n"
        )),
        "{stdout}"
    );
    assert!(
        stdout.contains(&format!(
            "# cut by the harness: {file}:6, 742 chars not in the transcript, and no file named\n"
        )),
        "{stdout}"
    );
    assert!(!stdout.contains(&format!("{file}:7,")), "{stdout}");
    assert_eq!(
        stdout.matches("# cut by the harness").count(),
        2,
        "{stdout}"
    );
    let (stdout, _, _) = transcript(
        "gaps",
        "gaps-cuts-sub",
        &["show", GAPS_SESSION, "--subagents", "--json"],
    );
    assert!(
        stdout.contains(r#""at":"-proj/9a9a9a9a-0000-0000-0000-000000000020/subagents/agent-c0ffee000001.jsonl:3""#),
        "{stdout}"
    );
    assert!(
        stdout.contains(r#""whole_in":"/cache/tasks/w2.output""#),
        "{stdout}"
    );
}

/// The claim: a file whose first chain line holds no `cwd` is left out of a run over one project,
/// and the header names it as a gap; a run over every project reads it.
#[test]
fn transcript_a_file_with_no_cwd_is_reported_where_it_is_left_out() {
    let (stdout, _, code) = transcript("gaps", "gaps-nocwd", &["list"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.contains(
            "# unrecognised first chain line \"no cwd\": 1, first at \
             -proj/7b7b7b7b-0000-0000-0000-000000000022.jsonl:2\n"
        ),
        "{stdout}"
    );
    assert!(stdout.contains("is a gap of this command"), "{stdout}");
    assert!(!stdout.contains("7b7b7b7b "), "{stdout}");
    let (stdout, _, _) = transcript("gaps", "gaps-nocwd-all", &["list", "--all-projects"]);
    assert!(stdout.contains("7b7b7b7b "), "{stdout}");
    assert!(!stdout.contains("no cwd"), "{stdout}");
}

/// The claim: a handle two sessions carry names both, so `show` refuses it with each `sessionId`,
/// exit 1, after the header; a session it cannot name is refused after the header too.
#[test]
fn transcript_show_refuses_a_handle_two_sessions_carry_after_its_header() {
    let (stdout, _, code) = transcript("gaps", "gaps-twohandles", &["show", "9a9a9a9a"]);
    assert_eq!(code, 1, "{stdout}");
    assert!(
        stdout.contains(
            "\"9a9a9a9a\" names 2 sessions: 9a9a9a9a-0000-0000-0000-000000000020, \
             9a9a9a9a-0000-0000-0000-000000000021"
        ),
        "{stdout}"
    );
    assert!(stdout.starts_with("# files read: 2;"), "{stdout}");
    let (stdout, _, code) = transcript("gaps", "gaps-unknown", &["show", "zzzzzzzz"]);
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.starts_with("# files read: 2;"), "{stdout}");
    assert!(
        stdout.contains("no session is named \"zzzzzzzz\""),
        "{stdout}"
    );
}

/// The claim: `--role tool` keeps the tool lines, as `--tools` does, rather than keep nothing.
#[test]
fn transcript_search_role_tool_keeps_the_tool_lines() {
    let (stdout, _, code) = transcript(
        "gaps",
        "gaps-roletool",
        &["search", "--role", "tool", "subagent's report"],
    );
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.contains("L6 tool Agent  the subagent's report"),
        "{stdout}"
    );
    let (stdout, _, code) = transcript(
        "gaps",
        "gaps-roleowner",
        &["search", "--role", "owner", "subagent's report"],
    );
    assert_eq!(code, 1, "{stdout}");
}

/// The claim: `--from-line` starts at the first kept line at that number or after it, the agent's
/// included; past the last kept line it prints the header and exits 1; with `--from` it is refused.
#[test]
fn transcript_show_starts_at_a_line_number() {
    let (stdout, _, code) = transcript(
        "gaps",
        "gaps-fromline",
        &["show", GAPS_SESSION, "--from-line", "4"],
    );
    assert_eq!(code, 0, "{stdout}");
    let body = &stdout[stdout.find("# session").expect("the session's title")..];
    assert!(
        body.lines()
            .nth(1)
            .is_some_and(|l| l.starts_with("[L4 agent ")),
        "{stdout}"
    );
    assert!(!body.contains("[L3 "), "{stdout}");
    assert!(body.contains("[L8 owner "), "{stdout}");
    let (stdout, _, code) = transcript(
        "gaps",
        "gaps-fromline-gap",
        &["show", GAPS_SESSION, "--from-line", "5"],
    );
    assert_eq!(code, 0, "{stdout}");
    let body = &stdout[stdout.find("# session").expect("the session's title")..];
    assert!(
        body.lines()
            .nth(1)
            .is_some_and(|l| l.starts_with("[L7 agent ")),
        "a tool line is not kept without --tools, so L7 is the first:\n{stdout}"
    );
    let (stdout, _, code) = transcript(
        "gaps",
        "gaps-fromline-past",
        &["show", GAPS_SESSION, "--from-line", "9"],
    );
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.starts_with("# files read"), "{stdout}");
    assert!(stdout.contains("is at L9 or after it"), "{stdout}");
    let (_, stderr, code) = transcript(
        "gaps",
        "gaps-fromline-both",
        &["show", GAPS_SESSION, "--from-line", "4", "--from", "start"],
    );
    assert_eq!(code, 2, "{stderr}");
}

/// The claim: `--from` matches only a line the owner typed, so words the agent wrote start nothing.
#[test]
fn transcript_from_matches_no_line_the_agent_wrote() {
    let (stdout, _, code) = transcript(
        "gaps",
        "gaps-from-agent",
        &[
            "show",
            GAPS_SESSION,
            "--from",
            "the work of the second branch",
        ],
    );
    assert_eq!(code, 1, "{stdout}");
    assert!(stdout.contains("no line the owner typed in"), "{stdout}");
}

/// The claim: a file whose `cwd` reaches the project's root through a symbolic link is the
/// project's, since both sides are canonicalised before they are compared.
#[test]
fn transcript_a_cwd_through_a_symlink_is_the_project_s() {
    let root = project("core").canonicalize().expect("the mock exists");
    let config =
        std::env::temp_dir().join(format!("klarch-transcript-symlink-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&config);
    let projects = config.join("projects/-linked");
    std::fs::create_dir_all(&projects).expect("a transcript directory");
    let link = config.join("link");
    std::os::unix::fs::symlink(&root, &link).expect("a symlink");
    let line = format!(
        r#"{{"type":"user","uuid":"l1","parentUuid":null,"sessionId":"5e5e5e5e-0000-0000-0000-000000000030","timestamp":"2026-02-01T10:00:00.000Z","cwd":"{}","version":"2.1.316","message":{{"role":"user","content":"through the link"}},"origin":{{"kind":"human"}}}}"#,
        link.display()
    );
    std::fs::write(
        projects.join("5e5e5e5e-0000-0000-0000-000000000030.jsonl"),
        line + "\n",
    )
    .expect("a transcript");
    let (stdout, stderr, code) = run_with_env(
        &project("core"),
        &["transcript", "list"],
        &[("CLAUDE_CONFIG_DIR", &config)],
    );
    let _ = std::fs::remove_dir_all(&config);
    assert_eq!(code, 0, "{stdout}{stderr}");
    assert!(stdout.contains("5e5e5e5e "), "{stdout}");
}

/// The claim: `klarch` lists the checker's library among those it links, so a copy of it at another
/// path than the binary's is refused, and the refusal names its package.
#[test]
fn a_copy_of_the_checker_s_library_is_refused() {
    let sandbox = Sandbox::seeded(
        "foreign-check",
        "minimal",
        &[(
            "crates/check/Cargo.toml",
            "[package]\nname = \"knowledge-architect-check\"\n",
        )],
    );
    let (out, stderr, code) = sandbox.run(&["issues"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains("its `knowledge-architect-check` was compiled"),
        "{stderr}"
    );
}

/// The claim of a refused invocation: exit 2, and the project left as it was.
fn refused_with_an_untouched_tree(sandbox: &Sandbox, args: &[&str]) {
    let status = || {
        let out = sandbox
            .git_command(&["status", "--porcelain", "--ignored"])
            .output()
            .expect("git status");
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    let before = status();
    let (out, stderr, code) = sandbox.run(args);
    assert_eq!(code, 2, "{args:?}: {out}{stderr}");
    assert_eq!(status(), before, "{args:?} touched the tree");
}

/// A sandbox, a work list `worklist --out` wrote outside it, which `--verify` would otherwise
/// answer about with exit 1, and a directory outside it that holds nothing, which `--out` would
/// otherwise write. So the refusal is the only thing that can make these runs exit 2.
fn with_a_work_list(tag: &str) -> (Sandbox, String, PathBuf) {
    let sandbox = Sandbox::new(tag, "dirhome");
    let outside = std::env::temp_dir().join(format!("{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&outside);
    std::fs::create_dir_all(&outside).expect("a directory outside the project");
    let written = outside.join("written");
    let (out, stderr, code) = sandbox.run(&["worklist", "--out", written.to_str().unwrap()]);
    assert_eq!(code, 0, "{out}{stderr}");
    let fresh = outside.join("fresh");
    (sandbox, written.to_str().unwrap().to_string(), fresh)
}

#[test]
fn check_fix_with_staged_is_refused_in_a_tree_it_leaves_as_it_was() {
    let sandbox = Sandbox::new("refused-fix-staged", "minimal");
    refused_with_an_untouched_tree(&sandbox, &["check", "--fix", "--staged"]);
}

#[test]
fn worklist_out_with_verify_is_refused() {
    let (sandbox, written, fresh) = with_a_work_list("refused-out-verify");
    let args = [
        "worklist",
        "--out",
        fresh.to_str().unwrap(),
        "--verify",
        &written,
    ];
    refused_with_an_untouched_tree(&sandbox, &args);
    assert!(!fresh.exists(), "the refused run wrote the work list");
}

#[test]
fn worklist_with_neither_out_nor_verify_is_refused() {
    let sandbox = Sandbox::new("refused-worklist-neither", "minimal");
    refused_with_an_untouched_tree(&sandbox, &["worklist"]);
}

#[test]
fn worklist_verify_with_a_bound_is_refused() {
    let (sandbox, written, _) = with_a_work_list("refused-verify-bound");
    refused_with_an_untouched_tree(
        &sandbox,
        &["worklist", "--verify", &written, "--bound", "5"],
    );
}

#[test]
fn worklist_verify_with_a_seed_is_refused() {
    let (sandbox, written, _) = with_a_work_list("refused-verify-seed");
    let args = ["worklist", "--verify", &written, "--seed", "design@a-b@c-d"];
    refused_with_an_untouched_tree(&sandbox, &args);
}

/// A command line of the shipped declaration, so the walk below names no command.
#[derive(clap::Parser)]
struct Declared {
    #[command(subcommand)]
    command: knowledge_architect::cli::Command,
}

/// Every leaf command of `command`, as the arguments that reach it.
fn leaves(command: &clap::Command, path: Vec<String>, out: &mut Vec<Vec<String>>) {
    let subcommands: Vec<&clap::Command> = command
        .get_subcommands()
        .filter(|c| c.get_name() != "help")
        .collect();
    if subcommands.is_empty() {
        out.push(path);
        return;
    }
    for sub in subcommands {
        let mut next = path.clone();
        next.push(sub.get_name().to_string());
        leaves(sub, next, out);
    }
}

/// The claim of `design@cli@a-refusal-by-arguments-writes-nothing`, over every command the declaration
/// holds, a future one included: an argument the command does not declare is exit 2, and the
/// project is left as it was.
#[test]
fn every_command_refuses_an_argument_it_does_not_declare() {
    use clap::CommandFactory;
    let mut commands = Vec::new();
    leaves(&Declared::command(), Vec::new(), &mut commands);
    assert!(commands.len() > 1, "the walk found {commands:?}");
    let sandbox = Sandbox::new("refused-unknown", "minimal");
    for command in commands {
        let mut args: Vec<&str> = command.iter().map(String::as_str).collect();
        args.push("--no-such-argument");
        refused_with_an_untouched_tree(&sandbox, &args);
    }
}

#[test]
fn a_command_missing_a_required_argument_is_refused() {
    let sandbox = Sandbox::new("refused-missing", "minimal");
    refused_with_an_untouched_tree(&sandbox, &["show"]);
}

/// The directory holds nothing and lies outside the project, so `--out` would write it were the
/// bound accepted.
#[test]
fn a_value_the_argument_cannot_read_is_refused() {
    let (sandbox, _, fresh) = with_a_work_list("refused-unreadable");
    let args = [
        "worklist",
        "--out",
        fresh.to_str().unwrap(),
        "--bound",
        "many",
    ];
    refused_with_an_untouched_tree(&sandbox, &args);
    assert!(!fresh.exists(), "the refused run wrote the work list");
}

/// The claim: `tripwires` takes the text it filters on.
#[test]
fn tripwires_filter_on_their_terms() {
    let sandbox = Sandbox::new("tripwires-terms", "dirhome");
    let (out, stderr, code) = sandbox.run(&["tripwires", "no-row-holds-this-text"]);
    assert_eq!(code, 1, "{out}{stderr}");
}

/// The claim: `klarch` lists the crate of installed text among those it links.
#[test]
fn a_copy_of_the_installed_text_s_crate_is_refused() {
    let sandbox = Sandbox::seeded(
        "foreign-skills",
        "minimal",
        &[(
            "crates/agent-skills/Cargo.toml",
            "[package]\nname = \"knowledge-architect-agent-skills\"\n",
        )],
    );
    let (out, stderr, code) = sandbox.run(&["issues"]);
    assert_eq!(code, 2, "{out}{stderr}");
    assert!(
        stderr.contains("its `knowledge-architect-agent-skills` was compiled"),
        "{stderr}"
    );
}

/// The claim: a path reference whose path holds a shape the resolver refuses, a leading slash or
/// an upward segment, at a named anchor or at `*`, is one finding at its line, and the rest of the
/// run goes on: git is asked nothing about such a path, since `git check-ignore` fails the whole
/// batch over a path outside the repository. Mutation checked: removing the test of `refused` from
/// the span arm of `ignore_queries` makes the run exit 2 with git's `outside repository`.
#[test]
fn a_refused_path_shape_is_one_finding_and_the_run_goes_on() {
    let sandbox = Sandbox::seeded(
        "refused-span",
        "core",
        &[(
            "README.md",
            "# core-mock\n\nA mock project for the binary.\n\n\
             See `path@core-mock@/etc/passwd`, `path@core-mock@../outside.md` and `path@*@/x.md`.\n",
        )],
    );
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(!err.contains("check-ignore"), "{err}");
    assert_eq!(
        out.matches("a leading slash is outside the project's namespace")
            .count(),
        2,
        "{out}"
    );
    assert_eq!(
        out.matches("an upward segment is anchored at the wrong place by definition")
            .count(),
        1,
        "{out}"
    );
}

// --- the register `goal` is the root's alone, and is cited with no anchor ----------------------

/// The `core` mock with a Component `part` that carries every home it owes, and `extra` written
/// beside them; staged.
fn core_with_a_part(tag: &str, extra: &[(&str, &str)]) -> Sandbox {
    let manifest = std::fs::read_to_string(source("core").join("knowledge-architect.toml"))
        .expect("the core manifest");
    let mut files: Vec<(&str, String)> = vec![
        (
            "knowledge-architect.toml",
            manifest.replace("components = []", "components = [\"part\"]"),
        ),
        ("part/README.md", "# part\n\nA Component.\n".to_string()),
        (
            "part/docs/design.md",
            "# part — design\n\nNo entry.\n".to_string(),
        ),
        (
            "part/docs/tripwires.md",
            "# part — tripwires\n\nNo entry.\n".to_string(),
        ),
        (
            "part/docs/rejected.md",
            "# part — rejected\n\nNo entry.\n".to_string(),
        ),
        (
            "part/docs/open-issues/README.md",
            "# part — open issues\n".to_string(),
        ),
        // The generated index of an issue home with no entry, as the root's in `core`; written
        // here since `index` refuses a tree holding a phase-2 finding.
        (
            "part/docs/open-issues/index.md",
            "**Generated — do not edit.** `klarch index`\n\n0 entries\n".to_string(),
        ),
    ];
    files.extend(extra.iter().map(|(p, t)| (*p, t.to_string())));
    let borrowed: Vec<(&str, &str)> = files.iter().map(|(p, t)| (*p, t.as_str())).collect();
    Sandbox::seeded(tag, "core", &borrowed)
}

/// The claim: a goals home in a Component other than the root is refused, at phase 2, with the
/// repair of its case: deleted where it holds no goal, its goals moved where it holds one; and the
/// same Component with no goals home passes. Mutations checked, in `foreign_root_homes` of
/// `path@model@src/tree.rs`: the presence test made `if true {`, which reports nothing; and
/// `holds_entries` made `false`, which names the deletion for a home holding a goal; each caught.
#[test]
fn a_goals_home_in_a_component_is_refused_with_the_repair_of_its_case() {
    let (out, err, code) = core_with_a_part("goals-home-none", &[]).run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");

    let empty = core_with_a_part(
        "goals-home-empty",
        &[("part/docs/goals.md", "# Goals — part\n\nNo entry.\n")],
    );
    let (out, err, code) = empty.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains("`part/docs/goals.md` is a home of the goal register in the component `part`"),
        "{out}"
    );
    assert!(out.contains("delete it; it holds no entry"), "{out}");
    assert!(out.contains("phase"), "{out}");

    let holding = core_with_a_part(
        "goals-home-holding",
        &[(
            "part/docs/goals.md",
            "# Goals — part\n\n## A goal of the part `##part-goal`\n\nIt.\n",
        )],
    );
    let (out, err, code) = holding.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("move each entry to the root's home"), "{out}");
    assert!(!out.contains("delete it; it holds no entry"), "{out}");

    // A heading with no slug defines no goal, so its home is deleted like an empty one.
    let unslugged = core_with_a_part(
        "goals-home-unslugged",
        &[(
            "part/docs/goals.md",
            "# Goals — part\n\n## Notes\n\nProse.\n",
        )],
    );
    let (out, err, code) = unslugged.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("delete it; it holds no entry"), "{out}");
}

/// The claim: a goals home in a Component in its directory shape is refused too, its repair by
/// what its files hold. Mutations checked, in `foreign_root_homes` of `path@model@src/tree.rs`:
/// `for home in [file, base] {` made `for home in [file] {`, which leaves the directory unreported;
/// and the file filter `d.rel == home || d.rel.starts_with(&home)` made `d.rel == home`, which tells
/// a directory holding a goal to be deleted; each caught.
#[test]
fn a_goals_home_in_a_component_in_its_directory_shape_is_refused() {
    let empty = core_with_a_part(
        "goals-dir-empty",
        &[("part/docs/goals/README.md", "# Goals — part\n\nNo entry.\n")],
    );
    let (out, err, code) = empty.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains("`part/docs/goals` is a home of the goal register in the component `part`"),
        "{out}"
    );
    assert!(out.contains("delete it; it holds no entry"), "{out}");

    let holding = core_with_a_part(
        "goals-dir-holding",
        &[
            (
                "part/docs/goals/README.md",
                "# Goals — part\n\n- [One](one.md)\n",
            ),
            (
                "part/docs/goals/one.md",
                "# One\n\n## A goal of the part `##part-goal`\n\nIt.\n",
            ),
        ],
    );
    let (out, err, code) = holding.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("move each entry to the root's home"), "{out}");
}

/// The claim: a goal is cited `goal@<id>` whichever theme file of the root's goals home in its
/// directory shape holds it. Test: the `dirhome` mock, whose goals home is a directory, with a
/// second theme file and a design head citing a goal of each.
#[test]
fn a_goal_is_cited_with_no_anchor_whatever_theme_file_holds_it() {
    let sandbox = Sandbox::seeded(
        "goal-theme-files",
        "dirhome",
        &[(
            "docs/goals/two-goal.md",
            "# Two\n\n## A second goal `##second-goal`\n\nIt.\n",
        )],
    );
    let readme = std::fs::read_to_string(sandbox.path("docs/goals/README.md")).expect("a README");
    sandbox.write(
        "docs/goals/README.md",
        &format!("{readme}- [Two](two-goal.md)\n"),
    );
    let first = std::fs::read_to_string(sandbox.path("docs/goals/one-goal.md")).expect("one goal");
    let slug = first
        .lines()
        .find_map(|l| {
            l.split("`##")
                .nth(1)
                .map(|s| s.trim_end_matches('`').to_string())
        })
        .expect("a slug");
    let design =
        std::fs::read_to_string(sandbox.path("docs/design/README.md")).expect("a design home");
    sandbox.write(
        "docs/design/README.md",
        &format!("{design}\nIt serves `goal@{slug}` and `goal@second-goal`.\n"),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim: a location that declares the register `goal` is refused, the register dropped from
/// its row and the run stopped at phase 1, the manifest's phase. Mutation checked: the refusal in `resolve_anchors` of
/// `path@model@src/manifest.rs`, `if known.scope == Scope::Root {` made `if false {`; caught.
#[test]
fn a_location_that_declares_the_goal_register_is_refused() {
    let manifest = std::fs::read_to_string(source("core").join("knowledge-architect.toml"))
        .expect("the core manifest");
    let manifest =
        format!("{manifest}\n[locations.notes]\npath = \"notes\"\nregisters = [\"goal\"]\n");
    let sandbox = Sandbox::seeded(
        "goal-location",
        "core",
        &[
            ("knowledge-architect.toml", &manifest),
            ("notes/goals.md", "# Goals — notes\n\nNo entry.\n"),
        ],
    );
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains("[locations.notes] carries `goal`, which the project's root alone carries"),
        "{out}"
    );
    assert!(!out.contains("notes/goals.md:"), "{out}");
}

/// The claim: a goal is cited `goal@<id>` and the root's goals home `register@goal`, and each
/// anchored form is refused with the form with no anchor as its repair. Mutations checked, in
/// `candidate` of `path@model@src/entity.rs`: the arm of a root-scoped register made to fall
/// through, `if anchors.registers.is_root_register(kind.name()) {` made `if false {`, which
/// refuses `goal@<id>` as two segments; the `register` arm's no-anchor case made to fall
/// through, `None if anchors.registers.is_root_register(rest)` made `None if false`, which refuses
/// `register@goal`; and its anchored case, `Some((_, id)) if anchors.registers.is_root_register(id)`
/// made `Some((_, id)) if false`, which accepts `register@core-mock@goal`; each caught.
#[test]
fn a_goal_is_cited_with_no_anchor_and_each_anchored_form_is_refused() {
    let goals = "# Goals — core-mock\n\n## A goal of the root `##core-goal`\n\nIt.\n";
    let cited = core_with_a_part(
        "goal-forms-ok",
        &[
            ("docs/goals.md", goals),
            (
                "part/docs/design.md",
                "# part — design\n\nIt serves `goal@core-goal`, in `register@goal`.\n",
            ),
        ],
    );
    let (out, err, code) = cited.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");

    let anchored = core_with_a_part(
        "goal-forms-anchored",
        &[
            ("docs/goals.md", goals),
            (
                "part/docs/design.md",
                "# part — design\n\nIt serves `goal@core-mock@core-goal`, in \
                 `register@core-mock@goal`, or `register@*@goal`.\n",
            ),
        ],
    );
    let (out, err, code) = anchored.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(
        out.contains("`goal@core-mock@core-goal` is malformed")
            && out.contains("write `<kind>@<id>` with no anchor"),
        "{out}"
    );
    for span in ["`register@core-mock@goal`", "`register@*@goal`"] {
        assert!(
            out.contains(&format!("{span} is malformed")),
            "{span}: {out}"
        );
    }
    assert!(
        out.contains("write `register@<register>` with no anchor"),
        "{out}"
    );
}

/// The claim: `show` resolves a goal and the root's goals home in the forms with no anchor, as the
/// check does, and lists each citation; `id` prints a goal in that form; an anchored form is
/// refused by `show` as malformed. Mutation checked: `spelled` of `path@model@src/entity.rs`
/// spelling every reference with its anchor, `if anchor.is_empty() || root_register(kind) ||`
/// made `if anchor.is_empty() ||`, which makes `id` print `goal@core-mock@core-goal`; caught.
#[test]
fn show_and_id_answer_a_goal_in_the_form_with_no_anchor() {
    let sandbox = core_with_a_part(
        "goal-forms-show",
        &[
            (
                "docs/goals.md",
                "# Goals — core-mock\n\n## A goal of the root `##core-goal`\n\nIt.\n",
            ),
            (
                "part/docs/design.md",
                "# part — design\n\nIt serves `goal@core-goal`, in `register@goal`.\n",
            ),
        ],
    );
    let (out, err, code) = sandbox.run(&["show", "goal@core-goal"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("part/docs/design.md:3"), "{out}");
    let (out, err, code) = sandbox.run(&["show", "register@goal"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(out.contains("docs/goals.md"), "{out}");
    for refused in [
        "goal@core-mock@core-goal",
        "register@core-mock@goal",
        "register@*@goal",
    ] {
        let (out, err, code) = sandbox.run(&["show", refused]);
        assert_eq!(code, 2, "{refused}: {out}{err}");
        assert!(err.contains("is malformed"), "{refused}: {err}");
    }
    let (out, err, code) = sandbox.run(&["id", "core-goal"]);
    assert_eq!(code, 1, "{out}{err}");
    assert!(out.contains("goal@core-goal"), "{out}");
    assert!(!out.contains("goal@core-mock@"), "{out}");
}
