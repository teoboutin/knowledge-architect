# Goals live at the project's root only, under a thinner goal standard

## Status and audience

This milestone document is the plan of a change of the goal standard and of where goals live: a
goal becomes a purpose, a scope or a ranking of the project as a whole, stated as a direction, with
no measure and no fixed block of cases, in the root's goals home alone. It holds what crosses the
slices; each slice's spec holds what only that slice builds. It leaves when its last slice has
landed and every default it lists is ruled.

It comes from one design discussion, held in one session, which began at the owner's message "I
want to discuss once more the goal standard and how goals are shaped." The discussion ran 13 rounds
and two searches of the whole record.

- Written for a session that did not witness the discussion.
- Where this document or a slice's spec and a design home disagree on a point "What is already
  decided" does not list as reversed or rewritten, the design home wins, and the disagreement is a
  defect of the plan document.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. Its defaults D1 to D4 are not ruled.

## How the work is done

Read `skill@knowledge-architect-planning@working-a-slice` whole before each slice, then follow it.
This repository adds:

- `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", read whole before any edit
  of the installed text's source, and `cargo x skill <name>` to read a shipped text with its `%%`
  comments;
- `skill@knowledge-architect-goal-setting`, as slice 1 has rewritten it, for every edit of a goals
  home: no goal is edited, moved into a head or removed outside it, per
  `design@agent-skills@goals-change-through-goal-setting`;
- `skill@klarch-development`, with `skill@knowledge-architect-development`, before slice 2's code;
- several sessions may carry slice 1: each resumes from this document, the slice's spec and the
  record committed in `path@knowledge-architect@docs/goal-reviews/`, never from a conversation.

## Names

| name | what it names |
| --- | --- |
| the standard | the goal standard, the fragment `path@agent-skills@fragments/goal-standard.md`, which the goal-setting skill and the goal reviewer host |
| the goal-setting skill | `path@agent-skills@content/skills/goal-setting/SKILL.md`, installed as `skill@knowledge-architect-goal-setting` |
| the goal reviewer | `agent@knowledge-architect-goal-reviewer` |
| a goals home | the home of the register `goal` of an anchor: docs/goals.md of a Component today |
| a Component goal | a goal in the goals home of a Component other than the root |
| a refining goal | a goal whose body holds a line that begins "It refines" |
| the measure | the paragraph of a goal that begins "It is met while" |
| the cases | the list of a goal that begins "Requires, for instance", "Admits, for instance" and "Rules out, for instance" |
| the whole-set review | the review of the whole goal set, `skill@knowledge-architect-goal-setting@whole-set-review`, whose rounds `design@agent-skills@whole-set-rounds` states |
| the first search, the second search | the two searches of the whole record the discussion ran, each by 17 agents of `agent@knowledge-architect-record-searcher` |
| a theme | a set of goals on one subject that touch each other, the set `design@agent-skills@whole-set-rounds` gives one reviewer; one theme file holds one theme. The analysis proposes the themes and the owner rules them with it |
| the reach by layer | what a goal draft requires, admits and rules out, shown to the owner at drafting, per `design@agent-skills@goal-draft-shows-reach` |
| a blocking finding | a finding the goal reviewer grades `blocks`, per `design@agent-skills@goal-finding-grades` |
| the need test | the test of `skill@knowledge-architect-agent-configuration` by which an instruction stays in agent-facing text only where an act needs it |
| the primer's source | `path@agent-skills@content/PRIMER.md` |
| the goal reviewer's source | `path@agent-skills@content/agents/goal-reviewer.md` |
| the decision-record reviewer's source | `path@agent-skills@content/agents/decision-record-reviewer.md` |
| the head-ground fragment | `path@agent-skills@fragments/head-ground.md` |
| the checkpoint table | the table of every thread with its state that the session showed in round 12, which the owner's word of round 13 answered; it is under Threads |

## What the work is

**What exists today.**

- 54 goals in 13 goals homes: 21 at the root, 33 in 12 Components, counted by
  `grep -c '^## '` over each goals home; 19,045 words by `wc -w` over the same 13 files. The
  discussion used 22,350, a figure the session took with a fourteenth file in the sum, the
  goal-setting skill; the review of this document corrected it.
- 24 goals hold a line that begins "It refines", 21 of them Component goals, counted by
  `grep -c '^It refines'`. One more states its refinement inside a paragraph,
  `goal@model@a-tool-reads-the-documents-with-no-check`, found by `grep -n 'refines'`; the
  discussion did not sort it.
- The standard has eight rules. Rule 1 lets a Component's goal name its own Component. Rule 2 asks
  one reading. Rule 6 asks a measure built in the tree. Rule 7 states how a goal refines another.
- `design@agent-skills@goals-required` asks every Component for at least one goal, and the
  component-creation skill has the owner rule a new Component's purpose and goals.
- The register `goal` is one of the registers every Component carries, per
  `design@model@registers-are-declared`; `klarch init` and `klarch new-component` write a goals
  home for each Component, per `design@create@component-command`.
- The root's design home holds a head that argues the existence of 9 of the 12 Components under
  it, such as `design@knowledge-architect@git-crate`. It holds none for `check`, `gates` and
  `xtask`; the head of `gates` is in its own home, `design@gates@gates-crate`, and
  `issue@gates@the-gates-crate-head-may-belong-in-the-root-design-home` asks whether it moves.

**What the work changes.**

- The standard, the goal-setting skill, the goal reviewer, the primer's text on goals, the
  component-creation skill, the setup skill, the head-ground fragment and the decision-recording
  skill; and each other installed text that a row of the table below makes false, such as the
  texts of the planning, design and audit skills that name the goals home of every Component.
- This repository's 54 goals: each is judged again, kept, merged, turned into a design head or
  removed, and the goals kept move to the root's goals home, a directory of files by theme.
- The checker, `model`, `query` and `create`: the register `goal` is the root's alone.

**Outside the work.**

- The wording of each goal: it is ruled under `skill@knowledge-architect-goal-setting`, in slice
  1's whole-set review, not by this document.
- A wider goal on modular units in place of
  `goal@knowledge-architect@projects-are-split-into-components`: the owner raised it, and it is a
  goal to draft in that review.
- A review axis or an audit axis that judges whether goals are pursued and met: parked, in
  `issue@agent-skills@audit-axes-beyond-the-design-record` and
  `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`.
- Any grouping of this repository's crates under an intermediate Component: the owner named one as
  an example and ruled it out of scope.
- Whether a default awaiting the owner enters a head before the ruling:
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`
  keeps it.

## What is already decided

**Recorded decisions the design rests on and does not argue again.**

- `design@agent-skills@goals-bind`: the goals are the only statements assumed to come from the
  owner, and a decision binds as a presumption.
- `design@agent-skills@goals-change-through-goal-setting`, its half that no goal changes outside
  the goal-setting skill.
- `design@agent-skills@whole-set-rounds`: the rounds of the whole-set review.
- `design@agent-skills@unargued-approval-is-argued` and
  `design@agent-skills@new-or-reshaped-head-needs-design`: no head is written without its argument.
- `design@agent-skills@dependency-decision-home`: which Components exist is the root's design
  home's.
- `design@agent-skills@goal-drafted-to-move-the-tree`: a new goal is expected to be unmet.
- `design@agent-skills@behaviour-additions-need-real-use`: an installed guideline needs evidence
  from real use.
- `design@agent-skills@synthetic-evidence-not-built`: no run is built to watch a session apply the
  new standard.
- `primer@branch-commits`: every commit of a branch passes the checker at its tip.

- `design@agent-skills@verdicts-reach-the-conformance-review`: the design-conformance axis runs at
  a merge only on a commit message that carries a search's line with a verdict other than
  `constrains` and `unrelated`.
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: no installed text tells an agent
  what to do when a goal it applies reads two ways. The rule that a question of when, how much or
  by what means is left to design is a rule for the drafter and the reviewer of a goal, and is
  written as no instruction to an agent that applies one.
- `rejected@agent-skills@owner-intent-entry-test` and
  `rejected@agent-skills@goal-measured-by-evidence-from-use`: both stay refused. The harvest of
  slice 1 narrows the reason of each to what still holds, as the table says.

The two searches graded six of these as conflicting with a first form of the design, or as coming
back; the design was corrected to respect each, and none is reversed or rewritten.

**Recorded entries the work reverses or rewrites.** The two searches graded each as conflicting.
Each row names what changes and the slice that carries it. Where each entry is referenced follows
the table, by Component and by kind of text; `cargo klarch show <entry>` lists the files, and the
design audit of the slice named runs it, since the list changes with every commit.

| entry | what changes | slice |
| --- | --- | --- |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | rewritten: an owner's intent is a goal only as a purpose, a scope or a ranking; any other enters design as a fact or a weighing of cost | 1 |
| `design@agent-skills@audit-outcomes` | rewritten: its outcome "an intent of the owner cited as a ground, drafted as a goal" narrows to the three kinds; its quotation of the complete-workflow goal is corrected | 1 |
| `design@agent-skills@capabilities-not-structure` | rewritten: its second paragraph takes a goal's weight from a refining goal the review judges again | 1 |
| `design@agent-skills@code-shape-axis` | rewritten: its ground is a line of a measure, which leaves; the head keeps the goal as its ground, and states itself the constraint that line carried, argued in the analysis | 1 |
| `design@agent-skills@component-skill-delivery` | rewritten: its fifth moment, a purpose stated for several Components, leaves; component creation no longer sends to goal-setting | 1 |
| `design@agent-skills@expectation-set-bounds-scope` | rewritten: its boundary sentence argues from the measure of a refining goal | 1 |
| `design@agent-skills@goal-carries-reading-examples` | reversed: a goal carries no fixed block of cases | 1 |
| `design@agent-skills@goal-draft-shows-reach` | rewritten: the reach by layer is shown to the owner, and is not kept in the goal | 1 |
| `design@agent-skills@goal-entry-shape` | rewritten: a goal entry holds a statement, with no measure and no fixed block of cases; its argument against a bound on length stands | 1 |
| `design@agent-skills@goal-exclusion-names-its-design` | rewritten: rule 8 becomes the condition of the scope kind; "what a Component is not for" leaves | 1 |
| `design@agent-skills@goal-finding-grades` | rewritten: the rows that grade a finding on a measure leave, in the head and in the goal reviewer's table; a question of when, how much or by what means is no finding | 1 |
| `design@agent-skills@goal-holds-one-intent` | rewritten: a measure is no longer part of a goal. A scope is a goal of its own only where it means something without any one purpose; otherwise it stays a qualifier of the purpose it bounds, as the head says of a domain | 1 |
| `design@agent-skills@goal-met-in-tree-or-in-use` | reversed: a goal carries no measure; how a goal is pursued and shown met is design | 1 |
| `design@agent-skills@goal-one-meaning-any-breadth` | rewritten: a question of when, how much or by what means is no finding; the clause on the set a universal ranges over leaves, D2 | 1 |
| `design@agent-skills@goal-outcome-survives-empty-skeleton` | rewritten: the sentence that a Component's goal may name its own Component leaves; a kind of interface is a goal only as a purpose | 1 |
| `design@agent-skills@goals-change-through-goal-setting` | rewritten: "at setup for every Component" leaves | 1 |
| `design@agent-skills@goals-required` | reversed: a Component states no goal | 1 |
| `design@agent-skills@goal-standard-rules` | rewritten with the new rules | 1 |
| `design@agent-skills@heads-stand-on-argument` | rewritten: its table's row on an intent that states an outcome narrows to the three kinds, with the head-ground fragment | 1 |
| `design@agent-skills@installed-skill-takes-precedence` | rewritten: its exception cites the root's complete-workflow goal directly; the owner agreed to that default of the session | 1 |
| `design@agent-skills@instruction-record-is-minimal` | rewritten: its recording test names a refining goal the review judges again | 1 |
| `design@agent-skills@intermediate-component-occasion` | rewritten: the occasion leaves, and the installed text states none. Of its six rivals, the three that name an occasion (a shared principle, a split, a count) are refused no longer; the two that name a form (a declared set, a crate whose Components are modules) stay refused. Its slug is renamed to what it then states | 1 |
| `design@agent-skills@plan-read-against-the-record` | rewritten: the reviewer reads the root's goals home; its quotation of a measure leaves | 1 |
| `design@agent-skills@questions-go-to-their-authority` | rewritten: "an intent is a goal to set" narrows to the three kinds, with `primer@question-authority` | 1 |
| `design@agent-skills@searcher-grades-goals` | rewritten: the session reads the root's goals home alone | 1 |
| `design@agent-skills@shipped-text-cites-no-entry` | rewritten: the form that names a goals home in the shipped text names the root's | 2 |
| `design@agent-skills@standing-argument-in-head` | rewritten: a head states, beside its reference to a goal, the constraint it derives from it, with the head-ground fragment | 1 |
| `design@check@every-path-names-its-anchor` | rewritten: its exception for a goals home leaves; the head is split, per `issue@check@core-heads-bundle-decisions-with-many-citing-sites` | 2 |
| `design@check@goals-home-cites-upward-only` | rewritten: one goals home remains, and no goal refines another | 2 |
| `design@check@nothing-of-a-project-is-compiled-in` | rewritten: the count of registers every Component carries, and the starting text of a goals home | 2 |
| `design@create@component-command` | rewritten: a new Component is written no goals home | 2 |
| `design@knowledge-architect@no-intermediate-component` | removed: its only argument is the occasion that leaves | 1 |
| `design@model@components-carry-the-same-documents` | rewritten: `goal` leaves the registers every Component carries | 2 |
| `design@model@registers-are-declared` | rewritten: `goal` is the root's register | 2 |
| `design@mutate@one-file-per-run` | rewritten: its sentence on a clause of a measure; the owner's choice it reports is carried into the head | 1 |
| `design@query@goal-context-once` | rewritten: the clauses on the goals of enclosing Components and of search units leave, with the `worklist` code they describe | 2 |
| `design@transcript@transcript-labels-in-code` | rewritten: it holds the constraint itself, where it cited a case of a goal | 1 |
| `rejected@agent-skills@goal-measured-by-evidence-from-use` | kept, its reason narrowed: rule 6, which it cites, leaves; what still holds is the owner's argument that a measure on use has limited feasibility. The head that replaces `design@agent-skills@goal-met-in-tree-or-in-use` cites it. The discussion the parked thread opens owes a new argument against it | 1 |
| `rejected@agent-skills@owner-intent-entry-test` | kept, its reason narrowed: an intent that states a purpose, a scope or a ranking is a goal, and any other is an argument or a choice at a fork. The harvest writes no head that stands on the owner's intent alone | 1 |
| `design@agent-skills@intent-through-goals-not-overrule` | read again in the whole-set review, with `goal@knowledge-architect@the-owner-decides`: "through a goal" is read as a goal of the three kinds, which the owner keeps as a way to change a decision | 1 |
| `design@agent-skills@goal-reviewer-agent` | rewritten: "every goals home" and its premise on root goals that refine no other | 1 |

Where each entry is referenced, outside this milestone's documents, at the commit that wrote this list:

- `design@agent-skills@a-head-is-owed-by-an-entry-test`: agent-skills: design heads (4), installed text source (1), issues (2), tripwires (1).
- `design@agent-skills@audit-outcomes`: agent-skills: design heads (5), issues (3), rejected designs (1); the root: design heads (1).
- `design@agent-skills@capabilities-not-structure`: agent-skills: design heads (6), issues (1), rejected designs (2).
- `design@agent-skills@code-shape-axis`: agent-skills: design heads (1), installed text source (1).
- `design@agent-skills@component-skill-delivery`: agent-skills: design heads (1), installed text source (2).
- `design@agent-skills@expectation-set-bounds-scope`: agent-skills: design heads (1), installed text source (1), issues (1), other documents (1), tripwires (2).
- `design@agent-skills@goal-carries-reading-examples`: agent-skills: design heads (2).
- `design@agent-skills@goal-draft-shows-reach`: agent-skills: design heads (1), installed text source (1).
- `design@agent-skills@goal-entry-shape`: agent-skills: installed text source (1).
- `design@agent-skills@goal-exclusion-names-its-design`: agent-skills: design heads (1).
- `design@agent-skills@goal-finding-grades`: agent-skills: design heads (2), installed text source (1), other documents (1).
- `design@agent-skills@goal-holds-one-intent`: agent-skills: design heads (1).
- `design@agent-skills@goal-met-in-tree-or-in-use`: agent-skills: design heads (2).
- `design@agent-skills@goal-one-meaning-any-breadth`: agent-skills: design heads (1).
- `design@agent-skills@goal-outcome-survives-empty-skeleton`: agent-skills: design heads (2).
- `design@agent-skills@goals-change-through-goal-setting`: check: design heads (1).
- `design@agent-skills@goals-required`: nowhere.
- `design@agent-skills@goal-standard-rules`: agent-skills: design heads (2), installed text source (2), issues (1), other documents (1).
- `design@agent-skills@heads-stand-on-argument`: agent-skills: design heads (9), installed text source (1), tripwires (3).
- `design@agent-skills@installed-skill-takes-precedence`: agent-skills: design heads (2), installed text source (1).
- `design@agent-skills@instruction-record-is-minimal`: agent-skills: design heads (2), other documents (1).
- `design@agent-skills@intermediate-component-occasion`: agent-skills: design heads (2), installed text source (3); check: design heads (1); the root: design heads (1).
- `design@agent-skills@plan-read-against-the-record`: agent-skills: design heads (2), installed text source (1), issues (1), rejected designs (1).
- `design@agent-skills@questions-go-to-their-authority`: agent-skills: design heads (14), issues (2), rejected designs (1), tripwires (1).
- `design@agent-skills@searcher-grades-goals`: agent-skills: installed text source (1), issues (1); query: design heads (1).
- `design@agent-skills@shipped-text-cites-no-entry`: agent-config: installed copies (1); agent-skills: design heads (1), issues (3), other documents (3).
- `design@agent-skills@standing-argument-in-head`: agent-skills: design heads (1), issues (2).
- `design@check@every-path-names-its-anchor`: agent-skills: installed text source (2); check: design heads (2), issues (2), other documents (1), rejected designs (3), source (2); model: design heads (6), issues (2), source (2); the root: another plan document (8).
- `design@check@goals-home-cites-upward-only`: agent-skills: design heads (2), installed text source (2); check: design heads (2), issues (3), other documents (1), source (3); model: design heads (1).
- `design@check@nothing-of-a-project-is-compiled-in`: agent-skills: issues (1); check: design heads (1), other documents (2); create: design heads (1), issues (1), source (2); model: design heads (3).
- `design@create@component-command`: cli: source (1); create: source (1).
- `design@knowledge-architect@no-intermediate-component`: nowhere.
- `design@model@components-carry-the-same-documents`: agent-skills: design heads (2), installed text source (1); check: design heads (1); model: design heads (1), issues (1), source (1); the root: design heads (1).
- `design@model@registers-are-declared`: check: design heads (1), other documents (1), rejected designs (1); model: design heads (2), source (2).
- `design@mutate@one-file-per-run`: mutate: issues (2).
- `design@query@goal-context-once`: agent-skills: design heads (3); query: design heads (3).
- `design@transcript@transcript-labels-in-code`: agent-skills: design heads (1); transcript: issues (1), other documents (1), rejected designs (1), source (7).
- `rejected@agent-skills@goal-measured-by-evidence-from-use`: agent-skills: design heads (1).
- `rejected@agent-skills@owner-intent-entry-test`: agent-skills: design heads (3).
- `design@agent-skills@intent-through-goals-not-overrule`: agent-skills: design heads (3).
- `design@agent-skills@goal-reviewer-agent`: agent-skills: design heads (1), installed text source (1).

**The verdicts of the two searches.** The first search ran on nine subjects and gave 318 entries a
verdict other than `unrelated`; the second ran on four subjects opened after it and gave 124. Their
generated lines follow, whole. The outcome of each is by its verdict:

| verdict, action | outcome |
| --- | --- |
| `constrains`, `respect` or `none` | the decided design respects the entry; where the entry cites a Component goal or a measure, slice 1's analysis of that goal re-points or rewrites it |
| `conflicts`, `check-reversal` | the entry is in the table above, with what changes and its slice, or among the decisions the design rests on, where the design was corrected to respect it |
| `conflicts`, `goal-setting` | each of the 54 goals; slice 1's whole-set review judges it |
| `brings-back`, `new-argument-needed` | in the table above: both rejected designs stay refused, their reasons narrowed |
| `closes`, `close-with-the-work` | closed by a commit of this document's own branch, as named below |
| `trigger-met`, `rule-before-work` | ruled below; the third such verdict is on the first issue closed below, whose trigger was this discussion |
| `worsens` or `touches` | stays open; named below where the work owes it an edit |

Issues that the branch of this document closes, their work being scheduled here. Each is named by
its bare slug, its entry being deleted on that branch:

- `the-goal-standard-does-not-say-what-a-goal-is-worth-or-what-it-is-for`: its
  re-entry point was this discussion. The owner's words it held are quoted below this list.
- `goal-setting-has-no-step-to-discover-an-intent-the-owner-does-not-hold-yet`:
  no step is added, `thread@goals-at-the-root@goal-discussion-before-the-draft`.
- `a-component-states-at-least-one-goal`: the check is refused, a Component stating no
  goal.
- `root-goals-about-the-checker-may-belong-to-an-intermediate-component`:
  goals live at the root, and the installed text states no occasion for an intermediate Component.
- `the-goal-s-ruled-out-case-reads-two-ways-on-a-harness-tag`: a case of a goal
  read two ways, the owner's reading held in `design@transcript@transcript-labels-in-code` alone.
  Its closing asked the case reworded in the goal. The design answers it the other way: the cases
  leave the goal, and slice 1 rewrites that head to hold the reading as its own statement,
  attributed to the owner as it is today. The reading: the goal rules out a test on what a line
  means in the discussion, not a label read from a tag the harness writes.
- `goal-design-heads-may-repeat-the-fragment-s-reasons`: slice 1 rewrites the
  section of 24 heads it describes, and its step 1 holds the pass the entry asked: head by head,
  under `skill@knowledge-architect-decision-recording`, a head that argues only a rule the fragment
  alone states is kept on a reason of its own or folded into the fragment's `%%` comment and
  `design@agent-skills@goal-standard-rules`.

The first of them held words of the owner from before the discussion, quoted on the owner's
request, which that entry said leave with it. They are kept here, as the discussion's first
statement. Given at the goal of `cli`, on a proposal to extend a test so that it reached every
command for each kind of refused invocation:

> tbh, the goal itself is probably enough. I don't think it is worth it to try everything to have a mechanical test that detects any breach of that goal. Keep the effort on that reasonable. Goals are supposed to cover aspects that shape the future of the project, and that argumentation alone would not necessarily be able to defend (for example the cost/effort being one day considered to high might end up outweighing the justification or value).
> Most of the time, a goal being met won't be testable by a mechanical test, and that is probably when goals have the most value.
> Here, I think the goals stand because even with a mechanical test, someone might judge the effort to have a clean cli api too high and just disable the test.

At the same goals, on whether an outcome should be a goal or a design head:

> Goals should not be added just for the sake of adding goals.

At the intake of forty retrospective files, ruling to keep the intent row of the table of what the
owner brings, by which an outcome a recorded decision already serves against every rival it is
cited to defeat is drafted as no goal:

> Since goals bind very strongly, I want to avoid having too much of them, especially when they are things that can rest on argument. Because that means a reversal would be a better shape. If a goal state it, it would never be questioned, thus never improve. I think goals should be overall focused more on outcomes of the project and not on its shape. And I think the current standard is creating goals that are a bit too focused on shapes than on outcomes and functions of the product the project delivers.

The closed entry on a discovery step recorded that the owner first proposed such a step, its shape
open, at an earlier slice. The closed entry on the missing check recorded that 8 of the 12
Components of thaum, the checker's first user, stated no goal, counted on thaum's main at e98e296.

Issues that stay open and that a slice edits:

- `issue@agent-skills@audit-axes-beyond-the-design-record` and
  `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`: slice 1
  writes the parked thread's leads into them, and removes from the first its bullet that treats a
  goal no head serves as a finding. What it writes respects the heads the second search set
  against those leads, each cited there: `design@agent-skills@audit-method`,
  `design@agent-skills@audit-axis-runs-a-workflow`, `design@agent-skills@audit-outcomes`,
  `design@agent-skills@audit-triggers`, `design@agent-skills@audit-agent-per-axis`,
  `design@agent-skills@searcher-grades-goals`, `design@agent-skills@premortem-as-watch-points`,
  `design@agent-skills@goal-lifecycle`, `design@agent-skills@goal-drafted-to-move-the-tree`,
  `design@agent-skills@goal-set-review-in-goal-setting`,
  `design@agent-skills@goals-axis-in-the-review-before-a-merge` and
  `design@agent-skills@goal-reviewer-agent`. Their re-entry is written as a design discussion, on
  the owner's word, since no review reads a re-entry point written as an event.
- `issue@agent-skills@a-target-project-collects-no-feedback-from-use`: worsened, since rule 6 was
  the one text that asked what a project builds to learn from use. Slice 1 rewrites its summary and
  cites it from the two issues above.
- `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`:
  slice 1 rewords its summary, on the owner's ruling that this work replaces the wish it records
  for more intent moved into goals.
- `issue@knowledge-architect@the-mechanical-tasks-goal-omits-what-a-helper-does-better`: the
  whole-set review judges that goal with the widening the issue holds, and closes or re-points it.
  It stays open because this document schedules the occasion and not the outcome: the wording of
  a goal is ruled in that review.
- `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`: slice 1 adds to it
  that each standing head citing a goal states its constraint beside the reference.
- `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`: slice 1
  rewrites its lead that the goal of the planned Component is written when it is created.
- `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`: read again
  in slice 1, the setup skill's two template goals for the maintenance crate leaving.
- `issue@check@no-checked-form-cites-a-point-inside-a-section`: slice 1 searches every citation of
  a rule of the standard by its number and rewrites it, the numbers changing.
- `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`: read at slice
  1's one pass, which empties twelve goals homes, and at slice 2, which deletes them.
- `issue@gates@the-gates-crate-head-may-belong-in-the-root-design-home`: answered by slice 1, which
  moves that head to the root's design home.
- `issue@model@the-resolver-s-arms-have-mutation-survivors` and
  `issue@check@some-findings-in-a-goals-home-name-a-form-refused-there`: slice 2 edits the code
  each names, and writes its own test of the rule on what a goals home cites.
- `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`:
  slice 1 removes its case about a Component's goals.
- `issue@query@show-lists-no-subdocument-of-a-directory-home`: worsened, the root's goals home
  becoming a directory. It stays open; a grounding reads the theme files by their paths.
- `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: worsened, the
  comparison of readings mapping each line of `cargo klarch show` to its head by hand. It stays open.
- `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`: worsened by the
  harvest's rows sent to an existing head. Slice 1 answers it by having the reviewers of the
  whole-set review read every row of the analysis, not only the rows sent nowhere.

Triggers met, ruled before the work:

- `issue@check@core-heads-bundle-decisions-with-many-citing-sites`: slice 2 edits
  `design@check@every-path-names-its-anchor` beyond a re-pointing, so that head is split in slice 2.
- `issue@gates@the-gate-list-head-bundles-the-library-s-independence`: slice 1's analysis of
  `goal@gates@gates-from-a-list` edits the gate-list head. The analysis puts the question the issue
  holds to the owner: the reason the gates library depends on nothing of the checker.

**The generated lines of the first search.**

- `design@query@query-depends-on-no-check`: constrains, respect.
- `design@query@a-function-takes-a-configured-manifest`: constrains, respect.
- `design@query@packed-by-search-unit`: constrains, respect.
- `design@query@cluster-rooted-at-a-subject`: constrains, respect.
- `design@query@goal-context-once`: conflicts, check-reversal.
- `design@query@occasion-shaped-output`: constrains, respect.
- `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, respect.
- `issue@query@show-lists-no-subdocument-of-a-directory-home`: worsens, respect.
- `design@git@git-is-asked-through-one-module`: constrains, respect.
- `design@git@repository-variables-dropped`: constrains, respect.
- `design@git@index-file-variable-kept`: constrains, respect.
- `design@git@failed-git-is-an-error`: constrains, respect.
- `issue@git@paths-are-decoded-outside-the-listings`: touches, respect.
- `issue@git@rev-parse-answers-none-for-every-git-failure`: touches, respect.
- `issue@git@tools-run-git-past-the-wrapper`: touches, respect.
- `issue@git@arguments-reach-git-unchecked`: touches, respect.
- `design@mutate@mutation-command`: constrains, respect.
- `design@mutate@one-file-per-run`: conflicts, check-reversal.
- `issue@mutate@a-mutation-of-several-files`: touches, respect.
- `design@mutate@build-probe-separates-unbuildable`: constrains, respect.
- `design@mutate@baseline-runs-by-default`: constrains, respect.
- `design@mutate@restore-by-comparison`: constrains, respect.
- `design@mutate@record-survives-the-run`: constrains, respect.
- `design@mutate@record-lives-in-the-git-directory`: constrains, respect.
- `issue@mutate@a-baseline-runs-on-the-build-the-target-holds`: touches, respect.
- `design@cli@the-commands-are-a-library-module`: constrains, respect.
- `design@cli@command-parsing-home`: constrains, respect.
- `design@cli@arguments-parse-through-clap`: constrains, respect.
- `design@cli@exit-code-ladder`: constrains, respect.
- `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation`: touches, none.
- `issue@cli@some-value-checks-are-made-after-parsing`: touches, respect.
- `design@create@init-command`: constrains, respect.
- `design@create@component-command`: conflicts, check-reversal.
- `rejected@create@init-refused-outside-a-repository-root`: constrains, respect.
- `issue@create@init-outside-a-worktree-deletes-an-owned-file-silently`: touches, respect.
- `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`: touches, respect.
- `design@transcript@transcript-command-reads-any-project`: constrains, respect.
- `design@transcript@transcript-labels-in-code`: constrains, respect.
- `the-goal-s-ruled-out-case-reads-two-ways-on-a-harness-tag`: closes, close-with-the-work.
- `design@gates@gates-crate`: constrains, respect.
- `design@gates@a-project-holds-its-gate-list`: constrains, respect.
- `design@gates@the-library-owns-the-flags`: constrains, respect.
- `issue@gates@the-gate-list-head-bundles-the-library-s-independence`: trigger-met, rule-before-work.
- `issue@gates@the-gates-crate-head-may-belong-in-the-root-design-home`: touches, respect.
- `issue@gates@the-library-has-no-merge-verdict`: touches, respect.
- `design@xtask@source-view-in-the-tool`: constrains, respect.
- `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`: touches, respect.
- `design@testing@binary-tests-in-testing`: constrains, respect.
- `design@agent-skills@audit-is-an-activity`: constrains, respect.
- `design@agent-skills@audit-axis-runs-a-workflow`: constrains, respect.
- `design@agent-skills@audit-outcomes`: conflicts, check-reversal.
- `design@agent-skills@audit-triggers`: constrains, respect.
- `design@agent-skills@audit-repairs-the-whole-record`: constrains, respect.
- `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`: touches, none.
- `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`: touches, none.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: touches, none.
- `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
- `design@agent-skills@a-past-sentence-is-rewritten`: constrains, respect.
- `design@agent-skills@plain-text-is-no-repair`: constrains, respect.
- `design@agent-skills@knowledge-table-home`: constrains, respect.
- `design@agent-skills@primer-limit`: constrains, respect.
- `design@agent-skills@checker-model-in-primer`: constrains, respect.
- `design@agent-skills@goals-bind`: constrains, respect.
- `design@agent-skills@branch-commits-judged-at-tip`: constrains, respect.
- `design@agent-skills@questions-go-to-their-authority`: conflicts, check-reversal.
- `design@agent-skills@one-home-for-head-rules`: constrains, respect.
- `design@agent-skills@restatement-size-test`: constrains, respect.
- `design@agent-skills@installed-skill-takes-precedence`: conflicts, check-reversal.
- `design@agent-skills@goals-required`: conflicts, check-reversal.
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: conflicts, check-reversal.
- `design@agent-skills@settled-means-recorded`: constrains, respect.
- `design@agent-skills@new-or-reshaped-head-needs-design`: conflicts, check-reversal.
- `design@agent-skills@title-states-the-rule`: constrains, respect.
- `design@agent-skills@one-decision-per-head`: constrains, respect.
- `design@agent-skills@extension-follows-the-ground`: constrains, respect.
- `design@agent-skills@existing-heads-on-touch`: constrains, respect.
- `design@agent-skills@design-home-write-loads-recording`: constrains, respect.
- `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`: worsens, respect.
- `issue@agent-skills@the-retrospective-counts-no-review-cost`: touches, respect.
- `design@agent-skills@shared-text-fragment`: constrains, respect.
- `design@agent-skills@shipped-text-line-comments`: constrains, respect.
- `design@agent-skills@shipped-text-cites-no-entry`: conflicts, check-reversal.
- `design@agent-skills@no-external-handoff`: constrains, respect.
- `issue@agent-skills@a-comment-line-may-copy-the-argument-of-the-head-it-cites`: touches, respect.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: touches, none.
- `design@agent-skills@default-code-shape`: constrains, respect.
- `design@agent-skills@one-record-searcher`: constrains, respect.
- `design@agent-skills@searcher-grades-goals`: conflicts, check-reversal.
- `design@agent-skills@verdict-report`: constrains, respect.
- `design@agent-skills@a-seed-and-its-citers-are-never-unrelated`: constrains, respect.
- `design@agent-skills@verdict-states-a-conclusion`: constrains, respect.
- `design@agent-skills@every-verdict-quotes-its-entry`: constrains, respect.
- `design@agent-skills@session-acts-on-the-verdicts`: constrains, respect.
- `design@agent-skills@seeds-name-decisions`: constrains, respect.
- `design@agent-skills@design-audit-checks-fit`: constrains, respect.
- `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`: touches, respect.
- `design@agent-skills@workflow-axis-lenses`: constrains, respect.
- `design@agent-skills@workflow-axis-finding-standard`: constrains, respect.
- `design@agent-skills@audit-net-growth`: constrains, respect.
- `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
- `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
- `issue@agent-skills@the-agentic-workflow-audit-s-repair-agents-read-the-admission-standard-by-a-pointer`: touches, respect.
- `design@agent-skills@component-skill-delivery`: conflicts, check-reversal.
- `design@agent-skills@component-crate-mapping`: constrains, respect.
- `design@agent-skills@split-cost-by-publication`: constrains, respect.
- `design@agent-skills@component-split-signals`: constrains, respect.
- `design@agent-skills@intermediate-component-occasion`: conflicts, check-reversal.
- `design@agent-skills@enclosing-design-read-at-grounding`: constrains, respect.
- `design@agent-skills@dependency-decision-home`: conflicts, check-reversal.
- `design@agent-skills@document-vocabulary`: constrains, respect.
- `design@agent-skills@milestone-split-by-lifetime`: constrains, respect.
- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: constrains, respect.
- `design@agent-skills@undesigned-work-is-an-issue`: constrains, respect.
- `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`: touches, respect.
- `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`: touches, respect.
- `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`: touches, respect.
- `design@agent-skills@instruction-record-is-minimal`: conflicts, check-reversal.
- `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
- `design@agent-skills@synthetic-evidence-not-built`: constrains, respect.
- `design@agent-skills@capabilities-not-structure`: conflicts, check-reversal.
- `design@agent-skills@expectation-set-bounds-scope`: conflicts, check-reversal.
- `design@agent-skills@statement-names-an-act`: constrains, respect.
- `design@agent-skills@term-defined-at-first-load`: constrains, respect.
- `design@agent-skills@admission-rules-home`: constrains, respect.
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: touches, none.
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: touches, respect.
- `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`: touches, respect.
- `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`: touches, respect.
- `design@agent-skills@premortem-as-watch-points`: constrains, respect.
- `design@agent-skills@intake-skill-home`: constrains, respect.
- `design@agent-skills@intake-outcome-authority`: constrains, respect.
- `design@agent-skills@goal-lifecycle`: constrains, respect.
- `design@agent-skills@goal-entry-shape`: conflicts, check-reversal.
- `design@agent-skills@goal-carries-reading-examples`: conflicts, check-reversal.
- `design@agent-skills@goals-change-through-goal-setting`: conflicts, check-reversal.
- `design@agent-skills@goal-standard-rules`: conflicts, check-reversal.
- `design@agent-skills@goal-outcome-survives-empty-skeleton`: conflicts, check-reversal.
- `design@agent-skills@goal-one-meaning-any-breadth`: conflicts, check-reversal.
- `design@agent-skills@requires-admits-rules-out`: constrains, respect.
- `design@agent-skills@goal-holds-one-intent`: conflicts, check-reversal.
- `design@agent-skills@goal-drafted-to-move-the-tree`: constrains, respect.
- `design@agent-skills@goal-met-in-tree-or-in-use`: conflicts, check-reversal.
- `design@agent-skills@goal-exclusion-names-its-design`: conflicts, check-reversal.
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: constrains, respect.
- `design@agent-skills@goal-draft-shows-reach`: conflicts, check-reversal.
- `design@agent-skills@reach-finding-asks-not-narrows`: constrains, respect.
- `design@agent-skills@goal-reviewer-agent`: constrains, respect.
- `design@agent-skills@goal-finding-grades`: conflicts, check-reversal.
- `design@agent-skills@goal-review-stops-on-old-wording`: constrains, respect.
- `design@agent-skills@goal-edit-ruled-alone`: constrains, respect.
- `design@agent-skills@goal-set-review-in-goal-setting`: constrains, respect.
- `design@agent-skills@whole-set-rounds`: constrains, respect.
- `design@agent-skills@goal-batch-ruled-in-the-tree`: constrains, respect.
- `design@agent-skills@goal-batch-drafts-committed-each-round`: constrains, respect.
- `rejected@agent-skills@goal-body-in-two-to-four-sentences`: constrains, respect.
- `rejected@agent-skills@goal-measured-by-evidence-from-use`: constrains, respect.
- `rejected@agent-skills@goal-set-review-as-an-audit-axis`: constrains, respect.
- `rejected@agent-skills@goal-universal-names-its-set`: constrains, respect.
- `rejected@agent-skills@ledger-written-during-discussion`: constrains, respect.
- `issue@agent-skills@a-target-project-collects-no-feedback-from-use`: worsens, respect.
- `goal-design-heads-may-repeat-the-fragment-s-reasons`: touches, respect.
- `goal-setting-has-no-step-to-discover-an-intent-the-owner-does-not-hold-yet`: closes, close-with-the-work.
- `the-goal-standard-does-not-say-what-a-goal-is-worth-or-what-it-is-for`: closes, close-with-the-work.
- `design@agent-skills@ruled-items-labelled`: constrains, respect.
- `design@agent-skills@design-home-is-built-intent`: constrains, respect.
- `design@agent-skills@standing-argument-in-head`: constrains, respect.
- `design@agent-skills@heads-stand-on-argument`: conflicts, check-reversal.
- `design@agent-skills@fork-choice-in-weak-form`: constrains, respect.
- `design@agent-skills@attribution-replaces-quotation`: constrains, respect.
- `design@agent-skills@unargued-approval-is-argued`: conflicts, check-reversal.
- `design@agent-skills@owner-rules-at-a-fork`: constrains, respect.
- `design@agent-skills@intent-through-goals-not-overrule`: constrains, respect.
- `rejected@agent-skills@head-cites-every-owner-ruling`: constrains, respect.
- `rejected@agent-skills@owner-intent-entry-test`: brings-back, new-argument-needed.
- `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`: touches, none.
- `design@agent-skills@plan-read-against-the-record`: conflicts, check-reversal.
- `design@agent-skills@transcript-reviewer-agent`: constrains, respect.
- `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: constrains, respect.
- `design@agent-skills@goals-axis-in-the-review-before-a-merge`: constrains, respect.
- `design@agent-skills@code-shape-axis`: conflicts, check-reversal.
- `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`: touches, respect.
- `design@check@a-commit-message-is-a-document`: constrains, respect.
- `design@check@branch-shas-are-refused`: constrains, respect.
- `issue@check@branch-sha-citations-are-judged-within-the-range-only`: touches, none.
- `issue@check@core-heads-bundle-decisions-with-many-citing-sites`: trigger-met, rule-before-work.
- `design@check@orphan-rejected-entry`: constrains, respect.
- `design@check@heading-register-two-shapes`: constrains, respect.
- `rejected@check@goal-citing-a-rejected-entry`: constrains, respect.
- `design@check@agents-table`: constrains, respect.
- `design@check@installed-files-compared-by-bytes`: constrains, respect.
- `rejected@check@claude-md-in-every-component`: constrains, respect.
- `issue@check@an-empty-harness-list-serves-a-state-that-follows-no-ask`: touches, respect.
- `issue@check@configuration-for-several-agent-providers`: touches, respect.
- `design@check@nothing-of-a-project-is-compiled-in`: conflicts, check-reversal.
- `design@check@phases-gate-the-report`: constrains, respect.
- `design@check@finding-names-the-repair`: constrains, respect.
- `design@check@the-regime-has-no-opt-out`: constrains, respect.
- `a-component-states-at-least-one-goal`: closes, close-with-the-work.
- `issue@check@no-checked-form-cites-a-point-inside-a-section`: touches, respect.
- `design@check@goals-home-cites-upward-only`: conflicts, check-reversal.
- `design@check@no-citation-direction-rule`: constrains, respect.
- `issue@check@some-findings-in-a-goals-home-name-a-form-refused-there`: touches, respect.
- `issue@check@texts-citing-the-relocation-goal-state-its-old-measure`: touches, respect.
- `design@check@an-extension-plugs-in-through-phased-hooks`: constrains, respect.
- `issue@check@an-extension-cannot-see-which-register-a-document-belongs-to`: touches, respect.
- `design@check@every-path-names-its-anchor`: conflicts, check-reversal.
- `design@check@links-are-navigation-rows`: constrains, respect.
- `design@knowledge-architect@installed-text-crate`: constrains, respect.
- `design@knowledge-architect@checker-asks-git-through-the-crate`: constrains, respect.
- `design@knowledge-architect@model-crate`: constrains, respect.
- `design@knowledge-architect@git-crate`: constrains, respect.
- `design@knowledge-architect@query-crate`: constrains, respect.
- `design@knowledge-architect@transcript-crate`: constrains, respect.
- `design@knowledge-architect@create-crate`: constrains, respect.
- `design@knowledge-architect@cli-crate`: constrains, respect.
- `design@knowledge-architect@testing-crate`: constrains, respect.
- `design@knowledge-architect@mock-projects-in-one-crate`: constrains, respect.
- `design@knowledge-architect@a-copy-of-the-writer-per-crate`: constrains, respect.
- `design@knowledge-architect@no-intermediate-component`: conflicts, check-reversal.
- `design@knowledge-architect@mutation-command-host`: constrains, respect.
- `design@knowledge-architect@the-changelog-ships-in-every-crate`: constrains, respect.
- `rejected@knowledge-architect@harness-kept-in-check`: constrains, respect.
- `issue@knowledge-architect@a-typed-command-trusts-its-caller-to-have-configured-the-manifest`: touches, none.
- `root-goals-about-the-checker-may-belong-to-an-intermediate-component`: closes, close-with-the-work.
- `issue@knowledge-architect@the-checker-may-depend-on-every-crate-below-it`: touches, none.
- `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`: touches, respect.
- `design@knowledge-architect@git-flow`: constrains, respect.
- `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`: touches, respect.
- `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`: touches, respect.
- `issue@knowledge-architect@the-mechanical-tasks-goal-omits-what-a-helper-does-better`: touches, respect.
- `issue@knowledge-architect@the-retrospective-asks-for-no-tool-a-session-set-up`: touches, respect.
- `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`: touches, respect.
- `design@knowledge-architect@versioning-policy`: constrains, respect.
- `design@knowledge-architect@changelog-entries`: constrains, respect.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect.
- `design@knowledge-architect@capability-cost-before-migration`: constrains, respect.
- `design@model@one-entity-table`: constrains, respect.
- `design@model@an-entry-is-a-heading-at-the-register-level`: constrains, respect.
- `design@model@a-slug-elsewhere-defines-nothing`: constrains, respect.
- `design@model@an-entity-belongs-to-its-anchor`: constrains, respect.
- `design@model@an-unresolved-reference-is-reported`: constrains, respect.
- `rejected@model@register-reference-without-anchor`: constrains, respect.
- `issue@model@a-slug-in-a-quotation-is-reported-as-a-misplaced-definition`: touches, respect.
- `issue@model@the-resolver-s-arms-have-mutation-survivors`: touches, respect.
- `design@model@a-reference-is-live-in-every-prose`: constrains, respect.
- `design@model@section-homes-carry-slugs`: constrains, respect.
- `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`: touches, respect.
- `design@model@reserved-anchors`: constrains, respect.
- `design@model@register-home-reference`: constrains, respect.
- `design@model@path-to-a-home-is-reported`: constrains, respect.
- `issue@model@an-ignored-path-with-no-slash-passes-by-build-state`: touches, none.
- `design@model@components-carry-the-same-documents`: conflicts, check-reversal.
- `design@model@anchors-are-components-and-locations`: constrains, respect.
- `design@model@anchor-names-unique`: constrains, respect.
- `design@model@a-component-carries-what-a-project-attaches`: constrains, respect.
- `issue@model@the-walk-may-exclude-a-file-the-model-requires`: touches, none.
- `design@model@registers-are-declared`: conflicts, check-reversal.
- `design@model@a-wrong-declaration-is-a-finding`: constrains, respect.
- `design@model@declarable-directory-register`: constrains, respect.
- `design@model@plans-dir-fixed`: constrains, respect.
- `design@model@plan-item-scope`: constrains, respect.
- `design@model@resolver-outcome`: constrains, respect.
- `design@model@no-check-below-the-model`: constrains, respect.
- `design@model@a-failed-parse-is-loud`: constrains, respect.
- `goal@knowledge-architect@agents-work-without-drift`: conflicts, goal-setting.
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: conflicts, goal-setting.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: conflicts, goal-setting.
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: conflicts, goal-setting.
- `goal@knowledge-architect@structure-and-workflow-work-together`: conflicts, goal-setting.
- `goal@knowledge-architect@records-live-beside-the-code`: conflicts, goal-setting.
- `goal@knowledge-architect@documentation-stays-consistent`: conflicts, goal-setting.
- `goal@knowledge-architect@the-owner-decides`: conflicts, goal-setting.
- `goal@knowledge-architect@the-owner-s-word-is-verified`: conflicts, goal-setting.
- `goal@knowledge-architect@design-with-agents-as-counterparts`: conflicts, goal-setting.
- `goal@knowledge-architect@owner-attention-on-what-is-theirs`: conflicts, goal-setting.
- `goal@knowledge-architect@serves-one-kind-of-project`: conflicts, goal-setting.
- `goal@knowledge-architect@every-target-project-can-adopt-it`: conflicts, goal-setting.
- `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: conflicts, goal-setting.
- `goal@knowledge-architect@the-workflow-leads-to-better-code`: conflicts, goal-setting.
- `goal@knowledge-architect@projects-are-split-into-components`: conflicts, goal-setting.
- `goal@knowledge-architect@adoption-is-easy`: conflicts, goal-setting.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: conflicts, goal-setting.
- `goal@knowledge-architect@setup-brings-quality-tools`: conflicts, goal-setting.
- `goal@knowledge-architect@records-reach-their-reader`: conflicts, goal-setting.
- `goal@knowledge-architect@documents-render-as-a-linked-site`: conflicts, goal-setting.
- `goal@agent-skills@one-skill-per-activity`: conflicts, goal-setting.
- `goal@agent-skills@no-contradiction-in-workflow`: conflicts, goal-setting.
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: conflicts, goal-setting.
- `goal@agent-skills@reviews-judge-what-no-tool-can`: conflicts, goal-setting.
- `goal@agent-skills@installed-text-works-in-every-target-project`: conflicts, goal-setting.
- `goal@agent-skills@installed-text-works-in-any-environment`: conflicts, goal-setting.
- `goal@agent-skills@only-needed-instructions`: conflicts, goal-setting.
- `goal@agent-skills@goals-state-the-owner-s-intent`: conflicts, goal-setting.
- `goal@agent-skills@session-orchestrates-the-work`: conflicts, goal-setting.
- `goal@check@documents-stay-current`: conflicts, goal-setting.
- `goal@check@document-rules-are-checked`: conflicts, goal-setting.
- `goal@check@relocation-needs-only-goal-changes`: conflicts, goal-setting.
- `goal@check@projects-add-their-own-checks`: conflicts, goal-setting.
- `goal@check@checks-a-commit-from-its-hook`: conflicts, goal-setting.
- `goal@cli@one-program-for-its-tools`: conflicts, goal-setting.
- `goal@cli@one-home-for-the-command-line`: conflicts, goal-setting.
- `goal@cli@a-refused-invocation-does-no-work`: conflicts, goal-setting.
- `goal@testing@an-extension-is-tested-over-a-project`: conflicts, goal-setting.
- `goal@testing@program-tests-over-whole-projects-have-one-home`: conflicts, goal-setting.
- `goal@testing@a-consumer-test-uses-what-a-consumer-can`: conflicts, goal-setting.
- `goal@create@a-project-adopts-by-one-command`: conflicts, goal-setting.
- `goal@create@a-project-gains-a-component-by-one-command`: conflicts, goal-setting.
- `goal@model@a-tool-reads-the-documents-with-no-check`: conflicts, goal-setting.
- `goal@model@a-tool-knows-the-read-is-right`: conflicts, goal-setting.
- `goal@model@recorded-items-can-be-referenced`: conflicts, goal-setting.
- `goal@xtask@one-command-runs-every-gate`: conflicts, goal-setting.
- `goal@xtask@repeated-tasks-are-automated`: conflicts, goal-setting.
- `goal@query@agents-query-the-project`: conflicts, goal-setting.
- `goal@query@the-record-is-consulted-as-the-workflow-requires`: conflicts, goal-setting.
- `goal@mutate@a-named-mutation-is-checked-safely`: conflicts, goal-setting.
- `goal@gates@gates-from-a-list`: conflicts, goal-setting.
- `goal@git@a-pure-wrapper-over-git`: conflicts, goal-setting.
- `goal@transcript@a-session-reads-past-sessions`: conflicts, goal-setting.

**The generated lines of the second search.**

- `design@query@a-function-takes-a-configured-manifest`: constrains, respect.
- `design@query@goal-context-once`: constrains, respect.
- `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: worsens, respect.
- `issue@create@init-outside-a-worktree-deletes-an-owned-file-silently`: touches, respect.
- `design@transcript@transcript-labels-in-code`: conflicts, check-reversal.
- `the-goal-s-ruled-out-case-reads-two-ways-on-a-harness-tag`: touches, respect.
- `issue@gates@the-gates-crate-head-may-belong-in-the-root-design-home`: constrains, none.
- `design@agent-skills@audit-is-an-activity`: constrains, respect.
- `design@agent-skills@audit-method`: constrains, respect.
- `design@agent-skills@audit-axis-runs-a-workflow`: constrains, respect.
- `design@agent-skills@audit-outcomes`: constrains, respect.
- `design@agent-skills@audit-triggers`: constrains, respect.
- `design@agent-skills@audit-agent-per-axis`: constrains, respect.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: touches, respect.
- `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
- `design@agent-skills@primer-limit`: constrains, respect.
- `design@agent-skills@goals-bind`: constrains, respect.
- `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
- `design@agent-skills@one-home-for-head-rules`: constrains, respect.
- `design@agent-skills@goals-required`: conflicts, check-reversal.
- `rejected@agent-skills@head-rules-in-decision-recording`: constrains, respect.
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: constrains, respect.
- `design@agent-skills@losing-alternatives-filter`: constrains, respect.
- `design@agent-skills@losing-shape-test`: constrains, respect.
- `design@agent-skills@rejected-citation-direction`: constrains, respect.
- `design@agent-skills@thread-slug-is-entry-id`: constrains, respect.
- `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`: touches, respect.
- `design@agent-skills@new-or-reshaped-head-needs-design`: constrains, respect.
- `design@agent-skills@existing-heads-on-touch`: constrains, respect.
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: touches, respect.
- `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: touches, respect.
- `issue@agent-skills@the-retrospective-counts-no-review-cost`: touches, none.
- `design@agent-skills@shared-text-fragment`: constrains, respect.
- `issue@agent-skills@a-comment-line-may-copy-the-argument-of-the-head-it-cites`: touches, none.
- `design@agent-skills@searcher-grades-goals`: constrains, respect.
- `design@agent-skills@record-searched-before-the-work`: constrains, none.
- `rejected@agent-skills@conformance-reader-at-the-checkpoint`: constrains, none.
- `design@agent-skills@workflow-axis-finding-standard`: constrains, respect.
- `design@agent-skills@component-skill-delivery`: conflicts, check-reversal.
- `design@agent-skills@component-split-signals`: constrains, none.
- `design@agent-skills@intermediate-component-occasion`: conflicts, check-reversal.
- `design@agent-skills@enclosing-design-read-at-grounding`: constrains, respect.
- `design@agent-skills@dependency-decision-home`: constrains, respect.
- `design@agent-skills@undesigned-work-is-an-issue`: constrains, respect.
- `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`: touches, none.
- `design@agent-skills@instruction-record-is-minimal`: constrains, respect.
- `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
- `design@agent-skills@synthetic-evidence-not-built`: constrains, respect.
- `design@agent-skills@capabilities-not-structure`: constrains, respect.
- `design@agent-skills@statement-names-an-act`: constrains, respect.
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: touches, respect.
- `rejected@agent-skills@one-agent-reads-every-standing-entry`: constrains, respect.
- `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`: touches, none.
- `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`: touches, respect.
- `design@agent-skills@premortem-as-watch-points`: constrains, respect.
- `design@agent-skills@goal-lifecycle`: constrains, respect.
- `design@agent-skills@goal-entry-shape`: conflicts, check-reversal.
- `design@agent-skills@goal-carries-reading-examples`: conflicts, check-reversal.
- `design@agent-skills@goals-change-through-goal-setting`: constrains, respect.
- `design@agent-skills@goal-standard-rules`: constrains, respect.
- `design@agent-skills@goal-outcome-survives-empty-skeleton`: constrains, respect.
- `design@agent-skills@goal-one-meaning-any-breadth`: constrains, respect.
- `design@agent-skills@requires-admits-rules-out`: constrains, respect.
- `design@agent-skills@goal-drafted-to-move-the-tree`: constrains, respect.
- `design@agent-skills@goal-met-in-tree-or-in-use`: conflicts, check-reversal.
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: conflicts, check-reversal.
- `design@agent-skills@goal-draft-shows-reach`: conflicts, check-reversal.
- `design@agent-skills@reach-finding-asks-not-narrows`: constrains, respect.
- `design@agent-skills@goal-reviewer-agent`: constrains, respect.
- `design@agent-skills@goal-finding-grades`: constrains, respect.
- `design@agent-skills@goal-edit-ruled-alone`: constrains, respect.
- `design@agent-skills@goal-set-review-in-goal-setting`: constrains, respect.
- `design@agent-skills@whole-set-rounds`: constrains, respect.
- `rejected@agent-skills@goal-body-in-two-to-four-sentences`: constrains, respect.
- `rejected@agent-skills@goal-measured-by-evidence-from-use`: brings-back, new-argument-needed.
- `rejected@agent-skills@goal-set-review-as-an-audit-axis`: constrains, respect.
- `rejected@agent-skills@goal-universal-names-its-set`: constrains, respect.
- `issue@agent-skills@a-target-project-collects-no-feedback-from-use`: touches, respect.
- `goal-design-heads-may-repeat-the-fragment-s-reasons`: touches, respect.
- `goal-setting-has-no-step-to-discover-an-intent-the-owner-does-not-hold-yet`: touches, respect.
- `the-goal-standard-does-not-say-what-a-goal-is-worth-or-what-it-is-for`: trigger-met, rule-before-work.
- `design@agent-skills@design-record-axis`: constrains, respect.
- `design@agent-skills@standing-argument-in-head`: conflicts, check-reversal.
- `design@agent-skills@heads-stand-on-argument`: constrains, respect.
- `design@agent-skills@unargued-approval-is-argued`: constrains, none.
- `issue@agent-skills@a-head-s-measurement-says-nothing-of-its-magnitude`: touches, respect.
- `design@agent-skills@plan-read-against-the-record`: conflicts, check-reversal.
- `design@agent-skills@verdicts-reach-the-conformance-review`: conflicts, check-reversal.
- `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: constrains, respect.
- `design@agent-skills@transcript-reviewer-agent`: constrains, respect.
- `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: constrains, respect.
- `design@agent-skills@goals-axis-in-the-review-before-a-merge`: constrains, respect.
- `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`: touches, respect.
- `design@check@orphan-rejected-entry`: constrains, respect.
- `design@check@goals-home-cites-upward-only`: conflicts, check-reversal.
- `design@knowledge-architect@no-intermediate-component`: conflicts, check-reversal.
- `root-goals-about-the-checker-may-belong-to-an-intermediate-component`: closes, close-with-the-work.
- `design@knowledge-architect@versioning-policy`: constrains, respect.
- `design@knowledge-architect@changelog-entries`: constrains, respect.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect.
- `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`: touches, none.
- `design@model@components-carry-the-same-documents`: constrains, respect.
- `design@model@a-wrong-declaration-is-a-finding`: constrains, respect.
- `design@model@a-failed-parse-is-loud`: constrains, respect.
- `goal@knowledge-architect@agents-work-without-drift`: constrains, respect.
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: constrains, respect.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: constrains, respect.
- `goal@knowledge-architect@structure-and-workflow-work-together`: constrains, respect.
- `goal@knowledge-architect@documentation-stays-consistent`: constrains, respect.
- `goal@knowledge-architect@the-owner-decides`: constrains, respect.
- `goal@knowledge-architect@design-with-agents-as-counterparts`: constrains, respect.
- `goal@knowledge-architect@owner-attention-on-what-is-theirs`: constrains, respect.
- `goal@knowledge-architect@projects-are-split-into-components`: constrains, respect.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: constrains, respect.
- `goal@knowledge-architect@records-reach-their-reader`: constrains, respect.
- `goal@agent-skills@one-skill-per-activity`: constrains, respect.
- `goal@agent-skills@no-contradiction-in-workflow`: constrains, respect.
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: constrains, respect.
- `goal@agent-skills@reviews-judge-what-no-tool-can`: constrains, respect.
- `goal@agent-skills@installed-text-works-in-every-target-project`: constrains, respect.
- `goal@agent-skills@only-needed-instructions`: constrains, respect.
- `goal@agent-skills@goals-state-the-owner-s-intent`: constrains, respect.
- `goal@check@relocation-needs-only-goal-changes`: constrains, respect.
- `goal@query@the-record-is-consulted-as-the-workflow-requires`: constrains, respect.

## Criteria

### The goals stay the only statements assumed to come from the owner `##goals-stay-the-owner-s`

Binding. Source: `primer@goals-bind`. Met: no thread changes it.

### No goal is edited, moved or removed outside goal-setting and the owner's ruling `##no-goal-changes-outside-goal-setting`

Binding. Source: `goal@knowledge-architect@the-owner-decides` and
`design@agent-skills@goals-change-through-goal-setting`. Met by
`thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`.

### The split into Components is design, and creating a Component needs no goal-setting session `##the-split-is-design`

Binding. Source: the owner, round 1, `argument@goals-at-the-root@arg-5` and
`argument@goals-at-the-root@arg-6`. Met by `thread@goals-at-the-root@goals-home-at-the-root-only`.

### No head stands on the owner's intent alone, unargued `##no-head-on-intent-alone`

Binding, as a presumption. Source: `design@agent-skills@unargued-approval-is-argued` and
`rejected@agent-skills@owner-intent-entry-test`. Met by
`thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`, as the first search
corrected it.

### A goal holds no detail decided without an argument and a rival `##no-unargued-detail-in-a-goal`

Weighed. Source: the owner, round 2, `argument@goals-at-the-root@arg-32`. Met by
`thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`,
`thread@goals-at-the-root@what-shows-a-goal-met-is-design` and
`thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`.

### Every commit of a branch passes the checker at the branch's tip `##every-commit-passes-the-tip`

Binding. Source: `primer@branch-commits`. Met by the two slices,
`thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

### Work is still held to the goals once they are thinner `##work-is-held-to-thinner-goals`

Weighed. Source: the owner, round 8, and the first search. Unmet: the thread
`thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit` is parked. The owner said the
question "should be left open for now"; the acceptance of this criterion as unmet needs the owner's
word naming it, D1.

## Threads

The checkpoint table the session showed in round 12, which the owner's word of round 13, "agreed on
the table, proceed to planning", answered. A thread it shows `presumed-settled` closed `approved` on
that word. "You" is the owner and "me" the session.

| state | thread | closure |
| --- | --- | --- |
| approved | #standard-and-rewrite-on-one-branch | your word, kept against cause 3 |
| ruled-out | #goal-discussion-before-the-draft | your word; its occasion, a new Component's goals, is gone |
| ruled-out | #component-goals-optional | fails rule 1's own test |
| withdrawn | #a-goal-is-what-argument-cannot-defend | by me; the purpose test needs no stop |
| withdrawn | #goal-interpretation-register | by you |
| superseded | #goal-cases-stay-in-the-discussion | by #a-goal-carries-an-example-only-for-its-direction |
| parked | #goal-pursuit-is-judged-by-review-and-audit | cause and re-entry in the two issues above |
| presumed-settled | #goals-home-at-the-root-only | a Component's head in the root's design home; no location carries goals |
| presumed-settled | #a-goal-is-a-purpose-the-project-exists-for | purpose, scope, ranking; a scope names the design it excludes |
| presumed-settled | #no-goal-refines-another | 7 stand alone, 14 become heads, 3 as you ruled |
| presumed-settled | #a-goal-states-a-direction-and-leaves-the-rest-to-design | when, how much and by what means are no finding |
| presumed-settled | #what-shows-a-goal-met-is-design | no measure, no check |
| presumed-settled | #a-goal-carries-an-example-only-for-its-direction | no fixed block of cases |
| presumed-settled | #a-removed-goal-is-harvested-sentence-by-sentence | a head only if argued; every goal judged for whether it stays |
| presumed-settled | #a-goal-s-reading-is-a-head-s-premise | comparison by the decision-record axis |
| presumed-settled | #no-installed-rule-on-when-an-intermediate-component-exists | the installed text is silent |
| presumed-settled | #goals-are-edited-in-one-pass-under-the-whole-set-review | yours, this round |

### Goals live in the root's goals home only, split into files by theme `##goals-home-at-the-root-only`

Proposed by the owner in round 1; named by the session. Final state: `approved`, on the owner's
word against the checkpoint table, round 13: "agreed on the table, proceed to planning". Earlier
words: round 1, "I was thinking that maybe, the current shape could be replaced by keeping only
goals only at root of the project, probably favoring splitting it into multiple files by thematic
(this is already supported AFAIK)."; round 2, "goals-home-at-the-root-only: You say there is a cost
that an agent can reverse the decision. To me, this is not the cost, but the feature here."

Moved by `argument@goals-at-the-root@arg-5`, `argument@goals-at-the-root@arg-6`,
`argument@goals-at-the-root@arg-9`, `argument@goals-at-the-root@arg-10`,
`argument@goals-at-the-root@arg-14`, `argument@goals-at-the-root@arg-15`,
`argument@goals-at-the-root@arg-28`, `argument@goals-at-the-root@arg-82`. Its costs:
`argument@goals-at-the-root@arg-16` to `argument@goals-at-the-root@arg-19`. Shape: "Decided
design", its first subsection. Harvest: slice 1 for the installed text and for the heads of
the root's design home that argue each Component's existence, slice 2 for the checker.

### A Component may keep goals, but none that states its place in the split `##component-goals-optional`

Proposed by the session in round 1, as the nearest rival of the thread above. Final state:
`ruled-out`, closed by the session in round 7 on `argument@goals-at-the-root@arg-15` and
`argument@goals-at-the-root@arg-95`. No word of the owner names it.

### A goal is what the owner holds against an argument on cost `##a-goal-is-what-argument-cannot-defend`

Proposed by the session in round 1, from the owner's words in an issue. Final state: `withdrawn`
by the session in round 2, on `argument@goals-at-the-root@arg-39`. The owner, round 2:
"`thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`: I am not fully in favor of this shape. I proposed it, but I
am uncertain it is truly the right answer here."

### A goal is a purpose the project exists for, a scope that bounds it, or a ranking between two purposes `##a-goal-is-a-purpose-the-project-exists-for`

Proposed by the session in round 2, from the owner's formulation; the scope and the ranking are the
session's addition. Final state: `approved`, round 13. The owner, round 2: "More than that, I think
that a goal is what drives the project itself to exist. If I remove all goals, then nothing in the
project would even need to exist to fulfill them."; round 3: "`thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`:
i think your three kinds of goals are probably right. purpose and scope are pretty much opposites:
one increases what the project is for, the other reduces it."

Moved by `argument@goals-at-the-root@arg-27`, `argument@goals-at-the-root@arg-31`,
`argument@goals-at-the-root@arg-39`, `argument@goals-at-the-root@arg-40`,
`argument@goals-at-the-root@arg-45`, `argument@goals-at-the-root@arg-90`. Shape: "Decided design".
Harvest: slice 1.

### No goal refines another: a statement grounded on a goal is a design head citing it `##no-goal-refines-another`

Proposed by the session in round 2, from the owner's observation. Final state: `approved`, round
13. The owner, round 2: "There is one more observation of mine that I forgot to give: about goals
that refine another one. I think those tend to be the one that are closest to a design entry.";
round 3: "About my statement on refinement: what I said might not be true of all cases. I expect
goals that refine another to be a shape grounded on the refined goal. This might not always be the
case though."; round 4: "I agree on your list for the refinements." and "I agree with
no-goal-refines-another, this is the final state we expect."

Moved by `argument@goals-at-the-root@arg-26`, `argument@goals-at-the-root@arg-36`,
`argument@goals-at-the-root@arg-37`, `argument@goals-at-the-root@arg-46` to
`argument@goals-at-the-root@arg-48`, `argument@goals-at-the-root@arg-50`,
`argument@goals-at-the-root@arg-51`, `argument@goals-at-the-root@arg-57` to
`argument@goals-at-the-root@arg-59`, `argument@goals-at-the-root@arg-86`. Shape: "Decided design"
and the first mapping table. Harvest: slice 1.

### A goal states one direction, and a question of when, how much or by what means is left to design `##a-goal-states-a-direction-and-leaves-the-rest-to-design`

Proposed by the session in round 1. Final state: `approved`, round 13. The owner, round 2:
"a-goal-states-a-direction-and-leaves-the-rest-to-design: Yes, this is the shape of what I want.
Once again, what you state as a cost is actually the feature here."

Moved by `argument@goals-at-the-root@arg-2`, `argument@goals-at-the-root@arg-3`,
`argument@goals-at-the-root@arg-12`, `argument@goals-at-the-root@arg-21`,
`argument@goals-at-the-root@arg-32`, `argument@goals-at-the-root@arg-91`. Shape: "Decided design".
Harvest: slice 1.

### A goal carries no measure: how it is pursued and shown met is design `##what-shows-a-goal-met-is-design`

Proposed by the session in round 1. Final state: `approved`, round 13. The owner, round 2:
"what-shows-a-goal-met-is-design: good idea. This was probably the source of the defect."; round 8,
against a check the session had added: "About \"No moment asks for the pursuit of a goal.\": a
check is not the right solution."

Moved by `argument@goals-at-the-root@arg-22`, `argument@goals-at-the-root@arg-33`,
`argument@goals-at-the-root@arg-43`, `argument@goals-at-the-root@arg-83`,
`argument@goals-at-the-root@arg-89`, `argument@goals-at-the-root@arg-99`,
`argument@goals-at-the-root@arg-106`. Shape: "Decided design". Harvest: slice 1.

### Goal-setting gains no discussion step before the draft `##goal-discussion-before-the-draft`

The proposal, a step before the draft that shows the owner consequences, was the owner's at an
earlier slice, its shape open, as the closed issue recorded; the session brought it into this
discussion in round 1. Final state: `ruled-out`, on the owner's word, round 5: "agreed to rule out
goal-discussion-before-the-draft". Earlier, round 4: "About the goal-discussion thread: I am not
sure whether keeping it is worth it. Since goals are expected to become thinner, maybe this step is
less needed." Moved by `argument@goals-at-the-root@arg-4`, `argument@goals-at-the-root@arg-23`,
`argument@goals-at-the-root@arg-61`, `argument@goals-at-the-root@arg-64`. Harvest: slice 1 judges
it as a losing alternative, and records that step 1 of goal-setting is kept in the `%%` comment of
that step.

### The cases of a goal are shown in the drafting exchange and not recorded `##goal-cases-stay-in-the-discussion`

Proposed by the session in round 1. Final state: `superseded` by
`thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`. The owner, round 2:
"goal-cases-stay-in-the-discussion: I am not sure here either. I think these clauses do make intent
clearer to any later reader of the goal."; round 6: "I agree the admits and rules out cases were
actually design territory. Your examples show it quite clearly." Moved by
`argument@goals-at-the-root@arg-24`, `argument@goals-at-the-root@arg-34`,
`argument@goals-at-the-root@arg-44`, `argument@goals-at-the-root@arg-67`,
`argument@goals-at-the-root@arg-69`.

### A goal holds no fixed block of cases, and an example only where its direction needs one `##a-goal-carries-an-example-only-for-its-direction`

Proposed by the session in round 5, as a synthesis after three goals were rewritten as a test, two
of which stayed goals and one of which became a head.
Final state: `approved`, round 13. The owner, round 6:
"a-goal-carries-an-example-only-for-its-direction: yes, this can be kept." Moved by
`argument@goals-at-the-root@arg-69`, `argument@goals-at-the-root@arg-70`,
`argument@goals-at-the-root@arg-71`, `argument@goals-at-the-root@arg-73`,
`argument@goals-at-the-root@arg-84`. Shape: "Decided design". Harvest: slice 1.

### Each goal removed or shortened is sorted sentence by sentence, and a carried sentence is a head only if argued `##a-removed-goal-is-harvested-sentence-by-sentence`

Proposed by the session in round 3, answering the owner's caution. Reopened in round 7 by the first
search. Final state: `approved`, round 13. The owner, round 3: "Also, if we remove any goal, we must
be careful that sometimes, we might need to create new design entries to carry their words that
have been carried. The whole set reword is gonna be quite hard to do properly here."; round 4: "I
agree with `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`, this will make sure we miss nothing.
Each goal (not just refining ones) should also be analyzed for whether it is kept or not at all,
though this can wait for when we start the rewrite of goals."; round 8: "I agree with you about the
3 findings."

Moved by `argument@goals-at-the-root@arg-49`, `argument@goals-at-the-root@arg-52` to
`argument@goals-at-the-root@arg-55`, `argument@goals-at-the-root@arg-60`,
`argument@goals-at-the-root@arg-81`. Shape: "Decided design" and the second mapping table. Harvest:
slice 1.

### The standard and the rewrite of this repository's goals land on one branch `##standard-and-rewrite-on-one-branch`

Proposed by the session in round 4, from the owner's doubt. Final state: `approved`. The owner,
round 4: "I agree on the milestone shape, though I am unsure whether the reworked standard, skill
and reviewer can be merged independently of the goal rewording itself. Maybe they should happen on
the same branch."; round 5: "agreed on standard-and-rewrite-on-one-branch"; round 10: "keep one
branch, against cause 3. I have no plan on starting any work on main while this branch is open
anyway. We can spend a bit of time on this."; round 11: "If the weight of the rewrite is too much,
multiple sessions can do. This is not a problem anyway." Moved by
`argument@goals-at-the-root@arg-56`, `argument@goals-at-the-root@arg-62`,
`argument@goals-at-the-root@arg-65`, `argument@goals-at-the-root@arg-66`,
`argument@goals-at-the-root@arg-109`, `argument@goals-at-the-root@arg-111` to
`argument@goals-at-the-root@arg-114`. Shape: "Decided design", its last subsection, and
"Implementation sequence". Harvest: none, since it decides how this work is cut and nothing a later
change reads.

### An interpretation of a goal is recorded in a register of its own `##goal-interpretation-register`

Proposed by the owner in round 6, as a question: "This makes me want to open another proposal:
maybe we should have another shape of admitted design entry, to record an \"interpretation of a
goal\", which would be what we consider to be admitted or ruled out by it. Or we could keep the
current shape. Do you think this case (or something similar) could be missing right now ?" Final
state: `withdrawn` by the owner, round 7: "Agreed, I withdraw goal-interpretation-register, and
a-goal-s-reading-is-a-head-s-premise is a good idea to solve the gap you identified". Moved by
`argument@goals-at-the-root@arg-74` to `argument@goals-at-the-root@arg-77`, and
`argument@goals-at-the-root@arg-79`.

### A head states the constraint it derives from a goal, and the decision-record axis compares the readings of one goal `##a-goal-s-reading-is-a-head-s-premise`

Proposed by the session in round 6. The owner agreed in round 7, quoted above, to the form whose
comparison was the design-conformance axis's; the session moved the comparison to the
decision-record axis in round 12 and said so, on `argument@goals-at-the-root@arg-117`. Final state:
`approved`, round 13, on the word against the table that showed the moved form. Moved by
`argument@goals-at-the-root@arg-75`, `argument@goals-at-the-root@arg-78`,
`argument@goals-at-the-root@arg-117`,
`argument@goals-at-the-root@arg-120`. Shape: "Decided design". Harvest: slice 1.

### The installed text states no occasion on which an intermediate Component is created `##no-installed-rule-on-when-an-intermediate-component-exists`

Proposed by the session in round 8, naming the owner's fallback. Final state: `approved`, round 13.
The owner, round 8: "If we find none that is general enough, we can just leave intermediate
component splits to per project design, and give no generic guideline about it, which would be
better than a flawed guideline."; round 9: "Agreed on
#no-installed-rule-on-when-an-intermediate-component-exists. If the usage of that actually surfaces
in a consumer, we can learn from it and observe whether it works well or not, and why. For now,
stay silent on that. Even on the split I proposed here, I am unsure whether it would have any
benefit at all. The split is out of scope for today btw, it was just an example." Moved by
`argument@goals-at-the-root@arg-85`, `argument@goals-at-the-root@arg-96`,
`argument@goals-at-the-root@arg-97`, `argument@goals-at-the-root@arg-101` to
`argument@goals-at-the-root@arg-105`, `argument@goals-at-the-root@arg-108`,
`argument@goals-at-the-root@arg-121`, `argument@goals-at-the-root@arg-122`. Shape: "Decided
design". Harvest: slice 1.

### Whether a review axis and an audit axis judge that goals are pursued and met `##goal-pursuit-is-judged-by-review-and-audit`

Recorded by the session in round 8, from the owner's re-entry. Final state: `parked`. Its cause:
goals grow thinner, so grounding in the goals and the record search may hold work to them less well
than today. The evidence that reopens it: a retrospective's or a review's finding of merged work
that moved against a goal with no procedure reporting it. Its re-entry point: the two issues named
under "What is already decided". The owner, round 8: "This should be left open for now. This is not
a new problem anyway. I think this can never be met mechanically anyway." and "I think the reentry
to that would be a discussion about a review axis against project goals and evidence collection at
merge and/or plans, and an audit axis trying to detect whether goals are met, and whether the
project is collecting enough evidence of goals being met or not." Moved by
`argument@goals-at-the-root@arg-89`, `argument@goals-at-the-root@arg-99`,
`argument@goals-at-the-root@arg-100`, `argument@goals-at-the-root@arg-107`,
`argument@goals-at-the-root@arg-118`, `argument@goals-at-the-root@arg-119`.

### The goals are analysed per theme, and all edited in one pass under the whole-set review `##goals-are-edited-in-one-pass-under-the-whole-set-review`

Proposed by the owner in round 12; named by the session. Final state: `approved`, round 13. The
owner, round 12: "The rewords should probably not happen in batches per theme. Or we will have half
finished states, where part of the goals are ported, and the other is not. This is a problem,
because the whole goal home have to be consistent with no contradictions. The analysis of each
sentence can be done per batch. But the edits to the goals cannot. They should happen under the
whole set review procedure, edit all goals, one reviewer per theme and one global reviewer, then
another round." Moved by `argument@goals-at-the-root@arg-115`,
`argument@goals-at-the-root@arg-116`. Shape: slice 1's spec. Harvest: none.
`design@agent-skills@whole-set-rounds` states the rounds, one reviewer per set of goals that touch
each other and one over the whole set; the order, the analysis first and then one pass, is this
slice's own and nothing a later change reads.

## Arguments

One item per argument of the discussion, in order of appearance, from the session's transcript. A
title is the argument's first words, cut where marked. "Bears on" names threads of this document.
The key words are verbatim, with two changes the checker requires: a slug or a path the speaker
wrote bare is written as its reference, and a reference in a title keeps its backticks. The same
holds for the owner's words quoted under Threads. A Note is the assembly's, not the speaker's. A
table an argument announces and does not hold is under "Decided design" or "Mapping tables".

### Overall, I think that goals should be about outcomes of the project as a whole, and that this is currently (cut) `##arg-1`

Round 1, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> Overall, I think that goals should be about outcomes of the project as a whole, and that this is currently not correctly reflected.

### they are too detailed, often to the point of clashing with design territory. This is probably due to the goal (cut) `##arg-2`

Round 1, by the owner. Bears on `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`, `thread@goals-at-the-root@what-shows-a-goal-met-is-design`, `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`.

> 1. they are too detailed, often to the point of clashing with design territory. This is probably due to the goal drafting procedure, which still pushes to close any ambiguity with the review rounds, and that is often pushing to take decision that should not be stated as absolute decision, but argued so that the project still has the leeway to improve.

### The goal is that the main session orchestrates subagents to carry the work, so that it can focus on deciding (cut) `##arg-3`

Round 1, by the owner. Bears on `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.

> The goal is that the main session orchestrates subagents to carry the work, so that it can focus on deciding design with the human interlocutor, and on reporting any decision taken. Nothing more detailed. When exactly these dispatch happen, and on which kind or size of work, is left to argumented design.

**Note**: The owner's example: the orchestration goal.

### I think that the session that drafted this goal took a wrong approach, hence why I want to reopen the (cut) `##arg-4`

Round 1, by the owner. Bears on `thread@goals-at-the-root@goal-discussion-before-the-draft`.

> I think that the session that drafted this goal took a wrong approach, hence why I want to reopen the standard (along with previously recorded issues, including the lack of a discussion step about the goal)

### goals under sub components often tend to be about how the component fits inside the rest of the project `##arg-5`

Round 1, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@component-goals-optional`.

> 2. goals under sub components often tend to be about how the component fits inside the rest of the project.
>
> However, this clashes strongly with the idea that the split in components is design, what is split, why and when is supposed to be left to design.

**Note**: The owner's example: the git driver Component.

### However, right now, creating a new component (which an implementer could find needed and want to propose, (cut) `##arg-6`

Round 1, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> However, right now, creating a new component (which an implementer could find needed and want to propose, though the design before should plan for it), requires a full goal drafting session with the owner. These kind of subcomponents goals are thus setting in stone the way the project is split under components, even when we consider the split to be design.
>
> Also, this is a bit weird, because if a goal inside one component has to justify the existence of the component, which is supposed to come first ?

### This being a goal does seem weird to me. First, because its existence is mandated by the installed set. And (cut) `##arg-7`

Round 1, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`.

> This being a goal does seem weird to me. First, because its existence is mandated by the installed set. And second, because it is more like design to me, which stands on a clear argument: having one place that wraps around maintenance tasks, and having a single gate command, makes it far easier to work in the project, as well as for keeping the agent instruction set coherent.

**Note**: The owner's example: the xtask maintenance crate.

### The record today: 54 goals in 13 homes, 21 at the root and 33 in 12 Components, 22,350 words (wc -w over the (cut) `##arg-8`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> **The record today**: 54 goals in 13 homes, 21 at the root and 33 in 12 Components, 22,350 words (`wc -w` over the goals homes). I read the root's titles, and `git`, `xtask`, `query`, `gates` and the orchestration goal whole; the other Component goals by title only.

**Note**: Measurement.

**Note**: Later: the review of this document corrected 22,350 to 19,045, the session having summed a fourteenth file, the goal-setting skill.

### The standard itself licenses defect 2. Rule 1 says: "A Component's goal may name its own Component and the (cut) `##arg-9`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@component-goals-optional`.

> 1. **The standard itself licenses defect 2.** Rule 1 says: "A Component's goal may name its own Component and the Components that enclose it." The same rule says a goal "names no part whose existence comes from a design choice". A Component is such a part, so the exception contradicts the rule it sits in.

### The split is already argued as design, beside the goals that restate it. docs/design.md holds `design@knowledge-architect@git-crate`, (cut) `##arg-10`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> 2. **The split is already argued as design, beside the goals that restate it.** `register@knowledge-architect@design` holds `design@knowledge-architect@git-crate`, `model-crate`, `query-crate`, `transcript-crate`, `create-crate`, `cli-crate`, `testing-crate` and `mutation-command-host`, one head per crate. `goal@git@a-pure-wrapper-over-git` and the head `design@knowledge-architect@git-crate` state the same thing twice, once as a presumption and once as the owner's intent.

### Component creation requires the goal session you describe. Step 3 of the creation procedure: "Write its (cut) `##arg-11`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> 3. **Component creation requires the goal session you describe.** Step 3 of the creation procedure: "Write its purpose and its goals under goal-setting, the owner ruling the purpose and each goal", and `design@agent-skills@goals-required` refuses a Component without one. The first consumer does not follow it: the issue on the missing check records 8 of thaum's 12 Components with no goal (counted at an old commit, not re-taken).

**Note**: Fact, with a count the session labels as not re-taken.

### The clauses come from three rules together. Rule 2 (one reading) has the reviewer block on any wording two (cut) `##arg-12`

Round 1, by the session. Bears on `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`, `thread@goals-at-the-root@what-shows-a-goal-met-is-design`, `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`.

> 4. **The clauses come from three rules together.** Rule 2 (one reading) has the reviewer block on any wording two agents could read differently; rule 6 demands a measure built in the tree; the boundary block adds cases. The orchestration goal is the result: 6 paragraphs, a measure that names the installed text and the retrospective, and 11 cases.

### `goal-setting-has-no-step-to-discover-an-intent-the-owner-does-not-hold-yet` a discussion step before the (cut) `##arg-13`

Round 1, by the session. Bears on `thread@goals-at-the-root@goal-discussion-before-the-draft`, `thread@goals-at-the-root@goals-home-at-the-root-only`.

> | `goal-setting-has-no-step-to-discover-an-intent-the-owner-does-not-hold-yet` | a discussion step before the draft; its named cost is "a discussion per Component whose goals are drafted" | that cost disappears if Components draft no goals |
>
> | `root-goals-about-the-checker-may-belong-to-an-intermediate-component` | move root goals down if the root grows | the opposite direction to yours; thematic files answer the size concern instead |
>
> | `a-component-states-at-least-one-goal` | a check that each Component has a goal | would close as refused, with `goals-required` reversed |

**Note**: The session's reading of the open issues; the whole table is under "Decided design" or "Mapping tables".

### Why it works: it answers "which comes first". The order becomes goals, then a head that argues a split, then (cut) `##arg-14`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> - **Why it works**: it answers "which comes first". The order becomes goals, then a head that argues a split, then the Component. A redesign of the split touches no goal.

### Nearest rival: #component-goals-optional, where a Component may keep goals but none that state its place. The (cut) `##arg-15`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@component-goals-optional`.

> - **Nearest rival**: #component-goals-optional, where a Component may keep goals but none that state its place. The fact against it: a goal anchored in a Component still fails rule 1's own test, since the Component does not exist in the project emptied of its design, and moving code between Components would still move goals only you may change.

**Note**: The fact that defeats the rival. Later: round 7, the session closes the rival `ruled-out` on it.

### you no longer rule each Component's purpose. "The git library knows nothing of the workflow" becomes a (cut) `##arg-16`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

>   - you no longer rule each Component's purpose. "The git library knows nothing of the workflow" becomes a presumption an agent may reverse on argument and report under "Decided";

**Note**: Stated as a cost. Later: round 2, the owner answers that this is the feature, not the cost (`argument@goals-at-the-root@arg-28`).

### a session working in one crate no longer has a 400-word local goals home; it reads the theme files that bear (cut) `##arg-17`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

>   - a session working in one crate no longer has a 400-word local goals home; it reads the theme files that bear on it;

**Note**: Stated as a cost. Not answered by the owner.

### a breaking checker change: 235 references of the form goal@<component>@… in this repository change anchor, (cut) `##arg-18`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

>   - a breaking checker change: 235 references of the form `goal@<component>@…` in this repository change anchor, against 419 to the root (mock projects included in neither figure), and every consumer needs a Migration entry;

**Note**: Stated as a cost, with a measurement. Not answered by the owner.

### `goal@knowledge-architect@projects-are-split-into-components` says a split "keeps each Component's goals and (cut) `##arg-19`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

>   - `goal@knowledge-architect@projects-are-split-into-components` says a split "keeps each Component's goals and design small", so that goal needs rewording, which is yours.

**Note**: Stated as a cost.

### The standard states what a goal is for, from your words in the issue: an outcome of the project that you want (cut) `##arg-20`

Round 1, by the session. Bears on `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`.

> The standard states what a goal is for, from your words in the issue: an outcome of the project that you want held even when an argument on cost would defeat it. The test at drafting: if the statement can be followed by "because" and an existing goal, it is a design head citing that goal.
>
> - **Cost**: every goal has a "why" above it, so the test needs a stop. My proposed stop is your ruling, asked in the discussion step: "would you hold this against an argument that it costs too much?"

**Note**: Later: round 2, the owner doubts it and the session withdraws the thread. The "because" test returns in round 2 as the ground of `thread@goals-at-the-root@no-goal-refines-another` (`argument@goals-at-the-root@arg-37`).

### Rule 2 changes from "one reading" to "one direction". A question of when, how much or by what means is listed (cut) `##arg-21`

Round 1, by the session. Bears on `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.

> Rule 2 changes from "one reading" to "one direction". A question of when, how much or by what means is listed as left to design, and the reviewer does not block on it. Illustration only, not a draft for ruling:
>
> - **Cost**: two sessions may resolve an open point differently until a head settles it.

**Note**: Later: round 2, the owner answers that the cost is the feature (`argument@goals-at-the-root@arg-32`). Round 7, the session says the change is narrower than stated (`argument@goals-at-the-root@arg-91`).

### Rule 6 leaves the goal. No goal owes "It is met while…". How the project pursues a goal and learns whether it (cut) `##arg-22`

Round 1, by the session. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`.

> Rule 6 leaves the goal. No goal owes "It is met while…". How the project pursues a goal and learns whether it is met is a head citing it. This reverses `design@agent-skills@goal-met-in-tree-or-in-use`.
>
> - **Cost**: nothing mechanical says a goal is unmet, and the primer's row "what would show it achieved" changes.

### The session first shows what a candidate outcome would have ruled in or out among recorded decisions; you (cut) `##arg-23`

Round 1, by the session. Bears on `thread@goals-at-the-root@goal-discussion-before-the-draft`.

> The session first shows what a candidate outcome would have ruled in or out among recorded decisions; you form the intent; then it drafts. The reviewer keeps three questions: fidelity to your statements, contradiction between goals, and a clause a head could carry.

**Note**: Later: the thread is ruled out on the owner's word of round 5. What became of the reviewer's "three questions" is not said afterwards in the transcript.

### The requires/admits/rules-out cases are shown to you but not recorded in the goal. They are where clauses (cut) `##arg-24`

Round 1, by the session. Bears on `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`.

> The requires/admits/rules-out cases are shown to you but not recorded in the goal. They are where clauses accumulate, yet they are also how a later reader sees the reach.

### Root-only does not by itself give few goals. By title, about 8 of the 33 Component goals state a place in the (cut) `##arg-25`

Round 1, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`.

> Root-only does not by itself give few goals. By title, about 8 of the 33 Component goals state a place in the split; the others read as product outcomes (`create`'s "a project adopts by one command", `check`'s five). Moved up unchanged, the root would hold about 45. Only #a-goal-is-what-argument-cannot-defend, applied in a review of the whole set, thins them.

**Note**: Later: round 2, the session withdraws the figure: "My earlier figure of about 45 root goals is withdrawn; the count after the review is not known."

### If a goal refines another one, it is most likely that the goal is trying to have the refined goal enforced by (cut) `##arg-26`

Round 2, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> If a goal refines another one, it is most likely that the goal is trying to have the refined goal enforced by giving a more detailed reading of the refined goal in the context of the component. Then, it is actually a design decision of what is done in this component, whose argument stands on trying to follow the refined goal. It is probably not deserving the status of goal.

**Note**: Later: round 3, the owner hedges it (`argument@goals-at-the-root@arg-46`).

### Overall, the point I want to make clear is that I feel there is no need for the project owner to take an (cut) `##arg-27`

Round 2, by the owner. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`, `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`, `thread@goals-at-the-root@goals-home-at-the-root-only`.

> Overall, the point I want to make clear is that I feel there is no need for the project owner to take an arbitrary decision in the project.
>
> So goals are supposed to steer the project, its design and the work, toward something that provides a given service or result, with the detailed features and their API shapes left to discussion, often to maximize ergonomics, and sometimes with specific constraints on more precise outcomes. They are not supposed to be about overruling on individual decisions. In practice, the only point where it makes sense to rule on a decision is by how costs are weighted.
>
> If I judge a cost too large, i can overrule a decision without needing a goal.

**Note**: The owner's principle on where the owner's word enters.

### To me, this is not the cost, but the feature here. It improves agent autonomy, and it should not in practice (cut) `##arg-28`

Round 2, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> To me, this is not the cost, but the feature here. It improves agent autonomy, and it should not in practice result in problems, because in practice this decision can only be reversed on a better argument.
>
> And in practice, reversing a decision is in itself considered a cost by agents, so they would choose to do it only if the rival shapes are truly much better, or if following the old decision leads to a contradiction or goal violation.

**Note**: Answers `argument@goals-at-the-root@arg-16`.

### If another shape has costs, the workflow has them go through me AFAIK. If it has no costs, I have nothing to (cut) `##arg-29`

Round 2, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> If another shape has costs, the workflow has them go through me AFAIK. If it has no costs, I have nothing to say against that.

**Note**: A premise, hedged "AFAIK". Later: round 2, the session corrects it from the primer (`argument@goals-at-the-root@arg-35`).

### And there is a last wall of reviews before merge, including transcript reviewers, which make sure that (cut) `##arg-30`

Round 2, by the owner. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> And there is a last wall of reviews before merge, including transcript reviewers, which make sure that significant decision reversals are not happening behind my back anyway.

### More than that, I think that a goal is what drives the project itself to exist. If I remove all goals, then (cut) `##arg-31`

Round 2, by the owner. Bears on `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`, `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> More than that, I think that a goal is what drives the project itself to exist. If I remove all goals, then nothing in the project would even need to exist to fulfill them.
>
> There is no argument that defend the existence of these goals, because they are just the purpose of the project, and nothing else.

**Note**: The owner's two worked examples of adding a goal are in the same paragraph.

### Once again, what you state as a cost is actually the feature here. Because a design decision has to close the (cut) `##arg-32`

Round 2, by the owner. Bears on `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.

> Once again, what you state as a cost is actually the feature here. Because a design decision has to close the reading, the choice will be discussed and argued, several shapes might be tested, and a change of design can happen without a goal drafting session. This has high chances of improving the design in the long term IMO.
>
> Right now, the goals carry a lot of details that were discussed under no argumentation and no rivals proposed, which is a major defect to me.

**Note**: Answers the cost of `argument@goals-at-the-root@arg-21`.

### If a goal has to show being met in the tree, obviously it will need to describe shapes that are complete and (cut) `##arg-33`

Round 2, by the owner. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`, `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.

> If a goal has to show being met in the tree, obviously it will need to describe shapes that are complete and not abstract, so they can be detected in the tree. What should show that a goal is being met (I think I said that in an earlier session) is that work is being done to meet it, and to detect whether it is met under real use. Both are actually met by design work.
>
> The goal should not constrain that shape to an exact retrospective question. The existence of the retrospective skill and the list and content of its questions is design.

### I think these clauses do make intent clearer to any later reader of the goal. But I think the conclusion on (cut) `##arg-34`

Round 2, by the owner. Bears on `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`.

> I think these clauses do make intent clearer to any later reader of the goal. But I think the conclusion on this would depend on the shapes of goals that are written against the above rules: one direction instead of one reading, and shown-by-design, which both would significantly reduce the content of a goal draft, and leave them with more interpretation room.

**Note**: For keeping the cases, hedged. Later: round 6, the owner agrees the cases were design territory.

### You wrote: "If another shape has costs, the workflow has them go through me AFAIK." The primer says less. A (cut) `##arg-35`

Round 2, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> You wrote: "If another shape has costs, the workflow has them go through me AFAIK." The primer says less. A head standing on its argument "is reversed by a better argument, which the owner approves where the reversal is a fork". A reversal reaches you for a ruling only when two shapes survive and no goal ranks their costs.
>
> It has a consequence in your favour: fewer and more abstract goals rank fewer costs, so more decisions become forks and reach you as cost weightings. That is the entry point you say you want, and it costs you more questions than today.

**Note**: Correction of the premise of `argument@goals-at-the-root@arg-29`. The owner did not answer it.

### 25 of the 54 goals carry a line "It refines …" (grep -c '^It refines' per goals home): 22 of the 33 Component (cut) `##arg-36`

Round 2, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> 25 of the 54 goals carry a line "It refines …" (`grep -c '^It refines'` per goals home): 22 of the 33 Component goals, all 9 of `agent-skills` and all 5 of `check`, and 3 root goals that refine another root goal (`mechanical-tasks-are-the-tool-s`, `goal@knowledge-architect@the-workflow-leads-to-better-code`, `goal@knowledge-architect@projects-are-split-into-components`).

**Note**: Measurement. Later: round 3, corrected: "A correction first: I wrote that 25 goals carry "It refines". The count is 24, 21 of them in Components; I mis-added the per-home figures."

### Your observation is the "because" test I proposed, in the cases where the drafter already wrote the (cut) `##arg-37`

Round 2, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`, `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`.

> Your observation is the "because" test I proposed, in the cases where the drafter already wrote the "because". So I open **#no-goal-refines-another**: a statement whose ground is another goal is a design head citing it, and rule 7's refinement clause leaves the standard.

### The finding: the orchestration goal is one of the 25. It refines `goal@knowledge-architect@agents-get-a-complete-workflow`. By your test (cut) `##arg-38`

Round 2, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> **The finding: the orchestration goal is one of the 25.** It refines `goal@knowledge-architect@agents-get-a-complete-workflow`. By your test it is a design head, not a goal, even in the short form I illustrated. The argument for that reading: dispatching agents is a means. The outcomes are that the session's context goes to design with you, and that you are told each decision. A harness with a far larger context could reach both with less dispatch, and a goal that names orchestration would forbid that improvement.

**Note**: Later: round 3, the owner contests it (`argument@goals-at-the-root@arg-47`); the session then sorts the goal as "orthogonal, on your word; the question of means stays for its rewrite". "25" is the figure later corrected to 24.

### I withdraw #a-goal-is-what-argument-cannot-defend. Your formulation needs no ruling from you as its stop: (cut) `##arg-39`

Round 2, by the session. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`, `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`.

> I withdraw #a-goal-is-what-argument-cannot-defend. Your formulation needs no ruling from you as its stop: **remove the goal, and some part of what the project provides has no reason left to exist.** I open it as **#a-goal-is-a-purpose-the-project-exists-for**.

### It fails on one class of goals you hold today. Remove `goal@knowledge-architect@serves-one-kind-of-project` (Rust, linear history) and (cut) `##arg-40`

Round 2, by the session. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> It fails on one class of goals you hold today. Remove `goal@knowledge-architect@serves-one-kind-of-project` (Rust, linear history) and nothing loses its reason to exist; more would be built. The same holds for a ranking between two purposes, which rule 5 asks a goal to state. No argument from the purposes yields either. So I propose three kinds, and nothing else is a goal:
>
> A cost weighed inside one decision stays in design, as you say. A ranking is a goal only when it recurs across decisions.

**Note**: The table of the three kinds is under "Decided design" or "Mapping tables".

### The primer's routing changes. Today an owner's intent "that states an outcome" is drafted as a goal, and at a (cut) `##arg-41`

Round 2, by the session. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> - **The primer's routing changes.** Today an owner's intent "that states an outcome" is drafted as a goal, and at a fork the session first asks "whether a missing goal ranks the shapes". Under the three kinds, most such intents become an argument or a cost weighting recorded in a head.

**Note**: A consequence the session says the owner had not stated. The owner did not answer it by name.

### Root-only plus no refinement removes more than relocation would. If the 22 refining Component goals become (cut) `##arg-42`

Round 2, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@no-goal-refines-another`.

> - **Root-only plus no refinement removes more than relocation would.** If the 22 refining Component goals become heads, at most 11 Component goals remain to be judged as purposes, and several of those are placement goals (`git`, `xtask`, `cli`). My earlier figure of about 45 root goals is withdrawn; the count after the review is not known.

**Note**: The figure 22 is the one corrected in round 3 to 21; the session did not restate the "at most 11" that followed from it.

### The measure clauses become heads, not nothing. Each "It is met while…" that names a retrospective question or (cut) `##arg-43`

Round 2, by the session. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`, `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> - **The measure clauses become heads, not nothing.** Each "It is met while…" that names a retrospective question or a test is a decision someone argued. The review of the whole set owes a head for each one worth keeping.

**Note**: Later: round 7, a carried sentence becomes a head only if argued (`argument@goals-at-the-root@arg-81`).

### I agree it depends on the rewritten goals. The evidence is cheap: three goals rewritten under one direction (cut) `##arg-44`

Round 2, by the session. Bears on `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`.

> I agree it depends on the rewritten goals. The evidence is cheap: three goals rewritten under one direction and shown-by-design, each with and without cases. I commit in advance: if a rewritten goal of five sentences or fewer leaves two readers moving in different directions on its purpose, the cases stay; otherwise they stay in the discussion only.

**Note**: A test committed to in advance. Result: round 5 (`argument@goals-at-the-root@arg-69`).

### `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`: i think your three kinds of goals are probably right. purpose and (cut) `##arg-45`

Round 3, by the owner. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`: i think your three kinds of goals are probably right. purpose and scope are pretty much opposites: one increases what the project is for, the other reduces it. Both are clearly needed to get a reasonable scope for one full project.
>
> The ranking one is also needed when purposes compete 

### About my statement on refinement: what I said might not be true of all cases. I expect goals that refine (cut) `##arg-46`

Round 3, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> About my statement on refinement: what I said might not be true of all cases. I expect goals that refine another to be a shape grounded on the refined goal. This might not always be the case though.

**Note**: Hedge of `argument@goals-at-the-root@arg-26`.

### For the orchestration goal, i find it weird that it refines `goal@knowledge-architect@agents-get-a-complete-workflow` `##arg-47`

Round 3, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> For the orchestration goal, i find it weird that it refines `goal@knowledge-architect@agents-get-a-complete-workflow`.
>
> They are pretty much orthogonal IMO. Maybe they were no longer, because the complete-workflow one got too detailed in its text. 

**Note**: Against `argument@goals-at-the-root@arg-38`.

### We should analyze refinements one by one, to identify those whose main intent is orthogonal. This can be (cut) `##arg-48`

Round 3, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> We should analyze refinements one by one, to identify those whose main intent is orthogonal. This can be identified by focusing on titles and slugs I think. Due to being short, those kept the heart of the intent, and did not grew to emcompass larger scopes.

**Note**: A method.

### Also, if we remove any goal, we must be careful that sometimes, we might need to create new design entries to (cut) `##arg-49`

Round 3, by the owner. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> Also, if we remove any goal, we must be careful that sometimes, we might need to create new design entries to carry their words that have been carried. The whole set reword is gonna be quite hard to do properly here.

### Your guess about `goal@knowledge-architect@agents-get-a-complete-workflow` matches the list: 10 of the 24 refine it. The goal-setting (cut) `##arg-50`

Round 3, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> Your guess about `goal@knowledge-architect@agents-get-a-complete-workflow` matches the list: 10 of the 24 refine it. The goal-setting skill's description says it covers "how a Component's goal names the project goal it refines", so a drafter of a Component goal looks for a parent, and that goal is the widest one available.

**Note**: Measurement and its explanation.

### That gives 7 orthogonal, 14 heads and 3 unsure. Your caveat changes the procedure, not the end state: after (cut) `##arg-51`

Round 3, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> That gives 7 orthogonal, 14 heads and 3 unsure. Your caveat changes the procedure, not the end state: after the review no goal says "It refines". An orthogonal goal drops the line and stands alone; a grounded one becomes a head. So #no-goal-refines-another stays, with each case judged by the three kinds.

**Note**: The table of the 24 refinements is under "Decided design" or "Mapping tables". The session calls it "a first pass for you to contest, not a verdict".

### One limit of the title method: it finds the heart of the intent, but your rulings sit in the bodies. The (cut) `##arg-52`

Round 3, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`, `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> One limit of the title method: it finds the heart of the intent, but your rulings sit in the bodies. The title decides the kind; the body still has to be read sentence by sentence.

### I think this is less hard than you expect, for one reason: most sentences of a removed goal owe no new head. (cut) `##arg-53`

Round 3, by the session. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> I think this is less hard than you expect, for one reason: most sentences of a removed goal owe no new head. The existing test of `primer@design-heads` already sorts them:
>
> The orchestration measure "the retrospective asks whether the main session spent its context on work an agent could have carried" is the first row: the retrospective skill already holds the question.

**Note**: Answers `argument@goals-at-the-root@arg-49`. The table of four destinations is under "Decided design" or "Mapping tables".

### The cost to name: a head made from goal words was never argued against a rival. I propose it says so in its (cut) `##arg-54`

Round 3, by the session. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> The cost to name: a head made from goal words was never argued against a rival. I propose it says so in its ground, so a later session knows the presumption is weak.

**Note**: Later: round 7, withdrawn by finding 1 of the first record search (`argument@goals-at-the-root@arg-81`): such a head is argued or not written.

### So I open #a-removed-goal-is-harvested-sentence-by-sentence: each removed or shortened goal gets a table of (cut) `##arg-55`

Round 3, by the session. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> So I open **#a-removed-goal-is-harvested-sentence-by-sentence**: each removed or shortened goal gets a table of its sentences with one of the four rows, committed in `path@knowledge-architect@docs/goal-reviews/`, and every sentence that goes "nowhere" is shown to you before it is deleted.

### This is a milestone, not one branch. Proposed order: the standard, the skill and the reviewer first; then (cut) `##arg-56`

Round 3, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> This is a milestone, not one branch. Proposed order: the standard, the skill and the reviewer first; then this repository's goals, one theme per batch; then the checker refusing a Component's goals home, last, since every commit of a branch must pass the tip's checker.

**Note**: Later: round 4, the owner doubts the standard merging apart from the rewrite (`argument@goals-at-the-root@arg-62`), and the cut becomes two slices (`argument@goals-at-the-root@arg-65`, `argument@goals-at-the-root@arg-66`). Round 12, "one theme per batch" is withdrawn for the edits (`argument@goals-at-the-root@arg-116`).

### `goal@knowledge-architect@projects-are-split-into-components`: It could be a mean to `goal@knowledge-architect@the-workflow-leads-to-better-code` I think (and (cut) `##arg-57`

Round 4, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> `goal@knowledge-architect@projects-are-split-into-components`: It could be a mean to `goal@knowledge-architect@the-workflow-leads-to-better-code` I think (and maybe others), because a modular project often leads to a clearer API, and design docs that are easier to act on and to follow. But I lean to keeping it as a goal. It is partially orthogonal to better code, and it bears on more than just code.
>
> Maybe it could be under a more general shape though, "Projects are split into modular units", which would bind at more layers than just components: splitting code of one component, as well as splitting files of design docs by subject in one components, are places where splitting matters too and makes exploring the project and working in it easier overall. It also makes it easier for a human to review changes done by agents, which is one point that also matters to me.

**Note**: Ruling on the first unsure row, with a wider wording offered as a "maybe".

### `goal@agent-skills@installed-text-works-in-any-environment`: this one is probably a mean to complete-workflow goal. If the (cut) `##arg-58`

Round 4, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> `goal@agent-skills@installed-text-works-in-any-environment`: this one is probably a mean to complete-workflow goal. If the workflow needs instructions from an outside environment to work, it is not complete. I think that was the main idea behind this goal.

**Note**: Ruling on the second unsure row, hedged "probably".

### `goal@check@document-rules-are-checked`: as you say, this one should probably just merge with the root goal `##arg-59`

Round 4, by the owner. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> `goal@check@document-rules-are-checked`: as you say, this one should probably just merge with the root goal.

**Note**: Ruling on the third unsure row, hedged "probably".

### I agree with `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`, this will make sure we miss nothing. Each goal (cut) `##arg-60`

Round 4, by the owner. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> I agree with `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`, this will make sure we miss nothing. Each goal (not just refining ones) should also be analyzed for whether it is kept or not at all, though this can wait for when we start the rewrite of goals.

**Note**: Widens the harvest to every goal.

### About the goal-discussion thread: I am not sure whether keeping it is worth it. Since goals are expected to (cut) `##arg-61`

Round 4, by the owner. Bears on `thread@goals-at-the-root@goal-discussion-before-the-draft`.

> About the goal-discussion thread: I am not sure whether keeping it is worth it. Since goals are expected to become thinner, maybe this step is less needed.

### I agree on the milestone shape, though I am unsure whether the reworked standard, skill and reviewer can be (cut) `##arg-62`

Round 4, by the owner. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> I agree on the milestone shape, though I am unsure whether the reworked standard, skill and reviewer can be merged independently of the goal rewording itself. Maybe they should happen on the same branch.

### `goal@knowledge-architect@projects-are-split-into-components` stays a goal. Your wider form, "projects are split into modular units", is (cut) `##arg-63`

Round 4, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> - **`goal@knowledge-architect@projects-are-split-into-components`** stays a goal. Your wider form, "projects are split into modular units", is a new goal wording, so it is drafted under goal-setting during the rewrite, not decided here. One consequence to weigh then: stated at every layer (the code of one Component, the files of a design home), it becomes the ground for many more heads than today, and it competes with `only-needed-instructions`-style economy wherever a split adds files to read. That may call for a ranking.

**Note**: Deferred to the rewrite. The owner did not answer it.

### I agree with dropping the new step, and the issue's own evidence supports it. Both recorded instances were (cut) `##arg-64`

Round 4, by the session. Bears on `thread@goals-at-the-root@goal-discussion-before-the-draft`.

> I agree with dropping the new step, and the issue's own evidence supports it. Both recorded instances were goals of a new Component, and the cost the issue names is "a discussion per Component whose goals are drafted". Under root-only goals that occasion no longer exists.
>
> What remains of its mechanism, an intent forming when a consequence is shown, applies to the scope and ranking kinds. The existing step that shows a draft's reach covers it, and a thin draft is cheap to show early. So I propose the thread closes `ruled-out`, and the issue closes with the work, with that reason.

### Your doubt is right, and I can give it an argument: the rewrite of the 54 goals is the only test the new (cut) `##arg-65`

Round 4, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> Your doubt is right, and I can give it an argument: the rewrite of the 54 goals is the only test the new standard gets. `design@agent-skills@synthetic-evidence-not-built` forbids a run built to watch a session apply it. Merged alone, the standard would reach main untested, and main would hold installed text that says goals live at the root beside 33 Component goals.

### The checker cannot join that branch, though. Every commit of a branch is judged by the checker at its tip, so (cut) `##arg-66`

Round 4, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> The checker cannot join that branch, though. Every commit of a branch is judged by the checker at its tip, so a checker that refuses a Component's goals home would need all 33 goals gone in the first commit. Hence two slices:
>
> The cost of slice 1 is its size: one branch carries the installed text and every batch of your rulings. The record of each batch in `path@knowledge-architect@docs/goal-reviews/` is committed round by round, which is what that directory exists for.

**Note**: The table of the two slices is under "Decided design" or "Mapping tables".

### Today it is 6 paragraphs, a measure and 11 cases. Sorting its cases `##arg-67`

Round 5, by the session. Bears on `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`, `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.

> Today it is 6 paragraphs, a measure and 11 cases. Sorting its cases:
>
> All 11 are of this kind. None changes the direction; each closes a question the rewrite leaves to a head.

**Note**: Rewrite 1, the orchestration goal. The session calls the rewrites "illustrations of shape, not drafts for ruling".

### A finding from this one: `design@knowledge-architect@git-crate` says "It serves goal@git@a-pure-wrapper-over-git". With that goal gone, (cut) `##arg-68`

Round 5, by the session. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`, `thread@goals-at-the-root@goals-home-at-the-root-only`.

> A finding from this one: `design@knowledge-architect@git-crate` says "It serves `goal@git@a-pure-wrapper-over-git`". With that goal gone, the head stands on its own fact, that the rival "puts the checker's own dependencies into the build of every tool that needs git". So a head that cited a placement goal does not always need another goal; some stand on a fact alone.

**Note**: Rewrite 3, `git`'s goal turned into its head. Its carried sentence ends "*(Carried from a goal; not argued against a rival.)*", the marking that finding 1 of round 7 withdraws.

### My commitment was: if a goal of five sentences or fewer leaves two readers moving in different directions, (cut) `##arg-69`

Round 5, by the session. Bears on `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`, `thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`.

> My commitment was: if a goal of five sentences or fewer leaves two readers moving in different directions, the cases stay. Neither rewritten goal does. Every case either repeated the statement or closed a question of how much, which is what you want argued in design. So the three-layer block is where the detail you object to enters.

**Note**: Result of the test of `argument@goals-at-the-root@arg-44`.

### Your point still holds in one form: a later reader sometimes needs an example to get the direction. I propose (cut) `##arg-70`

Round 5, by the session. Bears on `thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`.

> Your point still holds in one form: a later reader sometimes needs an example to get the direction. I propose a synthesis as **#a-goal-carries-an-example-only-for-its-direction**: no fixed "Requires, Admits, Rules out" block; a goal's prose may carry an example where the statement alone would be read in the wrong direction. The reach by layer is still shown to you in the drafting exchange.

### One limit: both rewrites are purposes. I did not test a scope goal, and `goal@knowledge-architect@serves-one-kind-of-project` is the (cut) `##arg-71`

Round 5, by the session. Bears on `thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`, `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> One limit: both rewrites are purposes. I did not test a scope goal, and `goal@knowledge-architect@serves-one-kind-of-project` is the long one. A scope names what it excludes, so its examples may be the statement itself.

**Note**: A stated limit of the evidence. Not taken up afterwards in the transcript.

### The 17 searchers returned, but the check of their reports failed on 11 of them: 625 findings, mostly (cut) `##arg-72`

Round 5, by the session. Bears on no thread by name.

> The 17 searchers returned, but the check of their reports failed on 11 of them: 625 findings, mostly quotations copied from an entry's title instead of its body, and in two reports (agents 3 and 13) lines that were never graded.

**Note**: Fact about the first record search. Later: round 7 gives another cause: "The check of a searcher's report refuses a quotation that stops before a word's trailing punctuation, with a message that does not say so; this caused most of the 625 first-pass findings."

### I agree the admits and rules out cases were actually design territory. Your examples show it quite clearly `##arg-73`

Round 6, by the owner. Bears on `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`, `thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`.

> I agree the admits and rules out cases were actually design territory. Your examples show it quite clearly.

### This makes me want to open another proposal: maybe we should have another shape of admitted design entry, to (cut) `##arg-74`

Round 6, by the owner. Bears on `thread@goals-at-the-root@goal-interpretation-register`.

> This makes me want to open another proposal: maybe we should have another shape of admitted design entry, to record an "interpretation of a goal", which would be what we consider to be admitted or ruled out by it. Or we could keep the current shape. Do you think this case (or something similar) could be missing right now ?

**Note**: A proposal put as a question. Later: round 7, withdrawn by the owner.

### A design head already records one. `primer@design-heads` says a head's standing argument holds "the goal it (cut) `##arg-75`

Round 6, by the session. Bears on `thread@goals-at-the-root@goal-interpretation-register`, `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise`.

> A design head already records one. `primer@design-heads` says a head's standing argument holds "the goal it derives a constraint from", as a reference. The constraint a head derives is its reading of the goal. Three cases, all covered by the existing tests of what earns a head:
>
> And they are found from the goal: `cargo klarch show goal@<anchor>@<id>` lists every entry that cites it. So "every reading of this goal" is one command.

**Note**: The table of three cases is under "Decided design" or "Mapping tables".

### Take the orchestration case "Admits: the main session reading one known file itself". As a head it must name (cut) `##arg-76`

Round 6, by the session. Bears on `thread@goals-at-the-root@goal-interpretation-register`.

> Take the orchestration case "Admits: the main session reading one known file itself". As a head it must name a decision, its nearest rival and the fact that defeats the rival: for instance, the threshold below which a dispatch costs more context than it saves, with a measurement. As an "interpretation" entry it can be written with none of that. It would be the cases block again, moved one file over, and again decided with no rival argued. The head shape is what forces the argument.

### A second cost: a reading recorded apart from any decision has no site that depends on it, so nothing tells a (cut) `##arg-77`

Round 6, by the session. Bears on `thread@goals-at-the-root@goal-interpretation-register`.

> A second cost: a reading recorded apart from any decision has no site that depends on it, so nothing tells a later session when it is stale.

### Thinner goals make it more likely that two heads read one goal in two ways, in two Components, and nothing (cut) `##arg-78`

Round 6, by the session. Bears on `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise`.

> Thinner goals make it more likely that two heads read one goal in two ways, in two Components, and nothing compares them. Today the record search's goal agents and the design-conformance reviewer read against the goals, not head against head.
>
> I propose **#a-goal-s-reading-is-a-head-s-premise**: no new register; a head that derives a constraint from a goal states that constraint in one sentence beside the reference, so the readings of one goal can be laid side by side from `show`; and the design-conformance axis compares a new head's reading with the others that cite the same goal. The first half is current practice stated as a rule. The second is the addition.
>
> Its cost: the readings of one goal are spread over several design homes. Reading them all takes a command and several files, where a register would give one file.

**Note**: Later: round 12, the comparison moves to the decision-record axis (`argument@goals-at-the-root@arg-117`).

### The fact that would change my position: a reading you want recorded that no decision, built or refused, (cut) `##arg-79`

Round 6, by the session. Bears on `thread@goals-at-the-root@goal-interpretation-register`.

> The fact that would change my position: a reading you want recorded that no decision, built or refused, depends on. I could not construct one; if you have one in mind, it decides this.

**Note**: The owner gave no such reading and withdrew the thread in round 7.

### The search passed its check: 318 entries bear on the work `##arg-80`

Round 7, by the session. Bears on no thread by name.

> The search passed its check: 318 entries bear on the work.

**Note**: Figure of the first record search.

### So I change #a-removed-goal-is-harvested-sentence-by-sentence. A carried sentence becomes a head only when it (cut) `##arg-81`

Round 7, by the session. Bears on `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence`.

> So I change #a-removed-goal-is-harvested-sentence-by-sentence. A carried sentence becomes a head only when it passes an entry test and the batch argues it against a rival. Otherwise it goes nowhere, or to a `design` issue to argue later. This costs more work per goal, and it is the outcome you asked for: no detail kept without an argument.

**Note**: From finding 1. Withdraws the marking of `argument@goals-at-the-root@arg-54`. The owner agreed in round 8.

### My draft put the head that argues a Component's existence "in the design home of the Component enclosing it". (cut) `##arg-82`

Round 7, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`.

> My draft put the head that argues a Component's existence "in the design home of the Component enclosing it". `design@agent-skills@dependency-decision-home` says: "Which Components exist … are the root's design home's", on your choice of the root over the lowest enclosing Component. The draft now follows it, which matches `register@knowledge-architect@design` as it stands.

**Note**: From finding 2. Corrects the session's own draft. The owner agreed in round 8.

### `design@agent-skills@goal-met-in-tree-or-in-use` already refused a measure "kept apart from the goal and (cut) `##arg-83`

Round 7, by the session. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`.

> - `design@agent-skills@goal-met-in-tree-or-in-use` already refused a measure "kept apart from the goal and revised by agents", and records that you "chose the mechanism as the reading that gives a systematic check". The new fact that reopens it: the measure is where mechanism enters a goal, as the orchestration measure naming the retrospective's questions shows.

**Note**: From finding 3: the argument a reversal owes.

### `design@agent-skills@goal-carries-reading-examples` already defeated "cases shown in the drafting exchange (cut) `##arg-84`

Round 7, by the session. Bears on `thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`, `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`.

> - `design@agent-skills@goal-carries-reading-examples` already defeated "cases shown in the drafting exchange only", on the reading being lost when the record leaves. The new fact: in both rewrites every case closed a question of how much. The synthesis keeps an example where the direction needs one, which answers the old objection in part.

**Note**: From finding 3: the argument a reversal owes. "in part" is the session's own hedge.

### Intermediate Components. `design@agent-skills@intermediate-component-occasion` records your argument that goals (cut) `##arg-85`

Round 7, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> 1. **Intermediate Components.** `design@agent-skills@intermediate-component-occasion` records your argument that goals are the top of a Component, from which its design derives, and that goals form a tree where principles do not. Root-only goals withdraw it. My default: an intermediate Component exists where several Components share design decisions the rest of the project does not.

**Note**: Question 1. Later: round 8, the owner contests the default (`argument@goals-at-the-root@arg-96`) and the session withdraws it (`argument@goals-at-the-root@arg-101`).

### An exception that rests on a goal. `design@agent-skills@installed-skill-takes-precedence` is exempt from (cut) `##arg-86`

Round 7, by the session. Bears on `thread@goals-at-the-root@no-goal-refines-another`.

> 2. **An exception that rests on a goal.** `design@agent-skills@installed-skill-takes-precedence` is exempt from `behaviour-additions-need-real-use` because `goal@agent-skills@installed-text-works-in-any-environment` is a goal. You said it is "probably a mean" to the complete-workflow goal. As a head it binds only as a presumption. My default: the exception cites the root goal directly and stands.

**Note**: Question 2. The owner agreed in round 8.

### A wish recorded in the other direction. (cut) `##arg-87`

Round 7, by the session. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> 3. **A wish recorded in the other direction.** `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules` records that you wanted more of your intent moved into goals. My default: this discussion replaces that wish, and the issue's summary is reworded on the work's branch.

**Note**: Question 3. The owner agreed in round 8, and left the fix to that issue's discussion (`argument@goals-at-the-root@arg-98`).

### The work is larger than drafted. 22 more heads state what the work makes false, in model, check, query, (cut) `##arg-88`

Round 7, by the session. Bears on no thread by name.

> - **The work is larger than drafted.** 22 more heads state what the work makes false, in `model`, `check`, `query`, `create`, `mutate` and `agent-skills`. Example: `design@model@registers-are-declared` gives as reason "their storage is what the word component means".

**Note**: Figure of the first record search.

### No moment asks for the pursuit of a goal. Rule 6 is today the only text that asks what a project builds to (cut) `##arg-89`

Round 7, by the session. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`, `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.

> - **No moment asks for the pursuit of a goal.** Rule 6 is today the only text that asks what a project builds to learn whether a goal is met (`issue@agent-skills@a-target-project-collects-no-feedback-from-use`). I propose a check: a goal that no design head cites is reported, since nothing pursues it.

**Note**: Later: round 8, the owner defeats the check (`argument@goals-at-the-root@arg-99`) and the session withdraws it (`argument@goals-at-the-root@arg-106`).

### Rule 8 and the scope kind overlap. Rule 8 admits an exclusion only when it names the design it excludes. I (cut) `##arg-90`

Round 7, by the session. Bears on `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`.

> - **Rule 8 and the scope kind overlap.** Rule 8 admits an exclusion only when it names the design it excludes. I propose the scope kind keeps that condition.

### One direction changes less than I said. `design@agent-skills@goal-one-meaning-any-breadth` already defines two readings as two (cut) `##arg-91`

Round 7, by the session. Bears on `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.

> - **One direction changes less than I said.** `design@agent-skills@goal-one-meaning-any-breadth` already defines two readings as two directions. What is new is that a question of when, how much or by what means is no finding. I propose the clause "every 'every' names the set it ranges over" leaves, since it produces such clauses.

**Note**: Narrows `argument@goals-at-the-root@arg-21`.

### A split comes due. Editing `design@check@every-path-names-its-anchor` meets the re-entry point of (cut) `##arg-92`

Round 7, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> - **A split comes due.** Editing `design@check@every-path-names-its-anchor` meets the re-entry point of `issue@check@core-heads-bundle-decisions-with-many-citing-sites`, 25 citing sites. I place that split in slice 2.

### The shipped text writes the form that names every Component's goals home at 7 sites `##arg-93`

Round 7, by the session. Bears on `thread@goals-at-the-root@goals-home-at-the-root-only`, `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> - **The shipped text writes `register@*@goal` at 7 sites**, which assumes every Component has a goals home. Slice 2 owes a form that names the root's.

**Note**: Count.

### Five questions belong to the rewrite, not here: whether `goal@agent-skills@goals-state-the-owner-s-intent`, (cut) `##arg-94`

Round 7, by the session. Bears on no thread by name.

> - **Five questions belong to the rewrite, not here**: whether `goal@agent-skills@goals-state-the-owner-s-intent`, `relocation-needs-only-goal-changes` and `a-refused-invocation-does-no-work` stay goals, the wording of `only-needed-instructions` once measures leave, and the reason the gates library depends on nothing of the checker.

**Note**: Deferred to the rewrite.

### #component-goals-optional is closed ruled-out: a goal anchored in a Component names a part whose existence (cut) `##arg-95`

Round 7, by the session. Bears on `thread@goals-at-the-root@component-goals-optional`.

> - #component-goals-optional is closed `ruled-out`: a goal anchored in a Component names a part whose existence comes from a design choice.

**Note**: The closing reason, under "Decided".

### maybe this needs further discussion. The split on design was argued to be flawed. TBH, I think the split that (cut) `##arg-96`

Round 8, by the owner. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> 1. maybe this needs further discussion. The split on design was argued to be flawed. TBH, I think the split that would not risk problems should be on a different axis than design. I think it is still about purpose.

**Note**: The whole paragraph, with the `klarch` binary example and its hedges, is the owner\'s message of round 8. Quoted there further: "I am not sure this split is the one that makes sense. And maybe it actually would not really benefit from the intermediate component."

### If we find none that is general enough, we can just leave intermediate component splits to per project (cut) `##arg-97`

Round 8, by the owner. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> If we find none that is general enough, we can just leave intermediate component splits to per project design, and give no generic guideline about it, which would be better than a flawed guideline.

**Note**: The fallback that became the thread.

### agreed, it replaces the wish. The fix to that should just be that the default can enter the record with no (cut) `##arg-98`

Round 8, by the owner. Bears on no thread by name.

> 3. agreed, it replaces the wish. The fix to that should just be that the default can enter the record with no ruling. Or maybe the opposite, a default that needs my ruling stops the work and asks. I am unsure. This should left to thqt issue's discussion anyway.

**Note**: Answer to question 3; two opposite fixes named, neither chosen.

### About "No moment asks for the pursuit of a goal.": a check is not the right solution. What about a new (cut) `##arg-99`

Round 8, by the owner. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`, `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.

> About "No moment asks for the pursuit of a goal.": a check is not the right solution. What about a new project with only goals and no implementation or design ? Under a check, a new project cannot exist, and a new goal can never enter unmet.

**Note**: Defeats `argument@goals-at-the-root@arg-89`.

### This should be left open for now. This is not a new problem anyway. I think this can never be met (cut) `##arg-100`

Round 8, by the owner. Bears on `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.

> This should be left open for now. This is not a new problem anyway. I think this can never be met mechanically anyway. Right now, the only thing that drove toward meeting goals was that design sessions were asked to ground in goals, and that the record search looks at all goals too. This was already plenty, and worked fine. Since the goals become less detailed, this might work less well.
>
> I think the reentry to that would be a discussion about a review axis against project goals and evidence collection at merge and/or plans, and an audit axis trying to detect whether goals are met, and whether the project is collecting enough evidence of goals being met or not.

### First, I withdraw my default. You are right that it was already argued as flawed: (cut) `##arg-101`

Round 8, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> First, I withdraw my default. You are right that it was already argued as flawed: `design@agent-skills@intermediate-component-occasion` refuses a layout drawn from shared design decisions, because it depends on the order in which the decisions were met. I proposed the rival that head defeated.

**Note**: Withdrawal of the default of `argument@goals-at-the-root@arg-85`.

### Your lead, tested. Stated generally: an intermediate Component groups the Components that together make one (cut) `##arg-102`

Round 8, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> Your lead, tested. Stated generally: **an intermediate Component groups the Components that together make one thing the project delivers.** On this repository it gives the grouping you describe:
>
> It keeps the property the old head wanted: a deliverable answers to a purpose goal at the root, so the layout does not depend on the order of decisions. But I see two weaknesses:

**Note**: The grouping table is under "Decided design" or "Mapping tables". A lead tested and not kept.

### What the group would hold is design. With no goals home, an intermediate Component carries a design home, (cut) `##arg-103`

Round 8, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> - **What the group would hold is design.** With no goals home, an intermediate Component carries a design home, issues and a `CLAUDE.md`. So the deliverable is the occasion, and shared design is still the content. The benefit exists only if those crates share decisions, which is the question the old head could not settle.

**Note**: First weakness of the lead.

### No instance exists. This repository has none (`design@knowledge-architect@no-intermediate-component`), and I (cut) `##arg-104`

Round 8, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> - **No instance exists.** This repository has none (`design@knowledge-architect@no-intermediate-component`), and I know of none in a consumer. `design@agent-skills@behaviour-additions-need-real-use` asks evidence from real use for an installed guideline, and `goal@knowledge-architect@projects-are-split-into-components` says "what makes a split good is learned from use, so this goal fixes no rule".

**Note**: Second weakness of the lead; the fact the session rests the fallback on.

### So I support your fallback, on that second fact: #no-installed-rule-on-when-an-intermediate-component-exists. (cut) `##arg-105`

Round 8, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> So I support your fallback, on that second fact: **#no-installed-rule-on-when-an-intermediate-component-exists**. The installed text keeps what an intermediate Component is and that its design binds what it encloses. It gives no occasion. A project argues one in its own design. The deliverable lead is recorded in an issue of kind `question`, which replaces `root-goals-about-the-checker-may-belong-to-an-intermediate-component`, with the `klarch` grouping as its first case.
>
> The cost: a session placing work gets no guidance on when to group, until use provides it.

**Note**: Later: round 9, the issue of kind `question` is dropped: "no issue is opened for it" (`argument@goals-at-the-root@arg-110`).

### Your argument defeats it, and the record agrees: `design@agent-skills@goal-drafted-to-move-the-tree` says a new (cut) `##arg-106`

Round 8, by the session. Bears on `thread@goals-at-the-root@what-shows-a-goal-met-is-design`.

> Your argument defeats it, and the record agrees: `design@agent-skills@goal-drafted-to-move-the-tree` says a new goal "is expected to be unmet". A check that reports a goal no head cites would report every new goal and every new project.

**Note**: Withdrawal of the check of `argument@goals-at-the-root@arg-89`.

### Cause: goals grow thinner, so grounding in the goals and the record search may hold work to them less well (cut) `##arg-107`

Round 8, by the session. Bears on `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.

> - **Cause**: goals grow thinner, so grounding in the goals and the record search may hold work to them less well than today.
>
> - **Evidence that reopens it**: a retrospective or a review finding of merged work that moved against a goal with no procedure reporting it.
>
> - **Re-entry point**: a `design` issue that slice 1 opens, naming your three leads: a review axis against the goals at a merge or a plan, the collection of evidence at those moments, and an audit axis on whether goals are met and whether enough evidence is collected. It cites `issue@agent-skills@a-target-project-collects-no-feedback-from-use`, which this work makes worse.

**Note**: Later: round 12, the re-entry point changes: no new issue, two existing issues are updated (`argument@goals-at-the-root@arg-118`).

### If the usage of that actually surfaces in a consumer, we can learn from it and observe whether it works well (cut) `##arg-108`

Round 9, by the owner. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> If the usage of that actually surfaces in a consumer, we can learn from it and observe whether it works well or not, and why. For now, stay silent on that. Even on the split I proposed here, I am unsure whether it would have any benefit at all. The split is out of scope for today btw, it was just an example.

### Cause 3 is the one I think most likely. The design already tolerates a partial state, because nothing refuses (cut) `##arg-109`

Round 9, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> **Cause 3 is the one I think most likely.** The design already tolerates a partial state, because nothing refuses a Component's goals home until slice 2. So there is an option you have not been shown: slice 1 merges once the standard has been applied to the root's 21 goals and the 9 of `agent-skills`, and the other 24 follow on a second branch. Your reason for one branch was that the rewrite tests the standard, and 30 goals of the 54 test it. I keep one branch as you ruled, with the batches committed theme by theme, unless you want the option.

**Note**: An option offered against premortem cause 3. Later: round 10, the owner keeps one branch (`argument@goals-at-the-root@arg-111`). The option lost.

### One item I drop rather than record: the klarch grouping. You called it an example and out of scope, so no (cut) `##arg-110`

Round 9, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> One item I drop rather than record: the `klarch` grouping. You called it an example and out of scope, so no issue is opened for it. `root-goals-about-the-checker-may-belong-to-an-intermediate-component` closes with the work, answered by goals living at the root.

**Note**: Withdraws the issue of kind `question` of `argument@goals-at-the-root@arg-105`.

### keep one branch, against cause 3. I have no plan on starting any work on main while this branch is open (cut) `##arg-111`

Round 10, by the owner. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> keep one branch, against cause 3. I have no plan on starting any work on main while this branch is open anyway. We can spend a bit of time on this.

**Note**: Defeats the option of `argument@goals-at-the-root@arg-109`, on a fact the owner holds.

### Recorded. Cause 3 keeps one branch, on a fact you hold: no other work starts on main while the branch is (cut) `##arg-112`

Round 10, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> Recorded. Cause 3 keeps one branch, on a fact you hold: no other work starts on main while the branch is open, which removes the half of the cause about falling behind. The other half, the weight of the rewrite, you accept.

**Note**: The session's reading of the owner's word. The owner's next message (`argument@goals-at-the-root@arg-113`) answers the second half.

### If the weight of the rewrite is too much, multiple sessions can do. This is not a problem anyway `##arg-113`

Round 11, by the owner. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`.

> If the weight of the rewrite is too much, multiple sessions can do. This is not a problem anyway.

### Noted, and the design supports it: each batch's record is committed in docs/goal-reviews/ round by round, and (cut) `##arg-114`

Round 11, by the session. Bears on `thread@goals-at-the-root@standard-and-rewrite-on-one-branch`, `thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`.

> Noted, and the design supports it: each batch's record is committed in `path@knowledge-architect@docs/goal-reviews/` round by round, and the milestone document carries the standard and the sorting rules, so a later session resumes from the tree and not from this conversation. I will write the slice's spec with that in mind: one batch per theme, each able to start cold.

**Note**: Later: round 12, "one batch per theme" is contested by the owner for the edits (`argument@goals-at-the-root@arg-115`) and withdrawn for them (`argument@goals-at-the-root@arg-116`).

### The rewords should probably not happen in batches per theme. Or we will have half finished states, where part (cut) `##arg-115`

Round 12, by the owner. Bears on `thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`.

> The rewords should probably not happen in batches per theme. Or we will have half finished states, where part of the goals are ported, and the other is not. This is a problem, because the whole goal home have to be consistent with no contradictions. 
>
> The analysis of each sentence can be done per batch. But the edits to the goals cannot. They should happen under the whole set review procedure, edit all goals, one reviewer per theme and one global reviewer, then another round.

### You are right, and the record already holds the procedure you describe. `design@agent-skills@whole-set-rounds`: (cut) `##arg-116`

Round 12, by the session. Bears on `thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`.

> You are right, and the record already holds the procedure you describe. `design@agent-skills@whole-set-rounds`: "Each round dispatches together one goal reviewer per set of goals that touch each other, in draft mode, and one in whole-set mode … the next round waiting for the answer." I withdraw "theme by theme" for the edits. Slice 1 becomes, in order:

**Note**: The order of slice 1 is under "Decided design" or "Mapping tables".

### The comparison of readings moves to another axis. I put it on the design-conformance axis. (cut) `##arg-117`

Round 12, by the session. Bears on `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise`.

> - **The comparison of readings moves to another axis.** I put it on the design-conformance axis. `design@agent-skills@verdicts-reach-the-conformance-review` runs that axis at a merge only when a commit message carries a record search's lines, so a head written on another branch would be compared by nobody. The decision-record axis reads every head a diff writes. The comparison goes there.

**Note**: Corrects `argument@goals-at-the-root@arg-78`. From the second record search.

### The parked thread gets no new issue. `issue@agent-skills@audit-axes-beyond-the-design-record` already keeps the (cut) `##arg-118`

Round 12, by the session. Bears on `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.

> - **The parked thread gets no new issue.** `issue@agent-skills@audit-axes-beyond-the-design-record` already keeps the audit leads, and `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear` the review lead. Slice 1 updates both. The first one's bullet that treats a goal no head serves as a finding is removed, as you ruled.

**Note**: Corrects the re-entry point of `argument@goals-at-the-root@arg-107`.

### One of your leads meets a rejected design. `rejected@agent-skills@goal-measured-by-evidence-from-use` refused (cut) `##arg-119`

Round 12, by the session. Bears on `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`, `thread@goals-at-the-root@what-shows-a-goal-met-is-design`.

> - **One of your leads meets a rejected design.** `rejected@agent-skills@goal-measured-by-evidence-from-use` refused evidence from use as what shows a goal met. The parked thread's discussion will owe a new argument against it. I note that in the issue.

### A cost of readings held in heads. `the-goal-s-ruled-out-case-reads-two-ways-on-a-harness-tag` (cut) `##arg-120`

Round 12, by the session. Bears on `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise`, `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.

> - **A cost of readings held in heads.** `the-goal-s-ruled-out-case-reads-two-ways-on-a-harness-tag` reports that a session reading a goal alone found a head in conflict with it and asked you again. Thin goals make this more frequent. It is not silent, since it reaches you each time, so it is no tripwire.

**Note**: A cost named and accepted by the session as not silent.

**Note**: The issue states it as what a later session would do, not as something that happened.

### This repository's head of absence leaves. `design@knowledge-architect@no-intermediate-component` argues only (cut) `##arg-121`

Round 12, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> - **This repository's head of absence leaves.** `design@knowledge-architect@no-intermediate-component` argues only from the installed occasion. With the installed text silent, it has no argument left, and slice 1 removes it.

### Silence refuses nothing. The old head refused four other occasions for an intermediate Component, grouping by (cut) `##arg-122`

Round 12, by the session. Bears on `thread@goals-at-the-root@no-installed-rule-on-when-an-intermediate-component-exists`.

> - **Silence refuses nothing.** The old head refused four other occasions for an intermediate Component, grouping by a shared principle among them. A project may now argue any of them.

**Note**: A consequence of the silence, stated and not argued further.

**Note**: The old head lists six rivals, three of which name an occasion; "Decided design" says which are refused no longer.

## New names, in one place

```
purpose, scope, ranking      the three kinds of goal; in `path@agent-skills@fragments/goal-standard.md`
theme file                   one file of the root's goals home, a directory behind a README;
                             in `planned@knowledge-architect@docs/goals/`
the analysis                 the record of slice 1's sorting of each goal and of each sentence;
                             one file in `path@knowledge-architect@docs/goal-reviews/`
```

Existing names the design uses: the register `goal`, `GOAL_REGISTER` in
`path@model@src/manifest.rs`; `home_text` in `path@create@src/command.rs`.

## Decided design

### Goals live at the root only

A goal is in the root's goals home, and nowhere else. The home is a directory of theme files behind
a README, the form a heading register already admits. A reference to a goal names the root as its
anchor, whatever the file. A Component carries no goals home, and neither does a location. A
Component's purpose is the statement of the design head that argues its existence, in the root's
design home, per `design@agent-skills@dependency-decision-home`. Creating or splitting a Component
needs a design argument and no goal-setting session. Three Components have no such head in the
root's design home today: slice 1 writes one for `check` and one for `xtask`, each argued under
the design skill, and moves the head of `gates` there.

The argument: rule 1 of the standard says a goal "names no part whose existence comes from a design
choice", and a Component is such a part. The order becomes goals, then a head that argues a split,
then the Component, so a redesign of the split touches no goal. The nearest rival,
`thread@goals-at-the-root@component-goals-optional`, fails the same test: a goal anchored in a
Component does not exist in the project emptied of its design, and moving code between Components
would move goals only the owner may change.

What it costs, as the owner was told: the owner no longer rules each Component's purpose, which an
agent may reverse on argument and report; a session in one crate reads the theme files that bear on
it; the checker's change breaks every reference to a Component goal.

### A goal is a purpose, a scope or a ranking

| kind | test |
| --- | --- |
| purpose | removed, some part of what the project provides, or is to provide, has no reason left to exist |
| scope | it bounds whom or what the purposes serve, and names the design it excludes |
| ranking | two purposes compete, and it says which comes first, across decisions |

Anything else is design. The owner's word enters design through a fact the owner holds and the
weighing of a cost at a fork, not through a goal on one decision. A goal of the three kinds stays
a way for the owner to change a decision, as `goal@knowledge-architect@the-owner-decides` says. A
kind of interface, such as a command line, is a goal only where it passes the purpose test. The
purpose test admits a purpose that nothing built serves yet, a new goal being expected to be unmet,
per `design@agent-skills@goal-drafted-to-move-the-tree`.

The argument is the owner's: a goal is what drives the project to exist, and no argument defends a
purpose. The scope and the ranking answer the one class the purpose test fails: removing a scope
makes nothing lose its reason to exist. The rival,
`thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`, needed a ruling of the owner as
its stop.

### No goal refines another

No goal holds a line "It refines". A statement grounded on another goal is a design head that cites
that goal. A goal whose intent is independent of the goal it named stands alone, without the line.
Rule 7's clause on refinement leaves. The first mapping table gives a first sorting of the 24 goals
that hold a line "It refines", by title and slug, to which the owner agreed. The analysis reads the
body of each sentence by sentence, may overturn a row with its reason, and the owner rules each
goal in the whole-set review; no row binds that ruling.

The fact behind it: 10 of the 24 refine `goal@knowledge-architect@agents-get-a-complete-workflow`,
the widest goal, because the standard has the drafter of a Component goal name a goal of the root.

### A goal states one direction

Rule 2 keeps "every term has one meaning", and its test of two readings as two directions, which
`design@agent-skills@goal-one-meaning-any-breadth` already states. What is new: a question of when,
how much or by what means is no finding, and the goal reviewer does not block on it. A design head
closes such a question, on argument against a rival. This is a rule for the drafter and the
reviewer of a goal: it adds no instruction for an agent that applies a goal and finds it open, per
`design@agent-skills@no-instruction-for-a-goal-read-two-ways`. The clause that every universal
names the set it ranges over leaves, since it produces such clauses, D2.

The cost, which the owner called the feature: two sessions may resolve an open point differently
until a head settles it.

### A goal carries no measure

Rule 6 leaves. No goal holds "It is met while". How the project pursues a goal, and how it learns
from use whether the goal is met, are design decisions, in heads that cite the goal. No check
reports a goal that no head cites: a new goal is expected to be unmet, and a new project holds
goals before any design.

This reverses `design@agent-skills@goal-met-in-tree-or-in-use`, which refused a measure "kept apart
from the goal and revised by agents". The fact its record did not hold: the measure is where a
mechanism enters a goal, as the measure of `goal@agent-skills@session-orchestrates-the-work` names
the retrospective's questions.

### A goal carries an example only for its direction

A goal entry holds no fixed block of cases. Its prose may carry an example where the statement
alone would be read in the wrong direction. The reach by layer, what a draft requires, admits and
rules out, is still shown to the owner in the drafting exchange.

This reverses `design@agent-skills@goal-carries-reading-examples`, which kept the cases so that the
reading is not lost. The fact its record did not hold: in the two goals that stayed goals among
three rewritten as a test, every case either repeated the statement or closed a question of how
much. The test covered two purposes and no scope.

### A removed goal is sorted sentence by sentence

Every goal is judged for whether it stays. For each goal removed, merged or shortened, the analysis
holds a table of its sentences, each with one destination of the second mapping table. A sentence
becomes a head only where it passes an entry test of `primer@design-heads` and the analysis argues
it against a rival. No head says it was carried and not argued. Such a head is a decision that
creates a head, so it goes through the in-change path of `skill@knowledge-architect-design`, per
`design@agent-skills@new-or-reshaped-head-needs-design`: the analysis holds its argument, and the
message of the commit that writes the head carries the deliberation.

The owner rules the analysis before any goal is edited: for each goal, whether it is kept, merged,
turned into a head or removed, and every sentence sent nowhere, which is shown to the owner before
it is deleted. The reviewers of the whole-set review read every row of the analysis, a line this
repository adds to their briefs for this one review; the shipped goal reviewer gains no such duty.

### A goal's reading is a head's premise

An interpretation of a goal has no register. A head that derives a constraint from a goal states
that constraint in one sentence beside its reference to the goal. The decision-record axis of a
review compares the reading of a head a diff writes with the readings of the other heads that cite
the same goal, which `cargo klarch show` lists, and reports only two readings that cannot both
hold: several heads may read one goal in ways that differ and agree.

Both are instructions about how an agent behaves, which
`design@agent-skills@behaviour-additions-need-real-use` and
`goal@knowledge-architect@the-workflow-improves-through-real-use` admit only on evidence from real
use or on a defect provable by reading. The first has such evidence: two recorded cases of a
reading held in a head and not found from the goal, in
`issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves` and in the
closed entry on the transcript goal's case. The comparison rests on a prediction, that thinner
goals make two heads read one goal in opposite ways. Whether it is built or parked as an issue is
D3.

The rival, `thread@goals-at-the-root@goal-interpretation-register`, loses on one fact: a head must
name a decision, its rival and the fact that defeats the rival, and an interpretation recorded apart
from a decision is argued against none. The axis is the decision-record one because
`design@agent-skills@verdicts-reach-the-conformance-review` runs the design-conformance axis at a
merge only on a commit message that carries a search's line with a verdict other than `constrains`
and `unrelated`.

### The installed text is silent on when an intermediate Component exists

The installed text keeps what an intermediate Component is, where a decision on the code of several
Components goes, and that its design binds what it encloses. It states no occasion. A project argues
one in its own design. No instance in use supports a replacement of the occasion that leaves, per
`design@agent-skills@behaviour-additions-need-real-use`. The old head lists six rivals. The three
that name an occasion, a shared principle, a split and a count of Components, are refused no
longer. The two that name a form, a declared set and a crate whose Components are its modules,
stay refused. The sixth is the absence of any intermediate Component.

### The goals are edited in one pass

Slice 1 runs in this order: the standard, the skill and the reviewer; the analysis, per theme,
committed, which edits no goals home; then every goal edited in one pass in the working tree and
reviewed in the rounds of `design@agent-skills@whole-set-rounds`, one reviewer per theme and one
over the whole set, the owner answering between rounds.

### The standard and the rewrite land on one branch

Slice 1 is one branch: the new standard reaches the main branch with the goals it was applied to,
since the rewrite of this repository's goals is the only test the standard gets. The owner keeps
one branch against its weight on a fact the owner holds: no other work starts on the main branch
while it is open, and several sessions may carry it. The checker's change is slice 2, since every
commit of a branch passes the checker at its tip. The two slices ship in one version: no release is
made between them, and the changelog's Migration entry is written once, for both.

## Mapping tables

**The goals that state a refinement.** A first sorting by title of the 24 that hold a line "It
refines", to which the owner agreed in round 4, the three unsure rows as the owner answered them,
with the hedges of those answers. "Stands alone" names the kind it is judged as. The analysis may
overturn any row, and the owner rules each goal.

| goal | outcome |
| --- | --- |
| `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` | stands alone: purpose |
| `goal@knowledge-architect@the-workflow-leads-to-better-code` | stands alone: purpose |
| `goal@knowledge-architect@projects-are-split-into-components` | stays a goal: the owner leans to keeping it; a wider wording is drafted in the review |
| `goal@agent-skills@only-needed-instructions` | stands alone: ranking |
| `goal@agent-skills@session-orchestrates-the-work` | stands alone: the owner finds it "pretty much orthogonal" to the goal it names; whether it is a purpose or a means is open |
| `goal@agent-skills@installed-text-works-in-any-environment` | head: the owner finds it "probably a mean" to the complete-workflow goal |
| `goal@agent-skills@one-skill-per-activity` | head |
| `goal@agent-skills@no-contradiction-in-workflow` | head |
| `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` | head |
| `goal@agent-skills@reviews-judge-what-no-tool-can` | head |
| `goal@agent-skills@installed-text-works-in-every-target-project` | head |
| `goal@agent-skills@goals-state-the-owner-s-intent` | head by its title; whether it stays a goal is open |
| `goal@check@relocation-needs-only-goal-changes` | stands alone by its title: purpose; whether it stays a goal is open |
| `goal@check@projects-add-their-own-checks` | stands alone: purpose |
| `goal@check@document-rules-are-checked` | merged into `goal@knowledge-architect@documentation-stays-consistent`: the owner finds it "should probably just merge" |
| `goal@check@documents-stay-current` | head |
| `goal@check@checks-a-commit-from-its-hook` | head |
| `goal@model@recorded-items-can-be-referenced` | stands alone: purpose |
| `goal@create@a-project-adopts-by-one-command` | head |
| `goal@gates@gates-from-a-list` | head |
| `goal@mutate@a-named-mutation-is-checked-safely` | head |
| `goal@query@the-record-is-consulted-as-the-workflow-requires` | head |
| `goal@testing@an-extension-is-tested-over-a-project` | head |
| `goal@transcript@a-session-reads-past-sessions` | head |

`goal@model@a-tool-reads-the-documents-with-no-check` states inside a paragraph that it refines
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`; the discussion did not sort it. It and
the 29 goals that refine none have no row: the analysis judges each by the three kinds.

**The destination of a sentence of a goal removed, merged or shortened.**

| the sentence states | destination |
| --- | --- |
| a shape one text or one piece of code already holds, with no argued reason | nowhere: the text or the code keeps it, free to change on argument |
| a shape an existing head already argues | that head; the goal's copy is deleted |
| a reason respected at several sites, or an interface others consume, which the analysis argues against a rival | a new head, citing the goal it is grounded on |
| such a reason the analysis does not argue | an issue of kind `design`, which argues it later |
| a ruling of the owner on a cost or a fact | the head it bears on, attributed to the owner; where no head exists, a new head argued as above, or the issue |
| what the goal it stays in, or the goal a merged goal joins, still states | that goal |
| a repetition of the goal's own statement | nowhere |

## Losing alternatives

- `thread@goals-at-the-root@component-goals-optional`, lost to
  `thread@goals-at-the-root@goals-home-at-the-root-only`: a goal anchored in a Component names a
  part whose existence comes from a design choice.
- `thread@goals-at-the-root@a-goal-is-what-argument-cannot-defend`, withdrawn for
  `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`: it needed the owner's
  ruling as its stop.
- `thread@goals-at-the-root@goal-discussion-before-the-draft`, ruled out: both recorded instances
  were goals of a new Component, an occasion that leaves.
- `thread@goals-at-the-root@goal-cases-stay-in-the-discussion`, superseded by
  `thread@goals-at-the-root@a-goal-carries-an-example-only-for-its-direction`: a later reader
  sometimes needs an example to get the direction.
- `thread@goals-at-the-root@goal-interpretation-register`, withdrawn for
  `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise`.
- Inside threads, with no slug of their own: a head carried from a goal and marked as not argued,
  which brought back `rejected@agent-skills@owner-intent-entry-test`; a check that reports a goal
  no head cites, which fails on a new goal and on a new project; an intermediate Component created
  where Components share design decisions, which `design@agent-skills@intermediate-component-occasion`
  had refused, and one created for a set of Components delivered as one unit, the owner's lead,
  left with no instance in use; the goal edits made theme by theme, which leaves a goals home half
  ported; a Component's head in the design home of the Component enclosing it, against
  `design@agent-skills@dependency-decision-home`; a new issue for the parked thread, two issues
  holding part of its leads, which slice 1 extends; slice 1 merged after 30 goals of the 54, which the owner refused.

## Readings

None: the work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| thin goals stop steering: merged work moves against a goal and no procedure reports it | `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`, `thread@goals-at-the-root@what-shows-a-goal-met-is-design` | named as a cause for the harvest of slice 1 to judge as a tripwire; it is the cause of the parked thread |
| the root's goals home ends larger than the 13 homes are today | `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`, `thread@goals-at-the-root@goals-home-at-the-root-only` | becomes `acceptance@goals-at-the-root@the-goals-home-is-not-larger` |
| the rewrite is too heavy for one branch | `thread@goals-at-the-root@standard-and-rewrite-on-one-branch` | named as a cause for the harvest of slice 1 to judge; expected to fail condition 1, the work being stopped by it. The owner keeps one branch: no other work starts on main meanwhile, and several sessions may carry it |
| with measures gone, a review removes an installed instruction that a measure protected, `goal@agent-skills@only-needed-instructions` keeping an instruction where a goal's body or measure states its act | `thread@goals-at-the-root@what-shows-a-goal-met-is-design` | converted into a clause of slice 1: no installed instruction is removed on the need test before the whole-set review has ruled that goal |
| two heads read one goal in opposite directions | `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise` | converted into the comparison by the decision-record axis, if D3 keeps it; otherwise the issue D3 opens holds it |
| the detail leaves the goals and returns in heads with thin arguments | `thread@goals-at-the-root@a-removed-goal-is-harvested-sentence-by-sentence` | named as a cause for the harvest of slice 1 to judge; expected to fail condition 1 of the tripwire standard, the decision-record axis judging each head's argument |
| an agent reverses a Component's purpose on argument and the owner learns it late | `thread@goals-at-the-root@goals-home-at-the-root-only` | named as a cause for the harvest of slice 1 to judge; expected to fail condition 1, the turn's "Decided" section and the transcript axis reporting it |
| a consumer does not move its pin, the review of its whole goal set costing too much | slice 2 | named as a cause for the harvest of slice 2 to judge; expected to be dropped on a doubt, the owner running the consumers |
| a session reading a goal alone finds a head in conflict with it and asks the owner again | `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise` | named as a cause for the harvest of slice 1 to judge; expected to fail condition 1, since it reaches the owner each time |

## Acceptance criteria

### The root's goals home after the rewrite holds no more words than the 13 goals homes held before it `##the-goals-home-is-not-larger`

Guards `thread@goals-at-the-root@a-goal-is-a-purpose-the-project-exists-for`. Judged at slice 1.
Fires when `wc -w` over the files of the root's goals home, after the whole-set review's last
round, exceeds 19,045, the total of the 13 goals homes at the commit this document was written on.
Its firing shows that the three kinds do not thin the goals, and reopens the thread.

It is a ceiling on the whole home, never a bound on one goal:
`design@agent-skills@goal-entry-shape` argues against a bound on a goal's length, and that
argument stands. It is a weak observable: 54 measures and 183 case lines leave the goals, so the
home shrinks unless the kept goals grow by as much. No tighter bound has a basis in a measurement.

## Implementation sequence

1. [The standard, and the rewrite of this repository's goals](standard-and-rewrite.md):
   `spec@goals-at-the-root@standard-and-rewrite`. It fails alone on a goal the new standard cannot
   sort, or on a round of the whole-set review that does not converge.
2. [The register `goal` is the root's alone](checker-root-only.md):
   `spec@goals-at-the-root@checker-root-only`. It fails alone on a tree the checker refuses for a
   goals home, or accepts with one outside the root.

## Order rationale

Slice 1 comes before slice 2 because a checker that refuses a Component's goals home needs every
Component goal gone in the first commit of its branch, per `primer@branch-commits`, and slice 1 is
what removes them, leaving each Component's goals home empty, which the checker accepts today.

## Defaults awaiting the owner

- **D1**, not ruled. The criterion `criterion@goals-at-the-root@work-is-held-to-thinner-goals` is
  weighed and unmet. Default: it is accepted as unmet, its thread parked, on the owner's words that
  the question is left open for now. It bears on
  `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit`.
- **D2**, not ruled. The clause of rule 2 that every universal in what a goal requires or rules out
  names the set it ranges over leaves the standard. The session proposed it in round 7, and the
  owner's answers named neither it nor the table row that holds it. It bears on
  `thread@goals-at-the-root@a-goal-states-a-direction-and-leaves-the-rest-to-design`.
- **D3**, not ruled, from the design-conformance review. The comparison of the readings of one goal
  by the decision-record axis is an instruction about how an agent behaves that rests on a
  prediction, against `goal@knowledge-architect@the-workflow-improves-through-real-use`. The owner
  approved the thread that holds it. Default: slice 1 does not build the comparison, and opens an
  issue of kind `design` that holds it with the cause it answers; the sentence a head states
  beside its reference to a goal is built. It bears on
  `thread@goals-at-the-root@a-goal-s-reading-is-a-head-s-premise`.
- **D4**, not ruled, from the design-conformance review. Slice 1 installs the new standard before
  the one pass, so between its first step and the one pass the branch holds installed text that
  `goal@agent-skills@goals-state-the-owner-s-intent` contradicts, that goal stating that what shows
  a goal met is in what the project builds. The first search asked that this goal be ruled with the
  standard, before the others. Default: the order stands, the state never reaching the main
  branch, and the whole-set review rules that goal with the others. It bears on
  `thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`.

One question is put to the owner at slice 2's design audit and has no default built before it: the
form the shipped text writes for the root's goals home, which slice 2's spec states.

## Harvest

| what | where | when |
| --- | --- | --- |
| each slice's decisions | its spec's harvest row | at that slice's landing |
| the reversed and rewritten entries of "What is already decided" | each in its own home, by the slice its row names | at that slice's landing |
| the losing alternatives above, and the three heads the work reverses, `design@agent-skills@goals-required`, `design@agent-skills@goal-met-in-tree-or-in-use` and `design@agent-skills@goal-carries-reading-examples`, each as `skill@knowledge-architect-decision-recording@reversal-check` asks | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | at slice 1's landing |
| every cause of the premortem, each listed with its outcome and, where dropped, the condition it fails | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` | at slice 1's landing; the cause on a consumer's pin at slice 2's |
| `acceptance@goals-at-the-root@the-goals-home-is-not-larger` | reported as `skill@knowledge-architect-planning@acceptance-criteria` says | at slice 1's landing, and when this document leaves |
| this document | nothing of its own: it leaves in the commit that deletes slice 2's spec, or the commit that records the last ruling of D1 and D2 | after slice 2 |

## Later consequences

- The consumers of the checker run the whole-set review when their pin crosses the version that
  ships slice 2: its Migration entry names that review.
- The discussion of `thread@goals-at-the-root@goal-pursuit-is-judged-by-review-and-audit` opens from
  the two issues that hold its leads, on the owner's word or on the evidence that reopens it.
- `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`: the pass it awaits
  brings each standing head that cites a goal to state its constraint beside the reference; slice
  1 writes that into the issue.
