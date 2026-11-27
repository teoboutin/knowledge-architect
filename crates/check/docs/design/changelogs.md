# Checker design: The changelog checks

## The changelog checks

### The changelog checks run only where the manifest declares `[changelog]` `##changelog-opt-in`

A project whose manifest declares no `[changelog]` table is asked nothing: no changelog finding, no
file assembled, and the check `changelog` is not named among the checks a report lists, neither as
checked nor as not run. Where the table is declared, the check judges the changelog crates of
`design@model@changelog-crates-are-listed-or-published`, each crate's `Cargo.toml` and
`CHANGELOG.md`, a changelog crate with no `CHANGELOG.md` being a finding, and every finding is of the last phase, per
`design@knowledge-architect@model-crate`. The table's `categories` key is required, since the
categories are the project's, per `design@check@nothing-of-a-project-is-compiled-in`. The
declaration adds obligations and removes none, per `design@check@the-regime-has-no-opt-out`.

The constraint derives from `goal@every-target-project-can-adopt-it`: what the checker provides holds
nothing particular to one target project, and a project keeps its changelog in a convention of its
own until it declares the table. The nearest rival, the checks on in every project with published crates, fails
every adopter that has published crates and a changelog of its own convention at its next pin move.

### Each released version follows the next lower one by the highest class of its entries, read in the tree alone `##changelog-bump-check`

A released version must follow the next lower version by the highest class among its entries:
patch, minor or major, as Cargo's semver reads the three numbers. At `1.0.0` and above, major bumps
the first number, minor the second and patch the third, each resetting those after it; under `0.x`,
major and minor both bump the second number and patch the third, and `0.0.z` is read as any other
`0.x`. The readings of the Cargo book, and the one the check takes where the passage admits two, are
`path@check@docs/grounding/cargo-semver.md`.

- **Under the lockstep**, where every changelog crate sets `version.workspace = true`, the versions
  are those every crate's file holds together, and a version's class is the highest among its
  entries over every crate, so a crate's file may skip a version, per
  `design@model@absent-version-section`.
- **Otherwise** each crate's file is judged alone, the same way.
- A version `[changelog] unclassed` lists, a version with no classed entry and the oldest version
  are not judged, and `Next release` carries no version.

The versions are those the changelogs hold: no tag and no history is read, per
`design@knowledge-architect@no-history-beyond-the-branch`. The nearest rival, the version of a
release chosen by a person from the classes of the `Next release` entries and checked by the
release's review alone, is what the check replaces: a count over a section that every gate passes
when it is wrong.

### A changelog crate that sets `include` packages its `CHANGELOG.md`, as Cargo matches the patterns `##changelog-include-check`

A changelog crate whose `Cargo.toml` sets `include`, with no pattern under which Cargo would package
its `CHANGELOG.md`, is a finding; a crate that sets no `include` packages every file Cargo does not
leave out, and is judged nothing. The match is `cargo-reader`'s, per
`design@knowledge-architect@tools-read-cargo-through-the-crate`, and its reading of Cargo's patterns,
measured with `cargo package --list`, is `path@cargo-reader@docs/grounding/cargo-manifest.md`. A crate
whose package omits its changelog publishes none, so a consumer reading the sources cargo fetched
finds no changelog of that crate.

### A crate's changelog opens with the preamble the manifest determines, which names its category order, so the file is read with no manifest `##category-order-in-preamble`

A crate's preamble, its lines above the first version heading, is determined by the manifest: it
names the crate's package; says the file holds that crate's entries only; where a file is
assembled, names it in words by the path `[changelog] assembled` configures, as the changelog of
the whole project in its repository, with `[changelog] assembled-url` as a link where set; lists the
categories in their declared order; and lists the `unclassed` versions, where any. A consumer reads a
crate's changelog in the sources cargo fetched, where it holds no manifest of the publisher, since
no crate ships knowledge-architect.toml: so each crate's file states what a reader needs to read it.
No link is built from a package's `repository`: a URL built from it assumes one host's layout,
against `design@agent-skills@installed-text-requires-no-ci-and-no-pull-request`, and the words name
the file without one.

A preamble that differs from the one the manifest determines by more than its line endings, read
as `design@check@generated-text-compared-without-line-endings` reads a generated text, is a finding
whose message gives the expected text whole. `check --fix` writes no byte of a crate's changelog,
which is hand-written, per `design@check@safe-fix-definition` and `design@check@fix-scope`.
