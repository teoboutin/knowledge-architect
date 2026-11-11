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

use std::io::Write;
use std::process::{Command, Stdio};

/// The text `node --check` reads for one installed workflow.
fn wrapped(text: &str) -> String {
    let body: Vec<&str> = text
        .lines()
        .map(|l| l.strip_prefix("export ").unwrap_or(l))
        .collect();
    format!("async function body() {{\n{}\n}}\n", body.join("\n"))
}

/// What `node --check` says of `text` once wrapped: `Ok` when it parses, `Err` with node's
/// message when it does not, and `None` when no `node` can be started.
fn node_check(text: &str) -> Option<Result<(), String>> {
    let mut child = Command::new("node")
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

/// The variable `cargo x gates --ci` sets, which CI passes, per
/// `design@xtask@gates-list-primary-home`: the one switch of every requirement CI holds and a local
/// run does not.
const CI_VAR: &str = "KNOWLEDGE_ARCHITECT_CI";

/// Where no `node` can be started: a skip a local run reports, and a failure where the run requires
/// `node`, so a CI runner without it fails the gate instead of passing it with nothing parsed.
fn without_node(required: bool) -> Result<&'static str, &'static str> {
    if required {
        Err(
            "no `node` can be started, and `KNOWLEDGE_ARCHITECT_CI` is set, as `cargo x gates --ci` \
             sets it: the saved workflows must be parsed",
        )
    } else {
        Ok("skipped: no `node` can be started, so no saved workflow was parsed")
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
/// `node`, a local run is skipped with a notice, and a run under `cargo x gates --ci`
/// fails, by `without_node`.
/// Mutation checked: in `wrapped`, `.map(|l| l.strip_prefix("export ").unwrap_or(l))` made
/// `.map(|l| l)`, which leaves `export` inside the function; caught.
#[test]
fn every_installed_saved_workflow_parses() {
    let all = workflows();
    assert!(!all.is_empty(), "no saved workflow is installed");
    for (path, text) in all {
        match node_check(text) {
            None => match without_node(std::env::var_os(CI_VAR).is_some()) {
                Ok(notice) => {
                    eprintln!("{notice}");
                    return;
                }
                Err(failure) => panic!("{failure}"),
            },
            Some(Ok(())) => {}
            Some(Err(message)) => panic!("{path} does not parse:\n{message}"),
        }
    }
}

/// The claim: the check discriminates. An installed workflow with one stray `)` planted on a
/// line fails it, and the same text with a top-level `return`, legal in the harness, passes.
/// Mutation checked: in `node_check`, `Some(if out.status.success() {` made `Some(if true {`;
/// caught.
#[test]
fn the_parse_check_fails_on_a_planted_syntax_error() {
    let (_, text) = workflows().into_iter().next().expect("a saved workflow");
    let Some(passing) = node_check(&format!("{text}\nreturn 1\n")) else {
        eprintln!("{}", without_node(false).expect("a local notice"));
        return;
    };
    assert_eq!(passing, Ok(()));
    let planted = text.replacen('\n', "\n)\n", 1);
    assert!(node_check(&planted).expect("node ran").is_err());
}

/// The claim: without `node`, a run that requires it fails and any other is skipped. Mutation
/// checked: in `without_node`, `if required {` made `if false {`; caught.
#[test]
fn a_missing_node_fails_where_required_and_skips_otherwise() {
    assert!(without_node(true).is_err());
    assert!(without_node(false).is_ok());
}
