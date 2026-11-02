# Development

## The developing activity

### The developing activity has an installed skill, which a project's own development skill only adds to `##development-skill`

`skill@knowledge-architect-development` covers writing a target project's Rust code, from grounding
to the commit and the reviews it owes: the reading of the record of the code changed, claims with
tests shown to discriminate by `cargo x mutate`, the default shape of the code, the gate, the
commit's claim check, the review axes for code and their cadence, the conditional performance step,
the optional diff mutation sweep, and the comment species. It is ported from this repository's own
development skill and thaum's, whose generic parts the owner took in, per
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, and it delivers the developing
activity of `goal@knowledge-architect@agents-get-a-complete-workflow`. A project's own development
skill stays and only adds, per `design@agent-skills@overlay-by-separate-skills`; this repository's
own keeps what is particular to it.

It states no pull request and no continuous integration, which the workflow ships only as offered
shapes, and names no scoped `CLAUDE.md`, which a project may not have. It restates no rule of the
primer beyond one sentence with its pointer, per `design@agent-skills@restatement-size-test`: the
commit's claim check is that sentence, since the check binds at each commit and the primer is read
once at the start, and the reading of the record of code is the primer's, which the skill points to.

### The development skill states a default shape of code, which binds the lines a change writes `##default-code-shape`

The installed text states one default shape of code: one responsibility per unit, read as one
reason to change; no fragmentation; the logic pure and its side effects in a thin layer that decides
nothing a test of the logic should cover; and the logic tested without the side effects. It is a
fragment, `path@agent-skills@fragments/default-code-shape.md`, hosted by the development skill and
the code-shape reviewer, so the reviewer applies the standard it reads. A departure the review
reports in code a change writes is restructured or argued in a comment at the code; in a unit the
change edits, only the lines the change writes are weighed.

`goal@knowledge-architect@the-workflow-leads-to-better-code` asks for a default shape and a review
that weighs a branch against it, and lists no property. The owner ruled that the development skill,
not a goal, states the shape, and how far it binds. "One reason to change" is the reading of one
responsibility that a test can apply: the reading "does one thing" leads to code where everything is
a function, which the fragmentation clause names as a departure too.

### Each test a change adds names its claim, and a test of logic carries a recorded mutation `##test-economy`

Mutation evidence is owed by the kind of the test: a test of logic carries a named, recorded
mutation, and for a golden test, a test with heavy mocks or a nondeterministic one the writer picks
the evidence, since whether a mutation means anything depends on a plausible wrong implementation in
the logic the test exercises. The table is in the default shape's fragment. The code-shape reviewer
judges it, so it costs no dispatch of its own: a separate axis would read the same diff again.

### A code path is performance-critical only where a design head of the project declares it, and a change to it measures before and after `##performance-opt-in`

A target project may declare a performance-critical path in one of its design heads, with the paths
it covers and the project's own command that measures it. A change that touches a declared path
measures before and after, and its commit message records both figures. No review judges speed: a
performance claim without a measurement cannot be reproduced, so a reviewer would drop its own
findings under `skill@knowledge-architect-review@review-invariants`, and the owner held that a
benchmark procedure, not a review, guards against a regression. A project that declares no path
pays nothing. The declaration is a design head, on the owner's ruling, and a manifest key comes only
if sessions miss the step in real use. Improving performance as an activity of its own is
`issue@agent-skills@improving-performance-has-no-activity`.

### A diff mutation sweep with cargo-mutants is an optional step, whose survivors are leads for the adversarial review `##diff-mutation-sweep`

A project may run `cargo mutants --in-diff` over a branch's changed code before the adversarial
review, and gives its survivors to the adversarial reviewers as leads. It costs one build and one
test run per mutant, so it is optional, and cargo-mutants is no quality tool the setup brings, on
the owner's ruling. cargo-mutants' book states the two caveats the skill carries: a diff that touches
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

### A published crate denies, through its clippy gate, a public item without documentation `##api-doc-completeness`

The setup asks a crate a target project publishes to warn on `missing_docs`,
`clippy::missing_errors_doc` and `clippy::missing_panics_doc`, which its clippy gate, denying
warnings, turns into failures. The owner approved the lints as an ask with
`design@agent-skills@code-claims-scope`: that a public item is documented is a lint's to check, and
what its documentation says is the code-claims axis's, which reads it by the same method it reads
any statement about code. A documentation axis of its own would duplicate that method.
