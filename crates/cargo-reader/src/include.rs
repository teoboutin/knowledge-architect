//! The match of a package's `include` and `exclude` patterns against a file of the package, as
//! `cargo package` reads them.
//!
//! The patterns are gitignore-style, as `path@cargo-reader@docs/grounding/cargo-manifest.md`
//! records them read in the Cargo book and measured with `cargo package --list`. A module of its
//! own beside the reader of the manifest's text: the patterns are a grammar of their own, which
//! Cargo can change without a change of the manifest's tables, and the manifest's reader holds the
//! patterns as written, per `design@cargo-reader@a-manifest-is-read-as-written`.

use std::fmt;

/// Whether a package whose `include` holds `patterns` packages the file `rel`, a path relative to
/// the package's root with `/` between its segments, such as `CHANGELOG.md`.
///
/// An empty list is no `include`, as Cargo reads it: the answer is then `true`, and whether the
/// file is packaged is what `exclude` says, which [`exclude_matches`] answers. Otherwise the file
/// is packaged where the patterns that match the file itself end with one that does not open with
/// `!`; where none of them matches it, the directories holding it are asked the same way, the
/// nearest first and the package's root last, as a directory whose name is empty. The grammar of
/// one pattern, read as Cargo reads it:
///
/// - a pattern opening with `#` is a comment; one opening with `!` negates what it matches, and
///   `\!` or `\#` opens one with that character instead; trailing whitespace is taken off, unless
///   a `\` escapes its last space;
/// - a pattern ending with `/` matches a directory alone; one with no other `/` matches a name at
///   any depth, and any other is anchored at the package's root, a leading `/` included;
/// - `*` matches any run of characters but `/`, `?` one character but `/`, `[...]` one character
///   of a set or a range, `[!...]` or `[^...]` one outside it, an unclosed `[` being the character
///   itself, `{a,b}` either alternative, `\` the character after it, and `**` any number of whole
///   segments where it is a segment of its own, a trailing `/**` matching inside a directory
///   alone;
/// - the comparison is case-sensitive.
///
/// The files Cargo packages whatever the patterns say, such as the `Cargo.toml` itself and the
/// files `readme` and `license-file` name, are not answered here: the answer is about the
/// patterns alone.
///
/// # Errors
///
/// Returns a [`PatternError`] naming the first pattern Cargo refuses, for which `cargo package`
/// lists no file: an unclosed `{`, a `}` that closes none, a range whose end is below its start,
/// and a `\` that ends the pattern.
pub fn include_matches(patterns: &[String], rel: &str) -> Result<bool, PatternError> {
    if patterns.is_empty() {
        return Ok(true);
    }
    let set = Set::parse(patterns)?;
    Ok(set.decides(rel, false) == Some(Verdict::Matched))
}

/// Whether a package whose `exclude` holds `patterns`, and that sets no `include`, leaves out the
/// file `rel`, a path relative to the package's root with `/` between its segments.
///
/// Cargo reads `exclude` only where `include` is unset or empty. A file is left out where a
/// directory holding it, below the package's root, is left out, read as [`include_matches`]
/// reads a file, its own directories asked after it; and otherwise where the patterns read so
/// leave out the file itself. So a `!` pattern brings back a file whose name a pattern matched,
/// and none inside a directory a pattern left out, as `cargo package --list` was measured to
/// list them. The grammar of a pattern is that of [`include_matches`], and its answer is about the
/// patterns alone in the same way.
///
/// # Errors
///
/// Returns a [`PatternError`] for a pattern Cargo refuses, as [`include_matches`] does.
pub fn exclude_matches(patterns: &[String], rel: &str) -> Result<bool, PatternError> {
    let set = Set::parse(patterns)?;
    let names: Vec<&str> = rel.split('/').filter(|s| !s.is_empty()).collect();
    for k in 1..names.len() {
        if set.decides(&names[..k].join("/"), true) == Some(Verdict::Matched) {
            return Ok(true);
        }
    }
    Ok(set.decides(rel, false) == Some(Verdict::Matched))
}

/// A pattern Cargo refuses, so that `cargo package` lists no file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatternError {
    pattern: String,
    reason: &'static str,
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the pattern `{}` {}", self.pattern, self.reason)
    }
}

impl std::error::Error for PatternError {}

/// What the patterns that match a path say of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    /// The last of them opens with no `!`.
    Matched,
    /// The last of them opens with `!`.
    Negated,
}

/// The patterns of one list, in order, a comment and an empty one dropped.
struct Set(Vec<Pattern>);

impl Set {
    fn parse(patterns: &[String]) -> Result<Set, PatternError> {
        let mut out = Vec::new();
        for pattern in patterns {
            if let Some(p) = Pattern::parse(pattern).map_err(|reason| PatternError {
                pattern: pattern.clone(),
                reason,
            })? {
                out.push(p);
            }
        }
        Ok(Set(out))
    }

    /// What the patterns say of `path`, a file or, where `is_dir`, a directory: those that match
    /// it decide, the last of them winning; where none does, those that match the nearest
    /// directory holding it, up to the root, whose path is empty.
    fn decides(&self, path: &str, is_dir: bool) -> Option<Verdict> {
        let names: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if let Some(v) = self.last_match(&names.join("/"), is_dir) {
            return Some(v);
        }
        (0..names.len())
            .rev()
            .find_map(|k| self.last_match(&names[..k].join("/"), true))
    }

    fn last_match(&self, path: &str, is_dir: bool) -> Option<Verdict> {
        let chars: Vec<char> = path.chars().collect();
        self.0
            .iter()
            .rev()
            .find(|p| (!p.directory || is_dir) && p.matches(&chars))
            .map(|p| {
                if p.negated {
                    Verdict::Negated
                } else {
                    Verdict::Matched
                }
            })
    }
}

/// One pattern, translated as Cargo's gitignore reading translates a line into a glob over the
/// whole path.
struct Pattern {
    /// Whether it opens with `!`.
    negated: bool,
    /// Whether it ends with `/`, so it matches a directory alone.
    directory: bool,
    /// Each glob the pattern's alternatives expand to; the pattern matches where one does.
    globs: Vec<Vec<Token>>,
}

impl Pattern {
    /// The pattern, `None` for a comment or a pattern empty once its trailing whitespace is off,
    /// or the reason Cargo refuses it.
    fn parse(line: &str) -> Result<Option<Pattern>, &'static str> {
        if line.starts_with('#') {
            return Ok(None);
        }
        let mut line = if line.ends_with("\\ ") {
            line
        } else {
            line.trim_end()
        };
        if line.is_empty() {
            return Ok(None);
        }
        let mut negated = false;
        let mut anchored = false;
        // A `\!` or a `\#` that opens a pattern needs no rule of its own: the `\` escapes the
        // character as it does anywhere, and the pattern does not open with `!` or `#`.
        if let Some(rest) = line.strip_prefix('!') {
            negated = true;
            line = rest;
        }
        if let Some(rest) = line.strip_prefix('/') {
            line = rest;
            anchored = true;
        }
        let mut directory = false;
        if let Some(rest) = line.strip_suffix('/') {
            directory = true;
            line = rest.strip_suffix('\\').unwrap_or(rest);
        }
        let mut glob = line.to_string();
        // A pattern with no `/` matches a name at any depth.
        if !anchored && !line.contains('/') && !(glob.starts_with("**/") || glob == "**") {
            glob = format!("**/{glob}");
        }
        // A trailing `/**` matches what is inside the directory, never the directory itself.
        if glob.ends_with("/**") {
            glob.push_str("/*");
        }
        let chars: Vec<char> = glob.chars().collect();
        let globs = expand(&chars)?
            .iter()
            .map(|alternative| tokens(alternative))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(Pattern {
            negated,
            directory,
            globs,
        }))
    }

    fn matches(&self, path: &[char]) -> bool {
        self.globs.iter().any(|glob| match glob.as_slice() {
            // A glob that is `**` alone matches every path.
            [Token::RecursivePrefix] => true,
            tokens => glob_match(tokens, path),
        })
    }
}

/// The alternatives of `{a,b}` groups expanded into one glob each, an empty alternative dropped,
/// as Cargo's glob reading drops it; a group inside a group is expanded as well. A `\` keeps the
/// character after it, and a bracket expression keeps its braces.
fn expand(p: &[char]) -> Result<Vec<Vec<char>>, &'static str> {
    let mut i = 0;
    let out = expand_seq(p, &mut i, false)?;
    if i < p.len() {
        return Err("closes with `}` a group it never opened");
    }
    Ok(out)
}

/// The expansions of `p` from `i` up to the end, or, `inside` a group, up to the `,` or the `}`
/// that ends the alternative, which it leaves at `i`.
fn expand_seq(p: &[char], i: &mut usize, inside: bool) -> Result<Vec<Vec<char>>, &'static str> {
    let mut out: Vec<Vec<char>> = vec![Vec::new()];
    let push = |out: &mut Vec<Vec<char>>, c: &[char]| {
        for o in out.iter_mut() {
            o.extend_from_slice(c);
        }
    };
    while *i < p.len() {
        match p[*i] {
            '\\' => {
                let end = (*i + 2).min(p.len());
                push(&mut out, &p[*i..end]);
                *i = end;
            }
            '[' => {
                let end = class_end(p, *i).map_or(*i + 1, |e| e + 1);
                push(&mut out, &p[*i..end]);
                *i = end;
            }
            '{' => {
                *i += 1;
                let mut alternatives: Vec<Vec<char>> = Vec::new();
                loop {
                    let alternative = expand_seq(p, i, true)?;
                    alternatives.extend(alternative.into_iter().filter(|a| !a.is_empty()));
                    match p.get(*i) {
                        Some(',') => *i += 1,
                        Some('}') => {
                            *i += 1;
                            break;
                        }
                        _ => return Err("opens with `{` a group it never closes"),
                    }
                }
                out = out
                    .iter()
                    .flat_map(|prefix| {
                        alternatives.iter().map(move |a| {
                            let mut whole = prefix.clone();
                            whole.extend_from_slice(a);
                            whole
                        })
                    })
                    .collect();
            }
            ',' | '}' if inside => return Ok(out),
            '}' => return Ok(out),
            c => {
                push(&mut out, &[c]);
                *i += 1;
            }
        }
    }
    Ok(out)
}

/// The index of the `]` that closes the bracket expression opening at `open`, or `None` where none
/// closes it.
fn class_end(p: &[char], open: usize) -> Option<usize> {
    let mut i = open + 1;
    if matches!(p.get(i), Some('!') | Some('^')) {
        i += 1;
    }
    let start = i;
    while i < p.len() {
        // A `]` right after the opening is a member, as in gitignore.
        if p[i] == ']' && i > start {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// One element of a glob over a whole path.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Literal(char),
    /// `?`: one character but `/`.
    Any,
    /// `*`: any run of characters but `/`.
    Star,
    /// `**/` at the glob's start: nothing, or any run of characters that ends with `/`.
    RecursivePrefix,
    /// `/**` at the glob's end: nothing, `/`, or `/` and any run of characters.
    RecursiveSuffix,
    /// `/**/` inside the glob: `/`, or any run of characters between two `/`.
    RecursiveMiddle,
    /// `[...]`: one character but `/`, of the ranges, or outside them where negated.
    Class {
        negated: bool,
        ranges: Vec<(char, char)>,
    },
}

/// The tokens of one glob with no group left in it.
fn tokens(p: &[char]) -> Result<Vec<Token>, &'static str> {
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    while i < p.len() {
        match p[i] {
            '\\' => {
                let c = *p
                    .get(i + 1)
                    .ok_or("ends with a `\\` that escapes nothing")?;
                out.push(Token::Literal(c));
                i += 2;
            }
            '?' => {
                out.push(Token::Any);
                i += 1;
            }
            '*' if p.get(i + 1) == Some(&'*') => {
                let after = p.get(i + 2).copied();
                let before = i.checked_sub(1).map(|b| p[b]);
                i += 2;
                if out.is_empty() && matches!(after, None | Some('/')) {
                    out.push(Token::RecursivePrefix);
                    if after.is_some() {
                        i += 1;
                    }
                } else if before == Some('/') && after.is_none() {
                    out.pop();
                    out.push(Token::RecursiveSuffix);
                } else if before == Some('/') && after == Some('/') {
                    out.pop();
                    out.push(Token::RecursiveMiddle);
                    i += 1;
                } else {
                    // A `**` that is no segment of its own is a `*`.
                    out.push(Token::Star);
                }
            }
            '*' => {
                out.push(Token::Star);
                i += 1;
            }
            '[' => match class_end(p, i) {
                Some(end) => {
                    out.push(class(&p[i + 1..end])?);
                    i = end + 1;
                }
                // An unclosed bracket is the character itself.
                None => {
                    out.push(Token::Literal('['));
                    i += 1;
                }
            },
            c => {
                out.push(Token::Literal(c));
                i += 1;
            }
        }
    }
    Ok(out)
}

/// The bracket expression between its `[` and its `]`: a `]` first is a member, a `-` between two
/// members names a range, and a `-` first or last is a member.
fn class(body: &[char]) -> Result<Token, &'static str> {
    let (negated, body) = match body.first() {
        Some('!') | Some('^') => (true, &body[1..]),
        _ => (false, body),
    };
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < body.len() {
        if i + 2 < body.len() && body[i + 1] == '-' {
            if body[i + 2] < body[i] {
                return Err("holds a range whose end is below its start");
            }
            ranges.push((body[i], body[i + 2]));
            i += 3;
        } else {
            ranges.push((body[i], body[i]));
            i += 1;
        }
    }
    Ok(Token::Class { negated, ranges })
}

/// Whether `tokens` match the whole of `path`.
fn glob_match(tokens: &[Token], path: &[char]) -> bool {
    let Some((first, rest)) = tokens.split_first() else {
        return path.is_empty();
    };
    match first {
        Token::Literal(c) => path.first() == Some(c) && glob_match(rest, &path[1..]),
        Token::Any => path.first().is_some_and(|c| *c != '/') && glob_match(rest, &path[1..]),
        Token::Class { negated, ranges } => path.first().is_some_and(|c| {
            *c != '/'
                && ranges.iter().any(|(lo, hi)| lo <= c && c <= hi) != *negated
                && glob_match(rest, &path[1..])
        }),
        Token::Star => (0..=path.len())
            .take_while(|&k| k == 0 || path[k - 1] != '/')
            .any(|k| glob_match(rest, &path[k..])),
        Token::RecursivePrefix => {
            glob_match(rest, path)
                || (path.first() == Some(&'/') && glob_match(rest, &path[1..]))
                || (1..=path.len()).any(|k| path[k - 1] == '/' && glob_match(rest, &path[k..]))
        }
        Token::RecursiveSuffix => {
            glob_match(rest, path)
                || (path.first() == Some(&'/')
                    && (1..=path.len()).any(|k| glob_match(rest, &path[k..])))
        }
        Token::RecursiveMiddle => {
            path.first() == Some(&'/')
                && (1..=path.len()).any(|k| path[k - 1] == '/' && glob_match(rest, &path[k..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(patterns: &[&str]) -> Vec<String> {
        patterns.iter().map(|p| p.to_string()).collect()
    }

    /// The verdicts for the file `CHANGELOG.md` at the package's root and for the one in its
    /// directory `docs`, under `patterns` read as `include`.
    fn included(patterns: &[&str]) -> (bool, bool) {
        let patterns = strings(patterns);
        let at = |rel| include_matches(&patterns, rel).expect("a pattern Cargo reads");
        (at("CHANGELOG.md"), at("docs/CHANGELOG.md"))
    }

    /// The same, read as `exclude`, each `true` where the file is packaged.
    fn kept(patterns: &[&str]) -> (bool, bool) {
        let patterns = strings(patterns);
        let at = |rel| !exclude_matches(&patterns, rel).expect("a pattern Cargo reads");
        (at("CHANGELOG.md"), at("docs/CHANGELOG.md"))
    }

    const BOTH: (bool, bool) = (true, true);
    const ROOT: (bool, bool) = (true, false);
    const DOCS: (bool, bool) = (false, true);
    const NONE: (bool, bool) = (false, false);

    /// The claim, the `include` patterns as `cargo package --list` lists the file `CHANGELOG.md` at
    /// a package's root and the one in its directory `docs` under each, every verdict measured, as
    /// `path@cargo-reader@docs/grounding/cargo-manifest.md` records them: the grammar of a
    /// segment, the anchoring, a pattern of a directory, the root read as a directory with an
    /// empty name, a trailing `/**`, the escapes and the trailing whitespace, the comments, the
    /// groups, the brackets and the empty list. Mutations checked, each caught: in `class_end`,
    /// `if p[i] == ']' && i > start {` made `if p[i] == ']' {`, so a `]` first closes the
    /// expression; in `class`, a patch reading a trailing `-` as a range up to `char::MAX`; in
    /// `include_matches`, `if patterns.is_empty() {` made `if false {`; in `Pattern::parse`,
    /// `if glob.ends_with("/**") {` made `if false {`, `line.trim_end()` made `line`,
    /// `if line.starts_with('#') {` made `if false {`, and `if !anchored && !line.contains('/')`
    /// made `if !line.contains('/')`; in `Set::decides`, a `.filter(|&k| k > 0)` added before the
    /// `find_map` over the directories, so the root is asked nothing; in `Set::last_match`,
    /// `(!p.directory || is_dir) && p.matches(&chars)` made `p.matches(&chars)`; in `tokens`,
    /// an escaped character pushed as `Token::Literal('\\')`; in `expand_seq`, the empty
    /// alternatives no longer dropped; and in `glob_match`, the `Star` arm's
    /// `.take_while(|&k| k == 0 || path[k - 1] != '/')` made `.take_while(|_| true)`.
    #[test]
    fn the_include_patterns_package_what_cargo_package_lists() {
        for (patterns, want) in [
            (&["CHANGELOG.md"][..], BOTH),
            (&["/CHANGELOG.md"], ROOT),
            (&["*.md"], BOTH),
            (&["/*.md"], ROOT),
            (&["CHANGELOG*"], BOTH),
            (&["**/CHANGELOG.md"], BOTH),
            (&["/**/CHANGELOG.md"], BOTH),
            (&["/**"], BOTH),
            (&["**"], BOTH),
            (&["/*"], BOTH),
            (&["*"], BOTH),
            (&["/"], BOTH),
            (&["/?HANGELOG.md"], ROOT),
            (&["changelog.md"], NONE),
            (&["/CHANGELOG.md/"], NONE),
            (&["CHANGELOG.md/"], NONE),
            (&["/src"], NONE),
            (&[""], NONE),
            (&["   "], NONE),
            (&["."], NONE),
            (&["./CHANGELOG.md"], NONE),
            (&["/src/**", "/README.md"], NONE),
            (&[], BOTH),
            (&["/CHANGELOG.md/**"], NONE),
            (&["CHANGELOG.md/**"], NONE),
            (&["**/CHANGELOG.md/**"], NONE),
            (&["docs/**"], DOCS),
            (&["docs/**/"], NONE),
            (&["/docs/**/*"], DOCS),
            (&["docs/**/CHANGELOG.md"], DOCS),
            (&["/**/CHANGELOG.md/"], NONE),
            (&["*/"], BOTH),
            (&["/*/"], BOTH),
            (&["**/"], BOTH),
            (&["/**/"], BOTH),
            (&["?*/"], DOCS),
            (&["CH**"], BOTH),
            (&["**CHANGELOG.md"], BOTH),
            (&["CHANGELOG\\.md"], BOTH),
            (&["CHANGE\\LOG.md"], BOTH),
            (&["\\CHANGELOG.md"], BOTH),
            (&["CHANGELOG.m\\?"], NONE),
            (&["CHANGELOG.md "], BOTH),
            (&["CHANGELOG.md\t"], BOTH),
            (&["CHANGELOG.md\\ "], NONE),
            (&[" CHANGELOG.md"], NONE),
            (&["#CHANGELOG.md"], NONE),
            (&["CHANGELOG.md", "#!CHANGELOG.md"], BOTH),
            (&["\\#CHANGELOG.md"], NONE),
            (&["\\!CHANGELOG.md"], NONE),
            (&["\\!CHANGELOG.md", "/CHANGELOG.md"], ROOT),
            (&["{CHANGELOG,README}.md"], BOTH),
            (&["{docs/CHANGELOG.md,x}"], DOCS),
            (&["/{CHANGELOG.md,x}"], ROOT),
            (&["{a,{b,CHANGELOG.md}}"], BOTH),
            (&["{,docs/}CHANGELOG.md"], DOCS),
            (&["/{**/,}CHANGELOG.md"], BOTH),
            (&["[C"], NONE),
            (&["CHANGELOG.md", "[x"], BOTH),
            (&["/[C]HANGELOG.md"], ROOT),
            (&["/[A-D]HANGELOG.md"], ROOT),
            (&["/[C-C]HANGELOG.md"], ROOT),
            (&["/[a-z]HANGELOG.md"], NONE),
            (&["/[!C]HANGELOG.md"], NONE),
            (&["/[^X]HANGELOG.md"], ROOT),
            (&["/[^C]HANGELOG.md"], NONE),
            (&["/[]]HANGELOG.md"], NONE),
            (&["/[]C]HANGELOG.md"], ROOT),
            (&["/[!]]HANGELOG.md"], ROOT),
            (&["/[\\]]HANGELOG.md"], NONE),
            (&["/[B-]HANGELOG.md"], NONE),
            (&["/[a-]HANGELOG.md"], NONE),
            (&["/[-C]HANGELOG.md"], ROOT),
            (&["/[C-]HANGELOG.md"], ROOT),
            (&["/[A-C-]HANGELOG.md"], ROOT),
        ] {
            assert_eq!(included(patterns), want, "{patterns:?}");
        }
    }

    /// The claim, among the patterns that match a file, the last decides, and only where none
    /// matches it do those matching a directory that holds it, as measured: a `!` excludes what
    /// it matches, `!` alone matches every path, and `!/` the root alone. Mutations checked, each
    /// caught: `Set::decides` made to take the last pattern that matches the file or any
    /// directory holding it, the rule of a patch; in `Pattern::parse`, `negated = true;` made
    /// `negated = false;`; and in `Pattern::matches`, `[Token::RecursivePrefix] => true,` made
    /// `[Token::RecursivePrefix] if false => true,`.
    #[test]
    fn the_patterns_of_the_file_itself_decide_before_its_directories() {
        for (patterns, want) in [
            (&["/CHANGELOG.md", "!CHANGELOG.md"][..], NONE),
            (&["!CHANGELOG.md", "/CHANGELOG.md"], ROOT),
            (&["/*", "!/CHANGELOG.md"], DOCS),
            (&["docs", "!docs/CHANGELOG.md"], NONE),
            (&["!docs/CHANGELOG.md", "docs"], NONE),
            (&["docs/CHANGELOG.md", "!docs"], DOCS),
            (&["docs/", "!CHANGELOG.md"], NONE),
            (&["!CHANGELOG.md", "/docs/"], NONE),
            (&["!CHANGELOG.md", "/"], NONE),
            (&["/", "!*.md"], NONE),
            (&["*", "!*/"], BOTH),
            (&["CHANGELOG.md", "!*/"], BOTH),
            (&["CHANGELOG.md", "!/"], BOTH),
            (&["!"], NONE),
            (&["CHANGELOG.md", "!"], NONE),
            (&["CHANGELOG.md", "!   "], NONE),
            (&["/[C]HANGELOG.md", "!/[\\C]HANGELOG.md"], NONE),
        ] {
            assert_eq!(included(patterns), want, "{patterns:?}");
        }
    }

    /// The claim, an unclosed `[` is the character itself, as measured with a file named `[C`.
    /// Mutation checked: in `tokens`, `out.push(Token::Literal('['));` made
    /// `return Err("unclosed");`; caught.
    #[test]
    fn an_unclosed_bracket_is_the_character_itself() {
        assert_eq!(include_matches(&strings(&["[C"]), "[C"), Ok(true));
        assert_eq!(
            include_matches(&strings(&["/[C"]), "CHANGELOG.md"),
            Ok(false)
        );
    }

    /// The claim, a pattern opening with `#` is a comment, and one opening with `\#` or `\!`
    /// names a file whose name opens with that character, as measured with files named `#x` and
    /// `!x`. Mutation checked: in `Pattern::parse`, `if line.starts_with('#') {` made
    /// `if false {`; caught.
    #[test]
    fn a_comment_and_an_escaped_opening_are_read_as_cargo_reads_them() {
        for (pattern, rel, want) in [
            ("#x", "#x", false),
            ("\\#x", "#x", true),
            ("/\\#x", "#x", true),
            ("\\!x", "!x", true),
            ("!x", "!x", false),
        ] {
            assert_eq!(
                include_matches(&strings(&[pattern]), rel),
                Ok(want),
                "{pattern}"
            );
        }
    }

    /// The claim, the `exclude` patterns leave out what `cargo package --list` does not list, as
    /// measured: a file a pattern names, brought back by a later `!` pattern that names it, and
    /// every file inside a directory a pattern leaves out, below the package's root, which no `!`
    /// pattern brings back. Mutation checked: in `exclude_matches`, `for k in 1..names.len() {`
    /// made `for k in 1..1 {`; caught.
    #[test]
    fn the_exclude_patterns_leave_out_what_cargo_package_does_not_list() {
        for (patterns, want) in [
            (&["/CHANGELOG.md"][..], DOCS),
            (&["CHANGELOG.md"], NONE),
            (&[], BOTH),
            (&["*/"], NONE),
            (&["/"], NONE),
            (&["*/", "!CHANGELOG.md"], ROOT),
            (&["/", "!CHANGELOG.md"], ROOT),
            (&["**", "!CHANGELOG.md"], ROOT),
            (&["docs", "!docs/CHANGELOG.md"], ROOT),
            (&["docs/", "!CHANGELOG.md"], ROOT),
            (&["docs", "!docs"], BOTH),
            (&["*.md", "!CHANGELOG.md"], BOTH),
            (&["CHANGELOG.md", "!/"], NONE),
            (&["CHANGELOG.md/**"], BOTH),
            (&["CHANGELOG.md "], NONE),
            (&["!"], BOTH),
            (&["CHANGELOG.md", "!"], BOTH),
            (&["{CHANGELOG,X}.md"], NONE),
        ] {
            assert_eq!(kept(patterns), want, "{patterns:?}");
        }
    }

    /// The claim, a pattern Cargo refuses, so that `cargo package` lists nothing, is an error
    /// naming it, in either list and wherever it stands in the list, as measured. Mutations
    /// checked, each caught: in `expand_seq`, the arm that returns the error of an unclosed group
    /// made `_ => break,`; and in `class`, `if body[i + 2] < body[i] {` made
    /// `if false {`.
    #[test]
    fn a_pattern_cargo_refuses_is_an_error() {
        for pattern in [
            "/[z-a]HANGELOG.md",
            "{CHANGELOG.md",
            "CHANGELOG.md}",
            "CHANGELOG.md\\",
        ] {
            let patterns = strings(&[pattern, "/CHANGELOG.md"]);
            let error = include_matches(&patterns, "CHANGELOG.md").expect_err(pattern);
            assert!(error.to_string().contains(pattern), "{error}");
            assert!(
                exclude_matches(&patterns, "CHANGELOG.md").is_err(),
                "{pattern}"
            );
        }
    }
}
