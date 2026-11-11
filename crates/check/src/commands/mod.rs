//! Each of `check`'s commands, as a typed function a binary or another frontend calls, with no type
//! of an argument parser in it, per `design@cli@the-commands-are-a-library-module`.
//!
//! This module owns what is printed, and which exit code a run ends with, which the checks
//! deliberately do not: a check returns its findings, touches no file and spawns no process, which
//! is the property the whole shape rests on. The exit codes are `design@cli@exit-code-ladder`: 0
//! ran-and-clean, 1 ran-and-negative, 2 could-not-run. `Err` is could-not-run, which the caller prints
//! before it exits 2, and `index` and `check --fix` also return exit 2 themselves, having printed why.
//! **Each function prints as it works**, per `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`,
//! so a closed stdout ends the process with exit 2 rather than returning, as `output::write` says.
//! `checker` is every directory of the running binary's own source, per
//! `design@check@checker-source-literals-are-data`.
//!
//! **Each function takes a manifest its extensions have configured**, with
//! [`crate::extension::configure`], as a binary's dispatch does before it calls one, which nothing
//! enforces yet, per `issue@knowledge-architect@a-typed-command-trusts-its-caller-to-have-configured-the-manifest`: a file they
//! generate is outside every walk, and a table they refuse is phase 1 of `check`.

use std::path::Path;
use std::process::ExitCode;

pub mod output;
use output::{out, outln};

use crate::check::{Phase, Report};
use crate::extension::{Extension, Prepared, Purpose, Tree};
use crate::Manifest;

mod history;
pub use history::commits;

mod gathered;
pub use gathered::Gathered;

// What `create` takes of the install: `init` refuses a destination in `shipped`, writes `IMPORT_LINE`
// into a root CLAUDE.md it creates, and outside a worktree installs with `install_into`.
pub use crate::agents::{install_into, shipped, Installed, IMPORT_LINE};

// A binary's `main` refuses a build made from another checkout before any command, per
// `design@check@a-foreign-build-is-refused`.
pub use crate::build_origin::{refuse_a_foreign_build, Library};

/// What `check` judges, and whether it writes first. One value, so that fixing the working tree and
/// judging the staged tree cannot be asked together, per `design@check@fix-with-staged`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckMode {
    /// Judge the working tree.
    Working,
    /// Judge the tree git's index would commit, read from git objects.
    Staged,
    /// Apply the safe fixes to the working tree, then judge it.
    Fix,
}

/// Which tree `index` writes the generated files of, and where.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexMode {
    /// The working tree's, into the working tree.
    Working,
    /// The tree git's index would commit, into the index, touching no working-tree file.
    Staged,
}

/// Every check, over one walk: exit 0 when it finds nothing, 1 when it reports a finding, `Err` when it
/// could not run. `CheckMode::Fix` writes first, and after a write a failure is exit 1, not 2, as
/// `fix_then_check` says.
pub fn check(
    manifest: &Manifest,
    mode: CheckMode,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    match mode {
        CheckMode::Working => check_working(manifest, checker, extensions),
        CheckMode::Staged => history::check_staged(manifest, checker, extensions),
        CheckMode::Fix => fix_then_check(manifest, checker, extensions),
    }
}

/// Whether `check --fix` would stop before the generated files over the staged changes this tree
/// holds, since a partial commit of what it writes would differ from them, per
/// `design@check@fix-refusal-mixed-state`. A caller about to run the fix says why before it does, as
/// `new-component` does. No answer from git, and a staged side that cannot be computed, are `false`:
/// the fix then reports them itself.
pub fn staged_changes_stop_the_fix(
    root: &Path,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> bool {
    crate::git::index_differs_from_head(root).unwrap_or(false)
        && matches!(
            history::staged_generated(root, checker, extensions),
            Ok(Some(_))
        )
}

/// Regenerate every generated index, printing each.
///
/// **It writes only where the bytes differ**, per `design@check@index-takes-no-dry-run`: what a
/// generated file holds is a function of the walked tree, so rewriting an already current one
/// moves nothing but its mtime, and running this to look must cost nothing. It takes no dry-run
/// flag for the same reason: nothing here can lose content, so there is nothing for a dry run
/// to protect. `IndexMode::Staged` writes into git's index instead.
///
/// **Whether a generated file is current is not this command's question.**
/// The `generated` check of `cargo klarch check` is the gate, and it names the first line at which
/// the committed file and the regenerated one disagree.
///
/// `Err` is a refusal over an incomplete model or a destination, nothing written. A write that fails
/// is exit 2 with nothing yet written, and exit 1 after another was rewritten.
pub fn index(
    manifest: &Manifest,
    mode: IndexMode,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    match mode {
        IndexMode::Working => index_working(manifest, checker, extensions),
        IndexMode::Staged => history::index_staged(manifest, checker, extensions),
    }
}

/// Write the shipped agent files into the project, per `design@check@installed-files-committed` and
/// `design@check@owned-namespace-check`.
///
/// Exit 0 when the installed set is the shipped one afterwards, or the project declares no harness and
/// nothing is installed. `Err` over a manifest holding refused declarations, nothing installed, and
/// when the filesystem refused a write, the earlier writes left in place. It edits nothing outside the installer's namespace: a primer import line missing from
/// its first line is said, not written, because the root CLAUDE.md belongs to the project.
pub fn install_agent_skills(manifest: &Manifest) -> Result<ExitCode, String> {
    if !manifest.complaints().is_empty() {
        return Err(format!(
            "the manifest holds declarations this tool refused: `{} check` reports them. \
             Nothing was installed.",
            manifest.command()
        ));
    }
    if !manifest.serves_claude() {
        outln!("the project declares no agent harness: nothing was installed");
        return Ok(ExitCode::SUCCESS);
    }
    let shipped = crate::agents::shipped(manifest);
    let done = crate::agents::install(manifest.root(), &shipped)?;
    for rel in &done.written {
        outln!("wrote    {}", rel.display());
    }
    for rel in &done.deleted {
        outln!("deleted  {}", rel.display());
    }
    if done.written.is_empty() && done.deleted.is_empty() {
        outln!("the installed set is already the shipped one");
    }
    let primer = std::path::Path::new(crate::agents::PRIMER);
    if shipped.iter().any(|(p, _)| p == primer) {
        // A root CLAUDE.md that is missing or cannot be read as text is the check's to report,
        // with its own cause: the import line is said only of a file whose first line is read.
        let read =
            std::fs::read_to_string(manifest.root().join(crate::model::declared::AGENT_DOCUMENT));
        if read.is_ok_and(|claude| !crate::agents::imports_primer(&claude)) {
            outln!(
                "the root CLAUDE.md does not import the primer on its first line: make that \
                 line exactly `{}`, and remove it wherever else it stands",
                crate::agents::IMPORT_LINE
            );
        }
    }
    Ok(ExitCode::SUCCESS)
}

/// What a run gathers from the working tree, once the first three phases have found the model
/// complete.
///
/// **A writer refuses over an incomplete model.** An index generated over a model missing a
/// home, an unreadable file or a refused declaration lists rows nobody asked for, so a command
/// that writes a generated file asks this first and touches nothing when it refuses: the
/// answer is exit 2 naming the phase, and `check` is what reports the findings. Releases are
/// not needed by the phases this runs, so nothing is fetched to refuse.
pub fn complete_working_tree(
    manifest: &Manifest,
    model: &crate::Model,
) -> Result<Gathered, String> {
    let tree = crate::model::foundation::complete_working_tree(manifest, model)?;
    Ok(Gathered::beside(tree, manifest, model))
}

/// Refuse to run over a project whose `[project] checker-version` this binary does not satisfy,
/// per `design@check@installed-binary-version-check` and `design@check@checked-sentinel-values`.
///
/// **Each binary that dispatches a command of the checker calls it in its `main`**, right after
/// [`refuse_a_foreign_build`] and before it dispatches any command but `init`, its own included,
/// with the directories of its own source: the same list it hands each command as `checker`. `init`
/// goes to `init` of `knowledge-architect-create` before a project is located, since no manifest exists to read a pin from,
/// and the manifest it writes pins this library's version. No command calls it, so a test driving a command
/// in-process over a mock project needs no confirmed pin; the parse still requires the key.
///
/// A version is accepted when it equals the version of this library, the checker every binary
/// links; `"fixture"` when the project lies strictly inside this library's directory or one of
/// `checker_dirs`, a mock project of a library the binary links; `"self"` when this library's
/// directory lies inside, or is, the project, and the project's git tracks its `Cargo.toml`, a
/// tree the checker is built from rather than one a registry sits in. While the key's
/// own complaint stands, that complaint is the refusal, so no command runs without a key. Every
/// refusal is could-not-run, exit 2, per `design@cli@exit-code-ladder`.
pub fn refuse_another_version(manifest: &Manifest, checker_dirs: &[&Path]) -> Result<(), String> {
    let root = manifest.root().to_path_buf();
    let tracked = move |rel: &Path| {
        crate::git::git(&root)
            .args(["ls-files", "--"])
            .args([rel])
            .output()
            .is_ok_and(|out| !out.is_empty())
    };
    refuse_another_version_built_at(
        manifest,
        &crate::component_dir(),
        crate::CHECKER_VERSION,
        checker_dirs,
        &tracked,
    )
}

/// The same, with the library's directory, its version, and the question "does the project's
/// git track this path" as parameters, so a test can place them.
fn refuse_another_version_built_at(
    manifest: &Manifest,
    own: &Path,
    version: &str,
    checker_dirs: &[&Path],
    tracked: &dyn Fn(&Path) -> bool,
) -> Result<(), String> {
    use crate::model::declared::Pin;
    let repair = "set [project] checker-version to the version of the checker the project runs";
    let Some(pin) = manifest.pin() else {
        let complaint = manifest
            .pin_complaint()
            .map(|f| (f.what.clone(), f.action.clone()));
        let (what, action) = complaint.unwrap_or_else(|| {
            (
                "[project] checker-version is absent".to_string(),
                repair.to_string(),
            )
        });
        return Err(format!("{what}\n       {action}"));
    };
    let root = manifest.root();
    match pin {
        Pin::Version(pinned) if pinned == version => Ok(()),
        Pin::Version(pinned) => {
            let side = match (triple(version), triple(pinned)) {
                (Some(binary), Some(pin)) if binary < pin => "older than",
                (Some(_), Some(_)) => "newer than",
                _ => "another version than",
            };
            let fix = if side == "older than" {
                format!("run or build knowledge-architect {pinned}, the version the manifest pins")
            } else {
                format!(
                    "run knowledge-architect {pinned}, the version the manifest pins, or move the \
                     pin: read the changelog of each version from {pinned} to {version}, then set \
                     [project] checker-version = \"{version}\""
                )
            };
            Err(format!(
                "this binary runs knowledge-architect {version}, {side} the {pinned} the manifest \
                 pins\n       {fix}"
            ))
        }
        Pin::Fixture if is_a_library_fixture(root, own, checker_dirs) => Ok(()),
        Pin::Fixture => Err(format!(
            "\"fixture\" is only valid for a mock project inside a library this binary links\n       \
             {repair}"
        )),
        Pin::OwnBuild if is_its_own_build(root, own, tracked) => Ok(()),
        Pin::OwnBuild => Err(format!(
            "\"self\" is only valid where the checker is built from this tree\n       {repair}"
        )),
    }
}

/// A version's three integers, or `None` for a version that is not three integers that fit in
/// a `u64`, which only a binary's own version can be: the manifest's is refused at parse.
fn triple(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.').map(|p| p.parse::<u64>().ok());
    let triple = (parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(triple)
}

/// A path as the filesystem resolves it, or as written where it cannot, as `foreign_copy` in
/// `build_origin` reads it.
fn canonical(path: &Path) -> std::path::PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// Whether the project lies strictly inside `check`'s directory or one of `checker_dirs`: a mock
/// project of a library the running binary links.
///
/// Strictly: a library's own directory is no mock of it. Where an extension's crate is the
/// project's root, the binary names the root among its directories, and an equal path would let
/// the project's real manifest say `"fixture"`.
fn is_a_library_fixture(root: &Path, own: &Path, checker_dirs: &[&Path]) -> bool {
    let root = canonical(root);
    std::iter::once(own)
        .chain(checker_dirs.iter().copied())
        .map(canonical)
        .any(|dir| root != dir && root.starts_with(&dir))
}

/// Whether `check`'s directory lies inside, or is, the project, and the project's git tracks its
/// `Cargo.toml`: a tree the checker is built from.
///
/// The tracking is what tells a checkout of the checker from a tree that a registry sits in, such
/// as a project whose `CARGO_HOME` is one of its own directories, where a registry build's
/// directory also lies inside the root.
fn is_its_own_build(root: &Path, own: &Path, tracked: &dyn Fn(&Path) -> bool) -> bool {
    let (root, own) = (canonical(root), canonical(own));
    match own.strip_prefix(&root) {
        Ok(rel) => tracked(&rel.join("Cargo.toml")),
        Err(_) => false,
    }
}

/// Every observation the walk and the scanner produced, one per line. `Err` is a project it cannot read.
///
/// This is what the model was compared against the implementation it replaces with, and it is
/// the way to see what a check is actually being handed. Filtering it on a rule number is also
/// how a citation is located inside the file the rule index names, because markers and rule
/// tokens are separate kinds here and a text grep tells them apart from neither each other nor
/// from a rule number that is data.
pub fn model(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker)
        .map_err(|e| format!("cannot read the project: {e}"))?;
    let extra: Vec<crate::model::table::DumpRow> =
        extensions.iter().flat_map(|e| e.dump(&model)).collect();
    let dump = model.canonical_with(&extra);
    // The count goes to stderr so that redirecting stdout gives a file that is only
    // observations, while a reader still learns how much was walked. A document with nothing
    // in it produces no line, so the two numbers together are what say whether the walk agrees
    // with another implementation of it.
    eprintln!(
        "{} documents, {} observations",
        model.documents().len(),
        dump.lines().count()
    );
    out!("{dump}");
    Ok(ExitCode::SUCCESS)
}

/// Every check, over the working tree: `check`'s, then each extension's.
///
/// An extension prepares its tree here rather than inside a check: preparing may read the
/// archive or reach the network, and a check may do neither.
fn check_working(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;

    // What a check may not fetch for itself is fetched here, once.
    let gathered = Gathered::over(manifest, &model)?;
    let inputs = gathered.inputs();
    if let Err(stop) = crate::check::foundation(&model, manifest, &inputs) {
        let report = Report::stopped(stop, &model);
        print_report(&report, &model, None);
        return Ok(ExitCode::FAILURE);
    }

    // The foundation has passed, so what each extension reads is needed now and by nothing
    // before: a run that stopped earlier resolved and fetched nothing.
    let mut prepared: Vec<(&'static [&'static str], Box<dyn Prepared>)> = Vec::new();
    for extension in extensions.iter_mut() {
        let p = extension.prepare(
            manifest,
            &model,
            Tree::Checkout(manifest.root()),
            Purpose::Check,
        )?;
        prepared.push((extension.checks(), p));
    }
    let with: Vec<(&[&'static str], &dyn Prepared)> =
        prepared.iter().map(|(c, p)| (*c, p.as_ref())).collect();
    let report = crate::check::run_with(&model, manifest, &inputs, &with);
    print_report(&report, &model, None);
    Ok(if report.failed() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Apply the safe fixes `design@check@fix-scope` admits, then run the full check: `check --fix`, per
/// `design@check@check-fix-flag`, `design@check@safe-fix-definition` and
/// `design@check@fix-before-the-checks`.
///
/// **A fix is safe when its bytes are determined by the tree and the pinned version, and it
/// writes or removes only files of the installer's namespace or of the generated list.** Two pass that test: the install
/// of the agent files, and the generated files. Every other finding's repair is a choice, or
/// touches git or a hand-written file, and stays the reader's.
///
/// **No check writes; every write happens before the model the checks read is built.** The order:
/// a manifest that holds refused declarations writes nothing, and the check reports them; the
/// repairs of the installed set, only what the installed-file check reports, judged from git's
/// listing; the writer gate of phases 1 to 3 over a model rebuilt
/// from the tree as the install left it, which stops the run and writes nothing more; the refusal
/// of a partial commit's mismatch, where the index differs from HEAD and the staged tree needs
/// other generated files than the working tree; the generated files; then the full check, whose
/// report and exit code are the run's. The installed
/// files' bytes do not depend on the model, which is why they are repaired before that gate: a writer whose output
/// is read off the model never writes over an incomplete one.
///
/// Each file written or removed is listed before the report, so a session sees what to commit.
/// A failed write exits 2 when nothing was written yet and 1 after any write, as `index` does:
/// here 2 promises a caller that the tree is as they left it. `worklist --out` and
/// `install-agent-skills` make no such promise, per `path@cli@README.md`.
fn fix_then_check(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    if !manifest.complaints().is_empty() {
        return check_working(manifest, checker, extensions);
    }
    let mut written = 0usize;
    let failed = |written: usize, e: String| {
        eprintln!("error: {e}");
        Ok(if written == 0 {
            ExitCode::from(2)
        } else {
            ExitCode::FAILURE
        })
    };

    // The installed set is repaired from git's listing of the namespace, judged as the check
    // judges it, so `--fix` writes or deletes only what the check reports: never an ignored file
    // in the namespace, which the install's filesystem walk would delete, and never a copy that
    // differs only by its line endings.
    if manifest.serves_claude() {
        let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
        let gathered = Gathered::over(manifest, &model)?;
        let inputs = gathered.inputs();
        let repairs = crate::agents::repairs(manifest.root(), inputs.shipped, inputs.installed);
        if !repairs.is_empty() {
            let mut done = crate::agents::Installed::default();
            let outcome =
                crate::agents::apply(manifest.root(), inputs.shipped, &repairs, &mut done);
            for rel in &done.written {
                outln!("fixed: wrote {} (installed)", rel.display());
            }
            for rel in &done.deleted {
                outln!("fixed: removed {} (installed)", rel.display());
            }
            written += done.written.len() + done.deleted.len();
            if let Err(e) = outcome {
                return failed(written, e);
            }
        }
    }

    // The writer gate, over the tree as the install left it. A stop is reported as `check`
    // reports it; what was installed is already listed above. A deletion the install made is
    // unstaged, and the last phase reports it, after the generated files are written: staging
    // touches git, which no fix does.
    let model = match crate::Model::build(manifest, checker) {
        Ok(model) => model,
        Err(e) => return failed(written, e.to_string()),
    };
    let gathered = match Gathered::over(manifest, &model) {
        Ok(gathered) => gathered,
        Err(e) => return failed(written, e),
    };
    if let Err(stop) = crate::check::foundation(&model, manifest, &gathered.inputs()) {
        let report = Report::stopped(stop, &model);
        if written > 0 {
            outln!();
        }
        print_report(&report, &model, None);
        return Ok(ExitCode::FAILURE);
    }

    let generated = match generated_list(
        manifest,
        &model,
        &gathered.inputs(),
        Tree::Checkout(manifest.root()),
        extensions,
    ) {
        Ok(generated) => generated,
        Err(e) => return failed(written, e),
    };
    // **A partial commit's mismatch is refused before any generated file is written**, per
    // `design@check@fix-before-the-checks`: where the index differs from HEAD, the files written
    // from the working tree would, staged with a partial commit, make a commit whose tree fails.
    // It comes after the installed repairs, whose bytes are the same for both trees, and after
    // the gate, whose model the comparison reads. Where no staged tree is there to commit, an
    // unmerged index, a staged tree holding no manifest or one stopped in phases 1 to 3, nothing
    // is compared; any other
    // failure to compute the staged side refuses, since a comparison skipped in silence is the
    // hazard it guards against.
    let staged_differs = match crate::git::index_differs_from_head(manifest.root()) {
        Ok(differs) => differs,
        Err(e) => return failed(written, e.to_string()),
    };
    if staged_differs {
        let staged = match history::staged_generated(manifest.root(), checker, extensions) {
            Ok(staged) => staged,
            Err(e) => {
                return failed(
                    written,
                    format!(
                        "the generated files of the staged tree cannot be computed, so a partial \
                         commit cannot be told apart: {e}. No generated file was written."
                    ),
                )
            }
        };
        if let Some(staged) = staged {
            // Only a file `--fix` would write can carry the working tree's bytes into a partial
            // commit: one already current on disk is the user's, written by `index` or by hand,
            // and refusing over it would refuse a run that writes nothing, which `index` cannot
            // clear.
            let written_here: Vec<(std::path::PathBuf, String)> = generated
                .iter()
                .filter(|(rel, text)| {
                    std::fs::read_to_string(manifest.root().join(rel))
                        .ok()
                        .as_deref()
                        != Some(text.as_str())
                })
                .cloned()
                .collect();
            let differing: Vec<String> = differing_paths(&generated, &staged)
                .into_iter()
                .filter(|path| {
                    written_here
                        .iter()
                        .any(|(rel, _)| crate::model::escaped(&rel.display().to_string()) == *path)
                })
                .collect();
            if !differing.is_empty() {
                let command = manifest.command();
                return failed(
                    written,
                    format!(
                        "these generated files would differ between the working tree and the \
                         staged tree: {}\n  → to commit the staged changes: {command} index \
                         --staged, then {command} check --staged\n  → to fix the working tree as \
                         a whole: {command} index",
                        differing.join(", ")
                    ),
                );
            }
        }
    }
    if let Err(e) = check_destinations(manifest, &generated) {
        return failed(written, e);
    }
    for (rel, text) in generated {
        let path = manifest.root().join(&rel);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(text.as_str()) {
            continue;
        }
        if let Err(e) = std::fs::write(&path, &text) {
            return failed(written, format!("{}: {e}", path.display()));
        }
        written += 1;
        outln!("fixed: wrote {} (regenerated)", rel.display());
    }
    if written > 0 {
        outln!();
    }
    // The final check can still fail to run: a file it reads, or an extension's preparation. With
    // files already written, that is no could-not-run, which here promises an untouched tree.
    match check_working(manifest, checker, extensions) {
        Err(e) => failed(written, e),
        outcome => outcome,
    }
}

/// The summary first, the findings under it, then the note on untracked files when one is owed,
/// and the verdict on the last line.
///
/// The order is the whole point: a caller reading the tail of the output has to reach the
/// answer, and when the findings came first every `| tail` and every `| grep` for a count
/// printed a success-shaped report over a failing run.
fn print_report(report: &Report, model: &crate::Model, tree: Option<&str>) {
    out!("{}", counts(report, tree));
    if !report.findings.is_empty() {
        outln!();
        for finding in &report.findings {
            outln!("{finding}");
        }
        if let Some(note) = untracked_note(&report.findings, model.untracked()) {
            outln!("{note}");
        }
    }
    outln!("{}", verdict(report));
}

/// The files of `findings` that git does not track, named in one note, or `None`.
///
/// The walk reads an untracked file like a committed one, so a scratch file left in the tree
/// fails the run with findings that say nothing of where the file came from. The note says it,
/// and what to do with a file that belongs to the project and with one that does not. It sits
/// above the verdict, which stays the last line.
///
/// The manifest is left out: it is read because it is the manifest, not because the walk lists
/// it, so neither moving it out nor ignoring it is a way out, and the note would say both.
fn untracked_note(
    findings: &[crate::Finding],
    untracked: &std::collections::HashSet<std::path::PathBuf>,
) -> Option<String> {
    let files: std::collections::BTreeSet<&std::path::PathBuf> = findings
        .iter()
        .map(|f| &f.file)
        .filter(|file| untracked.contains(*file))
        .filter(|file| file.as_path() != std::path::Path::new(crate::MANIFEST_NAME))
        .collect();
    if files.is_empty() {
        return None;
    }
    let names: Vec<String> = files
        .iter()
        .map(|file| crate::model::escaped(&file.display().to_string()))
        .collect();
    let (count, verb) = match names.len() {
        1 => ("1 file".to_string(), "is"),
        n => (format!("{n} files"), "are"),
    };
    Some(format!(
        "{count} above {verb} untracked: {}\n    → the walk reads every file git does not ignore, \
         tracked or not: commit a file that belongs to the project, and move out or ignore one \
         that does not",
        names.join(", ")
    ))
}

/// The generated files `index` would write for the working tree, or `None` where its model stops
/// before the last phase or a file cannot be generated, where `index` would refuse.
pub(super) fn working_generated(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Option<Vec<(std::path::PathBuf, String)>> {
    let model = crate::Model::build(manifest, checker).ok()?;
    let gathered = Gathered::over(manifest, &model).ok()?;
    crate::check::foundation(&model, manifest, &gathered.inputs()).ok()?;
    generated_list(
        manifest,
        &model,
        &gathered.inputs(),
        Tree::Checkout(manifest.root()),
        extensions,
    )
    .ok()
}

fn index_working(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    // The gate first: a file an extension reads that is not there is phase 2's finding, named
    // by path, and reading it before the gate would turn that into an error naming nothing.
    let gathered = complete_working_tree(manifest, &model)?;
    let generated = generated_list(
        manifest,
        &model,
        &gathered.inputs(),
        Tree::Checkout(manifest.root()),
        extensions,
    )?;
    check_destinations(manifest, &generated).map_err(|e| format!("{e} Nothing was written."))?;

    let mut written = 0usize;
    for (rel, text) in generated {
        let path = manifest.root().join(&rel);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(text.as_str()) {
            outln!("{:<40} already current", rel.display());
            continue;
        }
        if let Err(e) = std::fs::write(&path, &text) {
            eprintln!("error: {}: {e}", path.display());
            // Once anything has been written the run is no longer a could-not-run, and here 2
            // promises a caller that the tree is as they left it. The checks above catch the
            // reachable causes; a permission or device failure between them and here is what
            // this arm is for.
            return Ok(if written == 0 {
                ExitCode::from(2)
            } else {
                ExitCode::FAILURE
            });
        }
        written += 1;
        outln!("{:<40} rewritten", rel.display());
    }
    Ok(ExitCode::SUCCESS)
}

/// Every generated file, destination and bytes, over a model the writer gate has passed: each
/// extension's, then one index per file-register instance.
///
/// One function for `index`, `index --staged` and `check --fix`, so the three generate the same
/// files from one tree, per `design@model@one-list-of-generated-files`. `tree` is the tree each
/// extension is prepared over: the checkout, or the staged snapshot. Every generated file comes in
/// one call, so a flag choosing between them buys nothing and cannot be given an invalid
/// combination. `inputs` answers which instance directories are there, and an instance without one
/// contributes no index rather than having its home created here.
fn generated_list(
    manifest: &Manifest,
    model: &crate::Model,
    inputs: &crate::check::Inputs,
    tree: Tree,
    extensions: &mut [Box<dyn Extension>],
) -> Result<Vec<(std::path::PathBuf, String)>, String> {
    let mut generated = Vec::new();
    for extension in extensions.iter_mut() {
        let prepared = extension.prepare(manifest, model, tree, Purpose::Index)?;
        generated.extend(
            prepared
                .generated(model, manifest)
                .into_iter()
                .map(|g| (g.rel, g.text)),
        );
    }
    generated.extend(crate::model::generated::file_register_indexes(
        model,
        manifest,
        inputs.present,
        inputs.directories,
    ));
    Ok(generated)
}

/// The paths whose generated bytes differ between two lists, one held by one list alone
/// included, sorted and each named once.
fn differing_paths(
    working: &[(std::path::PathBuf, String)],
    staged: &[(std::path::PathBuf, String)],
) -> Vec<String> {
    let working: std::collections::BTreeMap<_, _> = working.iter().cloned().collect();
    let staged: std::collections::BTreeMap<_, _> = staged.iter().cloned().collect();
    let mut paths: std::collections::BTreeSet<&std::path::PathBuf> = working.keys().collect();
    paths.extend(staged.keys());
    paths
        .into_iter()
        .filter(|rel| working.get(*rel) != staged.get(*rel))
        .map(|rel| crate::model::escaped(&rel.display().to_string()))
        .collect()
}

/// Refuse every destination that cannot be written safely, before any is written.
///
/// The error names the path and says nothing of what was written: the caller knows that, and
/// `check --fix` may already have installed files when it calls this.
///
/// Every destination is checked before any is written, so a run that wrote one index cannot then
/// fail on the next and exit 2 having changed the tree, per `design@cli@exit-code-ladder`. A missing directory here is the
/// manifest declaring one the tree does not have; creating it would paper over that, and the
/// registers check is what reports it.
///
/// **The missing-directory arm is reached by an extension's generated file** whose directory
/// the tree does not hold: the extension names the path before the walk, and nothing creates its
/// directory. A file-register index cannot reach it, since one is generated only for an instance
/// whose directory the survey found.
fn check_destinations(
    manifest: &Manifest,
    generated: &[(std::path::PathBuf, String)],
) -> Result<(), String> {
    for (rel, _) in generated {
        let path = manifest.root().join(rel);
        let dir = path
            .parent()
            .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
        if !dir.is_dir() {
            return Err(format!(
                "{}: the directory this index is generated into is not there.",
                dir.display()
            ));
        }
        // `fs::write` follows a symlink and writes through it, so a generated path that is one
        // would replace whatever sits at the far end — which is the one way this command could
        // destroy something it did not generate, and what `design@check@index-takes-no-dry-run`
        // needs to be false for its claim to hold. The sibling tool refuses one for the same
        // reason, in `target_is_mutable`.
        if std::fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!(
                "{}: this index is a symlink, and writing would follow it.",
                path.display()
            ));
        }
    }
    Ok(())
}

/// The last line, and the only one that states the outcome.
///
/// **It is derived from the finding list rather than tracked beside it**, so it cannot
/// disagree with the exit code: `Report::failed` is the same predicate over the same vector.
/// A caller scripting against this reads the exit code; a person reads this line.
fn verdict(report: &Report) -> String {
    match report.findings.len() {
        0 => "PASSED: no findings".to_string(),
        1 => "FAILED: 1 finding above".to_string(),
        n => format!("FAILED: {n} findings above"),
    }
}

/// What a run looked at, as the block printed above the findings.
///
/// **A check that did not run contributes nothing.** Its counts are not zero, they are
/// unasked, and a zero would read as "nothing found" for a check that never ran. The
/// `checked:` line is what makes that legible for a check that carries no count of its own,
/// such as `generated`: without it, a run of `generated` alone and a run that performed
/// nothing look the same.
/// The checker's directories as the summary line names them, comma-separated.
fn sources(dirs: &[std::path::PathBuf]) -> String {
    dirs.iter()
        .map(|d| d.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// `tree` names the tree walked, where it is not the working tree: plain `check` prints no
/// `tree:` line, so its output is what it was before `--staged` existed, and a `walk:` count
/// with no `tree:` line is of the working tree.
fn counts(report: &Report, tree: Option<&str>) -> String {
    use std::fmt::Write;

    let mut out = String::new();
    if report.phase != Phase::Content {
        // A stop names itself, and prints what every phase read: the walk. No family count
        // follows, because no family ran, and a zero would read as nothing found.
        let _ = write!(out, "\n{}", report.phase.stop_line(report.findings.len()));
        let _ = write!(out, "\nwalk: {} file(s)", report.structure.walked);
        if let Some(tree) = tree {
            let _ = write!(out, "\ntree: {tree}");
        }
        if !report.structure.checker_sources.is_empty() {
            let _ = write!(
                out,
                "\nchecker source: {}, {} file(s) with string literals read as data",
                sources(&report.structure.checker_sources),
                report.structure.checker_files
            );
        }
        return out;
    }
    // Every check the last phase performed, by name. The line is what makes a check that
    // carries no count of its own — `generated` — legible as having run.
    let checked: Vec<&str> = report
        .checks
        .iter()
        .copied()
        .filter(|name| !report.not_run.contains(name))
        .collect();
    let _ = write!(out, "\nchecked: {}", checked.join(", "));
    // Not a check: every check read this walk. Git supplies it, per
    // `design@model@git-supplies-the-walk`, so the count is what a reader compares between CI and
    // a local run. It prints on a failing run as readily as a passing one.
    let _ = write!(out, "\nwalk: {} file(s)", report.structure.walked);
    if let Some(tree) = tree {
        let _ = write!(out, "\ntree: {tree}");
    }
    if !report.not_run.is_empty() {
        // Not performed. Without this line the two failures are the same output: a check
        // that ran and found nothing, and one that could not run at all.
        let _ = write!(
            out,
            "\nNOT RUN: {} — the input it reads is not there",
            report.not_run.join(", ")
        );
    }

    // Not a check: it describes the walk every check read, and a count of zero in a
    // checkout that holds the tool is the loud failure the decision promises.
    if !report.structure.checker_sources.is_empty() {
        let _ = write!(
            out,
            "\nchecker source: {}, {} file(s) with string literals read as data",
            sources(&report.structure.checker_sources),
            report.structure.checker_files
        );
    }

    let s = &report.structure;
    let _ = write!(
        out,
        "\n\nregisters: {} component(s), {} location(s), {} instance(s), {} file entry(ies)",
        s.components, s.locations, s.instances, s.entries
    );
    let _ = write!(
        out,
        "\nreferences: {} entities defined, {} reference(s), {} link(s)",
        s.entities, s.references, s.links
    );
    // Each extension's block, after `check`'s lines, per
    // `design@check@an-extension-plugs-in-through-phased-hooks`.
    for summary in &report.summaries {
        out.push_str(summary);
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::{is_a_library_fixture, is_its_own_build, refuse_another_version_built_at};

    /// The claim: a path one list holds and the other does not differs, as two lists' different
    /// bytes for one path do, and each path is named once, sorted.
    #[test]
    fn a_generated_path_in_one_list_alone_differs() {
        let file = |rel: &str, text: &str| (std::path::PathBuf::from(rel), text.to_string());
        let working = [file("a.md", "1"), file("b.md", "1"), file("c.md", "1")];
        let staged = [file("a.md", "1"), file("b.md", "2"), file("d.md", "1")];
        assert_eq!(
            super::differing_paths(&working, &staged),
            vec!["b.md", "c.md", "d.md"]
        );
        assert!(super::differing_paths(&working, &working).is_empty());
    }
    use crate::Manifest;
    use std::path::{Path, PathBuf};

    /// A fresh directory under the system's temporary directory, removed when dropped.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(tag: &str) -> Self {
            let dir =
                std::env::temp_dir().join(format!("knowledge-pin-{tag}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("a scratch directory");
            Scratch(dir)
        }

        fn dir(&self, rel: &str) -> PathBuf {
            let dir = self.0.join(rel);
            std::fs::create_dir_all(&dir).expect("a directory");
            dir
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A manifest rooted at `root` whose key holds `value`.
    fn pinned_at(root: &Path, value: &str) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"{value}\"\nname = \"p\"\ncomponents = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n"
        );
        Manifest::parse(root, &text).expect("a declaration")
    }

    /// A `tracked` answer that holds for every path, and one that holds for none.
    fn always(_: &Path) -> bool {
        true
    }
    fn never(_: &Path) -> bool {
        false
    }

    /// The claim: `"fixture"` holds where the project lies strictly inside `check`'s directory or
    /// a checker directory, and `"self"` where `check`'s directory lies inside, or is, the project
    /// and the project tracks its `Cargo.toml`; a library directory outside the project, standing
    /// for the registry, confirms neither, nor does a registry inside the project that git does
    /// not track.
    #[test]
    fn the_sentinels_hold_by_where_the_library_and_the_project_lie() {
        let scratch = Scratch::new("predicates");
        let tree = scratch.dir("tree");
        let own_inside = scratch.dir("tree/crates/check");
        let mock = scratch.dir("tree/crates/check/tests/projects/m");
        let registry = scratch.dir("registry/knowledge-architect-0.2.0");
        let extension = scratch.dir("ext/crates/rules");
        let ext_mock = scratch.dir("ext/crates/rules/tests/projects/m");

        assert!(is_a_library_fixture(&mock, &own_inside, &[]));
        assert!(is_a_library_fixture(
            &ext_mock,
            &registry,
            &[extension.as_path()]
        ));
        assert!(
            !is_a_library_fixture(&own_inside, &own_inside, &[]),
            "a library is no mock of itself"
        );
        assert!(
            !is_a_library_fixture(&extension, &registry, &[extension.as_path()]),
            "an extension crate at the root"
        );
        assert!(!is_a_library_fixture(&tree, &own_inside, &[]));
        assert!(!is_a_library_fixture(&tree, &registry, &[]));
        assert!(!is_a_library_fixture(&ext_mock, &registry, &[]));

        assert!(is_its_own_build(&tree, &own_inside, &always));
        assert!(
            is_its_own_build(&own_inside, &own_inside, &always),
            "equal is inside"
        );
        assert!(
            !is_its_own_build(&tree, &own_inside, &never),
            "an untracked library is a registry's"
        );
        assert!(!is_its_own_build(&tree, &registry, &always));
        assert!(!is_its_own_build(&mock, &own_inside, &always));
        let asked = std::cell::RefCell::new(Vec::new());
        let recording = |rel: &Path| {
            asked.borrow_mut().push(rel.to_path_buf());
            true
        };
        assert!(is_its_own_build(&tree, &own_inside, &recording));
        assert_eq!(
            *asked.borrow(),
            vec![PathBuf::from("crates/check/Cargo.toml")]
        );
    }

    /// The claim: a project reached through a symlink is read where the symlink resolves, so a
    /// mock reached through a link into `check`'s directory is a fixture.
    #[test]
    fn the_sentinels_read_paths_as_they_resolve() {
        let scratch = Scratch::new("symlink");
        let own = scratch.dir("tree/crates/check");
        let mock = scratch.dir("tree/crates/check/tests/projects/m");
        let link = scratch.0.join("link-to-mock");
        std::os::unix::fs::symlink(&mock, &link).expect("a symlink");
        assert!(is_a_library_fixture(&link, &own, &[]));
    }

    /// The claim: a pin equal to the binary's version passes; a version older or newer than the
    /// binary's refuses, naming which side is older and the repair that moves toward the pin;
    /// `"fixture"` and `"self"` pass where their predicate holds and refuse elsewhere; and a key
    /// absent or in none of the three forms refuses with its complaint's own text.
    #[test]
    fn the_refusal_follows_the_mapping_table() {
        let scratch = Scratch::new("refusal");
        let tree = scratch.dir("tree");
        let own_inside = scratch.dir("tree/crates/check");
        let mock = scratch.dir("tree/crates/check/tests/projects/m");
        let registry = scratch.dir("registry/knowledge-architect-0.2.0");
        let refuse = |root: &Path, value: &str, own: &Path| {
            refuse_another_version_built_at(&pinned_at(root, value), own, "0.2.0", &[], &always)
        };
        let err = |r: Result<(), String>| r.expect_err("a refusal");

        assert_eq!(refuse(&tree, "0.2.0", &registry), Ok(()));
        let older = err(refuse(&tree, "0.3.0", &registry));
        assert!(older.starts_with(
            "this binary runs knowledge-architect 0.2.0, older than the 0.3.0 the manifest pins"
        ));
        assert!(
            older.contains("run or build knowledge-architect 0.3.0"),
            "{older}"
        );
        let newer = err(refuse(&tree, "0.1.9", &registry));
        assert!(newer.starts_with(
            "this binary runs knowledge-architect 0.2.0, newer than the 0.1.9 the manifest pins"
        ));
        assert!(
            newer.contains("move the pin") && newer.contains("from 0.1.9 to 0.2.0"),
            "{newer}"
        );
        assert!(
            err(refuse(&tree, "0.10.0", &registry)).contains("older than"),
            "compared as integers"
        );
        assert_eq!(refuse(&mock, "fixture", &own_inside), Ok(()));
        assert!(err(refuse(&tree, "fixture", &registry)).starts_with(
            "\"fixture\" is only valid for a mock project inside a library this binary links"
        ));
        assert_eq!(refuse(&tree, "self", &own_inside), Ok(()));
        assert!(err(refuse(&tree, "self", &registry))
            .starts_with("\"self\" is only valid where the checker is built from this tree"));
        assert!(
            err(refuse(&mock, "self", &own_inside)).starts_with("\"self\""),
            "a mock is no own build"
        );

        // A key in none of the three forms, or none at all, refuses with its complaint's text.
        let bad = "[project]\nchecker-version = \"latest\"\nname = \"p\"\ncomponents = []\n\n\
                   [walk]\nskip-dirs = []\nskip-files = []\n";
        let malformed = Manifest::parse(&tree, bad).expect("a declaration");
        assert!(err(refuse_another_version_built_at(
            &malformed,
            &own_inside,
            "0.2.0",
            &[],
            &always
        ))
        .starts_with("[project] checker-version \"latest\" is not a version"));
        let text =
            "[project]\nname = \"p\"\ncomponents = []\n\n[walk]\nskip-dirs = []\nskip-files = []\n";
        let absent = Manifest::parse(&tree, text).expect("a declaration");
        assert!(err(refuse_another_version_built_at(
            &absent,
            &own_inside,
            "0.2.0",
            &[],
            &always
        ))
        .starts_with("[project] checker-version is absent"));
    }

    /// The claim: a binary whose own version is not three integers still refuses another pin,
    /// with the neutral wording.
    #[test]
    fn a_binary_version_that_is_not_three_integers_refuses_with_the_neutral_wording() {
        let scratch = Scratch::new("neutral");
        let tree = scratch.dir("tree");
        let registry = scratch.dir("registry");
        let r = refuse_another_version_built_at(
            &pinned_at(&tree, "0.2.0"),
            &registry,
            "0.3.0-dev",
            &[],
            &always,
        );
        assert!(r
            .expect_err("a refusal")
            .contains("another version than the 0.2.0"));
    }

    use super::{counts, Report};
    use crate::check::{Phase, CHECKS};

    fn report() -> Report {
        Report {
            phase: Phase::Content,
            findings: Vec::new(),
            structure: Default::default(),
            checks: CHECKS.to_vec(),
            not_run: Vec::new(),
            summaries: Vec::new(),
        }
    }

    /// A report whose every number is different from every other.
    ///
    /// Distinct values on purpose: with zeros everywhere, two counts printed in the wrong
    /// order render identically, so a line could read the reverse of what it means and no
    /// assertion would move.
    fn numbered() -> Report {
        let mut r = report();
        r.structure.components = 188;
        r.structure.locations = 199;
        r.structure.instances = 88;
        r.structure.entries = 99;
        r.structure.entities = 55;
        r.structure.references = 77;
        r.structure.links = 78;
        r.structure.checker_sources = vec!["tools/knowledge".into()];
        r.structure.checker_files = 200;
        r
    }

    #[test]
    fn every_count_is_rendered_in_the_position_its_label_promises() {
        let out = counts(&numbered(), None);
        for expected in [
            "registers: 188 component(s), 199 location(s), 88 instance(s), 99 file entry(ies)",
            "references: 55 entities defined, 77 reference(s), 78 link(s)",
            "checker source: tools/knowledge, 200 file(s) with string literals read as data",
        ] {
            assert!(out.contains(expected), "missing {expected:?} in {out}");
        }
    }

    #[test]
    fn each_extension_block_follows_the_lines_of_check_in_order() {
        let mut r = numbered();
        r.summaries = vec!["\nfirst: 1".to_string(), "\nsecond: 2".to_string()];
        let out = counts(&r, None);
        let references = out.find("references:").expect("`check` line");
        let first = out.find("first: 1").expect("the first block");
        let second = out.find("second: 2").expect("the second block");
        assert!(references < first && first < second, "{out}");
    }

    #[test]
    fn a_check_that_could_not_run_is_named_as_not_run_and_left_out_of_checked() {
        // The failure this guards is a check gated on an input that is not there: its name in
        // `checked:` would say it ran.
        let mut r = report();
        r.checks.extend(["history", "sources"]);
        r.not_run = vec!["history", "sources"];
        let out = counts(&r, None);
        assert!(out.contains("NOT RUN: history, sources"), "{out}");
        let checked = out
            .lines()
            .find(|l| l.starts_with("checked:"))
            .expect("the line");
        assert!(
            !checked.contains("sources") && !checked.contains("history"),
            "{checked}"
        );
        // Nothing is withheld when everything ran.
        assert!(!counts(&report(), None).contains("NOT RUN"));
    }

    #[test]
    fn a_full_run_names_every_check_it_performed() {
        // `generated` reports findings and counts nothing, so the `checked:` line is the
        // only thing separating "ran and found nothing" from "did not run".
        let mut r = report();
        r.checks.push("an-extension-check");
        let out = counts(&r, None);
        let checked = out
            .lines()
            .find(|l| l.starts_with("checked:"))
            .expect("the line");
        for name in CHECKS.iter().chain(&["an-extension-check"]) {
            assert!(
                checked.contains(name),
                "{name} should be named in {checked}"
            );
        }
    }

    #[test]
    fn a_stop_prints_its_line_and_no_check_count() {
        let mut r = report();
        r.phase = Phase::Tree;
        r.findings
            .push(crate::Finding::in_file("a.md", "what", "action"));
        r.structure.checker_sources = vec!["crates/check".into()];
        r.structure.checker_files = 27;
        let out = counts(&r, None);
        assert!(
            out.contains("phase 2: 1 finding(s); phases 3 and 4 were not judged"),
            "{out}"
        );
        // A stop still names the checker's directories, so the exemption's state is visible
        // in every run, per `design@check@checker-source-literals-are-data`.
        assert!(
            out.contains(
                "\nchecker source: crates/check, 27 file(s) with string literals read as data"
            ),
            "{out}"
        );
        assert!(out.contains("walk: "), "{out}");
        assert!(
            !out.contains("checked:") && !out.contains("registers:"),
            "{out}"
        );
    }
}
