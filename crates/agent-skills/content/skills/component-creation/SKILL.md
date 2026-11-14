---
name: knowledge-architect-component-creation
description: MUST use when new work may belong to no existing Component, when code being added gives a Component a second reason to change, when the code-split scout reports a signal for a Component's split, when an adopting project's crates are mapped to Components at setup, when an audit finds a Component holding two purposes or a head sitting in a Component that does not own its subject — creating a Component or splitting one. Covers the default criteria of a Component, the intermediate Component, the signals against a split, how Components map to crates, the shapes a Component can take, how a Component is named, and the procedure of a creation or a split. Not for the design discussion that decides it, the declaration in the manifest, or the goals, which their own skills cover.
---

%% The activity and its moments are `design@agent-skills@component-skill-delivery`. The criteria
%% are guidelines, never a rule a Component fails alone, per
%% `goal@projects-are-split-into-modular-units`.
# Creating or splitting a Component

Scope: deciding where a Component's boundary goes, and carrying out a creation or a split. One
activity, whether the Component is new or cut out of another: a split creates the Components it cuts
out.

Not covered here: **the design discussion** that decides a creation or a split,
`skill@knowledge-architect-design`; **declaring a Component** in the manifest, its documents and its
registers, `skill@knowledge-architect-setup`; **recording the decision**,
`skill@knowledge-architect-decision-recording`.

## What a Component is by default {{slug:component-criteria}}

%% The criteria are guidelines that real use corrects, through the retrospective's question, per
%% `design@agent-skills@component-skill-delivery`.
%% The domain criterion is stated here alone, so no head: the owner's argument that a Component takes
%% decisions about one main matter, with smaller side ones allowed; the word is "domain" because a
%% subject is a part of a design home, per `design@agent-skills@subject-split-standard`.
These criteria are guidelines: a Component that departs from one does not fail on that alone. A
Component that holds Rust code in its own files, by default:

- **serves one purpose**, which the design head that argues its existence states;
- **has an interface with a reason to change on its own**: one reason to change is the test for a
  Component as for a function, read the same way, as two concrete changes someone could plausibly
  ask for, each touching it independently;
- **has its own tests**;
- **takes decisions of its own about one main domain**, what its decisions are about, such as the
  layout of a rendered site or the command line, with small side domains allowed, so it has a
  design of its own. One domain can span several subjects of the Component's design home. A
  Component that takes no decision has no design to record, which is the sign it is a part of
  another Component.

Such a Component is, for example, a published package, a service or a maintenance tool. The root,
which is the whole project, and an intermediate Component are outside these criteria.

%% Stated here alone, so no head: the owner's argument that subcommands sharing most of their design
%% and one purpose differ only in subject and application, and the session's, that they change
%% together where that design changes; its rival, the test of two changes applied to each command,
%% passes for every command and splits a design that changes as one.
**Commands, subcommands or functions that share one purpose and one design are one Component**,
each command or subcommand in a module of its own: the test of two changes is applied to the
Component, not to each command, since a change of their shared design touches every one of them,
and a change to one command alone differs from the others only in subject and application.

%% What an intermediate Component is and holds is
%% `design@agent-skills@shared-code-decisions-go-to-the-lowest-enclosing-component`; no occasion on
%% which one is created is stated, per
%% `design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists`; and the home of
%% a decision about dependencies is `design@agent-skills@dependency-decision-home`.
**An intermediate Component** groups the Components nested under its directory. Its own files hold no Rust code, so it is no crate: each Component nested under it that
holds Rust code is a crate of its own. A decision that shapes how the code of several Components is
written belongs in the design home of the lowest intermediate Component enclosing them all, or in
the root's where none encloses them all, as `skill@knowledge-architect-decision-recording@owning-component` says; a
principle stated for every Component of a kind, such as every crate the project publishes, and a
decision about their shape, packaging or layout, are the root's.

Which Components exist, and a decision about how they depend on each other that constrains more than
one of them, belong in the root's design home; a rule that one Component does not depend on another
is that Component's, as `skill@knowledge-architect-decision-recording@owning-component` says.

A project with one purpose and one Component needs no split.

## Signals against a split {{slug:split-signals}}

%% The signals are `design@agent-skills@component-split-signals`.
A split that would show any of these is probably cut in the wrong place:

- it needs many new dependencies between the parts;
- it would create a dependency cycle;
- every change you expect to make crosses the new boundary, counting
  the changes the goals already predict, not only those of the history.

## Components and crates {{slug:component-crates}}

%% The mapping is `design@agent-skills@component-crate-mapping`, and the shape it does not keep,
%% `rejected@agent-skills@module-directory-component`; the weighing of the cost is
%% `design@agent-skills@split-cost-by-publication`; the Cargo facts are read in
%% `path@agent-skills@docs/grounding/crate-boundaries.md`.
**A Component that holds Rust code in its own files is one crate**, so a split creates one crate for
each Component it cuts out. What that crate costs depends on whether the project publishes:

%% The weight of a crate's use by others is stated here alone, so no head: the owner argued that it
%% is little more than a lead and pushes in one direction only, and the session added that, read
%% both ways, it would keep a Component inside another on a ground no default criterion asks; the
%% owner chose to call it minor over a weight equal to the default criteria, which no goal ranks.
| the project | the cost of the crate a split creates |
| --- | --- |
| publishes, or plans to | a low cost, since Rust practice accepts many small published crates: every crate a published crate depends on is published too, since crates.io refuses a dependency given only by a path, a dev-dependency excepted, and each is released in dependency order and bumped on its own. A minor weight, in one direction: a crate that others could use alone is an argument for the split; a crate no one would use alone is no argument against it |
| never publishes | none, and the crate boundary is the stronger isolation |

Read whether the project publishes from its record, its goals and its design. Where the record says
neither, it is a plan only the owner holds: ask, as `primer@question-authority` says. This weighs
the project's policy on publishing; it never sets it.

## Shapes beside one crate {{slug:component-shapes}}

%% The shared test Component is `design@agent-skills@shared-test-component`.
- **A shared test Component**, offered, not required: a Component that depends on the Components it
  tests, never the reverse, and holds the integrated tests that cross them, tests that drive the
  project through its public API as a consumer would, as
  `skill@knowledge-architect-development@discriminating-tests` says, with their fixtures and
  oracles. The tested Components' own unit tests use none of those fixtures, so no Component
  dev-depends on the shared test Component. It serves, among others, a crate A whose tests need a
  crate B that already depends on A. A cycle is avoided even among dev-dependencies, since it leaves
  unclear which crate is responsible for what; and a dev-dependency from A back onto B builds A
  twice in A's unit tests, where the types of the two copies do not match and the errors read as
  mismatched types.
%% Stated here alone, so no head: the owner's argument that the mocking tools must be published
%% because the project supports writing extensions, whether as a module or as a crate of their own.
- **A Component that supports extensions publishes the tools an extension's tests need**, its test
  harness, as a module of that Component or as a crate of its own, the project's choice. The
  fixtures of the shared test Component stay outside every published package.
%% The argument and the rival are `design@agent-skills@top-level-binary-crate`.
- **A top-level binary crate**: where several Components contribute commands to one binary, the
  binary and its interface sit in a crate above them, which parses the arguments, applies the
  checks that can be made while parsing, and calls the logic the Components expose as a typed API.
%% The argument and the rival are `design@agent-skills@command-parsed-nearest-the-binaries`.
- **A command's arguments are parsed in the crate nearest the binaries among those that every
  binary offering the command depends on**, so a library whose commands are flattened into each
  adopting project's own binary keeps its own parsing.
%% Stated here alone, so no head: the owner observed that separating pure logic from routines with
%% side effects is very good for a project's long-term health, and narrowed it on the owner's word
%% that the printing should probably stay in the Component that returns or defines the data, since
%% an output Component that takes no decision would have no design of its own. No rival was offered.
- **The pure and output layers.** The default code shape of
  `skill@knowledge-architect-development@discriminating-tests` keeps logic and side effects in two
  layers; at the scale of a Component, both sit by default in the Component that returns or defines
  the data. The output side is a Component of its own only where it takes
  decisions of its own, as a graphical interface that decides a layout does.

## Naming {{slug:component-naming}}

%% Stated here alone, so no head. The first rule is forced by the checker, per
%% `design@knowledge-architect@no-component-directory-named-after-the-project`, whose rival, a
%% directory named after its package, is `rejected@knowledge-architect@crate-directory-named-after-project`;
%% the prefix is the owner's recommendation, for short anchors in references; the refusals are
%% `design@model@anchor-names-unique`, `design@model@kind-names-refused-as-anchors` and
%% `design@model@reserved-anchors`, and the basename sentence is stated before any finding can show,
%% on the owner's word to forbid it and document it.
Name a Component before its directory is created:

- **Name its directory by what it does, never after the project.** The checker refuses a Component
  named like the project, since the root's anchor already carries that name.
- **Give the package name in `Cargo.toml` a prefix the project chooses for its packages**, apart from
  the prefix of its own skills. A reference names a Component by its directory's basename, so the
  directory name stays short.
- **Two Components never share a basename.** The checker refuses two anchors with one name, as it
  refuses an anchor named like a kind of reference or like a word it reserves: a reference names its
  anchor by that one word. Two Components with one basename are one Component, or are not named
  specifically enough.

## Creating or splitting {{slug:creation-procedure}}

1. **Decide it in a design discussion**, `skill@knowledge-architect-design`, with its search of the
   record. Who rules it is `primer@question-authority`: a creation or a split that the criteria and
   the record settle is argued and reported; where two cuts both survive, the choice is the owner's.
   At setup, the setup skill's proposal of the Components to the owner takes the place of this step.
   A Component kept against the default criteria is a decision about the layout, recorded in the
   root's design home.
2. **Create it**, named as `skill@knowledge-architect-component-creation@component-naming` says, by
   `skill@knowledge-architect-setup@components-and-locations`.
3. **Record its purpose** as the statement of the design head that argues its existence, in the
   root's design home, under `skill@knowledge-architect-decision-recording`. No goal is written
   for it: the goals are the project's, as `skill@knowledge-architect-goal-setting` says.
4. **For a split, move the record that is now the new Component's**: each design head, `rejected`
   entry, issue and tripwire whose subject it holds, the Component that owns each decided by
   `skill@knowledge-architect-decision-recording@owning-component`. A move adds and removes no
   decision. `{{command}} check` then reports every reference into a moved entry or file that it
   reads, since it names the old anchor, and each is re-pointed; a reference in a file the checker
   does not read is found by a search.
