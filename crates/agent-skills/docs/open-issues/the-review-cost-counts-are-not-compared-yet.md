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

The counts of the 26 workflow files of the intake of 47 files, 2026-11-07 to 2026-11-14, one line
per branch merged, as each file gives them. A file whose version precedes v0.9.0-385, where the
retrospective first asked for the three counts, gives what it reported unasked, and a count it did
not give is marked "not given":

- 2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow: 4 rounds (nine axes; five on the repairs; the decision-record axis alone on the second repairs; the transcript axis alone); repair commits not given; owner corrections not given.
- 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow: 2 rounds (ten axes; the transcript axis alone); 3 repair commits; owner corrections not given.
- 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow: 2 rounds (nine axes in parallel; the transcript axis alone and last); repair commits not given; owner corrections not given.
- 2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow: ten axes, two goals rounds, two decision-record re-reviews and the transcript review last, 5 or 6 rounds, since the file does not say whether the first goals round ran among the ten axes; repair commits not given; owner corrections not given.
- 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow: 4 rounds (an early review on four axes; ten axes before the merge; the decision-record axis alone on the repairs; the transcript axis), beside three rounds of the review of the whole goal set inside the work; one repair commit of about 40 heads and entries, with 8 decision-record findings folded into it; owner corrections not given.
- 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow: fourteen axes, the transcript axis among them, and two decision-record re-reviews of repairs, one of them after the transcript axis; the number of rounds not given; repair commits not given; owner corrections not given.
- 2026-11-11-knowledge-architect-issue-triage-klarch-workflow: 7 rounds (eleven axes at once; the goals axis; four rounds of the decision-record axis on the repairs; the transcript axis last); repair commits not given; owner corrections not given.
- 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow: 3 rounds (nine axes; a re-review; the transcript review); repair commits not given; owner corrections not given.
- 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow: 3 rounds (ten axes; the eight axes a head-changing repair owed; the transcript reviewer alone and last); repair commits not given; owner corrections not given.
- 2026-11-11-thaum-split-oversized-modules-klarch-workflow: 2 rounds (ten axes; the transcript axis last and alone); repair commits not given; owner corrections not given.
- 2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow: 3 rounds (the spec's review on seven axes; seven axes before the merge; the transcript axis last); repair commits not given; owner corrections not given.
- 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow: 5 rounds (twelve reviewers at once; three single decision-record reviews; the transcript review); 36 commits after the reviewed range, none folded, the file not saying how many repaired a finding; 0 owner corrections.
- 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow: "four rounds", listed as eight axes, two decision-record re-reviews, a transcript review, and a final round of the decision-record and agent-text axes, so 4 or 5 rounds; repair commits not given; owner corrections not given.
- 2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow: 4 rounds (changelog, standing state and self-consistency at once; the transcript reviewer; a reading of 48 commits; a reading for arguments from a future breaking cost); 3 repair commits; 0 owner corrections.
- 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow: 3 rounds (seven axes; three axes on the repairs that added a head; the transcript review); 2 repair commits, the second round's repairs folded into the first; 0 owner corrections.
- 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow: 6 rounds (ten axes; four decision-record rounds on the repairs, with 6, 3, 4 and 3 findings; the transcript axis last); repair commits not given; owner corrections not given.
- 2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow: 2 rounds (nine axes; the transcript axis alone); 1 repair commit; owner corrections not given.
- 2026-11-12-thaum-triggers-design-klarch-workflow: 2 rounds (nine axes in parallel; the transcript axis alone), beside four design-conformance readers dispatched at the grounding; repair commits not given; owner corrections not given.
- 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow: 5 rounds (the work's first review in two dispatches; eleven axes before the merge; the transcript axis; two axes on the commits of the owner's rulings taken after it); 8 repair commits; 0 owner corrections.
- 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow: 2 rounds (nine axes; the transcript axis alone), the goal reviewer's three draft-mode dispatches counted under goal-setting; 2 repair commits; 0 owner corrections.
- 2026-11-13-thaum-long-draw-klarch-workflow: 3 rounds (12 axes; a second decision-record review; the transcript review); 4 repair commits, and 1 history edit of messages; 0 owner corrections.
- 2026-11-13-thaum-pin-move-0-10-klarch-workflow: 3 rounds (ten axes; two axes again on the repairs; the transcript axis last), beside six rounds of the goal review inside the work; 12 repair commits, three message repairs folded; 0 owner corrections of drift, the owner correcting the session five times on its own drafts. The file's summary says twelve axes and its count line ten.
- 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow: 4 rounds (nine axes; a second pass of two axes; the transcript axis; the standing-state read of the milestone's deletion); 6 repair commits, beside three fixes of older defects; 0 owner corrections.
- 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow: 4 rounds (three axes on the work; seven axes before the merge; the decision-record review of its repair; the transcript review); 4 repair commits, plus two rewords of earlier messages; 0 owner corrections.
- 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow: 5 rounds (four reviewers on the work; nine before the merge; two decision-record reviews of repairs; the transcript review); 5 repair commits, beside two fixes of items met outside the task; 0 owner corrections, the owner ruling three questions.
- 2026-11-14-thaum-casting-process-design-klarch-workflow: 2 rounds (nine axes; the transcript axis alone and last); 3 repair commits, plus one amend of the plan commit's message folded into it and one commit recording the owner's answer to a corrected premise; 0 owner corrections.

### Why it matters

Until the counts are compared across branches, whether the workflow lowers the review rounds, the
repairs and the corrections that cost the efficiency `goal@agents-get-a-complete-workflow` is there
to raise is not known, and a proposal to add or cut workflow text, which `design@agent-skills@capabilities-not-structure`
weighs, is argued from intuition. The compared counts are also the evidence
`goal@the-workflow-improves-through-real-use` names: real sessions as the test
of the workflow. The count of drift corrections would also measure the supervision
`goal@agents-work-without-drift` saves.

### Trigger

The decision on a proposal to add or cut workflow text, or to change when a review runs, that
argues from review cost, where the decision turns on that cost; or the owner's request for the
comparison. The discussion or the intake that decides such a proposal runs the comparison over the
series above, and this entry closes with it. A proposal that arrives and is sent to another
discussion, or that loses on a fact no count bears on, is no occasion.
