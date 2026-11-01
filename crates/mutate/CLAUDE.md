# mutate

What the modes do and how to run them is `path@mutate@README.md`; why they are shaped the way they
are is `register@mutate@design`.

**This file holds what is true of the code as it stands.**

## Every line goes through `say!` and `complain!`

The macros of `path@mutate@src/print.rs` ignore a write error. A bare `println!` panics when the
reader closes the pipe, and a panic between the mutation and the restore leaves the target mutated
with its record standing.

## Only one branch of the restore writes to the target

The decision is `design@mutate@restore-by-comparison`; this is what enforces it. `restore_verdict`
returns `Safe` only when the file equals the mutant the run wrote, and that is the one path on which
`unwind` writes the pre-image over content. Its other write is to a target the command deleted,
where nothing is there to lose, and a dangling symlink there is refused. `AlreadyRestored` and
`FileChanged` write nothing. A write added
to either, or a wider `Safe`, removes the property that makes a run over uncommitted work safe.

## The stale-binary trap

Restoring a file with an old modification time leaves cargo running the previous mutant's build.
Every restore is a fresh `fs::write`, which updates it; a restore by renaming a saved copy brings
the trap back.

## The record's completeness is checked per file, not by a marker

`Record::exists` requires the manifest, the pre-image and the mutant. Checking the manifest alone,
relying on `Record::write` putting it last, would rest the guarantee `restore` depends on, that a
record reporting itself present has its pre-image, on a write order nothing observes.

## There is no automatic discharge of a record

The decision is `design@mutate@discard-is-the-callers-word`; the test
`a_duplicated_region_does_not_discharge_the_record` in `path@mutate@tests/modes.rs` guards it.

## The record is claimed by creating its directory

`Record::claim` calls `fs::create_dir`, which is atomic, so of two runs racing exactly one proceeds.
Checking `exists()` and then writing lets both pass and mixes their files. The `owner-pid` file
beside them is what makes `restore` refuse while its owner is alive.

## The target is checked for a symlink again before the restore writes

`target_is_mutable` refuses a symlink before the run, and `unwind` checks again before writing,
because the command in between can replace the path, and `fs::write` follows a symlink.

## Mutating this crate with `cargo x mutate`

This repository's xtask links the crate, so each `cargo x` call over a mutated source of it
rebuilds xtask before it runs, and a run of several mutations costs a build of xtask per mutation.
A mutant that removes a refusal of a FIFO target makes the test that builds one block on its read:
a test under mutation that never returns is a catch, ended by killing it.

## The tests re-enter their own executable

`path@mutate@tests/modes.rs` runs each mode as a child process, so a kill is observable: the
child is the test executable itself, started on its test `child_entry` with the arguments in an
environment variable. The command under `--` in a test is a shell builtin or `sh -c`, never
`cargo`, which would rebuild the workspace inside a test.
