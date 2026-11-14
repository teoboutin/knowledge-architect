# 2026-11-11-knowledge-architect-issue-triage

- **Received file**: 2026-11-11-knowledge-architect-issue-triage.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from source at main `v0.9.0-313-gc41d9e63` (`c41d9e63`), branch `issue-triage-quick-fixes`, tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.
  - `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
  - `design@agent-skills@fork-choice-in-weak-form`: constrains, respect.
  - `design@agent-skills@owner-rules-at-a-fork`: constrains, respect.
  - `design@agent-skills@derived-decision-proceeds-on-default`: constrains, respect.
  - `rejected@agent-skills@every-thread-closed-by-argument`: constrains, respect.
  - `goal@the-owner-decides`: constrains, respect.
  - `design@agent-skills@audit-repairs-the-whole-record`: constrains, respect.
  - `design@agent-skills@intake-outcome-authority`: constrains, respect.
  - `rejected@agent-skills@owner-rules-each-intake-finding`: constrains, respect.
  - `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`: touches, respect.
  - `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: conflicts, check-reversal.
  - `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: trigger-met, rule-before-work.
  - `design@agent-skills@title-states-the-rule`: constrains, respect.
  - `design@agent-skills@one-decision-per-head`: constrains, respect.
  - `design@agent-skills@existing-heads-on-touch`: conflicts, check-reversal.
  - `design@agent-skills@term-defined-at-first-load`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@one-session-cost-weighed-before-removal`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@one-repair-per-finding`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `design@agent-skills@scoped-auditor-on-touched-heads`: conflicts, check-reversal.
  - `design@agent-skills@losing-alternatives-filter`: constrains, respect.
  - `issue@check@core-heads-bundle-decisions-with-many-citing-sites`: touches, none.
  - `design@knowledge-architect@git-flow`: constrains, respect.
  - `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements`: trigger-met, rule-before-work.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.

## touched-head-owes-whole-audit: a head that a change touches owes its whole conformance, older debts included, at any edit

Members: 2026-11-11-knowledge-architect-issue-triage P4 (home, project file);
2026-11-11-knowledge-architect-issue-triage-klarch-workflow W1;
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow W2;
2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow W3.

- **a. What it says.**
  - Issue triage W1 (unclear, "wrong in effect"): the session edited one sentence of an old head
    that bundled about ten decisions; the decision-record reviewer asked for the whole head brought
    to the rules; "Each round found further statements, older than the branch"; four rounds of the
    decision-record axis followed the first review, stopped by putting the rest to the owner, "who
    ruled that an issue entry holds it". It blames
    `skill@knowledge-architect-decision-recording@head-shape`, "A head that a change touches is
    brought to the rules on design heads in that change: its title to the rule it argues, each
    decision it bundles to a head of its own, its ground to its argument." Proposal: scope the
    sentence "to the statements the change writes, moves or makes false. Each other statement of
    the head that fails the rules is opened as an issue entry naming the head, in the same change";
    net one instruction; nearest rival, a bound of one re-review round, which loses because "the
    rounds here were owed".
  - Issue triage P4 (interaction, project side): the head is the git flow head the root `CLAUDE.md`
    takes as its `## Git` home; four decisions were split out (one removed by the next round) and
    the owner ruled the rest waits for
    `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements`. Fix: the design-record
    audit of this repository, `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`.
  - Finding repairs W2 (cost, "rests on the owner's word"): removing one citation from a head that
    bundled five decisions with 26 citing sites owed its whole split; "One citation removed led to
    nine new heads, three review rounds on the decision-record axis, and the re-pointing of the 26
    sites". It blames `design@agent-skills@existing-heads-on-touch`, "Adding a reference, or
    rewording a sentence, is more than re-pointing, and so is removing one, on the owner's word."
    Proposal, for the owner: "a citation removed because its target left the record re-points
    nothing and is exempt, as re-pointing is"; rival, keep the rule and accept the cost.
  - Standing questions W3 (wrong, minor): adding a member to a head led the first decision-record
    review to retitle it, and the re-review asked for a third title; "Each title was a defensible
    reading of the body." One instance, no proposal.
- **b. Whether it still applies.** At the versions used (c41d9e63, `v0.9.0-313`; bd37bb85,
  `v0.9.0-387`; 5ce466db, `v0.9.0-384`), `path@agent-skills@fragments/head-shape.md` (`git show <rev>:crates/agent-skills/fragments/head-shape.md`), line 81,
  reads "**A head that a change touches is brought to the rules on design heads in that change**".
  At origin/main, `path@agent-skills@fragments/head-shape.md`, line 95: "**A head that a change
  touches is brought to the rules on design heads before the change lands**: its title to the rule
  it argues, each decision it bundles to a head of its own, its ground to its argument. In work a
  plan document plans, a head its harvest will touch [...] is brought to them by a step before the
  harvest [...]; a head the list missed, and a head any other change touches, in the change that
  touches it. Re-pointing a reference in a head, to a renamed entry or a migrated form, does not
  touch it; adding or removing one does." The change, by "[agent-skills] The heads a harvest
  touches, listed in the plan, judged by a scoped design-record auditor and conformed by a step
  before the harvest" (38a24ea4, `v0.10.0-37`, on main), moves planned work to a step before the
  harvest; for a change no plan document plans, which all four instances were, the rule is as
  reported. So it applies.
- **c. Whether it is already recorded.** The rule is `design@agent-skills@existing-heads-on-touch`
  (verdict `conflicts`, `check-reversal`, with an owner-question on the removal case). Its argument:
  "In a change no plan document plans, the rules apply at the moment a session already reads the
  head and changes what it says, so a head is repaired at the cost of one read." The instances show
  that premise false for a head that bundles decisions: the cost was a split, the re-pointing of
  every citing site (26 in finding repairs) and up to four re-review rounds. The removal clause is
  the owner's ruling, recorded in "[agent-skills] The intake's outcomes on the installed text: a
  spec's steps carry their claims, a review keeps its reports in files, a head is touched by an
  added or removed reference" (49341b93): "the owner ruled that re-pointing a reference [...] does
  not touch a head, and that adding or removing one does", with the owner's premise "that the
  exemption was meant for a renamed slug". Twice already the rule was escaped by an issue entry
  instead of the split: `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements`,
  on the owner's word, and `issue@check@core-heads-bundle-decisions-with-many-citing-sites` (verdict
  `touches`), whose heads "each listed one was left unsplit because a split must first read every
  site that cites the head, 6 to 39 sites per head". The review-cost series,
  `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`, holds an earlier instance: "a
  slice in thaum, on knowledge-architect 0.6.0: one finding of the decision-record axis cost three
  reviews of that axis, through `design@agent-skills@existing-heads-on-touch`". No issue entry holds
  the rule's reach as its subject: `git grep -l existing-heads-on-touch origin/main -- '*open-issues*'`
  returns two files, the review-cost series and the issue on the core heads. Constraining heads a fix must keep: `design@agent-skills@one-decision-per-head`
  and `design@agent-skills@title-states-the-rule` (the rules stay; only their moment changes);
  `design@agent-skills@audit-repairs-the-whole-record`, which already gives older debts to an audit;
  `design@agent-skills@term-defined-at-first-load` (the definition of a touch stays in one text);
  `design@agent-skills@scoped-auditor-on-touched-heads` and `design@agent-skills@pre-harvest-conformance`
  (verdict `conflicts` on the scope: a narrowed touch narrows the list those judge);
  `design@agent-skills@one-session-cost-weighed-before-removal`. The trigger of
  `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements` (verdict `trigger-met`)
  is "the next change that edits the head": the outcome here edits no head, so it is not met.
- **d. Its kind.** A defect of the text, unclear in its reach and wrong in effect: the rule cannot
  tell an edit of one statement from a rewrite, nor a debt the change created from one older than
  it, and the premise of its head, a repair at "the cost of one read", fails on a bundled head (three
  sessions, plus the 0.6.0 instance and two issues escaping it). Standing questions W3 is an
  observation within it: two defensible titles, one extra round.
- **e. Its scope.** `goal@design-is-recorded-with-its-arguments`, which the rule serves, against
  `goal@agents-get-a-complete-workflow`, whose efficiency the cascade costs. The owner of the text:
  the installed text, the fragment `path@agent-skills@fragments/head-shape.md` and its head
  `design@agent-skills@existing-heads-on-touch`. Either proposal reverses part of that head: issue
  triage W1 its scope in a change no plan plans; finding repairs W2 the owner's clause on a removed
  reference.
- **f. A better fix.** Rivals: (1) keep the rule and accept the cost; (2) issue triage W1, scope the
  touch to the statements the change writes, moves or makes false, other failures opened as an
  issue entry naming the head; (3) finding repairs W2, exempt a citation removed because its target
  left the record; (4) a bound on re-review rounds, which loses, since every round found real
  defects (issue triage W1, transcript reader W1). Rival (2) bounds the cascade at its start for
  every kind of edit, removal included, and keeps re-review for what the branch wrote, which is
  where the re-review rounds of `head-repair-rereview-unbounded` found most defects; it subsumes
  (3). It is also what practice did twice, by an issue entry. Its cost: older debts wait in issue
  entries for the design-record audit, which `design@agent-skills@audit-repairs-the-whole-record`
  already makes the record's whole repair. Choosing between (1), (2) and (3) reverses part of a
  head and narrows an owner's ruling (the removal clause), and the scoped auditor and the
  pre-harvest step read the same definition, so it needs a design discussion: not settled here.
- **g. Its route.** A decision whose design is not settled: an issue, for a discussion under
  `skill@knowledge-architect-design`. The owner-question of the verdict, whether a removal forced
  by the target's leaving counts as a touch, is the owner's (a narrowing of the owner's own ruling,
  per `primer@design-heads`, "The weak form"), and is put to the owner in that discussion; no part
  of this intake's work rests on it.
- **h. The proposal.** Open an issue in `agent-skills`, kind `design`, slug
  `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`. What: the rule's text at
  `path@agent-skills@fragments/head-shape.md`; the instances (issue triage, four rounds and the git
  flow head; finding repairs, one removed citation, nine heads, three rounds, 26 sites; the
  transcript reader's round asking for a bundle older than the branch; standing questions, a title
  asked twice; the 0.6.0 thaum slice from the review-cost series); the two issues that already hold
  touched heads left unsplit. Suspected mechanism: the rule assumed a touched head is one the
  change rewrites, and it reaches every head a change edits at all, older debts included. Shapes,
  each with the head it changes: (1), (2), (3) above, each against
  `design@agent-skills@existing-heads-on-touch`, (2) also against the touched-head lists of
  `design@agent-skills@scoped-auditor-on-touched-heads` and `design@agent-skills@pre-harvest-conformance`.
  Re-entry point: the next change whose touched head owes the split of a decision older than the
  branch, or the owner's word, for a discussion under `skill@knowledge-architect-design`. Why it
  matters: the cascade costs re-review rounds for debts no branch wrote, against
  `goal@agents-get-a-complete-workflow`, and twice the rule was set aside by an issue entry rather
  than followed, so the text no longer states the practice. The proposals of issue triage W1 and
  finding repairs W2 argue from review rounds, so they meet the trigger of
  `issue@agent-skills@the-review-cost-counts-are-not-compared-yet` (section
  `review-cost-trigger-met`): the issue's text says that the discussion deciding them runs that
  entry's comparison. Held by the session.

Default: open the issue above; no edit of the installed text in this intake.
Outcome: decided by the session: open an issue (`agent-skills`, `design`,
`issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`). Ground: the premise of
`design@agent-skills@existing-heads-on-touch`, a repair "at the cost of one read", fails on a
bundled head in three sessions; each fix reverses part of that head, one narrows an owner's
ruling, and the definition feeds `design@agent-skills@scoped-auditor-on-touched-heads`, so the
design is not settled and needs a discussion of its own.

## code-commit-with-partial-gates: a commit of Rust code made with part of the gates run

Members: 2026-11-11-knowledge-architect-issue-triage P2, and the standing answer of
2026-11-11-knowledge-architect-issue-triage-klarch-workflow on a project skill's addition missed.

- **a. What it says.** "The session ran only the tests it had written before one commit of the
  checker. A renamed variable shadowed another, and `commits` stopped reporting dangling
  references in messages. Four tests of the suite would have failed. The next commit's full gate
  run caught it, and the fix was folded into the commit." The file calls it a lapse, "Recorded as
  evidence only. The instruction is clear, and the repository's development skill restates it."
  No fix proposed.
- **b. Whether it still applies.** The instructions are unchanged from v0.9.0-313 to origin/main:
  `path@knowledge-architect@CLAUDE.md`, `instructions@verify-mechanically`, line 136, "Any change
  to the Rust source owes fmt, clippy and the test suite"; and
  `path@agent-skills@content/skills/development/SKILL.md`, its section "The gate", line 206, "Run
  the project's one gates command, which the setup skill recommends, before each commit that could
  fail it and before the review".
- **c. Whether it is already recorded.** No entry records it. The verdicts on
  `design@knowledge-architect@workflow-findings-stay-here`,
  `design@knowledge-architect@committed-findings-analysis`, `goal@only-needed-instructions` and
  `goal@the-workflow-leads-to-better-code` are `respect`.
- **d. Its kind.** A lapse of a clear instruction, once, caught by the next commit's gate run;
  the fix was folded into its commit.
- **e. Its scope.** `goal@the-workflow-leads-to-better-code`. Nothing is reversed.
- **f. A better fix.** Rival: none; a lapse is not repaired by adding text, and one instance shows
  no point of delivery to move.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, a lapse caught by the next gate run. Ground: the lapse
rule of `skill@knowledge-architect-retrospective-intake@admission-rules`.

## fork-classed-settled-near-miss: a fork closed "on the record", ruled by the owner before the merge

Members: 2026-11-11-knowledge-architect-issue-triage P3.

- **a. What it says.** "Evidence for a tripwire's re-entry, no finding. The session closed an
  issue entry that asked the owner to set the scope of the root's grounding row. It closed the entry
  on the record, "The record settles it, so the session decided", and reported this to the owner
  under "Decided". The standing-state reviewer named the tripwire that watches this shape. The
  transcript reviewer classed it as a near miss. The owner then ruled on it as Q15, keeping the
  session's default, before the merge." The file names no tripwire by slug and proposes no fix.
- **c. Whether it is already recorded.** The tripwire is
  `open-fork-classed-as-settled`, guarding
  `design@agent-skills@derived-decision-proceeds-on-default`, read whole. Its firing clause: "a
  decision found on the main branch was a choice between shapes that all met every binding
  constraint, which no goal ranked, and the branch that merged it reported it under "Decided" as
  settled by the record, without putting it to the owner, once on the main branch". This instance
  was put to the owner before the merge, so the clause is not met: the tripwire has not fired. Its
  horizon, "three commits on the main branch that add analyses of received retrospective files",
  is judged in the section `tripwire-open-fork-classed-as-settled` of this intake's standing
  answers, which another group drafts. The record search graded the tripwire `unrelated`, with the
  same reading: "both instances in the batch (issue-triage P3, closed "on the record" then ruled as
  Q15; quick-fixes' roadmap order) were put to the owner before the merge" (searcher 8's report),
  so no verdict file holds it. The verdicts on `design@agent-skills@questions-go-to-their-authority`,
  `design@agent-skills@fork-choice-in-weak-form`, `design@agent-skills@owner-rules-at-a-fork`,
  `design@agent-skills@derived-decision-proceeds-on-default`,
  `rejected@agent-skills@every-thread-closed-by-argument` and `goal@the-owner-decides` are
  `respect`. By `design@agent-skills@fork-choice-in-weak-form`, "the session's where the owner
  approved the session's default": the owner's Q15 keeps the default, so the choice lands as the
  session's, which the owner approved.
- **d. Its kind.** An observation: a near miss of a tripwire's clause, caught by two reviewers and
  ruled by the owner before the merge. No text is blamed.
- **e. Its scope.** `goal@the-owner-decides`. Nothing is reversed.
- **g. Its route.** Nothing; the tripwire's own section carries what its horizon owes.
- **h. The proposal.** No change. The near miss is named in the tripwire's section as evidence the
  watch saw, which does not fire it.

Default: no change.
Outcome: decided by the session: no change, an observation; the tripwire did not fire, since the
decision was put to the owner before the merge. Ground: the firing clause of
`open-fork-classed-as-settled`, which requires the decision found on the main
branch with no exchange.

## tripwire-open-fork-classed-as-settled: its horizon passes at this intake's commit, unfired, with three near misses caught before the merge

Members: `open-fork-classed-as-settled` (verdict `unrelated`, searcher 8);
the near miss of the subject `fork-classed-settled-near-miss` (P3 of
2026-11-11-knowledge-architect-issue-triage), and the roadmap order of the standing answer of
2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  files, counted from the commit that writes this line". `git blame` of
  crates/agent-skills/docs/tripwires.md, lines 304 and 305, gives 3fea6b60, "[review] Repairs of
  the review before the merge: the weak form gets a head of its own, the owner's hedges are
  restored, and the owner's wish on built defaults stays open", of 2026-10-29.
  `git log --diff-filter=A --format=%h 3fea6b60..origin/main -- docs/retrospective-reports/` prints
  e8562915 and a9e3b3c9. This intake's commit is the third occasion.
- **Whether it fired.** The clause needs "a decision found on the main branch" that was a fork no
  goal ranked, reported under "Decided" as settled by the record "without putting it to the owner".
  Two instances of the batch were put to the owner before the merge: the quick-fixes session
  ordered four roadmap rows itself, and "Three reviewers reported it, and the owner ruled before the
  merge" (its standing answer, lines 70 to 73); the issue-triage session closed a fork "on the
  record" that the owner then ruled, which its project file records as a near miss, analysed under
  `fork-classed-settled-near-miss`. A third, in 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow,
  lines 9 and 10: "The owner ruled two forks during the discussion and one more, raised by the
  transcript review, before the merge." None reached the main branch as settled. A decision on the main branch is the clause's
  evidence, which no received file need carry: over the watch, "unfired" rests on the chain of
  standing-state reviews of the merges since the line was written, the reading of the subject
  `unfired-judged-over-branch-only`. Unfired.
- **What leaving costs.** `git grep` finds no citation of the slug outside the tripwires home. The
  three near misses are evidence the reviews caught the act before the merge, which is where the
  tripwire's premise says the owner meets it, per `design@agent-skills@default-ruled-before-merge`.

Default: the tripwire is deleted at the commit that adds this intake's analyses, naming a9e3b3c9,
e8562915 and that commit as the occasions held through.
Outcome: decided by the session: delete it, unfired at its horizon. Ground: the horizon's third
occasion is this intake's commit; the three instances of the batch met the owner before the merge, so
the clause's "found on the main branch" is not met;
`skill@knowledge-architect-issue-tracking@movement-instruction` deletes a tripwire whose horizon
passes unfired. Where `fork-classed-settled-near-miss` decides otherwise on the same evidence, that
subject's outcome prevails here and this section is rewritten to point to it.

## push-chained-on-failing-check: a push that ran after a failing `commits` check

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow.md`, in its section on `push-chained-on-failing-check`, which gives the outcome.
