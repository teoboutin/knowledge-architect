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
That shape rested on two rules of Cargo that `path@agent-skills@docs/grounding/crate-boundaries.md`
reads in the Cargo Book, a crate published on crates.io cannot depend on a crate given only by a
path, a dev-dependency excepted, and the resolver refuses a dependency cycle outside
dev-dependencies, and neither rule argues it. A part that a crate of its own would set in a cycle
with the rest of its crate is a split that `design@agent-skills@component-split-signals` calls cut
in the wrong place: its parts belong in one crate, or the split belongs on another axis. A
publication the project would not make for one part alone is the cost that
`design@agent-skills@split-cost-by-publication` weighs as low, and the owner reports that the rules,
applied to this repository before any revision, proposed a number of crates the owner judged sane,
and that this repository needed no Component as a module. Every Component that holds Rust code then
has one shape, a crate, which the owner argued leaves a session one shape fewer to weigh in a split
and to search for in a project, against no measured gain of the module shape. "One reason to
change" is the test for a Component as for a function, so the criteria of
`goal@knowledge-architect@projects-are-split-into-components` and the default code shape of
`design@agent-skills@default-code-shape` read the same way at two scales.

### The cost of the crate a split creates is weighed by whether the project publishes, and its use by others weighs only for the split `##split-cost-by-publication`

For a project that never publishes, the crate a split creates costs nothing, and its boundary is the
stronger isolation. For a project that publishes, or plans to, the owner argued that, if Rust
practice accepts many small published crates, more crates can probably be counted a low cost; the
session reported that the practice does accept them. The cost is a publication of every crate a
published crate depends on, a release in dependency order and a version bump for each crate. How
usable a crate would be alone for others weighs in one direction only: a crate others could use
alone argues for the split, and a crate no one would use alone argues nothing against it. The owner
argued that the weight is little more than a lead, and should push in one direction only; read both
ways, the session added, it would keep a Component inside another on a ground no criterion of
`goal@knowledge-architect@projects-are-split-into-components` asks. The owner chose to call the
weight minor over a weight equal to the default criteria, which no goal ranks. The skill reads the
publication status from the project's record, asks the owner where the record states none, and
weighs the project's policy without setting it, per
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

### A principle that each of several Components follows, and not every Component, is held by an intermediate Component that groups them, which holds no code `##intermediate-component-occasion`

An intermediate Component groups the Components nested under its directory and holds the design they
share and the rest of the project does not: a decision that shapes how each of them is implemented
and not how every Component is. Its own files hold no Rust code, per
`goal@knowledge-architect@projects-are-split-into-components`, so it is no crate, and each Component
nested under it that holds Rust code is a crate, per `design@agent-skills@component-crate-mapping`.
Where several Components share such a principle and none groups them, creating one is a creation
`skill@knowledge-architect-component-creation` decides. A decision about one Component's interface
that the others consume is that Component's, and a decision about how Components depend on each
other is placed by `design@agent-skills@dependency-decision-home`: neither makes an intermediate
Component. The component-creation skill states the occasion, point 3 of the decision-recording
skill's rule on the owning Component sends such a decision there, and the primer states that a
Component is bound by the design of every Component enclosing it, so that a reading of a Component's
design home includes the design homes above it.

The goal defines the intermediate Component by what it holds, and the owner proposed it as the one
occasion of grouping. The rivals lost:

- **Grouping by a count of Components beside one another.** A count gives the group no design to
  hold, while the group still owes every document and register home a Component carries, per
  `design@core@components-carry-the-same-documents`. A directory the manifest does not declare
  groups Components with no document, and moving a Component into it is one manifest edit, per
  `goal@core@relocation-is-one-manifest-edit`.
- **The principle in the root's design home, its scope stated.** Every change to code reads the
  root's design home, per `skill@knowledge-architect-development@ground-before-editing`, so the principle would
  be loaded by every session working outside the Components it binds, against the reason of the goal
  above, that each Component's design stays small enough to read whole.
- **An intermediate Component made at a session's judgement, the principle placed elsewhere
  meanwhile.** No point of the rule on the owning Component gives the principle a home until one is
  made.
- **An intermediate Component that is a crate whose nested Components are its modules.** Its crate
  root holds Rust code, a public interface, a dependency list and a version, which the goal's
  definition excludes; and every dependency of one nested Component reaches every consumer of the
  crate. The owner chose to keep the goal as it stands.
- **A dependency decision counted as shared design.** Every pair of dependent Components would call
  for an intermediate Component of its own, the owner's argument.

The binding is stated once, in the primer's section on the project's documents, rather than in each
of the five texts that list the design homes a reading names: a sixth list written later would miss
a fact that five copies state. It costs one paragraph of the primer in every session, which the
owner accepted against the five copies.

### A decision about how Components depend on each other is the root's where it constrains more than one of them, and a rule that one Component does not depend on another is that Component's `##dependency-decision-home`

Which Components exist, and a decision that constrains the code of more than one Component, such as
one depending on another, are the root's design home's. A rule that one Component does not depend
on another constrains that Component's code alone, and is that Component's. Points 1 and 4 of the
decision-recording skill's rule on the owning Component state it, and the component-creation skill
restates it in one sentence. The test is what the decision constrains: a dependency of one Component
on another gives the second a consumer of its interface and bars it from depending on the first,
while a rule against a dependency leaves the other Component free, the owner's argument.

The rivals lost:

- **Every decision about dependencies in the root's design home.** A rule that binds one Component
  alone would be loaded by every session that reads the root's design home, per
  `skill@knowledge-architect-development@ground-before-editing`, outside the Component it binds.
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
