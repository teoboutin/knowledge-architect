---
kind: question
---
# Does the `"self"` pin hold where a project commits a copy of the checker's crate and builds from it

## Summary

`checker-version = "self"` is accepted when the check crate's directory lies in the project and the project's git tracks its
`Cargo.toml`. A project that commits a vendored copy and builds from it meets that test, and then runs with no comparison of
versions.

## Details

### What

Reproduced by an adversarial reviewer, with `cargo package` of the crates unpacked in a scratch repository and built under
`[patch.crates-io]`, manifest `checker-version = "self"`, and `knowledge-architect-check-0.9.0/Cargo.toml`: ignored or untracked,
exit 2, `"self" is only valid where the checker is built from this tree`; only the other crates' `Cargo.toml` tracked, exit 2;
`git add`ed, the command runs. `cargo vendor` itself was not run.

Not established: whether any project keeps such a copy. `is_its_own_build` of `path@check@src/commands/mod.rs` is what decides.

### Why it matters

It matches the words of `design@check@checked-sentinel-values`, whose tracking test tells a checkout from an untracked registry
and says nothing of a vendored copy, and it strains that head's words that no value of the key switches a real project's pin
off. `rejected@check@sentinels-accepted-everywhere` is the design this edges back toward.

### What would close it

An answer, in `design@check@checked-sentinel-values`: a tracked copy counts as a tree the checker is built from, or it does not,
with a test that holds the answer.
