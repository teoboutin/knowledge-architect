---
kind: defect
---
# A finding on a slug written in a retired or a misplaced form names a repair blind to what the slug resolves to

## Summary

Two findings on a slug name a fixed repair, whatever the slug resolves to. A backticked `#<id>` or
`<word>#<id>` whose id is an entry of this project is reported as the retired slug reference form,
and its repair always reads "write `design@<component>@<slug>`": when the id is an issue, a goal,
a tripwire, a `rejected` entry or a plan item, that repair names the wrong kind. A definition
written outside the place that defines, a `##<slug>` in a plan document's prose or in a table cell,
is reported as misplaced, and its repair is "move it there, or delete it": when the slug is a
section of the root `CLAUDE.md`, a skill or another register that the writer meant to point at, the
repair does not name the reference form that would.

## Details

### What

Established by reading the code, not by a run.

The retired form:

- `names_this_project` in `path@core@src/check/references.rs` reports the span when
  `Entities::defines_id` holds for its id.
- `Entities::defines_id` in `path@core@src/entity.rs` is true for an id of any register whose kind
  is not a harness kind: design, goal, tripwire, rejected, issue, spec, milestone and the plan items.
- The finding for `Observation::Retired(RetiredForm::SlugRef(span))`, in `judge_part` in
  `path@core@src/check/references.rs`, writes the
  fixed repair "write `design@<component>@<slug>`; the form with a `#` is no longer read as a
  reference".

So a span such as a backticked `#<id>` whose id is an issue entry's gets a repair that names
a design reference. The reproduction: a mock project with one issue entry, and a document that writes its
id with a `#` in backticks; the finding's repair line names a design reference.

The misplaced definition:

- `item_definitions` in `path@core@src/entity.rs` reports a `##<id>` written in a plan document
  outside its item sections as "written outside the item sections of the plan … and defines
  nothing", with the repair "an item of a plan is defined at the end of a level-3 heading under one
  of its sections …; move it there, or delete it".
- `heading_definitions` in the same file reports a `##<id>` in a table cell, at the head of a plain
  line, in the middle of a line or at another heading level as "written … and defines nothing", with
  the repair "a slug is defined at the end of a heading at its register's level … move it there, or
  delete it".
- Neither looks the slug up among what the entity table defines. A thaum session on
  knowledge-architect 0.7.0 wrote, in a plan document, `##<slug>` meaning a section of its root
  `CLAUDE.md`; the finding named the move or the deletion, and the session found the form
  `instructions@<slug>` without it. Reported as C3 of that session's retrospective workflow file.

Reported by the L7 lens of the fifth pass of the agentic-workflow audit, as an item met outside
its axis, for the retired form; and by the retrospective named above, for the misplaced definition.

### Why it matters

`design@core@an-unresolved-reference-is-reported` gives each finding the repair it needs, and
`design@core@finding-names-the-repair` names each cause with its repair where the repair depends on
the cause. An agent that follows either message writes a reference of the wrong kind, which the
next check reports as unresolved, or deletes a pointer it meant to write, so the repair costs a
second round or a lost pointer.

### What would close it

Each finding names the reference form of the kind the slug resolves to, or every kind that defines
it when several do, keeping its present repair when the slug resolves to nothing, with a test over a
mock project for each: a backticked `#<id>` naming an issue entry, and a `##<slug>` in a plan
document's prose naming a section of the root `CLAUDE.md`.
