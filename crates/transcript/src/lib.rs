//! The reader of the session transcripts an agent harness writes on disk, Claude Code's and no other
//! harness's, as a library: the logic of the command `klarch transcript`, behind one typed function
//! with no argument parser in it.
//!
//! [`transcript`] lists a project's sessions, shows one, or searches them, and prints as it works, per
//! `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`, so a closed stdout ends the
//! process with exit 2. It reads under `CLAUDE_CONFIG_DIR`, or under `.claude` of `HOME` where that is unset.
//! Its caller gives it the project's root; from a linked worktree, it also reads the sessions of the
//! same path in the main checkout. Every line is labelled by what the harness recorded of it, never by
//! a substring of its text, and every line or value the reader does not know is counted and reported, never a failure.
//!
//! **A project does not name this package**: it pins `knowledge-architect`, whose binary offers the
//! command. A Rust program that reads transcripts depends on it directly.
//!
//! ```no_run
//! use knowledge_architect_transcript::{transcript, ListOptions, TranscriptVerb};
//!
//! let root = std::path::Path::new(".");
//! let code = transcript(root, &TranscriptVerb::List(ListOptions::default()))?;
//! # let _ = code;
//! # Ok::<(), String>(())
//! ```

// A `pub` item no consumer can reach is refused, so an item is public only through the re-exports
// below, per `design@knowledge-architect@api-facade`.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

mod command;
// Declared for its own tests alone: the maintenance tool includes it, per
// `design@xtask@internals-included-not-exported`.
#[cfg(test)]
mod kinds;
mod output;
mod reader;

pub use command::{
    transcript, ListOptions, SearchOptions, ShowFrom, ShowOptions, TranscriptVerb, ROLES,
};

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
