---
kind: todo
---
# The head on labelled items lists no prefix for the items of a goal-review record

## Summary

`design@agent-skills@ruled-items-labelled` gives a label that reaches a committed document a prefix
of its own kind, listed in its table. A record of a goal-setting batch under docs/goal-reviews/ is
such a document, and the table lists no prefix for its items.

## Details

### What

The record of the review of this repository's whole goal set labelled its items with the common
`Q` in its analysis, and with `R`, `T` and `S` for the questions of its three rounds. None is in
the head's table, and the head reserves `Q` for a label that stays in the conversation. The
goal-setting skill names no prefix for a batch's record. `primer@labelled-items` reads more widely
than the head: it has an item take "`Q<n>` or the prefix a skill gives it", numbered within the
record the label reaches.

### Why it matters

The head keeps prefixes distinct across the installed workflow so that the owner tells a kind of
item by its label. A session writing a batch's record picks a prefix on its own, and two records
can use one letter for two kinds.

### What would close it

The head's table gains the prefix of a goal-review record's items and the goal-setting skill names
it, or the head states that such a record uses `Q` as the primer admits, through
`skill@knowledge-architect-agent-configuration`.
