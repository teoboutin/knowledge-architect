---
kind: question
---
# `init` removes a file of the installer's namespace that no git holds a copy of, and only says so

## Summary

`init` in a directory that holds an agent file of the installer's namespace, under the name knowledge-architect-mine.md, removes it and
lists it as `fixed: removed`. Outside a worktree, or where git does not track it, no git holds a copy, so the line is all that is
left of it. The question is whether that is enough, on the owner's ruling.

## Details

### What

Measured with `klarch init <dir> --name p --vcs none` and `--vcs git`, each exit 0, each listing the line. Listing it is what `finish` of
`path@create@src/command.rs` does outside a worktree, where it used to read `done.written` alone, and what `check --fix` does inside one. An
adversarial reviewer found the silent form.

### Why it matters

`design@create@init-keeps-existing-files` has `init` keep what it finds, and `design@check@owned-namespace-check`
makes the deletion legitimate as an act of the install a session can read. A loss with no copy in git is unrecoverable,
against `goal@knowledge-architect@agents-work-without-drift`, and bears on `design@create@init-runs-git-init` and
`design@create@init-asks-no-confirmation`.

### What would close it

The owner's ruling among: the listing as it stands; `init` judging such a file as a destination it refuses before its first write,
with a test; or `rejected@create@init-refused-outside-a-repository-root`, in its form that refuses outside every worktree whatever
the directory holds.
