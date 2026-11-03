---
kind: todo
---
# Goals of this repository still name parts of the project as it is built, until the review of the whole goal set brings them to the standard

## Summary

Rules 1 and 7 of the goal standard, argued in `design@agent-skills@goal-outcome-survives-empty-skeleton`
and `design@core@goals-home-cites-upward-only`, ask a goal to keep its meaning in the project emptied
of its design and its code, and to cite only goals and registers of its own or an enclosing
Component. The citations the check refuses are gone from this repository's goals homes, removed by
deletion alone on the owner's word; the goals that name a part as built, and those the deletion left
changed in meaning, wait for the review of the whole goal set.

## Details

### What

Goals naming a part that exists by a design choice, found at the commit that restores this entry with
`grep -n "compiles in\|no change to the core\|mdbook\|cargo x\|klarch-" crates/core/docs/goals.md crates/mutate/docs/goals.md tools/xtask/docs/goals.md`:

- `goal@core@document-rules-are-checked`: "the rules the core compiles in";
- `goal@core@documents-render-as-a-linked-site`: "an mdbook build";
- `goal@core@projects-add-their-own-checks`: "no change to the core";
- `goal@mutate@a-named-mutation-is-checked-safely`: a `cargo x` command;
- `goal@xtask@repeated-tasks-are-automated`: the `klarch-` prefix of this repository's skills.

No goal of this repository carries the "for instance" cases of
`design@agent-skills@goal-carries-reading-examples` yet.

### Why it matters

A goal that names a built part binds the part, so a design change the goal does not intend rewrites
goals the owner must drive, against `goal@knowledge-architect@the-owner-decides`.

### What would close it

The review of the whole goal set, `skill@knowledge-architect-goal-setting@whole-set-review`, on this
repository's goals, bringing each goal to the standard under the owner's rulings; the commit that
writes the goals the owner ruled last closes this entry.
