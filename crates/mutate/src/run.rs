//! The two modes: the hand mutation, `run`, and the recovery, `restore`.
//!
//! The exit codes are the table of the crate's documentation: 2 where the run never started, 1 where
//! it acted and something is owed on the tree, 0 for any completed run whatever its verdict.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crate::harness::{
    plan_replace, record_dir, restore_verdict, single_hunk_diff, target_is_mutable, Record,
    RestoreVerdict,
};
use crate::print::{complain, say};
use crate::RunArgs;

/// The recovery record of the repository that holds `root`, or `None`, said, when git cannot name
/// its directory: the run has not started, so the caller exits 2.
fn locate_record(root: &Path) -> Option<Record> {
    match record_dir(root) {
        Some(dir) => Some(Record::at(dir)),
        None => {
            complain!(
                "cannot locate the git directory of {}; the mutation record lives there, so the \
                 project must be inside a git working tree",
                root.display()
            );
            None
        }
    }
}

/// How the caller described the mutation. Both forms end at the same place — the mutant's bytes,
/// known before anything is written to the target.
enum Form {
    Replace { old: String, new: String },
    Patch { path: PathBuf },
}

pub(crate) fn run_hand_mutation(root: &Path, args: &RunArgs) -> ExitCode {
    let file = args.file.clone();
    let build = args.build.clone();
    let baseline = !args.no_baseline;
    let command = args.command.clone();
    // clap's `form` group makes exactly one of `--replace` and `--patch` present, and
    // `requires` binds `--with` to `--replace`, so no other combination reaches here from a
    // command line. `cli_declaration_is_consistent` is what checks that the declaration says so.
    let form = match (&args.replace, &args.with, &args.patch) {
        (Some(old), Some(new), None) => Form::Replace {
            old: old.clone(),
            new: new.clone(),
        },
        (None, None, Some(patch)) => Form::Patch {
            path: root.join(patch),
        },
        _ => unreachable!("the `form` argument group admits no other combination"),
    };

    // An empty probe names no command; refused before anything is claimed, as a bad argument.
    if build.as_deref().is_some_and(|b| build_parts(b).is_empty()) {
        complain!("--build was given an empty command");
        return ExitCode::from(2);
    }

    let target = match target_is_mutable(root, &file) {
        Ok(t) => t,
        Err(e) => {
            complain!("{e}");
            return ExitCode::from(2);
        }
    };

    let Some(record) = locate_record(root) else {
        return ExitCode::from(2);
    };
    // Claiming the directory is atomic, so of two runs racing here exactly one proceeds. Checking
    // for a record and then writing one is not the same thing: both runs pass the check, both
    // write, and the files left behind mix the two runs — a restore then compares against one
    // run's mutant and writes the other run's pre-image into it.
    if let Err(e) = record.claim() {
        complain!(
            "cannot claim the mutation record at {}: {e}\n\
             A run already holds it, or a previous one did not unwind. \
             `cargo x mutate restore` puts that run's file back; \
             `cargo x mutate restore --discard` drops the record without writing anything.",
            record.dir.display()
        );
        return ExitCode::from(2);
    }

    let pre_image = match fs::read_to_string(&target) {
        Ok(s) => s,
        Err(e) => {
            complain!("cannot read {file}: {e}");
            let _ = record.clear();
            return ExitCode::from(2);
        }
    };

    let (mutant, patch_text) = match &form {
        Form::Replace { old, new } => match plan_replace(&pre_image, old, new) {
            Ok(m) => {
                let p = single_hunk_diff(&pre_image, &m, &file);
                (m, p)
            }
            Err(e) => {
                complain!("{file}: {e}");
                let _ = record.clear();
                return ExitCode::from(2);
            }
        },
        Form::Patch { path } => {
            let text = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    complain!("cannot read the patch {}: {e}", path.display());
                    let _ = record.clear();
                    return ExitCode::from(2);
                }
            };
            // Learn the mutant by applying the patch to a scratch copy, so the record is
            // complete before the real file is touched. `git apply` refusing here is the
            // patch form's assertion that the mutation is the one described: a patch whose context moved
            // never reaches the tree.
            if let Some(other) = patch_paths(&text).into_iter().find(|p| p != &file) {
                complain!(
                    "the patch names {other}, which is not {file}; a mutation changes the one \
                     file --file names"
                );
                let _ = record.clear();
                return ExitCode::from(2);
            }
            match apply_to_scratch(&pre_image, &file, &text) {
                Ok(m) if m == pre_image => {
                    complain!("the patch does not change {file}; the run would measure the unmutated tree");
                    let _ = record.clear();
                    return ExitCode::from(2);
                }
                Ok(m) => (m, Some(text)),
                Err(e) => {
                    complain!("the patch does not apply to {file}:\n{e}");
                    let _ = record.clear();
                    return ExitCode::from(2);
                }
            }
        }
    };

    // Every verdict below is an exit code of the caller's command, so over a base that already
    // fails, each mutation reads as caught, whatever command the caller chose.
    if baseline {
        say!("baseline: running the command over the unmutated tree");
        match run_command(root, &command) {
            Some(true) => {}
            Some(false) => {
                complain!(
                    "the command already fails over the unmutated tree, so every mutation would \
                     read as caught. Fix that first, or pass --no-baseline when the failure is \
                     expected and understood."
                );
                let _ = record.clear();
                return ExitCode::from(2);
            }
            None => {
                let _ = record.clear();
                return ExitCode::from(2);
            }
        }
    }

    // The baseline can run for minutes, and an editor or a formatter may save the file meanwhile.
    // The mutant was planned from the content read before it, so writing it now would replace
    // that save; the run refuses instead, having written nothing.
    if baseline && fs::read_to_string(&target).ok().as_deref() != Some(pre_image.as_str()) {
        complain!(
            "{file} changed while the baseline ran; nothing was written. Run again over the file as \
             it is now."
        );
        let _ = record.clear();
        return ExitCode::from(2);
    }

    if let Err(e) = record.write(&file, &pre_image, &mutant, patch_text.as_deref()) {
        complain!("cannot write the recovery record: {e}");
        let _ = record.clear();
        return ExitCode::from(2);
    }
    if let Err(e) = fs::write(&target, &mutant) {
        complain!("cannot write the mutant: {e}");
        // A write that fails before opening the file, as on a read-only target, leaves it as it
        // was, and the record then protects nothing and would block every later run. A write that
        // fails after the open has truncated the file, and the record holds the only copy of its
        // content, so it stays.
        if fs::read_to_string(&target).ok().as_deref() == Some(pre_image.as_str()) {
            let _ = record.clear();
            return ExitCode::from(2);
        }
        complain!(
            "{file} no longer holds its content; its pre-image is at {}/pre-image, and \
             `cargo x mutate restore` writes it back",
            record.dir.display()
        );
        return ExitCode::FAILURE;
    }

    // The mutation is shown present by reading back what the run will actually compile, not by
    // trusting that the write did what it said. This is the assertion whose absence let three of
    // five checks in one unit report on unmutated code.
    match fs::read_to_string(&target) {
        Ok(on_disk) if on_disk == mutant && on_disk != pre_image => {
            let lines = pre_image
                .lines()
                .zip(mutant.lines())
                .position(|(a, b)| a != b)
                .map(|n| n + 1);
            match lines {
                Some(n) => say!("mutation applied to {file}, first changed line {n}"),
                None => say!("mutation applied to {file}"),
            }
        }
        Ok(_) => {
            complain!("{file} does not hold the mutant after writing it; refusing to run");
            unwind(root, &record, &target, &file);
            return ExitCode::FAILURE;
        }
        Err(e) => {
            complain!("cannot read {file} back: {e}");
            unwind(root, &record, &target, &file);
            return ExitCode::FAILURE;
        }
    }

    // A mutant that does not build makes the command fail for a reason that is not a test
    // noticing anything, and reported as `caught` it overstates coverage. A probe such as
    // `cargo test --no-run` separates the two; the caller names it, because the command is
    // theirs, and the tool cannot know which build the command implies.
    // A probe that could not be spawned reached no verdict on the build, so the run reaches
    // none either: it is no catch and no proof the mutant is unbuildable.
    let build_probe: Option<Option<bool>> = build.as_deref().map(|b| {
        say!("build probe: {b}");
        run_command(root, &build_parts(b))
    });

    let verdict: Option<String> = if build_probe == Some(None) {
        None
    } else if build_probe == Some(Some(false)) {
        Some("unmutatable here (the mutant does not build); owed a different mutation".to_string())
    } else {
        say!("running: {}", command.join(" "));
        match run_command(root, &command) {
            Some(true) => Some("SURVIVED — no test distinguished the mutation".to_string()),
            Some(false) if build_probe.is_some() => Some("caught".to_string()),
            // With no probe, a command that failed because the mutant did not compile is
            // indistinguishable from one that failed because a test noticed. The verdict says so
            // rather than claiming the stronger of the two.
            Some(false) => Some(
                "caught — no --build probe ran, so a mutant that failed to compile would read \
                 the same"
                    .to_string(),
            ),
            None => None,
        }
    };

    let unwound = unwind(root, &record, &target, &file);
    if let Some(v) = &verdict {
        say!("{file}: {v}");
    }
    match (verdict, unwound) {
        (Some(_), true) => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

/// The probe's words: `--build` runs with no shell, split on whitespace.
fn build_parts(build: &str) -> Vec<String> {
    build.split_whitespace().map(String::from).collect()
}

/// Runs one command from the project's root. `None` when it could not be spawned at all, which
/// is not a verdict about the mutation and must not be reported as one.
fn run_command(root: &Path, command: &[String]) -> Option<bool> {
    match Command::new(&command[0])
        .args(&command[1..])
        .current_dir(root)
        .status()
    {
        Ok(s) => Some(s.success()),
        Err(e) => {
            complain!("cannot run {:?}: {e}", command[0]);
            None
        }
    }
}

/// Puts the target back, or refuses and says what to run. Returns whether the tree is clean of
/// the mutation.
fn unwind(root: &Path, record: &Record, target: &Path, file: &str) -> bool {
    let recorded = match record.read() {
        Ok(r) => r,
        Err(e) => {
            complain!("cannot read the recovery record: {e}");
            return false;
        }
    };
    let current = match fs::read_to_string(target) {
        Ok(s) => s,
        // The command deleted the target. Writing the pre-image back cannot remove anything,
        // because there is nothing there to remove, and refusing would strand the record and
        // block every later run.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // A dangling symlink reads as not found, and `fs::write` would follow it out of the
            // project.
            if fs::symlink_metadata(target).is_ok_and(|m| m.file_type().is_symlink()) {
                complain!(
                    "{file} is now a dangling symlink; refusing to write through it. The pre-image \
                     is at {}/pre-image.",
                    record.dir.display()
                );
                return false;
            }
            match fs::write(target, &recorded.pre_image) {
                Ok(()) => {
                    let _ = record.clear();
                    say!("{file} was gone; its pre-image has been written back");
                    return true;
                }
                Err(e) => {
                    complain!(
                        "{file} is gone and its pre-image could not be written back: {e}\n\
                         The pre-image is at {}/pre-image.",
                        record.dir.display()
                    );
                    return false;
                }
            }
        }
        Err(e) => {
            complain!(
                "cannot read {file} to restore it: {e}\nThe pre-image is at {}/pre-image.",
                record.dir.display()
            );
            return false;
        }
    };
    match restore_verdict(&current, &recorded.mutant, &recorded.pre_image) {
        RestoreVerdict::Safe => {
            // The command may have replaced the path with a symlink since the run began, and
            // `fs::write` follows one — over whatever sits at the far end, which has included
            // this record's own files.
            if fs::symlink_metadata(target).is_ok_and(|m| m.file_type().is_symlink()) {
                complain!(
                    "{file} is now a symlink; refusing to write through it. The pre-image is at \
                     {}/pre-image.",
                    record.dir.display()
                );
                return false;
            }
            if let Err(e) = fs::write(target, &recorded.pre_image) {
                complain!("cannot restore {file}: {e}");
                return false;
            }
            let _ = record.clear();
            say!("{file} restored");
            true
        }
        RestoreVerdict::AlreadyRestored => {
            let _ = record.clear();
            say!("{file} already held its pre-image; nothing to restore");
            true
        }
        RestoreVerdict::FileChanged => {
            // Writing the pre-image here would take the concurrent edit with it. The record stays
            // on disk so nothing is lost and the repair is the caller's to choose.
            //
            // There is deliberately no automatic discharge. `git apply --check` succeeding was
            // tried as a proof that the mutant had gone, and it is not one: a patch applies
            // wherever its text can be placed, at any offset, so a command that duplicated the
            // region satisfied the check while the mutant sat at the site — and the record, the
            // only copy of the pre-image, was then deleted. Discharging is the caller's word,
            // through `--discard`, which writes nothing.
            let patch = record.dir.join("mutation.patch");
            complain!(
                "{file} changed while the mutation was applied, so it is NOT the mutant this run \
                 wrote.\nNothing has been written. The pre-image is at {}/pre-image.",
                record.dir.display()
            );
            if patch.is_file() {
                let shown = patch.strip_prefix(root).unwrap_or(&patch).display();
                // A reverse matches the hunk's three context lines, so an edit within them makes
                // it refuse. `-C1` narrows that to one line either side and recovers an edit two
                // or more lines away; an edit adjacent to the mutation, or on it, reverses under
                // neither, as `design@mutate@restore-by-comparison` records the measurement.
                complain!(
                    "To reverse just the mutation, keeping your edit:\n  \
                     git apply -R {shown}\n\
                     If that refuses because your edit is inside the patch's context:\n  \
                     git apply -R -C1 {shown}\n\
                     Then, once you have checked the mutation is gone:\n  \
                     cargo x mutate restore --discard"
                );
            }
            false
        }
    }
}

pub(crate) fn restore_record(root: &Path, discard: bool) -> ExitCode {
    let Some(record) = locate_record(root) else {
        return ExitCode::from(2);
    };
    if !record.claimed() {
        say!("no mutation record; nothing to restore");
        return ExitCode::SUCCESS;
    }
    // A restore during a live run puts the pre-image back underneath it, so that run's command is
    // measured over unmutated code and reports SURVIVED with its `mutation applied` line already
    // printed — both trust anchors present, both wrong. This guards `--discard` as well, since
    // dropping a live run's record deletes the only copy of its pre-image.
    if record.owner_is_alive() {
        complain!(
            "the run that claimed the record at {} is still going. Restoring now would un-mutate \
             its file underneath it, and its verdict would be taken over unmutated code. Wait for \
             it to finish, or stop it first.",
            record.dir.display()
        );
        return ExitCode::from(2);
    }
    // The escape for a mutation the caller unwound by some route this tool cannot recognise.
    // It writes nothing to the target, so it cannot lose content whatever the tree holds; all
    // it discards is the harness's claim that something may still be applied.
    if discard {
        let dir = record.dir.clone();
        match record.clear() {
            Ok(()) => {
                say!("record at {} discarded; no file was written", dir.display());
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                complain!("cannot remove the record: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    // Claimed but never completed: a run was interrupted between taking the directory and writing
    // the record, which is before it touches the target. Nothing is mutated, and only the claim
    // is in the way.
    if !record.exists() {
        complain!(
            "the record at {} is incomplete, so the run that claimed it never reached the target. \
             Nothing needs restoring; `cargo x mutate restore --discard` drops the claim.",
            record.dir.display()
        );
        return ExitCode::FAILURE;
    }
    let recorded = match record.read() {
        Ok(r) => r,
        Err(e) => {
            complain!("cannot read the recovery record: {e}");
            // 1, not 2. The record still stands and the target may still hold the mutant, and
            // both of those are what 1 means here. This is the one restore failure that has
            // already passed the preconditions.
            return ExitCode::FAILURE;
        }
    };
    let target = root.join(&recorded.target);
    if unwind(root, &record, &target, &recorded.target) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Applies `patch` to `pre_image` in a scratch repository and returns the result, so the mutant
/// is known before the real file is touched.
fn apply_to_scratch(pre_image: &str, rel_path: &str, patch: &str) -> Result<String, String> {
    let dir = env::temp_dir().join(format!("mutate-apply-{}", std::process::id()));
    let result = apply_in(&dir, pre_image, rel_path, patch);
    let _ = fs::remove_dir_all(&dir);
    result
}

/// The paths a patch names in its `---`, `+++` and `diff --git` lines, without their `a/` and `b/`
/// prefixes, `/dev/null` left out.
fn patch_paths(patch: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in patch.lines() {
        let names: Vec<&str> = if let Some(rest) = line.strip_prefix("diff --git ") {
            rest.split_whitespace().collect()
        } else if let Some(rest) = line.strip_prefix("--- ").or(line.strip_prefix("+++ ")) {
            rest.split('\t').next().into_iter().collect()
        } else {
            continue;
        };
        for name in names {
            let name = name.trim();
            if name == "/dev/null" {
                continue;
            }
            let name = name
                .strip_prefix("a/")
                .or(name.strip_prefix("b/"))
                .unwrap_or(name);
            paths.push(name.to_string());
        }
    }
    paths
}

/// The body of [`apply_to_scratch`], whose caller removes `dir` on every path.
fn apply_in(dir: &Path, pre_image: &str, rel_path: &str, patch: &str) -> Result<String, String> {
    let _ = fs::remove_dir_all(dir);
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;

    let staged = dir.join(rel_path);
    if let Some(parent) = staged.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&staged, pre_image).map_err(|e| e.to_string())?;
    fs::write(dir.join("m.patch"), patch).map_err(|e| e.to_string())?;

    let init = Command::new("git")
        .args(["init", "-q", "."])
        .current_dir(dir)
        .output()
        .map_err(|e| e.to_string())?;
    if !init.status.success() {
        return Err(String::from_utf8_lossy(&init.stderr).into_owned());
    }
    let out = Command::new("git")
        .args(["apply", "m.patch"])
        .current_dir(dir)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        let e = String::from_utf8_lossy(&out.stderr).into_owned();
        let _ = fs::remove_dir_all(dir);
        return Err(e);
    }
    let mutant = fs::read_to_string(&staged).map_err(|e| e.to_string())?;
    let _ = fs::remove_dir_all(dir);
    Ok(mutant)
}
