---
name: knowledge-architect-goal-setting
description: MUST use when the owner states a new purpose or abandons one, when a recorded decision conflicts with a goal, when the owner asks to review the whole goal set or a pin move crosses a version whose Migration entry names that review, and before writing or editing any goals home. Covers what a goal is and how it differs from a decision, the standard a proper goal meets, how the owner's intent is drawn out and written as a draft the goal reviewer reads before the owner rules on it, in the conversation for three goals or fewer and in the working tree for more, the review of the whole goal set, the shape of a goal entry, where the goals sit, and why a goal leaves only on the owner's word.
---

# Setting goals

Scope: the project's goals, which sit in the goals home of the project's root. Writing a goal,
changing its wording, and removing it.

Not covered here: **a decision about how something is built**, which is
`skill@knowledge-architect-decision-recording`; **setting up a Component**,
`skill@knowledge-architect-setup`.

**The goals are the only statements assumed to come from the owner**, per `primer@goals-bind`. Every design decision binds
as a presumption, and a goal binds outright: a constraint derived from a goal rules a proposal out.
That is why an agent never edits a goal outside this skill.

## A goal, and what is not one {{slug:goal-or-not}}

**A goal is met or unmet. A decision is won or lost.**

{{fragment:goal-standard.md}}

A goal stays in its home, **met or not yet met**. One removed when it is achieved stops being
checked, and can stop being met without anyone noticing. **A goal leaves only on the owner's
word**: when the owner abandons it, or rules that it is no purpose, scope or ranking.

## The shape of a goal entry {{slug:goal-entry-shape}}

%% The body has no length bound because a session that argues from a goal applies it whole, so its
%% words cost that session nothing, while a goal pressed into few sentences packs its sets into long
%% ones and reads two ways, per `design@agent-skills@goal-entry-shape`. It holds no fixed block of
%% cases, per `design@agent-skills@a-goal-carries-an-example-only-for-its-direction`; that it holds
%% no measure is rule 7 of the standard, stated once in this skill.
A level-two heading stating the goal as a sentence, its slug at the end, then the body: what the
goal means. The body holds no list of cases. It carries an example only where the statement alone
would be read in the wrong direction; an example shows a reading and binds nothing. The body holds what the goal standard asks of it, at the length that takes: short
sentences, and a list where it names the members of a set, such as the activities a goal covers.
Every level-two heading of a goals home is a goal and carries a slug: the head that says what the
home holds is prose under the level-one title, never a level-two section.

```markdown
## <The goal, stated as a sentence> `##<slug>`

<What it means: the outcome, for whom, in the direction it states, in sentences or in a list.>
```

%% `design@agent-skills@goals-home-at-the-root-only`: a goal anchored in a Component names a part
%% whose existence comes from a design choice, so a redesign of the split would move goals only the
%% owner may change.
**Where the goals sit.** Every goal is written in the goals home of the project's root, whichever
Components serve it. No other Component and no location states a goal: the root alone carries the
goals home, `register@goal`, and the checker refuses one anywhere else. The root's home may be a
directory behind a `README.md` that links each of its files, the form `primer@checker-model` gives a
heading register.

%% The head restates no rule of this skill, since a restatement there drifted from the skill and an
%% agent that edits a goal loads the skill anyway. The definitions are shared there because most
%% goals of a home use them: the session chose a paragraph of definitions in the head over a goal
%% per definition, which no goal ranks; the owner approved it.
The head of the root's goals home, in a directory its `README.md`, states what the project is for
as a whole, which the owner rules like a goal. It also holds the definitions its goals share: a few
general words for what several goals name, each passing rule 2, in a paragraph of their own. A
definition that is the outcome of one goal is stated in that goal instead. The head restates no
rule of this skill.

## Drawing out the owner's intent {{slug:drawing-out-intent}}

%% The agent drafts so that the owner gets help with the wording, and the owner rules on every goal
%% so that the goals home holds the owner's mind, per `goal@the-owner-decides`.
%% Drafting nothing and only asking gives up the help with the wording. Drafting and letting the
%% owner correct accepts a goal by the owner's silence, which is not the owner's word; for the same
%% reason a goal with no ruling is not written. It is not dropped either, since a goal with no
%% ruling may still be the owner's intent. A batch of more than three goals goes through the tree,
%% per `design@agent-skills@goal-batch-ruled-in-the-tree`. Step 1 gains no discussion of a draft's
%% consequences before the draft: the two recorded cases of an intent the owner did not hold yet
%% were goals of a new Component, which no Component states any longer. Its three questions follow
%% the three kinds of rule 1.
1. **Ask the owner to state their intent** for the project, and any goals they already have, in
   their own words. Three questions help: what is it for, and for whom; what is it explicitly not
   for; which comes first where two of these compete.
2. **Refine the wording** of what the owner stated. **Propose further goals** from the existing
   documentation, or from the code and content when the documentation does not say enough.
3. **Write a draft that goes into the goals home verbatim if it is approved**.
   **Mark each goal with its source**: "the owner's statement", or "proposed from <the document or
   the code it was read from>". The source mark is not written to the goals home. A batch of three
   goals or fewer is drafted in the message. A batch of more is written in the goals home of the
   working tree, and its source marks go in a record kept as
   `skill@knowledge-architect-goal-setting@whole-set-review` keeps one, in
   `docs/goal-reviews/<YYYY-MM-DD>-<subject>.md`.
%% Per `design@agent-skills@goal-reviewer-agent`. The whole batch is read at once, and again on any
%% rewording, since one goal's wording can change its relation to a sibling. The stop is
%% `design@agent-skills@goal-review-stops-on-old-wording`.
4. **Dispatch `agent@knowledge-architect-goal-reviewer` in its draft mode**, fresh, never a fork:
   its brief is the whole batch of new and reworded goals, every relevant statement of the owner,
   verbatim, whether or not the draft drew on it, and the project. Repair each `settled` finding,
   keeping a goal's wording where a finding is `intent` or `contradiction`: a narrower wording is
   drafted only on the owner's word, with what the narrowing rules out. Dispatch the reviewer again
   on the repaired batch, so that the version shown is a version reviewed. Keep the version each
   round read, in the conversation for a batch of three goals or fewer and in the record for a
   larger one, and from the second round sort each finding by the words it quotes against it: on
   old wording, the words the previous round read and no repair or rewording of the owner changed
   since, or on new wording. **Stop when a round, from the second on, reports no `blocks` finding
   on old wording, whatever its route, or when the owner says so.** From the third round on, where
   a round's `blocks` findings on old wording are no fewer than the previous round's, and from the
   second round on, where every `blocks` finding on old wording is a `contradiction`, put the stop
   to the owner instead of running another round. The findings of the round that stops go to the
   owner beside the goals, unrepaired. Where the owner stops the rounds after a round's repairs,
   the owner's stop prevails: tell the owner that those repairs are unreviewed, and the goals axis
   of the review before the merge, `agent@knowledge-architect-goal-reviewer` in its merge mode,
   reads them.
%% Per `design@agent-skills@goal-draft-shows-reach`.
5. **Show the owner the draft, each goal with its boundary block**, in the message, or, for a batch
   of more than three goals, in the tree and the record: what the goal asks of future work, the
   part the project does not hold yet; for each layer the goal has, requires, admits and rules out,
   at least one case, and the cases it reaches that the owner may not intend, those it only admits
   shown for information, since they hold no review open, from the project's
   existing or planned work, or, where none exists yet, from the uses the owner's statements
   describe; each sibling goal it touches with the relation, a conflict named as such; and each
   `intent` finding of the reviewer, with the case on which the readings differ. Put each `contradiction` at the top of the draft, quoting both sides:
   a statement of the owner that conflicts with a goal or with another statement, a part of the
   statements the draft drops, or a goal ruling out what another requires. Resolve none alone; a goal
   resting on one is not written before the owner's answer. The block and the reviewer's report are
   not written to the goals home; for a batch of more than three goals they are written in the
   record.
6. **Ask the owner to read the draft in full, and to rule on it**: for three goals or fewer, on each
   goal by its slug, approved or dropped; for more, on the batch as the tree holds it, the owner
   naming each goal dropped or reworded. A goal with no ruling is neither written nor dropped: ask
   again for it by its slug. A
   goal the owner rewords is reviewed again, with the whole batch, in the next round of the same
   review, and shown again in its new wording before it is written.
%% Per `design@agent-skills@goal-batch-drafts-committed-each-round`: the goals' commit can change
%% no file where the owner rules the batch as drafted.
7. Write the approved goals, exactly as approved, and the home's head, and nothing else; for a batch
   drafted in the tree, whose drafts are committed, the commit that writes the owner's last answers
   into the record records the ruling, since it always changes a file, in the order the closing
   paragraph of `skill@knowledge-architect-goal-setting@whole-set-review` gives.

%% Per `design@agent-skills@goal-edit-ruled-alone`.
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

%% Per `design@agent-skills@goal-set-review-in-goal-setting`.
## Reviewing the whole goal set {{slug:whole-set-review}}

%% Per `design@agent-skills@whole-set-rounds`, for the sets, the whole-set reviewer in every round,
%% the owner's answer between rounds and the committed record; the stop is
%% `design@agent-skills@goal-review-stops-on-old-wording`.
1. **Split the goals into sets**, each a cluster of goals on one subject that touch each other.
2. **Dispatch, in one message, one `agent@knowledge-architect-goal-reviewer` per set in its draft
   mode and one in its whole-set mode**, each fresh, over the goals as the tree holds them. Each
   set's brief names its goals; every brief names the review's record, below, which the reviewer
   reads whole, and asks it to report again a question the record holds as open only where the
   goal's text changed it. A set's brief carries the owner's statements and answers through the
   record.
3. **Write the repairs in the goals home of the working tree**: each `settled` finding's, and the
   session's default for each other finding, which keeps a goal's wording on a finding of its
   reach. Put to the owner, in one message for the round, each
   `intent` and `contradiction` finding with its default and the case on which the readings differ,
   and each default that departs from words the owner approved. The next round starts once the
   owner has answered.
4. **Keep the review's record** in one file, `docs/goal-reviews/<YYYY-MM-DD>-<subject>.md` at the
   project's root: for each round, the findings counted per reviewer, rule, grade, route tag and old
   or new wording, each finding's
   outcome, each default with its ground, each question put to the owner, labelled `G<n>`, with the
   owner's answer verbatim, and each goal's boundary block, from the round's reviewers; and the list of questions
   still open. Commit it at each round, with the round's drafts.
5. **Run the next round** from step 2, over the goals and the record as step 3 left them.
6. **Stop** by the rule of step 4 of
   `skill@knowledge-architect-goal-setting@drawing-out-intent`, over the `blocks` findings of every
   reviewer of the round. The round that meets the stop writes no repair: its findings go to the
   owner beside the goals, so that the goals the owner reads are the goals a round reviewed.

%% Rule 4 turns on what the owner ruled out knowingly, which the reviewer cannot judge without the
%% owner's statements; the excess reach of a goal approved without a reading of its reach is the
%% defect the standard was written for, so every goal is shown with its block once.
**Every goal is then put to the owner with its boundary block**, a goal with no other finding
included, for a ruling: kept as it stands, or as the rounds reworded it, as
`skill@knowledge-architect-goal-setting@drawing-out-intent` says in its steps 6 and 7, the blocks of
the round that met the stop, in the record, and the goals in the tree, since only the owner judges
rule 4. The record is committed with the owner's last answers before the goals the owner ruled last
are written; the commit that writes them, or a later commit of the same branch, deletes the record
and names it by its path.
