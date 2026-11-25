---
kind: todo
---
# A CI run skipped on a draft pull request fills the run history and reads as passing

## Summary

Each push to a draft pull request starts a run of the CI workflow whose job is skipped. Most runs of
the workflow's history are such runs, and GitHub counts a job that `if:` skips as passing, so a run
that checked nothing can be taken for one that passed.

## Details

### What

`path@knowledge-architect@.github/workflows/ci.yml` triggers on `pull_request`, on `opened`,
`synchronize`, `reopened` and `ready_for_review`, and its job holds
`if: ${{ !github.event.pull_request.draft }}`, per `design@knowledge-architect@ci-skips-a-draft`.

Measured on 2026-11-25, `gh run list --workflow ci.yml --limit 60 --json conclusion`: 55 runs
concluded `skipped` and 5 `success`.

`design@knowledge-architect@the-merge-waits-for-a-successful-run-on-the-head` answers the confusion at
the merge, by reading the draft flag and asking for a successful run. The owner reported that the
skipped runs pollute the history and are sometimes taken for runs that passed, and ruled that the
question can wait.

A lead, read in GitHub's documentation and not run: no filter of the `pull_request` event reads the
draft state, and `types: [ready_for_review]` alone starts no run for a push to a draft, and none for a
push to a pull request that is ready. After such a push the pull request is put back to draft and
marked ready again, `gh pr ready --undo` then `gh pr ready`. Until then the context the ruleset
requires is pending on the new head, which blocks the merge, where a skipped job does not. The merge
predicate could then be simpler.

Ruled out, by the same documentation: a run started by `workflow_dispatch`. The checks of its jobs do
not satisfy a required status check of a ruleset, even where they pass on the head commit.

### Why it matters

A session or the owner who reads the history, or `gh pr checks`, can take a skipped run for a verdict.
It strains `design@knowledge-architect@ci-skips-a-draft` by its means, and
`design@knowledge-architect@the-merge-waits-for-a-successful-run-on-the-head` exists to answer it.

### What would close it

The trigger reduced, with the toggle written in `instructions@git-workflow` and the two heads brought to
it, and a pull request seen to start no run on a draft push, to start one when marked ready, and to be
blocked after a push while ready; or the owner's ruling that the skipped runs stay.
