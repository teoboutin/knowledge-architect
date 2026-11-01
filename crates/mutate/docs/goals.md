# Goals — mutate

What the hand mutation, the package `knowledge-architect-mutate` called the crate below, is for is
the goal below. A goal is met or unmet, where a decision about how the crate is built is won or lost
and lives in `register@mutate@design`. A goal stays here while it is met, and leaves only when the
owner abandons it. This Component is published, so its goals are encouraged to refine a goal of
`register@knowledge-architect@goal`.

## A session checks that its test catches a wrong implementation the session names, over uncommitted work, and the crate's own writes lose nothing of the mutated files `##a-named-mutation-is-checked-safely`

An agent session working in a target project names a mutation of files of the project: the wrong
implementation a claim of its work rules out, written for the occasion or kept from an earlier one.
The crate applies it, runs against it a test command the session chooses, and reports whether the
command noticed. The crate never chooses or generates the mutation.

The crate's own writes lose nothing of the mutated files. Their content before the mutation,
uncommitted edits included, is back in them when the run ends; where the crate cannot put it back
without writing over content it did not write, or the run was interrupted, that content is kept
where the crate's recovery reads it, and the crate writes it back only over the mutant the crate
itself wrote, or where the file is gone. What the session's command writes is the session's.

A verdict is given only about the mutant read back from the files before the command ran. Where the
crate cannot tell the command noticing the mutant from a base on which the command already fails,
it refuses the run, unless the session skipped that baseline; where it cannot tell it from a mutant
that does not build, it says so when the session gave no build probe, and reports a mutant whose
probe fails as unbuildable when it gave one.

It is met while the setup skill gives a Rust target project a `cargo x` command that runs the
crate, and the crate's tests show, over uncommitted files: a caught verdict and a survived one,
each with the files restored byte for byte; the mutant read back before the command runs; a run
killed while its mutation is applied, and the files written back from what the crate kept; a
restore that refuses to overwrite a file edited during the run; a red base refused; a mutant that
fails its build probe reported as unbuildable; a verdict with no probe saying what it cannot tell;
and a mutation of several files applied and restored.

It refines `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.
