# 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow

- **Received file**: 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-21-ga9ed6e3f` when the work began, branch `planned-splits`, no uncommitted changes. The harness ran Claude Code 2.1.321, as the session's transcript records it."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@agent-skills@heads-argued-from-another-harness`: touches, respect.
  - `design@agent-skills@shared-text-fragment`: constrains, respect.
  - `design@agent-skills@no-thread-closes-before-the-search`: constrains, respect.
  - `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`: touches, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@agent-skills@admission-rules-home`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@design-with-agents-as-counterparts`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@agent-skills@development-skill`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@agent-skills@version-line-names-the-main-commit`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.

## harvest-closes-thread-without-search: a harvest re-argued a `rule-before-work` entry and closed the thread with no search

Members: 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow W4.

- **a. What it says.** A design audit graded an issue `trigger-met`, `rule-before-work`: its
  re-entry was met by the slice's change to one head. The harvest agent re-argued that head's rival
  on the in-change path and closed the thread `argued` with "No search of its own", on the ground
  that the audit's search had graded the issue. One premise also misread the harness's hook
  documentation. The review before the merge caught both; the re-argument was withdrawn, and the
  issue's re-entry rewritten so that a change of the head's host list no longer meets it. The file
  adds that the planning skill's sort of a `rule-before-work` verdict at a design audit says
  neither which reading applies, arguing the instance in the slice or ruling its re-entry, nor that
  arguing it opens a thread that owes its own search. Proposal: the sort states that arguing the
  entry in the slice is a design thread that owes its own search (+1).
- **b. Whether it still applies.** In part. The search rule exists and is clear, on main and at the
  file's version: `path@agent-skills@content/skills/design/SKILL.md` near line 493, "**Once the
  shapes settle, and before any thread closes, a search of" the whole record runs; and
  `design@agent-skills@record-searched-before-the-work` has the search run "again for a thread that
  opens on a subject the first work did not cover". The planning skill on main names neither
  `rule-before-work` nor `trigger-met` (a grep of `path@agent-skills@content/skills/planning/SKILL.md`
  returns none); its sorts at the audit (point 2) sort gaps by shape, not by verdict action.
- **c. Whether it is already recorded.** Yes, in two entries. `design@agent-skills@no-thread-closes-before-the-search`
  (`constrains`): "The installed design skill closes no thread, in any closed state and by the
  owner's word or the session's, before the search of loop step 1 has run". And
  `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`
  (`design`, `touches`), whose "What would close it" includes "the search's action on a met trigger
  changed from a ruling before the work to a report": this instance is a `trigger-met` verdict at
  an audit pulling a design into a slice, its kind of instance. Its re-entry point is the owner's
  word, so nothing here reopens it. `issue@agent-skills@heads-argued-from-another-harness`
  (`touches`) had its re-entry rewritten on the branch, which the file reports.
- **d. Its kind.** Two parts. The closure with no search is a lapse of a subagent: the instruction
  exists, is clear, and was not followed, once; no text repairs it. The silence of the planning sort
  on a `rule-before-work` verdict at an audit is the open question of the recorded design issue.
- **f. A better fix.** The report's +1 restates the design skill's search rule in planning: one
  lapse is no recurrence at one point of delivery, so the lapse rule adds no text. The sort's
  question, which act a met trigger asks at an audit, is the recorded issue's; adding the instance
  there is the nearest rival to opening a new entry, and wins since its closing condition already
  names that change of action.
- **g. Its route.** An instance added to a recorded issue; nothing for the lapse.
- **h. The proposal.** Add the instance below to
  `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`, in its
  "What", as a paragraph after the one on its second instance and before the one that names
  `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`, since the "What" ends with its
  re-entry point.

```text
A further instance, at a slice's design audit: in the slice on splits planned before the work, the
audit graded an issue `trigger-met`, `rule-before-work`, its re-entry met by the slice's change to
one head. The planning skill's sorts at a design audit name no verdict action, so the
session had two readings, arguing the instance in the slice or ruling its re-entry. A harvest agent
took the first and closed the thread with no search of its own, against
`design@agent-skills@no-thread-closes-before-the-search`; the review before the merge withdrew the
argument and the re-entry was rewritten so that a change of a host list no longer meets it, per W4
of the retrospective file `2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow`.
```

Default: an instance added to the recorded issue; no text for the lapse.
Outcome: decided by the session: an instance added to
`issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`; no
change for the closure with no search, a lapse. Ground: the search rule is in the design skill and
in `design@agent-skills@no-thread-closes-before-the-search`; one lapse is no recurrence, per the
lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`; the sort's question
is the recorded issue's.

## planning-says-no-development-skill: the planning skill said the workflow installs no development procedure

Members: 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow W3.

- **a. What it says.** The planning skill's "Not covered here" said of writing a step's code "This
  workflow installs none", while the set installs the development skill; the intake's route for a
  change to code named only "the project's own development procedure". An implementer met it; both
  were reworded on the branch.
- **b. Whether it still applies.** No. At the file's version, a9ed6e3f (`v0.10.0-21`), the source
  of the planning skill read "Writing the code** of a step, its claims and its tests: the
  project's own development procedure. This workflow installs none." (`git show a449de16^`). On
  main, `path@agent-skills@content/skills/planning/SKILL.md` near line 20: "**Writing the code** of
  a step, its claims and its tests: `skill@knowledge-architect-development` for the project's Rust
  code, and the project's own development procedure for any other code." The intake's route g, near
  line 161 of its source, names `skill@knowledge-architect-development` the same way. The planning
  sentence landed in a449de16, "[agent-skills] The planning skill names the installed development
  skill for writing a step's code", and the intake's route in 9a872a84, "[agent-skills] The
  retrospective intake's route for a change to code names the installed development skill", both on
  main (`git log -S` on each new phrase).
- **c. Whether it is already recorded.** `design@agent-skills@development-skill` (`constrains`,
  `respect`) is the head the old sentence denied; no issue.
- **d. Its kind.** A defect of the text, wrong, at the file's version.
- **h. The proposal.** No change: already repaired.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground: a449de16 and 9a872a84, on main,
name the installed development skill in the planning skill and in the intake's route.

## standing-2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow: the standing answers

Members: the section "Answers to the standing questions" of
2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow, lines 92 to 124. Its counts are
in the lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`; its tool reports
are item 7 of the owner item in the analysis of
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.

Carried by a subject, named only:

- the harvest agent's misread of the hook documentation, and the harvest's defeat of a rival (lines
  101, 112 to 114): W4 of this file, `harvest-closes-thread-without-search`;
- the search's dispatch, and each searcher's script to fill its report and repair quotations (lines
  104 and 105): `searcher-dispatch-exceeds-harness-cap` and `quotation-check-refuses-cut-punctuation`;
- "the issue on `check --staged` and installed text (C3), graded `unrelated` by its searcher, bore on
  the work's commits" (lines 115 and 116): `check-staged-misjudges-installed-copies`; it is not
  stated as a part added after the search, so it is not added to the issue on misses.

Analysed here:

- **15 of the 27 searchers' reports state a `| tail` on `worklist --check`; none chained a command on
  it** (lines 97 and 98), against `primer@checker-model`, "Run the checker bare, or redirect its
  output to a file". Kind: a lapse, by 15 agents of one dispatch. The searcher's definition asks it
  to "Return the path of your report, the last line `--check` printed on it"
  (.claude/agents/knowledge-architect-record-searcher.md, line 121), the line a `| tail` prints, so
  the definition's step is where the act happens. What the primer's rule prevents is a command
  chained on a verdict the pipe hid, and a lost line; the file reports neither, and the dispatcher's
  `worklist --verify`, which the searcher's description has run on every report, judges the report
  file itself, not the output a pipe cut. The lapse rule's move of delivery needs a
  recurrence that costs something; this one cost nothing reported.
- **One turn reported two premortem causes judged at the harvest with no "Decided" section** (lines
  99 and 100), against `primer@decided-section`. Kind: a lapse of one turn, reported by the session
  itself.
- **An implementer finished before a queued clarification reached it, and was resumed with it; no
  report needed redoing** (lines 110 and 111). Kind: an observation of the harness; the
  report-file trigger of `issue@agent-skills@dispatched-agents-write-their-report-as-they-work` is
  not met.
- **Work the session carried** (lines 108 and 109): small repairs of three rounds, "judged cheaper
  than a brief". **The session reversed its own decision, taken at the audit, to argue that rival in
  the slice** (lines 113 and 114). **Installed file changed**: the source, as the slice's purpose.
  **Code rules**: none met, the only code a test's data table. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: two lapses against clear rules of the primer,
neither with a consequence reported, the `| tail` one at a step of the searcher's definition whose
one dispatch shows a recurrence that cost nothing; the rest are observations. Should a later
session report a verdict lost to such a pipe, that is the evidence a move of the delivery, into the
searcher's step, would need.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## searcher-dispatch-exceeds-harness-cap: the record searcher's dispatch asks for every agent at once, above the harness's cap

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `searcher-dispatch-exceeds-harness-cap`, which gives the outcome.

## check-staged-misjudges-installed-copies: `check --staged` compares staged installed copies with shipped text built from the working tree

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow.md`, in its section on `check-staged-misjudges-installed-copies`, which gives the outcome.

## transcript-branch-attribution-wrong: `transcript list --branch` lists a session that did no work on the branch

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow.md`, in its section on `transcript-branch-attribution-wrong`, which gives the outcome.

## head-repair-rereview-unbounded: the decision-record re-review of a repair that writes a head has no bound but a round of rewordings

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `head-repair-rereview-unbounded`, which gives the outcome.

## fragment-host-list-undecided: no rule said how a fragment's host list is decided

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-knowledge-architect-planned-splits-slice.md`, in its section on `fragment-host-list-undecided`, which gives the outcome.

## repository-departure-buried: a subagent routed an installed-text fix to the workflow file, missing this repository's departure

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-knowledge-architect-planned-splits-slice.md`, in its section on `repository-departure-buried`, which gives the outcome.
