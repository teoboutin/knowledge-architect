//! The bump check: each released version follows the next lower one by its highest class, as
//! Cargo's semver reads the three numbers, `path@check@docs/grounding/cargo-semver.md`.
//!
//! **It reads the tree alone**: the versions are those the changelogs hold, and no history is
//! read. Under the lockstep, where every changelog crate sets `version.workspace = true`, the
//! versions are those every crate's file holds together, and a version's class is the highest
//! among its entries over every crate, so a crate's file may skip a version; otherwise each
//! crate's file is judged alone. A version `[changelog] unclassed` lists, and the oldest, are not
//! judged, and `Next release` carries no version.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::model::changelog::{Changelog, Class, Crate, Heading, Version};
use crate::model::declared::ChangelogTable;
use crate::model::Finding;

/// One version over the files judged together: its highest class, and where its heading is
/// first met, the first crate in the manifest's order that holds it.
struct Released {
    class: Option<Class>,
    file: PathBuf,
    line: u32,
}

/// The bump findings over the changelogs read.
pub(super) fn check(table: &ChangelogTable, read: &[(&Crate, Changelog)]) -> Vec<Finding> {
    let lockstep = !read.is_empty() && read.iter().all(|(c, _)| c.inherits_version);
    let groups: Vec<Vec<&(&Crate, Changelog)>> = if lockstep {
        vec![read.iter().collect()]
    } else {
        read.iter().map(|r| vec![r]).collect()
    };
    let mut out = Vec::new();
    for group in groups {
        let mut versions: BTreeMap<Version, Released> = BTreeMap::new();
        for (crate_, log) in group {
            for section in &log.sections {
                let Heading::Version(version) = section.heading else {
                    continue;
                };
                let highest = section
                    .entries
                    .iter()
                    .filter_map(|e| e.tag.as_ref().and_then(|t| Class::parse(&t.class)))
                    .max();
                let released = versions.entry(version).or_insert(Released {
                    class: None,
                    file: crate_.changelog(),
                    line: section.line,
                });
                released.class = released.class.max(highest);
            }
        }
        let ordered: Vec<(&Version, &Released)> = versions.iter().collect();
        for pair in ordered.windows(2) {
            let ((previous, _), (version, released)) = (pair[0], pair[1]);
            if table.unclassed().contains(version) {
                continue;
            }
            let Some(class) = released.class else {
                continue;
            };
            let expected = previous.bumped(class);
            if *version != expected {
                out.push(Finding::at(
                    &released.file,
                    released.line,
                    format!(
                        "`{version}` follows `{previous}`, and the highest class of its entries, \
                         {}, gives `{expected}`",
                        class.word()
                    ),
                    if lockstep {
                        "give the release the version its highest class over every changelog \
                         crate implies, or correct the class of the entry that is wrong"
                    } else {
                        "give the release the version its highest class implies, or correct the \
                         class of the entry that is wrong"
                    },
                ));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::changelog::parse;
    use std::path::Path;

    fn table() -> ChangelogTable {
        let manifest = crate::model::Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n[changelog]\ncategories = [\"M\"]\nunclassed = [\"0.1.0\"]\n",
        )
        .expect("a declaration");
        manifest.changelog().expect("the table").clone()
    }

    fn crate_(component: &str, inherits_version: bool) -> Crate {
        Crate {
            component: component.to_string(),
            dir: PathBuf::from(component),
            package: format!("p{component}"),
            inherits_version,
            include: None,
        }
    }

    fn found(read: &[(&Crate, Changelog)]) -> Vec<String> {
        check(&table(), read)
            .iter()
            .map(|f| format!("{}  {}", f.location(), f.what))
            .collect()
    }

    const A: &str = "# a\n\n## 0.4.1\n\n- M, major: a breaks\n\n## 0.3.0\n\n- M, minor: a adds\n\n\
        ## 0.2.0\n\n- M, minor: a adds\n\n## 0.1.0\n\n- the first\n";
    const B: &str = "# b\n\n## 0.5.0\n\n- M, patch: b fixes\n\n## 0.3.0\n\n- M, patch: b fixes\n\n## 0.2.1\n\n- M, patch: b fixes\n\n## 0.1.0\n\n- the first\n";

    /// The claim, claim 5: under the lockstep, the versions of every file together, each judged by
    /// its highest class over every crate, under 0.x as Cargo reads it; a version that follows by
    /// less or by more is a finding, one that follows exactly is none, and a crate's file may skip
    /// a version. Over A and B: 0.2.0 by minor from 0.1.0, 0.2.1 by patch, 0.3.0 by minor; 0.4.1
    /// by major gives 0.4.0, less; 0.5.0 by patch gives 0.4.2, more. Mutations checked, each
    /// caught: `released.class = released.class.max(highest);` made `released.class = highest;`,
    /// so the last crate's class wins, which 0.3.0, minor in A and patch in B, shows; and
    /// `let lockstep = !read.is_empty() && ...` made `let lockstep = false;`.
    #[test]
    fn under_the_lockstep_every_file_is_judged_together() {
        let (a, b) = (crate_("a", true), crate_("b", true));
        let read = [(&a, parse(A)), (&b, parse(B))];
        assert_eq!(
            found(&read),
            [
                "a/CHANGELOG.md:3  `0.4.1` follows `0.3.0`, and the highest class of its entries, major, gives `0.4.0`",
                "b/CHANGELOG.md:3  `0.5.0` follows `0.4.1`, and the highest class of its entries, patch, gives `0.4.2`",
            ]
        );
    }

    /// The claim, claim 6: where a crate does not inherit the workspace version, each file is
    /// judged alone: B alone has 0.2.1 by patch from 0.1.0, which gives 0.1.1, then 0.3.0 by patch
    /// from 0.2.1, and 0.5.0 by patch from 0.3.0. Mutation checked:
    /// `read.iter().map(|r| vec![r]).collect()` made `vec![read.iter().collect()]`; caught.
    #[test]
    fn otherwise_each_file_is_judged_alone() {
        let (a, b) = (crate_("a", true), crate_("b", false));
        let read = [(&a, parse(A)), (&b, parse(B))];
        assert_eq!(
            found(&read),
            [
                "a/CHANGELOG.md:3  `0.4.1` follows `0.3.0`, and the highest class of its entries, major, gives `0.4.0`",
                "b/CHANGELOG.md:11  `0.2.1` follows `0.1.0`, and the highest class of its entries, patch, gives `0.1.1`",
                "b/CHANGELOG.md:7  `0.3.0` follows `0.2.1`, and the highest class of its entries, patch, gives `0.2.2`",
                "b/CHANGELOG.md:3  `0.5.0` follows `0.3.0`, and the highest class of its entries, patch, gives `0.3.1`",
            ]
        );
    }

    /// The claim, claim 5's other direction: a release numbered below what its highest class
    /// gives is a finding, at 1.0 and above as under it: 1.2.1 by minor from 1.2.0 gives 1.3.0.
    /// Mutation checked: `if *version != expected` made `if *version > expected`; caught.
    #[test]
    fn a_release_numbered_below_its_class_is_a_finding() {
        let a = crate_("a", true);
        let log =
            parse("# a\n\n## 1.2.1\n\n- M, minor: an addition\n\n## 1.2.0\n\n- M, patch: a fix\n");
        assert_eq!(
            found(&[(&a, log)]),
            ["a/CHANGELOG.md:3  `1.2.1` follows `1.2.0`, and the highest class of its entries, minor, gives `1.3.0`"]
        );
    }

    /// The claim, the oldest version and an unclassed one are not judged: 0.0.9, the oldest, has
    /// none before it, and 0.1.0, which `unclassed` lists, would follow it by patch as 0.0.10.
    /// Mutation checked: `if table.unclassed().contains(version)` made `if false`; caught.
    #[test]
    fn an_unclassed_version_and_the_oldest_are_not_judged() {
        let a = crate_("a", true);
        let log = parse("# a\n\n## 0.1.0\n\n- M, patch: tagged in an unclassed version\n\n## 0.0.9\n\n- M, patch: the oldest\n");
        assert!(found(&[(&a, log)]).is_empty());
    }
}
