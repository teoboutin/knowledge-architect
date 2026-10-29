# Agent skills design: Intent and the owner's ground

## Intent and the owner's ground

### Every item the owner rules on by name and that carries no slug gets a label, of a prefix of its own kind where the label reaches a committed document and of the common `Q` where it does not `##ruled-items-labelled`

Every item the owner is asked to rule on by name, and which carries no slug when it is put to the
owner, gets a label: a prefix of capital letters naming its kind, and a number. Numbers run from 1
within each prefix, in order of appearance, and are never reused within the record the label
reaches: the committed document, a milestone's documents counting as one; a
file; or the commit messages of one branch, counting as one. A label that reaches none is numbered
within the message that puts it to the owner. An item that gains a
slug later keeps its label beside it. A label that reaches a committed document, even one that
leaves later, such as a plan document, takes a prefix of its own kind, distinct across the
installed workflow. A label used only in the conversation reuses the common prefix `Q`. A label
numbers what was put to the owner; it never names what decided, and an answer keeps the authority
that answered it, per `design@agent-skills@answer-keeps-its-authority`. Every distinct prefix is
listed here, and an edit that adds one checks it against the list:

| prefix | items | the committed document it reaches |
| --- | --- | --- |
| `D` | a plan document's defaults awaiting the owner | its "Defaults awaiting the owner" |
| `W` | the findings on the installed workflow: a retrospective's, and a project audit's sent upstream | the workflow's file of the retrospective, whose shape an audit's upstream file takes |
| `C`, `P` | a retrospective's other findings | the retrospective's files |
| `F` | a project audit's findings put to the owner | the audit report file of the pass |
| `Q` | every label that stays in the conversation | none |

The owner asked for it, from a premortem message that mixed causes, tripwires and findings: the
owner could not tell how many items there were to rule on, nor what each was about. A ruling given
in words then matches items by guess. The owner had asked before for the questions of an entry test
on the owner's intent, `rejected@agent-skills@owner-intent-entry-test`, to be numbered, and judged the pattern recurring, to cover
everything the owner rules on by name. A label that reaches no document reuses
`Q`, on the owner's argument that searching for unique letters would in the end need more letters
than exist. The structure passes
`design@agent-skills@capabilities-not-structure`, since the owner named the lack and the label's
scope follows the record that carries it. The numbering is stated once, in `primer@labelled-items`,
since labels are given in sessions of every activity and a range stated per skill drifted: three
skills gave `T`, `AC` and `F` three different ranges. Each installed skill that asks for such
rulings restates its own prefix where it asks. A project audit's findings sent upstream reuse `W`:
its upstream file takes the shape of the retrospective's workflow file and reaches the same
receiver, which cites each finding by its id and its file's stem, so one prefix names one kind of
finding in one kind of file. An `F` item is numbered within the audit report file of its pass,
which holds the item and the owner's ruling on it, per `design@agent-skills@committed-audit-report`.

### A design home holds built intent, and a plan document holds unbuilt intent `##design-home-is-built-intent`

A design home holds the design as built and its reasons, and the code is checked against it. A plan
document holds decided design that is not built yet, and each decision moves into the design home at
the landing of the work that builds it, not when the spec is written. While the work is open, the
spec or the milestone document, on the main branch or on the work's, is the only place the
decision exists. A decision that no work implements, such as a policy, and that is not part of
any spec, counts as built intent, since nothing waits to be built, and is recorded when it is made. The rival, a design home holding intent built or not, needs a
marker on every entry to tell the two apart, and checking the code against an unbuilt entry reports
a defect in code nobody has written. Unbuilt intent has a checked home because plan documents are a
structure the checker reads, per `design@core@plan-register`. This serves
`goal@knowledge-architect@documentation-stays-consistent`.

### A head prevails over diverging code until the divergence closes, never by following the code `##head-prevails-over-diverging-code`

A head can be wrong, and while it
diverges from the code it still prevails: the divergence closes when the code meets the head or when
the head is reversed, never by following the code, and work that goes on meanwhile builds on the
head. Without that, a session that must keep working would build on whichever
side it met first, and a divergence would close silently in the code's favour, against
`goal@knowledge-architect@agents-work-without-drift`.

### A divergence between a head and the code goes to the owner only where reversing the head is a fork `##divergence-routed-by-its-authority`

`primer@intent-and-claims` routes a divergence by the authority that settles it, per
`design@agent-skills@questions-go-to-their-authority`. Where the head stands, its argument
undefeated, the code is the defect: the session fixes it or opens an issue, and the work builds on
the head. Where the session finds the head's argument defeated, the head's reversal goes to the
design skill, and to the owner where it is a fork; the part of the task that rests on the point
waits for that answer. A head that records a fork's choice reopens as
`design@agent-skills@fork-choice-in-weak-form` says.

It serves `goal@knowledge-architect@the-owner-decides`, which gives the session what the argument
from the record settles. The nearest rival, every divergence holding its part of the task until the
owner's word, sends the owner a question the record answers wherever the head stands, since a design
home is built intent the code is checked against.

### A reason recorded at the code binds as intent at its scale, below the design home `##local-intent-binds`

`primer@intent-and-claims` has two classes, and `primer@verify-before-relying` a third. A
design home is authority. A reason recorded
at the code, an inline comment saying why the code is shaped so or the message of the commit that
argued it, is intent at the scale of that code: it binds as a presumption, a change that defeats it
argues against that reason in its own message, and where it conflicts with a design home, the design
home prevails, as in any divergence, per `design@agent-skills@head-prevails-over-diverging-code`. Between
a comment and a commit message, the comment is the current statement. A claim about the code as it
stands goes stale and is verified. Before code is removed or reshaped as unneeded, its comment and
the message of the commit that introduced it are read: absence from the design home is not evidence
that code is superfluous. The decision-record reviewer reports a diff that defeats such a reason
without arguing against it.

The owner's argument: "Since the workflow is instructing that "code follow design", anything that is
not argumented in design records could end up judged "superfluous" and deleted autonomously." Fewer
heads, per `design@agent-skills@a-head-is-owed-by-an-entry-test`, leaves more intent in comments.
Without a class of its own, an inline comment reads as a claim about the code, to verify and
discard; and `skill@knowledge-architect-design@incumbent-evaluation`'s rule on an implementation coincidence applies only while an incumbent
design is evaluated. A comment is part of the code, so the head prevails over it. The order serves
`goal@knowledge-architect@agents-work-without-drift`. In a project whose comments the checker does
not read, a reference in a comment is not checked, which weakens this record there:
`issue@core@references-are-read-in-markdown-and-rust-only`.

### A design head carries the standing argument, and history carries the deliberation `##standing-argument-in-head`

A design head carries the decision and its standing argument: every premise whose failure would
reopen it, which is the goal it derives a constraint from and a decision of another Component it
depends on, as references, the measurement it rests on, with the command that takes it again, and
the fact that defeated its nearest rival. The deliberation, which is what was weighed, in which
order, what evidence was built and who ruled what, stays in the spec while the spec exists, and in
history after that. What the owner brought lands in the head by its kind, per
`design@agent-skills@heads-stand-on-argument`. A decision taken with no spec carries its deliberation in its commit
message.

`goal@knowledge-architect@design-is-recorded-with-its-arguments` requires that a later session can
tell what a change costs without deriving the argument again, so the premises a reversal must
defeat are in the head. The deliberation is not, because every session reads the design homes
before it changes code, and a head that carried its whole deliberation would make that read cost
the length of every discussion ever held. History serves the reader who needs it, because a spec leaves when its work lands, per
`design@agent-skills@plan-document-leaves-at-landing`: a deleted spec is
listed by `git log --diff-filter=D` on the plans directory, and its last state is one `git show`
away. That agents take that route is measured, not assumed: a scan of a project's session logs for
a `git show` or `git log` command naming a deleted plan document re-takes it, and a scan that finds
none in a project whose specs are deleted reopens this decision. The cost is accepted: a reader
without a clone of the repository cannot reach a deliberation.

### A design head stands on its argument, and what the owner brings lands by its kind `##heads-stand-on-argument`

A head records a decision that was proposed, argued over its costs and rivals and approved, on that
argument, with its costs and rivals in its body or in the `rejected` entries it cites. What the
owner brings lands by its kind, as the head-ground fragment's table states:

- a fact only the owner holds, a doubt included, is a premise, attributed as the owner's report and
  open to challenge;
- an intent that states an outcome is a goal, drafted under `skill@knowledge-architect-goal-setting`,
  which the head cites;
- an argument, one that weighs a cost included, is argued and recorded on its merits;
- an approval of the session's default, threshold or weighing is no ground;
- a reading of a source is the source;
- a choice the argument leaves open is first asked whether a missing goal ranks its shapes, and is
  otherwise a fork's choice, recorded per `design@agent-skills@fork-choice-in-weak-form`.

A preference about a mechanism is no case of its own: it is an argument, or a choice at a fork. A
head whose ground cites the owner's words is read by the same table.

The rule of a head on the arguments discussed, not on the ground that the owner ruled, is the
owner's proposal: the owner judged this provenance to matter a lot for agents that weigh any change
or addition to the design. Its table is the owner's argument: going through challenge and
discussion makes the decision likely to have been selected from argument, so nothing or nearly
nothing in a head stands on the owner's word, and what does is a fact the owner holds or a wider
intent, which probably belongs in a goal. It serves `goal@knowledge-architect@the-owner-decides`,
under which a choice the owner made is recorded as the owner's and a decision the owner approved
stands on its argument.

The nearest rival, the owner's words as a ground wherever the decision came from the owner, admits
an approval of a default put at a fork as the owner's choice, which
`design@agent-skills@fork-choice-in-weak-form` records as the session's on the owner's report. Its
wider form, citing every ruling of the owner, is `rejected@agent-skills@head-cites-every-owner-ruling`,
which an audit measured.

### A choice at a fork is recorded in a weak form, the owner's where the owner picked and the session's where the owner approved its default, and reopens on an argument the fork's record does not hold `##fork-choice-in-weak-form`

A fork's choice names the rivals and the costs that were weighed. It is the owner's where the owner
picked, against the session's default or with none offered, and the session's where the owner
approved the session's default. It reopens without the owner on an argument or a fact that ranks
the shapes, since the question is then no fork, through the design skill and reported under
"Decided"; an argument or a fact the fork's record already holds reopens nothing. A goal changes
only through the owner.

The weak form is the owner's argument: an intent about a mechanism is always settled by an argument
or is a fork, whose free choice is stated with no strong ruling, so that a new argument found later
can reopen it freely. The guard against reversals back and forth is the planning skill's test of a
material finding, applied to a fork's choice: two sessions could otherwise flip a choice on
arguments of equal weight, each costing a rebuild. An approval at a fork is the session's choice
on the owner's report: an approval of a default is given without analysing the proposal deeply, and
is not to be attributed to the owner long term. It serves
`goal@knowledge-architect@agents-work-without-drift`, under which the owner rules only on what the
goals and the record leave open.

The nearest rival, a fork's choice reversed by the owner alone, protects a weighing the owner made
and did not state; a weighing the owner states is an argument or a goal, which the record keeps.
The rival of its second half, an approval at a fork recorded as the owner's choice, attributes to
the owner a weighing the owner did not make.
The design this decision did not keep: `rejected@agent-skills@fork-choice-reversed-only-by-the-owner`.

### A durable record attributes a decision to the owner, and the owner's words verbatim stay in a plan document or a file that leaves `##attribution-replaces-quotation`

`primer@design-heads` states it: a design head, a commit message, a tripwire, a `rejected` entry
and an issue entry state what the owner decided and how far the decision reaches, never the owner's words verbatim.
The verbatim words stay where the deliberation is kept: a plan document's items, per
`design@agent-skills@plan-records-the-exchange`, and a committed file that leaves once its outcomes
are carried out, per `design@agent-skills@committed-audit-report`. An attribution is checked against
the question it answered, as a quotation is: by its writer, with the owner's words beside it, at
the close of the decision-recording skill,
`skill@knowledge-architect-decision-recording@before-you-finish`, and by the transcript reviewer,
the decision-record reviewer and the auditors' verifiers, and it keeps the scope and the hedges of
the words: a hedge dropped records a firmer ruling than the owner gave, which the reading of a
ruling by its direction, `design@agent-skills@transcript-reviewer-agent`, does not excuse. A
quotation already in a head is converted when a change touches the head, per
`design@agent-skills@existing-heads-on-touch`; the reference a rename sets beside a quotation does
not count, per `design@agent-skills@a-past-sentence-is-rewritten`.

It serves `goal@knowledge-architect@the-owner-decides`, under which a choice the owner made is
recorded as the owner's: the attribution carries that, and the record shows the owner's choices.
The owner judged verbatim quotations of little value most of the time, as long as the attribution is
correct, and, as a hedged view, out of place in design heads. A verbatim quotation also costs the checker's syntax: two sessions bent one to get it past
the checker, one by writing spans of reference kinds still to come as placeholders, in the
agent-configuration-entities milestone, and one by writing the owner's words naming a skill's
reference form bare, in the session that wrote `spec@plans@path-quickfixes`, which
`design@agent-skills@plain-text-is-no-repair` does not admit. The nearest rival, keeping the
quotation in a head whose ground is the owner's words, keeps that cost where the ground is most
read. The risk is that an attribution loses a scope or a hedge the words showed: in this
repository, a paraphrase of the owner in `design@agent-skills@transcript-reviewer-agent` once
misstated the owner's request, and a quotation replaced it, in the commit "[review] The last
transcript review of the small-fixes branch: no ruling misstated, three items given a durable
outcome, one paraphrase replaced by the owner's words". The transcript reviewer reads the words
against the record.

The design this decision did not keep: `rejected@agent-skills@owner-words-quoted-in-head`.

### A decision approved with no argument is argued before it is recorded `##unargued-approval-is-argued`

Before a head is written from an approval that no argument, rival or cost stands behind, such as a
default approved in a batch, the session searches for the arguments for and against it, its costs
and its rivals, and records what it finds. Where the rivals come out equal, the fork goes to the
owner as a tie, and the owner's choice is recorded as a fork's choice in its weak form, per
`design@agent-skills@fork-choice-in-weak-form`. It is
the decision-recording skill's, `skill@knowledge-architect-decision-recording@unargued-approval`,
since recording is the last point before a head exists.

The rule is the owner's proposal: a decision approved with no argument always goes through a search
for the arguments for and against it, its costs and its rivals. Without it, a batch approval
of an unargued default gives a head with neither an argument nor, under
`design@agent-skills@heads-stand-on-argument`, a ground. The nearest rival, recording the
approval as it stands, is that head. The cost is one search per unargued default.

### A fork closes on the owner's word, and a thread with one shape standing closes on its argument `##owner-rules-at-a-fork`

`skill@knowledge-architect-design@threads-and-states` closes a thread in the state `argued` when
one shape stands, each rival out by a named fact or binding constraint, and the session closes the
rivals `ruled-out`, naming the fact. Both show in the next delta and in the checkpoint, where the
owner contests them with a goal, a fact or an argument, and the turn lists them under "Decided", per
`design@agent-skills@decisions-reported-to-the-owner`. A fork, two or more shapes that meet every
binding constraint with no fact defeating either, closes on the owner's word, as every question
`design@agent-skills@questions-go-to-their-authority` reserves to the owner does. A cost alone makes no fork.
The discussion stays an exchange: the owner proposes designs, the agent fills and argues them, and
either side's shape can win. A reopening of a closed fork decides nothing: a new pick at the same
fork needs a goal, a fact or an argument, a new weighing the owner states included, since the owner
changes a decision through one of those, per `design@agent-skills@intent-through-goals-not-overrule`.

It serves `goal@knowledge-architect@agents-work-without-drift`, under which the owner does not
answer what the record already answers, `goal@knowledge-architect@the-owner-decides`, which
keeps a fork with the owner, and `goal@knowledge-architect@design-with-agents-as-counterparts`, whose
exchange this is. The discussion form rests on a fact the owner reported: most of the time, the
owner's rough designs end up a better direction than the agent's proposals, the agent mostly
refining them, arguing the smaller items under them and measuring their cost, though not always,
the better design coming at times from the agent or from a fusion of both; and the owner truly
thinks that the discussion format drives the improvement and the quality of a design. The nearest
rival, every thread closing on its argument, gives up that exchange where it matters, at a fork. A
second rival, the owner closing any thread whose choice weighs a cost, closes every thread on the
owner's word, since nearly every choice has a cost or can be made to have one. The rule is the
design skill's tie for the owner at loop step 5 and its rule that no fork remains where no rival
stands, joined to closure. The
risk, a session judging a rival defeated on a weak argument, is what the checkpoint's contest
watches.

The design this decision did not keep: `rejected@agent-skills@every-thread-closed-by-argument`.

### The owner changes a decision through a goal, a fact or an argument, and gives no word that decides against an argument without one `##intent-through-goals-not-overrule`

The owner decides what `primer@question-authority` reserves to the owner, a fork among it, and
contests any closure, any default and any recorded decision with a goal, a fact or an argument,
which the session weighs like any other. No word of the owner decides against an argument without
one: "Stop" ends a discussion and decides nothing, a reopening decides nothing until a goal, a fact
or an argument backs a new pick, and a bounded default is stopped, or redirected with a reason. The
design skill's decision authority, its reopening rule and "Who moves what", the expectation set of
`skill@knowledge-architect-retrospective@expectation-sets` and its restatement in the README, the
decision-recording skill and the project-audit skill each state it, so the reason has several
sites. What stays the owner's beside it is what `primer@question-authority` lists: a goal, a fact
only the owner holds, a contradiction, a fork and an action that cannot be undone.

It serves `goal@knowledge-architect@the-owner-decides`, which states it. Its
argument is the owner's proposal: the overrule weakens the agent's challenge of the owner's
arguments, and lets the owner's preference pass as argument without being recorded as the owner's
intent; the owner judged it of little use, and probably problematic, and reported thinking
the path was never used. Asked for a reason, an overrule always yields a fact, an intent or an
argument, each of which lands by `design@agent-skills@heads-stand-on-argument`, and three cases, a
doubt the owner cannot yet state, a wish to stop, and a goal that answers one case wrongly, need
none: a doubt is a fact the discussion builds evidence on, "Stop" ends a discussion, and a goal
changes through `skill@knowledge-architect-goal-setting`. The nearest rival, the owner able to
overrule any decision for any reason, keeps that path for a case none of the three shows.

### A default that holds no fork, on bounded work or settled by the record, is built without waiting for the owner's word `##derived-decision-proceeds-on-default`

The bounded path's one message names its default, which the session builds: the owner stops it, or
redirects it with a goal, a fact or an argument, and a part that cannot be undone waits for the owner's word, per
`skill@knowledge-architect-design@bounded-work`, and the turn lists it under "Decided", per
`design@agent-skills@decisions-reported-to-the-owner`. A default whose every premise comes from
the record, a measurement or a source is the session's decision in the same way: in a plan
review's findings, written into the plan document with its ground and no label, per
`skill@knowledge-architect-planning@plan-reviews`; in a review's items, per
`skill@knowledge-architect-review@what-review-leaves`; and in an audit's rule read two ways that a
record states, per `skill@knowledge-architect-project-audit@finding-outcomes`.

It serves `goal@knowledge-architect@the-owner-decides`, under which the record settles what such a
default decides, since it holds no fork, while the owner, told of it in the bounded path's one
message or in the turn's "Decided" section, contests it at any time with a goal, a fact or an
argument. The nearest rival, the wait
for the owner's word, sends the owner the defaults that, by the owner's report recorded in
`design@agent-skills@questions-go-to-their-authority`, the recorded design heads should answer. It
is a delivery of
`design@agent-skills@questions-go-to-their-authority` where a skill writes a default, and a head
of its own because its rival, the wait, is another than that head's.

### A default put to the owner is built, and the branch that builds it merges once the owner has ruled on it `##default-ruled-before-merge`

`primer@question-authority` states it. A question the primer gives the owner, put with a default
the session judges sensible, lets the work go on with that default, as row 1 of
`primer@met-outside-the-task` and `skill@knowledge-architect-planning@working-a-slice` build it.
The owner's ruling is needed before the branch that builds the default merges, so the session
puts every default still open to the owner by the merge at the latest. A question with no default
the work can build on, a contradiction or an action that cannot be undone, holds its part instead,
per `design@agent-skills@owner-word-holds-as-far-as-its-premise` and
`goal@knowledge-architect@the-owner-s-word-is-verified`.

Such a question, with its built default and the merge waiting on the owner's ruling, is the
exchange `goal@knowledge-architect@design-with-agents-as-counterparts` asks the owner to meet a
decision the goals and the record leave open in: the owner reads the proposal, its rival and its
consequences, and can challenge it, add an argument or a fact, or change its direction before it
reaches the main branch, where a report alone, read after the merge, gives no such point.

The timing is the owner's: the work goes on with no interruption, and the owner's approval is asked
at the merge at the latest. Two rivals lost. The work waiting for each answer of the owner
interrupts the work, which the owner does not want, and makes a default no default. A branch
merging with a default unruled puts on the main branch a choice the owner has not made, against
`goal@knowledge-architect@the-owner-decides`; the check of a plan document's deletion in
`skill@knowledge-architect-planning@plan-document-leaves` catches it only when the document
leaves, which for a milestone is its last slice.
