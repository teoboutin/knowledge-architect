---
kind: design
---
# `history.rs` is past one read

## Summary

`path@check@src/commands/history.rs` is more than a session reads whole in one pass: a Read with no
range returns it cut short. The size signal for a split of
`design@agent-skills@module-split-guidelines` holds. No work has yet needed the split, so it is
weighed again at the design or the planning of the next work that changes the file's code, per
`design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of the file with no range returned lines 1 to 1320 of 1980, with the message
  "showing lines 1-1320 of 1980 total (31860 tokens, cap 25000)", measured after the pass that
  documented every public item, on the branch that added the doc gate.
- **The signals against.** None counted: no work has counted the parts of the file that change apart,
  nor what every change reads together. The branch that made the tests hold their scratch directories
  in guards weighed the size signal alone, and found that its work needs no split. A lead, not established: the module's documentation names
  two parts, the judging of commit messages over a range and the snapshot of a tree read from git
  objects, which `check --staged`, `index --staged` and `check --fix` read too.

The instance: the commit "[check] Every public item and every Result-returning function of the
facade is documented" added doc comments to the file and changed no code of it. The plan of that
branch decided no split in that work, on the owner's word, and an issue for each file of the pass
past one read.

A later instance: the commit "[tests] The tests outside `testing` hold each scratch directory in a
guard of `tempfile`, which removes it when the test ends" changed the file's test code, and its
branch weighed a split in its design discussion and found that the work needs none, which the commit
"[clippy] A test or a tool that calls `std::env::temp_dir` fails the clippy gate, which names the
guard to hold instead" argues. The weighing passes to the next work that changes the file's code
after it.

A later instance: the work that had the gates run on Windows and macOS changed the file's code, the calls that print a path, and the currency line of `index_staged`. It
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

The judging of commit messages and the snapshot of a tree it needs grew in one module, by works that
each added to one of them and weighed no split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@check@src/commands/history.rs`.
