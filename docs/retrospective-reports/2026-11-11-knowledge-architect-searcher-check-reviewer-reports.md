# 2026-11-11-knowledge-architect-searcher-check-reviewer-reports

- **Received file**: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.9.0-354-gb97596f1`, `b97596f1`, when the work began, on the branch `agents-write-their-reports`, the tree clean."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@xtask@gates-list-primary-home`: constrains, respect.
  - `design@gates@the-library-owns-the-flags`: constrains, respect.
  - `design@gates@ci-flag-for-ci-only-requirements`: constrains, respect.
  - `design@knowledge-architect@gates-crate`: constrains, respect.
  - `design@knowledge-architect@xtask-crate`: conflicts, check-reversal.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@agent-skills@review-cost-counts`: constrains, respect.
  - `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: trigger-met, rule-before-work.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `design@agent-skills@audit-axis-runs-a-workflow`: constrains, respect.
  - `design@agent-skills@tool-report-moment`: constrains, respect.
  - `rejected@agent-skills@retrospective-finding-straight-to-issue`: constrains, respect.
  - `design@agent-skills@transcript-review-last-before-merge`: constrains, respect.
  - `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`: touches, respect.
  - `issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness`: touches, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `goal@session-orchestrates-the-work`: constrains, respect.

## review-in-workflow-loses-transcript: a review run by a workflow leaves no report in the transcript

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow W4, and the project's side of the same interaction in 2026-11-11-knowledge-architect-searcher-check-reviewer-reports (its section "The project's side of an interaction").

- **a. What it says.** Both review rounds of the session ran as one saved workflow each. The script returned one line per axis, so no reviewer's report became a message of the session's transcript. The transcript reviewer found the reports only as tool results, read with `--tools`. The file blames `skill@knowledge-architect-review@review-invariants`, "and to return the same report whole as text ... the text returned keeps the findings in the transcript the transcript reviewer reads", and calls it missing: the invariant assumes that the session is the dispatcher. It proposes no edit. The session opened an issue, and the project file names it.
- **b. Whether it still applies.** Yes. The sentence is unchanged at origin/main, `path@agent-skills@content/skills/review/SKILL.md`, at the bullet that opens "Every brief names a path for the reviewer's report" (line 113): "and to return the same report whole as text. ... the text returned keeps the findings in the transcript the transcript reviewer reads." The installed review skill says nothing of a review that a saved workflow dispatches (`grep -c -i workflow .claude/skills/knowledge-architect-review/SKILL.md` prints 0).
- **c. Whether it is already recorded.** Yes: `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`, kind `todo`. Its "What" section describes this session's instance: "the review before the merge of the branch that gave reviewers their own report files ran its ten axes, then the eight a repair owed, each as one workflow whose script returned `{axis, returned: 'yes'}` per axis". It was opened in the reported session itself, in the commit on main "[review] The transcript review's repairs: the owner's reason for the shell write, `--ci`'s reach, and two issues opened" (c9cf649c, `git log -- crates/agent-skills/docs/open-issues/a-review-run-by-a-workflow-keeps-no-report-in-the-transcript.md`). Its "What would close it" names the two shapes the file names, chosen under `skill@knowledge-architect-design`. The verdict of `issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness` (touches, respect) bears on the repair, not on this outcome: the repair decides how the installed review skill meets a user-level configuration that runs reviews as workflows, which that issue says no head argues. The other verdicts are `constrains, respect` on heads and goals the repair must keep within (`design@agent-skills@transcript-review-last-before-merge`, `goal@agents-get-a-complete-workflow`, `goal@session-orchestrates-the-work`); none is met by a no-change outcome.

Default: no change: already recorded.
Outcome: decided by the session: no change, already recorded in `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`, opened by the reported session on main; its repair is a design choice that issue gives to `skill@knowledge-architect-design`. The issue gains the instance of `reviewer-returns-whole-report` (see that section), which proposes the same closing shape. Ground: the brief's rule that a subject item c shows recorded takes only the bullets that establish it; the issue's text, quoted above.

## ci-flag-home-reads-as-xtask: the xtask heads read as owning the gates' flags, which the gates library owns

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports P1.

- **a. What it says.** The first build of a parse test read the `CI` variable; the record search returned the xtask head as a conflict, so the session moved the switch to a flag of xtask; the first review found that this contradicted `design@gates@the-library-owns-the-flags`, and the flag moved to the gates library. It blames `design@xtask@gates-list-primary-home`: "Where CI and a local run differ in what they run, the difference is a flag.", which reads as owning the flags it lists. Its proposal: the paragraph that lists the flags names them the library's, as its pointer to `design@gates@ci-flag-for-ci-only-requirements` already does for `--ci`; a rewording.
- **b. Whether it still applies.** At the version, `b97596f1`, `git show b97596f1:tools/xtask/docs/design.md` holds the sentence at line 61. On origin/main, `register@xtask@design` line 64 holds it unchanged: "**Where CI and a local run differ in what they run, the difference is a flag.**" The paragraph then gives each flag's reason, `--locked`, `--fail-fast`, `--require-rebased`, `--full` and `--ci`, and only `--ci` points to a gates head. A second sentence reads the same way, in `design@knowledge-architect@xtask-crate`, `register@knowledge-architect@design` line 432: "`gates`, whose list and flags are this repository's while the published library runs them". It applies, at two sites.
- **c. Whether it is already recorded.** No issue holds it. The search: `design@gates@the-library-owns-the-flags` (`constrains`, respect): "The flags of a gates run are the library's `GatesArgs`, which derives clap's `Args`"; `design@gates@ci-flag-for-ci-only-requirements` and `design@knowledge-architect@gates-crate` (`constrains`, respect), the latter putting "the flags themselves" in the library's interface; `design@xtask@gates-list-primary-home` (`constrains`, respect): the rewording keeps the rule and its one gate list; `design@knowledge-architect@xtask-crate` (`conflicts`, check-reversal, agent-14): its sentence says the gates' flags "are this repository's", the same reading, so the repair narrows it too. I read the four heads with `cargo klarch show`: the xtask heads are about which flags CI passes and why, and the gates head owns the flags' definition; no reversal is needed, only words that name the owner.
- **d. Its kind.** A defect of the text, unclear: two sentences of design homes read as giving this repository the flags that `design@gates@the-library-owns-the-flags` gives the library. It produced a wrong first build and a review repair. Provable by reading.
- **e. Its scope.** `goal@design-is-recorded-with-its-arguments`, through the heads' accuracy. The owners: the xtask Component's design home, `register@xtask@design`, and the root's, `register@knowledge-architect@design`. It reverses nothing: the rewording states the division `design@gates@the-library-owns-the-flags` already decides, a rewording that adds or removes no decision, per `primer@design-heads`.
- **f. A better fix.** The report's fix reaches one of the two sentences; the nearest rival is that fix alone. The fact that decides: `design@knowledge-architect@xtask-crate` carries the same reading in its own words, so a repair of one leaves the other to mislead the next build; one repair covers both. No change is a weaker rival still: the wrong build cost a review repair.
- **g. Its route.** A text edit of two design homes, under `skill@knowledge-architect-decision-recording`, since it writes into a design home.
- **h. The proposal.** In `register@xtask@design`, head the section "gates-list-primary-home", replace the bold sentence by the first block. In `register@knowledge-architect@design`, head the section "xtask-crate", replace "`gates`, whose list and flags are this repository's while the published library runs them" by the second block. Instructions: 0 added, 0 removed; design homes, two rewordings. Nearest rival: the xtask head alone, defeated in item f.

```text
**Where CI and a local run differ in what they run, the difference is a flag of the gates library,
per `design@gates@the-library-owns-the-flags`, which CI passes and a local run does not.**
```

```text
`gates`, whose list, and the flags CI passes, are this repository's, while the published library
defines the flags and runs the gates
```

Default: handle now, the two rewordings above.
Outcome: decided by the session: handle now. Ground: both sentences read as giving this repository flags that `design@gates@the-library-owns-the-flags` gives the library, and one of them sent a build to the wrong Component; the rewording states that head's division and changes no decision, per `primer@design-heads`.

## procedure-token-cost-measured: the token cost of a record search and of review rounds on one branch

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports P2.

- **a. What it says.** An observation, with no proposal: from the harness's usage notices, each record search took about 2.2 million subagent tokens, the first review round about 1.2 million, the second about 0.8 million, the transcript reviewer about 0.3 million, on a code change of a few hundred lines. It names "the owner's issue on the retrospective's counting no review cost" as the home for such figures.
- **b. Whether it still applies.** An observation of one session; nothing to repair.
- **c. Whether it is already recorded.** The home it names is `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`, under `design@agent-skills@review-cost-counts`. That head counts rounds, not tokens: "A round is counted, not a reviewer, since the series carried from before the question counts rounds, and a count in another unit would not compare with it" (`path@agent-skills@docs/design/retrospective.md`, line 129). The search graded the issue `trigger-met` / `rule-before-work` (agent-6), on its trigger "A proposal to add or cut workflow text, or to change when a review runs, that argues from review cost". This subject proposes nothing, so it does not meet that trigger; the verdict's reason names the re-review loops and the touched-head cascade, which other subjects of this intake analyse, and the section that rules the trigger belongs with them. `goal@generic-tools-and-procedures-are-shipped` (`constrains`, with an owner-question on tools set up generically) bears on no part of this subject.
- **d. Its kind.** An observation.
- **e. Its scope.** `goal@agents-get-a-complete-workflow`, whose efficiency the counts measure. No owner of a fix.
- **f. A better fix.** The rival is adding the token figures to the issue's series. The fact that decides: the head fixes the series' unit as rounds, and a figure in tokens would not compare with it; the branch's rounds reach the series through the standing answers of its workflow file, which `skill@klarch-retrospective-intake@received-analysis` adds, not through this file.
- **g. Its route.** Nothing.
- **h. The proposal.** No change. The figures stay in the received file and in this analysis's history.

Default: no change.
Outcome: decided by the session: no change, an observation. Ground: it proposes nothing; its figures are in a unit `design@agent-skills@review-cost-counts` says would not compare with the series; it does not meet the trigger of `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`, which argues from no proposal here.
