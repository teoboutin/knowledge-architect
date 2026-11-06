//! `issues` and `tripwires`: the standing entries, one row each.

use std::path::Path;
use std::process::ExitCode;

use crate::model::declared::{ISSUE_REGISTER, TRIPWIRE_REGISTER};
use crate::model::table::{Anchors, Entities, Kind};
use crate::output::outln;
use crate::Manifest;

#[derive(Clone, Debug, Default)]
/// What `issues` lists.
pub struct IssuesFilter {
    /// Only the entries of this kind.
    pub kind: Option<String>,
    /// Only the entries in this group subdirectory.
    pub group: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    pub terms: Vec<String>,
}

#[derive(Clone, Debug, Default)]
/// What `tripwires` lists.
pub struct TripwiresFilter {
    /// Only the entries guarding this entry.
    pub guarding: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    pub terms: Vec<String>,
}

/// Every issue entry, one row each: exit 1 when no row is left, `Err` when the project cannot be read.
pub fn issues(
    manifest: &Manifest,
    args: &IssuesFilter,
    checker: &[&Path],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    let kind = Kind::new(ISSUE_REGISTER);
    let (anchor, needle) = split_terms(&args.terms, &anchors);
    let mut rows =
        crate::model::table::records(&model, &anchors, &Entities::build(&model, &anchors), &kind);
    rows.retain(|r| {
        anchor.is_none_or(|a| r.anchor == a)
            && args
                .kind
                .as_ref()
                .is_none_or(|k| r.metadata.as_deref() == Some(k.as_str()))
            && args
                .group
                .as_ref()
                .is_none_or(|g| r.group.as_deref() == Some(g.as_str()))
            && needle.as_ref().is_none_or(|t| r.matches(t))
    });
    // Kind then id, so entries of one kind read together whatever anchor they sit under.
    rows.sort_by(|a, b| (&a.metadata, &a.id).cmp(&(&b.metadata, &b.id)));

    // One `git log` for every instance directory at once. A per-row invocation is a process per
    // entry, and this column is not worth one.
    let dirs: Vec<std::path::PathBuf> = anchors
        .instances()
        .into_iter()
        .filter(|(_, register, _)| register.name == ISSUE_REGISTER)
        .map(|(_, _, home)| home.dir)
        .collect();
    let changed = crate::git::last_changed(manifest.root(), &dirs);

    let table: Vec<[String; 5]> = rows
        .iter()
        .map(|r| {
            [
                r.metadata.clone().unwrap_or_else(|| "-".to_string()),
                r.anchor.clone(),
                r.id.clone(),
                r.title.clone(),
                // No commit is `uncommitted`; git answering nothing at all is `-`, and the two
                // are different facts: the first is a new file, the second is no history to ask.
                match changed.get(&r.site.file) {
                    Some(date) => date.clone(),
                    None if changed.is_empty() => "-".to_string(),
                    None => "uncommitted".to_string(),
                },
            ]
        })
        .collect();
    print_rows(&["kind", "anchor", "id", "title", "last change"], &table);
    Ok(if table.is_empty() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Every tripwire entry, one row each: exit 1 when no row is left, `Err` when the project cannot be
/// read.
pub fn tripwires(
    manifest: &Manifest,
    args: &TripwiresFilter,
    checker: &[&Path],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    let kind = Kind::new(TRIPWIRE_REGISTER);
    let (anchor, needle) = split_terms(&args.terms, &anchors);
    let guarding = args
        .guarding
        .as_ref()
        .map(|g| g.trim_matches('`').to_string());
    let mut rows =
        crate::model::table::records(&model, &anchors, &Entities::build(&model, &anchors), &kind);
    rows.retain(|r| {
        anchor.is_none_or(|a| r.anchor == a)
            && guarding
                .as_ref()
                .is_none_or(|g| r.guards.iter().any(|c| c == g))
            && needle.as_ref().is_none_or(|t| r.matches(t))
    });
    rows.sort_by(|a, b| (&a.anchor, &a.id).cmp(&(&b.anchor, &b.id)));

    let table: Vec<[String; 4]> = rows
        .iter()
        .map(|r| {
            [
                r.anchor.clone(),
                r.id.clone(),
                r.title.clone(),
                r.guarding(),
            ]
        })
        .collect();
    print_rows(&["anchor", "id", "title", "guarding"], &table);
    Ok(if table.is_empty() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// The positional arguments of a listing: an anchor, then the text to filter on.
///
/// **The first term is the anchor only when something declares that name**, which is what lets
/// one positional list mean both. A search word that happens to be an anchor name is the one
/// ambiguity, and it is resolved towards the anchor because that is the documented first
/// position; a search for that word alone is written with the anchor before it.
fn split_terms<'a>(terms: &'a [String], anchors: &Anchors) -> (Option<&'a str>, Option<String>) {
    let mut rest = terms;
    let mut anchor = None;
    if let Some(first) = terms.first() {
        if anchors.by_name(first).is_some() {
            anchor = Some(first.as_str());
            rest = &terms[1..];
        }
    }
    let needle = (!rest.is_empty()).then(|| rest.join(" "));
    (anchor, needle)
}

/// A table with its header, each column as wide as its widest cell.
///
/// The header prints over an empty table too: a listing with no row has to say what it looked
/// for, or a filter that matched nothing reads as a project with nothing in it.
fn print_rows<const N: usize>(header: &[&str; N], rows: &[[String; N]]) {
    let mut width = [0usize; N];
    for (i, name) in header.iter().enumerate() {
        width[i] = name.chars().count();
    }
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            width[i] = width[i].max(cell.chars().count());
        }
    }
    let line = |cells: &[String; N]| {
        cells
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i + 1 == N {
                    c.clone()
                } else {
                    format!("{c:<w$}", w = width[i])
                }
            })
            .collect::<Vec<_>>()
            .join("  ")
    };
    let head: [String; N] = std::array::from_fn(|i| header[i].to_string());
    outln!("{}", line(&head));
    for row in rows {
        outln!("{}", line(row));
    }
    if rows.is_empty() {
        outln!("(no entry)");
    }
}
