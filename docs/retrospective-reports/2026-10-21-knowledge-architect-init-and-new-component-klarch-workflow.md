# 2026-10-21-knowledge-architect-init-and-new-component-klarch-workflow

- **Received file**: 2026-10-21-knowledge-architect-init-and-new-component-klarch-workflow.md, the
  workflow file of this repository's session that designed, specified, built and merged
  `klarch init` and `klarch new-component`, compacted several times.
- **Version used**: built from its own source, main at `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`. The review skill and the primer are the same
  at both.
- **Standing entries that bear on it**:
  - for W1: see the cluster it points to;
  - for W3: `issue@agent-skills@the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head`,
    `tripwire@agent-skills@goal-missed-though-graded`, not met, since no goal existed to be missed;
  - for W4: `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`,
    `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`,
    `tripwire@agent-skills@pointer-not-followed`, not met, since no restatement was replaced by a
    pointer;
  - for C1: `design@core@fix-refusal-mixed-state`;
  - for C2: `issue@core@git-environment-variables-redirect-the-walk`.

## W1. Whether the repairs of a follow-up review owe another review

This finding is analysed with its cluster in
`path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`,
section "W2, with the findings on how the review before a merge ends".

## W2. A commit that lands after the review before the merge and is not a repair of it

This finding is analysed with P2 of 2026-10-21-knowledge-architect-init-and-new-component, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-21-knowledge-architect-init-and-new-component.md`.

## W3. A purpose the owner stated was recorded as a premise, not as a goal

- **a. What it says.** The owner's opening message called Rust the primary focus language; the
  session recorded it in design heads "on the owner's word" and never asked whether it was a goal. A
  decision-record reviewer surfaced it at the merge review, and the owner then stated it as a goal.
  The file proposes no text: a lapse.
- **b. Whether it still applies.** The instruction is unchanged: the goal-setting skill's
  description, "MUST use when a Component has no goal, when the owner states a new purpose or
  abandons one". The goal is on main, `goal@knowledge-architect@rust-projects-come-first`, and
  `design@core@init-follows-cargo-init` cites it.
- **c. Whether it is already recorded.** In part:
  `issue@agent-skills@the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head` asks
  whether a head is a home for a project's purpose, from the primer's sentence "What the project is
  for, and how it is arranged in order to get there, belongs here", still in `primer@design-heads`.
  Its "Why it matters" names this failure: "A session that writes a purpose into a head records an
  intent no goal-setting ruled on." It has no real instance yet; this is one.
- **d. Its kind.** A lapse of the goal-setting trigger, beside an unclear sentence of the primer
  that the open issue records and that reads as permission for what the session did.
- **f. A better fix.** Rivals: no change, as the file proposes; the issue gains the instance, which
  is the evidence its question lacked; the primer's sentence reworded now, which the issue's close
  asks to follow a reading of `design@agent-skills@one-home-for-head-rules`, a design question.
- **g. Its route.** An edit of the issue entry.
- **h. The proposal.** Handle now: the issue gains this session as its first real instance.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W4, with W7 of 2026-10-23-rootpoint-editor-conveniences-milestone-klarch-workflow. Reviewers' reports not saved whole as they arrive

- **a. What it says.** Here, two reviewers briefed to save their reports did not, and the dispatcher
  did not save them; after a compaction their items survived only in the ledger and the transcript.
  In the other file, the session saved one-paragraph summaries instead of the reports, and three
  transcript reviews in a row found reviewers' "Decided" items and a measurement with no durable
  outcome. Both call it a lapse. The other file proposes a one-line restatement in each reviewer
  agent's definition, since the instruction is read when the review starts and the reports arrive
  later, often after a compaction.
- **b. Whether it still applies.** The instruction is unchanged,
  `skill@knowledge-architect-review@what-review-leaves`: "The dispatcher saves each report in a file
  outside the project, such as a scratch directory, as it arrives, and builds the record from those
  files: a compaction of the conversation keeps neither the reports nor their items." No agent
  definition under `path@agent-skills@content/agents/` mentions it. A fact measured in this intake's
  own session: the seven general-purpose subagents it dispatched to run checks a to c were each
  briefed to write a report file in a scratch directory of their own, and the harness refused each
  write with "Subagents should return findings as text, not write report files". So the reviewer's
  own write is no fallback for a general-purpose axis, and the dispatcher's save is the only one.
- **c. Whether it is already recorded.** In part:
  `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access` leans on the dispatcher's
  save, "so a refused write loses no report", which these two sessions show lapsing. Its close is
  "One dispatch of a general-purpose subagent briefed to write one file in a named scratch
  directory … If it is, the review skill states that a brief asks for the report as text, or names
  the agent types that write files". The seven refusals above are that dispatch, in the harness
  version current now. `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`
  records the general mechanism.
- **d. Its kind.** A lapse that recurs at one point of delivery: two sessions, and in the second
  three reviews in a row. Under the lapse rule of
  `skill@knowledge-architect-agent-configuration@admission-standard`, "the fix moves where it is
  delivered, and the recurrence is its evidence".
- **e. Its scope.** `goal@knowledge-architect@agents-work-without-drift`; installed text, the review
  skill and the reviewer agents.
- **f. A better fix.** Rivals: a line in each reviewer agent's definition, as proposed, which the
  reviewer reads and the dispatcher does not, unless the line makes the reviewer end its report with
  the reminder; a last line of every report, asked by the review skill's brief rule, so that it
  reaches every axis, general-purpose ones included, and arrives with each report, at the moment of
  the save; no change, the lapse rule's own outcome for a lapse that does not recur. The deciding
  fact: the moment the save is needed is the arrival of a report, and the only text that arrives
  then is the report itself. The brief rule reaches every axis, where an agent's definition reaches
  only the installed agents. Instructions added: 1, the brief's last-line request, delivered as one
  sentence with its pointer.
- **g. Its route.** An installed text edit under `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent", and an edit of the issue entry.
- **h. The proposal.** Handle now: `skill@knowledge-architect-review@review-invariants` asks every
  brief to have the reviewer end its report with one line saying that the dispatcher saves the
  report whole, per the sentence above; and the review skill states that a general-purpose reviewer
  returns its report as text, which closes the write half of
  `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access` on the measurement above.
  Its read half stays open.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C1. `check --fix` refused whenever the staged tree held no manifest, fixed on the branch

- **a.** In a repository whose manifest was untracked, with any other change staged, `check --fix`
  failed and wrote no file. Fixed on the branch.
- **b.** Fixed: `design@core@fix-refusal-mixed-state` now says "a staged tree holding no manifest
  … nothing is compared and the fix runs", from the commit "[review] Repairs of the review before
  the merge of `init` and `new-component`"; the code matches.
- **h.** No change: already repaired.
- **Default**: no change, already repaired.
- **Outcome**: accepted.

## C2. A git hook's environment can point the walk at another index

- **a.** A pre-commit hook gets `GIT_INDEX_FILE`; `init` refuses such variables, every other
  command reads them. Recorded as an issue.
- **b, c.** Recorded: `issue@core@git-environment-variables-redirect-the-walk`, with the
  measurement on git 2.43.0. `init`'s refusal is in `path@core@src/cli/create.rs`.
- **h.** No change: already recorded.
- **Default**: no change, already recorded.
- **Outcome**: accepted.

## The standing answers

- **a.** Three rules acted on as if absent, each caught before the merge: an answer to the owner
  that a tripwire had gained an unruled clause, unchecked; two rules added to a head under a
  relocation said to add no decision; a code comment naming a cause not established.
- **b.** The rules stand: `primer@verify-before-relying`, "The same checks apply to an answer given
  to the owner in the conversation"; the backstop of `primer@design-heads`. The relocation's
  repair is in the commit "[review] `init` follows `cargo init` wherever no argument departs from it,
  a head of its own".
- **d.** Lapses. The backstop lapse is one instance of the recurrence judged against
  `member-beyond-the-argument`, now
  `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`, and
  `tripwire@agent-skills@head-created-without-deliberation` in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-20-thaum-pin-move-0-8-0-klarch-workflow.md`,
  section "The standing answers"; the unchecked claims are instances of the recurrence analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`,
  section "The standing answers".
- The ruling is given where each recurrence is analysed.
