---
kind: defect
---
# The transcript header's request to report each gap reads as covering the designed text fallback

## Summary

When a transcript file's lines carry none of the fields the labels read, `klarch transcript`
labels them by their text, as designed, and prints a header line saying so. The line after it asks
for "each" item above to be reported as a gap, though the code leaves the fallback out of the gaps.
A retrospective reported the fallback as asked.

## Details

### What

`header` in `path@core@src/cli/transcript.rs` pushes one line per fallback file, "no line carries
a field in <file>: its lines are labelled by their text", then, when `Inventory::has_gap` holds,
"each is a gap of this command: report it as a finding on the checker in the retrospective".
`has_gap` leaves the fallback out, but the gap line is printed right after the fallback lines, so a
reader takes "each" to cover them.

Reproduction: `cargo klarch transcript list --all-projects` on a machine holding a `claude -p`
transcript, whose lines carry none of the fields, and a session with an unrecognised line. Seen
during the intake of seventeen retrospective files of 2026-10-25 to 2026-10-31:

```
# unlabelled lines: 10
# no line carries a field in <scratch>/<session>.jsonl: its lines are labelled by their text
# each is a gap of this command: report it as a finding on the checker in the retrospective
```

C2 of the retrospective of the session that built `klarch transcript`, 2026-10-25, reported the
fallback as a finding because the header asked for every item. Ruled out: the fallback is a defect.
It is designed, per `design@core@transcript-format-tolerated-and-reported`.

### Why it matters

Every retrospective run on such a machine reports a designed behaviour as a gap, and an intake
spends a section on it, against `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

### What would close it

The gap line is printed before the fallback lines, or names what it covers, and a test asserts the
order or the wording.
