//! The `include` check: a changelog crate whose `Cargo.toml` sets `include` packages its
//! `CHANGELOG.md`, as Cargo reads the patterns, through `cargo-reader`'s match, per
//! `design@knowledge-architect@tools-read-cargo-through-the-crate`.

use crate::model::changelog::{Crate, CHANGELOG_FILE};
use crate::model::Finding;

/// A finding for each changelog crate whose `include` packages no `CHANGELOG.md`; a crate that
/// sets no `include` packages every file Cargo does not leave out, and is judged nothing.
pub(super) fn check(crates: &[Crate]) -> Vec<Finding> {
    crates
        .iter()
        .filter(|c| {
            c.include.as_ref().is_some_and(|patterns| {
                !knowledge_architect_cargo_reader::include_matches(patterns, CHANGELOG_FILE)
            })
        })
        .map(|c| {
            Finding::in_file(
                c.manifest(),
                format!(
                    "the `include` of `{}` packages no {CHANGELOG_FILE}, so the published crate \
                     ships no changelog",
                    c.package
                ),
                format!("add `\"/{CHANGELOG_FILE}\"` to the crate's `include`"),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn with(include: Option<&[&str]>) -> Crate {
        Crate {
            component: "a".to_string(),
            dir: PathBuf::from("a"),
            package: "pa".to_string(),
            inherits_version: true,
            include: include.map(|p| p.iter().map(|s| s.to_string()).collect()),
        }
    }

    /// The claim, claim 7: an `include` that matches no `CHANGELOG.md` is a finding at the
    /// crate's `Cargo.toml`, one that matches it is none, and no `include` is none. Mutation
    /// checked: `c.include.as_ref().is_some_and(` made `c.include.as_ref().is_none_or(`; caught.
    #[test]
    fn an_include_that_packages_no_changelog_is_a_finding() {
        let found = check(&[
            with(Some(&["/src/**"])),
            with(Some(&["/src/**", "/CHANGELOG.md"])),
            with(None),
        ]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].file, PathBuf::from("a/Cargo.toml"));
    }
}
