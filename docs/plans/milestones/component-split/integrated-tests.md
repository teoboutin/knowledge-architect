# The shared test Component holds the fixtures and the integrated tests

The sixth and last slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `testing`, crates/testing, package `knowledge-architect-testing`, depending on
  `model` and `check`. Its library publishes the harness an extension's tests need, what
  `path@core@src/testing.rs` exposes today: `foundation`, `run_with`, `Phase`, `Report`, `Stop`,
  `Structure`, `CHECKS`. Its `tests/` holds this repository's mock projects and the integrated
  tests, which its `include` does not ship.
- The 36 mock tests of `path@core@src/mock_projects.rs`, and the integrated tests of
  `path@core@tests/extension_api.rs`, rewritten to drive the crates through their public API.
  A test whose claim no consumer could observe becomes a unit test over a model built in memory,
  per `design@core@model-then-checks`; the 5 that build `Inputs` by hand each take one of the two
  ways, a mock project whose tree holds the state, or a unit test.
- `design@core@api-facade` rewritten: its sentence "A test that needs a private item is a unit-test
  module inside the crate" gives way to the guideline of `spec@plans@component-rules-revision`.
- `testing`'s directory on the list of the binary's own source, so `"fixture"` is accepted for its
  mock projects, per `design@core@checked-sentinel-values`.

## Claims

- **No unit test of any crate uses a mock project.** Test: `git grep -n 'tests/projects'
  crates/*/src` lists none outside `testing`; refuted by any.
- **Every integrated test reaches the crates through their public API.** Test: the tests sit in
  another crate than the code, so the compiler refuses a private item; an item an integrated test
  needs and cannot reach is opened as `tripwire@core@private-item-needed`'s response says.
- **No package ships a fixture.** Test: `cargo package --list -p knowledge-architect-testing` lists
  no file under `tests/`.
- **Every claim the 36 mock tests made is still tested.** Test: the slice's commit lists each test,
  old and new, by name.

## Fixtures

The mock projects of `path@core@tests/projects/`, moved; a new mock project for each state one of
the 5 tests injected by hand, such as a tree with no committed index, in the vocabulary of the
existing ones.

## Audit subjects

`design@core@api-facade`, `design@core@inputs-are-gathered`, `design@core@checked-sentinel-values`,
`design@knowledge-architect@package-include-whitelist`; `rejected@core@inputs-struct-literal`,
`rejected@core@doc-hidden-test-items`, `rejected@core@testing-feature-for-test-items`,
`rejected@core@unit-tests-moved-under-tests`; `tripwire@core@private-item-needed`;
`issue@core@the-tests-read-the-machine-s-git-configuration`, whose helpers move here. Each
`@core@` reference here names the entry as this document is written; #check-renamed re-points them.

## Fails alone on

A claim of the old mock tests no test makes any more, or a fixture in a package.

## Premises that expire

None: this slice ends the premises of #model-out and #consumers-out on where the mock tests run.

## Decided design

The owner's argument: integrated tests out of unit tests and out of the crates test the public API,
and one home for the fixtures keeps crates from duplicating them or reaching into another's
(#arg-37, #arg-41); the harness is published because extensions are supported (#arg-40). The
fixtures stay unshipped (#arg-87). The scope is the integrated test, so the 437 unit tests stay
where they are (#arg-83, #arg-107, #arg-108). What an integrated test cannot observe is hidden from
consumers too (#arg-101), and the fix goes through `tripwire@core@private-item-needed`'s response
(#arg-109). The nearest rival, the mock tests kept as a unit-test module, is a losing alternative of
the milestone document.

## Implementation sequence

1. Create `testing` with the harness and no test: the mechanism alone.
2. Move the fixtures and rewrite the mock tests, the 5 that build `Inputs` last.
3. Rewrite `design@core@api-facade`.
4. The milestone's layout head, the report on #a1-no-check-below-the-tools, the judgement of C4, C5
   and C8; the harvest; the deletion of this spec, and of the milestone document once D1 is ruled.

## Harvest

| what | where |
| --- | --- |
| #testing-crate | `testing`'s design home; `design@core@api-facade` rewritten |
| the milestone's layout | its row in the milestone document's harvest |
