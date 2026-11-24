---
kind: defect
---
# A `--patch` run is refused where its patch removes a line opening with `-- ` or adds one opening with `++ `

## Summary

`--patch` reads the paths a patch names from every line that opens with `--- ` or `+++ `, and a hunk
line that removes a line opening with `-- `, as a comment of SQL or Lua does, or adds one opening
with `++ `, has that form. The run takes the rest of it for a path and refuses the patch as naming
another file, exit code 2, though git applies it.

## Details

### What

`patch_paths` in `path@mutate@src/run.rs`, named in prose per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`, reads each line of the patch by its
opening, and does not read the hunk headers, which give how many lines of a hunk follow.

The reproduction, in a new git repository holding an empty knowledge-architect.toml, run with the
`xtask` binary this repository builds:

```sh
mkdir src && printf 'a\n-- old header\nb\n' > src/q.sql
printf -- '--- a/src/q.sql\n+++ b/src/q.sql\n@@ -1,3 +1,3 @@\n a\n--- old header\n+x\n b\n' > q.diff
git apply --check q.diff          # exit 0
xtask mutate run --file src/q.sql --patch q.diff --no-baseline -- true
```

The last prints "the patch names old header, which is not src/q.sql; a mutation changes the one file
--file names" and exits 2, with the file as it was. A patch that adds `++ new header` in place of the
line `x` of the same three-line file, `+++ new header` in its hunk, is refused the same way, with
"the patch names new header".
Observed with git 2.53.0. `--replace='-- old header'` over the same file is applied and restored,
with the `=`, since the command line takes a value that opens with a dash in no other form.

### Why it matters

A mutation of a file that holds such a line, as a comment of SQL, Lua, Haskell or Ada is, cannot be
written as a patch that touches it. The message names a path the patch
does not name, so it sends the caller to the wrong file. Nothing is lost, as the refusal is before
the write. It strains no recorded decision: `design@mutate@one-file-per-run` refuses a patch that
names another file, and this one does not.

### What would close it

The paths read from the headers of the patch alone, the lines of each hunk left out by the counts of
its header, with a test over a removed line opening with `-- ` and an added one opening with `++ `.
