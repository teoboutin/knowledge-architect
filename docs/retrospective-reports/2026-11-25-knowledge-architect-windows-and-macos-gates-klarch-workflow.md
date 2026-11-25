# 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow

- **Received file**: 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow.md, the workflow file of a retrospective of this repository, on the branch that ported the gates to Windows and macOS.
- **Version used**: "Version: main at `9f1efd7b`, which `git describe --tags` gives as `v0.10.0-259-g9f1efd7b`, on branch `main`, the tree clean at the start. The project builds the checker from its own source."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@one-session-cost-weighed-before-removal`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `design@agent-skills@shipped-text-cites-no-entry`: constrains, respect.
  - `design@agent-skills@thread-slug-is-entry-id`: constrains, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `design@agent-skills@discussion-record-apart`: constrains, respect.
  - `design@agent-skills@instruction-record-is-minimal`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `design@check@plan-document-sections-by-kind`: constrains, respect.
  - `design@knowledge-architect@changelog-entries`: constrains, respect.
  - `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@model@plan-items-by-section`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `design@agent-skills@scoped-auditor-on-touched-heads`: constrains, respect.
  - `design@agent-skills@pre-harvest-conformance`: constrains, respect.
  - `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`: touches, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@agent-skills@admission-rules-home`: constrains, respect.
  - `design@model@plan-document-directory`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `rejected@agent-skills@checked-commit-form-by-subject`: constrains, respect.
  - `design@check@model-then-checks`: constrains, respect.
  - `design@check@finding-names-the-repair`: constrains, respect.
  - `design@check@a-commit-message-is-a-document`: constrains, respect.
  - `issue@check@a-message-with-no-blank-line-after-its-subject-passes`: touches, respect.
  - `design@knowledge-architect@versioning-policy`: constrains, respect.
  - `issue@knowledge-architect@the-changelog-holds-references-no-check-reads`: touches, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@cli@the-commands-are-a-library-module`: constrains, respect.
  - `design@cli@command-parsing-home`: constrains, respect.
  - `design@cli@arguments-parse-through-clap`: constrains, respect.
  - `design@cli@exit-code-ladder`: constrains, respect.
  - `design@cli@a-refusal-by-arguments-writes-nothing`: constrains, respect.
  - `rejected@cli@each-crate-parses-its-commands`: constrains, respect.
  - `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
  - `design@agent-skills@code-change-reads-its-record`: constrains, respect.
  - `issue@knowledge-architect@command-output-is-not-declared-a-contract`: touches, respect.
  - `design@model@a-failed-parse-is-loud`: constrains, respect.
  - `issue@model@references-are-read-in-markdown-and-rust-only`: touches, respect.
  - `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, respect.
  - `issue@query@show-s-answers-on-one-branch`: touches, none.
  - `design@query@query-depends-on-no-check`: constrains, respect.
  - `design@agent-skills@ruled-items-labelled`: constrains, respect.
  - `issue@agent-skills@the-labels-head-and-the-primer-differ-on-a-label-in-a-branch-s-messages`: touches, respect.
  - `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`: touches, respect.

## harvest-home-written-twice: a thread's harvest home is written in its item and in the Harvest table

Members: 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow W1.

- **a. What it says.** The planning skill asks for a thread's harvest home twice: in the thread's
  item in `deliberation.md`, "the durable home that will harvest it", and in the Harvest table of
  the `README.md`. The spec's first correction changed the table and left the items, so six threads
  named one home in `deliberation.md` and another in `README.md`. Three of the seven reviewers of
  the spec reported it separately (decision record, self-consistency, cold implementer). Cause: two
  homes for one moment. Proposal: the item drops the home and the Harvest table alone holds it; one
  instruction removed, none added. Rival named: the item keeps it and the table drops it, defeated
  since the table is the list the harvest works from.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/skills/planning/SKILL.md` is
  unchanged since the file's version (`git log 9f1efd7b..origin/main` on it lists no commit). Near
  line 229, the row "Threads" of the section table: one item per thread, holding "who proposed it
  and in which round, its final state, the arguments that moved it, the section that carries its
  shape, the durable home that will harvest it, and the owner's ruling that closed it". Near line
  241, the row "Harvest": "what lands where and when. In a spec, its rows". Near line 695, the
  slice's harvest runs "per the harvest row of the slice's spec", and near line 815 the readiness
  checks hold "every thread maps to a section and to a harvest home".
- **c. Whether it is already recorded.** No entry records the duplicate, and no head states that
  a thread's item names its harvest home: a grep of the design homes, the issues and the shipped
  text for "will harvest it" returns the planning skill's line and its installed copy alone. The verdicts are all
  `respect`. `design@agent-skills@discussion-record-apart` (`constrains`) supports the proposal:
  "every condition of a decided thread is written in the decided design of a `README.md`, and
  `deliberation.md` holds only the argument: a condition written there alone is one no search checks
  the record against." `design@check@plan-document-sections-by-kind` (`constrains`) leaves the lists
  of what each section holds to the planning skill, so the checker reads neither copy and needs no
  change. `design@agent-skills@thread-slug-is-entry-id` (`constrains`) names the harvest row as the
  place where a renamed pair of a thread and its entry is stated, which the proposal keeps.
- **d. Its kind.** A defect of the text, wrong: following both instructions produced six
  contradictory pairs and three reports. Its cause is two homes for one moment, per the shape of a
  repair in `skill@knowledge-architect-retrospective-intake@admission-rules`: the two instructions
  order two writes, the thread's item in `deliberation.md` and the row in `README.md`, of one fact,
  so it is no duplicate of one act under the need test, and the repair merges the homes into one.
  The home that stays is the Harvest table: the row "Harvest" says "what lands where", and the
  readiness check near line 815, "every thread maps to a section and to a harvest home", keeps the
  mapping of every thread to a home a judged requirement once the item no longer writes it.
- **e. Its scope.** The installed text: the planning skill, edited at its source in this
  repository. No goal of the project's.
- **f. A better fix.** The nearest rival is the file's own: the item keeps the home and the table
  drops it. It loses on two facts: the harvest works from the harvest row (near line 695), and a
  home written in `deliberation.md` alone is read by no search, per
  `design@agent-skills@discussion-record-apart`. The removal is weighed against the other occasions
  of the clause, per `design@agent-skills@one-session-cost-weighed-before-removal`: a reader of a
  thread's item who wants its home finds it in the Harvest table, whose rows named the threads in the port's spec
  (`git show 3fb1d582^` on its `README.md`), and
  the readiness check near line 815 still has every thread mapped to a harvest home, so a review
  still judges the mapping. No rival keeps both copies consistent at a lower cost.
- **g. Its route.** A text edit of the installed text at its source, as
  `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says, under
  `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** In `path@agent-skills@content/skills/planning/SKILL.md`, the row "Threads"
  of the section table, near line 229, delete the clause ", the durable home that will harvest it".
  The row then reads "... the arguments that moved it, the section that carries its shape, and the
  owner's ruling that closed it ...". Instructions: one removed, none added. Then
  `cargo klarch install-agent-skills`, and the installed copy committed with it. No design head is
  written, per `design@agent-skills@instruction-record-is-minimal`. The changelog entry is judged
  by `design@knowledge-architect@changelog-entries` in the commit: the default is a Workflow entry,
  surface agent-skills, a patch, since every thread item an agent writes loses one field, which a
  person reading the plan sees; its rival, a clarification that owes none, rests on the Harvest
  table holding the same fact.

Default: handle now: delete the clause from the row "Threads" of the planning skill; one instruction removed.
Outcome: decided by the session: handle now, the clause ", the durable home that will harvest it" deleted from the row "Threads" of `path@agent-skills@content/skills/planning/SKILL.md`, one instruction removed, none added. Ground: two homes for one moment are merged into one, per the shape of a repair in `skill@knowledge-architect-retrospective-intake@admission-rules`; the Harvest table is the list the harvest works from, the readiness check that every thread maps to a harvest home stays, and `design@agent-skills@discussion-record-apart` keeps every condition of a decided thread out of `deliberation.md` alone.

## conforming-step-rests-on-drafts: a conforming step was written from an auditor's drafts that no tree holds

Members: 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow W2.

- **a. What it says.** The planning skill says "Each head a verified draft finds non-conformant
  gets a step that brings it to the rules", and the drafts stay in the scratch space. The session
  wrote the step as built from the design-record audit's draft, with a claim that reads the drafts'
  list of citing sites. Four reviewers of the spec (routing, decision record, self-consistency,
  cold implementer) found that a session that did not see the discussion can neither build the step
  nor judge the claim. It was repaired by listing the sites in the step. Proposal: the sentence says
  that the step states each edit and the sites it re-points, since the drafts leave; one instruction
  added, none removed.
- **b. Whether it still applies.** The sentence is unchanged. In
  `path@agent-skills@content/skills/planning/SKILL.md`, near line 465, at 9f1efd7b and on main:
  "Each head a verified draft finds non-conformant gets a step that brings it to the rules of"
  `primer@design-heads` and `skill@knowledge-architect-decision-recording@head-shape`, "before the
  harvest". Near line 456 the drafter's
  brief names "a draft file named by each head's slug, and a scratch directory of its own", so the
  drafts never enter the tree. The plan review the same skill dispatches already asks the cold
  implementer, near line 759, to report "what it would have to reconstruct from a conversation it did
  not see", which is how the defect was found, before any code.
- **c. Whether it is already recorded.** No entry records it. The verdicts are all `respect`.
  `design@agent-skills@pre-harvest-conformance` (`constrains`) decides that the conforming step
  exists and is sized by the plan: "Listed, their conformance is a step the plan sizes and the
  review reads apart from the harvest." `design@agent-skills@scoped-auditor-on-touched-heads`
  (`constrains`) keeps the drafter a dispatch on the list alone, whose drafts the session verifies.
  `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`
  (`touches`): its re-entry point is the planning of the next plan document that writes its list,
  which an intake is not. `design@agent-skills@behaviour-additions-need-real-use` (`constrains`)
  admits this as real use, a session's behaviour seen once.
- **d. Its kind.** A defect of the text, missing, in the file's reading; in this analysis a gap of
  conformance: the session had the move, writing each edit into the step, and did not make it. The
  plan document's general standard, that a cold implementer can act on it, exists and caught it at
  the plan review.
- **e. Its scope.** The installed text: the planning skill.
- **f. A better fix.** The nearest rival to the file's +1 is no change. The fact that decides:
  the admission standard's weight of a gap, in `skill@knowledge-architect-retrospective-intake@admission-rules`,
  "A conformance rule, for a move the agent has and might not make, is worth text only where the
  default behaviour is systematically wrong." One instance shows no system, and the plan review the
  skill already dispatches found it before the code, at the cost of one repair round of the spec. A
  second rival, a readiness check that no step rests on a file the plan's directory does not hold,
  would cover the drafts, the scout's report and the verdict files at once, and is wider than the
  evidence. Should a later plan review report a step resting on a scratch file again, that recurrence
  is the evidence the +1 clause, at the conforming step, would need.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, a gap of conformance seen once. Ground: a conformance rule earns text only where the default behaviour is systematically wrong, per `skill@knowledge-architect-retrospective-intake@admission-rules`, and the cold-implementer review of `path@agent-skills@content/skills/planning/SKILL.md` caught the one instance before any code.

## commits-passes-no-blank-line: `klarch commits` passes a message with no blank line after its subject

Members: 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow C1.

- **a. What it says.** Four messages of one agent had no blank line after the subject, so git read
  the subject and the first paragraph as one subject; `commits` passed, and the agent found it by
  reading. The session opened the issue below.
- **c. Whether it is already recorded.** Yes: `issue@check@a-message-with-no-blank-line-after-its-subject-passes`
  (`touches`, `respect`), on main since 346a4a5e, "[knowledge-architect] The issues of what the port
  of the gates leaves outside its work, and the instances of the standing ones" (`git log` with
  `--diff-filter=A` on its file). Its "What would close it": "A decision on whether `commits` reports
  it, with its argument, ... and the check; or the reason that a message's shape is no concern of the
  checker." `rejected@agent-skills@checked-commit-form-by-subject` (`constrains`) bounds any check: it
  checks the message's shape, never that a commit with a cited subject exists.
  `issue@check@history-rs-is-past-one-read`, which the search graded `touches` against the cluster
  `past-one-read-trigger-met-by-any-edit`, is not met: its re-entry is the next work that changes
  the code of `history.rs`, and no outcome here changes it.
- **h. The proposal.** No change: the issue is this finding, opened by the session that reports it.

Default: no change, already recorded.
Outcome: decided by the session: no change, already recorded. Ground: `issue@check@a-message-with-no-blank-line-after-its-subject-passes` is on main and holds this finding and its close.

## show-misses-unread-files: `show` lists no citation in the files the checker does not read

Members: 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow C3.

- **a. What it says.** `cargo klarch show` is the list the planning skill takes the citing texts
  from. Four reviewers grepped and found the spec's table incomplete: the CI workflow at two heads
  and the toolchain file at one, text whose claim the split made false. The planning skill asks for
  the grep too, so the session missed it. Proposal: `show` prints, after its list, the number of
  tracked files it does not read that hold the entry's slug, or their names.
- **b. Whether it still applies.** Yes. `LIVE_SUFFIXES` in `path@model@src/walk.rs`, near line
  137, is `["md", "rs"]`, so `path@knowledge-architect@.github/workflows/ci.yml` and
  `path@knowledge-architect@rust-toolchain.toml` are not read; both hold design references in their
  comments on main (`grep -n 'design@'` on each: lines 4, 5, 6, 8, 9, 22, 44, 50, 53, 89 and 103 of
  the first, lines 3 and 5 of the second). The planning skill, near line 227, asks for every text referencing a
  reversed or rewritten decision, "those `show` lists" (written there with the install placeholder)
  "and those a grep of its wording returns".
- **c. Whether it is already recorded.** Yes, as the gap's cause:
  `issue@model@references-are-read-in-markdown-and-rust-only` (`touches`, `respect`), "A reference
  written in a comment of any other language is never read: it is not checked, `show` does not list
  it". Its close is the walk reading at least one further language chosen by the manifest, or the
  owner's ruling that the checker stays limited to Rust source. The file's shape, a count or a list
  of unread files holding the slug, is not among the shapes it names. `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`
  (`touches`) holds the neighbouring list of citing texts, and `issue@query@show-s-answers-on-one-branch`
  (`touches`, `none`) groups the changes of `show`'s output. The act verdict on
  `goal@tasks-a-tool-does-better-are-the-tool-s` carries the owner's question of which recurring
  tasks the owner takes in, this one among them: "A task is taken in by a decision of
  knowledge-architect's owner, never because the workflow asks it." The cli heads (`constrains`)
  bound where a new command's parsing and exit codes go; the file's shape changes `show`, a command
  of `query` that `klarch` already runs, and nothing is built here, so none of them is met.
- **d. Its kind.** Two parts. The missed grep is a lapse of the session, once: the instruction
  exists and is clear. The gap in `show` is a defect of the checker's reach, recorded.
- **e. Its scope.** The checker: the `model` walk and the `query` command `show`.
- **f. A better fix.** The nearest rival to the file's addition to `show` is the recorded close,
  a walk that reads the comments of further languages. The file's shape is narrower and reaches YAML
  and TOML, which that close does not name; which shape wins, and whether the task is taken in at
  all, is a choice the recorded issue holds and the goal above gives the owner. Adding the instance
  and the shape to the issue is the move that loses nothing; building either now would take the task
  in without the owner's decision.
- **g. Its route.** An instance added to a recorded issue; nothing for the lapse.
- **h. The proposal.** Add the instance below to the "What" of
  `issue@model@references-are-read-in-markdown-and-rust-only`, as a bullet after the instance that
  the analysis of 2026-11-16-knowledge-architect-record-search-workflow-klarch-workflow adds after
  the paragraph on the installed saved workflows. That instance, a reference in a shipped workflow
  script, is a second one of the same blind spot in another branch.

```text
- A further instance, in this repository's port of the gates to Windows and macOS: the spec's list
  of the texts citing the decisions its work rewrote, taken from `show`, missed the comments of
  `path@knowledge-architect@.github/workflows/ci.yml` at two heads and of
  `path@knowledge-architect@rust-toolchain.toml` at one, whose claims the work made false; four
  reviewers found them by grep. The retrospective proposed a shape narrower than reading another
  language: `show` printing, after its list, the number or the names of the tracked files it does
  not read that hold the entry's slug, which would reach YAML and TOML files as well, per C3 of the
  retrospective file 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow.
```

Default: an instance, with the file's shape, added to the recorded issue; no change to `show` now.
Outcome: decided by the session: an instance added to `issue@model@references-are-read-in-markdown-and-rust-only`, naming the file's shape; no change for the missed grep, a lapse. Ground: the issue records the gap and leaves its shape open, and `goal@tasks-a-tool-does-better-are-the-tool-s` gives the taking in of the task to the owner.

## question-labels-reused: the session's `Q` labels were reused for different items in one record

Members: 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow, the standing answer on primer rules acted on as if absent, its first item.

- **a. What it says.** The session's questions took `Q1` and `Q2` more than once, for different
  items, across messages that all reached the plan's record.
- **b. Whether it still applies.** The rule exists, is clear, and is the same at the file's version
  and on main: `primer@labelled-items`, near line 69 of `path@agent-skills@content/PRIMER.md` at
  9f1efd7b and on main, an item under a label takes "the next number of its prefix in the record the
  label reaches, from 1, and that number is never reused there."
- **c. Whether it is already recorded.** `design@agent-skills@ruled-items-labelled` (`constrains`)
  states the same rule. `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`
  (`touches`, `respect`) records the failure: "A label number reused in a plan makes two items put to
  the owner share a name, which no check sees". `issue@agent-skills@the-labels-head-and-the-primer-differ-on-a-label-in-a-branch-s-messages`
  (`touches`) is on a label that reaches a branch's commit messages; its re-entry point, the next
  review that reports a `Q` label in a commit message, or a change of either text, is not met here.
- **d. Its kind.** A lapse of the session.
- **f. A better fix.** No text: the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules` adds none, and one instance is no
  recurrence. The instance is evidence for the recorded query issue, with one fact its shapes need:
  the labels were given in the conversation before the plan document existed, so a command over the
  plan's files lists them only once the transcript is committed.
- **g. Its route.** An instance added to a recorded issue.
- **h. The proposal.** Add the instance below to the "What" of
  `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`, as a bullet after
  its instance from rootpoint.

```text
- A further instance, in this repository's port of the gates to Windows and macOS: the session's
  questions to the owner took `Q1` and `Q2` more than once, for different items, across messages
  of the design discussion that all reached the plan's record, per the standing answers of the
  retrospective file 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow. The
  labels were given in the conversation before the plan document existed, so a command over the
  plan's files would list them only once its transcript is committed.
```

Default: an instance added to the recorded issue; no text for the lapse.
Outcome: decided by the session: an instance added to `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`; no change for the lapse. Ground: `primer@labelled-items` states the rule at the file's version and on main, and one lapse is no recurrence, per the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`.

## standing-2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow: the standing answers that report nothing to act on

Members: six answers of the section "The standing questions" of
2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow.

- **An installed file changed**: yes, the setup skill's text on the aliases, by the branch's own
  source and install. It is the branch's work, on main in 7d5838d3, "[agent-skills, testing, xtask]
  The alias `x` runs the maintenance crate in the release profile, so no gate replaces the running
  binary", and later commits of the same merge (`git log 9f1efd7b..origin/main` on the setup skill
  and its snippets).
- **A project skill's addition missed**: no; `skill@klarch-development` was loaded before the
  first change to Rust.
- **A reference written without backticks for want of a form**: none known.
- **A register entry that bore on the work and the search missed**: none found; the entries a
  reviewer raised later were in the search's returns, or were read in part by the session, which is
  W3. Nothing is added to `issue@agent-skills@a-record-search-misses-what-is-added-after-it`.
- **A script or several steps by hand**, its part on the probe branch: rebuilt and pushed by hand at
  each state of the work, which the project's spec asked for and the workflow does not. The probe
  branch as a procedure is a member of the cluster `tools-built-in-sessions`, below.
- **A rule on code, tests or Components too hard**: none of code or tests, and the code-shape
  review's repairs cost the work nothing; its part on Components and modules is W4, in the cluster
  `past-one-read-trigger-met-by-any-edit`, below.

Default: no change.
Outcome: decided by the session: no change. Ground: the six answers report no defect of the installed text or the checker, and the items they name are the branch's own work or members of clusters analysed elsewhere.

## light-verdict-demands-unanswered: a light verdict's demand can be left unanswered

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-25-knowledge-architect-intake-items-divided-klarch-workflow.md`, in its section on `light-verdict-demands-unanswered`, which gives the outcome. Member here: W3.

## past-one-read-trigger-met-by-any-edit: a past-one-read issue's re-entry is met by any edit of its file

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow.md`, in its section on `past-one-read-trigger-met-by-any-edit`, which gives the outcome. Member here: W4, which the standing answer on rules on code, tests and Components names for Components and modules.

## reviewer-reports-enter-context-whole: every reviewer's report enters the session's context whole

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-knowledge-architect-intake-forty-seven-files-klarch-workflow.md`, in its section on `reviewer-reports-enter-context-whole`, which gives the outcome. Members here: W5 and the standing answer on work the session carried. The section "Interactions" names only P1 of the project file, whose interaction with W5 is a member there too.

## thread-slug-equals-head-slug: a head still to be created whose slug its thread holds has no checked bare-slug form

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-22-knowledge-architect-owner-rulings-and-title-rules-klarch-workflow.md`, in its section on `thread-slug-equals-head-slug`, which gives the outcome. Member here: C2.

## verify-before-relying-lapses: unchecked claims and premises told to the owner or written into messages

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `verify-before-relying-lapses`, which gives the outcome. Members here: the second and fifth items of the standing answer on primer rules acted on as if absent.

## decided-section-omitted: decisions missing from a "Decided" section

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-record-search-workflow-klarch-workflow.md`, in its section on `decided-section-omitted`, which gives the outcome. Member here: the third item of the standing answer on primer rules acted on as if absent.

## attribution-slips-recur: the owner quoted or misattributed in a record

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-thaum-cast-permissions-klarch-workflow.md`, in its section on `attribution-slips-recur`, which gives the outcome. Member here: the fourth item of the standing answer on primer rules acted on as if absent.

## reword-and-fold-have-no-command: rewording and folding are hand procedures

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `reword-and-fold-have-no-command`, which gives the outcome. Member here: the rewording part of the standing answer on a script or several steps by hand.

## searcher-fills-report-by-script: searchers fill their reports by scripts of their own

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-25-knowledge-architect-intake-items-divided-klarch-workflow.md`, in its section on `searcher-fills-report-by-script`, which gives the outcome. Member here: the searchers' part of the standing answer on a script or several steps by hand.

## agent-returned-unfinished: agents that returned unfinished or with a gap

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `agent-returned-unfinished`, which gives the outcome. Member here: the standing answer on an agent that returned without finishing.

## conclusions-wrong-after-verification: returned conclusions refuted by a later verification

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `conclusions-wrong-after-verification`, which gives the outcome. Member here: the standing answer on a returned conclusion that proved wrong.

## review-cost-answers: the counts of each branch's review

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `review-cost-answers`, which gives the outcome. Member here: the standing answer on the merged branch.

## tools-built-in-sessions: tools the sessions built, and whether their generic part ships

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `tools-built-in-sessions`, which gives the outcome. Member here: the standing answer on a tool or a procedure set up or changed.
