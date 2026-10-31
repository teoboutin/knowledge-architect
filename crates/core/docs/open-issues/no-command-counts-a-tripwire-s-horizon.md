---
kind: todo
---
# No command counts a tripwire's horizon

## Summary

The standing-state reviewer at every merge, and the tripwire auditor at every pass, count each
tripwire's horizon by hand: `git log -S` on its `Watches until` line for the commit that wrote it,
then a count of the occasions since. The occasions take a few fixed shapes, which a command could
count.

## Details

### What

Instances:

- the retrospective of the session that reduced the grounding's reading, 2026-10-25: the
  standing-state reviewer counted 41 tripwires so;
- C2 and W1 of the retrospective of the session that edited the primer's section on something met
  outside the task, 2026-10-29: the same count by hand, done first in a shallow clone, where the
  commits that wrote the lines were absent; the reviewer cloned again, and the full count found a
  horizon passing at the merge, ten commits changing the primer's source. In a clone of depth 3,
  `git log -S` on a `Watches until` line names the clone's boundary commit, so a count there comes
  out too low, with no error.

The shapes, per `skill@knowledge-architect-issue-tracking@tripwire-entry`: commits on the main
branch that change a named path or the guarded head's text, release tags, audit reports of an axis,
plan documents deleted at their landing, a relocation, a commit whose added lines are exactly the
lines it removes, counting as none; a horizon in free words stays the reviewer's to count.
`design@core@tripwire-horizon-unchecked` keeps the line out of the checks on the owner's choice; it
does not say whether a command that only counts, and judges nothing, is covered.

### Why it matters

The count follows from the tree and the history with no judgement, per
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, and is repeated over every tripwire at
every merge.

### What would close it

A discussion that settles whether a counting command stands against
`design@core@tripwire-horizon-unchecked`, then either the command, printing per tripwire the writing
commit and the occasions of each shape it recognises, less the relocations, and refusing to count
in a shallow clone; or this entry closed on that head.
