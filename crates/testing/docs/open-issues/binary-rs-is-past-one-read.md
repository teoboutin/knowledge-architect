---
kind: design
---
# `binary.rs`, the tests of the `klarch` binary, is past one read

## Summary

`path@testing@tests/binary.rs` holds the tests that run the `klarch` binary as a caller runs it,
more than a session reads whole in one pass: a Read with no range refuses it. The size signal for a
split of `design@agent-skills@module-split-guidelines` holds, and one signal against. No work has
yet needed the split, so it is weighed again at the design or the planning of the next work that
changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of the file with no range returned no content, with the message "File content
  (464.4KB) exceeds maximum allowed size (256KB)", measured on the branch that wrote
  `design@agent-skills@split-not-needed-is-an-issue`; `wc -l` gave 10979 lines then. It holds 268
  tests, counted by `grep -c '^#\[test\]' crates/testing/tests/binary.rs` on the same branch.
- **The signal against.** The tests share `klarch()`, the one build of the binary, and the sandbox
  helpers. A support module shared by several test files keeps that sharing across files, so it
  does not by itself argue for one file; not established that the build stays one, since each file
  under `path@testing@tests/` is a test crate of its own.

The instances: the spec of the record search's saved workflow, added by the commit "[plans] The spec
of the record search's saved workflow, with the closing sections and the verdict order a passing
`worklist --verify` writes", decided no split in that work, and so did the work of the commit
"[review] A cause's text is compared whole for a claim of absence, and four issue entries the review
found stale or missing are repaired", which argues it.

### Why it matters

A review of a change to the tests of one command reads in parts a file it cannot read whole, which
`design@agent-skills@module-split-guidelines` names as the failure a split prevents.

### What would close it

A split of the file by command, planned and committed before a piece of work that adds tests of a
command to it, as `skill@knowledge-architect-planning@cutting-steps-and-slices` says of a split,
with the shared build and helpers in one support module; or that work's argued decision that no
split pays.

### Suspected mechanism

Each work that adds a test of a command appends it to the one file, and none weighed a split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@testing@tests/binary.rs`, such as one that adds tests of a command to it.
