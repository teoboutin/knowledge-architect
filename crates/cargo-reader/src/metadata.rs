//! The run of `cargo metadata --format-version 1`, and the read of its output: the resolved
//! dependency graph, and each of its packages' name, version, manifest path and `repository`.
//!
//! The output is read as `path@cargo-reader@docs/grounding/cargo-metadata.md` records it. This
//! module is the one place of the library code of the published crates that runs
//! `cargo metadata` or reads its output, per
//! `design@knowledge-architect@tools-read-cargo-through-the-crate`, which leaves out of that rule
//! the runs of Cargo by `gates` and `mutate`.

use serde::Deserialize;
use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// What `cargo metadata` reports, of what the reader reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    /// The packages of the resolved graph, in the order of the output's `packages`. When the output
    /// carries no graph, as under `--no-deps`, every package it lists: the workspace's members.
    pub packages: Vec<MetadataPackage>,
    /// The resolved dependency graph, `None` when the output carries none, as under `--no-deps`.
    pub graph: Option<Graph>,
}

/// One package `cargo metadata` lists.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct MetadataPackage {
    /// Its package ID, an opaque string by which the graph names it.
    pub id: String,
    /// `name`.
    pub name: String,
    /// `version`.
    pub version: String,
    /// The absolute path of its `Cargo.toml`: in the workspace, at a path dependency, or in the
    /// unpacked source of a package from a registry or a git repository.
    pub manifest_path: PathBuf,
    /// `repository`, `None` where its manifest sets none.
    pub repository: Option<String>,
}

/// The resolved dependency graph.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Graph {
    /// Every package of the graph.
    pub nodes: Vec<Node>,
    /// The package of the directory cargo ran in, if `--manifest-path` is not given, and that
    /// manifest's package where it is; `None` in a virtual workspace.
    pub root: Option<String>,
}

/// One package of the graph, with the packages it depends on.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Node {
    /// Its package ID.
    pub id: String,
    /// The package IDs of its dependencies, of every kind and every platform the run covers.
    pub dependencies: Vec<String>,
}

/// Why the run or the read failed.
#[derive(Debug)]
pub enum MetadataError {
    /// Cargo could not be started.
    Spawn {
        /// The command line, as it was run: the program, the path `CARGO` names or else `cargo`,
        /// and its arguments.
        command: String,
        /// The error of the spawn.
        error: std::io::Error,
    },
    /// Cargo ran and failed.
    Failed {
        /// The command line, as it was run, named as in [`MetadataError::Spawn`].
        command: String,
        /// Its exit status.
        status: std::process::ExitStatus,
        /// What it wrote on its standard error, which holds Cargo's reason.
        stderr: String,
    },
    /// The output is not the JSON of format version 1 this reader reads.
    Output(String),
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetadataError::Spawn { command, error } => {
                write!(f, "`{command}` did not start: {error}")
            }
            MetadataError::Failed {
                command,
                status,
                stderr,
            } => write!(f, "`{command}` failed ({status}): {}", stderr.trim_end()),
            MetadataError::Output(why) => {
                write!(f, "the output of `cargo metadata` could not be read: {why}")
            }
        }
    }
}

impl std::error::Error for MetadataError {}

/// Run `cargo metadata --format-version 1` in `dir`, with `flags` after it, and read its output.
///
/// The Cargo that runs is the one named by the variable `CARGO`, which Cargo sets for the programs
/// it runs, or else `cargo` on the path. The run inherits the caller's environment, so Cargo's own
/// configuration and variables apply; whether it writes `Cargo.lock` or reaches the network is
/// what the caller's `flags`, such as `--locked` and `--offline`, say.
///
/// # Errors
///
/// Returns [`MetadataError::Spawn`] when Cargo cannot be started, [`MetadataError::Failed`] with
/// Cargo's standard error when it exits with a failure, as in a directory that holds no Cargo
/// project, and [`MetadataError::Output`] when what it prints cannot be read, per
/// [`read_metadata`]. A failure is never an empty graph.
pub fn run_metadata(dir: &Path, flags: &[&str]) -> Result<Metadata, MetadataError> {
    // `CARGO` names the Cargo that runs the caller, under `cargo run` and `cargo test`, so the
    // metadata is that toolchain's; outside Cargo, the one on the path.
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut words = vec!["metadata", "--format-version", "1"];
    words.extend_from_slice(flags);
    // The program as it is run, `$CARGO` where it is set, so a failure names the Cargo that failed.
    let command = format!("{} {}", cargo.to_string_lossy(), words.join(" "));
    let output = Command::new(cargo)
        .args(&words)
        .current_dir(dir)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| MetadataError::Spawn {
            command: command.clone(),
            error,
        })?;
    if !output.status.success() {
        return Err(MetadataError::Failed {
            command,
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    let json = std::str::from_utf8(&output.stdout)
        .map_err(|error| MetadataError::Output(error.to_string()))?;
    read_metadata(json)
}

/// The fields of the output this reader reads; serde ignores the others, as the format adds
/// fields within a version.
#[derive(Deserialize)]
struct Output {
    version: u64,
    packages: Vec<MetadataPackage>,
    resolve: Option<Graph>,
}

/// Read the JSON that `cargo metadata --format-version 1` prints.
///
/// # Errors
///
/// Returns [`MetadataError::Output`] when the text is no JSON, lacks a field read here other than
/// a package's `repository` and the output's `resolve`, holds one of another type, or states a
/// format version other than 1. A missing `repository` or `resolve` reads as `None`, as a `null`
/// does.
pub fn read_metadata(json: &str) -> Result<Metadata, MetadataError> {
    let output: Output =
        serde_json::from_str(json).map_err(|error| MetadataError::Output(error.to_string()))?;
    if output.version != 1 {
        return Err(MetadataError::Output(format!(
            "format version {}, where 1 was asked for",
            output.version
        )));
    }
    let packages = match &output.resolve {
        // Cargo lists in `packages` the packages of the graph, measured as the same set in this
        // repository with and without `--filter-platform`; the filter keeps that true of an
        // output where the two differ, so a package is yielded only as a part of the graph.
        Some(graph) => {
            let ids: HashSet<&str> = graph.nodes.iter().map(|node| node.id.as_str()).collect();
            output
                .packages
                .into_iter()
                .filter(|package| ids.contains(package.id.as_str()))
                .collect()
        }
        None => output.packages,
    };
    Ok(Metadata {
        packages,
        graph: output.resolve,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An output of the shape Cargo prints, trimmed to a workspace member, one dependency from a
    /// registry, and a package listed that no node of the graph holds.
    const OUTPUT: &str = r#"{
        "packages": [
            {
                "name": "a-member",
                "version": "0.10.0",
                "id": "path+file:///work/crates/a-member#0.10.0",
                "license": "MIT",
                "source": null,
                "manifest_path": "/work/crates/a-member/Cargo.toml",
                "repository": "https://example.org/work",
                "publish": null
            },
            {
                "name": "a-dependency",
                "version": "1.2.3",
                "id": "registry+https://github.com/rust-lang/crates.io-index#a-dependency@1.2.3",
                "source": "registry+https://github.com/rust-lang/crates.io-index",
                "manifest_path": "/home/u/.cargo/registry/src/index/a-dependency-1.2.3/Cargo.toml",
                "repository": null
            },
            {
                "name": "not-in-the-graph",
                "version": "0.1.0",
                "id": "registry+https://github.com/rust-lang/crates.io-index#not-in-the-graph@0.1.0",
                "manifest_path": "/home/u/.cargo/registry/src/index/not-in-the-graph-0.1.0/Cargo.toml",
                "repository": "https://example.org/other"
            }
        ],
        "workspace_members": ["path+file:///work/crates/a-member#0.10.0"],
        "resolve": {
            "nodes": [
                {
                    "id": "path+file:///work/crates/a-member#0.10.0",
                    "dependencies": ["registry+https://github.com/rust-lang/crates.io-index#a-dependency@1.2.3"],
                    "deps": [],
                    "features": []
                },
                {
                    "id": "registry+https://github.com/rust-lang/crates.io-index#a-dependency@1.2.3",
                    "dependencies": [],
                    "features": ["default"]
                }
            ],
            "root": null
        },
        "target_directory": "/work/target",
        "version": 1,
        "workspace_root": "/work",
        "metadata": null
    }"#;

    /// Claim 2: the graph's packages are yielded, each with its manifest path and its
    /// `repository`, and a package no node holds is not. Mutation checked: in `read_metadata`,
    /// `.filter(|package| ids.contains(package.id.as_str()))` made `.filter(|_| true)`; caught.
    #[test]
    fn the_graph_s_packages_are_read_with_their_manifest_path_and_repository() {
        let metadata = read_metadata(OUTPUT).expect("the fixture is read");
        let names: Vec<&str> = metadata.packages.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["a-member", "a-dependency"]);
        let member = &metadata.packages[0];
        assert_eq!(member.version, "0.10.0");
        assert_eq!(
            member.manifest_path,
            PathBuf::from("/work/crates/a-member/Cargo.toml")
        );
        assert_eq!(
            member.repository.as_deref(),
            Some("https://example.org/work")
        );
        let dependency = &metadata.packages[1];
        assert_eq!(
            dependency.manifest_path,
            PathBuf::from("/home/u/.cargo/registry/src/index/a-dependency-1.2.3/Cargo.toml")
        );
        assert_eq!(dependency.repository, None);
        let graph = metadata.graph.expect("the fixture has a graph");
        assert_eq!(graph.root, None);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(
            graph.nodes[0].dependencies,
            std::slice::from_ref(&dependency.id)
        );
    }

    /// Claim 2, an output with no graph, as `--no-deps` prints: every package listed, and no graph.
    /// Mutations checked, each caught: in `read_metadata`, `None => output.packages,` made
    /// `None => Vec::new(),`, and made `None => output.packages.into_iter().take(1).collect(),`.
    #[test]
    fn an_output_with_no_graph_yields_every_package_listed() {
        let json = r#"{"packages": [{"name": "a", "version": "1.0.0", "id": "a-id",
            "manifest_path": "/w/a/Cargo.toml", "repository": null},
            {"name": "b", "version": "1.0.0", "id": "b-id",
            "manifest_path": "/w/b/Cargo.toml", "repository": null}],
            "resolve": null, "version": 1}"#;
        let metadata = read_metadata(json).expect("the fixture is read");
        assert_eq!(metadata.graph, None);
        let names: Vec<&str> = metadata.packages.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["a", "b"]);
    }

    /// Claim 2, `resolve.root` is read: the package of the directory Cargo ran in. Mutation
    /// checked: `#[serde(skip)]` added to `Graph::root`; caught.
    #[test]
    fn the_graph_s_root_is_read() {
        let json = r#"{"packages": [{"name": "a", "version": "1.0.0", "id": "a-id",
            "manifest_path": "/w/a/Cargo.toml", "repository": null}],
            "resolve": {"nodes": [{"id": "a-id", "dependencies": []}], "root": "a-id"},
            "version": 1}"#;
        let graph = read_metadata(json)
            .expect("the fixture is read")
            .graph
            .expect("the fixture has a graph");
        assert_eq!(graph.root.as_deref(), Some("a-id"));
    }

    /// Claim 2, a key the reader reads as an `Option`, a package's `repository` or the output's
    /// `resolve`, reads as `None` where the output omits it, rather than as an error. A regression
    /// guard: serde's derive reads a missing `Option` field as `None`, which its first run
    /// measured, and the `# Errors` of `read_metadata` states.
    #[test]
    fn a_missing_optional_key_reads_as_none() {
        let json = r#"{"packages": [{"name": "a", "version": "1.0.0", "id": "a-id",
            "manifest_path": "/w/a/Cargo.toml"}], "version": 1}"#;
        let metadata = read_metadata(json).expect("the fixture is read");
        assert_eq!(metadata.graph, None);
        assert_eq!(metadata.packages[0].repository, None);
    }

    /// Claim 3, the caller's flags reach Cargo: with `--no-deps`, a package's metadata carries no
    /// graph, and without it, a graph whose root is that package. Mutation checked: in
    /// `run_metadata`, `words.extend_from_slice(flags);` made `let _ = flags;`; caught.
    #[test]
    fn the_caller_s_flags_reach_cargo() {
        let dir = tempfile::TempDir::with_prefix("klarch-cargo-reader-flags-")
            .expect("a scratch directory");
        std::fs::create_dir(dir.path().join("src")).expect("a source directory");
        std::fs::write(dir.path().join("src/lib.rs"), "").expect("a library");
        std::fs::write(
            dir.path().join("Cargo.toml"),
            "[package]\nname = \"a-crate\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .expect("a manifest");
        let no_deps = run_metadata(dir.path(), &["--offline", "--no-deps"]).expect("cargo runs");
        assert_eq!(no_deps.graph, None);
        assert_eq!(no_deps.packages.len(), 1);
        let graph = run_metadata(dir.path(), &["--offline"])
            .expect("cargo runs")
            .graph
            .expect("a graph without `--no-deps`");
        assert_eq!(graph.root.as_deref(), Some(no_deps.packages[0].id.as_str()));
    }

    /// Claim 2, an output this reader cannot read is an error: no JSON, a missing field, another
    /// format version, above or below 1. Mutations checked, each caught: in `read_metadata`,
    /// `if output.version != 1 {` made `if false {`, and made `if output.version > 1 {`.
    #[test]
    fn an_output_of_another_shape_is_an_error() {
        for json in [
            "not json",
            r#"{"resolve": null, "version": 1}"#,
            r#"{"packages": [{"name": "a"}], "resolve": null, "version": 1}"#,
            r#"{"packages": [], "resolve": null, "version": 2}"#,
            r#"{"packages": [], "resolve": null, "version": 0}"#,
        ] {
            assert!(read_metadata(json).is_err(), "{json} was read");
        }
    }

    /// Claim 3: a run where Cargo fails is an error carrying Cargo's message, never an empty
    /// graph. Mutation checked: in `run_metadata`, `if !output.status.success()` made
    /// `if false`; caught.
    #[test]
    fn a_run_outside_a_cargo_project_is_cargo_s_error() {
        let dir = tempfile::TempDir::with_prefix("klarch-cargo-reader-no-project-")
            .expect("a scratch directory");
        let why = run_metadata(dir.path(), &["--offline"]).expect_err("no Cargo project");
        assert!(
            matches!(why, MetadataError::Failed { .. }),
            "not cargo's failure: {why}"
        );
        assert!(
            why.to_string().contains("could not find `Cargo.toml`"),
            "{why}"
        );
    }
}
