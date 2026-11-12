---
kind: deferred
---
# How parallel work is partitioned among agents, who commits and how the parts are merged, is not designed

## Summary

`design@agent-skills@work-divided-by-judgement` lets a session split independent pieces of the work
among agents and leaves the partition to each brief. How a plan document partitions work among
agents dispatched in parallel, in a shared tree or in a worktree each, who commits, and how the
parts are merged, is decided by no text. The owner placed the question at design and planning, in a
later discussion.

## Details

### What

The owner's report: in another project of the owner's, the code modules were split into smaller
units across the whole project by agents dispatched in parallel, one per Component, which made no
commits; the main session committed each part after the agents returned. The owner's motive was
time more than context. The shape the owner named for a later design: parts carried in worktrees of
their own, committed there, then rebase-merged. The owner judged that such procedures are decisions
about how the work is carried, hard to make while carrying a task, so they belong to the design and
planning steps.

The facts a design starts from:

- A fresh worktree costs a cold build, measured in `rejected@mutate@worktree-holds-the-mutation`.
- Parallel target directories filled a shared tmpfs, per
  `issue@mutate@an-environment-failure-reads-as-caught`.
- Agents sharing one tree are safe only while their parts touch disjoint files: an amend, a rebase
  or a restore by one loses another's uncommitted content, which
  `design@knowledge-architect@no-operation-loses-content` forbids. A worktree per agent removes the
  condition, at the cost of a merge.
- A worktree that holds an agent's durable part of the work is not a scratch worktree, so
  `design@knowledge-architect@scratch-worktree-outside-git-rules` does not exempt it from the rules
  of git.
- `skill@knowledge-architect-review@review-invariants` gives a worktree to a subagent that judges a
  commit and runs the checker, a test, a mutation or a built program, and to no other.
- `design@agent-skills@step-is-no-commit` gives the number of commits to "the implementing
  session", which is ambiguous under a partition among agents.
- A staged check compares the installed files with the shipped text of the working tree, per
  `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, which an agent
  committing part of a shared tree meets.

Two entries partition work among agents too:
`issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session` and
`issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`.

### Why it matters

A session that partitions work among parallel agents with no design either serialises it, losing
the time the owner's report shows saved, or lets agents edit one tree and risks the content loss
above, against `goal@session-orchestrates-the-work`.

### Trigger

A plan document whose work wants agents dispatched in parallel: its design discussion decides the
partition, and this entry gives it the facts.
