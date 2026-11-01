# Core design: Registers and their homes

## Registers and their homes

### The designs a project did not keep are entries of a built-in heading register, `rejected`, at level three `##rejected-register`

Every component carries the register `rejected`, its home `register@*@rejected` a file
`rejected.md` or a directory `rejected/` with a README under the component's docs/, per
`design@core@heading-register-two-shapes`. An entry is a level-three
heading that names the design not kept, its slug last, and its body opens with a marker, `live` or
`void`, then the reason it was not kept. It is cited `rejected@<anchor>@<id>`, so `show` prints it
whole with every text that cites it, and a plan item, a review finding, a commit message or a head
can name one design.

It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`, which asks for the
alternatives that lost and why, and `goal@core@records-reach-their-reader`, which asks that a
recorded entry reach whoever needs it through checked references: an entry with no slug can be
cited by nothing, and `show` prints an entry whole only when it is one. The home stays apart from
the design home, on the owner's argument: an agent reading a decision file partly, by grep, could
take a rejected design for what is built, and separating the two at the level of the files keeps a
grep hit on one from reading as the other. The nearest rival, the paragraphs of a compiled
document with `show` printing each one that cites a head, delivers only to a session that runs
`show` on the right head, and leaves nothing for an item, a finding or a message to cite. The name
is `rejected`, and the home is named after it, both on the owner's word: the owner proposed the
name for a shorter citation, finding a register of alternatives alone a bit odd, and judged the
new home's name to read better and to avoid a confusion point. Which designs earn an entry is
`design@agent-skills@losing-alternatives-filter`.

The designs this decision did not keep: `rejected@core@paragraphs-with-show-printing-them`,
`rejected@core@kind-named-rejected-alternative`, `rejected@core@home-keeping-the-old-file-name` and
`rejected@core@validity-marker-checked`.

### A tripwire guards no `rejected` entry `##tripwire-guards-no-rejected-entry`

The `guarding` column of `cargo klarch tripwires`, and its `--guarding` filter, read every
reference of a tripwire to an entry of a register but `issue`, `tripwire` and `rejected`. A
tripwire guards a decision: a `rejected` entry records a design not kept, which is no decision, so
a tripwire that names one names the candidate its response would reopen, not what it guards. The
rival, counting it like a declared register's entry, would list a design not kept as guarded.

### The checker does not check a tripwire's horizon line, for now `##tripwire-horizon-unchecked`

A tripwire's `**Watches until:**` line, which `design@agent-skills@tripwire-horizon` gives every
entry, is written and read by the sessions and the reviewers that the installed workflow sends, and
no check of the core reads it. Whether a horizon has passed is an event over the history the
checker does not judge, and a presence check would read a field of a heading entry, the syntax
`rejected@core@validity-marker-checked` refused for the `rejected` register. The owner chose to
keep the line out of the checker for now, judging a check not yet needed. The tripwire axis of the
project audit brings every entry to the line, per `design@agent-skills@tripwire-audit-axis`, and the
standing-state reviewer reads each entry written after it, per
`design@agent-skills@standing-state-review-before-every-merge`.

### A `rejected` entry that no design entry cites is a finding `##orphan-rejected-entry`

The last phase reports, at its definition site, every `rejected` entry that no entry of a `design`
register cites inside its own section, in any anchor. A citation from any other text, a goal, an
issue, a tripwire, another `rejected` entry, or a design home outside its entries, does not count.
The finding names the repair: cite it from the design head whose argument covers it, or from a head
of absence that records the decision not to build it; or, where what it lost to is an instruction no
head states, delete it and name the design and its reason beside that instruction, which
`skill@knowledge-architect-decision-recording@losing-alternatives` admits since no head cites it. It is raised over a tree in
`path@core@src/check/references.rs`, not over a commit message, which holds no design entry.

It serves `goal@core@document-rules-are-checked`: a head cites the designs its argument
covers, per `design@agent-skills@rejected-citation-direction`, and this check is what makes every
`rejected` entry reachable from the text a design session reads, the head, rather than only from
`show`. Only a citation from a design entry counts, on the owner's judgement; a citation counts by
the reference rule of `design@agent-skills@a-reference-claims-a-revisit`.

The designs this decision did not keep: `rejected@core@goal-citing-a-rejected-entry`,
`rejected@core@entry-reachable-by-citing-a-goal` and `rejected@core@no-orphan-check`.

### Which registers exist beyond the built-in ones is the manifest's `##registers-are-declared`

`[registers.<name>]` declares a register's `scope`, `shape`, `dir`, `level`, `sections`,
`metadata.<key>.values`, for a Directory register `carries`, per
`design@core@declarable-directory-register`, and for an `opt-in` register the `components` that
carry it, per `design@core@a-component-carries-what-a-project-attaches`; `[locations.<name>]`
declares a directory and the registers it carries.
Eleven registers are compiled in — `design`, `goal`, `tripwire`, `rejected`, `issue`, carried
by every component; `spec` and `milestone`, carried by the anchor `plans`, `spec` also by each milestone
anchor, per `design@core@plan-register`; and the four item registers, carried by the plan
anchors, per `design@core@item-registers-built-in`. A declaration for one of the first five accepts `kinds` on `issue`
and nothing else, because their storage is what the word component means. A table for a plan
register or an item register is refused whole, and so is any of their names in a location's
`registers` list, and a
component register whose home at the root would be the plans directory: the plans layout is the
tool's, and a declaration that changes it changes nothing a project can rely on. A declared
register named `path`, `planned`, `register` or a harness kind is refused too: a reference whose
kind segment is that word never reaches the register. Setting `scope`, `shape`, `dir`, `level`,
`sections`, `metadata`, `carries` or `components` on a built-in is a finding, and the compiled
value stands.

**Declarations go in the manifest because its header promises that what is checked is said in one
place.** Its bulk is comments, which is what a declaration owes a reader.

### An `opt-in` register declared with `dir = "."` keeps its home at its carrier's own directory, and that carrier carries no other register `##home-at-the-carrier-path`

A location at `<path>` carrying such a register is that register's home, so a Directory register
places its entries directly under the location, as `<path>/<entry>/`, and a heading register's
subdocuments sit in the location's directory, behind its README. Any other register the location
named would keep its home inside that one, one path with two meanings, so the location carries the
first such register it names and the others leave its row, each a phase-1 finding, per
`design@core@anchor-collisions-refused`. At component scope it is refused: every component would
keep the home at its own `docs/`, among its other homes. A register at `.` has no single-file
shape, which would be `<path>.md`, outside the location: a file beside the location is one more
document of the anchor holding it.

**It serves a home directly at a directory outside every Component's `docs/`**, such as subjects
at `<path>/<subject>/` under the project's root. Its nearest rival is an ordinary `dir`, which puts
the home one segment under the location, at `<path>/<dir>/`, a directory level the records do not
need. A home under a Component's `docs/` is placed by attaching the register to the Component, per
`design@core@a-component-carries-what-a-project-attaches`, with no location to name. The owner chose
to keep this key beside that attachment rather than replace it: `rejected@core@carrier-dir-replaced-by-components`.

### A register instance's own options sit beside it, in `register.toml` `##instance-options-beside-the-instance`

Per-instance options go in `docs/<dir>/register.toml`. They cannot sit in
the manifest without a table keyed by anchor, and the shape that avoids the table — a single list
at the root — would put one component's groups in another's directory. Today the only such option
is `groups`. A key other than `groups` is a finding, a file with no `groups` is a finding, and a
`register.toml` beside a heading register is one too.

The design this decision did not keep: `rejected@core@instance-options-at-root`.

### A declaration that is wrong is a phase-1 finding, and absent from the configuration `##a-wrong-declaration-is-a-finding`

A declaration that is wrong is a finding rather than a load failure, and it is absent from the
configuration. A manifest that will not load reports nothing at all, and nothing at all is what a
session reads as conformance, against `goal@core@document-rules-are-checked`: a tree that passes the
checker conforms to what its manifest states. So `Manifest::parse` resolves the declaration into a
configuration and a list of complaints, each in the shape a finding takes, and every complaint is
the first phase of a run, per `design@core@phases-gate-the-report`. What a complaint is about is not
in the configuration: a refused register is no register, a refused anchor is no anchor, a refused
row is not in its list, so nothing acts on it and its consequences are never reported as defects of
the tree. An extension's tables follow the same rule, per
`design@core@an-extension-claims-its-manifest-tables`: its complaints are phase-1 findings, and a
run that holds one stops before the extension reads anything.

The design this decision did not keep: `rejected@core@refusing-a-wrong-declaration`.

### A heading register's home is `<dir>.md` or a `<dir>/` directory, never both `##heading-register-two-shapes`

The single file serves an anchor whose register fits one document. The directory serves one whose
register has outgrown it. In the directory shape, `README.md` is the head: an introduction, and a
bullet list of markdown links, one per subdocument, each target written relative to the README.
The entries and their slug anchors live in the subdocuments.

**Both shapes serve every heading register**, so `design`, `goal`, `tripwire` and `rejected` accept the same
two, and so does any heading register a project declares.

Five assertions carry the shape, split over three places. `check::tree` asserts exactly one
home per anchor and register and that a directory home carries its `README.md`;
`check::registers` asserts that the README links every markdown subdocument the walk covers — a gitignored scratch file owes nothing. That
each relative link resolves is the references family's assertion, made for every navigation file
alike per `design@core@links-are-navigation-rows`. The entity table of `design@core@one-entity-table`
accepts a slug definition only in the file home or in a subdocument, matched against the owning
anchor's own paths rather than against a filename suffix — a suffix match would accept a slug
in any file whose name ends in `design.md`, a plan document included.

**Exactly one home, because two give a decision two candidate places to land.** That is the
one-home failure, and the reader who finds one half acts on half the design.

**The README must link every subdocument, because a subdocument nobody links is a home nobody
finds.** A markdown link is the index's row because a reader can follow it where a renderer
shows the page, and a bullet list is its conventional shape — a table grows painful to edit as
soon as a row outgrows a short sentence. Both directions are asserted over the same links: an
existing subdocument no link names is reported, and a link that resolves to nothing is reported.
A fenced link is an illustration and discharges nothing, the stance a fenced definition takes
per `design@core@a-slug-elsewhere-defines-nothing`, and a link inside a code span is typography showing the
shape. **A row is any link shape `design@core@links-are-navigation-rows` reads**, a link
definition included, so a README written with reference-style links indexes its subdocuments
like one written with inline links.
A backticked path in the README stays what it is everywhere: a checked pointer in prose, not a
row of the index.

**The README defines no slugs.** It is the head and the index. A decision recorded there competes
with the subdocuments as a home, which is what the split exists to end.

### A file register is a directory of one file per entry, with a hand-written README and a generated index `##a-file-register-is-a-directory-of-entries`

Its home is `<dir>/`. Every `.md` file under it is an entry named by its basename, except
`README.md` and `index.md` at the instance's own top level; a subdirectory is a declared group
and nothing else, groups nest one level, and a file of another suffix is a finding. An entry
carries frontmatter with each declared metadata key, a level-one title, the declared level-two
sections in order, and — for the issue register — the level-three subsections its kind owes.

**One file per entry, because a tool that edits a tracker edits the whole file.** Sessions editing
a shared tracker with `sed` and a string replacement have replaced every occurrence in the file
more than once. One file per entry makes that failure unreachable across entries, gives each entry
its own history, and turns closing an entry into a dangling-reference work list: the file is
deleted, and every reference to it becomes a finding.

**The README is hand-written and the index is generated, and neither stands in for the other.**
The introduction is prose about the project and belongs in markdown, and the README is also what
keeps the directory in git, which an empty directory cannot be. The index is mandatory: with the
README already holding the directory open, the churn argument against generating one has no weight
left, and a listing nobody generates is one that silently stops listing. This head owns that the
index exists; what it holds is `design@core@a-file-register-index-is-rows`, and that it is generated
rather than written by hand is `design@core@generated-files-are-pure`.

The designs this decision did not keep: `rejected@core@index-introduction-in-manifest` and
`rejected@core@optional-index`.

### A file register's groups are subdirectories, declared beside the instance `##groups-are-subdirectories`

A subdirectory is visible to `ls` and to a listing without parsing anything, and the group is not
part of an entry's id, so regrouping is a `git mv` that breaks no reference. The declared list is what keeps the naming from
drifting, and it is checked in both directions. An entry may sit ungrouped at the top level, so a
two-entry instance owes no directory.

The design this decision did not keep: `rejected@core@theme-metadata-key`.

### An entry's frontmatter is a small subset, refused loudly outside it `##frontmatter-subset`

**The frontmatter subset is small and refused loudly outside itself**, per
`design@core@a-failed-parse-is-loud`: a block opened and closed by a line holding only `---`, at the
very top of the file, holding `key: value` lines with scalar values, no key written twice. A key
the register does not declare is a finding, so a typo in a key name cannot pass as an absent
optional. Frontmatter is required exactly when the register declares metadata, and refused when it
declares none — an undeclared block is metadata nothing checks.

**A blank line between the delimiters means there is no block**, which is what tells a block from
a document opening on a thematic break: a paragraph between two thematic breaks has blank lines
around it, and a block has none.

### An entry's frontmatter block is read as prose, outside the document's structure `##frontmatter-is-prose`

**The block is prose, entire, and its lines are marked as a fence.** That marking keeps a value
out of the document's structure — a heading, a slug definition, a navigation link — and buys
nothing else: a fence has never made its contents data here, per
`design@core@grammars-not-prefixes`. So a reference in a value is a reference, and an
extension that reads its own markers in prose reads them in a value too.

**The block is blanked byte for byte before the structure is read.** The closing `---` is a setext
underline for the `key: value` lines above it, so left in place a block would open the document
with a level-two heading nobody wrote. Each byte becomes a space rather than being removed, so
every offset the analysis returns is an offset into the file as written.

### The issue register's kind list is closed `##issue-kind-list-closed`

An unknown kind is a finding naming the list, and adding one is a reviewed manifest diff. The kind
decides which subsections the entry owes, so an unknown one would owe the wrong three in silence, which is what makes refusing it load-bearing
rather than tidy. The open set is `rejected@core@open-issue-kind-set`.

### A project declares a Directory register, each entry an anchor carrying the register its `carries` names `##declarable-directory-register`

`shape = "directory"` with `carries = [<register>]`: its home holds a hand-written `README.md`,
a generated `index.md`, and one directory per entry, each holding a file `README.md`, which is the
entry's document and owes a level-one title. Nothing else sits in the home, and no entry holds a
`register.toml` or a subdirectory; each of these is a phase-2 finding, because the entries are
anchors read off the tree. An entry is defined at its `README.md`, and only while the walk reads
that file, as a File entry the walk leaves out defines nothing, and it is cited
`<register>@<parent>@<id>`, the parent being the anchor that carries the home.

**Each entry is an anchor carrying, at its own directory, the one register `carries` names**, an
`opt-in` register of the heading or file shape, one a location could carry: a component-scoped
register is carried by every component already, a plan register by the plan anchors alone, and a
Directory register carried by an entry would nest anchors inside anchors, which no case has asked
for. **One register, on the owner's ruling**: a second would keep its home at the same directory,
one path with two meanings, which `design@core@anchor-collisions-refused` refuses for every other
anchor, and a home of its own under the entry would put a directory level between a subject and
its files. How a carried home is shaped is `design@core@entry-anchor-carries`, and what names an
entry's files is `design@core@entry-anchor-path-kind`.

**It is declared because a project can hold records whose subjects are directories**, as a game's
design split by subject: one subject per directory, several files each, every entry cited
`<kind>@<subject>@<id>`, which `goal@knowledge-architect@every-target-project-can-adopt-it` asks the manifest
to express without anything of the project compiled in. Adding a subject is creating a directory,
not a manifest edit, and moving the home is one manifest edit, per
`goal@core@relocation-is-one-manifest-edit`. A location declared per subject costs a manifest row
per subject. The rival that makes each subdirectory of a location an anchor, with no Directory
register, is `rejected@core@subdirectory-anchors-location`.

### A register an entry carries keeps the directory shape alone, the entry's README its head `##entry-anchor-carries`

The home of what an entry carries is the entry's own directory, beside its `README.md`, which is
both the entry's document and the head of that home: it links every file of a carried heading
register and defines no slug, per `design@core@heading-register-two-shapes`. The single-file shape
would be `<id>.md` beside the entry directory, so a heading register at an entry would have two
homes, which that head refuses: `<id>.md` beside an entry is a phase-2 finding. A file register an
entry carries keeps its generated `index.md` in the entry, as a milestone's slice specs do.

### An entry of a declared Directory register carries `path`, and its directory and README are named by the entry's reference `##entry-anchor-path-kind`

A file inside an entry, other than its README, is no entry of any register: it is a subdocument of
the heading register the entry carries, so a `path` is its only name, `path@<entry>@<file>`. The
rival is the plan anchors' rule, which carries no `path` kind because every plan document has a
kind's name, per `design@core@plan-document-kinds`; a subject's file has none. The entry's directory
and its README are the entry itself, so a `path` or a `register` reference to either is a finding
whose repair names `<register>@<parent>@<id>`, per `design@core@path-to-a-home-is-reported` and
`design@core@register-home-reference`: one directory has one name, so `show` finds every citation.
