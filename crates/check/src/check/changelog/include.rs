//! The `include` check: a changelog crate's package holds its own `CHANGELOG.md`, as Cargo lists
//! the files of a package, through `cargo-reader`'s match of the patterns, per
//! `design@knowledge-architect@tools-read-cargo-through-the-crate`.
//!
//! Cargo packages the file `readme` or `license-file` names whatever the patterns say, reads
//! `exclude` only where `include` is unset or empty, and lists no file of a package one of whose
//! patterns it refuses, in either list, as `path@cargo-reader@docs/grounding/cargo-manifest.md`
//! records measured; so each of these reads the same way here.

use knowledge_architect_cargo_reader::{exclude_matches, include_matches};

use crate::model::changelog::{Crate, CHANGELOG_FILE};
use crate::model::Finding;

/// A finding for each changelog crate whose package holds no `CHANGELOG.md` of its own, or whose
/// patterns Cargo refuses.
pub(super) fn check(crates: &[Crate]) -> Vec<Finding> {
    crates.iter().filter_map(finding).collect()
}

fn finding(c: &Crate) -> Option<Finding> {
    let changelog = c.changelog();
    let named = |path: &Option<std::path::PathBuf>| path.as_ref() == Some(&changelog);
    let excluded = c
        .exclude
        .as_ref()
        .map(|patterns| exclude_matches(patterns, CHANGELOG_FILE));
    let included = c
        .include
        .as_ref()
        .filter(|patterns| !patterns.is_empty())
        .map(|patterns| include_matches(patterns, CHANGELOG_FILE));
    let refused = [&included, &excluded]
        .into_iter()
        .flatten()
        .find_map(|r| r.as_ref().err());
    if let Some(error) = refused {
        return Some(Finding::in_file(
            c.manifest(),
            format!(
                "`cargo package` lists no file of `{}`, since {error}, which Cargo refuses",
                c.package
            ),
            "correct the pattern: close each `{` and each `[` range, and escape no character at \
             its end",
        ));
    }
    if named(&c.readme) || named(&c.license_file) {
        return None;
    }
    match (included, excluded) {
        (Some(Ok(false)), _) => Some(Finding::in_file(
            c.manifest(),
            format!(
                "the `include` of `{}` packages no {CHANGELOG_FILE}, so the published crate ships \
                 no changelog of its own",
                c.package
            ),
            format!("add `\"/{CHANGELOG_FILE}\"` at the end of the crate's `include`"),
        )),
        (None, Some(Ok(true))) => Some(Finding::in_file(
            c.manifest(),
            format!(
                "the `exclude` of `{}` leaves out its {CHANGELOG_FILE}, so the published crate \
                 ships no changelog of its own",
                c.package
            ),
            format!(
                "remove the pattern that covers it, or add `\"!/{CHANGELOG_FILE}\"` at the end of \
                 the crate's `exclude`"
            ),
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn with(include: Option<&[&str]>, exclude: Option<&[&str]>) -> Crate {
        named_with("pa", include, exclude)
    }

    fn named_with(package: &str, include: Option<&[&str]>, exclude: Option<&[&str]>) -> Crate {
        let list = |p: &[&str]| p.iter().map(|s| s.to_string()).collect();
        Crate {
            component: "a".to_string(),
            dir: PathBuf::from("a"),
            package: package.to_string(),
            inherits_version: true,
            include: include.map(list),
            exclude: exclude.map(list),
            readme: None,
            license_file: None,
        }
    }

    fn found(crates: &[Crate]) -> Vec<String> {
        check(crates)
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    /// The claim: an `include` that matches no `CHANGELOG.md` is a finding at the crate's
    /// `Cargo.toml`, one that matches it is none, and no `include` is none, nor an empty one,
    /// which Cargo reads as none. Mutation checked: the arm `(Some(Ok(false)), _) =>` made
    /// `(Some(Ok(true)), _) =>`; caught.
    #[test]
    fn an_include_that_packages_no_changelog_is_a_finding() {
        assert_eq!(
            found(&[
                named_with("p1", Some(&["/src/**"]), None),
                named_with("p2", Some(&["/src/**", "/CHANGELOG.md"]), None),
                named_with("p3", None, None),
                named_with("p4", Some(&[]), None),
            ]),
            ["a/Cargo.toml  the `include` of `p1` packages no CHANGELOG.md, so the published crate ships no changelog of its own"]
        );
    }

    /// The claim: where `include` is unset or empty, an `exclude` that leaves out the
    /// `CHANGELOG.md` is a finding, one that keeps it is none, and an `exclude` beside an
    /// `include` that is not empty is not read. Mutations checked, each caught: the arm
    /// `(None, Some(Ok(true))) =>` made `(_, Some(Ok(true))) =>`; and `.filter(|patterns|
    /// !patterns.is_empty())` made `.filter(|_| true)`, so an empty `include` stops `exclude`
    /// from being read.
    #[test]
    fn an_exclude_that_leaves_out_the_changelog_is_a_finding() {
        assert_eq!(
            found(&[
                with(None, Some(&["/CHANGELOG.md"])),
                with(Some(&[]), Some(&["*.md"])),
                with(None, Some(&["*.md", "!/CHANGELOG.md"])),
                with(Some(&["/CHANGELOG.md"]), Some(&["/CHANGELOG.md"])),
            ]),
            [
                "a/Cargo.toml  the `exclude` of `pa` leaves out its CHANGELOG.md, so the published crate ships no changelog of its own",
                "a/Cargo.toml  the `exclude` of `pa` leaves out its CHANGELOG.md, so the published crate ships no changelog of its own",
            ]
        );
    }

    /// The claim: the file `readme` or `license-file` names is packaged whatever the patterns
    /// say, and one that names another file packages no changelog. Mutation checked:
    /// `if named(&c.readme) || named(&c.license_file) {` made `if named(&c.readme) {`; caught.
    #[test]
    fn a_changelog_named_as_readme_or_license_file_is_packaged() {
        let readme = Crate {
            readme: Some(PathBuf::from("a/CHANGELOG.md")),
            ..with(Some(&["/src/**"]), None)
        };
        let license = Crate {
            license_file: Some(PathBuf::from("a/CHANGELOG.md")),
            ..with(None, Some(&["/CHANGELOG.md"]))
        };
        let elsewhere = Crate {
            readme: Some(PathBuf::from("CHANGELOG.md")),
            ..with(Some(&["/src/**"]), None)
        };
        assert_eq!(
            found(&[readme, license, elsewhere]),
            ["a/Cargo.toml  the `include` of `pa` packages no CHANGELOG.md, so the published crate ships no changelog of its own"]
        );
    }

    /// The claim: a pattern Cargo refuses, in `include` or in `exclude`, an `exclude` beside an
    /// `include` included, is a finding naming it, since Cargo then packages nothing. Mutation
    /// checked: `[&included, &excluded]` made `[&included]`; caught.
    #[test]
    fn a_pattern_cargo_refuses_is_a_finding() {
        assert_eq!(
            found(&[
                with(Some(&["{CHANGELOG.md"]), None),
                with(Some(&["/CHANGELOG.md"]), Some(&["{x"])),
            ]),
            [
                "a/Cargo.toml  `cargo package` lists no file of `pa`, since the pattern `{CHANGELOG.md` opens with `{` a group it never closes, which Cargo refuses",
                "a/Cargo.toml  `cargo package` lists no file of `pa`, since the pattern `{x` opens with `{` a group it never closes, which Cargo refuses",
            ]
        );
    }
}
