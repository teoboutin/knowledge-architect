---
kind: defect
---
# A test command that fails for its environment reads as `caught`

## Summary

`cargo x mutate run` takes any failure of the command after `--` as `caught`, so a failure of the environment, such as a full disk during the test run, reads as a mutation the test caught.

## Details

### What

Observed by two reviewers of the slice that moved the commands out of the checker, under a shared quota: two mutants read `caught` on `Disk quota exceeded` failures of unrelated tests, 59 and 4, and read `SURVIVED` when run again. `--build` guards a mutant that does not build, not a test run that fails for another reason.

### Why it matters

`design@mutate@report-not-gate` makes the verdict what a session records as evidence that a test discriminates, so a false `caught` is a false claim in a commit message.

### What would close it

The verdict names what failed, as the failing test's name or the command's last lines, so a reader can tell, with a test over a command that fails beside the test. The baseline's pass does not settle it, since the environment can change between them.
