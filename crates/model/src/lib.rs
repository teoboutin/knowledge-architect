//! What knowledge-architect reads of a project: its manifest, the walk git supplies, the parse of
//! each document, the entity table over them, and what a reference resolves to.
//!
//! It holds what a tool over a project's documents needs. A tool builds a [`Model`], asks
//! [`foundation::complete_working_tree`] whether it can be trusted, and reads it. The checks of a
//! document's content are the package `knowledge-architect`, which depends on this one; a project
//! that runs the checker names that package alone. The one check here, while it stays in phase 2,
//! compares a project's installed agent files with the text this version ships.
//!
//! The gate judges what the manifest declares against what the tool hands it. A manifest table
//! belongs to an extension, so a tool over a project with one calls [`extension::contribute`] before
//! [`Model::build`], or the table is reported, and the files the extension generates are walked.
//! The version a manifest pins is the binary's to refuse, not this library's.
//!
//! ```no_run
//! use knowledge_architect_model::{foundation, table, Manifest, Model};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let manifest = Manifest::find(std::env::current_dir()?.as_path())?;
//! let model = Model::build(&manifest, &[])?;
//! // Refuses a model phases 1 to 3 stop, naming the phase.
//! foundation::complete_working_tree(&manifest, &model)?;
//! let anchors = table::Anchors::of(&manifest, model.listing());
//! let entities = table::Entities::build(&model, &anchors);
//! # let _ = entities;
//! # Ok(())
//! # }
//! ```
//!
//! The public items are grouped by what a consumer is doing, and each is named below, so this
//! file is the public surface and moving an item between files changes no public path, per
//! `rejected@core@module-tree-as-public-api`.
//!
//! | module | for |
//! | --- | --- |
//! | the crate root | the nouns every consumer meets: [`Manifest`], [`Model`], [`Document`], [`Finding`], [`MANIFEST_NAME`], [`component_dir`] |
//! | [`declared`] | what a manifest declares: the registers, their shapes, the names of the built-in ones |
//! | [`document`] | reading a document's parse: prose regions, scopes, code spans, observations |
//! | [`listing`] | what the tree holds beside the walked documents, and how the walk filters it |
//! | [`table`] | the entity table, the anchors, and the records over them |
//! | [`resolve`] | what a reference names, or why it names nothing |
//! | [`generated`] | the files the checker generates and the paths they sit at |
//! | [`installed`] | the agent files this version ships, rendered for a project |
//! | [`foundation`] | phases 1 to 3: whether the model can be trusted |
//! | [`extension`] | what an extension gives phase 1 |

// A `pub` item no consumer can reach is refused, so an item is public only through the re-exports
// below, as the core's facade is, per `design@core@api-facade`.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

mod contribution;
mod entity;
mod finding;
mod index;
mod manifest;
mod model;
mod phases;
mod records;
mod resolver;
mod scan;
mod shipped;
mod source;
mod survey;
mod tree;
mod walk;

// Every git invocation is the `git` crate's, per
// `design@knowledge-architect@checker-asks-git-through-the-crate`. Named `git` here, so the modules that ask it
// read `crate::git::...`.
use knowledge_architect_git as git;

// The core run against whole mock projects, over what this crate holds, until the shared test
// Component takes them.
#[cfg(test)]
mod mock_projects;

pub use finding::{escaped, Finding};
pub use manifest::{Manifest, MANIFEST_NAME};
pub use model::{Document, Model};

/// What a manifest declares.
pub mod declared {
    pub use crate::manifest::{
        normalise_list, normalise_one, owned_path, LocationDecl, Pin, Register, Registers, Scope,
        Shape, Walk, AGENT_DOCUMENT, GOAL_REGISTER, ISSUE_REGISTER, MILESTONE_REGISTER,
        OWNED_PREFIX, REJECTED_REGISTER, SPEC_REGISTER, TRIPWIRE_REGISTER,
    };
}

/// The parse of a document, per `design@core@an-extension-builds-its-own-model`.
pub mod document {
    pub use crate::scan::{Located, Observation, RetiredForm, SlugSite};
    pub use crate::source::{Frontmatter, Literals, Parsed, Prose, Scope, ScopeKind};

    /// The markdown grammar.
    pub mod md {
        pub use crate::source::md::parse;
    }

    /// The Rust grammar.
    pub mod rs {
        pub use crate::source::rs::parse;
    }
}

/// What the tree holds beside the walked documents, and how the walk filters it.
pub mod listing {
    pub use crate::model::snapshot_checker_dirs;
    pub use crate::survey::{from_listing, survey, Outside, Survey};
    pub use crate::walk::{live_files, skipped};
}

/// The entity table, the anchors, and the records over them.
pub mod table {
    pub use crate::entity::{
        candidate, is_anchor_name, is_entity_id, is_entry_id, Anchor, Anchors, Candidate,
        Constructed, Entities, EntryOf, Home, Kind, Resolution, Site, EVERY_ANCHOR, REGISTER_KIND,
    };
    pub use crate::model::DumpRow;
    pub use crate::records::{cited_within, edges, records, EntryKey, Inbound, Record};
}

/// What a reference names, or why it names nothing, with no finding attached.
///
/// [`reference`](resolve::reference) resolves a candidate of an anchored kind by the function of its
/// kind, and [`harness`](resolve::harness) one of a harness kind. Each returns an
/// [`Outcome`](resolve::Outcome): a [`Target`](resolve::Target), or a [`Cause`](resolve::Cause), whose
/// [`texts`](resolve::Cause::texts) a consumer prints or writes into a finding of its own. `rel` names
/// the document that writes the reference.
pub mod resolve {
    pub use crate::resolver::{
        assert_target, harness, has_scheme, ignore_queries, in_plans_directory, is_navigation,
        path, planned, planned_target, reference, refused, register, table, Cause, Outcome, Target,
    };
}

/// The files the checker generates, and the paths they sit at.
pub mod generated {
    pub use crate::index::{file_register_indexes, generated_paths, label, rel_from};
}

/// The agent files this version ships, rendered for a project.
pub mod installed {
    pub use crate::shipped::{imports_primer, lf, render, shipped, IMPORT_LINE, PRIMER};
}

/// Phases 1 to 3: whether the model can be trusted.
pub mod foundation {
    pub use crate::phases::{complete_working_tree, foundation, Gathered, Phase, Stop, TreeInputs};
}

/// What an extension gives phase 1.
pub mod extension {
    pub use crate::contribution::{contribute, Contribution};
}

/// The directory of the Component this library belongs to: its own crate directory, evaluated at
/// build time.
///
/// A binary that links this library hands it to the walk beside the directories of its other
/// libraries, so the string literals of this crate's source are read as data, per
/// `design@core@checker-source-literals-are-data`. Once the library is consumed as a published
/// crate, the directory is outside the tree being checked and exempts nothing.
pub fn component_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(CRATE_DIR)
}

/// This crate's directory at build time, which a binary's refusal of a build from another
/// checkout compares with the tree it runs over.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, as its `Cargo.toml` declares it.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod facade_tests {
    /// The statements of this file, each with its comments taken off.
    fn statements(source: &str) -> Vec<String> {
        let code: Vec<&str> = source
            .lines()
            .map(|line| line.split("//").next().unwrap_or(""))
            .collect();
        code.join(" ")
            .split(';')
            .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect()
    }

    /// Whether a statement publishes a module of the tree, or a glob.
    fn publishes_the_tree(statement: &str) -> bool {
        let last = statement.rsplit(['{', '}']).next().unwrap_or("").trim();
        let module = last.starts_with("pub mod ");
        let glob = statement.contains("pub use ") && statement.contains('*');
        module || glob
    }

    /// The claim: this file names every public module with its body, so the public surface is read
    /// here and is not computed from the module tree, per
    /// `rejected@core@module-tree-as-public-api`. `unreachable_pub` is silent on both forms it
    /// refuses, since each makes the items behind it reachable.
    #[test]
    fn the_facade_publishes_no_module_of_the_tree_and_no_glob() {
        let source = include_str!("lib.rs");
        let code = &source[..source
            .find("#[cfg(test)]\nmod facade_tests")
            .expect("this module")];
        for statement in statements(code) {
            assert!(!publishes_the_tree(&statement), "{statement}");
        }
    }

    /// The claim: each spelling of the two forms is refused, a comment after it and a list holding
    /// a glob included, and a role module with its body is not.
    #[test]
    fn every_spelling_of_a_published_module_or_a_glob_is_refused() {
        for source in [
            "pub mod scan;",
            "pub mod scan; // a comment",
            "pub use crate::scan::*;",
            "pub use crate::scan::{Located, *};",
            "pub use crate::scan::{\n    Located,\n    *,\n};",
        ] {
            assert!(
                statements(source).iter().any(|s| publishes_the_tree(s)),
                "{source}"
            );
        }
        for source in [
            "mod scan;",
            "pub mod table { pub use crate::entity::{Kind}; }",
        ] {
            assert!(
                !statements(source).iter().any(|s| publishes_the_tree(s)),
                "{source}"
            );
        }
    }
}
