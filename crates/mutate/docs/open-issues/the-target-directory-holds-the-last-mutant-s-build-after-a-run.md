---
kind: observation
---
# After a run, the target directory holds the last mutant's build until the next build

## Summary

`cargo x mutate run` restores the mutated source file when its command ends, and builds nothing
after the restore. A binary the command built over the mutant, such as the checker in
`target/debug/`, stays the mutant's until the next cargo build. A probe run by hand after a run,
calling that binary directly, runs the mutant and reports a behaviour the restored code does not
have. The tool's README does not say so. Whether this is a defect of the tool, a gap of its README
or expected behaviour is not established.

## Details

### What

Reported by the adversarial reviewer of the review before the merge of the intake of twenty
retrospective files, which ran mutations of `path@query@src/id.rs` with `cargo x mutate run` and
then probed the checker by hand: a probe read the mutant once, and after a rebuild it read the
restored code.

The mechanism, read in `path@mutate@src/run.rs`: the function `run_hand_mutation` applies the
mutation, runs the build probe if `--build` names one and then the caller's command, and restores
the file; no build follows the restore. The restore writes the file fresh, so its modification time is new and the
next cargo build recompiles, per the stale-binary trap of `path@mutate@CLAUDE.md`; a binary called
without cargo is not rebuilt.

`design@mutate@baseline-runs-by-default` already names the case at the start of a run: with
`--build`, the baseline builds the unmutated tree, so it is not taken on "whatever the target
directory holds, a previous run's mutant included". Nothing covers the end of a run.

Reproduction, assumed from the mechanism and the reviewer's report, not re-run here: in a checkout,
`cargo x mutate run --file <a file of the checker> --replace <text> --with <mutant text> -- cargo
build -p knowledge-architect`, then `target/debug/klarch <a command the mutant changes>` before any
other build: the output is the mutant's.

### Why it matters

A session that probes by hand after a mutation run can take the mutant's behaviour for the code's,
and record a false observation or a false confirmation of a claim; the reviewer saw it because the
probe's output contradicted the code it had just read. No recorded decision is strained:
`design@mutate@one-command-owns-the-interval` bounds what the tool owns to the run.

### What would close it

One of these, decided against `design@mutate@one-command-owns-the-interval`: a sentence in
`path@mutate@README.md` saying that the target directory holds the last mutant's build after a run
until the next build; a rebuild by the tool after the restore, with the probe `--build` names; or a
ruling that the behaviour is expected and needs no text, the entry then deleted with the reason in
its commit.
