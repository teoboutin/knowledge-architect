---
kind: defect
---
# A transcript file with no fields is labelled by a tag in its text, which the goal rules out

## Summary

`design@transcript@transcript-labels-in-code` labels a top-level file none of whose `user` lines carries a field, such as one of a session run with `claude -p`, by a leading harness tag. `goal@transcript@a-session-reads-past-sessions` rules out labelling a line the owner typed by a phrase in its text.

## Details

### What

`file_needs_fallback` of `path@transcript@src/reader.rs` decides it. An owner's line in such a file that opens with a pasted tag reads as the harness's.

### Why it matters

The goal prevails over the head, on the owner's word that the rule-out stands. The head's own title says "never by a substring of its text".

### What would close it

Such a file is read by some field the harness records, or its lines are counted as unlabelled and reported, and the head is rewritten under `skill@knowledge-architect-decision-recording`, with a test over the `labels` fixtures.
