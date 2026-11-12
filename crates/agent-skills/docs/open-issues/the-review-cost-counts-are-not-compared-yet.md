---
kind: deferred
---
# The review-cost counts that retrospectives report are not compared across branches yet

## Summary

The retrospective asks, for each branch merged in its session, for three counts: the review
rounds, the commits that repaired a review's findings, and the owner's corrections of drift, per
`design@agent-skills@review-cost-counts`. This entry holds the series, the figures taken before the
question included, until a comparison across branches shows whether the workflow lowers the cost of a
change.

## Details

### What

The counts reach this repository in the answers of received workflow files, and
`skill@klarch-retrospective-intake` adds each branch's counts to the series below. No comparison has been made, so
whether the workflow lowers the cost of a change, which `goal@agents-get-a-complete-workflow` asks,
and whether the dispatches of `design@agent-skills@review-cadence` are too many, has no
measurement.

The series, with the figures and the reports that preceded the question:

The evaluation of the project's direction that first asked for the counts measured, over this repository's history since the checker was
split from thaum, that a third of the commits were `[review]` commits and that the project's
record about itself took more churn than its Rust. Those figures are in no file, and the window
was four days, so they separate no bootstrap from a steady state; that is what the counts are
for. Re-take them with `git log --format=%s` over the range, counting subjects that open with
`[review]`.

Two sessions reported their review cost without the counts, as impressions the counts would test:

- the owner-authority milestone, one design discussion and five pull requests: 37 review agents;
  13 of the 30 commit subjects from the milestone document's commit, "The owner-authority
  milestone: a question goes to the authority that answers it, and the owner rules at a fork", to
  the last commit of its last slice, "[review] Repairs from the transcript review of slice 4: the
  review skill's label sentence, and the owner's report kept with its hedges", open with
  `[review]`. Three extra rounds were each forced by a repair that touched a head before the
  transcript review could run. Its retrospective proposes measuring before letting the
  decision-record review of a head repair run in the transcript review's batch.
- a slice in thaum, on knowledge-architect 0.6.0: one finding of the decision-record axis cost
  three reviews of that axis, through `design@agent-skills@existing-heads-on-touch`, a split
  touching a second head that was then found bundled.

Two retrospectives of this repository carried the counts, unasked, analysed at the intake of
2026-10-24:

- the intake of thirteen files of 2026-10-19: 4 review dispatches, one of seven axes, two of the
  decision-record axis on repairs that added heads, one transcript review; 1 appended `[review]`
  commit, with later repairs folded into it; 0 corrections of drift by the owner. Each repair that
  added a head brought the decision-record axis back, and each pass found real defects.
- the spec on defined ids: 3 dispatches, one of seven axes, a re-review of two axes after a repair
  split a head, the transcript review; 3 appended `[review]` commits; 0 corrections of drift.
- in part, the branch that built `klarch init`: a review before the merge on eight axes, the
  transcript review alone and last, three follow-up reviews of repairs made after it, and a CI
  failure fixed after the pull request was marked ready.

Two more sessions reported the cost of the decision-record re-review of a repair, analysed at the
intake of forty files, in W5 of 2026-11-02-knowledge-architect-development-and-components-klarch-workflow,
in the commit "[intake] The analyses of forty received retrospective files, three outcomes for the
owner":

- the development milestone, third and fourth slices: in the third, the first repair commit
  widened a head's rule, the session ran the transcript review next, and found the missed
  decision-record review only on re-reading the skill; that review, out of order, found three firm
  defects. In the fourth slice it ran in order and found five;
- a slice in thaum, 2026-11-02: a split of a head was read as adding heads, by
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`, whose trigger is a heading
  whose slug appears, and sent to the decision-record axis twice more: three passes of that axis
  before the transcript review.

### Why it matters

Until the counts are compared across branches, whether the workflow lowers the review rounds, the
repairs and the corrections that cost the efficiency `goal@agents-get-a-complete-workflow` is there
to raise is not known, and a proposal to add or cut workflow text, which `design@agent-skills@capabilities-not-structure`
weighs, is argued from intuition. The compared counts are also the evidence
`goal@the-workflow-improves-through-real-use` names: real sessions as the test
of the workflow. The count of drift corrections would also measure the supervision
`goal@agents-work-without-drift` saves.

### Trigger

A proposal to add or cut workflow text, or to change when a review runs, that argues from review
cost; or the owner's request for the comparison. The comparison then reads the series above, and
this entry closes with it.
