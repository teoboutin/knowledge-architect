# The shared test Component holds the fixtures and the integrated tests

The sixth and last slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

As D2 of the milestone document rules it:

- The Component `testing`, crates/testing, package `knowledge-architect-testing`, published. Its
  library holds the harness an extension's tests need, what `path@check@src/testing.rs` exposes
  today: `foundation`, `run_with`, `Phase`, `Report`, `Stop`, `Structure`, `CHECKS`. It depends
  on `check` and `model` for the harness, and on `cli` as a dev-dependency for its integrated tests;
  no crate depends on it. Its `tests/` holds this repository's mock projects and its integrated
  tests, which its `include` does not ship.
- `cli`'s interim re-export of the harness, and `check`'s role module `testing`, removed; a
  Migration entry of the changelog names the new import of an extension's tests.
- The 36 mock tests of `path@check@src/mock_projects.rs`, and the integrated tests of
  `path@cli@tests/extension_api.rs`, rewritten to drive the crates through their public API, as
  an extension's test does. A test whose claim no consumer could observe becomes a unit test over a
  model built in memory, per `design@check@model-then-checks`. `mock_projects.rs` holds 5 struct
  literals of `Inputs`, two of them in the shared helpers `findings_with`, which 17 tests reach,
  through `findings` and `one` too, and `phase_two`, which 1 test reaches, counted by matching each
  `#[test]` body for `findings_with(`, `findings()`, `one("` and `phase_two()`; two literals inject a state, an empty map of committed files: each of those becomes
  a mock project whose tree holds no committed generated file, or a unit test.
- `design@check@api-facade` rewritten: its sentence "A test that needs a private item is a unit-test
  module inside the crate" gives way to the guideline of the code-shape fragment; its bullet
  "Nothing becomes public for the core's own tests" stays: an item becomes public because a
  consumer needs it, which an integrated test can show. Its restatement in the crate's CLAUDE.md is
  rewritten with it.
- `design@check@inputs-are-gathered` rewritten for its sentence on the mock tests that assemble
  `Inputs` by hand, which this slice ends.

## Claims

- **No unit test of any crate loads a mock project.** Test:
  `git grep -n 'join("tests/projects")' crates/*/src` lists none; today it lists the helper of
  `path@check@src/mock_projects.rs`. Shown to fail by a recorded unit test that joins the path.
- **Every integrated test reaches the crates through their public API.** Test: the tests sit in
  another crate than the code, so the compiler refuses a private item; an item an integrated test
  needs and cannot reach shows an item a consumer needs, and is opened as a `todo` under the role
  module its use belongs to. `tripwire@check@private-item-needed` is unchanged.
- **No package ships a fixture.** Test: `cargo package --list` of every published crate,
  `knowledge-architect-testing` included, lists no file under a directory named tests/projects;
  shown to fail by a recorded `include` row naming one.
- **An extension's tests reach the harness through `knowledge-architect-testing` alone.** Test:
  the integrated tests of `path@cli@tests/extension_api.rs`, moved, import the harness from that
  crate, and `cli` exposes no `testing` path; shown to fail by a recorded re-export left in `cli`,
  which `cargo tree -p knowledge-architect` then shows as a dependency on `knowledge-architect-testing`.
- **Every claim the 36 mock tests made is still tested.** Test: the slice's commit lists each old
  test with the test that now carries its claim; a claim with no successor refutes it.

## Fixtures

The mock projects of `path@check@tests/projects/`, moved; a new mock project for each of the two
states injected by hand, in the vocabulary of the existing ones.

## Audit subjects

`design@check@api-facade`, `design@check@inputs-are-gathered`; `design@check@checked-sentinel-values`
and `design@check@checker-source-literals-are-data`, with every `rejected` entry they cite.

**A load-bearing point this audit opens a design session on, before any code:** where the tests of
`cli`, the binary's, `init`'s and `path@cli@tests/extension_api.rs`'s, find their fixtures once the mock
projects move to `testing`. Its constraints, each from the record:

- the binary's tests stay in `cli`, the one package where `CARGO_BIN_EXE_klarch` is set;
- `"fixture"` is accepted only strictly inside the core's compiled directory or a directory the
  binary names as its own source, per `design@check@checked-sentinel-values`, and a binary names only
  the libraries it links, per `design@check@checker-source-literals-are-data`; `cli`'s binary links
  nothing of `testing`, per D2;
- no dependency cycle, dev-dependencies included, per `design@agent-skills@shared-test-component`;
  `testing` dev-depends on `cli`;
- one home for the fixtures, no crate reaching into another's, the argument of the same head;
- the ways around these that the record holds are each a live `rejected` entry:
  `rejected@check@exempt-directory-manifest-row`, `rejected@check@run-time-checker-directory-discovery`,
  `rejected@check@literals-as-data-in-every-crate`, `rejected@check@fixture-marker-macro` and
  `rejected@check@sentinels-accepted-everywhere`.

Its shapes, which the session measures before putting the fork to the owner: the binary's tests
moved into `testing`, driving `cli`'s library in process, if an in-process run reaches every claim
they hold, its exit code and its printed report included; the binary's fixtures kept in `cli`'s own
`tests/`, a second home; or a rewrite of the sentinel's rule, which reopens its head.

`design@knowledge-architect@package-include-whitelist`; `rejected@check@inputs-struct-literal`,
`rejected@check@doc-hidden-test-items`, `rejected@check@testing-feature-for-test-items`,
`rejected@check@unit-tests-moved-under-tests`; `tripwire@check@private-item-needed`;
`design@knowledge-architect@package-include-whitelist`; `rejected@check@inputs-struct-literal`,
`rejected@check@doc-hidden-test-items`, `rejected@check@testing-feature-for-test-items`,
`rejected@check@unit-tests-moved-under-tests`; `tripwire@check@private-item-needed`;
`issue@check@the-tests-read-the-machine-s-git-configuration`, whose helpers move here. Each
reference naming the anchor `core` here names the entry as this document is written;
#check-renamed re-points them.

## Fails alone on

A claim of the old mock tests no test makes any more, or a fixture in a package.

## Premises that expire

None: this slice ends the premises of #model-out and #consumers-out on where the mock tests run,
and of #cli-on-top on its interim re-export of the harness and on where its tests find their
fixtures.

## Decided design

The owner's argument: integrated tests out of unit tests and out of the crates test the public API,
and one home for the fixtures keeps crates from duplicating them or reaching into another's
(#arg-37, #arg-41); the harness is published because extensions are supported (#arg-40). The
fixtures stay unshipped (#arg-87). The scope is the integrated test, so the 437 unit tests stay
where they are (#arg-83, #arg-107, #arg-108). What an integrated test cannot observe is hidden from
consumers too (#arg-101). The harness is published by `testing` itself, which an extension's tests
depend on separately, by the owner's choice at D2. The nearest rival, the mock tests
kept as a unit-test module, is a losing alternative of the milestone document.

## Implementation sequence

1. Create `testing` with the harness and no test: the mechanism alone.
2. Move the fixtures and rewrite the mock tests, the two that inject a state last.
3. Rewrite `design@check@api-facade` and `design@check@inputs-are-gathered`.
4. The report on #a1-no-check-below-the-tools, the judgement of C4 and C8; the harvest; the
   deletion of this spec, and of the milestone document.

## Harvest

| what | where |
| --- | --- |
| #testing-crate | `testing`'s design home; `design@check@api-facade` and `design@check@inputs-are-gathered` rewritten |
| the harness kept in `check` and re-exported by `cli`, D2's losing shape | judged by `skill@knowledge-architect-decision-recording@losing-alternatives`; it changed the import path extension writers use, an interface others consume |
