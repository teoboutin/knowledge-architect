# 2026-11-14-knowledge-architect-component-arguments-slice

- **Received file**: 2026-11-14-knowledge-architect-component-arguments-slice.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from this tree's source: the main commit the session started from, `cac87723` (`v0.10.0-43-gcac87723`); the branch `component-arguments`; the tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@query@occasion-shaped-output`: conflicts, check-reversal.
  - `design@agent-skills@acceptance-criteria-in-the-document`: constrains, respect.
  - `design@agent-skills@criteria-test-silent-premises`: conflicts, check-reversal.
  - `design@agent-skills@design-audit-checks-fit`: conflicts, check-reversal.
  - `design@knowledge-architect@query-crate`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@agent-skills@one-home-for-head-rules`: conflicts, check-reversal.
  - `design@agent-skills@restatement-size-test`: constrains, respect.
  - `rejected@agent-skills@head-rules-in-decision-recording`: constrains, respect.
  - `rejected@agent-skills@head-rules-one-fragment`: constrains, respect.
  - `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`: touches, respect.
  - `design@agent-skills@shared-text-fragment`: constrains, respect.
  - `design@agent-skills@design-home-write-loads-recording`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `rejected@agent-skills@head-rules-whole-in-primer`: constrains, respect.
  - `design@agent-skills@a-head-is-owed-by-an-entry-test`: constrains, respect.
  - `design@agent-skills@component-head-reasons-over-each-criterion`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.

## criterion-evidence-in-no-record: a criterion admitted without condition 2 has its evidence in no record at the leaving

Members: 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow W3, C1; the
project interaction note of 2026-11-14-knowledge-architect-component-arguments-slice (W3).

- **a. What it says.** `skill@knowledge-architect-planning@acceptance-criteria`: "An acceptance
  criterion is admitted only where it meets conditions 1, 3 and 4 of the standard below". At the
  plan's leaving, a criterion that recurs is judged by all four. The milestone's one criterion
  fired on the `--work` files of a design audit's search, and no record the workflow requires names
  them: the audit's record is the fit lines, which name the plan document and not its work. The
  audits named their `--work` in their commit messages by the session's practice alone. At the
  leaving the criterion failed condition 2 and was dropped. C1: `worklist --verify` writes no `work`
  line of `work.txt` into `fit-lines.md`. Proposals: C1's, the fit lines name the work files
  (preferred); or the design audit names its `--work` in its commit message (+1 instruction).
- **b. Whether it still applies.** Yes. On main, `path@agent-skills@content/skills/planning/SKILL.md`
  near line 504 still admits a criterion on conditions 1, 3 and 4. The leaving judged the criterion,
  AC1 of the milestone on split guidelines and plan directories, guarding
  `design@agent-skills@discussion-record-apart`: 7e180a76 kept it as a tripwire on a doubt, and
  89c2d927 dropped it, "No record a design audit owes names its `--work`, so condition 2 [...] fails
  outright", citing `path@query@src/worklist.rs`, which skips every `work` line when it writes the
  fit lines (`git show 89c2d927`). Point 2 of working a slice fixes the audit's `--work`, "the
  milestone's `README.md` and the slice's `README.md`, and no other file", so a correct audit's work
  follows from the instruction, but a wrong one is in no record.
- **c. Whether it is already recorded.** No issue. Heads met:
  `design@agent-skills@criteria-test-silent-premises` (`conflicts`, `check-reversal`) rests on
  "Condition 2 does not apply: the landing's own work reports on a criterion", the premise this
  instance strains; `design@agent-skills@design-audit-checks-fit` (`conflicts`, `check-reversal`)
  and `design@query@occasion-shaped-output` (`conflicts`, `check-reversal`) fix what `fit-lines.md`
  holds, which C1 would widen; `design@agent-skills@acceptance-criteria-in-the-document`
  (`respect`) keeps the judgement at the leaving. `goal@tasks-a-tool-does-better-are-the-tool-s`
  carries an owner-question for a new tool; C1 is a change of an existing command's output, which
  the discussion below weighs first.
- **d. Its kind.** A defect of the text, missing: a criterion admitted at planning can be
  unjudgeable at the leaving, since its premise that the work reports on it does not hold for an
  observable no record names. One instance.
- **e. Its scope.** The installed text and the `query` Component. Each fix reverses or widens a
  head: C1 widens `design@agent-skills@design-audit-checks-fit` and
  `design@query@occasion-shaped-output`; W3's +1 changes what an audit's commit holds; a fix of the
  admission rule reverses the premise of `design@agent-skills@criteria-test-silent-premises`.
- **f. A better fix.** Rivals: C1; W3's commit-message line; condition 2 applied at admission to a
  criterion whose observable no record holds; no change, since the leaving's judgement dropped the
  criterion as the standard says. Nothing in the record ranks them: the first two keep the guard of
  `design@agent-skills@discussion-record-apart`, the third and the fourth leave that head with no
  guard of a wrong `--work`. That is a design discussion.
- **g. Its route.** An issue, for a design discussion on its own branch.
- **h. The proposal.** Open an issue in agent-skills, kind `design`, slug
  `a-criterion-admitted-without-condition-two-can-have-no-record-at-the-leaving`. Instance: AC1 of
  the milestone on split guidelines and plan directories, kept by 7e180a76 and dropped by 89c2d927.
  Mechanism: a criterion is admitted on the premise that the landing's work reports on it, and the
  work's records hold no observable for it. Re-entry point: the next acceptance criterion whose
  observable is an input of a command that no committed record names, at its admission. It names
  the four heads above, and the four rivals.

Default: open the issue above.
Outcome: decided by the session: open an issue, agent-skills, kind `design`,
`a-criterion-admitted-without-condition-two-can-have-no-record-at-the-leaving`. Ground: every fix
widens or reverses a head (`design@agent-skills@criteria-test-silent-premises`,
`design@agent-skills@design-audit-checks-fit`, `design@query@occasion-shaped-output`), and no
argument of the record ranks the rivals.

## head-rule-home-reads-two-ways: the decision on one home for head rules reads two ways on who points and who hosts

Members: 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow W4;
2026-11-14-knowledge-architect-component-arguments-slice P1 (interaction twins). P2 of the same
project file is analysed under `plan-deferred-delivery-decision`.

- **a. What it says.** The session built a new rule on one class of heads with one skill stating it
  and the decision-record reviewer and the design-record auditor pointing to it. The review before
  the merge read `design@agent-skills@one-home-for-head-rules` as giving such a rule one home, "a
  fragment hosted by every text whose act applies it", and the rule moved into
  `path@agent-skills@fragments/head-shape.md`. A record searcher had graded that head `constrains`
  on its body, "Every other text points to the rules", reading the two judges as other texts. W4
  blames the installed decision-recording skill, which does not say that a new rule on heads joins
  the fragments, and proposes one sentence there. P1 blames the head's body, and proposes that it
  state that a rule on heads joins the fragments and that "every other text" is a text that neither
  writes nor judges a head (a rewording, no instruction added).
- **b. Whether it still applies.** Yes. On main the head, in
  `path@agent-skills@docs/design/project-configuration.md` near line 393, still reads: "The
  decision-recording skill keeps the procedures of writing a head. Every other text points to the
  rules, or restates one sentence of them with its pointer, per
  `design@agent-skills@restatement-size-test`." The only commit to that file since cac87723 is
  42dd2be8, which did not touch this head (`git log cac87723..origin/main` on the file).
- **c. Whether it is already recorded.** No issue. `design@agent-skills@one-home-for-head-rules`
  is `conflicts`, `check-reversal`, on P1's rewording. The reversal check: the title states "a
  fragment hosted by every text whose act applies it", and the body names "the texts that judge a
  head" among the hosts of the shape rules; P1 states in the body what the title already says, so
  it reverses nothing. Bounds: `rejected@agent-skills@head-rules-one-fragment` (a rule joins the
  fragment its judges host, never one primer-hosted fragment for all) and
  `rejected@agent-skills@head-rules-in-decision-recording` (no move of the rules into that skill),
  both kept by P1. `design@agent-skills@component-head-reasons-over-each-criterion` records that the
  rule "is one of the shape rules on a head, delivered by" `path@agent-skills@fragments/head-shape.md`,
  hosted whole by the decision-recording skill, the decision-record reviewer and the design-record
  auditor. The issue
  `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`
  (`touches`) is a class this instance belongs to; it needs no edit, since its own search is what
  it asks.
- **d. Its kind.** A defect of the text, unclear: the head's body admits a reading its title does
  not. W4's installed side is a defect only for a maintainer of the installed text: no project that
  installs the workflow writes a rule on heads into the fragments, which this repository alone
  edits.
- **e. Its scope.** The agent-skills design home. W4's sentence, in installed text, would be read by
  maintainers alone, which the need test sends to a `%%` comment at most; the fragments' own first
  lines already are such comments, "%% The split of the rules on heads by act:
  `design@agent-skills@one-home-for-head-rules`." in each of the three.
- **f. A better fix.** One fix for both sides: P1's rewording of the head, which is what a
  maintainer's record search returns and what each fragment's comment points to. Nearest rival:
  W4's installed sentence; the fact that decides is that its only reader is the maintainer, whom
  the head already reaches. A second rival: leave the reading to each search,
  `rejected@agent-skills@two-readings-left-to-judgement`, which this instance shows wrong.
- **g. Its route.** A rewording of a design head, within its title's rule, under
  `skill@knowledge-architect-decision-recording`; no decision.
- **h. The proposal.** Handle now. In `design@agent-skills@one-home-for-head-rules`, replace the
  sentence "Every other text points to the rules, or restates one sentence of them with its
  pointer, per `design@agent-skills@restatement-size-test`." by the text below. No change to W4's
  installed skill. Instructions added and removed: 0 (a design home, and a rewording).

```text
A text whose act applies a rule on design heads, a text that writes or judges a head among them,
hosts the fragment that holds the rule rather than pointing to it, and a new rule on heads joins a
fragment whose every host applies it whole, an existing one or a new one. Every other text points
to the rules, or restates one sentence of them with its pointer, per
`design@agent-skills@restatement-size-test`.
```

Default: handle now, the rewording above.
Outcome: decided by the session: handle now, the rewording above, and no change to the installed
decision-recording skill. Ground: the head's title already gives every text whose act applies a
rule a host's place; the body is brought to it, which reverses nothing; W4's installed sentence
would have maintainers alone as readers.

## plan-deferred-delivery-decision: the plan left its one load-bearing delivery decision to the harvest

Members: 2026-11-14-knowledge-architect-component-arguments-slice P2.

- **a. What it says.** The slice's spec held "at two sites, so the instruction earns a head" in its
  decided design, and the plan review's lead "the reviewers judge the rule through the text that
  states it, a pointer, not a site; if step 3 ... alone states it, test 2 fails" in its harvest row,
  and handed the choice to the harvest. The session's first outcome, no head, was reversed at the
  review. The file proposes no change: "the plan review's lead was right, and the decision was
  missing from the record the plan was read against, which P1 repairs."
- **d. Its kind.** An observation of the session's work, with its cause in P1. The plan review
  raised the lead and the review before the merge corrected the outcome, so no instruction failed
  on its own.
- **h. The proposal.** No change. The cause is the reading of
  `design@agent-skills@one-home-for-head-rules` that `head-rule-home-reads-two-ways` repairs; whether
  the rule earns a head then follows from `design@agent-skills@a-head-is-owed-by-an-entry-test`
  (`constrains`, `respect`), "each text that states the instruction is one site", once the delivery
  is decided.

Default: no change.
Outcome: decided by the session: no change, an observation whose cause is repaired under
`head-rule-home-reads-two-ways`. Ground: the plan review and the review before the merge each
worked as their texts say; the file proposes nothing.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.
