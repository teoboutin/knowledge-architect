//! The reader of Claude Code's session transcripts: the files a project's sessions wrote, cut into
//! sessions, each kept line labelled by what the harness recorded of it, and the inventory of what was not recognised.
//!
//! What the reader does not know it counts, never fails on and never drops in silence, per
//! `design@transcript@transcript-format-tolerated-and-reported`: it knows the sets of values under the
//! "known" constants below, each used or ignored on purpose. Lines are labelled by what the
//! harness recorded of them, never by a substring of their text, per `design@transcript@transcript-labels-in-code`, and a file is
//! cut into sessions per `design@transcript@transcript-session-split`.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

/// The top-level types of a chain line: they carry `uuid`, `parentUuid`, `sessionId` and the rest.
const CHAIN_TYPES: &[&str] = &["user", "assistant", "attachment", "system"];

/// The metadata types the harness writes beside the chain, ignored on purpose: none carries a
/// message.
const KNOWN_METADATA_TYPES: &[&str] = &[
    "api-request",
    "api-request-blob",
    "api-request-shape",
    "last-prompt",
    "atis-latch",
    "mode",
    "permission-mode",
    "ai-title",
    "queue-operation",
    "pr-link",
    "file-history-delta",
    "file-history-snapshot",
    "cost-state",
    "artifact-autoreact-ledger",
    "artifact-comment-monitor",
    "frame-link",
    "bridge-session",
    "tools-used",
    // Measured over the 930 lines of this type in the transcripts of one machine, 2026-11-11: each
    // holds `type`, `accountUuid`, `organizationUuid` and `sessionId` alone, the account that owns
    // the session and no message.
    "session-owner",
];

/// The `system` subtypes: `local_command`, `compact_boundary` and `model_refusal_no_fallback` are
/// used, the others ignored.
const KNOWN_SYSTEM_SUBTYPES: &[&str] = &[
    "turn_duration",
    "compact_boundary",
    "scheduled_task_fire",
    "informational",
    "local_command",
    "model_refusal_no_fallback",
    "bridge_status",
    // Classed on the lines its issue quoted, of a cloud session of harness 2.1.312, with no file of
    // the machine the reader was measured on holding one: a hook's report, with no message of the
    // owner or of the session, which is an assumption over those lines alone.
    "stop_hook_summary",
];

/// The `attachment.type` values: `queued_command` is used, the others ignored on purpose.
const KNOWN_ATTACHMENT_TYPES: &[&str] = &[
    "total_tokens_reminder",
    "deferred_tools_record",
    "prompt_snapshot",
    "environment",
    "queued_command",
    "date",
    "instructions",
    "model",
    "session_context",
    "credential_org",
    "mcp_instructions_delta",
    "remote_session_change",
    "skill_listing",
    "auto_mode",
    "silent_turn_reminder",
    "deferred_tools_delta",
    "agent_listing_delta",
    "edited_text_file",
    "nested_memory",
    "command_permissions",
    "read_truncation_notice",
    "structured_output",
    "hook_success",
    "hook_additional_context",
    "file",
    "task_status",
    "compact_file_reference",
    "invoked_skills",
    "hook_system_message",
    "thinking_drop",
    // Classed on the one line its issue quoted, of a cloud session of harness 2.1.312, with no file
    // of the machine the reader was measured on holding one: a reminder the harness injects, with
    // no message of the owner or of the session, which is an assumption over that line alone.
    "task_reminder",
];

/// The type of the line that opens a forked subagent's file, outside the chain: it names the line
/// of the parent's file up to which the fork inherits its context, and is shown as the fork's first
/// entry.
const FORK_CONTEXT_TYPE: &str = "fork-context-ref";

/// The texts the harness writes for the owner's interrupt of a turn. Some interrupts carry no
/// `interruptedMessageId`, and no other field tells them apart, so the whole text places them, on a line no field places, per `design@transcript@transcript-labels-in-code`.
const INTERRUPT_TEXTS: &[&str] = &[
    "[Request interrupted by user]",
    "[Request interrupted by user for tool use]",
];

/// The tags that open the two lines a command the owner runs with the `!` prefix writes: the
/// command, then its output, whose parent is the command's line. Read on a top-level line no field
/// places, per
/// `design@transcript@transcript-labels-in-code`.
const SHELL_INPUT_TAG: &str = "<bash-input>";
const SHELL_OUTPUT_TAG: &str = "<bash-stdout>";

/// The tags that open the lines of a command the owner runs, such as `/compact`, that the harness
/// writes with no `commandRun`: the command, `<command-name>` then `<command-args>`, and its output,
/// whose parent is the command's line. A `<command-name>` line that a `commandRun` line names as its
/// parent is that command's echo, printed once by the `commandRun` line. Read on a top-level `user`
/// line no field places and on a `local_command` line with no `commandRun`, per
/// `design@transcript@transcript-labels-in-code`.
const COMMAND_NAME_TAG: &str = "<command-name>";
const COMMAND_OUTPUT_TAG: &str = "<local-command-stdout>";

/// The `origin.kind` values: `human` is the owner, the others are injected.
const KNOWN_ORIGIN_KINDS: &[&str] = &["human", "peer", "task-notification", "coordinator"];

/// The content item types of an `assistant` line.
const KNOWN_ASSISTANT_ITEMS: &[&str] = &["text", "tool_use", "thinking"];

/// The content item types of a `user` line.
const KNOWN_USER_ITEMS: &[&str] = &["text", "tool_result"];

/// The text that opens a skill's body, delivered as a `user` line with `isMeta`.
const SKILL_BODY_PREFIX: &str = "Base directory for this skill:";

/// Where the harness keeps its transcripts: `$CLAUDE_CONFIG_DIR/projects`, or `~/.claude/projects`.
///
/// `Err` when neither variable is set, or the directory cannot be read: the command could not run.
pub(crate) fn transcript_dir() -> Result<PathBuf, String> {
    let config =
        match std::env::var_os("CLAUDE_CONFIG_DIR") {
            Some(dir) if !dir.is_empty() => PathBuf::from(dir),
            _ => match std::env::var_os("HOME") {
                Some(home) if !home.is_empty() => PathBuf::from(home).join(".claude"),
                _ => return Err(
                    "neither CLAUDE_CONFIG_DIR nor HOME is set, so no transcript directory can be \
                     found"
                        .to_string(),
                ),
            },
        };
    let dir = config.join("projects");
    match std::fs::metadata(&dir) {
        Ok(meta) if meta.is_dir() => Ok(dir),
        Ok(_) => Err(format!("{} is not a directory", dir.display())),
        Err(e) => Err(format!(
            "cannot read the transcript directory {}: {e}",
            dir.display()
        )),
    }
}

/// One line of a file: its parsed value, or the fact that it is no JSON.
pub(crate) enum Raw {
    Json(Value),
    Malformed,
    /// A line of whitespace alone: no line at all, so neither read nor reported.
    Blank,
}

/// One `.jsonl` file, read whole.
pub(crate) struct File {
    pub(crate) path: PathBuf,
    /// Index `i` is line `i + 1`.
    pub(crate) lines: Vec<Raw>,
}

impl File {
    fn read(path: &Path) -> Result<File, String> {
        let text =
            std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let text = String::from_utf8_lossy(&text);
        let lines = text
            .lines()
            .map(|line| match serde_json::from_str::<Value>(line) {
                Ok(value) if value.is_object() => Raw::Json(value),
                _ if line.trim().is_empty() => Raw::Blank,
                _ => Raw::Malformed,
            })
            .collect();
        Ok(File {
            path: path.to_path_buf(),
            lines,
        })
    }

    fn json(&self, index: usize) -> Option<&Value> {
        match &self.lines[index] {
            Raw::Json(value) => Some(value),
            Raw::Malformed | Raw::Blank => None,
        }
    }

    /// The indices of the chain lines, in file order.
    fn chain(&self) -> Vec<usize> {
        (0..self.lines.len())
            .filter(|&i| self.json(i).is_some_and(is_chain))
            .collect()
    }
}

fn is_chain(value: &Value) -> bool {
    value
        .get("type")
        .and_then(Value::as_str)
        .is_some_and(|t| CHAIN_TYPES.contains(&t))
}

/// Whether a line's content holds a `tool_result` item.
fn holds_tool_results(value: &Value) -> bool {
    value
        .get("message")
        .and_then(|m| m.get("content"))
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items
                .iter()
                .any(|i| str_field(i, "type") == Some("tool_result"))
        })
}

/// The kind of an `origin` object: its `kind`, or `(none)` for an `origin` that holds none, which no
/// known set holds, so it is reported rather than read as no `origin` at all.
fn origin_kind(origin: Option<&Value>) -> Option<&str> {
    origin.map(|o| str_field(o, "kind").unwrap_or("(none)"))
}

pub(crate) fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

/// What the first chain line of a file says of the directory its session ran in.
enum FirstCwd {
    /// The file holds no chain line, so no session: leaving it out drops nothing.
    NoChain,
    /// The `cwd` of the first chain line.
    Cwd(String),
    /// The first chain line, at this line number from 1, holds no `cwd`.
    Missing(usize),
}

/// The `cwd` of the first chain line of a file, read without parsing the rest.
fn first_cwd(path: &Path) -> Result<FirstCwd, String> {
    use std::io::BufRead;
    let file =
        std::fs::File::open(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    // Read as bytes and decoded lossily, as `File::read` does, so a line of bytes that are no
    // UTF-8 is a line that is no JSON in this mode too, not a file that cannot be read.
    for (i, line) in std::io::BufReader::new(file).split(b'\n').enumerate() {
        let line = line.map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let line = String::from_utf8_lossy(&line);
        if let Ok(value) = serde_json::from_str::<Value>(&line) {
            if is_chain(&value) {
                return Ok(match str_field(&value, "cwd") {
                    Some(cwd) => FirstCwd::Cwd(cwd.to_string()),
                    None => FirstCwd::Missing(i + 1),
                });
            }
        }
    }
    Ok(FirstCwd::NoChain)
}

/// The top-level `.jsonl` files of every project directory under `dir`, sorted.
fn top_level_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let projects =
        std::fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    for project in projects {
        let project = project.map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
        if !project.path().is_dir() {
            continue;
        }
        let entries = std::fs::read_dir(project.path())
            .map_err(|e| format!("cannot read {}: {e}", project.path().display()))?;
        for entry in entries {
            let entry =
                entry.map_err(|e| format!("cannot read {}: {e}", project.path().display()))?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "jsonl") {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

/// The top-level files a verb reads, and those it left out for want of a `cwd`.
pub(crate) struct ProjectFiles {
    pub(crate) files: Vec<PathBuf>,
    /// Each top-level file whose first chain line holds no `cwd`, with that line's number: no
    /// project can be told for it, so a run over one project leaves it out, and its header says so.
    pub(crate) no_cwd: Vec<(PathBuf, usize)>,
}

/// The top-level files whose first chain line's `cwd` is one of `roots` or a path under one, or
/// every top-level file when `roots` is `None`.
///
/// Both sides are canonicalised where they exist, so a symlinked checkout matches; a `cwd` that no
/// longer exists is compared as written. Only the first chain line is read: a session that later
/// works in a worktree changes its `cwd` mid-file, and the directory it started in is what names its
/// project.
pub(crate) fn project_files(dir: &Path, roots: Option<&[PathBuf]>) -> Result<ProjectFiles, String> {
    let files = top_level_files(dir)?;
    let Some(roots) = roots else {
        return Ok(ProjectFiles {
            files,
            no_cwd: Vec::new(),
        });
    };
    let roots: Vec<PathBuf> = roots
        .iter()
        .map(|r| r.canonicalize().unwrap_or_else(|_| r.clone()))
        .collect();
    let mut kept = ProjectFiles {
        files: Vec::new(),
        no_cwd: Vec::new(),
    };
    for path in files {
        let cwd = match first_cwd(&path)? {
            FirstCwd::NoChain => continue,
            FirstCwd::Missing(line) => {
                kept.no_cwd.push((path, line));
                continue;
            }
            FirstCwd::Cwd(cwd) => PathBuf::from(cwd),
        };
        let cwd = cwd.canonicalize().unwrap_or(cwd);
        if roots.iter().any(|root| cwd.starts_with(root)) {
            kept.files.push(path);
        }
    }
    Ok(kept)
}

/// The role the command gives a kept line, per `design@transcript@transcript-labels-in-code`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Label {
    Owner,
    OwnerAnswer,
    OwnerQueued,
    OwnerCommand,
    /// The owner interrupted the session's turn: a `user` line that carries none of `isMeta`,
    /// `isCompactSummary` and `origin`, with a string `interruptedMessageId` or a whole text that is
    /// one of [`INTERRUPT_TEXTS`].
    OwnerInterrupt,
    /// A command the owner ran with the `!` prefix: a top-level line no field places that opens
    /// with [`SHELL_INPUT_TAG`]. Its output is `Injected("shell-output")`.
    OwnerShell,
    Agent,
    Dispatch,
    /// Injected text, with its kind: a known `origin.kind`, `unknown`, `skill`, `meta`,
    /// `compact-summary`, `compact-boundary`, `model-refusal`, `shell-output`, `command-output` or
    /// `fork-context`.
    Injected(String),
    /// A tool call or its result, with the tool's name where it is known.
    Tool(String),
    Unlabelled,
}

impl Label {
    /// The label as it is printed and as `--role` names it: the first word is the role.
    pub(crate) fn display(&self) -> String {
        match self {
            Label::Owner => "owner".to_string(),
            Label::OwnerAnswer => "owner-answer".to_string(),
            Label::OwnerQueued => "owner-queued".to_string(),
            Label::OwnerCommand => "owner-command".to_string(),
            Label::OwnerInterrupt => "owner-interrupt".to_string(),
            Label::OwnerShell => "owner-shell".to_string(),
            Label::Agent => "agent".to_string(),
            Label::Dispatch => "dispatch".to_string(),
            Label::Injected(kind) => format!("injected {kind}"),
            Label::Tool(name) if name.is_empty() => "tool".to_string(),
            Label::Tool(name) => format!("tool {name}"),
            Label::Unlabelled => "unlabelled".to_string(),
        }
    }

    /// The role alone, the first word of [`Label::display`].
    pub(crate) fn role(&self) -> &'static str {
        match self {
            Label::Owner => "owner",
            Label::OwnerAnswer => "owner-answer",
            Label::OwnerQueued => "owner-queued",
            Label::OwnerCommand => "owner-command",
            Label::OwnerInterrupt => "owner-interrupt",
            Label::OwnerShell => "owner-shell",
            Label::Agent => "agent",
            Label::Dispatch => "dispatch",
            Label::Injected(_) => "injected",
            Label::Tool(_) => "tool",
            Label::Unlabelled => "unlabelled",
        }
    }

    /// An owner's or the agent's text, a dispatch, or a line no row placed is never cut.
    pub(crate) fn never_cut(&self) -> bool {
        matches!(
            self,
            Label::Owner
                | Label::OwnerAnswer
                | Label::OwnerQueued
                | Label::OwnerCommand
                | Label::OwnerInterrupt
                | Label::OwnerShell
                | Label::Agent
                | Label::Dispatch
                | Label::Unlabelled
        )
    }

    /// The lines `list --containing` and `show --from` read: what the owner typed.
    pub(crate) fn owner_typed(&self) -> bool {
        matches!(self, Label::Owner | Label::OwnerQueued)
    }
}

/// One kept line, or one kept content item of a line.
#[derive(Clone, Debug)]
pub(crate) struct Entry {
    /// The line's number in its file, from 1.
    pub(crate) line: usize,
    pub(crate) label: Label,
    pub(crate) timestamp: String,
    pub(crate) text: String,
    /// For a tool call, its id; for a tool result, the id of the call it answers.
    pub(crate) tool_use_id: Option<String>,
}

/// What a run read and did not recognise, for the header.
#[derive(Default)]
pub(crate) struct Inventory {
    pub(crate) files: usize,
    pub(crate) versions: BTreeSet<String>,
    /// `(category, value)` to its count and its first location, `file:line`.
    pub(crate) unknown: BTreeMap<(String, String), (usize, String)>,
    pub(crate) unlabelled: usize,
    pub(crate) malformed: Vec<String>,
    pub(crate) repeated_uuids: usize,
    /// Each text the harness cut before writing it, in reading order.
    pub(crate) cuts: Vec<Cut>,
}

/// A text the harness cut before writing it into the transcript, found by the marker it writes at
/// the cut: the location of its line, `file:line`, the count of characters left out, and the file
/// the marker names as holding the whole, where it names one.
pub(crate) struct Cut {
    pub(crate) at: String,
    pub(crate) chars: String,
    pub(crate) whole_in: Option<String>,
}

/// The marker the harness writes where it cuts a long result of a subagent or a workflow, on a line
/// of its own: `... (truncated <N> chars)`, or with `, full result in <path>` before the parenthesis
/// closes, per `design@transcript@cut-text-reported`. It is read only in a text whose fields already
/// label it `injected` or `tool`, and it chooses no label, per
/// `design@transcript@transcript-labels-in-code`; the line break and the three dots before it keep
/// out a quotation of the marker, which an agent writes inside a sentence.
static CUT_MARKER: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"\n\.\.\. \(truncated (\d+) chars(?:, full result in ([^)\n]+))?\)")
        .expect("the pattern parses")
});

impl Inventory {
    /// Count a top-level file that a run over one project leaves out, since its first chain line,
    /// at `line`, holds no `cwd`: a value the reader does not know, so a gap.
    pub(crate) fn note_no_cwd(&mut self, name: &str, line: usize) {
        self.note_unknown("first chain line", "no cwd", format!("{name}:{line}"));
    }

    fn note_unknown(&mut self, category: &str, value: &str, at: String) {
        let slot = self
            .unknown
            .entry((category.to_string(), value.to_string()))
            .or_insert((0, at));
        slot.0 += 1;
    }

    /// Count an `origin` the labels cannot read: a `kind` outside the known set, an object with no
    /// `kind`, or a value that is no object, named by its JSON so the value a release wrote shows.
    fn note_origin(&mut self, field: &str, origin: Option<&Value>, at: &str) {
        match origin {
            None => {}
            Some(Value::Object(_)) => {
                let kind = origin_kind(origin).expect("an object gives a kind");
                if !KNOWN_ORIGIN_KINDS.contains(&kind) {
                    self.note_unknown(&format!("{field}.kind"), kind, at.to_string());
                }
            }
            Some(other) => {
                self.note_unknown(field, &format!("not an object: {other}"), at.to_string())
            }
        }
    }

    /// The header's lines, each as the JSON object `--json` prints and the text otherwise: what was
    /// read, each value not recognised, each text the harness cut, and, where any gap is named,
    /// the request to report it.
    pub(crate) fn header_lines(&self) -> Vec<(Value, String)> {
        let versions: Vec<&str> = self.versions.iter().map(String::as_str).collect();
        let mut lines: Vec<(serde_json::Value, String)> = vec![(
            json!({"header": "read", "files": self.files, "versions": versions}),
            format!(
                "# files read: {}; harness versions: {}",
                self.files,
                if versions.is_empty() {
                    "none".to_string()
                } else {
                    versions.join(", ")
                }
            ),
        )];
        for ((category, value), (count, at)) in &self.unknown {
            lines.push((
            json!({"header": "unrecognised", "category": category, "value": value, "count": count, "first": at}),
            format!("# unrecognised {category} {value:?}: {count}, first at {at}"),
        ));
        }
        if self.unlabelled > 0 {
            lines.push((
                json!({"header": "unlabelled", "count": self.unlabelled}),
                format!("# unlabelled lines: {}", self.unlabelled),
            ));
        }
        for at in &self.malformed {
            lines.push((
                json!({"header": "not-json", "at": at}),
                format!("# not JSON: {at}"),
            ));
        }
        if self.repeated_uuids > 0 {
            lines.push((
                json!({"header": "repeated-uuid", "count": self.repeated_uuids}),
                format!(
                    "# lines skipped for a repeated uuid: {}",
                    self.repeated_uuids
                ),
            ));
        }
        for cut in &self.cuts {
            lines.push((
            json!({"header": "cut", "at": cut.at, "chars": cut.chars, "whole_in": cut.whole_in}),
            match &cut.whole_in {
                Some(path) => format!(
                    "# cut by the harness: {}, {} chars not in the transcript, the whole in {path}",
                    cut.at, cut.chars
                ),
                None => format!(
                    "# cut by the harness: {}, {} chars not in the transcript, and no file named",
                    cut.at, cut.chars
                ),
            },
        ));
        }
        if self.has_gap() {
            // The repeated uuids above are no gap, so the request names what it covers.
            let gap =
                "each unrecognised value, unlabelled line and line that is no JSON above is a gap \
                   of this command: report it as a finding on the checker in the retrospective";
            lines.push((json!({"header": "gap", "text": gap}), format!("# {gap}")));
        }
        lines
    }

    /// Whether the header ends with the line asking for the gap to be reported.
    pub(crate) fn has_gap(&self) -> bool {
        !self.unknown.is_empty() || self.unlabelled > 0 || !self.malformed.is_empty()
    }

    /// Count one file's lines against the known sets. Every line is counted, chain or not, so a
    /// value outside a set is reported whatever session it sits in.
    fn survey(&mut self, file: &File, name: &str, in_subagent: bool) {
        self.files += 1;
        let mut forks_seen = 0;
        for (i, raw) in file.lines.iter().enumerate() {
            let at = format!("{name}:{}", i + 1);
            let value = match raw {
                Raw::Json(value) => value,
                Raw::Malformed => {
                    self.malformed.push(at);
                    continue;
                }
                Raw::Blank => continue,
            };
            if let Some(version) = str_field(value, "version") {
                self.versions.insert(version.to_string());
            }
            let Some(kind) = str_field(value, "type") else {
                self.note_unknown("type", "(none)", at);
                continue;
            };
            match kind {
                "user" | "assistant" => {
                    let known: &[&str] = if kind == "user" {
                        KNOWN_USER_ITEMS
                    } else {
                        KNOWN_ASSISTANT_ITEMS
                    };
                    // The shapes the labels read: a `user` line's content a string or a list, an
                    // `assistant` line's a list, each `text` item with its `text`. Another shape
                    // would print as nothing, so it is counted.
                    match value.get("message").and_then(|m| m.get("content")) {
                        Some(Value::Array(items)) => {
                            for item in items {
                                let item_type = str_field(item, "type").unwrap_or("(none)");
                                if !known.contains(&item_type) {
                                    self.note_unknown(
                                        &format!("{kind} content item"),
                                        item_type,
                                        at.clone(),
                                    );
                                } else if item_type == "text" && str_field(item, "text").is_none() {
                                    self.note_unknown(
                                        &format!("{kind} content item shape"),
                                        "text with no text",
                                        at.clone(),
                                    );
                                }
                            }
                        }
                        Some(Value::String(_)) if kind == "user" => {}
                        _ => self.note_unknown(
                            &format!("{kind} content shape"),
                            "neither a list nor a string the line takes",
                            at.clone(),
                        ),
                    }
                    if kind == "user" {
                        self.note_origin("origin", value.get("origin"), &at);
                    }
                }
                "attachment" => {
                    let attachment = value.get("attachment");
                    let attachment_type = attachment
                        .and_then(|a| str_field(a, "type"))
                        .unwrap_or("(none)");
                    if !KNOWN_ATTACHMENT_TYPES.contains(&attachment_type) {
                        self.note_unknown("attachment.type", attachment_type, at.clone());
                    }
                    if attachment_type == "queued_command" {
                        self.note_origin(
                            "attachment.origin",
                            attachment.and_then(|a| a.get("origin")),
                            &at,
                        );
                        if attachment.and_then(|a| a.get("prompt")).is_none() {
                            self.note_unknown("queued_command shape", "no prompt", at.clone());
                        }
                    }
                }
                "system" => {
                    let subtype = str_field(value, "subtype").unwrap_or("(none)");
                    if !KNOWN_SYSTEM_SUBTYPES.contains(&subtype) {
                        self.note_unknown("system subtype", subtype, at.clone());
                    }
                }
                FORK_CONTEXT_TYPE => {
                    // Only a subagent file's first such line is shown, by `fork_context`; one
                    // elsewhere would be read by nothing, so it is reported, and so is a shape the
                    // entry cannot print.
                    if !in_subagent {
                        self.note_unknown("fork-context-ref place", "outside a subagent file", at);
                    } else if forks_seen > 0 {
                        self.note_unknown(
                            "fork-context-ref place",
                            "not the first in its file",
                            at,
                        );
                    } else if parent_last_uuid(value).is_none() {
                        self.note_unknown("fork-context-ref shape", "no parentLastUuid", at);
                    } else if value
                        .get("contextLength")
                        .is_some_and(|n| n.as_u64().is_none())
                    {
                        self.note_unknown(
                            "fork-context-ref shape",
                            "contextLength not a count",
                            at,
                        );
                    }
                    forks_seen += 1;
                }
                other if KNOWN_METADATA_TYPES.contains(&other) => {}
                other => self.note_unknown("type", other, at),
            }
        }
    }
}

/// The text a command's tags are read in: a `local_command` line's `content`, or a `user` line's
/// message text.
fn line_text(value: &Value) -> String {
    match str_field(value, "type") {
        Some("system") => str_field(value, "content").unwrap_or("").to_string(),
        Some("user") => message_text(value),
        _ => String::new(),
    }
}

/// A command as `/name args`, from its `<command-name>` and `<command-args>` tags; the text whole
/// where it holds no name.
fn command_text(text: &str) -> String {
    let inside = |tag: &str| -> Option<&str> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = text.find(&open)? + open.len();
        let end = start + text[start..].find(&close)?;
        Some(text[start..end].trim())
    };
    match (inside("command-name"), inside("command-args")) {
        (Some(name), Some(args)) if !args.is_empty() => format!("{name} {args}"),
        (Some(name), _) => name.to_string(),
        (None, _) => text.to_string(),
    }
}

/// An attached subagent file.
pub(crate) struct Subagent {
    pub(crate) file: File,
    pub(crate) agent_id: String,
    pub(crate) agent_type: String,
    pub(crate) description: String,
    tool_use_id: Option<String>,
    pub(crate) entries: Vec<Entry>,
}

/// One session: a run of chain lines of one top-level file, with its subagent files.
pub(crate) struct Session {
    pub(crate) session_id: String,
    pub(crate) handle: String,
    /// The top-level file, shared by every session cut from it.
    pub(crate) file: usize,
    /// The session's place among the sessions of its file, from 0.
    pub(crate) ordinal: usize,
    pub(crate) entries: Vec<Entry>,
    pub(crate) subagents: Vec<Subagent>,
    pub(crate) branches: Vec<String>,
    pub(crate) first_timestamp: String,
    pub(crate) last_timestamp: String,
    pub(crate) compactions: usize,
}

/// Every session of a set of top-level files, with the inventory of what was read.
pub(crate) struct Corpus {
    pub(crate) dir: PathBuf,
    pub(crate) files: Vec<File>,
    pub(crate) sessions: Vec<Session>,
    pub(crate) inventory: Inventory,
    /// The subagent files read whose top-level file holds no session to attach them to: surveyed
    /// and labelled, and held here so the line-kind inventory counts them.
    pub(crate) unattached: Vec<Subagent>,
    /// How many sessions of each handle's 8 characters were cut so far, over every file read: a
    /// handle's `#n` counts across files and across `sessionId`s, so every handle names one
    /// session.
    ids: HashMap<String, usize>,
    /// Every handle given so far, over every file read.
    handles: HashSet<String>,
}

impl Corpus {
    /// Read `paths`, top-level files under `dir`, and their subagent files when `subagents`.
    pub(crate) fn read(dir: &Path, paths: &[PathBuf], subagents: bool) -> Result<Corpus, String> {
        let files = paths
            .iter()
            .map(|p| File::read(p))
            .collect::<Result<Vec<_>, _>>()?;
        let mut corpus = Corpus::of_files(dir, files);
        if subagents {
            corpus.attach_subagents()?;
        }
        Ok(corpus)
    }

    /// The corpus of top-level files already read, under `dir`, each surveyed and cut into
    /// sessions in order, and no subagent file attached.
    pub(crate) fn of_files(dir: &Path, files: Vec<File>) -> Corpus {
        let mut corpus = Corpus {
            dir: dir.to_path_buf(),
            files: Vec::new(),
            sessions: Vec::new(),
            inventory: Inventory::default(),
            unattached: Vec::new(),
            ids: HashMap::new(),
            handles: HashSet::new(),
        };
        for file in files {
            let name = corpus.name(&file.path);
            corpus.inventory.survey(&file, &name, false);
            let index = corpus.files.len();
            corpus.files.push(file);
            corpus.cut(index);
        }
        corpus
    }

    /// A path relative to the transcript directory, as the header names a location.
    pub(crate) fn name(&self, path: &Path) -> String {
        path.strip_prefix(&self.dir)
            .unwrap_or(path)
            .display()
            .to_string()
    }

    /// Cut a top-level file into sessions, label them, and count what the labels could not place.
    fn cut(&mut self, index: usize) {
        let name = self.name(&self.files[index].path);
        let file = &self.files[index];
        let chain = file.chain();
        let mut runs: Vec<(String, Vec<usize>)> = Vec::new();
        let mut previous: Option<String> = None;
        for &i in &chain {
            let value = file.json(i).expect("a chain line is JSON");
            let session_id = str_field(value, "sessionId").unwrap_or("").to_string();
            // The cut of `design@transcript@transcript-session-split`. Each line is judged on its own, so
            // a line written before its parent cuts nothing.
            let root = value.get("parentUuid").is_none_or(Value::is_null)
                && str_field(value, "subtype") != Some("compact_boundary");
            let opens = match &previous {
                None => true,
                Some(prev) => *prev != session_id || root,
            };
            if opens {
                runs.push((session_id.clone(), Vec::new()));
            }
            runs.last_mut().expect("a run is open").1.push(i);
            previous = Some(session_id);
        }
        for (ordinal, (session_id, lines)) in runs.into_iter().enumerate() {
            let short: String = session_id.chars().take(8).collect();
            // Counted per handle, so two `sessionId`s sharing their first 8 characters give two
            // handles, and a number already taken, as by a `sessionId` that holds `#`, is passed
            // over, so every handle names one session, per
            // `design@transcript@one-handle-per-session`.
            let n = self.ids.entry(short.clone()).or_insert(0);
            let handle = loop {
                *n += 1;
                let handle = if *n == 1 {
                    short.clone()
                } else {
                    format!("{short}#{n}")
                };
                if self.handles.insert(handle.clone()) {
                    break handle;
                }
            };
            let mut ctx = Labelling {
                file,
                name: &name,
                in_subagent: false,
                inventory: &mut self.inventory,
                shell_inputs: HashSet::new(),
                commands: HashSet::new(),
            };
            let entries = ctx.label(&lines);
            let mut branches: Vec<String> = Vec::new();
            let mut compactions = 0;
            for &i in &lines {
                let value = file.json(i).expect("a chain line is JSON");
                if let Some(branch) = str_field(value, "gitBranch") {
                    if !branches.iter().any(|b| b == branch) {
                        branches.push(branch.to_string());
                    }
                }
                if str_field(value, "subtype") == Some("compact_boundary") {
                    compactions += 1;
                }
            }
            let timestamp = |i: &usize| {
                file.json(*i)
                    .and_then(|v| str_field(v, "timestamp"))
                    .unwrap_or("")
                    .to_string()
            };
            let first_timestamp = lines.first().map(timestamp).unwrap_or_default();
            let last_timestamp = lines.last().map(timestamp).unwrap_or_default();
            self.sessions.push(Session {
                session_id,
                handle,
                file: index,
                ordinal,
                entries,
                subagents: Vec::new(),
                branches,
                first_timestamp,
                last_timestamp,
                compactions,
            });
        }
    }

    /// Attach each subagent file under `<file stem>/subagents/` of each top-level file read.
    ///
    /// A subagent file goes to a session of its parent file whose `sessionId` its lines carry:
    /// where that id holds one session, to it; where several, to the one holding the `tool_use`
    /// item its `meta.json` names, searched through the top-level file and the other subagent
    /// files; otherwise to the last.
    fn attach_subagents(&mut self) -> Result<(), String> {
        for index in 0..self.files.len() {
            let path = self.files[index].path.clone();
            let Some(stem) = path.file_stem() else {
                continue;
            };
            let base = path.with_file_name(stem).join("subagents");
            let mut paths = Vec::new();
            collect_agent_files(&base, &mut paths)?;
            paths.sort();
            let mut agents = Vec::new();
            for agent_path in paths {
                let file = File::read(&agent_path)?;
                let name = self.name(&agent_path);
                self.inventory.survey(&file, &name, true);
                let meta = std::fs::read(agent_path.with_extension("meta.json"))
                    .ok()
                    .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok());
                let meta_str = |key: &str| {
                    meta.as_ref()
                        .and_then(|m| str_field(m, key))
                        .map(str::to_string)
                };
                let agent_id = agent_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .trim_start_matches("agent-")
                    .to_string();
                let chain = file.chain();
                let session_id = chain
                    .first()
                    .and_then(|&i| file.json(i))
                    .and_then(|v| str_field(v, "sessionId"))
                    .unwrap_or("")
                    .to_string();
                let mut ctx = Labelling {
                    file: &file,
                    name: &name,
                    in_subagent: true,
                    inventory: &mut self.inventory,
                    shell_inputs: HashSet::new(),
                    commands: HashSet::new(),
                };
                let mut entries = ctx.label(&chain);
                if let Some(entry) = fork_context(&file, &self.files[index]) {
                    // A fork's file opens with a copy of the parent's call that dispatched it and
                    // of that call's result, which the parent's file shows: shown again here, they
                    // would read as a call the fork made.
                    if let Some(id) = meta_str("toolUseId") {
                        entries.retain(|e| {
                            !(matches!(e.label, Label::Tool(_))
                                && e.tool_use_id.as_deref() == Some(id.as_str()))
                        });
                    }
                    entries.insert(0, entry);
                }
                agents.push((
                    session_id,
                    Subagent {
                        agent_id,
                        agent_type: meta_str("agentType").unwrap_or_default(),
                        description: meta_str("description").unwrap_or_default(),
                        tool_use_id: meta_str("toolUseId"),
                        entries,
                        file,
                    },
                ));
            }
            // Which session holds each tool call, the top-level file's and every subagent's.
            let mut call_owner: HashMap<String, usize> = HashMap::new();
            for (s, session) in self.sessions.iter().enumerate() {
                if session.file != index {
                    continue;
                }
                for entry in &session.entries {
                    if let (Label::Tool(_), Some(id)) = (&entry.label, &entry.tool_use_id) {
                        call_owner.insert(id.clone(), s);
                    }
                }
            }
            let mut placed: Vec<(usize, Subagent)> = Vec::new();
            let mut pending = agents;
            // A nested agent is dispatched from another subagent's file, so placing it may wait
            // on that agent's placement; each pass places at least one, or the rest go last.
            loop {
                let mut progress = false;
                let mut rest = Vec::new();
                for (session_id, agent) in pending {
                    let candidates: Vec<usize> = (0..self.sessions.len())
                        .filter(|&s| {
                            self.sessions[s].file == index
                                && self.sessions[s].session_id == session_id
                        })
                        .collect();
                    let target = match candidates.as_slice() {
                        [] => None,
                        [one] => Some(*one),
                        _ => agent
                            .tool_use_id
                            .as_ref()
                            .and_then(|id| call_owner.get(id).copied()),
                    };
                    match target {
                        Some(s) => {
                            for entry in &agent.entries {
                                if let (Label::Tool(_), Some(id)) =
                                    (&entry.label, &entry.tool_use_id)
                                {
                                    call_owner.insert(id.clone(), s);
                                }
                            }
                            placed.push((s, agent));
                            progress = true;
                        }
                        None => rest.push((session_id, agent)),
                    }
                }
                pending = rest;
                if pending.is_empty() || !progress {
                    break;
                }
            }
            for (session_id, agent) in pending {
                let last = (0..self.sessions.len()).rev().find(|&s| {
                    self.sessions[s].file == index && self.sessions[s].session_id == session_id
                });
                let last = last.or_else(|| {
                    (0..self.sessions.len())
                        .rev()
                        .find(|&s| self.sessions[s].file == index)
                });
                match last {
                    Some(s) => placed.push((s, agent)),
                    None => self.unattached.push(agent),
                }
            }
            for (s, agent) in placed {
                self.sessions[s].subagents.push(agent);
            }
        }
        Ok(())
    }
}

fn collect_agent_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(format!("cannot read {}: {e}", dir.display())),
    };
    for entry in entries {
        let entry = entry.map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_agent_files(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "jsonl")
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("agent-"))
        {
            out.push(path);
        }
    }
    Ok(())
}

/// The entry of a forked subagent's `fork-context-ref` line, which names the line of the parent's
/// file up to which the fork inherits its context: the fork saw that context, and read nothing of
/// it in its own file. `None` for a file that holds no such line.
///
/// The line names the parent's line by its `uuid`, looked up in the top-level file; a `uuid` that file
/// does not hold, such as a nested fork's whose parent line sits in another subagent's file, is
/// printed as it is, with the words that the parent's file, the top-level one, does not hold it.
fn fork_context(agent: &File, parent: &File) -> Option<Entry> {
    let index = (0..agent.lines.len())
        .find(|&i| agent.json(i).and_then(|v| str_field(v, "type")) == Some(FORK_CONTEXT_TYPE))?;
    let value = agent.json(index).expect("the line is JSON");
    // A line with no `parentLastUuid` is reported by the survey, and printed as naming no line.
    let up_to = match parent_last_uuid(value) {
        None => "a line it does not name".to_string(),
        Some(uuid) => match (0..parent.lines.len())
            .find(|&i| parent.json(i).and_then(|v| str_field(v, "uuid")) == Some(uuid))
        {
            Some(i) => format!("L{}", i + 1),
            None => format!("the line {uuid:?}, which the parent's file does not hold"),
        },
    };
    // What the field counts is not established, so it is printed under its own name.
    let length = match value.get("contextLength").and_then(Value::as_u64) {
        Some(n) => format!(" (contextLength {n})"),
        None => String::new(),
    };
    // The line carries no timestamp: the fork's first chain line gives it.
    let timestamp = agent
        .chain()
        .first()
        .and_then(|&i| agent.json(i))
        .and_then(|v| str_field(v, "timestamp"))
        .unwrap_or("")
        .to_string();
    Some(Entry {
        line: index + 1,
        label: Label::Injected("fork-context".to_string()),
        timestamp,
        text: format!("a fork: it inherits its parent's context up to {up_to}{length}"),
        tool_use_id: None,
    })
}

/// The `parentLastUuid` of a `fork-context-ref` line, where it names one: an empty string names none.
fn parent_last_uuid(value: &Value) -> Option<&str> {
    str_field(value, "parentLastUuid").filter(|u| !u.is_empty())
}

/// The labelling of one file's chain lines.
struct Labelling<'a> {
    file: &'a File,
    /// The file's name, as the header names a location.
    name: &'a str,
    in_subagent: bool,
    inventory: &'a mut Inventory,
    /// The `uuid`s of the lines labelled `owner-shell` so far: a `<bash-stdout>` line is a shell
    /// command's output only as the child of one, so a pasted tag elsewhere places nothing.
    shell_inputs: HashSet<String>,
    /// The `uuid`s of the lines labelled `owner-command` from their [`COMMAND_NAME_TAG`] so far: a
    /// [`COMMAND_OUTPUT_TAG`] line is a command's output only as the child of one.
    commands: HashSet<String>,
}

impl Labelling<'_> {
    /// The entries of `lines`, in file order: the first row whose condition a line meets gives its
    /// label, the rows being this function's and `Labelling::user`'s branches, in order.
    fn label(&mut self, lines: &[usize]) -> Vec<Entry> {
        // The parents of the `local_command` lines that carry `commandRun`, by `uuid` over the
        // run: each is that command's echo, printed as the `owner-command` line that follows it. A
        // `local_command` echo is skipped; a `user` echo is consumed only where no field and no text
        // row but the command tag's places it, so a parent that is a message, the owner's or the
        // agent's, or an act a row places, such as an interrupt, is never dropped.
        let consumed: HashSet<&str> = lines
            .iter()
            .filter_map(|&i| self.file.json(i))
            .filter(|v| {
                str_field(v, "subtype") == Some("local_command") && v.get("commandRun").is_some()
            })
            .filter_map(|v| str_field(v, "parentUuid"))
            .collect();
        let mut seen: HashSet<String> = HashSet::new();
        let mut entries = Vec::new();
        for &i in lines {
            let value = self.file.json(i).expect("a chain line is JSON");
            if let Some(uuid) = str_field(value, "uuid") {
                if !seen.insert(uuid.to_string()) {
                    self.inventory.repeated_uuids += 1;
                    continue;
                }
                if consumed.contains(uuid)
                    && str_field(value, "type") == Some("user")
                    && !holds_tool_results(value)
                    && value.get("isMeta").is_none()
                    && value.get("isCompactSummary").is_none()
                    && value.get("origin").is_none()
                    && !carries_interrupt_id(value)
                    && matches!(
                        self.text_row(value, &message_text(value)),
                        None | Some(Label::OwnerCommand)
                    )
                {
                    continue;
                }
            }
            let line = i + 1;
            let timestamp = str_field(value, "timestamp").unwrap_or("").to_string();
            let entry = |label: Label, text: String, tool_use_id: Option<String>| Entry {
                line,
                label,
                timestamp: timestamp.clone(),
                text,
                tool_use_id,
            };
            match str_field(value, "type").unwrap_or("") {
                "user" => self.user(value, &entry, &mut entries),
                "assistant" => {
                    let items = value
                        .get("message")
                        .and_then(|m| m.get("content"))
                        .and_then(Value::as_array);
                    for item in items.into_iter().flatten() {
                        match str_field(item, "type") {
                            Some("text") => entries.push(entry(
                                Label::Agent,
                                str_field(item, "text").unwrap_or("").to_string(),
                                None,
                            )),
                            Some("tool_use") => {
                                let name = str_field(item, "name").unwrap_or("").to_string();
                                let input =
                                    item.get("input").map(Value::to_string).unwrap_or_default();
                                entries.push(entry(
                                    Label::Tool(name.clone()),
                                    format!("{name} {input}"),
                                    str_field(item, "id").map(str::to_string),
                                ));
                            }
                            // A thinking item is dropped; any other was counted by the survey.
                            _ => {}
                        }
                    }
                }
                "attachment" => {
                    let Some(attachment) = value.get("attachment") else {
                        continue;
                    };
                    if str_field(attachment, "type") != Some("queued_command") {
                        continue;
                    }
                    let prompt = text_of(attachment.get("prompt"));
                    let label = match origin_kind(attachment.get("origin")) {
                        Some("human") => Label::OwnerQueued,
                        Some(kind) if KNOWN_ORIGIN_KINDS.contains(&kind) => {
                            Label::Injected(kind.to_string())
                        }
                        Some(_) => Label::Injected("unknown".to_string()),
                        None if attachment.get("isMeta").and_then(Value::as_bool) == Some(true) => {
                            Label::Injected("meta".to_string())
                        }
                        None => self.unlabelled(),
                    };
                    entries.push(entry(label, prompt, None));
                }
                "system" => match str_field(value, "subtype") {
                    Some("local_command") if value.get("commandRun").is_none() => {
                        // A command the harness wrote with no `commandRun`, as `/restart` does: its
                        // subtype places it, and its leading tag names which command, in a top-level
                        // file, where the owner types. An echo that a `commandRun` line names is
                        // printed by that line. Any other such line prints nothing, so it is
                        // reported here, where the decision that it prints nothing is made: a release
                        // that dropped `commandRun` would otherwise lose the command in silence.
                        let uuid = str_field(value, "uuid").unwrap_or("");
                        let text = line_text(value);
                        if consumed.contains(uuid) {
                            continue;
                        }
                        if !self.in_subagent && text.starts_with(COMMAND_NAME_TAG) {
                            self.commands.insert(uuid.to_string());
                            entries.push(entry(Label::OwnerCommand, command_text(&text), None));
                        } else if !self.in_subagent
                            && text.starts_with(COMMAND_OUTPUT_TAG)
                            && str_field(value, "parentUuid")
                                .is_some_and(|p| self.commands.contains(p))
                        {
                            entries.push(entry(
                                Label::Injected("command-output".to_string()),
                                text,
                                None,
                            ));
                        } else {
                            self.inventory.note_unknown(
                                "system local_command shape",
                                "no commandRun",
                                format!("{}:{line}", self.name),
                            );
                        }
                    }
                    Some("local_command") => {
                        if let Some(run) = value.get("commandRun") {
                            let command = str_field(run, "command").unwrap_or("");
                            let args = str_field(run, "args").unwrap_or("");
                            let text = if args.is_empty() {
                                format!("/{command}")
                            } else {
                                format!("/{command} {args}")
                            };
                            entries.push(entry(Label::OwnerCommand, text, None));
                        }
                    }
                    Some("model_refusal_no_fallback") => {
                        let category = str_field(value, "apiRefusalCategory").unwrap_or("unknown");
                        entries.push(entry(
                            Label::Injected("model-refusal".to_string()),
                            format!("model refusal, category {category}"),
                            None,
                        ));
                    }
                    Some("compact_boundary") => {
                        let trigger = value
                            .get("compactMetadata")
                            .and_then(|m| str_field(m, "trigger"))
                            .unwrap_or("unknown");
                        entries.push(entry(
                            Label::Injected("compact-boundary".to_string()),
                            format!("compaction, trigger {trigger}"),
                            None,
                        ));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // A tool result names its call by id: it takes the call's tool name, where the call is in
        // the same run.
        let names: HashMap<String, String> = entries
            .iter()
            .filter_map(|e| match (&e.label, &e.tool_use_id) {
                (Label::Tool(name), Some(id)) if !name.is_empty() => {
                    Some((id.clone(), name.clone()))
                }
                _ => None,
            })
            .collect();
        for e in &mut entries {
            if let (Label::Tool(name), Some(id)) = (&mut e.label, &e.tool_use_id) {
                if name.is_empty() {
                    if let Some(called) = names.get(id) {
                        *name = called.clone();
                    }
                }
            }
        }
        for e in &entries {
            // A line its fields label injected or tool, per `design@transcript@cut-text-reported`:
            // the output of a command the owner ran, a shell command's or one a command tag names,
            // holds what the owner's command printed, a cut marker among it included, and no cut of
            // the harness.
            let placed_by_fields = match &e.label {
                Label::Tool(_) => true,
                Label::Injected(kind) => kind != "shell-output" && kind != "command-output",
                _ => false,
            };
            if !placed_by_fields {
                continue;
            }
            for found in CUT_MARKER.captures_iter(&e.text) {
                self.inventory.cuts.push(Cut {
                    at: format!("{}:{}", self.name, e.line),
                    chars: found[1].to_string(),
                    whole_in: found.get(2).map(|m| m.as_str().to_string()),
                });
            }
        }
        entries
    }

    /// The rows of the mapping table for a `user` line.
    fn user(
        &mut self,
        value: &Value,
        entry: &dyn Fn(Label, String, Option<String>) -> Entry,
        entries: &mut Vec<Entry>,
    ) {
        let content = value.get("message").and_then(|m| m.get("content"));
        let items = content.and_then(Value::as_array);
        let results: Vec<&Value> = items
            .into_iter()
            .flatten()
            .filter(|item| str_field(item, "type") == Some("tool_result"))
            .collect();
        if !results.is_empty() {
            let answers = value
                .get("toolUseResult")
                .filter(|r| r.get("questions").is_some() && r.get("answers").is_some());
            if let Some(result) = answers {
                entries.push(entry(Label::OwnerAnswer, render_answers(result), None));
            } else {
                for item in &results {
                    entries.push(entry(
                        Label::Tool(String::new()),
                        text_of(item.get("content")),
                        str_field(item, "tool_use_id").map(str::to_string),
                    ));
                }
            }
            // The text items of a line that also holds tool results are the line's own message.
            let texts: Vec<&Value> = items
                .into_iter()
                .flatten()
                .filter(|item| str_field(item, "type") == Some("text"))
                .collect();
            if texts.is_empty() {
                return;
            }
        }
        let text = message_text(value);
        let is_meta = value.get("isMeta").and_then(Value::as_bool) == Some(true);
        let origin = origin_kind(value.get("origin"));
        let compact = value.get("isCompactSummary").and_then(Value::as_bool) == Some(true);
        // The fields are tested by presence, as the consumption above tests them.
        let fieldless = value.get("isMeta").is_none()
            && value.get("isCompactSummary").is_none()
            && value.get("origin").is_none();
        // An interrupt is placed by its `interruptedMessageId`, or, on a line no field places, by its
        // whole text, as `design@transcript@transcript-labels-in-code` measures, and this row comes
        // first so that one in a subagent file is the owner's, not read as the dispatcher's text by
        // the row after it.
        let text_row = if fieldless {
            self.text_row(value, &text)
        } else {
            None
        };
        let label = if fieldless && carries_interrupt_id(value) {
            Label::OwnerInterrupt
        } else if let Some(label) = text_row {
            label
        } else if self.in_subagent && fieldless {
            // In a subagent file, a `user` line that carries none of the three fields is its
            // dispatcher's or its workflow's text, as `design@transcript@transcript-labels-in-code`
            // measures.
            Label::Dispatch
        } else if compact {
            Label::Injected("compact-summary".to_string())
        } else if let (true, Some(kind)) = (is_meta, origin) {
            if KNOWN_ORIGIN_KINDS.contains(&kind) {
                Label::Injected(kind.to_string())
            } else {
                Label::Injected("unknown".to_string())
            }
        } else if is_meta {
            // The one text this row reads, for a line its fields already mark injected: it chooses a
            // kind, never a side.
            if text.starts_with(SKILL_BODY_PREFIX) {
                Label::Injected("skill".to_string())
            } else {
                Label::Injected("meta".to_string())
            }
        } else {
            match origin {
                Some("human") => Label::Owner,
                Some(kind) if KNOWN_ORIGIN_KINDS.contains(&kind) => {
                    Label::Injected(kind.to_string())
                }
                Some(_) => Label::Injected("unknown".to_string()),
                None => self.unlabelled(),
            }
        };
        if label == Label::OwnerShell {
            if let Some(uuid) = str_field(value, "uuid") {
                self.shell_inputs.insert(uuid.to_string());
            }
        }
        let text = if label == Label::OwnerCommand {
            if let Some(uuid) = str_field(value, "uuid") {
                self.commands.insert(uuid.to_string());
            }
            command_text(&text)
        } else {
            text
        };
        entries.push(entry(label, text, None));
    }

    /// The label a text row gives a `user` line that no field places, or `None`: its whole text one
    /// of [`INTERRUPT_TEXTS`]; in a top-level file, a text that opens with [`SHELL_INPUT_TAG`], or
    /// with [`SHELL_OUTPUT_TAG`] on the child of a line labelled `owner-shell`, or that opens with
    /// [`COMMAND_NAME_TAG`], or with [`COMMAND_OUTPUT_TAG`] on the child of a line labelled
    /// `owner-command` from its tag.
    ///
    /// The rows read only a line no field places, so a line its fields give to a side keeps it: a
    /// whole text the harness writes, or a tag it writes first, never a substring, per
    /// `design@transcript@transcript-labels-in-code`. A shell line is the owner's in a top-level file
    /// alone, where the owner types; in a subagent file a field-less line that opens with the tag
    /// stays its dispatcher's.
    fn text_row(&self, value: &Value, text: &str) -> Option<Label> {
        if INTERRUPT_TEXTS.contains(&text) {
            Some(Label::OwnerInterrupt)
        } else if self.in_subagent {
            None
        } else if text.starts_with(SHELL_INPUT_TAG) {
            Some(Label::OwnerShell)
        } else if text.starts_with(SHELL_OUTPUT_TAG)
            && str_field(value, "parentUuid").is_some_and(|p| self.shell_inputs.contains(p))
        {
            Some(Label::Injected("shell-output".to_string()))
        } else if text.starts_with(COMMAND_NAME_TAG) {
            Some(Label::OwnerCommand)
        } else if text.starts_with(COMMAND_OUTPUT_TAG)
            && str_field(value, "parentUuid").is_some_and(|p| self.commands.contains(p))
        {
            Some(Label::Injected("command-output".to_string()))
        } else {
            None
        }
    }

    /// A line no row places: `unlabelled` and counted.
    fn unlabelled(&mut self) -> Label {
        self.inventory.unlabelled += 1;
        Label::Unlabelled
    }
}

/// Whether a line carries a string `interruptedMessageId`: a null value is no id, since the harness
/// writes null for a field it leaves empty.
pub(crate) fn carries_interrupt_id(value: &Value) -> bool {
    value
        .get("interruptedMessageId")
        .and_then(Value::as_str)
        .is_some()
}

/// The text of a `user` line's message: its content string, or its `text` items joined by line
/// breaks.
fn message_text(value: &Value) -> String {
    match value.get("message").and_then(|m| m.get("content")) {
        Some(Value::Array(items)) => items
            .iter()
            .filter(|item| str_field(item, "type") == Some("text"))
            .filter_map(|item| str_field(item, "text"))
            .collect::<Vec<_>>()
            .join("\n"),
        other => text_of(other),
    }
}

/// The text of a content value: a string, or the `text` items of a list joined by line breaks.
fn text_of(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| match item {
                Value::String(s) => Some(s.clone()),
                other => str_field(other, "text").map(str::to_string),
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Some(Value::Null) | None => String::new(),
        Some(other) => other.to_string(),
    }
}

/// The owner's answers to a question tool: each question with its options, then the answer.
fn render_answers(result: &Value) -> String {
    let mut out = Vec::new();
    let answers = result.get("answers");
    for question in result
        .get("questions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let text = str_field(question, "question").unwrap_or("");
        out.push(format!("Q: {text}"));
        let options: Vec<String> = question
            .get("options")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|o| {
                let label = str_field(o, "label")?;
                Some(match str_field(o, "description") {
                    Some(d) if !d.is_empty() => format!("{label} ({d})"),
                    _ => label.to_string(),
                })
            })
            .collect();
        if !options.is_empty() {
            out.push(format!("options: {}", options.join(" | ")));
        }
        let answer = answers
            .and_then(|a| a.get(text))
            .map(|a| text_of(Some(a)))
            .unwrap_or_default();
        out.push(format!("answer: {answer}"));
    }
    if let Some(annotations) = result.get("annotations").filter(|a| !a.is_null()) {
        out.push(format!("annotations: {annotations}"));
    }
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file read from these lines, as `File::read` parses them.
    fn file(lines: &[&str]) -> File {
        File {
            path: PathBuf::from("t.jsonl"),
            lines: lines
                .iter()
                .map(|l| Raw::Json(serde_json::from_str(l).expect("a fixture line is JSON")))
                .collect(),
        }
    }

    /// The labels of every line of `lines`, in a top-level file or a subagent file.
    fn labels(lines: &[&str], in_subagent: bool) -> Vec<String> {
        let file = file(lines);
        let mut inventory = Inventory::default();
        let mut ctx = Labelling {
            file: &file,
            name: "t.jsonl",
            in_subagent,
            inventory: &mut inventory,
            shell_inputs: HashSet::new(),
            commands: HashSet::new(),
        };
        let all: Vec<usize> = (0..file.lines.len()).collect();
        ctx.label(&all).iter().map(|e| e.label.display()).collect()
    }

    const INTERRUPT: &str = r#"{"type":"user","uuid":"a","message":{"content":[{"type":"text","text":"[Request interrupted by user]"}]}}"#;
    const INTERRUPT_MORE: &str = r#"{"type":"user","uuid":"b","message":{"content":"[Request interrupted by user] and more"}}"#;
    const SHELL_IN: &str =
        r#"{"type":"user","uuid":"c","message":{"content":"<bash-input>ls</bash-input>"}}"#;
    const SHELL_OUT: &str = r#"{"type":"user","uuid":"d","parentUuid":"c","message":{"content":"<bash-stdout>x</bash-stdout>"}}"#;
    const PASTED_OUT: &str = r#"{"type":"user","uuid":"e","parentUuid":"a","message":{"content":"<bash-stdout>pasted</bash-stdout>"}}"#;
    const TYPED_TAG: &str = r#"{"type":"user","uuid":"f","origin":{"kind":"human"},"message":{"content":"<bash-input>typed</bash-input>"}}"#;

    #[test]
    fn an_interrupt_with_no_id_is_placed_by_its_whole_text_only() {
        assert_eq!(
            labels(&[INTERRUPT, INTERRUPT_MORE], false),
            ["owner-interrupt", "unlabelled"]
        );
        assert_eq!(
            labels(&[INTERRUPT, INTERRUPT_MORE], true),
            ["owner-interrupt", "dispatch"]
        );
    }

    #[test]
    fn a_shell_tag_places_a_top_level_line_no_field_places_and_its_output_by_its_parent() {
        assert_eq!(
            labels(
                &[SHELL_IN, SHELL_OUT, INTERRUPT, PASTED_OUT, TYPED_TAG],
                false
            ),
            [
                "owner-shell",
                "injected shell-output",
                "owner-interrupt",
                "unlabelled",
                "owner"
            ]
        );
        assert_eq!(
            labels(&[SHELL_IN, SHELL_OUT], true),
            ["dispatch", "dispatch"]
        );
    }

    #[test]
    fn a_text_row_reads_neither_a_line_its_fields_place_nor_a_text_with_space_around_it() {
        let typed = r#"{"type":"user","uuid":"g","origin":{"kind":"human"},"message":{"content":"[Request interrupted by user]"}}"#;
        let meta = r#"{"type":"user","uuid":"h","isMeta":true,"message":{"content":"[Request interrupted by user]"}}"#;
        let compact = r#"{"type":"user","uuid":"i","isCompactSummary":true,"message":{"content":"[Request interrupted by user]"}}"#;
        let spaced = r#"{"type":"user","uuid":"j","message":{"content":" [Request interrupted by user]\n"}}"#;
        let indented =
            r#"{"type":"user","uuid":"k","message":{"content":" <bash-input>ls</bash-input>"}}"#;
        assert_eq!(
            labels(&[typed, meta, compact, spaced, indented], false),
            [
                "owner",
                "injected meta",
                "injected compact-summary",
                "unlabelled",
                "unlabelled"
            ]
        );
        assert_eq!(labels(&[compact], true), ["injected compact-summary"]);
    }

    #[test]
    fn a_command_s_parent_is_consumed_only_where_no_row_but_the_command_tag_places_it() {
        let command = |uuid: &str, parent: &str| {
            format!(
                r#"{{"type":"system","subtype":"local_command","uuid":"{uuid}","parentUuid":"{parent}","commandRun":{{"command":"clear","args":""}}}}"#
            )
        };
        let echo = r#"{"type":"user","uuid":"z","message":{"content":"<command-name>/clear</command-name>"}}"#;
        let (c1, c2, c3, c4) = (
            command("x1", "a"),
            command("x2", "c"),
            command("x3", "d"),
            command("x4", "z"),
        );
        let with_id = r#"{"type":"user","uuid":"y","interruptedMessageId":"m","message":{"content":"a stop"}}"#;
        let c5 = command("x5", "y");
        assert_eq!(
            labels(
                &[with_id, &c5, INTERRUPT, &c1, SHELL_IN, &c2, SHELL_OUT, &c3, echo, &c4],
                false
            ),
            [
                "owner-interrupt",
                "owner-command",
                "owner-interrupt",
                "owner-command",
                "owner-shell",
                "owner-command",
                "injected shell-output",
                "owner-command",
                "owner-command"
            ]
        );
    }

    /// The claim: the cut marker counts only on its own line, with or without the pointer, and
    /// never inside a sentence that quotes it.
    #[test]
    fn the_cut_marker_is_read_on_its_own_line_only() {
        let found = |text: &str| -> Vec<(String, Option<String>)> {
            CUT_MARKER
                .captures_iter(text)
                .map(|c| (c[1].to_string(), c.get(2).map(|m| m.as_str().to_string())))
                .collect()
        };
        assert_eq!(
            found("head\n... (truncated 12 chars, full result in /a/b.output)</result>"),
            [("12".to_string(), Some("/a/b.output".to_string()))]
        );
        assert_eq!(
            found("head\n... (truncated 7 chars)"),
            [("7".to_string(), None)]
        );
        assert!(found(r#"it was cut ("truncated 138376 chars")"#).is_empty());
        assert!(found("head ... (truncated 7 chars)").is_empty());
    }

    /// The texts of every line of `lines`, in a top-level file, beside their labels.
    fn labelled(lines: &[&str]) -> Vec<(String, String)> {
        let file = file(lines);
        let mut inventory = Inventory::default();
        let mut ctx = Labelling {
            file: &file,
            name: "t.jsonl",
            in_subagent: false,
            inventory: &mut inventory,
            shell_inputs: HashSet::new(),
            commands: HashSet::new(),
        };
        let all: Vec<usize> = (0..file.lines.len()).collect();
        ctx.label(&all)
            .iter()
            .map(|e| (e.label.display(), e.text.clone()))
            .collect()
    }

    const COMPACT: &str = r#"{"type":"user","uuid":"m1","message":{"content":"<command-name>/compact</command-name>\n<command-message>compact</command-message>\n<command-args>keep it short</command-args>"}}"#;
    const COMPACTED: &str = r#"{"type":"user","uuid":"m2","parentUuid":"m1","message":{"content":"<local-command-stdout>Compacted </local-command-stdout>"}}"#;
    const RESTART: &str = r#"{"type":"system","subtype":"local_command","uuid":"r1","content":"<command-name>/restart</command-name>\n<command-message>restart</command-message>\n<command-args></command-args>"}"#;
    const RESTARTED: &str = r#"{"type":"system","subtype":"local_command","uuid":"r2","parentUuid":"r1","content":"<local-command-stdout>Successfully updated</local-command-stdout>"}"#;

    /// The claim: a command the harness wrote with no `commandRun`, on a field-less `user` line or
    /// a `local_command` line, is the owner's, named by its tags, and its output is injected; an
    /// output tag on a line whose parent is no such command places nothing.
    #[test]
    fn a_command_with_no_command_run_is_named_by_its_tags() {
        let stray = r#"{"type":"user","uuid":"m3","parentUuid":"zz","message":{"content":"<local-command-stdout>pasted</local-command-stdout>"}}"#;
        let stray_system = r#"{"type":"system","subtype":"local_command","uuid":"r3","parentUuid":"zz","content":"<local-command-stdout>x</local-command-stdout>"}"#;
        let owned = |l: &str, t: &str| (l.to_string(), t.to_string());
        assert_eq!(
            labelled(&[COMPACT, COMPACTED, RESTART, RESTARTED, stray, stray_system]),
            [
                owned("owner-command", "/compact keep it short"),
                owned(
                    "injected command-output",
                    "<local-command-stdout>Compacted </local-command-stdout>"
                ),
                owned("owner-command", "/restart"),
                owned(
                    "injected command-output",
                    "<local-command-stdout>Successfully updated</local-command-stdout>"
                ),
                owned(
                    "unlabelled",
                    "<local-command-stdout>pasted</local-command-stdout>"
                ),
            ]
        );
        assert_eq!(
            labels(&[COMPACT, COMPACTED], true),
            ["dispatch", "dispatch"]
        );
    }

    /// The claim: a `<command-name>` line that a `commandRun` line names is that command's echo,
    /// printed once, on a `user` line and on a `local_command` line alike.
    #[test]
    fn a_command_s_echo_named_by_its_command_run_line_prints_once() {
        let echo_system = r#"{"type":"system","subtype":"local_command","uuid":"e1","content":"<command-name>/remote-control</command-name>\n<command-args></command-args>"}"#;
        let run = r#"{"type":"system","subtype":"local_command","uuid":"e2","parentUuid":"e1","commandRun":{"command":"remote-control","args":""},"content":"<local-command-stdout></local-command-stdout>"}"#;
        let echo_user = r#"{"type":"user","uuid":"e3","message":{"content":"<command-name>/clear</command-name>"}}"#;
        let run_clear = r#"{"type":"system","subtype":"local_command","uuid":"e4","parentUuid":"e3","commandRun":{"command":"clear","args":""}}"#;
        assert_eq!(
            labels(&[echo_system, run, echo_user, run_clear], false),
            ["owner-command", "owner-command"]
        );
    }

    /// The claim: a cut is read in a line its fields label injected or tool, with its file and line,
    /// its count and its pointer, and never in the output of a command the owner ran, which a text
    /// row places.
    #[test]
    fn a_cut_is_read_only_in_a_line_its_fields_label_injected_or_tool() {
        let notice = r#"{"type":"user","uuid":"n1","origin":{"kind":"task-notification"},"message":{"content":"report\n... (truncated 12 chars, full result in /t/w.output)"}}"#;
        let result = r#"{"type":"user","uuid":"n2","parentUuid":"n1","message":{"content":[{"type":"tool_result","tool_use_id":"t","content":"r\n... (truncated 7 chars)"}]}}"#;
        let shell = r#"{"type":"user","uuid":"n3","parentUuid":"n2","message":{"content":"<bash-input>cat x</bash-input>"}}"#;
        let printed = r#"{"type":"user","uuid":"n4","parentUuid":"n3","message":{"content":"<bash-stdout>head\n... (truncated 9 chars)</bash-stdout>"}}"#;
        let file = file(&[
            notice,
            result,
            shell,
            printed,
            RESTART,
            RESTARTED
                .replace("Successfully updated", "x\\n... (truncated 4 chars)")
                .as_str(),
            COMPACT,
            COMPACTED
                .replace("Compacted ", "x\\n... (truncated 5 chars)")
                .as_str(),
        ]);
        let mut inventory = Inventory::default();
        let mut ctx = Labelling {
            file: &file,
            name: "t.jsonl",
            in_subagent: false,
            inventory: &mut inventory,
            shell_inputs: HashSet::new(),
            commands: HashSet::new(),
        };
        let all: Vec<usize> = (0..file.lines.len()).collect();
        ctx.label(&all);
        let cuts: Vec<(&str, &str, Option<&str>)> = inventory
            .cuts
            .iter()
            .map(|c| (c.at.as_str(), c.chars.as_str(), c.whole_in.as_deref()))
            .collect();
        assert_eq!(
            cuts,
            [
                ("t.jsonl:1", "12", Some("/t/w.output")),
                ("t.jsonl:2", "7", None)
            ]
        );
    }

    /// The claim: a handle names one session, so two `sessionId`s sharing their first 8 characters,
    /// one of them cut twice, give three handles.
    #[test]
    fn two_session_ids_sharing_a_prefix_give_distinct_handles() {
        let line = |uuid: &str, parent: &str, id: &str| {
            format!(
                r#"{{"type":"user","uuid":"{uuid}","parentUuid":{parent},"sessionId":"{id}","origin":{{"kind":"human"}},"message":{{"content":"hi"}}}}"#
            )
        };
        let a = "9a9a9a9a-0000-0000-0000-000000000020";
        let b = "9a9a9a9a-0000-0000-0000-000000000021";
        let first = file(&[&line("a1", "null", a), &line("a2", "null", a)]);
        let second = file(&[&line("b1", "null", b)]);
        let corpus = Corpus::of_files(Path::new("."), vec![first, second]);
        let handles: Vec<&str> = corpus.sessions.iter().map(|s| s.handle.as_str()).collect();
        assert_eq!(handles, ["9a9a9a9a", "9a9a9a9a#2", "9a9a9a9a#3"]);
        // A `sessionId` that holds `#` takes a handle another would be numbered to.
        let hashed = file(&[&line("c1", "null", "ab#2")]);
        let plain = file(&[&line("d1", "null", "ab"), &line("d2", "null", "ab")]);
        let corpus = Corpus::of_files(Path::new("."), vec![hashed, plain]);
        let handles: Vec<&str> = corpus.sessions.iter().map(|s| s.handle.as_str()).collect();
        assert_eq!(handles, ["ab#2", "ab", "ab#3"]);
    }

    /// The values a labelling of `lines` reports, in a top-level file or a subagent file.
    fn reported(lines: &[&str], in_subagent: bool) -> Vec<(String, String)> {
        let file = file(lines);
        let mut inventory = Inventory::default();
        let mut ctx = Labelling {
            file: &file,
            name: "t.jsonl",
            in_subagent,
            inventory: &mut inventory,
            shell_inputs: HashSet::new(),
            commands: HashSet::new(),
        };
        let all: Vec<usize> = (0..file.lines.len()).collect();
        ctx.label(&all);
        inventory.unknown.keys().cloned().collect()
    }

    /// The claim: a `local_command` line with no `commandRun` that prints nothing is reported, and
    /// one that prints, a command named by its tag or its output, or an echo, is not: the output of
    /// an echo, of a line the owner typed, or of a subagent's command, and a subagent's command
    /// itself, are each reported.
    #[test]
    fn a_local_command_line_that_prints_nothing_is_reported() {
        let gap = vec![(
            "system local_command shape".to_string(),
            "no commandRun".to_string(),
        )];
        assert!(reported(&[RESTART, RESTARTED, COMPACT, COMPACTED], false).is_empty());
        let unknown = r#"{"type":"system","subtype":"local_command","uuid":"u1","content":"<something-else>x</something-else>"}"#;
        let stray = r#"{"type":"system","subtype":"local_command","uuid":"u2","parentUuid":"zz","content":"<local-command-stdout>x</local-command-stdout>"}"#;
        assert_eq!(reported(&[unknown], false), gap);
        assert_eq!(reported(&[stray], false), gap);
        let echo = r#"{"type":"user","uuid":"e3","message":{"content":"<command-name>/clear</command-name>"}}"#;
        let run = r#"{"type":"system","subtype":"local_command","uuid":"e4","parentUuid":"e3","commandRun":{"command":"clear","args":""}}"#;
        let of_echo = r#"{"type":"system","subtype":"local_command","uuid":"e5","parentUuid":"e3","content":"<local-command-stdout>OUT</local-command-stdout>"}"#;
        assert!(reported(&[echo, run], false).is_empty());
        assert_eq!(reported(&[echo, run, of_echo], false), gap);
        let typed = r#"{"type":"user","uuid":"t1","origin":{"kind":"human"},"message":{"content":"<command-name>/y</command-name>"}}"#;
        let of_typed = r#"{"type":"system","subtype":"local_command","uuid":"t2","parentUuid":"t1","content":"<local-command-stdout>OUT</local-command-stdout>"}"#;
        assert_eq!(reported(&[typed, of_typed], false), gap);
        let of_dispatch = r#"{"type":"system","subtype":"local_command","uuid":"m9","parentUuid":"m1","content":"<local-command-stdout>OUT</local-command-stdout>"}"#;
        assert_eq!(reported(&[COMPACT, of_dispatch], true), gap);
        assert_eq!(reported(&[RESTART], true), gap);
    }

    const PARENT: &str =
        r#"{"type":"user","uuid":"p1","origin":{"kind":"human"},"message":{"content":"hi"}}"#;
    const CHAIN: &str = r#"{"type":"assistant","uuid":"f1","timestamp":"2026-01-01T00:00:00Z","message":{"content":[]}}"#;

    fn fork_text(head: &str) -> Option<String> {
        let parent = file(&[PARENT, PARENT.replace("p1", "p2").as_str()]);
        let later = CHAIN.replace("2026-01-01", "2026-01-02");
        fork_context(&file(&[head, CHAIN, later.as_str()]), &parent).map(|e| {
            assert_eq!((e.line, e.timestamp.as_str()), (1, "2026-01-01T00:00:00Z"));
            e.text
        })
    }

    #[test]
    fn a_fork_line_names_the_parent_line_it_inherits_up_to() {
        let found = r#"{"type":"fork-context-ref","parentLastUuid":"p2","contextLength":3}"#;
        assert_eq!(
            fork_text(found).as_deref(),
            Some("a fork: it inherits its parent's context up to L2 (contextLength 3)")
        );
        let missing = r#"{"type":"fork-context-ref","parentLastUuid":"zz"}"#;
        assert_eq!(
            fork_text(missing).as_deref(),
            Some("a fork: it inherits its parent's context up to the line \"zz\", which the parent's file does not hold")
        );
        let empty = r#"{"type":"fork-context-ref","parentLastUuid":"","contextLength":3}"#;
        assert_eq!(
            fork_text(empty),
            fork_text(r#"{"type":"fork-context-ref","contextLength":3}"#)
        );
        let unnamed = r#"{"type":"fork-context-ref","contextLength":3}"#;
        assert_eq!(
            fork_text(unnamed).as_deref(),
            Some("a fork: it inherits its parent's context up to a line it does not name (contextLength 3)")
        );
        assert!(fork_context(&file(&[CHAIN]), &file(&[PARENT])).is_none());
    }

    #[test]
    fn the_survey_reports_a_fork_line_with_no_parent_line_or_outside_a_subagent_file() {
        let line = r#"{"type":"fork-context-ref","contextLength":3}"#;
        let mut sub = Inventory::default();
        sub.survey(&file(&[line]), "s", true);
        let mut top = Inventory::default();
        top.survey(&file(&[line]), "t", false);
        let keys = |i: &Inventory| i.unknown.keys().cloned().collect::<Vec<_>>();
        let good = r#"{"type":"fork-context-ref","parentLastUuid":"p","contextLength":3}"#;
        let empty = r#"{"type":"fork-context-ref","parentLastUuid":"","contextLength":3}"#;
        let odd = r#"{"type":"fork-context-ref","parentLastUuid":"p","contextLength":"3"}"#;
        let mut more = Inventory::default();
        more.survey(&file(&[good, good]), "a", true);
        more.survey(&file(&[empty]), "b", true);
        more.survey(&file(&[odd]), "c", true);
        assert_eq!(
            keys(&more),
            [
                (
                    "fork-context-ref place".to_string(),
                    "not the first in its file".to_string()
                ),
                (
                    "fork-context-ref shape".to_string(),
                    "contextLength not a count".to_string()
                ),
                (
                    "fork-context-ref shape".to_string(),
                    "no parentLastUuid".to_string()
                ),
            ]
        );
        assert_eq!(
            keys(&sub),
            [(
                "fork-context-ref shape".to_string(),
                "no parentLastUuid".to_string()
            )]
        );
        assert_eq!(
            keys(&top),
            [(
                "fork-context-ref place".to_string(),
                "outside a subagent file".to_string()
            )]
        );
    }
}
