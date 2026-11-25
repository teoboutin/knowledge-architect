---
kind: defect
---
# A pointer or a manifest row holding a backslash or a drive prefix is read by the system's rules, so it passes on Windows and fails on Linux

## Summary

A `path` reference, a relative link and a manifest row that hold `\` are read as paths with a separator on
Windows and as one name on Linux, so one tree gets two verdicts. A reference holding a drive prefix is read
as an absolute path on Windows. And a tracked document replaced by a directory reads there as a refused
access, so its finding names the wrong repair. Read and simulated, not run on Windows.

## Details

### What

- `refused` of `path@model@src/resolver.rs` judges a pointer's shape by splitting at `/` alone, so
  `path@<one>@src\x.rs` is refused nowhere. The adversarial review of the branch that ported the gates ran the
  checker on Linux over a standard library whose separator is `\`: it accepted that reference, where the
  ordinary build reports it missing.
- The same function refuses a leading `/` and no drive prefix. On Windows a join with `C:/…` gives that
  absolute path, and the ignore question over a path outside the repository is a failure of git, which
  stops the run. Read, not run.
- `normalise_one` of `path@model@src/manifest.rs` reads a row's segments by the system's rules: the row
  `a\b` is the path `a/b` on Windows and one name on Linux. A row with a drive prefix is refused on Windows
  as not project-relative, and is a relative path on Linux.
- `unread` of `path@model@src/model.rs` names the repair of a directory in a document's place for
  `ErrorKind::IsADirectory`. On Windows the read of a directory fails with `PermissionDenied` (assumption,
  from the system's "Access is denied"; not run), which takes the arm for a file the user cannot read.

The refusal of a `\` was ruled out of the port, on an argument: a document that passes on Linux holds none
that resolves; a `skip-files` row is how a project on Linux keeps a file whose name holds `\`, and a refusal
of `\` in a row removes it; and it makes the check of past commits stricter.

### Why it matters

Two systems disagree about one tree, so a check that passes where a document is written fails in CI, against
`goal@documentation-stays-consistent` read as one rule for every machine. It strains
`design@model@unportable-names-refused`, which refuses a name that one system cannot hold and says nothing
of a pointer to one.

### What would close it

A discussion under `skill@knowledge-architect-design` that decides whether a pointer holding `\` or a drive
prefix is refused, what becomes of `skip-files` rows, and where the branch that tightens it is placed; then
the refusal, or the reason there is none; and the repair of a directory in a document's place told on
Windows, by a test run there.
