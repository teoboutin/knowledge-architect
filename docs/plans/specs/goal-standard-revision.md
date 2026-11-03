# The goal standard asks for wide outcomes that name no built part, the goal reviewer asks the owner about reach instead of narrowing, and a goals home cites only goals and registers of its own or an enclosing Component

## Status and audience

This spec is the plan of the work that revises how a goal is written and reviewed. The work does
four things:

- the goal standard, `path@agent-skills@fragments/goal-standard.md`, is rewritten: a goal states an
  outcome that keeps its meaning in a project emptied of its design and its code; its rule of one
  reading judges ambiguity, never breadth; it reads a goal in three layers, what it requires, what
  it admits and what it rules out; and it sorts an outcome judged in the tree from one judged in use;
- `skill@knowledge-architect-goal-setting` drafts goals that are expected to be unmet, writes "for
  instance" cases per layer into each goal, shows the owner the reach of a draft by layer, and stops
  its reviews on a count of the findings left on wording the previous round also read;
  `agent@knowledge-architect-goal-reviewer` grades its findings `blocks`, `asks` or `minor`, and asks
  the owner about a goal's reach instead of proposing a narrower wording;
- the checker refuses, in a goals home, every reference that is not a goal or a register's home of
  the home's own Component or of a Component that encloses it;
- the goals of this repository lose, by deletion alone, the citations that check refuses, and are
  then brought to the new standard by rounds of the review of the whole goal set.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed, rewritten or departed from, the design home wins, and the disagreement is a defect of
  this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner's word is awaited,
  `skill@knowledge-architect-planning@working-a-slice` says what waits.
- **The work is one spec and one branch, carried in the session that converged, on the owner's
  word.** The owner, round 4: "The work should be one spec and one branch carried in this session."
  Step 1 makes the checker stricter, so its commit is the branch's first commit, per
  `primer@branch-commits`, and this spec lands after it. That departs from
  `design@agent-skills@plan-landing-is-not-tied-to-its-work`; the owner, round 5, on Q3: "this is an
  exception, keep it."
- **The branch merges only after rounds of the review of the whole goal set.** The owner, round 4:
  "After implementation, we will run the full review before merge, but not merge, and instead proceed
  to a full set goal review." Round 5, on Q1: "merge can only happen after we fix this under multiple
  rounds of the whole set review."
- **A session that did not take part in the discussion** owes the design audit of
  `skill@knowledge-architect-planning@working-a-slice` point 2 before step 2. Its amendments land in
  a commit of their own after this spec's, since the spec exists only after step 1's commit.

The discussion ran in one session. It begins at the owner's message "I would like to rediscuss the
goal standard and goal setting skill. We are splitting the project in smaller components, setting up
new goals in the meantime, and I am observing various issues."

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, taken once for the whole spec, as its
paragraph "The work of a spec" says.

## Names

| name | what it names |
| --- | --- |
| the goal standard | `path@agent-skills@fragments/goal-standard.md`, the fragment hosted by `skill@knowledge-architect-goal-setting@goal-or-not` and `agent@knowledge-architect-goal-reviewer@the-goal-standard` |
| rule 1 to rule 8 | the numbered rules of the goal standard, in its order |
| the boundary block | what `skill@knowledge-architect-goal-setting@drawing-out-intent` step 5 shows the owner beside each drafted goal |
| a goals home | the home of the `goal` register of an anchor, `register@*@goal`: a file, or a directory behind a `README.md` |
| a draft-mode round | one dispatch of the goal reviewer in draft mode over a batch, per `skill@knowledge-architect-goal-setting@drawing-out-intent` step 4 |
| a whole-set round | one round of `skill@knowledge-architect-goal-setting@whole-set-review` |
| old wording | in a round after the first, the words of a goal that the previous round of the same review read and that no repair changed since; every other word is new wording |
| direction | what a goal asks agents to move the project toward; two readers who would act differently in one situation read two directions |
| a divergence | two goals that can both be met, but not under a condition the project or its use makes possible, after `path@agent-skills@docs/grounding/goal-standard.md`, section "When two goals conflict" |
| the reference check | `judge_part` in `path@core@src/check/references.rs`, which judges every span of every document and of every commit message |
| the mechanical edit | step 1's removal of the citations the new check refuses from this repository's goals homes, by deletion of words alone |
| S1 to S5 | the survey items put to the owner in round 2: S1, Gilb's inspection and the count on old wording; S2, a separate layer of measures; S3, a KAOS divergence; S4, an accepted risk in a goal; S5, the sort of an outcome by what the project does not control |
| the checkpoint table | the table of round 3's reply, every thread of the discussion with its state and its position, which the section Threads holds, closed by the owner's "agreed on the table" in round 4 |
| finding 4 | the record search's finding, round 2, that the three layers reclassify existing records, #arg-21 |
| Q1 to Q5 | the questions put to the owner after the plan review, answered in round 5 |
| main's commit | `59ac0c32`, the tip of the main branch when the spec was written, whose tree the line numbers below name |

## What the work is

**The goal standard at main's commit.** Eight rules, hosted by two texts. Rule 1 makes a kind of
interface an outcome and its exact form a decision. Rule 2 asks one reading, and that every "every",
"no", "all" and "not in scope" names the set it ranges over. Rule 3 asks that a goal rule out nothing
the owner has not ruled out knowingly. Rule 5: "No state of the project meets one goal only by
failing another. Where two outcomes compete, a goal states which ranks first." Rule 6 asks that what
shows a goal met be "an observable of the tree as it stands", and, for an outcome judged only in use,
the mechanism the project builds to pursue it and learn from use. No rule limits what a goal names
or cites.

**What the discussion found the standard produces.** Every repair a finding asks adds words: rule 2
adds the set a word ranges over, rule 3 an exception, rule 6 the most specific observable. A
narrowing repair of a rule-3 finding rules out cases the owner never ruled out, which is a new rule-3
finding at the next round. The root goals home holds 19 goals in 6215 words at main's commit, taken
with `git show 59ac0c32:docs/goals.md | wc -w` and `grep -c '^## '`.
`a-goal-the-draft-review-read-was-reworded-again-for-a-defect` records two goals
that reached the tree with a defect the standard names, and three instances of the draft-mode loop,
one batch reviewed five times with 8, 10, 6, 6 and 6 findings. `design@agent-skills@whole-set-rounds`
records the non-convergence of the review of the whole goal set: 91 then 88 findings.

**The goal reviewer at main's commit.** `path@agent-skills@content/agents/goal-reviewer.md` tags
each finding `settled`, `intent` or `contradiction`, and also `minor` for a finding of rule 4 or of
rule 6 but one case. It holds no `%%` line and no stop.

**The goal-setting skill at main's commit.** `path@agent-skills@content/skills/goal-setting/SKILL.md`.
Its boundary block shows, for each universal or exclusion, at least one case it rules out and one it
admits, and is never written to the goals home. Its section "The shape of a goal entry" gives a
template with no example lines, and says "any goal can be referenced from anywhere", with a `%%`
comment saying "a reference reaches a goal of any Component from anywhere". Its draft-mode review has
no stop. Its review of the whole goal set stops on the owner's word or when a round's count of
findings that are not `minor` falls to a fifth of the first round's.

**The checker at main's commit.** The reference check resolves every reference and judges no
direction: `design@core@no-citation-direction-rule`. `Anchors::owning` and `Anchors::home` in
`path@core@src/entity.rs` give a file's owning anchor and an anchor's home of a register. The
`references` row of `PLANTED` in `path@core@src/mock_projects.rs` counts 27 planted findings.

**This repository's goals homes at main's commit.** The citations the new check refuses, found with
`grep -noE` over the seven goals homes for every backticked span that is not a goal or a register's
home of the home's own or an enclosing Component:

| goals home | line of the span | span |
| --- | --- | --- |
| `register@knowledge-architect@goal` | 19 | `design@agent-skills@goal-lifecycle` |
| `register@knowledge-architect@goal` | 87, 138 | `goal@agent-skills@one-skill-per-activity` |
| `register@knowledge-architect@goal` | 109, 133, 420, 487 | `goal@agent-skills@only-needed-instructions` |
| `register@knowledge-architect@goal` | 132, 142 | `goal@agent-skills@no-contradiction-in-workflow` |
| `register@knowledge-architect@goal` | 156 | `goal@gates@gates-from-a-list` |
| `register@agent-skills@goal` | 11 | `skill@knowledge-architect-agent-configuration@where-text-goes` |

No mock project's goals home under `path@core@tests/projects/` holds a reference. Every register
reference in a goals home names a register of its own Component or of the root; the root's home
holds none.

**Texts that cite the standard's rules by number**, found with `grep -rnE "\brules? [1-8]\b"`:
`path@agent-skills@content/skills/planning/SKILL.md`, `path@agent-skills@content/agents/transcript-reviewer.md`,
`path@agent-skills@content/agents/goal-reviewer.md`, the goal-setting skill and the fragment itself;
in the record, `register@agent-skills@design`, `register@agent-skills@rejected`, the issue on the
draft-mode loop and `issue@agent-skills@a-target-project-collects-no-feedback-from-use`. The
numbering is kept, so these stay true, per `issue@core@no-checked-form-cites-a-point-inside-a-section`.

**Outside the work:**

- what setup asks of a target project for feedback from use:
  `issue@agent-skills@a-target-project-collects-no-feedback-from-use`, a design session of its own;
- each goal of this repository brought to the new standard, other than by deletion: step 5, under
  goal-setting, with the owner's ruling on each goal, after the spec has left.

## What is already decided

**The decisions the design rests on, not argued again:**

- `design@agent-skills@shared-text-fragment`: the standard stays one fragment with its two hosts;
  text only one host applies, the grades and the stops, stays outside the fragment.
- `design@agent-skills@goal-set-review-in-goal-setting`, `design@agent-skills@goal-batch-ruled-in-the-tree`
  and `design@agent-skills@goal-lifecycle`.
- `design@agent-skills@shipped-text-cites-no-entry` and `design@agent-skills@shipped-text-line-comments`.
- `design@core@nothing-of-a-project-is-compiled-in`, `design@core@model-then-checks`,
  `design@core@phases-gate-the-report`, `design@core@finding-names-the-repair`,
  `design@core@the-regime-has-no-opt-out`, `design@core@one-entity-table` and
  `design@core@an-entity-belongs-to-its-anchor`: the new check is a phase-4 check over the entity
  table and the anchors, the same in every project, whose finding names its repair.
- `design@agent-skills@plain-text-is-no-repair`: the new finding's repair is a rewritten sentence or
  a cited goal, never the span with its backticks removed.
- `design@knowledge-architect@versioning-policy` and `design@knowledge-architect@changelog-entries`:
  a check added is a minor change; a change of what recorded goals must meet owes a Migration entry
  citing `skill@knowledge-architect-goal-setting@whole-set-review`, per `design@agent-skills@audit-triggers`;
  a change to something added since the last release edits the entry that announces it.

**The decisions the work departs from, for this work alone, on the owner's word:**

- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: this spec lands after step 1, on its
  branch; the owner, round 5, on Q3: "this is an exception, keep it."
- `design@agent-skills@goals-change-through-goal-setting` and `design@agent-skills@goal-edit-ruled-alone`:
  the mechanical edit changes goals outside goal-setting, with no draft and no review; the owner,
  round 4, quoted under #goal-standard-rereview-here. Two of its deletions change what a goal asks,
  at main's lines 108 and 486; the owner, round 5, on Q1: "Q1 default, merge can only happen after
  we fix this under multiple rounds of the whole set review."

**The decisions the work reverses or rewrites, with every text that references them:**

| decision | change | texts referencing it, and who updates them |
| --- | --- | --- |
| `design@core@no-citation-direction-rule` | narrowed to "outside a goals home" | none; step 1 rewrote it |
| `design@agent-skills@goal-standard-rules` | rewritten to the new rules | the heads `design@agent-skills@goal-entry-shape`, `design@agent-skills@goal-reviewer-agent` and `design@agent-skills@shared-text-fragment`, and the grounding document: the harvest; `issue@agent-skills@a-target-project-collects-no-feedback-from-use`, whose summary restates rule 6: the harvest rewrites that sentence |
| `design@agent-skills@goal-reviewer-agent` | the grades | the goal-setting skill's `%%` comment above its step 4: step 2; `design@agent-skills@goals-axis-in-the-review-before-a-merge`, the grounding document: the harvest; the issue on the draft-mode loop, closed at the harvest |
| `design@agent-skills@goal-draft-shows-reach` | the block by layer; "The block never reaches the goals home" reversed for the "for instance" cases | the skill's `%%` comment above its step 5: step 2; the issue on the draft-mode loop, closed at the harvest |
| `design@agent-skills@whole-set-rounds` | the stops on old wording | the skill's `%%` comment of its whole-set section: step 2; `design@agent-skills@goal-reviewer-agent`, `design@agent-skills@goal-set-review-in-goal-setting` ("a stop on a count", still true), the grounding document: the harvest; the issue on the draft-mode loop, closed |
| `design@agent-skills@goal-entry-shape` | the body carries "for instance" cases by layer | the skill's `%%` comment on the entry shape: step 2; the harvest |

**The record search of the discussion.** It ran with the eleven drafted shapes as its work and these
seeds: `design@agent-skills@goal-standard-rules`, `design@agent-skills@goal-reviewer-agent`,
`design@agent-skills@goal-draft-shows-reach`, `design@agent-skills@whole-set-rounds`,
`design@agent-skills@goal-entry-shape`, `design@agent-skills@goal-batch-ruled-in-the-tree`,
`design@core@an-entity-belongs-to-its-anchor`. Its check, `cargo klarch worklist --verify`, wrote 116
entries graded `bears` and 97 graded `may-bear`, in a scratch directory outside the tree: every entry
below is named with its outcome or its reason, so the spec holds what the scratch files held.

Graded `bears`, with their outcome:

- **Every goal of this repository**: may change, through step 5. Those that constrain the work:
  `goal@knowledge-architect@the-owner-decides` and `goal@knowledge-architect@serves-one-kind-of-project`,
  under which the new check, an ask binding every target project, is knowledge-architect's owner's to
  add, which the owner did in round 4; `goal@agent-skills@only-needed-instructions`, which forbids "a
  fixed sequence, count or mapping between a unit of the work and a unit of its record that another
  instruction needs to vary", so a goal carries one or more "for instance" cases for each layer it
  has, not a fixed count; `goal@knowledge-architect@agents-work-without-drift`, which ranks "a goal's
  exclusion that names the design it excludes" above itself, in agreement with "the exclusion holds";
  `goal@knowledge-architect@the-workflow-improves-through-real-use`, whose evidence for this work is
  the real use the issue on the draft-mode loop and `design@agent-skills@whole-set-rounds` record;
  `goal@core@relocation-is-one-manifest-edit`, which the new check strains, see #arg-34.
- `rejected@core@citation-direction-checked`: may bring back; not brought back. It rejected a one-way
  rule over every citation, whose gain was acyclicity for the record search's clusters, and which
  would fail goal-to-goal refinements. The new rule covers goals homes alone, for another reason, and
  admits a goal citing a goal it refines.
- `rejected@agent-skills@goal-measured-by-evidence-from-use`: may bring back; not brought back: rule
  6 keeps the mechanism the project builds as the measure, not the evidence.
- `rejected@agent-skills@owner-intent-entry-test`: may bring back; not brought back: the work moves
  no intent out of the goals, and its doubt, goals homes swelling with local preferences, is answered
  by the skeleton test, which removes the words that tie a goal to a mechanism.
- `design@agent-skills@heads-stand-on-argument`: may change; no change: an intent that states an
  outcome still goes to a goal, and the skeleton test bounds the goal's words, not the intent it holds.
- `design@agent-skills@searcher-grades-goals`: may change; its cost premise, goals homes small beside
  design homes, is taken again at step 5 with `wc -c` over the goals homes against the design homes,
  since the "for instance" cases lengthen the goals.
- `design@agent-skills@a-reference-claims-a-revisit`: no change: under the skeleton test a goal never
  has to be revisited when a design head or a skill changes, so the primer's rule asks for no such
  reference.
- `design@agent-skills@transcript-reviewer-agent` and `design@agent-skills@misstated-ruling-rated-by-what-it-changes`:
  they already rate a goal recorded stricter than the owner ruled out knowingly as major, which the
  work supports.
- `design@agent-skills@intermediate-component-occasion`: a nested Component's goal refines its
  intermediate Component's, which the upward citation rule admits.
- `design@core@a-commit-message-is-a-document` and `design@agent-skills@branch-commits-judged-at-tip`:
  they place step 1 first on the branch.
- `the-goal-standard-lets-a-goal-name-what-is-built-or-cite-an-entry`: closed by
  the work, at the harvest.
- `a-goal-the-draft-review-read-was-reworded-again-for-a-defect`: closed by the
  work, at the harvest: the reach is shown by layer, and both reviews stop.
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: worsens unless the new
  finding names its repair: step 1's claims C4 and C7.
- `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`: worsens: the Migration entry sends a
  consumer through a goal reviewer whose definition changes in the release, which a session that ran
  the install may hold at its old definition. Left as it is: the work adds an instance, named in the
  harvest's commit.
- `issue@agent-skills@a-comment-line-may-copy-the-argument-of-the-head-it-cites`: touches: step 2
  rewrites the comment above the goal standard in the skill's source, and writes it as a pointer to
  the head with no copy of its argument.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: touches: steps 2 and 3 edit shipped
  text, which only the agent-facing-text review checks for a citation of this repository's entries.
- `issue@core@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`,
  `issue@core@a-component-states-at-least-one-goal` and
  `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`: touch the
  goals step 5 reviews; step 5 reads them. The third also lists the asks the installed text states;
  the new check is one, which the Migration entry states.
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: the rules keep their numbers.
- Every other entry graded `bears` constrains how installed text, the checker or the changelog is
  written, and is applied by the step that writes it: the heads of the admission standard
  (`design@agent-skills@statement-names-an-act`, `design@agent-skills@capabilities-not-structure`,
  `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@root-cause-repair`,
  `design@agent-skills@audit-net-growth`, `design@agent-skills@repairs-read-against-the-record`,
  `design@agent-skills@term-defined-at-first-load`, `design@agent-skills@admission-rules-home`,
  `design@agent-skills@instruction-record-is-minimal`, `design@agent-skills@review-judges-additions`,
  `design@agent-skills@restatement-size-test`, `design@agent-skills@context-cost-of-agent-text`), the
  heads on heads (`design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@title-states-the-rule`,
  `design@agent-skills@one-decision-per-head`, `design@agent-skills@existing-heads-on-touch`,
  `design@agent-skills@design-home-write-loads-recording`, `design@agent-skills@thread-slug-is-entry-id`),
  the record heads (`design@agent-skills@goals-bind`, `design@agent-skills@questions-go-to-their-authority`,
  `design@agent-skills@fork-choice-in-weak-form`, `design@agent-skills@goals-required`,
  `design@agent-skills@checker-model-in-primer`, `design@agent-skills@audit-triggers`), the checker
  heads (`design@core@section-homes-carry-slugs`, `design@core@grammars-not-prefixes`,
  `design@core@an-entry-is-a-heading-at-the-register-level`, `design@core@owned-namespace-check`) and
  the release heads (`design@knowledge-architect@the-changelog-ships-in-every-crate`,
  `design@knowledge-architect@the-branch-writes-its-changelog-entries`,
  `design@knowledge-architect@capability-cost-before-migration`).

Graded `may-bear` and read, with whether each bears:

- `design@agent-skills@convergence-count-scope`: bears: its mechanism, repairs writing new text that
  agents find defects in, grounds the count on old wording.
- `rejected@agent-skills@repair-defects-left-to-next-pass`: bears: the stop leaves findings on new
  wording to the owner rather than to a further round, which this entry's cost, a whole pass per
  finding on repaired text, argues for.
- `rejected@agent-skills@two-readings-left-to-judgement`: bears on nothing: it concerns an
  instruction's readings, not a goal's.
- `issue@mutate@a-mutation-of-several-files`, `design@mutate@one-file-per-run` and
  `issue@git@tools-run-git-past-the-wrapper`: bear: each reads what a goal "admits" or "can" as what it
  requires; under the three layers that reading is asked of the owner at step 5, which reads all
  three. The owner accepted that existing records are reclassified then, in round 3.
- `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`:
  bears on nothing: it concerns when a default enters a head.

Graded `may-bear` and not read, each with the searcher's reason; the design-conformance review reads
them:

- `design@agent-skills@primer-limit`: Shape 4 defines a contradiction between goals ("one goal rules out what another requires; the exclusion wins"), a rule any session ranking two goals applies; the head holds in the primer "what every session needs and no installed skill delivers". The work places it in the goal standard only, so no bearing is established unless the rule is needed outside goal-setting.
- `design@agent-skills@answer-keeps-its-authority`: Shape 8 records a narrowing "on the owner's word with its Rules-out list"; the head says an owner's answer on a reading is recorded with its source, never as a ruling. A narrowing of a goal is intent, so no bearing is established unless a reach finding is answered as a reading of the goal.
- `design@agent-skills@owner-word-holds-as-far-as-its-premise`: Shape 11 rewords this repository's 37 goals, each ruled by the owner on premises; the head says a ruling on intent whose premise proves false is a contradiction put to the owner. The work reshapes goals rather than finds a premise false, so no bearing is established unless a reworded goal's premise is found false in the review.
- `rejected@agent-skills@directive-restated-everywhere`: If the goal-setting skill's whole-set rounds or the reviewer's stop rules restate the new grades outside the fragment, that brings back the restatement this entry rejects; the work places them in the fragment, so no bearing is established.
- `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`: The goal standard is such a whole standard; if the revision adds a pointer that applies it whole without the instruction to read it whole (e.g. from the whole-set round steps), it adds an instance; no such pointer is drafted, so no bearing is established. / if the work adds text (reviewer stop rules, Migration review) that applies the goal standard behind a pointer, the issue's 'A pointer that applies a whole standard with no such word near it ... was not found' meets it; no such pointer is drafted
- `issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`: Its instances are the two plan reviews of the previous goal-standard spec, where unread `may-bear` entries (five goals and `design@agent-skills@goals-required`) were found bearing after the scope widened to this repository's goals; this work revises the same standard and also rewords the 37 goals (shape 11), so its own plan reviews may add an instance, but it does not change the reading procedure. / shape 11 widens the work to this repository's goals, as the earlier goal-standard spec did; the issue records that after "the owner widened the work to bring this repository's goals into conformance", six entries' searcher reasons "no longer held"; it bears on how this search's grades are read, not on an element of the change
- `design@agent-skills@tripwire-audit-axis`: Shape 11 names a whole-set goal review in a Migration entry; this head states the parallel for tripwires, "The standard changes the rules recorded tripwires must meet, so a version shipping it names this axis in its Migration entry". It is about tripwires only, so no bearing is established beyond the shared pattern.
- `design@agent-skills@design-record-axis-dispatch`: The non-convergence of whole-set goal reviews (shapes 9-10) may partly be the calibration problem this head measures, "agents dispatched together do not read a rule alike"; it governs the design-record axis, not goal review rounds, so no bearing is established.
- `rejected@agent-skills@audit-without-calibration-or-recheck`: Shape 10 stops whole-set review at a count of blocks+asks and draft review at no `blocks`; this entry rejects a method with no re-check, since "the re-check of each pass found violations the pass had missed or made". It is about the audit method, so no bearing is established unless the stop rule drops a re-check.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: Its proposed "goal coverage" axis reads each goal against the heads that serve it; shapes 1 and 2 change what a goal may name and cite, which shapes that axis's reading, but the axis is not designed, so no bearing is established. / its proposed "goal coverage" axis reads each goal against the design heads that serve it; shape 1 (a goal names no part a design choice brings) changes what such an axis could match; the axis is not decided / its "goal coverage" axis reads each goal against the heads that serve it; #goal-outcome-survives-empty-skeleton removes design parts from goal texts, which could change what such an axis matches; no relation established, the axis is not built. / its proposed goal-coverage axis reads "each goal against the design heads that serve it", and shape 2 (a goal cites only goals) and shape 1 change what a goal may name about heads; the direction head-to-goal is unchanged, so no bearing is established / its proposed 'goal coverage' axis reads 'each goal against the design heads that serve it'; shapes 1-2 forbid a goal to name design-born parts or cite anything but goals, which leaves the head-to-goal direction intact, so no strain is established
- `design@agent-skills@audit-is-an-activity`: items 9 and 10 give draft mode and whole-set mode different stop rules; the head keeps the whole-set review in goal-setting because it is "one standard that serves a draft and a whole set", per the seed-adjacent `design@agent-skills@goal-set-review-in-goal-setting`; no bearing unless the revision splits the standard by mode
- `design@agent-skills@audit-passes-repeat-while-the-count-falls`: item 10's stop rules (stop on zero `blocks`; whole-set stop counting blocks+asks) answer the same problem as this head's owner-given stop rule, "If you ask an LLM to find a flaw ... it will nearly always find one"; the head governs audit passes, not goal reviews, so no relation is established, but its argument is the nearest precedent for the new stop rules
- `design@agent-skills@audit-outcomes`: item 4 makes the goal standard settle a goal-vs-goal conflict ("ruling out what another admits is no conflict, the exclusion wins"); this head sends "which side of a contradiction wins where no design head, goal or source settles it" and "a conflict with a goal" to the owner; consistent if the new rule counts as settling it, not established
- `design@agent-skills@audit-agent-per-axis`: the work revises `agent@knowledge-architect-goal-reviewer`, which hosts the goal-standard fragment; this head's principle, "The agent's definition reaches a subagent from its first token, so the axis's standard does not depend on what a brief asks it to read", holds for audit agents; a revision that moves part of the standard (grades, layers) into a brief would meet it; seed `design@agent-skills@goal-reviewer-agent` is the governing head
- `design@agent-skills@audit-repairs-the-whole-record`: item 11 brings every goal to the revised standard at once at the migration, the goal analogue of "a project audit brings every head and every rejected alternative to the rules at once"; the head covers heads and rejected entries, not goals
- `rejected@agent-skills@head-repaired-only-when-touched`: item 11's whole-set re-review at the migration is the goal analogue of what this entry rejects for heads ("a repair on touch never reaches a head no change touches"); it covers heads only
- `rejected@agent-skills@plain-confirmed-findings-count`: item 10's whole-set stop "counts blocks+asks only" over the whole set is a plain count of confirmed findings, the rival this entry rejected for audits because "The counts of two passes are taken over two different corpora"; it is an audit entry, so no bringing back is established
- `issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s`: item 9's grades and item 10's stop rule count `blocks` and `asks` apart, the analogue of this issue's severe vs total counts ("the stop rule alone never reaches the case where the total falls and the severe count does not"); the issue is about audit passes only
- `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: it was found by the goal reviewer's draft-mode reviews, and item 9's `asks` grade routes findings to the owner; the closed list of `goal@knowledge-architect@the-owner-decides` ("and nothing else") limits what `asks` may send; the issue's own texts left are the audit's, so no relation is established / found by the goal reviewer's draft-mode reviews; #9's `asks` grade and #8's narrowing "only on the owner's word" might send the owner a question the argument settles; the issue's listed texts are the audit's and the primer's, none of goal-setting, so no worsening is established
- `tripwire@agent-skills@axis-not-run-at-a-pin-move`: item 11's Migration entry makes the next version one "whose changelog cites ... the review of the whole goal set", so its consumers' pin moves enter this tripwire's watch; it fires only on received retrospective files, which the work does not produce
- `design@agent-skills@settled-means-recorded`: the drafted shapes may be the owner's proposals, which this head sends through the design skill's grounding before building; the work is already at a search of the record, so no bearing beyond the procedure under way is established
- `design@agent-skills@structure-the-flow`: shape 10 sets stop rules (rounds stop on zero `blocks`, a fifth whole-set round) for goal reviews, and this head rejects "a bound on threads or rounds" for a design discussion; the head's scope is the design skill, a goal-setting review is another activity, so no conflict is established
- `design@agent-skills@extension-follows-the-ground`: shape 7 adds a test to rule 6 and shape 5 an example line per layer, possibly recorded as members of the seed heads; whether each is a member the seed's argument covers or a change routed to the design skill is not established
- `design@agent-skills@outcome-over-display`: shape 9's `minor` grade separates findings that do not change a goal's outcome, as this head bounds which findings about the design skill become text by whether they corrupt the outcome; the head is about the design skill's tables only, so no relation is established
- `rejected@agent-skills@head-split-by-independent-reversal`: splitting the seed `design@agent-skills@goal-standard-rules` into 11 heads may use "reversing one part leaves the other standing" as the split criterion, e.g. separating rule 2's breadth exception (shape 3); no shape is argued that way yet
- `rejected@agent-skills@title-states-built-instance`: shape 9's slug #goal-finding-grades and shape 4's boundary block labels list the members built; whether such a title states the instance or a set closed on purpose is not established
- `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`: shapes 3, 5 and 7 add rules to seed heads that may be recorded as members the seed's argument covers, the act this issue records lying beyond the argument; no such recording is made yet
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: a premortem of the work (reviews still not converging, goals still narrowed) would name causes in agent behaviour in consumer sessions, which this issue says no occasion of this repository reads; whether the work writes such a tripwire is not established / #goal-met-in-tree-or-in-use has an in-use outcome met by "means of collecting and analysing evidence from use"; this issue is the gap in carrying evidence from consumer sessions to this repository; no relation established, the work names no tripwire. / shape 7's "means of collecting and analysing evidence from use" for outcomes judged in use is the gap this issue records (evidence of agent behaviour arises in consumer sessions no occasion reads); the work need not build those means, so no bearing is established / the premortem of shapes 9 and 10 (reviews converge, stop rules) would name causes in reviewer behaviour whose evidence lies in sessions; this issue states such a tripwire "guards nothing" and `design@agent-skills@tripwire-admission-test` now refuses it; no tripwire of the work is drafted yet, so no bearing is established / the work's premortem may name causes about agent behaviour (reviews still not converging, drafters still narrowing) whose evidence lives in consumer sessions; the entry says such tripwires are read by no step, and the tripwire-admission test it cites refuses them. No tripwire of the work exists yet. / a tripwire the work might record on reviewer behaviour under the revised standard has its evidence in consumers' sessions, the case this issue describes; the issue's closing sentence says such a tripwire "is no longer written"; no tripwire is drafted in the work
- `issue@agent-skills@the-retrospective-counts-no-review-cost`: the work's problem is review rounds that loop and do not converge, the cost this issue says no instrument counts; shapes 9-10 change stop rules on counts of findings, not rounds per branch, so no relation is established / the work aims to cut goal-review rounds, which this issue's "review rounds" count would measure; the work adds no count, so no relation is established
- `tripwire@agent-skills@rule-title-wider-than-its-argument`: the work writes up to 11 heads, read at its decision-record review; titles such as #goal-one-meaning-any-breadth ("never breadth") may state more than their argument argues; it fires only on two branches' findings or an audit, not established here
- `tripwire@agent-skills@split-rule-from-exception`: shape 3 (rule 2's exception: "every X names its set" only for what a goal requires or rules out) and shape 7 (rule 6's test) as heads apart from the seed `design@agent-skills@goal-standard-rules` could split a rule from its own exception, which this tripwire watches; fires only on two branches' findings or an audit
- `goal@knowledge-architect@records-live-beside-the-code`: shape 11's whole-set review applies shape 1 to this goal, which names "the knowledge table" and "an external tracker"; shape 5 writes example lines into goals homes, files of the repository as this goal asks; no conflict established
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: shape 11's whole-set review of 37 goals writes a record in docs/goal-reviews/, a pending home this issue names as reached by no installed search; the work does not change that search, so no relation is established / the re-review of 37 goals (11) writes a docs/goal-reviews/ record, which this issue says no installed search reads; the work does not change the search, so no relation is established / shape 11's whole-set review of 37 goals writes a record in docs/goal-reviews/; the issue names that directory "whose rulings the owner gives on goals the working tree holds uncommitted" as outstanding work no search reads; not established that the work leaves such a file pending past its branch
- `rejected@agent-skills@terms-used-by-two-installed-texts`: the terms the work coins are used by two installed texts (the goal-setting skill and the goal-reviewer agent, through the fragment); placing them in the primer would bring this rival back, but no shape proposes that
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: if the session applies the drafted standard to this repository's 37 goals through reviewer agents in order to choose between shapes, that is the census this issue leaves open; shape 11 runs the review only after the change, so no dependence is established
- `tripwire@agent-skills@agent-text-written-without-the-standard`: the work adds agent-facing text; the tripwire fires only if an addition that fails the need test is made in a session that did not load the agent-configuration skill, which the work's design does not establish
- `design@knowledge-architect@changelog-reviewed-at-the-release`: the Migration entry's claims about what a new goals-home check (2) refuses are read for truth only at the release review, per this head; the work does not change that review, so no bearing is established
- `design@knowledge-architect@no-future-breaking-cost-argument`: whether to enforce shape 2 by a checker rule now could be argued on "changing it later would be breaking", which this head refuses; no such argument is in the work yet
- `issue@core@finding-texts-are-not-audited-for-a-needless-cause`: the new finding of #2's checker rule is a finding text written under `design@core@finding-names-the-repair`; the issue is about existing texts, so the new one neither closes nor worsens it unless written against the rule.
- `tripwire@core@a-second-producer-chain`: its re-entry "any new module under the check directory" is met if #2's rule lands as a new check module; its firing clause (a check reading another check's findings) is not met by a reference-target check, as far as the drafted shape says.
- `issue@core@cross-project-references`: cites the seed `design@core@an-entity-belongs-to-its-anchor`; #2's rule restricts goal references by anchor (own or enclosing Component) inside one project, and meets no element of a cross-project form. / cites the seed `design@core@an-entity-belongs-to-its-anchor`; #2's rule refuses references within one project, and no element of the work reaches another project's entries
- `design@agent-skills@review-repair-appended-or-folded`: shape 2's possible checker rule refusing non-goal references in a goals home changes what the per-commit gate checks, so a later review repair of the 37 goals might have to be folded ("folded only where appending leaves an earlier commit failing"); whether the rule is built is open, so no bearing is established
- `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`: it rests on the CI clause of `goal@knowledge-architect@agents-get-a-complete-workflow`, which the shape 11 re-review may reword; whether the goal-standard fragment or the goal reviewer assumes pull requests was not checked
- `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`: shape 8's narrowing "on the owner's word with its Rules-out list" is recorded across a goal-review file (verbatim regime) and goals and commit messages (attribution regime), the slip the issue describes; the work does not change either regime, so no bearing is established
- `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`: its closure puts "each intent on the owner list as a goal to draft", goals that would be drafted under the revised standard (shape 1 may refuse an intent that names a design-chosen part); no bearing on its subject, the heads, is established
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: "an intent that states an outcome is drafted as a goal" meets shape 1, under which a goal names no part whose existence comes from a design choice: an owner intent naming such a part would then be neither goal nor head; the head says a preference about a mechanism is an argument or a fork, so no conflict is established
- `issue@knowledge-architect@the-retrospective-asks-for-no-tool-a-session-set-up`: it rests on the measure of `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, which the shape 11 re-review may reword; the work does not touch the report it asks for
- `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: it rests on the activity lists of `goal@knowledge-architect@agents-get-a-complete-workflow` and `goal@agent-skills@one-skill-per-activity`, which shape 3's breadth rule and the shape 11 re-review may reword; the work does not touch those skills
- `design@core@reference-shape-outside-span`: shape 2's possible checker rule refuses every non-goal reference in a goals home; a writer repairing it by dropping the backticks meets this lint ("a reference written without its backticks"), so the new finding's repair must be a rewrite in words; no conflict established, it only bounds the repair the new rule names
- `design@core@harness-kinds-cited-without-anchor`: shape 2 would refuse a harness-kind reference in a goals home, as `goal@agent-skills@one-skill-per-activity` cites `skill@knowledge-architect-agent-configuration@where-text-goes`; the head defines these forms "for every kind" and the new rule narrows where one may stand, with no sentence of the head contradicted
- `design@core@backticked-bare-id`: a repair of a shape-2 finding that writes a design head's slug alone in backticks in a goal is this lint's finding; the new rule's repair has to avoid it; no conflict established
- `design@core@plain-word-lint`: a goal reworded under shape 1 or 2 that names a design subject in words may hit "a plain word that is exactly a defined id is a finding"; no case of it is established by the drafted shapes
- `issue@core@configuration-for-several-agent-providers`: its trigger is "the owner changes the ask of `goal@knowledge-architect@serves-one-kind-of-project` that every target project's agents run under `claude`"; shape 11's re-review of that goal, with shape 1 on parts that come from a design choice, could reword that ask; not established that it would change it
- `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`: "Found by the whole-set review of this repository's goals ... whose reviewers reported it in every layout round": shape 6 (a goal is expected to be unmet) and shape 10 (whole-set stop counts blocks and asks) change whether the shape-11 review reports such an unmet goal again; the issue's own subject, the missing commands, is not touched
- `design@agent-skills@plan-read-against-the-record`: the work's plan document will be read by the design-conformance reviewer, which "reports a contradiction of a goal always" and "an acceptance criterion whose observable the record rules out"; shapes 4 and 11 redefine "rules out" and reword goals, so the reviewer's yardstick could shift. No conflict with a sentence is established.
- `design@agent-skills@additions-review-agent`: the head says the agent-text reviewer hosts the fragments of the admission standard and reads a changed definition from disk; the goal standard is likewise a fragment hosted by a skill and an agent (goal-setting, goal-reviewer), and the work changes that fragment and that agent. The pattern is parallel; no sentence of this head is contradicted.
- `design@core@every-path-names-its-anchor`: Cites seed `design@core@an-entity-belongs-to-its-anchor`; draft #2's checker rule refusing in a goals home any reference that is not a goal would refuse there a `path` reference, the checked form this head gives every path, but no goal entry of this repository's goals homes holds a `path` reference (grep over the goals files), so no conflict is established.
- `design@core@register-home-reference`: Draft #2's checker rule ("refusing in a goals home any reference that is not such a goal") would refuse the `register@<anchor>@design` and `register@knowledge-architect@goal` citations each Component's goals-home preamble holds (e.g. crates/core/docs/goals.md:4,8), the form this head says a home is cited by; a rule scoped to entries only, as `design@core@record-is-every-register-entry` scopes edges, would not meet it, and the scope is undrafted.
- `design@core@record-is-every-register-entry`: Goals are record entries whose edges are "a reference written inside an entry's own text"; #2 restricts a goal's edges to goals and possibly checks it, and this head's sentence "a home's prose outside every entry yields none" is the scope distinction such a rule would need; no conflict established, since the head states no rule on what an entry may cite.
- `design@core@goals-join-a-cluster-as-context`: "A cluster holds the goals its unit's entries cite, and every goal those cite in turn": #2 makes goals cite only goals upward, which this transitive join reads; today no goal entry cites a non-goal entry of the record (grep), so no change of membership is established.
- `design@agent-skills@premortem-as-watch-points`: item 7 (outcomes judged only in use, met by means of collecting evidence from use) could add or lean on a standing retrospective question; the head says each question watches a goal (it cites `goal@knowledge-architect@the-workflow-leads-to-better-code`, `goal@knowledge-architect@projects-are-split-into-components`), goals item 11 may reword; no change to the questions is drafted
- `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`: its closure has "the owner state the goal of the reuse ... under `skill@knowledge-architect-goal-setting`", so a goal to be drafted under #6 (expected unmet) and #1; no bearing established, since the goal is not written yet.
- `design@agent-skills@workflow-axis-lenses`: #9 gives the goal reviewer, a review standard, new grades of what counts as a finding; L8 reads "each pair of a review standard and an audit standard that judge one subject" for divergent verdicts; no audit standard on goals is established in this cluster, so no pair is shown
- `design@agent-skills@workflow-axis-finding-standard`: #8 to #10 add instructions on how the goal reviewer and drafter behave; the head says "an instruction about how an agent behaves is added only on evidence from real use"; the head governs the audit axis, and the work cites observed review loops, so whether those count as real use is not established here
- `rejected@agent-skills@one-repair-per-finding`: the whole-set rounds (seed `design@agent-skills@whole-set-rounds`) repair each reviewer finding at its goal, and the work reports that narrowing repairs create new findings; the entry rejects "one repair per finding, at its site" for audits of agent-facing text, and goals are not instructions, so no bearing is established
- `design@core@an-unresolved-reference-is-reported`: #2 adds a refusal of a reference that resolves (to a non-goal or a goal of a non-enclosing Component); the head counts "five ways" a reference resolves to nothing, each with its repair; the new finding is not an unresolved one, so no widening is established, though its repair message would follow the same `design@core@finding-names-the-repair` shape
- `rejected@core@configurable-reference-separator`: cites the seed `design@core@an-entity-belongs-to-its-anchor`; no element of the work touches the reference separator
- `rejected@core@hash-reference-separator`: cites the seed `design@core@an-entity-belongs-to-its-anchor`; no element of the work touches the reference separator
- `design@agent-skills@ruled-items-labelled`: shape 9's `blocks`/`asks`/`minor` grades and shape 8's narrowing "on the owner's word" put reviewer findings to the owner; the head says "Every distinct prefix is listed here, and an edit that adds one checks it against the list", and its table lists no prefix for goal-review items reaching a docs/goal-reviews/ file; not established whether the revised findings take a label of their own
- `design@agent-skills@attribution-replaces-quotation`: shape 8 records a narrowing "on the owner's word" with its Rules-out list, and shape 11 rewords goals on rulings; the head says a durable record "state[s] what the owner decided ... never the owner's words verbatim", and its list names no goals home; not established whether a goal body or its commit would quote the owner's ruling
- `design@agent-skills@intent-through-goals-not-overrule`: the head makes a goal the owner's channel for intent ("a goal that answers one case wrongly ... changes through goal-setting"); shapes 1 and 6 restrict what a goal may say, which may narrow that channel; no conflict established, since a preference about a mechanism is already an argument there
- `design@agent-skills@default-ruled-before-merge`: shape 11's whole-set review rewords goals on the work's branch; the head says "The owner's ruling is needed before the branch that builds the default merges"; a goal rewording already goes to the owner, so no conflict established beyond the timing of the rulings
- `design@core@orphan-rejected-entry`: shape 2's checker rule refusing every non-goal reference in a goals home meets the sentence 'A citation from any other text, a goal, ... does not count': the rule only removes citations the check already ignores, so no conflict is established
- `design@core@registers-are-declared`: shape 2's possible checker rule is a new compiled rule on the built-in `goal` register; the head fixes what a declaration may set on a built-in ('accepts kinds on issue and nothing else'); a compiled rule is no declaration, so no contradiction is established, unless the rule is made declarable
- `design@core@heading-register-two-shapes`: shape 2's rule would read references in a goals home, which 'Both shapes serve every heading register' lets be a file or a README-led directory; the rule must judge both shapes, and nothing in the work says otherwise
- `rejected@core@goal-citing-a-rejected-entry`: shape 2 states 'a goal cites only goals', the same direction as this entry's reason 'the citation fails the revisit test'; the work cannot bring this design back, and only a rule written as a refusal would turn the entry's argument into a check, with no relation established
- `design@core@harness-kinds`: shape 2's possible checker rule refusing in a goals home every reference that is not a goal would refuse `skill@<name>`/`agent@<name>`/`primer@<slug>` spans there, against the head's "a citation of a skill, an agent or a section is checked"; a narrowing of where a harness kind is accepted, not established as a change of this head
- `design@core@installed-entities-from-the-tree`: the revised standard may rename or add sections of the goal-setting skill and the goal-reviewer agent (the drafted `#goal-...` slugs); the head's "a branch that renames a section in an upgrade and repairs its citations passes `commits` on every commit" then binds how the rename lands; not established that any section is renamed
- `design@core@init-command`: `init` and `new-component` write an empty goals home; shape 2's possible checker rule over goals-home references, and shape 5's per-layer example lines, act on that home; nothing established that the written skeleton must change
- `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`: the work edits several installed texts (the fragment renders into the goal-setting skill and the goal-reviewer agent); the issue's reproduction is "a commit that stages one edited skill and its installed copy, while another edited skill and its copy stay unstaged", which a partial commit of this work would meet; no bearing on the design established
- `design@agent-skills@goal-lifecycle`: shape 6 drafts every new or reworded goal to be unmet and shape 7 sorts how a goal is met; the head says "A goal is met or unmet, and stays in its goals home while it is met"; an unmet goal still stays, so no conflict is established, but shape 6 changes what "met" means at drafting time
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: shape 4's "ruling out what another admits is no conflict, the exclusion wins" and shape 5's reading examples in the goals home guide an agent applying goals; the head says "No installed text tells an agent what to do when a goal it applies reads two ways"; whether a precedence between goals is such an instruction is not established
- `rejected@agent-skills@goal-set-review-as-an-audit-axis`: shapes 10 and 11 change and run the whole-set review; the entry "stays the design to reach for if a goal set grows past what one agent reads"; 37 goals is below the owner's reported 100, so no reproposal is established
- `design@knowledge-architect@no-intermediate-component`: item 2 (a goal cites goals of its own or an enclosing Component only) and item 1 (a Component's goal may name its own and enclosing Components) meet the head's argument "Every goal of these Components refines a goal of the root's goals home directly, or none"; the root goals that cite agent-skills goals downward would lose those citations, which leaves the upward refinements the head counts unchanged, so no bearing is established
- `design@core@plan-item-scope`: shape 2's possible checker rule refuses a reference by the file it sits in, as this head refuses an item citation "from anywhere else ... before its id is looked up"; a plan item cited in a goals home would meet both refusals, and which finding and repair the checker gives is not established; no conflict found
- `design@agent-skills@component-skill-delivery`: it states that the goal-setting skill points to the component-creation skill for a purpose the owner states for several Components; the work rewrites the goal-setting skill's standard text, and only a rewrite of that pointer would meet the head, which the work does not plan.
- `design@agent-skills@default-code-shape`: it says the shape's being stated by the skill "not a goal ... listing the properties" is the session's choice; #5's per-goal example lines and #3's breadth rule could prompt a re-review (#11) of `goal@knowledge-architect@the-workflow-leads-to-better-code` to list properties, which the head argues against; no element of the work proposes it.
- `design@agent-skills@tripwire-admission-test`: the work's own premortem causes (reviews still narrowing goals, draft reviews still looping) are causes on how sessions follow an instruction; the head's clause "firing on a recurrence in separate pieces of work" and condition 2 (evidence lands where tripwires are read) decide whether any becomes a tripwire; no element of the change alters the standard itself
- `design@core@anchors-are-components-and-locations`: shape 2's possible checker rule refuses in a goals home a reference that is not a goal of its own or an enclosing Component; which anchors enclose a goals home rests on the component/location split this head states, and whether a location may carry a goals home is not stated in the work
- `issue@agent-skills@no-reviewer-writes-its-own-report-file`: the work revises the goal reviewer agent's output (shape 9 grades); the issue says "No installed reviewer agent is told to write its report to a file"; whether the goal reviewer is among the reviewers whose reports the review skill saves is not established, and the work names no change to how its report is delivered

## Criteria

### Ordinary work moves toward a goal without rewording it `##c-goals-change-rarely`

Binding, from `goal@knowledge-architect@agents-work-without-drift`. Met by #goal-outcome-survives-empty-skeleton,
#goal-cites-goals-upward-only and #goal-one-meaning-any-breadth.

### A goal rules out nothing the owner did not rule out knowingly `##c-intent-not-deformed`

Binding, from `goal@knowledge-architect@the-owner-decides`. Met by #reach-finding-asks-not-narrows,
#requires-admits-rules-out and #goal-carries-reading-examples.

### Draft-mode and whole-set reviews stop within a bounded number of rounds `##c-review-converges`

Binding, from `goal@knowledge-architect@owner-attention-on-what-is-theirs`. Met by
#goal-finding-grades and #review-stops-on-blocking-zero.

### The standard and the check work in a target project without this repository's record `##c-works-in-a-target-project`

Binding, from `goal@agent-skills@installed-text-works-in-every-target-project`. Met: the check is
compiled in and the same in every project; the standard cites no entry of this repository.

### The cost of bringing existing goals to the new standard `##c-migration-cost`

Weighed. Unmet and accepted on the owner's word, round 5, on Q4: "cost accepted". This repository's
37 goals, counted with `grep -c '^## '` over the seven goals homes, go through rounds of the review of
the whole goal set, and a consumer's goals homes fail the new check until their citations are
rewritten, which the check's Migration entry names.

## Threads

### A goal keeps its meaning in a project emptied of its design and its code `##goal-outcome-survives-empty-skeleton`

Proposed by the owner, round 1; shaped by the session, round 1. Approved: presumed settled by the
owner's "I think we fully align on the conclusions and the proposals here." (round 2), closed by the
owner's word on the checkpoint table, round 4: "agreed on the table." Arguments: #arg-1, #arg-6,
#arg-7, #arg-12, #arg-30. Shape: "Decided design", rule 1. Harvest: `design@agent-skills@goal-standard-rules`.

### A goals home cites only goals and registers of its own Component or of an enclosing one, and the checker enforces it `##goal-cites-goals-upward-only`

Proposed by the owner, round 1; the checker rule proposed by the session, round 1; widened by the
owner, round 3. Approved, round 4: "goal-cites-goals-upward-only accepted, this is the shape I want".
The owner's words, round 3: "The checker rule holds, and the core head should be updated." and "I
think citing design heads should be forbidden in all goal docs. There is no argument for citing
design@agent-skills@goal-lifecycle in root goals. A consumer project cannot do that in particular,
and its goal lifecycle is only grounded in what the goal setting skill says. Citing register should
be allowed upward, the same way it is for citing upward goals." Arguments: #arg-2, #arg-8, #arg-13,
#arg-14, #arg-15, #arg-34. Shape: "Decided design", the check and rule 7. Harvest: step 1 wrote
`design@core@goals-home-cites-upward-only`, the pair #goal-cites-goals-upward-only →
`design@core@goals-home-cites-upward-only`, since the rule covers a home's head and its registers, and narrowed
`design@core@no-citation-direction-rule`; `design@agent-skills@goal-standard-rules` states rule 7 and
cites the core head, which holds the argument.

### Rule 2 judges ambiguity, never breadth `##goal-one-meaning-any-breadth`

Proposed by the owner, round 1. Approved, closed by the owner's word on the checkpoint table, round 4.
Arguments: #arg-3, #arg-9, #arg-29, #arg-30. Shape: "Decided design", rule 2. Harvest:
`design@agent-skills@goal-standard-rules`.

### A goal is read in three layers, requires, admits and rules out, and only ruling out what another requires is a contradiction `##requires-admits-rules-out`

Proposed by the owner, round 1. Approved for the reclassification of existing records, round 3:
"Reclassifying existing records is accepted (point 4)", point 4 being finding 4, and closed by the
owner's word on the checkpoint table, round 4. Arguments: #arg-4, #arg-16, #arg-17, #arg-21. Shape:
"Decided design", rules 3 and 5. Harvest: `design@agent-skills@goal-standard-rules`.

### A goal carries "for instance" cases for each layer it has `##goal-carries-reading-examples`

Proposed by the owner, round 1. Approved with the session's default of round 2, one or more cases per
layer the goal has, closed by the owner's word on the checkpoint table, round 4. Arguments: #arg-5,
#arg-18, #arg-19. Shape: "Decided design", the goal entry. Harvest: `design@agent-skills@goal-entry-shape`,
`design@agent-skills@goal-draft-shows-reach`.

### A new or reworded goal is drafted to move the tree, and is expected to be unmet `##goal-drafted-to-move-the-tree`

Proposed by the owner, round 1. Approved, closed by the owner's word on the checkpoint table, round 4.
Arguments: #arg-10, #arg-20, #arg-31. Shape: "Decided design", drafting. Harvest:
`design@agent-skills@goal-standard-rules`.

### Rule 6 sorts an outcome by whether it depends on what the project does not control `##goal-met-in-tree-or-in-use`

Proposed by the owner, round 1. The wording of its test approved with the survey's defaults, round 3:
"I agree with your defaults proposals for the S items from the survey.", and closed by the owner's
word on the checkpoint table, round 4. Arguments: #arg-11, #arg-22, #arg-32. Shape: "Decided design",
rule 6. Harvest: `design@agent-skills@goal-standard-rules`.

### A finding on a goal's reach lists the cases by layer and asks the owner, and the drafter narrows only on the owner's word `##reach-finding-asks-not-narrows`

Proposed by the owner, round 1. Approved, closed by the owner's word on the checkpoint table, round 4.
The reviewer's half, a case a goal only admits being no finding while the drafter shows the reach of
every layer, is the session's shape, approved in that table. Arguments: #arg-4, #arg-12, #arg-17,
#arg-33. Shape: "Decided design", the reviewer and drafting. Harvest: `design@agent-skills@goal-reviewer-agent`,
`design@agent-skills@goal-draft-shows-reach`.

### The goal reviewer grades each finding `blocks`, `asks` or `minor` `##goal-finding-grades`

Proposed by the owner, round 1. Approved, closed by the owner's word on the checkpoint table, round 4.
Arguments: #arg-23. Shape: "Decided design", the grades, and the mapping table. Harvest:
`design@agent-skills@goal-reviewer-agent`.

### A review stops on the findings left on old wording `##review-stops-on-blocking-zero`

Proposed by the session from the issue on the draft-mode loop, round 1; the count on old wording
from the record search and from S1, round 2, approved, round 3: "I agree with your defaults proposals
for the S items from the survey."; closed by the owner's word on the checkpoint table, round 4. The
stops as "Decided design" states them, the first round counted over all wording and the draft-mode
stop counting only the `blocks` findings the drafter repairs, are the session's repair of the plan
review, within that approval. The slug keeps the discussion's name. Arguments: #arg-24, #arg-25,
#arg-27, #arg-28. Shape: "Decided design", the stops. Harvest: `design@agent-skills@whole-set-rounds`,
and the draft-mode stop in `design@agent-skills@goal-reviewer-agent`.

### This repository's goals are brought to the new standard by rounds of the review of the whole goal set, after a mechanical edit `##goal-standard-rereview-here`

Proposed by the session, round 1. Approved, round 4, with the owner's shape for the first edit: "The
initial fix of the goals can be considered a mechanical edit of it, and be done under no review and
no word from me. It should just focus on getting rid of citations that will be linted without
rewording sentences (a pure mechanical edit). The goal whole set review will then bring the goals
into shape after the work, and de-narrow them when relevant." Round 5, on Q1: "merge can only happen
after we fix this under multiple rounds of the whole set review." Arguments: #arg-26, #arg-31,
#arg-34. Shape: "Decided design", the mechanical edit, and step 5. Harvest: the Migration entries of
`path@knowledge-architect@CHANGELOG.md`.

## Arguments

### The goals should drive change, not follow it `##arg-1`

Round 1, the owner: "goal changes are a rare occurence. they must be shaped so that standard work
happen in their direction without any need to change them all the time." and "goals must keep making
sense with the same wording, if the project was removed of all its design and implementation, left to
an empty skeleton." Bears on #goal-outcome-survives-empty-skeleton.

### A goal tied to a lower Component is tied to a shape that can change `##arg-2`

Round 1, the owner: "never goals that are in a lower level component, because a lower level
component can be replaced or split without changing the purpose of the containing component, or of
the project." Bears on #goal-cites-goals-upward-only.

### No ambiguity is not narrowness `##arg-3`

Round 1, the owner: "It is important for goals to have no ambiguity, but not to the point of
narrowing down." Round 1, the session: ambiguity is two agents in the same situation moving in
different directions; breadth is one direction admitting many shapes. Bears on
#goal-one-meaning-any-breadth.

### Admitting is not requiring `##arg-4`

Round 1, the owner: "If one goal rules out somehting another requires, this is a contradiction. If
one goal rules out what another admits, this is normal. Admitting is not requiring, thus there is no
conflict. The goal that rules out wins." Bears on #requires-admits-rules-out,
#reach-finding-asks-not-narrows.

### Examples carry the intended reading `##arg-5`

Round 1, the owner: "maybe keeping one line of examples for each of Requires/Admits/Rules out in the
written goals could help better transmit the intended readings." Round 1, the session: the boundary
block is deleted after the ruling, so a later agent reads the bare sentence. Bears on
#goal-carries-reading-examples.

### The repairs the standard asks only add words `##arg-6`

Round 1, the session: rule 2 adds the set a word ranges over, rule 3 an exception, rule 6 the most
specific observable; no rule removes words. The root goals home holds 19 goals in 6215 words. Bears
on #goal-outcome-survives-empty-skeleton, #goal-one-meaning-any-breadth.

### A goal that names a built part binds the part `##arg-7`

Round 1, the owner: "it is fine to define the checker as a command/tool provided by the project,
without using it as a name, in goals. But it should not use it as \"the checker crate\", because the
checker being its own create is an outcome of design choices and argumentation." Bears on
#goal-outcome-survives-empty-skeleton.

### A goal cites no entry but goals `##arg-8`

Round 1, the owner: "it becomes obvious that a goal must carry no citations of entries other than
goals." Round 1, the session: the root home cites lower Components' goals 9 times and one goal cites
a skill section; the rule is mechanically checkable. Bears on #goal-cites-goals-upward-only.

### The review narrows to the shape that admits the owner's intent and nothing more `##arg-9`

Round 1, the owner: "the review passes tend to \"narrow\" goals to the shape that admits my intent, and
nothing more. This ends up with drafts that rule out a lot of consequences I never ruled out."
Bears on #goal-one-meaning-any-breadth, #reach-finding-asks-not-narrows.

### A new goal being unmet is the point `##arg-10`

Round 1, the owner: "the drafting agent tries to write a goal that the current tree meets. This is
the wrong angle." and "An new goal being unmet is the point, most of the time." Bears on
#goal-drafted-to-move-the-tree.

### Some goals are met only through use `##arg-11`

Round 1, the owner: "goals that are related to providing a public service/feature with effects on
their usage environments (not just generate results, but also analyze or modify external data),
cannot be shown to be met purely in tree, and I think this aspect is not clearly pointed out in the
goal standard." Bears on #goal-met-in-tree-or-in-use.

### An accepted wording becomes the owner's word `##arg-12`

Round 1, the owner: "Once I have accepted a wording, the narrowing and what it rules out become \"my
word\" for the agent, even if I did not noticed all of what it rules out, and the goal no longer
state my intent." and "the default effect for such findings should not be to propose a narrower
wording, but instead to ask about the full list of what the goal admits that might not have been part
of my intent". Bears on #reach-finding-asks-not-narrows, #goal-outcome-survives-empty-skeleton.

### The rejected one-way rule was refused as unneeded, and this rule is narrower `##arg-13`

Round 2, the session: `rejected@core@citation-direction-checked` was refused because a one-way rule
"buys only acyclicity" for the record search; the new rule covers goals homes for goal stability.
Round 3, the owner: "most of the argument behind the reject was that it was unnecessary for the
settled design. Anyway, here our proposal has a lesser scope: it only applies to goals. design and
issues can still cite in whatever direction." Bears on #goal-cites-goals-upward-only.

### A consumer cannot cite this repository's design heads `##arg-14`

Round 3, the owner, quoted under #goal-cites-goals-upward-only: a consumer project cannot cite
`design@agent-skills@goal-lifecycle`, and its goal lifecycle rests on the goal-setting skill alone.
Bears on #goal-cites-goals-upward-only.

### The check needs the preambles and the first commit `##arg-15`

Round 2, the record search: six goals homes' preambles cite a register's home, five goals and the
root's preamble hold citations the check refuses, and a stricter checker goes in the branch's first
commit with every fix, per `design@core@a-commit-message-is-a-document`. Bears on
#goal-cites-goals-upward-only, #goal-standard-rereview-here.

### Rule 5 already reads the three layers' way `##arg-16`

Round 1, the session: "No state of the project meets one goal only by failing another": a state
without X meets a goal that admits X and one that rules it out. KAOS, RE'01: goals conflict when no
behaviour meets them all. Bears on #requires-admits-rules-out.

### A narrowing repair creates the next round's finding `##arg-17`

Round 1, the session: a narrowing repair rules out cases the owner never ruled out, a new rule-3
finding the next round reports. Bears on #reach-finding-asks-not-narrows, #requires-admits-rules-out.

### A fixed count of example lines is structure the work does not need `##arg-18`

Round 2, the record search: `goal@agent-skills@only-needed-instructions` forbids "a fixed sequence,
count or mapping between a unit of the work and a unit of its record that another instruction needs
to vary"; the same goal says "A category or an example that a goal names … makes no instruction
needed". Bears on #goal-carries-reading-examples.

### An example can become the measure `##arg-19`

Round 1, the session: an agent can take a Requires case as the measure, the list-of-properties
defect of rule 6, and an example can name a built part. Survey, round 2: a measure taken as the goal
replaces it (Manheim and Garrabrant 2018; Choi, Hecht and Tayler 2012). Bears on
#goal-carries-reading-examples.

### Objectives always fully met are not ambitious `##arg-20`

Round 2, the survey: re:Work, a team that always reaches 1.0 has objectives that "aren't ambitious
enough". Bears on #goal-drafted-to-move-the-tree.

### The three layers reclassify existing records `##arg-21`

Round 2, the record search: `issue@mutate@a-mutation-of-several-files` reads "admits a mutation of
several files … so the goal is unmet for that case". Bears on #requires-admits-rules-out.

### KAOS sorts goals by who can enforce them `##arg-22`

Round 2, the survey, S5: "Unlike requirements, assumptions cannot be enforced by the software-to-be"
(RE'01). Bears on #goal-met-in-tree-or-in-use.

### Only one grade exists today, and more grades help convergence `##arg-23`

Round 1, the owner: "Right now, the finding on overlap which does not change what an agent would do
is the only graded (minor), and more variation in gravity of findings might help convergence."
Bears on #goal-finding-grades.

### The whole-set review did not converge, and findings were about intent `##arg-24`

Round 1, the owner: "the goal review in whole set mode was observed not to converge. I think this in
itself is a smell that the standard has a problem." and "most raised findings in the later rounds
were actually about intent". Bears on #review-stops-on-blocking-zero.

### Repairs write new text that agents find defects in `##arg-25`

Round 2, the record search: `design@agent-skills@convergence-count-scope`, "each pass's repairs
write new text, and the agents find defects in new text at least as readily as in old", measured at
30 of 38 findings. The survey, S1: Gilb's inspection finds about 30% of defects per pass, so one
clean round is weak evidence, an assumption for goal reviews. Bears on #review-stops-on-blocking-zero.

### The first edit is mechanical, and the review of the whole set brings the goals into shape `##arg-26`

Round 4, the owner, quoted under #goal-standard-rereview-here. Bears on #goal-standard-rereview-here.

### Repairs applied together had unforeseen effects `##arg-27`

Round 1, the owner: "The non-convergence is in part due to corrections applied at previous rounds,
which tend to have unforeseen effects when applied all together." Bears on
#review-stops-on-blocking-zero.

### A small batch converged `##arg-28`

Round 1, the owner, a fact only the owner holds: "When applied to a single or low volume of goals,
the reviews were converging more reliably." Bears on #review-stops-on-blocking-zero.

### Vaguer goals add fewer conflicting instructions `##arg-29`

Round 1, the owner: "But I believe that if we promote goals that are \"more vague\" as I propose, this
should no longer happen, because less \"potentially conflicting\" instructions will be added, and the
whole set review would have higher chances to converge." and "goals should be more about describing
far reached outcomes that the project, the owner, and the agents work toward, incrementally." Bears
on #goal-one-meaning-any-breadth, #review-stops-on-blocking-zero.

### Goals state outcomes that do not depend on the implemented shape `##arg-30`

Round 1, the owner: "Overall, I think that all goals should aim at being \"vague\", imprecise, and state
outcomes that do not depend the implemented shape of the project. In particular root goals." Bears on
#goal-outcome-survives-empty-skeleton, #goal-one-meaning-any-breadth.

### A reworded goal left unmet surfaces a misword `##arg-31`

Round 1, the owner: "The exception might be when goals are being reviewed and findings corrected. But
even then, a reworded goal ending up unmet might be positive, because this is the kind of time where
I have high chances of noticing if a goal was misworded". Bears on #goal-drafted-to-move-the-tree,
#goal-standard-rereview-here.

### The goals met through usage reports are the model `##arg-32`

Round 1, the owner: "I think that the goals of this project related to development and component
creation have ended up in a good shape (more than the others). The key idea that was put into them
is that meeting these goals is not observable in the tree \"directly\", instead only indirectly,
through usage reports." Bears on #goal-met-in-tree-or-in-use; step 5 reads
`goal@knowledge-architect@the-workflow-leads-to-better-code` and
`goal@knowledge-architect@projects-are-split-into-components` as that shape.

### Narrowing surfaces a goal's wider effects `##arg-33`

Round 1, the owner: "The narrowing is still an interesting property though. It truly surfaces
whether a goal has wider effects than I could have intended." Bears on #reach-finding-asks-not-narrows:
the reach is kept as a finding put to the owner, not dropped.

### A relocation is a change of purpose, or a mechanical edit `##arg-34`

Round 5, the owner, on Q2: "the relocation goal should be updated, but not with an exception. Under
our new constraints on goals, relocation can only happen if there is a change of purpose of the
component that justifies it, or if it does not break the checker (a relocation with no intermediate
component would never break the goals for example, and a rename is a mechanical edit in the goals no
rewording needed)Under that, changing purpose (goals) should be the only manual work that needs to
happen at a relocation, in addition  to a few mechanical edits tolerated. But the goal in itself
should change shape, under the whole set review." Bears on #goal-cites-goals-upward-only,
#goal-standard-rereview-here.

### Asks reported by a check need no listing `##arg-35`

Round 6, the owner, quoted under "Defaults awaiting the owner", D3. Bears on
#goal-cites-goals-upward-only and step 5.

## New names, in one place

```text
crates/core/src/check/references.rs   goals_home_owner, goals_home_citation: the check, built at step 1
crates/core/tests/projects/planted/docs/goals.md   one planted citation, built at step 1
crates/core/src/mock_projects.rs      PLANTED's references row, 28, built at step 1
fragments/goal-standard.md            existing; rewritten at step 2
grades `blocks`, `asks`, `minor`      the goal reviewer's grades, beside its route tags
layers Requires, Admits, Rules out    the labels of the boundary block and of a goal's cases
```

## Decided design

### Rule 1: the skeleton test

A goal states an outcome the owner intends, and keeps its meaning, word for word, in the project
reduced to an empty skeleton, every design head, document and line of code removed. It may define
what the project provides and then name it, as "the project provides a command that checks its
documents". It names no part whose existence comes from a design choice. A Component's goal may
name its own Component and the Components that enclose it. The kind of interface the owner intends
stays an outcome; its exact form stays a decision, as rule 1 says today. Nearest rival: the test
"every word under the goal replaced, same function", the owner's other wording of round 1; it follows
from the skeleton test, which a reviewer applies directly by deleting the project and reading the
sentence.

### Rule 2: one meaning, any breadth

Each term has one meaning in the project. A goal is ambiguous when two agents in the same situation,
reading it, would move in different directions; breadth, a direction that admits many shapes, is no
defect. "Every", "no", "all" and "not in scope" name their set only where a goal requires or rules
something out, never where it states the direction. Nearest rival: the rule as it stands, which the
discussion found to be the source of the narrowing, #arg-6 and #arg-9.

### Rules 3 and 5: three layers

A goal is read in three layers: what it **requires**, what it **admits**, and what it **rules
out**. Rule 3: a goal requires nothing and rules out nothing the owner has not knowingly required or
ruled out; what it admits needs no ruling. Rule 5: a contradiction is one goal ruling out what
another requires; where one goal rules out what another only admits, there is no conflict and the
exclusion holds. "Where two outcomes compete, a goal states which ranks first" is kept, and covers a
divergence. The rules keep their numbers. Nearest rival: two layers, which reads an admitted case as
a requirement, the reading of #arg-21.

### Rule 6: in the tree or in use

The test that sorts an outcome: **does it depend on anything the project does not control, such as
its users, their data or their environment?** If not, what shows it met is a check or a test over
what the project builds. If so, what shows it met is what the project builds to pursue it and to
learn from use whether it is met, as rule 6 says today, never a list of the properties the outcome is
taken to consist of. Nearest rival: a sort by kind of outcome, correctness against effects; the KAOS
sort by who can enforce it states the owner's case more directly, #arg-22.

### Rules 7 and 8

Rule 7: a refinement neither widens nor contradicts the goal it refines, as today; and a goals home,
its goals and its head alike, cites only a goal or a register's home of its own Component or of a
Component that encloses it, which the checker enforces. Rule 8 keeps its rule, with its design named
without a part of the project as built: an exclusion names the design it excludes, described so that
the sentence keeps its meaning under rule 1. Rule 4 is unchanged.

### Drafting

`skill@knowledge-architect-goal-setting@drawing-out-intent` drafts a new or reworded goal to move
what the project builds: it is expected to be unmet, and is never shaped so that the tree as it
stands meets it. Its boundary block states what the goal asks of future work, and shows its reach by
layer: for each layer the goal has, the cases it reaches that the owner may not intend, from the
project's existing or planned work, or from the uses the owner's statements describe. The section
"The shape of a goal entry" says that a goal is cited from anywhere but a goals home, where rule 7
holds, and its `%%` comment says the same.

### The goal entry

The body of a goal states what it means and what shows it met, as today, and carries one or more "for
instance" cases for each layer the goal has, labelled Requires, Admits and Rules out; a layer the goal
does not have is omitted. The cases written are those of the boundary block the owner's ruling kept,
each passing the skeleton test; a case binds nothing, it shows a reading. The template shows them as
one list item per case, `- Requires, for instance: <case>.`, under the body; the labels are not
headings of level two, per `design@core@an-entry-is-a-heading-at-the-register-level`.

### The reviewer

- **A reach finding** lists the cases the goal reaches in its requires and rules-out layers that the
  owner's statements do not cover, and is put to the owner. A case the goal only admits is no
  finding. The drafter keeps the wording by default, and drafts a narrower one only on the owner's
  word, showing what the narrowing rules out.
- **Grades and routes.** Each finding carries a grade, by what it changes in an agent's act, beside
  the route tag it has today, `settled`, `intent` or `contradiction`; the mapping table "Each finding,
  its grade and its route" is total. `minor` replaces today's separate `minor` tag.
- The reviewer's definition states the grades; the stops are the skill's, which dispatches it.

### The stops

- **Draft mode.** After each round, the drafter repairs each `settled` finding, and the `intent` and
  `contradiction` findings go to the owner with the draft. A further round runs on the repaired batch.
  From the second round on, the review stops when a round reports no `blocks` finding tagged `settled`
  on old wording. The session classifies each finding as on old or new wording by the words the
  finding quotes, against the version the previous round read, kept in the conversation for a batch
  of three goals or fewer and in the record for a larger one. The findings of the stopping round go to
  the owner beside the goals, unrepaired.
- **The whole set.** The first round's count is of its `blocks` and `asks` findings over all wording.
  From the second round on, the count is of the `blocks` and `asks` findings on old wording, and the
  rounds stop on the owner's word or when that count falls to a fifth of the first round's; the round
  that meets the stop writes no repair, as today. The record counts each round's findings per
  reviewer, rule, grade, route and old or new wording.
- Nearest rival: a stop on no `blocks` finding over all wording, which the rewording each round writes
  holds open, per `design@agent-skills@convergence-count-scope`.

### The check

In the reference check, every reference written in a goals home, its entries and its head alike, is
refused unless it is a `goal` reference or a `register` reference whose anchor is the home's own or a
Component whose directory encloses it, the root included. `*` and `elsewhere` name no such Component.
A harness reference, `skill@<name>`, `agent@<name>`, `primer@<slug>` or `instructions@<slug>`, and
every other kind, `design`, `issue`, `path` and the rest, is refused there. A reference that resolves
to nothing and is refused gets both findings. A relative link is no reference. The finding names the
span, the home, and the repair: cite a goal of the home's own or an enclosing Component, or rewrite
the sentence so it names no entry; never remove the backticks. The finding of an id lint in a goals
home names the same repair, not the reference form the check refuses there. The check reads the
anchors and the entity table only, and holds in every project with no flag. Nearest rival: a rule
judged by the goal reviewer alone, which the check makes cheaper and certain, per
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

### The mechanical edit

Step 1 removes each citation of the table under "What the work is" by deleting words, adding none:
the clause that holds the span, such as ", per `<reference>`" or ", as `<reference>` measures"; or,
where the span is the whole object of a sentence's verb, the sentence. On the owner's word, round 4,
it takes no review and no word of the owner, and the rounds of the review of the whole goal set bring
the goals into shape afterwards.

## Mapping tables

### Each finding, its grade and its route

| finding | rule | grade | route |
| --- | --- | --- | --- |
| the goal names a part whose existence comes from a design choice | 1 | `blocks` | `settled` |
| a sentence states a mechanism as the outcome, naming no built part | 1 | `asks` | `intent` |
| a term with two meanings in the project | 2 | `blocks` | `settled` |
| a universal in a requiring or excluding clause that names no set | 2 | `asks` | `intent` |
| a case required or ruled out that the owner's statements do not cover | 3 | `asks` | `intent` |
| two intents in one goal | 4 | `minor` | `settled` |
| one goal rules out what another requires | 5 | `blocks` | `contradiction` |
| two outcomes compete, unranked, or diverge under a condition | 5 | `asks` | `intent` |
| a measure that is a plan document or anything that leaves | 6 | `blocks` | `settled` |
| an outcome judged in use measured by a list of the properties it is taken to consist of | 6 | `asks` | `intent` |
| any other wording of a measure | 6 | `minor` | `settled` |
| a refinement that widens or contradicts the goal it refines | 7 | `blocks` | `settled` |
| a citation the check refuses | 7 | `blocks` | `settled` |
| an exclusion that names no design another goal would argue for | 8 | `asks` | `intent` |
| a statement of the owner in conflict with a goal or another statement, or a part of the statements a draft drops | none | `blocks` | `contradiction` |
| a case the goal only admits | 3 | no finding | none |

### The mechanical edit, over every citation the check refuses in this repository

The lines are main's commit's, the line where the deleted words begin.

| site | deleted |
| --- | --- |
| root, line 19 | ", per `design@agent-skills@goal-lifecycle`" |
| root, line 87 | ", among those `goal@agent-skills@one-skill-per-activity` lists", leaving "include these, a list" |
| root, line 108 | the sentence "What the step says, beyond what the session's judgement handles, follows `goal@agent-skills@only-needed-instructions`." |
| root, line 131 | ", as `goal@agent-skills@no-contradiction-in-workflow` states" |
| root, line 133 | ", per `goal@agent-skills@only-needed-instructions`" |
| root, line 137 | " as\n  `goal@agent-skills@one-skill-per-activity` measures,", leaving "taken in, and in it" |
| root, line 141 | ", as `goal@agent-skills@no-contradiction-in-workflow` measures" |
| root, line 155 | ", per `goal@gates@gates-from-a-list`" |
| root, line 420 | ", per `goal@agent-skills@only-needed-instructions`" |
| root, line 486 | " in the form `goal@agent-skills@only-needed-instructions` admits" |
| agent-skills, line 11 | ", as `skill@knowledge-architect-agent-configuration@where-text-goes` uses the term" |

## Losing alternatives

- **Every word under the goal replaced, same function**, lost to the skeleton test, which it
  follows from, under #goal-outcome-survives-empty-skeleton.
- **The citation rule judged by the goal reviewer alone**, lost to the check, under
  #goal-cites-goals-upward-only.
- **A fixed count of one example line per layer**, lost to one or more cases per layer the goal has,
  on `goal@agent-skills@only-needed-instructions`, under #goal-carries-reading-examples.
- **A stop on no `blocks` finding over all wording**, lost to the count on old wording, on
  `design@agent-skills@convergence-count-scope`, under #review-stops-on-blocking-zero.
- **A separate layer of measures agents revise** (OKR key results, S2), lost to `primer@goals-bind`:
  the goals are the only statements assumed to come from the owner, so the measure stays stated as a
  mechanism inside the goal, under #goal-met-in-tree-or-in-use.
- **An accepted-risk layer in a goal** (commander's intent, S4), lost to rule 5's ranking and a design
  head's costs, under #requires-admits-rules-out.
- **A new rule for a divergence** (KAOS, S3), absorbed by rule 5's ranking sentence, under
  #requires-admits-rules-out.
- **An exception in the relocation goal for a move that changes nesting**, lost to the owner's word,
  round 5 on Q2, #arg-34, under #goal-cites-goals-upward-only.

`path@agent-skills@docs/grounding/goal-standard.md` holds the survey the first seven come from.

## Readings

None: the work implements no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| P1. Goals become too wide to constrain anything, and forks to the owner multiply | #goal-one-meaning-any-breadth | named as a cause for the harvest to judge as a tripwire |
| P2. `asks` lists grow long, and the owner narrows to shorten them | #reach-finding-asks-not-narrows | named as a cause for the harvest to judge |
| P3. Reviewers read "whose existence comes from a design choice" two ways | #goal-outcome-survives-empty-skeleton | named as a cause for the harvest to judge |
| P4. A "for instance" case becomes the measure: an issue or a head declares a goal unmet, or a design required, from a case alone | #goal-carries-reading-examples | named as a cause for the harvest to judge |
| P5. A consumer strips the backticks to clear the new finding | #goal-cites-goals-upward-only | a claim of step 1: C4 and C7 |
| P6. The reclassification of "admits" closes an issue whose work the owner wanted | #requires-admits-rules-out | named as a cause for the harvest to judge |

## Acceptance criteria

None. Each cause the code of this work could check is a claim of the step that builds it; the
others are judged at the harvest as causes.

## Implementation sequence

### Step 1: the check, the core head, and the mechanical edit

Builds: the check of "Decided design", in `path@core@src/check/references.rs`; the repair the id
lints name in a goals home; a planted citation of the `planted` mock's one design head in its goals
home, with `PLANTED`'s references row raised to 28; the mechanical edit;
`design@core@no-citation-direction-rule` narrowed and `design@core@goals-home-cites-upward-only`
written; the check's sentence in `path@core@README.md`; the check's CHANGELOG Migration entry. Its
commit is the branch's first, and its message names this spec by its bare id, since neither its
tree nor its parent's holds it.

Claims:

- C1. A `goal` or `register` reference of the home's own Component or of an enclosing one passes in
  a goals home, in its entries and in its head: a unit test over a two-level anchor list; refuted by
  a mutation that treats the root as enclosing nothing.
- C2. A `goal` or `register` reference of a nested or sibling Component, and every other kind, is
  refused in a goals home: a unit test; refuted by a mutation that drops the kind test.
- C3. The same references outside a goals home are untouched: a unit test over a design home;
  refuted by a mutation that applies the rule to every file.
- C4. The finding names the span and the repair, and its repair never says to remove the backticks:
  a unit test on the finding's text.
- C5. A goals home in the directory shape is judged like a file home, its `README.md` included: a
  unit test; refuted by a mutation that drops the directory arm.
- C6. The planted mock reports the planted citation, and `cargo klarch check` passes on this
  repository after the mechanical edit: `mock_projects::planted::every_check_has_a_row_and_the_run_is_their_sum`
  and the repository's check.
- C7. An id lint's finding in a goals home names the repair of C4, not a reference form: a unit
  test; refuted by a mutation that drops the goals-home arm of the id lint's repair.

Each claim's test is shown to fail with `cargo x mutate`, per `skill@knowledge-architect-development`.
Fails alone on: a finding in a goals home of this repository or of a mock other than `planted`.

### Step 2: the goal standard and the goal-setting skill

Builds: the fragment rewritten to "Decided design", rules 1 to 8 numbered as today; in the skill, the
drafting, the boundary block by layer, the goal entry shape and its template, the sentence on where a
goal is cited and its `%%` comment, both stops and the record's columns; the `%%` comments citing
the heads, written as pointers; `cargo klarch install-agent-skills` run and the installed copies
committed; the existing `Next release` Migration entry on rule 6 edited to announce the revised
standard, citing `skill@knowledge-architect-goal-setting@whole-set-review`, since the rule it
announces was added since the last release; a Workflow entry for the reviewer's grades and the stops.

Claims:

- C8. The fragment states each rule of "Decided design", rules 1 to 8 numbered as today: read against
  this spec by the agent-facing-text review.
- C9. The skill states both stops of "Decided design" and the record's columns: read by the
  agent-facing-text review.
- C10. The installed copies equal the shipped text: the existing check of
  `path@core@src/check/agents.rs`.

Fails alone on: a host's installed copy differing from its source.

### Step 3: the goal reviewer

Builds: the grades beside the route tags, by the mapping table, and the reach finding, in
`path@agent-skills@content/agents/goal-reviewer.md`; installed.

Claims:

- C11. The reviewer's report section states the grade and the route of every row of the mapping
  table, and no stop: read by the agent-facing-text review.

Fails alone on: the reviewer's grades disagreeing with the skill's stops.

### Step 4: the harvest, the review before the merge, and the spec's deletion

The harvest rows below; then the review of `skill@knowledge-architect-review`, which is the review
before the merge, as `skill@knowledge-architect-planning@working-a-slice` point 6 says, with its
repairs; then the record of step 5's review, `path@knowledge-architect@docs/goal-reviews/2026-11-03-goal-standard.md`,
which carries what step 5 must read and do once this spec has left; then this spec's deletion in a
commit of its own. The branch does not merge.

### Step 5: rounds of the review of the whole goal set

`skill@knowledge-architect-goal-setting@whole-set-review` over this repository's goals, under the new
standard, in as many rounds as its stop takes, with the owner's ruling on every goal. It brings each
goal to the standard, the two the mechanical edit left changed in meaning among them. It reshapes
`goal@core@relocation-is-one-manifest-edit`, on the owner's word of round 5 on Q2, #arg-34: a
relocation is a change of purpose, which the goals carry, or a move that leaves the checker passing,
with a few mechanical edits tolerated. It closes
`issue@agent-skills@goals-name-built-parts-until-the-whole-set-review` in the commit that writes the
goals the owner ruled last. Its record holds the rest of what it reads. It reads `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`,
`issue@mutate@a-mutation-of-several-files`, `design@mutate@one-file-per-run`,
`issue@git@tools-run-git-past-the-wrapper` and the three issues the search graded as touching the
goals. Then the review before the merge of the whole branch, on every axis
`skill@knowledge-architect-review@review-axes` gives its diff, and the merge.

## Order rationale

- Step 1 before step 2: the stricter checker is the branch's first commit, per `primer@branch-commits`.
- Step 2 before step 3: the reviewer grades by the rules step 2 writes.
- Step 3 before step 4: the harvest records what steps 1 to 3 built.
- Step 4 before step 5: the goals are reviewed under the standard as the merged text states it, and
  the owner asked for the review of the whole set after the review before the merge.

## Defaults awaiting the owner

Q1 to Q5 are ruled, round 5. Put to the owner after the review before the merge, and ruled in
round 6:

- **D1**, #review-stops-on-blocking-zero: the whole-set stop's fifth, argued on the old count,
  applied to the new count. The owner: "we will observe the whole set review and decide after. I
  lean that blocks should probably required to reach 0. Practical use will tell us if that is
  achievable." The fifth stays as the session's default, awaiting the owner's ruling after step 5.
- **D2**, #goal-cites-goals-upward-only: how a consumer brings a goal the check refuses to the rule.
  The owner: "through a whole set review". Ruled: the Migration entry and the finding's repair send
  the rewording through goal-setting.
- **D3**, #goal-cites-goals-upward-only: whether the setup skill lists the check as an ask. The
  owner: "i think asks from checks do not need listing in setup skill, because the checks themselves
  report the ask to consumers already." Ruled: nothing is listed; the argument bears on the measure
  of `goal@knowledge-architect@serves-one-kind-of-project`, which step 5 reviews.

## Harvest

| when | what | where |
| --- | --- | --- |
| step 1 | #goal-cites-goals-upward-only → `design@core@goals-home-cites-upward-only`, and the narrowing of `design@core@no-citation-direction-rule` | `register@core@design`, done |
| step 4 | the hosting of the standard, and rule 7 citing the core head | `design@agent-skills@goal-standard-rules`, rewritten, citing the grounding document |
| step 4 | #goal-outcome-survives-empty-skeleton, #goal-one-meaning-any-breadth, #requires-admits-rules-out, #goal-drafted-to-move-the-tree, #goal-met-in-tree-or-in-use, and rules 4 and 8 | one head each, under the thread's slug where it has one: `design@agent-skills@goal-outcome-survives-empty-skeleton`, `design@agent-skills@goal-one-meaning-any-breadth`, `design@agent-skills@requires-admits-rules-out`, `design@agent-skills@goal-drafted-to-move-the-tree`, `design@agent-skills@goal-met-in-tree-or-in-use`, `design@agent-skills@goal-holds-one-intent`, `design@agent-skills@goal-exclusion-names-its-design` |
| step 4 | #goal-finding-grades, #reach-finding-asks-not-narrows | `design@agent-skills@goal-finding-grades`, `design@agent-skills@reach-finding-asks-not-narrows`; `design@agent-skills@goal-reviewer-agent` keeps the reviewer's modes |
| step 4 | #review-stops-on-blocking-zero → `design@agent-skills@goal-review-stops-on-old-wording`, since the thread's slug names a stop on zero, and the whole-set stop in `design@agent-skills@whole-set-rounds` | both, citing the grounding document |
| step 4 | #goal-carries-reading-examples | `design@agent-skills@goal-carries-reading-examples`; `design@agent-skills@goal-entry-shape` and `design@agent-skills@goal-draft-shows-reach` rewritten, and `design@agent-skills@goal-owner-statements-go-to-the-reviewer` split from the latter |
| step 4 | the losing alternatives and the statements the rewrite replaced | judged under `skill@knowledge-architect-decision-recording`: `rejected@agent-skills@goal-review-stop-over-all-wording` and `rejected@agent-skills@goal-universal-names-its-set` |
| step 4 | P1, P2, P3, P4 and P6 | judged as tripwires of `register@agent-skills@tripwire` |
| step 4 | `the-goal-standard-lets-a-goal-name-what-is-built-or-cite-an-entry` and `a-goal-the-draft-review-read-was-reworded-again-for-a-defect` | closed; the first's whole-set half restored as `issue@agent-skills@goals-name-built-parts-until-the-whole-set-review`, closed at step 5 |
| step 4 | the summary of `issue@agent-skills@a-target-project-collects-no-feedback-from-use` | rewritten to the new rule 6 |
| step 1 and step 2 | #goal-standard-rereview-here | the Migration entries in `path@knowledge-architect@CHANGELOG.md` |

## Later consequences

- Step 5 rewords this repository's goals; each reworded goal is ruled by the owner, and the branch
  merges after it.
- A consumer's pin move across the release meets the new check and the Migration entry's review of
  the whole goal set.
- `issue@agent-skills@a-target-project-collects-no-feedback-from-use` stays open: rule 6 asks an
  outcome judged in use to be measured by what learns from use, which raises the stakes of what setup
  asks of a target project.
