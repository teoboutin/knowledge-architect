---
kind: defect
---
# A path reference holding a refused shape, or reaching through a symlink, stops the whole run instead of being one finding

## Summary

`ignore_queries` asks git about the path of every anchored span with no test of its shape, so a span such as `path@<anchor>@/x` makes `git check-ignore` fail, and the run exits 2 naming no file and no line.

## Details

### What

`ignore_queries` in `path@model@src/resolver.rs` pushes the anchor's path joined to the span's path for each span. Its arm for a markdown link tests `refused` first; its arm for a span does not.

Reproduction: copy the `core` mock of `path@testing@tests/projects/`, write a span naming that mock's anchor with the path `/etc/passwd` into its README, and run `check`. It prints ``error: `git check-ignore -z --stdin` failed in ...: fatal: /etc/passwd: '/etc/passwd' is outside repository`` and exits 2. An upward path above the root, at a named anchor, at `*` or under `planned`, does the same, and so does a path whose first segment is a symlink the walk keeps out, with `fatal: pathspec ... is beyond a symbolic link`.

The same span makes `index` and `check --staged` exit 2, and `commits` report that the commit's tree does not load. The unit test `an_upward_a_current_and_an_absolute_path_are_refused` passes, since it injects the ignore answers and never spawns git. So through the binary the `Refused` and `EveryRefused` causes are never reached for these shapes.

Reproduced on the main branch before the crate split: not caused by the move. Found by a search of the record for "outside repository": no entry held it.

### Why it matters

It strains `design@check@finding-names-the-repair` and `design@check@phases-gate-the-report`: one prose span takes every check from a project, with a message that names no file. And `model`'s `Gathered::over` now fails the same way for any tool over the documents.

### What would close it

`ignore_queries` asks git about no path `refused` refuses, and a test through the binary shows the finding of each shape, a path through a symlink included.
