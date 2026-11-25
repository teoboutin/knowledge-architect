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
/// it, and not by CI alone. The `-gnu` target, since the gate's stub fails for an `-msvc` one.
const WINDOWS: &str = "x86_64-pc-windows-gnu";

/// This repository's gates: the recommended list of a Rust project that uses the checker, with
/// the checker being this repository's own package, built from the checkout, and clippy for Windows
/// after clippy, except on Windows, where clippy judges the same code. CI runs it through
/// `cargo --locked x gates --locked --fail-fast --require-rebased --full --ci`: `--locked` is a
/// flag because locally a legitimately updated `Cargo.lock` must not fail a gate, while in CI lock
/// drift is exactly what must fail.
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

    /// The claim: CI runs the gates with `--ci`, so a test that needs a tool a machine may lack
    /// fails there without it, per `design@gates@ci-flag-for-ci-only-requirements`. Mutations
    /// checked: `--ci` removed from the gates step of the CI workflow, and turned into a comment
    /// of it, ` # --ci`; each caught.
    #[test]
    fn ci_runs_the_gates_with_the_ci_flag() {
        let workflow = include_str!("../../../.github/workflows/ci.yml");
        let step = workflow
            .lines()
            .find(|l| l.trim_start().starts_with("run: cargo --locked x gates"))
            .expect("the CI workflow runs the gates");
        // YAML reads ` #` in a plain scalar as the start of a comment, which CI does not run.
        let command = step.split(" #").next().unwrap_or(step);
        assert!(command.split_whitespace().any(|w| w == "--ci"), "{step}");
    }

    /// The lines of one job of a workflow, trimmed, without its comments: the lines indented under
    /// `  <name>:`, which is how this repository's workflow is written. YAML reads ` #` in a plain
    /// scalar as the start of a comment too, and no scalar of the file holds one otherwise.
    fn job<'a>(workflow: &'a str, name: &str) -> Vec<&'a str> {
        let header = format!("  {name}:");
        workflow
            .lines()
            .map(|l| l.split(" #").next().unwrap_or(l).trim_end())
            .filter(|l| !l.is_empty() && !l.trim_start().starts_with('#'))
            .skip_while(|l| *l != header)
            .skip(1)
            .take_while(|l| l.starts_with("   "))
            .map(str::trim)
            .collect()
    }

    /// The claim: the toolchain file declares the cross gate's target, or the gate fails on a machine
    /// whose toolchain does not hold it, which CI's are. Mutation checked: `targets = []`; caught.
    #[test]
    fn the_toolchain_file_declares_the_cross_gate_s_target() {
        let toolchain = include_str!("../../../rust-toolchain.toml");
        let line = format!("targets = [\"{WINDOWS}\"]");
        assert!(toolchain.lines().any(|l| l == line), "no `{line}`");
    }

    /// The conditions a job or one of its steps carries: the lines that open with `if:`.
    fn conditions<'a>(job: &[&'a str]) -> Vec<&'a str> {
        job.iter()
            .map(|l| l.trim_start_matches("- "))
            .filter(|l| l.starts_with("if:"))
            .collect()
    }

    /// The claims: one matrix job runs CI's gates command on each of the three systems, on the pull
    /// request's head with its history, off a draft, under a timeout, with one artifact per system;
    /// and the job `check`, the context the ruleset requires, runs whatever that job answered, off a
    /// draft, and passes on `success` alone. No line added beside those takes a system out, lets a
    /// failure pass, or leaves a step out on a system, and a pull request alone starts the workflow.
    /// Mutations checked, each caught: a system removed; `fetch-depth` removed; the artifact's name
    /// without the system; the draft condition removed; `always()` removed; the comparison changed to
    /// `!= failure`; the `needs` removed; `continue-on-error` added to either job or to a step of it;
    /// `exclude` added to the matrix; a condition added to the gates step; a `push` trigger added.
    #[test]
    fn ci_runs_the_gates_on_three_systems_behind_one_gathering_job() {
        let workflow = include_str!("../../../.github/workflows/ci.yml");
        let gates = job(workflow, "gates");
        for line in [
            "fail-fast: false",
            "os: [ubuntu-latest, windows-latest, macos-latest]",
            "runs-on: ${{ matrix.os }}",
            "timeout-minutes: 30",
            "shell: bash",
            "ref: ${{ github.event.pull_request.head.sha }}",
            "fetch-depth: 0",
            "name: gate-logs-${{ matrix.os }}",
        ] {
            assert!(gates.contains(&line), "no `{line}` in {gates:#?}");
        }
        let steps: Vec<_> = gates.iter().filter(|l| l.contains("x gates")).collect();
        assert_eq!(
            steps,
            [&"run: cargo --locked x gates --locked --fail-fast --require-rebased --full --ci"]
        );

        assert_eq!(
            conditions(&gates),
            [
                "if: ${{ !github.event.pull_request.draft }}",
                "if: failure()"
            ],
            "the job's and the artifact step's"
        );
        assert!(!gates.contains(&"exclude:"), "{gates:#?}");

        let check = job(workflow, "check");
        assert_eq!(
            conditions(&check),
            ["if: ${{ !github.event.pull_request.draft && always() }}"]
        );
        for line in gates.iter().chain(&check) {
            let line = line.trim_start_matches("- ");
            assert!(!line.starts_with("continue-on-error"), "{line}");
        }
        let triggers: Vec<_> = workflow
            .lines()
            .skip_while(|l| *l != "on:")
            .skip(1)
            .take_while(|l| l.starts_with(' ') || l.is_empty())
            .filter(|l| l.starts_with("  ") && !l.starts_with("   "))
            .collect();
        assert_eq!(triggers, ["  pull_request:"]);
        for line in [
            "needs: gates",
            "run: test '${{ needs.gates.result }}' = success",
        ] {
            assert!(check.contains(&line), "no `{line}` in {check:#?}");
        }
    }

    /// The claim: the alias `x` runs the maintenance crate in release, here and in the configuration a
    /// project is given, so no gate replaces the binary that runs the gates where the gate's cargo
    /// reads the inputs of the cargo that built it, which a caller's `RUSTC_BOOTSTRAP` changes, since
    /// the gates library removes it from every child. The snippet is no TOML
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
