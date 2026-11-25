---
kind: defect
---
# The tests of `check`, `cli` and `testing` read the machine's git configuration, so a machine setting can fail a correct tree

## Summary

Several test helpers of `check`, `cli` and `testing` run git with the machine's own configuration. A global
`commit.gpgsign` that cannot sign, or a global `core.hooksPath` whose `pre-commit` hook fails,
fails dozens of tests on a correct tree, and a per-user ignore file decides which fixture files one
helper stages. Only `Sandbox::git_command` in `path@testing@tests/binary.rs` is isolated, and only for
the identity.

## Details

### What

Measured by the adversarial reviewer of the review before the merge of the branch that gave
`Sandbox::git_command` the sandbox's identity, at that branch's tip, with the machine's real
configuration untouched: each probe set `GIT_CONFIG_GLOBAL` to a file of its own and
`GIT_CONFIG_NOSYSTEM=1`, then ran `cargo test --no-fail-fast` over the package that then held all these tests, which today is
`-p knowledge-architect-check -p knowledge-architect -p knowledge-architect-testing`.

- A global configuration holding `commit.gpgsign = true` and `gpg.program = false`: 9 failures in
  the library's unit tests, 55 in `path@testing@tests/binary.rs`, 1 in `path@testing@tests/extension_api.rs`; the test
  `a_sandbox_commit_carries_the_sandbox_identity` fails with "gpg failed to sign the data".
- A global `core.hooksPath` pointing at a `pre-commit` hook that exits 1: the same counts, 9, 55
  and 1.
- Adding `-c commit.gpgsign=false -c core.hooksPath=/dev/null` to `Sandbox::git` alone
  brought `path@testing@tests/binary.rs` down to 43 failures under either probe; the other 43 run git through
  `History::git` and other direct `Command::new("git")` calls.
- The per-user ignore file: `Sandbox::stage` pins `core.excludesFile=/dev/null`, and so do
  `History::commit` and the helper of `path@testing@tests/extension_api.rs`. One `git add -A` call of
  `path@testing@tests/binary.rs`, through `History::git`, does not. Found by reading by the self-consistency reviewer of the same review; no test was shown failing.
- Not established: which other settings a test reads, and how the library's unit-test repositories
  in `path@git@src/git.rs` and `path@check@src/commands/history.rs` are configured. The failure counts
  above were not re-run by the session that opened this entry.

A failure in real use, the first: the test file of `init`, `path@cli@tests/init.rs`, runs git and
the binary through helpers of its own, with no isolation. One case, which relies on git refusing a
repository as another user's, passed on the workstation and failed in CI; it reproduced locally
under a global configuration holding `safe.directory = *`. The commit "The test of a repository git
refuses as another user's keeps the machine's git configuration out" isolated that one invocation
with `GIT_CONFIG_NOSYSTEM=1` and an empty `GIT_CONFIG_GLOBAL`, and its comment cites this entry;
the file's other helpers still read the machine's configuration. That the CI runner's configuration
holds the setting is not established.

Probed by an adversarial reviewer over `path@testing@tests/binary.rs`, `path@testing@tests/checks_over_mocks.rs`, `path@testing@tests/model_over_mocks.rs` and `path@cli@tests/fixture_pin.rs`, each with `GIT_CONFIG_GLOBAL` set and `GIT_CONFIG_NOSYSTEM=1`:

- `init.templateDir` whose `info/exclude` holds `*`: 142 failures in the first and 16 in the second. Pinning `core.excludesFile` does not cover it.
- `core.excludesFile` with `*.md` and `*.toml`: 7 failures, through `Sandbox::git`.
- `init.defaultObjectFormat = sha256`: 1 failure, `a_gitlink_is_a_phase_two_finding_and_an_exclude_row_declares_the_silence`, whose `--cacheinfo` holds a 40-hex object id.
- `GIT_INDEX_FILE=.git/index` over `path@testing@tests/binary.rs`, as a commit hook sets it: 3 failures, which were not inspected. The helpers `git` and `Sandbox::git_command` remove that variable, and a run of the binary keeps it, as `design@git@index-file-variable-kept` asks.

The variables that name a repository are a setting of the same kind, which a commit hook gives a suite, and a
helper that keeps them writes into the caller's repository. Reproduced by an adversarial reviewer: with
`GIT_DIR` naming a throwaway repository, `path@testing@tests/extension_refusals.rs` replaced its index, from 1
entry to 40, and `path@xtask@tests/line_endings.rs` did too and passed. Both drop `GIT_DIR`, `GIT_INDEX_FILE` and
`GIT_WORK_TREE` now, as do the helpers of `path@testing@tests/extension_api.rs`, `path@xtask@tests/gates_bin.rs` and
`path@xtask@tests/mutate_bin.rs`. Under the first two variables the test of extension_refusals.rs still fails, since it
runs the checker in process, which keeps `GIT_INDEX_FILE` as `design@git@index-file-variable-kept` asks, and
leaves that index as it was. `git` of `path@cli@tests/init.rs`, and `Sandbox` and `linked_worktree` of
`path@mutate@tests/modes.rs`, drop none; read, not run.

### Why it matters

A contributor or a CI runner with commit signing or a hooks directory set globally sees `check`'s
suite fail on a correct tree, and a per-user ignore rule can change what a fixture stages, so a test
can pass or fail by the machine it runs on. The gate `cargo x gates` then reports a failure that is
no defect of the change under test: on such a machine, its exit code cannot be trusted. It strains
no recorded decision.

### What would close it

Every helper of the tests of `check`, `cli` and `testing` that runs git goes through one function that isolates the
configuration a test does not mean to read: the identity as `Sandbox::git_command` does, plus at
least `commit.gpgsign=false`, `core.hooksPath=/dev/null` and `core.excludesFile=/dev/null`, or a
`GIT_CONFIG_GLOBAL` pointing at an empty file with `GIT_CONFIG_NOSYSTEM=1`. The two probes above
then pass, run as a test or by hand, and `path@testing@CLAUDE.md` states the contract for
every helper. The unit tests of `path@git@src/git.rs`, which build repositories of their own, are
held to the same function or to one of the git crate's, and `path@git@CLAUDE.md` states it.
