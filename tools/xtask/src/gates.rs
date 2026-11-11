//! The `gates` subcommand: this repository's gate list, run by the published gates library.
//!
//! The list below is the primary home of what this repository's gates are, per
//! `design@xtask@gates-list-primary-home`. How they run is the library's,
//! `register@gates@design`.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::GatesArgs;
use knowledge_architect_gates::{process::complain, project_root, rust_project, Checker, Gate};
use std::process::ExitCode;

/// This repository's gates: the recommended list of a Rust project that uses the checker, with
/// the checker being this repository's own package, built from the checkout. CI runs it through
/// `cargo --locked x gates --locked --fail-fast --require-rebased --full --ci`: `--locked` is a
/// flag because locally a legitimately updated `Cargo.lock` must not fail a gate, while in CI lock
/// drift is exactly what must fail.
fn gates() -> Vec<Gate> {
    rust_project(
        Checker {
            package: "knowledge-architect",
            prefix: &[],
        },
        "origin/main",
    )
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

    /// The claim: this repository runs the recommended list unchanged, with its own package as
    /// the checker and origin/main as the base. Mutation check: another package or base fails
    /// the commits gate's assertion.
    #[test]
    fn this_repository_runs_the_recommended_list_over_its_own_checker() {
        let gates = gates();
        let names: Vec<&str> = gates.iter().map(|gate| gate.name.as_str()).collect();
        assert_eq!(
            names,
            ["rebased", "fmt", "check", "commits", "clippy", "test"]
        );
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
