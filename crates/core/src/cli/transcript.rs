//! `transcript`: the sessions of Claude Code's transcripts, listed, shown and searched.
//!
//! The reader is `crate::transcript`; this module parses the verbs' arguments, picks the files a
//! verb reads, and prints. Its output, text and `--json` alike, is for a reader and is no contract,
//! per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

use std::path::Path;
use std::process::ExitCode;

use clap::{Args, Subcommand};
use serde_json::json;

use super::output::outln;
use crate::transcript::{project_files, transcript_dir, Corpus, Entry, Inventory, Label, Session};
use crate::Manifest;

/// The arguments of `transcript`.
#[derive(Args)]
pub struct TranscriptArgs {
    #[command(subcommand)]
    pub verb: TranscriptVerb,
}

/// The verbs of `transcript`.
#[derive(Subcommand)]
pub enum TranscriptVerb {
    /// Every session of the project, one row each, oldest first.
    List(TranscriptListArgs),
    /// One session's kept lines, in file order, each with its line number and its label.
    Show(TranscriptShowArgs),
    /// Every kept line whose text matches a regular expression, one line each.
    Search(TranscriptSearchArgs),
}

/// The arguments of `transcript list`.
#[derive(Args)]
pub struct TranscriptListArgs {
    /// Keep a session one of whose lines carries this git branch.
    #[arg(long, value_name = "BRANCH")]
    pub branch: Option<String>,
    /// Keep a session holding a line the owner typed whose text contains TEXT.
    #[arg(long, value_name = "TEXT")]
    pub containing: Option<String>,
    /// Keep a session whose last line is on or after DATE, `YYYY-MM-DD`, in UTC.
    #[arg(long, value_name = "DATE")]
    pub since: Option<String>,
    /// Read the sessions of every project, not only this one's.
    #[arg(long)]
    pub all_projects: bool,
    /// One JSON object per line, the header first. No contract.
    #[arg(long)]
    pub json: bool,
}

/// The arguments of `transcript show`.
#[derive(Args)]
pub struct TranscriptShowArgs {
    /// A session's handle, as `list` prints it, or a prefix of a `sessionId` that names one
    /// session.
    #[arg(value_name = "SESSION")]
    pub session: String,
    /// Start at the first line the owner typed whose text contains TEXT.
    #[arg(long, value_name = "TEXT")]
    pub from: Option<String>,
    /// Print each attached subagent file after the session.
    #[arg(long)]
    pub subagents: bool,
    /// Keep tool calls and tool results.
    #[arg(long)]
    pub tools: bool,
    /// Print injected and tool texts whole.
    #[arg(long)]
    pub full: bool,
    /// Resolve SESSION among the sessions of every project.
    #[arg(long)]
    pub all_projects: bool,
    /// One JSON object per line, the header first. No contract.
    #[arg(long)]
    pub json: bool,
}

/// The arguments of `transcript search`.
#[derive(Args)]
pub struct TranscriptSearchArgs {
    /// A regular expression, in the syntax of the `regex` crate.
    #[arg(value_name = "REGEX")]
    pub pattern: String,
    /// Keep lines of this role, repeatable: owner, owner-answer, owner-queued, owner-command,
    /// agent, dispatch, injected, tool, unlabelled.
    #[arg(long = "role", value_name = "ROLE")]
    pub roles: Vec<String>,
    /// Keep the calls of this tool and their results.
    #[arg(long, value_name = "NAME")]
    pub tool: Option<String>,
    /// Search tool calls and tool results too.
    #[arg(long)]
    pub tools: bool,
    /// Search only the sessions one of whose lines carries this git branch.
    #[arg(long, value_name = "BRANCH")]
    pub branch: Option<String>,
    /// Search only the sessions whose last line is on or after DATE, `YYYY-MM-DD`, in UTC.
    #[arg(long, value_name = "DATE")]
    pub since: Option<String>,
    /// Search the sessions of every project, not only this one's.
    #[arg(long)]
    pub all_projects: bool,
    /// Search the attached subagent files too.
    #[arg(long)]
    pub subagents: bool,
    /// One JSON object per line, the header first. No contract.
    #[arg(long)]
    pub json: bool,
}

/// An injected or tool text above this many lines prints its first [`HEAD_LINES`] unless `--full`:
/// a choice of display, not a measurement.
const CUT_ABOVE: usize = 40;
const HEAD_LINES: usize = 5;
/// The characters `list` keeps of the first owner message, and `search` of a matching line.
const LIST_EXCERPT: usize = 80;
const SEARCH_EXCERPT: usize = 200;

/// Run one verb of `transcript`.
pub(super) fn transcript(manifest: &Manifest, args: &TranscriptArgs) -> Result<ExitCode, String> {
    let dir = transcript_dir()?;
    match &args.verb {
        TranscriptVerb::List(a) => list(manifest, &dir, a),
        TranscriptVerb::Show(a) => show(manifest, &dir, a),
        TranscriptVerb::Search(a) => search(manifest, &dir, a),
    }
}

fn root_filter(manifest: &Manifest, all_projects: bool) -> Option<&Path> {
    (!all_projects).then(|| manifest.root())
}

fn check_date(date: &Option<String>) -> Result<(), String> {
    if let Some(date) = date {
        let bytes = date.as_bytes();
        let shaped = bytes.len() == 10
            && bytes.iter().enumerate().all(|(i, b)| match i {
                4 | 7 => *b == b'-',
                _ => b.is_ascii_digit(),
            });
        if !shaped {
            return Err(format!("--since takes a date as YYYY-MM-DD, not {date:?}"));
        }
    }
    Ok(())
}

/// Whether a session passes `--branch` and `--since`. A timestamp is ISO 8601 in UTC, so its
/// first ten characters compare as text with a `YYYY-MM-DD` date.
fn session_kept(session: &Session, branch: &Option<String>, since: &Option<String>) -> bool {
    branch
        .as_ref()
        .is_none_or(|b| session.branches.iter().any(|x| x == b))
        && since
            .as_ref()
            .is_none_or(|d| session.last_timestamp.get(..10).unwrap_or("") >= d.as_str())
}

fn list(manifest: &Manifest, dir: &Path, args: &TranscriptListArgs) -> Result<ExitCode, String> {
    check_date(&args.since)?;
    let files = project_files(dir, root_filter(manifest, args.all_projects))?;
    let corpus = Corpus::read(dir, &files, true)?;
    let mut sessions: Vec<&Session> = corpus
        .sessions
        .iter()
        .filter(|s| session_kept(s, &args.branch, &args.since))
        .filter(|s| {
            args.containing.as_ref().is_none_or(|text| {
                s.entries
                    .iter()
                    .any(|e| e.label.owner_typed() && e.text.contains(text.as_str()))
            })
        })
        .collect();
    sessions.sort_by(|a, b| a.first_timestamp.cmp(&b.first_timestamp));
    header(&corpus.inventory, args.json);
    for s in &sessions {
        let owner = s.entries.iter().filter(|e| e.label.owner_typed()).count();
        let first = s
            .entries
            .iter()
            .find(|e| e.label.owner_typed())
            .map(|e| one_line(&e.text, LIST_EXCERPT))
            .unwrap_or_default();
        if args.json {
            outln!(
                "{}",
                json!({
                    "session": s.handle,
                    "session_id": s.session_id,
                    "file": corpus.name(&corpus.files[s.file].path),
                    "started": s.first_timestamp,
                    "ended": s.last_timestamp,
                    "branches": s.branches,
                    "owner_lines": owner,
                    "compactions": s.compactions,
                    "subagents": s.subagents.len(),
                    "first_owner_message": first,
                })
            );
        } else {
            outln!(
                "{}  {}  {}  {}  owner {}  compactions {}  subagents {}  {}",
                s.handle,
                clock(&s.first_timestamp, true),
                clock(&s.last_timestamp, true),
                s.branches.join(", "),
                owner,
                s.compactions,
                s.subagents.len(),
                first
            );
        }
    }
    Ok(if sessions.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn show(manifest: &Manifest, dir: &Path, args: &TranscriptShowArgs) -> Result<ExitCode, String> {
    let files = project_files(dir, root_filter(manifest, args.all_projects))?;
    // Resolution reads the files the handle may name; the header counts only the session's own
    // file, and its subagent files under `--subagents`.
    let index = Corpus::read(dir, &files, false)?;
    let chosen = match resolve(&index.sessions, &args.session) {
        Ok(s) => s,
        Err(message) => {
            if args.json {
                outln!("{}", json!({ "result": "no-session", "message": message }));
            } else {
                outln!("{message}");
            }
            return Ok(ExitCode::from(1));
        }
    };
    let file = index.files[index.sessions[chosen].file].path.clone();
    let handle = index.sessions[chosen].handle.clone();
    let ordinal = index.sessions[chosen].ordinal;
    // Cut again from its file alone, the session is found by its place in the file: its handle's
    // `#n` counts over every file read, which this second read does not see.
    let corpus = Corpus::read(dir, &[file], args.subagents)?;
    let session = corpus
        .sessions
        .iter()
        .find(|s| s.ordinal == ordinal)
        .expect("the session is cut again from its file");
    let mut entries: Vec<&Entry> = session
        .entries
        .iter()
        .filter(|e| args.tools || !matches!(e.label, Label::Tool(_)))
        .collect();
    if let Some(text) = &args.from {
        match entries
            .iter()
            .position(|e| e.label.owner_typed() && e.text.contains(text.as_str()))
        {
            Some(start) => entries.drain(..start).for_each(drop),
            None => {
                header(&corpus.inventory, args.json);
                let message = format!("no line the owner typed in {handle} contains {text:?}");
                if args.json {
                    outln!("{}", json!({ "result": "no-line", "message": message }));
                } else {
                    outln!("{message}");
                }
                return Ok(ExitCode::from(1));
            }
        }
    }
    header(&corpus.inventory, args.json);
    let mut printed = 0;
    if !args.json {
        outln!(
            "# session {handle}: {}",
            corpus.name(&corpus.files[session.file].path)
        );
    }
    for e in &entries {
        print_entry(&handle, e, args.full, args.json);
        printed += 1;
    }
    if args.subagents {
        for agent in &session.subagents {
            let tag = format!("{handle}/{}", short(&agent.agent_id));
            if args.json {
                outln!(
                    "{}",
                    json!({
                        "subagent": tag,
                        "agent_type": agent.agent_type,
                        "description": agent.description,
                        "file": corpus.name(&agent.file.path),
                    })
                );
            } else {
                outln!(
                    "# subagent {tag}: {} — {} ({})",
                    agent.agent_type,
                    agent.description,
                    corpus.name(&agent.file.path)
                );
            }
            for e in agent
                .entries
                .iter()
                .filter(|e| args.tools || !matches!(e.label, Label::Tool(_)))
            {
                print_entry(&tag, e, args.full, args.json);
                printed += 1;
            }
        }
    }
    Ok(if printed == 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn search(
    manifest: &Manifest,
    dir: &Path,
    args: &TranscriptSearchArgs,
) -> Result<ExitCode, String> {
    check_date(&args.since)?;
    let pattern = regex::Regex::new(&args.pattern)
        .map_err(|e| format!("the regular expression does not parse: {e}"))?;
    let files = project_files(dir, root_filter(manifest, args.all_projects))?;
    let corpus = Corpus::read(dir, &files, args.subagents)?;
    header(&corpus.inventory, args.json);
    let mut hits = 0;
    for s in corpus
        .sessions
        .iter()
        .filter(|s| session_kept(s, &args.branch, &args.since))
    {
        let mut units: Vec<(String, &Vec<Entry>)> = vec![(s.handle.clone(), &s.entries)];
        if args.subagents {
            for agent in &s.subagents {
                units.push((
                    format!("{}/{}", s.handle, short(&agent.agent_id)),
                    &agent.entries,
                ));
            }
        }
        for (tag, entries) in units {
            // A tool result names its call by id, so `--tool` keeps the results of the calls
            // that tool made.
            let calls: std::collections::HashSet<&str> = entries
                .iter()
                .filter(|e| matches!(&e.label, Label::Tool(n) if Some(n) == args.tool.as_ref()))
                .filter_map(|e| e.tool_use_id.as_deref())
                .collect();
            for e in entries {
                let is_tool = matches!(e.label, Label::Tool(_));
                if is_tool {
                    let kept = match &args.tool {
                        Some(name) => {
                            matches!(&e.label, Label::Tool(n) if n == name)
                                || e.tool_use_id
                                    .as_deref()
                                    .is_some_and(|id| calls.contains(id))
                        }
                        None => args.tools,
                    };
                    if !kept {
                        continue;
                    }
                } else if args.tool.is_some() {
                    continue;
                }
                if !args.roles.is_empty() && !args.roles.iter().any(|r| r == e.label.role()) {
                    continue;
                }
                // The pattern reads the line's whole text, so it may span a line break; what prints
                // is the text's line where the match starts.
                let Some(found) = pattern.find(&e.text) else {
                    continue;
                };
                let start = e.text[..found.start()].rfind('\n').map_or(0, |i| i + 1);
                let end = e.text[found.start()..]
                    .find('\n')
                    .map_or(e.text.len(), |i| found.start() + i);
                let matched = &e.text[start..end];
                hits += 1;
                if args.json {
                    outln!(
                        "{}",
                        json!({
                            "session": tag,
                            "line": e.line,
                            "label": e.label.display(),
                            "timestamp": e.timestamp,
                            "text": matched,
                        })
                    );
                } else {
                    outln!(
                        "{tag} L{} {}  {}",
                        e.line,
                        e.label.display(),
                        cut(matched, SEARCH_EXCERPT)
                    );
                }
            }
        }
    }
    Ok(if hits == 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

/// The session `arg` names: a handle exactly, or a prefix of a `sessionId` that names one session.
fn resolve(sessions: &[Session], arg: &str) -> Result<usize, String> {
    if let Some(i) = sessions.iter().position(|s| s.handle == arg) {
        return Ok(i);
    }
    let candidates: Vec<usize> = (0..sessions.len())
        .filter(|&i| sessions[i].session_id.starts_with(arg))
        .collect();
    match candidates.as_slice() {
        [one] => Ok(*one),
        [] => Err(format!("no session is named {arg:?}")),
        many => Err(format!(
            "{arg:?} names {} sessions: {}",
            many.len(),
            many.iter()
                .map(|&i| sessions[i].handle.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// The header: what was read, and what was not recognised.
fn header(inventory: &Inventory, json: bool) {
    let versions: Vec<&str> = inventory.versions.iter().map(String::as_str).collect();
    let mut lines: Vec<(serde_json::Value, String)> = vec![(
        json!({"header": "read", "files": inventory.files, "versions": versions}),
        format!(
            "# files read: {}; harness versions: {}",
            inventory.files,
            if versions.is_empty() {
                "none".to_string()
            } else {
                versions.join(", ")
            }
        ),
    )];
    for ((category, value), (count, at)) in &inventory.unknown {
        lines.push((
            json!({"header": "unrecognised", "category": category, "value": value, "count": count, "first": at}),
            format!("# unrecognised {category} {value:?}: {count}, first at {at}"),
        ));
    }
    if inventory.unlabelled > 0 {
        lines.push((
            json!({"header": "unlabelled", "count": inventory.unlabelled}),
            format!("# unlabelled lines: {}", inventory.unlabelled),
        ));
    }
    for at in &inventory.malformed {
        lines.push((
            json!({"header": "not-json", "at": at}),
            format!("# not JSON: {at}"),
        ));
    }
    if inventory.repeated_uuids > 0 {
        lines.push((
            json!({"header": "repeated-uuid", "count": inventory.repeated_uuids}),
            format!(
                "# lines skipped for a repeated uuid: {}",
                inventory.repeated_uuids
            ),
        ));
    }
    for file in &inventory.fallback {
        lines.push((
            json!({"header": "fallback", "file": file}),
            format!("# no line carries a field in {file}: its lines are labelled by their text"),
        ));
    }
    if inventory.has_gap() {
        let gap = "each is a gap of this command: report it as a finding on the checker in the \
                   retrospective";
        lines.push((json!({"header": "gap", "text": gap}), format!("# {gap}")));
    }
    for (object, text) in lines {
        if json {
            outln!("{object}");
        } else {
            outln!("{text}");
        }
    }
}

fn print_entry(tag: &str, e: &Entry, full: bool, json: bool) {
    if json {
        outln!(
            "{}",
            json!({
                "session": tag,
                "line": e.line,
                "label": e.label.display(),
                "timestamp": e.timestamp,
                "text": e.text,
            })
        );
        return;
    }
    // A subagent's line carries its agent's id, so a quotation of it is told from the session's own.
    let agent = tag
        .split_once('/')
        .map(|(_, id)| format!("{id} "))
        .unwrap_or_default();
    outln!(
        "[{agent}L{} {} {}]",
        e.line,
        e.label.display(),
        clock(&e.timestamp, true)
    );
    let lines: Vec<&str> = e.text.lines().collect();
    if !full && !e.label.never_cut() && lines.len() > CUT_ABOVE {
        for line in &lines[..HEAD_LINES] {
            outln!("{line}");
        }
        outln!(
            "({} lines omitted; --full prints them)",
            lines.len() - HEAD_LINES
        );
    } else {
        outln!("{}", e.text);
    }
    outln!();
}

/// `2026-10-25T15:36:05.195Z` as `2026-10-25 15:36`, or the time alone.
fn clock(timestamp: &str, with_date: bool) -> String {
    match (timestamp.get(..10), timestamp.get(11..16)) {
        (Some(date), Some(time)) if with_date => format!("{date} {time}"),
        (Some(_), Some(time)) => time.to_string(),
        _ => timestamp.to_string(),
    }
}

fn short(id: &str) -> String {
    id.chars().take(8).collect()
}

/// The first non-empty line of a text, cut to `max` characters.
fn one_line(text: &str, max: usize) -> String {
    cut(
        text.lines().find(|l| !l.trim().is_empty()).unwrap_or(""),
        max,
    )
}

fn cut(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        let mut s: String = text.chars().take(max).collect();
        s.push('…');
        s
    }
}
