# 2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow

- **Received file**: 2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: knowledge-architect 0.9.0, the version the project pins.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: none beyond those its sections cite.

## W1: whether a split of a design head is "adding" a head, for the re-review rule

Analysed with W5 of 2026-11-02-knowledge-architect-development-and-components-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-development-and-components-klarch-workflow.md`.

## W2, with W1 of 2026-11-03-thaum-engine-driven-halts-klarch-workflow: a head bundling several decisions is reached one split at a time

- **a.** What it says.
  - W2: `skill@knowledge-architect-decision-recording@head-shape`, "A head holds one decision. Two statements are one decision when they answer one
    question and lose to the same nearest rival." The harvest wrote a head bundling four decisions;
    the first reviewer reported two, the next found a fourth. No instruction asks the writer or the
    reviewer to list every rival a head's statements lose to before judging it one decision.
    Proposal: the `skill@knowledge-architect-decision-recording@head-shape` section asks, before a head is written, for each statement's nearest
    rival, and one head per rival. +1.
  - W1 of the engine file: the spec's harvest row sent three threads with different nearest rivals,
    which its "Losing alternatives" listed apart, to one existing head; no plan reviewer flagged it;
    three decision-record rounds split it in turn. Cause: the cut is made at the harvest row, the
    test applied only at the review before the merge. Proposal: the design-conformance plan reviewer
    applies the test to each harvest row that sends several threads to one head. +1.
- **b.** Whether it still applies: yes. `path@agent-skills@fragments/head-shape.md`, lines 57 to 64,
  read the same at `v0.9.0` and at HEAD. Point 6 of
  `path@agent-skills@content/skills/planning/SKILL.md`, line 468, at `v0.9.0` and at HEAD: "the
  rules on a head's shape decide how the decisions are cut into heads, and they govern where the two
  disagree", so the row's cut binds nothing against the rule.
- **c.** Whether it is already recorded: `design@agent-skills@one-decision-per-head` holds the
  criterion and its nearest rival, `rejected@agent-skills@head-split-by-independent-reversal`. No
  issue records the passes. Instances: these two, in two sessions of one project on 0.9.0, each two
  or three decision-record passes; the engine file's standing answer adds that its harvest
  subagent followed the row. This repository's history holds review repairs that split heads,
  8cf468fc ("the timing head split in two") and ad97d553 ("the heads one decision each"), read from
  their subjects only: whether each took more than one pass is not established.
- **d.** Its kind: a conformance gap, a move the agent has and does not make in full: the writer, and
  each reviewer, test a head against one rival at a time.
- **e.** Its scope: installed text, the `skill@knowledge-architect-decision-recording@head-shape` fragment, hosted by the decision-recording skill,
  the decision-record reviewer and the design-record auditor. Neither proposal changes the
  criterion; neither brings back `rejected@agent-skills@head-split-by-independent-reversal`.
- **f.** A better fix: one fix for the cluster, W2's, in the `skill@knowledge-architect-decision-recording@head-shape` fragment: before a head is
  written or judged one decision, each of its statements is given its nearest rival, and statements
  that share one rival form one head. It reaches every writer (a harvest, an in-change commit, a
  review repair, a harvest subagent briefed with the decision-recording skill) and, through the same
  fragment, the decision-record reviewer, so one pass finds every bundle. The engine file's rival, a
  check at plan time by the design-conformance reviewer, loses: point 6 already makes the `skill@knowledge-architect-decision-recording@head-shape`
  rules govern the row's cut, so a row that bundles does no harm when the writer applies the rule,
  and a check at plan time adds a second home for the one test.
- **g.** Its route: an edit of the installed text, recorded under
  `skill@knowledge-architect-decision-recording` as a delivery that belongs to the head of
  `design@agent-skills@one-decision-per-head` ("its delivery belong to its head").
- **h.** The proposal: the sentence above in `path@agent-skills@fragments/head-shape.md`.
  Instructions: 1 added, 0 removed. It is an instruction about how an agent behaves, so
  `skill@knowledge-architect-retrospective-intake@admission-rules` asks the owner to name what a
  session would have lacked without it, a fact only the owner holds.
- **Default**: handle now.
- **Outcome**: for the owner (a fact only the owner holds: what the sessions lacked without the
  rule): pending.

## W4: the boundary between an argument of the owner and an intent to draft as a goal

Analysed with W2 of 2026-11-01-knowledge-architect-pre-release-fixes-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-pre-release-fixes-klarch-workflow.md`.

## W5: harvest items moved to a later slice, and the milestone document left assigning them here

- **a.** What it says: the slice's harvest row named five items whose work the next slice builds. The
  session moved them to the next slice's row, on "A decision is recorded when the work that
  implements it lands". Three reviewers found the milestone document still assigning them to this
  slice in six places. Proposal: point 6 says a row item moved to a later slice is moved in every
  plan document that names it. +1.
- **b.** Whether it still applies: the instruction that covers it stands, point 7 of
  `skill@knowledge-architect-planning@working-a-slice`, line 476 of
  `path@agent-skills@content/skills/planning/SKILL.md`, at `v0.9.0` and at HEAD: "What crosses slices
  stays in the milestone document, amended in place where the landing changed it."
- **c.** Whether it is already recorded: no entry.
- **d.** Its kind: a lapse, once: the landing changed which slice lands the items, and point 7 asks
  the milestone document to be amended in place where the landing changed it.
- **e.** Its scope: installed text, the planning skill. Reverses nothing.
- **f.** A better fix: the file's +1 is the rival; it loses on one instance of a lapse of an
  instruction that exists, per `skill@knowledge-architect-retrospective-intake@admission-rules`.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: point 7 of
  `skill@knowledge-architect-planning@working-a-slice` covers the amendment; one instance.

## W3 and C1: the audit's record costs one line per graded entry, almost all of one outcome

Analysed with C3 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## C2: agents filling a worklist report wrote scripts

Analysed with C2 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## The standing answers

- **a.** Items:
  - Installed file: no. An observation.
  - The search claim in a harvest message, and the commands a commit was said to keep: analysed
    with the claim-check cluster of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - An approval of a default recorded as the owner's choice at a fork: analysed with the
    attribution cluster of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.
  - No pointer written bare.
  - Hand tasks: C1 is cluster B (G1); C2 is cluster L (G1).
- **Default**: no change.
- **Outcome**: decided by the session: no change for the observation analysed here. Ground: no
  text is blamed.

---
