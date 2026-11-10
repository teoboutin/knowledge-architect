---
kind: todo
---
# `init` and `new-component` remove a file the project wrote and list it, where the root's decision on a tool's writes has them refuse

## Summary

`init` in a directory that holds an agent file of the installer's namespace, under the name knowledge-architect-mine.md, removes it and
lists it as `fixed: removed`. `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` has the run refuse before it writes. The code is built as it stands only so as not to widen a slice that moved this code.

## Details

### What

Measured with `klarch init <dir> --name p --vcs none` and `--vcs git`, each exit 0, each listing the line, with no copy in git. `finish` of `path@create@src/command.rs` lists it outside a worktree, and `check --fix` inside one, the path `new-component` also ends on, since it ends as `init` ends, per `design@create@component-command`, so `new-component` removes such a file the same way.

The reading this entry takes of what the project wrote: a file at a path this version's installed text is installed to is rewritten, even where edited by hand; any other file the install would remove counts as the project's, one an older version installed included. Refused so, a project with no manifest that holds such a leftover meets a refusal, which names it.

`init --vcs none` also writes no index, since the walk needs git, so it does not reach the state `design@create@init-command` gives a run inside a worktree, with every index written.

### Why it matters

`design@create@init-asks-no-confirmation` argues from "overwrites no hand-written file outside the installer's namespace", and `design@create@init-keeps-existing-files` says "the install writes and deletes there". Each conflicts with `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` until this work rewrites it, and no goal ranks the three heads, with `design@check@owned-namespace-check` read for what an install does outside these two commands.

### What would close it

`init` and `new-component` refuse such a file before their first write, with a test of each, the listing leaves `finish`, and the two heads are rewritten under `skill@knowledge-architect-decision-recording`. A test also shows that `new-component` keeps a document already there: in `path@cli@tests/init.rs`, each assertion of a `kept` line is in a test that runs `init`.
