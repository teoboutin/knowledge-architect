---
kind: design
---
# An owner's interrupt and a command the owner ran with the `!` prefix are unlabelled in a transcript

## Summary

`klarch transcript` labels no line of `[Request interrupted by user]`, nor a `<bash-input>` line
with the `<bash-stdout>` or `<bash-stderr>` line after it, which a command the owner typed with the
`!` prefix writes. Both are acts of the owner. `design@core@transcript-labels-in-code` places a
line by its fields, "never by a substring of its text", and these lines carry no field that tells
them apart from a typed message.

## Details

### What

Instances:

- C1 of the retrospective of the session that built `klarch transcript`, 2026-10-25: 4 interrupt
  lines in 3 sessions, and one `<bash-input>` line with its output, at harness versions 2.1.306 to
  2.1.310.
- C1 of the retrospective of the intake deciding by authority, 2026-10-31: the header's
  "unlabelled lines: 3" every run, with nothing recording them.
- the intake of seventeen retrospective files of 2026-10-25 to 2026-10-31: `cargo klarch transcript list --all-projects` on the owner's machine reported 10 unlabelled
  lines, 3 of this repository: 7 `[Request interrupted by user]`, 1 `[Request interrupted by user
  for tool use]`, and one `<bash-input>` line with its `<bash-stdout>` line. A `jq` read of each
  found no field that differs from a typed message's.

Suspected mechanism: the harness writes these acts as user messages whose text alone carries their
kind, and the head's rule refuses a text test.

The received file proposed two label rows, an owner interrupt and an owner shell command, matched
by the line's text where no field places it. That contradicts the head, so it is a reversal of part
of it, under `skill@knowledge-architect-decision-recording@reversal-check`.

### Why it matters

A transcript reviewer that filters on the owner's lines misses both, against
`goal@knowledge-architect@the-owner-s-word-is-verified`. The `!` case weighs most: the harness asks
a session to have the owner run an interactive command with that prefix, so its output can be what
the session acted on. A label would also change which line
`issue@core@the-transcript-list-previews-a-pasted-content-tag-instead-of-the-owner-s-words`
previews.

### Re-entry point

The next change to `path@core@src/transcript.rs`'s labels, or the next retrospective file that
reports an owner's act the transcript reviewer missed.

### What would close it

A discussion under `skill@knowledge-architect-design` that either labels these lines, rewriting
`design@core@transcript-labels-in-code` to admit a text test for a line no field places, or keeps
the head and names the two shapes in the transcript reviewer's text, so it reads unlabelled lines.
