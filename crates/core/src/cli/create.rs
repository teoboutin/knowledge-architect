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
use crate::extension::Extension;
use crate::manifest::{MANIFEST_NAME, OWNED_PREFIX};
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
    _checker: &[&Path],
    _extensions: &mut [Box<dyn Extension>],
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
    Ok(ExitCode::SUCCESS)
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
