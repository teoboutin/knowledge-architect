# 2026-11-12-knowledge-architect-finding-repairs

- **Received file**: 2026-11-12-knowledge-architect-finding-repairs.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.9.0-387-gbd37bb85` (`bd37bb85`) when the work began, branch `finding-repairs`, tree clean."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@mutate@an-environment-failure-reads-as-caught`: touches, respect.
  - `issue@testing@a-test-run-leaves-directories-in-the-temporary-directory`: touches, respect.
  - `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
  - `design@agent-skills@intake-outcome-authority`: constrains, respect.
  - `rejected@agent-skills@owner-rules-each-intake-finding`: constrains, respect.
  - `design@agent-skills@derived-decision-proceeds-on-default`: constrains, respect.
  - `design@agent-skills@default-ruled-before-merge`: constrains, respect.
  - `issue@check@a-home-for-developer-contracts-outside-agent-configuration`: touches, respect.
  - `design@knowledge-architect@testing-crate`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.

## test-tempdir-rule-absent: no project text says whether the tests are bound by the owner's rule keeping working files out of `/tmp`

Members: 2026-11-12-knowledge-architect-finding-repairs P2.

- **a. What it says.** "The owner's user-level instructions keep a session's working files out of
  `/tmp`. The repository's integrated tests create their sandboxes with `std::env::temp_dir()`, which
  is `/tmp` on the owner's machine, and this branch's new test file followed that idiom. No project
  text says whether tests are bound by that rule." Judged needing nothing in that branch, and
  reported "so the owner can decide whether the testing Component states it".
- **b. Whether it still applies.** Yes. At origin/main, `grep -rn 'temp_dir()'` over the crates'
  `src/` and `tests/` and `path@xtask@src/` gives 59 sites: 32 in `testing`, 11 in `check`, 8 in
  `git`, 3 in `mutate`, 2 in `xtask`, and 1 each in `cli`, `model` and `query`, one of the `mutate`
  sites outside a test, `apply_to_scratch` in `path@mutate@src/run.rs`, which removes its directory.
  No project text names `TMPDIR` for a test run: the only `path@agent-config@settings.json` setting is
  `SHELLOPTS`. On this machine, `TMPDIR` is unset in a session, so `temp_dir()` is `/tmp`, a tmpfs of
  1,048,576 inodes (`df -i /tmp`), which today holds 38,249 top-level entries, 28,885 of them
  `klarch-transcript-*` and 7,985 `knowledge-*` (`ls /tmp`, counted by prefix).
  The owner's rule, in the owner's private instructions for this machine, keeps the working files
  and builds of a session, and of the subagents it briefs, out of `/tmp`; it says nothing of the
  files a test binary creates when a session runs it.
- **c. Whether it is already recorded.** The failure the rule's reason names is recorded:
  `issue@testing@a-test-run-leaves-directories-in-the-temporary-directory` (verdict `touches`, with
  an owner-question) measured `/tmp` full of these entries, builds failing with "No space left on
  device", and a mutation reading that failure as `caught`, which
  `issue@mutate@an-environment-failure-reads-as-caught` (verdict `touches`) also holds. Its closing
  work, "Each helper removes what it creates when dropped, and a run with its own `TMPDIR` leaves
  none", ends the leftovers whatever the answer here.
  `design@knowledge-architect@testing-crate` (verdict `constrains`, with the same owner-question)
  places the means the tests share, where such a rule would be read.
  `issue@check@a-home-for-developer-contracts-outside-agent-configuration` (verdict `touches`) records
  the scoped `CLAUDE.md` as an unreliable home for such a developer contract.
- **d. Its kind.** A question on the reach of an instruction of the owner, outside the project. Not
  a gap of the project's text: the tests' directory is a portable default, and the rule is a fact
  of one machine.
- **e. Its scope.** The owner's own instruction, and `goal@agents-get-a-complete-workflow`. The
  owner of a fix: `testing`, or every crate that tests, if the project states it; the owner's
  user-level or local file, if the owner's rule states it.
- **f. A better fix.** Three shapes. A project text that binds every crate's tests, with the
  sandboxes moved, puts one machine's tmpfs into a rule every contributor and CI follows. `cargo x
  gates` setting `TMPDIR` for its test gate reaches only the gate, and not a test run by hand or by
  `cargo x mutate`. The owner's rule extended, in the owner's own file, to the tests a session runs,
  for instance by having the session set `TMPDIR` to a directory of its own, binds this machine
  alone and changes nothing in the project. The deciding fact is the rule's intent, which only the
  owner holds: the session cannot read from its words whether it reaches a test binary's files.
- **g. Its route.** For the owner. Whatever the answer, the leftovers stay the testing issue's work.
- **h. The proposal.** Ask the owner whether the rule reaches the temporary directories of the tests
  a session runs. Default: no change in the project; the tests keep `std::env::temp_dir()`; if the
  rule does reach them, the owner states it in the owner's own file, which the session does not edit.

Default: no change in the project.
Outcome: for the owner (a fact only the owner holds: whether the owner's user-level rule reaches the files a test run creates): no change in the project; the tests keep `std::env::temp_dir()`. The owner: "Agreed on these 4 defaults"

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.
