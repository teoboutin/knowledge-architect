//! The read of a `Cargo.toml`'s text: the facts of `[package]` the tools of knowledge-architect use,
//! and the keys of `[workspace.package]` a package inherits them from.
//!
//! Cargo's manifest format is read as `path@cargo-reader@docs/grounding/cargo-manifest.md` records
//! it. This module is the one reader of that format in the code of the published crates.

use std::fmt;

/// What a manifest declares, of the tables the reader reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    /// The `[package]` table, absent from a virtual workspace's manifest.
    pub package: Option<Package>,
    /// The `[workspace.package]` keys, present whenever the manifest holds a `[workspace]` table,
    /// each `None` where that table does not set it. `None` for a manifest with no `[workspace]`.
    pub workspace: Option<WorkspacePackage>,
}

/// The facts of a `[package]` table, each as the manifest states it.
///
/// A field set to `{ workspace = true }` is [`Field::Workspace`] until [`Package::inherit`]
/// replaces it with the workspace's value. No default of Cargo's is applied: an absent `version`
/// is [`Field::Absent`], which Cargo reads as `0.0.0` and as a package it does not publish.
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
}

/// A field of `[package]` that `[workspace.package]` can supply.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Field<T> {
    /// The manifest does not set it.
    Absent,
    /// The manifest sets it to this value.
    Set(T),
    /// The manifest sets it to `{ workspace = true }`, so its value is the workspace's.
    Workspace,
}

/// The value of `publish`, as the manifest writes it.
///
/// Cargo reads `false` and an empty list alike, as a package published to no registry, and
/// `true` as a package with no restriction, the same as an absent `publish`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Publish {
    /// `publish = true` or `publish = false`.
    Flag(bool),
    /// `publish = [...]`: the registries the package may be published to, by name.
    Registries(Vec<String>),
}

/// Why a manifest's text could not be read: no TOML, or a table or a field this reader reads of a
/// shape Cargo refuses.
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
/// # Errors
///
/// Returns a [`ManifestError`] when the text is no TOML; when `[package]` or `[workspace]` is not
/// a table, or `[workspace.package]` is not one; when `[package]` has no `name` that is a string;
/// and when a field read here holds a value of a type Cargo refuses for it, a
/// `{ workspace = false }` and a table holding another key than `workspace` included.
pub fn read_manifest(text: &str) -> Result<Manifest, ManifestError> {
    let table: toml::Table = text
        .parse()
        .map_err(|error: toml::de::Error| ManifestError::new(error.message().to_string()))?;
    let package = match table.get("package") {
        None => None,
        Some(value) => Some(package(value)?),
    };
    let workspace = match table.get("workspace") {
        None => None,
        Some(value) => Some(workspace_package(value)?),
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
    })
}

fn workspace_package(value: &toml::Value) -> Result<WorkspacePackage, ManifestError> {
    let workspace = value
        .as_table()
        .ok_or_else(|| ManifestError::new("`workspace` is not a table"))?;
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
        version: workspace_key(table, "version", string)?,
        publish: workspace_key(table, "publish", publish)?,
        repository: workspace_key(table, "repository", string)?,
        include: workspace_key(table, "include", strings)?,
    })
}

fn workspace_key<T>(
    table: &toml::Table,
    key: &str,
    read: fn(&toml::Value) -> Option<T>,
) -> Result<Option<T>, ManifestError> {
    table
        .get(key)
        .map(|value| read(value).ok_or_else(|| wrong_type("workspace.package", key)))
        .transpose()
}

/// A field of `[package]`: absent, a value `read` accepts, or `{ workspace = true }`, the one table
/// Cargo accepts for an inheritable field of `[package]`.
fn field<T>(
    table: &toml::Table,
    key: &str,
    read: fn(&toml::Value) -> Option<T>,
) -> Result<Field<T>, ManifestError> {
    let Some(value) = table.get(key) else {
        return Ok(Field::Absent);
    };
    if let Some(inline) = value.as_table() {
        return match (inline.len(), inline.get("workspace")) {
            (1, Some(toml::Value::Boolean(true))) => Ok(Field::Workspace),
            (1, Some(toml::Value::Boolean(false))) => Err(ManifestError::new(format!(
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
    /// with a `[workspace]` table in a parent directory, a lookup this function does not make.
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

    /// Claim 1, the facts of `[package]` a manifest states directly. Mutation checked: in `field`,
    /// `.map(Field::Set)` made `.map(|_| Field::Absent)`; caught.
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

    /// Claim 1, an absent `version`, `repository` and `include`: no default of Cargo's applied.
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

    /// Claim 1, each shape of `publish`. Mutations checked, each caught: in `publish`,
    /// `Some(Publish::Flag(*flag))` made `Some(Publish::Flag(true))`; in `field`,
    /// `(1, Some(toml::Value::Boolean(true))) => Ok(Field::Workspace),` made `=> Ok(Field::Absent),`.
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

    /// Claim 1, a manifest with no `[package]`: a virtual workspace's. Mutation checked: in
    /// `workspace_package`, `return Ok(WorkspacePackage::default());` made
    /// `return Err(ManifestError::new("x"));`; caught.
    #[test]
    fn a_manifest_without_a_package_has_none() {
        let manifest =
            read_manifest("[workspace]\nmembers = [\"a\"]\n").expect("the fixture is read");
        assert_eq!(manifest.package, None);
        assert_eq!(manifest.workspace, Some(WorkspacePackage::default()));
        let manifest = read_manifest("[dependencies]\n").expect("the fixture is read");
        assert_eq!(
            manifest,
            Manifest {
                package: None,
                workspace: None
            }
        );
    }

    /// Claim 1, `version.workspace = true` and `repository.workspace = true` beside a workspace
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
        .expect("it has a workspace");
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

    /// Claim 1, an inherited key the workspace does not set is an error naming it, as Cargo's
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

    /// Claim 1, the shapes Cargo refuses are errors, never a fact read: no TOML, no name, a name
    /// or a field of the wrong type, `{ workspace = false }`, and a table beside `workspace`.
    /// Mutations checked, each caught: in `field`, `_ => Err(wrong_type("package", key)),` made
    /// `_ => Ok(Field::Absent),`; in `package`, `None => return Err(ManifestError::new("missing
    /// field `package.name`")),` made `None => String::new(),`.
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
            "[package]\nname = \"a\"\nversion = { workspace = true, other = 1 }\n",
            "workspace = 3\n",
            "[workspace]\npackage = 3\n",
            "[workspace.package]\nversion = 1\n",
            "[workspace.package]\npublish = \"yes\"\n",
        ] {
            assert!(read_manifest(text).is_err(), "{text:?} was read");
        }
    }
}
