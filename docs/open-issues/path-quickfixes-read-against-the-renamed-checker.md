---
kind: question
---
# Does the quick-fixes spec still hold now that the installed-file findings are in the last phase?

## Summary

`spec@plans@path-quickfixes` cited the spec of the slice that renamed the core and moved the installed-file findings to the last phase, which left the plans directory at its harvest. The
quick-fixes spec plans a rewrite of the same order of a `check --fix` run. Does it still hold, accounting for what landed?

## Details

### What

The leaving spec rewrote `design@check@fix-before-the-checks`, which no longer says an upgrade takes two runs, and `design@check@phases-gate-the-report`, whose last phase now holds the family `agents`.
It also added repair text for a reference to a missing skill of the installer's namespace, in `path@model@src/resolver.rs`, and moved the shipped text to `path@check@src/agents.rs`. The quick-fixes spec's rows on the order
of a run, and its paths, were re-pointed mechanically and not read against these.

### Why it matters

A spec on hold written against a superseded order would be implemented from false statements, against `design@check@fix-before-the-checks`.

### What would close it

Reading `spec@plans@path-quickfixes` against those heads when its work resumes, correcting it in place, or a judgement that it holds. It leaves with that spec.
