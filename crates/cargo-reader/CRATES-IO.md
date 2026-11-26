# knowledge-architect-cargo-reader

Reads what the tools of knowledge-architect need of Cargo: the text of a `Cargo.toml`, and the
output of `cargo metadata`.

The checker depends on it; a project names the package `knowledge-architect` alone. A Rust program
can depend on it directly, with `cargo add knowledge-architect-cargo-reader`.

## Documentation

- The library API: [docs.rs](https://docs.rs/knowledge-architect-cargo-reader).
- The [repository README of this crate](https://github.com/teoboutin/knowledge-architect/blob/main/crates/cargo-reader/README.md).
- The project: [the repository](https://github.com/teoboutin/knowledge-architect).
