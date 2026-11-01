//! The harness: each function establishes one of the two properties a hand mutation cannot
//! establish for itself, that the mutation reached the code that ran, and that removing it removes
//! nothing else, or carries them across a kill, as the recovery record does.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Three lines either side of a change, which is what `diff -u` and `git apply` both default to.
const CONTEXT: usize = 3;

/// Why a find/replace mutation could not be planned. Every variant refuses before anything is
/// written: a mutation that cannot be shown to be unique is one whose outcome says nothing.
#[derive(Debug, PartialEq, Eq)]
pub enum PlanError {
    /// The pattern is absent. The mutation would silently not apply — the failure that reports
    /// "nothing caught it" about unmutated code.
    NotFound,
    /// The pattern occurs more than once, with the count. Replacing all of them is a different
    /// mutation from the one the author described, and replacing the first is a coin toss.
    NotUnique(usize),
    /// The replacement equals the pattern, so the file would not change and the run would
    /// measure the unmutated tree.
    NoChange,
    /// An empty pattern matches at every position; the count would be meaningless.
    EmptyPattern,
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::NotFound => write!(
                f,
                "the pattern does not occur in the target; the mutation would not have applied"
            ),
            PlanError::NotUnique(n) => write!(
                f,
                "the pattern occurs {n} times; a mutation must name exactly one site"
            ),
            PlanError::NoChange => {
                write!(
                    f,
                    "the replacement equals the pattern; the file would not change"
                )
            }
            PlanError::EmptyPattern => write!(f, "the pattern is empty"),
        }
    }
}

/// The mutated source, or why the mutation cannot be shown to name one site.
///
/// This is the `--replace` form's whole assertion. `str::replace` replaces every occurrence and
/// a presence check (`assert old in s`) does not see the difference, which is how a duplicated
/// edit reaches a file unnoticed.
pub fn plan_replace(source: &str, old: &str, new: &str) -> Result<String, PlanError> {
    if old.is_empty() {
        return Err(PlanError::EmptyPattern);
    }
    if old == new {
        return Err(PlanError::NoChange);
    }
    match count_placements(source, old) {
        0 => Err(PlanError::NotFound),
        1 => Ok(source.replacen(old, new, 1)),
        n => Err(PlanError::NotUnique(n)),
    }
}

/// How many positions `pat` occurs at, counting overlapping ones.
///
/// `str::matches` counts non-overlapping matches, so it reports `AAAA` as holding one `AAA`. The
/// claim this function serves is that the caller named **one site**, and `AAAA` offers two, so the
/// non-overlapping count would let an ambiguous mutation through with a verdict about whichever
/// placement `replacen` happened to take.
fn count_placements(source: &str, pat: &str) -> usize {
    if pat.is_empty() {
        return 0;
    }
    let mut found = 0;
    let mut from = 0;
    while let Some(i) = source[from..].find(pat) {
        found += 1;
        from += i + 1;
        // `find` panics on a slice that starts inside a character, so step to the next boundary.
        while from < source.len() && !source.is_char_boundary(from) {
            from += 1;
        }
        if from >= source.len() {
            break;
        }
    }
    found
}

/// Lines without their terminators, plus whether the text ended with one.
///
/// Unified diff distinguishes a file ending in a newline from one that does not, and marks the
/// difference; a generator that drops it produces a patch `git apply` refuses.
fn split_lines(s: &str) -> (Vec<&str>, bool) {
    let ends_with_newline = s.ends_with('\n');
    let body = if ends_with_newline {
        &s[..s.len() - 1]
    } else {
        s
    };
    if body.is_empty() {
        return (
            if ends_with_newline {
                vec![""]
            } else {
                Vec::new()
            },
            ends_with_newline,
        );
    }
    (body.split('\n').collect(), ends_with_newline)
}

/// A unified diff over one contiguous change, `None` when the two texts are equal.
///
/// One hunk is enough because the mutations this harness applies are one contiguous edit: the
/// `--replace` form substitutes one span, and a caller supplying its own patch does not come
/// here.
///
/// The hunk carries [`CONTEXT`] lines either side. A plain `git apply -R` matches all of them, so
/// an edit anywhere inside the hunk makes it refuse; `git apply -R -C1` narrows the match to one
/// line either side and recovers an edit two or more lines from the mutation. Neither reverses an
/// edit adjacent to the mutated line or on it, as `design@mutate@restore-by-comparison` records.
///
/// `path` is the path relative to the project's root that the `a/` and `b/` headers name, so the
/// patch applies with `git apply` run from the project's root.
pub fn single_hunk_diff(pre: &str, post: &str, path: &str) -> Option<String> {
    if pre == post {
        return None;
    }
    let (pre_lines, pre_nl) = split_lines(pre);
    let (post_lines, post_nl) = split_lines(post);

    // The changed span is what survives trimming the lines both texts share at each end. The
    // suffix is bounded so the two spans cannot overlap when one text is a prefix of the other.
    let mut prefix = pre_lines
        .iter()
        .zip(post_lines.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut suffix = {
        let max = pre_lines.len().min(post_lines.len()) - prefix;
        pre_lines
            .iter()
            .rev()
            .zip(post_lines.iter().rev())
            .take_while(|(a, b)| a == b)
            .count()
            .min(max)
    };

    // A line's terminator is part of it, and `split_lines` dropped it, so two final lines with
    // equal text but different terminators compare equal here and would be trimmed into the
    // common region — leaving a hunk with no changed line, which `git apply` reports as a
    // corrupt patch. When the terminators differ the final line is the changed one, whatever its
    // text.
    if pre_nl != post_nl {
        suffix = 0;
        prefix = prefix.min(pre_lines.len().min(post_lines.len()).saturating_sub(1));
    }

    let start = prefix.saturating_sub(CONTEXT);
    let pre_end = (pre_lines.len() - suffix + CONTEXT).min(pre_lines.len());
    let post_end = (post_lines.len() - suffix + CONTEXT).min(post_lines.len());
    let pre_count = pre_end - start;
    let post_count = post_end - start;

    // Unified diff numbers an empty side 0, not 1: the hunk sits *before* `start`.
    let pre_start = if pre_count == 0 { start } else { start + 1 };
    let post_start = if post_count == 0 { start } else { start + 1 };

    let mut out = format!("--- a/{path}\n+++ b/{path}\n");
    out.push_str(&format!(
        "@@ -{pre_start},{pre_count} +{post_start},{post_count} @@\n"
    ));

    // A missing final newline is a property of the last line of the *file*, so the marker is
    // owed only when the hunk actually reaches it.
    let mut emit = |sign: char, line: &str, is_last_of: Option<bool>| {
        out.push(sign);
        out.push_str(line);
        out.push('\n');
        if is_last_of == Some(false) {
            out.push_str("\\ No newline at end of file\n");
        }
    };

    for (i, line) in pre_lines[start..prefix].iter().enumerate() {
        let idx = start + i;
        // A leading context line is shared, so it is the last line of both or of neither.
        let last = (idx + 1 == pre_lines.len()).then_some(pre_nl);
        emit(' ', line, last);
    }
    for (i, line) in pre_lines[prefix..pre_lines.len() - suffix]
        .iter()
        .enumerate()
    {
        let idx = prefix + i;
        let last = (idx + 1 == pre_lines.len()).then_some(pre_nl);
        emit('-', line, last);
    }
    for (i, line) in post_lines[prefix..post_lines.len() - suffix]
        .iter()
        .enumerate()
    {
        let idx = prefix + i;
        let last = (idx + 1 == post_lines.len()).then_some(post_nl);
        emit('+', line, last);
    }
    for (i, line) in pre_lines[pre_lines.len() - suffix..pre_end]
        .iter()
        .enumerate()
    {
        let idx = pre_lines.len() - suffix + i;
        let last = (idx + 1 == pre_lines.len()).then_some(pre_nl);
        emit(' ', line, last);
    }

    Some(out)
}

/// What restoring the target may do, decided by comparing bytes rather than by patching.
#[derive(Debug, PartialEq, Eq)]
pub enum RestoreVerdict {
    /// The file is byte-identical to the mutant this harness wrote, so writing the pre-image
    /// back is exact and total: it can neither fail nor remove anything the harness did not put
    /// there.
    Safe,
    /// The file already holds the pre-image. A run killed between recording and applying leaves
    /// this state, and there is nothing to undo.
    AlreadyRestored,
    /// Something edited the file while the mutation was applied. Writing the pre-image back
    /// would take that edit with it, so the harness writes nothing and hands over the patch.
    FileChanged,
}

/// Which of the three states the target is in. The guarantee that a restore loses no content rests here: the
/// only branch that writes is the one where the file is exactly what this harness produced.
pub fn restore_verdict(current: &str, recorded_mutant: &str, recorded_pre: &str) -> RestoreVerdict {
    if current == recorded_mutant {
        RestoreVerdict::Safe
    } else if current == recorded_pre {
        RestoreVerdict::AlreadyRestored
    } else {
        RestoreVerdict::FileChanged
    }
}

/// A recovery record: the pre-image, the mutant, and the mutation's patch under one directory.
/// Whether one is complete is [`Record::exists`]'s question, and it is answered per file.
pub struct Record {
    pub dir: PathBuf,
}

/// A record read back from disk.
pub struct Recorded {
    /// The mutated file's path, relative to the project's root.
    pub target: String,
    pub pre_image: String,
    pub mutant: String,
    pub patch: Option<String>,
}

impl Record {
    pub fn at(dir: impl Into<PathBuf>) -> Self {
        Record { dir: dir.into() }
    }

    /// Whether a complete record is present. All three required files are checked rather than
    /// the manifest alone: the guarantee `cargo x mutate restore` needs is that a record reporting
    /// itself present has its pre-image on disk, and resting that on the order
    /// [`Record::write`] happens to use makes it unobservable. Checked this way, a half-written
    /// record is inert whatever interrupted it.
    pub fn exists(&self) -> bool {
        self.dir.join("manifest").is_file()
            && self.dir.join("pre-image").is_file()
            && self.dir.join("mutant").is_file()
    }

    /// Whether the directory is present at all, complete or not. A claim that never became a
    /// record still blocks the next one, and the caller has to be able to say so.
    pub fn claimed(&self) -> bool {
        self.dir.exists()
    }

    /// Takes exclusive ownership of the record directory, or fails because someone else holds it.
    ///
    /// `fs::create_dir` is atomic: of two runs racing here exactly one succeeds. Checking
    /// [`Record::exists`] and then writing is not the same thing — both runs pass the check, both
    /// write, and the four files end up a mix of two runs, so a restore compares against one run's
    /// mutant and writes the other run's pre-image into it.
    pub fn claim(&self) -> io::Result<()> {
        if let Some(parent) = self.dir.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::create_dir(&self.dir)?;
        // The owner's pid is how `restore` tells a crashed run from a live one.
        fs::write(self.dir.join("owner-pid"), std::process::id().to_string())?;
        Ok(())
    }

    /// Whether the run that claimed this record is still running. A restore during a live run
    /// un-mutates the file under it, so the command's verdict is taken over unmutated code.
    /// Unreadable or unparsable means no owner, which is the safe answer: it permits the restore.
    pub fn owner_is_alive(&self) -> bool {
        fs::read_to_string(self.dir.join("owner-pid"))
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .is_some_and(|pid| pid != std::process::id() && process_is_alive(pid))
    }

    /// Writes the record. Every byte of it lands before the caller touches the target, so a hard
    /// kill at any later instant leaves the pre-image recoverable — the property
    /// `cargo x mutate restore` depends on.
    pub fn write(
        &self,
        target: &str,
        pre_image: &str,
        mutant: &str,
        patch: Option<&str>,
    ) -> io::Result<()> {
        fs::write(self.dir.join("pre-image"), pre_image)?;
        fs::write(self.dir.join("mutant"), mutant)?;
        match patch {
            Some(p) => fs::write(self.dir.join("mutation.patch"), p)?,
            None => {
                let _ = fs::remove_file(self.dir.join("mutation.patch"));
            }
        }
        fs::write(self.dir.join("manifest"), target)?;
        Ok(())
    }

    pub fn read(&self) -> io::Result<Recorded> {
        Ok(Recorded {
            target: fs::read_to_string(self.dir.join("manifest"))?,
            pre_image: fs::read_to_string(self.dir.join("pre-image"))?,
            mutant: fs::read_to_string(self.dir.join("mutant"))?,
            patch: fs::read_to_string(self.dir.join("mutation.patch")).ok(),
        })
    }

    /// Removes the record. Called only after the target is back to its pre-image, so a record
    /// left on disk always means a mutation that was not unwound.
    pub fn clear(&self) -> io::Result<()> {
        if self.dir.exists() {
            fs::remove_dir_all(&self.dir)?;
        }
        Ok(())
    }
}

/// The recovery record's directory, inside the git directory of the repository that holds
/// `root`, as git resolves `knowledge-architect-mutate` there with `git rev-parse --git-path`.
///
/// There a record survives a `cargo clean` and a `git clean`, no project ignores it by hand, the
/// checker's walk never lists it, and each worktree of a repository gets its own. `None` when git
/// cannot answer, as outside a repository.
pub fn record_dir(root: &Path) -> Option<PathBuf> {
    let out = Command::new("git")
        .args(["rev-parse", "--git-path", "knowledge-architect-mutate"])
        .current_dir(root)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let path = String::from_utf8(out.stdout).ok()?;
    let path = Path::new(path.trim_end_matches(['\n', '\r']));
    // git answers relative to the directory it ran in, unless the git directory is elsewhere.
    Some(root.join(path))
}

/// Whether a process with this pid is running. On Linux `/proc` answers; on another Unix,
/// `kill -0`. Where neither answers, the process reads as gone, which permits a restore.
fn process_is_alive(pid: u32) -> bool {
    if cfg!(target_os = "linux") {
        return Path::new(&format!("/proc/{pid}")).exists();
    }
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Whether `file` names a location this harness may mutate: inside the project, and reached
/// without following a symlink.
///
/// Both halves are about the restore rather than about tidiness. An absolute path escapes the
/// project entirely — `Path::join` discards `root` when its argument is absolute — and the
/// generated patch's `a/`/`b/` headers are then meaningless, so the fallback leg is dead. A
/// symlink is worse: `fs::write` follows it, so the restore writes the pre-image through the link
/// and over whatever sits at the far end, which has included the record's own files.
pub fn target_is_mutable(root: &Path, file: &str) -> Result<PathBuf, String> {
    let rel = Path::new(file);
    if rel.is_absolute() {
        return Err(format!(
            "{file} is an absolute path; --file is relative to the project's root"
        ));
    }
    if rel.components().any(|c| c.as_os_str() == "..") {
        return Err(format!(
            "{file} leaves the project; --file may not contain `..`"
        ));
    }
    // The record lives in the git directory, so a target there could hide the record from the
    // recovery that has to find it.
    if rel.components().any(|c| c.as_os_str() == ".git") {
        return Err(format!(
            "{file} is inside the git directory; --file names a file of the working tree"
        ));
    }
    // Every component is checked, not the last alone: a symlinked directory on the way would
    // take the mutation, and the restore, out of the project.
    let mut walked = root.to_path_buf();
    for component in rel.components() {
        walked.push(component);
        if fs::symlink_metadata(&walked).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!(
                "{file} goes through a symlink; a restore would write through it and out of the \
                 project"
            ));
        }
    }
    let target = root.join(rel);
    match fs::symlink_metadata(&target) {
        Ok(m) if m.is_file() => Ok(target),
        // A FIFO or a device would block the read, or take a write that no restore undoes.
        Ok(_) => Err(format!("{file} is not a regular file")),
        Err(e) => Err(format!("cannot stat {file}: {e}")),
    }
}
