---
kind: defect
---
# A path reference reaching through a symlink stops the whole run instead of being one finding

## Summary

`ignore_queries` asks git about the path of every anchored span it does not refuse, so a span whose path runs through a
symlink, such as `path@<anchor>@lnk/x.md` with `lnk` a symlink, makes `git check-ignore` fail, and the run exits 2 naming
no file and no line.

## Details

### What

`ignore_queries` in `path@model@src/resolver.rs` pushes the anchor's path joined to the span's path for each span whose path
`refused` accepts. A path whose first segment is a symlink passes `refused`, and git refuses the whole batch over it.

Reproduction, with git 2.53.0: copy the `core` mock of `path@testing@tests/projects/` outside any repository, pin it to the
binary's version, add a directory `real` and a symlink `lnk` to it, append to its README a line holding a path reference at the anchor
`core-mock` with the path `lnk/x.md`,
`git init` and `git add -A`, and run `klarch check`. It prints
``error: `git check-ignore -z --stdin` failed in ...: fatal: pathspec 'lnk/x.md' is beyond a symbolic link`` and exits 2.

Ruled out: the shapes `refused` refuses, a leading slash and an upward segment, at a named anchor or at `*`. The span arm of
`ignore_queries` asks git nothing about them, and `a_refused_path_shape_is_one_finding_and_the_run_goes_on` in
`path@testing@tests/binary.rs` shows each as one finding.

Not established: what the finding of such a span should be, since the walk keeps a symlink out, and whether the link arm,
which asks git about a relative markdown link through a symlink, fails the same way.

### Why it matters

It strains `design@check@finding-names-the-repair` and `design@check@phases-gate-the-report`: one prose span takes every
check from a project, with a message that names no file. `model`'s `Gathered::over` fails the same way for any tool over the
documents.

### What would close it

`ignore_queries` asks git about no path that runs through a symlink, and a test through the binary shows the finding such a
span gets.
