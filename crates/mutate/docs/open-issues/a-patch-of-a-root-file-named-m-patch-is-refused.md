---
kind: defect
---
# A `--patch` run on a file at the project's root named `m.patch`, or under a directory so named, is refused

## Summary

To plan a `--patch` mutant, the run stages the target in a scratch directory and writes the patch
beside it, as a file named `m.patch`. Where the target's path at the project's root is that name, the
patch overwrites the staged target; where the target is under a directory of that name, the write
fails. Either way the run is refused, exit code 2, as a patch that does not apply, and the file is
unchanged.

## Details

### What

`apply_in` in `path@mutate@src/run.rs`, named in prose per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`, joins the scratch directory with the target's path, writes the
pre-image there, then writes the patch at `m.patch` in the same directory, and runs `git apply m.patch`.

The reproduction, in a new git repository holding an empty knowledge-architect.toml, run with the
`xtask` binary this repository builds:

```sh
printf 'one\ntwo\nthree\n' | tee m.patch > g.txt
printf -- '--- a/m.patch\n+++ b/m.patch\n@@ -1,3 +1,3 @@\n one\n-two\n+TWO\n three\n' > m.patch.diff
sed s/m.patch/g.txt/ m.patch.diff > g.diff
xtask mutate run --file m.patch --patch m.patch.diff --no-baseline -- true   # refused
xtask mutate run --file g.txt --patch g.diff --no-baseline -- true           # applied
```

- The first run prints "the patch does not apply to m.patch:", then
  "error: patch failed: m.patch:1", exits 2, and leaves the file as it was.
- The second, the same patch over the same content under another name, prints "g.txt restored", reads
  `SURVIVED` and exits 0.
- A target `m.patch/x.txt` is refused with "Is a directory (os error 21)", exit 2.
- A target `sub/m.patch` is applied and restored, and reads `SURVIVED`: the collision is at the
  root alone.
- `--replace` on the root's `m.patch` is applied and restored: the form that writes no patch does not
  meet it.

Measured with git 2.53.0. Ruled out: the recovery record and the place of the scratch, since the write of
`m.patch` beside the staged target is the same line in the code before the scratch moved into the
record.

### Why it matters

A target file of that name cannot be mutated with a patch, and the message blames the patch. It is
refused before anything is written to the target, so nothing is lost. It strains no recorded decision.

### What would close it

The patch kept outside the directory the target is staged in, or handed to `git apply` on its
standard input, with a test over a root file and a directory named `m.patch`.
