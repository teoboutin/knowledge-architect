---
kind: defect
---
# `init` outside a worktree deletes a file of the namespace and lists nothing

## Summary

`init --vcs none` in a directory that holds an agent file of the installer's namespace, under the name knowledge-architect-mine.md, removes it and prints no
line naming it. No git holds a copy. Inside a worktree the same file gets `fixed: removed`.

## Details

### What

`finish` of `path@check@src/commands/create.rs` prints `done.written` and never reads `done.deleted`. Reproduced by an
adversarial reviewer with `klarch init <dir> --vcs none`, exit 0.

### Why it matters

`design@check@init-keeps-existing-files` has `init` keep what it finds, and `design@check@owned-namespace-check`
makes the deletion legitimate only as an act of the install a session can read. A silent loss with no copy in git
is unrecoverable, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

`finish` lists each deletion, or `init` judges such a file as a destination it refuses before its first write, with
a test for the listing or the refusal.
