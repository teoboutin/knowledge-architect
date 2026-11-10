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

`init` in `path@create@src/command.rs` refuses before any write when `--rust` meets a
`Cargo.toml` at PATH or in an ancestor of it, and when any file `--rust` writes already exists.
Adding the crate to an existing workspace means editing its `members` and its cargo
configuration, files the project owns, which no command edits today.

### Why it matters

A project adopting the workflow, new or existing, is the case `goal@knowledge-architect@adoption-is-easy`
names, and the gates `--rust` writes are a generic part that
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped` has shipped so that it serves each target project that carries out its activity. An existing project is the one the
owner gave as `init`'s example, per `design@create@init-keeps-existing-files`. The refusal is the
shape `design@create@rust-scaffold` decides.

### Trigger

A project asks for `--rust` over its existing Cargo workspace or package: that request is the work
of designing the edit of its `Cargo.toml` and of its cargo configuration.
