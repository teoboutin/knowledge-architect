---
kind: deferred
---
# How parallel work is partitioned among agents, who commits and how the parts are merged, is not designed

## Summary

`design@agent-skills@work-divided-by-judgement` lets a session split independent pieces of the work
among agents and leaves the partition to each brief. How a plan document partitions work among
agents dispatched in parallel, in a shared tree or in a worktree each, who commits, and how the
parts are merged, is decided by no text. The owner suggested that the question could be left to design and
planning, in a later discussion.

## Details

### What

The owner's report: in another project of the owner's, the code modules were split into smaller
units across the whole project by agents dispatched in parallel, one per Component, which made no
commits; the main session committed each part after the agents returned. The owner's motive was
to save time as well as context. The shape the owner named for a later design: parts carried in worktrees of
their own, committed there, then rebase-merged. The owner judged that such procedures are decisions
about how the work is carried, hard to make while carrying a task, so they could be left to the
design and planning steps.

The facts a design starts from:

- A fresh worktree carries no `target/` directory, measured in
  `rejected@mutate@worktree-holds-the-mutation`, so the first build in it is a cold build.
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
- A partition by file sends back every item whose fix crosses it: in the branch that repaired the
  checker's finding texts, three waves of implementers in worktrees each held a disjoint set of
  files, and five items needed a file another agent owned; each came back with its evidence and was
  carried by a later agent, and the main session assembled each wave by cherry-picks (the standing
  answers of 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow).
- Four more sessions of this repository combined parallel agents' branches by cherry-pick, each
  procedure written for its session: the intake of forty-seven files, 2026-11-14, four appliers in
  worktrees of their own, stacked in a fixed order, one cherry-pick conflicting in a generated
  `index.md`; the branch on the documentation gate, 2026-11-16, commits copied with
  `git cherry-pick -x`, whose trailer cites the source commit, a SHA no merge keeps, removed by a
  history edit before any check ran; the branch on mutation survivors, 2026-11-16, repairs split
  over three agents by area, then cherry-picked in order with a per-file check that each agent's
  files landed as written; the branch on the record search's workflow, 2026-11-16, three
  implementers on sibling branches, combined with no conflict and every gate passing (the
  standing answers of 2026-11-15-knowledge-architect-intake-forty-seven-files-klarch-workflow,
  2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow and
  2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow, and P2 of
  2026-11-16-knowledge-architect-record-search-workflow). The doc-gate branch and the record-search
  branch each worked from a spec whose work dispatched agents in parallel, which this entry's
  trigger names, and neither retrospective file reports a partition designed by its discussion.

Two entries partition work among agents too:
`issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session` and
`issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`.

### Why it matters

A session that partitions work among parallel agents with no design either serialises it, losing
the time the owner's report shows saved, or lets agents edit one tree and risks the content loss
above, against `goal@session-orchestrates-the-work`.

### Trigger

A plan document whose work wants agents dispatched in parallel: its design discussion designs the
partition, who commits and how the parts are merged, from the facts this entry gives, and records it
as a design head of agent-skills, which closes this entry.
