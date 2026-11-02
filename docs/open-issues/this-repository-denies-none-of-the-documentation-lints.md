---
kind: todo
---
# This repository's published crates warn on none of the documentation lints the setup asks for

## Summary

The setup skill asks a crate a target project publishes to warn on `missing_docs`,
`clippy::missing_errors_doc` and `clippy::missing_panics_doc`, per
`design@agent-skills@api-doc-completeness`. This repository publishes four crates and declares none
of the three, so it does not follow an ask it ships.

## Details

### What

Measured on the owner's workstation, at slice 2 of the milestone that shipped the development
skill, with `RUSTFLAGS="-W missing_docs" cargo clippy -p <crate> --lib -- -W
clippy::missing_errors_doc -W clippy::missing_panics_doc`, counting distinct warnings: the core 70,
the gates library 2, the mutation library 12, agent-skills 0. Re-take it with that command per
published crate.

### Why it matters

A target project that does not yet follow an ask brings itself into conformance over time, per
`goal@knowledge-architect@serves-one-kind-of-project`; meanwhile this repository's public items can
lack the documentation a consumer of its libraries needs, and the code-claims axis reads documents
that may not exist.

### What would close it

Each published crate's `Cargo.toml` declares the three lints, every warning is answered by the
documentation it asks for, and the clippy gate passes.
