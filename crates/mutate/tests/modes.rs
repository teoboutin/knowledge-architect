//! The `run` and `restore` modes end to end, each in a process of its own.
//!
//! `harness.rs` covers the harness; claiming the record, the write path, the read-back
//! assertion, the unwind and `restore` are reachable only by running a mode, and a kill is
//! observable only on a process. The crate holds a library alone, so each run re-enters this test
//! executable as its child: `child_entry` parses the arguments a maintenance binary would, runs
//! the mode over its working directory, and exits with the mode's code.
//!
//! Every test builds its own sandbox, a git repository in the temporary directory, removed when
//! the test ends, so a run's record and target stay inside it and never touch this repository.
//! **The command under `--` is always a shell builtin or `sh -c`, never `cargo`**: a nested cargo
//! would rebuild the workspace inside a test and contend for the same target directory.

mod links;
mod live;

use links::{link_dir, link_file};
use live::Live;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Sandbox {
    dir: tempfile::TempDir,
}

impl Sandbox {
    fn new(tag: &str) -> Self {
        let dir = tempfile::TempDir::with_prefix(format!("klarch-mutate-modes-{tag}-"))
            .expect("creating the sandbox");
        // The record lives in the git directory, so the sandbox is a repository of its own.
        let init = Command::new("git")
            .args(["init", "-q", "."])
            .current_dir(dir.path())
            .status()
            .expect("running git init");
        assert!(init.success(), "git init");
        Sandbox { dir }
    }

    fn root(&self) -> &Path {
        self.dir.path()
    }

    fn write(&self, rel: &str, content: &str) -> PathBuf {
        let p = self.root().join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("creating a parent directory");
        }
        fs::write(&p, content).expect("writing a sandbox file");
        p
    }

    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root().join(rel)).expect("reading a sandbox file")
    }

    /// Runs the binary in the sandbox. Returns (success, stdout and stderr together).
    fn run(&self, args: &[&str]) -> (bool, String) {
        let (ok, text, _) = self.run_coded(args);
        (ok, text)
    }

    /// The same, keeping the exit code. Success alone cannot tell 1 from 2, and the whole
    /// point of the ladder is that those two say different things about the tree.
    fn run_coded(&self, args: &[&str]) -> (bool, String, i32) {
        let out = child(self.root(), args)
            .output()
            .expect("running the child");
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));
        (out.status.success(), text, out.status.code().unwrap_or(-1))
    }

    fn record(&self) -> PathBuf {
        self.root().join(".git/knowledge-architect-mutate")
    }
}

const FILE: &str = "src/f.rs";
const BODY: &str = "line1\nline2\nSITE\nline4\nline5\n";

#[test]
fn a_clean_run_restores_the_file_and_leaves_no_record() {
    let s = Sandbox::new("clean");
    s.write(FILE, BODY);
    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--",
        "true",
    ]);
    assert!(ok, "{out}");
    assert!(out.contains("mutation applied"), "{out}");
    assert!(out.contains("SURVIVED"), "{out}");
    assert_eq!(s.read(FILE), BODY, "the file must come back byte-exact");
    assert!(!s.record().exists(), "no record may be left behind");
}

#[test]
fn a_failing_command_reads_as_caught() {
    let s = Sandbox::new("caught");
    s.write(FILE, BODY);
    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "false",
    ]);
    assert!(ok, "{out}");
    assert!(out.contains("caught"), "{out}");
    assert_eq!(s.read(FILE), BODY);
}

#[test]
fn a_non_unique_pattern_refuses_and_touches_nothing() {
    let s = Sandbox::new("nonunique");
    s.write(FILE, "x\nx\n");
    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "x",
        "--with",
        "y",
        "--",
        "true",
    ]);
    assert!(!ok, "{out}");
    assert!(out.contains("occurs 2 times"), "{out}");
    assert_eq!(s.read(FILE), "x\nx\n");
    assert!(
        !s.record().exists(),
        "a refusal must not leave a record blocking later runs"
    );
}

#[test]
fn an_absolute_target_is_refused() {
    let s = Sandbox::new("abs");
    let outside = s.write("outside.txt", "outside\n");
    let (ok, out) = s.run(&[
        "run",
        "--file",
        outside.to_str().expect("utf-8 path"),
        "--replace",
        "outside",
        "--with",
        "INSIDE",
        "--",
        "true",
    ]);
    assert!(!ok, "{out}");
    assert!(out.contains("absolute path"), "{out}");
    assert_eq!(s.read("outside.txt"), "outside\n");
}

#[test]
fn a_symlinked_target_is_refused() {
    let s = Sandbox::new("symlink");
    s.write("real.txt", "real\n");
    link_file(s.root().join("real.txt"), s.root().join("link.txt"));
    let (ok, out) = s.run(&[
        "run",
        "--file",
        "link.txt",
        "--replace",
        "real",
        "--with",
        "MUT",
        "--",
        "true",
    ]);
    assert!(!ok, "{out}");
    assert!(out.contains("symlink"), "{out}");
    assert_eq!(s.read("real.txt"), "real\n");
}

#[test]
fn a_red_baseline_refuses_before_anything_is_written() {
    let s = Sandbox::new("baseline");
    s.write(FILE, BODY);
    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--",
        "false",
    ]);
    assert!(!ok, "{out}");
    assert!(
        out.contains("already fails over the unmutated tree"),
        "{out}"
    );
    assert_eq!(s.read(FILE), BODY);
    assert!(!s.record().exists());
}

/// The claim, per `design@mutate@baseline-runs-by-default`: with `--build`, the baseline runs the
/// probe over the unmutated tree before the command, so a command that builds nothing takes its
/// baseline on the unmutated build, not on a previous run's mutant left in place. The build here
/// copies the file to `built`, and the command reads `built` alone. Mutation checked: the probe
/// before the baseline removed makes the baseline read the stale mutant and refuse the run.
#[test]
fn the_baseline_runs_the_build_probe_over_the_unmutated_tree() {
    let s = Sandbox::new("baseline-build");
    s.write(FILE, BODY);
    s.write("built", "a previous run's mutant, MUT\n");
    let (ok, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--build",
        &format!("cp {FILE} built"),
        "--",
        "grep",
        "-q",
        "SITE",
        "built",
    ]);
    assert!(ok, "{code}: {out}");
    assert!(out.contains("baseline: build probe:"), "{out}");
    assert!(out.contains(": caught"), "{out}");
    assert_eq!(s.read(FILE), BODY);
}

/// The claim: a build probe that fails, or cannot be spawned, over the unmutated tree refuses the
/// run before the target is written, exit 2, as a red baseline does. Mutations checked: the
/// refusal's arm of a failing probe made a no-op, and that of a probe that cannot start made a
/// no-op, each runs the mutation.
#[test]
fn a_build_probe_failing_over_the_unmutated_tree_refuses_the_run() {
    for probe in ["false", "no-such-program-of-this-test"] {
        let s = Sandbox::new("baseline-build-red");
        s.write(FILE, BODY);
        let (_, out, code) = s.run_coded(&[
            "run",
            "--file",
            FILE,
            "--replace",
            "SITE",
            "--with",
            "MUT",
            "--build",
            probe,
            "--",
            "true",
        ]);
        assert_eq!(code, 2, "{probe}: {out}");
        assert!(!out.contains("mutation applied"), "{probe}: {out}");
        assert_eq!(s.read(FILE), BODY);
        assert!(!s.record().exists());
    }
}

#[test]
fn a_failing_build_probe_reports_unmutatable_rather_than_caught() {
    // The verdict this separates: a command that failed because a test noticed, from one that
    // failed because the mutant never compiled.
    let s = Sandbox::new("probe");
    s.write(FILE, BODY);
    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--build",
        "false",
        "--",
        "false",
    ]);
    assert!(ok, "{out}");
    assert!(out.contains("unmutatable here"), "{out}");
    assert!(
        !out.contains(": caught"),
        "a compile failure is not a caught mutation: {out}"
    );
    assert_eq!(s.read(FILE), BODY);
}

#[test]
fn without_a_build_probe_the_caught_verdict_says_what_it_cannot_tell() {
    let s = Sandbox::new("noprobe");
    s.write(FILE, BODY);
    let (_, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "false",
    ]);
    assert!(out.contains("no --build probe ran"), "{out}");
}

#[test]
fn a_command_that_edits_the_file_stops_the_restore_and_keeps_both_changes() {
    let s = Sandbox::new("edited");
    s.write(FILE, BODY);
    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "printf 'appended\\n' >> src/f.rs",
    ]);
    assert!(!ok, "an incomplete unwind must not exit 0: {out}");
    assert!(out.contains("Nothing has been written"), "{out}");
    let after = s.read(FILE);
    assert!(after.contains("MUT"), "the mutation stays: {after}");
    assert!(
        after.contains("appended"),
        "the concurrent edit stays: {after}"
    );
    assert!(
        s.record().join("pre-image").is_file(),
        "the pre-image must survive"
    );
}

#[test]
fn a_duplicated_region_does_not_discharge_the_record() {
    // `git apply --check` succeeding was tried as a proof that the mutant had gone. It is not
    // one: a patch applies wherever its text can be placed, at any offset, so a command that
    // copies the pre-mutation region satisfies it while the mutant sits at the site. Discharging
    // on that deleted the only copy of the pre-image and reported success.
    let s = Sandbox::new("dup");
    let body: String = (1..=20).map(|n| format!("line {n}\n")).collect();
    let body = body.replace("line 10", "SITE");
    s.write(FILE, &body);
    s.write("copy.txt", &body);

    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "cat copy.txt >> src/f.rs",
    ]);
    assert!(!ok, "{out}");
    assert!(
        s.read(FILE).contains("MUT"),
        "the mutation is still applied"
    );
    assert!(
        s.record().join("pre-image").is_file(),
        "the pre-image must not be deleted while the mutation is live"
    );
}

#[test]
fn a_command_that_deletes_the_target_gets_the_pre_image_written_back() {
    let s = Sandbox::new("deleted");
    s.write(FILE, BODY);
    let (_, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "rm src/f.rs",
    ]);
    assert!(out.contains("was gone"), "{out}");
    assert_eq!(
        s.read(FILE),
        BODY,
        "the file comes back rather than stranding the record"
    );
    assert!(!s.record().exists());
}

#[test]
fn a_second_run_is_refused_while_a_record_stands() {
    let s = Sandbox::new("second");
    s.write(FILE, BODY);
    s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "printf 'x\\n' >> src/f.rs",
    ]);
    assert!(s.record().exists(), "the first run leaves its record");

    let (ok, out) = s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "line1",
        "--with",
        "L1",
        "--no-baseline",
        "--",
        "true",
    ]);
    assert!(!ok, "{out}");
    assert!(out.contains("cannot claim"), "{out}");
    assert_eq!(
        fs::read_to_string(s.record().join("pre-image")).expect("pre-image"),
        BODY,
        "the first run's pre-image must not be overwritten"
    );
}

// The liveness of a process on Windows is `issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`.
#[test]
#[cfg_attr(
    windows,
    ignore = "the liveness of a run is read wrongly on Windows, which is development workflow"
)]
fn restore_refuses_while_the_owning_run_is_alive() {
    // Built by hand rather than by racing two runs: the guard is what is under test, and the owner is
    // a child of this test. A restore here would put the pre-image back underneath a live run, whose
    // command would then be measured over unmutated code.
    let s = Sandbox::new("alive");
    s.write(FILE, BODY);
    s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "printf 'x\\n' >> src/f.rs",
    ]);
    let live = Live::start();
    fs::write(s.record().join("owner-pid"), live.pid().to_string()).expect("writing a live owner");

    let (ok, out) = s.run(&["restore"]);
    assert!(!ok, "{out}");
    assert!(out.contains("still going"), "{out}");
    let (ok, out) = s.run(&["restore", "--discard"]);
    assert!(!ok, "discard must be guarded too: {out}");
    assert!(s.record().join("pre-image").is_file());
}

#[test]
fn restore_discard_drops_the_record_without_writing() {
    let s = Sandbox::new("discard");
    s.write(FILE, BODY);
    s.run(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "printf 'x\\n' >> src/f.rs",
    ]);
    let before = s.read(FILE);

    let (ok, out) = s.run(&["restore", "--discard"]);
    assert!(ok, "{out}");
    assert!(out.contains("no file was written"), "{out}");
    assert_eq!(s.read(FILE), before, "discard writes nothing");
    assert!(!s.record().exists());
}

#[test]
fn restore_with_no_record_says_so_and_succeeds() {
    let s = Sandbox::new("norecord");
    let (ok, out) = s.run(&["restore"]);
    assert!(ok, "{out}");
    assert!(out.contains("nothing to restore"), "{out}");
}

#[test]
fn an_incomplete_record_is_reported_rather_than_restored() {
    let s = Sandbox::new("incomplete");
    fs::create_dir_all(s.record()).expect("claiming by hand");
    let (ok, out) = s.run(&["restore"]);
    assert!(!ok, "{out}");
    assert!(out.contains("incomplete"), "{out}");

    let (ok, out) = s.run(&["restore", "--discard"]);
    assert!(
        ok,
        "discard clears a claim that never became a record: {out}"
    );
    assert!(!Path::new(&s.record()).exists());
}

/// The claim: an argument the declaration does not admit exits 2 and touches nothing.
///
/// 2 rather than merely nonzero, because `run`'s 1 means its run did not complete and the tree
/// or the record is owed a look. While a usage error shared that code, that signal could not be
/// read. Both halves are the table of the crate's documentation.
#[test]
fn an_argument_the_declaration_does_not_admit_exits_two() {
    let s = Sandbox::new("usage");
    let invalid: [&[&str]; 7] = [
        // An unknown flag, on each subcommand.
        &["run", "--nonsense", "x", "--", "true"],
        &["restore", "--bogus"],
        // No subcommand at all: `cargo x mutate` alone prints the help and starts nothing.
        &[],
        // Neither mutation form, both forms, and half of one.
        &["run", "--file", FILE, "--", "true"],
        &[
            "run",
            "--file",
            FILE,
            "--replace",
            "a",
            "--patch",
            "p.diff",
            "--",
            "true",
        ],
        &["run", "--file", FILE, "--replace", "a", "--", "true"],
        // `--file` with a flag where its value should be. The hand parser took the next token
        // whatever it was, so this reported a missing value for the argument it had swallowed.
        &[
            "run",
            "--file",
            "--replace",
            "X",
            "--with",
            "Y",
            "--",
            "true",
        ],
    ];
    for args in invalid {
        let (ok, out, code) = s.run_coded(args);
        assert!(!ok, "{args:?} must be refused: {out}");
        assert_eq!(
            code, 2,
            "{args:?} is a could-not-run, not a tree report: {out}"
        );
    }
    // And the command after `--` is required, so a run cannot start with nothing to measure.
    let (_, _, code) = s.run_coded(&["run", "--file", FILE, "--replace", "a", "--with", "b"]);
    assert_eq!(code, 2);
}

// ---- the child process, and the claims of the spec this crate's slice landed ----

/// The variable that tells this test executable it was started as a mode's child, and carries the
/// arguments, separated by the unit separator.
const CHILD_ARGS: &str = "KNOWLEDGE_ARCHITECT_MUTATE_TEST_CHILD";

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: knowledge_architect_mutate::MutateCommand,
}

/// A command that runs one mode as a child process, in `root`, with `args` as a maintenance
/// binary's `mutate` command would receive them.
fn child(root: &Path, args: &[&str]) -> Command {
    let mut c = Command::new(std::env::current_exe().expect("the test executable"));
    c.args(["--exact", "child_entry", "--nocapture", "--test-threads=1"])
        .env(CHILD_ARGS, args.join("\u{1f}"))
        .current_dir(root);
    c
}

/// Not a test of its own: does nothing unless started by [`child`]. Then it parses the arguments
/// as a maintenance binary does, clap exiting 2 on a refusal, runs the mode over the working
/// directory and exits with the mode's code.
#[test]
fn child_entry() {
    let Ok(joined) = std::env::var(CHILD_ARGS) else {
        return;
    };
    use clap::Parser;
    let args = std::iter::once("mutate").chain(joined.split('\u{1f}').filter(|a| !a.is_empty()));
    let cli = Cli::try_parse_from(args).unwrap_or_else(|e| e.exit());
    let root = std::env::current_dir().expect("a working directory");
    let code = knowledge_architect_mutate::run(&root, &cli.command);
    let code = [0u8, 1, 2]
        .into_iter()
        .find(|n| code == std::process::ExitCode::from(*n))
        .expect("the modes exit 0, 1 or 2");
    std::process::exit(i32::from(code));
}

// The claim: a run killed while its mutation is applied is restored by `restore`, byte for byte,
// uncommitted content included, and no content is lost. The sandbox commits nothing, so every byte
// of the file is uncommitted work.
#[test]
fn a_run_killed_mid_mutation_is_restored_byte_for_byte() {
    let s = Sandbox::new("killed");
    let body = "uncommitted edit\nline2\nSITE\nline4\nno newline at the end";
    s.write(FILE, body);
    let mut run = child(
        s.root(),
        &[
            "run",
            "--file",
            FILE,
            "--replace",
            "SITE",
            "--with",
            "MUT",
            "--no-baseline",
            "--",
            "sh",
            "-c",
            // It outlives the run, so it stops on the target too: the test can end, and its
            // directory go, before the loop next looks for `stop`.
            "while [ -e src/f.rs ] && [ ! -e stop ]; do sleep 0.05; done",
        ],
    )
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null())
    .spawn()
    .expect("starting the run");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !s.read(FILE).contains("MUT") {
        assert!(
            std::time::Instant::now() < deadline,
            "the run never applied its mutation"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    run.kill().expect("killing the run");
    run.wait().expect("reaping the run");
    // The command the run started outlives it, per `design@mutate@hard-kill-leaves-the-command`;
    // stop it.
    s.write("stop", "");
    assert!(
        s.read(FILE).contains("MUT"),
        "the kill left the mutant on disk"
    );

    let (ok, out, code) = s.run_coded(&["restore"]);
    assert!(ok, "{out}");
    assert_eq!(code, 0, "{out}");
    assert_eq!(s.read(FILE), body, "the file must come back byte for byte");
    assert!(!s.record().exists(), "{out}");
}

// The claim: a refused pattern exits 2, leaves the target untouched and removes the record
// directory the run claimed, for zero and for two occurrences.
#[test]
fn a_pattern_that_does_not_occur_once_exits_two_and_claims_nothing() {
    let s = Sandbox::new("refused");
    s.write(FILE, "x\nx\n");
    for (pattern, says) in [("x", "occurs 2 times"), ("absent", "does not occur")] {
        let (_, out, code) = s.run_coded(&[
            "run",
            "--file",
            FILE,
            "--replace",
            pattern,
            "--with",
            "y",
            "--",
            "true",
        ]);
        assert_eq!(code, 2, "{out}");
        assert!(out.contains(says), "{out}");
        assert_eq!(s.read(FILE), "x\nx\n");
        assert!(!s.record().exists(), "{out}");
    }
}

// The claim: a red baseline exits 2, and no mutation is applied.
#[test]
fn a_red_baseline_exits_two_before_the_target_is_written() {
    let s = Sandbox::new("red");
    s.write(FILE, BODY);
    // The command records what it saw, so a mutation applied before it ran would show.
    let (_, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--",
        "sh",
        "-c",
        "cat src/f.rs >> seen; false",
    ]);
    assert_eq!(code, 2, "{out}");
    assert_eq!(
        s.read("seen"),
        BODY,
        "the baseline ran once, over the unmutated file"
    );
    assert_eq!(s.read(FILE), BODY);
}

// The claim: `restore` refuses a file changed since the mutation, writes nothing, exits 1, and
// prints the `git apply -R` commands of the record's patch.
#[test]
fn restore_refuses_a_changed_file_and_prints_the_reverse_patch() {
    let s = Sandbox::new("changed");
    s.write(FILE, BODY);
    // A run whose command edits the file leaves its record, as a killed run would.
    let (_, _, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "printf 'edited\\n' >> src/f.rs",
    ]);
    assert_eq!(code, 1);
    let changed = s.read(FILE);

    let (_, out, code) = s.run_coded(&["restore"]);
    assert_eq!(code, 1, "{out}");
    assert_eq!(s.read(FILE), changed, "restore wrote nothing");
    assert!(out.contains("git apply -R"), "{out}");
    assert!(out.contains("git apply -R -C1"), "{out}");
    assert!(s.record().join("pre-image").is_file());
}

// The claim: the read-back line names the file and the first changed line, and is printed before
// the command runs.
#[test]
fn the_mutation_is_shown_present_before_the_command_runs() {
    let s = Sandbox::new("shown");
    s.write(FILE, BODY);
    let (_, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "echo COMMAND-RAN",
    ]);
    assert_eq!(code, 0, "{out}");
    let shown = out
        .find("mutation applied to src/f.rs, first changed line 3")
        .expect("the read-back line");
    let ran = out.find("COMMAND-RAN").expect("the command's output");
    assert!(shown < ran, "{out}");
}

// The claim: outside a git repository the run never starts, and exits 2.
#[test]
fn outside_a_repository_the_run_exits_two() {
    let s = Sandbox::new("nogit");
    fs::remove_dir_all(s.root().join(".git")).expect("removing the repository");
    s.write(FILE, BODY);
    // GIT_CEILING_DIRECTORIES keeps git from finding a repository above the temporary directory.
    let out = child(
        s.root(),
        &[
            "run",
            "--file",
            FILE,
            "--replace",
            "SITE",
            "--with",
            "MUT",
            "--",
            "true",
        ],
    )
    .env(
        "GIT_CEILING_DIRECTORIES",
        s.root().parent().expect("a parent"),
    )
    .output()
    .expect("running the child");
    assert_eq!(
        out.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(s.read(FILE), BODY);
}

// The claim: the `--patch` form applies a mutation that is not one substitution, runs the command
// over it and restores the file.
#[test]
fn a_patch_mutation_runs_and_restores() {
    let s = Sandbox::new("patch");
    s.write(FILE, BODY);
    s.write(
        "m.diff",
        "--- a/src/f.rs\n+++ b/src/f.rs\n@@ -1,5 +1,4 @@\n line1\n line2\n-SITE\n line4\n line5\n",
    );
    let (_, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--patch",
        "m.diff",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "! grep -q SITE src/f.rs",
    ]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("mutation applied to src/f.rs"), "{out}");
    assert!(
        out.contains("SURVIVED"),
        "the command saw the line deleted: {out}"
    );
    assert_eq!(s.read(FILE), BODY);
}

/// A patch that deletes the line `SITE` of [`BODY`].
const DELETE_SITE: &str =
    "--- a/src/f.rs\n+++ b/src/f.rs\n@@ -1,5 +1,4 @@\n line1\n line2\n-SITE\n line4\n line5\n";

/// The arguments of a `--patch` run over a command that passes.
const RUN_PATCH: [&str; 7] = ["run", "--file", FILE, "--patch", "m.diff", "--", "true"];

// The claim: a `--patch` run makes nothing in the temporary directory, so one that names a
// directory that does not exist neither stops the run nor is made by it.
#[test]
fn a_patch_run_makes_nothing_in_the_temporary_directory() {
    let s = Sandbox::new("patch-tmpdir");
    s.write(FILE, BODY);
    s.write("m.diff", DELETE_SITE);
    let absent = s.root().join("absent");
    let out = child(s.root(), &RUN_PATCH)
        // Windows reads the two others.
        .env("TMPDIR", &absent)
        .env("TMP", &absent)
        .env("TEMP", &absent)
        .output()
        .expect("running the child");
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(!absent.exists(), "the run made its temporary directory");
    assert!(!s.record().exists());
    assert_eq!(s.read(FILE), BODY);
}

/// A linked worktree of the sandbox, at `linked`, holding a commit of what the sandbox holds.
fn linked_worktree(s: &Sandbox) -> PathBuf {
    for args in [
        &["add", "-A"][..],
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-qm",
            "base",
        ],
        &["worktree", "add", "-q", "linked"],
    ] {
        let git = Command::new("git")
            // The worktree's file is compared byte for byte, and the checkout converts under the
            // machine's setting, per `design@mutate@git-converts-no-line-ending`.
            .args(["-c", "core.autocrlf=false"])
            .args(args)
            .current_dir(s.root())
            .status()
            .expect("running git");
        assert!(git.success(), "git {args:?}");
    }
    let linked = s.root().join("linked");
    assert!(linked.join(".git").is_file());
    linked
}

// The claim: in a linked worktree, whose `.git` is a file and whose git directory is elsewhere, a
// `--patch` run plans its mutant, restores the file and leaves no record.
#[test]
fn a_patch_run_works_in_a_linked_worktree() {
    let s = Sandbox::new("patch-worktree");
    s.write(FILE, BODY);
    let linked = linked_worktree(&s);
    s.write("linked/m.diff", DELETE_SITE);
    let out = child(&linked, &RUN_PATCH)
        .output()
        .expect("running the child");
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{text}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("SURVIVED"), "{text}");
    assert_eq!(s.read("linked/src/f.rs"), BODY);
    let record = knowledge_architect_mutate::record_dir(&linked).expect("a repository");
    // Git resolves the links and the short names of the path it answers. The record does not exist, so
    // its parent, which does, is what is compared.
    let canonical = |path: &Path| path.canonicalize().expect("the directory exists");
    assert!(canonical(record.parent().expect("a parent"))
        .starts_with(canonical(&s.root().join(".git/worktrees"))));
    assert!(!record.exists());
}

// The claim: the mutant of a `--patch` run is the one the patch names, whatever the machine's
// `core.autocrlf`, as Git for Windows sets it: the file keeps its LF, which the command reads, and a
// patch that changes nothing is refused. Mutation checked: `core.autocrlf=false` of `git_in` made
// `core.autocrlf=true`; caught.
#[test]
fn a_patch_run_converts_no_line_ending_whatever_the_machine_s_git_says() {
    let s = Sandbox::new("patch-autocrlf");
    s.write(FILE, BODY);
    s.write("m.diff", DELETE_SITE);
    s.write(
        "empty.diff",
        "diff --git a/src/f.rs b/src/f.rs\nold mode 100644\nnew mode 100755\n",
    );
    s.write("expected", &BODY.replace("SITE\n", ""));
    for (patch, code, says) in [
        ("m.diff", 0, "SURVIVED"),
        ("empty.diff", 2, "does not change"),
    ] {
        let out = child(
            s.root(),
            &[
                "run",
                "--file",
                FILE,
                "--patch",
                patch,
                "--no-baseline",
                "--",
                "git",
                "diff",
                "--no-index",
                "--exit-code",
                "expected",
                FILE,
            ],
        )
        .env("GIT_CONFIG_COUNT", "1")
        .env("GIT_CONFIG_KEY_0", "core.autocrlf")
        .env("GIT_CONFIG_VALUE_0", "true")
        .output()
        .expect("running the child");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(out.status.code(), Some(code), "{patch}: {text}");
        assert!(text.contains(says), "{patch}: {text}");
        assert_eq!(s.read(FILE), BODY);
    }
}

// The claim: `GIT_DIR` and an absolute `GIT_WORK_TREE` in the caller's environment, as git sets both
// for an alias run through `--git-dir` and `--work-tree`, do not stop a `--patch` run.
#[test]
fn a_patch_run_ignores_a_work_tree_the_environment_names() {
    let s = Sandbox::new("patch-work-tree");
    s.write(FILE, BODY);
    s.write("m.diff", DELETE_SITE);
    let out = child(s.root(), &RUN_PATCH)
        .env("GIT_DIR", s.root().join(".git"))
        .env("GIT_WORK_TREE", s.root())
        .output()
        .expect("running the child");
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{text}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("SURVIVED"), "{text}");
    assert_eq!(s.read(FILE), BODY);
}

// The claim: a `--patch` run that git started in a linked worktree, with the `GIT_DIR` git exports
// there, writes nothing to the repository's configuration, so the main repository is not made bare.
#[test]
fn a_patch_run_started_by_git_leaves_the_main_repository_as_it_was() {
    let s = Sandbox::new("patch-by-git");
    s.write(FILE, BODY);
    let linked = linked_worktree(&s);
    s.write("linked/m.diff", DELETE_SITE);
    let config = s.read(".git/config");
    let run = child(&linked, &RUN_PATCH);
    let mut git = Command::new("git");
    git.args([
        "-c",
        "alias.mm=!\"$RUN\" --exact child_entry --nocapture",
        "mm",
    ])
    .env("RUN", run.get_program())
    .current_dir(&linked);
    for (name, value) in run.get_envs() {
        git.env(name, value.expect("a value"));
    }
    let out = git.output().expect("running git");
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{text}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("SURVIVED"), "{text}");
    assert_eq!(s.read(".git/config"), config);
    assert_eq!(s.read("linked/src/f.rs"), BODY);
}

/// A patch that gives the line `SITE` of [`BODY`] two trailing spaces, which `apply.whitespace=fix`
/// takes away, and the arguments of a run of it whose command passes only over the two spaces.
const ADD_SPACES: &str = "--- a/src/f.rs\n+++ b/src/f.rs\n@@ -1,5 +1,5 @@\n line1\n line2\n-SITE\n+SITE  \n line4\n line5\n";
const RUN_ADD_SPACES: [&str; 11] = [
    "run",
    "--file",
    FILE,
    "--patch",
    "m.diff",
    "--no-baseline",
    "--",
    "grep",
    "-q",
    "SITE  $",
    FILE,
];

// The claim: a patch is planned in a repository of its own, so the project's own git configuration
// does not rewrite the mutant.
#[test]
fn the_project_s_git_configuration_does_not_rewrite_the_mutant() {
    let s = Sandbox::new("patch-config");
    s.write(FILE, BODY);
    s.write("m.diff", ADD_SPACES);
    let config = format!("{}[apply]\n\twhitespace = fix\n", s.read(".git/config"));
    s.write(".git/config", &config);
    let (_, out, code) = s.run_coded(&RUN_ADD_SPACES);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("SURVIVED"), "{out}");
    assert_eq!(s.read(FILE), BODY);
}

// The claim: `GIT_COMMON_DIR` and `GIT_OBJECT_DIRECTORY` in the caller's environment reach neither
// git call that plans the mutant: the configuration of the one does not rewrite the mutant, and
// nothing is written in the other.
#[test]
fn a_patch_run_ignores_the_common_and_object_directories_the_environment_names() {
    let s = Sandbox::new("patch-common-dir");
    s.write(FILE, BODY);
    s.write("m.diff", ADD_SPACES);
    s.write("other.git/config", "[apply]\n\twhitespace = fix\n");
    fs::create_dir_all(s.root().join("other.git/refs")).expect("made");
    let objects = s.root().join("objects");
    fs::create_dir(&objects).expect("made");

    let out = child(s.root(), &RUN_ADD_SPACES)
        .env("GIT_COMMON_DIR", s.root().join("other.git"))
        .env("GIT_OBJECT_DIRECTORY", &objects)
        .output()
        .expect("running the child");
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{text}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("SURVIVED"), "{text}");
    assert_eq!(fs::read_dir(&objects).expect("readable").count(), 0);
    assert_eq!(s.read(FILE), BODY);
}

// The claim: a run killed while it plans its patch leaves its scratch in the record's claim, which
// `restore` refuses as incomplete and `restore --discard` removes, and nothing in the temporary
// directory. The target is named as a file of the record, so the claim is shown to hold it in a
// directory of its own, and `git apply` is a program that kills the run that started it. That program is
// a shell script, which Windows does not find as `git`, per
// `issue@mutate@the-hand-mutation-fails-in-several-ways-on-windows`.
#[test]
#[cfg_attr(
    windows,
    ignore = "a shell script stands for git, which Windows does not run, and the hand mutation is development workflow"
)]
fn a_run_killed_while_planning_its_patch_leaves_its_scratch_in_the_claim_alone() {
    let s = Sandbox::new("patch-killed");
    s.write("manifest", BODY);
    s.write("m.diff", &DELETE_SITE.replace("src/f.rs", "manifest"));
    s.write("tmp/kept", "");
    // Written by a process of its own: a program this process holds open for writing while another
    // test starts a child cannot be run.
    let stub = "mkdir bin && cat > bin/git && chmod +x bin/git";
    let mut write = Command::new("sh")
        .args(["-c", stub])
        .current_dir(s.root())
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("running sh");
    use std::io::Write;
    write
        .stdin
        .take()
        .expect("its stdin")
        .write_all(
            b"#!/bin/sh\nif [ \"$3\" = apply ]; then kill -9 $PPID; exit 1; fi\n\
              PATH=${PATH#*:} exec git \"$@\"\n",
        )
        .expect("the program is written");
    assert!(write.wait().expect("sh").success());
    let path = std::env::var("PATH").expect("PATH is set");
    let out = child(
        s.root(),
        &[
            "run", "--file", "manifest", "--patch", "m.diff", "--", "true",
        ],
    )
    .env("PATH", format!("{}:{path}", s.root().join("bin").display()))
    .env("TMPDIR", s.root().join("tmp"))
    .output()
    .expect("running the child");
    assert_eq!(out.status.code(), None, "the run is killed: {out:?}");
    assert_eq!(
        fs::read_dir(s.root().join("tmp"))
            .expect("readable")
            .count(),
        1,
        "only the file the test wrote"
    );

    let scratch = fs::read_dir(s.record())
        .expect("the claim stands")
        .filter(|e| {
            e.as_ref()
                .expect("an entry")
                .path()
                .join("manifest")
                .is_file()
        })
        .count();
    assert_eq!(scratch, 1, "the scratch is in the claim");
    let (_, out, code) = s.run_coded(&["restore"]);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("incomplete"), "{out}");
    let (_, out, code) = s.run_coded(&["restore", "--discard"]);
    assert_eq!(code, 0, "{out}");
    assert!(!s.record().exists(), "{out}");
    assert_eq!(s.read("manifest"), BODY);
}

// The claim: a build probe that cannot be spawned reaches no verdict, exits 1 and restores the
// file; an empty probe is a bad argument, refused with 2 before anything is claimed.
#[test]
fn a_probe_that_cannot_run_gives_no_verdict_and_an_empty_one_never_starts() {
    let s = Sandbox::new("probe-odd");
    s.write(FILE, BODY);
    let (_, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--no-baseline",
        "--build",
        "no-such-program-for-a-probe",
        "--",
        "true",
    ]);
    assert_eq!(code, 1, "{out}");
    assert!(!out.contains("unmutatable"), "{out}");
    assert!(!out.contains("SURVIVED"), "{out}");
    assert_eq!(s.read(FILE), BODY);

    let (_, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--replace",
        "SITE",
        "--with",
        "MUT",
        "--build",
        " ",
        "--",
        "true",
    ]);
    assert_eq!(code, 2, "{out}");
    assert!(!out.contains("mutation applied"), "{out}");
    assert!(!s.record().exists(), "{out}");
}

// ---- the claims of the review's repairs ----

const RUN_MUT: [&str; 7] = ["run", "--file", FILE, "--replace", "SITE", "--with", "MUT"];

fn args<'a>(rest: &[&'a str]) -> Vec<&'a str> {
    RUN_MUT
        .iter()
        .copied()
        .chain(rest.iter().copied())
        .collect()
}

// The claim: an edit saved while the baseline runs is kept: the run refuses with 2 and writes
// nothing.
#[test]
fn an_edit_saved_during_the_baseline_is_kept() {
    let s = Sandbox::new("baseline-edit");
    s.write(FILE, BODY);
    let script = "if [ ! -e done ]; then printf 'saved\\n' >> src/f.rs; touch done; fi";
    let (_, out, code) = s.run_coded(&args(&["--", "sh", "-c", script]));
    assert_eq!(code, 2, "{out}");
    assert_eq!(s.read(FILE), format!("{BODY}saved\n"), "{out}");
    assert!(!s.record().exists(), "{out}");
}

// The claim: a patch that names another file than `--file`, that does not apply, or that changes
// nothing, is refused with 2 and its own reason before the baseline runs, and leaves no record.
#[test]
fn a_patch_naming_another_file_not_applying_or_changing_nothing_is_refused() {
    let s = Sandbox::new("patch-odd");
    s.write(FILE, BODY);
    s.write(
        "other.diff",
        "--- a/src/f.rs\n+++ b/src/f.rs\n@@ -1,5 +1,4 @@\n line1\n line2\n-SITE\n line4\n line5\n\
         --- /dev/null\n+++ b/src/g.rs\n@@ -0,0 +1 @@\n+new\n",
    );
    s.write(
        "empty.diff",
        "diff --git a/src/f.rs b/src/f.rs\nold mode 100644\nnew mode 100755\n",
    );
    s.write("moved.diff", &DELETE_SITE.replace("line2", "line9"));
    for (patch, says) in [
        ("other.diff", "the patch names src/g.rs"),
        ("moved.diff", "the patch does not apply to src/f.rs"),
        ("empty.diff", "the patch does not change src/f.rs"),
    ] {
        let (_, out, code) = s.run_coded(&[
            "run",
            "--file",
            FILE,
            "--patch",
            patch,
            "--",
            "sh",
            "-c",
            "touch ran",
        ]);
        assert_eq!(code, 2, "{patch}: {out}");
        assert!(out.contains(says), "{patch}: {out}");
        assert!(!s.record().exists(), "{patch}: {out}");
        assert!(
            !s.root().join("ran").exists(),
            "{patch}: the baseline ran: {out}"
        );
        assert!(!s.root().join("src/g.rs").exists(), "{patch}: {out}");
        assert_eq!(s.read(FILE), BODY);
    }
}

// The claim: a target reached through a symlinked directory, inside the git directory, or that
// is no regular file, is refused with 2.
#[test]
fn a_target_through_a_symlink_in_git_or_not_a_file_is_refused() {
    let s = Sandbox::new("odd-targets");
    s.write("real/f.rs", BODY);
    link_dir(s.root().join("real"), s.root().join("link"));
    fs::create_dir(s.root().join("dir")).expect("a directory");
    let mut cases = vec![
        ("link/f.rs", "symlink"),
        (".git/HEAD", "git directory"),
        ("dir", "not a regular file"),
    ];
    // Windows has no named pipe in a directory.
    if cfg!(unix) {
        let fifo = Command::new("mkfifo")
            .arg(s.root().join("pipe"))
            .status()
            .expect("mkfifo runs");
        assert!(fifo.success());
        cases.push(("pipe", "not a regular file"));
    }
    for (file, says) in cases {
        let (_, out, code) = s.run_coded(&[
            "run",
            "--file",
            file,
            "--replace",
            "SITE",
            "--with",
            "MUT",
            "--",
            "true",
        ]);
        assert_eq!(code, 2, "{file}: {out}");
        assert!(out.contains(says), "{file}: {out}");
    }
    assert_eq!(s.read("real/f.rs"), BODY);
}

// The claim: the restore writes through no symlink the command left at the target, dangling or
// not, and exits 1 with the record standing. The `ln -s` of Git for Windows copies the file instead, per
// `issue@mutate@the-hand-mutation-fails-in-several-ways-on-windows`.
#[test]
#[cfg_attr(
    windows,
    ignore = "`ln -s` makes a copy under Git for Windows, and the hand mutation is development workflow"
)]
fn the_restore_writes_through_no_symlink_the_command_left() {
    let s = Sandbox::new("symlink-left");
    s.write(FILE, BODY);
    // The link points at a copy of the mutant, so the comparison alone would read it as safe.
    let to_copy = "cp src/f.rs copy.rs && rm src/f.rs && ln -s ../copy.rs src/f.rs";
    let (_, out, code) = s.run_coded(&args(&["--no-baseline", "--", "sh", "-c", to_copy]));
    assert_eq!(code, 1, "{out}");
    assert!(
        s.read("copy.rs").contains("MUT"),
        "nothing was written through the link"
    );
    assert!(s.record().join("pre-image").is_file());

    let t = Sandbox::new("dangling-left");
    t.write(FILE, BODY);
    let dangling = "rm src/f.rs && ln -s ../far.rs src/f.rs";
    let (_, out, code) = t.run_coded(&args(&["--no-baseline", "--", "sh", "-c", dangling]));
    assert_eq!(code, 1, "{out}");
    assert!(
        !t.root().join("far.rs").exists(),
        "nothing was written through the link"
    );
    assert!(t.record().join("pre-image").is_file());
}

// The claim: a failing command under a passing probe reads `caught`, with no caveat; a command
// that cannot be spawned reaches no verdict and exits 1 over a restored file.
#[test]
fn a_probe_makes_a_failure_a_catch_and_an_unspawnable_command_no_verdict() {
    let s = Sandbox::new("verdicts");
    s.write(FILE, BODY);
    let (_, out, code) = s.run_coded(&args(&["--no-baseline", "--build", "true", "--", "false"]));
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("src/f.rs: caught\n") || out.ends_with("src/f.rs: caught"),
        "{out}"
    );
    assert!(!out.contains("no --build probe"), "{out}");

    let (_, out, code) = s.run_coded(&args(&["--no-baseline", "--", "no-such-program-to-run"]));
    assert_eq!(code, 1, "{out}");
    assert!(
        !out.contains("caught") && !out.contains("SURVIVED"),
        "{out}"
    );
    assert_eq!(s.read(FILE), BODY);
}

// The claim: every refusal before the target is written leaves no record behind: an unspawnable
// baseline, an unreadable target, a read-only target.
#[test]
fn a_refusal_before_the_write_leaves_no_record() {
    let s = Sandbox::new("refusals");
    s.write(FILE, BODY);
    let (_, out, code) = s.run_coded(&args(&["--", "no-such-program-to-run"]));
    assert_eq!(code, 2, "{out}");
    assert!(!s.record().exists(), "unspawnable baseline: {out}");

    // Windows has no Unix mode bits, so the two modes are for the systems that have them.
    #[cfg(unix)]
    for mode in [0o000, 0o444] {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(s.root().join(FILE), fs::Permissions::from_mode(mode)).expect("chmod");
        let (_, out, code) = s.run_coded(&args(&["--no-baseline", "--", "true"]));
        fs::set_permissions(s.root().join(FILE), fs::Permissions::from_mode(0o644)).expect("chmod");
        assert_eq!(code, 2, "mode {mode:o}: {out}");
        assert!(!s.record().exists(), "mode {mode:o}: {out}");
        assert_eq!(s.read(FILE), BODY);
    }
}

// The claim: a record whose target already holds the pre-image, as a kill between the record and
// the write leaves, is cleared by `restore`, which exits 0.
#[test]
fn a_record_over_an_unmutated_file_is_cleared_by_restore() {
    let s = Sandbox::new("already");
    s.write(FILE, BODY);
    knowledge_architect_mutate::Record::at(s.record())
        .claim()
        .expect("claiming");
    knowledge_architect_mutate::Record::at(s.record())
        .write(FILE, BODY, "mutant\n", None)
        .expect("writing the record");
    // A process of the test's own that has ended, whose pid names no live run.
    let mut ended = Command::new("git")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("running git");
    assert!(ended.wait().expect("git").success());
    fs::write(s.record().join("owner-pid"), ended.id().to_string()).expect("no live owner");
    let (_, out, code) = s.run_coded(&["restore"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("already held its pre-image"), "{out}");
    assert!(!s.record().exists(), "{out}");
}

// The claims: the patch form records its patch, so a restore that refuses a changed file prints the
// reverse of the caller's own patch; and the record that stands holds its own files alone, nothing
// of the planning of its mutant.
#[test]
fn a_patch_run_records_its_patch_for_the_reverse() {
    let s = Sandbox::new("patch-record");
    s.write(FILE, BODY);
    s.write(
        "m.diff",
        "--- a/src/f.rs\n+++ b/src/f.rs\n@@ -1,5 +1,4 @@\n line1\n line2\n-SITE\n line4\n line5\n",
    );
    let (_, out, code) = s.run_coded(&[
        "run",
        "--file",
        FILE,
        "--patch",
        "m.diff",
        "--no-baseline",
        "--",
        "sh",
        "-c",
        "printf 'edited\\n' >> src/f.rs",
    ]);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("git apply -R"), "{out}");
    let mut held: Vec<String> = fs::read_dir(s.record())
        .expect("the record stands")
        .map(|e| {
            let name = e.expect("an entry").file_name();
            name.to_string_lossy().into_owned()
        })
        .collect();
    held.sort();
    assert_eq!(
        held,
        [
            "manifest",
            "mutant",
            "mutation.patch",
            "owner-pid",
            "pre-image"
        ],
        "{out}"
    );
}
