# Core design: Paths, homes and links

## Paths, homes and links

### Every path reference names its kind and its anchor, `` `path@<anchor>@<path>` ``, and the deepest anchor wins `##every-path-names-its-anchor`

A checked path reference is one backticked span in the one grammar of
`design@core@an-entity-belongs-to-its-anchor`, with `path` as its kind and the path under the named
anchor's directory as its id. The root is a component like any other, named by `project.name`.
The kind prefix costs five characters at every path reference and buys one grammar: the scanner
has one tokenizer, the old two-segment form `<anchor>@<path>` is reported as an anchor in kind
position by `design@core@at-span-candidate-rule`, and the unanchored lint is left with
one job. There is no unanchored form for a path of this tree: a backticked span of path characters
with two or more segments and no `@` is a finding naming the grammar when its first segment names a
file or a directory this tree's listing holds. One segment is a name rather
than a path, though the exact name of a skill or an agent is reported, per
`design@core@backticked-bare-id`, and a span holding a space, an angle bracket, or a colon anywhere but in a line
suffix is not path-shaped, which is what lets documentation of the syntax show a placeholder with no
carve-out.

**A span whose first segment names nothing here is another tool's notation, and is silent**: a
media type `application/json`, a unit `km/h`, a git ref `origin/main`, the fixture path of a test,
a path on another machine. The first segment is read from the root, from every anchor that is a
directory, and from the document's own directory, a `..` climbing from each; a leading slash is
read from the root alone. A spec is an anchor and a file, so it is no place a path is read from.
Only the first segment is asked, so a pointer whose first segment is here and whose rest is not is
still a finding, and the anchored form it is rewritten in reports it dangling. A commit message is
judged against two trees, per `design@core@a-commit-message-is-a-document`.

**The listing alone answers; the ignore rules are not asked.** A pointer into an ignored directory,
such as a build output, is silent. Asking git costs one spelling per span and per place it is read
from, makes every root name a pointer under a whitelist `.gitignore`, and fails the run outright
for a spelling through a symlink, which git refuses.

**What it costs**: a pointer written after its first segment left the tree is silent, and so is
one whose first segment carries a typo or names an ignored directory; one written while its first
segment stood was reported then. **What it buys** is the silence of another tool's notation, which
is nearly every path-shaped span of a project new to the tool. Re-take it with `cargo klarch check`
over a foreign project carrying the required structure, counting the findings under each rule; a
census in which the silence hides a real pointer of this tree reopens it. The census that decided
it is in the commit that rewrote this head, found with
`git log --grep='only where it names something of this project'`. Reporting every path-shaped span
is `rejected@core@every-path-shaped-span-reported`. This serves `goal@knowledge-architect@every-target-project-can-adopt-it`
and `goal@knowledge-architect@adoption-is-easy`.

**A line suffix or a fragment keeps a span path-shaped, after a file with an extension.** A
path followed by `:12`, the location an editor prints, and a path followed by a `#` and a
heading fragment opening with a letter each point into a file, so each is a finding. Without an
extension, or with a number after the `#`, the suffix is an image tag or an issue number, and
the span is not path-shaped. Its repair names the file and drops the suffix: a reference is to a file,
and a line number goes stale at the next edit.

**Four shapes are outside the lint on purpose**, each silent:

- a path in plain prose, with no backticks. The backticks are what mark a pointer, and plain
  text stays free prose.
- a span holding a space, such as a path with a space in a name. It is prose as much as a
  path.
- a span opening with `~/` or `$VAR/`. It names a path outside every tree this tool checks.
- a span written with a backslash separator. No project this tool checks writes one, and the
  census finds none.

Each widening would be measured the way the two-segment clause was: a census of what it
reports in a tree that is conformant. The census that kept these four outside is in the commit
that recorded them, found with `git log -G'outside the lint on purpose'`.

**The path is plain**: `..`, a `.` segment and a leading `/` are refused. An upward path is
anchored at the wrong place by definition, and it is the shape that breaks when the referencing
file moves. The id is everything after the second `@`, so a path may itself hold an `@`.

**The deepest anchor wins, and inside means a proper descendant.** A reference whose target
sits inside another anchor is refused with the right anchor named. What this buys is the
same property `design@core@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
anchor edits its one line of the project's `knowledge-architect.toml` and no document, which
`goal@core@relocation-is-one-manifest-edit` asks, and one fixed-string grep per anchor is an
anchor's complete inbound-reference list. An anchor's own directory is the one target with no
spelling under its own name, so it is named from an ancestor — a reference that names a
location, which a move is expected to break.

**A plan document is cited by its kind, never by its path**, per
`design@core@plan-document-kinds`. A
`path` citation of a spec, of a milestone directory or of a file inside one is refused from every
anchor, and the finding names the form that resolves: `spec@plans@<id>`, `milestone@plans@<id>`
or `spec@<milestone>@<slice>`. It is judged by where the target sits, before the deepest-anchor
rule and whether or not it exists, so a citation of a deleted plan gets the same repair. This
overrides the ancestor's spelling of an anchor's own directory for a milestone. A plan anchor, a
milestone or a spec, carries no `path` kind. One name per document is what keeps `show` complete: a citation written as a path
is one `show spec@…` would not list. The README of the plans directory, and the README and index
of each of its two homes, are not plan documents, and are cited `path@plans@<file>`.

**A fenced path reference is live**, as every reference is, per
`design@core@a-reference-is-live-in-every-prose`: a sketch names its paths on purpose, and an
illustration that needs a fake path writes the escape anchor or an angle-bracket placeholder.

The designs this decision did not keep: `rejected@core@bare-root-relative-path`,
`rejected@core@path-without-kind-prefix`, `rejected@core@at-prefix-root-relative-path`,
`rejected@core@retired-form-rs-suffix` and `rejected@core@ignored-first-segment-reported`.

### The escape for a path this tree does not hold, under `path`, and `*` for every component's own copy, under `path` and `register`, are reserved anchor words `##reserved-anchors`

**The escape anchor** — the word elsewhere in anchor position, `path@elsewhere@<path>` — marks a
path deliberately not resolvable here: a surveyed engine's layout, a deleted file a tracker
entry discusses, a hypothetical location. It is scanned and counted like any reference, so a
typo'd escape is still a registered pointer; it is exempt from the existence, kind and shape
assertions, because a foreign layout may spell anything. One assertion runs against it: a
target that DOES resolve here, read from every place the unanchored finding reads a path from, the
root, an anchor's directory or the document's own, is a finding — without it, the escape is the
cheap way to silence the unanchored finding on a real path. The finding names the form at the
deepest anchor that holds the target, per `design@core@every-path-names-its-anchor`.

**A file of another project is written with that project's name as its first segment**, as
`path@elsewhere@<project>/<path>`, and the refusal's repair names that form. A foreign file often
shares its relative path with a file here, such as a dependency's src/lib.rs, and without the
prefix the assertion above refuses it. A repair that names only the local anchor leaves the writer
of a foreign pointer no checked form, and prose follows. The prefix is no cheap silence: it is false of
every file in this tree, so writing it before a local path is a deliberate misstatement, which costs
what removing the backticks costs, and that move is refused as a repair by
`design@agent-skills@plain-text-is-no-repair`.

**The generic anchor `*`**, `path@*@<path>`, marks each component's own copy of a path, as in
`path@*@README.md`. It is accepted when the path is a required document name
— a compiled document, or a file register's README or generated index — and otherwise when at
least one component carries the path with the claimed kind. The file-register set is derived from
the component-scoped registers rather than written out, so a project that declares one gets its
names in the set. A register's home is no `path` target, per
`design@core@path-to-a-home-is-reported`: `register@*@<register>` names every component's own home
of a component-scoped register, in whichever shape each holds, per
`design@core@register-home-reference`. A generic reference nothing resolves rots exactly like a
dangling one. The anchors the tool constructs hold no copy the generic form names: what they hold
is cited by its kind.

**Under any other kind the escape names nothing, and `*` names nothing but under `register`.** A
`design@*@<id>` is reported as an unknown anchor, and the finding says which kinds each word
serves. The `planned` kind refuses them
with a finding of its own, per `design@core@planned-path-form`. The manifest's resolution
refuses a declared anchor wearing either word, and it is no anchor: every pointer at it
would read as the reserved meaning.

The designs this decision did not keep: `rejected@core@at-prefix-escape` and
`rejected@core@component-placeholder-anchor`.

### A register's home is cited by its own kind, `register@<anchor>@<register>`, which names the home in whichever shape it has `##register-home-reference`

`register@<anchor>@<register>` names the home that the anchor keeps the register in. The check
resolves it against the anchors, which the table's module holds, and asks nothing of the tree,
since a missing home is the tree check's finding; `show` reads the tree's listing to print the
home it holds. No register's name becomes an entity id, so a register's name in backticks with a
hash sign before it stays silent under `design@core@retired-slug-reference-reported`.

- **The anchor** is a component, a declared location or the anchor `plans`, where it carries the
  register; `*` names every component's own home of a component-scoped register, per
  `design@core@reserved-anchors`. An entry or a spec anchor is no anchor of the form: an entry's
  homes are its own directory, which the entry's reference already names, as
  `milestone@plans@<id>` names a milestone's, and one document has one name, per
  `design@core@plan-document-kinds` and `design@core@declarable-directory-register`.
- **The id** is a register's name, built in or declared. An item register is no id of the form:
  its home is a plan's own documents, cited by the plan document's kind.
- **Each refusal names the form that resolves**, per `design@core@finding-names-the-repair`.
- **`show`** prints the home the tree holds, the file or the directory, with the register's shape,
  then every site that cites it, a citation of a component's home through `*` included.

**The kind exists because a home has two shapes**, per `design@core@heading-register-two-shapes`.
A `path` names one of them, so a home that outgrows one file dangles every `path` to it, against
`goal@core@relocation-is-one-manifest-edit`, and a text that must hold in every project names both
shapes, per `goal@agent-skills@installed-text-works-in-every-target-project`. This reference names the register and lets the manifest and the tree say which shape it is.
The nearest rival is a `path` that resolves to either shape; it loses on
`design@core@trailing-slash-claims-directory`, since a path claims its target's kind, and a path
that matches either shape asserts no claim and puts a special case into path resolution.

**The kind's word is the owner's choice**: "I'd rather use register. home just reads weirdly." The
rival word, `home`, matched the documents' phrase "design home" better and cost the same refusal of
an anchor name. A kind's name is refused for every anchor, per
`design@core@kind-names-refused-as-anchors`, and for a declared register, per
`design@core@registers-are-declared`. The cost the form keeps: a reader outside the tool does not
see the file a reference names, which `issue@core@the-documents-do-not-render-as-a-site` would turn
into a link.

The designs this decision did not keep: `rejected@core@home-kind-word` and
`rejected@core@path-to-either-home-shape`.

### A `path` reference whose target is a register's home is a finding, whose repair names the `register` form `##path-to-a-home-is-reported`

A home target is a heading register's file, its directory, or that directory's `README.md`, which
is the home's head per `design@core@heading-register-two-shapes`; a file register's directory; a
Directory register's directory. A file register's `README.md` and generated `index.md`, a
subdocument and an entry are files inside a home, and stay `path` targets. **An entry's directory
and its `README.md` are no home target**: they are the entry, and a `path` to either is a finding
whose repair names the entry's reference, `<register>@<parent>@<id>`, per
`design@core@declarable-directory-register`; a milestone's, by `design@core@plan-document-kinds`.

- The finding is raised for a named anchor after the refusals of a refused shape and of a plan
  document cited by its path, on the target resolved to the anchor that keeps the home, and in
  place of the deeper-anchor finding: its repair names `register@<that anchor>@<register>`. Under
  `*`, it is raised before the required set and any kind claim are judged, and names
  `register@*@<register>`.
- The kind claim and the existence of the target are not judged: the manifest decides whether a
  path is a home, and either shape names the same register.
- A line number or a fragment is stripped before the test, and the repair says to drop it.
- A `planned` reference is not judged by it.

**Both forms legal** loses to this finding: a writer keeps writing the `path`, and a home that
changes shape then dangles every one of them, which `design@core@register-home-reference` exists to
prevent, against `goal@core@relocation-is-one-manifest-edit`. The finding repairs nothing by itself:
no fix of `check --fix` respells it.

The design this decision did not keep: `rejected@core@path-to-home-beside-register-form`.

### The word `plans` names the anchor the tool constructs at the plans directory, and no project may declare it `##plans-is-a-reserved-anchor`

The anchor is constructed per `design@core@plan-register`. The word is reserved like the two words
of `path`, so a citation of a plan reads `spec@plans@<id>` in every project, with no project name
in it: there is one plans directory, so the root anchor in every such citation would carry no
information. The manifest's resolution refuses a declared anchor named `plans`, and it is no
anchor: every pointer at it would read as the reserved meaning.

### A path a plan's work will create is cited `planned@<anchor>@<path>`, from the plans directory only, and reported once it exists `##planned-path-form`

A plan document names files and directories its work will create. The kind `planned` names one:
its anchor and its path follow every rule of a `path` reference, the named anchor, the deepest
anchor, the refused shapes and the trailing-slash claim, and its one assertion is the reverse of
`path`'s: the target does not exist. Once it exists, the finding names the `path` form and
nothing else, so the commit that creates the file converts the reference by its kind word, and
whether the plan still holds is that session's report to make. A target the ignore rules cover is
not asserted, per `design@core@ignored-targets-are-not-asserted`. `*` and the escape anchor are
refused: a planned path is in this tree, at one anchor.

**It is legal in the plans directory alone**, since a plan document is the one place that
describes unbuilt work, per `design@core@plan-register`. Elsewhere it is a finding that asks for
the plan that creates the file. An issue entry states what is missing rather than the shape of its
fix, and a hypothetical file there is `path@elsewhere@<path>`.

It is the checked form for a pointer that `design@agent-skills@plain-text-is-no-repair`
would otherwise send to plain text: the escape anchor is refused once the work creates the file.
The escape anchor as the planned form lost on the anchor: it names none, so the deepest-anchor rule goes unchecked until the file
exists and the conversion cannot be done by the kind word alone. `planned` is a reserved kind, and
the manifest refuses a register of that name, as it refuses `path`.

The design this decision did not keep: `rejected@core@planned-path-in-every-document`.

### A trailing slash claims a directory, and its absence claims a file `##trailing-slash-claims-directory`

The kind claim sits in the span itself — greppable, visible to the reader, checkable — rather
than inferred from what happens to exist. The claim is asserted as a fact in both directions, every
required document is asserted to be a file rather than a directory wearing its name. A register's
home is no `path` target, so its two shapes take no claim: the `register` kind names either, per
`design@core@path-to-a-home-is-reported`.

### A target the ignore rules cover is not asserted `##ignored-targets-are-not-asserted`

Resolution asks `git check-ignore` whether the ignore rules cover the target before asking
whether it is present. Covered means accepted with no existence or kind assertion: a generated
path cannot be asserted to exist, per `design@core@git-supplies-the-walk`, and deciding by the
RULES rather than by presence makes the verdict identical on a fresh clone and a built tree. A
verdict that depends on the checking machine's build state is a check nobody can trust twice.
The exemption is exactly as wide as git's own answer, deliberately, nested ignore files included.

**The question is asked with the reference's own kind claim.** A directory claim is spelled with
its trailing slash and a file claim without, because a `dir/` pattern matches a path git can tell
is a directory, and a target that does not exist yet is a directory only if the spelling says so.

### A relative markdown link is a navigation row, and README.md and index.md files are the navigation homes `##links-are-navigation-rows`

A README is directions about what a directory holds and an index is a listing, so a relative
link — the row a reader follows where a renderer shows the page — is legal there and reported
everywhere else; in prose, a pointer is a backticked anchored path. A navigation link resolves
against the linking file's own directory, under the same kind claim and the same refusals as
an anchored path, in the references family; `check::registers` keeps the inverse assertion,
that a directory home's README links every subdocument, for every heading register.

**Every markdown link shape a row is written in is read, and resolved like the plain one.**
The inline form `[text](target)`, with an angle-bracketed target that may hold a space, and
with a title in double quotes, single quotes or parentheses after the target. And the definition `[label]: target`
at the head of a line, which is the target a reference-style link `[text][label]` resolves
through; a label opening with `^` is a footnote, whose definition holds text. An image,
`![alt](target)`, is a link to its target.

**An HTML link, `<a href="…">`, is not read.** Markdown is where a row is written, and the
census of walked markdown finds no HTML link outside this paragraph:
`git grep -n -i --untracked '<a href' -- '*.md'`. One written as a row reopens this clause.

**Markdown documents only.** In Rust prose a markdown link is rustdoc's mechanism, resolved by
rustdoc against the crate namespace, and reading those as index rows would report every
intra-doc link.

**The generated indexes emit links**, relative to their own directory, upward segments
included. Generated text cannot go stale, which is what the upward ban on hand-written links
exists against.

The design this decision did not keep: `rejected@core@single-link-shape`.
