---
kind: question
---
# CI does not run on a draft pull request, and no record says why

## Summary

`design@knowledge-architect@git-flow` states that CI does not run on a draft, and gives no reason.
A branch's new tests then run in CI for the first time after its whole review, and a failure only
CI's machine shows is fixed by a commit no review covered. Should CI run on a draft?

## Details

### What

- `design@knowledge-architect@git-flow`: "Once it holds a first commit, it is pushed and a draft
  pull request is opened. CI does not run on a draft." The CI workflow, ci.yml under
  .github/workflows, runs its job only where the pull request is not a draft; its comment explains
  why a skipped job counts as passing, and not why drafts are skipped. Both the condition and the
  head's sentence come from the commit that imported the checker into this workspace, whose message
  says nothing of drafts.
- The instance: the branch that built `klarch init` and `klarch new-component`. A test relying on
  git refusing a repository as another user's passed on the workstation and failed in CI, which
  first ran after the review before the merge and three follow-up reviews; the failure reproduced
  locally under a global git configuration holding `safe.directory = *`. The fix, "The test of a
  repository git refuses as another user's keeps the machine's git configuration out", landed with
  the gates alone. Reported in that session's retrospective, analysed at the intake of 2026-10-24.
- The cost a draft run adds: `design@xtask@ci-builds-the-tip-alone` gives the suite's time per run
  on the runner, re-taken with `gh run view <id> --json jobs`.

### Why it matters

A failure found after the review lands unreviewed or costs another round, against
`goal@knowledge-architect@agents-get-a-complete-workflow`. Running CI on drafts would change
`design@knowledge-architect@git-flow`, `design@xtask@gates-list-primary-home`, the CI file, and
points 1, 4 and 5 of `instructions@git-workflow`.

### What would close it

The owner weighs a CI run per push of a draft against a late discovery, and the head records the
reason either way; or the draft rule is kept with the reason found, if the history of the imported
source holds one.
