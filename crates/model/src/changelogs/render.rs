//! The texts the manifest determines: a crate's preamble, and the assembled file of the whole
//! project, rendered from the parsed changelogs. Nothing here writes, as nothing in
//! `crate::index` does.
//!
//! A module of its own, not `crate::index`, whose reference is the register indexes' heads.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::{parse, Changelog, ChangelogTable, Heading, Version};
use crate::finding::slashed;
use crate::manifest::Manifest;
use crate::model::Model;

/// The opening of every file this tool generates, as an `index.md` opens: the assembled file is
/// told apart from a hand-written one at its path by it.
pub const GENERATED_HEADER: &str = "**Generated — do not edit.**";

/// The preamble a crate's changelog opens with, as the manifest determines it: the package's
/// name, that the file holds that crate's entries only, the assembled file named in words where
/// one is assembled, with its link where `assembled-url` sets one, the categories in their
/// order, and the `unclassed` versions where any.
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

/// The assembled file's text over the changelogs of `crates`, each a package name and its parse,
/// in the manifest's order of the Components.
///
/// The layout: the generated header; then each version, `Next release` first and the released
/// ones in descending order, as `## <version>`; under it each category in its declared order, as
/// `### <category>`; under it each crate, as `#### <package>`; under it the crate's entries of
/// that category, `- <class>: <text>`, in the order of its file. A version `unclassed` lists has
/// no category level: `#### <package>`, then `- <text>`. An empty level is omitted, and an entry
/// whose category `admitted` refuses is left out, which `refusal` refuses to write.
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
        let unclassed = matches!(version, Heading::Version(v) if table.unclassed().contains(&v));
        let entries = |component: &str, log: &Changelog, category: Option<&str>| -> Vec<String> {
            log.sections
                .iter()
                .filter(|s| s.heading == version)
                .flat_map(|s| &s.entries)
                .filter_map(|e| match (category, &e.tag) {
                    (None, _) => Some(format!("- {}", e.raw)),
                    (Some(c), Some(tag))
                        if tag.category == c && table.admits(&tag.category, component) =>
                    {
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

/// One crate's level: its heading and its entries, or nothing where it has none.
fn block(package: &str, entries: &[String]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    format!("\n#### {package}\n\n{}\n", entries.join("\n"))
}

/// The changelog of each changelog crate, parsed from the walked document at its path, with its
/// Component and its package; a crate whose file the walk does not hold has an empty one.
fn parsed(model: &Model) -> Vec<(String, String, Changelog)> {
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
/// manifest declares `[changelog]`, and the model reads two changelog crates or more.
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

/// Why no writer may write the assembled file, with the repair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    /// What stands in the way, naming the file and the line.
    pub why: String,
    /// The repair.
    pub repair: &'static str,
}

/// The repair of a hand-written file at the assembled file's path.
const HAND_WRITTEN_REPAIR: &str = "move the file away, or set `[changelog] assembled` to another \
     path, and run again; a file without the generated header is not this tool's to replace";

/// The repair of a line the assembled file would omit.
const OMITTED_REPAIR: &str = "repair the line as the changelog check reports it, and run again";

/// Whether a writer of the generated files, `index`, `index --staged` or `check --fix`, may write
/// the assembled file of the tree it writes, or why not, per
/// `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` and
/// `design@check@fix-before-the-checks`: one function, so the three refuse alike.
///
/// It refuses where a file at the path, in that tree, does not open with the generated header,
/// `committed` holding the tree's text of each generated path and `unreadable` each one whose
/// bytes are not text; and where a changelog holds an unread line, or an entry whose category
/// the manifest does not declare for its crate, an entry with no tag in a classed version
/// included, since the assembled file would omit it. Nothing is refused where nothing is
/// assembled.
pub fn refusal(
    manifest: &Manifest,
    model: &Model,
    committed: &HashMap<PathBuf, String>,
    unreadable: &HashSet<PathBuf>,
) -> Option<Refused> {
    let table = manifest.changelog()?;
    let path = model.changelog_crates().assembled(manifest)?;
    let hand_written = unreadable.contains(path)
        || committed.get(path).is_some_and(|text| {
            !text
                .trim_start_matches('\u{feff}')
                .starts_with(GENERATED_HEADER)
        });
    if hand_written {
        return Some(Refused {
            why: format!(
                "{}: the file at the assembled changelog's path does not open with the generated \
                 header, so writing it would lose what it holds. Nothing was written.",
                slashed(path)
            ),
            repair: HAND_WRITTEN_REPAIR,
        });
    }
    for (crate_, (component, _, log)) in model.changelog_crates().all().iter().zip(parsed(model)) {
        let rel = slashed(&crate_.changelog());
        if let Some(unread) = log.unread.first() {
            return Some(Refused {
                why: format!(
                    "{rel}:{}: the line is no entry, no version heading and no continuation of an \
                     entry, and the assembled changelog would omit it. Nothing was written.",
                    unread.line
                ),
                repair: OMITTED_REPAIR,
            });
        }
        for section in &log.sections {
            let unclassed =
                matches!(section.heading, Heading::Version(v) if table.unclassed().contains(&v));
            if unclassed {
                continue;
            }
            for entry in &section.entries {
                let admitted = entry
                    .tag
                    .as_ref()
                    .is_some_and(|t| table.admits(&t.category, &component));
                if !admitted {
                    return Some(Refused {
                        why: format!(
                            "{rel}:{}: the entry's category is none the manifest declares for \
                             `{component}`, and the assembled changelog would omit it. Nothing \
                             was written.",
                            entry.line
                        ),
                        repair: OMITTED_REPAIR,
                    });
                }
            }
        }
    }
    None
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

    /// The claim, claim 9's layout: version, then category in its declared order, then crate in
    /// the given order, an empty level omitted, and an unclassed version with no category level.
    /// Mutations checked, each caught: in `assembled_text`, the sort's `=> y.cmp(x),` made
    /// `=> x.cmp(y),`; and an unclassed version's entries, `(None, _) => Some(format!("- {}", e.raw)),`,
    /// made `(None, _) => None,`.
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

    /// The claim, claim 11: the assembled text, read back per crate, yields every entry of every
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

    /// The claim, claim 8's expected text: the package, the assembled path named in words, its
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
