//! A library that wraps git for the tools of knowledge-architect: a command to run git with, and
//! the readings the checker of a project's documents asks for today, the listing a walk reads,
//! the ignore rules a path reference is asked against, the dates of a listing's last change, the
//! revisions of a range, the paths a commit and the work tree differ by, and the trees, blobs and
//! messages of a commit or of the tree git's index would commit.
//!
//! Every function that asks git runs it as a child process, with the directory it is given as its
//! working directory; [`ignore_query`] and [`object_name`] only build an argument. Every question
//! but [`inside_work_tree`] and [`init_repository`] needs that directory inside a git worktree,
//! whose `.git` sits at or above it, and [`blobs`] and [`blob_bytes`] need `git` 2.36 or newer.
//! Whatever the caller's environment names, every invocation runs with the per-user ignore file
//! pointed at the null device and without the variables that would point git at another
//! repository, and keeps `GIT_INDEX_FILE`, so a pre-commit hook reads the index its commit
//! records.
//!
//! A failure of an invocation whose answer a verdict depends on, such as a listing, is an `Err`
//! carrying what was run and git's own reason, never an empty answer: a listing that came back
//! empty because git failed would read as a project with no documents. Two functions answer
//! nothing where git cannot say, since no verdict depends on them: [`last_changed`], whose dates
//! fill a convenience column, and [`main_worktree_path`]. [`rev_parse`] answers `None` for every
//! failure, against its contract, as `issue@git@rev-parse-answers-none-for-every-git-failure`
//! records.
//!
//! ```no_run
//! use std::path::Path;
//!
//! let root = Path::new(".");
//! let (entries, untracked) = knowledge_architect_git::entries(root)?;
//! println!("{} entries, {} of them untracked", entries.len(), untracked.len());
//! # Ok::<(), std::io::Error>(())
//! ```

// A `pub` item no consumer can reach is warned of, and refused by the clippy gate's `-D warnings`,
// so an item is public only through the re-exports below.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

mod git;

pub use git::{
    blob_bytes, blobs, commit_message, entries, git, ignore_query, ignored, ignores,
    index_differs_from_head, init_repository, inside_work_tree, last_changed, main_worktree_path,
    object_name, paths_changed_since, rev_list, rev_parse, snapshot_entries, stage_generated,
    tracked_and_ignored, unmerged, Entry, EntryKind, Invocation, SnapshotEntry, Source,
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
