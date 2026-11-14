# 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow

- **Received file**: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.9.0-354-gb97596f1` when the work began, on the branch `agents-write-their-reports`, the tree clean."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@query@worklist-writes-and-verifies-the-reports`: conflicts, check-reversal.
  - `design@query@occasion-shaped-output`: conflicts, check-reversal.
  - `design@agent-skills@met-outside-the-task-outcomes`: constrains, respect.
  - `design@agent-skills@decisions-reported-to-the-owner`: constrains, respect.
  - `rejected@agent-skills@finding-reported-not-recorded`: constrains, respect.
  - `design@agent-skills@session-acts-on-the-verdicts`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@the-owner-decides`: constrains, respect.
  - `goal@session-orchestrates-the-work`: constrains, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `design@agent-skills@in-change-path`: constrains, respect.
  - `design@agent-skills@design-hands-off-to-planning`: constrains, respect.
  - `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`: touches, respect.
  - `design@agent-skills@search-reads-the-draft`: conflicts, check-reversal.
  - `design@agent-skills@bounded-path-in-design`: constrains, respect.
  - `design@agent-skills@outcome-over-display`: constrains, respect.
  - `design@agent-skills@instruction-record-is-minimal`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@one-repair-per-finding`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `goal@design-with-agents-as-counterparts`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.
  - `design@agent-skills@premortem-as-watch-points`: constrains, respect.
  - `design@agent-skills@search-runs-once-the-shapes-settle`: conflicts, check-reversal.
  - `design@agent-skills@no-thread-closes-before-the-search`: constrains, respect.
  - `design@agent-skills@record-searched-before-the-work`: conflicts, check-reversal.
  - `rejected@agent-skills@search-before-and-after-the-challenge`: constrains, respect.
  - `issue@agent-skills@a-record-search-misses-what-is-added-after-it`: touches, respect.
  - `goal@records-reach-their-reader`: constrains, respect.

## closing-sections-missed-in-cut-returns: no command gathers the searchers' closing sections, and a dispatcher missed sixteen

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow W3, C1.

- **a. What it says.** In the second search of that session, the 18 searchers' returns reached the session cut short by the harness after the first two. The session ran `--verify`, read the generated lines and the verdict files, and never read the closing sections of reports 3 to 18. Two items in them bore on the work (a path question, and a defect of the installer); the transcript reviewer found both. The blamed text is the record searcher's description: "Read the verdict files yourself ... and carry each report's `## Decided` and `## Met outside the task` into your own sections." The file classes W3 as wrong in effect, the act being a task with no judgement that only the report files hold. The fix (C1): a passing `--verify` also writes one file listing each report's two closing sections, by agent, which the description then names.
- **b. Whether it still applies.** Yes. The description at the file's version, b97596f1 (`v0.9.0-354`), and at origin/main, in `path@agent-skills@content/agents/record-searcher.md`, line 3, holds the blamed sentence unchanged. At origin/main `--verify` judges that each report holds both closing sections (`judge_report` in `path@query@src/worklist.rs`, near line 1203, "no section `{section}`: close the report with"), and writes no file of their content: `design@query@occasion-shaped-output` lists what a passing run writes, the verdict files and `generated-lines.md`, or at an audit `gaps.md` and `fit-lines.md`.
- **c. Whether it is already recorded.** No issue records it. The heads that bear: `design@agent-skills@session-acts-on-the-verdicts` (`constrains`, `respect`), "and carries each report's `## Decided` and `## Met outside the task` into its own sections", which any fix keeps; `design@query@occasion-shaped-output` (`conflicts`, `check-reversal`), whose list of outputs a new file would widen; `design@query@worklist-writes-and-verifies-the-reports` (`conflicts`, `check-reversal`) for what `--verify` writes. `goal@tasks-a-tool-does-better-are-the-tool-s` carries an owner-question on taking in each recurring task; the record search is a task the record already gives `worklist`, and the closing sections are part of each report it judges, so gathering them extends that tool and takes in no new task (a reading of the goal, listed under Decided). The outcome does not rest on that reading: opening the issue takes in no task, and the issue's closing condition keeps the owner's ruling that the reading stays by hand, so the design discussion that takes the issue up puts the owner-question if the reading is contested.
- **d. Its kind.** A defect of the text and the checker in interaction: the instruction is clear, and the session did not follow it for 16 reports, so W3 alone is a lapse; but the content is held only by 18 files, which the session reads by hand or by a script (done here by a script after the review), and the command that reads every report writes none of it. One received file reports it. A second instance is this intake's own: its main session reports, in its ledger, that it gathered the eighteen searchers' closing sections by reading each report as a message, which missed none and cost the reading by hand that C1 would replace (the main session's report, not checked here).
- **e. Its scope.** The checker, Component `query`, and the installed searcher description, Component `agent-skills`. The fix widens `design@query@occasion-shaped-output`, so it goes through `skill@knowledge-architect-decision-recording@reversal-check` and the design skill.
- **f. A better fix.** Rivals: (1) the file of C1, written by a passing `--verify`; (2) the closing sections appended to an existing output, such as the head of `generated-lines.md`; (3) no change, the dispatcher reading each `report.md`'s two sections, which the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules` gives for a clear instruction not followed. What decides between (1) and (3) is whether the lapse recurs at this point of delivery, since the harness cuts returns whenever many agents return together (assumption: two instances, the received file's and this intake's, only the first of them a miss); the review skill solved the same loss for reviewers by a report file each writes, `skill@knowledge-architect-review@review-invariants`. The shape is a design question with more than one defensible answer, and needs a branch of its own.
- **g. Its route.** An issue; its work goes to `skill@knowledge-architect-design`, then `skill@klarch-development`.
- **h. The proposal.** Open an issue in the `query` anchor, kind `todo`, slug `issue@query@no-command-gathers-the-closing-sections-of-the-searchers-reports`, stating the two instances, the two items missed in the first, the three rivals, and what would close it: a shape argued under the design skill that puts every report's `## Decided` and `## Met outside the task` before the dispatching session, or the owner's ruling that the reading stays by hand.

Default: open an issue.
Outcome: decided by the session: open an issue, anchor `query`, kind `todo`, slug `issue@query@no-command-gathers-the-closing-sections-of-the-searchers-reports`. Ground: the fix the file proposes widens `design@query@occasion-shaped-output`, which goes to the design skill, and the intake's branch carries no design discussion; the act it serves is `design@agent-skills@session-acts-on-the-verdicts`.

## in-change-path-taken-for-unstarted-work: the in-change path's example "as an issue fix" is taken for requested work not yet started

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow W1; 2026-11-13-thaum-long-draw-klarch-workflow W1.

- **a. What it says.** Two sessions took the in-change path for a requested change whose work had
  not started: in searcher-check W1, "Both discussions were requested changes, not bounded (each had a fork
  the owner ruled), and neither change had started", and the session "stayed on the in-change path
  without ever applying the test"; in long-draw W1, "No change was under way when the discussion
  opened", the session "took the in-change path", and "The deliberation went into a commit message
  of more than 300 lines", which no reviewer questioned. The blamed
  text, `skill@knowledge-architect-design@decision-authority`: "**The in-change path.** It is open
  when the decision lands in the change under way: its work, where that change began before the
  discussion opened, as an issue fix or a review repair does, ...", beside the full path's "(it needs
  a branch of its own, several pull requests, or nobody has started it)". long-draw names the cause:
  a restatement inside one text reading against its own condition, the example naming "an issue fix"
  while the condition excludes one not started. Fixes: reword the example to "as an issue fix or a
  review repair already under way does" (long-draw); loop step 1 applies the path test when the
  shapes settle, before the draft is written (searcher-check, a rewording).
- **b. Whether it still applies.** Yes. The sentence reads so at `b97596f1` (searcher-check's
  version), at `v0.10.0` (long-draw's pin) and at origin/main, `v0.10.0-59-g605d1538`, line 151 of
  `path@agent-skills@content/skills/design/SKILL.md`; the same words stand in the head,
  `design@agent-skills@in-change-path`, line 185 of `path@agent-skills@docs/design/plan-documents.md`.
- **c. Whether it is already recorded.** The decision is recorded and agrees with the full path:
  `design@agent-skills@in-change-path` says "A requested change that is not bounded, and whose work
  has not started when its discussion opens, takes the full path too, even when one pull request
  builds it." No entry records the misreading. searcher-check's rival conflicts with
  `design@agent-skills@search-reads-the-draft` ("in the form of the path the session expects"),
  which it would rewrite. `issue@agent-skills@installed-text-assumes-pull-requests-and-ci` (touches)
  names the full path's "several pull requests"; the rewording leaves that clause as it is.
- **d. Its kind.** Two members, two causes. long-draw W1 is a defect of the text: unclear, its
  example read against its own condition ("The session read 'as an issue fix ... does' as covering
  the case"). searcher-check W1 reports no reading of the example: the session "stayed on the
  in-change path without ever applying the test", and the file labels it "A lapse, by the admission
  standard, unless it recurs". The moment of the test is in the text, loop step 1 of
  `path@agent-skills@content/skills/design/SKILL.md`, line 501: "The classification after the
  search still decides the path." long-draw's session misread the example rather than skipping the
  test, so it is no recurrence of that lapse at its point of delivery; whether the example also led
  searcher-check's session is not stated by its file.
- **e. Its scope.** The installed design skill and the head that states the same sentence, both in
  `agent-skills`. The rewording reverses nothing: it brings the example to the condition the head
  already states.
- **f. A better fix.** The cause long-draw shows is the example's wording at the site where the
  path is chosen, so the fix is that wording. Rivals:
  - searcher-check's move of the path test into loop step 1, before the draft: it rewrites
    `design@agent-skills@search-reads-the-draft` and moves an instruction for a lapse seen once,
    where the step already names the moment of the test, after the search; a lapse is not repaired
    by adding text, and the move is owed only where the lapse recurs at one point of delivery, per
    `skill@knowledge-architect-retrospective-intake@admission-rules`. It is not proposed.
  - deleting the example: it removes the misread words, but also the two common cases a reader
    recognises the path by; the rewording keeps them and closes the misreading.
- **g. Its route.** A fix in the installed text, at its source under `path@agent-skills@content/`,
  as `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says; the same
  words in the head are rewritten in place under `skill@knowledge-architect-decision-recording`, a
  rewording that changes no decision.
- **h. The proposal.** Handle now. In `path@agent-skills@content/skills/design/SKILL.md`, the
  in-change path's first sentence becomes:

  ```text
  - **The in-change path.** It is open when the decision lands in the
    change under way: its work, where that change began before the
    discussion opened, as an issue fix or a review repair already under
    way does, or its
  ```

  and in `design@agent-skills@in-change-path`, "as an issue fix or a review repair does" becomes "as
  an issue fix or a review repair already under way does". Instructions added: 0; removed: 0; one
  reworded at each site. Nearest rival: the path test applied at loop step 1, which loses as item f
  says. The argument goes in the commit message, per `design@agent-skills@instruction-record-is-minimal`.

Default: handle now, with the rewording above.
Outcome: decided by the session: handle now, with the rewording above; searcher-check W1 is a lapse
once, no change. Ground: long-draw's session read the example against its condition;
`design@agent-skills@in-change-path` already decides that unstarted requested work takes the full
path, so the rewording changes no decision and adds no instruction; the step names the moment of
the test that searcher-check's session skipped.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow W2; 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow W1; 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow W2; 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow W4(b); the standing answers on a missed entry of 2026-11-13-thaum-long-draw-klarch-workflow, 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow and 2026-11-14-thaum-casting-process-design-klarch-workflow.

- **a. What it says.** Seven sessions report an entry the work bore on that the search had graded
  against an earlier shape, found only by a review: a thread widened to new sites (searcher-check
  W2); a pick at a fork that replaced the draft's shape, a `constrains` verdict not meeting the picked
  shape (new-crate W1); a decided thread whose shape moved, a command moved behind a newly exported
  library function that a rejected design argued against (transcript-reader-gaps W2); a thread's new
  position, an issue's re-entry met (split-guidelines W4(b)); two heads graded `constrains` against a
  draft that did not yet hold the part needing an exception in each, and a tripwire whose horizon
  counts commits to one file (long-draw); a head and its rejected rival bearing on a sample the audit
  added after the search (conformance-before-harvest); a head contradicting a thread opened after the
  second search, judged covered by it (casting). The blamed text, loop step 1 of
  `skill@knowledge-architect-design@discussion-loop`: "When a thread opens, after the search, on a
  subject its work did not cover, run a search of its own, with the draft extended by that thread as
  the work and the thread as its subject". Rewordings proposed: "When the work gains a subject or a
  site its draft did not hold, a new thread or a widened one, run a search of its own"
  (searcher-check W2); "When the draft's shape changes after the search, by a new thread or by a pick
  at a fork" (new-crate W1); "a thread opens, or a decided thread's shape changes, after the search"
  (transcript-reader-gaps W2); none beyond the issue (split-guidelines, and the three standing
  answers).
- **b. Whether it still applies.** Yes: the sentence reads so at origin/main,
  `v0.10.0-59-g605d1538`, in `path@agent-skills@content/skills/design/SKILL.md`, and at
  `b97596f1`, the earliest member's version.
- **c. Whether it is already recorded.** Yes: `issue@agent-skills@a-record-search-misses-what-is-added-after-it`
  (kind `design`), whose suspected mechanism is "a search judges the draft it is given, so a thread
  or a fix added to the work after it ran is judged by no search unless the session runs one", and
  whose second instance is searcher-check W2 itself (the in-change discussion that gave the record
  searcher `worklist --check`). Its re-entry: "The next design discussion whose own work includes
  reopening `design@agent-skills@one-record-searcher`, which the owner schedules"; on the owner's word
  the reopening was kept out of the last milestone. The rewordings widen
  `design@agent-skills@search-runs-once-the-shapes-settle` ("A thread that opens after the search on
  a subject its work did not cover gets a search of its own") and
  `design@agent-skills@record-searched-before-the-work`, each graded `conflicts`; their cost is
  weighed by `rejected@agent-skills@search-before-and-after-the-challenge` ("doubles the cost of the
  search"). `skill@klarch-retrospective-intake@received-analysis` adds a missed entry an answer
  reports as bearing on a part added after the search to that issue, in the commit that writes the
  analysis.
- **d. Its kind.** A defect of the text, missing: the rule reaches a new thread, not a changed shape;
  recurring in seven sessions, six beyond the issue's recorded instances.
- **e. Its scope.** The installed design skill, and the two heads above; the fix is the decision the
  issue's discussion holds.
- **f. A better fix.** A rewording now would widen two heads by a rule whose cost the record
  measures against it, ahead of the discussion the issue schedules, which weighs every cause the
  tripwire's response named at once. The six new instances go to the issue as that discussion's
  evidence.
- **h. The proposal.** Four of the seven members are already carried to the issue by the
  block of instances this intake writes for the standing answers on a missed entry:
  split-guidelines W4(b) and the three standing answers (long-draw, conformance-before-harvest,
  casting). That block leaves out long-draw's second item, the tripwire whose horizon counts commits
  to one file, which the file does not state as added after the search. searcher-check W2 is the
  issue's second instance already. Two instances remain for this section, new-crate W1 and
  transcript-reader-gaps W2, both a decided shape that changed with no new thread; they are added
  after the standing block, in the commit that writes the analysis, with the three rewordings the
  files propose as evidence for the discussion:

  ```text
  Two more, in the same files, each a decided shape that changed after the search with no new thread, so that no rule asked for a search:

  - W1 of 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow: the search judged a draft that carried the session's default at a fork; the owner picked the other shape, and no second search ran. A `constrains` verdict, on the order of what cannot be undone, did not meet the picked shape; the design-conformance review before the merge found the conflict.
  - W2 of 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow: the session moved a command, graded `conflicts` against a placement head, to the maintenance tool within the same thread, backed by a newly exported library function. A rejected design that argues against a function exported for one caller had been graded `unrelated` against the earlier shape. The design-conformance review found it before the merge, and the export was reversed.

  The rewordings of loop step 1 the files propose: a search whenever the work gains a subject or a site its draft did not hold, by a new thread or a widened one (W2 of 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow, the second instance above); whenever the draft's shape changes, by a new thread or a pick at a fork (new-crate W1); and when a thread opens, or a decided thread's shape changes, after the search (transcript-reader-gaps W2).
  ```

Default: no change to the text; the two instances above added to the recorded issue, after the
standing analysis's four.
Outcome: decided by the session: no change now; two instances added to
`issue@agent-skills@a-record-search-misses-what-is-added-after-it`, the four others by the standing
analysis. Ground: the issue holds the defect and its re-entry, scheduled by the owner;
`skill@klarch-retrospective-intake@received-analysis` routes such instances there; the rewordings
widen two heads ahead of that discussion.

## standing-2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow, lines 113 to 130.
The file predates the questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- "five of the twelve sessions" written into a head where the source gives seven, and a message
  saying an axis was not owed where it was (lines 117 to 120): `message-claims-written-unchecked`;
- searchers' quotations checked "with their own looser scripts, against their definition's
  'Reproduce what you report'" (lines 120 and 121), and the searchers filling their reports by
  scripts (line 127): `searcher-fills-report-by-script` and `quotation-check-refuses-cut-punctuation`;
- the one-agent copy of the work list built so 12 searchers could check their own reports "before
  `--check` existed" (lines 124 and 125): `searcher-cannot-check-own-report`; the 18 reports' closing
  sections read by a script (line 126): `closing-sections-missed-in-cut-returns`; two rewords with
  `git filter-branch` (lines 126 and 127): `no-command-rewords-branch-messages`.

Analysed here:

- **Installed file changed** (lines 115 and 116): the agents, the review skill, the record searcher
  and the three saved workflows, at their source, as the work. **Project skill addition missed**:
  no. **Backticks**: no. **Code rules** (lines 128 to 130): "about thirty mutation runs and two
  refactors that made a decision testable; the reviews found survivors the author's own mutations
  had not." Observations: the rules held, at the cost the file counts.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: every answer not carried by a subject is an
observation of a rule working as written.

## searcher-fills-report-by-script: searchers still fill their reports by scripts of their own

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.md`, in its section on `searcher-fills-report-by-script`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## transcript-unlabelled-lines: `transcript` counts lines it leaves unlabelled, without locating them, and asks a finding for each

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow.md`, in its section on `transcript-unlabelled-lines`, which gives the outcome.

## report-file-write-refused: the harness refuses a subagent's write of a report file by its name

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow.md`, in its section on `report-file-write-refused`, which gives the outcome.

## review-in-workflow-loses-transcript: a review run by a workflow leaves no report in the transcript

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports.md`, in its section on `review-in-workflow-loses-transcript`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
