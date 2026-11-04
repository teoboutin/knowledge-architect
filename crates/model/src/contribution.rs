//! What an extension gives phase 1, as data, per `design@model@contribution-is-data`.

use std::path::PathBuf;

use crate::finding::Finding;
use crate::manifest::{Manifest, MANIFEST_NAME};

/// The phase-1 contribution of every extension of a binary.
#[derive(Debug, Default)]
pub struct Contribution {
    /// The top-level manifest tables they read.
    pub tables: Vec<&'static str>,
    /// Declarations they refused, each a finding of phase 1. A refused declaration is absent from
    /// their configuration, as the core's are.
    pub complaints: Vec<Finding>,
    /// The paths they declare, by the label a finding names them with. Phase 2 asserts that each
    /// exists, with the core's own declared paths.
    pub paths: Vec<(String, Vec<PathBuf>)>,
    /// The files they generate. The walk leaves them out, and they are read as committed.
    pub generated: Vec<PathBuf>,
}

/// Record a contribution in the manifest. Call it once, before `Model::build`, which leaves the
/// generated files out of the walk: a second call adds again, and a tool that never calls it has every
/// extension table of the manifest reported.
///
/// **A table no extension claims, and a claimed table the manifest does not hold, are phase-1
/// findings**, per `design@check@an-extension-claims-its-manifest-tables`. The first is a
/// declaration nothing reads, which a session would take for a regime in force; the second would
/// let a manifest switch an extension off by leaving its table out. They come before the
/// extensions' own complaints.
pub fn contribute(manifest: &mut Manifest, contribution: Contribution) {
    let Contribution {
        tables,
        complaints: theirs,
        paths,
        generated,
    } = contribution;
    let mut complaints = Vec::new();
    for table in manifest.extension_tables() {
        if !tables.contains(&table) {
            let what = if manifest.table(table).is_some_and(toml::Value::is_table) {
                format!("[{table}] is a table no extension of this binary reads")
            } else {
                format!("`{table}` is a top-level key no extension of this binary reads")
            };
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                what,
                "delete it, or run the binary that registers the extension it belongs to; a \
                 declaration nothing reads looks like a check in force",
            ));
        }
    }
    let present: Vec<String> = manifest.extension_tables().map(str::to_string).collect();
    for table in &tables {
        if !present.iter().any(|p| p == table) {
            complaints.push(Finding::in_file(
                MANIFEST_NAME,
                format!("[{table}] is missing, and an extension this binary registers reads it"),
                "declare it; a manifest cannot switch an extension off by leaving its table out",
            ));
        }
    }
    complaints.extend(theirs);
    manifest.record_extensions(complaints, paths, generated);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "{extra}\n[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The claim: phase 1 is driven by a value: a claimed table present is silent, one absent is
    /// reported, an unclaimed one is reported, and the extensions' own complaints follow them.
    #[test]
    fn a_contribution_is_judged_with_no_extension() {
        let mut m = manifest("[rules]\nx = 1\n\n[other]\ny = 2\n");
        contribute(
            &mut m,
            Contribution {
                tables: vec!["rules", "absent"],
                complaints: vec![Finding::in_file(MANIFEST_NAME, "theirs", "repair")],
                paths: vec![("a label".to_string(), vec![PathBuf::from("a/path")])],
                generated: vec![PathBuf::from("gen.md")],
            },
        );
        assert_eq!(
            m.extension_paths(),
            [("a label".to_string(), vec![PathBuf::from("a/path")])]
        );
        assert_eq!(m.extension_generated(), [PathBuf::from("gen.md")]);
        let whats: Vec<&str> = m.complaints().iter().map(|f| f.what.as_str()).collect();
        assert_eq!(
            whats,
            [
                "[other] is a table no extension of this binary reads",
                "[absent] is missing, and an extension this binary registers reads it",
                "theirs",
            ]
        );
    }
}
