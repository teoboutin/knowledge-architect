//! Whether the running binary was built from the tree it checks.
//!
//! Every command judges a tree with the binary the cargo alias builds from the checkout, and
//! one built from another checkout is refused, per `design@check@a-foreign-build-is-refused`. Two checkouts that share one target
//! directory break that, unless every build is tied to its checkout: the last build writes the
//! one binary both run, and cargo does not rebuild it for the other checkout, whose own package is
//! still fresh. That checkout then runs
//! another checkout's code, and its tool fixtures, exempted under the other checkout's paths, are
//! read as citations.
//!
//! What can be known at run time is where each library the binary links was compiled, and
//! under which package name. A tree that holds the same package at the same place relative to
//! itself, but not at the compiled directory, is a checkout of the same tool: the binary was
//! built from a different one, and the run is refused.
//!
//! A tree that holds no copy of a library is left alone, which covers a mock project under the
//! tool's own directory and a project that consumes the library as a published crate.

use std::path::{Component, Path, PathBuf};

/// One library a binary links: where it was compiled, and its package name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Library {
    /// The library's own crate directory, its `CARGO_MANIFEST_DIR` at build time.
    pub crate_dir: PathBuf,
    /// Its package name, as its `Cargo.toml` declares it.
    pub package: &'static str,
}

/// The first library the tree holds a second copy of, with the copy's directory.
///
/// Every trailing run of the compiled directory's components, one component long or more, is tried
/// under the root, and a directory there whose `Cargo.toml` declares the same package name, and
/// which is not the compiled directory itself, is the copy. A trailing run is where a checkout
/// of the same repository holds the crate, wherever that checkout sits, a worktree nested in
/// the tree included; trying each one needs no knowledge of where the repository's root was.
/// Both sides are canonicalised before they are compared, so a tree reached through a symlink
/// is not its own copy.
///
/// Three limits follow from reading only names and places. A checkout that moved the crate to
/// another relative path is not seen. A tree holding an unrelated crate of the same name at the
/// same relative path is taken for a copy. A copy at the tree's root is not tried, as the comment
/// below says.
pub(crate) fn foreign_copy(root: &Path, libraries: &[Library]) -> Option<(Library, PathBuf)> {
    let canonical = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let root = canonical(root);
    for library in libraries {
        let compiled = canonical(&library.crate_dir);
        let parts: Vec<&std::ffi::OsStr> = compiled
            .components()
            .filter_map(|c| match c {
                Component::Normal(p) => Some(p),
                _ => None,
            })
            .collect();
        // Not the empty run: it would make the root's own `Cargo.toml` a candidate, and refuse
        // every project whose root package shares a name with a linked crate.
        for start in (0..parts.len()).rev() {
            let candidate: PathBuf = parts[start..]
                .iter()
                .fold(root.clone(), |dir, part| dir.join(part));
            if declares(&candidate.join("Cargo.toml"), library.package)
                && canonical(&candidate) != compiled
            {
                return Some((library.clone(), candidate));
            }
        }
    }
    None
}

/// Refuse to run when the tree holds another copy of a library the binary links.
///
/// `packages` is what the refusal tells the user to clean: the binary's own package and every
/// library's, so that the next run rebuilds all of them from this checkout. The refusal also
/// names the prevention: a build tied to its checkout.
///
/// # Errors
///
/// Returns the refusal's message when the tree under `root` holds, at a trailing run of a
/// library's compiled directory, another directory whose `Cargo.toml` declares the same package
/// name: the first such library, in the order of `libraries`, is the one named. A tree that holds
/// no such copy returns `Ok`.
pub fn refuse_a_foreign_build(
    root: &Path,
    libraries: &[Library],
    packages: &[&str],
) -> Result<(), String> {
    let Some((library, copy)) = foreign_copy(root, libraries) else {
        return Ok(());
    };
    // The profile this binary was built in, since `cargo clean -p` clears one profile only.
    let profile = if cfg!(debug_assertions) {
        ""
    } else {
        " --release"
    };
    let clean: String = packages.iter().map(|p| format!(" -p {p}")).collect();
    Err(format!(
        "this binary was built from another checkout: its `{}` was compiled at {}, and the tree \
         being checked holds that package at {}\n       a target directory shared between two \
         checkouts does this; rebuild from this one with `cargo clean{profile}{clean}`\n       \
         to prevent it, tie each build to its checkout: a cargo `[env]` variable valued at the \
         checkout's root, read by every crate and named by every build script, as the \
         knowledge-architect setup skill shows",
        library.package,
        library.crate_dir.display(),
        copy.display(),
    ))
}

/// Whether a `Cargo.toml` declares this package name. A manifest that cannot be read, or that
/// `cargo-reader` refuses, declares none.
fn declares(manifest: &Path, package: &str) -> bool {
    let Ok(text) = std::fs::read_to_string(manifest) else {
        return false;
    };
    knowledge_architect_cargo_reader::read_manifest(&text)
        .is_ok_and(|read| read.package.is_some_and(|p| p.name == package))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::links::link_dir;

    /// A fresh directory under the system's temporary directory, removed when dropped.
    struct Scratch(tempfile::TempDir);

    impl Scratch {
        fn new(tag: &str) -> Self {
            Scratch(
                tempfile::TempDir::with_prefix(format!("klarch-build-origin-{tag}-"))
                    .expect("a scratch directory"),
            )
        }

        fn path(&self, rel: &str) -> PathBuf {
            self.0.path().join(rel)
        }

        fn crate_at(&self, rel: &str, package: &str) -> PathBuf {
            let dir = self.path(rel);
            std::fs::create_dir_all(&dir).expect("a crate directory");
            std::fs::write(
                dir.join("Cargo.toml"),
                format!("[package]\nname = \"{package}\"\n"),
            )
            .expect("a crate manifest");
            dir
        }
    }

    fn library(crate_dir: PathBuf) -> Library {
        Library {
            crate_dir,
            package: "a-tool",
        }
    }

    #[test]
    fn a_second_checkout_holding_the_same_package_is_a_foreign_copy() {
        let scratch = Scratch::new("second-checkout");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        let copy = scratch.crate_at("two/tools/tool/lib", "a-tool");
        let found = foreign_copy(&scratch.path("two"), &[library(built)]);
        assert_eq!(
            found.map(|(_, dir)| dir.canonicalize().expect("the copy")),
            Some(copy.canonicalize().expect("the copy"))
        );
    }

    #[test]
    fn a_library_inside_the_tree_a_tree_inside_it_and_an_unrelated_tree_are_left_alone() {
        let scratch = Scratch::new("left-alone");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        // The ordinary case: the tree holds the compiled directory.
        assert_eq!(
            foreign_copy(&scratch.path("one"), &[library(built.clone())]),
            None
        );
        // A mock project under the tool's own directory holds no copy.
        let mock = scratch.path("one/tools/tool/lib/tests/projects/mock");
        std::fs::create_dir_all(&mock).expect("a mock");
        assert_eq!(foreign_copy(&mock, &[library(built.clone())]), None);
        // A project consuming the library from elsewhere holds no copy either.
        let other = scratch.path("elsewhere");
        std::fs::create_dir_all(&other).expect("a project");
        assert_eq!(foreign_copy(&other, &[library(built.clone())]), None);
        // A crate at the same place under another package name is someone else's.
        scratch.crate_at("three/tools/tool/lib", "another-tool");
        assert_eq!(
            foreign_copy(&scratch.path("three"), &[library(built)]),
            None
        );
    }

    #[test]
    fn a_checkout_nested_in_the_tree_a_deep_crate_and_a_crate_at_the_root_are_found() {
        let scratch = Scratch::new("nested");
        // A worktree nested in the tree: its compiled directory sits under the root, and the
        // root holds the crate at the same trailing run.
        let tree = scratch.path("tree");
        let nested = scratch.crate_at("tree/.worktrees/inner/tools/tool/lib", "a-tool");
        let copy = scratch.crate_at("tree/tools/tool/lib", "a-tool");
        let found = foreign_copy(&tree, &[library(nested)]);
        assert_eq!(
            found.map(|(_, dir)| dir.canonicalize().expect("the copy")),
            Some(copy.canonicalize().expect("the copy"))
        );
        // Five components deep.
        let deep = scratch.crate_at("one/a/b/c/d/lib", "a-tool");
        scratch.crate_at("two/a/b/c/d/lib", "a-tool");
        assert!(foreign_copy(&scratch.path("two"), &[library(deep)]).is_some());
        // A project whose root package shares the name is not taken for a copy: the empty
        // trailing run is not tried.
        let elsewhere = scratch.crate_at("built/a-tool", "a-tool");
        scratch.crate_at("unrelated", "a-tool");
        assert_eq!(
            foreign_copy(&scratch.path("unrelated"), &[library(elsewhere)]),
            None
        );
    }

    /// The claim: the trailing run as long as the compiled directory's whole path is tried too, so
    /// a tree holding the crate at that whole path under its root holds a copy. Mutation checked:
    /// in `foreign_copy`, `for start in (0..parts.len()).rev()` made
    /// `for start in (1..parts.len()).rev()`, which finds no copy; caught.
    #[test]
    fn a_copy_at_the_compiled_directory_s_whole_path_under_the_root_is_found() {
        let scratch = Scratch::new("whole-path");
        let built = scratch.crate_at("one/lib", "a-tool");
        let whole: PathBuf = built
            .canonicalize()
            .expect("the compiled directory")
            .components()
            .filter(|c| matches!(c, Component::Normal(_)))
            .collect();
        let copy = scratch.crate_at(&format!("mirror/{}", whole.display()), "a-tool");
        let found = foreign_copy(&scratch.path("mirror"), &[library(built)]);
        assert_eq!(
            found.map(|(_, dir)| dir.canonicalize().expect("the copy")),
            Some(copy.canonicalize().expect("the copy"))
        );
    }

    /// The claim: a candidate whose `Cargo.toml` cannot be read, is no TOML, or names no package,
    /// with a `[package]` and no `name` or with no `[package]`, is no copy, whatever else it holds,
    /// so a read that fails is never taken for a match. Mutations checked in `declares`, each
    /// caught: `.is_ok_and(|read|` made `.map_or(true, |read|`, and `read.package.is_some_and(`
    /// made `read.package.is_none_or(`.
    #[test]
    fn a_candidate_whose_manifest_cannot_be_read_or_names_no_package_is_no_copy() {
        let scratch = Scratch::new("unreadable");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        // A directory where the manifest would be: the read fails.
        std::fs::create_dir_all(scratch.path("dir/tools/tool/lib/Cargo.toml"))
            .expect("a directory");
        for (tree, text) in [
            ("broken", "[package\nname = \"a-tool\"\n"),
            ("nameless", "[package]\ndescription = \"a-tool\"\n"),
            ("packageless", "[workspace]\nmembers = [\"a-tool\"]\n"),
        ] {
            let dir = scratch.path(&format!("{tree}/tools/tool/lib"));
            std::fs::create_dir_all(&dir).expect("a crate directory");
            std::fs::write(dir.join("Cargo.toml"), text).expect("a crate manifest");
        }
        for tree in ["dir", "broken", "nameless", "packageless"] {
            assert_eq!(
                foreign_copy(&scratch.path(tree), &[library(built.clone())]),
                None,
                "{tree}"
            );
        }
    }

    /// The claim: a candidate is a copy when its manifest declares the package name exactly, and
    /// whatever Cargo accepts beside it, such as a `version` table whose `workspace` is `true` with
    /// another key; a name the package's name only begins is someone else's. Mutations checked,
    /// each caught: in `field` of `path@cargo-reader@src/manifest.rs`, the arm for a table whose
    /// `workspace` is `true` given the guard `if inline.len() == 1`; in `declares`,
    /// `p.name == package` made `p.name.starts_with(package)`.
    #[test]
    fn a_candidate_is_a_copy_when_its_manifest_declares_the_exact_name() {
        let scratch = Scratch::new("exact-name");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        for (tree, text) in [
            (
                "inheriting",
                "[package]\nname = \"a-tool\"\nversion = { workspace = true, other = 1 }\n",
            ),
            ("longer", "[package]\nname = \"a-tool-fork\"\n"),
        ] {
            let dir = scratch.path(&format!("{tree}/tools/tool/lib"));
            std::fs::create_dir_all(&dir).expect("a crate directory");
            std::fs::write(dir.join("Cargo.toml"), text).expect("a crate manifest");
        }
        assert!(
            foreign_copy(&scratch.path("inheriting"), &[library(built.clone())]).is_some(),
            "a manifest Cargo accepts declares its name"
        );
        assert_eq!(
            foreign_copy(&scratch.path("longer"), &[library(built)]),
            None,
            "a longer name is another package"
        );
    }

    #[test]
    fn a_crate_compiled_through_a_symlink_is_not_a_copy_of_itself() {
        // `cargo build --manifest-path` through a symlinked path compiles that path in.
        let scratch = Scratch::new("compiled-symlink");
        scratch.crate_at("one/tools/tool/lib", "a-tool");
        let link = scratch.path("link");
        link_dir(scratch.path("one"), &link);
        let through = library(link.join("tools/tool/lib"));
        assert_eq!(foreign_copy(&scratch.path("one"), &[through]), None);
    }

    #[test]
    fn a_tree_reached_through_a_symlink_is_not_its_own_copy() {
        let scratch = Scratch::new("symlink");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        let link = scratch.path("link");
        link_dir(scratch.path("one"), &link);
        assert_eq!(foreign_copy(&link, &[library(built)]), None);
        // A symlink inside the tree that resolves to the compiled directory is the same
        // directory, not a copy.
        let real = scratch.crate_at("tree/real/tool/lib", "a-tool");
        link_dir(scratch.path("tree/real/tool"), scratch.path("tree/tool"));
        assert_eq!(foreign_copy(&scratch.path("tree"), &[library(real)]), None);
    }

    #[test]
    fn the_refusal_names_both_directories_every_package_to_clean_and_the_prevention() {
        let scratch = Scratch::new("refusal");
        let built = scratch.crate_at("one/tools/tool/lib", "a-tool");
        scratch.crate_at("two/tools/tool/lib", "a-tool");
        let why = refuse_a_foreign_build(
            &scratch.path("two"),
            &[library(built.clone())],
            &["a-binary", "a-tool"],
        )
        .expect_err("a refusal");
        assert!(why.contains(&built.display().to_string()), "{why}");
        assert!(why.contains("two"), "{why}");
        let profile = if cfg!(debug_assertions) {
            ""
        } else {
            " --release"
        };
        assert!(
            why.contains(&format!("cargo clean{profile} -p a-binary -p a-tool")),
            "{why}"
        );
        assert!(why.contains("tie each build to its checkout"), "{why}");
        assert_eq!(
            refuse_a_foreign_build(&scratch.path("one"), &[library(built)], &["a-tool"]),
            Ok(())
        );
    }
}
