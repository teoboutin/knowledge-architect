# Goals — knowledge-architect

knowledge-architect is a tool that projects use to keep the quality of their documentation, with a
focus on the record of design: what was decided, and the intent and the arguments behind it. It is
built for agentic work. It gives AI agents a complete workflow, which aims to raise the quality and
the efficiency of their work, in particular in projects developed mostly by agents. Its main purpose
is to let agents work autonomously without drifting from the design recorded in their project, per
`goal@knowledge-architect@agents-work-without-drift`. The documentation structure and the workflow
are designed to work together.

A goal is met or unmet. A decision about how the project is built is won or lost, and lives in the
design home. **A goal stays in this document while it is met.** A goal removed when it is achieved
stops being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word, per `design@agent-skills@goal-lifecycle`. The goals are the owner's
intent: an agent may propose wording, and the owner decides it.

## Agents work productively and autonomously without drifting from the design recorded in their project `##agents-work-without-drift`

The project as a whole, the workflow and the checker, lets AI agents carry more of a project's work
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
  `goal@knowledge-architect@owner-attention-on-what-is-theirs` give.

The supervision it saves is measured by `goal@knowledge-architect@the-owner-decides`.

It is met while the workflow and the checker are built so that, in a project that adopts them, a
departure from that recorded design, or a rejected design brought back without that procedure, is
caught before it merges. That project's goals, its design heads, those reasons and its rejected
designs, with the tripwires and the issues that bear on the work, are read against the work by the
checker or by a step the installed text owes an agent or a reviewer. A departure, or a rejected
design brought back, that reaches a project's main branch shows the goal unmet in practice.

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

It is met while the installed text asks for each of these to be recorded in the home the installed
text gives it:

- a head carrying its standing argument, naming its rivals in its body or in the rejected designs it
  cites;
- for a decision that earns no head, a reason in its commit message, and beside the code or the
  agent-facing text;
- what is outstanding, as an issue;
- that evidence, as a tripwire.

## AI agents get a complete workflow that raises the quality and the efficiency of their work `##agents-get-a-complete-workflow`

The workflow covers these activities, among those `goal@agent-skills@one-skill-per-activity` lists,
a list that grows with the workflow:

- setting up a project;
- setting its goals;
- discussing designs;
- planning;
- recording decisions;
- tracking open issues;
- reviewing;
- auditing a project on one aspect by the project audit's method;
- the retrospective.

It targets projects where agents do most of the development. No installed skill covers writing the
code itself, which is left to each project. Each installed instruction says what it asks plainly
enough to follow without guessing what it means. No two instructions are ones a session cannot both
obey, as `goal@agent-skills@no-contradiction-in-workflow` states. What no instruction covers is left
to the agent's judgement on purpose, per `goal@agent-skills@only-needed-instructions`.

It is met while the installed text holds:

- a skill for each of these activities and for each activity that goal names;
- each instruction with one reading;
- no two instructions a session cannot both obey, as
  `goal@agent-skills@no-contradiction-in-workflow` measures.

A review round, an argument derived again, or a decision reversed by accident, that the workflow
would have spared, shows the goal unmet in practice.

## A mechanical task the workflow asks of a session is done by the checker `##mechanical-tasks-are-the-tool-s`

Where the installed text asks a session for a task whose result follows from the tree and the
workflow's rules, with no judgement, and that takes several steps or a script the session would
write, the published checker offers a command that does it, so every session gets the same result
without building it again. When such a command is built follows the order of known work, which is
the owner's, per `goal@knowledge-architect@the-owner-decides`.

It is met while each such task the installed text asks of a session has a command of the checker. A
task found in real use with none shows the goal unmet in practice.

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
project sends to the maintainers of the workflow, and the project files. An intake makes their
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
- whatever the argument settles, an action that cannot be undone, such as a publication.

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

- asks for that check before an act on a word of the owner, in every activity of the workflow;
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

## Any project can adopt it, and a new project is the primary target `##any-project-can-adopt-it`

Nothing particular to one adopting project is compiled into the published crates, the tree they are
built from aside. For every other statement of the goals, this repository is a project that adopts,
installs and follows the workflow and uses the checker. A project declares in its manifest the
structure particular to it, and the installed text names no path or convention particular to one
adopting project. Every entry is cited in one readable grammar, the same in every project, so what
an agent learns in one project applies in the next. A project pins the version it uses, and moves to
a new one when it chooses.

The target is projects developed mostly by AI agents. Among them, a new project, which adopts the
checker and the workflow from its first commit, is the primary target. An existing project can adopt
them too, by bringing its documents into conformance; what that costs it is never an argument
against a check the workflow needs to work well.

A project without agents can still use the checker alone. A project in a language other than Rust
adopts it in the reduced mode of `goal@knowledge-architect@rust-projects-come-first`.

It is met while:

- the checker and the installed text hold no path or convention particular to one adopting project;
- the checker enforces one reference grammar in every project;
- a project's pin moves only by an explicit edit;
- the setup skill leads a new project and an existing one to a passing check.

## A Rust project gets every feature, and a project in another language a reduced mode `##rust-projects-come-first`

Rust is the language knowledge-architect serves first. A feature that depends on an adopting
project's language is built for Rust first. A feature is not given up because it can serve only Rust
projects. A project in another language can still adopt the checker and the workflow, without what
depends on Rust.

It is met while a Rust project gets every feature the published crates offer, such as a check, a
tool or a library API, and the core's README and the setup skill state what a project in another
language adopts.

## Adopting knowledge-architect is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect finds what to do in its documentation, where they
already look for it:

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

An adopter who had to read the source or ask the authors shows the goal unmet in practice.

## The workflow improves through real use `##the-workflow-improves-through-real-use`

Real use is the test of how an agent behaves under the workflow: a session that applies it inside a
real project that already follows it, as part of a contribution submitted to that project, under the
supervision of a person who contributes to the project or owns it.

No mock task and no mock project is built to apply the workflow to and observe how an agent behaves,
whether to originate an edit, to choose between shapes, to accept a piece of work, or to check that
a text runs as written.

The retrospective, and the observations of real sessions by the owner of the project they ran in,
collect what was unclear, missing or wrong. Where `goal@knowledge-architect@the-owner-decides` gives
a change in the workflow to an owner, that owner is knowledge-architect's.

It is met while the installed text:

- admits an instruction about how an agent behaves only on evidence from real use or on a defect
  provable by reading;
- has no session build a mock task or a mock project to apply the workflow to and observe how an
  agent behaves;
- has the retrospective collect the evidence of real use.

## Adopting knowledge-architect sets up quality tools proven to work with it `##setup-brings-quality-tools`

A project that adopts knowledge-architect is led to set up the quality-assurance tools that have
proven useful alongside it in the projects that follow the workflow, knowledge-architect's own and
thaum's among them, in recommended shapes that work well with the checker. Among these tools are
four:

- one command running every gate owed before a merge;
- continuous integration running that command;
- commit messages checked as documents;
- the gates of the project's language.

It is met while the setup skill gives:

- for a Rust project, each of the four in its recommended shape, so that a project set up by
  following it designs none of the four itself;
- for a project in another language, the one command, its continuous integration and its
  commit-message check in shapes that depend on no language, with a place for the gates of its
  language, so that such a project designs only the gates of its language.
