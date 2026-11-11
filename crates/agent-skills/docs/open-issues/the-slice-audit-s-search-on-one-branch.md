---
kind: todo
---
# What the record search of a slice's design audit reads and seeds from, fixed on one branch

## Summary

A cluster of the issue record: two entries on the design audit of a milestone's slice, which one
branch closes together. Each member's own entry holds its observation, its reason and its closing
work; this entry holds only that they go together.

## Details

### What

The members:

- `issue@agent-skills@a-slice-audit-s-search-reads-the-whole-milestone-document-in-every-agent`
- `issue@agent-skills@a-slice-s-spec-with-no-section-of-decided-entries-gives-its-audit-no-seed`

They go on one branch because both change the design audit of `skill@knowledge-architect-planning`
and what `klarch worklist` is given for it. Found by a triage of the whole issue record on
2026-11-11, which read every entry against the tree and sized it: each member was judged to have a
clear path to closing, and to be more work than a fix of a few dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.