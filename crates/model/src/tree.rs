//! Phase 2: what the tree holds against what the manifest declares, and what the walk
//! could not read.
//!
//! Every finding here says that the model is incomplete: a file the walk could not read or
//! refused, an anchor or a register home that is not there, a home a walk row keeps out,
//! a declared path nothing answers to. A finding computed from the model afterwards — a
//! reference to a slug the unread file defined, an entry the missing home would have held —
//! is then unreliable in both directions, so `foundation` reports this phase and
//! judges nothing later. What this phase leaves standing, `check::registers` reads for shape.

use std::path::{Path, PathBuf};

use crate::entity::{self, Anchor, Anchors, Constructed, Home};
use crate::finding::Finding;
use crate::git::EntryKind;
use crate::manifest::{
    Manifest, Register, Shape, MANIFEST_NAME, MILESTONE_REGISTER, PLANS_DIR, SPEC_REGISTER,
};
use crate::model::Model;

use crate::phases::TreeInputs;

pub(crate) fn check(model: &Model, manifest: &Manifest, inputs: &TreeInputs) -> Vec<Finding> {
    let mut out = Vec::new();

    // A parse that could not be trusted must be LOUD. Silently it removes every citation in
    // the file from the walk while the run reports success — indistinguishable from a clean
    // file, and the guard the premortem named for a grammar that changes under an upgrade.
    for doc in model.documents() {
        if let Some(trouble) = &doc.parsed.trouble {
            out.push(Finding::at(
                &doc.rel,
                1,
                trouble.clone(),
                "nothing here is verified; fix the source, or what it says is checked by nothing",
            ));
        }
        // A heading markdown shows and the scanner does not read opens a section no check
        // sees: a slug on it defines nothing, and in a plan document the items under it are
        // read under the section before it. So the model is incomplete, which is this phase's,
        // per `design@model@headings-open-with-hash-marks`.
        for line in crate::scan::unread_headings(&doc.parsed) {
            out.push(Finding::at(
                &doc.rel,
                line,
                "markdown reads a heading here that the checker does not read",
                "write it as a line that opens with `#` marks and holds text, with no list or \
                 block-quote marker before them",
            ));
        }
    }

    declarations(&mut out, manifest, inputs);

    let anchors = Anchors::of(manifest, inputs.present);
    for anchor in anchors.all() {
        // An anchor directory that is not there is ONE finding. One per document it does not
        // carry would bury the single fact that explains all of them.
        if anchor.constructed == Some(Constructed::Plans)
            && !inputs.directories.contains(&anchor.path)
        {
            out.push(Finding::in_file(
                &anchor.path,
                format!(
                    "the plans directory `{}` does not exist",
                    anchor.path.display()
                ),
                "create it with a README.md, specs/ and milestones/; the tool constructs the \
                 anchor `plans` there in every project, and its two homes are where plan \
                 documents live",
            ));
            continue;
        }
        if !anchor.is_root() && !inputs.present.contains(&anchor.path) {
            out.push(Finding::in_file(
                &anchor.path,
                format!(
                    "`{}` is declared an anchor and does not exist",
                    anchor.path.display()
                ),
                format!("create it, or stop declaring it in {MANIFEST_NAME}"),
            ));
            continue;
        }
        // A location whose path is a file is `declarations`' finding, above; asserting its
        // homes as well would ask for a directory inside a file.
        if !anchor.is_root() && !inputs.directories.contains(&anchor.path) {
            continue;
        }
        if anchor.is_component {
            for name in manifest.required_documents() {
                let path = anchor.path.join(name);
                if !inputs.present.contains(&path) {
                    // The root is no member of `[project] components`, so the repair that
                    // stops listing a Component cannot be carried out there.
                    let repair = if anchor.is_root() {
                        "create it; every component carries the same documents, the project's \
                         root included"
                            .to_string()
                    } else {
                        format!(
                            "create it, or stop listing the component in {MANIFEST_NAME} \
                             [project]; every component carries the same documents"
                        )
                    };
                    out.push(Finding::in_file(
                        &path,
                        format!("the component `{}` carries no {name}", anchor.name),
                        repair,
                    ));
                } else if inputs.directories.contains(&path) {
                    // A directory wearing the document's name satisfies a presence test and is
                    // read by nothing: the walk never reads a directory as a document, so every
                    // claim that should live in it is outside every check.
                    out.push(Finding::in_file(
                        &path,
                        format!(
                            "`{}` is a directory wearing the document's name",
                            path.display()
                        ),
                        "the component documents are files; move the directory aside and \
                         create the file",
                    ));
                }
            }
        }
        if anchor.constructed == Some(Constructed::Plans) {
            let readme = anchor.path.join("README.md");
            if !inputs.present.contains(&readme) || inputs.directories.contains(&readme) {
                out.push(Finding::in_file(
                    &readme,
                    "the plans directory has no README.md".to_string(),
                    "create it; it says what the plans directory holds, and keeps it in git \
                     while no plan document is open",
                ));
            }
        }
        for name in &anchor.registers {
            let Some(register) = anchors.registers().by_name(name) else {
                continue;
            };
            let home = anchor.home_of(register);
            // The plan documents are judged by the File and Directory homes that hold them, so
            // an item register's home is not judged twice.
            if register.shape != Shape::Section {
                outside_the_walk(&mut out, manifest, anchor, register, &home);
            }
            match register.shape {
                Shape::Heading => heading_home(&mut out, anchor, register, &home, inputs),
                Shape::File => file_home(&mut out, anchor, register, &home, inputs),
                Shape::Directory => directory_home(&mut out, anchor, register, &home, inputs),
                // An item register's home is the plan's own documents, which exist when the
                // plan anchor does.
                Shape::Section => {}
            }
        }
    }
    plans_layout(&mut out, manifest, &anchors, inputs);
    // What every other Directory home holds, after the plans layout's, by the same rule.
    let entries = entity::placed_entries(manifest, &inputs.present.iter().collect::<Vec<_>>());
    for (_, register, home) in anchors.instances() {
        if register.shape == Shape::Directory && register.name != MILESTONE_REGISTER {
            directory_layout(&mut out, register, &home, &entries, inputs);
        }
    }
    out
}

/// A Directory register has its directory. What it holds is `directory_layout`'s question.
fn directory_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    inputs: &TreeInputs,
) {
    if !inputs.directories.contains(&home.dir) {
        out.push(Finding::in_file(
            &home.dir,
            format!(
                "the anchor `{}` carries no {} directory",
                anchor.name, register.name
            ),
            "create it with a README.md and an index.md; each of its entries is a directory \
             holding a README.md",
        ));
    }
}

/// What the plans directory and its milestones home hold, and the names of the plans.
///
/// - **The plans directory holds its README.md, specs/ and milestones/, and nothing else**, so
///   every plan document sits in a home.
/// - **milestones/ holds its README.md, its index.md and one directory per milestone**, by
///   `directory_layout`, which judges every Directory home alike.
/// - **A plan's name is no other anchor's and no reserved word**, per
///   `design@model@a-placed-name-reads-as-nothing-else`: a milestone
///   directory's name and a spec's id, compared with the components, the locations and each
///   other. A milestone the comparison refuses is no anchor, per `entity::milestone_refusal`.
///
/// Each is phase 2: the anchors and the entries of the plans layout are read off the tree, so a
/// later phase over a tree that breaks one would judge plan documents no anchor owns.
fn plans_layout(
    out: &mut Vec<Finding>,
    manifest: &Manifest,
    anchors: &Anchors,
    inputs: &TreeInputs,
) {
    let Some(plans) = anchors
        .all()
        .iter()
        .find(|a| a.constructed == Some(Constructed::Plans))
    else {
        return;
    };
    // A missing plans directory is one finding, above, and the rest would bury it.
    if !inputs.directories.contains(&plans.path) {
        return;
    }
    let (Some(spec), Some(milestone)) = (
        anchors.registers().by_name(SPEC_REGISTER),
        anchors.registers().by_name(MILESTONE_REGISTER),
    ) else {
        return;
    };
    let specs = plans.home_of(spec);
    let milestones = plans.home_of(milestone);
    let children = |dir: &Path| -> Vec<&PathBuf> {
        let mut out: Vec<&PathBuf> = inputs
            .present
            .iter()
            .filter(|p| p.parent() == Some(dir))
            .collect();
        out.sort();
        out
    };

    for path in children(&plans.path) {
        // The retired single file of `specs/` is `file_home`'s finding, named once.
        if *path == plans.path.join("README.md")
            || *path == specs.dir
            || *path == milestones.dir
            || *path == specs.file
        {
            continue;
        }
        out.push(Finding::in_file(
            path,
            format!(
                "`{}` sits in the plans directory, which holds its README.md, specs/ and \
                 milestones/ and nothing else",
                path.display()
            ),
            format!(
                "move a spec into {PLANS_DIR}/specs/ and a milestone into \
                 {PLANS_DIR}/milestones/, and anything else out of the plans directory"
            ),
        ));
    }

    let placed = entity::milestone_dirs(inputs.present.iter());
    let entries = entity::placed_entries(manifest, &inputs.present.iter().collect::<Vec<_>>());
    directory_layout(out, milestone, &milestones, &entries, inputs);

    // The same rule over the specs, per `design@model@a-placed-name-reads-as-nothing-else`: a spec's id is a name
    // the next step of the plans layout makes an anchor, so it may be
    // no kind's, component's, location's or reserved word, and no milestone's.
    let mut ids: Vec<(String, &PathBuf)> = inputs
        .present
        .iter()
        .filter(|p| !inputs.directories.contains(*p))
        .filter_map(|p| entity::entry_id(p, &specs.dir).map(|id| (id, p)))
        .collect();
    ids.sort();
    for (id, path) in ids {
        let why = if entity::is_reserved_anchor(&id) {
            Some("is a word the tool reserves")
        } else if entity::is_kind_name(&id, manifest.registers()) {
            Some("is the name of a kind")
        } else if manifest.components().all().iter().any(|c| c.name == id) {
            Some("is the name of a component")
        } else if manifest.locations().contains_key(&id) {
            Some("is the name of a location")
        } else if placed.contains_key(&id) {
            Some("is also the name of a milestone")
        } else {
            None
        };
        if let Some(why) = why {
            out.push(Finding::in_file(
                path,
                format!("the spec `{id}`'s name {why}"),
                "rename the file; a plan's name is cited in an anchor's place, so it must not \
                 read as another anchor or another plan",
            ));
        }
    }
}

/// What one Directory home holds, per `design@model@declarable-directory-register`: its
/// README.md, its index.md and one directory per entry, each holding a README.md, and no entry
/// holding a register.toml or a subdirectory; an entry's name is no other anchor's, per
/// `design@model@a-placed-name-reads-as-nothing-else`, and an entry it refuses is no anchor.
///
/// Phase 2, because the entries are anchors read off the tree: a later phase over a tree that
/// breaks one would judge documents no anchor owns. The `milestone` register keeps the wording
/// of the plans layout, whose words its findings and the planning skill share.
fn directory_layout(
    out: &mut Vec<Finding>,
    register: &Register,
    home: &Home,
    entries: &[entity::PlacedEntry],
    inputs: &TreeInputs,
) {
    if !inputs.directories.contains(&home.dir) {
        return;
    }
    let milestone = register.name == MILESTONE_REGISTER;
    let listing = home
        .dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let noun = if milestone {
        "milestone".to_string()
    } else {
        format!("{} entry", register.name)
    };
    let mut children: Vec<&PathBuf> = inputs
        .present
        .iter()
        .filter(|p| p.parent() == Some(home.dir.as_path()))
        .collect();
    children.sort();
    let placed = |name: &str| {
        entries
            .iter()
            .find(|e| e.register == register.name && e.path == home.dir.join(name))
    };
    for path in children {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        if inputs.directories.contains(path) {
            match placed(&name) {
                None => out.push(Finding::in_file(
                    path,
                    format!(
                        "`{}` holds no README.md, so it is no {noun}",
                        path.display()
                    ),
                    if milestone {
                        "create its README.md, the milestone document, or move the directory \
                         out of milestones/"
                            .to_string()
                    } else {
                        format!(
                            "create its README.md, the entry's document, or move the directory \
                             out of {listing}/"
                        )
                    },
                )),
                Some(entry) => {
                    if let Some(why) = &entry.refusal {
                        out.push(Finding::in_file(
                            path,
                            format!("the {noun} `{name}` is no anchor: its name {why}"),
                            if milestone {
                                "rename the directory; a milestone is cited \
                                 `milestone@plans@<id>` and its slices `spec@<id>@<slice>`, so \
                                 its name must not read as anything else"
                                    .to_string()
                            } else {
                                format!(
                                    "rename the directory; the entry is cited `{}@{}@<id>` and \
                                     what it carries `<kind>@<id>@<entry>`, so its name must not \
                                     read as anything else",
                                    register.name, entry.parent
                                )
                            },
                        ));
                    } else {
                        // What an entry carries sits beside its README, so a subdirectory is no
                        // group there: the File shape would ask for a register.toml, which is
                        // refused below, and the entry is an anchor read off the tree.
                        let mut inner: Vec<&PathBuf> = inputs
                            .present
                            .iter()
                            .filter(|p| p.parent() == Some(path.as_path()))
                            .filter(|p| inputs.directories.contains(*p))
                            .collect();
                        inner.sort();
                        for inner in inner {
                            out.push(Finding::in_file(
                                inner,
                                format!("`{}` is a subdirectory of a {noun}", inner.display()),
                                if milestone {
                                    "move its slice specs up beside the milestone's README; a \
                                     milestone holds its README, its index and one file per slice"
                                } else {
                                    "move its files up beside the entry's README; an entry holds \
                                     its README and the files of what it carries, ungrouped"
                                },
                            ));
                        }
                    }
                }
            }
            let config = path.join("register.toml");
            if inputs.present.contains(&config) {
                out.push(Finding::in_file(
                    &config,
                    format!("a {noun} directory holds no register.toml"),
                    if milestone {
                        "delete it; a milestone's slice specs sit beside its README, ungrouped"
                    } else {
                        "delete it; what an entry carries sits beside its README, ungrouped"
                    },
                ));
            }
            continue;
        }
        // The `<id>.md` beside an entry anchor is the single-file shape of what it carries, which
        // its home's own check names; beside a directory that is no anchor, nothing else names it.
        let beside = path.extension().is_some_and(|e| e == "md")
            && path.file_stem().is_some_and(|stem| {
                placed(&stem.to_string_lossy()).is_some_and(|e| e.refusal.is_none())
            });
        if name == "README.md" || name == "index.md" || beside {
            continue;
        }
        out.push(Finding::in_file(
            path,
            format!(
                "`{}` sits in {listing}/, which holds its README.md, its index.md and one \
                 directory per {noun}",
                path.display()
            ),
            if milestone {
                "a milestone is a directory holding its README.md; move a single-file plan into \
                 specs/, and anything else out"
            } else {
                "an entry is a directory holding its README.md; move anything else out"
            },
        ));
    }
}

/// A heading register has exactly one home shape, and the directory shape has a head.
fn heading_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    inputs: &TreeInputs,
) {
    // The survey records each path's kind, so both impostor shapes land in the no-home arm by
    // fact rather than by inference: a directory wearing the file home's name is not the file
    // home, and a plain file named like the directory is not the directory home.
    let file_home = inputs.present.contains(&home.file) && !inputs.directories.contains(&home.file);
    let dir_home = inputs.directories.contains(&home.dir);
    match (file_home, dir_home) {
        (true, false) => {}
        (false, false) => out.push(Finding::in_file(
            &home.file,
            format!(
                "the anchor `{}` carries no {} home",
                anchor.name, register.name
            ),
            format!(
                "create `{}`, or `{}/` with a README.md; a heading register keeps its entries \
                 in exactly one of the two",
                home.file.display(),
                home.dir.display()
            ),
        )),
        (true, true) => out.push(Finding::in_file(
            &home.dir,
            format!(
                "the anchor `{}` carries both `{}` and `{}`",
                anchor.name,
                home.file.display(),
                home.dir.display()
            ),
            "keep exactly one home; with two, an entry lands in either and the reader who \
             finds the other acts on half the register",
        )),
        (false, true) => {
            // Without a head nothing can list the subdocuments, so the per-subdocument
            // findings would bury the one repair that fixes them all. A directory wearing
            // the README's name is no head either: the walk never reads a directory.
            if !inputs.present.contains(&home.readme) || inputs.directories.contains(&home.readme) {
                out.push(Finding::in_file(
                    &home.readme,
                    format!(
                        "the {} directory `{}` has no README.md",
                        register.name,
                        home.dir.display()
                    ),
                    "create it; the README is the directory home's head — an introduction, \
                     and a bullet list of markdown links naming every subdocument",
                ));
            }
        }
    }
}

/// A file register has its directory, and not the retired single file.
fn file_home(
    out: &mut Vec<Finding>,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
    inputs: &TreeInputs,
) {
    // The single-file shape a heading register accepts is not a shape here, and a project
    // migrating from one is exactly where the mistake is made, so it is named. A register at its
    // carrier's own path has none, and its home's directory stands in for it.
    if home.file != home.dir && inputs.present.contains(&home.file) {
        out.push(Finding::in_file(
            &home.file,
            format!(
                "`{}` is the retired file shape of the {} register",
                home.file.display(),
                register.name
            ),
            format!(
                "split it into `{}/`, one file per entry, with a README.md and an index.md",
                home.dir.display()
            ),
        ));
    }
    if !inputs.directories.contains(&home.dir) {
        out.push(Finding::in_file(
            &home.dir,
            format!(
                "the anchor `{}` carries no {} directory",
                anchor.name, register.name
            ),
            "create it with a README.md and an index.md; a file register keeps one file per \
             entry, and the README is what keeps the directory in git",
        ));
    }
}

/// A register home the walk does not read is a register the regime does not reach.
///
/// The home exists, so the declared-path check passes; its entries exist, so the directory
/// listing passes; but no document under it is in the model, so every entry-shape assertion
/// judges nothing and the run is green. One `[walk] skip-dirs` row would take a whole
/// register out of the regime, which `design@core@the-regime-has-no-opt-out` refuses.
fn outside_the_walk(
    out: &mut Vec<Finding>,
    manifest: &Manifest,
    anchor: &Anchor,
    register: &Register,
    home: &Home,
) {
    let walk = manifest.walk();
    let covers = |list: &[PathBuf]| {
        list.iter()
            .find(|p| home.dir.starts_with(p) || home.file.starts_with(p))
            .cloned()
    };
    let hit = covers(&walk.skip_dirs)
        .map(|p| ("[walk] skip-dirs", p))
        .or_else(|| covers(&walk.exclude).map(|p| ("[walk] exclude", p)))
        .or_else(|| covers(&walk.skip_files).map(|p| ("[walk] skip-files", p)));
    if let Some((list, path)) = hit {
        out.push(Finding::in_file(
            MANIFEST_NAME,
            format!(
                "`{}` in {list} takes the {} register of `{}` out of the walk",
                path.display(),
                register.name,
                anchor.name
            ),
            "delete the row, or move the register home out from under it; a home no \
             document of is walked is a register every assertion passes over in silence",
        ));
    }
}

/// What the manifest declares about anchors, judged before anything is looked for on disk.
fn declarations(out: &mut Vec<Finding>, manifest: &Manifest, inputs: &TreeInputs) {
    for (name, decl) in manifest.locations() {
        if inputs.present.contains(&decl.path) && !inputs.directories.contains(&decl.path) {
            // Presence alone is satisfied by a file, and every home under it then reads as
            // absent, with a repair that says to create a directory inside a file.
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "[locations.{name}] names `{}`, which is a file",
                    decl.path.display()
                ),
                "a location is a directory carrying register homes; point it at one",
            ));
        }
    }

    // **Every path the manifest declares is checked to exist.** A row naming a deleted file is
    // silent in both directions: nobody is told it is dead, and a file later created at that
    // path inherits what the row grants: an extension's exemption list, for one, exempts a
    // document from whatever that extension checks without anyone deciding to.
    //
    // Existence, never file-ness: `skip-dirs` names directories, `exclude` names either, and
    // an archive directory is a declared skip that is legitimately empty in a fresh checkout.
    //
    // A location's own path is not in this list: it is an anchor, and the anchor loop below
    // reports a directory that is not there. Both would be one fact reported twice.
    let walk = manifest.walk();
    let mut declared: Vec<(&str, &Vec<PathBuf>)> = vec![
        ("[walk] skip-dirs", &walk.skip_dirs),
        ("[walk] skip-files", &walk.skip_files),
        ("[walk] exclude", &walk.exclude),
    ];
    // Each extension's declared paths, per
    // `design@core@an-extension-plugs-in-through-phased-hooks`.
    declared.extend(
        manifest
            .extension_paths()
            .iter()
            .map(|(label, paths)| (label.as_str(), paths)),
    );
    for (list, paths) in declared {
        for path in paths {
            if inputs.present.contains(path) {
                continue;
            }
            out.push(Finding::in_file(
                MANIFEST_NAME,
                format!(
                    "`{}` is declared in {list} and does not exist",
                    path.display()
                ),
                "delete the row, or restore what it names; a declaration nothing checks \
                 silently covers whatever is created at that path next. What git ignores is \
                 outside the listing the walk reads and is never declared here",
            ));
        }
    }

    // **A file git both tracks and ignores is a finding naming the file.** The two states
    // contradict each other and the contradiction is silent: the walk reads the file, because
    // the tracked listing is unaffected by the ignore rules; and every path reference to it is
    // asserted, because `git check-ignore` skips what the index holds. So the ignore rule says
    // the file is out of the project and every check reads it in, and untracking it would flip
    // both answers at once. `design@model@tracked-and-ignored-is-a-finding` is the head.
    for path in inputs.tracked_and_ignored {
        out.push(Finding::in_file(
            path,
            "git tracks this file and the ignore rules also cover it".to_string(),
            "untrack the file, or narrow the ignore rule that covers it; while both hold, the \
             walk reads the file and a path reference to it is asserted, which is the reverse \
             of what the ignore rule says",
        ));
    }

    // **A symlink and a gitlink are findings naming the entry.** The walk reads through
    // neither: a symlink read off the filesystem yields its target's bytes and read off a
    // commit's tree yields the target's path as text, and a checkout without symlinks holds
    // that text as a plain file, so two readers of one tree would judge two documents; a
    // gitlink names a commit of another repository the listing never descends into. A walk
    // row is the declared way to keep either.
    for entry in inputs.links {
        let (what, action) = match entry.kind {
            EntryKind::Symlink => (
                format!(
                    "`{}` is a symlink, and the walk reads no document through one",
                    entry.rel.display()
                ),
                "replace it with the file it points at, or with a reference to that file; a \
                 checkout without symlinks holds the target's path as text here, so two readers \
                 of this tree would judge two documents. Name it in [walk] skip-files to keep it",
            ),
            EntryKind::Gitlink => (
                format!(
                    "`{}` is a submodule, and the walk reads nothing under it",
                    entry.rel.display()
                ),
                "name it in [walk] exclude to declare the silence; the tool models no \
                 submodule, and the checker's open issues hold the question",
            ),
            EntryKind::File => continue,
        };
        out.push(Finding::in_file(&entry.rel, what, action));
    }

    // **A file outside the walk that could not be read is a finding naming it.** An extension
    // may assert that no unwalked file says something, and a file it cannot read is one it
    // cannot judge; the run stops here rather than pass it over.
    for (path, read) in inputs.outside {
        use crate::survey::Outside;
        let (what, action) = match read {
            Outside::Text(_) | Outside::Binary => continue,
            Outside::Missing => (
                "git lists this file and the working tree does not hold it".to_string(),
                "stage the deletion, or restore the file",
            ),
            Outside::Directory => (
                "git lists this path and the working tree holds a directory there: a repository \
                 nested in this one, whose files no check reads, or a tracked file replaced by a \
                 directory"
                    .to_string(),
                "for a nested repository, name it in an ignore rule or in [walk] skip-dirs to \
                 declare the silence; for a replaced file, stage the change or restore the file",
            ),
            Outside::Unreadable(why) => (
                format!(
                    "this file is outside the walk and could not be read, so no check judged \
                     what it says: {why}"
                ),
                "make it readable to the user running the check, or name it in [walk] \
                 skip-files if it is deliberately unchecked",
            ),
        };
        out.push(Finding::in_file(path, what, action));
    }

    // **A name the walk refuses is a finding naming the file**, per `walk::refused`. The file
    // was read by no check, so a rule quote in it is verified by nothing, and a reference is
    // one backticked span, so nothing can point at it either.
    for path in inputs.refused {
        let why = crate::walk::refusal(path).unwrap_or_else(|| "is refused".to_string());
        out.push(Finding::in_file(
            path,
            format!("this file's name {why}, and no check read it"),
            "rename the file; a name with a line break in it fits on no output line and in no \
             reference, and Windows creates no file under a name it forbids. Name it in [walk] \
             skip-files or in an ignore rule to keep it as it is",
        ));
    }
}

#[cfg(test)]
mod plans_tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::PathBuf;

    /// Every path a component owes, under `dir`, with the file-shaped heading homes.
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
                crate::phases::testing::PLANS_TREE
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

    /// The claim: a row the manifest refused is reported absent by nothing, since the row is gone.
    /// Mutation checked: `normalise_list` keeping the row it complains about.
    #[test]
    fn a_declared_path_the_manifest_refused_is_not_reported_absent() {
        let manifest = declaring_full("", "", "[\"/build\"]", "[]", "[\"vendor/../vendor\"]", "[]");
        assert_eq!(
            manifest.complaints().len(),
            2,
            "{:#?}",
            manifest.complaints()
        );
        let found = tree_findings(&manifest, &all_of(""));
        assert_eq!(found, Vec::<String>::new());
    }

    // A copy of the core's helpers of the same name: a helper of a test is shared by no public item,
    // per `design@model@model-facade`, until the shared test Component holds them.

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

    /// The claim: the plans anchor's name is reserved, and the two plan registers are the
    /// tool's to place, so a declaration of either, or a location naming one, is a phase-1
    /// complaint. Mutation checked: `plans` left out of `entity::is_reserved_anchor`.
    #[test]
    fn the_plans_name_and_the_plan_registers_are_refused_in_a_declaration() {
        let manifest = declaring_full(
            "",
            "[locations.plans]\npath = \"x\"\nregisters = []\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"spec\", \"issue\"]\n\n\
             [registers.spec]\nscope = \"opt-in\"\n\n\
             [registers.milestone]\n\n",
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
        let one = |needle: &str| {
            let hits = complaints.iter().filter(|c| c.contains(needle)).count();
            assert_eq!(hits, 1, "{needle:?} in {complaints:#?}");
        };
        one("gives the anchor the reserved name `plans`");
        one("[locations.notes] carries `spec`, a plan register");
        one("[registers.spec] declares a plan register");
        one("[registers.milestone] declares a plan register");
        assert_eq!(complaints.len(), 4, "{complaints:#?}");
        // The refused row is no anchor, and the tool's own `plans` stands.
        assert!(!manifest.locations().contains_key("plans"));
        assert!(manifest.plans());
        assert_eq!(manifest.locations()["notes"].registers, vec!["issue"]);
        // The repair for a register nothing declares offers the ones a location may name.
        let manifest = declaring_full(
            "",
            "[locations.notes]\npath = \"notes\"\nregisters = [\"nonesuch\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let action = &manifest.complaints()[0].action;
        assert!(
            action.contains("issue") && !action.contains("spec"),
            "{action}"
        );
    }

    /// The claim: a declared anchor at the plans directory is the one refused, and the tool's
    /// `plans` stands, so the repair names the declaration a project can change. Mutation
    /// checked: `plans` pushed after the declared locations.
    #[test]
    fn a_declared_anchor_at_the_plans_directory_is_refused_and_plans_stands() {
        for row in [
            "[locations.papers]\npath = \"docs/plans\"\nregisters = []\n\n",
            "[locations.papers]\npath = \"docs/plans/specs/inner\"\nregisters = []\n\n",
        ] {
            let manifest = declaring_full("", row, "[]", "[]", "[]", "[]");
            assert!(manifest.plans(), "{row}");
            assert!(!manifest.locations().contains_key("papers"), "{row}");
            assert_eq!(
                manifest.complaints().len(),
                1,
                "{row}: {:#?}",
                manifest.complaints()
            );
        }
        let manifest = declaring("\"docs/plans\"");
        assert!(manifest.plans());
        assert_eq!(
            manifest.components().all().len(),
            1,
            "{:#?}",
            manifest.complaints()
        );
    }

    /// The claims of the layout's odd shapes, each one finding in phase 2: a plans README that
    /// is a directory, a milestone whose README is a directory (no anchor, so no milestone), the
    /// retired single file of specs/ and of a milestone (named once, by the File rule), and a
    /// milestone named like a location. Mutations checked, each failing this test: the
    /// directory test dropped from the plans README check; `|| *path == specs.file` deleted;
    /// `|| beside` deleted; the location arm of `milestone_refusal` deleted.
    #[test]
    fn each_odd_shape_of_the_plans_layout_is_one_phase_two_finding() {
        let manifest = declaring_full(
            "",
            "[locations.side-notes]\npath = \"notes\"\nregisters = [\"issue\"]\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let base = || {
            let mut present = all_of("");
            for p in ["notes", "notes/open-issues", "notes/open-issues/README.md"] {
                present.push(p.to_string());
            }
            present.push("notes/open-issues/index.md".to_string());
            present
        };
        assert!(tree_findings(&manifest, &base()).is_empty());
        for (added, removed, at, what) in [
            (
                vec!["docs/plans/README.md/inner.md"],
                vec!["docs/plans/README.md"],
                "docs/plans/README.md",
                "the plans directory has no README.md",
            ),
            (
                vec!["docs/plans/milestones/m-one/README.md/inner.md"],
                vec![],
                "docs/plans/milestones/m-one",
                "holds no README.md, so it is no milestone",
            ),
            (
                vec!["docs/plans/specs.md"],
                vec![],
                "docs/plans/specs.md",
                "retired file shape of the spec register",
            ),
            (
                vec![
                    "docs/plans/milestones/m-one/README.md",
                    "docs/plans/milestones/m-one.md",
                ],
                vec![],
                "docs/plans/milestones/m-one.md",
                "retired file shape of the spec register",
            ),
            (
                vec!["docs/plans/milestones/side-notes/README.md"],
                vec![],
                "docs/plans/milestones/side-notes",
                "is the name of a location",
            ),
            (
                vec!["docs/plans/milestones/plans/README.md"],
                vec![],
                "docs/plans/milestones/plans",
                "is not two or more lower-case words",
            ),
            (
                vec!["docs/plans/milestones/index/README.md"],
                vec![],
                "docs/plans/milestones/index",
                "is not two or more lower-case words",
            ),
            (
                vec![
                    "docs/plans/milestones/m-one/README.md",
                    "docs/plans/milestones/m-one/sub/a-slice.md",
                ],
                vec![],
                "docs/plans/milestones/m-one/sub",
                "one file per slice",
            ),
        ] {
            let mut present = base();
            present.retain(|p| !removed.contains(&p.as_str()));
            // A listing holds every directory above a file, as the survey's does.
            for path in &added {
                for ancestor in std::path::Path::new(path).ancestors() {
                    let a = ancestor.display().to_string();
                    if !a.is_empty() && !present.contains(&a) {
                        present.push(a);
                    }
                }
            }
            let found = tree_findings(&manifest, &present);
            assert_eq!(found.len(), 1, "{added:?}: {found:#?}");
            assert!(
                found[0].starts_with(at) && found[0].contains(what),
                "{added:?}: {found:#?}"
            );
        }
        // A milestone whose README is a directory is no anchor, so nothing defines it.
        let present: Vec<PathBuf> = ["docs/plans/milestones/m-one/README.md/inner.md"]
            .iter()
            .map(PathBuf::from)
            .collect();
        assert!(Anchors::of(&manifest, &present).by_name("m-one").is_none());
    }

    /// The claim: a spec named after a kind is reported by its name, as a milestone is, per
    /// `design@model@a-placed-name-reads-as-nothing-else`. Mutation checked:
    /// the kind test removed from the spec-name check reports nothing.
    #[test]
    fn a_spec_named_after_a_kind_is_reported_by_its_name() {
        let manifest = declaring("");
        for name in ["skill", "design"] {
            let mut present = all_of("");
            present.push(format!("docs/plans/specs/{name}.md"));
            let found = tree_findings(&manifest, &present);
            assert!(
                found
                    .iter()
                    .any(|f| f.contains(&format!("the spec `{name}`'s name is the name of a kind"))),
                "{name}: {found:#?}"
            );
        }
    }

    /// The claim: a walk row that takes a milestone out of the walk is one finding, against its
    /// `spec` register, and none against the item registers, whose home is the same documents.
    /// Mutation checked: the guard of `outside_the_walk` on the Section shape removed.
    #[test]
    fn a_milestone_outside_the_walk_is_one_finding() {
        let manifest = declaring_full(
            "",
            "",
            "[\"docs/plans/milestones/m-one\"]",
            "[]",
            "[]",
            "[]",
        );
        let mut present = all_of("");
        for p in [
            "docs/plans/milestones/m-one",
            "docs/plans/milestones/m-one/README.md",
        ] {
            present.push(p.to_string());
        }
        let found: Vec<String> = tree_findings(&manifest, &present)
            .into_iter()
            .filter(|f| f.contains("out of the walk"))
            .collect();
        assert_eq!(found.len(), 1, "{found:#?}");
        assert!(
            found[0].contains("the spec register of `m-one`"),
            "{found:#?}"
        );
    }

    /// The claim: a component register whose home at the root would be the plans directory is
    /// refused at the declaration, and the tool's `plans` stands.
    #[test]
    fn a_component_register_at_the_plans_directory_is_refused() {
        let manifest = declaring_full(
            "",
            "[registers.proposal]\nscope = \"component\"\nshape = \"file\"\ndir = \"plans\"\n\n",
            "[]",
            "[]",
            "[]",
            "[]",
        );
        let complaints: Vec<&str> = manifest
            .complaints()
            .iter()
            .map(|f| f.what.as_str())
            .collect();
        assert_eq!(complaints.len(), 1, "{complaints:#?}");
        assert!(
            complaints[0].contains("[registers.proposal]")
                || complaints[0].contains("proposal register")
        );
        assert!(manifest.plans());
        assert!(manifest.registers().by_name("proposal").is_none());
    }

    /// The findings of phase 2 alone, over a listing.
    fn tree_findings(manifest: &Manifest, present: &[String]) -> Vec<String> {
        let present: HashSet<PathBuf> = present.iter().map(PathBuf::from).collect();
        let directories = crate::phases::testing::implied_directories(&present);
        let inputs = TreeInputs {
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
            installed: &[],
            shipped: &[],
        };
        check(&Model::from_documents(Vec::new()), manifest, &inputs)
            .iter()
            .map(|f| format!("{}  {}  → {}", f.location(), f.what, f.action))
            .collect()
    }

    /// The claim: a missing plans directory, plans README or plans home is one phase-2
    /// finding each, at the path it belongs at, and a missing plans directory is one finding
    /// rather than one per part. Mutation checked: the README requirement skipped.
    #[test]
    fn each_missing_part_of_the_plans_layout_is_one_phase_two_finding() {
        let manifest = declaring("");
        for (gone, at, what) in [
            (
                "docs/plans",
                "docs/plans",
                "the plans directory `docs/plans` does not exist",
            ),
            (
                "docs/plans/README.md",
                "docs/plans/README.md",
                "the plans directory has no README.md",
            ),
            (
                "docs/plans/specs",
                "docs/plans/specs",
                "the anchor `plans` carries no spec directory",
            ),
            (
                "docs/plans/milestones",
                "docs/plans/milestones",
                "the anchor `plans` carries no milestone directory",
            ),
        ] {
            let mut present = all_of("");
            present.retain(|p| p != gone && !p.starts_with(&format!("{gone}/")));
            let found = tree_findings(&manifest, &present);
            assert_eq!(found.len(), 1, "{gone}: {found:#?}");
            assert!(
                found[0].starts_with(at) && found[0].contains(what),
                "{gone}: {found:#?}"
            );
        }
    }

    #[test]
    fn a_refused_name_is_one_finding_naming_the_file_on_one_line() {
        let manifest = declaring("");
        let present: HashSet<PathBuf> = all_of("").iter().map(PathBuf::from).collect();
        let directories = crate::phases::testing::implied_directories(&present);
        let refused = vec![
            PathBuf::from("docs/a\nb.md"),
            PathBuf::from("docs/c\rd.md"),
            PathBuf::from("docs/e:f.md"),
        ];
        let inputs = TreeInputs {
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &HashSet::new(),
            tracked_and_ignored: &[],
            refused: &refused,
            links: &[],
            installed: &[],
            shipped: &[],
        };
        let found = check(&Model::from_documents(Vec::new()), &manifest, &inputs);
        assert_eq!(found.len(), 3, "{found:#?}");
        for (finding, (head, why)) in found.iter().zip([
            ("docs/a\\nb.md  ", "holds a line break"),
            ("docs/c\\rd.md  ", "holds a line break"),
            ("docs/e:f.md  ", "holds `:`, which Windows forbids"),
        ]) {
            assert!(finding.what.contains(why), "{found:#?}");
            assert_eq!(finding.to_string().lines().count(), 2, "{finding}");
            assert!(finding.to_string().starts_with(head), "{finding}");
        }
    }
}
