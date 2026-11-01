---
kind: observation
---
# A reused process id reads as a live run, and blocks `restore` and `--discard`

## Summary

`restore` refuses while the run that claimed the record is alive, read from the pid the record
holds. A killed run's pid that another process has since taken, or a zombie, reads as alive, so
`restore` and `restore --discard` refuse with "still going" and exit 2 over a target that may hold
the mutant.

## Details

### What

Simulated by the adversarial reviewer of the branch that landed the crate: a stale record of a killed
run, its `owner-pid` replaced by the pid of an unrelated `sleep 300`; both `restore` and
`restore --discard` refused, exit 2. A real reuse needs the pid space to wrap, or a container with a
small one.

### Why it matters

While it lasts, the tool offers no way to restore the file, and its exit code, 2, says the run never
started where the tree is owed a look. The pre-image stays in the record, so no content is lost: a
caller can copy it back by hand. It strains `design@mutate@restore-by-comparison` only in that the
refusal guarding a live run cannot tell a dead one by its pid alone.

### What would close it

A liveness check that also matches the process's start time, read with the pid at the claim, or a
decision that the case is rare enough to leave, with the README naming the manual way back.
