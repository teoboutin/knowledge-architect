---
kind: design
---
# `reader.rs` is past one read

## Summary

`path@transcript@src/reader.rs` is more than a session reads whole in one pass: a Read with no range
returns it cut short. The size signal for a split of `design@agent-skills@module-split-guidelines`
holds. No work has weighed a split, so it is weighed at the design or the planning of the next work
that changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of the file with no range returned lines 1 to 1332 of 2336, with the message
  "showing lines 1-1332 of 2336 total (37240 tokens, cap 25000)", measured on the branch that made
  its tests hold their scratch directories in guards.
- **The signals against.** Not weighed: no work has counted the parts of the file that change apart,
  nor what every change reads together.

The instance: the commit "[tests] The tests outside `testing` hold each scratch directory in a
guard of `tempfile`, which removes it when the test ends" changed the file's test code, and its
branch's design discussion did not name the file among those past one read.

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
