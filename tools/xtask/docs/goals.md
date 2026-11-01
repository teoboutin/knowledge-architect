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

By default it runs every gate of the list but the rebase check, all of them when one fails, and its
exit code says whether the gates it ran pass. Run with the rebase check, its exit code is the one
verdict, on the working tree and the branch's commits, which says whether the gates let the branch
merge.

It is met while the command's list holds each of these gates, and the continuous integration runs
the same command.

## The local part of a mechanical task this repository's own instructions require repeatedly is a command of the tool `##repeated-tasks-are-automated`

A task becomes a command of the tool, so it runs the same way each time, when it meets all of these:

- needs no judgement, and takes several steps;
- is local to this repository: one its own instructions require, or the procedure they fix for one
  the installed text requires;
- comes at a recurring moment, such as each commit, each reword or each merge.

A task of the set that `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` names, with no
procedure of this repository's own, is a published crate of knowledge-architect's. The part of a
task's procedure that knowledge-architect's owner has taken in as generic is not local once it is
shipped, per `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`; until then the
local command stays, and the rest of the task is the tool's where it meets these conditions.

It is met while each such task that this repository's root and scoped `CLAUDE.md` files and its
`klarch-` skills and agent definitions require, or whose procedure they fix, has a command of the
tool. Such a task left to be done by hand shows the goal unmet in practice.
