# Goals — mutate

The hand mutation, the package `knowledge-architect-mutate`, called the crate below, lets a session
check that a test catches a wrong implementation the session names.

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

A verdict is given only about the mutant read back from the files before the command ran. The
verdict tells the command noticing the mutant apart from a base on which the command already fails,
unless the session waived that, and from a mutant that does not build; where it cannot tell them
apart, the crate refuses the run or says so in the verdict.

It is met while the workflow's setup gives a Rust target project a command that runs the crate, and
the crate's tests show, over uncommitted files: a caught verdict and a survived one, each with the
files restored byte for byte; the mutant read back before the command runs; a run killed while its
mutation is applied, and the files written back from what the crate kept; a restore that refuses to
overwrite a file edited during the run; a base on which the command already fails refused, or
reported as such in the verdict; a mutant that does not build reported as such; a verdict that says
what it cannot tell; and a mutation of several files applied and restored.

It refines `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

- Requires, for instance: a file with uncommitted edits holds those edits, byte for byte, after a
  run that mutated it.
- Admits, for instance: a test command that runs one test, or the whole suite.
- Rules out, for instance: the crate proposing a mutation from the session's diff.
