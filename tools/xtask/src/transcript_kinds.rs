//! The `transcript-kinds` subcommand: the inventory of the line kinds of Claude Code's transcripts,
//! which this repository's transcript heads take before a value of the format is classed.
//!
//! The measurement serves the maintainers of the reader alone, so it is a command of this tool
//! rather than a verb of `klarch transcript`, per `design@xtask@a-local-task-is-the-maintenance-tool-s`;
//! the reading and the labels it counts by are the published reader's, through
//! `knowledge_architect_transcript::kinds`, so the rows hold what every verb reads.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::process::complain;
use knowledge_architect_gates::project_root;
use knowledge_architect_transcript::KindsOptions;
use std::process::ExitCode;

pub fn run(options: &KindsOptions) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(error) => return abort(&format!("no working directory: {error}")),
    };
    let Some(root) = project_root(&cwd, MANIFEST_NAME) else {
        return abort(&format!(
            "not inside the project: no ancestor holds {MANIFEST_NAME}"
        ));
    };
    match knowledge_architect_transcript::kinds(&root, options) {
        Ok(code) => code,
        Err(why) => {
            complain(&format!("xtask: {why}"));
            ExitCode::from(2)
        }
    }
}

fn abort(message: &str) -> ExitCode {
    complain(&format!("xtask: {message}"));
    ExitCode::FAILURE
}
