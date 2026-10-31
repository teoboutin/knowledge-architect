# Goals — xtask

What this tool is for. A goal is met or unmet, where a decision about how the tool is built is won
or lost and lives in `register@xtask@design`. A goal stays here while it is met, and leaves only
when the owner abandons it. This tool serves the project rather than its consumers, so its goals
need not refine a goal of `register@knowledge-architect@goal`.

## Every gate the project owes before a merge runs from one command `##one-command-runs-every-gate`

One command runs every gate of the project's list, the rebase check only when asked. The gates
include:

- formatting;
- the document check;
- the commit messages;
- the linter;
- the tests;
- the rebase check.

By default it runs every gate of the list but the rebase check, all of them when one fails. It gives
one verdict as its exit code, on the working tree and the branch's commits, which says whether the
gates let the branch merge.

It is met while the command's list holds each of these gates, and the continuous integration runs
the same command.

## A mechanical task this repository's own instructions ask repeatedly is a command of the tool `##repeated-tasks-are-automated`

A task becomes a command of the tool, so it runs the same way each time, when it meets all of these:

- needs no judgement, and takes several steps;
- is local to this repository: one its own instructions ask, or the procedure they fix for one the
  installed text asks;
- comes at a recurring moment, such as each commit, each reword or each merge.

A task the installed text asks of every project that adopts the workflow, whose result follows from
the tree and the workflow's rules, with no procedure of this repository's own, is the published
checker's, per `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

It is met while each such task that this repository's root and scoped `CLAUDE.md` files and its
`klarch-` skills and agent definitions ask, or whose procedure they fix, has a command of the tool.
Such a task left to be done by hand shows the goal unmet in practice.
