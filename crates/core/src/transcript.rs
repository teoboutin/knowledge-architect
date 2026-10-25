//! The reader of Claude Code's session transcripts: the files a project's sessions wrote, cut into
//! sessions, each kept line labelled by its fields, and the inventory of what was not recognised.
//!
//! The format is the harness's, which its vendor documents as internal and liable to change between
//! versions, so the reader tolerates what it does not know and counts it, never fails on it and never
//! drops it in silence. What it knows is the sets of values under the "known" constants below, each
//! marked as used or ignored on purpose: a value outside them is a gap of this reader, which the
//! command's header reports.
//!
//! Lines are selected and labelled by their fields, never by a substring of their text: a filter on
//! text once dropped one of the owner's messages from a review. The one text test is the fallback for
//! a file whose `user` lines carry no field at all, and the prefix that names a skill's body inside a
//! line its fields already mark as injected.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

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
];

/// The `system` subtypes: `local_command` and `compact_boundary` are used, the others ignored.
const KNOWN_SYSTEM_SUBTYPES: &[&str] = &[
    "turn_duration",
    "compact_boundary",
    "scheduled_task_fire",
    "informational",
    "local_command",
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
];

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
            Raw::Malformed => None,
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

fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

/// The `cwd` of the first chain line of a file, read without parsing the rest.
fn first_cwd(path: &Path) -> Result<Option<String>, String> {
    use std::io::BufRead;
    let file =
        std::fs::File::open(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        if let Ok(value) = serde_json::from_str::<Value>(&line) {
            if is_chain(&value) {
                return Ok(str_field(&value, "cwd").map(str::to_string));
            }
        }
    }
    Ok(None)
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

/// The top-level files whose first chain line's `cwd` is `root` or a path under it, or every
/// top-level file when `root` is `None`.
///
/// Both sides are canonicalised where they exist, so a symlinked checkout matches; a `cwd` that no
/// longer exists is compared as written. Only the first chain line is read: a session that later
/// works in a worktree changes its `cwd` mid-file, and the directory it started in is what names its
/// project.
pub(crate) fn project_files(dir: &Path, root: Option<&Path>) -> Result<Vec<PathBuf>, String> {
    let files = top_level_files(dir)?;
    let Some(root) = root else {
        return Ok(files);
    };
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let mut kept = Vec::new();
    for path in files {
        let Some(cwd) = first_cwd(&path)? else {
            continue;
        };
        let cwd = PathBuf::from(&cwd);
        let cwd = cwd.canonicalize().unwrap_or(cwd);
        if cwd.starts_with(&root) {
            kept.push(path);
        }
    }
    Ok(kept)
}

/// The role the command gives a kept line, per the spec's mapping table "Line to label".
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Label {
    Owner,
    OwnerAnswer,
    OwnerQueued,
    OwnerCommand,
    Agent,
    Dispatch,
    /// Injected text, with its kind: a known `origin.kind`, `unknown`, `skill`, `meta`,
    /// `compact-summary` or `compact-boundary`.
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
            Label::Agent => "agent".to_string(),
            Label::Dispatch => "dispatch".to_string(),
            Label::Injected(kind) => format!("injected {kind}"),
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
            Label::Agent => "agent",
            Label::Dispatch => "dispatch",
            Label::Injected(_) => "injected",
            Label::Tool(_) => "tool",
            Label::Unlabelled => "unlabelled",
        }
    }

    /// An owner's or the agent's text, or a dispatch, is never cut.
    pub(crate) fn never_cut(&self) -> bool {
        matches!(
            self,
            Label::Owner
                | Label::OwnerAnswer
                | Label::OwnerQueued
                | Label::OwnerCommand
                | Label::Agent
                | Label::Dispatch
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
    /// The files the fallback labelled at least one line in.
    pub(crate) fallback: BTreeSet<String>,
}

impl Inventory {
    fn note_unknown(&mut self, category: &str, value: &str, at: String) {
        let slot = self
            .unknown
            .entry((category.to_string(), value.to_string()))
            .or_insert((0, at));
        slot.0 += 1;
    }

    /// Whether the header ends with the line asking for the gap to be reported.
    pub(crate) fn has_gap(&self) -> bool {
        !self.unknown.is_empty() || self.unlabelled > 0 || !self.malformed.is_empty()
    }

    /// Count one file's lines against the known sets. Every line is counted, chain or not, so a
    /// value outside a set is reported whatever session it sits in.
    fn survey(&mut self, file: &File, name: &str) {
        self.files += 1;
        for (i, raw) in file.lines.iter().enumerate() {
            let at = format!("{name}:{}", i + 1);
            let value = match raw {
                Raw::Json(value) => value,
                Raw::Malformed => {
                    self.malformed.push(at);
                    continue;
                }
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
                    if let Some(items) = value
                        .get("message")
                        .and_then(|m| m.get("content"))
                        .and_then(Value::as_array)
                    {
                        for item in items {
                            let item_type = str_field(item, "type").unwrap_or("(none)");
                            if !known.contains(&item_type) {
                                self.note_unknown(
                                    &format!("{kind} content item"),
                                    item_type,
                                    at.clone(),
                                );
                            }
                        }
                    }
                    if kind == "user" {
                        if let Some(origin) = value.get("origin").and_then(|o| str_field(o, "kind"))
                        {
                            if !KNOWN_ORIGIN_KINDS.contains(&origin) {
                                self.note_unknown("origin.kind", origin, at.clone());
                            }
                        }
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
                        if let Some(origin) = attachment
                            .and_then(|a| a.get("origin"))
                            .and_then(|o| str_field(o, "kind"))
                        {
                            if !KNOWN_ORIGIN_KINDS.contains(&origin) {
                                self.note_unknown("attachment.origin.kind", origin, at.clone());
                            }
                        }
                    }
                }
                "system" => {
                    let subtype = str_field(value, "subtype").unwrap_or("(none)");
                    if !KNOWN_SYSTEM_SUBTYPES.contains(&subtype) {
                        self.note_unknown("system subtype", subtype, at.clone());
                    } else if subtype == "local_command" && value.get("commandRun").is_none() {
                        self.note_unknown("system local_command shape", "no commandRun", at);
                    }
                }
                other if KNOWN_METADATA_TYPES.contains(&other) => {}
                other => self.note_unknown("type", other, at),
            }
        }
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
}

impl Corpus {
    /// Read `paths`, top-level files under `dir`, and their subagent files when `subagents`.
    pub(crate) fn read(dir: &Path, paths: &[PathBuf], subagents: bool) -> Result<Corpus, String> {
        let mut corpus = Corpus {
            dir: dir.to_path_buf(),
            files: Vec::new(),
            sessions: Vec::new(),
            inventory: Inventory::default(),
        };
        for path in paths {
            let file = File::read(path)?;
            let name = corpus.name(path);
            corpus.inventory.survey(&file, &name);
            let index = corpus.files.len();
            corpus.files.push(file);
            corpus.cut(index);
        }
        if subagents {
            corpus.attach_subagents()?;
        }
        Ok(corpus)
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
        let file = &self.files[index];
        let chain = file.chain();
        let mut runs: Vec<(String, Vec<usize>)> = Vec::new();
        let mut previous: Option<String> = None;
        for &i in &chain {
            let value = file.json(i).expect("a chain line is JSON");
            let session_id = str_field(value, "sessionId").unwrap_or("").to_string();
            // A session opens at the first chain line, where the `sessionId` changes, and at a
            // root that is no compaction. A compaction's boundary has a null parent and stays in
            // its session; a resumed process's first line has a parent, measured over every
            // resume on the machine the design was measured on. Each line is judged on its own, so
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
        let fallback = file_needs_fallback(file);
        let name = self.name(&file.path);
        let mut seen_ids: HashMap<String, usize> = HashMap::new();
        for (session_id, lines) in runs {
            let n = seen_ids.entry(session_id.clone()).or_insert(0);
            *n += 1;
            let short: String = session_id.chars().take(8).collect();
            let handle = if *n == 1 {
                short
            } else {
                format!("{short}#{n}")
            };
            let mut ctx = Labelling {
                file,
                name: &name,
                fallback,
                in_subagent: false,
                inventory: &mut self.inventory,
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
                self.inventory.survey(&file, &name);
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
                // The fallback is a top-level file's: a subagent file's lines with no field are
                // its dispatcher's, which the `dispatch` row places.
                let mut ctx = Labelling {
                    file: &file,
                    name: &name,
                    fallback: false,
                    in_subagent: true,
                    inventory: &mut self.inventory,
                };
                let entries = ctx.label(&chain);
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
                if let Some(s) = last {
                    placed.push((s, agent));
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

/// Whether no `user` line of the file carries `isMeta`, `isCompactSummary` or `origin`: a log
/// written before those fields, or a session run with `claude -p`, whose lines the labels cannot
/// place by their fields.
fn file_needs_fallback(file: &File) -> bool {
    !file.lines.iter().any(|raw| match raw {
        Raw::Json(value) => {
            str_field(value, "type") == Some("user")
                && (value.get("isMeta").is_some()
                    || value.get("isCompactSummary").is_some()
                    || value.get("origin").is_some())
        }
        Raw::Malformed => false,
    })
}

/// Whether a text opens with a harness tag: `<`, a name of lowercase letters, digits, `-` and `_`,
/// then `>` or a space, leading whitespace aside.
fn opens_with_tag(text: &str) -> bool {
    let rest = match text.trim_start().strip_prefix('<') {
        Some(rest) => rest,
        None => return false,
    };
    let name_len = rest
        .chars()
        .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-' || *c == '_')
        .count();
    name_len > 0 && matches!(rest[name_len..].chars().next(), Some('>') | Some(' '))
}

/// The labelling of one file's chain lines.
struct Labelling<'a> {
    file: &'a File,
    name: &'a str,
    fallback: bool,
    in_subagent: bool,
    inventory: &'a mut Inventory,
}

impl Labelling<'_> {
    /// The entries of `lines`, in file order, per the spec's mapping table "Line to label": the
    /// first row whose condition a line meets gives its label.
    fn label(&mut self, lines: &[usize]) -> Vec<Entry> {
        // The parents of the `local_command` lines, by `uuid` over the run: the command's own
        // `user` line is printed as the `owner-command` line that follows it.
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
                if consumed.contains(uuid) {
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
                    let label = match attachment.get("origin").and_then(|o| str_field(o, "kind")) {
                        Some("human") => Label::OwnerQueued,
                        Some(kind) if KNOWN_ORIGIN_KINDS.contains(&kind) => {
                            Label::Injected(kind.to_string())
                        }
                        Some(_) => Label::Injected("unknown".to_string()),
                        None if attachment.get("isMeta").and_then(Value::as_bool) == Some(true) => {
                            Label::Injected("meta".to_string())
                        }
                        None => self.unlabelled(&prompt),
                    };
                    entries.push(entry(label, prompt, None));
                }
                "system" => match str_field(value, "subtype") {
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
        let text = match content {
            Some(Value::Array(items)) => items
                .iter()
                .filter(|item| str_field(item, "type") == Some("text"))
                .filter_map(|item| str_field(item, "text"))
                .collect::<Vec<_>>()
                .join("\n"),
            other => text_of(other),
        };
        let is_meta = value.get("isMeta").and_then(Value::as_bool) == Some(true);
        let origin = value.get("origin").and_then(|o| str_field(o, "kind"));
        let compact = value.get("isCompactSummary").and_then(Value::as_bool) == Some(true);
        // In a subagent file, a `user` line with no field is text its dispatcher or the workflow
        // harness wrote: the brief that opens the file, or a workflow's task. Measured over every
        // subagent file on the machine the design was measured on, that held for every such line
        // but one owner's interrupt, which the agent never saw as a message.
        let label = if self.in_subagent && !is_meta && !compact && value.get("origin").is_none() {
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
                None => self.unlabelled(&text),
            }
        };
        entries.push(entry(label, text, None));
    }

    /// A line no row places: `unlabelled` and counted, or, in a file with no field, labelled by
    /// its text.
    fn unlabelled(&mut self, text: &str) -> Label {
        if self.fallback {
            self.inventory.fallback.insert(self.name.to_string());
            if opens_with_tag(text) {
                Label::Injected("tag".to_string())
            } else {
                Label::Owner
            }
        } else {
            self.inventory.unlabelled += 1;
            Label::Unlabelled
        }
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
        let options: Vec<&str> = question
            .get("options")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|o| str_field(o, "label"))
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

    #[test]
    fn a_tag_opens_with_a_lowercase_name_then_a_bracket_or_a_space() {
        assert!(opens_with_tag("<system-reminder>x"));
        assert!(opens_with_tag("\n\n<pasted_content id=\"1\">"));
        assert!(opens_with_tag("<task-notification>"));
        assert!(!opens_with_tag("< not a tag"));
        assert!(!opens_with_tag("<A>"));
        assert!(!opens_with_tag("plain text <b>"));
        assert!(!opens_with_tag("<>"));
    }
}
