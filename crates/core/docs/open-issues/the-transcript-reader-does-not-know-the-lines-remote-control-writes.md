---
kind: todo
---
# The transcript reader does not know the lines a session under `/remote-control` writes, harness version 2.1.310

## Summary

`klarch transcript list` over the sessions of this repository on the owner's machine reports three
values outside the sets `path@core@src/transcript.rs` knows, all written by sessions of harness
version 2.1.310 that ran the `/remote-control` command: the top-level type `bridge-session`, the
`system` subtype `bridge_status`, and a `system` line of subtype `local_command` with no
`commandRun` field. Each is to be classed as used or ignored on purpose.

## Details

### What

Seen on 2026-10-29 by `cargo klarch transcript list`, run in this repository, whose header printed:

```
# unrecognised system local_command shape "no commandRun": 1, first at <session>.jsonl:452
# unrecognised system subtype "bridge_status": 1, first at <session>.jsonl:455
# unrecognised type "bridge-session": 219, first at <session>.jsonl:2
```

The 219 `bridge-session` lines are in two session files, and the `bridge_status` line in one of
them; each file records `"version":"2.1.310"`. One line of each, its long or identifying values
replaced:

```
{"type":"bridge-session","sessionId":"<id>","bridgeSessionId":"<id>","lastSequenceNum":162,"ownerAccountUuid":"<uuid>","ownerOrganizationUuid":"<uuid>"}
{"parentUuid":"<uuid>","isSidechain":false,"type":"system","subtype":"local_command","content":"<command-name>/remote-control</command-name>\n<command-message>remote-control</command-message>\n<command-args></command-args>","level":"info",...}
{"parentUuid":"<uuid>","isSidechain":false,"type":"system","subtype":"bridge_status","content":"/remote-control is active · Continue here, on your phone, or at <url>","url":"<url>",...}
```

The sets are `KNOWN_METADATA_TYPES` and `KNOWN_SYSTEM_SUBTYPES` in `path@core@src/transcript.rs`,
and the `local_command` line is reported where its parser reads `commandRun`. From these lines
alone: `bridge-session` carries no `parentUuid` and reads as metadata beside the chain;
`bridge_status` reports the bridge's state; the `local_command` line is the owner's command
`/remote-control`, whose shape differs from the command lines the reader knows. That is an
assumption, not established over other sessions or versions. The `tools-used` type, reported in the
same header, is `issue@core@the-transcript-reader-does-not-know-three-kinds-of-harness-version-2-1-312`'s.

### Why it matters

Until the three are classed, every run over such a session prints them and asks for them to be
reported, which hides a new value that matters among ones already known, against
`design@core@transcript-format-tolerated-and-reported`. The `local_command` line is a command the
owner typed: misclassed as ignored, it drops one of the owner's commands from the transcript
reviewer's extraction.

### What would close it

Each of the three is classed in `path@core@src/transcript.rs`, as used or ignored on purpose, after
the `find ... | jq ...` measurement that `design@core@transcript-format-tolerated-and-reported`
names is taken over the sessions at hand, with a test over a fixture line of each; and
`klarch transcript list` over a 2.1.310 session that ran `/remote-control` reports none of them.
