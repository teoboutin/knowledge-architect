---
kind: defect
---
# `show` answers otherwise than the check on some references, and `issues` calls a new file's date unknown

## Summary

`show` and `issues` disagree with the model or with their own comments over inputs the check accepts. Each predates the move to `query`, whose review found them.

## Details

### What

- `show 'path@*@README.md'` prints `no anchor is named *`, exit 1, while the check accepts that reference and the primer uses the form; `show path@<anchor>@README.md` then ends `referenced by nothing`, since `cited_at` adds the `*` citations for a register alone.
- `section_text` takes a heading only from `## ` and `# ` at the line's start, and opens a fence on any line starting with three backticks. So `show instructions@<id>` runs past a heading indented by one space, or written `##` and a tab, both of which the model defines, and past the end of the file after a paragraph that opens with an inline span of three backticks.
- `issues` prints `-` for a new file in a repository with history, where `last_changed` covers the issue directories and they hold no commit, against its comment that `-` means no history to ask.
- `show path@<anchor>@.` answers, where the check refuses that form, unlike the `planned` and `register` branches.
- A mutation that removes `kind.is_named() && anchor.is_empty()` in `show`, which stops `show skill@x` at its first heading, survives every test, and so does one that swaps the `-` and `uncommitted` of `issues`.

### Why it matters

`design@check@one-question-one-command` and `goal@records-reach-their-reader` rely on `show` naming every citation of an entry, and `path@query@src/show.rs` says a reference the check refuses resolves to nothing there either.

### What would close it

Each case gives the check's answer, with a test per case in `path@testing@tests/binary.rs`.
