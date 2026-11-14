---
kind: design
---
# The prompt of a `claude -p` session, left unlabelled on purpose, asks for a finding in every header

## Summary

The reader leaves the first prompt of a `claude -p` session unlabelled by decision, and its header
asks that every unlabelled line be reported as a finding, without saying where the line is. In a
run over all projects on a machine where `claude -p` ran, the request fires in every header for
lines the design decided. Two sessions spent a finding on it; one could not locate the lines.

## Details

### What

The two heads that, together, make the request:

- `design@transcript@transcript-labels-in-code`: "A file with no field is a session run with
  `claude -p` ... its prompt carries `turnOrigin` `sdk` where a typed message carries `human`, and
  no field names who called `claude -p`, a person or a script, so its prompt is unlabelled and
  reported rather than given to the owner by a test on what it says";
- `design@transcript@transcript-format-tolerated-and-reported` gives `unlabelled` lines a count
  alone, where each unrecognised value gets its count and its first file and line, and ends the
  header, "where any of them is non-zero, with the line asking for it to be reported as a finding".

Measured on the owner's machine at the intake of 2026-11-14:

- `cargo klarch transcript list --all-projects` prints `# unlabelled lines: 6`, with no file or
  line;
- `cargo x transcript-kinds` gives them one row, "top user unlabelled (no field): 6 lines; versions
  2.1.309, 2.1.321", with keys `promptSource`, `turnOrigin` and `turnPosition`;
- a `python3 -I` script over `~/.claude/projects/*/*.jsonl`, listing each top-level `user` line with
  `promptSource` and none of `origin`, `isMeta`, `isCompactSummary` and `toolUseResult`, finds
  exactly six: the first prompt of six headless sessions, each `promptSource` `sdk` and
  `turnOrigin` `sdk`, five of them probes a review of this repository ran with `claude -p`;
- none is in this project's transcript directory: each lies in the directory the harness names
  after the scratch directory the probe ran in, so a run over this project alone, such as
  `cargo klarch transcript list --branch planned-splits`, prints no `unlabelled` line; only a run
  with `--all-projects`, or `cargo x transcript-kinds`, counts them.

Instances:

- the first slice of the goals' milestone, 2026-11-11, C1(b) of its workflow file: "4 lines are
  counted unlabelled", reported as asked;
- the session on the searcher's check and the reviewers' reports, 2026-11-11, C3 of its workflow
  file: "`transcript list` reports four unlabelled lines across all sessions"; "they were not
  located".

That the four lines of 2026-11-11 were of the `claude -p` kind is an assumption: the transcripts of
that day are no longer told apart from the later ones by the count. Ruled out: the owner's
`/compact` line the first file also reported, labelled `owner-command` since the change
"[transcript] A command the harness writes with no `commandRun` is the owner's, named by its leading
`<command-name>` tag".

The suspected mechanism, in one sentence: the request for a finding counts every unlabelled line,
the ones the labels head leaves unlabelled on purpose included.

The shapes, none argued yet:

- a label of its own for a `claude -p` prompt, placed by its field `turnOrigin` `sdk`, which gives
  it neither to the owner nor to injected text, so no test on its text is made;
- the header counting such prompts apart, with no request;
- the header naming the first file and line of the unlabelled lines, as it does for an
  unrecognised value, which locates them and keeps the request;
- no change, on the ground that the request is cheap; against it, the request fires in every run
  over all projects while the six lines exist, two sessions spent a finding on it, and one could
  not locate them.

`rejected@transcript@fields-alone-place-every-line` keeps the text rows, and bounds the first
shape.

Re-entry point: a design discussion under `skill@knowledge-architect-design` on a branch of
`transcript`, which every shape needs since each rewrites one of the two heads.

### Why it matters

The request is the route by which an unrecognised entry reaches the workflow's maintainers, per
`design@agent-skills@unrecognised-entries-reach-upstream` and
`goal@the-workflow-improves-through-real-use`; a request met by decided lines teaches a reader to
skip it, and a line the reader cannot locate cannot be diagnosed.

### What would close it

The discussion picks a shape, which rewrites `design@transcript@transcript-labels-in-code` or
`design@transcript@transcript-format-tolerated-and-reported`, or rules that the request stays as it
is; the shape built with a test over a transcript fixture holding a `claude -p` prompt.
