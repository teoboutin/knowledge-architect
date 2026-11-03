# The goal standard asks for wide outcomes that name no built part, the goal reviewer asks the owner about reach instead of narrowing, and a goals home cites only goals and registers of its own or an enclosing Component

## Status and audience

This spec is the plan of the work that revises how a goal is written and reviewed. The work does
four things:

- the goal standard, `path@agent-skills@fragments/goal-standard.md`, is rewritten: a goal states an
  outcome that keeps its meaning in a project emptied of its design and its code; its rule of one
  reading judges ambiguity, never breadth; it reads a goal in three layers, what it requires, what
  it admits and what it rules out; and it sorts an outcome judged in the tree from one judged in use;
- `skill@knowledge-architect-goal-setting` drafts goals that are expected to be unmet, writes "for
  instance" cases per layer into each goal, and shows the owner the reach of a draft by layer;
  `agent@knowledge-architect-goal-reviewer` grades its findings `blocks`, `asks` or `minor`, asks the
  owner about a goal's reach instead of proposing a narrower wording, and its reviews stop on a
  count of the findings left on wording the previous round also read;
- the checker refuses, in a goals home, every reference that is not a goal or a register's home of
  the home's own Component or of a Component that encloses it;
- the goals of this repository lose, by deletion alone, the citations that check refuses, and are
  then brought to the new standard by a review of the whole goal set.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner's word is awaited,
  `skill@knowledge-architect-planning@working-a-slice` says what waits.
- **The work is one spec and one branch, carried in the session that converged, on the owner's
  word.** The owner, round 4: "The work should be one spec and one branch carried in this session."
  Step 1 makes the checker stricter, so its commit is the branch's first commit, per
  `primer@branch-commits`: the commits written before it on the branch, the grounding document and
  this spec, are moved after it by a history edit with a clean tree, as the root `CLAUDE.md`,
  `instructions@git-workflow` point 2, says. That departs from the planning skill's rule that a spec
  whose work's first commit changes the per-commit gate is merged before that work, on the owner's
  word for this work.
- **After the review before the merge, the branch does not merge.** The owner, round 4: "After
  implementation, we will run the full review before merge, but not merge, and instead proceed to a
  full set goal review." The review of the whole goal set, step 5, runs on the branch, and the branch
  merges after it.

The discussion ran in one session. It begins at the owner's message "I would like to rediscuss the
goal standard and goal setting skill. We are splitting the project in smaller components, setting up
new goals in the meantime, and I am observing various issues."

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, taken once for the whole spec, as its last
paragraph says.

## Names

| name | what it names |
| --- | --- |
| the goal standard | `path@agent-skills@fragments/goal-standard.md`, the fragment hosted by `skill@knowledge-architect-goal-setting@goal-or-not` and `agent@knowledge-architect-goal-reviewer@the-goal-standard` |
| rule 1 to rule 8 | the numbered rules of the goal standard, in its order |
| the boundary block | what `skill@knowledge-architect-goal-setting@drawing-out-intent` step 5 shows the owner beside each drafted goal |
| a goals home | the home of the `goal` register of an anchor, `register@*@goal`: a file, or a directory behind a `README.md` |
| a draft-mode round | one dispatch of the goal reviewer in draft mode over a batch, per `skill@knowledge-architect-goal-setting@drawing-out-intent` step 4 |
| a whole-set round | one round of `skill@knowledge-architect-goal-setting@whole-set-review` |
| old wording | the words of a goal that the previous round of the same review also read, unchanged since; the rest is new wording |
| the reference check | `judge_part` in `path@core@src/check/references.rs`, which judges every span of every document |
| the mechanical edit | step 1's removal of the citations the new check refuses from this repository's goals homes, by deletion of words alone |

## What the work is

**The goal standard today.** Eight rules, hosted by two texts. Rule 1 makes a kind of interface an
outcome and its exact form a decision. Rule 2 asks one reading, and that every "every", "no", "all"
and "not in scope" names the set it ranges over. Rule 3 asks that a goal rule out nothing the owner
has not ruled out knowingly. Rule 5: "No state of the project meets one goal only by failing
another. Where two outcomes compete, a goal states which ranks first." Rule 6 asks that what shows a
goal met be "an observable of the tree as it stands", and, for an outcome judged only in use, the
mechanism the project builds to pursue it and learn from use. No rule limits what a goal names or
cites.

**What the discussion found the standard produces.** Every repair a finding asks adds words: rule 2
adds the set a word ranges over, rule 3 adds an exception, rule 6 pulls the measure toward what
exists. A narrowing repair of a rule-3 finding rules out cases the owner never ruled out, which is a
new rule-3 finding at the next round. The root goals home holds 19 goals in 6215 words, taken with
`wc -w docs/goals.md` at the commit this spec is written on. The draft-mode loop is recorded in
`issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`: four instances,
one batch reviewed five times with 8, 10, 6, 6 and 6 findings. The non-convergence of the review of
the whole goal set is recorded in `design@agent-skills@whole-set-rounds`: 91 then 88 findings.

**The goal reviewer today.** `path@agent-skills@content/agents/goal-reviewer.md` tags each finding
`settled`, `intent` or `contradiction`, and also `minor` for a finding of rule 4 or of rule 6 but
one case. The draft-mode review has no stop. The whole-set review stops on the owner's word or when
a round's count of findings that are not `minor` falls to a fifth of the first round's.

**The goal-setting skill today.** `path@agent-skills@content/skills/goal-setting/SKILL.md`. Its
boundary block shows, for each universal or exclusion, a case it rules out and a case it admits,
and is never written to the goals home. Its section "The shape of a goal entry" gives a template
with no example lines.

**The checker today.** The reference check resolves every reference and judges no direction:
`design@core@no-citation-direction-rule`. `Anchors::owning` and `Anchors::home` in
`path@core@src/entity.rs` give a file's owning anchor and an anchor's home of a register.

**This repository's goals homes today.** The citations the new check refuses, found with
`grep -noE` over the seven goals homes for every backticked span that is not a goal or a register's
home of the home's own or an enclosing Component, at the commit this spec is written on:

| file | line | span |
| --- | --- | --- |
| `register@knowledge-architect@goal` | 19 | `design@agent-skills@goal-lifecycle` |
| `register@knowledge-architect@goal` | 87, 138 | `goal@agent-skills@one-skill-per-activity` |
| `register@knowledge-architect@goal` | 109, 133, 420, 487 | `goal@agent-skills@only-needed-instructions` |
| `register@knowledge-architect@goal` | 132, 142 | `goal@agent-skills@no-contradiction-in-workflow` |
| `register@knowledge-architect@goal` | 156 | `goal@gates@gates-from-a-list` |
| `register@agent-skills@goal` | 11 | `skill@knowledge-architect-agent-configuration@where-text-goes` |

No mock project's goals home under `path@core@tests/projects/` holds a reference. Every register reference
span in a goals home names a register of its own Component or of the root.

**Texts that cite the standard's rules by number**: `path@agent-skills@content/skills/planning/SKILL.md`
and `path@agent-skills@content/agents/transcript-reviewer.md` cite "rule 3 of
`skill@knowledge-architect-goal-setting@goal-or-not`", and the goal reviewer cites "rules 1, 2 and 4
to 8". The numbering is kept, so these stay true, per
`issue@core@no-checked-form-cites-a-point-inside-a-section`.

**Outside the work:**

- what setup asks of a target project for feedback from use:
  `issue@agent-skills@a-target-project-collects-no-feedback-from-use`, a design session of its own;
- the purpose each goals home's head states:
  `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`, which step 5's
  review of the whole goal set reads;
- rewording any goal of this repository to the new standard: step 5, under goal-setting, with the
  owner's ruling on each goal; step 1 deletes words only.

## What is already decided

**The decisions the design rests on, not argued again:**

- `design@agent-skills@shared-text-fragment`: the standard stays one fragment with its two hosts;
  text only the reviewer applies, the grades and the stop rules, stays outside the fragment.
- `design@agent-skills@goal-set-review-in-goal-setting`, `design@agent-skills@goal-batch-ruled-in-the-tree`,
  `design@agent-skills@goal-edit-ruled-alone`, `design@agent-skills@goals-change-through-goal-setting`
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
  a check added is a minor change; the change of what recorded goals must meet owes a Migration
  entry that cites `skill@knowledge-architect-goal-setting@whole-set-review`, per
  `design@agent-skills@audit-triggers`.

**The decisions the work reverses or rewrites, with every text that references them:**

| decision | change | texts referencing it, and who updates them |
| --- | --- | --- |
| `design@core@no-citation-direction-rule` | narrowed: no direction is checked outside a goals home | referenced by nothing; the harvest rewrites it |
| `design@agent-skills@goal-standard-rules` | rewritten to the new rules | the goal-setting skill's `%%` comment, step 2; the heads of the decisions below, the harvest |
| `design@agent-skills@goal-reviewer-agent` | the grades and the draft-mode stop | the reviewer's `%%` comments, step 3; `design@agent-skills@goals-axis-in-the-review-before-a-merge`, the harvest; `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`, closed at the harvest |
| `design@agent-skills@goal-draft-shows-reach` | the block by layer; "The block never reaches the goals home" reversed for the "for instance" cases | the skill's `%%` comment, step 2; the harvest |
| `design@agent-skills@whole-set-rounds` | the stop counts `blocks` and `asks` on old wording | the skill's `%%` comment, step 2; the harvest |
| `design@agent-skills@goal-entry-shape` | the body carries "for instance" cases by layer | the skill's `%%` comment, step 2; the harvest |

**The entries the discussion's search graded, with their outcome.** The search ran with the drafted
shapes as its work and these seeds: `design@agent-skills@goal-standard-rules`,
`design@agent-skills@goal-reviewer-agent`, `design@agent-skills@goal-draft-shows-reach`,
`design@agent-skills@whole-set-rounds`, `design@agent-skills@goal-entry-shape`,
`design@agent-skills@goal-batch-ruled-in-the-tree`, `design@core@an-entity-belongs-to-its-anchor`.
It graded 116 entries `bears` and 97 `may-bear`.

- **Every goal of this repository**: `bears`, may change, through step 5. Those that constrain the
  work: `goal@knowledge-architect@the-owner-decides` and `goal@knowledge-architect@serves-one-kind-of-project`,
  under which the new check, an ask that binds every target project, is knowledge-architect's
  owner's to add, which the owner did in round 4;
  `goal@agent-skills@only-needed-instructions`, which forbids "a fixed sequence, count or mapping
  between a unit of the work and a unit of its record that another instruction needs to vary", so a
  goal carries one or more "for instance" cases for each layer it has, not a fixed count;
  `goal@knowledge-architect@agents-work-without-drift`, which ranks "a goal's exclusion that names the
  design it excludes" above itself, in agreement with "the exclusion wins";
  `goal@knowledge-architect@the-workflow-improves-through-real-use`, whose evidence for this work is
  the real use recorded in the issue on the draft-mode loop and in `design@agent-skills@whole-set-rounds`.
- `rejected@core@citation-direction-checked`: may bring back. It is not brought back: it rejected a
  one-way rule over every citation, whose only gain was acyclicity for the record search's clusters,
  and which would fail goal-to-goal refinements. The new rule covers goals homes alone, for another
  reason, and admits a goal citing a goal it refines.
- `rejected@agent-skills@goal-measured-by-evidence-from-use`: may bring back. It is not brought back:
  rule 6 keeps the mechanism the project builds as the measure, not the evidence.
- `rejected@agent-skills@owner-intent-entry-test`: may bring back. Not brought back: the work moves no
  intent out of the goals, and its doubt, goals homes swelling with local preferences, is answered by
  the skeleton test, which removes the words that tie a goal to a mechanism.
- `design@agent-skills@a-reference-claims-a-revisit`: bears, no change: under the skeleton test a goal
  never has to be revisited when a design head or a skill changes, so the primer's rule asks for no
  such reference.
- `design@agent-skills@transcript-reviewer-agent` and `design@agent-skills@misstated-ruling-rated-by-what-it-changes`:
  bear; they already rate a goal recorded stricter than the owner ruled out knowingly as major, which
  the work supports.
- `design@agent-skills@convergence-count-scope`: may-bear, read: its mechanism, repairs writing new
  text that agents find defects in, grounds the count on old wording.
- `issue@mutate@a-mutation-of-several-files` and `issue@git@tools-run-git-past-the-wrapper`:
  may-bear, read: each reads what a goal "admits" or "can" as what it requires; under the three
  layers that reading is asked of the owner at step 5. The owner accepted that existing records are
  reclassified then, in round 3.
- `design@agent-skills@intermediate-component-occasion`: bears, constrains: a nested Component's goal
  refines its intermediate Component's, which the upward citation rule admits.
- `design@core@a-commit-message-is-a-document`, `design@agent-skills@branch-commits-judged-at-tip`:
  bear; they place step 1 first on the branch.
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: bears, worsens unless
  the new finding names its repair: step 1's claim C4.
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: bears: the rules keep their numbers.
- `issue@agent-skills@the-goal-standard-lets-a-goal-name-what-is-built-or-cite-an-entry`: closed by
  the work, at the harvest.
- Every other entry graded `bears` constrains how installed text, the checker or the changelog is
  written, and is applied by the step that writes it: the heads of the admission standard
  (`design@agent-skills@statement-names-an-act`, `design@agent-skills@capabilities-not-structure`,
  `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@root-cause-repair`,
  `design@agent-skills@audit-net-growth`, `design@agent-skills@repairs-read-against-the-record`,
  `design@agent-skills@term-defined-at-first-load`, `design@agent-skills@admission-rules-home`,
  `design@agent-skills@instruction-record-is-minimal`), the heads on heads
  (`design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@title-states-the-rule`,
  `design@agent-skills@one-decision-per-head`, `design@agent-skills@existing-heads-on-touch`,
  `design@agent-skills@design-home-write-loads-recording`, `design@agent-skills@thread-slug-is-entry-id`),
  and `design@core@section-homes-carry-slugs`, `design@core@grammars-not-prefixes`,
  `design@core@an-entry-is-a-heading-at-the-register-level`, `design@core@owned-namespace-check`,
  `design@knowledge-architect@the-changelog-ships-in-every-crate`.
- **`may-bear` entries not read**: every entry of the search's `may-bear.md` other than the four read
  above, each with the searcher's reason in that file. The design-conformance review reads them.

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

Weighed. Unmet and accepted on the owner's word, round 4: this repository's 37 goals go through a
review of the whole goal set, and a consumer's goals homes fail the new check until their citations
are rewritten, which a Migration entry names.

## Threads

### A goal keeps its meaning in a project emptied of its design and its code `##goal-outcome-survives-empty-skeleton`

Proposed by the owner, round 1; shaped by the session, round 1. Approved: presumed settled by the
owner's "I think we fully align on the conclusions and the proposals here." (round 2), closed by the
owner's word on the checkpoint table, round 4: "agreed on the table." Arguments: #arg-1, #arg-6,
#arg-7, #arg-12. Shape: "Decided design", rule 1. Harvest: `design@agent-skills@goal-standard-rules`.

### A goals home cites only goals and registers of its own Component or of an enclosing one, and the checker enforces it `##goal-cites-goals-upward-only`

Proposed by the owner, round 1; the checker rule proposed by the session, round 1; widened by the
owner, round 3. Approved, round 4: "goal-cites-goals-upward-only accepted, this is the shape I want".
The owner's words, round 3: "The checker rule holds, and the core head should be updated." and "I
think citing design heads should be forbidden in all goal docs. There is no argument for citing
design@agent-skills@goal-lifecycle in root goals. A consumer project cannot do that in particular,
and its goal lifecycle is only grounded in what the goal setting skill says. Citing register should
be allowed upward, the same way it is for citing upward goals." Arguments: #arg-2, #arg-8, #arg-13,
#arg-14, #arg-15. Shape: "Decided design", the check and rule 7. Harvest: a new head in
`register@core@design` and `design@core@no-citation-direction-rule` narrowed;
`design@agent-skills@goal-standard-rules`.

### Rule 2 judges ambiguity, never breadth `##goal-one-meaning-any-breadth`

Proposed by the owner, round 1. Approved, closed by the checkpoint word of round 4. Arguments:
#arg-3, #arg-9. Shape: "Decided design", rule 2. Harvest: `design@agent-skills@goal-standard-rules`.

### A goal is read in three layers, requires, admits and rules out, and only ruling out what another requires is a contradiction `##requires-admits-rules-out`

Proposed by the owner, round 1. Approved for the reclassification of existing records, round 3:
"Reclassifying existing records is accepted (point 4)", and closed by the checkpoint word of
round 4. Arguments: #arg-4, #arg-16, #arg-17, #arg-21. Shape: "Decided design", rules 3 and 5.
Harvest: `design@agent-skills@goal-standard-rules`.

### A goal carries "for instance" cases for each layer it has `##goal-carries-reading-examples`

Proposed by the owner, round 1. Approved with the session's default of round 2, one or more cases
per layer the goal has, closed by the checkpoint word of round 4. Arguments: #arg-5, #arg-18,
#arg-19. Shape: "Decided design", the goal entry. Harvest: `design@agent-skills@goal-entry-shape`,
`design@agent-skills@goal-draft-shows-reach`.

### A new or reworded goal is drafted to move the tree, and is expected to be unmet `##goal-drafted-to-move-the-tree`

Proposed by the owner, round 1. Approved, closed by the checkpoint word of round 4. Arguments:
#arg-10, #arg-20. Shape: "Decided design", drafting. Harvest: `design@agent-skills@goal-standard-rules`.

### Rule 6 sorts an outcome by whether it depends on what the project does not control `##goal-met-in-tree-or-in-use`

Proposed by the owner, round 1. The wording of its test approved with the survey's defaults,
round 3: "I agree with your defaults proposals for the S items from the survey." Arguments: #arg-11,
#arg-22. Shape: "Decided design", rule 6. Harvest: `design@agent-skills@goal-standard-rules`.

### A finding on a goal's reach lists the cases by layer and asks the owner, and the drafter narrows only on the owner's word `##reach-finding-asks-not-narrows`

Proposed by the owner, round 1. Approved, closed by the checkpoint word of round 4. Arguments:
#arg-4, #arg-12, #arg-17. Shape: "Decided design", the reviewer and the drafting. Harvest:
`design@agent-skills@goal-reviewer-agent`, `design@agent-skills@goal-draft-shows-reach`.

### The goal reviewer grades each finding `blocks`, `asks` or `minor` `##goal-finding-grades`

Proposed by the owner, round 1. Approved, closed by the checkpoint word of round 4. Arguments:
#arg-23. Shape: "Decided design", the reviewer. Harvest: `design@agent-skills@goal-reviewer-agent`.

### A review stops when no `blocks` finding is left on old wording `##review-stops-on-blocking-zero`

Proposed by the session from the issue on the draft-mode loop, round 1; the count on old wording
from the record search and the survey, round 2, approved, round 3: "I agree with your defaults
proposals for the S items from the survey." Arguments: #arg-24, #arg-25. Shape: "Decided design",
the stops. Harvest: `design@agent-skills@goal-reviewer-agent`, `design@agent-skills@whole-set-rounds`.

### This repository's goals are brought to the new standard by a review of the whole goal set, after a mechanical edit `##goal-standard-rereview-here`

Proposed by the session, round 1. Approved, round 4, with the owner's shape for the first edit: "The
initial fix of the goals can be considered a mechanical edit of it, and be done under no review and
no word from me. It should just focus on getting rid of citations that will be linted without
rewording sentences (a pure mechanical edit). The goal whole set review will then bring the goals
into shape after the work, and de-narrow them when relevant." Arguments: #arg-26. Shape: "Decided
design", the mechanical edit and step 5. Harvest: the Migration entry of `path@knowledge-architect@CHANGELOG.md`.

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

Round 2, the record search: every goals home's preamble cites `register@<anchor>@<register>`, and five goals and the
root preamble hold citations the check refuses; a stricter checker goes in the branch's first
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

## New names, in one place

```text
crates/core/src/check/references.rs   a check, within the reference check, of each reference
                                      written in a goals home (name chosen at step 1)
crates/core/tests/projects/planted/    one planted citation the new check refuses
fragments/goal-standard.md            existing; rewritten
grades `blocks`, `asks`, `minor`      the goal reviewer's grades
layers Requires, Admits, Rules out    the labels of the boundary block and of a goal's cases
```

## Decided design

### Rule 1: the skeleton test

A goal states an outcome the owner intends, and keeps its meaning, word for word, in the project
reduced to an empty skeleton, every design head, document and line of code removed. It may define
what the project provides and then name it, as "the project provides a command that checks its
documents". It names no part whose existence comes from a design choice. A Component's goal may
name its own Component and the Components that enclose it. The kind of interface the owner intends
stays an outcome; its exact form stays a decision, as rule 1 says today. An exclusion of rule 8
describes the design it excludes without naming a built part. Nearest rival: the test "every word
under the goal replaced, same function", the owner's other wording of round 1; it follows from the
skeleton test, which a reviewer applies directly by deleting the project and reading the sentence.

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
exclusion holds. "Where two outcomes compete, a goal states which ranks first" is kept, and covers an
outcome that competes with another under a condition, which the reviewer reports with the
condition, graded `asks`. The rules keep their numbers. Nearest rival: two layers, which reads an
admitted case as a requirement, the reading of #arg-21.

### Rule 6: in the tree or in use

The test that sorts an outcome: **does it depend on anything the project does not control, such as
its users, their data or their environment?** If not, what shows it met is a check or a test over
what the project builds. If so, what shows it met is what the project builds to pursue it and to
learn from use whether it is met, as rule 6 says today, never a list of the properties the outcome is
taken to consist of. Nearest rival: a sort by kind of outcome, correctness against effects; the KAOS
sort by who can enforce it states the owner's case more directly, #arg-22.

### Rule 7: citations

A refinement neither widens nor contradicts the goal it refines, as today. A goals home, its goals
and its head alike, cites only a goal or a register's home of its own Component or of a Component
that encloses it. The checker enforces this.

### Drafting

`skill@knowledge-architect-goal-setting@drawing-out-intent` drafts a new or reworded goal to move
what the project builds: it is expected to be unmet, and is never shaped so that the tree as it
stands meets it. Its boundary block states what the goal asks of future work, and shows its reach by
layer: for each layer the goal has, the cases it reaches that the owner may not intend, from the
project's existing or planned work, or from the uses the owner's statements describe.

### The goal entry

The body of a goal states what it means and what shows it met, as today, and carries one or more "for
instance" cases for each layer the goal has, labelled Requires, Admits and Rules out; a layer the goal
does not have is omitted. A case passes the skeleton test, and binds nothing: it shows a reading. The
labels are not level-two headings, per `design@core@an-entry-is-a-heading-at-the-register-level`.

### The reviewer

- **A reach finding** lists the cases the goal reaches, by layer. A case it only admits is no
  finding. A case it requires or rules out that the owner's statements do not cover is put to the
  owner. The drafter keeps the wording by default, and drafts a narrower one only on the owner's
  word, showing what the narrowing rules out.
- **Grades**, by what a finding changes in an agent's act: `blocks`, a contradiction under rule 5, a
  requirement or an exclusion the owner did not state, a built part named; `asks`, a reach in the
  requires or rules-out layer, a term with two meanings, a divergence; `minor`, an overlap, a case
  only admitted, the wording of a measure. Each finding keeps the route it has today: repaired by the
  drafter where the standard decides it, put to the owner where it is intent or a contradiction.
- **The stops.** A draft-mode review of a batch stops when a round leaves no `blocks` finding on old
  wording; its other findings go to the owner beside the goals, unrepaired. The whole-set review
  stops on the owner's word or when a round's count of `blocks` and `asks` findings on old wording
  falls to a fifth of the first round's; the round that meets the stop writes no repair, as today.
- The grades and the stops are the reviewer's and the skill's, outside the fragment, per
  `design@agent-skills@shared-text-fragment`.

### The check

In the reference check, every reference written in a goals home, its entries and its head alike, is
refused unless it is a `goal` reference or a `register` reference whose anchor is the home's own
Component or a Component whose directory encloses it, the root included. A harness reference,
`skill@<name>`, `agent@<name>`, `primer@<slug>` or `instructions@<slug>`, and every other kind, `design`, `issue`,
`path` and the rest, is refused there. The finding names the span, the home, and the repair: cite a
goal of the home's own or an enclosing Component, or rewrite the sentence so it names no entry;
never remove the backticks. The check reads the anchors and the entity table only, and holds in every
project with no flag. Nearest rival: a rule judged by the goal reviewer alone, which the check makes
cheaper and certain, per `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

### The mechanical edit

Step 1 removes each citation of the table under "What the work is" by deleting words, adding none:
the clause that holds the span, such as ", per `<reference>`" or ", as `<reference>` measures"; or,
where the span is the whole object of a sentence's verb, the sentence. On the owner's word, round 4,
it takes no review and no word of the owner, and the review of the whole goal set brings the goals
into shape afterwards.

## Mapping tables

The mechanical edit, over every citation the check refuses in this repository:

| site | deleted |
| --- | --- |
| root, line 19 | ", per `design@agent-skills@goal-lifecycle`" |
| root, line 87 | ", among those `goal@agent-skills@one-skill-per-activity` lists" |
| root, line 109 | the sentence "What the step says, beyond what the session's judgement handles, follows `goal@agent-skills@only-needed-instructions`." |
| root, lines 132-133 | ", as `goal@agent-skills@no-contradiction-in-workflow` states" and ", per `goal@agent-skills@only-needed-instructions`" |
| root, line 138 | ", as `goal@agent-skills@one-skill-per-activity` measures," with the comma kept as the list needs |
| root, line 142 | ", as `goal@agent-skills@no-contradiction-in-workflow` measures" |
| root, line 156 | ", per `goal@gates@gates-from-a-list`" |
| root, line 420 | ", per `goal@agent-skills@only-needed-instructions`" |
| root, line 487 | " in the form `goal@agent-skills@only-needed-instructions` admits" |
| agent-skills, line 11 | ", as `skill@knowledge-architect-agent-configuration@where-text-goes` uses the term" |

The line numbers are those of the commit this spec is written on.

## Losing alternatives

- **Every word under the goal replaced, same function**, lost to the skeleton test, which it
  follows from, under #goal-outcome-survives-empty-skeleton.
- **The citation rule judged by the goal reviewer alone**, lost to the check, under
  #goal-cites-goals-upward-only.
- **A fixed count of one example line per layer**, lost to one or more cases per layer the goal has,
  on `goal@agent-skills@only-needed-instructions`, under #goal-carries-reading-examples.
- **A draft-mode stop on no `blocks` finding over all wording**, lost to the count on old wording, on
  `design@agent-skills@convergence-count-scope`, under #review-stops-on-blocking-zero.
- **A separate layer of measures agents revise** (OKR key results), lost to `primer@goals-bind`:
  the measure stays stated as a mechanism inside the goal, under #goal-met-in-tree-or-in-use.
- **An accepted-risk layer in a goal** (commander's intent), lost to rule 5's ranking and a design
  head's costs, under #requires-admits-rules-out.
- **A new rule for a divergence** (KAOS), absorbed by rule 5's ranking sentence, under
  #requires-admits-rules-out.

`path@agent-skills@docs/grounding/goal-standard.md` holds the survey these come from.

## Readings

None: the work implements no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| P1. Goals become too wide to constrain anything, and forks to the owner multiply | #goal-one-meaning-any-breadth | dropped: the owner sees it in the fork questions |
| P2. `asks` lists grow long, and the owner narrows to shorten them | #reach-finding-asks-not-narrows | dropped: the owner sees it while ruling |
| P3. Reviewers read "whose existence comes from a design choice" two ways | #goal-outcome-survives-empty-skeleton | dropped: it surfaces as reviewer findings the owner reads |
| P4. A "for instance" case becomes the measure: an issue or a head declares a goal unmet, or a design required, from a case alone | #goal-carries-reading-examples | named as a cause for the harvest to judge as a tripwire |
| P5. A consumer strips the backticks to clear the new finding | #goal-cites-goals-upward-only | a claim of step 1: C4 |
| P6. The reclassification of "admits" closes an issue whose work the owner wanted | #requires-admits-rules-out | dropped: the owner rules each one at step 5 |

## Acceptance criteria

None. P4, the one cause that is silent and lands in the tree, is judged at the harvest as a cause,
since only later work, not this work's code, can show it.

## Implementation sequence

### Step 1: the check, the core head, and the mechanical edit

Builds: the check of "Decided design", in `path@core@src/check/references.rs`; a planted citation in
the `planted` mock; the mechanical edit; `design@core@no-citation-direction-rule` narrowed and the new
core head written, per `skill@knowledge-architect-decision-recording`, since the check's commit must
carry its decision; the CHANGELOG entry of the check.

Claims:

- C1. A `goal` or `register` reference of the home's own Component or of an enclosing one passes in
  a goals home, in its entries and in its head: a unit test over a two-level anchor list; refuted by
  a mutation that treats the root as enclosing nothing.
- C2. A `goal` or `register` reference of a nested or sibling Component, and every other kind, the
  harness kinds included, is refused in a goals home: a unit test per kind; refuted by a mutation that
  drops the kind test.
- C3. The same references outside a goals home are untouched: a unit test over a design home;
  refuted by a mutation that applies the rule to every file.
- C4. The finding names the span and the repair, and its repair never says to remove the backticks:
  a unit test on the finding's text.
- C5. A goals home that is a directory behind a `README.md` is judged like a file home: a unit test
  over the directory shape.
- C6. `cargo klarch check` passes on this repository after the mechanical edit, and the planted mock
  reports the planted citation: the existing binary tests over `planted` and the repository's check.

Each claim's test is shown to fail with `cargo x mutate`, per
`skill@knowledge-architect-development`. Fails alone on: a finding in a goals home of this repository
or of a mock other than `planted`.

### Step 2: the goal standard and the goal-setting skill

Builds: the fragment rewritten to the rules of "Decided design"; the skill's drafting, boundary block
and goal entry shape; its whole-set stop; the `%%` comments citing the heads; `cargo klarch
install-agent-skills` run and the installed copies committed; the CHANGELOG Workflow entry.

Claims:

- C7. The fragment states each rule of "Decided design", rules 1 to 8 numbered as today: read against
  this spec by the agent-facing-text review.
- C8. The installed copies equal the shipped text: the existing check.

Fails alone on: a host's installed copy differing from its source.

### Step 3: the goal reviewer

Builds: the grades, the reach finding, the draft-mode and whole-set stops on old wording, in
`path@agent-skills@content/agents/goal-reviewer.md`; installed.

Claims:

- C9. The reviewer's report section names the three grades and the route of each finding, and its
  modes name the stops: read by the agent-facing-text review.

Fails alone on: the reviewer's text disagreeing with the skill's stop.

### Step 4: the review before the merge, and the harvest

The review of `skill@knowledge-architect-review`, then the harvest rows below, then this spec's
deletion in a commit of its own. The branch does not merge.

### Step 5: the review of the whole goal set

`skill@knowledge-architect-goal-setting@whole-set-review` over this repository's goals, under the new
standard, with the owner's ruling on every goal; it reads
`issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`,
`issue@mutate@a-mutation-of-several-files` and `issue@git@tools-run-git-past-the-wrapper`. Then the
goals axis of the review before the merge, and the merge.

## Order rationale

- Step 1 before step 2: the stricter checker is the branch's first commit, per `primer@branch-commits`.
- Step 2 before step 3: the reviewer judges by the fragment step 2 writes.
- Step 3 before step 4: the review reads the whole revision.
- Step 4 before step 5: the goals are reviewed under the standard as merged text will state it, and
  the owner asked for the review of the whole set after the review before the merge.

## Defaults awaiting the owner

None.

## Harvest

| when | what | where |
| --- | --- | --- |
| step 1 | the check's decision, #goal-cites-goals-upward-only, and the narrowing of `design@core@no-citation-direction-rule` | `register@core@design` |
| step 4 | #goal-outcome-survives-empty-skeleton, #goal-one-meaning-any-breadth, #requires-admits-rules-out, #goal-drafted-to-move-the-tree, #goal-met-in-tree-or-in-use, the citation rule | `design@agent-skills@goal-standard-rules`, rewritten |
| step 4 | #reach-finding-asks-not-narrows, #goal-finding-grades, #review-stops-on-blocking-zero | `design@agent-skills@goal-reviewer-agent` and `design@agent-skills@whole-set-rounds`, rewritten |
| step 4 | #goal-carries-reading-examples | `design@agent-skills@goal-entry-shape` and `design@agent-skills@goal-draft-shows-reach`, rewritten |
| step 4 | the losing alternatives | judged under `skill@knowledge-architect-decision-recording` for `register@agent-skills@rejected` |
| step 4 | P4 | judged as a tripwire of `register@agent-skills@tripwire` |
| step 4 | `issue@agent-skills@the-goal-standard-lets-a-goal-name-what-is-built-or-cite-an-entry` and `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect` | closed |
| step 4 | #goal-standard-rereview-here | the Migration entry in `path@knowledge-architect@CHANGELOG.md`, citing `skill@knowledge-architect-goal-setting@whole-set-review` |

## Later consequences

- Step 5 rewords this repository's goals; each reworded goal is ruled by the owner.
- A consumer's pin move across the release meets the new check and the Migration entry's review of
  the whole goal set.
- `issue@agent-skills@a-target-project-collects-no-feedback-from-use` stays open: rule 6 now asks an
  outcome judged in use to be measured by what learns from use, which raises the stakes of what setup
  asks of a target project.
