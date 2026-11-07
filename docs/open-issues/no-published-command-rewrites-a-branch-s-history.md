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

A command of a published crate for each of the three tasks, per `goal@cli@one-program-for-its-tools`,
which closes the choice of a library function alone. `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`
records the same tasks as missing commands of this repository's own tool.

### Why it matters

Each session builds the procedure again, and a script whose pattern misses a wrapped line leaves a
message unchanged with an empty diff, which only the `range-diff` check the root `CLAUDE.md` asks
catches. The goal is unmet while the tasks have no command.

### What would close it

The three tasks ship in a published crate, with tests over a scratch repository, and the installed
branch-work skill names them.

### Leads from the design of the Component split

These are leads for this work's own design session:

- **A Component of its own, `rewrite`**, a published crate in crates/rewrite, created by this work,
  after the milestone has landed. It depends on the milestone's `git` Component alone: the
  rewriting commands share one purpose and one design, so they are one crate, one module per
  command. Its commands are subcommands of `klarch`, their arguments parsed in the milestone's `cli`
  Component, with a typed API in the crate.
- **The source is a port of the owner's Python tool, `git-history-tools`**, kept in a checkout
  beside this repository. Its five commands are `list-large-paths`, `prune-dead-paths`,
  `squash-scattered`, `move-commits` and `split-by-path`; each reports by default and rewrites only
  with `--apply`. Its foundation is a byte-safe model of a git invocation (its module common/gitcmd.py, under src/git_history_tools/ of its checkout,
  which pins `gc.auto=0` on every command that writes an object or moves a ref) and a reader of
  tracked files and commit contents; its module common/rebase.py drives `git rebase --interactive` with
  a scripted todo.
- **The port needs a write side in the `git` Component**: object writes and ref moves, beside the
  readers the milestone moves there. The milestone's premortem names the cause to watch: a write
  may need an environment that the git heads say every invocation drops or keeps, such as a
  temporary `GIT_INDEX_FILE`.
- **The three tasks above are taken in already**, by `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.
  The port's other commands are a generic part whose taking in is the owner's word, at the port,
  per `goal@knowledge-architect@the-owner-decides`.
- Every command it ships is bound by `design@knowledge-architect@no-operation-loses-content`.
- **Whether it belongs in `klarch` at all is for this design session to read.** The owner leans to putting it there, since that makes
  shipping easier, and doubts it, since it is somewhat off the subject of a binary focused on documentation.
  `goal@cli@one-program-for-its-tools` places it there, so a different answer changes that goal, on the owner's word. The owner would not
  want it to run a project's gates, and holds that such a feature would be a tool of its own; running a command its caller names on the
  invocation is admitted. That needs a place in the design, since `goal@git@a-pure-wrapper-over-git` models no such spawn.
