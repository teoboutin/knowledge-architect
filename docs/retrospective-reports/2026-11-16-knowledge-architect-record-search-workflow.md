# 2026-11-16-knowledge-architect-record-search-workflow

- **Received file**: 2026-11-16-knowledge-architect-record-search-workflow.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, at `v0.10.0-103-ga973e5cb`. The project's main commit at the session's start was `a973e5cb`. The work was on the branch `record-search-workflow`, with a clean tree."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
  - `issue@agent-skills@no-test-runs-a-saved-workflow-s-logic`: touches, respect.
  - `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: touches, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@check@a-commit-message-is-a-document`: constrains, respect.
  - `design@check@one-command-judges-messages-after-the-commit`: constrains, respect.
  - `rejected@check@commit-msg-hook`: constrains, respect.
  - `issue@check@a-failing-message-waits-for-a-history-edit-while-work-is-uncommitted`: touches, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`: touches, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.

## message-claim-stale-within-branch: a message's claim about another entry goes stale when a later commit of the branch changes that entry

Members: P1 of 2026-11-16-knowledge-architect-record-search-workflow.

- **a. What it says.** A message of the branch cited `issue@agent-skills@no-test-runs-a-saved-workflow-s-logic`
  as recording that the workflow's body is untested. The claim was true when written; a later commit
  of the branch, the harvest, narrowed that issue, and the claim became false. Two of the branch's
  four history passes were for claims "made stale or wrong this way". The file names the reword
  procedure of the root `CLAUDE.md`, `instructions@git-workflow` point 2, and the primer's rule that
  a commit message is a document, finds the rule that a message is judged against its own commit
  clear, and asks whether the project's `## Git` should order a branch's commits so that a harvest
  lands before the messages that cite what it rewrites. "That may be judgement"; no proposal.
- **b. Whether it still applies.** The rule the file finds clear was in the text at the file's
  version and is on main: `path@agent-skills@content/skills/review/SKILL.md`, in
  `skill@knowledge-architect-review@what-review-leaves`, near line 218 on main and 219 at a973e5cb:
  "A message is judged against its own commit: a plan it states that a later commit of the branch
  changes is no mistake when the later commit's message names the change." The root `CLAUDE.md`
  holds no rule on the order of a branch's commits.
- **c. Whether it is already recorded.** `design@check@a-commit-message-is-a-document` and
  `design@check@one-command-judges-messages-after-the-commit` (both `constrains, respect`): a
  message is judged as a document against its own commit's tree, after the commit.
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
  (`touches, respect`) collects claims false when written; this one was true when written.
  `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command` (`touches, respect`)
  holds the cost of each reword.
- **d. Its kind.** Not a defect of the text. The rule judges a message against its own commit, so a
  claim true at its commit is no mistake when a later commit changes the entry and its message says
  so, as the harvest's does; the rule's example names a plan, and its first clause is general. A
  history pass for a claim that was only made stale was not owed; one for a claim false at its own
  commit was, and the file counts both together. Ordering commits around a harvest is the session's
  judgement, which `primer@room-to-judge` leaves open on purpose.
- **e. Its scope.** The project's `## Git` section, the blamed text, which the finding does not
  change; the rule it rests on is installed text.
- **f. A better fix.** No change. Rival: a rule ordering a harvest before the messages that cite what
  it rewrites; a fixed sequence imposed on what a session orders by judgement, which the work did not
  need, since the existing rule already makes a stale claim no mistake. A rewording that says the
  rule covers any claim about another entry, not only a plan, would change no act the clause does
  not already give.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change. Ground: `skill@knowledge-architect-review@what-review-leaves` judges "a message against its own commit", at a973e5cb and on main, so a claim made stale by a later commit that names the change needs no reword; ordering a branch's commits is left to judgement, per `primer@room-to-judge`.

## parallel-agents-combined-by-cherry-pick: parallel implementers on sibling branches combined by cherry-pick

Members: P2 of 2026-11-16-knowledge-architect-record-search-workflow, with the members of the
cluster.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-knowledge-architect-intake-forty-seven-files-klarch-workflow.md`, in its section on `parallel-agents-combined-by-cherry-pick`, which gives the outcome.
