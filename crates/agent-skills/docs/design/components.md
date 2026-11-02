# Agent skills design: Components

## Creating or splitting a Component

### Creating or splitting a Component is an activity of its own, with a skill that triggers at design, development, setup and audit, and a shared fragment that sends the design and development skills to it `##component-skill-delivery`

`skill@knowledge-architect-component-creation` holds the default criteria of a Component, the
signals against a split, the mapping to crates, the shapes, the naming and the procedure of a
creation or a split. A Component is created or split at four moments, and its description names
each: new work that may belong to no existing Component, met in a design discussion; a second
reason to change found while adding code, met in development; an adopting project's crates mapped to
Components, at setup; and a Component found holding two purposes at an audit, which today is the
design-record axis's finding on the Component a head sits in, since no axis reads a Component's
purposes, per `issue@agent-skills@audit-axes-beyond-the-design-record`. The fragment
`path@agent-skills@fragments/when-to-split.md` states the occasion and the signals, and sends the
session to the skill; the design skill and the development skill host it, the two activities in
which the first two moments are met, per `design@agent-skills@shared-text-fragment`. The setup skill
points to the skill for the criteria and the naming, and the primer's list of installed skills names
its moments.

`goal@knowledge-architect@projects-are-split-into-components` asks that the installed text state
default criteria and lead a session to apply them whenever it places new work, and wherever it finds
a Component holding two purposes. The activity's methodology, a cut of a boundary followed by a
move of the record, is apart from a design discussion's and from the setup's, so
`goal@agent-skills@one-skill-per-activity` gives it a skill of its own; the owner proposed the skill
and the fragment, and the four moments extend that proposal. The skill decides nothing itself: the
decision to create or split goes through `skill@knowledge-architect-design`, and the skill points to
the setup and goal-setting skills and restates neither, per
`goal@agent-skills@no-contradiction-in-workflow`.

Its nearest rival placed the instructions in the planning skill. Whether a Component exists is a
decision about how the project is built, which `primer@design-heads` places in the design record,
not a choice of how work is cut into branches.

### One Component is one crate by default, weighed by whether the project publishes, and a Component may be a module directory where a crate would force a publication or a cycle `##component-crate-mapping`

The default holds more strongly for a project that never publishes, since a crate boundary is the
stronger isolation and more crates cost it nothing. For a project that publishes, or plans to, each
further crate costs a release in dependency order and a version bump of its own, weighed against how
usable each crate is alone. The skill reads the publication status from the project's record, asks
the owner where the record states none, and weighs the project's policy without setting it, per
`goal@knowledge-architect@agents-get-a-complete-workflow`. "One reason to change" is the test for a
Component as for a function, so the criteria of
`goal@knowledge-architect@projects-are-split-into-components` and the default code shape of
`design@agent-skills@default-code-shape` read the same way at two scales. The weighting by
publication is the owner's argument.

The exception, a module directory inside a crate, rests on two behaviours of Cargo, read in the Cargo
Book: a crate published on crates.io cannot depend on a crate given only by path, dev-dependencies
excepted ("Specifying Dependencies", section "Local paths in published crates"), and the resolver
refuses a dependency cycle outside dev-dependencies ("Dependency Resolution", section
"dev-dependency cycles"). Its nearest rival, a crate for every Component always, would force a
publication of an internal part, or a cycle Cargo refuses.

### Signals against a split: many new dependencies, a dependency cycle, or every expected change crossing the new boundary `##component-split-signals`

The skill and the fragment state the same three signals, which a session judges, and nested
Components carry the layering where many Components would sit side by side. The first two are the
owner's argument that a split which multiplies dependencies or creates a cycle is cut in the wrong
place; the third follows from the test of one reason to change, since a boundary every change crosses
separates no reason to change. Its nearest rival splits on the default criteria alone, which reads
two reasons to change in a unit and cuts it with no regard to what the cut costs between the parts.

### A shared test Component depends on the Components it tests, never the reverse, where several crates share testing tools and fixtures `##shared-test-component`

The skill offers it as a shape, not a rule: a Component that holds the tests crossing several
Components, with their fixtures and oracles, and depends on them. Its condition is that several
crates share testing tools and fixtures without depending on each other; a crate A whose tests need
a crate B that already depends on A is one case of it. The owner's argument is that it centralises
the tooling, which improves the tests, and avoids a dependency cycle.

Its nearest rival, a dev-dependency from A back onto B, is admitted by Cargo and fails in A's unit
tests: the Cargo Book, "Dependency Resolution", section "dev-dependency cycles", states that two
copies of A are then linked into the test binary, whose types are not compatible. Helpers behind a feature of A serve A's own tests and no test that needs B.
