# Model design: The changelogs

## A crate's changelog and the assembled file

### The changelog crates are the Components `[changelog]` lists, or else every Component whose `Cargo.toml` Cargo would publish `##changelog-crates-are-listed-or-published`

Where the manifest declares `[changelog]`, per `design@check@changelog-opt-in`, the changelog crates
are the Components its `components` key lists, in the manifest's order of the Components. Where it
lists none, they are every Component whose `Cargo.toml` has a `[package]` table that Cargo would
publish: a `version` set, its own or inherited, and `publish` absent, `true`, or a list naming a
registry. That reading of Cargo's `version` and `publish`, an empty list read as `false`, is
`path@cargo-reader@docs/grounding/cargo-manifest.md`, section "Which package Cargo would publish". An
inherited field is resolved from the nearest `Cargo.toml` at or above the Component's directory,
within the project, that holds a `[workspace]` table, per
`design@cargo-reader@the-caller-finds-the-workspace-manifest`, and the manifest is read through
`cargo-reader`, per `design@knowledge-architect@tools-read-cargo-through-the-crate`.

**The published crates are a list each `Cargo.toml` already declares.** A list written by hand in
the manifest beside it drifts, and the drift it hides, a new published crate with no changelog, is
what the changelog checks exist to report. Reading `publish` discovers nothing by filename: it picks
among the Components the manifest declares, by what each one's `Cargo.toml` holds, per
`design@model@components-carry-the-same-documents`.

### The changelog crates are read where the model is built, before the walk `##changelog-crates-read-before-the-walk`

The walk leaves out every path of the generated list, per
`design@model@one-list-of-generated-files`, and whether the assembled changelog is on that list
depends on how many changelog crates there are, per `design@model@assembled-changelog-location`,
which each Component's `Cargo.toml` decides where `[changelog] components` lists none. So each
`Cargo.toml` is read where the model is built, from the tree the model is built from: the disk for
the working tree, a snapshot's blobs for `check --staged` and `commits`, so every run over one tree
reads one set of changelog crates. The rival, reading each `Cargo.toml` from the files the survey
hands over after the walk, would decide the walk from what the walk produces.

### A changelog entry is one list item that tags its category and its class, under no category heading `##changelog-entry-categories`

A crate's changelog is its preamble, per `design@check@category-order-in-preamble`; then
`## Next release`, which may be absent or empty; then one `## <version>` per released version, a
version being three numbers joined by `.`, in descending order. A version heading is a line markdown
reads as a level-two heading, per `design@model@headings-open-with-hash-marks`, so a `## ` line in a
fenced block of an entry's text is text. An entry is one list item, `- <category>, <class>: <text>`,
its text continuing on the lines indented under it; in a version that `[changelog] unclassed` lists,
`- <text>`, with no category and no class. Every other line below the first version heading is a line
the parse records as unread, raising no finding, per `design@knowledge-architect@model-crate`.

The classes are patch, minor and major, and the grammar and the classes are compiled in, per
`design@check@nothing-of-a-project-is-compiled-in`. The categories are the project's,
`[changelog] categories`, in the order the assembled changelog gives them. A category holds no `,`
and no `:`, which the manifest's resolve refuses, since the tag of an entry of that category would
read back as another category or another class. `[changelog.category-components]` restricts a
category to the Components it lists; a category it does not name is open to every changelog crate.
The test each category applies, such as what earns a Migration entry, stays in the workflow's text,
since no mechanical check can apply it.

The nearest rival, a category heading in each crate's file, loses because a crate with three
entries would carry three headings, and a category that one crate alone uses
would exist in one file, as Workflow would in this repository.

### A crate's changelog holds a section only for a version with an entry in that crate `##absent-version-section`

A released version with no entry in a crate has no heading in that crate's file, and a version
heading of a released version with no entry is a finding. Under the lockstep, the bump check reads the versions every
crate's file holds together, per `design@check@changelog-bump-check`, so a heading with no entry
adds nothing a check or a reader uses. The rival, every version's heading in every crate's file,
adds an empty heading to each file a release does not change.

### The assembled changelog is at the path `[changelog] assembled` names, and only a project with two changelog crates or more has one `##assembled-changelog-location`

The assembled changelog, the one changelog of the whole project, is generated at the path
`[changelog] assembled` names, `CHANGELOG.md` at the project's root where the key is absent. The path
is spelled as git lists it and read as a `[walk]` row is, per `design@model@git-supplies-the-walk`.
With one changelog crate, nothing is assembled, whatever the key: that crate's changelog is the
project's. An assembled path that is a changelog crate's own `CHANGELOG.md` is a finding, since the
generated file would take the place of the crate's hand-written one, against
`design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`. No crate ships the
assembled changelog, which is committed for the readers of the repository.

The nearest rival, the assembled changelog placed by Component, covers neither the project whose root
is no published crate, as this repository's is, nor the project with one crate, whose file is that
crate's; a path covers both.

### The assembled changelog groups each version's entries by category, then by crate in the manifest's order `##assembled-changelog-layout`

The assembled changelog opens with a generated header, as an `index.md` does; then each version,
`Next release` first, as `## <version>`; under it each category in its declared order, as
`### <category>`; under it each changelog crate in the manifest's order of the Components, as
`#### <package name>`, the published name; under it the crate's entries of that category, each
`- <class>: <text>`, in the order of the crate's file. A version that `unclassed` lists has no
category level: `#### <package name>`, then `- <text>`. An empty level is omitted. The rendering is a
pure function of the parsed changelogs and the manifest, which raises no finding.

**The category comes before the crate because a reader moving a pin acts on a version's Migration
entries as one block.** The nearest rival, version then crate then category, scatters those entries
over every crate. The crates follow the manifest's order, a choice the project already makes and
which needs no new option; the rival, the alphabetical order of the package names, puts the binary
first only by chance. Package names over the Components' names, which no goal ranks: the owner's
ranking.

### Every writer of the assembled changelog refuses, writing nothing, over a file at its path without the generated header and while a changelog holds a line the file would omit `##assembled-file-not-overwritten`

`cargo klarch index`, `cargo klarch index --staged` and `cargo klarch check --fix` each refuse to write
the assembled changelog, and write no byte of any generated file, where:

- a file at its path, in the tree the writer writes, does not open with the generated header, since
  that file is what the project wrote, per
  `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`; `index --staged`
  reads it from the staged blob;
- a changelog holds a line the parse could not read, or an entry whose category the manifest does
  not declare for its crate, an entry with no tag in a classed version among them, since the
  assembled changelog has no place for either and would omit it, per
  `design@check@fix-before-the-checks`.

Each refusal names its repair. The three writers refuse by one function of `model`, over the parsed
changelogs and the texts the gathering already reads of each generated path, so no writer refuses
what another would write over.
