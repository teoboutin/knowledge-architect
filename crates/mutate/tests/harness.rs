//! The hand-mutation harness. The claims are the two properties a hand mutation cannot establish
//! for itself — that the mutation reached the code that ran, and that removing it removes nothing
//! else — plus the recovery record that carries them across a kill.
//!
//! The patch tests run real `git apply`, because a generated patch is only correct if the tool
//! that consumes it accepts it: hunk arithmetic that is wrong by one produces a patch that reads
//! plausibly and applies to nothing.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use knowledge_architect_mutate::{
    plan_replace, record_dir, restore_verdict, single_hunk_diff, target_is_mutable, PlanError,
    Record, RestoreVerdict,
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_dir(tag: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .subsec_nanos();
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut p = std::env::temp_dir();
    p.push(format!(
        "mutate-test-{tag}-{}-{nanos}-{n}",
        std::process::id()
    ));
    fs::create_dir_all(&p).expect("creating the temp dir");
    p
}

// ---- plan_replace: the mutation names exactly one site ----

#[test]
fn plan_replace_substitutes_exactly_one_site() {
    let src = "alpha\nbeta\ngamma\n";
    assert_eq!(
        plan_replace(src, "beta", "BETA").expect("unique pattern"),
        "alpha\nBETA\ngamma\n"
    );
}

#[test]
fn plan_replace_refuses_an_absent_pattern() {
    // The recorded failure: a scripted edit matched no text because the target line had been
    // rewrapped, and the run that followed reported a surviving mutation over unmutated code.
    assert_eq!(
        plan_replace("alpha\nbeta\n", "delta", "DELTA"),
        Err(PlanError::NotFound)
    );
}

#[test]
fn plan_replace_refuses_a_repeated_pattern_and_names_the_count() {
    // `str::replace` would edit all three; a presence check cannot see the difference.
    assert_eq!(
        plan_replace("x\nx\nx\n", "x", "y"),
        Err(PlanError::NotUnique(3))
    );
}

#[test]
fn plan_replace_counts_overlapping_placements() {
    // `AAAA` offers two positions for `AAA`, so it names no single site. `str::matches` counts
    // non-overlapping matches and reports one, which would let the mutation through with a
    // verdict about whichever placement `replacen` happened to take.
    assert_eq!(
        plan_replace("AAAA", "AAA", "B"),
        Err(PlanError::NotUnique(2))
    );
    assert_eq!(plan_replace("AAA", "AAA", "B"), Ok("B".to_string()));
}

#[test]
fn plan_replace_counts_placements_across_multibyte_text() {
    // The scan steps one byte at a time to catch overlaps, so it must skip to the next character
    // boundary or `find` panics on a slice starting inside a character.
    assert_eq!(
        plan_replace("héllo héllo", "héllo", "x"),
        Err(PlanError::NotUnique(2))
    );
    assert_eq!(
        plan_replace("héllo wörld", "héllo", "x"),
        Ok("x wörld".to_string())
    );
}

#[test]
fn plan_replace_refuses_an_identity_replacement() {
    assert_eq!(
        plan_replace("alpha\n", "alpha", "alpha"),
        Err(PlanError::NoChange)
    );
}

#[test]
fn plan_replace_refuses_an_empty_pattern() {
    assert_eq!(
        plan_replace("alpha\n", "", "x"),
        Err(PlanError::EmptyPattern)
    );
}

// ---- single_hunk_diff: the patch a killed run leaves behind actually works ----

/// Writes `pre` into a fresh git repository, applies the generated patch, and reverses it.
/// Returns (text after apply, text after reverse).
fn apply_and_reverse(pre: &str, post: &str) -> (String, String) {
    let dir = temp_dir("patch");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(&dir)
            .output()
            .expect("running git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q", "."]);

    let target = dir.join("f.rs");
    fs::write(&target, pre).expect("writing the pre-image");
    let patch = single_hunk_diff(pre, post, "f.rs").expect("the texts differ");
    let patch_path = dir.join("m.patch");
    fs::write(&patch_path, &patch).expect("writing the patch");

    git(&["apply", "m.patch"]);
    let after_apply = fs::read_to_string(&target).expect("reading after apply");
    git(&["apply", "-R", "m.patch"]);
    let after_reverse = fs::read_to_string(&target).expect("reading after reverse");

    fs::remove_dir_all(&dir).ok();
    (after_apply, after_reverse)
}

#[test]
fn patch_round_trips_a_change_in_the_middle() {
    let pre = "a\nb\nc\nd\ne\nf\ng\nh\n";
    let post = "a\nb\nc\nD\ne\nf\ng\nh\n";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_a_change_on_the_first_line() {
    let pre = "a\nb\nc\nd\n";
    let post = "A\nb\nc\nd\n";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_a_change_on_the_last_line() {
    let pre = "a\nb\nc\nd\n";
    let post = "a\nb\nc\nD\n";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_a_file_with_no_trailing_newline() {
    // The `\ No newline at end of file` marker. Without it `git apply` refuses the patch.
    let pre = "a\nb\nc";
    let post = "a\nB\nc";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_a_mutation_that_removes_the_trailing_newline() {
    let pre = "a\nb\nc\n";
    let post = "a\nb\nc";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_an_insertion() {
    let pre = "a\nb\nc\n";
    let post = "a\nb\nb2\nb3\nc\n";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_a_deletion() {
    let pre = "a\nb\nb2\nc\n";
    let post = "a\nc\n";
    assert_eq!(
        apply_and_reverse(pre, post),
        (post.to_string(), pre.to_string())
    );
}

#[test]
fn patch_round_trips_a_multi_line_replacement_in_a_long_file() {
    let pre: String = (0..40).map(|n| format!("line {n}\n")).collect();
    let post = pre.replace("line 20\nline 21\n", "MUTANT\n");
    assert_eq!(apply_and_reverse(&pre, &post), (post.clone(), pre.clone()));
}

#[test]
fn identical_texts_yield_no_patch() {
    assert_eq!(single_hunk_diff("a\nb\n", "a\nb\n", "f.rs"), None);
}

#[test]
fn the_hunk_carries_three_context_lines_either_side() {
    // Context is what `git apply -R` matches on, so its width is the measured bound on which
    // concurrent edits a reverse survives. A patch with less context reverses less often.
    let pre: String = (0..20).map(|n| format!("line {n}\n")).collect();
    let post = pre.replace("line 10", "MUTANT");
    let patch = single_hunk_diff(&pre, &post, "f.rs").expect("the texts differ");
    assert!(
        patch.contains("@@ -8,7 +8,7 @@"),
        "unexpected hunk header in:\n{patch}"
    );
    assert!(
        patch.contains(" line 7\n"),
        "leading context missing:\n{patch}"
    );
    assert!(
        patch.contains(" line 13\n"),
        "trailing context missing:\n{patch}"
    );
    assert!(
        !patch.contains(" line 6\n"),
        "too much leading context:\n{patch}"
    );
    assert!(
        !patch.contains(" line 14\n"),
        "too much trailing context:\n{patch}"
    );
}

// ---- restore_verdict: the only branch that writes is the one that cannot lose anything ----

#[test]
fn restore_is_safe_only_when_the_file_is_the_recorded_mutant() {
    assert_eq!(
        restore_verdict("MUTANT", "MUTANT", "PRE"),
        RestoreVerdict::Safe
    );
}

#[test]
fn restore_reports_already_restored_when_the_file_holds_the_pre_image() {
    // A run killed between writing the record and writing the mutant leaves exactly this.
    assert_eq!(
        restore_verdict("PRE", "MUTANT", "PRE"),
        RestoreVerdict::AlreadyRestored
    );
}

#[test]
fn restore_refuses_when_the_file_changed_under_the_harness() {
    // Writing the pre-image here would take the concurrent edit with it, which is the content
    // loss this harness exists to make impossible.
    assert_eq!(
        restore_verdict("MUTANT PLUS AN EDIT", "MUTANT", "PRE"),
        RestoreVerdict::FileChanged
    );
}

// ---- the recovery record ----

#[test]
fn the_record_round_trips() {
    let dir = temp_dir("record");
    let record = Record::at(dir.join("current"));
    assert!(!record.exists());

    record.claim().expect("claiming the record");
    record
        .write("crates/x/src/y.rs", "PRE", "MUTANT", Some("PATCH"))
        .expect("writing the record");
    assert!(record.exists());

    let read = record.read().expect("reading the record");
    assert_eq!(read.target, "crates/x/src/y.rs");
    assert_eq!(read.pre_image, "PRE");
    assert_eq!(read.mutant, "MUTANT");
    assert_eq!(read.patch.as_deref(), Some("PATCH"));

    record.clear().expect("clearing the record");
    assert!(!record.exists());
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_half_written_record_does_not_read_as_present() {
    // What `cargo mutate restore` rests on: a record reporting itself present has its pre-image.
    // Testing the write order directly would need to observe an interruption, so the guarantee
    // is carried by `exists()` requiring every file instead.
    let dir = temp_dir("partial");
    let record = Record::at(dir.join("current"));
    fs::create_dir_all(&record.dir).expect("creating the record dir");

    fs::write(record.dir.join("pre-image"), "PRE").expect("writing");
    assert!(
        !record.exists(),
        "pre-image alone must not read as a record"
    );

    fs::write(record.dir.join("mutant"), "MUTANT").expect("writing");
    assert!(
        !record.exists(),
        "a missing manifest must not read as a record"
    );

    fs::write(record.dir.join("manifest"), "f.rs").expect("writing");
    assert!(record.exists());

    fs::remove_file(record.dir.join("pre-image")).expect("removing");
    assert!(
        !record.exists(),
        "a record without its pre-image protects nothing"
    );

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_record_holds_the_pre_image_before_it_is_complete() {
    // Every file the record needs is on disk once `write` returns, which is what makes a record
    // found after a hard kill usable. `exists()` is what enforces completeness, per the test
    // above; this one checks that `write` produces all of it.
    let dir = temp_dir("order");
    let record = Record::at(dir.join("current"));
    record.claim().expect("claiming the record");
    record
        .write("f.rs", "PRE", "MUTANT", None)
        .expect("writing the record");

    assert!(record.dir.join("pre-image").is_file());
    assert!(record.dir.join("mutant").is_file());
    assert!(record.exists());
    // No patch was supplied, so none is recorded and `read` reports its absence rather than
    // inventing one.
    assert_eq!(record.read().expect("reading").patch, None);
    fs::remove_dir_all(&dir).ok();
}

// ---- what may be mutated ----

#[test]
fn an_absolute_path_is_refused() {
    // `Path::join` discards the root when its argument is absolute, so this would mutate a file
    // outside the repository and the patch's `a/`/`b/` headers would name nothing.
    assert!(target_is_mutable(Path::new("/repo"), "/etc/passwd").is_err());
}

#[test]
fn a_path_climbing_out_of_the_repository_is_refused() {
    assert!(target_is_mutable(Path::new("/repo"), "../outside.rs").is_err());
    assert!(target_is_mutable(Path::new("/repo"), "a/../../outside.rs").is_err());
}

#[test]
fn a_symlinked_target_is_refused() {
    // `fs::write` follows a symlink, so a restore would write the pre-image over whatever sits at
    // the far end rather than over the mutation.
    let dir = temp_dir("symlink");
    fs::write(dir.join("real.rs"), "content\n").expect("writing");
    std::os::unix::fs::symlink(dir.join("real.rs"), dir.join("link.rs")).expect("linking");

    assert!(target_is_mutable(&dir, "link.rs").is_err());
    assert!(target_is_mutable(&dir, "real.rs").is_ok());
    fs::remove_dir_all(&dir).ok();
}

// ---- claiming the record ----

#[test]
fn a_second_claim_on_one_record_fails() {
    // The guarantee is atomicity, not politeness: two runs that both got past a check-then-write
    // leave four files mixing two runs, and a restore then writes one run's pre-image into the
    // other run's file.
    let dir = temp_dir("claim");
    let record = Record::at(dir.join("current"));
    record.claim().expect("the first claim");
    assert!(record.claim().is_err(), "the second claim must fail");

    record.clear().expect("clearing");
    record.claim().expect("claiming again after a clear");
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_claim_is_visible_before_the_record_is_complete() {
    let dir = temp_dir("claimed");
    let record = Record::at(dir.join("current"));
    record.claim().expect("claiming");
    assert!(record.claimed(), "the directory is taken");
    assert!(!record.exists(), "but no record has been written into it");
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_record_this_process_claimed_does_not_read_as_owned_by_a_live_run() {
    // `owner_is_alive` gates `restore`. Reporting the calling process as a live owner would make
    // every restore refuse.
    let dir = temp_dir("owner");
    let record = Record::at(dir.join("current"));
    record.claim().expect("claiming");
    assert!(!record.owner_is_alive());

    fs::write(record.dir.join("owner-pid"), "1").expect("writing a live pid");
    assert!(record.owner_is_alive(), "pid 1 is always running");

    fs::write(record.dir.join("owner-pid"), "not a pid").expect("writing");
    assert!(
        !record.owner_is_alive(),
        "an unreadable owner permits the restore"
    );
    fs::remove_dir_all(&dir).ok();
}

// The claim: the record lives in the git directory of the repository that holds the root, where
// neither `cargo clean` nor `git clean` reaches and the checker's walk never looks.
#[test]
fn the_record_lives_in_the_git_directory() {
    let dir = temp_dir("gitdir");
    let init = Command::new("git")
        .args(["init", "-q", "."])
        .current_dir(&dir)
        .status()
        .expect("running git init");
    assert!(init.success());
    let record = record_dir(&dir).expect("git names the path");
    // The record does not exist yet, so its parent, which does, is what is compared.
    let parent = record.parent().expect("a parent");
    assert_eq!(
        parent.canonicalize().expect("the parent exists"),
        dir.join(".git").canonicalize().expect("the git directory"),
        "found {record:?}"
    );
    assert!(record.ends_with("knowledge-architect-mutate"));
    fs::remove_dir_all(&dir).ok();
}

// The claim: completeness is checked per file: a record with its manifest and pre-image but no
// mutant does not read as present.
#[test]
fn a_record_without_its_mutant_does_not_read_as_present() {
    let dir = temp_dir("nomutant");
    let record = Record::at(dir.join("current"));
    record.claim().expect("claiming");
    fs::write(record.dir.join("manifest"), "f.rs").expect("manifest");
    fs::write(record.dir.join("pre-image"), "pre").expect("pre-image");
    assert!(!record.exists());
    fs::remove_dir_all(&dir).ok();
}
