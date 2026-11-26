//! `id`: whether each id is free.

use std::path::Path;
use std::process::ExitCode;

use crate::model::table::{Anchors, Entities};
use crate::output::outln;
use crate::Manifest;

/// Whether each id is free: every entity the table defines under it, as the reference `show`
/// takes, or `free`, or that it is no id, with, for an argument holding `@`, the repair of giving
/// the id alone.
///
/// **A `free` is an answer over the whole table**, so it is given only over a complete model: the
/// run asks the first three phases as a writer does, per `design@check@phases-gate-the-report`,
/// where an entity an unread file defines would make the answer false. `show` needs no such gate,
/// since what it prints exists whatever else the model misses.
///
/// **The exit codes**, per `design@cli@exit-code-ladder`: 0 when every argument is an id nothing
/// defines; 1 otherwise, an argument outside the grammar included, since that is a negative
/// answer about the candidate, and the run still answers every other one; 2 when the command
/// cannot run, an incomplete model included.
///
/// Every kind and every anchor is listed, unfiltered: which collision refuses an id is the
/// reader's rule, such as a design thread's slug, which collides only with a design or a
/// `rejected` entry of the Component that will own it.
///
/// # Errors
///
/// An `Err`, the could-not-run exit 2, with nothing answered:
///
/// - `ids` is empty;
/// - the project cannot be read: its root cannot be resolved, or git cannot list it;
/// - the model is incomplete: git or the filesystem refused a read, or the first three phases
///   found anything, the message then naming the phase and the command that reports it.
pub fn id(manifest: &Manifest, ids: &[String], checker: &[&Path]) -> Result<ExitCode, String> {
    if ids.is_empty() {
        return Err("no id to look up".to_string());
    }
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    crate::model::foundation::complete_working_tree(manifest, &model).map_err(|e| {
        e.replace(
            "Nothing was written.",
            "No id is answered over it, since one it defines would read as free.",
        )
    })?;
    let anchors = Anchors::of(manifest, model.listing());
    let entities = Entities::build(&model, &anchors);
    let by_id = entities.by_id();
    let width = ids.iter().map(|i| i.len()).max().unwrap_or(0);
    let mut all_free = true;
    for candidate in ids {
        let pad = " ".repeat(width - candidate.len());
        if !crate::model::table::is_entry_id(candidate) {
            all_free = false;
            outln!("{candidate}{pad}  {}", not_an_id(candidate));
            continue;
        }
        let mut forms: Vec<String> = by_id
            .get(candidate.as_str())
            .into_iter()
            .flatten()
            .map(|(kind, anchor)| crate::model::table::spelled(kind.name(), anchor, candidate))
            .collect();
        forms.sort();
        forms.dedup();
        if forms.is_empty() {
            outln!("{candidate}{pad}  free");
            continue;
        }
        all_free = false;
        let indent = " ".repeat(width + 2);
        for (n, form) in forms.iter().enumerate() {
            if n == 0 {
                outln!("{candidate}{pad}  {form}");
            } else {
                outln!("{indent}{form}");
            }
        }
    }
    Ok(if all_free {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// The refusal of a candidate outside the id grammar: the grammar, and, for a candidate holding
/// `@`, the repair of giving the id alone.
fn not_an_id(candidate: &str) -> String {
    // A candidate holding `@` was given as a whole reference, whose repair differs from a
    // malformed id's, so the refusal names it. The clause names no kind and no anchor, since a
    // goal's reference names none; it does not print the last segment as the id to try, since
    // that segment may itself be outside the grammar.
    let repair = if candidate.contains('@') {
        "; give the id alone, the part after the last `@`"
    } else {
        ""
    };
    format!("not an id: an id matches [a-z0-9]+(-[a-z0-9]+)+{repair}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim: a candidate holding `@`, a goal's reference included, is refused with the repair
    /// of giving the id alone after the grammar, and one without `@` with the grammar alone.
    /// Mutation checked: in `not_an_id`, the condition `candidate.contains('@')` replaced by
    /// `true`, which adds the repair to `two`, and by `false`, which drops it from the references;
    /// both caught.
    #[test]
    fn not_an_id_adds_the_repair_for_a_reference_alone() {
        let grammar = "not an id: an id matches [a-z0-9]+(-[a-z0-9]+)+";
        let repair = "; give the id alone, the part after the last `@`";
        assert_eq!(
            not_an_id("design@core-mock@first-head"),
            format!("{grammar}{repair}")
        );
        assert_eq!(not_an_id("goal@top-goal"), format!("{grammar}{repair}"));
        assert_eq!(not_an_id("two"), grammar);
    }

    /// The claim: `id` with no id is refused, where a loop over nothing would read as every id free.
    #[test]
    fn id_refuses_no_id() {
        let text =
            "[project]\nname = \"probe\"\nchecker-version = \"fixture\"\ncomponents = []\n\n\
                    [walk]\nskip-dirs = []\nskip-files = []\n\n[agents]\nharness = []\n";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("the manifest parses");
        assert!(super::id(&manifest, &[], &[]).is_err());
    }
}
