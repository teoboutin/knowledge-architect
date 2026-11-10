---
kind: todo
---
# A mutation spanning several files cannot be applied in one run

## Summary

A wrong implementation can span several files, such as a trait's declaration and its implementation
in two modules. The crate applies a mutation to the one file `--file` names, and refuses a `--patch`
that names any other file, per `design@mutate@one-file-per-run`, which states that limit as the
current state of the tool and not as its scope.

## Details

### What

`run` takes one `--file`; its record holds one pre-image, one mutant and one patch; `restore`
compares and writes back one file. A wrong implementation across two files can be checked today
only by mutating one of them, which may not build, or not at all.

### Why it matters

The owner chose not to limit the crate to one file, at slice 1 of the milestone that shipped it.
Until this is built, a claim whose wrong implementation spans
files has no mutation check by the tool, and a session falls back to a hand procedure, which
`design@mutate@one-command-owns-the-interval` argues against.

### What would close it

A run that takes several files, or a `--patch` naming several, records the pre-image and the
mutant of each before touching any, restores each by comparison, and is tested by a mutation of
several files applied and restored; `design@mutate@one-file-per-run` is then rewritten.
