---
kind: todo
---
# A result the harness cut is printed by `transcript show --full` as if whole

## Summary

The harness cuts a long result of a dynamic workflow or of a subagent before writing it into the
transcript, and writes in the line a pointer to the full output file in its task directory.
`transcript show --full` prints the cut text, and nothing in its header says that a text is cut, so
a transcript reviewer reads a cut report as whole.

## Details

### What

`transcript show --help`: "--full Print injected and tool texts whole". The instance: the transcript
review of the slice that moved the commands out of the checker, 2026-11-06. A workflow's result
reached the transcript cut, marked "truncated 138376 chars", the rest in the harness's task
directory under the user's cache; the transcript reviewer read those files by hand.

Measured at the intake of 2026-11-07 over every transcript of the owner's machine, by a
`grep -rhoE` of the phrase "truncated", a count and "chars": 50 cut texts carry no pointer, and the
others carry "full result in" and a path under the harness's task directory, some with
"Per-agent results:" after it.

`design@transcript@transcript-format-tolerated-and-reported` asks every run to report what it does
not know; a cut text is known to be cut, and is printed as given with no line of the header saying
so. `design@transcript@transcript-directory-resolution` reads the transcript directory alone, and the
output files lie outside it, so following the pointer widens what the reader reads, a decision.

The two shapes a file and the intake weighed: following the pointer to the output file, which
serves the reviewer more where the file is still there; and the header counting each cut text with
its pointer as a gap, within the format head's argument with no new input. The output files sit in a
temporary directory and may be gone when the reviewer reads.

### Why it matters

The transcript reviewer reads a cut report as whole and can miss an outcome it holds, against
`goal@transcript@a-session-reads-past-sessions`.

### What would close it

The header counting each cut text, with its pointer, or `show` printing the output file where it
exists, the second argued under `skill@knowledge-architect-design` since it widens what the reader
reads; with a test over a fixture holding a cut line.
