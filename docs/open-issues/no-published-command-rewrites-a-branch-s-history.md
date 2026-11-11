---
kind: todo
---
# No published crate offers the history tasks the workflow asks of a session

## Summary

The workflow asks three tasks of rewriting a branch's history of a session: a conflict-free rebase
onto a moved main branch with the check that every commit carried over, applying a message the
session wrote to a commit that is not the newest, and folding a repair into the commit it repairs
with the check that no content was lost. The owner counts the three as taken in, so
`goal@tasks-a-tool-does-better-are-the-tool-s` owes each a tool that
knowledge-architect provides. No published crate offers any of them. Sessions run them by hand, as this
repository's root `CLAUDE.md`, section `## Git`, describes, with `git filter-branch`, `git rebase -i
--autosquash` and `git range-diff`.

## Details

### What

A command of a published crate for each of the three tasks. Whether a library function alone
would do is part of the rule that every tool is a command of `klarch`, which no head argues, per
`issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`: the session's default, which the owner approved, was a command. `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`
records the same tasks as missing commands of this repository's own tool.

Instances in a project that installs the workflow, from the retrospective files received at the
intake of 2026-11-07: three sessions of thaum, 2026-11-01 and 2026-11-02, a pass of the
design-record axis and two milestone slices, edited a branch's history with no recipe. The harness
refuses `git rebase -i`, an interactive command, and the installed text names no recipe: the review
skill asks a repair "by a history edit of the branch after that". One session replayed the branch by
cherry-pick, and its first attempt amended the wrong commit. A command meets that refusal: it drives
the rebase with no editor, as this repository's fold does with `GIT_SEQUENCE_EDITOR=true`.

A further task, not taken in: moving a commit to the branch's front, as a session of this
repository did once, 2026-11-01, when a stricter check landed mid-branch; its scripted sequence
editor stopped twice on a generated index and left a later commit's index stale, so the task
includes regenerating the generated files at each replayed commit. `primer@branch-commits` offers
the squash to one commit as the existing move. Whether a command does it is the owner's word, per
`goal@tasks-a-tool-does-better-are-the-tool-s`, as for the port's other commands below.

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
- **The three tasks above are taken in already**, on the owner's word, so
  `goal@tasks-a-tool-does-better-are-the-tool-s` owes each a tool.
  The port's other commands are a generic part whose taking in is the owner's word, at the port,
  per `goal@the-owner-decides`.
- Every command it ships is bound by `design@knowledge-architect@no-operation-loses-content`.
- **Whether it belongs in `klarch` at all is for this design session to read.** The owner leans to putting it there, since that makes
  shipping easier, and doubts it, since it is somewhat off the subject of a binary focused on documentation.
  The rule that every tool is a command of `klarch` places it there; no head argues that rule, per
  `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`, so this design session argues it with the placement. The owner would not
  want it to run a project's gates, and holds that such a feature would be a tool of its own; running a command its caller names on the
  invocation is admitted, per `design@knowledge-architect@mutation-command-host`. That needs a place in the design, since an operation of the `git` library is stated in git's own
  terms, per `design@git@an-operation-in-git-s-terms-is-the-library-s`, and a spawn of a caller's command is none.
