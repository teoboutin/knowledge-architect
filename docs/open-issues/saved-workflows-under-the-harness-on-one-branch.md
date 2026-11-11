---
kind: todo
---
# Two questions on how a saved workflow runs under the harness, answered on one branch

## Summary

A cluster of the issue record: two entries on the saved workflows a project holds beside the
installed ones, which one branch closes together. Each member's own entry holds its observation, its
reason and its closing work; this entry holds only that they go together.

## Details

### What

The members:

- `issue@check@a-project-workflow-can-declare-an-installed-workflow-s-name`
- `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule`

They go on one branch because both concern a project's saved workflows beside the installed ones
under .claude/workflows/, and how the harness and the checker treat the two kinds side by side.
Found by a triage of the whole issue record on 2026-11-11, which read every entry against the tree
and sized it: each member was judged to have a clear path to closing, and to be more work than a fix
of a few dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.