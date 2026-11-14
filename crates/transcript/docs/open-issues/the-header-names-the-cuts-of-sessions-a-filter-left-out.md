---
kind: design
---
# A filtered `transcript list` names in its header the cuts of sessions the filter left out

## Summary

The header of every verb of the transcript reader names each text the harness cut, over every file
the verb reads. A `list` filtered by branch or by words reads every file and lists a few sessions,
so its header names the cuts of sessions it excluded, before the rows asked for. Narrowing the
header to the listed sessions rewrites `design@transcript@cut-text-reported`.

## Details

### What

- `design@transcript@cut-text-reported`: "Every verb's header prints one line per such marker in a
  kept line its fields label injected or tool"; the header is taken over every file the verb reads,
  as the unrecognised values are, per `design@transcript@transcript-format-tolerated-and-reported`.
- Measured at the intake of 2026-11-14, on the owner's machine:
  `cargo klarch transcript list --branch planned-splits` lists two sessions, `80036d81` and
  `3e0588d2`, and its header prints 34 cut notices, in files of nine other sessions and of their
  subagents, 14 of them in one file, the transcript of session `221be2f0`; none is in a file of the
  two sessions listed, counted by `grep '^# cut'` over the output, grouped by file.

Instances:

- the session on the retrospective's standing questions, 2026-11-12, C2 of its workflow file:
  `transcript list --branch <branch>` and `--containing "<words>"` printed lines opening "# cut by
  the harness:" for subagent transcripts of other sessions, one naming "/t/x.output", before the one
  row that answered;
- the slice on the conformance check before the harvest, 2026-11-14, C1(b) of its workflow file:
  the same notices about other sessions' subagent files, "which the reader must skip".

The question the discussion answers is what a header is about. An unrecognised value is a fact
about the reader, true whichever session holds it, which argues for a header over every file read;
a cut text is a fact about one session's content, which a reader needs only for a session it reads.

The shapes: the header names the cuts of the listed sessions alone, the files' proposal; or no
change, since the notices are cheap, against which the run above carried 34 notices, every one
about a session it excluded.

The same discussion can settle `issue@transcript@a-persisted-tool-result-is-named-in-no-header`, a
second kind of cut the head does not name.

Re-entry point: a design discussion under `skill@knowledge-architect-design` on a branch of
`transcript`.

### Why it matters

A reader of a filtered list reads notices about sessions it did not ask for before the rows it
asked for, and learns to skip the cut notices that `design@transcript@cut-text-reported` prints so
that a reviewer does not read a cut text as whole.

### What would close it

The discussion decides what a header covers, rewriting `design@transcript@cut-text-reported` or
keeping it with this cost stated, and the shape is built with a test over transcript fixtures in
which a filtered `list` excludes a session holding a cut text.
