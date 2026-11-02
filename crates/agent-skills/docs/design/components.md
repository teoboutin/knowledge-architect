# Agent skills design: Components

## Creating or splitting a Component

### Creating or splitting a Component is an activity of its own, whose skill each activity that meets its occasion reaches by its description, the primer's list, or a shared fragment `##component-skill-delivery`

`skill@knowledge-architect-component-creation` holds the default criteria of a Component, the
signals against a split, the mapping to crates, the shapes, the naming and the procedure of a
creation or a split. Its description names the moments a Component is created or split, and the
primer's list of installed skills names them too. Today they are four: new work that may belong to
no existing Component, met in a design discussion; a second reason to change found while adding
code, met in development; an adopting project's crates mapped to Components, at setup; and a
Component found holding two purposes at an audit, which today is the design-record axis's finding on
the Component a head sits in, since no axis reads a Component's purposes, per
`issue@agent-skills@audit-axes-beyond-the-design-record`. The fragment
`path@agent-skills@fragments/when-to-split.md` states the occasion and the signals, and sends the
session to the skill; the design skill and the development skill host it, the two activities in
which the first two moments are met, per `design@agent-skills@shared-text-fragment`. The setup skill
points to the skill for the criteria and the naming.

`goal@knowledge-architect@projects-are-split-into-components` asks that the installed text state
default criteria and lead a session to apply them whenever it places new work, and wherever it finds
a Component holding two purposes. The activity's methodology, a cut of a boundary followed by a
move of the record, is apart from a design discussion's and from the setup's, so
`goal@agent-skills@one-skill-per-activity` gives it a skill of its own: that goal counts every
activity the owner has taken in, and its list of activities grows with the workflow. The owner
proposed a skill of its own with a fragment in the design skill, on the argument that not every design
discussion needs the procedure, since creating a Component should be occasional; the session
extended the proposal to the development skill and to the four moments. The skill decides nothing itself: the decision to create or split goes through
`skill@knowledge-architect-design`, and the skill points to the setup and goal-setting skills for
the declaration and the goals.

Two rivals lost. Writing the whole procedure into the design skill would load it into every design
discussion, most of which create no Component. Placing it in the planning skill misplaces the
decision: whether a Component exists is a decision about how the project is built, which
`primer@design-heads` places in the design record, not a choice of how work is cut into branches.

### A Component that holds Rust code is one crate `##component-crate-mapping`

A split creates one crate for each Component it cuts out that holds Rust code in its own files, and
a Component is never a module directory inside a crate, `rejected@agent-skills@module-directory-component`.
The shape that admits a module directory rests on two rules of Cargo that
`path@agent-skills@docs/grounding/crate-boundaries.md` reads in the Cargo Book, a crate published on
crates.io cannot depend on a crate given only by a path, a dev-dependency excepted, and the resolver
refuses a dependency cycle outside dev-dependencies, and neither rule argues it:

- **A cycle.** A cycle that a split creates runs among the parts it cuts out: where a crate X is
  split, a part depending on another crate Y means X depended on Y, and Y depending on a part means
  Y depended on X, so a cycle through Y existed before the split and Cargo refused it. A split whose
  parts would depend on each other is, by `design@agent-skills@component-split-signals`, probably
  cut in the wrong place: its parts belong in one crate, or the split belongs on another axis, the
  owner's argument.
- **A publication.** A publication the project would not make for one part alone is the cost that
  `design@agent-skills@split-cost-by-publication` weighs as low. The owner reports that the rules,
  applied to this repository, proposed a number of crates the owner judged sane, with no Component
  as a module.

Every Component that holds Rust code has one shape, a crate, which the owner argued leaves a session
one shape fewer to weigh in a split and to search for in a project, against no measured gain of the
module shape. "One reason to change" is the test for a Component as for a function, so the criteria
of `goal@knowledge-architect@projects-are-split-into-components` and the default code shape of
`design@agent-skills@default-code-shape` read the same way at two scales.

### The cost of the crate a split creates is weighed by whether the project publishes `##split-cost-by-publication`

For a project that never publishes, the crate a split creates costs nothing, and its boundary is the
stronger isolation. For a project that publishes, or plans to, the owner argued that, if Rust
practice accepts many small published crates, more crates can probably be counted a low cost; the
practice does accept them, as gitoxide's crates read in
`path@agent-skills@docs/grounding/crate-boundaries.md` show. The cost is a publication of every crate a
published crate depends on, a release in dependency order and a version bump for each crate. The
skill reads the publication status from the project's record, asks the owner where the record
states none, and weighs the project's policy without setting it, per
`goal@knowledge-architect@agents-get-a-complete-workflow`.

### A split is weighed against what it costs between its parts `##component-split-signals`

The skill and the fragment state the same signals, which a session judges: a split that needs many
new dependencies between its parts, creates a dependency cycle, or has every expected change cross
its boundary is probably cut in the wrong place. The expected changes count those the goals already
predict, not only those of the history. The first two are the owner's argument that a split
multiplying dependencies or creating a cycle is cut wrongly. The third is the session's, derived from
the test of one reason to change of `goal@knowledge-architect@projects-are-split-into-components`: a
boundary every change crosses separates no reason to change. The rival the session weighed splits on
the default criteria alone, which reads two reasons to change in a unit and cuts it with no regard to
what the cut costs between the parts; no rival was offered in the discussion. The cost of the signals
is that they are judged, never computed, so two sessions can weigh one split differently. The rival
reading of the third signal, against the changes of the history alone, lost on its first use in real
use: the session that applied it counted past commits only, and kept in one Component two parts that
the goals predicted would change apart, as new consumers of one part needing nothing of the other.

### A principle on how the code of several Components is written, which they follow as one part of the project, is held by an intermediate Component that groups them `##intermediate-component-occasion`

An intermediate Component groups the Components nested under its directory and holds the design they
share as one part of the project: a decision that shapes how the code of each of them is written. Its
own files hold no Rust code, per `goal@knowledge-architect@projects-are-split-into-components`, so it
is no crate, and each Component nested under it that holds Rust code is a crate, per
`design@agent-skills@component-crate-mapping`. Three decisions shared by several Components are the
root's instead:

- **A decision about their shape, packaging or layout**, as which Components exist and how they
  depend on each other are, per `design@agent-skills@dependency-decision-home`, so that the layout
  is read in one place; the owner's condition.
- **A principle stated for every Component of a kind**: a property a Component has whatever part of
  the project it belongs to, and can gain or lose without moving, such as being published. Grouping
  the present members of a kind would bind the layout to the kind, every Component that gains it
  moving under the group.
- **A principle whose Components cross the set an existing intermediate Component groups.** A
  Component sits under one directory, so the sets intermediate Components group are each nested in,
  or apart from, every other; of two principles whose sets cross, at most one is held by an
  intermediate Component, and where both groupings survive the default criteria the choice is the
  owner's, under `skill@knowledge-architect-component-creation`. The owner raised the case.

In these three cases the root's design home is loaded by sessions working outside the Components a
decision binds, the cost an intermediate Component avoids; the session chose that cost over a layout
bound to a kind or to one of two crossing groupings, which no goal ranks, and the owner approved it.

Where several Components share a principle the rule sends to an intermediate Component and none
groups them, the session proposes creating one under `skill@knowledge-architect-component-creation`,
and asks the owner where the principle goes until one is created. A decision about one Component's
interface that the others consume is that Component's. The component-creation skill states the
occasion, and point 3 of the decision-recording skill's rule on the owning Component sends such a
decision there. How a reading reaches the intermediate's design is
`design@agent-skills@enclosing-design-read-at-grounding`.

The goal defines the intermediate Component by what it holds, and the owner proposed it as the one
occasion of grouping. The rivals lost:

- **Grouping by a count of Components beside one another.** A count gives the group no design to
  hold, while the group still owes every document and register home a Component carries, per
  `design@core@components-carry-the-same-documents`. A directory the manifest does not declare
  groups Components with no document, and moving a Component into it is one manifest edit, per
  `goal@core@relocation-is-one-manifest-edit`.
- **Every principle shared by a subset in the root's design home, its scope stated.** Every change
  to code reads the root's design home, per
  `skill@knowledge-architect-development@ground-before-editing`, so such a principle is loaded by
  every session working outside the Components it binds, against the reason of
  `goal@knowledge-architect@projects-are-split-into-components`, that each Component's design stays
  small enough to read whole. The three cases above keep that cost where an intermediate Component
  would bind the layout.
- **A shared principle left in the home of one member, or in the root's, at a session's judgement,
  an intermediate Component made only when a session sees fit.** The principle then sits in a home
  that no point of the rule on the owning Component gives it, unseen by every other member's
  reading. The rule gives it the intermediate Component as its home, so a session proposes the
  creation, and the owner places it only while the creation is pending.
- **An intermediate Component that is a crate whose nested Components are its modules.** Its crate
  root holds Rust code, a public interface, a dependency list and a version, which the goal's
  definition excludes; and every dependency of one nested Component reaches every consumer of the
  crate, the fact `rejected@knowledge-architect@git-module-made-public` lost on. The owner chose to
  keep the goal as it stands.
- **A dependency decision counted as shared design.** Every pair of dependent Components would call
  for an intermediate Component of its own, the owner's argument.

### That a Component is bound by the design of every intermediate Component enclosing it is stated once, in the primer, for every reading of a design home `##enclosing-design-read-at-grounding`

A reading of a Component's design home, before a change or in a discussion, includes the design
homes of the intermediate Components enclosing it, as it includes the root's. The primer's section
on the project's documents states it, in one paragraph. Without it, the design an intermediate
Component holds binds code that no reading reaches: none of the texts that list the design homes a
reading names, the primer's subsection on the record of code, the development skill's grounding,
the design skill's loop step 1, the planning skill's grounding of a slice and the design-conformance
reviewer, names an intermediate Component's. The development skill reads the root's because its
heads bind every Component or several of them, and an intermediate Component's heads bind its nested
Components the same way, per `design@agent-skills@intermediate-component-occasion`.

The session chose one paragraph of the primer over a clause in each of those five texts, which no
goal ranks; the owner approved it. The five clauses state one fact five times, and a sixth list
written later would miss it; the paragraph costs a place in every session's load, which
`design@agent-skills@primer-limit` admits for what every reading of a design home meets.

### A decision about how Components depend on each other belongs to the one Component it constrains, or to the root where it constrains more than one `##dependency-decision-home`

Which Components exist, and a decision that constrains the code of more than one Component, such as
one depending on another, are the root's design home's. A rule that one Component does not depend on
another constrains that Component's code alone, and is that Component's. Points 1 and 4 of the
decision-recording skill's rule on the owning Component state it, and the component-creation skill
restates it in one sentence. The test is what the decision constrains: a dependency of one
Component on another gives the second a consumer of its interface and bars it from depending on the
first, while a rule against a dependency leaves the other Component free, the owner's argument.

The rivals lost:

- **Every decision about dependencies in the root's design home.** A rule that binds one Component
  alone would be loaded by every session that reads the root's design home, per
  `skill@knowledge-architect-development@ground-before-editing`, outside the Component it binds,
  against the reason of `goal@knowledge-architect@projects-are-split-into-components`, that each
  Component's design stays small enough to read whole.
- **The lowest Component enclosing every Component the decision constrains.** It keeps the root
  smaller, and moves every dependency decision among a group into the intermediate Component made
  for it, with the references to them. The owner chose the root over it, which no goal ranks, so that
  every constraint crossing Components stays in one place, where the project's whole dependency
  constraints can be read.

### A shared test Component depends on the Components it tests, never the reverse, and holds the integrated tests that cross them, whose fixtures no unit test of a tested Component uses `##shared-test-component`

The skill offers it as a shape, not a rule: a Component that holds the integrated tests crossing
several Components, with their fixtures and oracles, and depends on them. An integrated test drives
the project through its public API, per `design@agent-skills@integrated-test-through-public-api`.
The tested Components' own unit tests use none of its fixtures, so no Component dev-depends on it
from below. It serves, among others, a crate A whose tests need a crate B that already depends on
A.
thaum's testing crate has this shape, as `path@agent-skills@docs/grounding/crate-boundaries.md`
records. The owner's argument is that it centralises the tooling for integrated testing, such as
fixtures and oracles, so that crates neither duplicate fixtures nor reach into another's; that
integrated tests moved out of the unit tests, and sometimes out of the crates, test the public API
rather than the internals; and that it avoids a dependency cycle even among dev-dependencies, since
such a cycle leaves unclear which crate is responsible for what. A condition that the crates share
their testing tools without depending on each other nearly always fails where a crate's own unit
tests use the fixtures; keeping the fixtures out of every unit test is what makes the shape
reachable.

Its nearest rival, a dev-dependency from A back onto B, is admitted by Cargo and fails in A's unit
tests, which link two copies of A whose types do not match, as the same grounding file reads in the
Cargo Book and measures. Helpers behind a feature of A serve A's own tests and no test that needs B.
