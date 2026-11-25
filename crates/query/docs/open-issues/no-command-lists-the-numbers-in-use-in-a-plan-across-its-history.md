---
kind: todo
---
# No command lists the item and label numbers in use in a plan

## Summary

A plan's items and its labels take the next number of their prefix and never reuse one, across a
milestone's documents, its harvested slices included. A session finds the next free number by
searching the plan's files by hand. No command lists the highest number in use per prefix.

## Details

### What

- `primer@labelled-items`: an item under a label "takes the next number of its prefix in the record
  the label reaches, from 1, and that number is never reused there. That record is a plan document,
  a milestone's documents counting as one".
- `skill@knowledge-architect-planning` numbers arguments "in one sequence across a milestone's README
  and its slice specs".
- The checker's duplicate check, `design@model@plan-items-by-section`, sees the documents in the
  tree, a harvested slice's frozen files included, per `design@model@a-harvested-slice-is-frozen`:
  a milestone keeps every slice's directory until it leaves, so an item number reused across its
  slices is reported. No check reads a label's number.
- The instance, from before slices were kept under harvested/: a session in rootpoint found the
  next argument number, the next criterion id and the next numbers of five label prefixes by
  searching `git log -p` over the milestone's directory.
  Reported in that session's retrospective, analysed at the intake of 2026-10-24. `cargo klarch
  --help` lists no such command.
- A further instance, in this repository's port of the gates to Windows and macOS: the session's
  questions to the owner took `Q1` and `Q2` more than once, for different items, across messages
  of the design discussion that all reached the plan's record, per the standing answers of the
  retrospective file 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow. The
  labels were given in the conversation before the plan document existed, so a command over the
  plan's files would list them only once its transcript is committed.

### Why it matters

A label number reused in a plan makes two items put to the owner share a name, which no check
sees, against `goal@tasks-a-tool-does-better-are-the-tool-s`: finding the numbers in use is a task with no judgement the installed text asks. The search by hand over the plan's files is a task of the kind
`goal@tasks-a-tool-does-better-are-the-tool-s` gives the published checker.

### What would close it

A command, shaped under `design@check@one-question-one-command`, that prints per prefix the highest
number in use in a plan, tested over a mock milestone whose harvested slice holds the highest
number; or `check` reporting a label number reused in a plan,
a rival that reports a reused number where the command lets a session avoid it; or the owner's
ruling that the search by hand stays.
