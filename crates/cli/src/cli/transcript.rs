//! The arguments of `transcript`, per `design@cli@arguments-parse-through-clap`. Its logic is
//! `knowledge_architect_transcript::transcript`.

use clap::{Args, Subcommand};

use knowledge_architect_transcript::{ListOptions, SearchOptions, ShowOptions};

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
    /// owner-interrupt, agent, dispatch, injected, tool, unlabelled.
    #[arg(
        long = "role",
        value_name = "ROLE",
        value_parser = clap::builder::PossibleValuesParser::new(knowledge_architect_transcript::ROLES)
    )]
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

impl From<TranscriptVerb> for knowledge_architect_transcript::TranscriptVerb {
    fn from(verb: TranscriptVerb) -> Self {
        match verb {
            TranscriptVerb::List(a) => Self::List(ListOptions {
                branch: a.branch,
                containing: a.containing,
                since: a.since,
                all_projects: a.all_projects,
                json: a.json,
            }),
            TranscriptVerb::Show(a) => Self::Show(ShowOptions {
                session: a.session,
                from: a.from,
                subagents: a.subagents,
                tools: a.tools,
                full: a.full,
                all_projects: a.all_projects,
                json: a.json,
            }),
            TranscriptVerb::Search(a) => Self::Search(SearchOptions {
                pattern: a.pattern,
                roles: a.roles,
                tool: a.tool,
                tools: a.tools,
                branch: a.branch,
                since: a.since,
                all_projects: a.all_projects,
                subagents: a.subagents,
                json: a.json,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use knowledge_architect_transcript::TranscriptVerb as Typed;

    /// The claim: each field of `transcript show` reaches the typed function.
    #[test]
    fn show_converts_every_field() {
        let args = TranscriptShowArgs {
            session: "s".to_string(),
            from: Some("f".to_string()),
            subagents: true,
            tools: true,
            full: true,
            all_projects: true,
            json: true,
        };
        let Typed::Show(o) = TranscriptVerb::Show(args).into() else {
            panic!("another verb")
        };
        assert_eq!((o.session.as_str(), o.from.as_deref()), ("s", Some("f")));
        assert!(o.subagents && o.tools && o.full && o.all_projects && o.json);
    }

    /// The claim: each field of `transcript search` reaches the typed function.
    #[test]
    fn search_converts_every_field() {
        let args = TranscriptSearchArgs {
            pattern: "p".to_string(),
            roles: vec!["owner".to_string()],
            tool: Some("t".to_string()),
            tools: true,
            branch: Some("b".to_string()),
            since: Some("d".to_string()),
            all_projects: true,
            subagents: true,
            json: true,
        };
        let Typed::Search(o) = TranscriptVerb::Search(args).into() else {
            panic!("another verb")
        };
        assert_eq!(o.pattern, "p");
        assert_eq!(o.roles, ["owner"]);
        assert_eq!(
            (o.tool.as_deref(), o.branch.as_deref(), o.since.as_deref()),
            (Some("t"), Some("b"), Some("d"))
        );
        assert!(o.tools && o.all_projects && o.subagents && o.json);
    }
}
