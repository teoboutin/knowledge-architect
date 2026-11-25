//! This repository's `.gitattributes` gives every checkout LF and takes CRLF into no commit, whatever
//! the machine's git says. Every file of this repository is compared, embedded or parsed as the bytes
//! its commit holds, and Git for Windows checks out CRLF by default, per
//! `design@knowledge-architect@line-endings-of-this-repository`.

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
        .args(["-c", "core.excludesFile=/dev/null"])
        .args(args)
        // A suite run from a commit hook inherits them, and would otherwise write into the outer
        // repository, whose index `add -A` would replace.
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
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

/// The claim: under this repository's `.gitattributes` every text file is checked out with LF, and one
/// that holds CRLF is committed with LF, whether the machine's git converts or not, and a file that is
/// no text is left as it is. Mutations checked: the file emptied; `* -text` in its line's place;
/// `eol=lf` removed; `auto` removed, `* text eol=lf`; the line made one of `*.md` and one of `*.rs`;
/// each caught. That the repository variables are dropped was run: with `GIT_DIR` and
/// `GIT_INDEX_FILE` naming another repository, its index is left as it was.
#[test]
fn every_text_file_is_checked_out_with_lf_and_committed_with_lf() {
    for autocrlf in ["true", "false"] {
        let scratch = tempfile::TempDir::with_prefix("klarch-line-endings-").expect("a directory");
        let repo = scratch.path();
        git(repo, autocrlf, &["init", "-q"]);
        let attributes = include_str!("../../../.gitattributes");
        std::fs::write(repo.join(".gitattributes"), attributes).expect("the attributes");
        // One name git's own attributes know nothing of, and one with no extension at all.
        for file in ["lf.md", "lf.unnamed", "Lf"] {
            std::fs::write(repo.join(file), "one\ntwo\n").expect("a file with LF");
        }
        for file in ["crlf.rs", "crlf.unnamed", "Crlf"] {
            std::fs::write(repo.join(file), "one\r\ntwo\r\n").expect("a file with CRLF");
        }
        let binary = b"one\0\r\ntwo\n";
        std::fs::write(repo.join("binary"), binary).expect("a file that is no text");
        git(repo, autocrlf, &["add", "-A"]);
        assert_eq!(
            git(repo, autocrlf, &["cat-file", "blob", ":binary"]),
            binary
        );
        for file in [
            "lf.md",
            "lf.unnamed",
            "Lf",
            "crlf.rs",
            "crlf.unnamed",
            "Crlf",
        ] {
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
