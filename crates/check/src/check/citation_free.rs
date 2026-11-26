//! The findings of a file the manifest declares free of citations, in `[walk.citation-free]`, at
//! the level `all` or `entries`, and the repair each names.
//!
//! **The level is asked where a span's finding is written**, in `references::judge_part`, so a
//! review of either rule reads that one arm, as the goals-home rule is read there; this module holds
//! the rule's own logic: which span a level refuses, the finding it raises, and the repairs a file
//! at the level `all` names in place of a reference form.
//!
//! **No repair here is the span with its backticks removed**, per
//! `design@agent-skills@plain-text-is-no-repair`: each names a rewrite in words, or a placeholder
//! in angle brackets for a sentence that illustrates a form, which no level refuses.

use std::path::Path;

use crate::model::declared::{CitationFree, Level};
use crate::model::document::Observation;
use crate::model::table::{self, Anchors, Candidate};
use crate::model::{Finding, Model};

/// The repair of every finding a span raises in a file at the level `all`: the reference, and
/// each lint whose repair elsewhere names a reference form or a `path` form, which the file
/// refuses. The cost the level accepts: such a file cannot print an existing path of the tree in
/// backticks.
pub(crate) const ALL_REPAIR: &str = "rewrite the sentence in words, naming what it points at with \
     no reference and no path in backticks, or write a placeholder in angle brackets, as \
     `<kind>@<anchor>@<id>`, where the sentence illustrates a form: [walk.citation-free] all \
     declares this file free of citations. Never keep the span with its backticks removed";

/// The repair of every finding a link raises in a file at the level `all`, whose repairs
/// elsewhere name a `path` reference or a plan document's reference.
pub(crate) const ALL_LINK_REPAIR: &str = "rewrite the link in words, naming what it points at \
     with no link and no reference: [walk.citation-free] all declares this file free of \
     citations";

/// What an id lint adds to [`ALL_REPAIR`] for an id written as a plain word, which is often the
/// ordinary word it spells.
const PLAIN_WORD_RENAME: &str = "; where the word is meant as an ordinary term, rename what it \
     names so that its slug is not an ordinary word, a goal through its project's goal-setting";

/// The repair of a span the level `entries` refuses.
const ENTRIES_REPAIR: &str = "rewrite the sentence in words, naming what it points at with no \
     reference, or cite only what every installing project holds: a skill, an agent or a section \
     of the primer, a path under the anchor `*`, `plans` or `elsewhere`, a register's home under `*` \
     or `plans`, or `register@goal`; where the sentence illustrates a form, write a placeholder in \
     angle brackets, as `<kind>@<anchor>@<id>`. Never keep the span with its backticks removed";

/// The finding on a span that parses as a reference, in a file at `level`, or `None` where the
/// level admits it or the file is at none. A span the level admits is resolved by the caller as
/// anywhere else; one it refuses is not, since its repair is to rewrite it.
///
/// At the level `all` every reference is refused, whatever it names and whether or not it
/// resolves. At the level `entries`, a span is refused by its kind and the anchor it names, as
/// [`refused_at_entries`] says. A candidate that is no reference, a malformed span or an anchor
/// in the kind's position, is refused at neither: its own finding is the caller's, with the
/// repair the level names.
pub(crate) fn span_finding(
    level: Option<Level>,
    candidate: &Candidate,
    rel: &Path,
    line: u32,
    span: &str,
    anchors: &Anchors,
) -> Option<Finding> {
    if !matches!(
        candidate,
        Candidate::Reference { .. } | Candidate::Harness { .. }
    ) {
        return None;
    }
    match level? {
        Level::All => Some(Finding::at(
            rel,
            line,
            format!("`{span}` is a reference, in a file [walk.citation-free] all declares free of citations"),
            ALL_REPAIR,
        )),
        Level::Entries => refused_at_entries(candidate, anchors).map(|why| {
            Finding::at(
                rel,
                line,
                format!(
                    "`{span}` {why}, in a file [walk.citation-free] entries declares free of \
                     citations of the project's entries and anchors"
                ),
                ENTRIES_REPAIR,
            )
        }),
    }
}

/// Why the level `entries` refuses a span, or `None` where it admits it.
///
/// **It admits what every installing project holds**: a harness kind but `instructions`, since
/// the shipped set defines every skill, agent and section of the primer; a `path` or a `register`
/// reference under a reserved anchor, `*` and `plans` naming what each project carries and
/// `elsewhere` nothing of the project; and `register@goal`, the root's home of a register only
/// the root carries. **It refuses what names this project**: an entry of any register, a plan
/// item included; a `path` or a `register` reference under an anchor the project declares or
/// constructs, a Component's, a location's or a plan document's; a `planned` reference, a form
/// legal only in a plan document; and `instructions@<slug>`, a section of this project's own root
/// CLAUDE.md. Any other candidate, such as a malformed span, is no reference, and is refused here
/// by nothing.
fn refused_at_entries(candidate: &Candidate, anchors: &Anchors) -> Option<&'static str> {
    match candidate {
        Candidate::Harness { kind, .. } if kind.name() == table::INSTRUCTIONS_KIND => {
            Some("names a section of this project's root CLAUDE.md")
        }
        Candidate::Harness { .. } => None,
        Candidate::Reference { kind, .. } if kind.is_planned() => {
            Some("is a `planned` reference, a form legal only in a plan document")
        }
        Candidate::Reference { kind, anchor, id } if kind.is_path() || kind.is_register() => {
            let root_home = kind.is_register() && anchors.registers().is_root_register(id);
            (!table::is_reserved_anchor(anchor) && !root_home)
                .then_some("names an anchor of this project")
        }
        Candidate::Reference { .. } => Some("names an entry of this project"),
        _ => None,
    }
}

/// An id lint's finding in a file at `level`: at the level `all`, its repair replaced by a
/// rewrite in words, since the reference form it names is what the file refuses; at any other
/// level, or at none, the finding as it is.
pub(crate) fn id_lint_at(level: Option<Level>, what: &Observation, found: Finding) -> Finding {
    if level != Some(Level::All) {
        return found;
    }
    let rename = match what {
        Observation::PlainWord(_) => PLAIN_WORD_RENAME,
        _ => "",
    };
    Finding {
        action: format!("{ALL_REPAIR}{rename}"),
        ..found
    }
}

/// The findings of the level `entries` over the installed copies a glob of it matches.
///
/// **An installed copy is read for its references here, and for nothing else**: the walk leaves
/// it out, per `design@check@installed-files-outside-the-walk`, so no other rule reports anything
/// in it, and a span the level admits is not resolved, since its resolution within the shipped set
/// is the shipped set's own test. A glob of `all` is matched against the walked files alone, so a
/// copy is read at the level `entries` or not at all.
pub(crate) fn installed(model: &Model, free: &CitationFree, anchors: &Anchors) -> Vec<Finding> {
    let mut out = Vec::new();
    for doc in model.installed() {
        let matched = free
            .globs()
            .any(|(level, glob)| level == Level::Entries && glob.matches(&doc.rel));
        if !matched {
            continue;
        }
        for l in &doc.observations {
            let Observation::Span(span) = &l.what else {
                continue;
            };
            let candidate = table::candidate(span, anchors);
            out.extend(span_finding(
                Some(Level::Entries),
                &candidate,
                &doc.rel,
                l.line,
                span,
                anchors,
            ));
        }
    }
    out
}
