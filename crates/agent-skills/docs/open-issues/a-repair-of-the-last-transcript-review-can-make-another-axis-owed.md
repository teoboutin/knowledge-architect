---
kind: design
---
# A repair of the last transcript review can meet another axis's condition, and the review skill reads two ways on whether that axis is owed

## Summary

`skill@knowledge-architect-review` ends a review with the repairs of its last transcript review,
re-reviewing only a repair that changes a design head. A repair made on an item of that last review
can still reword a goal, list entries left unread, or record new rulings of the owner, each the
condition of another axis. Four sessions met it and each chose differently; every proposed fix
contradicts a head.

## Details

### What

The texts, in `path@agent-skills@content/skills/review/SKILL.md`:

- `skill@knowledge-architect-review@review-axes`: "A review owes every axis of this table whose
  condition holds"; the transcript reviewer runs "after every other axis has run, its repairs are
  committed and every item of their reports has its outcome";
- `skill@knowledge-architect-review@what-review-leaves`: "A repair that rewords a goal is drafted as
  its own item under `skill@knowledge-architect-goal-setting@drawing-out-intent`"; and "A repair
  that adds, removes or reverses a design head is reviewed by the decision-record axis, and by any
  other axis whose condition it meets", a repair of the last transcript review being reviewed after
  it, and any other repair being reviewed by no axis again.

The goals axis in merge mode, `agent@knowledge-architect-goal-reviewer`, reports "each goal the
range adds, rewords or removes whose ruling by the owner no commit message of the range records":
it checks that a ruling is recorded, not that it is recorded faithfully.

The text gives an answer for a goal repair (drafted under goal-setting) and for a repair that
changes no head (no axis again), while "owes every axis whose condition holds" reads as owing an
axis whose condition a later repair makes hold: two readings.

The instances, in the retrospective workflow files of the intake of forty files:

- the narrowed goals, 2026-11-01: the goals axis of the merge review found eight wordings a goal
  round's repairs had reached in part; drafting them under goal-setting needed a record file, two
  draft rounds and a ruling, and the owner's answers were written after the last review,
  unreviewed. Proposal: a repair that brings a goal's text to what the owner already ruled is
  checked by the goals axis of the same review and shown to the owner as a diff, with no draft
  batch, +1 instruction;
- the slice that moved `git` out, 2026-11-02: the transcript review's repair, on the owner's
  answer, listed entries left unread, which made the design-conformance axis owed after the
  transcript review; the session ran it and no second transcript review. Proposal: one clause
  naming an axis a repair makes owed;
- the slice that renamed `check`, 2026-11-04: the last transcript review found three goal items
  unruled; their rulings reworded two goals; the session could not tell whether that owes draft
  rounds, the goals axis or nothing, and asked the owner, who asked for one more goals review.
  Proposal: a goal reworded by a repair after the goals axis is read once more by that axis,
  unless the owner stops it, +1 instruction;
- the slice that put `cli` on top, 2026-11-06: an item of the last transcript review led to two new
  goals, a new head and 16 further rulings; no transcript reviewer read them, and the goals axis in
  merge mode found the commit message overstated or left out several. Proposal: the goals axis in
  merge mode also reads the commit message recording rulings made after the last transcript review;
  rival named, a second transcript review.

The shapes, each with the head it would change:

- one rule: a repair of the last transcript review is reviewed by each axis whose condition it
  newly meets (the decision-record axis for a head, the goals axis for a goal, design conformance
  for a list of unread entries), and the review ends with those reviews' repairs. Its trigger is
  read from the diff and the message, as the head's is, so the argument of
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` against a judged trigger does
  not cover it, but its title, that no other repair is reviewed again, is contradicted;
- the owner ruling each time, as the slice that renamed `check` did, at one question per branch;
- a second transcript review, at a whole-transcript read per repair, which
  `design@agent-skills@transcript-review-last-before-merge` ends;
- the goals axis widened to read the commit message's rulings, a fidelity check that
  `design@agent-skills@transcript-reviewer-agent` places in the transcript reviewer;
- the lighter route for a goal repair of the narrowed goals, which contradicts
  `design@agent-skills@goal-edit-ruled-alone`.

Choosing among them weighs the review's cost against what a ruling recorded after the last
transcript review risks, which `design@agent-skills@misstated-ruling-rated-by-what-it-changes`
rates.

The suspected mechanism, in one sentence: the review's end is defined by the head-changing repair
alone, while a repair of the last transcript review can meet the condition of any axis.

Re-entry point: the next review whose last transcript review's repair makes an axis owed, or the
owner's word, for a design discussion under `skill@knowledge-architect-design`.

Source: W2 of the analysis 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow, in the
commit "[intake] The analyses of forty received retrospective files, three outcomes for the
owner", with the findings it groups as cluster K.

### Why it matters

A ruling of the owner recorded after the last transcript review is read by no reviewer, against
`goal@the-owner-decides` and `goal@design-is-recorded-with-its-arguments`;
re-reviewing every such repair works against the efficiency that
`goal@agents-get-a-complete-workflow` is there to raise. It strains
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.

### What would close it

A discussion under `skill@knowledge-architect-design` that picks one of the shapes above, or keeps
the text with a stated answer to the two readings, records it in the head it changes, and words the
review skill so that one reading remains.
