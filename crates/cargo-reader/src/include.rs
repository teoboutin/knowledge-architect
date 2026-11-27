//! The match of a package's `include` patterns against a file of the package, as `cargo package`
//! reads them.
//!
//! The patterns are gitignore-style, as `path@cargo-reader@docs/grounding/cargo-manifest.md`
//! records them read in the Cargo book and measured with `cargo package --list`. A module of its
//! own beside the reader of the manifest's text: the patterns are a grammar of their own, which
//! Cargo can change without a change of the manifest's tables, and the manifest's reader holds the
//! patterns as written, per `design@cargo-reader@a-manifest-is-read-as-written`.

/// Whether a package whose `include` holds `patterns` packages the file `rel`, a path relative to
/// the package's root with `/` between its segments, such as `CHANGELOG.md`.
///
/// The last pattern that matches decides: a pattern opening with `!` excludes what it matches, and
/// any other includes it. A pattern matches a file where it matches the file's path or the path of
/// a directory that holds it. A pattern holding a `/` before its last character is anchored at the
/// package's root, a leading `/` included; any other matches a name at any depth. A pattern ending
/// with `/` matches a directory alone. Within a segment, `*` matches any run of characters but
/// `/`, `?` one character but `/`, and `[...]` one character of a set or a range, `[!...]` or
/// `[^...]` one outside it; a segment `**` matches any number of whole segments, none included.
/// The comparison is case-sensitive.
///
/// The files Cargo packages whatever `include` says, such as the `Cargo.toml` itself, are not
/// answered here: the answer is about the patterns alone.
pub fn include_matches(patterns: &[String], rel: &str) -> bool {
    let names: Vec<&str> = rel.split('/').filter(|s| !s.is_empty()).collect();
    if names.is_empty() {
        return false;
    }
    let mut included = false;
    for pattern in patterns {
        let (negated, body) = match pattern.strip_prefix('!') {
            Some(rest) => (true, rest),
            None => (false, pattern.as_str()),
        };
        if Pattern::parse(body).is_some_and(|p| p.matches(&names)) {
            included = !negated;
        }
    }
    included
}

/// One pattern, its `!` taken off.
struct Pattern {
    segments: Vec<String>,
    /// Whether it names a path from the package's root, rather than a name at any depth.
    anchored: bool,
    /// Whether it ends with `/`, so it matches a directory alone.
    directory: bool,
}

impl Pattern {
    /// The pattern, or `None` for one that matches nothing: an empty one, and `.`, since no
    /// segment of a path Cargo lists is `.`. A lone `/` names the package's root, which holds
    /// every file, as `cargo package --list` was measured to read it.
    fn parse(body: &str) -> Option<Self> {
        if body == "/" {
            return Some(Pattern {
                segments: vec!["**".to_string()],
                anchored: true,
                directory: false,
            });
        }
        let directory = body.ends_with('/');
        let trimmed = body.strip_suffix('/').unwrap_or(body);
        let anchored = trimmed.contains('/');
        let trimmed = trimmed.strip_prefix('/').unwrap_or(trimmed);
        let segments: Vec<String> = trimmed
            .split('/')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        if segments.is_empty() || segments.iter().any(|s| s == ".") {
            return None;
        }
        Some(Pattern {
            segments,
            anchored,
            directory,
        })
    }

    /// Whether the pattern matches the file `names` or one of the directories holding it.
    fn matches(&self, names: &[&str]) -> bool {
        (1..=names.len()).any(|k| {
            let is_directory = k < names.len();
            if self.directory && !is_directory {
                return false;
            }
            let candidate = &names[..k];
            if self.anchored {
                whole(&self.segments, candidate)
            } else {
                let name = candidate[candidate.len() - 1];
                self.segments.len() == 1 && segment(&self.segments[0], name)
            }
        })
    }
}

/// Whether `segments` match `names` whole, a segment `**` taking any number of names.
fn whole(segments: &[String], names: &[&str]) -> bool {
    match segments.split_first() {
        None => names.is_empty(),
        Some((first, rest)) if first == "**" => {
            (0..=names.len()).any(|skip| whole(rest, &names[skip..]))
        }
        Some((first, rest)) => match names.split_first() {
            Some((name, others)) => segment(first, name) && whole(rest, others),
            None => false,
        },
    }
}

/// Whether one segment's pattern matches one name.
fn segment(pattern: &str, name: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let n: Vec<char> = name.chars().collect();
    glob(&p, &n)
}

fn glob(p: &[char], n: &[char]) -> bool {
    match p.split_first() {
        None => n.is_empty(),
        Some(('*', rest)) => (0..=n.len()).any(|skip| glob(rest, &n[skip..])),
        Some(('?', rest)) => !n.is_empty() && glob(rest, &n[1..]),
        Some(('[', rest)) => match class(rest) {
            Some((set, after)) => n
                .first()
                .is_some_and(|c| set.contains(*c) && glob(after, &n[1..])),
            // An unclosed bracket is the character itself.
            None => n.first() == Some(&'[') && glob(rest, &n[1..]),
        },
        Some((c, rest)) => n.first() == Some(c) && glob(rest, &n[1..]),
    }
}

/// A bracket expression after its `[`: the set it names and the pattern after its `]`, or `None`
/// where no `]` closes it.
fn class(p: &[char]) -> Option<(Set, &[char])> {
    let (negated, mut i) = match p.first() {
        Some('!') | Some('^') => (true, 1),
        _ => (false, 0),
    };
    let mut ranges = Vec::new();
    let start = i;
    while i < p.len() {
        // A `]` right after the opening is a member, as in gitignore.
        if p[i] == ']' && i > start {
            return Some((Set { negated, ranges }, &p[i + 1..]));
        }
        if i + 2 < p.len() && p[i + 1] == '-' && p[i + 2] != ']' {
            ranges.push((p[i], p[i + 2]));
            i += 3;
        } else {
            ranges.push((p[i], p[i]));
            i += 1;
        }
    }
    None
}

struct Set {
    negated: bool,
    ranges: Vec<(char, char)>,
}

impl Set {
    fn contains(&self, c: char) -> bool {
        c != '/' && self.ranges.iter().any(|(lo, hi)| *lo <= c && c <= *hi) != self.negated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn includes(patterns: &[&str], rel: &str) -> bool {
        let patterns: Vec<String> = patterns.iter().map(|p| p.to_string()).collect();
        include_matches(&patterns, rel)
    }

    /// The claim, the patterns measured with `cargo package --list` over a package holding
    /// `CHANGELOG.md` at its root, as `path@cargo-reader@docs/grounding/cargo-manifest.md` records
    /// each verdict. Mutations checked, each caught: in `Pattern::parse`, `if body == "/" {` made
    /// `if false {`; and in `glob`, the `?` arm's `glob(rest, &n[1..])` made `glob(rest, n)`, so
    /// `?` takes no character.
    #[test]
    fn a_root_file_is_matched_as_cargo_package_lists_it() {
        for (patterns, want) in [
            (&["CHANGELOG.md"][..], true),
            (&["/CHANGELOG.md"], true),
            (&["*.md"], true),
            (&["/*.md"], true),
            (&["CHANGELOG*"], true),
            (&["**/CHANGELOG.md"], true),
            (&["/**"], true),
            (&["**"], true),
            (&["/*"], true),
            (&["*"], true),
            (&["/"], true),
            (&["/[C]HANGELOG.md"], true),
            (&["/?HANGELOG.md"], true),
            (&["CHANGE*.md"], true),
            (&["/**/CHANGELOG.md"], true),
            (&["changelog.md"], false),
            (&["/CHANGELOG.md/"], false),
            (&["/src"], false),
            (&[""], false),
            (&["."], false),
            (&["/src/**", "/README.md"], false),
        ] {
            assert_eq!(includes(patterns, "CHANGELOG.md"), want, "{patterns:?}");
        }
    }

    /// The claim, the last matching pattern decides, `!` excluding, as measured. Mutation
    /// checked: `included = !negated;` made `included = true;`; caught.
    #[test]
    fn the_last_matching_pattern_decides() {
        assert!(!includes(
            &["/CHANGELOG.md", "!CHANGELOG.md"],
            "CHANGELOG.md"
        ));
        assert!(includes(
            &["!CHANGELOG.md", "/CHANGELOG.md"],
            "CHANGELOG.md"
        ));
        assert!(!includes(&["/*", "!/CHANGELOG.md"], "CHANGELOG.md"));
        assert!(!includes(
            &["docs", "!docs/CHANGELOG.md"],
            "docs/CHANGELOG.md"
        ));
        assert!(!includes(&["docs/", "!CHANGELOG.md"], "docs/CHANGELOG.md"));
    }

    /// The claim, a nested file is matched through the directories holding it, and a pattern with
    /// an inner `/` is anchored, as measured. Mutations checked, each caught: in
    /// `Pattern::matches`, `(1..=names.len())` made `(names.len()..=names.len())`, so no directory
    /// holding the file is asked; and `let anchored = trimmed.contains('/');` made
    /// `let anchored = trimmed.starts_with('/');`.
    #[test]
    fn a_nested_file_is_matched_through_its_directories() {
        for (pattern, want) in [
            ("docs", true),
            ("docs/", true),
            ("/docs/*", true),
            ("CHANGELOG.md", true),
            ("docs/**", true),
            ("*/CHANGELOG.md", true),
            ("/*/CHANGELOG.md", true),
            ("**/docs", true),
            ("docs/CHANGELOG.md", true),
            ("do*", true),
            ("/docs/**/CHANGELOG.md", true),
            ("/docs/CHANGELOG.md/", false),
            ("a/**/b", false),
            ("ocs/CHANGELOG.md", false),
        ] {
            assert_eq!(includes(&[pattern], "docs/CHANGELOG.md"), want, "{pattern}");
        }
    }

    /// The claim, a bracket expression matches one character of its set or range, or outside it
    /// when negated. Mutation checked: in `Set::contains`, `!= self.negated` made `!= false`;
    /// caught.
    #[test]
    fn a_bracket_names_a_set_of_characters() {
        assert!(includes(&["/[A-D]HANGELOG.md"], "CHANGELOG.md"));
        assert!(!includes(&["/[!C]HANGELOG.md"], "CHANGELOG.md"));
        assert!(includes(&["/[^X]HANGELOG.md"], "CHANGELOG.md"));
        assert!(!includes(&["/[a-z]HANGELOG.md"], "CHANGELOG.md"));
    }
}
