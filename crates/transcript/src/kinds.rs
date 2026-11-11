//! The inventory of the line kinds of a corpus of transcripts, which `cargo x transcript-kinds`
//! prints: the measurement this repository's transcript heads take before a value of the format is
//! classed, per `design@transcript@transcript-format-tolerated-and-reported`.
//!
//! The maintenance tool includes this file and `crate::reader` with `#[path]`, and the library
//! declares it for its own tests alone, so the published API gains no function no consumer calls,
//! per `design@xtask@view-included-not-exported`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde_json::Value;

use crate::reader::{carries_interrupt_id, str_field, Corpus, Entry, File, Label, Raw, Session};

/// One row of `cargo x transcript-kinds`: the lines of one kind in one place.
#[derive(Default)]
pub(crate) struct KindRow {
    pub(crate) count: usize,
    pub(crate) versions: BTreeSet<String>,
    /// Each top-level key, with the count of the row's lines that carry it.
    pub(crate) keys: BTreeMap<String, usize>,
}

/// The inventory of every line of the files a corpus read, top-level and attached subagent files
/// alike, by place, `top` or `subagent`, and kind.
///
/// A line's kind is its `type`, with its subtype for `system` and its `attachment.type` for
/// `attachment`; a `user` line's adds the label the reader gave it, its first entry's, `tool` for a
/// tool result whatever its tool, or `(no entry)` for a line the labels print nothing of, such as a
/// command's echo or a repeated uuid, and `(no field)` after a label where the line carries none of
/// the fields the mapping reads. So the lines no field places are rows of their own, which
/// `design@transcript@transcript-labels-in-code` measures, and a line that is no JSON is the row
/// `(not JSON)`. A subagent file no session holds is counted too.
pub(crate) fn line_kinds(corpus: &Corpus) -> BTreeMap<(&'static str, String), KindRow> {
    let mut rows: BTreeMap<(&'static str, String), KindRow> = BTreeMap::new();
    let mut count = |place: &'static str, file: &File, entries: &[&Entry]| {
        let mut labels: HashMap<usize, &Label> = HashMap::new();
        for e in entries {
            labels.entry(e.line).or_insert(&e.label);
        }
        for (i, raw) in file.lines.iter().enumerate() {
            let value = match raw {
                Raw::Blank => continue,
                Raw::Malformed => {
                    rows.entry((place, "(not JSON)".to_string()))
                        .or_default()
                        .count += 1;
                    continue;
                }
                Raw::Json(value) => value,
            };
            let kind = match str_field(value, "type") {
                None => "(no type)".to_string(),
                Some("system") => {
                    format!("system {}", str_field(value, "subtype").unwrap_or("(none)"))
                }
                Some("attachment") => format!(
                    "attachment {}",
                    value
                        .get("attachment")
                        .and_then(|a| str_field(a, "type"))
                        .unwrap_or("(none)")
                ),
                // A line that carries none of the fields the mapping reads is marked, so the lines
                // the text rows and the dispatch row place, an interrupt with no id among them, are
                // rows of their own.
                Some("user") => match labels.get(&(i + 1)) {
                    None => "user (no entry)".to_string(),
                    Some(Label::Tool(_)) => "user tool".to_string(),
                    Some(label) if carries_no_field(value) => {
                        format!("user {} (no field)", label.display())
                    }
                    Some(label) => format!("user {}", label.display()),
                },
                Some(other) => other.to_string(),
            };
            let row = rows.entry((place, kind)).or_default();
            row.count += 1;
            if let Some(version) = str_field(value, "version") {
                row.versions.insert(version.to_string());
            }
            for key in value.as_object().into_iter().flat_map(|o| o.keys()) {
                *row.keys.entry(key.clone()).or_default() += 1;
            }
        }
    };
    for (index, file) in corpus.files.iter().enumerate() {
        let sessions: Vec<&Session> = corpus.sessions.iter().filter(|s| s.file == index).collect();
        let entries: Vec<&Entry> = sessions.iter().flat_map(|s| &s.entries).collect();
        count("top", file, &entries);
        for agent in sessions.iter().flat_map(|s| &s.subagents) {
            let entries: Vec<&Entry> = agent.entries.iter().collect();
            count("subagent", &agent.file, &entries);
        }
    }
    for agent in &corpus.unattached {
        let entries: Vec<&Entry> = agent.entries.iter().collect();
        count("subagent", &agent.file, &entries);
    }
    rows
}

/// Whether a `user` line carries none of the fields the mapping reads: `isMeta`, `isCompactSummary`,
/// `origin` and a string `interruptedMessageId`.
fn carries_no_field(value: &Value) -> bool {
    value.get("isMeta").is_none()
        && value.get("isCompactSummary").is_none()
        && value.get("origin").is_none()
        && !carries_interrupt_id(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn file(lines: &[&str]) -> File {
        File {
            path: PathBuf::from("t.jsonl"),
            lines: lines
                .iter()
                .map(|l| Raw::Json(serde_json::from_str(l).expect("a fixture line is JSON")))
                .collect(),
        }
    }

    /// The claim: a line's kind is its type, its subtype or attachment type, and for a `user` line
    /// the label it takes, marked where the line carries no field the mapping reads, `tool` for any
    /// tool result and `(no entry)` where it prints nothing; each
    /// key is counted over the lines of its row, and a line that is no JSON is a row of its own.
    #[test]
    fn a_line_s_kind_names_its_label_and_its_keys_are_counted() {
        let mut lines = file(&[
            r#"{"type":"user","uuid":"a","sessionId":"s","version":"1","origin":{"kind":"human"},"message":{"content":"hi"}}"#,
            r#"{"type":"user","uuid":"a2","parentUuid":"a","sessionId":"s","origin":{"kind":"human"},"message":{"content":"again hi"}}"#,
            r#"{"type":"assistant","uuid":"a3","parentUuid":"a2","sessionId":"s","message":{"content":[{"type":"tool_use","id":"t","name":"Bash","input":{}}]}}"#,
            r#"{"type":"user","uuid":"b","parentUuid":"a3","sessionId":"s","promptId":"p","message":{"content":"stray"}}"#,
            r#"{"type":"user","uuid":"c","parentUuid":"b","sessionId":"s","message":{"content":[{"type":"tool_result","tool_use_id":"t","content":"r"}]}}"#,
            r#"{"type":"user","uuid":"c","parentUuid":"b","sessionId":"s","message":{"content":"again"}}"#,
            r#"{"type":"system","subtype":"turn_duration","uuid":"d","parentUuid":"c","sessionId":"s"}"#,
            r#"{"type":"attachment","uuid":"e","parentUuid":"d","sessionId":"s","attachment":{"type":"date"}}"#,
            r#"{"type":"last-prompt","sessionId":"s"}"#,
            r#"{"type":"user","uuid":"f","parentUuid":"e","sessionId":"s","interruptedMessageId":"m","message":{"content":"[Request interrupted by user]"}}"#,
            r#"{"type":"user","uuid":"g","parentUuid":"f","sessionId":"s","message":{"content":"[Request interrupted by user]"}}"#,
        ]);
        lines.lines.push(Raw::Malformed);
        let corpus = Corpus::of_files(&PathBuf::from("."), vec![lines]);
        let rows = line_kinds(&corpus);
        let kinds: Vec<(&str, &str, usize)> = rows
            .iter()
            .map(|((place, kind), row)| (*place, kind.as_str(), row.count))
            .collect();
        assert_eq!(
            kinds,
            [
                ("top", "(not JSON)", 1),
                ("top", "assistant", 1),
                ("top", "attachment date", 1),
                ("top", "last-prompt", 1),
                ("top", "system turn_duration", 1),
                ("top", "user (no entry)", 1),
                ("top", "user owner", 2),
                ("top", "user owner-interrupt", 1),
                ("top", "user owner-interrupt (no field)", 1),
                ("top", "user tool", 1),
                ("top", "user unlabelled (no field)", 1),
            ]
        );
        let unlabelled = &rows[&("top", "user unlabelled (no field)".to_string())];
        assert_eq!(unlabelled.keys.get("promptId"), Some(&1));
        assert!(!unlabelled.keys.contains_key("origin"));
        let owner = &rows[&("top", "user owner".to_string())];
        assert_eq!(owner.versions.iter().collect::<Vec<_>>(), ["1"]);
        assert_eq!(owner.keys.get("origin"), Some(&2));
    }
}
