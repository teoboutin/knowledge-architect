//! The work list of a search of the whole record: which entries are read together, and which
//! agent reads them.
//!
//! The record is every entry of every register but the plan registers. Every Component and every
//! location the manifest declares is a **search unit**, and an entry belongs to the search unit of
//! its anchor; an anchor the tool constructs, the entry anchor of a Directory register or `plans`,
//! belongs to the search unit that holds its home. Inside each search unit, its entries are grouped
//! into clusters, each rooted at a unit of a unit register, which is `design` or any register a
//! project declares: a unit holds the entries a piece of work reads together, a level-two subject
//! of a design home, or a group of a file register. A cluster adds to its unit the `rejected`
//! entries of its search unit that the unit's entries cite, and the issues and tripwires of its
//! search unit that cite them. What no cluster holds, its goals apart, is the search unit's
//! remainder, one group. The goals are grouped apart, one group per goals file of a search unit,
//! with the file's prose above its first goal, and read by goal agents.
//!
//! A cluster agent is given, beside the entries it grades, two things it reads and does not grade:
//! the **context entries**, every head, entry of a declared register or `rejected` entry an entry
//! it grades cites and it does not grade itself, whole; and the **title lines** of the goals of the root, of each Component
//! enclosing a search unit it holds, and of those search units.
//!
//! The groups are then packed into agents under a bound in words, by search unit, as [`plan`]
//! says. Everything here is a pure function of the model and the entity table, so a test states its
//! project as text, as the tests of `path@model@src/records.rs` do.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::model::declared::{
    Registers, Shape, GOAL_REGISTER, ISSUE_REGISTER, REJECTED_REGISTER, TRIPWIRE_REGISTER,
};
use crate::model::document::Observation;
use crate::model::table::{self, EntryKey, Record};
use crate::model::table::{Anchors, Entities, Kind};
use crate::model::{Document, Model};

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

    /// Whether an entry of this role is given whole as a context entry where an entry an agent
    /// grades cites it: a head, an entry of a declared register, or a `rejected` entry.
    fn is_context(self) -> bool {
        matches!(self, Role::Unit | Role::Rejected)
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
    /// The search unit the entry belongs to, by its anchor's name.
    pub unit: String,
}

impl Entry {
    /// The entry's reference, `<kind>@<anchor>@<id>`.
    pub(crate) fn reference(&self) -> String {
        let (kind, anchor, id) = &self.key;
        format!("{}@{anchor}@{id}", kind.name())
    }

    /// The words of the entry's title line, the heading's statement without its slug.
    fn title_words(&self) -> usize {
        self.record.title.split_whitespace().count()
    }
}

/// What a group is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GroupKind {
    /// A unit with what it gathers.
    Cluster,
    /// What no cluster of a search unit holds, its goals apart.
    Remainder,
    /// The goals of one goals file of a search unit.
    Goals,
}

/// A cluster, the remainder of a search unit, or a file of its goals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Group {
    /// What a reader is told the group is: for a cluster, `<anchor> :: <heading>`, or
    /// `<anchor> :: <path>` for a unit with no heading, the anchor being its unit's; for a goals
    /// file, `<search unit> :: <path>`; for a remainder, `<search unit> :: (remainder)`.
    pub name: String,
    /// The order of two groups of one load, each part compared as bytes: for a cluster, its unit's
    /// anchor, path and heading; for a goals file, its search unit and its path; for a remainder,
    /// its search unit and `(remainder)`.
    pub key: (String, String, String),
    /// The search unit the group belongs to.
    pub unit: String,
    pub kind: GroupKind,
    /// The entries the group's agent grades, indices into [`Graph::entries`]: a cluster's unit
    /// entries in the order they are written, then its `rejected` entries, its issues and its
    /// tripwires, each by reference; a remainder's by role, then by reference; a goals file's in
    /// the order they are written.
    pub entries: Vec<usize>,
    /// For a goals file, its text above its first goal; empty for the other kinds.
    pub prose: String,
}

/// Every entry of the record and every group it is read in.
#[derive(Clone, Debug)]
pub(crate) struct Graph {
    pub entries: Vec<Entry>,
    pub groups: Vec<Group>,
    /// For each entry, by index, the entries of the record it cites.
    pub cites: BTreeMap<usize, BTreeSet<usize>>,
    /// For each search unit, by name, the goals whose title lines an agent holding it is given:
    /// the root's, each enclosing Component's from the outermost, then its own, each in the order
    /// they are written.
    pub titles: BTreeMap<String, Vec<usize>>,
}

impl Graph {
    /// The entries the groups grade, each once.
    fn graded(&self, groups: &[usize]) -> BTreeSet<usize> {
        groups
            .iter()
            .flat_map(|&g| self.groups[g].entries.iter().copied())
            .collect()
    }

    /// The context entries of an agent given `groups`: every head, entry of a declared register or
    /// `rejected` entry an entry they grade cites, that none of them grades, by reference.
    pub(crate) fn context(&self, groups: &[usize]) -> Vec<usize> {
        let graded = self.graded(groups);
        let mut out: BTreeSet<usize> = BTreeSet::new();
        for &i in &graded {
            out.extend(
                self.cites
                    .get(&i)
                    .into_iter()
                    .flatten()
                    .filter(|&&c| self.entries[c].role.is_context() && !graded.contains(&c)),
            );
        }
        sorted_by_reference(out, &self.entries)
    }

    /// The goals whose title lines an agent given `groups` holds: those of the search unit of
    /// each cluster and remainder among them, each once, in the order of the groups. A goal
    /// agent holds none, since it reads its goals whole.
    pub(crate) fn title_lines(&self, groups: &[usize]) -> Vec<usize> {
        let mut seen: BTreeSet<usize> = BTreeSet::new();
        let mut out = Vec::new();
        for &g in groups {
            let group = &self.groups[g];
            if group.kind == GroupKind::Goals {
                continue;
            }
            for &goal in self.titles.get(&group.unit).into_iter().flatten() {
                if seen.insert(goal) {
                    out.push(goal);
                }
            }
        }
        out
    }

    /// The load of one agent given `groups`, as if they were one group: the words of the entries
    /// it grades, of its context entries and of its title lines, each once, and the prose of each
    /// goals file among them.
    pub(crate) fn load(&self, groups: &[usize]) -> usize {
        let graded: usize = self
            .graded(groups)
            .iter()
            .map(|&i| self.entries[i].words)
            .sum();
        let context: usize = self
            .context(groups)
            .iter()
            .map(|&i| self.entries[i].words)
            .sum();
        let titles: usize = self
            .title_lines(groups)
            .iter()
            .map(|&i| self.entries[i].title_words())
            .sum();
        let distinct: BTreeSet<usize> = groups.iter().copied().collect();
        let prose: usize = distinct
            .iter()
            .map(|&g| self.groups[g].prose.split_whitespace().count())
            .sum();
        graded + context + titles + prose
    }

    /// The words of the entries one group grades, which the notice of a group above the bound
    /// weighs: a split of its subject reduces them, and not its context.
    pub(crate) fn own_words(&self, group: usize) -> usize {
        self.groups[group]
            .entries
            .iter()
            .map(|&i| self.entries[i].words)
            .sum()
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

/// The record's entries, grouped into clusters, remainders and goals files, each inside its
/// search unit.
pub(crate) fn graph(model: &Model, anchors: &Anchors, entities: &Entities) -> Graph {
    let kinds = record_kinds(anchors.registers());
    let mut entries: Vec<Entry> = Vec::new();
    for kind in &kinds {
        for record in table::records(model, anchors, entities, kind) {
            let words = record.body.split_whitespace().count();
            let unit = search_unit(anchors, &record.anchor);
            entries.push(Entry {
                key: (kind.clone(), record.anchor.clone(), record.id.clone()),
                role: Role::of(kind),
                record,
                words,
                unit,
            });
        }
    }
    let index: BTreeMap<EntryKey, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.key.clone(), i))
        .collect();
    let edges: Vec<(usize, usize)> = table::edges(model, anchors, entities, &kinds)
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
        // Every member of a unit has the unit's anchor, so one search unit.
        let search = entries[members[0]].unit.clone();
        let unit: BTreeSet<usize> = members.iter().copied().collect();
        let gathered = |role: Role| -> Vec<usize> {
            let mut out: BTreeSet<usize> = BTreeSet::new();
            for &m in &unit {
                let near = match role {
                    // A standing entry joins by citing the unit; a rejected entry by being cited
                    // from it.
                    Role::Issue | Role::Tripwire => cited_by.get(&m),
                    _ => cites.get(&m),
                };
                // Only the search unit's own: an entry of another search unit is graded there,
                // and reaches this cluster's agent as a context entry where the cluster cites it.
                out.extend(
                    near.into_iter()
                        .flatten()
                        .filter(|&&n| entries[n].role == role && entries[n].unit == search),
                );
            }
            sorted_by_reference(out, &entries)
        };
        let mut list = members.clone();
        list.extend(gathered(Role::Rejected));
        list.extend(gathered(Role::Issue));
        list.extend(gathered(Role::Tripwire));
        held.extend(list.iter().copied());
        groups.push(Group {
            name: format!("{} :: {unit_name}", key.0),
            key,
            unit: search,
            kind: GroupKind::Cluster,
            entries: list,
            prose: String::new(),
        });
    }

    let mut remainder: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut goals: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (i, entry) in entries.iter().enumerate() {
        if entry.role == Role::Goal {
            goals
                .entry((entry.unit.clone(), path_text(&entry.record.site.file)))
                .or_default()
                .push(i);
        } else if !held.contains(&i) {
            remainder.entry(entry.unit.clone()).or_default().push(i);
        }
    }
    for (unit, mut members) in remainder {
        members.sort_by(|&a, &b| {
            (entries[a].role, entries[a].reference())
                .cmp(&(entries[b].role, entries[b].reference()))
        });
        groups.push(Group {
            name: format!("{unit} :: (remainder)"),
            key: (unit.clone(), String::new(), "(remainder)".to_string()),
            unit,
            kind: GroupKind::Remainder,
            entries: members,
            prose: String::new(),
        });
    }
    let mut titles: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for ((unit, file), mut members) in goals {
        members.sort_by_key(|&i| entries[i].record.site.line);
        let first = entries[members[0]].record.site.line;
        let prose = by_path
            .get(entries[members[0]].record.site.file.as_path())
            .map(|doc| above_line(&doc.text, first))
            .unwrap_or_default();
        groups.push(Group {
            name: format!("{unit} :: {file}"),
            key: (unit.clone(), file, String::new()),
            unit,
            kind: GroupKind::Goals,
            entries: members,
            prose,
        });
    }
    for anchor in anchors.all().iter().filter(|a| a.constructed.is_none()) {
        let mut list: Vec<usize> = Vec::new();
        for owner in title_units(anchors, &anchor.name) {
            let mut own: Vec<usize> = (0..entries.len())
                .filter(|&i| entries[i].role == Role::Goal && entries[i].unit == owner)
                .collect();
            own.sort_by(|&a, &b| {
                let (sa, sb) = (&entries[a].record.site, &entries[b].record.site);
                (&sa.file, sa.line).cmp(&(&sb.file, sb.line))
            });
            list.extend(own);
        }
        titles.insert(anchor.name.clone(), list);
    }
    Graph {
        entries,
        groups,
        cites,
        titles,
    }
}

/// The search unit of the anchor `name`: the anchor itself where the manifest declares it; for an
/// entry anchor of a Directory register, the search unit of the anchor that carries the register;
/// for another anchor the tool constructs, `plans` or a spec, the deepest declared anchor whose path
/// holds it, a Component on a tie, as `Anchors::owning` decides a document's anchor.
fn search_unit(anchors: &Anchors, name: &str) -> String {
    let mut current = anchors.by_name(name);
    while let Some(anchor) = current {
        if anchor.constructed.is_none() {
            return anchor.name.clone();
        }
        current = match &anchor.entry_of {
            Some(of) => anchors.by_name(&of.parent),
            None => anchors
                .all()
                .iter()
                .filter(|d| d.constructed.is_none() && anchor.path.starts_with(&d.path))
                .max_by_key(|d| (d.path.components().count(), d.is_component)),
        };
    }
    // An anchor no declaration reaches, which the entity table never places an entry in.
    name.to_string()
}

/// The search units whose goals an agent holding `unit` is given as title lines: the root, each
/// Component enclosing `unit` from the outermost, and `unit` itself.
fn title_units(anchors: &Anchors, unit: &str) -> Vec<String> {
    let Some(own) = anchors.by_name(unit) else {
        return Vec::new();
    };
    let mut enclosing: Vec<&table::Anchor> = anchors
        .all()
        .iter()
        .filter(|c| {
            c.is_component
                && c.constructed.is_none()
                && c.name != own.name
                && own.path.starts_with(&c.path)
        })
        .collect();
    enclosing.sort_by_key(|c| c.path.components().count());
    let mut out: Vec<String> = enclosing.iter().map(|c| c.name.clone()).collect();
    out.push(own.name.clone());
    out
}

/// The lines of `text` above the 1-based line `line`.
fn above_line(text: &str, line: u32) -> String {
    let n = (line as usize).saturating_sub(1);
    text.lines().take(n).collect::<Vec<_>>().join("\n")
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

/// One agent of a plan: the groups it is given, and the items they were packed as, each with its
/// label and its load.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Agent {
    /// Indices into [`Graph::groups`], in the order the agent reads them.
    pub groups: Vec<usize>,
    /// `unit <name>` for a search unit packed whole, or a group's name for a group of a split
    /// search unit, each with the load `pack` weighed it by. Their sum is the agent's summed load.
    pub items: Vec<(String, usize)>,
}

/// Which agent reads which groups.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Plan {
    /// The cluster agents, then the goal agents, numbered from 1 in this order.
    pub agents: Vec<Agent>,
    /// The groups of a split search unit whose load is above the bound, each on an agent alone.
    pub above: Vec<usize>,
}

/// Packs the groups into agents under `bound`, the bound left once the work's words are counted,
/// clusters and remainders into cluster agents and goals files into goal agents, by search unit.
///
/// A search unit's load is [`Graph::load`] of all its groups of one family. A search unit whose
/// load exceeds half the bound gets agents of its own: one where its load fits the bound, else its
/// groups spread by [`pack`] over as many agents as the bound needs, after [`assign`] has left each
/// of its entries in one cluster. The search units whose load is at most half the bound are packed
/// together whole by [`pack`], each as one item of its load. Half the bound is the one point where
/// the rule only keeps a small search unit out of a large one's agent: two search units above it
/// can never share an agent anyway. The agents of the shared packing come first, then those of each
/// search unit with agents of its own, in the order of the search units' names.
pub(crate) fn plan(graph: &mut Graph, bound: usize) -> Plan {
    let mut agents: Vec<Agent> = Vec::new();
    let mut above: Vec<usize> = Vec::new();
    for goals in [false, true] {
        let mut by_unit: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (g, group) in graph.groups.iter().enumerate() {
            if (group.kind == GroupKind::Goals) == goals {
                by_unit.entry(group.unit.clone()).or_default().push(g);
            }
        }
        let mut small: Vec<(String, Vec<usize>, usize)> = Vec::new();
        let mut large: Vec<(String, Vec<usize>, usize)> = Vec::new();
        for (unit, groups) in by_unit {
            let load = graph.load(&groups);
            if load * 2 > bound {
                large.push((unit, groups, load));
            } else {
                small.push((unit, groups, load));
            }
        }
        let loads: Vec<(usize, (String, String, String))> = small
            .iter()
            .map(|(unit, _, load)| (*load, (unit.clone(), String::new(), String::new())))
            .collect();
        for members in pack(&loads, bound).agents {
            agents.push(Agent {
                groups: members
                    .iter()
                    .flat_map(|&u| small[u].1.iter().copied())
                    .collect(),
                items: members
                    .iter()
                    .map(|&u| (format!("unit {}", small[u].0), small[u].2))
                    .collect(),
            });
        }
        for (unit, groups, load) in large {
            if load <= bound {
                agents.push(Agent {
                    groups,
                    items: vec![(format!("unit {unit}"), load)],
                });
                continue;
            }
            assign(graph, &groups);
            let loads: Vec<(usize, (String, String, String))> = groups
                .iter()
                .map(|&g| (graph.load(&[g]), graph.groups[g].key.clone()))
                .collect();
            let packing = pack(&loads, bound);
            for members in packing.agents {
                agents.push(Agent {
                    groups: members.iter().map(|&i| groups[i]).collect(),
                    items: members
                        .iter()
                        .map(|&i| (graph.groups[groups[i]].name.clone(), loads[i].0))
                        .collect(),
                });
            }
            above.extend(packing.above.iter().map(|&i| groups[i]));
        }
    }
    Plan { agents, above }
}

/// Leaves each entry of a split search unit in one of its clusters, before they are packed apart,
/// so that every entry is graded in exactly one agent.
///
/// A unit's own entries are in one cluster already; an issue or a tripwire that cites heads of
/// several clusters, and a `rejected` entry that heads of several clusters cite, are in each of
/// them. Each stays in the cluster it is most tied to, the first in the search unit's order on a
/// tie: a standing entry in the cluster holding most of the heads it cites; a `rejected` entry in
/// the cluster holding most of the heads that cite it, the same rule read from the other end of the
/// citation. A `rejected` entry left out of a cluster that cites it reaches that cluster's agent as
/// a context entry where the two clusters are packed into different agents; packed together, the
/// agent grades it in the cluster that kept it. A search unit packed
/// whole needs none of this: its clusters share one agent, which grades an entry they share once.
pub(crate) fn assign(graph: &mut Graph, groups: &[usize]) {
    let clusters: Vec<usize> = groups
        .iter()
        .copied()
        .filter(|&g| graph.groups[g].kind == GroupKind::Cluster)
        .collect();
    let heads: Vec<BTreeSet<usize>> = clusters
        .iter()
        .map(|&g| {
            graph.groups[g]
                .entries
                .iter()
                .copied()
                .filter(|&i| graph.entries[i].role == Role::Unit)
                .collect()
        })
        .collect();
    let mut holders: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (c, &g) in clusters.iter().enumerate() {
        for &i in &graph.groups[g].entries {
            if graph.entries[i].role != Role::Unit {
                holders.entry(i).or_default().push(c);
            }
        }
    }
    for (entry, held_by) in holders.into_iter().filter(|(_, h)| h.len() > 1) {
        let ties = |c: usize| -> usize {
            match graph.entries[entry].role {
                Role::Rejected => heads[c]
                    .iter()
                    .filter(|h| graph.cites.get(h).is_some_and(|s| s.contains(&entry)))
                    .count(),
                _ => graph
                    .cites
                    .get(&entry)
                    .map_or(0, |s| s.intersection(&heads[c]).count()),
            }
        };
        // `max_by_key` keeps the last of equal keys, so the order is reversed to keep the first.
        let keep = held_by
            .iter()
            .copied()
            .rev()
            .max_by_key(|&c| ties(c))
            .expect("an entry held twice has a holder");
        for &c in held_by.iter().filter(|&&c| c != keep) {
            graph.groups[clusters[c]].entries.retain(|&i| i != entry);
        }
    }
}

/// Which item each agent of one packing reads, by index into the loads given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Packing {
    /// One list of item indices per agent, in the order the agents are numbered: the agents of
    /// the items within the bound, then one agent per item above it.
    pub agents: Vec<Vec<usize>>,
    /// The items above the bound, each sent to an agent alone.
    pub above: Vec<usize>,
}

/// Packs items, groups or whole search units, into the fewest agents first-fit-decreasing allows,
/// then spreads them over that many agents, so that the loads sit near their mean rather than at
/// the bound.
///
/// `loads` holds each item's load and the key that orders two items of one load. An item above
/// `bound` goes to an agent alone. The others are placed largest first: first-fit-decreasing
/// fixes the number of agents, then each item goes to the least-loaded agent it fits in, the
/// lowest-numbered among equals. Where the spreading cannot place an item under the bound, the
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
    use crate::model::Manifest;
    use std::path::PathBuf;

    // Every fixture is inline: the checker reads no string literal of its own source, per
    // `design@check@checker-source-literals-are-data`.

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
        graph_with(&manifest(), docs)
    }

    fn graph_with(manifest: &Manifest, docs: &[(&str, &str)]) -> Graph {
        let model = Model::from_documents(
            docs.iter()
                .map(|(p, t)| (PathBuf::from(p), t.to_string()))
                .collect(),
        );
        // A model built from text has no listing, so the anchors read the documents' paths, which
        // is where a Directory register's entries are placed.
        let paths: Vec<PathBuf> = docs.iter().map(|(p, _)| PathBuf::from(p)).collect();
        let anchors = Anchors::of(manifest, &paths);
        let entities = Entities::build(&model, &anchors);
        graph(&model, &anchors, &entities)
    }

    /// The index of the group named `name`.
    fn at(graph: &Graph, name: &str) -> usize {
        graph
            .groups
            .iter()
            .position(|grp| grp.name == name)
            .unwrap_or_else(|| panic!("no group {name}"))
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
            (
                "notes/topics/alpha-topic/README.md",
                "# Alpha\n\n- [Rules](rules.md)\n",
            ),
            ("notes/topics/beta-topic/README.md", "# Beta\n\nx\n"),
            // A rule of the entry anchor `alpha-topic`, which the tool constructs.
            (
                "notes/topics/alpha-topic/rules.md",
                "# Alpha rules\n\n## An alpha rule `##alpha-rule`\n\nx\n",
            ),
            (
                "widget/docs/goals.md",
                "# Widget goals\n\nThe widget's prose.\n\n## A widget goal `##widget-goal`\n\n\
                 It refines `goal@a-project@top-goal`.\n",
            ),
            (
                "widget/docs/open-issues/on-a-root-head.md",
                "---\nkind: todo\n---\n# On a root head\n\n## Summary\n\n\
                 It strains `design@a-project@other-head` and `rejected@a-project@lost-shape`.\n",
            ),
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
    /// Mutation checked: the section range in `table::edges` widened to the whole document,
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
        let edges = table::edges(&model, &anchors, &entities, &kinds);
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
        for grp in g.groups.iter().filter(|grp| grp.kind == GroupKind::Cluster) {
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

    /// The claim: a cluster holds the `rejected` entries of its search unit its unit cites and the
    /// standing entries of its search unit that cite it, and no goal; an entry of another search
    /// unit that cites the unit joins no cluster of it. Mutation checked: the test of the search
    /// unit removed from the filter of `gathered` in `graph`, which puts widget's issue in the
    /// root's `Second subject`.
    #[test]
    fn a_cluster_gathers_its_search_unit_s_rejected_entries_and_citers_and_no_goal() {
        let g = graph_of(&project());
        assert_eq!(
            names(&g, group(&g, "a-project :: First subject")),
            [
                "design@a-project@first-head",
                "rejected@a-project@lost-shape",
                "tripwire@a-project@first-premise",
            ]
        );
        assert_eq!(
            names(&g, group(&g, "a-project :: Second subject")),
            ["design@a-project@other-head", "issue@a-project@on-the-head"]
        );
    }

    /// The claim: every entry is graded in a group of its own search unit; an entry no cluster
    /// holds is in its search unit's remainder, goals apart, which are in the group of their
    /// goals file, after the file's prose; and an entry of an anchor the tool constructs belongs to
    /// the search unit holding its home. Mutation checked: `search_unit` returning the anchor's own
    /// name, which makes `alpha-topic` a search unit of its own.
    #[test]
    fn every_entry_is_in_a_group_of_its_search_unit() {
        let g = graph_of(&project());
        let mut covered: BTreeSet<usize> = BTreeSet::new();
        for grp in &g.groups {
            for &i in &grp.entries {
                assert_eq!(g.entries[i].unit, grp.unit, "{}", g.entries[i].reference());
            }
            covered.extend(grp.entries.iter().copied());
        }
        assert_eq!(covered.len(), g.entries.len());
        assert_eq!(
            names(&g, group(&g, "a-project :: (remainder)")),
            ["issue@a-project@on-the-goal"]
        );
        assert_eq!(
            names(&g, group(&g, "widget :: (remainder)")),
            ["issue@widget@on-a-root-head"]
        );
        let alpha = group(&g, "alpha-topic :: notes/topics/alpha-topic/rules.md");
        assert_eq!(names(&g, alpha), ["rule@alpha-topic@alpha-rule"]);
        assert_eq!(alpha.unit, "notes");
        let goals = group(&g, "a-project :: docs/goals.md");
        assert_eq!(goals.kind, GroupKind::Goals);
        assert_eq!(
            names(&g, goals),
            [
                "goal@a-project@fine-goal",
                "goal@a-project@middle-goal",
                "goal@a-project@top-goal",
                "goal@a-project@unserved-goal",
            ]
        );
        let widget = group(&g, "widget :: widget/docs/goals.md");
        assert_eq!(widget.prose, "# Widget goals\n\nThe widget's prose.\n");
    }

    /// The claim: the title lines of a search unit are the root's goals, then each enclosing
    /// Component's, then its own; a goal agent holds none. Mutations checked: `title_units` leaving
    /// out the unit itself, which drops the widget goal; and leaving out the enclosing Components,
    /// which drops the root's goals from widget's list.
    #[test]
    fn the_title_lines_are_the_root_s_the_enclosing_components_and_the_unit_s_goals() {
        let g = graph_of(&project());
        let refs = |unit: &str| -> Vec<String> {
            g.titles[unit]
                .iter()
                .map(|&i| g.entries[i].reference())
                .collect()
        };
        let root = [
            "goal@a-project@fine-goal",
            "goal@a-project@middle-goal",
            "goal@a-project@top-goal",
            "goal@a-project@unserved-goal",
        ];
        assert_eq!(refs("a-project"), root);
        assert_eq!(refs("notes"), root);
        let mut widget: Vec<String> = root.iter().map(|s| s.to_string()).collect();
        widget.push("goal@widget@widget-goal".to_string());
        assert_eq!(refs("widget"), widget);
        let goals = g
            .groups
            .iter()
            .position(|grp| grp.kind == GroupKind::Goals)
            .expect("a goals group");
        assert!(g.title_lines(&[goals]).is_empty());
    }

    /// The claim: an agent's context entries are the heads and `rejected` entries its graded
    /// entries cite that it does not grade, and its load counts its graded entries, its context
    /// entries and its title lines each once. Mutation checked: the test of the graded set
    /// removed from `context`, which lists the root's head beside the root's own cluster.
    #[test]
    fn the_context_is_what_the_agent_cites_and_does_not_grade() {
        let g = graph_of(&project());
        let at = |name: &str| {
            g.groups
                .iter()
                .position(|grp| grp.name == name)
                .expect("the group")
        };
        let widget = at("widget :: (remainder)");
        let refs = |list: Vec<usize>| -> Vec<String> {
            list.iter().map(|&i| g.entries[i].reference()).collect()
        };
        assert_eq!(
            refs(g.context(&[widget])),
            [
                "design@a-project@other-head",
                "rejected@a-project@lost-shape"
            ]
        );
        let second = at("a-project :: Second subject");
        let first = at("a-project :: First subject");
        assert_eq!(
            refs(g.context(&[widget, second, first])),
            Vec::<String>::new()
        );
        let words = |list: &[usize]| -> usize { list.iter().map(|&i| g.entries[i].words).sum() };
        let titles: usize = g.titles["widget"]
            .iter()
            .map(|&i| g.entries[i].title_words())
            .sum();
        assert_eq!(
            g.load(&[widget]),
            words(&g.groups[widget].entries) + words(&g.context(&[widget])) + titles
        );
    }

    /// The claim: in a split search unit, an issue that cites heads of several clusters stays in
    /// the one holding most of them, and a `rejected` entry that heads of several clusters cite
    /// stays in the one holding most of its citers, the first on a tie, so each is in one cluster.
    /// Mutations checked: `ties` made to return 0 for every cluster, `.max_by_key(|&c| ties(c))`
    /// made `.max_by_key(|&c| ties(c) * 0)`, which keeps the first holder whatever the counts; the
    /// count of a `rejected` entry's citers alone made 0, `Role::Rejected => heads[c]` made
    /// `Role::Rejected => 0 * heads[c]`, which keeps the second shape in One; and the `.rev()`
    /// before `max_by_key` removed, which keeps the last holder on a tie.
    #[test]
    fn a_split_unit_s_shared_entry_stays_in_the_cluster_it_is_most_tied_to() {
        let design = "# Design\n\n## One\n\n### Head a `##head-a`\n\nIt covers \
            `rejected@a-project@lost-shape`, `rejected@a-project@tied-shape` and \
            `rejected@a-project@second-shape`.\n\n\
            ### Head b `##head-b`\n\nIt covers `rejected@a-project@lost-shape`.\n\n\
            ## Two\n\n### Head c `##head-c`\n\nIt covers `rejected@a-project@lost-shape` and \
            `rejected@a-project@tied-shape` and `rejected@a-project@second-shape`.\n\n\
            ### Head d `##head-d`\n\nIt covers `rejected@a-project@second-shape`.\n";
        let rejected = "# Rejected\n\n### A lost shape `##lost-shape`\n\n`live`. x\n\n\
            ### A tied shape `##tied-shape`\n\n`live`. x\n\n\
            ### A second shape `##second-shape`\n\n`live`. x\n";
        let issue = "---\nkind: todo\n---\n# Two\n\n## Summary\n\nIt strains \
            `design@a-project@head-a`, `design@a-project@head-c`, `design@a-project@head-d`.\n";
        let mut docs = project();
        docs.retain(|(p, _)| {
            !matches!(
                *p,
                "docs/design.md" | "docs/rejected.md" | "docs/tripwires.md"
            ) && !p.starts_with("docs/open-issues/")
        });
        docs.push(("docs/design.md", design));
        docs.push(("docs/rejected.md", rejected));
        docs.push(("docs/open-issues/on-two.md", issue));
        docs.push(("docs/tripwires.md", "# Tripwires\n"));
        let mut g = graph_of(&docs);
        let root: Vec<usize> = (0..g.groups.len())
            .filter(|&i| g.groups[i].unit == "a-project" && g.groups[i].kind != GroupKind::Goals)
            .collect();
        let one = root
            .iter()
            .copied()
            .find(|&i| g.groups[i].name == "a-project :: One")
            .expect("One");
        let two = root
            .iter()
            .copied()
            .find(|&i| g.groups[i].name == "a-project :: Two")
            .expect("Two");
        assign(&mut g, &root);
        // The lost shape is cited by two heads of One and one of Two, so One keeps it; the tied
        // shape by one head of each, so the first, One, keeps it; the second shape by one head of
        // One and two of Two, and the issue cites one head of One and two of Two, so Two keeps
        // both.
        assert_eq!(
            names(&g, &g.groups[one]),
            [
                "design@a-project@head-a",
                "design@a-project@head-b",
                "rejected@a-project@lost-shape",
                "rejected@a-project@tied-shape",
            ]
        );
        assert_eq!(
            names(&g, &g.groups[two]),
            [
                "design@a-project@head-c",
                "design@a-project@head-d",
                "rejected@a-project@second-shape",
                "issue@a-project@on-two",
            ]
        );
        let shape = g
            .entries
            .iter()
            .position(|e| e.reference() == "rejected@a-project@lost-shape")
            .expect("the lost shape");
        assert!(g.context(&[two]).contains(&shape));
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

    /// The claim: an agent's load counts an entry, a context entry and a title line its groups
    /// share once, and a goals file's prose. Mutations checked: in `title_lines`,
    /// `if seen.insert(goal) {` made `if seen.insert(goal) || true {`, which counts the root's title
    /// lines twice for an agent holding two search units; and in `load`, the prose term made
    /// `0 * self.groups[g].prose.split_whitespace().count()`; each caught.
    #[test]
    fn an_agent_s_load_counts_what_its_groups_share_once_and_a_goals_file_s_prose() {
        let g = graph_of(&project());
        let first = at(&g, "a-project :: First subject");
        let widget = at(&g, "widget :: (remainder)");
        let refs = |list: Vec<usize>| -> Vec<String> {
            list.iter().map(|&i| g.entries[i].reference()).collect()
        };
        assert_eq!(
            refs(g.title_lines(&[first, widget])),
            [
                "goal@a-project@fine-goal",
                "goal@a-project@middle-goal",
                "goal@a-project@top-goal",
                "goal@a-project@unserved-goal",
                "goal@widget@widget-goal",
            ]
        );
        // Apart, both groups count the root's title lines, and `widget`'s count the lost shape as
        // a context entry, which `First subject` grades.
        let root_titles: usize = g.titles["a-project"]
            .iter()
            .map(|&i| g.entries[i].title_words())
            .sum();
        let shape = g
            .entries
            .iter()
            .find(|e| e.reference() == "rejected@a-project@lost-shape")
            .expect("the lost shape")
            .words;
        assert_eq!(
            g.load(&[first, widget]),
            g.load(&[first]) + g.load(&[widget]) - root_titles - shape
        );
        let goals = at(&g, "widget :: widget/docs/goals.md");
        let goal_words: usize = g.groups[goals]
            .entries
            .iter()
            .map(|&i| g.entries[i].words)
            .sum();
        // "# Widget goals\n\nThe widget's prose." holds 6 words.
        assert_eq!(g.load(&[goals]), goal_words + 6);
    }

    /// The claim: a location enclosing a search unit gives it no title lines: only the root and
    /// the Components enclosing it do. Mutation checked: `c.is_component &&` removed from the
    /// filter of `title_units`, which gives the location `inner` the goal of the location `outer`
    /// that encloses it; caught.
    #[test]
    fn a_location_enclosing_a_search_unit_gives_it_no_title_lines() {
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\n\
             components = []\n\n\
             [locations.outer]\npath = \"outer\"\nregisters = [\"goal\"]\n\n\
             [locations.inner]\npath = \"outer/inner\"\nregisters = [\"rule\"]\n\n\
             [registers.rule]\nscope = \"opt-in\"\nshape = \"heading\"\ndir = \"rules\"\nlevel = 2\n\n\
             [agents]\nharness = []\n\n\
             [walk]\nskip-dirs = []\nskip-files = []\n\n";
        let manifest = Manifest::parse(Path::new("/nowhere"), text).expect("a declaration");
        let g = graph_with(
            &manifest,
            &[
                ("docs/goals.md", ROOT_GOALS),
                (
                    "outer/goals.md",
                    "# Outer goals\n\n## An outer goal `##outer-goal`\n\nx\n",
                ),
                (
                    "outer/inner/rules.md",
                    "# Rules\n\n## An inner rule `##inner-rule`\n\nx\n",
                ),
            ],
        );
        let refs = |unit: &str| -> Vec<String> {
            g.titles[unit]
                .iter()
                .map(|&i| g.entries[i].reference())
                .collect()
        };
        assert!(
            refs("outer").contains(&"goal@outer@outer-goal".to_string()),
            "{:?}",
            refs("outer")
        );
        assert_eq!(
            refs("inner"),
            [
                "goal@a-project@fine-goal",
                "goal@a-project@middle-goal",
                "goal@a-project@top-goal",
                "goal@a-project@unserved-goal",
            ]
        );
    }

    /// The groups of `plan`'s agent, by search unit, each once.
    fn units_of(g: &Graph, agent: &Agent) -> BTreeSet<String> {
        agent
            .groups
            .iter()
            .map(|&i| g.groups[i].unit.clone())
            .collect()
    }

    /// The claim: `plan` numbers the cluster agents before the goal agents, and packs a search
    /// unit whose load is exactly half the bound with the others, since only one that exceeds half
    /// gets agents of its own. Mutations checked: in `plan`, `for goals in [false, true] {` made
    /// `for goals in [true, false] {`, which numbers the goal agents first; and `load * 2 > bound`
    /// made `load * 2 >= bound`, which gives the root an agent of its own; each caught.
    #[test]
    fn plan_numbers_cluster_agents_first_and_keeps_a_unit_at_half_the_bound_shared() {
        // A long issue of the root, so the root's load is above the others' together.
        let long = format!(
            "---\nkind: todo\n---\n# Long\n\n## Summary\n\n{}\n",
            "word ".repeat(200)
        );
        let mut docs = project();
        docs.push(("docs/open-issues/a-long-one.md", &long));
        let base = graph_of(&docs);
        let family = |unit: &str| -> Vec<usize> {
            (0..base.groups.len())
                .filter(|&i| base.groups[i].unit == unit && base.groups[i].kind != GroupKind::Goals)
                .collect()
        };
        let root = base.load(&family("a-project"));
        let others = base.load(&family("widget")) + base.load(&family("notes"));
        let bound = root * 2;
        assert!(root + others <= bound, "the units do not fit one agent");
        let mut g = base.clone();
        let plan = plan(&mut g, bound);
        let goal_agent = |a: &Agent| {
            a.groups
                .iter()
                .all(|&i| g.groups[i].kind == GroupKind::Goals)
        };
        let first_goal = plan
            .agents
            .iter()
            .position(goal_agent)
            .expect("a goal agent");
        assert!(first_goal > 0, "a goal agent comes first");
        assert!(plan.agents[first_goal..].iter().all(goal_agent));
        assert!(plan.agents[..first_goal].iter().all(|a| a
            .groups
            .iter()
            .all(|&i| g.groups[i].kind != GroupKind::Goals)));
        let holder = plan.agents[..first_goal]
            .iter()
            .find(|a| units_of(&g, a).contains("a-project"))
            .expect("the root's agent");
        assert!(
            units_of(&g, holder).len() > 1,
            "the root at half the bound is packed alone: {:?}",
            holder.items
        );
        assert!(plan.above.is_empty());
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
