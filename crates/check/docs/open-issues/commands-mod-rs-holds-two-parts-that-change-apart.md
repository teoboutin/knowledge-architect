---
kind: design
---
# `commands/mod.rs` holds two parts that change apart, and is near one read

## Summary

`path@check@src/commands/mod.rs` holds the refusal of a checker at another version than the pin beside the
runs of `check` and of `check --fix`. The two change apart, a signal for a split of
`design@agent-skills@module-split-guidelines`, and the file is close to what a session reads whole in one
pass. No work has split it, so it is weighed again at the design or the planning of the next work that
changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Two parts that change apart.** At main 863ad964, the refusal at lines 215 to 377 and the runs at lines
  425 to 657: `git log -L` over each range, with its commits counted once, gives 9 and 14 commits, 4 of
  them both. They are read against different heads, `design@check@checked-sentinel-values` and
  `design@check@fix-scope`.
- **Size.** 68 158 bytes there, returned whole by a Read with no range, whose cap is 25000 tokens.
- **The signals against.** The runs and the refusal share 4 commits of 19, and the runs share
  `generated_list`, `check_destinations` and `print_report`.

The instance: the branch that made the gates run on Linux, Windows and macOS changed the file, nearly every
edit replacing one call at one site, and split nothing. Its plan weighed the signals and opened no issue;
its review before the merge asked for this entry, since the signal here is of parts that change apart, by
the counts above, where the other modules the plan weighed, `references.rs`, `registers.rs` and `tree.rs`,
hold one of size.

A second instance: the slice of the per-crate changelogs that adds `klarch changelog` changes the message of the
refusal, a string literal with the binding and the comment before it, and its unit test, and
nothing of the runs. It plans no split, since that work does not need
one, and the signal stays as counted above.

### Why it matters

A review that reads the file whole judges two subjects, and a change of one shows the other in its diff,
against `goal@projects-are-split-into-modular-units`.

### What would close it

The split, or an argument that none pays against the signals, in the work that weighs it.

### Suspected mechanism

The refusal of a checker at another version grew beside the runs it guards, by works that each added to one
of them and weighed no split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@check@src/commands/mod.rs`.
