---
name: knowledge-architect-decision-recording
description: MUST use before writing into a design home, whatever the activity, and when a design decision has been made or reversed — a choice about how the project or one of its Components is built, including anything a consumer of it may depend on — in order to decide whether it earns durable text at all (most implementation choices do not) and where that text lands. Covers when recording happens, the reversal check that comes before everything else, which Component owns it, the split between the design home, the rejected designs and history, what earns a head, what a head holds and how it is shaped, where a head is written and how a slug is renamed, and how a premortem's causes become tripwires.
---

# Recording a decision

Scope: turning a decision about **what we build** into durable text. Whether it earns durable text
at all, when it is written, which Component owns it, what goes in the design home, what stays in
history, and which of a premortem's causes become tripwires, and when.

Not covered here: **having** the discussion (`skill@knowledge-architect-design`);
**writing the spec or the milestone document** that carries a decision until its work lands
(`skill@knowledge-architect-planning`); **parking something undecided**, and writing a tripwire's
entry (`skill@knowledge-architect-issue-tracking`). A decision about the agent configuration itself
is both this and `skill@knowledge-architect-agent-configuration`: that skill owns how to write the
configuration, this one owns where the argument lands.

## When recording happens `##when-recording-happens`

**A decision is recorded when the work that implements it lands, not when it is decided.** A
design home holds built intent: the design as built and its reasons, against which the code is
checked. A head written before the code exists would report a defect in code nobody has written.
While the work is open, the decision is unbuilt intent, and the spec or the milestone document is
the only place it exists, on the main branch or on the work's. Writing it into the design homes from there is
the **harvest**, and it happens in the change that lands the work.

A decision that no work implements, such as a policy, and that is not part of any spec, is
recorded when it is made: there is no implementing change to wait for. The second entry test of
`skill@knowledge-architect-decision-recording@what-earns-a-head` admits it, as a decision that no code and no text of the project states.

**A default awaiting the owner is built as written, per
`skill@knowledge-architect-planning@working-a-slice`, and enters a design head once the owner rules
on it.** Until then it is no approved decision, and a head is read against the owner's approval
before it is written, per `skill@knowledge-architect-decision-recording@head-shape`.

## Does it reverse something already recorded? `##reversal-check`

**Ask this first, because the answer decides everything else the recording does.** A decision routed by what it
_is_ lands in the Component that fits its subject. A decision that reverses a recorded one belongs
where the incumbent already lives, whatever the subject test would say. Get the order wrong and the
tree ends with two homes for one question, the old one still asserting what the new one denies.

**Search for the statement you are about to replace, not for a slug.** The slug you would look for
is the one you have not written yet, and the incumbent's own name is rarely the words you are
thinking in. Grep the subject's terms across every document that carries decisions (the design homes, the
`rejected` homes, the scoped `CLAUDE.md` files) and the comments of the code the subject
touches, then `git log -G'<term>'` for the argument behind them. Say in the commit message what you
searched and what it returned. A search that returned nothing is a finding worth one line there.

**If nothing holds it, carry on to the entry tests of `primer@design-heads`.** If something does, do all of the following, in the same
change:

- **The Component is the incumbent's.** The test of `skill@knowledge-architect-decision-recording@owning-component` is not asked again. If the incumbent looks misfiled, that is a separate change with its
  own argument.
- **It earns durable text whatever the entry tests of `primer@design-heads` say.** The
  incumbent's head now states something false, so the minimum is rewriting it. The entry tests
  decide whether a _new_ decision earns an entry, not whether a false one may stand.
- **Rewrite the head in place** (`skill@knowledge-architect-decision-recording@current-design`), and give the new statement its own slug when it is a new
  statement. Keep the old slug only where it still names the same decision. The slug and the
  title stay aligned with the full scope of the decision, per `skill@knowledge-architect-decision-recording@head-shape`. A rename is
  cheap, and the checker lists every reference it leaves dangling in the files it reads, per
  `primer@where-knowledge-goes`.
- **Move the incumbent into the Component's `register@*@rejected`** (`skill@knowledge-architect-decision-recording@losing-alternatives`) with its reason and a
  validity marker, stated as strongly as it was originally made, if it meets one of the tests of `skill@knowledge-architect-decision-recording@losing-alternatives`, and cite it from the new head.
- **Argue it under `skill@knowledge-architect-design` first**, if it was not: a reversal contradicts
  a statement of the incumbent, the second case of the backstop of `primer@design-heads`.
- **Delete the tripwires guarding the reversed decision.** `skill@knowledge-architect-issue-tracking` owns that movement.
- **Repair what pointed at the old behaviour**: skills, subagent definitions, scoped `CLAUDE.md`
  files, generated headers. Grep the behaviour's wording as well as the slug, since a pointer that
  describes the behaviour without naming it is the one a slug grep misses. A sentence about the
  past that named the old slug is rewritten to state the present, or removed, never retargeted and
  never turned into plain text, as `skill@knowledge-architect-issue-tracking` says of a deleted
  entry, a record of a deliberation naming it as `primer@where-knowledge-goes` says; a verbatim quotation of the owner is left as it is, with a reference to the current entry
  beside it.
- **Close the issue entry that asked the question**, in this commit rather than a later one, and
  rewrite whatever half of it survives rather than deleting the whole.

## What earns a head `##what-earns-a-head`

Most implementation choices earn no head. A unit of work produces dozens of them, and a design home that
records dozens per unit of work stops being readable and stops being ranked.

**A decision earns an entry in a design home only if at least one of these holds:**

1. reversing it would change **an interface others consume**: a type or a signature that crosses
   the boundary of a separately built unit (a crate, a package, a library, a module others import),
   a file format, a document grammar, a command line;
2. **the same reason must be respected at more than one site, or at none.** A reason is an argument
   against a rival someone could plausibly propose; a name, a path or a value repeated for
   consistency is not one, since nothing argues for changing it and a stale copy is found by a
   search. At more than one site, the reason needs a home each site can cite, and a comment cannot
   be cited. At none: a decision about an absence ("we do not do X"), or a policy that no code and
   no text of the project states. For a decision about agent-facing text, a skill, an agent or a
   `CLAUDE.md`, each text that states the instruction is one site, a design head of the project
   that restates it included, other than the decision's own: a decision stated by one text
   has one site, and its reason lives in that text, beside the instruction; or
3. **its argument turns on the behaviour of something outside the project**: an external
   specification the project implements (a standard, a protocol, a rule set), or an external tool's
   behaviour, read in its documentation or measured.

**The owner's intent earns no head of its own.** A decision that would earn a head only as the
owner's intent states an outcome, which is drafted as a goal under
`skill@knowledge-architect-goal-setting`; an argument the owner made earns a head by the tests
above, like any other.

A choice that turns on what the specification means, or on how the tool behaves, is expensive to
get wrong and expensive to derive again, and it is visible: the argument quotes the specification
or the documentation, or names the measurement.

Otherwise it belongs in an **inline comment at the code it explains, plus the commit message**.
The comment is read by every session that touches the code, which a head is only where the session
finds it on the code's subject, and the commit
carries the argument. A reason that fits in one comment at one site, about that site's own code,
fails test 2.

**A head carries the intent it serves, its shape and the standing argument, not implementation.**
How the project is arranged to reach its goals belongs here, each head citing the goal it derives a
constraint from; what the project is for is its goals home's. How a particular function does its
work belongs in a comment at that function. The test: **if changing a piece of code would force a
change to the design home, it is design; if the design home would be unaffected, it is a comment.**

**The backstop. A decision that creates a head, contradicts a statement of one, its argument
included, or adds a member the head's argument does not cover, goes to the design skill before its
text is written**, unless it was argued there, under `skill@knowledge-architect-design`. This is the case of
a decision met during another task and settled there, by the owner's word or by the session's own
choice. The design skill's in-change path keeps the deliberation in the commit message, so the task
needs no plan document and no new session. An addition that answers another question than the
head's takes a head of its own, by the test of one decision per head of
`skill@knowledge-architect-decision-recording@head-shape`. A member the head's
argument covers is recorded directly, routed as the next paragraph says; where the owner approved
it, the approval is recorded in the commit, as the session's decision, not in the head. A change that relocates or rewords
recorded decisions, a split of a head included, and adds or removes none, is not a decision: it
needs no design skill, and the routing and fidelity-of-relocation review axes judge that it adds or
removes none.

**A new member of a head is routed by the head's ground.** A member is what a head's rule admits
and its body does not yet name. Where the part of the head that admits it stands on its argument,
and the argument covers the member, it is recorded directly. Where that part is a fork's choice and
its words state the rule, the member is within the choice and is recorded directly, the choice
attributed as before. Where its words name the members and the choice was the owner's, it goes to
the owner as one proposal: one message stating the rule, the member and a default, and nothing of it is written
before the owner's word; it is no change of the decision, so it owes no reversal. A member the rule
does not admit, or that the argument does not cover, is a change of the decision, and goes to the
design skill, as the backstop says.

## Which Component owns it `##owning-component`

The project is partitioned into Components, which its manifest declares. The project's root is a
Component too.

1. Does the decision shape the layout of the project: which Components exist, and how they depend
   on each other? Then it belongs in the root Component's design home.
2. Does it shape how every Component is implemented: a design principle every Component follows?
   Then it belongs in the root Component's design home.
3. Does it shape how a single Component is implemented, or its own interface? Then it belongs in
   that Component's design home.

If it is a statement about what the project or a Component is _for_ rather than about how it is
built, it is a goal, not a decision. Goals are a register of their own, the goals home of each
Component, each entry a level-two heading carrying a slug. A decision that leans on one names it as
`goal@<component>@<slug>`.

If none of these fits, **ask the owner before writing it anywhere.**

## A decision approved with no argument `##unargued-approval`

**A decision approved with no argument is argued before it is recorded.** Before a head is written
from an approval that no argument for the decision, no rival and no cost stand behind, such as a
default approved in a batch, search for the arguments for and against it, its costs and its rivals,
and record what the search finds. When the search leaves the rivals equal, put the fork to the
owner as a tie. The owner's choice is then recorded in the weak form of a fork's choice, per
`skill@knowledge-architect-decision-recording@head-shape`.

## Three homes, split by function `##three-homes`

| what | where | why there |
| --- | --- | --- |
| the current design, and its **standing argument** | the Component's design home | it is rewritten in place, so it cannot go stale |
| a design not kept, and why | the Component's `register@*@rejected`, each entry cited by the heads whose argument covers it | a reader about to propose it must meet it without knowing to look, and a reader of the head meets its citation |
| the **deliberation**: what was weighed, in which order, what evidence was built, who ruled what | the spec or milestone document while it exists, then history; the commit message when there was no spec | failing to find it costs a re-derivation, not a wrong decision |

**The standing argument** is `primer@design-heads`, and what stays out of the head is `skill@knowledge-architect-decision-recording@head-shape`.

**The deliberation is not copied into the head**, and, where a plan document carries it, not into
the harvest's commit message either.
A plan document is deleted when its work lands, per `skill@knowledge-architect-planning`, and the
commit that deletes it cites it by its kind. A reader who needs the deliberation finds it there:

```sh
git log --diff-filter=D --name-only -- docs/plans/         # every deleted plan document, with its commit
git log -G'<slug>'                                         # every commit whose diff touches the slug
git show <commit>^:<path of the document>                  # the document as it stood before deletion
```

**`-G` matches any commit whose diff touches the slug. `-S` matches only a change in how many times
it occurs, so it misses every revision.**

When a decision was taken with no spec, on the in-change path of `skill@knowledge-architect-design`,
the deliberation is where that path puts it.

## What a head holds and how it is shaped `##head-shape`

### What a head stands on

**The standing argument** is every premise whose failure would reopen the decision: the goal it
derives a constraint from and a decision of another Component it depends on, as references; the
measurement it rests on, with the command that takes it again, one a reader can run on the corpus
it names: a script that no tree holds is replaced by such a command, or kept in the message of the
commit that took it, which the head names by its subject; the fact that defeated its nearest
rival. A constraint from a goal binds outright, where one from a decision is a presumption, and the
reference is what tells a reader which. **The test: if this premise turned false, would the
decision have to be argued again? If yes, it is in the head.** If no, it is deliberation.

**A head stands on its argument.** Most decisions are proposed, argued over their costs and rivals,
and approved by the owner. The head records such a decision on that argument, with its costs and
rivals in its body or in the `rejected` entries it cites. What the owner brings lands by its kind:

| what the owner brings | where it lands |
| --- | --- |
| a fact only the owner holds, a doubt included | a premise, attributed as the owner's report and open to challenge; it gives way to a measurement, per `primer@owner-word-premise` |
| an intent an agent cannot derive, which states an outcome | a goal, drafted under `skill@knowledge-architect-goal-setting`, which the head cites |
| an argument, one that weighs a cost included | argued, and recorded on its merits |
| an approval of the session's default, threshold or weighing, whatever its words ("approved", "agreed", "all defaults approved", "accepted the cost") | no ground: the session's decision, kept in the deliberation; at a fork, the session's choice, in the weak form below, "the session chose X over Y, which no goal ranks; the owner approved it" |
| a reading of a source | the source |
| a choice the argument leaves open, made by the owner: a pick at a fork against the session's default or with none offered, a threshold the owner sets, a local cost the owner accepts | first, ask whether a missing goal ranks the shapes; otherwise, the weak form: "the owner chose X over Y, which no goal ranks" |

A preference about a mechanism is no case of its own: it is an argument, or a choice at a fork.

**The weak form.** A fork's choice, the owner's or the session's, names the rivals and the costs
that were weighed, and reopens without the owner on an argument or a fact that ranks the shapes,
since the question is then no longer a fork. An argument or a fact that the fork's record
already holds reopens nothing: this guards a choice from reversals back and forth. A goal still
changes only through the owner.

**A head written before this rule is read by the same table**: a ground that cites the owner's pick
at a fork is a fork's choice in the weak form, an approval cited as a ground is no ground, and an
intent is a goal to draft.

**A durable record attributes a decision to the owner; it does not quote the owner.** A head, a
commit message, a tripwire, a `rejected` entry and an issue entry state what the owner decided and how far the
decision reaches, as "the owner chose X over Y" or "on the owner's word", never the owner's words
verbatim. The verbatim words stay where the deliberation is kept: a plan document's items, and a
committed file that leaves once its outcomes are carried out. An attribution keeps the scope and the
hedges of the owner's words, and is checked, like a quotation, against the question it answered.

**What the ground means to a later session.** A head that stands on its argument is reversed by a
better argument, which the owner approves where the reversal is a fork, per
`primer@question-authority`. A fork's choice reopens as the weak form says, through
`skill@knowledge-architect-design`, the reopening reported under "Decided". A premise the owner
reported gives way to a measurement, as `primer@owner-word-premise` says, and a goal changes only
through the owner.

### What else a head holds, and how it is shaped

**The deliberation is not copied into the head.** Where it is kept, and how a reader finds it,
is `skill@knowledge-architect-decision-recording@three-homes`.

A head is written **as if the design had always been so**. Present tense, no dates, no "formerly", no
account of the change. If you find yourself writing "we used to…", that sentence belongs in the
commit, and so does an opening that motivates a decision by describing the state before it.

Never list what cites a head: `cargo klarch show design@<component>@<slug>` computes it.

A decision that relies on the checker of this workflow states that it relies on the checker
working as intended. It cannot reference the checker's own decisions: a reference resolves only
against the project that holds it.

Give the decision a **slug anchor**: a short hyphenated name in backticks, prefixed with `##`. It
goes at the **end of a level-three heading**, the level the design register declares, so that the
outline reads as decisions under level-two subjects. **Every level-three heading in a design home
is an entry** and carries a slug: one without is a finding, so a heading that is section text sits
at level two or four. Nowhere else: a slug at another heading level, in a table cell, at the head
of a plain line, in the middle of a line or in a file that is not the design home defines nothing.
Outside a fenced block, `cargo klarch check` reports it as a misplaced definition; a reference to it
resolves to nothing.
The slug is an id, unique in the design home.

**When the decision was a thread of a design discussion, its slug is the thread's name**, unless
that name misdescribes the decision as approved, as a name for the change it proposed does. The
discussion minted it in the same grammar and checked it for a collision with the Component's
entries before using it, for that reason. When the name misdescribes the decision, the entry takes
a slug that names the decision, by the alignment rule below, and the text that keeps the
deliberation states the pair, `#<thread> → <entry slug>`: the plan document's harvest row, or the
commit message on the in-change path. `git log --grep` on either name then finds a commit message
that states the pair, and `git log -G` a plan document's diff that does.

**A list item is not a definition site either**, so a decision written as one bullet among several
carries no anchor and cannot be cited or found by `git log -G`. This is a constraint on the
document rather than a gap in the checker: **a decision worth a slug is worth its own level-three
heading**. When a section of bulleted arguments produces one, break it out of the list.

The statement comes first and the slug last, with no bold and no em dash between them, so that a
document outline reads as a list of decisions rather than a list of identifiers. The body follows as
ordinary prose.

**A title states a decision only while it is false of the nearest rival it beat.** A title that the
losing alternative would make true names a subject, not a decision: "The configuration is read
once, at start-up" is false of a configuration read again on every request, and "The configuration
is read with care" is true of nearly any rival.

**A title states the rule that decided, not the list of what it admits today.** When a decision
admits members (fixes, kinds, verbs, sections, consumers), or picks a mechanism to meet a
requirement, the title states the property that admits a member, in the argument's own terms, and
the body names the members built as what the rule admits today. A count or a list in a title goes
stale at the next member, and every later member then reads as a change of the decision. A set
closed on purpose keeps its list in the title, with the sentence that argues the closure in the
body. A title states no more than its argument argues.

**A head holds one decision.** Two statements are one decision when they answer one question and
lose to the same nearest rival. A rule's exception, its parameter and its delivery belong to its
head, since none means anything without the rule, unless one loses to a nearest rival of its own. Two statements that lose to different nearest
rivals, or whose arguments share no premise, are two decisions. Each takes a head of its own when
it passes an entry test, and a comment at its code when it passes none. A title joined by "and"
over two decisions shows a bundle. Rewording the title to state both does not make them one. A
member of a set the title's rule admits is no decision, so a member missing from a title is no
finding.

**A head written from a decision the owner approved is read against that approval before it is
written.** The title and the body state the position as the owner saw it, in a plan document's
thread or in the discussion of the in-change path. A clause that widens it, narrows it, or drops
part of it can still be false of the nearest rival, and is still not what was approved: it goes to
the owner, as a change of the decision.

**The slug and the title stay aligned with the full scope of the decision**: the slug is often the
only part a reader sees, in a citing document or in code, and the title is what a document outline
shows. A slug or a title that misdescribes its decision misinforms every reader, or undermines the
decision it names, so rename it even when that means rewriting every reference in the project.

**A head that a change touches is brought to the rules on design heads in that change**: its title
to the rule it argues, each decision it bundles to a head of its own, its ground to its argument. Re-pointing a
reference in a head, to a renamed entry or a migrated form, does not touch it; adding or removing
one does. Where bringing it
to the rule would widen what the owner's words in it approved, as a title moved from the members
the owner named to the rule, the change goes to the owner as one proposal, as a new member does.

## The current design `##current-design`

In the Component's design home, `register@*@design`: one file, or, for a Component whose design
has outgrown one file, a directory. In the directory shape the decision goes in the
subdocument owning its subject, never in the directory's `README.md`, which is the head and the
index. A new subdocument is linked from that index, `[title](file.md)`, the target relative to the
README, conventionally one bullet per subdocument.

A level-two subject of a design home holds the heads a piece of work reads together: one part of the
Component, such as a command, a register or an activity. The record search reads each subject's
heads together, with the `rejected` entries they cite and the issues and tripwires that cite them,
as one cluster that one agent reads whole. A subject is split into subjects read apart when:

- `cargo klarch worklist --out <dir>` names its cluster in a notice, as above the bound of the words
  one agent is given; or
- its heads answer questions about parts of the Component that change apart.

A split moves headings and decides nothing, so it is a fix in a commit of its own, as
`primer@met-outside-the-task` says. To see the notices, run `cargo klarch worklist --out <dir>` with
`<dir>` a new directory of your scratch space outside the project.

**What a head holds and how it is shaped is `skill@knowledge-architect-decision-recording@head-shape`.** The template:

```markdown
### <The decision, stated as a sentence> `##<slug>`

<The shape, present tense. Then the standing argument, per `skill@knowledge-architect-decision-recording@head-shape`.>
```

The slug is an identifier. Code comments, tripwires, other documents and `git log -G` all cite it,
so **renaming one means rewriting every reference in the same change.** Grep before you rename.

## Rejected designs `##losing-alternatives`

In the Component's `register@*@rejected`: a heading register, one level-three heading per entry,
cited `rejected@<anchor>@<id>`.

**Most designs not kept earn no entry.** An entry is read by someone reasoning from first
principles who was not there, and it is reached by grep from anywhere in the tree, so it is a
description of a rejected design that must never be read as the current one.

**Which threads of a discussion are candidates.** A thread's final state does not decide it. The
question is whether **a shape lost to an argument**.

| the thread | what happens to it |
| --- | --- |
| ruled-out | judged by the tests below |
| withdrawn with a defeating reason | the same as a `ruled-out` thread: its own proposer gave it up on an argument |
| withdrawn with no defeating reason | the deliberation only. It carries no argument a later reader could test. |
| superseded, when the absorbing thread carries its shape whole | the deliberation only |
| superseded, when a distinct shape lost | judged by the tests below, as a ruled-out thread |
| a shape rejected with nothing built in its place | judged by the tests below; the decision not to build it is a head of absence, "we do not do X", which entry test 2 of `primer@design-heads` admits, and which cites the entry |
| a shape that lost while the question stays open | the deliberation only, and the question is an issue under `skill@knowledge-architect-issue-tracking` |

**A design earns an entry only if at least one of these holds:**

1. it would change an interface others consume, as entry test 1 of `primer@design-heads` defines
   it;
2. its reason rests on a reading of an external specification the project implements;
3. it was refuted by **evidence that cost work to obtain**: a measurement, a prototype, a survey of
   other projects;
4. there is still some doubt that the design kept will achieve every goal of the project or of its
   Component.

Test 3 is the load-bearing one. Proposing something again costs one discussion round, and that is
healthy. What decides an entry is whether the refutation can be derived again inside that round.
Reasoning derives again for free, so a design refuted by an argument any competent reader
reconstructs needs no entry. Evidence does not, so a design refuted by measurement does.

Test 4 gives future design sessions the designs that were not kept, so they can start from
somewhere when the one kept proves insufficient.

A design meeting none of those stays in the deliberation, where
`skill@knowledge-architect-decision-recording@three-homes` keeps it, and nowhere else.

**Write the entry so that it cannot be read as the current design.** Name the design and the reason
it was not kept. Do not describe how it would work: an entry detailed enough to build from is the
one a grep turns into a specification.

One entry per design, keyed by **the design**, not by date. Flat, unordered, and **edited in
place**. Where it was a thread of a design discussion, its slug is the thread's name.

```markdown
### <The design not kept, named in a few words> `##<slug>`

`live`. <The reason it was not kept, checkable without leaving the entry.>
```

Each entry carries: what the design was, a **validity marker** opening its body, and the reason.
It names no winner as such, and its reason may name a head where the reason needs one: **each
design head whose argument covers the design cites the entry**, inside the sentence where
its argument meets that design, or in a closing sentence of its body. Writing the entry and its
citation is one act: `cargo klarch check` reports an entry that no design entry cites.

- `live`: the reason still holds.
- `void`: the reason no longer holds. **Mark it; never delete it.** The design may still be wrong,
  but _this_ argument no longer supports that, and the marker prevents reopening a settled question
  on a dead argument.

**A reason defeated in part**, as when a design the entry never weighed solves the problem its
reason called unsolved, is narrowed to what still holds, and the entry stays `live` for its own
design, since `void` would say its reason no longer holds while the design is still not wanted; the
head of that other design cites it, where its argument meets the entry's design.

**`live` records that the reason held against the arguments seen, not that the question is shut.**
A new argument or new evidence reopens any entry here at any time, and that is an ordinary move
rather than a transgression.
What counts as new is the record test of `skill@knowledge-architect-design@threads-and-states`.

**Every entry states a reason a reader can check without leaving the entry, or points at where the
reason is.** _"Refuted by measurement"_ satisfies neither: it asserts that a checkable reason exists
somewhere and does not say where.

**A rejected design that is reopened, chosen and implemented moves out of the home.** The home
must not describe as rejected a design the project now has.

**An entry also leaves when the owner rules that it fails every recording test**, as an audit may
propose, **or when no design head cites it any longer and no head of absence is written for it**,
as when the heads that cited it leave the design home for a line beside the instruction they state,
or when what it lost to is an instruction that no head states: the line beside that instruction then
names the design and the reason it was not kept, so a session that reopens the instruction still
meets it.

## Recording a premortem's tripwires `##ruled-tripwires`


A premortem names causes. **At the end of the work that implements the
decision**, the session judges each surviving cause by the standard below, on the wording the cause
has then: on the full path of `skill@knowledge-architect-design`, at the harvest; on its in-change
path, in the commit that records the decision. A parked thread's tripwire is judged the same way, and so is a tripwire anything else names, such
as a recurring acceptance criterion, written in the change that records its decision or in a later
one.
**It writes each cause that passes with the decision it guards**, in the tripwires home of the
Component that owns that decision, so the decision's head exists before the tripwire that names it,
and **lists every cause with its outcome**, a dropped one with the condition it fails, in its turn's
"Decided" section and in the record of the work: the harvest's commit message, or the in-change
commit message. No cause is put to the owner, who reads the work and contests any outcome with a
goal, a fact or an argument.

**A recorded tripwire's wording, scope and removal are the session's**, judged by the standard
below and listed under "Decided": it is reworded while it watches the same cause, and removed when a
condition excludes it, the commit naming the condition. A removal on the doubt's weight alone is the
owner's, since it rests on the owner's own weighing of what the owner would see. A parked thread
whose question the owner deferred records no decision: its cause is the trigger of a `deferred`
issue instead. The entry's lifecycle is `skill@knowledge-architect-issue-tracking`.

A tripwire watches one premise of a recorded decision: a failure that would show the decision
wrong and that nothing else in the project reports. Most causes a premortem names are not that,
and are not written.

**A cause becomes a tripwire only when all four conditions hold.**

1. **The failure is silent.** Every procedure that meets it counts it as success: the tests and
   gates pass, no work is stopped by it, no review reports it as a defect of the decision, and no
   request names it. A failure that fails a test or a check, stops the work that meets it, or
   arrives as a request for what the decision refused reaches the decision without a tripwire.
   Where a script could compute the firing, write that check instead. A check of the work being
   built is no tripwire: it is that work's acceptance criterion where its firing would show a
   decision wrong and it meets conditions 1, 3 and 4, and otherwise a claim of the step that builds
   it. A check that can be written only after another event,
   such as the end of a migration, is a `deferred` issue whose trigger is that event.
2. **The evidence lands where tripwires are read.** It is in the tree, the history, an audit
   report, or a retrospective file the intake receives. Evidence that exists only in a session's
   transcript, or only in what the owner sees while driving a session, reaches no occasion that
   reads the tripwires.
3. **The evidence is what the decision says must not happen.** A clause that counts what the
   decision instructs or endorses fires on the decision working.
4. **A number has a basis.** A bound in the firing clause comes from a measurement or an argument
   the decision's record carries, its head, or its plan document before the head is written, and
   exceeds the noise of the instrument that takes it. Otherwise the clause names the event,
   specific enough that two readers agree whether it occurred: "it gets slow" names none.

**The owner's word admits no tripwire alone.** A cause the owner proposes, or one derived from the
owner's words, is judged by these conditions like any other, and a tripwire that fails them is not
kept, whoever proposed it. A ruling of the owner that a tripwire restates, such as a bound or an
exception, lives in a design head, which the tripwire cites.

**A slip is no firing.** A clause on how sessions follow an instruction never fires on one instance
that a review, a check or a retrospective reported under its own procedure: one slip is the review
working. It fires on the same failure recurring in separate pieces of work, or on one instance that
reached the main branch with no procedure reporting it. Where a clause fires on both a slip and
something more, strike the slip.

**A doubt keeps the cause, briefly.** A cause that no condition clearly excludes is recorded with a
short horizon. A wrong keep costs reading until its horizon; a wrong drop loses the evidence for
good, unless the owner would see the failure anyway, by using the project or by carrying work with
its agents: then a wrong drop costs little, and a cause in doubt may be dropped. That weighs a
doubt only; it never drops a cause the four conditions admit.

The entry is a level-two heading in the tripwires home of the Component that owns the decision, its
slug last:

```markdown
## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`

**Fires when:** the event, or a count with its basis.
**Response:** reopen the decision by its reference, or open an issue of a named kind.
**Re-entry:** an occasion that holds the evidence.
**Watches until:** the occasions after which the cause, unfired, no longer will.
```

**Watches until** names the first occasions that exercise the decision, so that a command over the
history counts them from the commit that wrote the line: commits on the main branch that change a
named path or the guarded head's text, release tags, audit reports of an axis, plan documents
deleted at their landing, or one event the tree shows, such as the first issue a consumer of an
interface opens; never an occasion that only a session's transcript holds. Its length is the
writer's judgement, short for a cause kept on a doubt, and no count is kept in the entry.
`standing` is for a guarantee whose failure stays silent and permanent, with the reason in the
entry.

## Before you finish `##before-you-finish`

```sh
cargo klarch check
```

Every reference you wrote must resolve.

Then re-read each head you wrote: present tense, as if the design had always been so (`skill@knowledge-architect-decision-recording@head-shape`).

Then apply the tests of `skill@knowledge-architect-decision-recording@what-earns-a-head` again to each decision of the change, and to each
head it writes or extends, now that every text of it is written: a site the change itself wrote counts, a skill or a restatement of the project's own
included. A decision judged while its texts were still being written misses the second site the
same change adds.

Then, for each head the change edits, whatever the edit, run `cargo klarch show` on it and read
every text that cites it, and re-read the head's whole body, its argument included, against the
change; repair in the change each text it leaves false. A reference is a claim that its text is
revisited when the entry changes, and an edit that keeps the slug dangles no reference, so no check
reports a citing text it made false.

Then, for each attribution of the owner the change writes, read the owner's words beside it, from
the plan document or the transcript, and check that it keeps their scope and every hedge, as
`skill@knowledge-architect-decision-recording@head-shape` asks.

Then re-read what you wrote against the head you replaced: **rewriting argued text is where
fidelity gets lost.** If you cannot restate a rejected design as strongly as it was written, you
have not understood it well enough to move it.
