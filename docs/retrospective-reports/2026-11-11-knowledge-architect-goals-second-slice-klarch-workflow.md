# 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow

- **Received file**: 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "The main commit the session's tree contained when its work began is `4a8345c2` (`git describe --tags`: `v0.9.0-288-g4a8345c2`). The session worked on the branch `goals-checker-root-only`, merged. Whether the tree held uncommitted changes at the start is not established".
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@query@worklist-writes-and-verifies-the-reports`: conflicts, check-reversal.
  - `design@query@worklist-check-judges-one-report`: conflicts, check-reversal.
  - `issue@query@a-quotation-cut-from-its-punctuation-is-refused-as-out-of-order`: touches, respect.
  - `design@agent-skills@committed-intake-analysis`: constrains, respect.
  - `design@agent-skills@every-verdict-quotes-its-entry`: conflicts, check-reversal.
  - `design@check@finding-names-the-repair`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@agent-skills@transcript-review-last-before-merge`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@agent-skills@plan-reviewed-as-a-commit`: constrains, respect.
  - `design@agent-skills@a-scratch-directory-per-subagent`: constrains, respect.
  - `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`: touches, respect.
  - `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`: touches, respect.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `rejected@agent-skills@retrospective-finding-straight-to-issue`: constrains, respect.
  - `design@agent-skills@milestone-split-by-lifetime`: constrains, respect.
  - `design@agent-skills@discussion-record-apart`: constrains, respect.
  - `rejected@agent-skills@deliberation-in-the-searched-document`: constrains, respect.
  - `design@agent-skills@record-searched-before-the-work`: conflicts, check-reversal.
  - `design@model@plan-document-directory`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `goal@projects-are-split-into-modular-units`: constrains, respect.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Members: 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow C4(a); 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow C2(a); 2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow C1; 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow C1; 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow C1; 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow C1; 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow C1; 2026-11-13-thaum-long-draw-klarch-workflow C1; 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow C3; 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow C2(b); 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow C1; 2026-11-14-thaum-casting-process-design-klarch-workflow C1.

- **a. What it says.** Twelve sessions report that `worklist --check` (or `--verify`) refused most first drafts of the searchers' quotations. Most refused quotations ended one word short of the punctuation the entry's text attaches to that word, such as `job` where the text has `job,`; a few held fewer than eight words. The counts reported per report run from 2 to 50. Nearly every searcher then wrote a script of its own (`fixq.py`, `fix.py`, `python3 -I` scripts) that extended each quotation to whole words, and ran the check 2 to 4 times. The dispatch-judgement file says "16 of the 18 searchers' reports state that their first `--check` failed". The finding-repairs file adds that the message named an order problem, "not in this order", which made the searcher take the matcher for broken. The proposed fixes are three: the finding prints the whole-word run it would accept (dispatch-judgement, 2026-11-12-knowledge-architect-new-crate-placeholder, retrospective-standing-questions, split-guidelines, long-draw, casting, planned-splits); `worklist --check --fix` writes that run (dispatch-judgement); or the check accepts a quotation that differs from the text only by punctuation at its ends (2026-11-12-knowledge-architect-new-crate-placeholder, split-guidelines, long-draw, planned-splits).
- **b. Whether it still applies.** Yes. Two files used a tree before the message repair: goals-second-slice at `v0.9.0-288-g4a8345c2` and primer-import-first-line at `v0.9.0-304-gd687aa43`. The commit 8af05fc3 (`v0.9.0-320`), "[query] The report check says what keeps a quotation from being found, and which line it may not come from", made the message name the cause. The ten other files used trees from `v0.9.0-371` to `v0.10.0-43`, or the release 0.10.0, all after it, and still report the refusal and the scripts. At origin/main, `path@query@src/worklist.rs` still matches a quotation as a run of whole words (the comment "Whole words, so a quotation that starts or ends inside a word is not found", near line 1716), and `quotation_miss` (near line 1344) still falls to "each of its words is in the text, but not in this order" when the cut word stands whole elsewhere in the body. No finding prints the run that would pass.
- **c. Whether it is already recorded.** Yes, in part. `issue@query@a-quotation-cut-from-its-punctuation-is-refused-as-out-of-order` (kind `defect`, verdict `touches`, action `respect`) records the misleading message, opened by 6b52f8f5 from the finding-repairs session. Its closing condition is the message naming the cut edge word and the body word that holds it. It does not record the recurrence across twelve sessions, nor that searchers script the repair even where the message is right: the issue's own observation, from the finding-repairs session, shows one quotation named right ("its word `less` is part of the word `less:` there") among the refused ones, and the sessions after `v0.9.0-320` still scripted. Its last paragraph already sends the rival "accept the quotation" to `skill@knowledge-architect-design`, since it would change the run of whole words of `design@query@worklist-writes-and-verifies-the-reports` (verdict `conflicts`, `check-reversal`). The rival "accept" also relaxes "verbatim" in `design@agent-skills@every-verdict-quotes-its-entry` (`conflicts`, `check-reversal`). The rival `--check --fix` reverses "writes and removes no file" of `design@query@worklist-check-judges-one-report` (`conflicts`, `check-reversal`). The rival "print the accepted run" keeps all three heads, and is what `design@check@finding-names-the-repair` asks of a finding ("A finding's action names the repair"), a head of `check` whose rule the query command follows by analogy only (assumption: no head of `query` states it for `worklist`).
- **d. Its kind.** A defect of the checker: the message is wrong in one case (recorded), and needless in effect in every case, since each searcher builds the same repair again.
- **e. Its scope.** The checker, Component `query`, per `design@knowledge-architect@query-crate`. Under `goal@tasks-a-tool-does-better-are-the-tool-s`, the record search's check is already a tool the record gives the task to, `design@query@worklist-writes-and-verifies-the-reports`; making its finding name the passing run changes that tool, and takes in no new task, so the searcher's owner-question on taking in a task does not arise here. Accepting the cut quotation or writing it reverses a head, so goes to the design skill.
- **f. A better fix.** One fix for the whole cluster: the finding names the passing quotation, the quoted run extended to the whole words of the text, punctuation included. It keeps the verbatim rule and the read-only check, and it covers the message defect the issue records. Nearest rival: accepting a quotation that differs only by edge punctuation, which removes the round trip entirely but reverses two heads. The fact that decides between them is not established: whether a printed run still leaves the searcher scripting the edit of 6 to 50 lines per report. That is the design question the issue should carry. Rival "no change": rejected, twelve sessions pay it.
- **g. Its route.** A change to the Rust source, `skill@klarch-development`, on a branch of its own, after the shape is argued under `skill@knowledge-architect-design`.
- **h. The proposal.** Add the recurrence to the recorded issue, and name in it the three rivals and the open fact, so that the branch that closes it argues the shape. Widen the issue's closing condition in the same edit. As it stands, "What would close it" is met by the message repair alone, and the commit that closes the entry would delete it, the recurrence with it, while the round trip stays: the files after `v0.9.0-320` show scripts that follow a right message too. The issue's own "Why it matters" already names that round trip ("at the cost of one more `--check` round per quotation"), so the widening follows from the entry and decides nothing new. The message repair is not handled now on its own: a printed passing run, one of the shapes, would make it needless.

Default: instance added to the recorded issue, and its closing condition widened.
Outcome: decided by the session: instance added to `issue@query@a-quotation-cut-from-its-punctuation-is-refused-as-out-of-order`, and its "What would close it" widened to the round trip, its work kept for a branch of its own. Ground: the refusal recurs in twelve sessions, ten of them after the message repair of 8af05fc3; of the three shapes that remove the round trip, two change `design@query@worklist-writes-and-verifies-the-reports` and `design@agent-skills@every-verdict-quotes-its-entry`, or `design@query@worklist-check-judges-one-report`, and the choice among the three turns on a fact not established (item f), so it goes to the design skill, which an intake branch does not carry; the entry's closing condition as written would close it on the message alone and drop the recurrence, while its "Why it matters" is the round trip.

Text to add to the issue, as a paragraph at the end of "Details", "What", after "Ruled out":

```markdown
**Recurrence**, from the retrospective files received at the intake of 2026-11-14: twelve sessions,
ten of them on a tree after the message repair of 8af05fc3, report that most searchers' first
`--check` failed on quotations cut one word short of their punctuation, or under eight words, from
2 to 50 per report, and that nearly every searcher wrote a script of its own to extend each
quotation to whole words, then ran the check 2 to 4 times. The scripts follow a right message as
well as a wrong one, so a repair of the message alone leaves the round trip.
The files propose three shapes: the finding prints the whole-word run it would accept, punctuation
included, which keeps every head; `--check --fix` writes that run, against
`design@query@worklist-check-judges-one-report`; or the check accepts a quotation that differs only
by punctuation at its ends, against the run of whole words of
`design@query@worklist-writes-and-verifies-the-reports` and the verbatim quotation of
`design@agent-skills@every-verdict-quotes-its-entry`. Not established: whether a printed run still
leaves each searcher scripting the edit of its refused lines.
```

Text to add to the issue, as a paragraph at the end of "What would close it", after the paragraph on accepting such a quotation:

```markdown
The message repair alone no longer closes this entry, since the round trip of the recurrence under
"What" follows a right message too. The entry closes with a shape, argued under
`skill@knowledge-architect-design`, by which a searcher repairs a refused quotation with no script
of its own, the message repair above built with it or made needless by it; or with the owner's
ruling, in the commit that deletes this entry, that the round trip stays.
```

## repair-skipped-review-before-transcript: a head-changing repair reached the last transcript review with no decision-record review, a second time

Members: 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow W2.

- **a. What it says.** "A repair commit that changed five heads, made on the owner's answer to a
  material finding, went to the transcript reviewer with no decision-record review. The transcript
  reviewer named the gap. The decision-record review then ran after it and found seven items." It
  blames `skill@knowledge-architect-review@what-review-leaves`, "A repair made before the last
  transcript review is reviewed before it", and calls it a lapse: "The instruction is clear and
  exists, so this is a lapse, and per the admission standard it earns no text." No proposal. It adds
  that the rule's occasion came three times in the session, and that "whether a repair "made before
  the last transcript review" exists is known only once the reviews stop".
- **b. Whether it still applies.** The sentence is unchanged from 4a8345c2 (`v0.9.0-288`, the file's
  version), lines 168 to 169, to origin/main, `path@agent-skills@content/skills/review/SKILL.md`,
  lines 178 to 179. The sentence the session reads when it dispatches the transcript reviewer is in
  `skill@knowledge-architect-review@review-axes`, line 66 at origin/main and line 64 at 4a8345c2,
  the same text: "**Last, at the review before a merge, the transcript reviewer runs alone**: after
  every other axis has run, its repairs are committed and every item of their reports has its
  outcome under `skill@knowledge-architect-review@what-review-leaves`". It names no re-review a
  repair owes. That the repair met the trigger (added or removed a head) is the report's claim, not
  re-checked against the branch: an assumption.
- **c. Whether it is already recorded.** The rule is
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: "A repair made before the last
  transcript review is reviewed before it"; the order is
  `design@agent-skills@transcript-review-last-before-merge` (verdict `constrains`, `respect`). The
  same lapse was analysed once before, W5 of
  2026-11-02-knowledge-architect-development-and-components-klarch-workflow, in "[intake] The
  analyses of forty received retrospective files, three outcomes for the owner" (e8562915, on main):
  in a third slice "the session ran the transcript review next and found the missed decision-record
  review only on re-reading the skill"; outcome "W5 no change, a lapse", on the ground "one miss,
  followed in order at the next slice, so no move of the sentence", the report having proposed that
  "if it recurs, the sentence moves". That instance is in the series of
  `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`.
- **d. Its kind.** A lapse of the session, now recurring: two sessions, each at the same moment, the
  dispatch of the last transcript review after a head-changing repair. By the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules`, "Where the lapse recurs at one
  point of delivery, because the instruction is read at one moment and needed at another, the fix
  moves where it is delivered, and the recurrence is its evidence." The instruction is stated where
  repairs are described, and needed where the transcript reviewer is dispatched, whose sentence lists
  what must precede it and omits it.
- **e. Its scope.** `goal@design-is-recorded-with-its-arguments`, which the re-review serves, and
  `goal@only-needed-instructions` (no instruction added). Owner: the installed review skill. The edit
  moves a sentence within one skill and changes no decision: the rule and the order of the two heads
  above stay as they are.
- **f. A better fix.** Rivals: (1) no change, as at the last intake; (2) the 2026-11-02 report's own
  site, "where a repair is committed", which is where the sentence already sits, in
  `skill@knowledge-architect-review@what-review-leaves`, and where both lapses happened; (3) a
  duplicate at the dispatch sentence, which the need test fails while its twin stays. The fact that
  decides: both misses were found at the transcript reviewer's dispatch, or after it, so the
  condition belongs in the sentence that lists what precedes that dispatch, and moving it there
  leaves one statement of it. With (1), a third miss would cost a third out-of-order review.
- **g. Its route.** A text edit of installed text whose design is settled, at its source, as
  `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", says. Handle now.
- **h. The proposal.** In `path@agent-skills@content/skills/review/SKILL.md`, the paragraph of
  `skill@knowledge-architect-review@review-axes` at line 66 becomes:

  ```
  **Last, at the review before a merge, the transcript reviewer runs alone**:
  after every other axis has run, the re-review that each repair adding, removing or reversing a
  design head owes included, their repairs are committed and every item of their reports has its
  outcome under `skill@knowledge-architect-review@what-review-leaves`, since an answer of the owner
  given after its extraction reads to it as missing.
  ```

  The rest of the paragraph, from "Its range is the whole branch", is kept. "The re-review that
  each repair ... owes" keeps the trigger's whole reach, the decision-record axis "and any other axis
  whose condition it meets", which a sentence naming the decision-record review alone would narrow.

  and in `skill@knowledge-architect-review@what-review-leaves`, lines 178 to 180, the sentence
  "A repair made before the last transcript review is reviewed before it, and that transcript review
  then runs as `skill@knowledge-architect-review@review-axes` says; a repair of the last transcript
  review is reviewed after it, and the review ends with the repairs of that decision-record review."
  becomes:

  ```
  A repair of the last transcript review is reviewed after it, and the review ends with the repairs
  of that decision-record review; one made before it is reviewed before it, as
  `skill@knowledge-architect-review@review-axes` says.
  ```

  Instructions: 0 added, 0 removed (one moved to its point of delivery, its old site reduced to a
  pointer). Nearest rival: no change; deciding fact: the second miss at the same moment. The `%%`
  comment above the paragraph of `skill@knowledge-architect-review@what-review-leaves` keeps its citation of
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`; the head needs no edit, since
  it states the rule, not where the skill delivers it. Held by the session.

Default: handle now, the move above.
Outcome: decided by the session: handle now (the condition moved into the transcript reviewer's
dispatch sentence of `skill@knowledge-architect-review@review-axes`, its old site a pointer).
Ground: the lapse recurs at one point of delivery, the second instance after the one analysed in
e8562915, and the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`
moves the instruction to where it is needed; `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`
and `design@agent-skills@transcript-review-last-before-merge` stand unchanged.

## reviewer-worktrees-by-hand: each reviewer's worktree and scratch directory made by hand

Members: 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow standing answer "A task carried by hand or by a script?"; 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow standing answer "A task carried by script or by hand?"; 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow standing answer "A script, or a mechanical task by hand?".

- **a. What it says.** Three sessions report, in their standing answers, that they made each reviewer's detached worktree and scratch directory by hand. goals-second-slice, lines 105-106: "the worktree and scratch directory of each of about twenty reviewers, made by hand; the project has an issue on it." 2026-11-12-knowledge-architect-new-crate-placeholder, lines 94-95: "the worktrees and scratch directories of each reviewer, made by hand, which the project records as" the issue below. retrospective-standing-questions, lines 95-96: "The session created 7 detached worktrees and scratch directories for the review by a shell loop, then 3, then 1, and removed them, as the review skill's invariants ask." No file proposes a fix beyond the recorded issue.
- **b. Whether it still applies.** Yes. The invariant still asks for one worktree and one scratch directory per subagent, at origin/main, `path@agent-skills@content/skills/review/SKILL.md`, the bullet that opens "Every subagent that judges a commit": "The shape is `git worktree add --detach <path> <commit>`, ... and `git worktree remove <path>` once the subagent's findings have their outcomes." No command makes them: the issue's own check, `klarch --help` and the commands of the maintenance tool, still holds; `git log --oneline -3` on the issue file shows no commit that closed it, the last being "[model] The slice's design audit of the goals' second slice ...".
- **c. Whether it is already recorded.** Yes: `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand` (kind todo), graded `touches`, action `respect`, by agent 15, whose reason names these three standing answers as further instances. Its instance list holds three sessions, of 2026-10-31, 2026-11-06 and 2026-11-07; none of the three files here. Its "What would close it" opens "The owner takes the task in or declines it", per `goal@tasks-a-tool-does-better-are-the-tool-s` (agent 18, `constrains`, `respect`: "A task is taken in by a decision of knowledge-architect's owner, never because the workflow asks it."). The issue is in the roadmap's unordered section, `path@knowledge-architect@docs/roadmap.md` line 50. `design@agent-skills@a-scratch-directory-per-subagent` (agent 6, `respect`) keeps one scratch directory per subagent, which a command must still give.
- **d. Its kind.** An observation: three further instances of a recorded todo, no new defect of the text.
- **h. The proposal.** Add the three instances to the issue. Whether the task is taken in stays the question the issue already parks for the owner; the intake does not put it again, since the issue carries it and no file adds a fact that answers it.

```markdown
- three sessions of 2026-11-11 and 2026-11-12, a goals slice, a new crate's placeholder and the
  retrospective's standing questions: about twenty reviewers' worktrees and scratch directories
  made by hand in the first; by hand in the second; by shell loops for 7, then 3, then 1
  reviewers, each removed after, in the third.
```

Default: add the three instances to the issue's instance list.
Outcome: decided by the session: no change to the text; the instances are added to `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`. Ground: the issue records the task and parks the owner's take-in, per `goal@tasks-a-tool-does-better-are-the-tool-s`; the three files add instances and no new fact.

## slice-audit-searches-whole-milestone: a slice's design audit sends the whole milestone document to every searcher

Members: 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow W1.

- **a. What it says.** `skill@knowledge-architect-planning@working-a-slice` had the design audit
  search the record with the slice's spec and the milestone document as its work. The milestone
  document held 24,977 words, so each agent had about 13,800 words of entries under the bound of
  40,000, and the search took 44 agents. The owner judged it a defect of the planning skill and of
  the size of milestone documents. The session opened an issue; the proposal was left to that
  issue's discussion.
- **b. Whether it still applies.** No. At the file's version, `v0.9.0-288-g4a8345c2`, the planning
  skill's point 2, line 407 of the source, read: "`--work` the slice's spec and the milestone
  document". On main, the same point, near line 571 of
  `path@agent-skills@content/skills/planning/SKILL.md`, reads: "`--work` the milestone's `README.md`
  and the slice's `README.md`, and no other file", and "`deliberation.md`, `search/` and
  `transcript.txt` are never the search's work". It entered in f93bc0d9 (`v0.10.0-7`).
- **c. Whether it is already recorded.** Yes, and closed. The issue the session opened, added in
  b2bfe329, was `a-slice-audit-s-search-reads-the-whole-milestone-document-in-every-agent`
  (agent-skills); 1beacf9d closed it as scheduled by the milestone on split guidelines and plan
  directories, "by the slice audit's search reading the milestone's and the slice's own documents"
  (`git show 1beacf9d`). That issue's figures are the measurement of
  `rejected@agent-skills@deliberation-in-the-searched-document` and of
  `design@agent-skills@discussion-record-apart`. One verdict is `conflicts`, `check-reversal`, on
  `design@agent-skills@record-searched-before-the-work`; it bears on a fix that would change the
  audit's work again, and this section proposes none, so no reversal check is owed.
- **d. Its kind.** A defect of the text, needless (cost no result needed), at the file's version.
- **h. The proposal.** No change: repaired by the milestone's work.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground: the slice audit's `--work`
is now the two `README.md` files alone (planning skill, point 2), built in f93bc0d9; the issue the
file names was closed by 1beacf9d with that work.

## standing-2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow, lines 87 to 107. The file
predates the questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- the rewrite of a released changelog line put to the owner as "a case you rule under" a rule that
  admitted only another case (lines 93 to 97): `owner-premise-unchecked`;
- the refused form written bare at ten sites, "beside no issue entry" (lines 102 to 104):
  `refused-form-escapes-bare-lint`;
- C3 and C4 (line 105): `named-repairs-applied-by-hand`, `quotation-check-refuses-cut-punctuation`
  and `no-command-rewords-branch-messages`; the worktree and scratch directory of each of about
  twenty reviewers, made by hand (lines 105 and 106): `reviewer-worktrees-by-hand`.

Analysed here:

- **The reversal's message stated no search** (lines 98 to 100):
  `skill@knowledge-architect-decision-recording@reversal-check`, "Say in the commit message what
  you searched and what it returned", not done; "The review found three stale statements a search
  would have found." Kind: a lapse against a clear instruction, caught by the review. Counted with
  the batch's two other lapses of `skill@knowledge-architect-decision-recording@reversal-check` in the standing section of
  2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow, each on another step.
- **Installed file changed** (lines 89 to 91): at its source, two sentences the slice made false.
  **Project skill addition missed**: none. **Code rules**: none. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: one lapse against a clear instruction of the
decision-recording skill, caught by the review, with no recurrence at that step in the batch; the
rest are observations.

## named-repairs-applied-by-hand: the checker names each repair of a reference, and sessions apply them by scripts

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `named-repairs-applied-by-hand`, which gives the outcome.

## refused-form-escapes-bare-lint: a refused reference form written without backticks escapes every check

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice.md`, in its section on `refused-form-escapes-bare-lint`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## session-owner-line-unrecognised: the transcript reader reports the harness's `session-owner` line type as unrecognised

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow.md`, in its section on `session-owner-line-unrecognised`, which gives the outcome.

## owner-premise-unchecked: questions put to the owner on premises the session had not checked

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.md`, in its section on `owner-premise-unchecked`, which gives the outcome.
