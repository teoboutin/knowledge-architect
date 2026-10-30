---
kind: defect
---
# The transcript list previews a pasted-content tag instead of the owner's words, when the first message is pasted

## Summary

`klarch transcript list` prints, at the end of each session's line, an excerpt of the session's
first message the owner typed. Where that message is wrapped in the harness's pasted-content tag,
the excerpt shows the tag, `<pasted_content id="…">`, and none of the owner's words.

## Details

### What

Seen on 2026-10-30, in this repository, for the session of the design discussion on goals, by
`cargo klarch transcript list --containing "I want to discuss the goal setting skill"`. Its line,
the identifiers kept since they identify nothing outside the machine:

```
a67d6180  2026-10-30 12:36  2026-10-30 13:28  main, goal-standard  owner 3  compactions 0  subagents 13  <pasted_content id="eaaa">
```

The excerpt is built in the list command of `path@core@src/cli/transcript.rs`: the first entry whose
label is owner-typed, passed through `one_line` with `LIST_EXCERPT`. The message's text opens with
blank lines and the opening tag, so the excerpt holds the tag alone. The `--containing` match is not
affected: it found the session by words inside the tag.

### Why it matters

A session confirms a transcript by its opening words, as the installed transcript reviewer's
`agent@knowledge-architect-transcript-reviewer@extract-transcripts` asks. When the preview shows the
tag, the reader cannot see the words it matched, so two sessions holding the same words cannot be
told apart from the list, against `goal@core@records-reach-their-reader`.

### What would close it

The excerpt skips the harness's pasted-content tags and starts at the message's first words inside
them, with a test over a session whose first owner message is pasted.
