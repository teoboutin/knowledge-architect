# Development

## The developing activity

### The developing activity has an installed skill, which a project's own development skill only adds to `##development-skill`

`skill@knowledge-architect-development` covers writing a target project's Rust code, from grounding
to the commit and the reviews it owes: the reading of the record of the code changed, claims with
tests shown to discriminate by `cargo x mutate`, the default shape of the code, the module
guidelines and who splits a module, the gate, the
commit's claim check, the review axes for code and their cadence, the conditional performance step,
the optional diff mutation sweep, the comment species, and the occasion of a Component's creation
or split, at which the skill sends the session to `skill@knowledge-architect-component-creation`. It holds the generic part of this
repository's own development procedure and of thaum's, which the owner took in, per
`goal@generic-tools-and-procedures-are-shipped`, and it delivers the developing
activity of `goal@agents-get-a-complete-workflow`. A project's own development
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
change edits, only the lines the change writes are weighed. Shaping the code a change writes, a new
module for it included, is the writer's; moving code the change does not write, between modules or
out of one, is a split, which the module guidelines of
`design@agent-skills@module-split-guidelines` weigh and
`design@agent-skills@splits-planned-before-work` plans before the work.

`goal@the-workflow-leads-to-better-code` has what the workflow says about code
aim at code of better quality, and states no property of it, since a property is a mechanism and no
goal states one, per `design@agent-skills@goal-outcome-survives-empty-skeleton`: the default shape
is how the workflow pursues that aim, and the review of `design@agent-skills@code-shape-axis`
weighs each branch against it. The shape is a guideline, not a rule a unit fails alone: a departure
stays where its reason is argued at the code. What makes code better is learned from use rather
than fixed before it: the owner reported that it is hard to define before use, and that a rule or a
guideline too strict to follow makes agents take actions misaligned with what it was meant for. So
the retrospective asks whether a rule about code proved too hard or too strict, per
`design@agent-skills@premortem-as-watch-points`, and its findings change the shape, per
`goal@the-workflow-improves-through-real-use`. The nearest rival, a definition
of quality fixed as a rule every unit must meet, leaves a session whose code the definition fits
badly no move but to break the rule or to act against the code's interest. The shape binds the lines a change writes, not every unit it touches: a rule over the
whole unit would leave a one-line fix in a long function only two moves, both against the change's
interest, restructuring the function or departing from the rule. For the same reason the boundary
of what the writer reshapes stops at the lines it writes: a move of code the change does not write
would enter the diff the review of the change reads. "One reason to change" is the reading of one
responsibility that a test can apply: the reading "does one thing" leads to code where everything is
a function, which the fragmentation clause names as a departure too.

### No installed instruction teaches how to write Rust idiomatically `##the-workflow-teaches-no-rust-idiom`

The installed text teaches no idiom of Rust, such as its error types or its lifetimes, though idiom
does not depend on what a project builds. What it says about code is stated from a higher vantage:
the default shape of code, per `design@agent-skills@default-code-shape`, the module guidelines, per
`design@agent-skills@module-split-guidelines`, what a test owes, per
`design@agent-skills@test-economy`, and the criteria of a Component, per
`design@agent-skills@component-skill-delivery`. A mechanical gate on idiom, such as a set of lints,
may ship once knowledge-architect's owner takes it in; the clippy gate of the recommended list is
one, per `design@gates@a-project-holds-its-gate-list`.

`goal@only-needed-instructions` gives no instruction to a case that an agent's
judgement handles well enough, whichever goal the instruction would serve. The nearest rival is
guidance on idiom in the development skill, which
`goal@the-workflow-leads-to-better-code` could be read to call for. The owner
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
guideline as that head argues, and the
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
`goal@serves-one-kind-of-project`: the session chose so, which no goal ranks, and
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
a documentation axis of its own would duplicate that method. The lints are asked of every target
project by this head, on its argument: `goal@the-owner-decides` reserves to
knowledge-architect's owner the conditions and the asks that
`goal@serves-one-kind-of-project` itself names, a git repository and the
`claude` harness, so an ask a design head holds, as this one, changes on an argument, and the change
is reported to the owner. The setup delivers them so that every target project gets them, per
`goal@generic-tools-and-procedures-are-shipped`, which ships the generic part of
a tool once knowledge-architect's owner takes it in; the lints were taken in on the owner's word.

## Splitting code

### A module holds what one review reads against one reference, and is split on signals a project weighs, size among them only as a file a session cannot read whole `##module-split-guidelines`

The module guidelines are the fragment `path@agent-skills@fragments/module-split.md`. A module holds
what one review reads against one reference: a design head, a section of a specification the
project implements, a set of invariants, or one external effect, which one module owns, as
`design@git@git-is-asked-through-one-module` makes one module the owner of git. Three signals make a
module a candidate for a split, each with the failure a split prevents: a second reference entering
the module, a review reading code outside its subject; two parts that change apart, counting the
changes the goals predict, unrelated changes in one diff; an invariant only a boundary of visibility
protects, a helper that bypasses it made visible to code that must not call it. Three signals weigh
against a split: two parts every change reads together, one item large by itself, which no move
divides, and one owner per external effect. Size has one role, a file a session cannot read whole in
one pass, and no number of lines is a signal. A split is a move before the work, per
`design@agent-skills@splits-planned-before-work`. The fragment names no construct of one language,
per `design@agent-skills@the-workflow-teaches-no-rust-idiom`, and its hosts are the development
skill, `agent@knowledge-architect-code-shape-reviewer` and
`agent@knowledge-architect-code-split-scout`, per `design@agent-skills@shared-text-fragment`.

`goal@projects-are-split-into-modular-units` names the code of one part among the layers where a
split matters, and makes one purpose per unit a direction, not a rule a unit fails alone: so these are
guidelines a project weighs, never a rule a module fails alone. The criterion is the goal's reason
made checkable: an agent reads one unit whole and a person reviews one change, and a module read
against one reference is one that a review reads whole without reading outside its subject. The
owner reported that without such guidance modules are in practice never split and grow to thousands
of lines; that report is the real-use evidence `design@agent-skills@behaviour-additions-need-real-use`
asks of an instruction about agent behaviour. The size signal rests on a measurement of the harness:
the Read tool, given with no range a file above its cap, returned only the first part of the file,
so a review that reads such a file in parts judges code it has not read. It is recorded, with the
file read and the cap, in the message of the commit "[plans] The design audit of the slice on splits
planned before the work: its gaps answered in place, Q16 ruled, and the lines of its record search",
and taken again by one read, with no range, of a file whose tokens exceed the cap. Were a session
to read any file whole in one pass, the size signal would leave the fragment.

The nearest rival, a threshold of lines as the criterion, says nothing of what a review reads
against. The one project of the owner's that holds such a threshold gives it no measurement or
argument, and its record shows an item a move cannot bring under it, as
`path@agent-skills@docs/grounding/module-split.md` reads it: the design not kept is
`rejected@agent-skills@module-split-at-a-size-threshold`.

### A split of code the work needs is decided where the work is designed and planned as a step before the work, never left to its implementer `##splits-planned-before-work`

A split of a module or of a Component that a piece of work bears on is decided in the design
discussion of that work, and planned in its plan document as a step before the work that needs it,
or, in a milestone, as a slice of its own where the move is large. It is never left to the
implementer, and never parked as an issue. The move lands in commits that hold the move alone, each
building and passing the project's checks before its push, so that the review of the move reads a
move and the review of the work reads the work; the installed text names neither continuous
integration nor a pull request for it, per
`design@agent-skills@installed-text-requires-no-ci-and-no-pull-request`. Where no plan document
carries the work, the split is commits of its own before the work, on the same branch. The boundary
with the writer's own shaping is `design@agent-skills@default-code-shape`'s: moving code the change
does not write is a split. A signal met outside the work's area, by
`agent@knowledge-architect-code-split-scout` or by the code-shape reviewer, is routed by
`primer@met-outside-the-task`, since a plan takes no work its discussion did not decide, per
`design@agent-skills@met-outside-the-task-outcomes`. The split of a subject of a design home is
planned the same way before the harvest that writes into it, per
`design@agent-skills@subject-split-standard`.

`goal@projects-are-split-into-modular-units` leads a project toward units that each serve one
purpose, and a split nobody carries out serves none. The owner reported that a split left to
implementation is never done, since an implementer judges it too large to carry within the same
work, that an issue on a split sits untouched for a long time, and that a task not planned ahead
never gets done; these are the owner's reports, a premise open to challenge. That report defeats
the nearest rival, the split left to whoever meets it while writing the code.

### An installed scout reads the code a planned piece of work touches for split signals, before the record search of its design and at each design audit, and decides nothing `##code-split-scout`

`agent@knowledge-architect-code-split-scout` reads a draft of a plan document, or the `README.md`
files a design audit reads, and the code the work touches, and reports, per module and per
Component, each signal for a split the work pushes the unit past or that holds already, and each
signal against a split that holds. It hosts the module guidelines and the occasion of a Component's
split, `path@agent-skills@fragments/when-to-split.md`, and reads the component-creation skill's
criteria and signals against a split by their references. The design skill dispatches it at the
grounding of a discussion that expects its full path, once the draft exists and before the search of
the whole record, so that each thread a signal opens is in the work the search reads, per
`design@agent-skills@search-runs-once-the-shapes-settle`; the in-change path and bounded work
dispatch none. The planning skill dispatches it again at the design audit of each slice, and of a
spec where one runs, since the code moves between slices and between a spec's writing and its work,
and each split it reports that the planned steps do not take is a gap the audit sorts, per
`design@agent-skills@design-audit-checks-fit`. Its report is never committed, since a plan
document's directory holds no such file, per `design@model@plan-document-directory`.

The dispatch is fixed at those steps on a point of view apart from the session's, the property
`design@agent-skills@work-divided-by-judgement` admits: the session drafted the shapes the scout
judges. The nearest rival, the session reading the code for signals itself, judges the code against
the shapes it chose, from the view that chose them. The scout decides nothing and proposes no
place to cut: the session weighs each signal and decides whether and how to split, in the thread the
signal opens or by the audit's sorts, since a split is a design decision, weighed in the discussion
the session holds against the record it has read; the owner directed so when ruling on the fragment
on when to split. The record side of a split planned before the work is a command, not a scout, per
`design@agent-skills@subject-split-standard`.
