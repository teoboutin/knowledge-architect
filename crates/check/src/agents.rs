//! The text the checker installs into a project's agent configuration, rendered for that project.
//!
//! The text itself is the crate `knowledge-architect-agent-skills`. This module renders it, says what
//! the installed set must be, and writes it; the check of it is `crate::check::agents`. The decisions are
//! `design@check@agents-table`, `design@check@owned-namespace-check` and
//! `design@check@declared-command`.

use std::path::{Path, PathBuf};

use crate::model::declared::owned_path;
use crate::model::listing::Outside;
use crate::model::Manifest;

/// What the shipped text writes where the project's declared command goes.
const PLACEHOLDER: &str = "{{command}}";

/// The install path of the primer, which the project's root CLAUDE.md imports.
pub(crate) const PRIMER: &str = ".claude/knowledge-architect/PRIMER.md";

/// The line that imports the primer, with no line ending: the root CLAUDE.md's first line, per
/// `design@check@import-on-first-line`, followed by `\n` or `\r\n` unless it is the whole file.
pub const IMPORT_LINE: &str = "@.claude/knowledge-architect/PRIMER.md";

/// A shipped template with the project's command filled in, with LF line endings.
pub(crate) fn render(template: &str, command: &str) -> String {
    lf(&template.replace(PLACEHOLDER, command))
}

/// The text with every CRLF line ending made LF, so a checkout that converts line endings holds
/// the same installed text as one that does not.
pub(crate) fn lf(text: &str) -> String {
    text.replace("\r\n", "\n")
}

/// Every file this version installs into the project, at its install path, rendered. Empty when
/// the project serves no agent harness.
pub fn shipped(manifest: &Manifest) -> Vec<(PathBuf, String)> {
    shipped_from(manifest, knowledge_architect_agent_skills::FILES)
}

/// The same, over a stated set of templates: what a test hands in.
pub(crate) fn shipped_from(
    manifest: &Manifest,
    templates: &[(&str, &str)],
) -> Vec<(PathBuf, String)> {
    if !manifest.serves_claude() {
        return Vec::new();
    }
    templates
        .iter()
        .map(|(path, text)| (PathBuf::from(path), render(text, manifest.command())))
        .collect()
}

/// Whether a root CLAUDE.md's first line is exactly the primer's import line, per
/// `design@check@import-on-first-line` and `design@check@import-line-exact-bytes`.
///
/// A byte-order mark before the line and the line ending after it, `\n` or `\r\n`, are not part
/// of it; nothing else is stripped. The import line anywhere else, or with a space before or
/// after it, does not count.
pub(crate) fn imports_primer(text: &str) -> bool {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    text.lines().next() == Some(IMPORT_LINE)
}

/// What an install did, each list in path order.
#[derive(Debug, Default)]
pub struct Installed {
    /// The shipped files whose bytes the install wrote, because they were missing or differed.
    pub written: Vec<PathBuf>,
    /// The files of the owned namespace that the shipped set does not hold, removed.
    pub deleted: Vec<PathBuf>,
}

/// Write every shipped file whose bytes differ from what is on disk, and delete every file of the
/// owned namespace that the shipped set does not hold. It edits nothing outside the namespace: in
/// particular not the root CLAUDE.md, which belongs to the project.
///
/// **A symbolic link on an owned path is refused before anything is touched**: following one
/// would write or delete in another directory, possibly another project's. The error names the
/// path, and so does every filesystem error.
pub(crate) fn install(root: &Path, shipped: &[(PathBuf, String)]) -> Result<Installed, String> {
    let mut out = Installed::default();
    install_into(root, shipped, &mut out)?;
    Ok(out)
}

/// Write `shipped` under `root` and delete every file of the installer's namespace it does not hold,
/// recording into `out` as it goes, so a caller still knows what was written or deleted when a
/// later write fails part-way. `init` prints that record outside a worktree, either way.
///
/// `Err` is a filesystem error, naming the path, or a symbolic link on an owned path, which this
/// refuses rather than writes or deletes through: following one would reach another directory, possibly
/// another project's. A link met after the first write leaves the earlier writes in place, per
/// `issue@check@install-agent-skills-exits-two-after-writing`.
pub fn install_into(
    root: &Path,
    shipped: &[(PathBuf, String)],
    out: &mut Installed,
) -> Result<(), String> {
    for rel in [
        ".claude",
        ".claude/skills",
        ".claude/agents",
        ".claude/workflows",
        ".claude/knowledge-architect",
    ] {
        refuse_link(root, Path::new(rel))?;
    }
    for (rel, _) in shipped {
        for ancestor in rel.ancestors() {
            refuse_link(root, ancestor)?;
        }
    }
    let at = |rel: &Path, e: std::io::Error| format!("{}: {e}", rel.display());
    for (rel, text) in shipped {
        let path = root.join(rel);
        if std::fs::read(&path).ok().as_deref() == Some(text.as_bytes()) {
            continue;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| at(rel, e))?;
        }
        std::fs::write(&path, text).map_err(|e| at(rel, e))?;
        out.written.push(rel.clone());
    }
    for rel in owned_on_disk(root)? {
        if !shipped.iter().any(|(s, _)| *s == rel) {
            std::fs::remove_file(root.join(&rel)).map_err(|e| at(&rel, e))?;
            out.deleted.push(rel);
        }
    }
    remove_empty_owned_dirs(root).map_err(|e| at(Path::new(".claude/skills"), e))?;
    out.written.sort();
    out.deleted.sort();
    Ok(())
}

/// What `check --fix` repairs in the installed set: exactly what the installed-file check
/// reports and the install would repair, and nothing else.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Repairs {
    /// Shipped files missing from git's listing, or listed and differing from the shipped text
    /// once line endings are normalised.
    pub write: Vec<PathBuf>,
    /// Files of the namespace that git lists, the working tree holds, and this version does not
    /// ship.
    pub delete: Vec<PathBuf>,
}

impl Repairs {
    pub(crate) fn is_empty(&self) -> bool {
        self.write.is_empty() && self.delete.is_empty()
    }
}

/// The repairs of the installed set, judged from git's listing of the namespace and compared
/// after normalising line endings, exactly as the installed-file check judges it, per
/// `design@check@fix-scope`.
///
/// **A file git does not list is never deleted.** The install walks the filesystem, so it would
/// remove an ignored file in the namespace, such as an editor's swap file, which the check never
/// reports and git holds no copy of. **A file differing only by its line endings is never
/// rewritten**: the check accepts it. So a `--fix` run touches only what the check reported.
pub(crate) fn repairs(
    root: &Path,
    shipped: &[(PathBuf, String)],
    installed: &[(PathBuf, Outside)],
) -> Repairs {
    let mut out = Repairs::default();
    for (rel, expected) in shipped {
        match installed
            .iter()
            .find(|(p, _)| p == rel)
            .map(|(_, state)| state)
        {
            Some(Outside::Text(text)) if lf(text) == lf(expected) => {}
            // A directory in the file's place is reported, and replacing it is no safe fix.
            Some(Outside::Directory) => {}
            // Not listed: missing, or an ignore rule covers it. An ignored copy that already holds
            // the shipped text gains nothing from a write; the check keeps reporting it.
            None => {
                let current = std::fs::read_to_string(root.join(rel))
                    .is_ok_and(|on_disk| lf(&on_disk) == lf(expected));
                if !current {
                    out.write.push(rel.clone());
                }
            }
            Some(_) => out.write.push(rel.clone()),
        }
    }
    for (rel, state) in installed {
        if shipped.iter().any(|(s, _)| s == rel) {
            continue;
        }
        if matches!(state, Outside::Text(_) | Outside::Binary) {
            out.delete.push(rel.clone());
        }
    }
    out
}

/// Apply `repairs`, recording into `out` as it goes, so a caller still knows what was written or
/// deleted when a later step fails part-way. A symbolic link on an owned path is refused before
/// anything is touched, as `install` refuses one.
pub(crate) fn apply(
    root: &Path,
    shipped: &[(PathBuf, String)],
    repairs: &Repairs,
    out: &mut Installed,
) -> Result<(), String> {
    for rel in [
        ".claude",
        ".claude/skills",
        ".claude/agents",
        ".claude/workflows",
        ".claude/knowledge-architect",
    ] {
        refuse_link(root, Path::new(rel))?;
    }
    for rel in repairs.write.iter().chain(&repairs.delete) {
        for ancestor in rel.ancestors() {
            refuse_link(root, ancestor)?;
        }
    }
    let at = |rel: &Path, e: std::io::Error| format!("{}: {e}", rel.display());
    for rel in &repairs.write {
        let Some((_, text)) = shipped.iter().find(|(s, _)| s == rel) else {
            continue;
        };
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| at(rel, e))?;
        }
        std::fs::write(&path, text).map_err(|e| at(rel, e))?;
        out.written.push(rel.clone());
    }
    for rel in &repairs.delete {
        std::fs::remove_file(root.join(rel)).map_err(|e| at(rel, e))?;
        out.deleted.push(rel.clone());
    }
    remove_empty_owned_dirs(root).map_err(|e| at(Path::new(".claude/skills"), e))?;
    Ok(())
}

/// An error naming the path when it is a symbolic link.
fn refuse_link(root: &Path, rel: &Path) -> Result<(), String> {
    if rel.as_os_str().is_empty() {
        return Ok(());
    }
    match std::fs::symlink_metadata(root.join(rel)) {
        Ok(m) if m.file_type().is_symlink() => Err(format!(
            "{} is a symbolic link: the install writes and deletes only inside the project, so \
             it refuses to follow one",
            rel.display()
        )),
        _ => Ok(()),
    }
}

/// Every file of the owned namespace on disk, tracked or not, project-relative.
fn owned_on_disk(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    for base in [
        ".claude/knowledge-architect",
        ".claude/skills",
        ".claude/agents",
        ".claude/workflows",
    ] {
        files_under(root, Path::new(base), &mut out)?;
    }
    out.retain(|rel| owned_path(rel));
    out.sort();
    Ok(out)
}

/// Every file under a directory, without following a symbolic link: an owned link is refused.
fn files_under(root: &Path, rel: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let at = |e: std::io::Error| format!("{}: {e}", rel.display());
    let entries = match std::fs::read_dir(root.join(rel)) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(at(e)),
    };
    for entry in entries {
        let entry = entry.map_err(at)?;
        let child = rel.join(entry.file_name());
        let kind = entry.file_type().map_err(at)?;
        if kind.is_symlink() {
            if owned_path(&child) || owned_path(&child.join("x")) {
                refuse_link(root, &child)?;
            }
        } else if kind.is_dir() {
            files_under(root, &child, out)?;
        } else {
            out.push(child);
        }
    }
    Ok(())
}

/// Remove the owned directories a deletion left empty, so a removed skill leaves no directory.
fn remove_empty_owned_dirs(root: &Path) -> std::io::Result<()> {
    let skills = root.join(".claude/skills");
    if let Ok(entries) = std::fs::read_dir(&skills) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let owned = name
                .to_str()
                .is_some_and(|n| n.starts_with(crate::model::declared::OWNED_PREFIX));
            if owned && entry.file_type()?.is_dir() && is_empty_tree(&entry.path())? {
                std::fs::remove_dir_all(entry.path())?;
            }
        }
    }
    Ok(())
}

fn is_empty_tree(dir: &Path) -> std::io::Result<bool> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() || !is_empty_tree(&entry.path())? {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim: every file this version ships is installed inside the owned namespace, so the
    /// install edits nothing a project owns. The install paths come from the build script of
    /// `knowledge-architect-agent-skills`. Mutation: a prefix other than the owned one in that
    /// script fails it.
    #[test]
    fn every_shipped_file_is_installed_in_the_owned_namespace() {
        for (path, _) in knowledge_architect_agent_skills::FILES {
            assert!(
                owned_path(Path::new(path)),
                "{path} is outside the namespace"
            );
        }
    }

    /// The claim: the repairs are what the installed-file check reports and nothing else: a
    /// missing or differing shipped file, and an unshipped file git lists. A copy differing only by
    /// line endings, and a file git does not list, such as an ignored swap file, are left alone.
    /// Mutations checked: comparing raw bytes puts the CRLF copy in `write`; deleting from the
    /// filesystem rather than from the listing puts the swap file in `delete`.
    #[test]
    fn the_repairs_are_what_the_check_reports_and_nothing_else() {
        let root = std::env::temp_dir().join(format!("ka-repairs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".claude/agents")).unwrap();
        std::fs::write(root.join(".claude/agents/.swap.swp"), "unsaved").unwrap();
        let shipped = vec![
            (
                PathBuf::from(".claude/agents/knowledge-architect-a.md"),
                "one\ntwo\n".to_string(),
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-b.md"),
                "b\n".to_string(),
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-c.md"),
                "c\n".to_string(),
            ),
        ];
        let installed = vec![
            (
                PathBuf::from(".claude/agents/knowledge-architect-a.md"),
                Outside::Text("one\r\ntwo\r\n".to_string()),
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-b.md"),
                Outside::Text("edited\n".to_string()),
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-old.md"),
                Outside::Text("unshipped".to_string()),
            ),
        ];
        let found = repairs(&root, &shipped, &installed);
        assert_eq!(
            found,
            Repairs {
                write: vec![
                    PathBuf::from(".claude/agents/knowledge-architect-b.md"),
                    PathBuf::from(".claude/agents/knowledge-architect-c.md"),
                ],
                delete: vec![PathBuf::from(".claude/agents/knowledge-architect-old.md")],
            }
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The claim, per `design@check@fix-scope`: a shipped copy git lists as deleted or as binary
    /// is written again; a shipped copy git does not list is written only where the disk does not already hold
    /// the shipped text; a directory in a shipped file's place is left alone; an unshipped binary
    /// file git lists is deleted, and an unshipped path the working tree does not hold as a file is
    /// not. Mutations checked: `Outside::Binary` dropped from the deletion's match keeps the binary
    /// stray; `if !current` made `if true` rewrites the current ignored copy; the `Directory` arm
    /// made a write writes over the directory; the `Missing` state left unwritten keeps the deleted
    /// copy missing; a `Binary` arm that writes nothing keeps the binary copy.
    #[test]
    fn the_repairs_restore_a_deleted_or_binary_copy_and_leave_a_current_one_and_a_directory() {
        let root = std::env::temp_dir().join(format!("ka-repairs-states-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".claude/agents")).unwrap();
        let ignored_current = ".claude/agents/knowledge-architect-ignored-current.md";
        let ignored_stale = ".claude/agents/knowledge-architect-ignored-stale.md";
        std::fs::write(root.join(ignored_current), "current\r\n").unwrap();
        std::fs::write(root.join(ignored_stale), "stale\n").unwrap();
        let shipped: Vec<(PathBuf, String)> = [
            (".claude/agents/knowledge-architect-binary.md", "text\n"),
            (".claude/agents/knowledge-architect-deleted.md", "deleted\n"),
            (".claude/agents/knowledge-architect-directory.md", "dir\n"),
            (ignored_current, "current\n"),
            (ignored_stale, "fresh\n"),
        ]
        .iter()
        .map(|(p, t)| (PathBuf::from(p), t.to_string()))
        .collect();
        let installed = vec![
            (
                PathBuf::from(".claude/agents/knowledge-architect-binary.md"),
                Outside::Binary,
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-deleted.md"),
                Outside::Missing,
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-directory.md"),
                Outside::Directory,
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-blob.bin"),
                Outside::Binary,
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-gone.md"),
                Outside::Missing,
            ),
            (
                PathBuf::from(".claude/agents/knowledge-architect-nested"),
                Outside::Directory,
            ),
        ];
        let found = repairs(&root, &shipped, &installed);
        assert_eq!(
            found,
            Repairs {
                write: vec![
                    PathBuf::from(".claude/agents/knowledge-architect-binary.md"),
                    PathBuf::from(".claude/agents/knowledge-architect-deleted.md"),
                    PathBuf::from(ignored_stale),
                ],
                delete: vec![PathBuf::from(".claude/agents/knowledge-architect-blob.bin")],
            }
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The claim: an install writes a shipped workflow and removes a prefixed one the version does
    /// not ship from .claude/workflows/, and leaves the project's own workflows there. Mutation: the
    /// workflows directory left out of `owned_on_disk` keeps the old workflow.
    #[test]
    fn an_install_owns_the_prefixed_workflows_alone() {
        let root = std::env::temp_dir().join(format!("ka-install-wf-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let write = |rel: &str, text: &str| {
            let p = root.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        };
        write(".claude/workflows/knowledge-architect-old.js", "old");
        write(".claude/workflows/project-own.js", "kept");
        let shipped = vec![(
            PathBuf::from(".claude/workflows/knowledge-architect-w.js"),
            "fresh".to_string(),
        )];
        let done = install(&root, &shipped).expect("the install runs");
        assert_eq!(
            done.written,
            vec![PathBuf::from(".claude/workflows/knowledge-architect-w.js")]
        );
        assert_eq!(
            done.deleted,
            vec![PathBuf::from(
                ".claude/workflows/knowledge-architect-old.js"
            )]
        );
        assert!(root.join(".claude/workflows/project-own.js").exists());
        std::fs::remove_dir_all(&root).unwrap();
    }

    /// The claim: an install writes what is missing or differs, removes an unshipped file of the
    /// namespace with its emptied skill directory, and touches nothing outside the namespace.
    #[test]
    fn an_install_writes_the_shipped_set_and_removes_the_rest_of_the_namespace() {
        let root = std::env::temp_dir().join(format!("ka-install-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let write = |rel: &str, text: &str| {
            let p = root.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        };
        write(".claude/skills/knowledge-architect-old/SKILL.md", "old");
        write(".claude/skills/project-own/SKILL.md", "kept");
        write(".claude/agents/knowledge-architect-a.md", "stale");
        write("CLAUDE.md", "# P\n");
        let shipped = vec![
            (
                PathBuf::from(".claude/agents/knowledge-architect-a.md"),
                "fresh".to_string(),
            ),
            (PathBuf::from(PRIMER), "primer".to_string()),
        ];
        let done = install(&root, &shipped).expect("the install runs");
        assert_eq!(
            done.written,
            vec![
                PathBuf::from(".claude/agents/knowledge-architect-a.md"),
                PathBuf::from(PRIMER)
            ]
        );
        assert_eq!(
            done.deleted,
            vec![PathBuf::from(
                ".claude/skills/knowledge-architect-old/SKILL.md"
            )]
        );
        assert!(!root.join(".claude/skills/knowledge-architect-old").exists());
        assert!(root.join(".claude/skills/project-own/SKILL.md").exists());
        assert_eq!(
            std::fs::read_to_string(root.join("CLAUDE.md")).unwrap(),
            "# P\n"
        );
        let again = install(&root, &shipped).expect("a second install runs");
        assert!(again.written.is_empty() && again.deleted.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The claim: an install refuses a symbolic link on an owned path before touching anything,
    /// and names it. Mutation: following the link deletes the other directory's file.
    #[cfg(unix)]
    #[test]
    fn an_install_refuses_a_symlinked_namespace() {
        let base = std::env::temp_dir().join(format!("ka-link-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let shared = base.join("shared/knowledge-architect-other");
        std::fs::create_dir_all(&shared).unwrap();
        std::fs::write(shared.join("SKILL.md"), "keep").unwrap();
        let root = base.join("proj");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::os::unix::fs::symlink(base.join("shared"), root.join(".claude/skills")).unwrap();
        let e = install(&root, &[]).expect_err("a link is refused");
        assert!(e.contains(".claude/skills is a symbolic link"), "{e}");
        assert!(shared.join("SKILL.md").exists());
        let _ = std::fs::remove_dir_all(&base);
    }

    /// The claim: an unshipped file under the installer's own directory is removed, and a
    /// project's own skill directory, empty or not, is never touched. Mutations: dropping the
    /// installer's directory from the scan, or removing empty skill directories without the
    /// prefix test, each fail a case.
    #[test]
    fn the_install_scans_its_own_directory_and_spares_the_projects() {
        let root = std::env::temp_dir().join(format!("ka-scan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join(".claude/knowledge-architect")).unwrap();
        std::fs::write(root.join(".claude/knowledge-architect/OLD.md"), "old").unwrap();
        std::fs::create_dir_all(root.join(".claude/skills/project-empty")).unwrap();
        let done = install(&root, &[]).expect("the install runs");
        assert_eq!(
            done.deleted,
            vec![PathBuf::from(".claude/knowledge-architect/OLD.md")]
        );
        assert!(root.join(".claude/skills/project-empty").is_dir());
        let _ = std::fs::remove_dir_all(&root);
    }
}

#[cfg(test)]
mod shipped_text_tests {
    use super::*;

    /// The setup skill tells a project to write the import line this module checks for. The
    /// agent-skills build script holds a copy of the line, which it cannot read from here, since
    /// this crate depends on that one; this test holds the two equal.
    #[test]
    fn the_setup_skill_ships_the_import_line_the_check_looks_for() {
        let (_, setup) = knowledge_architect_agent_skills::FILES
            .iter()
            .find(|(path, _)| *path == ".claude/skills/knowledge-architect-setup/SKILL.md")
            .expect("the setup skill is shipped");
        assert!(
            setup.contains(&format!("`{IMPORT_LINE}`")),
            "the setup skill does not show {IMPORT_LINE}"
        );
    }

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The claim: the placeholder takes the declared command, and the default command when
    /// none is declared. Mutation: rendering with a fixed command fails the declared case.
    #[test]
    fn the_shipped_text_is_rendered_with_the_declared_command() {
        let templates = [(
            ".claude/agents/knowledge-architect-a.md",
            "run `{{command}} check`",
        )];
        let declared = manifest("command = \"cargo klarch\"\n");
        assert_eq!(
            shipped_from(&declared, &templates)[0].1,
            "run `cargo klarch check`"
        );
        assert_eq!(
            shipped_from(&manifest(""), &templates)[0].1,
            "run `klarch check`"
        );
    }

    /// The claim: a project that serves no harness is shipped nothing.
    #[test]
    fn no_harness_ships_nothing() {
        let templates = [(".claude/agents/knowledge-architect-a.md", "text")];
        let none = manifest("\n[agents]\nharness = []\n");
        assert!(shipped_from(&none, &templates).is_empty());
    }

    /// The claim: the import line counts as the file's first line, whatever follows it, with a
    /// byte-order mark before it and either line ending after it. Mutation: not stripping the
    /// mark fails the third case.
    #[test]
    fn the_import_line_counts_as_the_first_line() {
        for text in [
            "@.claude/knowledge-architect/PRIMER.md\n\n# P\n\nThe project's own rules.\n",
            "@.claude/knowledge-architect/PRIMER.md",
            "\u{feff}@.claude/knowledge-architect/PRIMER.md\r\n# P\r\n",
        ] {
            assert!(imports_primer(text), "{text:?}");
        }
    }

    /// The claim: the import line counts nowhere but as the first line, and only exactly. The
    /// first three cases are shapes the harness evaluates no import in, which a scanner of fences
    /// and comments let pass, per `rejected@check@import-in-prose-by-parser`. Mutations: comparing
    /// any line fails the cases whose import line is lower; trimming the first line fails the
    /// cases with spaces; stripping every byte-order mark fails the case with two; stripping a
    /// carriage return fails the case whose line ends with one alone.
    #[test]
    fn the_import_line_counts_nowhere_else_and_only_exactly() {
        for text in [
            "````\n```\n@.claude/knowledge-architect/PRIMER.md\n````\n",
            "```\n```md\n@.claude/knowledge-architect/PRIMER.md\n```\n",
            "x <!-- y\n@.claude/knowledge-architect/PRIMER.md\n-->\n",
            "# P\n\n@.claude/knowledge-architect/PRIMER.md\n",
            "\n@.claude/knowledge-architect/PRIMER.md\n",
            "    @.claude/knowledge-architect/PRIMER.md\n",
            " @.claude/knowledge-architect/PRIMER.md\n",
            "@.claude/knowledge-architect/PRIMER.md \n",
            "@.claude/knowledge-architect/PRIMER.md.bak\n",
            "see @.claude/knowledge-architect/PRIMER.md\n",
            "\u{feff}\u{feff}@.claude/knowledge-architect/PRIMER.md\n",
            "@.claude/knowledge-architect/PRIMER.md\r",
            "",
        ] {
            assert!(!imports_primer(text), "{text:?}");
        }
    }
}
