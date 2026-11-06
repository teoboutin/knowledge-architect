# Model design: The walk

## The walk

### Git supplies the walk, and the manifest declares only what git tracks `##git-supplies-the-walk`

The walked set is git's listing from the manifest's directory, `git ls-files --cached --others
--exclude-standard`, with `[walk] skip-dirs`, `skip-files` and `exclude` applied after it, and the
generated indexes and the installed agent files removed by construction. Nothing generated is declared: `target` and
`path@agent-config@worktrees/` are covered by the ignore rules, so no `[walk]` row names them. A path
reference whose target the ignore rules cover is exempt from assertion the same way, per
`design@model@ignored-targets-are-not-asserted`.

**A generated index is outside the walk by construction, so its rows are read by nothing.** The
tool derives the set from the register instances rather than from a declared row, which is why no
`[walk] skip-files` row names a file-register index and a new instance cannot arrive with its
index inside the walk; an extension's generated files, such as thaum's rule index, are in the same
set, so no row names them either. Outside the walk also means outside the files the survey hands
over as unwalked: a listing is a function of the tree rather than a claim anybody wrote, so no
check reads its rows as one.

**Every remaining declaration is checkable.** A generated path cannot be asserted to exist — a
fresh clone has none of them. A manifest naming one holds a row no check can ask about: nobody
is told the row is dead, and a file later created at that path inherits what the row granted. No
generated path is declared, so every path in the manifest is one git tracks and
`tree` asserts each one exists. **A declared path is spelled the way git lists
it**, with no `.` segment, no `..` segment and no leading `/`: a `.` segment is dropped when
the manifest loads, so two spellings of one directory are one declaration, and a row holding
`..` or an absolute path is refused as a finding and acted on by nothing, because what it
names depends on where the manifest sits rather than on the tree.

**No ignore rule can take a tracked file out of the walk.** `--cached` is unaffected by the ignore
rules, so no ignore line, however written, can take a live document out of every check. What
leaves the walk does so by the manifest: a declared row, or an exclusion derived from a
declaration, as the generated indexes and the installed agent files are, per `design@check@installed-files-outside-the-walk`. That is what a
hand-rolled matcher cannot promise, and it is the property the whole walk is
chosen for; the matcher and the three things it could not do are
`rejected@model@hand-rolled-gitignore-matcher`.

A tracked path the working tree does not hold — a deletion nobody has staged — stays
in the walk and is reported, because dropping it would take a live document out of every check on
the strength of a working-tree state.

**The summary block prints the walked-file count**, so two machines disagreeing about the walk is
visible in the output rather than inferred from a finding list. The tripwire is in
`register@model@tripwire`.

**This is the working tree's walk, the one plain `check` reads.** `check --staged` and `commits`
read a snapshot instead, from git objects alone, per `design@check@staged-tree-source` and
`design@check@a-commit-message-is-a-document`.

### A symlink and a gitlink are read as no document, and each is a phase-2 finding naming it `##links-are-no-documents`

Git records a symlink as mode `120000` and a submodule's gitlink as `160000`, in the index and in
every commit's tree alike, so the listing classifies an entry by mode before either reader touches
a byte and `check` and `commits` agree by construction. Read through the filesystem a symlink
yields its target's bytes, read from a tree it yields the target's path as text, and a checkout on
a platform without symlinks holds that text as a plain file, so two readers of one tree would
judge two documents; a gitlink names a commit of another repository the listing never descends
into, so everything under it would be conformant by vacuum. A `skip-files` row keeps a symlink
and an `exclude` row a submodule, as declared silences. What a submodule is to the project that
holds it is an open question, `issue@check@a-submodule-is-a-project-of-its-own`.

### A name holding a line break, or one Windows cannot create, is refused with a finding naming the file and the reason `##unportable-names-refused`

Two grounds, and the second covers more names than the first:

- A finding is one line opening with its path, an index row is one line, and a reference is one
  backticked span, so a file whose name holds a newline or a carriage return can be printed by
  nothing here and pointed at by nothing.
- Windows refuses to create a file whose path has a component holding one of `<>:"|?*\` or a
  control character below the space, ending in a space or a period, or naming a device: `CON`,
  `PRN`, `AUX`, `NUL`, `COM0` to `COM9` and `LPT0` to `LPT9`, with the superscript digits `¹²³`,
  in any case and whatever follows the first period. A tree holding one cannot be checked out
  there, and a project learns it from this gate rather than from a failed checkout.

The walk drops the file, no check reads it, and `tree` reports it
once with any line break escaped — `Finding` escapes a line break in the path it is located at
and in the statement it makes, so a reader taking the output by line meets one finding per line
whatever route a name took to reach it. A project that means to keep such a file names it in `skip-files` or in an ignore rule.

### A tracked file the ignore rules also cover is a phase-2 finding naming the file `##tracked-and-ignored-is-a-finding`

The two states contradict each other and the contradiction is otherwise silent: the walk reads the
file, and `git check-ignore` skips what the index holds so a reference to it is asserted too — the
reverse of what the ignore rule says. It is reported in phase 2, by `tree`, with the other findings
that judge what the tree holds against what the project declares. The run stops there, as for
every phase-2 finding, per `design@check@phases-gate-the-report`: the model holds a file the
project says is out of it, so a finding computed from that model is unreliable.
