//! The `transcript-kinds` subcommand: the inventory of the line kinds of Claude Code's transcripts,
//! over every project's, which this repository's transcript heads take before a value of the format
//! is classed.
//!
//! The measurement serves the maintainers of the reader alone, so it is a command of this tool
//! rather than a verb of `klarch transcript`, per `design@xtask@a-local-task-is-the-maintenance-tool-s`.
//! It reads through the published reader's own files, which `main.rs` includes with `#[path]`, so
//! its rows hold what every verb reads and labels, and the reader's library exports nothing for it,
//! per `design@xtask@internals-included-not-exported`.

use crate::kinds::line_kinds;
use crate::reader::{project_files, transcript_dir, Corpus};
use knowledge_architect_gates::process::{complain, say};
use std::process::ExitCode;

/// Print the header, then one row per place and kind; one JSON object per line under `json`. Exit 1
/// when no line is read, 2 when the transcripts cannot be read.
pub fn run(json: bool) -> ExitCode {
    let corpus = match transcript_dir()
        .and_then(|dir| Ok((project_files(&dir, None)?, dir)))
        .and_then(|(found, dir)| Corpus::read(&dir, &found.files, true))
    {
        Ok(corpus) => corpus,
        Err(why) => {
            complain(&format!("xtask: {why}"));
            return ExitCode::from(2);
        }
    };
    for (object, text) in corpus.inventory.header_lines() {
        say(&format!(
            "{}\n",
            if json { object.to_string() } else { text }
        ));
    }
    let rows = line_kinds(&corpus);
    for ((place, kind), row) in &rows {
        let versions: Vec<&str> = row.versions.iter().map(String::as_str).collect();
        let line = if json {
            serde_json::json!({
                "place": place,
                "kind": kind,
                "lines": row.count,
                "versions": versions,
                "keys": row.keys,
            })
            .to_string()
        } else {
            let keys: Vec<String> = row.keys.iter().map(|(k, n)| format!("{k} {n}")).collect();
            format!(
                "{place} {kind}: {} lines; versions {}; keys {}",
                row.count,
                if versions.is_empty() {
                    "none".to_string()
                } else {
                    versions.join(", ")
                },
                keys.join(", ")
            )
        };
        say(&format!("{line}\n"));
    }
    if rows.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
