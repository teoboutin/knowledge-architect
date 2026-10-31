# Core design: Entities, ids and headings

## Entities, ids and headings

### Every citeable thing is an entity with a kind, an id, a definition site and, where its kind takes one, an anchor, held in one table built from the model, and every check that resolves a name reads that table `##one-entity-table`

**A kind is a register's name, or `path`, or `planned`, or `register`, or, under the `claude`
harness, a harness kind**: `skill`, `agent`, `primer` or `instructions`, which is no register and takes no anchor, per
`design@core@harness-kinds`. The eleven built-in registers give the kinds `design`,
`goal`, `tripwire`, `rejected`, `issue`, `spec`, `milestone`, `thread`, `argument`, `criterion` and
`acceptance`, and a project's own declarations give the rest, so the kind set is
data rather than a compiled enumeration. A heading register's entities are slugs defined in the
register's home under an anchor, per `design@core@an-entry-is-a-heading-at-the-register-level`; a file register's are the
files under its instance directory, one per entry, per
`design@core@a-file-register-is-a-directory-of-entries`. `path` is defined by the tree itself and is
resolved against the survey, under the same anchors and with the candidate rule and segmentation
the table's resolver applies to every kind, and `planned` with it, per
`design@core@planned-path-form`. `register` names a register's home and is resolved against the
anchors, under the same candidate rule and segmentation, per
`design@core@register-home-reference`. The
table lives in
`path@core@src/entity.rs`, is built once per run from the model, and is what
`check::references` resolves against.

**An anchor is a named directory carrying registers**, per
`design@core@anchors-are-components-and-locations`. It carries its register list and its home base
as data, so a component and a location are two constructors and not two resolvers. The deepest
anchor owns a document, as `design@core@an-entity-belongs-to-its-anchor` states.

**One table, because a notion of a name held per check drifts.** With one table there is one
resolver, one dangling check over every kind, one command that can print any entity given its
reference, and a one-pass rewrite of every reference into a link when the documents are
published. A check that resolves a name of its own is the shape this refuses: two resolvers
disagree the first time one of them is edited, and only the entities one of them knows can be
printed or rewritten.

### Each heading register declares the one heading level its entries sit at, and every heading at that level in its home carries a slug `##an-entry-is-a-heading-at-the-register-level`

**The level is part of the register.** `design` and `rejected` sit at level three, `goal` and
`tripwire` at level two, compiled in; a declared heading register states its level with the `level` key of its
`[registers.<name>]` table, from 2 to 6, and a file register takes none. Level one is the
document's title. With a declared level, "every heading at level N is an entry, and only those"
is a checked invariant, so the tool can tell a heading that is no entry from an entry whose slug
is missing. Without it, a tripwire written without a slug is defined nowhere, listed by nothing
and reported by nothing.

A heading at another level is section text and owes nothing: a design home groups its
level-three decisions under level-two subjects.

**The homes are the design, rejected, goals and tripwires homes of the owning anchor**, and the home of
any heading register the project declares, each in either shape of
`design@core@heading-register-two-shapes`: the single file, or a subdocument of the directory.
The directory's `README.md` is the head: it defines nothing and owes no slug, for every register
and not only design. A plan's items are the one register family read otherwise: their home is the
plan's own documents, a milestone's README included, and the section a heading sits under gives
its kind, per `design@core@items-as-section-registers`. Which anchor owns a definition is where its document sits, per
`design@core@an-entity-belongs-to-its-anchor`.

**The section homes of the harness keep the same rule at level two**, a skill's, an agent's, the
primer and the root CLAUDE.md, per `design@core@section-homes-carry-slugs`: their entries are
the harness kinds', not a register's, and a slug at a level-two heading there defines a section.

A heading at the register's level with no slug in its home is reported, naming the heading. An id
has two segments or more, per `design@core@two-segment-ids`.

Two definitions of one id in one register instance are a finding at each site, each naming the
other.

**Which homes an anchor must carry is `check::tree`'s question, not the table's.** The
table defines from every home shape of every heading register the anchor carries, and reports a
definition that sits where none may; whether the home is there at all, and in which shape, is
`check::tree`'s.

The design this decision did not keep: `rejected@core@slug-at-any-heading-or-cell`.

### Every id the checker defines has two segments or more `##two-segment-ids`

An id is `[a-z0-9]+(-[a-z0-9]+)+`: a register entry's id, a plan document's, a slice's, a plan
item's, a skill's or an agent's name and a section slug, at every site that judges one, the
shipped text's section placeholders included. A slug of one segment is reported where it is
defined, with the repair "rename it with a second segment", so a heading carrying one reads as an
entry with a wrong slug rather than as one with none. A plan document numbers its argument items
`arg-1`, `arg-2`, as `skill@knowledge-architect-planning@spec-contents` states. A register's name
and its directory keep the grammar of one segment or more: they are the kind and the home a
reference names, and no lint on ids reports one as an id.

**The rule is the owner's proposal.** Two segments keep a defined id from being an ordinary word,
so the lints on ids, `design@core@backticked-bare-id` and `design@core@plain-word-lint`, report a
pointer and not a word: in the census of the design discussion, the one-word ids of thaum, its
issues named after a feature area and its argument items, drew most of its plain hits and its
ordinary words in backticks. A rename of the id is a migration, which costs a consumer no capability, per
`design@knowledge-architect@capability-cost-before-migration`: a two-word id
that names the subject without a claim, such as `combat-coverage`, keeps the stability a feature's
own name had. The nearest rival, one-word ids exempt from the lints, leaves every mention of such an
id unchecked for good: `rejected@core@one-word-id-exception`. A second rival, plan items exempt
from the grammar since they are scoped to their plan, adds an exception justified by nothing but
that scope: `rejected@core@plan-items-exempt-from-the-grammar`. The census figures are those of
the design discussion, kept in its spec, `path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`. They are re-taken over a
tree that still holds one-word ids, as a project's at its upgrade, by counting the plain words and
the backticked words equal to each, `git grep -nwF` on each id, since the lints read ids of two
segments only. This serves `goal@knowledge-architect@documentation-stays-consistent`.

### An entry's heading puts its statement before its slug `##statement-precedes-the-slug`

**The statement precedes the slug in the heading**, so a document outline lists the entries
rather than a set of identifiers, and an editor's outline view is the index.

### A slug is read wherever it sits in its heading `##a-slug-is-read-anywhere-in-its-heading`

**The slug may sit anywhere in the heading.** A pattern that requires text after the slug matches
no heading carrying nothing but the slug, and every reference to such an anchor is then reported
as dangling while the definition sits in the file.

### A slug anywhere but at a heading of its register's level in the register's home defines nothing, and is reported as a misplaced definition `##a-slug-elsewhere-defines-nothing`

A slug anywhere but at a heading of its register's level in the register's home defines nothing
and is reported as a misplaced definition: at a heading of another level, level one included, in a
table cell, at the head of a plain line, in the middle of a line, as a second slug on a definition
line, in a file that is no register home and no section home of the harness — a Rust comment
included — or in a directory home's README.

- **A table cell defines nothing.** A decision written as a row has no heading, so the outline does
  not list every decision, and one register would have two definition sites.
- **A slug at the head of a plain line is reported, not accepted.** It is neither a definition
  nor a reference, so every pointer at it dangles and the site itself is named. Accepting it
  beside the heading form would leave the two indistinguishable, and a project moving off the
  line-head form could not tell which of its slugs are left to move.
- **A mid-line slug is a pointer written in the definition form**, and recording it is what makes
  such a pointer visible: a census found such pointers in thaum's design homes, checked by
  nothing, in the message of 86adaf6.

**A fenced heading is an illustration, so a fenced slug neither defines nor is misplaced.** A
definition site is a heading, and the scanner already reads no heading inside a fence; the one
stance covers both. This is the one place a fence still suppresses anything in the grammar —
references are live in a fence for every kind, per `design@core@a-reference-is-live-in-every-prose`.

The design this decision did not keep: `rejected@core@table-cell-definition-site`.

### A heading is a line that opens with `#` marks, and a heading markdown reads in any other shape is reported `##headings-open-with-hash-marks`

The checker reads a heading where markdown reads an ATX heading at the start of a line: one to six
`#` marks, at most three spaces in, then a space or a tab, then text. A line indented four spaces
or more, or by a tab, opens no heading, so a slug on it is a mention and defines nothing; so does
a line whose marks are followed by any other character, a no-break space included. Every other
heading markdown reads is reported in phase 2, at its first line, in every markdown document of
the walk: a setext heading, a line underlined with `=` or `-`; a heading that follows a list
marker or a block-quote marker on its line; and a heading with no text. The repair is to write it
as a line that opens with `#` marks and holds text.

The opposite direction does not hold yet: a heading-shaped line inside an HTML block or a metadata
block that markdown does not render as a heading is still read as one, per
`issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`.

- **It serves `design@core@an-entry-is-a-heading-at-the-register-level`**, which rests on the
  checker reading headings as markdown does: an entry is a heading. A heading markdown shows and
  the checker does not read is a section no check sees. A slug on it defines nothing, and in a plan
  document the items under it are read under the section before it, which changes their kind.
- **Phase 2, because the model is incomplete**, per `design@core@phases-gate-the-report`. A
  finding of the entity table computed while such a heading stands is unreliable in both
  directions.
- **Reported, not read.** The scanner reads one line at a time. A setext heading's level is known
  only at its underline, which follows its last text line, and a heading inside a container starts
  after the container's markers. One shape per heading keeps the scanner line by line, and keeps
  every heading findable by a search for lines opening with `#`.
- **A Rust comment holds no section**, so a comment line underlined with dashes is not reported.

### An entity of a register belongs to its anchor and is unique inside one register instance, and a reference to it names all three: `` `<kind>@<anchor>@<id>` `` `##an-entity-belongs-to-its-anchor`

A definition is owned by where its document sits — the deepest anchor whose path holds it, and
the register whose home holds it — rather than by anything the line says, so moving a document
moves the entities in it. Two anchors may therefore each record an entity they call the same
word, and two registers of one anchor may too, which is what naming the kind and the anchor in a
reference buys. The id of a heading-register entity is called a slug.

**A harness kind names no anchor**, per `design@core@harness-kinds-cited-without-anchor`: its entities live
where the harness puts them, one namespace per kind, so an anchor would carry no information. The
refusal of a reference that names no anchor holds for every other kind.

The designs this decision did not keep: `rejected@core@project-wide-unique-slug`,
`rejected@core@implicit-own-component-reference`, `rejected@core@hash-reference-separator`,
`rejected@core@configurable-reference-separator` and
`rejected@core@register-reference-without-anchor`.

### A reference that resolves to nothing is reported, with the repair its failing segment needs `##an-unresolved-reference-is-reported`

**A reference resolves to nothing in five ways, and each is reported as the repair it needs.**
The kind position holds an anchor, the old form, and the repair names the kinds; the anchor is
not declared, and the repair lists the anchors; the anchor is declared and carries no register of
that kind, and the repair lists the anchors that do; an item of a plan is cited from outside that
plan, and the repair names the whole document, per `design@core@plan-item-scope`; the anchor
carries the register and does not define that id, and the reference is dangling. One finding for
all five would send a reader to check the wrong segment of the pointer most of the time. The third
way is reached by any anchor that does not carry the register: a location that does not declare
it, or a component naming an opt-in register, as the `spec` register only the anchor `plans`
carries. A kind resolved outside the table has ways of its own, each reported as the repair it
needs: `path` and `planned` against the tree, per `design@core@every-path-names-its-anchor` and
`design@core@planned-path-form`, and `register` against the anchors, per
`design@core@register-home-reference`.

**A reference that resolves to nothing is recorded and reported, never dropped.** The scanner could
require a resolvable shape and see nothing without one, which needs no finding and no migration. It
would also mean every pointer written in an older form stops being checked with nothing saying so. A
silent false negative passes a tree that does not conform, against
`goal@core@document-rules-are-checked`: a tree that passes the checker conforms to the reference grammar in the texts the checker reads, save what a declaration switches off. The candidate rule that bounds this is
`design@core@at-span-candidate-rule`.

### `id` answers whether an id is free with every entity the table defines under it, of every kind and every anchor, and leaves to its reader which collision matters `##id-lookup-command`

`klarch id <id>...` prints, for each argument in order, `free` where the entity table defines
nothing under it, every entity it defines under it, as the reference `show` takes, or that the
argument is outside the grammar of `design@core@two-segment-ids`. It reads the table through
`Entities::by_id`, unfiltered, the set the lints on ids filter per citing document. It exits 0 when
every argument is free and 1 otherwise, a malformed argument being a negative answer about the
candidate, per `design@core@exit-code-ladder`, since the run answers every other argument; and 2
over an incomplete model, since a `free` there is the finding computed over an incomplete model that
`design@core@phases-gate-the-report` refuses.

It answers a question no other command answers, per `design@core@one-question-one-command`: `show`
takes a whole reference, and the lints compute the set only to report a span. A design thread's slug
is checked against the design and `rejected` entries of the Component that will own its decision,
per `design@agent-skills@thread-slug-is-entry-id`, and without `id` a session checks it by a loop
over two `show` calls per candidate, the case `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`
gives the checker. An anchor's name, a Component's, a location's or a plan's, is no entity, so `id`
lists none: a plan named like an anchor is refused by `check`, per
`design@core@anchors-are-components-and-locations`. The rule on which collision refuses an id stays with the reader, since another
reader, such as a label or a new entry of another register, has another rule. The nearest rival, a
test of one named anchor's design and `rejected` entries, needs the owning anchor before the thread
can name it: `rejected@core@slug-test-of-one-anchor`.
