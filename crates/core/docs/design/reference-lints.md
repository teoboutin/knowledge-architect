# Core design: Reference spans and their lints

## Reference spans and their lints

### A backticked `@` span is a reference candidate when its head is a kind or an anchor, and every other `@` span is silent `##at-span-candidate-rule`

The scanner records every backticked span that holds an `@` and no whitespace, backtick or
angle bracket, as written; it has no manifest, so it cannot tell a kind from an email address.
The resolver reads the head — the text before the first `@` — and decides: a known kind is
segmented and resolved; a declared anchor or a reserved anchor in that position is the old form
and is reported with the repair "prefix the kind"; anything else is not a reference and reports
nothing, so an email address or a git remote in backticks is silent unless the project declares
an anchor by that word. Every kind but `path`, `planned` and the harness kinds takes exactly three
non-empty segments; `path` and `planned` take an anchor and then everything after the second `@`;
a harness kind takes the arities of `design@core@harness-kinds-cited-without-anchor`.

**The silence is bounded and named.** A typo inside the kind, `desing@<anchor>@<id>`, is silent under this
rule, because widening it to "any span with two `@`" would report every email address with a
plus tag. `register@core@tripwire` guards the gap: a review finding a reference the scanner
reported nothing for widens the rule to the shape found.

**A fullwidth at sign is not an `@`.** A span written with `＠` is no candidate, and nothing
reports it. Reading lookalike characters as the grammar's would put every script's
punctuation inside the tokenizer, and a census of walked markdown finds no such span outside this
paragraph; re-take it with `git grep -n --untracked '＠' -- '*.md'`, and a span meant as a
reference reopens the silence.

**A span with no `@` is read by three lints, and by nothing else**: a path-shaped span, per
`design@core@every-path-names-its-anchor`, the retired slug reference, per
`design@core@retired-slug-reference-reported`, and a defined id written alone, per
`design@core@backticked-bare-id`.

### A backticked span that opens on one line and closes on the next is a finding where its halves join into a pointer `##a-span-is-one-line`

A backticked span that opens on one line and closes on the next is a finding when its two halves,
joined, would be a reference candidate or a path, and the half before the break holds an `@` or a
`/`: read line by line, neither half is a closed span, so the pointer would be resolved by nothing,
and a renderer shows the line break as a space inside it. A wrapped span that is no pointer, such as
a command or a clause in backticks, is prose and reports nothing. The backtick runs are read as
CommonMark reads them: a span closes on a run of the length that opened it, a backslash escapes a
backtick outside a span, and a blank line ends an open span. **This reading only adds findings.**
Every other pattern reads each line as written, so a line whose backticks are misread can gain a
wrong finding and cannot lose a reference or a path it held.

### The retired slug reference is a finding wherever it names something of this project, and a shape that names nothing here is silent `##retired-slug-reference-reported`

A backticked `<word>#<id>` or a bare `#<id>` names the form it was when its id is an entry some
register defines, a harness kind's names and slugs excluded, since that form never cited one and
a link to a heading in another tool's notation spells its slug so, and a qualified one also when its word is an anchor or a kind. The id is asked
whatever the word, because a form copied out of the history may carry the name an anchor had
before a rename, such as the core's former name `knowledge`. It has no `@` and no two path
segments, so without this clause a slug reference the migration missed would be silent, which is
the founding failure class. The clause does not expire with the migration: the commit history is
read by every session that runs `git log`, it holds the form, and a reference copied out of it
would be checked by nothing.

**A shape that names nothing here is another tool's notation, and is silent**: an issue number
`#123`, a preprocessor directive `#include`, a crate's item `serde#derive`. None was ever this
grammar's, so none is a reference a migration missed, and a project that never used the form would
otherwise have to rewrite its own notation to pass. What the silence costs is a copied form whose
entry has since left the tree and whose word is no anchor: it names nothing here, so it is not
reported. Re-take it as the path lint's census is re-taken, counting this lint's findings; a
census in which the silence hides a pointer the migration missed reopens it. Reporting every such
shape is `rejected@core@retired-slug-reference-always-reported`. This serves `goal@knowledge-architect@any-project-can-adopt-it`.

**The interpretation register's old entry numbers are not read.** They were thaum's own, and a
lint naming them would belong to thaum's extension rather than to the core. Keeping that lint permanently is
`rejected@core@permanent-interpretation-number-form`.

### A reference is live wherever it is prose, fenced blocks included, for every kind `##a-reference-is-live-in-every-prose`

A sketch names what it names on purpose, and an illustration writes a placeholder in angle
brackets, which the tokenizer does not record. Reading a fenced slug reference as an illustration is
`rejected@core@fenced-reference-as-illustration`. A string literal bound to a name in Rust yields no
reference, per `design@core@grammars-not-prefixes`. Checking the literal parts of a placeholder
span is `rejected@core@placeholder-literal-part-check`.

### A reference-shaped token outside backticks is a finding in every prose region the checker reads `##reference-shape-outside-span`

A token outside every backtick span and every quotation, in a document, a Rust comment, an unbound
string literal outside the checker's own source or a commit message, fenced blocks included, is a
reference written without its backticks when it starts with a kind the entity table knows and has
the segments a reference of that kind takes, as `design@core@at-span-candidate-rule` segments a
backticked span, each in its grammar: an anchor name or `*`, and an id of two segments or more, per
`design@core@two-segment-ids`, a register's name for the `register` kind. So an email address, a
URL or a git remote never matches: its head is no kind, or its last segment is no id. The token
starts after a character that is no letter, digit, `@`, `/`, `.` or backtick, and ends at
whitespace, at a `]`, which closes a link's text, or at an apostrophe, which opens a possessive,
less the punctuation that ends a sentence or a clause and the markdown that closes an emphasis or a
quotation around it. A token holding an angle
bracket is a placeholder, and is silent. The finding names its repair, the token in backticks, and,
where the token names nothing of this project, a rewrite of the sentence; `check --fix` writes
nothing for it, as for the other lints on ids, per `design@core@backticked-bare-id`.

**Beside a backticked reference to an issue entry, in the same paragraph, the token is silent.** It
is then the first exception of `design@agent-skills@plain-text-is-no-repair`: text in the checker's
syntax that no checked form expresses, written beside the issue that records the missing form. The
check reads the exception mechanically rather than leaving it to review, so the escape stays
available and nothing else clears the finding. **A quotation is read by none of the lints on ids**,
per `design@core@quotation-read-by-no-id-lint`.

The token is the same defect in any prose, so the check reads every region the scanner reads,
fenced blocks included, as `design@core@a-reference-is-live-in-every-prose` reads a fence. The
nearest rival, the check limited to the `%%` comment lines of the shipped text, which a
retrospective proposed, adds a rule with no reason behind it: a token written without its backticks
in a design home dangles as silently as one in a comment line. The census taken in the design
discussion, kept in its spec, `path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`,
found such tokens in this repository, every one a pointer, and none in thaum; the lint reports a
token a writer stripped of its backticks, which is the evasion
`design@agent-skills@plain-text-is-no-repair` refuses, and serves
`goal@knowledge-architect@documentation-stays-consistent`.

### A harness kind is cited with no anchor `##harness-kinds-cited-without-anchor`

A harness kind takes no anchor: today a skill, an agent, the primer and the root CLAUDE.md. A skill or an agent is cited by its name, two segments, and one of
its sections by its name and the section's slug, three: `skill@<name>`, `skill@<name>@<slug>`,
`agent@<name>`, `agent@<name>@<slug>`. The primer and the root CLAUDE.md are one document each, so
a section of either is cited by its slug alone: `primer@<slug>`, `instructions@<slug>`. Any other
arity, and an empty segment, is malformed, with the kind's forms as its repair. The third segment
narrows within the kind, as a path's last segment does, so the kind keeps the information that a
section belongs to a skill or to an agent.

- **No anchor, because the harness fixes one namespace per kind**: a skill or an agent is a
  directory or a file of the harness's layout, unique by name, and a project has one primer and
  one root CLAUDE.md. An anchor would carry no information, the argument
  `design@core@plans-is-a-reserved-anchor` makes for `plans`. A reference to a harness kind still serves
  `goal@core@relocation-is-one-manifest-edit`, which binds a reference to what the manifest
  places: nothing a harness kind names moves with a manifest edit.
- **The rejected design "A register reference with no anchor"**,
  `rejected@core@register-reference-without-anchor`, lost on two reasons. Its first, that the tool cannot
  know which register is single-instance in a project, does not hold here: the tool fixes the four
  kinds and their layout, and no project declares one. Its second, one grammar with no special case
  in the resolver, is the cost of this form.
- **The kind is read first**, so a word that is both a kind and an anchor would read as the kind;
  no anchor may wear a kind's name, per `design@core@kind-names-refused-as-anchors`.
- **`instructions` names the document's role**, not one harness's file name, so a move of the root
  instructions to another file name changes no reference, per
  `issue@core@configuration-for-several-agent-providers`.

The nearest rival, an anchor the tool constructs for the agent configuration with three segments
everywhere, lost: every name for it stutters or names a provider, and it carries nothing. It is
`rejected@core@agent-configuration-anchor`. Serves `goal@knowledge-architect@any-project-can-adopt-it`:
one grammar, the same in every project, with a global namespace naming no anchor.

The design this decision did not keep: `rejected@core@section-kind`.

### Every level-two heading of a section home of the harness is a section, and owes a slug `##section-homes-carry-slugs`

A section home is a skill's `SKILL.md`, an agent's file, the primer and the root CLAUDE.md, under
the `claude` harness. The rule of `design@core@an-entry-is-a-heading-at-the-register-level` holds
there at level two: every level-two heading outside a fence owes a slug, which defines a section
of that home's skill, agent or document, and a slug anywhere else in it defines nothing, per
`design@core@a-slug-elsewhere-defines-nothing`. An id is
unique within its document. A heading-shaped line inside the frontmatter block, as the harness
reads that block, is no heading.

- **A section slug names a section apart from its position**, where a paragraph number goes stale
  at every insertion. The rule, for a skill's sections, is the owner's proposal: "Better than
  paragraph numbers, which might get stale at any time. Use this in the published skills to refer
  between each other." A slug of the installed text is an interface: a rename dangles a consuming project's
  citations of it, which `design@knowledge-architect@changelog-entries` owes a Migration entry for.
  The owner, on that consequence: "The consequence you stated is accepted and positive in my view.
  This is the fragility I mentionned at the beginning with using paragrap numbers."
- **An installed copy defines its sections and raises no finding.** It is never the project's to
  repair, and under `commits` a commit holding an earlier version's set is judged by a later
  checker. The shipped set is held to the rule where it is written, by a test of the core over the
  shipped text, which also requires every reference of a harness kind in it to resolve within the
  set.
- **A project skill's or agent's slug at a level-two heading defines a section** of that skill or
  agent, whatever anchor holds its file.

The finding on an unslugged heading names the repair, and the setup skill states the requirement,
so an adopting project meets it in the documentation. Serves `goal@core@records-reach-their-reader`:
`show` on a section lists every text that cites it.

### A backticked span that is exactly a defined id is a finding, whose repair names every reference the id can take `##backticked-bare-id`

The scanner records every backticked span that is one word in the grammar of one segment or more,
a span wrapped across a line break at one of its hyphens joined, outside a quotation, per
`design@core@quotation-read-by-no-id-lint`. A lint of the last phase
reports such a span when the entity table defines its id: a register entry's id, a plan document's,
a slice's, a skill's or an agent's name, a section slug of a skill, an agent, the primer or the
root CLAUDE.md, and an item's id inside its own plan, the only place an item may be cited, per
`design@core@plan-item-scope`. Its repair is the entry's reference. A section slug is in scope as a
register's id is: the census of the design discussion, kept in its spec,
`path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`, found 34 plain hits on 10
section slugs in this repository, most of them pointers written in words. Under `harness = []` the table defines no harness
kind, per `design@core@harness-kinds`, so their names and slugs are silent. A commit message is
judged as every lint is, per `design@core@a-commit-message-is-a-document`, so a span naming an
entry the commit deletes is reported against its first parent's tree, and the candidates of both
trees are listed in one finding.

**A bare id is a pointer written with no kind.** Silent, it dangles unseen when the entry is renamed
or deleted. The reference costs the kind and its anchor, and `show` then lists the text among what
cites the entry, per `goal@core@records-reach-their-reader`. The nearest rival, no finding and a
listing of such spans, leaves every such pointer silent: `rejected@core@bare-id-listing-only`.

**The match is exact against the table.** A crate's name, an id of another project and an id whose
entry has left are silent, so the lint reports no span that names nothing here. Every id has two
segments or more, per `design@core@two-segment-ids`, so no ordinary word in backticks is a
defined id; a hyphenated term that is, such as a project skill named like a command, is renamed,
as `design@core@plain-word-lint` says of a subject slug. Whether a skill's name meets such a term
rests on the names: the installed ones carry the installer's prefix, and a project's own carry its
prefix, per `design@agent-skills@project-name-prefix`, which nothing checks yet, per
`issue@core@tooling-for-project-skills`. The census of the design discussion found few spans that
were no pointer, in thaum thread names backticked in a plan table against the design skill's own
rule. Re-take the false positives with `cargo klarch check` over a tree that has not
rewritten its bare ids, such as another project's at its upgrade, counting this lint's findings: a
finding on a span that is not meant as a pointer reopens the exact match.

**An id that several entries define gets one finding, which lists every reference it can take and
chooses none.** A skill and an agent of one name, and an agent's section slug shared by three agents,
are among the cases the census found, 18 such ids here and 41 in thaum, in the spec named above. Choosing the nearest definition would make the checker supply a meaning the writer did not
write, so the writer chooses. The same holds for the plain-word lint.

**The lints on ids report, and `check --fix` writes nothing for them**: no respelling of a
hand-written file is among the fixes `design@core@fix-scope` admits, per
`design@core@safe-fix-definition`. Respelling a span into its reference is the subject of
`spec@plans@path-quickfixes`. The rule holds for the three lints on ids.

**In a released section of CHANGELOG.md the id is written as its reference too**, a structural
change, and a skill's or an agent's name as the bare name again once it is deleted, per
`design@knowledge-architect@changelog-section-fixed-at-release`. That bare name names nothing in
the table, so the lint leaves it silent.

### A plain word that is exactly a defined id is a finding `##plain-word-lint`

A word outside every backtick span and every quotation, per
`design@core@quotation-read-by-no-id-lint`, in every prose region the checker reads, that is exactly
a defined id, as `design@core@backticked-bare-id` defines one, is a finding. A word
is bounded as an id is: no letter, digit, hyphen, `/`, `@`, `#` or brace on either side, and no `.`
followed by a letter or a digit after it, so a file name, a path, a reference, a slug with its
hashes and a template's placeholder holding an id are no word, and an id that ends a sentence is
one. A URL and an HTML comment inside a line hold no word. The value of a frontmatter `name` line is
the name a skill or an agent is defined by, not a mention, and is not read. Its repair names the
reference, and adds what to do where the word is meant as an ordinary term: a design entry's slug
then names a subject, and the entry is renamed to state its decision, as `skill@knowledge-architect-decision-recording@head-shape` asks
of a slug; any other entry is renamed so that its slug is not an ordinary word, since, on the
owner's word, a slug has no reason to be used as an ordinary word, and that is the only way the
lint reports a wrong use reliably. The owner's ruling covers the slugs of the installed text too,
which a consuming project cannot rename: such a slug met there as an ordinary word is reported
upstream, by the project's retrospective.

**The rule is the owner's**, given against the session's proposal of a listing in `show`: a word
meant as an ordinary term probably shows a slug wrongly chosen and not specific enough, in the
owner's view. The census of the design discussion, kept in its spec,
`path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`, bore it out in thaum, where it
sorted the hits: every plain hit that was no pointer came from a subject slug, and every other hit
was a pointer written in words. A listing would leave those pointers unchecked: `rejected@core@plain-word-mention`. A slug kept as a concept index, so
that a term's mentions read as its pointers, is not the intended use of the workflow, in the
owner's view, and `show` lists every citation of an entry.

**A released section of CHANGELOG.md is linted** as any other prose; how a finding there is ruled
is `design@knowledge-architect@changelog-section-fixed-at-release`.

**The lints cost little time.** Re-take it with `klarch check` over this repository, release
build, median of five runs on one machine, against the same commit built with the lints' arms and
the scanner's reading for them removed: a run that takes more than 10% longer than that one reopens
the scan.

### A quotation is read by none of the lints on ids `##quotation-read-by-no-id-lint`

A quotation is text between `"` and `"`, or `“` and `”`, within one paragraph, or a blockquote
line. None of the three lints on ids, `design@core@reference-shape-outside-span`,
`design@core@backticked-bare-id` and `design@core@plain-word-lint`, reads one. A quotation is
someone else's words, such as the owner's ruling on a thread named by its slug, or the rules text of
a specification, which its writer may not respell, so a finding there would have no repair the
writer may make. A quotation left open ends at its paragraph, which ends at a blank line, a heading,
a fence's edge, a list item or a table row, the blocks a renderer shows apart, so it cannot silence
the rest of a file. A pointer someone writes inside a quotation goes unchecked, which is what the
exemption costs. The nearest rival, no exemption, would report words the writer may not change. It
is the fourth exception of `design@agent-skills@plain-text-is-no-repair`.
