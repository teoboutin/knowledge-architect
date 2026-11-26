//! The findings of a file the manifest declares free of citations, in `[walk.citation-free]`, at
//! the level `all` or `entries`, and the repair each names.
//!
//! **The level is asked at three sites, and its rule is this module's alone.** In
//! `references::judge_part`, [`span_finding`] is asked before a span is resolved, since a span the
//! level refuses is not resolved, and [`repairs_at`] is asked once after each observation is
//! judged, over the findings it raised, since every lint keeps reading a matched file and only its
//! repair changes. [`installed`] reads the installed copies, which `judge_part` never sees. A
//! change to what a level refuses, or to the repairs it names, is a change of this module, and no
//! arm of `judge_part` names a level.
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
const ALL_REPAIR: &str = "rewrite the sentence in words, naming what it points at with \
     no reference and no path in backticks, or write a placeholder in angle brackets, as \
     `<kind>@<anchor>@<id>`, where the sentence illustrates a form: [walk.citation-free] all \
     declares this file free of citations. Never keep the span with its backticks removed";

/// The repair of every finding a link raises in a file at the level `all`, whose repairs
/// elsewhere name a `path` reference or a plan document's reference.
const ALL_LINK_REPAIR: &str = "rewrite the link in words, naming what it points at \
     with no link and no reference: [walk.citation-free] all declares this file free of \
     citations";

/// The repair of every finding a link raises in a file at the level `entries`, whose repairs
/// elsewhere name a `path` reference under an anchor of the project.
const ENTRIES_LINK_REPAIR: &str = "rewrite the link in words, naming what it points at with no \
     link, or cite only what every installing project holds, as a path under the anchor `*`, \
     `plans` or `elsewhere`: [walk.citation-free] entries declares this file free of citations \
     of the project's entries and anchors";

/// What a level's repair adds for an id written as a plain word, which is often the ordinary word
/// it spells.
const PLAIN_WORD_RENAME: &str = "; where the word is meant as an ordinary term, rename what it \
     names so that its slug is not an ordinary word, a goal through its project's goal-setting";

/// The repair of a span the level `entries` refuses, and of every other finding whose repair
/// elsewhere names a form of the project's entries or anchors.
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

/// Replace the repair of each finding `found`, which one observation raised in a file at `level`,
/// where it would name a form the level refuses; at no level, leave them.
///
/// **No repair of a matched file names what the file refuses**, since following it would raise
/// the level's own finding in place of the old one:
///
/// - at the level `all`, every repair: each names a reference form or a `path` form. A link's
///   findings take `ALL_LINK_REPAIR`, and every other finding `ALL_REPAIR`;
/// - at the level `entries`, every repair but three, which name only what the level admits: a
///   harness reference's that resolves to nothing, which names the skill, the agent or the
///   primer it cites; and a wrapped span's and an unquoted reference's whose span the level
///   admits, which name the span itself, on one line or in backticks. A link's findings take
///   `ENTRIES_LINK_REPAIR`, and every other finding `ENTRIES_REPAIR`: the id lints name a
///   reference to an entry, the unanchored-path lint, an anchor in the kind's position and a link
///   a `path` reference under an anchor of the project, and an admitted `path` or `register`
///   reference that resolves to nothing an anchor of the project to cite instead.
///
/// A finding on an id written as a plain word adds the advice to rename what it names.
pub(crate) fn repairs_at(
    level: Option<Level>,
    what: &Observation,
    anchors: &Anchors,
    found: &mut [Finding],
) {
    let Some(level) = level else {
        return;
    };
    let admitted = |span: &str| {
        let candidate = table::candidate(span, anchors);
        matches!(
            candidate,
            Candidate::Reference { .. } | Candidate::Harness { .. }
        ) && refused_at_entries(&candidate, anchors).is_none()
    };
    let repair = match (level, what) {
        (Level::All, Observation::Link(_)) => ALL_LINK_REPAIR.to_string(),
        (Level::All, _) => ALL_REPAIR.to_string(),
        (Level::Entries, Observation::Link(_)) => ENTRIES_LINK_REPAIR.to_string(),
        (Level::Entries, Observation::Span(span))
            if matches!(table::candidate(span, anchors), Candidate::Harness { .. }) =>
        {
            return;
        }
        (Level::Entries, Observation::WrappedSpan(span)) if admitted(span) => return,
        (Level::Entries, Observation::UnquotedReference { token, .. }) if admitted(token) => {
            return;
        }
        (Level::Entries, _) => ENTRIES_REPAIR.to_string(),
    };
    let rename = match what {
        Observation::PlainWord(_) => PLAIN_WORD_RENAME,
        _ => "",
    };
    for f in found {
        f.action = format!("{repair}{rename}");
    }
}

/// The findings of the level `entries` over the installed copies a glob of it matches.
///
/// **An installed copy is read for its backticked spans here, and for no other rule of the
/// walk**, per `design@check@installed-copies-read-at-the-level-entries`: the walk leaves it out,
/// per `design@check@installed-files-outside-the-walk`, so no lint reads it, a wrapped span and a
/// reference outside backticks included, and its bytes are judged apart, by the comparison with
/// the shipped set. A span the level admits is not resolved, since its resolution within the
/// shipped set is the shipped set's own test. A glob of `all` is matched against the walked files
/// alone, so a copy is read at the level `entries` or not at all, and a copy that is not text is
/// read by nothing here: under `check` and `check --staged` the comparison of its bytes reports
/// it, and `commits` compares no installed file.
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
