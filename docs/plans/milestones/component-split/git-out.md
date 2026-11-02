# The git invocation layer and the readers of git's objects become the `git` crate

The first slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `git`, crates/git, package `knowledge-architect-git`, with its documents and its
  register homes; its goals are drafted under `skill@knowledge-architect-goal-setting` and put to
  the owner, and the slice merges once the owner has ruled on them.
- `path@elsewhere@crates/core/src/git.rs` moved into it whole, as `path@git@src/git.rs`. Its public API, through a facade in its lib.rs, is
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
- The core depends on `git` by path and by exact version, `=` the workspace's, as it depends on
  `agent-skills`, per `design@knowledge-architect@version-lockstep`: a caret requirement would let
  a consumer resolve a `git` of another patch than the core's. Nothing else changes in the core's
  behaviour.
- `design@git@failed-git-is-an-error` moved to `git` with its exit-code clause reworded: a library
  sets no exit code, so the head states the crate's contract, every invocation a verdict depends on
  fails as an error carrying git's stderr and never as an empty answer, `last_changed` alone
  degrading, and cites `design@core@exit-code-ladder` for the code 2 the core's commands give that
  error. The decision is unchanged; its exit code was always the ladder's.
- The sentences that name the moved file or the crates a binary links, which `cargo klarch check`
  does not report when they turn false, rewritten where they name the list:
  `design@core@single-crate` ("lists this one crate for both, beside the crate of installed text it
  links") and `design@core@checker-source-literals-are-data` ("The core is one crate whose directory
  is its Component"); the sentences of `design@core@staged-tree-source`,
  `design@core@index-staged-write` and `design@core@init-runs-git-init` naming
  `path@elsewhere@crates/core/src/git.rs` are re-pointed only, which touches no head; and the count "publishes four
  crates" of `issue@knowledge-architect@this-repository-warns-on-none-of-the-documentation-lints`.
- The head of absence of #no-intermediate-component, in the root design home: no intermediate
  Component groups the Components. **It waits for the owner's ruling on Q1 of this slice's design
  audit**, which asks whether the component-creation skill's sentence on layering through
  intermediate Components needs a revision, and its title is narrowed to "the new Components", the
  scope #arg-17 argues, per `design@agent-skills@title-states-the-rule`, once Q1 is ruled.
- `design@knowledge-architect@installed-text-crate` rewritten in the form the milestone document states,
  under a new slug, its old slug kept beside the new reference in the commit that renames it, and
  citing `rejected@knowledge-architect@single-published-package`, which no other head cites.
- The publish order: `git` before the core, in both crate lists of
  `path@knowledge-architect@.github/workflows/release.yml`, the verify job's and the publish job's,
  and in `design@knowledge-architect@partial-publish-rerun`, found with the search the milestone
  document's "How the work is done" names.
- A `New features` entry of CHANGELOG.md, surface `library`, the closed list's surface for a
  library API: the crate `knowledge-architect-git`, a minor.

**The first change** sets nothing stricter for an earlier tree: adding crates/git to the compiled
lists of `design@core@checker-source-literals-are-data` and `design@core@a-foreign-build-is-refused`
exempts or compares a directory no earlier tree holds. Step 1 still lands as one commit, since its
intermediate trees do not compile.

## Claims

- **Every git invocation of the core's library code goes through the `git` crate.** Test:
  `git grep -n 'Command::new("git")' crates/core/src` lists none outside a `#[cfg(test)]` module;
  today it lists `path@elsewhere@crates/core/src/git.rs` and one test of `path@core@src/source/rs.rs`. Shown to fail by a recorded
  mutation that adds a `Command::new("git")` call to a module of the core's library.
- **The environment every invocation runs with is unchanged**: `core.excludesFile` at the null
  device, the repository variables dropped and `GIT_INDEX_FILE` kept. Test: the test of the
  per-user ignore file in `path@core@tests/binary.rs` and the tests of those heads, moved with the
  code; shown to discriminate by a recorded mutation that drops the pin of `core.excludesFile`,
  which that test fails on.
- **The core's tests and the binary's tests pass with no change but what the crate boundary
  forces.** Test: `cargo test --workspace`, and a recorded mutation of a moved function's
  behaviour fails a test of the core. The core names the crate `crate::git`, so no import path
  changes. The diff of the test files is three changes, each forced: a unit test of the core reads a
  snapshot entry's path through `SnapshotEntry::entry`, since the field is the crate's; and the two
  binary tests that assert what the binary prints about its linked crates name the `git` crate,
  which the binary now lists, per `design@core@checker-source-literals-are-data` and
  `design@core@a-foreign-build-is-refused`.
- **A tree holding the `git` crate at another path than the binary's is refused**, as one holding
  the core is. Test: a binary test over a copy declaring `knowledge-architect-git` under
  crates/git, shown to fail by a recorded mutation that lists the installed-text crate in its
  place.

## Audit subjects

`path@git@src/git.rs`; `design@git@git-is-asked-through-one-module`, rewritten here to name the
crate; the git heads the milestone document lists as moving whole, with
`rejected@git@every-git-variable-dropped`, which `design@git@index-file-variable-kept` cites;
`design@core@git-supplies-the-walk`, `design@core@links-are-no-documents` and
`design@core@staged-tree-source`, which stay with the walk and the snapshot's assembly, and
`tripwire@core@walked-count-differs-between-machines`, which stays with the walk;
`issue@git@index-staged-clears-an-entrys-flags` and
`issue@git@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`, re-anchored;
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
   `design@git@git-is-asked-through-one-module` rewritten, `rejected@git@every-git-variable-dropped`,
   and the two issues above.
3. The head of absence, `design@knowledge-architect@installed-text-crate` and the publish order.
4. The harvest.

## Harvest

| what | where |
| --- | --- |
| #git-crate | `git`'s design home: the moved heads, `design@git@git-is-asked-through-one-module` rewritten to name the crate |
| #no-intermediate-component | its head of absence in `register@knowledge-architect@design` |
| the milestone's table | `design@knowledge-architect@installed-text-crate`, rewritten and renamed; `design@knowledge-architect@partial-publish-rerun`, for the publish order |
