# A tripwire is written only where its failure is silent and its evidence is read, it leaves at a horizon, and the standard reaches every text that applies it by a build-filled fragment

## Status and audience

This spec is the plan of the work that rewrites the tripwire standard of the installed workflow.
The work does six things:

- it states what earns a tripwire: four conditions, a rule on slips, and a default that keeps a
  doubtful cause with a short horizon;
- it gives every tripwire an end, a fourth line `**Watches until:**`, which the standing-state
  reviewer reads at every merge as it reads the firing clause;
- it moves the choice of which causes become tripwires from the owner to the session, which records
  or drops each cause at the end of the work that implements its decision and lists every cause
  with its outcome; the owner overrules by reviewing;
- it adds a build pass, `{{fragment:<name>}}`, that copies one file of shared text into every
  installed text that needs it, and delivers the standard that way;
- it adds an axis to the project audit, the tripwire axis, which brings every standing tripwire to
  the standard, and runs its first pass on this repository;
- it keeps the owner's word on acceptance criteria, on a ground of their own, and holds a criterion
  to the two conditions of the standard that its defects break.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list
  as reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says
  what waits: the default is built as written, and the branch merges once the owner has ruled.
- **One branch and one pull request**, the spec's first commit included, on the owner's word,
  round 5: "this should take the shape of a single spec with one PR." No commit of the work changes
  what `cargo klarch commits` judges in a commit's tree: the checker compares no installed copy
  under `commits`, per `design@core@installed-entities-from-the-tree`, and no checker code changes.

**The discussion.** It ran in one session, which begins at the owner's message opening "Right now,
tripwires accumulate indefinetely in projects following this workflow." This spec is assembled from
that session's transcript. A **round** here is one owner message and everything up to the next:
round 1 is that opening message, and the owner's messages of rounds 2 to 5 open "tripwire-admission-test:
i think this is good", "tripwire-recording-authority: accepted", "I think the standard is good" and
"recording T1 no longer needs my call". The session's own headings, "Round 1" to "Round 5", are
numbered differently, and this spec does not use them.

The measurements the discussion rests on were taken by read-only subagents in that session, over
this repository at commit 6da4781 and over thaum, a project that installs the workflow, at its
checkout of 2026-10-24. Each figure below names its source and which parts are a subagent's
judgement.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, its last paragraph, for the work of a spec.
Point 2, the design audit, is skipped when the work starts in the session that converged and no
commit other than this spec's has landed on main since; otherwise it runs.

## Names

| name | what it is |
| --- | --- |
| the standard | the text of `planned@agent-skills@fragments/tripwire-standard.md`: what earns a tripwire and the entry's shape |
| the lifecycle | the text of `planned@agent-skills@fragments/tripwire-lifecycle.md`: when a tripwire leaves its home |
| condition 1 to 4 | the four numbered conditions of the standard, in "Decided design" |
| a fragment | a file under `planned@agent-skills@fragments/`, copied by the build into every content file that names it |
| a host | a content file that names a fragment by a placeholder line |
| the horizon | the `**Watches until:**` line of a tripwire entry |
| silent | the property of condition 1: every procedure that meets the failure counts it as success |
| the census | the subagent's application of the admission test to the 85 premortem causes of this repository's deleted plan documents and in-change premortems, in two versions, v1 and v2 |
| the counterfactual | the subagents' reading of 12 firings of this repository and 37 of thaum, for whether each fact would have reached a session that reopens the decision without the tripwire |
| the surveys | the two subagents' readings of the tripwire registers of this repository (64 standing) and of thaum (130 standing), with their history |
| the tripwire axis | the third axis of `skill@knowledge-architect-project-audit`, built by this work |
| the tripwire auditor | the agent `knowledge-architect-tripwire-auditor`, built by this work |
| the tripwire audit | the saved workflow `knowledge-architect-tripwire-audit`, built by this work |
| build.rs | `path@agent-skills@build.rs` |
| render.rs | `path@agent-skills@src/render.rs`, the passes build.rs includes |
| the installed copies | the files `cargo klarch install-agent-skills` writes under .claude, committed in this repository |

## What the work is

**Today.**

- **The entry shape** is `skill@knowledge-architect-issue-tracking@tripwire-entry`: a level-two
  heading `## Guarding <decision>: <title>` with `**Fires when:**`, `**Response:**` and
  `**Re-entry:**`. Its only bar on the firing evidence is falsifiability, "specific enough that
  both parties would agree it fired". "If a script could check the firing evidence, it is not a
  tripwire" is stated there; step 7 of `skill@knowledge-architect-design@discussion-loop` restates
  only the falsifiability bar, in one sentence.
- **The exits** are `skill@knowledge-architect-issue-tracking@movement-instruction`: fired,
  reversed, absorbed, and retired when the instructions prevent the artifact the evidence names. A
  tripwire guarding a standing guarantee is restated when it fires and stays.
- **Who decides** is `skill@knowledge-architect-decision-recording@ruled-tripwires`: "A tripwire is
  written only on the owner's word". The premortem of step 7 puts each surviving cause to the owner
  under a label `T<n>`, and each acceptance criterion under `AC<n>`.
- **The standing-state reviewer**, `agent@knowledge-architect-standing-state-reviewer`, reads every
  tripwire at every merge and asks "has it fired?".
- **The decision-record reviewer**, `agent@knowledge-architect-decision-record-reviewer`, asks
  "Does a tripwire stand on the owner's word?" of each tripwire a diff adds, at its line 106 of the
  shipped source.
- **The build** applies to each file of content/, in order: the workflow-name check on the raw text
  of a saved workflow, the `%%` comment strip, the `{{slug:…}}` rendering, the `SUBSTITUTIONS` pass,
  and the `{{snippet:…}}` pass, at build.rs lines 101 to 120. An unused snippet and an unused
  substitution each fail the build. Snippets are inlined last and unprocessed, and are listed as
  `SNIPPETS` for `klarch init --rust` to write into a new project.
- **The project audit** has two axes, `skill@knowledge-architect-project-audit@design-record-axis`
  and `skill@knowledge-architect-project-audit@agentic-workflow-axis`. Each has a section of the
  skill, an agent of its own and a saved workflow under content/workflows/.

**What the surveys measured**, over both registers since 2026-08-13: 250 tripwires added, 194
standing, 0 retired, 1 of 194 stating when it stops being needed. The counts are git history read by
script; the class columns are a subagent's judgement. The full table is under `argument@tripwire-standard@arg-5`.

**Outside the work**, each with what owns it:

- **The retrospective watching session behaviour against the tripwires**, the owner's plan in
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`. This work
  takes from that issue only the rule that a re-entry holds the firing evidence, as condition 2.
  The issue is rewritten to the rest, under "Harvest".
- **Migrating other standards to fragments.** The owner, round 3: "others will be migrated later."
  `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it` is
  rewritten as the `todo` that carries it.
- **The issue half of a "standing state as a whole" audit**, in
  `issue@agent-skills@audit-axes-beyond-the-design-record`. The tripwire axis reads tripwires only.
- **thaum's first pass of the tripwire axis.** It runs at thaum's next pin move, under the
  Migration entry this work writes, per `design@agent-skills@audit-triggers`.
- **A checker rule on the horizon line**, ruled out by #tripwire-shape-unchecked.

## What is already decided

**Decisions the design rests on and does not argue again.**

- `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@agents-work-without-drift`,
  `goal@agent-skills@installed-text-leaves-room-to-judge`, `goal@agent-skills@no-contradiction-in-workflow`,
  `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@installed-text-works-anywhere`:
  the criteria of the discussion derive from them.
- `skill@knowledge-architect-issue-tracking@trigger-test`: an exit occasion is one whose own work
  already reads the entry.
- `design@agent-skills@audit-triggers`, `design@agent-skills@audit-is-an-activity`,
  `design@agent-skills@audit-agent-per-axis`, `design@agent-skills@audit-axis-runs-a-workflow`,
  `design@agent-skills@audit-method`, `design@agent-skills@one-run-pass`,
  `design@agent-skills@committed-audit-report`, `design@agent-skills@apply-stays-with-the-session`,
  `design@agent-skills@verifier-in-the-run`: the tripwire axis takes the shape every axis takes.
- `design@agent-skills@audit-outcomes`: a removal of an entry goes to the owner. The tripwire axis's
  removals do, grouped, per #removals-grouped-by-condition.
- `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`:
  a new rule on recorded content owes a Migration entry that cites the axis bringing content to it.
- `design@core@installed-entities-from-the-tree`: `commits` compares no installed copy, so the
  work's commits need no ordering for the per-commit gate.
- `design@core@tripwire-guards-no-rejected-entry`: unchanged; the horizon re-test of a `rejected`
  entry reads the entry's own text, not the `guarding` column.
- `rejected@core@validity-marker-checked`: no field of a heading entry is read by the checker;
  #tripwire-shape-unchecked keeps it so.
- `design@agent-skills@shipped-text-cites-no-entry` and `design@agent-skills@shipped-text-line-comments`:
  a fragment cites no entry of this repository, and its `%%` lines are removed like any content
  file's.
- `design@knowledge-architect@package-include-whitelist`: the crate's `include` list gains the
  fragments directory.

**Decisions this work reverses or rewrites**, each with every text that cites it, as
`cargo klarch show` lists them at commit 6da4781, and the step or harvest that updates each.

| decision | change | citing texts, and who updates them |
| --- | --- | --- |
| `design@agent-skills@premortem-tripwires-on-the-owners-word` | reversed: moves to `rejected` as `owner-rules-each-tripwire` | the `%%` at content/skills/decision-recording/SKILL.md:251 (step 2); design.md:1031, 1608, 1629, the bodies of the three heads below (harvest) |
| `design@agent-skills@premortem-tripwire-written-at-harvest` | rewritten: a cause is judged and written at the end of the work, by the session | the `%%` at decision-recording/SKILL.md:252 (step 2); design.md:1020 (harvest) |
| `design@agent-skills@tripwire-wording-is-the-agents` | absorbed by the head of #tripwire-recording-authority, and deleted | design.md:1631 (harvest); `issue@agent-skills@eight-tripwires-respond-with-an-action-the-tripwire-shape-does-not-name` lines 34 and 46, which closes at step 5 |
| `design@agent-skills@acceptance-criteria-on-the-owners-word` | rewritten on its own ground, per #acceptance-criterion-authority | the `%%` at content/skills/design/SKILL.md:595 and planning/SKILL.md:328 (step 2); `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` line 48 (harvest re-reads it: the head keeps its rule on a criterion proposed after the premortem) |
| `design@agent-skills@acceptance-criteria-in-the-document` | rewritten: a recurring criterion becomes a tripwire by the standard, not on the owner's word | nothing cites it |
| `design@agent-skills@restatement-size-test` | rewritten: a fragment is the main delivery of a standard read whole at several places | design.md:585, 1264, 3007 (harvest re-reads each: they cite the size rule, which stands for restatements); `tripwire@agent-skills@pointer-not-followed` (step 5 judges it); `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` line 72 (harvest re-reads); the pointer issue (rewritten at harvest) |
| `design@agent-skills@ruled-items-labelled` | rewritten: the `T` row leaves the prefix table | the `%%` at decision-recording/SKILL.md:250 and design/SKILL.md:596 (step 2); the other `%%` sites, PRIMER.md:65 and 446, planning 329 and 520, project-audit 73, retrospective 69, review 163, setup 297, cite the head for other prefixes and stay; design.md:1632, 2789, 2919 (harvest re-reads) |
| `design@agent-skills@design-hands-off-to-planning` | rewritten: the design skill ends at convergence, the premortem and the owner's rulings on its acceptance criteria | design.md:1536; crates/core/docs/design.md:1001, which cites it for the plan-document section lists this work leaves unchanged (harvest re-reads both) |
| `design@agent-skills@in-change-path` | rewritten: the commit message carries every premortem cause with its outcome and the rulings on acceptance criteria | design.md:1566, 1784; `tripwire@agent-skills@ruling-lost-in-change` (step 5 judges it); the agent-behaviour issue line 20 (harvest) |
| `design@agent-skills@standing-state-review-before-every-merge` | rewritten: the reviewer also reads every horizon | design.md:1200; the agent-behaviour issue line 66; `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` line 50 (harvest re-reads all three) |
| `design@agent-skills@design-record-axis` | rewritten: "the conformance of a tripwire … to its own register's rules is the axis of that register" now names the tripwire axis | `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component` line 25 (harvest re-reads); `tripwire@agent-skills@component-judgement-noisy` (step 5) |
| `design@agent-skills@audit-is-an-activity` | rewritten only where its body counts the axes | the audit-axes issue line 9 (rewritten at harvest) |
| `design@agent-skills@knowledge-table-home` | unchanged; the primer's tripwire row it ships gains "or its horizon passes" | CLAUDE.md:47 and design.md:2185, 3080, 3383 cite the head, not the row: nothing to update |
| `design@agent-skills@content-mirrors-the-install-layout` | rewritten: the build also fills fragment placeholders, first | crates/agent-skills/CLAUDE.md:6 (step 1); crates/core/docs/design.md:2742 (harvest re-reads) |
| `rejected@agent-skills@directive-restated-everywhere` | unchanged: its recorded reason, nine inconsistencies between restatements written by hand, does not cover a copy the build fills | design.md:3058 (the rewritten `design@agent-skills@restatement-size-test` keeps citing it) |

**Standing entries the discussion's searches returned, with their outcome.**

| entry | outcome |
| --- | --- |
| `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` | closed: the rule "a slip is no firing" states the bound; its four tripwires go through step 5 |
| `issue@agent-skills@eight-tripwires-respond-with-an-action-the-tripwire-shape-does-not-name` | closed at step 5: each of the eight is reworded to the shape, or removed |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | rewritten at harvest: condition 2 is its re-entry rule; the retrospective watch stays open |
| `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it` | rewritten at harvest as a `todo`: migrate the other standards to fragments |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | rewritten at harvest: the tripwire half of "standing state as a whole" exists |
| `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` | bears on nothing here; it is cited as the evidence of `argument@tripwire-standard@arg-21` |
| `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` | bears on the census only as a question the discussion answered for itself: the census tested the owner's real rulings; the issue stays open |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`, `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt`, `issue@agent-skills@the-retrospective-counts-no-review-cost` | bear on nothing in the work: the searcher's input shrinks, its procedure does not change |
| every tripwire of this repository | judged at step 5, the first pass of the tripwire axis |

**Design and `rejected` entries graded `bears`**, beyond those above: `design@agent-skills@owner-rules-at-a-fork`
(a cost alone makes no fork, which #tripwire-recording-authority rests on);
`design@agent-skills@questions-go-to-their-authority` (the same); `design@agent-skills@losing-alternatives-filter`
(the horizon re-test of a `rejected` entry kept on a doubt applies its tests); `design@agent-skills@goal-lifecycle`
(answered in `argument@tripwire-standard@arg-14`); `rejected@agent-skills@delegated-decision-state` (not brought back: `argument@tripwire-standard@arg-18`);
`rejected@agent-skills@audit-without-calibration-or-recheck` (not brought back: the axis keeps its
verify step and its re-check).

## Criteria

### The owner rules only on what the goals and the record leave open `##owner-rules-only-what-is-open`

Binding, from `goal@knowledge-architect@agents-work-without-drift`. Met by #tripwire-recording-authority
and #tripwire-admission-test: once the standard ranks a cause, recording it is no fork.

### The owner is informed of every decision and can overrule it `##owner-informed-of-every-decision`

Binding, from `goal@knowledge-architect@the-owner-decides`. Met by #tripwire-recording-authority:
every cause is listed in the turn's "Decided" section and in the record of the work, with its
outcome.

### No fixed count or sequence where judgement serves `##room-to-judge-kept`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met by #tripwire-admission-test,
which names conditions and a default and no count, and by #tripwire-horizon, whose length is the
writer's judgement.

### An exit occasion is one whose own work already reads the entry `##exit-occasion-reads-the-entry`

Binding as a presumption, from `skill@knowledge-architect-issue-tracking@trigger-test`. Met by
#tripwire-horizon: the standing-state reviewer reads every entry at every merge already.

### One standard, applied alike by the writer, the reviewer and the auditor `##one-standard-everywhere`

Binding, from `goal@agent-skills@no-contradiction-in-workflow`. Met by #shared-text-fragment.

### An audit runs on an occasion, never on a period `##audit-on-an-occasion`

Binding as a presumption, from `design@agent-skills@audit-triggers`. Met by #tripwire-audit-axis:
it runs at the Migration entry and on the owner's request.

### Every standing entry earns its reading cost `##entry-earns-its-reading`

Weighed. Its checkable form: the share of standing entries with an exit beyond firing, and the
share that fail the standard. Met by #tripwire-admission-test, #tripwire-horizon and
#tripwire-audit-axis; the first pass of step 5 measures it.

## Threads

### A cause becomes a tripwire only where its failure is silent, its evidence is read, it is what the decision forbids and its numbers have a basis `##tripwire-admission-test`

Proposed by the session, round 1. Final state: `approved`. Shape: "Decided design", "The standard".
Harvest: a head in `register@agent-skills@design`, under this slug.

The first shape, three tests (delivery, seen where read, weight), was approved in round 2: "i
think this is good. we'll go with that and see whether i notice improvements." The census then
showed it drops 46 of the 62 causes the owner kept and every tripwire that later fired (`argument@tripwire-standard@arg-15`); the
session presented that as a material finding and refined the tests (v2, `argument@tripwire-standard@arg-16`), which raised the
unclear verdicts from 15 to 25 (`argument@tripwire-standard@arg-17`). The owner, round 3, added the counterfactual test: "would
the facts that made the fired tripwires fire have been found out without the tripwires in the
record ? if yes, that probably means they were not that useful anyway." The counterfactual (`argument@tripwire-standard@arg-19`,
`argument@tripwire-standard@arg-20`) reshaped the test around silence, and produced conditions 3 and 4. The owner closed it,
round 4: "I think the standard is good. even if it drops a few of the useful ones, i'd keep it,
unless a small refinement can make them pass. what passes will always carry a little bit of
"subjectivity" anyway, since not all long term conseauences may come to the mind of everyone. the
important criteria is rather that accepted tripwires are valuable, rather than no past useful
tripwire stays accepted. the future side is more important than the past here."

Arguments: `argument@tripwire-standard@arg-1`, `argument@tripwire-standard@arg-3`, `argument@tripwire-standard@arg-6` to `argument@tripwire-standard@arg-9`, `argument@tripwire-standard@arg-15` to `argument@tripwire-standard@arg-24`.

### Every tripwire names the occasions after which, unfired, it leaves `##tripwire-horizon`

Proposed by the session, round 1, answering the owner's "retirement clause". Final state:
`approved`. The owner, round 2: "this is good, this is what I meant by "retirement clause"". The
lifecycle shown in round 4, its exit "a firing's repair adds a test or a check" included, was
closed with the standard: "I think the standard is good". Shape: "Decided design", "The entry and
its horizon" and "The lifecycle". Harvest: a head in `register@agent-skills@design`, under this slug.

Arguments: `argument@tripwire-standard@arg-1`, `argument@tripwire-standard@arg-4`, `argument@tripwire-standard@arg-10` to `argument@tripwire-standard@arg-14`, `argument@tripwire-standard@arg-22`.

### The session records or drops each cause at the end of the work, and the owner overrules by review `##tripwire-recording-authority`

Proposed by the session, round 1, as shape (A), against shape (B), a filter that still puts each
admitted cause to the owner. Final state: `approved`. The owner, round 2: "Also, I think the
ruling on whether to record tripwires should be reversed: keep is the default, my word is only
needed for a tripwire to be dropped, and asked only at the end of the work that creates the
tripwire, with the wording it gets at that moment, not the wording from the premortem, which often
needs adjustements. or maybe i do not even need to be asked at all. i overrule by reviwing myself,
the plan or the work. this is your position on tripwire-recording-authority, and i agree with it."
The session stated its reading in its next turn: the session records the causes that pass and drops
those that fail, with no question to the owner, and a recorded tripwire leaves by firing, reversal,
horizon, the owner's word or an audit removal ruled by the owner. The owner, round 3:
"tripwire-recording-authority: accepted". In round 5, on T1: "recording T1 no longer needs my call
under the rworked workflow we decided, no ?", which the session confirmed.

Shape: "Decided design", "Who records". Harvest: a head under this slug in `register@agent-skills@design`,
absorbing `design@agent-skills@tripwire-wording-is-the-agents`; the reversed
`design@agent-skills@premortem-tripwires-on-the-owners-word` moves to `register@agent-skills@rejected`.

Arguments: `argument@tripwire-standard@arg-6`, `argument@tripwire-standard@arg-18`, `argument@tripwire-standard@arg-25`.

### The standard has one home, delivered whole where it is applied `##tripwire-standard-home`

Proposed by the session, round 1, as a section of the issue-tracking skill that step 7 and the
recording skill point to. Final state: `superseded` by #shared-text-fragment. The owner, round 2,
proposed the substitution: "keep the standard written once, subsitute it at install time in all the
skills and subagents that need it, so the standard cannot drift and is still delivered reliably.
the delivery through a pointer and asking to read in full is flawed, because it causes lots of
overhead for the agent: need to think about the instruction to read, find the correct method to
access, then write it, and i believe it has high chances of causing agents to ignore it with the
argument of "already in memory from earlier"." The session withdrew its pointer shape. The owner,
round 3: "tripwire-standard-home: accepted with shared-text-fragment, good thing that your proposal
is reusable. I think the location table is right."

Arguments: `argument@tripwire-standard@arg-26`, `argument@tripwire-standard@arg-27`.

### A fragment the build copies is the main delivery of a standard read whole at several places `##shared-text-fragment`

Proposed by the session, round 2, as the mechanism for the owner's substitution. Final state:
`approved`. The owner, round 3: "agreed on rewriting design@agent-skills@restatement-size-test,
fragments have to become the main delivery mechanism for standards and similar large blocks of
coherent instructions that need to be read as a whole at multiple locations. others will be
migrated later." Shape: "Decided design", "Fragments" and "The hosts". Harvest: a head under this
slug in `register@agent-skills@design`, and the rewrite of `design@agent-skills@restatement-size-test`
and `design@agent-skills@content-mirrors-the-install-layout`.

Arguments: `argument@tripwire-standard@arg-26` to `argument@tripwire-standard@arg-29`.

### An audit axis brings every standing tripwire to the standard `##tripwire-audit-axis`

Proposed by the session, round 1. Final state: `approved`. The owner, round 2: "tripwire-audit-axis:
shape looks good". Shape: "Decided design", "The tripwire axis". Harvest: a head under this slug in
`register@agent-skills@design`, and the rewrites of `design@agent-skills@design-record-axis` and
`design@agent-skills@audit-is-an-activity` where their bodies count the axes.

Arguments: `argument@tripwire-standard@arg-3`, `argument@tripwire-standard@arg-30`.

### The checker reads no line of a tripwire entry `##tripwire-shape-unchecked`

Proposed by the session, round 1, as the default against a presence check of the horizon line.
Final state: `approved`. The owner, round 2: "not much reason to check yet, we can keep this out of
the checker". Shape: nothing is built. Harvest: judged by the entry tests at harvest; the session's
reading is that it earns no head, since `rejected@core@validity-marker-checked` already records the
rival's reason, and the commit message carries it.

Arguments: `argument@tripwire-standard@arg-31`.

### An acceptance criterion keeps the owner's word, on the ground that it decides what the branch must build `##acceptance-criterion-authority`

Proposed by the session, round 2, split from #tripwire-recording-authority, since
`design@agent-skills@acceptance-criteria-on-the-owners-word` argued from the tripwire analogy that
(A) removes. Final state: `approved`. The owner, round 3: "i'd like to discuss this further. I want
to increase autonomy of agents. but i have to agree that acceptance criteria are an important point
were my ruling is useful." Round 4: "acceptance-criterion-authority and acceptance-criterion-quality
accepted, do it in the same work". Shape: "Decided design", "Acceptance criteria". Harvest: the
rewrite of `design@agent-skills@acceptance-criteria-on-the-owners-word`.

Arguments: `argument@tripwire-standard@arg-32`.

### An acceptance criterion names a built result under which it fails, and its bound carries its basis `##acceptance-criterion-quality`

Proposed by the session, round 3, from the owner's report of round 3: "The main problem i noticed
is that I think the acceptance criteria produced this way are too often of low quality, in
particular they carry numerical bounds that are not truly relevant, or they are basically
guaranteed to pass. but maybe this is outside the scope of today's work." Final state: `approved`,
in scope, round 4: "do it in the same work". Shape: "Decided design", "Acceptance criteria".
Harvest: a head under this slug in `register@agent-skills@design`.

Arguments: `argument@tripwire-standard@arg-32`, `argument@tripwire-standard@arg-33`.

## Arguments

### The register only grows, because nothing watches for a decision's success `##arg-1`

Round 1, the owner: "they only guard against failure of a design, but nearly never watch for
success. This means that on success of a design which has been observed sound and robust, nothing
will ever be observed that would retire them." Bears on #tripwire-horizon and
#tripwire-admission-test.

### The accumulation is not this repository's alone `##arg-2`

Round 1, the owner: "I believe this is also the case in thaum (../games/thaum/), a project focused
on code [...] Tripwires about things that are coded accumulate, even if the trigger conditions are
proved by experience to not matter much." Bears on #tripwire-audit-axis.

### The cost is the reading every design session and reviewer owes `##arg-3`

Round 1, the owner: "it grows the worklist of design sessions, standing state reviewers, adn
others, indefinetely." Round 3, the owner, on the cost of watching: "the volume that design sessions
need to look at, and i mentionned it because the tripwires in question were not something an agent
session would be able to notice, so they would be reading them for no gain." Bears on
#tripwire-admission-test, condition 2, and #tripwire-audit-axis.

### The owner's cases of a tripwire not worth recording `##arg-4`

Round 1, the owner: tripwires "hard to observe by an agent, while they would be obvious to me when
driving a session"; "the failure of a design that I am certain will never happen"; "one that will
probably happen but is minor (like an agent not following an instruction once or twice in a session,
which is normal, no one can perfectly follow such massive amounts of instruction, and reviews are
meant to catch that". And the aim: "make sure tripwire volume stays controlled (not a hard cap, but
making sure those added are worth it, and those that prove not worth are removed), as well as
improve agent autonomy on selecting tripwires". Bears on every tripwire thread.

### The surveys: 250 added, 194 standing, none retired `##arg-5`

Round 1, the session, relaying the two surveys. Counts read from git history by script; the class
and observability columns are a subagent's judgement.

| | knowledge-architect | thaum |
| --- | --- | --- |
| ever added | 85 | 165 |
| standing | 64 | 130 |
| removed: fired | 9 | 7 |
| removed: decision reversed | 4 | 8 |
| removed: moved out with a Component | 5 | 10 |
| removed: absorbed | 0 | 6 |
| removed: the owner's manual cleanup | 2 | 3 |
| retired | 0 | 0 |
| stating when it stops being needed | 1 | 0 |
| caught anyway by a review, a test, the checker or the owner | about 45 | 34, and 22 partly |
| firing evidence a script could check | about 7 | 28, and 23 partly |
| observable only by the owner or only in another project | 22 | 1 |

Of 67 premortem causes then counted here, the owner recorded 43 and dropped 23; the census later
recounted 85 (`argument@tripwire-standard@arg-15`). Bears on #tripwire-horizon and #tripwire-admission-test.

### The firings came early `##arg-6`

Round 1, the session: the 9 firings of this repository came 1, 0, 34, 0, 3, 3, 13, 15 and 15 days
after their entry was written; the register is 72 days old, so the data is censored. Bears on
#tripwire-horizon.

### The existing standard is not applied where tripwires are written `##arg-7`

Round 1, the session: about 35 entries across both projects break "If a script could check the
firing evidence, it is not a tripwire", which step 7 never sends the writer to read: step 7 restates
only the falsifiability bar. Bears on #tripwire-standard-home and #shared-text-fragment.

### The mechanism of the growth `##arg-8`

Round 1, the session: "every premortem cause you accept becomes an entry with no end, and the only
events that remove it, a firing or a reversal, are rare by construction, because a premortem names
unlikely failures." Bears on #tripwire-horizon.

### The first admission test: delivery, seen where read, weight `##arg-9`

Round 1, the session: a cause becomes a tripwire only when its evidence would not otherwise reach a
session that would reopen the decision; when its re-entry holds the evidence; and when the evidence
shows the decision wrong, not one execution of it. Bears on #tripwire-admission-test; superseded by
the standard of round 4.

### A horizon, not a count of occasions survived `##arg-10`

Round 1, the session: a count of occasions survived, kept in the entry, "is churn, and wrong
whenever a session forgets to update it". Bears on #tripwire-horizon.

### An age or version sunset has no evidence for its period `##arg-11`

Round 1, the session: a sunset by date or version fails `design@agent-skills@audit-triggers`, since
a period has no evidence for it, and it fires on whoever crosses the date. Bears on #tripwire-horizon.

### The standing-state reviewer is already at the exit occasion `##arg-12`

Round 1, the session: the reviewer reads every tripwire at every merge and asks "has it fired?"; it
asks "has its horizon passed?" at the same reading, so no new re-entry point is needed. Bears on
#tripwire-horizon and #exit-occasion-reads-the-entry.

### A tripwire that leaves on its horizon answers a `rejected` entry's doubt `##arg-13`

Round 1, the session: 13 `rejected` entries stay only because a doubt remains that a named tripwire
watches (the design-entry searches of round 1). When the tripwire leaves unfired, the doubt is
answered, and the entry is judged again under `design@agent-skills@losing-alternatives-filter`.
Bears on #tripwire-horizon.

### A goal is standing intent; a tripwire is a hypothesis on one premise `##arg-14`

Round 1, the session, answering `design@agent-skills@goal-lifecycle`: "a goal is your standing
intent, met continuously. A tripwire is a hypothesis about one premise. After the tripwire leaves,
the premise is still written in the decision's head, and every grounding reads it." Bears on
#tripwire-horizon.

### Census v1: the test drops 46 of 62 kept causes and every firing `##arg-15`

Round 2, the session, relaying the census, a subagent's judgement without hindsight on 83 causes
(85 found, 2 voided). Recorded by the owner: 4 record, 46 drop, 12 unclear. Dropped by the owner: 0
record, 18 drop, 3 unclear. All 9 tripwires that later fired were test drops. The owner's drops went
by plan: 15 of 21 came from 5 premortems dropped whole. Bears on #tripwire-admission-test, as a
material finding against its round 2 approval.

### The retrospective intake reads tripwires `##arg-16`

Round 2, the session, correcting its own brief on the census's finding: the installed retrospective
intake reads every tripwire against a received file, its step 3 and its section on a tripwire that
may have fired; 3 of the 9 firings arrived that way. Bears on condition 2.

### Census v2: refining the sub-cases raised the unclear verdicts `##arg-17`

Round 2, the session, relaying census v2 on refined tests: agreement 34 of 83, disagreement 24,
unclear 25, from 15. Recorded by the owner: 25 record, 18 drop, 19 unclear. Dropped by the owner: 6
record, 9 drop, 6 unclear. Of the fired: 1 recorded, 6 unclear, 2 dropped on 1a, whose firing added
nothing the request did not deliver (hindsight). Bears on #tripwire-admission-test: a test that must
decide every case grows its doubts.

### Horizons make the two errors unequal, so a doubt keeps the cause `##arg-18`

Round 2, the session: "A wrong keep costs reading until its horizon. A wrong drop loses a signal for
good." Since the unclear verdicts held 6 of the 9 firings, a cause no named fact excludes is kept
with a short horizon. Also: (A) is no grant within a boundary, so it does not bring back
`rejected@agent-skills@delegated-decision-state`; it is `primer@question-authority` applied once the
record ranks the question. Bears on #tripwire-admission-test and #tripwire-recording-authority.

### The owner's counterfactual test `##arg-19`

Round 3, the owner: "would the facts that made the fired tripwires fire have been found out without
the tripwires in the record ? if yes, that probably means they were not that useful anyway." Bears
on #tripwire-admission-test: it became condition 1.

### The counterfactual: 8 of 49 firings needed the tripwire `##arg-20`

Round 3, the session, relaying the counterfactual, a subagent's hindsight judgement per firing with
its commit. This repository, 12 firings: 8 found and acted on anyway, 1 raised to the decision by
the tripwire, 3 found only through it, 2 of those 3 false alarms. thaum, 37 firings: 17 found
anyway, 4 raised, 6 found only through it, 4 that changed only the tripwire's own record, 6 misfires
of its own wording. So 8 needed the tripwire, at most 13 if the raised ones count. Bears on
#tripwire-admission-test.

### What the useful firings share: the failure was silent `##arg-21`

Round 3, the session: in every useful firing the tests and gates passed and every review counted
the fact as success or a repaired slip; the tripwire changed what the fact meant. Here:
`decision-trigger-does-not-fire` (the commit carried a deliberation) and `member-beyond-the-argument`
(each instance repaired as a lapse, the third showing the premise false, now
`issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`). In thaum: code on
the wrong side of a boundary a head draws, a tool's coverage shrinking with no failure, a text
property no check reads; 5 of 6 found by the standing-state reviewer reading the list. Bears on
condition 1 and on the rule on slips.

### What the useless firings share `##arg-22`

Round 3, the session: the fact surfaced through someone's own job (25 of 49); the clause counted
what the decision instructs or endorses (the 3 false alarms, `AWA T4` and `scope-and-distance`);
the threshold sat inside its instrument's noise (thaum's `t-event-payload`, firings 3 to 5 only
moving its own threshold); the wording misfired (6 in thaum). Bears on conditions 1, 3 and 4, and on
the lifecycle's exit when a repair adds a check (thaum's restated standing entries on work the plan
did on purpose, 10 of 17 found-anyway firings there).

### Precision matters more than recall `##arg-23`

Round 4, the owner: "the important criteria is rather that accepted tripwires are valuable, rather
than no past useful tripwire stays accepted. the future side is more important than the past here."
Bears on #tripwire-admission-test: no census v3 is run; the first audit pass measures precision.

### Each condition's reason `##arg-24`

Round 3, the session, presenting the standard with a reason per condition: silent is the owner's
counterfactual stated as a test the writer can apply in advance; "lands where read" is the owner's
cost of watching; "not the decision's own instruction" because such a clause "can only fire on the
decision working"; "a number has a basis" because "a number invented to make a clause 'specific'
fires on noise, or never"; a slip is no firing because one lapse is the review working, and a
recurrence is kept because it is how such a premise shows false. Bears on #tripwire-admission-test.

### The owner's weighing of a cause stops being a fork once the standard ranks it `##arg-25`

Round 1, the session: `design@agent-skills@premortem-tripwires-on-the-owners-word` stands on "the
cost of watching against the risk, which no goal or decision ranks". With the standard recorded, a
decision ranks it, so per `primer@question-authority` the cause is no longer a fork. Bears on
#tripwire-recording-authority.

### A pointer is delivered only if the session fetches it `##arg-26`

Round 2, the owner (quoted under #tripwire-standard-home), and the session: `argument@tripwire-standard@arg-7` is consistent with
it, since step 7 never sent the session to the rule it broke. Bears on #tripwire-standard-home and
#shared-text-fragment.

### The snippet pass cannot carry prose `##arg-27`

Round 2, the session: a snippet is inlined last, so never stripped of `%%` lines nor filled; every
snippet is listed in `SNIPPETS`, which `klarch init --rust` writes into new projects; a snippet is
code a test checks. Bears on #shared-text-fragment.

### A harness hook ties the workflow to one harness `##arg-28`

Round 2, the session, against the owner's second lead in the pointer issue: a hook that delivers
pointed sections "ties the shipped workflow to one harness's hook system", against
`goal@agent-skills@installed-text-works-anywhere`. Bears on #shared-text-fragment.

### A build-filled copy answers the drift ground; the host table answers the size ground `##arg-29`

Round 2, the session: `design@agent-skills@restatement-size-test` stands on two grounds of the
owner's, drift and size ("avoid loading excessive unrelated informations"). A copy the build fills
cannot drift; a fragment goes only where its act applies the whole standard. And
`rejected@agent-skills@directive-restated-everywhere` stays rejected: its reason is nine
inconsistencies between restatements written by hand. Bears on #shared-text-fragment.

### The first audit removes what never deserved an entry; horizons prune after `##arg-30`

Round 1, the session: "horizons do the ongoing pruning, and the audit is only for migration and for
when you ask. That answers your worry that a retirement clause alone may not be enough." Bears on
#tripwire-audit-axis.

### The checker cannot judge a horizon `##arg-31`

Round 1, the session: whether a horizon has passed is an event the checker does not see, and a
presence check is a field on a heading register, which `rejected@core@validity-marker-checked`
already rejected for lacking a syntax. Bears on #tripwire-shape-unchecked.

### An acceptance criterion gates a merge; a tripwire changes nothing until it fires `##arg-32`

Round 2, the session: "An acceptance criterion gates the merge of work being built: applying it
decides what the branch must build. A tripwire changes nothing until it fires." The owner, round 3,
agreed that "acceptance criteria are an important point were my ruling is useful". Bears on
#acceptance-criterion-authority.

### The two defects of acceptance criteria are conditions 3 and 4 `##arg-33`

Round 3, the session: "a criterion bound to pass checks what the design instructs (condition 3); an
irrelevant bound is a number with no basis (condition 4)." Conditions 1 and 2 do not apply: a
criterion is checked once, by the landing's own work. Bears on #acceptance-criterion-quality.

## New names, in one place

```text
crates/agent-skills/fragments/                      new directory: the fragments
crates/agent-skills/fragments/tripwire-standard.md  new: the standard
crates/agent-skills/fragments/tripwire-lifecycle.md new: the lifecycle
{{fragment:<name>}}                                 new placeholder line, rendered by build.rs
FRAGMENT                                            new const in build.rs: "{{fragment:"
Fragments                                           new struct in build.rs, beside Snippets
**Watches until:**                                  new line of a tripwire entry
crates/agent-skills/content/agents/tripwire-auditor.md      new agent: knowledge-architect-tripwire-auditor
crates/agent-skills/content/workflows/tripwire-audit.js     new saved workflow: knowledge-architect-tripwire-audit
tripwire-axis                                       new section slug of the project-audit skill
docs/audit-reports/<YYYY-MM-DD>-tripwire-axis-pass-<n>.md   the report file of a pass, by the generic name
Snippets, SUBSTITUTIONS, strip_comments, render_slugs, substitute   existing, build.rs and render.rs
```

## Decided design

### The standard

The text of `planned@agent-skills@fragments/tripwire-standard.md`, as the owner approved it in round
4, with three editorial changes: the second person becomes "the session", the T-labels of the
premortem are gone, and the criterion's paragraph is added per #acceptance-criterion-quality. The
implementing session may reword it; a change of what a condition admits is a change of the
decision. An illustration of the text, not authority:

```markdown
A tripwire watches one premise of a recorded decision: a failure that would show the decision
wrong and that nothing else in the project reports. Most causes a premortem names are not that,
and are not written.

**A cause becomes a tripwire only when all four conditions hold.** The session that implements the
decision judges each cause at the end of that work, on the wording it has then. It records those
that pass, and lists every cause in its turn's "Decided" section and in the record of the work,
each dropped cause with the condition it fails. The owner overrules by reading the work.

1. **The failure is silent.** Every procedure that meets it counts it as success: the tests and
   gates pass, no work is stopped by it, no review reports it as a defect of the decision, and no
   request names it. A failure that fails a test or a check, stops the work that meets it, or
   arrives as a request for what the decision refused reaches the decision without a tripwire.
   Where a script could compute the firing, write that check instead.
2. **The evidence lands where tripwires are read.** It is in the tree, the history, an audit
   report, or a retrospective file the intake receives. Evidence that exists only in a session's
   transcript, or only in what the owner sees while driving a session, is read by no occasion that
   holds it.
3. **The evidence is what the decision says must not happen.** A clause that counts what the
   decision instructs or endorses fires on the decision working.
4. **A number has a basis.** A bound in the firing clause comes from a measurement or an argument
   the decision's head carries, and exceeds the noise of the instrument that takes it. Otherwise
   the clause names the event.

**A slip is no firing.** A clause on how sessions follow an instruction fires on the same failure
recurring in separate pieces of work, never on one instance a review repaired: one slip is the
review working. Where a clause fires on both a slip and something more, strike the slip.

**A doubt keeps the cause, briefly.** A cause that no condition clearly excludes is recorded with a
short horizon. A wrong keep costs reading until its horizon; a wrong drop loses the evidence for
good.

<the entry block of "The entry and its horizon">

**An acceptance criterion meets conditions 3 and 4**: it names a built result under which it fails,
and a bound in it carries its basis. Conditions 1 and 2 do not apply to it: the landing's own work
checks it once.
```

The argument: `argument@tripwire-standard@arg-19` to `argument@tripwire-standard@arg-24`. Nearest rival: the three tests of round 1, refined (census v2);
defeated by `argument@tripwire-standard@arg-15` (v1 drops every firing) and `argument@tripwire-standard@arg-17` (v2's sub-cases raise the unclear verdicts
from 15 to 25).

### The entry and its horizon

```markdown
## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`

**Fires when:** the event, or a count with its basis.
**Response:** reopen the decision by its reference, or open an issue of a named kind.
**Re-entry:** an occasion that holds the evidence.
**Watches until:** the occasions after which the cause, unfired, no longer will.
```

`**Watches until:**` names the first occasions that exercise the decision: a number of reviews of
changes touching it, of audit passes, or of releases, or the first consumer of an interface. Its
length is the writer's judgement, short for a cause kept on a doubt. `standing` is for a guarantee
whose failure stays silent and permanent, with the reason in the entry. The argument: `argument@tripwire-standard@arg-6`, `argument@tripwire-standard@arg-8`,
`argument@tripwire-standard@arg-10` to `argument@tripwire-standard@arg-12`. Nearest rivals: a count of occasions survived (`argument@tripwire-standard@arg-10`) and an age or version
sunset (`argument@tripwire-standard@arg-11`).

### The lifecycle

The text of `planned@agent-skills@fragments/tripwire-lifecycle.md`, as shown in round 3 and approved
in round 4. An illustration, not authority:

```markdown
A tripwire leaves its home when:

- **it fires**: an issue is opened or the decision is reopened, as its response says;
- **its horizon passes unfired**: the commit that deletes it names the occasions the decision held
  through. A `rejected` entry kept because a doubt remained, and naming this tripwire, is then
  judged again by the recording tests, and leaves if none holds;
- **its decision is reversed**;
- **another entry guards the same premise**: its clause is folded into that entry;
- **a firing's repair adds a test or a check that now catches the failure**: the failure is no
  longer silent, and the check carries it.

A `standing` tripwire that fires is restated only while its failure stays silent after the repair.
A firing whose only response would be to change the tripwire's own bound shows that the bound had
no basis: rewrite the clause under condition 4.
```

The "retired" exit of the current movement instruction, "when its firing evidence names an artifact
the project's own instructions prevent from existing", is not carried: the surveys found it used 0
times in 250 entries, and the first exit of condition 1 covers its case. The issue-tracking skill
keeps, as host text beside the fragment, its clause on a firing whose reversal is recorded only at
a plan document's harvest, and the rules on deleting a closed issue.

### Who records

At the end of the work that implements a decision (on the full path, the harvest; on the in-change
path, the commit that records the decision), the session judges each premortem cause by the
standard, on its wording then. It writes those that pass, as the standard's entry, in the tripwires
home of the Component that owns the decision, and lists every cause with its outcome in its turn's
"Decided" section and in the record of the work: the harvest's commit message, or the in-change
commit message. No cause is put to the owner. The premortem names causes and no `T` label; the
prefix `T` leaves the table of `design@agent-skills@ruled-items-labelled`. A parked thread's
tripwire is judged by the same standard. A recorded tripwire leaves only as the lifecycle says or on
the owner's word; an audit's removal goes to the owner, per `design@agent-skills@audit-outcomes`.

The decision-record reviewer's predicate "Does a tripwire stand on the owner's word?" becomes: does
each tripwire the diff adds meet the standard, and does the record of the work list every premortem
cause with its outcome. Hosting the standard in that reviewer is D1.

The argument: `argument@tripwire-standard@arg-18`, `argument@tripwire-standard@arg-25`. Nearest rival: shape (B), which still puts each admitted cause to the
owner; defeated by the owner's ruling in round 2.

### Fragments

- **The placeholder** is a line that is exactly `{{fragment:<name>}}`, `<name>` a file directly
  under `planned@agent-skills@fragments/`, with its `.md` extension, so `{{fragment:tripwire-standard.md}}`.
  Like a snippet placeholder, it is a whole line.
- **The pass runs first**, before the `%%` strip, the `{{slug:…}}` rendering and `SUBSTITUTIONS`,
  so a fragment's text is treated as the text around it: its `%%` lines are removed, a delivery
  placeholder in it is filled, and `{{command}}` is filled at install.
- **The build fails** on a placeholder naming no file; on a fragment no placeholder names; on a
  fragment holding a `{{fragment:` line, since a fragment does not nest; and on a fragment holding a
  `{{slug:` placeholder, since the host supplies the heading and its slug, so no slug is defined
  twice in one installed file.
- **The fragments directory** is in the package's `include` list, `/fragments/**`, and in a
  `cargo:rerun-if-changed=fragments` line.
- **The checker** reads a fragment as a document of this repository, as it reads content/: its
  references are checked here, and it cites no entry, per `design@agent-skills@shipped-text-cites-no-entry`.
- **crates/agent-skills/CLAUDE.md** states the fragments directory beside snippets/: what it holds,
  that it is in the walk, and that a fragment is edited, never its hosts' copies.

The argument: `argument@tripwire-standard@arg-26` to `argument@tripwire-standard@arg-29`. Nearest rival: a pointer read whole (`argument@tripwire-standard@arg-26`); a harness hook
(`argument@tripwire-standard@arg-28`).

### The hosts

Each host names the fragment where its act applies the whole text; the table is the owner's,
round 3: "I think the location table is right."

| host | its act | fragments |
| --- | --- | --- |
| content/skills/issue-tracking/SKILL.md, its tripwire section | writing or moving any tripwire | both |
| content/skills/design/SKILL.md, step 7 | proposing causes and criteria | the standard |
| content/skills/decision-recording/SKILL.md, the section now "Writing a tripwire the owner ruled on" | recording or dropping causes at the landing | the standard |
| content/agents/standing-state-reviewer.md | has it fired; has its horizon passed | the lifecycle |
| content/agents/tripwire-auditor.md | judging each entry | both |
| content/skills/planning/SKILL.md, its section on acceptance criteria | a recurring criterion becoming a tripwire; a criterion's quality | the standard |
| content/agents/decision-record-reviewer.md (D1) | judging each tripwire a diff adds | the standard |

Every other text that names tripwires keeps its sentence: they list, search or count them. The
decision-recording section's title changes, since no tripwire is ruled on; its slug
`skill@knowledge-architect-decision-recording@ruled-tripwires` is renamed only if the title's
alignment needs it, and every citation listed under "What is already decided" is then repaired in
the same step.

### Acceptance criteria

- **The owner's word stays**, on its own ground: a criterion decides, before the merge, what the
  branch must build, which a tripwire never does (`argument@tripwire-standard@arg-32`). The rule on a criterion proposed after the
  premortem, marked as awaiting the owner, is unchanged.
- **A criterion meets conditions 3 and 4** of the standard (`argument@tripwire-standard@arg-33`): the planning skill's
  section on acceptance criteria hosts the standard, whose last paragraph says so, and step 7 of the
  design skill hosts it too. The sentence "A number in a criterion is a threshold the owner sets" of
  the planning skill stays, and its proposal carries the basis.
- **A recurring criterion** at its plan document's leaving becomes a tripwire by the standard, not
  on the owner's word.

### The tripwire axis

A third axis of `skill@knowledge-architect-project-audit`, in its section `tripwire-axis`, in the
shape every axis takes:

- **The corpus:** every entry of every tripwires home, read with the head it guards, whole, and with
  the history of that head where the tripwire's ground is the owner's.
- **The rules:** the standard and the lifecycle, carried by the tripwire auditor.
- **Not in this axis:** issues; the design heads the tripwires guard (the design-record axis).
- **The agent:** the tripwire auditor, a draft per entry, and a verify brief, per
  `design@agent-skills@verifier-in-the-run`.
- **The saved workflow:** the tripwire audit: a preflight that saves `cargo klarch tripwires` and
  `cargo klarch show` of each entry and of each guarded head; drafts in groups, the group size the
  session's; a verify step on every draft whose outcome is a removal or a conversion; a report.
- **The outcomes of this axis:**

| outcome | when | applied or to the owner |
| --- | --- | --- |
| kept | it meets the standard | applied: a horizon is written |
| reworded | a slip struck, a response brought to the shape, a number given its basis or replaced by the event | applied: wording is the session's |
| converted | a script could compute its firing | the check is written in the branch where it is small, else a `todo` issue; the tripwire leaves with the check, applied |
| absorbed | another entry guards the same premise | applied |
| removed | it fails condition 1, 2 or 3 | to the owner, grouped by the condition failed, per #removals-grouped-by-condition |
| fired | its evidence is in the tree | the session judges it, and it moves by the lifecycle |

- **No calibration sample.** The design-record axis calibrates per-entry verdicts across groups; here
  every removal reaches the owner, whose ruling on each group is the calibration of the verdicts that
  matter. Decided by the session, listed under "Decided" of the turn that wrote this spec.
- **A `standing` horizon** is read hardest: the auditor names the guarantee and why no check can
  carry it.

## Mapping tables

### From the current exits to the lifecycle

| current exit, `skill@knowledge-architect-issue-tracking@movement-instruction` | lifecycle |
| --- | --- |
| fires: deleted, an issue created or the decision reopened | fires |
| fires, guarding a standing guarantee: restated, stays | a `standing` tripwire fires: restated only while its failure stays silent after the repair |
| decision reversed: deleted | its decision is reversed |
| absorbed into another entry on the same decision | another entry guards the same premise |
| retired: the instructions prevent the artifact | not carried (see "The lifecycle") |
| none | its horizon passes unfired |
| none | a firing's repair adds a check |
| fired, reversal recorded only at a plan's harvest: narrowed to except that plan | kept as host text of the issue-tracking skill |

### From the current premortem labels to the new record

| today | after |
| --- | --- |
| a cause put to the owner as `T<n>` | a cause named in the premortem, no label; judged at the end of the work |
| the owner's ruling on `T<n>` | the session's outcome per cause in "Decided" and in the record of the work |
| an acceptance criterion as `AC<n>` | unchanged |

## Losing alternatives

| alternative | lost to | the fact |
| --- | --- | --- |
| the falsifiability bar alone, with the owner filtering | #tripwire-admission-test | the owner dropped 23 of 67 proposals, and about 45 of 64 standing entries are caught anyway (`argument@tripwire-standard@arg-5`) |
| the three tests of round 1 | #tripwire-admission-test | they drop every firing (`argument@tripwire-standard@arg-15`) |
| the sub-cases of census v2 | #tripwire-admission-test | they raise the unclear verdicts from 15 to 25 (`argument@tripwire-standard@arg-17`) |
| a count of occasions survived, kept in the entry | #tripwire-horizon | churn, and wrong when a session forgets (`argument@tripwire-standard@arg-10`) |
| an age or version sunset | #tripwire-horizon | no evidence for a period (`argument@tripwire-standard@arg-11`) |
| shape (B): admitted causes still put to the owner | #tripwire-recording-authority | the owner's ruling, round 2 |
| a pointer to the issue-tracking section, read whole | #shared-text-fragment | a pointer is delivered only if fetched (`argument@tripwire-standard@arg-26`) |
| a harness hook | #shared-text-fragment | it binds the workflow to one harness (`argument@tripwire-standard@arg-28`) |
| the snippet pass reused for prose | #shared-text-fragment | snippets are inlined last, unprocessed, and written by `init` (`argument@tripwire-standard@arg-27`) |
| a presence check of the horizon line | #tripwire-shape-unchecked | the owner's ruling, round 2; `argument@tripwire-standard@arg-31` |
| an acceptance criterion following (A) | #acceptance-criterion-authority | a criterion gates the merge (`argument@tripwire-standard@arg-32`); the owner's ruling, round 4 |
| census v3 before convergence | #tripwire-admission-test | precision over recall, the owner (`argument@tripwire-standard@arg-23`) |

Which of these earn a `rejected` entry is the harvest's, per `design@agent-skills@losing-alternatives-filter`:
the reversed `design@agent-skills@premortem-tripwires-on-the-owners-word` does, as `owner-rules-each-tripwire`,
refuted by evidence that cost work (`argument@tripwire-standard@arg-5`, `argument@tripwire-standard@arg-15`).

## Readings

Empty: the work reads no external specification.

## Premortem

| # | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| 1 | sessions read "silent" too widely and drop a cause whose failure later shows silent | #tripwire-admission-test | a tripwire: it meets all four conditions; recorded by the session, as the owner confirmed in round 5 that this is the session's call |
| 2 | a fragment reads true in one host and false in another, and no reviewer reads the expanded text, since the agent-text review reads content/, which holds the placeholder | #shared-text-fragment | AC1, on the owner's word, round 5: "apply AC1 and AC2" |
| 3 | the first audit's list of removals is too long to rule on, and removals stall | #tripwire-audit-axis | AC2, on the owner's word, round 5 |
| 4 | writers default to long horizons or to `standing`, and volume does not fall | #tripwire-horizon | dropped: it fails condition 1, since the audit pass shows it and the auditor reads every `standing` entry |
| 5 | sessions record many marginal tripwires, and the owner's load moves from ruling to reviewing | #tripwire-recording-authority | dropped: it fails condition 2, since only the owner sees it, and the owner overrules |
| 6 | criteria still carry bounds with no basis | #acceptance-criterion-quality | dropped: it fails condition 1, since the owner rules on every criterion |

The tripwire of cause 1, written at the harvest in `register@agent-skills@tripwire`, guarding the
head of #tripwire-admission-test:

- **Fires when:** an issue is opened on a silent failure of a decision, and a commit message or a
  plan document of the work that recorded that decision lists the cause as dropped under condition 1.
- **Response:** reopen the admission test, on how condition 1 is worded.
- **Re-entry:** the standing-state review: the issue and the commit are in the tree and the history.
- **Watches until:** the fifth design decision recorded with a premortem after this work lands, in
  this repository or in thaum through an intake.

## Acceptance criteria

### AC1: every host's installed copy reads true with its fragments expanded `##fragments-read-in-their-hosts`

- **Guards:** #shared-text-fragment.
- **Judged at:** the review of the work, point 4: the agent-text axis is briefed to read every host's
  installed copy under .claude, the fragments expanded, beside the source.
- **Fires when:** a sentence of a fragment is false, or names an act the host's reader cannot
  perform, in a host's context: for example "the session that implements the decision judges each
  cause" read by the standing-state reviewer, which implements nothing.
- **Response:** reopen #shared-text-fragment on what a fragment may hold, and split the fragment or
  move the sentence to the hosts that need it. The design consequence the session drew, that the
  agent-text reviewer reads its hosts' installed copies whenever a fragment changes, is written into
  `agent@knowledge-architect-agent-text-reviewer` at step 2 whatever this criterion's outcome.

### AC2: the first pass's report groups its proposed removals by the condition each fails `##removals-grouped-by-condition`

- **Guards:** #tripwire-audit-axis.
- **Judged at:** step 5, the first pass.
- **Fires when:** the report's list for the owner holds a removal not in a group of the condition it
  fails, or a group without a default.
- **Response:** reopen #tripwire-audit-axis on how the report is shaped, and repair the saved
  workflow's report step before the owner rules.

## Implementation sequence

### Step 1: the fragment pass, empty

Builds `Fragments` and `FRAGMENT` in build.rs, the pass run first, its four build failures, the
`include` entry, the `rerun-if-changed` line, and the CLAUDE.md paragraph. No content file uses a
placeholder yet, and the fragments directory is absent or empty, which the pass accepts as snippets/
is accepted.

- **Claims:** a placeholder line is replaced by its file's text; the replacement happens before the
  `%%` strip, so a `%%` line of a fragment is absent from the rendered host; each of the four
  failures fails the build. **Tests:** unit tests of the rendering function, in the shape of
  render.rs's, which are 28 today: one per claim, and one per failure, each shown to fail by
  mutating the function (the replacement skipped, the order swapped, each assertion removed), per
  `skill@klarch-development`.
- **Fails alone on:** a rendered file that differs from its source with no placeholder in it.

### Step 2: the standard, its hosts and the text that states the old rules

Builds the two fragments and every host edit of "The hosts"; the rewrite of
`skill@knowledge-architect-decision-recording@ruled-tripwires` to "Who records";
`skill@knowledge-architect-issue-tracking@tripwire-entry` and
`skill@knowledge-architect-issue-tracking@movement-instruction` to host the fragments, keeping their
text on issue entries; step 7 of the design skill, its parked-thread paragraph and the end of
`skill@knowledge-architect-design@discussion-overview`; the planning skill's Premortem row, its
section on acceptance criteria and its harvest point; the primer's tripwire row of the knowledge table
and its tripwire example in `primer@labelled-items`; the decision-record reviewer's predicate; the
agent-text reviewer's reading of hosts (AC1's consequence); the `%%` lines listed under "What is
already decided". Then `cargo klarch install-agent-skills`.

- **Claims:** no shipped text states that a tripwire is written on the owner's word, or puts a cause
  to the owner under `T<n>`; every host's installed copy holds its fragment's text once.
  **Tests:** the first is a corpus claim, tested by reading every sentence of content/ that names a
  tripwire, the list in the session's scratch file of the work's grounding taken again, not by a
  phrase search alone, per `skill@knowledge-architect-planning@working-a-slice` point 3; the second
  by a test over `FILES` in src/lib.rs, shown to fail by removing one placeholder.
- **Fails alone on:** an installed copy that cites a section the edit renamed.

### Step 3: the tripwire axis

Builds the tripwire auditor, the tripwire audit, and the section `tripwire-axis` of the project-audit
skill, in the shape of the design-record axis's three files. Reinstall.

- **Claims:** the workflow parses (`node --check` on its body wrapped as crates/agent-skills/CLAUDE.md
  says); its `meta.name` matches its file stem, which the build asserts; the section names the
  agent and the workflow by their checked references. **Tests:** the build, and `cargo klarch check`.
- **Fails alone on:** a dry run of the preflight that lists fewer entries than `cargo klarch tripwires`.

### Step 4: the review of the work

Per `skill@knowledge-architect-review`, on every axis its table owes; the agent-text axis briefed
for AC1, which is judged here and reported in the commit that answers the review.

### Step 5: the first pass of the tripwire axis on this repository

Runs the tripwire audit, verifies the report, commits it under docs/audit-reports/, applies the
applied outcomes, and puts the removals to the owner as the report's grouped list. AC2 is judged
here. The removals are applied once the owner has ruled, before the merge.

### Step 6: the harvest, the changelog and the spec's leaving

Per "Harvest". Then the review before the merge, then the spec's deletion in a commit of its own.

## Order rationale

- Step 1 before step 2: the hosts need the pass, and an empty pass fails on the mechanism alone.
- Step 2 before step 3: the auditor hosts the fragments step 2 writes.
- Step 3 before step 4: the review reads the whole built work once.
- Step 4 before step 5: the audit applies a standard the review has read.
- Step 5 before step 6: the harvest writes T1 and the heads after the old entries are judged, so the
  audit judges none of the work's own records.

## Defaults awaiting the owner

### D1: the decision-record reviewer hosts the standard

The approved host table has no reviewer that judges a tripwire a diff adds: the standing-state
reviewer hosts the lifecycle only. Today the decision-record reviewer judges each added tripwire
against the owner's word, a predicate this work removes. **Default:** the decision-record reviewer
hosts the standard and judges each added tripwire by it, and checks that the record lists every
premortem cause with its outcome. This adds a host to the table the owner approved, so it is the
owner's. Bears on #shared-text-fragment and #tripwire-recording-authority.

## Harvest

| what | where | when |
| --- | --- | --- |
| #tripwire-admission-test | a head in `register@agent-skills@design`, under the thread's slug | the harvest of step 6 |
| #tripwire-horizon | a head in `register@agent-skills@design`, under the thread's slug | step 6 |
| #tripwire-recording-authority | a head under the thread's slug, absorbing `design@agent-skills@tripwire-wording-is-the-agents`, which is deleted; `design@agent-skills@premortem-tripwires-on-the-owners-word` moves to `register@agent-skills@rejected` as `owner-rules-each-tripwire`; `design@agent-skills@premortem-tripwire-written-at-harvest` rewritten | step 6 |
| #shared-text-fragment | a head; `design@agent-skills@restatement-size-test` and `design@agent-skills@content-mirrors-the-install-layout` rewritten | step 6 |
| #tripwire-audit-axis | a head; `design@agent-skills@design-record-axis` and `design@agent-skills@audit-is-an-activity` rewritten where they count the axes | step 6 |
| #tripwire-shape-unchecked | judged by the entry tests; the session's reading is no head | step 6 |
| #acceptance-criterion-authority | `design@agent-skills@acceptance-criteria-on-the-owners-word` rewritten on its own ground | step 6 |
| #acceptance-criterion-quality | a head in `register@agent-skills@design`, under the thread's slug; `design@agent-skills@acceptance-criteria-in-the-document` rewritten | step 6 |
| the other rewrites of "What is already decided" | in place | step 6 |
| T1 | `register@agent-skills@tripwire`, guarding the head of #tripwire-admission-test | step 6 |
| the issues of "What is already decided" | closed or rewritten as listed | the severity-bound and eight-tripwires issues at step 5; the others at step 6 |
| CHANGELOG.md, `Next release` | Workflow entries for the standard, the horizon, who records, fragments and criteria; New features for the tripwire axis; a Migration entry: every tripwire gains a horizon and meets the standard, by running `skill@knowledge-architect-project-audit` on its tripwire axis; then `cargo x changelog` | step 6 |
| this spec | deleted in a commit of its own | after the review before the merge |

## Later consequences

- **Other standards move to fragments**: the `todo` the pointer issue becomes. Its first candidate,
  named in it, is the retrospective intake's pointers to the report format.
- **The retrospective watches the tripwires on session behaviour**: the rest of the agent-behaviour
  issue. Condition 2 already counts a received retrospective file as a reading occasion.
- **thaum's first pass** runs at its next pin move, under the Migration entry.
- **The issue half of "standing state as a whole"** stays in the audit-axes issue, and may share
  this axis's preflight.
