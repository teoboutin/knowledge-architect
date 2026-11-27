//! Which Components hold a changelog, read from each one's `Cargo.toml` through `cargo-reader`.
//!
//! **Read before the walk.** The assembled file is a generated path, outside the walk, only where
//! two changelog crates or more are assembled, and which Components are changelog crates is
//! what their `Cargo.toml` says where `[changelog] components` lists none. So the read happens
//! where the model is built, from the listing and the reader of the tree the model is built from,
//! the disk or a snapshot's blobs, so `check`, `check --staged` and `commits` read one set.
//!
//! **A `Cargo.toml` this read cannot use is a fact kept for the last phase, never a failure of
//! the build**: the model is built whatever a manifest holds, so a parent tree of `commits`
//! assembles over it.

use std::path::{Path, PathBuf};

use knowledge_architect_cargo_reader::{read_manifest, Field, Package, Publish, WorkspacePackage};

use super::CHANGELOG_FILE;
use crate::manifest::Manifest;

/// A Cargo manifest's name, in a Component's directory.
const CARGO_FILE: &str = "Cargo.toml";

/// One changelog crate: a Component whose crate holds a changelog.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crate {
    /// The Component's name.
    pub component: String,
    /// The Component's directory, project-relative, empty at the root.
    pub dir: PathBuf,
    /// The package name its `Cargo.toml` declares, as it is published.
    pub package: String,
    /// Whether its `version` is inherited from the workspace, `version.workspace = true`.
    pub inherits_version: bool,
    /// Its `include` patterns, inherited ones resolved, or `None` where it sets none.
    pub include: Option<Vec<String>>,
}

impl Crate {
    /// The path of its changelog.
    pub fn changelog(&self) -> PathBuf {
        self.dir.join(CHANGELOG_FILE)
    }

    /// The path of its `Cargo.toml`.
    pub fn manifest(&self) -> PathBuf {
        self.dir.join(CARGO_FILE)
    }
}

/// Why a Component's `Cargo.toml` gives the changelog checks nothing they can use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CargoTrouble {
    /// `cargo-reader` refused it, or the read of its bytes failed, with the reason.
    Unreadable {
        /// The `Cargo.toml`.
        file: PathBuf,
        /// Why.
        reason: String,
    },
    /// A `[walk]` row keeps it out, so it is read as it is in no tree.
    KeptOut {
        /// The `Cargo.toml`.
        file: PathBuf,
        /// The row, as a finding names it.
        row: String,
    },
    /// `[changelog] components` lists the Component, and its directory holds no `Cargo.toml`.
    Missing {
        /// The `Cargo.toml` that is not there.
        file: PathBuf,
    },
    /// `[changelog] components` lists the Component, and its `Cargo.toml` declares no package.
    NoPackage {
        /// The `Cargo.toml`.
        file: PathBuf,
    },
}

/// The changelog crates of a project, and what kept a Component from being judged one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Crates {
    crates: Vec<Crate>,
    trouble: Vec<CargoTrouble>,
}

impl Crates {
    /// The changelog crates, in the manifest's order of the Components.
    pub fn all(&self) -> &[Crate] {
        &self.crates
    }

    /// What kept a Component's `Cargo.toml` from being read, each a finding of the last phase.
    pub fn trouble(&self) -> &[CargoTrouble] {
        &self.trouble
    }

    /// The path of the assembled file, where one is assembled: the manifest declares
    /// `[changelog]` and two changelog crates or more are read. With one, nothing is assembled.
    pub fn assembled<'a>(&self, manifest: &'a Manifest) -> Option<&'a Path> {
        let table = manifest.changelog()?;
        (self.crates.len() >= 2).then(|| table.assembled())
    }
}

/// The changelog crates of the tree whose listing is `listing`, each file's text given by `read`,
/// which answers `None` for bytes it could not read as text. Empty where the manifest declares no
/// `[changelog]`.
///
/// What [`Model::build`](crate::Model::build) reads from the disk, for a caller that builds a
/// model from a snapshot's blobs: it hands the result to
/// [`generated_paths`](crate::index::generated_paths) before the walk, and to
/// [`Model::set_changelog_crates`](crate::Model::set_changelog_crates) after.
pub fn read(
    manifest: &Manifest,
    listing: &[PathBuf],
    read: &dyn Fn(&Path) -> Option<String>,
) -> Crates {
    let mut out = Crates::default();
    let Some(table) = manifest.changelog() else {
        return out;
    };
    let walk = manifest.walk();
    let listed = !table.components().is_empty();
    for component in manifest.components().all() {
        if listed && !table.components().contains(&component.name) {
            continue;
        }
        let file = component.path.join(CARGO_FILE);
        if !listing.contains(&file) {
            if listed {
                out.trouble.push(CargoTrouble::Missing { file });
            }
            continue;
        }
        if let Some(row) = crate::walk::keeping_row(&file, walk) {
            out.trouble.push(CargoTrouble::KeptOut { file, row });
            continue;
        }
        let unreadable = |reason: String| CargoTrouble::Unreadable {
            file: file.clone(),
            reason,
        };
        let Some(text) = read(&file) else {
            out.trouble.push(unreadable(
                "its bytes cannot be read as UTF-8 text".to_string(),
            ));
            continue;
        };
        let package = match read_manifest(&text) {
            Ok(manifest) => manifest.package,
            Err(e) => {
                out.trouble.push(unreadable(e.to_string()));
                continue;
            }
        };
        let Some(package) = package else {
            if listed {
                out.trouble.push(CargoTrouble::NoPackage { file });
            }
            continue;
        };
        let inherits_version = package.version == Field::Workspace;
        let resolved = match inherited(&package, &component.path, listing, walk, read) {
            Ok(resolved) => resolved,
            Err(reason) => {
                out.trouble.push(unreadable(reason));
                continue;
            }
        };
        if !listed && !published(&resolved) {
            continue;
        }
        out.crates.push(Crate {
            component: component.name.clone(),
            dir: component.path.clone(),
            package: resolved.name.clone(),
            inherits_version,
            include: match resolved.include {
                Field::Set(patterns) => Some(patterns),
                _ => None,
            },
        });
    }
    out
}

/// Whether Cargo would publish the package, as
/// `path@cargo-reader@docs/grounding/cargo-manifest.md` reads `version` and `publish`: a `version`
/// set, and `publish` absent, `true`, or a list naming a registry. An empty list names none and is
/// read as `false`, a reading that grounding file records.
fn published(package: &Package) -> bool {
    matches!(package.version, Field::Set(_))
        && match &package.publish {
            Field::Absent | Field::Set(Publish::Flag(true)) => true,
            Field::Set(Publish::Registries(names)) => !names.is_empty(),
            _ => false,
        }
}

/// The package with every inherited field resolved from the workspace's `[workspace.package]`:
/// that of the nearest `Cargo.toml` at or above the Component's directory, within the project,
/// that holds a `[workspace]` table, per `design@cargo-reader@the-caller-finds-the-workspace-manifest`.
/// A manifest on the way that a `[walk]` row keeps out, or that cannot be read, is passed over.
fn inherited(
    package: &Package,
    dir: &Path,
    listing: &[PathBuf],
    walk: &crate::manifest::Walk,
    read: &dyn Fn(&Path) -> Option<String>,
) -> Result<Package, String> {
    let inherits = [
        package.version == Field::Workspace,
        package.publish == Field::Workspace,
        package.repository == Field::Workspace,
        package.include == Field::Workspace,
    ];
    if !inherits.contains(&true) {
        return Ok(package.clone());
    }
    let workspace = dir
        .ancestors()
        .map(|d| d.join(CARGO_FILE))
        .filter(|f| listing.contains(f) && !crate::walk::kept_out(f, walk))
        .find_map(|f| read(&f).and_then(|t| read_manifest(&t).ok()?.workspace));
    let workspace: WorkspacePackage = workspace.ok_or_else(|| {
        "it inherits a field from its workspace, and no `Cargo.toml` at or above its directory \
         within the project holds a `[workspace]` table"
            .to_string()
    })?;
    package.inherit(&workspace).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const WORKSPACE: &str =
        "[workspace]\nmembers = [\"a\"]\n\n[workspace.package]\nversion = \"0.3.0\"\n";

    /// The changelog crates of a project whose manifest's `[changelog]` table is `table`, its
    /// Components `a`, `b` and `c`, each holding the `Cargo.toml` `files` gives.
    fn crates_of(table: &str, files: &[(&str, &str)]) -> Crates {
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            &format!(
                "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\", \"c\"]\n\n\
                 [walk]\nskip-dirs = []\nskip-files = [\"c/Cargo.toml\"]\n\n[changelog]\ncategories = [\"A\"]\n{table}"
            ),
        )
        .expect("a declaration");
        let files: HashMap<PathBuf, String> = files
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let listing: Vec<PathBuf> = files.keys().cloned().collect();
        read(&manifest, &listing, &|rel| files.get(rel).cloned())
    }

    fn names(crates: &Crates) -> Vec<&str> {
        crates.all().iter().map(|c| c.component.as_str()).collect()
    }

    /// The claim: with `components` absent, a package is a changelog crate
    /// where Cargo would publish it, its `version` its own or inherited, and an empty `publish`
    /// list, a `publish = false` and a missing `version` make none. Mutations checked, each caught:
    /// in `published`, `Field::Set(Publish::Registries(names)) => !names.is_empty(),` made
    /// `Field::Set(Publish::Registries(_)) => true,`; `Field::Absent | Field::Set(Publish::Flag(true)) => true,`
    /// made `Field::Set(Publish::Flag(true)) => true,`; and the test of the `version`,
    /// `matches!(package.version, Field::Set(_))`, made `true`.
    #[test]
    fn a_component_is_a_changelog_crate_where_cargo_would_publish_it() {
        let crates = |b: &str| {
            crates_of(
                "",
                &[
                    ("Cargo.toml", WORKSPACE),
                    (
                        "a/Cargo.toml",
                        "[package]\nname = \"pa\"\nversion.workspace = true\n",
                    ),
                    ("b/Cargo.toml", b),
                ],
            )
        };
        assert_eq!(
            names(&crates(
                "[package]\nname = \"pb\"\nversion = \"1.0.0\"\npublish = [\"r\"]\n"
            )),
            ["a", "b"]
        );
        assert_eq!(
            names(&crates(
                "[package]\nname = \"pb\"\nversion = \"1.0.0\"\npublish = true\n"
            )),
            ["a", "b"]
        );
        assert_eq!(
            names(&crates("[package]\nname = \"pb\"\nversion = \"1.0.0\"\n")),
            ["a", "b"]
        );
        assert_eq!(
            names(&crates(
                "[package]\nname = \"pb\"\nversion = \"1.0.0\"\npublish = []\n"
            )),
            ["a"]
        );
        assert_eq!(
            names(&crates(
                "[package]\nname = \"pb\"\nversion = \"1.0.0\"\npublish = false\n"
            )),
            ["a"]
        );
        assert_eq!(names(&crates("[package]\nname = \"pb\"\n")), ["a"]);
        let all = crates("[package]\nname = \"pb\"\n");
        assert!(all.all()[0].inherits_version);
        assert_eq!(all.all()[0].package, "pa");
    }

    /// The claim: with `components` listed, exactly those are read,
    /// whatever `publish` says, and a listed one with no `Cargo.toml` or no package is trouble.
    /// Mutation checked: `if !listed && !published(&resolved)` made `if !published(&resolved)`;
    /// caught.
    #[test]
    fn a_listed_component_is_read_whatever_cargo_would_publish() {
        let crates = crates_of(
            "components = [\"a\", \"b\"]\n",
            &[(
                "a/Cargo.toml",
                "[package]\nname = \"pa\"\npublish = false\n",
            )],
        );
        assert_eq!(names(&crates), ["a"]);
        assert_eq!(
            crates.trouble(),
            [CargoTrouble::Missing {
                file: PathBuf::from("b/Cargo.toml")
            }]
        );
    }

    /// The claim: a `Cargo.toml` that `cargo-reader` refuses, one that
    /// inherits from no workspace, and one a `[walk]` row keeps out are each trouble carrying its
    /// reason, and no crate. Mutation checked: in `read`,
    /// `out.trouble.push(unreadable(e.to_string()));` made `let _ = e;`; caught.
    #[test]
    fn a_cargo_toml_the_read_cannot_use_is_trouble_with_its_reason() {
        let crates = crates_of(
            "",
            &[
                ("a/Cargo.toml", "[package\n"),
                (
                    "b/Cargo.toml",
                    "[package]\nname = \"pb\"\nversion.workspace = true\n",
                ),
                (
                    "c/Cargo.toml",
                    "[package]\nname = \"pc\"\nversion = \"1.0.0\"\n",
                ),
            ],
        );
        assert!(crates.all().is_empty(), "{crates:?}");
        let trouble = crates.trouble();
        assert_eq!(trouble.len(), 3, "{trouble:?}");
        assert!(
            matches!(&trouble[0], CargoTrouble::Unreadable { file, .. } if file == Path::new("a/Cargo.toml"))
        );
        assert!(
            matches!(&trouble[1], CargoTrouble::Unreadable { reason, .. } if reason.contains("[workspace]"))
        );
        assert!(
            matches!(&trouble[2], CargoTrouble::KeptOut { row, .. } if row.contains("skip-files"))
        );
    }
}
