# Checker design: The changelog checks

## The changelog checks

### The changelog checks run only where the manifest declares `[changelog]` `##changelog-opt-in`

A project whose manifest declares no `[changelog]` table is asked nothing: no changelog finding, no
file assembled, and the check `changelog` is not named among the checks a report lists, neither as
checked nor as not run. Where the table is declared, the check judges the changelog crates of
`design@model@changelog-crates-are-listed-or-published`, each crate's `Cargo.toml` and
`CHANGELOG.md`, a changelog crate with no `CHANGELOG.md` being a finding, and every finding over
those files is of the last phase, per `design@knowledge-architect@model-crate`. The table's `categories` key is required, since the
categories are the project's, per `design@check@nothing-of-a-project-is-compiled-in`. The
declaration adds obligations and removes none, per `design@check@the-regime-has-no-opt-out`.

The constraint derives from `goal@every-target-project-can-adopt-it`: what the checker provides holds
nothing particular to one target project, and a project keeps its changelog in a convention of its
own until it declares the table. The nearest rival, the checks on in every project with published crates, fails
every adopter that has published crates and a changelog of its own convention at its next pin move.
The goal makes no exception for them: they are no check the workflow needs to work well, since the
workflow's procedures read the changelog of the checker a project pins, at a pin move, and never the
project's own. And a check on in every such project would be an ask of it, which
`issue@knowledge-architect@no-head-argues-what-becomes-an-ask` records that no head can argue yet.

### Each released version follows the next lower one by the highest class of its entries, read in the tree alone `##changelog-bump-check`

A released version must be a successor of the next lower version that the highest class among its
entries admits, as Cargo's semver reads the three numbers:

- at `1.0.0` and above, major bumps the first number, minor the second and patch the third, each
  resetting those after it;
- under `0.y.z` with `y` at 1 or more, patch gives `0.y.(z+1)`, minor gives `0.(y+1).0`, and major
  gives `0.(y+1).0` or `1.0.0`;
- under `0.0.z`, every class gives `0.0.(z+1)`, and major may also give `0.1.0` or `1.0.0`;
- where the number a class bumps would overflow the integer the check reads it into, the version
  has no successor of that class, and a release that needs one is a finding.

These are Cargo's readings: Cargo reads a change of the left-most non-zero number as incompatible,
so under `0.y.z` a major release bumps `y` or leaves `0.x`, and under `0.0.z` every release is
incompatible, as the book's sentence that "0.0.z" releases "are always major changes" says. For a
minor release under `0.y.z` the book admits a bump of `z` or of `y`. The check takes `y`, which
Cargo never forbids, since a bump of the left-most non-zero number is one it reads as
incompatible: a minor release then never reaches a consumer's requirement on `0.y` through
`cargo update`, the property `design@knowledge-architect@versioning-policy` gives this repository's
releases. The passages of the Cargo book, quoted, are `path@check@docs/grounding/cargo-semver.md`. A
pre-release version is no version of the grammar, per
`design@model@changelog-version-is-a-release-version`, so the check judges none.

- **Under the lockstep**, where every changelog crate sets `version.workspace = true`, whether or
  not its `CHANGELOG.md` was read, the versions are those every crate's file holds together, and a
  version's class is the highest among its entries over every crate, so a crate's file may skip a
  version, per `design@model@absent-version-section`.
- **Otherwise** each crate's file is judged alone, the same way.
- A version `[changelog] unclassed` lists, a version with no classed entry and the oldest version
  are not judged, and `Next release` carries no version.

The versions are those the changelogs hold: no tag and no history is read, per
`design@knowledge-architect@no-history-beyond-the-branch`. The nearest rival, the version of a
release chosen by a person from the classes of the `Next release` entries and checked by the
release's review alone, is what the check replaces: a count over a section that every gate passes
when it is wrong.

### A changelog crate's package holds its `CHANGELOG.md`, as Cargo lists the files it packages `##changelog-include-check`

A changelog crate whose `Cargo.toml` sets `include`, or sets `exclude` and no `include`, under which
Cargo would not package its `CHANGELOG.md`, is a finding; a crate that sets neither packages every
file Cargo does not leave out, and is judged nothing. The patterns are read as Cargo reads them:
`exclude` only where `include` is unset, an empty `include` counting as unset, and the files Cargo
packages whatever the patterns say, the `Cargo.toml` and the files `license-file` and `readme`
name, counted as packaged; a pattern Cargo refuses, in either list, under which Cargo lists no file
of the package, is a finding too. The match is `cargo-reader`'s, per
`design@knowledge-architect@tools-read-cargo-through-the-crate`, and its reading of Cargo's patterns,
measured with `cargo package --list`, is `path@cargo-reader@docs/grounding/cargo-manifest.md`. A crate
whose package omits its changelog, by whichever key, publishes none, so a consumer reading the
sources cargo fetched finds no changelog of that crate.

### A crate's changelog opens with the preamble the manifest determines, which names its category order, so the file is read with no manifest `##category-order-in-preamble`

A crate's preamble, its lines above the first version heading, is determined by the manifest: it
names the crate's package; says the file holds that crate's entries only; where a file is
assembled, names it in words by the path `[changelog] assembled` configures, as the changelog of
the whole project in its repository, with a link only as `design@check@preamble-links-only-the-declared-url`
says; lists the categories in their declared order; and lists the `unclassed` versions, where any. A
consumer reads a crate's changelog in the sources cargo fetched, where it holds no manifest of the
publisher, since no crate ships knowledge-architect.toml: so each crate's file states what a reader
needs to read it. `klarch changelog` reads it so, through the read of a preamble of `model`, which
gives back the categories and the `unclassed` versions and reports each line of no shape the
preamble takes, per `design@query@changelog-places-each-crate-by-its-own-preamble` and
`design@query@changelog-reports-what-it-cannot-print`.

A preamble that differs from the one the manifest determines by more than its line endings, read
as `design@check@generated-text-compared-without-line-endings` reads a generated text, is a finding
whose message gives the expected text whole. `check --fix` writes no byte of a crate's changelog,
which is hand-written, per `design@check@safe-fix-definition` and `design@check@fix-scope`.

### The preamble links the assembled changelog only by the URL `[changelog] assembled-url` declares, never by one built from a package's `repository` `##preamble-links-only-the-declared-url`

Where `[changelog] assembled-url` is set, the preamble gives it as the link to the assembled
changelog; where it is not, the preamble names the file in words alone, and no package's
`repository` is required. A URL built from `repository` assumes one hosting service's layout,
against `goal@every-target-project-can-adopt-it`, under which what the checker provides holds
nothing particular to one target project, since `goal@serves-one-kind-of-project` names no hosting
service among the conditions of a target project; the words name the file without one.

### With one changelog crate, a file at the assembled path that opens with the generated header is a finding `##generated-header-at-an-unassembled-path`

Where one changelog crate is read, nothing is assembled, per
`design@model@assembled-changelog-from-two-crates`, and a walked file at the path
`[changelog] assembled` names, its default included, that opens with the generated header is a finding whose repair is
its deletion. Such a file is generated text that no writer renews and that no other check holds to
the tree, against `goal@documentation-stays-consistent`: a tree that passes conforms to every rule
the checker holds, a generated file's rule, that it holds what the tree renders, among them. The
nearest rival, the file walked with no finding, leaves that stale text standing in a passing tree.
A file at the path without the header is the project's, and is judged as any walked file.
