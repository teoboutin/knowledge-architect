---
kind: question
---
# A conflict in a generated file, met when commits are replayed, is resolved by hand

## Summary

A commit that passes its checks regenerates the generated files its change touches, such as a
register's index. Replaying a series of such commits, by cherry-pick or rebase, conflicts in those
files at nearly every commit, and the content to take follows from the tree with no judgement. No
installed text and no command resolves it; this repository's root states the resolution for itself.

## Details

### What

- No installed text says how a conflict in a generated file is resolved: a search of
  `path@agent-skills@content/` for "conflict" beside "index" or "generated" returns nothing at the
  intake of 2026-11-14.
- This repository's root `CLAUDE.md`, `instructions@git-workflow` point 3: "A rebase that stops on
  a conflict in a generated `index.md` is resolved by regenerating it: `cargo klarch index`, then
  `git add` on the file."

The instance: a move-only split of oversized modules in thaum, on 0.9.0, 2026-11-11, C5 of its
workflow file. Fifteen commits, made on separate branches, each regenerated the rule index;
applying them in sequence conflicted in that file at nearly every commit. The session wrote a
script: take either side, run `cargo klarch index`, stage, continue. The conflicts are expected,
since under `design@agent-skills@move-lands-in-move-only-commits` each move commit passes its
checks, so each regenerates the index.

Bounds on a fix:

- the file's proposal, a `.gitattributes` entry and a merge driver shipped by
  `install-agent-skills`, is refused by `design@check@owned-namespace-check`: "The install edits
  nothing outside the namespace". A driver is also per-clone configuration, the reason
  `rejected@check@commit-msg-hook` records against a hook; and whether the rest of the tree the
  replayed commit will hold is in place when git runs a driver, which is given the three versions of
  one file, is not established;
- a regeneration is sound because a generated file is a function of the tree, per
  `design@check@generated-files-are-pure`, and any list of the files a command touches is the one
  list of `design@model@one-list-of-generated-files`.

The nearer shapes: a command that resolves a stopped replay whose only conflicts are in generated
files, by regenerating from the tree and staging; or one sentence of installed text where the
checker's model is taught, as this repository's root states for a rebase.

### Why it matters

The task follows from the tree with no judgement and recurs wherever a branch's commits are
replayed, which `goal@tasks-a-tool-does-better-are-the-tool-s` owes a tool once the owner takes it
in; meanwhile each session writes its own script, in a state, a stopped replay, where a wrong
script loses a resolution.

### What would close it

The owner's word on the question whether the task is taken in. Taken in: this entry becomes a
`todo`, closed by a design discussion that places the move, in the checker or in the installed
text, with a test that replays a commit regenerating an index over a mock project. Declined: this
entry is deleted with that reason in the commit.
