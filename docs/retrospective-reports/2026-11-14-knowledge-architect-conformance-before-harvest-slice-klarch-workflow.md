# 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow

- **Received file**: 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-35-g2abdaa89` when the work began, branch `conformance-before-harvest`, tree clean."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`: touches, respect.
  - `design@agent-skills@plan-read-against-the-record`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.

## plan-row-points-to-show-only: the planning row "What is already decided" lists only the citers `show` gives

Members: 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow W1.
Related: `reversal-lists-omit-readme`, which repairs the list this row is brought to.

- **a. What it says.** The planning skill's table, row "What is already decided": "each recorded
  decision the work reverses or rewrites, with every text that `{{command}} show` lists as
  referencing it". The slice's spec followed it. The rule it rewrote was also stated, citing no
  entry, in a fragment shipped in three installed texts, so `show` did not list it; the code-split
  scout found it at the design audit. The reversal check already says "Grep the behaviour's wording
  as well as the slug, since a pointer that describes the behaviour without naming it is the one a
  slug grep misses". Proposal: the row points to
  `skill@knowledge-architect-decision-recording@reversal-check` instead of "every text that `show`
  lists".
- **b. Whether it still applies.** Yes. At the file's version, 2abdaa89 (`v0.10.0-35`), line 224 of
  `path@agent-skills@content/skills/planning/SKILL.md` holds the row; on main the clause quoted in a
  is unchanged, on the same line, and the row has gained since a clause on the heads its harvest
  will touch, which this repair leaves as it is.
- **c. Whether it is already recorded.** No issue or head on the row. Verdicts: all `constrains` or
  `touches`, action `respect`. `design@agent-skills@plan-read-against-the-record`: the row is what
  the design-conformance reviewer reads declared reversals from, so a reworded row must stay a list.
  `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`
  (`touches`): a pointer that applies a whole section must say to read it whole; the fix below
  applies one bullet of the reversal check, not the section. `design@agent-skills@root-cause-repair`
  and `design@agent-skills@audit-net-growth` bound the repair's shape and count.
- **d. Its kind.** A defect of the text, missing; cause: two homes for one moment. The row and the
  reversal check's repair bullet each define the set of texts a reversal leaves to repair, and the
  row's restatement is narrower than its home.
- **e. Its scope.** The installed text. No head is reversed: the row still lists, now the set its
  home defines.
- **f. A better fix.** The report's fix replaces the row's set by a bare pointer; the row's
  examples, which the design-conformance reviewer reads, would go. The repair below keeps a list and
  takes its set from the reversal check, restating its one sentence with its pointer, as
  `primer@where-knowledge-goes` admits. Nearest rival: the bare pointer; the fact that decides is
  that the row is read as a list by a reviewer, which a pointer alone does not give. Applied with
  `reversal-lists-omit-readme`, the two repairs give the set one home.
- **g. Its route.** A fix in the installed text, at its source, as `path@agent-skills@CLAUDE.md`
  says.
- **h. The proposal.** Handle now. In the table of the section on what a spec holds, row "What is
  already decided", replace "with every text that `{{command}} show` lists as referencing it (a
  tripwire, an issue, a restatement in a `CLAUDE.md` or a skill, a README, a comment)" by the text
  below. Instructions added: 0; removed: 0 (the act of listing stays; its set is the home's).

```text
with every text the reversal will repair, as the repair bullet of
`skill@knowledge-architect-decision-recording@reversal-check` finds them, those `{{command}} show`
lists as referencing it and those a grep of its wording returns (a tripwire, an issue, a
restatement in a `CLAUDE.md` or a skill, a README, a comment)
```

Default: handle now, the rewording above.
Outcome: decided by the session: handle now, the rewording above. Ground: the row restates the set
of the reversal check's repair bullet narrower than its home; brought back to it with its pointer,
it keeps the list `design@agent-skills@plan-read-against-the-record` reads.

## standing-2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow, lines 69 to 110. Its counts are in the lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`, its missed
entry added after the search in the lines for
`issue@agent-skills@a-record-search-misses-what-is-added-after-it`.

Carried by a subject, named only:

- "The session's own answer to a search verdict was wrong: it gave a head a member the head's
  argument did not cover. The merge review found it" (lines 96 to 98): `head-changed-without-design-skill`,
  which adds it to `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`;
- "one text, the planning skill, states" where five texts did, and "156 verdicts" where the
  committed lines hold 171 (lines 74 to 80): `message-claims-written-unchecked`;
- the searchers' quotation repairs, and "Two searchers finished after their first return, still
  repairing quotations" (lines 84, 95): `quotation-check-refuses-cut-punctuation`; two rewords by a
  `--msg-filter` script (lines 85 and 86): `no-command-rewords-branch-messages`;
- "The scout could not write its report file: the harness refused it. It returned the report as
  text" (lines 93 and 94): `report-file-write-refused`;
- `design@agent-skills@calibration-in-the-run` and the rejected design it beat, graded `unrelated`
  before the audit added the cut list's sample (lines 100 to 103): `thread-shape-change-owes-no-search`,
  and an instance in the lines above.

Analysed here:

- **"The scope of the incumbent head's argument was found by the review before the merge"** (line
  104). Kind: an observation; the search returned the head, and its scope was the review's reading.
- **Work the session carried** (lines 89 to 91): the harvest and every repair round, "because it
  reasoned over them later. The context spent was large: four rounds of reports." Kind: the dispatch
  judgement of `primer@dispatching-work`, which keeps with the session a piece it will reason over
  later; an observation, its cost counted.
- **Code rules** (lines 87 and 88): "The scout ran at the audit and its signals, all weighed, led to
  no split." **Installed file changed**: the source, as the work. **Project skill addition**: none.
  **Backticks**: none. **Tool**: none. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: observations of the search, the review and the
dispatch judgement working as the record decides.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## searcher-dispatch-exceeds-harness-cap: the record searcher's dispatch asks for every agent at once, above the harness's cap

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `searcher-dispatch-exceeds-harness-cap`, which gives the outcome.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `thread-shape-change-owes-no-search`, which gives the outcome.

## check-staged-misjudges-installed-copies: `check --staged` compares staged installed copies with shipped text built from the working tree

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow.md`, in its section on `check-staged-misjudges-installed-copies`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## transcript-branch-attribution-wrong: `transcript list --branch` lists a session that did no work on the branch

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow.md`, in its section on `transcript-branch-attribution-wrong`, which gives the outcome.

## transcript-list-cut-notices-other-sessions: a filtered `transcript list` prints the harness's cut notices of sessions the filter left out

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow.md`, in its section on `transcript-list-cut-notices-other-sessions`, which gives the outcome.

## report-file-write-refused: the harness refuses a subagent's write of a report file by its name

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow.md`, in its section on `report-file-write-refused`, which gives the outcome.

## silent-condition-read-alone: condition 1 of the tripwire standard read without the paragraph on slips

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow.md`, in its section on `silent-condition-read-alone`, which gives the outcome.

## head-changed-without-design-skill: code or an agent's edit contradicted, narrowed or rewrote a head with no deliberation

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage-klarch-workflow.md`, in its section on `head-changed-without-design-skill`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
