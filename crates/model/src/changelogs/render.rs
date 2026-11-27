//! The texts the manifest determines: a crate's preamble, and the assembled file of the whole
//! project, rendered from the parsed changelogs, with the rule of what the assembled file omits.
//! Nothing here writes, as nothing in `crate::index` does.
//!
//! A module of its own, not `crate::index`, whose reference is the register indexes' heads. The
//! rule of what the assembled file omits is here, beside the layout that applies it, since every
//! change to one is read with the other; the writers' refusal, which reads that rule beside the
//! heads on what a writer may lose, is `super::refusal`'s.

use std::path::{Path, PathBuf};

use super::{parse, Changelog, ChangelogTable, Entry, Heading, Tag, Version};
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
/// The layout, per `design@model@assembled-changelog-layout`: the generated header; then each
/// version, `Next release` first and the released ones in descending order, as `## <version>`;
/// under it each category in its declared order, as `### <category>`; under it each crate, as
/// `#### <package>`; under it the crate's entries of that category, `- <class>: <text>`, in the
/// order of its file. A version `unclassed` lists has no category level: `#### <package>`, then
/// `- <text>`. An empty level is omitted.
///
/// It omits a line the parse could not read, an entry with no tag in a classed version, and an
/// entry whose category the manifest does not admit for its crate: a caller that must lose
/// nothing, as a writer must, asks [`refusal`](super::refusal) first, which refuses over each.
pub fn assembled_text(
    table: &ChangelogTable,
    crates: &[(&str, &str, &Changelog)],
    command: &str,
) -> String {
    let mut versions: Vec<Heading> = Vec::new();
    for (_, _, log) in crates {
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
    let mut out = format!("{GENERATED_HEADER} `{command} index`\n");
    for version in versions {
        let unclassed = is_unclassed(table, version);
        let entries = |component: &str, log: &Changelog, category: Option<&str>| -> Vec<String> {
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
        if unclassed {
            for (component, package, log) in crates {
                body.push_str(&block(package, &entries(component, log, None)));
            }
        } else {
            for category in table.categories() {
                let mut under = String::new();
                for (component, package, log) in crates {
                    under.push_str(&block(package, &entries(component, log, Some(category))));
                }
                if !under.is_empty() {
                    body.push_str(&format!("\n### {category}\n{under}"));
                }
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
/// **The one rule of what the assembled file keeps**, which [`assembled_text`] writes by and
/// [`omitted`] reads, so a writer never drops a line its refusal let through, per
/// `design@model@assembled-file-not-overwritten`.
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

/// A line of a crate's changelog the assembled file omits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Omitted {
    /// A line the parse could not read, at this one-based line.
    Unread(u32),
    /// An entry the assembled file has no place for, at its first line.
    Entry(u32),
}

/// The first line of `component`'s changelog `log` that the assembled file omits, or `None`
/// where it omits nothing: an unread line first, then each entry [`place`] has no place for.
pub(crate) fn omitted(table: &ChangelogTable, component: &str, log: &Changelog) -> Option<Omitted> {
    if let Some(unread) = log.unread.first() {
        return Some(Omitted::Unread(unread.line));
    }
    log.sections.iter().find_map(|section| {
        section
            .entries
            .iter()
            .find(|e| place(table, component, section.heading, e).is_none())
            .map(|e| Omitted::Entry(e.line))
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
}
