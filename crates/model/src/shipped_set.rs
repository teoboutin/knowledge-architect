//! The shipped text held to what the entity table reads, per `design@model@section-homes-carry-slugs`.
//!
//! These tests read the table's own vocabulary, none of which is public, so they stay beside it and
//! reach the shipped text by a dev-dependency, which builds into no consumer of this crate. No
//! installing project judges them: a reference in an installed copy is read by nothing.

use std::path::{Path, PathBuf};

use crate::manifest::Manifest;

fn manifest(extra: &str) -> Manifest {
    let text = format!(
        "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n{extra}\n\
         [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n"
    );
    Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
}

/// Every shipped file at its install path, with the default command in its placeholder.
fn shipped() -> Vec<(PathBuf, String)> {
    knowledge_architect_agent_skills::FILES
        .iter()
        .map(|(path, text)| (PathBuf::from(path), text.replace("{{command}}", "klarch")))
        .collect()
}

/// What the shipped set, installed as `copies`, owes and fails of the section rule and of its
/// own references: each finding of the entity table over the copies, each reference of a
/// harness kind that resolves within no copy, and each `instructions` reference, which
/// names a section of each project's own root CLAUDE.md.
fn shipped_set_violations(copies: &[(PathBuf, String)]) -> Vec<String> {
    use crate::entity::{candidate, Anchors, Candidate, Entities, INSTRUCTIONS_KIND};
    let m = manifest("");
    // The copies are read as walked documents, not as installed ones: an installed copy
    // reports nothing in a project, and the shipped set is held to the section rule here.
    let model = crate::model::Model::from_documents(copies.to_vec());
    let anchors = Anchors::declared(&m);
    let entities = Entities::build(&model, &anchors);
    let mut out: Vec<String> = entities
        .definition_findings()
        .iter()
        .map(|f| f.to_string())
        .collect();
    for doc in model.documents() {
        for l in &doc.observations {
            let crate::scan::Observation::Span(span) = &l.what else {
                continue;
            };
            let found = candidate(span, &anchors);
            // A malformed span headed by a harness kind is one this judges; any other head is
            // the walk's to judge, as the primer's import line, which ships on purpose.
            let head = span.split('@').next().unwrap_or_default();
            if let Candidate::Malformed { why, .. } = &found {
                if crate::entity::HARNESS_KINDS.contains(&head) {
                    out.push(format!(
                        "{}:{} writes `{span}`, malformed: {why}",
                        doc.rel.display(),
                        l.line
                    ));
                }
                continue;
            }
            if let Candidate::Harness { kind, owner, id } = found {
                if kind.name() == INSTRUCTIONS_KIND {
                    out.push(format!(
                        "{}:{} cites `{span}`, a project's own section",
                        doc.rel.display(),
                        l.line
                    ));
                } else if !entities.defines(&kind, owner, id) {
                    out.push(format!(
                        "{}:{} cites `{span}`, which resolves to nothing",
                        doc.rel.display(),
                        l.line
                    ));
                }
            }
        }
    }
    out
}

/// The claim, per `design@model@section-homes-carry-slugs`: every level-two
/// heading of each shipped skill, agent and the primer carries a slug unique within its file,
/// and every reference of a harness kind in the shipped text resolves within the shipped set,
/// citing no project's own root CLAUDE.md. The set is judged here because no installing
/// project reads a reference in its installed copies.
#[test]
fn the_shipped_set_cites_only_sections_it_defines() {
    let copies = shipped();
    assert!(!copies.is_empty());
    assert_eq!(shipped_set_violations(&copies), Vec::<String>::new());
}

/// The same judgement, shown to fail: a copy of the set with a reference to a missing section
/// of a sibling skill, an `instructions` reference, a malformed span and an unslugged heading
/// planted.
#[test]
fn the_shipped_set_judgement_reports_what_is_planted() {
    let mut copies = shipped();
    let (_, text) = copies
        .iter_mut()
        .find(|(p, _)| p.ends_with("knowledge-architect-review/SKILL.md"))
        .expect("the review skill ships");
    text.push_str(
        "\n## Planted\n\nSee `skill@knowledge-architect-design@no-such-section`, \
         `instructions@git-workflow` and `primer@a@b`.\n",
    );
    let found = shipped_set_violations(&copies);
    assert_eq!(found.len(), 4, "{found:#?}");
    assert!(
        found.iter().any(|f| f.contains("`primer@a@b`, malformed")),
        "{found:#?}"
    );
    assert!(
        found.iter().any(|f| f.contains("\"Planted\"")),
        "{found:#?}"
    );
    assert!(
        found
            .iter()
            .any(|f| f.contains("no-such-section`, which resolves to nothing")),
        "{found:#?}"
    );
    assert!(
        found.iter().any(|f| f.contains("a project's own section")),
        "{found:#?}"
    );
}

/// The claim: the shipped primer's section `primer@checker-model` names every reference kind the
/// checker accepts, every built-in register, and every reserved anchor word, so a session
/// reading any reference meets its kind there, per
/// `design@agent-skills@checker-model-in-primer`. A kind or a word added to the checker fails
/// this test until the section names it. Mutation: dropping `planned` from the section, or
/// adding a kind to `HARNESS_KINDS`, fails it.
#[test]
fn the_primer_s_model_section_names_every_kind_and_reserved_word() {
    use crate::entity::{
        ESCAPE_ANCHOR, EVERY_ANCHOR, HARNESS_KINDS, PATH_KIND, PLANNED_KIND, PLANS_ANCHOR,
        REGISTER_KIND,
    };
    let m = manifest("");
    let primer = shipped()
        .into_iter()
        .find(|(path, _)| path == Path::new(".claude/knowledge-architect/PRIMER.md"))
        .expect("the primer ships")
        .1;
    let start = primer
        .find("`##checker-model`")
        .expect("the primer has a section `checker-model`");
    let rest = &primer[start..];
    let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
    let registers = m.registers().all().iter().map(|r| r.name.as_str());
    let kinds = [PATH_KIND, PLANNED_KIND, REGISTER_KIND]
        .into_iter()
        .chain(HARNESS_KINDS)
        .chain(registers);
    for word in kinds
        .map(|k| format!("`{k}"))
        .chain([EVERY_ANCHOR, ESCAPE_ANCHOR, PLANS_ANCHOR].map(|w| format!("`{w}`")))
    {
        assert!(
            section.contains(&word),
            "the section `checker-model` does not name {word}"
        );
    }
}
