# The shared test Component holds the fixtures and the integrated tests

The sixth and last slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

Under D2's default, as the milestone document states it; the owner's ruling on D2 may change the
first two bullets, and the slice is built on the ruling it has.

- The Component `testing`, crates/testing, a crate no other crate depends on and no package ships
  (`publish = false`), depending on `cli`, `check` and `model`. Its `tests/` holds this
  repository's mock projects and its integrated tests. The harness an extension's tests need stays
  `check`'s role module `testing`, which `cli` re-exports.
- The 36 mock tests of `path@core@src/mock_projects.rs`, and the integrated tests of
  `path@core@tests/extension_api.rs`, rewritten to drive the crates through their public API, as
  an extension's test does. A test whose claim no consumer could observe becomes a unit test over a
  model built in memory, per `design@core@model-then-checks`. `mock_projects.rs` holds 5 struct
  literals of `Inputs`, two of them in the shared helpers `findings_with` and `phase_two`, which 11
  tests reach; two literals inject a state, an empty map of committed files: each of those becomes
  a mock project whose tree holds no committed generated file, or a unit test.
- `design@core@api-facade` rewritten: its sentence "A test that needs a private item is a unit-test
  module inside the crate" gives way to the guideline of the code-shape fragment; its bullet
  "Nothing becomes public for the core's own tests" stays: an item becomes public because a
  consumer needs it, which an integrated test can show. Its restatement in the crate's CLAUDE.md is
  rewritten with it.
- `design@core@inputs-are-gathered` rewritten for its sentence on the mock tests that assemble
  `Inputs` by hand, which this slice ends.
- The root's layout head extended with `testing`.

## Claims

- **No unit test of any crate loads a mock project.** Test:
  `git grep -n 'join("tests/projects")' crates/*/src` lists none; today it lists the helper of
  `path@core@src/mock_projects.rs`. Shown to fail by a recorded unit test that joins the path.
- **Every integrated test reaches the crates through their public API.** Test: the tests sit in
  another crate than the code, so the compiler refuses a private item; an item an integrated test
  needs and cannot reach shows an item a consumer needs, and is opened as a `todo` under the role
  module its use belongs to. `tripwire@core@private-item-needed` is unchanged.
- **No package ships a fixture.** Test: `cargo package --list -p knowledge-architect-testing`
  refuses, since the crate is not published, and `cargo package --list` of every published crate
  lists no file under a directory named tests/projects; shown to fail by a recorded `include` row naming
  one.
- **Every claim the 36 mock tests made is still tested.** Test: the slice's commit lists each old
  test with the test that now carries its claim; a claim with no successor refutes it.

## Fixtures

The mock projects of `path@core@tests/projects/`, moved; a new mock project for each of the two
states injected by hand, in the vocabulary of the existing ones.

## Audit subjects

`design@core@api-facade`, `design@core@inputs-are-gathered`, `design@core@checked-sentinel-values`,
which accepts `"fixture"` strictly inside the core's compiled directory or a directory the binary
names as its own source, and `rejected@core@sentinels-accepted-everywhere`, whose doubt this move
meets: the binary's tests stay in `cli`, the one package where `CARGO_BIN_EXE_klarch` is set, and
run over the mock projects, which move to `testing`; how they keep reaching them, and keep
`"fixture"` accepted there, is decided at this audit;
`design@knowledge-architect@package-include-whitelist`; `rejected@core@inputs-struct-literal`,
`rejected@core@doc-hidden-test-items`, `rejected@core@testing-feature-for-test-items`,
`rejected@core@unit-tests-moved-under-tests`; `tripwire@core@private-item-needed`;
`issue@core@the-tests-read-the-machine-s-git-configuration`, whose helpers move here. Each
reference naming the anchor `core` here names the entry as this document is written;
#check-renamed re-points them.

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
consumers too (#arg-101). Where the harness is published is D2. The nearest rival, the mock tests
kept as a unit-test module, is a losing alternative of the milestone document.

## Implementation sequence

1. Create `testing` with no test: the mechanism alone.
2. Move the fixtures and rewrite the mock tests, the two that inject a state last.
3. Rewrite `design@core@api-facade` and `design@core@inputs-are-gathered`; extend the layout head.
4. The report on #a1-no-check-below-the-tools, the judgement of C4 and C8; the harvest; the
   deletion of this spec, and of the milestone document once D2 is ruled.

## Harvest

| what | where |
| --- | --- |
| #testing-crate | `testing`'s design home; `design@core@api-facade` and `design@core@inputs-are-gathered` rewritten |
