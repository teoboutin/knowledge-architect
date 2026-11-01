# Core design: Anchors and Components

## Anchors and Components

### Every component carries the same documents and the home of every component register `##components-carry-the-same-documents`

The components are the manifest's `[project]` list, per
`design@core@nothing-of-a-project-is-compiled-in`: the project root, named by `project.name`, and
every other project-relative directory, named by the basename of its path. Nothing is discovered by filename, so a document in a
directory that is not a component is not that kind of document: nothing counts it and nothing
reports it. A component carries `README.md`, `CLAUDE.md` while the project serves the `claude`
agent harness (per `design@core@agents-table`), and the home of every `component`-scoped register
— `design`, `goal`, `tripwire`, `rejected` and `issue`, and any the manifest declares at that
scope. No document of the compiled list lies under `docs/`, where those homes are. The root also
holds the plans directory, which is not a component's document but the anchor `plans`, per
`design@core@plan-register`. A component that does not carry one it owes is a finding, raised by
`path@core@src/check/tree.rs`.

**Which components exist is declared, and what every component carries at least is compiled in.**
That is not an exception to `design@core@nothing-of-a-project-is-compiled-in`: the document list and
the five built-in component registers in `manifest.rs` belong to no project, they are what the word
_component_ means here. A project adds to that set, per
`design@core@a-component-carries-what-a-project-attaches`, and removes nothing from it but the agent
document, below: a project free to declare its own set would be conformant with whatever it
declared, which is the same as being checked against nothing, against
`goal@core@document-rules-are-checked`: a tree that passes the checker conforms to the rules the
core compiles in, and no project holds back any of them beyond what a declaration of its manifest
switches off. The one declaration that changes the set is the agent harness, and it removes only the
agent document. The flaw this leaves, that a developer's contracts lose their required home in a
target project that uses the checker alone and declares no harness, is `issue@core@a-home-for-developer-contracts-outside-agent-configuration`.

**A directory that carries outstanding state without being a component is a location**, per
`design@core@anchors-are-components-and-locations`.

### An anchor is a component or a location, and a location carries the registers it declares or the tool gives it `##anchors-are-components-and-locations`

Both carry registers. Every anchor is a named directory and a path anchor under
`design@core@every-path-names-its-anchor`, except the plan anchors the tool constructs, below: a
spec anchor is a file, and no plan anchor carries the `path` kind. What separates them is what they owe. A **component**
owes the compiled documents, every `component`-scoped register and every `opt-in` register the
project attaches to it, with its homes under `<component>/docs/`. A **location** owes the homes of the registers its `[locations.<name>]`
row declares and nothing else, with its homes directly under its own path, or at it for a register
declared there, per `design@core@home-at-the-carrier-path` — a location's directory is already
documentation, where a component's is source code.

**A location exists so that a directory outside every component can be cited.** Naming the
directory gives every entry in it a reference and makes relocating it one manifest edit, which
`goal@core@relocation-is-one-manifest-edit` asks and `design@core@every-path-names-its-anchor`
buys everywhere else. What a location is instead of
is `rejected@core@additional-trackers-file-list`.

**A location's own registers are the only ones it carries**, and a skill or an agent of the
project's own under one, such as this repository's under `agent-config`, is a section home of a
harness kind, which no anchor carries, per `design@core@harness-kinds`.

**Three kinds of location are constructed by the tool rather than declared**, and each is a location
in every respect but the one its layout fixes:

- **`plans`**, at docs/plans/ of the root, carrying `spec` and `milestone`, per
  `design@core@plan-register`. It is judged against the declared anchors like a location, first
  among the anchors of its depth, so a declaration that collides with it is the one refused and
  its complaint names a declaration the project can change. It owes a `README.md` beside its two
  homes.
- **One anchor per entry of a Directory register**: each directory of a Directory register's
  home holding a file `README.md`, named by its basename, per
  `design@core@declarable-directory-register`. Each register it carries has its home at the
  anchor's own path. A directory whose name `design@core@a-placed-name-reads-as-nothing-else`
  refuses is no anchor. A milestone is one: its `spec` register's slice specs sit beside the
  milestone document and are cited `spec@<milestone>@<slice>`, unique only inside the milestone,
  per `design@core@plan-document-kinds`; it carries the four item registers, per
  `design@core@plan-items-by-section`, and no `path` kind.
- **One anchor per spec**: each spec file of specs/, grouped or not, named by its id, per
  `design@core@spec-file-is-an-anchor`. It owns its own file, carries the four item registers,
  and carries no `path` kind. A spec whose name is refused, or that a milestone holds, is no
  anchor, and a directory is never a spec.

**The entry and spec anchors are read off the tree, so every builder of the anchors is handed the
tree's paths**: a check the survey's listing, a command git's listing, `commits` each commit's own
listing. An entry is a directory a session creates, not a declaration, so asking the manifest
alone would give a commit the working tree's entries and judge its message against anchors it
never held. `manifest::collides` does not judge them, because a register's home places them; the
name check is their collision check, and it is a phase-2 finding.

### No two anchors give one path two meanings: an anchor sits inside another's directory, never at its path or comparable with one of its register homes `##anchor-collisions-refused`

A location inside a component is the ordinary case, and the deepest anchor wins, so a document
under a location belongs to it and not to the component holding it. Four declarations take that rule away. Each is a complaint of the manifest's
resolution, and **the refused anchor is no anchor**: it owns nothing, carries nothing and is
asserted against nothing, per `design@core@a-wrong-declaration-is-a-finding`. Candidates are taken
shallowest first, a component before a location at equal depth, then components in declaration
order and locations in name order, so of two that collide the deeper or the later one in that
order is the one refused:

- **a component inside a location.** It nests one full register set inside a partial one and
  gives a document two candidate homes.
- **an anchor inside, or at, another anchor's register home.** Every file under it would be read
  as an entry or a subdocument of the outer register, and reported against that register and its
  anchor rather than against the declaration that caused it.
- **an anchor whose directory holds another anchor's register home** — a location at a
  component's `docs/`. Being deeper, it would own every document in that home, and each slug
  defined there would be misplaced against an anchor carrying no such register.
- **two anchors at one path.** Neither is deeper, so nothing decides which owns the documents
  under it.

The principle behind the four is that no two declarations give one path two meanings. No
compiled document lies under `docs/`, where the component registers keep their homes, so no
register's home can be a compiled document. Outside those shapes two anchors' homes cannot overlap: a home sits under its anchor's directory, so two
overlapping homes put one anchor's directory on the path to the other's home, which is one of the
shapes above.

### No anchor wears a kind's name `##kind-names-refused-as-anchors`

A reference's head is read as a kind before it is read as an anchor, so an anchor named `design`, `path` or `skill` would be shadowed there, and the old form
written with it would be misread. Every kind's name, the harness kinds' whatever the harness and
every declared register's included, is refused for a Component, a location, the project, a
plan and an entry of a directory register, per `design@core@a-placed-name-reads-as-nothing-else`
for the last two. The owner, on the question:
"My ruling on this is that all kind names should be refused for anything that can be an anchor
name (components, custom locations...). I'd rather make this decision early to avoid painful
migrations."

### A Component carries every register the workflow needs, and any register the project attaches to it `##a-component-carries-what-a-project-attaches`

Every Component carries the component registers, the built-in ones and those declared with
`scope = "component"`, which the workflow needs in every Component, per
`design@core@components-carry-the-same-documents`. An `opt-in` register names the Components that
carry it besides, in its own table, `components = [<component>, …]`, the root's name included, and
each keeps its home under its own `docs/`, as `<component>/docs/<dir>/`. A name that is no
Component, and a home that would give a directory of the Component two meanings, its `docs/` itself
with `dir = "."`, the root's plans directory, or the directory of another register it carries, is a
phase-1 finding, and that Component does not carry it.

**The ground is the owner's intent**: a Component is always required to carry the registers the
workflow needs to function, and a project may add any kind of register to any Component, the root
included. The floor holds for every project, one that installs no agent harness included, on the
owner's word that such a project is still supposed, in most cases, to have people working by the
workflow, without which the checker has little use. Which registers can be attached is the
session's argued limit within that intent: a register the project declares, since a built-in
component register is carried by every Component already and a plan register belongs to the plans
layout, per `design@core@plan-register`. A register attached adds obligations and removes none, per
`design@core@the-regime-has-no-opt-out`. Against a location carrying the register at its own path,
per `design@core@home-at-the-carrier-path`, attaching costs one key, invents no anchor name, and
cites what the register holds under the Component's name. The rival, a Component's register list
fixed by the word _component_, is `rejected@core@component-register-list-fixed`.

**The list is the register's**, beside its `scope`, because a Component is declared by its path in
`[project] components` and has no table of its own: a table per Component would declare the
Components twice.

### A name the tree places as an anchor reads as no kind's name, no declared anchor's and no other placed anchor's `##a-placed-name-reads-as-nothing-else`

An entry's name, a milestone's among them, and a spec's id sit in the anchor position of a
reference, since each entry directory and each spec file is an anchor. So such a name that is a
kind's name, a component's name, a location's name or a reserved word is a phase-2 finding, and so
is one another placed anchor already holds; a name so refused is no anchor. Otherwise a reference
reads as the other anchor, and which one wins depends on the order the anchors were built in.
**The order is fixed**: the milestones first, then the specs, then the entries of the declared
Directory registers, by register name, each register's entries by path. Of a spec and a milestone
of one name, the milestone keeps it, and an entry of a declared register gives way to every plan,
since the plans layout is the tool's and every project has it. An entry's name is also an entity
id, since it is the id of an entry of its register, and is not `index`: beside an entry, `<id>.md`
is the refused single-file shape of what it carries, which for `index` is its home's own listing.
