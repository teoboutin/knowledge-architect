---
kind: design
---
# `tree.rs` is past one read

## Summary

`path@model@src/tree.rs` is more than a session reads whole in one pass: a Read of every line was
refused. The size signal for a split of `design@agent-skills@module-split-guidelines` holds. The
first work that changed the file's code with the signal known weighed it and took no split, so it
is weighed again at the design or the planning of the next work that changes the file's code, per
`design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of every line, with no size override, was refused at 32450 tokens, against a
  reader's limit of 25000, by the code-split scout dispatched at the design audit of slice 1 of
  `milestone@plans@per-crate-changelogs`, at the commit "[review] Repairs of the transcript review:
  D2 closed by the session on the owner's argument, and the owner's words restored where the record
  changed them". The code above the tests read whole, so the size comes from the tests.
- **Two parts that change apart.** The plan layout's code and `declarations`, the check of the
  declared paths, change apart; `issue@model@the-plans-layout-is-read-in-three-units` records the
  first part.
- **The signal against.** `declarations` is one process over the declared paths, and the findings
  slice 1 adds, a glob of `[walk.citation-free]` that matches no file and a path matched at both
  levels, are changes that process applies.

The instance: slice 1 of that milestone added those two findings to `declarations`, and its
design audit decided no split.

### Why it matters

A review that reads the file in parts judges code it has not read whole, which is the failure the
size signal names in `design@agent-skills@module-split-guidelines`.
`goal@projects-are-split-into-modular-units` asks for units with one purpose.

### What would close it

A split of the module planned and committed before the next work that changes its code, as
`skill@knowledge-architect-planning@cutting-steps-and-slices` says of a split, such as the tests
moved out; or that work's argued decision that no split pays.

### Suspected mechanism

Each check of phase 2 was added to the one module of the tree, with its tests inline, and none
weighed a split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@model@src/tree.rs`.
