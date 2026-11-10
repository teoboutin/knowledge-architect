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

The cost per reviewer is one full build, since a worktree with a target directory of its own builds
from empty. The two disk failures have causes recorded elsewhere: the location the session chose,
/tmp, which the owner's user-level instructions now exclude, and the leak of
`issue@testing@a-test-run-leaves-directories-in-the-temporary-directory`. A shared build directory
is ruled out by the invariant's own reason and by `design@check@a-foreign-build-is-refused`.

### Why it matters

The task follows from the axes and the commit with no judgement, the kind of task
`goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` gives a command once the owner takes its
generic part in, and every review before a merge runs it again. `design@agent-skills@a-scratch-directory-per-subagent`
records why each subagent gets its own directory.

### What would close it

The owner takes the task in or declines it. If taken in: a command that makes, for a commit and a
list of names, one detached worktree and one scratch directory per name, prints the paths for the
briefs, can report the disk a batch of builds needs, and removes them all; with a test over a
scratch repository; and the review skill naming it. If declined, this entry is deleted with that
reason in the commit.
