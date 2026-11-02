# The model, the scan, phases 1 to 3 and the resolver become the `model` crate

The second slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `model`, crates/model, package `knowledge-architect-model`, depending on `git`;
  its goals drafted and put to the owner as the milestone document says.
- The files the mapping table of the milestone document sends to `model`, and its share of the
  files it splits: the foundation, phases 1 to 3, the installed-file check still in phase 2 until
  #check-renamed moves it; `complete_working_tree`, the gate the writers and `worklist` call; the
  resolver, cut from the lints of `path@core@src/check/references.rs`, returning a
  `model::resolve::Outcome`, a target or the cause of its failure, which the lints turn into
  findings; the phase-1 contribution as data, the struct `model::extension::Contribution`, which
  `check` computes from each extension's `Extension::tables` and `Extension::resolve`, whose
  signatures do not change, so no extension changes. A mapping table from each failure arm of the
  six resolver functions to a cause of `Outcome` is written at this slice's audit, before the cut.
- `model`'s facade, one file, with `#![warn(unreachable_pub)]`.

## Claims

- **The foundation gives the same verdicts.** Test: the binary's suite,
  `path@core@tests/binary.rs`, passes unchanged; shown to discriminate by a recorded mutation of a
  phase-2 assertion moved to `model`.
- **Every reference finding is unchanged in text and in count.** Test: the existing reference tests
  pass; a recorded mutation of the resolver's handling of one failing segment fails one.
- **`show` prints what it printed.** It calls `check::references::register` and
  `check::references::planned_target` today, for a target and for a refusal; the resolver keeps
  both. Test: the binary's tests of `show` pass; a recorded mutation of the resolver's refusal of a
  `planned` path fails one.
- **`model`'s public API is a facade of role modules**: premortem C1. Test: lib.rs names every
  public item; `unreachable_pub` warns on any other, and the clippy gate denies warnings; shown to
  fail by a recorded `pub` item outside the facade, which the gate refuses.
- **An extension's tables, declared paths and generated files reach phase 1 unchanged.** Test:
  `path@core@tests/extension_api.rs` passes with no change, and the tests of claimed tables do:
  `a_table_no_extension_claims_is_a_complaint` and `a_claimed_table_the_manifest_leaves_out_is_a_complaint`
  in `path@core@src/extension.rs`, and `the_core_binary_refuses_a_table_no_extension_of_it_claims`
  in `path@core@tests/binary.rs`; shown to discriminate by a recorded mutation that drops the claimed
  tables from the contribution, which the last of these fails on.
- **Every git invocation of the core and of `model` goes through `git`**: the claim of #git-out,
  taken again over both crates.

## Fixtures

The mock projects of `path@core@tests/projects/`, unchanged.

## Audit subjects

`path@core@src/check/mod.rs`, `path@core@src/check/references.rs`, `path@core@src/extension.rs`
(`Extension::tables`, `Extension::resolve`, `Resolution`, `extension::configure`),
`path@core@src/cli/gathered.rs`, `path@core@src/agents.rs`, and `show` in
`path@core@src/cli/mod.rs`; `design@core@model-then-checks`, `design@core@phases-gate-the-report`,
`design@core@an-extension-plugs-in-through-phased-hooks`,
`design@core@an-extension-claims-its-manifest-tables`, `design@core@a-wrong-declaration-is-a-finding`,
`design@core@inputs-are-gathered`, `design@core@one-entity-table`,
`design@core@one-list-of-generated-files`, `design@core@orphan-rejected-entry` and
`design@core@heading-register-two-shapes`, whose text names files this slice cuts;
`rejected@core@module-tree-as-public-api`.

## Fails alone on

A finding of phases 1 to 3, or a reference finding, that differs from the tree's before the move.

## Premises that expire

- The mock tests of `path@core@src/mock_projects.rs` read internals this slice moves into `model`.
  Until #integrated-tests, each runs where its internals are: those over the model as a unit-test
  module of `model`, those over the checks as one of the core. Those in `model` reach the mock
  projects of the core's directory by a path relative to `model`'s, the one crate reaching another's
  fixtures in the milestone, for four slices. #integrated-tests ends it; the claim of that slice that
  no unit test loads a mock project guards it.
- The installed-file check stays in phase 2 until #check-renamed, so for this slice it stays in the
  foundation, in `model`, and `model` depends on `agent-skills` for the shipped text it compares.
  The claim of #check-renamed that `model` no longer depends on `agent-skills` guards it.

## Decided design

The tools over the record need the model and at most phases 1 to 3, never phase 4: `id`,
`worklist` and the writers run the foundation and no check, and `show` runs no phase but reads the
resolver (#arg-42, corrected at the plan's review); the foundation says whether the model can be
trusted, not whether the content conforms (#arg-68). The resolver is cut from the lints because the
site renderer needs the resolution with no finding attached (#arg-44). The boundary is crossed by 44
of the 99 commits that touched either side, not by every change (#arg-45), and the goals predict
more consumers (#arg-30, #arg-46). An extension's phase-1 contribution sits in `model` because phase
1 does (#arg-93); it is data rather than a trait so that an extension's code does not change.
`complete_working_tree` sits in `model` because `worklist` calls it, and `query` must reach it with
no check (the plan's review, premortem C2).

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
| #model-crate | `model`'s design home: the moved heads; `design@core@model-then-checks` and `design@core@an-extension-plugs-in-through-phased-hooks` rewritten for the two crates; `design@core@phases-gate-the-report` rewritten for the crate that runs phases 1 to 3; `design@core@inputs-are-gathered` rewritten for the two gatherings, one in each crate |
| premortem C1 | its claim above |
