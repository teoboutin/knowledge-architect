---
kind: todo
---
# A tool result the harness persisted is shown as its preview, and no header names it

## Summary

The harness writes a long tool result to a file of its own and leaves in the transcript a block of
about 2 KB that opens `<persisted-output>`. The reader names in every header each text cut by the
marker `... (truncated <N> chars)`, and names none of these: a reviewer reads the preview as the
whole result. The header should name each persisted result with the path of its file.

## Details

### What

The persisted form of a cut tool result is a text that opens `<persisted-output>`, then the line
"Output too large (29.5KB). Full output saved to: <path>/tool-results/<id>.txt", then
"Preview (first 2KB):" and the preview. The size is the result's own.

- `design@transcript@cut-text-reported` names in every header each text the harness cut, and
  recognises only the marker `... (truncated <N> chars)`; `grep -rn -i persisted crates/transcript/`
  returns nothing at the intake of 2026-11-14.
- In one session of this repository, `cargo klarch transcript show 2a29314b --full` names one cut in
  its header, the `truncated` one, and not the persisted block its transcript holds.
- The text is not silent to a reader who looks: `show --full --tools` prints the block whole, its
  "Full output saved to" and "Preview (first 2KB):" lines included. Without `--tools` a Bash result
  is not printed at all.
- Measured by a `python3 -I` script over the top-level files `~/.claude/projects/*/*.jsonl` of the
  machine of this repository's owner, subagent files apart, at the intake of 2026-11-14: 84 tool
  results carry the block, 82 of `Bash` and 2 of `WebFetch`, none of `Agent`. Every "Full output
  saved to" path in those files, 106 occurrences by `grep -oh`, names a file under a `tool-results/`
  directory in the session's own directory under `~/.claude/projects/`, beside `subagents/`.

Instance: the intake of forty files, 2026-11-07. Three checking subagents' reports reached the
session as previews of about 2 KB; the transcript reviewer found the cut by hand, and one cut file
held the item that became its first major finding. That the three were Bash results is an
assumption.

A rival shape: `show` prints the persisted file whole where it exists. The head rejects printing
the whole for the `truncated` form on the ground that it reads outside the transcript directory
`design@transcript@transcript-directory-resolution` resolves, where the file may be gone. That
ground does not hold for a persisted result, whose file lies in the session's own directory. Not
established: whether the harness keeps `tool-results/` files as long as the transcript.

### Why it matters

A reviewer that reads a session through the reader takes a preview for a whole report, against the
argument of `design@transcript@cut-text-reported`; and the assembly of a plan document's transcript
reads through the same reader, per `design@agent-skills@ledger-from-transcript`, so it inherits the
gap.

### What would close it

A change to `transcript` under which every header names each persisted tool result with the path
its block gives, a member of `design@transcript@cut-text-reported` recorded directly, with a test
over a transcript fixture holding the block. The rival of printing the file whole is a change of
that head: it is argued under `skill@knowledge-architect-design`, on the measurement of how long
the harness keeps `tool-results/` files.
