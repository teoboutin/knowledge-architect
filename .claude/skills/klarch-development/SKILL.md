---
name: klarch-development
description: MUST use, together with `skill@knowledge-architect-development`, which it adds to, before starting or iterating on any change to this repository's Rust source — the crates under crates/ and the maintenance tool under tools/. Covers what this repository adds to the installed procedure - the Component documents its grounding reads, the gate and the tests that must never run it, the branch placement of a change to the core, the build tie in a worktree, the cargo-configuration trap, and a shipped tool improved here. Not for the installed text under crates/agent-skills/content/, the CI workflow, or the agent configuration.
---

# Development in this repository

Scope: writing this repository's Rust source, the crates under `path@knowledge-architect@crates/`
and the maintenance tool under `path@knowledge-architect@tools/`. It adds to
`skill@knowledge-architect-development`, which holds the procedure: read that skill whole first.

**Not the installed text, the CI workflow, or the agent configuration.** The gate compiles and
tests Rust. It passes over a workflow file without seeing it, so a session that follows this skill
for one of those gets a green gate on an unverified change.

- The installed text, at its source under `path@agent-skills@content/`, follows
  `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", read whole.
- This repository's own agent configuration is `skill@knowledge-architect-agent-configuration`.
- A release is `skill@klarch-release`.

## Ground before editing `##ground-in-this-repository`

Beside what the installed skill's grounding reads, read the Component's own files: its
`CLAUDE.md`, for the invariants and traps that hold of the code as it stands; its issues and its
tripwires, for what is outstanding and what would reopen a decision. A behaviour that looks like a
new defect is checked with the two listings `instructions@verify-before-relying` adds.

## The gate here `##gate-in-this-repository`

**`cargo x gates` runs every gate in one command**, and the gate list's home is
`design@xtask@gates-list-primary-home`. How the gates are run, the checks owed before and after
each commit, and the repair of a failing one are `instructions@verify-mechanically` and
`primer@checker-model`, read whole.

**A change to the core under which a tree that passed before can fail**, such as a stricter check
of a commit's tree, a change of the manifest format or a change of what a generated file holds,
**is placed on the branch as `primer@branch-commits` says**, read whole.

**No test may run the gates over this repository**, since the test gate would run the suite that is
running it: `path@gates@CLAUDE.md` and `path@xtask@CLAUDE.md`.

## A mutation or a review in another checkout `##other-checkouts`

Every build here is tied to its checkout, per
`design@knowledge-architect@a-build-is-tied-to-its-checkout`, so a reviewer's worktree builds into a
target directory of its own, `export CARGO_TARGET_DIR="$PWD/target"` inside it: a build directory
shared with the main tree would rebuild at each switch.

**A mutation of the cargo configuration runs in a worktree outside the main tree**, such as one
under the session's scratch directory. Cargo merges the configuration of every parent directory
field by field, so in a worktree nested in the main tree the main tree's
`path@knowledge-architect@.cargo/config.toml` restores a field the mutation removed, and the mutant
survives a test that would catch it.

A mutation of `path@mutate@src/` with `cargo x mutate` rebuilds xtask at each run, since xtask links
the mutation library: `path@mutate@CLAUDE.md` says what that costs.

## A shipped tool found insufficient here `##shipped-tool-here`

This repository maintains the shipped tools, so it fixes one at its source rather than building a
tool of its own beside it, and a finding about one stays here, per
`design@knowledge-architect@workflow-findings-stay-here`.

## Comments in the checker's source `##checker-source-comments`

In the checker's own source, every string literal is data, bound or not, per
`design@core@checker-source-literals-are-data`, so a reference written in one is checked by nobody;
a comment is prose, and its reference is checked.
