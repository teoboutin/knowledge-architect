# Goals — gates

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@gates@design`. A goal stays here while it is met, and leaves only when the owner
abandons it. This component is published, so its goals are encouraged to refine a goal of the
project as a whole, `register@knowledge-architect@goal`.

## A target project gets its merge gates by writing no gate logic beyond its list of gates and the call that runs them `##gates-from-a-list`

A target project runs every gate of its list through this library, as
`goal@knowledge-architect@serves-one-kind-of-project` asks of it, writing no gate logic beyond that
list and the call that runs it. Gate logic is each item below. A behaviour of a gate command whose
generic part the owner has not yet taken in, or has declined, may stay in a project's binary until
it is taken in. These come with the library:

- the runner;
- the complete logs;
- the distilled failures;
- the verdict, and the verdict that says whether the gates let a branch merge;
- each other behaviour of a gate command that knowledge-architect's owner has taken in as generic,
  per `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

It is met while the library provides each of these, so that a maintenance binary using it holds no
gate logic beyond its list and the call that runs it.

It refines `goal@knowledge-architect@serves-one-kind-of-project`, whose ask it serves, and
`goal@knowledge-architect@setup-brings-quality-tools`, whose recommended shape of the one command
it provides.
