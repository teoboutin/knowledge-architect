---
kind: defect
---
# Resolving a rebase conflict as the git workflow says drops every line of the commit's message that opens with `#`

## Summary

`git rebase --continue` run with `GIT_EDITOR=true`, the way a session finishes a rebase it cannot
edit interactively, applies git's default cleanup to the message of the commit whose conflict it
resolved, and that cleanup removes every line opening with `#`. A message using `## ` lines as
section headings loses them, and the checks the root CLAUDE.md prescribes after a history edit do
not see it.

## Details

### What

On the branch that made the directory register declarable, the rebase onto `origin/main` stopped
on a conflict in CHANGELOG.md and its three copies. The session resolved them, staged the tree and
ran `GIT_EDITOR=true git rebase --continue`. The rebased commit's message had lost its two lines
`## Deliberation (in-change path)` and `## Tests and their mutation checks`, and the blank line
after each, every other line kept; `git range-diff <old-base>..<old-head> origin/main..HEAD` showed the two lines as removed under
`@@ Commit message`. The session restored the message with
`git commit --amend --cleanup=verbatim -F <the old commit's message>`.

The same branch's earlier fold, `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash --keep-base
origin/main`, kept both lines: no conflict stopped it, so no message went through an editor.

Assumption, not established by a reproduction of its own: the cause is git's default
`commit.cleanup` mode, `strip` when the message passes through an editor, which removes lines
opening with the comment character `#`; a `--cleanup=verbatim` commit, or `core.commentChar` set
to another character, would keep them.

### Why it matters

A commit message is a document of the project, per `design@core@a-commit-message-is-a-document`,
and on the in-change path of the design skill it is the only home of the deliberation. The rule of
`design@knowledge-architect@no-operation-loses-content` bounds a history edit by the check that no
content was lost, and the root CLAUDE.md states that check, in `instructions@git-workflow`, as `git diff <old-head> HEAD` for a
rewrite and `git range-diff` for a rebase onto a moved base. The diff compares trees and sees no
message; the range-diff shows a changed message only to a reader who reads it, and a session
reading for its resolved conflict passes over it. `cargo klarch commits` judges the shortened
message and finds nothing wrong with it.

### What would close it

The rebase procedure of `instructions@git-workflow`, its point 3, finishes a resolved conflict
with a form measured to keep a message whole, such as
`git -c core.commentChar=';' rebase --continue` or an amend with `--cleanup=verbatim` from the old
commit's message, and its verification states that every commit's message is unchanged in the
range-diff. A reproduction, a commit whose message holds a `## ` line rebased over a conflicting
change, shows the form keeping the line.
