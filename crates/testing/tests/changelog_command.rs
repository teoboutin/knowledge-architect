//! `klarch changelog` through the binary, over a mock consumer: a copy of `core` given a Cargo
//! package of its own, whose dependencies are path dependencies on mock published packages outside
//! it, as a registry's sources are outside a consumer. Three of them stand for one project,
//! `knowledge-architect`, `-gates` and `-mutate`, with one `repository`, the first depending on
//! neither of the other two, as the real `knowledge-architect` does not; one stands for another
//! project; and three set no `repository`, one of them shipping no changelog and one holding the
//! defects the output cannot hold. The tests of slice 4 of the per-crate changelogs.
//!
//! A path dependency's package is reported by `cargo metadata` as a registry one is, a
//! `manifest_path` at its source, per `path@cargo-reader@docs/grounding/cargo-metadata.md`, and
//! needs no network.

mod support;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use support::Sandbox;

/// The `repository` of the mock project the three packages stand for.
const KA: &str = "https://example.org/knowledge-architect";

/// Each mock package: its name, its `repository`, and its `CHANGELOG.md` below the preamble, or
/// `None` where it ships none.
const PACKAGES: &[(&str, Option<&str>, Option<&str>)] = &[
    (
        "knowledge-architect",
        Some(KA),
        Some(
            "## Next release\n\n- Migration, major: ka not yet released.\n\n\
             ## 0.2.0\n\n- Migration, minor: ka two moves.\n- New features, minor: ka two adds.\n\n\
             ## 0.1.0\n\n- Migration, minor: ka one moves.\n",
        ),
    ),
    (
        "knowledge-architect-gates",
        Some(KA),
        Some(
            "## Next release\n\n## 0.2.0\n\n- Migration, patch: gates two moves.\n\n\
             ## 0.1.0\n\n- New features, minor: gates one adds.\n",
        ),
    ),
    (
        "knowledge-architect-mutate",
        Some(KA),
        Some("## Next release\n\n## 0.1.0\n\n- Migration, minor: mutate one moves.\n"),
    ),
    (
        "other",
        Some("https://example.org/other"),
        Some("## Next release\n\n## 0.2.0\n\n- Migration, major: other two moves.\n"),
    ),
    (
        "lone",
        None,
        Some("## Next release\n\n## 0.2.0\n\n- New features, minor: lone two adds.\n"),
    ),
    ("nolog", None, None),
    (
        "defects",
        None,
        Some(
            "## Next release\n\n## 0.2.0\n\n- Migration, minor: defects two moves.\n\
             - Workflow, patch: no category the preamble lists.\nstray text in the range\n\n\
             ## 0.1.0\n\nstray text below the range\n- an entry with no tag below the range\n",
        ),
    ),
];

/// A crate's preamble, as the publisher's manifest determines it.
fn preamble(package: &str) -> String {
    format!(
        "# Changelog of {package}\n\nThis file holds the entries of this crate only. The changelog \
         of the whole project is CHANGELOG.md in its repository.\n\nCategories, in order: \
         Migration, New features."
    )
}

/// The version every mock package starts at, which the consumer requires exactly.
const VERSION: &str = "0.2.0";

/// The mock consumer, its lock file generated offline: a copy of `core`, which `klarch` runs over,
/// holding the package `consumer`, and the mock packages under `deps/` outside the project.
fn consumer(tag: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "core");
    let mut dependencies = String::new();
    for (name, repository, log) in PACKAGES {
        let dir = sandbox.outside(&format!("deps/{name}"));
        let repository = repository.map_or(String::new(), |r| format!("repository = \"{r}\"\n"));
        write(
            &dir.join("Cargo.toml"),
            &package_manifest(name, VERSION, &repository),
        );
        write(&dir.join("src/lib.rs"), "");
        if let Some(log) = log {
            write(
                &dir.join("CHANGELOG.md"),
                &format!("{}\n\n{log}", preamble(name)),
            );
        }
        dependencies.push_str(&format!(
            "{name} = {{ path = \"{}\", version = \"={VERSION}\" }}\n",
            slashed(&dir)
        ));
    }
    sandbox.write(
        "Cargo.toml",
        &format!(
            "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n\
             [workspace]\n\n[dependencies]\n{dependencies}"
        ),
    );
    sandbox.write("src/lib.rs", "");
    cargo(&sandbox.dir, &["generate-lockfile", "--offline"]);
    sandbox
}

/// A package's `Cargo.toml`, with a `[workspace]` table of its own, so cargo searches no
/// directory above it for one, whatever the temporary directory sits under.
fn package_manifest(name: &str, version: &str, repository: &str) -> String {
    format!(
        "[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\n{repository}\n\
         [workspace]\n"
    )
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("the parent directory");
    std::fs::write(path, text).expect("a written file");
}

/// A path as a TOML string holds it, with `/` between its segments on every system.
fn slashed(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// One run of the cargo that runs this test, in `dir`, which must succeed.
fn cargo(dir: &Path, args: &[&str]) {
    let cargo = std::env::var_os("CARGO").expect("cargo sets CARGO for a test it runs");
    let out = Command::new(cargo)
        .args(args)
        .current_dir(dir)
        .output()
        .expect("cargo runs");
    assert!(
        out.status.success(),
        "cargo {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Every file under `dir`, `.git` apart, with its bytes.
fn files(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a readable directory") {
            let entry = entry.expect("a directory entry");
            let path = entry.path();
            if entry.file_type().expect("a file type").is_dir() {
                if entry.file_name() != ".git" {
                    stack.push(path);
                }
            } else {
                out.insert(path.clone(), std::fs::read(&path).expect("a readable file"));
            }
        }
    }
    out
}

/// Every file of the consumer and of its dependencies.
fn snapshot(sandbox: &Sandbox) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut all = files(&sandbox.dir);
    all.extend(files(&sandbox.outside("deps")));
    all
}

/// The layout of the three packages of `KA` over (0.1.0, 0.2.0].
const KA_TWO: &str =
    "## 0.2.0\n\n### Migration\n\n#### knowledge-architect\n\n- minor: ka two moves.\n\n\
    #### knowledge-architect-gates\n\n- patch: gates two moves.\n\n### New features\n\n\
    #### knowledge-architect\n\n- minor: ka two adds.\n";

/// Claim 1 of slice 4, the selection: the packages of the named package's `repository` and no
/// other; the named package alone where it sets none; and exactly the named set where `--package`
/// is given. A name the graph does not hold is a could-not-run. Mutations checked, each caught: in
/// `select` of `path@query@src/changelog.rs`, `if package.repository.as_ref() == Some(repository) {`
/// made `if true {`; and `names.sort_unstable();` removed.
#[test]
fn the_selection_is_the_repository_s_packages_or_the_named_set() {
    let sandbox = consumer("changelog-selection");
    let (out, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.1.0"]);
    assert_eq!((code, out.as_str()), (0, KA_TWO), "{err}");

    let (out, err, code) = sandbox.run(&["changelog", "lone", "--from", "0.1.0"]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "## 0.2.0\n\n### New features\n\n#### lone\n\n- minor: lone two adds.\n"
        ),
        "{err}"
    );

    let (out, err, code) = sandbox.run(&[
        "changelog",
        "lone",
        "--from",
        "0.1.0",
        "--package",
        "other",
        "--package",
        "knowledge-architect-gates",
    ]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "## 0.2.0\n\n### Migration\n\n#### knowledge-architect-gates\n\n\
             - patch: gates two moves.\n\n#### other\n\n- major: other two moves.\n\n\
             ### New features\n\n#### lone\n\n- minor: lone two adds.\n"
        ),
        "{err}"
    );

    for args in [
        &["changelog", "absent", "--from", "0.1.0"][..],
        &[
            "changelog",
            "lone",
            "--from",
            "0.1.0",
            "--package",
            "absent",
        ][..],
    ] {
        let (out, err, code) = sandbox.run(args);
        assert_eq!((code, out.as_str()), (2, ""), "{args:?}: {err}");
        assert!(err.contains("`absent` is no package"), "{err}");
    }
}

/// Claim 2 of slice 4, the range: the versions in (`--from`, `--to`], and with `--to` absent
/// every version above `--from`, `Next release` excluded, in the assembled layout, the named
/// package first, then the others by package name. Mutation checked: in `assemble` of
/// `path@query@src/changelog.rs`, `.filter(|s| range.contains(s.heading))` made
/// `.filter(|_| true)`; caught.
#[test]
fn the_output_is_the_range_in_the_assembled_layout() {
    let sandbox = consumer("changelog-range");
    let (out, err, code) = sandbox.run(&[
        "changelog",
        "knowledge-architect-mutate",
        "--from",
        "0.0.1",
        "--to",
        "0.1.0",
    ]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "## 0.1.0\n\n### Migration\n\n#### knowledge-architect-mutate\n\n\
             - minor: mutate one moves.\n\n#### knowledge-architect\n\n- minor: ka one moves.\n\n\
             ### New features\n\n#### knowledge-architect-gates\n\n- minor: gates one adds.\n"
        ),
        "{err}"
    );
    let (out, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.0.1"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.starts_with(KA_TWO), "{out}");
    assert!(out.contains("\n## 0.1.0\n"), "{out}");
    assert!(!out.contains("not yet released"), "{out}");
    let (out, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.2.0"]);
    assert_eq!(
        (code, out.as_str()),
        (0, "No entry of the selected changelogs is in the range.\n"),
        "{err}"
    );
    let (out, err, code) = sandbox.run(&[
        "changelog",
        "knowledge-architect",
        "--from",
        "0.2.0",
        "--to",
        "0.2.0",
    ]);
    assert_eq!(code, 2, "{out}");
    assert!(err.contains("`--to` is not above `--from`"), "{err}");
}

/// Claim 3 of slice 4, the order of the pin move, and claim 4, no write: after the pin's edit, the
/// command run before `cargo fetch` fails with cargo's message and a repair naming the fetch; run
/// after it, it prints the new version's section. No run writes a file of the consumer or of its
/// dependencies, `Cargo.lock` included, and a consumer with no lock file is refused rather than
/// given one. Mutation checked: in `changelog` of `path@query@src/changelog.rs`,
/// `run_metadata(manifest.root(), &["--locked", "--offline"])` made
/// `run_metadata(manifest.root(), &["--offline"])`; caught.
#[test]
fn the_command_runs_after_the_fetch_and_writes_nothing() {
    let sandbox = consumer("changelog-pin-move");
    let before = snapshot(&sandbox);
    let (_, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.1.0"]);
    assert_eq!(code, 0, "{err}");
    assert!(snapshot(&sandbox) == before, "the run wrote nothing");

    // The pin's edit: the new version of the dependency, and the consumer's requirement.
    let ka = sandbox.outside("deps/knowledge-architect");
    write(
        &ka.join("Cargo.toml"),
        &package_manifest(
            "knowledge-architect",
            "0.3.0",
            &format!("repository = \"{KA}\"\n"),
        ),
    );
    let log = std::fs::read_to_string(ka.join("CHANGELOG.md")).expect("the changelog");
    write(
        &ka.join("CHANGELOG.md"),
        &log.replace(
            "## 0.2.0\n",
            "## 0.3.0\n\n- Migration, major: ka three moves.\n\n## 0.2.0\n",
        ),
    );
    let manifest = std::fs::read_to_string(sandbox.path("Cargo.toml")).expect("the manifest");
    let edited = manifest.replacen(
        &format!("version = \"={VERSION}\" }}\n"),
        "version = \"=0.3.0\" }\n",
        1,
    );
    assert_ne!(edited, manifest);
    sandbox.write("Cargo.toml", &edited);

    let edited = snapshot(&sandbox);
    let (out, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.2.0"]);
    assert_eq!((code, out.as_str()), (2, ""), "{err}");
    assert!(err.contains("cannot update the lock file"), "{err}");
    assert!(err.contains("run `cargo fetch` first"), "{err}");
    assert!(
        snapshot(&sandbox) == edited,
        "the failing run wrote nothing"
    );

    cargo(&sandbox.dir, &["fetch"]);
    let fetched = snapshot(&sandbox);
    let (out, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.2.0"]);
    assert_eq!(
        (code, out.as_str()),
        (
            0,
            "## 0.3.0\n\n### Migration\n\n#### knowledge-architect\n\n- major: ka three moves.\n"
        ),
        "{err}"
    );
    assert!(snapshot(&sandbox) == fetched, "the run wrote nothing");

    std::fs::remove_file(sandbox.path("Cargo.lock")).expect("the lock is removed");
    let (out, err, code) = sandbox.run(&["changelog", "knowledge-architect", "--from", "0.2.0"]);
    assert_eq!((code, out.as_str()), (2, ""), "{err}");
    assert!(err.contains("cannot create the lock file"), "{err}");
    assert!(!sandbox.path("Cargo.lock").exists(), "no lock is written");
}

/// Claim 5 of slice 4, what the output cannot hold is reported, never dropped: a selected package
/// with no `CHANGELOG.md`, a line of a preamble the read does not take, every unread line whatever
/// its section, and, in a section of the range, an entry with no place, each with its file and,
/// but for the missing file, its line, the rest of the layout still printed, and exit 1; an entry
/// with no place below the range is not reported. Mutation checked: in `changelog` of
/// `path@query@src/changelog.rs`, `Ok(ExitCode::from(code))` made `Ok(ExitCode::SUCCESS)`;
/// caught.
#[test]
fn what_the_output_cannot_hold_is_reported_after_it() {
    let sandbox = consumer("changelog-unheld");
    let defects = sandbox.outside("deps/defects/CHANGELOG.md");
    let log = std::fs::read_to_string(&defects).expect("the changelog");
    write(
        &defects,
        &log.replacen("only.", "only.\nA line of no shape.", 1),
    );
    let (out, err, code) = sandbox.run(&[
        "changelog",
        "defects",
        "--from",
        "0.1.0",
        "--package",
        "nolog",
    ]);
    assert_eq!(code, 1, "{out}{err}");
    let file = slashed(&defects);
    let nolog = slashed(&sandbox.outside("deps/nolog/CHANGELOG.md"));
    let (layout, reported) = out
        .split_once("\n\nNot printed, since the layout has no place for it:\n")
        .expect("the report after the layout");
    assert_eq!(
        layout,
        "## 0.2.0\n\n### Migration\n\n#### defects\n\n- minor: defects two moves."
    );
    let reported: Vec<String> = reported.lines().map(|l| l.replace('\\', "/")).collect();
    assert_eq!(reported.len(), 5, "{reported:#?}");
    assert!(
        reported[0].starts_with(&format!(
            "{file}:4: a line of the preamble of no shape the read takes"
        )),
        "{reported:#?}"
    );
    assert!(
        reported[1].starts_with(&format!("{file}:13: an entry")),
        "{reported:#?}"
    );
    assert!(
        reported[2].starts_with(&format!("{file}:14: a line the parse could not read")),
        "{reported:#?}"
    );
    assert!(
        reported[3].starts_with(&format!("{file}:18: a line the parse could not read")),
        "{reported:#?}"
    );
    assert!(
        reported[4].starts_with(&format!("{nolog}: cannot be read")),
        "{reported:#?}"
    );
}

/// Claim 3 of the repairs of slice 4's first review: the command reads the Cargo project the
/// project's root holds, never one of a package enclosing it. A project whose root holds no
/// `Cargo.toml` is refused, exit 2, naming the reading by hand of a project with no Rust and no
/// `cargo fetch`, even nested in the directory of the consumer's package. Mutation checked: in
/// `changelog` of `path@query@src/changelog.rs`, `if !cargo_toml.is_file() {` made `if false {`;
/// caught.
#[test]
fn a_root_with_no_cargo_project_is_refused() {
    let sandbox = consumer("changelog-no-cargo");
    let nested = sandbox.path("docs-project");
    support::copy_dir(&support::source("core"), &nested);
    let (out, err, code) = support::run_in(
        &nested,
        &["changelog", "knowledge-architect", "--from", "0.1.0"],
    );
    assert_eq!((code, out.as_str()), (2, ""), "{err}");
    assert!(err.contains("holds no Cargo project at its root"), "{err}");
    assert!(err.contains("a project with no Rust reads"), "{err}");
    assert!(!err.contains("cargo fetch"), "{err}");
}
