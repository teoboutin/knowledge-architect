# Goals — gates

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@gates@design`. A goal stays here while it is met, and leaves only when the owner
abandons it. This component is published, so its goals are encouraged to refine a goal of the
project as a whole, `register@knowledge-architect@goal`.

## A Rust project gets its merge gates by writing no gate logic beyond its list of gates and the call that runs them `##gates-from-a-list`

A Rust project that adopts knowledge-architect runs every gate of its list through this library,
writing no gate logic beyond that list and the call that runs it. These come with the library:

- the runner;
- the complete logs;
- the distilled failures;
- the verdict.

It is met while the library provides each of these, so that a maintenance binary using it holds no
gate logic beyond its list and the call that runs it: no runner, no log, no distiller and no verdict
of its own.

It refines `goal@knowledge-architect@setup-brings-quality-tools`.
