---
kind: todo
---
# No reviewer writes its own report file, so the dispatcher saves each report by hand

## Summary

`skill@knowledge-architect-review@what-review-leaves` asks the dispatcher to save each reviewer's
report whole. No installed reviewer agent is told to write its report to a file, so the report
arrives as a message and the dispatcher writes it out, token by token. The tools allow it: the
reviewers' tools are `Read, Grep, Glob, Bash`, the same as the auditor agents', which write their
drafts through the shell to the scratch directory their brief names.

## Details

### What

Instance: the retrospective of the session that reduced the grounding's reading, 2026-10-25:
sixteen reports arrived as messages, and the session saved condensed copies rather than whole ones,
against the instruction, since writing each whole meant restating it.

### Why it matters

A condensed copy loses what the record of the review needs, and the save follows from the report
with no judgement, per `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

### What would close it

A shape that saves a report with no restatement: a reviewer that writes its own report file in a
scratch directory the brief names, which the review invariants already give each subagent, while
it keeps from editing the tree; or a command that writes a subagent's last message from the
harness's transcript. Chosen under `skill@knowledge-architect-design`.
