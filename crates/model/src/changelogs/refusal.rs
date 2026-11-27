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
use super::GENERATED_HEADER;
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

/// Whether `text`, the text of the file at the assembled file's path, is not this tool's to
/// replace: it does not open with [`GENERATED_HEADER`], a byte-order mark aside.
pub fn hand_written(text: &str) -> bool {
    !text
        .trim_start_matches('\u{feff}')
        .starts_with(GENERATED_HEADER)
}

/// Whether a writer of the generated files, `index`, `index --staged` or `check --fix`, may write
/// the assembled file of the tree it writes, or why not, per
/// `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` and
/// `design@check@fix-before-the-checks`: one function, so the three refuse alike.
///
/// It refuses where a file at the path, in that tree, is [`hand_written`], `committed` holding
/// the tree's text of each generated path and `unreadable` each one whose bytes are not text; and
/// where a changelog holds a line the assembled file would omit: an unread line, or an entry whose
/// category the manifest does not declare for its crate, an entry with no tag in a classed version
/// included. Nothing is refused where nothing is assembled.
pub fn refusal(
    manifest: &Manifest,
    model: &Model,
    committed: &HashMap<PathBuf, String>,
    unreadable: &HashSet<PathBuf>,
) -> Option<Refused> {
    let table = manifest.changelog()?;
    let path = model.changelog_crates().assembled(manifest)?;
    if unreadable.contains(path) || committed.get(path).is_some_and(|text| hand_written(text)) {
        return Some(Refused {
            why: format!(
                "{}: the file at the assembled changelog's path does not open with the generated \
                 header, so writing it would lose what it holds. Nothing was written.",
                slashed(path)
            ),
            repair: HAND_WRITTEN_REPAIR,
        });
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
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = [\"a\", \"b\"]\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n[changelog]\ncategories = [\"A\", \"W\"]\n\
             unclassed = [\"0.1.0\"]\n\n[changelog.category-components]\nW = [\"b\"]\n",
        )
        .expect("a declaration");
        let cargo: HashMap<PathBuf, String> = ["a", "b"]
            .iter()
            .map(|c| {
                (
                    PathBuf::from(c).join("Cargo.toml"),
                    format!("[package]\nname = \"p{c}\"\nversion = \"0.1.0\"\n"),
                )
            })
            .collect();
        let listing: Vec<PathBuf> = cargo.keys().cloned().collect();
        let crates =
            super::super::changelog_crates(&manifest, &listing, &|rel| cargo.get(rel).cloned());
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
        refusal(&manifest, &model, &HashMap::new(), &unreadable)
    }

    const CLEAN: &str = "## Next release\n\n- A, patch: a fix\n";

    /// The claim: a file at the assembled path whose bytes are not text is refused as one without
    /// the generated header, since it is not this tool's to replace. Mutation checked: in
    /// `refusal`, `if unreadable.contains(path) || committed` made `if committed`; caught.
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
}
