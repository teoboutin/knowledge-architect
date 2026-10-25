//! The files `knowledge-architect` installs into a project's agent configuration: skills,
//! subagent definitions and the primer.
//!
//! Each entry is the path the file is installed at, relative to the project root, and its text.
//! The list is generated from this crate's content/ directory by its build script, each file
//! rendered with the snippets of snippets/ that its placeholder lines name.

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

/// Every file the installer writes, as its install path and its text, in path order.
pub static FILES: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/files.rs"));

/// Every file of the snippets the setup skill inlines, as its file name and its text, in name
/// order: `klarch init --rust` writes every one of them but `ci.yml`, so a project gets what the
/// skill shows.
pub static SNIPPETS: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/snippets.rs"));

/// This crate's directory, compiled in, so a binary that links it can refuse a build made from
/// another checkout, per `design@core@a-foreign-build-is-refused`.
pub const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// This crate's package name, for the same refusal.
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod render;

#[cfg(test)]
mod tests {
    use super::FILES;

    /// The setup skill ships the maintenance crate's main that xtask's example compiles.
    #[test]
    fn the_setup_skill_ships_the_compiled_snippet() {
        let (_, setup) = FILES
            .iter()
            .find(|(path, _)| *path == ".claude/skills/knowledge-architect-setup/SKILL.md")
            .expect("the setup skill is shipped");
        // The build drops the snippet's own final line break and keeps the placeholder line's.
        let snippet = include_str!("../snippets/xtask-main.rs").trim_end_matches('\n');
        assert!(
            setup.contains(&format!("```rust\n{snippet}\n```\n")),
            "the setup skill does not hold snippets/xtask-main.rs as a fenced block"
        );
    }

    /// No shipped file holds a `%%` line: the build removes each comment line, and refuses one it
    /// would ship, per `design@agent-skills@shipped-text-line-comments`. The rules of the pass
    /// itself are held by the tests of `render`.
    #[test]
    fn no_shipped_file_holds_a_comment_line() {
        for (path, text) in FILES {
            assert!(
                !text.lines().any(|line| line.trim_start().starts_with("%%")),
                "{path} ships a `%%` line"
            );
        }
    }

    /// No shipped file holds an unfilled substitution placeholder. The text holds `{{` only as the
    /// install's `{{command}}` and as the `${{ ` that opens a GitHub Actions expression, which the
    /// setup skill writes with a space.
    #[test]
    fn no_shipped_file_holds_an_unfilled_placeholder() {
        for (path, text) in FILES {
            let rest = text.replace("{{command}}", "").replace("${{ ", "");
            assert!(!rest.contains("{{"), "{path} ships an unfilled placeholder");
        }
    }

    /// Each host ships every fragment its source names, rendered, once per placeholder: the
    /// fragment pass of the build, `render::render_fragments`, ran over it and its text was not
    /// lost to a later pass. The source of a shipped file is read back through its install path.
    #[test]
    fn every_host_ships_each_fragment_it_names() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut fragments = Vec::new();
        for entry in std::fs::read_dir(root.join("fragments")).expect("fragments/ is readable") {
            let path = entry.expect("fragments/ is readable").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .expect("a UTF-8 name");
            let text = std::fs::read_to_string(&path).expect("a fragment is UTF-8");
            fragments.push((name.to_owned(), text));
        }
        let shared: Vec<(&str, &str)> = fragments
            .iter()
            .map(|(n, t)| (n.as_str(), t.as_str()))
            .collect();
        let mut hosts = 0;
        for (path, shipped) in FILES {
            let source = match path.strip_prefix(".claude/") {
                Some("knowledge-architect/PRIMER.md") => "PRIMER.md".to_owned(),
                Some(rest) => rest.replacen("knowledge-architect-", "", 1),
                None => panic!("{path} is installed outside .claude/"),
            };
            let text = std::fs::read_to_string(root.join("content").join(&source))
                .unwrap_or_else(|e| panic!("{path} maps to content/{source}, unreadable: {e}"));
            for line in text.lines() {
                if !line.trim_start().starts_with(crate::render::FRAGMENT) {
                    continue;
                }
                hosts += 1;
                let (block, _) = crate::render::render_fragments(
                    &format!("{line}\n"),
                    &shared,
                    std::path::Path::new(&source),
                );
                assert_ne!(
                    block.trim_end(),
                    line,
                    "{source}: `{}` is not rendered as a fragment placeholder",
                    line.trim()
                );
                let placed = text.lines().filter(|l| *l == line).count();
                assert_eq!(
                    shipped.matches(block.as_str()).count(),
                    placed,
                    "{path} does not ship `{}` once per placeholder",
                    line.trim()
                );
            }
        }
        assert!(hosts > 0, "no content file names a fragment");
    }

    /// Each fragment reaches every text whose act applies it whole, and no other, per the hosts the
    /// owner approved, `design@agent-skills@shared-text-fragment`. A host that loses its
    /// placeholder still ships and still passes the test above, so this list is what fails: change
    /// it with the hosts.
    #[test]
    fn each_fragment_has_its_hosts() {
        let hosts: &[(&str, &[&str])] = &[
            (
                "tripwire-standard.md",
                &[
                    ".claude/agents/knowledge-architect-decision-record-reviewer.md",
                    ".claude/agents/knowledge-architect-tripwire-auditor.md",
                    ".claude/skills/knowledge-architect-decision-recording/SKILL.md",
                    ".claude/skills/knowledge-architect-design/SKILL.md",
                    ".claude/skills/knowledge-architect-issue-tracking/SKILL.md",
                    ".claude/skills/knowledge-architect-planning/SKILL.md",
                ],
            ),
            (
                "tripwire-lifecycle.md",
                &[
                    ".claude/agents/knowledge-architect-standing-state-reviewer.md",
                    ".claude/agents/knowledge-architect-tripwire-auditor.md",
                    ".claude/skills/knowledge-architect-issue-tracking/SKILL.md",
                ],
            ),
        ];
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for (fragment, expected) in hosts {
            let text = std::fs::read_to_string(root.join("fragments").join(fragment))
                .expect("the fragment is readable");
            let first = text
                .lines()
                .find(|l| !l.trim().is_empty() && !l.starts_with("%%"))
                .expect("a fragment holds a line");
            let mut found: Vec<&str> = FILES
                .iter()
                .filter(|(_, shipped)| shipped.lines().any(|l| l.trim() == first.trim()))
                .map(|(path, _)| *path)
                .collect();
            found.sort();
            assert_eq!(&found, expected, "the hosts of fragments/{fragment}");
        }
    }

    /// No shipped file holds a snippet placeholder: a line that is not exactly one, such as an
    /// indented one, is left as it is by the build.
    #[test]
    fn no_shipped_file_holds_a_snippet_placeholder() {
        for (path, text) in FILES {
            assert!(
                !text.contains("{{snippet:"),
                "{path} ships a snippet placeholder"
            );
        }
    }
}
