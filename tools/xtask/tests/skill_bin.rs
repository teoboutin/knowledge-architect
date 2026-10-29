//! Runs of the built `xtask` binary's `skill` subcommand over this checkout: where its output goes
//! and what it exits with.

use std::path::PathBuf;
use std::process::Command;

// Read so that cargo rebuilds this test when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// This checkout's root: this crate sits at tools/xtask.
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root exists")
}

fn skill(name: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["skill", name])
        .current_dir(workspace_root())
        .output()
        .expect("the built xtask binary runs")
}

// The claim: a view goes to stdout whole, with nothing on stderr, and the run exits 0.
#[test]
fn a_view_goes_to_stdout_and_exits_zero() {
    let out = skill("primer");
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
    let stdout = String::from_utf8(out.stdout).expect("UTF-8");
    assert!(stdout.starts_with("# "), "{stdout}");
    assert!(stdout.contains("%% >>> fragments/"), "{stdout}");
}

// The claim: a refusal goes to stderr, with nothing on stdout, and the run exits 1.
#[test]
fn a_refusal_goes_to_stderr_and_exits_one() {
    let out = skill("no-such-skill");
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert!(out.stdout.is_empty(), "{out:?}");
    let stderr = String::from_utf8(out.stderr).expect("UTF-8");
    assert!(stderr.starts_with("xtask: no skill"), "{stderr}");
}
