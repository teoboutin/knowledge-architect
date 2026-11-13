//! The plans layout: every plan document a directory, read off the tree.
//!
//! A spec is a directory of specs/, or of a group of specs/, holding its `README.md`; a milestone
//! is a directory of milestones/ holding its `README.md`, its generated `index.md`, one directory
//! per live slice, and its `harvested/`, which holds one directory per landed slice. Each plan
//! document's directory holds, beside its `README.md`, its `deliberation.md`, its `search/` and its
//! `transcript.txt`, and nothing else but what a milestone's holds besides. These are types of their own, not entries of the generic Directory shape, which
//! a declared register uses carrying one register, per `design@model@declarable-directory-register`.
//!
//! **The one frozen location is a milestone's `harvested/`.** Every file under it leaves the walk
//! by construction; a markdown file there is parsed for its definitions only, as the installed
//! files are, and any other file is never read, per
//! `design@check@installed-files-outside-the-walk`: a harvested slice is judged while it is live,
//! and the move under `harvested/` is what freezes it.
//!
//! What this module answers is where things sit, from the paths alone; which of them is an anchor,
//! and what a misplaced one is reported as, is `entity`'s and `tree`'s.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::manifest::{
    PlanKind, DELIBERATION_FILE, HARVESTED_DIR, MILESTONES_HOME, PLANS_DIR, SEARCH_DIR, SPECS_HOME,
    TRANSCRIPT_FILE,
};

/// The head of every plan document's directory.
pub(crate) const README: &str = "README.md";

/// The generated listing of a home, and of a milestone's slices.
pub(crate) const INDEX: &str = "index.md";

/// specs/, the home of the specs, project-relative.
pub(crate) fn specs_dir() -> PathBuf {
    Path::new(PLANS_DIR).join(SPECS_HOME)
}

/// milestones/, the home of the milestones, project-relative.
pub(crate) fn milestones_dir() -> PathBuf {
    Path::new(PLANS_DIR).join(MILESTONES_HOME)
}

/// One plan document's directory the tree places: its basename, its path, its kind, and for a
/// slice whether it sits under its milestone's `harvested/`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PlanDir {
    pub name: String,
    pub path: PathBuf,
    pub kind: PlanKind,
    pub harvested: bool,
}

impl PlanDir {
    /// The plan document's head, which defines it.
    pub(crate) fn readme(&self) -> PathBuf {
        self.path.join(README)
    }
}

fn text(c: Component<'_>) -> String {
    c.as_os_str().to_string_lossy().into_owned()
}

/// The components of `rel` under `dir`, or `None` where `dir` does not hold it.
fn parts_under(rel: &Path, dir: &Path) -> Option<Vec<String>> {
    rel.strip_prefix(dir)
        .ok()
        .map(|inside| inside.components().map(text).collect())
}

/// Each directory under `dir`, at the depth given by the length of the prefix `matches` accepts,
/// that holds a `README.md` as a file, in path order.
///
/// A listing of files and directories holds a `README.md` that is a directory as a path too, and
/// a listing of files alone does not, so one is read off what sits under it: every caller then
/// places the same plan documents, whichever listing it holds.
fn headed<'a>(
    dir: &Path,
    paths: impl IntoIterator<Item = &'a PathBuf>,
    matches: impl Fn(&[String]) -> bool,
) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut not_files: Vec<PathBuf> = Vec::new();
    for path in paths {
        let Some(parts) = parts_under(path, dir) else {
            continue;
        };
        let Some(at) = parts.iter().position(|p| p == README) else {
            continue;
        };
        if at == 0 || !matches(&parts[..at]) {
            continue;
        }
        let head = parts[..at].iter().fold(dir.to_path_buf(), |p, s| p.join(s));
        if at + 1 == parts.len() {
            found.push(head);
        } else {
            not_files.push(head);
        }
    }
    found.retain(|p| !not_files.contains(p));
    found.sort();
    found.dedup();
    found
}

/// Every spec directory of specs/, grouped or not, in path order, two of one id included: a
/// directory of specs/ holding a `README.md` is a spec, and one holding none is a group, whose
/// directories holding a `README.md` are specs, per `design@check@groups-are-subdirectories`.
pub(crate) fn spec_dirs_all<'a>(paths: impl IntoIterator<Item = &'a PathBuf>) -> Vec<PlanDir> {
    let home = specs_dir();
    let paths: Vec<&PathBuf> = paths.into_iter().collect();
    let top = headed(&home, paths.iter().copied(), |p| p.len() == 1);
    let grouped = headed(&home, paths.iter().copied(), |p| {
        p.len() == 2 && !top.contains(&home.join(&p[0]))
    });
    let mut all: Vec<PathBuf> = top.into_iter().chain(grouped).collect();
    all.sort();
    all.into_iter()
        .map(|path| PlanDir {
            name: path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default(),
            path,
            kind: PlanKind::Spec,
            harvested: false,
        })
        .collect()
}

/// Each spec of specs/ by id, the first in path order where two share one: the spec anchors a
/// tree places, before their names are judged. The second is the duplicate entry finding.
pub(crate) fn spec_dirs<'a>(
    paths: impl IntoIterator<Item = &'a PathBuf>,
) -> BTreeMap<String, PathBuf> {
    let mut out = BTreeMap::new();
    for spec in spec_dirs_all(paths) {
        out.entry(spec.name).or_insert(spec.path);
    }
    out
}

/// The slices of the milestone at `milestone`, live then harvested, each in path order: a
/// directory of the milestone holding a `README.md`, but `harvested/` and `search/`, and a
/// directory of its `harvested/` holding one.
pub(crate) fn slices<'a>(
    milestone: &Path,
    paths: impl IntoIterator<Item = &'a PathBuf>,
) -> Vec<PlanDir> {
    let paths: Vec<&PathBuf> = paths.into_iter().collect();
    let live = headed(milestone, paths.iter().copied(), |p| {
        p.len() == 1 && p[0] != HARVESTED_DIR && p[0] != SEARCH_DIR
    });
    let harvested = headed(&milestone.join(HARVESTED_DIR), paths.iter().copied(), |p| {
        p.len() == 1
    });
    let slice = |path: PathBuf, harvested: bool| PlanDir {
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
        path,
        kind: PlanKind::Slice,
        harvested,
    };
    live.into_iter()
        .map(|p| slice(p, false))
        .chain(harvested.into_iter().map(|p| slice(p, true)))
        .collect()
}

/// Whether `rel` sits under a milestone's `harvested/`, the one frozen location: out of the walk
/// by construction, parsed for definitions only where it is markdown, and its content judged by no
/// check, per `design@check@a-frozen-file-is-guarded-by-the-review`. Where it sits is the layout's to judge: a file directly under
/// `harvested/` is frozen, and phase 2 reports it.
///
/// Read off the path alone, whatever the milestone's directory holds, so the walk, the survey and
/// `commits` agree on it with no anchor built.
pub(crate) fn is_frozen(rel: &Path) -> bool {
    frozen_place(rel).is_some()
}

/// Where a frozen file sits, read off its path alone: its milestone's directory and name, and, for
/// a file inside a directory of `harvested/`, that directory's name, the slice's. A file directly
/// under `harvested/` is frozen too, in no slice, and the layout reports it.
pub(crate) fn frozen_place(rel: &Path) -> Option<crate::model::Frozen> {
    let parts = parts_under(rel, &milestones_dir())?;
    if parts.len() < 3 || parts[1] != HARVESTED_DIR {
        return None;
    }
    let milestone_dir = milestones_dir().join(&parts[0]);
    let slice = (parts.len() >= 4).then(|| parts[2].clone());
    Some(crate::model::Frozen {
        milestone: parts[0].clone(),
        milestone_dir,
        slice,
    })
}

/// Whether a path holds a plan document by its position: a markdown file inside a spec's
/// directory or a spec file of specs/, grouped or not, or a markdown file inside a milestone's
/// directory. The README and the index of the two homes are not, and no file other than
/// markdown is.
pub(crate) fn is_plan_position(rel: &Path) -> bool {
    if rel.extension().is_none_or(|e| e != "md") {
        return false;
    }
    if let Some(parts) = parts_under(rel, &specs_dir()) {
        return match parts.as_slice() {
            [name] => name != README && name != INDEX,
            _ => true,
        };
    }
    parts_under(rel, &milestones_dir()).is_some_and(|parts| parts.len() >= 2)
}

/// The files a plan document's `search/` holds, each the generated lines of one search, named by
/// its occasion: the grounding of its discussion, and its design audit.
pub(crate) const SEARCH_FILES: [&str; 2] = ["discussion.txt", "design-audit.txt"];

/// Whether `name` is a file a plan document's `search/` holds.
pub(crate) fn is_search_file(name: &str) -> bool {
    SEARCH_FILES.contains(&name)
}

/// Whether `name` is a file a plan document's directory holds beside its `README.md`, at its top:
/// its `deliberation.md` or its `transcript.txt`.
pub(crate) fn is_beside_readme(name: &str) -> bool {
    name == DELIBERATION_FILE || name == TRANSCRIPT_FILE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(list: &[&str]) -> Vec<PathBuf> {
        list.iter().map(PathBuf::from).collect()
    }

    /// The claim: a directory of specs/ holding a README.md is a spec, one holding none is a
    /// group whose headed directories are specs, and a spec file is no spec. Mutations checked:
    /// the group arm's `!top.contains` dropped, which reads a spec's subdirectory as a spec; the
    /// `not_files` retain dropped, which reads a README.md that is a directory as a head.
    #[test]
    fn a_spec_is_a_headed_directory_of_specs_or_of_a_group() {
        let tree = paths(&[
            "docs/plans/specs/a-spec/README.md",
            "docs/plans/specs/a-spec/deliberation.md",
            "docs/plans/specs/a-spec/inner/README.md",
            "docs/plans/specs/group/b-spec/README.md",
            "docs/plans/specs/c-spec.md",
            "docs/plans/specs/d-spec/README.md/x.md",
        ]);
        let found: Vec<(String, String)> = spec_dirs_all(&tree)
            .into_iter()
            .map(|s| (s.name, s.path.display().to_string()))
            .collect();
        assert_eq!(
            found,
            vec![
                ("a-spec".to_string(), "docs/plans/specs/a-spec".to_string()),
                (
                    "b-spec".to_string(),
                    "docs/plans/specs/group/b-spec".to_string()
                ),
            ]
        );
    }

    /// The claim: a milestone's slices are its headed directories but `search/` and
    /// `harvested/`, then the headed directories of `harvested/`, marked harvested. Mutations
    /// checked, in `slices`: `&& p[0] != SEARCH_DIR` dropped, `p[0] != HARVESTED_DIR &&` dropped,
    /// and the harvested slices marked live; each caught.
    #[test]
    fn a_slice_is_live_or_harvested_by_where_its_directory_sits() {
        let m = Path::new("docs/plans/milestones/m-one");
        let tree = paths(&[
            "docs/plans/milestones/m-one/README.md",
            "docs/plans/milestones/m-one/b-slice/README.md",
            "docs/plans/milestones/m-one/search/README.md",
            "docs/plans/milestones/m-one/harvested/a-slice/README.md",
            "docs/plans/milestones/m-one/harvested/README.md",
        ]);
        let found: Vec<(String, bool)> = slices(m, &tree)
            .into_iter()
            .map(|s| (s.name, s.harvested))
            .collect();
        assert_eq!(
            found,
            vec![
                ("b-slice".to_string(), false),
                ("a-slice".to_string(), true)
            ]
        );
    }

    /// The claim: every path under a milestone's `harvested/` is frozen, a file directly under it
    /// included, and nothing else is. Mutation checked: `parts.len() < 3` of `frozen_place` made
    /// `parts.len() < 4`, which leaves a file directly under `harvested/` live; caught.
    #[test]
    fn only_a_path_under_a_milestone_s_harvested_is_frozen() {
        assert!(is_frozen(Path::new(
            "docs/plans/milestones/m-one/harvested/a-slice/README.md"
        )));
        assert!(is_frozen(Path::new(
            "docs/plans/milestones/m-one/harvested/loose.md"
        )));
        assert!(is_frozen(Path::new(
            "docs/plans/milestones/m-one/harvested/a-slice/search/discussion.txt"
        )));
        assert!(!is_frozen(Path::new(
            "docs/plans/milestones/m-one/a-slice/README.md"
        )));
        assert!(!is_frozen(Path::new(
            "docs/plans/milestones/harvested/x/README.md"
        )));
        assert!(!is_frozen(Path::new(
            "docs/plans/specs/harvested/x/README.md"
        )));
    }

    /// The claim: a frozen file of a slice's directory names its milestone and its slice, and a
    /// file directly under `harvested/` is in no slice. Mutation checked: `parts.len() >= 4` made
    /// `parts.len() >= 3`, which puts the loose file in a slice named by its file name; caught.
    #[test]
    fn a_frozen_file_names_its_milestone_and_its_slice() {
        let frozen = frozen_place(Path::new(
            "docs/plans/milestones/m-one/harvested/a-slice/search/discussion.txt",
        ))
        .expect("frozen");
        assert_eq!(
            frozen.milestone_dir,
            Path::new("docs/plans/milestones/m-one")
        );
        assert_eq!(frozen.milestone, "m-one");
        assert_eq!(frozen.slice.as_deref(), Some("a-slice"));
        let loose = frozen_place(Path::new("docs/plans/milestones/m-one/harvested/loose.md"))
            .expect("frozen");
        assert_eq!(loose.slice, None);
    }

    /// The claim: where two specs share an id, the anchor is the first in path order, and the
    /// second is the duplicate entry finding. Mutation checked: `or_insert` of `spec_dirs` made
    /// `insert`, which keeps the last; caught.
    #[test]
    fn two_specs_of_one_id_anchor_the_first_in_path_order() {
        let tree = paths(&[
            "docs/plans/specs/a-group/a-spec/README.md",
            "docs/plans/specs/b-group/a-spec/README.md",
        ]);
        assert_eq!(
            spec_dirs(&tree).get("a-spec"),
            Some(&PathBuf::from("docs/plans/specs/a-group/a-spec"))
        );
    }

    /// The claim: a Markdown file of specs/ or of a milestone's directory is at a plan position,
    /// but the README and the index of specs/ and anything not Markdown. Mutation checked: the
    /// `[name]` arm of `is_plan_position` made `true`, which reads the home's README and index as
    /// plan documents; caught.
    #[test]
    fn the_homes_navigation_files_are_no_plan_position() {
        for (rel, at) in [
            ("docs/plans/specs/README.md", false),
            ("docs/plans/specs/index.md", false),
            ("docs/plans/specs/a-spec.md", true),
            ("docs/plans/specs/a-spec/README.md", true),
            ("docs/plans/specs/a-spec/transcript.txt", false),
            ("docs/plans/milestones/m-one/README.md", true),
            ("docs/plans/milestones/README.md", false),
        ] {
            assert_eq!(is_plan_position(Path::new(rel)), at, "{rel}");
        }
    }
}
