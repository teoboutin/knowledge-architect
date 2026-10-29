---
kind: design
---
# An item met outside the task and kept for the retrospective's workflow file is lost when no retrospective follows

## Summary

`primer@met-outside-the-task` sends an item whose fix lies in the installed text or in the checker
to the workflow's file of `skill@knowledge-architect-retrospective`. Nothing writes that item down
before the retrospective runs, and in three cases none runs after it is met, so the item reaches
no file. The owner accepted that loss for now and plans to rethink this part of the workflow.

## Details

### What

The item is listed in the turn's "Met outside the task" section, as "kept for the retrospective's
workflow file", and in nothing durable. Of the texts that write a workflow's file, the
retrospective is the only one a session runs on its own work: the project audit writes its own
file for its own findings, and the intake adds only a finding of a project file it analyses.
`skill@knowledge-architect-retrospective@when-offered` bounds when the retrospective runs:

1. the owner declines the offer: "A declined retrospective is not offered again in that session";
2. the retrospective already ran earlier in the session: it is offered "once per session, at the
   first of these moments", so an item met after a merge that already prompted it reaches no
   later retrospective;
3. the session ends without any of the three moments, such as a session the owner leaves without
   saying it is ending.

In each case the item survives only in the transcript. A fourth case is not established: a
retrospective that runs may still miss the item, since
`skill@knowledge-architect-retrospective@what-it-examines` reads back over the session's
instructions and does not name the items its turns listed as kept for its file. The intake's outcome of
`design@agent-skills@installed-fix-goes-upstream`, adding a finding to a workflow file not yet
sent, does not reach it: that outcome is for a finding of a project file under analysis.

Instances seen: none yet. The loss follows from the text as written, read when the routing was
added to the primer. The entry is a `design` entry because the owner named the re-entry point
below; the fit with "instances seen" is stated here.

### Why it matters

Every lost item is a finding on the installed text or the checker that never reaches this
repository, against `goal@knowledge-architect@the-workflow-improves-through-real-use`. It strains
`design@agent-skills@installed-fix-goes-upstream`, whose argument for the route is that the other
outcomes lose such a finding.

Suspected mechanism, in one sentence: the route gives the item no durable record between the turn
that meets it and the retrospective that would write it.

### What would close it

A design discussion under `skill@knowledge-architect-design` that decides where such an item is
recorded between the moment it is met and the moment it is sent upstream, and its work landed, or
the owner's ruling that the loss stays accepted, recorded in
`design@agent-skills@installed-fix-goes-upstream`.

### Re-entry point

The owner's next work on how findings reach the workflow's repository, which the owner said they
will take up.
