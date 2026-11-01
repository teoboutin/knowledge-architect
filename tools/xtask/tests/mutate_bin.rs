//! End-to-end runs of the built `xtask` binary's `mutate` subcommand over a scratch project: the
//! subcommand reaches the library's two modes. What the modes do is tested in the library's own
//! suite, `path@mutate@tests/modes.rs`.
#![cfg(unix)]

// This target links no library of its package, so it reads the variable itself, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A git repository holding the checker's manifest, so xtask accepts it as a project root, and
/// one file to mutate.
fn scratch_project(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("xtask-mutate-bin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch project can be created");
    std::fs::write(dir.join("knowledge-architect.toml"), "").expect("the root marker");
    std::fs::write(dir.join("f.txt"), "one\nSITE\nthree\n").expect("the target");
    let init = Command::new("git")
        .args(["init", "-q", "."])
        .current_dir(&dir)
        .status()
        .expect("git init runs");
    assert!(init.success());
    dir
}

fn xtask(project: &Path, args: &[&str]) -> (Output, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("mutate")
        .args(args)
        .current_dir(project)
        .output()
        .expect("the xtask binary runs");
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    (out, text)
}

// The claim: `cargo x mutate run` applies the mutation, runs the command, restores the file and
// exits 0; `cargo x mutate restore` reaches the recovery mode.
#[test]
fn the_subcommand_reaches_run_and_restore() {
    let project = scratch_project("modes");
    let (out, text) = xtask(
        &project,
        &[
            "run",
            "--file",
            "f.txt",
            "--replace",
            "SITE",
            "--with",
            "MUT",
            "--",
            "true",
        ],
    );
    assert_eq!(out.status.code(), Some(0), "{text}");
    assert!(text.contains("mutation applied to f.txt"), "{text}");
    assert!(text.contains("SURVIVED"), "{text}");
    assert_eq!(
        std::fs::read_to_string(project.join("f.txt")).expect("the target"),
        "one\nSITE\nthree\n"
    );

    let (out, text) = xtask(&project, &["restore"]);
    assert_eq!(out.status.code(), Some(0), "{text}");
    assert!(text.contains("no mutation record"), "{text}");
    std::fs::remove_dir_all(&project).ok();
}

// The claim: outside a project the subcommand never starts, and exits 2, even inside a git
// repository, where the library alone would run.
#[test]
fn outside_a_project_the_subcommand_exits_two() {
    let dir = scratch_project("none");
    std::fs::remove_file(dir.join("knowledge-architect.toml")).expect("the marker removed");
    let (out, text) = xtask(&dir, &["restore"]);
    assert_eq!(out.status.code(), Some(2), "{text}");
    assert!(text.contains("not inside the project"), "{text}");
    std::fs::remove_dir_all(&dir).ok();
}

// The claim: from a subdirectory, `--file` is relative to the project's root, which the
// subcommand finds above the working directory.
#[test]
fn from_a_subdirectory_the_file_is_relative_to_the_root() {
    let project = scratch_project("subdir");
    let sub = project.join("sub");
    std::fs::create_dir_all(&sub).expect("a subdirectory");
    let (out, text) = xtask(
        &sub,
        &[
            "run",
            "--file",
            "f.txt",
            "--replace",
            "SITE",
            "--with",
            "MUT",
            "--",
            "true",
        ],
    );
    assert_eq!(out.status.code(), Some(0), "{text}");
    assert!(text.contains("mutation applied to f.txt"), "{text}");
    std::fs::remove_dir_all(&project).ok();
}
