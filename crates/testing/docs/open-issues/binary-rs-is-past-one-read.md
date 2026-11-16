---
kind: todo
---
# `binary.rs`, the tests of the `klarch` binary, is past one read

## Summary

`path@testing@tests/binary.rs` holds the tests that run the `klarch` binary as a caller runs it,
267 tests in 10931 lines, more than a session reads whole in one pass. The size signal for a split of
`design@agent-skills@module-split-guidelines` holds, and one signal against.

## Details

### What

- **Size.** `wc -l crates/testing/tests/binary.rs` gives 10931 lines, and
  `grep -c '^#\[test\]' crates/testing/tests/binary.rs` 267 tests, counted at the review before the
  merge of the branch that closed the mutation survivors of three suites. The Read tool reads 2000
  lines by default.
- **The signal against.** The tests share `klarch()`, the one build of the binary, and the sandbox
  helpers. A support module shared by several test files keeps that sharing across files, so it
  does not by itself argue for one file; not established that the build stays one, since each file
  under `path@testing@tests/` is a test crate of its own.

The spec of the record search's saved workflow, added by the commit "[plans] The spec of the record
search's saved workflow, with the closing sections and the verdict order a passing `worklist
--verify` writes", decided no split in that work, and so did the work of the commit "[review] A cause's text is compared whole for a claim of absence, and four issue
entries the review found stale or missing are repaired", which argues it.

### Why it matters

A review of a change to the tests of one command reads in parts a file it cannot read whole, which
`design@agent-skills@module-split-guidelines` names as the failure a split prevents.

### What would close it

A split of the file by command, planned and committed before a piece of work that adds tests of a
command to it, as `skill@knowledge-architect-planning@cutting-steps-and-slices` says of a split,
with the shared build and helpers in one support module; or that work's argued decision that no
split pays.
