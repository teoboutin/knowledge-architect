---
kind: design
---
# Decisions argued from a second agent harness, which the scope goal now asks against

## Summary

`goal@serves-one-kind-of-project` asks every target project's agents to run
under the `claude` harness, excludes a mode of the checker for another harness, and guarantees
knowledge-architect's working only as far as a project follows what it asks. Several decisions
defeated a rival, or keep a rejected design, on the ground that another harness would lack
something. That ground stands on no goal.

## Details

### What

The instances, found by the review before the merge of the branch that narrowed the goals:

- `design@agent-skills@audit-axis-runs-a-workflow`: the rival of a skill written for the Workflow
  tool alone lost on its failing in every other harness, and the head's title says the saved
  workflows are "their one home for any harness";
- `design@agent-skills@shared-text-fragment`: the rival of a harness hook delivering pointed
  sections lost on another harness lacking the hook;
- `design@agent-skills@workflow-axis-corpus`: its definition "holds for a second harness", one of
  its reasons;
- `rejected@agent-skills@head-rules-in-decision-recording`, kept because "another agent provider
  may not load the primer"; `rejected@check@session-start-hook-primer`, kept because "another
  context or harness might not follow it"; `rejected@model@agent-configuration-anchor`, whose second
  reason is a skill served to two harnesses;
- the `instructions` bullet of `design@model@harness-kinds-cited-without-anchor`, which names a
  second harness.

Mechanism, in one sentence: these were argued while no goal named one harness, and the batch that
made `claude` an ask re-pointed their references without arguing them again.

### Why it matters

Each defeat or retention now rests on a case no goal covers. A head whose rival stands undefeated
may be wrong, and a `rejected` entry whose recording test is gone is kept for nothing. The first two
heads bind every session that edits the audit's saved workflows or the shipped fragments.

### What would close it

Each instance argued again under `skill@knowledge-architect-design` on the `claude` ask: the head
kept on a ground that still holds, or reversed; each `rejected` entry kept on a recording test of
`skill@knowledge-architect-decision-recording`, or its removal put to the owner.

### Re-entry

The next design discussion that changes what an audit's saved workflow does, or one that changes
how the shipped fragments are delivered, or a design-record audit pass, whichever comes first. A
test of the saved workflows that changes none of them is no such discussion.
