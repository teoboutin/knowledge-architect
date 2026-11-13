# Model design: Plan documents

## Plan documents

### Plan documents are a structure the checker reads: the anchor `plans` carries a `spec` and a `milestone` register `##plan-register`

The plans directory holds the decided design of work that is not built yet, which no other home
holds, per `goal@design-is-recorded-with-its-arguments`, and, until a milestone leaves, the specs
of its landed slices, per `design@model@a-harvested-slice-is-frozen`. The checker reads it
as anchors and registers, so a citation of a plan is checked like any other, per
`goal@documentation-stays-consistent`: the tool constructs the anchor `plans`,
carrying two built-in registers, `spec` and `milestone`, that no project declares, each of the plan
shape, per `design@model@plan-model-first-class`. `spec` is
also carried by each milestone anchor, for its slices, per
`design@model@anchors-are-components-and-locations`; no other anchor carries either. Inside each
plan, its threads, arguments, criteria and acceptance criteria are items, cited and checked like
any entry, per `design@model@plan-items-by-section`, so the record of a discussion is checked
before it is harvested. Plan documents read as free prose was the alternative, and it leaves
every citation of a plan unchecked and every plan document unlisted.

### The plans directory is docs/plans/ of the root, fixed by the tool `##plans-dir-fixed`

A component gets its register homes at paths the tool fixes, and a built-in register refuses a
`dir` key, so a declared plans directory would be the only declared path in that family. The tool
fixing it costs a project whose plans sit elsewhere one move, and a skill written against the fixed
path can be followed in every project that installs it, as
`design@agent-skills@installed-text-works-with-what-an-installing-project-holds` has an installed text be. A fixed path is the tool's convention, not a project compiled in, per
`design@check@nothing-of-a-project-is-compiled-in`.

### A project has one plans directory, in its root component `##plans-at-root`

Most work spans components, so a plan sits at the root whatever its subject: the milestone that
built the plans layout touched three, and the milestone that released version 0.1 touched every
component. Two premises are the owner's weighing: that so many planned tasks should never be open
at once that splitting the directory would make sense, and that plans tend to break each other when
one is designed before the other is implemented, which one directory keeps in one view. The reference grammar names the anchor, so a later split adds
anchors and rewrites no citation.

The design this decision did not keep: `rejected@model@plans-directory-per-component`.

### The plans directory holds its README and two homes, specs/ and milestones/, and nothing else `##plans-split-dirs`

A spec is one directory under specs/, and a milestone one directory under milestones/, per
`design@model@plan-document-directory`, each in a register of its own: one register with two kinds
taken from the entry's shape would be a new kind of register, where two registers each take the
plan shape of their kind, per `design@model@plan-model-first-class`. Anything else directly under
docs/plans/ is a phase-2 finding, so a plan document always sits in a home and is always listed.
specs/ keeps its groups, per `design@check@groups-are-subdirectories`: a directory of specs/, or of
a group of specs/, that holds a `README.md` is a spec, and any other directory of specs/ is a group.
A spec file of specs/ is a phase-2 finding whose repair names the directory it moves to.

### Every plan document is a directory, holding its `README.md` and only the files the plans layout admits beside it `##plan-document-directory`

A spec, a milestone and each slice of a milestone is a directory, and its `README.md` is the plan
document, which defines it and which the searches read. Beside it, the directory may hold these, and
nothing else:

- `deliberation.md`, the Threads and the Arguments of the discussion, per
  `design@model@plan-items-by-section`;
- `search/`, the generated lines of the document's searches, as `discussion.txt` and
  `design-audit.txt`;
- `transcript.txt`, the discussion's trimmed transcript;
- in a milestone besides, its generated `index.md`, one directory per live slice, and `harvested/`,
  which holds one directory per landed slice, per `design@model@a-harvested-slice-is-frozen`.

Any other file or directory of a plan document's directory is a phase-2 finding, and so is a file of
`search/` other than those two. The set is closed because the searches read the `README.md` alone,
per `design@agent-skills@discussion-record-apart`, and each file beside it has a role the planning
skill states and a regime `design@model@a-harvested-slice-is-frozen` states: a file of no stated
role, admitted by an open set, is where a session can write a condition of the design that no
search then reads, with no rule saying what it may hold. What each file holds, and when it is written, is the planning
skill's, per `design@agent-skills@discussion-record-apart`. A spec of specs/ and a slice take the
same shape, so the planning skill states one shape of a plan document's directory.

**A plan document has artifacts with no other home.** Its deliberation sits beside the document the
search reads, not inside it, per `design@agent-skills@discussion-record-apart`, and the generated
lines of its searches and the transcript of its discussion are kept beside the document they
record, per `design@agent-skills@plan-commits-the-generated-lines` and
`design@agent-skills@committed-transcript`. A spec as one file could hold none of them
beside it without a second home for each, of a kind no register has: that rival is
`rejected@model@spec-as-one-file`. The layout is a file format every adopting project writes, so a
change to it is a Migration entry of the checker's changelog.

### The plan registers take a shape of their own, and leave the Directory shape to a declared register `##plan-model-first-class`

`spec` and `milestone` take the plan shape, of the kind spec or milestone, which no declaration can
give a register, beside the Directory shape that a declared register keeps as
`design@model@declarable-directory-register` states it. The model reads the plans layout off the
paths, in `path@model@src/plan.rs`: which directory is a spec, a milestone or a slice, live or
harvested, and which files sit beside its `README.md`.

The rival extends the Directory shape to carry plan documents. A milestone departs from that shape
in three ways: it carries five registers, `spec` for its slices and the four item registers, where
a declared Directory register carries one, on the owner's ruling that
`design@model@declarable-directory-register` records; it carries no `path` kind, where a declared
entry does, per `design@model@entry-anchor-path-kind`; and the sections its documents owe are
compiled in, per `design@check@plan-document-sections-by-kind`. A spec departs the same ways, with
four item registers. Extending the generic shape to admit them breaks that ruling, or holds each
departure as a condition on a register's name inside a shape a project declares, which then changes
whenever the planning skill changes what a plan document holds. That rival is
`rejected@model@plans-on-the-directory-shape`.

### A plan document is cited by its kind, `spec` or `milestone`, and a slice in its milestone's anchor `##plan-document-kinds`

A spec is `spec@plans@<id>` and a milestone `milestone@plans@<id>`, each naming the plan document's
directory. Each milestone directory is an anchor carrying `spec` for its slices,
`spec@<milestone>@<slice>`, naming the slice's directory, live or under `harvested/`, so a slice's
name need only be unique inside its milestone, and two milestones may each have a slice called
`the-harvest`. A `path` citation of a plan document, or of a file of its directory, is refused, per
`design@check@every-path-names-its-anchor`: with two names for one document, `show` would miss the
citations written the other way, and no plan anchor carries the `path` kind, so a `README.md`
reaches the files beside it by relative navigation links, which the link check judges. A whole plan
document may be cited from anywhere; when it leaves, each citation dangles, and that is the revisit
it asks for. A harvested slice is cited from inside its milestone only, per
`design@model@a-harvested-slice-is-cited-from-inside-its-milestone`.

### A plan's items are the level-three headings of its item sections, and the section gives the kind `##plan-items-by-section`

A plan document's threads, arguments, criteria and acceptance criteria are items: a level-three
heading ending with its slug, `### <statement> ##<id>`, under the level-two section Threads,
Arguments, Criteria or Acceptance criteria, defines an item of the kind `thread`, `argument`,
`criterion` or `acceptance`, in the plan anchor that owns the document. The Threads and the
Arguments sit in the plan document's `deliberation.md`, and the Criteria and the Acceptance criteria
in its `README.md`; a `README.md` holding a section Threads or Arguments is a finding, per
`design@check@plan-document-sections-by-kind`. A milestone's documents and those of its slices,
live or harvested, share one namespace, so an id defined in two of them is the duplicate finding, a
frozen definition counting against a live one, per `design@model@a-harvested-slice-is-frozen`.
Every level-three heading of those four sections owes a slug; any other level-three heading of a
plan document is section text, and a slug anywhere else in it defines nothing. Items need a model
of their own because a heading register has one home file per anchor and one level, where a plan
holds four kinds across the documents of its directory; an extension runs after the entity table is
built, so items are the table's own, per `design@check@an-extension-plugs-in-through-phased-hooks`.

### An item is cited from inside its own plan only `##plan-item-scope`

An item citation, `<kind>@<plan>@<id>`, resolves only from a file inside the plan's directory: a
spec's, or a milestone's, its slices' included, live or harvested. From anywhere
else, a commit message included, it is refused before its id is looked up, and the repair names
the document to cite whole: the spec, the live slice that defines the item, or the milestone, for an
item its own documents or a harvested slice define, with the item named beside it as `#<id>`, the
name `skill@knowledge-architect-planning@spec-contents` gives an item outside its plan. A
whole plan may be cited from anywhere but a goals home, per
`design@check@goals-home-cites-only-goals-and-root-homes`, a harvested slice excepted, so a
dependency between plans is carried by the document, and retiring a plan asks no other plan to be
redesigned item by item. The premise is that a whole-document citation carries every dependency
between plans that matters while few plans are open at once.

The design this decision did not keep: `rejected@model@cross-plan-item-citation`.

### Item registers are the plan's own register shape, `Section`, whose home is the plan's documents `##items-as-section-registers`

The four item registers have the shape `Section`: each names its level-two section, its entries
sit at level three, and its home is the `README.md` and the `deliberation.md` of each plan document
of the plan anchor's directory, which the plan shape's homes already judge, so a `Section` register
owes no home of its own in phase 2, no index and no walk check. A harvested slice's documents count
for their definitions alone, per `design@model@a-harvested-slice-is-frozen`. The kind is read from
the level-two heading in force above an item, out of the heading records the scanner makes. Four
ordinary heading registers at one home would each claim the same file, and only the first would
define anything.

### The item registers are built in, and carried by the plan anchors alone `##item-registers-built-in`

Like `spec` and `milestone`, the item registers are the tool's, because their storage is the plan
documents, which the plans layout fixes. A declaration of one is refused, per
`design@model@registers-are-declared`.

### A spec's directory is an anchor of its own, so a spec's items are cited by the spec's name `##spec-directory-is-an-anchor`

Each spec of specs/, grouped or not, is an anchor at its directory, named by its id, so its items
are cited `<kind>@<spec>@<id>`, as a slice's are cited by its milestone's name, in every project
that writes specs. The plan anchor stands in the anchor position, so an item citation keeps the
three-part grammar, shorter than the root anchor and a compound id by the root anchor's name and one
separator. The anchor holds every file of its directory, and the directory stays an entry of the
`spec` register that holds it.

### A milestone is a directory, an anchor carrying `spec` for its slices and the four item registers `##milestone-entries-are-directories`

Each milestone is a directory of milestones/, an anchor carrying `spec`, for its slices, and the
four item registers, per `design@model@plan-document-kinds`. Each slice is a directory of the
milestone holding its `README.md`, live, or under the milestone's `harvested/` once it lands, per
`design@model@a-harvested-slice-is-frozen`.

What a plan adds on top of its registers is the tool's and keyed on the register: its documents owe
the sections of their kind, per `design@check@plan-document-sections-by-kind`, its `README.md` and
`deliberation.md` define items, its items are cited from inside the plan only, per
`design@model@plan-item-scope`, and it carries no `path` kind. The `milestone` register exists on
the owner's choice to cite a milestone as `milestone@plans@<id>` over a path to its `README.md`,
which needs no register; no goal ranks the two.

### A slice's directory is no anchor: its milestone is the anchor of every file under it `##slice-directory-is-no-anchor`

The milestone is the anchor of every file under its directory, its slices' included, so it owns
their items, per `design@model@an-entity-belongs-to-its-anchor`, and its slices and itself share one
namespace of items, which the duplicate check reads whole, and the citation forms through the
milestone need no slice anchor. The rival is `rejected@model@slice-directory-as-an-anchor`.

### A harvested slice is frozen: its directory under its milestone's `harvested/` leaves the walk by construction and is parsed for definitions only `##a-harvested-slice-is-frozen`

**A milestone's `harvested/` is the one frozen location.** When a slice lands, its directory is
moved under its milestone's `harvested/`, and the move is the harvested marker: the slice is live
until the commit that moves it, and frozen from that commit on. Every file under `harvested/` leaves
the walk by construction, as the installed files do, per
`design@check@installed-files-outside-the-walk`, and the model parses its markdown files for their
definitions alone. The regime of each file of a plan document's directory:

| file | regime |
| --- | --- |
| `README.md` or `deliberation.md` of a spec, a milestone or a live slice | live: walked, its definitions and its references judged |
| `transcript.txt`, and the `.txt` files of `search/` | never parsed: the walk reads `.md` and `.rs` files alone, `LIVE_SUFFIXES` in `path@model@src/walk.rs` |
| any file under `harvested/` | frozen |

- **Parsed for definitions only**: a slice or an item a frozen file defines resolves from inside its
  milestone; a live definition that reuses one of its ids is the duplicate finding, at the live site
  alone, so the duplicate check sees every number of the milestone; the milestone's `index.md` lists
  the slice, titled from its frozen `README.md`, per `design@model@a-file-register-index-is-rows`;
  and the text of a frozen file raises no finding of its own, while the layout of `harvested/` is
  judged, per `design@model@plan-document-directory`: a file directly under it, and a directory
  there that holds no `README.md` or whose name is no id, is a phase-2 finding.
- **Cited from inside its milestone only**, per
  `design@model@a-harvested-slice-is-cited-from-inside-its-milestone`.
- **Never modified**: no commit modifies a frozen file, or adds one other than by the move that
  freezes its slice; a deletion is admitted, since a frozen file leaves with its milestone. The
  review before a merge judges it, and no check does, per `design@check@a-frozen-file-is-guarded-by-the-review`.

**A slice is judged while it is live, and never after.** A harvested slice records a design its
harvest wrote into the design homes, and the later slices of its milestone go on changing the
entries it cites. Judged against the tree of each later day, it would fail, and a session would edit
it to pass, rewriting the record of what was decided. Under `goal@documentation-stays-consistent`, a
tree that passes conforms in the texts the checker reads; a frozen file is read for its definitions
alone, as an installed file is. The session took that reading of the goal in the discussion of
the milestone that built this, and the owner confirmed it at the review before the merge of the
slice that built this, with the difference between the two classes before the owner: an installed
file's source is judged on every run, and a frozen file's citations on none. The exemption names a location whose files leave the tree with their
milestone, the case `design@check@the-regime-has-no-opt-out` admits: an exemption that names files
leaving the tree expires with its subject. The generated lines of a search and the transcript are
records of their time in the same way; being `.txt` files, the walk never parses them, and they need
no rule of a frozen location.

The rivals, and the facts that defeat them:

- **A frozen file judged only while it is new or changed against HEAD.** No phase of the checker may
  spawn git, per `path@model@CLAUDE.md`, so the comparison would be a question the gathering asks and
  hands in, which none names. And a record written from the tree of its own time fails it in the
  commit that adds it, wherever that commit deletes an entry the record cites: the generated lines
  of the search of the milestone that built this cite the issues that the commit adding the
  milestone deletes.
- **The frozen files left in the walk with their references unjudged.** The heads that state the
  regime of every reference in a walked file, `design@check@the-regime-has-no-opt-out` and
  `design@model@a-reference-is-live-in-every-prose` among them, refuse a walked file whose references
  go unjudged.

### A harvested slice, and every item it defines, is cited from inside its milestone only `##a-harvested-slice-is-cited-from-inside-its-milestone`

A citation of a harvested slice, or of an item it defines, from a file outside the milestone's
directory, a commit message included, is refused, its repair naming the milestone whole, per
`design@model@an-unresolved-reference-is-reported`, and so is a relative link into a harvested
slice from outside the milestone. A frozen file is never judged again, per
`design@model@a-harvested-slice-is-frozen`, so it can state a design a later slice of its milestone
reversed; cited from inside the milestone alone, it is read beside the documents that reversed it,
and every citation of it leaves with the milestone's directory, which leaves whole. The rival, a
harvested slice cited from anywhere, as a live slice is, puts a citation of a frozen record in a
document that outlives it and reads it apart from the documents that reversed it.
