# Development and Components: the installed development skill, its review axes, its mutation tool, and the Component skill

## Status and audience

This is the milestone document of the work that ships the workflow's development activity to every
target project. After it lands, the installed text holds a development skill ported from
`skill@klarch-development` and from thaum's development skill, three new review agents and a widened
code-claims reviewer, a mutation command in a published crate, a skill for creating or splitting a
Component, a goal standard that admits an outcome judged only in use, the code-quality goal
reworded under that standard, and a design skill whose record search may wait until the scope
settles. It is written for a session that did not witness the discussion.

- It leaves the repository in the commit that deletes the spec of its last slice, once every
  harvest row is done, as `skill@knowledge-architect-planning@plan-document-leaves` says.
- Where it and a design home disagree on a point that "What is already decided" does not list as
  reversed or rewritten, the design home wins.
- Every name it uses is defined under Names or "New names, in one place", or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. No default awaits the owner: Q1 to Q9, D1 and D2 are ruled, as "Defaults awaiting the
  owner" says.

The discussion ran in one session of Claude Code, with no compaction. That session is found by its
opening message, the owner's "I have just reworked the goals of the project, in particular with the
goal of covering every activity of a target project." The discussion begins at that message and
ends at the owner's message that opens "Agreed on the table. Plan it as a milestone." This document
was assembled from that session's transcript, extracted with `cargo klarch transcript show`.

Rounds are numbered by the owner's messages: R1 is the opening message, R2 to R6 the owner's
answers to each of the agent's rounds, and R7 the closing message. "The reply to Rn" is the agent's
message that answers Rn. The record search ran during the reply to R5, and its results were
reported at the end of that reply.

## How the work is done

Each slice follows `skill@knowledge-architect-planning@working-a-slice`: read it whole before each
slice. This milestone adds:

- **Slice 0 is worked on the branch that adds these documents, before their reviews**, on the
  owner's word in R7: "The goal standard and reworded goal should be done quickly, merged with the
  milestone plan (or maybe applied before reviewing the plan), as a "slice 0". If we apply it after
  the milestone plan, the plans won't be aligned with current goals ,and review might end up giving
  conflicting resluts I fear." The plan documents' reviews then read them against the reworded goal
  and the amended goal standard. Slice 0's commits re-point every reference this milestone's
  documents make to the goal it renames.
- **Each slice after slice 0 is one branch and one pull request, merged before the next begins**, as
  `skill@knowledge-architect-planning@cutting-steps-and-slices` requires.
- **Each slice grounds the Components it edits**: `agent-skills` for the installed text and its
  heads, the root `knowledge-architect` for the goals and the release, `xtask` for the maintenance
  tool, and, from slice 1 on, `mutate`.
- **Every edit under `path@agent-skills@content/` or `path@agent-skills@fragments/`** passes the
  tests of "Editing an installed skill or agent" in `path@agent-skills@CLAUDE.md`, read whole, and is
  followed by `cargo klarch install-agent-skills`, which rewrites the installed copies under .claude.
- **Every slice writes its CHANGELOG.md entries** in the `Next release` section, then runs
  `cargo x changelog`, per `design@knowledge-architect@the-branch-writes-its-changelog-entries`, with
  the class `design@knowledge-architect@versioning-policy` gives each change.
- **Every Rust change follows `skill@klarch-development`** until slice 2 lands; from then on, the
  installed development skill with `skill@klarch-development` as the overlay slice 2 leaves.

## Names

| name | what it names |
| --- | --- |
| the local development skill | `skill@klarch-development`, this repository's own skill for its Rust source |
| thaum | the project at `path@elsewhere@~/dev/games/thaum/` on the owner's machine, a consumer of knowledge-architect that implements a card game's rules; its skills thaum-development and thaum-review, and its tool tools/mutate, are read by this work |
| thaum's mutation tool | `path@elsewhere@~/dev/games/thaum/tools/mutate/`: its src/lib.rs (404 lines), src/main.rs (1066 lines) and README.md (147 lines), counted with `wc -l` in thaum's checkout |
| `run`, `restore` | the two generic modes of thaum's mutation tool: `run` applies one mutation the session writes, by `--replace OLD --with NEW` or `--patch`, runs a command against it and restores the file; `restore` unwinds a run that was killed; in thaum's main.rs, `run_hand_mutation`, `unwind`, `restore_record` and `apply_to_scratch` |
| `sweep` | thaum's third mode, which deletes each invariant-check site under one directory of thaum's engine; thaum's own, not taken in |
| cargo-mutants | the external mutation tool for Rust, which generates its own mutants; its option `--in-diff FILE` restricts it to mutants in changed code |
| the goal standard | `path@agent-skills@fragments/goal-standard.md`, hosted by `skill@knowledge-architect-goal-setting@goal-or-not` and `agent@knowledge-architect-goal-reviewer@the-goal-standard` |
| rule 6 | rule 6 of the goal standard, "What shows it met is in what the project builds" |
| the new clause | the sentence #goal-standard-measure adds to rule 6, written by slice 0 into rule 6 of `path@agent-skills@fragments/goal-standard.md` |
| the code-quality goal | `goal@knowledge-architect@the-workflow-leads-to-better-code`, named `code-has-one-responsibility-per-unit-and-is-testable` before slice 0 reworded and renamed it |
| the development skill | the installed skill slice 2 ships, `knowledge-architect-development` |
| the component skill | the installed skill slice 3 ships, default name `knowledge-architect-component-creation` |
| the fragment "when to split" | the shared fragment slice 3 adds, hosted by the design skill and the development skill |
| the mutation crate | the published crate slice 1 adds: Component `mutate`, package `knowledge-architect-mutate` |
| the maintenance crate | a project's crate that holds its repeated tasks, run as `cargo x`; here `path@knowledge-architect@tools/xtask/`, per `design@agent-skills@xtask-pins-checker` |
| a declared path | a code path a target project declares performance-critical in one of its design heads, with the command that measures it, per #performance-opt-in |
| the search | the record search of `agent@knowledge-architect-record-searcher`, which the design skill dispatches at its loop step 1 |
| Q1 | the question whether thaum's mutation tool is taken in, ruled R3; see "Defaults awaiting the owner" |
| Q2 | the question whether a declared path is a design head or a manifest key, ruled R7; see "Defaults awaiting the owner" |
| Q3 | the question whether the generic `run` and `restore` ship as a command of a published crate, ruled R3; see "Defaults awaiting the owner" |
| Q4 | the question whether the harness issue is handled in this milestone, ruled R6; see "Defaults awaiting the owner" |
| Q5 | the question whether slice 0's design audit reuses the discussion's record search, ruled after the plan's review; see "Defaults awaiting the owner" |
| Q6 | the question of the force of the default shape on code a change writes or edits, ruled during slice 0; see "Defaults awaiting the owner" |
| Q7 | the question of which text states the content of the default shape, ruled during slice 0; see "Defaults awaiting the owner" |
| Q8 | the question whether a lint the setup ships is an installed rule about code, ruled during slice 0; see "Defaults awaiting the owner" |
| Q9 | the question of which text states that a Component takes decisions of its own, ruled during slice 0; see "Defaults awaiting the owner" |
| D1 | the default on whether cargo-mutants is an ask of every target project, ruled at the plan's review; see "Defaults awaiting the owner" |
| D2 | the default on how the development skill names the mutation command in a Rust project with no maintenance crate, ruled at the plan's review by a new ask, #maintenance-crate-ask; see "Defaults awaiting the owner" |

## What the work is

**Today**, at each site the work touches:

| site | what it holds today |
| --- | --- |
| `skill@klarch-development` | this repository's own development skill, 235 lines: grounding, the loop, claims and tests that discriminate with a manual scratch-worktree mutation procedure, one review axis (adversarial), the gate, incompleteness recorded, comment species. Nothing installed covers the activity |
| thaum's development and review skills | the same procedure with refinements: each adversarial reviewer scoped to one subsystem, claim family or boundary; the reviewer picks its tests and runs the wider suite before reporting a survivor; a rules-first axis for its external specification; `cargo mutate run` and `restore`; bench runs before and after work that could change performance |
| thaum's mutation tool | `run`, `restore` and `sweep`; thaum-specific parts: the `THAUM_CHECKOUT` build tie in lib.rs, `sweep`, the record directory `path@elsewhere@~/dev/games/thaum/tools/mutate/local/current/` that `record_dir` returns, and `project_root`, which finds the root by the checker's manifest through the knowledge-architect crate |
| rule 6 of the goal standard | the measure is "an observable of the tree as it stands"; evidence from use "is not the measure"; where the outcome is judged only in use, the observable is what the project builds to pursue it and to learn from its use whether it is met, and a list of the properties the outcome is taken to consist of is no such observable: slice 0 added that clause |
| `agent@knowledge-architect-goal-reviewer@what-to-report` | "A finding of rule 4 or of rule 6 is also tagged `minor`", except a finding that a goal judged only in use is measured by a list of the properties of its outcome, which "is tagged `intent` instead of `minor`": slice 0 added the exception |
| the code-quality goal | `goal@knowledge-architect@the-workflow-leads-to-better-code` defines no quality: met while the installed text "states a default shape for the code a change writes, and has a review of code weigh each branch's changes to Rust code against it, at the latest at the review before the branch merges", and while the installed retrospective skill asks whether a rule or guideline about code or tests proved too hard or too strict; slice 0 reworded and renamed it, from a goal that required three properties of every unit a change writes or edits |
| `agent@knowledge-architect-code-claims-reviewer` | reads plan documents only |
| `skill@knowledge-architect-review@review-axes` | no axis for code: the adversarial axis is `skill@klarch-development`'s own |
| `skill@knowledge-architect-setup` | recommends a gates command whose gates include "the linter, where the project has one"; names no lint |
| `skill@knowledge-architect-retrospective@what-it-examines` | its standing questions hold none on the rules about code |
| `skill@knowledge-architect-design@discussion-loop`, step 1 | dispatches the search after the first challenge, per `design@agent-skills@search-after-first-challenge` |
| Components | `goal@knowledge-architect@projects-are-split-into-components` asks the installed text for default criteria for placing work in Components and for splitting them, as guidelines, and asks the installed retrospective skill whether a rule or guideline about splitting proved too hard or too strict, as slice 0 reworded it; no installed text says how to create or split a Component beyond declaring it, in `skill@knowledge-architect-setup@components-and-locations`; `a-skill-for-creating-a-component`, closed when this milestone scheduled it, held the question, which #component-crate-mapping and #component-skill-delivery carry |

**After**, the work as five slices, in order: the goal standard and the reworded goal; the mutation
crate; the development skill and its axes; the component skill; the search timing. The slices are
listed under "Implementation sequence".

**Outside the work:**

- **Branch work and releasing**, the other two activities of
  `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`, which
  stays open for them. The development skill stops at the commit and the review; it states no pull
  request and no continuous integration, per criterion #no-pr-or-ci-assumed.
- **Improving performance as an activity of its own**, per #performance-review-axis: slice 2 opens a
  `todo` issue for it.
- **Collecting feedback from real use in a target project**, per #use-feedback-collection: slice 2
  opens a `todo` issue for it.
- **Co-development of a shipped tool through a Cargo `[patch]`**, per #upstream-co-development: slice
  2 opens a `deferred` issue for it.
- **How shared fragments are delivered**, per #harness-argued-heads-scope:
  `issue@agent-skills@heads-argued-from-another-harness` stays open, its re-entry narrowed at slice 0.
- **A recorded list of mutations**, thaum's `sweep` generalised: it waits for a second real use
  (`argument@development-and-components@arg-37`).
- **A manifest key for a declared path**: the declaration is a design head, and a manifest key comes
  only if sessions miss the step in real use (Q2).

## What is already decided

**Rested on and not argued again:** `design@agent-skills@additions-review-agent` and
`rejected@agent-skills@one-generic-auditor-agent`, which make each new axis an installed agent;
`design@agent-skills@installed-fix-goes-upstream` and `primer@met-outside-the-task`, which word
#upstream-tool-loop; `rejected@gates@gates-command-of-the-checker`, which keeps the mutation command
out of the checker's binary; `design@gates@gates-crate`, whose purpose keeps it out of the gates
library; `design@knowledge-architect@no-operation-loses-content`, which bounds the mutation tool;
`design@agent-skills@settled-means-recorded`, which sends a change the owner asks for or proposes to the design skill, and its search, unless a record the session can see states it, so that #fix-grounding needs no search per fix; `design@agent-skills@shared-text-fragment`, the mechanism of the fragment
"when to split"; `design@agent-skills@naming-rule`, which names the new skills;
`design@core@anchor-collisions-refused` and the refusal of two anchors with one name in
`resolve_anchors` of `path@core@src/manifest.rs`, which #component-basename-collision documents;
`design@knowledge-architect@no-component-directory-named-after-the-project`, which #component-naming
recommends; `design@agent-skills@rust-templates-one-home`, which tests the maintenance crate's
snippet; `design@agent-skills@behaviour-additions-need-real-use`, which asks each new instruction for
its evidence; `design@agent-skills@tripwire-recording-authority` and
`design@agent-skills@tripwire-admission-test`, which govern the premortem's harvest.

**Reversed or rewritten**, each with the texts `cargo klarch show` lists as citing it, and the slice
that judges or updates each:

| entry | change | slice | citing texts, and what happens to each |
| --- | --- | --- | --- |
| `design@agent-skills@goal-standard-rules` | rule 6 gains the new clause, which adds to its rule and reverses nothing; a finding under the new clause is tagged `intent` instead of `minor` | 0 | `design@agent-skills@goal-entry-shape`, `design@agent-skills@goal-reviewer-agent`, `design@agent-skills@shared-text-fragment`: reread at slice 0, the second rewritten there |
| `design@agent-skills@goal-reviewer-agent` | the `intent` tag of #rule-6-finding-weight | 0 | `skill@knowledge-architect-goal-setting@drawing-out-intent`, reread at slice 0; `design@agent-skills@goals-axis-in-the-review-before-a-merge`, reread at slice 0; `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`, updated or closed at slice 0 |
| `goal@knowledge-architect@the-workflow-leads-to-better-code`, named `code-has-one-responsibility-per-unit-and-is-testable` before slice 0 | reworded and renamed under `skill@knowledge-architect-goal-setting` | 0 | `goal@knowledge-architect@agents-get-a-complete-workflow` and `goal@knowledge-architect@projects-are-split-into-components`, in `register@knowledge-architect@goal`, and `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: each reference re-pointed at slice 0, and each sentence that leans on the old body reread there; this milestone's own documents, re-pointed at slice 0 |
| `goal@knowledge-architect@projects-are-split-into-components` | reworded at slice 0 on the owner's word: the default criteria are guidelines that real use corrects, a Component with no Rust code of its own is outside the rule of one purpose, and the installed retrospective skill asks whether a rule or guideline about splitting proved too hard or too strict | 0 | `goal@knowledge-architect@agents-get-a-complete-workflow`, in `register@knowledge-architect@goal`, and `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`: reread at slice 3, which builds what the goal's first "met while" item asks; `a-skill-for-creating-a-component`, closed when this milestone scheduled it, carried by #component-crate-mapping; the retrospective clause of its second item is written at slice 2, per #retrospective-code-rules-question |
| `design@knowledge-architect@publish-credential` | "Each of the three crates" becomes four; the mutation crate's first version is published with an API token, outside the release workflow | 1 | `design@knowledge-architect@owner-word-mechanism`, which states that the trusted publisher of each crate names the environment `release`, and the comment at the head of `path@knowledge-architect@.github/workflows/release.yml`, which names "the three crates": each reread at slice 1 and updated where the fourth crate makes it false; `skill@klarch-release`, whose scope names the three published crates, gains the fourth at slice 1 |
| `design@knowledge-architect@versioning-policy` | its sentence "a release that changes only skill text publishes all three crates" counts the published crates, which the mutation crate makes four | 1, at the harvest | `skill@klarch-release`, whose versioning sentence names "the three crates": updated at slice 1 with `design@knowledge-architect@publish-credential`'s row above; `agent@klarch-changelog-reviewer`, `instructions@git-workflow` and `instructions@release-status` in the root `CLAUDE.md`, `design@knowledge-architect@changelog-entries`, `design@core@api-facade`, `design@core@ne-minimal`, `issue@core@manifest-table-exposes-the-toml-crate`, `issue@knowledge-architect@command-output-is-not-declared-a-contract` and `rejected@xtask@source-view-exported-from-the-library`: each cites its bump table, which does not change, so none is touched; this milestone's own documents name it as the class of each change |
| `design@agent-skills@pinned-binary-runs-inside-the-project` | it admits, on its owner's ruling, a Rust project that installs the checker as any other project does and builds its own gates, with no maintenance crate; #maintenance-crate-ask makes the maintenance crate an ask of every Rust target project, so that admission goes | 1 | `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`, whose list states that this head admits the local install: updated at slice 1, which also adds the ask to that list, per `spec@development-and-components@mutation-crate` Builds item 8; this milestone's own documents, which name it as rewritten at slice 1 |
| `design@agent-skills@xtask-pins-checker` | it speaks of "a Rust project with a maintenance crate", and every Rust target project now has one, per #maintenance-crate-ask | 1 | `design@agent-skills@pinned-binary-runs-inside-the-project`, rewritten at slice 1 in the row above; `design@agent-skills@gates-convention`, whose title and body say "where the project has a maintenance crate", and `design@core@rust-scaffold`: reread at slice 1, each sentence that the ask makes false rewritten; this milestone's own documents, which name it as rewritten at slice 1 |
| `design@knowledge-architect@scratch-worktree-outside-git-rules` | its example, the worktree mutation procedure of `skill@klarch-development`, is replaced by `cargo x mutate` | 2 | `instructions@git-workflow`, in the root `CLAUDE.md`, `design@knowledge-architect@git-flow` and `design@knowledge-architect@no-operation-loses-content`: reread at slice 2, each sentence naming the worktree procedure rewritten |
| `design@agent-skills@plan-read-against-the-record` | it places the code-claims reviewer among the reviewers of a plan document; #code-claims-scope also sends it on a diff's statements about code | 2 | `skill@knowledge-architect-planning@plan-reviews`, `design@agent-skills@record-searched-before-the-work`, `design@agent-skills@unread-entries-reach-the-conformance-review`, `issue@agent-skills@audit-axes-beyond-the-design-record`: reread at slice 2; `the-code-claims-axis-reads-plan-documents-only`, closed when this milestone scheduled it: carried by #code-claims-scope, built at slice 2 |
| `design@agent-skills@code-change-reads-its-record` | its last sentence names #fix-grounding, which answers the question of a search per fix, as decided and not built; slice 2 states it as built | 2 | the primer's subsection "Read the record of code before changing it", `design@agent-skills@design-home-write-loads-recording`, `design@agent-skills@primer-limit`, `design@agent-skills@verification-in-primer`, `design@agent-skills@record-searched-before-the-work`, `design@agent-skills@a-head-is-owed-by-an-entry-test`: reread at slice 2; `a-fix-is-grounded-by-no-record-search`, closed when this milestone scheduled it: carried by #fix-grounding, built at slice 2 |
| `design@agent-skills@premortem-as-watch-points` | its standing questions, each with the decision or the goal it watches, gain #retrospective-code-rules-question, which watches `goal@knowledge-architect@the-workflow-leads-to-better-code` and `goal@knowledge-architect@projects-are-split-into-components` | 2 | `design@agent-skills@installed-fix-goes-upstream`, `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` and `issue@agent-skills@patching-an-installed-skill`: reread at slice 2; each cites a question the new one leaves as it is; this milestone's own documents, which name it as rewritten at slice 2 |
| `design@agent-skills@search-after-first-challenge` | the search may wait until the scope settles, and no thread closes before it has run | 4 | `design@agent-skills@record-searched-before-the-work`, rewritten at slice 4 |
| `design@agent-skills@bounded-path-in-design` | it classifies bounded work only after the grounding, "its search for standing entries included": a delayed search delays the classification, so no one-message bounded proposal is sent before the search has run | 4 | `design@agent-skills@settled-means-recorded`, `design@agent-skills@record-searched-before-the-work`, `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`, `issue@agent-skills@the-open-space-test-may-admit-every-problem`, `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`, `tripwire@agent-skills@a-shortcut-decision-earns-a-head`: reread at slice 4 |
| `design@agent-skills@record-searched-before-the-work` | its sentence on when the design skill dispatches the search follows #search-timing; its "Other work that does not go through the design skill sends no search" stands, per #fix-grounding | 4 | `design@agent-skills@code-change-reads-its-record`, `design@agent-skills@search-after-first-challenge`, `design@agent-skills@standing-state-review-before-every-merge`, `design@agent-skills@unread-entries-reach-the-conformance-review`, `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`, `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`, `tripwire@agent-skills@record-search-misses`: reread at slice 4; `a-fix-is-grounded-by-no-record-search`, closed when this milestone scheduled it: carried by #fix-grounding, built at slice 2 |

**The entries the discussion's search graded `bears`**, 136, with their outcome. The search ran
during the reply to R5, with 18 searchers over the whole record; every report passed
`cargo klarch worklist --verify`. A goal was read by its grades, the other entries whole.

| entry | outcome |
| --- | --- |
| `goal@agent-skills@installed-text-works-in-any-environment` | constrains; read whole at grounding |
| `goal@agent-skills@installed-text-works-in-every-target-project` | constrains; the source of criterion #works-without-performance-or-spec |
| `goal@agent-skills@no-contradiction-in-workflow` | constrains; slice 3's skill points to the setup skill and restates none of it (`argument@development-and-components@arg-48`) |
| `goal@agent-skills@one-skill-per-activity` | constrains, and may change: slice 3's skill covers creating a Component, which its audit checks against the goal's list of activities |
| `goal@agent-skills@only-needed-instructions` | constrains; the source of criterion #only-needed-instructions-with-evidence |
| `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` | constrains; read whole at grounding |
| `goal@gates@gates-from-a-list` | constrains; read whole at grounding: the mutation command is no gate (`argument@development-and-components@arg-77`) |
| `goal@knowledge-architect@adoption-is-easy` | constrains; read whole at grounding |
| `goal@knowledge-architect@agents-get-a-complete-workflow` | constrains; slice 2 delivers its developing activity, and the source of criterion #no-pr-or-ci-assumed |
| `goal@knowledge-architect@agents-work-without-drift` | constrains; read whole at grounding |
| `goal@knowledge-architect@the-workflow-leads-to-better-code`, named `code-has-one-responsibility-per-unit-and-is-testable` before slice 0 | reworded and renamed at slice 0, per #code-quality-goal-wording; the source of criterion #code-shape-is-reviewed |
| `goal@knowledge-architect@design-is-recorded-with-its-arguments` | constrains; read whole at grounding |
| `goal@knowledge-architect@design-with-agents-as-counterparts` | constrains #search-timing; read whole at grounding |
| `goal@knowledge-architect@documentation-stays-consistent` | constrains; read whole at grounding |
| `goal@knowledge-architect@every-target-project-can-adopt-it` | constrains; read whole at grounding |
| `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` | constrains, and may change at slice 2: the generic part of `skill@klarch-development` and of thaum's mutation tool is taken in (Q1, Q3), and the local copies go once it ships |
| `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` | constrains; the source of criterion #mechanical-step-is-a-command |
| `goal@knowledge-architect@owner-attention-on-what-is-theirs` | constrains; read whole at grounding |
| `goal@knowledge-architect@projects-are-split-into-components` | constrains; slice 3 delivers how the installed text leads there, and slice 2 writes its retrospective clause, per #retrospective-code-rules-question |
| `goal@knowledge-architect@serves-one-kind-of-project` | constrains slice 1 and slice 2: the lints of #api-doc-completeness are an ask of every target project, which the owner approved with that thread in the checkpoint table of the reply to R6, by R7; cargo-mutants per D1 |
| `goal@knowledge-architect@setup-brings-quality-tools` | constrains slice 1 and slice 2: the mutation command and the lints are quality tools the setup brings |
| `goal@knowledge-architect@structure-and-workflow-work-together` | constrains; read whole at grounding |
| `goal@knowledge-architect@the-owner-decides` | constrains; read whole at grounding |
| `goal@knowledge-architect@the-owner-s-word-is-verified` | constrains slice 0, which rewords a goal: its premises are checked |
| `goal@knowledge-architect@the-workflow-improves-through-real-use` | constrains #retrospective-code-rules-question and the reworded goal's mechanism; read whole at grounding |
| `goal@xtask@repeated-tasks-are-automated` | constrains slice 1: `cargo x mutate` in this repository's maintenance tool |
| `design@agent-skills@primer-limit` | reread at slice 2's audit: the development skill points to the primer's "Read the record of code before changing it" and does not move it out, since every session that changes code still meets that moment |
| `design@agent-skills@verification-in-primer` | constrains slice 2: the development skill restates no rule of `primer@verify-before-relying` beyond one sentence with its pointer, as #commit-claim-check does |
| `design@agent-skills@code-change-reads-its-record` | the reading #fix-grounding keeps for a fix; its last sentence, which names #fix-grounding as decided and not built, is rewritten at slice 2's harvest |
| `design@agent-skills@met-outside-the-task-outcomes` | constrains #upstream-tool-loop: the shipped text words a local improvement as a tool of the project's own beside the shipped one, never a fix of the shipped one (`argument@development-and-components@arg-57`); judged at slice 2's audit |
| `design@agent-skills@restatement-size-test` | constrains slices 2 and 3: a primer directive is restated in one sentence with its pointer at most; the "when to split" text is a fragment (#component-skill-delivery) |
| `design@agent-skills@context-cost-of-agent-text` | constrains slices 2 and 3: the fragment and the new agents are weighed in the context of the sessions that load them |
| `design@agent-skills@overlay-by-separate-skills` | constrains slice 2: `skill@klarch-development` and thaum-development stay project skills that add to the installed one (`argument@development-and-components@arg-64`) |
| `design@agent-skills@installed-skill-takes-precedence` | constrains slice 2: the primer's list of installed skills gains the development skill, and slice 3 the component skill |
| `design@agent-skills@pinned-binary-runs-inside-the-project` | constrains slice 1: the mutation command runs through the project's maintenance crate, built from the project; rewritten at slice 1, per #maintenance-crate-ask, as "Reversed or rewritten" says |
| `design@agent-skills@gates-convention` | constrains slice 2: the gate step runs the one gates command; the documentation lints join the linter the gates run |
| `design@agent-skills@xtask-pins-checker` | constrains #mutation-command-host: the maintenance crate carries "the project's other repeated tasks", `cargo x mutate` among them (`argument@development-and-components@arg-66`); rewritten at slice 1, per #maintenance-crate-ask, as "Reversed or rewritten" says |
| `design@agent-skills@goals-required` | constrains slice 3: the creation procedure leaves each new Component with a goal, through goal-setting |
| `a-fix-is-grounded-by-no-record-search` | closed when this milestone scheduled it; carried by #fix-grounding, built at slice 2 |
| `issue@agent-skills@a-head-s-measurement-says-nothing-of-its-magnitude` | closed by slice 2 if #performance-opt-in's head carries out its proposal, as the searcher graded; read whole at slice 2's audit |
| `a-skill-for-creating-a-component` | closed when this milestone scheduled it; carried by #component-crate-mapping, built at slice 3; its premises are those #component-crate-mapping restates (`argument@development-and-components@arg-62`) |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | touched by slice 2, whose installed skill now covers the activity of `skill@klarch-development`; reread at slice 2's audit |
| `issue@agent-skills@patching-an-installed-skill` | constrains slice 2: the project skills become overlays that may only add (`argument@development-and-components@arg-64`); stays open |
| `issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition` | evidence for #search-timing (`argument@development-and-components@arg-61`); read at slice 4's audit |
| `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter` | touched by slice 2's lint ask and slice 1's setup snippet, new asks of a target project; reread at slice 2's audit |
| `rejected@agent-skills@one-generic-auditor-agent` | defeats a briefed general-purpose reviewer: #code-shape-axis and #spec-first-axis are installed agents (`argument@development-and-components@arg-57`, `argument@development-and-components@arg-68`) |
| `issue@agent-skills@heads-argued-from-another-harness` | its re-entry is narrowed at slice 0, per #harness-argued-heads-scope |
| `design@core@a-commit-message-is-a-document` | constrains slice 2's commit step, which runs `commits` after the commit |
| `design@core@branch-shas-are-refused` | constrains slice 2's commit step, which names the range of the after-commit check |
| `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit` | its re-entry is met; slice 2 restates the claim check in its commit step, and the issue stays open for its other options (#commit-claim-check) |
| `issue@core@branch-sha-citations-are-judged-within-the-range-only` | slice 2's commit step states the range as the installed text states it today; the issue stays open |
| `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve` | touched by slice 3's creation procedure, which writes a Component's goals through goal-setting; reread at slice 3's audit |
| `issue@knowledge-architect@the-retrospective-asks-for-no-tool-a-session-set-up` | touched by #retrospective-code-rules-question and #upstream-tool-loop; reread at slice 2's audit |
| `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing` | narrowed when this milestone scheduled its developing part, which slice 2 builds; branch work and releasing stay open |
| `design@agent-skills@bounded-path-in-design` | rewritten at slice 4: bounded work is classified after a grounding whose search may now wait, so a delayed search delays the classification (#search-timing) |
| `design@agent-skills@settled-means-recorded` | constrains slice 2: a change that decides something goes to the design skill before the development skill builds it (`argument@development-and-components@arg-58`) |
| `design@agent-skills@new-or-reshaped-head-needs-design` | constrains slice 2's implement step, which routes a decision met while building by the backstop |
| `design@agent-skills@existing-heads-on-touch` | constrains every slice's harvest: a head the change touches is brought to the rules on heads |
| `design@agent-skills@design-home-write-loads-recording` | constrains every slice's harvest, which writes heads under decision-recording |
| `design@agent-skills@outcome-over-display` | constrains slice 4's change of the design skill |
| `issue@agent-skills@a-lighter-pass-for-untouched-components` | holds premortem cause 6 (#component-crate-mapping); stays open |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | touched by #retrospective-code-rules-question, the channel it names; read at slice 2's harvest, where premortem causes 1 and 2 are judged |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` | reread at slice 2's audit: the development skill's grounding reads the record of the code, not every issue's re-entry |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | the instrument of #review-cadence (`argument@development-and-components@arg-10`) and premortem cause 3; stays open |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | constrains slice 2's port: the shipped text cites no entry of this repository; reread at slice 2's audit |
| `design@agent-skills@instruction-record-is-minimal` | constrains every slice's harvest |
| `design@agent-skills@behaviour-additions-need-real-use` | constrains slices 2 to 4: each new instruction names its real-use evidence (`argument@development-and-components@arg-64`) |
| `design@agent-skills@no-size-budget` | constrains slices 2 and 3 |
| `design@agent-skills@synthetic-evidence-not-built` | constrains every slice's claims: no run observes an agent under the new text |
| `design@agent-skills@capabilities-not-structure` | constrains slice 2's step sequence and #review-cadence |
| `design@agent-skills@expectation-set-bounds-scope` | reread at slice 2's and slice 3's audits, for whether the new skills state an expectation set |
| `design@agent-skills@statement-names-an-act` | constrains slices 2 and 3 |
| `design@agent-skills@term-defined-at-first-load` | constrains slices 2 and 3: each coined term is defined in the text that coins it |
| `design@agent-skills@admission-rules-home` | constrains slices 2 and 3 |
| `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` | reread at slice 4's audit, which changes when the design skill's search runs |
| `a-reviewer-s-running-time-is-unbounded` | closed when this milestone scheduled it; carried by #adversarial-scoping, built at slice 2 (`argument@development-and-components@arg-70`, `argument@development-and-components@arg-78`) |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | reread at slice 2's and slice 3's audits |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | constrains #performance-opt-in, whose declaration is a design head of the target project, and every slice's harvest |
| `design@agent-skills@losing-alternatives-filter` | constrains every slice's harvest of the losing alternatives |
| `design@agent-skills@losing-shape-test` | constrains the harvest of #performance-review-axis and the other alternatives with nothing built in their place |
| `design@agent-skills@thread-slug-is-entry-id` | constrains every slice's harvest: a thread's slug is checked for collision before it becomes an entry's slug |
| `design@agent-skills@step-is-no-commit` | constrains slice 2's commit step |
| `design@agent-skills@completeness-check-is-a-step-claim` | constrains slice 2's claims step |
| `design@agent-skills@review-repair-appended-or-folded` | constrains slice 2's review step |
| `design@agent-skills@undesigned-work-is-an-issue` | constrains slice 2's "incompleteness is recorded" section |
| `issue@agent-skills@installed-text-assumes-pull-requests-and-ci` | constrains slice 2: no PR or CI wording (criterion #no-pr-or-ci-assumed); stays open |
| `design@agent-skills@content-mirrors-the-install-layout` | constrains slices 2 and 3: the new files' places |
| `design@agent-skills@shared-text-fragment` | constrains slice 3's fragment, whose host list is the design and development skills (#component-skill-delivery); stays as it is, per #harness-argued-heads-scope |
| `design@agent-skills@rust-templates-one-home` | constrains slice 1 (the maintenance crate's snippet gains the mutation command) and slice 2 (the lints, where a snippet carries them) |
| `design@agent-skills@shipped-text-line-comments` | constrains slices 0, 2 and 3: reasons and citations go in `%%` lines |
| `design@agent-skills@shipped-text-cites-no-entry` | constrains slice 2's port of `skill@klarch-development` |
| `design@agent-skills@no-external-handoff` | constrains slices 2 and 3: they hand off to installed skills and agents only |
| `design@agent-skills@naming-rule` | constrains the names of slice 2's and slice 3's skills, checked at their audits |
| `design@core@section-homes-carry-slugs` | constrains slices 2 and 3: every level-two heading of a new skill or agent carries a slug |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | touched by slice 0, which edits rule 6 that the goal reviewer cites by number; stays open |
| `design@knowledge-architect@versioning-policy` | constrains every slice's changelog entries; rewritten at slice 1's harvest where it counts the published crates, as "Reversed or rewritten" says |
| `design@knowledge-architect@changelog-entries` | constrains every slice's changelog entries |
| `design@knowledge-architect@the-branch-writes-its-changelog-entries` | constrains every slice |
| `design@agent-skills@a-reference-claims-a-revisit` | constrains slice 2's comment species, which points to the primer and restates nothing (`argument@development-and-components@arg-64`) |
| `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions` | constrains slice 2: the comment species does not restate the guard-reference rule (`argument@development-and-components@arg-64`); stays open |
| `design@agent-skills@plan-read-against-the-record` | rewritten at slice 2: the code-claims reviewer is also sent on a diff's statements about code (#code-claims-scope) |
| `design@agent-skills@transcript-review-last-before-merge` | constrains #review-cadence: the new read-only axes run before the transcript reviewer |
| `design@agent-skills@a-scratch-directory-per-subagent` | constrains #adversarial-scoping: each scoped reviewer gets its own scratch directory and worktree |
| `design@agent-skills@additions-review-agent` | the ground of #code-shape-axis and #spec-first-axis as installed agents (`argument@development-and-components@arg-57`, `argument@development-and-components@arg-68`) |
| `design@agent-skills@goals-axis-in-the-review-before-a-merge` | constrains slice 0, which rewords a goal: its review owes the goals axis |
| `the-code-claims-axis-reads-plan-documents-only` | closed when this milestone scheduled it; carried by #code-claims-scope, built at slice 2 |
| `design@core@agents-table` | constrains slice 3: each new Component carries a `CLAUDE.md` under the `claude` harness |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | constrains slice 2: the development skill names no scoped `CLAUDE.md` (`argument@development-and-components@arg-64`); stays open |
| `design@agent-skills@premortem-as-watch-points` | rewritten at slice 2: the new standing question joins its list and names the goals it watches (#retrospective-code-rules-question), as "Reversed or rewritten" says |
| `design@agent-skills@retrospective-destination` | constrains #upstream-tool-loop's report to the workflow's file |
| `design@agent-skills@installed-fix-goes-upstream` | the ground of #upstream-tool-loop's wording (`argument@development-and-components@arg-57`, `argument@development-and-components@arg-68`); stands |
| `issue@agent-skills@an-item-kept-for-the-retrospective-is-lost-when-no-retrospective-follows` | touched by #upstream-tool-loop, which sends one more kind of item to the workflow's file; stays open, reread at slice 2's audit |
| `design@agent-skills@design-home-is-built-intent` | constrains slice 2's grounding step |
| `design@agent-skills@head-prevails-over-diverging-code` | constrains slice 2's grounding step |
| `design@agent-skills@divergence-routed-by-its-authority` | constrains slice 2's grounding step |
| `design@agent-skills@local-intent-binds` | constrains slice 2's comment species and #code-shape-axis's departures argued at the code |
| `design@agent-skills@standing-argument-in-head` | constrains #performance-opt-in's head in a target project: it names its measure command |
| `design@agent-skills@search-after-first-challenge` | rewritten at slice 4, per #search-timing |
| `design@agent-skills@record-searched-before-the-work` | rewritten at slice 4 where it states the order; slice 2 keeps its "Other work that does not go through the design skill sends no search" (#fix-grounding) |
| `design@core@component-command` | constrains slice 3's procedure, which runs `klarch new-component` |
| `design@core@components-carry-the-same-documents` | constrains slice 3's shapes: each Component carries the same documents |
| `design@core@anchor-collisions-refused` | constrains #component-basename-collision and nested Components; the refusal of two anchors with one name is in `resolve_anchors` (`argument@development-and-components@arg-75`) |
| `design@knowledge-architect@no-operation-loses-content` | constrains slice 1: no run loses uncommitted content (claims of `spec@development-and-components@mutation-crate`) |
| `design@knowledge-architect@scratch-worktree-outside-git-rules` | rewritten at slice 2, when its example, the worktree procedure of `skill@klarch-development`, is replaced by `cargo x mutate` |
| `issue@agent-skills@no-reviewer-writes-its-own-report-file` | touched by slice 2's three new reviewer agents; stays open |
| `design@agent-skills@goal-entry-shape` | constrains slice 0's reworded goal |
| `design@agent-skills@goals-change-through-goal-setting` | constrains slice 0: the goal is reworded under goal-setting only |
| `design@agent-skills@goal-standard-rules` | rewritten at slice 0, per #goal-standard-measure and #rule-6-finding-weight |
| `design@agent-skills@goal-draft-shows-reach` | constrains slice 0's draft |
| `design@agent-skills@goal-reviewer-agent` | rewritten at slice 0, per #rule-6-finding-weight |
| `design@agent-skills@goal-edit-ruled-alone` | constrains slice 0: the reworded goal is ruled as its own item |
| `design@agent-skills@goal-batch-ruled-in-the-tree` | constrains slice 0: one goal, drafted in the conversation |
| `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect` | its re-entry is met; updated or closed at slice 0, per #rule-6-finding-weight |
| `design@knowledge-architect@workflow-findings-stay-here` | constrains #upstream-tool-loop in this repository: here a finding on the workflow stays here |
| `design@agent-skills@tripwire-recording-authority` | constrains the premortem's harvest |
| `design@agent-skills@tripwire-admission-test` | constrains the premortem's harvest: condition 2 for causes 1 and 2, whose evidence arises in target projects |

**The entries the search graded `may-bear`**, 127. Four were read whole in the reply to R5:

- `rejected@agent-skills@conformance-reader-at-the-checkpoint`: a reader at the checkpoint "reads
  rulings already given"; it is evidence against a later search, and the synthesis of #search-timing
  answers that cost by leaving threads `presumed-settled` until the search has run
  (`argument@development-and-components@arg-62`, `argument@development-and-components@arg-63`). Read
  again at slice 4's audit, with `tripwire@agent-skills@record-search-misses`, which it names.
- `rejected@agent-skills@delta-search-at-convergence`: a second search, of the entries changed since
  the first. The synthesis keeps one search and moves it, so it does not bring this design back.
  Read again at slice 4's audit.
- `rejected@core@split-enforced-by-a-check`: a check failing the tree on a record cluster above the
  bound. #component-split-signals add no check; the entry bears only on premortem cause 6, where
  many Components multiply the clusters. Read again at slice 3's audit.
- `rejected@gates@gates-command-of-the-checker`: "It would make the document checker run cargo, the
  linter and the tests"; it defeats the checker's binary as the host of the mutation command
  (`argument@development-and-components@arg-57`, `argument@development-and-components@arg-66`).

The other 123 were not read; each is listed with its searcher's reason, for the
design-conformance review:

- `design@agent-skills@knowledge-table-home`: not read. #performance-opt-in and #upstream-tool-loop place new kinds of statement (a performance-critical path with its benchmark command; a local improvement of a shipped tool kept until it ships) whose home the knowledge table's rows decide; both seem to fit existing rows (design home; the retrospective's workflow file), so no new row is established.
- `design@agent-skills@branch-commits-judged-at-tip`: not read. the development skill's "commit" step, and the branch-work activity the issue names, could restate when a repair is folded into an earlier commit; the head keeps that rule in the primer because each skill meets one case; no restatement is drafted yet.
- `design@agent-skills@questions-go-to-their-authority`: not read. #spec-first-axis derives what an external specification requires before reading code, treating the spec's text as the authority this head routes reading questions to; #mutation-command and lints in the setup ask go to the owner as taken-in parts/asks; no conflict with the routing established.
- `design@agent-skills@owner-word-holds-as-far-as-its-premise`: not read. #spec-first-axis compares code with a specification; the head says "a departure from a specification is a scope decision the project states", so the axis must tell a stated departure from a defect; whether the axis's design reads departures is not drafted.
- `design@agent-skills@one-home-for-head-rules`: not read. cites seed `design@agent-skills@shared-text-fragment`, which #component-skill-delivery uses for the "when to split" fragment; #performance-opt-in creates a kind of head (a performance-critical path naming its benchmark command) that the head rules this entry homes must admit; no conflict with where the rules live is established.
- `design@agent-skills@no-routing-table`: not read. once an installed development skill covers the activity of `skill@klarch-development` and thaum-development, those project skills reach a session by their descriptions and are read against the installed text after an install; the work may tempt a mapping row; none is proposed.
- `design@agent-skills@adopting-existing-docs`: not read. #component-skill-delivery has the component-creation skill's description trigger at setup; this head says setting up stops at a conformant structure and a move is planned work; a split of an existing project's Components triggered at setup could cross that line; not drafted.
- `design@agent-skills@staged-check-before-each-commit`: not read. the development skill's "commit" step meets the primer's `check --staged` before each commit; a restatement or a different pre-commit sequence in the skill could diverge; not drafted.
- `rejected@agent-skills@delegated-decision-state`: not read. #upstream-tool-loop lets a session build an improvement of a shipped tool locally on its own judgement; the entry refuses handing judgement over "within a stated boundary"; the loop is a stated permission of the installed text rather than an owner's grant, so no return of the design is established.
- `rejected@agent-skills@directive-restated-everywhere`: not read. porting `skill@klarch-development` "made generic" could carry its restatements of primer and root directives whole into the installed skill, the design this entry rejects; the port's text is not drafted.
- `rejected@agent-skills@owner-intent-entry-test`: not read. #code-quality-goal-wording and #goal-standard-measure move what a goal states toward a mechanism; the entry's doubt is that "intents moved into goals could swell the goals homes with local preferences"; no proposal to bring back the test is made / thread 12 rewrites a goal toward a "vaguer intent", and thread 11 changes what a goal's observable is; the entry's remaining doubt is that "intents moved into goals could swell the goals homes with local preferences". The threads narrow a goal rather than add one, so no bearing is established.
- `rejected@agent-skills@root-routing-table`: not read. with an installed development skill beside `skill@klarch-development` and thaum-development, a session may propose a root row mapping each project skill to the installed one it adds to, the design rejected here; not proposed.
- `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`: not read. cites seed `design@agent-skills@shared-text-fragment`, which #component-skill-delivery applies to a new fragment; the work does not sweep the pointers this issue lists. / cites the seed; the new development skill and agents may point to standards they apply whole (the review axes, the goal standard, "when to split"), which the issue's sweep would judge as fragment candidates, but the threads name no such pointer.
- `issue@agent-skills@the-material-finding-duty-has-no-head`: not read. #search-timing runs the record search after a few rounds, so an entry it finds after the owner ruled a thread becomes a material finding under `skill@knowledge-architect-design@decision-authority`, the duty this issue says has no head; no change to the duty is proposed
- `issue@core@a-component-states-at-least-one-goal`: not read. thread 14's one-Component-per-crate default and the component-creation procedure multiply Components each owing a goal, which nothing checks; the issue's trigger is thaum's state, not this work. / Thread 14's one-Component-per-crate default multiplies Components, and thread 15's procedure writes the goals home through goal-setting; the issue's check and its note that goal-setting would join reaching a passing check could interact; no relation established.
- `tripwire@agent-skills@departure-overridden-as-a-reading`: not read. #spec-first-axis judges code against a specification's text; a reviewer applying the text over a departure the owner gave would meet this firing clause; the axis is not yet drafted to read departures.
- `design@agent-skills@design-record-axis`: not read. #code-claims-scope widens the code-claims reviewer to heads' statements about code; this head says "The truth of a head about the code is another axis too"; the widened reviewer is a review, not an audit axis, so no conflict is established.
- `design@agent-skills@design-record-axis-dispatch`: not read. #adversarial-scoping dispatches reviewers per subsystem; this head's argument that "agents dispatched together do not read a rule alike" bears on scoped reviewers judging one standard, but the review dispatches no calibration; not established.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: not read. cites seed `design@agent-skills@plan-read-against-the-record`; its unbuilt axis "alignment of implementation with design and goals" overlaps #code-claims-scope (every statement a diff writes about code) and #spec-first-axis at review scale; the work adds review axes, not audit axes. / cites seed `design@agent-skills@plan-read-against-the-record`; the entry keeps an 'alignment of implementation with design and goals' audit axis, while thread 8 widens code-claims at review scale and thread 15 triggers at an audit; no audit axis is added by the work / names seed `design@agent-skills@plan-read-against-the-record`; #component-skill-delivery triggers the component-creation skill at an audit, and its "alignment of implementation with design" axis neighbours the code-claims widening (#code-claims-scope); neither thread adds an audit axis. / cites seed `design@agent-skills@plan-read-against-the-record`; its axis "alignment of implementation with design and goals" overlaps #code-claims-scope, which reads every statement a diff writes about code, heads included, at review scale, and #component-skill-delivery triggers a skill at an audit; neither adds an audit axis, so no bearing is established / cites seed `design@agent-skills@plan-read-against-the-record`; the issue's unbuilt "alignment of implementation with design and goals" audit axis and its lead that an audit and a review sharing a subject get compatible instructions may meet #code-shape-axis, and #component-skill-delivery triggers at an audit; no audit axis is proposed. / it cites the seed `design@agent-skills@plan-read-against-the-record`; thread 15's component-creation skill also triggers "at an audit", and the issue lists axes ("alignment of implementation with design and goals") that a Component-split audit could touch. No thread adds an audit axis.
- `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes`: not read. the heads this work writes will lean on thaum's findings (thaum-development, its hand-mutation tool); the issue asks whether such a figure goes in the head or in a docs/grounding directory of the Component; not established that the work records one.
- `design@agent-skills@audit-is-an-activity`: not read. thread 15 has the component-creation skill trigger at an audit, and threads 2/5/8 add review axes; the head says an audit reads the whole project on one axis and a review stays the reading of a diff; no bearing established unless a Component split becomes an audit occasion or axis
- `design@agent-skills@audit-triggers`: not read. thread 15's component-creation skill 'triggering at setup and at an audit' adds an occasion near an audit; the head says an audit runs only on an occasion whose own work needs the record read under the rules; whether the work adds an audit occasion is not established
- `design@agent-skills@audit-agent-per-axis`: not read. thread 2 makes the code-shape axis 'likely an installed agent' and threads 4/5 add reviewers; the head's argument (standard in the agent's definition, not behind a brief) is stated for audit axes only, so whether it binds review axes is not established
- `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text`: not read. thread 15 states the component-creation skill triggers at setup and at an audit, 'not in planning'; the entry shows an obligation stated in setup and audit texts is unread by sessions landing the work under planning; whether a Component split lands through planned slices is not established
- `rejected@core@commit-msg-hook`: not read. thread 1's commit step could take a hook to deliver the claim check at each commit, as the claim-check issue lists; the entry rejects a commit-msg hook; no such proposal is drafted
- `rejected@core@commit-subcommand`: not read. thread 1's commit step could be wrapped in a command; the entry rejects a `commit` subcommand; no such proposal is drafted
- `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`: not read. thread 6 ships a command of a published crate, the shape this entry asks for history tasks, and the seed issue also covers branch work; no drafted thread ships or names the history commands
- `design@agent-skills@structure-the-flow`: not read. #search-timing's "after a few rounds" may set a count on rounds before the search, and the head says "No bound is set ... on the number of ... rounds"; no bearing established, since the thread does not yet say whether the timing is a fixed count or a judgement.
- `design@agent-skills@title-states-the-rule`: not read. #performance-opt-in has a project declare a performance-critical path as a design head naming its benchmark command; such a title naming the path or the command could state the instance, against "the head's title states the property that admits a member"; the head's shape is not drafted, so no bearing is established.
- `rejected@agent-skills@design-load-unconditioned`: not read. the dev skill's "ground before editing" could send every change to the design skill before editing, which is this entry's design; whether the dev skill routes that way is not drafted.
- `rejected@agent-skills@grounding-split-from-discussion`: not read. #search-timing separates the record search from the start of the discussion, and the dev skill holds its own "ground before editing"; this entry rejects a grounding apart from the discussion; neither thread yet moves the grounding into another skill.
- `rejected@agent-skills@trigger-judges-settled-design`: not read. the dev skill's description or steps could send a change to the design skill only "when its design is not settled", the trigger this entry rejects; the dev skill's trigger to the design skill is not drafted.
- `issue@agent-skills@the-open-space-test-may-admit-every-problem`: not read. #search-timing moves the search after a few rounds; the bounded classification this issue watches runs on the grounding, and a later search could change how often work is found bounded; no thread changes the open-space test.
- `issue@gates@the-gate-list-head-bundles-the-library-s-independence`: not read. #mutation-command could be hosted in the gates crate as a published tool library; this issue says the gates library "depends on nothing of the checker" with no recorded reason, which would bind such a placement; the host is not settled
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: not read. names seed `design@agent-skills@record-searched-before-the-work`; the dev skill is the host for "a search before such work" that this tripwire's response proposes, but the work does not meet its firing clause (a second recorded instance). / cites the seed; its response "propose a search before such work, with its host" names what thread 1's development skill could host; the firing clause (a second instance) is not met by the work
- `tripwire@agent-skills@owner-proposal-built-ungrounded`: not read. a dev skill that implements an owner-requested change without the design skill's grounding would produce the commits this firing clause counts; the dev skill's routing of owner requests is not drafted.
- `design@knowledge-architect@no-component-directory-named-after-the-project`: not read. thread 14 has the component-creation skill create Components, one per crate; a crate directory named after the project collides with the root anchor ("names 2 anchors"), which the creation procedure may need to state; whether it names directories is not drafted.
- `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`: not read. names seed `design@agent-skills@shared-text-fragment`; #component-skill-delivery adds a shared fragment through the fragment pass this issue lists, which it uses as built; no thread changes the passes or their crate. / cites the seed; thread 6 chooses which published crate hosts the mutation command, and the issue contemplates splitting the agent-skills crate into a bundling utility and the skills, but neither names the other. / cites seed `design@agent-skills@shared-text-fragment`, whose fragment pass #component-skill-delivery and #goal-standard-measure use; the Components thread's one-Component-per-crate default and split signals could bear on this issue's proposed split of agent-skills; no change to the passes is proposed. / cites the seed `design@agent-skills@shared-text-fragment`; thread 15 adds a shared fragment rendered by the fragment pass this issue lists, but the work does not change the passes or their tie to this project.
- `rejected@agent-skills@separate-expectation-sets-file`: not read. if the new development or component-creation skill states an expectation set, where it lives could be reopened; no thread proposes a separate file, so no bearing established
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: not read. applying the proposed code-shape or test economy criteria to this repository's code by subagents to shape them would be such a census; no thread proposes one, so no bearing established
- `tripwire@agent-skills@agent-text-written-without-the-standard`: not read. the work writes much agent-facing text; it fires only on an unneeded addition in a session that did not load the agent-configuration skill, which the design work need not be; no firing established
- `tripwire@agent-skills@expectation-set-closes-a-contradiction`: not read. names seed `design@agent-skills@expectation-set-bounds-scope`; a set for the new skills could later be cited to close a contradiction between the development and review skills, but no thread closes one so
- `design@core@transcript-command-in-core`: not read. #mutation-command adds a command to a published crate; this head argues a command the installed text asks of every project lives in the core's command enum, not the maintenance tool or a second crate per `design@core@single-crate`, which the choice of crate for the mutation command meets; the subject differs, so no bearing established
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: not read. #mutation-command adds a published command (`run`/`restore`) whose printed output class this undecided policy would govern; the new command could declare its output no contract as `transcript` does, so no conflict established / #mutation-command adds a published command with `run`/`restore` output; the issue says nothing states which part of a command's output is a contract, and whether the new command's output is consumed is not established / #mutation-command adds a command to a published crate whose printed output would need a class under the versioning policy; the issue says "Nothing states which part of a command's printed output is a contract". The command's output is not drafted yet.
- `design@core@nothing-of-a-project-is-compiled-in`: not read. #component-skill-delivery's creation procedure (manifest entry, goals home, documents) and #mutation-command add to what a published crate holds; the head's list of five compiled-in things includes the starting text `new-component` writes; no conflict established unless the component-creation skill wants that starting text changed, or the mutation tool compiles in a project's paths
- `design@core@exit-code-ladder`: not read. #mutation-command adds a `run`/`restore` command to a published crate; the head "binds the core's commands in every binary that offers them" (0 in order, 1 negative answer, 2 could not run); if the command lands in core, a failed baseline or a failed build probe must be mapped to 1 or 2 by this ladder; the crate is not chosen yet, so no bearing is established
- `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`: not read. its close says the suffix shapes are "settled under `skill@klarch-development`"; porting that skill into the installed set may shrink or remove it, which would leave that pointer to re-aim; no conflict with the fix itself / its closing clause says the suffix shapes are "settled under `skill@klarch-development`". Thread 1 ports that skill into an installed development skill, which could thin or remove the local skill and leave the pointer stale. Whether the local skill survives is not decided.
- `issue@gates@a-run-that-could-not-start-exits-one`: not read. if #mutation-command ships in the gates crate, or a gate runs #diff-mutation-sweep, the new command's "could not run" (failed build probe, no baseline) faces the same open exit-code contract "No recorded decision says which exit-code contract binds the gates"; the crate is not chosen
- `design@agent-skills@rejected-citation-direction`: not read. a head of absence for "no performance review axis", or the code-shape axis head, would cite the rejected entries its argument covers; routine recording, no conflict established
- `design@agent-skills@subject-split-standard`: not read. cites seed `design@agent-skills@shared-text-fragment`; #component-skill-delivery adds a shared fragment "when to split" for Components, beside this head's fragment `path@agent-skills@fragments/subject-split.md` on splitting a design subject; the two split notions and fragment names could collide in the hosts; no conflict established
- `goal@core@records-reach-their-reader`: not read. #search-timing moves when the design skill's record search delivers the checker's listings to the session; the goal is met by the listings themselves, which the thread does not change
- `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`: not read. the harvest of #mutation-command (a command line, an interface head) would meet test 1 for its rivals, the reading this issue says is applied two ways; the work does not change the tests
- `design@agent-skills@no-untested-snippet-is-authority`: not read. thread 1's "implement" step implements from a plan document; the head's "no untested code snippet in it is presented as authority" could need restating or respecting in the skill, but the threads say nothing of how the skill reads a spec's snippets.
- `design@agent-skills@criteria-test-silent-premises`: not read. thread 9 (#performance-opt-in) has a change measure a benchmark before and after and the review check the figures; the head admits a criterion only where no test, gate or review reports its failure, and a measured performance bound could be proposed as a criterion, but the thread routes it to a review check, so no conflict is established.
- `design@agent-skills@plan-reviewed-as-a-commit`: not read. the head states the review skill gives each reviewer that runs the tests a worktree detached at the commit under review; threads 4 and 10 send adversarial reviewers, who run tests and mutations, at development checkpoints, and thread 6's tool mutates uncommitted work, which could conflict if a checkpoint reviews uncommitted code; the threads do not say what a checkpoint review reads.
- `tripwire@agent-skills@ruling-lost-in-change`: not read. if this 17-thread discussion's rulings are carried on the in-change path, its commit message is an occasion the tripwire watches, and its response asks whether a discussion with more than one approved thread takes the full path; the path is not yet chosen.
- `design@core@harness-kinds-cited-without-anchor`: not read. the new installed skills and agents (development, component-creation, code-shape and spec-first reviewers) are cited as `skill@<name>@<slug>`; the entry fixes that form, and the work only uses it, so no bearing beyond following it is established
- `goal@core@relocation-is-one-manifest-edit`: not read. #components and the component-creation procedure (manifest entry) split a Component and move code into a new directory; the goal covers moving a Component, not splitting one, so no conflict is established
- `issue@core@a-plain-hash-item-name-is-read-by-nothing`: not read. the discussion names its threads as plain #code-shape-axis and so on, the form this issue says no check reads, and its plan's harvest will carry them; the work changes neither the form nor the check, so no bearing is established
- `design@knowledge-architect@version-lockstep`: not read. #mutation-command ships a command "of a published knowledge-architect crate"; if it lands in a new crate, "every crate carries one version" and the gates' lockstep apply to it, and which crate is not decided
- `design@knowledge-architect@capability-cost-before-migration`: not read. #review-cadence moves the code-claims review to once per branch and #code-quality-goal-wording drops the departure clause; this head ranks the cost of a lost consumer use first, and no lost use is established
- `design@knowledge-architect@publish-after-merge`: not read. the issue the work answers also names releasing, and a generic release skill would be ported from `skill@klarch-release`, which this head's procedure is; no thread drafts releasing, so no bearing is established
- `design@knowledge-architect@publish-credential`: not read. #mutation-command's home crate is undecided; if a new crate, "a new crate's first version is outside what the release workflow can publish", which the head states and the work does not yet meet
- `issue@gates@the-library-has-no-merge-verdict`: not read. the development skill's "gate" step and the work's port of thaum's development skill would name the gates invocation, and thaum's `--merge-ready` is the merge mode this issue says the library lacks; the threads do not define the invocation, so no dependency is established
- `design@core@every-path-names-its-anchor`: not read. thread 14's module-directory and nested Components add anchors inside a crate's Component; "The deepest anchor wins" then refuses every existing `path` reference into the new Component's directory, so the creation procedure of thread 15 may owe a re-pointing step; whether the drafted procedure omits it is not established
- `issue@core@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`: not read. thread 14's nested and module-directory Components create anchors whose own directory is named from the parent anchor, the form this issue says breaks on a move; the drafted shapes move no anchor, so no bearing is established
- `design@agent-skills@standing-state-review-before-every-merge`: not read. cites the seed `design@agent-skills@record-searched-before-the-work`; #search-timing moves when the design skill's search runs, and the head contrasts that search (only for designed work) with this reviewer (every merge); #review-cadence adds axes to the review before the merge but no conflict with this reviewer's reading is established.
- `design@agent-skills@unread-entries-reach-the-conformance-review`: not read. cites both seeds; #search-timing changes when the search that produces the may-bear list runs, and the head fixes where that list is kept (plan's "What is already decided" or the in-change commit message); no change to the list's reader is proposed.
- `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: not read. thread 1 and #review-cadence define review at development checkpoints and how code is repaired; the head says "Any other repair is reviewed by no axis again", which may conflict if the development skill re-sends adversarial or code-shape review on a repair; no such rule is drafted yet.
- `design@agent-skills@review-judges-additions`: not read. the threads add rows to `skill@knowledge-architect-review@review-axes` (#code-shape-axis, #spec-first-axis, #code-claims-scope); this head holds a row there and states "No other axis judges whether a statement is needed"; the code-claims widening to doc comments, READMEs and heads judges truth against code, not need, so no overlap is established.
- `design@agent-skills@transcript-reviewer-agent`: not read. #code-quality-goal-wording drops a clause of a goal; the head states the reading by direction with the goal exception ("a goal's wording that rules out what the owner has not ruled out knowingly is a finding"), and #retrospective-code-rules-question adds to the retrospective, which applies the same reading; no change to the reviewer is proposed.
- `design@core@declared-command`: not read. #mutation-command ships a command in a published crate; installed text names the checker's command by the `{{command}}` placeholder, and if the tool sits in a crate other than the checker's binary the declared command does not name it; the crate is not chosen.
- `design@core@owned-namespace-check`: not read. #upstream-tool-loop lets a session improve a shipped tool locally; the head checks installed files byte for byte and says "A change a project needs to an installed skill belongs in a skill of its own"; conflict only if "tool" covers installed skills, agents or saved workflows.
- `design@agent-skills@workflow-axis-corpus`: not read. threads 1 and 15 add two installed skills and threads 2/5 likely new agents to the corpus; the head rests on "one agent being able to read it whole" and reopens the corpus's sharing when an auditor cannot, but no measurement shows the additions reach that point.
- `design@agent-skills@workflow-axis-lenses`: not read. threads 2, 5 and 8 write new review standards (code shape, spec-first, widened code claims); L8 "reads each pair of a review standard and an audit standard that judge one subject" for divergent verdicts, and no audit standard is shown to judge code, so no pair is established.
- `design@agent-skills@workflow-auditor-agent`: not read. threads 2 and 5 (and the per-subsystem adversarial reviewers of thread 4) may each ship as an installed agent; the head records the owner's weighing of "the count of published agents", but that weighing was made for the audit's lenses, not review axes.
- `design@agent-skills@root-cause-repair`: not read. thread 15 delivers "when to split" in a fragment hosted by two skills plus a new component-creation skill, and thread 1 leaves this repository's `skill@klarch-development` beside an installed development skill; the head names "two homes for one moment" as a cause of findings, but a build-copied fragment is no restatement per the primer, so no conflict is established.
- `rejected@agent-skills@workflow-auditor-per-lens`: not read. threads 2/4/5 may publish one agent per review axis or per adversarial scope; this entry records the owner not wanting "publishing too many agents and polluting the directory", stated for audit lenses only, so no bringing back is established.
- `rejected@agent-skills@workflow-corpus-shared-out`: not read. it is the candidate if "the corpus grows with every skill ... and may outgrow one agent's reading"; the work adds at least two skills, with no measurement that it outgrows.
- `design@agent-skills@unrecognised-entries-reach-upstream`: not read. thread 13 adds a standing retrospective question; this head's argument weighs a standing question that "every retrospective of every project would load and answer though most of their sessions never ran the command". The same cost may apply to a code-rules question in sessions that write no code; whether it applies is not established.
- `rejected@agent-skills@retrospective-asks-for-unrecognised-lines`: not read. thread 13 proposes a new standing retrospective question. The question this entry rejects is a different one, but it lost on the cost of a standing question "loaded and answered by every retrospective of every project". Whether that cost argument extends to the code-rules question is not established.
- `design@agent-skills@ruled-items-labelled`: not read. the new development and component-creation skills may ask the owner for rulings under a label; the head says "Every distinct prefix is listed here, and an edit that adds one checks it against the list". No thread drafts a new prefix, so no bearing is established.
- `design@agent-skills@heads-stand-on-argument`: not read. the heads this discussion records (e.g. #performance-opt-in, the Components shape, #search-timing) come out of owner-proposed shapes, and "what the owner brings lands by its kind". No thread changes that rule, so no bearing is established.
- `design@agent-skills@fork-choice-in-weak-form`: not read. threads such as #search-timing or the Components default may close as forks; "A fork's choice names the rivals and the costs that were weighed". No thread changes the rule.
- `design@agent-skills@unargued-approval-is-argued`: not read. seventeen drafted shapes may be approved in a batch; the head says such an approval is "searched for the arguments for and against it" before a head is written. This is a procedure the discussion follows, not one it changes.
- `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`: not read. thread 1's ground before editing step may restate what work waits on a head/code divergence; the issue asks "Does work on the point in divergence wait, or go on building on the head?". No thread states that step's wording yet.
- `design@core@index-takes-no-dry-run`: not read. #mutation-command adds a command of a published crate that writes over uncommitted work and restores it; the head says "The dry run a write command normally owes exists because a write can lose something". The head rules only on `index`, so no bearing is established.
- `design@core@one-question-one-command`: not read. #mutation-command adds a command to a published crate; the head says "A second command answering the same question in its own format is what drifts". No existing command answers the mutation question, so no bearing is established.
- `design@agent-skills@one-record-searcher`: not read. thread #17 moves the moment of the search and thread 1 may have the development skill dispatch it for a fix; the head says the searcher's description "carries the dispatch", so a new dispatcher or moment may need that description changed; no change of the searcher itself is drafted
- `design@agent-skills@search-apart-from-the-standing-state-review`: not read. thread #test-economy folds a second standard into the code-shape axis, and #code-claims-scope widens one reviewer's scope; the head's argument that "both standards in one text ... lets a session apply the wrong one" may weigh against that folding, though the head is about the searcher and the standing-state reviewer only
- `design@agent-skills@search-reads-the-whole-record`: not read. if thread 1 has the development skill dispatch a search before a fix (per the trigger of `a-fix-is-grounded-by-no-record-search`), its cost per fix may argue for a lighter pass, which this head defers; not drafted in the work
- `design@agent-skills@may-bear-read-on-touch`: not read. thread #17 (search after a few rounds) answers the case of a scope widened after the search, which this discussion met; the head's reopening condition and the open issue on it name that case, so #17 may change what the grounding reads and when, though the head itself rules only what is read
- `design@agent-skills@seeds-name-decisions`: not read. thread 1 may make the development skill dispatch a search before a fix, and the deferred issue's trigger says that design decides "with which seeds"; this head rules seeds only for the design grounding and the design audit, so a new dispatcher may need a seed rule of its own
- `rejected@agent-skills@search-work-is-the-question`: not read. thread #17 changes what the search's work holds at dispatch (shapes after rounds); this entry's kept doubt, that "a work statement holding the shapes may leave an entry that bears on the problem outside them graded `unrelated`", bears on that choice
- `issue@agent-skills@a-spec-worked-without-its-design-audit-missed-a-critical-gap`: not read. thread 1's "review at checkpoints" and #review-cadence set reviews during a spec's development; this issue reopens whether a spec's work gets a design audit, which may meet that cadence, though no thread touches the planning skill's audit rule
- `tripwire@agent-skills@record-search-misses`: not read. cites the seed; its response asks whether a miss came "from a thread ... on a subject the search's work did not cover", which thread #17 addresses; the work does not meet its firing clause
- `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`: not read. the source issue of the work also covers branch work; if the discussion ports this repository's reword, fold and merge-predicate procedures into an installed skill, this issue's closing (a command per procedure, or a published one) meets it; the drafted threads name no branch-work step
- `design@core@init-command`: not read. Thread 15's creation procedure (manifest entry, documents) may name `init --component` beside `new-component`; the head says init writes every document a listed Component owes through shared code; no conflict established.
- `design@core@component-manifest-edit`: not read. Thread 15's "manifest entry" step might tell a session to edit `[project] components` by hand, where this head has `new-component` append it with a format-preserving editor; whether the skill names the command is not yet drafted.
- `design@core@rust-scaffold`: not read. Thread 14's one-Component-per-crate default and shared test Component meet this head's scaffold, which lists the `xtask` crate as a Component; consistent as drafted, no conflict established.
- `design@core@git-is-asked-through-one-module`: not read. Thread 6 ships the mutation command (baseline, byte-exact restore over uncommitted work) in a published crate; if that crate is core and the command asks git anything, this head puts every git question in `path@core@src/git.rs`; crate and git use not yet decided.
- `design@core@anchors-are-components-and-locations`: not read. Thread 14's shared test Component and nested layer Components choose Component over location; this head says a location owes only its declared registers, a cheaper rival for a directory without its own purpose; no conflict established.
- `design@core@kind-names-refused-as-anchors`: not read. Thread 14's one Component per crate (or per module directory) names each by its basename; a crate or module named like a kind (e.g. `design`, `path`, `spec`) is refused by this head; the component-creation skill's naming step may need to say so.
- `design@core@a-component-carries-what-a-project-attaches`: not read. Thread 15's creation procedure: an `opt-in` register is not guessed by `new-component`, the user adds the Component to its `components` list per this head; whether the skill names that step is not drafted.
- `design@core@the-core-cli-is-a-library-module`: not read. #mutation-command ships a command of "a published knowledge-architect crate"; if that crate is the core, the new command joins the list this head states of the core's commands, held in the library module and offered by every extension binary; which crate hosts it is not settled
- `design@core@arguments-parse-through-clap`: not read. #mutation-command adds `run`/`restore` subcommands to a published crate; were they a core command, the head requires them declared through clap and tested by debug_assert; the hosting crate is not settled
- `design@core@a-foreign-build-is-refused`: not read. #adversarial-scoping and the mutation procedure run builds in other checkouts or worktrees that may share a target directory, which this head refuses; #mutation-command works in the checkout itself, so no conflict is established
- `design@knowledge-architect@git-flow`: not read. cites seed `design@knowledge-architect@scratch-worktree-outside-git-rules`; the development skill's "commit; review at checkpoints" and #review-cadence meet its review-before-merge rule only as this repository's own flow, which the generic skill may not assume (no PR/CI)
- `design@knowledge-architect@a-build-is-tied-to-its-checkout`: not read. #adversarial-scoping reviewers and mutation runs in worktrees share target directories; this head notes the separate target directory "stays an instruction of the review skill"; a generic development skill porting that instruction meets it, no conflict established
- `issue@knowledge-architect@ci-does-not-run-on-a-draft-and-no-record-says-why`: not read. #review-cadence places reviews at development checkpoints and the skill's gate step; this issue is about tests first running in CI after the whole review; the development skill assumes no CI, so no bearing established
- `issue@core@references-are-read-in-markdown-and-rust-only`: not read. the development skill's comment species (a comment naming the issue at the site) yields references nothing reads in a target project's non-Rust glue, which this issue says forces skills "to tell the agent to grep instead"
- `design@core@plan-document-sections-by-kind`: not read. thread 1's step "claims each with a test that discriminates", ported from `skill@klarch-development`, meets the slice spec sections this head fixes ("Builds, Claims, Audit subjects, Fails alone on ..."), which it says are the planning skill's; no bearing established unless the development skill restates or reshapes what a slice spec's Claims hold, and thread 15 keeps the component skill out of planning
- `design@gates@gates-crate`: not read. thread 6 ships the mutation command in "a published knowledge-architect crate"; this head argues a gates command of the checker's binary lost because "it would make the document checker run cargo, the linter and the tests", and that a project keeps its own binary for other repeated tasks; whether that argument covers a mutation command (which builds and runs tests) placed in klarch is not established, since the crate is not chosen
- `design@gates@a-project-holds-its-gate-list`: not read. threads 7 (cargo-mutants --in-diff) and 8 (lints in the setup's linter ask) could add to `rust_project`'s recommended list; the drafts call 7 an optional step, not a gate, so no bearing established
- `design@gates@one-spawn-helper`: not read. if thread 6's mutation command lands in the gates crate or a project's command, it spawns builds and test runs; the head says "a project's own commands may [use it]: a new command calls it and never edits it"; the crate is not chosen, so no bearing established
- `design@gates@gates-scrub-rustc-bootstrap`: not read. thread 6's mutation tool runs a baseline and a build probe; this head's false-green argument ("RUSTC_BOOTSTRAP=1 makes a whole build nightly-equivalent") would apply to those runs if they are started outside the spawn helper; holds only if the tool spawns through the gates library, not established
- `issue@gates@the-one-spawn-helper-has-no-recorded-argument`: not read. its re-entry point is "the next change that ... adds a second way to start a child in the gates library"; thread 6's mutation command would meet it if placed in the gates crate with its own spawning; placement not decided / thread 6 puts a mutation command that runs builds in "a published knowledge-architect crate", unnamed; if that is the gates library and it starts a child outside the one spawn helper, it meets the re-entry point "adds a second way to start a child in the gates library"; the crate is not decided.
- `design@agent-skills@goal-lifecycle`: not read. thread 12 renames the goal toward `goal@knowledge-architect@the-workflow-leads-to-better-code`; the head says a goal "leaves only when the owner abandons it", and a rename that drops a clause is a rewording, not an abandonment, so no bearing unless the rename is carried out as removal plus a new goal.
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: not read. thread 12 rewords the code-quality goal "to a vaguer intent"; the head relies on the goal reviewer and `primer@goals-bind` for a goal that reads two ways, and adds no instruction; a vaguer goal could meet that case, though the work proposes no such instruction.
- `design@agent-skills@goal-set-review-in-goal-setting`: not read. thread 11 changes rule 6 of the goal standard; the head runs the whole-set review "at a pin move across a version whose Migration entry names it", and whether the changed rule owes such a Migration entry is not established.
- `design@xtask@gates-list-primary-home`: not read. thread 8 adds doc lints to the setup's linter ask, and thread 7 an optional cargo-mutants step; the head makes `path@xtask@src/gates.rs` the one home of this repository's gates; no thread adds a gate here, so no bearing is established.
- `design@agent-skills@tripwire-horizon`: not read. any tripwire the discussion's premortem writes needs a `**Watches until:**` line counted by a command, per "Every tripwire names the occasions after which, unfired, it leaves"; no thread changes the lifecycle itself, so no relation beyond the recording procedure is established.
- `tripwire@agent-skills@silent-read-too-widely`: not read. thread 1 has each claim carried by a test; the firing clause counts a check "listed as a claim of the step that builds it rather than an acceptance criterion"; it fires only together with an issue on a silent failure, which the work does not open, so no firing is established.

**Entries the plan's review found missing**, with their outcome:

| entry | outcome |
| --- | --- |
| `design@knowledge-architect@changelog-reviewed-at-the-release` | rested on: the widened code-claims reviewer of #code-claims-scope leaves the entries of CHANGELOG.md to the release's review |
| `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` | constrains #spec-first-axis: the reviewer judges the code only against what binds its writer, the specification as the project follows it |
| `tripwire@agent-skills@departure-overridden-as-a-reading` | constrains #spec-first-axis: the reviewer reads the departures from the specification the project states, and applies no text of the specification over them |
| `design@core@exit-code-ladder` | constrains slice 1: the mutation crate's outcome-to-exit-code mapping is read against it at that slice's audit |

## Criteria

The criteria of the checkpoint the agent proposed in the reply to R6, which the owner's "Agreed on
the table." of R7 closed with the rest of the checkpoint.

### A review judges the shape of code `##code-shape-is-reviewed`

Binding, from `goal@knowledge-architect@the-workflow-leads-to-better-code`, whose first "met while"
item asks that the installed text "has a review of code weigh each branch's changes to Rust code
against it, at the latest at the review before the branch merges". Met by #code-shape-axis and
#review-cadence.

### The installed text works in a project that values neither performance nor an external specification `##works-without-performance-or-spec`

Binding, from `goal@agent-skills@installed-text-works-in-every-target-project`. Met by
#performance-opt-in and #spec-first-axis, both conditional: a project that declares no path and
implements no specification never triggers them.

### No instruction the session's judgement already handles, and each new behaviour instruction names its real-use evidence `##only-needed-instructions-with-evidence`

Binding, from `goal@agent-skills@only-needed-instructions` and
`design@agent-skills@behaviour-additions-need-real-use`. Met by the evidence named per instruction
in `argument@development-and-components@arg-64`, which slice 2 carries into each instruction's `%%`
lines.

### A mechanical step is a command `##mechanical-step-is-a-command`

Binding, from `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`. Met by #mutation-command
and #mutation-command-host: the manual worktree procedure becomes `cargo x mutate`.

### No pull request or continuous integration is assumed outside an offered shape `##no-pr-or-ci-assumed`

Binding, from `goal@knowledge-architect@agents-get-a-complete-workflow`: "The installed text
requires neither, and no installed instruction outside an offered shape assumes either." A
constraint on the development skill of slice 2, per
`issue@agent-skills@installed-text-assumes-pull-requests-and-ci`.

### The review cost per branch stays low `##review-cost`

Weighed, from the owner's R1: "Without creating to much overhead or review costs." Met by
#review-cadence, at most two more dispatches per branch with no build; by #adversarial-scoping; and
by #fix-grounding, no search per fix.

## Threads

Each thread with its proposer and round, its states, the arguments that moved it, the owner's words
that closed it, verbatim, the slice whose spec carries its shape, and the durable home that will
harvest it. "Approved" with no further words means the owner's word in R7, "Agreed on the table.",
on the checkpoint of the reply to R6, which listed the thread in its bucket.

### The code-shape axis is an installed read-only reviewer of the default shape, reading one responsibility as one reason to change, with fragmentation a finding too `##code-shape-axis`

- **Proposed:** the agent, reply to R1, answering the owner's "overall quality" axis of R1.
- **States:** new (reply to R1); in discussion (reply to R2); approved R3; an installed agent, reply
  to R5 (`argument@development-and-components@arg-68`).
- **Arguments:** `argument@development-and-components@arg-1`, `argument@development-and-components@arg-3`,
  `argument@development-and-components@arg-11`, `argument@development-and-components@arg-14`,
  `argument@development-and-components@arg-19`, `argument@development-and-components@arg-33`,
  `argument@development-and-components@arg-39`, `argument@development-and-components@arg-57`,
  `argument@development-and-components@arg-64`.
- **Closed by:** R2: "code-shape-axis: overall I agree." R3: "code-shape-axis: I agree with the one
  reason to change rule, this is much more testable and lenient, and this is the actual benefit of
  the principle I believe."
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slug its thread's, slice 2.
- **Relations:** absorbs the owner's "overall quality" axis and #test-economy; constrained by the
  reworded goal of #code-quality-goal-wording.

### Each test the diff adds names its claim, and tests of logic carry mutation evidence; the code-shape reviewer judges it `##test-economy`

- **Proposed:** the agent, reply to R1, answering the owner's R1 remark that "agents tend to add
  lots of tests on a whim".
- **States:** new (reply to R1); in discussion (reply to R2); approved R3.
- **Arguments:** `argument@development-and-components@arg-4`, `argument@development-and-components@arg-15`,
  `argument@development-and-components@arg-20`, `argument@development-and-components@arg-82`.
- **Closed by:** R2: "test-economy: this is a good idea." R3: "test-economy agreed on refinements".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** with the head of
  #code-shape-axis, or a head of its own where decision-recording's test of one decision per head
  says so, slice 2.

### Adversarial reviewers are scoped to one subsystem, claim family or boundary each, pick their tests, and run the wider suite before reporting a survivor `##adversarial-scoping`

- **Proposed:** the agent, reply to R1, from thaum.
- **States:** new (reply to R1); approved R2. The owner's R6 confirmed it as the fix of reviewers'
  running time.
- **Arguments:** `argument@development-and-components@arg-2`, `argument@development-and-components@arg-5`,
  `argument@development-and-components@arg-65`, `argument@development-and-components@arg-70`,
  `argument@development-and-components@arg-78`, `argument@development-and-components@arg-83`.
- **Closed by:** R2: "adversarial-scoping agreed". R6: "the fix I mentionned was sending multiple
  adversarial on a smaller, scoped set of the changes, plus the instruction to run partial tests. In
  practice, until now, this has proven to be enough of a fix, by sufficiently reducing the time
  adversarial reviewers take to report, without adding constraints or a cap that might prevent them
  from finding more."
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 2. This thread carries `a-reviewer-s-running-time-is-unbounded`, closed when this milestone scheduled it.

### A spec-first axis, an installed agent, works out what an external specification requires before reading the code, where the project implements one and the diff changes how `##spec-first-axis`

- **Proposed:** the agent, reply to R1, generalising thaum's rules-first axis.
- **States:** new (reply to R1); approved R2; an installed agent, reply to R5.
- **Arguments:** `argument@development-and-components@arg-2`, `argument@development-and-components@arg-6`,
  `argument@development-and-components@arg-57`, `argument@development-and-components@arg-68`,
  `argument@development-and-components@arg-84`.
- **Closed by:** R2: "spec-first-axis good idea too".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 2.

### The generic `run` and `restore` of thaum's mutation tool ship as a command; `sweep` stays thaum's `##mutation-command`

- **Proposed:** the agent, reply to R1, from thaum (Q1, then Q3).
- **States:** new (reply to R1); in discussion (reply to R2); approved R3, `run` and `restore` only;
  its host split out to #mutation-command-host, reply to R5.
- **Arguments:** `argument@development-and-components@arg-2`, `argument@development-and-components@arg-7`,
  `argument@development-and-components@arg-15`, `argument@development-and-components@arg-21`,
  `argument@development-and-components@arg-23`, `argument@development-and-components@arg-37`,
  `argument@development-and-components@arg-39`, `argument@development-and-components@arg-57`.
- **Closed by:** R3: "mutation-command: I see, sot the main value  lies in allowing to write our own
  mutation on the go, or use a list of recorded ones, right ? Instead of cargo-mutants which chooses
  them itself automatically ? In that case, I think it is quite worth it to generalize it."
- **Shape:** `spec@development-and-components@mutation-crate`. **Harvest:** heads in the new
  Component's design home, `mutate`, slice 1.

### The code-claims reviewer reads every statement a diff writes about code, and the completeness of a public interface's documentation `##code-claims-scope`

- **Proposed:** the agent, reply to R1, as the owner's documentation axis with no new axis.
- **States:** new (reply to R1); in discussion (reply to R2); approved R3, absorbing
  #api-doc-completeness.
- **Arguments:** `argument@development-and-components@arg-8`, `argument@development-and-components@arg-16`,
  `argument@development-and-components@arg-24`.
- **Closed by:** R3: "api-doc-completeness agreed to fold into widened code-claim agent".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 2. This thread carries
  `the-code-claims-axis-reads-plan-documents-only`, closed when this milestone scheduled it.

### The presence of documentation is a lint, and its content is read by the widened code-claims reviewer `##api-doc-completeness`

- **Proposed:** the agent, reply to R2, from the owner's R2 remark on completeness.
- **States:** new (reply to R2); approved R3.
- **Arguments:** `argument@development-and-components@arg-16`, `argument@development-and-components@arg-24`.
- **Closed by:** R3: "api-doc-completeness agreed to fold into widened code-claim agent".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** with the head of
  #code-claims-scope; the lints in the setup skill, slice 2.

### An optional step runs cargo-mutants over the branch's changed code before the adversarial review, and gives its survivors to the reviewer as leads `##diff-mutation-sweep`

- **Proposed:** the agent, reply to R2, from the comparison of thaum's tool with cargo-mutants.
- **States:** new (reply to R2); approved R3.
- **Arguments:** `argument@development-and-components@arg-21`, `argument@development-and-components@arg-22`,
  `argument@development-and-components@arg-26`.
- **Closed by:** R3: "diff-mutation-sweep is quite worth it too."
- **Shape:** `spec@development-and-components@development-skill`, with the caveat that cargo-mutants
  ignores rust flags set in the cargo configuration file, `path@knowledge-architect@.cargo/config.toml` in this repository. **Harvest:** with the head of #review-cadence or
  of its own, slice 2.

### Performance is a conditional development step on paths a project declares in a design head, measured before and after, with no reviewer `##performance-opt-in`

- **Proposed:** the owner's idea of a performance review, R1, reshaped by the agent in the reply to
  R1.
- **States:** new (reply to R1); approved R2; Q2's default, a design head, carried in the checkpoint
  and approved R7.
- **Arguments:** `argument@development-and-components@arg-2`, `argument@development-and-components@arg-9`,
  `argument@development-and-components@arg-17`, `argument@development-and-components@arg-84`,
  `argument@development-and-components@arg-85`.
- **Closed by:** R2: "performance-opt-in: good idea, I think this is the correct solution, for
  performance non-regression, review is not needed, a benchmark procedure is better." R7: "Agreed on
  the table."
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 2; judged against
  `issue@agent-skills@a-head-s-measurement-says-nothing-of-its-magnitude`.

### The read-only axes run once per branch, at the review before the merge, and the adversarial and spec-first axes at development checkpoints `##review-cadence`

- **Proposed:** the agent, reply to R1.
- **States:** new (reply to R1); approved R2.
- **Arguments:** `argument@development-and-components@arg-10`.
- **Closed by:** R2: "review-cadence agreed".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 2.

### No performance review axis: improving performance is a task of its own, withdrawn `##performance-review-axis`

- **Proposed:** the owner, R1.
- **States:** withdrawn, reply to R2, on the owner's R2.
- **Arguments:** `argument@development-and-components@arg-9`, `argument@development-and-components@arg-17`,
  `argument@development-and-components@arg-25`.
- **Withdrawn by:** R2: "What I had in mind was more like performance improvements, whether any hot
  path can be improved. But this should not happen on a review, now that I think about it, more as a
  task of its own, and would need the same mechanical benchmark procedure to carry."
- **Shape:** "Losing alternatives". **Harvest:** a `todo` issue for improving performance, opened at
  slice 2; a `rejected` entry only if `design@agent-skills@losing-alternatives-filter` admits it.

### The code-quality goal is reworded under goal-setting: a vague intent, measured by its mechanism, slug toward `goal@knowledge-architect@the-workflow-leads-to-better-code` `##code-quality-goal-wording`

- **Proposed:** the agent, reply to R2, from the owner's R2 worry about the goal.
- **States:** new (reply to R2); in discussion (reply to R3); direction approved, wording drafted
  under goal-setting in the first branch (checkpoint of the reply to R6); approved R7.
- **Arguments:** `argument@development-and-components@arg-13`, `argument@development-and-components@arg-18`,
  `argument@development-and-components@arg-27`, `argument@development-and-components@arg-81`.
- **Closed by:** R3: "Agreed on rewording the goal. I would also consider changing the slug, which
  should be more along the lines of "the workflow leads to improved code". The current one looks like
  it applies to the project itself, not to the shipped skills and has to be observed in target
  projects. overall, I would like to keep this goal's wording relatively vague." R7: "Agreed on the
  table." The wording itself is the owner's to rule, at slice 0, under goal-setting.
- **Shape:** slice 0, landed. **Harvest:** the goal in
  `register@knowledge-architect@goal`, slice 0.

### Rule 6 gains a clause: an outcome judged only in use is measured by what the project builds to pursue it and to learn from its use whether it is met `##goal-standard-measure`

- **Proposed:** the agent, reply to R3, from the owner's R3 diagnosis of the goal standard.
- **States:** new (reply to R3); in discussion, reworded for any project (reply to R4); approved in
  its first reading, reply to R5.
- **Arguments:** `argument@development-and-components@arg-27`, `argument@development-and-components@arg-31`,
  `argument@development-and-components@arg-40`, `argument@development-and-components@arg-45`,
  `argument@development-and-components@arg-50`, `argument@development-and-components@arg-81`.
- **Closed by:** R4: "goal-standard-measure: I agree on the intention, and to fix in in this branch."
  R5: "goal-standard-measure: Agreed on this standard. You are right that the evidence needs to be
  read in the tree. I think that for this to work, it means that the projects must implement a way to
  receive feedback form real use, and collect it in the project, much like we do with retrospectives.
  I think that this is the better reading of both, because it has a systematic method of checking it
  (though hard to implement in practice)."
- **Shape:** slice 0, landed. **Harvest:** the rewrite of
  `design@agent-skills@goal-standard-rules`, slice 0.

### The retrospective asks a standing question on whether an installed rule about code or tests proved too hard or too strict `##retrospective-code-rules-question`

- **Proposed:** the agent, reply to R3, from the owner's R3.
- **States:** new (reply to R3); approved R4.
- **Arguments:** `argument@development-and-components@arg-28`, `argument@development-and-components@arg-32`,
  `argument@development-and-components@arg-69`.
- **Closed by:** R4: "retrospective-code-rules-question: agreed on that question, it looks wide
  enough to detect problems, and narrow enough to be understood and answerable."
- **Widened in slice 0:** the question gains a second clause, on rules or guidelines about splitting
  the project into Components, since slice 0 reworded
  `goal@knowledge-architect@projects-are-split-into-components` to ask the retrospective it; and a
  lint the setup ships counts as an installed rule about code (Q8). Its wording is in
  `spec@development-and-components@development-skill`; slice 2 writes both clauses.
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** the rewrite of
  `design@agent-skills@premortem-as-watch-points`, whose list of standing questions it joins, slice 2.

### One Component is one crate by default, weighed by whether the project publishes, and a Component may be a module directory where a crate would force publication or a cycle `##component-crate-mapping`

- **Proposed:** the agent, reply to R3, from the owner's R3 notes.
- **States:** new (reply to R3); approved R4, with the owner's weighting; the table approved R5.
- **Arguments:** `argument@development-and-components@arg-29`, `argument@development-and-components@arg-33`,
  `argument@development-and-components@arg-36`, `argument@development-and-components@arg-39`,
  `argument@development-and-components@arg-41`, `argument@development-and-components@arg-46`,
  `argument@development-and-components@arg-57`.
- **Closed by:** R4: "component-crate-mapping, component-split-signals, pure-output-layer: agreed."
  R5: "component-crate-mapping agreed on the table."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 3. This thread carries `a-skill-for-creating-a-component`, closed when this milestone scheduled it.

### Signals against a split: many new dependencies, a cycle, every expected change crossing the boundary; nested Components carry the layering `##component-split-signals`

- **Proposed:** the agent, reply to R3.
- **States:** new (reply to R3); approved R4.
- **Arguments:** `argument@development-and-components@arg-29`, `argument@development-and-components@arg-34`.
- **Closed by:** R4: "component-crate-mapping, component-split-signals, pure-output-layer: agreed."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** with the head of
  #component-crate-mapping or of its own, slice 3.

### A Component's logic and the code that prints or serialises its data are by default two layers of one Component, split into two Components only where the output side takes decisions of its own `##pure-output-layer`

- **Proposed:** the agent, reply to R3, from the owner's R3 note on pure functions and output.
- **States:** new (reply to R3); approved R4.
- **Arguments:** `argument@development-and-components@arg-29`, `argument@development-and-components@arg-35`.
- **Closed by:** R4: "component-crate-mapping, component-split-signals, pure-output-layer: agreed."
  Narrowed during slice 0, after version 3 of the goal draft, on the owner's word that the printing
  stays by default in the Component that returns or defines the struct ("should probably"): "About this
  component split, in the case of a returned struct and the printer function: I think the component
  split is too strict here. The pirnt function should probably be a function in the crate/component
  that returns or defines the struct. Or we would have a component that takes no decision, thus has
  no design record, and this is also something that a component requires IMO: its own decisions and
  design."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** a shape of the
  component skill, with its head, slice 3.

### A session may build an improvement to a shipped tool as a tool of the project's own beside it, and report it in the workflow's file `##upstream-tool-loop`

- **Proposed:** the agent, reply to R3, from the owner's R3 fear about generalised tools.
- **States:** new (reply to R3); approved R4, point 1 stated in the shipped text; worded as a tool of
  the project's own, reply to R5.
- **Arguments:** `argument@development-and-components@arg-30`, `argument@development-and-components@arg-38`,
  `argument@development-and-components@arg-44`, `argument@development-and-components@arg-56`,
  `argument@development-and-components@arg-57`, `argument@development-and-components@arg-68`.
- **Closed by:** R4: "upstream-tool-loop: i think both points are very applicable. The logic to
  support that properly can wait, though the shipped agent text should state the possibility of
  doing that. For now, there is no such need, hence whhy the details can wait. Point 1. can be
  applied by any project without further support provided by knowledge-architect after all." R5:
  "upstream-tool-loop agreed."
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a head in
  `register@agent-skills@design` citing `design@agent-skills@installed-fix-goes-upstream`, slice 2.

### A test Component may depend on the Components it tests, never the reverse, where several crates share test tooling and fixtures `##shared-test-component`

- **Proposed:** the agent, reply to R4, from the owner's R4 remark on thaum-testing.
- **States:** new (reply to R4); approved R5; its condition stated as the second case, with the first
  as its example, reply to R5, approved R7.
- **Arguments:** `argument@development-and-components@arg-43`, `argument@development-and-components@arg-47`,
  `argument@development-and-components@arg-51`, `argument@development-and-components@arg-53`.
- **Closed by:** R5: "shared-test-component: agreed. This applies *at least* when testing a crate (A)
  needs to pull in another crate (B) which already depends on A, and do not want to create a cycle,
  even dev only. In additiom, this also applies more generally, whenever several crates might share
  testing mecanisms and fixtures, without depending on each other (though maybe that is the same
  case, generalized)." R7: "Agreed on the table."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** a shape of the
  component skill, slice 3.

### The Component instructions are a new component skill, with a "when to split" fragment in the design and development skills, triggered at setup and at an audit too, and nothing in planning `##component-skill-delivery`

- **Proposed:** the owner, R4, a fragment in design and a new skill; extended by the agent to four
  moments, reply to R4.
- **States:** new (reply to R4); open (reply to R5); `presumed-settled` (checkpoint of the reply to
  R6); approved R7.
- **Arguments:** `argument@development-and-components@arg-42`, `argument@development-and-components@arg-48`,
  `argument@development-and-components@arg-56`.
- **Closed by:** R7: "Agreed on the table."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** a head in
  `register@agent-skills@design`, slice 3.

### Co-development of a shipped tool through a Cargo `[patch]` waits for a first real need, as a deferred issue `##upstream-co-development`

- **Proposed:** the agent, reply to R4, split out of #upstream-tool-loop's point 2.
- **States:** new, a `deferred` issue entry by default (reply to R4); approved (checkpoint of the
  reply to R6, R7).
- **Arguments:** `argument@development-and-components@arg-38`, `argument@development-and-components@arg-44`,
  `argument@development-and-components@arg-49`.
- **Closed by:** R5: "upstream-tool-loop agreed." R7: "Agreed on the table."
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a `deferred` issue in
  `register@agent-skills@issue`, trigger "a consumer needs a change to a shipped tool before the next
  release", with its unverified premise on the checker's pin, opened at slice 2.

### Collecting feedback from real use in a target project is later work, a todo issue `##use-feedback-collection`

- **Proposed:** the owner, R5; made a `todo` issue by the agent's default, reply to R5.
- **States:** a `todo` issue entry (reply to R5); approved (checkpoint of the reply to R6, R7).
- **Arguments:** `argument@development-and-components@arg-50`.
- **Closed by:** R5: "Also, it could be generalized, as something this project asks of consumer
  projects: when a goal has observables in real use, setup a way to collect data and feedback from
  such real use, so that the goal being met can be verified. (without going overboard on data
  collection obviously). This can be left for later design work though." R7: "Agreed on the table."
- **Shape:** "Later consequences". **Harvest:** a `todo` issue in `register@agent-skills@issue`,
  opened at slice 2.

### The design skill's search may wait until the scope settles, but no thread closes before it has run, and the owner's positive words leave threads presumed-settled until then `##search-timing`

- **Proposed:** the owner, R5; the synthesis by the agent, reply to R5.
- **States:** new (reply to R5); in discussion, synthesis proposed (reply to R5); approved R6.
- **Arguments:** `argument@development-and-components@arg-52`, `argument@development-and-components@arg-54`,
  `argument@development-and-components@arg-55`, `argument@development-and-components@arg-61`,
  `argument@development-and-components@arg-62`, `argument@development-and-components@arg-63`.
- **Closed by:** R6: "search-timing: agreed on your synthesis."
- **Shape:** `spec@development-and-components@search-timing`. **Harvest:** the rewrite of
  `design@agent-skills@search-after-first-challenge`, of
  `design@agent-skills@record-searched-before-the-work` and of
  `design@agent-skills@bounded-path-in-design`, slice 4.

### A fix runs no record search; the development skill's grounding reads the record of the code it changes `##fix-grounding`

- **Proposed:** the agent, reply to R5, from the trigger of `a-fix-is-grounded-by-no-record-search`, closed when this milestone scheduled it.
- **States:** new (reply to R5); approved R6.
- **Arguments:** `argument@development-and-components@arg-58`.
- **Closed by:** R6: "fix-grounding agreed".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** the rewrite of
  `design@agent-skills@code-change-reads-its-record`, slice 2. This thread carries
  `a-fix-is-grounded-by-no-record-search`, closed when this milestone scheduled it.

### The development skill's commit step restates the claim check in one sentence, with its pointer `##commit-claim-check`

- **Proposed:** the agent, reply to R5, from the trigger of
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`.
- **States:** new (reply to R5); approved R6.
- **Arguments:** `argument@development-and-components@arg-59`.
- **Closed by:** R6: "commit-claim-check agreed".
- **Shape:** `spec@development-and-components@development-skill`. **Harvest:** a `%%` line beside the
  sentence; the issue stays open for its other options, slice 2.

### A rule-6 finding under the new clause is tagged intent instead of minor `##rule-6-finding-weight`

- **Proposed:** the agent, reply to R5, from the re-entry of
  `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`.
- **States:** new (reply to R5); approved R6.
- **Arguments:** `argument@development-and-components@arg-60`.
- **Closed by:** R6: "rule-6-finding-weight agreed".
- **Refined in slice 0:** the session refined "also tagged `intent`" to "tagged `intent` instead of
  `minor`", since `minor` marks a finding that changes no agent's act; recorded as the session's
  decision in the slice 0 commit.
- **Shape:** slice 0, landed. **Harvest:** the rewrite of
  `design@agent-skills@goal-reviewer-agent` and `design@agent-skills@goal-standard-rules`, slice 0.

### The mutation command is a separately published crate, a dependency of the maintenance crate, run as `cargo x mutate` `##mutation-command-host`

- **Proposed:** the agent, reply to R5, default the gates library; changed by the owner, R6, to a
  separately published crate; the agent then made it a dependency of the maintenance crate, not of
  the gates library, reply to R6.
- **States:** new (reply to R5); `presumed-settled`, a separate crate depended on by the maintenance
  crate (checkpoint of the reply to R6); approved R7.
- **Arguments:** `argument@development-and-components@arg-66`, `argument@development-and-components@arg-71`,
  `argument@development-and-components@arg-77`.
- **Closed by:** R6: "mutation-command-host: I would consider making it a separately published
  crate. It would either be pulled by the xtask crate of consumer projects, or pulled in directly by
  the gates crate, and as you say, available by cargo x mutate like in thaum." R7: "Agreed on the
  table."
- **Shape:** `spec@development-and-components@mutation-crate`. **Harvest:** a head in
  `register@knowledge-architect@design` for the fourth published crate, and the rewrite of
  `design@knowledge-architect@publish-credential`, slice 1.

### Two Components with one basename are refused by the checker, and the component skill documents it, closed argued `##component-basename-collision`

- **Proposed:** the agent, reply to R5, from `a-skill-for-creating-a-component`, closed when this milestone scheduled it.
- **States:** new, to ground next round (reply to R5); `argued` (reply to R6).
- **Arguments:** `argument@development-and-components@arg-57`, `argument@development-and-components@arg-72`,
  `argument@development-and-components@arg-75`.
- **Closed by:** the argument of `argument@development-and-components@arg-75`: the checker already
  refuses two anchors with one name, in `resolve_anchors` of `path@core@src/manifest.rs`, with its test
  `two_anchors_sharing_a_name_keep_the_first_declared`. The owner's R6 agreed in advance: "would
  there be value in supporting multiple components with same basename ? I would consider this to be
  quite bad practice, I see no reason to pick another fix than simply forbidding it and documenting
  that."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** a statement of the
  component skill, and a head in `register@core@design`, under its anchors subject, recording that
  two anchors never share a name, slice 3. `design@core@anchor-collisions-refused` refuses two
  anchors at one path, not two anchors of one name, and no head states the refusal by name: only
  `resolve_anchors` in `path@core@src/manifest.rs` and its test hold it. Entry test 1 of
  `primer@design-heads`, an interface others consume, admits the head, since it decides what the
  manifest accepts. Its argument: the owner's R6, "Two components with the same basename are either
  the same one, or they do not have a specific enough name.", and the reference rule that a
  reference names its anchor by one word, so two anchors of one name would make it name two.

### Out of this milestone: the fragments are not delivered differently, so the re-entry of the harness issue narrows `##harness-argued-heads-scope`

- **Proposed:** the agent, reply to R5 (Q4).
- **States:** new (reply to R5); approved R6.
- **Arguments:** `argument@development-and-components@arg-67`, `argument@development-and-components@arg-74`,
  `argument@development-and-components@arg-79`.
- **Closed by:** R6: "Q4: right, we are not changing how those are being delivered, there is no
  point handling it, change the re entry."
- **Shape:** slice 0, landed. **Harvest:** the re-entry of
  `issue@agent-skills@heads-argued-from-another-harness`, slice 0.
- **Outcome at slice 0:** the issue's re-entry reads "The next design discussion on the audit's
  saved workflows, or one that changes how the shipped fragments are delivered, or a design-record
  audit pass, whichever comes first.", which keeps the two other occasions of the earlier re-entry
  beside the narrowed one that `argument@development-and-components@arg-79` words.

### A Component's directory is named by what it does, never after the project, and its package name carries the project prefix `##component-naming`

- **Proposed:** the owner, R6; the agent named the thread, reply to R6.
- **States:** `presumed-settled` (reply to R6); approved R7.
- **Arguments:** `argument@development-and-components@arg-73`, `argument@development-and-components@arg-76`.
- **Closed by:** R6: "About component/crate naming, I think the practice from here, to avoid the
  project name from components/crate basename directories, but always prefix in crate names in
  Cargo.toml, can also be recommended. It simplifies anchors in component citation, which is quite
  nice." R7: "Agreed on the table."
- **Shape:** `spec@development-and-components@component-skill`. **Harvest:** a head in
  `register@agent-skills@design` for the recommendation the installed text makes, citing
  `design@knowledge-architect@no-component-directory-named-after-the-project`, slice 3.

### Every Rust target project has a maintenance crate whose commands run as `cargo x <command>` `##maintenance-crate-ask`

- **Proposed:** the owner, after the plan's review, ruling D2 against its default.
- **States:** approved at its proposal; an ask of knowledge-architect, which
  `goal@knowledge-architect@serves-one-kind-of-project` gives to knowledge-architect's owner.
- **Arguments:** the owner's ruling; and `goal@knowledge-architect@serves-one-kind-of-project`:
  "Nothing in knowledge-architect is built only to serve a state that does not follow an ask, other
  than what helps a project into conformance: not support in the installed text or the crates for a
  gate runner a project keeps beside the gates library, not a mode of the checker for another
  harness." This removes D2's fallback, a binary installed into a project with no maintenance crate:
  that binary helps the project into no conformance, since the project keeps running without a
  maintenance crate; it serves the state that does not follow the ask.
- **Closed by:** the owner: "D2 the xtask maintenance crate with the shape of `cargo x <command>`
  should become a required ask from this project to consumer Rust projects."
- **Shape:** `spec@development-and-components@mutation-crate`, Builds item 8. **Harvest:**
  `design@agent-skills@pinned-binary-runs-inside-the-project` and
  `design@agent-skills@xtask-pins-checker` rewritten in place, slice 1.

## Arguments

Each argument's round, who gave it, the threads it bears on, and its key words. Text in quotation
marks is verbatim; the rest is a summary written at assembly. A reference that a quotation holds is
kept as the quotation wrote it, except where noted; a bare path in a quotation is written as a
reference to the file it names, or in words where it names a kind of file in any project.
Arguments `argument@development-and-components@arg-82` to `argument@development-and-components@arg-85`
were added at the plan's review and belong to R1; they follow `argument@development-and-components@arg-81`
since a number is never reused.

### The code-quality goal asks for a review of code that the local development skill does not hold `##arg-1`

**The agent, in its reply to R1.** Threads: #code-shape-axis. Words: "`code-has-one-responsibility-per-unit-and-is-testable` says it is met only while the installed text 'holds a review of code that judges each property and whether each departure is argued'. `skill@klarch-development` only has the adversarial axis, so the published skill can't be a copy of it."

### Thaum's refinements over the local development skill, and which of them are generic `##arg-2`

**The agent, in its reply to R1.** Threads: #adversarial-scoping, #spec-first-axis, #mutation-command, #performance-opt-in, #code-shape-axis. Words: thaum's refinements over `skill@klarch-development`: scope each adversarial reviewer to one subsystem, claim family or boundary; reviewer picks tests and runs the wider suite before reporting a survivor; `rules-first` generalised to "an external specification the project implements"; `cargo mutate run/restore` "replaces about 40 lines of manual worktree procedure"; bench runs before and after (shape for performance); 800-line split threshold is thaum's own.

### A general quality reviewer has no standard to judge against `##arg-3`

**The agent, in its reply to R1.** Threads: #code-shape-axis. Words: "The nearest rival is a general 'overall quality' reviewer. That rival loses because it has no standard to judge against. It would return taste findings, which `primer@working-prose` asks to turn into checkable forms anyway, and the goal already names the checkable forms. Cost: one read-only agent, with no build."

### Test economy folds into the code-shape axis, since a separate axis reads the same diff again `##arg-4`

**The agent, in its reply to R1.** Threads: #test-economy. Words: "each test the diff adds names its claim and has a recorded discrimination check; a test with neither is a finding. Today only the author checks this, and the author is the one adding the tests. The rival, a separate axis, costs one more dispatch to read the same diff."

### Nothing defeats the adversarial scoping `##arg-5`

**The agent, in its reply to R1.** Threads: #adversarial-scoping. Words: "Nothing defeats this."

### The spec-first axis applies only where a project implements an external specification `##arg-6`

**The agent, in its reply to R1.** Threads: #spec-first-axis. Words: "Projects without one never trigger it, which meets the binding criterion with no opt-in mechanism."

### Taking in the mutation command is the owner's scope choice, and keeping the manual procedure fails the mechanical-tasks goal `##arg-7`

**The agent, in its reply to R1.** Threads: #mutation-command. Words: Q1 scope choice per `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`; "Rival: keep the manual worktree procedure in the installed text. That rival fails the mechanical-tasks goal." Size: "about 1470 lines in thaum, all of `path@elsewhere@~/dev/games/thaum/tools/mutate/src/`".

### Widening the code claims closes an open issue, and a documentation axis would duplicate its method `##arg-8`

**The agent, in its reply to R1.** Threads: #code-claims-scope. Words: widening code claims "closes `the-code-claims-axis-reads-plan-documents-only`. That issue already records a thaum case where an improvised reviewer for this question produced 'the review's most consequential findings'. The rival is a separate documentation axis. It would duplicate the code-claims agent's method."

### A reviewer that judges performance by reading code cannot reproduce its findings `##arg-9`

**The agent, in its reply to R1.** Threads: #performance-opt-in, #performance-review-axis. Words: "The rival is a reviewer that judges performance by reading code. It loses because a performance claim without a measurement can't be reproduced, so `skill@knowledge-architect-review@review-invariants` 'reproduce before claiming' would have it drop its own findings. A project that declares no path pays nothing." Shape: design head declaring paths and a measure command, commit message records both figures; Q2 design head vs manifest key.

### Read-only axes once per branch and the adversarial axis at checkpoints add at most two dispatches `##arg-10`

**The agent, in its reply to R1.** Threads: #review-cadence. Words: "adversarial and spec-first run at development checkpoints, as today; code shape (with test economy) and code claims are read-only and run once per branch ... That adds at most two dispatches per branch, with no builds. `issue@agent-skills@the-retrospective-counts-no-review-cost` is the instrument that would show whether this is too many."

### No security axis and no general quality axis `##arg-11`

**The agent, in its reply to R1.** Threads: #code-shape-axis. Words: "A security axis. The adversarial brief already attacks accepted inputs. `unsafe` is a lint gate ... not a reviewer's job." and no general "overall quality" axis, for the reason under #code-shape-axis.

### The work takes the full path `##arg-12`

**The agent, in its reply to R1.** Threads: #code-shape-axis, #code-claims-scope, #mutation-command. Words: "This takes the full path: a new installed skill, probably one new installed agent (code shape), a scope change to an existing agent (code claims), and possibly a new command."

### A goal written before any design can restrict itself and work against its title `##arg-13`

**The owner, R2.** Threads: #code-quality-goal-wording. Words: "it could significantly restrict itself, and the body of the goal could even end up working against the general intent of its title." and "Rules too strict tend to make agents take considerably misaligned actions when following them becomes impossible." and "this aspect of promoting better code quality has to come from long term experience and usage, rather than a pre stated goal with restricting clauses."

### "One responsibility" has readings that lead to unreadable code `##arg-14`

**The owner, R2.** Threads: #code-shape-axis. Words: "single responsibility can have a wide range of readings, the most extreme can lead to requiring only one statement per function and write unreadable code where everything is a function, and everything ends up includign everything else."

### Mutation testing and test economy apply less to tests of mocks and golden references `##arg-15`

**The owner, R2.** Threads: #mutation-command, #test-economy. Words: "mutation testing does not apply well to every project or kind of tests. It should apply well as long as the tests are on pure logic ... But tests that mock large environments, or golden reference tests, I think this would apply a bit less. Same for the test-economy axis btw."

### Code claims cover the correctness of documentation, not its completeness `##arg-16`

**The owner, R2.** Threads: #code-claims-scope, #api-doc-completeness. Words: "this covers the side of documentation correctness ... but not the side of documentation completeness. This matters mostly for documentation of public facing code/API."

### Performance non-regression needs a benchmark procedure, and improvement is a task of its own `##arg-17`

**The owner, R2.** Threads: #performance-opt-in, #performance-review-axis. Words: "for performance non-regression, review is not needed, a benchmark procedure is better. ... [improvement] should not happen on a review ... more as a task of its own, and would need the same mechanical benchmark procedure."

### The goal's departure clause leaves a one-line fix only two misaligned moves `##arg-18`

**The agent, in its reply to R2.** Threads: #code-quality-goal-wording. Words: the goal clause "A change that departs from them, or that leaves an edited unit departing from them, states why at the code, or in a design head the code cites." Take "a one-line fix in an existing 300-line function that does three things. Under this clause the session has two compliant moves: write a justification comment it has no real argument for; refactor the function, which nobody asked for." Proposed reword: keep two-layer testability, three properties become the default shape the review weighs, departure argued when the review reports it, instructions grow from `goal@knowledge-architect@the-workflow-improves-through-real-use`. "Until you rule, the goal stands as written."

### One responsibility read as one reason to change, with fragmentation a finding too `##arg-19`

**The agent, in its reply to R2.** Threads: #code-shape-axis. Words: reading "one reason to change", not "does one thing". "A finding must name two concrete changes someone could plausibly ask for, show that each would touch the unit independently, and show that they are entangled. Fragmentation is a finding too." (with the two-sided code example

### Mutation evidence depends on a plausible wrong implementation, not on the kind of test `##arg-20`

**The agent, in its reply to R2.** Threads: #test-economy. Words: "Whether a mutation applies depends less on the kind of test than on whether a plausible wrong implementation exists in the logic it exercises." Table per kind (pure logic, golden/snapshot, heavy mocks, nondeterministic). "every added test names its claim, for any kind. Mutation evidence is required for tests of logic only." For heavy mocks the finding is usually the test of the side-effect layer, which the goal says needs no tests. cargo-mutants' "Limitations" chapter cited for nondeterministic.

### Thaum's run and restore are generic, its sweep is not, and cargo-mutants does another job `##arg-21`

**The agent, in its reply to R2.** Threads: #mutation-command, #diff-mutation-sweep. Words: thaum `path@elsewhere@~/dev/games/thaum/tools/mutate/` has two modes, 1470 lines. `run`/`restore` is generic (applies one mutation via `--replace OLD --with NEW` or `--patch`, baseline, optional `--build` probe, uniqueness check, prints `mutation applied to <file>`, byte-for-byte restore, crash recovery; `path@elsewhere@~/dev/games/thaum/tools/mutate/src/lib.rs` 404 lines plus `run`/`unwind`/`restore` in `main.rs`; thaum glue is the `THAUM_CHECKOUT` build tie). `sweep` is thaum's own (deletes invariant-check sites under one directory). cargo-mutants (book chapters "Mutation patterns", "Copying the tree", "Testing code changed in a diff"): generates its own mutants, copies the tree, baseline, `--in-diff FILE`; "None of the chapters I read offers a mutation you write yourself." Jobs table: author check of a known trap -> `run`; adversarial reviewer's own mutations -> `run`; sweep for survivors in changed code -> cargo-mutants `--in-diff`.

### A diff mutation sweep with cargo-mutants is opt-in, at one build and one test run per mutant `##arg-22`

**The agent, in its reply to R2.** Threads: #diff-mutation-sweep. Words: optional step `cargo mutants --in-diff` before adversarial review, survivors as leads. "It costs one build and one test run per mutant, so it is opt-in. Its book warns that a diff touching only test code produces no mutants."

### Take in run and restore, since the author's check is the core of the discrimination step `##arg-23`

**The agent, in its reply to R2.** Threads: #mutation-command. Words: Q3: "take in the generic `run`/`restore` part ... My default is yes, because the author's check is the core of the discrimination step, and today it is about 40 lines of manual procedure. cargo-mutants would be an offered quality tool, not a replacement."

### The presence of documentation is a lint, and its content is read by the widened code-claims reviewer `##arg-24`

**The agent, in its reply to R2.** Threads: #api-doc-completeness, #code-claims-scope. Words: "Presence is a lint: `missing_docs` (rustc), and `clippy::missing_errors_doc` and `clippy::missing_panics_doc` (pedantic). ... Content needs reading ... folding the reading part into the widened code-claims agent ... costs no extra dispatch. It applies when the diff changes a public interface others consume."

### The performance review is withdrawn, and improving performance becomes an activity of its own `##arg-25`

**The agent, in its reply to R2.** Threads: #performance-review-axis. Words: "Your original performance-review thread is withdrawn. Improving performance becomes a separate activity that needs the benchmark procedure. I'll open it as an issue entry, not design it here."

### cargo-mutants ignores rust flags set in the cargo configuration file `##arg-26`

**The agent, in its reply to R2.** Threads: #diff-mutation-sweep. Words: "cargo-mutants and the cargo configuration file: its 'Limitations' chapter says it ignores rust flags set in the cargo configuration file, `path@knowledge-architect@.cargo/config.toml` in this repository. This bears on #diff-mutation-sweep: a project that sets flags there gets mutants built differently from its gates."

### The goal standard asks for a goal checkable from the tree, which is not always doable `##arg-27`

**The owner, R3.** Threads: #goal-standard-measure, #code-quality-goal-wording. Words: "the goal standard asks for a goal that can be checked from the tree. However, this is not always doable. I think this is the main defect of the goal standard that led to this goal being too specific and risking to have it work against itself." Proposes an issue on the goal standard, perhaps corrected now. Also: change slug to "the workflow leads to improved code"; "The current one looks like it applies to the project itself, not to the shipped skills and has to be observed in target projects." Keep wording "relatively vague".

### The retrospective watches whether a rule on Rust code was too hard or too strict `##arg-28`

**The owner, R3.** Threads: #retrospective-code-rules-question. Words: "the retrospective has to watch specifically for how any Rust implementation went, whether any rule/guideline was too hard to follow or too strict."

### The owner's guidelines for splitting a project into Components `##arg-29`

**The owner, R3.** Threads: #component-crate-mapping, #component-split-signals, #pure-output-layer. Words: component splitting notes (pasted block): one component = coherent whole, own public API with a reason to change, own tests, coherent design; compartmentalisation "simplif[ies] design searches and allow[s] working locally"; "a crate exists when it has its own tests, its own public API, and a reason to change on its own"; layers/nested components; dependency count or cycle means wrong split; split pure libraries from non pure ones; "Separating pure logic from non pure routines with side effects is very good for long term health"; drawback: many small crates on crates.io.

### A target project that no longer owns a tool may stop improving it `##arg-30`

**The owner, R3.** Threads: #upstream-tool-loop. Words: "if the target project no longer own them, this might discourage the target projects to work on improving those provided generic tools ... target project are actually the ones that can drive this ... This is kinda the original reason why we wrote our own mutation tool in thaum." Possible solution: consumer pins a branch of knowledge-architect.

### Rule 6 does not forbid a vague goal, since the tree shows the mechanism that pursues it `##arg-31`

**The agent, in its reply to R3.** Threads: #goal-standard-measure. Words: rule 6 of `skill@knowledge-architect-goal-setting@goal-or-not` says "What shows it met is in what the project builds: an observable of the tree as it stands [...] Evidence from the project's use [...] is not the measure". `design@agent-skills@goal-standard-rules` records "on the owner's argument that in most projects a measure on use has limited feasibility". With rule 2 ("It has one reading") an outcome judged only in use is turned into a property list. "Rule 6 doesn't actually forbid a vague goal. The tree can also show the mechanism that pursues the outcome and corrects itself from use ... The standard never says this, so the drafter reaches for the most specific observable available. I'd call that the defect." Proposed clause (version 1) and "My recommendation: fix it in this branch, not through an issue" because a draft written under the old standard "gets pushed back toward the list"; adds to rule 6, no reversal; goes through the design skill's backstop.

### The standing question proposed for the retrospective `##arg-32`

**The agent, in its reply to R3.** Threads: #retrospective-code-rules-question. Words: proposed standing question: "Did an installed rule about code or tests prove too hard or too strict to follow in this session's Rust work, or get followed against the change's interest?" fits `goal@knowledge-architect@the-workflow-improves-through-real-use`; "I have no rival to offer."

### One reason to change is the test for a function and for a Component alike `##arg-33`

**The agent, in its reply to R3.** Threads: #component-crate-mapping, #code-shape-axis. Words: `goal@knowledge-architect@projects-are-split-into-components` "already holds four of your points almost word for word" so "'one reason to change' is the test for a function and for a Component alike." Default one Component = one crate; may be a module directory when a crate would force publication or a dependency cycle. Two Cargo facts, "from Cargo's documentation but did not re-check this session": "a published crate can only depend on published crates"; "Cargo rejects cyclic package dependencies."

### The signals against a split `##arg-34`

**The agent, in its reply to R3.** Threads: #component-split-signals. Words: signals against a split: many new dependencies, a cycle, every expected change crossing the boundary; nested Components for layering.

### The pure and output layers are the two-layer property at the scale of a Component `##arg-35`

**The agent, in its reply to R3.** Threads: #pure-output-layer. Words: the owner's pure struct plus printing layer "is the two-layer property of the code-quality goal, applied to a Component. Same rule, two scales." Issue likely `issue@knowledge-architect@command-output-is-not-declared-a-contract` (written under the core's anchor in the discussion).

### Many small published crates are accepted practice, at a cost in release order `##arg-36`

**The agent, in its reply to R3.** Threads: #component-crate-mapping. Words: "Rust practice accepts it (gitoxide publishes dozens of `gix-*` crates). The cost is release order and version bumps, and the module-directory Component is the way out where that cost dominates."

### The value of a run is a mutation one writes oneself, and a recorded list waits for a second use `##arg-37`

**The agent, in its reply to R3.** Threads: #mutation-command. Words: "the value is a mutation you write yourself, applied safely over uncommitted work. cargo-mutants chooses its own. Thaum's 'list of recorded ones' is `sweep` ... I'd ship `run`/`restore` alone and wait for a second real use before generalising a recorded list."

### A local improvement reported upstream, and co-development through a Cargo patch `##arg-38`

**The agent, in its reply to R3.** Threads: #upstream-tool-loop, #upstream-co-development. Words: shape: (1) local improvement allowed and reported to the workflow file (route exists in `primer@met-outside-the-task`; `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: "Until a part taken in ships, the project's own copy of it stays"); (2) co-development via Cargo `[patch.crates-io]` on a knowledge-architect git branch. "There is an unverified premise in point 2. A patched build reports knowledge-architect's in-tree version, so the consumer's `checker-version` pin must match it, or the check refuses to run."

### One reason to change applies at both scales, and the sweep stays out `##arg-39`

**The agent, in its reply to R3.** Threads: #code-shape-axis, #component-crate-mapping, #mutation-command. Words: "One reason to change is applied at both unit and Component scale. Ground: the goal's own wording"; "`sweep` stays out of #mutation-command. Ground: it is tied to thaum's invariant-check directory and its `InvariantViolation` pattern (`path@elsewhere@~/dev/games/thaum/tools/mutate/src/main.rs`)."

### The goal standard serves every project, whose observables in use may be other than instructions to agents `##arg-40`

**The owner, R4.** Threads: #goal-standard-measure. Words: "this goal standard applies to other projects, and their on-use observable criteria might go through other means that installed agent instructions (such as a tool producing the correct effect in practical conditions, or users being satisfied by gui ergonomy)."

### The crate count is weighed by whether the project publishes `##arg-41`

**The owner, R4.** Threads: #component-crate-mapping. Words: weighting by publication: "If the practice accepts it, it can probably be considered a low weight cost. It also has to be weighed against how useable the crates would be for the community ... For a non publishing target project, one component = one crates has more weights, because the crate split is stronger isolation." (thaum and rootpoint plan to publish; two planned consumers will not.)

### Component instructions enter at a design discussion, through a fragment and a new skill `##arg-42`

**The owner, R4.** Threads: #component-skill-delivery. Words: "the moment were they make sense to enter is design discussion ... not all design discussion might need the procedure (this should be an occasional activy, creating a component). I suggest design has a fragment on when to split, while a new skill for component creation holds the split standard, plus how to create it, and the possible shapes ... Or maybe the methodology enters at planning phase."

### A shared testing crate centralises the test tooling and avoids dependency cycles `##arg-43`

**The owner, R4.** Threads: #shared-test-component. Words: "splits integrated testing under a dedicated crate thaum-testing is also quite good general practice, as it allows to centralize tooling for integrated testing (such as fixtures and oracles) ... and it also avoids circular dependencies (even if dev only, circular deps make separation of responsibility quite unclear IMO)."

### The shipped text states the local improvement, and its support waits `##arg-44`

**The owner, R4.** Threads: #upstream-tool-loop, #upstream-co-development. Words: "The logic to support that properly can wait, though the shipped agent text should state the possibility of doing that. ... Point 1. can be applied by any project without further support provided by knowledge-architect."

### The clause reworded for any project, with two readings and the first as the default `##arg-45`

**The agent, in its reply to R4.** Threads: #goal-standard-measure. Words: "Where the outcome is judged only in use, the observable is what the project builds to pursue it and to learn from its use whether it is met: such as a test of a tool under the conditions of its use, a route by which its users' reports reach the project, or, for instructions to agents, a review that weighs them and a retrospective that asks how they fared. It is not a list of the properties the outcome is taken to consist of." Consequence: two readings; "My default is the first reading. It keeps that premise [owner's feasibility argument], and evidence from use still 'shows it unmet in practice' as rule 6 already says." The second reading "would reverse part of rule 6 ... it changes one sentence of rule 6".

### The crate mapping as a table by publication status `##arg-46`

**The agent, in its reply to R4.** Threads: #component-crate-mapping. Words: table: publishing projects default one crate per Component, low cost weighed against standalone usability; never-publishing, stronger default. "The Cargo fact still applies to the first row: a published crate can only depend on published crates."

### Thaum's testing crate is depended on one way only, and a dev-dependency back builds the crate twice `##arg-47`

**The agent, in its reply to R4.** Threads: #shared-test-component. Words: thaum's shape checked in its `Cargo.toml` files: `path@elsewhere@~/dev/games/thaum/tools/thaum-testing/` depends on `thaum-engine` and `thaum-ai`; holds cross-Component tests in its own `tests/` with fixtures, traces, oracles; only `path@elsewhere@~/dev/games/thaum/tools/bench/` depends on it; no crate under test depends on it. Rival: shared helpers behind a `test-utils` feature or dev-dependency back onto the testing crate; "From Cargo's behaviour as I know it (not re-checked this session), that rival is allowed, but it builds the crate under test twice, and its types then don't match across the two copies." A shape of the component skill, not a rule.

### A Component is created at four moments, and planning is not one of them `##arg-48`

**The agent, in its reply to R4.** Threads: #component-skill-delivery. Words: decision to create or split a Component arises at four moments (design discussion, development, setup, project audit). Proposal: a shared "when to split" fragment in design and development per `design@agent-skills@shared-text-fragment`; a new component-creation skill (four properties, signals, crate mapping by publication, shapes, procedure), description triggering at setup and audit too. "Planning: no. Whether a Component exists is a decision about how the project is built, with goals of its own. Planning only cuts work that is already decided." Premise to check: the skill must point to `skill@knowledge-architect-setup`, not restate it, or it breaks `goal@agent-skills@no-contradiction-in-workflow`.

### Co-development waits for a first real need, as a deferred issue `##arg-49`

**The agent, in its reply to R4.** Threads: #upstream-co-development. Words: point 2 (`[patch]`) waits for first real need; record as a `deferred` issue "whose trigger is 'a consumer needs a change to a shipped tool before the next release'", with the unverified pin premise.

### A project with a goal judged in use collects feedback from that use `##arg-50`

**The owner, R5.** Threads: #goal-standard-measure, #use-feedback-collection. Words: "the evidence needs to be read in the tree. ... the projects must implement a way to receive feedback form real use, and collect it in the project, much like we do with retrospectives. I think that this is the better reading of both, because it has a systematic method of checking it ... when a goal has observables in real use, setup a way to collect data and feedback ... (without going overboard on data collection obviously). This can be left for later design work though."

### A shared test Component applies at least to a dependency cycle, and more generally to shared fixtures `##arg-51`

**The owner, R5.** Threads: #shared-test-component. Words: "This applies *at least* when testing a crate (A) needs to pull in another crate (B) which already depends on A, and do not want to create a cycle, even dev only. In additiom, this also applies more generally, whenever several crates might share testing mecanisms and fixtures, without depending on each other."

### Searching after a few rounds gave more precise results here `##arg-52`

**The owner, R5.** Threads: #search-timing. Words: "Usually, it runs much more early ... But I think it was a good thing that it ran late here, because the subject has been shifted and expanded at least twice ... Maybe this should become the usual practice, searching after a few rounds. It allows freer discussion of the design, which will then be confronted to the current records."

### The cycle case is a special case of a shared test setup `##arg-53`

**The agent, in its reply to R5.** Threads: #shared-test-component. Words: "The first case is a special case of the second, because B is one of the crates sharing the test setup. The condition can be stated as the second case alone, with the first given as its example."

### The current search order was the owner's proposal, and threads may close before the record is read `##arg-54`

**The agent, in its reply to R5.** Threads: #search-timing. Words: head `design@agent-skills@search-after-first-challenge` "records the reason as 'so that the costly search gets the most precise work', and the order was your proposal. The rival it records, a search before the challenge and another after, lost on cost: it doubles the search." Cost of the owner's proposal: "the risk that threads close before the record has been read. In this discussion, six threads were approved before the search ran."

### The search before the first round was skipped `##arg-55`

**The agent, in its reply to R5.** Threads: #search-timing. Words: "I skipped the search before the first round, which `skill@knowledge-architect-design@discussion-loop` step 1 requires. That was a departure from the installed skill, not a decision."

### Leads met while the search ran `##arg-56`

**The agent, in its reply to R5.** Threads: #component-skill-delivery, #upstream-tool-loop. Words: two progress notes while the search ran. A lead: `a-skill-for-creating-a-component` already exists. A lead: `design@agent-skills@installed-fix-goes-upstream` may conflict with #upstream-tool-loop.

### Nothing the search returned defeats an approved thread, and four constraints follow `##arg-57`

**The agent, in its reply to R5.** Threads: #code-shape-axis, #spec-first-axis, #upstream-tool-loop, #mutation-command, #component-crate-mapping, #component-basename-collision. Words: "Nothing the search returned defeats a thread you approved." Four constraints: #code-shape-axis and #spec-first-axis must be installed agents because `rejected@agent-skills@one-generic-auditor-agent` ("drafts came out uneven where the brief was the only standard") and `design@agent-skills@additions-review-agent`; #upstream-tool-loop per `design@agent-skills@installed-fix-goes-upstream` must be "a tool of the project's own beside the shipped one, never a patch of it"; #mutation-command not in the checker's binary per `rejected@gates@gates-command-of-the-checker` ("It would make the document checker run cargo, the linter and the tests") and `design@knowledge-architect@no-operation-loses-content`; #component-crate-mapping: `a-skill-for-creating-a-component` already records the owner's lean and crates.io doubt; leaves open two Components with the same basename (`design@core@anchor-collisions-refused` refuses only two anchors at one path).

### A fix runs no record search `##arg-58`

**The agent, in its reply to R5.** Threads: #fix-grounding. Words: from `a-fix-is-grounded-by-no-record-search`. "Default: no search per fix. The development skill's grounding step is the reading in `design@agent-skills@code-change-reads-its-record` ... New features and additions already go through the design skill's grounding, and its search, per `design@agent-skills@settled-means-recorded`." Ground: the issue records the owner's statement "that grounding in the design record concerns additions and new features, not bug fixes"; "The cost: the issue's search took 10 agents, and this one took 18." The issue closes with that decision.

### The commit step restates the claim check in one sentence `##arg-59`

**The agent, in its reply to R5.** Threads: #commit-claim-check. Words: from `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`. Default: "the development skill's commit step restates the claim check in one sentence, with its pointer to `primer@verify-before-relying`. `primer@where-knowledge-goes` allows a one-sentence restatement." "It only partly fixes the mechanism ('an instruction read once at the start ... is not recalled at the moment it binds'), since the skill is also loaded once per piece of work." The issue stays open for the other options (a line printed by `check --staged` or `commits`).

### A rule-6 finding under the new clause is also tagged intent `##arg-60`

**The agent, in its reply to R5.** Threads: #rule-6-finding-weight. Words: from `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`; re-entry "when work next changes the goal reviewer's definition". Mechanism: "the reviewer reports a clause under the rule its wording breaks most visibly, and a reach shown as a minor measure finding does not reach the owner as a question of intent." `design@agent-skills@goal-standard-rules` tags every rule-6 finding `minor`. Default: "a rule-6 finding under the new clause is also tagged `intent`".

### The evidence for a later search `##arg-61`

**The agent, in its reply to R5.** Threads: #search-timing. Words: "This session. The scope widened twice before the search. ... Running it at the first round would have meant one search followed by searches for each new subject, per `design@agent-skills@record-searched-before-the-work`." and `issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`: a scope "widened after the search makes its reasons stale".

### The evidence against a later search `##arg-62`

**The agent, in its reply to R5.** Threads: #search-timing. Words: `rejected@agent-skills@conformance-reader-at-the-checkpoint`: a reader after the owner has ruled "reads rulings already given". "In this session I closed seven threads before the search. None was defeated, but you approved #component-crate-mapping while restating premises that `a-skill-for-creating-a-component` already held."

### The synthesis: the search may wait, and no thread closes before it has run `##arg-63`

**The agent, in its reply to R5.** Threads: #search-timing. Words: "the search may wait until the scope settles, but no thread closes before it has run. Until then, your positive words leave threads `presumed-settled`. That keeps the free discussion and the more precise grades, and removes the 'rulings already given' cost. It changes `design@agent-skills@search-after-first-challenge`, whose order was your proposal, so the change is yours to approve."

### The constraints on the development skill `##arg-64`

**The agent, in its reply to R5.** Threads: #code-shape-axis, #test-economy, #adversarial-scoping, #spec-first-axis, #code-claims-scope, #fix-grounding. Words: no PR or CI wording (`issue@agent-skills@installed-text-assumes-pull-requests-and-ci`); no reference to scoped `CLAUDE.md` (`issue@core@a-home-for-developer-contracts-outside-agent-configuration`); do not restate the guard-reference rule (`issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`); each new behaviour instruction names real-use evidence (`design@agent-skills@behaviour-additions-need-real-use`; table: ported steps -> `skill@klarch-development` and thaum, and the owner's R1 on the discrimination step (`argument@development-and-components@arg-82`); adversarial scoping, spec-first -> thaum, and for adversarial scoping the owner's R1 (`argument@development-and-components@arg-83`); code shape, test economy -> owner's reports in this discussion, test economy among them the owner's R1 (`argument@development-and-components@arg-82`); code claims -> the issue's two cases); `skill@klarch-development` and thaum-development become overlays that may only add (`issue@agent-skills@patching-an-installed-skill`).

### The adversarial axis shipping ends the premise of the running-time issue `##arg-65`

**The agent, in its reply to R5.** Threads: #adversarial-scoping. Words: both instances were adversarial reviewers queuing mutations; owner had withdrawn approval as "a project specific problem ... fixed locally on thaum's side"; "Once the adversarial axis ships, that premise no longer holds. Unverified: what thaum's local fix is."

### The mutation command hosted by the gates library, and no loss of content `##arg-66`

**The agent, in its reply to R5.** Threads: #mutation-command-host. Words: "Default: the gates library, run through the maintenance crate as `cargo x mutate`, beside `cargo x gates`. The gates library already starts cargo, and `design@agent-skills@xtask-pins-checker` gives the maintenance crate 'the project's other repeated tasks'. Rival: the checker's binary. It loses on the rejected entry above." Constraint from `design@knowledge-architect@no-operation-loses-content`: "the tool must never lose uncommitted content. Thaum's tool writes a recovery record before touching the file and restores only a byte-identical file; its README states both."

### The fragments issue is out of this milestone, with a narrower re-entry `##arg-67`

**The agent, in its reply to R5.** Threads: #harness-argued-heads-scope. Words: `issue@agent-skills@heads-argued-from-another-harness` re-entry "the next design discussion ... on the shipped fragments" is met; "My default: take it out of this milestone. The issue stays, its re-entry becomes 'the next design discussion that changes how fragments are delivered', and our work only adds a fragment and edits one."

### Three decisions grounded in the record `##arg-68`

**The agent, in its reply to R5.** Threads: #code-shape-axis, #spec-first-axis, #upstream-tool-loop, #mutation-command-host. Words: "#code-shape-axis and #spec-first-axis are installed agents. Ground: `design@agent-skills@additions-review-agent` and `rejected@agent-skills@one-generic-auditor-agent`." "#upstream-tool-loop is worded as a project's own tool. Ground: `design@agent-skills@installed-fix-goes-upstream`." "The mutation command does not go in the checker's binary. Ground: `rejected@gates@gates-command-of-the-checker`."

### The size of the search, and one searcher's hand-back `##arg-69`

**The record search's searchers, relayed by the agent in its reply to R5.** Threads: #retrospective-code-rules-question. Words: 18 searchers, 136 `bears`, 127 `may-bear`; 123 of 127 `may-bear` not read whole. A searcher's hand-back: `design@agent-skills@tripwire-recording-authority` and `design@agent-skills@tripwire-admission-test` constrain the premortem; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` touched by the retrospective question.

### Scoped adversarial reviewers that run partial tests fixed the running time `##arg-70`

**The owner, R6.** Threads: #adversarial-scoping. Words: "the fix I mentionned was sending multiple adversarial on a smaller, scoped set of the changes, plus the instruction to run partial tests. In practice, until now, this has proven to be enough of a fix ... without adding constraints or a cap that might prevent them from finding more."

### The mutation tool as a separately published crate `##arg-71`

**The owner, R6.** Threads: #mutation-command-host. Words: "I would consider making it a separately published crate. It would either be pulled by the xtask crate of consumer projects, or pulled in directly by the gates crate ... available by cargo x mutate like in thaum."

### Two Components with one basename are one Component, or badly named `##arg-72`

**The owner, R6.** Threads: #component-basename-collision. Words: "Two components with the same basename are either the same one, or they do not have a specific enough name."

### No project name in a Component's directory, and the project prefix in a crate's name `##arg-73`

**The owner, R6.** Threads: #component-naming. Words: avoid the project name in component/crate directory basenames, prefix crate names in Cargo.toml: "It simplifies anchors in component citation, which is quite nice."

### The fragments are not being delivered differently, so the re-entry changes `##arg-74`

**The owner, R6.** Threads: #harness-argued-heads-scope. Words: "we are not changing how those are being delivered, there is no point handling it, change the re entry."

### The checker already refuses two anchors with one name `##arg-75`

**The agent, in its reply to R6.** Threads: #component-basename-collision. Words: "The checker already refuses two anchors with the same name, so #component-basename-collision only needs documenting. The evidence is `path@core@src/manifest.rs`, line 1364, which reports "`{}` names 2 anchors: {} and {}" and says to "rename or move one", plus its test `two_anchors_sharing_a_name_keep_the_first_declared`. That settles what `a-skill-for-creating-a-component` left "not established". The component skill states the rule; the checker already enforces it." The refusal now stands in `resolve_anchors` of `path@core@src/manifest.rs`, at line 1367 of the tree this document was written against.

### The directory half of the naming rule is forced, and the package prefix is a convention `##arg-76`

**The agent, in its reply to R6.** Threads: #component-naming. Words: "a Component's directory is named by what it does, never after the project; the package name in `Cargo.toml` carries the project prefix. The first half is forced: a directory named after the project collides with the root's anchor, which the checker refuses. The second half is a convention, and its benefit is the one you name: short anchors in references." Recorded here in `design@knowledge-architect@no-component-directory-named-after-the-project`.

### The maintenance crate depends on the mutation crate directly, not through the gates library `##arg-77`

**The agent, in its reply to R6.** Threads: #mutation-command-host. Words: "I'd have the maintenance crate depend on it directly, not through the gates library. The gates library runs a project's merge gates, and a mutation is no gate: routing it through gates would make every gates user build it." Consequences of a fourth published crate: joins the release procedure (package lists, version rules, publish order); "Its first version can't be published by the release workflow, per `design@knowledge-architect@publish-credential`, which the searcher cited and I did not read whole"; the setup's maintenance-crate snippet gains the dependency and subcommand, tested per `design@agent-skills@rust-templates-one-home`. Decided line: "Ground: the gates library's purpose (`design@gates@gates-crate`, cited by the searchers, not read whole). Contest it if you meant the gates crate."

### The owner's fix for the running time is the adversarial scoping `##arg-78`

**The agent, in its reply to R6.** Threads: #adversarial-scoping. Words: "Your fix is exactly #adversarial-scoping: scoped reviewers that run partial tests. So `a-reviewer-s-running-time-is-unbounded` closes when #adversarial-scoping ships, with no cap added."

### The re-entry is changed in the milestone's first branch `##arg-79`

**The agent, in its reply to R6.** Threads: #harness-argued-heads-scope. Words: "The `heads-argued-from-another-harness` re-entry becomes 'the next design discussion that changes how fragments are delivered'. The issue is edited in the milestone's first branch."

### No acceptance criterion, and five proposed slices `##arg-80`

**The agent, in its reply to R6.** Threads: #code-quality-goal-wording, #mutation-command-host, #component-skill-delivery, #search-timing. Words: no acceptance criterion: "each cause is either a step claim (cause 8) or evidence that arrives only through retrospectives or history." Proposed slices (one branch each): 1 goal-standard clause, rule-6 weight, goal reworded; 2 mutation crate and `cargo x mutate`; 3 installed development skill (port, axis agents, code-claims widening, cadence, retrospective question); 4 component skill, fragment, naming rule; 5 design skill search timing.

Slices 1 to 5 of this proposal are slices 0 to 4 of the implementation sequence.

### The goal standard and the reworded goal land first, as slice 0 `##arg-81`

**The owner, R7.** Threads: #goal-standard-measure, #code-quality-goal-wording. Words: "The goal standard and reworded goal should be done quickly, merged with the milestone plan (or maybe applied before reviewing the plan), as a "slice 0". If we apply it after the milestone plan, the plans won't be aligned with current goals ,and review might end up giving conflicting resluts I fear. The other slices of the milestone are good"

### Showing that tests discriminate costs time and keeps the test suite clean `##arg-82`

**The owner, R1.** Threads: #test-economy, and the discrimination step the development skill ports
from `skill@klarch-development`. Words: "In particular showing that tests discriminates. This often
costs times during development, but it leads to overall much better quality of the test suite,
because agents tend to add lots of tests on a whim, which pollute the test suite for little to no
gain."

### The adversarial axis is the most important one `##arg-83`

**The owner, R1.** Threads: #adversarial-scoping. Words: "The adversarial axis is the most important
one IMO, and works really well".

### Coverage of the external specification comes before performance `##arg-84`

**The owner, R1.** Threads: #performance-opt-in, #spec-first-axis. Words: "I have never focused on
performance reviews, because in thaum, for initial development, I rank coverage of the external spec
to come first, optimisations being added later (there is no point optimizing a non finished engine),
and here because it is less critical, everything already runs quite fast for the use cases."

### Performance is opt in per project, for the code paths where it is critical `##arg-85`

**The owner, R1.** Threads: #performance-opt-in. Words: "So it should be opt in per project, for code
paths where it is actually critical, or a si;milar strategy."

## New names, in one place

```text
name                                              file                                                          slice
the new clause of rule 6                          crates/agent-skills/fragments/goal-standard.md                0
goal `goal@knowledge-architect@the-workflow-leads-to-better-code` (the     docs/goals.md, replacing code-has-one-responsibility-         0
  owner ruled its wording at slice 0)               per-unit-and-is-testable
Component mutate, package knowledge-architect-    crates/mutate/ (its Cargo.toml, src/lib.rs, a library        1
  mutate                                            alone, README.md, CLAUDE.md, CRATES-IO.md,
                                                    docs/)
subcommand mutate of the maintenance crate,       tools/xtask/src/main.rs, and a module of its own              1
  cargo x mutate run, cargo x mutate restore
skill knowledge-architect-development             crates/agent-skills/content/skills/development/SKILL.md      2
agent knowledge-architect-code-shape-reviewer     crates/agent-skills/content/agents/code-shape-reviewer.md     2
agent knowledge-architect-spec-first-reviewer     crates/agent-skills/content/agents/spec-first-reviewer.md     2
agent knowledge-architect-adversarial-reviewer    crates/agent-skills/content/agents/adversarial-reviewer.md    2
  (default name; the audit checks it against
  the naming rule)
the standing question on rules about code         crates/agent-skills/content/skills/retrospective/SKILL.md     2
skill knowledge-architect-component-creation      crates/agent-skills/content/skills/component-creation/        3
  (default name; the audit checks it against        SKILL.md
  the naming rule)
fragment when-to-split                            crates/agent-skills/fragments/when-to-split.md               3
```

Existing, kept: `agent@knowledge-architect-code-claims-reviewer`, widened at slice 2;
`skill@klarch-development`, cut at slice 2 to what is particular to this repository; the design
skill's thread state `presumed-settled`, used by #search-timing.

The issue entries slice 2 opens take their slugs at that slice, by
`skill@knowledge-architect-issue-tracking`.

## Decided design

Each slice's spec holds the decided design of the threads it builds. One shape crosses slices:
**the mutation command that the development skill names is the one slice 1 builds**, so slice 2
writes `cargo x mutate run` and `restore` as the maintenance crate exposes them once slice 1 has
merged, and the setup skill's maintenance-crate snippet, which slice 1 extends, is what a target
project copies. Every Rust target project has a maintenance crate, per #maintenance-crate-ask.

## Mapping tables

None at the milestone's level. `spec@development-and-components@development-skill` holds the table
of the review axes the development skill dispatches.

## Losing alternatives

| alternative | lost to | the fact that decided it |
| --- | --- | --- |
| a performance review axis, the owner's | #performance-opt-in, and a `todo` issue for improving performance | withdrawn on the owner's R2: improvement "should not happen on a review", and non-regression needs "a benchmark procedure"; a performance claim without a measurement cannot be reproduced, so a reviewer would drop its own findings under `skill@knowledge-architect-review@review-invariants` (`argument@development-and-components@arg-9`, `argument@development-and-components@arg-17`, `argument@development-and-components@arg-25`) |
| a general "overall quality" axis | #code-shape-axis | it has no standard to judge against and returns taste findings (`argument@development-and-components@arg-3`, `argument@development-and-components@arg-11`) |
| a security axis | the adversarial axis and a lint gate | the adversarial brief already attacks accepted inputs, and `unsafe` is a lint gate, not a reviewer's job (`argument@development-and-components@arg-11`) |
| a separate documentation axis | #code-claims-scope with #api-doc-completeness | it would duplicate the code-claims agent's method; presence is a lint (`argument@development-and-components@arg-8`, `argument@development-and-components@arg-24`) |
| a separate test economy axis | #test-economy in the code-shape reviewer | it costs one more dispatch to read the same diff (`argument@development-and-components@arg-4`) |
| the manual worktree mutation procedure kept in the installed text | #mutation-command | it fails `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` (`argument@development-and-components@arg-7`) |
| thaum's `sweep` shipped | #mutation-command, `run` and `restore` only | it is tied to thaum's invariant-check directory and its `InvariantViolation` pattern; a recorded list waits for a second real use (`argument@development-and-components@arg-37`, `argument@development-and-components@arg-39`) |
| the checker's binary as the mutation command's host | #mutation-command-host | `rejected@gates@gates-command-of-the-checker`: "It would make the document checker run cargo, the linter and the tests" (`argument@development-and-components@arg-57`) |
| the gates library as the host, or as the dependency the maintenance crate reaches the mutation crate through | #mutation-command-host | the gates library runs a project's merge gates, and a mutation is no gate: through it, every gates user would build the mutation crate (`argument@development-and-components@arg-77`); the owner's R6 moved the default from the gates library to a crate of its own (`argument@development-and-components@arg-71`) |
| the second reading of the new clause: the evidence from use is itself the measure where it can be gathered | #goal-standard-measure, first reading | it would reverse part of rule 6, whose premise is the owner's argument that a measure on use has limited feasibility; the owner chose the first reading, R5 (`argument@development-and-components@arg-45`, `argument@development-and-components@arg-50`) |
| a search before the challenge and another after it | #search-timing | recorded in `design@agent-skills@search-after-first-challenge`: it doubles the cost of the search; the synthesis keeps one search |
| general-purpose reviewers briefed with the standard, for the code-shape and spec-first axes | installed agents | `rejected@agent-skills@one-generic-auditor-agent`: "drafts came out uneven where the brief was the only standard" (`argument@development-and-components@arg-57`) |
| planning as the place of the component instructions | #component-skill-delivery | whether a Component exists is a decision about how the project is built, which `primer@design-heads` places in design; planning only cuts work already decided (`argument@development-and-components@arg-48`) |
| handling `issue@agent-skills@heads-argued-from-another-harness` in this milestone | #harness-argued-heads-scope | the work adds a fragment and edits one, and changes no delivery of fragments; the owner ruled Q4, R6 (`argument@development-and-components@arg-67`, `argument@development-and-components@arg-74`) |
| a `test-utils` feature, or a dev-dependency back onto the testing crate | #shared-test-component | it builds the crate under test twice, and its types then do not match across the two copies, from Cargo's behaviour as the agent knew it, not re-checked (`argument@development-and-components@arg-47`); slice 3's audit checks it |
| always one crate per Component | #component-crate-mapping | a published crate can depend only on published crates, and Cargo rejects cyclic package dependencies, from Cargo's documentation, not re-checked (`argument@development-and-components@arg-33`); slice 3's audit checks both |

## Readings

None. The work implements no external specification. The external tools' behaviours it rests on,
Cargo's, cargo-mutants' and crates.io's, are premises of the slices that state them, each checked at
that slice's audit.

## Premortem

The causes the agent named in the reply to R6, assuming the work shipped and failed. Each is judged
by the tripwire standard of `skill@knowledge-architect-planning@acceptance-criteria` at the harvest
of the slice named, not ruled by the owner now.

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| 1 | Sessions act on shape findings by splitting code that gained nothing from it. Each procedure counts it as success; the evidence would arrive through #retrospective-code-rules-question | #code-shape-axis | named as a cause for slice 2's harvest to judge as a tripwire; condition 2 is in doubt, since the evidence arises in target projects, per `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` |
| 2 | Sessions call logic tests "golden" or "integration" tests to skip mutation evidence | #test-economy | named as a cause for slice 2's harvest to judge, with the same doubt on condition 2 |
| 3 | Shape findings arrive only before the merge, after later work was built on the code, so they cost rework. The counts of `issue@agent-skills@the-retrospective-counts-no-review-cost` would show it | #review-cadence | named as a cause for slice 2's harvest to judge |
| 4 | A long discussion keeps many threads `presumed-settled`, and a late search reopens several of them. The cost is visible in the discussion itself, so it is probably not silent | #search-timing | named as a cause for slice 4's harvest to judge; the discussion judged condition 1 probably unmet |
| 5 | The "mechanism" observable becomes vacuous ("a review exists"), so a goal can never be shown unmet. #rule-6-finding-weight partly guards this | #goal-standard-measure | judged at slice 0's harvest by the tripwire standard: dropped. No condition clearly excludes it, and the doubt clause of `skill@knowledge-architect-decision-recording@ruled-tripwires` lets a cause in doubt be dropped where the owner would see the failure anyway: the owner rules every goal with its boundary block before it is written, per `design@agent-skills@goal-draft-shows-reach`, so a goal whose measure is a vacuous mechanism reaches the owner's reading. The doubt is kept in `rejected@agent-skills@goal-measured-by-evidence-from-use` |
| 6 | A project of about 30 Components multiplies record clusters and search agents. The worklist's agent count measures it; `issue@agent-skills@a-lighter-pass-for-untouched-components` already holds this | #component-crate-mapping | named as a cause for slice 3's harvest to judge; an entry already holds it, so the harvest folds it there or drops it |
| 7 | A change touches a declared path and records no figures. A script over the history could compute this, so it is a check to write later, not a tripwire | #performance-opt-in | named as a cause for slice 2's harvest to judge; the discussion judged it a check, which condition 1 sends to a script |
| 8 | A run killed mid-mutation loses uncommitted content | #mutation-command | a claim of the step that builds the tool, with its test: kill the run, then restore; in `spec@development-and-components@mutation-crate` |

## Acceptance criteria

None: each cause is a step claim (cause 8) or evidence that arrives only through retrospectives or
history, per `argument@development-and-components@arg-80`.

## Implementation sequence

0. The goal standard and the reworded goal, landed: the new clause of rule 6, the
   `intent` tag, the goal reworded and renamed, the re-entry of the harness issue. Fails alone on
   the reviewer's tag of a property-list measure, and on the reworded goal.
1. [The mutation crate](mutation-crate.md): the published crate `knowledge-architect-mutate`, `cargo x
   mutate` here, the setup snippet, the release procedure. Fails alone on a run that does not
   restore, or a refusal that does not refuse.
2. [The development skill](development-skill.md): the installed skill, the code-shape, spec-first and
   adversarial agents, the widened code-claims reviewer, the review cadence, the lints, the retrospective's
   question, the local skill cut down. Fails alone on a missing or contradicting instruction of the
   installed skill, or an agent that does not install.
3. [The component skill](component-skill.md): the installed skill for creating or splitting a
   Component and its fragment "when to split". Fails alone on the skill or the fragment.
4. [The search timing](search-timing.md): the design skill's loop step 1. Fails alone on the step.

## Order rationale

- Slice 0 before slice 1: the plan documents' reviews and every later slice read the reworded goal
  and the amended goal standard, on the owner's R7.
- Slice 1 before slice 2: the development skill names `cargo x mutate`, which must exist and be
  measured first.
- Slice 2 before slice 3: the fragment "when to split" is hosted by the development skill, which
  slice 2 creates.
- Slice 3 before slice 4: slice 4 edits the design skill's loop step 1, and slice 3 adds a fragment
  to the design skill; each edit then meets the design skill as the previous one left it, and slice
  4 depends on no other.

## Defaults awaiting the owner

None is open. The discussion put four labelled questions to the owner, and the owner ruled each;
slice 0 put five more, Q5 to Q9, and the plan's review found two more defaults, D1 and D2, all ruled:

- **Q1, ruled R3:** taking in thaum's mutation tool, the owner's scope choice per
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. The owner asked back in R2,
  then: "In that case, I think it is quite worth it to generalize it." It bears on #mutation-command.
- **Q2, ruled R7:** a declared path is a design head now, and a manifest key only if sessions miss
  the step in real use. The default was carried in the checkpoint, and the owner's R7 "Agreed on the
  table." closed it. It bears on #performance-opt-in.
- **Q3, ruled R3:** take in the generic `run` and `restore` as a command of a published crate: "In
  that case, I think it is quite worth it to generalize it." It bears on #mutation-command.
- **Q4, ruled R6:** "Q4: right, we are not changing how those are being delivered, there is no point
  handling it, change the re entry." It bears on #harness-argued-heads-scope.

- **Q5, ruled after the plan's review:** slice 0's design audit reused the record search the
  discussion ran, instead of dispatching a new one, a departure from point 2 of
  `skill@knowledge-architect-planning@working-a-slice` for this slice alone. The owner: "Q5 there is
  no need for a further design audit of slice 0. The design was very small, and most of the work was
  goals." It bears on #goal-standard-measure and #rule-6-finding-weight. Every later slice's audit
  dispatches its search as the skill says.
- **Q6, ruled during slice 0:** where the review reports a departure from the default shape in code a
  change writes, the writer restructures it or argues it at the code; in an existing unit the change
  edits, only the lines the change writes are weighed. Left to the development skill, not the goal.
  It bears on #code-shape-axis.
- **Q7, ruled during slice 0:** no goal states the content of the default shape; the development
  skill does. It bears on #code-quality-goal-wording.
- **Q8, ruled during slice 0:** a lint the setup ships is an installed rule about code, which the
  retrospective's question covers. It bears on #retrospective-code-rules-question and
  #api-doc-completeness.
- **Q9, ruled during slice 0:** a Component takes decisions of its own and carries a design of them,
  stated by the component skill, not the goal. It bears on #component-crate-mapping and
  #pure-output-layer.

- **D1, ruled after the plan's review:** whether cargo-mutants is an ask of every target project.
  `goal@knowledge-architect@serves-one-kind-of-project` and
  `goal@knowledge-architect@setup-brings-quality-tools` make a quality tool that the project takes
  in an ask, which the setup gives in its recommended shape. Default: cargo-mutants is not taken in
  as a quality tool and is no ask; the development skill names the diff sweep as a step a project
  may run, with no setup shape. Ruled: "D1 agreed." It bears on #diff-mutation-sweep.
- **D2, ruled after the plan's review, against the default:** what the development skill tells a Rust project that has no
  maintenance crate, which the setup skill allows with its local install. Default: the skill names
  the command as `cargo x mutate` where the project has a maintenance crate, and otherwise as the
  mutation crate's own binary installed into a directory of the project with
  `cargo install --locked --root`, as `design@agent-skills@pinned-binary-runs-inside-the-project`
  does for the checker; slice 1 therefore gives the crate a binary as well as a library. The owner
  ruled otherwise, by a new ask: "D2 the xtask maintenance crate with the shape of `cargo x
  <command>` should become a required ask from this project to consumer Rust projects." So no Rust
  target project lacks a maintenance crate, the default's fallback serves no state that follows the
  asks, and the crate holds a library alone; the ask is #maintenance-crate-ask. It bears on
  #mutation-command and #mutation-command-host.

The owner's words on Q6 to Q9, verbatim: "Agreed on all three defaults here. The current wording
factors them in, right ? If yes, we can go with those.", then "Right, I wanted to write all four
defaults".

The other defaults the discussion stated were ruled with their threads, as each thread item says.

## Harvest

Each slice's row is in its spec. This document's own row: it leaves in the commit that deletes
`spec@development-and-components@search-timing`, the last slice's spec, once each later consequence
below is in an issue entry and every premortem cause has its verdict from the harvest named in the
premortem's table.

## Later consequences

- **Improving performance as an activity of its own**, with the benchmark procedure: a `todo` issue
  slice 2 opens, per #performance-review-axis.
- **A target project collects feedback from real use** where a goal is judged in use: a `todo` issue
  slice 2 opens, per #use-feedback-collection; the setup skill would be the one to ask it.
- **Co-development of a shipped tool through a Cargo `[patch]`**: a `deferred` issue slice 2 opens,
  per #upstream-co-development, with its unverified premise that a patched build's version must match
  the consumer's `checker-version` pin.
- **Branch work and releasing**, the other two activities of
  `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`, which
  stays open for them after slice 2.
