//! A project's per-crate changelogs: the manifest's `[changelog]` table, the crates that hold one,
//! read from each Component's `Cargo.toml`, the parse of a crate's changelog, and the rendering of
//! its preamble and of the assembled file.
//!
//! **Every parse and every rendering is here, and no finding is.** Nothing else in the model
//! depends on a changelog's content, so the parse records what it could not read rather than
//! raising a finding of phases 1 to 3, which say whether the model can be trusted, per
//! `design@knowledge-architect@model-crate`; every changelog finding is a check of `check`'s last
//! phase, per `design@check@changelog-opt-in`, so a tool that asks for a complete model answers
//! over a malformed entry. The manifest's
//! own table is the exception, as any table is: a declaration it refuses is a complaint of
//! phase 1.
//!
//! The grammar and the classes are compiled in, as the tool's convention for a kind of document;
//! the categories are the project's, declared in the manifest, per
//! `design@model@changelog-entry-categories`.

mod crates;
mod parse;
mod refusal;
mod render;
mod table;

pub use crates::{read as changelog_crates, CargoTrouble, Crate, Crates};
pub use parse::{parse, Changelog, Entry, Heading, Section, Tag, Unread};
pub use refusal::{hand_written, refusal, Refused, WrittenTree};
pub use render::{assembled_file, assembled_text, expected_preamble, GENERATED_HEADER};
pub use table::ChangelogTable;
pub(crate) use table::{resolve, Declared};

use std::fmt;

/// The name of a crate's changelog, in the crate's directory.
pub const CHANGELOG_FILE: &str = "CHANGELOG.md";

/// The heading of the section of the entries not yet released.
pub const NEXT_RELEASE: &str = "Next release";

/// A released version, as a version heading writes it: three numbers joined by `.`, each with no
/// leading zero, and no pre-release and no build part, per
/// `design@model@changelog-version-is-a-release-version`, as
/// `path@check@docs/grounding/cargo-semver.md` reads Cargo's versions.
///
/// Each number is at most `u64::MAX`: [`Version::parse`] refuses a larger one, and
/// [`Version::successors`] gives none where the number it would bump is already that.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// The first number.
    pub major: u64,
    /// The second number.
    pub minor: u64,
    /// The third number.
    pub patch: u64,
}

impl Version {
    /// The version `text` spells, or `None` where it is not three numbers joined by `.`: a part
    /// that is empty, holds anything but ASCII digits, opens with `0` and has another digit after
    /// it, as Cargo refuses `01.2.3`, or exceeds `u64::MAX`, gives `None`, and so does a
    /// pre-release or a build part.
    pub fn parse(text: &str) -> Option<Version> {
        let mut parts = text.split('.');
        let mut number = || -> Option<u64> {
            let part = parts.next()?;
            if part.is_empty()
                || !part.bytes().all(|b| b.is_ascii_digit())
                || (part.len() > 1 && part.starts_with('0'))
            {
                return None;
            }
            part.parse().ok()
        };
        let version = Version {
            major: number()?,
            minor: number()?,
            patch: number()?,
        };
        parts.next().is_none().then_some(version)
    }

    /// The versions a release of the class `class` may take after this one, as Cargo's semver
    /// reads the three numbers, per `design@check@changelog-bump-check` and
    /// `path@check@docs/grounding/cargo-semver.md`:
    ///
    /// - at `1.0.0` and above, a major change bumps the first number, a minor one the second and a
    ///   patch the third;
    /// - under `0.y.z` with `y` at least 1, a major change bumps `y` or gives `1.0.0`, a minor one
    ///   bumps `y` and a patch `z`;
    /// - under `0.0.z`, every class bumps `z`, and a major change may also give `0.1.0` or
    ///   `1.0.0`.
    ///
    /// Each bump resets the numbers after it. The first version is the bump of the class, and any
    /// other follows it. A bump of a number that is `u64::MAX` has no result, so the list holds
    /// the other versions alone, and is empty where the class admits no other.
    pub fn successors(self, class: Class) -> Vec<Version> {
        let next = |n: u64| n.checked_add(1);
        let first = |major| Version {
            major,
            minor: 0,
            patch: 0,
        };
        let second = |minor| Version {
            minor,
            patch: 0,
            ..self
        };
        let third = |patch| Version { patch, ..self };
        let stable = first(1);
        match (class, self.major, self.minor) {
            (Class::Major, 1.., _) => next(self.major).map(first).into_iter().collect(),
            (Class::Minor, 1.., _) => next(self.minor).map(second).into_iter().collect(),
            (Class::Major, 0, 1..) => next(self.minor)
                .map(second)
                .into_iter()
                .chain([stable])
                .collect(),
            (Class::Minor, 0, 1..) => next(self.minor).map(second).into_iter().collect(),
            (Class::Major, 0, 0) => next(self.patch)
                .map(third)
                .into_iter()
                .chain([second(1), stable])
                .collect(),
            (Class::Patch, _, _) | (Class::Minor, 0, 0) => {
                next(self.patch).map(third).into_iter().collect()
            }
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The bump class of an entry: Cargo's semver, compiled in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    /// A fix, compatible both ways.
    Patch,
    /// An addition, compatible with what came before.
    Minor,
    /// A change a consumer can meet as breaking.
    Major,
}

impl Class {
    /// The class `word` names, or `None` where it names none.
    pub fn parse(word: &str) -> Option<Class> {
        match word {
            "patch" => Some(Class::Patch),
            "minor" => Some(Class::Minor),
            "major" => Some(Class::Major),
            _ => None,
        }
    }

    /// The word an entry writes it with.
    pub fn word(self) -> &'static str {
        match self {
            Class::Patch => "patch",
            Class::Minor => "minor",
            Class::Major => "major",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(text: &str) -> Version {
        Version::parse(text).expect("a version")
    }

    fn vs(texts: &[&str]) -> Vec<Version> {
        texts.iter().map(|t| v(t)).collect()
    }

    /// The claim, the versions each class admits at and above `1.0.0`, under `0.y.z` and under
    /// `0.0.z`, as Cargo's semver reads them. Mutations checked, each caught: in
    /// `Version::successors`, the `.chain([stable])` of a major change under `0.y.z` removed, and
    /// the `.chain([second(1), stable])` of one under `0.0.z` removed.
    #[test]
    fn each_class_admits_the_versions_cargo_s_semver_reads() {
        assert_eq!(v("1.2.3").successors(Class::Major), vs(&["2.0.0"]));
        assert_eq!(v("1.2.3").successors(Class::Minor), vs(&["1.3.0"]));
        assert_eq!(v("1.2.3").successors(Class::Patch), vs(&["1.2.4"]));
        assert_eq!(v("0.2.3").successors(Class::Major), vs(&["0.3.0", "1.0.0"]));
        assert_eq!(v("0.2.3").successors(Class::Minor), vs(&["0.3.0"]));
        assert_eq!(v("0.2.3").successors(Class::Patch), vs(&["0.2.4"]));
        assert_eq!(
            v("0.0.3").successors(Class::Major),
            vs(&["0.0.4", "0.1.0", "1.0.0"])
        );
        assert_eq!(v("0.0.3").successors(Class::Minor), vs(&["0.0.4"]));
        assert_eq!(v("0.0.3").successors(Class::Patch), vs(&["0.0.4"]));
    }

    /// The claim, a bump of a number that is `u64::MAX` gives no version, never a panic or a
    /// wrapped number, while the other versions the class admits remain. Mutation checked:
    /// `let next = |n: u64| n.checked_add(1);` made `let next = |n: u64| Some(n.wrapping_add(1));`;
    /// caught.
    #[test]
    fn a_bump_past_the_largest_number_gives_no_version() {
        let max = u64::MAX;
        assert_eq!(v(&format!("{max}.0.0")).successors(Class::Major), vs(&[]));
        assert_eq!(v(&format!("1.{max}.0")).successors(Class::Minor), vs(&[]));
        assert_eq!(v(&format!("1.0.{max}")).successors(Class::Patch), vs(&[]));
        assert_eq!(
            v(&format!("0.{max}.0")).successors(Class::Major),
            vs(&["1.0.0"])
        );
        assert_eq!(v(&format!("0.{max}.0")).successors(Class::Minor), vs(&[]));
        assert_eq!(
            v(&format!("0.0.{max}")).successors(Class::Major),
            vs(&["0.1.0", "1.0.0"])
        );
        assert_eq!(Version::parse("0.18446744073709551616.0"), None);
    }

    /// The claim, a version heading is three numbers with no leading zero and nothing else.
    /// Mutations checked, each caught: `parts.next().is_none().then_some(version)` made
    /// `Some(version)`; the test of a leading zero,
    /// `|| (part.len() > 1 && part.starts_with('0'))`, removed; and the test of the digits,
    /// `|| !part.bytes().all(|b| b.is_ascii_digit())`, removed, so a part with a sign in front,
    /// which `u64::from_str` reads, is a number.
    #[test]
    fn a_version_is_three_numbers() {
        assert_eq!(v("0.10.0").to_string(), "0.10.0");
        for text in [
            "0.10",
            "0.10.0.1",
            "0.10.x",
            "v0.1.0",
            "0.1.0-rc.1",
            "",
            "1..0",
            "01.2.3",
            "1.02.3",
            "1.2.03",
            "1.2.00",
            "+1.2.3",
            "1.+2.3",
        ] {
            assert_eq!(Version::parse(text), None, "{text}");
        }
        assert!(v("0.10.0") > v("0.9.9"));
        assert_eq!(v("0.0.0").to_string(), "0.0.0");
        assert_eq!(v("10.20.30").to_string(), "10.20.30");
    }
}
