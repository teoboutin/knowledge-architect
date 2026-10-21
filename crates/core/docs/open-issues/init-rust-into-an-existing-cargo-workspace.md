---
kind: deferred
---
# `init --rust` cannot add the maintenance crate to an existing Cargo workspace or package

## Summary

`klarch init --rust` refuses a `Cargo.toml` at or above PATH, so a Rust project that already has
a workspace or a package writes the maintenance crate, its workspace member and its cargo aliases
by hand, from the setup skill's blocks.

## Details

### What

`init` in `path@core@src/cli/create.rs` refuses before any write when `--rust` meets a
`Cargo.toml` at PATH or in an ancestor of it, and when any file `--rust` writes already exists.
Adding the crate to an existing workspace means editing its `members` and its cargo
configuration, files the project owns, which no command edits today.

### Why it matters

A Rust project adopting the workflow is the case `goal@knowledge-architect@adoption-is-easy` and
`goal@knowledge-architect@setup-brings-quality-tools` name, and an existing project is the one the
owner gave as `init`'s example, per `design@core@init-keeps-existing-files`. The refusal is the
shape `design@core@rust-scaffold` decides.

### Trigger

A project asks for `--rust` over its existing Cargo workspace or package: that request is the work
of designing the edit of its `Cargo.toml` and of its cargo configuration.
