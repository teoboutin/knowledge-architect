//! The changelog checks, opt-in: each finding over the changelog crates, their `Cargo.toml` and
//! their `CHANGELOG.md`, all of the last phase.
//!
//! The parse and every rendering are `model`'s, and raise nothing, so a tool that asks for a
//! complete model answers over a malformed entry; what they record is judged here. A project whose
//! manifest declares no `[changelog]` is asked nothing, per `design@check@changelog-opt-in`. The
//! bump check and the `include` check are
//! modules of their own, since each reads an external reference, Cargo's semver and Cargo's
//! `include` patterns.

mod bump;
mod include;

use crate::model::changelog::{
    expected_preamble, parse, CargoTrouble, Changelog, Class, Crate, Heading, CHANGELOG_FILE,
    GENERATED_HEADER, NEXT_RELEASE,
};
use crate::model::declared::ChangelogTable;
use crate::model::{slashed, Finding, Manifest, Model};

use super::Inputs;

/// Every changelog finding, or none where the manifest declares no `[changelog]`.
pub(crate) fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Vec<Finding> {
    let Some(table) = manifest.changelog() else {
        return Vec::new();
    };
    let crates = model.changelog_crates();
    let assembled = crates.assembled(manifest);
    let mut out: Vec<Finding> = crates.trouble().iter().map(trouble).collect();
    let mut read: Vec<(&Crate, Changelog)> = Vec::new();
    for crate_ in crates.all() {
        let rel = crate_.changelog();
        let expected = expected_preamble(table, &crate_.package, assembled);
        match model.documents().iter().find(|d| d.rel == rel) {
            Some(doc) => {
                let log = parse(&doc.text);
                grammar(table, crate_, &log, &mut out);
                if log.preamble.trim_end() != expected {
                    out.push(Finding::at(
                        &rel,
                        1,
                        "the preamble, the lines above the first version heading, differs from \
                         the one the manifest determines",
                        format!(
                            "replace the lines above the first version heading with these, which \
                             `check --fix` does not write, since the file is hand-written:\n\n{expected}\n"
                        ),
                    ));
                }
                read.push((crate_, log));
            }
            None if assembled == Some(rel.as_path()) => out.push(Finding::in_file(
                &rel,
                format!(
                    "the assembled changelog's path is the changelog of `{}`",
                    crate_.component
                ),
                "set `[changelog] assembled` to a path no crate's changelog takes",
            )),
            None => match crate::model::listing::keeping_row(&rel, manifest.walk())
                .filter(|_| inputs.present.contains(&rel))
            {
                Some(row) => out.push(Finding::in_file(
                    &rel,
                    format!(
                        "{row} keeps out the changelog of `{}`, which the changelog checks read",
                        crate_.component
                    ),
                    "remove the row, or narrow it so it no longer covers the file: a crate's \
                     changelog is read for its structure",
                )),
                None => out.push(Finding::in_file(
                    &rel,
                    format!(
                        "the changelog crate `{}` holds no {CHANGELOG_FILE}",
                        crate_.component
                    ),
                    format!(
                        "write it, opening with the preamble the manifest determines:\n\n{expected}\n"
                    ),
                )),
            },
        }
    }
    if assembled.is_none() {
        out.extend(stale_assembled(table, crates.all(), model));
    }
    out.extend(bump::check(table, crates.all(), &read));
    out.extend(include::check(crates.all()));
    out
}

/// The finding of a file at the declared assembled path that opens with the generated header
/// where no file is assembled, with fewer than two changelog crates: no writer writes it again and
/// no other check compares it, so its stale text would stand judged by nothing, per
/// `design@check@generated-header-at-an-unassembled-path`. A crate's own changelog at that path is
/// judged by the grammar instead.
fn stale_assembled(table: &ChangelogTable, crates: &[Crate], model: &Model) -> Option<Finding> {
    let path = table.assembled();
    if crates.iter().any(|c| c.changelog() == path) {
        return None;
    }
    let doc = model.documents().iter().find(|d| d.rel == path)?;
    doc.text
        .trim_start_matches('\u{feff}')
        .starts_with(GENERATED_HEADER)
        .then(|| {
            Finding::in_file(
                path,
                "the file opens with the generated header, and no changelog is assembled, since \
                 the project has fewer than two changelog crates",
                "delete the file: the assembled changelog is generated only where two changelog \
                 crates or more are read",
            )
        })
}

/// The finding of a `Cargo.toml` the read could not use.
fn trouble(trouble: &CargoTrouble) -> Finding {
    match trouble {
        CargoTrouble::Unreadable { file, reason } => Finding::in_file(
            file,
            format!(
                "the changelog checks cannot read `{}`: {reason}",
                slashed(file)
            ),
            "repair the manifest as the reason says; until then its Component is judged no \
             changelog crate",
        ),
        CargoTrouble::KeptOut { file, row } => Finding::in_file(
            file,
            format!(
                "{row} keeps out `{}`, which the changelog checks read",
                slashed(file)
            ),
            "remove the row, or narrow it so it no longer covers the file",
        ),
        CargoTrouble::Missing { file } => Finding::in_file(
            file,
            format!(
                "`{}` is not there, and [changelog] components lists its Component",
                slashed(file)
            ),
            "give the Component its crate's Cargo.toml, or remove it from [changelog] components",
        ),
        CargoTrouble::NoPackage { file } => Finding::in_file(
            file,
            format!(
                "`{}` declares no package, and [changelog] components lists its Component",
                slashed(file)
            ),
            "declare the crate's `[package]`, or remove the Component from [changelog] components",
        ),
    }
}

/// The findings of the grammar over one crate's changelog: each unread line, a missing
/// `## Next release`, each section out of order or with no entry, and each entry whose tag the
/// version or the manifest refuses.
fn grammar(table: &ChangelogTable, crate_: &Crate, log: &Changelog, out: &mut Vec<Finding>) {
    let rel = crate_.changelog();
    for unread in &log.unread {
        out.push(Finding::at(
            &rel,
            unread.line,
            "the line is no entry, no version heading and no continuation of an entry",
            "write it as an entry, `- <category>, <class>: <text>`, indent it under the entry it \
             continues, or remove it; an entry's category is its first tag, so a crate's changelog \
             holds no subsection heading",
        ));
    }
    if !log
        .sections
        .iter()
        .any(|s| s.heading == Heading::NextRelease)
    {
        out.push(Finding::at(
            &rel,
            log.sections.first().map_or(1, |s| s.line),
            format!("the changelog holds no `## {NEXT_RELEASE}`"),
            format!(
                "write `## {NEXT_RELEASE}` below the preamble, above the first version heading: it \
                 may hold no entry"
            ),
        ));
    }
    let mut previous: Option<Heading> = None;
    for section in &log.sections {
        let name = match section.heading {
            Heading::NextRelease => NEXT_RELEASE.to_string(),
            Heading::Version(v) => v.to_string(),
        };
        let in_order = match (previous, section.heading) {
            (None, _) => true,
            (Some(Heading::NextRelease), Heading::Version(_)) => true,
            (Some(Heading::Version(p)), Heading::Version(v)) => v < p,
            (_, Heading::NextRelease) => false,
        };
        if !in_order {
            out.push(Finding::at(
                &rel,
                section.line,
                format!("the section `## {name}` is out of order"),
                "order the sections: `## Next release` first, then each released version once, \
                 in descending order",
            ));
        }
        previous = Some(section.heading);
        let unclassed =
            matches!(section.heading, Heading::Version(v) if table.unclassed().contains(&v));
        if section.entries.is_empty() && section.heading != Heading::NextRelease {
            out.push(Finding::at(
                &rel,
                section.line,
                format!("the section `## {name}` holds no entry"),
                "remove the heading: a crate's changelog holds a section for each version with an \
                 entry in that crate alone",
            ));
        }
        if unclassed {
            continue;
        }
        for entry in &section.entries {
            let Some(tag) = &entry.tag else {
                out.push(Finding::at(
                    &rel,
                    entry.line,
                    format!(
                        "the entry carries no category and no class, and `{name}` is no version \
                         [changelog] unclassed lists"
                    ),
                    "open it `- <category>, <class>: `, a declared category and one of patch, \
                     minor and major",
                ));
                continue;
            };
            if !table.categories().contains(&tag.category) {
                out.push(Finding::at(
                    &rel,
                    entry.line,
                    format!(
                        "`{}` is no category [changelog] categories declares",
                        tag.category
                    ),
                    format!(
                        "tag the entry with a declared category, {}, or declare it in \
                         [changelog] categories",
                        table.categories().join(", ")
                    ),
                ));
            } else if !table.admits(&tag.category, &crate_.component) {
                let to = table.restricted_to(&tag.category).unwrap_or_default();
                out.push(Finding::at(
                    &rel,
                    entry.line,
                    format!(
                        "the category `{}` is restricted to {} by [changelog.category-components], \
                         and this is the changelog of `{}`",
                        tag.category,
                        to.iter()
                            .map(|c| format!("`{c}`"))
                            .collect::<Vec<_>>()
                            .join(", "),
                        crate_.component
                    ),
                    "move the entry to the changelog of a Component its category admits, or add \
                     this Component to the category's row",
                ));
            }
            if Class::parse(&tag.class).is_none() {
                out.push(Finding::at(
                    &rel,
                    entry.line,
                    format!(
                        "`{}` is no class: the classes are patch, minor and major",
                        tag.class
                    ),
                    "write the class as one of patch, minor and major, Cargo's semver classes",
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};

    /// The preamble of the crate `package` under the table of `findings_over`, with no file
    /// assembled.
    fn preamble(package: &str) -> String {
        format!(
            "# Changelog of {package}\n\nThis file holds the entries of this crate only.\n\n\
             Categories, in order: M."
        )
    }

    /// The changelog findings of a project of the Components `a` and `b`, its manifest's
    /// `[changelog]` table holding `table` beside `categories = ["M"]`, each file of `files`
    /// present and each Markdown one walked, but those `skip` lists; each finding as
    /// `<location>  <what>`.
    fn findings_over(table: &str, skip: &[&str], files: &[(&str, &str)]) -> Vec<String> {
        let skips = skip
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            &format!(
                "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\"]\n\n\
                 [walk]\nskip-dirs = []\nskip-files = [{skips}]\n\n[changelog]\ncategories = [\"M\"]\n{table}"
            ),
        )
        .expect("a declaration");
        let texts: HashMap<PathBuf, String> = files
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let listing: Vec<PathBuf> = texts.keys().cloned().collect();
        let crates = crate::model::changelog::changelog_crates(&manifest, &listing, &|rel| {
            texts
                .get(rel)
                .cloned()
                .ok_or_else(|| std::io::ErrorKind::NotFound.into())
        });
        let walked = texts
            .iter()
            .filter(|(p, _)| p.extension().is_some_and(|e| e == "md"))
            .filter(|(p, _)| !skip.iter().any(|s| Path::new(s) == p.as_path()))
            .map(|(p, t)| (p.clone(), t.clone()))
            .collect();
        let mut model = Model::from_documents(walked);
        model.set_changelog_crates(crates);
        let present: HashSet<PathBuf> = listing.iter().cloned().collect();
        let inputs = Inputs {
            committed: &HashMap::new(),
            configs: &HashMap::new(),
            configs_not_text: &HashSet::new(),
            generated_unreadable: &HashSet::new(),
            present: &present,
            directories: &HashSet::new(),
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        let mut found: Vec<String> = check(&model, &manifest, &inputs)
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect();
        found.sort();
        found
    }

    const PUBLISHED_A: &str = "[package]\nname = \"pa\"\nversion = \"0.1.0\"\n";

    /// The findings over the one crate `a`, whose changelog holds `body` below its preamble.
    fn grammar_of(body: &str) -> Vec<String> {
        let log = format!("{}\n\n{body}", preamble("pa"));
        findings_over(
            "",
            &[],
            &[("a/Cargo.toml", PUBLISHED_A), ("a/CHANGELOG.md", &log)],
        )
    }

    /// The claim: a changelog with no `## Next release` is a finding at its first version heading,
    /// and an empty one is none, nor a finding of a section with no entry. Mutations checked, each
    /// caught: in `grammar`, `.any(|s| s.heading == Heading::NextRelease)` made `.any(|s| s.heading
    /// == Heading::NextRelease || true)`; and `if section.entries.is_empty() && section.heading !=
    /// Heading::NextRelease {` made `if section.entries.is_empty() {`.
    #[test]
    fn next_release_is_required_and_may_be_empty() {
        assert_eq!(
            grammar_of("## 0.1.0\n\n- M, minor: the first\n"),
            ["a/CHANGELOG.md:7  the changelog holds no `## Next release`"]
        );
        assert!(
            grammar_of("## Next release\n\n## 0.1.0\n\n- M, minor: the first\n").is_empty(),
            "an empty Next release is no finding"
        );
    }

    /// The claim: a `## Next release` after a version heading, or a second one, is out of order,
    /// and a version written twice is out of order at its second heading, since each released
    /// version is written once. Mutations checked, each caught: in `grammar`,
    /// `(_, Heading::NextRelease) => false,` made `=> true,`; and
    /// `(Some(Heading::Version(p)), Heading::Version(v)) => v < p,` made `=> v <= p,`.
    #[test]
    fn a_next_release_after_a_version_and_a_version_twice_are_out_of_order() {
        assert_eq!(
            grammar_of(
                "## Next release\n\n- M, patch: a fix\n\n## 0.1.0\n\n- M, minor: the first\n\n\
                 ## Next release\n\n- M, patch: another\n"
            ),
            ["a/CHANGELOG.md:15  the section `## Next release` is out of order"]
        );
        assert_eq!(
            grammar_of(
                "## Next release\n\n## 0.2.0\n\n- M, minor: an addition\n\n## 0.2.0\n\n- M, minor: again\n"
            ),
            ["a/CHANGELOG.md:13  the section `## 0.2.0` is out of order"]
        );
    }

    /// The claim: an assembled path that is a crate's changelog, where that file is not there, is
    /// a finding saying so, never that the crate holds no changelog. Mutation checked: in
    /// `check`, `None if assembled == Some(rel.as_path()) => out.push(Finding::in_file(` made
    /// `None if false => out.push(Finding::in_file(`; caught.
    #[test]
    fn an_assembled_path_that_is_a_crate_s_changelog_is_named_as_such() {
        let b_log = format!("{}\n\n## Next release\n", preamble("pb"));
        let found = findings_over(
            "assembled = \"a/CHANGELOG.md\"\n",
            &[],
            &[
                ("a/Cargo.toml", PUBLISHED_A),
                (
                    "b/Cargo.toml",
                    "[package]\nname = \"pb\"\nversion = \"0.1.0\"\n",
                ),
                ("b/CHANGELOG.md", &b_log),
            ],
        );
        assert!(
            found.contains(
                &"a/CHANGELOG.md  the assembled changelog's path is the changelog of `a`"
                    .to_string()
            ),
            "{found:#?}"
        );
        assert!(
            !found.iter().any(|f| f.contains("holds no CHANGELOG.md")),
            "{found:#?}"
        );
    }

    /// The claim: a `Cargo.toml` that a `[walk]` row keeps out, a listed Component with no
    /// `Cargo.toml`, and one whose `Cargo.toml` declares no package are each a finding naming the
    /// file and what keeps it from being read. Mutations checked, each caught: in `trouble`, the
    /// message of `KeptOut`, of `Missing` and of `NoPackage` each made `String::new()`.
    #[test]
    fn a_cargo_toml_the_checks_cannot_use_is_a_finding_naming_why() {
        assert_eq!(
            findings_over(
                "",
                &["a/Cargo.toml"],
                &[("a/Cargo.toml", PUBLISHED_A)],
            ),
            ["a/Cargo.toml  [walk] skip-files `a/Cargo.toml` keeps out `a/Cargo.toml`, which the changelog checks read"]
        );
        assert_eq!(
            findings_over(
                "components = [\"a\", \"b\"]\n",
                &[],
                &[("b/Cargo.toml", "[workspace]\n")],
            ),
            [
                "a/Cargo.toml  `a/Cargo.toml` is not there, and [changelog] components lists its Component",
                "b/Cargo.toml  `b/Cargo.toml` declares no package, and [changelog] components lists its Component",
            ]
        );
    }

    /// The claim: with fewer than two changelog crates, a walked file at the declared assembled
    /// path that opens with the generated header is a finding naming its deletion, and a
    /// hand-written one is none, nor a crate's own changelog at that path, which the grammar
    /// judges. Mutations checked, each caught: in `check`, `if assembled.is_none() {` made
    /// `if false {`; and in `stale_assembled`, `if crates.iter().any(|c| c.changelog() == path)
    /// {` made `if false {`.
    #[test]
    fn a_generated_file_at_the_assembled_path_with_nothing_assembled_is_a_finding() {
        let a_log = format!("{}\n\n## Next release\n", preamble("pa"));
        let files = |all: &str| {
            findings_over(
                "",
                &[],
                &[
                    ("a/Cargo.toml", PUBLISHED_A),
                    ("a/CHANGELOG.md", &a_log),
                    ("CHANGELOG.md", all),
                ],
            )
        };
        assert_eq!(
            files(&format!("{GENERATED_HEADER} `klarch index`\n\n## Next release\n")),
            ["CHANGELOG.md  the file opens with the generated header, and no changelog is assembled, since the project has fewer than two changelog crates"]
        );
        assert!(files("# Changes\n\nWritten by hand.\n").is_empty());
        let own = findings_over(
            "assembled = \"a/CHANGELOG.md\"\n",
            &[],
            &[
                ("a/Cargo.toml", PUBLISHED_A),
                (
                    "a/CHANGELOG.md",
                    &format!("{GENERATED_HEADER} `klarch index`\n\n## Next release\n"),
                ),
            ],
        );
        assert!(
            !own.iter()
                .any(|f| f.contains("opens with the generated header")),
            "{own:#?}"
        );
    }
}
