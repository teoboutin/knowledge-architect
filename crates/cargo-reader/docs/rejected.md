# Rejected designs

The designs cargo-reader did not keep, and why.

### Cargo's defaults applied by the reader of a manifest `##cargo-defaults-applied`

`live`. An absent `version` read as `0.0.0`, and an absent `publish` read as published or not by
whether `version` is set, take the defaults of one Cargo: `version` became optional at Cargo 1.75,
so before it the same manifest was refused, and a reader applying the defaults of the Cargo it was
written for answers wrongly for a caller that runs another.

### An empty answer on a failure of Cargo or a read that fails `##an-empty-answer-on-failure`

`live`. A manifest that cannot be read taken as one with no package, and a failed `cargo metadata`
as a graph with no dependency, give a verdict on a project Cargo itself refuses, as if its tree
stated nothing.

### Every combination of fields Cargo refuses refused by the reader `##combinations-refused`

`live`. Which combinations Cargo refuses depends on the Cargo version: `publish = true` with no
`version` is refused because `version` became optional at Cargo 1.75, before which a manifest with
no `version` was refused whatever its `publish`, so the reader would answer for the Cargo it was
written for.

### A table holding `workspace = true` beside another key refused `##a-key-beside-workspace-refused`

`live`. Cargo 1.98 accepts `version = { workspace = true, other = 1 }` and inherits the workspace's
version, measured with `cargo metadata --format-version 1 --no-deps --offline`, so a reader that
refused it would declare no package for a manifest Cargo builds.

### The workspace's manifest found by the reader of a manifest `##workspace-manifest-found-by-the-reader`

`live`. Finding it repeats Cargo's lookup, by `package.workspace` or by a search of the parent
directories, which reads files the caller already holds, as the checker holds the tree it walks,
and makes a read of a manifest touch the file system.

### A `[project]` table read as a manifest with no package `##project-table-read-as-no-package`

`live`. Cargo 1.98 reports the package of a manifest whose `[project]` table names it, where the
manifest sets no `edition`, measured in `path@cargo-reader@docs/grounding/cargo-manifest.md`; a read
with no package there is an empty answer about a package Cargo reads, and misses a second checkout
of the tool holding such a manifest.

### A `[project]` table refused `##project-table-refused`

`live`. The same measurement: Cargo reports the package of such a manifest, so a refusal is an
error on a manifest Cargo reads, which no caller can repair in a project it does not own.
