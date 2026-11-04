---
kind: question
---
# Would the model's structural findings be better checked at scan time?

## Summary

The owner believes the structural findings of phases 2 and 3, such as a slug defined twice, make more sense
checked at scan time. They run today after the scan, in a separate call.

## Details

### What

`Model::build` reads a project and `foundation` judges what was read, so a tool can read a model it never
asks about, and `show` does. The owner stated the preference as a belief, on the question of where these
findings belong, without asking for a change at this slice.

### Why it matters

It bears on `design@core@phases-gate-the-report` and on `goal@model@a-tool-knows-the-read-is-right`: with these
findings at scan time, a tool has no way to read a model that is wrong, which `show` relies on today, since what it
prints exists whatever else the model misses.

### What would close it

A design discussion under `skill@knowledge-architect-design`, at the audit of the slice that moves the commands
over the model, `spec@component-split@consumers-out`, whether to run these findings as part of the build.
