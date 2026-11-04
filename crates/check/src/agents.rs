//! The text the checker installs into a project's agent configuration, rendered for that project.
//!
//! The text itself is the crate `knowledge-architect-agent-skills`, and `model::installed` renders
//! it and says what the installed set must be. This module writes it. The decisions are
//! `design@check@agents-table`, `design@check@owned-namespace-check` and
//! `design@check@declared-command`.

use std::path::{Path, PathBuf};

use crate::model::declared::owned_path;
use crate::model::installed::lf;
use crate::model::listing::Outside;

/// What an install did, each list in path order.
#[derive(Debug, Default)]
pub(crate) struct Installed {
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

/// The same, recording into `out` as it goes, so a caller still knows what was written or
/// deleted when a later write fails part-way. `check --fix` prints that record either way, and
/// its exit code depends on whether anything was written.
pub(crate) fn install_into(
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
    use crate::model::installed::PRIMER;

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
