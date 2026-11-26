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
  helpers. These are in the support module `path@testing@tests/support/mod.rs`, which binary.rs and
  another test file include, so that sharing holds across files and does not by itself argue for
  one file; not established that the build stays one, since each file under `path@testing@tests/`
  is a test crate of its own.

The instances: the spec of the record search's saved workflow, added by the commit "[plans] The spec
of the record search's saved workflow, with the closing sections and the verdict order a passing
`worklist --verify` writes", decided no split in that work, and so did the work of the commit
"[review] A cause's text is compared whole for a claim of absence, and four issue entries the review
found stale or missing are repaired", which argues it.

A later instance: the commit "[model] A plan document's directory holds its transcripts and its
search files numbered, one per design discussion and per design audit" changed the file's code,
and its branch weighed the size signal against no signal against a split, on the owner's direction
that work built before its spec takes in correctness changes only. The weighing passes to the next
work that changes the file's code after it.

A later instance: the commit "[testing] Each test over a mock owns its copy, and the scratch
directories that `testing` makes in the temporary directory go when its tests end" changed the
file's code, and its branch weighed a split in its design discussion and found that the work needs
none, which the commit "[clippy] A test or a tool that calls `std::env::temp_dir` fails the clippy
gate, which names the guard to hold instead" argues. The weighing passes to the next work that
changes the file's code after it.

A later instance: the work that had the gates run on Windows and macOS changed the file's code, the tests of line endings, the helpers that write a path into JSON, and the cases that a system lacks. It
weighed a split and made none, since nearly every edit replaces one call at one site and a move beside such
edits would hide them, which the commit "[model, check, query, create, xtask] A project-relative path is
printed with slashes on every system, by one function of the model" argues. The weighing passes to the next
work that changes the file's code after it.

A later instance: the commit "[testing] The binary's builder and the sandbox move into a support
module the test files share" moved `klarch()`, `Sandbox` and the helpers a new test file needed out
of the file into `path@testing@tests/support/mod.rs`, which both files include. Its branch weighed
the split by command at its slice's design audit, recorded in the commit "[plans] The design audit
of slice 1 of the per-crate changelogs: the glob findings in phase 2, the touched heads listed with
their verdicts, and four defaults for the owner", and made none: that work added no test of a
command to the file, and its new tests went in a file of their own. The weighing passes to the next
work that changes the file's code after it.

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
