# Gates — design

The library that runs a project's merge gates: every check a branch must pass before it merges,
run in order, with complete logs, distilled failures and one verdict from exit codes. That it is
a published library of its own, which a project's maintenance crate calls, is
`design@knowledge-architect@gates-crate`. The designs
it did not keep are `register@gates@rejected`, each cited by the heads whose argument covers it.

**What belongs here:** how the gates run. Which gates a project runs is that project's own; this
repository's are `design@xtask@gates-list-primary-home`.

### A project holds the gate list the library runs `##a-project-holds-its-gate-list`

The library runs the gates it is given and decides none of them, so a project writes no gate logic
beyond its list and the call that runs it: the rest is the library's, per
`design@knowledge-architect@gates-crate`. A project adds, removes or reorders gates in the list it
receives, so a Rust project's list is the recommended one, `design@gates@recommended-gate-list`,
with its own edits. The nearest rival, a list the library fixes, would leave no place for what is
particular to one project, which `goal@generic-tools-and-procedures-are-shipped` keeps in that
project: a gate of one project's own is in that project's list alone until knowledge-architect's
owner takes it in.

### The library offers a recommended gate list to a Rust project that uses the checker `##recommended-gate-list`

`rust_project` builds the recommended list of a Rust project that uses the checker. It takes how
the project runs its checker, a package and the arguments before the checker's command, and the
main branch as git names it. The list's gates, in the cost order `design@gates@gates-run-all` runs
them in, with the distiller each one is named with, from the closed set of
`design@gates@distillers-are-a-closed-set`, what each distiller keeps being the doc comment of
`Distiller`:

| gate | what it runs | its distiller |
| --- | --- | --- |
| `rebased` | the rebase check, behind its flag, per `design@gates@rebase-checks-behind-a-flag` | `Ancestry` |
| `fmt` | formatting | `Whole` |
| `check` | the document check | `Whole` |
| `commits` | the commit messages of the branch | `Whole` |
| `clippy` | clippy, with its warnings denied | `WithoutProgress` |
| `doc` | the documentation build, with rustdoc's warnings denied | `WithoutProgress` |
| `test` | the tests | `Tests` |

Where two gates' costs do not separate, the reason for their order is a comment at the gate.

The list is the generic part of a Rust project's gates, and
`goal@generic-tools-and-procedures-are-shipped` ships such a part to each target project that
carries out its activity once knowledge-architect's owner takes it in. So a gate enters the list on
the owner's word, which is that taking-in: the owner took the documentation build in. The nearest
rival, no recommended list, leaves each project to write its own list, so no gate the owner took in
reaches a target project.

### The `doc` gate denies every rustdoc warning through the source of rustdoc flags cargo reads first, set in its own environment `##doc-gate-denial`

The `doc` gate of the recommended list, `design@gates@recommended-gate-list`, runs
`cargo doc --workspace --no-deps` with `CARGO_ENCODED_RUSTDOCFLAGS` in its `envs`, set to `-D` and
`warnings` joined by the byte 0x1f, so every rustdoc warning fails it, a link that names nothing
among them, and the project declares nothing for it, as the clippy gate denies its warnings by its
`-- -D warnings`. Cargo has four sources of rustdoc flags and uses the first one set:
`CARGO_ENCODED_RUSTDOCFLAGS`, then `RUSTDOCFLAGS`, then every matching
`target.<triple>.rustdocflags` and `target.<cfg>.rustdocflags` of the configuration, joined, then
its `build.rustdocflags`, as the cargo reference's entry on `build.rustdocflags` says. The gate's variable replaces in its child one of the same name the
caller exported, as the doc comment of `Gate::envs` says of any variable it names, so it is the
source cargo uses.

The standing argument is how cargo and rustdoc behave, observed on cargo and rustdoc 1.98.0 over a
crate whose one doc comment links to an item that does not exist:

| row | how the deny is set | the broken link | exit |
| --- | --- | --- | --- |
| 1 | none, `cargo doc --no-deps` | a warning | 0 |
| 2 | `RUSTDOCFLAGS="-D warnings"` | an error | 101 |
| 3 | `[lints.rustdoc] broken_intra_doc_links = "deny"` in the crate's manifest | an error | 101 |
| 4 | `build.rustdocflags = ["-D", "warnings"]` in the crate's cargo configuration file | an error | 101 |
| 5 | that configuration, with `RUSTDOCFLAGS="--cfg foo"` exported | a warning | 0 |
| 6 | `--config 'build.rustdocflags=["-D","warnings"]'`, with `RUSTDOCFLAGS="--cfg foo"` exported | a warning | 0 |
| 7 | `cargo test --doc`, with `RUSTDOCFLAGS="-D warnings"` | not reported | 0 |
| 8 | `RUSTDOCFLAGS="-D warnings"`, with `CARGO_ENCODED_RUSTDOCFLAGS` exported empty | a warning | 0 |
| 9 | `CARGO_ENCODED_RUSTDOCFLAGS` set to `-D` and `warnings` joined by 0x1f | an error | 101 |
| 10 | that variable, with `RUSTDOCFLAGS="--cfg foo"` exported and `--config 'build.rustdocflags=["--cfg","bar"]'` | an error | 101 |

Each row is re-taken by this script, run in a new empty directory, under the toolchain rustup
selects there, or the one `RUSTUP_TOOLCHAIN=<version>` names. Each line that ends in
`echo "row <n>: $?"` prints the exit code of its row; each other line after the first two prepares
the crate for the next row:

```sh
cargo new -q --lib probe && cd probe
printf '//! Crate doc linking [`Nowhere`].\n' > src/lib.rs
cargo doc --no-deps -q; echo "row 1: $?"
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -q; echo "row 2: $?"
printf '[lints.rustdoc]\nbroken_intra_doc_links = "deny"\n' >> Cargo.toml
cargo doc --no-deps -q; echo "row 3: $?"
sed -i '/lints.rustdoc/,$d' Cargo.toml
mkdir -p .cargo && printf '[build]\nrustdocflags = ["-D", "warnings"]\n' > .cargo/config.toml
cargo doc --no-deps -q; echo "row 4: $?"
RUSTDOCFLAGS="--cfg foo" cargo doc --no-deps -q; echo "row 5: $?"
rm -r .cargo
RUSTDOCFLAGS="--cfg foo" cargo doc --no-deps -q --config 'build.rustdocflags=["-D","warnings"]'; echo "row 6: $?"
RUSTDOCFLAGS="-D warnings" cargo test --doc -q; echo "row 7: $?"
CARGO_ENCODED_RUSTDOCFLAGS= RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -q; echo "row 8: $?"
CARGO_ENCODED_RUSTDOCFLAGS="$(printf '%s\037%s' -D warnings)" cargo doc --no-deps -q; echo "row 9: $?"
CARGO_ENCODED_RUSTDOCFLAGS="$(printf '%s\037%s' -D warnings)" RUSTDOCFLAGS="--cfg foo" cargo doc --no-deps -q --config 'build.rustdocflags=["--cfg","bar"]'; echo "row 10: $?"
```

Row 1 defeats the nearest rival, a plain `cargo doc` with an ask that each project declares its
`[lints.rustdoc]` levels: a project that has not yet followed the ask gets a gate that passes on a
broken link, `rejected@gates@plain-cargo-doc-with-a-lints-ask`. Rows 5 and 6 defeat the deny
passed through cargo's configuration, which an exported `RUSTDOCFLAGS` drops with no sign,
`rejected@gates@deny-through-build-rustdocflags`. Row 8 defeats the deny passed through
`RUSTDOCFLAGS` in the gate's environment, which an exported `CARGO_ENCODED_RUSTDOCFLAGS`, even
empty, drops with no sign, `rejected@gates@deny-through-rustdocflags`; rows 9 and 10 show that the
deny through `CARGO_ENCODED_RUSTDOCFLAGS` holds over an exported `RUSTDOCFLAGS` and a configured
flag. Row 7 rules out the `test` gate as the carrier of the check,
`rejected@gates@link-check-in-the-test-gate`.

The costs accepted:

- in the gate's child, no rustdoc flag the caller exported or the project configured is used, by
  the order the cargo reference states, which rows 5, 6 and 8 show for three of its sources; a project whose documentation needs a rustdoc flag adds it to the doc gate's
  variable in its own list, after `warnings`, separated by the byte 0x1f, per
  `design@gates@a-project-holds-its-gate-list`;
- the variable's value cannot be typed as plain text in a shell: a reader who runs the gate's
  command by hand writes the separator, as `CARGO_ENCODED_RUSTDOCFLAGS="$(printf '%s\037%s' -D warnings)"`;
- alternating a plain `cargo doc` with the gate documents the workspace's crates again, since the
  flags differ;
- rustdoc reports `missing_docs` too, where the project's lints warn on it, so one missing doc
  fails both `clippy` and `doc`. `-A missing_docs` in the gate would avoid the second report, and
  is gate logic only for that.

### The `doc` gate leaves out `--document-private-items` and `--lib`, so it documents a library's public items, the text docs.rs renders, and a binary's private items `##doc-gate-item-scope`

The doc gate runs without `--document-private-items`, so a broken link in the documentation of a
library's private item passes it. Cargo enables the flag by default for a binary target it
documents, as the reference of `cargo doc` says of the flag, so a broken link in the documentation
of such a binary's private item fails the gate; the same reference's Target Selection skips a
binary named as its package's library, one with `doc = false`, and one missing its required
features. For a binary, two scopes were weighed: its private items, as cargo documents them
by default, since rust-analyzer resolves a private doc comment's links on hover; and `--lib`, which
leaves binaries out, the text docs.rs renders being a library's. The session chose the binaries'
private items over `--lib`, which no goal ranks; the owner approved it. For a library, two scopes were weighed: public items alone, the text docs.rs renders, which a consumer of the crate reads; and
private items too, since rust-analyzer resolves the links of a private doc comment on hover, so a
broken one misleads a developer the same way, at the cost, at each adopting project's pin move, of
whatever warnings its private documentation gives. The session chose public items alone over
private items too, which no goal ranks; the owner approved it. A project that wants its private
documentation checked adds the flag to the doc gate in its own list, per
`design@gates@a-project-holds-its-gate-list`. What the wider scope adds in a tree is re-taken by
running `cargo doc --workspace --no-deps --document-private-items --message-format=short` and the
same command without the flag, and comparing their warnings.

### The library owns the gates' flags `##the-library-owns-the-flags`

The flags of a gates run are the library's `GatesArgs`, which derives clap's `Args`, so a project's
binary flattens them into its own command and every project's gates take the same flags: the flags
are gate logic, of which a project writes none, per `design@gates@a-project-holds-its-gate-list`.
clap's major version is therefore part of the library's interface, as it is of `cli`'s.

### `--ci` sets one variable in every gate's environment, which every later requirement CI holds and a local run does not reads `##ci-flag-for-ci-only-requirements`

Under `--ci`, every gate's child runs with `KNOWLEDGE_ARCHITECT_CI=1`, the library's `CI_VAR`, beside
the environment it inherits. A project's test that needs a tool a machine may lack, such as a
JavaScript runtime, reads it: unset, the test skips where the tool is missing, so a machine without
the tool can run the gates; set, it fails there, so CI, which passes the flag, never passes with the
check unrun. It is one flag for every such requirement, a behaviour made stricter on CI as much as a
tool CI holds: a project's CI command, which a reader copies to reproduce CI's verdict, states the
difference between CI and a local run, and a second requirement adds no flag. The flags each argued
before it, `--locked`, `--fail-fast`, `--require-rebased` and `--full`, keep their own. Three shapes were weighed: a test reading the `CI` variable every
CI system sets, which needs no flag and leaves CI's verdict out of the command a reader copies; one
flag per requirement, which keeps the command whole and adds a flag at each requirement; and one
flag setting one variable. The owner chose the last over both, which no goal ranks. The flag is the
library's, per `design@gates@the-library-owns-the-flags`: a requirement a project's tests read is
met by the same flag in every project.

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
to iterate on. Distillers start conservative; the distiller of each gate of the recommended list is
`design@gates@recommended-gate-list`. The nearest rival, a failed gate's full output on stdout,
buries what failed: a test run's log holds one line per passing test, which
`grep -c ' \.\.\. ok$' target/gates/test.log` counts after a passing `cargo x gates`, beside the
log's length, `wc -l target/gates/test.log`. The rival reopens if those lines become a small part
of the log.

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
