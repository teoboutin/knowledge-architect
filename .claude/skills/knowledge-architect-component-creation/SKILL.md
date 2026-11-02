---
name: knowledge-architect-component-creation
description: MUST use when new work may belong to no existing Component, when code being added gives a Component a second reason to change, when an adopting project's crates are mapped to Components at setup, and when an audit finds a Component holding two purposes or a head sitting in a Component that does not own its subject — creating a Component or splitting one. Covers the default criteria of a Component, the intermediate Component, the signals against a split, how Components map to crates, the shapes a Component can take, how a Component is named, and the procedure of a creation or a split. Not for the design discussion that decides it, the declaration in the manifest, or the goals, which their own skills cover.
---

# Creating or splitting a Component

Scope: deciding where a Component's boundary goes, and carrying out a creation or a split. One
activity, whether the Component is new or cut out of another: a split creates the Components it cuts
out.

Not covered here: **the design discussion** that decides a creation or a split,
`skill@knowledge-architect-design`; **declaring a Component** in the manifest, its documents and its
registers, `skill@knowledge-architect-setup`; **writing its goals**,
`skill@knowledge-architect-goal-setting`; **recording the decision**,
`skill@knowledge-architect-decision-recording`.

## What a Component is by default `##component-criteria`

These criteria are guidelines: a Component that departs from one does not fail on that alone. A
Component that holds Rust code in its own files, by default:

- **serves one purpose**, which every goal of its goals home serves;
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

**Commands, subcommands or functions that share one purpose and one design are one Component**,
each command or subcommand in a module of its own: the test of two changes is applied to the
Component, not to each command, since a change of their shared design touches every one of them,
and a change to one command alone differs from the others only in subject and application.

**An intermediate Component** holds no Rust code in its own files. It groups nested Components and
holds the design and the documents several of them share but the rest of the project does not.
Which Components exist and how they depend on each other is a decision about the layout, which
belongs in the root's design home, as
`skill@knowledge-architect-decision-recording@owning-component` says.

A project with one purpose and one Component needs no split.

## Signals against a split `##split-signals`

A split that would show any of these is probably cut in the wrong place:

- it needs many new dependencies between the parts;
- it would create a dependency cycle;
- every change you expect to make crosses the new boundary, counting
  the changes the goals already predict, not only those of the history.

Where many Components would sit beside one another, nested Components under an intermediate
Component carry the layering.

## Components and crates `##component-crates`

**One Component is one crate by default.** How strong that default is depends on whether the
project publishes:

| the project | one Component per crate | the cost of more crates |
| --- | --- | --- |
| publishes, or plans to | the default | a low cost, since Rust practice accepts many small published crates: each crate is released in dependency order and bumped on its own. A minor weight, a lead in one direction: a crate that others could use alone is an argument for the split; a crate no one would use alone is no argument against it |
| never publishes | a stronger default: the crate boundary is the stronger isolation | none |

Read whether the project publishes from its record, its goals and its design. Where the record says
neither, it is a plan only the owner holds: ask, as `primer@question-authority` says. This weighs
the project's policy on publishing; it never sets it.

**A Component may be a module directory inside a crate** where a crate of its own would force a
publication or a dependency cycle: a crate published on crates.io cannot depend on a crate given
only by a path, except as a dev-dependency, and Cargo refuses a dependency cycle outside
dev-dependencies. **Where the cycle comes only from a decision of the project's record**, such as
where a binary lives, that decision is reopened first under `skill@knowledge-architect-design`: the
exception holds only for a cycle that Cargo's rules force whatever the project decides. The module's
directory is the Component's directory, and carries its documents; whether the published crate ships
them is the project's packaging choice.

## Shapes beside one crate `##component-shapes`

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
- **A Component that supports extensions publishes the tools an extension's tests need**, its test
  harness, as a module of that Component or as a crate of its own, the project's choice. The
  fixtures of the shared test Component stay outside every published package.
- **A top-level binary crate**: where several Components contribute commands to one binary, the
  binary and its interface sit in a crate above them, which parses the arguments, applies the
  checks that can be made while parsing, and calls the logic the Components expose as a typed API.
- **A command's arguments are parsed in the crate nearest the binaries among those that every
  binary offering the command depends on**, so a library whose commands are flattened into each
  adopting project's own binary keeps its own parsing.
- **The pure and output layers.** The default code shape of
  `skill@knowledge-architect-development@discriminating-tests` keeps logic and side effects in two
  layers; at the scale of a Component, both sit by default in the Component that returns or defines
  the data. The output side is a Component of its own only where it takes
  decisions of its own, as a graphical interface that decides a layout does.

## Naming `##component-naming`

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

## Creating or splitting `##creation-procedure`

1. **Decide it in a design discussion**, `skill@knowledge-architect-design`, with its search of the
   record. Who rules it is `primer@question-authority`: a creation or a split that the criteria and
   the record settle is argued and reported; where two cuts both survive, the choice is the owner's.
   At setup, the setup skill's proposal of the Components to the owner takes the place of this step.
   A Component kept against the default criteria is a decision about the layout, recorded in the
   root's design home.
2. **Create it**, named as `skill@knowledge-architect-component-creation@component-naming` says, by
   `skill@knowledge-architect-setup@components-and-locations`.
3. **Write its purpose and its goals** under `skill@knowledge-architect-goal-setting`, the owner
   ruling the purpose and each goal. A split's goals moved from the Component it leaves are goals of the new
   Component, ruled the same way.
4. **For a split, move the record that is now the new Component's**: each design head, `rejected`
   entry, issue and tripwire whose subject it holds, the Component that owns each decided by
   `skill@knowledge-architect-decision-recording@owning-component`. A move adds and removes no
   decision. `cargo klarch check` then reports every reference into a moved entry or file that it
   reads, since it names the old anchor, and each is re-pointed; a reference in a file the checker
   does not read is found by a search.
