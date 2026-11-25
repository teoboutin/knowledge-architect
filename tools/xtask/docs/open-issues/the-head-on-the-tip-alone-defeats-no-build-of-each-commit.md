---
kind: design
---
# The head on the branch's tip alone names no build of each commit as its rival, and ranks no cost

## Summary

`design@xtask@ci-builds-the-tip-alone` has CI build and test the last commit of a branch, and no commit
before it. Its one fact, the time the test suite takes, defeats testing each commit. The nearest rival, a
build of each commit with no test, is named nowhere in the head and defeated by nothing in it. The head
accepts a cost, and no goal and no ranking stands behind the acceptance.

## Details

### What

The instance is the one head, found by the design-record audit that read it for the port of the gates
to Windows and macOS, which changed its command alone. Two failures of
`skill@knowledge-architect-decision-recording@head-shape`:

- **The nearest rival.** "nothing compiles or tests a commit before the tip: the suite takes minutes per
  run" argues against running the suite on each commit of `origin/main..HEAD`. A build of each commit,
  such as `cargo check --workspace --all-targets` with no test run, does not run the suite, so the fact
  does not reach it. The rival is on record: the tripwire that guarded the head named it as the candidate
  if the head reopened, "a build-only pass over each commit of the range origin/main..HEAD", and left as
  condition 3 of the tripwire standard excludes a clause that counts what the decision endorses, in the
  commit "[audit] Pass 1 of the tripwire axis applied: every tripwire kept has a horizon, and 20 leave".
  The cost of such a pass is not established: no run of it is on record.
- **The ranking.** "A commit before the tip that does not compile passes CI, and is found only by whoever
  builds it later" is a cost accepted against CI's time. No goal of `register@goal` ranks the two, and the
  head attributes no ranking. The commit that wrote it, "Restructure the imported checker into the
  knowledge-architect workspace", argues nothing of it in its message, and no statement of the owner on it
  was found in the history of the head, `git log -L` over it.

The head's own statements hold: the `gates` step of each job of the probe's run 45637540683 took from 4 to
12 minutes, by the head's command.

The suspected mechanism, in one sentence: the head was written with the workspace, before a head owed its
nearest rival and the weak form of a fork's choice, and no change since has argued it again.

The re-entry point: the next change to what CI runs over the commits of a branch, such as a command run on
each commit or a move of the `commits` gate, or a run of the design-record axis over
`register@xtask@design`.

### Why it matters

A session that meets a commit of main that does not build, or proposes a pass over each commit, finds in
`design@xtask@ci-builds-the-tip-alone` a fact that does not answer it, and can neither keep the decision by
its argument nor tell whether the cost was ranked.

### What would close it

A design discussion that names the build of each commit in the head and either defeats it on a fact, such
as the measured time of the pass over the commits of a few branches, or records the choice in the weak form,
with the costs weighed and whose ranking it is; or that takes the pass, and reverses the head.
