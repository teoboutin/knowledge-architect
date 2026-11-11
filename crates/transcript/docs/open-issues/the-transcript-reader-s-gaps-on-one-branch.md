---
kind: todo
---
# The transcript reader's gaps in what it prints and what it reads, fixed on one branch

## Summary

A cluster of the issue record: four entries on the reader and the verbs of `klarch transcript`,
which one branch closes together. Each member's own entry holds its observation, its reason and its
closing work; this entry holds only that they go together.

## Details

### What

The members:

- `issue@transcript@a-result-the-harness-cut-is-printed-as-whole`
- `issue@transcript@no-verb-takes-the-inventory-of-the-transcripts-line-kinds`
- `issue@transcript@some-inputs-read-as-an-answer-about-the-project`
- `issue@transcript@show-starts-only-at-a-line-the-owner-typed`

They go on one branch because each changes the reader in `path@transcript@src/` and its binary tests
over the fixtures of `path@testing@tests/transcripts/`. Found by a triage of the whole issue record
on 2026-11-11, which read every entry against the tree and sized it: each member was judged to have
a clear path to closing, and to be more work than a fix of a few dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.