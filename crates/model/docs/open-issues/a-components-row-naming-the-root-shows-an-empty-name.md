---
kind: defect
---
# The refusal of a `[project] components` row naming the project's root shows the anchor's name as an empty span, not the row as written

## Summary

A `[project] components` row of `.` or `""` reduces to an empty path, whose basename is the empty
name, and `resolve_anchors` in `path@model@src/manifest.rs` refuses that name. Its repair is right,
delete the row since the root is a Component already, but its statement shows the row and the name
as empty backticked spans, so the reader is not shown which row to delete.

## Details

### What

The statement reads "[project] components row `` gives the anchor the name ``": both spans print
the normalised empty path, not the row's text, `.` or `""`. The unit test that covers the refusal,
in the tests of `path@model@src/manifest.rs`, pins that exact text. Reported by the implementer of
the manifest's repair texts on the branch that repairs the finding texts, from its reading of the
code: the statement is built from the normalised candidate, after `normalise_one` has dropped the
`.`. Not reproduced by a run.

### Why it matters

`design@check@finding-names-the-repair` asks the finding to name the repair; a repair that says to
delete a row, beside a statement that does not show the row, leaves the reader to find it among the
rows of the table.

### What would close it

The statement shows the row as written in the manifest, and the unit test pins that text.
