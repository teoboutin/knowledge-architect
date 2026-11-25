---
kind: design
---
# `reader.rs` is past one read

## Summary

`path@transcript@src/reader.rs` is more than a session reads whole in one pass: a Read with no range
returns it cut short. The size signal for a split of `design@agent-skills@module-split-guidelines`
holds. No work has yet needed the split, so it is weighed again at the design or the planning of the
next work that changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of the file with no range returned lines 1 to 1332 of 2336, with the message
  "showing lines 1-1332 of 2336 total (37240 tokens, cap 25000)", measured on the branch that made
  its tests hold their scratch directories in guards.
- **The signals against.** None counted: no work has counted the parts of the file that change apart,
  nor what every change reads together. The branch that made the tests hold their scratch directories
  in guards weighed the size signal alone, and found that its work needs no split.

The instance: the commit "[tests] The tests outside `testing` hold each scratch directory in a
guard of `tempfile`, which removes it when the test ends" changed the file's test code. Its branch
weighed a split after the change, since its design discussion had not named the file, and found that
the work needs none, which the commit "[clippy] A test or a tool that calls `std::env::temp_dir`
fails the clippy gate, which names the guard to hold instead" argues. The weighing passes to the
next work that changes the file's code after it.

A later instance: the work that had the gates run on Windows and macOS changed the file's test code, the helper that makes a symbolic link. It
weighed a split and made none, since nearly every edit replaces one call at one site and a move beside such
edits would hide them, which the commit "[model, check, query, create, xtask] A project-relative path is
printed with slashes on every system, by one function of the model" argues. The weighing passes to the next
work that changes the file's code after it.

### Why it matters

A review of a change to the file reads in parts a file it cannot read whole, which
`design@agent-skills@module-split-guidelines` names as the failure a split prevents.

### What would close it

A split of the module planned and committed before the next work that changes its code, as
`skill@knowledge-architect-planning@cutting-steps-and-slices` says of a split; or that work's argued
decision that no split pays, weighing the size signal against the signals against a split.

### Suspected mechanism

The reader, its labels and its tests share one module, and the works that added to each weighed no
split. Read from the file alone.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@transcript@src/reader.rs`.
