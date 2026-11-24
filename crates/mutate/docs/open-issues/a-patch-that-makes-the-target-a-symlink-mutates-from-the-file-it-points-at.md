---
kind: defect
---
# A `--patch` run whose patch replaces the target by a symlink mutates the target to the content of the file the link points at

## Summary

`--patch` accepts a patch that deletes the target and creates it again as a symbolic link. The run
reads the result of the patch through that link, so the mutant is the content of the file the link
points at, which may lie outside the project, and not what the patch states. The target is restored
after the run.

## Details

### What

`apply_in` in `path@mutate@src/run.rs`, named in prose per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`, reads the patched file with
`fs::read_to_string`, which follows a link, in the scratch repository where `git apply` made it one.
Every check on the target's kind is made on the project's own file, before the patch is applied there.

The reproduction, in a new git repository holding an empty knowledge-architect.toml, run with the
`xtask` binary this repository builds:

```sh
mkdir src && printf 'line1\nline2\nSITE\nline4\nline5\n' > src/f.rs
printf 'diff --git a/src/f.rs b/src/f.rs\ndeleted file mode 100644\n--- a/src/f.rs\n+++ /dev/null\n@@ -1,5 +0,0 @@\n-line1\n-line2\n-SITE\n-line4\n-line5\ndiff --git a/src/f.rs b/src/f.rs\nnew file mode 120000\n--- /dev/null\n+++ b/src/f.rs\n@@ -0,0 +1 @@\n+/etc/hostname\n\\ No newline at end of file\n' > s.diff
xtask mutate run --file src/f.rs --patch s.diff --no-baseline -- cat src/f.rs
```

It prints "mutation applied to src/f.rs, first changed line 1", then the machine's host name, then
"src/f.rs restored" and `SURVIVED`, and exits 0. The file then holds its first content. Observed with
git 2.53.0 on Linux. Ruled out: the place of the scratch, since an adversarial reviewer saw the same
on the code that kept it in the temporary directory. Not established: a link whose target does not
exist, and a link into the project.

### Why it matters

The verdict is about a mutant the caller did not write, and the run writes the content of a file
outside the project into the target, where the command under check reads it. Nothing is lost: the
restore writes the pre-image back, per `design@mutate@restore-by-comparison`.

### What would close it

The result of the patch refused where it is no regular file, with the reason in the message, and a
test over a patch that makes the target a symlink.
