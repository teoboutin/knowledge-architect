---
kind: defect
---
# Some inputs of `transcript` read as an answer about the project

## Summary

`transcript` keeps nothing, drops a file or picks a session in silence over inputs it accepts. Each predates the move to `transcript`, whose adversarial review found them.

## Details

### What

- `search --role tool word` prints the header alone, exit 1, while `--tools` beside it finds the lines, though `SearchOptions::roles` refuses another role "since it would keep nothing and read as an answer about the project".
- A file whose first chain line holds no `cwd` is left out of `list`, with no line reporting it, and `--all-projects` shows it. `path@transcript@src/reader.rs` says what it does not know it "never drops in silence".
- Two `sessionId`s sharing their first 8 characters give one handle, and `show` picks the first, exit 0, rather than "names 2 sessions".
- `show` of an unknown session prints no header, so the gaps `list` reports go unreported, against `design@transcript@transcript-query-verbs`, "Each prints a header first".

### Why it matters

`design@transcript@transcript-format-tolerated-and-reported` rests on every run reporting what it does not know.

### What would close it

Each case refuses or reports, with a test per case in `path@testing@tests/binary.rs`.
