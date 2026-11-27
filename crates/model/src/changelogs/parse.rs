//! The parse of a crate's changelog: its preamble, its sections, its entries and the lines it
//! could not read. It raises no finding.
//!
//! The grammar, compiled in:
//!
//! - the preamble: every line above the first version heading;
//! - a version heading: a level-two heading, as markdown reads one, per
//!   `design@model@headings-open-with-hash-marks`, whose text is `Next release` or a version,
//!   three numbers joined by `.`; a heading-shaped line inside a fenced block is text;
//! - an entry: a line opening with `- `, `- <category>, <class>: <text>` in a classed version and
//!   `- <text>` in one the manifest's `unclassed` lists, its text continuing on the lines indented
//!   under it, blank lines between them included.
//!
//! Every other line below the first version heading is an unread line: a heading of another
//! level or text, a line of text that is not indented under an entry. Which of the two forms an
//! entry takes is the manifest's to say, so the parse records the tag wherever the first line
//! reads as one, and the reader decides.

use std::sync::LazyLock;

use regex::Regex;

use super::{Version, NEXT_RELEASE};
use crate::scan::Observation;

/// A crate's changelog, parsed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Changelog {
    /// The lines above the first version heading, joined by `\n`, the blank lines that end them
    /// taken off; the whole text where no version heading is.
    pub preamble: String,
    /// The sections, in the order of the file.
    pub sections: Vec<Section>,
    /// The lines below the first version heading that are no heading of a section, no entry and
    /// no continuation of one.
    pub unread: Vec<Unread>,
}

/// One section: a version heading and the entries under it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    /// What the heading names.
    pub heading: Heading,
    /// The heading's line, one-based.
    pub line: u32,
    /// Its entries, in the order of the file.
    pub entries: Vec<Entry>,
}

/// What a version heading names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heading {
    /// `## Next release`, the entries not yet released.
    NextRelease,
    /// `## <version>`, a released version.
    Version(Version),
}

/// One entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Its first line, one-based.
    pub line: u32,
    /// Everything after its `- `: its first line, then each line of its continuation as written,
    /// joined by `\n`, the blank lines that end it taken off.
    pub raw: String,
    /// Its category and its class, where its first line opens `<category>, <class>: `.
    pub tag: Option<Tag>,
}

/// An entry's tag, read where the first line opens `<category>, <word>: `.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag {
    /// The category, as written.
    pub category: String,
    /// The word in the class's place, as written; a class only where `Class::parse` reads it.
    pub class: String,
    /// The entry's text after its tag, continuation included.
    pub text: String,
}

/// A line the parse could not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unread {
    /// One-based.
    pub line: u32,
    /// The line as written.
    pub text: String,
}

static TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([^,:\n]+), ([A-Za-z]+): ").expect("the tag pattern compiles"));

/// Parse a crate's changelog.
pub fn parse(text: &str) -> Changelog {
    // The headings as the checker's markdown reading finds them, so a heading-shaped line in a
    // fenced block, or indented four spaces, is none.
    let parsed = crate::source::md::parse(text);
    let headings: Vec<(u32, u8, String)> = crate::scan::scan(&parsed)
        .into_iter()
        .filter_map(|l| match l.what {
            Observation::Heading { level, text } => Some((l.line, level, text)),
            _ => None,
        })
        .collect();
    let heading_at = |n: u32| headings.iter().find(|(line, _, _)| *line == n);

    let mut out = Changelog::default();
    let mut preamble: Vec<&str> = Vec::new();
    let mut in_preamble = true;
    // The entry open, and the blank lines met since its last line, which join it only where an
    // indented line follows them.
    let mut open = false;
    let mut blanks: Vec<&str> = Vec::new();
    for (i, line) in text.split('\n').enumerate() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        let n = i as u32 + 1;
        if let Some((_, level, heading)) = heading_at(n) {
            let section = (*level == 2)
                .then(|| section_heading(heading.trim()))
                .flatten();
            if let Some(heading) = section {
                in_preamble = false;
                open = false;
                blanks.clear();
                out.sections.push(Section {
                    heading,
                    line: n,
                    entries: Vec::new(),
                });
                continue;
            }
        }
        if in_preamble {
            preamble.push(line);
            continue;
        }
        let is_heading = heading_at(n).is_some();
        if line.trim().is_empty() {
            if open {
                blanks.push(line);
            }
            continue;
        }
        // Below the preamble, a section is always open.
        let section = out.sections.last_mut();
        if let (false, Some(rest), Some(section)) = (is_heading, line.strip_prefix("- "), section) {
            section.entries.push(Entry {
                line: n,
                raw: rest.to_string(),
                tag: None,
            });
            open = true;
            blanks.clear();
            continue;
        }
        let entry = out.sections.last_mut().and_then(|s| s.entries.last_mut());
        if let (false, true, true, Some(entry)) =
            (is_heading, open, line.starts_with([' ', '\t']), entry)
        {
            for blank in blanks.drain(..) {
                entry.raw.push('\n');
                entry.raw.push_str(blank);
            }
            entry.raw.push('\n');
            entry.raw.push_str(line);
            continue;
        }
        open = false;
        blanks.clear();
        out.unread.push(Unread {
            line: n,
            text: line.to_string(),
        });
    }
    while preamble.last().is_some_and(|l| l.trim().is_empty()) {
        preamble.pop();
    }
    out.preamble = preamble.join("\n");
    for entry in out.sections.iter_mut().flat_map(|s| s.entries.iter_mut()) {
        entry.tag = TAG.captures(&entry.raw).and_then(|c| {
            Some(Tag {
                category: c.get(1)?.as_str().to_string(),
                class: c.get(2)?.as_str().to_string(),
                text: entry.raw[c.get(0)?.end()..].to_string(),
            })
        });
    }
    out
}

/// What a level-two heading's text names as a section, or `None` where it names none.
fn section_heading(text: &str) -> Option<Heading> {
    if text == NEXT_RELEASE {
        return Some(Heading::NextRelease);
    }
    Version::parse(text).map(Heading::Version)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOG: &str = "# Changelog of a\n\nIts preamble.\n\n## Next release\n\n\
        - Migration, minor: a change\n  that goes on\n\n  past a blank line\n- New features, patch: one more\n\n\
        ## 0.2.0\n\n- an entry with no tag\n";

    /// The claim, the parse reads the preamble, each section and each entry, its continuation
    /// lines kept as written, blank lines between them included. Mutations checked, each caught:
    /// in `parse`, `for blank in blanks.drain(..)` made `for blank in blanks.drain(..).take(0)`;
    /// and the preamble's trailing blank lines kept, `while preamble.last()` made
    /// `while false && preamble.last()`.
    #[test]
    fn the_preamble_the_sections_and_the_entries_are_read() {
        let log = parse(LOG);
        assert_eq!(log.preamble, "# Changelog of a\n\nIts preamble.");
        assert_eq!(log.sections.len(), 2);
        assert_eq!(log.sections[0].heading, Heading::NextRelease);
        assert_eq!(log.sections[0].line, 5);
        let first = &log.sections[0].entries[0];
        assert_eq!(first.line, 7);
        let tag = first.tag.as_ref().expect("a tag");
        assert_eq!(tag.category, "Migration");
        assert_eq!(tag.class, "minor");
        assert_eq!(tag.text, "a change\n  that goes on\n\n  past a blank line");
        assert_eq!(
            log.sections[0].entries[1].raw,
            "New features, patch: one more"
        );
        assert_eq!(
            log.sections[1].heading,
            Heading::Version(Version::parse("0.2.0").unwrap())
        );
        assert_eq!(log.sections[1].entries[0].tag, None);
        assert!(log.unread.is_empty(), "{:?}", log.unread);
    }

    /// The claim, a line of no grammar below the first version heading is recorded unread, and no
    /// finding is raised: a subsection heading, a heading that names no version, a paragraph line.
    /// A heading-shaped line in a fenced block is text, and a heading of another level is unread
    /// whatever it names. Mutation checked: in `parse`, `let section = (*level == 2)` made
    /// `let section = (*level >= 2)`, so `### 0.1.9` opens a section; caught.
    #[test]
    fn a_line_of_no_grammar_is_unread() {
        let log = parse(
            "# P\n\n## 0.2.0\n\n### Migration\n\n- Migration, minor: a\n\n  ```text\n  ## 9.9.9\n  ```\n\
             A paragraph line\n## Not a version\n### 0.1.9\n",
        );
        assert_eq!(log.sections.len(), 1, "{:?}", log.sections);
        let unread: Vec<u32> = log.unread.iter().map(|u| u.line).collect();
        assert_eq!(unread, [5, 12, 13, 14]);
        assert!(log.sections[0].entries[0].raw.contains("## 9.9.9"));
    }
}
