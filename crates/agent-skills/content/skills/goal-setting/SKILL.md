---
name: knowledge-architect-goal-setting
description: MUST use when a Component has no goal, when the owner states a new purpose or abandons one, when a recorded decision conflicts with a goal, when the owner asks to review the whole goal set or a pin move crosses a version whose Migration entry names that review, and before writing or editing any goals home. Covers what a goal is and how it differs from a decision, the standard a proper goal meets, how the owner's intent is drawn out and written as a draft the goal reviewer reads before the owner rules on it goal by goal, the review of the whole goal set, the shape of a goal entry, how a Component's goal names the project goal it refines, where each goal sits, and why a goal leaves only on the owner's word.
---

# Setting goals

Scope: the goals homes of the project, one per Component, `register@*@goal`. Writing
a goal, changing its wording, and removing it.

Not covered here: **a decision about how something is built**, which is
`skill@knowledge-architect-decision-recording`; **setting up the rest of a Component**,
`skill@knowledge-architect-setup`.

**The goals are the only statements assumed to come from the owner**, per `primer@goals-bind`. Every design decision binds
as a presumption, and a goal binds outright: a constraint derived from a goal rules a proposal out.
That is why an agent never edits a goal outside this skill.

## A goal, and what is not one {{slug:goal-or-not}}

**A goal is met or unmet. A decision is won or lost.**

{{fragment:goal-standard.md}}

A goal stays in its home **while it is met**. One removed when it is achieved stops being checked,
and can stop being met without anyone noticing. **A goal leaves only when the owner abandons it**,
on the owner's word.

## The shape of a goal entry {{slug:goal-entry-shape}}

A level-two heading stating the goal as a sentence, its slug at the end, then one short paragraph:
what the goal means, and **what would show it is met**. The paragraph stays short because a goals
home is read whenever a decision is argued from one. Every level-two heading of a goals home is a
goal and carries a slug: the head that says what the home holds is prose under the level-one title,
never a level-two section.

```markdown
## <The goal, stated as a sentence> `##<slug>`

<What it means, in two to four sentences, and what would show it is met.>
```

%% A goal sits in the Component responsible for fulfilling it because a reference reaches a goal of
%% any Component from anywhere, per `design@core@an-entity-belongs-to-its-anchor`, so serving a goal
%% does not require moving it. The rival, each goal in the smallest scope serving it, promotes goals
%% into the root documents, and leaves a goal with no Component responsible for it, against
%% `goal@knowledge-architect@the-owner-decides`. A published Component serves the consumers whose
%% provision the root's goals state, so its goals refine a root goal; this is an encouragement, not a
%% rule.
**Where a goal goes.** A goal is written in the goals home of the Component whose responsibility it
is to fulfil it, even when other Components' decisions serve it too: any goal can be referenced from
anywhere. The root's goals state what the project provides to its consumers. A published Component
serves those consumers, so its goals are encouraged to be sub-goals: more specific than a root
goal, perhaps not stated by it, and still to be fulfilled. A Component that serves only the
project, such as a maintenance tool, serves every root goal at once, and its goals need not refine
one. Every Component
states at least one goal.

%% The reference makes rewording or abandoning the root goal list every Component goal derived from
%% it, per `design@agent-skills@a-reference-claims-a-revisit`.
A Component's goal that refines a goal of the project's root names it, with a reference in its
body: `goal@<root anchor>@<slug>`. Then `{{command}} show goal@<root anchor>@<slug>` lists every
Component goal that refines it, and removing the root goal leaves each of those references dangling,
which the check reports.

The head of the goals home says what it holds: a goal is met or unmet, it stays while it is met,
and it leaves only on the owner's word.

## Drawing out the owner's intent {{slug:drawing-out-intent}}

%% The agent drafts so that the owner gets help with the wording, and the owner rules on every goal by
%% its slug so that the goals home holds the owner's mind, per
%% `goal@knowledge-architect@the-owner-decides`. Drafting nothing and only asking gives up the help
%% with the wording. Drafting and letting the owner correct accepts a goal by the owner's silence,
%% which is not the owner's word; for the same reason a goal with no ruling is not written. It is not
%% dropped either, since a goal with no ruling may still be the owner's intent, and dropping it
%% silently can discard it.
1. **Ask the owner to state their intent** for the Component, and any goals they already have, in
   their own words. Three questions help: what is it for, and for whom; what would show that it
   fails at that; what is it explicitly not for.
2. **Refine the wording** of what the owner stated. **Propose further goals** from the existing
   documentation, or from the code and content when the documentation does not say enough.
3. **Write a draft that goes into the goals home verbatim if it is approved**.
   **Mark each goal with its source**: "the owner's statement", or "proposed from <the document or
   the code it was read from>". The source mark is not written to the goals home.
%% The reviewer is fresh because the drafter is anchored on its own reading, and a draft's excess
%% reach went unnoticed by the drafter and the owner alike in real projects. It reads every relevant
%% statement of the owner, not only those the drafter drew on, so a statement the drafter left out
%% still reaches the check of a part dropped. The whole batch is read at once, and again on any
%% rewording, since one goal's wording can change its relation to a sibling.
4. **Dispatch `agent@knowledge-architect-goal-reviewer` in its draft mode**, fresh, never a fork:
   its brief is the whole batch of new and reworded goals, every relevant statement of the owner,
   verbatim, whether or not the draft drew on it, and the project. Repair each `settled` finding,
   and dispatch it again on the repaired batch, so that the version shown is a version reviewed.
%% The block turns a goal's reach into cases the owner judges by intent; a prose goal is easy to
%% approve without seeing what it rules out. The owner's statements stay out of the display: an
%% intent spread over several statements is carried by the distilled goal, not by a quotation.
5. **Show the owner the draft, each goal with its boundary block**: for each universal or exclusion,
   at least one case it rules out and one it admits, from the project's existing or planned work,
   or, where none exists yet, from the uses the owner's statements describe; and each sibling goal it touches with the
   relation, a conflict named as such; and each `intent` finding of the reviewer, with the case on
   which the readings differ. Put each `contradiction` at the top of the draft, quoting both sides:
   a statement of the owner that conflicts with a goal or with another statement, or a part of the
   statements the draft drops. Resolve none alone; a goal resting on one is not written before the
   owner's answer. The block and the reviewer's report are not written to the goals home.
6. **Ask the owner to read the draft in full, and to rule on each goal by its slug**: approved, or
   dropped. A goal with no ruling is neither written nor dropped: ask again for it by its slug. A
   goal the owner rewords is reviewed again, with the whole batch, and shown again in its new
   wording before it is written.
7. Write the approved goals, exactly as approved, and the home's head, and nothing else.

%% A goal reworded as a default of a plan was approved in one word with the plan's other defaults,
%% and set the goal against another; a review caught it only after the ruling.
**A goal's wording proposed in another activity**, a plan's default, a review's repair, an audit's
item or a goal a design discussion sets, is drafted here as its own item, with its boundary block,
and the owner's word on a batch of other items never covers it.

%% A goal is intent about where the project should get to, so it constrains work from the moment it is
%% written, met or not. The `todo` issue lists the gap between an unmet goal and the tree as
%% outstanding work where no plan document already schedules it.
**A goal need not be met yet.** A goal is the owner's intent about where the project should get to,
and it constrains future work and design from the moment it is written. When nothing fulfils it yet and
no plan document schedules the work that would, open a `todo` issue for that work, under
`skill@knowledge-architect-issue-tracking`, and reference the goal from it.

## When this runs again {{slug:when-it-runs-again}}

- **The owner states a new purpose, abandons one, or rewords one.** A goal is added or reworded
  through `skill@knowledge-architect-goal-setting@drawing-out-intent`. When one is removed, every reference to it dangles, and `{{command}} check` lists each
  one: each is a text that derived something from the goal, and is read again.
- **The owner asks to review the whole goal set, or a pin move crosses a version whose Migration
  entry names that review**: `skill@knowledge-architect-goal-setting@whole-set-review`.
- **A decision conflicts with a goal**, and the primer's rule, `primer@goals-bind`, sends the conflict to the owner. The
  goal prevails until the owner rules; if the ruling changes the goal, it changes through `skill@knowledge-architect-goal-setting@drawing-out-intent`.

The goals of one round of rulings are written in one commit, whose message records each ruling,
attributed to the owner, as `primer@design-heads` says. A
goal's removal and the repair of every reference it leaves dangling are one commit, so the commit
passes the check.

%% Every goal edit is the owner's, so an audit's split between edits applied in its branch and items
%% put to the owner has nothing to split, and the draft review of each finding re-checks it; this
%% follows goal-setting's methodology, a fresh reviewer and goals ruled one by one, not the audit's.
## Reviewing the whole goal set {{slug:whole-set-review}}

Run on the owner's request, and at a pin move across a version whose Migration entry names this
section, as `skill@knowledge-architect-setup@moving-the-pin` says. Dispatch
`agent@knowledge-architect-goal-reviewer` in its whole-set mode, fresh, with the project as its
brief. Each finding becomes a draft that `skill@knowledge-architect-goal-setting@drawing-out-intent`
takes, its draft-mode review included, so the owner rules on every rewording, retitling, split,
removal or addition goal by goal.
%% Rule 3 turns on what the owner ruled out knowingly, which the reviewer cannot judge without the
%% owner's statements; the excess reach of a goal approved without a reading of its reach is the
%% defect the standard was written for, so every goal is shown with its block once.
**Every goal is put to the owner with its boundary block**, a goal with no other finding included,
for a ruling: kept as it stands, or reworded through
`skill@knowledge-architect-goal-setting@drawing-out-intent`, since only the owner judges rule 3.
