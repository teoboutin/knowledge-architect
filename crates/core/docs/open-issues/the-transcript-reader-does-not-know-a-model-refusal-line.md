---
kind: todo
---
# The transcript reader does not know the `system` subtype `model_refusal_no_fallback`

## Summary

`klarch transcript list --all-projects` reports a `system` subtype outside the sets
`path@core@src/transcript.rs` knows: `model_refusal_no_fallback`, written by harness version
2.1.310 when the model refused a request and no fallback model ran. It is to be classed as used or
ignored on purpose.

## Details

### What

Seen during the intake of seventeen retrospective files of 2026-10-25 to 2026-10-31, on the owner's machine, by `cargo klarch transcript list --all-projects` over 947
files of harness versions 2.1.306 to 2.1.316, whose header printed:

```
# unrecognised system subtype "model_refusal_no_fallback": 4, first at <project>/<session>.jsonl:2747
```

The line's keys, its long strings cut: `type` `system`, `subtype` `model_refusal_no_fallback`,
`content` empty, `level` `warning`, `originalModel`, `requestId`, `apiRefusalCategory`,
`apiRefusalExplanation`, `refusedUserMessageUuid` (null in the line seen), `isMeta` false,
`parentUuid`, `uuid`, `timestamp`, `version` `2.1.310`. It carries no text the owner or the session
wrote; it reports that a turn ended in a refusal. That reading is an assumption over the four lines
of one session.

### Why it matters

Until it is classed, every run over such a session asks for it to be reported, against
`design@core@transcript-format-tolerated-and-reported`. A transcript reviewer that misses it reads
a turn with no answer as a turn the session ended.

### What would close it

The subtype is added to its set in `path@core@src/transcript.rs`, as used, labelled as a harness
line, or ignored on purpose, after the measurement that
`design@core@transcript-format-tolerated-and-reported` names, with a test over a fixture line; and
`klarch transcript list` over such a session reports it no more.
