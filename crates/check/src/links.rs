//! Symbolic links for the tests, on every system they run on.
//!
//! On Windows a link is made to a file or to a directory, and a target that does not exist has no
//! kind to read, so the caller names it. Each package whose tests make a link holds its own copy, per
//! `design@knowledge-architect@symlink-helper-home`, so an edit of this one goes to the others. No test
//! skips where the machine refuses a link, per
//! `design@knowledge-architect@no-skip-where-links-are-refused`.

use std::path::{Path, PathBuf};

/// Makes `link` a symbolic link to `target`, a file or nothing yet. A relative target is written with
/// the system's separator, since Windows follows no link whose relative target holds `/`.
///
/// # Panics
///
/// Where the machine refuses the link.
#[track_caller]
pub(crate) fn link_file(target: impl AsRef<Path>, link: impl AsRef<Path>) {
    let target: PathBuf = target.as_ref().components().collect();
    #[cfg(unix)]
    let made = std::os::unix::fs::symlink(target, link);
    #[cfg(windows)]
    let made = std::os::windows::fs::symlink_file(target, link);
    made.expect("a symbolic link");
}

/// Makes `link` a symbolic link to `target`, a directory or nothing yet. A relative target is written with
/// the system's separator, since Windows follows no link whose relative target holds `/`.
///
/// # Panics
///
/// Where the machine refuses the link.
#[track_caller]
pub(crate) fn link_dir(target: impl AsRef<Path>, link: impl AsRef<Path>) {
    let target: PathBuf = target.as_ref().components().collect();
    #[cfg(unix)]
    let made = std::os::unix::fs::symlink(target, link);
    #[cfg(windows)]
    let made = std::os::windows::fs::symlink_dir(target, link);
    made.expect("a symbolic link");
}
