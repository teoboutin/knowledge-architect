//! The `gates` subcommand: this repository's gate list, run by the published gates library.
//!
//! The list below is the primary home of what this repository's gates are, per
//! `design@xtask@gates-list-primary-home`. How they run is the library's,
//! `register@gates@design`.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::GatesArgs;

/// The variable `--ci` sets for every gate, which a test that needs a tool a machine may lack reads,
/// such as the parse test of the saved workflows in `path@agent-skills@src/workflow_parse.rs`: set,
/// the test fails where the tool is missing instead of skipping, per
/// `design@xtask@gates-list-primary-home`.
const CI_VAR: &str = "KNOWLEDGE_ARCHITECT_CI";

/// This repository's `gates` command: the library's flags, and one of its own.
#[derive(clap::Args, Debug, Default, Clone)]
pub struct Args {
    #[command(flatten)]
    pub gates: GatesArgs,

    /// Hold every requirement CI holds and a local run does not: a test that needs a tool a
    /// machine may lack, such as `node`, fails without it instead of skipping. CI passes it.
    #[arg(long)]
    pub ci: bool,
}
use knowledge_architect_gates::{process::complain, project_root, rust_project, Checker, Gate};
use std::process::ExitCode;

/// This repository's gates: the recommended list of a Rust project that uses the checker, with
/// the checker being this repository's own package, built from the checkout. CI runs it through
/// `cargo x gates --locked --fail-fast --require-rebased --full --ci`: `--locked` is a
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

pub fn run(args: &Args) -> ExitCode {
    if args.ci {
        // Set before any gate is spawned, in a process with one thread, and inherited by every
        // gate, the test gate's test binaries included.
        std::env::set_var(CI_VAR, "1");
    }
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
    knowledge_architect_gates::run(&root, &gates(), &args.gates)
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
}
