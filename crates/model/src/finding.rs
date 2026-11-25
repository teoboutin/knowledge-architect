//! The one shape every check reports.

use std::fmt;
use std::path::{Path, PathBuf};

/// One thing a check found wrong, in the one shape every check reports.
///
/// `what` states the defect and `action` states what to do about it. Both are required: a
/// report that says only what is wrong makes every reader derive the same fix, and deriving
/// it is where they disagree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Finding {
    /// Repository-relative, so a finding reads the same from any working directory. It is printed
    /// as it stands: [`Finding::at`] and [`Finding::in_file`] spell it as [`slashed`] does, and code
    /// that sets this field spells it itself.
    pub file: PathBuf,
    /// One-based, as an editor counts. `None` where the defect is the file itself — a
    /// missing tracker, a stale generated file — and there is no line to point at.
    pub line: Option<u32>,
    /// What is wrong: the defect, stated so a reader needs no other context.
    pub what: String,
    /// What to do about it: the repair.
    pub action: String,
}

impl Finding {
    /// A finding at a one-based line of a file.
    pub fn at(
        file: impl AsRef<Path>,
        line: u32,
        what: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            file: spelled(file.as_ref()),
            line: Some(line),
            what: what.into(),
            action: action.into(),
        }
    }

    /// A finding about a file as a whole, with no line to point at.
    pub fn in_file(
        file: impl AsRef<Path>,
        what: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            file: spelled(file.as_ref()),
            line: None,
            what: what.into(),
            action: action.into(),
        }
    }

    /// A finding about a file whose name is the subject, printed as git listed it.
    ///
    /// On Windows a name can hold `\` as a character of its own where git lists it from a commit or
    /// an index written on another system. Spelled as [`slashed`] does, it would be another file's.
    pub(crate) fn of_a_listed_name(
        file: &Path,
        what: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            file: file.to_path_buf(),
            line: None,
            what: what.into(),
            action: action.into(),
        }
    }

    /// `file:line`, or just the file where there is no line.
    ///
    /// This is the half a terminal turns into a link, so it is built once here rather than
    /// formatted at each call site where one check would eventually differ from the rest.
    ///
    /// **A finding's first line is one line, whatever the path holds.** A line break in the
    /// file's name is written as `\n` or `\r`, so a reader taking the output by line meets a
    /// finding per line rather than a truncated path and a stray tail. The walk refuses such
    /// a name, so the one finding that reports the refusal is the ordinary way a path
    /// reaches here; `Display` escapes the statement the same way.
    pub fn location(&self) -> String {
        let file = escaped(&self.file.to_string_lossy());
        match self.line {
            Some(n) => format!("{file}:{n}"),
            None => file,
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The statement is escaped like the location: a path a check formatted into it — an
        // anchor's path out of the manifest, a declared row — may hold a line break the walk
        // never saw, and the first line is the one a reader takes by line.
        write!(
            f,
            "{}  {}\n    → {}",
            self.location(),
            escaped(&self.what),
            self.action
        )
    }
}

/// A project-relative path as the tools print it and write it in a repair: with `/` between its
/// segments on every system, as git, a reference and a markdown link spell it, so a repair written
/// on one system resolves on every other.
///
/// It takes a project-relative path. An absolute path, or one the user gave, is printed as the
/// system writes it. A name that is not UTF-8 is decoded with replacement characters.
pub fn slashed(path: &Path) -> String {
    respelled(&path.to_string_lossy(), cfg!(windows))
}

/// The path as [`slashed`] spells it. Elsewhere than on Windows it is the path itself, so a name that
/// is not UTF-8 stays the bytes it has.
fn spelled(path: &Path) -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(slashed(path))
    } else {
        path.to_path_buf()
    }
}

/// The text of a path, with `/` for the separator `\` where `windows`. Elsewhere `\` is a character
/// of a name, and the text stays as it is.
fn respelled(text: &str, windows: bool) -> String {
    if windows {
        text.replace('\\', "/")
    } else {
        text.to_owned()
    }
}

/// The text with each line break written as `\n` or `\r`.
pub fn escaped(text: &str) -> String {
    text.replace('\n', "\\n").replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_located_finding_renders_file_line_then_the_action() {
        let f = Finding::at(
            "crates/an-engine/docs/design.md",
            604,
            "`an-engine#staging-not-mutation` is defined twice",
            "delete one definition; a reference resolves to exactly one anchor",
        );
        assert_eq!(f.location(), "crates/an-engine/docs/design.md:604");
        assert_eq!(
            f.to_string(),
            "crates/an-engine/docs/design.md:604  `an-engine#staging-not-mutation` is defined twice\n    \
             → delete one definition; a reference resolves to exactly one anchor"
        );
    }

    #[test]
    fn a_line_break_in_the_path_is_escaped_so_the_finding_stays_one_line() {
        let f = Finding::in_file("docs/a\nb\rc.md", "two anchors sit at `pa\npers`", "action");
        assert_eq!(f.location(), "docs/a\\nb\\rc.md");
        assert!(
            f.to_string()
                .starts_with("docs/a\\nb\\rc.md  two anchors sit at `pa\\npers`\n"),
            "{f}"
        );
        let shown = f.to_string();
        assert_eq!(
            shown.lines().count(),
            2,
            "the location and the action, and nothing the path adds: {shown:?}"
        );
    }

    /// The claim: a path is spelled with `/` under Windows' rule, and as it is under Unix's, where `\`
    /// is a character of a name. Mutation: the rule swapped.
    #[test]
    fn a_path_is_respelled_with_slashes_under_windows_rule_alone() {
        let mixed = "docs\\design/a\\b.md";
        assert_eq!(respelled(mixed, true), "docs/design/a/b.md");
        assert_eq!(respelled(mixed, false), mixed);
        assert_eq!(respelled("docs/a b.md", true), "docs/a b.md");
    }

    /// The claim: a printed path follows the rule of the system it runs on, and so does a finding's
    /// file, but that of a finding about a listed name, which is printed as git listed it. Each test
    /// machine shows its own half. Mutations: `cfg!(windows)` of `slashed` made `true`, caught where
    /// `\` is a character of a name.
    #[test]
    fn a_finding_s_file_is_spelled_by_the_system_s_rule_unless_its_name_is_the_subject() {
        let path = Path::new(r"docs\a.md");
        let printed = if cfg!(windows) {
            "docs/a.md"
        } else {
            r"docs\a.md"
        };
        assert_eq!(slashed(path), printed);
        assert_eq!(
            Finding::at(path, 1, "", "").location(),
            format!("{printed}:1")
        );
        assert_eq!(Finding::in_file(path, "", "").location(), printed);
        assert_eq!(
            Finding::of_a_listed_name(path, "", "").location(),
            r"docs\a.md"
        );
    }

    #[test]
    fn a_whole_file_finding_omits_the_colon_rather_than_inventing_a_line() {
        let f = Finding::in_file(
            "crates/a-library/docs/open-issues.md",
            "the component carries no tripwires.md",
            "create it, or stop declaring the directory a component",
        );
        assert_eq!(f.location(), "crates/a-library/docs/open-issues.md");
        assert!(!f.to_string().contains(":0"));
    }
}
