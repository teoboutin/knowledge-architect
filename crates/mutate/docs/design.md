# Design — mutate

How the hand mutation is built, and why. It serves the check that a test discriminates, performed
over work that is not committed yet, which the installed development procedure,
`skill@knowledge-architect-development@discriminating-tests`, asks of every claim.
Where the crate sits in the project, and who depends on it, is the root's
`design@knowledge-architect@mutation-command-host`.

## What a run mutates

### A mutation is one the caller writes and names; the tool generates and enumerates none `##mutation-command`

`run` takes one mutation from its caller, `--replace OLD --with NEW` or `--patch`, and applies
it to the live tree; the tool chooses no site and generates no mutant. The value of the check is a
mutation its author writes against a claim, the plausible wrong implementation the claim is about,
and a tool that generates its own mutants answers another question: whether some mutant of a
function survives, not whether the test written for one claim catches the wrong implementation of
that claim. That is why cargo-mutants, which generates its own, is no substitute,
`rejected@mutate@cargo-mutants-instrument`, and why thaum's sweep, which enumerates the
invariant-check sites of one directory of its engine, is not taken in,
`rejected@mutate@sweep-shipped`. A recorded list of mutations, the sweep generalised, waits for a
second real use.

It serves `goal@mutate@a-named-mutation-is-checked-safely`, which excludes a mutation the crate
chooses, and `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: the generic part of
thaum's tool, which its owner took in, and `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
since a mutation check performed by hand, as a scratch worktree with each step written out, leaves
each step to a session.

### A run mutates one file, and refuses a patch that names another `##one-file-per-run`

`run` takes the one file `--file` names; a `--patch` that names any other path is refused with 2,
before the baseline. The record, its comparison and its restore are per file, and a patch whose
other hunks the run would drop gives a verdict about a mutant that is not the one the caller wrote.
`goal@mutate@a-named-mutation-is-checked-safely` admits a mutation of several files, so this is the
current limit of the tool, not its scope: `issue@mutate@a-mutation-of-several-files`.

### The apply, the run and the restore are one command `##one-command-owns-the-interval`

A hand mutation performed as separate acts leaves a window in which the file is mutated and
anything may edit it. Both failures the tool exists to prevent live in that window: an edit inside
it defeats the exact reversal, and nothing between applying and observing establishes that what
ran was mutated. One command bounds the window to its own execution, so the tool asserts both ends,
the read-back line before the command runs and the comparison before the restore, rather than
instructing a session to.

The measurement it rests on was taken in thaum, where the tool was built: performed by hand against
an instruction, three of five mutation checks in a single unit of work were invalid, two because the
edit never applied and one because the verdict was read off a grep that matched cargo's own error
line, as thaum's commit "1f unit 10: who answers a question is one value, so the fabrication is
unrepresentable" records.

## How a run gives its verdict

### A verdict is never the tool's exit code, which says only whether the run started and whether the tree is owed a look `##report-not-gate`

The verdict is the exit code of the caller's command, printed as `caught` or `SURVIVED`, or
`unmutatable here` where the `--build` probe failed. A command or a probe that cannot be spawned
gives no verdict. The tool's own code is 0 after any completed run, 1 when it acted and something is
owed on the tree (no verdict reached, the target left mutated, a record still standing), and 2 when
the run never started (a bad argument, no repository, or a precondition that refused before the
target was written). The table's home is the crate-level documentation of `path@mutate@src/lib.rs`.

A survivor is information the session records in its commit, and a run whose mutation survives has
done its work; a nonzero code for it would read as the tool failing. The line between 2 and the
rest keeps the reason of `design@core@exit-code-ladder`, which binds the core's commands alone: a
caller told "never ran" from "ran". 1 means something else than the ladder's negative answer,
because no answer about a subject is the tool's to give.

### A build failure of the mutant is told apart from a catch only by a probe the caller names `##build-probe-separates-unbuildable`

A mutant that does not compile makes the command fail for a reason that is no test noticing. With
`--build`, a probe that fails reads `unmutatable here`; without one, a failing command reads
`caught` with the words that it cannot tell the two apart, as
`goal@mutate@a-named-mutation-is-checked-safely` asks. The probe is the caller's, because the
command is: the tool cannot know which build a command implies. The probe is not run over the
unmutated tree, so a probe that always fails reads every mutant as unbuildable, which the printed
verdict shows the caller.

### The command runs over the unmutated tree first, and a failure there refuses the run, unless the caller skips it `##baseline-runs-by-default`

Over a base on which the command already fails, every mutation reads as caught, so the baseline is
the default and `--no-baseline` the caller's word that the failure is expected and understood. An
opt-in baseline would leave the default run open to that false catch, which
`goal@mutate@a-named-mutation-is-checked-safely` rules out unless the session skipped the
baseline.

### The caller's command runs in the caller's environment `##caller-environment-kept`

The baseline, the probe and the command inherit the environment of the process that runs the mode,
`RUSTC_BOOTSTRAP` included. The gates library scrubs that variable from its gates, per
`design@gates@gates-scrub-rustc-bootstrap`, because a verdict of the gates is taken from one run.
A verdict here compares the baseline and the mutant, both under the same environment, so a variable
both inherit changes neither side of the comparison, and a caller who set one for its command gets
it.

## How a run restores

### The restore compares bytes, and writes only what the run produced `##restore-by-comparison`

Before restoring, the run reads the target back and compares it with the mutant it wrote. It writes
the pre-image only on equality. When the file differs it writes nothing at all and prints the
mutation's patch, which reverses the hunk and leaves a concurrent edit standing. The restore can
never remove content the run did not itself put there, which is what lets a mutation run over
uncommitted work, per `design@knowledge-architect@no-operation-loses-content`, and what
`goal@mutate@a-named-mutation-is-checked-safely` asks of the crate's own writes. Holding the mutation
in a worktree instead is `rejected@mutate@worktree-holds-the-mutation`.

The patch is the fallback, and its bound is measured: `git apply -R` matches the hunk's three
context lines, so any edit inside them makes it refuse; `-C1` narrows the match to one line either
side and recovers an edit two or more lines from the mutation; an edit adjacent to the mutated line,
or on it, reverses under neither, as thaum's commit "[mutate] The review's findings repaired: an
unsound discharge dropped, and the binary put under test" measured. Byte comparison has no such
bound. Every failure of either is a refusal that writes nothing.

### Nothing discharges a recovery record but the caller's `--discard` `##discard-is-the-callers-word`

A restore that refuses leaves the record standing, and the caller drops it with `--discard` once
they have unwound the mutation themselves. `--discard` writes no file, so it cannot be wrong about
the tree. A test that answers "somewhere in this file" where the question is "at this position"
clears a record over a live mutation, and the record is the only copy of the pre-image:
`rejected@mutate@forward-apply-check-proof`.

### The recovery record is written whole before the target is touched, and kept while the target may lack its content `##record-survives-the-run`

The pre-image, the mutant and the patch are written, complete, before the target is written. A hard
kill at any later instant leaves the original recoverable by `restore`, and a record that reports
itself present always carries its pre-image. A write of the mutant that fails after the file was
opened has truncated it, so the record stays unless the file still holds its pre-image. This is
where `goal@mutate@a-named-mutation-is-checked-safely` asks the content to be kept when the crate
cannot put it back.

### The recovery record lives in the git directory `##record-lives-in-the-git-directory`

The record is at the path `git rev-parse --git-path knowledge-architect-mutate` names. The git
directory is the one place every target project has, since a target project is a git
repository, per `goal@knowledge-architect@serves-one-kind-of-project`, where neither `cargo clean`
nor `git clean` reaches the only copy of a pre-image. No project writes an ignore rule for it, the
checker's walk, which lists untracked files that no ignore rule covers, per
`design@core@git-supplies-the-walk`, never reads it, and each worktree of a repository gets its
own. A directory of the working tree, as thaum's tool keeps under its own directory, would ask every
project to choose and ignore one, and `target/` would lose the record to a routine `cargo clean`.
