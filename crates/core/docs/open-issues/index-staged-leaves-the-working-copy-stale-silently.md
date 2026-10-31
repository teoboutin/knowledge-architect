---
kind: todo
---
# `index --staged` gives no sign when the working copy of a file it staged now differs

## Summary

`cargo klarch index --staged` writes a generated file into git's index only, as designed, and
prints `<path>  staged`. When the working tree's copy then differs from the staged one, as after a
`git rm` of an entry with no plain `index`, the command does not say so, and `git status` shows the
file `MM`.

## Details

### What

`index_staged` in `path@core@src/cli/history.rs` prints only that the file was staged.
`design@core@index-staged-write` leaves the working-tree file as it is, and
`rejected@core@index-staged-writes-working-file` rejects writing it; neither weighs printing a line.

Instance: C1 of the retrospective of the session that built `klarch worklist`,
2026-10-25. After `git rm` of a spec, `index --staged` wrote the specs index into git's index; the
working copy still listed the deleted spec, and a plain `cargo klarch index` was needed.

### Why it matters

A session that commits, then edits the register again, works from a working copy that lists what
the commit deleted, and the next `check --fix` may refuse over the mixed state that
`design@core@fix-refusal-mixed-state` names, with no word of how it arose.

### What would close it

`index --staged` prints, for each file whose working copy differs from what it staged, one line
naming the file and the repair, `cargo klarch index`, with a test over a register whose entry was
removed with `git rm`.
