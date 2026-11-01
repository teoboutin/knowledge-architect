# Goals — knowledge-architect

knowledge-architect is the common toolchain of one kind of project, the target project that
`goal@knowledge-architect@serves-one-kind-of-project` defines. It keeps the quality of a target
project's documentation, with a focus on the record of design: what was decided, and the intent and
the arguments behind it. It gives AI agents a workflow that covers each activity of a target project
whose generic part knowledge-architect's owner has taken in, development included, and aims to raise
the quality and the efficiency of their work and of the code they write. It has two main purposes,
and the first ranks above the second. The first is to let agents work autonomously without drifting
from the design recorded in their project, per `goal@knowledge-architect@agents-work-without-drift`.
The second is to hold in one toolchain the generic part of the tools and procedures that target
projects set up or change and that knowledge-architect's owner takes in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. The documentation structure and
the workflow are designed to work together.

A goal is met or unmet. A decision about how the project is built is won or lost, and lives in the
design home. **A goal stays in this document while it is met.** A goal removed when it is achieved
stops being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word, per `design@agent-skills@goal-lifecycle`. The goals are the owner's
intent: an agent may propose wording, and the owner decides it.

## Agents work productively and autonomously without drifting from the design recorded in their project `##agents-work-without-drift`

The project as a whole, the workflow and the checker, lets AI agents carry a target project's work
with less supervision. What they build stays within the design recorded in that project, as it
stands until a change reverses part of it through the workflow's procedure:

- its goals;
- its design heads;
- the reasons recorded beside its code and its agent-facing text.

What they build brings back none of its rejected designs, unless a change reopens one through the
workflow's procedure.

Where two goals compete, it ranks first among knowledge-architect's goals, every Component's
included. A goal that refines it takes no rank from it. It never ranks over:

- a goal's exclusion that names the design it excludes;
- the owner's authority, exchange and attention that `goal@knowledge-architect@the-owner-decides`,
  `goal@knowledge-architect@design-with-agents-as-counterparts` and
  `goal@knowledge-architect@owner-attention-on-what-is-theirs` give;
- the four conditions of a target project, and what
  `goal@knowledge-architect@serves-one-kind-of-project` states and lists among its asks; an ask that
  a design decision adds, or that a quality tool taken in brings, ranks as the decision that states
  it.

The supervision it saves is measured by `goal@knowledge-architect@the-owner-decides`.

It is met while the workflow and the checker are built so that, in a target project that adopts
both, a departure from that recorded design, or a rejected design brought back without that
procedure, is caught before it merges. That project's goals, its design heads, those reasons and its
rejected designs, with the tripwires and the issues that bear on the work, are read against the work
by the checker or by a step the installed text owes an agent or a reviewer. A departure, or a
rejected design brought back, that reaches a project's main branch shows the goal unmet in practice.

## The documentation records the design, with the intent and the arguments behind it `##design-is-recorded-with-its-arguments`

In a project that adopts the workflow:

- each decision that earns a design head is recorded with its intent and its argument, and with the
  alternatives that lost and why they lost;
- a decision that earns no head has its reason in the message of the commit that made it, and beside
  the code or the agent-facing text it shapes;
- what is outstanding is recorded;
- the evidence that would flip a recorded decision is recorded.

A later session can then tell what it may change, and what a change costs, without deriving the
argument again or reversing a decision blind.

It is met while the installed text requires each of these to be recorded in the home the installed
text gives it:

- a head carrying its standing argument, naming its rivals in its body or in the rejected designs it
  cites;
- for a decision that earns no head, a reason in its commit message, and beside the code or the
  agent-facing text;
- what is outstanding, as an issue;
- that evidence, as a tripwire.

## AI agents get a workflow that covers each activity of a target project whose generic part knowledge-architect's owner has taken in, and raises the quality and the efficiency of their work `##agents-get-a-complete-workflow`

A generic activity is one whose procedure has a part that does not depend on what the project
builds; that part is its generic part. The workflow covers the generic part of each activity that
knowledge-architect's owner has taken in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. An activity that only some
target projects carry out, such as releasing, is covered for those that carry it out or are meant
to. The activities include these, among those `goal@agent-skills@one-skill-per-activity` lists, a
list that grows with the workflow:

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
- auditing a project on one aspect by the project audit's method;
- the retrospective.

Each part of an item of the list above, after its colon, names a step that the skill of the item's
activity holds. What the step says, beyond what the session's judgement handles, follows
`goal@agent-skills@only-needed-instructions`.

No installed instruction teaches how to write Rust idiomatically, such as its error types or its
lifetimes, whatever the test of a generic part finds; a mechanical gate on it, such as a set of
lints, may ship once taken in. The shape of the code and of the project's Components, which
`goal@knowledge-architect@code-has-one-responsibility-per-unit-and-is-testable` and
`goal@knowledge-architect@projects-are-split-into-components` state, is covered.

What is particular to one project stays in that project, beside the installed text:

- the code of the project's domain;
- the project's versioning and publishing policy;
- whether the project runs continuous integration and pull requests, and how. The installed text
  requires neither, and no installed instruction outside an offered shape assumes either. An offered
  shape is a part of the installed text, or a file the setup writes, that a project takes or leaves
  as a whole; one for either may ship, per
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. In a private project with one
  contributor, continuous integration on pull requests spends limited minutes, and a pull request
  brings little value.

Each installed instruction says what it asks plainly enough to follow without guessing what it
means. No two instructions are ones a session cannot both obey, as
`goal@agent-skills@no-contradiction-in-workflow` states. What no instruction covers is left to the
agent's judgement on purpose, per `goal@agent-skills@only-needed-instructions`.

It is met while the installed text holds:

- a skill for each generic activity, as `goal@agent-skills@one-skill-per-activity` measures, and in
  it each step the list above names for that activity;
- each instruction with one reading;
- no two instructions a session cannot both obey, as
  `goal@agent-skills@no-contradiction-in-workflow` measures;
- no instruction outside an offered shape that requires or assumes continuous integration or a pull
  request.

A review round, an argument derived again, or a decision reversed by accident, that the workflow
would have spared, shows the goal unmet in practice.

## A mechanical task the workflow asks of a session is done by a command or a library function of a published crate of knowledge-architect `##mechanical-tasks-are-the-tool-s`

Where the installed text asks a session for a task whose result follows, with no judgement, from the
tree, the repository's history, the workflow's rules and what the session gives it, and that takes
several steps or a script the session would write, a published crate of knowledge-architect offers a
command that does it, or a library function that the installed text has the project's maintenance
binary call, the binary from which the project runs its gates, per `goal@gates@gates-from-a-list`,
so every session gets the same result without building it again. Rewriting a branch's history holds
such tasks: rebasing it onto a moved main branch where no conflict arises, with the check that every
commit carried over; applying a message the session wrote to a commit that is not the newest; and
folding a repair into the commit it repairs, with the check that no content was lost. When such a
command or function is built follows the order of known work, which is knowledge-architect's
owner's, per `goal@knowledge-architect@the-owner-decides`.

It is met while each such task the installed text asks of a session has such a command, or such a
function that the installed text has that maintenance binary call, of a published crate of
knowledge-architect. A task found in real use with none shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## The documentation structure and the workflow work together `##structure-and-workflow-work-together`

The workflow writes into the structure, and the structure gives the workflow what it needs:

- one place, the home the knowledge table gives, to find each decision that earns a design head with
  its argument;
- one such home each, in the anchor it belongs to, for what is outstanding and for the evidence that
  would flip a recorded decision.

The checker computes two lists the workflow asks a session to read:

- what a reversed decision touches;
- which issues and tripwires a change must read again.

Neither the structure nor the workflow is designed without the other.

It is met while each kind of record the installed text asks a session to write has a home the
installed text gives it, the manifest declaring each such home that is a register's, and those two
lists are computed by the checker.

## The project's records live in its repository, beside the code `##records-live-beside-the-code`

In a project that adopts the workflow, these live in its repository beside the code:

- the project's own decisions;
- what it has outstanding;
- the evidence that would flip a decision.

They live in the homes the knowledge table gives them, the project's own rows included. They are
files an agent reads and edits like any other, and the history of the repository's commits, rather
than entries in an external tracker it would need a separate tool to reach.

A retrospective's files are no record of the project that wrote them: the workflow's file, which a
project sends to knowledge-architect's owner, and the project files. An intake makes their
analysis a document of the project it runs in.

It is met while every home the knowledge table gives a decision, an outstanding item or a tripwire
of the project is a file or a directory of the repository, or its commit history.

## The documentation stays consistent with the code and with itself `##documentation-stays-consistent`

In a project that adopts the workflow, its reviews and its audits keep the documentation consistent
with the code and with itself. In a project that uses the checker, the checker supports them, and
makes the same guarantees where it is used alone:

- its check guarantees that, in the texts the checker reads, nothing cited is missing;
- its check guarantees that, in the texts the checker reads, every rule the checker holds, compiled
  in or declared in the manifest, is met, save those a declaration of the manifest switches off;
- its commands list and display the entries a review must read.

It is met while:

- the installed text owes, before every merge into the main branch, a review that reads the
  documents the branch touches, and those that describe the code it touches, against the code and
  against each other;
- the installed text offers an audit that reads the whole project on one aspect;
- the check reports every reference that resolves to nothing in the texts the checker reads, and
  every breach of such a rule.

Development driven by agents changes code faster than review alone can follow, so the reviews need
these tools to keep up.

## The owner decides what the argument from the goals, the record, the code and the sources leaves open, and every action that cannot be undone, and is informed of every decision the agents make that earns durable text or a line of a turn's Decided section `##the-owner-decides`

In a project that follows the workflow, this one included, its owner states the project's intent in
its goals and brings the facts only the owner holds. The record, in the singular, in this goal and
in the other goals of this home, is what the project keeps beside its goals: its design heads, its
rejected designs, its tripwires, its issues, and the reasons recorded beside its code and its
agent-facing text and in the messages of the commits that made them.

Agents decide, by argument, what these settle: the goals, the record, the code and its measurements,
and the sources the project follows. They report each such decision that earns durable text, as
below.

The workflow sends to the owner, for a ruling or an approval, these and nothing else:

- a choice between shapes that all survive the argument, where choosing weighs a cost or a scope
  that no goal or decision ranks, such as the order of known work;
- a contradiction that no source or argument resolves;
- a goal to set or change;
- a fact only the owner holds;
- whatever the argument settles, an action that cannot be undone, such as a publication;
- whatever the argument settles, an ask of knowledge-architect that binds every target project, per
  `goal@knowledge-architect@serves-one-kind-of-project`.

A project may reserve further actions to its owner. A proposal on what the argument settles, which
the agent puts before the owner for challenge inside an exchange, as
`goal@knowledge-architect@design-with-agents-as-counterparts` defines one, is not sent for a ruling.

The owner changes a decision the argument settled through a goal, a fact or an argument, which the
agents weigh like any other, and never by a word that gives none. A choice at a fork is the owner's
to make by a bare pick.

Each decision the agents make that earns durable text, such as a design head, a `rejected` entry, an
issue, a tripwire, a commit message or a comment, or that earns a line of a turn's "Decided"
section, is reported in the record, in a commit message, or in that section. The owner can open the
exchange on it. The two halves are one intent, since agents decide what the argument settles only
while the owner can see each such decision.

It is met while the installed text:

- sends each question to the authority that answers it;
- sends to the owner, for a ruling or an approval, these and no other;
- has the session report each such decision so.

A question put to the owner for a ruling that the goals, the record, the code and its measurements,
or the sources answered, or such a decision reported nowhere, shows the goal unmet in practice.

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
- asks for a goal's premises to be checked when the goal is written or reworded, and when work meets
  evidence against one;
- holds the work that rests on a contradiction.

Work built on a word whose premise a check would have shown false shows the goal unmet in practice.

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

## The owner's attention is spent on what is the owner's to read or answer `##owner-attention-on-what-is-theirs`

The agent carries the investigation, the measurements and the tracking of what is outstanding, so
that the owner's attention goes to what is the owner's to read or answer:

- the proposals put before the owner;
- their arguments;
- the questions only the owner answers;
- the reports of what agents decided.

It is met while the installed text gives the agent each of those tasks, and puts a proposal before
the owner with its grounding and its consequences already established.

## knowledge-architect serves one kind of project: a Rust project whose development AI agents drive, under a linear history `##serves-one-kind-of-project`

knowledge-architect is built for its owner's own projects, open source or private. They share one
toolchain, so that one set of agent instructions and mechanical tools serves all of them, and work
on one makes each of the others more efficient. That is the motive; ownership is no condition of
what follows. A target project is a project that holds all of these:

- the code it builds and ships is Rust, built by Cargo, apart from the glue its targets need, such
  as the JavaScript and the HTML of a web target, or shaders. Files the workflow installs, the
  configuration of a continuous integration, and scripts kept beside the code are no code it ships;
- AI agents drive its development: its code, its documents, its records and its reviews. An edit
  that a person makes now and then, the project's owner or an outside contributor, leaves it a
  target project. The part of the project's owner is what
  `goal@knowledge-architect@the-owner-decides`,
  `goal@knowledge-architect@design-with-agents-as-counterparts` and
  `goal@knowledge-architect@owner-attention-on-what-is-theirs` describe for a project that follows
  the workflow, the same in a project that uses the checker alone; their measures apply where the
  installed text is installed;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect, from the moment of that decision.

knowledge-architect asks more of a target project, and every target project is expected to follow
what it asks. The set of what it asks is open. An ask that binds every target project is added by a
design decision of knowledge-architect, or by taking in a quality tool, which then becomes an ask so
that every target project adopts it; either way, adding it goes to knowledge-architect's owner,
whatever the argument settles, per `goal@knowledge-architect@the-owner-decides`. A quality tool, in
every goal of knowledge-architect, is a tool that judges the project's quality mechanically, such as
a gate or a check; a tool that serves efficiency, such as a command that rewrites history, becomes
no ask; a procedure of the installed workflow is none, and a shape of continuous integration or of
pull requests becomes no ask, but ships only as an offered shape. Using the installed workflow is
never asked: a target project may use the checker alone. The installed text states each ask, those
listed here included. What it asks includes:

- the project is a git repository, with its `.git` at or above the project's directory, and its
  agents run under the `claude` harness;
- it runs every gate it owes through the gates library of knowledge-architect, from one command;
- that command judges whether the gates let a branch merge, with the same invocation in every
  target project.

A branch merges into the main branch only once that command, in its invocation for a merge, passes
on the branch's head, and once
every other condition the goals owe before a merge holds, among them: where the project uses the
installed workflow, the review the installed text owes, the ruling of the project's owner on each
choice and goal left open that the branch builds or holds as a default, per
`goal@knowledge-architect@design-with-agents-as-counterparts`, and the answer to a contradiction the
branch's work rests on, per `goal@knowledge-architect@the-owner-s-word-is-verified`; and, in every
target project, the word of the project's owner on each action of the branch's work that cannot be
undone, such as a publication, and on the merge where the project reserves it. A project may reserve
to its owner actions, such as a merge, never the work the definition above gives to agents: its
code, its documents, its records and its reviews.

How an ask is met in every target project, such as the name of the command and its invocation for a
merge, is a design decision of knowledge-architect; how one project meets the rest is its own.

A target project that does not yet follow an ask is still a target project, and brings itself into
conformance over time. knowledge-architect guarantees its working in a target project as far as the
project follows what it asks: a defect that shows only where an ask is not followed is no defect of
knowledge-architect. Nothing in knowledge-architect is built only to serve a state that does not
follow an ask, other than what helps a project into conformance: not support in the installed text
or the crates for a gate runner a project keeps beside the gates library, not a mode of the checker
for another harness.

The definition of a target project, and what knowledge-architect covers, its activities, its checks
and its tools, are not limited, and nothing of them is removed, on the argument that a current
target project does not need it or does otherwise. That argument still counts for the economy of
the installed text, per `goal@agent-skills@only-needed-instructions`, for the admission of an
instruction, per `goal@knowledge-architect@the-workflow-improves-through-real-use`, for the
choice of knowledge-architect's owner whether to take in a generic part, per
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

It is met while the installed text states each ask, and the setup skill and the
README of each published crate state the four conditions and every ask the installed text states,
and say that knowledge-architect guarantees nothing outside the four conditions, and nothing where
an ask is not followed.

## Every target project can adopt knowledge-architect, and a new one is served first `##every-target-project-can-adopt-it`

Nothing particular to one target project is compiled into the published crates, the tree they are
built from aside. For every other statement of the goals, this repository is a target project that
adopts, installs and follows the workflow and uses the checker. A project declares in its manifest
the structure particular to it. The installed text names no path or convention particular to one
target project, and may rely on what knowledge-architect asks of every target project, as
the installed text states it, per
`goal@knowledge-architect@serves-one-kind-of-project`. Every entry is cited in one readable grammar,
the same in every project, so what an agent learns in one project applies in the next. A project
pins the version it uses, and moves to a new one when it chooses.

A new target project, which adopts the checker and the workflow from its first commit, is served
first. An existing target project can adopt them too, by bringing its documents into
conformance; what that costs it is never an argument against a check the workflow needs to work
well.

It is met while:

- the checker and the installed text hold no path or convention particular to one target project;
- the checker enforces one reference grammar in every project;
- a project's pin moves only by an explicit edit;
- the setup skill leads a new target project and an existing one to a passing check.

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
a procedure that the session's judgement already carries out well enough, it ships in the form
`goal@agent-skills@only-needed-instructions` admits. Until a part taken in ships, the project's own
copy of it stays.

It is met while:

- the installed text has each tool and procedure that a session sets up or changes, to make
  development more efficient or the project better, reported to knowledge-architect's owner, each
  report saying which part of it the session finds generic by the test, and whether
  knowledge-architect already ships it;
- each part the owner has taken in is shipped, or an issue entry of knowledge-architect records the
  work;
- no skill, agent definition, `CLAUDE.md` file or command of this repository's own states the
  procedure of a part that knowledge-architect ships, beyond a restatement the primer admits.

A part the owner has taken in and knowledge-architect ships, found held again in one target project
only, shows the goal unmet in practice.

## The workflow leads agents to write code of better quality, whose units each have one responsibility and whose logic is testable apart from its side effects `##code-has-one-responsibility-per-unit-and-is-testable`

In a target project, the installed text about developing aims at better code quality overall. What
it requires are three properties of the project's Rust code, the Rust of its tests and of its
maintenance tools included, the third property excepting the tests:

- each unit, such as a function, a type, a trait or a module, has one responsibility;
- the code is written in two layers: the logic, as pure code, and a layer that carries out its side
  effects, such as reading or writing files, running a process, or reading the clock, the network or
  a display, and that takes no decision a test of the logic should cover. That layer needs no tests
  of its own;
- the logic is testable without the side effects, and is tested where a change adds or changes
  logic.

Any other instruction toward code quality is admitted as
`goal@agent-skills@only-needed-instructions` admits one.

The installed text states them as the default shape of a change, for the code the change writes
and for each unit it edits. A change that departs from them, or that leaves an edited unit departing
from them, states why at the code, or in a design head the code cites. The review of code during
development judges each property, and whether each departure is argued. The goal covers the shape of
the code, not how to write Rust idiomatically.

It is met while the installed text about developing requires each of the three properties as the
default shape of a change, for the code the change writes and for each unit it edits, requires that
each departure be argued at the code or in a design head the code cites, and holds a review of
code that judges each property and whether each departure is argued.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## The workflow leads a target project to be split into Components that each function as one coherent whole with one purpose `##projects-are-split-into-components`

A Component is what the checker's manifest declares: a directory, or the project's root. The
installed text leads a session toward a target project whose Components that hold Rust code in
their own files, those that no Component nested under their directory contains, the Rust of
maintenance tools and code not yet released included, each:

- function as one coherent whole that achieves one purpose, which the head of its goals home states
  and its goals serve. A goal of the Component that does not serve that purpose shows a second
  purpose;
- have their own public interface, such as a library API, a command line, a user interface or a
  network protocol, with a reason to change on its own;
- have their own tests, a Component that holds only a layer of side effects excepted, as
  `goal@knowledge-architect@code-has-one-responsibility-per-unit-and-is-testable` excepts that
  layer;
- can carry a coherent design in their own documents.

A Component's goals home and design home then stay small enough to read whole, its code stays
focused, and a change to the structure of the code touches few Components. A Component whose own
files hold no Rust code, such as a project's root over a workspace, is outside these four. How a
Component maps to Cargo packages, such as one crate per Component, is a design decision.

The installed text leads a session to place each piece of work by these properties, new work and
the split of an existing Component that holds two purposes alike. A Component that departs from
them, such as one that holds a second purpose, states why in a head of its own design home. How the
installed text leads there is a design decision. A target project with one purpose and one
Component needs no split.

It is met while the installed text states the four properties, leads a session to place new work
by them and to split an existing Component that holds two purposes, and requires that a Component
that departs from them state why in a head of its own design home.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## Adopting knowledge-architect is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect in a target project finds what to do in its
documentation, where they already look for it:

- the README of each published package;
- the API documentation of each library;
- the installed skills;
- the rendered site of knowledge-architect's documents.

The documentation says what to do in plain words, with examples.

It is met while:

- each published package's README and each library's API documentation state what an adopter of that
  package does;
- the installed skills, taken together, state the whole of adopting it, and so does the rendered
  site;
- each of them does so in plain words and with examples;
- the setup skill leads an adopter from the first install of the skills to a passing check.

An adopter who had to read the source or ask knowledge-architect's owner shows the goal unmet in
practice.

## The workflow improves through real use `##the-workflow-improves-through-real-use`

Real use is the test of how an agent behaves under the workflow: a session that applies it inside a
real target project that has installed it, whether or not the project follows every ask yet, as part
of work meant to merge into that project's main branch, under the supervision of a person who
contributes to the project or owns it.

No mock task and no mock project is built to apply the workflow to and observe how an agent behaves,
whether to originate an edit, to choose between shapes, to accept a piece of work, or to check that
a text runs as written.

The retrospective, and the observations of real sessions by the owner of the project they ran in,
collect what was unclear, missing or wrong. Where `goal@knowledge-architect@the-owner-decides` gives
a change in the workflow to an owner, that owner is knowledge-architect's.

An installed text that fails a goal of knowledge-architect, every Component's included, is a
defect provable by reading.

It is met while the installed text:

- admits an instruction about how an agent behaves only on evidence from real use or on a defect
  provable by reading;
- has no session build a mock task or a mock project to apply the workflow to and observe how an
  agent behaves;
- has the retrospective collect the evidence of real use.

## Adopting knowledge-architect sets up quality tools that serve target projects `##setup-brings-quality-tools`

A target project that adopts knowledge-architect is led to set up the quality tools, as
`goal@knowledge-architect@serves-one-kind-of-project` defines them, whose generic part
knowledge-architect's owner has taken in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: each is an ask, and the setup
skill gives it in a recommended shape that works well with the checker. Among these tools are three:

- one command running every gate owed before a merge;
- commit messages checked as documents;
- the gates of Rust: formatting, the linter and the tests.

Continuous integration is none of the three. The setup skill may offer a shape for it, and requires
none, per `goal@knowledge-architect@agents-get-a-complete-workflow`.

It is met while the setup skill gives each such tool in its recommended shape, the three included,
so that a target project set up by following it designs none of them itself.
