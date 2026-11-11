# Check design: Paths, homes and links

## Paths, homes and links

### Every path reference names its kind and its anchor, `` `path@<anchor>@<path>` `` `##every-path-names-its-anchor`

A checked path reference is one backticked span in the one grammar of
`design@model@an-entity-belongs-to-its-anchor`, with `path` as its kind and the path under the named
anchor's directory as its id. The root is a component like any other, named by `project.name`.
The kind prefix costs five characters at every path reference and buys one grammar: the scanner
has one tokenizer, the old two-segment form `<anchor>@<path>` is reported as an anchor in kind
position by `design@model@at-span-candidate-rule`, and the unanchored lint is left with
one job, per `design@check@unanchored-path-first-segment-here`. There is no unanchored form for a
path of this tree.

**The path is plain**: `..`, a `.` segment and a leading `/` are refused. An upward path is
anchored at the wrong place by definition, and it is the shape that breaks when the referencing
file moves. The id is everything after the second `@`, so a path may itself hold an `@`. Which
anchor a path names is `design@check@deepest-anchor-wins`.

**A plan document is cited by its kind, never by its path**, per
`design@model@plan-document-kinds`. A
`path` citation of a spec, of a milestone directory or of a file inside one is refused from every
anchor, and the finding names the form that resolves: `spec@plans@<id>`, `milestone@plans@<id>`
or `spec@<milestone>@<slice>`. It is judged by where the target sits, before the deepest-anchor
rule and whether or not it exists, so a citation of a deleted plan gets the same repair. This
overrides the ancestor's spelling of an anchor's own directory for a milestone. A plan anchor, a
milestone or a spec, carries no `path` kind. One name per document is what keeps `show` complete: a citation written as a path
is one `show spec@…` would not list. The README of the plans directory, and the README and index
of each of its two homes, are not plan documents, and are cited `path@plans@<file>`.

**A fenced path reference is live**, as every reference is, per
`design@model@a-reference-is-live-in-every-prose`: a sketch names its paths on purpose, and an
illustration that needs a fake path writes the escape anchor or an angle-bracket placeholder.

The designs this decision did not keep: `rejected@check@bare-root-relative-path`,
`rejected@check@path-without-kind-prefix`, `rejected@check@at-prefix-root-relative-path` and
`rejected@check@retired-form-rs-suffix`.

### A backticked path-shaped span with no `@` is a finding only when its first segment names something this tree's listing holds `##unanchored-path-first-segment-here`

A backticked span of path characters with two or more segments and no `@` is a finding naming the
grammar of `design@check@every-path-names-its-anchor` when its first segment names a
file or a directory this tree's listing holds. One segment is a name rather
than a path, though the exact name of a skill or an agent is reported, per
`design@check@backticked-bare-id`, and a span holding a space, an angle bracket, or a colon anywhere but in a line
suffix is not path-shaped, which is what lets documentation of the syntax show a placeholder with no
carve-out.

**A span whose first segment names nothing here is another tool's notation, and is silent**: a
media type `application/json`, a unit `km/h`, a git ref `origin/main`, the fixture path of a test,
a path on another machine. The first segment is read from the root, from every anchor that is a
directory, and from the document's own directory, a `..` climbing from each; a leading slash is
read from the root alone. A spec is an anchor and a file, so it is no place a path is read from.
Only the first segment is asked, so a pointer whose first segment is here and whose rest is not is
still a finding, and the anchored form it is rewritten in reports it dangling. A commit message is
judged against two trees, per `design@check@a-commit-message-is-a-document`.

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
it is in the message of the commit "Report an unanchored path or a retired slug form only where it
names something of this project". Reporting every path-shaped span
is `rejected@check@every-path-shaped-span-reported`. This serves `goal@every-target-project-can-adopt-it`
and `goal@adoption-is-easy`.

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

The design this decision did not keep: `rejected@check@ignored-first-segment-reported`.

### A path reference names the deepest anchor that holds its target, inside meaning a proper descendant `##deepest-anchor-wins`

**The deepest anchor wins, and inside means a proper descendant.** A reference whose target
sits inside another anchor is refused with the right anchor named. What this buys is the
same property `design@model@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
anchor edits its one line of the project's `knowledge-architect.toml` and no document, and one
fixed-string grep per anchor is an
anchor's complete inbound-reference list. An anchor's own directory is the one target with no
spelling under its own name, so it is named from an ancestor — a reference that names a
location, which a move is expected to break. A plan document is the exception: the rule on a
`path` to a spec or a milestone is judged before this one, and overrides the ancestor's spelling of
a milestone's own directory, per `design@check@every-path-names-its-anchor`.

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

The design this decision did not keep: `rejected@check@single-link-shape`.

That the backticked reference is the primary form of a pointer, and the link a second form for
navigation alone, is a design decision and no goal's: the owner's argument for one primary form is
that it is easier to work with.
