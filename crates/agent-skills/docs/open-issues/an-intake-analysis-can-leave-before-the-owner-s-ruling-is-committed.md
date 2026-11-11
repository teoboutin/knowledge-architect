---
kind: design
---
# An intake's analysis can leave before the owner's ruling in it is committed

## Summary

The retrospective intake commits its analysis as soon as it is written, owner items pending, and
lets the commit that carries out the last outcome, or a later one, delete it. Nothing orders the
commit that writes the owner's verbatim ruling into the analysis before the deletion. A ruling given
in the conversation and copied into the analysis by the commit that also carries out its outcome and
deletes the file sits verbatim in no tree, since a commit message attributes and does not quote.

## Details

### What

- The text: `skill@knowledge-architect-retrospective-intake@after-the-ruling`, "The analysis is
  committed as soon as it is written, under the project's own branch rules, owner items pending",
  and "The commit that carries out the last outcome, or a later commit of the same branch, deletes
  the analysis".
- The same loss was met and repaired for the record of a goal-setting batch, whose record is now
  committed with the owner's last answers before the goals are written, per
  `design@agent-skills@whole-set-rounds`; the record of a pass of a project audit keeps that order,
  per `skill@knowledge-architect-project-audit@record-of-an-audit`. The intake's analysis states no
  such order.
- Found by a checking subagent of the intake of forty retrospective files, met outside its task,
  and by the transcript review before that intake's merge. No real session has lost a ruling this
  way: that intake committed its three rulings in a commit of their own before any deletion. This is
  a prediction of a loss the other two records already met.

The suspected mechanism, in one sentence: the intake's deletion rule names the last outcome's commit
and no ruling's commit, so a session that carries out an outcome and records its ruling together
deletes the file in that same commit.

### Why it matters

The owner's verbatim words are kept only in a deliberation record, per `primer@design-heads`; a lost
ruling leaves the outcome attributed with no words to check it against, against
`goal@the-owner-s-word-is-verified`.

### What would close it

The installed intake states the order the goal record and the audit record keep: an owner's ruling
is committed in the analysis before the commit that deletes it, edited as `path@agent-skills@CLAUDE.md`
says, through `skill@knowledge-architect-design` since it changes a statement of
`design@agent-skills@committed-intake-analysis`; or a discussion that judges the order already
followed in practice, with the reason.
