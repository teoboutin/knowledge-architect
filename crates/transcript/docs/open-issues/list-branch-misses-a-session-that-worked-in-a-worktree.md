---
kind: defect
---
# `transcript list --branch` misses a session that worked on the branch from a linked worktree

## Summary

`list --branch <branch>` finds the sessions of which at least one chain line carries that branch in
its `gitBranch` field. A session that worked on a branch from a linked worktree can carry it in none
of its chain lines, each naming the branch the main checkout had or another one, and `--branch` then
does not list it. The review skill finds a branch's sessions with `--branch`, so the transcript
review can miss the session that did the work.

## Details

### What

`design@transcript@transcript-query-verbs`: "`list --branch` computes the sessions that worked on a
branch from the `gitBranch` field every chain line carries". The reader collects that field in
`path@transcript@src/reader.rs`, in `Corpus::cut`, which cuts a top-level file into sessions and
keeps each distinct `gitBranch` value of a session's chain lines as its branches. The filter,
`session_kept` in `path@transcript@src/command.rs`, keeps a session when any of those branches
equals the one asked for, so a session is missed only when none of its chain lines carries it.

The instance, reported by a retrospective of a session of thaum, a project that installs the
workflow, 2026-11-03: the whole branch was worked on from a worktree; the session's `gitBranch`
fields read the main checkout's branch and the branch of another piece of work;
`klarch transcript list --branch <branch>` printed its header alone and exited 1; the transcript
reviewer fell back to `--containing`.

Reproduction, `assumption`: the steps below are inferred from that report and from the counts that
follow, and were not run as such.

1. In a repository whose main checkout holds branch A, start a session there.
2. Add a linked worktree holding branch B, and do B's work from that worktree in the same session.
3. Run `klarch transcript list --branch B`: expected, the session listed; actual, the header alone,
   exit 1.

Measured at the intake of 2026-11-07, over the transcripts of the owner's machine, in the top-level
lines whose `cwd` lies in a linked worktree:

- in this repository, 22 lines in one worktree carry the branch the main checkout held;
- in a worktree of an agent in thaum, 47 lines carry `main` and 23 the worktree's branch.

The thaum session of the second count carries the worktree's branch in 23 lines, so `--branch` lists
it: it is no instance of the defect. What it shows is that one session's lines can carry several
branches, the worktree's among them in some and not in others, so whether the worktree's branch
appears in at least one chain line depends on which lines the harness writes with it. The field
names the branch of the session's start, not always the branch worked on. `assumption`: that reading
of the harness is an inference from these counts, not from its documentation; which harness events
write the worktree's branch is not established.

Ruled out: a change to the field's reading since the release of 0.9.0, as
`git log -S gitBranch v0.9.0..HEAD` returned nothing at the intake.
`design@transcript@transcript-directory-resolution` makes a worktree's sessions the project's, which
is a different question: the session is found under the project, and the branch filter drops it.

### Why it matters

The head's premise, that the field names the branch a session worked on, can fail for work in a
worktree, as in the thaum instance. The review skill, `path@agent-skills@content/skills/review/SKILL.md`, has the
dispatcher find the sessions of a branch with `--branch`, so the transcript review can miss the
session that did the work, against `goal@transcript@a-session-reads-past-sessions`.

### What would close it

A discussion under `skill@knowledge-architect-design`, since a fix changes what
`design@transcript@transcript-query-verbs` says `--branch` computes, that measures the field over
worktree sessions and either widens what `--branch` matches, for instance a line whose `cwd` is a
worktree that had the branch checked out, which git cannot tell once the worktree is removed, or has
the review skill name `--containing` for that case; with a test over a fixture of a worktree
session.
