# 2026-11-13-thaum-long-draw-klarch-workflow

- **Received file**: 2026-11-13-thaum-long-draw-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: "knowledge-architect 0.10.0, as the project pins it."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@no-routing-table`: constrains, respect.
  - `design@agent-skills@development-skill`: constrains, respect.
  - `design@agent-skills@intake-takes-the-project-file`: constrains, respect.
  - `design@agent-skills@received-file-marked`: constrains, respect.
  - `design@agent-skills@spec-first-axis`: constrains, respect.
  - `design@agent-skills@review-cadence`: constrains, respect.
  - `design@agent-skills@code-axes-rows-home`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@agent-skills@expectation-set-bounds-scope`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@agent-skills@admission-rules-home`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `design@agent-skills@work-divided-by-judgement`: conflicts, check-reversal.
  - `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`: touches, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.
  - `goal@session-orchestrates-the-work`: constrains, respect.
  - `design@agent-skills@needless-instruction`: constrains, respect.
  - `design@agent-skills@reviewer-returns-its-report-whole`: conflicts, check-reversal.
  - `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`: touches, respect.
  - `design@agent-skills@capabilities-not-structure`: constrains, respect.
  - `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies`: touches, respect.
  - `design@agent-skills@local-intent-binds`: constrains, respect.
  - `design@agent-skills@one-session-cost-weighed-before-removal`: constrains, respect.
  - `design@knowledge-architect@the-record-lives-in-the-repository`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.

## checkpoint-axes-unowned-when-dispatched: a dispatched implementer ran none of the development checkpoint's axes

Members: 2026-11-13-thaum-long-draw-klarch-workflow W3 (interaction; the project's side is P1 of that session's project file, read for grounding only).

- **a. What it says.** The session dispatched one implementer agent and told it to read the
  development skills. The agent ran its own mutation checks and dispatched no reviewer; its brief
  named no checkpoint axis. The adversarial axis, and an axis the project adds at a checkpoint, ran
  first at the review before the merge. There the adversarial reviewer found five claims that no
  test guarded, so one more repair round came after the merge review. The file blames the
  development skill, which puts the adversarial axis at each checkpoint, and states the gap as
  "Nothing says whether a dispatched implementer runs them, or the session that dispatched it." It
  proposes one sentence: the checkpoint's axes are dispatched by the session that holds the work's
  design, after a dispatched implementer returns (+1 instruction).
- **b. Whether it still applies.** The file's version is the released 0.10.0. The development skill,
  `path@agent-skills@content/skills/development/SKILL.md`, is the same at `v0.10.0` and at
  `v0.10.0-59-g605d1538` in the lines that bear: the loop's step 6, line 50 at both, reads "**Review**
  (`skill@knowledge-architect-development@code-review-axes`), at any checkpoint where a coherent
  piece works, not only at the end."; the axes table's adversarial row, line 148 at `v0.10.0` and
  line 169 on main, reads "at each checkpoint, for any change to the code". The only diff between the
  two versions adds the module-split section (`git diff v0.10.0 origin/main -- <path>`). No text names
  who dispatches the axes when the implementer is itself dispatched; the loop is addressed to whoever
  runs it.
- **c. Whether it is already recorded.** Yes, as a decision that answers the question the other way.
  `design@agent-skills@work-divided-by-judgement` (agent-9, `conflicts`, `check-reversal`) states, in
  its bullet on the primer as home: "A dispatched developer loads the development skill through the
  primer's list of installed skills, and sends that skill's checkpoint reviews itself." Its deciding
  fact is a harness measurement in the same head: "a general-purpose subagent holds `Agent` and can
  dispatch a nested agent", taken in Claude Code 2.1.321, "and the primer reaches a nested agent".
  `design@agent-skills@review-cadence` and `design@agent-skills@code-axes-rows-home` (agent-6,
  `constrains`, `respect`) keep the adversarial and spec-first axes at each checkpoint, as rows of the
  development skill's table. `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`
  (agent-9, `touches`) is about partitioning parallel work in a plan document; this was one
  implementer, so its trigger is not met. No issue or tripwire records a dispatched implementer that
  skipped its checkpoint review (`grep -rn work-divided-by-judgement` over every tripwires home
  returns nothing).
- **d. Its kind.** A lapse, of the implementer and of the brief that sent it. The instruction exists
  and is clear for whoever runs the development loop: step 6 is one step of the loop the brief told
  the agent to follow, and `primer@installed-skills` asks a skill to be followed whole. The primer's
  `primer@dispatching-work` says "A dispatched piece is what its brief gives the agent, and the agent
  carries all of it" (line 315 of the source, `path@agent-skills@content/PRIMER.md`; line 296 of the installed copy); a brief that gives the development loop gives its
  review step. The agent could dispatch: the file reports that it dispatched no reviewer, not that it
  could not. That the harness of that session held `Agent` in the subagent is an assumption, from the
  head's measurement in 2.1.321; the file does not name the harness version. This is one instance.
- **e. Its scope.** The installed text, the development skill, owned by `agent-skills`; it serves
  `goal@session-orchestrates-the-work` and `goal@the-workflow-leads-to-better-code`. The proposed
  sentence reverses the quoted sentence of `design@agent-skills@work-divided-by-judgement`, so it is
  a reversal under `skill@knowledge-architect-decision-recording@reversal-check`, and it adds an
  instruction on how an agent behaves, which `design@agent-skills@behaviour-additions-need-real-use`
  admits only with the owner naming what a session would have lacked. The project's own axis at a
  checkpoint is the project's, fixed in its own text (its P1), not here.
- **f. A better fix.** The proposal's rival is no change. The fact that decides: the head's argument
  stands. A subagent can dispatch, so the developer that holds the claims and the code is the one able
  to scope the adversarial reviewers by subsystem and claim, as
  `skill@knowledge-architect-development@code-review-axes` asks; moving the dispatch to the session
  after the implementer returns would put the checkpoint at the end of the piece, which step 6's "not
  only at the end" forbids, and would make the session read again the claims the implementer holds.
  The finding brings no argument against the head, only one instance of the loop's last step not
  run. By the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`, a lapse
  is not repaired by adding text; a recurrence at one point of delivery would move the delivery, for
  example to a sentence where a brief for an implementer is written, and is not shown by one instance.
  A second rival, a sentence in the primer telling a brief for an implementer to name the checkpoint
  axes, is a rule written for one interaction and fails the same test.
  The file's second deciding fact, that a fresh reviewer should not be anchored on the
  implementer's account, does not rank the two carriers: under either one the reviewer is a fresh
  agent briefed blind, whose brief "must not assert what the tree contains", per
  `skill@knowledge-architect-review@review-invariants`; and a session that dispatches after the
  implementer returns writes its brief from the implementer's report, the account the fact warns of.
- **g. Its route.** Nothing.
- **h. The proposal.** No change: a lapse, one instance, against a clear step of the development
  loop; the proposed sentence contradicts `design@agent-skills@work-divided-by-judgement` and brings
  no argument that defeats its harness fact. The head's own watch points cover a recurrence: the
  retrospective's standing question on an agent that came back unfinished. The session holds it.

Default: no change, a lapse.
Outcome: decided by the session: no change, with the reason: a lapse of a clear instruction, step 6
of `skill@knowledge-architect-development@development-loop`, read by the implementer that its brief
sent to that skill; the proposed carrier contradicts `design@agent-skills@work-divided-by-judgement`
("A dispatched developer ... sends that skill's checkpoint reviews itself"), whose measured fact that a
subagent holds `Agent` the file does not defeat; the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules` repairs a lapse by moving its
delivery only on a recurrence, and this is the first instance.

## reviewer-returns-whole-report: every reviewer returns its whole report as text beside its file

Members: 2026-11-13-thaum-long-draw-klarch-workflow W2.

- **a. What it says.** At a review before the merge of 12 axes, then a second decision-record review, then the transcript review, every reviewer wrote its report file and also returned the report whole, at roughly 1 000 to 3 000 words each. The session needed one line per finding to dispatch the repairs, and wrote those into an outcomes file pointing at the report files. The file blames `skill@knowledge-architect-review@review-invariants`, "asks the reviewer to write the report whole there with a shell redirect, … and to return the same report whole as text", and labels it needless, in part: the stated reason is that the text keeps the findings in the transcript the transcript reviewer reads, and in that session the transcript reviewer read the report files and the outcomes file as well as the transcript. It proposes that a reviewer return its verdict line, its finding count and the report's path, and that the transcript reviewer's brief list the report files (1 reworded).
- **b. Whether it still applies.** Yes. The file used 0.10.0. The bullet is the same at v0.10.0 (line 111) and at origin/main (line 113) of `path@agent-skills@content/skills/review/SKILL.md` (`git show v0.10.0:crates/agent-skills/content/skills/review/SKILL.md`): "and to return the same report whole as text. ... the file is the copy the record of the review is built from, since a compaction of the conversation keeps no report, and the text returned keeps the findings in the transcript the transcript reviewer reads." Its source comment names the head: "its return as text `design@agent-skills@reviewer-returns-its-report-whole`".
- **c. Whether it is already recorded.** The proposal is the rival `design@agent-skills@reviewer-returns-its-report-whole` defeated (verdict conflicts, check-reversal): "The nearest rival, a reviewer that returns only the path, keeps its findings out of the transcript the last review reads." The head's ground is that the transcript reviewer finds the other axes' findings as messages of the transcript, per `design@agent-skills@transcript-review-last-before-merge`, "and its extraction drops a tool result, such as the dispatcher's reading of a file, unless asked for it". The file's proposal adds what the defeated rival lacked: the transcript reviewer's brief names the files. That is the second closing shape of `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript` (verdict touches): "or the transcript reviewer's brief names the report files and its definition reads them; chosen under `skill@knowledge-architect-design`". That issue's own premise for keeping the transcript copy, files "which a later session may not have kept", is the fact still open between the two shapes.
- **d. Its kind.** A defect of the text, needless in part, on one session's cost. The file also brings a new argument the head does not weigh: the context the returned reports fill in the dispatching session. `goal@session-orchestrates-the-work` bears on that cost but fixes no size: "This goal does not say when a dispatch happens, nor for which kind or size of work", so the head does not conflict with the goal.
- **e. Its scope.** The installed review skill, the agent-skills Component. The fix reverses `design@agent-skills@reviewer-returns-its-report-whole`, so it goes through `skill@knowledge-architect-decision-recording@reversal-check` and is decided under `skill@knowledge-architect-design`.
- **f. A better fix.** The nearest rival is to keep the head and only add the report files to the transcript reviewer's brief, so the transcript reviewer reads both. What decides between them is whether the transcript reviewer can rely on the files, which it cannot when a later session no longer holds them; that question is the open one of the workflow issue, and one fix answers both: once the transcript reviewer's definition reads the report files, a review run by a workflow and a review run by the session reach it the same way, and the return of the whole text loses its reason. A second rival, a short return like the record search's summary, is this proposal under another name. The admission standard asks a needless instruction on one session's cost to be weighed against its other occasions (`design@agent-skills@needless-instruction`, verdict constrains: "A needless item is the evidence of one task at one moment"); the occasion that keeps it, a review with no files left, is that open question.
- **g. Its route.** A decision, whose design is not settled: `skill@knowledge-architect-design`, inside the recorded issue.
- **h. The proposal.** Add the instance to `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`, whose second closing shape would let a reviewer return less. No edit now. The session holds it: the record settles the route (a reversal goes through the design skill), and no fork is put to the owner before that discussion.

Default: an instance added to `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`.
Outcome: decided by the session: open an issue, as an instance added to `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`. Ground: the proposal reverses `design@agent-skills@reviewer-returns-its-report-whole` on a new argument (the context cost) and one instance of a new fact; a reversal is decided under `skill@knowledge-architect-design`, and the recorded issue already holds the closing shape this proposal needs.

Text to add to the issue, as a paragraph at the end of its "What" section:

```
A second instance, from the other side: at a review before the merge of 12 axes in a project that
uses the workflow, every reviewer returned its whole report as text beside its file, at roughly
1 000 to 3 000 words each, and the dispatching session needed one line per finding; the transcript
reviewer read the report files as well as the transcript (W2 of
2026-11-13-thaum-long-draw-klarch-workflow). Its proposal, that a reviewer return its verdict line,
its finding count and the report's path while the transcript reviewer's brief lists the report
files, is the rival `design@agent-skills@reviewer-returns-its-report-whole` defeated, with this
issue's second closing shape added: closing the issue by that shape would also decide whether a
reviewer still returns its report whole, and weigh the context the returned reports fill.
```

## standing-2026-11-13-thaum-long-draw-klarch-workflow: the standing answers

Members: the section "Standing questions" of 2026-11-13-thaum-long-draw-klarch-workflow, lines 94
to 144, of a project that installs the workflow, pinned at 0.10.0. Its counts are in the lines for
`issue@agent-skills@the-review-cost-counts-are-not-compared-yet`, its missed heads added after the
search in the lines for `issue@agent-skills@a-record-search-misses-what-is-added-after-it`, and its
tool reports are item 4 of the owner item in the analysis of
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.

Carried by a subject, named only:

- a premortem cause dropped on an unchecked claim, "exhaustive enums make a driver that ignores a
  new halt fail to compile", where seven test files compare with `!=` (lines 98 to 101):
  `message-claims-written-unchecked`;
- a repair agent changing what a head states about unbuilt work, with no argument (lines 102 and
  103): `head-changed-without-design-skill`;
- the project's rules-first axis at a development checkpoint missed (lines 108 and 109): W3 of this
  file, `checkpoint-axes-unowned-when-dispatched`;
- the searchers' quotation repairs and the branch's message rewrites (lines 111 and 112):
  `quotation-check-refuses-cut-punctuation` and `no-command-rewords-branch-messages`;
- every reviewer's whole report read as it arrived (lines 115 and 116): W2 of this file,
  `reviewer-returns-whole-report`;
- the two design heads graded against a draft that did not yet say how the built code writes (lines
  125 to 128): `thread-shape-change-owes-no-search`, and an instance in the lines above.

Analysed here:

- **The implementer placed a second decision inside an existing head; three reviewers caught it**
  (lines 104 and 105). Kind: a lapse against one decision per head,
  `skill@knowledge-architect-decision-recording@head-shape`, caught by the review.
- **A repair agent amended a message while other changes were uncommitted; it reported this itself,
  and no content was lost** (lines 106 and 107). Kind: a lapse against the project's own git
  instruction; no installed text.
- **A tripwire whose horizon counts commits to one file, graded `unrelated`; the standing-state
  reviewer found the branch's commit reached the horizon** (lines 129 to 131). Kind: no miss of the
  search. A horizon reached is no bearing of the work on the tripwire's clause, and reading every
  horizon against the change is the standing-state review's subject, per
  `design@agent-skills@standing-state-review-before-every-merge`, which found it. The file does not
  say it was added after the search, so it is not added to the issue on misses.
- **A conclusion verified** (lines 120 to 123): the prior-art survey's two key claims checked
  against the cloned sources; a code-claims reviewer later dropped the same claim as unverifiable in
  the tree, "so the provenance was written into the head". An observation of
  `primer@verify-before-relying` at work across a claim taken from another project.
- **Installed file changed**: no. **Backticks**: no. **Code rules** (lines 113 and 114): the split
  of two files above the size limit "was in the change's interest"; it bears on
  `tripwire@agent-skills@shape-findings-fragment-code` and does not fire it. **Agents unfinished**:
  no; "The first repair agent left two drivers untested and opened an issue for them, and said so."
  Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: two lapses caught by review or reported by the
agent, one against the project's own instruction; the horizon found by the standing-state review is
that review's subject, not a miss of the search; the rest are observations.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## in-change-path-taken-for-unstarted-work: the in-change path's example "as an issue fix" is taken for requested work not yet started

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `in-change-path-taken-for-unstarted-work`, which gives the outcome.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `thread-shape-change-owes-no-search`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## head-changed-without-design-skill: code or an agent's edit contradicted, narrowed or rewrote a head with no deliberation

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage-klarch-workflow.md`, in its section on `head-changed-without-design-skill`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
