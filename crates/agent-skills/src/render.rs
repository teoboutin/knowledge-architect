//! The passes of the shipped text that the build script and this crate's tests share: the build
//! script includes this file with `#[path]`, and it has no test target of its own.
//!
//! - **The comment pass.** A line whose first two characters are `%%` is a comment for this
//!   repository's maintainers, per `design@agent-skills@shipped-text-line-comments`.
//! - **The section slug pass.** A level-two heading ends with a placeholder `{{slug:<id>}}`, which
//!   ships as the slug the checker reads, so content/ defines no slug in this repository's walk
//!   and the installed copy does.
//! - **The workflow name check.** A saved workflow is called by the name its `meta` declares, not
//!   by its file's name, so the name must carry the installer's prefix as the file does.
//! - **The fragment pass.** A line that is a fragment placeholder, `{{fragment:<file>}}`, is replaced
//!   with a file of shared text from fragments/, so one text applied whole at several places has
//!   one home and every host ships it, per `design@agent-skills@content-mirrors-the-install-layout`.

use std::path::Path;

/// What opens a comment line of the shipped text.
pub(crate) const COMMENT: &str = "%%";

/// What opens a section slug placeholder; `}}` closes it.
pub(crate) const SLUG: &str = "{{slug:";

/// What opens a fragment placeholder; `}}` closes it.
pub(crate) const FRAGMENT: &str = "{{fragment:";

/// Whether a line is inside a fenced block, read as CommonMark reads one.
///
/// A fence opens on a line indented by at most three spaces with a run of at least three
/// backticks or three tildes, and closes on a line indented by at most three spaces holding a run
/// of the same character at least as long, followed by spaces alone. A fence left open runs to
/// the end of the text.
#[derive(Default)]
struct Fence(Option<(char, usize)>);

impl Fence {
    /// Step over one line, its line break removed: whether the line itself is part of a fenced
    /// block, its opening and closing lines included.
    fn fenced(&mut self, bare: &str) -> bool {
        let trimmed = bare.trim_start_matches(' ');
        let indent = bare.len() - trimmed.len();
        let run = |c: char| trimmed.chars().take_while(|&x| x == c).count();
        match self.0 {
            None if indent <= 3 => {
                for c in ['`', '~'] {
                    let len = run(c);
                    // A backtick fence's info string holds no backtick.
                    if len >= 3 && !(c == '`' && trimmed[len..].contains('`')) {
                        self.0 = Some((c, len));
                        return true;
                    }
                }
                false
            }
            None => false,
            Some((c, len)) => {
                let closing = run(c);
                if indent <= 3
                    && closing >= len
                    && trimmed[closing..].trim_end_matches(' ').is_empty()
                {
                    self.0 = None;
                }
                true
            }
        }
    }
}

/// `text` without its comment lines. A `%%` line inside a fenced block, or one with leading
/// spaces, panics with the file and the line: it would ship rather than be removed.
pub(crate) fn strip_comments(text: &str, file: &Path) -> String {
    let mut stripped = String::with_capacity(text.len());
    let mut fence = Fence::default();
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let bare = line.trim_end_matches(['\n', '\r']);
        let fenced = fence.fenced(bare);
        let trimmed = bare.trim_start_matches(' ');
        if !trimmed.starts_with(COMMENT) {
            stripped.push_str(line);
            continue;
        }
        let at = format!("{}:{}", file.display(), n + 1);
        assert!(
            !fenced,
            "{at}: a `%%` line inside a fenced block would ship"
        );
        assert!(
            bare.len() == trimmed.len(),
            "{at}: a `%%` line with leading spaces would ship"
        );
    }
    stripped
}

/// `text` with each section slug placeholder rendered into the slug the checker reads.
///
/// A placeholder is legal at one place only: at the end of a level-two heading outside a fenced
/// block, after a space, holding an id in the grammar `[a-z0-9]+(-[a-z0-9]+)+`. It ships as a
/// backticked `##<id>`, which defines a section in the installed copy, per the section rule of
/// the checker's entity table. Anywhere else it panics with the file and the line: a placeholder
/// that shipped unrendered would leave the section without its slug, and one in a fence would
/// render an illustration into a definition.
pub(crate) fn render_slugs(text: &str, file: &Path) -> String {
    let mut rendered = String::with_capacity(text.len());
    let mut fence = Fence::default();
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let bare = line.trim_end_matches(['\n', '\r']);
        let fenced = fence.fenced(bare);
        let Some(at) = bare.find(SLUG) else {
            rendered.push_str(line);
            continue;
        };
        let site = format!("{}:{}", file.display(), n + 1);
        assert!(
            !fenced,
            "{site}: a section slug placeholder inside a fenced block would ship"
        );
        assert!(
            bare.starts_with("## ") && !bare.starts_with("###"),
            "{site}: a section slug placeholder ends a level-two heading, and this line is none"
        );
        let id = bare[at + SLUG.len()..]
            .strip_suffix("}}")
            .filter(|id| !id.contains('}'))
            .unwrap_or_else(|| {
                panic!("{site}: a section slug placeholder ends its heading, closed by `}}}}`")
            });
        assert!(
            is_id(id),
            "{site}: `{id}` is no slug: two or more lower-case words and digits joined by hyphens"
        );
        assert!(
            bare[..at].ends_with(' ')
                && !bare[..at].trim_end().is_empty()
                && bare[..at].trim_end() != "##",
            "{site}: a section slug placeholder follows the heading's text and a space"
        );
        rendered.push_str(&bare[..at]);
        rendered.push_str("`##");
        rendered.push_str(id);
        rendered.push('`');
        rendered.push_str(&line[bare.len()..]);
    }
    rendered
}

/// `text` with each fragment placeholder line replaced by its fragment, and the names of the
/// fragments it used, in order, a name once per use.
///
/// A placeholder is a line that is `{{fragment:<file>}}` after any leading spaces, `<file>` the
/// name of one of `fragments`, each a file name and its text. The fragment's own `%%` lines are
/// removed first, by the comment pass's rules, and each of its non-blank lines is then prefixed with
/// the placeholder's leading spaces, so a fragment can sit inside a list item; its final line break
/// is the placeholder line's own. The pass runs before the comment and slug passes of the host, so
/// the fragment's text is otherwise treated as the host's.
///
/// It panics, naming the file and the line, on a placeholder naming no fragment; and, naming the
/// fragment, on a fragment that holds a fragment placeholder, since a fragment does not nest, or a
/// section slug placeholder, since its host supplies the heading and the slug, and the same slug
/// rendered twice in one installed file would be defined twice.
pub(crate) fn render_fragments<'a>(
    text: &str,
    fragments: &[(&'a str, &str)],
    file: &Path,
) -> (String, Vec<&'a str>) {
    let mut rendered = String::with_capacity(text.len());
    let mut used = Vec::new();
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let bare = line.trim_end_matches(['\n', '\r']);
        let trimmed = bare.trim_start_matches(' ');
        let Some(name) = trimmed
            .strip_prefix(FRAGMENT)
            .and_then(|rest| rest.strip_suffix("}}"))
        else {
            rendered.push_str(line);
            continue;
        };
        let Some(&(name, fragment)) = fragments.iter().find(|(n, _)| *n == name) else {
            panic!(
                "{}:{}: names fragments/{name}, which does not exist",
                file.display(),
                n + 1
            )
        };
        let own = format!("fragments/{name}");
        let body = strip_comments(fragment, Path::new(&own));
        assert!(
            !body
                .lines()
                .any(|l| l.trim_start_matches(' ').starts_with(FRAGMENT)),
            "{own}: a fragment holds no fragment placeholder"
        );
        assert!(
            !body.contains(SLUG),
            "{own}: a fragment holds no section slug placeholder: its host supplies the heading"
        );
        let indent = &bare[..bare.len() - trimmed.len()];
        for part in body
            .strip_suffix('\n')
            .unwrap_or(&body)
            .split_inclusive('\n')
        {
            if !part.trim().is_empty() {
                rendered.push_str(indent);
            }
            rendered.push_str(part);
        }
        rendered.push_str(&line[bare.len()..]);
        used.push(name);
    }
    (rendered, used)
}

/// What opens a saved workflow's declaration; its `name` key follows inside the object.
const WORKFLOW_META: &str = "export const meta = {";

/// Panics unless the saved workflow `text` declares `name` once, as `expected`, its installed
/// file's stem. Claude Code calls a saved workflow by its declared `name` and never by its file
/// name, so a prefixed file whose declared name lacks the prefix would sit in the installer's
/// namespace while it answers to a name a project's own workflow can take.
///
/// The declaration is a literal the harness parses, so its keys and values are literals too: the
/// scan reads the keys of the object's own level alone, skipping strings, comments and nested
/// objects and arrays, since a `name` key nested inside a value, in a comment or in a string is
/// not the one the harness reads. A `name` given twice is refused, since JavaScript keeps the last
/// and a reader takes the first.
pub(crate) fn check_workflow_name(text: &str, expected: &str, file: &Path) {
    let site = file.display();
    let start = text
        .find(WORKFLOW_META)
        .unwrap_or_else(|| panic!("{site}: a saved workflow opens with `{WORKFLOW_META}`"));
    let names = top_level_names(&text[start + WORKFLOW_META.len()..])
        .unwrap_or_else(|why| panic!("{site}: the workflow's meta {why}"));
    let [name] = names.as_slice() else {
        panic!(
            "{site}: the workflow's meta declares `name` {} times, and declares it once",
            names.len()
        )
    };
    assert!(
        name == expected,
        "{site}: the workflow declares the name `{name}`, and is installed as `{expected}`: \
         the harness calls it by the declared name, so the two are equal"
    );
}

/// The value of every `name` key at the first level of the object whose body `body` opens,
/// up to its closing brace, or why it cannot be read.
fn top_level_names(body: &str) -> Result<Vec<String>, String> {
    let chars: Vec<char> = body.chars().collect();
    let mut names = Vec::new();
    let mut depth = 1usize;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if c == '/' && chars.get(i + 1) == Some(&'*') {
            i += 2;
            while i < chars.len() && !(chars[i] == '*' && chars.get(i + 1) == Some(&'/')) {
                i += 1;
            }
            i += 2;
        } else if matches!(c, '\'' | '"' | '`') {
            let (literal, end) = string_at(&chars, i)?;
            i = end;
            if depth == 1 && literal == "name" && colon_after(&chars, &mut i) {
                names.push(value_at(&chars, &mut i)?);
            }
        } else if matches!(c, '{' | '[' | '(') {
            depth += 1;
            i += 1;
        } else if matches!(c, '}' | ']' | ')') {
            depth -= 1;
            if depth == 0 {
                return Ok(names);
            }
            i += 1;
        } else if depth == 1 && (c.is_alphabetic() || c == '_' || c == '$') {
            let from = i;
            while i < chars.len()
                && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '$')
            {
                i += 1;
            }
            let word: String = chars[from..i].iter().collect();
            if word == "name" && colon_after(&chars, &mut i) {
                names.push(value_at(&chars, &mut i)?);
            }
        } else {
            i += 1;
        }
    }
    Err("is not closed".to_string())
}

/// Whether a `:` follows position `i` after spaces, moving `i` past it when it does.
fn colon_after(chars: &[char], i: &mut usize) -> bool {
    let mut j = *i;
    while j < chars.len() && chars[j].is_whitespace() {
        j += 1;
    }
    if chars.get(j) == Some(&':') {
        *i = j + 1;
        true
    } else {
        false
    }
}

/// The quoted string that follows position `i` after spaces, `i` moved past it.
fn value_at(chars: &[char], i: &mut usize) -> Result<String, String> {
    while *i < chars.len() && chars[*i].is_whitespace() {
        *i += 1;
    }
    if !matches!(chars.get(*i), Some('\'' | '"')) {
        return Err("gives `name` a value that is no quoted string".to_string());
    }
    let (value, end) = string_at(chars, *i)?;
    *i = end;
    Ok(value)
}

/// The string literal opening at `at`, its escapes kept as written, and the position after it.
fn string_at(chars: &[char], at: usize) -> Result<(String, usize), String> {
    let quote = chars[at];
    let mut i = at + 1;
    let mut out = String::new();
    while i < chars.len() {
        match chars[i] {
            '\\' => {
                out.extend(chars.get(i..i + 2).unwrap_or(&chars[i..]));
                i += 2;
            }
            c if c == quote => return Ok((out, i + 1)),
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    Err("holds a string that is not closed".to_string())
}

/// The grammar of an entry's id in the checker's entity table, `[a-z0-9]+(-[a-z0-9]+)+`, two
/// segments or more, written out: this crate depends on no regex engine, and the checker's own
/// pattern is in the core, which depends on this crate.
fn is_id(id: &str) -> bool {
    id.contains('-')
        && id.split('-').all(|w| {
            !w.is_empty()
                && w.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use super::{check_workflow_name, render_fragments, render_slugs, strip_comments};
    use std::path::Path;

    fn fragments(text: &str, shared: &[(&'static str, &str)]) -> (String, Vec<&'static str>) {
        render_fragments(text, shared, Path::new("t.md"))
    }

    #[test]
    fn a_placeholder_line_is_replaced_by_its_fragment() {
        let shared = [("f.md", "one\ntwo\n")];
        let rendered = "# T\none\ntwo\nend\n";
        let (text, used) = fragments("# T\n{{fragment:f.md}}\nend\n", &shared);
        assert_eq!(text, rendered);
        assert_eq!(used, ["f.md"]);
    }

    #[test]
    fn an_indented_placeholder_indents_every_non_blank_line() {
        let shared = [("f.md", "one\n\n- two\n")];
        let rendered = "1. Step\n   one\n\n   - two\n";
        let (text, _) = fragments("1. Step\n   {{fragment:f.md}}\n", &shared);
        assert_eq!(text, rendered);
    }

    #[test]
    fn a_fragment_s_comment_lines_are_removed() {
        let shared = [("f.md", "%% why\nkept\n")];
        let rendered = "kept\n";
        let (text, _) = fragments("{{fragment:f.md}}\n", &shared);
        assert_eq!(text, rendered);
    }

    #[test]
    fn a_text_with_no_fragment_placeholder_is_unchanged() {
        let text = "a {{fragment:f.md}} in prose\n";
        let (rendered, used) = fragments(text, &[("f.md", "x\n")]);
        assert_eq!(rendered, text);
        assert!(used.is_empty());
    }

    #[test]
    #[should_panic(expected = "t.md:2: names fragments/g.md, which does not exist")]
    fn a_placeholder_naming_no_fragment_fails() {
        fragments("x\n{{fragment:g.md}}\n", &[("f.md", "x\n")]);
    }

    #[test]
    #[should_panic(expected = "fragments/f.md: a fragment holds no fragment placeholder")]
    fn a_nested_fragment_fails() {
        fragments(
            "{{fragment:f.md}}\n",
            &[("f.md", "  {{fragment:g.md}}\n"), ("g.md", "x\n")],
        );
    }

    #[test]
    #[should_panic(expected = "fragments/f.md: a fragment holds no section slug placeholder")]
    fn a_fragment_holding_a_slug_placeholder_fails() {
        fragments("{{fragment:f.md}}\n", &[("f.md", "## A {{slug:a-b}}\n")]);
    }

    fn strip(text: &str) -> String {
        strip_comments(text, Path::new("t.md"))
    }

    fn slugs(text: &str) -> String {
        render_slugs(text, Path::new("t.md"))
    }

    #[test]
    fn a_placeholder_ending_a_level_two_heading_ships_as_its_slug() {
        // Each expected text is bound to a name: an unbound literal of this crate is prose to the
        // checker's walk, and the slug it holds would read as a definition.
        let rendered = "# T\n\n## The axes `##review-axes`\nbody\n";
        assert_eq!(
            slugs("# T\n\n## The axes {{slug:review-axes}}\nbody\n"),
            rendered
        );
        let crlf = "## A `##a-1`\r\nx";
        assert_eq!(slugs("## A {{slug:a-1}}\r\nx"), crlf);
    }

    #[test]
    fn a_text_with_no_placeholder_is_unchanged() {
        let text = "## Plain\n```\n## {{x}}\n```\n";
        assert_eq!(slugs(text), text);
    }

    #[test]
    #[should_panic(
        expected = "t.md:2: a section slug placeholder inside a fenced block would ship"
    )]
    fn a_placeholder_inside_a_fence_fails() {
        slugs("```\n## A {{slug:a-b}}\n```\n");
    }

    #[test]
    #[should_panic(expected = "t.md:1: a section slug placeholder ends a level-two heading")]
    fn a_placeholder_on_a_level_three_heading_fails() {
        slugs("### A {{slug:a-b}}\n");
    }

    #[test]
    #[should_panic(expected = "ends a level-two heading")]
    fn a_placeholder_in_prose_fails() {
        slugs("see {{slug:a-b}} here\n");
    }

    #[test]
    #[should_panic(expected = "closed by")]
    fn a_placeholder_before_more_text_fails() {
        slugs("## A {{slug:a-b}} more\n");
    }

    #[test]
    #[should_panic(expected = "is no slug")]
    fn a_placeholder_outside_the_id_grammar_fails() {
        slugs("## A {{slug:Not_An_Id}}\n");
    }

    /// The claim: a slug of one segment is no slug, since every id the checker defines has two or
    /// more. Mutation checked: `is_id` reverted to one segment or more passes this placeholder.
    #[test]
    #[should_panic(expected = "is no slug")]
    fn a_placeholder_of_one_segment_fails() {
        slugs("## A {{slug:outcomes}}\n");
    }

    #[test]
    #[should_panic(expected = "follows the heading's text and a space")]
    fn a_placeholder_with_no_heading_text_fails() {
        slugs("## {{slug:a-b}}\n");
    }

    #[test]
    fn a_comment_line_is_removed_and_nothing_else() {
        assert_eq!(strip("a\n%% why\nb\n"), "a\nb\n");
        assert_eq!(strip("a\r\n%% why\r\nb"), "a\r\nb");
        assert_eq!(strip("a\n%% last, no line break"), "a\n");
    }

    #[test]
    fn a_comment_after_a_closed_fence_is_removed() {
        assert_eq!(strip("```sh\nls\n```\n%% why\nb\n"), "```sh\nls\n```\nb\n");
        assert_eq!(strip("~~~~\nx\n~~~~~\n%% why\n"), "~~~~\nx\n~~~~~\n");
    }

    #[test]
    #[should_panic(expected = "t.md:2: a `%%` line inside a fenced block would ship")]
    fn a_comment_inside_a_fence_fails() {
        strip("```\n%% x\n```\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_shorter_run_does_not_close_a_longer_fence() {
        strip("````markdown\n```sh\n%% x\n```\n````\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_bare_shorter_run_does_not_close_a_longer_fence() {
        strip("````\n```\n%% x\n````\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_tilde_run_does_not_close_a_backtick_fence() {
        strip("```\n~~~\n%% x\n```\n");
    }

    #[test]
    #[should_panic(expected = "inside a fenced block")]
    fn a_fence_left_open_runs_to_the_end() {
        strip("```\nx\n%% y\n");
    }

    #[test]
    fn a_run_with_an_info_string_does_not_close_a_fence() {
        assert_eq!(
            strip("```markdown\n```rust\n```\n\n%% outside\nb\n"),
            "```markdown\n```rust\n```\n\nb\n"
        );
    }

    #[test]
    #[should_panic(expected = "t.md:1: a `%%` line with leading spaces would ship")]
    fn an_indented_comment_fails() {
        strip("  %% x\n");
    }

    #[test]
    #[should_panic(expected = "follows the heading's text and a space")]
    fn a_placeholder_glued_to_the_heading_text_fails() {
        slugs("## Text{{slug:a-b}}\n");
    }

    fn named(text: &str) {
        check_workflow_name(text, "knowledge-architect-w", Path::new("w.js"));
    }

    #[test]
    fn a_workflow_declaring_its_installed_name_passes() {
        named("export const meta = {\n  name: 'knowledge-architect-w',\n  description: 'd',\n}\n");
        named("export const meta = {\n  name: \"knowledge-architect-w\"\n}\nreturn 1\n");
    }

    #[test]
    #[should_panic(expected = "w.js: the workflow declares the name `w`")]
    fn a_workflow_declaring_another_name_fails() {
        named("export const meta = {\n  name: 'w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "declares `name` 0 times")]
    fn a_name_outside_the_declaration_is_not_read() {
        named("export const meta = {\n  description: 'd',\n}\nconst x = {\n  name: 'knowledge-architect-w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "w.js: the workflow declares the name `w`")]
    fn a_name_nested_in_a_value_is_not_read() {
        named("export const meta = {\n  example: {\n    name: 'knowledge-architect-w',\n  },\n  name: 'w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "declares `name` 2 times")]
    fn a_name_given_twice_fails() {
        named("export const meta = {\n  name: 'knowledge-architect-w',\n  name: 'w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "declares `name` 0 times")]
    fn a_name_after_a_one_line_meta_is_not_read() {
        named("export const meta = { description: 'd' }\nconst x = {\n  name: 'knowledge-architect-w',\n}\n");
    }

    #[test]
    #[should_panic(expected = "w.js: the workflow declares the name `w`")]
    fn a_name_in_a_comment_or_a_string_is_not_read() {
        named("export const meta = {\n  /* name: 'knowledge-architect-w' */\n  description: `a\nname: 'knowledge-architect-w'`,\n  name: 'w',\n}\n");
    }

    #[test]
    fn a_name_on_the_opening_line_or_quoted_as_a_key_is_read() {
        named("export const meta = { name: 'knowledge-architect-w', phases: [{ title: 'a' }] }\n");
        named("export const meta = {\r\n  'name': \"knowledge-architect-w\", // the name\r\n}\r\n");
    }

    #[test]
    #[should_panic(expected = "w.js: a saved workflow opens with")]
    fn a_workflow_with_no_declaration_fails() {
        named("return 1\n");
    }
}
