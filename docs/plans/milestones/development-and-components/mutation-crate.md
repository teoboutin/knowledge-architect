# Slice 1: the mutation crate, a published crate run as `cargo x mutate`

## Builds

1. **The Component `mutate`**, directory `planned@knowledge-architect@crates/mutate/`, package
   `knowledge-architect-mutate`, a workspace member and the fourth published crate, created with
   `cargo klarch new-component` and carrying the documents every Component carries. Its goals are
   drafted under `skill@knowledge-architect-goal-setting` and ruled by the owner at this slice.
2. **Its code**, taken from the generic `run` and `restore` of thaum's mutation tool: the library
   `path@elsewhere@~/dev/games/thaum/tools/mutate/src/lib.rs` and, from
   `path@elsewhere@~/dev/games/thaum/tools/mutate/src/main.rs`, the functions `run_hand_mutation`,
   `run_command` (which thaum's `sweep` shares), `unwind`, `restore_record` and `apply_to_scratch`,
   and the types `Form`, `RunArgs` and `RestoreArgs`, with their tests, and the behaviour its README
   states under "The hand mutation". Without `sweep`, and with the thaum-specific parts the audit
   lists replaced: the record directory and the lookup of the project root. The crate carries this
   repository's build tie, `KNOWLEDGE_ARCHITECT_CHECKOUT`, in place of thaum's `THAUM_CHECKOUT`, per
   `design@knowledge-architect@a-build-is-tied-to-its-checkout`, as every crate here does. The crate
   holds a library, which a project's maintenance crate calls; every Rust target project has one,
   per #maintenance-crate-ask. The thaum-specific parts are replaced as the design audit settled,
   under "Decided design": the caller passes the project's root, and the record lives in the git
   directory. The crate depends on neither the checker nor the gates library.
3. **`cargo x mutate`** in this repository's maintenance crate, `path@knowledge-architect@tools/xtask/`,
   depending on the mutation crate directly, not through the gates library, per
   #mutation-command-host.
4. **The setup skill's maintenance-crate snippet** gains the dependency and the subcommand, in
   `path@agent-skills@snippets/xtask-cargo.toml` and `path@agent-skills@snippets/xtask-main.rs`, each
   tested as `design@agent-skills@rust-templates-one-home` says; `cargo klarch install-agent-skills`
   run.
5. **`skill@klarch-release`** lists the fourth crate in its package lists, version rules and publish
   order, and states that the crate's first version is published by hand with an API token, per
   `design@knowledge-architect@publish-credential`, which the slice rewrites. **Publishing that first
   version is no part of this slice's landing**: it cannot be undone, so it happens at the release,
   after the merge and on the owner's word, per `design@knowledge-architect@publish-after-merge`.
6. **The CHANGELOG.md entries** in `Next release`, and `cargo x changelog`, which writes the copy the
   new crate ships too, per `design@knowledge-architect@the-changelog-ships-in-every-crate`.
7. **Every other site a fourth published crate reaches:**
   - `path@knowledge-architect@.github/workflows/release.yml`: the comment at its head, which names
     the three crates, and the crate lists of its steps "the tag is the version of every published
     crate" and "publish", whose order is the publish order;
   - `path@xtask@src/changelog.rs`, test `every_published_crate_carries_the_root_changelog`, which
     asserts three copies; the new crate's Cargo.toml lists `/CHANGELOG.md` in `include`, per
     `design@knowledge-architect@package-include-whitelist`;
   - `skip-files` in `path@knowledge-architect@knowledge-architect.toml`, which lists each crate's
     copy of CHANGELOG.md;
   - the Component table of the root `path@knowledge-architect@CLAUDE.md` and the crate table of the
     root `path@knowledge-architect@README.md`;
   - `design@knowledge-architect@versioning-policy`, which says a release "publishes all three
     crates", and `design@knowledge-architect@publish-credential`, which says "Each of the three
     crates", both rewritten at the harvest;
   - `path@xtask@tests/scaffold.rs`: the `[patch.crates-io]` that its test
     `a_project_written_by_init_rust_builds_and_checks` writes gains the mutation crate, since the
     test builds offline; the comment above that test states a threshold of 60 seconds on the
     owner's workstation, a premise whose guard is that test;
   - `path@xtask@examples/setup_snippet.rs`, which compiles the snippet's main;
   - the new crate's `CRATES-IO.md`, named by `readme` in its Cargo.toml, per
     `design@knowledge-architect@crates-io-page-file`, and its copies of `LICENSE-MIT` and
     `LICENSE-APACHE`, in its `include`, per `design@knowledge-architect@package-include-whitelist`;
   - the surface list of `design@knowledge-architect@changelog-entries` and of the head of
     CHANGELOG.md, which gains `mutate`, the new crate's library, as `gates` is the gates library's;
     the head is rewritten at the harvest, with
     `design@knowledge-architect@the-changelog-ships-in-every-crate`, whose body counts "three
     `skip-files` rows";
   - the publish order of the release workflow and of `skill@klarch-release`: the mutation crate
     depends on no crate of this repository, so it is published after the gates library and before
     the core, per `design@knowledge-architect@partial-publish-rerun`;
   - `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`, whose list of the
     commands under `path@xtask@src/` gains `mutate`.

8. **The maintenance crate as an ask of every Rust target project**, per #maintenance-crate-ask:
   the setup skill, `skill@knowledge-architect-setup`, states it among the asks, gives the
   maintenance crate as the one shape for a Rust project, and no longer offers a Rust project the
   local install that leaves it with no maintenance crate and no `cargo x gates`, there and in its
   pin-move steps, which name "the version of the local install"; a project installed that way
   brings itself into conformance, as the target-project goal says. A Migration entry in
   `Next release` tells a consumer what to add. The heads
   `design@agent-skills@pinned-binary-runs-inside-the-project`, which admits that local install on
   the owner's ruling, and `design@agent-skills@xtask-pins-checker`, which speaks of "a Rust project
   with a maintenance crate", are rewritten, and `design@gates@gates-crate`, which calls the
   maintenance binary "conventionally a crate named xtask", is reread, under `skill@knowledge-architect-decision-recording`, and
   every text `cargo klarch show` lists for them is reread. The setup skill's case of a project with
   no Rust is outside this ask. `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`
   gains the ask among those it lists.

The local development skill's worktree procedure is replaced by the command at slice 2, not here.

## Claims

Each claim's test is a test of the mutation crate, shown to fail by a recorded mutation of the code
it guards. The slice's first claims are shown with the manual procedure of
`skill@klarch-development@discriminating-tests`, lines 79 to 120 of its installed copy, the worktree
procedure; the claims written after the command builds are shown with `cargo x mutate run`, as its
first use.

- **A run killed mid-mutation is restored by `restore`, and no content is lost**, per
  `design@knowledge-architect@no-operation-loses-content` and premortem cause 8. Test: start a run
  whose command blocks, kill it, run `restore`, and compare the file byte for byte with its
  pre-image, uncommitted edits elsewhere in the file included. Mutation: write the record after the
  target instead of before.
- **A pattern that does not occur exactly once is refused, and leaves the target untouched.** Test:
  a pattern with zero and with two occurrences; the target is unchanged, the record directory the run
  claimed is removed, and the exit code is 2. Mutation: accept the first occurrence.
- **A red baseline is refused.** The baseline runs by default, and `--no-baseline` skips it. Test: a
  command that fails over the unmutated tree; no mutation is applied. Mutation: skip the baseline.
- **A mutant that fails the `--build` probe is told apart from a caught one.** Test: a mutation that
  does not compile, with a probe; the verdict is "unmutatable here", thaum's verdict when the
  `--build` probe fails, which says the mutant does not build and a different mutation is owed, not
  "caught". Mutation: read the
  probe's failure as a catch.
- **`restore` refuses a file changed since the mutation, and writes nothing.** Test: edit the file
  after the run applies the mutation and before the restore; the file keeps the edit, and the tool
  prints the `git apply -R` commands of the record's patch, where the record holds one. Mutation: restore by writing the pre-image without the comparison.
- **The mutation is shown present before the command runs.** Test: the read-back line names the file
  and the first changed line. Mutation: print the line before writing the file.
- **`cargo x mutate` reaches the crate's `run` and `restore`.** Test: a test of the maintenance crate
  that runs the subcommand on a scratch project. Mutation: a subcommand that does nothing.
- **The setup snippet builds and runs the subcommand.** Test: the existing test
  `a_project_written_by_init_rust_builds_and_checks` in `path@xtask@tests/scaffold.rs`, which builds
  a project `klarch init --rust` wrote, extended to call `cargo x mutate` with no argument, which
  prints the help and exits 2, as thaum's tool does. Shown to fail: the snippet before the edit has no
  such subcommand.
- **Every outcome maps to one exit code**, by the table under "Decided design". Test: a test per
  row, among the claims above. Shown to fail: a run that never started exiting 1, as a recorded
  mutation.

## Audit subjects

- `design@knowledge-architect@a-build-is-tied-to-its-checkout`, for the crate's build tie.
- `design@core@exit-code-ladder` and `issue@gates@a-run-that-could-not-start-exits-one`, for the
  table of exit codes.
- thaum's `rejected` entries of the tool, in
  `path@elsewhere@~/dev/games/thaum/tools/mutate/docs/rejected.md`: `forward-apply-check-proof`,
  `worktree-holds-the-mutation` and `cargo-mutants-instrument`; its tripwire guarding
  `restore-by-comparison`, in `path@elsewhere@~/dev/games/thaum/tools/mutate/docs/tripwires.md`; its
  open issue `path@elsewhere@~/dev/games/thaum/tools/mutate/docs/open-issues/hard-kill-orphans-the-command.md`,
  which the kill-and-restore claim exercises; its head `discard-is-the-callers-word`; and thaum's
  root heads `root-by-the-checkers-manifest-name` and the one whose slug is that of
  `design@knowledge-architect@a-build-is-tied-to-its-checkout`, in
  `path@elsewhere@~/dev/games/thaum/docs/design.md`.
- The liveness check of a recorded run, which reads `/proc` and holds on Linux only, against
  `goal@agent-skills@installed-text-works-in-any-environment`.
- `project_root` of the gates library, in `path@gates@src/process.rs`, as the lookup of the project
  root.
- The print rule of the maintenance crate, `say` and `complain`, in `path@xtask@CLAUDE.md`, for
  `cargo x mutate`.
- `design@gates@gates-scrub-rustc-bootstrap`, for the baseline and the probe the tool spawns.
- What becomes of thaum's records of the tool once the tool moves here: decided at this slice's
  harvest; thaum keeps its own copy until the crate ships, per the sentence "Until a part taken in
  ships, the project's own copy of it stays" of
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.
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
- `issue@core@init-rust-into-an-existing-cargo-workspace`, for #maintenance-crate-ask: a Rust
  project with an existing workspace or package that adds the maintenance crate the ask requires
  meets the refusal of `klarch init --rust` that the issue records.
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
- **Until the first version is published, the setup snippet pins no version absent from
  crates.io.** The snippet's dependency lands in this slice: it pins the checker's own version, as
  its two other dependencies do, so no consumer meets it before the release that publishes all four
  crates at that version, and in the tree the `[patch.crates-io]` of the scaffold test covers it.
  The test `a_project_written_by_init_rust_builds_and_checks` is the guard, and the release skill
  stops the release until the first version is published by hand.

## Decided design

### The generic run and restore ship; sweep stays thaum's

Thread: `thread@development-and-components@mutation-command`.

**Shape.** One mutation the session writes, `--replace OLD --with NEW` with a uniqueness check or
`--patch`, applied to the live tree; a baseline run first, refused when red; an optional `--build`
probe; a read-back line; a byte-for-byte restore only when the file is the mutant the tool wrote;
a record on disk before the target is touched, so a killed run is recovered by `restore`. The
verdict is the exit code of the command after `--`, and the tool's own exit code is no verdict.
The baseline runs by default, and `--no-baseline` skips it. On a changed file, `restore` writes
nothing and prints the `git apply -R` commands of the record's patch, where the record holds one. A
refused pattern leaves the target untouched: the record directory the run claimed is removed.

The tool's exit code is a total function of the run's outcome, the three outcomes of thaum's README:

| outcome | exit code |
| --- | --- |
| the run completed, whatever the verdict | 0 |
| the tool acted, and something is owed on the tree: no verdict reached, the target left mutated, or a record still standing | 1 |
| the run never started: a bad argument, no project found, or a precondition that refused before anything was written | 2 |

The audit read the table against `design@core@exit-code-ladder`, which binds the core's commands
alone: the table keeps the ladder's line between 2 and the rest, for the ladder's reason, a caller
told "never ran" from "ran", and gives 1 its own meaning, something owed on the tree, since a
verdict is no exit code here. A refused pattern claims the record directory and removes it, which
touches no file of the working tree, since the record lives in the git directory. The maintenance
crate's `mutate` arm exits 2 when no project lies above the working directory.
`issue@gates@a-run-that-could-not-start-exits-one` is about the gates and stays open.

**Settled at the design audit**, each the one answer the decisions it names leave:

- **The root is the caller's.** `run` takes the project's root from the maintenance crate, which
  finds it with the gates library's `project_root` and the checker's manifest name, as its gates arm
  does. The mutation crate then links neither the checker nor the gates library, per
  #mutation-command-host, and `design@core@installed-binary-version-check` asks no version check of
  it.
- **The record lives in the git directory**, at the path `git rev-parse --git-path
  knowledge-architect-mutate` names. A target project is a git repository, per
  `goal@knowledge-architect@serves-one-kind-of-project`. There the record survives a `cargo clean`
  and a `git clean`, as `design@knowledge-architect@no-operation-loses-content` requires of the only
  copy of a pre-image; no project writes an ignore rule for it; the checker's walk, which lists
  untracked files that no ignore rule covers, per `design@core@git-supplies-the-walk`, never reads
  it; and each worktree gets its own. thaum's `path@elsewhere@~/dev/games/thaum/tools/mutate/local/` would need each project to pick
  and ignore a directory.
- **A run's liveness is read portably**: `/proc` on Linux, `kill -0` on another Unix; where neither
  answers, the owner reads as gone, which permits the restore. The goal the spec named,
  `goal@agent-skills@installed-text-works-in-any-environment`, ranges over the harness's
  configurations, not over operating systems, so it does not decide this.
- **The caller's command runs in the caller's environment**: `RUSTC_BOOTSTRAP` is not scrubbed.
  `design@gates@gates-scrub-rustc-bootstrap` guards a verdict of the gates; here the verdict compares
  the baseline and the mutant under one environment, so a variable both inherit changes neither
  side of the comparison.
- **The messages name `cargo x mutate`**, the invocation #maintenance-crate-ask gives every Rust
  target project.

**Argument.** The value is a mutation the session writes itself, applied safely over uncommitted
work; cargo-mutants chooses its own (`argument@development-and-components@arg-21`,
`argument@development-and-components@arg-37`). It replaces about 40 lines of manual procedure in the
local development skill, lines 79 to 120 of the installed copy of `skill@klarch-development`, the
worktree procedure (`argument@development-and-components@arg-23`), which serves
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
  `design@knowledge-architect@publish-credential`, `design@knowledge-architect@versioning-policy`,
  `design@knowledge-architect@the-changelog-ships-in-every-crate` and
  `design@knowledge-architect@changelog-entries` rewritten in place where they count or list the
  published crates, each stating the rule that admits a crate rather than a count, per
  `design@agent-skills@title-states-the-rule`.
- **thaum's records of the tool:** what becomes of them is decided here, as "Audit subjects" says.
- **Goals:** the new Component's, on the owner's ruling.
- **The ask:** `design@agent-skills@pinned-binary-runs-inside-the-project` and
  `design@agent-skills@xtask-pins-checker` rewritten for #maintenance-crate-ask, the owner's ask
  attributed to the owner.
- **Rejected:** `sweep` shipped, and the gates library as host, each where
  `design@agent-skills@losing-alternatives-filter` admits it.
- **Tripwires:** none from the premortem: cause 8 is this slice's claim.
- **Plan document:** `planned@knowledge-architect@crates/mutate/` becomes a `path` reference in the
  commit that creates it, in every document of this milestone.
