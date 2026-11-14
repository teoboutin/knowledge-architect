# 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow

- **Received file**: 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, at main `v0.9.0-360-gc9cf649c`, branch `transcript-reader-gaps`, tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@mutate@mutation-command`: conflicts, check-reversal.
  - `design@mutate@one-file-per-run`: constrains, respect.
  - `design@mutate@one-command-owns-the-interval`: constrains, respect.
  - `rejected@mutate@sweep-shipped`: brings-back, new-argument-needed.
  - `design@mutate@report-not-gate`: constrains, respect.
  - `design@agent-skills@test-economy`: constrains, respect.
  - `issue@agent-skills@the-upstream-set-omits-the-published-libraries`: touches, none.
  - `design@knowledge-architect@mutation-command-host`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@agent-skills@admission-rules-home`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `design@agent-skills@tripwire-recording-authority`: constrains, respect.
  - `design@agent-skills@tripwire-admission-test`: conflicts, check-reversal.
  - `tripwire@agent-skills@silent-read-too-widely`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.

## mutate-has-no-batch-mode: the hand mutation runs one mutation per invocation, so a claim list is a shell loop each session writes

Members: 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow C1.

- **a. What it says.** The session ran 36 mutations, each an invocation of `cargo x mutate run` with
  its own build probe and test command, wrapped in a shell function and a loop, and filtered the
  output for the verdict lines; one filter matched only the `baseline:` lines, hid every verdict of
  a batch of eleven, and the batch ran again. Blamed:
  `skill@knowledge-architect-development@discriminating-tests`, each claim's test "is shown to fail
  against a plausible wrong implementation". Proposed fix: a batch mode, a file of mutations each
  naming its replacement, build probe and test, printing one verdict line per mutation.
- **b. Whether it still applies.** Yes. At origin/main `design@mutate@mutation-command` says "A run
  takes one mutation", and `run` takes one `--replace` or one `--patch` (`path@mutate@README.md`,
  "The two input forms").
- **c. Whether it is already recorded.** As a decision with a reopening point.
  `design@mutate@mutation-command` (verdict `conflicts`, `check-reversal`, with an owner-question):
  "A run takes one mutation: the owner chose that over a recorded list of mutations, the sweep
  generalised, which no goal ranks, and which waits for a second real use."
  `rejected@mutate@sweep-shipped` (verdict `brings-back`, `new-argument-needed`): "A generic sweep
  would need each project to declare its sites and its mutation, a recorded list that no second
  project has asked for." No issue records a batch.
  - On the owner-question, whether this loop is the "second real use": the head's choice is a fork's
    choice in the weak form, which "reopens without the owner on an argument or a fact that ranks
    the shapes", per `primer@design-heads`, and the owner named the fact it waits for. The first
    real use was thaum's sweep. This loop is a second: a list of 36 mutations, each written and
    named by its caller, run in one session of this repository, a second project. So the reopening
    is the session's to open, and the discussion it opens decides, going to the owner where it ends
    at a fork again. The searchers' question asks the owner to rule on the reading of a condition
    the owner already stated, which the facts above settle.
  - The new argument `rejected@mutate@sweep-shipped` needs: a batch of caller-written mutations is
    no sweep, since the tool would still generate and enumerate none, per the title of
    `design@mutate@mutation-command`; and the "second project" that entry says has not asked is this
    repository, whose session asked.
  - `design@mutate@one-file-per-run` and `design@mutate@one-command-owns-the-interval` (verdicts
    `constrains`) stand under a batch: each mutation still names one file, and each apply, run and
    restore stays inside the command.
- **d. Its kind.** A missing capability of the checker's tools, in one session: the loop and the
  filter that hid eleven verdicts are the cost.
- **e. Its scope.** `goal@tasks-a-tool-does-better-are-the-tool-s`, for a task the head already took
  in, and `goal@the-workflow-leads-to-better-code`. The owner of the fix is `mutate`. The fix
  rewrites `design@mutate@mutation-command`, and argues `rejected@mutate@sweep-shipped` again, so it
  goes through `skill@knowledge-architect-design`.
- **f. A better fix.** The rival is no change, the loop kept a session's own: one session in this
  batch met it, and about thirty runs in another (searcher-check-reviewer-reports, its standing
  answer on code rules) are reported with no loop named. The deciding fact is the head's own
  reopening condition, now met; whether a batch earns its cost is the discussion's to weigh, not the
  intake's.
- **g. Its route.** An issue only: a design discussion and a branch of `mutate`.
- **h. The proposal.** Open a design issue in `mutate`,
  `a-list-of-mutations-runs-as-a-loop-of-the-session`, naming the head's reopening condition as met,
  the new argument for the rejected design, and the filter that hid eleven verdicts as the cost the
  file reports.

Default: open the issue.
Outcome: decided by the session: open an issue, `mutate`, kind `design`, `a-list-of-mutations-runs-as-a-loop-of-the-session`. Ground: `design@mutate@mutation-command` waits for a second real use, which this session's list of 36 caller-written mutations is; a weak-form choice reopens on that fact without the owner, per `primer@design-heads`, and the discussion goes to the owner if it ends at a fork.

## silent-condition-read-alone: condition 1 of the tripwire standard read without the paragraph on slips

Members: 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow W2. One session; a second instance stands in the answers to the standing questions of 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow, below.

- **a. What it says.** The session dropped a premortem cause on condition 1, "The failure is silent. Every procedure that meets it counts it as success", because the new instruction has the harvest name each instance in its commit and a review report it. Two reviewers showed it silent: each named instance is the procedure working, and a recurrence across plans is reported by nothing. The paragraph "A slip is no firing" holds the answer. The session read condition 1 alone, and wrote the cause as a tripwire after the review. A second session reports the same slip among its standing answers, under `primer@verify-before-relying`: "premortem causes were dropped under a condition that did not exclude them, and found by a reviewer", repaired before the merge; that file does not name the condition. Proposal: condition 1 gains a clause pointing to the slip paragraph, "a failure reported one instance at a time is silent when it recurs across pieces of work" (+1); rival: nothing, leaving it to the review.
- **b. Whether it still applies.** Unchanged: `path@agent-skills@fragments/tripwire-standard.md` line 8 for condition 1 and line 33 for "**A slip is no firing.**", in the same fragment, both at `2abdaa89` (`grep -c` gives 1 for each). The slip paragraph reads: "It fires on the same failure recurring in separate pieces of work, or on one instance that reached the main branch with no procedure reporting it."
- **c. Whether it is already recorded.** `tripwire@agent-skills@silent-read-too-widely` (constrains) watches exactly this reading of "silent", and has not fired: it fires when "an issue is opened on a silent failure of a decision" and a message lists its cause dropped under condition 1, and here the review caught the drop before the merge, in both sessions, and no such issue exists. `design@agent-skills@tripwire-admission-test` (conflicts, check-reversal) records the standard; the clause would move the recurrence rule into condition 1. `design@agent-skills@behaviour-additions-need-real-use` (respect) and `goal@only-needed-instructions` (respect) ask more than one instance caught by its review before an instruction on how agents read is added.
- **d. Its kind.** A lapse: both statements are in one fragment of 67 lines, read whole when loaded, and the answer follows from them. Two instances in two sessions, each caught by its review before the merge. The recurrence does not move the fix, per the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`: the instruction is read and needed at one moment, the premortem's grading of each cause, in one text.
- **e. Its scope.** The installed text, `agent-skills`.
- **f. A better fix.** No change. A lapse is not repaired by adding text, per `skill@knowledge-architect-retrospective-intake@admission-rules`, and the recorded tripwire is the instrument that watches whether this reading recurs and reaches the main branch.
- **g. Its route.** Nothing.

Default: no change.
Outcome: decided by the session: no change, a lapse. Ground: condition 1 and "A slip is no firing" are in one text read whole, read and needed at the same moment; two instances, each caught by its review; `tripwire@agent-skills@silent-read-too-widely` watches the recurrence and did not fire.

## standing-2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow, lines 48 to 62. The file
predates the questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- a version range in a head written before it was measured, and a version range and a search result
  in messages from memory (lines 52 and 53): `message-claims-written-unchecked`;
- the hand mutations (C1), the searchers' report filling (C2) and four `filter-branch` rewords
  (lines 59 to 61): `mutate-has-no-batch-mode`, `searcher-fills-report-by-script` and
  `no-command-rewords-branch-messages`;
- the touched-head rule joined with the re-review of a repair (line 62): W1 of this file,
  `head-repair-rereview-unbounded`.

Analysed here:

- **Premortem causes dropped under a condition that did not exclude them, found by a reviewer**
  (line 54). Kind: a lapse in applying the tripwire standard of
  `skill@knowledge-architect-decision-recording@ruled-tripwires`, caught by its review. The batch
  has a second, W2 of 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow,
  where condition 1 was read alone: the subject `silent-condition-read-alone` weighs the reading of
  that condition, and this instance is evidence for its count. Whether either fires
  `tripwire@agent-skills@silent-read-too-widely` is weighed under `passed-horizon-left-standing`.
- **A finding asked for a pure function holding a single boolean, tested apart from an integrated
  test that already discriminates; judged to need nothing, "as the default shape's rule on
  fragmentation allows"** (line 62). Kind: an observation of the fragmentation clause working; it
  bears on `tripwire@agent-skills@shape-findings-fragment-code` and does not fire it.
- **Installed file changed**: the sources of the review skill and the transcript reviewer, as the
  work. **Project skill addition missed**: no. **Backticks**: no. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: one lapse caught by its review, counted under
`silent-condition-read-alone` where the reading of the condition is weighed, and observations.

## searcher-build-broken-by-live-edit: the dispatcher's edit of the Rust source broke the searchers' `cargo klarch` builds

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-transcript-reader-gaps.md`, in its section on `searcher-build-broken-by-live-edit`, which gives the outcome.

## searcher-fills-report-by-script: searchers still fill their reports by scripts of their own

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.md`, in its section on `searcher-fills-report-by-script`, which gives the outcome.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `thread-shape-change-owes-no-search`, which gives the outcome.

## whole-record-search-for-narrow-work: a search of the whole record runs for a narrow or follow-up piece of work

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow.md`, in its section on `whole-record-search-for-narrow-work`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## head-repair-rereview-unbounded: the decision-record re-review of a repair that writes a head has no bound but a round of rewordings

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `head-repair-rereview-unbounded`, which gives the outcome.

## review-cost-trigger-met: the trigger of the review-cost series, met by three proposals argued from review rounds

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage-klarch-workflow.md`, in its section on `review-cost-trigger-met`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
