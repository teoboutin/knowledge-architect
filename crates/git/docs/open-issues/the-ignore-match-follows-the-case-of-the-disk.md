---
kind: defect
---
# git's ignore match ignores case where the repository says so, so a reference passes on one machine and fails on another

## Summary

A `path` reference to an ignored path passes where git ignores it. Under `core.ignorecase=true`, which
`git init` sets on a disk that ignores case, `git check-ignore` matches a rule in any case, so a reference
spelled in another case than the rule passes on such a machine and fails on Linux.

## Details

### What

Run on Linux, with git 2.53.0, over a project of `klarch init`: a
`.gitignore` holding `build/` and the reference `path@<p>@Build/out.txt` in README.md. `klarch check` reports
"`path@<p>@Build/out.txt` does not exist, at `Build/out.txt`", exit 1. After
`git config core.ignorecase true` the same run passes, with no finding. The case where the disk itself
ignores case, on Windows and macOS, is not run.

The question is `ignored` of `path@git@src/git.rs`, which asks `git check-ignore -z --stdin` under the
repository's own configuration. The model asks it for each path a reference names, per
`path@model@CLAUDE.md`.

Ruled out: the listing itself, which is compared byte for byte, so a reference to a walked file in another
case fails on every machine.

### Why it matters

One tree passes on the machine that writes it and fails in CI on Linux, against
`goal@documentation-stays-consistent` and what `design@model@git-supplies-the-walk` supplies: one answer for
one tree.

### What would close it

The ignore question asked with `-c core.ignorecase=false`, if git's walk of the working tree on such a disk
bears it, with a test that sets the variable in its repository; or an argument that a machine's own setting
is to decide.
