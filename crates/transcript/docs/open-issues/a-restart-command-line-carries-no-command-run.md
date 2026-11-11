---
kind: todo
---
# The `local_command` line of `/restart` carries no `commandRun`, so every header reports it as a gap

## Summary

At Claude Code 2.1.321, the command `/restart` writes a `system` line of subtype `local_command`
that holds the command's name in its `content` and carries no `commandRun`, and no later line names
it as its parent. The survey of `path@transcript@src/reader.rs` counts such a line as the
unrecognised value `system local_command shape "no commandRun"`, so every run over a file holding
one ends its header with the request to report a gap, and the command prints nothing of the
owner's `/restart`.

## Details

### What

Measured on 2026-11-11 with `cargo x transcript-kinds --all-projects` over every transcript of the
owner's machine: the header holds
`# unrecognised system local_command shape "no commandRun": 2`, both in one top-level file of the
project thaum. Read in that file, with a `python3 -I` script printing each line's `type`, `subtype`,
`version`, `content` and `parentUuid`, the line before the second is the echo
`<local-command-stdout></local-command-stdout>`; the line holds
`<command-name>/restart</command-name>` with empty `<command-args>`; the line after it, its child,
holds `<local-command-stdout>Successfully updated from 2.1.321 to version 2.1.322</local-command-stdout>`.
`command_echoes` ignores a `local_command` line with no `commandRun` only where a line carrying
`commandRun` names it as its parent, and no such line follows here.

Not established: whether the first of the two counted lines is the same shape, and whether
2.1.322 still writes it.

The same header counts 10 `unlabelled` lines over the machine, at 2.1.309 and 2.1.321. Six carry
`turnOrigin` `sdk`, prompts of sessions run with `claude -p`, which
`design@transcript@transcript-labels-in-code` leaves unlabelled on purpose. The other four were not
examined.

### Why it matters

`design@transcript@transcript-format-tolerated-and-reported` has the header ask for a report only
for a value the reader does not know. A shape the harness writes for an ordinary command, reported
on every run over its file, trains a reader to skip that request, and the owner's `/restart` is
missing from the session `show` prints, where `owner-command` would place it.

### What would close it

The shape measured over the versions that write it, then either the reader taking the command
from the line's `<command-name>` when no `commandRun` is present, as a field-less tag the harness
writes first, argued against `design@transcript@transcript-labels-in-code`, or the line classed as
an echo ignored on purpose; a test over a fixture holding the line, and the four other unlabelled
lines examined and their outcome recorded.
