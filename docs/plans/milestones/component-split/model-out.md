# The model, the scan, phases 1 to 3 and the resolver become the `model` crate

The second slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `model`, crates/model, package `knowledge-architect-model`, depending on `git`.
- The files the mapping table of the milestone document sends to `model`, and its share of the
  files it splits: the foundation, phases 1 to 3 without the installed-file check; the resolver,
  cut from the lints of `path@core@src/check/references.rs`, returning a target or the cause of a
  failure, which the lints turn into findings; the phase-1 contribution, a trait of `model` that
  `check`'s `Extension` builds on.
- `model`'s facade, one file, with `#![warn(unreachable_pub)]`.

## Claims

- **The foundation gives the same verdicts.** Test: the binary's suite, `path@core@tests/binary.rs`,
  passes unchanged; shown to discriminate by a recorded mutation of a phase-2 assertion moved to
  `model`.
- **Every reference finding is unchanged in text and in count.** Test: the existing reference tests
  pass; a recorded mutation of the resolver's handling of one failing segment fails one.
- **`model`'s public API is a facade of role modules**: premortem C1. Test: lib.rs names every public
  item; `unreachable_pub` warns on any other, and the clippy gate denies warnings.
- **An extension's tables, declared paths and generated files reach phase 1 unchanged.** Test:
  `path@core@tests/extension_api.rs` passes.

## Fixtures

The mock projects of `path@core@tests/projects/`, unchanged.

## Audit subjects

`path@core@src/check/mod.rs`, `path@core@src/check/references.rs`, `path@core@src/extension.rs`,
`path@core@src/cli/gathered.rs`, `path@core@src/agents.rs`; `design@core@model-then-checks`,
`design@core@phases-gate-the-report`, `design@core@an-extension-plugs-in-through-phased-hooks`,
`design@core@an-extension-claims-its-manifest-tables`, `design@core@a-wrong-declaration-is-a-finding`,
`design@core@inputs-are-gathered`, `design@core@one-entity-table`,
`design@core@one-list-of-generated-files`; `rejected@core@module-tree-as-public-api`;
`issue@core@core-heads-bundle-decisions-with-many-citing-sites`, for the heads it moves.

## Fails alone on

A finding of phases 1 to 3, or a reference finding, that differs from the tree's before the move.

## Premises that expire

- The mock tests of `path@core@src/mock_projects.rs` read internals that this slice moves into
  `model`. Until #integrated-tests, each runs where its internals are: those over the model as a
  unit-test module of `model`, those over the checks as one of the core. The milestone document's
  #testing-crate ends it.
- The installed-file check stays in phase 2 until #check-renamed moves it, so for this slice it
  stays in the foundation, in `model`, and `model` still depends on `agent-skills` for the shipped
  text it compares.

## Decided design

The tools over the record need the model and phases 1 to 3, never phase 4: `show` and `worklist`
run the foundation and no check (#arg-42), and the foundation says whether the model can be
trusted, not whether the content conforms (#arg-68). The resolver is cut from the lints because the
site renderer needs the resolution with no finding attached (#arg-44). The boundary is crossed by 44
of the 99 commits that touched either side, not by every change (#arg-45), and the goals predict
more consumers (#arg-30, #arg-46). An extension's phase-1 contribution sits in `model` because phase
1 does (#arg-93).

The nearest rival, the model kept inside the checker (#arg-16), is a losing alternative of the
milestone document.

## Implementation sequence

1. Cut the resolver from the lints inside the core, with no crate boundary: the mechanism alone.
2. Create `model` and move the files and the foundation, behind the facade.
3. Split `Extension` into the phase-1 contribution and the rest.
4. Move the heads, issues and tripwires whose subject is the model.
5. The harvest.

## Harvest

| what | where |
| --- | --- |
| #model-crate | `model`'s design home: the moved heads; `design@core@model-then-checks` and `design@core@an-extension-plugs-in-through-phased-hooks` rewritten for the two crates |
| premortem C1 | its claim above |
