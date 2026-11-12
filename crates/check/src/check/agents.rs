//! The last phase: the installed agent configuration against what this version ships.
//!
//! Under the `claude` harness, each shipped file must be present at its install path with the
//! bytes of its template rendered for the project; no file of the installer's namespace may be
//! one this version does not ship; and the root CLAUDE.md's first line must import the primer when
//! the primer is shipped. The decisions are `design@check@installed-files-compared-by-bytes`,
//! `design@check@primer-import-line`, `design@check@import-on-first-line` and `design@check@agents-table`. The repair of a missing, differing or unshipped file is the
//! install, after the step a cause needs first: removing a directory or an ignore rule in a
//! missing file's way, or renaming a project's own file out of the namespace. A deletion awaiting
//! staging and an import line not on the first line each name their own.
//!
//! **The last phase, since nothing else reads these findings and the model is complete without
//! them.** The installed bytes are read from the survey and the model holds the copies for
//! definitions alone, so a stale copy leaves it right, and a missing one leaves it right about the
//! tree as it is, per `design@check@phases-gate-the-report`.

use std::path::{Path, PathBuf};

use super::Inputs;
use crate::agents::{imports_primer, lf, IMPORT_LINE, PRIMER};
use crate::model::declared::{Pin, AGENT_DOCUMENT, OWNED_PREFIX};
use crate::model::listing::Outside;
use crate::model::{Finding, Manifest, Model};
use crate::CHECKER_VERSION;

pub(crate) fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> Vec<Finding> {
    let mut out = Vec::new();
    if !manifest.serves_claude() {
        return out;
    }
    let install = format!("run `{} install-agent-skills`", manifest.command());
    // Under the `"self"` pin the text compared is what this build ships, which can differ from
    // the release its package version names, so the message names the build, not a version.
    let shipper = match manifest.pin() {
        Some(Pin::OwnBuild) => "this build".to_string(),
        _ => format!("version {}", CHECKER_VERSION),
    };
    let installed = |rel: &Path| inputs.installed.iter().find(|(p, _)| p == rel);
    for (rel, expected) in inputs.shipped {
        match installed(rel).map(|(_, state)| state) {
            // Listed and gone from the working tree: deleted by hand, with the deletion unstaged.
            Some(Outside::Missing) => out.push(Finding::in_file(
                rel,
                format!(
                    "{shipper} ships this file, and git lists it while the working tree does not \
                     hold it"
                ),
                format!("{install}, which writes it again"),
            )),
            None => out.push(Finding::in_file(
                rel,
                format!(
                    "{shipper} ships this file, and git lists no installed copy: it is \
                     missing, an ignore rule covers it, or an untracked directory stands in its \
                     place"
                ),
                // Each cause takes its own first step, since the install alone clears only the
                // first: it writes no file over a directory, and a file it writes under an
                // ignore rule stays out of git's listing, so the finding stays.
                format!(
                    "if an untracked directory stands in its place, remove it; if an ignore rule \
                     covers it, remove the rule; then {install}, and commit the file"
                ),
            )),
            // The install and `check --fix` never replace a directory, so its repair is named.
            Some(Outside::Directory) => out.push(Finding::in_file(
                rel,
                format!(
                    "{shipper} ships this file, and the working tree holds a directory where \
                     git lists it"
                ),
                format!("remove the directory, then {install}"),
            )),
            Some(Outside::Text(text)) if lf(text) == *expected => {}
            Some(_) => out.push(Finding::in_file(
                rel,
                format!(
                    "this installed file differs from what {shipper} ships for this \
                     project"
                ),
                format!(
                    "{install}; a change a project needs belongs in a skill of its own, never \
                     in an installed one"
                ),
            )),
        }
    }
    for (rel, state) in inputs.installed {
        if inputs.shipped.iter().any(|(s, _)| s == rel) {
            continue;
        }
        if *state == Outside::Missing {
            // The install removed it and git still lists it: the deletion awaits staging.
            out.push(Finding::in_file(
                rel,
                "this file of the installer's namespace is deleted, and the deletion is not \
                 staged",
                "stage the deletion",
            ));
            continue;
        }
        out.push(Finding::in_file(
            rel,
            format!(
                "this file sits in the installer's namespace, and {shipper} does \
                 not ship it"
            ),
            // The install removes every file of the namespace this version does not ship, so a
            // project's own file is renamed before it, per `design@check@owned-namespace-check`.
            format!(
                "if the project wrote this file, rename it first to a name outside the \
                 namespace, which is .claude/knowledge-architect/ and the names beginning with \
                 `{OWNED_PREFIX}` under .claude/skills, .claude/agents and .claude/workflows; \
                 otherwise {install}, which removes it"
            ),
        ));
    }
    if inputs.shipped.iter().any(|(s, _)| s == Path::new(PRIMER)) {
        let root = PathBuf::from(AGENT_DOCUMENT);
        match model.documents().iter().find(|d| d.rel == root) {
            Some(doc) if !imports_primer(&doc.text) => out.push(Finding::in_file(
                &root,
                "the root CLAUDE.md does not import the installed primer on its first line",
                format!(
                    "make the file's first line exactly `{IMPORT_LINE}`, and remove that line \
                     wherever else it stands"
                ),
            )),
            Some(_) => {}
            // A root CLAUDE.md the walk keeps out is phase 2's, in `tree`, and a missing one is
            // the required-documents finding of the same phase.
            None => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn manifest(extra: &str) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"p\"\ncomponents = []\n{extra}\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    fn run(
        manifest: &Manifest,
        claude_md: &str,
        installed: &[(&str, Outside)],
        shipped: &[(&str, &str)],
    ) -> Vec<Finding> {
        let model =
            Model::from_documents(vec![(PathBuf::from("CLAUDE.md"), claude_md.to_string())]);
        let installed: Vec<(PathBuf, Outside)> = installed
            .iter()
            .map(|(p, s)| (PathBuf::from(p), s.clone()))
            .collect();
        let shipped: Vec<(PathBuf, String)> = shipped
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let empty_set = HashSet::new();
        let empty_map = HashMap::new();
        let inputs = Inputs {
            committed: &empty_map,
            configs: &empty_map,
            configs_not_text: &std::collections::HashSet::new(),
            generated_unreadable: &std::collections::HashSet::new(),
            present: &empty_set,
            directories: &empty_set,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &installed,
            shipped: &shipped,
            judged: crate::check::Judged::WorkingTree,
        };
        check(&model, manifest, &inputs)
    }

    const SKILL: &str = ".claude/skills/knowledge-architect-a/SKILL.md";
    const IMPORTING: &str = "@.claude/knowledge-architect/PRIMER.md\n\n# P\n";

    /// The claim: an installed set equal to the shipped one, with the primer imported, passes.
    /// Mutation: comparing against the template instead of the rendered text, or requiring the
    /// import line when nothing is shipped, each fails one of the cases below.
    #[test]
    fn a_matching_installed_set_passes() {
        let m = manifest("");
        let shipped = [(SKILL, "run `klarch check`"), (PRIMER, "primer")];
        let installed = [
            (SKILL, Outside::Text("run `klarch check`".into())),
            (PRIMER, Outside::Text("primer".into())),
        ];
        assert!(run(&m, IMPORTING, &installed, &shipped).is_empty());
        assert!(
            run(&m, "# P\n", &[], &[]).is_empty(),
            "nothing shipped, nothing owed"
        );
    }

    /// The claim: under the `"self"` pin, the three findings on the installed set name this
    /// build as what ships, since the text compared is the build's and its package version names
    /// a release whose text can differ; under a version or the fixture pin they name the version.
    /// Mutation: naming the version whatever the pin fails the first half.
    #[test]
    fn the_self_pin_names_the_build_as_what_ships() {
        let shipped = [(SKILL, "fresh"), (PRIMER, "primer")];
        let installed = [
            (SKILL, Outside::Text("edited".into())),
            (
                ".claude/agents/knowledge-architect-old.md",
                Outside::Text("old".into()),
            ),
        ];
        let own = Manifest::parse(
            Path::new("/nowhere"),
            "[project]\nchecker-version = \"self\"\nname = \"p\"\ncomponents = []\n\
             [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n",
        )
        .expect("a declaration");
        let found = run(&own, IMPORTING, &installed, &shipped);
        assert_eq!(found.len(), 3, "{found:?}");
        for f in &found {
            assert!(f.what.contains("this build"), "{f:?}");
            assert!(!f.what.contains("version"), "{f:?}");
        }
        let pinned = Manifest::parse(
            Path::new("/nowhere"),
            &format!(
                "[project]\nchecker-version = \"{}\"\nname = \"p\"\ncomponents = []\n\
                 [walk]\nskip-dirs = []\nskip-files = []\nexclude = []\n",
                CHECKER_VERSION
            ),
        )
        .expect("a declaration");
        let found = run(&pinned, IMPORTING, &installed, &shipped);
        assert_eq!(found.len(), 3, "{found:?}");
        for f in &found {
            assert!(
                f.what.contains(&format!("version {}", CHECKER_VERSION)),
                "a version pin: {f:?}"
            );
        }
        let found = run(&manifest(""), IMPORTING, &installed, &shipped);
        assert_eq!(found.len(), 3, "{found:?}");
        for f in &found {
            assert!(
                f.what.contains(&format!("version {}", CHECKER_VERSION)),
                "{f:?}"
            );
        }
    }

    /// The claim: a root CLAUDE.md whose import line is not exactly its first line is one finding
    /// on that file, the text handed to the check as it was read. Mutation: trimming the text's
    /// start at the call site loses the first two findings.
    #[test]
    fn an_import_line_off_the_first_line_is_one_finding() {
        let m = manifest("");
        let shipped = [(PRIMER, "primer")];
        let installed = [(PRIMER, Outside::Text("primer".into()))];
        for text in [
            "    @.claude/knowledge-architect/PRIMER.md\n",
            "\n@.claude/knowledge-architect/PRIMER.md\n",
            "# P\n\n@.claude/knowledge-architect/PRIMER.md\n",
        ] {
            let found = run(&m, text, &installed, &shipped);
            assert_eq!(found.len(), 1, "{text:?}: {found:?}");
            assert_eq!(found[0].file, Path::new("CLAUDE.md"), "{text:?}");
            assert!(found[0].what.contains("on its first line"), "{found:?}");
        }
    }

    /// The claim: each defect of the installed set is one finding on its own path: a missing
    /// shipped file, a differing one, an unshipped file in the namespace, and a root CLAUDE.md
    /// that does not import the shipped primer. Mutation: deleting any of the four branches
    /// loses its finding.
    #[test]
    fn each_defect_of_the_installed_set_is_one_finding() {
        let m = manifest("");
        let shipped = [(SKILL, "fresh"), (PRIMER, "primer")];
        let installed = [
            (SKILL, Outside::Text("edited".into())),
            (
                ".claude/agents/knowledge-architect-old.md",
                Outside::Text("old".into()),
            ),
        ];
        let found = run(&m, "# P\n", &installed, &shipped);
        let at: Vec<String> = found.iter().map(|f| f.file.display().to_string()).collect();
        assert_eq!(
            at,
            [
                SKILL,
                PRIMER,
                ".claude/agents/knowledge-architect-old.md",
                "CLAUDE.md"
            ],
            "{found:?}"
        );
        assert!(found[0].what.contains("differs"), "{found:?}");
        assert!(found[1].what.contains("no installed copy"), "{found:?}");
        assert!(found[2].what.contains("does not ship"), "{found:?}");
        assert!(
            found
                .iter()
                .all(|f| f.action.contains("install-agent-skills")
                    || f.file == Path::new("CLAUDE.md"))
        );
    }

    /// The claim: a shipped file git does not list names the repair of each cause that needs a
    /// step before the install, an untracked directory in its place and an ignore rule over it,
    /// since the install alone clears neither. Mutation: the action without the ignore rule's
    /// removal fails.
    #[test]
    fn an_unlisted_shipped_file_names_the_step_each_cause_needs() {
        let found = run(&manifest(""), IMPORTING, &[], &[(SKILL, "x")]);
        assert_eq!(found.len(), 1, "{found:?}");
        let action = &found[0].action;
        let install = action.find("install-agent-skills").expect("the install");
        for step in ["remove it", "remove the rule"] {
            let at = action
                .find(step)
                .unwrap_or_else(|| panic!("{step}: {action}"));
            assert!(at < install, "{step} comes before the install: {action}");
        }
    }

    /// The claim: an unshipped file of the namespace names the rename of a project's own file
    /// before the install, which would remove it, and names the namespace. Mutation: the action
    /// naming the install alone fails.
    #[test]
    fn an_unshipped_file_names_the_rename_before_the_install() {
        let stray = ".claude/skills/knowledge-architect-mine/SKILL.md";
        let found = run(
            &manifest(""),
            "# P\n",
            &[(stray, Outside::Text("mine".into()))],
            &[],
        );
        assert_eq!(found.len(), 1, "{found:?}");
        let action = &found[0].action;
        let rename = action.find("rename it").expect("the rename");
        let install = action.find("install-agent-skills").expect("the install");
        assert!(rename < install, "{action}");
        assert!(action.contains("`knowledge-architect-`"), "{action}");
    }

    /// The claim: under `harness = []` nothing about the agent configuration is checked.
    #[test]
    fn no_harness_checks_nothing() {
        let m = manifest("\n[agents]\nharness = []\n");
        let installed = [(SKILL, Outside::Text("anything".into()))];
        assert!(run(&m, "# P\n", &installed, &[(PRIMER, "primer")]).is_empty());
    }

    /// The claim: the repair names the project's declared command.
    #[test]
    fn the_repair_names_the_declared_command() {
        let m = manifest("command = \"cargo klarch\"\n");
        let found = run(&m, "# P\n", &[], &[(SKILL, "x")]);
        assert!(
            found[0]
                .action
                .contains("`cargo klarch install-agent-skills`"),
            "{found:?}"
        );
    }

    /// The claim: a shipped file git lists and the working tree does not hold is reported as such, a binary or unreadable file of the namespace is reported as unshipped, and CRLF
    /// line endings in an installed copy do not make it differ. Mutations: passing a `Missing`
    /// state silently, skipping a non-text stray, or comparing without normalising line endings
    /// each fail one case.
    #[test]
    fn missing_binary_and_crlf_states_are_judged() {
        let m = manifest("");
        let found = run(&m, "# P\n", &[(SKILL, Outside::Missing)], &[(SKILL, "x\n")]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].what.contains("the working tree does not hold it"),
            "{found:?}"
        );
        let stray = ".claude/agents/knowledge-architect-blob.md";
        let found = run(&m, "# P\n", &[(stray, Outside::Binary)], &[]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].what.contains("does not ship"), "{found:?}");
        let crlf = [(SKILL, Outside::Text("a\r\nb\r\n".into()))];
        assert!(run(&m, "# P\n", &crlf, &[(SKILL, "a\nb\n")]).is_empty());
    }
}
