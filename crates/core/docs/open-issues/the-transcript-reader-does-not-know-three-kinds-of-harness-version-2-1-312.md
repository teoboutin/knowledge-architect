---
kind: todo
---
# The transcript reader does not know three kinds of line that harness version 2.1.312 writes

## Summary

`klarch transcript list` over a session of harness version 2.1.312 reports three values outside
the sets `path@core@src/transcript.rs` knows: the top-level type `tools-used`, the
`attachment.type` `task_reminder`, and the `system` subtype `stop_hook_summary`. Each is to be
classed as used or ignored on purpose.

## Details

### What

Seen in a cloud session of this repository on 2026-10-29, harness version 2.1.312, by
`cargo klarch transcript list --branch <branch>`, whose header printed:

```
# unrecognised attachment.type "task_reminder": 5, first at <session>.jsonl:75
# unrecognised system subtype "stop_hook_summary": 3, first at <session>.jsonl:128
# unrecognised type "tools-used": 6, first at <session>.jsonl:21
```

One line of each, from that session's file:

```
{"type":"tools-used","tools":["Bash"],"sessionId":"<id>"}
{"parentUuid":"<uuid>","isSidechain":false,"attachment":{"type":"task_reminder","content":[],"itemCount":0},"type":"attachment",...,"rendered":[{"content":"<system-reminder>\nThe task tools ..."}]}
{"parentUuid":"<uuid>","isSidechain":false,"type":"system","subtype":"stop_hook_summary","hookCount":1,"hookInfos":[{"command":"~/.claude/stop-hook-git-check.sh","durationMs":49}],"hookErrors":[],...}
```

The sets are `KNOWN_METADATA_TYPES`, `KNOWN_SYSTEM_SUBTYPES` and `KNOWN_ATTACHMENT_TYPES` in
`path@core@src/transcript.rs`. From the three lines alone, none appears to carry a message of the
owner or of the session: `tools-used` carries no `parentUuid` and reads as metadata beside the
chain, `task_reminder` is a reminder the harness injects, and `stop_hook_summary` reports a hook's
run. That is an assumption, not established over other sessions or versions.

### Why it matters

Until the three are classed, every run over such a session prints the three lines and asks for
them to be reported, which hides a new value that does matter among ones already known, against
`design@core@transcript-format-tolerated-and-reported`. A line misclassed as ignored would drop a
message from the transcript reviewer's extraction.

### What would close it

Each of the three is added to its set in `path@core@src/transcript.rs`, as used or ignored on
purpose, after the `find ... | jq ...` measurement that
`design@core@transcript-format-tolerated-and-reported` names is taken over the sessions at hand,
with a test over a fixture line of each; and `klarch transcript list` over a 2.1.312 session
reports none of them.
