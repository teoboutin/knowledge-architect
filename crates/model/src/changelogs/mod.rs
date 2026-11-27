//! A project's per-crate changelogs: the manifest's `[changelog]` table, the crates that hold one,
//! read from each Component's `Cargo.toml`, the parse of a crate's changelog, and the rendering of
//! its preamble and of the assembled file.
//!
//! **Every parse and every rendering is here, and no finding is.** Nothing else in the model
//! depends on a changelog's content, so the parse records what it could not read rather than
//! raising a finding of phases 1 to 3, which say whether the model can be trusted, per
//! `design@knowledge-architect@model-crate`; every changelog finding is a check of `check`'s last
//! phase, so a tool that asks for a complete model answers over a malformed entry. The manifest's
//! own table is the exception, as any table is: a declaration it refuses is a complaint of
//! phase 1.
//!
//! The grammar and the classes are compiled in, as the tool's convention for a kind of document;
//! the categories are the project's, declared in the manifest.

mod crates;
mod parse;
mod refusal;
mod render;
mod table;

pub use crates::{read as changelog_crates, CargoTrouble, Crate, Crates};
pub use parse::{parse, Changelog, Entry, Heading, Section, Tag, Unread};
pub use refusal::{hand_written, refusal, Refused};
pub use render::{assembled_file, assembled_text, expected_preamble, GENERATED_HEADER};
pub use table::ChangelogTable;
pub(crate) use table::{resolve, Declared};

use std::fmt;

/// The name of a crate's changelog, in the crate's directory.
pub const CHANGELOG_FILE: &str = "CHANGELOG.md";

/// The heading of the section of the entries not yet released.
pub const NEXT_RELEASE: &str = "Next release";

/// A released version, as a version heading writes it: three numbers joined by `.`, with no
/// pre-release and no build part.
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
    /// The version `text` spells, or `None` where it is not three numbers joined by `.`.
    pub fn parse(text: &str) -> Option<Version> {
        let mut parts = text.split('.');
        let mut number = || -> Option<u64> {
            let part = parts.next()?;
            if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
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

    /// The version a release of the class `class` gives after this one, as Cargo's semver reads
    /// the three numbers: at `1.0.0` and above, a major change bumps the first, a minor one the
    /// second and a patch the third; under `0.x`, a major and a minor change both bump the second
    /// and a patch the third. Each bump resets the numbers after it.
    pub fn bumped(self, class: Class) -> Version {
        match (class, self.major) {
            (Class::Major, 0) | (Class::Minor, _) => Version {
                minor: self.minor + 1,
                patch: 0,
                ..self
            },
            (Class::Major, _) => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            (Class::Patch, _) => Version {
                patch: self.patch + 1,
                ..self
            },
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

    /// The claim, the bump of each class above and under `1.0.0`, as Cargo's semver reads them.
    /// Mutation checked: in `Version::bumped`, `(Class::Major, 0) | (Class::Minor, _) => Version {`
    /// made `(Class::Minor, _) => Version {`, so a major change under `0.x` bumps the first number;
    /// caught.
    #[test]
    fn each_class_bumps_the_number_cargo_s_semver_reads() {
        assert_eq!(v("1.2.3").bumped(Class::Major), v("2.0.0"));
        assert_eq!(v("1.2.3").bumped(Class::Minor), v("1.3.0"));
        assert_eq!(v("1.2.3").bumped(Class::Patch), v("1.2.4"));
        assert_eq!(v("0.2.3").bumped(Class::Major), v("0.3.0"));
        assert_eq!(v("0.2.3").bumped(Class::Minor), v("0.3.0"));
        assert_eq!(v("0.2.3").bumped(Class::Patch), v("0.2.4"));
    }

    /// The claim, a version heading is three numbers and nothing else. Mutation checked:
    /// `parts.next().is_none().then_some(version)` made `Some(version)`; caught.
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
        ] {
            assert_eq!(Version::parse(text), None, "{text}");
        }
        assert!(v("0.10.0") > v("0.9.9"));
    }
}
