# knowledge-architect-mutate

The hand mutation: apply one mutation you write, run a command against it, report whether the
command noticed, and put the file back, over a working tree that may hold uncommitted work. It is
the check that a test discriminates, made one command.

A project calls it from its own maintenance binary, a crate named xtask run through
the cargo alias `cargo x`, as `cargo x mutate`.

## Documentation

- The library API: [docs.rs](https://docs.rs/knowledge-architect-mutate).
- The [repository README of this crate](https://github.com/teoboutin/knowledge-architect/blob/main/crates/mutate/README.md).
- The project: [the repository](https://github.com/teoboutin/knowledge-architect).
