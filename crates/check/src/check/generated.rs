//! The generated files are current: each extension's, the register indexes, and the assembled
//! changelog.
//!
//! A generated file that drifts is worse than no generated file: thaum's `bumping-rules` skill
//! calls the rule index, which thaum's rules extension generates, *the work list* for a
//! release, and a stale work list decides what a renumbering breaks.
//!
//! **The comparison is against a string.** Nothing here writes the file and restores it, so a
//! run that dies half-way leaves the tree exactly as it found it. Line endings are left out of it, per
//! `design@check@generated-text-compared-without-line-endings`.

use std::path::Path;

use crate::currency::{current, lf};
use crate::extension::Generated;
use crate::model::generated;
use crate::model::Finding;
use crate::model::Manifest;
use crate::model::Model;

use super::{Inputs, Judged};

/// Compare each generated file with what it would be generated as now: every file an
/// extension generates, then one index per file-register instance, then the assembled changelog
/// where one is assembled.
///
/// An extension's files arrive already rendered, because only the extension can render them;
/// the file-register indexes are rendered here, so a caller handing in no extension still has
/// every listing compared.
pub(crate) fn check(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    extension_files: &[Generated],
) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in extension_files {
        compare(&file.rel, &file.text, inputs, file.action, &mut findings);
    }
    for (rel, expected) in
        generated::file_register_indexes(model, manifest, inputs.present, inputs.directories)
    {
        compare(
            &rel,
            &expected,
            inputs,
            &index_action(manifest, inputs.judged),
            &mut findings,
        );
    }
    // The assembled changelog, where one is assembled, is missing or out of date as an index is,
    // and its repair is the one the register indexes name for the tree judged; but over a file
    // the writers refuse to replace, one with no generated header or not text, the repair moves
    // it away first, since `index` refuses until then, per `design@check@finding-names-the-repair`.
    if let Some((rel, expected)) = crate::model::changelog::assembled_file(manifest, model) {
        let index = index_action(manifest, inputs.judged);
        let hand_written = inputs.generated_unreadable.contains(&rel)
            || inputs
                .committed
                .get(&rel)
                .is_some_and(|text| crate::model::changelog::hand_written(text));
        let action = if hand_written {
            format!(
                "move the file away, or set `[changelog] assembled` to another path, then {index}: \
                 a file without the generated header is not this tool's to replace"
            )
        } else {
            index
        };
        compare(&rel, &expected, inputs, &action, &mut findings);
    }
    findings
}

/// The repair of a file-register index that is missing or out of date, which the registers'
/// finding on a missing index names too, so the two findings on one file name one repair.
///
/// It names the repair and no cause, per `design@check@finding-names-the-repair`: a hand edit,
/// an upgrade of the checker and a branch that changes the generator all leave an index out of
/// date, and the repair is the same for each. The tree the run judges decides the command:
/// `index` writes the working tree and leaves the index as it was, so under `check --staged` it
/// would leave the finding standing, and `index --staged` is the repair there. A commit's file
/// is repaired by a history edit, whose step is `index`.
pub(crate) fn index_action(manifest: &Manifest, judged: Judged) -> String {
    match judged {
        Judged::WorkingTree | Judged::Commit => format!("run `{} index`", manifest.command()),
        Judged::Index => format!("run `{} index --staged`", manifest.command()),
    }
}

/// The repair of a generated file whose path holds a directory: no writer of a generated file
/// replaces a directory, so it is removed before `action`.
///
/// Git lists no empty directory, so the directory always holds a file, and the run may judge it,
/// as an entry where it is markdown under a register's home. Its removal alone would delete what
/// the same run asks to repair, so the repair moves out what it holds first.
pub(crate) fn directory_action(action: &str) -> String {
    format!("move out what the directory holds, then remove it, then {action}")
}

/// What this family says of a generated file that differs from what it would be generated as.
const MISSING: &str = "the generated file is missing";
const UNREADABLE: &str = "the generated file cannot be read as UTF-8 text";
const DIRECTORY: &str = "a directory stands where the generated file belongs";
const OUT_OF_DATE: &str = "the generated file is out of date";

/// Whether `finding` is one this family raised on a generated file: missing, unreadable, a
/// directory in its place, or out of date.
pub(crate) fn is_generated_finding(finding: &Finding) -> bool {
    [MISSING, UNREADABLE, DIRECTORY, OUT_OF_DATE].contains(&finding.what.as_str())
}

/// Compare one generated file with `expected`, the text it would be generated as.
///
/// A file that differs by its line endings alone is current, as `current` says, and the line of one
/// that differs otherwise is counted over the two texts with their line endings made one.
fn compare(path: &Path, expected: &str, inputs: &Inputs, action: &str, out: &mut Vec<Finding>) {
    let committed = inputs.committed.get(path).map(String::as_str);
    match committed {
        // The caller reads the file as UTF-8 text and holds no copy where the read failed: a
        // listed directory, a file the caller found and could not read, and otherwise a file
        // that is absent, git listing it or not.
        None if inputs.directories.contains(path) => {
            out.push(Finding::in_file(path, DIRECTORY, directory_action(action)))
        }
        None if inputs.generated_unreadable.contains(path) => {
            out.push(Finding::in_file(path, UNREADABLE, action))
        }
        None => out.push(Finding::in_file(path, MISSING, action)),
        Some(on_disk) if !current(on_disk, expected) => {
            let line = first_difference(&lf(on_disk), &lf(expected));
            out.push(Finding::at(path, line, OUT_OF_DATE, action));
        }
        Some(_) => {}
    }
}

/// The first line where the committed file and the regenerated one disagree.
///
/// A file-level *this is stale* sends a reader to diff the whole thing; a line sends them to
/// the change. Both files are generated, so the first disagreement is where the content
/// actually moved. The texts hold LF line endings, so a carriage return left in one is content:
/// `str::lines` would drop it and count past the line.
fn first_difference(a: &str, b: &str) -> u32 {
    let mut n = 1;
    for (x, y) in a.split('\n').zip(b.split('\n')) {
        if x != y {
            return n;
        }
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_with_no_extension_still_judges_the_file_register_indexes() {
        use crate::model::Manifest;
        use std::collections::{HashMap, HashSet};
        use std::path::PathBuf;

        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let committed = HashMap::new();
        let configs = HashMap::new();
        let present: HashSet<PathBuf> = HashSet::new();
        let directories: HashSet<PathBuf> = [PathBuf::from("docs/open-issues")].into();
        let inputs = Inputs {
            committed: &committed,
            configs: &configs,
            configs_not_text: &std::collections::HashSet::new(),
            generated_unreadable: &std::collections::HashSet::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        let found: Vec<String> = check(&model, &manifest, &inputs, &[])
            .iter()
            .map(|f| f.location())
            .collect();
        // The issue instance's index, and nothing an extension would have generated.
        assert_eq!(found, vec!["docs/open-issues/index.md".to_string()]);
    }

    /// The findings on an issue instance's index, over a listing of `present` with the issue
    /// directory, no generated file read, and the files of `unreadable` found and not read.
    fn on_the_index(present: &[&str], unreadable: &[&str]) -> Vec<Finding> {
        use crate::model::Manifest;
        use std::collections::{HashMap, HashSet};
        use std::path::PathBuf;

        let text =
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n\
             ";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let model = Model::from_documents(Vec::new());
        let committed = HashMap::new();
        let configs = HashMap::new();
        let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let mut directories = crate::check::testing::implied_directories(&present);
        directories.insert(PathBuf::from("docs/open-issues"));
        let unreadable: HashSet<PathBuf> = unreadable.iter().map(PathBuf::from).collect();
        let inputs = Inputs {
            committed: &committed,
            configs: &configs,
            configs_not_text: &std::collections::HashSet::new(),
            generated_unreadable: &unreadable,
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        check(&model, &manifest, &inputs, &[])
    }

    /// The claim: a generated file the caller holds no text for is told apart by the listing and
    /// by the caller's read: a listed directory is named as one, with its removal before the
    /// regeneration, since no writer replaces a directory; a file the caller found and could not
    /// read is unreadable; and any other is missing, whether git lists it or not, as git lists a
    /// file deleted from the working tree until the deletion is staged. Each statement counts as
    /// this family's. Mutations: dropping the directory arm or the unreadable arm each fail one
    /// case, and the unreadable arm keyed on `inputs.present` in place of
    /// `inputs.generated_unreadable` fails the listed absent file's.
    #[test]
    fn a_generated_file_not_read_is_named_by_what_the_listing_holds() {
        let index = "docs/open-issues/index.md";
        let cases = [
            (vec![], vec![], MISSING, "run `klarch index`"),
            (vec![index], vec![], MISSING, "run `klarch index`"),
            (vec![index], vec![index], UNREADABLE, "run `klarch index`"),
            (
                vec![index, "docs/open-issues/index.md/a.md"],
                vec![],
                DIRECTORY,
                "move out what the directory holds, then remove it, then run `klarch index`",
            ),
        ];
        for (present, unreadable, what, action) in cases {
            let found = on_the_index(&present, &unreadable);
            assert_eq!(found.len(), 1, "{present:?}: {found:?}");
            assert_eq!(found[0].file, Path::new(index), "{found:?}");
            assert_eq!(found[0].what, what, "{present:?}");
            assert_eq!(found[0].action, action, "{present:?}");
            assert!(is_generated_finding(&found[0]), "{found:?}");
        }
    }

    /// The findings on one generated file that holds `on_disk` and would be generated as
    /// `expected`.
    fn compared(on_disk: &str, expected: &str) -> Vec<Finding> {
        use std::collections::{HashMap, HashSet};
        use std::path::PathBuf;

        let path = Path::new("docs/index.md");
        let committed: HashMap<PathBuf, String> = [(path.to_owned(), on_disk.to_owned())].into();
        let configs = HashMap::new();
        let present: HashSet<PathBuf> = HashSet::new();
        let inputs = Inputs {
            committed: &committed,
            configs: &configs,
            configs_not_text: &HashSet::new(),
            generated_unreadable: &HashSet::new(),
            present: &present,
            directories: &HashSet::new(),
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        let mut out = Vec::new();
        compare(path, expected, &inputs, "regenerate", &mut out);
        out
    }

    /// The claim: a generated file that differs from what it would be generated as by its line
    /// endings alone is current, whichever of the two holds CRLF, and one that differs otherwise is
    /// out of date at the line that differs, line endings apart. Mutations: normalising only one
    /// side of the comparison, or of the texts the line is counted over, each fail it.
    #[test]
    fn a_generated_file_that_differs_by_line_endings_alone_is_current() {
        let lf = "a\nb\nc\n";
        let crlf = "a\r\nb\r\nc\r\n";
        for (on_disk, expected) in [(lf, lf), (crlf, lf), (lf, crlf), (crlf, crlf)] {
            assert_eq!(compared(on_disk, expected), [], "{on_disk:?} {expected:?}");
        }
        for (on_disk, expected) in [
            ("a\r\nB\r\nc\r\n", lf),
            ("a\nB\nc\n", crlf),
            // A carriage return that ends no line is content.
            ("a\nb\r\r\nc\n", lf),
        ] {
            let found = compared(on_disk, expected);
            assert_eq!(found.len(), 1, "{on_disk:?}: {found:?}");
            assert_eq!(found[0].what, OUT_OF_DATE);
            assert_eq!(found[0].line, Some(2), "{on_disk:?} {expected:?}");
        }
    }

    #[test]
    fn the_first_differing_line_is_reported() {
        assert_eq!(first_difference("a\nb\nc", "a\nB\nc"), 2);
        // A file that is a prefix of the other differs at the line past the shorter one.
        assert_eq!(first_difference("a\nb", "a\nb\nc"), 3);
        assert_eq!(first_difference("same", "same"), 2);
    }
}
