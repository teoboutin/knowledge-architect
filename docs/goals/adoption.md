# knowledge-architect goals: Whom the project serves, and adoption

The scope that says which projects knowledge-architect serves, and the goals on how such a project adopts it.

## knowledge-architect serves one kind of project: a Rust project whose development AI agents drive, under a linear history `##serves-one-kind-of-project`

knowledge-architect is built for its owner's own projects, open source or private, so that one set
of agent instructions and tools serves all of them, and work on one makes the others more
efficient. Ownership is no condition. A target project is a project that holds all of these:

- the code it builds and ships is Rust, built by Cargo, apart from the code in another language
  that a platform it is built for requires. Files
  knowledge-architect installs, the configuration of a continuous integration and scripts kept beside the
  code are no code it ships;
- AI agents drive its development: its code, its documents, its entries and its reviews. An edit
  that a person makes now and then leaves it a target project;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect, from the moment of that decision.

knowledge-architect's own repository is one. Every other goal that speaks of a project, or of an
adopter, means a target project.

knowledge-architect asks more of a target project than these conditions. Among what it asks, the
project is a git repository, and its agents run under the `claude` harness. A project that does not
yet follow an ask is still a target project. knowledge-architect guarantees its working only as far
as a project follows what it asks, and builds nothing only to serve a state that does not follow an ask,
other than what helps a project into conformance: not a mode of the checker for another harness.

Nothing is built or repaired only to serve another project: not a mode for a project in another
language, not support for merge commits, not instructions for a project whose development persons
drive. A defect that shows only in such a project is no defect of knowledge-architect, and what
exists only for such a project may be removed. Anyone may use knowledge-architect in another
project, with no guarantee that it works there.

The kind is these conditions, not the projects that exist today: that a current target project does
not need a thing, or does otherwise, is no reason to narrow the conditions, or to limit or remove
what knowledge-architect covers. That argument still counts for the economy of the workflow's text,
for whether an instruction enters it, for the choice of knowledge-architect's owner to take in the
generic part of a tool or a procedure, and for the order of known work, which is
knowledge-architect's owner's.

## Every target project can adopt knowledge-architect, and a new one is served first `##every-target-project-can-adopt-it`

What knowledge-architect provides, the checker and the workflow included, holds nothing particular
to one target project, so what an agent learns in one target project applies in the next. Each
target project moves to a new version of knowledge-architect when it chooses.

A new target project, which adopts knowledge-architect from its first commit, is served first. An
existing one can adopt it too, by bringing its documents into conformance. What that costs it is
never an argument against a check the workflow needs to work well.

## Adopting knowledge-architect is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect in a target project, new or existing, finds what
to do in its documentation, where they already look for it, in plain words and with examples,
without reading its source or asking knowledge-architect's owner.
