# cargo-reader

The library that reads what the tools of knowledge-architect need of Cargo. What it is for is
`design@knowledge-architect@cargo-reader-crate`; what it does for a user is
`path@cargo-reader@README.md`; why it is shaped this way is `register@cargo-reader@design`.

**One module per format of Cargo's**: `path@cargo-reader@src/manifest.rs` reads a `Cargo.toml`'s
text, `path@cargo-reader@src/include.rs` matches a package's `include` and `exclude` patterns, a
gitignore-style grammar of their own, against a file of the package, and `path@cargo-reader@src/metadata.rs` runs
`cargo metadata` and reads its output. A new fact
the tools need is a field or a function in the module of its format; a new format is a module of
its own.

**`path@cargo-reader@docs/grounding/` holds the readings of Cargo that the code relies on**, each
read in the Cargo book or measured with the command it names.

**Every failure is an `Err` carrying the reason, never an empty answer**, per
`design@cargo-reader@failed-cargo-is-an-error`, and no default of Cargo's is applied to a field,
per `design@cargo-reader@a-manifest-is-read-as-written`.

**Every item public here is the library's interface**, per `design@knowledge-architect@api-facade`:
`path@cargo-reader@src/lib.rs` re-exports each public item of a module, and
`#![warn(unreachable_pub)]` makes any other a warning the clippy gate refuses. An item is made
public because a tool that calls this library needs it, never for a test of this crate, as
`check`'s facade is shaped, per `design@check@the-facade-publishes-what-a-consumer-needs`.
