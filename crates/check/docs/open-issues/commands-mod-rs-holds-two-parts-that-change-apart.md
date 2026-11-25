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

- **Two parts that change apart.** At main 9f1efd7b, the refusal at lines 213 to 375 and the runs at lines
  423 to 660: `git log -L` over each range, with its commits counted once, gives 9 and 8 commits, 4 of
  them both. They are read against different heads, `design@check@checked-sentinel-values` and
  `design@check@fix-scope`.
- **Size.** 65 354 bytes there, returned whole by a Read with no range, whose cap is 25000 tokens; by the
  tokens of another file of this crate per byte, 1000 to 3000 under it.
- **The signals against.** The runs and the refusal share 4 commits of 13, and the runs share
  `generated_list`, `check_destinations` and `print_report`.

The instance: the branch that made the gates run on Linux, Windows and macOS changed the file, nearly every
edit replacing one call at one site, and split nothing. Its review before the merge asked for this entry.

### Why it matters

A review that reads the file whole judges two subjects, and a change of one shows the other in its diff,
against `goal@projects-are-split-into-modular-units`.

### What would close it

The split, or an argument that none pays against the signals, in the work that weighs it.
