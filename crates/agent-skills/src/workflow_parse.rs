//! The parse check of every saved workflow the installer writes.
//!
//! The harness runs a saved workflow's body as an async function, so a top-level `return` and
//! `await` are legal there and a parser refuses them on the bare file. The check wraps the
//! installed text, its `%%` lines already removed by the build, in `async function body() {…}`,
//! with `export` removed from the start of each line, and hands it to `node --check` on stdin.
//!
//! Its domain of validity: a syntax error, which node reports for that wrapped text. It runs no
//! part of the script and judges none of its logic, and it checks nothing the harness checks of
//! `meta` beyond its parse.
//!
//! Why `node` and not a JavaScript parser compiled into the test: a parser crate would run on every
//! machine, but it adds a large dev-dependency to every build of the suite, and it parses only,
//! where testing what the audit workflows compute needs an engine that runs the script. The cost
//! taken instead: a machine without `node` skips the check, and `cargo x gates --ci`, which CI
//! runs, fails it there, per `design@gates@ci-flag-for-ci-only-requirements`.

use std::io::Write;
use std::process::{Command, Stdio};

/// The variable the gates library's `--ci` sets in every gate's environment, spelled here as the
/// library spells it, per `design@gates@ci-flag-for-ci-only-requirements`: this crate depends on
/// no other.
const CI_VAR: &str = "KNOWLEDGE_ARCHITECT_CI";

/// The program the check runs.
const NODE: &str = "node";

/// The text `node --check` reads for one installed workflow.
fn wrapped(text: &str) -> String {
    let body: Vec<&str> = text
        .lines()
        .map(|l| l.strip_prefix("export ").unwrap_or(l))
        .collect();
    format!("async function body() {{\n{}\n}}\n", body.join("\n"))
}

/// What `<program> --check` says of `text` once wrapped: `Ok` when it parses, `Err` with its
/// message when it does not, and `None` when `program` cannot be started.
fn node_check(program: &str, text: &str) -> Option<Result<(), String>> {
    let mut child = Command::new(program)
        .args(["--check", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    child
        .stdin
        .take()
        .expect("a piped stdin")
        .write_all(wrapped(text).as_bytes())
        .expect("node reads the script");
    let out = child.wait_with_output().expect("node ran");
    Some(if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    })
}

/// Whether this run requires `node`: the variable `--ci` sets. Read in this one place, the side
/// effect of the decision `checked` takes as an argument.
fn required() -> bool {
    std::env::var_os(CI_VAR).is_some()
}

/// The outcome of a check that needed `node`: `Ok(Some(_))` when it ran, `Ok(None)` when no
/// `node` could be started and the run does not require it, and `Err` when the run requires it,
/// so a CI runner without `node` fails the gate instead of passing it with nothing parsed.
fn checked<T>(outcome: Option<T>, required: bool) -> Result<Option<T>, &'static str> {
    match outcome {
        Some(result) => Ok(Some(result)),
        None if required => Err(
            "no `node` can be started, and `KNOWLEDGE_ARCHITECT_CI` is set, as `cargo x gates --ci` \
             sets it: the saved workflows must be parsed",
        ),
        None => {
            eprintln!("skipped: no `node` can be started, so no saved workflow was parsed");
            Ok(None)
        }
    }
}

/// The installed saved workflows, by install path and text.
fn workflows() -> Vec<(&'static str, &'static str)> {
    crate::FILES
        .iter()
        .filter(|(path, _)| path.starts_with(".claude/workflows/") && path.ends_with(".js"))
        .copied()
        .collect()
}

/// The claim: every saved workflow the installer writes parses, as the harness runs it. Without
/// `node`, a run is skipped with a notice, and a run under `cargo x gates --ci` fails, by
/// `checked`.
#[test]
fn every_installed_saved_workflow_parses() {
    let all = workflows();
    assert!(!all.is_empty(), "no saved workflow is installed");
    for (path, text) in all {
        match checked(node_check(NODE, text), required()) {
            Err(failure) => panic!("{failure}"),
            Ok(None) => return,
            Ok(Some(Ok(()))) => {}
            Ok(Some(Err(message))) => panic!("{path} does not parse:\n{message}"),
        }
    }
}

/// The claim: the check discriminates. Each installed workflow with one stray `)` planted on a
/// line fails it, and the same text with a top-level `return`, legal in the harness, passes.
/// Mutation checked: in `node_check`, `Some(if out.status.success() {` made `Some(if true {`;
/// caught.
#[test]
fn the_parse_check_fails_on_a_planted_syntax_error() {
    for (path, text) in workflows() {
        let passing = match checked(node_check(NODE, &format!("{text}\nreturn 1\n")), required()) {
            Err(failure) => panic!("{failure}"),
            Ok(None) => return,
            Ok(Some(result)) => result,
        };
        assert_eq!(passing, Ok(()), "{path}");
        let planted = text.replacen('\n', "\n)\n", 1);
        assert!(
            node_check(NODE, &planted).expect("node ran").is_err(),
            "{path}"
        );
    }
}

/// The claim: where the program cannot be started, a run that requires it fails and any other is
/// skipped, and a program that ran gives its result through. Mutation checked: in `checked`,
/// `None if required =>` made `None if false =>`; caught.
#[test]
fn a_missing_node_fails_where_required_and_skips_otherwise() {
    let absent = "knowledge-architect-no-such-program";
    assert!(checked(node_check(absent, "x"), true).is_err());
    assert_eq!(checked(node_check(absent, "x"), false), Ok(None));
    assert_eq!(checked(Some(1), true), Ok(Some(1)));
}

/// The claim: the wrapping removes `export` at the start of a line only, and puts the text inside
/// an async function. Mutation checked: `.map(|l| l.strip_prefix("export ").unwrap_or(l))` made
/// `.map(|l| l)`; caught.
#[test]
fn the_wrapping_drops_export_and_encloses_the_body() {
    assert_eq!(
        wrapped("export const meta = 1\nconst exported = 'export x'"),
        "async function body() {\nconst meta = 1\nconst exported = 'export x'\n}\n"
    );
}
