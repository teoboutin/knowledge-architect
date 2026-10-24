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
- it adds a build pass, `{{fragment:<file>}}`, that copies one file of shared text into every
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
  what waits: a default is built as written, and the branch merges once the owner has ruled.
- **One branch and one pull request**, the spec's first commit included, on the owner's word,
  round 5: "this should take the shape of a single spec with one PR." That word also gives the
  first pass of the tripwire axis to this branch, which
  `skill@knowledge-architect-project-audit@when-an-audit-runs` requires of an audit run in another
  work's branch. No commit of the work changes what `cargo klarch commits` judges in a commit's
  tree: the checker compares no installed copy under `commits`, per
  `design@core@installed-entities-from-the-tree`, and no checker code changes.

**The discussion.** It ran in one session, which begins at the owner's message opening "Right now,
tripwires accumulate indefinetely in projects following this workflow." This spec is assembled from
that session's transcript. A **round** here is one owner message and everything up to the next:
round 1 opens with that message, and the owner's messages of rounds 2 to 5 open
"tripwire-admission-test: i think this is good", "tripwire-recording-authority: accepted", "I think
the standard is good" and "recording T1 no longer needs my call". The session's own headings,
"Round 1" to "Round 5", are numbered differently, and this spec does not use them.

The measurements the discussion rests on were taken by read-only subagents in that session, over
this repository at commit 6da4781 and over thaum, a project that installs the workflow, at its
checkout of 2026-10-24. Their scripts are not in the tree. Each figure below names its source and
which parts are a subagent's judgement; the first pass of step 5 takes this repository's register
again under the standard.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, as it applies to the work of a spec.

## Names

| name | what it is |
| --- | --- |
| the standard | the text of `planned@agent-skills@fragments/tripwire-standard.md`: what earns a tripwire and the entry's shape |
| the lifecycle | the text of `planned@agent-skills@fragments/tripwire-lifecycle.md`: when a tripwire leaves its home |
| condition 1 to 4 | the four numbered conditions of the standard, in "Decided design" |
| a fragment | a file directly under `planned@agent-skills@fragments/`, copied by the build into every content file that names it |
| a host | a content file that names a fragment by a placeholder line |
| the horizon | the `**Watches until:**` line of a tripwire entry |
| silent | the property of condition 1: every procedure that meets the failure counts it as success |
| shape (A) | the session records or drops each premortem cause by the standard, with no question to the owner, and lists every cause with its outcome; the owner overrules by reviewing. Built as "Who records" |
| shape (B) | the rival of shape (A): the session drops the causes the standard excludes, and puts each admitted cause to the owner |
| T1 | the label under which the session put this spec's premortem cause 1 to the owner, before shape (A) was built; its tripwire keeps the label in its text |
| the census | the subagent's application of the admission test, as worded in rounds 1 and 2, to the 85 premortem causes of this repository's deleted plan documents and in-change premortems, in two versions, v1 and v2 |
| the counterfactual | the subagents' reading, in hindsight, of 12 firings of this repository and 37 of thaum: would each fact have reached a session that reopens the decision without the tripwire |
| the useful firings | the 8 firings the counterfactual judged to need the tripwire: here `decision-trigger-does-not-fire` and `member-beyond-the-argument`; in thaum `one-mutation-vocabulary-three` (twice), the commit-message tripwire, `a-commit-is-skipped`, the premortem tripwire on last known information, and `one-target-hardcoded` of its mutate Component |
| the surveys | the two subagents' readings of the tripwire registers of this repository (64 standing) and of thaum (130 standing), with their history |
| the severity issue | `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` |
| the eight-responses issue | `issue@agent-skills@eight-tripwires-respond-with-an-action-the-tripwire-shape-does-not-name` |
| the agent-behaviour issue | `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` |
| the pointer issue | `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it` |
| the audit-axes issue | `issue@agent-skills@audit-axes-beyond-the-design-record` |
| design.md:NNNN | a line of crates/agent-skills/docs/design.md at commit 6da4781; the core's design home is written out in full |
| the tripwire axis | the third axis of `skill@knowledge-architect-project-audit`, built by this work |
| the tripwire auditor | the agent `knowledge-architect-tripwire-auditor`, built by this work |
| the tripwire audit | the saved workflow `knowledge-architect-tripwire-audit`, built by this work |
| build.rs | `path@agent-skills@build.rs` |
| render.rs | `path@agent-skills@src/render.rs`, the passes build.rs includes and the crate's tests compile |
| the installed copies | the files `cargo klarch install-agent-skills` writes under .claude, committed in this repository |

## What the work is

**Today.**

- **The entry shape** is `skill@knowledge-architect-issue-tracking@tripwire-entry`: a level-two
  heading `## Guarding <decision>: <title>` with `**Fires when:**`, `**Response:**` and
  `**Re-entry:**`. It holds two bars on the firing evidence: falsifiability, "specific enough that
  both parties would agree it fired", and "If a script could check the firing evidence, it is not a
  tripwire". Step 7 of `skill@knowledge-architect-design@discussion-loop` restates the first in one
  sentence and does not deliver the second.
- **The exits** are `skill@knowledge-architect-issue-tracking@movement-instruction`: fired,
  reversed, absorbed, and retired when the instructions prevent the artifact the evidence names. A
  tripwire guarding a standing guarantee is restated when it fires and stays.
- **Who decides** is `skill@knowledge-architect-decision-recording@ruled-tripwires`: "A tripwire is
  written only on the owner's word". The premortem of step 7 puts each surviving cause to the owner
  under a label `T<n>`, and each acceptance criterion under `AC<n>`. The same rule is stated or
  assumed in these texts of content/, each edited at step 2: design/SKILL.md lines 147, 154 (the
  in-change path), 343 (a parked thread's tripwire), 595 to 621 (step 7) and 629 (step 8);
  planning/SKILL.md lines 18, 174, 178, 328 to 358 and 444; decision-recording/SKILL.md lines 3
  and 13; agents/standing-state-reviewer.md lines 102 to 104; agents/decision-record-reviewer.md
  lines 49 and 105 to 106; PRIMER.md lines 234 to 236 and agents/transcript-reviewer.md lines 20
  to 22 (a rewording "that keeps the cause it watches" is the session's).
- **The standing-state reviewer**, `agent@knowledge-architect-standing-state-reviewer`, reads every
  tripwire at every merge and asks "has it fired?".
- **The build** applies to each file of content/, in order: the workflow-name check on the raw text
  of a saved workflow, the `%%` comment strip, the `{{slug:…}}` rendering, the `SUBSTITUTIONS` pass,
  and the `{{snippet:…}}` pass, at build.rs lines 109 to 117. An unused snippet and an unused
  substitution each fail the build. Snippets are inlined last and unprocessed, and are listed as
  `SNIPPETS`, four of which `klarch init --rust` writes into a new project. A placeholder line is
  matched only at column 0. build.rs has no test target; the comment and slug passes live in
  render.rs so that the crate's tests hold their rules.
- **The project audit** has two axes, `skill@knowledge-architect-project-audit@design-record-axis`
  and `skill@knowledge-architect-project-audit@agentic-workflow-axis`. Each has a section of the
  skill, an agent of its own and a saved workflow under content/workflows/.

**What the surveys measured**, over both registers since 2026-08-13: 250 tripwires added, 194
standing, 0 retired, 1 of 194 stating when it stops being needed. The full table is under
`argument@tripwire-standard@arg-5`.

**Outside the work**, each with what owns it:

- **The retrospective watching session behaviour against the tripwires**, the owner's plan in the
  agent-behaviour issue. This work takes from that issue only the rule that the evidence lands where
  tripwires are read, as condition 2, which counts a received retrospective file. The issue is
  rewritten to the rest, under "Harvest".
- **Migrating other standards to fragments.** The owner, round 3: "others will be migrated later."
  The pointer issue is rewritten as the `todo` that carries it.
- **The issue half of a "standing state as a whole" audit**, in the audit-axes issue. The tripwire
  axis reads tripwires only.
- **thaum's first pass of the tripwire axis.** It runs at thaum's next pin move, under the
  Migration entry this work writes, per `design@agent-skills@audit-triggers`.
- **A checker rule on the horizon line**, kept out for now by #tripwire-shape-unchecked.

## What is already decided

**Decisions the design rests on and does not argue again.**

- `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@agents-work-without-drift`,
  `goal@agent-skills@installed-text-leaves-room-to-judge`, `goal@agent-skills@no-contradiction-in-workflow`,
  `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@installed-text-works-anywhere`:
  the criteria of the discussion derive from them.
- `skill@knowledge-architect-issue-tracking@trigger-test`: a trigger names an occasion whose own work
  already includes the work it names. The horizon's exit is read at an occasion whose work already
  reads every tripwire.
- `design@agent-skills@audit-triggers`, `design@agent-skills@audit-is-an-activity`,
  `design@agent-skills@audit-agent-per-axis`, `design@agent-skills@audit-axis-runs-a-workflow`,
  `design@agent-skills@audit-method`, `design@agent-skills@one-run-pass`,
  `design@agent-skills@committed-audit-report`, `design@agent-skills@apply-stays-with-the-session`,
  `design@agent-skills@verifier-in-the-run`, `design@agent-skills@design-record-axis-dispatch`,
  `design@agent-skills@calibration-in-the-run`: the tripwire axis takes the shape the design-record
  axis takes, step by step, as "The tripwire axis" states.
- `design@agent-skills@audit-outcomes`: a removal of an entry goes to the owner. Every outcome of
  the tripwire axis that deletes an entry does, grouped, per #removals-grouped-by-condition.
- `design@agent-skills@synthetic-evidence-not-built`: no run of an agent following a planned
  instruction chooses between shapes. The census is a lead the owner asked for, round 2: "You can
  run the census you proposed of the past premortems"; no shape of this spec, and no `rejected`
  entry it plans, rests on its counts. The shapes rest on the surveys and the counterfactual, which
  read what happened in real sessions, and on the owner's proposals.
- `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`:
  a new rule on recorded content owes a Migration entry that cites the axis bringing content to it.
- `design@knowledge-architect@changelog-section-fixed-at-release`: the released 0.6.0 section of
  CHANGELOG.md cites `skill@knowledge-architect-decision-recording@ruled-tripwires` at its line
  361, so that section keeps its slug.
- `design@core@installed-entities-from-the-tree`: `commits` compares no installed copy, so the
  work's commits need no ordering for the per-commit gate.
- `design@core@tripwire-guards-no-rejected-entry`: unchanged; the horizon's re-test of a `rejected`
  entry reads the entry's own text, not the `guarding` column.
- `rejected@core@validity-marker-checked`: the nearest rival the checker has refused, a marker read
  on a heading entry, for lacking a syntax of its own.
- `design@agent-skills@shipped-text-cites-no-entry` and `design@agent-skills@shipped-text-line-comments`:
  a fragment cites no entry of this repository, and its `%%` lines are removed like any content
  file's.
- `design@knowledge-architect@package-include-whitelist`: the crate's `include` list gains the
  fragments directory.

**Decisions this work reverses or rewrites**, each with every text that cites it, as
`cargo klarch show` lists them at commit 6da4781, and the step or harvest that updates each.

| decision | change | citing texts, and who updates them |
| --- | --- | --- |
| `design@agent-skills@premortem-tripwires-on-the-owners-word` | reversed: moves to `register@agent-skills@rejected` as `owner-rules-each-tripwire` | the `%%` at content/skills/decision-recording/SKILL.md:251 (step 2); design.md:1031, 1608 and 1629, in the bodies of three heads this table rewrites (harvest) |
| `design@agent-skills@premortem-tripwire-written-at-harvest` | absorbed by the head of #tripwire-recording-authority, which states where and when a tripwire is written, and deleted: on the in-change path the end of the work is no harvest | the `%%` at decision-recording/SKILL.md:252 (step 2); design.md:1020 (harvest) |
| `design@agent-skills@tripwire-wording-is-the-agents` | absorbed by the head of #tripwire-recording-authority, and deleted | design.md:1631 (harvest); the eight-responses issue lines 34 and 46, which closes at step 5 |
| `design@agent-skills@decisions-reported-to-the-owner` | rewritten where its body, at design.md:2992, names a rewording of a tripwire that "keeps the cause it watches": a tripwire recorded, dropped or reworded is a "Decided" item | harvest; the same rule in PRIMER.md:234 to 236 and agents/transcript-reviewer.md:20 to 22 (step 2) |
| `design@agent-skills@acceptance-criteria-on-the-owners-word` | rewritten on its own ground, per #acceptance-criterion-authority | the `%%` at content/skills/design/SKILL.md:595 and planning/SKILL.md:328 (step 2); `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` line 48 (harvest re-reads it: the head keeps its rule on a criterion proposed after the premortem) |
| `design@agent-skills@acceptance-criteria-in-the-document` | rewritten: a recurring criterion becomes a tripwire by the standard, not on the owner's word | nothing cites it; step 2 adds a `%%` line citing it at the planning skill's section on acceptance criteria |
| `design@agent-skills@restatement-size-test` | rewritten: a fragment the build fills is the main delivery of a standard, or a similar large block of coherent instructions, read whole at several places | design.md:585, 1264, 3007 (harvest re-reads each: they cite the size rule, which stands for restatements); `tripwire@agent-skills@pointer-not-followed` (step 5 judges it); `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` line 72 (harvest re-reads); the pointer issue (rewritten at harvest); the rule's text in `primer@where-knowledge-goes`, PRIMER.md:324 to 328 (step 2) |
| `design@agent-skills@ruled-items-labelled` | rewritten: the `T` row leaves the prefix table | the `%%` at decision-recording/SKILL.md:250 and design/SKILL.md:596 (step 2); the other `%%` sites, PRIMER.md:65 and 446, planning 329 and 520, project-audit 73, retrospective 69, review 163, setup 297, cite the head for other prefixes and stay; design.md:1632, 2789, 2919 (harvest re-reads) |
| `design@agent-skills@design-hands-off-to-planning` | rewritten: the design skill ends at convergence, the premortem and the owner's rulings on its acceptance criteria | design.md:1536; crates/core/docs/design.md:1001, which cites it for the plan-document section lists this work leaves unchanged (harvest re-reads both) |
| `design@agent-skills@in-change-path` | rewritten: the commit message carries every premortem cause with its outcome, and the rulings on acceptance criteria | design.md:1566, 1784; `tripwire@agent-skills@ruling-lost-in-change` (step 5 judges it); the agent-behaviour issue line 20 (harvest) |
| `design@agent-skills@standing-state-review-before-every-merge` | rewritten: the reviewer also reads every horizon | design.md:1200; the agent-behaviour issue line 66; `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` line 50 (harvest re-reads all three) |
| `design@agent-skills@standing-entries-searched-before-the-work` | rewritten where it lists "a tripwire of the milestone document's premortem" at design.md:1192 to 1194: a premortem holds causes, and a cause enters a tripwires home only at the end of the work | harvest |
| `design@agent-skills@design-record-axis` | rewritten: "the conformance of a tripwire … to its own register's rules is the axis of that register", at design.md:2261 to 2262, names the tripwire axis | `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component` line 25 (harvest re-reads); `tripwire@agent-skills@component-judgement-noisy` (step 5) |
| `design@agent-skills@content-mirrors-the-install-layout` | rewritten: the build also fills fragment placeholders | crates/agent-skills/CLAUDE.md:6 (step 1); crates/core/docs/design.md:2742 (harvest re-reads) |
| `design@agent-skills@knowledge-table-home` | unchanged; the primer's tripwire row it ships gains "or its horizon passes" (step 2) | CLAUDE.md:47 and design.md:2185, 3080, 3383 cite the head, not the row: nothing to update |
| `rejected@agent-skills@directive-restated-everywhere` | unchanged: its recorded reason, nine inconsistencies between hand-written restatements and their home, does not cover a copy the build fills from that home | design.md:3058 (the rewritten `design@agent-skills@restatement-size-test` keeps citing it) |

**Standing entries the discussion's searches returned, with their outcome.**

| entry | outcome |
| --- | --- |
| the severity issue | closed: the rule "a slip is no firing" states the bound; its four tripwires go through step 5 |
| the eight-responses issue | closed at step 5: each of the eight is reworded to the shape, or removed on the owner's word |
| the agent-behaviour issue | rewritten at harvest: condition 2 is its re-entry rule; the retrospective watch stays open |
| the pointer issue | rewritten at harvest as a `todo`: migrate the other standards to fragments |
| the audit-axes issue | rewritten at harvest: the tripwire half of "standing state as a whole" exists, and the count of axes in its title and summary changes |
| `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` | bears on nothing here; it is cited as evidence in `argument@tripwire-standard@arg-21` |
| `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` | bears on the census: the census decides no shape here, per `design@agent-skills@synthetic-evidence-not-built`; the issue stays open, its question the owner's |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`, `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt`, `issue@agent-skills@the-retrospective-counts-no-review-cost` | bear on nothing in the work: the searcher's input shrinks, its procedure does not change |
| every tripwire of this repository | judged at step 5, the first pass of the tripwire axis; 37 lines of them carry a `T<n>` label or a ground "on the owner's word", which the pass rewords where the entry is kept |

**Design and `rejected` entries graded `bears`**, beyond those above: `design@agent-skills@owner-rules-at-a-fork`
(a cost alone makes no fork, which #tripwire-recording-authority rests on);
`design@agent-skills@questions-go-to-their-authority` (the same); `design@agent-skills@losing-alternatives-filter`
(the horizon's re-test of a `rejected` entry kept on a doubt applies its tests); `design@agent-skills@goal-lifecycle`
(answered in `argument@tripwire-standard@arg-14`); `rejected@agent-skills@delegated-decision-state`
(not brought back: `argument@tripwire-standard@arg-34`); `rejected@agent-skills@audit-without-calibration-or-recheck`
(not brought back: the axis keeps the calibration and the re-check of the design-record axis).

## Criteria

### The owner rules only on what the goals and the record leave open `##owner-rules-only-what-is-open`

Binding, from `goal@knowledge-architect@agents-work-without-drift`. Met by #tripwire-recording-authority
and #tripwire-admission-test: once the standard ranks a cause, recording it is no fork.

### The owner is informed of every decision and can overrule it `##owner-informed-of-every-decision`

Binding, from `goal@knowledge-architect@the-owner-decides`. Met by #tripwire-recording-authority:
every cause is listed in the turn's "Decided" section, and, under D2, in the record of the work,
with its outcome.

### No fixed count or sequence where judgement serves `##room-to-judge-kept`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met by #tripwire-admission-test,
which names conditions and a default and no count, and by #tripwire-horizon, whose length is the
writer's judgement.

### A horizon's exit is read at an occasion whose work already reads the entry `##exit-occasion-reads-the-entry`

Binding as a presumption, derived from `skill@knowledge-architect-issue-tracking@trigger-test`, which
asks the same of a trigger. Met by #tripwire-horizon: the standing-state reviewer reads every entry
at every merge already.

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
reported that it drops 46 of the 62 causes the owner kept and every tripwire that later fired
(`argument@tripwire-standard@arg-15`); the session presented that as a material finding and refined
the tests (v2), which raised the unclear verdicts from 15 to 25 (`argument@tripwire-standard@arg-17`).
The owner, round 3, proposed the counterfactual test: "would the facts that made the fired
tripwires fire have been found out without the tripwires in the record ? if yes, that probably means
they were not that useful anyway." The counterfactual (`argument@tripwire-standard@arg-20`,
`argument@tripwire-standard@arg-21`, `argument@tripwire-standard@arg-22`) reshaped the test around
silence, the owner's proposal, and produced conditions 3 and 4. The owner closed it, round 4: "I
think the standard is good. even if it drops a few of the useful ones, i'd keep it, unless a small
refinement can make them pass. what passes will always carry a little bit of "subjectivity" anyway,
since not all long term conseauences may come to the mind of everyone. the important criteria is
rather that accepted tripwires are valuable, rather than no past useful tripwire stays accepted. the
future side is more important than the past here." The condition "unless a small refinement can
make them pass" is carried by step 2, which applies the standard to the useful firings first.

### Every tripwire names the occasions after which, unfired, it leaves `##tripwire-horizon`

Proposed by the session, round 1, answering the owner's "retirement clause". Final state:
`approved`. The owner, round 2: "this is good, this is what I meant by "retirement clause"". The
lifecycle shown in round 3, its exit "a firing's repair adds a test or a check" included, was
approved with the standard in round 4: "I think the standard is good". Shape: "Decided design",
"The entry and its horizon" and "The lifecycle". Harvest: a head in `register@agent-skills@design`,
under this slug.

### The session records or drops each cause at the end of the work, and the owner overrules by review `##tripwire-recording-authority`

Proposed by the session, round 1, as shape (A), against shape (B). Final state: `approved`. The
owner, round 2: "Also, I think the ruling on whether to record tripwires should be reversed: keep is
the default, my word is only needed for a tripwire to be dropped, and asked only at the end of the
work that creates the tripwire, with the wording it gets at that moment, not the wording from the
premortem, which often needs adjustements. or maybe i do not even need to be asked at all. i
overrule by reviwing myself, the plan or the work. this is your position on
tripwire-recording-authority, and i agree with it." The session stated its reading in its next
turn: the session records the causes that pass and drops those that fail, with no question to the
owner, and a recorded tripwire leaves by firing, reversal, horizon, the owner's word or an audit
removal the owner rules on. The owner, round 3: "tripwire-recording-authority: accepted". In round
5 the owner asked, "recording T1 no longer needs my call under the rworked workflow we decided, no
?", and the session confirmed.

The owner's aim, round 1, reaches a recorded tripwire too: "a lot of the findings I am asked to rule
on are about wording and small scope changes of tripwires, which judgement should be able to cover,
under a clearer standard." So a change of a recorded tripwire's wording or scope is the session's,
judged by the standard, and its removal outside the lifecycle is the owner's.

Shape: "Decided design", "Who records". Harvest: a head under this slug in `register@agent-skills@design`,
standing on `argument@tripwire-standard@arg-25` and `argument@tripwire-standard@arg-34`, absorbing
`design@agent-skills@tripwire-wording-is-the-agents` and `design@agent-skills@premortem-tripwire-written-at-harvest`;
the reversed `design@agent-skills@premortem-tripwires-on-the-owners-word` moves to `register@agent-skills@rejected`.

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

### A fragment the build copies is the main delivery of a standard, or a similar large block of coherent instructions, read whole at several places `##shared-text-fragment`

Proposed by the session, round 2, as the mechanism for the owner's substitution. Final state:
`approved`. The owner, round 3: "agreed on rewriting design@agent-skills@restatement-size-test,
fragments have to become the main delivery mechanism for standards and similar large blocks of
coherent instructions that need to be read as a whole at multiple locations. others will be
migrated later." Shape: "Decided design", "Fragments" and "The hosts". Harvest: a head under this
slug in `register@agent-skills@design`, and the rewrites of `design@agent-skills@restatement-size-test`
and `design@agent-skills@content-mirrors-the-install-layout`.

### An audit axis brings every standing tripwire to the standard `##tripwire-audit-axis`

Proposed by the session, round 1. Final state: `approved`. The owner, round 2: "tripwire-audit-axis:
shape looks good". The round-1 outcome "sent to the retrospective's watch", for a behaviour pattern
seen only in sessions, is overtaken by the standard approved in round 4: its condition 2 counts a
received retrospective file as a place tripwires are read, so such an entry is kept with a horizon
when its evidence can reach one, and removed, on the owner's word, when it cannot. Shape: "Decided
design", "The tripwire axis". Harvest: a head under this slug in `register@agent-skills@design`, and
the rewrite of `design@agent-skills@design-record-axis`.

### The checker does not check the horizon line, for now `##tripwire-shape-unchecked`

Proposed by the session, round 1, as the default against a presence check of the horizon line.
Final state: `approved`. The owner, round 2: "not much reason to check yet, we can keep this out of
the checker". Shape: nothing is built. Harvest: a head of absence in `register@core@design`, which
entry test 2 admits as a decision that no code states, its ground attributed to the owner with the
hedge "for now".

### An acceptance criterion keeps the owner's word, on the ground that it decides what the branch must build `##acceptance-criterion-authority`

Proposed by the session, round 2, split from #tripwire-recording-authority, since
`design@agent-skills@acceptance-criteria-on-the-owners-word` argued from the tripwire analogy that
shape (A) removes. Final state: `approved`. The owner, round 3: "i'd like to discuss this further. I
want to increase autonomy of agents. but i have to agree that acceptance criteria are an important
point were my ruling is useful." Round 4: "acceptance-criterion-authority and
acceptance-criterion-quality accepted, do it in the same work". Shape: "Decided design", "Acceptance
criteria". Harvest: the rewrite of `design@agent-skills@acceptance-criteria-on-the-owners-word`, the
pair `#acceptance-criterion-authority → acceptance-criteria-on-the-owners-word` stated in the
harvest's commit message.

### An acceptance criterion names a built result under which it fails, and its bound carries its basis `##acceptance-criterion-quality`

Proposed by the session, round 3, from the owner's report of round 3: "The main problem i noticed
is that I think the acceptance criteria produced this way are too often of low quality, in
particular they carry numerical bounds that are not truly relevant, or they are basically
guaranteed to pass. but maybe this is outside the scope of today's work." Final state: `approved`,
in scope, round 4: "do it in the same work". Shape: "Decided design", "Acceptance criteria".
Harvest: a head under this slug in `register@agent-skills@design`.

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

### The owner's cases of a tripwire not worth recording, and the aims of the rework `##arg-4`

Round 1, the owner: tripwires "hard to observe by an agent, while they would be obvious to me when
driving a session"; "the failure of a design that I am certain will never happen"; "one that will
probably happen but is minor (like an agent not following an instruction once or twice in a session,
which is normal, no one can perfectly follow such massive amounts of instruction, and reviews are
meant to catch that". And the aims: "make sure tripwire volume stays controlled (not a hard cap, but
making sure those added are worth it, and those that prove not worth are removed), as well as
improve agent autonomy on selecting tripwires (I am dropping a lot of the proposed ones right now,
and a lot of the findings I am asked to rule on are about wording and small scope changes of
tripwires, which judgement should be able to cover, under a clearer standard)." Bears on every
tripwire thread.

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
| removed: other | 1 (two entries merged into one) | 1 (dropped with no stated reason) |
| retired | 0 | 0 |
| stating when it stops being needed | 1 | 0 |
| caught anyway by a review, a test, the checker or the owner | about 45 | 34, and 22 partly |
| firing evidence a script could check | about 7 | 28, and 23 partly |
| observable only by the owner or only in another project | 22 | 1 |

Of 67 premortem causes then counted here, the owner recorded 43, dropped 23, and 1 was voided with
its thread; the census later recounted 85. A later reviewer counted 83 distinct slugs added here by
another method, since early headings carried no slug. Bears on #tripwire-horizon,
#tripwire-admission-test and #tripwire-recording-authority.

### The firings came early `##arg-6`

Round 1, the session: the 9 tripwires of this repository removed on firing had fired 1, 0, 34, 0,
3, 3, 13, 15 and 15 days after their entry was written; the register is 72 days old, so the data is
censored. Bears on #tripwire-horizon.

### The existing standard is not applied where tripwires are written `##arg-7`

Round 1, the session: about 35 entries across both projects break "If a script could check the
firing evidence, it is not a tripwire", which step 7 never sends the writer to read. Bears on
#tripwire-standard-home and #shared-text-fragment.

### The mechanism of the growth `##arg-8`

Round 1, the session: "every premortem cause you accept becomes an entry with no end, and the only
events that remove it, a firing or a reversal, are rare by construction, because a premortem names
unlikely failures." Bears on #tripwire-horizon.

### The first admission test: delivery, seen where read, weight `##arg-9`

Round 1, the session: a cause becomes a tripwire only when its evidence would not otherwise reach a
session that would reopen the decision; when its re-entry holds the evidence; and when the evidence
shows the decision wrong, not one execution of it. Bears on #tripwire-admission-test; overtaken by
the standard of round 4.

### A horizon, not a count of occasions survived `##arg-10`

Round 1, the session: a count of occasions survived, kept in the entry, is churn, and wrong
whenever a session forgets to update it. Bears on #tripwire-horizon.

### An age or version sunset has no evidence for its period `##arg-11`

Round 1, the session: a sunset by date or version fails `design@agent-skills@audit-triggers`, since
a period has no evidence for it, and it fires on whoever crosses the date. Bears on #tripwire-horizon.

### The standing-state reviewer is already at the exit occasion `##arg-12`

Round 1, the session: the reviewer reads every tripwire at every merge and asks "has it fired?"; it
asks "has its horizon passed?" at the same reading, so no new re-entry point is needed. Bears on
#tripwire-horizon and #exit-occasion-reads-the-entry.

### A tripwire that leaves on its horizon answers a `rejected` entry's doubt `##arg-13`

Round 1, the session: 13 `rejected` entries stay only because a doubt remains that a named tripwire
watches, as the design-entry searches of round 1 found. When the tripwire leaves unfired, the doubt
is answered, and the entry is judged again under `design@agent-skills@losing-alternatives-filter`.
Bears on #tripwire-horizon.

### A goal is standing intent; a tripwire is a hypothesis on one premise `##arg-14`

Round 1, the session, answering `design@agent-skills@goal-lifecycle`: "a goal is your standing
intent, met continuously. A tripwire is a hypothesis about one premise. After the tripwire leaves,
the premise is still written in the decision's head, and every grounding reads it." Bears on
#tripwire-horizon.

### Census v1: the test drops 46 of 62 kept causes and every firing `##arg-15`

Round 2, the session, relaying the census, a subagent's judgement without hindsight on 83 causes
(85 found, 2 voided). Recorded by the owner: 4 record, 46 drop, 12 unclear. Dropped by the owner: 0
record, 18 drop, 3 unclear. The 9 causes whose tripwires later fired were all test drops. The
owner's drops went by plan: 15 of 21 came from 5 premortems dropped whole. A lead, per
`design@agent-skills@synthetic-evidence-not-built`: it reopened the round-2 approval as a material
finding and decided no shape. Bears on #tripwire-admission-test.

### The retrospective intake reads tripwires `##arg-16`

Round 2, the session, correcting its own brief on the census's finding: the installed retrospective
intake reads every tripwire against a received file, its step 3 and its section on a tripwire that
may have fired; 3 of the 9 firings the census found arrived that way. Bears on condition 2.

### Census v2: refining the sub-cases raised the unclear verdicts `##arg-17`

Round 2, the session, relaying census v2 on refined tests: agreement 34 of 83, disagreement 24,
unclear 25, from 15. Recorded by the owner: 25 record, 18 drop, 19 unclear. Dropped by the owner: 6
record, 9 drop, 6 unclear. Of the 9 fired: 1 recorded, 6 unclear, and 2 dropped as requests for what
their decision refused, whose firing added nothing the request did not deliver (hindsight). A lead:
the session withdrew its sub-cases on it. Bears on #tripwire-admission-test.

### Horizons make the two errors unequal, so a doubt keeps the cause `##arg-18`

Round 2, the session: "A wrong keep costs reading until its horizon. A wrong drop loses a signal
for good." Since the unclear verdicts held 6 of the 9 firings, a cause no named fact excludes is
kept with a short horizon. Bears on #tripwire-admission-test.

### The owner's counterfactual test `##arg-19`

Round 3, the owner: "would the facts that made the fired tripwires fire have been found out without
the tripwires in the record ? if yes, that probably means they were not that useful anyway." Bears
on #tripwire-admission-test: it became condition 1.

### The counterfactual: 8 of 49 firings needed the tripwire `##arg-20`

Round 3, the session, relaying the counterfactual, a subagent's hindsight judgement per firing with
its commit. This repository, 12 firings (the census's 9 fired causes and 3 firings with no premortem
ruling): 8 found and acted on anyway, 1 raised to a decision-level outcome by the tripwire, 3 found
only through it, 2 of those 3 false alarms. thaum, 37 firings: 17 found anyway, 4 raised, 6 found
only through it, 4 that changed only the tripwire's own record, 6 misfires of its own wording. So 8
firings needed the tripwire: here 2 (one found only through it, one raised by it), in thaum 6; 12
if thaum's 4 raised firings count. Bears on #tripwire-admission-test.

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
what the decision instructs or endorses (the 3 false alarms: tripwire T4 of the deleted spec of the
agentic-workflow axis, which counted one agent per lens, what auditors are told to do, and
`scope-and-distance` here and in thaum, which measured the behaviour its decision endorses); the
threshold sat inside its instrument's noise (thaum's `t-event-payload`, whose firings 3 to 5 only
moved its own threshold); the wording misfired (6 in thaum). Bears on conditions 1, 3 and 4, and on
the lifecycle's exit when a repair adds a check (thaum's restated standing entries on work the plan
did on purpose, 10 of its 17 found-anyway firings).

### Precision matters more than recall, unless a small refinement admits the useful ones `##arg-23`

Round 4, the owner: "even if it drops a few of the useful ones, i'd keep it, unless a small
refinement can make them pass. [...] the important criteria is rather that accepted tripwires are
valuable, rather than no past useful tripwire stays accepted. the future side is more important than
the past here." Bears on #tripwire-admission-test. The session then decided to run no census v3; the
owner's condition is carried by step 2, which applies the standard to the useful firings before the
fragments are written.

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
decision ranks it, so per `primer@question-authority` the cause is no longer a fork. And the surveys
show the cost of the incumbent: the owner dropped 23 of 66 ruled causes, and about 45 of 64 standing
entries watch a failure caught anyway (`argument@tripwire-standard@arg-5`). Bears on
#tripwire-recording-authority.

### A pointer is delivered only if the session fetches it `##arg-26`

Round 2, the owner, quoted under #tripwire-standard-home; and the session: `argument@tripwire-standard@arg-7`
is consistent with it, since step 7 never sent the session to the rule it broke. Bears on
#tripwire-standard-home and #shared-text-fragment.

### The snippet pass cannot carry prose `##arg-27`

Round 2, the session: a snippet is inlined last, so never stripped of `%%` lines nor filled; every
snippet is listed in `SNIPPETS`, from which `klarch init --rust` writes four into new projects; a
snippet is code a test checks. Bears on #shared-text-fragment.

### A harness hook ties the workflow to one harness `##arg-28`

Round 2, the session, against the owner's second lead in the pointer issue: a hook that delivers
pointed sections ties the shipped workflow to one harness's hook system, against
`goal@agent-skills@installed-text-works-anywhere`. Bears on #shared-text-fragment.

### A build-filled copy answers the drift argument; the host table answers the size ground `##arg-29`

Round 2, the session: `design@agent-skills@restatement-size-test` stands on the owner's size ground,
"avoid loading excessive unrelated informations", and on its own drift argument, "A partial copy
drifts". A copy the build fills cannot drift; a fragment goes only where its act applies the whole
text. And `rejected@agent-skills@directive-restated-everywhere` stays rejected: its reason is nine
inconsistencies between hand-written restatements and their home. Bears on #shared-text-fragment.

### The first audit removes what never deserved an entry; horizons prune after `##arg-30`

Round 1, the session: "horizons do the ongoing pruning, and the audit is only for migration and for
when you ask. That answers your worry that a retirement clause alone may not be enough." Bears on
#tripwire-audit-axis.

### The checker cannot judge a horizon `##arg-31`

Round 1, the session: whether a horizon has passed is an event the checker does not see, and a
presence check would read a field of a heading entry, the kind of syntax
`rejected@core@validity-marker-checked` refused for the `rejected` register. Bears on
#tripwire-shape-unchecked.

### An acceptance criterion gates a merge; a tripwire changes nothing until it fires `##arg-32`

Round 2, the session: "An acceptance criterion gates the merge of work being built: applying it
decides what the branch must build. A tripwire changes nothing until it fires." The owner, round 3,
agreed that "acceptance criteria are an important point were my ruling is useful". Bears on
#acceptance-criterion-authority.

### The two defects of acceptance criteria are conditions 3 and 4 `##arg-33`

Round 3, the session: "a criterion bound to pass checks what the design instructs (condition 3); an
irrelevant bound is a number with no basis (condition 4)." Conditions 1 and 2 do not apply: a
criterion is checked once, by the landing's own work. Bears on #acceptance-criterion-quality.

### Shape (A) is no grant within a boundary `##arg-34`

Round 1, the session: shape (A) "is no grant within a boundary, so it does not bring back
`rejected@agent-skills@delegated-decision-state`. It is `primer@question-authority` applied once the
record ranks the question." Bears on #tripwire-recording-authority.

### The source holds the placeholder, but sessions read the installed copies `##arg-35`

Round 2, the session: "No primer change. content/ holds the placeholder, not a copy, so the source
restates nothing." Defeated after the convergence, by the design-conformance review of this spec:
the installed copies hold the expanded text, sessions and the agentic-workflow audit read the
installed copies, and `primer@where-knowledge-goes` says a directive longer than one sentence "is
never restated". So step 2 adds one sentence to the primer's rule: a copy the build fills from one
home is no restatement. Bears on #shared-text-fragment.

### A fragment sentence is judged in each host's context `##arg-36`

Round 4, the session, premortem cause 2: a fragment sentence true in one host can be false in
another, and a review that reads the fragment file alone sees it in no host. So the agent-text
reviewer reads the installed copy of every host of a fragment the diff changes. Bears on
#shared-text-fragment and AC1.

## New names, in one place

```text
crates/agent-skills/fragments/                       new directory: the fragments
crates/agent-skills/fragments/tripwire-standard.md   new: the standard
crates/agent-skills/fragments/tripwire-lifecycle.md  new: the lifecycle
{{fragment:<file>}}                                  new placeholder line, rendered by the fragment pass
render_fragments                                     new function in src/render.rs: the fragment pass over one text
FRAGMENT                                             new const in src/render.rs: "{{fragment:"
Fragments                                            new struct in build.rs: the files under fragments/, read once, with which ones a placeholder named
**Watches until:**                                   new line of a tripwire entry
crates/agent-skills/content/agents/tripwire-auditor.md       new agent: knowledge-architect-tripwire-auditor
crates/agent-skills/content/workflows/tripwire-audit.js      new saved workflow: knowledge-architect-tripwire-audit
tripwire-axis                                        new section slug of the project-audit skill
docs/audit-reports/<YYYY-MM-DD>-tripwire-axis-pass-<n>.md    the report file of a pass, by the generic name
Snippets, SUBSTITUTIONS, substitute                  existing, build.rs
strip_comments, render_slugs, check_workflow_name    existing, src/render.rs
FILES                                                existing, src/lib.rs
```

## Decided design

### The standard

The text of `planned@agent-skills@fragments/tripwire-standard.md`, as the owner approved it in round
4, with three editorial changes: the second person becomes "the session", the T-labels of the
premortem are gone, and the criterion's paragraph is added per #acceptance-criterion-quality. The
clause "and in the record of the work" is D2. The implementing session may reword it; a change of
what a condition admits is a change of the decision. An illustration of the text, not authority:

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

**A recorded tripwire's wording and scope are the session's**, judged by this standard and listed
in the turn's "Decided" section. Its removal outside the lifecycle is the owner's.

<the entry block and the horizon paragraph of "The entry and its horizon">

**An acceptance criterion meets conditions 3 and 4**: it names a built result under which it fails,
and a bound in it carries its basis. Conditions 1 and 2 do not apply to it: the landing's own work
checks it once.
```

The argument: `argument@tripwire-standard@arg-19` to `argument@tripwire-standard@arg-24`, and, for
the paragraph on a recorded tripwire's wording and scope, the owner's aim in
`argument@tripwire-standard@arg-4`. Condition 1 is the owner's proposal. Nearest rival: the three
tests of round 1 (`argument@tripwire-standard@arg-9`); the owner's counterfactual test replaced them,
and the counterfactual showed silence to be what the useful firings share
(`argument@tripwire-standard@arg-21`).

### The entry and its horizon

```markdown
## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`

**Fires when:** the event, or a count with its basis.
**Response:** reopen the decision by its reference, or open an issue of a named kind.
**Re-entry:** an occasion that holds the evidence.
**Watches until:** the occasions after which the cause, unfired, no longer will.
```

- **What a horizon names.** The first occasions that exercise the decision, named so that a command
  over the history counts them from the commit that wrote the line: commits on the main branch that
  change a named path or the guarded head's text, release tags, audit report files of an axis, plan
  documents deleted at their landing, or a single event the tree shows, such as the first issue
  opened by a consumer of an interface. Never an occasion only a session's transcript holds.
- **Its length** is the writer's judgement, short for a cause kept on a doubt. No count is kept in
  the entry (`argument@tripwire-standard@arg-10`).
- **`standing`** is for a guarantee whose failure stays silent and permanent, with the reason in the
  entry.
- **The standing-state reviewer** finds the commit that wrote the horizon line, counts the occasions
  since with the command the line implies, and reports a horizon passed as it reports a firing.
- **At the first audit** of an existing entry, its horizon counts from the commit that writes it.

The argument: `argument@tripwire-standard@arg-6`, `argument@tripwire-standard@arg-8`,
`argument@tripwire-standard@arg-10` to `argument@tripwire-standard@arg-12`. Nearest rivals: a count
of occasions survived (`argument@tripwire-standard@arg-10`) and an age or version sunset
(`argument@tripwire-standard@arg-11`).

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
times in 250 entries, and the first exclusion of condition 1, a failure that a test or a check
reaches, covers its case. The issue-tracking skill keeps, as host text beside the fragment, its
clause on a firing whose reversal is recorded only at a plan document's harvest, and the rules on
issue entries.

### Who records

At the end of the work that implements a decision (on the full path, the harvest; on the in-change
path, the commit that records the decision), the session judges each premortem cause by the
standard, on its wording then. It writes those that pass, as the standard's entry, in the tripwires
home of the Component that owns the decision, and lists every cause with its outcome in its turn's
"Decided" section and, under D2, in the record of the work: the harvest's commit message, or the
in-change commit message. No cause is put to the owner. The premortem names causes and no `T`
label; the prefix `T` leaves the table of `design@agent-skills@ruled-items-labelled`. A parked
thread's tripwire is judged by the same standard. A recorded tripwire's wording and scope change on
the session's judgement; it leaves only as the lifecycle says or on the owner's word, and an audit's
deletion of an entry goes to the owner, per `design@agent-skills@audit-outcomes`.

The decision-recording skill's section keeps its slug,
`skill@knowledge-architect-decision-recording@ruled-tripwires`, since the released 0.6.0 section of
CHANGELOG.md cites it and does not change; its title becomes one that names the act, such as
"Recording a premortem's tripwires".

Under D1's default, the decision-record reviewer's predicate "Does a tripwire stand on the owner's
word?" becomes: does each tripwire the diff adds meet the standard, and, under D2, does the record of
the work list every premortem cause with its outcome.

The argument: `argument@tripwire-standard@arg-25` and `argument@tripwire-standard@arg-34`. Nearest
rival: shape (B), which halves the owner's rulings rather than removing them, while the standard
ranks every cause it admits as it ranks those it excludes.

### Fragments

- **The placeholder** is a line that is `{{fragment:<file>}}` after any leading spaces, `<file>` a
  file directly under `planned@agent-skills@fragments/`, with its `.md` extension, as
  `{{fragment:tripwire-standard.md}}`. Its leading spaces are its indentation.
- **The pass** is `render_fragments` in render.rs, a pure function over one text and the fragments'
  texts, which returns the rendered text and the names it used. It runs after the workflow-name
  check and before the `%%` strip, the `{{slug:…}}` rendering and `SUBSTITUTIONS`. It removes the
  `%%` lines of the fragment's own text first, then prefixes every non-blank line of it with the
  placeholder's indentation, so a fragment sits inside a list item, as step 7 of the design skill
  needs. A `{{command}}` or a delivery placeholder in a fragment is filled like any text.
- **The build fails** on a placeholder naming no file; on a fragment no placeholder names; on a
  fragment holding a `{{fragment:` line, since a fragment does not nest; and on a fragment holding a
  `{{slug:` placeholder, since the host supplies the heading and its slug. The first, third and
  fourth are raised by `render_fragments`; the second by build.rs over the names each call returned,
  as `Snippets::all_used` does.
- **The fragments directory** is in the package's `include` list, `/fragments/**`, and in a
  `cargo:rerun-if-changed=fragments` line beside the one for snippets/.
- **The checker** reads a fragment as a document of this repository, as it reads content/: its
  references are checked here, and it cites no entry, per `design@agent-skills@shipped-text-cites-no-entry`.
- **crates/agent-skills/CLAUDE.md** states the fragments directory beside snippets/: what it holds,
  that it is in the walk, and that a fragment is edited, never its hosts' installed copies.
- **`primer@where-knowledge-goes`** gains one sentence: in the shipped workflow, a copy the build
  fills from one home into several installed texts is no restatement
  (`argument@tripwire-standard@arg-35`).

The argument: `argument@tripwire-standard@arg-26` to `argument@tripwire-standard@arg-29`. Nearest
rivals: a pointer read whole (`argument@tripwire-standard@arg-26`); a harness hook
(`argument@tripwire-standard@arg-28`).

### The hosts

Each host names the fragment where its act applies the whole text; the table is the owner's,
round 3: "I think the location table is right." D1 adds one row.

| host | its act | fragment |
| --- | --- | --- |
| content/skills/issue-tracking/SKILL.md, section `skill@knowledge-architect-issue-tracking@tripwire-entry` | writing a tripwire | the standard |
| content/skills/issue-tracking/SKILL.md, section `skill@knowledge-architect-issue-tracking@movement-instruction` | moving a tripwire | the lifecycle |
| content/skills/design/SKILL.md, step 7 | proposing causes and criteria | the standard |
| content/skills/decision-recording/SKILL.md, section `skill@knowledge-architect-decision-recording@ruled-tripwires` | recording or dropping causes at the end of the work | the standard |
| content/agents/standing-state-reviewer.md | has it fired; has its horizon passed | the lifecycle |
| content/agents/tripwire-auditor.md | judging each entry | the standard and the lifecycle |
| content/skills/planning/SKILL.md, section `skill@knowledge-architect-planning@acceptance-criteria` | a recurring criterion becoming a tripwire; a criterion's quality | the standard |
| content/agents/decision-record-reviewer.md (D1) | judging each tripwire a diff adds | the standard |

**What each issue-tracking host keeps.** The section on a tripwire entry keeps its first paragraph, its
bullet on the guarding reference and `tripwires --guarding`, the paragraph that every level-two
heading of a tripwires home is a tripwire, and the paragraph naming the standing re-entry point; the
fragment replaces its entry block, its bullets on firing evidence, response and re-entry, and the
paragraph on a script-checkable firing. The section on movement keeps its clause on a
firing whose reversal is recorded only at a plan's harvest and every rule on issue entries; the
fragment replaces its tripwire exits.

**The other texts that state the old rule**, listed under "What the work is", are edited at step 2
to state shape (A) in one sentence each, or to drop the clause. Every other text that names
tripwires lists, searches or counts them, and keeps its sentence.

### Acceptance criteria

- **The owner's word stays**, on its own ground: a criterion decides, before the merge, what the
  branch must build, which a tripwire never does (`argument@tripwire-standard@arg-32`). The rule on
  a criterion proposed after the premortem, marked as awaiting the owner, is unchanged.
- **A criterion meets conditions 3 and 4** of the standard (`argument@tripwire-standard@arg-33`):
  the planning skill's section on acceptance criteria hosts the standard, whose last paragraph says
  so, and step 7 of the design skill hosts it too. The sentence "A number in a criterion is a
  threshold the owner sets" of the planning skill stays, and the proposal carries the basis.
- **A recurring criterion** at its plan document's leaving becomes a tripwire by the standard, not
  on the owner's word.

### The tripwire axis

A third axis of `skill@knowledge-architect-project-audit`, in its section `tripwire-axis`. **It takes
the design-record axis's section, agent and saved workflow as its model, step for step**, with these
differences:

- **The corpus:** every entry of every tripwires home, read with the head it guards, whole, and with
  the history of that head where the tripwire's ground is the owner's.
- **The rules:** the standard and the lifecycle, carried by the tripwire auditor, which hosts both.
- **Not in this axis:** issues; the design heads the tripwires guard, which are the design-record
  axis's.
- **The order of the entries** is the order of `cargo klarch tripwires`, which the preflight saves;
  the groups are cut from it as the design-record axis cuts its own, a Component larger than the
  group size alone and the smaller ones packed whole.
- **The steps of a run**, as the design-record axis's: a preflight; one auditor per group, its brief
  listing its entries and a shared calibration sample; the calibration across groups; one verifier
  per group on every draft; a cross-check that maps the findings touching one entry, an absorption
  and a rewording of the same entry included; the report. The checker step of the design-record
  axis, which judges heads a split creates, has no counterpart: the axis creates no head.
- **Its `args`** are the design-record axis's, `commit`, `scratch`, `worktrees`, `history`,
  `previous`, `kept`, `sample`, `counts` and `from` with `applied`, with `groupSize` its own default
  and no `checkersPerGroup`.
- **The verification of a report, the application run and the old-and-new-text rule** are the
  design-record axis's, read over tripwire entries.
- **The draft of an entry** names its verdict, the condition or exit it rests on, the edit it
  proposes, and, for a kept entry, its horizon.
- **The outcomes of this axis:**

| outcome | when | where it goes |
| --- | --- | --- |
| kept | it meets the standard | applied in the branch: a horizon is written, and a `T<n>` label or a ground "on the owner's word" is reworded to the entry's own premise |
| reworded | a slip struck, a response brought to the shape, a number given its basis or replaced by the event | applied in the branch: wording and scope are the session's |
| converted | a script could compute its firing | put to the owner: the entry leaves once the check exists, written in the branch where it is small, else a `todo` issue |
| absorbed | another entry guards the same premise | put to the owner: the entry leaves, its clause folded into the other |
| removed | it fails condition 1, 2 or 3 | put to the owner |
| its decision is gone | the head it guards no longer exists | put to the owner |
| horizon passed | from a re-check on, the horizon's occasions have passed | put to the owner |
| fired | its evidence is in the tree | the session judges it; the deletion and its issue go on the owner list, per the lifecycle |

Every outcome that deletes an entry reaches the owner, grouped by its outcome and, for a removal, by
the condition failed, per #removals-grouped-by-condition.

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

### From the lifecycle's exits to the tripwire axis's outcomes

| lifecycle exit | axis outcome |
| --- | --- |
| it fires | fired |
| its horizon passes unfired | horizon passed |
| its decision is reversed | its decision is gone |
| another entry guards the same premise | absorbed |
| a firing's repair adds a check | converted, where the check is written without a firing |
| none: the entry fails the standard | removed |
| none: the entry meets it | kept, or reworded |

### From the current premortem labels to the new record

| today | after |
| --- | --- |
| a cause put to the owner as `T<n>` | a cause named in the premortem, no label; judged at the end of the work |
| the owner's ruling on `T<n>` | the session's outcome per cause in "Decided" and, under D2, in the record of the work |
| an acceptance criterion as `AC<n>` | unchanged |

## Losing alternatives

| alternative | lost to | the fact |
| --- | --- | --- |
| the falsifiability bar alone, with the owner filtering | #tripwire-admission-test | the owner dropped 23 of 66 ruled causes, and about 45 of 64 standing entries watch a failure caught anyway (`argument@tripwire-standard@arg-5`) |
| the three tests of round 1 | #tripwire-admission-test | replaced by the owner's counterfactual test; the counterfactual showed silence to be what the useful firings share (`argument@tripwire-standard@arg-21`) |
| the sub-cases of census v2 | #tripwire-admission-test | withdrawn by the session on census v2, a lead (`argument@tripwire-standard@arg-17`) |
| a count of occasions survived, kept in the entry | #tripwire-horizon | churn, and wrong when a session forgets (`argument@tripwire-standard@arg-10`) |
| an age or version sunset | #tripwire-horizon | no evidence for a period (`argument@tripwire-standard@arg-11`) |
| the owner rules on each cause | #tripwire-recording-authority | the standard ranks the cause, so it is no fork; the incumbent's cost (`argument@tripwire-standard@arg-25`) |
| shape (B): admitted causes still put to the owner | #tripwire-recording-authority | the standard ranks what it admits as it ranks what it excludes (`argument@tripwire-standard@arg-25`) |
| a pointer to the issue-tracking section, read whole | #shared-text-fragment | a pointer is delivered only if fetched (`argument@tripwire-standard@arg-26`) |
| a harness hook | #shared-text-fragment | it binds the workflow to one harness (`argument@tripwire-standard@arg-28`) |
| the snippet pass reused for prose | #shared-text-fragment | snippets are inlined last, unprocessed, and written by `init` (`argument@tripwire-standard@arg-27`) |
| a presence check of the horizon line | #tripwire-shape-unchecked | the checker cannot judge the horizon (`argument@tripwire-standard@arg-31`); the owner kept it out for now |
| an acceptance criterion following shape (A) | #acceptance-criterion-authority | a criterion gates the merge (`argument@tripwire-standard@arg-32`) |
| census v3 before convergence | #tripwire-admission-test | the session's decision: the owner ranked precision over recall, and step 2 carries the owner's condition on the useful firings (`argument@tripwire-standard@arg-23`) |

Which of these earn a `rejected` entry is the harvest's, per `design@agent-skills@losing-alternatives-filter`.
The reversed `design@agent-skills@premortem-tripwires-on-the-owners-word` does, as
`owner-rules-each-tripwire`, on `argument@tripwire-standard@arg-25`: its refutation rests on the
surveys, evidence that cost work. The three tests of round 1 and the sub-cases of v2 earn none: what
refuted them is the census, a lead that decides no shape, and the owner's replacement of them, which
the deliberation in this spec and the harvest's commit message keep.

## Readings

Empty: the work reads no external specification.

## Premortem

| # | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| T1 | sessions read "silent" too widely and drop a cause whose failure later shows silent | #tripwire-admission-test | a tripwire: it meets all four conditions; recorded by the session at the harvest, the owner having asked in round 5 whether it was the session's call and the session having confirmed; judged again at the harvest on its wording then |
| 2 | a fragment reads true in one host and false in another, and no review reads it in each host's context | #shared-text-fragment | AC1, on the owner's word, round 5: "apply AC1 and AC2" |
| 3 | the first audit's owner list is too long to rule on, and its deletions stall | #tripwire-audit-axis | AC2, on the owner's word, round 5 |
| 4 | writers default to long horizons or to `standing`, and volume does not fall | #tripwire-horizon | dropped: it fails condition 1, since the audit pass shows it and the auditor reads every `standing` entry |
| 5 | sessions record many marginal tripwires, and the owner's load moves from ruling to reviewing | #tripwire-recording-authority | dropped: it fails condition 2, since only the owner sees it, and the owner overrules |
| 6 | criteria still carry bounds with no basis | #acceptance-criterion-quality | dropped: it fails condition 1, since the owner rules on every criterion |

T1, written at the harvest in `register@agent-skills@tripwire`, guarding the head of
#tripwire-admission-test:

- **Fires when:** an issue is opened on a silent failure of a decision, and a commit message or a
  plan document of the work that recorded that decision lists the cause as dropped under condition 1.
- **Response:** reopen the admission test, on how condition 1 is worded.
- **Re-entry:** the standing-state review: the issue and the commit are in the tree and the history.
- **Watches until:** the fifth premortem recorded in this repository after this work lands: a plan
  document's Premortem section, or an in-change commit message listing premortem causes. The length
  is the session's judgement, per "The entry and its horizon".

## Acceptance criteria

### AC1: every host's installed copy reads true with its fragments expanded `##fragments-read-in-their-hosts`

- **Guards:** #shared-text-fragment.
- **Judged at:** the review of the work, step 4: the agent-text axis is briefed to read every host's
  installed copy under .claude, the fragments expanded, beside the source.
- **Fires when:** a sentence of a fragment is false, or names an act the host's reader cannot
  perform, in a host's context: for example "The session that implements the decision judges each
  cause at the end of that work" read in the tripwire auditor, which implements no decision.
- **Response:** reopen #shared-text-fragment on what a fragment may hold, and split the fragment or
  move the sentence to the hosts that need it. Whatever this criterion's outcome, step 2 writes into
  `agent@knowledge-architect-agent-text-reviewer` that it reads the installed copy of every host of
  a fragment the diff changes (`argument@tripwire-standard@arg-36`).

### AC2: the first pass's owner list groups every proposed deletion by its outcome and condition `##removals-grouped-by-condition`

- **Guards:** #tripwire-audit-axis.
- **Judged at:** step 5, the first pass.
- **Fires when:** the report's owner list holds a deletion not in a group of its outcome, a removal
  not in a group of the condition it fails, or a group without a default.
- **Response:** reopen #tripwire-audit-axis on how the report is shaped, and repair the saved
  workflow's report step before the owner rules.

## Implementation sequence

### Step 1: the fragment pass, empty

Builds `render_fragments` and `FRAGMENT` in render.rs, `Fragments` in build.rs, the pass's call
before the `%%` strip, its four build failures, the `include` entry, the `rerun-if-changed` line,
and the paragraph of crates/agent-skills/CLAUDE.md. No content file uses a placeholder yet, and the
fragments directory is absent or empty, which `Fragments` accepts as `Snippets` accepts an absent
snippets/.

- **Claims:** a placeholder line is replaced by its fragment's text; an indented placeholder
  indents every non-blank line of it; the fragment's `%%` lines are absent from the result; each of
  the three failures `render_fragments` raises is raised. **Tests:** unit tests of `render_fragments`
  in render.rs, one per claim, each shown to fail by a mutation of the function (the replacement
  skipped, the indentation dropped, the strip skipped, each refusal removed), per
  `skill@klarch-development`. The fourth failure, an unused fragment, is an assertion of build.rs,
  like the unused snippet's, which every build runs.
- **Fails alone on:** an installed copy that differs from the copy the build wrote before the step,
  since no content file holds a placeholder yet.

### Step 2: the useful firings, then the standard, its hosts and every text that states the old rule

First, the owner's condition of round 4: the session applies the standard, as "The standard"
illustrates it, to each of the useful firings, from the entry's text when it was written and the
decision it guarded, without hindsight. Each one the standard drops is checked for a small
refinement that would admit it without admitting the firings the counterfactual found acted on
anyway. A refinement found changes what a condition admits, so it goes to the owner before the
fragment is written; none found is reported in the step's commit with each firing's verdict.

Then the two fragments and every host edit of "The hosts"; the rewrite of
`skill@knowledge-architect-decision-recording@ruled-tripwires` to "Who records";
`skill@knowledge-architect-issue-tracking@tripwire-entry` and
`skill@knowledge-architect-issue-tracking@movement-instruction` as "What each issue-tracking host
keeps" says; step 7 of the design skill; the other texts listed under "What the work is" as stating
the old rule; the primer's tripwire row of the knowledge table, its tripwire example in
`primer@labelled-items`, and its sentence on fragments in `primer@where-knowledge-goes`; the
agent-text reviewer's reading of hosts; the `%%` lines listed under "What is already decided",
and a new one citing `design@agent-skills@acceptance-criteria-in-the-document`. Then
`cargo klarch install-agent-skills`.

- **Claims:** no shipped text states that a tripwire is written on the owner's word, puts a cause to
  the owner under `T<n>`, or reserves to the owner a rewording that changes a tripwire's cause; every
  host's installed copy holds its fragment's rendered text once.
  **Tests:** the first is a corpus claim, tested by reading every line that
  `grep -rn -i tripwire crates/agent-skills/content crates/agent-skills/fragments` returns at the
  step's commit, per `skill@knowledge-architect-planning@working-a-slice` point 3, since a phrase
  search alone misses a phrase split over a line break; the second by a test over `FILES` in
  src/lib.rs, which compares each host's text with `render_fragments` applied to its source, shown to
  fail by removing one placeholder.
- **Fails alone on:** an installed copy that cites a section the edit retitled without its slug.

### Step 3: the tripwire axis

Builds the tripwire auditor, the tripwire audit, and the section `tripwire-axis` of the project-audit
skill, as "The tripwire axis" states, taking the design-record axis's three files as the model.
Reinstall.

- **Claims:** the workflow parses (`node --check` on its body wrapped as crates/agent-skills/CLAUDE.md
  says); its `meta.name` matches its file stem, which the build asserts; the section names the agent
  by its checked agent reference and the workflow by its name in backticks, as the
  other two sections do. **Tests:** the build, and `cargo klarch check`.
- **Fails alone on:** a preflight whose saved listing has fewer rows than `cargo klarch tripwires`
  prints, observed at step 5's run.

### Step 4: the review of the work

Per `skill@knowledge-architect-review`, on every axis its table owes; the agent-text axis briefed
for AC1, which is judged here and reported in the commit that answers the review.

### Step 5: the first pass of the tripwire axis on this repository

Per `skill@knowledge-architect-project-audit@audit-method`, on this branch by the owner's word of
round 5: the tripwire audit runs; the session verifies the report; the owner rules on the owner
list, grouped as AC2 says, which is judged here; the report file is committed with every `Outcome`
line, per `skill@knowledge-architect-project-audit@record-of-an-audit`, before any edit of the pass
is applied; the session applies the pass and runs the application judgement; a re-check pass follows
as the method says, and the audit stops on the owner's word. The severity issue and the
eight-responses issue close here.

### Step 6: the harvest, the changelog and the spec's leaving

Per "Harvest". The harvest's commit message lists causes T1 to 6 of this spec's Premortem with their
outcomes, and every premortem cause of the work is in it. Then the review before the merge, whose
briefs name each report file step 5 added and the commit that holds it, per
`skill@knowledge-architect-project-audit@audit-method`. Then the spec's deletion in a commit of its
own.

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
hosts the standard and judges each added tripwire by it. This adds a host to the table the owner
approved, so it is the owner's. Bears on #shared-text-fragment and #tripwire-recording-authority.

**Ruled**: accepted, the owner, after the spec's reviews: "D1 and D2 accepted, proceed to
implementation of the whole work".

### D2: the record of the work lists every premortem cause with its outcome

The standard the owner approved in round 4 lists every cause "in its turn's 'Decided' section". A
turn's sections reach no durable record, and a review before the merge reads the branch, not the
turns. **Default:** the standard says "in its turn's 'Decided' section and in the record of the
work", the harvest's or the in-change commit message, and, under D1, the decision-record reviewer
checks it. This adds an obligation to the text the owner approved, so it is the owner's. Bears on
#tripwire-recording-authority.

**Ruled**: accepted, in the same words as D1.

## Harvest

| what | where | when |
| --- | --- | --- |
| #tripwire-admission-test | a head in `register@agent-skills@design`, under the thread's slug | step 6 |
| #tripwire-horizon | a head in `register@agent-skills@design`, under the thread's slug | step 6 |
| #tripwire-recording-authority | a head under the thread's slug, absorbing `design@agent-skills@tripwire-wording-is-the-agents` and `design@agent-skills@premortem-tripwire-written-at-harvest`, both deleted; `design@agent-skills@premortem-tripwires-on-the-owners-word` moves to `register@agent-skills@rejected` as `owner-rules-each-tripwire` | step 6 |
| #shared-text-fragment | a head under the thread's slug; `design@agent-skills@restatement-size-test` and `design@agent-skills@content-mirrors-the-install-layout` rewritten | step 6 |
| #tripwire-audit-axis | a head under the thread's slug; `design@agent-skills@design-record-axis` rewritten | step 6 |
| #tripwire-shape-unchecked | a head of absence in `register@core@design`, attributed to the owner with "for now" | step 6 |
| #acceptance-criterion-authority | `design@agent-skills@acceptance-criteria-on-the-owners-word` rewritten on its own ground; the pair stated in the commit message | step 6 |
| #acceptance-criterion-quality | a head in `register@agent-skills@design`, under the thread's slug; `design@agent-skills@acceptance-criteria-in-the-document` rewritten | step 6 |
| the other rewrites of "What is already decided" | in place, `design@agent-skills@decisions-reported-to-the-owner` and `design@agent-skills@standing-entries-searched-before-the-work` among them | step 6 |
| T1 | `register@agent-skills@tripwire`, guarding the head of #tripwire-admission-test | step 6 |
| the issues of "What is already decided" | closed or rewritten as listed | the severity issue and the eight-responses issue at step 5; the others at step 6 |
| CHANGELOG.md, `Next release` | Workflow entries for the standard, the horizon, who records, fragments and criteria; New features for the tripwire axis; a Migration entry: every tripwire gains a horizon and meets the standard, by running `skill@knowledge-architect-project-audit` on its tripwire axis; then `cargo x changelog` | step 6 |
| this spec | deleted in a commit of its own | after the review before the merge |

## Later consequences

- **Other standards move to fragments**: the `todo` the pointer issue becomes. Its first candidate,
  named in it, is the retrospective intake's pointers to the report format.
- **The retrospective watches the tripwires on session behaviour**: the rest of the agent-behaviour
  issue. Condition 2 already counts a received retrospective file as a place tripwires are read.
- **thaum's first pass** runs at its next pin move, under the Migration entry.
- **The issue half of "standing state as a whole"** stays in the audit-axes issue, and may share
  this axis's preflight.
