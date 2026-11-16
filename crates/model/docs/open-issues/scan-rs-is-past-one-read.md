---
kind: design
---
# `scan.rs` is past one read

## Summary

`path@model@src/scan.rs` is more than a session reads whole in one pass: a Read with no range
returns it cut short. The size signal for a split of `design@agent-skills@module-split-guidelines`
holds. The work that last wrote into it, the pass that documented every public item the
documentation lints name, wrote only documentation and weighed no split, so the split is weighed at
the design or the planning of the next work that changes the file's code, per
`design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of the file with no range returned lines 1 to 1227 of 1635, with the message
  "showing lines 1-1227 of 1635 total (28295 tokens, cap 25000)", measured after that documentation
  pass, on the branch that added the doc gate.
- **The signals against.** Not weighed: no work has counted the parts of the file that change apart,
  nor what every change reads together.

The instance: the commit "[model] Every public item the documentation lints name is documented"
added doc comments to the file and changed no code of it. The plan of that branch decided no split
in that work, on the owner's word, and an issue for each file of the pass past one read.

### Why it matters

A review of a change to the file reads in parts a file it cannot read whole, which
`design@agent-skills@module-split-guidelines` names as the failure a split prevents.

### What would close it

A split of the module planned and committed before the next work that changes its code, as
`skill@knowledge-architect-planning@cutting-steps-and-slices` says of a split; or that work's argued
decision that no split pays, weighing the size signal against the signals against a split.

### Suspected mechanism

The one pass over a document's prose grew in one module, by works that each taught it a form and
weighed no split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@model@src/scan.rs`.
