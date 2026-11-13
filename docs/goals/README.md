# knowledge-architect goals

knowledge-architect is the common toolchain of one kind of project, the target project.

Its starting point is a documentation structure that it defines, a model that represents that
structure, and a scanner. This is the starting point of the other goals: some add content to the
model, such as references, design entries and issues, while others add features that work upon it,
such as the checker and the render.

Beside that, knowledge-architect ships a workflow for agents, which has its own goals.

The documentation structure and the workflow are designed to work together: the agents record
knowledge in the modelled documentation, while tools are built upon the model, or outside of it, to
make agents more efficient at searching, updating and verifying knowledge.

These words hold in these senses for the goals of this home:

- **the checker**: a command knowledge-architect provides that checks a project's documents against
  the rules it holds for every project and those a project declares to it, its reference grammar
  among them, and reports each breach;
- **the workflow**, or **the installed text**: the instructions knowledge-architect installs in a
  target project for its agents;
- **the record** of a project is made of: its decisions with their arguments, the designs it did
  not keep and why, what is outstanding, the evidence that would flip a recorded decision, the
  entries of the kinds of entries the project declares, and the reasons recorded beside its code,
  its agent-facing text and in the messages of the commits that made them. Its goals are no part of
  it, and neither is anything else;
- **a recorded item**: a goal, or an entry of a kind that keeps a home of its own, one the workflow
  gives or one a project declares, such as a decision, a design not kept, something outstanding, or
  a piece of evidence that would flip a decision. A reason recorded beside code or in a commit
  message is none;
- **a reference**: a pointer by which one text names a recorded item, a file, a directory or a
  section of a text;
- **a tool**: a program, a command or a library function that runs without an agent's judgement,
  and that a person or an agent working in a target project runs;
- **a session**: one run of an AI agent on a project's work;
- **the generic part** of an activity, a tool or a procedure: the part that does not depend on what
  the project builds;
- **a target project**: the kind of project knowledge-architect serves, which
  `goal@serves-one-kind-of-project` defines.

The goals are in these files, in this order:

- [The documentation model](model.md): the structure a project's documentation has and the model
  of it that tools read, what the documentation records, and how a recorded item reaches the work
  it bears on.
- [What is built on the model](built-on-the-model.md): the documentation kept consistent, the
  checks a project adds, the linked site, and the tasks that are a tool's.
- [The workflow](workflow.md): the workflow agents get, the bound on what it states, and what it
  leads agents and projects to.
- [The two together](together.md): agents working autonomously without drifting from the recorded
  design, with the ranking of that goal, and the generic part of a tool or a procedure shipped once
  taken in.
- [The owner and the agents](owner.md): what the owner decides, how the owner's word is verified,
  and the design with agents as counterparts.
- [Whom the project serves, and adoption](adoption.md): the kind of project served, and its
  adoption.
