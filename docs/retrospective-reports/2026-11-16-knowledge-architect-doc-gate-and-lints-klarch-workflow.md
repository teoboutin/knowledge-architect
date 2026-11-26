# 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow

- **Received file**: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-147-gbb93531b` when the work began, branch `doc-gate-and-lints`, tree clean."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@splits-planned-before-work`: constrains, respect.
  - `design@agent-skills@split-not-needed-is-an-issue`: constrains, respect.
  - `design@agent-skills@code-shape-axis`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@projects-are-split-into-modular-units`: constrains, respect.
  - `design@agent-skills@rust-templates-one-home`: constrains, respect.
  - `design@agent-skills@documentation-presence-is-linted`: constrains, respect.
  - `design@knowledge-architect@doc-lints-table`: constrains, respect.
  - `rejected@knowledge-architect@lint-table-per-published-crate`: constrains, respect.
  - `goal@every-target-project-can-adopt-it`: constrains, respect.
  - `goal@adoption-is-easy`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `design@agent-skills@shared-text-fragment`: constrains, respect.
  - `design@agent-skills@shipped-text-cites-no-entry`: constrains, respect.
  - `design@agent-skills@module-split-guidelines`: constrains, respect.
  - `rejected@agent-skills@module-split-at-a-size-threshold`: constrains, respect.
  - `design@agent-skills@restatement-size-test`: constrains, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `design@agent-skills@instruction-record-is-minimal`: constrains, respect.
  - `issue@check@history-rs-is-past-one-read`: touches, respect.
  - `issue@git@git-rs-is-past-one-read`: touches, respect.
  - `design@knowledge-architect@changelog-entries`: constrains, respect.
  - `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect.
  - `issue@model@resolver-rs-is-past-one-read`: touches, respect.
  - `issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read`: touches, respect.
  - `issue@transcript@reader-rs-is-past-one-read`: touches, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@cli@the-commands-are-a-library-module`: constrains, respect.
  - `design@cli@command-parsing-home`: constrains, respect.
  - `design@cli@arguments-parse-through-clap`: constrains, respect.
  - `design@cli@exit-code-ladder`: constrains, respect.
  - `design@cli@a-refusal-by-arguments-writes-nothing`: constrains, respect.
  - `rejected@cli@each-crate-parses-its-commands`: constrains, respect.
  - `design@agent-skills@verification-in-primer`: constrains, respect.
  - `design@knowledge-architect@versioning-policy`: constrains, respect.
  - `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`: touches, respect.
  - `issue@query@no-command-searches-the-entries-or-lists-an-audit-s-work`: touches, respect.
  - `design@query@query-depends-on-no-check`: constrains, respect.

## search-misses-work-added-after-it, members W1 and a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow W1, and its standing
answer on an entry the record search did not return.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on search-misses-work-added-after-it, which gives the outcome and weighs W1's
proposal.

## split-rule-two-texts: the rule on splits had two texts that left no move satisfying both

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow W2.

- **a. What it says.** `design@agent-skills@splits-planned-before-work` said of a split a work bears
  on "Neither is left to the implementer, nor parked as an issue", the planning skill "never left to
  the implementer, and never an issue", while the design skill's loop step 1 let the discussion write
  "the split's shape, or that none is made". A documentation pass over files past one read needed no
  split, and the owner ruled an issue per file. Repaired in the session: two heads, loop step 1 the
  one installed home. No proposal.
- **b. Whether it still applies.** No. At `bb93531b`, line 394 of
  `git show bb93531b:crates/agent-skills/content/skills/planning/SKILL.md` read "never left to the
  implementer, and never an" with "issue." on the next line, and the head's body read "Neither is
  left to the implementer, nor parked as an" with "issue." on the next. On main,
  `path@agent-skills@content/skills/planning/SKILL.md` line 417 reads "never left to the implementer.
  A split", and a `perl -0ne` search across line breaks for "never an issue" and "nor parked as an
  issue" over `path@agent-skills@content/`, `path@agent-skills@fragments/` and
  `register@agent-skills@design` returns nothing. Loop step 1 of
  `path@agent-skills@content/skills/design/SKILL.md`, near line 529, now says that where no split is
  made over code the work does not write, and the signal is not outweighed, "it becomes an issue of
  kind `design`". The repair is 29fe9643, "[agent-skills] A split signal a work does not need is an
  issue of kind `design`, weighed again at the next design or planning of that code", on main, which
  wrote `design@agent-skills@split-not-needed-is-an-issue` and
  `design@agent-skills@split-issue-reenters-at-planning`.
- **c. Whether it is already recorded.** The two heads above, and
  `design@agent-skills@splits-planned-before-work` (`constrains`), whose body now points to them;
  `design@agent-skills@no-two-installed-instructions-contradict` (`constrains`) is the rule the
  repair met.
- **d. Its kind.** A defect of the text, two installed instructions leaving no move satisfying both,
  at the file's version.
- **h. The proposal.** No change: already repaired. The cost of the new re-entry point is the subject
  past-one-read-trigger-met-by-any-edit, analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow.md`.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground: 29fe9643, on main, removed both texts' "never an issue" and wrote `design@agent-skills@split-not-needed-is-an-issue`, loop step 1 its installed home.

## setup-lint-keys-refused: the setup skill asked for lint keys cargo refuses

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow W3.

- **a. What it says.** The setup skill asked a published crate for the documentation lints "in its
  `Cargo.toml`'s `[lints.rust]` table"; cargo refuses a crate's own lint keys beside
  `[lints] workspace = true` ("cannot override `workspace.lints` in `lints`", cargo 1.98.0).
  Repaired on the branch: the lints go in the table the crate takes.
- **b. Whether it still applies.** No. At `bb93531b`, line 454 of the setup skill's source read
  "turns into failures: `missing_docs` in its `Cargo.toml`'s `[lints.rust]` table". On main,
  `path@agent-skills@content/skills/setup/SKILL.md` lines 457 to 462 read "`missing_docs` in the
  `rust` table of the lints the crate takes", and "A crate that takes the workspace's lints, with
  `[lints] workspace = true`, takes them from the workspace's `[workspace.lints.rust]` and
  `[workspace.lints.clippy]`, since cargo refuses a crate's own lint keys beside that line". The
  repair is 4fb30348, "[agent-skills] The setup asks for the documentation lints and `unsafe_code` in
  the lint table a crate takes, its own or the workspace's", on main.
- **c. Whether it is already recorded.** `design@knowledge-architect@doc-lints-table` (`constrains`)
  holds the measured refusal, "a crate's `[lints]` cannot take the workspace's table and add lints of
  its own", and `rejected@knowledge-architect@lint-table-per-published-crate` (`constrains`) the
  shape it refuses; `design@agent-skills@documentation-presence-is-linted` keeps the lints asked of
  every target project.
- **d. Its kind.** A defect of the text, wrong at the file's version: its argument turns on cargo's
  behaviour, measured.
- **h. The proposal.** No change: already repaired.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground: 4fb30348, on main, puts the lints in the table the crate takes, as `design@knowledge-architect@doc-lints-table` measures cargo.

## past-one-read-test-unstated: "a file a session cannot read whole" names no test

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow W4, and the second item
of its standing answer on primer rules acted on as absent, "past one read" measured by lines.

- **a. What it says.** `path@agent-skills@fragments/module-split.md`: "Size has one role: a file a
  session cannot read whole in one pass is a signal ... No number of lines is a signal by itself."
  The session's spec measured it by the Read tool's default of 2000 lines; two reviewers found a
  file of 1949 lines past one read, the Read having cut it at a cap of 25000 tokens, and the spec
  contradicting the head it cited. The test, a Read with no range that returns the file cut short,
  is in the head's argument, not in the installed text. Proposal: the fragment's sentence names its
  test, a rewording adding 0 instructions; cause, a restatement that drifted from its home; rival, a
  threshold of lines, which `rejected@agent-skills@module-split-at-a-size-threshold` records against.
- **b. Whether it still applies.** Yes. Lines 33 and 34 of `path@agent-skills@fragments/module-split.md`
  on main read "**Size has one role: a file a session cannot read whole in one pass is a signal**,
  since a review that reads it in parts judges code it has not read. No number of lines is a signal
  by itself.", the same as lines 31 and 32 at `bb93531b`. The test is stated in
  `design@agent-skills@module-split-guidelines`, whose argument says "the Read tool returns a file
  given with no range only up to a cap"; a search of `path@agent-skills@content/` and
  `path@agent-skills@fragments/` for "cut short" and "with no range" finds no size test. The
  fragment's hosts are the development skill, `agent@knowledge-architect-code-split-scout` and
  `agent@knowledge-architect-code-shape-reviewer`, by a `grep` of `module-split` under
  `path@agent-skills@content/`. The issues on files past one read measure it as the head does:
  `issue@testing@binary-rs-is-past-one-read` says "a Read with no range refuses it".
- **c. Whether it is already recorded.** `design@agent-skills@module-split-guidelines` (`constrains`):
  "Size has one role, a file a session cannot read whole in one pass, and no number of lines is a
  signal." `rejected@agent-skills@module-split-at-a-size-threshold` (`constrains`) refuses a count of
  lines. No issue.
- **d. Its kind.** A defect of the text, unclear: the spec's writer measured by lines and two
  reviewers by a read cut short, two readings of one sentence in one work, which
  `design@agent-skills@no-two-installed-instructions-contradict` has repaired once a reading
  confirms them. The test is a fact
  fixed outside the text, the harness's cap on a read, which judgement alone does not supply.
- **e. Its scope.** Installed text, a fragment of `agent-skills` shipped whole in three hosts, per
  `design@agent-skills@shared-text-fragment`; no goal of the project. It reverses no head: the
  rewording states the head's own test, and keeps size one signal with no number of lines, as
  `goal@projects-are-split-into-modular-units` and the rejected threshold ask.
- **f. A better fix.** The nearest rival is no change, the measure by lines read as a lapse of "No
  number of lines is a signal by itself". It loses on the fact that the sentence never says how a
  session knows it cannot read a file whole, so the lapse reading still leaves the act without its
  test; the reviewers who found the defect used the head's test, which the text they apply does not
  hold. The other rival, a threshold of lines, is the rejected design. The rewording names no tool
  of one harness, so it holds for each of the three hosts, every one of which reads files with a
  reader that has a size limit: the development skill's session and the two agents, whose tools
  include Read. It says "a read that asks for the whole file, under the reader's usual size limit",
  not "a read with no range", for two facts of the harness. A read with no range stops at a default
  count of lines, 2000 in the Read tool's description, so "with no range" would make that count the
  signal for a long file of short lines that a read asking all its lines returns whole, against "No
  number of lines is a signal by itself". And an option that lifts the size limit, `allow_large`,
  returns a file up to what fits in the context left, an option the head's argument says the tool's
  description reserves for a need of the whole file; the head's measure, and the issues on files
  past one read, are of the read without it.
- **g. Its route.** A fix in the installed text: edited at its source,
  `path@agent-skills@fragments/module-split.md`, as `path@agent-skills@CLAUDE.md`, section "Editing
  an installed skill or agent", says, then installed with `cargo klarch install-agent-skills`. A
  rewording needs no design entry, per `design@agent-skills@instruction-record-is-minimal`; its
  argument goes in the commit message.
- **h. The proposal.** Replace lines 33 and 34 of `path@agent-skills@fragments/module-split.md`:

```text
**Size has one role: a file a session cannot read whole in one pass is a signal**, since a review
that reads it in parts judges code it has not read; a read that asks for the whole file, under the
reader's usual size limit, and returns it cut short or refuses it shows the signal. No number of
lines is a signal by itself.
```

  Instructions added: 0, removed: 0; the clause states the test of an existing instruction, a
  rewording that makes its intent clearer. No changelog entry by the rewording test of
  `design@knowledge-architect@changelog-entries`, as the search's verdict on that head reads it.

Default: the fragment's size sentence reworded as above, handled now.
Outcome: decided by the session: handle now, the rewording above at `path@agent-skills@fragments/module-split.md` (0 instructions added, 0 removed). Ground: two readings of one sentence in one work, repaired under `design@agent-skills@no-two-installed-instructions-contradict`; the test is the one `design@agent-skills@module-split-guidelines` argues, and no number of lines enters, per `rejected@agent-skills@module-split-at-a-size-threshold`.

## phrase-search-across-line-break: no command searches a phrase across a line break

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow C1, and its standing
answer on judgement-free tasks, the line-joining search.

- **a. What it says.** `primer@verify-before-relying` has "a search returns nothing" run "so that
  they match across a line break and across emphasis marks, since wrapped prose splits a phrase over
  two lines". The spec's claim that a rewritten rule left no "never an issue" behind was first
  checked by a plain `grep`, which three reviewers showed passed vacuously, both target sentences
  wrapped; the session wrote `perl -0ne '...\s+...'`. Proposal: a checker command, or an option of
  one, that searches a phrase across line breaks and inline markup over given paths (adds 0
  instructions, and lets the primer's sentence name it); rival, the `perl` line each session
  rewrites.
- **b. Whether it still applies.** Yes: `path@agent-skills@content/PRIMER.md` line 149 holds the
  rule, and no command of `klarch` searches a phrase over files (`cargo klarch --help` lists none;
  `transcript search` reads transcripts). The vacuous case is real at the file's version: in
  `git show bb93531b:crates/agent-skills/content/skills/planning/SKILL.md` the phrase is split
  "never an" and "issue." over lines 394 and 395, which `perl -0ne` finds and a `grep` does not.
- **c. Whether it is already recorded.** No entry records this task.
  `issue@query@no-command-searches-the-entries-or-lists-an-audit-s-work` (`touches`) holds a ranked
  keyword search of the record's entries, another task; the verdict asks that this one be placed
  apart or argued a member, and it is apart: an exact phrase over given paths.
  `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head` (`touches`)
  leaves where a new tool lives open. `goal@tasks-a-tool-does-better-are-the-tool-s` (`constrains`,
  with an owner question across seven recurring tasks): "A task is taken in by a decision of
  knowledge-architect's owner, never because the workflow asks it."
  `design@agent-skills@verification-in-primer` keeps the rule in the primer.
- **d. Its kind.** A missing command for a task the workflow asks of every session that writes a
  claim of absence, whose result follows with no judgement from the phrase and the paths.
- **e. Its scope.** The checker's programs, installed text only for the primer's sentence once a
  command exists; the goal above makes the taking in the owner's. Placed in `query`, it names and
  runs no check, per `design@query@query-depends-on-no-check`.
- **f. A better fix.** The rival, a recipe in the primer's sentence, such as the `perl -0ne` line,
  adds a statement to the text every session loads, most of which never write such a claim, and
  gives each session a script it may still get wrong on emphasis marks. A command costs no session
  that does not run it. The fact that decides is the goal: the command exists only if the owner
  takes the task in; until then the rule stands as it is, and the recipe would be text for a task
  not yet ruled.
- **g. Its route.** An issue, the work needing a branch of its own under `skill@klarch-development`,
  once the task is taken in.
- **h. The proposal.** The owner takes the task in, and the issue is opened in `query`, kind `todo`,
  `issue@query@no-command-searches-a-phrase-across-line-breaks`, opened on this default in a commit
  of its own. If the owner does not take it in, that commit leaves the branch and the primer's rule
  stands.

Default: the task taken in, and the issue opened in `query` from the draft.
Outcome: for the owner (a task to take in, which `goal@tasks-a-tool-does-better-are-the-tool-s` reserves to the owner): the task taken in, the issue opened in `query`. The owner: "6,7,8 should be taken in."

## transcript-anonymized-by-hand, members C2 and a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow C2, and its standing
answer on judgement-free tasks, the transcript's anonymization.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow.md`,
in its section on transcript-anonymized-by-hand, which gives the outcome.

## verify-before-relying-lapses, members two standing answers

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
primer rules acted on as absent, first and second items (a head named under the wrong Component; a
head placed in the wrong file and "past one read" measured by lines).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on verify-before-relying-lapses, which gives the outcome. The measure by lines is
also a member of past-one-read-test-unstated, above, for the text it shows unclear.

## parallel-agents-combined-by-cherry-pick, members two standing answers

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
primer rules acted on as absent, third item (`git cherry-pick -x` trailers citing source commits,
removed by a history edit), and the third task of its answer on judgement-free tasks (the
integration of commits from parallel worktrees by `cherry-pick` and a trailer strip).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-knowledge-architect-intake-forty-seven-files-klarch-workflow.md`,
in its section on parallel-agents-combined-by-cherry-pick, which gives the outcome.

## attribution-slips-recur, member a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
primer rules acted on as absent, fourth item (hedges of an owner's report dropped by three agents).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-thaum-cast-permissions-klarch-workflow.md`,
in its section on attribution-slips-recur, which gives the outcome.

## code-rules-held, member a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
code, test and split rules (279 answered warnings, not too strict; the split rule as it stood too
strict, W2).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on code-rules-held, which gives the outcome. The split rule's part is
split-rule-two-texts, above.

## session-carried-agent-work, member a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
work the session carried (the spec's README and its first review's repairs).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on session-carried-agent-work, which gives the outcome.

## agent-alive-after-reporting: a repair agent stayed alive on background work after its report

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer
"An agent that returned unfinished, or a report not actionable?".

- **a. What it says.** "None unfinished. One repair agent stayed alive on background work after
  reporting and had to be stopped."
- **c. Whether it is already recorded.** No entry; the neighbouring cases, agents that hand back
  while agents they dispatched still run, are child-agents-outlive-parent-hand-back, analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-record-search-workflow-klarch-workflow.md`.
  This one is the reverse: the report came, and the agent's own work went on after it.
- **d. Its kind.** An observation: the report was complete and acted on; the session stopped the
  agent, at the cost of one action. The file names no instruction followed or lacking, and states no
  mechanism, so an instruction on it would be a behaviour addition with no evidence of what the
  session lacked, per `skill@knowledge-architect-retrospective-intake@admission-rules`.
- **h. The proposal.** No change.

Default: no change, an observation.
Outcome: decided by the session: no change, an observation. Ground: the report was finished and acted on, and no instruction is blamed; a behaviour addition needs the evidence `skill@knowledge-architect-retrospective-intake@admission-rules` names.

## conclusions-wrong-after-verification, member a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
a returned conclusion that proved wrong (a premortem cause judged kept, its tripwire then deleted).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on conclusions-wrong-after-verification, which gives the outcome.

## review-cost-answers, member a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
the merged branch's reviews.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on review-cost-answers, which gives the outcome.

## tools-built-in-sessions, member a standing answer

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow, its standing answer on
a tool or procedure set up (the doc gate, shipped by the branch).

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on tools-built-in-sessions, which gives the outcome.

## standing-doc-gate-and-lints-no-subject: the standing answers that report nothing to change

Members: the standing answers of 2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow
"Did this session need to change an installed file?" (yes, as this repository's own work: the setup
skill for W3, the planning, design and development skills and the code-shape reviewer for W2); "Did
the session miss something a project skill adds?" (none found); "A reference or a path written
without backticks for want of a checked form?" (none).

- **a. What it says.** Each answer as quoted above.
- **d. Its kind.** Observations. The installed files changed were the work's subject, edited at their
  source, W2 and W3 above.
- **h. The proposal.** No change.

Default: no change, observations.
Outcome: decided by the session: no change. Ground: no answer reports a defect or a missing form; the edits to installed text are W2 and W3, analysed above.
