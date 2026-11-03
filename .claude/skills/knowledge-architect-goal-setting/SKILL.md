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

**A goal is read in three layers**: what it **requires**, what it **admits**, and what it **rules
out**. What it admits, it neither asks for nor forbids. A goal states a direction: what it asks
agents to move the project toward. It is drafted to move what the project builds, so a new or
reworded goal is expected to be unmet, and is never shaped so that the project as it stands meets
it.

1. **It states an outcome the owner intends, which keeps its meaning in the project emptied of its
   design and its code.** Read the goal as if every design head, document and line of code under it
   were removed: each word still means what it meant. A goal may define what the project provides
   and then name it, as "the project provides a command that checks its documents"; it names no
   part whose existence comes from a design choice. A Component's goal may name its own Component
   and the Components that enclose it. The kind of interface the Component provides, a command line,
   a library API or a check, is an outcome where the owner intends to provide that kind, and the
   drafter asks the owner where the statements do not say. Its exact form, a command's name, a flag
   or a signature, is a decision: an interface others consume earns a design head. The argument
   that a design follows from a goal goes in that design's head, which cites it.
2. **It has one reading.** Every term has one meaning in the project; a word the project already
   uses in another sense is defined or replaced. A goal reads two ways when two agents in the same
   situation, reading it, would move in different directions. A goal that admits many shapes in one
   direction reads one way: breadth is no defect. Every "every", "no", "all" and "not in scope" in
   what a goal requires or rules out names the set it ranges over.
3. **It requires nothing, and rules out nothing, the owner has not required or ruled out
   knowingly.** Where it generalises the owner's statements, what the generalisation requires and
   rules out is shown to the owner before the ruling. What it admits needs no ruling.
4. **It holds one intent.** Two statements are two goals when each means something without the
   other; a qualifier, a domain or a measure belongs to the goal it qualifies. Two goals may share
   an aspect: a goal that touches what another states breaks no rule, and is a finding of rule 5
   only where the two cannot both be met.
5. **It contradicts no other goal of the project.** No state of the project meets one goal only by
   failing another: a goal that rules out what another requires contradicts it. A goal that rules
   out what another only admits does not, and the exclusion holds. Where two outcomes compete,
   including two that can both be met except under a condition the project or its use makes
   possible, a goal states which ranks first.
6. **What shows it met is in what the project builds**, never a plan document or anything that
   leaves. **Ask whether the outcome depends on anything the project does not control, such as its
   users, their data or their environment.** If not, what shows it met is a check or a test over
   what the project builds or produces. If so, the outcome is judged only in use, and what shows it
   met is what the project builds to pursue it and to learn from its use whether it is met, such as
   a test of a tool under the conditions of its use, a route by which its users' reports reach the
   project, or, for instructions to agents, a review that weighs them and a retrospective that asks
   how they fared. A list of the properties the outcome is taken to consist of is no such
   observable, since it narrows the outcome to what was foreseen before any use. Evidence from use,
   such as a review's finding, a retrospective's finding or what the owner observes in sessions, is
   not the measure: it shows the goal unmet in practice where what is built fails it, and leads to a
   change of what is built.
7. **A refinement neither widens nor contradicts the goal it refines, and a goal cites only goals
   of its own Component or of one enclosing it.** A goals home, its goals and its head alike, cites
   no entry other than a goal, and no register's home other than one of its own Component or of one
   enclosing it; the checker reports any other reference there. A goal holds what it needs in its
   own words.
8. **A statement of what the Component is not for is a goal only when it names the design it
   excludes**, one that another goal would otherwise argue for, described in words that keep their
   meaning under rule 1. A property of smallness with no named design is no goal: it is the owner's
   scope control, exercised by what the owner asks for, or a quality argument a design head makes
   against its own rival.

A goal stays in its home **while it is met**. One removed when it is achieved stops being checked,
and can stop being met without anyone noticing. **A goal leaves only when the owner abandons it**,
on the owner's word.

## The shape of a goal entry `##goal-entry-shape`

A level-two heading stating the goal as a sentence, its slug at the end, then the body: what the
goal means, **what would show it is met**, and, for each layer the goal has, one or more "for
instance" cases: what it requires, what it admits, what it rules out. A layer the goal does not have
is omitted. The cases written are those of the boundary block the owner's ruling kept, each in words
that keep their meaning under rule 1. A case shows a reading and binds nothing: what the goal
requires is its sentences, not the cases. The body holds what the goal standard asks of it, at the
length that takes: short sentences, and a list where it names the members of a set, such as the
cases of a closed list or the activities a goal covers. Every level-two heading of a goals home is a
goal and carries a slug: the head that says what the home holds is prose under the level-one title,
never a level-two section.

```markdown
## <The goal, stated as a sentence> `##<slug>`

<What it means: the outcome, for whom, in the direction it states, in sentences or in a list.>

It is met while <what the project builds that shows it>.

- Requires, for instance: <a case>.
- Admits, for instance: <a case>.
- Rules out, for instance: <a case>.
```

**Where a goal goes.** A goal is written in the goals home of the Component whose responsibility it
is to fulfil it, even when other Components' decisions serve it too: any goal can be referenced from
anywhere but a goals home, which cites only goals of its own Component or of one enclosing it, as
rule 7 says. The root's goals state what the project provides to its consumers. A published Component
serves those consumers, so its goals are encouraged to be sub-goals: more specific than a root
goal, perhaps not stated by it, and still to be fulfilled. A Component that serves only the
project, such as a maintenance tool, serves every root goal at once, and its goals need not refine
one. Every Component
states at least one goal.

A Component's goal that refines a goal of the project's root names it, with a reference in its
body: `goal@<root anchor>@<slug>`. Then `cargo klarch show goal@<root anchor>@<slug>` lists every
Component goal that refines it, and removing the root goal leaves each of those references dangling,
which the check reports.

An intermediate Component, which `skill@knowledge-architect-component-creation@component-criteria`
defines, states the purpose that the Components nested under it serve together, and a goal of each
of them refines a goal of that intermediate Component, naming it, as a Component's goal names the
root goal it refines. A purpose the owner states for several Components together is the occasion to
create one, under `skill@knowledge-architect-component-creation`.

The head of the goals home says what it holds: a goal is met or unmet, it stays while it is met,
and it leaves only on the owner's word. It also states the purpose every goal of its Component
serves, which the owner rules like a goal; the root's states the project's.

## Drawing out the owner's intent `##drawing-out-intent`

1. **Ask the owner to state their intent** for the Component, and any goals they already have, in
   their own words. Three questions help: what is it for, and for whom; what would show that it
   fails at that; what is it explicitly not for.
2. **Refine the wording** of what the owner stated. **Propose further goals** from the existing
   documentation, or from the code and content when the documentation does not say enough. A
   proposed goal states where the project should get to, not what it holds today.
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
   keeping a goal's wording where a finding is `intent` or `contradiction`: a narrower wording is
   drafted only on the owner's word, with what the narrowing rules out. Dispatch the reviewer again
   on the repaired batch, so that the version shown is a version reviewed. From the second round,
   sort each finding by the words it quotes: on old wording, the words the previous round read and
   no repair changed since, or on new wording. **Stop when a round, from the second on, reports no
   `blocks` finding tagged `settled` on old wording.** The findings of the round that stops go to the
   owner beside the goals, unrepaired.
5. **Show the owner the draft, each goal with its boundary block**, in the message, or, for a batch
   of more than three goals, in the tree and the record: what the goal asks of future work, the
   part the project does not hold yet; for each layer the goal has, requires, admits and rules out,
   at least one case, and the cases it reaches that the owner may not intend, from the project's
   existing or planned work, or, where none exists yet, from the uses the owner's statements
   describe; each sibling goal it touches with the relation, a conflict named as such; and each
   `intent` finding of the reviewer, with the case on which the readings differ. Put each `contradiction` at the top of the draft, quoting both sides:
   a statement of the owner that conflicts with a goal or with another statement, or a part of the
   statements the draft drops. Resolve none alone; a goal resting on one is not written before the
   owner's answer. The block and the reviewer's report are not written to the goals home; for a
   batch of more than three goals they are written in the record.
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

1. **Split the goals into sets**, each a cluster of goals on one subject that touch each other.
2. **Dispatch, in one message, one `agent@knowledge-architect-goal-reviewer` per set in its draft
   mode and one in its whole-set mode**, each fresh, over the goals as the tree holds them. Each
   set's brief names its goals; every brief names the review's record, below, which the reviewer
   reads whole, and asks it to report again a question the record holds as open only where the
   goal's text changed it. A set's brief carries the owner's statements and answers through the
   record.
3. **Write the repairs in the goals homes of the working tree**: each `settled` finding's, and the
   session's default for each other finding, which keeps a goal's wording on a finding of its
   reach. Put to the owner, in one message for the round, each
   `intent` and `contradiction` finding with its default and the case on which the readings differ,
   and each default that departs from words the owner approved. The next round starts once the
   owner has answered.
4. **Keep the review's record** in one file, `docs/goal-reviews/<YYYY-MM-DD>-<subject>.md` at the
   project's root: for each round, the findings counted per reviewer, rule, grade, route tag and old
   or new wording, each finding's
   outcome, each default with its ground, each question put to the owner with the owner's answer
   verbatim, and each goal's boundary block; and the list of questions still open. Commit it at each
   round, apart from the drafts, which stay uncommitted until the owner rules on them.
5. **Run the next round** from step 2, over the goals and the record as step 3 left them.
6. **Stop** when the owner says so, or when a round's count, from the second round on, falls to a
   fifth of the first round's: the `blocks` and `asks` findings of every reviewer of the round,
   counted under the same brief in every round, over all wording in the first round and over old
   wording in each later one. The round that meets the stop writes no repair: its findings go to the
   owner beside the goals, so that the goals the owner reads are the goals a round reviewed.

**Every goal is then put to the owner with its boundary block**, a goal with no other finding
included, for a ruling: kept as it stands, or as the rounds reworded it, as
`skill@knowledge-architect-goal-setting@drawing-out-intent` says in its steps 6 and 7, the blocks in
the record and the goals in the tree, since only the owner judges rule 3. The commit that writes the
goals the owner ruled last, or a later commit of the same branch, deletes the record and names it by
its path.
