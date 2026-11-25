---
kind: todo
---
# The worktrees and scratch directories of a review's subagents are made and removed by hand

## Summary

`skill@knowledge-architect-review@review-invariants` gives each subagent of a review its own
detached worktree and its own scratch directory. Every path follows from the list of axes and the
commit under review, and no command makes or removes them: the session writes a shell loop for it
at every review before a merge.

## Details

### What

What the invariant asks, in `path@agent-skills@content/skills/review/SKILL.md`: a worktree "at a
path outside the project, or under a path its ignore rules cover", a scratch directory "distinct for
each subagent dispatched together", and, in a Rust project, `CARGO_TARGET_DIR` set by the subagent
to a directory inside its worktree that the ignore rules cover. Neither `klarch --help` nor the
commands of this repository's tool, `gates`, `changelog`, `skill` and `mutate`, holds a command that
makes them.

Instances, from retrospective files of this repository's sessions:

- the session that closed seven transcript issues, 2026-10-31: shell loops created and removed seven
  worktrees and seven scratch directories, then one more of each for the transcript axis;
- the slice that moved the commands out of the checker, 2026-11-06: twelve reviewers built under
  /tmp and met a per-user quota, "Disk quota exceeded"; two mutations read `caught` on it, and one
  test run could not finish;
- the slice that holds the fixtures and the integrated tests, 2026-11-07: eleven reviewers each
  built into their own target directory, 0.7 to 2.3 GB each, on a shared tmpfs that hit its quota;
  three reviewers met build failures.
- three sessions of 2026-11-11 and 2026-11-12, a goals slice, a new crate's placeholder and the
  retrospective's standing questions: about twenty reviewers' worktrees and scratch directories
  made by hand in the first; by hand in the second; by shell loops for 7, then 3, then 1
  reviewers, each removed after, in the third.
- the slice on the dispatch judgement, 2026-11-12: of 15 reviewers sent at the plan and at the
  merge, the first reported the refusal of `design@check@a-foreign-build-is-refused`, met with the
  release binary built in the main checkout; the session's later briefs told each reviewer to build
  in its worktree, a cold release build each; two reviewers ran no checker. The invariant's
  sentence on `CARGO_TARGET_DIR` is the subagent's act, read by the dispatcher, so a brief that
  omits it leaves the reviewer without it.
- the branch that closed the mutation survivors of three suites, 2026-11-16: eleven reviewers
  dispatched together, each building in its own worktree at cargo's default job count, and the
  machine ran out of memory and restarted, which stopped all eleven. The owner reported other
  processes of the owner's own running at the same time. The machine has 30 GB of memory and 32
  cores, read with `free -g` and `nproc`, so eleven builds can each start 32 compiler jobs. After
  the restart, with the owner's other processes stopped, each reviewer was told to set
  `CARGO_BUILD_JOBS=3`, and the eleven ran to the end. Nothing tells a dispatcher how many builds a
  batch may run at once.

The cost per reviewer is one full build, since a worktree with a target directory of its own builds
from empty. The two disk failures have two causes: the location the session chose, /tmp, which the
owner's user-level instructions exclude, and the directories the test helpers left there, which a
test no longer leaves, per `design@knowledge-architect@scratch-removed-by-a-guard`. A shared build
directory is ruled out by the invariant's own reason: here every target is tied to its checkout, per
`design@knowledge-architect@a-build-is-tied-to-its-checkout`, so a shared directory costs a rebuild
at each switch between checkouts and a wait on the build lock; an untied project would run another
checkout's build, which `design@check@a-foreign-build-is-refused` detects.

### Why it matters

The task follows from the axes and the commit with no judgement, the kind of task
`goal@tasks-a-tool-does-better-are-the-tool-s` gives a command once the owner takes its
generic part in, and every review before a merge runs it again. `design@agent-skills@a-scratch-directory-per-subagent`
records why each subagent gets its own directory.

### What would close it

The owner takes the task in or declines it. If taken in: a command that makes, for a commit and a
list of names, one detached worktree and one scratch directory per name, prints the paths for the
briefs, can report the disk and bound the build jobs a batch of builds needs, and removes them all; with a test over a
scratch repository; and the review skill naming it. If declined, this entry is deleted with that
reason in the commit.
