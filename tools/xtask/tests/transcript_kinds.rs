//! `cargo x transcript-kinds` run as a binary over transcripts this test writes, in a directory of
//! its own that it removes.

use std::path::{Path, PathBuf};
use std::process::Command;

// This target links no library of its package, so it reads the variable itself, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// A transcript directory holding `files`, each a path under `projects/` and its lines.
fn config(tag: &str, files: &[(&str, &[&str])]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "xtask-transcript-kinds-{tag}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("projects")).expect("a transcript directory");
    for (path, lines) in files {
        let path = dir.join("projects").join(path);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("a project directory");
        std::fs::write(&path, lines.join("\n") + "\n").expect("a transcript");
    }
    dir
}

fn run(config: &Path, args: &[&str]) -> (String, i32) {
    let mut all = vec!["transcript-kinds"];
    all.extend_from_slice(args);
    let out = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .args(&all)
        .env("CLAUDE_CONFIG_DIR", config)
        .output()
        .expect("the binary runs");
    let _ = std::fs::remove_dir_all(config);
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

const OWNER: &str = r#"{"type":"user","uuid":"a1","parentUuid":null,"sessionId":"s","cwd":"/p","version":"2.1.316","origin":{"kind":"human"},"message":{"content":"hi"}}"#;
const SUB: &str = r#"{"type":"user","uuid":"b1","parentUuid":null,"sessionId":"s","version":"2.1.316","message":{"content":"go"}}"#;
const OWNERLESS: &str = r#"{"type":"session-owner","sessionId":"o"}"#;

/// The claim: the command prints the reader's header, then one row per place and kind with its
/// count, its versions and its keys, the subagent files included, those of a top-level file that
/// holds no session among them; and exits 0.
#[test]
fn the_inventory_counts_each_kind_of_every_file_the_reader_reads() {
    let dir = config(
        "rows",
        &[
            ("-p/s.jsonl", &[OWNER]),
            ("-p/s/subagents/agent-a1.jsonl", &[SUB]),
            ("-q/o.jsonl", &[OWNERLESS]),
            ("-q/o/subagents/agent-a2.jsonl", &[SUB]),
        ],
    );
    let (stdout, code) = run(&dir, &[]);
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.starts_with("# files read: 4; harness versions: 2.1.316\n"),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "\ntop user owner: 1 lines; versions 2.1.316; keys cwd 1, message 1, origin 1,"
        ),
        "{stdout}"
    );
    assert!(
        stdout.contains("\nsubagent user dispatch (no field): 2 lines;"),
        "{stdout}"
    );
    assert!(
        stdout.contains("\ntop session-owner: 1 lines; versions none; keys sessionId 1, type 1\n"),
        "{stdout}"
    );
}

/// The claim: `--json` prints the header and each row as one JSON object.
#[test]
fn the_inventory_prints_json_under_its_flag() {
    let dir = config("json", &[("-p/s.jsonl", &[OWNER])]);
    let (stdout, code) = run(&dir, &["--json"]);
    assert_eq!(code, 0, "{stdout}");
    assert!(
        stdout.starts_with(r#"{"files":1,"header":"read","versions":["2.1.316"]}"#),
        "{stdout}"
    );
    assert!(
        stdout.contains(r#""kind":"user owner","lines":1,"place":"top","versions":["2.1.316"]"#),
        "{stdout}"
    );
}

/// The claim: a corpus of no line exits 1, a negative answer, after its header.
#[test]
fn the_inventory_of_no_line_exits_one() {
    let dir = config("empty", &[]);
    let (stdout, code) = run(&dir, &[]);
    assert_eq!(code, 1, "{stdout}");
    assert_eq!(stdout, "# files read: 0; harness versions: none\n");
}

/// The claim: transcripts that cannot be read exit 2, the command could not run, never 1.
#[test]
fn the_inventory_of_no_transcript_directory_exits_two() {
    let missing = std::env::temp_dir().join(format!(
        "xtask-transcript-kinds-missing-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&missing);
    let (stdout, code) = run(&missing, &[]);
    assert_eq!(code, 2, "{stdout}");
}
