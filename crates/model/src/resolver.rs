//! What a reference resolves to, with no finding attached.
//!
//! One grammar, `` `<kind>@<anchor>@<id>` ``, and one resolver. A reference to a table kind —
//! `design`, `goal`, `tripwire` — is looked up in the table `entity::Entities` builds from the
//! walk; a `path` reference is resolved against the survey under the same anchors, and a
//! `register` reference against the anchors that carry the register, per
//! `design@model@register-home-reference`. A table reference that resolves to nothing has a cause:
//! the anchor is unknown, the anchor does not carry that register, an item of a plan is cited from
//! outside it, or the id is not defined there; the kinds resolved outside the table have refusals of
//! their own. A span whose kind position holds an anchor is no reference, and the caller reports
//! it. The argument is
//! `design@model@an-unresolved-reference-is-reported`.
//!
//! **The function of each kind returns an [`Outcome`]**, per `design@model@resolver-outcome`. The words
//! of a cause live with the cause, since the lints and `show` print the same two texts and a renderer
//! reads the cause and prints neither. This crate writes no finding from one: a consumer that reports
//! does.

use std::path::{Path, PathBuf};

use crate::entity::{
    self, Anchors, Candidate, Constructed, Entities, Kind, Resolution, ESCAPE_ANCHOR, PLANS_ANCHOR,
};
use crate::manifest::{Scope, Shape, MANIFEST_NAME};
use crate::model::Model;
use crate::scan::Observation;

use crate::phases::TreeInputs;

/// What a reference names, once it resolves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// An entry the entity table holds, of a table kind or a harness kind.
    Entity,
    /// A register's home under the anchor named, or every Component's under `*`, whatever shape
    /// the home has. Whether it exists is `tree`'s question.
    Home,
    /// A path under a named anchor. The ignore rules may cover it, so the listing need not hold it.
    Path(PathBuf),
    /// The copies of a path under `*`, one per Component that carries it under the claimed kind.
    Carried(Vec<PathBuf>),
    /// A required document under `*`, which is accepted whether or not a Component carries it.
    Required,
    /// A path at the escape anchor, which this tree does not hold.
    Elsewhere,
    /// The path a plan's work will create.
    Planned(PathBuf),
}

/// The repair of a reference to a skill or an agent no file defines. A name of the installer's
/// namespace has a second cause, an installed file that is missing, whose repair is the install, so
/// the text names each case, per `design@check@finding-names-the-repair`.
fn missing_owner_repair(kind: impl std::fmt::Display, name: &str) -> String {
    if name.starts_with(crate::manifest::OWNED_PREFIX) {
        format!(
            "run `install-agent-skills` if the installed {kind} is missing; otherwise name an \
             existing {kind}, or repair the reference"
        )
    } else {
        format!("name an existing {kind}, or repair the reference")
    }
}

/// Why a reference resolves to nothing.
///
/// Each variant holds what its two texts need beyond the span, so the words are the same wherever
/// a consumer prints them, per `design@check@finding-names-the-repair`. A variant is named for the arm
/// that refuses, and a `String` field such as `listed` or `carriers` holds text ready to print.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cause {
    TableUnknownAnchor {
        anchor: String,
        listed: String,
    },
    AnchorLacksKind {
        anchor: String,
        kind: String,
        carriers: Vec<String>,
    },
    OutsidePlan {
        plan: String,
        form: String,
        id: String,
    },
    UndefinedItem {
        anchor: String,
        kind: String,
        id: String,
        section: String,
    },
    Undefined {
        anchor: String,
        kind: String,
        id: String,
        home: String,
    },
    NoOwner {
        kind: String,
        owner: String,
    },
    NoOwnerSection {
        kind: String,
        owner: String,
        id: String,
    },
    NoNamed {
        kind: String,
        id: String,
    },
    NoSection {
        document: &'static str,
        id: String,
    },
    NoRegister {
        id: String,
        homed: String,
    },
    ItemRegister {
        id: String,
    },
    CarriedByEntries {
        id: String,
        by: String,
    },
    RegisterAtEscape {
        carriers: String,
        every: &'static str,
    },
    RegisterEveryAttached {
        id: String,
        components: String,
        carriers: String,
    },
    RegisterEveryNotAll {
        id: String,
        carriers: String,
    },
    RegisterUnknownAnchor {
        name: String,
        carriers: String,
        every: &'static str,
    },
    RegisterAtPlan {
        name: String,
        form: String,
    },
    RegisterAtEntry {
        name: String,
        form: String,
    },
    RegisterNotCarried {
        name: String,
        id: String,
        carriers: String,
        registers: String,
    },
    EntryByPath {
        form: String,
    },
    HomeByPath {
        anchor: String,
        register: String,
        suffix: bool,
    },
    ResolvesHere {
        form: String,
    },
    EveryRefused {
        why: &'static str,
    },
    AttachedHomesByPath {
        register: String,
        components: String,
    },
    RequiredKind,
    InNoComponent,
    ClaimsFile,
    ClaimsDirectory,
    PathUnknownAnchor {
        name: String,
        names: String,
    },
    PlannedUnknownAnchor {
        name: String,
        names: String,
    },
    AtMilestone {
        name: String,
    },
    AtSpec {
        name: String,
    },
    Refused {
        why: &'static str,
    },
    PlanDocumentByPath {
        form: String,
    },
    InsideDeeperAnchor {
        owner: String,
    },
    PlannedAtReserved {
        anchor: String,
    },
    PlannedOutsidePlans,
    PointsIntoExisting {
        file: String,
        form: String,
    },
    NowExists {
        target: PathBuf,
        form: String,
    },
    Missing {
        target: PathBuf,
    },
}

impl Cause {
    /// What is wrong with the reference written `span`, and what repairs it: the two texts of a
    /// finding.
    pub fn texts(&self, span: &str) -> (String, String) {
        use Cause::*;
        match self {
            TableUnknownAnchor { anchor, listed } => (
                format!("`{span}` names `{anchor}`, which is no anchor of this project"),
                format!(
                    "anchor at one of {listed}; `{ESCAPE_ANCHOR}` serves the path kind alone, and \
                     `*` the path and register kinds"
                ),
            ),
            AnchorLacksKind {
                anchor,
                kind,
                carriers,
            } => (
                format!("`{span}` names `{anchor}`, which carries no {kind} register"),
                format!("the anchors that carry one: {}", carriers.join(", ")),
            ),
            OutsidePlan { plan, form, id } => (
                format!("`{span}` cites an item of the plan `{plan}` from outside it"),
                format!(
                    "cite the plan whole, as `{form}`, and name the item beside it in plain text as \
                     #{id}; an item is cited only from inside its own plan, so retiring the plan asks \
                     no other document to be redesigned"
                ),
            ),
            UndefinedItem {
                anchor,
                kind,
                id,
                section,
            } => (
                format!("`{span}` is referenced and `{anchor}` defines no {kind} `{id}`"),
                format!(
                    "define it under the {section} section of the plan `{anchor}`, or repair the \
                     reference"
                ),
            ),
            Undefined {
                anchor,
                kind,
                id,
                home,
            } => (
                format!("`{span}` is referenced and `{anchor}` defines no {kind} `{id}`"),
                format!("define it in the {home} of `{anchor}`, or repair the reference"),
            ),
            NoOwner { kind, owner } => (
                format!("`{span}` is referenced and no {kind} is named `{owner}`"),
                missing_owner_repair(kind, owner),
            ),
            NoOwnerSection { kind, owner, id } => (
                format!(
                    "`{span}` is referenced and the {kind} `{owner}` defines no section `{id}`"
                ),
                if owner.starts_with(crate::manifest::OWNED_PREFIX) {
                    format!(
                        "run `install-agent-skills` if the installed {kind} is stale; otherwise \
                         repair the reference"
                    )
                } else {
                    format!(
                        "end a level-2 heading of the {kind} `{owner}` with that slug, or repair \
                         the reference"
                    )
                },
            ),
            NoNamed { kind, id } => (
                format!("`{span}` is referenced and no {kind} is named `{id}`"),
                missing_owner_repair(kind, id),
            ),
            NoSection { document, id } => (
                format!("`{span}` is referenced and {document} defines no section `{id}`"),
                if *document == "the primer" {
                    "run `install-agent-skills` if the installed primer is stale; otherwise repair \
                     the reference"
                        .to_string()
                } else {
                    format!(
                        "end a level-2 heading of {document} with that slug, or repair the \
                         reference"
                    )
                },
            ),
            NoRegister { id, homed } => (
                format!("`{span}` names `{id}`, which is no register of this project"),
                format!("name one of {homed}"),
            ),
            ItemRegister { id } => (
                format!("`{span}` names the {id} register, whose home is a plan's own documents"),
                format!(
                    "cite the plan document whole, as `spec@{PLANS_ANCHOR}@<id>` or \
                     `milestone@{PLANS_ANCHOR}@<id>`"
                ),
            ),
            CarriedByEntries { id, by } => (
                format!(
                    "`{span}` names the {id} register, whose homes are the entries of the {by} \
                     register"
                ),
                format!(
                    "cite the entry whose home it is, as `{by}@<anchor>@<entry>`; one directory has \
                     one name, so `show` finds every citation of it"
                ),
            ),
            RegisterAtEscape { carriers, every } => (
                format!("`{span}` names `{ESCAPE_ANCHOR}`, which serves the path kind alone"),
                format!("anchor at one of {carriers}{every}"),
            ),
            RegisterEveryAttached {
                id,
                components,
                carriers,
            } => (
                format!(
                    "`{span}` names `*`, which names a component register's homes, and the {id} \
                     register is attached to {components}"
                ),
                format!("anchor at one of {carriers}"),
            ),
            RegisterEveryNotAll { id, carriers } => (
                format!("`{span}` names `*`, and not every component carries a {id} register"),
                format!("anchor at one of {carriers}"),
            ),
            RegisterUnknownAnchor {
                name,
                carriers,
                every,
            } => (
                format!("`{span}` names `{name}`, which is no anchor of this project"),
                format!("anchor at one of {carriers}{every}"),
            ),
            RegisterAtPlan { name, form } => (
                format!(
                    "`{span}` names the plan `{name}`, whose documents are cited by their kind"
                ),
                format!(
                    "cite it as `{form}`; a plan document has one name, so `show` finds every \
                     citation of it"
                ),
            ),
            RegisterAtEntry { name, form } => (
                format!("`{span}` names the entry `{name}`, whose homes are its own directory"),
                format!(
                    "cite the entry as `{form}`; one directory has one name, so `show` finds every \
                     citation of it"
                ),
            ),
            RegisterNotCarried {
                name,
                id,
                carriers,
                registers,
            } => (
                format!("`{span}` names `{name}`, which carries no {id} register"),
                format!(
                    "the anchors that carry one: {carriers}; the registers `{name}` carries: \
                     {registers}"
                ),
            ),
            EntryByPath { form } => (
                format!("`{span}` cites an entry by its path"),
                format!(
                    "cite it as `{form}`; one directory has one name, so `show` finds every \
                     citation of it"
                ),
            ),
            HomeByPath {
                anchor,
                register,
                suffix,
            } => {
                let whose = if anchor == entity::EVERY_ANCHOR {
                    "every component's".to_string()
                } else {
                    format!("`{anchor}`'s")
                };
                (
                    format!("`{span}` names {whose} home of the {register} register by its path"),
                    format!(
                        "write `register@{anchor}@{register}`, which names the home in either of \
                         its shapes, so a home that changes shape breaks no reference{}",
                        if *suffix {
                            ". A reference names the home: drop the line number or the fragment, \
                             and name the point inside it in prose"
                        } else {
                            ""
                        }
                    ),
                )
            }
            ResolvesHere { form } => (
                format!("`{span}` resolves in this tree, as `{form}`"),
                format!(
                    "the escape anchor is for a path this tree does not hold: write `{form}`, or, \
                     for a file of another project, begin the path with that project's name, as \
                     `path@elsewhere@<project>/<path>`"
                ),
            ),
            EveryRefused { why } => (
                format!("`{span}` is refused: {why}"),
                "the generic form takes a plain component-relative path".to_string(),
            ),
            AttachedHomesByPath {
                register,
                components,
            } => (
                format!("`{span}` names the homes of the {register} register by their path"),
                format!(
                    "write `register@<component>@{register}` for one of {components}, which names \
                     the home in either of its shapes"
                ),
            ),
            RequiredKind => (
                format!("`{span}` claims the wrong kind for a required document"),
                "a required document is a file and takes no trailing slash; the slash is the kind \
                 claim"
                    .to_string(),
            ),
            InNoComponent => (
                format!("`{span}` resolves in no component"),
                "the generic form names a required document, or a path at least one component \
                 carries; repair the path, or anchor at one component"
                    .to_string(),
            ),
            ClaimsFile => (
                format!("`{span}` claims a file and names a directory"),
                "add the trailing slash, or repair the path; the slash is the kind claim"
                    .to_string(),
            ),
            ClaimsDirectory => (
                format!("`{span}` claims a directory and names a file"),
                "drop the trailing slash, or repair the path; the slash is the kind claim"
                    .to_string(),
            ),
            PathUnknownAnchor { name, names } => (
                format!("`{span}` names `{name}`, which is no anchor of this project"),
                format!(
                    "anchor at one of {names}, at `{ESCAPE_ANCHOR}` for a path this tree does not \
                     hold, or at `*` for every component's own copy"
                ),
            ),
            PlannedUnknownAnchor { name, names } => (
                format!("`{span}` names `{name}`, which is no anchor of this project"),
                format!("anchor at the one of {names} that will hold the target"),
            ),
            AtMilestone { name } => (
                format!("`{span}` names the milestone `{name}`, which carries no path kind"),
                format!(
                    "cite a slice spec as `spec@{name}@<slice>`, the milestone document as \
                     `milestone@{PLANS_ANCHOR}@{name}`, and from inside it its items as \
                     `<kind>@{name}@<id>`; a plan document has one name, so `show` finds every \
                     citation of it"
                ),
            ),
            AtSpec { name } => (
                format!("`{span}` names the spec `{name}`, which carries no path kind"),
                format!(
                    "cite the spec as `spec@{PLANS_ANCHOR}@{name}`, and from inside it its items as \
                     `<kind>@{name}@<id>`; a plan document has one name, so `show` finds every \
                     citation of it"
                ),
            ),
            Refused { why } => (
                format!("`{span}` is refused: {why}"),
                "anchor at the anchor that holds the target, with a plain relative path"
                    .to_string(),
            ),
            PlanDocumentByPath { form } => (
                format!("`{span}` cites a plan document by its path"),
                format!(
                    "cite it as `{form}`; a plan document has one name, so `show` finds every \
                     citation of it"
                ),
            ),
            InsideDeeperAnchor { owner } => (
                format!("`{span}` reaches inside the anchor `{owner}`"),
                format!(
                    "anchor at the deepest anchor holding the target, so a move edits \
                     {MANIFEST_NAME} and no document"
                ),
            ),
            PlannedAtReserved { anchor } => (
                format!("`{span}` gives a planned path the anchor `{anchor}`"),
                format!(
                    "anchor at the anchor that will hold the target; a planned path sits at one \
                     anchor, never `*` or `{ESCAPE_ANCHOR}`"
                ),
            ),
            PlannedOutsidePlans => (
                format!("`{span}` is a planned path outside the plans directory"),
                "a planned path is cited from a plan document only; name the plan that creates it"
                    .to_string(),
            ),
            PointsIntoExisting { file, form } => (
                format!("`{span}` points into `{file}`, which exists"),
                format!("write `{form}`, and name the function or the heading in prose"),
            ),
            NowExists { target, form } => (
                format!("`{span}` now exists, at `{}`", target.display()),
                format!("write `{form}`; the planned form names a path that does not exist yet"),
            ),
            Missing { target } => (
                format!("`{span}` does not exist, at `{}`", target.display()),
                "repair the pointer, or delete it; a path that does not resolve is a guess"
                    .to_string(),
            ),
        }
    }
}

/// What the resolver returns for a reference: its target, or the cause of its failure.
pub type Outcome = Result<Target, Cause>;

/// A reference candidate of an anchored kind, resolved by the function of its kind. A harness kind
/// is [`harness`]'s, and reaching this function with one reads as a table kind.
pub fn reference(
    kind: &Kind,
    anchor: &str,
    id: &str,
    rel: &Path,
    anchors: &Anchors,
    entities: &Entities,
    tree: &TreeInputs,
) -> Outcome {
    if kind.is_path() {
        path(rel, anchor, id, anchors, tree)
    } else if kind.is_planned() {
        planned(rel, anchor, id, anchors, tree)
    } else if kind.is_register() {
        // Never through `table`: a register's home is no entity id of the table, and `*` resolves
        // under this kind, which `table` refuses.
        register(anchor, id, anchors)
    } else {
        table(kind, anchor, id, rel, anchors, entities)
    }
}

/// A reference to a table kind, resolved through the entity table.
pub fn table(
    kind: &Kind,
    anchor: &str,
    id: &str,
    rel: &Path,
    anchors: &Anchors,
    entities: &Entities,
) -> Outcome {
    match entities.resolve_from(anchors, kind, anchor, id, Some(rel)) {
        Resolution::Resolved => Ok(Target::Entity),
        Resolution::UnknownAnchor => Err(Cause::TableUnknownAnchor {
            anchor: anchor.to_string(),
            listed: anchors.listed(),
        }),
        Resolution::AnchorLacksRegister { carriers } => Err(Cause::AnchorLacksKind {
            anchor: anchor.to_string(),
            kind: kind.to_string(),
            carriers: carriers.iter().map(|c| c.to_string()).collect(),
        }),
        Resolution::OutsidePlan { form } => Err(Cause::OutsidePlan {
            plan: anchor.to_string(),
            form: form.to_string(),
            id: id.to_string(),
        }),
        Resolution::Undefined => {
            let register = anchors.registers().by_name(kind.name());
            // An item is defined under its section of the plan, which has no home of its own.
            match register.and_then(|r| r.section.as_deref()) {
                Some(section) => Err(Cause::UndefinedItem {
                    anchor: anchor.to_string(),
                    kind: kind.to_string(),
                    id: id.to_string(),
                    section: section.to_string(),
                }),
                None => Err(Cause::Undefined {
                    anchor: anchor.to_string(),
                    kind: kind.to_string(),
                    id: id.to_string(),
                    home: register
                        .map(|r| {
                            if r.dir == crate::manifest::CARRIER_DIR {
                                "home at its own directory".to_string()
                            } else {
                                format!("{} home", r.dir)
                            }
                        })
                        .unwrap_or_else(|| "register home".to_string()),
                }),
            }
        }
    }
}

/// A reference of a harness kind, against the entity table: the skill or the agent it names, and
/// the section, or the section of the primer or of the root instructions.
///
/// A section of a skill or an agent that does not exist is reported as the missing skill or
/// agent, not as the missing section: its repair is the name, whatever the slug says.
pub fn harness(kind: &Kind, owner: &str, id: &str, entities: &Entities) -> Outcome {
    if entities.defines(kind, owner, id) {
        return Ok(Target::Entity);
    }
    Err(if !owner.is_empty() && !entities.defines(kind, "", owner) {
        Cause::NoOwner {
            kind: kind.to_string(),
            owner: owner.to_string(),
        }
    } else if !owner.is_empty() {
        Cause::NoOwnerSection {
            kind: kind.to_string(),
            owner: owner.to_string(),
            id: id.to_string(),
        }
    } else if kind.is_named() {
        Cause::NoNamed {
            kind: kind.to_string(),
            id: id.to_string(),
        }
    } else {
        Cause::NoSection {
            document: if kind.name() == entity::PRIMER_KIND {
                "the primer"
            } else {
                "the root CLAUDE.md"
            },
            id: id.to_string(),
        }
    })
}

/// A `register` reference: the home an anchor keeps a register in, resolved against the anchors,
/// whatever shape the home has, per `design@model@register-home-reference`.
///
/// Whether the home exists is not asked here: a missing home is `tree`'s finding, and the
/// reference names what the manifest declares. Every refusal names the form that resolves, so a
/// repair lists the anchors that carry the register, not every anchor as `table` does: an anchor
/// that does not carry it is one more refusal for a writer who follows the repair.
pub fn register(anchor: &str, id: &str, anchors: &Anchors) -> Outcome {
    let registers = anchors.registers();
    let Some(r) = registers.by_name(id) else {
        // An item register's home is a plan's own documents, so it has no home of its own to name,
        // and neither has a plan anchor's register: both are cited by the plan document's kind.
        return Err(Cause::NoRegister {
            id: id.to_string(),
            homed: registers
                .all()
                .iter()
                .filter(|r| r.shape != Shape::Section)
                .map(|r| r.name.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        });
    };
    if r.shape == Shape::Section {
        return Err(Cause::ItemRegister { id: id.to_string() });
    }
    // `*` resolves only for a register every component carries, so a repair offers it only then.
    let every = if r.scope == Scope::Component {
        ", or at `*` for every component's own home"
    } else {
        ""
    };
    let carriers = || {
        anchors
            .all()
            .iter()
            .filter(|a| !a.is_entry() && !a.is_plan() && a.registers.iter().any(|n| n == id))
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    // A register only the entries of a Directory register carry has its homes at those entries'
    // own directories, which each entry's reference names, so no anchor gives it this form. At an
    // entry, the arm below names that entry's own reference.
    if carriers().is_empty() && !anchors.by_name(anchor).is_some_and(|a| a.is_entry()) {
        if let Some(by) = registers
            .all()
            .iter()
            .find(|d| d.shape == Shape::Directory && d.carries.iter().any(|c| c == id))
        {
            return Err(Cause::CarriedByEntries {
                id: id.to_string(),
                by: by.name.to_string(),
            });
        }
    }
    match anchor {
        ESCAPE_ANCHOR => Err(Cause::RegisterAtEscape {
            carriers: carriers(),
            every,
        }),
        entity::EVERY_ANCHOR if !r.components.is_empty() => Err(Cause::RegisterEveryAttached {
            id: id.to_string(),
            components: r.components.join(", "),
            carriers: carriers(),
        }),
        entity::EVERY_ANCHOR => {
            if r.scope != Scope::Component {
                Err(Cause::RegisterEveryNotAll {
                    id: id.to_string(),
                    carriers: carriers(),
                })
            } else {
                Ok(Target::Home)
            }
        }
        name => match anchors.by_name(name) {
            None => Err(Cause::RegisterUnknownAnchor {
                name: name.to_string(),
                carriers: carriers(),
                every,
            }),
            Some(a) if a.is_plan() => Err(Cause::RegisterAtPlan {
                name: name.to_string(),
                form: if a.is_milestone() {
                    format!("milestone@{PLANS_ANCHOR}@{name}")
                } else {
                    format!("spec@{PLANS_ANCHOR}@{name}")
                },
            }),
            // An entry keeps what it carries at its own directory, which the entry's own
            // reference names, per `design@model@register-home-reference`.
            Some(a) if a.is_entry() => Err(Cause::RegisterAtEntry {
                name: name.to_string(),
                form: a.entry_form().unwrap_or_default(),
            }),
            Some(a) if !a.registers.iter().any(|n| n == id) => Err(Cause::RegisterNotCarried {
                name: name.to_string(),
                id: id.to_string(),
                carriers: carriers(),
                registers: a.registers.join(", "),
            }),
            Some(_) => Ok(Target::Home),
        },
    }
}

/// A path written with a suffix, a line number `:<n>` or a fragment `#<anchor>`, split into the
/// path and whether a suffix was there. Only the home-target test reads it: a suffix on any other
/// `path` reference keeps the finding of the target it does not name.
fn without_suffix(path: &str) -> (&str, bool) {
    // A fragment is a heading anchor, which may open on a digit, as `#1-intro`; a line suffix is a
    // number or a closed range. Each is stripped once, the fragment first and again last, so a span
    // that carries both, in either order, loses both. An empty or open suffix is no suffix.
    fn fragment(p: &str) -> Option<usize> {
        let (head, frag) = p.rsplit_once('#')?;
        let ok = !frag.is_empty()
            && frag
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        ok.then_some(head.len())
    }
    fn line(p: &str) -> Option<usize> {
        let (head, num) = p.rsplit_once(':')?;
        let digits = |t: &str| !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit());
        let ok = match num.split_once('-') {
            Some((a, b)) => digits(a) && digits(b),
            None => digits(num),
        };
        ok.then_some(head.len())
    }
    let mut bare = path;
    for strip in [fragment, line, fragment] {
        if let Some(end) = strip(bare) {
            bare = &bare[..end];
        }
    }
    (bare, bare.len() != path.len())
}

/// Why a path's shape is refused before any resolution, or `None` for a plain relative path.
///
/// An upward or explicit-current segment breaks when the referencing file moves, which is
/// the failure the anchored grammar exists to remove, and a leading slash is outside the
/// project's namespace.
pub fn refused(path: &str) -> Option<&'static str> {
    if path.starts_with('/') {
        return Some("a leading slash is outside the project's namespace");
    }
    if path.split('/').any(|s| s == "..") {
        return Some("an upward segment is anchored at the wrong place by definition");
    }
    if path.split('/').any(|s| s == ".") {
        return Some("an explicit-current segment adds nothing and survives no move");
    }
    None
}

/// Where `path`, the path of an escaped reference, its line suffix or fragment ignored, resolves in
/// this tree: read from the root, from each anchor's directory, then from the directory of `rel`,
/// the document that writes it, a `.` and a `..` followed as written; a path opening with `.` or
/// `..` is read from the document first, and a leading `/` from the root alone. The first that the
/// listing holds is the target, as a project-relative path.
fn resolved_here(path: &str, rel: &Path, anchors: &Anchors, tree: &TreeInputs) -> Option<PathBuf> {
    let (bare, _) = without_suffix(path);
    let segments: Vec<&str> = bare.split('/').filter(|s| !s.is_empty()).collect();
    // A path opening with `.` or `..` is read from its document first, as a relative markdown
    // path is, so the form names the file its writer meant, where another anchor's directory
    // would reach another one.
    let document = rel.parent().unwrap_or(Path::new(""));
    let dotted = matches!(segments.first(), Some(&".") | Some(&".."));
    let mut bases: Vec<&Path> = if dotted { vec![document] } else { Vec::new() };
    bases.push(Path::new(""));
    if !bare.starts_with('/') {
        bases.extend(anchors.all().iter().map(|a| a.path.as_path()));
        if !dotted {
            bases.push(document);
        }
    }
    bases.into_iter().find_map(|base| {
        let mut at = base.to_path_buf();
        for segment in &segments {
            match *segment {
                "." => {}
                ".." => {
                    if !at.pop() {
                        return None;
                    }
                }
                name => at.push(name),
            }
        }
        (!at.as_os_str().is_empty() && tree.present.contains(&at)).then_some(at)
    })
}

/// A `path` reference, resolved against the survey under its anchor.
pub fn path(rel: &Path, anchor: &str, path: &str, anchors: &Anchors, tree: &TreeInputs) -> Outcome {
    // The trailing slash is the writer's claim about the target's kind; the lookup drops it.
    let claims_dir = path.ends_with('/');
    let trimmed = path.trim_end_matches('/');
    match anchor {
        ESCAPE_ANCHOR => {
            // The escape is exempt from existence and kind assertions — there is nothing
            // stable to assert — and from the shape refusals, because a foreign layout may
            // spell anything. The one assertion is that it stays honest: a target that
            // resolves here is a real reference wearing the escape, which would otherwise
            // be the cheap way to silence the unanchored finding.
            //
            // The target is read from every place the unanchored lint reads a path from, the
            // document's own directory included, or a file named relative to its document would
            // be silenced here and reported bare.
            let Some(target) = resolved_here(trimmed, rel, anchors, tree) else {
                return Ok(Target::Elsewhere);
            };
            // A home is named by its own kind, so the repair names that form rather than
            // a `path` the next run would report.
            let (_, suffix) = without_suffix(trimmed);
            if let Some(form) = entry_at(anchors, &target) {
                return Err(Cause::EntryByPath { form });
            }
            if let Some((keeper, r)) = anchors.home_target(&target) {
                return Err(Cause::HomeByPath {
                    anchor: keeper.name.to_string(),
                    register: r.name.to_string(),
                    suffix,
                });
            }
            // A plan document is cited by its kind, never by a path.
            let form = match plan_document(anchors, &target) {
                Some(form) => form,
                None => {
                    // The deepest anchor that holds the target and carries the `path` kind,
                    // per `design@check@every-path-names-its-anchor`: a plan anchor carries
                    // none, and an anchor's own directory is named from the anchor above it.
                    let mut owner = anchors.owning(&target);
                    while owner.is_plan()
                        || (owner.path == target && !target.as_os_str().is_empty())
                    {
                        owner = anchors.owning(owner.path.parent().unwrap_or(Path::new("")));
                    }
                    let under = target.strip_prefix(&owner.path).unwrap_or(&target);
                    let slash = if tree.directories.contains(&target) {
                        "/"
                    } else {
                        ""
                    };
                    format!("path@{}@{}{slash}", owner.name, under.display())
                }
            };
            Err(Cause::ResolvesHere { form })
        }
        entity::EVERY_ANCHOR => {
            if let Some(why) = refused(path) {
                return Err(Cause::EveryRefused { why });
            }
            // A register's home is named by its own kind, before any kind claim or presence is
            // judged: either shape of a home names the same register, and the manifest, not the
            // listing, says which paths are homes.
            let (bare, suffix) = without_suffix(path);
            if let Some(r) = anchors.generic_home_target(bare.trim_end_matches('/')) {
                // A register attached to some Components has no `register` form under `*`: each
                // of its homes is named under the Component that keeps it.
                return Err(if r.scope != Scope::Component {
                    Cause::AttachedHomesByPath {
                        register: r.name.to_string(),
                        components: r.components.join(", "),
                    }
                } else {
                    Cause::HomeByPath {
                        anchor: entity::EVERY_ANCHOR.to_string(),
                        register: r.name.to_string(),
                        suffix,
                    }
                });
            }
            // The compiled-in required set is accepted whether or not any component carries
            // the file yet. Every name in it is a file, since a register's home was answered
            // above, so the trailing-slash claim is asserted here too.
            if let Some(required_dir) = anchors.required_kind(trimmed) {
                return if claims_dir != required_dir {
                    Err(Cause::RequiredKind)
                } else {
                    Ok(Target::Required)
                };
            }
            // Anything else must be real somewhere, with the claimed kind. A hit is an
            // anchor's OWN copy: a path reaching a nested anchor from above is not this
            // anchor's, or the generic form would evade the deepest-anchor rule. A
            // gitignored copy is not a hit either — presence of generated content is build
            // state, and a verdict may not depend on the checking machine's. Presence and
            // kind are asked apart, so a path some component carries under the other kind
            // gets the kind-claim repair rather than "repair the path".
            // Components only, per `design@model@reserved-anchors`: the generic form claims
            // every component's own copy, so a copy a declared location alone holds makes the
            // claim false for every component. The anchors the tool constructs are no
            // components either: they hold plan documents, which are cited by their kind, per
            // `design@model@plan-document-kinds`.
            let carried: Vec<PathBuf> = anchors
                .all()
                .iter()
                .filter(|a| a.is_component)
                .map(|a| (a, a.path.join(trimmed)))
                .filter(|(a, t)| {
                    anchors.owning(t).path == a.path
                        && !tree
                            .ignored
                            .contains(&crate::git::ignore_query(t, claims_dir))
                        && tree.present.contains(t)
                })
                .map(|(_, t)| t)
                .collect();
            if carried.is_empty() {
                Err(Cause::InNoComponent)
            } else {
                let of_the_kind: Vec<PathBuf> = carried
                    .iter()
                    .filter(|t| claims_dir == tree.directories.contains(*t))
                    .cloned()
                    .collect();
                if of_the_kind.is_empty() {
                    Err(if tree.directories.contains(&carried[0]) {
                        Cause::ClaimsFile
                    } else {
                        Cause::ClaimsDirectory
                    })
                } else {
                    Ok(Target::Carried(of_the_kind))
                }
            }
        }
        name => {
            let target = anchored_target(name, path, anchors, false)?;
            assert_target(&target, claims_dir, tree)?;
            Ok(Target::Path(target))
        }
    }
}

/// The target of a `path` or `planned` reference at a named anchor, or the cause that refuses it:
/// an unknown anchor, a plan anchor, a refused shape, a plan document cited by its path, or a
/// target inside a deeper anchor. Both kinds share these rules, so a planned path converts to a
/// `path` reference by its kind word alone.
fn anchored_target(
    name: &str,
    path: &str,
    anchors: &Anchors,
    planned: bool,
) -> Result<PathBuf, Cause> {
    let trimmed = path.trim_end_matches('/');
    let Some(a) = anchors.by_name(name) else {
        // A planned path sits at one anchor, never at a reserved word, so its repair names the
        // declared anchors only.
        // A plan anchor carries no path kind, so it is no candidate for either kind.
        let names = anchors
            .all()
            .iter()
            .filter(|a| !a.is_plan())
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let name = name.to_string();
        return Err(if planned {
            Cause::PlannedUnknownAnchor { name, names }
        } else {
            Cause::PathUnknownAnchor { name, names }
        });
    };
    if a.is_milestone() {
        return Err(Cause::AtMilestone {
            name: name.to_string(),
        });
    }
    if a.is_plan() {
        return Err(Cause::AtSpec {
            name: name.to_string(),
        });
    }
    // Judged on the path as written: a lone `/` trims to nothing and would resolve
    // to the anchor's own directory, which has no spelling under its own name.
    if let Some(why) = refused(path) {
        return Err(Cause::Refused { why });
    }
    let target = a.path.join(trimmed);
    // Per `design@model@plan-document-kinds`, before the deepest-anchor rule: a plan document cited by its path is
    // refused from every anchor, and the repair names the one form that resolves.
    if let Some(form) = plan_document(anchors, &target) {
        return Err(Cause::PlanDocumentByPath { form });
    }
    // An entry's directory, and its README, which is both the entry's document and the head of
    // what it carries, are named by the entry's own reference, per
    // `design@model@path-to-a-home-is-reported`; a file beside the README is a path target. A
    // `planned` path to an entry that now exists takes the same repair, which is the one form
    // that resolves.
    let (bare, _) = without_suffix(trimmed);
    if let Some(form) = entry_at(anchors, &a.path.join(bare.trim_end_matches('/'))) {
        return Err(Cause::EntryByPath { form });
    }
    // A register's home is named by its own kind, whichever anchor the path was written from:
    // the finding names the anchor that keeps the home, so it replaces the deeper-anchor finding
    // rather than following it. A `planned` path names no home: a home is a declared anchor's.
    if !planned {
        let (bare, suffix) = without_suffix(trimmed);
        if let Some((keeper, r)) = anchors.home_target(&a.path.join(bare.trim_end_matches('/'))) {
            return Err(Cause::HomeByPath {
                anchor: keeper.name.to_string(),
                register: r.name.to_string(),
                suffix,
            });
        }
    }
    // The deepest anchor wins: a reference reaching inside another anchor breaks
    // when that anchor moves, and the anchor is what a move must not break. Inside
    // means a PROPER descendant: an anchor's own directory has no spelling under
    // its own name, so pointing at it from an ancestor is the one legal way to name
    // where it lives — and that reference names a location, which is exactly what
    // a move is expected to break.
    let owner = anchors.owning(&target);
    if owner.path != a.path && target != owner.path {
        return Err(Cause::InsideDeeperAnchor {
            owner: owner.name.to_string(),
        });
    }
    Ok(target)
}

/// A `planned` reference: a path a plan's work will create, per `design@model@planned-path-form`.
///
/// It is legal in the plans directory alone, takes a named anchor under the rules of a `path`
/// reference, and asserts that its target does not exist. Once the target exists the cause
/// names the `path` form and nothing else: the session that created the file converts the
/// reference, and whether the plan still holds is that session's report to make, not the
/// check's.
pub fn planned(
    rel: &Path,
    anchor: &str,
    path: &str,
    anchors: &Anchors,
    tree: &TreeInputs,
) -> Outcome {
    if !in_plans_directory(rel, anchors) {
        return Err(Cause::PlannedOutsidePlans);
    }
    let claims_dir = path.ends_with('/');
    let target = planned_target(anchor, path, anchors)?;
    // A line or a fragment names no file: `x.rs:12` of an existing `x.rs` is a pointer into it,
    // which the planned form cannot be.
    if let Some((file, _)) = path.split_once([':', '#']) {
        if let Some(a) = anchors.by_name(anchor) {
            if tree.present.contains(&a.path.join(file)) {
                return Err(Cause::PointsIntoExisting {
                    file: file.to_string(),
                    form: converted(anchors, anchor, &a.path.join(file), file),
                });
            }
        }
    }
    if tree
        .ignored
        .contains(&crate::git::ignore_query(&target, claims_dir))
    {
        return Ok(Target::Planned(target));
    }
    if tree.present.contains(&target) {
        // The repair carries the kind the target has, so applying it raises no slash finding.
        let slash = if tree.directories.contains(&target) {
            "/"
        } else {
            ""
        };
        let trimmed = path.trim_end_matches('/');
        let form = converted(anchors, anchor, &target, &format!("{trimmed}{slash}"));
        return Err(Cause::NowExists { target, form });
    }
    Ok(Target::Planned(target))
}

/// The form a planned path converts to once its target exists: the `register` form where the
/// target is a register's home, per `design@model@path-to-a-home-is-reported`, else the `path`
/// form at the planned anchor.
fn converted(anchors: &Anchors, anchor: &str, target: &Path, path: &str) -> String {
    match anchors.home_target(Path::new(
        target.to_str().unwrap_or("").trim_end_matches('/'),
    )) {
        Some((keeper, r)) => format!("register@{}@{}", keeper.name, r.name),
        None => format!("path@{anchor}@{path}"),
    }
}

/// The target of a planned reference, or the cause that refuses it: the two reserved words, and
/// every rule of a named anchor that `anchored_target` applies. `show` asks the same, so a planned
/// reference this refuses resolves to nothing in either.
pub fn planned_target(anchor: &str, path: &str, anchors: &Anchors) -> Result<PathBuf, Cause> {
    if anchor == ESCAPE_ANCHOR || anchor == entity::EVERY_ANCHOR {
        return Err(Cause::PlannedAtReserved {
            anchor: anchor.to_string(),
        });
    }
    anchored_target(anchor, path, anchors, true)
}

/// Whether a document lies in the plans directory, where a planned path is legal.
pub fn in_plans_directory(rel: &Path, anchors: &Anchors) -> bool {
    anchors
        .all()
        .iter()
        .any(|a| a.constructed == Some(Constructed::Plans) && rel.starts_with(&a.path))
}

/// The reference naming the entry of a declared Directory register whose directory, or whose
/// README, `target` is, or `None` where it is neither. A milestone's are `plan_document`'s.
fn entry_at(anchors: &Anchors, target: &Path) -> Option<String> {
    anchors
        .all()
        .iter()
        .find(|e| {
            e.is_entry() && !e.is_plan() && (target == e.path || target == e.path.join("README.md"))
        })
        .and_then(|e| e.entry_form())
}

/// The kind form of a plan document at `target`, or `None` when no plan document is there.
///
/// A plan document is a spec file of `specs/`, a milestone directory, or a file inside a
/// milestone directory, per `design@model@plan-document-kinds`. The README and the index of the plans
/// directory and of its two homes are not plan documents, and are cited by path. Judged by
/// where the target sits, not by whether it exists: a path to a deleted slice spec gets the same
/// repair as one to a present one.
pub(crate) fn plan_document(anchors: &Anchors, target: &Path) -> Option<String> {
    use crate::manifest::{MILESTONES_HOME, SPECS_HOME};
    let plans = anchors
        .all()
        .iter()
        .find(|a| a.constructed == Some(Constructed::Plans))?;
    if let Ok(inside) = target.strip_prefix(plans.path.join(MILESTONES_HOME)) {
        let parts: Vec<String> = inside
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect();
        // The first segment names a milestone only in the id grammar: a directory of another
        // name is no milestone, and its finding is phase 2's.
        let m = parts.first().filter(|m| entity::is_entry_id(m))?;
        let slice = match parts.as_slice() {
            [_, file] => file
                .strip_suffix(".md")
                .filter(|s| *s != "README" && *s != "index"),
            _ => None,
        };
        return Some(match slice {
            Some(slice) => format!("spec@{m}@{slice}"),
            None => format!("milestone@{PLANS_ANCHOR}@{m}"),
        });
    }
    entity::entry_id(target, &plans.path.join(SPECS_HOME))
        .map(|id| format!("spec@{PLANS_ANCHOR}@{id}"))
}

/// Assert one resolved target: it exists and has the claimed kind, unless the ignore rules
/// cover it.
///
/// The exemption runs on the ignore RULES, as `git check-ignore` states them, rather than on
/// what happens to exist, so a reference to a generated path passes on a fresh clone exactly
/// as it passes on a built tree — a verdict that depends on build state is a check nobody can
/// trust twice. The batch was taken by the caller over `ignore_queries` below, so a target
/// this function asks about that the collector did not gather reads as not ignored.
pub fn assert_target(target: &Path, claims_dir: bool, tree: &TreeInputs) -> Result<(), Cause> {
    if tree
        .ignored
        .contains(&crate::git::ignore_query(target, claims_dir))
    {
        return Ok(());
    }
    if !tree.present.contains(target) {
        return Err(Cause::Missing {
            target: target.to_path_buf(),
        });
    }
    let is_dir = tree.directories.contains(target);
    if claims_dir && !is_dir {
        Err(Cause::ClaimsDirectory)
    } else if !claims_dir && is_dir {
        Err(Cause::ClaimsFile)
    } else {
        Ok(())
    }
}

/// Whether relative markdown links are legal in this file.
///
/// A README is directions about what a directory holds and an index is a generated or
/// hand-written listing, so both are navigation homes; everywhere else a pointer in prose is
/// a backticked reference, and a link is reported.
pub fn is_navigation(rel: &Path) -> bool {
    rel.file_name()
        .is_some_and(|n| n == "README.md" || n == "index.md")
}

/// Whether a link target opens with a URI scheme — letters then a colon, before any slash.
///
/// A filename may hold a colon in principle; a relative path in this project never does, and
/// reading such a target as a scheme passes it over rather than reporting a false dangling.
pub fn has_scheme(target: &str) -> bool {
    target
        .split('/')
        .next()
        .is_some_and(|head| head.contains(':'))
}

/// Every spelling a path reference in this model could ask the ignore rules about.
///
/// **One batch per run, taken by the caller.** The caller batches these through one
/// `git check-ignore` and hands the answers back in `TreeInputs::ignored`, because a check may spawn
/// nothing, per `design@check@model-then-checks`. **Every arm that reaches
/// `assert_target` is here**, and there are three: an anchored reference resolves under the
/// anchor it names, the generic form resolves under every anchor at once, and a relative
/// markdown link resolves beside its own file. The escape anchor resolves under none, so it
/// contributes nothing.
///
/// **A spelling this misses is a target the check reads as not ignored**, which is a finding
/// rather than a silence: the reference is asserted to exist. `an_ignored_target_is_exempt_in_
/// every_arm_the_collector_gathers` is what holds the two lists together.
pub fn ignore_queries(model: &Model, anchors: &Anchors) -> Vec<String> {
    let mut out = Vec::new();
    for doc in model.documents() {
        let nav = is_navigation(&doc.rel);
        for l in &doc.observations {
            match &l.what {
                Observation::Span(span) => {
                    let Candidate::Reference { kind, anchor, id } =
                        entity::candidate(span, anchors)
                    else {
                        continue;
                    };
                    if !kind.takes_a_path() {
                        continue;
                    }
                    let claims_dir = id.ends_with('/');
                    let trimmed = id.trim_end_matches('/');
                    match anchor {
                        ESCAPE_ANCHOR => {}
                        entity::EVERY_ANCHOR => {
                            for a in anchors.all() {
                                out.push(crate::git::ignore_query(
                                    &a.path.join(trimmed),
                                    claims_dir,
                                ));
                            }
                        }
                        name => {
                            if let Some(a) = anchors.by_name(name) {
                                out.push(crate::git::ignore_query(
                                    &a.path.join(trimmed),
                                    claims_dir,
                                ));
                            }
                        }
                    }
                }
                // The arms `link` returns on before resolving are the arms that ask git
                // nothing, in the order it takes them.
                Observation::Link(target) if doc.is_markdown() && nav => {
                    if target.starts_with('#') {
                        continue;
                    }
                    let file_part = target.split('#').next().unwrap_or(target);
                    if has_scheme(file_part) || refused(file_part).is_some() {
                        continue;
                    }
                    let dir = doc.rel.parent().unwrap_or(Path::new(""));
                    let resolved = dir.join(file_part.trim_end_matches('/'));
                    out.push(crate::git::ignore_query(
                        &resolved,
                        file_part.ends_with('/'),
                    ));
                }
                _ => {}
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Manifest;
    use std::collections::HashSet;

    fn manifest() -> Manifest {
        let text = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\n\
                    components = [\"crates/one\"]\n\n[walk]\nskip-dirs = []\nskip-files = []\n\
                    exclude = []\n";
        Manifest::parse(Path::new("/nowhere"), text).expect("a declaration")
    }

    /// Resolves a `path` reference over a listing and the ignore answers given.
    fn path_over(
        anchor: &str,
        id: &str,
        files: &[&str],
        directories: &[&str],
        ignored: &[String],
    ) -> Outcome {
        let m = manifest();
        let anchors = Anchors::of(&m, &[] as &[PathBuf]);
        let present: HashSet<PathBuf> =
            files.iter().chain(directories).map(PathBuf::from).collect();
        let directories: HashSet<PathBuf> = directories.iter().map(PathBuf::from).collect();
        let ignored: HashSet<String> = ignored.iter().cloned().collect();
        let tree = TreeInputs {
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &ignored,
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
        };
        path(Path::new("README.md"), anchor, id, &anchors, &tree)
    }

    #[test]
    fn a_path_under_a_named_anchor_resolves_to_its_target_or_says_why_not() {
        let target = PathBuf::from("crates/one/src/a.rs");
        assert_eq!(
            path_over("one", "src/a.rs", &["crates/one/src/a.rs"], &[], &[]),
            Ok(Target::Path(target.clone()))
        );
        assert_eq!(
            path_over("one", "src/a.rs", &[], &[], &[]),
            Err(Cause::Missing {
                target: target.clone()
            })
        );
        // The rules cover it, so the listing need not hold it: a verdict over build state would
        // differ between a fresh clone and a built tree.
        let query = crate::git::ignore_query(&target, false);
        assert_eq!(
            path_over("one", "src/a.rs", &[], &[], &[query]),
            Ok(Target::Path(target))
        );
    }

    #[test]
    fn the_escape_resolves_to_nothing_here_and_the_generic_form_to_each_copy() {
        assert_eq!(
            path_over("elsewhere", "other/a.rs", &[], &[], &[]),
            Ok(Target::Elsewhere)
        );
        assert_eq!(
            path_over("*", "Cargo.toml", &["crates/one/Cargo.toml"], &[], &[]),
            Ok(Target::Carried(vec![PathBuf::from(
                "crates/one/Cargo.toml"
            )]))
        );
        assert_eq!(
            path_over("*", "README.md", &[], &[], &[]),
            Ok(Target::Required)
        );
    }

    /// The claim: an anchor that carries no such register resolves to the cause naming the anchors
    /// that do, over an anchor list no manifest produces today: an anchor carrying tripwires alone.
    #[test]
    fn an_anchor_carrying_no_such_register_is_reported_with_the_anchors_that_do() {
        let m = manifest();
        let base = Anchors::of(&m, &[] as &[PathBuf]);
        let root = base.by_name("a-project").expect("the root").clone();
        let bare = crate::entity::Anchor::location(
            "bare",
            Path::new("bare"),
            vec!["tripwire".to_string()],
        );
        let anchors = Anchors::from_list(vec![root, bare], base.registers().clone());
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n### A decision `##a-decision`\n".to_string(),
        )]);
        let entities = Entities::build(&model, &anchors);
        let cause = table(
            &Kind::new("design"),
            "bare",
            "a-decision",
            Path::new("notes/a.md"),
            &anchors,
            &entities,
        )
        .expect_err("the anchor carries no design register");
        let (what, action) = cause.texts("design@bare@a-decision");
        assert!(
            what.contains("`bare`, which carries no design register"),
            "{what}"
        );
        assert!(
            action.contains("the anchors that carry one: a-project"),
            "{action}"
        );
    }

    /// The claim: the repair a refused plan citation names is the form that resolves for each position: a slice for
    /// a slice spec, the milestone for its README, its index or anything deeper, a spec for a
    /// spec, and nothing for a navigation file. Mutations checked: the README and index filter,
    /// and the one-segment test, each removed from `plan_document`.
    #[test]
    fn the_p1_repair_names_the_form_for_each_position() {
        let m = manifest();
        let anchors = Anchors::of(&m, &[] as &[PathBuf]);
        for (target, form) in [
            (
                "docs/plans/milestones/m-one/a-slice.md",
                Some("spec@m-one@a-slice"),
            ),
            (
                "docs/plans/milestones/m-one/README.md",
                Some("milestone@plans@m-one"),
            ),
            (
                "docs/plans/milestones/m-one/index.md",
                Some("milestone@plans@m-one"),
            ),
            (
                "docs/plans/milestones/m-one/sub/x.md",
                Some("milestone@plans@m-one"),
            ),
            (
                "docs/plans/milestones/m-one/a.md/x.md",
                Some("milestone@plans@m-one"),
            ),
            ("docs/plans/milestones/m-one", Some("milestone@plans@m-one")),
            ("docs/plans/specs/a-spec.md", Some("spec@plans@a-spec")),
            ("docs/plans/specs/README.md", None),
            ("docs/plans/milestones/README.md", None),
            ("docs/plans/README.md", None),
        ] {
            assert_eq!(
                plan_document(&anchors, Path::new(target)).as_deref(),
                form,
                "{target}"
            );
        }
    }

    /// The claim: an entry of the table, a skill and a register's home are each said to be what they
    /// are, so a consumer with no finding can tell them apart.
    #[test]
    fn a_kind_outside_the_path_kinds_resolves_to_what_it_names() {
        let m = manifest();
        let anchors = Anchors::of(&m, &[] as &[PathBuf]);
        let model = Model::from_documents(vec![(
            PathBuf::from("docs/design.md"),
            "# Design\n\n### A decision `##a-decision`\n".to_string(),
        )]);
        let entities = Entities::build(&model, &anchors);
        assert_eq!(
            table(
                &Kind::new("design"),
                "a-project",
                "a-decision",
                Path::new("r.md"),
                &anchors,
                &entities,
            ),
            Ok(Target::Entity)
        );
        assert_eq!(register("a-project", "issue", &anchors), Ok(Target::Home));
        assert_eq!(register("*", "issue", &anchors), Ok(Target::Home));
    }

    /// The claim: a planned path resolves to the path it will create, whether absent or covered by
    /// the ignore rules.
    #[test]
    fn a_planned_path_resolves_to_the_path_it_will_create() {
        let m = manifest();
        let anchors = Anchors::of(&m, &[] as &[PathBuf]);
        let target = PathBuf::from("crates/one/src/new.rs");
        let (present, directories) = (HashSet::new(), HashSet::new());
        for ignored in [
            HashSet::new(),
            HashSet::from([crate::git::ignore_query(&target, false)]),
        ] {
            let tree = TreeInputs {
                present: &present,
                directories: &directories,
                outside: &[],
                ignored: &ignored,
                tracked_and_ignored: &[],
                refused: &[],
                links: &[],
            };
            assert_eq!(
                planned(
                    Path::new("docs/plans/README.md"),
                    "one",
                    "src/new.rs",
                    &anchors,
                    &tree
                ),
                Ok(Target::Planned(target.clone()))
            );
        }
    }

    /// The claim: the generic form's target holds the copies of the claimed kind and no other.
    #[test]
    fn the_generic_form_holds_only_the_copies_of_the_claimed_kind() {
        let two = "[project]\nchecker-version = \"fixture\"\nname = \"a-project\"\n\
                   components = [\"crates/one\", \"crates/two\"]\n\n[walk]\nskip-dirs = []\n\
                   skip-files = []\nexclude = []\n";
        let m = Manifest::parse(Path::new("/nowhere"), two).expect("a declaration");
        let anchors = Anchors::of(&m, &[] as &[PathBuf]);
        let (one, other) = (
            PathBuf::from("crates/one/src"),
            PathBuf::from("crates/two/src"),
        );
        let present: HashSet<PathBuf> = [one.clone(), other.clone()].into_iter().collect();
        let directories: HashSet<PathBuf> = [one.clone()].into_iter().collect();
        let ignored = HashSet::new();
        let tree = TreeInputs {
            present: &present,
            directories: &directories,
            outside: &[],
            ignored: &ignored,
            tracked_and_ignored: &[],
            refused: &[],
            links: &[],
        };
        let rel = Path::new("README.md");
        assert_eq!(
            path(rel, "*", "src/", &anchors, &tree),
            Ok(Target::Carried(vec![one]))
        );
        assert_eq!(
            path(rel, "*", "src", &anchors, &tree),
            Ok(Target::Carried(vec![other]))
        );
    }
}
