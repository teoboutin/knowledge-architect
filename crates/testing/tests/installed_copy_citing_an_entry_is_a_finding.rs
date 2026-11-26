//! The level `entries` of `[walk.citation-free]` over a project that serves the `claude` harness,
//! its shipped set installed: the cases of the harness kinds, which name no kind under
//! `harness = []`, and the read of the installed copies, which only an installed set holds.
//!
//! Each test runs `klarch` in a copy of `minimal`, served for `claude` and its set installed, as
//! `support::harnessed` makes it, its manifest declaring the table.

mod support;

use support::{harnessed, Sandbox};

/// The globs of the installed copies: the primer, each installed skill and each installed agent.
const INSTALLED: &str = "entries = [\".claude/knowledge-architect/PRIMER.md\", \
     \".claude/skills/knowledge-architect-*/\", \".claude/agents/knowledge-architect-*.md\"]";

/// The installed skill copy a test plants a span in.
const COPY: &str = ".claude/skills/knowledge-architect-design/SKILL.md";

/// A reference to an entry the sandbox defines, the design head of `minimal`.
const ENTRY: &str = "`design@minimal@mock-anchor`";

/// A harnessed copy of `minimal` whose manifest declares `[walk.citation-free]` with `table`,
/// staged.
fn declaring(tag: &str, table: &str) -> Sandbox {
    let sandbox = harnessed(tag);
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    sandbox.write(
        "knowledge-architect.toml",
        &format!("{manifest}\n[walk.citation-free]\n{table}\n"),
    );
    sandbox.stage();
    sandbox
}

/// Plant `line` at the end of the installed copy `COPY`, staged.
fn plant(sandbox: &Sandbox, line: &str) {
    let text = std::fs::read_to_string(sandbox.path(COPY)).expect("the installed copy");
    sandbox.write(COPY, &format!("{text}\n{line}\n"));
    sandbox.stage();
}

/// The findings `check` printed at `rel`, each as its first line.
fn findings_at(out: &str, rel: &str) -> Vec<String> {
    out.lines()
        .filter(|l| l.starts_with(&format!("{rel}:")) || l.starts_with(&format!("{rel}  ")))
        .map(str::to_string)
        .collect()
}

/// The claim, claim 9 over the sandbox: the installed set, read at the level `entries` with
/// the globs of the installed copies declared, passes. Mutation checked, caught: in
/// `refused_at_entries` of `path@check@src/check/citation_free.rs`, the arm that admits a
/// harness kind made to refuse it, so every `skill@<name>` citation of the copies is reported.
#[test]
fn the_installed_set_passes_the_level_entries() {
    let sandbox = declaring("free-installed", INSTALLED);
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim, claim 10, the closing test of the issue the slice answers: a reference to an entry
/// of the project planted in an installed skill's copy, with `entries` declared over the
/// installed copies, makes `check` exit 1 with the level's finding at the copy. Mutation
/// checked, caught: the read of the installed copies skipped, `citation_free::installed` made to
/// read no copy.
#[test]
fn an_installed_copy_citing_an_entry_is_a_finding() {
    let sandbox = declaring("free-cited", INSTALLED);
    plant(&sandbox, &format!("A planted citation: {ENTRY}."));
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    let found: Vec<String> = findings_at(&out, COPY)
        .into_iter()
        .filter(|l| l.contains("names an entry of this project"))
        .collect();
    assert_eq!(found.len(), 1, "{out}");
    assert!(found[0].contains(ENTRY), "{out}");
}

/// The claim, claim 4: the level `entries` reads an installed copy only where a glob matches it.
/// The copy planted with a reference to an entry is the level's finding with the glob declared,
/// and no finding of it without; and no rule of the walk reports anything in the copy either
/// way, the comparison of its bytes with the shipped set apart, which reports the planted copy
/// in both runs. Mutation checked, caught: the glob's match in `citation_free::installed` made
/// to hold for every copy.
#[test]
fn an_installed_copy_is_read_only_where_a_glob_matches_it() {
    let line = format!("A planted citation: {ENTRY}, and an unanchored `notes/a.md`.");
    let declared = declaring("free-copy-matched", INSTALLED);
    plant(&declared, &line);
    let unmatched = declaring(
        "free-copy-unmatched",
        "entries = [\".claude/knowledge-architect/PRIMER.md\"]",
    );
    plant(&unmatched, &line);
    let (with, err, _) = declared.run(&["check"]);
    let (without, err_without, _) = unmatched.run(&["check"]);
    let level = "names an entry of this project";
    let others = |out: &str| -> Vec<String> {
        findings_at(out, COPY)
            .into_iter()
            .filter(|l| !l.contains(level) && !l.contains("this installed file differs from what"))
            .collect()
    };
    let at_level = |out: &str| {
        findings_at(out, COPY)
            .iter()
            .filter(|l| l.contains(level))
            .count()
    };
    assert_eq!(at_level(&with), 1, "{with}{err}");
    assert_eq!(at_level(&without), 0, "{without}{err_without}");
    assert_eq!(others(&with), Vec::<String>::new(), "{with}");
    assert_eq!(others(&without), Vec::<String>::new(), "{without}");
}

/// The claim, claim 3, its cases of the harness kinds: at the level `entries`, in a walked file,
/// `skill@<name>`, `agent@<name>` and `primer@<slug>` of the shipped set are admitted, each
/// resolved, so one naming no shipped skill is reported as dangling; and `instructions@<slug>`
/// is the level's finding. Mutation checked, caught: in `refused_at_entries`, the arm that
/// refuses `instructions` removed, so it is admitted.
#[test]
fn the_level_entries_admits_the_harness_kinds_but_instructions() {
    let file = "free/entries-harness.md";
    let sandbox = declaring("free-harness", "entries = [\"free/entries-*.md\"]");
    sandbox.write(
        file,
        "# The harness kinds at the level of the entries\n\n\
         A skill: `skill@knowledge-architect-design`.\n\n\
         An agent: `agent@knowledge-architect-goal-reviewer`.\n\n\
         A section of the primer: `primer@working-prose`.\n\n\
         A section of the root instructions: `instructions@a-section`.\n\n\
         A skill the shipped set does not hold: `skill@knowledge-architect-nonesuch`.\n",
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}{err}");
    let found = findings_at(&out, file);
    assert_eq!(found.len(), 2, "{out}");
    assert!(
        found[0].starts_with(&format!(
            "{file}:9  `instructions@a-section` names a section"
        )),
        "{out}"
    );
    assert!(
        found[1].starts_with(&format!("{file}:11  `skill@knowledge-architect-nonesuch`")),
        "{out}"
    );
    assert!(
        !found[1].contains("in a file [walk.citation-free]"),
        "{out}"
    );
}
