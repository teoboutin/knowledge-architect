# Check design: Paths, homes and links

## Paths, homes and links

### A backticked path-shaped span with no `@` is a finding only when its first segment names something this tree's listing holds `##unanchored-path-first-segment-here`

A backticked span with no `@` that is path-shaped, per `design@model@path-shaped-span`, is a
finding naming the grammar of `design@model@every-path-names-its-anchor` when its first segment
names a file or a directory this tree's listing holds.

**A span whose first segment names nothing here is another tool's notation, and is silent**: a
media type `application/json`, a unit `km/h`, a git ref `origin/main`, the fixture path of a test,
a path on another machine. The first segment is read from the root, from every anchor that is a
directory, and from the document's own directory, a `..` climbing from each; a leading slash is
read from the root alone. **A plan document's directory, a spec's or a milestone's, and every
directory under it, is no place a first segment is read from**, whether the document is inside
the plan or outside it: a plan anchor carries no `path` kind, so the finding would name a repair the
resolver refuses; the names of the plan's layout, its deliberation file, its search directory or
its harvested directory, recur in every text that describes the layout; and a plan's `README.md`
reaches the files beside it by a relative link, which the link check judges.
Only the first segment is asked, so a pointer whose first segment is here and whose rest is not is
still a finding, and the anchored form it is rewritten in reports it dangling. A commit message is
judged against two trees, per `design@check@message-references-resolve-in-either-tree`.

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

The repair of a span that carries a line suffix or a fragment names the file and drops the suffix:
a reference is to a file, and a line number goes stale at the next edit.

The design this decision did not keep: `rejected@check@ignored-first-segment-reported`.

### A path reference names the deepest anchor that holds its target, inside meaning a proper descendant `##deepest-anchor-wins`

**The deepest anchor wins, and inside means a proper descendant.** A reference whose target
sits inside another anchor is refused with the right anchor named. What this buys is the
same property `design@model@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
anchor edits its one line of the project's `knowledge-architect.toml` and no document, and one
fixed-string grep per anchor is an
anchor's complete inbound-reference list. An anchor's own directory is the one target with no
spelling under its own name, so it is named from an ancestor — a reference that names a
location, which a move is expected to break. A file under a plan's directory, a slice's
included, has that plan as its deepest anchor, since a slice's directory is no anchor, per
`design@model@anchors-are-components-and-locations`. A plan document is the exception to the
spelling: the rule on a `path` to a spec or a milestone is judged before this one, and overrides
the ancestor's spelling of a spec's or a milestone's own directory, per
`design@model@plan-document-path-citation-refused`.

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
