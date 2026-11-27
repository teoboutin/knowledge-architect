//! The texts the manifest determines: a crate's preamble, and the assembled file of the whole
//! project, rendered from the parsed changelogs, with the rule of what the assembled file omits.
//! Nothing here writes, as nothing in `crate::index` does.
//!
//! A module of its own, not `crate::index`, whose reference is the register indexes' heads. The
//! rule of what the assembled file omits is here, beside the layout that applies it, since every
//! change to one is read with the other; the writers' refusal, which reads that rule beside the
//! heads on what a writer may lose, is `super::refusal`'s.

use std::path::{Path, PathBuf};

use super::{parse, Changelog, ChangelogTable, Entry, Heading, Section, Tag, Version};
use crate::finding::slashed;
use crate::manifest::Manifest;
use crate::model::Model;

/// The opening of every file this tool generates, as an `index.md` opens: the assembled file is
/// told apart from a hand-written one at its path by it.
pub const GENERATED_HEADER: &str = "**Generated — do not edit.**";

/// The preamble a crate's changelog opens with, as the manifest determines it: the package's
/// name, that the file holds that crate's entries only, the assembled file named in words where
/// one is assembled, with its link where `assembled-url` sets one, the categories in their
/// order, and the `unclassed` versions where any, per `design@check@category-order-in-preamble`;
/// the link as `design@check@preamble-links-only-the-declared-url` says.
///
/// Each crate's file is thereby self-contained: a reader with no manifest of the publisher, such
/// as a consumer's command over the registry sources, reads the categories and the unclassed
/// versions here.
pub fn expected_preamble(
    table: &ChangelogTable,
    package: &str,
    assembled: Option<&Path>,
) -> String {
    let mut out =
        format!("# Changelog of {package}\n\nThis file holds the entries of this crate only.");
    if let Some(path) = assembled {
        let path = slashed(path);
        let named = match table.assembled_url() {
            Some(url) => format!("[{path}]({url})"),
            None => path,
        };
        out.push_str(&format!(
            " The changelog of the whole project is {named} in its repository."
        ));
    }
    out.push_str(&format!(
        "\n\nCategories, in order: {}.",
        table.categories().join(", ")
    ));
    if !table.unclassed().is_empty() {
        let versions: Vec<String> = table.unclassed().iter().map(Version::to_string).collect();
        out.push_str(&format!(
            "\n\nVersions with no category and no class: {}.",
            versions.join(", ")
        ));
    }
    out
}

/// The assembled file's text over the changelogs of `crates`, each `(component, package, log)`:
/// the Component's name, which `[changelog.category-components]` is read against, the package
/// name, which the crate's heading writes, and the crate's parse, in the manifest's order of the
/// Components. `command` is the command the generated header names, as in `<command> index`.
///
/// The layout, per `design@model@assembled-changelog-layout`, the crates in the manifest's order,
/// per `design@model@assembled-crates-in-the-manifest-s-order`, and named by package, per
/// `design@model@assembled-crates-named-by-package`: the generated header; then each
/// version, `Next release` first and the released ones in descending order, as `## <version>`;
/// under it each category in its declared order, as `### <category>`; under it each crate, as
/// `#### <package>`; under it the crate's entries of that category, `- <class>: <text>`, in the
/// order of its file. A version `unclassed` lists has no category level: `#### <package>`, then
/// `- <text>`. An empty level is omitted.
///
/// It omits every crate's preamble, a line the parse could not read, an entry with no tag in a
/// classed version, and an entry whose category the manifest does not admit for its crate: a caller that must lose
/// nothing, as a writer must, asks [`refusal`](super::refusal) first, which refuses over each.
pub fn assembled_text(
    table: &ChangelogTable,
    crates: &[(&str, &str, &Changelog)],
    command: &str,
) -> String {
    // One layout for the assembled file and a consumer's command, `assembled_body_by_table`, here
    // with the manifest's one table for every crate, so the two keep one rule of the layout.
    let tabled: Vec<(&str, &str, &ChangelogTable, &Changelog)> = crates
        .iter()
        .map(|(component, package, log)| (*component, *package, table, *log))
        .collect();
    format!(
        "{GENERATED_HEADER} `{command} index`\n{}",
        assembled_body_by_table(table.categories(), &tabled)
    )
}

/// The layout of [`assembled_text`] with no generated header, over crates each placed by a table
/// of its own, each
/// `(component, package, table, log)`: an entry is placed under its own crate's table by the rule
/// [`unplaced`] reads, and the categories are `categories`, in their order, each written where a
/// crate has an entry of it. In a version a crate's table lists as `unclassed`, that crate has no category
/// level, `#### <package>` then `- <text>`, and its blocks come first under the version, before
/// the categories the other crates' entries take. Each version is `## <version>` preceded by a
/// blank line, and the text is empty where no crate holds an entry the layout has a place for.
/// With one table for every crate, as [`assembled_text`] passes, the text is the assembled
/// file's below its header.
///
/// For a text that is no file, such as a consumer's command over crates whose preambles it read
/// one by one: the caller keeps in each [`Changelog`] the sections it prints, and passes as
/// `categories` every category of the tables, and [`unplaced`] under each crate's table and the
/// parse's `unread` lines say what this text omits of them.
pub fn assembled_body_by_table(
    categories: &[String],
    crates: &[(&str, &str, &ChangelogTable, &Changelog)],
) -> String {
    let mut versions: Vec<Heading> = Vec::new();
    for (_, _, _, log) in crates {
        for section in &log.sections {
            if !versions.contains(&section.heading) {
                versions.push(section.heading);
            }
        }
    }
    versions.sort_by(|a, b| match (a, b) {
        (Heading::NextRelease, Heading::NextRelease) => std::cmp::Ordering::Equal,
        (Heading::NextRelease, _) => std::cmp::Ordering::Less,
        (_, Heading::NextRelease) => std::cmp::Ordering::Greater,
        (Heading::Version(x), Heading::Version(y)) => y.cmp(x),
    });
    let mut out = String::new();
    for version in versions {
        let entries = |component: &str,
                       table: &ChangelogTable,
                       log: &Changelog,
                       category: Option<&str>|
         -> Vec<String> {
            log.sections
                .iter()
                .filter(|s| s.heading == version)
                .flat_map(|s| &s.entries)
                .filter_map(|e| match (category, place(table, component, version, e)?) {
                    (None, Place::Unclassed) => Some(format!("- {}", e.raw)),
                    (Some(c), Place::Under(tag)) if tag.category == c => {
                        Some(format!("- {}: {}", tag.class, tag.text))
                    }
                    _ => None,
                })
                .collect()
        };
        let mut body = String::new();
        for (component, package, table, log) in crates {
            if is_unclassed(table, version) {
                body.push_str(&block(package, &entries(component, table, log, None)));
            }
        }
        for category in categories {
            let mut under = String::new();
            // A crate whose table lists the version as unclassed places no entry under a
            // category, by `place`, so its block here is empty.
            for (component, package, table, log) in crates {
                under.push_str(&block(
                    package,
                    &entries(component, table, log, Some(category)),
                ));
            }
            if !under.is_empty() {
                body.push_str(&format!("\n### {category}\n{under}"));
            }
        }
        if !body.is_empty() {
            let name = match version {
                Heading::NextRelease => super::NEXT_RELEASE.to_string(),
                Heading::Version(v) => v.to_string(),
            };
            out.push_str(&format!("\n## {name}\n{body}"));
        }
    }
    out
}

/// Where the assembled file writes an entry.
enum Place<'e> {
    /// In a version `unclassed` lists, with no category level, as written.
    Unclassed,
    /// Under its tag's category.
    Under(&'e Tag),
}

/// Whether `unclassed` lists the version `heading` names.
fn is_unclassed(table: &ChangelogTable, heading: Heading) -> bool {
    matches!(heading, Heading::Version(v) if table.unclassed().contains(&v))
}

/// Where the assembled file writes `entry` of `component`'s changelog, in the section `heading`
/// names, or `None` where it has no place for it: in a version `unclassed` lists, every entry,
/// as written; in any other, an entry whose tag names a category the manifest admits for
/// `component`, and no entry with no tag.
///
/// **The one rule of what the assembled file keeps**, which [`assembled_body_by_table`], and
/// through it [`assembled_text`], writes by, and [`unplaced`] and
/// [`omitted`] read, so a writer never drops a line its refusal let through, per
/// `design@model@assembled-file-not-overwritten`, and a consumer's command reports every entry it
/// leaves out.
fn place<'e>(
    table: &ChangelogTable,
    component: &str,
    heading: Heading,
    entry: &'e Entry,
) -> Option<Place<'e>> {
    if is_unclassed(table, heading) {
        return Some(Place::Unclassed);
    }
    entry
        .tag
        .as_ref()
        .filter(|tag| table.admits(&tag.category, component))
        .map(Place::Under)
}

/// The first line of each entry of `section`, in `component`'s changelog, that the assembled
/// layout has no place for, in the order of the file: in a version the table lists as
/// `unclassed`, none; in any other, an entry with no tag, and one whose tag names a category the
/// table does not admit for `component`. [`assembled_body_by_table`] leaves each of them out.
pub fn unplaced(table: &ChangelogTable, component: &str, section: &Section) -> Vec<u32> {
    section
        .entries
        .iter()
        .filter(|e| place(table, component, section.heading, e).is_none())
        .map(|e| e.line)
        .collect()
}

/// What a crate's preamble states, read with no manifest of its publisher, as
/// [`read_preamble`] reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadPreamble {
    /// The categories, in their order, and the unclassed versions, each category admitted for
    /// every crate: a preamble states no `[changelog.category-components]`, and the publisher's
    /// check enforces its own.
    pub table: ChangelogTable,
    /// The one-based line of each line of the preamble of no shape [`expected_preamble`] writes,
    /// in the order of the file.
    pub unread: Vec<u32>,
}

/// The prefix and the end of the line that lists the categories.
const CATEGORIES: (&str, &str) = ("Categories, in order: ", ".");
/// The prefix and the end of the line that lists the unclassed versions.
const UNCLASSED: (&str, &str) = ("Versions with no category and no class: ", ".");
/// The sentence that opens the line saying whose entries the file holds.
const OWN_ENTRIES: &str = "This file holds the entries of this crate only.";

/// Read back what `preamble`, a parsed changelog's [`Changelog::preamble`], states: the inverse of
/// [`expected_preamble`], for a reader with no manifest of the crate's publisher, such as a
/// consumer's command over the sources cargo fetched, per `design@check@category-order-in-preamble`.
///
/// Each line is read by its shape, compared without the white space that ends it, so a blank
/// line is read as nothing. A line opening as one of these is of that kind: `# Changelog of
/// <package>`; the sentence that the file holds the crate's entries only, alone or followed by
/// ` The changelog of the whole project is <named> in its repository.`; `Categories, in order:
/// <c>, <c>.`; and `Versions with no category and no class: <v>, <v>.`. The first line of each
/// kind is read where it has that shape whole, and any other line is in
/// [`ReadPreamble::unread`]: a line of no kind, a second line of a kind, whether the first was
/// read or not, a package that is empty or opens with white space, which no Cargo package name
/// does, an empty name of the whole project's changelog, a list of categories holding an empty,
/// a padded or a repeated item, and a version that is no version: [`expected_preamble`] writes
/// none of them, since a package's name and the manifest's table refuse each. The name of the
/// whole project's changelog is whatever lies between the sentence's opening and its end, as the
/// table admits an assembled path that opens with white space; a version repeated in the list of
/// unclassed versions is read, as the table admits it. A
/// category holds no `,`, `:` or line break, so the list reads back as it was written. A preamble
/// that lacks the line of the categories gives a table with none, so every entry of a classed
/// version has no place in the layout.
pub fn read_preamble(preamble: &str) -> ReadPreamble {
    const TITLE: &str = "# Changelog of ";
    const WHOLE: (&str, &str) = (
        " The changelog of the whole project is ",
        " in its repository.",
    );
    // A package's name, read where it is not empty and opens with no white space, as no Cargo
    // package name holds white space: the line's end is trimmed already.
    let named = |name: &str| !name.is_empty() && name.trim_start() == name;
    let mut categories: Option<Vec<String>> = None;
    let mut unclassed: Option<Vec<Version>> = None;
    // Whether a line of each kind was met: title, own entries, categories, unclassed.
    let mut seen = [false; 4];
    let mut unread = Vec::new();
    for (i, line) in preamble.split('\n').enumerate() {
        let line = line.trim_end();
        let kind = [TITLE, OWN_ENTRIES, CATEGORIES.0, UNCLASSED.0]
            .iter()
            .position(|opening| line.starts_with(opening));
        let read = match kind {
            None => line.is_empty(),
            // The guard marks the kind met, so every later line of it is unread.
            Some(k) if std::mem::replace(&mut seen[k], true) => false,
            Some(0) => named(&line[TITLE.len()..]),
            Some(1) => {
                let rest = &line[OWN_ENTRIES.len()..];
                rest.is_empty()
                    || rest
                        .strip_prefix(WHOLE.0)
                        .and_then(|r| r.strip_suffix(WHOLE.1))
                        .is_some_and(|name| !name.is_empty())
            }
            Some(2) => {
                categories = listed(line, CATEGORIES, false)
                    .map(|list| list.iter().map(|c| c.to_string()).collect());
                categories.is_some()
            }
            Some(_) => {
                unclassed = listed(line, UNCLASSED, true)
                    .and_then(|list| list.into_iter().map(Version::parse).collect());
                unclassed.is_some()
            }
        };
        if !read {
            // The preamble opens the file, so its `i`th line is the file's line `i + 1`.
            unread.push(i as u32 + 1);
        }
    }
    ReadPreamble {
        table: ChangelogTable::with_no_manifest(
            categories.unwrap_or_default(),
            unclassed.unwrap_or_default(),
        ),
        unread,
    }
}

/// The items of `line`, a list opening with `shape.0` and ending with `shape.1`, its items
/// joined by `, `: empty where nothing is between the two; `None` where it is no such list, or an
/// item is empty, padded, holds a `,` or a `:`, which no category the manifest's table admits
/// holds, or, unless `repeats`, is repeated.
fn listed<'l>(line: &'l str, shape: (&str, &str), repeats: bool) -> Option<Vec<&'l str>> {
    let list = line.strip_prefix(shape.0)?.strip_suffix(shape.1)?;
    // A table that declares no category is written `Categories, in order: .`.
    if list.is_empty() {
        return Some(Vec::new());
    }
    let items: Vec<&str> = list.split(", ").collect();
    let well_formed = items.iter().enumerate().all(|(i, item)| {
        !item.is_empty()
            && item.trim() == *item
            && !item.contains([',', ':'])
            && (repeats || !items[..i].contains(item))
    });
    well_formed.then_some(items)
}

/// A line of a crate's changelog the assembled file omits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Omitted {
    /// A line of the preamble that the preamble the manifest determines lacks, at this one-based
    /// line.
    Preamble(u32),
    /// A line the parse could not read, at this one-based line.
    Unread(u32),
    /// An entry the assembled file has no place for, at its first line.
    Entry(u32),
}

/// The first line of `component`'s changelog `log` that the assembled file omits, or `None`
/// where it omits nothing: a line of its preamble that `expected`, the preamble the manifest
/// determines for the crate, lacks, first, then an unread line, then each entry [`place`] has no
/// place for.
///
/// The assembled file writes no preamble, since each crate's is the manifest's own text, which
/// [`expected_preamble`] gives again. A line the expected text lacks is the project's, such as
/// an entry under a misspelled version heading, which leaves every line below it in the preamble.
/// A line is compared without the white space that ends it, so a blank line, and a line that
/// differs from one of `expected` by trailing spaces alone, hold nothing to omit. A preamble that
/// lacks a line of `expected` omits nothing: the changelog check's finding on it stands alone.
pub(crate) fn omitted(
    table: &ChangelogTable,
    component: &str,
    expected: &str,
    log: &Changelog,
) -> Option<Omitted> {
    let known: Vec<&str> = expected.split('\n').map(str::trim_end).collect();
    if let Some(i) = log
        .preamble
        .split('\n')
        .position(|line| !known.contains(&line.trim_end()))
    {
        // The preamble opens the file, so its `i`th line is the file's line `i + 1`.
        return Some(Omitted::Preamble(i as u32 + 1));
    }
    if let Some(unread) = log.unread.first() {
        return Some(Omitted::Unread(unread.line));
    }
    log.sections.iter().find_map(|section| {
        unplaced(table, component, section)
            .first()
            .map(|line| Omitted::Entry(*line))
    })
}

/// One crate's level: its heading and its entries, or nothing where it has none.
fn block(package: &str, entries: &[String]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    format!("\n#### {package}\n\n{}\n", entries.join("\n"))
}

/// The changelog of each changelog crate, parsed from the walked document at its path, with its
/// Component and its package; a crate whose file the walk does not hold has an empty one.
pub(super) fn parsed(model: &Model) -> Vec<(String, String, Changelog)> {
    model
        .changelog_crates()
        .all()
        .iter()
        .map(|c| {
            let rel = c.changelog();
            let log = model
                .documents()
                .iter()
                .find(|d| d.rel == rel)
                .map(|d| parse(&d.text))
                .unwrap_or_default();
            (c.component.clone(), c.package.clone(), log)
        })
        .collect()
}

/// The assembled file, its path and the text it should hold, where one is assembled: the
/// manifest declares `[changelog]`, and the model reads two changelog crates or more, per
/// `design@model@assembled-changelog-from-two-crates`.
pub fn assembled_file(manifest: &Manifest, model: &Model) -> Option<(PathBuf, String)> {
    let table = manifest.changelog()?;
    let path = model.changelog_crates().assembled(manifest)?;
    let logs = parsed(model);
    let crates: Vec<(&str, &str, &Changelog)> = logs
        .iter()
        .map(|(c, p, l)| (c.as_str(), p.as_str(), l))
        .collect();
    Some((
        path.to_path_buf(),
        assembled_text(table, &crates, manifest.command()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(extra: &str) -> ChangelogTable {
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            &format!(
                "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\"]\n\n\
                 [walk]\nskip-dirs = []\nskip-files = []\n\n[changelog]\n\
                 categories = [\"Migration\", \"New features\", \"Workflow\"]\nunclassed = [\"0.1.0\"]\n{extra}"
            ),
        )
        .expect("a declaration");
        manifest.changelog().expect("the table").clone()
    }

    const A: &str = "# Changelog of pa\n\n## Next release\n\n- New features, minor: a feature\n  over two lines\n\
        - Migration, major: a break\n\n## 0.2.0\n\n- Migration, patch: an old fix\n\n## 0.1.0\n\n- the first\n";
    const B: &str = "# Changelog of pb\n\n## Next release\n\n- Migration, minor: b moves\n- Workflow, patch: b's workflow\n\n\
        ## 0.3.0\n\n- New features, minor: b adds\n";

    /// The claim, the layout: version, then category in its declared order, then crate in
    /// the given order, an empty level omitted, and an unclassed version with no category level.
    /// Mutations checked, each caught: in `assembled_text`, the sort's `=> y.cmp(x),` made
    /// `=> x.cmp(y),`; and an unclassed version's entries, `(None, Place::Unclassed) => Some(format!("- {}", e.raw)),`,
    /// made `(None, Place::Unclassed) => None,`.
    #[test]
    fn the_assembled_layout_is_version_then_category_then_crate() {
        let t = table("");
        let (a, b) = (parse(A), parse(B));
        let text = assembled_text(&t, &[("a", "pa", &a), ("b", "pb", &b)], "klarch");
        let want = "**Generated — do not edit.** `klarch index`\n\
            \n## Next release\n\
            \n### Migration\n\n#### pa\n\n- major: a break\n\n#### pb\n\n- minor: b moves\n\
            \n### New features\n\n#### pa\n\n- minor: a feature\n  over two lines\n\
            \n### Workflow\n\n#### pb\n\n- patch: b's workflow\n\
            \n## 0.3.0\n\n### New features\n\n#### pb\n\n- minor: b adds\n\
            \n## 0.2.0\n\n### Migration\n\n#### pa\n\n- patch: an old fix\n\
            \n## 0.1.0\n\n#### pa\n\n- the first\n";
        assert_eq!(text, want);
    }

    /// The assembled text read back: each crate's entries, by version and category, each with its
    /// class and text. A reader of the layout `assembled_text` writes, for the round trip alone.
    fn read_back(text: &str) -> Vec<(String, String, Option<String>, String)> {
        let mut out = Vec::new();
        let (mut version, mut category, mut package) = (String::new(), None, String::new());
        let mut last: Option<usize> = None;
        for line in text.lines() {
            if let Some(v) = line.strip_prefix("## ") {
                (version, category, last) = (v.to_string(), None, None);
            } else if let Some(c) = line.strip_prefix("### ") {
                (category, last) = (Some(c.to_string()), None);
            } else if let Some(p) = line.strip_prefix("#### ") {
                (package, last) = (p.to_string(), None);
            } else if let Some(e) = line.strip_prefix("- ") {
                let e = match &category {
                    Some(c) => format!("{c}, {e}"),
                    None => e.to_string(),
                };
                out.push((package.clone(), version.clone(), category.clone(), e));
                last = Some(out.len() - 1);
            } else if let (Some(i), true) = (last, line.starts_with(' ') || line.is_empty()) {
                out[i].3.push('\n');
                out[i].3.push_str(line);
            }
        }
        for entry in &mut out {
            entry.3 = entry.3.trim_end_matches('\n').to_string();
        }
        out
    }

    /// The claim: the assembled text, read back per crate, yields every entry of every
    /// crate with its category, class and text unchanged. Mutation checked: in `assembled_text`,
    /// `Some(format!("- {}: {}", tag.class, tag.text))` made to write `tag.text` with each blank
    /// line of its continuation taken out, `tag.text.replace("\n\n", "\n")`; caught.
    #[test]
    fn the_rendering_round_trips() {
        let t = table("");
        let a = parse(&format!(
            "{A}\n## Next release\n\n- Workflow, patch: kept \n\n  after a blank\n"
        ));
        let b = parse(B);
        let text = assembled_text(&t, &[("a", "pa", &a), ("b", "pb", &b)], "klarch");
        let mut back = read_back(&text);
        let mut want = Vec::new();
        for (package, log) in [("pa", &a), ("pb", &b)] {
            for section in &log.sections {
                let version = match section.heading {
                    Heading::NextRelease => "Next release".to_string(),
                    Heading::Version(v) => v.to_string(),
                };
                let unclassed = version == "0.1.0";
                for entry in &section.entries {
                    let category =
                        (!unclassed).then(|| entry.tag.as_ref().unwrap().category.clone());
                    want.push((
                        package.to_string(),
                        version.clone(),
                        category,
                        entry.raw.clone(),
                    ));
                }
            }
        }
        back.sort();
        want.sort();
        assert_eq!(back, want);
    }

    /// The claim: a line of a crate's preamble that the expected preamble lacks is omitted, at
    /// its line, before any unread line, and so are the lines below a misspelled version
    /// heading, which the parse leaves in the preamble; a preamble that lacks a line of the
    /// expected one, or ends a line with spaces, omits nothing. Mutation checked: in `omitted`,
    /// `.position(|line| !known.contains(&line.trim_end()))` made `.position(|_| false)`; caught.
    #[test]
    fn a_preamble_line_the_manifest_s_lacks_is_omitted() {
        let t = table("");
        let expected = expected_preamble(&t, "pa", None);
        let body = "## Next release\n\n- Migration, patch: a fix\n";
        let omits = |text: &str| omitted(&t, "a", &expected, &parse(text));
        assert_eq!(omits(&format!("{expected}\n\n{body}")), None);
        let trailing = expected.replace("only.", "only.  ");
        assert_eq!(omits(&format!("{trailing}\n\n{body}")), None);
        let lacking = expected.replace("This file holds the entries of this crate only.\n\n", "");
        assert_ne!(lacking, expected);
        assert_eq!(omits(&format!("{lacking}\n\n{body}")), None);
        assert_eq!(
            omits(&format!("{expected}\n\nAn extra line.\n\n{body}stray\n")),
            Some(Omitted::Preamble(9))
        );
        let misspelled = format!("{expected}\n\n{}", body.replace("release", "Release"));
        assert_eq!(omits(&misspelled), Some(Omitted::Preamble(9)));
    }

    /// The claim, the expected preamble: the package, the assembled path named in words, its
    /// link where set, the categories in their order, the unclassed versions; and no sentence on
    /// an assembled file where none is. Mutation checked: in `expected_preamble`, the
    /// `assembled_url` arm made to name the path alone; caught.
    #[test]
    fn the_preamble_is_the_manifest_s() {
        let t = table("");
        assert_eq!(
            expected_preamble(&t, "pa", Some(Path::new("docs/CHANGES.md"))),
            "# Changelog of pa\n\nThis file holds the entries of this crate only. The changelog of \
             the whole project is docs/CHANGES.md in its repository.\n\nCategories, in order: \
             Migration, New features, Workflow.\n\nVersions with no category and no class: 0.1.0."
        );
        let linked = table("assembled-url = \"https://example.org/c\"\n");
        assert!(
            expected_preamble(&linked, "pa", Some(Path::new("CHANGELOG.md")))
                .contains("is [CHANGELOG.md](https://example.org/c) in its repository.")
        );
        assert!(!expected_preamble(&t, "pa", None).contains("whole project"));
    }

    /// The claim, the preamble reads back: the categories in their order and the unclassed
    /// versions that `expected_preamble` writes are what `read_preamble` gives, with every line
    /// read, an assembled file named or not, its path opening with white space or not, linked or
    /// not, unclassed versions listed or not, a version repeated among them, and no category
    /// declared; and each line of no shape it writes is reported by its one-based line, a second
    /// list of the categories, a padded or a repeated item and a version that is no version among
    /// them. Mutations checked, each caught: in `read_preamble`, `unread.push(i as u32 + 1);` made
    /// `unread.push(i as u32);`; and in `listed`, `&& (repeats || !items[..i].contains(item))` made
    /// `&& !items[..i].contains(item)`.
    #[test]
    fn the_preamble_reads_back() {
        let declared = table("");
        let linked = table("assembled-url = \"https://example.org/c\"\n");
        let bare = ChangelogTable::with_no_manifest(vec!["Only".to_string()], Vec::new());
        let none = ChangelogTable::with_no_manifest(Vec::new(), Vec::new());
        let v = |t: &str| Version::parse(t).expect("a version");
        // A version repeated in `unclassed`, which the manifest's table admits.
        let repeated = ChangelogTable::with_no_manifest(Vec::new(), vec![v("0.1.0"), v("0.1.0")]);
        for t in [&declared, &linked, &bare, &none, &repeated] {
            // An assembled path opening with white space, which the manifest's table admits.
            for assembled in [
                None,
                Some(Path::new("docs/CHANGES.md")),
                Some(Path::new(" C.md")),
            ] {
                let text = expected_preamble(t, "pa", assembled);
                let read = read_preamble(&text);
                assert_eq!(read.unread, Vec::<u32>::new(), "{text}");
                assert_eq!(read.table.categories(), t.categories(), "{text}");
                assert_eq!(read.table.unclassed(), t.unclassed(), "{text}");
            }
        }
        let text = expected_preamble(&declared, "pa", None);
        assert_eq!(text.lines().count(), 7, "{text}");
        let bad = format!(
            "{text}\nAn extra line.\nCategories, in order: A.\n# Changelog of pb\n\
             This file holds the entries of this crate only. And more."
        );
        assert_eq!(read_preamble(&bad).unread, [8, 9, 10, 11]);
        assert_eq!(
            read_preamble(&bad).table.categories(),
            declared.categories(),
            "the first list stands"
        );
        for line in [
            "Categories, in order: A,  B.",
            "Categories, in order: A, A.",
            "Categories, in order: A, , B.",
            "Categories, in order: A,B.",
            "Categories, in order: A",
            "Versions with no category and no class: 0.1.",
            "# Changelog of ",
        ] {
            let read = read_preamble(&format!("# Changelog of pa\n\n{line}"));
            assert_eq!(read.unread, [3], "{line}");
        }
    }

    /// The claim: the first line of each kind is read where it has that shape whole, and every
    /// other line is unread: a second line of a kind, read or not the first, a package that is
    /// empty or opens with white space, an empty name of the whole project's changelog, the
    /// sentence naming it without its end, and a category holding a `:`; a line ending with white
    /// space is read, as the publisher's check admits on the preamble's last line; a version
    /// repeated among the unclassed is read. Mutations checked, each caught: in `read_preamble`,
    /// the guard `Some(k) if std::mem::replace(&mut seen[k], true) => false,` made
    /// `Some(_) if false => false,`; `.is_some_and(|name| !name.is_empty())` made `.is_some()`;
    /// `.and_then(|r| r.strip_suffix(WHOLE.1))` made
    /// `.map(|r| r.strip_suffix(WHOLE.1).unwrap_or(r))`; `Some(0) => named(&line[TITLE.len()..]),`
    /// made `Some(0) => true,`; `let line = line.trim_end();` removed; and in `listed`,
    /// `&& !item.contains([',', ':'])` made `&& !item.contains([','])`.
    #[test]
    fn each_kind_of_line_is_read_once_in_its_whole_shape() {
        let v = |t: &str| Version::parse(t).expect("a version");
        let own = "This file holds the entries of this crate only.";
        let versions = "Versions with no category and no class:";
        for (text, unread, unclassed) in [
            (
                format!("# Changelog of pa\n\n{own}\n{own}"),
                vec![4],
                vec![],
            ),
            (
                format!("{own} The changelog of the whole project is  in its repository."),
                vec![1],
                vec![],
            ),
            (
                format!("{versions} 0.1.0.\n{versions} 0.2.0."),
                vec![2],
                vec![v("0.1.0")],
            ),
            (
                format!("{versions} 0.1.\n{versions} 0.2.0."),
                vec![1, 2],
                vec![],
            ),
            (
                format!("{versions} 0.1.0, 0.1.0."),
                vec![],
                vec![v("0.1.0"), v("0.1.0")],
            ),
            (
                format!("{own} The changelog of the whole project is CHANGES.md"),
                vec![1],
                vec![],
            ),
            (format!("{versions} 0.1.0.   \t"), vec![], vec![v("0.1.0")]),
            ("Categories, in order: A: B.".to_string(), vec![1], vec![]),
            ("# Changelog of  pa".to_string(), vec![1], vec![]),
            ("# Changelog of".to_string(), vec![1], vec![]),
        ] {
            let read = read_preamble(&text);
            assert_eq!(read.unread, unread, "{text}");
            assert_eq!(read.table.unclassed(), unclassed, "{text}");
        }
    }

    /// The claim: over crates each placed by its own table, an entry its own table does not admit
    /// is left out, and a version one crate's table lists as unclassed has no category level for
    /// that crate alone, its block before the categories; the categories are those given, in
    /// their order. Mutation checked: in `assembled_body_by_table`, the unclassed blocks,
    /// `if is_unclassed(table, version) {`, made `if false {`; caught.
    #[test]
    fn each_crate_is_placed_by_its_own_table() {
        let v = |t: &str| Version::parse(t).expect("a version");
        let a_table = ChangelogTable::with_no_manifest(vec!["A".to_string()], Vec::new());
        let b_table = ChangelogTable::with_no_manifest(vec!["B".to_string()], vec![v("0.1.0")]);
        let a = parse(
            "# Changelog of a\n\n## 0.2.0\n\n- A, minor: a two.\n- B, minor: a lists no B.\n\n\
             ## 0.1.0\n\n- A, minor: a one.\n",
        );
        let b =
            parse("# Changelog of b\n\n## 0.2.0\n\n- B, patch: b two.\n\n## 0.1.0\n\n- b one\n");
        let categories = ["A".to_string(), "B".to_string()];
        let text = assembled_body_by_table(
            &categories,
            &[("a", "a", &a_table, &a), ("b", "b", &b_table, &b)],
        );
        assert_eq!(
            text,
            "\n## 0.2.0\n\n### A\n\n#### a\n\n- minor: a two.\n\n### B\n\n#### b\n\n- patch: b two.\n\
             \n## 0.1.0\n\n#### b\n\n- b one\n\n### A\n\n#### a\n\n- minor: a one.\n"
        );
        assert_eq!(unplaced(&a_table, "a", &a.sections[0]), [6]);
    }

    /// The claim: `omitted` names the first entry the assembled file has no place for, in the order
    /// of the file. Mutation checked: in `omitted`, `.first()` made `.last()`; caught.
    #[test]
    fn the_first_entry_with_no_place_is_omitted() {
        let t = table("");
        let expected = expected_preamble(&t, "pa", None);
        let log = parse(&format!(
            "{expected}\n\n## Next release\n\n- Other, patch: first.\n- Other, patch: second.\n"
        ));
        let first = log.sections[0].entries[0].line;
        assert_eq!(
            omitted(&t, "a", &expected, &log),
            Some(Omitted::Entry(first))
        );
    }
}
