# Check design: Reference spans and their lints

## Reference spans and their lints

### The retired slug reference is a finding wherever it names something of this project, and a shape that names nothing here is silent `##retired-slug-reference-reported`

A backticked `<word>#<id>` or a bare `#<id>` names the form it was when its id is an entry some
register defines, a harness kind's names and slugs excluded, since that form never cited one and
a link to a heading in another tool's notation spells its slug so, and a qualified one also when its word is an anchor or a kind. The id is asked
whatever the word, because a form copied out of the history may carry the name an anchor had
before a rename, such as `check`'s former name `knowledge`. It has no `@` and no two path
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
shape is `rejected@check@retired-slug-reference-always-reported`. This serves `goal@knowledge-architect@every-target-project-can-adopt-it`.

**The interpretation register's old entry numbers are not read.** They were thaum's own, and a
lint naming them would belong to thaum's extension rather than to `check`. Keeping that lint permanently is
`rejected@check@permanent-interpretation-number-form`.

### A reference-shaped token outside backticks is a finding in every prose region the checker reads `##reference-shape-outside-span`

A token outside every backtick span and every quotation, in a document, a Rust comment, an unbound
string literal outside the checker's own source or a commit message, fenced blocks included, is a
reference written without its backticks when it starts with a kind the entity table knows and has
the segments a reference of that kind takes, as `design@model@at-span-candidate-rule` segments a
backticked span, each in its grammar: an anchor name or `*`, and an id of two segments or more, per
`design@model@two-segment-ids`, a register's name for the `register` kind. So an email address, a
URL or a git remote never matches: its head is no kind, or its last segment is no id. The token
starts after a character that is no letter, digit, `@`, `/`, `.` or backtick, and ends at
whitespace, at a `]`, which closes a link's text, or at an apostrophe, which opens a possessive,
less the punctuation that ends a sentence or a clause and the markdown that closes an emphasis or a
quotation around it. A token holding an angle
bracket is a placeholder, and is silent. The finding names its repair, the token in backticks, and,
where the token names nothing of this project, a rewrite of the sentence; `check --fix` writes
nothing for it, as for the other lints on ids, per `design@check@backticked-bare-id`.

**Beside a backticked reference to an issue entry, in the same paragraph, the token is silent.** It
is then the first exception of `design@agent-skills@plain-text-is-no-repair`: text in the checker's
syntax that no checked form expresses, written beside the issue that records the missing form. The
check reads the exception mechanically rather than leaving it to review, so the escape stays
available and nothing else clears the finding. **A quotation is read by none of the lints on ids**,
per `design@check@quotation-read-by-no-id-lint`.

The token is the same defect in any prose, so the check reads every region the scanner reads,
fenced blocks included, as `design@model@a-reference-is-live-in-every-prose` reads a fence. The
nearest rival, the check limited to the `%%` comment lines of the shipped text, which a
retrospective proposed, adds a rule with no reason behind it: a token written without its backticks
in a design home dangles as silently as one in a comment line. The census taken in the design
discussion, kept in its spec, `path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`,
found such tokens in this repository, every one a pointer, and none in thaum; the lint reports a
token a writer stripped of its backticks, which is the evasion
`design@agent-skills@plain-text-is-no-repair` refuses, and serves
`goal@knowledge-architect@documentation-stays-consistent`.

### A backticked span that is exactly a defined id is a finding, whose repair names every reference the id can take `##backticked-bare-id`

The scanner records every backticked span that is one word in the grammar of one segment or more,
a span wrapped across a line break at one of its hyphens joined, outside a quotation, per
`design@check@quotation-read-by-no-id-lint`. A lint of the last phase
reports such a span when the entity table defines its id: a register entry's id, a plan document's,
a slice's, a skill's or an agent's name, a section slug of a skill, an agent, the primer or the
root CLAUDE.md, and an item's id inside its own plan, the only place an item may be cited, per
`design@model@plan-item-scope`. Its repair is the entry's reference. A section slug is in scope as a
register's id is: the census of the design discussion, kept in its spec,
`path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`, found 34 plain hits on 10
section slugs in this repository, most of them pointers written in words. Under `harness = []` the table defines no harness
kind, per `design@model@harness-kinds`, so their names and slugs are silent. A commit message is
judged as every lint is, per `design@check@a-commit-message-is-a-document`, so a span naming an
entry the commit deletes is reported against its first parent's tree, and the candidates of both
trees are listed in one finding.

**A bare id is a pointer written with no kind.** Silent, it dangles unseen when the entry is renamed
or deleted. The reference costs the kind and its anchor, and `show` then lists the text among what
cites the entry, per `goal@knowledge-architect@records-reach-their-reader`. The nearest rival, no finding and a
listing of such spans, leaves every such pointer silent: `rejected@check@bare-id-listing-only`.

**The match is exact against the table.** A crate's name, an id of another project and an id whose
entry has left are silent, so the lint reports no span that names nothing here. Every id has two
segments or more, per `design@model@two-segment-ids`, so no ordinary word in backticks is a
defined id; a hyphenated term that is, such as a project skill named like a command, is renamed,
as `design@check@plain-word-lint` says of a subject slug. Whether a skill's name meets such a term
rests on the names: the installed ones carry the installer's prefix, and a project's own carry its
prefix, per `design@agent-skills@project-name-prefix`, which nothing checks yet, per
`issue@check@tooling-for-project-skills`. The census of the design discussion found few spans that
were no pointer, in thaum thread names backticked in a plan table against the design skill's own
rule. Re-take the false positives with `cargo klarch check` over a tree that has not
rewritten its bare ids, such as another project's at its upgrade, counting this lint's findings: a
finding on a span that is not meant as a pointer reopens the exact match.

**An id that several entries define gets one finding, which lists every reference it can take and
chooses none.** A skill and an agent of one name, and an agent's section slug shared by three agents,
are among the cases the census found, 18 such ids here and 41 in thaum, in the spec named above. Choosing the nearest definition would make the checker supply a meaning the writer did not
write, so the writer chooses. The same holds for the plain-word lint.

**The lints on ids report, and `check --fix` writes nothing for them**: no respelling of a
hand-written file is among the fixes `design@check@fix-scope` admits, per
`design@check@safe-fix-definition`. Respelling a span into its reference is the subject of
`spec@plans@path-quickfixes`. The rule holds for the three lints on ids.

**In a released section of CHANGELOG.md the id is written as its reference too**, a structural
change, and a skill's or an agent's name as the bare name again once it is deleted, per
`design@knowledge-architect@changelog-section-fixed-at-release`. That bare name names nothing in
the table, so the lint leaves it silent.

### A plain word that is exactly a defined id is a finding `##plain-word-lint`

A word outside every backtick span and every quotation, per
`design@check@quotation-read-by-no-id-lint`, in every prose region the checker reads, that is exactly
a defined id, as `design@check@backticked-bare-id` defines one, is a finding. A word
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
was a pointer written in words. A listing would leave those pointers unchecked: `rejected@check@plain-word-mention`. A slug kept as a concept index, so
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
line. None of the three lints on ids, `design@check@reference-shape-outside-span`,
`design@check@backticked-bare-id` and `design@check@plain-word-lint`, reads one. A quotation is
someone else's words, such as the owner's ruling on a thread named by its slug, or the rules text of
a specification, which its writer may not respell, so a finding there would have no repair the
writer may make. A quotation left open ends at its paragraph, which ends at a blank line, a heading,
a fence's edge, a list item or a table row, the blocks a renderer shows apart, so it cannot silence
the rest of a file. A pointer someone writes inside a quotation goes unchecked, which is what the
exemption costs. The nearest rival, no exemption, would report words the writer may not change. It
is the fourth exception of `design@agent-skills@plain-text-is-no-repair`.
