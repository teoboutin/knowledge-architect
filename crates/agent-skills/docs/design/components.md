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

`goal@knowledge-architect@projects-are-split-into-modular-units` has the workflow lead a target
project toward units that each serve one purpose, the project's parts among them, as a direction and
not a rule a unit fails alone. So the installed text states default criteria of a Component, as
guidelines a Component that departs from one does not fail on that alone, and leads a session to
apply them at each of the moments above. What makes a split good is learned from use rather than
fixed before it: the criteria are corrected by what the retrospective's question on a rule about
splitting that proved too hard or too strict brings back, per
`design@agent-skills@premortem-as-watch-points`, and the changes real use asks of the workflow, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`. The nearest rival, a rule of
splitting each Component must meet, fixes before any use what the goal leaves to a direction, and
leaves a session whose project the rule fits badly no move but to break it. The activity's methodology, a cut of a boundary followed by a
move of the record, is apart from a design discussion's and from the setup's, so it is an activity
of its own and takes a skill of its own, per `design@agent-skills@one-skill-per-activity`;
`goal@knowledge-architect@agents-get-a-complete-workflow` covers each activity the owner has taken
in, and its list of activities grows. The owner
proposed a skill of its own with a fragment in the design skill, on the argument that not every design
discussion needs the procedure, since creating a Component should be occasional; the session
extended the proposal to the development skill and to the four moments. The skill decides nothing itself: the decision to create or split goes through
`skill@knowledge-architect-design`, and the skill points to the setup skill for the declaration and to the
decision-recording skill for the head that states the Component's purpose.

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
of `design@agent-skills@component-skill-delivery` and the default code shape of
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
the test of one reason to change of the default criteria, per
`design@agent-skills@component-skill-delivery`: a
boundary every change crosses separates no reason to change. The rival the session weighed splits on
the default criteria alone, which reads two reasons to change in a unit and cuts it with no regard to
what the cut costs between the parts; no rival was offered in the discussion. The cost of the signals
is that they are judged, never computed, so two sessions can weigh one split differently. The rival
reading of the third signal, against the changes of the history alone, lost on its first use in real
use: the session that applied it counted past commits only, and kept in one Component two parts that
the goals predicted would change apart, as new consumers of one part needing nothing of the other.

### Where several Components contribute commands to one binary, the binary and its interface sit in a crate above them `##top-level-binary-crate`

That crate parses the arguments, applies the checks that can be made while parsing, and calls the logic the Components
expose as a typed API. The owner argued that grouping the parsing, with the checks applied at parsing, is more
convenient, and that the logic is mostly pulled from below. The interface's decisions then have one home, the parser's
library stays out of the domain crates' interface, and a frontend that is no command line gets a typed API. The rival,
the binary inside a domain crate, puts the interface's decisions in that crate's design, and forms a cycle with every
crate that contributes a command and depends on it. It is stated by the component-creation skill.

### A command's arguments are parsed in the crate nearest the binaries among those every binary offering it depends on `##command-parsed-nearest-the-binaries`

So a library flattened into binaries no shared crate sits above keeps its own parsing: parsed anywhere else, each binary
would declare its flags again, and the copies would drift. The rival, each Component parsing its own commands wherever a
shared crate sits above them and the binary flattening them, restates the interface's decisions in each domain crate. It
is stated by the component-creation skill.

### An intermediate Component is a directory of Components with no code of its own, and a decision on how the code of several Components is written goes to the lowest one enclosing them `##shared-code-decisions-go-to-the-lowest-enclosing-component`

An intermediate Component groups the Components nested under its directory. Its own files hold no
Rust code, per `goal@knowledge-architect@projects-are-split-into-modular-units`, so it is no crate, and
each Component nested under it that holds Rust code is a crate, per
`design@agent-skills@component-crate-mapping`.
A decision that shapes how the code of several Components is written goes to the design home of the
lowest intermediate Component enclosing them all, or to the root's where none encloses them all.
Two decisions binding several Components are the root's whatever encloses them: a decision on their
shape, packaging or layout, as which Components exist and how they depend on each other, per
`design@agent-skills@dependency-decision-home`, on the owner's condition, which the session argued
by extending the owner's argument for dependency decisions, that the layout is then read in one
place, at the cost of loading it on sessions outside the Components it binds; and a principle stated
for every Component of a kind, a property a Component has whatever part of the project it belongs to and can gain or lose without moving, such as being published,
since a Component anywhere in the tree can gain it. The component-creation skill states what an
intermediate Component is, and point 3 of the decision-recording skill's rule on the owning
Component places the decision. How a reading reaches the intermediate's design is
`design@agent-skills@enclosing-design-read-at-grounding`. When one is created is
`design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists`.

The directories form a tree, so every set of Components has one lowest enclosing Component, and the
same decisions give the same homes in whatever order they were met. Its costs: a decision in the
lowest enclosing Component is loaded by the sessions of the other Components it encloses; and
moving a Component under or out of an intermediate Component moves the decisions whose lowest
enclosing Component changes.

The rivals lost:

- **No intermediate Component, every decision binding several Components in the root's design
  home.** It holds one shape fewer, and every principle shared by a subset is loaded by every
  session, since every change to code reads the root's design home, per
  `skill@knowledge-architect-development@ground-before-editing`, against the reason of
  `goal@knowledge-architect@projects-are-split-into-modular-units`, that each Component's design stays
  small enough to read whole. The owner chose over it the intermediate Component created for a
  purpose stated in goals, which no goal ranked, to be judged when a project first needs one. That
  occasion is no longer stated, and the choice was not put to the owner again without it.
- **An intermediate Component as a declared set of Components rather than a directory.**
  `goal@knowledge-architect@projects-are-split-into-modular-units` defines a Component as what the
  checker's manifest declares, a directory or the project's root, and the owner keeps each
  Component tied to the filesystem tree.
- **An intermediate Component that is a crate whose nested Components are its modules.** Its crate
  root holds Rust code, a public interface, a dependency list and a version, which the goal's
  definition excludes; and every dependency of one nested Component reaches every consumer of the
  crate, the fact `rejected@knowledge-architect@git-module-made-public` lost on. The owner chose to
  keep the goal as it stands.

### The installed text states no occasion on which an intermediate Component is created `##no-installed-rule-on-when-an-intermediate-component-exists`

The installed text says what an intermediate Component is and which decisions it holds, per
`design@agent-skills@shared-code-decisions-go-to-the-lowest-enclosing-component`, and names no
occasion on which a project creates one: a project argues one in its own design.

No occasion has an instance in use, and an instruction on how an agent behaves needs one, per
`design@agent-skills@behaviour-additions-need-real-use`. The rival, an occasion stated, has one
candidate with a ground in the record: a purpose the nested Components serve together, stated in
the intermediate Component's goals home and refined by their goals. It needs goals of a Component,
which `design@agent-skills@goals-home-at-the-root-only` excludes. The owner chose silence over a
guideline that no use supports, to learn from the first project that creates one.

### That a Component is bound by the design of every intermediate Component enclosing it is stated once, in the primer, for every reading of a design home `##enclosing-design-read-at-grounding`

A reading of a Component's design home, before a change or in a discussion, includes the design
homes of the intermediate Components enclosing it, as it includes the root's. The primer's section
on the project's documents states it, in one paragraph. Without it, the design an intermediate
Component holds binds code that no reading reaches: the texts that list the design homes a reading
names, the primer's subsection on the record of code, the development skill's grounding, the design
skill's loop step 1, the planning skill's grounding of a slice and the design-conformance reviewer,
name the design home of the Component touched, and some the root's, and none of them would otherwise
name an intermediate Component's. An intermediate Component's heads bind the Components nested under
it as the root's bind the Components they concern, per
`design@agent-skills@shared-code-decisions-go-to-the-lowest-enclosing-component`. The development skill, which names the
root's beside the Component's, restates the binding in one sentence with its pointer to the primer,
as `primer@where-knowledge-goes` admits.

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
  against the reason of `goal@knowledge-architect@projects-are-split-into-modular-units`, that each
  Component's design stays small enough to read whole.
- **The lowest Component enclosing every Component the decision constrains.** It keeps the root
  smaller, and moves every dependency decision among a group into the intermediate Component that
  encloses it, with the references to them. The owner chose the root over it, which no goal ranks, so that
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
