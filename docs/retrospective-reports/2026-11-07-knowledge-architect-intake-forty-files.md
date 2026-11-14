# 2026-11-07-knowledge-architect-intake-forty-files

- **Received file**: 2026-11-07-knowledge-architect-intake-forty-files.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: `v0.9.0-184-g00533a5c` when the work began." / "The project's main commit at the start: `00533a5c`. Branch `intake-forty-files`, a clean tree."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@xtask@a-local-task-is-the-maintenance-tool-s`: constrains, respect.
  - `design@xtask@one-module-per-subcommand`: constrains, respect.
  - `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`: touches, respect.
  - `design@git@git-is-asked-through-one-module`: constrains, respect.
  - `issue@git@tools-run-git-past-the-wrapper`: touches, respect.
  - `design@git@an-operation-in-git-s-terms-is-the-library-s`: constrains, respect.
  - `design@git@the-library-brings-no-parse-of-the-model-into-a-tool-s-build`: constrains, respect.
  - `issue@git@whom-the-git-library-serves-and-why-no-tool-wraps-git-have-no-argued-head`: depends-on, rule-before-work.
  - `design@agent-skills@xtask-pins-checker`: constrains, respect.
  - `design@agent-skills@maintenance-crate-ask`: constrains, respect.
  - `design@agent-skills@upstream-tool-loop`: constrains, respect.
  - `design@agent-skills@premortem-as-watch-points`: constrains, respect.
  - `design@agent-skills@committed-intake-analysis`: constrains, respect.
  - `design@knowledge-architect@checker-asks-git-through-the-crate`: constrains, respect.
  - `design@knowledge-architect@git-crate`: constrains, respect.
  - `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`: touches, respect.
  - `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`: touches, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`: touches, respect.
  - `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: touches, respect.
  - `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`: constrains, respect.
  - `goal@serves-one-kind-of-project`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `design@agent-skills@code-change-reads-its-record`: constrains, respect.
  - `design@agent-skills@no-routing-table`: constrains, respect.
  - `rejected@agent-skills@root-routing-table`: constrains, respect.
  - `design@agent-skills@development-skill`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `rejected@agent-skills@admission-standard-in-primer`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Members: 2026-11-07-knowledge-architect-intake-forty-files P2; 2026-11-10-knowledge-architect-record-search-verdicts P1; 2026-11-11-knowledge-architect-goals-first-slice P2; 2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow C4(b); 2026-11-11-knowledge-architect-issue-triage-klarch-workflow C2; 2026-11-12-knowledge-architect-finding-repairs P1 and 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow C2; 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow C2; 2026-11-13-thaum-long-draw-klarch-workflow C2; 2026-11-13-thaum-pin-move-0-10-klarch-workflow C2; 2026-11-14-knowledge-architect-component-arguments-slice P3. Also met in the standing answers on scripts of 2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow, 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow, 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow, 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow, 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow, 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow and 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow.

- **a. What it says.** Fourteen sessions reworded the message of a commit that is not the newest by
  a script written for the occasion: twelve of this repository, with
  `git filter-branch --msg-filter` as `instructions@git-workflow` point 2 has it, and two of thaum,
  with a loop of `git commit-tree` over the same trees, since the harness refuses an interactive
  rebase. Counts the files give: twelve rewords in four passes, plus one more pass after `git am`
  stripped the bracketed subject prefixes (intake-forty-files P2); six (record-search-verdicts P1);
  four (goals-second-slice C4(b)); three (issue-triage C2, 2026-11-12-knowledge-architect-new-crate-placeholder C2); seven runs,
  four of them moves of a review's record between messages (finding-repairs P1); two and three
  `commit-tree` loops (long-draw C2, pin-move-0-10 C2). Two files report a script that left
  the branch as it was with nothing to say so but the checks: a pattern that missed a wrapped line
  (goals-first-slice P2), and a script asserting its pattern, which failed on the commits lacking
  it, so that `filter-branch` stopped and applied nothing, caught because `range-diff` showed no `!`
  (component-arguments P3). The text blamed is root
  `CLAUDE.md` point 2: "write a script that reads a message on stdin and writes the reworded one,
  then run `git filter-branch -f --msg-filter '<script>' origin/main..HEAD` with a clean tree", and,
  in the thaum files, `skill@knowledge-architect-review@what-review-leaves`, "a history edit of the
  branch". Proposed fixes: none beyond the recorded issue (most project files); a `cargo x` command
  until the checker ships one (finding-repairs P1); a `klarch` command that rewords one message from
  a file and runs the empty-diff and `range-diff` checks itself (finding-repairs C2); a command
  rewriting named commits' messages from files, refusing a dirty tree and printing the `range-diff`,
  or a recipe in `skill@knowledge-architect-review@what-review-leaves` (long-draw C2); `cargo x reword <commit> --message-file <f>`,
  "Generic to every project under the per-commit gate" (pin-move-0-10 C2).
- **b. Whether it still applies.** Yes. `path@knowledge-architect@CLAUDE.md`, at origin/main
  `v0.10.0-59-g605d1538`, line 237, still has the session write the script and run
  `git filter-branch -f --msg-filter '<script>' origin/main..HEAD`; `path@xtask@src/` holds no
  reword command (`ls tools/xtask/src/` lists `changelog.rs`, `gates.rs`, `main.rs`, `mutate.rs`,
  `skill.rs` and `transcript_kinds.rs`), and no `klarch` verb rewrites history.
- **c. Whether it is already recorded.** Yes, twice.
  `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command` (verdict `touches`, a
  seed) records the reword, the fold and the merge predicate as missing commands of this
  repository's tool, with instances from earlier sessions; its "What would close it" is "A command
  of the tool for the merge predicate, and for the reword and the fold until a published crate ships
  them". `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history` (verdict
  `touches`, a seed) records the published side, with thaum instances from the intake of 2026-11-07,
  and says that "the installed text names no recipe". The searchers' owner-question on
  `goal@tasks-a-tool-does-better-are-the-tool-s`, whether the owner takes the task in, is already
  answered: that issue's summary says "The owner counts the three as taken in". The
  `rule-before-work` verdict on
  `issue@git@whom-the-git-library-serves-and-why-no-tool-wraps-git-have-no-argued-head` bears on
  where a command that runs git sits; the outcome below builds no command, so it waits on nothing.
  The recipe in installed text that long-draw C2 offers as its rival is the work of
  `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing` (verdict
  `touches`). A `cargo x` command does not reach another project: the maintenance crate is never
  published, per the root `CLAUDE.md` table of Components ("never published"), so pin-move's
  "generic to every project" lands on the published-crate issue, not on the xtask one.
- **d. Its kind.** A missing capability, recorded twice; each instance is the procedure followed as
  written. The two silent failures are the failure the xtask issue's "Why it matters" names, "a
  reword whose pattern misses leaves the diff empty all the same".
- **e. Its scope.** `goal@tasks-a-tool-does-better-are-the-tool-s`, for a task already taken in. The
  owner of the fix: `xtask` for this repository's command, the root Component for the published one.
  Nothing reverses a head: both issues stand.
- **f. A better fix.** The nearest rival is building the command on this branch. The deciding fact:
  the published command's design is open on its own issue (a `rewrite` Component, a port of the
  owner's tool, the placement in `klarch` argued with the unargued rule of
  `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`), and the
  xtask issue asks a command of the tool now, "for the reword and the fold until a published crate
  ships them", a new command whose shape no head settles; neither is settled work for an intake
  branch, and each is a change of the Rust source that takes a branch of its own. A recipe in the installed review skill is the other rival, held
  by the branch-work skill's issue. No text edit: the root procedure is clear, and its checks caught
  both silent failures.
- **g. Its route.** An issue only: the instances are added to the two recorded issues.
- **h. The proposal.** Add this batch's instances to both issues, this repository's to the xtask
  issue and thaum's to the published-crate issue, in the commit that writes the analysis.

Default: instances added to `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command` and `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`.
Outcome: decided by the session: open an issue, as instances added to the two recorded issues. Ground: both issues record the task and its work; the owner already took the task in, per the published-crate issue's summary; no command's design is settled for this branch.

Text to add to `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`, at the end of "What":

```
Further instances, from the retrospective files received at the intake of 2026-11-14: the reword
by hand in twelve sessions of this repository, 2026-11-07 to 2026-11-14, eight reporting it as a
finding and four in their answer on scripts. Counts the files give: twelve rewords in four
`filter-branch` passes, and a fifth pass after `git am` stripped the bracketed subject prefixes, in
the intake of forty files; six in the session on the record search's verdicts; four, and three, in
the second slice on goals and the issue triage; three in the session on a new crate's placeholder;
seven runs in the session on the finding repairs, four of them moves of a review's record from one
commit message to another. Two scripts left the branch as it was: one pattern missed a wrapped line (the first slice on
goals), and one script that asserted its pattern failed on the commits lacking it, so that
`filter-branch` stopped and applied nothing (the slice on Components' arguments); each was caught
by the `range-diff` check alone.
```

Text to add to `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`, in "What", as a paragraph of its own after the paragraph of the thaum instances of 2026-11-07 and before the paragraph that opens "A further task, not taken in":

```
Further instances, from the retrospective files received at the intake of 2026-11-14: two sessions
of thaum, 2026-11-13, reworded earlier commits' messages with a hand-written loop of
`git commit-tree` over the same trees, authors and dates, two and three times, checking an empty
`git diff` and the `range-diff` by hand, since the harness refuses an interactive rebase. One
proposed a command of the maintenance crate, which reaches no other project since that crate is
never published.
```

## rust-edit-skips-development-skill: a small Rust edit inside another task, made without the development skills

Members: 2026-11-07-knowledge-architect-intake-forty-files P1, and the standing answer of
2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow on a primer rule acted on
as absent and on a project skill's addition missed.

- **a. What it says.**
  - intake-forty-files P1: "A review repair changed one word of a test's assertion message in
    `path@check@src/check/references.rs`. The session ran fmt and the crate's tests, and the full
    gates later passed, but it did not load the development skill." It calls it the second
    instance, after P1 of 2026-10-31-knowledge-architect-intake-seventeen-files, "also an intake,
    also a one-line edit inside a text change. The instruction is read at the start of a session
    and needed at an edit that looks like text." Its proposal: "None yet; a third instance would
    argue moving the delivery to where a session meets a Rust file in a text change."
  - plan-directories-slice-1, standing answer: "the session wrote a change to Rust code (a lint
    repair) without loading the development skills", in the same repair that decided a change
    against a head's recorded reading without the design skill (analysed in
    `head-changed-without-design-skill`); and, under a project skill's addition missed, "The
    project's development skill, in the lint repair above."
  - Three more loads of a skill the primer's list names were skipped at an act inside another task,
    each a standing answer: primer-import-first-line edited docs/roadmap.md "without loading
    `skill@knowledge-architect-planning`", which the list names for "before editing the roadmap";
    and plan-directories-slice-1 "opened an issue entry without loading the issue-tracking skill",
    and its dispatched agent "made two edits of installed text without loading the
    agent-configuration skill whole". They are not Rust edits, so they are not instances of this
    subject; they show the same mechanism at the primer's list, and the issue's Details name them
    for its discussion, which may weigh a delivery serving every such load.
- **b. Whether it still applies.** The instructions are unchanged from 00533a5c and a0454f46 to
  origin/main: `path@knowledge-architect@CLAUDE.md`, `instructions@verify-before-relying`, line 82,
  "**Before touching the code of a Component, ground first, per
  `skill@knowledge-architect-development@ground-before-editing` and
  `skill@klarch-development@ground-in-this-repository`.**"; and the description of
  `skill@klarch-development`, which the harness lists each session: "MUST use, together with
  `skill@knowledge-architect-development`, ... before starting or iterating on any change to this
  repository's Rust source". This is the third instance: the first, P1 of the intake of seventeen
  files, a row of a test constant edited in an installed-text repair, was analysed in the commit
  "[intake] The analyses of forty received retrospective files, three outcomes for the owner" as
  "a lapse of a clear instruction, once".
- **c. Whether it is already recorded.** No entry records it. The verdicts on
  `design@agent-skills@code-change-reads-its-record`, `design@agent-skills@no-routing-table`,
  `rejected@agent-skills@root-routing-table`, `design@agent-skills@development-skill`,
  `design@agent-skills@behaviour-additions-need-real-use`, `goal@agents-work-without-drift`,
  `goal@only-needed-instructions` and `goal@the-workflow-leads-to-better-code` are `respect`: a
  repair is no table in the root mapping the project skill to the installed one, and a project's
  development skill stays an overlay. `rejected@agent-skills@admission-standard-in-primer` is
  `respect` too: the plan-directories agent that edited installed text without the
  agent-configuration skill is its doubt, and no outcome here moves the standard into the primer.
- **d. Its kind.** A lapse of a clear instruction, recurring at one point of delivery: three
  sessions, each a small Rust edit made inside a task that is not Rust work (an installed-text
  repair, a review repair in an intake, a lint repair in a slice's migration of documents). The
  instruction is delivered at the session's start, by the root and by the skill's description;
  the edit comes later, inside the other task. By the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules`, "Where the lapse recurs at one
  point of delivery, because the instruction is read at one moment and needed at another, the fix
  moves where it is delivered, and the recurrence is its evidence."
- **e. Its scope.** `goal@the-workflow-leads-to-better-code`; the owner of the text is this
  repository's agent configuration, the root `CLAUDE.md` and `skill@klarch-development`.
  Nothing is reversed yet; a delivery chosen later may touch
  `design@agent-skills@no-routing-table`.
- **f. A better fix.** The rivals, none settled by the record:
  - a pointer to `skill@klarch-development` in each crate's scoped `CLAUDE.md`, read when a
    session works on that crate's files: twelve copies of one pointer, and whether the harness
    loads a scoped `CLAUDE.md` at an edit made without a read of the directory is an assumption to
    measure;
  - a sentence where the three tasks are delivered: the review skill's repair step, the agent-skills
    crate's `CLAUDE.md` section on editing an installed skill, a brief; three homes for one moment;
  - no change: the three edits were one line each, the gates passed on each, and the review
    judged the third. It loses on the lapse rule's own test: the recurrence is its evidence, and
    the plan-directories repair carried a decision against a head that the development skill's
    grounding reads.

  Choosing among them is a design question, with no record that ranks them.
- **g. Its route.** An issue, since the delivery needs a discussion under
  `skill@knowledge-architect-design`.
- **h. The proposal.** Open an issue entry in `register@agent-config@issue`, kind `design`, slug
  `issue@agent-config@a-rust-edit-inside-another-task-is-made-without-the-development-skills`. It states the
  instruction and its two deliveries, the three instances with their tasks, the suspected
  mechanism (read at the start, needed at an edit made inside another task), the three other
  skipped loads of item a as the same mechanism at the primer's list, the rivals above, and
  what would close it: a discussion that picks a delivery at the moment of a Rust edit, or rules
  that none is worth its cost; its re-entry point, the next instance, or the owner's word.

Default: open the issue.
Outcome: decided by the session: open an issue, anchor `agent-config`, kind `design`, slug
`issue@agent-config@a-rust-edit-inside-another-task-is-made-without-the-development-skills`. Ground: the third
instance meets the condition the intake-forty-files P1 named; the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules` moves a recurring lapse's delivery;
the rivals need a discussion, and the root's table puts what is outstanding about the agent
configuration in `register@agent-config@issue`.

## tripwire-agent-text-written-without-the-standard: its horizon passes at this intake's commit, unfired, and the rejected design it watched is judged again

Members: `agent-text-written-without-the-standard` (verdict `unrelated`,
searcher 8, who left the count to this intake); `rejected@agent-skills@admission-standard-in-primer`,
which names it.

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  files under docs/retrospective-reports/, counted from the commit that writes this line".
  `git blame` of crates/agent-skills/docs/tripwires.md on that line and the next gives 2ad7d3d9 ("[audit]
  Pass 1 of the tripwire axis applied: every tripwire kept has a horizon, and 20 leave"). Since it,
  a9e3b3c9 and e8562915 added analyses
  (`git log --diff-filter=A --format=%h 2ad7d3d9..origin/main -- docs/retrospective-reports/`). This intake's commit is the third occasion.
- **Whether it fired.** The clause needs "an addition to agent-facing text that fails the need test
  of `design@agent-skills@statement-names-an-act` or the real-use rule of
  `design@agent-skills@behaviour-additions-need-real-use`, made in a session that did not load the
  agent-configuration skill", reported in two pieces of work or merged unreported. The batch holds
  the precondition twice and the failure never: the standing answer of
  2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow, line 78, "A dispatched
  agent made two edits of installed text without loading the agent-configuration skill whole", with
  "the reviews judged the results" and no addition reported failing either test; and the standing
  answer of 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow, lines 150 to 153, where
  the session did not re-read this repository's own section on editing an installed skill after a
  compaction, which is no load of the agent-configuration skill missed. `grep -n -i "need test"`
  over the 47 received files finds nothing. The batch's one addition reported as needing no text
  is in 2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow, lines 120 to 123: the
  implementing agent's admission sentence named a property that "excludes nothing", which "A review
  caught", as the decision expects. The file does not say whether that agent loaded the
  agent-configuration skill, and a single instance a review reported is a slip, no firing, per
  `skill@knowledge-architect-issue-tracking@tripwire-entry`. Over the rest of the watch, "unfired"
  rests on the chain of standing-state reviews of the merges since the line was written, the
  reading of the subject `unfired-judged-over-branch-only`. Unfired.
- **The rejected design it watched.** `rejected@agent-skills@admission-standard-in-primer`
  (crates/agent-skills/docs/rejected.md, near line 450): "It is kept here because a doubt remains: a
  session that edits agent-facing text inside another task may not load the skill, which
  `agent-text-written-without-the-standard` watches." The movement
  instruction asks, when the tripwire leaves at its horizon, that such an entry be judged again by
  the recording tests of `skill@knowledge-architect-decision-recording@losing-alternatives`, and
  that its removal go to the owner if none holds. Test 4, "there is still some doubt that the design
  kept will achieve every goal", still holds on this batch's evidence: the doubt's first half, a
  session editing agent-facing text without the skill loaded, was observed (plan-directories-slice-1,
  above); only its consequence, a failing addition, was not. So the entry is kept, and its last
  sentence loses the pointer to the deleted tripwire.

Default: the tripwire is deleted at the commit that adds this intake's analyses, naming a9e3b3c9,
e8562915 and that commit as the occasions held through; the rejected entry's last sentence is
reworded as below (0 instructions added or removed; a `rejected` entry carries none).

```
`live`. Writing agent-facing text is not every session's work, and the agent-configuration skill's
trigger delivers the standard when it is, per `design@agent-skills@primer-limit`; the session that
wrote the statement its owner judged useless had loaded that skill. It is kept here because a doubt
remains: a session that edits agent-facing text inside another task may not load the skill, as a
dispatched agent of this repository did in the first slice of plan documents as directories, though
no addition that fails the standard was reported from it.
```

Outcome: decided by the session: delete the tripwire, unfired at its horizon, and keep
`rejected@agent-skills@admission-standard-in-primer` on test 4 with its last sentence reworded.
Ground: the horizon's third occasion is this intake's commit; no received file meets the clause;
`skill@knowledge-architect-issue-tracking@movement-instruction` deletes a tripwire whose horizon
passes unfired and has a `rejected` entry naming it judged again; test 4 of
`skill@knowledge-architect-decision-recording@losing-alternatives` holds on the observed
precondition, so the entry's removal, which would go to the owner, is not proposed.

## tripwire-checker-source-literals-are-data-whole-source-scope: its horizon passes at this intake's commit, unfired

Members: `checker-source-literals-are-data-whole-source-scope` (verdict
`unrelated`, its searcher noting that the intake of thaum's files meets its re-entry).

- **The horizon.** "the next three commits on the main branch that add an analysis under
  `path@knowledge-architect@docs/retrospective-reports/`" (crates/check/docs/tripwires.md, lines 23
  and 24). `git blame` of both lines gives 2ad7d3d9, when the file was crates/core/docs/tripwires.md.
  Since it, a9e3b3c9 and e8562915 added analyses, by
  `git log --diff-filter=A --format=%h 2ad7d3d9..origin/main -- docs/retrospective-reports/`. This
  intake's commit is the third occasion.
- **Whether it fired.** The clause: "a review finds a string literal in the tool's non-test source
  that cites a rule: a `CR:` marker followed by a rule number written out, with a sentence around
  it". Its re-entry is "the intake of a retrospective file from a project whose binary registers an
  extension", which the five thaum workflow files of this batch are, as the searcher states (taken
  here as an assumption; the outcome does not rest on it). `grep -n "CR:"` over the 47 received
  files finds nothing, nor does `grep -n -i literal` find a literal that cites a rule. The
  analysis of the intake of forty files ruled the same of the thaum audit that re-entered it,
  "no change, the tripwire has not fired" (e8562915's message). Unfired.
- **What leaving costs.** `git grep` finds no citation of the slug outside the tripwires home.

Default: the tripwire is deleted at the commit that adds this intake's analyses, naming a9e3b3c9,
e8562915 and that commit as the occasions held through.
Outcome: decided by the session: delete it, unfired at its horizon. Ground: the horizon's third
occasion is this intake's commit; no received file reports a literal of the clause;
`skill@knowledge-architect-issue-tracking@movement-instruction` deletes a tripwire whose horizon
passes unfired.

## tripwire-deferred-trigger-met-by-undesigned-work: its horizon passes at this intake's commit, unfired

Members: `deferred-trigger-met-by-undesigned-work` (verdict `unrelated`); one
citation, in `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`.
Found by the search of the tripwires homes for horizons counted in analyses; the brief did not name
it.

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  files under docs/retrospective-reports/, counted from the commit that writes this line".
  `git blame` of crates/agent-skills/docs/tripwires.md, line 35, gives 2ad7d3d9. Since it, a9e3b3c9 and
  e8562915 added analyses, by
  `git log --diff-filter=A --format=%h 2ad7d3d9..origin/main -- docs/retrospective-reports/`. This
  intake's commit is the third occasion.
- **Whether it fired.** The clause: "a second instance is recorded of a deferred trigger met by work
  that did not go through the design skill, and found only at the standing-state review." The
  standing-state findings of the batch are of two kinds, neither of them this: a re-entry or a
  trigger met in work that went through the design skill or a slice's design audit (split-guidelines
  W4, second item; planned-splits W4; plan-directories-slice-1 P2), and a tripwire's horizon reached
  (thaum long-draw, standing, lines 129 to 131; release-0-10-0, W1 and W2). The two branches of the
  batch that went through no design grounding, the release and thaum's pin move, report no deferred
  trigger met: "no record search ran, since the release decided no design" (release, standing,
  lines 118 and 119), "No record search ran (no design grounding)" (pin move, standing, line 187).
  Unfired.
- **What leaving costs.** The issue that holds the class of tripwires on agent behaviour names this
  one as a live example (crates/agent-skills/docs/open-issues/a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see.md,
  lines 88 to 93): "A tripwire of the same class, `deferred-trigger-met-by-undesigned-work`,
  waits for evidence that arises mostly in the sessions of projects that use the workflow; its
  re-entry is the standing-state review of this repository, which holds only the tree." Once deleted,
  that sentence is false and its reference dangles. The issue-tracking skill's rule on such a
  sentence, `skill@knowledge-architect-issue-tracking@movement-instruction`, "A sentence about the past
  whose reference dangles is rewritten to state the present, or removed ... Its history stays in the
  commit messages": a past-tense account of a tripwire that left is the history that rule keeps out.
  The sentence carries no present fact the issue needs: its "What" holds the instances, and its
  closing condition does not rest on it. So the sentence is removed, and the deleting commit names
  the tripwire, by its bare slug, as one more of the class that left unfired. The two sentences
  around it read on: the owner's stated aim, then "`klarch transcript search`, with
  `--all-projects` and `--subagents`, reads every session on a machine, consumers' included, which
  that work can use".
- **The issue after this commit.** The two other tripwires that cite the issue,
  `departure-overridden-as-a-reading` and
  `axis-not-run-at-a-pin-move`, leave at the same commit, so no tripwire cites
  it afterwards. Its question, how a tripwire on agent behaviour is re-entered, stands on its own
  instances and needs no edit for that.

Default: the tripwire is deleted at the commit that adds this intake's analyses, naming a9e3b3c9,
e8562915 and that commit as the occasions held through; the issue's sentence, from "A tripwire of
the same class," to "which holds only the tree." (lines 88 to 91 of
crates/agent-skills/docs/open-issues/a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see.md),
is removed (0 instructions added or removed; an issue entry carries none). Rival: the draft's first
rewording into the past tense, which keeps the instance in the issue; it loses to the rule above,
since the instance's history is the deleting commit's.

Outcome: decided by the session: delete it, unfired at its horizon, and remove the issue's sentence
that names it. Ground: the horizon's third occasion is this intake's commit; no received file
reports a deferred trigger met by work outside the design skill;
`skill@knowledge-architect-issue-tracking@movement-instruction` deletes a tripwire whose horizon
passes unfired; a sentence about the past whose reference the deletion dangles is removed, per
`skill@knowledge-architect-issue-tracking@movement-instruction`, in the same commit.

## tripwire-departure-overridden-as-a-reading: its horizon passes at this intake's commit, unfired

Members: `departure-overridden-as-a-reading` (the record search's verdict
`unrelated`, searcher 3, with its horizon noted under "Met outside the task").

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  files under docs/retrospective-reports/, counted from the commit that writes this line"
  (crates/agent-skills/docs/tripwires.md, the entry's `**Watches until:**` line). `git blame` of
  that line and the next gives 2ad7d3d9, "[audit] Pass 1 of the tripwire axis applied: every
  tripwire kept has a horizon, and 20 leave".
  `git log --diff-filter=A --format=%h 2ad7d3d9..origin/main -- docs/retrospective-reports/` prints a9e3b3c9 ("[intake] The analyses of
  seventeen received retrospective files, every outcome decided by the session") and e8562915
  ("[intake] The analyses of forty received retrospective files, three outcomes for the owner").
  Two occasions have passed; the commit that adds this intake's analyses is the third.
- **Whether it fired.** The clause: "a session applies a source's text over an answer the owner
  gave as a deliberate departure from that source, in two separate pieces of work, or once in a
  record that reaches the main branch with no review reporting it." The source the decision means
  is a specification the project follows, per `primer@owner-word-premise`. No received file
  reports the act. `grep -n -i departure` over the 47 received files finds eight lines, none of it:
  a design-conformance finding on a departure from a head (intake-forty-files, workflow file,
  line 32); a plan document explaining a departure from the installed text (thaum
  triggers-design, line 34); a project's own departures from the owner's global instructions,
  recorded by that project (thaum split-oversized-modules, lines 42 and 112); P4 of
  2026-11-11-knowledge-architect-issue-triage, a departure the owner ruled, held by an issue; and
  P2 of 2026-11-14-knowledge-architect-planned-splits-slice (lines 26, 33 and 49), where a
  subagent routed a fix in the installed text by the primer's rule and not by this repository's
  departure from it in the root `CLAUDE.md`. That last one is the nearest: but the text applied is
  the installed primer, no specification, the departure was missed where it is buried rather than
  read as an answer on a reading, and the session repaired it on the branch, one instance its own
  procedure reported; the subject `repository-departure-buried` analyses it. The firing over the rest of the watch rests on the chain of standing-state
  reviews of the merges since the line was written, none of which opened the response, the
  reading of the subject `unfired-judged-over-branch-only`.
- **What leaving costs.** `git grep` finds no citation of the slug outside the tripwires home.

Default: the tripwire is deleted at the commit that adds this intake's analyses, its message
naming the occasions held through: a9e3b3c9, e8562915 and that commit.
Outcome: decided by the session: delete it, unfired at its horizon. Ground: its horizon's third
occasion is this intake's commit, no received file of the three intakes meets its clause, and
`skill@knowledge-architect-issue-tracking@movement-instruction` deletes a tripwire whose horizon
passes unfired, the deleting commit naming the occasions the decision held through; the
nearest instance, P2 of planned-splits, is no source applied over a departure given as a reading.

## tripwire-expectation-set-closes-a-contradiction: its horizon passes at this intake's commit, unfired

Members: `expectation-set-closes-a-contradiction` (verdict `unrelated`,
searcher 8).

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  files under docs/retrospective-reports/, counted from the commit that writes this line".
  `git blame` of crates/agent-skills/docs/tripwires.md, lines 18 and 19, gives 2ad7d3d9. Since it,
  a9e3b3c9 and e8562915 added analyses
  (`git log --diff-filter=A --format=%h 2ad7d3d9..origin/main -- docs/retrospective-reports/`). This intake's commit is the third occasion.
- **Whether it fired.** The clause: "a finding that two installed instructions leave no move
  satisfying both is closed by citing an expectation set, in two separate analyses or reviews, or
  in a record that reaches the main branch with no review reporting it." Its evidence lands in an
  analysis or a review. No received file closes a finding by an expectation set:
  `grep -n -i expectation` over the 47 received files finds two lines, both the common word (searcher-check
  reviewer-reports, lines 34 and 135). The batch's one finding of two statements in conflict,
  the subject `q-label-primer-head-conflict` (W2 of 2026-11-11-knowledge-architect-issue-triage-klarch-workflow),
  is analysed under its own subject, not closed by a set; that subject's draft cites
  `design@agent-skills@expectation-set-bounds-scope` with the verdict `respect` only. Unfired.
- **What leaving costs.** One citation: a `%%` comment of the intake skill's source,
  crates/agent-skills/content/skills/retrospective-intake/SKILL.md, lines 131 to 134, "the closing
  of a contradiction by a set is watched by `expectation-set-closes-a-contradiction`."
  Once the tripwire is deleted that clause is false and its reference dangles, so the clause is
  removed in the same commit. The build strips `%%` comments, so no installed copy changes, and the
  edit follows `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", with an install
  run to show it writes nothing.

Default: the tripwire is deleted at the commit that adds this intake's analyses, naming a9e3b3c9,
e8562915 and that commit as the occasions held through; the comment loses its last clause, to read:

```
%% The sets bound a skill's scope per `design@agent-skills@expectation-set-bounds-scope`; the
%% missing sets are `issue@agent-skills@expectation-sets-for-the-installed-skills`.
```

Outcome: decided by the session: delete it, unfired at its horizon, and remove the comment's
clause that cites it. Ground: the horizon's third occasion is this intake's commit; no analysis or
review of the three intakes meets the clause; `skill@knowledge-architect-issue-tracking@movement-instruction`
deletes a tripwire whose horizon passes unfired; a comment's clause on a deleted entry is wording
now false, fixed in the same commit.

## tripwire-logic-tests-relabelled: re-entered at this intake, unfired; its horizon's second occasion

Members: `tripwire@agent-skills@logic-tests-relabelled` (verdict `unrelated`). Its re-entry is
"the intake of a retrospective workflow file, `skill@klarch-retrospective-intake`".

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  workflow files, counted from the commit that writes this line". `git blame` of
  crates/agent-skills/docs/tripwires.md, line 374, gives 87fea465, of 2026-11-02. Since it, e8562915
  added analyses. This intake's commit is the second occasion: the horizon does not pass.
- **Whether it fired.** The clause: "a test of pure logic classed as golden, mocked, integrated or
  external and carrying no recorded mutation, in two separate pieces of work". The batch reports the
  reverse: logic tested through the binary alone was given unit tests when the code-shape reviewer
  asked ("repaired with unit tests over files built in memory", transcript-unrecognised-lines,
  standing, lines 115 to 117; "The code-shape axis asked unit tests for logic reached only through
  the binary; that held", finding-repairs, standing, lines 81 and 82). No test is reported classed
  to skip its mutation. Unfired.

Default: no change; the tripwire stands, its horizon at its second occasion.
Outcome: decided by the session: no change, unfired. Ground: no answer of the batch reports a test
of logic relabelled to skip its mutation evidence; the horizon's third occasion is a later intake.

## tripwire-needless-removal-found-missing: re-entered at this intake, unfired; its horizon's first occasion

Members: `tripwire@agent-skills@needless-removal-found-missing` (verdict `unrelated`). Its
re-entry is "the intake of a retrospective workflow file, `skill@klarch-retrospective-intake`".

- **The horizon.** "three commits on the main branch that add an analysis under
  `path@knowledge-architect@docs/retrospective-reports/`, counted from the commit that writes this
  line". `git blame` of crates/agent-skills/docs/tripwires.md, line 412, gives 019ed7a8, "[review]
  Repairs of the review before the merge: the weighing before a removal in a head of its own, the
  counts in review rounds, and the content the closed issues held kept", of 2026-11-12. No analysis
  has been added since. This intake's commit is the first occasion.
- **Whether it fired.** The clause: "a session that lacked an instruction which an earlier intake
  removed or narrowed on a needless finding". The nearest finding of the batch, W3 of
  2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow (the planning skill said the
  workflow installs no development procedure), is a false sentence, not a removal by an intake, as
  its searcher found with `git log -S'This workflow installs'`. Unfired.

Default: no change; the tripwire stands, its horizon at its first occasion.
Outcome: decided by the session: no change, unfired. Ground: no answer or finding of the batch
reports a session lacking an instruction an intake removed.

## tripwire-owner-fact-recorded-unattributed: its horizon passes at this intake's commit, unfired

Members: `owner-fact-recorded-unattributed` (verdict `unrelated`, searcher 8).

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  files, counted from the commit that writes this line". `git blame` of
  crates/agent-skills/docs/tripwires.md, lines 320 and 321, gives a516aa2c, "[review] Repairs of the
  decision-record re-review: the weak form's citations repointed, two premortem causes recorded as
  tripwires, and the owner keeps the closure timing", of 2026-10-29.
  `git log --diff-filter=A --format=%h a516aa2c..origin/main -- docs/retrospective-reports/` prints
  e8562915 and a9e3b3c9. This intake's commit is the third occasion.
- **Whether it fired.** The clause: "a head on the main branch states, with no attribution, a
  premise that the record of its deliberation, a plan document or a commit message, shows the owner
  reported". The attribution slips of the batch run the other way, a premise or a reason of the
  session given to the owner: "attributing to the owner a premise the session had supplied"
  (primer-import-first-line, "Items that are no finding", line 116), "briefed the implementing agent that a
  sentence was 'ruled by the owner'" (dispatch-judgement, standing, line 97), "gave the owner a
  reason the owner did not give" (split-guidelines-plan-directories, standing, line 127), the
  owner's words quoted verbatim (thaum triggers-design, standing, line 101). The subject
  `owner-attribution-slips` analyses them. None is a fact the owner reported stated unattributed in
  a head. The one fact the owner reported that a head of the batch rests near is in P3 of
  2026-11-14-knowledge-architect-planned-splits-slice, line 40: "The owner reported knowing of no
  argument for it". The head that replaced the reservation, `design@agent-skills@host-lists-are-the-session-s`,
  states no such premise: its ground is "The owner chose the lists decided by the session over
  lists reserved to the owner, which no goal ranks", an attributed choice. A head on the main
  branch is the clause's evidence, which no received file need carry: over the watch, "unfired"
  rests on the chain of standing-state reviews of the merges since the line was written, the
  reading of the subject `unfired-judged-over-branch-only`. Unfired.
- **What leaving costs.** `git grep` finds no citation of the slug outside the tripwires home.

Default: the tripwire is deleted at the commit that adds this intake's analyses, naming a9e3b3c9,
e8562915 and that commit as the occasions held through.
Outcome: decided by the session: delete it, unfired at its horizon. Ground: the horizon's third
occasion is this intake's commit; no received file reports the act the clause names;
`skill@knowledge-architect-issue-tracking@movement-instruction` deletes a tripwire whose horizon
passes unfired.

## tripwire-shape-findings-fragment-code: re-entered at this intake, unfired; its horizon's second occasion

Members: `tripwire@agent-skills@shape-findings-fragment-code` (verdict `unrelated`). Its re-entry
is "the intake of a retrospective workflow file, `skill@klarch-retrospective-intake`".

- **The horizon.** "three commits on the main branch that add analyses of received retrospective
  workflow files, counted for both decisions from the commit that writes this line". `git blame` of
  crates/agent-skills/docs/tripwires.md, line 358, gives 87fea465, "[agent-skills] The installed
  development skill, its review agents, and the local skill cut to an overlay", of 2026-11-02. Since
  it, e8562915 added analyses, by
  `git log --diff-filter=A --format=%h 87fea465..origin/main -- docs/retrospective-reports/`. This
  intake's commit is the second occasion: the horizon does not pass.
- **Whether it fired.** The clause needs, in two pieces of work, "a split that bought nothing": a
  function made for the default shape with one caller, "no test of its own and a name that says no
  more than its body", or a module split whose parts change together. The answers on rules about
  code report three extractions, each with a test of its own: "a one-line predicate be a pure
  function with a unit test; it cost one function and one test" (quick-fixes, standing, lines 89 to
  91); "split out of an I/O loop as a pure function, and that split cost one test and no rework"
  (record-search-verdicts, standing, lines 106 to 108); one finding asking for a pure function
  "was judged to need nothing" (transcript-reader-gaps, standing, line 62). Thaum's size split
  "was in the change's interest" (long-draw, standing, lines 113 and 114). Unfired.

Default: no change; the tripwire stands, its horizon at its second occasion.
Outcome: decided by the session: no change, unfired. Ground: no answer of the batch reports a
split that bought nothing as the clause defines it; the horizon's third occasion is a later intake.
