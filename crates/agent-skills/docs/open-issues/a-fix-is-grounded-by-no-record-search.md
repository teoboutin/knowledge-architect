---
kind: deferred
---
# A fix that does not go through the design skill is grounded by no search of the whole record

## Summary

A session about to propose a fix reads the record of that code, per
`design@agent-skills@code-change-reads-its-record`: the entries its comments cite, the entries that
cite its file, the heads on its subject in its Component's design home, and the entries on it in
its `rejected` home. A head
that bears on the fix from another subject or another Component, a tripwire whose firing the fix
meets, or an issue the fix touches is not on that path. The search of the whole record that the
design skill's grounding dispatches would read them, at the cost of its agents.

## Details

### What

The rival was weighed in the design discussion of the spec "The primer defines the terms it uses,
and sends a session about to change code to the record of that code", and set aside because the
owner said a minimal fix would do for now: that session's search took 10 agents and about 1.07M subagent
tokens, summed from the agents' completion reports. The owner plans a goal on development work as a
whole, beyond the design, the plan and the review the installed skills cover, and stated it is not
for now. Not established: how often a fix meets an entry outside the record of its code.

Later, the owner stated that a remark of theirs, that development can probably not work in the long
term if changes are made without grounding in design records, concerns additions and new features,
not the fix of a bug that affects recorded design. The owner named an exception, a bug found to make
the design fail or to be inherent to it and so unfixable, which the remark would cover, and was not
sure such cases occur, not thinking they had met one.

### Why it matters

A fix that meets a head outside its code's record, or a standing entry a diagnosis did not list, is
found at the review before the merge at the earliest, after the work, against `goal@knowledge-architect@agents-work-without-drift`. It
strains `design@agent-skills@record-searched-before-the-work`, which keeps the search for the work
through the design skill, a plan or an intake.

### Trigger

The design of the developing skill that `goal@knowledge-architect@agents-get-a-complete-workflow`
asks, per `issue@knowledge-architect@the-workflow-has-no-skill-for-developing-branch-work-or-releasing`:
that design decides whether a fix dispatches the search, and with which seeds.
