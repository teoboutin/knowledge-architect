---
kind: defect
---
# The primer's import line is accepted inside some fences and comments

## Summary

`imports_primer` of `path@check@src/agents.rs` says a line inside a fence or an HTML comment does not count. Three
shapes pass the check while the harness evaluates no import.

## Details

### What

Each written over the root CLAUDE.md, then `klarch check` exits 0, as an adversarial reviewer ran:

- a four-backtick fence holding a three-backtick line and the import line, since only three characters are compared;
- a three-backtick fence, a line opening with three backticks and an info string, then the import line, since such a
  line closes the fence here;
- a comment opened in the middle of a line, then the import line.

### Why it matters

A root CLAUDE.md in one of these shapes delivers no primer to any session and passes
`design@check@primer-import-line`'s check, against `goal@agents-get-a-complete-workflow`.

### What would close it

The function reads fences and comments as markdown does, with a test per shape above.
