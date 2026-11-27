//! The manifest's `[changelog]` table: which Components hold a changelog, the categories, the
//! versions with no category and no class, and where the assembled file is written.
//!
//! **The table is opt-in**: a manifest that declares none is asked nothing of its changelogs, per
//! `design@check@changelog-opt-in`. It is a built-in table, so the name `changelog` is taken from
//! every extension, per `design@model@an-extension-claims-its-manifest-tables`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::Version;
use crate::finding::Finding;
use crate::manifest::{normalise_one, MANIFEST_NAME};

/// The table as written. `categories` is required: the categories are the project's, so none is
/// compiled in, per `design@check@changelog-opt-in`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct Declared {
    #[serde(default)]
    components: Vec<String>,
    #[serde(default)]
    assembled: Option<PathBuf>,
    #[serde(default)]
    assembled_url: Option<String>,
    categories: Vec<String>,
    #[serde(default)]
    unclassed: Vec<String>,
    #[serde(default)]
    category_components: BTreeMap<String, Vec<String>>,
}

/// The table, resolved: each name it gives checked against the Components and the categories, and
/// each one refused left out, so it declares nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangelogTable {
    components: Vec<String>,
    assembled: PathBuf,
    assembled_url: Option<String>,
    categories: Vec<String>,
    unclassed: Vec<Version>,
    category_components: BTreeMap<String, Vec<String>>,
}

/// Where the assembled file is written when `assembled` is absent.
const DEFAULT_ASSEMBLED: &str = "CHANGELOG.md";

impl ChangelogTable {
    /// The Components `components` lists, in the manifest's order of the Components; empty where
    /// it lists none, and the changelog crates are then those Cargo would publish.
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// The path of the assembled file, spelled as git lists it: `assembled`, or `CHANGELOG.md`.
    pub fn assembled(&self) -> &Path {
        &self.assembled
    }

    /// The link the preamble gives for the assembled file, where `assembled-url` sets one.
    pub fn assembled_url(&self) -> Option<&str> {
        self.assembled_url.as_deref()
    }

    /// The categories, in their declared order, which is the assembled file's.
    pub fn categories(&self) -> &[String] {
        &self.categories
    }

    /// The versions whose entries carry no category and no class.
    pub fn unclassed(&self) -> &[Version] {
        &self.unclassed
    }

    /// Whether the Component `component`'s changelog may hold an entry of `category`: the
    /// category is declared, and `[changelog.category-components]` names `component` for it or
    /// does not name the category.
    pub fn admits(&self, category: &str, component: &str) -> bool {
        self.categories.iter().any(|c| c == category)
            && self
                .category_components
                .get(category)
                .is_none_or(|list| list.iter().any(|c| c == component))
    }

    /// The Components a category is restricted to, where `[changelog.category-components]` names
    /// it.
    pub fn restricted_to(&self, category: &str) -> Option<&[String]> {
        self.category_components.get(category).map(Vec::as_slice)
    }
}

/// Resolve the table against the project's Component names, `names` in the manifest's order, the
/// root's first, holding a complaint of phase 1 for each name it cannot act on.
pub(crate) fn resolve(
    declared: &Declared,
    names: &[String],
    complaints: &mut Vec<Finding>,
) -> ChangelogTable {
    let mut complain = |what: String, action: &str| {
        complaints.push(Finding::in_file(MANIFEST_NAME, what, action));
    };
    let known = |list: &str, component: &str, complain: &mut dyn FnMut(String, &str)| {
        let found = names.iter().any(|n| n == component);
        if !found {
            complain(
                format!("`{component}` in [changelog] {list} names no Component"),
                "name a Component the manifest declares, by the basename of its directory or the \
                 project's name for the root; the name declares nothing until then",
            );
        }
        found
    };
    // In the manifest's order of the Components, whatever order the list writes them in: that is
    // the order of the assembled file's crates, per
    // `design@model@assembled-crates-in-the-manifest-s-order`.
    let listed: Vec<&String> = declared
        .components
        .iter()
        .filter(|c| known("components", c, &mut complain))
        .collect();
    let components: Vec<String> = names
        .iter()
        .filter(|n| listed.contains(n))
        .cloned()
        .collect();

    let mut categories: Vec<String> = Vec::new();
    for category in &declared.categories {
        // An entry writes its category before `, <class>: `, and the preamble lists them joined by
        // `, `, so a category holding either separator could be read back as another, per
        // `design@model@changelog-entry-categories`.
        let why = if category.trim().is_empty() {
            Some("is empty")
        } else if category.trim() != category {
            Some("opens or ends with white space")
        } else if category.contains([',', ':', '\n', '\r']) {
            Some("holds a `,`, a `:` or a line break")
        } else if categories.contains(category) {
            Some("is declared twice")
        } else {
            None
        };
        match why {
            Some(why) => complain(
                format!("the category `{category}` in [changelog] categories {why}"),
                "write each category once, as words with no `,`, `:` or line break; an entry of \
                 it is read as of no declared category until then",
            ),
            None => categories.push(category.clone()),
        }
    }

    let mut unclassed = Vec::new();
    for text in &declared.unclassed {
        match Version::parse(text) {
            Some(version) => unclassed.push(version),
            None => complain(
                format!("`{text}` in [changelog] unclassed is no version"),
                "write the version as its heading writes it, three numbers joined by `.`",
            ),
        }
    }

    let mut category_components = BTreeMap::new();
    for (category, list) in &declared.category_components {
        if !categories.contains(category) {
            complain(
                format!("[changelog.category-components] names `{category}`, which [changelog] categories does not declare"),
                "declare the category in [changelog] categories, or remove its row; the row \
                 restricts nothing until then",
            );
            continue;
        }
        let kept: Vec<String> = list
            .iter()
            .filter(|c| known(&format!("category-components {category}"), c, &mut complain))
            .cloned()
            .collect();
        category_components.insert(category.clone(), kept);
    }

    // A path refused here, per `design@model@assembled-path-taken-by-no-other-file`, is a
    // complaint of phase 1, which stops every run before a check or a writer reads the table, so
    // the default it falls back to is never acted on.
    let mut assembled = declared
        .assembled
        .clone()
        .unwrap_or_else(|| PathBuf::from(DEFAULT_ASSEMBLED));
    if !normalise_one("[changelog] assembled", &mut assembled, complaints) {
        assembled = PathBuf::from(DEFAULT_ASSEMBLED);
    } else if assembled.as_os_str().is_empty() {
        complaints.push(Finding::in_file(
            MANIFEST_NAME,
            "[changelog] assembled names the project's root, which no file can be written at"
                .to_string(),
            "name the file the assembled changelog is written to, or remove the key for \
             `CHANGELOG.md` at the root",
        ));
        assembled = PathBuf::from(DEFAULT_ASSEMBLED);
    }

    ChangelogTable {
        components,
        assembled,
        assembled_url: declared.assembled_url.clone(),
        categories,
        unclassed,
        category_components,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolved(text: &str) -> (ChangelogTable, Vec<Finding>) {
        let declared: Declared = toml::from_str(text).expect("the table deserialises");
        let names: Vec<String> = ["root", "alpha", "beta"]
            .iter()
            .map(|n| n.to_string())
            .collect();
        let mut complaints = Vec::new();
        let table = resolve(&declared, &names, &mut complaints);
        (table, complaints)
    }

    /// The claim, the names the table gives are checked: a Component it lists is kept in the
    /// manifest's order, and one it cannot name is a complaint that declares nothing. Mutation
    /// checked: in `resolve`, `.filter(|n| listed.contains(n))` made `.filter(|_| true)`; caught.
    #[test]
    fn the_listed_components_are_kept_in_the_manifest_s_order() {
        let (table, complaints) =
            resolved("categories = [\"A\"]\ncomponents = [\"beta\", \"alpha\", \"gamma\"]\n");
        assert_eq!(table.components(), ["alpha", "beta"]);
        assert_eq!(complaints.len(), 1, "{complaints:?}");
        assert!(complaints[0].what.contains("`gamma`"), "{complaints:?}");
    }

    /// The claim, the defaults and the refusals of the other keys: `assembled` defaults to
    /// `CHANGELOG.md`, and is spelled as git lists it; a category that could be read back as
    /// another, an unclassed text that is no version and a restriction of an undeclared category
    /// are each a complaint. Mutation checked: in `resolve`, the arm
    /// `category.contains([',', ':', '\n', '\r'])` made `false`; caught.
    #[test]
    fn each_key_is_resolved_or_refused() {
        let (table, complaints) = resolved("categories = [\"A\", \"B\"]\n");
        assert_eq!(table.assembled(), Path::new("CHANGELOG.md"));
        assert!(complaints.is_empty(), "{complaints:?}");
        let (table, complaints) = resolved(
            "categories = [\"A\", \"B, C\", \"A\"]\nunclassed = [\"0.1.0\", \"one\"]\n\
             assembled = \"./docs/CHANGES.md\"\n\n[category-components]\nZ = [\"alpha\"]\nB = [\"nope\", \"beta\"]\n",
        );
        assert_eq!(table.categories(), ["A"]);
        assert_eq!(table.unclassed(), [Version::parse("0.1.0").unwrap()]);
        assert_eq!(table.assembled(), Path::new("docs/CHANGES.md"));
        assert!(table.restricted_to("Z").is_none());
        let found: Vec<&str> = complaints.iter().map(|f| f.what.as_str()).collect();
        assert_eq!(found.len(), 5, "{found:#?}");
    }

    /// The claim: a category that is empty, and one that opens or ends with white space, are each
    /// a complaint saying which, and declare nothing, since no entry's tag could name it as
    /// declared. Mutations checked, each caught: in `resolve`,
    /// `let why = if category.trim().is_empty() {` made `let why = if false {`; and
    /// `} else if category.trim() != category {` made `} else if false {`.
    #[test]
    fn an_empty_or_padded_category_is_refused() {
        for (category, why) in [
            ("", "is empty"),
            ("  ", "is empty"),
            (" A", "opens or ends with white space"),
            ("A ", "opens or ends with white space"),
        ] {
            let (table, complaints) = resolved(&format!("categories = [\"{category}\"]\n"));
            assert!(table.categories().is_empty(), "{category:?}: {table:?}");
            assert_eq!(complaints.len(), 1, "{category:?}: {complaints:?}");
            assert!(
                complaints[0].what.ends_with(why),
                "{category:?}: {complaints:?}"
            );
        }
    }

    /// The claim: an `assembled` path that names the root, empty or `.`, is a complaint, as one
    /// that leaves the project is. Mutation checked: in `resolve`, the arm
    /// `} else if assembled.as_os_str().is_empty() {` made `} else if false {`; caught.
    #[test]
    fn an_assembled_path_naming_the_root_is_refused() {
        for path in ["", ".", "./", "../x.md"] {
            let (_, complaints) =
                resolved(&format!("categories = [\"A\"]\nassembled = \"{path}\"\n"));
            assert_eq!(complaints.len(), 1, "{path:?}: {complaints:?}");
        }
    }

    /// The claim, a category restricted by `[changelog.category-components]` is admitted in the
    /// Components it lists alone, an unrestricted one in every Component, and an undeclared one in
    /// none. Mutation checked: in `admits`, `.is_none_or(` made `.is_some_and(`; caught.
    #[test]
    fn a_category_is_admitted_where_the_table_admits_it() {
        let (table, _) =
            resolved("categories = [\"A\", \"W\"]\n\n[category-components]\nW = [\"beta\"]\n");
        assert!(table.admits("A", "alpha"));
        assert!(table.admits("W", "beta"));
        assert!(!table.admits("W", "alpha"));
        assert!(!table.admits("Z", "alpha"));
    }
}
