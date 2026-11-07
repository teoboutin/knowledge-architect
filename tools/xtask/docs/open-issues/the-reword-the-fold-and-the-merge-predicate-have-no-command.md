---
kind: todo
---
# The reword of an older commit, the fold of a repair and the merge predicate have no command of the tool

## Summary

`goal@xtask@repeated-tasks-are-automated` asks a command of the tool for each task with no
judgement, of several steps, that this repository's own instructions ask, or whose procedure they
fix, at a recurring moment. Three procedures of the root `CLAUDE.md`, section `## Git`, meet all of
these and have none: the reword of a commit that is not the newest, the fold of a repair into an
earlier commit, and the merge predicate. A session runs each by hand, step by step.

## Details

### What

`path@knowledge-architect@CLAUDE.md`, `instructions@git-workflow`:

- **The reword**, point 2: write a script that reads a message on stdin, run
  `git filter-branch -f --msg-filter '<script>' origin/main..HEAD` with a clean tree, check that
  `git diff <old-head> HEAD` is empty and that `git range-diff` marks with `!` each commit meant to
  change and no other, then delete the backup refs under `refs/original/`. The new wording is
  judgement; the steps around it are not.
- **The fold of a repair**, point 2: `git commit --fixup=<commit>`, then
  `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash --keep-base origin/main` with a clean tree,
  then check that `git diff <old-head> HEAD` is empty and that the branch holds one commit fewer.
  The installed text asks for the fold, per `primer@branch-commits`; this repository fixes its
  procedure.
- **The merge predicate**, point 5: fetch `origin/main`, check that it is an ancestor of HEAD, read
  `isDraft` and `headRefOid` with `gh pr view`, check with `gh run list` that a run on the head
  succeeded, then `gh pr merge --rebase --delete-branch` and `git pull --ff-only`. Its result
  depends on GitHub's state as well as the tree. Right after `gh pr ready`, the head carries a run
  skipped while it was a draft and a run in progress, so the predicate owes a wait: until no run on
  the head is in progress, then the `success` test. The procedure names no wait.

`path@xtask@src/` holds the commands `gates`, `changelog`, `skill` and `mutate`, and none of these.

Found by the whole-set review of this repository's goals on the branch that brought them to the
goal standard, whose reviewers reported it in every layout round.

A further instance: P2 of the retrospective of the session of the intake deciding by authority,
2026-10-31: four rewords of branch messages in one session, each with a new script and the two
checks.

Further instances, from the retrospective files received at the intake of 2026-11-07:

- the reword by hand, in ten sessions of this repository, 2026-10-31 to 2026-11-07: the intake of
  seventeen files, the narrowing of the goals, the development-and-components milestone, the
  revision of the rules on Components, the revision of the goal standard, the slice that created
  the `model` Component, the slice that renamed a Component, the slice that put the `cli`
  Component on top, the slice that moved the commands out of the checker, and the slice that holds
  the fixtures and the integrated tests. The development-and-components milestone ran the
  procedure five times in two slices, each with a new script. In the narrowing of the goals, two
  rewords ran by `git filter-branch --msg-filter` and a Python script, and the session's count of
  changed commits read `git range-diff` with the wrong pattern: it gave 0 where one commit had
  changed, the failure a command removes;
- the wait on CI, in three sessions: the closing of seven transcript issues, 2026-10-31, whose first
  wait loop stopped on the skipped run; the narrowing of the goals, 2026-11-01, which wrote a polling
  loop twice, the first stopping while a run was in progress; and the session that closed about
  twenty issues before a release, 2026-11-01.

### Why it matters

Each procedure is run by hand at each reword, each repair folded and each merge, and a step missed
loses a check: a reword whose pattern misses leaves the diff empty all the same, which only the
`range-diff` check shows. The goal is unmet on all three while they have no command.

### What would close it

A command of the tool for the merge predicate, and for the reword and the fold until a published
crate ships them, per `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`,
when the tool's command calls the published one and keeps only this repository's part; each in the
order of known work of `goal@knowledge-architect@the-owner-decides`. A command that runs the reword's or the
fold's checks without running the history edit itself also meets the goal for the checks.
