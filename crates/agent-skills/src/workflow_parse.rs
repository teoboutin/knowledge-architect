//! The tests that run `node` on the saved workflows the installer writes: the parse check of every
//! one, and the tests of the record search's flag builder and of its merge of failing reports.
//!
//! The harness runs a saved workflow's body as an async function, so a top-level `return` and
//! `await` are legal there and a parser refuses them on the bare file. The parse check wraps the
//! installed text, its `%%` lines already removed by the build, in `async function body() {…}`,
//! with `export` removed from the start of each line, and hands it to `node --check` on stdin. Its
//! domain of validity: a syntax error, which node reports for that wrapped text. It runs no part of
//! the script, and it checks nothing the harness checks of `meta` beyond its parse.
//!
//! The tests of the record search's workflow run two parts of its script: `worklistCommand`, the
//! flag builder, with `quoted`, on fixed `args`, and `failingReports`, the merge of the failing
//! reports, with `agentName`, on fixed returns of the check's agent. Each function is taken from the
//! installed text, `{{command}}` replaced by a command of the test's, and run under `node`. The
//! script's body, which calls the harness's `agent` and `phase`, never runs; the rest of its logic
//! is judged by no test, per `issue@agent-skills@no-test-runs-the-record-search-workflow-s-body`.
//!
//! Why `node` and not a JavaScript parser compiled into the test: a parser crate would run on every
//! machine, but it adds a large dev-dependency to every build of the suite, and it parses only,
//! where testing what a workflow computes needs an engine that runs the script. The cost taken
//! instead: a machine without `node` skips both, and `cargo x gates --ci`, which CI runs, fails them
//! there, per `design@gates@ci-flag-for-ci-only-requirements`.

use std::io::Write;
use std::process::{Command, Stdio};

/// The variable the gates library's `--ci` sets in every gate's environment, spelled here as the
/// library spells it, per `design@gates@ci-flag-for-ci-only-requirements`: this crate depends on
/// no other. A test of xtask, `the_parse_test_reads_the_library_s_ci_variable`, reads this line and
/// fails where the two spellings differ.
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

/// What `<program> <args>` does with `input` on its stdin: `Ok` with its stdout when it exits 0,
/// `Err` with its stderr when it does not, and `None` when `program` cannot be started.
fn node_run(program: &str, args: &[&str], input: &str) -> Option<Result<String, String>> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    child
        .stdin
        .take()
        .expect("a piped stdin")
        .write_all(input.as_bytes())
        .expect("node reads the script");
    let out = child.wait_with_output().expect("node ran");
    Some(if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    })
}

/// What `<program> --check` says of `text` once wrapped: `Ok` when it parses, `Err` with its
/// message when it does not, and `None` when `program` cannot be started.
fn node_check(program: &str, text: &str) -> Option<Result<(), String>> {
    node_run(program, &["--check", "-"], &wrapped(text)).map(|r| r.map(|_| ()))
}

/// Whether this run requires `node`: the variable `--ci` sets. Read in this one place, the side
/// effect of the decision `checked` takes as an argument.
fn required() -> bool {
    std::env::var_os(CI_VAR).is_some()
}

/// What a run that skips prints, on its own stderr, which `cargo test` shows under `--nocapture`.
const SKIPPED: &str = "skipped: no `node` can be started, so no saved workflow was run";

/// The outcome of a check that needed `node`: `Ok(Some(_))` when it ran, `Ok(None)` when no
/// `node` could be started and the run does not require it, and `Err` when the run requires it,
/// so a CI runner without `node` fails the gate instead of passing it with nothing parsed.
fn checked<T>(outcome: Option<T>, required: bool) -> Result<Option<T>, &'static str> {
    match outcome {
        Some(result) => Ok(Some(result)),
        None if required => Err(
            "no `node` can be started, and `KNOWLEDGE_ARCHITECT_CI` is set, as `cargo x gates --ci` \
             sets it: the saved workflows must be run",
        ),
        None => Ok(None),
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
            Ok(None) => return eprintln!("{SKIPPED}"),
            Ok(Some(Ok(()))) => {}
            Ok(Some(Err(message))) => panic!("{path} does not parse:\n{message}"),
        }
    }
}

/// The claim: the check discriminates. Each installed workflow with one stray `)` planted on a
/// line fails it, and the same text with a top-level `return`, legal in the harness, passes.
/// Mutation checked: in `node_run`, which `node_check` calls, `Some(if out.status.success() {` made
/// `Some(if true {`; caught.
#[test]
fn the_parse_check_fails_on_a_planted_syntax_error() {
    for (path, text) in workflows() {
        let passing = match checked(node_check(NODE, &format!("{text}\nreturn 1\n")), required()) {
            Err(failure) => panic!("{failure}"),
            Ok(None) => return eprintln!("{SKIPPED}"),
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

/// The installed text of the record search's workflow.
fn record_search() -> &'static str {
    workflows()
        .into_iter()
        .find(|(path, _)| *path == ".claude/workflows/knowledge-architect-record-search.js")
        .expect("the record search's workflow is installed")
        .1
}

/// The source of the top-level function `name` of `text`: from its line `function <name>(` to the
/// next line that is a lone `}`, both included. It panics where the script holds no such function,
/// so a reshaped function fails the test instead of leaving it to judge nothing.
fn function_source(text: &str, name: &str) -> String {
    let opening = format!("function {name}(");
    let mut taken = text.lines().skip_while(|l| !l.starts_with(&opening));
    let mut source = Vec::new();
    for line in taken.by_ref() {
        source.push(line);
        if line == "}" {
            return source.join("\n");
        }
    }
    panic!("no top-level function `{name}`, closed by a lone `}}`, in the script")
}

/// What `node` prints on stdout for the top-level `functions` of the installed record search,
/// `{{command}}` replaced by `klarch`, followed by the statement `run`; `None` where no `node` can
/// be started and the run does not require it.
fn record_search_run(functions: &[&str], run: &str) -> Option<String> {
    let text = record_search().replace("{{command}}", "klarch");
    let mut script: String = functions
        .iter()
        .map(|name| format!("{}\n", function_source(&text, name)))
        .collect();
    script.push_str(run);
    match checked(node_run(NODE, &["-"], &script), required()) {
        Err(failure) => panic!("{failure}"),
        Ok(None) => None,
        Ok(Some(Err(message))) => panic!("{functions:?} did not run:\n{message}"),
        Ok(Some(Ok(stdout))) => Some(stdout),
    }
}

/// What `worklistCommand` of the installed record search returns for each of `cases`, a JavaScript
/// array of `args` objects, `{{command}}` replaced by `klarch`; `None` where no `node` can be
/// started and the run does not require it.
fn worklist_commands(cases: &str) -> Option<Vec<String>> {
    let run = format!("process.stdout.write({cases}.map(worklistCommand).join('\\n'))\n");
    let stdout = record_search_run(&["quoted", "worklistCommand"], &run)?;
    Some(stdout.lines().map(str::to_string).collect())
}

/// The claim: `worklistCommand` maps every argument of the workflow in a fixed order, `--out`, then
/// `--work`, `--subject`, `--seed`, `--since`, `--seeds-from` and `--bound`, each element once and
/// single-quoted, each inner `'` written `'\''`, so a backticked seed runs no substitution; no
/// `subjects` and no `seeds` add no flag; and an empty `since` is passed on, for `--out` to refuse.
/// Mutation checked: in `worklistCommand`, `for (const s of a.seeds || [])` made
/// `for (const s of (a.seeds || []).slice(0, -1))`, which drops the last seed; caught. And in
/// `quoted`, `String(value).replace(/'/g, "'\\''")` made `String(value)`, and the same made
/// `String(value).replace(/'/, "'\\''")`, which escapes the first `'` alone; each caught. And
/// `if (a.since !== undefined)` made `if (a.since)`, which drops an empty `since`; caught.
#[test]
fn the_record_search_s_flag_builder_maps_every_argument() {
    // Bound by a `const`, which the checker does not read: a seed here names no entry.
    const EXPECTED: [&str; 2] = [
        r#"klarch worklist --out '/s/a b' --work 'w.md' --work 'it'\''s o'\''k.md' --subject 't-one' --subject 't-two' --seed '`design@x@one`' --seed 'issue@x@two' --since 'abc123' --seeds-from 'docs/plans/specs/s/README.md' --bound '30000'"#,
        "klarch worklist --out '/s/b' --work 'w.md' --since ''",
    ];
    let cases = r#"[
        { scratch: '/s/a b', work: ['w.md', "it's o'k.md"], subjects: ['t-one', 't-two'],
          seeds: ['`design@x@one`', 'issue@x@two'], since: 'abc123',
          seedsFrom: 'docs/plans/specs/s/README.md', bound: 30000, maxRounds: 2 },
        { scratch: '/s/b', work: ['w.md'], since: '' },
    ]"#;
    let Some(got) = worklist_commands(cases) else {
        return eprintln!("{SKIPPED}");
    };
    assert_eq!(got, EXPECTED);
}

/// What `failingReports` of the installed record search returns for each of `cases`, a JavaScript
/// array of returns of the check's agent, each as one line of JSON; `None` where no `node` can be
/// started and the run does not require it.
fn failing_reports(cases: &str) -> Option<Vec<String>> {
    let run = format!(
        "process.stdout.write({cases}.map((c) => JSON.stringify(failingReports(c))).join('\\n'))\n"
    );
    let stdout = record_search_run(&["agentName", "failingReports"], &run)?;
    Some(stdout.lines().map(str::to_string).collect())
}

/// The claim: `failingReports` sends back each report the check's output names, an agent's report
/// once with its findings in order and without a repeated line, whether or not the check's agent
/// listed it, the agent named by the last `agent-<n>` of a path it returned; and the `failing` it
/// gives the workflow to return is the list it sends back, or, where it sends back none, the
/// agent's own list. Mutation checked: in `failingReports`, `return { sendBack, failing: sendBack.length ?
/// sendBack : checked.failing }` made `return { sendBack, failing: checked.failing }`, the defect
/// the commit "[agent-skills] The record search's workflow returns the failing reports it sent
/// back, not only those the check's agent listed" repaired; caught. And the scan of the output,
/// `if (m) add(m[1], l)` made `if (false) add(m[1], l)`, which sends back only what the agent
/// listed; caught. And `sendBack.length ? sendBack : checked.failing` made `sendBack`, which
/// returns an empty list where the output names no report; caught. And
/// `if (l && !byAgent.get(name).includes(l))` made `if (l)`, which repeats a line the agent and the
/// output both give; caught.
#[test]
fn the_record_search_s_failing_reports_merge_the_check_s_output() {
    // Bound by a `const`, which the checker does not read.
    const ONE: &str = r#"{"agent":"agent-1","findings":"/s/agent-1/report.md:3: no quotation\n/s/agent-1/report.md:9: no reason"}"#;
    const TWO: &str =
        r#"{"agent":"agent-2","findings":"error: /s/agent-2/report.md:7: not a line"}"#;
    let expected = [
        format!(r#"{{"sendBack":[{ONE},{TWO}],"failing":[{ONE},{TWO}]}}"#),
        r#"{"sendBack":[],"failing":[{"agent":"the work list","findings":"error: /s/work.txt: unreadable"}]}"#
            .to_string(),
    ];
    let cases = r#"[
        { passed: false,
          output: '/s/agent-1/report.md:3: no quotation\nerror: /s/agent-2/report.md:7: not a line\n/s/agent-1/report.md:9: no reason\nFAILED',
          failing: [{ agent: '/s/agent-1', findings: '/s/agent-1/report.md:3: no quotation' }] },
        { passed: false,
          output: 'error: /s/work.txt: unreadable',
          failing: [{ agent: 'the work list', findings: 'error: /s/work.txt: unreadable' }] },
    ]"#;
    let Some(got) = failing_reports(cases) else {
        return eprintln!("{SKIPPED}");
    };
    assert_eq!(got, expected);
}
