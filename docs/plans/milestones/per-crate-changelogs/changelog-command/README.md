# Slice 4: `klarch changelog`, the pin move that runs it, and the pin refusal that names it

## Builds

- The command `klarch changelog <package> --from <version> [--to <version>] [--package <name>]...`:
  its declaration and dispatch in `cli`, per `design@cli@command-parsing-home`, and its logic in
  `query`, behind a typed function, which `query` reaches through `cargo-reader` for the registry sources
  and through `model` for the parse and the rendering.
- In `model`, beside `expected_preamble` and `assembled_text` in `path@model@src/changelogs/render.rs`,
  public through its facade: the read of a crate's preamble that gives back the categories and the
  unclassed versions it states, as the table of a crate read with no manifest of its publisher,
  each line of another shape reported by its line; the rendering of the assembled layout of a
  range of versions with no generated header; and the lines of a crate's changelog that the
  rendering has no place for.
- `query` depending on `cargo-reader`; its purpose widened from the project's record to the record of the
  project or of one of its dependencies, in the rewritten `design@knowledge-architect@query-crate`
  and in every text the milestone document's "Reversed or rewritten" lists beside it: the `query`
  row of the root CLAUDE.md's Component table, `path@query@CLAUDE.md`, `path@query@CRATES-IO.md`,
  the `description` of `path@query@Cargo.toml`, the crate documentation of `path@query@src/lib.rs`,
  whose first two lines list the commands it holds, `path@query@README.md`'s first sentence, and
  the command list of `path@cli@README.md`.
- The installed setup skill's pin move, step 2: after the fetch, run `cargo klarch changelog
  <checker package> --from <old pin>` and read its output, in place of reading one file in the
  registry, for a project that has a Cargo project for `cargo metadata` to read. A project with no
  Rust, which the pin move's step 1 covers by "the version of the local install", keeps a reading by
  hand: the `CHANGELOG.md` of each crate of the checker in the registry, listed by package name,
  which step 2 names. Its listing of the Migration entries that trigger an audit stays as it is
  until slice 5.
- The message of the pin refusal in the function `refuse_another_version_built_at` of
  `path@check@src/commands/mod.rs`, line 305, "read the changelog of each version from {pinned} to
  {version}", which names the command, by the project's declared command, in the order the pin move
  gives: the pin moved, the new version fetched, then the command run.
- The measurement of `cargo metadata` with and without `--locked` and `--offline`, recorded in
  a grounding file of `cargo-reader`.
- The entries of this slice in the shared changelog's `Next release` section, under its current
  grammar.

## Claims

Each test is shown to fail against a wrong implementation by a mutation of the code it guards, run
with `cargo x mutate`.

1. **The selection**: over a mock consumer whose resolved graph holds packages of two projects, the
   command reads every package whose `repository` equals the named package's, and no other; with a
   named package that sets no `repository`, it reads that package and those `--package` names; with
   `--package` given, it reads exactly the named set.
2. **The range**: the output holds the versions in (`--from`, `--to`], and, with `--to` absent, every
   version above `--from` that the files hold, `Next release` excluded, in the assembled layout the
   model renders, with no generated header, the crates under each category in the order of
   "Decided design", one selected crate included.
3. **The order of the pin move**, premortem cause 2: run in the order step 2 gives, after the pin's
   edit and `cargo fetch`, the command succeeds over a mock consumer; run before the fetch, it fails
   with cargo's message and a repair naming the fetch.
4. **No write**: a run writes no file of the consumer, `Cargo.lock` included, as the measurement of
   the flags shows.
5. **An unreadable changelog is reported, never dropped**: a selected package with no
   `CHANGELOG.md`, a line of its preamble the read does not take, and, in a section of the range, an
   unread line or an entry the rendering has no place for, each make the command exit with the
   ladder's negative code and name the file and the line, the rest of the output still printed.
6. **The pin refusal names the command**: the refusal's text for a binary newer than the pin holds
   the project's declared command followed by `changelog`, the old pin as `--from`, and the order
   pin, fetch, command. Its test is the unit test of `refuse_another_version_built_at` in
   `path@check@src/commands/mod.rs`, so `path@testing@tests/binary.rs` does not change.
7. **The preamble reads back**: the categories and the unclassed versions that `expected_preamble`
   writes are what the read gives back, and a preamble line of no shape it writes is reported by its
   line.

## Fixtures

A mock consumer, built by its test in a copy outside this repository's tree, or committed under
`path@testing@tests/projects/` with a `[workspace]` table of its own: cargo searches the parent
directories for a `Cargo.toml` with a `[workspace]` table, per the Cargo reference's chapter on
workspaces, and the root `Cargo.toml` here lists its members and excludes nothing, so a nested
project without its own table is read as part of this workspace. The consumer is a Cargo project
whose dependencies are path
dependencies on three mock published packages of one project, each holding a `Cargo.toml` with the
same `repository` and a `CHANGELOG.md` in the model's format, standing for `knowledge-architect`,
`-gates` and `-mutate`, where the first does not depend on the other two, as
`path@cli@Cargo.toml` depends on neither `gates` nor `mutate`; and one package of another project.
A path dependency's source is reported by `cargo metadata` as a registry source is, which the slice
measures before relying on it. The tests of the command go in a new file under
`path@testing@tests/`, not in `path@testing@tests/binary.rs`.

## Audit subjects

- `path@query@src/lib.rs`, `path@cli@src/cli/mod.rs`, `path@check@src/commands/mod.rs`,
  `path@agent-skills@content/skills/setup/SKILL.md`, `path@testing@tests/binary.rs`.
- `design@knowledge-architect@query-crate`, `design@query@a-function-takes-a-configured-manifest`,
  `design@query@query-depends-on-no-check`, `design@cli@command-parsing-home`,
  `design@cli@exit-code-ladder`, `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`,
  `design@agent-skills@xtask-pins-checker`, `design@agent-skills@pinned-binary-runs-inside-the-project`.
- `design@knowledge-architect@one-program-for-the-tools`, which this slice's branch writes, closing
  the issue `the-rule-of-one-program-for-the-tools-is-argued-by-no-head`, D17 of the milestone.
- `issue@knowledge-architect@a-typed-command-prints-and-exits-rather-than-returning-data`,
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
- `issue@check@commands-mod-rs-holds-two-parts-that-change-apart` and
  `issue@testing@binary-rs-is-past-one-read`, weighed below.

## Fails alone on

- The command's output, or its exit code, over the mock consumer.

## Premises that expire

- Until slice 5 merges, this repository's crates ship the shared changelog, which the model's parse
  does not read, so the rewritten step 2 cannot be followed against a version of this repository
  published before slice 5. What ends it: slice 5's merge. The guard is D1 of the milestone
  document, ruled by the owner: no release is cut between this slice's merge and slice 5's.
- The listing of the audit triggers in step 2 keeps its wording "cites an axis's section" until
  slice 5 rewrites it with `thread@per-crate-changelogs@axis-named-in-words`.
- One range, `--from` and `--to`, is applied to every selected crate, which holds while every crate
  of a project takes the same version, per `design@knowledge-architect@version-lockstep`. What ends
  it: a reversal of the lockstep under
  `issue@knowledge-architect@every-crate-takes-a-version-at-each-release`, whose discussion the
  milestone's "Later consequences" sends this command's range to.

## What the work is

What this slice alone changes, beside what the milestone document's "What the work is" says:

- **`issue@check@commands-mod-rs-holds-two-parts-that-change-apart`, weighed.** The slice changes
  one string literal of the refusal part of `path@check@src/commands/mod.rs`, its unit test, and
  nothing of the runs of `check`. No split is planned: this work does not need it. The signal still
  holds, so the issue stays open at its re-entry point, per
  `design@agent-skills@split-not-needed-is-an-issue`; its figures are taken again in a commit of
  their own.
- **`issue@testing@binary-rs-is-past-one-read`, weighed.** The tests of the new command go in a
  file of their own, so `path@testing@tests/binary.rs` does not change and the issue's re-entry
  point is not met; it stays open.

## What is already decided

This slice's design audit recorded its fit lines in
[the design audit's search](search/design-audit-1.txt), and its thread in
[deliberation](deliberation.md).

The seeds of this slice's design audit are the design entries this section cites, as
`skill@knowledge-architect-planning@working-a-slice` takes them; those of "Audit subjects" it does
not cite below are restated here: `design@query@query-depends-on-no-check`,
`design@cli@command-parsing-home`, `design@cli@exit-code-ladder`,
`design@knowledge-architect@a-typed-command-prints-and-returns-its-code` and
`design@agent-skills@xtask-pins-checker`.

Rewritten by this slice, as the milestone document's "Reversed or rewritten" lists it with the
texts it repairs: `design@knowledge-architect@query-crate`. Its purpose widens to the record of the
project or of one of its dependencies; its dependency line gains `cargo-reader`; its reasoning over the
default criteria gains a second reason to change, a dependency's changelog read through `cargo-reader`,
argued at this slice's design audit; and the placement of `klarch changelog` in `klarch` lands in
it, or in a root head beside it.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@knowledge-architect@partial-publish-rerun`: its publish order gains `cargo-reader` before
  `query`, which this slice makes depend on it.
- `design@query@a-function-takes-a-configured-manifest`: judged at the audit, since the command
  reads the changelogs of dependencies, which no manifest of the caller describes.
- `design@agent-skills@pinned-binary-runs-inside-the-project`: judged at the audit, since the pin
  move now runs the pinned binary before the install.
- `design@knowledge-architect@cargo-reader-crate`: the crates that depend on `cargo-reader` gain
  `query`, a member its argument covers; added to this list at the design audit.

The audit judged the four: `design@knowledge-architect@partial-publish-rerun` and
`design@agent-skills@pinned-binary-runs-inside-the-project` conform;
`design@query@a-function-takes-a-configured-manifest` and
`design@knowledge-architect@cargo-reader-crate` are brought to the rules by a conforming step
before the harvest. The command's function takes the manifest its caller configured, like every
other function of `query`, and reads only its root, so the title of the first stays true.

## Decided design

### `klarch changelog` assembles the dependencies' changelogs from the registry sources

From `thread@per-crate-changelogs@registry-changelog-command`, argued, and
`thread@per-crate-changelogs@changelog-the-pin-reads`.

- It runs `cargo metadata --format-version 1` in the consuming project, through `cargo-reader`, with
  `--locked --offline` unless the slice's measurement shows otherwise: the discussion assumed that
  without them the run may write `Cargo.lock` or reach the network, and the slice measures it. The
  path of each package's source comes from `cargo metadata`, so the command needs no knowledge of
  the registry's layout.
- **The selection**: every package of the resolved graph whose `repository` equals the named
  package's; where the named package sets no `repository`, the named package and those
  `--package` names; where `--package` is given, the named set. The closure of the named package's
  own dependencies is not the selection, since the closure of `knowledge-architect` holds neither
  `gates` nor `mutate`, which a project's maintenance crate depends on, per
  `design@agent-skills@xtask-pins-checker`.
- It parses each selected package's `CHANGELOG.md` with the model's parse, by its preamble, and
  prints the versions in (`--from`, `--to`] in the assembled layout, with the model's rendering, so
  its text is what the publisher's assembled file holds for those versions. `--to` absent, the range
  runs to the newest version the files hold.
- It works for any dependency whose changelogs follow the model, which serves
  `goal@generic-tools-and-procedures-are-shipped`.
- Its logic is in `query`, whose purpose widens to the record of the project or of one of its
  dependencies; its declaration is in `cli`. It is placed in `klarch`, since the pin move runs the
  binary the project pinned, which links the model, by the rule of
  `design@knowledge-architect@one-program-for-the-tools`, which this slice's branch argues, D17.
- The pin move's step 2 runs it after the fetch, and reads its output, as "Builds" says.
- **The flags, measured**: `--locked --offline`, both. Measured with cargo 1.98.0 over a mock
  consumer of path dependencies and over a registry dependency in an empty `CARGO_HOME`: without
  `--locked`, the run writes `Cargo.lock` where it is missing or stale; without `--offline`, it
  updates the index and downloads crates; with both, a lock made stale by the pin's edit fails with
  "cannot update the lock file … because --locked was passed", and after `cargo fetch` the run
  passes and writes nothing; with both, a registry source removed from `registry/src` is unpacked
  again from the local cache, with no network. The grounding file of `cargo-reader` records it.
- **Cargo's failure** is the ladder's could-not-run, with Cargo's message and a repair naming
  `cargo fetch` after the pin's edit, since cargo's own help names `--offline` alone, which writes
  the lock.
- **The function takes the manifest its caller configured**, as every function of `query` does, per
  `design@query@a-function-takes-a-configured-manifest`, and reads only its root, where `cargo
  metadata` runs. It is dispatched like every command but `init`, so a binary of another version
  than the pin refuses it: the pin refusal names the pin's edit and the fetch before the command.
- **A named package or a `--package` name the graph does not hold** is a could-not-run, exit 2: the
  invocation names nothing to read. A version that is no version is refused while parsing, per
  `design@cli@command-parsing-home`, and `--to` not above `--from` by `cli::run`, by a type of
  `query` that cannot hold it.
- **A package the graph holds at several versions** is read in its newest version's file, which
  holds the sections of every earlier one.
- **The table of a dependency is read from its preamble**: its categories in their order and its
  unclassed versions, with every category admitted for every crate, since a preamble states no
  `[changelog.category-components]` and the publisher's check enforces it.
- **The output** is the assembled layout of the selected crates over the range, with no generated
  header, since it is no file; one selected crate is printed in the same layout, since
  `design@model@assembled-changelog-from-two-crates` decides when a project's file is assembled,
  not what this command prints. The crates under each category are the named package first, then
  the others in the order of their package names, per
  `thread@per-crate-changelogs@consumer-crate-order`: no file a consumer receives states the
  publisher's order of `design@model@assembled-crates-in-the-manifest-s-order`.
- **What the output cannot hold is reported**, never dropped: after the layout, one line per
  selected package with no `CHANGELOG.md`, per line of a preamble the read does not take, and per
  unread line or entry with no place in a section of the range, each naming the file and the line;
  the exit is then 1. A defect in a section outside the range is not reported, since nothing of it
  is printed.
- **Its output is for a reader, not a contract**, as `anchors` and `worklist` say of theirs, per
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
- Argument: `argument@per-crate-changelogs@arg-25`, `argument@per-crate-changelogs@arg-31`,
  `argument@per-crate-changelogs@arg-39`, `argument@per-crate-changelogs@arg-41`,
  `argument@per-crate-changelogs@arg-50`, `argument@per-crate-changelogs@arg-52`. Nearest rival: the
  selection by the named package's dependency closure, defeated by the measurement that
  `path@cli@Cargo.toml` depends on neither `gates` nor `mutate`.

## Acceptance criteria

### The command prints every Migration entry of the range from every crate of the consumer's graph `##selection-reaches-every-crate`

- Guards `thread@per-crate-changelogs@registry-changelog-command`, its selection rule.
- Judged by slice 5, whose conversion gives this repository's crates the per-crate files the
  command reads, and at the deletion of the milestone document; slice 4 holds the selection rule as
  claim 1, whose test is no observation of real use.
- Fires when a pin move, in this repository or in a project that pins it, as a retrospective file
  or an issue entry records, finds a Migration entry of a crossed version, in a crate of the named
  package's project, missing from the command's output: the selection by `repository` then misses a
  crate of the project, such as one whose `repository` is spelled otherwise.

## Harvest

| what | thread | home |
| --- | --- | --- |
| the command's placement in `klarch`, with its declaration in `cli` and its logic in `query` | `thread@per-crate-changelogs@registry-changelog-command` | the root's design home: the rewritten `design@knowledge-architect@query-crate`, or a root head beside it, since the placement constrains `cli` and `query` together |
| the command's selection over the resolved graph | `thread@per-crate-changelogs@registry-changelog-command` | a head of `register@query@design`; the rival selection in `register@query@rejected` if a recording test holds |
| the pin move's step 2 | `thread@per-crate-changelogs@changelog-the-pin-reads` | the installed setup skill's text, and a head of `register@agent-skills@design` where the tests of `skill@knowledge-architect-decision-recording` admit one |
| the measurement of the flags | | a grounding file of `cargo-reader` |
| the acceptance criterion | `thread@per-crate-changelogs@registry-changelog-command` | reported in this slice's landing commit, as `skill@knowledge-architect-planning@acceptance-criteria` says |
| premortem cause 2 | `thread@per-crate-changelogs@registry-changelog-command` | claim 3; no tripwire |
| the crate order of the output | `thread@per-crate-changelogs@consumer-crate-order` | the head of `register@query@design` on the selection, or one beside it, where a recording test admits it; the rival in `register@query@rejected` if a recording test holds |
| the read of a preamble | | `design@check@category-order-in-preamble`, whose reader with no manifest it builds, where a recording test admits a member |
