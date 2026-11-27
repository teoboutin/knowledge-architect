//! The writers' refusal: whether `index`, `index --staged` and `check --fix` may write the
//! assembled changelog over the tree each writes.
//!
//! A module of its own, not `super::render`: its reference is what a writer may lose,
//! `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` and
//! `design@check@fix-before-the-checks`, where the layout's is the assembled file's heads. What
//! the assembled file omits is `super::render`'s rule, read here through `omitted`, so the
//! refusal and the layout cannot disagree on it.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::render::{omitted, parsed, Omitted};
use super::{CargoTrouble, GENERATED_HEADER};
use crate::finding::slashed;
use crate::manifest::Manifest;
use crate::model::Model;

/// Why no writer may write the assembled file, with the repair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    /// What stands in the way, naming the file and the line.
    pub why: String,
    /// The repair.
    pub repair: &'static str,
}

/// The repair of a hand-written file at the assembled file's path.
const HAND_WRITTEN_REPAIR: &str = "move the file away, or set `[changelog] assembled` to another \
     path, and run again; a file without the generated header is not this tool's to replace";

/// The repair of a line the assembled file would omit.
const OMITTED_REPAIR: &str = "repair the line as the changelog check reports it, and run again";

/// The repair of a directory at the assembled file's path, which no writer replaces.
const DIRECTORY_REPAIR: &str = "move out what the directory holds, then remove it, and run again";

/// The repair of a changelog crate's file the changelog checks cannot read.
const UNREAD_FILE_REPAIR: &str = "repair the file as the changelog check reports it, and run again";

/// What the tree a writer writes holds, as the gathering read it, which [`refusal`] judges.
#[derive(Clone, Copy, Debug)]
pub struct WrittenTree<'a> {
    /// The tree's listing, every file it holds.
    pub present: &'a HashSet<PathBuf>,
    /// Every directory the tree holds.
    pub directories: &'a HashSet<PathBuf>,
    /// The tree's text of each generated path it holds as a text file.
    pub committed: &'a HashMap<PathBuf, String>,
    /// Each generated path the tree holds whose bytes could not be read as text.
    pub unreadable: &'a HashSet<PathBuf>,
}

/// Whether `text`, the text of the file at the assembled file's path, is not this tool's to
/// replace: it does not open with [`GENERATED_HEADER`], a byte-order mark aside.
pub fn hand_written(text: &str) -> bool {
    !text
        .trim_start_matches('\u{feff}')
        .starts_with(GENERATED_HEADER)
}

/// Whether a writer of the generated files, `index`, `index --staged` or `check --fix`, may write
/// the assembled file of the tree it writes, or why not, per
/// `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`,
/// `design@check@fix-before-the-checks` and `design@model@assembled-file-not-overwritten`: one
/// function, so the three refuse alike.
///
/// It refuses where a directory stands at the path, in that tree, and where a file there is
/// [`hand_written`] or not text; where the changelog checks cannot read a changelog crate's file:
/// a `Cargo.toml` that [`Crates::trouble`](super::Crates::trouble) names, or a `CHANGELOG.md` that
/// the tree holds and a `[walk]` row keeps out; and where a changelog holds a line the
/// assembled file would omit: an unread line, or an entry whose category the manifest does not
/// declare for its crate, an entry with no tag in a classed version included. Nothing is refused
/// where nothing is assembled. A changelog crate whose tree holds no `CHANGELOG.md` is no refusal:
/// it has no entry to omit.
pub fn refusal(manifest: &Manifest, model: &Model, tree: WrittenTree) -> Option<Refused> {
    let table = manifest.changelog()?;
    let path = model.changelog_crates().assembled(manifest)?;
    if tree.directories.contains(path) {
        return Some(Refused {
            why: format!(
                "{}: a directory stands at the assembled changelog's path. Nothing was written.",
                slashed(path)
            ),
            repair: DIRECTORY_REPAIR,
        });
    }
    let committed = tree.committed.get(path);
    if tree.unreadable.contains(path) || committed.is_some_and(|text| hand_written(text)) {
        return Some(Refused {
            why: format!(
                "{}: the file at the assembled changelog's path does not open with the generated \
                 header, so writing it would lose what it holds. Nothing was written.",
                slashed(path)
            ),
            repair: HAND_WRITTEN_REPAIR,
        });
    }
    // **A file the changelog checks cannot read refuses as an unread line does**: the assembled
    // file would omit what it holds, a crate's whole entries where its `Cargo.toml` cannot be read
    // or its `CHANGELOG.md` is kept out, per `design@model@assembled-file-not-overwritten`.
    // Every kind of trouble counts: a `Cargo.toml` that is missing or declares no package, where
    // `[changelog] components` lists its Component, leaves that crate out as an unreadable one does.
    if let Some(trouble) = model.changelog_crates().trouble().first() {
        let why = match trouble {
            CargoTrouble::Unreadable { file, reason } => format!(
                "{}: the changelog checks cannot read it, {reason}",
                slashed(file)
            ),
            CargoTrouble::KeptOut { file, row } => format!("{}: {row} keeps it out", slashed(file)),
            CargoTrouble::Missing { file } => format!(
                "{}: it is not there, and [changelog] components lists its Component",
                slashed(file)
            ),
            CargoTrouble::NoPackage { file } => format!(
                "{}: it declares no package, and [changelog] components lists its Component",
                slashed(file)
            ),
        };
        return Some(Refused {
            why: format!(
                "{why}; the assembled changelog would omit that crate's entries. Nothing was \
                 written."
            ),
            repair: UNREAD_FILE_REPAIR,
        });
    }
    for crate_ in model.changelog_crates().all() {
        let rel = crate_.changelog();
        let row =
            crate::walk::keeping_row(&rel, manifest.walk()).filter(|_| tree.present.contains(&rel));
        if let Some(row) = row {
            return Some(Refused {
                why: format!(
                    "{}: {row} keeps it out; the assembled changelog would omit its entries. \
                     Nothing was written.",
                    slashed(&rel)
                ),
                repair: UNREAD_FILE_REPAIR,
            });
        }
    }
    model
        .changelog_crates()
        .all()
        .iter()
        .zip(parsed(model))
        .find_map(|(crate_, (component, _, log))| {
            let rel = slashed(&crate_.changelog());
            let why = match omitted(table, &component, &log)? {
                Omitted::Unread(line) => format!(
                    "{rel}:{line}: the line is no entry, no version heading and no continuation \
                     of an entry, and the assembled changelog would omit it. Nothing was written."
                ),
                Omitted::Entry(line) => format!(
                    "{rel}:{line}: the entry's category is none the manifest declares for \
                     `{component}`, and the assembled changelog would omit it. Nothing was \
                     written."
                ),
            };
            Some(Refused {
                why,
                repair: OMITTED_REPAIR,
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const PREAMBLE: &str = "# Changelog\n\n";

    /// A project of two changelog crates, `a` and `b`, whose manifest declares the categories `A`
    /// and `W`, `W` restricted to `b`, and `0.1.0` unclassed; each crate's changelog is the text
    /// `logs` gives.
    fn project(logs: [&str; 2]) -> (Manifest, Model) {
        project_with(logs, "", None)
    }

    /// The same, `skip_files` the `[walk] skip-files` row, and a third Component `c`, listed with
    /// no changelog, whose `Cargo.toml` is `c_cargo` where given.
    fn project_with(logs: [&str; 2], skip_files: &str, c_cargo: Option<&str>) -> (Manifest, Model) {
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            &format!(
                "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\", \"c\"]\n\n\
                 [walk]\nskip-dirs = []\nskip-files = [{skip_files}]\n\n[changelog]\ncategories = [\"A\", \"W\"]\n\
                 unclassed = [\"0.1.0\"]\n\n[changelog.category-components]\nW = [\"b\"]\n"
            ),
        )
        .expect("a declaration");
        let mut cargo: HashMap<PathBuf, String> = ["a", "b"]
            .iter()
            .map(|c| {
                (
                    PathBuf::from(c).join("Cargo.toml"),
                    format!("[package]\nname = \"p{c}\"\nversion = \"0.1.0\"\n"),
                )
            })
            .collect();
        if let Some(text) = c_cargo {
            cargo.insert(PathBuf::from("c/Cargo.toml"), text.to_string());
        }
        let listing: Vec<PathBuf> = cargo.keys().cloned().collect();
        let crates = super::super::changelog_crates(&manifest, &listing, &|rel| {
            cargo
                .get(rel)
                .cloned()
                .ok_or_else(|| std::io::ErrorKind::NotFound.into())
        });
        assert_eq!(crates.all().len(), 2, "{crates:?}");
        let mut model = Model::from_documents(vec![
            (
                PathBuf::from("a/CHANGELOG.md"),
                format!("{PREAMBLE}{}", logs[0]),
            ),
            (
                PathBuf::from("b/CHANGELOG.md"),
                format!("{PREAMBLE}{}", logs[1]),
            ),
        ]);
        model.set_changelog_crates(crates);
        (manifest, model)
    }

    fn refused(logs: [&str; 2], unreadable: &[&str]) -> Option<Refused> {
        let (manifest, model) = project(logs);
        let unreadable: HashSet<PathBuf> = unreadable.iter().map(PathBuf::from).collect();
        let none = HashSet::new();
        refusal(
            &manifest,
            &model,
            WrittenTree {
                present: &none,
                directories: &none,
                committed: &HashMap::new(),
                unreadable: &unreadable,
            },
        )
    }

    const CLEAN: &str = "## Next release\n\n- A, patch: a fix\n";

    /// The claim: a file at the assembled path whose bytes are not text is refused as one without
    /// the generated header, since it is not this tool's to replace. Mutation checked: in
    /// `refusal`, `if tree.unreadable.contains(path) || committed` made `if committed`; caught.
    #[test]
    fn a_file_that_is_not_text_at_the_assembled_path_is_refused() {
        assert_eq!(refused([CLEAN, CLEAN], &[]), None);
        let refused = refused([CLEAN, CLEAN], &["CHANGELOG.md"]).expect("a refusal");
        assert_eq!(refused.repair, HAND_WRITTEN_REPAIR);
    }

    /// The claim: an entry with no tag in a version `unclassed` lists is written, so no writer
    /// refuses over it. Mutation checked: in `render::place`, the early
    /// `return Some(Place::Unclassed);` made `return None;`; caught.
    #[test]
    fn an_untagged_entry_of_an_unclassed_version_is_no_refusal() {
        let log = "## Next release\n\n- A, patch: a fix\n\n## 0.1.0\n\n- the first release\n";
        assert_eq!(refused([log, log], &[]), None);
    }

    /// The claim: an entry of a declared category that `[changelog.category-components]` does not
    /// admit for its crate is refused, naming its line, as one the assembled file would omit; in
    /// the crate it admits, it is none. Mutation checked: in `render::place`,
    /// `.filter(|tag| table.admits(&tag.category, component))` made
    /// `.filter(|tag| table.categories().contains(&tag.category))`; caught.
    #[test]
    fn an_entry_of_a_category_its_crate_is_not_admitted_to_is_refused() {
        let restricted = "## Next release\n\n- W, patch: a workflow change\n";
        assert_eq!(refused([CLEAN, restricted], &[]), None);
        let refused = refused([restricted, CLEAN], &[]).expect("a refusal");
        assert_eq!(refused.repair, OMITTED_REPAIR);
        assert!(
            refused.why.starts_with("a/CHANGELOG.md:5:"),
            "{}",
            refused.why
        );
    }

    /// The claim: a changelog crate's `Cargo.toml` that the changelog checks cannot read refuses
    /// the writers, since the assembled file would omit that crate's entries. Mutation checked: in
    /// `refusal`, `if let Some(trouble) = model.changelog_crates().trouble().first()` made
    /// `if let Some(trouble) = None::<&CargoTrouble>`; caught.
    #[test]
    fn a_cargo_toml_the_checks_cannot_read_is_refused() {
        let (manifest, model) = project_with([CLEAN, CLEAN], "", Some("[package\n"));
        let none = HashSet::new();
        let tree = WrittenTree {
            present: &none,
            directories: &none,
            committed: &HashMap::new(),
            unreadable: &none,
        };
        let refused = refusal(&manifest, &model, tree).expect("a refusal");
        assert!(refused.why.starts_with("c/Cargo.toml: "), "{}", refused.why);
        assert_eq!(refused.repair, UNREAD_FILE_REPAIR);
    }

    /// The claim: a crate's `CHANGELOG.md` that the tree holds and a `[walk]` row keeps out refuses
    /// the writers, naming the row, and one the tree does not hold is no refusal, as a missing
    /// file omits nothing. Mutations checked, each caught: in `refusal`,
    /// `.filter(|_| tree.present.contains(&rel));` made `.filter(|_| true);`, and the value of
    /// `row`, `crate::walk::keeping_row(&rel, manifest.walk())` with that filter, made
    /// `None::<String>`.
    #[test]
    fn a_changelog_a_walk_row_keeps_out_is_refused_where_the_tree_holds_it() {
        let (manifest, model) = project_with([CLEAN, CLEAN], "\"b/CHANGELOG.md\"", None);
        let refuse = |present: &[&str]| {
            let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
            let none = HashSet::new();
            let tree = WrittenTree {
                present: &present,
                directories: &none,
                committed: &HashMap::new(),
                unreadable: &none,
            };
            refusal(&manifest, &model, tree)
        };
        assert_eq!(refuse(&["a/CHANGELOG.md"]), None);
        let refused = refuse(&["a/CHANGELOG.md", "b/CHANGELOG.md"]).expect("a refusal");
        assert!(
            refused
                .why
                .starts_with("b/CHANGELOG.md: [walk] skip-files `b/CHANGELOG.md`"),
            "{}",
            refused.why
        );
    }

    /// The claim: a directory at the assembled path is refused as a directory, with the repair
    /// that removes it, not as a file without the generated header. Mutation checked: in
    /// `refusal`, `if tree.directories.contains(path) {` made `if false {`; caught.
    #[test]
    fn a_directory_at_the_assembled_path_is_refused_as_one() {
        let (manifest, model) = project([CLEAN, CLEAN]);
        let at = HashSet::from([PathBuf::from("CHANGELOG.md")]);
        let none = HashSet::new();
        let tree = WrittenTree {
            present: &none,
            directories: &at,
            committed: &HashMap::new(),
            unreadable: &at,
        };
        let refused = refusal(&manifest, &model, tree).expect("a refusal");
        assert_eq!(refused.repair, DIRECTORY_REPAIR);
        assert!(
            refused.why.contains("a directory stands"),
            "{}",
            refused.why
        );
    }
}
