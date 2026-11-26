# Slice 4: `klarch changelog`, the pin move that runs it, and the pin refusal that names it

## Builds

- The command `klarch changelog <package> --from <version> [--to <version>] [--package <name>]...`:
  its declaration and dispatch in `cli`, per `design@cli@command-parsing-home`, and its logic in
  `query`, behind a typed function, which `query` reaches through `cargo` for the registry sources
  and through `model` for the parse and the rendering.
- `query` depending on `cargo`; its purpose widened from the project's record to the record of the
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
  `path@check@src/commands/mod.rs`, line 304, "read the changelog of each version from {pinned} to
  {version}", which names the command.
- The measurement of `cargo metadata` with and without `--locked` and `--offline`, recorded in
  a grounding file of `cargo`.
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
   model renders.
3. **The order of the pin move**, premortem cause 2: run in the order step 2 gives, after the pin's
   edit and `cargo fetch`, the command succeeds over a mock consumer; run before the fetch, it fails
   with cargo's message and a repair naming the fetch.
4. **No write**: a run writes no file of the consumer, `Cargo.lock` included, as the measurement of
   the flags shows.
5. **An unreadable changelog is reported, never dropped**: a dependency whose changelog holds an
   unread line, or whose preamble the parse cannot read, makes the command exit with the ladder's
   negative code and name the file and the line, the rest of the output still printed.
6. **The pin refusal names the command**: the refusal's text for a binary newer than the pin holds
   `klarch changelog` with the two versions.

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
- `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`,
  `issue@knowledge-architect@a-typed-command-prints-and-exits-rather-than-returning-data`,
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

## What the work is

What this slice alone changes, beside what the milestone document's "What the work is" says:

- **`issue@check@commands-mod-rs-holds-two-parts-that-change-apart`, weighed.** The slice changes
  one string literal of the refusal part of `path@check@src/commands/mod.rs`, and nothing of the
  runs of `check`. The default: no split is planned; the change adds to one of the two parts and
  nothing to the other. The slice's design audit closes the issue on that argued decision or plans
  the split before the work.
- **`issue@testing@binary-rs-is-past-one-read`, weighed.** The tests of the new command go in a
  file of their own, so `path@testing@tests/binary.rs` does not change and the issue's re-entry
  point is not met; it stays open.

## What is already decided

The seeds of this slice's design audit are the design entries this section cites, as
`skill@knowledge-architect-planning@working-a-slice` takes them; those of "Audit subjects" it does
not cite below are restated here: `design@query@query-depends-on-no-check`,
`design@cli@command-parsing-home`, `design@cli@exit-code-ladder`,
`design@knowledge-architect@a-typed-command-prints-and-returns-its-code` and
`design@agent-skills@xtask-pins-checker`.

Rewritten by this slice, as the milestone document's "Reversed or rewritten" lists it with the
texts it repairs: `design@knowledge-architect@query-crate`. Its purpose widens to the record of the
project or of one of its dependencies; its dependency line gains `cargo`; its reasoning over the
default criteria gains a second reason to change, a dependency's changelog read through `cargo`,
argued at this slice's design audit; and the placement of `klarch changelog` in `klarch` lands in
it, or in a root head beside it.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@query@a-function-takes-a-configured-manifest`: judged at the audit, since the command
  reads the changelogs of dependencies, which no manifest of the caller describes.
- `design@agent-skills@pinned-binary-runs-inside-the-project`: judged at the audit, since the pin
  move now runs the pinned binary before the install.

## Decided design

### `klarch changelog` assembles the dependencies' changelogs from the registry sources

From `thread@per-crate-changelogs@registry-changelog-command`, argued, and
`thread@per-crate-changelogs@changelog-the-pin-reads`.

- It runs `cargo metadata --format-version 1` in the consuming project, through `cargo`, with
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
  binary the project pinned, which links the model; the general rule of one program stays
  `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`'s question,
  and this slice's commit names the argument for that issue.
- The pin move's step 2 runs it after the fetch, and reads its output, as "Builds" says.
- Argument: `argument@per-crate-changelogs@arg-25`, `argument@per-crate-changelogs@arg-31`,
  `argument@per-crate-changelogs@arg-39`, `argument@per-crate-changelogs@arg-41`,
  `argument@per-crate-changelogs@arg-50`, `argument@per-crate-changelogs@arg-52`. Nearest rival: the
  selection by the named package's dependency closure, defeated by the measurement that
  `path@cli@Cargo.toml` depends on neither `gates` nor `mutate`.

## Acceptance criteria

### The command prints every Migration entry of the range from every crate of the consumer's graph `##selection-reaches-every-crate`

- Guards `thread@per-crate-changelogs@registry-changelog-command`, its selection rule.
- Judged by this slice, by its test over the mock consumer that depends on the stand-ins for
  `knowledge-architect`, `-gates` and `-mutate`.
- Fires when one Migration entry of the range, in any crate of the consumer's resolved graph that
  shares the named package's project, is missing from the output: the selection rule is then wrong.

## Harvest

| what | thread | home |
| --- | --- | --- |
| the command's placement in `klarch`, with its declaration in `cli` and its logic in `query` | `thread@per-crate-changelogs@registry-changelog-command` | the root's design home: the rewritten `design@knowledge-architect@query-crate`, or a root head beside it, since the placement constrains `cli` and `query` together |
| the command's selection over the resolved graph | `thread@per-crate-changelogs@registry-changelog-command` | a head of `register@query@design`; the rival selection in `register@query@rejected` if a recording test holds |
| the pin move's step 2 | `thread@per-crate-changelogs@changelog-the-pin-reads` | the installed setup skill's text, and a head of `register@agent-skills@design` where the tests of `skill@knowledge-architect-decision-recording` admit one |
| the measurement of the flags | | a grounding file of `cargo` |
| the acceptance criterion | `thread@per-crate-changelogs@registry-changelog-command` | reported in this slice's landing commit, as `skill@knowledge-architect-planning@acceptance-criteria` says |
| premortem cause 2 | `thread@per-crate-changelogs@registry-changelog-command` | claim 3; no tripwire |
