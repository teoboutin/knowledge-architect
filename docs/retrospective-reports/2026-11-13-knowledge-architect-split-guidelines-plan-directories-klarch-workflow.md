# 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow

- **Received file**: 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from source: main at `v0.10.0` (`git describe --tags`), the session's work on one branch cut from it, the tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@undesigned-work-is-an-issue`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `design@agent-skills@needless-instruction`: constrains, respect.
  - `design@agent-skills@goal-review-stops-on-old-wording`: conflicts, check-reversal.
  - `rejected@agent-skills@goal-review-stop-at-a-fifth`: constrains, respect.
  - `rejected@agent-skills@goal-review-stop-over-all-wording`: constrains, respect.
  - `design@agent-skills@local-intent-binds`: constrains, respect.
  - `design@agent-skills@one-session-cost-weighed-before-removal`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `design@check@installed-files-outside-the-walk`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `design@transcript@transcript-format-tolerated-and-reported`: constrains, respect.
  - `design@transcript@format-classed-on-the-inventory`: constrains, respect.
  - `rejected@transcript@strict-transcript-reader`: constrains, respect.
  - `issue@transcript@the-reader-does-not-know-the-api-error-system-subtype`: touches, respect.
  - `design@agent-skills@unrecognised-entries-reach-upstream`: constrains, respect.
  - `rejected@agent-skills@retrospective-asks-for-unrecognised-lines`: constrains, respect.
  - `design@agent-skills@transcript-extraction-is-the-checker-s`: constrains, respect.
  - `design@knowledge-architect@transcript-crate`: constrains, respect.
  - `design@query@context-entries-given-whole`: constrains, respect.
  - `rejected@agent-skills@search-output-committed-whole`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.

## worklist-output-mostly-copies: the search's scratch output is mostly copies of the record

Members: 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow C3.

- **a. What it says.** A measurement, labelled "Not a defect for the scratch use": `worklist --out` and `--verify` wrote 8,193,395 bytes for one search, of which `shown.md` 2,051,313, the verdict files 1,922,848 (617 copied entry blocks for 301 entries), the agents' entry copies about 3.5 MB, the 19 reports 413,365 and `generated-lines.md` 22,581. It decided that a plan directory commits the generated lines only. "Noted for the size of a scratch directory per search."
- **c. Whether it is already recorded.** The measurement is the ground of `rejected@agent-skills@search-output-committed-whole` (`constrains`, `respect`), which kept a plan from committing the search's output whole. `design@query@context-entries-given-whole` (`constrains`) holds the copies of context entries in each agent's file on purpose: "given whole, read only, and given no verdict there". Nothing proposes a change.
- **d. Its kind.** An observation.

Default: no change.
Outcome: decided by the session: no change, an observation. Ground: the file names no defect and no fix; the copies are what `design@query@context-entries-given-whole` and `design@query@occasion-shaped-output` decide, and the size falls on a scratch directory outside the project.

## searcher-misgraded-walk-head: a searcher graded `unrelated` a head on files leaving the walk

Members: 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow W4(a).

- **a. What it says.** "A design head stating that installed files leave the walk by construction,
  parsed for definitions only, was graded `unrelated` against a thread whose subject was which files
  the checker stops reading. [...] The searcher read the draft's 'outside the walk' as about the
  transcript file only." The design-conformance review found it, and it resolved the thread. The
  file's kind: "a grading miss (the first)", with "No proposal beyond that issue" (the issue on
  searches that miss what is added after them, which takes W4(b), not this item).
- **b. Whether it still applies.** The head is `design@check@installed-files-outside-the-walk`
  ("no check walks them for references"), at origin/main, `v0.10.0-59-g605d1538`. The instruction
  the searcher had, at `v0.10.0`, the file's version, and at origin/main: "Read `entries.md` whole,
  so that each head is read with the entries that cite it and those it cites. Do not decide from a
  title which entries are worth reading", and the `unrelated` row, "an entry nothing in the work
  meets", in `path@agent-skills@content/agents/record-searcher.md`.
- **c. Whether it is already recorded.** No entry records searcher grading misses as such. A miss
  is watched by the retrospective's standing question on a missed entry, per
  `design@agent-skills@premortem-as-watch-points`, whose "three causes tell a searcher's judgement
  from an entry or a part of the work that came after the search"; only the last has an issue,
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it`. The searcher's recall stands on
  no measurement, per `design@query@worklist-bound-is-40000-words` ("No measurement of recall stands
  behind any value of the bound"). `rejected@agent-skills@subject-groups-bounded-at-sixty` reopens on
  reports "of large subjects" that "come with missed entries"; this miss is a misreading of the
  draft's words, which the file states, not a size effect.
- **d. Its kind.** A lapse of one agent: the instructions to read whole and to grade by what the
  work meets exist and are clear, and the searcher read the draft too narrowly. One instance, caught
  by the design-conformance review, the reviewer that
  `design@agent-skills@record-searched-before-the-work` names as the backstop through the
  plan-document review of `design@agent-skills@plan-read-against-the-record`. A lapse is not repaired by adding
  text, per `skill@knowledge-architect-retrospective-intake@admission-rules`.

Default: no change.
Outcome: decided by the session: no change, a grading lapse of one searcher, caught by the design-conformance
review, which the record names as the backstop. Ground: item d; the file proposes no fix for it.

## transcript-api-error-subtype-unrecognised: `transcript` reports the `system` subtype `api_error` as unrecognised

Members: 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow C2.

- **a. What it says.** `transcript show` reported `unrecognised system subtype "api_error"` on six
  lines; "the project opened an issue". No further fix proposed.
- **b. Whether it still applies.** Yes. At origin/main, every header of `cargo klarch transcript`
  over this project prints
  `# unrecognised system subtype "api_error": 6, first at <transcript directory of this repository>/2ae9c5de-281f-5d6e-a114-34de90c9fac3.jsonl:751`,
  and `KNOWN_SYSTEM_SUBTYPES` in `path@transcript@src/reader.rs`, lines 47 to 59, does not hold it.
  `cargo x transcript-kinds` gives the line kind "top system api_error: 6 lines; versions 2.1.321"
  and its keys `error`, `level`, `maxRetries`, `retryAttempt` and `retryInMs` beside those of every
  chain line.
- **c. Whether it is already recorded.** Yes:
  `issue@transcript@the-reader-does-not-know-the-api-error-system-subtype` (verdict `touches`),
  opened by the same session on these six lines, with the fields measured and its closing work:
  "`api_error` classed in `KNOWN_SYSTEM_SUBTYPES` as ignored, with a comment naming the fields it
  was classed on, and a test that a line of that subtype raises no note, shown to fail without the
  entry". The class is measured on the inventory, as
  `design@transcript@format-classed-on-the-inventory` asks; the reader stays tolerant, per
  `rejected@transcript@strict-transcript-reader`.
- **f. A better fix.** The rival is handling the issue now, closing it on the intake's branch,
  since its closing work is settled by the issue itself: one entry in `KNOWN_SYSTEM_SUBTYPES`, a
  comment naming the fields, and a test shown to fail without the entry. What decides between them
  is whether the intake's branch already changes Rust source. If other sections' outcomes edit Rust
  on that branch, the code axes of the review and `skill@klarch-development` are owed there anyway,
  and the change costs one more claim under them; the header line it removes is printed by every
  run over this project, the transcript reviews of the intake's own branch included. If the branch
  changes text alone, closing the issue brings those axes onto it for one subtype whose only cost
  is a header line, and the issue waits for a branch of the `transcript` Component.

Default: no change, unless the intake's branch carries Rust edits of other sections, in which case handle now, closing the issue.
Outcome: decided by the session: no change, already recorded by `issue@transcript@the-reader-does-not-know-the-api-error-system-subtype`, which holds the measurement and the closing work. If the intake's branch carries the Rust edits other sections propose, handle now instead: close the issue by its own closing work, under `skill@klarch-development`. Ground: the member is the instance that issue was opened on, and nothing in this batch adds to it; the issue settles the work, so only the cost of the code axes on the branch decides when it lands.

Under the handle-now branch of the outcome, the entry added to `KNOWN_SYSTEM_SUBTYPES` in `path@transcript@src/reader.rs`, after `"stop_hook_summary",`, with the test the issue's closing work names written under `skill@klarch-development` (no agent-facing text changes, so no instruction is added or removed):

```
    // Classed on the 6 lines of this type in the transcripts of one machine, 2026-11-14, harness
    // 2.1.321: each a report of an API call the harness retried, `level` `error`, with `error`,
    // `retryAttempt`, `maxRetries` and `retryInMs`, and no message of the owner or of the session.
    "api_error",
```

## goal-stop-needless-second-round: the stop rule asks a second round when the first repaired nothing

Members: 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow W3. One session.

- **a. What it says.** A batch of one goal had a first draft-mode round with no `blocks` and no `settled` finding, so nothing was repaired. The session stopped after one round. It blames step 4 of `skill@knowledge-architect-goal-setting@drawing-out-intent`, "Stop when a round, from the second on, reports no `blocks` finding on old wording", and proposes allowing the stop at the first round when it reports no `blocks` and no repair was made (0 added). Kind as reported: needless, one occasion.
- **b. Whether it still applies.** Unchanged: `path@agent-skills@content/skills/goal-setting/SKILL.md` lines 104 and 105 at origin/main read "**Stop when a round, from the second on, reports no `blocks` finding on old wording, whatever its route, or when the owner says so.**", as at `v0.10.0`.
- **c. Whether it is already recorded.** `design@agent-skills@goal-review-stops-on-old-wording` (conflicts, check-reversal) records "from the second on" in its title and statement. Its argument does not name a first-round stop as a rival, so the proposal is not one that already lost; `rejected@agent-skills@goal-review-stop-at-a-fifth` and `rejected@agent-skills@goal-review-stop-over-all-wording` (respect) do not cover it either. `design@agent-skills@one-session-cost-weighed-before-removal` (respect) weighs a cost met once against every other occasion before an instruction is narrowed.
- **d. Its kind.** A lapse of the session, not a needless instruction: step 4 states "from the second on", which reads one way, and the session stopped after the first round. The second round is not needless by the definition of `skill@knowledge-architect-retrospective-intake@report-format`, since the head's argument for the rule needs it: its bullet "One round with no `blocks` finding is weak evidence that none remain" is the value of a second, fresh reading.
- **e. Its scope.** The installed text, owned by `agent-skills`. A first-round stop would contradict the statement of `design@agent-skills@goal-review-stops-on-old-wording`, so it is a reversal, owed `skill@knowledge-architect-decision-recording@reversal-check`.
- **f. A better fix.** No change. The text already gives the move the session needed: the same sentence ends "or when the owner says so", so a first round with nothing to repair costs one question to the owner, not a round. Against the reversal: the head states that the stop bounds "a cost no goal ranks", so it reopens only on an argument or a fact that ranks the shapes, and the proposal brings none. Its stated ground, "the version shown is then the version reviewed", holds under the current rule too, since the second round reads that same version; and the head's bullet "One round with no `blocks` finding is weak evidence that none remain" gives the second, fresh reading the value the proposal denies it.
- **g. Its route.** Nothing.

Default: no change.
Outcome: decided by the session: no change, a lapse. Ground: step 4 of `skill@knowledge-architect-goal-setting@drawing-out-intent` admits the owner's word as a stop, which serves this case at the cost of one question; a stop at the first round would reverse `design@agent-skills@goal-review-stops-on-old-wording` with no argument that ranks the shapes, against the weak-evidence bullet of that head, on a cost met once, per `design@agent-skills@one-session-cost-weighed-before-removal`.

## closed-issue-content-lost-at-plan: an issue closed at the plan's commit loses content the plan is not asked to carry

Members: 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow W1.

- **a. What it says.** `skill@knowledge-architect-planning@spec-or-milestone`: "The plan document
  that schedules it closes that issue in the commit that adds the plan document, so the work is
  listed in one place at a time." The milestone closed four issues on that rule. The
  fidelity-of-relocation review found that two lost content no other text held: the baseline counts
  and the closing condition of one, and a second premise of another (7 of 27 searcher reports could
  not read the work whole). A repair pass carried them into the plan from the deleted files'
  history. Proposal: the sentence says the plan document carries the issue's measurements and its
  closing conditions ("adds no instruction"). A second instance, in a project that installs the
  workflow, is in the standing answers of another received file of this batch: a session closed a
  `todo` entry at its plan's commit and dropped three of its leads silently, against a rule of that
  project's own planning skill; four reviewers caught it.
- **b. Whether it still applies.** Yes. At `v0.10.0`, line 86 of
  `path@agent-skills@content/skills/planning/SKILL.md` holds the sentence; on main it is unchanged,
  lines 88 and 89.
- **c. Whether it is already recorded.** No issue. The one head is
  `design@agent-skills@undesigned-work-is-an-issue` (`constrains`, `respect`): "The plan document
  that schedules it closes the issue in the commit that adds the document." A fix keeps the close at
  that commit.
- **d. Its kind.** A defect of the text, unclear in its scope. The act exists:
  `skill@knowledge-architect-issue-tracking@movement-instruction`, lines 233 to 235 of its source,
  "If a closed entry still holds something live, such as an instruction about how to work in that
  area or an uncertainty that survived the fix, that content is not an open issue. Move it to the
  home the knowledge table [...] gives it, **then** delete the file." The primer delivers it at the
  close: its list of installed skills loads `skill@knowledge-architect-issue-tracking` when "work
  closes an entry". Its examples name a close by a fix only. An entry closed because a plan
  document schedules it holds content that is all live, since its work is not done, and the rule
  does not say so, so a session reads the move as owed for a fixed entry. Two instances in two
  projects, one in each, are no recurrence of a lapse at one point of delivery; the rule's scope is
  what both sessions missed.
- **e. Its scope.** The installed text. No head is reversed:
  `design@agent-skills@undesigned-work-is-an-issue` keeps the close at the plan's commit, and the
  knowledge table's row for a plan document is the home of work designed and not built, so the
  live-content rule already sends a scheduled entry's content there.
- **f. A better fix.** The cause is a rule whose examples narrow its scope. The repair is at the
  rule's home, read at every close: its examples name the case of an entry a plan document
  schedules. Nearest rival: the report's fix, a clause in the planning sentence, or a pointer from
  it to the movement instruction; either is a second delivery of an act the primer's load list
  already delivers at the close, and leaves every other close of a scheduled entry, by a spec or by
  a slice, to the same narrow reading. The fact that decides: the movement instruction is the text
  loaded at every close, the planning sentence only where the plan document is written. A second
  rival: no change, leaving the fidelity-of-relocation axis to catch it, as it did here at the cost
  of a repair pass, and in the other project at the cost of four reviewers' findings.
- **g. Its route.** A fix in the installed text, at its source, as `path@agent-skills@CLAUDE.md`
  says, under `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** Handle now. In `path@agent-skills@content/skills/issue-tracking/SKILL.md`,
  section `skill@knowledge-architect-issue-tracking@movement-instruction`, replace the first sentence
  of the paragraph quoted in d by the text below; the next sentence stays. The planning sentence is
  unchanged. Instructions added: 0 (an example within an existing instruction, which states its
  scope); removed: 0.

```text
If a closed entry still holds something live, such as an instruction about how to work in that
area, an uncertainty that survived the fix, or, for an entry a plan document schedules, its leads,
its measurements and its closing condition, that content is not an open issue.
```

Default: handle now, the example above in the movement instruction.
Outcome: decided by the session: handle now, the example above in
`skill@knowledge-architect-issue-tracking@movement-instruction`. Ground: the act exists in that
section, which the primer delivers at every close; its examples narrowed its scope to a fix, and
two sessions in two projects missed it on a scheduled entry; the knowledge table already makes the
plan document the home of that content; `design@agent-skills@undesigned-work-is-an-issue` is kept.

## standing-2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow: the standing answers

Members: the section "Answers to the standing questions" of
2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow, lines 116 to 150.
Its counts are in the lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`;
its missed entry added after the search, W4's second item, is in the lines for
`issue@agent-skills@a-record-search-misses-what-is-added-after-it`.

Carried by a subject, named only:

- the argument that above the worklist bound "a search agent cannot read its cluster whole", and a
  writer agent's "the defaults ... are reported in the commit", where the commit reported none
  (lines 120 to 125): `message-claims-written-unchecked`;
- the first goal commit giving the owner a reason the owner did not give, and dropping a hedge
  (lines 126 to 129): `owner-attribution-slips`;
- the searchers' quotation repairs (line 132): `quotation-check-refuses-cut-punctuation`;
- the 19 search reports and 9 review reports returned into the session's context as messages, "a
  cost the dispatch shape imposes" (lines 137 and 138): `searcher-returns-summary-not-path` and
  `reviewer-returns-whole-report`;
- "A register entry the search missed: W4" (line 145): its first item `searcher-misgraded-walk-head`,
  its second `thread-shape-change-owes-no-search`.

Analysed here:

- **Section sizes of past plan documents extracted by `git show` and `awk`** (lines 132 to 134), "to
  measure their growth, which no command lists". Kind: a measurement taken once by script, for one
  discussion. `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history` is a
  neighbour, on numbers rather than sizes; one session, no command proposed.
- **An agent returned unfinished** (lines 139 to 142): "one searcher stopped twice on an API
  authentication error before doing any work; a third dispatch completed it. The writer agent's plan
  document failed four readiness checks of the cold-implementer review", repaired by a dispatched
  agent "without redoing the assembly". Kind: observations, an environment failure and a review
  working as designed; the report-file trigger of
  `issue@agent-skills@dispatched-agents-write-their-report-as-they-work` is not met, since nothing
  was redone.
- **Work an agent could have carried** (lines 136 and 137): the transcript review's repairs and two
  goal commits, "each was a few lines". **A conclusion that proved wrong**: none acted on; "The
  session spot-checked each investigator's load-bearing quotes, which the primer asks" (lines 143 and
  144). **Installed file changed**: none. **Project skill addition**: none bore. **Backticks**: none.
  **Tool**: none. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: observations, and one measurement taken once
with no command asked for.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `thread-shape-change-owes-no-search`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.

## owner-attribution-slips: the owner quoted, or given a ruling or a reason the owner did not give

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.md`, in its section on `owner-attribution-slips`, which gives the outcome.

## broad-trigger-met-by-every-change: a trigger or re-entry point written as "the next change that touches X" is met by nearly every change

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-13-knowledge-architect-plan-directories-slice-1.md`, in its section on `broad-trigger-met-by-every-change`, which gives the outcome.
