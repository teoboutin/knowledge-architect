---
kind: design
---
# `manifest.rs` is past one read

## Summary

`path@model@src/manifest.rs` is more than a session reads whole in one pass: a Read with no range
returns it cut short. The size signal for a split of `design@agent-skills@module-split-guidelines`
holds. No work has yet needed the split, so it is weighed again at the design or the planning of the
next work that changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of the file with no range returned lines 1 to 1410 of 3512, with the message
  "showing lines 1-1410 of 3512 total (52899 tokens, cap 25000)", measured after the pass that
  documented every public item, on the branch that added the doc gate.
- **The signals against.** None counted: no work has counted the parts of the file that change apart,
  nor what every change reads together. The branch that made the tests hold their scratch directories
  in guards weighed the size signal alone, and found that its work needs no split.

The instance: the commit "[model] Every public item the documentation lints name is documented"
added doc comments to the file and changed no code of it. The plan of that branch decided no split
in that work, on the owner's word, and an issue for each file of the pass past one read.

A later instance: the commit "[model] A plan document's directory holds its transcripts and its
search files numbered, one per design discussion and per design audit" changed the file's code,
and its branch weighed the size signal against no signal against a split, on the owner's direction
that work built before its spec takes in correctness changes only. The weighing passes to the next
work that changes the file's code after it.

A later instance: the commit "[tests] The tests outside `testing` hold each scratch directory in a
guard of `tempfile`, which removes it when the test ends" changed the file's test code, and its
branch weighed a split in its design discussion and found that the work needs none, which the commit
"[clippy] A test or a tool that calls `std::env::temp_dir` fails the clippy gate, which names the
guard to hold instead" argues. The weighing passes to the next work that changes the file's code
after it.

A later instance: the work of `spec@plans@gates-on-windows-and-macos` changed the file's code, the calls that print a path. It
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

The reading of the manifest and its refusals grew in one module, by works that each added a key or a
refusal and weighed no split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@model@src/manifest.rs`.
