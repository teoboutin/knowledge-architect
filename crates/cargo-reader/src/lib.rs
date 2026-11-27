//! A library that reads what the tools of knowledge-architect need of Cargo, in two formats:
//!
//! - the text of a `Cargo.toml`, [`read_manifest`]: the package name, and its `version`,
//!   `publish`, `repository`, `include`, `exclude`, `readme`, `license-file` and `workspace`, each
//!   as the manifest states it, with [`Package::inherit`] resolving a field set to a table whose
//!   `workspace` is `true` from the workspace's `[workspace.package]`; and a `[workspace]` table's
//!   `members`, `exclude` and `[workspace.package]`;
//! - the output of `cargo metadata --format-version 1`, [`run_metadata`], which runs it in a
//!   directory with the flags its caller passes, and [`read_metadata`], which reads its JSON: the
//!   resolved graph, and each of its packages' name, version, manifest path and `repository`;
//! - the `include` and `exclude` patterns of a package, [`include_matches`] and
//!   [`exclude_matches`], which say whether they package a file as `cargo package` reads them.
//!
//! Reading a manifest touches no file and runs nothing; the caller reads the text, and finds the
//! workspace's manifest a package inherits from. Running `cargo metadata` needs Cargo, named by the
//! variable `CARGO` or found on the path. Every failure is an error carrying the reason, never an
//! empty answer: a manifest that is no TOML or holds a field of a type Cargo refuses, Cargo's own
//! failure with its message, an output of another shape, a pattern Cargo refuses.
//!
//! ```
//! use knowledge_architect_cargo_reader::{read_manifest, Field};
//!
//! let manifest = read_manifest("[package]\nname = \"a-crate\"\nversion = \"1.0.0\"\n")?;
//! let package = manifest.package.expect("a package");
//! assert_eq!(package.name, "a-crate");
//! assert_eq!(package.version, Field::Set("1.0.0".to_string()));
//! # Ok::<(), knowledge_architect_cargo_reader::ManifestError>(())
//! ```

// A `pub` item no consumer can reach is warned of, and refused by the clippy gate's `-D warnings`,
// so an item is public only through the re-exports below.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

// One module per external format, so a change of one format, or of the facts the tools read of
// it, changes one module.
mod include;
mod manifest;
mod metadata;

pub use include::{exclude_matches, include_matches, PatternError};
pub use manifest::{
    read_manifest, Field, Manifest, ManifestError, Package, Publish, Readme, Workspace,
    WorkspacePackage,
};
pub use metadata::{
    read_metadata, run_metadata, Graph, Metadata, MetadataError, MetadataPackage, Node,
};

/// The directory of the Component this library belongs to: its own crate directory, evaluated at
/// build time.
///
/// A binary that links this library hands it to the walk beside the directories of its other
/// libraries, so the string literals of this crate's source are read as data, as the checker's
/// own source is. Once the library is consumed as a published crate, the directory is outside
/// the tree being checked and exempts nothing.
pub fn component_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(CRATE_DIR)
}

/// This crate's directory at build time, which a binary's refusal of a build from another
/// checkout compares with the tree it runs over.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, as its `Cargo.toml` declares it.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");
