//! The changelog checks, opt-in: each finding over the changelog crates, their `Cargo.toml` and
//! their `CHANGELOG.md`, all of the last phase.
//!
//! The parse and every rendering are `model`'s, and raise nothing, so a tool that asks for a
//! complete model answers over a malformed entry; what they record is judged here. A project whose
//! manifest declares no `[changelog]` is asked nothing. The bump check and the `include` check are
//! modules of their own, since each reads an external reference, Cargo's semver and Cargo's
//! `include` patterns.

mod bump;
mod include;

use crate::model::changelog::{
    expected_preamble, parse, CargoTrouble, Changelog, Class, Crate, Heading, CHANGELOG_FILE,
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
    out.extend(bump::check(table, &read));
    out.extend(include::check(crates.all()));
    out
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

/// The findings of the grammar over one crate's changelog: each unread line, each section out of
/// order or with no entry, and each entry whose tag the version or the manifest refuses.
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
    let mut previous: Option<Heading> = None;
    for section in &log.sections {
        let name = match section.heading {
            Heading::NextRelease => crate::model::changelog::NEXT_RELEASE.to_string(),
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
