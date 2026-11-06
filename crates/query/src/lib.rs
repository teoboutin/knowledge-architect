//! Tools over a project's scanned record, as a library: the logic of the commands `show`, `issues`,
//! `tripwires`, `anchors`, `id` and `worklist` of `klarch`, behind typed functions with no argument
//! parser in them. They read the model, run no check, and depend on nothing of the checks.
//!
//! Each function takes a manifest, and the directories of the running binary's own source, and
//! prints as it works, so a closed stdout ends the process with exit 2. **Its exit code says whether
//! it ran before it says what it found**, per `design@cli@exit-code-ladder`: 0 ran-and-clean, 1
//! ran-and-negative, and `Err` could-not-run, which the caller prints before it exits 2. `id` and
//! `worklist` refuse over a model whose first three phases stop.
//!
//! **A project does not name this package**: it pins `knowledge-architect`, whose binary offers these
//! commands. A Rust program that consults a record the same way depends on it directly.
//!
//! ```no_run
//! use knowledge_architect_query::{issues, IssuesFilter};
//! use knowledge_architect_model::Manifest;
//!
//! let manifest = Manifest::find(std::path::Path::new(".")).map_err(|e| e.to_string())?;
//! let code = issues(&manifest, &IssuesFilter::default(), &[])?;
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

use knowledge_architect_git as git;
use knowledge_architect_model as model;
use model::{Manifest, Model};

mod anchors;
mod graph;
mod id;
mod output;
mod rows;
mod show;
mod worklist;

pub use anchors::anchors;
pub use id::id;
pub use rows::{issues, tripwires, IssuesFilter, TripwiresFilter};
pub use show::show;
pub use worklist::{worklist, WorklistMode};

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
