---
kind: todo
---
# Tools of knowledge-architect run git past the wrapper, whose public interface lacks what they need

## Summary

`goal@git@a-pure-wrapper-over-git` is met while the git library's public interface lets every tool
of knowledge-architect run every git invocation it needs. Today the interface keeps private the
environment of an invocation, its standard input and its exit code as an answer, and ten files of
the other crates' code, their tests included, start git themselves.

## Details

### What

What the public interface lacks, in `path@git@src/git.rs`: `Invocation::env`, `Invocation::stdin`
and `Invocation::code` are `pub(crate)`, and so is `Invocation::paths`. A test that commits needs
an identity in the environment; a gate reads the exit code as its answer.

The code outside the crate that starts git itself, found with
`grep -rln 'Command::new("git")' crates tools --include=*.rs`, the git crate's own files left out:

- library code: `path@mutate@src/run.rs` (`git init` and `git apply` in a scratch directory) and
  `path@mutate@src/harness.rs` (`rev-parse --git-path`);
- tests: `path@mutate@tests/harness.rs`, `path@mutate@tests/modes.rs`,
  `path@check@tests/extension_api.rs`, `path@check@tests/init.rs`, `path@check@tests/binary.rs`, the
  unit test of `path@model@src/source/rs.rs`, `path@xtask@tests/gates_bin.rs` and
  `path@xtask@tests/mutate_bin.rs`.

The gates library's `rebased` gate runs `git merge-base --is-ancestor` as a gate's command line,
through its own spawn helper, in `path@gates@src/gates.rs`.

The goal asks that the library's interface hold each operation in git's terms that a tool composes
from several invocations, so these sites' operations are owed to the library, while a tool "can"
use it and need not move.

Two items of the library's interface may serve the checker rather than git: `component_dir` and
`CRATE_DIR` in `path@git@src/lib.rs` exist for the checker's reading of its own source's literals
as data and for its refusal of a build from another checkout; and `stage_generated` names its files
by the checker's word, though it stages any contents a caller gives. The goal's library knows
nothing of the workflow.

### Why it matters

A tool that cannot reach an invocation through the library keeps a wrapper of git of its own, the
outcome `goal@git@a-pure-wrapper-over-git` exists to spare, and the environment the library sets
for every invocation, per `design@git@repository-variables-dropped`, reaches none of it.

### What would close it

The public interface of the library carries what these sites need, each addition argued against
the design heads of `register@git@design`; each site either moves onto it or is left with the
reason it stays, in a comment at the site; and the two items above are renamed, moved out of
the library's interface, or kept with the argument that they fit the goal, as their design
decides: the owner ruled that an item shaped for its first caller, `component_dir` and `CRATE_DIR`
among them, knows nothing of a project's documents.
