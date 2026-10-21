//! The snippets of the setup skill that are not Rust, checked by what `klarch init --rust` writes
//! from them, per `spec@plans@init-and-new-component`: the workspace, the maintenance crate's
//! manifest and the cargo configuration must build and run the checker, and the CI workflow's
//! gates command must parse as the gates library declares its flags.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use clap::Parser;
use knowledge_architect::cli::{self, InitArgs};
use knowledge_architect_gates::GatesArgs;

// This target links no library of its package, so it reads the variable itself.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// A scratch directory outside every git worktree, removed when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("xtask-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        Scratch(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A cargo command in `dir`, offline, building into `dir/target`, with none of the variables
/// cargo set for this test's own build, which would describe this workspace rather than the
/// scaffold. The toolchain this test runs under is kept, through rustup's own variable.
fn cargo(dir: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or("cargo".into()));
    for (key, _) in std::env::vars_os() {
        let key = key.to_string_lossy();
        if key.starts_with("CARGO_") && key != "CARGO_HOME" {
            command.env_remove(&*key);
        }
    }
    command
        .args(args)
        .current_dir(dir)
        .env("CARGO_NET_OFFLINE", "true")
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .expect("cargo runs")
}

/// AC2: a project written by `init --rust` builds against this workspace's crates, and its
/// `cargo klarch check` passes. Guards #rust-templates-one-home of
/// `spec@plans@init-and-new-component`: a change to the gates library or the core that the
/// shipped snippets do not follow fails it.
#[test]
fn a_project_written_by_init_rust_builds_and_checks() {
    let scratch = Scratch::new("scaffold");
    let root = scratch.0.join("probe");
    #[derive(Parser)]
    struct Init {
        #[command(flatten)]
        args: InitArgs,
    }
    let init = Init::try_parse_from(["init", root.to_str().expect("UTF-8"), "--rust"])
        .expect("the arguments parse");
    let code = cli::init(init.args, &[], &[], &[], &mut []).expect("init runs");
    assert_eq!(code, ExitCode::SUCCESS);

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = workspace.canonicalize().expect("the workspace");
    let manifest = root.join("Cargo.toml");
    let mut text = std::fs::read_to_string(&manifest).expect("init wrote Cargo.toml");
    text.push_str(&format!(
        "\n[patch.crates-io]\nknowledge-architect = {{ path = {:?} }}\n\
         knowledge-architect-gates = {{ path = {:?} }}\n",
        workspace.join("crates/core"),
        workspace.join("crates/gates"),
    ));
    std::fs::write(&manifest, text).expect("the patch is written");

    let build = cargo(&root, &["build", "-q", "-p", "xtask"]);
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let check = cargo(&root, &["klarch", "check"]);
    let out = String::from_utf8_lossy(&check.stdout);
    assert!(
        check.status.success() && out.contains("PASSED: no findings"),
        "{out}{}",
        String::from_utf8_lossy(&check.stderr)
    );
}

/// The CI workflow's gates command parses as the gates library declares its flags: a flag the
/// library renamed or dropped fails here instead of in every project's CI.
#[test]
fn the_ci_workflow_runs_flags_the_gates_library_declares() {
    #[derive(Parser)]
    struct Gates {
        #[command(flatten)]
        args: GatesArgs,
    }
    let ci = include_str!("../../../crates/agent-skills/snippets/ci.yml");
    let line = ci
        .lines()
        .find(|l| l.contains(" x gates"))
        .expect("the workflow runs `x gates`");
    let flags = line
        .split_once(" x gates")
        .map(|(_, rest)| rest.split_whitespace())
        .expect("words follow `x gates`");
    let parsed = Gates::try_parse_from(std::iter::once("gates").chain(flags));
    assert!(parsed.is_ok(), "{line}: {:?}", parsed.err());
}
