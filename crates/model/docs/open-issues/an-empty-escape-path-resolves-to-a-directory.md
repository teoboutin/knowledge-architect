---
kind: defect
---
# An escape path that spells nothing is reported as resolving to an unrelated directory

## Summary

`resolved_here` lets an escape path with no segment answer with the first anchor directory, so the finding names a repair that replaces the span with that directory.

## Details

### What

A span at the escape anchor whose path is a bare line suffix, a bare fragment, `/`, `//`, `.` or a directory and `..` is reported as resolving in this tree. For example, the bare suffix `:12` over the `core` mock gets a finding naming that mock's plans directory as the form to write. In `resolved_here` of `path@model@src/resolver.rs`, empty segments leave the candidate equal to the base.

Reproduced on the main branch before the crate split.

### Why it matters

It stands against `design@core@finding-names-the-repair`: following the repair writes a pointer nobody meant.

### What would close it

An escape path with no segment is silent or has a finding of its own, with a test over each spelling.
