---
kind: defect
---
# The reader does not know the `system` subtype `api_error`, so every transcript holding one reports a gap

## Summary

`klarch transcript show` over a session whose transcript holds a `system` line of subtype
`api_error` prints `unrecognised system subtype "api_error"` in its header, and its own output asks
the reader to report each such value as a finding on the checker. The subtype is missing from
`KNOWN_SYSTEM_SUBTYPES` in `path@transcript@src/reader.rs`.

## Details

### What

The session that designed `milestone@plans@split-guidelines-and-plan-directories`, extracted on
2026-11-13 by its transcript review with `cargo klarch transcript show <session> --from <opening
words> --full`, printed `unrecognised system subtype "api_error": 6`. The six lines are in the
session's top-level transcript file: `grep -c '"subtype":"api_error"'` over it gives 6. Each is a
`system` line of `level` `error` whose fields are `error`, `retryAttempt`, `maxRetries` and
`retryInMs` beside the fields every line carries: a report of an API call the harness retried. It
holds no message of the owner or of the session. In that session, a dispatched agent stopped twice
on an API authentication error before a third dispatch of it completed.

The reader notes each subtype outside the list with `note_unknown`, at the line of
`path@transcript@src/reader.rs` that tests `KNOWN_SYSTEM_SUBTYPES`.

### Why it matters

A header that names an unrecognised value tells the reader that the extraction may have dropped
content, and asks for a finding each time; a subtype that holds no content makes that warning
noise, which trains readers to skip it, and the warning exists for the value that does hold content.

### What would close it

`api_error` classed in `KNOWN_SYSTEM_SUBTYPES` as ignored, with a comment naming the fields it was
classed on, and a test that a line of that subtype raises no note, shown to fail without the entry.
