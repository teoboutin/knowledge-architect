# Model design: Paths and homes

## Paths and homes

### `elsewhere` and `*` are reserved anchor words, each naming something only under the kinds it serves: the escape under `path`, and every component's own copy under `path` and `register` `##reserved-anchors`

The word elsewhere in anchor position is the escape for a path this tree does not hold, per
`design@model@escape-anchor-elsewhere`, and serves `path` alone. The word `*` names every
component's own copy, per `design@model@generic-anchor-names-every-component-copy`, and serves
`path` and `register`.

**Under any other kind the escape names nothing, and `*` names nothing but under `register`.** A
`design@*@<id>` is reported as an unknown anchor, and the finding says which kinds each word
serves. The `planned` kind refuses them
with a finding of its own, per `design@model@planned-path-form`. The manifest's resolution
refuses a declared anchor wearing either word, and it is no anchor: every pointer at it
would read as the reserved meaning.

The design this decision did not keep: `rejected@model@at-prefix-escape`.

### A path this tree does not hold is written at the escape anchor, `path@elsewhere@<path>`, a registered pointer exempt from the existence, kind and shape assertions and refused where its target resolves here `##escape-anchor-elsewhere`

**The escape anchor** — the word elsewhere in anchor position, `path@elsewhere@<path>` — marks a
path deliberately not resolvable here: a surveyed engine's layout, a deleted file a tracker
entry discusses, a hypothetical location. It is scanned and counted like any reference, so a
typo'd escape is still a registered pointer, which `rejected@model@at-prefix-escape` lacks; it is
exempt from the existence, kind and shape
assertions, because a foreign layout may spell anything. One assertion runs against it: a
target that DOES resolve here, read from every place the unanchored finding reads a path from, the
root, an anchor's directory or the document's own, is a finding — without it, the escape is the
cheap way to silence the unanchored finding on a real path. The finding names the form at the
deepest anchor that holds the target, per `design@check@deepest-anchor-wins`.

### A file of another project is written at the escape anchor with that project's name as its first segment, `path@elsewhere@<project>/<path>` `##foreign-file-names-its-project`

**A file of another project is written with that project's name as its first segment**, as
`path@elsewhere@<project>/<path>`, and the refusal's repair names that form. A foreign file often
shares its relative path with a file here, such as a dependency's src/lib.rs, and without the
prefix the assertion of `design@model@escape-anchor-elsewhere` refuses it. A repair that names only the local anchor leaves the writer
of a foreign pointer no checked form, and prose follows. The prefix is no cheap silence: it is false of
every file in this tree, so writing it before a local path is a deliberate misstatement, which costs
what removing the backticks costs, and that move is refused as a repair by
`design@agent-skills@plain-text-is-no-repair`.

### The generic anchor `*` names every component's own copy, of a path under `path` and of a register's home under `register` `##generic-anchor-names-every-component-copy`

**The generic anchor `*`**, `path@*@<path>`, marks each component's own copy of a path, as in
`path@*@README.md`. It is accepted when the path is a required document name
— a compiled document, or a file register's README or generated index — and otherwise when at
least one component carries the path with the claimed kind. The file-register set is derived from
the component-scoped registers rather than written out, so a project that declares one gets its
names in the set. A register's home is no `path` target, per
`design@model@path-to-a-home-is-reported`: `register@*@<register>` names every component's own home
of a component-scoped register, in whichever shape each holds, per
`design@model@register-home-reference`. A generic reference nothing resolves rots exactly like a
dangling one. The anchors the tool constructs hold no copy the generic form names: what they hold
is cited by its kind.

The design this decision did not keep: `rejected@model@component-placeholder-anchor`.

### A register's home is cited by its own kind, `register@<anchor>@<register>`, but for a register the root alone carries, and the reference names the home in whichever shape it has `##register-home-reference`

`register@<anchor>@<register>` names the home that the anchor keeps the register in. The check
resolves it against the anchors, which the table's module holds, and asks nothing of the tree,
since a missing home is the tree check's finding; `show` reads the tree's listing to print the
home it holds. No register's name becomes an entity id, so a register's name in backticks alone
stays silent under `design@check@backticked-bare-id`.

- **The anchor** is a component, a declared location or the anchor `plans`, where it carries the
  register; `*` names every component's own home of a component-scoped register, per
  `design@model@generic-anchor-names-every-component-copy`. The home of a register the root alone carries names no anchor,
  `register@<register>`, as `register@goal`, per `design@model@root-register-reference-has-no-anchor`. An entry or a spec anchor is no anchor of the form: an entry's
  homes are its own directory, which the entry's reference already names, as
  `milestone@plans@<id>` names a milestone's, and one document has one name, per
  `design@model@plan-document-kinds` and `design@model@declarable-directory-register`.
- **The id** is a register's name, built in or declared. An item register is no id of the form:
  its home is a plan's own documents, cited by the plan document's kind.
- **Each refusal names the form that resolves**, per `design@check@finding-names-the-repair`.
- **`show`** prints the home the tree holds, the file or the directory, with the register's shape,
  then every site that cites it, a citation of a component's home through `*` included.

**The kind exists because a home has two shapes**, per `design@check@heading-register-two-shapes`.
A `path` names one of them, so a home that outgrows one file dangles every `path` to it, each then
respelled by hand, and an installed text, which
`design@agent-skills@installed-text-works-with-what-an-installing-project-holds` has followed with what each
installing project holds, would name both shapes. This reference names the register and lets the manifest and the tree say which shape it is.
The nearest rival is a `path` that resolves to either shape; it loses on
`design@model@trailing-slash-claims-directory`, since a path claims its target's kind, and a path
that matches either shape asserts no claim and puts a special case into path resolution.

**The kind's word is the owner's choice**: the owner chose `register` over `home`, which the owner found to
read oddly. The rival word, `home`, matched the documents' phrase "design home" better and cost the same refusal of
an anchor name. A kind's name is refused for every anchor, per
`design@model@kind-names-refused-as-anchors`, and for a declared register, per
`design@model@registers-are-declared`. The cost the form keeps: a reader outside the tool does not
see the file a reference names, which `issue@knowledge-architect@the-documents-do-not-render-as-a-site` would turn
into a link.

The designs this decision did not keep: `rejected@model@home-kind-word` and
`rejected@model@path-to-either-home-shape`.

### A `path` reference whose target is a register's home is a finding, whose repair names the `register` form `##path-to-a-home-is-reported`

A home target is a heading register's file, its directory, or that directory's `README.md`, which
is the home's head per `design@check@heading-register-two-shapes`; a file register's directory; a
Directory register's directory. A file register's `README.md` and generated `index.md`, a
subdocument and an entry are files inside a home, and stay `path` targets. **An entry's directory
and its `README.md` are no home target**: they are the entry, and a `path` to either is a finding
whose repair names the entry's reference, `<register>@<parent>@<id>`, per
`design@model@declarable-directory-register`; a milestone's, by `design@model@plan-document-kinds`.

- The finding is raised for a named anchor after the refusals of a refused shape and of a plan
  document cited by its path, on the target resolved to the anchor that keeps the home, and in
  place of the deeper-anchor finding: its repair names `register@<that anchor>@<register>`, or
  `register@<register>` for the home of a register the root alone carries, per
  `design@model@root-register-reference-has-no-anchor`. Under
  `*`, it is raised before the required set and any kind claim are judged, and names
  `register@*@<register>`.
- The kind claim and the existence of the target are not judged: the manifest decides whether a
  path is a home, and either shape names the same register.
- A line number or a fragment is stripped before the test, and the repair says to drop it.
- A `planned` reference is not judged by it.

**Both forms legal** loses to this finding: a writer keeps writing the `path`, and a home that
changes shape then dangles every one of them, which `design@model@register-home-reference` exists to
prevent. The finding repairs nothing by itself:
no fix of `check --fix` respells it.

The design this decision did not keep: `rejected@model@path-to-home-beside-register-form`.

### The word `plans` names the anchor the tool constructs at the plans directory, and no project may declare it `##plans-is-a-reserved-anchor`

The anchor is constructed per `design@model@plan-register`. The word is reserved like the two words
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
not asserted, per `design@model@ignored-targets-are-not-asserted`. `*` and the escape anchor are
refused: a planned path is in this tree, at one anchor.

**It is legal in the plans directory alone**, since a plan document is the one place that
describes unbuilt work, per `design@model@plan-register`. Elsewhere it is a finding that asks for
the plan that creates the file. An issue entry states what is missing rather than the shape of its
fix, and a hypothetical file there is `path@elsewhere@<path>`.

It is the checked form for a pointer that `design@agent-skills@plain-text-is-no-repair`
would otherwise send to plain text: the escape anchor is refused once the work creates the file.
The escape anchor as the planned form lost on the anchor: it names none, so the deepest-anchor rule goes unchecked until the file
exists and the conversion cannot be done by the kind word alone. `planned` is a reserved kind, and
the manifest refuses a register of that name, as it refuses `path`.

The design this decision did not keep: `rejected@model@planned-path-in-every-document`.

### A trailing slash claims a directory, and its absence claims a file `##trailing-slash-claims-directory`

The kind claim sits in the span itself — greppable, visible to the reader, checkable — rather
than inferred from what happens to exist. The claim is asserted as a fact in both directions, every
required document is asserted to be a file rather than a directory wearing its name. A register's
home is no `path` target, so its two shapes take no claim: the `register` kind names either, per
`design@model@path-to-a-home-is-reported`.

### A target the ignore rules cover is not asserted `##ignored-targets-are-not-asserted`

Resolution asks `git check-ignore` whether the ignore rules cover the target before asking
whether it is present. Covered means accepted with no existence or kind assertion: a generated
path cannot be asserted to exist, per `design@model@git-supplies-the-walk`, and deciding by the
RULES rather than by presence makes the verdict identical on a fresh clone and a built tree. A
verdict that depends on the checking machine's build state is a check nobody can trust twice.
The exemption is exactly as wide as git's own answer, deliberately, nested ignore files included.

**The question is asked with the reference's own kind claim.** A directory claim is spelled with
its trailing slash and a file claim without, because a `dir/` pattern matches a path git can tell
is a directory, and a target that does not exist yet is a directory only if the spelling says so.
