//! `changelog`: the changelogs of a project's dependencies, from the sources cargo fetched, over a
//! range of versions, in the layout of an assembled changelog with no generated header.
//!
//! The selection, the range, the order of the crates, what the output cannot hold, and the text
//! printed with its exit code are pure, over what `cargo metadata` reports and the texts of the
//! files; the run of Cargo, the reads of the files and the printing are the thin layer of
//! [`changelog`].

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::cargo_reader::{run_metadata, MetadataPackage};
use crate::model::changelog::{
    assembled_body_by_table, parse, read_preamble, unplaced, Changelog, Heading, Version,
    CHANGELOG_FILE,
};
use crate::model::declared::ChangelogTable;
use crate::output::outln;
use crate::Manifest;

/// The versions a run prints: those above `from`, up to `to` included where it is given, and
/// every one above `from` where it is not. `Next release` is in no range, since it is no
/// version a pin can cross, per `design@query@changelog-range-excludes-the-old-pin`.
///
/// It cannot hold a `to` that is not above `from`: [`VersionRange::new`] refuses it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionRange {
    from: Version,
    to: Option<Version>,
}

impl VersionRange {
    /// The range (`from`, `to`], or (`from`, ∞) where `to` is `None`.
    ///
    /// # Errors
    ///
    /// A message naming both versions where `to` is not above `from`: such a range holds no
    /// version.
    pub fn new(from: Version, to: Option<Version>) -> Result<VersionRange, String> {
        match to {
            Some(to) if to <= from => Err(format!(
                "the range's end {to} is not above its start {from}, so it holds no version"
            )),
            _ => Ok(VersionRange { from, to }),
        }
    }

    /// Its start, excluded.
    pub fn from(&self) -> Version {
        self.from
    }

    /// Its end, included, where one is given.
    pub fn to(&self) -> Option<Version> {
        self.to
    }

    /// Whether the section `heading` names is in the range.
    pub fn contains(&self, heading: Heading) -> bool {
        match heading {
            Heading::NextRelease => false,
            Heading::Version(v) => v > self.from && self.to.is_none_or(|to| v <= to),
        }
    }
}

/// What `changelog` is asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangelogRequest {
    /// The package named, which the output gives first.
    pub package: String,
    /// The versions printed.
    pub range: VersionRange,
    /// The packages `--package` names; where any is given, the selection is `package` and these.
    pub packages: Vec<String>,
}

/// The changelogs of the packages a project depends on, over a range of versions: the command
/// `changelog`, which the pin move runs after the pin's edit and the fetch, per
/// `skill@knowledge-architect-setup@moving-the-pin`.
///
/// It runs `cargo metadata --format-version 1 --manifest-path <root>/Cargo.toml --locked
/// --offline` at the manifest's root, so the graph is that of the Cargo project the root holds,
/// never one of a package enclosing it, per `design@query@changelog-reads-the-root-s-graph`; it
/// writes nothing of the project, `Cargo.lock` included, and reaches no network, per
/// `design@query@changelog-runs-cargo-locked-and-offline`, as
/// `path@cargo-reader@docs/grounding/cargo-metadata.md` measures the flags. Of the resolved graph
/// it selects, per `design@query@changelog-selects-by-repository`, each package in its newest
/// version where the graph holds several: with `request.packages` empty, every package
/// whose `repository` equals the named package's, or the named package alone where it sets none;
/// otherwise the named package and those. It reads the
/// `CHANGELOG.md` beside each one's `Cargo.toml`, which lies in the sources cargo fetched or, for a
/// member or a path dependency, in its directory, possibly inside the project; places each one's
/// entries by the categories and the unclassed versions its own preamble states, per
/// `design@query@changelog-places-each-crate-by-its-own-preamble`; and prints the
/// sections of the range in the layout of an assembled changelog with no generated header, the
/// named package first under each category, then the others by package name, per
/// `design@query@consumer-crate-order`. A version none of them holds an entry of is not printed;
/// where the range holds no entry, a line says so, and where it holds entries the layout has no
/// place for alone, a line says that instead.
///
/// After the layout, it prints one line per thing the layout cannot hold, per
/// `design@query@changelog-reports-what-it-cannot-print`, naming the file and,
/// but for a file it does not read, the line: a selected package with no readable `CHANGELOG.md`,
/// or one holding a carriage return that no line feed follows, which ends no line the parse
/// takes, so that the file would read as one line;
/// every line of a preamble of no shape the read takes and every line the parse could not read,
/// whatever its section, since a version heading the parse does not take is such a line, and
/// leaves its entries in the section above it; and, in a section of the range, each entry its
/// preamble gives no place, of a category it does not list or with no tag in a classed version.
///
/// The output is for a reader, not a contract, per
/// `issue@knowledge-architect@command-output-is-not-declared-a-contract`. Exit 0 when it reports
/// nothing after the layout, 1 when it reports a line it cannot hold, per
/// `design@cli@exit-code-ladder`.
///
/// # Errors
///
/// An `Err`, the could-not-run exit 2, with nothing printed: a root that holds no `Cargo.toml`; Cargo's failure, with its message and the repair of running `cargo fetch` after the pin's
/// edit, as a lock file that a pin's edit left stale fails under `--locked`; and a named package
/// or a `request.packages` name that the resolved graph does not hold.
pub fn changelog(manifest: &Manifest, request: &ChangelogRequest) -> Result<ExitCode, String> {
    let root = manifest.root();
    let cargo_toml = root.join("Cargo.toml");
    if !cargo_toml.is_file() {
        return Err(format!(
            "the project holds no Cargo project at its root, {}: `changelog` reads the dependency \
             graph `cargo metadata` resolves from the root's Cargo.toml",
            root.display()
        ));
    }
    let cargo_toml = cargo_toml
        .to_str()
        .ok_or_else(|| format!("the path {} is not UTF-8", cargo_toml.display()))?;
    // Per `design@query@changelog-runs-cargo-locked-and-offline`: no write and no network.
    let flags = ["--manifest-path", cargo_toml, "--locked", "--offline"];
    let metadata = run_metadata(root, &flags).map_err(|e| {
        format!(
            "{e}\n       where the pin was just edited, run `cargo fetch` first, which \
             fetches the new version and updates the lock file, then run this command again"
        )
    })?;
    let selected = select(&metadata.packages, &request.package, &request.packages)?;
    let read: Vec<Read> = selected
        .iter()
        .map(|package| {
            let file = package
                .manifest_path
                .parent()
                .unwrap_or(Path::new(""))
                .join(CHANGELOG_FILE);
            let text = std::fs::read_to_string(&file).map_err(|e| e.to_string());
            Read {
                package: package.name.clone(),
                file,
                text,
            }
        })
        .collect();
    let (text, code) = output(&assemble(&request.range, &read));
    outln!("{text}");
    Ok(ExitCode::from(code))
}

/// The text the command prints, with no final line break, and its exit code, from what
/// [`assemble`] gives: the layout, or, where it is empty, a line saying the range holds no entry,
/// where it holds none and nothing is listed below, or else that nothing of the range is printed;
/// then, where any line cannot be held, a blank line, a heading and each such line, with the code
/// 1; else 0. A file not read and a version heading the parse did not take hide entries
/// `in_range` does not count, so the line claims an empty range only where nothing is listed.
fn output(assembled: &Assembled) -> (String, u8) {
    // The body opens each version with a blank line, as the assembled file holds it below its
    // header; printed alone, that line would open the output.
    let body = assembled.body.strip_prefix('\n').unwrap_or(&assembled.body);
    let body = body.trim_end_matches('\n');
    let nothing_listed = assembled.unheld.is_empty();
    let mut text = match (body.is_empty(), assembled.in_range, nothing_listed) {
        (false, _, _) => body.to_string(),
        (true, 0, true) => "No entry of the selected changelogs is in the range.".to_string(),
        (true, _, _) => {
            "Nothing of the range is printed; what the layout cannot hold is listed below."
                .to_string()
        }
    };
    if nothing_listed {
        return (text, 0);
    }
    text.push_str("\n\nNot printed, since the layout has no place for it:");
    for line in &assembled.unheld {
        text.push('\n');
        text.push_str(line);
    }
    (text, 1)
}

/// The packages of the graph `packages` that a request selects, per
/// `design@query@changelog-selects-by-repository`, in the order of the output: the named package
/// first, then the others by package name, per `design@query@consumer-crate-order`, each once; each
/// in its newest version, as the comment below says why.
fn select<'m>(
    packages: &'m [MetadataPackage],
    named: &str,
    extra: &[String],
) -> Result<Vec<&'m MetadataPackage>, String> {
    // One package per name. The graph can hold one at several versions, each with its own
    // file; the newest one's holds the sections of every earlier version, as each release adds
    // its section and keeps the others. A version that is no release version, a pre-release
    // among them, ranks below every release version, as no version heading names one.
    let mut newest: BTreeMap<&str, &MetadataPackage> = BTreeMap::new();
    for package in packages {
        let rank = |p: &MetadataPackage| Version::parse(&p.version);
        match newest.get(package.name.as_str()) {
            Some(held) if rank(held) >= rank(package) => {}
            _ => {
                newest.insert(&package.name, package);
            }
        }
    }
    let absent = |name: &str| {
        format!(
            "`{name}` is no package of the project's resolved dependency graph: name a package \
             that `cargo metadata` lists in the project, as its `Cargo.toml` depends on it"
        )
    };
    let first = *newest.get(named).ok_or_else(|| absent(named))?;
    let mut out = vec![first];
    let mut take = |package: &'m MetadataPackage| {
        if !out.iter().any(|p| p.name == package.name) {
            out.push(package);
        }
    };
    if !extra.is_empty() {
        let mut names: Vec<&str> = extra.iter().map(String::as_str).collect();
        names.sort_unstable();
        for name in names {
            take(newest.get(name).ok_or_else(|| absent(name))?);
        }
    } else if let Some(repository) = &first.repository {
        // `newest` iterates by package name, which is the order of the output.
        for package in newest.values() {
            if package.repository.as_ref() == Some(repository) {
                take(package);
            }
        }
    }
    Ok(out)
}

/// One selected package's changelog, as the thin layer read it.
struct Read {
    /// The package name, which the layout names the crate by.
    package: String,
    /// The path of its `CHANGELOG.md`, which a line the layout cannot hold names.
    file: PathBuf,
    /// The file's text, or why it could not be read.
    text: Result<String, String>,
}

/// What [`assemble`] gives the output.
struct Assembled {
    /// The layout, with no generated header.
    body: String,
    /// One line per thing the layout cannot hold, in the order of the crates, then of the file.
    unheld: Vec<String>,
    /// The entries of the sections of the range, printed or not.
    in_range: usize,
}

/// The layout of `crates` over `range`, with no generated header, one line per thing it cannot
/// hold, in the order of the crates, then of the file, and the count of the range's entries.
///
/// Each crate's entries are placed by the table its own preamble states, as its publisher's check
/// judged them: an entry its preamble does not admit is reported, whatever another crate's
/// preamble lists, and a version its preamble lists as unclassed has no category level for it
/// alone. The layout's categories are every category of the preambles, in the order of first
/// appearance in the order of the crates. Each entry of a section of the range is thereby either
/// printed or reported, never both and never neither, since both read its crate's table through
/// `unplaced`'s rule.
///
/// Every unread line and every preamble line of no shape is reported, whatever section it lies
/// in: a version heading the parse does not take, such as one followed by a date, is an unread
/// line, and its entries sit in the section above it, so an unread line outside the range can
/// stand for entries inside it. A file holding a carriage return that no line feed follows is
/// reported and not read: the parse ends a line at a line feed alone, so such a file would read
/// as one line of its preamble, and its entries would be dropped with no report.
fn assemble(range: &VersionRange, crates: &[Read]) -> Assembled {
    // One list per crate, so the lines stay in the order of the crates, then of the file.
    let mut unheld: Vec<Vec<String>> = vec![Vec::new(); crates.len()];
    let mut read_logs: Vec<(usize, ChangelogTable, Changelog)> = Vec::new();
    for (i, read) in crates.iter().enumerate() {
        let file = read.file.display();
        let text = match &read.text {
            Ok(text) => text,
            Err(why) => {
                unheld[i].push(format!("{file}: cannot be read: {why}"));
                continue;
            }
        };
        if text.replace("\r\n", "").contains('\r') {
            unheld[i].push(format!(
                "{file}: not read: it holds a carriage return that no line feed follows, and a \
                 line ends at a line feed alone"
            ));
            continue;
        }
        let log = parse(text);
        let preamble = read_preamble(&log.preamble);
        for line in preamble.unread {
            unheld[i].push(format!(
                "{file}:{line}: a line of the preamble of no shape the read takes"
            ));
        }
        read_logs.push((i, preamble.table, log));
    }
    let mut categories: Vec<String> = Vec::new();
    for (_, table, _) in &read_logs {
        for category in table.categories() {
            if !categories.contains(category) {
                categories.push(category.clone());
            }
        }
    }
    let mut ranged: Vec<(&str, ChangelogTable, Changelog)> = Vec::new();
    let mut in_range = 0;
    for (i, table, log) in read_logs {
        let read = &crates[i];
        let (package, file) = (read.package.as_str(), read.file.display());
        let mut lines: Vec<(u32, String)> = log
            .unread
            .iter()
            .map(|unread| {
                (
                    unread.line,
                    format!("{file}:{}: a line the parse could not read", unread.line),
                )
            })
            .collect();
        let sections: Vec<_> = log
            .sections
            .into_iter()
            .filter(|s| range.contains(s.heading))
            .collect();
        for section in &sections {
            in_range += section.entries.len();
            for line in unplaced(&table, package, section) {
                lines.push((
                    line,
                    format!(
                        "{file}:{line}: an entry with no category its preamble lists, or with no \
                         tag in a classed version"
                    ),
                ));
            }
        }
        lines.sort_by_key(|(line, _)| *line);
        unheld[i].extend(lines.into_iter().map(|(_, text)| text));
        ranged.push((
            package,
            table,
            Changelog {
                sections,
                ..Changelog::default()
            },
        ));
    }
    // The assembled layout with no generated header, since the output is no file; one selected
    // crate is printed in that layout too, since `design@model@assembled-changelog-from-two-crates`
    // decides when a project's file is assembled, not what this command prints. The categories
    // are the union of the preambles' in the order of first appearance in the output's crate
    // order: no file a consumer receives states the publisher's order of the categories across
    // its crates, and the named package's own order comes first.
    let crates: Vec<(&str, &str, &ChangelogTable, &Changelog)> =
        ranged.iter().map(|(p, t, l)| (*p, *p, t, l)).collect();
    Assembled {
        body: assembled_body_by_table(&categories, &crates),
        unheld: unheld.concat(),
        in_range,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(text: &str) -> Version {
        Version::parse(text).expect("a version")
    }

    fn package(name: &str, version: &str, repository: Option<&str>) -> MetadataPackage {
        MetadataPackage {
            id: format!("{name}@{version}"),
            name: name.to_string(),
            version: version.to_string(),
            manifest_path: PathBuf::from(format!("/registry/{name}-{version}/Cargo.toml")),
            repository: repository.map(str::to_string),
        }
    }

    const R: Option<&str> = Some("https://example.org/ka");

    fn graph() -> Vec<MetadataPackage> {
        vec![
            package("consumer", "0.1.0", None),
            package("ka-mutate", "0.2.0", R),
            package("knowledge-architect", "0.2.0", R),
            package("other", "1.0.0", Some("https://example.org/other")),
            package("ka-gates", "0.2.0", R),
            package("knowledge-architect", "0.1.0", R),
            package("lone", "0.3.0", None),
        ]
    }

    fn names(selected: &[&MetadataPackage]) -> Vec<String> {
        selected
            .iter()
            .map(|p| format!("{}@{}", p.name, p.version))
            .collect()
    }

    /// Claim 1 of slice 4, the selection, with the order of the output and the newest version of a
    /// package the graph holds at several: every package of the named package's `repository`, the
    /// named one first, then by package name; the named package alone where it sets none; the
    /// named set where `--package` is given; and a name the graph does not hold refused.
    /// Mutation checked: in `select`, `if package.repository.as_ref() == Some(repository) {` made
    /// `if true {`; caught.
    #[test]
    fn the_selection_is_the_repository_s_packages_or_the_named_set() {
        let graph = graph();
        let by_repository = select(&graph, "knowledge-architect", &[]).unwrap();
        assert_eq!(
            names(&by_repository),
            [
                "knowledge-architect@0.2.0",
                "ka-gates@0.2.0",
                "ka-mutate@0.2.0"
            ]
        );
        assert_eq!(names(&select(&graph, "lone", &[]).unwrap()), ["lone@0.3.0"]);
        let named = select(
            &graph,
            "lone",
            &["other".to_string(), "consumer".to_string()],
        )
        .unwrap();
        assert_eq!(
            names(&named),
            ["lone@0.3.0", "consumer@0.1.0", "other@1.0.0"]
        );
        let named = select(
            &graph,
            "knowledge-architect",
            &["ka-gates".to_string(), "knowledge-architect".to_string()],
        )
        .unwrap();
        assert_eq!(
            names(&named),
            ["knowledge-architect@0.2.0", "ka-gates@0.2.0"]
        );
        for (named, extra) in [("absent", vec![]), ("lone", vec!["absent".to_string()])] {
            let refusal = select(&graph, named, &extra).expect_err("refused");
            assert!(refusal.contains("`absent` is no package"), "{refusal}");
        }
    }

    /// The claim, the newest version is the release version that compares highest, whatever the
    /// order of the graph, and a pre-release ranks below every release version. Mutation checked:
    /// in `select`, `Some(held) if rank(held) >= rank(package) => {}` made `Some(_) => {}`, which
    /// keeps the first met; caught.
    #[test]
    fn a_package_at_several_versions_is_read_in_its_newest() {
        for order in [["0.10.0", "0.9.0"], ["0.9.0", "0.10.0"]] {
            let graph: Vec<_> = order.iter().map(|v| package("p", v, None)).collect();
            assert_eq!(names(&select(&graph, "p", &[]).unwrap()), ["p@0.10.0"]);
        }
        let graph = vec![
            package("p", "1.0.0-rc.1", None),
            package("p", "0.9.0", None),
        ];
        assert_eq!(names(&select(&graph, "p", &[]).unwrap()), ["p@0.9.0"]);
    }

    /// The claim: the range holds the versions above its start up to its end included, every one
    /// above its start where it has no end, and never `Next release`; an end not above the start
    /// is refused. Mutation checked: in `VersionRange::contains`, `v > self.from` made
    /// `v >= self.from`; caught.
    #[test]
    fn the_range_is_open_below_and_closed_above() {
        let range = VersionRange::new(v("0.1.0"), Some(v("0.3.0"))).unwrap();
        let held = |r: &VersionRange, t: &str| r.contains(Heading::Version(v(t)));
        assert!(!held(&range, "0.1.0"));
        assert!(held(&range, "0.2.0"));
        assert!(held(&range, "0.3.0"));
        assert!(!held(&range, "0.3.1"));
        assert!(!range.contains(Heading::NextRelease));
        let open = VersionRange::new(v("0.1.0"), None).unwrap();
        assert!(held(&open, "9.0.0"));
        assert!(!held(&open, "0.1.0"));
        assert!(!open.contains(Heading::NextRelease));
        for to in ["0.1.0", "0.0.9"] {
            assert!(VersionRange::new(v("0.1.0"), Some(v(to))).is_err(), "{to}");
        }
    }

    fn preamble(package: &str) -> String {
        format!(
            "# Changelog of {package}\n\nThis file holds the entries of this crate only.\n\n\
             Categories, in order: Migration, New features."
        )
    }

    fn read(package: &str, body: &str) -> Read {
        Read {
            package: package.to_string(),
            file: PathBuf::from(format!("/r/{package}/CHANGELOG.md")),
            text: Ok(format!("{}\n\n{body}", preamble(package))),
        }
    }

    /// Claim 2 of slice 4, the range in the layout: the sections of the range alone, `Next
    /// release` excluded, each crate under its category in the order given, with no generated
    /// header. Mutation checked: in `assemble`, `.filter(|s| range.contains(s.heading))` made
    /// `.filter(|_| true)`; caught.
    #[test]
    fn the_layout_holds_the_range_s_sections_alone() {
        let a = read(
            "a",
            "## Next release\n\n- Migration, major: not yet.\n\n## 0.3.0\n\n- Migration, minor: a three.\n\n\
             ## 0.2.0\n\n- New features, minor: a two.\n\n## 0.1.0\n\n- Migration, minor: a one.\n",
        );
        let b = read(
            "b",
            "## Next release\n\n## 0.3.0\n\n- Migration, patch: b three.\n",
        );
        let range = VersionRange::new(v("0.1.0"), Some(v("0.3.0"))).unwrap();
        let Assembled { body, unheld, .. } = assemble(&range, &[a, b]);
        assert_eq!(
            body,
            "\n## 0.3.0\n\n### Migration\n\n#### a\n\n- minor: a three.\n\n#### b\n\n- patch: b three.\n\
             \n## 0.2.0\n\n### New features\n\n#### a\n\n- minor: a two.\n"
        );
        assert_eq!(unheld, Vec::<String>::new());
    }

    /// Claim 5 of slice 4, what the layout cannot hold is reported, never dropped: a file that
    /// cannot be read, a line of a preamble the read does not take, every unread line whatever
    /// its section, and, in a section of the range, an entry with no place, each with its file and
    /// its line; an entry with no place below the range is not reported. Mutation checked: in
    /// `assemble`, the unread lines filtered to the sections of the range, by
    /// `.filter(|u| log.sections.iter().rfind(|s| s.line < u.line).is_some_and(|s| range.contains(s.heading)))`
    /// added before `.map(|unread| {`; caught.
    #[test]
    fn what_the_layout_cannot_hold_is_reported() {
        let mut a = read(
            "a",
            "## 0.3.0\n\n- Migration, minor: kept.\n- Workflow, patch: no such category.\nstray text\n\n\
             ## 0.1.0\n\nstray text below the range\n- Workflow, patch: below the range.\n",
        );
        if let Ok(text) = &mut a.text {
            *text = text.replace("only.\n", "only.\nA line of no shape.\n");
        }
        let missing = Read {
            package: "m".to_string(),
            file: PathBuf::from("/r/m/CHANGELOG.md"),
            text: Err("No such file or directory (os error 2)".to_string()),
        };
        let range = VersionRange::new(v("0.1.0"), None).unwrap();
        let Assembled { body, unheld, .. } = assemble(&range, &[a, missing]);
        assert_eq!(
            body,
            "\n## 0.3.0\n\n### Migration\n\n#### a\n\n- minor: kept.\n"
        );
        assert_eq!(
            unheld,
            [
                "/r/a/CHANGELOG.md:4: a line of the preamble of no shape the read takes",
                "/r/a/CHANGELOG.md:11: an entry with no category its preamble lists, or with no \
                 tag in a classed version",
                "/r/a/CHANGELOG.md:12: a line the parse could not read",
                "/r/a/CHANGELOG.md:16: a line the parse could not read",
                "/r/m/CHANGELOG.md: cannot be read: No such file or directory (os error 2)",
            ]
        );
    }

    /// Claim 5, a version heading the parse does not take: under a bounded `--to`, its entries
    /// sit in the section above it, outside the range, and the heading is reported as an unread
    /// line. Mutation checked: the filter of the unread lines to the sections of the range, as for
    /// `what_the_layout_cannot_hold_is_reported`; caught.
    #[test]
    fn a_misspelled_version_heading_under_a_bounded_range_is_reported() {
        let a = read(
            "a",
            "## 0.4.0\n\n- Migration, minor: a four.\n\n## 0.3.0 (2026-01-01)\n\n\
             - Migration, major: a three.\n\n## 0.2.0\n\n- Migration, minor: a two.\n",
        );
        let range = VersionRange::new(v("0.2.0"), Some(v("0.3.0"))).unwrap();
        let Assembled { body, unheld, .. } = assemble(&range, &[a]);
        assert_eq!(body, "");
        assert_eq!(
            unheld,
            ["/r/a/CHANGELOG.md:11: a line the parse could not read"]
        );
    }

    /// Claim 1 with claim 5, each crate placed by its own preamble: an entry of a category only
    /// another selected crate's preamble lists is reported, not printed, and a version another
    /// crate's preamble lists as unclassed keeps its category level for this one; the categories
    /// follow the order of first appearance in the order of the crates. Mutations checked, each
    /// caught: in `assemble`, the reports made under the merged categories,
    /// `for line in unplaced(&table, package, section) {` made
    /// `for line in unplaced(&read_preamble(&format!("Categories, in order: {}.", categories.join(", "))).table, package, section) {`;
    /// and the first crate's table given to every crate's rendering,
    /// `ranged.iter().map(|(p, t, l)| (*p, *p, t, l))` made
    /// `ranged.iter().map(|(p, _, l)| (*p, *p, &ranged[0].1, l))`.
    #[test]
    fn each_crate_is_placed_by_its_own_preamble() {
        let b = Read {
            package: "b".to_string(),
            file: PathBuf::from("/r/b/CHANGELOG.md"),
            text: Ok("# Changelog of b\n\nCategories, in order: Breaking.\n\n\
                      ## 0.2.0\n\n- Breaking, major: b breaks.\n- Migration, minor: b lists no Migration.\n"
                .to_string()),
        };
        let a = Read {
            package: "a".to_string(),
            file: PathBuf::from("/r/a/CHANGELOG.md"),
            text: Ok("# Changelog of a\n\nCategories, in order: Migration.\n\n\
                      Versions with no category and no class: 0.2.0.\n\n## 0.2.0\n\n- a unclassed\n"
                .to_string()),
        };
        let range = VersionRange::new(v("0.1.0"), None).unwrap();
        let Assembled { body, unheld, .. } = assemble(&range, &[b, a]);
        assert_eq!(
            body,
            "\n## 0.2.0\n\n#### a\n\n- a unclassed\n\n### Breaking\n\n#### b\n\n- major: b breaks.\n"
        );
        assert_eq!(
            unheld,
            ["/r/b/CHANGELOG.md:8: an entry with no category its preamble lists, or with no tag in a \
              classed version"]
        );
    }

    /// The claim: the text printed is the layout with no opening blank line, or, where it is
    /// empty, a line saying the range holds no entry, or that no entry of it has a place in the
    /// layout; the code is 0 with nothing it cannot hold, else the lines it cannot hold follow a
    /// blank line and a heading, and the code is 1. Mutations checked, each caught: in `output`,
    /// `return (text, 0);` made `return (text, 1);`; `(true, 0, true) =>` made `(true, _, _) =>`;
    /// and made `(true, 0, _) =>`, which claims an empty range over a file not read.
    #[test]
    fn the_output_is_the_layout_then_what_it_cannot_hold() {
        let made = |body: &str, unheld: &[&str], in_range: usize| Assembled {
            body: body.to_string(),
            unheld: unheld.iter().map(|l| l.to_string()).collect(),
            in_range,
        };
        let body = "\n## 0.2.0\n\n#### a\n\n- one\n";
        assert_eq!(
            output(&made(body, &[], 1)),
            ("## 0.2.0\n\n#### a\n\n- one".to_string(), 0)
        );
        assert_eq!(
            output(&made("", &[], 0)),
            (
                "No entry of the selected changelogs is in the range.".to_string(),
                0
            )
        );
        assert_eq!(
            output(&made("", &["/r/a:9: x"], 1)),
            (
                "Nothing of the range is printed; what the layout cannot hold is listed below.\n\n\
                 Not printed, since the layout has no place for it:\n/r/a:9: x"
                    .to_string(),
                1
            )
        );
        // A file not read holds entries `in_range` does not count.
        assert_eq!(
            output(&made("", &["/r/a: not read"], 0)).0,
            "Nothing of the range is printed; what the layout cannot hold is listed below.\n\n\
             Not printed, since the layout has no place for it:\n/r/a: not read"
        );
        assert_eq!(
            output(&made(body, &["/r/a:3: x", "/r/b: y"], 1)),
            (
                "## 0.2.0\n\n#### a\n\n- one\n\nNot printed, since the layout has no place for \
                 it:\n/r/a:3: x\n/r/b: y"
                    .to_string(),
                1
            )
        );
    }

    /// The claim: the categories of the layout are every category of the crates' preambles, in
    /// the order of first appearance in the order of the crates, two crates listing them in
    /// different orders and each printing under several; and the count of the range's entries is
    /// every entry of its sections. Mutations checked, each caught: in `assemble`,
    /// `for (_, table, _) in &read_logs {` made `for (_, table, _) in read_logs.iter().rev() {`, and
    /// made `for (_, table, _) in read_logs.iter().take(1) {`.
    #[test]
    fn the_categories_are_the_union_in_the_order_of_first_appearance() {
        let crate_of = |package: &str, categories: &str, body: &str| Read {
            package: package.to_string(),
            file: PathBuf::from(format!("/r/{package}/CHANGELOG.md")),
            text: Ok(format!(
                "# Changelog of {package}\n\nCategories, in order: {categories}.\n\n{body}"
            )),
        };
        let a = crate_of(
            "a",
            "Migration, New features",
            "## 0.2.0\n\n- New features, minor: a adds.\n- Migration, minor: a moves.\n",
        );
        let b = crate_of(
            "b",
            "Workflow, New features, Migration",
            "## 0.2.0\n\n- Workflow, patch: b works.\n- Migration, patch: b moves.\n",
        );
        let range = VersionRange::new(v("0.1.0"), None).unwrap();
        let assembled = assemble(&range, &[a, b]);
        assert_eq!(
            assembled.body,
            "\n## 0.2.0\n\n### Migration\n\n#### a\n\n- minor: a moves.\n\n#### b\n\n- patch: b moves.\n\
             \n### New features\n\n#### a\n\n- minor: a adds.\n\n### Workflow\n\n#### b\n\n- patch: b works.\n"
        );
        assert_eq!(assembled.unheld, Vec::<String>::new());
        assert_eq!(assembled.in_range, 4);
    }

    /// The claim: a file holding a carriage return that no line feed follows is reported and not
    /// read, one that ends its lines with it and one holding a single one in a file of LF line
    /// endings, where a file of CRLF line endings is read. Mutations checked, each caught: in
    /// `assemble`, `if text.replace("\r\n", "").contains('\r') {` made `if false {`, and made
    /// `.contains("\r\r")`.
    #[test]
    fn a_lone_carriage_return_is_reported_and_the_file_not_read() {
        let lone = Read {
            package: "a".to_string(),
            file: PathBuf::from("/r/a/CHANGELOG.md"),
            text: Ok("# Changelog of a\r\rCategories, in order: Migration.\r\r\
                      ## 0.2.0\r\r- Migration, major: a two.\r"
                .to_string()),
        };
        let crlf = Read {
            package: "b".to_string(),
            file: PathBuf::from("/r/b/CHANGELOG.md"),
            text: Ok(
                "# Changelog of b\r\n\r\nCategories, in order: Migration.\r\n\r\n\
                      ## 0.2.0\r\n\r\n- Migration, major: b two.\r\n"
                    .to_string(),
            ),
        };
        let inside = Read {
            package: "c".to_string(),
            file: PathBuf::from("/r/c/CHANGELOG.md"),
            text: Ok("# Changelog of c\n\nCategories, in order: Migration.\n\n\
                      ## 0.2.0\n\n- Migration, major: c two.\rmore\n"
                .to_string()),
        };
        let range = VersionRange::new(v("0.1.0"), None).unwrap();
        let assembled = assemble(&range, &[lone, crlf, inside]);
        assert_eq!(
            assembled.body,
            "\n## 0.2.0\n\n### Migration\n\n#### b\n\n- major: b two.\n"
        );
        assert_eq!(
            assembled.unheld,
            [
                "/r/a/CHANGELOG.md: not read: it holds a carriage return that no line feed \
                 follows, and a line ends at a line feed alone",
                "/r/c/CHANGELOG.md: not read: it holds a carriage return that no line feed \
                 follows, and a line ends at a line feed alone"
            ]
        );
    }
}
