# Slice 2: the `cargo` crate, which reads what the tools need of Cargo

## Builds

- A new Component `cargo` at crates/cargo, package `knowledge-architect-cargo`, created under
  `skill@knowledge-architect-component-creation`: its README, CLAUDE.md, CRATES-IO.md, the homes of
  its component registers, its declaration in knowledge-architect.toml, its `include` listing
  `/CHANGELOG.md`, and its copy of the shared changelog written by `cargo x changelog`, which still
  runs until slice 5.
- Its library, behind a typed API:
  - the read of a `Cargo.toml`'s text: the package name, the version with the inheritance of
    `version.workspace = true` from the workspace's manifest, `publish`, `repository` and `include`;
  - the run of `cargo metadata --format-version 1` in a given directory, with the further flags
    its caller passes, which slice 4 measures and fixes for its command, and the read of its output: the resolved graph, and each package's name,
    version, `manifest_path` and `repository`.
- The version in lockstep: `version.workspace = true`, and an exact pin wherever another crate of
  the workspace depends on it, per `design@knowledge-architect@version-lockstep`.
- The readings of Cargo's manifest format and of `cargo metadata`'s output, in its grounding
  files, per the root CLAUDE.md's knowledge row for a finding read outside the project.
- No crate depends on it yet. Slice 3 makes `model` depend on it, slice 4 `query`.
- After the merge, the owner's publish of its crates.io placeholder, as
  `skill@klarch-release@new-crate-placeholder` says: the owner is told once the merge has landed.

## Claims

Each test is shown to fail against a wrong implementation by a mutation of the code it guards, run
with `cargo x mutate`.

1. **A `Cargo.toml` is read for each fact**: fixture manifests with and without `[package]`, with
   `publish = false`, `publish = true`, an absent `publish`, a `repository`, `repository.workspace =
   true`, an `include` list, and `version.workspace = true` beside a workspace manifest, each yield
   the facts they state.
2. **A `cargo metadata` output is read**: a fixture output yields the resolved graph's packages,
   each with its manifest path and its `repository`, and a package absent from the resolved graph
   is not yielded.
3. **The run of `cargo metadata` reports cargo's failure as an error**, not as an empty graph: a
   run in a directory holding no Cargo project returns the error with cargo's message.
4. **The workspace's packages read through the library agree with this repository**: every
   crate under crates/ yields its package name, the workspace version and `publish` unset.

## Fixtures

The tests of claims 1 and 2 hold their fixture manifests and their fixture output inline in the
crate's unit tests; no mock project is needed, since no check reads the facts yet.

## Audit subjects

- `design@knowledge-architect@git-crate`, the precedent; `design@git@an-operation-in-git-s-terms-is-the-library-s`.
- `design@knowledge-architect@version-lockstep`, `design@agent-skills@split-cost-by-publication`.
- `path@check@src/build_origin.rs`, the one reader of a package name today, which keeps its
  compiled constants: it reads no Cargo file.
- `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`, which reads the same facts.
- `issue@knowledge-architect@every-crate-takes-a-version-at-each-release`, which this slice does
  not answer.

## Fails alone on

- A fact read wrong from a fixture `Cargo.toml` or a fixture `cargo metadata` output.

## Premises that expire

- The crate ships the shared changelog's copy, as every crate does until slice 5 removes `cargo x
  changelog`; slice 5 writes its own changelog. The guard is the test of `path@xtask@src/changelog.rs`,
  which fails while the new crate's copy is missing.

## What is already decided

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- The root CLAUDE.md's Component table, which gains the row of `cargo` (no head).
- `design@knowledge-architect@model-crate` and `design@knowledge-architect@query-crate` change when
  slices 3 and 4 make those crates depend on `cargo`, not in this slice.

## Decided design

### A Component `cargo` reads Cargo's manifest and the output of `cargo metadata`

From `thread@per-crate-changelogs@cargo-crate`, argued.

- The library reads what the tools of knowledge-architect need of Cargo, as `git` wraps git for
  them: it holds Cargo's formats, so no other Component reads one of them.
- `model` and `query` depend on it; it depends on no crate of the workspace, so no cycle forms.
- It takes the workspace version, in lockstep. How the crates are versioned is
  `issue@knowledge-architect@every-crate-takes-a-version-at-each-release`, a `todo` at row 1 of the
  roadmap, on the owner's word at R7: a new crate never argues versioning.
- A known later reader: the release procedure the owner intends to ship (R2), and
  `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`.
- Argument: `argument@per-crate-changelogs@arg-38`, `argument@per-crate-changelogs@arg-55`,
  `argument@per-crate-changelogs@arg-57`. Nearest rival: a module of `model`, defeated because
  `model` would gain a second reason to change, Cargo's formats, and `query` would read
  `cargo metadata`'s output in a second Component.

## Harvest

| what | thread | home |
| --- | --- | --- |
| what the Component is for | `thread@per-crate-changelogs@cargo-crate` | a head of `register@knowledge-architect@design`, beside `design@knowledge-architect@git-crate` |
| the rival, a module of `model` | `thread@per-crate-changelogs@cargo-crate` | `register@knowledge-architect@rejected`, if a recording test holds |
| how it reads and runs Cargo | | the new Component's design home, by the tests of `skill@knowledge-architect-decision-recording`; the readings in its grounding files |
