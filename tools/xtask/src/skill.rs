//! The `skill` subcommand: print the source of one shipped text whole, its fragments and snippets
//! inlined and its `%%` comments kept.
//!
//! The installed copy under .claude is what a session in an adopting project reads, and it holds
//! no `%%` comment of this repository: the build removes them, per
//! `design@agent-skills@shipped-text-line-comments`. The source under the agent-skills crate's
//! content/ holds the comments and names each fragment by a placeholder line. This view joins the
//! two, so a maintainer reads a skill whole, with the reasons and the citations its comments give,
//! and still finds each line in the file to edit. It prints the files of the checkout, not the text
//! a binary embeds. The command builds the agent-skills crate, whose build refuses a source it would
//! not ship, so a source the build refuses has no view: the build's message names its line.
//!
//! It is a subcommand of this tool rather than of the checker, per
//! `design@xtask@source-view-in-the-tool`.

use knowledge_architect::MANIFEST_NAME;
use knowledge_architect_gates::process::{complain, say};
use knowledge_architect_gates::project_root;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// The build's own file, so the view reads placeholders through the readers the build's passes
// use, per `design@xtask@passes-included-not-exported`. The source view and the placeholder
// constants serve here; the rest of the file is the build's.
#[path = "../../../crates/agent-skills/src/render.rs"]
#[allow(dead_code)]
mod render;

/// The agent-skills crate, from the project root.
const CRATE: &str = "crates/agent-skills";

/// The installer's prefix, which a name given with it drops.
const PREFIX: &str = "knowledge-architect-";

/// The file of content/ that `name` names, relative to content/, or why none or several do.
///
/// `name` is a skill, an agent or a saved workflow, by its installed name or by that name without
/// the installer's prefix, or `primer`. A name that is not one plain path component is refused, so
/// no name reaches a file outside content/'s three directories.
fn resolve(content: &Path, name: &str) -> Result<PathBuf, String> {
    if name == "primer" {
        return Ok(PathBuf::from("PRIMER.md"));
    }
    if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\']) {
        return Err(format!(
            "`{name}` is no name: a name is one installed name, such as \
             `knowledge-architect-review`, or `primer`"
        ));
    }
    let bare = name.strip_prefix(PREFIX).unwrap_or(name);
    let found: Vec<PathBuf> = [
        format!("skills/{bare}/SKILL.md"),
        format!("agents/{bare}.md"),
        format!("workflows/{bare}.js"),
    ]
    .into_iter()
    .map(PathBuf::from)
    .filter(|rel| content.join(rel).is_file())
    .collect();
    match found.as_slice() {
        [one] => Ok(one.clone()),
        [] => Err(format!(
            "no skill, agent or saved workflow is named `{name}` under {}",
            content.display()
        )),
        several => Err(format!(
            "`{name}` names several files: {}",
            several
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// Every file directly under `dir`, as its name and its text; none when `dir` is absent. An error
/// names the file or the directory it met, from `root`.
fn files(root: &Path, dir: &Path) -> Result<Vec<(String, String)>, String> {
    let shown = |path: &Path| {
        path.strip_prefix(root)
            .unwrap_or(path)
            .display()
            .to_string()
    };
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    let entries =
        std::fs::read_dir(dir).map_err(|error| format!("{} is unreadable: {error}", shown(dir)))?;
    for entry in entries {
        let path = entry
            .map_err(|error| format!("{} is unreadable: {error}", shown(dir)))?
            .path();
        if path.is_file() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("{} has no UTF-8 name", shown(&path)))?
                .to_owned();
            let text = std::fs::read_to_string(&path)
                .map_err(|error| format!("{} is unreadable: {error}", shown(&path)))?;
            out.push((name, text));
        }
    }
    Ok(out)
}

/// `files` as the pairs of borrowed names and texts the passes of `render` take.
fn borrowed(files: &[(String, String)]) -> Vec<(&str, &str)> {
    files
        .iter()
        .map(|(n, t)| (n.as_str(), t.as_str()))
        .collect()
}

/// The view of the file `name` names, under the agent-skills crate of the checkout at `root`.
fn view(root: &Path, name: &str) -> Result<String, String> {
    let krate = root.join(CRATE);
    let content = krate.join("content");
    let rel = resolve(&content, name)?;
    let fragments = files(root, &krate.join("fragments"))?;
    let snippets = files(root, &krate.join("snippets"))?;
    let text = std::fs::read_to_string(content.join(&rel))
        .map_err(|error| format!("{CRATE}/content/{} is unreadable: {error}", rel.display()))?;
    render::source_view(&text, &borrowed(&fragments), &borrowed(&snippets))
        .map_err(|why| format!("{CRATE}/content/{}: {why}", rel.display()))
}

pub fn run(name: &str) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(error) => return abort(&format!("no working directory: {error}")),
    };
    let Some(root) = project_root(&cwd, MANIFEST_NAME) else {
        return abort(&format!(
            "not inside the project: no ancestor holds {MANIFEST_NAME}"
        ));
    };
    match view(&root, name) {
        Ok(text) => {
            say(&text);
            ExitCode::SUCCESS
        }
        Err(why) => abort(&why),
    }
}

fn abort(message: &str) -> ExitCode {
    complain(&format!("xtask: {message}"));
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This checkout's root: this crate sits at tools/xtask.
    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("the workspace root exists")
    }

    /// Each name and the file of content/ it resolves to; bound to a name, so the checker reads
    /// the installed names as data rather than as prose.
    const RESOLVED: &[(&str, &str)] = &[
        ("knowledge-architect-review", "skills/review/SKILL.md"),
        ("review", "skills/review/SKILL.md"),
        (
            "knowledge-architect-record-searcher",
            "agents/record-searcher.md",
        ),
        ("tripwire-audit", "workflows/tripwire-audit.js"),
        ("primer", "PRIMER.md"),
    ];

    // The claim: a name resolves with or without the installer's prefix, to a skill, an agent or
    // a saved workflow, and `primer` to the primer; a name of nothing is refused.
    #[test]
    fn a_name_resolves_with_or_without_the_prefix() {
        let content = workspace_root().join(CRATE).join("content");
        for &(name, rel) in RESOLVED {
            assert_eq!(resolve(&content, name), Ok(PathBuf::from(rel)), "{name}");
        }
        assert!(resolve(&content, "no-such-skill").is_err());
    }

    // The claim: a name that is not one plain path component is refused, so none reaches a file
    // outside content/'s three directories, though each of these names one that exists.
    #[test]
    fn a_name_that_is_a_path_is_refused() {
        let content = workspace_root().join(CRATE).join("content");
        for name in [
            "",
            ".",
            "..",
            "../../../../CLAUDE",
            "../skills/review/SKILL",
            "/review",
            "review/",
        ] {
            let refused = resolve(&content, name).expect_err(name);
            assert!(refused.contains("is no name"), "{name}: {refused}");
        }
    }

    // The claim: a name that matches two kinds is refused with both, rather than one picked.
    #[test]
    fn a_name_of_two_files_is_refused() {
        let dir = std::env::temp_dir().join(format!("xtask-skill-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("skills/x")).expect("a skill directory");
        std::fs::create_dir_all(dir.join("agents")).expect("an agents directory");
        std::fs::write(dir.join("skills/x/SKILL.md"), "").expect("a skill");
        std::fs::write(dir.join("agents/x.md"), "").expect("an agent");
        let refused = resolve(&dir, "x").expect_err("two files");
        assert!(
            refused.contains("skills/x/SKILL.md") && refused.contains("agents/x.md"),
            "{refused}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // The claim, over the checkout's own shipped text: every skill, agent, saved workflow and the
    // primer has a view, no fragment or snippet placeholder line is left in it, and every fragment
    // and every snippet is inlined in at least one view, so a view misses no shared text.
    #[test]
    fn every_shipped_text_has_a_view_that_inlines_its_shared_text() {
        let root = workspace_root();
        let krate = root.join(CRATE);
        let content = krate.join("content");
        let mut names = vec!["primer".to_string()];
        for entry in std::fs::read_dir(content.join("skills")).expect("skills/") {
            let name = entry.expect("skills/").file_name();
            names.push(name.to_str().expect("a UTF-8 name").to_string());
        }
        for (dir, suffix) in [("agents", ".md"), ("workflows", ".js")] {
            for (name, _) in files(&root, &content.join(dir)).expect("a content directory") {
                names.push(name.strip_suffix(suffix).unwrap_or(&name).to_string());
            }
        }
        assert!(names.len() > 1, "no shipped text was found");
        let mut shared = Vec::new();
        for dir in ["fragments", "snippets"] {
            for (file, text) in files(&root, &krate.join(dir)).expect("a shared directory") {
                shared.push((format!("{dir}/{file}"), text));
            }
        }
        let mut inlined = Vec::new();
        for name in &names {
            let text = view(&root, name).unwrap_or_else(|why| panic!("{name}: {why}"));
            let mut lines = text.lines();
            while let Some(line) = lines.next() {
                let trimmed = line.trim_start_matches(' ');
                assert!(
                    !trimmed.starts_with(render::FRAGMENT) && !trimmed.starts_with(render::SNIPPET),
                    "{name} keeps a placeholder line: {line}"
                );
                let Some(file) = trimmed.strip_prefix("%% >>> ") else {
                    continue;
                };
                // The lines up to the closing one, their indentation removed, are the file's own.
                let indent = &line[..line.len() - trimmed.len()];
                let close = format!("{indent}%% <<< {file}");
                let mut body = String::new();
                for inner in lines.by_ref() {
                    if inner == close {
                        break;
                    }
                    body.push_str(inner.strip_prefix(indent).unwrap_or(inner));
                    body.push('\n');
                }
                let (_, own) = shared
                    .iter()
                    .find(|(path, _)| path == file)
                    .unwrap_or_else(|| panic!("{name} frames {file}, which is no shared file"));
                assert_eq!(
                    &body, own,
                    "{name}: the lines framed as {file} are not its text"
                );
                inlined.push(file.to_owned());
            }
        }
        for (file, _) in &shared {
            assert!(inlined.contains(file), "no view inlines {file}");
        }
    }
}
