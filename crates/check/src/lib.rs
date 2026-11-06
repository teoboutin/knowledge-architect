//! The checker of knowledge-architect, as a library: every check over a project's documents, and the
//! logic of each command of the binary `klarch`, behind typed functions with no argument parser in
//! them.
//!
//! The checker reads a project's documents once, through git's listing, into one model. Every
//! check is then a pure function over that model: it touches no file and spawns no process.
//! The project's `knowledge-architect.toml` declares what is checked, and nothing about a
//! project is compiled in. The decisions behind the design, with their arguments, are in the
//! repository, <https://github.com/teoboutin/knowledge-architect>, at
//! `register@check@design`.
//!
//! **A project does not name this package**: it pins `knowledge-architect`, whose binary and library
//! are built on it, and whose documentation shows how to write an extension. The items here reach
//! that library's consumers through its re-exports.
//!
//! | module | for |
//! | --- | --- |
//! | the crate root | the nouns every consumer meets: [`Manifest`], [`Model`], [`Document`], [`Finding`], [`MANIFEST_NAME`], [`CHECKER_VERSION`], [`component_dir`], [`CRATE_DIR`], [`PACKAGE`] |
//! | [`commands`] | each command's logic as a typed function, gathering a run's inputs, refusing a build from another checkout or another version |
//! | [`extension`] | writing an extension: the two traits, what `check` hands them, and what they return |
//! | [`document`] | reading a document's parse: prose regions, scopes, code spans, observations |
//! | [`testing`] | testing an extension over a mock project, as the binary would run it |

// A `pub` item no consumer can reach is refused, so an item is public only through the
// re-exports below and the role modules, per `design@knowledge-architect@api-facade`.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

// The implementation modules. They are private: what a consumer uses is re-exported below
// and by the role modules, so a file can move without changing a public path.
mod agents;
mod build_origin;
mod check;
mod graph;
mod transcript;

// Every git invocation is the `git` crate's, per
// `design@knowledge-architect@checker-asks-git-through-the-crate`. Named `git` here, so the modules that ask it
// read `crate::git::...` as when it was a module of this crate.
use knowledge_architect_git as git;

// What phases 1 to 3 read and what they build are the `model` crate's, which every module here
// names `crate::model`.
use knowledge_architect_model as model;

// The role modules: one per kind of consumer.
pub mod commands;
pub mod document;
pub mod extension;
pub mod testing;

// The checks run against whole mock projects. A unit-test module rather than an integration
// test, because its assertions read the checks' own `run` and `Inputs` literal, which are not
// public.
#[cfg(test)]
mod mock_projects;

pub use model::{Document, Finding, Manifest, Model, MANIFEST_NAME};

/// The version of this library, which is the checker's version whatever binary links it.
///
/// It is the value `[project] checker-version` must hold for [`commands::refuse_another_version`] to
/// accept a project. A test that copies a mock project out of its library's directory, where `"fixture"`
/// is refused, writes this value into the copy's manifest. In an extension's crate, `CARGO_PKG_VERSION` is the extension's version, not this
/// one.
pub const CHECKER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The directory of the Component this library belongs to: its own crate directory, evaluated at
/// build time.
///
/// A binary built from this library hands it to the walk with the directories of its other
/// libraries, so the tool's own fixtures are read as data, per
/// `design@check@checker-source-literals-are-data`. Once the library is consumed as a published
/// crate, the directory is outside the tree being checked and exempts nothing.
pub fn component_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(CRATE_DIR)
}

/// This crate's directory at build time, which a binary compares with the tree it runs over, per
/// `design@check@a-foreign-build-is-refused`.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, as its `Cargo.toml` declares it.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");
