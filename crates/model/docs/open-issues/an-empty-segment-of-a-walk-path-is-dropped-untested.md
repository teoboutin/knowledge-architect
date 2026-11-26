---
kind: todo
---
# The drop of an empty segment of a `[walk]` path is tested by nothing, and its head names only the `.` drop

## Summary

`normalise_one` in `path@model@src/manifest.rs` drops a `.` segment and an empty segment of a
declared `[walk]` path, since `Path::components` yields neither. `design@model@git-supplies-the-walk`
states the `.` drop alone, and no test of manifest.rs passes a path holding `//`.

## Details

### What

- `normalise_one` builds the path again from `path.components()`, keeping `Normal` segments,
  skipping `CurDir`, and refusing `ParentDir`, `RootDir` and `Prefix`. `Path::components` yields no
  component for an empty segment, so `a//b` becomes `a/b` with no complaint.
- `design@model@git-supplies-the-walk` says a `.` segment is dropped and says nothing of an empty
  one. `design@model@citation-free-glob-spelling`, which contrasts a glob's refusal with a `[walk]`
  path's drop, states both drops and names `Path::components` as the source of the second.
- A search for a string literal holding `//` in manifest.rs, `grep -n '"[^"]*//[^"]*"'
  crates/model/src/manifest.rs`, finds none, so no unit test there exercises the empty-segment drop.
  The testing crate was not searched (`not established` that no test elsewhere covers it).

Met by the review before the merge of slice 1 of `milestone@plans@per-crate-changelogs`, whose
repairs wrote the contrast into the glob-spelling head.

### Why it matters

A change to `normalise_one` that refused, or kept, an empty segment would pass every test, and
`design@model@git-supplies-the-walk` would not tell its reader the drop exists. The glob-spelling
head argues against that drop as its rival, so it rests on behaviour nothing pins.

### What would close it

A unit test of `normalise_one` with a path holding `//`, shown to discriminate by a recorded
mutation, and one clause in `design@model@git-supplies-the-walk` naming the drop of an empty
segment beside the drop of a `.` segment.
