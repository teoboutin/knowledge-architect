---
kind: todo
---
# Mutation survivors left in three suites, caught on one branch

## Summary

A cluster of the issue record: three entries, each a mutation a suite lets survive, which one branch
closes together. Each member's own entry holds its observation, its reason and its closing work;
this entry holds only that they go together.

## Details

### What

The members:

- `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation`
- `issue@model@the-resolver-s-arms-have-mutation-survivors`
- `issue@testing@the-planted-test-judges-its-counts-only-as-a-sum`

They go on one branch because each is closed the same way, a test shown to catch a recorded mutation
with `cargo x mutate run`, and needs no design. Found by a triage of the whole issue record on
2026-11-11, which read every entry against the tree and sized it: each member was judged to have a
clear path to closing, and to be more work than a fix of a few dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.