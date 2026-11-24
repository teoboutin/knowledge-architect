//! `klarch` over a project inside the directory of a library it links, per
//! `design@check@checked-sentinel-values`.
//!
//! This stays in `cli`: every other test of the binary runs in a copy, since `klarch` links nothing
//! of the Component that holds the mock projects, so this crate's own directory is the place where
//! the binary's list of its own source can be seen to reach the refusal.

use std::path::Path;
use std::process::Command;

/// The claim: `"fixture"` is accepted for a project inside a library `klarch` links.
#[test]
fn a_project_inside_a_linked_library_passes_the_pin_with_fixture() {
    // Under this crate's own directory, in one git ignores, so a killed run leaves nothing a check
    // of this tree reads. The project is a git repository of its own.
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("target");
    std::fs::create_dir_all(&target).expect("the ignored directory");
    let scratch = tempfile::TempDir::with_prefix_in("klarch-fixture-pin-", &target)
        .expect("a scratch project");
    std::fs::write(
        scratch.path().join("knowledge-architect.toml"),
        "[project]\nchecker-version = \"fixture\"\nname = \"pin-mock\"\ncomponents = []\n\n[walk]\nskip-dirs = []\nskip-files = []\n\n[agents]\nharness = []\n",
    )
    .expect("a manifest");
    let init = Command::new("git")
        .args(["init", "-q"])
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .current_dir(scratch.path())
        .status()
        .expect("git runs");
    assert!(init.success());
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .arg("issues")
        .current_dir(scratch.path())
        .output()
        .expect("the binary runs");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(!err.contains("checker-version"), "{err}");
    // `issues` ran: 0 with a row, 1 with none. A refusal is 2, and a panic is neither.
    assert!(matches!(out.status.code(), Some(0 | 1)), "{err}");
}
