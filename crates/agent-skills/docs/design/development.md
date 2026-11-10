# Development

## The developing activity

### The developing activity has an installed skill, which a project's own development skill only adds to `##development-skill`

`skill@knowledge-architect-development` covers writing a target project's Rust code, from grounding
to the commit and the reviews it owes: the reading of the record of the code changed, claims with
tests shown to discriminate by `cargo x mutate`, the default shape of the code, the gate, the
commit's claim check, the review axes for code and their cadence, the conditional performance step,
the optional diff mutation sweep, the comment species, and the occasion of a Component's creation
or split, at which the skill sends the session to `skill@knowledge-architect-component-creation`. It holds the generic part of this
repository's own development procedure and of thaum's, which the owner took in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, and it delivers the developing
activity of `goal@knowledge-architect@agents-get-a-complete-workflow`. A project's own development
skill stays and only adds, per `design@agent-skills@overlay-by-separate-skills`, since one installed
text cannot hold what is particular to each project.

### The development skill states a default shape of code, which binds the lines a change writes `##default-code-shape`

The installed text states one default shape of code: one responsibility per unit, read as one
reason to change; no fragmentation; the logic pure and its side effects in a thin layer that decides
nothing a test of the logic should cover; the logic tested without the side effects; and an
integrated test that drives the project through its public API, per
`design@agent-skills@integrated-test-through-public-api`. It is a
fragment, `path@agent-skills@fragments/default-code-shape.md`, hosted by the development skill and
the code-shape reviewer, so the reviewer applies the standard it reads. A departure the review
reports in code a change writes is restructured or argued in a comment at the code; in a unit the
change edits, only the lines the change writes are weighed.

`goal@knowledge-architect@the-workflow-leads-to-better-code` asks for a default shape and a review
that weighs a branch against it, and lists no property. The shape binds the lines a change writes,
not every unit it touches: a rule over the whole unit would leave a one-line fix in a long function
only two moves, both against the change's interest, restructuring the function or departing from the
rule, and the owner reported that rules too strict make agents take misaligned actions. That the
development skill, not a goal, states the shape is the session's choice over a goal listing the
properties, which no goal ranks; the owner approved it. "One reason to change" is the reading of one
responsibility that a test can apply: the reading "does one thing" leads to code where everything is
a function, which the fragmentation clause names as a departure too.

### No installed instruction teaches how to write Rust idiomatically `##the-workflow-teaches-no-rust-idiom`

The installed text teaches no idiom of Rust, such as its error types or its lifetimes, though idiom
does not depend on what a project builds. What it says about code is stated from a higher vantage:
the default shape of code, per `design@agent-skills@default-code-shape`, what a test owes, per
`design@agent-skills@test-economy`, and the criteria of a Component, per
`design@agent-skills@component-skill-delivery`. A mechanical gate on idiom, such as a set of lints,
may ship once knowledge-architect's owner takes it in; the clippy gate of the recommended list is
one, per `design@gates@a-project-holds-its-gate-list`.

`goal@knowledge-architect@only-needed-instructions` gives no instruction to a case that an agent's
judgement handles well enough, whichever goal the instruction would serve. The nearest rival is
guidance on idiom in the development skill, which
`goal@knowledge-architect@the-workflow-leads-to-better-code` could be read to call for. The owner
argued that LLM agents know how to write Rust code, at least those performant enough to follow this
workflow, and that guidelines from a higher vantage are a more valuable use of the instruction
budget. The workflow targets frontier-tier models, per
`design@agent-skills@frontier-tier-only`, so idiom is a case their judgement handles, and an
instruction on it is read by every session that loads the development skill.

### Each test a change adds names its claim, and a test of logic carries a recorded mutation `##test-economy`

A test that maps to no claim is cost with no information: the owner reported that agents add tests
on a whim, which the claim each test names exposes. Mutation evidence is owed by a test of logic, a
named and recorded mutation; for a golden test, a test with heavy mocks or a nondeterministic one
the writer picks the evidence, since whether a mutation means anything depends on a plausible wrong
implementation in the logic the test exercises. An integrated test also has the kind of what it
exercises, so a test of logic run in a mock environment, or through the public API as an integrated
test, owes its mutation all the same. The rule and its table are in
`path@agent-skills@fragments/default-code-shape.md`, which `design@agent-skills@default-code-shape`
places, and the code-shape reviewer judges them, per `design@agent-skills@code-shape-axis`: a
separate axis would read the same diff again.

### An integrated test drives the project through its public API, as a consumer would, and what it cannot observe there is fixed in the API or belongs to a unit test `##integrated-test-through-public-api`

An integrated test, in a mock environment or not, uses only what a consumer of the project could
use. Where what it needs has an effect a consumer could see and it cannot observe it in what the
public API returns, the API hides it from consumers too, and the API is what is fixed; where what it
would observe has no effect a consumer could see, the claim is a unit test's. Code that no
integrated test can reach and no consumer can observe is suspect, and its comment and the commit
that added it are read before any of it is removed, per `design@agent-skills@local-intent-binds`.
The statement is a property of the default shape of `design@agent-skills@default-code-shape`, a
guideline per `goal@knowledge-architect@the-workflow-leads-to-better-code`, and the
component-creation skill defines the integrated test in the same words, for the tests a shared test
Component holds.

The owner argued that testing through the public API drives the improvement of that API, shows where
it lacks, benefits its consumers, such as those who write extensions, and shows code that is
unreachable or has effects a consumer cannot observe, which a unit test inside the crate cannot test
either, probably a sign of a defect in the design or of dead code. The owner scoped it to the
integrated test, and to no mock environment in particular, since not every project uses mocks or
serves other projects. Its nearest rival, every test through the public API, is
`rejected@check@unit-tests-moved-under-tests`: as its measurement shows, a unit test often runs over
private functions, which such a move makes public or whose tests it drops, so the guideline leaves a
unit test where it is.

### A change to a code path a project declares performance-critical measures before and after, and no review judges speed by reading `##performance-opt-in`

A target project may declare a performance-critical path in one of its design heads, with the paths
it covers and the project's own command that measures it. A change that touches a declared path
measures before and after, its commit message records both figures, and the code-shape reviewer
checks that they are recorded. No review judges speed: a performance claim without a measurement
cannot be reproduced, so a reviewer would drop its own findings under
`skill@knowledge-architect-review@review-invariants`, and a benchmark procedure guards a regression
where reading code cannot. A project that declares no path meets no step. That the declaration is a
design head rather than a manifest key is the session's choice, which no goal ranks, the key to come
only if sessions miss the step in real use; the owner approved it. Improving performance as an
activity of its own is `issue@agent-skills@improving-performance-has-no-activity`, and the check that
a change to a declared path records its figures is
`issue@agent-skills@no-mechanical-check-reads-a-declared-path-s-figures`.

### A diff mutation sweep with cargo-mutants is an optional step, whose survivors are leads for the adversarial review `##diff-mutation-sweep`

A project may run `cargo mutants --in-diff` over a branch's changed code before the adversarial
review, and gives its survivors to the adversarial reviewers as leads. It costs one build and one
test run per mutant, so it is optional. cargo-mutants is no gate of the list the setup recommends, per
`design@gates@a-project-holds-its-gate-list`, and no ask, per
`goal@knowledge-architect@serves-one-kind-of-project`: the session chose so, which no goal ranks, and
the owner approved it. cargo-mutants' book states the two caveats the skill carries: a diff that touches
only test code yields no mutant ("Testing code changed in a diff"), and rust flags set in a cargo
configuration file are ignored ("Limitations"). It replaces no recorded mutation check: it chooses
its own mutants, per `rejected@mutate@cargo-mutants-instrument`.

### A project that finds a shipped tool insufficient builds its improvement as a tool of its own beside it, and reports it upstream `##upstream-tool-loop`

The improvement is never a patch of the shipped tool: a fix of the installed text or of the checker
goes upstream, per `design@agent-skills@installed-fix-goes-upstream`, and what a project adds stays
its own. The project's own tool stays until the improvement ships, and the report goes to the
retrospective's workflow file. A consumer that no longer owns a tool may otherwise stop improving
it, and thaum's mutation tool was built because outside tools fell short. Co-developing a shipped
tool through a Cargo `[patch]` waits for a first real need, `issue@agent-skills@co-development-through-a-cargo-patch`.

### That a published crate's public items are documented is a lint's to check, through the clippy gate `##documentation-presence-is-linted`

The setup asks a crate a target project publishes to warn on `missing_docs`,
`clippy::missing_errors_doc` and `clippy::missing_panics_doc`, which its clippy gate, denying
warnings, turns into failures. A lint answers whether an item is documented with no reading, and
what its documentation says is the code-claims axis's, per `design@agent-skills@code-claims-scope`;
a documentation axis of its own would duplicate that method. The lints are an ask of every target
project, which adding goes to the owner's word, per `goal@knowledge-architect@the-owner-decides`,
and the owner gave it; the setup delivers them so that every target project gets them, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.
