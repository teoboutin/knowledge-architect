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

### One Component is one crate by default, weighed by whether the project publishes, and a Component may be a module directory where Cargo's rules, not a decision of the project, make a crate of its own force a publication or a cycle `##component-crate-mapping`

The default holds more strongly for a project that never publishes, since a crate boundary is the
stronger isolation and more crates cost it nothing. For a project that publishes, or plans to, the
owner argued that, if Rust practice accepts many small published crates, more crates can probably
be counted a low cost; the session reported that the practice does accept them. The cost is a
release in dependency order and a version bump for each crate. How usable a crate would be alone
for others weighs in one direction only: a crate others could use alone argues for the split, and a
crate no one would use alone argues nothing against it, on the owner's argument that the weight is
mostly a lead, and that read both ways it would keep a Component inside another on a ground no
criterion of `goal@knowledge-architect@projects-are-split-into-components` asks. The owner chose
to call the weight minor over a weight equal to the default criteria, which no goal ranks. The skill
reads the publication status from the project's record, asks the owner where the record states
none, and weighs the project's policy without setting it, per `goal@knowledge-architect@agents-get-a-complete-workflow`. "One reason to
change" is the test for a Component as for a function, so the criteria of
`goal@knowledge-architect@projects-are-split-into-components` and the default code shape of
`design@agent-skills@default-code-shape` read the same way at two scales.

The exception, a module directory inside a crate, rests on two rules of Cargo that
`path@agent-skills@docs/grounding/crate-boundaries.md` reads in the Cargo Book: a crate published on
crates.io cannot depend on a crate given only by a path, a dev-dependency excepted, and the resolver
refuses a dependency cycle outside dev-dependencies. Its nearest rival, a crate for every Component
always, would force a publication of an internal part, or a cycle Cargo refuses. A publication or a
cycle that comes only from a decision of the project's record, such as where a binary lives, is no
such rule: the skill sends the session to reopen that decision under
`skill@knowledge-architect-design` first, since the decision can be reversed and its reversal gives
the Component a crate of its own, where the exception applied to every cycle would keep a decision
whose premise may have failed by folding a Component into a module.

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

### A shared test Component depends on the Components it tests, never the reverse, and holds the integrated tests that cross them, whose fixtures no unit test of a tested Component uses `##shared-test-component`

The skill offers it as a shape, not a rule: a Component that holds the integrated tests crossing
several Components, with their fixtures and oracles, and depends on them. An integrated test drives
the project through its public API, per `design@agent-skills@integrated-test-through-public-api`.
The tested Components' own unit tests use none of its fixtures, so no Component dev-depends on it
from below; a crate A whose tests need a crate B that already depends on A is one case of it.
thaum's testing crate has this shape, as `path@agent-skills@docs/grounding/crate-boundaries.md`
records. The owner's argument is that it centralises the tooling for integrated testing, such as
fixtures and oracles, so that crates neither duplicate fixtures nor reach into another's; that
integrated tests moved out of the unit tests, and out of the crates, test the public API rather
than the internals; and that it avoids a dependency cycle even among dev-dependencies, since such a
cycle leaves unclear which crate is responsible for what. A condition that the crates share their
testing tools without depending on each other nearly always fails where a crate's own unit tests
use the fixtures; keeping the fixtures out of every unit test is what makes the shape reachable.

Its nearest rival, a dev-dependency from A back onto B, is admitted by Cargo and fails in A's unit
tests, which link two copies of A whose types do not match, as the same grounding file reads in the
Cargo Book and measures. Helpers behind a feature of A serve A's own tests and no test that needs B.

### A binary that several Components contribute commands to sits in a top-level crate above them, and a command's arguments are parsed in the lowest crate every binary offering it depends on `##top-level-binary-crate`

The top-level crate holds the binary and its interface: it parses the arguments, applies the checks
that can be made while parsing, and calls the logic the Components expose as a typed API. The owner
argued that grouping the parsing, with the checks that can be applied at parsing, is the more
convenient shape, while the application logic is pulled from the crates below. The session added
that parsing in one crate gives the decisions about the whole interface one home, such as an exit
code or a flag shared across commands; keeps the parser's library out of the domain crates'
interface; and gives a frontend that is no command line a typed API to call. A library whose
commands are flattened into each adopting project's own binary has no shared crate above it, so it
keeps its own parsing: parsed anywhere else, each binary would declare its flags again, and the
copies would drift. `skill@knowledge-architect-component-creation` states the shape, per
`goal@knowledge-architect@projects-are-split-into-components`.

Two rivals lost. The binary inside one of the domain crates makes that crate's design hold the
interface's decisions beside its own domain, and makes every other contributing crate depend on it
or below it, a cycle that comes from the decision alone. Each Component parsing its own commands,
the binary flattening them, restates the interface's decisions in each domain crate and puts the
parser's library in each one's interface; it stays the shape of a library flattened into binaries
that no shared crate sits above.
