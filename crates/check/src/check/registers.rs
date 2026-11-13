//! Every anchor carries the registers it owes, in the shape that register has.
//!
//! An anchor is a component or a location. A **component** carries its README, its scoped
//! `CLAUDE.md` while the project serves the `claude` harness, and every component-scoped
//! register, `rejected` among them; a
//! **location** carries the registers it declares and nothing else. What that buys is one home
//! per anchor for each kind of statement, and one place to look for it.
//!
//! **A missing home is a finding rather than an absence.** A component with no design home has
//! its design recorded wherever the last session happened to put it, and a component with no
//! issue directory has what is open about it in no report.
//!
//! **Three register shapes, and this module asserts each.** A heading register has the two homes
//! of `design@check@heading-register-two-shapes`: the single file, or the directory whose `README.md`
//! links every subdocument. A file register has one home, a directory holding one file per
//! entry beside a hand-written `README.md`, a generated `index.md` and an optional
//! `register.toml`; the entries' frontmatter, title, sections and owed subsections are asserted
//! here, per `design@check@a-file-register-is-a-directory-of-entries` and
//! `design@model@frontmatter-subset`. A Directory register owes the same two navigation files, and
//! each of its entries is a directory whose README is asserted as a File entry is. So do the plan
//! types' homes, specs/ and milestones/: each plan document is a directory whose `README.md`
//! owes the sections of its kind and whose `deliberation.md` owes the Threads and the Arguments,
//! per `design@check@plan-document-sections-by-kind`.
//!
//! **Whether what is there exists is not this check's question.** An anchor or a home that is
//! not there is a finding of phase 2, and a definition sitting where none may is
//! the entity table's, phase 3; both stop the run before this check reads anything. What is
//! asserted here is the shape of what the earlier phases found present.

use crate::model::MANIFEST_NAME;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::model::declared::{Register, Shape};
use crate::model::document::Observation;
use crate::model::table::{Anchor, Anchors, Home};
use crate::model::Finding;
use crate::model::Manifest;
use crate::model::{Document, Model};

use super::Inputs;

/// What the check looked at.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Counts {
    pub components: usize,
    pub locations: usize,
    /// Anchor-and-register pairs whose home was asserted.
    pub instances: usize,
    /// File-register entries whose shape was asserted.
    pub entries: usize,
}

/// The per-instance options, the only file a register instance carries beside its entries.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegisterConfig {
    groups: Vec<String>,
}

pub(crate) fn check(model: &Model, manifest: &Manifest, inputs: &Inputs) -> (Vec<Finding>, Counts) {
    let anchors = Anchors::of(manifest, inputs.present);
    check_under(model, manifest, inputs, &anchors)
}

/// The same, over a stated anchor list, for a test that needs anchors no manifest declares.
pub(crate) fn check_under(
    model: &Model,
    manifest: &Manifest,
    inputs: &Inputs,
    anchors: &Anchors,
) -> (Vec<Finding>, Counts) {
    let mut out: Vec<Finding> = Vec::new();
    let mut counts = Counts {
        components: anchors.all().iter().filter(|a| a.is_component).count(),
        locations: manifest.locations().len(),
        ..Counts::default()
    };
    let index = super::generated::index_action(manifest, inputs.judged);

    // Whether an anchor and its homes EXIST is phase 2's question, judged before this
    // phase runs; what is asserted here is the shape of what is there.
    for anchor in anchors.all() {
        for name in &anchor.registers {
            let Some(register) = anchors.registers().by_name(name) else {
                continue;
            };
            counts.instances += 1;
            let home = anchor.home_of(register);
            match register.shape {
                Shape::Heading => {
                    heading_home(&mut out, anchor, register, &home, model, anchors, inputs)
                }
                Shape::File => {
                    counts.entries += file_home(
                        &mut out, anchor, register, &home, model, anchors, inputs, &index,
                    );
                }
                Shape::Directory => {
                    counts.entries +=
                        directory_home(&mut out, register, &home, model, anchors, inputs, &index);
                }
                // An item is a heading of a plan document, judged where it is defined, in the
                // entity table; its home is the plan's documents, which the plan homes that hold
                // them judge.
                Shape::Section => {}
                Shape::Plan(_) => {
                    counts.entries += plan_home(
                        &mut out, anchor, register, &home, model, anchors, inputs, &index,
                    );
                }
            }
        }
    }

    (out, counts)
}

/// A heading register: exactly one home shape, and a directory home naming every subdocument.
#[allow(clippy::too_many_arguments)]
fn heading_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    model: &Model,
    anchors: &Anchors,
    inputs: &Inputs,
) {
    // A heading register has no per-instance option, so a file declaring one is a grouping
    // nothing reads. Left silent it looks like configuration that took effect.
    if inputs.present.contains(&home.config) {
        out.push(Finding::in_file(
            &home.config,
            format!(
                "`{}` sits beside the {} register, which is a heading register",
                home.config.display(),
                register.name
            ),
            "delete it; per-instance options belong to a file register, and a heading \
             register has none",
        ));
    }
    // The directory shape with its head present is the one shape with something to assert
    // beyond existence; phase 2 has already reported a missing head.
    let dir_home = inputs.directories.contains(&home.dir);
    let headed =
        inputs.present.contains(&home.readme) && !inputs.directories.contains(&home.readme);
    if dir_home && headed {
        {
            let linked = links(model, &home.readme, &home.dir);
            // Enumerated from the model rather than from the listing: a subdocument is what
            // the walk covers, so a gitignored scratch file owes no naming, a directory is
            // never one, and a document owned by an anchor nested under this directory is
            // that anchor's rather than a subdocument of this home.
            let mut subdocuments: Vec<&PathBuf> = model
                .documents()
                .iter()
                .map(|d| &d.rel)
                .filter(|p| {
                    p.starts_with(&home.dir)
                        && **p != home.readme
                        && p.extension().is_some_and(|e| e == "md")
                        && anchors.owning(p).path == anchor.path
                })
                .collect();
            subdocuments.sort();
            for subdocument in subdocuments {
                if !linked.contains(subdocument) {
                    out.push(Finding::in_file(
                        &home.readme,
                        format!(
                            "`{}` is not linked from its {} README",
                            subdocument.display(),
                            register.name
                        ),
                        "link it — [title](file.md), relative to the README — or delete the \
                         subdocument; a subdocument nobody links is a home nobody finds",
                    ));
                }
            }
        }
    }
}

/// A file register: the directory, its two navigation files, its groups, and every entry.
///
/// Returns how many entries were judged. `index` is the repair of a missing index.
#[allow(clippy::too_many_arguments)]
fn file_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    model: &Model,
    anchors: &Anchors,
    inputs: &Inputs,
    index: &str,
) -> usize {
    // A missing directory is phase 2's finding; there is no shape to assert in one.
    if !inputs.directories.contains(&home.dir) {
        return 0;
    }
    navigation(out, register, home, inputs, index);

    // `None` where the options could not be read: the declared groups are unknown, and a
    // finding that follows only from them would name a repair the file may already hold.
    let groups = config(out, register, home, inputs);
    // What sits under an anchor nested inside this home is that anchor's, as the heading
    // shape reads it: the nesting is refused by the manifest, and reading the nested
    // anchor's files as entries of this register would report the refusal's consequences
    // against the wrong register and the wrong anchor, nine times over.
    let owned = |rel: &Path| anchors.owns_entry(anchor, rel);
    directory_contents(out, register, home, groups.as_ref(), inputs, &owned);

    let mut judged = 0;
    for doc in model.documents() {
        if !is_entry(&doc.rel, home) || !owned(&doc.rel) {
            continue;
        }
        judged += 1;
        entry(out, register, &anchor.sections_of(register), doc);
    }
    judged
}

/// A Directory register: its two navigation files, and each entry's README as an entry.
///
/// Its entries are the entry anchors directly under its home. What else the home holds, and
/// which directories are entries, is phase 2's, in phase 2; what is asserted here
/// is the same shape a File home owes, over the README of each entry. Returns how many entries
/// were judged. `index` is the repair of a missing index.
fn directory_home(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    model: &Model,
    anchors: &Anchors,
    inputs: &Inputs,
    index: &str,
) -> usize {
    if !inputs.directories.contains(&home.dir) {
        return 0;
    }
    navigation(out, register, home, inputs, index);
    let mut judged = 0;
    for entry in anchors.all() {
        if !entry.is_entry_in(&register.name, &home.dir) {
            continue;
        }
        let readme = entry.path.join("README.md");
        if let Some(doc) = model.documents().iter().find(|d| d.rel == readme) {
            judged += 1;
            self::entry(out, register, &register.sections, doc);
        }
    }
    judged
}

/// A plan register's home: specs/ and milestones/ at the anchor `plans`, and a milestone's slices
/// at the milestone. Each home's two navigation files, specs/'s declared groups, and of each plan
/// document the sections of its `README.md` and of its `deliberation.md`.
///
/// What the plans layout admits, and which directories are plan documents, is phase 2's; what is
/// asserted here is the shape of what it left standing. A harvested slice is frozen and judged by
/// nothing. Returns how many plan documents were judged. `index` is the repair of a missing index.
#[allow(clippy::too_many_arguments)]
fn plan_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    model: &Model,
    anchors: &Anchors,
    inputs: &Inputs,
    index: &str,
) -> usize {
    if !inputs.directories.contains(&home.dir) {
        return 0;
    }
    navigation(out, register, home, inputs, index);
    let headed = |dir: &Path| {
        let readme = dir.join("README.md");
        inputs.present.contains(&readme) && !inputs.directories.contains(&readme)
    };
    // The plan documents this home holds, by their directories.
    let mut dirs: Vec<PathBuf> = Vec::new();
    if anchor.is_milestone() {
        for path in inputs
            .present
            .iter()
            .filter(|p| p.parent() == Some(home.dir.as_path()))
        {
            let name = path.file_name().unwrap_or_default();
            if inputs.directories.contains(path)
                && name != "harvested"
                && name != "search"
                && headed(path)
            {
                dirs.push(path.clone());
            }
        }
    } else if register.name == crate::model::declared::MILESTONE_REGISTER {
        dirs.extend(
            anchors
                .all()
                .iter()
                .filter(|a| a.is_milestone() && a.path.parent() == Some(home.dir.as_path()))
                .map(|a| a.path.clone()),
        );
    } else {
        // A directory of specs/ holding no README.md is a group, per
        // `design@check@groups-are-subdirectories`, and its headed directories are specs.
        let groups = config(out, register, home, inputs);
        let mut present: BTreeSet<String> = BTreeSet::new();
        let mut top: Vec<&PathBuf> = inputs
            .present
            .iter()
            .filter(|p| p.parent() == Some(home.dir.as_path()) && inputs.directories.contains(*p))
            .collect();
        top.sort();
        for path in top {
            if headed(path) {
                dirs.push(path.clone());
                continue;
            }
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            present.insert(name.clone());
            if groups
                .as_ref()
                .is_some_and(|groups| !groups.contains(&name))
            {
                out.push(Finding::in_file(
                    path,
                    format!(
                        "`{name}` is a subdirectory of the {} register and is no declared group",
                        register.name
                    ),
                    "declare it in the instance's register.toml, or move its specs up; a \
                     grouping nobody announced is one nobody reviewed",
                ));
            }
            dirs.extend(
                inputs
                    .present
                    .iter()
                    .filter(|p| p.parent() == Some(path.as_path()) && headed(p))
                    .cloned(),
            );
        }
        for group in groups.into_iter().flatten() {
            if !present.contains(&group) {
                out.push(Finding::in_file(
                    home.dir.join(&group),
                    format!(
                        "the {} register declares the group `{group}` and there is no directory",
                        register.name
                    ),
                    format!(
                        "move or write the group's first spec under `{}/`, or remove the group \
                         from the instance's register.toml; the declaration is the grouping, so \
                         changing it is its own decision",
                        home.dir.join(&group).display()
                    ),
                ));
            }
        }
    }
    dirs.sort();
    let sections = anchor.sections_of(register);
    let deliberation: Vec<&str> = crate::model::declared::DELIBERATION_SECTIONS.to_vec();
    let mut judged = 0;
    for dir in dirs {
        let readme = dir.join("README.md");
        if let Some(doc) = model.documents().iter().find(|d| d.rel == readme) {
            judged += 1;
            entry(out, register, &sections, doc);
            deliberation_in_readme(out, register, doc, &deliberation);
        }
        let file = dir.join(crate::model::declared::DELIBERATION_FILE);
        if let Some(doc) = model.documents().iter().find(|d| d.rel == file) {
            in_order(out, doc, register, 2, &deliberation, &headings(doc), None);
        }
    }
    judged
}

/// Each section of a plan's `README.md` that its `deliberation.md` holds, Threads or Arguments,
/// one finding per heading: a plan's `README.md` holds what was decided, and its argument lives
/// beside it, so a project that only moved a plan file to its directory's `README.md` is told to
/// split it.
fn deliberation_in_readme(
    out: &mut Vec<Finding>,
    register: &Register,
    doc: &Document,
    deliberation: &[&str],
) {
    for l in &doc.observations {
        let Observation::Heading { level: 2, text } = &l.what else {
            continue;
        };
        if !deliberation.contains(&text.as_str()) {
            continue;
        }
        let beside = doc
            .rel
            .with_file_name(crate::model::declared::DELIBERATION_FILE);
        out.push(Finding::at(
            &doc.rel,
            l.line,
            format!(
                "this {} entry's README.md holds the section `## {text}`, which its \
                 deliberation.md holds",
                register.name
            ),
            format!(
                "move the section, with its items, into `{}` beside it; a plan's README.md holds \
                 the design it decided, and its deliberation.md the threads and the arguments",
                beside.display()
            ),
        ));
    }
}

/// The headings of a document, by level and text, in order.
fn headings(doc: &Document) -> Vec<(u8, &str)> {
    doc.observations
        .iter()
        .filter_map(|l| match &l.what {
            Observation::Heading { level, text } => Some((*level, text.as_str())),
            _ => None,
        })
        .collect()
}

/// A home's hand-written README and generated index are both there, as files. `index` is the
/// repair of a missing index.
fn navigation(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    inputs: &Inputs,
    index: &str,
) {
    // That the index is THERE is a fact about the register's shape, and it is mandatory. What it
    // holds is `check::generated`'s question, which compares the committed bytes against a
    // regeneration and reports the same missing index or directory: this finding names the
    // repair that one names, so the two findings on one file never disagree.
    let readme = "write it by hand: the README is the register's hand-written head, which the \
                  generated index does not replace";
    for (path, what, repair) in [
        (&home.readme, "README.md", readme),
        (&home.index, "index.md", index),
    ] {
        let action = if inputs.directories.contains(path) {
            super::generated::directory_action(repair)
        } else if !inputs.present.contains(path) {
            repair.to_string()
        } else {
            continue;
        };
        out.push(Finding::in_file(
            path,
            format!(
                "the {} directory `{}` has no {what}",
                register.name,
                home.dir.display()
            ),
            action,
        ));
    }
}

/// The declared groups of one instance, and the verdicts on the file that declares them, per
/// `design@check@groups-are-subdirectories`. `None` where the file's bytes are not UTF-8 or
/// its text does not parse, so what it declares is unknown.
fn config(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    inputs: &Inputs,
) -> Option<BTreeSet<String>> {
    if inputs.configs_not_text.contains(&home.config) {
        out.push(Finding::in_file(
            &home.config,
            format!(
                "the {} register's options are not UTF-8 text, so the groups they declare are \
                 read by no check",
                register.name
            ),
            "re-encode the file as UTF-8",
        ));
        return None;
    }
    let Some(text) = inputs.configs.get(&home.config) else {
        return Some(BTreeSet::new());
    };
    match toml::from_str::<RegisterConfig>(text) {
        Ok(parsed) => Some(parsed.groups.into_iter().collect()),
        Err(e) => {
            out.push(Finding::in_file(
                &home.config,
                format!("the {} register's options do not parse: {e}", register.name),
                // One repair fits every cause the parser reports, a key other than `groups`, a
                // value of another type or the key missing: the file's one valid shape.
                "rewrite it as the one key `groups = [\"<group>\", ...]`, naming each group \
                 directory of the instance",
            ));
            None
        }
    }
}

/// Every subdirectory is a declared group, every declared group is a subdirectory, groups
/// nest one level, and nothing but an entry and the options file sits in the instance.
///
/// `owned` says whether a path under the home belongs to this instance's anchor; what a
/// nested anchor owns is left to it.
fn directory_contents(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    groups: Option<&BTreeSet<String>>,
    inputs: &Inputs,
    owned: &dyn Fn(&Path) -> bool,
) {
    let mut present: BTreeSet<String> = BTreeSet::new();
    let mut paths: Vec<&PathBuf> = inputs
        .present
        .iter()
        .filter(|p| p.starts_with(&home.dir) && **p != home.dir && owned(p))
        .collect();
    paths.sort();
    for path in paths {
        let inside = path.strip_prefix(&home.dir).unwrap_or(path);
        let depth = inside.components().count();
        let name = inside
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if inputs.directories.contains(path) {
            // A directory at the README's or the index's path is `navigation`'s finding, with
            // its removal as the repair; read as a group, it would get a second repair that
            // contradicts the first.
            if *path == home.readme || *path == home.index {
                continue;
            }
            if depth == 1 {
                present.insert(name.clone());
                if groups.is_some_and(|groups| !groups.contains(&name)) {
                    out.push(Finding::in_file(
                        path,
                        format!("`{name}` is a subdirectory of the {} register and is no declared group", register.name),
                        "declare it in the instance's register.toml, or move its entries up; \
                         a grouping nobody announced is one nobody reviewed",
                    ));
                }
            } else {
                out.push(Finding::in_file(
                    path,
                    format!("`{}` nests a group inside a group", path.display()),
                    "groups are one level deep; move the entries into a group of their own \
                     beside this one",
                ));
            }
            continue;
        }
        if *path == home.config {
            continue;
        }
        if path.extension().is_none_or(|e| e != "md") {
            out.push(Finding::in_file(
                path,
                format!(
                    "`{}` is not an entry of the {} register",
                    path.display(),
                    register.name
                ),
                // An entry written with another extension is read as no entry, so its repair
                // is a rename, where any other file's is a move.
                "if it is an entry, rename it to end in `.md`; otherwise move it out, since an \
                 instance holds one markdown file per entry, its README, its index and its \
                 register.toml",
            ));
        }
    }
    for group in groups.into_iter().flatten() {
        if !present.contains(group) {
            out.push(Finding::in_file(
                home.dir.join(group),
                format!(
                    "the {} register declares the group `{group}` and there is no directory",
                    register.name
                ),
                // Git lists no empty directory, so the group's first entry is what creates it.
                format!(
                    "move or write the group's first entry under `{}/`, or remove the group from \
                     the instance's register.toml; the declaration is the grouping, so changing \
                     it is its own decision",
                    home.dir.join(group).display()
                ),
            ));
        }
    }
}

/// Whether a walked document is an entry of this instance.
fn is_entry(rel: &Path, home: &Home) -> bool {
    if !rel.starts_with(&home.dir) || rel.extension().is_none_or(|e| e != "md") {
        return false;
    }
    *rel != home.readme && *rel != home.index
}

/// One entry: its frontmatter, its title, its sections and the subsections its kind owes.
///
/// `sections` are the level-two sections the entry owes at its anchor, per
/// `Anchor::sections_of`: a register's own, or a slice's `README.md`'s.
fn entry(out: &mut Vec<Finding>, register: &Register, sections: &[String], doc: &Document) {
    let declared: Vec<&str> = register.metadata.iter().map(|(k, _)| k.as_str()).collect();
    let mut values: BTreeMap<&str, &str> = BTreeMap::new();
    // A built-in register's metadata is compiled in, and a declaration of it is refused, per
    // `design@model@registers-are-declared`: only a declared register can be given a key.
    let declare = |what: &str| {
        if register.built_in {
            String::new()
        } else {
            format!(
                ", or declare {what} in {MANIFEST_NAME} under [registers.{}.metadata]",
                register.name
            )
        }
    };
    let subset = "a block opened and closed by a line holding only `---`, at the very top of the \
                  file, of `key: value` lines with scalar values, each key once";
    match (&doc.parsed.frontmatter, declared.is_empty()) {
        // Where the register declares no key, a block that parses is still refused, below, so
        // the repair that clears both is the deletion, or the declaration of its keys.
        (Some(Err(why)), true) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "the frontmatter of this {} entry does not parse: {why}",
                register.name
            ),
            match declare("its keys").as_str() {
                "" => "delete the block".to_string(),
                or_declare => format!("delete the block{or_declare}, and rewrite it as {subset}"),
            },
        )),
        (Some(Err(why)), false) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "the frontmatter of this {} entry does not parse: {why}",
                register.name
            ),
            format!("rewrite it as {subset}"),
        )),
        (Some(Ok(_)), true) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "this {} entry carries frontmatter and the register declares none",
                register.name
            ),
            format!("delete the block{}", declare("its keys")),
        )),
        // The parser reads a block with a blank line in it, or with no closing line, as none: its
        // opening `---` is a thematic break. So the repair names the shape that is read.
        (None, false) => out.push(Finding::in_file(
            &doc.rel,
            format!(
                "this {} entry carries no frontmatter and the register declares {}",
                register.name,
                declared.join(", ")
            ),
            "open the file with a block: a first line `---`, one `key: value` line per declared \
             key with no blank line between them, and a closing `---`; a block with a blank line \
             or no closing line is read as none",
        )),
        (None, true) => {}
        (Some(Ok(keys)), false) => {
            for (key, value) in keys {
                match register.metadata.iter().find(|(k, _)| k == key) {
                    None => out.push(Finding::in_file(
                        &doc.rel,
                        format!(
                            "`{key}` is no frontmatter key of the {} register",
                            register.name
                        ),
                        match declare("it").as_str() {
                            "" => format!(
                                "rename it to a declared key ({}), or delete it",
                                declared.join(", ")
                            ),
                            or_declare => format!(
                                "rename it to a declared key ({}), delete it{or_declare}",
                                declared.join(", ")
                            ),
                        },
                    )),
                    Some((_, accepted)) if !accepted.iter().any(|v| v == value) => {
                        out.push(Finding::in_file(
                            &doc.rel,
                            format!(
                                "`{key}: {value}` is no accepted value of the {} register",
                                register.name
                            ),
                            format!("the accepted values are {}", accepted.join(", ")),
                        ))
                    }
                    Some(_) => {
                        values.insert(key.as_str(), value.as_str());
                    }
                }
            }
            for (key, _) in &register.metadata {
                if !keys.iter().any(|(k, _)| k == key) {
                    out.push(Finding::in_file(
                        &doc.rel,
                        format!("this {} entry declares no `{key}`", register.name),
                        "every declared metadata key is carried by every entry, so a listing \
                         can be built without reading the bodies",
                    ));
                }
            }
        }
    }

    let headings: Vec<(u8, &str)> = doc
        .observations
        .iter()
        .filter_map(|l| match &l.what {
            Observation::Heading { level, text } => Some((*level, text.as_str())),
            _ => None,
        })
        .collect();
    match headings.first() {
        Some((1, _)) => {}
        _ => out.push(Finding::in_file(
            &doc.rel,
            format!("this {} entry opens with no level-one title", register.name),
            "the first heading is the entry's title, on one line; a listing prints it and a \
             reader reads it before the body",
        )),
    }

    in_order(
        out,
        doc,
        register,
        2,
        &sections.iter().map(String::as_str).collect::<Vec<_>>(),
        &headings,
        None,
    );
    // The subsections an issue owes sit under its last declared section, which is `Details`.
    // Only the issue register owes any, and the list its kind takes is the compiled one.
    let kind = register
        .metadata
        .first()
        .and_then(|(k, _)| values.get(k.as_str()).copied());
    let owed = register.owed_subsections(kind);
    if !owed.is_empty() {
        // Under that section and nowhere else. Read over the whole document, the three
        // subsections satisfy the assertion while sitting under the summary and leaving the
        // section that owes them empty, which is the shape a reader would call a defect.
        in_order(
            out,
            doc,
            register,
            3,
            owed,
            under_last_section(register, &headings),
            register.sections.last().map(String::as_str),
        );
    }
}

/// The headings under a register's last declared section, which is where its owed subsections
/// sit.
///
/// Empty where the section is absent, so every owed subsection is reported and the absent
/// section is reported beside them: two facts, and the reader needs both.
fn under_last_section<'a>(
    register: &Register,
    headings: &'a [(u8, &'a str)],
) -> &'a [(u8, &'a str)] {
    let Some(last) = register.sections.last() else {
        return headings;
    };
    let Some(from) = headings.iter().position(|(l, t)| *l == 2 && t == last) else {
        return &[];
    };
    let rest = &headings[from + 1..];
    let to = rest.iter().position(|(l, _)| *l <= 2).unwrap_or(rest.len());
    &rest[..to]
}

/// Every owed heading of one level, two or three, is present in the declared order, among
/// `headings`; `under` names the section that holds them, where the caller passed only its
/// headings.
fn in_order(
    out: &mut Vec<Finding>,
    doc: &Document,
    register: &Register,
    level: u8,
    owed: &[&str],
    headings: &[(u8, &str)],
    under: Option<&str>,
) {
    let what = if level == 2 {
        "level-two section"
    } else {
        "level-three subsection"
    };
    // A heading found under another section is reported as absent, since `headings` holds only
    // those under `under`, so the repair names where they go.
    let place = under.map_or(String::new(), |section| format!(", under `## {section}`"));
    let present: Vec<&str> = headings
        .iter()
        .filter(|(l, _)| *l == level)
        .map(|(_, t)| *t)
        .collect();
    let mut at = 0usize;
    for name in owed {
        match present[at..].iter().position(|t| t == name) {
            Some(i) => at += i + 1,
            None => out.push(Finding::in_file(
                &doc.rel,
                format!(
                    "this {} entry carries no {what} `{name}` after the ones before it",
                    register.name
                ),
                format!(
                    "the owed {what}s, in this order{place}: {}",
                    owed.join(", ")
                ),
            )),
        }
    }
}

/// Every path the README's markdown links resolve to.
///
/// A link's target resolves against the README's own directory, the way a renderer follows
/// it. A URL, a bare fragment and an absolute path are not index rows and are passed over; a
/// fragment on a file target is dropped before resolution. Whether each link RESOLVES is
/// `check::references`' assertion — what is asserted here is the other direction, that every
/// subdocument has a row. Backticked paths are the pointer forms of prose and do not name a
/// subdocument: the index is made of links a reader can follow.
fn links(model: &Model, readme: &PathBuf, dir: &Path) -> HashSet<PathBuf> {
    let Some(doc) = model.documents().iter().find(|d| d.rel == *readme) else {
        return HashSet::new();
    };
    let mut resolved = HashSet::new();
    for l in &doc.observations {
        let Observation::Link(target) = &l.what else {
            continue;
        };
        if target.starts_with('#') {
            continue;
        }
        let file_part = target.split('#').next().unwrap_or(target);
        if crate::model::resolve::has_scheme(file_part) || file_part.starts_with('/') {
            continue;
        }
        resolved.insert(dir.join(file_part));
    }
    resolved
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::MANIFEST_NAME;
    use std::collections::HashMap;
    use std::path::PathBuf;

    // Every fixture is written as the bytes it means: the checker reads no string literal of
    // its own source, per `design@check@checker-source-literals-are-data`.

    /// A manifest declaring `components`, against a root nothing reads.
    fn declaring(components: &str) -> Manifest {
        declaring_full(components, "", "[]", "[]", "[]", "[]")
    }

    /// The same, with every path list the manifest can declare spelled out.
    fn declaring_full(
        components: &str,
        extra: &str,
        skip_dirs: &str,
        skip_files: &str,
        exclude: &str,
        ext_files: &str,
    ) -> Manifest {
        let text = format!(
            "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\ncomponents = [{components}]\n\n\
             {extra}\n\
             [walk]\nskip-dirs = {skip_dirs}\nskip-files = {skip_files}\n\
             exclude = {exclude}\n\n\
             [ext]\nfiles = {ext_files}\n"
        );
        Manifest::parse(Path::new("/nowhere"), &text).expect("a declaration")
    }

    /// The findings against a listing of what exists.
    fn findings<S: AsRef<str>>(manifest: &Manifest, present: &[S]) -> Vec<String> {
        findings_over(manifest, present, Model::from_documents(Vec::new()), &[])
    }

    /// The same, with documents in the model and register options beside the instances.
    ///
    /// The kinds are implied by the listing's shape: a path with an entry beneath it is a
    /// directory, which is how every committable tree looks.
    fn findings_over<S: AsRef<str>>(
        manifest: &Manifest,
        present: &[S],
        model: Model,
        configs: &[(&str, &str)],
    ) -> Vec<String> {
        let committed = HashMap::new();
        let present: HashSet<PathBuf> = present.iter().map(|p| PathBuf::from(p.as_ref())).collect();
        let directories = crate::check::testing::implied_directories(&present);
        let outside = Vec::new();
        let configs: HashMap<PathBuf, String> = configs
            .iter()
            .map(|(p, t)| (PathBuf::from(p), t.to_string()))
            .collect();
        let inputs = Inputs {
            committed: &committed,
            configs: &configs,
            configs_not_text: &std::collections::HashSet::new(),
            generated_unreadable: &std::collections::HashSet::new(),
            present: &present,
            directories: &directories,
            outside: &outside,
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        // What the first phase that finds anything reports, then the shapes: a test here asserts
        // the shapes' own findings, and `model` holds the tests of the phases before them.
        let mut found: Vec<Finding> = match crate::check::foundation(&model, manifest, &inputs) {
            Ok(()) => Vec::new(),
            Err(stop) => stop.findings,
        };
        found.extend(check(&model, manifest, &inputs).0);
        found
            .iter()
            .map(|f| format!("{}  {}  → {}", f.location(), f.what, f.action))
            .collect()
    }

    /// Every path a component owes, under `dir`, with the file-shaped heading homes, the root's
    /// goals home among them, which a Component other than the root does not owe.
    fn all_of(dir: &str) -> Vec<String> {
        let at = |name: &str| {
            if dir.is_empty() {
                name.to_string()
            } else {
                format!("{dir}/{name}")
            }
        };
        let mut out: Vec<String> = declaring("")
            .required_documents()
            .iter()
            .map(|n| at(n))
            .collect();
        out.push(at("docs"));
        for name in ["design.md", "goals.md", "tripwires.md", "rejected.md"] {
            out.push(at(&format!("docs/{name}")));
        }
        out.push(at("docs/open-issues"));
        out.push(at("docs/open-issues/README.md"));
        out.push(at("docs/open-issues/index.md"));
        if dir.is_empty() {
            out.extend(
                crate::check::testing::PLANS_TREE
                    .iter()
                    .map(|p| p.to_string()),
            );
        }
        // The corpus the declaration below names, which is a declared path like any other.
        for name in ["r", "r/t", "r/v", "r/p", "r/m"] {
            out.push(name.to_string());
        }
        out
    }

    #[test]
    fn a_component_carrying_every_home_reports_nothing() {
        let manifest = declaring("");
        let present = all_of("");
        let found = findings(&manifest, &present);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn each_missing_home_is_reported_at_the_path_it_belongs_at() {
        let manifest = declaring("");
        // Every home but the issue directory, so exactly one finding is owed.
        let mut present = all_of("");
        present.retain(|p| !p.starts_with("docs/open-issues"));
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("docs/open-issues") && found[0].contains("no issue directory"),
            "{found:#?}"
        );
        // The goals home is required of the root, which carries the register `goal` alone.
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no goal home"), "{found:#?}");
        // So is the rejected home, per `design@check@rejected-register`.
        let mut present = all_of("");
        present.retain(|p| p != "docs/rejected.md");
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no rejected home"), "{found:#?}");
    }

    #[test]
    fn the_retired_file_shape_of_a_file_register_is_named_as_what_it_was() {
        // The one migration a project meets: a tracker that was a document becomes a
        // directory of entries, and the old file left beside it is read by nothing.
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("retired file shape"), "{found:#?}");
    }

    #[test]
    fn a_file_register_owes_a_readme_and_an_index_and_neither_stands_in_for_the_other() {
        let manifest = declaring("");
        for missing in ["docs/open-issues/README.md", "docs/open-issues/index.md"] {
            let mut present = all_of("");
            present.retain(|p| p != missing);
            let found = findings(&manifest, &present);
            assert_eq!(found.len(), 1, "{missing}: {found:#?}");
            assert!(found[0].starts_with(missing), "{found:#?}");
        }
    }

    #[test]
    fn a_heading_register_carries_exactly_one_home_shape() {
        let manifest = declaring("");
        // Both shapes at once: the entries have two candidate places to land.
        let mut present = all_of("");
        present.push("docs/design".to_string());
        present.push("docs/design/README.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("carries both"), "{found:#?}");
        // The directory shape alone, headed by its README: nothing is owed.
        let mut present = all_of("");
        present.retain(|p| p != "docs/tripwires.md");
        present.push("docs/tripwires".to_string());
        present.push("docs/tripwires/README.md".to_string());
        assert!(findings(&manifest, &present).is_empty());
    }

    #[test]
    fn a_register_options_file_beside_a_heading_register_is_reported() {
        // A heading register has no per-instance option, so a file declaring one took effect
        // nowhere. Silent, it reads as configuration that did.
        let manifest = declaring("");
        let mut present = all_of("");
        present.retain(|p| p != "docs/design.md");
        present.push("docs/design".to_string());
        present.push("docs/design/README.md".to_string());
        present.push("docs/design/register.toml".to_string());
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(vec![(
                PathBuf::from("docs/design/README.md"),
                "# Design\n".to_string(),
            )]),
            &[("docs/design/register.toml", "groups = []\n")],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("is a heading register"), "{found:#?}");
    }

    #[test]
    fn a_directory_home_readme_must_link_every_subdocument_of_any_heading_register() {
        // Generalised from design: the assertion is the register's, not one register's.
        let manifest = declaring("");
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/goal-one.md".to_string());
        let model = Model::from_documents(vec![
            (
                PathBuf::from("docs/goals/README.md"),
                "# Goals\n\n- [Another](other.md)\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/goal-one.md"),
                "## A goal `##goal-one`\n".to_string(),
            ),
        ]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("docs/goals/goal-one.md") && found[0].contains("goal README"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_subdocument_is_a_markdown_file_this_anchor_owns_and_nothing_else() {
        // A Rust source under the home is held by the model like any other document and is
        // no subdocument, so the README is not asked to link it. An anchor nested under the
        // home is refused at resolution and is no anchor, so no second exclusion is needed.
        let manifest = declaring("");
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/goal-one.md".to_string());
        present.push("docs/goals/gen.rs".to_string());
        let model = Model::from_documents(vec![
            (
                PathBuf::from("docs/goals/README.md"),
                "# Goals\n\n- [One](goal-one.md)\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/goal-one.md"),
                "## A goal `##goal-one`\n".to_string(),
            ),
            (
                PathBuf::from("docs/goals/gen.rs"),
                "// a generator that lives beside the home\n".to_string(),
            ),
        ]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn a_group_is_declared_in_the_instance_and_asserted_in_both_directions() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        present.push("docs/open-issues/a-group".to_string());
        present.push("docs/open-issues/a-group/an-entry.md".to_string());
        // Declared and present: silent.
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = [\"a-group\"]\n")],
        );
        assert!(found.is_empty(), "{found:#?}");
        // Present and undeclared.
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = []\n")],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("no declared group"), "{found:#?}");
        // Declared and absent.
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[(
                "docs/open-issues/register.toml",
                "groups = [\"a-group\", \"gone\"]\n",
            )],
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("`gone`"), "{found:#?}");
    }

    #[test]
    fn a_group_inside_a_group_and_a_file_of_another_suffix_are_each_reported() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        present.push("docs/open-issues/a-group".to_string());
        present.push("docs/open-issues/a-group/deeper".to_string());
        present.push("docs/open-issues/a-group/deeper/an-entry.md".to_string());
        present.push("docs/open-issues/notes.txt".to_string());
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = [\"a-group\"]\n")],
        );
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found
                .iter()
                .any(|f| f.contains("nests a group inside a group")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.contains("is not an entry of the issue register")),
            "{found:#?}"
        );
    }

    /// The claims: a register.toml that does not parse is one finding, whose action names the
    /// file's one valid shape, whatever the parser's cause; and a group directory beside it gets
    /// no finding of its own, since whether it is declared is unknown.
    #[test]
    fn a_register_options_file_that_does_not_parse_is_reported_rather_than_ignored() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        present.push("docs/open-issues/a-group".to_string());
        present.push("docs/open-issues/a-group/an-entry.md".to_string());
        for text in ["theme = [\"x\"]\n", "groups = 3\n", "\n"] {
            let found = findings_over(
                &manifest,
                &present,
                Model::from_documents(Vec::new()),
                &[("docs/open-issues/register.toml", text)],
            );
            assert_eq!(found.len(), 1, "{text:?}: {found:#?}");
            assert!(found[0].contains("do not parse"), "{text:?}: {found:#?}");
            assert!(
                found[0].contains("→ rewrite it as the one key `groups = [\"<group>\", ...]`"),
                "{text:?}: {found:#?}"
            );
        }
    }

    /// The listing and the model of one issue instance holding one entry.
    fn with_entry(body: &str) -> (Vec<String>, Model) {
        let mut present = all_of("");
        present.push("docs/open-issues/an-entry.md".to_string());
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/open-issues/an-entry.md"),
            body.to_string(),
        )]);
        (present, model)
    }

    fn entry_findings(body: &str) -> Vec<String> {
        let manifest = declaring("");
        let (present, model) = with_entry(body);
        findings_over(&manifest, &present, model, &[])
    }

    /// A well-formed issue entry, which every case below breaks in exactly one way.
    const ENTRY: &str = "---\nkind: defect\n---\n# The title\n\n## Summary\n\nWhat it is.\n\n\
                         ## Details\n\n### What\n\nOne.\n\n### Why it matters\n\nTwo.\n\n\
                         ### What would close it\n\nThree.\n";

    #[test]
    fn a_well_formed_entry_reports_nothing() {
        assert!(
            entry_findings(ENTRY).is_empty(),
            "{:#?}",
            entry_findings(ENTRY)
        );
    }

    #[test]
    fn an_entry_owes_its_declared_metadata_with_a_declared_value() {
        let no_block = entry_findings(&ENTRY.replace("---\nkind: defect\n---\n", ""));
        assert_eq!(no_block.len(), 1, "{no_block:#?}");
        assert!(
            no_block[0].contains("carries no frontmatter"),
            "{no_block:#?}"
        );
        let wrong = entry_findings(&ENTRY.replace("kind: defect", "kind: nonesuch"));
        assert_eq!(wrong.len(), 1, "{wrong:#?}");
        assert!(wrong[0].contains("no accepted value"), "{wrong:#?}");
        let extra = entry_findings(&ENTRY.replace("kind: defect", "kind: defect\ntheme: x"));
        assert_eq!(extra.len(), 1, "{extra:#?}");
        assert!(
            extra[0].contains("`theme` is no frontmatter key"),
            "{extra:#?}"
        );
        let other = entry_findings(&ENTRY.replace("kind: defect", "theme: x"));
        assert_eq!(other.len(), 2, "{other:#?}");
        assert!(
            other.iter().any(|f| f.contains("declares no `kind`")),
            "{other:#?}"
        );
    }

    #[test]
    fn an_entry_owes_a_level_one_title_its_sections_in_order_and_the_subsections_its_kind_owes() {
        let no_title = entry_findings(&ENTRY.replace("# The title", "## The title"));
        assert!(
            no_title.iter().any(|f| f.contains("no level-one title")),
            "{no_title:#?}"
        );
        let no_section = entry_findings(&ENTRY.replace("## Summary", "## Overview"));
        assert!(
            no_section
                .iter()
                .any(|f| f.contains("no level-two section `Summary`")),
            "{no_section:#?}"
        );
        // Present, and after the section that must follow it: order is asserted, not presence.
        let swapped = ENTRY.replace(
            "## Summary\n\nWhat it is.\n\n## Details",
            "## Details\n\nx\n\n## Summary",
        );
        let out_of_order = entry_findings(&swapped);
        assert!(
            out_of_order
                .iter()
                .any(|f| f.contains("no level-two section `Details`")),
            "{out_of_order:#?}"
        );
        let no_sub = entry_findings(&ENTRY.replace("### Why it matters", "### Why"));
        assert!(
            no_sub
                .iter()
                .any(|f| f.contains("no level-three subsection `Why it matters`")),
            "{no_sub:#?}"
        );
    }

    #[test]
    fn the_owed_subsections_are_asserted_under_the_last_declared_section_and_nowhere_else() {
        // All three present, all three under the summary, and the section that owes them
        // left empty. Read over the whole document the entry passes, which is the shape a
        // reader would call a defect.
        let misplaced = "---\nkind: defect\n---\n# The title\n\n## Summary\n\n\
                         ### What\n\nOne.\n\n### Why it matters\n\nTwo.\n\n\
                         ### What would close it\n\nThree.\n\n## Details\n\nNothing here.\n";
        let found = entry_findings(misplaced);
        assert_eq!(found.len(), 3, "{found:#?}");
        for name in ["What", "Why it matters", "What would close it"] {
            assert!(
                found
                    .iter()
                    .any(|f| f.contains(&format!("subsection `{name}`"))),
                "{name}: {found:#?}"
            );
        }
        // And the section itself absent: the subsections are owed and none is found, beside
        // the finding naming the missing section.
        let no_section = "---\nkind: defect\n---\n# The title\n\n## Summary\n\nWhat it is.\n";
        let found = entry_findings(no_section);
        assert_eq!(found.len(), 4, "{found:#?}");
        assert!(
            found.iter().any(|f| f.contains("section `Details`")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_deferred_entry_owes_a_trigger_where_the_others_owe_a_closure() {
        // The kind decides the last owed subsection, which is what makes the kind list worth
        // being closed: an unknown kind would silently owe the wrong three.
        let deferred = ENTRY
            .replace("kind: defect", "kind: deferred")
            .replace("### What would close it", "### Trigger");
        assert!(
            entry_findings(&deferred).is_empty(),
            "{:#?}",
            entry_findings(&deferred)
        );
        let wrong = ENTRY.replace("kind: defect", "kind: deferred");
        let found = entry_findings(&wrong);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("`Trigger`"), "{found:#?}");
    }

    #[test]
    fn frontmatter_outside_the_subset_is_reported_and_a_register_declaring_none_refuses_a_block() {
        let bad = entry_findings(&ENTRY.replace("kind: defect", "kind:\n  nested: 1"));
        assert!(bad.iter().any(|f| f.contains("does not parse")), "{bad:#?}");
        // A register with no declared metadata refuses a block: the keys would be unchecked.
        let manifest = declaring_full(
            "",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"reading\"]\n\n\
             [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("notes".to_string());
        present.push("notes/readings".to_string());
        present.push("notes/readings/README.md".to_string());
        present.push("notes/readings/index.md".to_string());
        present.push("notes/readings/a-reading.md".to_string());
        let model = Model::from_documents(vec![(
            PathBuf::from("notes/readings/a-reading.md"),
            "---\nstatus: settled\n---\n# A reading\n".to_string(),
        )]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("declares none"), "{found:#?}");
    }

    /// The findings of the shapes and of the generated files together, as a run over the working
    /// tree raises them, over the issue instance with `drop` left out of the listing and `add`
    /// put in.
    fn shapes_and_generated(drop: &str, add: &[&str]) -> Vec<Finding> {
        shapes_and_generated_in(drop, add, crate::check::Judged::WorkingTree)
    }

    /// The same, as a run judging the tree `judged` raises them.
    fn shapes_and_generated_in(
        drop: &str,
        add: &[&str],
        judged: crate::check::Judged,
    ) -> Vec<Finding> {
        let manifest = declaring("");
        let mut present: HashSet<PathBuf> = all_of("")
            .iter()
            .filter(|p| *p != drop)
            .map(PathBuf::from)
            .collect();
        present.extend(add.iter().map(PathBuf::from));
        let directories = crate::check::testing::implied_directories(&present);
        // Every generated index read as current but the one dropped, so the generated family
        // speaks of that one alone.
        let model = Model::from_documents(Vec::new());
        let generated = crate::model::generated::file_register_indexes(
            &model,
            &manifest,
            &present,
            &directories,
        );
        let committed: HashMap<PathBuf, String> = generated
            .into_iter()
            .filter(|(rel, _)| rel != Path::new(drop))
            .collect();
        let configs = HashMap::new();
        let inputs = Inputs {
            committed: &committed,
            configs: &configs,
            configs_not_text: &std::collections::HashSet::new(),
            generated_unreadable: &std::collections::HashSet::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged,
        };
        let mut found = check(&model, &manifest, &inputs).0;
        found.extend(crate::check::generated::check(
            &model,
            &manifest,
            &inputs,
            &[],
        ));
        found
    }

    /// The claim: a missing index, and a directory standing at its path, each raise a finding of
    /// the shapes and one of the generated files, and the two name one repair: the regeneration,
    /// after the directory's removal where one stands there. A missing README names a hand-written
    /// file, after the same removal, and no finding reads either directory as a group. Mutations:
    /// the shapes' finding naming a repair of its own for the index, or the directory's removal
    /// dropped from it, each fail a case; and `directory_contents`'s skip
    /// `if *path == home.readme || *path == home.index {` given `if *path == home.index {`,
    /// which reads the README's directory as an undeclared group, is caught by the README's
    /// count.
    #[test]
    fn a_missing_index_raises_two_findings_that_name_one_repair() {
        let index = "docs/open-issues/index.md";
        let cases = [
            (vec![], "run `klarch index`"),
            (
                vec![index, "docs/open-issues/index.md/a.md"],
                "move out what the directory holds, then remove it, then run `klarch index`",
            ),
        ];
        for (add, action) in cases {
            let found = shapes_and_generated(index, &add);
            let on: Vec<&Finding> = found
                .iter()
                .filter(|f| f.file == Path::new(index))
                .collect();
            // The shapes' finding and the generated file's, and no third reading the directory
            // as a group.
            assert_eq!(on.len(), 2, "{add:?}: {found:#?}");
            assert!(on.iter().all(|f| f.action == action), "{add:?}: {on:#?}");
        }
        let readme = "docs/open-issues/README.md";
        let found = shapes_and_generated(readme, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].action.starts_with("write it by hand"),
            "{found:#?}"
        );
        let found = shapes_and_generated(readme, &[readme, "docs/open-issues/README.md/a.md"]);
        let on: Vec<&Finding> = found
            .iter()
            .filter(|f| f.file == Path::new(readme))
            .collect();
        // The shapes' finding alone, and no second one reading the directory as a group.
        assert_eq!(on.len(), 1, "{found:#?}");
        assert!(
            on[0].action.starts_with(
                "move out what the directory holds, then remove it, then write it by hand"
            ),
            "{found:#?}"
        );
    }

    /// The claim: the repair both families name for a missing index, and for a directory at its
    /// path, is the command that writes the tree the run judges: `index --staged` under
    /// `check --staged`, and `index` over the working tree and under `commits`, where a history
    /// edit runs it. Mutations, each caught: `index_action`'s `Judged::Index` arm given
    /// `index`; and `check_under` passing `crate::check::Judged::WorkingTree` to `index_action`
    /// in place of `inputs.judged`, which leaves the generated family's finding alone naming
    /// `index --staged`.
    #[test]
    fn the_index_repair_names_the_command_that_writes_the_tree_judged() {
        use crate::check::Judged;
        let index = "docs/open-issues/index.md";
        for (judged, command) in [
            (Judged::WorkingTree, "run `klarch index`"),
            (Judged::Index, "run `klarch index --staged`"),
            (Judged::Commit, "run `klarch index`"),
        ] {
            for (add, action) in [
                (vec![], command.to_string()),
                (
                    vec![index, "docs/open-issues/index.md/a.md"],
                    format!("move out what the directory holds, then remove it, then {command}"),
                ),
            ] {
                let found = shapes_and_generated_in(index, &add, judged);
                let on: Vec<&Finding> = found
                    .iter()
                    .filter(|f| f.file == Path::new(index))
                    .collect();
                assert_eq!(on.len(), 2, "{judged:?} {add:?}: {found:#?}");
                assert!(
                    on.iter().all(|f| f.action == action),
                    "{judged:?} {add:?}: {on:#?}"
                );
            }
        }
    }

    /// The claim: a register's options whose bytes are not UTF-8 are one finding, naming their
    /// re-encoding, and the groups they would declare are unknown, so no subdirectory of the
    /// instance is reported as an undeclared group. Mutations,
    /// each caught: `config`'s test `if inputs.configs_not_text.contains(&home.config) {` given
    /// `if false {`, which reads the options as absent and reports the subdirectory; and its
    /// `return None;` given `return Some(BTreeSet::new());`, which reports it too.
    #[test]
    fn options_that_are_not_utf8_are_one_finding_and_declare_no_group_read() {
        let manifest = declaring("");
        let config = "docs/open-issues/register.toml";
        let mut present: HashSet<PathBuf> = all_of("").iter().map(PathBuf::from).collect();
        present.extend(
            [
                config,
                "docs/open-issues/a-group",
                "docs/open-issues/a-group/an-entry.md",
            ]
            .iter()
            .map(PathBuf::from),
        );
        let directories = crate::check::testing::implied_directories(&present);
        let not_text: HashSet<PathBuf> = [PathBuf::from(config)].into();
        let inputs = Inputs {
            committed: &HashMap::new(),
            configs: &HashMap::new(),
            configs_not_text: &not_text,
            generated_unreadable: &std::collections::HashSet::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        let found = check(&Model::from_documents(Vec::new()), &manifest, &inputs).0;
        let under: Vec<&Finding> = found
            .iter()
            .filter(|f| f.file.starts_with("docs/open-issues"))
            .collect();
        assert_eq!(under.len(), 1, "{found:#?}");
        assert_eq!(under[0].file, Path::new(config), "{found:#?}");
        assert!(under[0].what.contains("are not UTF-8 text"), "{found:#?}");
        assert_eq!(under[0].action, "re-encode the file as UTF-8", "{found:#?}");
    }

    /// The claim: a file of another suffix in an instance names the rename of an entry beside
    /// the move of anything else, and a declared group with no directory names its first entry,
    /// since git lists no empty directory. Mutations: either action without its case fails.
    #[test]
    fn a_stray_file_and_an_absent_group_name_the_repair_git_can_record() {
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("docs/open-issues/register.toml".to_string());
        present.push("docs/open-issues/an-entry.markdown".to_string());
        let found = findings_over(
            &manifest,
            &present,
            Model::from_documents(Vec::new()),
            &[("docs/open-issues/register.toml", "groups = [\"gone\"]\n")],
        );
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found.iter().any(|f| f.contains("is not an entry")
                && f.contains(
                    "→ if it is an entry, rename it to end in `.md`; otherwise move it out"
                )),
            "{found:#?}"
        );
        assert!(
            found.iter().any(|f| f.contains("`gone`")
                && f.contains("first entry under `docs/open-issues/gone/`")),
            "{found:#?}"
        );
    }

    /// A manifest declaring the location `notes` with the register `reading`, declaring the
    /// metadata `extra`, and the listing of its one entry.
    fn a_reading(extra: &str) -> (Manifest, Vec<String>) {
        let manifest = declaring_full(
            "",
            &format!(
                "[locations.notes]\npath = \"notes\"\nregisters = [\"reading\"]\n\n\
                 [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\
                 {extra}\n"
            ),
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        for p in [
            "notes",
            "notes/readings",
            "notes/readings/README.md",
            "notes/readings/index.md",
            "notes/readings/a-reading.md",
        ] {
            present.push(p.to_string());
        }
        (manifest, present)
    }

    fn reading_findings(extra: &str, body: &str) -> Vec<String> {
        let (manifest, present) = a_reading(extra);
        let model = Model::from_documents(vec![(
            PathBuf::from("notes/readings/a-reading.md"),
            body.to_string(),
        )]);
        findings_over(&manifest, &present, model, &[])
    }

    /// The claims: a block that does not parse, in a register declaring no key, names the
    /// deletion that the finding on a parsed block would name next; an undeclared key names its
    /// rename, its deletion, and its declaration where the register is declared, and no
    /// declaration in a built-in register, whose metadata a declaration cannot change. Mutations:
    /// the first arm's old rule text, and the declaration offered whatever the register, each
    /// fail a case.
    #[test]
    fn a_frontmatter_finding_names_the_repairs_its_register_admits() {
        let found = reading_findings("", "---\nstatus:\n  nested: 1\n---\n# A reading\n");
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("does not parse")
                && found[0].contains("→ delete the block, or declare its keys in")
                && found[0].contains("[registers.reading.metadata]"),
            "{found:#?}"
        );
        let found = reading_findings(
            "metadata.status.values = [\"open\"]\n",
            "---\nstatus: open\ntheme: x\n---\n# A reading\n",
        );
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].ends_with(
                "→ rename it to a declared key (status), delete it, or declare it in \
                 knowledge-architect.toml under [registers.reading.metadata]"
            ),
            "{found:#?}"
        );
        let found = entry_findings(&ENTRY.replace("kind: defect", "kind: defect\ntheme: x"));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].ends_with("→ rename it to a declared key (kind), or delete it"),
            "{found:#?}"
        );
        let found = entry_findings(&ENTRY.replace("kind: defect", "kind:\n  nested: 1"));
        assert!(
            found
                .iter()
                .any(|f| f.contains("does not parse") && f.contains("→ rewrite it as")),
            "{found:#?}"
        );
    }

    /// The claim: a block that does not parse, in a built-in register that declares no key, names
    /// the deletion alone: a built-in register's metadata is compiled in, so no declaration is
    /// offered, and the deletion is what the finding on a parsed block would name next. A spec is
    /// such an entry. Mutation: the arm's `"" => "delete the block".to_string(),` given
    /// `"" => "rewrite the block".to_string(),`, caught.
    #[test]
    fn a_frontmatter_that_does_not_parse_in_a_built_in_register_names_its_deletion() {
        let table = Anchors::of(&declaring(""), &[]);
        let spec = table.registers().by_name("spec").expect("built in");
        assert!(spec.metadata.is_empty(), "{:?}", spec.metadata);
        let sections: Vec<&str> = spec.sections.iter().map(String::as_str).collect();
        let at = "docs/plans/specs/a-spec/README.md";
        let text = format!(
            "---\nstatus:\n  nested: 1\n---\n{}",
            with_sections("A spec", &sections)
        );
        let mut present = all_of("");
        present.push(at.to_string());
        present.push("docs/plans/specs/a-spec".to_string());
        let model = Model::from_documents(vec![(PathBuf::from(at), text)]);
        let found: Vec<String> = findings_over(&declaring(""), &present, model, &[])
            .into_iter()
            .filter(|f| f.starts_with(at))
            .collect();
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not parse"), "{found:#?}");
        assert!(found[0].ends_with("→ delete the block"), "{found:#?}");
    }

    /// The claims: an entry whose block holds a blank line is read as carrying none, and the
    /// repair names the shape that is read; owed subsections found under another section name
    /// the section they go under. Mutations: the old action of each fails its case.
    #[test]
    fn a_misshapen_block_and_misplaced_subsections_name_where_the_shape_fails() {
        let found = entry_findings(&ENTRY.replace("kind: defect", "\nkind: defect"));
        assert!(
            found
                .iter()
                .any(|f| f.contains("carries no frontmatter") && f.contains("no blank line")),
            "{found:#?}"
        );
        let misplaced = "---\nkind: defect\n---\n# The title\n\n## Summary\n\n\
                         ### What\n\nOne.\n\n### Why it matters\n\nTwo.\n\n\
                         ### What would close it\n\nThree.\n\n## Details\n\nNothing here.\n";
        let found = entry_findings(misplaced);
        assert_eq!(found.len(), 3, "{found:#?}");
        assert!(
            found.iter().all(|f| f.contains(
                "→ the owed level-three subsections, in this order, under `## Details`: What, \
                 Why it matters, What would close it"
            )),
            "{found:#?}"
        );
    }

    #[test]
    fn a_location_carries_the_registers_it_declares_and_no_others() {
        let manifest = declaring_full(
            "",
            "[locations.agent-config]\npath = \".claude\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push(".claude".to_string());
        present.push(".claude/open-issues".to_string());
        present.push(".claude/open-issues/README.md".to_string());
        present.push(".claude/open-issues/index.md".to_string());
        // A location owes no README of its own, no CLAUDE.md and no design home.
        assert!(
            findings(&manifest, &present).is_empty(),
            "{:#?}",
            findings(&manifest, &present)
        );
        // Its path is an anchor's directory, and a missing one is ONE finding rather than a
        // declared-path finding beside it.
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("`.claude` is declared an anchor and does not exist"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_location_naming_no_such_register_and_a_component_inside_one_are_each_reported() {
        let manifest = declaring_full(
            "\"notes/a-part\"",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"nonesuch\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &["notes"]);
        assert!(
            found
                .iter()
                .any(|f| f.contains("`nonesuch`, which is no register")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.contains("sits inside the location `notes`")),
            "{found:#?}"
        );
        // The refused component is no anchor: nothing asserts its documents.
        assert!(
            !found.iter().any(|f| f.contains("`a-part` carries no")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_register_declaration_the_manifest_refused_is_reported_here() {
        // The manifest holds the complaint rather than refusing to load, because a manifest
        // that will not load reports nothing at all, and nothing at all reads as conformance.
        let manifest = declaring_full(
            "",
            "[registers.issue]\ndir = \"issues\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with(MANIFEST_NAME) && found[0].contains("built-in register"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_declared_path_the_manifest_refused_is_reported_here_and_acted_on_by_nothing() {
        // A `..` row in `exclude` would otherwise be reported absent and, read as one more
        // segment, could pass or fail the nesting check for a place it does not name. The
        // complaint is the one finding: the row is gone, so nothing reports it absent, and a
        // dot-spelled location is judged at the path it means.
        let manifest = declaring_full(
            "",
            "[locations.papers]\npath = \"./docs/papers\"\nregisters = [\"issue\"]\n\n",
            "[\"/build\"]",
            "[]",
            "[\"vendor/../vendor\"]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/papers".to_string());
        present.push("docs/papers/open-issues".to_string());
        present.push("docs/papers/open-issues/README.md".to_string());
        present.push("docs/papers/open-issues/index.md".to_string());
        let found = findings(&manifest, &present);
        // Every complaint, not the first: two refused rows are two findings.
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found[0].starts_with(MANIFEST_NAME)
                && found[0].contains("`/build` in [walk] skip-dirs is not project-relative"),
            "{found:#?}"
        );
        assert!(
            found[1].starts_with(MANIFEST_NAME)
                && found[1].contains("`vendor/../vendor` in [walk] exclude spells a `..` segment"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_declared_path_that_does_not_exist_is_reported_wherever_it_is_declared() {
        let manifest = declaring_full("", "", "[\"gone\"]", "[]", "[]", "[]");
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("[walk] skip-dirs"), "{found:#?}");
    }

    #[test]
    fn a_component_directory_that_does_not_exist_is_one_finding() {
        let manifest = declaring("\"parts/gone\"");
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("does not exist"), "{found:#?}");
    }

    #[test]
    fn an_anchor_name_that_is_ambiguous_or_reserved_is_reported_at_the_manifest() {
        let manifest = declaring("\"a/widget\", \"b/widget\"");
        let found = findings(&manifest, &["a/widget", "b/widget"]);
        assert!(
            found.iter().any(|f| f.contains("names 2 anchors")),
            "{found:#?}"
        );
        let manifest = declaring_full(
            "",
            "[locations.elsewhere]\npath = \"x\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &["x"]);
        assert!(
            found
                .iter()
                .any(|f| f.contains("the reserved name `elsewhere`")),
            "{found:#?}"
        );
    }

    /// A plan document with the given level-two sections, each holding one line.
    fn with_sections(title: &str, sections: &[&str]) -> String {
        let mut out = format!("# {title}\n");
        for section in sections {
            out.push_str(&format!("\n## {section}\n\nNone.\n"));
        }
        out
    }

    /// The claim: the README.md of a spec of specs/ and of a milestone owe the plan sections in
    /// order, the Threads and the Arguments apart, the README.md of a slice owes the slice
    /// sections, and a deliberation.md owes the Threads and the Arguments, each missing one a
    /// finding naming the list; a README.md holding a Threads or an Arguments section is a
    /// finding per section, naming its deliberation.md. Mutations checked: a slice's README
    /// checked against the plan sections; the `in_order` call over a deliberation.md dropped; the
    /// `deliberation_in_readme` call dropped; its `with_file_name` made `join`, which names a
    /// path inside the README.md.
    #[test]
    fn a_plan_document_owes_the_sections_of_its_kind() {
        let table = Anchors::of(
            &declaring(""),
            &[PathBuf::from("docs/plans/milestones/m-one/README.md")],
        );
        let spec = table.registers().by_name("spec").expect("built in");
        let plan = spec.sections.clone();
        let slice = table
            .by_name("m-one")
            .expect("the milestone's anchor")
            .sections_of(spec);
        let plan_sections: Vec<&str> = plan.iter().map(String::as_str).collect();
        let slice_sections: Vec<&str> = slice.iter().map(String::as_str).collect();
        let manifest = declaring("");
        let docs = |spec_sections: &[&str], slice_sections: &[&str]| {
            vec![
                (
                    "docs/plans/specs/a-spec/README.md".to_string(),
                    with_sections("A spec", spec_sections),
                ),
                (
                    "docs/plans/milestones/m-one/README.md".to_string(),
                    with_sections("A milestone", &plan_sections),
                ),
                (
                    "docs/plans/milestones/m-one/a-slice/README.md".to_string(),
                    with_sections("A slice", slice_sections),
                ),
            ]
        };
        let run = |docs: Vec<(String, String)>| {
            let mut present = all_of("");
            for (p, _) in &docs {
                present.push(p.clone());
            }
            for p in [
                "docs/plans/specs/a-spec",
                "docs/plans/milestones/m-one",
                "docs/plans/milestones/m-one/a-slice",
                "docs/plans/milestones/m-one/index.md",
            ] {
                present.push(p.to_string());
            }
            let model = Model::from_documents(
                docs.into_iter()
                    .map(|(p, t)| (PathBuf::from(p), t))
                    .collect(),
            );
            findings_over(&manifest, &present, model, &[])
                .into_iter()
                .filter(|f| !f.contains("generated"))
                .collect::<Vec<String>>()
        };
        let clean = run(docs(&plan_sections, &slice_sections));
        assert!(clean.is_empty(), "{clean:#?}");
        let without_names: Vec<&str> = plan_sections
            .iter()
            .copied()
            .filter(|s| *s != "Names")
            .collect();
        let found = run(docs(&without_names, &slice_sections));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].starts_with("docs/plans/specs/a-spec/README.md")
                && found[0].contains("level-two section `Names`"),
            "{found:#?}"
        );
        // The second section is titled as written here, not as the constant says: a spec that
        // keeps the title it had before the rename, or writes it in another case, owes it. The
        // literal is what a document must hold, so deleting the title from the list fails here.
        for written in ["How a step is worked", "How the Work is Done"] {
            let renamed: Vec<&str> = plan_sections
                .iter()
                .map(|s| {
                    if *s == "How the work is done" {
                        written
                    } else {
                        *s
                    }
                })
                .collect();
            let found = run(docs(&renamed, &slice_sections));
            assert_eq!(found.len(), 1, "{written}: {found:#?}");
            assert!(
                found[0].starts_with("docs/plans/specs/a-spec/README.md")
                    && found[0].contains("level-two section `How the work is done`"),
                "{written}: {found:#?}"
            );
        }
        // A slice spec holding the plan sections but none of its own owes each of its own.
        let found = run(docs(&plan_sections, &plan_sections));
        assert_eq!(found.len(), slice_sections.len(), "{found:#?}");
        assert!(found
            .iter()
            .all(|f| f.starts_with("docs/plans/milestones/m-one/a-slice/README.md")));
        // A README no longer owes the Threads and the Arguments, which its deliberation.md
        // holds, in a spec's directory and in a slice's alike; a deliberation.md missing one owes
        // it there.
        assert!(!plan_sections.contains(&"Threads") && !plan_sections.contains(&"Arguments"));
        let with_deliberation = |threads: &str| {
            let mut all = docs(&plan_sections, &slice_sections);
            for dir in [
                "docs/plans/specs/a-spec",
                "docs/plans/milestones/m-one/a-slice",
            ] {
                all.push((
                    format!("{dir}/deliberation.md"),
                    format!(
                        "# Its deliberation\n\n## {threads}\n\nNone.\n\n## Arguments\n\nNone.\n"
                    ),
                ));
            }
            all
        };
        let clean = run(with_deliberation("Threads"));
        assert!(clean.is_empty(), "{clean:#?}");
        let found = run(with_deliberation("Discussion"));
        assert_eq!(found.len(), 2, "{found:#?}");
        assert!(
            found.iter().all(|f| f.contains("deliberation.md")
                && f.contains("level-two section `Threads`")),
            "{found:#?}"
        );
        // A README.md that holds a Threads or an Arguments section, a spec's, a milestone's or a
        // slice's, is one finding per section, its repair naming the deliberation.md beside it.
        let mut held = docs(&plan_sections, &slice_sections);
        for (rel, text) in held.iter_mut() {
            let section = if rel.contains("a-spec") {
                "Threads"
            } else {
                "Arguments"
            };
            text.push_str(&format!("\n## {section}\n\nNone.\n"));
        }
        let found = run(held);
        assert_eq!(found.len(), 3, "{found:#?}");
        for (dir, section) in [
            ("docs/plans/specs/a-spec", "Threads"),
            ("docs/plans/milestones/m-one", "Arguments"),
            ("docs/plans/milestones/m-one/a-slice", "Arguments"),
        ] {
            assert!(
                found
                    .iter()
                    .any(|f| f.starts_with(&format!("{dir}/README.md:"))
                        && f.contains(&format!("holds the section `## {section}`"))
                        && f.contains(&format!("into `{dir}/deliberation.md` beside it"))),
                "{dir}: {found:#?}"
            );
        }
    }

    /// The claim: the item registers are the tool's, so a declaration of one, or a location
    /// naming one, is refused as the plan registers are. Mutation checked: `thread` left out of
    /// `Registers::is_plan_register`.
    #[test]
    fn an_item_register_is_refused_in_a_declaration() {
        let manifest = declaring_full(
            "",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"criterion\", \"issue\"]\n\n\
             [registers.thread]\nscope = \"opt-in\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let complaints: Vec<String> = manifest
            .complaints()
            .iter()
            .map(|f| f.what.clone())
            .collect();
        assert_eq!(complaints.len(), 2, "{complaints:#?}");
        assert!(complaints
            .iter()
            .any(|c| c.contains("[registers.thread] declares a plan register")));
        assert!(complaints
            .iter()
            .any(|c| c.contains("[locations.notes] carries `criterion`, a plan register")));
    }

    /// The claim: over a missing part of the plans layout, the shapes add nothing to what the phases
    /// report.
    #[test]
    fn a_missing_part_of_the_plans_layout_adds_nothing_after_its_phase() {
        let manifest = declaring("");
        for gone in [
            "docs/plans",
            "docs/plans/README.md",
            "docs/plans/specs",
            "docs/plans/milestones",
        ] {
            let mut present = all_of("");
            present.retain(|p| p != gone && !p.starts_with(&format!("{gone}/")));
            let found = findings(&manifest, &present);
            assert_eq!(found.len(), 1, "{gone}: {found:#?}");
        }
    }

    /// Whether phases 1 to 3 pass over a listing.
    fn phases_pass(manifest: &Manifest, present: &[String]) -> bool {
        let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let directories = crate::check::testing::implied_directories(&present);
        let inputs = Inputs {
            committed: &HashMap::new(),
            configs: &HashMap::new(),
            configs_not_text: &std::collections::HashSet::new(),
            generated_unreadable: &std::collections::HashSet::new(),
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
            judged: crate::check::Judged::WorkingTree,
        };
        crate::check::foundation(&Model::from_documents(Vec::new()), manifest, &inputs).is_ok()
    }

    /// The claim: a plans home's README and index are owed as every File home's are, and
    /// reported in phase 4, for the milestones home whose shape is new too. Mutation checked:
    /// the Directory arm of `check_under` skipping `navigation`.
    #[test]
    fn a_plans_home_without_its_readme_or_index_is_a_phase_four_finding() {
        let manifest = declaring("");
        for gone in [
            "docs/plans/milestones/index.md",
            "docs/plans/milestones/README.md",
            "docs/plans/specs/index.md",
        ] {
            let mut present = all_of("");
            present.retain(|p| p != gone);
            assert!(phases_pass(&manifest, &present), "{gone}");
            let found = findings(&manifest, &present);
            assert_eq!(found.len(), 1, "{gone}: {found:#?}");
            assert!(
                found[0].starts_with(gone) && found[0].contains("has no"),
                "{found:#?}"
            );
        }
    }

    #[test]
    fn a_register_home_the_walk_does_not_read_is_reported_at_the_row_that_takes_it() {
        // One `[walk]` row would otherwise take a whole register out of the regime: the home
        // is there, its entries are there, and no document under it is in the model, so
        // every entry-shape assertion judges nothing and the run is green.
        for (row, list) in [
            ("[walk] skip-dirs", "skip_dirs"),
            ("[walk] exclude", "exclude"),
        ] {
            let (dirs, exclude) = if list == "skip_dirs" {
                ("[\"docs/open-issues\"]", "[]")
            } else {
                ("[]", "[\"docs/open-issues\"]")
            };
            let manifest = declaring_full("", "", dirs, "[]", exclude, "[]");
            let found = findings(&manifest, &all_of(""));
            assert_eq!(found.len(), 1, "{row}: {found:#?}");
            assert!(
                found[0].contains(row) && found[0].contains("out of the walk"),
                "{row}: {found:#?}"
            );
        }
    }

    #[test]
    fn two_registers_of_one_anchor_may_not_share_a_home() {
        // The first register asked answers for every entry in the shared home, and every
        // reference of the other kind resolves to nothing for ever with nothing said.
        let manifest = declaring_full(
            "",
            "[registers.note]\nscope = \"component\"\nshape = \"heading\"\nlevel = 3\ndir = \"design\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("share the home"), "{found:#?}");
    }

    #[test]
    fn a_register_directory_that_is_not_one_plain_segment_is_reported() {
        // `..` puts the home outside the anchor and the real one is then read by nothing.
        for dir in ["", "..", "a/b", "Design"] {
            let manifest = declaring_full(
                "",
                &format!(
                    "[registers.note]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"{dir}\"\n\n"
                ),
                "[]",
                "[]",
                "[]",
                "[]",
            );
            let found = findings(&manifest, &all_of(""));
            assert_eq!(found.len(), 1, "{dir:?}: {found:#?}");
            assert!(
                found[0].contains("not one plain segment"),
                "{dir:?}: {found:#?}"
            );
        }
    }

    #[test]
    fn a_location_that_names_the_root_or_a_file_is_reported() {
        // The root is a component, carrying what is attached to it; a file satisfies a
        // presence test and every home under it then reads as absent.
        let manifest = declaring_full(
            "",
            "[locations.here]\npath = \"\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("names the project root"), "{found:#?}");
        // Spelled `.`, the same declaration: one finding, and not a second one saying the
        // directory `.` does not exist.
        let manifest = declaring_full(
            "",
            "[locations.here]\npath = \".\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("names the project root"), "{found:#?}");
        let manifest = declaring_full(
            "",
            "[locations.here]\npath = \"README.md\"\nregisters = []\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("which is a file"), "{found:#?}");
    }

    #[test]
    fn a_register_name_no_reference_can_spell_and_the_reserved_kind_are_each_reported() {
        for (body, needle) in [
            (
                "[registers.\"odd name\"]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"odd\"\n\n",
                "kind segment",
            ),
            (
                "[registers.path]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"paths\"\n\n",
                "reserved kind",
            ),
        ] {
            let manifest = declaring_full("", body, "[]", "[]", "[]", "[]");
            let found = findings(&manifest, &all_of(""));
            assert_eq!(found.len(), 1, "{body:?}: {found:#?}");
            assert!(found[0].contains(needle), "{body:?}: {found:#?}");
        }
    }

    #[test]
    fn an_anchor_name_no_reference_can_spell_is_reported_at_the_manifest() {
        // The name a reference spells comes from the path's basename, so a space in the path
        // makes an anchor nothing can point at.
        let manifest = declaring("\"parts/my thing\"");
        let found = findings(&manifest, &all_of(""));
        assert!(
            found
                .iter()
                .any(|f| f.contains("cannot be spelled in a reference")),
            "{found:#?}"
        );
    }

    #[test]
    fn a_directory_wearing_a_component_document_s_name_is_reported() {
        // A presence test is satisfied and the walk never reads a directory as a document,
        // so every claim that should live in it is outside every check.
        let manifest = declaring("");
        let mut present = all_of("");
        present.push("CLAUDE.md/inside.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("is a directory wearing the document's name"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_declaration_the_tool_then_discards_is_complained_about() {
        // Each of these is stored and read by nobody, so without the complaint it is
        // configuration that looks applied and is not.
        for (body, needle) in [
            (
                "[registers.issue]\n[registers.issue.metadata.theme]\nvalues = [\"a\"]\n\n",
                "sets metadata on a built-in register",
            ),
            (
                "[registers.note]\nscope = \"component\"\nshape = \"heading\"\nlevel = 3\ndir = \"notes\"\n\
                 sections = [\"One\"]\n\n",
                "on a heading register",
            ),
            (
                "[registers.note]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"notes\"\n\
                 kinds = [\"a\"]\n\n",
                "which only [registers.issue] takes",
            ),
        ] {
            let manifest = declaring_full("", body, "[]", "[]", "[]", "[]");
            let found = findings(&manifest, &all_of(""));
            assert!(
                found.iter().any(|f| f.contains(needle)),
                "{body:?}: {found:#?}"
            );
        }
    }

    #[test]
    fn a_misplaced_definition_is_reported_by_the_phase_the_helper_reads() {
        // The entity table's finding, phase 3, which the helper here reads beside this check's.
        let manifest = declaring("");
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# A title `##at-level-one`\n".to_string(),
        )]);
        let found = findings_over(&manifest, &all_of(""), model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("defines nothing"), "{found:#?}");
    }

    #[test]
    fn an_anchor_inside_another_anchor_s_register_home_is_reported() {
        // The reachable shape: a location declared inside a component's issue directory.
        // Every file under it would be read as an entry of the OUTER register, and the
        // findings would name that register and its anchor rather than the declaration.
        let manifest = declaring_full(
            "",
            "[locations.nested]\npath = \"docs/open-issues/nested\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/open-issues/nested".to_string());
        present.push("docs/open-issues/nested/open-issues".to_string());
        present.push("docs/open-issues/nested/open-issues/README.md".to_string());
        present.push("docs/open-issues/nested/open-issues/index.md".to_string());
        present.push("docs/open-issues/nested/scratch.txt".to_string());
        // The refused location is no anchor. The complaint comes first, nothing is asserted
        // against `nested` as an anchor, and what its directory then is — a stray
        // subdirectory of the outer issue register — is reported as that, which is true of
        // the tree. The phase gate is what keeps those later findings off the report.
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/open-issues/nested/open-issues/README.md"),
            "# Nested issues\n".to_string(),
        )]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert!(
            found[0].contains("the location `nested` sits inside the issue register's directory")
                && found[0].contains("`docs/open-issues` of `a-project`"),
            "{found:#?}"
        );
        assert!(
            !found
                .iter()
                .any(|f| f.contains("of `nested`") || f.contains("anchor `nested`")),
            "{found:#?}"
        );
        assert!(
            found
                .iter()
                .any(|f| f.contains("`nested` is a subdirectory of the issue register")),
            "{found:#?}"
        );
    }

    #[test]
    fn an_anchor_at_exactly_another_s_register_home_is_reported() {
        // At the home, not only inside it: the anchor's directory IS the outer register's
        // directory, and every file it holds would be read twice.
        let manifest = declaring_full(
            "",
            "[locations.at]\npath = \"docs/open-issues\"\nregisters = [\"tripwire\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/open-issues/tripwires.md".to_string());
        let found = findings(&manifest, &present);
        assert!(
            found.iter().any(|f| f.contains(
                "the location `at` sits inside the issue register's directory `docs/open-issues`"
            )),
            "{found:#?}"
        );
    }

    #[test]
    fn a_component_inside_a_location_s_home_is_one_finding_and_not_two() {
        // The pair is refused as a component inside a location, and not judged again for the
        // home it also sits in: one declaration, one finding.
        let manifest = declaring_full(
            "\"notes/open-issues/part\"",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("notes".to_string());
        present.push("notes/open-issues".to_string());
        present.push("notes/open-issues/README.md".to_string());
        present.push("notes/open-issues/index.md".to_string());
        present.push("notes/open-issues/part".to_string());
        present.extend(all_of("notes/open-issues/part"));
        let found = findings(&manifest, &present);
        let complaints: Vec<&String> = found
            .iter()
            .filter(|f| f.starts_with(MANIFEST_NAME) && f.contains("`part`"))
            .collect();
        assert_eq!(complaints.len(), 1, "{found:#?}");
        assert!(
            complaints[0].contains("the component `part` sits inside the location `notes`"),
            "{found:#?}"
        );
        // What the refused component's directory then is: a stray subdirectory of the
        // location's issue register, which is true of the tree.
        assert!(
            found
                .iter()
                .any(|f| f.contains("`part` is a subdirectory of the issue register")),
            "{found:#?}"
        );
    }

    #[test]
    fn an_anchor_whose_directory_holds_another_s_register_home_is_reported() {
        // A location at a component's `docs/` is deeper than the component, so it would own
        // every document in the component's homes, and each slug defined there would be
        // reported as misplaced against a location that carries no such register.
        let manifest = declaring_full(
            "",
            "[locations.papers]\npath = \"docs\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        let holding: Vec<&String> = found
            .iter()
            .filter(|f| f.contains("`papers`") && f.contains("holds the"))
            .collect();
        // One finding for the pair, naming every home held rather than one per home: four
        // findings from one declaration would bury the declaration.
        assert_eq!(holding.len(), 1, "{found:#?}");
        assert!(
            holding[0].contains(
                "the location `papers` at `docs` holds the design, goal, tripwire, rejected and \
                 issue homes of `a-project`"
            ),
            "{found:#?}"
        );
    }

    #[test]
    fn a_component_inside_another_component_s_register_home_is_reported() {
        // The same collision between two components: a component declared under the outer
        // goal home's directory would be both a subdirectory of that home and an anchor.
        let manifest = declaring("\"docs/goals/nested\"");
        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/nested".to_string());
        present.extend(all_of("docs/goals/nested"));
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/goals/README.md"),
            "# Goals\n".to_string(),
        )]);
        let found = findings_over(&manifest, &present, model, &[]);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("the component `nested` sits inside the goal register's directory"),
            "{found:#?}"
        );
    }

    #[test]
    fn two_anchors_at_one_path_are_reported_once() {
        // Two locations at one directory: `Anchors::owning` picks one on a tie, and every
        // document under the path then belongs to whichever it picked. A component and a
        // location at one path is the same fact, and it is reported as this rather than as
        // the component sitting inside the location.
        for (extra, needle) in [
            (
                "[locations.a]\npath = \"notes\"\nregisters = [\"issue\"]\n\n\
                 [locations.b]\npath = \"notes\"\nregisters = [\"tripwire\"]\n\n",
                "`a`, `b`",
            ),
            (
                "[locations.notes]\npath = \"parts\"\nregisters = [\"tripwire\"]\n\n",
                "`parts`, `notes`",
            ),
        ] {
            let components = if extra.contains("parts") {
                "\"parts\""
            } else {
                ""
            };
            let manifest = declaring_full(components, extra, "[]", "[]", "[]", "[]");
            let mut present = all_of("");
            present.push("notes".to_string());
            present.push("notes/open-issues".to_string());
            present.push("notes/open-issues/README.md".to_string());
            present.push("notes/open-issues/index.md".to_string());
            present.push("notes/tripwires.md".to_string());
            present.push("parts".to_string());
            present.push("parts/tripwires.md".to_string());
            present.extend(all_of("parts"));
            let found = findings(&manifest, &present);
            let same: Vec<&String> = found
                .iter()
                .filter(|f| f.contains("two anchors sit at"))
                .collect();
            assert_eq!(same.len(), 1, "{extra}: {found:#?}");
            assert!(same[0].contains(needle), "{extra}: {found:#?}");
            assert!(
                !found.iter().any(|f| f.contains("sits inside the location")),
                "{extra}: {found:#?}"
            );
        }
    }

    #[test]
    fn an_anchor_inside_another_s_directory_and_beside_its_homes_is_silent() {
        // The ordinary case, which the deepest-anchor rule exists for: a location inside a
        // component, away from every register home, carrying homes of its own.
        let manifest = declaring_full(
            "",
            "[locations.papers]\npath = \"docs/papers\"\nregisters = [\"issue\"]\n\n\
             [locations.inner]\npath = \"docs/papers/inner\"\nregisters = [\"tripwire\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        present.push("docs/papers".to_string());
        present.push("docs/papers/open-issues".to_string());
        present.push("docs/papers/open-issues/README.md".to_string());
        present.push("docs/papers/open-issues/index.md".to_string());
        present.push("docs/papers/inner".to_string());
        present.push("docs/papers/inner/tripwires.md".to_string());
        let found = findings(&manifest, &present);
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn a_directory_wearing_a_home_s_or_a_head_s_name_is_neither() {
        // A directory named like the file home is no file home, and one named like the
        // README is no head: the survey records each path's kind, and both impostors land
        // in the no-home and no-head arms by fact.
        let manifest = declaring("");
        let mut present = all_of("");
        present.retain(|p| p != "docs/design.md");
        present.push("docs/design.md".to_string());
        present.push("docs/design.md/inside.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("carries no design home"), "{found:#?}");

        let mut present = all_of("");
        present.retain(|p| p != "docs/goals.md");
        present.push("docs/goals".to_string());
        present.push("docs/goals/README.md".to_string());
        present.push("docs/goals/README.md/inside.md".to_string());
        let found = findings(&manifest, &present);
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("`docs/goals` has no README.md"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_skip_files_row_over_a_file_shaped_home_takes_the_register_out_of_the_walk() {
        let manifest = declaring_full("", "", "[]", "[\"docs/design.md\"]", "[]", "[]");
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("`docs/design.md` in [walk] skip-files takes the design register"),
            "{found:#?}"
        );
    }

    #[test]
    fn a_location_that_is_a_file_is_reported_once() {
        // The second finding would ask to create a directory inside a file.
        let manifest = declaring_full(
            "",
            "[locations.here]\npath = \"README.md\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let found = findings(&manifest, &all_of(""));
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(found[0].contains("which is a file"), "{found:#?}");
    }
}
