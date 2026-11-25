---
kind: observation
---
# `commits` passes a message with no blank line after its subject

## Summary

A commit message whose body follows its subject with no blank line passes `klarch commits`. git takes the
whole first paragraph as the subject, so `git log --format=%s` joins its lines, and so does every text that
names the commit by its subject. Whether the checker should report it is not established.

## Details

### What

Run at this entry's opening, over a project of `klarch init`: a commit whose message is the two lines
"a subject" and "its body with no blank line." passes `klarch commits HEAD~1..HEAD`, exit 0, and
`git log -1 --format='[%s] [%b]'` prints `[a subject its body with no blank line.] []`. The references of
such a message are checked all the same: a reference to no entry on its second line is a finding at line 2.

It reproduces. What is read of a message is `path@check@src/commands/history.rs`, per
`design@check@a-commit-message-is-a-document`, which says that a message is parsed as one markdown
document, with "its subject line, its blank line and its body".

### Why it matters

This project names a commit by its subject in its documents, per
`design@knowledge-architect@no-branch-sha-is-cited`, and a subject of several lines cannot be named by its
first line. A document that does so finds nothing in `git log --format=%s`.

### What would close it

A decision on whether `commits` reports it, with its argument, such as that naming a commit by its subject
is asked of every target project, and the check; or the reason that a message's shape is no concern of the
checker.
