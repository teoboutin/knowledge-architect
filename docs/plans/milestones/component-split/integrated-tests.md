# The shared test Component holds the fixtures and the integrated tests

The sixth and last slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

As D2 of the milestone document and the threads below rule it:

- The Component `testing`, crates/testing, package `knowledge-architect-testing`, published. Its
  library holds the harness an extension's tests need: `foundation`, `run_with`, `Phase`, `Report`,
  `Stop`, `Structure`, `CHECKS`. It depends on `check` for the harness, and, as
  dev-dependencies by path alone, on `cli`, `model` and `agent-skills` for its tests; no crate depends on it. Its
  `tests/` holds this repository's mock projects, the transcripts' fixtures and the tests over them,
  which its `include` does not ship.
- `cli`'s interim re-export of the harness removed. `check`'s role module `testing` becomes the role module
  `run`, `path@check@src/run.rs`, since the harness re-exports items only `check` can name; its section
  "Testing an extension" moves from `path@cli@src/lib.rs` to `testing`'s crate documentation, and the
  setup skill's pointer to it follows. A Migration entry of the changelog names the new dev-dependency and import
  of an extension's tests, and the entry of the `Next release` section naming the old path takes the new one.
- The tests of `path@testing@tests/binary.rs`, 198 of them, and of `path@testing@tests/extension_api.rs`, 9, move to `testing`,
  per #binary-tests-in-testing, and the 36 mock tests, 20 in `path@testing@tests/checks_over_mocks.rs` and 16 in
  `path@testing@tests/model_over_mocks.rs`, are rewritten to drive the crates through their public API, as an
  extension's test does. A test whose claim no consumer could observe becomes a unit test over a
  model built in memory, per `design@check@model-then-checks`, or names why it leaves. The state their 3 struct
  literals of `Inputs` and 2 of `TreeInputs` injected is a copy of the mock whose tree holds that state.
  `path@cli@tests/init.rs` stays in `cli`, since it uses no mock project, and `path@cli@tests/fixture_pin.rs`, new, holds the acceptance of
  `"fixture"` over `cli`'s own directory.
- `[walk] exclude` of the manifest re-pointed, per #test-files-outside-the-walk.
- `design@check@the-facade-publishes-what-a-consumer-needs` and `design@model@model-facade` rewritten: their sentence that a test that
  needs a private item is a unit test gives way to the guideline of the code-shape fragment; "Nothing becomes
  public for the core's own tests" stays: an item becomes public because a consumer needs it, which
  an integrated test can show. Their role tables and their restatement in the crates' CLAUDE.md follow.
- `design@check@inputs-are-gathered` rewritten for its sentence on the mock tests that assemble
  `Inputs` by hand, which this slice ends. `design@knowledge-architect@the-binary-crate-keeps-the-package-name` and
  `design@check@checker-source-literals-are-data` rewritten for what the threads below make false. `model`'s
  `Entities::definition_findings` becomes public: phase 3's findings have an effect a consumer can see, and an integrated test could
  not observe them over a project that stops at phase 2.
- The contracts of the mock projects and of the test helpers move from the `CLAUDE.md` of `check` and of `cli` to `testing`'s.
- What "How the work is done" owes for a new published crate, but the lists of
  `design@check@a-foreign-build-is-refused` and `design@check@checker-source-literals-are-data`, since `klarch` links nothing of
  `testing`. It goes after `check` and `model` in the publish order.

## Claims

- **No unit test of any crate loads a mock project.** Test: `git grep -n 'tests/projects' crates/*/src`
  read whole: each hit is a comment, a scratch path of a test or the walk's own literal. Shown to fail by a recorded unit
  test that joins the path.
- **Every integrated test reaches the crates through their public API.** Test: the tests sit in
  another crate than the code, so the compiler refuses a private item; an item an integrated test
  needs with an effect a consumer can see is published, and otherwise the claim is a unit test's.
  A crate of this workspace fires no `tripwire@check@private-item-needed`, per D10.
- **No package ships a fixture.** Test: `cargo package --list` of every published crate,
  `knowledge-architect-testing` included, lists no file under tests/projects or tests/transcripts;
  shown to fail by a recorded `include` row naming one.
- **An extension's tests reach the harness through `knowledge-architect-testing` alone.** Test:
  `path@testing@tests/extension_api.rs` imports the harness from that crate, `path@cli@tests/public_paths.rs` names no `testing` path, and
  `cargo tree -e normal -p knowledge-architect` lists no `knowledge-architect-testing`. A mutation of the dependency
  and the re-export together spans two files, which `design@mutate@one-file-per-run` refuses, so the test of the dependency is shown by a recorded row in crates/cli/Cargo.toml alone.
- **Every claim the moved tests made is still tested.** Test: a commit of the slice lists each of the 36 mock tests with
  the test that now carries its claim, and each test of the binary that changed its claim; a claim with no successor
  refutes it.
- **The real binary runs under the invocations the gates use.** Test: `cargo --locked x gates --locked --fail-fast` and `cargo test --release -p knowledge-architect-testing --test binary` pass; a binary that does not build fails a test, shown by a recorded mutant.
- **A test of logic the slice rewrites keeps its power.** Test: a recorded mutation of the logic behind each family the rewrite touched: the generated files' state, phase 2 through `foundation`, phase 3, and the pin over `cli`'s directory.

## Fixtures

The mock projects of `path@testing@tests/projects/` and the transcripts of `path@testing@tests/transcripts/`, moved. No new mock project: a state a literal injected is made in a copy.

## Audit subjects

`design@check@the-facade-publishes-what-a-consumer-needs`, `design@check@inputs-are-gathered`; `design@check@checked-sentinel-values`
and `design@check@checker-source-literals-are-data`, with every `rejected` entry they cite;
`design@knowledge-architect@package-include-whitelist`; `rejected@check@inputs-struct-literal`,
`rejected@check@doc-hidden-test-items`, `rejected@check@testing-feature-for-test-items`,
`rejected@check@unit-tests-moved-under-tests`; `tripwire@check@private-item-needed`;
`issue@check@the-tests-read-the-machine-s-git-configuration`, whose helpers move here.

## Fails alone on

A claim of the old mock tests no test makes any more, a fixture in a package, or a test that cannot build `klarch`.

## Premises that expire

None: this slice ends the premises of #model-out and #consumers-out on where the mock tests run,
and of #cli-on-top on its interim re-export of the harness and on where its tests find their
fixtures.

## Threads

### The tests of the binary sit in `testing` and build the real `klarch` with a nested cargo `##binary-tests-in-testing`

Proposed by the owner, at this slice's audit. Final state: approved. The audit opened the point the spec left load-bearing. The session measured that an in-process run cannot reach what these tests hold (#arg-119), proposed S1, the fixtures reached by a dev-dependency of `cli` on `testing` with `testing`'s on `cli` removed, and the owner answered "Q1 maybe thaum testing should also centralise the tests of cli that need mock projects.", which the session read as this repository's `testing`, and measured possible (#arg-120). Put as S5 against S1, the owner: "And agreed on q1 revised". The transcripts' fixtures follow, since `path@testing@tests/binary.rs` is their one reader. Shape: Builds. Harvest: `design@testing@binary-tests-in-testing`.

### `testing`'s test files are outside this repository's walk `##test-files-outside-the-walk`

Proposed by the session as L2, one of three shapes, at this slice's audit, after the owner asked "And what is the problem with the literal exemption ? The mocks should never be considered for anything when the checks are running on the real knowledge architect project itself". Final state: approved. The owner: "Q3: take the easiest to implement solution here. I want to reopen a discussion later on whether literals in code should even ever be considered for linting and for anything else that is being scanned. This is just weird I think. No one would even try to reference entries with literals. I am not even sure why wee are doing that right now. Open this as a design issue". The session named L2 the easiest (#arg-121), and first told the owner it changes no rule, which #arg-122 corrects. Shape: Builds. Harvest: `design@testing@test-files-outside-the-walk`, and `issue@model@string-literals-in-code-may-not-need-reading-at-all`.

## Arguments

### An in-process run reaches neither the printed report nor the parse `##arg-119`

The session, at this audit. Threads: #binary-tests-in-testing. "`cli::run` prints to the real stdout, and a closed stdout exits the process. `Cli::parse` is in `main.rs`, so clap's exit 2 cannot be reached."

### A test of another package can build the real binary `##arg-120`

The session, at this audit. Threads: #binary-tests-in-testing. In a scratch workspace, `CARGO_BIN_EXE_` is unset in another package and a dev-dependency does not build the binary, `artifact =` requires `-Z bindeps`, and a test ran `$CARGO build -p a --bin abin` and then the binary it built.

### Each other shape costs more to build `##arg-121`

The session, at this audit. Threads: #test-files-outside-the-walk. L2 is one manifest row. L1 binds each literal: 24 findings in the two mock test files, with phase 4 of `model`'s unjudged, and 26 in `path@testing@tests/binary.rs` at phase 3 alone, measured by copies under the directory. L3, `klarch` built from this checkout naming the directory as its own source, reopens two heads and builds a compiled path.

### The row does touch heads `##arg-122`

The session, at this audit, on the readers of the search. `design@check@checker-source-literals-are-data` says "only the checker's own source can ever be exempt", with `rejected@check@exempt-directory-manifest-row`, and its test modules' pointers "stay checked". The row exempts no literal of a file the walk reads and leaves the file out whole, as any project's row can today, so it reverses neither; but 60 comment lines lose their check, and the head states the limit. L2 stays the easiest, since L3 rewrites the same heads and L1 costs at least 50 edits, so the owner's word holds.

## Decided design

The owner's argument: integrated tests out of unit tests and out of the crates test the public API,
and one home for the fixtures keeps crates from duplicating them or reaching into another's
(#arg-37, #arg-41); the harness is published because extensions are supported (#arg-40). The
fixtures stay unshipped (#arg-87). The scope is the integrated test, so the 437 unit tests stay
where they are (#arg-83, #arg-107, #arg-108). The harness's items are public in `check` under its role module `run`, whose name the audit gave it; `testing` depends on `check`, so nothing else names them. What an integrated test cannot observe is hidden from
consumers too (#arg-101). The harness is published by `testing` itself, which an extension's tests
depend on separately, by the owner's choice at D2. The nearest rival, the mock tests
kept as a unit-test module, is a losing alternative of the milestone document.

## Implementation sequence

1. Create `testing` with the harness and no test: the mechanism alone.
2. Move the fixtures and the tests, re-point the manifest's row and every reference, and rewrite the mock tests. Each tree must pass, so this lands with step 1 in one change, and a commit of the slice lists the claims.
3. Rewrite the heads the Builds name, and write the goals, once ruled.
4. The report on #a1-no-check-below-the-tools, the judgement of C4 and C8; the harvest.
5. After the review before the merge has read that tip, the deletion of this spec and of the milestone document, each cited by its kind, the roadmap's row and the citations of them in the issues repaired, in a commit of its own.

## Harvest

| what | where |
| --- | --- |
| #testing-crate | that `testing` exists and how it depends on the other crates: the root's design home, `design@knowledge-architect@testing-crate`, and `design@knowledge-architect@mock-projects-in-one-crate`, a decision of its own, whose rival, the mock tests kept as a unit-test module, earns no entry, since an argument any reader reconstructs refutes it; `design@check@the-facade-publishes-what-a-consumer-needs`, `design@model@model-facade` and `design@check@inputs-are-gathered` rewritten |
| #binary-tests-in-testing, #test-files-outside-the-walk | `testing`'s design home, under those slugs, with the build of the binary in `design@testing@nested-cargo-builds-the-binary`, a decision of its own; `design@check@checker-source-literals-are-data` and `design@knowledge-architect@the-binary-crate-keeps-the-package-name` rewritten |
| the harness kept in `check` and re-exported by `cli`, D2's losing shape | judged by `skill@knowledge-architect-decision-recording@losing-alternatives`; it changed the import path extension writers use, an interface others consume |
| S1 | judged by the same tests; named in `design@testing@binary-tests-in-testing` as its nearest rival |
| L1, L3 | the deliberation only: they lost while `issue@model@string-literals-in-code-may-not-need-reading-at-all` keeps the question open |
| C4, C8, #a1-no-check-below-the-tools | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires`, in the harvest's commit |
| the issues the move or the new crate makes false | each re-pointed or reworded with its sentence: the helpers' issues, the lists of published crates, the sentence on `"fixture"` |
