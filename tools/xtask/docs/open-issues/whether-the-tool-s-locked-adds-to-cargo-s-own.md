---
kind: question
---
# Whether the tool's `--locked` adds anything beside cargo's own, which CI also passes

## Summary

CI passes cargo's `--locked` ahead of the alias `x`, and the tool's `--locked` after it. No head says what the
second catches that the first does not.

## Details

### What

`design@xtask@ci-passes-locked-ahead-of-the-alias` is titled on both, and argues the first alone: the rival it
defeats is the tool's flag alone. The rival it leaves, cargo's flag alone, has a fact on its side: a lock that has
drifted stops cargo before the tool starts, as `cargo --locked x --help` exits 101 over a lock with a block
deleted. The tool's flag would then catch only a lock that changes while the gates run, and
`design@xtask@ci-differs-by-flags`, which names it, gives "lock drift must fail" for both.

### Why it matters

The command of CI holds a flag that a reader cannot explain, and a design that is stated without defeating its
nearest rival. It strains `design@xtask@ci-passes-locked-ahead-of-the-alias`.

### What would close it

A measurement of what each flag catches alone, then the argument for the tool's flag in a head, or the flag dropped
from CI's command.
