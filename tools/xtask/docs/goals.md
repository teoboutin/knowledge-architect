# Goals — xtask

xtask is the maintenance tool of this repository: it runs, as commands, the mechanical tasks that
this repository's own instructions require. It serves this repository, not another target project.

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

It is met while the command's list holds each of these gates, and wherever the verdict on whether
the gates let a branch merge is taken, it is this one command.

- Requires, for instance: the check that decides whether a branch may merge takes the verdict on
  whether the gates let it merge from this command run with the rebase check, not from a list of
  gates of its own.
- Admits, for instance: a session running one gate by hand where that is cheaper.
- Rules out, for instance: a verdict on whether the gates let a branch merge taken from another list
  of gates than the command's.

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

It is met while each such task that this repository's own instructions require, or whose procedure
they fix, has a command of the tool. Such a
task left to be done by hand shows the goal unmet in practice.

- Requires, for instance: the check, at each reword, that the reword lost no content is a command
  of the tool.
- Admits, for instance: a command of the tool for a task done once.
- Rules out, for instance: the checks of the merge predicate, which each merge repeats, left to a
  session to type one by one.
