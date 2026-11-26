---
kind: design
---
# A small Rust edit made inside another task is made without the development skills

## Summary

Four small edits of this repository's Rust source were made inside another task, the first three by
a session and the fourth by an agent a session dispatched, two of them one-line edits inside text
work. None of the four loaded `skill@klarch-development`, and the first three did not load
`skill@knowledge-architect-development` either; whether the fourth did is not established.
The instruction is delivered at the session's start; the edit comes later, inside the other task.
Where to deliver it at the moment of the edit is a design question with three rivals.

## Details

### What

The instruction and its two deliveries, both read at the session's start:

- the root `CLAUDE.md`, `instructions@verify-before-relying`: "**Before touching the code of a
  Component, ground first, per `skill@knowledge-architect-development@ground-before-editing` and
  `skill@klarch-development@ground-in-this-repository`.**";
- the description of `skill@klarch-development`, which the harness lists in each session: "MUST
  use, together with `skill@knowledge-architect-development`, ... before starting or iterating on
  any change to this repository's Rust source".

The instances, from this repository's retrospective files, three project files and one workflow
file:

- the intake of seventeen files, 2026-10-31, P1: a row of a test constant edited in an
  installed-text repair; analysed then as a lapse of a clear instruction, once;
- the intake of forty files, 2026-11-07, P1: a review repair changed one word of a test's
  assertion message in `path@check@src/check/references.rs`; the session ran fmt and the crate's
  tests, and the full gates later passed, but it did not load the development skill;
- slice 1 of the plan directories, 2026-11-13, its standing answer: a lint repair in Rust written
  without loading the development skills, in the same repair that decided a change against a
  head's recorded reading without the design skill. By its details, the repair is the commit
  "[check] A plan document's directory is no place the first-segment lint reads a path from" on
  main: 60 lines added and 2 removed in `path@check@src/check/references.rs`, with a test and a
  `cargo x mutate` run, so no one-line edit. The slice also changed Rust code in its own work, in
  "[model] Plan documents as directories, with model types of their own, and the tree's plan
  documents migrated", so that this edit was made inside a task that looked like text work is an
  assumption, not established.
- the branch that built the record search's saved workflow, 2026-11-16, its workflow file's
  standing answer: one implementer of that branch's work did not load `skill@klarch-development` for
  two comment lines in Rust; nothing followed from it, as the file reports, and the file does not
  say whether the agent loaded `skill@knowledge-architect-development`. This instance meets the
  entry's re-entry point, "the next instance", and the edit was made by a dispatched agent under a
  brief, one of the homes the second rival names.

The suspected mechanism, in one sentence: an instruction read at the session's start is not
recalled at an edit of Rust made inside a task that looks like text work.

The same mechanism at the primer's list of installed skills, not Rust edits, named for the
discussion, which may weigh a delivery serving every such load: the session on the primer's import
line, 2026-11-11, edited docs/roadmap.md without loading `skill@knowledge-architect-planning`; slice
1 of the plan directories opened an issue entry without loading
`skill@knowledge-architect-issue-tracking`, and its dispatched agent made two edits of installed
text without loading `skill@knowledge-architect-agent-configuration` whole.

The rivals, none ranked by the record:

- a pointer to `skill@klarch-development` in each crate's scoped `CLAUDE.md`, read when a session
  works on that crate's files: twelve copies of one pointer; whether the harness loads a scoped
  `CLAUDE.md` at an edit made without a read of the directory is an assumption to measure;
- a sentence where the three tasks are delivered: the review skill's repair step, the section of
  `path@agent-skills@CLAUDE.md` on editing an installed skill, a brief; three homes for one moment;
- no change: the first two edits were one line, the gates passed on each, and the review judged
  the third. It
  loses on the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`, whose
  recurrence at one point of delivery moves the delivery; and the third repair carried a decision
  against a head that the development skill's grounding reads.

A delivery chosen may touch `design@agent-skills@no-routing-table`: a repair is no table in the
root mapping the project skill to the installed one, and the project's development skill stays an
overlay.

Re-entry point: the next instance, or the owner's word.

### Why it matters

The development skill's grounding reads the record of the code before an edit, per
`design@agent-skills@code-change-reads-its-record`; an edit made without it can undo a decision the
code does not show, as the third instance's did, against `goal@the-workflow-leads-to-better-code`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that picks a delivery at the moment of
a Rust edit made inside another task, or rules that none is worth its cost; the delivery written in
this repository's agent configuration.
