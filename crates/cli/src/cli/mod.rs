//! `check`'s commands as a command line: their clap declarations, their dispatch, and finding the
//! project, per `design@cli@the-commands-are-a-library-module`.
//!
//! A binary parses its arguments, flattening [`Command`] into its own command enum, and hands the
//! parsed command to [`run`] with the directories of its own source. A check does not find the project
//! itself: it would have to do it from its own location, and then could not be run against a model
//! built in memory. The logic of each command is
//! the typed function of the crate that holds it, such as [`knowledge_architect_check::commands`]; a value of an `*Args` struct holding a combination that
//! function's type refuses is refused here, before any work, so a value clap never saw is refused.
//!
//! Arguments are declared, never parsed by hand, per `design@cli@arguments-parse-through-clap`, and
//! the exit codes are `design@cli@exit-code-ladder`: 0 ran-and-clean, 1 ran-and-negative, 2
//! could-not-run.

use std::path::Path;
use std::process::ExitCode;

use clap::{Args, Subcommand};

use knowledge_architect_query::{self as query, IssuesFilter, TripwiresFilter, WorklistMode};

use crate::check::commands::{self, CheckMode, IndexMode};
use crate::extension::Extension;
use crate::Manifest;

mod transcript;
pub use transcript::{
    TranscriptArgs, TranscriptListArgs, TranscriptSearchArgs, TranscriptShowArgs, TranscriptVerb,
};

pub use crate::check::commands::{
    complete_working_tree, output, refuse_a_foreign_build, refuse_another_version, Gathered,
    Library,
};

/// This library, as its own build saw it: the package `knowledge-architect`. The checks' crate, which
/// it re-exports, is an entry of its own in a list that names it.
pub fn this_library() -> Library {
    Library {
        crate_dir: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        package: env!("CARGO_PKG_NAME"),
    }
}

/// The default of `worklist --bound`, which `--verify` takes no other value of.
const DEFAULT_WORKLIST_BOUND: usize = 40000;

/// `check`'s commands. A binary flattens this enum into its own.
///
/// Non-exhaustive: a binary hands a parsed command to [`run`] without matching it, so a new
/// command is not a breaking change for any binary.
// Non-exhaustive per `design@check@ne-minimal`. clap reads the doc comment above as the about and
// long-about text of the binary that flattens this enum; the binary's own attributes win over
// them, but a binary that sets an about and no long-about showed this text under `--help`. The
// attribute clears both, so `--help` shows only what the binary says of itself.
#[derive(Subcommand)]
#[command(about = None, long_about = None)]
#[non_exhaustive]
pub enum Command {
    /// Every check, over one walk, in four phases.
    Check(CheckArgs),
    /// One recorded entry, whole, and every reference to it.
    Show(ShowArgs),
    /// Whether each id is free: every entity the project defines under it, or `free`, or that it
    /// is no id.
    Id(IdArgs),
    /// Every issue entry, one row each.
    Issues(IssuesArgs),
    /// Every tripwire entry, one row each.
    Tripwires(TripwiresArgs),
    /// Regenerate every generated index in place.
    Index(IndexArgs),
    /// Every observation the walk produced: file, line, kind, value.
    Model,
    /// Every anchor of the project, with its kind, its path, and each register's home.
    Anchors,
    /// Judge every commit of a range, its message and its tree, each message against its own
    /// commit's tree.
    Commits(CommitsArgs),
    /// Write the agent files this version ships into the project, and remove the ones it no
    /// longer ships.
    InstallAgentSkills,
    /// Create a knowledge-architect project: its manifest, the installed agent files, and the
    /// documents and homes the check requires. It overwrites no hand-written file outside the
    /// installer's namespace, which every install rewrites.
    Init(InitArgs),
    /// Add a Component to the project: its line in the manifest, its documents and its register
    /// homes. It overwrites no hand-written file outside the installer's namespace.
    NewComponent(NewComponentArgs),
    /// The work list of a search of the whole record: every entry grouped into clusters inside its
    /// search unit, a Component or a declared location, packed into agents by search unit, with one
    /// report skeleton per agent; or the check of the filled reports.
    Worklist(WorklistArgs),
    /// The sessions of Claude Code's transcripts that ran in this project: listed, one shown, or
    /// searched, each line labelled by its fields, and every line not recognised reported.
    Transcript(TranscriptArgs),
}

#[derive(Args)]
pub struct IdArgs {
    /// The ids to look up, in the grammar `[a-z0-9]+(-[a-z0-9]+)+`.
    #[arg(required = true, value_name = "ID")]
    pub ids: Vec<String>,
}

#[derive(Args)]
pub struct WorklistArgs {
    /// Write the work list into DIR, a new or empty directory outside the project: `agents.txt`,
    /// `work.txt`, `shown.md`, the snapshot no agent is given, and per agent a directory holding
    /// its entries whole, with its context entries and its goal title lines, and the report it
    /// fills.
    #[arg(
        long,
        value_name = "DIR",
        conflicts_with = "verify",
        required_unless_present = "verify"
    )]
    pub out: Option<std::path::PathBuf>,
    /// Check the reports the agents filled in DIR, a directory `--out` wrote. It first removes
    /// every `verdicts-*.md`, the `generated-lines.md`, the `gaps.md` and the `fit-lines.md` of
    /// DIR. Every entry an agent was given
    /// has one verdict line in its report, in the verdicts its register admits and the actions its
    /// verdict admits; `against` names subjects `work.txt` lists, or is `-` on `unrelated` and on
    /// `constrains` with the action `none`; the quotation is a run of at least eight whole words of
    /// the entry's own text, below its first line, or all of a shorter one; a reason is given; and
    /// a seed or an entry citing one is never `unrelated`. Each report closes with `## Decided`
    /// and `## Met outside the task`. When every report passes, it writes `verdicts-<subject>.md`
    /// per subject and `generated-lines.md`, the lines a commit message owes; at a design audit,
    /// a work list `--out` wrote with `--seeds-from`, `gaps.md`, every verdict but `constrains`
    /// and `unrelated`, and `fit-lines.md`, the lines under the plan document's citation, in place
    /// of `generated-lines.md`.
    #[arg(long, value_name = "DIR")]
    pub verify: Option<std::path::PathBuf>,
    /// The words one agent is given at most, the work's words included; a group above what the
    /// work leaves goes to an agent alone.
    #[arg(
        long,
        value_name = "WORDS",
        default_value_t = DEFAULT_WORKLIST_BOUND,
        conflicts_with = "verify"
    )]
    pub bound: usize,
    /// A reference the work names, repeatable: each report line of an entry that cites it, or is
    /// it, names it, and such an entry is never given the verdict `unrelated`.
    #[arg(long = "seed", value_name = "REF", conflicts_with = "verify")]
    pub seeds: Vec<String>,
    /// A file of the work, repeatable: `work.txt` lists it for the agents to read, and its words
    /// come out of the bound. A relative path is read from the working directory; a file given
    /// twice, in any spelling, counts once. A file that cannot be read, or whose path holds a line
    /// break, is refused, nothing written.
    #[arg(long = "work", value_name = "PATH", conflicts_with = "verify")]
    pub work: Vec<std::path::PathBuf>,
    /// A subject of the work, repeatable, an id in the grammar `[a-z0-9]+(-[a-z0-9]+)+`; with
    /// none, the one subject is `whole-work`.
    #[arg(
        long = "subject",
        value_name = "ID",
        conflicts_with = "verify",
        value_parser = subject_id
    )]
    pub subjects: Vec<String>,
    /// A revision naming one commit: every entry whose file differs between it and the work tree,
    /// an untracked file included, is marked `changed-since` in `agents.txt` and in the entries
    /// files, a mark, not a filter. A revision git cannot resolve to one commit, or a failure of
    /// git asked what changed since it or for its message, is refused with git's reason, nothing
    /// written. The work list names the commit by its subject.
    #[arg(long, value_name = "COMMIT", conflicts_with = "verify")]
    pub since: Option<String>,
    /// The path of a plan document, a spec, a slice's spec or a milestone document, for its
    /// design audit: the design entries and declared-register entries its section "What is
    /// already decided" cites become seeds, beside any `--seed`, and `--verify` writes the
    /// audit's outputs. A path that is no plan document, or that holds a line break, is refused,
    /// nothing written.
    #[arg(long = "seeds-from", value_name = "SPEC", conflicts_with = "verify")]
    pub seeds_from: Option<std::path::PathBuf>,
}

/// A subject of the work, checked by its form while parsing.
fn subject_id(value: &str) -> Result<String, String> {
    if knowledge_architect_model::table::is_entry_id(value) {
        Ok(value.to_string())
    } else {
        Err(format!(
            "`{value}` is no id: a subject matches [a-z0-9]+(-[a-z0-9]+)+"
        ))
    }
}

#[derive(Args)]
pub struct NewComponentArgs {
    /// The Component's directory, relative to the working directory and inside the project.
    #[arg(value_name = "DIR")]
    pub dir: std::path::PathBuf,
}

#[derive(Args)]
pub struct InitArgs {
    /// The project's directory, created when it does not exist.
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: std::path::PathBuf,
    /// The project's name. The default is the basename of PATH.
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
    /// A Component to add, as `new-component` adds one; repeatable. DIR is relative to PATH.
    #[arg(long = "component", value_name = "DIR")]
    pub components: Vec<std::path::PathBuf>,
    /// Also write the maintenance crate of a Rust project, `xtask`, with its virtual workspace,
    /// its cargo aliases and `/target` in `.gitignore`, as the setup skill shows them. Refused
    /// where a `Cargo.toml` lies at or above PATH, a file it writes already exists, or a Component
    /// lies at or under `target`.
    #[arg(long)]
    pub rust: bool,
    /// The project's command, written as `[project] command`; the default is `klarch`, or
    /// `cargo klarch` under `--rust`.
    #[arg(long, value_name = "CMD")]
    pub command: Option<String>,
    /// Whether to run `git init`. The default runs it outside every git worktree, and refuses
    /// a directory there that already holds files.
    #[arg(long, value_enum, value_name = "VCS")]
    pub vcs: Option<Vcs>,
}

/// The value of `init --vcs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Vcs {
    /// Run `git init` at PATH, inside a worktree or outside, unless PATH already holds `.git`.
    Git,
    /// Never run `git init`.
    None,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Apply the safe fixes the checker admits before checking: write the generated files and the
    /// installed agent files whose bytes the tree and this version determine, and list each.
    #[arg(long)]
    pub fix: bool,
    /// Judge the tree git's index would commit, instead of the working tree: HEAD's tree with
    /// the staged changes, read from git objects. To repair its generated files, run `index
    /// --staged`, then `check --staged` again; `--fix` repairs the working tree only.
    #[arg(long, conflicts_with = "fix")]
    pub staged: bool,
}

#[derive(Args)]
pub struct IndexArgs {
    /// Write the generated files the tree git's index would commit needs into the index, and
    /// touch no working-tree file: for a commit of part of the working tree.
    #[arg(long)]
    pub staged: bool,
}

#[derive(Args)]
pub struct CommitsArgs {
    /// The range to walk, as `git rev-list` reads one.
    #[arg(value_name = "RANGE")]
    pub range: String,
}

#[derive(Args)]
pub struct ShowArgs {
    /// The reference to print, `<kind>@<anchor>@<id>`.
    #[arg(value_name = "REF")]
    pub reference: String,
}

#[derive(Args)]
pub struct IssuesArgs {
    /// Only the entries of this kind.
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,
    /// Only the entries in this group subdirectory.
    #[arg(long, value_name = "GROUP")]
    pub group: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    #[arg(value_name = "ANCHOR|TEXT")]
    pub terms: Vec<String>,
}

#[derive(Args)]
pub struct TripwiresArgs {
    /// Only the entries guarding this entry: a decision, a goal or a declared register's
    /// entry, as the `guarding` column lists them.
    #[arg(long, value_name = "REF")]
    pub guarding: Option<String>,
    /// An anchor to list, then text every row's id or title must contain.
    #[arg(value_name = "ANCHOR|TEXT")]
    pub terms: Vec<String>,
}

/// Run one of `check`'s commands over `manifest`.
///
/// `checker` is every directory of the running binary's own source, compiled into it, per
/// `design@check@checker-source-literals-are-data`. The exit code is
/// `design@cli@exit-code-ladder`: `Err` is could-not-run, and the caller prints it and exits 2.
/// `extensions` is every extension the binary registers, per
/// `design@check@an-extension-plugs-in-through-phased-hooks`; `klarch` passes none. A value holding a
/// combination the command refuses, a `worklist` subject that is no id in the grammar of an entry
/// id, which clap refuses while parsing, and `Command::Init`, which goes to [`init`], are `Err`. The command
/// prints, so a closed stdout ends the process with exit 2, and `index` and `check --fix` can return
/// exit 2 themselves, having printed why.
pub fn run(
    command: Command,
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    // Refused before an extension's `resolve` runs, so a value that holds a refused combination
    // does no work, per `design@cli@command-parsing-home`.
    match &command {
        Command::Check(args) => drop(check_mode(args)?),
        Command::Worklist(args) => drop(worklist_mode(args)?),
        _ => {}
    }
    // Every command reads the manifest as its extensions resolved it: the files they generate
    // are outside every walk, and a table they refuse is phase 1 of `check`.
    let mut configured = manifest.clone();
    crate::extension::configure(&mut configured, extensions);
    let manifest = &configured;
    match command {
        Command::Check(args) => commands::check(manifest, check_mode(&args)?, checker, extensions),
        Command::Show(args) => query::show(manifest, &args.reference, checker),
        Command::Id(args) => query::id(manifest, &args.ids, checker),
        Command::Issues(args) => query::issues(
            manifest,
            &IssuesFilter {
                kind: args.kind,
                group: args.group,
                terms: args.terms,
            },
            checker,
        ),
        Command::Tripwires(args) => query::tripwires(
            manifest,
            &TripwiresFilter {
                guarding: args.guarding,
                terms: args.terms,
            },
            checker,
        ),
        Command::Index(args) => {
            let mode = if args.staged {
                IndexMode::Staged
            } else {
                IndexMode::Working
            };
            commands::index(manifest, mode, checker, extensions)
        }
        Command::Model => commands::model(manifest, checker, extensions),
        Command::Anchors => query::anchors(manifest, checker),
        Command::Commits(args) => commands::commits(manifest, &args.range, checker, extensions),
        Command::InstallAgentSkills => commands::install_agent_skills(manifest),
        Command::NewComponent(args) => {
            knowledge_architect_create::new_component(manifest, &args.dir, checker, extensions)
        }
        Command::Worklist(args) => query::worklist(manifest, &worklist_mode(&args)?, checker),
        Command::Transcript(args) => {
            knowledge_architect_transcript::transcript(manifest.root(), &args.verb.into())
        }
        // A binary hands `init` to [`init`] before it locates a project, since `init` is what
        // makes one, per `design@cli@the-commands-are-a-library-module`; reaching here means the
        // binary skipped that step, inside a project.
        Command::Init(_) => Err(format!(
            "`init` creates a project, and {} already holds one; a binary dispatches `init` to \
             `cli::init` before it locates a project",
            manifest.root().display()
        )),
    }
}

/// The mode of `check`, or the refusal of a value that asks two, which clap refuses while parsing
/// and only a value built by hand can hold.
fn check_mode(args: &CheckArgs) -> Result<CheckMode, String> {
    match (args.fix, args.staged) {
        (true, true) => Err("`--fix` and `--staged` exclude each other".to_string()),
        (true, false) => Ok(CheckMode::Fix),
        (false, true) => Ok(CheckMode::Staged),
        (false, false) => Ok(CheckMode::Working),
    }
}

/// The mode of `worklist`, or the refusal of a value clap would have refused while parsing.
fn worklist_mode(args: &WorklistArgs) -> Result<WorklistMode, String> {
    for subject in &args.subjects {
        subject_id(subject)?;
    }
    match (&args.out, &args.verify) {
        (Some(dir), None) => Ok(WorklistMode::Write {
            dir: dir.clone(),
            bound: args.bound,
            seeds: args.seeds.clone(),
            work: args.work.clone(),
            subjects: args.subjects.clone(),
            since: args.since.clone(),
            seeds_from: args.seeds_from.clone(),
        }),
        (None, Some(dir))
            if args.seeds.is_empty()
                && args.bound == DEFAULT_WORKLIST_BOUND
                && args.work.is_empty()
                && args.subjects.is_empty()
                && args.since.is_none()
                && args.seeds_from.is_none() =>
        {
            Ok(WorklistMode::Verify { dir: dir.clone() })
        }
        (None, Some(_)) => Err("`--verify` takes none of `--bound`, `--seed`, `--work`, \
             `--subject`, `--since` and `--seeds-from`"
            .to_string()),
        _ => Err("give exactly one of `--out` and `--verify`".to_string()),
    }
}

/// Create a knowledge-architect project at `args.path`, as `klarch init` does.
///
/// A binary calls this before [`locate`], with the same `libraries` and `packages` it hands
/// [`refuse_a_foreign_build`] and the same `checker` and `extensions` it hands [`run`]. `Err` is a
/// refusal: nothing was written, and the binary exits 2 with it. After the first write, a failure is
/// exit 1 with the files in place, and a closed stdout still ends the process with exit 2.
pub fn init(
    args: InitArgs,
    libraries: &[Library],
    packages: &[&str],
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let options = knowledge_architect_create::InitOptions {
        path: args.path,
        name: args.name,
        components: args.components,
        rust: args.rust,
        command: args.command,
        vcs: args.vcs.map(|vcs| match vcs {
            Vcs::Git => knowledge_architect_create::Vcs::Git,
            Vcs::None => knowledge_architect_create::Vcs::None,
        }),
    };
    knowledge_architect_create::init(options, libraries, packages, checker, extensions)
}

/// The project is whatever declares itself one at or above the working directory.
///
/// Nothing about any particular repository is compiled in, so pointing the tool at a copy of a mock
/// project needs no flag and no special case. `Err` is a working
/// directory it cannot read, or no manifest at or above it.
pub fn locate() -> Result<Manifest, String> {
    let cwd =
        std::env::current_dir().map_err(|e| format!("cannot read the working directory: {e}"))?;
    Manifest::find(&cwd).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worklist_args() -> WorklistArgs {
        WorklistArgs {
            out: None,
            verify: None,
            bound: DEFAULT_WORKLIST_BOUND,
            seeds: Vec::new(),
            work: Vec::new(),
            subjects: Vec::new(),
            since: None,
            seeds_from: None,
        }
    }

    fn a_project() -> Manifest {
        let text =
            "[project]\nname = \"probe\"\nchecker-version = \"fixture\"\ncomponents = []\n\n\
                    [walk]\nskip-dirs = []\nskip-files = []\n\n[agents]\nharness = []\n";
        Manifest::parse(Path::new("/nowhere"), text).expect("the manifest parses")
    }

    /// The claim: a binary that hands `init` to [`run`] inside a project gets a refusal naming
    /// `cli::init`, not a run of anything.
    #[test]
    fn run_refuses_init() {
        let args = InitArgs {
            path: std::path::PathBuf::from("."),
            name: None,
            command: None,
            vcs: None,
            components: Vec::new(),
            rust: false,
        };
        let err = run(Command::Init(args), &a_project(), &[], &mut []).expect_err("refused");
        assert!(err.contains("cli::init"), "{err}");
    }

    /// An extension that fails the test if any of its hooks runs, so a run that passes shows the
    /// refusal came before them.
    struct NoHookRuns;

    impl Extension for NoHookRuns {
        fn tables(&self) -> &'static [&'static str] {
            panic!("a refused value reached an extension")
        }
        fn resolve(&mut self, _: &Manifest) -> crate::extension::Resolution {
            panic!("a refused value reached an extension")
        }
        fn checks(&self) -> &'static [&'static str] {
            panic!("a refused value reached an extension")
        }
        fn dump(&self, _: &crate::Model) -> Vec<crate::extension::DumpRow> {
            panic!("a refused value reached an extension")
        }
        fn prepare(
            &mut self,
            _: &Manifest,
            _: &crate::Model,
            _: crate::extension::Tree<'_>,
            _: crate::extension::Purpose,
        ) -> Result<Box<dyn crate::extension::Prepared>, String> {
            panic!("a refused value reached an extension")
        }
    }

    /// `run` refuses `command`, and no hook of an extension ran.
    fn refused_by_run(command: Command) {
        let mut extensions: Vec<Box<dyn Extension>> = vec![Box::new(NoHookRuns)];
        assert!(run(command, &a_project(), &[], &mut extensions).is_err());
    }

    fn a_dir() -> Option<std::path::PathBuf> {
        Some(std::path::PathBuf::from("d"))
    }

    /// The claim: `run` refuses a `CheckArgs` that asks both modes, before an extension runs.
    #[test]
    fn run_refuses_a_check_value_asking_both_modes() {
        refused_by_run(Command::Check(CheckArgs {
            fix: true,
            staged: true,
        }));
    }

    /// The claim: `run` refuses a `WorklistArgs` naming both directories.
    #[test]
    fn run_refuses_a_worklist_value_with_both_directories() {
        refused_by_run(Command::Worklist(WorklistArgs {
            out: a_dir(),
            verify: a_dir(),
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a `WorklistArgs` naming neither directory.
    #[test]
    fn run_refuses_a_worklist_value_with_neither_directory() {
        refused_by_run(Command::Worklist(worklist_args()));
    }

    /// The claim: `run` refuses a bound beside `--verify`.
    #[test]
    fn run_refuses_a_worklist_value_with_a_bound_beside_verify() {
        refused_by_run(Command::Worklist(WorklistArgs {
            verify: a_dir(),
            bound: 1,
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a seed beside `--verify`.
    #[test]
    fn run_refuses_a_worklist_value_with_a_seed_beside_verify() {
        refused_by_run(Command::Worklist(WorklistArgs {
            verify: a_dir(),
            seeds: vec!["a".to_string()],
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a file of the work beside `--verify`. Mutation checked: the test of
    /// `work` removed from the guard of `worklist_mode`'s `--verify` arm.
    #[test]
    fn run_refuses_a_worklist_value_with_a_work_beside_verify() {
        refused_by_run(Command::Worklist(WorklistArgs {
            verify: a_dir(),
            work: vec![std::path::PathBuf::from("w.md")],
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a subject beside `--verify`. Mutation checked: the test of
    /// `subjects` removed from the guard of `worklist_mode`'s `--verify` arm.
    #[test]
    fn run_refuses_a_worklist_value_with_a_subject_beside_verify() {
        refused_by_run(Command::Worklist(WorklistArgs {
            verify: a_dir(),
            subjects: vec!["a-subject".to_string()],
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a commit of `--since` beside `--verify`. Mutation checked: the
    /// test of `since` removed from the guard of `worklist_mode`'s `--verify` arm.
    #[test]
    fn run_refuses_a_worklist_value_with_a_since_beside_verify() {
        refused_by_run(Command::Worklist(WorklistArgs {
            verify: a_dir(),
            since: Some("HEAD".to_string()),
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a plan document of `--seeds-from` beside `--verify`. Mutation
    /// checked: the test of `seeds_from` removed from the guard of `worklist_mode`'s `--verify` arm.
    #[test]
    fn run_refuses_a_worklist_value_with_a_seeds_from_beside_verify() {
        refused_by_run(Command::Worklist(WorklistArgs {
            verify: a_dir(),
            seeds_from: Some(std::path::PathBuf::from("s.md")),
            ..worklist_args()
        }));
    }

    /// The claim: `run` refuses a subject that is no id, which clap refuses while parsing.
    /// Mutation checked: the loop over the subjects removed from `worklist_mode`.
    #[test]
    fn run_refuses_a_worklist_value_with_a_subject_that_is_no_id() {
        refused_by_run(Command::Worklist(WorklistArgs {
            out: a_dir(),
            subjects: vec!["Not An Id".to_string()],
            ..worklist_args()
        }));
    }

    /// The claim: the mode holds `--verify` alone as a check of the reports.
    #[test]
    fn verify_alone_is_the_check_of_the_reports() {
        let verify = WorklistArgs {
            verify: a_dir(),
            ..worklist_args()
        };
        assert!(matches!(
            worklist_mode(&verify),
            Ok(WorklistMode::Verify { .. })
        ));
    }
}
