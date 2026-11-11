---
kind: todo
---
# No command lists the entries two commits differ by

## Summary

The commit message of an audit pass holds its counts and the slugs it created, removed and
retitled, per `skill@knowledge-architect-project-audit@record-of-an-audit`. Those follow from the
pass's first and last trees, and no command reads them: the listings read the working tree only.
Sessions take them by grep, sort and diff between two commits.

## Details

### What

What the commands read today: `issues`, `tripwires` and `model` read the working tree;
`tripwires --help` offers `--guarding` alone; the only command that reads a given commit's tree is
`commits`, which judges and lists nothing, and `worklist --since <commit>` marks the entries whose
file differs between a commit and the work tree, listing none created, removed or retitled.

Instances, both from retrospective files of sessions of thaum, a project that installs the
workflow, on 2026-11-01:

- a pass of the design-record axis: the session listed every design slug at two commits by grep,
  sorted and diffed the lists, and counted the heads per file; its first sort ran under the wrong
  locale and gave a wrong list;
- a pass of the tripwire axis: the session listed the tripwires at two commits by script and
  printed one line per entry with its outcome. Its word count, a `wc -w` over every tripwires home,
  is no listing of entries, and stays the session's, as the audit skill names it.

Ruled out: an option `--at <commit>` on each listing, the shape the tripwire pass's file proposed.
The counts asked are a difference between two trees, which a script would still compute from two
such lists. `design@check@check-judges-no-given-commit` bars `check` from judging a given commit; a
listing judges nothing, so it is not barred. `issue@query@no-command-searches-the-entries-or-lists-an-audit-s-work`
records the lists an audit axis builds before its agents judge, not the counts after a pass.

### Why it matters

The counts and slug lists follow from two trees with no judgement, a task
`goal@tasks-a-tool-does-better-are-the-tool-s` gives the published checker, and a
script written for one pass gave a wrong list with no error.

### What would close it

A command, or an option of `issues`, `tripwires` and a listing of design heads, that prints per
register the entries created, removed and retitled between two commits, with a test over a mock
project of two commits.
