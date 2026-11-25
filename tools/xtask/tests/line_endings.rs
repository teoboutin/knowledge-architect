//! This repository's `.gitattributes` gives every checkout LF and takes CRLF into no commit, whatever
//! the machine's git says. Every file of this repository is compared, embedded or parsed as the bytes
//! its commit holds, and Git for Windows checks out CRLF by default.

use std::path::Path;
use std::process::Command;

// This target links no library of its package, so it reads the variable itself, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// What one git command over `repo` printed, under this value of `core.autocrlf`.
fn git(repo: &Path, autocrlf: &str, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .args(["-c", &format!("core.autocrlf={autocrlf}")])
        // Both warn or refuse on a conversion by the machine's own configuration.
        .args(["-c", "core.safecrlf=false", "-c", "core.eol=native"])
        .args(args)
        .current_dir(repo)
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    out.stdout
}

/// The claim: under this repository's `.gitattributes` a file is checked out with LF, and a file that
/// holds CRLF is committed with LF, whether the machine's git converts or not. Mutations checked: the
/// file emptied, `* -text` in its line's place, and `eol=lf` removed; each caught.
#[test]
fn a_file_is_checked_out_with_lf_and_committed_with_lf() {
    for autocrlf in ["true", "false"] {
        let scratch = tempfile::TempDir::with_prefix("klarch-line-endings-").expect("a directory");
        let repo = scratch.path();
        git(repo, autocrlf, &["init", "-q"]);
        let attributes = include_str!("../../../.gitattributes");
        std::fs::write(repo.join(".gitattributes"), attributes).expect("the attributes");
        std::fs::write(repo.join("lf.md"), "one\ntwo\n").expect("a file with LF");
        std::fs::write(repo.join("crlf.rs"), "one\r\ntwo\r\n").expect("a file with CRLF");
        git(repo, autocrlf, &["add", "-A"]);
        for file in ["lf.md", "crlf.rs"] {
            let staged = git(repo, autocrlf, &["cat-file", "blob", &format!(":{file}")]);
            assert_eq!(staged, b"one\ntwo\n", "{file} staged, autocrlf={autocrlf}");
            std::fs::remove_file(repo.join(file)).expect("the file");
            git(repo, autocrlf, &["checkout-index", "--", file]);
            let checked_out = std::fs::read(repo.join(file)).expect("the file checked out");
            assert_eq!(
                checked_out, b"one\ntwo\n",
                "{file} checked out, autocrlf={autocrlf}"
            );
        }
    }
}
