# 2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow

- **Received file**: 2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, at main `v0.9.0-223-g350d4d59`. The session worked on the branch `goal-session-orchestrates-the-work`, and the tree was clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@goals-axis-in-the-review-before-a-merge`: constrains, respect.
  - `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`: touches, respect.
  - `design@agent-skills@goal-reviewer-agent`: constrains, respect.
  - `design@agent-skills@goal-review-stops-on-old-wording`: conflicts, check-reversal.
  - `design@agent-skills@goal-edit-ruled-alone`: constrains, respect.
  - `design@agent-skills@expectation-set-bounds-scope`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `design@transcript@transcript-format-tolerated-and-reported`: constrains, respect.
  - `rejected@transcript@strict-transcript-reader`: constrains, respect.
  - `design@agent-skills@unrecognised-entries-reach-upstream`: constrains, respect.
  - `rejected@agent-skills@retrospective-asks-for-unrecognised-lines`: constrains, respect.
  - `design@agent-skills@transcript-extraction-is-the-checker-s`: constrains, respect.
  - `design@knowledge-architect@transcript-crate`: constrains, respect.
  - `design@agent-skills@test-economy`: constrains, respect.
  - `design@agent-skills@code-shape-axis`: constrains, respect.
  - `design@knowledge-architect@mutation-command-host`: constrains, respect.
  - `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`: touches, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.

## session-owner-line-unrecognised: the transcript reader reports the harness's `session-owner` line type as unrecognised

Members: 2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow C1; 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow C1(a); 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow C5; 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow C3; 2026-11-11-thaum-split-oversized-modules-klarch-workflow C1, with its `local_command` lines "no commandRun".

- **a. What it says.** Five sessions carried the header's request into their files:
  `klarch transcript` reported `unrecognised type "session-owner"` (206, 435, 211, 757 and 250
  lines; four keys, `type`, `accountUuid`, `organizationUuid` and `sessionId`, first at harness
  2.1.319, per record-search-verdicts C1). thaum's C1 also reports two `local_command` lines with
  "no commandRun". Proposed fix: class the type as a metadata line, counted, its values not printed;
  recognise both.
- **b. Whether it still applies.** No. The commit "[transcript] The line type `session-owner` is
  classed as metadata that carries no message" (`4e6e4ceb`, `v0.9.0-322`, in `v0.10.0`) added the
  type to `KNOWN_METADATA_TYPES` in `path@transcript@src/reader.rs`, line 42 at origin/main, on a
  measurement of 930 lines, and closed the issue on that type. The commit "[transcript] A command
  the harness writes with no `commandRun` is the owner's, named by its leading `<command-name>` tag"
  (`ba463928`, `v0.9.0-362`, in `v0.10.0`) classed the `local_command` lines with no `commandRun`
  that `/restart` writes. The members' versions are `v0.9.0-223`, `-257`, `-288` and `-304` of this
  repository and thaum's pin `=0.9.0`, each before both commits. Re-taken at origin/main:
  `cargo klarch transcript list --all-projects` names no `session-owner` and no `local_command`
  value in its header, only `unrecognised system subtype "api_error": 6`, which is
  `transcript-api-error-subtype-unrecognised`.
- **c. Whether it is already recorded.** The route that brought these findings is
  `design@agent-skills@unrecognised-entries-reach-upstream` (verdict `constrains`, `respect`); the
  repair kept the reader tolerant, per `rejected@transcript@strict-transcript-reader`.

Default: no change.
Outcome: decided by the session: no change, already repaired. Ground: `4e6e4ceb` and `ba463928`, both after every member's version and in `v0.10.0`; the header at origin/main names neither value.

## stale-mutation-records-unfound: no command finds a test's recorded mutation whose quoted code has left the source

Members: 2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow C2.

- **a. What it says.** The code-shape reviewer wrote a script of about twenty lines that gathers
  each test's doc comment recording a mutation, extracts each fragment quoted before "made", and
  reports those no longer in the source; it found two records whose quoted code had changed since
  the mutation ran, one stating an outcome the code no longer produced. Proposed fix: a command
  listing each recorded mutation whose quoted fragment no longer appears in its file, which the
  code-shape reviewer points to.
- **b. Whether it still applies.** Yes. At origin/main
  `path@agent-skills@content/skills/development/SKILL.md`, line 104: "**A recorded check quotes**
  the `-` and `+` lines of the mutation, the file and the function or type that holds them, and the
  verdict"; the code-shape reviewer's report names "for a test of logic, its recorded mutation"
  where missing (`path@agent-skills@content/agents/code-shape-reviewer.md`, line 46), and nothing
  asks whether the quoted code still exists. No command does it: no verb of `klarch` and no
  `cargo x` command reads recorded mutations (`ls tools/xtask/src/`, and `klarch --help`, list
  none).
- **c. Whether it is already recorded.** No entry records the task.
  `design@agent-skills@code-shape-axis` and `design@agent-skills@test-economy` (verdicts
  `constrains`) place the judgement of a test's recorded mutation with the code-shape reviewer,
  which a command would serve. Its placement is open: under
  `design@knowledge-architect@mutation-command-host` (verdict `constrains`) a tool whose result
  exists without running the tests is no command of the mutation crate, and the rule that places
  every other tool in `klarch` is argued by no head,
  `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head` (verdict
  `touches`).
- **d. Its kind.** A missing capability: a task whose result follows from the tree with no
  judgement, carried by a script, met once.
- **e. Its scope.** `goal@tasks-a-tool-does-better-are-the-tool-s` and
  `goal@the-workflow-leads-to-better-code`. The goal reserves the step this subject needs: "A task
  is taken in by a decision of knowledge-architect's owner, never because the workflow asks it." No
  decision takes this task in: `design@mutate@mutation-command` takes in the run of a mutation, not
  a reading of the records.
- **f. A better fix.** The rival is a sentence in the code-shape reviewer's definition asking it to
  check each recorded mutation's quoted lines against the source, with no command; the deciding fact
  is the goal's, that a task whose result follows from the tree is a tool's once taken in, while the
  reviewer already found the two records unprompted. The other rival is no change, on one instance.
- **g. Its route.** For the owner first, since taking the task in is the owner's by the goal; then
  an issue, whose placement waits on the unargued rule above.
- **h. The proposal.** Ask the owner whether the task is taken in. Default: open a question issue at
  the root, `knowledge-architect`,
  `issue@knowledge-architect@no-command-finds-a-recorded-mutation-whose-code-left-the-source`, holding the instance, the
  proposal, its two rivals and the placement question, so the instance is kept until the owner's
  word; taken in, the issue becomes a todo; not taken in, it closes with that word.

Default: open the question issue `issue@knowledge-architect@no-command-finds-a-recorded-mutation-whose-code-left-the-source` at the root anchor `knowledge-architect`.
Outcome: decided by the session: open the question issue `issue@knowledge-architect@no-command-finds-a-recorded-mutation-whose-code-left-the-source` at the root anchor `knowledge-architect`. Ground: opening an issue takes no task in, so the issue parks for the owner whether the task is taken in, which `goal@tasks-a-tool-does-better-are-the-tool-s` reserves to the owner; the intake reads every "no command does X" finding of this batch so, and the owner's answer, given in that issue, needs no wait here.

## goal-rounds-and-merge-axis-unjoined: how a goal rewording drafted at a review joins the drafting rounds

Members: 2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow W1, 2026-11-13-thaum-pin-move-0-10-klarch-workflow W3. Two sessions.

- **a. What it says.** W1: the goals axis of a review before a merge, in merge mode, reported a `blocks` finding and several `asks` findings on two goals. The session put defaults to the owner, then ran a second merge-mode round on its own judgement, and repaired a `settled` finding that round reported, though the stop rule sends a stopping round's findings to the owner unrepaired. It blames `skill@knowledge-architect-review@what-review-leaves`, "A repair that rewords a goal is drafted as its own item under `skill@knowledge-architect-goal-setting@drawing-out-intent`", beside step 4 of that section, "Stop when a round, from the second on, reports no `blocks` finding on old wording". It asks whether the merge-mode report is round one of the drafting procedure, and proposes one sentence in the review skill saying it is (1 added). W3, from the other project: each round's repairs are new wording, so a round with repairs always owes another; the session sent the rewordings approved at the stop to the merge-mode goals axis, on the owner's word. It proposes saying so in `skill@knowledge-architect-goal-setting@whole-set-review` (0 added).
- **b. Whether it still applies.** The blamed texts are unchanged from both versions to `v0.10.0-59-g605d1538`: `git show 350d4d59:crates/agent-skills/content/skills/goal-setting/SKILL.md` and the same at `v0.10.0` hold step 4 word for word, and the review skill at `350d4d59` holds the sentence (`grep -c` gives 1). At origin/main, `path@agent-skills@content/skills/review/SKILL.md` line 159 reads "A repair that rewords a goal is drafted as its own item under `skill@knowledge-architect-goal-setting@drawing-out-intent`, never ruled inside a batch of items that are not goals." Step 4 of `skill@knowledge-architect-goal-setting@drawing-out-intent`, `path@agent-skills@content/skills/goal-setting/SKILL.md` lines 104 and 105 for the stop, dispatches "`agent@knowledge-architect-goal-reviewer` in its draft mode" for each round. W3's request is already in that step: "Where the owner stops the rounds after a round's repairs, the owner's stop prevails: tell the owner that those repairs are unreviewed, and the goals axis of the review before the merge, `agent@knowledge-architect-goal-reviewer` in its merge mode, reads them." Step 6 of `skill@knowledge-architect-goal-setting@whole-set-review` stops "by the rule of step 4 of `skill@knowledge-architect-goal-setting@drawing-out-intent`", so that sentence reaches the whole-set review by its pointer. W3 applies no longer as a gap: the session did what the text says. W1 applies: the text names no first round for a rewording that a review proposed.
- **c. Whether it is already recorded.** `design@agent-skills@goal-review-stops-on-old-wording` (conflicts, check-reversal) states the stop for "a goal review in rounds, in draft mode and over the whole goal set alike", and does not name a merge-mode report as a round. `design@agent-skills@goal-edit-ruled-alone` and `design@agent-skills@goals-axis-in-the-review-before-a-merge` (respect) keep the repair drafted as its own item and the merge mode's checks as they are. `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed` (touches) is on a neighbouring question, when a review's repair owes another axis, and is not this one. Nothing records the join.
- **d. Its kind.** W1: a defect of the text, unclear, read two ways by one session, and in part a lapse: step 4 owes a draft-mode round before the owner rules, and the session put defaults to the owner with none. W3: already stated, so no defect.
- **e. Its scope.** The installed text, `path@agent-skills@content/skills/goal-setting/SKILL.md`, owned by `agent-skills`; it serves `goal@the-owner-decides` (the owner rules each goal with its review read). The repair below states the reading step 4 already gives, so it reverses no head: `design@agent-skills@goal-review-stops-on-old-wording` names draft-mode and whole-set rounds only.
- **f. A better fix.** The nearest rival is W1's: count the merge-mode report as round one, so the stop can come at the first draft-mode round. The fact that decides: merge mode checks less than draft mode. Its definition, `path@agent-skills@content/agents/goal-reviewer.md` section "The modes", reads "Report each such goal that fails rules 1 to 3 or 5 to 8", with no owner's statements, no check (a) or (b) and no boundary block, while draft mode reads "every relevant statement of the owner, verbatim" and gives the boundary block. Counted as round one, it would let the stop sort as "old wording" words that no draft-mode reader had judged against rule 4 or the owner's statements. The second rival, a sentence in the review skill as W1 proposes, puts the rule in the text that only points; the act is taken in goal-setting, the home of the rounds. The fix goes where the act is delivered: the paragraph of `skill@knowledge-architect-goal-setting@drawing-out-intent` on "A goal's wording proposed in another activity".
- **g. Its route.** A text edit of the installed text at its source, as `path@agent-skills@CLAUDE.md` section "Editing an installed skill or agent" says, under `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** Handle now, in `path@agent-skills@content/skills/goal-setting/SKILL.md`, the paragraph after step 7 of `skill@knowledge-architect-goal-setting@drawing-out-intent`, line 141. Current text: "**A goal's wording proposed in another activity**, a plan's default, a review's repair, an audit's item or a goal a design discussion sets, is drafted here as its own item, with its boundary block, and the owner's word on a batch of other items never covers it." New text:

```markdown
**A goal's wording proposed in another activity**, a plan's default, a review's repair, an audit's
item or a goal a design discussion sets, is drafted here as its own item, with its boundary block,
and the owner's word on a batch of other items never covers it. The report that proposed it, a
merge-mode goals axis's included, is no round of step 4: the rounds start with the first
draft-mode dispatch on the new wording.
```

  Instructions added 0, removed 0: a rewording that makes step 4's intent explicit. Nearest rival: the merge-mode report counted as round one, which loses on the narrower checks of merge mode. W3 takes no change.

Default: handle now, the rewording above; W3 no change, already stated.
Outcome: decided by the session: handle now (W1); no change, already stated (W3). Ground: step 4 of `skill@knowledge-architect-goal-setting@drawing-out-intent` runs its rounds in draft mode, and the merge mode of `agent@knowledge-architect-goal-reviewer` reads neither the owner's statements nor a boundary block, so the rewording states the reading the text gives and changes no act; W3's sentence is in step 4, which step 6 of `skill@knowledge-architect-goal-setting@whole-set-review` points to.

## standing-2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow: the standing answers

Members: the section "Answers to the standing questions" of
2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow, lines 79 to 108. The file
predates the questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- five commit messages "false of their own trees", among them "returns only", a count of commits
  and "resolved once" (lines 93 to 100): `message-claims-written-unchecked`;
- the rewords of older messages by hand-written scripts six times (lines 103 to 105), which the file
  sends to its project file: `no-command-rewords-branch-messages`; the stale mutation records found
  by a script, C2: `stale-mutation-records-unfound`.

Analysed here:

- **An argument against a shape from a goal not read whole** (lines 85 to 92): the session relied
  "on a searcher's quotation and its own memory of the goal. The owner caught it ('Have you read
  that goal in full?')". Kind: a lapse against `primer@verify-before-relying` ("Names, doc comments
  and prose are leads to check, never evidence"). The file adds that the design of that discussion
  "now has the session act on a verdict only after reading the decision it names", a claim not
  checked here, since the outcome does not rest on it. One instance, caught by the owner.
- **Installed files changed** (lines 81 to 83): the source, as the work. **Project skill addition
  missed**: none. **Backticks**: none. **Code rules** (lines 106 to 108): a pure function split out
  of an I/O loop, "one test and no rework"; it bears on
  `tripwire@agent-skills@shape-findings-fragment-code` and does not fire it.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: the one lapse, against an instruction that
exists and is clear, was caught by the owner, and the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules` does not repair one instance by
text; the rest are observations.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
