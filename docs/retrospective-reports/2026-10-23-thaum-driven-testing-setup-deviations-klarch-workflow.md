# 2026-10-23-thaum-driven-testing-setup-deviations-klarch-workflow

- **Received file**: 2026-10-23-thaum-driven-testing-setup-deviations-klarch-workflow.md, the
  workflow file of a session in thaum that worked one milestone slice from the owner's one-line order
  to its merge, with a review on ten axes and two defaults put to the owner.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`; the installed text and the checker's messages
  this file blames are the same at both.
- **Standing entries that bear on it**:
  - for W1: `design@agent-skills@milestone-split-by-lifetime`, `design@core@plan-document-sections-by-kind`;
  - for W2 and C1: see the clusters they point to.

## W1. Point 2 of working a slice has no place for a gap the plan document leaves to the slice's commit

- **a. What it says.** The milestone document said its new names are defaults the slice may change
  "with the reason in its commit"; the audit met gaps of that grain and answered them in the first
  commit's message, and a reviewer reported a departure from point 2. Both readings were defensible,
  and the plan's sentence sat outside "How the work is done", the section the skill lets a milestone
  extend. The fix proposed: point 2 lets a gap the plan document leaves to the slice's commit be
  answered there, the audit's record listing it. The file's own rival: no change, the plan's sentence
  is out of place.
- **b. Whether it still applies.** Yes. Point 2 of `skill@knowledge-architect-planning@working-a-slice`:
  "**Applied in place.** … Apply the answer in place, in the milestone document or the slice's spec,
  wherever the shape it changes is written. Commit the amended documents alone". The row of "How the
  work is done": "a pointer to `skill@knowledge-architect-planning@working-a-slice`, … and any
  addition the milestone's work needs". No other sentence lets a plan leave a gap to a commit.
- **c. Whether it is already recorded.** As the decision, `design@agent-skills@milestone-split-by-lifetime`:
  "the audit of a slice applies its findings in place, in whichever document holds the shape … The
  audit's commit message lists each finding with its gap, its answer and the decision it follows
  from, so an edit in place loses no record of what the audit found." The proposal contradicts it.
- **d. Its kind.** An interaction with the project's plan text, which departed from the installed
  skill outside the section that may extend it. The installed text is clear: the plan's sentence was
  the cause, and its repair is on the project's side of the interaction.
- **f. A better fix.** Rivals: the proposal, a reversal of the head's "in place" for one kind of gap;
  no change, the plan's sentence being the project's to move or delete. One instance.
- **h. The proposal.** No change in the installed text.
- **Default**: no change, as d says.
- **Outcome**: accepted.

## W2. A tripwire that a harvest row names still needs the owner's word

This finding is analysed with W3 of 2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow,
in `path@knowledge-architect@docs/retrospective-reports/2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow.md`.

## C1. Deleting a file-register entry leads through two refusals

This finding is analysed with C1 of 2026-10-21-thaum-reader-refusals-slice-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-21-thaum-reader-refusals-slice-klarch-workflow.md`,
with the judgement of the tripwire it meets.

## The standing answers

- **a.** Two rules acted on as if absent, each found by reviewers before the merge: an exception the
  owner approved widened on the ground of a recorded reading, which went to the owner as a default and
  was approved; a count in a commit message that contradicted another, and a tripwire said to fire
  when its clause did not.
- **b.** The rules stand: `primer@design-heads`, "A clause that widens it, narrows it, or drops part of
  it … goes to the owner, as a change of the decision"; `primer@verify-before-relying`.
- **d.** Lapses, caught by the review. The claims are instances of the recurrence analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`.
  The checker's findings in the session each named a real defect and its repair, as the file says.
- **h.** No change for the widened exception: the rule worked through the transcript review.
- **Default**: no change, as h says.
- **Outcome**: accepted.
