//! The work list of a search of the whole record: which entries are read together, and which
//! agent reads them.
//!
//! The record is every entry of every register but the plan registers. Its entries are grouped
//! into clusters, each rooted at a unit of a unit register, which is `design` or any register a
//! project declares: a unit holds the entries a piece of work reads together, a level-two subject
//! of a design home, or a group of a file register. A cluster adds to its unit the `rejected`
//! entries the unit's entries cite, the issues and tripwires that cite them, and the goals they
//! cite with every goal those refine. Every entry no cluster holds is in the residual group of
//! its anchor. The groups are then packed into agents under a bound in words.
//!
//! Everything here is a pure function of the model and the entity table, so a test states its
//! project as text, as the tests of `path@core@src/records.rs` do.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::entity::{Anchors, Entities, Kind};
use crate::manifest::{
    Registers, Shape, GOAL_REGISTER, ISSUE_REGISTER, REJECTED_REGISTER, TRIPWIRE_REGISTER,
};
use crate::model::{Document, Model};
use crate::records::{self, EntryKey, Record};
use crate::scan::Observation;

/// What an entry is in a search: a member of a unit, or one of the four kinds a cluster gathers
/// around its unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Role {
    Unit,
    Goal,
    Rejected,
    Issue,
    Tripwire,
}

impl Role {
    /// The word `agents.txt` writes for the role.
    pub(crate) fn word(self) -> &'static str {
        match self {
            Role::Unit => "unit",
            Role::Goal => "goal",
            Role::Rejected => "rejected",
            Role::Issue => "issue",
            Role::Tripwire => "tripwire",
        }
    }

    fn of(kind: &Kind) -> Role {
        match kind.name() {
            GOAL_REGISTER => Role::Goal,
            REJECTED_REGISTER => Role::Rejected,
            ISSUE_REGISTER => Role::Issue,
            TRIPWIRE_REGISTER => Role::Tripwire,
            _ => Role::Unit,
        }
    }
}

/// One entry of the record.
#[derive(Clone, Debug)]
pub(crate) struct Entry {
    pub key: EntryKey,
    pub role: Role,
    pub record: Record,
    /// The whitespace-separated tokens of the entry's body.
    pub words: usize,
}

impl Entry {
    /// The entry's reference, `<kind>@<anchor>@<id>`.
    pub(crate) fn reference(&self) -> String {
        let (kind, anchor, id) = &self.key;
        format!("{}@{anchor}@{id}", kind.name())
    }
}

/// A cluster, or the residual group of one anchor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Group {
    /// What a reader is told the group is: `<anchor> :: <heading>`, `<anchor> :: <path>` for a
    /// unit with no heading, or `<anchor> :: (residual)`.
    pub name: String,
    /// The order of two groups of one load: anchor, path, heading, each compared as bytes.
    pub key: (String, String, String),
    /// Indices into [`Graph::entries`]: the unit's entries in the order they are written, then
    /// the goals, the `rejected` entries, the issues and the tripwires, each by reference. A
    /// residual group's entries are by role, then by reference.
    pub entries: Vec<usize>,
    pub residual: bool,
}

/// Every entry of the record and every group it is read in.
#[derive(Clone, Debug)]
pub(crate) struct Graph {
    pub entries: Vec<Entry>,
    pub groups: Vec<Group>,
    /// For each entry, by index, the entries of the record it cites.
    pub cites: BTreeMap<usize, BTreeSet<usize>>,
}

impl Graph {
    /// The words of a group: each of its entries counted once.
    pub(crate) fn load(&self, group: &Group) -> usize {
        group.entries.iter().map(|&i| self.entries[i].words).sum()
    }
}

/// The kinds of the record: every register but the plan registers and the item registers.
pub(crate) fn record_kinds(registers: &Registers) -> Vec<Kind> {
    registers
        .all()
        .iter()
        .filter(|r| !Registers::is_plan_register(&r.name) && r.shape != Shape::Section)
        .map(|r| Kind::new(&r.name))
        .collect()
}

/// The record's entries, grouped into clusters and residual groups.
pub(crate) fn graph(model: &Model, anchors: &Anchors, entities: &Entities) -> Graph {
    let kinds = record_kinds(anchors.registers());
    let mut entries: Vec<Entry> = Vec::new();
    for kind in &kinds {
        for record in records::records(model, anchors, entities, kind) {
            let words = record.body.split_whitespace().count();
            entries.push(Entry {
                key: (kind.clone(), record.anchor.clone(), record.id.clone()),
                role: Role::of(kind),
                record,
                words,
            });
        }
    }
    let index: BTreeMap<EntryKey, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.key.clone(), i))
        .collect();
    let edges: Vec<(usize, usize)> = records::edges(model, anchors, entities, &kinds)
        .into_iter()
        .filter_map(|(from, to)| Some((*index.get(&from)?, *index.get(&to)?)))
        .collect();
    let mut cites: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut cited_by: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for &(from, to) in &edges {
        cites.entry(from).or_default().insert(to);
        cited_by.entry(to).or_default().insert(from);
    }

    let by_path: BTreeMap<&Path, &Document> = model
        .documents()
        .iter()
        .map(|d| (d.rel.as_path(), d))
        .collect();
    let mut units: BTreeMap<(String, String, String), (String, Vec<usize>)> = BTreeMap::new();
    for (i, entry) in entries.iter().enumerate() {
        if entry.role != Role::Unit {
            continue;
        }
        let (key, name) = unit_key(entry, anchors, &by_path);
        units
            .entry(key)
            .or_insert_with(|| (name, Vec::new()))
            .1
            .push(i);
    }

    let mut groups = Vec::new();
    let mut held: BTreeSet<usize> = BTreeSet::new();
    for (key, (unit_name, mut members)) in units {
        members.sort_by(|&a, &b| {
            let (sa, sb) = (&entries[a].record.site, &entries[b].record.site);
            (&sa.file, sa.line).cmp(&(&sb.file, sb.line))
        });
        let unit: BTreeSet<usize> = members.iter().copied().collect();
        let gathered = |role: Role| -> Vec<usize> {
            let mut out: BTreeSet<usize> = BTreeSet::new();
            for &m in &unit {
                let near = match role {
                    // A standing entry joins by citing the unit; a goal or a rejected entry by
                    // being cited from it.
                    Role::Issue | Role::Tripwire => cited_by.get(&m),
                    _ => cites.get(&m),
                };
                out.extend(
                    near.into_iter()
                        .flatten()
                        .filter(|&&n| entries[n].role == role),
                );
            }
            sorted_by_reference(out, &entries)
        };
        let goals = upward(&gathered(Role::Goal), &cites, &entries);
        let mut list = members.clone();
        list.extend(goals);
        list.extend(gathered(Role::Rejected));
        list.extend(gathered(Role::Issue));
        list.extend(gathered(Role::Tripwire));
        held.extend(list.iter().copied());
        groups.push(Group {
            name: format!("{} :: {unit_name}", key.0),
            key,
            entries: list,
            residual: false,
        });
    }

    let mut residual: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, entry) in entries.iter().enumerate() {
        if !held.contains(&i) {
            residual.entry(entry.key.1.clone()).or_default().push(i);
        }
    }
    for (anchor, mut members) in residual {
        members.sort_by(|&a, &b| {
            (entries[a].role, entries[a].reference())
                .cmp(&(entries[b].role, entries[b].reference()))
        });
        groups.push(Group {
            name: format!("{anchor} :: (residual)"),
            key: (anchor, String::new(), "(residual)".to_string()),
            entries: members,
            residual: true,
        });
    }
    Graph {
        entries,
        groups,
        cites,
    }
}

/// The unit an entry of a unit register belongs to, as its key and its name.
///
/// A heading register whose entries sit at level 3 or deeper is cut at the nearest heading above
/// an entry that is shallower than the entries, in each file of its home, so a design home is cut
/// at its level-two subjects, in a single file or in each subdocument. Each such heading is a unit
/// of its own, keyed by its line, so two subjects of one text stay two units. A unit is named by
/// its heading's text where the heading sits one level above the entries; the entries under a
/// file's first heading, its title, or under no heading are named by the file; under any other
/// shallower heading, by the file and that heading. A heading register at level 2 has no heading
/// above its entries but the title, so each file is one unit. A file register is cut at its group
/// subdirectories, and the entries at the top of its home are one unit. A Directory register's
/// entries are one unit per anchor that carries it.
fn unit_key(
    entry: &Entry,
    anchors: &Anchors,
    by_path: &BTreeMap<&Path, &Document>,
) -> ((String, String, String), String) {
    let (kind, anchor, _) = &entry.key;
    let register = anchors.registers().by_name(kind.name());
    let home = anchors.by_name(anchor).and_then(|a| anchors.home(a, kind));
    let site = &entry.record.site;
    let whole = |path: &Path| {
        let text = path_text(path);
        ((anchor.clone(), text.clone(), String::new()), text)
    };
    match register.map(|r| (r.shape, r.level)) {
        Some((Shape::File, _)) => {
            let dir = home.map(|h| h.dir).unwrap_or_default();
            match &entry.record.group {
                Some(group) => whole(&dir.join(group)),
                None => whole(&dir),
            }
        }
        Some((Shape::Directory, _)) => whole(&home.map(|h| h.dir).unwrap_or_default()),
        Some((_, Some(level))) if level >= 3 => {
            let file = path_text(&site.file);
            let above = by_path
                .get(site.file.as_path())
                .and_then(|doc| shallower_above(doc, site.line, level));
            match above {
                None => whole(&site.file),
                Some((at, n, text, first)) => {
                    let name = if n == level - 1 {
                        text
                    } else if first {
                        file.clone()
                    } else {
                        format!("{file} :: {text}")
                    };
                    ((anchor.clone(), file, format!("{at:010}")), name)
                }
            }
        }
        _ => whole(&site.file),
    }
}

/// The nearest heading above `line` shallower than `level`: its line, its level, its text, and
/// whether it is the document's first heading.
fn shallower_above(doc: &Document, line: u32, level: u8) -> Option<(u32, u8, String, bool)> {
    let headings: Vec<(u32, u8, &String)> = doc
        .observations
        .iter()
        .filter_map(|l| match &l.what {
            Observation::Heading { level: n, text } => Some((l.line, *n, text)),
            _ => None,
        })
        .collect();
    let first = headings.first().map(|h| h.0);
    headings
        .iter()
        .rev()
        .find(|(at, n, _)| *at < line && *n < level)
        .map(|&(at, n, text)| (at, n, text.clone(), Some(at) == first))
}

fn path_text(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn sorted_by_reference(set: BTreeSet<usize>, entries: &[Entry]) -> Vec<usize> {
    let mut out: Vec<usize> = set.into_iter().collect();
    out.sort_by_key(|&i| entries[i].reference());
    out
}

/// The goals `start` names, and every goal reached from them upward: from a goal to each goal it
/// cites, which is a goal it refines, transitively.
fn upward(
    start: &[usize],
    cites: &BTreeMap<usize, BTreeSet<usize>>,
    entries: &[Entry],
) -> Vec<usize> {
    let mut seen: BTreeSet<usize> = start.iter().copied().collect();
    let mut stack: Vec<usize> = start.to_vec();
    while let Some(goal) = stack.pop() {
        for &next in cites.get(&goal).into_iter().flatten() {
            if entries[next].role == Role::Goal && seen.insert(next) {
                stack.push(next);
            }
        }
    }
    sorted_by_reference(seen, entries)
}

/// Which agent reads which groups, by index into the loads given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Packing {
    /// One list of group indices per agent, in the order the agents are numbered from 1: the
    /// agents of the groups within the bound, then one agent per group above it.
    pub agents: Vec<Vec<usize>>,
    /// The groups above the bound, each sent to an agent alone.
    pub above: Vec<usize>,
}

/// Packs groups into the fewest agents first-fit-decreasing allows, then spreads them over that
/// many agents, so that the loads sit near their mean rather than at the bound.
///
/// `loads` holds each group's load and the key that orders two groups of one load. A group above
/// `bound` goes to an agent alone. The others are placed largest first: first-fit-decreasing
/// fixes the number of agents, then each group goes to the least-loaded agent it fits in, the
/// lowest-numbered among equals. Where the spreading cannot place a group under the bound, the
/// first-fit-decreasing assignment is kept, so the number of agents never grows. One input gives
/// one packing.
pub(crate) fn pack(loads: &[(usize, (String, String, String))], bound: usize) -> Packing {
    let mut order: Vec<usize> = (0..loads.len()).collect();
    order.sort_by(|&a, &b| {
        loads[b]
            .0
            .cmp(&loads[a].0)
            .then_with(|| loads[a].1.cmp(&loads[b].1))
    });
    let (above, within): (Vec<usize>, Vec<usize>) =
        order.into_iter().partition(|&g| loads[g].0 > bound);

    let mut fit: Vec<(usize, Vec<usize>)> = Vec::new();
    for &g in &within {
        match fit.iter_mut().find(|(load, _)| load + loads[g].0 <= bound) {
            Some((load, members)) => {
                *load += loads[g].0;
                members.push(g);
            }
            None => fit.push((loads[g].0, vec![g])),
        }
    }
    let n = fit.len();
    let mut spread: Vec<(usize, Vec<usize>)> = vec![(0, Vec::new()); n];
    let mut spread_holds = true;
    for &g in &within {
        let target = (0..n)
            .filter(|&a| spread[a].0 + loads[g].0 <= bound)
            .min_by_key(|&a| (spread[a].0, a));
        match target {
            Some(a) => {
                spread[a].0 += loads[g].0;
                spread[a].1.push(g);
            }
            None => {
                spread_holds = false;
                break;
            }
        }
    }
    let chosen = if spread_holds { spread } else { fit };
    let mut agents: Vec<Vec<usize>> = chosen.into_iter().map(|(_, members)| members).collect();
    agents.extend(above.iter().map(|&g| vec![g]));
    Packing { agents, above }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Manifest;
    use std::path::PathBuf;

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `design@core@checker-source-literals-are-data`.

    /// A project with the root Component `a-project`, a Component `widget` whose design home is a
    /// directory, and a location `notes` carrying a declared file register `reading` with groups,
    /// a declared heading register `rule` at level 2, and one `law` at level 4.
    fn manifest() -> Manifest {
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\n\
             components = [\"widget\"]\n\n\
             [locations.notes]\npath = \"notes\"\nregisters = [\"issue\", \"reading\", \"rule\", \"law\", \"topic\"]\n\n\
             [registers.reading]\nscope = \"opt-in\"\nshape = \"file\"\ndir = \"readings\"\n\
             sections = [\"Reading\"]\n\n\
             [registers.rule]\nscope = \"opt-in\"\nshape = \"heading\"\ndir = \"rules\"\nlevel = 2\n\n\
             [registers.law]\nscope = \"opt-in\"\nshape = \"heading\"\ndir = \"laws\"\nlevel = 4\n\n\
             [registers.topic]\nscope = \"opt-in\"\nshape = \"directory\"\ndir = \"topics\"\n\
             carries = [\"rule\"]\n\n\
             [agents]\nharness = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n";
        Manifest::parse(Path::new("/nowhere"), text).expect("a declaration")
    }

    fn graph_of(docs: &[(&str, &str)]) -> Graph {
        let manifest = manifest();
        let model = Model::from_documents(
            docs.iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        // A model built from text has no listing, so the anchors read the documents' paths, which
        // is where a Directory register's entries are placed.
        let paths: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let anchors = Anchors::of(&manifest, &paths);
        let entities = Entities::build(&model, &anchors);
        graph(&model, &anchors, &entities)
    }

    fn names(graph: &Graph, group: &Group) -> Vec<String> {
        group
            .entries
            .iter()
            .map(|&i| graph.entries[i].reference())
            .collect()
    }

    fn group<'a>(graph: &'a Graph, name: &str) -> &'a Group {
        graph
            .groups
            .iter()
            .find(|g| g.name == name)
            .unwrap_or_else(|| panic!("no group {name}: {:?}", graph.groups))
    }

    // The root's design home, one file: a head before any subject, then two subjects.
    const ROOT_DESIGN: &str = "# Design\n\n\
        ### A head before any subject `##early-head`\n\nText.\n\n\
        ## First subject\n\nProse that cites `design@a-project@other-head`, outside every entry.\n\n\
        ### A first head `##first-head`\n\nIt serves `goal@a-project@fine-goal` and covers \
        `rejected@a-project@lost-shape`.\n\n\
        ## Second subject\n\n\
        ### Another head `##other-head`\n\nIt cites `design@a-project@first-head`.\n";

    const ROOT_GOALS: &str = "# Goals\n\n\
        ## A fine goal `##fine-goal`\n\nIt refines `goal@a-project@middle-goal`.\n\n\
        ## A middle goal `##middle-goal`\n\nIt refines `goal@a-project@top-goal`.\n\n\
        ## A top goal `##top-goal`\n\nThe top.\n\n\
        ## An unserved goal `##unserved-goal`\n\nNothing serves it.\n";

    const ROOT_REJECTED: &str =
        "# Rejected\n\n### A lost shape `##lost-shape`\n\n`live`. It lost.\n";

    const ROOT_TRIPWIRES: &str = "# Tripwires\n\n\
        ## Guarding `design@a-project@first-head`: a premise `##first-premise`\n\nIt fires.\n";

    const ISSUE_ON_HEAD: &str = "---\nkind: todo\n---\n# On the head\n\n## Summary\n\n\
        It strains `design@a-project@other-head`.\n\n## Details\n\n### What\n\nx\n\n\
        ### Why it matters\n\nx\n\n### What would close it\n\nx\n";

    const ISSUE_ON_GOAL: &str = "---\nkind: todo\n---\n# On the goal\n\n## Summary\n\n\
        It threatens `goal@a-project@fine-goal`.\n\n## Details\n\n### What\n\nx\n\n\
        ### Why it matters\n\nx\n\n### What would close it\n\nx\n";

    fn project() -> Vec<(&'static str, &'static str)> {
        vec![
            ("docs/design.md", ROOT_DESIGN),
            ("docs/goals.md", ROOT_GOALS),
            ("docs/rejected.md", ROOT_REJECTED),
            ("docs/tripwires.md", ROOT_TRIPWIRES),
            ("docs/open-issues/on-the-head.md", ISSUE_ON_HEAD),
            ("docs/open-issues/on-the-goal.md", ISSUE_ON_GOAL),
            ("widget/docs/design/README.md", "# Widget design\n\n- [Parts](parts.md)\n"),
            (
                "widget/docs/design/parts.md",
                "# Parts\n\n## The gears\n\n### Gears turn `##gears-turn`\n\nThey do.\n\n\
                 ## The springs\n\n### Springs coil `##springs-coil`\n\nThey do.\n",
            ),
            ("notes/readings/top-reading.md", "# Top\n\n## Reading\n\nx\n"),
            ("notes/readings/loops/loop-reading.md", "# Loop\n\n## Reading\n\nx\n"),
            ("notes/readings/loops/other-loop.md", "# Other\n\n## Reading\n\nx\n"),
            ("notes/topics/README.md", "# Topics\n"),
            ("notes/topics/alpha-topic/README.md", "# Alpha\n\nx\n"),
            ("notes/topics/beta-topic/README.md", "# Beta\n\nx\n"),
            (
                "notes/rules.md",
                "# Rules\n\n## A first rule `##first-rule`\n\nx\n\n## A second rule `##second-rule`\n\nx\n",
            ),
            (
                "notes/laws.md",
                "# Laws\n\n## Old laws\n\n### Of the old\n\n#### An old law `##old-law`\n\nx\n\n\
                 ### Of the new\n\n#### A new law `##new-law`\n\nx\n",
            ),
        ]
    }

    /// The claim: `edges` reads a reference inside the citing entry's section only, so prose
    /// under a subject before its first head yields no edge, and an entry's own slug is no edge.
    /// Mutation checked: the section range in `records::edges` widened to the whole document,
    /// which gives `first-head` an edge to `other-head` from the subject's prose.
    #[test]
    fn an_edge_is_read_inside_the_citing_entry_only() {
        let manifest = manifest();
        let model = Model::from_documents(
            project()
                .iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        let anchors = Anchors::of(&manifest, model.listing());
        let entities = Entities::build(&model, &anchors);
        let kinds = record_kinds(anchors.registers());
        let edges = records::edges(&model, &anchors, &entities, &kinds);
        let pair = |a: &str, b: &str| {
            let k = |r: &str| {
                let mut parts = r.split('@');
                let kind = Kind::new(parts.next().expect("a kind"));
                (
                    kind,
                    parts.next().expect("an anchor").to_string(),
                    parts.next().expect("an id").to_string(),
                )
            };
            (k(a), k(b))
        };
        assert!(edges.contains(&pair(
            "design@a-project@other-head",
            "design@a-project@first-head"
        )));
        assert!(!edges.contains(&pair(
            "design@a-project@first-head",
            "design@a-project@other-head"
        )));
        assert!(!edges.contains(&pair(
            "design@a-project@early-head",
            "design@a-project@other-head"
        )));
    }

    /// The claim: every entry of a unit register is in exactly one unit, by the rule of its
    /// register's shape and level. Mutations checked: `subject_of` returning the nearest shallower
    /// heading whatever its level, which puts `early-head` under the title rather than alone; and
    /// a Directory register's unit keyed by the entry's file, which gives each topic a unit.
    #[test]
    fn each_unit_entry_is_in_one_unit_by_its_register_s_rule() {
        let g = graph_of(&project());
        let mut seen: BTreeMap<String, usize> = BTreeMap::new();
        for grp in g.groups.iter().filter(|grp| !grp.residual) {
            for &i in &grp.entries {
                if g.entries[i].role == Role::Unit {
                    *seen.entry(g.entries[i].reference()).or_default() += 1;
                }
            }
        }
        for e in g.entries.iter().filter(|e| e.role == Role::Unit) {
            assert_eq!(seen.get(&e.reference()), Some(&1), "{}", e.reference());
        }
        let unit = |name: &str| -> Vec<String> {
            names(&g, group(&g, name))
                .into_iter()
                .filter(|r| {
                    !r.starts_with("goal@")
                        && !r.starts_with("rejected@")
                        && !r.starts_with("issue@")
                        && !r.starts_with("tripwire@")
                })
                .collect()
        };
        assert_eq!(
            unit("a-project :: docs/design.md"),
            ["design@a-project@early-head"]
        );
        assert_eq!(
            unit("a-project :: First subject"),
            ["design@a-project@first-head"]
        );
        assert_eq!(unit("widget :: The gears"), ["design@widget@gears-turn"]);
        assert_eq!(
            unit("widget :: The springs"),
            ["design@widget@springs-coil"]
        );
        assert_eq!(
            unit("notes :: notes/readings"),
            ["reading@notes@top-reading"]
        );
        assert_eq!(
            unit("notes :: notes/readings/loops"),
            ["reading@notes@loop-reading", "reading@notes@other-loop"]
        );
        assert_eq!(
            unit("notes :: notes/rules.md"),
            ["rule@notes@first-rule", "rule@notes@second-rule"]
        );
        assert_eq!(unit("notes :: Of the old"), ["law@notes@old-law"]);
        assert_eq!(unit("notes :: Of the new"), ["law@notes@new-law"]);
        assert_eq!(
            unit("notes :: notes/topics"),
            ["topic@notes@alpha-topic", "topic@notes@beta-topic"]
        );
    }

    /// The claim: a cluster holds the goals its unit cites and every goal reached from them
    /// upward, the rejected entries it cites and the standing entries that cite it, and a goal
    /// pulls in no issue. Mutations checked: `upward` returning `start` alone, which drops the
    /// middle and top goals; and the issues that cite the cluster's goals added to it, which pulls
    /// the issue on the goal in.
    #[test]
    fn a_cluster_gathers_upward_goals_and_its_citers_and_no_goal_s_issue() {
        let g = graph_of(&project());
        assert_eq!(
            names(&g, group(&g, "a-project :: First subject")),
            [
                "design@a-project@first-head",
                "goal@a-project@fine-goal",
                "goal@a-project@middle-goal",
                "goal@a-project@top-goal",
                "rejected@a-project@lost-shape",
                "tripwire@a-project@first-premise",
            ]
        );
        assert_eq!(
            names(&g, group(&g, "a-project :: Second subject")),
            ["design@a-project@other-head", "issue@a-project@on-the-head"]
        );
    }

    /// The claim: every entry of the record is in a cluster or in its anchor's residual group,
    /// and the residual holds exactly what no cluster holds. Mutation checked: the residual loop
    /// skipping goals, which leaves the unserved goal in no group.
    #[test]
    fn every_entry_is_in_a_cluster_or_its_anchor_s_residual() {
        let g = graph_of(&project());
        let mut covered: BTreeSet<usize> = BTreeSet::new();
        for grp in &g.groups {
            covered.extend(grp.entries.iter().copied());
        }
        assert_eq!(covered.len(), g.entries.len());
        assert_eq!(
            names(&g, group(&g, "a-project :: (residual)")),
            [
                "goal@a-project@unserved-goal",
                "issue@a-project@on-the-goal"
            ]
        );
    }

    /// The claim: two subjects of one text in one file are two units, and an entry under a
    /// heading shallower than the subjects, after them, is a unit of its own, named by the file
    /// and that heading, not a member of the file's first unit. Mutation checked: the heading's
    /// line dropped from the unit's key, which merges the two `Commands` subjects.
    #[test]
    fn subjects_of_one_text_stay_apart_and_a_later_shallower_heading_is_its_own_unit() {
        let design = "# Design\n\n### Early `##early`\n\nx\n\n\
            ## Commands\n\n### Head a `##head-a`\n\nx\n\n\
            ## Storage\n\n### Head b `##head-b`\n\nx\n\n\
            ## Commands\n\n### Head c `##head-c`\n\nx\n\n\
            # Part two\n\n### Head e `##head-e`\n\nx\n";
        let mut docs = project();
        docs.retain(|(p, _)| *p != "docs/design.md");
        docs.push(("docs/design.md", design));
        let g = graph_of(&docs);
        let commands: Vec<Vec<String>> = g
            .groups
            .iter()
            .filter(|grp| grp.name == "a-project :: Commands")
            .map(|grp| names(&g, grp))
            .collect();
        assert_eq!(
            commands,
            [["design@a-project@head-a"], ["design@a-project@head-c"]]
        );
        assert_eq!(
            names(&g, group(&g, "a-project :: docs/design.md :: Part two")),
            ["design@a-project@head-e"]
        );
        assert_eq!(
            names(&g, group(&g, "a-project :: docs/design.md")),
            ["design@a-project@early"]
        );
    }

    fn key(name: &str) -> (String, String, String) {
        (name.to_string(), String::new(), String::new())
    }

    /// The claim: the packing uses as many agents as first-fit-decreasing, spreads the groups
    /// within the bound over them, and sends a group above the bound alone, after them.
    /// Mutations checked: the spreading dropped, so the agents keep first-fit's loads of 10 and
    /// 4; and the least-loaded agent replaced by the first that fits, which puts `c` with `a`.
    #[test]
    fn the_packing_spreads_over_the_fewest_agents_and_sends_an_oversized_group_alone() {
        let loads = vec![
            (6, key("a")),
            (4, key("b")),
            (4, key("c")),
            (12, key("d")),
            (0, key("e")),
        ];
        // First-fit-decreasing at 10: [a 6, b 4, e 0], [c 4], and d alone: 2 agents within.
        // The spread: a to agent 1, b to agent 2, c to agent 2, at 4 the less loaded though
        // agent 1 at 6 + 4 = 10 fits too, and e to agent 1, at 6 the less loaded.
        let packing = pack(&loads, 10);
        assert_eq!(packing.above, [3]);
        assert_eq!(packing.agents, vec![vec![0, 4], vec![1, 2], vec![3]]);
    }

    /// The claim: where the spreading cannot place a group under the bound, the
    /// first-fit-decreasing assignment stands, so the number of agents never grows. Mutation
    /// checked: the fallback removed, which leaves the spreading's partial assignment.
    #[test]
    fn the_packing_keeps_first_fit_where_the_spreading_cannot_fit() {
        // At 10: first-fit gives [a 5, b 5], [c 4, d 3, e 3]: 2 agents. The spread gives a and
        // b to the two agents, c to agent 1 (9), d to agent 2 (8), and e fits in neither (12,
        // 11): first-fit stands.
        let loads = vec![
            (5, key("a")),
            (5, key("b")),
            (4, key("c")),
            (3, key("d")),
            (3, key("e")),
        ];
        let packing = pack(&loads, 10);
        assert_eq!(packing.agents, vec![vec![0, 1], vec![2, 3, 4]]);
    }

    /// The claim: two groups of one load are ordered by their key, so one input gives one
    /// packing whatever the order it is given in. Mutation checked: the key tie-break removed
    /// from the sort, which keeps the input order.
    #[test]
    fn equal_loads_are_ordered_by_key() {
        let one = pack(&[(5, key("b")), (5, key("a"))], 5);
        let two = pack(&[(5, key("a")), (5, key("b"))], 5);
        assert_eq!(one.agents, vec![vec![1], vec![0]]);
        assert_eq!(two.agents, vec![vec![0], vec![1]]);
    }
}
