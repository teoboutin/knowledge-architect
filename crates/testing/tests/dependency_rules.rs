//! Rules of the dependency graph a build would not report, read from cargo: a dependency added against
//! one of them passes every other gate.

use std::process::Command;

/// The packages a normal dependency of `package` reaches, as `cargo tree` prints them. A dev-dependency
/// builds into no consumer, so it does not count.
fn normal_tree(package: &str) -> String {
    let cargo = std::env::var_os("CARGO").expect("cargo sets CARGO for a test it runs");
    let out = Command::new(cargo)
        .args(["tree", "-e", "normal", "-p", package])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The claim of `design@model@no-check-below-the-model` and `design@query@query-depends-on-no-check`:
/// the tools over the record need the model and no check.
#[test]
fn no_crate_below_the_tools_depends_on_the_checks() {
    for package in ["knowledge-architect-model", "knowledge-architect-query"] {
        let tree = normal_tree(package);
        assert!(!tree.contains("knowledge-architect-check"), "{tree}");
    }
}

/// The claim: `cli` and the checks hold nothing of this crate, so a binary that links them builds none
/// of it, per `design@knowledge-architect@testing-crate`.
#[test]
fn no_crate_depends_on_this_one() {
    for package in ["knowledge-architect", "knowledge-architect-check"] {
        let tree = normal_tree(package);
        assert!(!tree.contains("knowledge-architect-testing"), "{tree}");
    }
}
