//! The changelog checks through the binary, over a copy of `core` grown into a project of two
//! crates: the root's package and the Component `lib`, in lockstep, `[changelog]` declaring the
//! assembled file at `ALL-CHANGES.md`. The tests of slice 3 of the per-crate changelogs whose state
//! no committed fixture holds: the opt-in, the parse that stops no phase, the assembled file
//! missing, stale and written, the writers' refusals, the preamble left unwritten, and what a
//! changelog crate's files give.

mod support;

use support::Sandbox;

/// The table this project declares.
const TABLE: &str = "\n[changelog]\nassembled = \"ALL-CHANGES.md\"\ncategories = [\"Migration\", \"New features\"]\n";

/// The preamble of the crate `package`, where the assembled file is named, as the manifest above
/// determines it.
fn preamble(package: &str) -> String {
    format!(
        "# Changelog of {package}\n\nThis file holds the entries of this crate only. The changelog \
         of the whole project is ALL-CHANGES.md in its repository.\n\nCategories, in order: \
         Migration, New features."
    )
}

/// A changelog of `package` holding `body` below its preamble.
fn changelog(package: &str, body: &str) -> String {
    format!("{}\n\n{body}", preamble(package))
}

const ROOT_BODY: &str = "## Next release\n\n- Migration, minor: the root moves.\n\n## 0.1.0\n\n- New features, minor: the root's first.\n";
const LIB_BODY: &str = "## Next release\n\n- New features, patch: the library adds\n  over two lines.\n\n## 0.1.0\n\n- Migration, minor: the library's first.\n";

/// The project of two crates, its manifest declaring `table`, staged, its register indexes written
/// and staged. The assembled file is not written.
fn project(tag: &str, table: &str) -> Sandbox {
    let sandbox = Sandbox::new(tag, "core");
    let manifest =
        std::fs::read_to_string(sandbox.path("knowledge-architect.toml")).expect("the manifest");
    assert_eq!(manifest.matches("components = []").count(), 1, "{manifest}");
    sandbox.write(
        "knowledge-architect.toml",
        &format!(
            "{}{table}",
            manifest.replace("components = []", "components = [\"lib\"]")
        ),
    );
    for (rel, text) in [
        ("lib/README.md", "# lib\n\nThe library crate.\n"),
        ("lib/docs/design.md", "# lib — design\n\nNo decision recorded yet.\n"),
        ("lib/docs/rejected.md", "# lib — rejected designs\n\nNo design rejected yet.\n"),
        ("lib/docs/tripwires.md", "# Tripwires — lib\n\nNo tripwire yet.\n"),
        ("lib/docs/open-issues/README.md", "# Open issues — lib\n\nNothing outstanding.\n"),
        (
            "Cargo.toml",
            "[package]\nname = \"core-root\"\nversion.workspace = true\n\n[workspace]\nmembers = [\"lib\"]\n\n\
             [workspace.package]\nversion = \"0.1.0\"\n",
        ),
        (
            "lib/Cargo.toml",
            "[package]\nname = \"core-lib\"\nversion.workspace = true\ninclude = [\"/src/**\", \"/CHANGELOG.md\"]\n",
        ),
    ] {
        sandbox.write(rel, text);
    }
    sandbox.write("CHANGELOG.md", &changelog("core-root", ROOT_BODY));
    sandbox.write("lib/CHANGELOG.md", &changelog("core-lib", LIB_BODY));
    sandbox.stage();
    // The register index of `lib`'s issue home, written by `index` before the assembled file is
    // asked of it, and removed again where `index` also wrote the assembled file.
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{err}");
    let _ = std::fs::remove_file(sandbox.path("ALL-CHANGES.md"));
    sandbox.stage();
    sandbox
}

/// The project with its assembled file written by `index`, staged, which `check` passes.
fn current(tag: &str) -> Sandbox {
    let sandbox = project(tag, TABLE);
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{err}");
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "the project passes: {out}{err}");
    sandbox
}

fn read(sandbox: &Sandbox, rel: &str) -> String {
    std::fs::read_to_string(sandbox.path(rel)).expect("a file of the project")
}

/// The claim, claim 1: a project with published crates, each `CHANGELOG.md` of another shape,
/// that declares no `[changelog]`, yields no changelog finding, names no such check, and has no file
/// assembled. Mutation checked, caught: in `run` of `path@check@src/check/mod.rs`,
/// `.filter(|c| *c != "changelog" || manifest.changelog().is_some())` made `.filter(|_| true)`.
#[test]
fn a_project_that_declares_no_changelog_is_asked_nothing() {
    let sandbox = project("changelog-opt-in", "");
    let other = "# Changes\n\n### Migration\n\n* Something of another shape.\n";
    sandbox.write("CHANGELOG.md", other);
    sandbox.write("lib/CHANGELOG.md", other);
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
    let checked = out
        .lines()
        .find(|l| l.starts_with("checked:"))
        .expect("the line");
    assert!(!checked.contains("changelog"), "{checked}");
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!sandbox.path("ALL-CHANGES.md").exists(), "{out}");
    assert_eq!(read(&sandbox, "CHANGELOG.md"), other);
}

/// The claim, claim 9: with two changelog crates, a missing assembled file and a stale one are
/// each a finding, and `check --fix` writes it in the layout of the assembled file. Mutation
/// checked, caught: in `check` of `path@check@src/check/generated.rs`,
/// `crate::model::changelog::assembled_file(manifest, model)` made
/// `None::<(std::path::PathBuf, String)>`, so the assembled file is compared with nothing.
#[test]
fn a_missing_or_stale_assembled_file_is_a_finding_and_check_fix_writes_it() {
    let sandbox = project("changelog-assembled", TABLE);
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("ALL-CHANGES.md  the generated file is missing"),
        "{out}"
    );
    let (out, err, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(
        out.contains("fixed: wrote ALL-CHANGES.md (regenerated)"),
        "{out}"
    );
    let assembled = read(&sandbox, "ALL-CHANGES.md");
    assert_eq!(
        assembled,
        "**Generated — do not edit.** `klarch index`\n\
         \n## Next release\n\
         \n### Migration\n\n#### core-root\n\n- minor: the root moves.\n\
         \n### New features\n\n#### core-lib\n\n- patch: the library adds\n  over two lines.\n\
         \n## 0.1.0\n\
         \n### Migration\n\n#### core-lib\n\n- minor: the library's first.\n\
         \n### New features\n\n#### core-root\n\n- minor: the root's first.\n"
    );
    sandbox.write(
        "lib/CHANGELOG.md",
        &changelog(
            "core-lib",
            &LIB_BODY.replace("the library adds", "the library adds more"),
        ),
    );
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("ALL-CHANGES.md:15  the generated file is out of date"),
        "{out}"
    );
}

/// The claim, claim 9's other half: with one changelog crate, nothing is assembled, whatever the
/// key, and its preamble names no assembled file. Mutation checked, caught: in
/// `Crates::assembled`, `(self.crates.len() >= 2)` made `(self.crates.len() >= 1)`.
#[test]
fn with_one_changelog_crate_nothing_is_assembled() {
    let sandbox = project(
        "changelog-one-crate",
        &format!("{TABLE}components = [\"lib\"]\n"),
    );
    sandbox.write(
        "lib/CHANGELOG.md",
        &format!(
            "# Changelog of core-lib\n\nThis file holds the entries of this crate only.\n\n\
             Categories, in order: Migration, New features.\n\n{LIB_BODY}"
        ),
    );
    sandbox.stage();
    let (out, err, code) = sandbox.run(&["check"]);
    assert_eq!(code, 0, "{out}{err}");
    let (out, err, code) = sandbox.run(&["index"]);
    assert_eq!(code, 0, "{out}{err}");
    assert!(!sandbox.path("ALL-CHANGES.md").exists(), "{out}");
}

/// The claim, claim 3: a changelog holding a misspelt category and a line of no grammar stops no
/// phase, so `id` and `worklist` answer over it, and the last phase reports each line. Mutations
/// checked, each caught: in `check` of `path@model@src/tree.rs`, phase 2, a finding pushed for each
/// unread line of each changelog crate's file, after `declarations(&mut out, model, manifest, inputs);`;
/// and in `grammar` of `path@check@src/check/changelog/mod.rs`, `for unread in &log.unread {` made
/// `for unread in log.unread.iter().take(0) {`.
#[test]
fn a_malformed_changelog_stops_no_phase() {
    let sandbox = current("changelog-malformed");
    sandbox.write(
        "lib/CHANGELOG.md",
        &changelog(
            "core-lib",
            "## Next release\n\n- Migraton, minor: a misspelt category.\nA line of no grammar.\n\n\
             ## 0.1.0\n\n- Migration, minor: the library's first.\n",
        ),
    );
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("checked: "), "the last phase ran: {out}");
    assert!(
        out.contains("lib/CHANGELOG.md:9  `Migraton` is no category"),
        "{out}"
    );
    assert!(
        out.contains("lib/CHANGELOG.md:10  the line is no entry"),
        "{out}"
    );
    let (out, err, code) = sandbox.run(&["id", "a-free-id"]);
    assert_eq!(code, 0, "{out}{err}");
    let dir = sandbox.outside("worklist");
    let dir_text = dir.to_string_lossy().into_owned();
    let (out, err, code) = sandbox.run(&["worklist", "--out", &dir_text]);
    assert_eq!(code, 0, "{out}{err}");
}

/// The claim, claim 10, first half: each writer refuses to write the assembled file over a file
/// at its path, in the tree it writes, that lacks the generated header, and writes no byte of any
/// generated file. Mutation checked, caught: in `refusal` of
/// `path@model@src/changelogs/render.rs`, `if hand_written {` made `if false {`.
#[test]
fn every_writer_refuses_over_a_file_without_the_generated_header() {
    let sandbox = current("changelog-hand-written");
    let hand = "# Changes written by hand\n";
    sandbox.write("ALL-CHANGES.md", hand);
    // A stale register index the refusal must leave as it is too.
    let index = "lib/docs/open-issues/index.md";
    let stale = format!("{}\nstale\n", read(&sandbox, index));
    sandbox.write(index, &stale);
    sandbox.stage();
    for args in [&["check", "--fix"][..], &["index"], &["index", "--staged"]] {
        let (out, err, code) = sandbox.run(args);
        assert_eq!(code, 2, "{args:?}: {out}{err}");
        assert!(
            err.contains("ALL-CHANGES.md: the file at the assembled changelog's path does not open with the generated header"),
            "{args:?}: {err}"
        );
        assert!(err.contains("Nothing was written."), "{args:?}: {err}");
        assert_eq!(read(&sandbox, "ALL-CHANGES.md"), hand, "{args:?}");
        assert_eq!(read(&sandbox, index), stale, "{args:?}");
    }
    // Nothing was staged either: the index holds the bytes written by hand.
    let staged = sandbox
        .git_command(&["show", ":ALL-CHANGES.md"])
        .output()
        .expect("git runs");
    assert_eq!(String::from_utf8_lossy(&staged.stdout), hand);
}

/// The claim, claim 10, second half: each writer refuses while a changelog holds an unread line,
/// or an entry whose category the manifest does not declare, naming the line, and writes no byte.
/// Mutations checked, each caught: in `refusal`, `if let Some(unread) = log.unread.first() {` made
/// `if let Some(unread) = log.unread.first().filter(|_| false) {`; and `if !admitted {` made
/// `if false {`.
#[test]
fn every_writer_refuses_while_a_line_would_be_omitted() {
    for (tag, body, line) in [
        (
            "changelog-refuse-unread",
            "## Next release\n\n- Migration, minor: kept.\nA line of no grammar.\n",
            10,
        ),
        (
            "changelog-refuse-category",
            "## Next release\n\n- Migration, minor: kept.\n- Nonesuch, minor: no category.\n",
            10,
        ),
    ] {
        let sandbox = current(tag);
        let before = read(&sandbox, "ALL-CHANGES.md");
        sandbox.write("lib/CHANGELOG.md", &changelog("core-lib", body));
        sandbox.stage();
        for args in [&["check", "--fix"][..], &["index"], &["index", "--staged"]] {
            let (out, err, code) = sandbox.run(args);
            assert_eq!(code, 2, "{tag} {args:?}: {out}{err}");
            assert!(
                err.contains(&format!("lib/CHANGELOG.md:{line}: ")),
                "{tag} {args:?}: {err}"
            );
            assert!(
                err.contains("the assembled changelog would omit it"),
                "{err}"
            );
            assert_eq!(read(&sandbox, "ALL-CHANGES.md"), before, "{tag} {args:?}");
        }
    }
}

/// The claim, claim 8: a preamble that differs is a finding whose repair gives the expected text
/// whole, and `check --fix` writes no byte of the crate's changelog. Mutation checked, caught: in
/// `check` of `path@check@src/check/changelog/mod.rs`,
/// `expected_preamble(table, &crate_.package, assembled)` made
/// `expected_preamble(table, &crate_.package, None)`, so the repair gives another text.
#[test]
fn a_preamble_that_differs_is_a_finding_check_fix_does_not_repair() {
    let sandbox = current("changelog-preamble");
    let edited = format!("# Changelog of core-lib\n\nAnother preamble.\n\n{LIB_BODY}");
    sandbox.write("lib/CHANGELOG.md", &edited);
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check", "--fix"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains(
            "lib/CHANGELOG.md:1  the preamble, the lines above the first version heading, differs"
        ),
        "{out}"
    );
    let want = preamble("core-lib");
    assert!(
        out.contains(&want),
        "the repair gives the expected text whole: {out}"
    );
    assert_eq!(read(&sandbox, "lib/CHANGELOG.md"), edited);
}

/// The claim, claim 13: a changelog crate whose `CHANGELOG.md` a `[walk]` row keeps out is a
/// finding naming the row, never one that reports it missing; one whose `CHANGELOG.md` is not
/// there is reported missing, claim 2's last part. Mutation checked, caught: in `check` of
/// `path@check@src/check/changelog/mod.rs`, `.filter(|_| inputs.present.contains(&rel))` made
/// `.filter(|_| false)`, so the file kept out is reported missing.
#[test]
fn a_changelog_a_walk_row_keeps_out_is_named_by_the_row() {
    let sandbox = current("changelog-kept-out");
    let manifest = read(&sandbox, "knowledge-architect.toml");
    sandbox.write(
        "knowledge-architect.toml",
        &manifest.replace("skip-files = []", "skip-files = [\"lib/CHANGELOG.md\"]"),
    );
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("lib/CHANGELOG.md  [walk] skip-files `lib/CHANGELOG.md` keeps out the changelog of `lib`"),
        "{out}"
    );
    assert!(!out.contains("holds no CHANGELOG.md"), "{out}");

    let sandbox = current("changelog-missing");
    std::fs::remove_file(sandbox.path("lib/CHANGELOG.md")).expect("the changelog");
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("lib/CHANGELOG.md  the changelog crate `lib` holds no CHANGELOG.md"),
        "{out}"
    );
}

/// The claim, claim 13's other half: a changelog crate whose `Cargo.toml` `cargo-reader` refuses
/// is a finding carrying its reason, of the last phase, so `id` answers over it. Mutation checked,
/// caught: in `read` of `path@model@src/changelogs/crates.rs`,
/// `out.trouble.push(unreadable(e.to_string()));` made `let _ = e;`.
#[test]
fn a_cargo_toml_cargo_reader_refuses_is_a_finding_with_its_reason() {
    let sandbox = current("changelog-refused-cargo");
    sandbox.write("lib/Cargo.toml", "[package]\nname = 3\n");
    sandbox.stage();
    let (out, _, code) = sandbox.run(&["check"]);
    assert_eq!(code, 1, "{out}");
    assert!(
        out.contains("lib/Cargo.toml  the changelog checks cannot read `lib/Cargo.toml`: `package.name` is not a string"),
        "{out}"
    );
    let (out, err, code) = sandbox.run(&["id", "a-free-id"]);
    assert_eq!(code, 0, "{out}{err}");
}
