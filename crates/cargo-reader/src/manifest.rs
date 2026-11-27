//! The read of a `Cargo.toml`'s text: the facts of `[package]` the tools of knowledge-architect
//! use, and the keys of `[workspace.package]` a package inherits them from.
//!
//! Cargo's manifest format is read as `path@cargo-reader@docs/grounding/cargo-manifest.md` records
//! it. This module is the one reader of that format in the library code of the published crates,
//! per `design@knowledge-architect@tools-read-cargo-through-the-crate`.

use std::fmt;

/// What a manifest declares, of the tables the reader reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    /// The `[package]` table, or the `[project]` table where the manifest holds no `[package]`, as
    /// Cargo reads it, per `design@cargo-reader@project-table-read-as-the-package`; absent from a
    /// virtual workspace's manifest.
    pub package: Option<Package>,
    /// The `[workspace]` table, `None` for a manifest that holds none.
    pub workspace: Option<Workspace>,
}

/// What a `[workspace]` table declares, of the keys the reader reads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Workspace {
    /// `members`, the paths and the glob patterns as written, `None` where the table does not set
    /// it.
    pub members: Option<Vec<String>>,
    /// `exclude`, the paths as written, `None` where the table does not set it.
    pub exclude: Option<Vec<String>>,
    /// The `[workspace.package]` keys, each `None` where that table does not set it, or where the
    /// workspace holds no such table.
    pub package: WorkspacePackage,
}

/// The facts of a `[package]` table, each as the manifest states it.
///
/// A field set to a table whose `workspace` is `true`, such as `{ workspace = true }`, is
/// [`Field::Workspace`] until [`Package::inherit`] replaces it with the workspace's value. No
/// default of Cargo's is applied: an absent `version` is [`Field::Absent`], which Cargo reads as
/// `0.0.0` and as a package it does not publish.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Package {
    /// `name`, which Cargo requires of every `[package]`.
    pub name: String,
    /// `version`.
    pub version: Field<String>,
    /// `publish`.
    pub publish: Field<Publish>,
    /// `repository`.
    pub repository: Field<String>,
    /// `include`, its patterns as written.
    pub include: Field<Vec<String>>,
    /// `exclude`, its patterns as written.
    pub exclude: Field<Vec<String>>,
    /// `readme`.
    pub readme: Field<Readme>,
    /// `license-file`, the path as written.
    pub license_file: Field<String>,
    /// `workspace`: the path of the workspace's root directory, relative to the package's, as
    /// written, `None` where it is not set. It is no field `[workspace.package]` can supply.
    pub workspace: Option<String>,
}

/// The value of `readme`, as the manifest writes it.
///
/// A path names the file relative to the package's root, or, inherited from `[workspace.package]`,
/// relative to the workspace's root, as Cargo reads it; `true` and an absent `readme` both have
/// Cargo look for a file named `README.md`, `README.txt` or `README`, and `false` names none.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Readme {
    /// `readme = true` or `readme = false`.
    Flag(bool),
    /// `readme = "<path>"`.
    Path(String),
}

/// The keys of `[workspace.package]` that the fields of [`Package`] can inherit.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorkspacePackage {
    /// `version`.
    pub version: Option<String>,
    /// `publish`.
    pub publish: Option<Publish>,
    /// `repository`.
    pub repository: Option<String>,
    /// `include`.
    pub include: Option<Vec<String>>,
    /// `exclude`.
    pub exclude: Option<Vec<String>>,
    /// `readme`, a path relative to the workspace's root.
    pub readme: Option<Readme>,
    /// `license-file`, a path relative to the workspace's root.
    pub license_file: Option<String>,
}

/// A field of `[package]` that `[workspace.package]` can supply.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Field<T> {
    /// The manifest does not set it.
    Absent,
    /// The manifest sets it to this value.
    Set(T),
    /// The manifest sets it to a table whose `workspace` is `true`, such as `{ workspace = true }`,
    /// so its value is the workspace's.
    Workspace,
}

/// The value of `publish`, as the manifest writes it.
///
/// Cargo reads `false` and an empty list alike, as a package published to no registry. It reads
/// `true` as a package with no restriction, the same as an absent `publish` when `version` is set;
/// with no `version`, an absent `publish` is a package it does not publish, and `true` is refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Publish {
    /// `publish = true` or `publish = false`.
    Flag(bool),
    /// `publish = [...]`: the registries the package may be published to, by name.
    Registries(Vec<String>),
}

/// Why a manifest's text could not be read: no TOML, or a table or a field this reader reads that
/// holds a value of a type Cargo refuses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestError {
    message: String,
}

impl ManifestError {
    fn new(message: impl Into<String>) -> Self {
        ManifestError {
            message: message.into(),
        }
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ManifestError {}

/// Read a `Cargo.toml`'s text.
///
/// Only the tables and the fields [`Manifest`] holds are read; every other key is ignored, whether
/// Cargo knows it or not. Inheritance is not resolved here: [`Package::inherit`] does it, given the
/// workspace's manifest, which the caller finds.
///
/// An `Ok` does not mean that Cargo accepts the manifest. The read checks the type of each field
/// on its own, and no combination of fields or of values: `publish = true` with no `version`, or a
/// `[project]` table under the 2024 edition, both refused by Cargo, are read without error.
///
/// # Errors
///
/// Returns a [`ManifestError`] when the text is no TOML; when `[package]` or `[workspace]` is not
/// a table, or `[workspace.package]` is not one; when `[package]` has no `name` that is a string;
/// when `package.workspace` is no string, or `workspace.members` or `workspace.exclude` no list of
/// strings;
/// and when a field read here holds a value of a type Cargo refuses for it, a table whose
/// `workspace` is `false` and a table with no `workspace` key included.
pub fn read_manifest(text: &str) -> Result<Manifest, ManifestError> {
    let table: toml::Table = text
        .parse()
        .map_err(|error: toml::de::Error| ManifestError::new(error.message().to_string()))?;
    // Cargo reads the `[project]` table, the older name of `[package]`, where the manifest holds no
    // `[package]`, as `path@cargo-reader@docs/grounding/cargo-manifest.md` records measured, per
    // `design@cargo-reader@project-table-read-as-the-package`.
    let package = match table.get("package").or_else(|| table.get("project")) {
        None => None,
        Some(value) => Some(package(value)?),
    };
    let workspace = match table.get("workspace") {
        None => None,
        Some(value) => Some(workspace(value)?),
    };
    Ok(Manifest { package, workspace })
}

fn package(value: &toml::Value) -> Result<Package, ManifestError> {
    let table = value
        .as_table()
        .ok_or_else(|| ManifestError::new("`package` is not a table"))?;
    let name = match table.get("name") {
        Some(toml::Value::String(name)) => name.clone(),
        Some(_) => return Err(ManifestError::new("`package.name` is not a string")),
        None => return Err(ManifestError::new("missing field `package.name`")),
    };
    Ok(Package {
        name,
        version: field(table, "version", string)?,
        publish: field(table, "publish", publish)?,
        repository: field(table, "repository", string)?,
        include: field(table, "include", strings)?,
        exclude: field(table, "exclude", strings)?,
        readme: field(table, "readme", readme)?,
        license_file: field(table, "license-file", string)?,
        workspace: plain(table, "package", "workspace", string)?,
    })
}

fn workspace(value: &toml::Value) -> Result<Workspace, ManifestError> {
    let workspace = value
        .as_table()
        .ok_or_else(|| ManifestError::new("`workspace` is not a table"))?;
    Ok(Workspace {
        members: plain(workspace, "workspace", "members", strings)?,
        exclude: plain(workspace, "workspace", "exclude", strings)?,
        package: workspace_package(workspace)?,
    })
}

/// A key no table can supply, as written, `None` where it is not set.
fn plain<T>(
    table: &toml::Table,
    name: &str,
    key: &str,
    read: fn(&toml::Value) -> Option<T>,
) -> Result<Option<T>, ManifestError> {
    table
        .get(key)
        .map(|value| read(value).ok_or_else(|| wrong_type(name, key)))
        .transpose()
}

fn workspace_package(workspace: &toml::Table) -> Result<WorkspacePackage, ManifestError> {
    let Some(value) = workspace.get("package") else {
        return Ok(WorkspacePackage::default());
    };
    let table = value
        .as_table()
        .ok_or_else(|| ManifestError::new("`workspace.package` is not a table"))?;
    // Each key is read with the reader of the field that inherits it, so a value the field would
    // refuse is refused here too; `{ workspace = true }` has no meaning in the workspace's own
    // table, and is refused as a value of the wrong type.
    Ok(WorkspacePackage {
        version: plain(table, "workspace.package", "version", string)?,
        publish: plain(table, "workspace.package", "publish", publish)?,
        repository: plain(table, "workspace.package", "repository", string)?,
        include: plain(table, "workspace.package", "include", strings)?,
        exclude: plain(table, "workspace.package", "exclude", strings)?,
        readme: plain(table, "workspace.package", "readme", readme)?,
        license_file: plain(table, "workspace.package", "license-file", string)?,
    })
}

/// A field of `[package]`: absent, a value `read` accepts, or a table whose `workspace` is `true`,
/// the one table Cargo accepts for an inheritable field of `[package]`.
fn field<T>(
    table: &toml::Table,
    key: &str,
    read: fn(&toml::Value) -> Option<T>,
) -> Result<Field<T>, ManifestError> {
    let Some(value) = table.get(key) else {
        return Ok(Field::Absent);
    };
    if let Some(inline) = value.as_table() {
        // Cargo reads the table by its `workspace` key and ignores any other key beside it, as
        // `path@cargo-reader@docs/grounding/cargo-manifest.md` records measured.
        return match inline.get("workspace") {
            Some(toml::Value::Boolean(true)) => Ok(Field::Workspace),
            Some(toml::Value::Boolean(false)) => Err(ManifestError::new(format!(
                "`package.{key}`: `workspace` cannot be false"
            ))),
            _ => Err(wrong_type("package", key)),
        };
    }
    read(value)
        .map(Field::Set)
        .ok_or_else(|| wrong_type("package", key))
}

fn wrong_type(table: &str, key: &str) -> ManifestError {
    ManifestError::new(format!(
        "`{table}.{key}` holds a value of a type Cargo refuses"
    ))
}

fn string(value: &toml::Value) -> Option<String> {
    value.as_str().map(str::to_string)
}

fn strings(value: &toml::Value) -> Option<Vec<String>> {
    value
        .as_array()?
        .iter()
        .map(|item| item.as_str().map(str::to_string))
        .collect()
}

fn readme(value: &toml::Value) -> Option<Readme> {
    match value {
        toml::Value::Boolean(flag) => Some(Readme::Flag(*flag)),
        _ => string(value).map(Readme::Path),
    }
}

fn publish(value: &toml::Value) -> Option<Publish> {
    match value {
        toml::Value::Boolean(flag) => Some(Publish::Flag(*flag)),
        _ => strings(value).map(Publish::Registries),
    }
}

impl Package {
    /// This package with every [`Field::Workspace`] replaced by the workspace's value, as Cargo
    /// resolves `{ workspace = true }`; a field set or absent is left as it is.
    ///
    /// `workspace` is the `[workspace.package]` of the workspace's root manifest, which the caller
    /// finds: Cargo takes the manifest named by `package.workspace`, or else the nearest manifest
    /// with a `[workspace]` table in a parent directory that does not exclude the package, a
    /// lookup this function does not make. Each value is taken as the workspace writes it: an
    /// inherited `readme` or `license-file` stays relative to the workspace's root, where Cargo
    /// reads it, and the caller rebases it where it needs the path from the package's root.
    ///
    /// # Errors
    ///
    /// Returns a [`ManifestError`] naming the key when a field inherits a key that `workspace`
    /// does not set, as Cargo refuses such a manifest.
    pub fn inherit(&self, workspace: &WorkspacePackage) -> Result<Package, ManifestError> {
        Ok(Package {
            name: self.name.clone(),
            version: inherited(&self.version, &workspace.version, "version")?,
            publish: inherited(&self.publish, &workspace.publish, "publish")?,
            repository: inherited(&self.repository, &workspace.repository, "repository")?,
            include: inherited(&self.include, &workspace.include, "include")?,
            exclude: inherited(&self.exclude, &workspace.exclude, "exclude")?,
            readme: inherited(&self.readme, &workspace.readme, "readme")?,
            license_file: inherited(&self.license_file, &workspace.license_file, "license-file")?,
            workspace: self.workspace.clone(),
        })
    }
}

fn inherited<T: Clone>(
    field: &Field<T>,
    workspace: &Option<T>,
    key: &str,
) -> Result<Field<T>, ManifestError> {
    match field {
        Field::Workspace => workspace.clone().map(Field::Set).ok_or_else(|| {
            ManifestError::new(format!(
                "`package.{key}` inherits `workspace.package.{key}`, which the workspace does not set"
            ))
        }),
        other => Ok(other.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package_of(text: &str) -> Package {
        read_manifest(text)
            .expect("the fixture is read")
            .package
            .expect("the fixture has a package")
    }

    const NAMED: &str = "[package]\nname = \"a-crate\"\n";

    /// The claim, the facts of `[package]` a manifest states directly. Mutation checked: in
    /// `field`, `.map(Field::Set)` made `.map(|_| Field::Absent)`; caught.
    #[test]
    fn a_package_s_facts_are_read_as_the_manifest_states_them() {
        let package = package_of(
            "[package]\nname = \"a-crate\"\nversion = \"1.2.3\"\n\
             repository = \"https://example.org/a\"\ninclude = [\"/src/**\", \"/README.md\"]\n",
        );
        assert_eq!(package.name, "a-crate");
        assert_eq!(package.version, Field::Set("1.2.3".to_string()));
        assert_eq!(
            package.repository,
            Field::Set("https://example.org/a".to_string())
        );
        assert_eq!(
            package.include,
            Field::Set(vec!["/src/**".to_string(), "/README.md".to_string()])
        );
        assert_eq!(package.publish, Field::Absent);
    }

    /// The claim, an absent `version`, `repository` and `include`: no default of Cargo's applied.
    /// Mutation checked: in `field`, `return Ok(Field::Absent);` made
    /// `return Ok(Field::Workspace);`; caught.
    #[test]
    fn an_absent_field_is_absent() {
        let package = package_of(NAMED);
        assert_eq!(package.version, Field::Absent);
        assert_eq!(package.repository, Field::Absent);
        assert_eq!(package.include, Field::Absent);
        assert_eq!(package.publish, Field::Absent);
    }

    /// The claim, each shape of `publish`. Mutations checked, each caught: in `publish`,
    /// `Some(Publish::Flag(*flag))` made `Some(Publish::Flag(true))`; in `field`, `(1,
    /// Some(toml::Value::Boolean(true))) => Ok(Field::Workspace),` made `=> Ok(Field::Absent),`.
    #[test]
    fn each_shape_of_publish_is_read() {
        let publish = |value: &str| package_of(&format!("{NAMED}publish = {value}\n")).publish;
        assert_eq!(publish("false"), Field::Set(Publish::Flag(false)));
        assert_eq!(publish("true"), Field::Set(Publish::Flag(true)));
        assert_eq!(publish("[]"), Field::Set(Publish::Registries(vec![])));
        assert_eq!(
            publish("[\"a-registry\"]"),
            Field::Set(Publish::Registries(vec!["a-registry".to_string()]))
        );
        assert_eq!(publish("{ workspace = true }"), Field::Workspace);
    }

    /// The claim, a manifest with no `[package]`: a virtual workspace's. Mutation checked: in
    /// `workspace_package`, `return Ok(WorkspacePackage::default());` made
    /// `return Err(ManifestError::new("x"));`; caught.
    #[test]
    fn a_manifest_without_a_package_has_none() {
        let manifest =
            read_manifest("[workspace]\nmembers = [\"a\"]\n").expect("the fixture is read");
        assert_eq!(manifest.package, None);
        assert_eq!(
            manifest.workspace,
            Some(Workspace {
                members: Some(vec!["a".to_string()]),
                ..Workspace::default()
            })
        );
        let manifest = read_manifest("[dependencies]\n").expect("the fixture is read");
        assert_eq!(
            manifest,
            Manifest {
                package: None,
                workspace: None
            }
        );
    }

    /// The claim, `version.workspace = true` and `repository.workspace = true` beside a workspace
    /// manifest: each takes the workspace's value, and a field the member sets keeps its own.
    /// Mutation checked: in `inherited`, `Field::Workspace => workspace.clone().map(Field::Set)`
    /// made `Field::Workspace => Some(Field::Workspace)`; caught.
    #[test]
    fn a_field_inherited_from_the_workspace_takes_its_value() {
        let workspace = read_manifest(
            "[workspace]\nmembers = [\"a\"]\n\n[workspace.package]\nversion = \"0.10.0\"\n\
             repository = \"https://example.org/w\"\ninclude = [\"/src/**\"]\n",
        )
        .expect("the workspace manifest is read")
        .workspace
        .expect("it has a workspace")
        .package;
        let member = package_of(
            "[package]\nname = \"a\"\nversion.workspace = true\n\
             repository = { workspace = true }\ninclude = [\"/lib/**\"]\n",
        );
        assert_eq!(member.version, Field::Workspace);
        assert_eq!(member.repository, Field::Workspace);
        let resolved = member.inherit(&workspace).expect("every key is set");
        assert_eq!(resolved.version, Field::Set("0.10.0".to_string()));
        assert_eq!(
            resolved.repository,
            Field::Set("https://example.org/w".to_string())
        );
        assert_eq!(resolved.include, Field::Set(vec!["/lib/**".to_string()]));
        assert_eq!(resolved.publish, Field::Absent);
    }

    /// The claim, an inherited key the workspace does not set is an error naming it, as Cargo's
    /// "error inheriting `publish` from workspace root manifest's `workspace.package.publish`".
    /// Mutation checked: in `inherited`, `.map(Field::Set).ok_or_else(|| {` made
    /// `.map(Field::Set).or(Some(Field::Absent)).ok_or_else(|| {`; caught.
    #[test]
    fn a_field_inheriting_a_key_the_workspace_lacks_is_an_error() {
        let member = package_of(&format!("{NAMED}publish.workspace = true\n"));
        let why = member
            .inherit(&WorkspacePackage::default())
            .expect_err("the workspace sets no publish");
        assert!(
            why.to_string().contains("workspace.package.publish"),
            "{why}"
        );
    }

    /// The claim, a value of a type Cargo refuses is an error, never a fact read: no TOML, no name,
    /// a name or a field of the wrong type, a table whose `workspace` is `false`, and a table with
    /// no `workspace` key. Mutations checked, each caught: in `field`, the arm `_ =>
    /// Err(wrong_type("package", key)),` made `_ => Ok(Field::Absent),`; in `package`, the arm for
    /// a missing name made `None => String::new(),`.
    #[test]
    fn a_shape_cargo_refuses_is_an_error() {
        for text in [
            "[package\n",
            "[package]\nversion = \"1.0.0\"\n",
            "[package]\nname = 3\n",
            "package = 3\n",
            "[package]\nname = \"a\"\npublish = \"yes\"\n",
            "[package]\nname = \"a\"\npublish = [3]\n",
            "[package]\nname = \"a\"\nversion = 1\n",
            "[package]\nname = \"a\"\nversion.workspace = false\n",
            "[package]\nname = \"a\"\nversion = { workspace = false, other = 1 }\n",
            "[package]\nname = \"a\"\nversion = { other = 1 }\n",
            "workspace = 3\n",
            "[workspace]\npackage = 3\n",
            "[workspace.package]\nversion = 1\n",
            "[workspace.package]\npublish = \"yes\"\n",
        ] {
            assert!(read_manifest(text).is_err(), "{text:?} was read");
        }
    }

    /// The claim, a table whose `workspace` is `true` inherits whatever other key it holds beside
    /// it, as Cargo 1.98 reads it, and one whose `workspace` is `false` is refused whatever it
    /// holds. Mutation checked: in `field`, `Some(toml::Value::Boolean(true)) =>
    /// Ok(Field::Workspace),` made `Some(toml::Value::Boolean(true)) if inline.len() == 1 =>
    /// Ok(Field::Workspace),`; caught.
    #[test]
    fn a_table_whose_workspace_is_true_inherits_whatever_else_it_holds() {
        let package = package_of(&format!(
            "{NAMED}version = {{ workspace = true, other = 1 }}
publish.workspace = true
             publish.x = \"y\"\n"
        ));
        assert_eq!(package.version, Field::Workspace);
        assert_eq!(package.publish, Field::Workspace);
    }

    /// The claim, `publish.workspace = true` and `include.workspace = true` take the workspace's
    /// value, and `[workspace.package]` holds `publish` and `include` as written. Mutations
    /// checked, each caught: in `Package::inherit`, `&workspace.publish` made
    /// `&workspace.publish.clone().map(|_| Publish::Flag(true))`, and
    /// `include: inherited(&self.include, &workspace.include, "include")?,` made
    /// `include: self.include.clone(),`; in `workspace_package`, the `publish` key's read followed
    /// by `.map(|_| Publish::Flag(true))`, and the `include` key's by `.map(|_| vec![])`.
    #[test]
    fn publish_and_include_are_inherited_as_the_workspace_writes_them() {
        let workspace = read_manifest(
            "[workspace]\n\n[workspace.package]\npublish = [\"a-registry\"]\n\
             include = [\"/src/**\", \"/README.md\"]\n",
        )
        .expect("the workspace manifest is read")
        .workspace
        .expect("it has a workspace")
        .package;
        let registries = Publish::Registries(vec!["a-registry".to_string()]);
        let patterns = vec!["/src/**".to_string(), "/README.md".to_string()];
        assert_eq!(workspace.publish, Some(registries.clone()));
        assert_eq!(workspace.include, Some(patterns.clone()));
        let member = package_of(&format!(
            "{NAMED}publish.workspace = true\ninclude.workspace = true\n"
        ));
        let resolved = member.inherit(&workspace).expect("every key is set");
        assert_eq!(resolved.publish, Field::Set(registries));
        assert_eq!(resolved.include, Field::Set(patterns));
    }

    /// The claim, a `[project]` table is read as the package where the manifest holds no
    /// `[package]`, and a `[package]` beside it is the one read, as Cargo 1.98 reads them.
    /// Mutations checked, each caught: in `read_manifest`,
    /// `table.get("package").or_else(|| table.get("project"))` made `table.get("package")`, and
    /// made `table.get("project").or_else(|| table.get("package"))`.
    #[test]
    fn a_project_table_is_the_package_where_no_package_table_is() {
        let package = package_of("[project]\nname = \"a-crate\"\nversion = \"1.0.0\"\n");
        assert_eq!(package.name, "a-crate");
        assert_eq!(package.version, Field::Set("1.0.0".to_string()));
        let package = package_of("[package]\nname = \"a\"\n\n[project]\nname = \"b\"\n");
        assert_eq!(package.name, "a");
    }

    /// The claim: `exclude`, `readme`, `license-file` and `package.workspace` are read as written,
    /// each of the first three inherited from `[workspace.package]` with its value as the workspace
    /// writes it, and a `[workspace]` table's `members` and `exclude` as written. Mutations
    /// checked, each caught: in `package`, `exclude: field(table, "exclude", strings)?,` made
    /// `exclude: Field::Absent,`; `workspace: plain(table, "package", "workspace", string)?,` made
    /// `workspace: None,`; in `readme`, `Some(Readme::Flag(*flag))` made
    /// `Some(Readme::Flag(true))`; in `workspace`, `exclude: plain(workspace, "workspace",
    /// "exclude", strings)?,` made `exclude: None,`; and in `Package::inherit`, the `readme`
    /// field's `inherited(&self.readme, &workspace.readme, "readme")?` made `self.readme.clone()`.
    #[test]
    fn exclude_readme_license_file_and_the_workspace_are_read_as_written() {
        let package = package_of(&format!(
            "{NAMED}exclude = [\"/docs\"]\nreadme = \"CHANGELOG.md\"\nlicense-file = \"LICENSE\"\n\
             workspace = \"../..\"\n"
        ));
        assert_eq!(package.exclude, Field::Set(vec!["/docs".to_string()]));
        assert_eq!(
            package.readme,
            Field::Set(Readme::Path("CHANGELOG.md".to_string()))
        );
        assert_eq!(package.license_file, Field::Set("LICENSE".to_string()));
        assert_eq!(package.workspace, Some("../..".to_string()));
        let flag = package_of(&format!("{NAMED}readme = false\n"));
        assert_eq!(flag.readme, Field::Set(Readme::Flag(false)));
        assert_eq!(flag.workspace, None);
        let workspace = read_manifest(
            "[workspace]\nmembers = [\"a\", \"b/*\"]\nexclude = [\"c\"]\n\n[workspace.package]\n\
             exclude = [\"/x\"]\nreadme = \"README.md\"\nlicense-file = \"LICENSE\"\n",
        )
        .expect("the workspace manifest is read")
        .workspace
        .expect("it has a workspace");
        assert_eq!(
            workspace.members,
            Some(vec!["a".to_string(), "b/*".to_string()])
        );
        assert_eq!(workspace.exclude, Some(vec!["c".to_string()]));
        let member = package_of(&format!(
            "{NAMED}exclude.workspace = true\nreadme.workspace = true\nlicense-file.workspace = true\n"
        ));
        let resolved = member
            .inherit(&workspace.package)
            .expect("every key is set");
        assert_eq!(resolved.exclude, Field::Set(vec!["/x".to_string()]));
        assert_eq!(
            resolved.readme,
            Field::Set(Readme::Path("README.md".to_string()))
        );
        assert_eq!(resolved.license_file, Field::Set("LICENSE".to_string()));
    }

    /// The claim: a value of a type Cargo refuses in a key read here for the first time is an
    /// error, never a key read as unset. Mutation checked: in `plain`,
    /// `.map(|value| read(value).ok_or_else(|| wrong_type(name, key)))` then `.transpose()` made
    /// `.and_then(|value| read(value))` then `.map_or(Ok(None), |v| Ok(Some(v)))`; caught.
    #[test]
    fn a_new_key_of_a_type_cargo_refuses_is_an_error() {
        for text in [
            "[package]\nname = \"a\"\nexclude = \"x\"\n",
            "[package]\nname = \"a\"\nreadme = 3\n",
            "[package]\nname = \"a\"\nlicense-file = true\n",
            "[package]\nname = \"a\"\nworkspace = true\n",
            "[workspace]\nmembers = \"a\"\n",
            "[workspace]\nexclude = [3]\n",
            "[workspace.package]\nreadme = [\"a\"]\n",
        ] {
            assert!(read_manifest(text).is_err(), "{text:?} was read");
        }
    }
}
