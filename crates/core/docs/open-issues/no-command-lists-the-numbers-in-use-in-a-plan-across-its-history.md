---
kind: todo
---
# No command lists the item and label numbers in use in a plan, its deleted documents included

## Summary

A plan's items and its labels take the next number of their prefix and never reuse one, across a
milestone's documents, slice specs that have left included. A session finds the next free number
by searching the plan's history by hand. No command lists the highest number in use per prefix.

## Details

### What

- `primer@labelled-items`: an item under a label "takes the next number of its prefix in the record
  the label reaches, from 1, and that number is never reused there. That record is a plan document,
  a milestone's documents counting as one".
- `skill@knowledge-architect-planning` numbers arguments "in one sequence across a milestone's README
  and its slice specs".
- The checker's duplicate check, `design@core@plan-items-by-section`, sees the documents in the
  tree only.
- The instance: a session in rootpoint found the next argument number, the next criterion id and the
  next numbers of five label prefixes by searching `git log -p` over the milestone's directory.
  Reported in that session's retrospective, analysed at the intake of 2026-10-24. `cargo klarch
  --help` lists no such command.

### Why it matters

A number reused across a deleted spec makes two items of one plan share a name in its history, which
no check sees, against `goal@knowledge-architect@structure-and-workflow-work-together`, "the tool
computes the workflow's work lists".

### What would close it

A command, shaped under `design@core@one-question-one-command`, that prints per prefix the highest
number in use in a plan and its deleted documents, tested over a mock milestone whose deleted slice
spec holds the highest number; or the owner's ruling that the search by hand stays.
