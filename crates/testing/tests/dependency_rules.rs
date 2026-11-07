//! Rules of the dependency graph a build would not report, read from cargo: a dependency added against
//! one of them passes every other gate.

use std::path::Path;
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

/// The claim of `design@knowledge-architect@testing-crate`: no crate of the workspace names this one, a
/// dev-dependency included, so the graph holds no cycle and a build pulls in nothing of it.
#[test]
fn no_crate_of_the_workspace_depends_on_this_one() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = std::fs::read_to_string(root.join("Cargo.toml")).expect("the workspace's manifest");
    let workspace: toml::Table = text.parse().expect("TOML");
    let members = workspace["workspace"]["members"]
        .as_array()
        .expect("members");
    assert!(members.len() > 5, "{members:?}");
    for member in members {
        let member = member.as_str().expect("a path");
        if member == "crates/testing" {
            continue;
        }
        let text =
            std::fs::read_to_string(root.join(member).join("Cargo.toml")).expect("a manifest");
        let manifest: toml::Table = text.parse().expect("TOML");
        for table in ["dependencies", "dev-dependencies", "build-dependencies"] {
            let names = manifest.get(table).and_then(|t| t.as_table());
            assert!(
                !names.is_some_and(|t| t.contains_key("knowledge-architect-testing")),
                "{member} names this crate in {table}"
            );
        }
    }
}
