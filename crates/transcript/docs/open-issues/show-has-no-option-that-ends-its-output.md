---
kind: todo
---
# `transcript show` has no option that ends its output at a message

## Summary

`klarch transcript show` starts its output at a message, `--from`, and has no option that ends it.
A committed transcript ends at the owner's word on the last convergence checkpoint, so every one is
cut by hand after `show` has printed it.

## Details

### What

- `ShowOptions` in `path@transcript@src/command.rs` holds `session`, `from`, `subagents`, `tools`,
  `full`, `all_projects` and `json`: a start, as `ShowFrom::Text` or `ShowFrom::Line`, and no end.
  `path@cli@src/cli/transcript.rs` maps only `from` and `from_line`.
- `design@agent-skills@committed-transcript` ends a committed transcript at the owner's word on the
  last checkpoint table of the discussion, and the planning skill says to cut the last session's
  output after that message of the owner, since `show` cannot end it.
- The missing work: an option that ends the output after a message, by its text or by its line, as
  `--from` starts it.

### Why it matters

A cut by hand is a session reading the file for the end point and deleting the rest: a cut one
message late commits words after the checkpoint, which `design@agent-skills@committed-transcript`
leaves out, and a cut one message early loses the owner's word on the checkpoint, which no check
reports.

### What would close it

The option built in `klarch transcript show`, with its test, and the planning skill's extraction and
`design@agent-skills@committed-transcript` using it in place of the cut by hand.
