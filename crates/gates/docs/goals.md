# Goals — gates

The package `knowledge-architect-gates`, called the library below, runs a target project's merge
gates, so that the project writes as little gate logic of its own as the goal below allows.

## A target project gets its merge gates by writing no gate logic beyond its list of gates and the call that runs them, save a behaviour it keeps until it moves to a version that ships it `##gates-from-a-list`

A target project runs every gate of its list through this library, as
`goal@knowledge-architect@serves-one-kind-of-project` asks of it, writing no gate logic beyond that
list and the call that runs it. Gate logic is each item below. A behaviour of a gate command that no
released version of knowledge-architect ships may be written in the project's own code, and stay
there until the project's own move to a released version that ships it: the copy goes at that move.
In knowledge-architect's own repository, which runs the library it builds, a behaviour is shipped
once it merges into the main branch, and the copy goes at that merge. These come with the library:

- the runner;
- the complete logs;
- the distilled failures;
- the verdict, and the verdict that says whether the gates let a branch merge;
- each other behaviour of a gate command that knowledge-architect's owner has taken in as generic,
  per `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

It is met while the library's tests show that it provides each of these, so that a project's own
code using it holds no gate logic beyond its list and the call that runs it, save a behaviour of a
gate command that the project keeps until its own move to a released version of knowledge-architect
that ships it, or, in knowledge-architect's own repository, until the merge that ships it.

It refines `goal@knowledge-architect@serves-one-kind-of-project`, whose ask it serves, and
`goal@knowledge-architect@setup-brings-quality-tools`, whose recommended shape of the one command
it provides.

- Requires, for instance: a target project writes no code of its own to keep a gate's complete log
  or to distil its failure.
- Admits, for instance: a project that keeps in its own code a behaviour of a gate command until
  its own move to a released version of knowledge-architect that ships it.
- Admits, for instance: a project that writes in its own code a behaviour knowledge-architect's
  owner has taken in, before a released version ships it.
- Rules out, for instance: a target project that writes its own runner for its gates.
- Rules out, for instance: a project that keeps its own copy of a behaviour after its move to a
  released version that ships it.
