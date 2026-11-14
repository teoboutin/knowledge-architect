---
kind: defect
---
# `transcript list --branch` lists a session that did no work on the branch

## Summary

`list --branch` counts a session on every branch its lines' `gitBranch` field carries. A session
that shares the main checkout with the next one carries, in its last lines, the branch the next
session checked out, and is listed on it though it did no work there. Three transcript reviewers
had to read such a session to rule it out. The fix changes what
`design@transcript@transcript-query-verbs` says `--branch` computes.

## Details

### What

- `design@transcript@transcript-query-verbs`: "`list --branch` computes the sessions that worked on
  a branch from the `gitBranch` field every chain line carries". The reader keeps each distinct
  value of that field as a session's branches, in `Corpus::cut` of `path@transcript@src/reader.rs`.
- Reproduction, at the intake of 2026-11-14 on the owner's machine:
  `cargo klarch transcript list --branch planned-splits` lists `80036d81` (2026-11-13 14:17 to
  21:05, branches `main, plan-directories, plan-directories-new, planned-splits`) beside `3e0588d2`,
  the session that worked there, which starts at 21:04.
- Measured by a `python3 -I` script over `80036d81`'s file, 1,282 lines: the lines carrying
  `gitBranch` `planned-splits` are 6, the first at line 1271 at 21:04:21, four `assistant` and two
  `user` lines; line 1270 carries `main` at 21:04:19. The six are the session's own `thinking`, two
  `Write` tool calls, their two results and its last `text`, all with `cwd` the main checkout,
  written after the other session moved the checkout the two shared. So the field follows the
  branch the checkout held when each line was written.

Instances, from this repository's workflow files:

- slice 1 of the plan directories, 2026-11-13, C2: the previous session's last lines, a
  retrospective after its own merge, ran after the checkout had moved;
- the slice on the conformance check before the harvest, 2026-11-14, C1(a), and the slice of the
  planned splits, 2026-11-14, C2: lines written into the old session's file at the new session's
  start.

The files' proposals: count a session on a branch from the first line whose checkout is that branch
and that its own tools wrote; derive a session's branches from the owner's or the agent's lines, or
mark a branch seen only in the harness's lines; list a session only where one of its tool calls ran
on the branch's checkout. Ruled out by the measurement: each still lists `80036d81`, since its own
agent lines and tool calls carry the branch; the session wrote outside the tree while the shared
checkout had moved. What tells the two sessions apart is what the session did on the branch, not
which lines carry its name.

The converse, a session missed, is
`issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree`; one reading of the field
decides both, and a rule that narrows `--branch` can widen or narrow what that case needs.

### Why it matters

The transcript review finds a branch's sessions by the listing, per
`design@agent-skills@transcript-found-by-listing`; a false listing sends the reviewer to read a
session that did no work there, and a reviewer that does not read it may take its lines as the
branch's.

### What would close it

The design discussion of `issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree`,
under `skill@knowledge-architect-design`, rewriting what `design@transcript@transcript-query-verbs`
says `--branch` computes, on a measurement of which lines carry which branch in sessions whose
checkout moved; then the rule built, with a test over a fixture of a session whose last lines carry
the next branch, which `list --branch` of that branch does not list.
