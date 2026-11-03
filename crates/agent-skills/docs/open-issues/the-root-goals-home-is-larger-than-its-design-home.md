---
kind: question
---
# The root goals home is now larger than the root design home, which reopens the cost of having the searcher grade every goal

## Summary

`design@agent-skills@searcher-grades-goals` rests its cost on a goal being small beside the
entries of the clusters it joins, and says that goals homes that approach the size of their design
homes reopen the cost. After the review of the whole goal set, this repository's root goals home is
larger than the root design home. Does the cost of the searcher grading every goal, beside the
session's own reading of every goals home, still hold?

## Details

### What

Measured at the commit "The record of the review of the whole goal set leaves, its goals ruled and
written", with `git show <commit>:<path> | wc -c` on each goals home and on each design home's
files:

| Component | goals home, bytes | design home, bytes |
| --- | --- | --- |
| the root | 48144 | 43737 |
| core | 9583 | 253698 |
| agent-skills | 13600 | 413475 |
| gates | 2661 | 9608 |
| mutate | 2623 | 9746 |
| git | 2089 | 7975 |
| xtask | 2936 | 6113 |

The root's goals home, `register@knowledge-architect@goal`, exceeds its design home,
`register@knowledge-architect@design`. Over every Component, the goals homes hold 81636 bytes
against 744352 for the design homes. Re-take with `wc -c` over the goals homes and the design homes,
as the head says.

The head does not say whether "their design homes" is read per Component or over the whole
project. Per Component, the root meets the reopening condition; over the whole project, it does
not.

### Why it matters

The decision was the owner's, after design sessions missed a goal twice. A cost premise that no
longer holds leaves the decision on a ground its head no longer states, per `primer@goals-bind` and
`primer@intent-and-claims`. It strains `design@agent-skills@searcher-grades-goals`. A searcher
that receives the root's goals as context in every cluster spends its context on them, against
`goal@agent-skills@only-needed-instructions` if they do not change its grades.

### What would close it

The real cost judged from the worklist's packing: run `cargo klarch worklist --out <dir>` over this
repository, with the seeds of a recent design grounding, and measure how much of each agent's
worklist the root's goals take. Where the share is small, the head's cost sentence is re-argued on
that measurement; where it is not, the decision goes back to the owner with the measurement, under
`skill@knowledge-architect-design`.
