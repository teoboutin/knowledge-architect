---
kind: defect
---
# A baseline whose command builds nothing runs on whatever build the target directory holds, a previous run's mutant included

## Summary

`cargo x mutate run` runs its baseline, the caller's command over the unmutated tree, without
running `--build` first, and the build it makes of the mutant stays in the target directory after
the source is restored. A command that builds nothing, such as a script that runs
`target/debug/klarch`, therefore takes its baseline on the previous run's mutant, so the baseline
can fail or pass on code that is not the tree's.

## Details

### What

In `path@mutate@src/run.rs`, `run`: the block under `if baseline` calls
`run_command(root, &command)` over the unmutated tree, and the `--build` probe,
`run_command(root, &build_parts(b))`, runs only after the mutant is written, before the mutant's
command. Nothing rebuilds the unmutated tree after the source is restored.

Seen once, by an adversarial reviewer of the record search's audit options on the branch that built `design@query@since-marks-changed-entries`, whose mutation command was a shell script calling
`target/debug/klarch` over a project in a subdirectory of its repository: the baseline of that
script failed until the reviewer rebuilt by hand, after an earlier run had left its mutant built.
The mechanism above is read from the code; the run itself was not repeated by the session that
writes this entry, so its exact output is not established here.

Reproduction, by construction: run a mutation with `--build "cargo build -p knowledge-architect
--bin klarch"` whose mutant breaks a behaviour a script checks, then run any mutation whose command
is that script and builds nothing. The second run's baseline runs the first run's mutant binary.

Not affected: a command that builds what it runs, such as `cargo test`, which recompiles the
restored source before its baseline.

### Why it matters

A baseline that runs on the wrong build answers for code that is not the tree's: it can refuse a run
over a sound tree, or pass one whose command would fail on the tree, and every verdict then rests
on it, against `goal@mutate@a-named-mutation-is-checked-safely`.

### What would close it

The baseline runs `--build` first when one is given, or the run rebuilds the restored tree before it
ends, with a test of a command that builds nothing taking its baseline on the unmutated build; or
the run's documentation states that the command must build what it runs, if the owner judges the
case out of scope.
