//! The command that creates a project, `init`.
//!
//! **It writes where no project exists yet**, so it runs before [`super::locate`] and before the
//! version refusal, which read a manifest it is about to write. Every binary that dispatches
//! [`super::Command`] hands it here first, per `design@core@the-core-cli-is-a-library-module`.
//! Its no-loss guarantee is that every refusal comes before the first write, per
//! `design@core@init-asks-no-confirmation`: a refused run leaves the filesystem as it found it,
//! and exits 2. Once it has written, a failure exits 1; a stdout closed early still exits 2, as
//! in every command, and `check --fix` completes that run.

use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

use super::output::outln;
use super::{InitArgs, Library, NewComponentArgs, Vcs};
use crate::agents::IMPORT_LINE;
use crate::entity::{Anchor, Anchors, Constructed};
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
        None => match root.file_name() {
            Some(n) => n.to_str().map(str::to_string).ok_or_else(|| {
                format!(
                    "{}'s basename is not UTF-8, and a project's name is: pass --name",
                    root.display()
                )
            })?,
            None => {
                return Err(format!(
                    "{} has no basename to name the project by: pass --name",
                    root.display()
                ))
            }
        },
    };

    // Every refusal before the first write.
    // Anything at the manifest's path refuses, a dangling symlink included, which `exists`
    // would not see and a write would follow out of the project.
    if std::fs::symlink_metadata(root.join(MANIFEST_NAME)).is_ok() {
        return Err(format!(
            "{} already holds {MANIFEST_NAME}: `init` creates a project, and this is one",
            root.display()
        ));
    }
    // The nearest path at or above PATH that exists is a directory, or the first write fails: a
    // PATH under a file, or a dangling symlink at PATH.
    if let Some(found) = root
        .ancestors()
        .find(|dir| std::fs::symlink_metadata(dir).is_ok())
    {
        if !found.is_dir() {
            return Err(format!(
                "{} lies at or under {}, which is not a directory",
                root.display(),
                found.display()
            ));
        }
    }
    // A repository's `.git` directory is no walk's, as a Component there is refused.
    if root.components().any(|part| part.as_os_str() == ".git") {
        return Err(format!(
            "{} lies in a git directory, which no walk reads",
            root.display()
        ));
    }
    // Per `design@core@init-refuses-the-installer-s-prefix`: a project's own skills take its
    // name and a hyphen as their prefix by default, per
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
    let mut components = args
        .components
        .iter()
        .map(|dir| component_path(&root, &lexical(&root.join(dir))))
        .collect::<Result<Vec<_>, _>>()?;
    let mut command = args.command.clone();
    if args.rust {
        // `--rust` writes a workspace of its own. Under a directory holding a Cargo.toml, cargo
        // would read that manifest as an enclosing workspace, and a `members` glob covering PATH
        // breaks it with two workspace roots; adding the maintenance crate to an existing
        // workspace or package is a design of its own, per `design@core@rust-scaffold`.
        if let Some(cargo) = root
            .ancestors()
            .map(|dir| dir.join("Cargo.toml"))
            .find(|path| path.exists())
        {
            return Err(format!(
                "{} exists: `--rust` writes a Cargo workspace of its own, and adding the \
                 maintenance crate to an existing workspace or package is left to the project. \
                 Nothing was written.",
                cargo.display()
            ));
        }
        components.push(XTASK_DIR.to_string());
        command.get_or_insert_with(|| "cargo klarch".to_string());
    }
    let manifest = manifest_text(&name, command.as_deref(), &components);
    let parsed = Manifest::parse(&root, &manifest)
        .map_err(|e| format!("the manifest `init` would write does not parse: {e}"))?;
    if let Some(complaint) = parsed.complaints().first() {
        return Err(format!(
            "the manifest `init` would write is refused: {}. Pass another --name or \
             --component",
            complaint.what
        ));
    }
    if root.is_dir() {
        super::refuse_a_foreign_build(&root, libraries, packages)?;
    }
    // Every invocation keeps `GIT_INDEX_FILE`, since a pre-commit hook gets it for the index its
    // commit records, per `design@core@repository-variables-dropped`. `init` belongs to no commit:
    // a repository it creates has no index the variable can name, so git would read and stage
    // through another one, and `init` refuses it. The variables that select another repository
    // are removed from every invocation, `init`'s included.
    if std::env::var_os("GIT_INDEX_FILE").is_some() {
        return Err(format!(
            "GIT_INDEX_FILE is set, so git would stage into another index than {}'s: run `init` \
             without it. Nothing was written.",
            root.display()
        ));
    }
    let run_git = git_rule(&root, args.vcs)?;
    if !run_git {
        refuse_ignored(&root, &components)?;
    }
    let mut documents = owed_documents(&parsed, &root, |_| true)?;
    let mut ignore = None;
    if args.rust {
        documents.extend(rust_files(&root, &name)?);
        ignore = Some(read_gitignore(&root)?);
    }
    let mut destinations: Vec<PathBuf> = documents.iter().map(|d| d.rel.clone()).collect();
    destinations.push(PathBuf::from(MANIFEST_NAME));
    destinations.push(PathBuf::from(".gitignore"));
    for (rel, _) in crate::agents::shipped(&parsed) {
        refuse_unwritable(&root, &root.join(&rel))?;
        destinations.push(rel);
    }
    refuse_overlaps(&destinations)?;
    refuse_present_indexes(&parsed, &root, |_| true)?;
    let enclosing = root
        .ancestors()
        .skip(1)
        .find(|dir| dir.join(MANIFEST_NAME).is_file())
        .map(Path::to_path_buf);

    // The writes. Once the first is made, a failure exits 1: exit 2 promises an untouched tree,
    // per `design@core@init-asks-no-confirmation`. Their lines are printed once every document
    // is written, since a stdout closed early ends the process with 2, as in every command, per
    // `design@core@exit-code-ladder`: a run stopped between two documents would leave a project
    // that `init` refuses, holding a manifest, and that `check --fix` cannot complete, since it
    // writes no document. Stopped after them, it can.
    let mut log = vec![format!(
        "creating a knowledge-architect project `{name}` in {}",
        root.display()
    )];
    let written = (|| -> Result<(), String> {
        std::fs::create_dir_all(&root)
            .map_err(|e| format!("cannot create {}: {e}", root.display()))?;
        if run_git {
            crate::git::init_repository(&root).map_err(|e| e.to_string())?;
            log.push("created  a git repository".to_string());
        }
        create_new(&root.join(MANIFEST_NAME), &manifest)?
            .then_some(())
            .ok_or_else(|| {
                format!("{MANIFEST_NAME} appeared during the run: it is left as found")
            })?;
        log.push(format!("wrote    {MANIFEST_NAME}"));
        write_documents(&root, &documents, &mut log)?;
        if let Some(existing) = ignore {
            let in_repository =
                run_git || crate::git::inside_work_tree(&root).map_err(|e| e.to_string())?;
            ignore_target(&root, existing, in_repository, &mut log)?;
        }
        Ok(())
    })();
    for line in &log {
        outln!("{line}");
    }
    if let Some(outer) = enclosing {
        outln!(
            "note: the project at {} walks this one too, unless its [walk] exclude names it",
            outer.display()
        );
    }
    match written.and_then(|()| Manifest::load(&root)) {
        Ok(manifest) => finish(&manifest, checker, extensions),
        Err(e) => {
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
    let inside = match crate::git::inside_work_tree(manifest.root()) {
        Ok(inside) => inside,
        Err(e) => {
            eprintln!("error: the check cannot run: {e}");
            return Ok(ExitCode::FAILURE);
        }
    };
    if !inside {
        // The walk is git's listing, so outside a worktree no index is written and no check
        // runs, per `design@core@failed-git-is-exit-two`. The install reads no listing, so the
        // agent files are written here all the same: a session started in the project finds them.
        if manifest.serves_claude() {
            let shipped = crate::agents::shipped(manifest);
            let mut done = crate::agents::Installed::default();
            let outcome = crate::agents::install_into(manifest.root(), &shipped, &mut done);
            for rel in &done.written {
                outln!("fixed: wrote {} (installed)", rel.display());
            }
            if let Err(e) = outcome {
                eprintln!("error: {e}");
                return Ok(ExitCode::FAILURE);
            }
        }
        outln!(
            "not inside a git repository: run `git init`, then `{} check --fix`, to write the \
             indexes and run the check",
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
    /// The project-relative path of the document already there, in this shape or in the other
    /// shape of its heading register, which is left as it is.
    kept: Option<PathBuf>,
}

/// Every document the anchors `wanted` admits owe under `manifest`, of the Components and the
/// plans directory, each judged against the filesystem before anything is written.
///
/// The list is not written down here, per `design@core@init-command`: it is the required
/// documents and the register homes the check asks for, from [`Manifest::required_documents`] and [`Anchors::of`], with the plans
/// directory's README, which the check requires by its own rule. So a register declared, or a
/// document a Component comes to owe, reaches this with no edit. `Err` refuses the run: a
/// destination that is a symlink, a directory where a file is due, or under a file.
fn owed_documents(
    manifest: &Manifest,
    root: &Path,
    wanted: impl Fn(&Anchor) -> bool,
) -> Result<Vec<Document>, String> {
    let anchors = Anchors::of(manifest, &[] as &[PathBuf]);
    let mut out = Vec::new();
    for anchor in anchors.all().iter().filter(|a| wanted(a)) {
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
    for (anchor, register, home) in anchors
        .instances()
        .into_iter()
        .filter(|(a, _, _)| wanted(a))
    {
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
            refuse_unwritable(root, &root.join(&rel))?;
            let kept = std::iter::once(rel.clone())
                .chain(other_shape)
                .find(|r| root.join(r).is_file());
            Ok(Document { rel, text, kept })
        })
        .collect()
}

/// `Err` where writing `path` could follow a symlink, would meet anything but a regular file,
/// such as a directory or a FIFO a write would block on, or would need a directory where a file
/// stands, so the run refuses before its first write, as `index` refuses a symlinked destination,
/// per `design@core@index-takes-no-dry-run`.
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
        } else if at == path && !meta.is_file() {
            Some("is not a regular file")
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

/// `Err` where two destinations overlap: one is another, or lies inside another, as a Component
/// named like a file the run also writes. The filesystem cannot show it before the run writes.
fn refuse_overlaps(destinations: &[PathBuf]) -> Result<(), String> {
    for (i, a) in destinations.iter().enumerate() {
        for b in &destinations[i + 1..] {
            if a.starts_with(b) || b.starts_with(a) {
                return Err(format!(
                    "{} and {} would both be written: nothing was written",
                    a.display(),
                    b.display()
                ));
            }
        }
    }
    Ok(())
}

/// `Err` where a file stands at a generated index's path of an anchor `wanted` admits before the
/// run: the index writer replaces it, and the text there is not one this tool generated, so
/// `design@core@index-takes-no-dry-run`'s "a generated file destroys nothing" does not cover it.
fn refuse_present_indexes(
    manifest: &Manifest,
    root: &Path,
    wanted: impl Fn(&Anchor) -> bool,
) -> Result<(), String> {
    let anchors = Anchors::of(manifest, &[] as &[PathBuf]);
    for (_, register, home) in anchors
        .instances()
        .into_iter()
        .filter(|(a, _, _)| wanted(a))
    {
        if matches!(register.shape, Shape::File | Shape::Directory)
            && std::fs::symlink_metadata(root.join(&home.index)).is_ok()
        {
            return Err(format!(
                "{} exists, and the index generated there would replace it: move it aside. \
                 Nothing was written.",
                home.index.display()
            ));
        }
    }
    Ok(())
}

/// Write `text` at `path` only if nothing is there yet; `Ok(false)` where something appeared
/// since the destinations were judged, which is then left as found.
fn create_new(path: &Path, text: &str) -> Result<bool, String> {
    use std::io::Write;
    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(mut file) => file
            .write_all(text.as_bytes())
            .map(|()| true)
            .map_err(|e| format!("cannot write {}: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(e) => Err(format!("cannot write {}: {e}", path.display())),
    }
}

/// Write every document not kept, listing each; one that appeared during the run is kept too.
fn write_documents(
    root: &Path,
    documents: &[Document],
    log: &mut Vec<String>,
) -> Result<(), String> {
    for doc in documents {
        if let Some(kept) = &doc.kept {
            log.push(format!("kept     {}", kept.display()));
            continue;
        }
        let path = root.join(&doc.rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
        }
        log.push(if create_new(&path, &doc.text)? {
            format!("wrote    {}", doc.rel.display())
        } else {
            format!(
                "kept     {}, which appeared during the run",
                doc.rel.display()
            )
        });
    }
    Ok(())
}

/// The starting text of a Component's README.md or CLAUDE.md, per
/// `design@core@nothing-of-a-project-is-compiled-in`. The root's CLAUDE.md is its title and the
/// primer's import line, and nothing else, since what every root would say alike is the
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

/// The starting text of a register's home under `anchor`, per
/// `design@core@nothing-of-a-project-is-compiled-in`: a title and a line on what it holds, with no heading at a register's level, no reference and no link, so it passes every
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

/// The directory of the maintenance crate `--rust` writes, which is also a Component.
const XTASK_DIR: &str = "xtask";

/// The files `--rust` writes, each the setup skill's snippet with its placeholders filled: the
/// version this binary is, and the project's checkout variable. An existing one refuses the run,
/// per `design@core@rust-scaffold`, and so does an unwritable one.
fn rust_files(root: &Path, name: &str) -> Result<Vec<Document>, String> {
    let checkout = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let snippet = |file: &str| -> String {
        knowledge_architect_agent_skills::SNIPPETS
            .iter()
            .find(|(n, _)| *n == file)
            .map(|(_, text)| {
                text.replace("<version>", crate::CHECKER_VERSION)
                    .replace("<PROJECT>", &checkout)
            })
            .unwrap_or_else(|| panic!("the agent-skills crate ships the snippet {file}"))
    };
    [
        ("Cargo.toml", "workspace-cargo.toml"),
        ("xtask/Cargo.toml", "xtask-cargo.toml"),
        ("xtask/src/main.rs", "xtask-main.rs"),
        (".cargo/config.toml", "cargo-config.toml"),
    ]
    .into_iter()
    .map(|(rel, file)| {
        let path = root.join(rel);
        refuse_unwritable(root, &path)?;
        // Kept, the file would leave the project with a manifest declaring `cargo klarch` that an
        // older configuration may not define: the setup is the project's own, as a Cargo.toml is.
        if std::fs::symlink_metadata(&path).is_ok() {
            return Err(format!(
                "{} exists: `--rust` writes the maintenance crate's files and its cargo \
                 configuration whole, and merging into existing ones is left to the project. \
                 Nothing was written.",
                path.display()
            ));
        }
        Ok(Document {
            rel: PathBuf::from(rel),
            text: snippet(file),
            kept: None,
        })
    })
    .collect()
}

/// The `.gitignore` at `root` as it stands, `None` where there is none, read before the run's
/// first write: one `--rust` cannot append to, a symlink, a directory or text that is not UTF-8,
/// refuses the run.
fn read_gitignore(root: &Path) -> Result<Option<String>, String> {
    let path = root.join(".gitignore");
    refuse_unwritable(root, &path)?;
    match std::fs::read_to_string(&path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!(
            "cannot read {}: {e}. Nothing was written.",
            path.display()
        )),
    }
}

/// Make `.gitignore` ignore `/target`, where cargo builds: an existing file keeps every line and
/// gains the rule under a line saying who added it, as `cargo init` appends its own; a file whose
/// rules already ignore it is left as it is.
fn ignore_target(
    root: &Path,
    existing: Option<String>,
    in_repository: bool,
    log: &mut Vec<String>,
) -> Result<(), String> {
    let path = root.join(".gitignore");
    // Git decides what it ignores, a negation or a leading space included, so it is asked where
    // there is a repository; with none, one of the four usual spellings on a line of its own is
    // taken for the rule.
    let ignored = |text: &str| -> Result<bool, String> {
        if in_repository {
            crate::git::ignores(root, "target/").map_err(|e| e.to_string())
        } else {
            Ok(text
                .lines()
                .any(|l| matches!(l.trim_end(), "/target" | "/target/" | "target" | "target/")))
        }
    };
    let text = match existing {
        Some(text) if ignored(&text)? => {
            log.push("kept     .gitignore".to_string());
            return Ok(());
        }
        Some(text) => {
            let sep = if text.is_empty() || text.ends_with('\n') {
                ""
            } else {
                "\n"
            };
            format!("{text}{sep}\n# Added by klarch\n/target\n")
        }
        None => "/target\n".to_string(),
    };
    std::fs::write(&path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    log.push("wrote    .gitignore".to_string());
    Ok(())
}

/// Add a Component to the project `manifest` describes, as `klarch new-component` does.
///
/// It runs inside a located project and behind the writer gate, so over a model phases 1 to 3
/// find incomplete it refuses, per `design@core@phases-gate-the-report`. Every refusal comes
/// before its first write and is `Err`, exit 2. It edits the manifest with `toml_edit`, which
/// keeps every byte it does not change, comments included, per
/// `design@core@component-manifest-edit`.
pub(super) fn new_component(
    manifest: &Manifest,
    args: &NewComponentArgs,
    checker: &[&Path],
    extensions: &mut [Box<dyn Extension>],
) -> Result<ExitCode, String> {
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    super::complete_working_tree(manifest, &model)?;
    let cwd =
        std::env::current_dir().map_err(|e| format!("cannot read the working directory: {e}"))?;
    let root = manifest.root();
    let rel = component_path(root, &lexical(&cwd.join(&args.dir)))?;

    let path = root.join(MANIFEST_NAME);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let edited = with_component(&text, &rel)?;
    let parsed = Manifest::parse(root, &edited)
        .map_err(|e| format!("{MANIFEST_NAME} with `{rel}` added does not parse: {e}"))?;
    if let Some(complaint) = parsed.complaints().first() {
        return Err(format!(
            "{MANIFEST_NAME} with `{rel}` added is refused: {}. Nothing was written.",
            complaint.what
        ));
    }
    let wanted = |a: &Anchor| a.is_component && a.path == Path::new(&rel);
    let documents = owed_documents(&parsed, root, wanted)?;
    refuse_present_indexes(&parsed, root, wanted)?;

    std::fs::write(&path, &edited).map_err(|e| format!("cannot write {MANIFEST_NAME}: {e}"))?;
    // Printed once the documents are written, as `init` prints its own.
    let mut log = vec![format!(
        "wrote    {MANIFEST_NAME}, its components gaining `{rel}`"
    )];
    let written = write_documents(root, &documents, &mut log);
    for line in &log {
        outln!("{line}");
    }
    if let Err(e) = written {
        eprintln!("error: {e}");
        return Ok(ExitCode::FAILURE);
    }
    finish(&parsed, checker, extensions)
}

/// `text`, a manifest, with `rel` appended to `[project] components` and nothing else changed.
///
/// `toml_edit` writes the whole document back with `\n` line breaks and no byte-order mark, so
/// only the array's own text is taken from it, spliced over the array's span in `text`: every
/// byte outside the array is kept, line endings and a mark included. Inside the array, `\r\n` is
/// written where the array's text held one.
fn with_component(text: &str, rel: &str) -> Result<String, String> {
    let does_not_parse = |e: toml_edit::TomlError| format!("{MANIFEST_NAME} does not parse: {e}");
    let no_array = || format!("{MANIFEST_NAME} holds no `[project] components` array");
    let span = toml_edit::Document::parse(text)
        .map_err(does_not_parse)?
        .get("project")
        .and_then(|p| p.get("components"))
        .filter(|c| c.is_array())
        .and_then(toml_edit::Item::span)
        .ok_or_else(no_array)?;
    let mut doc: toml_edit::DocumentMut = text.parse().map_err(does_not_parse)?;
    let components = doc
        .get_mut("project")
        .and_then(|p| p.get_mut("components"))
        .and_then(|c| c.as_array_mut())
        .ok_or_else(no_array)?;
    append(components, rel);
    let mut array = toml_edit::Value::Array(components.clone());
    array.decor_mut().clear();
    let mut array = array.to_string();
    if text[span.clone()].contains("\r\n") {
        array = array.replace('\n', "\r\n");
    }
    Ok(format!(
        "{}{array}{}",
        &text[..span.start],
        &text[span.end..]
    ))
}

/// Push `rel` onto `components`, keeping the array's layout.
fn append(components: &mut toml_edit::Array, rel: &str) {
    // In an array written one element per line, the new element takes a line of its own at the
    // last element's indentation. What follows the last element, up to the last line break before
    // `]`, is its reason comment, or comment lines before `]`: it moves ahead of the new element,
    // after the comma, so every comment line stays byte for byte on its own line and a reason never
    // reads as the new element's. `push` alone writes `"a", "svc", # reason for a`. An array on one
    // line, or an empty one, holds no element line, and `push` keeps it as it is.
    let indent = components
        .iter()
        .last()
        .and_then(|last| last.decor().prefix().and_then(|p| p.as_str()))
        .and_then(|prefix| prefix.rfind('\n').map(|at| prefix[at + 1..].to_string()));
    let Some(indent) = indent else {
        components.push(rel);
        return;
    };
    let last = components.len() - 1;
    let decor = components
        .get_mut(last)
        .expect("the array holds its last element")
        .decor_mut();
    let suffix = decor
        .suffix()
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();
    decor.set_suffix("");
    let tail = format!("{suffix}{}", components.trailing().as_str().unwrap_or(""));
    let (comments, closing) = match tail.rfind('\n') {
        Some(at) => (&tail[..=at], &tail[at + 1..]),
        None => ("\n", tail.as_str()),
    };
    let mut value = toml_edit::Value::from(rel);
    value.decor_mut().set_prefix(format!("{comments}{indent}"));
    components.push_formatted(value);
    components.set_trailing(if tail.contains('\n') {
        format!("\n{closing}")
    } else {
        closing.to_string()
    });
}

/// `path` relative to `root`, with `/` separators, as a manifest lists a Component; `Err` where it
/// is the root itself, lies outside it, or lies where no Component can live, per
/// `design@core@component-command`: in a repository's
/// `.git` directory, which git keeps out of every walk, or in the installer's namespace, whose
/// files the install deletes.
fn component_path(root: &Path, path: &Path) -> Result<String, String> {
    let rel = project_relative(root, path)?;
    if rel.split('/').any(|part| part == ".git") {
        return Err(format!(
            "`{rel}` lies in a git directory, which no walk reads"
        ));
    }
    // The namespace is judged over files, so the README the Component would hold is asked.
    if crate::manifest::owned_path(&Path::new(&rel).join("README.md")) {
        return Err(format!(
            "`{rel}` lies in the installer's namespace, whose files every install deletes"
        ));
    }
    Ok(rel)
}

/// `path` relative to `root`, with `/` separators; `Err` where it is the root itself or lies
/// outside it.
fn project_relative(root: &Path, path: &Path) -> Result<String, String> {
    let rel = path.strip_prefix(root).map_err(|_| {
        format!(
            "{} is outside the project at {}",
            path.display(),
            root.display()
        )
    })?;
    if rel.as_os_str().is_empty() {
        return Err(format!(
            "{} is the project's root, which is a Component already",
            path.display()
        ));
    }
    let parts: Vec<&str> = rel
        .components()
        .map(|c| {
            c.as_os_str()
                .to_str()
                .ok_or("a Component's path is not UTF-8")
        })
        .collect::<Result<_, _>>()?;
    Ok(parts.join("/"))
}

/// Whether `init` runs `git init` at `root`, or `Err` where its rule refuses.
///
/// Inside a worktree no repository is created unless `--vcs git` asks: a project in a
/// subdirectory of a repository is legal. Outside every worktree a new or empty directory gets
/// one, as `cargo init` gives one. A directory there that already holds files is refused unless
/// `--vcs` decides, since an accidental run would leave its files untracked beside the new ones,
/// where a `git clean` deletes both. The rule and its argument are `design@core@init-runs-git-init`.
fn git_rule(root: &Path, vcs: Option<Vcs>) -> Result<bool, String> {
    let existing = root
        .ancestors()
        .find(|dir| dir.is_dir())
        .ok_or_else(|| format!("no directory at or above {} exists", root.display()))?;
    // Asked whatever `--vcs` says: the check the run ends with reads the tree through git, so a
    // git that cannot answer refuses the run before its first write.
    let inside = crate::git::inside_work_tree(existing).map_err(|e| e.to_string())?;
    match vcs {
        // A repository's root already holds one, and `git init` there would only re-initialise it.
        Some(Vcs::Git) => return Ok(!root.join(".git").exists()),
        Some(Vcs::None) => return Ok(false),
        None => {}
    }
    if inside {
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

/// `Err` where PATH or a Component's directory lies in a worktree and git ignores it: the walk
/// is git's listing, so the check would report every file written there missing, with no repair
/// but the ignore rule. A repository `init` creates holds no rule yet, so the caller asks only
/// where it creates none.
fn refuse_ignored(root: &Path, components: &[String]) -> Result<(), String> {
    let existing = root
        .ancestors()
        .find(|dir| dir.is_dir())
        .ok_or_else(|| format!("no directory at or above {} exists", root.display()))?;
    if !crate::git::inside_work_tree(existing).map_err(|e| e.to_string())? {
        return Ok(());
    }
    let from_existing = |dir: &Path| -> String {
        let rel = dir.strip_prefix(existing).unwrap_or(dir);
        if rel.as_os_str().is_empty() {
            ".".to_string()
        } else {
            format!("{}/", rel.display())
        }
    };
    let dirs = std::iter::once(root.to_path_buf()).chain(components.iter().map(|c| root.join(c)));
    for dir in dirs {
        if crate::git::ignores(existing, &from_existing(&dir)).map_err(|e| e.to_string())? {
            return Err(format!(
                "git ignores {}, so the check would read none of the files written there: \
                 remove the ignore rule, or choose another place. Nothing was written.",
                dir.display()
            ));
        }
    }
    Ok(())
}

/// The manifest `init` writes: the setup skill's smallest one, named, pinned, and listing the
/// Components it creates, each a project-relative path with `/` separators.
fn manifest_text(name: &str, command: Option<&str>, components: &[String]) -> String {
    let command = command
        .map(|c| format!("command = {}\n", toml_string(c)))
        .unwrap_or_default();
    let components = components
        .iter()
        .map(|c| toml_string(c))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "[project]\n\
         name = {}\n\
         checker-version = {}\n\
         {command}\
         components = [{components}]\n\
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
            &manifest_text("probe", None, &[]).replace(crate::CHECKER_VERSION, "fixture"),
        )
        .expect("the written manifest parses");
        let args = InitArgs {
            path: PathBuf::from("."),
            name: None,
            command: None,
            vcs: None,
            components: Vec::new(),
            rust: false,
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
