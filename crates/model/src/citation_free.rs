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
    /// No reference to an entry or to an anchor of the project: a span's kind and the anchor it
    /// names decide whether the file may hold it. A span the level refuses is its finding and is
    /// not resolved; one it admits is resolved as anywhere else in a file the walk reads, and not
    /// at all in an installed copy. Matched against the files the walk reads and against the
    /// installed markdown copies, which the walk leaves out.
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
    ///
    /// The time is polynomial in the lengths of the glob and the path, whatever the count of
    /// stars: a manifest the resolve accepts never stalls the check.
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
    wildcard(
        segments,
        names,
        |s| matches!(s, Segment::Any),
        |s, name| matches!(s, Segment::Pattern(p) if within(p, name)),
    )
}

/// Whether one segment's pattern matches one name, each `*` taking any run of characters.
///
/// Compared byte by byte: a literal of the pattern is whole UTF-8, and its first byte is never a
/// continuation byte, so a star's run ends on a character boundary wherever the rest matches.
fn within(pattern: &str, name: &str) -> bool {
    wildcard(
        pattern.as_bytes(),
        name.as_bytes(),
        |&b| b == b'*',
        |&p, &n| p == n,
    )
}

/// Whether `pattern` matches `names` whole, where a star takes any run of names, an empty one
/// included, and every other token matches one name, as `one` says.
///
/// **One pass with one point of return, the last star met**, so the cost is at most the product of
/// the two lengths. A recursion that tries every run for each star costs about the binomial of the
/// name's length over the count of stars: ten stars over a name of forty characters took over a
/// minute. Returning to the last star alone loses no match: the tokens after it match a run of
/// fixed length each, so the leftmost place where they match leaves the most names to what
/// follows, and an earlier star's run need never grow.
fn wildcard<P, N>(
    pattern: &[P],
    names: &[N],
    star: impl Fn(&P) -> bool,
    one: impl Fn(&P, &N) -> bool,
) -> bool {
    let (mut p, mut n) = (0, 0);
    // The token after the last star met, and the first name its run does not take.
    let mut back: Option<(usize, usize)> = None;
    while n < names.len() {
        match pattern.get(p) {
            Some(token) if star(token) => {
                back = Some((p + 1, n));
                p += 1;
            }
            Some(token) if one(token, &names[n]) => {
                p += 1;
                n += 1;
            }
            _ => {
                let Some((after, end)) = back else {
                    return false;
                };
                // The last star takes one more name, and what follows it is tried again.
                back = Some((after, end + 1));
                p = after;
                n = end + 1;
            }
        }
    }
    pattern[p..].iter().all(star)
}

/// The table, resolved: the globs of each level whose spelling was accepted.
#[derive(Clone, Debug, Default)]
pub struct CitationFree {
    all: Vec<Glob>,
    entries: Vec<Glob>,
}

impl CitationFree {
    /// The level the file `rel` is declared at, or `None` where no glob matches it. A path both
    /// levels match is answered `all`, the stricter one: phase 2 reports such a path where the
    /// walk reads it. An installed copy is read at `entries` wherever a glob of `entries` matches
    /// it, whatever this answers, since a glob of `all` is matched against the walked files
    /// alone.
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
    /// an empty one included, and never a `/`; each star of a segment does, the second and the
    /// third included. Mutations checked, each caught: a segment `*` read as `**` in
    /// `Glob::parse`, so the glob of the crates' pages matches a nested page; the star of `within`
    /// read as a literal, `|_| false`, so `a/x*y.md` misses `a/x-and-y.md`; and in `wildcard`, the
    /// point of return kept at the first star met, `back = back.or(Some((p + 1, n)))`, so
    /// `a/*x*y*.md` misses `a/1x2y3.md`.
    #[test]
    fn a_star_matches_within_one_segment_and_never_a_slash() {
        assert!(matches("crates/*/CRATES-IO.md", "crates/cli/CRATES-IO.md"));
        assert!(matches("a/x*y.md", "a/xy.md"));
        assert!(matches("a/x*y.md", "a/x-and-y.md"));
        assert!(matches("a/*", "a/b"));
        assert!(matches("a/*x*y*.md", "a/1x2y3.md"));
        assert!(matches("a/*x*y*.md", "a/xy.md"));
        assert!(matches("a/*ab.md", "a/aab.md"));
        assert!(!matches("a/*x*y*.md", "a/yx.md"));
        assert!(!matches("a/*x*y*.md", "a/1x2y3.txt"));
        assert!(!matches("crates/*/CRATES-IO.md", "crates/a/b/CRATES-IO.md"));
        assert!(!matches("a/*", "a/b/c"));
        assert!(!matches("a/x*y.md", "a/xy.txt"));
        assert!(!matches("a/*.md", "b/c.md"));
    }

    /// The claim, rule 2: a segment `**` matches any number of whole segments, none included.
    /// Mutation checked, caught: in `wildcard`, the run a star takes made one name at least, its
    /// arm `back = Some((p + 1, n)); p += 1;` made `back = Some((p + 1, n + 1)); p += 1; n += 1;`,
    /// so a glob `**` between two segments no longer matches a path that holds only those two.
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
    /// matches, at any depth, and not the directory's own name taken as a file; with rule 2, `**/`
    /// names every file, a file at the root included. Mutations checked, each caught: the
    /// directory arm of `Glob::matches` widened to `0..=names.len()`, so the whole path counts as
    /// the directory and `docs/` matches the file `docs`; and narrowed to `1..names.len()`, so
    /// `**/` misses `README.md`.
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
        assert!(matches("**/", "README.md"));
        assert!(matches("**/", "a/b/c.md"));
    }

    /// The claim: a star's run may hold a character of several bytes, and the match neither
    /// panics nor misses there. A regression guard: the match compares bytes, so no slice of a
    /// name is cut, and no mutation of it is specific to such a name.
    #[test]
    fn a_star_takes_a_character_of_several_bytes() {
        assert!(matches("docs/*t*.md", "docs/été.md"));
        assert!(matches("docs/é*.md", "docs/été.md"));
        assert!(!matches("docs/e*.md", "docs/été.md"));
    }

    /// The claim: the time of a match is polynomial in the lengths of the glob and the path, so a
    /// glob of many stars the resolve accepts never stalls the check. The input is the one a review
    /// timed: ten stars over a name of forty characters took over a minute with a recursion over
    /// every run; here sixteen stars over two hundred, which no such recursion finishes in any
    /// time a run waits. Mutation checked, caught by the deadline: `within` made the recursion
    /// that tries every byte offset for each star, as in the review's report.
    #[test]
    fn a_glob_of_many_stars_matches_in_polynomial_time() {
        let name = format!("docs/{}.md", "a".repeat(200));
        let glob = glob(&format!("docs/{}b.md", "*a".repeat(16)));
        let (sent, got) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = sent.send(glob.matches(Path::new(&name)));
        });
        let matched = got
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect("the match ends within ten seconds");
        assert!(!matched);
    }

    /// The claim: every other character is itself, so a glob without a star names one path,
    /// and `?` and `[` are no wildcards. Mutation checked, caught: `within`'s comparison of one
    /// byte, `|&p, &n| p == n`, made `|_, _| true`.
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
    /// and leaves its list, and a leading `/` is named as what it is. Mutations checked, each
    /// caught: the arm `"." | ".."` of `Glob::parse` narrowed to `"."`, which lets `../x` through;
    /// and the test of a leading `/` removed, so `/abs` is named as holding an empty segment.
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
        assert_eq!(
            complaints[0].what,
            "`/abs` in [walk.citation-free] all is not project-relative"
        );
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

    /// A manifest declaring `[walk.citation-free]` with `table`, parsed.
    fn parsed(table: &str) -> Result<crate::manifest::Manifest, String> {
        crate::manifest::Manifest::parse(
            Path::new("/nowhere"),
            &format!(
                "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\n\
                 components = []\n\n[walk]\nskip-dirs = []\nskip-files = []\n\n\
                 [walk.citation-free]\n{table}\n"
            ),
        )
    }

    /// The claim: the manifest's resolve holds a refused glob as a complaint of phase 1, and the
    /// glob declares nothing; a glob it accepts is the resolved table's. Mutation checked, caught:
    /// in `Manifest::parse`, the complaints of `resolve` sent to a fresh `Vec::new()`.
    #[test]
    fn the_manifest_holds_a_refused_glob_as_a_complaint() {
        let manifest = parsed("all = [\"../x\", \"kept/*.md\"]").expect("a manifest");
        let complaints: Vec<&str> = manifest
            .complaints()
            .iter()
            .map(|c| c.what.as_str())
            .collect();
        assert_eq!(
            complaints,
            vec!["`../x` in [walk.citation-free] all spells a `.` or a `..` segment"]
        );
        let kept: Vec<&str> = manifest
            .citation_free()
            .globs()
            .map(|(_, g)| g.as_str())
            .collect();
        assert_eq!(kept, vec!["kept/*.md"]);
    }

    /// The claim: a key of the table other than `all` and `entries` is refused, so a misspelt
    /// level is never a list that declares nothing in silence. Mutation checked, caught:
    /// `#[serde(deny_unknown_fields)]` removed from `Declared`.
    #[test]
    fn an_unknown_key_of_the_table_is_refused() {
        let refused = parsed("entires = [\"README.md\"]").expect_err("a refused manifest");
        assert!(refused.contains("entires"), "{refused}");
    }
}
