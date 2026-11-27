# Model design: The changelogs

## A crate's changelog and the assembled file

### The changelog crates are the Components `[changelog]` lists, or else every Component whose `Cargo.toml` Cargo would publish `##changelog-crates-are-listed-or-published`

Where the manifest declares `[changelog]`, per `design@check@changelog-opt-in`, the changelog crates
are the Components its `components` key lists, in the manifest's order of the Components. Where it
lists none, they are every Component whose `Cargo.toml` has a `[package]` table that Cargo would
publish: a `version` set, its own or inherited, and `publish` absent, `true`, or a list naming a
registry. That reading of Cargo's `version` and `publish`, an empty list read as `false`, is
`path@model@docs/grounding/cargo-packages.md`, section "Which package Cargo would publish". An
inherited field is resolved from the workspace Cargo uses for the package, per
`design@cargo-reader@the-caller-finds-the-workspace-manifest`: the `Cargo.toml` that the package's
`package.workspace` names, where set, and otherwise the nearest `Cargo.toml` at or above the
Component's directory, within the project, whose `[workspace]` has the package as a member, one
whose `[workspace]` excludes it being passed over, as `cargo metadata` resolves it. The manifest is
read through `cargo-reader`, per `design@knowledge-architect@tools-read-cargo-through-the-crate`.

**The published crates are a list each `Cargo.toml` already declares.** A list written by hand in
the manifest beside it drifts, and the drift it hides, a new published crate with no changelog, is
what the changelog checks exist to report. The `components` key stays beside it because a
changelog is not tied to publishing alone: the owner reported that a project may want a checked
changelog for a crate it does not publish, and judged that a crate published with no changelog on
purpose is rarer. The owner also judged its drift unlikely, since a Component is created rarely and
with strong consequences, so a review sees a list left behind. Reading `publish` discovers nothing by filename: it picks
among the Components the manifest declares, by what each one's `Cargo.toml` holds, per
`design@model@components-carry-the-same-documents`.

### The changelog crates are read where the model is built, before the walk `##changelog-crates-read-before-the-walk`

The walk leaves out every path of the generated list, per
`design@model@one-list-of-generated-files`, and whether the assembled changelog is on that list
depends on how many changelog crates there are, per `design@model@assembled-changelog-from-two-crates`,
which each Component's `Cargo.toml` decides where `[changelog] components` lists none. So each
`Cargo.toml` is read where the model is built, from the tree the model is built from: the disk for
the working tree, a snapshot's blobs for `check --staged` and `commits`, so every run over one tree
reads one set of changelog crates. The rival, reading each `Cargo.toml` from the files the survey
hands over after the walk, would decide the walk from what the walk produces.

### A changelog entry is one list item that tags its category and its class, under no category heading `##changelog-entry-categories`

A crate's changelog is its preamble, per `design@check@category-order-in-preamble`; then
`## Next release`, required and possibly empty; then one `## <version>` per released version, in
descending order. Its version headings are `## Next release` and each `## <version>`, so the
preamble ends at the first of them. A version heading is a line markdown reads as a level-two
heading, per `design@model@headings-open-with-hash-marks`, so a `## ` line in a fenced block of an
entry's text is text. An entry is one list item, `- <category>, <class>: <text>`, its text
continuing on the lines indented under it; in a version that `[changelog] unclassed` lists,
`- <text>`, with no category and no class. Every other line below the first version heading is a
line the parse records as unread, raising no finding, per `design@knowledge-architect@model-crate`.
A file with no `## Next release` is a finding of the grammar, which gives every crate's file the
section an entry of the next release is written in, at one place, empty while no entry is written.

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

### A version heading names a release version: three numbers with no leading zero, no pre-release and no build metadata `##changelog-version-is-a-release-version`

The `<version>` of a version heading, per `design@model@changelog-entry-categories`, is three
numbers joined by `.`, none with a leading zero. Cargo refuses a leading zero in a `version`, as
`path@check@docs/grounding/cargo-semver.md` measures it, so no package carries that version. A heading of a
pre-release version, such as `## 1.0.0-rc.1`, or of one with build metadata, such as
`## 1.0.0+build.5`, is a line the parse records as unread: the bump check has no reading of a
pre-release's class, since the Cargo book's semver chapter gives none, and build metadata is
ignored when versions are ordered, by item 10 of the SemVer specification Cargo follows, quoted in
`path@check@docs/grounding/cargo-semver.md`, so a
heading carrying it names no release the check could order apart from the same version without it.
While such a line stands the writers refuse, per `design@model@assembled-file-not-overwritten`, so
no entry is lost. The nearest rival, any version
Cargo accepts, would hand the bump check a version it cannot judge.

### A crate's changelog holds a section only for a version with an entry in that crate `##absent-version-section`

A released version with no entry in a crate has no heading in that crate's file, and a version
heading of a released version with no entry is a finding. Under the lockstep, the bump check reads the versions every
crate's file holds together, per `design@check@changelog-bump-check`, so a heading with no entry
adds nothing a check or a reader uses. The rival, every version's heading in every crate's file,
adds an empty heading to each file a release does not change.

### The assembled changelog is at the path `[changelog] assembled` names `##assembled-changelog-location`

The assembled changelog, the one changelog of the whole project, is generated at the path
`[changelog] assembled` names, `CHANGELOG.md` at the project's root where the key is absent. The path
is spelled as git lists it and read as a `[walk]` row is, per `design@model@git-supplies-the-walk`,
and it is committed for the readers of the repository.

The constraint derives from `goal@every-target-project-can-adopt-it`: what the checker provides
holds nothing particular to one target project, so the location fits every shape of project. The
nearest rival, the assembled changelog placed by Component, does not cover the project whose root
is no published crate, as this repository's is; a path covers it.

### An assembled path that its writers could not carry out is refused `##assembled-path-taken-by-no-other-file`

The path `[changelog] assembled` names, its default included, per
`design@model@assembled-changelog-location`, is refused where the writers could not act on it:

- an assembled path that is empty or `.`, which names no file; a path of the installer's
  namespace, whose files the install alone writes, per `design@check@owned-namespace-check`; or
  another generated path, a register's index or an extension's generated file, where each write of
  one of the two files would overwrite the other, so `--fix` would rewrite the file at every run and
  the finding over it would never clear. Each is a finding of the model, raised where the manifest
  is resolved or where the model lists the generated paths;
- an assembled path that is a changelog crate's own `CHANGELOG.md` is a finding, since the
  generated file would take the place of the crate's hand-written one, against
  `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`.

The nearest rival, a path accepted as declared, leaves a declaration that no writer can carry out:
a run that loops, a write into the install's files, or a hand-written changelog overwritten.

### Only a project with two changelog crates or more has an assembled changelog `##assembled-changelog-from-two-crates`

With one changelog crate, nothing is assembled, whatever `[changelog] assembled` says: that crate's
changelog is the project's. The path is then no generated path, and a file at it is walked as any
other, a file there that opens with the generated header being judged as
`design@check@generated-header-at-an-unassembled-path` says.

The rival, an assembled changelog for one crate, would hold that crate's entries alone, re-sorted:
a second file that says what the crate's own file says. At the default path, in a project whose one
crate is its root, it would take the place of that crate's own `CHANGELOG.md`.

### The assembled changelog groups each version's entries by category, then by crate `##assembled-changelog-layout`

The assembled changelog opens with a generated header, as an `index.md` does; then each version,
`Next release` first, as `## <version>`; under it each category in its declared order, as
`### <category>`; under it each changelog crate, in the order of
`design@model@assembled-crates-in-the-manifest-s-order`, as `#### <package name>`, per
`design@model@assembled-crates-named-by-package`; under it the crate's entries of that category,
each `- <class>: <text>`, in the order of the crate's file. A version that `unclassed` lists has no
category level: `#### <package name>`, then `- <text>`. An empty level is omitted. The rendering is a
pure function of the parsed changelogs and the manifest, which raises no finding. Where each crate
is placed by a table of its own, as a consumer's `klarch changelog` places the crates it reads, per
`design@query@changelog-places-each-crate-by-its-own-preamble`, the rendering is a pure function of
the parsed changelogs, their tables and the categories its caller gives, in the caller's order, and
a crate whose table lists the version as unclassed is given under it before its categories, since
it has no category to sit under.

**The category comes before the crate because a reader moving a pin acts on a version's Migration
entries as one block.** The nearest rival, version then crate then category, scatters those entries
over every crate. Its cost: the assembled file is a re-sort of the crates' entries, not their
concatenation.

### The assembled changelog gives a category's crates in the manifest's order of the Components `##assembled-crates-in-the-manifest-s-order`

Under each category of a version, and under a version that `unclassed` lists, the changelog crates
follow the order in which the manifest declares the Components. That order is a choice the project
already sets, so it needs no option of its own, where the project would have no say in the
alphabetical order. The nearest rival, the alphabetical order of the package
names, puts the binary first only by chance.

### The assembled changelog names each crate by the package name it is published under `##assembled-crates-named-by-package`

Each crate's heading in the assembled changelog is its package name, as its `Cargo.toml` gives it.
A consumer meets a crate by that name, in its own
`Cargo.toml`, on its registry and in its documentation, and the crate's own changelog names it so
in its preamble, per `design@check@category-order-in-preamble`. The nearest rival, the Component's
name, is a name of the project's own tree, which a consumer of the published crates needs nowhere
else.

### Every writer of the assembled changelog refuses, writing nothing, over a file at its path without the generated header and while the file would omit what a changelog crate holds `##assembled-file-not-overwritten`

`cargo klarch index`, `cargo klarch index --staged` and `cargo klarch check --fix` each refuse to write
the assembled changelog, and write no byte of any generated file, where:

- a file at its path, in the tree the writer writes, does not open with the generated header, since
  that file is what the project wrote, per
  `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`; `index --staged`
  reads it from the staged blob;
- a changelog holds a line the parse could not read, or an entry whose category the manifest does
  not declare for its crate, an entry with no tag in a classed version among them, since the
  assembled changelog has no place for either;
- a changelog's preamble holds a line that the preamble the manifest determines, per
  `design@check@category-order-in-preamble`, lacks, compared without the white space that ends a
  line, since the assembled changelog writes no preamble, and a line no manifest gives again is
  the project's own, as the entries under a misspelled version heading are, which the parse reads
  as preamble; a preamble that only lacks a line of the expected one holds nothing to omit, so it
  stops no writer, and its own finding stands;
- a changelog crate's files cannot be read: its `CHANGELOG.md` is kept out by a `[walk]` row, or
  its `Cargo.toml` is kept out by a `[walk]` row, refused by `cargo-reader` or unusable, so the
  crate's entries, or the crate itself, are missing from the model.

The assembled changelog is the one changelog of the whole project, so it must omit nothing the tree
holds: written in any of the last three cases, it would leave out a line or a crate with no sign of
it, as a generated file is never written over a model that misses what the tree holds, per
`design@check@phases-gate-the-report`. A changelog crate with no `CHANGELOG.md` holds no entry to
leave out, so it stops no writer, and its own finding stands, per `design@check@changelog-opt-in`.

Each refusal names its repair. The generated-files check's finding over a file at the path that
lacks the header names the repair the writers accept, moving that file away, and never `index`,
which would refuse, per `design@check@finding-names-the-repair`. The three writers refuse by one
function of `model`, over the parsed changelogs and the texts the gathering already reads of each
generated path, so no writer refuses what another would write over.
