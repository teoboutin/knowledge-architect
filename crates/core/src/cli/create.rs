//! The command that creates a project, `init`.
//!
//! **It writes where no project exists yet**, so it runs before [`super::locate`] and before the
//! version refusal, which read a manifest it is about to write. Every binary that dispatches
//! [`super::Command`] hands it here first, per `design@core@the-core-cli-is-a-library-module`.
//! Its no-loss guarantee is that every refusal comes before the first write: a refused run
//! leaves the filesystem as it found it, and exits 2.

use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

use super::output::outln;
use super::{InitArgs, Library, Vcs};
use crate::agents::IMPORT_LINE;
use crate::entity::{Anchors, Constructed};
use crate::extension::Extension;
use crate::manifest::{
    Shape, AGENT_DOCUMENT, ISSUE_REGISTER, MANIFEST_NAME, MILESTONE_REGISTER, OWNED_PREFIX,
    REJECTED_REGISTER, SPEC_REGISTER, TRIPWIRE_REGISTER,
};
use crate::Manifest;

/// Create a knowledge-architect project at `args.path`, as `klarch init` does.
///
/// A binary calls this before [`super::locate`], with the same `libraries` and `packages` it
/// hands [`super::refuse_a_foreign_build`] and the same `checker` and `extensions` it hands
/// [`super::run`]. `Err` is a refusal: nothing was written, and the binary exits 2 with it.
pub fn init(
    args: InitArgs,
    libraries: &[Library],
    packages: &[&str],
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let cwd =
        std::env::current_dir().map_err(|e| format!("cannot read the working directory: {e}"))?;
    let root = lexical(&cwd.join(&args.path));
    let name = match &args.name {
        Some(name) => name.clone(),
        None => root
            .file_name()
            .and_then(|n| n.to_str())
            .map(str::to_string)
            .ok_or_else(|| {
                format!(
                    "{} has no basename to name the project by: pass --name",
                    root.display()
                )
            })?,
    };

    // Every refusal before the first write.
    if root.join(MANIFEST_NAME).exists() {
        return Err(format!(
            "{} already holds {MANIFEST_NAME}: `init` creates a project, and this is one",
            root.display()
        ));
    }
    if root.exists() && !root.is_dir() {
        return Err(format!("{} exists and is not a directory", root.display()));
    }
    // A project's own skills take its name and a hyphen as their prefix by default, per
    // `design@agent-skills@project-name-prefix`; under a name that begins with the installer's
    // namespace, that default lies inside it, and the install deletes such files as unshipped.
    // A name equal to the namespace without its hyphen is accepted: its project takes another
    // prefix, as the setup skill says.
    if name.starts_with(OWNED_PREFIX) {
        return Err(format!(
            "the project name `{name}` begins with `{OWNED_PREFIX}`, the installer's \
             namespace: the project's own skills, named after it, would be deleted by every \
             install. Pass another --name"
        ));
    }
    let manifest = manifest_text(&name, args.command.as_deref());
    let parsed = Manifest::parse(&root, &manifest)
        .map_err(|e| format!("the manifest `init` would write does not parse: {e}"))?;
    if let Some(complaint) = parsed.complaints().first() {
        return Err(format!(
            "the manifest `init` would write is refused: {}. Pass another --name",
            complaint.what
        ));
    }
    if root.is_dir() {
        super::refuse_a_foreign_build(&root, libraries, packages)?;
    }
    let run_git = git_rule(&root, args.vcs)?;
    let documents = owed_documents(&parsed, &root)?;

    // The writes.
    std::fs::create_dir_all(&root).map_err(|e| format!("cannot create {}: {e}", root.display()))?;
    outln!(
        "creating a knowledge-architect project `{name}` in {}",
        root.display()
    );
    if run_git {
        crate::git::init_repository(&root).map_err(|e| e.to_string())?;
        outln!("created  a git repository");
    }
    std::fs::write(root.join(MANIFEST_NAME), manifest)
        .map_err(|e| format!("cannot write {MANIFEST_NAME}: {e}"))?;
    outln!("wrote    {MANIFEST_NAME}");
    let written = write_documents(&root, &documents);
    let manifest = Manifest::load(&root);
    match (written, manifest) {
        (Ok(()), Ok(manifest)) => finish(&manifest, checker, extensions),
        // Files were written, so the run is no longer one that touched nothing: exit 2 would
        // promise an untouched tree, per `design@core@exit-code-ladder`.
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("error: {e}");
            Ok(ExitCode::FAILURE)
        }
    }
}

/// The last part of a run that wrote: the installed set and the indexes, through the code
/// `check --fix` runs, then the check, whose verdict is the run's.
///
/// Outside every worktree the walk cannot run, per `design@core@failed-git-is-exit-two`, so
/// nothing more is done: the run said what it wrote, and says what comes next.
fn finish(
    manifest: &Manifest,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    if !crate::git::inside_work_tree(manifest.root()).unwrap_or(false) {
        outln!(
            "not inside a git repository: run `git init`, then `{} check --fix`, to install the \
             agent files and write the indexes",
            manifest.command()
        );
        return Ok(ExitCode::SUCCESS);
    }
    match super::fix_then_check(manifest, checker, extensions) {
        // A 2 from `--fix` means it wrote nothing, which is not true of this run.
        Ok(code) if code == ExitCode::from(2) => Ok(ExitCode::FAILURE),
        Ok(code) => Ok(code),
        Err(e) => {
            eprintln!("error: {e}");
            Ok(ExitCode::FAILURE)
        }
    }
}

/// One hand-written document a Component or the plans directory owes, and what `init` does
/// with it.
struct Document {
    /// Project-relative.
    rel: PathBuf,
    text: String,
    /// It exists already, in this shape or in the other shape of its heading register, and is
    /// left as it is.
    kept: bool,
}

/// Every document the root Component and the plans directory owe under `manifest`, each judged
/// against the filesystem before anything is written.
///
/// The list is not written down here: it is the required documents and the register homes the
/// check asks for, from [`Manifest::required_documents`] and [`Anchors::of`], with the plans
/// directory's README, which the check requires by its own rule. So a register declared, or a
/// document a Component comes to owe, reaches this with no edit. `Err` refuses the run: a
/// destination that is a symlink, a directory where a file is due, or under a file.
fn owed_documents(manifest: &Manifest, root: &Path) -> Result<Vec<Document>, String> {
    let anchors = Anchors::of(manifest, &[] as &[PathBuf]);
    let mut out = Vec::new();
    for anchor in anchors.all() {
        if anchor.is_component {
            for name in manifest.required_documents() {
                let text =
                    component_document(name, &anchor.name, anchor.path.as_os_str().is_empty());
                out.push((anchor.path.join(name), text, None));
            }
        }
        if anchor.constructed == Some(Constructed::Plans) {
            out.push((
                anchor.path.join("README.md"),
                "# Plans\n\nThe plan documents of work designed and not yet built.\n".to_string(),
                None,
            ));
        }
    }
    for (anchor, register, home) in anchors.instances() {
        let text = home_text(&register.name, &anchor.name);
        match register.shape {
            Shape::Heading => out.push((home.file, text, Some(home.readme))),
            Shape::File | Shape::Directory => out.push((home.readme, text, None)),
            // An item register lives inside a plan document and has no home of its own.
            Shape::Section => {}
        }
    }
    out.into_iter()
        .map(|(rel, text, other_shape)| {
            let path = root.join(&rel);
            refuse_unwritable(root, &path)?;
            let kept = path.is_file() || other_shape.is_some_and(|o| root.join(o).is_file());
            Ok(Document { rel, text, kept })
        })
        .collect()
}

/// `Err` where writing `path` could follow a symlink, would replace a directory, or would need a
/// directory where a file stands, so the run refuses before its first write, as `index` refuses
/// a symlinked destination, per `design@core@index-takes-no-dry-run`.
fn refuse_unwritable(root: &Path, path: &Path) -> Result<(), String> {
    for at in path
        .ancestors()
        .take_while(|a| a.starts_with(root) && *a != root)
    {
        let Ok(meta) = std::fs::symlink_metadata(at) else {
            continue;
        };
        let refusal = if meta.file_type().is_symlink() {
            Some("is a symlink")
        } else if at == path && meta.is_dir() {
            Some("is a directory where a file is due")
        } else if at != path && !meta.is_dir() {
            Some("is a file where a directory is due")
        } else {
            None
        };
        if let Some(why) = refusal {
            return Err(format!("{} {why}: nothing was written", at.display()));
        }
    }
    Ok(())
}

/// Write every document not kept, listing each.
fn write_documents(root: &Path, documents: &[Document]) -> Result<(), String> {
    for doc in documents {
        if doc.kept {
            outln!("kept     {}", doc.rel.display());
            continue;
        }
        let path = root.join(&doc.rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
        }
        std::fs::write(&path, &doc.text)
            .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        outln!("wrote    {}", doc.rel.display());
    }
    Ok(())
}

/// The starting text of a Component's README.md or CLAUDE.md. The root's CLAUDE.md is its title
/// and the primer's import line, and nothing else, since what every root would say alike is the
/// primer's, per `design@agent-skills@primer-limit`.
fn component_document(document: &str, name: &str, root: bool) -> String {
    match (document, root) {
        (AGENT_DOCUMENT, true) => format!("# {name}\n\n{IMPORT_LINE}\n"),
        (AGENT_DOCUMENT, false) => format!(
            "# {name}\n\nThe contracts and traps a developer of {name} needs: none recorded yet.\n"
        ),
        _ => format!("# {name}\n\nWhat {name} is for, and how to use it: not written yet.\n"),
    }
}

/// The starting text of a register's home under `anchor`: a title and one sentence on what it
/// holds, with no heading at a register's level, no reference and no link, so it passes every
/// check in any project. No goal is written: the goals are the owner's to state.
fn home_text(register: &str, anchor: &str) -> String {
    let (title, holds) = match register {
        "goal" => (
            "Goals",
            format!("What {anchor} is for. No entry yet: the goals are stated with the owner."),
        ),
        "design" => (
            "Design",
            format!("How {anchor} is built, and why. No entry yet."),
        ),
        REJECTED_REGISTER => (
            "Rejected designs",
            format!("The designs {anchor} did not keep, and why. No entry yet."),
        ),
        TRIPWIRE_REGISTER => (
            "Tripwires",
            format!("The evidence that would reopen a decision of {anchor}. No entry yet."),
        ),
        ISSUE_REGISTER => (
            "Open issues",
            format!("One file per outstanding item of {anchor}. No entry yet."),
        ),
        SPEC_REGISTER => ("Specs", "No entry yet.".to_string()),
        MILESTONE_REGISTER => ("Milestones", "No entry yet.".to_string()),
        other => (
            other,
            format!("The entries of the {other} register. No entry yet."),
        ),
    };
    format!("# {title}\n\n{holds}\n")
}

/// Whether `init` runs `git init` at `root`, or `Err` where its rule refuses.
///
/// Inside a worktree no repository is created unless `--vcs git` asks: a project in a
/// subdirectory of a repository is legal. Outside every worktree a new or empty directory gets
/// one, as `cargo init` gives one. A directory there that already holds files is refused unless
/// `--vcs` decides, since an accidental run would leave its files untracked beside the new ones,
/// where a `git clean` deletes both: the owner's choice of that guard over `cargo init`'s plain
/// rule is recorded in `spec@plans@init-and-new-component`.
fn git_rule(root: &Path, vcs: Option<Vcs>) -> Result<bool, String> {
    match vcs {
        Some(Vcs::Git) => return Ok(true),
        Some(Vcs::None) => return Ok(false),
        None => {}
    }
    let existing = root
        .ancestors()
        .find(|dir| dir.is_dir())
        .ok_or_else(|| format!("no directory at or above {} exists", root.display()))?;
    if crate::git::inside_work_tree(existing).map_err(|e| e.to_string())? {
        return Ok(false);
    }
    let holds_files = root.is_dir()
        && std::fs::read_dir(root)
            .map_err(|e| format!("cannot read {}: {e}", root.display()))?
            .next()
            .is_some();
    if holds_files {
        return Err(format!(
            "{} is outside every git repository and already holds files: `git init` there \
             would leave them untracked beside the project's. Pass --vcs git to create the \
             repository anyway, or --vcs none to create none",
            root.display()
        ));
    }
    Ok(true)
}

/// The manifest `init` writes: the setup skill's smallest one, named and pinned.
fn manifest_text(name: &str, command: Option<&str>) -> String {
    let command = command
        .map(|c| format!("command = {}\n", toml_string(c)))
        .unwrap_or_default();
    format!(
        "[project]\n\
         name = {}\n\
         checker-version = {}\n\
         {command}\
         components = []\n\
         \n\
         [walk]\n\
         skip-dirs = []\n\
         skip-files = []\n\
         exclude = []\n",
        toml_string(name),
        toml_string(crate::CHECKER_VERSION),
    )
}

/// A TOML basic string holding `text`.
fn toml_string(text: &str) -> String {
    toml::Value::String(text.to_string()).to_string()
}

/// `path` with every `.` and `..` folded away, without asking the filesystem, which may not hold
/// it yet.
fn lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for part in path.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim: a binary that hands `init` to [`super::super::run`] inside a project gets a
    /// refusal naming `cli::init`, not a run of anything.
    #[test]
    fn run_refuses_init() {
        let manifest = Manifest::parse(
            Path::new("/nowhere"),
            &manifest_text("probe", None).replace(crate::CHECKER_VERSION, "fixture"),
        )
        .expect("the written manifest parses");
        let args = InitArgs {
            path: PathBuf::from("."),
            name: None,
            command: None,
            vcs: None,
        };
        let outcome = super::super::run(super::super::Command::Init(args), &manifest, &[], &mut []);
        let err = outcome.expect_err("refused");
        assert!(err.contains("cli::init"), "{err}");
    }

    /// The claim: `.` and `..` fold away without the filesystem.
    #[test]
    fn lexical_folds_dots() {
        assert_eq!(lexical(Path::new("/a/./b/../c")), PathBuf::from("/a/c"));
    }
}
