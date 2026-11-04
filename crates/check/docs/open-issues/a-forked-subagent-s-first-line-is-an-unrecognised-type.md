---
kind: defect
---
# A forked subagent's transcript opens with a line of type `fork-context-ref`, which `klarch transcript` counts as unrecognised

## Summary

The harness writes, as the first line of a forked subagent's transcript, a line of type
`fork-context-ref`. `klarch transcript list` and `show` report it in their header as an unrecognised
type, and ask for it to be reported as a gap of the checker. The line carries no message: it names
the fork's agent, its parent session, the parent's last line and the length of the context the fork
inherits.

## Details

### What

Reproduction, on the owner's machine on 2026-11-02, over transcripts of harness versions 2.1.306 to
2.1.316: `cargo klarch transcript list` prints, in its header,

```text
# unrecognised type "fork-context-ref": 4, first at -home-teo-dev-knowledge-architect/9a4cb430-e75c-4bcf-a01d-03625a4db9e6/subagents/agent-a2399b9c41d6288ad.jsonl:1
```

The four are the first lines of the four forks that session dispatched. The first line of that file
is, in full:

```json
{"type":"fork-context-ref","agentId":"a2399b9c41d6288ad","parentSessionId":"9a4cb430-e75c-4bcf-a01d-03625a4db9e6","parentLastUuid":"250b4f78-fe43-4282-9f22-04d234025031","contextLength":67}
```

Expected: a known type, read or skipped as the head on the transcript's labels decides. Actual: the
`match` on the line's `type` in `path@check@src/transcript.rs` knows `user`, `assistant`,
`attachment`, `system` and the types it lists, and notes any other through `note_unknown`.

### Why it matters

Every run over a session that forked a subagent prints the gap and asks for a report, so the header
stops telling a reader that something new is unread: a real unknown type would sit beside a known
one. The line also says which part of the parent's transcript the fork inherited, which a
transcript reviewer reading a fork's report needs to tell what the fork saw from what it read.

### What would close it

`klarch transcript` reads `fork-context-ref` as a known type: skipped, or shown as a line that names
the parent session and the context inherited, as `design@check@transcript-labels-in-code` decides for
a line no field labels as the owner's; and a test over a fixture holding the line.
