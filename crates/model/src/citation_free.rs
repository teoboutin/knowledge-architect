//! The manifest's `[walk.citation-free]` table: the files a project declares free of citations,
//! each at one of two levels, and the globs that name them.
//!
//! **A glob is resolved here, and matched in phase 2.** `Manifest::parse` reads no file, so this
//! module refuses only a glob whose spelling can name nothing git lists; a glob that matches no
//! file, and a path two levels match, are `tree`'s findings, which hold the walked paths and the
//! installed copies.
//!
//! **The glob dialect is hand-written, with no dependency**: `*` matches any run of characters
//! within one path segment, never a `/`; a segment that is exactly `**` matches any number of whole
//! segments, none included; a glob ending with `/` names every file under the directory it
//! matches. Every other character, `?` and `[` included, is itself.

use std::path::Path;

use serde::Deserialize;

use crate::finding::Finding;
use crate::manifest::MANIFEST_NAME;

/// The table as written: two lists of globs, each empty by default.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Declared {
    #[serde(default)]
    all: Vec<String>,
    #[serde(default)]
    entries: Vec<String>,
}

/// What a citation-free file may hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// No reference at all: a span that parses as one is a finding whether or not it resolves,
    /// and no span of the file is resolved. Matched against the files the walk reads.
    All,
    /// No reference to an entry or to an anchor of the project: a span is resolved as anywhere
    /// else, and its kind decides whether the file may hold it. Matched against the files the
    /// walk reads and against the installed copies, which the walk leaves out.
    Entries,
}

impl Level {
    /// The level's key in the table, as a finding names it.
    pub fn key(self) -> &'static str {
        match self {
            Level::All => "all",
            Level::Entries => "entries",
        }
    }
}

/// One glob of the table, parsed. Built by the manifest's resolve alone, so every glob a caller
/// meets is one whose spelling the resolve accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glob {
    /// The glob as written, for a finding to name.
    text: String,
    segments: Vec<Segment>,
    /// Whether the glob ends with `/`, so it names every file under the directories it matches.
    directory: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    /// `**`: any number of whole segments, none included.
    Any,
    /// A segment's pattern, each `*` in it matching any run of characters, an empty one included.
    Pattern(String),
}

impl Glob {
    /// Parse one glob, or say why its spelling is refused.
    ///
    /// A glob is compared with git's listing, which spells a path relative to the root with no
    /// leading `/`, no `.` or `..` segment and no empty segment; a glob spelled otherwise could
    /// match nothing, or what it matched would depend on where the manifest sits.
    fn parse(text: &str) -> Result<Self, &'static str> {
        if text.is_empty() {
            return Err("is empty");
        }
        if text.starts_with('/') {
            return Err("is not project-relative");
        }
        let directory = text.ends_with('/');
        let body = text.strip_suffix('/').unwrap_or(text);
        let mut segments = Vec::new();
        for segment in body.split('/') {
            segments.push(match segment {
                "" => return Err("holds an empty segment"),
                "." | ".." => return Err("spells a `.` or a `..` segment"),
                "**" => Segment::Any,
                pattern => Segment::Pattern(pattern.to_string()),
            });
        }
        Ok(Self {
            text: text.to_string(),
            segments,
            directory,
        })
    }

    /// The glob as the manifest writes it.
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Whether the glob matches the project-relative file `rel`. A path that is not UTF-8
    /// matches no glob, since no glob of a TOML file can spell it.
    pub fn matches(&self, rel: &Path) -> bool {
        let Some(path) = rel.to_str() else {
            return false;
        };
        let names: Vec<&str> = path.split('/').collect();
        if self.directory {
            // The directory is any proper prefix of the path: the file itself is under it.
            (0..names.len()).any(|k| whole(&self.segments, &names[..k]))
        } else {
            whole(&self.segments, &names)
        }
    }
}

/// Whether `segments` match `names` whole, each `**` taking any number of names.
fn whole(segments: &[Segment], names: &[&str]) -> bool {
    match segments.split_first() {
        None => names.is_empty(),
        Some((Segment::Any, rest)) => (0..=names.len()).any(|i| whole(rest, &names[i..])),
        Some((Segment::Pattern(pattern), rest)) => names
            .split_first()
            .is_some_and(|(name, tail)| within(pattern, name) && whole(rest, tail)),
    }
}

/// Whether one segment's pattern matches one name, each `*` taking any run of characters.
fn within(pattern: &str, name: &str) -> bool {
    let Some((head, tail)) = pattern.split_once('*') else {
        return pattern == name;
    };
    let Some(rest) = name.strip_prefix(head) else {
        return false;
    };
    // The star takes some run of `rest`, possibly empty; the remainder of the pattern matches
    // what is left. Each byte offset that is a character boundary is tried.
    (0..=rest.len())
        .filter(|&i| rest.is_char_boundary(i))
        .any(|i| within(tail, &rest[i..]))
}

/// The table, resolved: the globs of each level whose spelling was accepted.
#[derive(Clone, Debug, Default)]
pub struct CitationFree {
    all: Vec<Glob>,
    entries: Vec<Glob>,
}

impl CitationFree {
    /// The level the file `rel` is declared at, or `None` where no glob matches it. A path both
    /// levels match, which phase 2 reports, is read at the level `all`, the stricter one.
    pub fn level(&self, rel: &Path) -> Option<Level> {
        if self.all.iter().any(|g| g.matches(rel)) {
            Some(Level::All)
        } else if self.entries.iter().any(|g| g.matches(rel)) {
            Some(Level::Entries)
        } else {
            None
        }
    }

    /// Each glob with its level, those of `all` first, in the order the manifest writes them.
    pub fn globs(&self) -> impl Iterator<Item = (Level, &Glob)> {
        self.all
            .iter()
            .map(|g| (Level::All, g))
            .chain(self.entries.iter().map(|g| (Level::Entries, g)))
    }
}

/// Parse each glob of the table, holding a complaint for each one whose spelling is refused; a
/// refused glob leaves its list, so it declares nothing.
pub(crate) fn resolve(declared: &Declared, complaints: &mut Vec<Finding>) -> CitationFree {
    let mut parsed = |level: Level, globs: &[String]| -> Vec<Glob> {
        globs
            .iter()
            .filter_map(|text| match Glob::parse(text) {
                Ok(glob) => Some(glob),
                Err(why) => {
                    complaints.push(Finding::in_file(
                        MANIFEST_NAME,
                        format!("`{text}` in [walk.citation-free] {} {why}", level.key()),
                        "spell the glob the way git lists a path, relative to the manifest's \
                         directory, with no leading `/`, no `.` or `..` segment and no empty \
                         segment; the glob declares nothing until then",
                    ));
                    None
                }
            })
            .collect()
    };
    let all = parsed(Level::All, &declared.all);
    let entries = parsed(Level::Entries, &declared.entries);
    CitationFree { all, entries }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glob(text: &str) -> Glob {
        Glob::parse(text).expect("an accepted glob")
    }

    fn matches(text: &str, rel: &str) -> bool {
        glob(text).matches(Path::new(rel))
    }

    /// The claim, rule 1 of the dialect: `*` matches any run of characters within one segment,
    /// an empty one included, and never a `/`. Mutations checked, each caught: a segment `*` read
    /// as `**` in `Glob::parse`, so the glob of the crates' pages matches a nested page; and the run a
    /// star takes in `within` made non-empty, `1..=rest.len()`, so `a/x*y.md` misses `a/xy.md`.
    #[test]
    fn a_star_matches_within_one_segment_and_never_a_slash() {
        assert!(matches("crates/*/CRATES-IO.md", "crates/cli/CRATES-IO.md"));
        assert!(matches("a/x*y.md", "a/xy.md"));
        assert!(matches("a/x*y.md", "a/x-and-y.md"));
        assert!(matches("a/*", "a/b"));
        assert!(!matches("crates/*/CRATES-IO.md", "crates/a/b/CRATES-IO.md"));
        assert!(!matches("a/*", "a/b/c"));
        assert!(!matches("a/x*y.md", "a/xy.txt"));
        assert!(!matches("a/*.md", "b/c.md"));
    }

    /// The claim, rule 2: a segment `**` matches any number of whole segments, none included.
    /// Mutation checked, caught: `0..=names.len()` in `whole` narrowed to `1..=names.len()`, so
    /// a glob `**` between two segments no longer matches a path that holds only those two.
    #[test]
    fn a_double_star_matches_any_number_of_whole_segments_none_included() {
        assert!(matches("docs/**/x.md", "docs/x.md"));
        assert!(matches("docs/**/x.md", "docs/a/x.md"));
        assert!(matches("docs/**/x.md", "docs/a/b/x.md"));
        assert!(matches("**/x.md", "x.md"));
        assert!(matches("**", "a/b/c.md"));
        assert!(!matches("docs/**/x.md", "other/x.md"));
        assert!(!matches("docs/**/x.md", "docs/a/y.md"));
    }

    /// The claim, rule 3: a glob ending with `/` names every file under each directory it
    /// matches, at any depth, and not the directory's own name taken as a file. Mutation
    /// checked, caught: the directory arm of `Glob::matches` widened to `0..=names.len()`, so the
    /// whole path counts as the directory and `docs/` matches the file `docs`.
    #[test]
    fn a_trailing_slash_names_every_file_under_the_directory() {
        let skills = ".claude/skills/knowledge-architect-*/";
        assert!(matches(
            skills,
            ".claude/skills/knowledge-architect-design/SKILL.md"
        ));
        assert!(matches(
            skills,
            ".claude/skills/knowledge-architect-design/sub/more.md"
        ));
        assert!(!matches(skills, ".claude/skills/klarch-release/SKILL.md"));
        assert!(!matches("docs/", "docs"));
        assert!(matches("docs/", "docs/a.md"));
        assert!(!matches("docs/", "docsx/a.md"));
    }

    /// The claim: every other character is itself, so a glob without a star names one path,
    /// and `?` and `[` are no wildcards. Mutation checked, caught: `within`'s exact comparison
    /// `pattern == name` made `true`.
    #[test]
    fn every_other_character_is_itself() {
        assert!(matches(
            ".claude/knowledge-architect/PRIMER.md",
            ".claude/knowledge-architect/PRIMER.md"
        ));
        assert!(!matches(
            ".claude/knowledge-architect/PRIMER.md",
            ".claude/knowledge-architect/OTHER.md"
        ));
        assert!(!matches("a?.md", "ab.md"));
        assert!(matches("a?.md", "a?.md"));
        assert!(!matches("[ab].md", "a.md"));
    }

    /// The claim: a glob whose spelling can name nothing git lists is a complaint of phase 1,
    /// and leaves its list. Mutation checked, caught: the arm `"." | ".."` of `Glob::parse`
    /// narrowed to `"."`, which lets `../x` through.
    #[test]
    fn a_glob_spelled_unlike_git_s_listing_is_a_complaint_and_declares_nothing() {
        let declared = Declared {
            all: vec![
                "/abs".into(),
                "../x".into(),
                "a//b".into(),
                String::new(),
                "./a".into(),
                "kept/*.md".into(),
            ],
            entries: Vec::new(),
        };
        let mut complaints = Vec::new();
        let resolved = resolve(&declared, &mut complaints);
        assert_eq!(complaints.len(), 5, "{complaints:#?}");
        assert!(complaints
            .iter()
            .all(|c| c.what.contains("in [walk.citation-free] all")));
        let kept: Vec<&str> = resolved.globs().map(|(_, g)| g.as_str()).collect();
        assert_eq!(kept, vec!["kept/*.md"]);
    }

    /// The claim: a file is at the level a glob of its list matches, `all` where both match,
    /// and at none where neither does. Mutation checked, caught: `all` read in `level` only where
    /// no glob of `entries` also matches, so `both.md` reads at `entries`.
    #[test]
    fn a_file_is_at_the_level_its_glob_names() {
        let declared = Declared {
            all: vec!["crates/*/CRATES-IO.md".into(), "both.md".into()],
            entries: vec![
                ".claude/skills/knowledge-architect-*/".into(),
                "both.md".into(),
            ],
        };
        let free = resolve(&declared, &mut Vec::new());
        let level = |rel: &str| free.level(Path::new(rel));
        assert_eq!(level("crates/cli/CRATES-IO.md"), Some(Level::All));
        assert_eq!(
            level(".claude/skills/knowledge-architect-design/SKILL.md"),
            Some(Level::Entries)
        );
        assert_eq!(level("both.md"), Some(Level::All));
        assert_eq!(level("README.md"), None);
    }
}
