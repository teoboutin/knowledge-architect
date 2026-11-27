//! The read of a `Cargo.toml` over this repository's own crates, which the library reads as any
//! project's.

use knowledge_architect_cargo_reader::{read_manifest, Field};
use std::path::Path;

/// The claim: every crate under crates/ yields its package name, the workspace's version through
/// `version.workspace = true`, and `publish` unset. The version is compared with this crate's own,
/// which Cargo resolved from the same workspace. Mutation checked: in `inherited` of
/// `path@cargo-reader@src/manifest.rs`, `Field::Workspace => workspace.clone().map(Field::Set)`
/// made `Field::Workspace => Some(Field::Absent)`; caught.
#[test]
fn every_crate_of_the_workspace_yields_its_name_the_workspace_version_and_no_publish() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |path: &Path| {
        let text =
            std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        read_manifest(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    };
    let workspace = read(&root.join("Cargo.toml"))
        .workspace
        .expect("the root manifest holds the workspace")
        .package;
    let mut crates = 0;
    for entry in std::fs::read_dir(root.join("crates")).expect("crates/ is readable") {
        let dir = entry.expect("an entry of crates/").path();
        let manifest = dir.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        crates += 1;
        let package = read(&manifest)
            .package
            .unwrap_or_else(|| panic!("{} has no package", manifest.display()))
            .inherit(&workspace)
            .unwrap_or_else(|e| panic!("{}: {e}", manifest.display()));
        // The package names follow the directories, the binary's crate taking the project's name.
        let basename = dir
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a UTF-8 name");
        let name = match basename {
            "cli" => "knowledge-architect".to_string(),
            other => format!("knowledge-architect-{other}"),
        };
        assert_eq!(package.name, name, "{}", manifest.display());
        assert_eq!(
            package.version,
            Field::Set(env!("CARGO_PKG_VERSION").to_string()),
            "{}",
            manifest.display()
        );
        assert_eq!(package.publish, Field::Absent, "{}", manifest.display());
    }
    assert!(crates > 0, "no crate was read under crates/");
}
