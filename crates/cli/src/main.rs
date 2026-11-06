//! The binary `klarch`: every command of the checker, with no extension registered, per
//! `design@cli@the-commands-are-a-library-module`.
//!
//! A project whose manifest declares only what the checker reads runs this. A project with a
//! subject of its own builds a binary that registers the extension for it.

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "klarch",
    version,
    about = "What a project's documents hold, and whether it still holds.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: knowledge_architect::cli::Command,
}

fn main() -> ExitCode {
    // Parsed before the project is located, so `--help` answers from anywhere. clap exits 2 on
    // a parse failure, which is already the could-not-run code.
    let cli = Cli::parse();
    // The paths compiled in: this crate's own Component, the checker's, and each crate's below it, whose fixtures are read as data, per `design@check@checker-source-literals-are-data`.
    let dir = knowledge_architect::component_dir();
    let check_dir = knowledge_architect_check::component_dir();
    let git_dir = knowledge_architect_git::component_dir();
    let model_dir = knowledge_architect_model::component_dir();
    let create_dir = knowledge_architect_create::component_dir();
    let query_dir = knowledge_architect_query::component_dir();
    let transcript_dir = knowledge_architect_transcript::component_dir();
    let dirs: Vec<&Path> = vec![
        dir.as_path(),
        check_dir.as_path(),
        git_dir.as_path(),
        model_dir.as_path(),
        create_dir.as_path(),
        query_dir.as_path(),
        transcript_dir.as_path(),
    ];
    // Every library this binary links, so a stale copy of any is refused. The first covers the
    // binary too, since the two are one package.
    let library = |crate_dir: &str, package| knowledge_architect::cli::Library {
        crate_dir: std::path::PathBuf::from(crate_dir),
        package,
    };
    let libraries = [
        knowledge_architect::cli::this_library(),
        library(
            knowledge_architect_check::CRATE_DIR,
            knowledge_architect_check::PACKAGE,
        ),
        library(
            knowledge_architect_git::CRATE_DIR,
            knowledge_architect_git::PACKAGE,
        ),
        library(
            knowledge_architect_model::CRATE_DIR,
            knowledge_architect_model::PACKAGE,
        ),
        library(
            knowledge_architect_create::CRATE_DIR,
            knowledge_architect_create::PACKAGE,
        ),
        library(
            knowledge_architect_query::CRATE_DIR,
            knowledge_architect_query::PACKAGE,
        ),
        library(
            knowledge_architect_transcript::CRATE_DIR,
            knowledge_architect_transcript::PACKAGE,
        ),
        library(
            knowledge_architect_agent_skills::CRATE_DIR,
            knowledge_architect_agent_skills::PACKAGE,
        ),
    ];
    let packages = [
        env!("CARGO_PKG_NAME"),
        knowledge_architect_check::PACKAGE,
        knowledge_architect_git::PACKAGE,
        knowledge_architect_model::PACKAGE,
        knowledge_architect_create::PACKAGE,
        knowledge_architect_query::PACKAGE,
        knowledge_architect_transcript::PACKAGE,
        knowledge_architect_agent_skills::PACKAGE,
    ];
    // `init` makes the project the other commands locate, so it is dispatched first, per
    // `design@cli@the-commands-are-a-library-module`.
    let outcome = match cli.command {
        knowledge_architect::cli::Command::Init(args) => {
            knowledge_architect::cli::init(args, &libraries, &packages, &dirs, &mut [])
        }
        command => knowledge_architect::cli::locate().and_then(|manifest| {
            // A binary built from another checkout would judge this tree with that checkout's
            // code, so no command runs, per `design@check@a-foreign-build-is-refused`.
            knowledge_architect::cli::refuse_a_foreign_build(
                manifest.root(),
                &libraries,
                &packages,
            )?;
            // A binary of another version than the project pins would judge it by other rules, or
            // install other skills over it, per `design@check@installed-binary-version-check`.
            knowledge_architect::cli::refuse_another_version(&manifest, &dirs)?;
            knowledge_architect::cli::run(command, &manifest, &dirs, &mut [])
        }),
    };
    match outcome {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // clap's own consistency check over the whole derive, `check`'s commands flattened in:
    // two arguments claiming one name, a conflict naming an argument that does not exist. A
    // malformed declaration is otherwise found at the first invocation that reaches it, per
    // `design@cli@arguments-parse-through-clap`.
    #[test]
    fn cli_declaration_is_consistent() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    /// The claim: an argument nobody declared is refused while parsing, which exits 2, rather
    /// than ignored.
    #[test]
    fn an_unknown_argument_is_refused() {
        assert!(Cli::try_parse_from(["klarch", "check"]).is_ok());
        assert!(Cli::try_parse_from(["klarch", "check", "--only", "references"]).is_err());
        assert!(Cli::try_parse_from(["klarch", "no-such-command"]).is_err());
    }

    /// The claim: a required argument is required while parsing, on the command that declares it.
    #[test]
    fn id_requires_an_id_while_parsing() {
        assert!(Cli::try_parse_from(["klarch", "id"]).is_err());
        assert!(Cli::try_parse_from(["klarch", "id", "an-id"]).is_ok());
    }

    /// The claim: `worklist`'s combinations are refused while parsing, not only by `cli::run`'s
    /// conversion: both directories, neither, and a bound or a seed beside `--verify`.
    #[test]
    fn worklist_combinations_are_refused_while_parsing() {
        let parse = |args: &[&str]| Cli::try_parse_from([&["klarch", "worklist"], args].concat());
        assert!(parse(&["--out", "d"]).is_ok());
        assert!(parse(&["--verify", "d"]).is_ok());
        assert!(parse(&[]).is_err());
        assert!(parse(&["--out", "d", "--verify", "d"]).is_err());
        assert!(parse(&["--verify", "d", "--bound", "5"]).is_err());
        assert!(parse(&["--verify", "d", "--seed", "s"]).is_err());
    }

    /// The claim: `--fix` and `--staged` are refused together while parsing, since `--fix`
    /// repairs the working tree and `--staged` judges the index.
    #[test]
    fn fix_and_staged_are_refused_together() {
        assert!(Cli::try_parse_from(["klarch", "check", "--staged"]).is_ok());
        assert!(Cli::try_parse_from(["klarch", "check", "--fix"]).is_ok());
        assert!(Cli::try_parse_from(["klarch", "check", "--fix", "--staged"]).is_err());
    }
}
