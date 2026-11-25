//! Whether a file of a project is current, where this tool would write one: an installed file and a
//! generated one alike, per `design@check@generated-text-compared-without-line-endings`.
//!
//! A module of its own: the installed text, the generated files and the commands that write either ask
//! the one question, and none of them owns it.

use std::path::Path;

/// The text with every CRLF line ending made LF, so a checkout that converts line endings holds
/// the same text as one that does not.
pub(crate) fn lf(text: &str) -> String {
    text.replace("\r\n", "\n")
}

/// Whether a file that holds `on_disk` is current, where this version would write `expected`: the
/// two differ by their line endings at most, which a checkout under `core.autocrlf` converts.
///
/// **It is the one comparison of currency**, of an installed file and of a generated one, so what
/// `check` calls current is what `check --fix` neither writes nor refuses over, and what
/// `index --staged` does not name. What asks to write, `index`, the entries `index --staged` stages
/// and the install, compares bytes.
pub(crate) fn current(on_disk: &str, expected: &str) -> bool {
    lf(on_disk) == lf(expected)
}

/// Whether the working-tree file at `path` is current for `text`: one that is absent, or that cannot
/// be read as text, is not.
pub(crate) fn is_current(path: &Path, text: &str) -> bool {
    std::fs::read_to_string(path).is_ok_and(|on_disk| current(&on_disk, text))
}

#[cfg(test)]
mod tests {
    /// The claim: a working copy is current where it holds the text, line endings apart, and not where
    /// it is absent or is no text. Mutation checked: the `.is_ok_and(` of `is_current` made `.map_or(path.exists(), `, under
    /// which a file that is no text is current; caught.
    #[test]
    fn a_working_copy_that_is_absent_or_no_text_is_not_current() {
        let scratch = tempfile::TempDir::with_prefix("klarch-current-").expect("a directory");
        for (name, bytes, current) in [
            ("same.md", Some(&b"a\nb\n"[..]), true),
            ("crlf.md", Some(&b"a\r\nb\r\n"[..]), true),
            ("other.md", Some(&b"a\nc\n"[..]), false),
            ("binary.md", Some(&b"\xff\xfe"[..]), false),
            ("absent.md", None, false),
        ] {
            let path = scratch.path().join(name);
            if let Some(bytes) = bytes {
                std::fs::write(&path, bytes).expect("a file");
            }
            assert_eq!(super::is_current(&path, "a\nb\n"), current, "{name}");
        }
    }
}
