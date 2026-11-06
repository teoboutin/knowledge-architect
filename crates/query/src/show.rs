//! `show`: one recorded entry, whole, and every reference to it, and the texts `worklist --out`
//! writes into its snapshot, so the snapshot holds what `show` prints.

use std::path::Path;
use std::process::ExitCode;

use crate::model::table::{Anchors, Candidate, Entities};
use crate::output::outln;
use crate::Manifest;

/// One recorded entry, whole, and every reference to it.
///
/// **The two failure codes are different questions**, per `design@cli@exit-code-ladder`: an argument
/// that is not reference-shaped could not be run and exits 2, and a reference the grammar accepts
/// that names nothing is a negative answer and exits 1. A reader who mistyped the grammar and a
/// reader who named a deleted entry need different things.
pub fn show(manifest: &Manifest, reference: &str, checker: &[&Path]) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    let reference = reference.trim_matches('`');
    let (kind, anchor, id) = match crate::model::table::candidate(reference, &anchors) {
        Candidate::Reference { kind, anchor, id } => (kind, anchor, id),
        // The owner stands where the anchor does: the table keys a harness kind's entity so.
        Candidate::Harness { kind, owner, id } => (kind, owner, id),
        Candidate::Malformed { why, .. } => {
            return Err(format!("{reference} is malformed: {why}"));
        }
        Candidate::AnchorInKindPosition { head } => {
            return Err(format!(
                "{reference} opens with {head}, an anchor, where the kind goes: write \
                 <kind>@<anchor>@<id>, one of {}",
                anchors.kinds_listed()
            ));
        }
        Candidate::NotOne => {
            return Err(format!(
                "{reference} is not a reference: write <kind>@<anchor>@<id>, the kind one of {}",
                anchors.kinds_listed()
            ));
        }
    };

    // The body first, then what points at it. A reader asking for an entry wants the entry; the
    // inbound list is what tells them what closing it would break.
    let mut found = false;
    if kind.is_harness() {
        // A skill or an agent prints whole, a section from its heading to the next heading of
        // its level or above, out of the walked document or the installed copy that defines it.
        let entities = Entities::build(&model, &anchors);
        if let Some(site) = entities.sites(&kind, anchor, id).first() {
            let doc = model
                .documents()
                .iter()
                .chain(model.installed())
                .find(|d| d.rel == site.file);
            if let Some(doc) = doc {
                let body = if kind.is_named() && anchor.is_empty() {
                    doc.text.trim_end().to_string()
                } else {
                    section_text(&doc.text, site.line)
                };
                outln!("{reference}  {site}");
                outln!();
                outln!("{body}");
                found = true;
            }
        }
    } else if kind.takes_a_path() {
        // A path's entity is the tree's, so it is shown from the walked document where there is
        // one and from the survey where there is not: a directory and an unwalked file both
        // exist and are both worth resolving, and neither has a body to print.
        let Some(a) = anchors.by_name(anchor) else {
            outln!(
                "no anchor is named {anchor}; the anchors are {}",
                anchors.listed()
            );
            return Ok(ExitCode::FAILURE);
        };
        let rel = a.path.join(id.trim_end_matches('/'));
        let present = || -> Result<bool, String> {
            Ok(model.documents().iter().any(|d| d.rel == rel)
                || crate::model::listing::survey(manifest, &model)
                    .map_err(|e| e.to_string())?
                    .present
                    .contains(&rel))
        };
        if kind.is_planned() {
            // A planned path is valid while it is absent, so it resolves either way, and what is
            // worth printing is the plans that cite it: the sites a conversion will edit, per
            // `design@model@planned-path-form`. A reference the check refuses resolves to nothing
            // here either. The listing holds no ignored file, so an absent target and an
            // ignored one read the same.
            if let Err(cause) = crate::model::resolve::planned_target(anchor, id, &anchors) {
                let (what, action) = cause.texts(reference);
                outln!("{reference} resolves to nothing: {what}");
                outln!("  → {action}");
                return Ok(ExitCode::FAILURE);
            }
            outln!("{reference}  {}", rel.display());
            outln!();
            if present()? {
                outln!("(the target now exists; each citation converts to the path form)");
            } else {
                outln!("(planned; the tree's listing does not hold the target)");
            }
            found = true;
        } else if let Some(doc) = model.documents().iter().find(|d| d.rel == rel) {
            outln!("{reference}  {}", doc.rel.display());
            outln!();
            outln!("{}", doc.text.trim_end());
            found = true;
        } else {
            let survey =
                crate::model::listing::survey(manifest, &model).map_err(|e| e.to_string())?;
            if survey.present.contains(&rel) {
                outln!("{reference}  {}", rel.display());
                outln!();
                outln!("(outside the walk; nothing to print)");
                found = true;
            }
        }
    } else if kind.is_register() {
        // A register's home is the anchors' and the listing's, not the table's: the path the tree
        // holds, in whichever shape, with the register's shape, or a word that the tree holds
        // neither. `*` names every component's own home. A reference the check refuses resolves
        // to nothing here either, with the check's own finding.
        if let Err(cause) = crate::model::resolve::register(anchor, id, &anchors) {
            let (what, action) = cause.texts(reference);
            outln!("{reference} resolves to nothing: {what}");
            outln!("  → {action}");
            return Ok(ExitCode::FAILURE);
        }
        let register = anchors
            .registers()
            .by_name(id)
            .expect("the check resolved the register");
        let listed = |p: &Path| model.listing().iter().any(|l| l == p || l.starts_with(p));
        let homes: Vec<&crate::model::table::Anchor> = anchors
            .all()
            .iter()
            .filter(|a| {
                if anchor == crate::model::table::EVERY_ANCHOR {
                    a.is_component
                } else {
                    a.name == anchor
                }
            })
            .collect();
        outln!("{reference}  the {id} register, shape {:?}", register.shape);
        outln!();
        for a in homes {
            let home = a.home_of(register);
            let file = home.file.clone();
            let held = if register.shape == crate::model::declared::Shape::Heading && listed(&file)
            {
                file.display().to_string()
            } else if listed(&home.dir) {
                format!("{}/", home.dir.display())
            } else {
                "(the tree holds neither shape)".to_string()
            };
            outln!("{}: {held}", a.name);
        }
        found = true;
    } else if let Some(record) =
        crate::model::table::records(&model, &anchors, &Entities::build(&model, &anchors), &kind)
            .into_iter()
            .find(|r| r.anchor == anchor && r.id == id)
    {
        outln!("{}", record_text(reference, &record));
        found = true;
    }
    if !found {
        outln!("{reference} resolves to nothing");
        return Ok(ExitCode::FAILURE);
    }

    let index = crate::model::table::Inbound::build(&model, &anchors);
    outln!();
    outln!(
        "{}",
        inbound_text(&cited_at(&index, &anchors, &kind, anchor, id))
    );
    Ok(ExitCode::SUCCESS)
}

/// A register entry as `show` prints it: its reference and its site, a blank line, its body.
///
/// `show` prints it, a blank line and [`inbound_text`]; `worklist --out` writes the same three
/// into its snapshot, so the snapshot holds what `show` prints rather than a second rendering of
/// it, per `design@check@one-question-one-command`.
pub(crate) fn record_text(reference: &str, record: &crate::model::table::Record) -> String {
    format!("{reference}  {}\n\n{}", record.site, record.body.trim_end())
}

/// What points at an entity, as `show` prints it: `referenced by nothing`, or `referenced at:`
/// with one indented site per line.
pub(crate) fn inbound_text(sites: &[crate::model::table::Site]) -> String {
    if sites.is_empty() {
        return "referenced by nothing".to_string();
    }
    let mut out = "referenced at:".to_string();
    for site in sites {
        out.push_str(&format!("\n  {site}"));
    }
    out
}

/// Every site that cites an entity, as `show` lists them.
///
/// A component's home of a register is cited through `*` too, and a change of that home's shape
/// concerns every such citation, so the list holds them with the named ones.
pub(crate) fn cited_at(
    index: &crate::model::table::Inbound,
    anchors: &Anchors,
    kind: &crate::model::table::Kind,
    anchor: &str,
    id: &str,
) -> Vec<crate::model::table::Site> {
    let mut sites = index.of(kind, anchor, id);
    if kind.is_register() && anchors.by_name(anchor).is_some_and(|a| a.is_component) {
        sites.extend(index.of(kind, crate::model::table::EVERY_ANCHOR, id));
        sites.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    }
    sites
}

/// The section of `text` whose level-two heading is at `line`, up to the next heading of level
/// two or one outside a fenced block.
fn section_text(text: &str, line: u32) -> String {
    let mut out = Vec::new();
    // The fence that is open, as its character and run length: a fence closes only on a run of
    // its own character at least as long followed by nothing, so a shorter run is its content.
    let mut fence: Option<(char, usize)> = None;
    for (n, l) in text
        .lines()
        .enumerate()
        .skip(line.saturating_sub(1) as usize)
    {
        let trimmed = l.trim_start();
        for c in ['`', '~'] {
            let run = trimmed.chars().take_while(|&x| x == c).count();
            match fence {
                None if run >= 3 => fence = Some((c, run)),
                Some((open, len))
                    if open == c && run >= len && trimmed[run..].trim().is_empty() =>
                {
                    fence = None
                }
                _ => {}
            }
        }
        let heading = fence.is_none() && (l.starts_with("## ") || l.starts_with("# "));
        if heading && n + 1 != line as usize {
            break;
        }
        out.push(l);
    }
    out.join("\n").trim_end().to_string()
}

#[cfg(test)]
mod tests {

    /// The claim: a section runs to the next heading of level two or one outside a fence, and a
    /// shorter run inside a longer fence does not close it. Mutation checked: a fence toggled on
    /// any run stops at the fenced heading; a level-one heading ignored runs past it.
    #[test]
    fn a_section_runs_to_the_next_heading_outside_a_fence() {
        let text = "# T\n\n## A `##a-section`\n\n````\n```sh\n## In the fence\n````\n\nend of A\n\n# Next\n\nafter\n";
        let section = super::section_text(text, 3);
        assert!(section.contains("end of A"), "{section}");
        assert!(!section.contains("after"), "{section}");
    }
}
