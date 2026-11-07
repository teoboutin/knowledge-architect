---
kind: todo
---
# No gate builds the crates' documentation, so a broken link in it reaches no run

## Summary

`cargo x gates` runs `fmt`, `check`, `commits`, `clippy` and `test`, and none of them builds the documentation with `cargo doc`. A broken intra-doc link in a crate's documentation, the text docs.rs renders, then passes every gate.

## Details

### What

A review of the branch that created `testing` found `path@check@src/lib.rs` linking a module the branch had removed: `cargo doc -p knowledge-architect-check --no-deps` printed `warning: unresolved link to \`testing\``, and every gate passed. The link is repaired. `cargo doc -p knowledge-architect-check -p knowledge-architect --no-deps` printed no warning after the repair, measured on this machine.

### Why it matters

The crate documentation is the home of a library's API description, per the root `CLAUDE.md`, and `goal@knowledge-architect@adoption-is-easy` counts on it. A link that names nothing there is found by a reader, not by a gate.

### What would close it

A gate that runs `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS="-D warnings"`, or the `test` gate's equivalent, in `path@knowledge-architect@tools/xtask/`, and a mutation that breaks one link and fails it.
