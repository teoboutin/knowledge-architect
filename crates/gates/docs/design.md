# Gates — design

The library that runs a project's merge gates: every check a branch must pass before it merges,
run in order, with complete logs, distilled failures and one verdict from exit codes. That it is
a published library of its own, which a project's maintenance crate calls, is
`design@knowledge-architect@gates-crate`. The designs
it did not keep are `register@gates@rejected`, each cited by the heads whose argument covers it.

**What belongs here:** how the gates run. Which gates a project runs is that project's own; this
repository's are `design@xtask@gates-list-primary-home`.

### A project holds its gate list, and the library offers a recommended one `##a-project-holds-its-gate-list`

The library runs the gates it is given and decides none of them, so a project writes no gate logic
beyond its list and the call that runs it: the rest is the library's, per
`design@knowledge-architect@gates-crate`. `rust_project` builds
the recommended list of a Rust project that uses the checker, in cost order: the rebase check behind
its flag, formatting, the document check, the commit messages of the branch, clippy, and the tests.
It takes how the project runs its checker, a package and the arguments before the checker's command,
and the main branch as git names it. A project adds, removes or reorders gates in the list it
receives, so a Rust project's list is the recommended one with its own edits.

### The library owns the gates' flags `##the-library-owns-the-flags`

The flags of a gates run are the library's `GatesArgs`, which derives clap's `Args`, so a project's
binary flattens them into its own command and every project's gates take the same flags: the flags
are gate logic, of which a project writes none, per `design@gates@a-project-holds-its-gate-list`.
clap's major version is therefore part of the library's interface, as it is of `cli`'s.

### The distillers are a closed set the library holds `##distillers-are-a-closed-set`

The distillers are a closed set, `Distiller`: a project chooses one for each gate and cannot add
its own, so every distiller is tested here. A project that needs another distiller is a reason to
add one to the library.

### The gates start every child through the library's one spawn helper, which a project's commands may use `##one-spawn-helper`

`path@gates@src/process.rs` spawns a child, captures its two streams into one buffer, optionally
mirrors them live, and holds the best-effort output helpers every print goes through. The gates use
it, and a project's own commands may: a new command calls it and never edits it.

**The contracts on a child hold only where it is started there.** Its one `Command::new` is where
every child loses `RUSTC_BOOTSTRAP`, per `design@gates@gates-scrub-rustc-bootstrap`, gets a null
stdin, and has both streams drained concurrently, per `design@gates@streams-through-two-pipes`.
The nearest rival, each gate or each command of a project spawning its child by its own code,
repeats each of those at every site, and a site that misses the scrub passes nightly-gated code
that CI rejects, the false green the gates exist to exclude. A gate with a need the helper does
not meet is met by extending the helper, so every child keeps the contracts. The command
`grep -rn 'Command::new' crates/gates/src` finds the one site.

### A child's stdout and stderr are read concurrently through two pipes `##streams-through-two-pipes`

The spawn helper, `design@gates@one-spawn-helper`, reads a child's stdout and stderr concurrently
through two pipes into one buffer. Reading both pipes concurrently is what prevents the deadlock
where the child blocks writing a pipe nobody drains. Two pipes have none of the three losses a
regular file shared by both streams showed: it lost output written through /dev/stdout and output
of a descendant that outlives the child, and a killed run left the file behind to fail a later run,
as the rejected alternative on it records.

The design this decision did not keep: `rejected@gates@streams-through-one-file`.

### Every gate runs; nothing fails fast by default `##gates-run-all`

CI stops at the first failing step because its deliverable is a red X. The gates' deliverable
is a complete work list, and a fail-fast default costs one full re-run per fixed gate. So the
default runs all gates in cost order whatever the earlier ones returned, and `--fail-fast`
is the flag CI or an impatient session opts into. The accepted cost: on a broken build,
`cargo test` repeats the compile errors clippy already reported.

### The verdict comes from child exit codes alone `##verdict-from-exit-codes`

The run exits 0 exactly when every gate ran and passed. No distiller, no log writing, no
slow-test extraction sits between a child's exit status and the run's own: a bug in output
handling can degrade what is printed, never flip a verdict. Printing itself is inside the
contract: every stdout and stderr write is best-effort, so a write error truncates the report
and never the run, the logs or the exit code.

### A run that could not start exits 2, and a gate that failed exits 1 `##a-refusal-exits-two`

The run exits 1 when a gate ran and failed or could not be started, and 2 when the run could not
start: a refusal before any gate ran, such as a pipe on stdout or a log directory it cannot create,
and, in a caller that finds the project first, as a maintenance crate does, no project. A parse
failure clap refuses already exits 2. So a caller tells "the gates never ran" from "a gate
failed", as the checker's commands let it, per `design@cli@exit-code-ladder`, and as the hand
mutation exits for a run that never started, per `design@mutate@report-not-gate`. The nearest rival, one failure code for both, would
keep 1 for a refusal and 2 for a mistyped flag of the same run, a split no argument supports.

### Outside GitHub Actions, a gates run refuses a pipe on stdout `##gates-refuse-a-pipe`

**Outside GitHub Actions, a gates run refuses a pipe on stdout before running anything.** The
exit code is the run's product, and a pipe hands it to the reader at the far end: `| tail`
exits with tail's status in a shell without `pipefail`, which
`design@agent-skills@harness-shell-fails-on-a-pipe` sets in the `claude` harness, and in any shell a filter drops lines of a
report that is already distilled, so
the loss is silent both ways. A terminal and a regular file pass, so `> log 2>&1` stays
available, and the complete logs under the project's target/gates/ need no `tee`.

**Under GitHub Actions the pipe is accepted.** The runner captures a step's output through a
pipe and takes the step's verdict from the shell's exit code, so no reader stands between the
run and its verdict, which is the refusal's reason. `GITHUB_ACTIONS` set to `true` is how the
tool knows; the output then stays live in the job log.

### A gate marked as a rebase check runs only under a flag `##rebase-checks-behind-a-flag`

`--require-rebased` adds the gates marked as rebase checks. The recommended list holds one,
`rebased`, first: `git merge-base --is-ancestor <base> HEAD`, where the base is the main branch
as the project names it, such as origin/main. Under a fast-forward or rebase merge, the tree CI judges
is the tree the main branch receives only when the branch contains the base; a branch cut before
`main` moved is tested alone and lands on a different tree. It is first because it builds
nothing. It is behind a flag because a branch not yet rebased is normal while it is worked on;
CI passes the flag. It reads the local copy of the base, so a caller fetches first.

### Under GitHub Actions, the run marks each gate with workflow commands, so the job's log and summary name it `##annotations-under-actions`

A CI job that runs every gate in one step has a step list that does not name the failing gate. Under
Actions the library prints GitHub workflow commands around each gate: `::group::<gate>` before it
and `::endgroup::` after its verdict line, so the job log folds per gate, and for a failed gate
one `::error title=<gate>::` line, which the job summary lists by the gate's name. Outside
Actions none of it is printed.

### A failed gate's stdout is a distilled extract of a log that keeps its full output `##distill-over-full-logs`

Every gate's full output — both streams — is always written to `target/gates/<gate>.log`,
under `target/` so it is ignored and leaves with the build. stdout gets one verdict line per
gate and, for each failed gate, a distilled extract headed by its log path. A distiller is named on its gate
and chooses what to print from the full text; it can be aggressively
wrong at the cost of one read of the log, which is what makes conservative distillation safe
to iterate on. Distillers start conservative: in the recommended list, fmt, check and commits pass through
whole, clippy drops cargo's progress lines, test keeps the failures and the compiler's own
diagnostics.

### No gate inherits `RUSTC_BOOTSTRAP` from its caller `##gates-scrub-rustc-bootstrap`

The spawn helper removes `RUSTC_BOOTSTRAP` from the environment of every child the library starts,
gates included, and a child that needs a value sets it explicitly. `RUSTC_BOOTSTRAP=1` makes a
whole build nightly-equivalent, so nightly-gated code that CI rejects compiles and passes: thaum's
review reproduced it with a `#![feature(test)]` doc test. A crate whose one doc test opens with
`#![feature(test)]` and `extern crate test;` takes it again: `cargo test --doc` fails on a stable
toolchain, and `RUSTC_BOOTSTRAP=1 cargo test --doc` passes. libtest also reads the variable's mere
presence. A caller who exported it would turn every local gate run into that false green, and the
verdict run exists to exclude it. The test `gate_children_never_inherit_a_caller_exported_bootstrap`
in `path@xtask@tests/gates_bin.rs` asserts the scrub.
