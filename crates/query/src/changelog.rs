//! `changelog`: the changelogs of a project's dependencies, from the sources cargo fetched, over a
//! range of versions, in the layout of an assembled changelog with no generated header.
//!
//! The selection, the range, the order of the crates and what the output cannot hold are pure,
//! over what `cargo metadata` reports and the texts of the files; the run of Cargo, the reads of
//! the files and the printing are the thin layer of [`changelog`].

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::cargo_reader::{run_metadata, MetadataPackage};
use crate::model::changelog::{
    assembled_body, parse, read_preamble, unplaced, Changelog, Heading, Version, CHANGELOG_FILE,
};
use crate::model::declared::ChangelogTable;
use crate::output::outln;
use crate::Manifest;

/// The versions a run prints: those above `from`, up to `to` included where it is given, and
/// every one above `from` where it is not. `Next release` is in no range, since it is no
/// version a pin can cross.
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
/// It runs `cargo metadata --format-version 1 --locked --offline` at the manifest's root, and
/// reads nothing else of the project, so it writes no file, `Cargo.lock` included, and reaches no
/// network, as `path@cargo-reader@docs/grounding/cargo-metadata.md` measures the two flags. Of the
/// resolved graph it selects, each package in its newest version where the graph holds several:
/// with `request.packages` empty, every package whose `repository` equals the named package's, or
/// the named package alone where it sets none; otherwise the named package and those. It reads
/// the `CHANGELOG.md` beside each one's `Cargo.toml`, takes each one's categories and unclassed
/// versions from its preamble, and prints the sections of the range in the layout of an
/// assembled changelog with no generated header, the named package first under each category,
/// then the others by package name. A version none of them holds an entry of is not printed.
///
/// After the layout, it prints one line per thing the layout cannot hold, naming the file and,
/// but for a file it cannot read, the line: a selected package with no readable `CHANGELOG.md`, a
/// line of a preamble of no shape the read takes, and, in a section of the range, a line the
/// parse could not read and an entry the layout has no place for. A section outside the range is
/// not judged.
///
/// The output is for a reader, not a contract, per
/// `issue@knowledge-architect@command-output-is-not-declared-a-contract`. Exit 0 when it printed
/// with nothing it could not hold, 1 when it printed something it could not hold, per
/// `design@cli@exit-code-ladder`.
///
/// # Errors
///
/// An `Err`, the could-not-run exit 2, with nothing printed: Cargo's failure, with its message
/// and the repair of running `cargo fetch` after the pin's edit, as a lock file that a pin's edit
/// left stale fails under `--locked`; and a named package or a `request.packages` name that the
/// resolved graph does not hold.
pub fn changelog(manifest: &Manifest, request: &ChangelogRequest) -> Result<ExitCode, String> {
    let metadata = run_metadata(manifest.root(), &["--locked", "--offline"]).map_err(|e| {
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
    let (body, unheld) = assemble(&request.range, &read);
    // The body opens each version with a blank line, as the assembled file holds it below its
    // header; printed alone, that line would open the output.
    let body = body.strip_prefix('\n').unwrap_or(&body);
    if body.is_empty() {
        outln!("No entry of the selected changelogs is in the range.");
    } else {
        outln!("{}", body.trim_end_matches('\n'));
    }
    if unheld.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    outln!();
    outln!("Not printed, since the layout has no place for it:");
    for line in &unheld {
        outln!("{line}");
    }
    Ok(ExitCode::from(1))
}

/// The packages of the graph `packages` that a request selects, in the order of the output: the
/// named package first, then the others by package name, each once, in its newest version.
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

/// The layout of `crates` over `range`, with no generated header, and one line per thing it
/// cannot hold, in the order of the crates, then of the file.
///
/// The crates are rendered under one table, every category and unclassed version of their
/// preambles merged in the order of the crates: the crates of one project state one table, since
/// the publisher's check gives each the preamble of one manifest, and crates of two projects,
/// which `--package` can select together, each keep their categories. Each entry of a section of
/// the range is thereby either printed or reported, never both and never neither, since both
/// read the one table through `unplaced`'s rule.
fn assemble(range: &VersionRange, crates: &[Read]) -> (String, Vec<String>) {
    // One list per crate, so the lines stay in the order of the crates, then of the file.
    let mut unheld: Vec<Vec<String>> = vec![Vec::new(); crates.len()];
    let mut logs: Vec<(usize, Changelog)> = Vec::new();
    let mut tables: Vec<ChangelogTable> = Vec::new();
    for (i, read) in crates.iter().enumerate() {
        let file = read.file.display();
        let text = match &read.text {
            Ok(text) => text,
            Err(why) => {
                unheld[i].push(format!("{file}: cannot be read: {why}"));
                continue;
            }
        };
        let log = parse(text);
        let preamble = read_preamble(&log.preamble);
        for line in preamble.unread {
            unheld[i].push(format!(
                "{file}:{line}: a line of the preamble of no shape the read takes"
            ));
        }
        tables.push(preamble.table);
        logs.push((i, log));
    }
    let table = ChangelogTable::merged(&tables);
    let mut ranged: Vec<(&str, Changelog)> = Vec::new();
    for (i, log) in logs {
        let read = &crates[i];
        let (package, file) = (read.package.as_str(), read.file.display());
        let mut lines: Vec<(u32, String)> = Vec::new();
        for unread in &log.unread {
            // An unread line lies below the first version heading, so in the section of the last
            // heading above it.
            let section = log.sections.iter().rfind(|s| s.line < unread.line);
            if section.is_some_and(|s| range.contains(s.heading)) {
                lines.push((
                    unread.line,
                    format!("{file}:{}: a line the parse could not read", unread.line),
                ));
            }
        }
        let sections: Vec<_> = log
            .sections
            .into_iter()
            .filter(|s| range.contains(s.heading))
            .collect();
        for section in &sections {
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
            Changelog {
                sections,
                ..Changelog::default()
            },
        ));
    }
    let crates: Vec<(&str, &str, &Changelog)> = ranged.iter().map(|(p, l)| (*p, *p, l)).collect();
    (assembled_body(&table, &crates), unheld.concat())
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
        let (body, unheld) = assemble(&range, &[a, b]);
        assert_eq!(
            body,
            "\n## 0.3.0\n\n### Migration\n\n#### a\n\n- minor: a three.\n\n#### b\n\n- patch: b three.\n\
             \n## 0.2.0\n\n### New features\n\n#### a\n\n- minor: a two.\n"
        );
        assert_eq!(unheld, Vec::<String>::new());
    }

    /// Claim 5 of slice 4, what the layout cannot hold is reported, never dropped: a file that
    /// cannot be read, a line of a preamble the read does not take, and, in a section of the
    /// range, an unread line and an entry with no place, each with its file and its line; and
    /// nothing of a section outside the range. Mutation checked: in `assemble`,
    /// `if section.is_some_and(|s| range.contains(s.heading)) {` made `if section.is_some() {`;
    /// caught.
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
        let (body, unheld) = assemble(&range, &[a, missing]);
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
                "/r/m/CHANGELOG.md: cannot be read: No such file or directory (os error 2)",
            ]
        );
    }
}
