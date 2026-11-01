---
kind: todo
---
# No published crate offers the history tasks the workflow asks of a session

## Summary

`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` names three tasks of rewriting a branch's
history as tasks a published crate of knowledge-architect does: a conflict-free rebase onto a moved
main branch with the check that every commit carried over, applying a message the session wrote to a
commit that is not the newest, and folding a repair into the commit it repairs with the check that no
content was lost. No published crate offers any of them. Sessions run them by hand, as this
repository's root `CLAUDE.md`, section `## Git`, describes, with `git filter-branch`, `git rebase -i
--autosquash` and `git range-diff`.

## Details

### What

A command of a published crate, or a library function that the project's maintenance binary calls,
for each of the three tasks. `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`
records the same tasks as missing commands of this repository's own tool.

### Why it matters

Each session builds the procedure again, and a script whose pattern misses a wrapped line leaves a
message unchanged with an empty diff, which only the `range-diff` check the root `CLAUDE.md` asks
catches. The goal is unmet while the tasks have no command.

### What would close it

The three tasks ship in a published crate, with tests over a scratch repository, and the installed
branch-work skill names them.
