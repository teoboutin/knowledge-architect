---
kind: design
---
# A small Rust edit made inside another task is made without the development skills

## Summary

Three sessions made a one-line edit of this repository's Rust source inside a task that was not
Rust work, and none loaded `skill@knowledge-architect-development` and `skill@klarch-development`.
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

The instances, from this repository's retrospective project files:

- the intake of seventeen files, 2026-10-31, P1: a row of a test constant edited in an
  installed-text repair; analysed then as a lapse of a clear instruction, once;
- the intake of forty files, 2026-11-07, P1: a review repair changed one word of a test's
  assertion message in `path@check@src/check/references.rs`; the session ran fmt and the crate's
  tests, and the full gates later passed, but it did not load the development skill;
- slice 1 of the plan directories, 2026-11-13, its standing answer: a lint repair in Rust written
  without loading the development skills, in the same repair that decided a change against a
  head's recorded reading without the design skill.

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
- no change: each edit was one line, the gates passed on each, and the review judged the third. It
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
