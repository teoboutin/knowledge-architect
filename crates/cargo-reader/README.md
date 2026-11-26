# knowledge-architect-cargo-reader

A library that reads what the tools of knowledge-architect need of Cargo: the checker of a
project's documents reads a crate's package name through it today. A project that uses
knowledge-architect names the package `knowledge-architect` alone, which depends on this one, and
never names this crate. A Rust program can depend on it directly; its interface is at
<https://docs.rs/knowledge-architect-cargo-reader>.

## What it reads

- **A `Cargo.toml`'s text**: the package name, and its `version`, `publish`, `repository` and
  `include`, each as the manifest writes it, with no default of Cargo's applied, from its
  `[package]`, or from its `[project]` where it holds no `[package]`; a field set to a table whose
  `workspace` is `true` takes the value of the workspace's `[workspace.package]` when the caller
  hands that table in.
- **The output of `cargo metadata --format-version 1`**, run in a directory with the flags the
  caller passes: the resolved dependency graph, and each of its packages' name, version, manifest
  path and `repository`.

## What it needs and guarantees

- **Reading a manifest touches no file and runs nothing.** The caller reads the text, and finds the
  workspace's manifest a package inherits from.
- **Running `cargo metadata` needs Cargo**: the one the variable `CARGO` names, which Cargo sets for
  the programs it runs, or else `cargo` on the path. The run inherits the caller's environment, and
  writes `Cargo.lock` or reaches the network as Cargo does under the flags passed.
- **A failure is an error carrying its reason, never an empty answer**: a text that is no TOML or
  holds a field of a type Cargo refuses, Cargo's own failure with its standard error, an output of
  another shape or another format version.
- **A key the reader does not read is ignored**, so a manifest Cargo accepts is not refused for it.

Why it is shaped this way is `register@cargo-reader@design`.
