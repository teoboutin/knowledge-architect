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

use knowledge_architect_cargo_reader::{read_manifest, Field, Package, Publish, Readme, Workspace};

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
    /// Its `exclude` patterns, inherited ones resolved, or `None` where it sets none.
    pub exclude: Option<Vec<String>>,
    /// The file its `readme` names, project-relative, an inherited one read from the workspace's
    /// root; `None` where it names no path, or one outside the project.
    pub readme: Option<PathBuf>,
    /// The file its `license-file` names, read as `readme` is.
    pub license_file: Option<PathBuf>,
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

/// The changelog crates of the tree whose listing is `listing`, the files of that tree and no
/// symlink or gitlink, each file's text given by `read`. Empty where the manifest declares no
/// `[changelog]`.
///
/// `read` answers an error of the kind `NotFound` for a listed file the tree does not hold, such
/// as one deleted in the working tree and not yet staged, which is read as a file the listing
/// lacks; `InvalidData` for bytes that are not UTF-8 text; and any other error for a read that
/// failed, which is trouble carrying the error.
///
/// What [`Model::build`](crate::Model::build) reads from the disk, for a caller that builds a
/// model from a snapshot's blobs: it hands the result to
/// [`generated_paths`](crate::index::generated_paths) before the walk, and to
/// [`Model::set_changelog_crates`](crate::Model::set_changelog_crates) after.
pub fn read(
    manifest: &Manifest,
    listing: &[PathBuf],
    read: &dyn Fn(&Path) -> std::io::Result<String>,
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
        let missing = |out: &mut Crates, file: PathBuf| {
            if listed {
                out.trouble.push(CargoTrouble::Missing { file });
            }
        };
        if !listing.contains(&file) {
            missing(&mut out, file);
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
        let text = match read(&file) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                missing(&mut out, file);
                continue;
            }
            Err(e) => {
                out.trouble.push(unreadable(match e.kind() {
                    std::io::ErrorKind::InvalidData => {
                        "its bytes cannot be read as UTF-8 text".to_string()
                    }
                    _ => format!("it cannot be read: {e}"),
                }));
                continue;
            }
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
            readme: match &resolved.readme {
                Field::Set(Readme::Path(path)) => normalised(&component.path.join(path)),
                _ => None,
            },
            license_file: match &resolved.license_file {
                Field::Set(path) => normalised(&component.path.join(path)),
                _ => None,
            },
            include: match resolved.include {
                Field::Set(patterns) => Some(patterns),
                _ => None,
            },
            exclude: match resolved.exclude {
                Field::Set(patterns) => Some(patterns),
                _ => None,
            },
        });
    }
    out
}

/// Whether Cargo would publish the package, as `path@model@docs/grounding/cargo-packages.md` reads
/// `version` and `publish`: a `version`
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

/// The package with every inherited field resolved from the workspace's `[workspace.package]`,
/// the workspace Cargo uses, per `design@cargo-reader@the-caller-finds-the-workspace-manifest`, as
/// `cargo metadata` was measured to find it, per
/// `path@model@docs/grounding/cargo-packages.md`: the directory `package.workspace` names
/// where it is set; otherwise the nearest at or above the Component's directory, within the
/// project, whose `Cargo.toml` holds a `[workspace]` table that does not exclude the package. An
/// inherited `readme` or `license-file` is a path from that workspace's root, and is rebased to
/// the Component's directory. A manifest on the way that a `[walk]` row keeps out, or that cannot
/// be read, is passed over. Cargo refuses a workspace that does not list the package among its
/// members; that is not judged here, and the values are read from the workspace Cargo finds.
fn inherited(
    package: &Package,
    dir: &Path,
    listing: &[PathBuf],
    walk: &crate::manifest::Walk,
    read: &dyn Fn(&Path) -> std::io::Result<String>,
) -> Result<Package, String> {
    let inherits = [
        package.version == Field::Workspace,
        package.publish == Field::Workspace,
        package.repository == Field::Workspace,
        package.include == Field::Workspace,
        package.exclude == Field::Workspace,
        package.readme == Field::Workspace,
        package.license_file == Field::Workspace,
    ];
    if !inherits.contains(&true) {
        return Ok(package.clone());
    }
    let workspace_at = |d: &Path| -> Option<Workspace> {
        let file = d.join(CARGO_FILE);
        if !listing.contains(&file) || crate::walk::kept_out(&file, walk) {
            return None;
        }
        read(&file)
            .ok()
            .and_then(|t| read_manifest(&t).ok()?.workspace)
    };
    let (root, workspace) = match &package.workspace {
        Some(path) => {
            let root = normalised(&dir.join(path)).ok_or_else(|| {
                format!("its `package.workspace`, `{path}`, names a directory outside the project")
            })?;
            let workspace = workspace_at(&root).ok_or_else(|| {
                format!(
                    "it inherits a field from its workspace, and the `Cargo.toml` of `{path}`, \
                     which its `package.workspace` names, holds no `[workspace]` table"
                )
            })?;
            (root, workspace)
        }
        None => dir
            .ancestors()
            .find_map(|d| {
                let workspace = workspace_at(d)?;
                let member = dir.strip_prefix(d).unwrap_or(dir);
                (d == dir || !excludes(&workspace, member)).then(|| (d.to_path_buf(), workspace))
            })
            .ok_or_else(|| {
                "it inherits a field from its workspace, and no `Cargo.toml` at or above its \
                 directory within the project holds a `[workspace]` table that does not exclude it"
                    .to_string()
            })?,
    };
    let mut resolved = package
        .inherit(&workspace.package)
        .map_err(|e| e.to_string())?;
    // The inherited path names a file from the workspace's root; the Component's own is read from
    // its directory, so both become project-relative where `read` joins them.
    let from_root = |path: &str| {
        relative(dir, &root)
            .join(path)
            .to_string_lossy()
            .replace('\\', "/")
    };
    if package.readme == Field::Workspace {
        if let Field::Set(Readme::Path(path)) = &resolved.readme {
            resolved.readme = Field::Set(Readme::Path(from_root(path)));
        }
    }
    if package.license_file == Field::Workspace {
        if let Field::Set(path) = &resolved.license_file {
            resolved.license_file = Field::Set(from_root(path));
        }
    }
    Ok(resolved)
}

/// Whether the workspace excludes the package at `member`, its directory relative to the
/// workspace's: an `exclude` entry names that directory or one above it, and no `members` entry
/// does, each entry read as a literal path, so a glob in `members` names nothing, as measured.
fn excludes(workspace: &Workspace, member: &Path) -> bool {
    let covers = |entries: &Option<Vec<String>>| {
        entries
            .iter()
            .flatten()
            .any(|e| normalised(Path::new(e)).is_some_and(|p| member.starts_with(p)))
    };
    covers(&workspace.exclude) && !covers(&workspace.members)
}

/// `path` with each `.` taken off and each `..` taking off the segment before it, or `None` where
/// a `..` would leave the project, or where the path is absolute.
fn normalised(path: &Path) -> Option<PathBuf> {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !out.pop() {
                    return None;
                }
            }
            std::path::Component::Normal(name) => out.push(name),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => return None,
        }
    }
    Some(out)
}

/// The path from the directory `from` to the directory `to`, both project-relative and
/// normalised.
fn relative(from: &Path, to: &Path) -> PathBuf {
    let common = from
        .components()
        .zip(to.components())
        .take_while(|(a, b)| a == b)
        .count();
    let mut out = PathBuf::new();
    for _ in common..from.components().count() {
        out.push("..");
    }
    for component in to.components().skip(common) {
        out.push(component);
    }
    out
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
        read(&manifest, &listing, &|rel| {
            files
                .get(rel)
                .cloned()
                .ok_or_else(|| std::io::ErrorKind::NotFound.into())
        })
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

    /// The claim: a listed `Cargo.toml` the tree does not hold, as one deleted and not staged, is
    /// read as one the listing lacks: no crate, and trouble only where `[changelog] components`
    /// lists its Component; one whose bytes are not text is trouble saying so. Mutation checked:
    /// in `read`, the arm `Err(e) if e.kind() == std::io::ErrorKind::NotFound => {` made
    /// `Err(e) if false => {`; caught.
    #[test]
    fn a_cargo_toml_the_tree_does_not_hold_is_read_as_unlisted() {
        let manifest = |table: &str| {
            Manifest::parse(
                Path::new("/nowhere"),
                &format!(
                    "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\"]\n\n\
                     [walk]\nskip-dirs = []\nskip-files = []\n\n[changelog]\ncategories = [\"A\"]\n{table}"
                ),
            )
            .expect("a declaration")
        };
        let listing = [PathBuf::from("a/Cargo.toml"), PathBuf::from("b/Cargo.toml")];
        let reader = |rel: &Path| -> std::io::Result<String> {
            Err(if rel.starts_with("a") {
                std::io::ErrorKind::NotFound.into()
            } else {
                std::io::ErrorKind::InvalidData.into()
            })
        };
        let unlisted = read(&manifest(""), &listing, &reader);
        assert!(unlisted.all().is_empty());
        assert_eq!(unlisted.trouble().len(), 1, "{unlisted:?}");
        assert!(matches!(
            &unlisted.trouble()[0],
            CargoTrouble::Unreadable { file, reason }
                if file == Path::new("b/Cargo.toml") && reason.contains("UTF-8")
        ));
        let listed = read(&manifest("components = [\"a\"]\n"), &listing, &reader);
        assert_eq!(
            listed.trouble(),
            [CargoTrouble::Missing {
                file: PathBuf::from("a/Cargo.toml")
            }]
        );
    }

    /// The changelog crates of a project whose one Component is `x/a`, each file of `files` in
    /// its listing.
    fn nested(files: &[(&str, &str)]) -> Crates {
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"x/a\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n[changelog]\ncategories = [\"A\"]\n",
        )
        .expect("a declaration");
        let files: HashMap<PathBuf, String> = files
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let listing: Vec<PathBuf> = files.keys().cloned().collect();
        read(&manifest, &listing, &|rel| {
            files
                .get(rel)
                .cloned()
                .ok_or_else(|| std::io::ErrorKind::NotFound.into())
        })
    }

    /// A workspace manifest whose `[workspace]` table holds `keys`, its `include` naming `tag`.
    fn workspace(keys: &str, tag: &str) -> String {
        format!(
            "[workspace]\n{keys}\n\n[workspace.package]\nversion = \"0.1.0\"\ninclude = [\"{tag}\"]\n"
        )
    }

    const MEMBER: &str =
        "[package]\nname = \"pa\"\nversion.workspace = true\ninclude.workspace = true\n";

    fn include_of(crates: &Crates) -> Vec<String> {
        crates.all()[0].include.clone().unwrap_or_default()
    }

    /// The claim: the workspace a package inherits from is the nearest at or above it that does
    /// not exclude it, an `exclude` entry naming its directory or one above it, written with a
    /// `./` or a trailing `/` or not, and a `members` entry naming it overriding the `exclude`,
    /// a glob in `members` naming nothing, as `cargo metadata` was measured to find it. Mutations
    /// checked, each caught: in `inherited`, `(d == dir || !excludes(&workspace, member))` made
    /// `(d == dir || true)`; in `excludes`, `covers(&workspace.exclude) &&
    /// !covers(&workspace.members)` made `covers(&workspace.exclude)`; and in `normalised`, the
    /// `CurDir` arm made `std::path::Component::CurDir => out.push("."),`.
    #[test]
    fn the_workspace_inherited_from_is_the_nearest_that_does_not_exclude_the_package() {
        let root = workspace("members = [\"x/a\"]", "root");
        for (inner, want) in [
            ("exclude = [\"a\"]", "root"),
            ("exclude = [\"./a/\"]", "root"),
            ("exclude = [\".\"]", "root"),
            ("exclude = [\"ab\"]", "inner"),
            ("exclude = [\"*\"]", "inner"),
            ("members = [\"a\"]\nexclude = [\"a\"]", "inner"),
            ("members = [\"*\"]\nexclude = [\"a\"]", "root"),
            ("members = [\"a\"]", "inner"),
        ] {
            let crates = nested(&[
                ("Cargo.toml", &root),
                ("x/Cargo.toml", &workspace(inner, "inner")),
                ("x/a/Cargo.toml", MEMBER),
            ]);
            assert_eq!(include_of(&crates), [want], "{inner}");
        }
    }

    /// The claim: `package.workspace` names the workspace, the nearer one passed over, and one
    /// that names a directory outside the project, or one whose `Cargo.toml` holds no
    /// `[workspace]`, is trouble with its reason. Mutation checked: in `inherited`,
    /// `let (root, workspace) = match &package.workspace {` made
    /// `let (root, workspace) = match &None::<String> {`; caught.
    #[test]
    fn package_workspace_names_the_workspace() {
        let member = |path: &str| format!("{MEMBER}workspace = \"{path}\"\n");
        let crates = nested(&[
            ("Cargo.toml", &workspace("members = [\"x/a\"]", "root")),
            ("x/Cargo.toml", &workspace("members = [\"a\"]", "inner")),
            ("x/a/Cargo.toml", &member("../..")),
        ]);
        assert_eq!(include_of(&crates), ["root"]);
        for path in ["../../..", "."] {
            let crates = nested(&[
                ("Cargo.toml", &workspace("members = [\"x/a\"]", "root")),
                ("x/a/Cargo.toml", &member(path)),
            ]);
            assert!(crates.all().is_empty(), "{path}: {crates:?}");
            assert!(
                matches!(&crates.trouble()[0], CargoTrouble::Unreadable { reason, .. } if reason.contains("package.workspace")),
                "{path}: {crates:?}"
            );
        }
    }

    /// The claim: the file `readme` and `license-file` name is project-relative, read from the
    /// Component's directory where the package sets it and from the workspace's root where it
    /// inherits it, as Cargo reads an inherited one, measured with `cargo package`; one outside
    /// the project names none. Mutation checked: in `inherited`, `relative(dir, &root)` made
    /// `PathBuf::new()`, so an inherited path is read from the Component's directory; caught.
    #[test]
    fn readme_and_license_file_name_a_file_from_where_cargo_reads_them() {
        let crates = nested(&[
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"x/a\"]\n\n[workspace.package]\nversion = \"0.1.0\"\n\
                 readme = \"CHANGELOG.md\"\nlicense-file = \"x/a/CHANGELOG.md\"\n",
            ),
            (
                "x/a/Cargo.toml",
                "[package]\nname = \"pa\"\nversion.workspace = true\nreadme.workspace = true\n\
                 license-file.workspace = true\n",
            ),
        ]);
        let a = &crates.all()[0];
        assert_eq!(a.readme, Some(PathBuf::from("CHANGELOG.md")));
        assert_eq!(a.license_file, Some(PathBuf::from("x/a/CHANGELOG.md")));
        let crates = nested(&[(
            "x/a/Cargo.toml",
            "[package]\nname = \"pa\"\nversion = \"0.1.0\"\nreadme = \"./docs/../CHANGELOG.md\"\n\
             license-file = \"../../../LICENSE\"\n",
        )]);
        let a = &crates.all()[0];
        assert_eq!(a.readme, Some(PathBuf::from("x/a/CHANGELOG.md")));
        assert_eq!(a.license_file, None);
    }
}
