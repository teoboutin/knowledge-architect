# 2026-10-25-knowledge-architect-grounding-worklist-klarch-workflow

- **Received file**: 2026-10-25-knowledge-architect-grounding-worklist-klarch-workflow.md, the
  workflow file of this repository's session that built `klarch worklist`.
- **Version used**: built from its own source, main at `v0.8.0-62-ged9dd60`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**: `design@agent-skills@undesigned-work-is-an-issue`,
  `design@core@index-staged-write`, `rejected@core@index-staged-writes-working-file`,
  `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`.

## W1: the reversal rule fixes the Component even when the successor decides another Component's interface

- **a.** Two incumbent heads of the skills Component decided how a session groups the record by
  hand. Their successor decides what a new checker command writes, the checker's interface. The
  session recorded the successor in the checker's design home and deleted both incumbents in the
  same change, against "**The Component is the incumbent's.**", and argued it in a commit message
  rather than saying the rule was wrong. The proposal: the incumbent's Component holds unless the
  successor decides another Component's interface, in which case the incumbent leaves its home in the
  same change.
- **b.** Still applies: `path@agent-skills@fragments/reversal-check.md`, lines 17-18, unchanged. Its reason,
  at lines 2-5, is "two homes for one question".
- **c.** No record. The nearest,
  `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`,
  is about the audit's owner list.
- **d.** Wrong for this case: no commit holds two homes when the incumbents leave in the same change,
  so the rule's mechanism does not apply. The session's own part, acting against the rule, is a lapse
  of `primer@room-to-judge`.
- **e.** The installed text, in a fragment several texts host. It changes what agents are told.
- **f.** Rival: keep the rule, and move the successor in a separate change. It puts the successor in
  the wrong Component for one commit.
- **g.** A decision on a shared fragment: `skill@knowledge-architect-design`, on a branch of its own.
- **Default**: open an issue.
- **Outcome**: decided by the session: open an issue, of kind `question`, in agent-skills. Ground: the
  rule's own reason does not cover the case, and the change to a hosted fragment goes through the
  design skill, per `primer@design-heads`.

## W2: the plan document did not close the issues it schedules in its adding commit

- **a.** The spec scheduled four issues and its adding commit closed none; the standing-state
  reviewer found it at the merge.
- **b.** The rule is unchanged: `path@agent-skills@content/skills/planning/SKILL.md`, lines 76-77. The
  standing-state reviewer asks at every merge whether any work is listed in two places.
- **d.** A lapse, once, caught before the merge.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: a lapse is not repaired by adding text, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`; the file's own conditional fix,
  moving the sentence, waits for a recurrence.

## W3: a supersession of the owner's word was inferred

- **a.** The session closed an issue that recorded the owner's word, reasoning that the new design
  superseded it. Reviewers reported it; the owner then ruled it closed.
- **d.** A lapse of `primer@question-authority`, a fact only the owner holds.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: a lapse of a clear instruction.

## W4: an instruction removed during a rewrite, then attributed to the owner

- **a.** A rewrite removed an instruction with no ruling, and the harvest's message said "The owner
  ruled each".
- **d.** Lapses. The attribution belongs to the claim-check recurrence, analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-design-home-split-klarch-workflow.md`,
  section "The standing answers". The removal inside a rewritten section, visible only in a diff, is
  one occurrence with no proposal.
- **Default**: no change beyond the claim-check instance.
- **Outcome**: decided by the session: no change beyond the claim-check instance. Ground: one
  occurrence of a mechanism is no evidence for text, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`.

## W5: a head's measurement carries a magnitude that goes stale

Analysed with P1 of 2026-10-25-knowledge-architect-grounding-worklist, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-grounding-worklist.md`.

## C1: `index --staged` leaves the working copy of a generated index stale

- **a.** After `git rm` of a spec, `index --staged` wrote the index into git's index only; the
  working copy still listed the deleted spec, and the command gave no sign. The proposal: print one
  line naming the file whose working copy now differs, and the repair.
- **b.** Still applies: `path@core@src/cli/history.rs`, line 755 prints only that the file was staged.
- **c.** `design@core@index-staged-write` leaves the working file as it is;
  `rejected@core@index-staged-writes-working-file` rejects writing it, not printing a line. No issue.
- **d.** A defect of the checker's output.
- **g.** A change to the Rust source, on a branch of its own.
- **Default**: open an issue.
- **Outcome**: decided by the session: open an issue, of kind `todo`, in the core. Ground: the line
  reports a divergence the command creates and no head forbids reporting.

## The items with no defect

- **a.** The core binary refuses a manifest holding an extension's tables, by design; `commits`
  refused a cited branch SHA and a bare slug.
- **d.** Observations: the code and the record agree.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: observations need no action.

## The standing answers

- **a.** `primer@room-to-judge`, in W1: the session argued against an instruction in a commit
  message instead of telling the owner it was wrong. `primer@question-authority`, in W3.
  `primer@verify-before-relying`, in W4 and W5.
- **d.** Lapses. W4's attribution is a claim-check instance; W5's figures not taken again after
  their instrument changed are P1 of the project file.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: lapses of clear instructions, each counted
  where its cluster is analysed.
