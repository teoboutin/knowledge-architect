---
kind: design
---
# The plans layout is read in three units, which no test compares

## Summary

Which directory of the plans directory is a spec, a group of specs, a live slice or a harvested
slice is read in three units: `crate::plan` of `model`, the layout check of
`path@model@src/tree.rs`, and `plan_home` in `path@check@src/check/registers.rs`. A change to the
layout must be made in all three, and where they disagree phases 2, 3 and 4 read different plan
documents.

## Details

### What

- `crate::plan` reads the layout for the entity table and the anchors (`spec_dirs_all`, `slices`,
  `headed`).
- The layout check of phase 2 in `path@model@src/tree.rs` reads it again (`specs_layout`,
  `plan_directory`, and a `headed` of its own).
- `plan_home` in `path@check@src/check/registers.rs`, of phase 4, reads it a third time, with its own
  `headed`, and writes the names README.md, harvested and search as literals. Its specs/ arm also
  repeats, almost word for word, the findings on declared groups that `directory_contents` gives.
- No test compares the three readings over one tree. Found by the code-shape reviewer at the review
  before the merge of slice 1 of `milestone@plans@split-guidelines-and-plan-directories`; its findings
  3 and 4.

### Why it matters

A second level of groups, or another subdirectory of a milestone, is three edits; one missed is a
plan document that one phase reads and another does not, which the phase-gated report of
`design@check@phases-gate-the-report` would show as a finding of the wrong phase, or as none.

### What would close it

`plan_home` reads the plan documents through a reader `model` exports, and one function judges
declared groups for both homes; or a test that reads one tree through the three units and compares
what each finds. The first makes the reader public API of `model`, with a changelog entry.
