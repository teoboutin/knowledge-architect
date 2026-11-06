//! Creating a knowledge-architect project, as a library: the logic of the commands `init` and
//! `new-component` of `klarch`, behind typed functions with no argument parser in them.
//!
//! [`init`] writes a project's manifest, the installed agent files and every document and home the
//! check requires, and ends as `check --fix` ends inside a git worktree; outside one it installs the
//! agent files, says what comes next and exits 0. It runs where no project exists yet, so a caller
//! calls it before it locates one. [`new_component`] adds a Component to a located project. Every
//! refusal of either comes before its first write, and is `Err`, which the caller prints before it
//! exits 2; once it has written, a failure is exit 1. Each function prints as it works, per
//! `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`, so a closed stdout ends the
//! process with exit 2. [`new_component`] takes a manifest as its caller's extensions have configured
//! it, which nothing enforces, per
//! `issue@knowledge-architect@a-typed-command-trusts-its-caller-to-have-configured-the-manifest`.
//!
//! **A project does not name this package**: it pins `knowledge-architect`, whose binary offers these
//! commands, and whose documentation names the library entry an extension's binary calls.

// A `pub` item no consumer can reach is refused, so an item is public only through the re-exports
// below, per `design@knowledge-architect@api-facade`.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

use knowledge_architect_check as check;
use knowledge_architect_git as git;
use knowledge_architect_model as model;

mod command;
mod output;

pub use command::{init, new_component, InitOptions, Vcs};

/// The directory of the Component this library belongs to: its own crate directory, evaluated at
/// build time.
///
/// A binary that links this library hands it to the walk beside the directories of its other
/// libraries, so the string literals of this crate's source are read as data, per
/// `design@check@checker-source-literals-are-data`. Once the library is consumed as a published
/// crate, the directory is outside the tree being checked and exempts nothing.
pub fn component_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(CRATE_DIR)
}

/// This crate's directory at build time, which a binary's refusal of a build from another checkout
/// compares with the tree it runs over, per `design@check@a-foreign-build-is-refused`.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, as its `Cargo.toml` declares it.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");
