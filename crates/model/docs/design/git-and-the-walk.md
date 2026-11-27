# Model design: The walk

## The walk

### Git supplies the walk, and the manifest declares only what git tracks `##git-supplies-the-walk`

The walked set is git's listing from the manifest's directory, `git ls-files --cached --others
--exclude-standard`, with `[walk] skip-dirs`, `skip-files` and `exclude` applied after it, and the
generated files, the installed agent files and the frozen files of a harvested slice removed by
construction, the last per `design@model@a-harvested-slice-is-frozen`. No generated path is
declared by a `[walk]` row: `target` and `path@agent-config@worktrees/` are covered by the ignore
rules, so no row names them. A path
reference whose target the ignore rules cover is exempt from assertion the same way, per
`design@model@ignored-targets-are-not-asserted`.

**A generated index is outside the walk by construction, so its rows are read by nothing.** The
tool derives the set from the register instances rather than from a declared row, which is why no
`[walk] skip-files` row names a file-register index and a new instance cannot arrive with its
index inside the walk; an extension's generated files, such as thaum's rule index, are in the same
set, so no row names them either, and so is the assembled changelog, where two changelog crates or
more are read, per `design@model@one-list-of-generated-files`. Outside the walk also means outside the files the survey hands
over as unwalked: a listing is a function of the tree rather than a claim anybody wrote, so no
check reads its rows as one.

**Every remaining declaration is checkable.** A generated path cannot be asserted to exist: a
fresh clone may have none of them. A manifest naming one as a row holds a row no check can ask
about: nobody is told the row is dead, and a file later created at that path inherits what the row
granted. One generated path is declared, the assembled changelog's, `[changelog] assembled`, per
`design@model@assembled-changelog-location`, and no file at it escapes a check: the
generated-files check holds the file to the bytes it generates and reports it missing or out of
date, as it reports an index, so `tree` does not assert that it exists. Every other path in the manifest is one git tracks, and `tree` asserts each one
exists. **A declared path is spelled the way git lists
it**, with no `.` segment, no `..` segment and no leading `/`: a `.` segment is dropped when
the manifest loads, so two spellings of one directory are one declaration, and a row holding
`..` or an absolute path is refused as a finding and acted on by nothing, because what it
names depends on where the manifest sits rather than on the tree.

**No ignore rule can take a tracked file out of the walk.** `--cached` is unaffected by the ignore
rules, so no ignore line, however written, can take a live document out of every check. What
leaves the walk does so by the manifest or by the layout the tool fixes: a declared row, an
exclusion derived from a declaration, as the generated indexes and the installed agent files are,
per `design@check@installed-files-outside-the-walk`, or the frozen location of the plans layout, as
the files of a harvested slice are. That is what a
hand-rolled matcher cannot promise, and it is the property the whole walk is
chosen for; the matcher and the three things it could not do are
`rejected@model@hand-rolled-gitignore-matcher`.

A tracked path the working tree does not hold, a deletion nobody has staged, stays
in the walk and is reported, because dropping it would take a live document out of every check on
the strength of a working-tree state.

**The summary block prints the walked-file count**, so two machines disagreeing about the walk is
visible in the output rather than inferred from a finding list. The tripwire is in
`register@model@tripwire`.

**This is the working tree's walk, the one plain `check` reads.** `check --staged` and `commits`
read a snapshot instead, from git objects alone, per `design@check@staged-tree-source` and
`design@check@a-commit-message-is-a-document`.

### A glob of `[walk.citation-free]` at the level `all` is matched against the walked files, and one at the level `entries` against those and the installed copies `##citation-free-globs`

The table holds two lists of globs, `all` and `entries`; what a file at each level may hold, and
the findings on what it holds, are `design@check@citation-free-paths`, the rules a glob is
written in are `design@model@citation-free-glob-dialect`, and the spellings a glob refuses are
`design@model@citation-free-glob-spelling`. A glob of `all` is matched against the
files the walk reads, and against no installed copy: the shipped text cites the shipped skills,
agents and primer on purpose, per `design@agent-skills@shipped-text-cites-no-entry`, so a copy at
that level would hold findings that no edit of the project's own text removes. A glob of `entries`
is matched against the walked files and against the installed markdown copies git lists, which the
walk leaves out, per `design@check@installed-files-outside-the-walk`, since the level `entries`
reads the installed copies, per `design@check@installed-copies-read-at-the-level-entries`, and not
their source, `rejected@check@citation-free-entries-over-the-source`. A generated file is in
neither set, so no glob matches it: its findings are its sources'. The resolved table is
`Manifest::citation_free`, and the library's `declared` module names `CitationFree`, `Glob` and
`Level`.

### A glob of `[walk.citation-free]` that matches nothing a level reads is a phase-2 finding at the manifest `##citation-free-glob-matches-nothing`

**A glob is checkable, as every other declaration of the walk is**, per
`design@model@git-supplies-the-walk`. A glob of `all` that matches no walked file, and one of
`entries` that matches no walked file and no installed copy, is a phase-2 finding at the manifest:
it declares nothing, and silently covers whatever file is next created at a matching path, the case
of a `skip-files` row that names no file. A glob of `entries` that matches only installed files that
are not markdown, the saved workflows, is a phase-2 finding of its own, which says that no level
reads them: it matches files, so the finding of a glob that matches nothing would be false of it.
Each finding is phase 2's, since `Manifest::parse` reads no file, and the tree's phase alone holds
the walked paths and the installed copies.

A copy is matched whether or not the checker can read it, since a glob that names a copy is no glob
that matches nothing: a finding at the glob would send the writer to delete a correct glob. Under
`check` and `check --staged`, the comparison of its bytes,
`design@check@installed-files-compared-by-bytes`, reports a copy the checker cannot read, with the
repair that reaches it; `commits` compares no installed file, per
`design@check@only-check-compares-installed-files`, so under it such a copy is reported by nothing.
The rival, a glob that matches nothing passing in silence, is the dead row that
`design@model@git-supplies-the-walk` refuses for every declared path.

### A walked file that a glob of each level of `[walk.citation-free]` matches is a phase-2 finding at the file `##citation-free-levels-overlap`

The two levels admit different spans, so a declaration that puts one file under both says two
things of it, and no precedence between them is what the manifest declared: the rival, `all`
taking precedence in silence, would leave the declaration's contradiction unreported. The finding
is the tree's phase's, since `Manifest::parse` reads no file and the tree's phase alone holds the
walked paths. `CitationFree::level` answers `all` for such a file, the stricter level: there every
span that parses as a reference is a finding, per `design@check@citation-free-paths`, so a caller
that reads the file past the phase-2 finding reports every span that either level reports.

### A glob of `[walk.citation-free]` is written in a dialect of three rules, and in no glob library's dialect `##citation-free-glob-dialect`

`*` matches any run of characters within one path segment, never a `/`; a segment that is exactly
`**` matches any number of whole segments, none included; and a glob ending with `/` names every
file under each directory it matches. Every other character is itself, `?` and `[` included, and a
`**` inside a longer segment is two stars within that segment. The matcher is `Glob` in
`path@model@src/citation_free.rs`, one pass that returns only to the last star met, so its cost is
polynomial in the lengths of the glob and the path.

The nearest rival is a glob library's dialect. It brings a dialect of its own, with character
classes, alternatives and escapes, and each of its spellings would be one a manifest may write and
a later version must keep accepting, by the manifest test of
`design@knowledge-architect@versioning-policy`. A dialect of three rules, written here, holds no
spelling the project did not choose.

### A glob of `[walk.citation-free]` holding a leading `/`, a `.`, a `..` or an empty segment is refused, where a `[walk]` path's `.` and empty segments are dropped `##citation-free-glob-spelling`

A glob is compared with git's listing, which spells a path with no leading `/`, no `.` or `..`
segment and no empty segment, so a glob spelled otherwise matches nothing git lists. Such a glob is
a complaint of the manifest's resolve, and declares nothing, written by `Glob::parse` in
`path@model@src/citation_free.rs`.

A declared `[walk]` path is refused for a leading `/` or a `..` segment alike, per
`design@model@git-supplies-the-walk`, the rule this one departs from for the other two. The
nearest rival is the drop a `[walk]` path takes when the manifest loads: its `.` segment, per that
head, and its empty segment with it, since `normalise_one` in `path@model@src/manifest.rs` reads
the path by its components. A `[walk]` path names one path, which the drop leaves the same, while a
`*` beside an empty segment, as in `a/*//b`, leaves open which segment the star was meant to
cover, so the writer, not the loader, spells it again. That argument is the empty segment's. A `.`
segment is refused with it: one rule for every segment a glob must not spell, over a respelling the
drop would spare, which no goal ranks: the session's ranking.

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
