---
kind: design
---
# A commit or a push chained after a failing check recurs, in compound commands that run it whatever the check found

## Summary

The installed primer asks that a failing staged check be repaired before the commit, and this
repository's root asks that a push follow a passing `commits` check. Sessions keep joining the
check and the commit or the push in one command whose separator runs the second whatever the check
found: seven instances in five sessions of this repository, one in thaum. Each is a lapse of a
clear instruction; the recurrence is the argument for moving where it is delivered.

## Details

### What

The instructions, as they stand:

- the commit, in the installed primer, `primer@checker-model`: `cargo klarch check --staged` "Run
  it after staging and before each commit, and repair a failing one before the commit"; and "Never
  chain a command on a verdict that went through a pipe". The primer states no rule on a semicolon
  after a bare check; "repair a failing one before the commit" covers that chain.
  `design@agent-skills@harness-shell-fails-on-a-pipe` delivers `pipefail` at every command, which
  stops the piped chain and not the semicolon, and says the primer's rule against the chain stays,
  for `;`. `design@agent-skills@staged-check-before-each-commit` is the primer's staged-check
  instruction;
- the push, in this repository's root, `instructions@git-workflow`: "A commit is pushed only after
  `cargo klarch commits` has passed on it: in a command of its own, or behind `&&` on the bare
  check. A push after `;`, or after a check whose output went through a pipe in a shell without
  `pipefail`, runs whatever the check found", whose home is
  `design@knowledge-architect@push-after-the-commits-check`.

The instances, as the retrospective workflow and project files of the intake of forty files report
them, each called a lapse by its file:

- the development milestone's plan, 2026-11-01: one command ran the `commits` check, then
  `echo`, `tail` and `git push`, joined with semicolons; the check failed, and the push went out;
- the development milestone's slices, 2026-11-02: two commits followed a failed staged
  check, once chained with `&&` on `tail` of its output, once after a semicolon;
- the slice that renamed `check`, 2026-11-04: one commit, in its standing answers;
- the slice that put `cli` on top, 2026-11-06: twice, a semicolon between the staged check, or a
  failing edit script, and `git commit`;
- the slice that moved the consumers out, 2026-11-06: one commit, in its standing answers;
- a session of thaum's design-record audit, 2026-11-01, in its standing answers.

That is one push and six commits in this repository, from 2026-11-01 to 2026-11-06, and one in
thaum. A further lapse of the push rule, of another shape, in the slice that moved the consumers
out: after a `git rebase --onto` that detached HEAD, the session ran `commits` on the detached
commits and pushed the branch's name, which still held the commit that had failed.

No hook is installed in this repository's checkout (its hooks directory holds samples only, and
`git config --get core.hooksPath` exits 1), and the installed setup skill names none.

The candidates:

1. a pre-commit hook running `cargo klarch check --staged`, which the retrospective of the slice
   that put `cli` on top proposes, and itself calls weak: it stops the commit whatever the shell
   chained. Against it: the per-clone configuration that `rejected@check@commit-msg-hook` counted,
   "per-clone configuration every clone had to install and every session had to check", and, in
   this repository, a staged check that refuses a commit `commits` accepts, per
   `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`. It does not
   bring back that rejected design, which judged messages, and `check --staged` run from a
   pre-commit hook judges the tree the commit records, per `design@git@index-file-variable-kept`;
2. the root's sentence moved to the commit's moment in its `## Git` section, which the slices'
   project file names: it moves text the session reads at the start, as now;
3. no change: each instance was caught by the next command and cost one amend.

The suspected mechanism, in one sentence: the rule is read once at the session's start and needed
at each commit, inside a compound command written for speed, the same mechanism
`issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
records for the claim check, whose discussion weighs "a project's commit hook" among its deliveries.

Re-entry point: the design discussion of
`issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
since candidate 1, or a delivery that issue weighs, would serve both instructions at once; or the
owner's word.

Source: P1 of the analysis 2026-11-01-knowledge-architect-development-milestone-plan, in the commit
"[intake] The analyses of forty received retrospective files, three outcomes for the owner", with
the findings it groups as cluster H2. It is an entry of its own, not instances of the claim-check
issue, whose title names another instruction.

### Why it matters

A push that runs after a failing check publishes a commit the gate refused, and a commit after a
failing staged check records a tree with findings, each repaired by an amend or a history edit,
against `goal@knowledge-architect@agents-work-without-drift`. Under the lapse rule of
`skill@knowledge-architect-agent-configuration@admission-standard`, a lapse that recurs at one
point of delivery is repaired by moving where the instruction is delivered.

### What would close it

The discussion picks a delivery at the commit and at the push among the candidates above, or rules
that none is worth its cost; the commit rule's delivery is the installed text's or the setup's, and
the push rule's is this repository's root's. Then the retrospectives that follow show whether the
count falls.
