# knowledge-architect-gates

Runs a project's merge gates as one command: every check a branch must pass before it merges, run
in order, each one's complete output kept under target/gates/, a distilled extract of what
failed, and one verdict taken from the children's exit codes alone.

A project calls it from its own maintenance binary, conventionally a crate named `xtask` run
through the cargo alias `cargo x`. The binary holds the project's gate list, usually the
recommended list of a Rust project that uses knowledge-architect, and hands it to the library with
the flags it parsed. How to set that up is part of the setup skill that knowledge-architect
installs; the library's interface is at <https://docs.rs/knowledge-architect-gates>.

## What a run produces

- **stdout**: one verdict line per gate, and for each failed gate a distilled extract, headed by
  the path of its log. A run refuses to write into a pipe outside GitHub Actions: a pipe's reader
  replaces the exit code, and a filter drops lines. Redirect to a file when the text is wanted.
- **`target/gates/<gate>.log`**: the gate's complete output, both streams, always written.
- **A slow-tests line** whenever libtest warned that a test ran over 60 seconds.
- **Exit code**: 0 exactly when every gate ran and passed, 1 when a gate failed or could not be
  started, and 2 when the run could not start, refused before any gate ran. Output handling never
  changes it.

Under GitHub Actions, each gate is wrapped in a `::group::`, and a failed gate adds one `::error`
line naming it.

## The flags

| flag | effect |
| --- | --- |
| `--fail-fast` | stop at the first failing gate; by default every gate runs |
| `--full` | stream each gate's raw output live as well |
| `--locked` | pass `--locked` to every gate that resolves dependencies |
| `--require-rebased` | run the rebase checks: HEAD must contain the main branch |
| `--ci` | set `KNOWLEDGE_ARCHITECT_CI=1` in every gate's environment, `CI_VAR`, so a test that needs a tool a machine may lack fails without it instead of skipping |

## Clippy for another system

The recommended list judges the code of the system it runs on. Beside that list, the library offers
`cross_clippy(target)`, a gate that runs clippy for another target on the machine at hand, with no cross
compiler. A project adds it to its own list, and one that never does is not concerned:

```rust
let mut gates = rust_project(checker, "origin/main");
gates.extend(cross_clippy("x86_64-pc-windows-gnu"));
```

It gives the gate `clippy-<target>`, or nothing on a machine whose own system is the target's. It is a gate
for another system alone: another architecture or another C library of the machine's own system, such as
`aarch64-unknown-linux-gnu` or `x86_64-unknown-linux-musl` on an `x86_64` Linux machine, gets nothing. The
target's standard library must be in the toolchain: `rustup target add <target>`, or `targets` in the project's
`rust-toolchain.toml`, which rustup installs on first use.

What it reports is compile errors and lints, and no failure of a test. Its limits:

- **It names a program that does nothing as the target's C compiler, C++ compiler and archiver**, per
  `design@gates@cross-clippy-stub`, so a build script for which that is not enough fails the gate, such as
  those behind `openssl` and `aws-lc-rs`.
- **A build script that runs the compiler to choose a feature can be answered wrongly**, so the code the
  gate judges can differ from what the target builds. CI on that system judges the rest.
- **An `-msvc` target fails**, so code particular to `-msvc` is not judged: name the `-gnu` target.
- The gate empties `RUSTC_WRAPPER`, so its builds use no wrapper's cache.
- Every toolchain that reads the project's `targets` downloads the target, CI's jobs included; each target
  directory holds a build for the target; and a lint in code the systems share is reported twice.

Why it is offered beside the list is `design@gates@cross-clippy-gate`. A project for which it comes to fail
on something of its own build drops the gate from its list, and one
with a real cross compiler removes the variables from the gate's `envs`.

## On Windows and macOS

The gates run on Linux, Windows and macOS, with the same command. The cargo alias `cargo x` runs the
maintenance crate in the release profile, `run -q --release -p xtask --`, and must: in the dev profile a gate
can replace the program that runs the gates, which Windows refuses, per
`design@agent-skills@release-maintenance-build`. A gate that runs the maintenance crate itself, as `check` and
`commits` do in the scaffold's list, leaves that program in place while the gate's cargo reads the inputs of
the cargo that built it: a `RUSTC_BOOTSTRAP` that the caller exports, and a profile setting given by
`--config` to one of the two, break that, and the gate fails there on Windows. Developing a project on Windows is
not covered yet: a run does not refuse a pipe on stdout there.

## License

Licensed under either of the MIT license (LICENSE-MIT) or the Apache License 2.0
(LICENSE-APACHE), at your option.
