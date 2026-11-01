//! The `mutate` subcommand: the published mutation library, run over this repository.
//!
//! What its modes do is `path@mutate@README.md`, and why they are shaped so is
//! `register@mutate@design`; this module finds the root and hands it over.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::{process::complain, project_root};
use knowledge_architect_mutate::MutateCommand;
use std::process::ExitCode;

pub fn run(command: &MutateCommand) -> ExitCode {
    let Some(root) = std::env::current_dir()
        .ok()
        .and_then(|cwd| project_root(&cwd, MANIFEST_NAME))
    else {
        // 2: the run never started, per the library's table of exit codes.
        complain(&format!(
            "xtask: not inside the project: no ancestor holds {MANIFEST_NAME}"
        ));
        return ExitCode::from(2);
    };
    knowledge_architect_mutate::run(&root, command)
}
