# Slice 1: the mutation crate, a published crate run as `cargo x mutate`

## Builds

1. **The Component `mutate`**, directory `planned@knowledge-architect@crates/mutate/`, package
   `knowledge-architect-mutate`, a workspace member and the fourth published crate, created with
   `cargo klarch new-component` and carrying the documents every Component carries. Its goals are
   drafted under `skill@knowledge-architect-goal-setting` and ruled by the owner at this slice.
2. **Its code**, taken from the generic `run` and `restore` of thaum's mutation tool: the library
   `path@elsewhere@~/dev/games/thaum/tools/mutate/src/lib.rs` and, from
   `path@elsewhere@~/dev/games/thaum/tools/mutate/src/main.rs`, `run_hand_mutation`, `unwind`,
   `restore_record` and `apply_to_scratch`, with their tests, and the behaviour its README states
   under "The hand mutation". Without `sweep`, without the `THAUM_CHECKOUT` tie, and with the
   thaum-specific parts the audit lists replaced: the record directory and the lookup of the project
   root.
3. **`cargo x mutate`** in this repository's maintenance crate, `path@knowledge-architect@tools/xtask/`,
   depending on the mutation crate directly, not through the gates library, per
   #mutation-command-host.
4. **The setup skill's maintenance-crate snippet** gains the dependency and the subcommand, in
   `path@agent-skills@snippets/xtask-cargo.toml` and `path@agent-skills@snippets/xtask-main.rs`, each
   tested as `design@agent-skills@rust-templates-one-home` says; `cargo klarch install-agent-skills`
   run.
5. **`skill@klarch-release`** lists the fourth crate in its package lists, version rules and publish
   order; the first version of the crate is published by hand with an API token, per
   `design@knowledge-architect@publish-credential`, which the slice rewrites.
6. **The CHANGELOG.md entries** in `Next release`, and `cargo x changelog`, which writes the copy the
   new crate ships too, per `design@knowledge-architect@the-changelog-ships-in-every-crate`.

The local development skill's worktree procedure is replaced by the command at slice 2, not here.

## Claims

Each claim's test is a test of the mutation crate, shown to fail by a recorded mutation of the code
it guards, run with the procedure `skill@klarch-development@discriminating-tests` gives today, then,
once the command exists, with `cargo x mutate run` on a second claim as its first use.

- **A run killed mid-mutation is restored by `restore`, and no content is lost**, per
  `design@knowledge-architect@no-operation-loses-content` and premortem cause 8. Test: start a run
  whose command blocks, kill it, run `restore`, and compare the file byte for byte with its
  pre-image, uncommitted edits elsewhere in the file included. Mutation: write the record after the
  target instead of before.
- **A pattern that does not occur exactly once is refused, before any file is written.** Test: a
  pattern with zero and with two occurrences; the target is unchanged and the exit code says nothing
  started. Mutation: accept the first occurrence.
- **A red baseline is refused.** Test: a command that fails over the unmutated tree; no mutation is
  applied. Mutation: skip the baseline.
- **A mutant that fails the `--build` probe is told apart from a caught one.** Test: a mutation that
  does not compile, with a probe; the verdict is "unmutatable here", not "caught". Mutation: read the
  probe's failure as a catch.
- **`restore` refuses a file changed since the mutation, and writes nothing.** Test: edit the file
  after the run applies the mutation and before the restore; the file keeps the edit, and the tool
  prints the reverse patch. Mutation: restore by writing the pre-image without the comparison.
- **The mutation is shown present before the command runs.** Test: the read-back line names the file
  and the first changed line. Mutation: print the line before writing the file.
- **`cargo x mutate` reaches the crate's `run` and `restore`.** Test: a test of the maintenance crate
  that runs the subcommand on a scratch project. Mutation: a subcommand that does nothing.
- **The setup snippet builds and runs the subcommand.** Test: the existing test that builds a project
  `klarch init --rust` wrote, extended to call `cargo x mutate` with no argument. Shown to fail: the
  snippet before the edit has no such subcommand.

## Audit subjects

- `path@elsewhere@~/dev/games/thaum/tools/mutate/`, whole: its src/lib.rs, src/main.rs, README.md, CLAUDE.md and
  tests/, and its design home, `path@elsewhere@~/dev/games/thaum/tools/mutate/docs/design.md`, whose heads, among them
  `report-not-gate`, `restore-by-comparison`, `record-survives-the-run` and
  `one-command-owns-the-interval`, are thaum's record: each is read and either carried into the new
  Component's design home with its argument, or left out with the reason.
- The thaum-specific parts, verified in thaum's tree when this document was written:
  `record_dir` returns `path@elsewhere@~/dev/games/thaum/tools/mutate/local/current/`; `project_root` finds the root by
  the checker's manifest name through the `knowledge_architect` crate; lib.rs ties the build to
  `THAUM_CHECKOUT`. The audit settles where the record lives in a target project and how the root is
  found, against `design@knowledge-architect@no-operation-loses-content`.
- `design@gates@gates-crate`, `rejected@gates@gates-command-of-the-checker`,
  `design@agent-skills@xtask-pins-checker`, `design@agent-skills@pinned-binary-runs-inside-the-project`.
- `design@knowledge-architect@publish-credential`, `design@knowledge-architect@version-lockstep`,
  `design@knowledge-architect@publish-after-merge`, `skill@klarch-release`, and the release
  workflow's package list.
- `design@knowledge-architect@no-component-directory-named-after-the-project`, for the directory and
  the package name.
- `design@agent-skills@rust-templates-one-home` and the snippets it names.
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`,
  `goal@xtask@repeated-tasks-are-automated`.

## Fails alone on

- A run that does not restore, or that loses content, observed by the kill-and-restore test.
- A refusal that does not refuse: a non-unique pattern, a red baseline or a changed file accepted.

## Premises that expire

- **The local development skill's manual worktree procedure stays** until slice 2 replaces it with
  `cargo x mutate`; the milestone's order is the guard.
- **The crate's first version is published by hand**, and every later one by the release workflow,
  per `design@knowledge-architect@publish-credential`; the release skill stops a release that would
  need the first, and the owner's word publishes it.

## Decided design

### The generic run and restore ship; sweep stays thaum's

Thread: `thread@development-and-components@mutation-command`.

**Shape.** One mutation the session writes, `--replace OLD --with NEW` with a uniqueness check or
`--patch`, applied to the live tree; a baseline run first, refused when red; an optional `--build`
probe; a read-back line; a byte-for-byte restore only when the file is the mutant the tool wrote;
a record on disk before the target is touched, so a killed run is recovered by `restore`. The
verdict is the exit code of the command after `--`, and the tool's own exit code is no verdict.

**Argument.** The value is a mutation the session writes itself, applied safely over uncommitted
work; cargo-mutants chooses its own (`argument@development-and-components@arg-21`,
`argument@development-and-components@arg-37`). It replaces about 40 lines of manual procedure in the
local development skill (`argument@development-and-components@arg-23`), which serves
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

**Nearest rival and the fact that defeated it.** The manual worktree procedure kept in the installed
text fails that goal (`argument@development-and-components@arg-7`). Shipping `sweep` too: it is tied
to thaum's invariant-check directory and its `InvariantViolation` pattern
(`argument@development-and-components@arg-39`).

### A separately published crate, a dependency of the maintenance crate

Thread: `thread@development-and-components@mutation-command-host`.

**Shape.** The package `knowledge-architect-mutate`, a Component of its own, `mutate`. A project's
maintenance crate depends on it and exposes `cargo x mutate`; the gates library does not depend on
it, and the checker's binary does not hold it.

**Argument.** The owner proposed a crate of its own, R6 (`argument@development-and-components@arg-71`).
The gates library runs a project's merge gates, and a mutation is no gate: through it, every gates
user would build the mutation crate (`argument@development-and-components@arg-77`). The maintenance
crate holds "the project's other repeated tasks", per `design@agent-skills@xtask-pins-checker`.

**Nearest rival and the fact that defeated it.** The checker's binary:
`rejected@gates@gates-command-of-the-checker`, "It would make the document checker run cargo, the
linter and the tests" (`argument@development-and-components@arg-57`). The gates library as host, the
agent's first default: the purpose of `design@gates@gates-crate`.

## Harvest

- **Heads:** in the new Component's design home, the shape above and each of thaum's heads the audit
  carries over; in `register@knowledge-architect@design`, the fourth published crate, and
  `design@knowledge-architect@publish-credential` rewritten in place.
- **Goals:** the new Component's, on the owner's ruling.
- **Rejected:** `sweep` shipped, and the gates library as host, each where
  `design@agent-skills@losing-alternatives-filter` admits it.
- **Tripwires:** none from the premortem: cause 8 is this slice's claim.
- **Plan document:** `planned@knowledge-architect@crates/mutate/` becomes a `path` reference in the
  commit that creates it, in every document of this milestone.
