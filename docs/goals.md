# Goals — knowledge-architect

knowledge-architect is a tool that projects use to keep the quality of their documentation, with a
focus on the record of design: what was decided, and the intent and the arguments behind it. It is
built for agentic work. It gives AI agents a complete workflow, which aims to raise the quality and
the efficiency of their work, in particular in projects developed mostly by agents. Its main
purpose is to let agents work autonomously without drifting from the design recorded in their
project, per `goal@knowledge-architect@agents-work-without-drift`. The
documentation structure and the workflow are designed to work together.

A goal is met or unmet. A decision about how the project is built is won or lost, and lives in the
design home. **A goal stays in this document while it is met.** A goal removed when it is achieved
stops being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word, per `design@agent-skills@goal-lifecycle`. The goals are the
owner's intent: an agent may propose wording, and the owner decides it.

## Agents work productively and autonomously without drifting from the design recorded in their project `##agents-work-without-drift`

The project as a whole, the workflow and the checker, lets AI agents carry more of a project's work
with less supervision, while what they build stays within the design recorded in that project. It
ranks first among the project's goals, every Component's included, where two compete, but never over
a goal's exclusion of a design, nor over the owner's authority, exchange and attention that
`goal@knowledge-architect@the-owner-decides`,
`goal@knowledge-architect@design-with-agents-as-counterparts` and
`goal@knowledge-architect@owner-attention-on-what-is-theirs` give. The supervision it saves is
measured by `goal@knowledge-architect@the-owner-decides`. It is met while the workflow and the
checker are built so that, in a project that adopts them, a departure from the recorded design is
caught before it merges: each component register, the goals, the design heads, the rejected designs,
the tripwires and the issues, is read against the work by a check or by a review step the installed
text owes. A departure that reaches a project's main branch shows the goal unmet in practice. The
checker exists mostly for this: it reports where the documents and the code drift from the recorded
design, and it makes keeping that design current cheap.

## The documentation records the design, with the intent and the arguments behind it `##design-is-recorded-with-its-arguments`

Each decision is recorded with its intent and its argument, the alternatives that lost and why they
lost, and what is still open. A later session can then tell what it may change, and what a change
costs, without re-deriving the argument or reversing a decision blind.

## AI agents get a complete workflow that raises the quality and the efficiency of their work `##agents-get-a-complete-workflow`

The workflow covers setting up a project and its goals, design discussion, planning, recording
decisions, tracking open issues, review, auditing a whole project on one aspect, and a
retrospective. It targets projects where agents do
most of the development. It aims for fewer review rounds, less re-derivation of past arguments, and
fewer decisions reversed by accident. Each instruction says what it asks plainly enough to
follow without guessing what it means, and no two parts contradict each other. What no instruction
covers is left to the agent's judgement on purpose, per
`goal@agent-skills@only-needed-instructions`. Writing the code itself is left to each project.

## A mechanical task the workflow asks of a session is done by the tool `##mechanical-tasks-are-the-tool-s`

Where the installed text asks a session for a task whose result follows from the tree and the
workflow's rules, with no judgement, and that takes several steps or a script the session would
write, the published checker offers a command that does it, so every session gets the same result
without building it again. When such a command is built follows the order of known work, which is
the owner's, per `goal@knowledge-architect@the-owner-decides`. It is met while each such task the
installed text asks of a session has a command of the checker. A task found in real use with none
shows the goal unmet in practice. It refines
`goal@knowledge-architect@agents-get-a-complete-workflow`.

## The documentation structure and the workflow work together `##structure-and-workflow-work-together`

The workflow writes into the structure, and the structure gives the workflow what it needs: one
place to find each decision with its argument, and one place for what is open. Both live in the
repository beside the code, as files an agent reads and edits like any other, rather than in an
external tracker it would need a separate tool to reach. The tool computes the workflow's work
lists: what a reversed decision touches, and which issues and tripwires a change must re-read.
Neither is designed without the other.

## The documentation stays consistent with the code and with itself `##documentation-stays-consistent`

The reviews of the workflow keep it consistent. The tool supports them. Its check guarantees that
nothing cited is missing and that the declared structure holds. Its commands list and display the
entries a review must read. Development driven by agents changes code faster than review alone can
follow, so the reviews need these tools to keep up.

## The owner decides what the argument from the goals, the record, the code and the sources leaves open, and is informed of every decision the agents make that earns a text `##the-owner-decides`

The owner states the project's intent in its goals and brings the facts only the owner holds. Agents
decide what the goals, the record, the code and its measurements, and the sources the project
follows settle, by argument, and report each such decision that earns a text, as below. The workflow
sends to the owner these, and nothing else: a choice between shapes that all survive the argument,
where choosing weighs a cost or a scope that no goal or decision ranks, such as the order of known
work; a contradiction that no source or argument resolves; a goal to set or change; a fact only the
owner holds; and, whatever the argument settles, an action that cannot be undone, such as a
publication. A project may reserve further actions to its owner. A proposal on what the argument
settles, which the agent puts before the owner for challenge inside an exchange, is not sent for a
ruling. The owner changes a decision the argument settled through a goal, a fact or an argument,
which the agents weigh like any other, and never by a word that gives none; a choice at a fork is
the owner's to make by a bare pick. Each decision the agents make that earns a design head, a
`rejected` entry, an issue, a tripwire, a commit message or a comment, or that earns a line of a
turn's "Decided" section, is reported in the record, a commit message included, or in that section,
and the owner can open the exchange on it; the two halves are one intent, since agents decide what
the record settles only while the owner sees each such reported decision. It is met while the
installed text sends each question to the authority that answers it, sends to the owner these and no
other, and has the session report each such decision so. A question put to the owner for a ruling
that the goals, the record, the code and its measurements, or the sources answered, or such a
decision reported nowhere, shows the goal unmet in practice.

## Agents verify the owner's word before acting on it, and build nothing on a contradiction `##the-owner-s-word-is-verified`

An agent following the workflow takes no word of the owner as granted. Before acting on a word other
than a goal, the agent checks its premises against the goals, the record, the code and its
measurements, and the sources the project follows. A goal's premises, those its text and the commit
messages that record a ruling on its current text state, a rewrap or a re-pointed reference
recording none, are checked when the goal is written or reworded, and again when work meets evidence
against one. A contradiction it finds that no source or argument resolves goes to the owner, and the
part of the work that rests on it waits for the owner's answer; the rest goes on, and a goal whose
premise proved false still binds the rest. It is met while the installed text asks for that check
before an act on a word of the owner in every activity of the workflow, asks for a goal's premises
to be checked when the goal is written or reworded and when work meets evidence against one, and
holds the work that rests on a contradiction. Work built on a word whose premise a check would have
shown false shows the goal unmet in practice.

## The owner designs with agents as counterparts `##design-with-agents-as-counterparts`

The owner and the agent both propose, and each challenges and refines the other's proposals on their
failures, their costs and their benefits, grounded in the record and backed by the measurements the
agent takes. The exchange is open and argued: the owner meets each such proposal with its
consequences, and its rival where one stands, before it reaches the main branch, and can challenge
any proposal, the agent's own and its refinements of the owner's included, add arguments or facts,
or change the direction of the design, reversing what a goal, the record or the argument settled,
with a goal, a fact or an argument; a pick between shapes that all survive the argument is no change
of direction. A design discussion, a goal-setting draft and a question put with a built default that
the owner rules on before the merge are such exchanges; a report alone is not. It is met while the
installed text has the agent ground and challenge every proposal of the owner that decides something
before it is built; puts before the owner, in such an exchange and with its consequences and its
rival where one stands, the agent's own proposals on a choice or a goal that the goals, the record,
the code and its measurements, and the sources leave open, and every proposal it makes inside an
exchange the owner is already in; and brings every such choice and goal to the owner before it
reaches the main branch.

## The owner's attention is spent on what is the owner's to read or answer `##owner-attention-on-what-is-theirs`

The agent carries the investigation, the measurements and the tracking of what is outstanding, so
that the owner's attention goes to what is the owner's to read or answer: the proposals put before
the owner, their arguments, the questions only the owner answers, and the reports of what agents
decided. It is met while the installed text gives the agent each of those tasks, and puts a proposal
before the owner with its grounding and its consequences already established.

## Any project can adopt it, and a new project is the primary target `##any-project-can-adopt-it`

Nothing about one project is compiled in. A project declares its structure in its manifest, and the
installed skills name no project's paths. Every record is cited in one readable grammar, the same in
every project, so what an agent learns in one project applies in the next. A project pins the
version it uses, and moves to a new one when it chooses. The target is projects developed mostly by
AI agents, and a new project, which adopts the checker and the workflow from its first commit, comes
first. An existing project can adopt them too, by bringing its documents into conformance; what that
costs it is never an argument against a check the workflow needs to work well. A project without
agents can still use the checker alone. A project in a language other than Rust adopts it in the
reduced mode of `goal@knowledge-architect@rust-projects-come-first`.

## A Rust project gets every feature, and a project in another language a reduced mode `##rust-projects-come-first`

Rust is the language the project serves first. A feature that depends on the project's language is
built for Rust, and a feature is not given up because it can serve only Rust projects. A project in
another language can still adopt the checker and the workflow, without what depends on Rust. It is
met while a Rust project gets every check and tool the project offers, and a project in another
language adopts the rest.

## Adopting the project is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect finds what to do in its documentation, where they
already look for it: the README of each package, the API documentation of each library, the
installed skills, and the rendered site of the documents. The documentation says what to do in
plain words, with examples. It is met while a project adopts the checker and the workflow by
following that documentation alone, without reading the source or asking the authors.

## The workflow improves through real use `##the-workflow-improves-through-real-use`

Real use is the test of how an agent behaves under the workflow: a session that applies it inside a
real project that already follows it, as part of a contribution submitted to that project, under the
supervision of a person who contributes to the project or owns it. No mock task and no mock project
is built to apply the workflow to and observe how an agent behaves, whether to originate an edit, to
choose between shapes, to accept a piece of work, or to check that a text runs as written. The
retrospective, and the owner's observations of real sessions, collect what was unclear, missing or
wrong; what changes is decided per `goal@knowledge-architect@the-owner-decides`. It is met while the
installed text admits an instruction about how an agent behaves only on evidence from real use or on
a defect provable by reading, has no session build a mock task or a mock project to apply the
workflow to and observe how an agent behaves, and has the retrospective collect the evidence of real
use.

## Adopting the workflow sets up quality tools proven to work with it `##setup-brings-quality-tools`

A project that adopts knowledge-architect is led to set up the quality-assurance tools that have
proven useful alongside it, in recommended shapes that work well with the checker: one command
running every check owed before a merge, continuous integration running that command, commit
messages checked as documents, and the checks of the project's language. It is met while a project
set up by following the workflow reaches these tools without designing them itself.
