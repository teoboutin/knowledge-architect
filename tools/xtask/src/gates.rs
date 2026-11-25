//! The `gates` subcommand: this repository's gate list, run by the published gates library.
//!
//! The list below is the primary home of what this repository's gates are, per
//! `design@xtask@gates-list-primary-home`. How they run is the library's,
//! `register@gates@design`.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::GatesArgs;
use knowledge_architect_gates::{
    cross_clippy, process::complain, project_root, rust_project, Checker, Gate,
};
use std::process::ExitCode;

/// The target of the cross gate: the code particular to Windows is judged on the machine that writes
/// it, and not by CI alone. The `-gnu` target, since the gate's stub fails for an `-msvc` one, per
/// `design@xtask@gates-list-primary-home`.
const WINDOWS: &str = "x86_64-pc-windows-gnu";

/// This repository's gates: the recommended list of a Rust project that uses the checker, with
/// the checker being this repository's own package, built from the checkout, and clippy for Windows
/// after clippy, except on Windows, where clippy runs for the machine's own target. CI runs it through
/// `cargo --locked x gates --locked --fail-fast --require-rebased --full --ci`: `--locked` is a
/// flag because locally a legitimately updated `Cargo.lock` must not fail a gate, while in CI lock
/// drift is exactly what must fail, per `design@xtask@ci-differs-by-flags`.
fn gates() -> Vec<Gate> {
    let mut gates = rust_project(
        Checker {
            package: "knowledge-architect",
            prefix: &[],
        },
        "origin/main",
    );
    let after_clippy = gates
        .iter()
        .position(|gate| gate.name == "clippy")
        .map_or(gates.len(), |clippy| clippy + 1);
    gates.splice(after_clippy..after_clippy, cross_clippy(WINDOWS));
    gates
}

pub fn run(args: &GatesArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(error) => return abort(&format!("no working directory: {error}")),
    };
    // The root is the nearest ancestor holding the checker's manifest, its name spelled by
    // `check`'s constant, so a rename of the manifest cannot leave xtask looking for the old one;
    // the dependency on `check` costs nothing, since the check gate builds it anyway.
    let Some(root) = project_root(&cwd, MANIFEST_NAME) else {
        return abort(&format!(
            "not inside the project: no ancestor holds {MANIFEST_NAME}"
        ));
    };
    knowledge_architect_gates::run(&root, &gates(), args)
}

/// A run that could not start exits 2, as the library's own refusals do, per
/// `design@gates@a-refusal-exits-two`.
fn abort(message: &str) -> ExitCode {
    complain(&format!("xtask: {message}"));
    ExitCode::from(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim: this repository runs the recommended list, with its own package as the checker and
    /// origin/main as the base, and, after `clippy`, the cross gate, but on Windows. Mutation check:
    /// another package or base fails the commits gate's assertion; the insertion of the cross gate
    /// removed fails the names, and so does `cross_clippy` asked for the machine `windows` elsewhere.
    #[test]
    fn this_repository_runs_the_recommended_list_over_its_own_checker_and_the_cross_gate() {
        let gates = gates();
        let names: Vec<&str> = gates.iter().map(|gate| gate.name.as_str()).collect();
        let mut expected = vec!["rebased", "fmt", "check", "commits", "clippy"];
        // The machine, never the library's answer, which is what the test judges.
        if !cfg!(windows) {
            expected.push("clippy-x86_64-pc-windows-gnu");
        }
        expected.extend(["doc", "test"]);
        assert_eq!(names, expected);
        assert_eq!(
            gates[3].args,
            [
                "run",
                "-q",
                "--release",
                "-p",
                "knowledge-architect",
                "--",
                "commits",
                "origin/main..HEAD"
            ]
        );
    }

    /// The claim: the toolchain file declares the cross gate's target, or the gate fails on a machine
    /// whose toolchain does not hold it, which CI's are. Mutation checked: `targets = []`; caught.
    #[test]
    fn the_toolchain_file_declares_the_cross_gate_s_target() {
        let toolchain = include_str!("../../../rust-toolchain.toml");
        let line = format!("targets = [\"{WINDOWS}\"]");
        assert!(toolchain.lines().any(|l| l == line), "no `{line}`");
    }

    /// The lines of a workflow, without its blank ones and its comments. YAML reads ` #` in a plain
    /// scalar as the start of a comment too, which CI does not run, and no scalar of the file holds one.
    fn lines(workflow: &str) -> Vec<&str> {
        workflow
            .lines()
            .map(|l| l.split(" #").next().unwrap_or(l).trim_end())
            .filter(|l| !l.is_empty() && !l.trim_start().starts_with('#'))
            .collect()
    }

    /// The claims: one matrix job runs CI's gates command on each of the three systems, on the pull
    /// request's head with its history, off a draft, under a timeout, with one artifact per system;
    /// and the job `check`, the context the ruleset requires, runs whatever that job answered, off a
    /// draft, and passes on `success` alone; a pull request alone starts the workflow. The whole file but
    /// its comments is compared, since a line added beside those, in any spelling YAML takes, can take a
    /// system out, let a failure pass or leave a step out; a change of the workflow is made here too. The command holds `--ci`, so a test that needs a tool a
    /// machine may lack fails there without it, per `design@gates@ci-flag-for-ci-only-requirements`.
    /// Mutations checked, each caught: `exclude: [{ os: windows-latest }]` added to the matrix;
    /// `shell: bash -c "exit 0" {0}` added to the gates step, and to the step of `check`;
    /// `"continue-on-error": true` added to the job; `"if": runner.os == 'Linux'` added to the gates step;
    /// `--ci` made a comment of the command, ` # --ci`.
    #[test]
    fn ci_runs_the_gates_on_three_systems_behind_one_gathering_job() {
        let workflow = include_str!("../../../.github/workflows/ci.yml");
        let expected = "\
name: ci
on:
  pull_request:
    types: [opened, synchronize, reopened, ready_for_review]
permissions:
  contents: read
concurrency:
  group: ci-${{ github.event.pull_request.number }}-${{ github.event.pull_request.draft }}
  cancel-in-progress: true
jobs:
  gates:
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    if: ${{ !github.event.pull_request.draft }}
    runs-on: ${{ matrix.os }}
    timeout-minutes: 30
    defaults:
      run:
        shell: bash
    steps:
      - uses: actions/checkout@v5
        with:
          ref: ${{ github.event.pull_request.head.sha }}
          fetch-depth: 0
      - name: toolchains
        run: |
          cargo --version
          rustc -vV
      - uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6
      - name: gates
        run: cargo --locked x gates --locked --fail-fast --require-rebased --full --ci
      - name: gate logs
        if: failure()
        uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a
        with:
          name: gate-logs-${{ matrix.os }}
          path: target/gates/
  check:
    needs: gates
    if: ${{ !github.event.pull_request.draft && always() }}
    runs-on: ubuntu-latest
    timeout-minutes: 5
    steps:
      - name: the gates passed on every system
        run: test '${{ needs.gates.result }}' = success
";
        assert_eq!(lines(workflow), lines(expected));
    }

    /// The claim: the alias `x` runs the maintenance crate in release, here and in the configuration a
    /// project is given, so no gate replaces the binary that runs the gates where the gate's cargo
    /// reads the inputs of the cargo that built it, which a caller's `RUSTC_BOOTSTRAP` changes, since
    /// the gates library removes it from every child, per
    /// `design@agent-skills@release-maintenance-build`. The snippet is no TOML
    /// before its placeholder is filled, so the lines are read. Mutations checked: `--release` removed
    /// from either; each caught.
    #[test]
    fn the_x_alias_runs_the_maintenance_crate_in_release() {
        for (file, text) in [
            (
                "the cargo configuration",
                include_str!("../../../.cargo/config.toml"),
            ),
            (
                "the snippet",
                include_str!("../../../crates/agent-skills/snippets/cargo-config.toml"),
            ),
        ] {
            let aliases: Vec<&str> = text.lines().filter(|l| l.starts_with("x = ")).collect();
            assert_eq!(
                aliases,
                ["x = \"run -q --release -p xtask --\""],
                "in {file}"
            );
        }
    }

    /// The claim: the parse test of the saved workflows reads the variable the library's `--ci`
    /// sets, though it spells it by hand, since its crate depends on no other. Mutation checked:
    /// the spelling in `path@agent-skills@src/workflow_parse.rs` changed by one letter; caught.
    #[test]
    fn the_parse_test_reads_the_library_s_ci_variable() {
        let source = include_str!("../../../crates/agent-skills/src/workflow_parse.rs");
        let line = format!(
            "const CI_VAR: &str = \"{}\";",
            knowledge_architect_gates::CI_VAR
        );
        assert!(source.contains(&line), "no `{line}` in the parse test");
    }
}
