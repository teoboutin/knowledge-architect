---
kind: question
---
# A retrospective that runs may miss an item a session kept for its workflow file

## Summary

`primer@met-outside-the-task` sends an item whose fix lies in the installed text or in the checker
to the workflow's file of `skill@knowledge-architect-retrospective`. The loss when no retrospective
follows is accepted, per `design@agent-skills@installed-fix-goes-upstream`. Whether a retrospective
that does run reaches every such item is not established.

## Details

### What

`skill@knowledge-architect-retrospective@what-it-examines` reads back over the session's
instructions, where they were followed, with difficulty or not, and what the owner corrected. It
does not name the items the session's turns listed under "Met outside the task" as kept for the
workflow's file, so a retrospective may write its file without one of them. Instances seen: none.
The question follows from the text as written, read when the routing was added to the primer.

### Why it matters

An item kept for the workflow's file and missed by the retrospective that runs reaches no file and
no issue, in a session where the owner accepted the retrospective, which the loss accepted in
`design@agent-skills@installed-fix-goes-upstream` does not cover.

### What would close it

A received workflow file, or the owner's report, that shows a kept item missed by the
retrospective that ran, which then makes the retrospective name those items in its read-back,
under `skill@knowledge-architect-agent-configuration@admission-standard`; or one whose session kept
items and whose file carries them all, which answers the question for that text.
