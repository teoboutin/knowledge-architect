---
kind: deferred
---
# No mechanical check reads that a change to a declared performance path records its figures

## Summary

`design@agent-skills@performance-opt-in` has a change that touches a code path a project declares
performance-critical record its measurement before and after in its commit message, and the
code-shape reviewer reads that it does. A commit that records no figure passes every gate, and a
reviewer may miss it.

## Details

### What

Premortem cause 7 of the milestone that shipped the development skill: a change touches a declared
path and records no figures. Judged at its slice 2 harvest a check, not a tripwire, since a script
over a branch's commits could compute it from the declared paths and the commit messages. No target
project declares a path yet, so the check has no declaration to read.

### Why it matters

A regression on a declared path lands unmeasured, and the step `design@agent-skills@performance-opt-in`
rests on is followed only as far as a reviewer reads for it.

### Trigger

A design head of this repository, or a finding of a received retrospective file, declares a
performance-critical path: writing the check over that declaration is then the work.
