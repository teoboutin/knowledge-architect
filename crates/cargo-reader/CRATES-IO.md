# knowledge-architect-cargo-reader

Reads what the tools of knowledge-architect need of Cargo. Through it, the checker of a project's
documents reads the facts of a `Cargo.toml`, the package name, its version with the inheritance from
the workspace, `publish`, `repository` and `include`, and the resolved graph that `cargo metadata`
reports, each package with its manifest path and its repository. A failure, of Cargo or of a text
of a shape Cargo refuses, is an error carrying the reason, never an empty answer.

The checker depends on it; a project names the package `knowledge-architect` alone.

## Documentation

- The library API: [docs.rs](https://docs.rs/knowledge-architect-cargo-reader).
- The [repository README of this crate](https://github.com/teoboutin/knowledge-architect/blob/main/crates/cargo-reader/README.md).
- The project: [the repository](https://github.com/teoboutin/knowledge-architect).
