---
name: knowledge-architect-goal-setting
description: MUST use when a Component has no goal, when the owner states a new purpose or abandons one, when a recorded decision conflicts with a goal, when the owner asks to review the whole goal set or a pin move crosses a version whose Migration entry names that review, and before writing or editing any goals home. Covers what a goal is and how it differs from a decision, the standard a proper goal meets, how the owner's intent is drawn out and written as a draft the goal reviewer reads before the owner rules on it, in the conversation for three goals or fewer and in the working tree for more, the review of the whole goal set, the shape of a goal entry, how a Component's goal names the project goal it refines, where each goal sits, and why a goal leaves only on the owner's word.
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

## A goal, and what is not one `##goal-or-not`

**A goal is met or unmet. A decision is won or lost.**

**A goal is proper when all of these hold.** The drafter writes to them, and the goal reviewer
judges by them.

1. **It states an outcome the owner intends**: what the Component is for, and for whom. A sentence
   is a mechanism when, changed, the Component would still be for what the owner intends, and an
   outcome when the change would make it for something else. So the kind of interface the Component
   provides, a command line, a library API or a check, is an outcome where the owner intends to
   provide that kind, and the drafter asks the owner where the statements do not say. Its exact
   form, a command's name, a flag or a signature, is a decision: an interface others consume earns a
   design head. The argument that a design follows from a goal goes in that design's head, which
   cites it.
2. **It has one reading.** Every term has one meaning in the project; a word the project already
   uses in another sense is defined or replaced. Every "every", "no", "all" and "not in scope" names
   the set it ranges over.
3. **It rules out nothing the owner has not ruled out knowingly.** Where it generalises the owner's
   statements, the generalisation and the cases it rules out are shown to the owner before the
   ruling.
4. **It holds one intent.** Two statements are two goals when each means something without the
   other; a qualifier, a domain or a measure belongs to the goal it qualifies. Two goals may share
   an aspect: a goal that touches what another states breaks no rule, and is a finding of rule 5
   only where the two cannot both be met.
5. **It contradicts no other goal of the project.** No state of the project meets one goal only by
   failing another. Where two outcomes compete, a goal states which ranks first.
6. **What shows it met is in what the project builds**: an observable of the tree as it stands, in
   the instructions, the checks, the code and the documents that work toward the outcome, never a
   plan document or anything that leaves. Evidence from the project's use, such as a review's
   finding, a retrospective's finding or what the owner observes in sessions, is not the measure: it
   shows the goal unmet in practice where what is built fails it, and leads to a change of what is
   built.
7. **A refinement neither widens nor contradicts the goal it refines.**
8. **A statement of what the Component is not for is a goal only when it names the design it
   excludes**, one that another goal would otherwise argue for. A property of smallness with no
   named design is no goal: it is the owner's scope control, exercised by what the owner asks for,
   or a quality argument a design head makes against its own rival.

A goal stays in its home **while it is met**. One removed when it is achieved stops being checked,
and can stop being met without anyone noticing. **A goal leaves only when the owner abandons it**,
on the owner's word.

## The shape of a goal entry `##goal-entry-shape`

A level-two heading stating the goal as a sentence, its slug at the end, then the body: what the
goal means, and **what would show it is met**. The body holds what the goal standard asks of it, at
the length that takes: short sentences, and a list where it names the members of a set, such as the
cases of a closed list or the activities a goal covers. Every level-two heading of a goals home is a
goal and carries a slug: the head that says what the home holds is prose under the level-one title,
never a level-two section.

```markdown
## <The goal, stated as a sentence> `##<slug>`

<What it means: the outcome, for whom, and the set each of its words ranges over, in sentences or
in a list.>

It is met while <what the project builds that shows it>.
```

**Where a goal goes.** A goal is written in the goals home of the Component whose responsibility it
is to fulfil it, even when other Components' decisions serve it too: any goal can be referenced from
anywhere. The root's goals state what the project provides to its consumers. A published Component
serves those consumers, so its goals are encouraged to be sub-goals: more specific than a root
goal, perhaps not stated by it, and still to be fulfilled. A Component that serves only the
project, such as a maintenance tool, serves every root goal at once, and its goals need not refine
one. Every Component
states at least one goal.

A Component's goal that refines a goal of the project's root names it, with a reference in its
body: `goal@<root anchor>@<slug>`. Then `cargo klarch show goal@<root anchor>@<slug>` lists every
Component goal that refines it, and removing the root goal leaves each of those references dangling,
which the check reports.

The head of the goals home says what it holds: a goal is met or unmet, it stays while it is met,
and it leaves only on the owner's word.

## Drawing out the owner's intent `##drawing-out-intent`

1. **Ask the owner to state their intent** for the Component, and any goals they already have, in
   their own words. Three questions help: what is it for, and for whom; what would show that it
   fails at that; what is it explicitly not for.
2. **Refine the wording** of what the owner stated. **Propose further goals** from the existing
   documentation, or from the code and content when the documentation does not say enough.
3. **Write a draft that goes into the goals home verbatim if it is approved**.
   **Mark each goal with its source**: "the owner's statement", or "proposed from <the document or
   the code it was read from>". The source mark is not written to the goals home. A batch of three
   goals or fewer is drafted in the message. A batch of more is written in the goals homes of the
   working tree, uncommitted, and its source marks go in a record kept as
   `skill@knowledge-architect-goal-setting@whole-set-review` keeps one, in
   `docs/goal-reviews/<YYYY-MM-DD>-<subject>.md`.
4. **Dispatch `agent@knowledge-architect-goal-reviewer` in its draft mode**, fresh, never a fork:
   its brief is the whole batch of new and reworded goals, every relevant statement of the owner,
   verbatim, whether or not the draft drew on it, and the project. Repair each `settled` finding,
   and dispatch it again on the repaired batch, so that the version shown is a version reviewed.
   Count a batch's findings apart from its `minor` ones, those of rules 4 and 6, which change no
   act of an agent.
5. **Show the owner the draft, each goal with its boundary block**, in the message, or, for a batch
   of more than three goals, in the tree and the record: for each universal or exclusion,
   at least one case it rules out and one it admits, from the project's existing or planned work,
   or, where none exists yet, from the uses the owner's statements describe; and each sibling goal it touches with the
   relation, a conflict named as such; and each `intent` finding of the reviewer, with the case on
   which the readings differ. Put each `contradiction` at the top of the draft, quoting both sides:
   a statement of the owner that conflicts with a goal or with another statement, or a part of the
   statements the draft drops. Resolve none alone; a goal resting on one is not written before the
   owner's answer. The block and the reviewer's report are not written to the goals home; for a batch
   of more than three goals they are written in the record.
6. **Ask the owner to read the draft in full, and to rule on it**: for three goals or fewer, on each
   goal by its slug, approved or dropped; for more, on the batch as the tree holds it, the owner
   naming each goal dropped or reworded. A goal with no ruling is neither written nor dropped: ask
   again for it by its slug. A
   goal the owner rewords is reviewed again, with the whole batch, and shown again in its new
   wording before it is written.
7. Write the approved goals, exactly as approved, and the home's head, and nothing else; for a batch
   drafted in the tree, commit the goals as the tree holds them.

**A goal's wording proposed in another activity**, a plan's default, a review's repair, an audit's
item or a goal a design discussion sets, is drafted here as its own item, with its boundary block,
and the owner's word on a batch of other items never covers it.

**A goal need not be met yet.** A goal is the owner's intent about where the project should get to,
and it constrains future work and design from the moment it is written. When nothing fulfils it yet and
no plan document schedules the work that would, open a `todo` issue for that work, under
`skill@knowledge-architect-issue-tracking`, and reference the goal from it.

## When this runs again `##when-it-runs-again`

- **The owner states a new purpose, abandons one, or rewords one.** A goal is added or reworded
  through `skill@knowledge-architect-goal-setting@drawing-out-intent`. When one is removed, every reference to it dangles, and `cargo klarch check` lists each
  one: each is a text that derived something from the goal, and is read again.
- **The owner asks to review the whole goal set, or a pin move crosses a version whose Migration
  entry names that review**: `skill@knowledge-architect-goal-setting@whole-set-review`.
- **A decision conflicts with a goal**, and the primer's rule, `primer@goals-bind`, sends the conflict to the owner. The
  goal prevails until the owner rules; if the ruling changes the goal, it changes through `skill@knowledge-architect-goal-setting@drawing-out-intent`.

The goals of one round of rulings are written in one commit, whose message records each ruling,
attributed to the owner, as `primer@design-heads` says. A
goal's removal and the repair of every reference it leaves dangling are one commit, so the commit
passes the check.

## Reviewing the whole goal set `##whole-set-review`

Run on the owner's request, and at a pin move across a version whose Migration entry names this
section, as `skill@knowledge-architect-setup@moving-the-pin` says. The review runs in rounds, and
the owner answers between two rounds.

1. **Split the goals into sets**, each a cluster of goals on one subject that touch each other.
2. **Dispatch, in one message, one `agent@knowledge-architect-goal-reviewer` per set in its draft
   mode and one in its whole-set mode**, each fresh, over the goals as the tree holds them. Each
   set's brief names its goals; every brief names the review's record, below, which the reviewer
   reads whole, and asks it to report again a question the record holds as open only where the
   goal's text changed it.
3. **Write the repairs in the goals homes of the working tree**: each `settled` finding's, and the
   session's default for each other finding. Put to the owner, in one message for the round, each
   `intent` and `contradiction` finding with its default and the case on which the readings differ,
   and each default that departs from words the owner approved. The next round starts once the
   owner has answered.
4. **Keep the review's record** in one file, `docs/goal-reviews/<YYYY-MM-DD>-<subject>.md` at the
   project's root: for each round, the findings counted per reviewer, rule and tag, each finding's
   outcome, each default with its ground, each question put to the owner with the owner's answer
   verbatim, and each goal's boundary block; and the list of questions still open. Commit it at each
   round, apart from the drafts, which stay uncommitted until the owner rules on them.
5. **Run the next round** from step 2, over the goals and the record as step 3 left them.
6. **Stop** when the owner says so, or when a round's count falls to a fifth of the first round's:
   the findings that are not `minor`, of every reviewer of the round, counted under the same brief
   in every round.

**Every goal is then put to the owner with its boundary block**, a goal with no other finding
included, for a ruling: kept as it stands, or as the rounds reworded it, as
`skill@knowledge-architect-goal-setting@drawing-out-intent` says in its steps 6 and 7, the blocks
in the record and the goals in the tree, since only
the owner judges rule 3. The commit that writes the goals the owner ruled last, or a later commit of
the same branch, deletes the record and names it by its path.
