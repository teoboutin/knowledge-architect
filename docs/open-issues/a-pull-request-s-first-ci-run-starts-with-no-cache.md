---
kind: todo
---
# A pull request's first CI run starts with no build cache, since nothing runs on main to fill one

## Summary

The CI workflow caches builds with the action `Swatinem/rust-cache`, and runs on pull requests alone. GitHub
gives a pull request the caches of its own branch and of the default branch, and no run on main fills one, so
the first run of each pull request builds everything.

## Details

### What

Read in the log of run 45581213477, a pull request's run of 2026-11-25: the cache step prints
"No cache found." The workflow is `path@knowledge-architect@.github/workflows/ci.yml`, whose one trigger is
`pull_request`. That a cache is read from the pull request's branch or the default branch alone is GitHub's
documentation as remembered, not read again. With the matrix of three systems, three caches are missing in
place of one.

### Why it matters

The first run of a pull request, often the only one, waits for a cold build on the slowest system. The cost
is a wait, and nothing fails.

### What would close it

A run on main after a merge that fills the caches, with the argument that its cost and its trigger are right
against the comment of the workflow on the `push` trigger, or a decision that the wait is accepted.
