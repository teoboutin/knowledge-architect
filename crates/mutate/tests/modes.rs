//! The `run` and `restore` modes end to end, each in a process of its own.
//!
//! `harness.rs` covers the harness; claiming the record, the write path, the read-back
//! assertion, the unwind and `restore` are reachable only by running a mode, and a kill is
//! observable only on a process. The crate holds a library alone, so each run re-enters this test
//! executable as its child: `child_entry` parses the arguments a maintenance binary would, runs
//! the mode over its working directory, and exits with the mode's code.
//!
//! Every test builds its own sandbox, a git repository in the temporary directory, so a run's
//! record and target stay inside it and never touch this repository. **The command under `--` is
//! always a shell builtin or `sh -c`, never `cargo`**: a nested cargo would rebuild the workspace
//! inside a test and contend for the same target directory.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(tag: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .subsec_nanos();
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut root = std::env::temp_dir();
        root.push(format!(
            "mutate-bin-{tag}-{}-{nanos}-{n}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("creating the sandbox");
        // The record lives in the git directory, so the sandbox is a repository of its own.
        let init = Command::new("git")
            .args(["init", "-q", "."])
            .current_dir(&root)
            .status()
            .expect("running git init");
        assert!(init.success(), "git init");
        Sandbox { root }
    }

    fn write(&self, rel: &str, content: &str) -> PathBuf {
        let p = self.root.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("creating a parent directory");
        }
        fs::write(&p, content).expect("writing a sandbox file");
        p
    }

    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root.join(rel)).expect("reading a sandbox file")
    }

    /// Runs the binary in the sandbox. Returns (success, stdout and stderr together).
    fn run(&self, args: &[&str]) -> (bool, String) {
        let (ok, text, _) = self.run_coded(args);
        (ok, text)
    }

    /// The same, keeping the exit code. Success alone cannot tell 1 from 2, and the whole
    /// point of the ladder is that those two say different things about the tree.
    fn run_coded(&self, args: &[&str]) -> (bool, String, i32) {
        let out = child(&self.root, args).output().expect("running the child");
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));
        (out.status.success(), text, out.status.code().unwrap_or(-1))
    }

    fn record(&self) -> PathBuf {
        self.root.join(".git/knowledge-architect-mutate")
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).ok();
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
    std::os::unix::fs::symlink(s.root.join("real.txt"), s.root.join("link.txt")).expect("linking");
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

#[test]
fn restore_refuses_while_the_owning_run_is_alive() {
    // Built by hand rather than by racing two runs: the guard is what is under test, and pid 1 is
    // always running. A restore here would put the pre-image back underneath a live run, whose
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
    fs::write(s.record().join("owner-pid"), "1").expect("writing a live owner");

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
        &s.root,
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
            "while [ ! -e stop ]; do sleep 0.05; done",
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
    // The command the run started outlives it, per the crate's issue on a hard kill; stop it.
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
    fs::remove_dir_all(s.root.join(".git")).expect("removing the repository");
    s.write(FILE, BODY);
    // GIT_CEILING_DIRECTORIES keeps git from finding a repository above the temporary directory.
    let out = child(
        &s.root,
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
        s.root.parent().expect("a parent"),
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

// The claim: a patch that names another file than `--file`, or that changes nothing, is refused
// with 2 before the baseline runs.
#[test]
fn a_patch_naming_another_file_or_changing_nothing_is_refused() {
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
    for patch in ["other.diff", "empty.diff"] {
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
        assert!(
            !s.root.join("ran").exists(),
            "{patch}: the baseline ran: {out}"
        );
        assert!(!s.root.join("src/g.rs").exists(), "{patch}: {out}");
        assert_eq!(s.read(FILE), BODY);
    }
}

// The claim: a target reached through a symlinked directory, inside the git directory, or that
// is no regular file, is refused with 2.
#[test]
fn a_target_through_a_symlink_in_git_or_not_a_file_is_refused() {
    let s = Sandbox::new("odd-targets");
    s.write("real/f.rs", BODY);
    std::os::unix::fs::symlink(s.root.join("real"), s.root.join("link")).expect("linking");
    let fifo = Command::new("mkfifo")
        .arg(s.root.join("pipe"))
        .status()
        .expect("mkfifo runs");
    assert!(fifo.success());
    for (file, says) in [
        ("link/f.rs", "symlink"),
        (".git/HEAD", "git directory"),
        ("pipe", "not a regular file"),
    ] {
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
// not, and exits 1 with the record standing.
#[test]
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
        !t.root.join("far.rs").exists(),
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
    use std::os::unix::fs::PermissionsExt;
    let s = Sandbox::new("refusals");
    s.write(FILE, BODY);
    let (_, out, code) = s.run_coded(&args(&["--", "no-such-program-to-run"]));
    assert_eq!(code, 2, "{out}");
    assert!(!s.record().exists(), "unspawnable baseline: {out}");

    for mode in [0o000, 0o444] {
        fs::set_permissions(s.root.join(FILE), fs::Permissions::from_mode(mode)).expect("chmod");
        let (_, out, code) = s.run_coded(&args(&["--no-baseline", "--", "true"]));
        fs::set_permissions(s.root.join(FILE), fs::Permissions::from_mode(0o644)).expect("chmod");
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
    fs::write(s.record().join("owner-pid"), "0").expect("no live owner");
    let (_, out, code) = s.run_coded(&["restore"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("already held its pre-image"), "{out}");
    assert!(!s.record().exists(), "{out}");
}
