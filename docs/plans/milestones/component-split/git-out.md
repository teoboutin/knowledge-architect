# The git invocation layer and the readers of git's objects become the `git` crate

The first slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `git`, crates/git, package `knowledge-architect-git`, with its documents, its
  register homes and its goals, stated with the owner.
- `path@core@src/git.rs` moved into it whole: the invocation builder `git::Invocation`, the
  listings, the snapshot readers (`snapshot_entries`, `blobs`, `blob_bytes`, `commit_message`,
  `rev_list`, `rev_parse`), `stage_generated`, `last_changed`, and the queries of the ignore rules.
  Its items become the crate's public API through a facade, per `design@core@api-facade`'s rule
  applied to the new crate.
- The core depends on `git`; nothing else changes in the core's behaviour.

## Claims

- **Every git invocation of the core goes through the `git` crate.** Test: `git grep -n
  'Command::new("git")' crates/core/src` lists none outside test helpers; refuted by any hit in
  library code.
- **The environment every invocation runs with is unchanged**: `core.excludesFile` at the null
  device, the repository variables dropped and `GIT_INDEX_FILE` kept. Test: the existing tests of
  those heads pass, moved with the code; shown to discriminate by a recorded mutation that drops
  the pin of `core.excludesFile`, which a test fails on.
- **The core's tests and the binary's tests pass unchanged**, apart from import paths.

## Audit subjects

`path@core@src/git.rs`; `design@core@git-is-asked-through-one-module`, rewritten here to name the
crate; the git heads the milestone lists as moving whole; `issue@core@index-staged-clears-an-entrys-flags`
and `issue@core@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`, re-anchored;
`path@xtask@tests/checkout.rs`, which asserts the checkout tie of every target.

## Fails alone on

A git invocation left in the core's library code, or a test of the environment that fails after
the move.

## Premises that expire

The core still holds every caller of the crate; #model-out moves most of them into `model`.

## Decided design

`git` is a Component of its own because its interface changes for reasons of its own: a git
query or a minimum version the later history rewriting needs and the checker does not (#arg-5),
and it takes decisions of its own, the heads that move (#arg-6). Without it, the rewriting work
would depend on the whole checker to reach a module that is crate-private today (#arg-7). The name
stays `git`: the one backticked `git@host:...` span of the tree is data of the checker's own
source (#arg-97). The nearest rival, the module kept in the core and made public, puts the
checker's parsers into the rewriting crate's build.

## Implementation sequence

1. Create the Component and move the module, the core depending on it.
2. Move the heads, the issues and the tripwires whose subject is git into `git`'s homes.
3. The harvest.

## Harvest

| what | where |
| --- | --- |
| #git-crate | `git`'s design home: the moved heads, `design@core@git-is-asked-through-one-module` rewritten to name the crate |
