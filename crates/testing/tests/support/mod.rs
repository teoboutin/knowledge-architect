//! What the test files that run the binary share: `klarch`, built from this checkout, and
//! `Sandbox`, a throwaway copy of a mock project to run it in.

// Each test file that includes this module uses what it needs of it.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// The real binary, built from this checkout once per run of this test file.
///
/// `CARGO_BIN_EXE_klarch` is set in `klarch`'s own package alone, and a dev-dependency on it does not
/// build its binary, so this test asks the cargo that runs it, per
/// `design@testing@nested-cargo-builds-the-binary`. The path is the one cargo reports for the build, so a
/// `CARGO_BUILD_TARGET` or any other setting that moves the output cannot leave a test judging an older
/// binary. A binary that does not build fails each test that asks for it: a mutation run owes a
/// `--build` probe that builds `klarch`, or a mutant that does not compile reads as caught.
pub fn klarch() -> &'static Path {
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
pub fn built_executable(messages: &str) -> Option<PathBuf> {
    const KEY: &str = "\"executable\":\"";
    let line = messages
        .lines()
        .rfind(|l| l.contains("\"name\":\"klarch\"") && l.contains(KEY))?;
    let rest = &line[line.find(KEY)? + KEY.len()..];
    Some(PathBuf::from(&rest[..rest.find('"')?]))
}

/// One mock project as it sits in this crate, the source of every copy.
pub fn source(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/projects")
        .join(name)
}

/// Stdout, stderr and the exit code of one run in a directory.
pub fn run_in(dir: &Path, args: &[&str]) -> (String, String, i32) {
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

/// A throwaway copy of a mock project, pinned to this binary's version, removed when dropped.
///
/// **The copy is a git repository of its own.** The walk is `git ls-files` from the project
/// root, so a copy outside any worktree walks nothing and every test over it passes for the
/// wrong reason. `git init` and `git add -A` are what make the copied files live, and anything
/// a test wants staged — a `.gitignore` above all — is written by `seeded` BEFORE the add.
///
/// **It sits in the temporary directory**, under a random name, so two runs do not meet, and it is
/// outside every worktree, this repository's included, as long as that directory is. The project is
/// a directory of the sandbox's own, so a test has a place outside the project that goes with it,
/// `outside`.
pub struct Sandbox {
    scratch: tempfile::TempDir,
    pub dir: PathBuf,
}

impl Sandbox {
    pub fn new(tag: &str, from: &str) -> Self {
        Self::seeded(tag, from, &[])
    }

    /// The same, with extra files written into the copy before it is staged.
    pub fn seeded(tag: &str, from: &str, files: &[(&str, &str)]) -> Self {
        let sandbox = Self::without_git(tag, from);
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
    pub fn without_git(tag: &str, from: &str) -> Self {
        let scratch =
            tempfile::TempDir::with_prefix(format!("klarch-{tag}-")).expect("a scratch directory");
        let dir = scratch.path().join("project");
        copy_dir(&source(from), &dir);
        Sandbox { scratch, dir }
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
    pub fn stage(&self) {
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
    pub fn git_command(&self, args: &[&str]) -> Command {
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
    pub fn git(&self, args: &[&str]) {
        let out = self.git_command(args).output().expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?} in the sandbox: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    pub fn path(&self, rel: &str) -> PathBuf {
        self.dir.join(rel)
    }

    /// A path outside the project, under the sandbox's own directory. It holds nothing until the test
    /// writes it.
    pub fn outside(&self, rel: &str) -> PathBuf {
        self.scratch.path().join(rel)
    }

    /// Write a file into the copy; a manifest is written pinned to this binary's version.
    pub fn write(&self, rel: &str, text: &str) {
        let path = self.path(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
        let text = if rel == "knowledge-architect.toml" {
            pinned(text)
        } else {
            text.to_string()
        };
        std::fs::write(&path, text).expect("a written fixture file");
    }

    pub fn run(&self, args: &[&str]) -> (String, String, i32) {
        run_in(&self.dir, args)
    }

    /// Make the copy serve the `claude` harness, as a project that adopted the workflow does: the
    /// manifest declares the harness, and the root CLAUDE.md imports the shipped primer. Then
    /// stage it. The mocks declare `harness = []`, because the shipped set changes with every
    /// version of the skills; a test about the installed set declares the harness in its own copy.
    pub fn serve_claude(&self) {
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

/// Copy a mock project, with its manifest pinned to this binary's version.
///
/// A mock says `checker-version = "fixture"`, which the binary confirms only inside a library it
/// links, per `design@check@checked-sentinel-values`, and this crate is none, so a copy carries the
/// version.
pub fn copy_dir(from: &Path, to: &Path) {
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
pub fn pinned(text: &str) -> String {
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

/// A copy of `minimal` serving the `claude` harness, its shipped set installed and staged: a tree
/// the check passes.
pub fn harnessed(tag: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "minimal");
    sandbox.serve_claude();
    let (out, err, code) = sandbox.run(&["install-agent-skills"]);
    assert_eq!(code, 0, "{out}{err}");
    sandbox.stage();
    sandbox
}
