---
kind: todo
---
# The goal standard lets a goal name a part of the project as it is built, or cite an entry other than a goal

## Summary

The owner ruled, at the drafting of the `git` Component's goals, that a goal never names a part of
the project as it is built, and never cites an entry other than another goal: a goal stays vague on
the shape of its outcome. The goal standard of `skill@knowledge-architect-goal-setting@goal-or-not`
states neither, and the goals homes of this repository break both.

## Details

### What

The owner's ruling, attributed: naming in a goal a part of the project that is already built sets
as an end what was a means; and a goal cites no issue, no design head and no entry other than
another goal, since a goal changes rarely and only as the owner drives it in detail, while agents
move the other entries on their own, and a goal tied to them could no longer change on its own
terms or let them change on theirs. The owner held that the standard should apply to every goal.

What the standard says today: its rule 1 makes a command's name, a flag or a signature a decision,
and says the argument that a design follows from a goal goes in the design's head; no rule says a
goal names no built part, and none limits what a goal cites. Its rule 8 asks a goal stating what a
Component is not for to name the design it excludes.

Instances in this repository's goals homes, found by the goal reviewer of the `git` goals, which
read every goals home whole:

- a citation of an entry other than a goal: `goal@agent-skills@one-skill-per-activity` cites a
  section of the agent-configuration skill; the head of `register@knowledge-architect@goal`, which
  is no goal, cites a design head for the lifecycle of goals;
- names of built parts: `goal@core@document-rules-are-checked` ("the rules the core compiles in"),
  `goal@core@projects-add-their-own-checks` ("no change to the core"),
  `goal@core@documents-render-as-a-linked-site` (an mdbook build),
  `goal@mutate@a-named-mutation-is-checked-safely` (a `cargo x` command) and
  `goal@xtask@repeated-tasks-are-automated` (the `klarch-` prefix of this repository's skills).

### Why it matters

A goal that names a built part binds the part, so a design change the goal does not intend, such as
the core's split into Components, rewrites goals the owner must drive. A goal that cites an issue
or a head is touched when the entry closes or moves. Both strain
`goal@knowledge-architect@the-owner-decides`, under which a goal changes only through the owner.

### What would close it

The goal standard of the installed goal-setting skill states both rules, with the goal reviewer's
standard, which reads it, and the rule 8 sentence brought to them; then a review of the whole goal
set, `skill@knowledge-architect-goal-setting@whole-set-review`, on this repository's goals, bringing
each goal to them under the owner's rulings.
