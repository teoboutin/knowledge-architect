---
kind: design
---
# A command the owner ran with the `!` prefix is unlabelled in a transcript

## Summary

`klarch transcript` labels no `<bash-input>` line, nor the `<bash-stdout>` or `<bash-stderr>` line
after it, which a command the owner typed with the `!` prefix writes. Both are acts of the owner.
`design@core@transcript-labels-in-code` places a line by its fields, "never by a substring of its
text", and these lines carry no field that tells them apart from a typed message.

## Details

### What

Instances, each counted by `cargo klarch transcript list --all-projects` and a read of the lines:

- C1 of the retrospective of the session that built `klarch transcript`, 2026-10-25: one
  `<bash-input>` line with its output.
- On the owner's machine on 2026-10-31, over 956 files of harness versions 2.1.306 to 2.1.316: the
  header's "unlabelled lines: 2" are that `<bash-input>` line and its `<bash-stdout>` line, the only
  pair on the machine. The `<bash-input>` line carries none of `isMeta`, `isCompactSummary`,
  `origin` or `turnOrigin`; the `<bash-stdout>` line carries `turnOrigin` `human`, which two typed
  messages with no `origin` also carry. No `<bash-stderr>` line was found.

A text test confined to the lines no field places would move a line into an owner label and never
out of one, since the owner's typed messages are placed by `origin.kind` first. That is the
argument for labelling the pair by its leading tag; it contradicts the head, so it is a reversal of
part of it, under `skill@knowledge-architect-decision-recording@reversal-check`. The evidence is one
pair, written by one harness version.

### Why it matters

A transcript reviewer that filters on the owner's lines misses the pair, against
`goal@knowledge-architect@the-owner-s-word-is-verified`. The harness asks a session to have the
owner run an interactive command with that prefix, so its output can be what the session acted on.
Every run over a session that holds the pair prints "unlabelled lines" and asks for it to be
reported as a gap.

### Re-entry point

The next change to the labels of `path@core@src/transcript.rs`, a second instance of the pair in a
retrospective file, or the owner's word.

### What would close it

A discussion under `skill@knowledge-architect-design` that either labels the pair, rewriting
`design@core@transcript-labels-in-code` to admit a text test for a line no field places, or keeps
the head and names the two shapes in the transcript reviewer's text, so it reads unlabelled lines.
