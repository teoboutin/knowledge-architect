---
kind: todo
---
# The documentation lints and a gate that builds the documentation, on one branch

## Summary

A cluster of the issue record: two entries on rustdoc in this repository, which one branch closes
together. Each member's own entry holds its observation, its reason and its closing work; this entry
holds only that they go together.

## Details

### What

The members:

- `issue@knowledge-architect@no-gate-builds-the-documentation`
- `issue@knowledge-architect@this-repository-warns-on-none-of-the-documentation-lints`

They go on one branch because both change the crates' lint tables and the gate list of
`design@xtask@gates-list-primary-home`, and the lints' findings are fixed by the same pass over the
public items. Found by a triage of the whole issue record on 2026-11-11, which read every entry
against the tree and sized it: each member was judged to have a clear path to closing, and to be
more work than a fix of a few dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.