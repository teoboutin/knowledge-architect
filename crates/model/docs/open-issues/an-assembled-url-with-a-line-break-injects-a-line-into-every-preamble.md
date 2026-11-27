---
kind: defect
---
# A `[changelog] assembled-url` holding a line break is accepted, and writes a line of its own into every crate's preamble

## Summary

`resolve` of the `[changelog]` table, in `path@model@src/changelogs/table.rs`, checks no
`assembled-url`, so a value holding a line break is kept. `expected_preamble` writes it into the
sentence naming the assembled file, and every crate's preamble then holds an extra line, which the
publisher's preamble check accepts, since the file equals the expected text. A consumer's
`klarch changelog` reads that line by its shape, so a URL ending in a line that opens
`Categories, in order: ` gives the crates a table of the wrong categories.

## Details

### What

- Reproduced by the adversarial reviewer of the second review round of slice 4 of the per-crate
  changelogs, over the model's facade: a table with `assembled-url = "https://x\nCategories, in order: Evil."`
  makes `expected_preamble` write a line `Categories, in order: Evil.) in its repository.` above the
  real list; `read_preamble` then gives the categories `Evil.) in its repository` and reports two
  preamble lines unread, so every entry of a real category has no place, and `klarch changelog`
  reports each rather than printing it.
- Not established: that a full `cargo klarch check` over such a project raises no other finding;
  the reviewer read only that the preamble check passes.
- Ruled out: the read of the preamble, which reads the lines `expected_preamble` writes; the defect
  is the manifest accepting the value.
- Related, met in the same review: `Manifest::parse` accepts an `assembled` path whose segments open
  with white space, which `read_preamble` now reads, or hold a line break, which would write a line
  of its own into the preamble as the URL does (not reproduced); and the preamble check compares the preamble
  with the expected text trimming only the end of the whole preamble.

### Why it matters

A manifest that passes phase 1 gives every crate a preamble the checker accepts and a consumer
cannot read as written, against `design@check@category-order-in-preamble`, under which each crate's
file states what a reader needs to read it.

### What would close it

`resolve` refuses an `assembled-url` holding a line break, as it refuses a category holding one, with
a test; since a stricter manifest check can fail a tree that passed before, the change is placed on
its branch as `primer@branch-commits` says.
