---
kind: question
---
# Does the CI workflow that the setup skill shows run on three systems, and with `--ci`?

## Summary

The CI workflow that the setup skill shows a project, `path@agent-skills@snippets/ci.yml`, runs the
gates on Linux alone and without `--ci`. This repository's own runs them on Linux, Windows and macOS,
behind one job, with `--ci`. Which of the two a project is shown is not decided.

## Details

### What

Read in `path@agent-skills@snippets/ci.yml`, which `skill@knowledge-architect-setup@rust-project` inlines:

- its one job, `check`, runs on `ubuntu-latest`;
- its gates step is `cargo --locked x gates --locked --fail-fast --require-rebased --full`, with no
  `--ci`, and `git log -G'--ci'` over it and the skill's source returns no commit, so no commit argued
  for or against it there.

The question has two parts. Whether a project is shown three systems is a choice of what a project is
offered, and the workflow stays as it is for now. Its costs either way: a project that takes three
systems runs three jobs per push, needs its tests to pass on each, and takes the gathering job, or
changes the contexts its ruleset requires; a project that takes one system and delivers on another meets
there what its gates would have reported. `--ci` is argued by `design@gates@ci-flag-for-ci-only-requirements`
for every project whose tests read the variable, and it costs a project whose tests read none nothing:
the library sets one variable.

### Why it matters

The workflow is the text a project copies, so it decides what a project's CI judges. Without `--ci`, a
test that skips where a tool is missing skips in the project's CI too, against
`design@gates@ci-flag-for-ci-only-requirements`, under which CI never passes with the check unrun. On
Linux alone, a project that delivers on Windows or macOS is told by no gate that it fails there, though
`design@knowledge-architect@platforms-served` has the gates run on each system.

### What would close it

The owner's choice on the three systems, then the snippet, the paragraph of the setup skill below it
and the test of `path@testing@tests/scaffold.rs` that parses its gates command, edited as
`path@agent-skills@CLAUDE.md` says: with three systems, the matrix and the gathering job, after
`design@knowledge-architect@one-required-context`; without, the reason kept beside the skill's block, in
a `%%` comment. `--ci` is added with either, or the reason it is not is kept there.
