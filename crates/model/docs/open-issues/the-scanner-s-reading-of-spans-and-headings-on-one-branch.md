---
kind: todo
---
# The scanner's reading of a span or a heading that differs from what the text means, fixed on one branch

## Summary

A cluster of the issue record: five entries on how the scan of a document reads a backticked span or
a heading line, which one branch closes together. Each member's own entry holds its observation, its
reason and its closing work; this entry holds only that they go together.

## Details

### What

The members:

- `issue@model@a-span-with-an-empty-head-is-malformed-against-its-head`
- `issue@model@a-span-whose-head-shortens-a-kind-passes-the-check-silent`
- `issue@model@a-slug-in-a-quotation-is-reported-as-a-misplaced-definition`
- `issue@model@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`
- `issue@model@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`

They go on one branch because each changes the scan or the candidate rule of
`path@model@src/entity.rs` and the source readers, and their unit tests. Found by a triage of the
whole issue record on 2026-11-11, which read every entry against the tree and sized it: each member
was judged to have a clear path to closing, and to be more work than a fix of a few dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.