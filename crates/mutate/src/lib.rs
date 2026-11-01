//! The hand mutation: apply one mutation the caller writes, run a command against it, and put the
//! file back, over a working tree that may hold uncommitted work.
//!
//! A project's maintenance binary exposes it as `cargo x mutate`: it flattens [`MutateCommand`]
//! into its own command line and hands the parsed value to [`run`] with the project's root, which
//! it finds itself. What each mode does and what a caller must respect is the crate's README; why
//! the tool is shaped this way is its design home.
//!
//! **A verdict is never an exit code.** The verdict is the exit code of the caller's command:
//! nonzero reads `caught`, zero reads `SURVIVED`. [`run`] returns, whatever the verdict:
//!
//! | code | meaning |
//! | ---- | ------- |
//! | 0 | the run completed |
//! | 1 | the tool acted and something is owed on the tree: no verdict was reached, the target was left mutated, or a record still stands |
//! | 2 | the run never started: a bad argument, no repository, or a precondition that refused before the target was written |
//!
//! The functions of the harness, which establish that the mutation reached the code that ran and
//! that removing it removes nothing else, are public for a caller that composes its own run.

// Ties this crate's build to the checkout it is built from: cargo rebuilds it, and everything
// linking it, when `KNOWLEDGE_ARCHITECT_CHECKOUT` changes value, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration sees it unset, and still compiles.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

mod harness;
mod print;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{ArgGroup, Args, Subcommand};

pub use harness::{
    plan_replace, record_dir, restore_verdict, single_hunk_diff, target_is_mutable, PlanError,
    Record, Recorded, RestoreVerdict,
};

/// The two modes. A maintenance binary carries them under a command of its own:
///
/// ```ignore
/// Mutate {
///     #[command(subcommand)]
///     command: knowledge_architect_mutate::MutateCommand,
/// },
/// ```
#[derive(Subcommand, Debug)]
pub enum MutateCommand {
    /// Apply one mutation you write, run a command against it, and put the file back.
    Run(RunArgs),
    /// Unwind a run that was killed.
    Restore(RestoreArgs),
}

/// The mutation is described in exactly one of two forms, and clap enforces it.
///
/// `--replace` and `--with` are one form and travel together; `--patch` is the other. The group
/// makes one of the two required and both together impossible.
#[derive(Args, Debug)]
#[command(group = ArgGroup::new("form").required(true).args(["replace", "patch"]))]
pub struct RunArgs {
    /// The file to mutate, relative to the project's root.
    #[arg(long, value_name = "FILE")]
    pub file: String,
    /// The text to substitute. Refused unless it occurs exactly once.
    #[arg(long, value_name = "OLD", requires = "with", conflicts_with = "patch")]
    pub replace: Option<String>,
    /// What to substitute for it.
    #[arg(long, value_name = "NEW", requires = "replace")]
    pub with: Option<String>,
    /// A diff, relative to the project's root, for a mutation that is not one substitution.
    #[arg(long, value_name = "PATCH", conflicts_with = "with")]
    pub patch: Option<PathBuf>,
    /// Probe that the mutant builds, split on whitespace and run with no shell; without it a
    /// compile failure reads as caught.
    #[arg(long, value_name = "COMMAND")]
    pub build: Option<String>,
    /// Skip the check that the command passes over the unmutated tree.
    #[arg(long)]
    pub no_baseline: bool,
    /// The command whose exit code is the verdict, run from the project's root. Everything after
    /// `--`.
    #[arg(last = true, required = true, num_args = 1.., value_name = "COMMAND")]
    pub command: Vec<String>,
}

/// The arguments of the recovery mode.
#[derive(Args, Debug)]
pub struct RestoreArgs {
    /// Drop the record, writing no file.
    #[arg(long)]
    pub discard: bool,
}

/// Runs one mode over the project at `root`, and returns the exit code of the table above.
///
/// `root` is the directory the target paths and the commands are relative to: a maintenance
/// binary passes the project's root, found by the checker's manifest name. The recovery record
/// lives in the git directory of the repository that holds `root`, per [`record_dir`], so `root`
/// must lie inside a git working tree.
pub fn run(root: &Path, command: &MutateCommand) -> ExitCode {
    match command {
        MutateCommand::Run(args) => run::run_hand_mutation(root, args),
        MutateCommand::Restore(args) => run::restore_record(root, args.discard),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{CommandFactory, Parser};

    #[derive(Parser)]
    struct Cli {
        #[command(subcommand)]
        command: MutateCommand,
    }

    // clap's own consistency check over the whole derive, which catches a malformed declaration
    // at test time rather than at first invocation.
    #[test]
    fn cli_declaration_is_consistent() {
        Cli::command().debug_assert();
    }

    // The claim: every shape the declaration means to refuse is refused at parse time, so the
    // match in `run_hand_mutation` has no reachable third arm.
    #[test]
    fn every_shape_the_declaration_rejects_is_refused_at_parse_time() {
        let refused: &[&[&str]] = &[
            &["t", "run", "--file", "f", "--", "true"],
            &["t", "run", "--file", "f", "--replace", "a", "--", "true"],
            &["t", "run", "--file", "f", "--with", "b", "--", "true"],
            &[
                "t",
                "run",
                "--file",
                "f",
                "--replace",
                "a",
                "--with",
                "b",
                "--patch",
                "p",
                "--",
                "true",
            ],
            &[
                "t", "run", "--file", "f", "--patch", "p", "--with", "b", "--", "true",
            ],
            &["t", "run", "--file", "f", "--replace", "a", "--with", "b"],
            &["t", "run", "--replace", "a", "--with", "b", "--", "true"],
            &["t"],
        ];
        for argv in refused {
            assert!(
                Cli::try_parse_from(*argv).is_err(),
                "{argv:?} must be refused"
            );
        }
    }

    #[test]
    fn the_accepted_shapes_parse_to_what_they_name() {
        let cli = Cli::parse_from([
            "t",
            "run",
            "--file",
            "f",
            "--replace",
            "a",
            "--with",
            "b",
            "--no-baseline",
            "--build",
            "cargo build",
            "--",
            "cargo",
            "test",
        ]);
        let MutateCommand::Run(args) = cli.command else {
            panic!("`run` parses as the run mode");
        };
        assert_eq!(args.replace.as_deref(), Some("a"));
        assert_eq!(args.with.as_deref(), Some("b"));
        assert!(args.no_baseline);
        assert_eq!(args.build.as_deref(), Some("cargo build"));
        assert_eq!(args.command, ["cargo", "test"]);

        let cli = Cli::parse_from(["t", "restore", "--discard"]);
        let MutateCommand::Restore(args) = cli.command else {
            panic!("`restore` parses as the restore mode");
        };
        assert!(args.discard);
    }
}
