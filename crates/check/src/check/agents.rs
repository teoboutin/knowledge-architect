//! The last phase: the installed agent configuration against what this version ships.
//!
//! Under the `claude` harness, each shipped file must be present at its install path with the
//! bytes of its template rendered for the project; no file of the installer's namespace may be
//! one this version does not ship; and the root CLAUDE.md must import the primer when the primer
//! is shipped. The decisions are `design@check@owned-namespace-check`,
//! `design@check@primer-import-line` and `design@check@agents-table`. The repair of a missing, differing or unshipped file is the
//! install; a deletion awaiting staging and a missing import line each name their own.
//!
//! **The last phase, since nothing else reads these findings and the model is complete without
//! them.** The installed bytes are read from the survey and the model holds the copies for
//! definitions alone, so a stale copy leaves it right, and a missing one leaves it right about the
//! tree as it is, per `design@check@phases-gate-the-report`.

use std::path::{Path, PathBuf};

use super::Inputs;
use crate::agents::{imports_primer, lf, IMPORT_LINE, PRIMER};
use crate::model::declared::{Pin, AGENT_DOCUMENT};
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
            None | Some(Outside::Missing) => out.push(Finding::in_file(
                rel,
                format!(
                    "{shipper} ships this file, and git lists no installed copy: it is \
                     missing, or an ignore rule covers it"
                ),
                format!("{install}, and commit the file; an installed file is never ignored"),
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
            format!("{install}, which removes it; a project's own file takes a name of its own"),
        ));
    }
    if inputs.shipped.iter().any(|(s, _)| s == Path::new(PRIMER)) {
        let root = PathBuf::from(AGENT_DOCUMENT);
        match model.documents().iter().find(|d| d.rel == root) {
            Some(doc) if !imports_primer(&doc.text) => out.push(Finding::in_file(
                &root,
                "the root CLAUDE.md does not import the installed primer",
                format!("add a line holding exactly `{IMPORT_LINE}`"),
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
            present: &empty_set,
            directories: &empty_set,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &installed,
            shipped: &shipped,
        };
        check(&model, manifest, &inputs)
    }

    const SKILL: &str = ".claude/skills/knowledge-architect-a/SKILL.md";
    const IMPORTING: &str = "# P\n\n@.claude/knowledge-architect/PRIMER.md\n";

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

    /// The claim: a shipped file git lists and the working tree does not hold is reported as not
    /// installed, a binary or unreadable file of the namespace is reported as unshipped, and CRLF
    /// line endings in an installed copy do not make it differ. Mutations: passing a `Missing`
    /// state silently, skipping a non-text stray, or comparing without normalising line endings
    /// each fail one case.
    #[test]
    fn missing_binary_and_crlf_states_are_judged() {
        let m = manifest("");
        let found = run(&m, "# P\n", &[(SKILL, Outside::Missing)], &[(SKILL, "x\n")]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].what.contains("no installed copy"), "{found:?}");
        let stray = ".claude/agents/knowledge-architect-blob.md";
        let found = run(&m, "# P\n", &[(stray, Outside::Binary)], &[]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].what.contains("does not ship"), "{found:?}");
        let crlf = [(SKILL, Outside::Text("a\r\nb\r\n".into()))];
        assert!(run(&m, "# P\n", &crlf, &[(SKILL, "a\nb\n")]).is_empty());
    }
}
