---
kind: defect
---
# A mutation run whose test filter matches no test reads as `SURVIVED`

## Summary

`cargo x mutate` runs a command and reads its exit code alone. A test filter that names no test, by
a wrong module path, runs nothing and exits 0, so the baseline passes and the mutated run reads as
`SURVIVED`: a false record that a test does not discriminate. The development skill warns of it;
two sessions read the warning and met it.

## Details

### What

- The development skill, in its source `path@agent-skills@content/skills/development/SKILL.md`:
  "**The command names the test under check by its module path, with `-- --exact` after it.** A
  filter that matches no test runs nothing, cargo exits 0, and the mutation reads as `SURVIVED`".
- `run_hand_mutation` in `path@mutate@src/run.rs` runs the baseline, then the mutated command,
  through `run_command`, which returns whether the command exited 0; no code of
  `path@mutate@src/` reads a test count: `grep -rn -i 'running 0\|0 tests' crates/mutate/src/`
  returns nothing at the intake of 2026-11-14.
- `--exact` does not guard the case: a wrong module path with `--exact` still matches no test.
  Reproduction at the intake of 2026-11-14:
  `cargo test -q -p knowledge-architect-mutate --lib no_such_module::no_such_test -- --exact`
  prints `running 0 tests` and `0 passed; ... 3 filtered out`, and exits 0. Any `cargo x mutate`
  run with that command after `--` passes its baseline and reads its mutant as `SURVIVED`.

Instances:

- the session of quick fixes, 2026-11-08, C1 of its workflow file: a test named by a wrong module
  path; the baseline printed `running 0 tests` and passed, and the mutated run printed
  `running 0 tests` and `SURVIVED`;
- the session on the primer's import line, 2026-11-11, its project file: the same slip, noted as no
  finding and corrected by the session.

Ruled out: a lapse. The instruction says what to look for and gives no check, and both sessions
read it.

The shapes:

- the baseline refuses a run that reports zero tests, naming the filter, and the skill's sentence
  shrinks to the `--exact` rule: it widens `design@mutate@baseline-runs-by-default`, whose rule is
  "a failure there refuses the run", with a second refusal, and ties the tool to cargo's output,
  where it otherwise takes any command after `--` and reads only its exit code, per
  `design@mutate@report-not-gate`;
- every verdict line prints the command's last lines, the closing work of
  `issue@mutate@an-environment-failure-reads-as-caught` widened from a `caught` to every verdict,
  which shows `0 passed` beside the `SURVIVED` with no new refusal and no reading of cargo's format;
- no change, the warning kept: it loses on the instances, since the warning was read and the
  verdict was still false, twice.

The second closes both false verdicts with one change, so the choice is weighed on the branch of
`issue@mutate@the-mutation-run-s-false-verdicts-on-one-branch`.

### Why it matters

A false `SURVIVED` records a test as not discriminating when it never ran, a false record of a
test's evidence, against `design@agent-skills@test-economy` and
`goal@the-workflow-leads-to-better-code`; a session that believes it adds a test or a claim for a
gap that does not exist.

### What would close it

A change to `mutate` under which a run whose command ran no test no longer reads as a bare
`SURVIVED`, by one of the first two shapes, with a test in `path@mutate@tests/modes.rs` whose
command matches no test; then the skill's sentence brought to what the tool now carries.
