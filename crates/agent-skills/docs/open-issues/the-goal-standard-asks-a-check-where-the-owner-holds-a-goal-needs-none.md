---
kind: question
---
# The goal standard asks for a check or a test where the owner holds that a goal often needs none

## Summary

Rule 6 of `skill@knowledge-architect-goal-setting@goal-or-not` says that an outcome depending on nothing the project
does not control shows itself met by a check or a test over what the project builds. At the goal of `cli`, the owner
held the reverse as a general view: most goals cannot be met by a mechanical test, those are probably the goals of most
value, and the effort spent on a test that detects any breach of a goal should stay reasonable.

## Details

### What

Seen at `goal@cli@a-refused-invocation-does-no-work`. Its draft measure asked for a test that finds every command rather
than listing them, for one of four kinds of refused invocation. The goal reviewer graded that `blocks`, since the owner had
asked that future commands be covered without naming them, and the session proposed extending the walk to two more
kinds. The owner refused the extension: the goal is enough, it stands because someone could one day judge the effort too
high and disable a test, and a goal covers what argument alone might not defend, such as a cost later judged to outweigh
a value. The measure was then written to name the tests that exist. The reviewer applied the standard as it reads, so
this is no lapse of the reviewer.

Not established: how many of the project's goals name a mechanical measure only because rule 6 asks for one.

### Why it matters

It strains `design@agent-skills@goal-met-in-tree-or-in-use`, which sorts what shows a goal met. A drafter and a reviewer who follow rule 6 push each goal toward a test of every breach, and a session builds that test.
That spends effort the owner does not want, and it can narrow a goal to what a test can see, which rule 1 and rule 3
guard against for other reasons. It strains `goal@knowledge-architect@design-with-agents-as-counterparts` only as far as
the owner's time goes on refusing such measures.

### What would close it

A reading of rule 6 against the owner's view under `skill@knowledge-architect-goal-setting`, its wording in the installed
text changed or kept on the owner's word, with the review of the whole goal set judged for what the change owes. Its
source is `path@agent-skills@content/skills/goal-setting/SKILL.md`, edited as `path@agent-skills@CLAUDE.md` says.
