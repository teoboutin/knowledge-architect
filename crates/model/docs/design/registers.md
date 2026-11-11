# Model design: Registers and their homes

## Registers and their homes

### Which registers exist beyond the built-in ones is the manifest's `##registers-are-declared`

`[registers.<name>]` declares a register's `scope`, `shape`, `dir`, `level`, `sections`,
`metadata.<key>.values`, for a Directory register `carries`, per
`design@model@declarable-directory-register`, and for an `opt-in` register the `components` that
carry it, per `design@model@a-component-carries-what-a-project-attaches`; `[locations.<name>]`
declares a directory and the registers it carries.
Eleven registers are compiled in — `design`, `goal`, `tripwire`, `rejected`, `issue`, carried
by every component; `spec` and `milestone`, carried by the anchor `plans`, `spec` also by each milestone
anchor, per `design@model@plan-register`; and the four item registers, carried by the plan
anchors, per `design@model@item-registers-built-in`. A declaration for one of the first five accepts `kinds` on `issue`
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
`design@model@anchor-collisions-refused`. At component scope it is refused: every component would
keep the home at its own `docs/`, among its other homes. A register at `.` has no single-file
shape, which would be `<path>.md`, outside the location: a file beside the location is one more
document of the anchor holding it.

**It serves a home directly at a directory outside every Component's `docs/`**, such as subjects
at `<path>/<subject>/` under the project's root. Its nearest rival is an ordinary `dir`, which puts
the home one segment under the location, at `<path>/<dir>/`, a directory level the records do not
need. A home under a Component's `docs/` is placed by attaching the register to the Component, per
`design@model@a-component-carries-what-a-project-attaches`, with no location to name. The owner chose
to keep this key beside that attachment rather than replace it: `rejected@model@carrier-dir-replaced-by-components`.

### A declaration that is wrong is a phase-1 finding, and absent from the configuration `##a-wrong-declaration-is-a-finding`

A declaration that is wrong is a finding rather than a load failure, and it is absent from the
configuration. A manifest that will not load reports nothing at all, and nothing at all is what a
session reads as conformance, against `goal@documentation-stays-consistent`: a tree that passes the
checker conforms to what its manifest states. So `Manifest::parse` resolves the declaration into a
configuration and a list of complaints, each in the shape a finding takes, and every complaint is
the first phase of a run, per `design@check@phases-gate-the-report`. What a complaint is about is not
in the configuration: a refused register is no register, a refused anchor is no anchor, a refused
row is not in its list, so nothing acts on it and its consequences are never reported as defects of
the tree. An extension's tables follow the same rule, per
`design@check@an-extension-claims-its-manifest-tables`: its complaints are phase-1 findings, and a
run that holds one stops before the extension reads anything.

The design this decision did not keep: `rejected@model@refusing-a-wrong-declaration`.

### An entry's frontmatter is a small subset, refused loudly outside it `##frontmatter-subset`

**The frontmatter subset is small and refused loudly outside itself**, per
`design@model@a-failed-parse-is-loud`: a block opened and closed by a line holding only `---`, at the
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
`design@model@grammars-not-prefixes`. So a reference in a value is a reference, and an
extension that reads its own markers in prose reads them in a value too.

**The block is blanked byte for byte before the structure is read.** The closing `---` is a setext
underline for the `key: value` lines above it, so left in place a block would open the document
with a level-two heading nobody wrote. Each byte becomes a space rather than being removed, so
every offset the analysis returns is an offset into the file as written.

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
one path with two meanings, which `design@model@anchor-collisions-refused` refuses for every other
anchor, and a home of its own under the entry would put a directory level between a subject and
its files. How a carried home is shaped is `design@model@entry-anchor-carries`, and what names an
entry's files is `design@model@entry-anchor-path-kind`.

**It is declared because a project can hold records whose subjects are directories**, as a game's
design split by subject: one subject per directory, several files each, every entry cited
`<kind>@<subject>@<id>`, which `goal@every-target-project-can-adopt-it` asks the manifest
to express without anything of the project compiled in. Adding a subject is creating a directory,
not a manifest edit, and moving the home is one manifest edit, the property
`design@check@every-path-names-its-anchor` buys for every anchor. A location declared per subject costs a manifest row
per subject. The rival that makes each subdirectory of a location an anchor, with no Directory
register, is `rejected@model@subdirectory-anchors-location`.

### A register an entry carries keeps the directory shape alone, the entry's README its head `##entry-anchor-carries`

The home of what an entry carries is the entry's own directory, beside its `README.md`, which is
both the entry's document and the head of that home: it links every file of a carried heading
register and defines no slug, per `design@check@heading-register-two-shapes`. The single-file shape
would be `<id>.md` beside the entry directory, so a heading register at an entry would have two
homes, which that head refuses: `<id>.md` beside an entry is a phase-2 finding. A file register an
entry carries keeps its generated `index.md` in the entry, as a milestone's slice specs do.

### An entry of a declared Directory register carries `path`, and its directory and README are named by the entry's reference `##entry-anchor-path-kind`

A file inside an entry, other than its README, is no entry of any register: it is a subdocument of
the heading register the entry carries, so a `path` is its only name, `path@<entry>@<file>`. The
rival is the plan anchors' rule, which carries no `path` kind because every plan document has a
kind's name, per `design@model@plan-document-kinds`; a subject's file has none. The entry's directory
and its README are the entry itself, so a `path` or a `register` reference to either is a finding
whose repair names `<register>@<parent>@<id>`, per `design@model@path-to-a-home-is-reported` and
`design@model@register-home-reference`: one directory has one name, so `show` finds every citation.
