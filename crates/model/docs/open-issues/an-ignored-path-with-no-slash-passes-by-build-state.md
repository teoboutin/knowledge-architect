---
kind: defect
---
# An ignored directory cited with no trailing slash gets a verdict that depends on what is on disk

## Summary

A span naming an ignored directory with no slash is reported missing on a fresh clone and passes on a built tree, which `design@model@ignored-targets-are-not-asserted` says cannot happen.

## Details

### What

`.gitignore` holds `build/`, and a README holds a `path` span at its anchor naming `build`, with no slash. With no `build` directory, `check` reports that it does not exist, and exits 1. After `mkdir build; echo x > build/o`, it prints `PASSED: no findings`.

`ignore_query` gives git the spelling with no slash, and git stats the disk to learn whether a rule for a directory applies. Reproduced on the main branch before the crate split.

### Why it matters

It stands against `design@model@ignored-targets-are-not-asserted`, whose argument is that the verdict is the same on a fresh clone and on a built tree, and `goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

The same verdict in both trees, with a test that runs the check over each, or the head narrowed to what git can answer, under the design skill.
