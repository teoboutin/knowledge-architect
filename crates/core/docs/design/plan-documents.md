# Core design: Plan documents

## Plan documents

### Plan documents are a structure the checker reads: the anchor `plans` carries a `spec` and a `milestone` register `##plan-register`

The plans directory holds the decided design of work that is not built yet, which no other home
holds, per `goal@knowledge-architect@design-is-recorded-with-its-arguments`. The checker reads it
as anchors and registers, so a citation of a plan is checked like any other, per
`goal@knowledge-architect@documentation-stays-consistent`: the tool constructs the anchor `plans`,
carrying two built-in registers, `spec` and `milestone`, that no project declares. `spec` is
also carried by each milestone anchor, for its slice specs, per
`design@core@anchors-are-components-and-locations`; no other anchor carries either. Inside each
plan, its threads, arguments, criteria and acceptance criteria are items, cited and checked like
any entry, per `design@core@plan-items-by-section`, so the record of a discussion is checked
before it is harvested. Plan documents read as free prose was the alternative, and it leaves
every citation of a plan unchecked and every plan document unlisted.

### The plans directory is docs/plans/ of the root, fixed by the tool `##plans-dir-fixed`

A component gets its register homes at paths the tool fixes, and a built-in register refuses a
`dir` key, so a declared plans directory would be the only declared path in that family. The tool
fixing it costs a project whose plans sit elsewhere one move, and a skill written against the fixed
path works in every project, per `goal@agent-skills@installed-text-works-anywhere`. A fixed path is the tool's convention, not a project compiled in, per
`design@core@nothing-of-a-project-is-compiled-in`.

### A project has one plans directory, in its root component `##plans-at-root`

Most work spans components, so a plan sits at the root whatever its subject: the milestone that
built the plans layout touched three, and the milestone that released version 0.1 touched every
component. Two premises are the owner's weighing: "there should never be that many planned tasks
open at once that splitting the directory would make sense", and "plans have a tendency to break
each other, when they are designed before the other is implemented", which one directory keeps in
one view. The reference grammar names the anchor, so a later split adds
anchors and rewrites no citation.

The design this decision did not keep: `rejected@core@plans-directory-per-component`.

### The plans directory holds its README and two homes, specs/ and milestones/, and nothing else `##plans-split-dirs`

A spec is one file under specs/, and a milestone one directory under milestones/, each in a
register of its own: one register with two kinds taken from the entry's shape would be a new kind
of register, where two registers reuse the File shape and add one entry shape. Anything else
directly under docs/plans/ is a phase-2 finding, so a plan document always sits in a home and is
always listed.

### A plan document is cited by its kind, `spec` or `milestone`, and a slice spec in its milestone's anchor `##plan-document-kinds`

A spec is `spec@plans@<id>` and a milestone `milestone@plans@<id>`. Each milestone directory is an
anchor carrying `spec` for its slice specs, `spec@<milestone>@<slice>`, so a slice's name need only
be unique inside its milestone, and two milestones may each have a slice called `the-harvest`. The
`milestone` register's entries are directories because each is an anchor, per
`design@core@milestone-entries-are-directories`. A `path` citation of a plan document is
refused, per `design@core@every-path-names-its-anchor`: with two names for one document,
`show` would miss the citations written the other way. A whole plan document may be cited from
anywhere; when it leaves, each citation dangles, and that is the revisit it asks for.

### A plan's items are the level-three headings of its item sections, and the section gives the kind `##plan-items-by-section`

A plan document's threads, arguments, criteria and acceptance criteria are items: a level-three
heading ending with its slug, `### <statement> ##<id>`, under the level-two section Threads,
Arguments, Criteria or Acceptance criteria, defines an item of the kind `thread`, `argument`,
`criterion` or `acceptance`, in the plan anchor that owns the document. A milestone's README and
its slice specs share one namespace, so an id defined in two of them is the duplicate finding.
Every level-three heading of those four sections owes a slug; any other level-three heading of a
plan document is section text, and a slug anywhere else in it defines nothing. Items need a model
of their own because a heading register has one home file per anchor and one level, where a plan
has one home and four kinds; an extension runs after the entity table is built, so items are the
core's, per `design@core@an-extension-plugs-in-through-phased-hooks`.

### An item is cited from inside its own plan only `##plan-item-scope`

An item citation, `<kind>@<plan>@<id>`, resolves only from a file inside the plan: the spec file, or a file of the milestone's directory. From anywhere
else, a commit message included, it is refused before its id is looked up, and the repair names
the document to cite whole: the spec, the milestone, or the slice spec that defines the item, with
the item named beside it as `#<id>`, the label `skill@knowledge-architect-planning@spec-contents`
gives an item outside its plan. A
whole plan may be cited from anywhere, so a dependency between plans is carried by the document,
and retiring a plan asks no other plan to be redesigned item by item. The premise is that a
whole-document citation carries every dependency between plans that matters while few plans are
open at once.

The design this decision did not keep: `rejected@core@cross-plan-item-citation`.

### Item registers are the plan's own register shape, `Section`, whose home is the plan's documents `##items-as-section-registers`

The four item registers have the shape `Section`: each names its level-two section, its entries
sit at level three, and its home is the plan anchor's own documents, which the File and Directory
homes that hold them already judge, so a `Section` register owes no home of its own in phase 2,
no index and no walk check. The kind is read from the level-two heading in force above an item,
out of the heading records the scanner makes. Four ordinary heading registers at one home would
each claim the same file, and only the first would define anything.

### The item registers are built in, and carried by the plan anchors alone `##item-registers-built-in`

Like `spec` and `milestone`, the item registers are the tool's, because their storage is the plan
documents, which the plans layout fixes. A declaration of one is refused, per
`design@core@registers-are-declared`.

### A spec file is an anchor of its own, so a spec's items are cited by the spec's name `##spec-file-is-an-anchor`

Each spec file of specs/ is an anchor, so its items are cited `<kind>@<spec>@<id>`, as a slice's
are cited by its milestone's name, in every project that writes specs. The plan anchor stands in
the anchor position, so an item citation keeps the three-part grammar, shorter than the root
anchor and a compound id by the root anchor's name and one separator. The anchor holds its own
file, which stays an entry of the `spec` register that holds it. Making every spec a directory was
the rival; it reopens the decision that a spec is one file.

### A plan document owes the sections of its kind, and a slice spec owes its own `##plan-document-sections-by-kind`

A spec of specs/ and a milestone's README owe the plan sections, in order, with Arguments right
after Threads, matched as the documents write them: Status and audience, How the work is done,
Names, What the work is, What is already decided, Criteria, Threads, Arguments, "New names, in one
place", Decided design, Mapping tables, Losing alternatives, Readings, Premortem, Acceptance
criteria, Implementation sequence, Order rationale, Defaults awaiting the owner, Harvest, Later
consequences. A slice spec owes Builds, Claims, Audit subjects, Fails alone on and Premises that
expire, in order; the plan sections a slice spec holds, for the design only its slice builds, are
not ordered against them. Fixtures is owed only where a Component drives its tests with authored
content, a condition no check can read, so it is not checked. One `spec` register owes two lists,
chosen by the anchor that holds the spec.
The two lists are the planning skill's, which owns the shape of a plan document, per
`design@agent-skills@design-hands-off-to-planning`; a change to them changes this head.

### A milestone entry is a directory, carrying the slice specs and the four item registers `##milestone-entries-are-directories`

`milestone` is the built-in Directory register, per `design@core@declarable-directory-register`,
carried by the anchor `plans`: each milestone is a directory of milestones/ holding its
`README.md`, the milestone document, and its entry carries `spec`, for its slice specs, and the
four item registers, per `design@core@plan-document-kinds`. What a plan adds on top of the shape is
the tool's and keyed on the register: its documents owe the sections of their kind, per
`design@core@plan-document-sections-by-kind`, its README defines items, its items are cited from
inside the plan only, per `design@core@plan-item-scope`, and it carries no `path` kind. The shape
exists for it because the owner asked to cite a milestone as `milestone@plans@<id>`, and a
milestone is a directory: a File entry is one file and could hold no slice specs. Citing the
milestone by a path to its README needs no new shape, and loses to that request.
