//! `cargo x transcript-kinds` run as a binary over the transcripts' case `gaps` of the testing
//! crate, read in place under `--all-projects`, so no `cwd` of the case needs rendering.

use std::path::Path;
use std::process::Command;

// This target links no library of its package, so it reads the variable itself, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// The claim: the command prints the reader's header, then one row per place and kind with its
/// count and its keys, the subagent files included, and exits 0.
#[test]
fn the_inventory_counts_each_kind_of_every_file_the_reader_reads() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let config = root.join("crates/testing/tests/transcripts/gaps");
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(["transcript-kinds", "--all-projects"])
        .current_dir(&root)
        .env("CLAUDE_CONFIG_DIR", &config)
        .output()
        .expect("the binary runs");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(0), "{stdout}");
    assert!(stdout.starts_with("# files read: 4;"), "{stdout}");
    assert!(stdout.contains("\ntop user owner: 4 lines;"), "{stdout}");
    assert!(
        stdout.contains("\ntop user injected task-notification: 1 lines;"),
        "{stdout}"
    );
    assert!(
        stdout.contains("\nsubagent user dispatch: 1 lines;"),
        "{stdout}"
    );
    assert!(
        stdout.contains("\nsubagent user tool: 1 lines;"),
        "{stdout}"
    );
    assert!(
        stdout.contains("\ntop session-owner: 1 lines; versions none; keys sessionId 1, type 1\n"),
        "{stdout}"
    );
}
