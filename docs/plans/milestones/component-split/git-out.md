# The git invocation layer and the readers of git's objects become the `git` crate

The first slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `git`, crates/git, package `knowledge-architect-git`, with its documents and its
  register homes; its goals are drafted under `skill@knowledge-architect-goal-setting` and put to
  the owner, and the slice merges once the owner has ruled on them.
- `path@core@src/git.rs` moved into it whole. Its public API, through a facade in its lib.rs, is
  every item the core calls today, counted with
  `grep -rn 'git::' crates/core/src --include=*.rs | grep -v 'src/git.rs'`: the builder
  `git` and the type `Invocation` with its methods; `inside_work_tree`, `ignores`,
  `init_repository`; the listings `entries`, `tracked_and_ignored`, `unmerged`,
  `index_differs_from_head`; the ignore batch `ignore_query` and `ignored`; `last_changed`,
  `rev_list`, `rev_parse`, `main_worktree_path`; the snapshot readers `Source`, `SnapshotEntry`,
  `snapshot_entries`, `object_name`, `blobs`, `blob_bytes`, `commit_message`; `stage_generated`;
  the types `Entry` and `EntryKind`. `Entry` and `EntryKind` keep their public path for an
  extension, `extension::Entry` and `extension::EntryKind`, which `path@core@src/extension.rs`
  re-exports.
- The core depends on `git`; nothing else changes in the core's behaviour.
- The root design home's head on the layout, written here: which Components exist, `git` among
  them, and that no intermediate Component groups them. Each later slice extends it.
- `design@knowledge-architect@two-crates` rewritten in the form the milestone document states.
- The publish order: `git` before the core, in `path@knowledge-architect@.github/workflows/release.yml`
  and in `design@knowledge-architect@partial-publish-rerun`, found with the search the milestone
  document's "How the work is done" names.

## Claims

- **Every git invocation of the core's library code goes through the `git` crate.** Test:
  `git grep -n 'Command::new("git")' crates/core/src` lists none outside a `#[cfg(test)]` module;
  today it lists `path@core@src/git.rs` and one test of `path@core@src/source/rs.rs`. Shown to fail by a recorded
  mutation that adds a `Command::new("git")` call to a module of the core's library.
- **The environment every invocation runs with is unchanged**: `core.excludesFile` at the null
  device, the repository variables dropped and `GIT_INDEX_FILE` kept. Test: the test of the
  per-user ignore file in `path@core@tests/binary.rs` and the tests of those heads, moved with the
  code; shown to discriminate by a recorded mutation that drops the pin of `core.excludesFile`,
  which that test fails on.
- **The core's tests and the binary's tests pass with no change but their import paths.** Test:
  `cargo test --workspace`; a diff of the test files limited to `use` lines shows the claim, and a
  recorded mutation of a moved function's behaviour fails a test of the core.

## Audit subjects

`path@core@src/git.rs`; `design@core@git-is-asked-through-one-module`, rewritten here to name the
crate; the git heads the milestone document lists as moving whole, with
`rejected@core@every-git-variable-dropped`, which `design@core@index-file-variable-kept` cites;
`design@core@git-supplies-the-walk`, `design@core@links-are-no-documents` and
`design@core@staged-tree-source`, which stay with the walk and the snapshot's assembly, and
`tripwire@core@walked-count-differs-between-machines`, which stays with the walk;
`issue@core@index-staged-clears-an-entrys-flags` and
`issue@core@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`, re-anchored;
`path@xtask@tests/checkout.rs`, which asserts the checkout tie of every target.

## Fails alone on

A git invocation left in the core's library code, or a test of the environment that fails after
the move.

## Premises that expire

The core still holds every caller of the crate until #model-out moves most of them into `model`;
the claim on invocations is taken again there, over both crates.

## Decided design

`git` is a Component of its own because its interface changes for reasons of its own: a git
query or a minimum version the later history rewriting needs and the checker does not (#arg-5),
and it takes decisions of its own, the heads that move (#arg-6). Without it, the rewriting work
would depend on the whole checker to reach a module that is crate-private today (#arg-7). The name
stays `git`: on `origin/main` the one backticked span whose head is that word is data of the
checker's own source (#arg-97). The nearest rival, the module kept in the core and made public,
puts the checker's parsers into the rewriting crate's build.

## Implementation sequence

1. Create the Component and move the module behind its facade, the core depending on it.
2. Move into `git`'s homes the heads the milestone document lists as moving whole, with
   `design@core@git-is-asked-through-one-module` rewritten, `rejected@core@every-git-variable-dropped`,
   and the two issues above.
3. The layout head, `design@knowledge-architect@two-crates` and the publish order.
4. The harvest.

## Harvest

| what | where |
| --- | --- |
| #git-crate | `git`'s design home: the moved heads, `design@core@git-is-asked-through-one-module` rewritten to name the crate; the root's layout head, written |
