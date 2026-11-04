# Goals — knowledge-architect

knowledge-architect is the common toolchain of one kind of project, the target project that
`goal@knowledge-architect@serves-one-kind-of-project` defines. It keeps the quality of a target
project's documentation, with a focus on its record: what was decided, and the intent and the
arguments behind it. It gives AI agents a workflow that covers each activity of a target project
whose generic part knowledge-architect's owner has taken in, development included, and aims to raise
the quality and the efficiency of their work and of the code they write. It has two main purposes,
and the first ranks above the second. The first is to let agents work autonomously without drifting
from the design recorded in their project, per `goal@knowledge-architect@agents-work-without-drift`.
The second is to hold in one toolchain the generic part of the tools and procedures that target
projects set up or change and that knowledge-architect's owner takes in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. The documentation structure and
the workflow are designed to work together.

These words hold in these senses for the goals of this home and of every Component:

- **the checker**: a command knowledge-architect provides that checks a project's documents against the rules it holds for every project and those a project declares
  to it, its reference grammar among them, and reports each breach;
- **the workflow**, or **the installed text**: the instructions knowledge-architect installs in a
  target project for its agents, one set for each activity it covers;
- **the record** of a project is made of: its decisions with their arguments, the designs it did
  not keep and why, what is outstanding, the evidence that would flip a recorded decision, and the
  reasons recorded beside its code, its agent-facing text and in the messages of the commits that
  made them. Nothing else is part of it, its goals included;
- **a recorded item**: a goal, or an entry of a kind that keeps a home of its own, one the workflow gives or
  one a project declares, such as a decision, a design not kept, something outstanding, or a piece of
  evidence that would flip a decision. A reason recorded beside code or in a commit message is none, and neither is a plan document, or a file a session keeps until its outcomes are carried out.

## Agents work productively and autonomously without drifting from the design recorded in their project `##agents-work-without-drift`

The project as a whole, the workflow and the checker, lets AI agents carry a target project's work
with less supervision. What they build stays within the design recorded in that project, as it
stands until a change reverses part of it through the workflow's procedure:

- its goals;
- the decisions its record holds, with their arguments;
- the reasons recorded beside its code and its agent-facing text.

What they build brings back none of the designs its record did not keep, unless a change reopens
one through the workflow's procedure.

Where two goals compete, it ranks first among knowledge-architect's goals, every Component's
included. A goal that refines it takes no rank from it. It never ranks over:

- a goal's exclusion that names the design it excludes;
- the owner's authority, exchange and attention that `goal@knowledge-architect@the-owner-decides`,
  `goal@knowledge-architect@design-with-agents-as-counterparts` and
  `goal@knowledge-architect@owner-attention-on-what-is-theirs` give, as the goals that refine them
  state it;
- the check of the owner's word that `goal@knowledge-architect@the-owner-s-word-is-verified` asks;
- the four conditions of a target project, and what
  `goal@knowledge-architect@serves-one-kind-of-project` states and lists among its asks; an ask that
  a design decision adds, or that a quality tool taken in brings, ranks as the decision that states
  it.

It is met while the workflow is built, and learns from its retrospectives, so that, in a target project, a departure
from that recorded design, or a design not kept brought back without that procedure, is caught
before it merges. The parts of that project's goals and of its record that bear on the work are read against the
work by a step the installed text owes an agent or a reviewer, which the checker supports as
`goal@knowledge-architect@documentation-stays-consistent` says. A
departure, or a design not kept brought back, that reaches a project's main branch shows the goal
unmet in practice.

- Requires, for instance: a change that contradicts a recorded decision is caught before it merges,
  by a review the installed text owes.
- Admits, for instance: a change that reverses a recorded decision through the workflow's procedure
  for reversing one.
- Rules out, for instance: a goal of a Component that the list above does not cover, ranked above
  this one where the two compete.

## The documentation records the design, with the intent and the arguments behind it `##design-is-recorded-with-its-arguments`

In a project that adopts the workflow:

- each decision whose reason one comment at one site cannot carry is recorded in an entry of its
  own, with its intent and its argument, and with the alternatives that lost and why they lost;
- any other decision whose reason the code or the agent-facing text it shapes does not show has its
  reason beside that code or that text, and in the message of the commit that made it;
- what is outstanding is recorded;
- the evidence that would flip a recorded decision is recorded.

A later session can then tell what it may change, and what a change costs, without deriving the
argument again or reversing a decision blind.

It is met while the installed text requires each of these to be recorded in the home the installed
text gives it:

- each decision whose reason one comment at one site cannot carry, in an entry of its own, carrying
  its standing argument, naming its rivals in its body or in the entries of the designs not kept
  that it cites;
- for any other decision whose reason the code or the agent-facing text it shapes does not show, a
  reason beside that code or that text, and in its commit message;
- what is outstanding;
- that evidence.

- Requires, for instance: a recorded decision names the alternative that lost and why it lost.
- Admits, for instance: a decision and the designs it did not keep recorded in separate documents.
- Rules out, for instance: a decision recorded with its conclusion alone, its argument left to be
  derived again.

## AI agents get a workflow that covers each activity of a target project whose generic part knowledge-architect's owner has taken in, and raises the quality and the efficiency of their work `##agents-get-a-complete-workflow`

A generic activity is one whose procedure has a part that does not depend on what the project
builds; that part is its generic part. The workflow covers the generic part of each activity that
knowledge-architect's owner has taken in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. An activity that only some
target projects carry out, such as releasing, is covered for those that carry it out or are meant
to. The activities include these, a list that grows with the workflow:

- setting up a project;
- setting its goals;
- discussing designs;
- planning;
- recording decisions;
- tracking open issues;
- developing: where to ground before an edit, what to be careful of in a change, how to test
  properly: what to test, and so that a test discriminates, and when code is reviewed during
  development, on which axes, and how it is repaired;
- working on a branch under a linear history: rebasing it, rewording a commit's message, folding a
  repair into an earlier commit, with no content lost, and merging it once every condition
  `goal@knowledge-architect@serves-one-kind-of-project` sets for a merge holds;
- releasing: its generic steps and guidelines;
- reviewing: the procedure by which a review is dispatched;
- auditing a project on one aspect;
- the retrospective.

Each part of an item of the list above, after its colon, names a step that the skill of the item's
activity holds.

No installed instruction teaches how to write Rust idiomatically, such as its error types or its
lifetimes, whatever the test of a generic part finds; a mechanical gate on it, such as a set of
lints, may ship once taken in. The default shape of the code, which
`goal@knowledge-architect@the-workflow-leads-to-better-code` asks the installed text to give, and
the default criteria for placing work in Components and for splitting them, which
`goal@knowledge-architect@projects-are-split-into-components` asks it to give, are covered.

What is particular to one project stays in that project, beside the installed text:

- the code of the project's domain;
- the project's versioning and publishing policy;
- whether the project runs continuous integration and pull requests, and how. The installed text
  requires neither, and no installed instruction outside an offered shape assumes either. An offered
  shape is a part of the installed text, or a file the workflow's setup writes, that a project takes
  or leaves as a whole; one for either may ship, per
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. In a private project with one
  contributor, continuous integration on pull requests spends limited minutes, and a pull request
  brings little value.

Each installed instruction says what it requires plainly enough to follow without guessing what it
means. No two instructions are ones a session cannot both obey. What no instruction covers is left
to the agent's judgement on purpose.

It is met while the installed text holds:

- a skill for each activity whose generic part knowledge-architect's owner has taken in, and in
  it each step the list above names for that activity;
- each instruction with one reading;
- no two instructions a session cannot both obey;
- no instruction outside an offered shape that requires or assumes continuous integration or a pull
  request.

A review round, an argument derived again, or a decision reversed by accident, that the workflow
would have spared, shows the goal unmet in practice.

- Requires, for instance: the skill for developing says how to make a test discriminate.
- Admits, for instance: the skill for releasing installed in a project that never releases.
- Rules out, for instance: an installed instruction that teaches when a Rust function takes a
  lifetime parameter.
- Rules out, for instance: an installed instruction outside an offered shape that has the session
  open a pull request.

## A mechanical task the workflow requires of a session is done by a command or a library function of a published crate of knowledge-architect `##mechanical-tasks-are-the-tool-s`

Where the installed text requires of a session a task whose result follows, with no judgement, from
the tree, the repository's history, the workflow's rules and what the session gives it, and that
takes several steps or a script the session would write, a published crate of knowledge-architect
offers a command that does it, or a library function that the installed text has the project's own
tooling call, so every session gets the same result without building it again.
Rewriting a branch's history holds such tasks: rebasing it onto a moved main branch where no
conflict arises, with the check that every commit carried over; applying a message the session wrote
to a commit that is not the newest; and folding a repair into the commit it repairs, with the check
that no content was lost. Those three count as taken in; any other such task is owed such a command
or such a library function once knowledge-architect's owner takes its generic part in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. Taking a part in is an act of
knowledge-architect's owner of its own: a task is not taken in because the installed text requires
it. When such a command or function is built follows the order of known work, which is
knowledge-architect's owner's, per `goal@knowledge-architect@the-owner-decides`.

It is met while each such task the installed text requires of a session, and whose generic part
knowledge-architect's owner has taken in, has such a command, or such a function that the installed
text has the project's own tooling call, of a published crate of knowledge-architect. A task taken
in and found in real use with none shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: a command that folds a repair into the commit it repairs and checks that
  no content was lost.
- Admits, for instance: one task offered both as a command and as a library function.
- Rules out, for instance: installed text that has each session write its own script to reword a
  commit that is not the newest.

## The documentation structure and the workflow work together `##structure-and-workflow-work-together`

The workflow writes into the structure, and the structure gives the workflow what it needs:

- one place, the home the installed text gives, to find each decision the workflow records in an
  entry of its own, with its argument;
- one such home each, in the part of the project it belongs to, for what is outstanding and for the
  evidence that would flip a recorded decision.

Neither the structure nor the workflow is designed without the other.

It is met while each kind of entry the installed text requires a session to write has a home the
installed text gives it, and each such home whose entries any document of the project cites is declared to the checker.

- Requires, for instance: a home of decisions that a document cites is declared to the checker.
- Admits, for instance: a project that keeps a kind of entry of its own beside those the workflow
  writes.
- Rules out, for instance: a kind of entry the installed text has a session write with no home the
  installed text gives it.

## The project's record lives in its repository, beside the code `##records-live-beside-the-code`

In a project that adopts the workflow, its whole record lives in its repository beside the code: its
decisions with their arguments, the designs it did not keep, what it has outstanding, the evidence
that would flip a decision, and the reasons recorded beside its code and its agent-facing text.

The record lives in the homes the installed text gives each kind of it, and in those the project
gives its own kinds of entry. They are files an agent reads and edits like any other, and the
history of the repository's commits, rather than entries in an external tracker it would need a
separate tool to reach.

What a retrospective finds is no record of the project where it ran: what it finds about the
workflow, which a project sends to knowledge-architect's owner, and what it finds about the
project's own instructions. The analysis of those findings, made in the project that acts on them,
is a document of that project.

It is met while every home that the installed text gives a kind of the project's record is a file
or a directory of the repository, or its commit history, and the installed text has a session give
each of the project's own kinds of entry a home in the repository.

- Requires, for instance: a defect the project has outstanding is recorded in a file of the
  repository.
- Admits, for instance: a pull request's discussion kept on the hosting service, with the decisions
  it reached recorded in the repository.
- Rules out, for instance: a decision of the project recorded only in an external tracker.

## The documentation stays consistent with the code and with itself `##documentation-stays-consistent`

In a project that adopts the workflow, its reviews and its audits keep the documentation consistent
with the code and with itself: they judge whether the code departs from what the documents state, and
whether two documents agree in what they mean. The checker and the commands that list the recorded items, as knowledge-architect ships them, support them,
and judge no meaning, whatever a project's own extension may do:

- the check guarantees that, in the texts the checker reads, nothing cited is missing;
- the check guarantees that, in the texts the checker reads, every rule the checker holds, its own
  and those the project declares to it, is met, save those the project switches off by a
  declaration;
- those commands list and display the entries a review must read.

A mechanical rule, such as that a flag a document names is one the binary declares, judges none.

It is met while:

- the installed text owes, before every merge into the main branch, a review that reads the
  documents the branch touches, and those that describe the code it touches, against the code and
  against each other;
- the installed text offers an audit that reads the whole project on one aspect;
- the installed text asks in its retrospective how the reviews and the audits fared;
- the check reports every reference that resolves to nothing in the texts the checker reads, and
  every breach of such a rule;
- those commands list and display the entries a review must read.

Development driven by agents changes code faster than review alone can follow, so the reviews need
these tools to keep up.

- Requires, for instance: the check reports a reference to an entry that no document defines.
- Admits, for instance: a rule the project declares to the checker for its own documents.
- Rules out, for instance: a branch merged into the main branch with no review that read the
  documents it touches against the code.

## The owner decides what the argument from the goals, the record, the code and the sources leaves open, every action that cannot be undone, and, as knowledge-architect's owner, each change of an ask and the taking in of a generic part, and is informed of every decision the agents make that earns durable text or that the owner could have ruled `##the-owner-decides`

In a project that follows the workflow, this one included, its owner states the project's intent in
its goals and brings the facts only the owner holds.

Agents decide, by argument, what these settle: the goals, the record, the code and its measurements,
and the sources the project follows, save the items the list below sends whatever the argument
settles. They report each such decision that earns durable text, as below.

The workflow sends to the owner, for a ruling or an approval, these and nothing else:

- a choice between shapes that all survive the argument, where choosing weighs a cost or a scope
  that no goal or decision ranks, such as the order of known work;
- a contradiction that no source or argument resolves;
- a goal to set or change;
- a fact only the owner holds;
- whatever the argument settles, an action that cannot be undone, such as a publication;
- to knowledge-architect's owner, whatever the argument settles, each addition, tightening or
  removal of an ask of knowledge-architect that binds every target project, per
  `goal@knowledge-architect@serves-one-kind-of-project`;
- to knowledge-architect's owner, whatever the argument settles, whether knowledge-architect takes
  in a generic part, and when, per
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

A project may reserve further actions to its owner. A proposal on what the argument settles, which
the agent puts before the owner for challenge inside an exchange, as
`goal@knowledge-architect@design-with-agents-as-counterparts` defines one, is not sent for a ruling.

The owner changes a decision the argument settled through a goal, a fact or an argument, which the
agents weigh like any other, and never by a word that gives none. A choice at a fork is the owner's
to make by a bare pick, and so is an item the list above sends whatever the argument settles: the
project's owner's for an action that cannot be undone, knowledge-architect's owner's for a
change of an ask and for taking in a generic part.

Each decision the agents make that earns durable text, such as a recorded decision, the record of a
design not kept, an outstanding item, the evidence that would flip a decision, a commit message or
a comment, is reported where the owner reads it, and the durable text that records it is that
report. So is each choice the agents make that the owner could have ruled, other than an
implementation choice that a commit message records. The owner can open the exchange on it. The two
halves are one intent, since agents decide what the argument settles only while the owner can see
each such decision.

It is met while the installed text:

- sends each question to the authority that answers it;
- sends to the owner, for a ruling or an approval, these and no other;
- has the session report, where the owner reads it, each decision that earns durable text, the
  text that records it being its report, and each choice the owner could have ruled, other than an
  implementation choice that a commit message records.

A question put to the owner for a ruling that the goals, the record, the code and its measurements,
or the sources answered, other than an item the list sends whatever the argument settles, or such a
decision or such a choice reported nowhere, shows the goal unmet in practice.

- Requires, for instance: a choice of which known work comes first goes to the owner.
- Admits, for instance: a project that reserves the merge of a branch to its owner.
- Admits, for instance: a decision reported in the record, in a commit message, or at the end of
  the turn that made it.
- Rules out, for instance: a question put to the owner for a ruling that a measurement answers.

## Agents verify the owner's word before acting on it, and build nothing on a contradiction `##the-owner-s-word-is-verified`

An agent following the workflow takes no word of the owner as granted. Before acting on a word other
than a goal, the agent checks its premises against the goals, the record, the code and its
measurements, and the sources the project follows.

A goal's premises are those its text states and those the commit messages that record a ruling on
its current text state; a rewrap or a re-pointed reference records none. They are checked when the
goal is written or reworded, and again when work meets evidence against one.

A contradiction the agent finds that no source or argument resolves goes to the owner. The part of
the work that rests on it waits for the owner's answer, and the rest goes on. A goal whose premise
proved false still binds the rest.

It is met while the installed text:

- requires that check before an act on a word of the owner, in every activity of the workflow;
- requires a goal's premises to be checked when the goal is written or reworded, and when work meets
  evidence against one;
- holds the work that rests on a contradiction.

Work built on a word whose premise a check would have shown false shows the goal unmet in practice.

- Requires, for instance: an agent asked to build on a premise that the code shows false says so
  before it builds.
- Admits, for instance: a premise checked by a measurement rather than by reading.
- Rules out, for instance: work built on the part of a word of the owner that rests on a
  contradiction no source or argument resolves.

## The owner designs with agents as counterparts `##design-with-agents-as-counterparts`

The owner and the agent both propose. Each challenges and refines the other's proposals on their
failures, their costs and their benefits, grounded in the record and backed by the measurements the
agent takes.

The exchange is open and argued. The owner meets each proposal of the exchange with its
consequences, and its rival where one stands, before it reaches the main branch. The owner can:

- challenge any proposal, the agent's own and its refinements of the owner's included;
- add arguments or facts;
- change the direction of the design, reversing, with a goal, a fact or an argument, what a goal,
  the record or the argument settled.

A pick between shapes that all survive the argument is no change of direction. A design discussion,
a goal-setting draft and a question put with a built default that the owner rules on before the
merge are such exchanges; a report alone is not.

It is met while the installed text:

- has the agent ground and challenge every proposal of the owner that decides something, before it
  is built;
- puts before the owner, in such an exchange and with its consequences and its rival where one
  stands, the agent's own proposals on a choice or a goal that the goals, the record, the code and
  its measurements, and the sources leave open, and every proposal it makes inside an exchange the
  owner is already in;
- brings every such choice and goal to the owner before it reaches the main branch.

- Requires, for instance: the agent challenges a design the owner proposes, on its costs, before it
  builds it.
- Admits, for instance: the owner picks between two shapes that both survive the argument, with no
  argument given.
- Rules out, for instance: a choice the argument leaves open that reaches the main branch before the
  owner has met it.

## The owner's attention is spent on what is the owner's to read or answer `##owner-attention-on-what-is-theirs`

The agent carries the investigation, the measurements and the tracking of what is outstanding, so
that the owner's attention goes to what is the owner's to read or answer:

- the proposals put before the owner;
- their arguments;
- the questions only the owner answers;
- the reports of what agents decided.

It is met while the installed text gives the agent each of those tasks, and puts a proposal before
the owner with its grounding and its consequences already established.

- Requires, for instance: the agent takes the measurements a proposal rests on before it puts the
  proposal to the owner.
- Admits, for instance: the owner reads the whole transcript of a session by choice.
- Rules out, for instance: a proposal put to the owner whose consequences the owner must work out.

## knowledge-architect serves one kind of project: a Rust project whose development AI agents drive, under a linear history `##serves-one-kind-of-project`

knowledge-architect is built for its owner's own projects, open source or private. They share one
toolchain, so that one set of agent instructions and mechanical tools serves all of them, and work
on one makes each of the others more efficient. That is the motive; ownership is no condition of
what follows. A target project is a project that holds all of these:

- the code it builds and ships is Rust, built by Cargo, apart from the glue its targets need, such
  as the JavaScript and the HTML of a web target, or shaders. Files the workflow installs, the
  configuration of a continuous integration, and scripts kept beside the code are no code it ships;
- AI agents drive its development: its code, its documents, its entries and its reviews. An edit
  that a person makes now and then, the project's owner or an outside contributor, leaves it a
  target project. The part of the project's owner is what
  `goal@knowledge-architect@the-owner-decides`,
  `goal@knowledge-architect@design-with-agents-as-counterparts` and
  `goal@knowledge-architect@owner-attention-on-what-is-theirs` describe for a project that follows
  the workflow;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect, from the moment of that decision.

knowledge-architect asks more of a target project, and every target project is expected to follow
what it asks. The set of what it asks is open. An ask that binds every target project is added by a
design decision of knowledge-architect, or by taking in a quality tool, which then becomes an ask so
that every target project adopts it; either way, adding it, and tightening or removing it, goes to
knowledge-architect's owner, whatever the argument settles, per
`goal@knowledge-architect@the-owner-decides`. A quality tool, in every goal of knowledge-architect,
is a tool that judges the project's quality mechanically, such as a gate or a check; a tool that
serves efficiency, such as a command that rewrites history, becomes no ask; a procedure of the
installed workflow is none, and a shape of continuous integration or of pull requests becomes no
ask, but ships only as an offered shape. The installed text states each ask, those listed here
included. What it asks includes:

- the project uses both the workflow and the checker. A project that uses the checker alone is a
  target project that does not follow this ask;
- the project is a git repository, with its `.git` at or above the project's directory, and its
  agents run under the `claude` harness;
- it runs every gate it owes, from one command, through a library that knowledge-architect
  publishes to run a project's gates;
- that command judges whether the gates let a branch merge, with the same invocation in every
  target project.

A branch merges into the main branch only once that command, in its invocation for a merge, passes
on the branch's head, and once every other condition the goals owe before a merge holds, among
them: the review the installed text owes, the ruling of the project's owner on each choice and goal
left open that the branch builds or holds as a default, per
`goal@knowledge-architect@design-with-agents-as-counterparts`, and the answer to a contradiction the
branch's work rests on, per `goal@knowledge-architect@the-owner-s-word-is-verified`; the word of
the project's owner on each action of the branch's work that cannot be undone, such as a
publication, and on the merge where the project reserves it; and, where the branch adds, tightens
or removes an ask of knowledge-architect or takes in a generic part, the word of
knowledge-architect's owner on it. A project may reserve to its owner actions, such as a merge,
never the work the definition above gives to agents: its code, its documents, its entries and its
reviews.

How an ask is met in every target project, such as the name of the command and its invocation for a
merge, is a design decision of knowledge-architect; how one project meets the rest is its own.

A target project that does not yet follow an ask is still a target project, and brings itself into
conformance over time. knowledge-architect guarantees its working in a target project as far as the
project follows what it asks: a defect that shows only where an ask is not followed is no defect of
knowledge-architect. Nothing in knowledge-architect is built only to serve a state that does not
follow an ask, other than what helps a project into conformance: not support in the installed text
or the crates for a gate runner a project keeps beside that library, not a mode of the checker
for another harness.

The definition of a target project, and what knowledge-architect covers, its activities, its checks
and its tools, are not limited, and nothing of them is removed, on the argument that a current
target project does not need it or does otherwise. That argument still counts for the economy of
the installed text, for whether an instruction enters the installed text, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`, for the choice of
knowledge-architect's owner whether to take in a generic part, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, and for the order of known
work, which is knowledge-architect's owner's.

Every other goal of knowledge-architect, every Component's included, that speaks of a project, a
project that adopts or follows the workflow or uses the checker, an adopter, or every project,
ranges over target projects alone. A project that follows the workflow is one that has installed it,
whether or not it follows every ask yet.

Anyone may use the published crates and the installed text in another project, with no guarantee
that they work there. Nothing in knowledge-architect is built or repaired only to serve a project
that is not a target project: not a mode for a project in another language, not support for merge
commits, not an instruction for a project whose development persons drive. A defect that shows only
outside a target project is no defect of knowledge-architect, and what exists only for such a
project may be removed.

It is met while:

- the installed text states each ask, and the workflow's setup and the README of each published
  crate state the four conditions and every ask the installed text states that no check reports,
  and say that knowledge-architect guarantees nothing outside the four conditions, and nothing where
  an ask is not followed;
- the installed text has a branch merge into the main branch only once every condition this goal
  sets for a merge holds;
- nothing in the crates or the installed text is built or repaired only to serve a project that is
  not a target project;
- nothing in the crates or the installed text is built only to serve a state that does not follow
  an ask, other than what helps a project into conformance;
- no decision of knowledge-architect limits or removes the definition of a target project, or what
  knowledge-architect covers, on the argument that a current target project does not need it or does
  otherwise, save where the body above says that argument still counts: for the economy of the
  installed text, for whether an instruction enters it, for the choice whether to take in a generic
  part, and for the order of known work.

- Requires, for instance: the workflow's setup states that a target project's main branch keeps a
  linear history.
- Admits, for instance: a target project whose web target ships JavaScript glue beside its Rust
  code.
- Rules out, for instance: a mode of the checker built for a project in another language.

## Every target project can adopt knowledge-architect, and a new one is served first `##every-target-project-can-adopt-it`

Nothing particular to one target project is compiled into the published crates, the tree they are
built from aside. For every other statement of the goals, this repository is a target project that
adopts, installs and follows the workflow and uses the checker. A project declares to the checker
the structure particular to it. The installed text names no path or convention particular to one
target project, and may rely on what knowledge-architect asks of every target project, as the
installed text states it, per `goal@knowledge-architect@serves-one-kind-of-project`. Every entry is
cited in one readable grammar, the same in every project, so what an agent learns in one project
applies in the next. A project pins the version it uses, and moves to a new one when it chooses.

A new target project, which adopts the checker and the workflow from its first commit, is served
first. An existing target project can adopt them too, by bringing its documents into
conformance; what that costs it is never an argument against a check the workflow needs to work
well.

It is met while:

- the checker and the installed text hold no path or convention particular to one target project;
- the checker enforces one reference grammar in every project;
- a project's pin moves only by an explicit edit;
- the workflow's setup leads a new target project and an existing one to a passing check.

- Requires, for instance: a reference to an entry is written the same way in two target projects.
- Admits, for instance: an existing project that brings its documents into conformance over several
  branches.
- Rules out, for instance: a project's version of knowledge-architect that moves with no edit that
  names the new version.

## The generic part of a tool or a procedure that a target project sets up or changes is shipped by knowledge-architect once knowledge-architect's owner takes it in `##generic-tools-and-procedures-are-shipped`

Work in one target project sets up or changes tools and procedures that make its development more
efficient or the project better, such as a gate, a maintenance command, a kind of reviewer, an
external survey, a testing method or a publishing method. The generic part of one is the part whose
procedure does not depend on what the project builds, as
`goal@knowledge-architect@agents-get-a-complete-workflow` defines the generic part of an activity:
it serves each target project that carries out its activity, or is meant to. That definition is the
test. Whether knowledge-architect takes in a part the test finds generic, and when, is a choice of
scope that knowledge-architect's owner makes from the report of each one, as
`goal@knowledge-architect@the-owner-decides` sends such a choice to the owner. The rule of
`goal@knowledge-architect@serves-one-kind-of-project` against limits argued from what a current
target project needs does not bind that choice.

A part taken in is integrated into knowledge-architect, as installed text, as an offered shape, or
as a command or a library of a published crate of knowledge-architect, so that each target project
it serves gets it when it installs the workflow, moves its pin, or updates its dependency on that
crate. What is particular to one project stays in it, declared or configured beside what
knowledge-architect ships. So knowledge-architect is about more than keeping a project's documents:
it holds the generic part of each such tool and procedure that its owner takes in. Where the part is
a procedure that the session's judgement already carries out well enough, it ships only as the acts
that judgement does not carry out. A project's own copy of a part stays until a released version of
knowledge-architect ships the part. The project considers following that version at its own move to
it, and may keep its copy once it has considered it. A fix to a copy the project keeps is
maintenance of that copy, not a copy written after the move. In this repository, which runs the
checker it builds, a part ships when it merges into the main branch, and this repository's own
instructions then point to the part rather than state its procedure again.

It is met while:

- the installed text has each tool and procedure that a session sets up or changes, to make
  development more efficient or the project better, reported to knowledge-architect's owner, each
  report saying which part of it the session finds generic by the test, and whether
  knowledge-architect already ships it;
- each part the owner has taken in is shipped in a released version of knowledge-architect;
- the installed text has a project, at its move to a released version, consider following each part
  that version ships of which the project holds its own copy;
- no skill, agent definition, `CLAUDE.md` file or command of this repository's own states the
  procedure of a part that knowledge-architect ships, beyond a pointer to it or a restatement no
  longer than that pointer.

A copy of a part that knowledge-architect ships, written in one target project after its move to a
version that ships the part, shows the goal unmet in practice.

- Requires, for instance: a gate that one target project built and the owner took in ships in a
  released version for every target project.
- Admits, for instance: a project that keeps its own copy of a part until it moves to a released
  version that ships the part.
- Admits, for instance: a project that considers the shipped part at its own move to that version,
  and keeps its own copy.
- Admits, for instance: a fix to a copy a project kept after considering the shipped part.
- Rules out, for instance: a skill of this repository that restates the whole procedure of a part
  knowledge-architect ships.

## The workflow leads agents to write better Rust code in a target project `##the-workflow-leads-to-better-code`

In a target project, the installed rules and guidelines about code or tests aim at Rust code of
better quality, the Rust of the project's tests and of its maintenance tools included. What makes
code better is hard to define before use, and a rule or a guideline too strict to follow leads
agents to act against what it was meant for. So this goal defines no quality: the installed text
gives a default shape for the code a change writes, as a guideline, not a rule every unit a change
edits must meet, and real use corrects it.

It is met while:

- the installed text states a default shape for the code a change writes, and has a review of code
  weigh each branch's changes to Rust code against it, at the latest at the review before the branch
  merges;
- the retrospective asks whether an installed rule or guideline about code or tests proved too hard
  or too strict to follow in the session's Rust work, or was followed against the change's interest,
  so that those rules and guidelines change with real use, per
  `goal@knowledge-architect@the-workflow-improves-through-real-use`.

Code that a project's owner judges worse for following the installed rules and guidelines about
code or tests shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: a review of code weighs a branch's Rust changes against the default shape
  before the branch merges.
- Admits, for instance: a change whose code departs from the default shape, with its reason stated.
- Rules out, for instance: a fixed definition of code quality that every unit a change edits must
  meet.

## The workflow leads a target project to be split into Components, each with one purpose `##projects-are-split-into-components`

A Component is a part of a project that the project declares to the checker: a directory, or the
project's root. A Component's own files are those that no Component nested under its directory
contains. A target project whose Components that hold Rust code in their own files each serve one
purpose keeps each Component's goals and design small enough to read whole, and lets a session
ground in one design and change one part of the project at a time. The project's root, which is the
whole project being split, is outside that, whatever its own files hold; so is an intermediate
Component whose own files hold no Rust code, which groups nested Components and holds the design
and the documents several of them share but the rest of the project does not. One purpose per
Component is the direction that the default criteria pursue, not a rule a Component fails alone:
what makes a split good is learned from use, so this goal fixes no rule of splitting: the installed
text gives default criteria for placing work in Components and for splitting them, as guidelines,
and real use corrects them. A target project with one purpose and one Component needs no split.

It is met while:

- the installed text states default criteria for placing work in Components and for splitting them,
  and leads a session to apply them whenever it places new work, and wherever it finds an existing
  Component that holds two purposes;
- the retrospective asks whether an installed rule or guideline about splitting a project into
  Components proved too hard or too strict to follow, or was followed against the project's
  interest, so that those rules and guidelines change with real use, per
  `goal@knowledge-architect@the-workflow-improves-through-real-use`.

A split that a project's owner judges worse for following the installed rules and guidelines about
splitting a project into Components shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: the installed text has a session apply the default criteria when it
  places new work.
- Admits, for instance: a target project with one purpose kept as one Component.
- Rules out, for instance: a rule of splitting that a Component fails alone, whatever use shows.

## Adopting knowledge-architect is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect in a target project finds what to do in its
documentation, where they already look for it:

- the README of each published package;
- the API documentation of each library;
- the installed skills.

The documentation says what to do in plain words, with examples.

It is met while:

- each published package's README and each library's API documentation state what an adopter of that
  package does;
- the installed skills, taken together, state the whole of adopting it;
- each of them does so in plain words and with examples;
- the workflow's setup leads an adopter from the first install of the skills to a passing check.

An adopter who had to read the source or ask knowledge-architect's owner shows the goal unmet in
practice.

- Requires, for instance: the README of each published package states, with an example, what an
  adopter of it does.
- Admits, for instance: an adopter who reads the source by choice.
- Rules out, for instance: a step of adoption stated only in the source of a crate.

## The workflow improves through real use `##the-workflow-improves-through-real-use`

Real use is the test of how an agent behaves under the workflow: a session that applies it inside a
real target project that has installed it, whether or not the project follows every ask yet, as part
of work meant to merge into that project's main branch, under the supervision of a person who
contributes to the project or owns it.

No mock task and no mock project is built to apply the workflow to and observe how an agent behaves,
whether to originate an edit, to choose between shapes, to accept a piece of work, or to check that
a text runs as written.

The retrospective, and the observations of real sessions by the owner of the project they ran in,
collect what was unclear, missing or wrong. A goal is judged in use where its outcome depends on
what the project does not control, such as agents, owners, adopters and the environments of its
users. The retrospective collects the cases where the installed text failed what it is for in the
session, and knowledge-architect, taking them in, judges each against its goals judged in use. Where
`goal@knowledge-architect@the-owner-decides` gives a change in the workflow to an owner, that owner
is knowledge-architect's.

An installed text that fails a goal of knowledge-architect, every Component's included, is a
defect provable by reading.

It is met while the installed text:

- lets an instruction about how an agent behaves into the installed text only on evidence from real
  use or on a defect provable by reading;
- has no session build a mock task or a mock project to apply the workflow to and observe how an
  agent behaves;
- has the retrospective collect the evidence of real use, the cases where the installed text failed
  what it is for in the session among it, for knowledge-architect's owner;

and while knowledge-architect's own instructions for taking those cases in judge each against its
goals judged in use.

- Requires, for instance: a retrospective that records a case where an installed instruction failed
  what it is for in the session, for knowledge-architect's owner.
- Admits, for instance: an instruction added on a defect found by reading the installed text against
  a goal.
- Rules out, for instance: a mock project built to observe how an agent follows an instruction.

## Adopting knowledge-architect sets up quality tools that serve target projects `##setup-brings-quality-tools`

A target project that adopts knowledge-architect is led to set up the quality tools, as
`goal@knowledge-architect@serves-one-kind-of-project` defines them, whose generic part
knowledge-architect's owner has taken in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: each is an ask, and the
workflow's setup gives it in a recommended shape that works well with the checker. Among these
tools are three:

- one command running every gate owed before a merge;
- commit messages checked as documents;
- the gates of Rust: formatting, the linter and the tests.

Continuous integration is none of the three. The workflow's setup may offer a shape for it, and
requires none, per `goal@knowledge-architect@agents-get-a-complete-workflow`.

It is met while the workflow's setup gives each such tool in its recommended shape, the three
included, so that a target project set up by following it designs none of them itself.

- Requires, for instance: the workflow's setup gives the check of commit messages as documents in
  its recommended shape.
- Admits, for instance: a target project that adds a quality tool of its own beside those the
  workflow's setup gives.
- Rules out, for instance: a target project set up by following the workflow's setup that has to
  design its own runner of the gates.

## A recorded item reaches the work it bears on, through checked references `##records-reach-their-reader`

A recorded item keeps one home and is cited at each place where it bears on other work and a project may
write a checked reference, so a session working there meets it, and a session changing it can list what to
read again.
knowledge-architect shares this out:

- a way to give each recorded item a name, and to reference it from any text the checker reads, each reference saying what it names, with one reading;
- the checker, which reports a reference that is ambiguous, that resolves to nothing or that breaks the
  grammar, so that renaming or removing an item gives the list of what to repair;
- ways to consult the recorded items, computed from the tree, among them two listings: for any item, every
  text of the tree the checker reads that references it; and everything the project records as outstanding
  and every piece of evidence that would flip a recorded decision;
- the installed text, which requires a session to read what cites a decision before it reverses it, and what
  is outstanding and the evidence that would flip a decision before a change they bear on;
- the review the installed text owes before a merge, which judges whether each item is cited where it bears and a checked reference may stand, and
  whether the work departs from the goals, the decisions and the reasons recorded beside its code and its
  agent-facing text that bear on it, or brings back a design the record did not keep.

It is met while knowledge-architect provides each of these, the listings are never kept by hand, and the installed text requires that reading, owes those two questions of the review before a merge and
asks in its retrospective how that review fared.

- Requires, for instance: the listing for a decision names a code comment that cites it.
- Requires, for instance: a reference with two readings is reported.
- Admits, for instance: a listing filtered to one kind of text, beside the whole one.
- Rules out, for instance: a list of what is outstanding kept by hand.
- Rules out, for instance: a review that takes a passing check as showing that the work follows the
  record.

## The documents render as one linked site `##documents-render-as-a-linked-site`

knowledge-architect's documents render as one site, each reference rendered as a link to its
target, hosted with the project once it is released.

It is met while a build renders knowledge-architect's Markdown documents that the checker reads into
one site, each reference that resolves rendered as a link to its target, and while what
knowledge-architect builds publishes that site with the project, such as a step of its release that
publishes it.

- Requires, for instance: a reference to a decision, in a recorded decision's text, renders as a link
  to that decision.
- Admits, for instance: a site built by any rendering tool.
- Rules out, for instance: a resolved reference rendered as plain text.
