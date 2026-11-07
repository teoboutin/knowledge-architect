//! `klarch` over a project inside the directory of a library it links, per
//! `design@check@checked-sentinel-values`.
//!
//! This stays in `cli`: every other test of the binary runs in a copy, since `klarch` links nothing
//! of the Component that holds the mock projects, so this crate's own directory is the place where
//! the binary's list of its own source can be seen to reach the refusal.

use std::path::Path;
use std::process::Command;

/// A project written under this crate's directory for the length of one test, a git repository of its own.
struct Scratch(std::path::PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The claim: `"fixture"` is accepted for a project inside a library `klarch` links.
#[test]
fn a_project_inside_a_linked_library_passes_the_pin_with_fixture() {
    // Under a directory git ignores, so a killed run leaves nothing a check of this tree reads.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!("fixture-pin-{}", std::process::id()));
    let scratch = Scratch(dir);
    std::fs::create_dir_all(&scratch.0).expect("a scratch project");
    std::fs::write(
        scratch.0.join("knowledge-architect.toml"),
        "[project]\nchecker-version = \"fixture\"\nname = \"pin-mock\"\ncomponents = []\n\n[walk]\nskip-dirs = []\nskip-files = []\n\n[agents]\nharness = []\n",
    )
    .expect("a manifest");
    let init = Command::new("git")
        .args(["init", "-q"])
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .current_dir(&scratch.0)
        .status()
        .expect("git runs");
    assert!(init.success());
    let out = Command::new(env!("CARGO_BIN_EXE_klarch"))
        .arg("issues")
        .current_dir(&scratch.0)
        .output()
        .expect("the binary runs");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(!err.contains("checker-version"), "{err}");
    // `issues` ran: 0 with a row, 1 with none. A refusal is 2, and a panic is neither.
    assert!(matches!(out.status.code(), Some(0 | 1)), "{err}");
}
