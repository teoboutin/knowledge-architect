//! The refusal a command prints when an extension cannot prepare over the working tree.

use std::path::{Path, PathBuf};

use knowledge_architect::extension::{DumpRow, Extension, Prepared, Purpose, Resolution, Tree};
use knowledge_architect::{cli, Manifest, Model};

/// An extension that can prepare for nothing, with its own reason.
struct NeverPrepares;

impl Extension for NeverPrepares {
    fn tables(&self) -> &'static [&'static str] {
        &[]
    }

    fn resolve(&mut self, _: &Manifest) -> Resolution {
        Resolution::default()
    }

    fn checks(&self) -> &'static [&'static str] {
        &[]
    }

    fn dump(&self, _: &Model) -> Vec<DumpRow> {
        Vec::new()
    }

    fn prepare(
        &mut self,
        _: &Manifest,
        _: &Model,
        _: Tree<'_>,
        _: Purpose,
    ) -> Result<Box<dyn Prepared>, String> {
        Err("the extension's own reason".into())
    }
}

/// Copy `from` into `to`, recursively.
fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the destination");
    for entry in std::fs::read_dir(from).expect("a readable directory") {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copy");
        }
    }
}

/// Run git in `dir`, with the per-user ignore file pinned away, as the binary tests do.
fn git(dir: &Path, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(["-c", "core.excludesFile=/dev/null"])
        .args(args)
        .current_dir(dir)
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?}");
}

/// The claim: `check` and `index` over the working tree refuse an extension that cannot prepare
/// with its reason and, under it, the repair, as the commands over a staged tree do. Mutation
/// checked: `extension_refusal` returning the reason alone fails both cases.
#[test]
fn an_extension_that_cannot_prepare_is_refused_with_its_repair() {
    let root = std::env::temp_dir().join(format!("ka-unpreparable-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/projects/dirhome"),
        &root,
    );
    git(&root, &["init", "-q"]);
    git(&root, &["add", "-A"]);
    let manifest = Manifest::load(&root).expect("the copy's manifest");
    let commands = [
        cli::Command::Check(cli::CheckArgs {
            fix: false,
            staged: false,
        }),
        cli::Command::Index(cli::IndexArgs { staged: false }),
    ];
    for command in commands {
        let mut extensions: Vec<Box<dyn Extension>> = vec![Box::new(NeverPrepares)];
        let error =
            cli::run(command, &manifest, &[], &mut extensions).expect_err("the command refuses");
        assert_eq!(
            error,
            "the extension's own reason\n       repair what the extension's reason names, and \
             run again"
        );
    }
    let _ = std::fs::remove_dir_all(&root);
}
