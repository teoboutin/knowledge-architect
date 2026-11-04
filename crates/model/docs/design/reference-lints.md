# Model design: Reference spans

## Reference spans

### A backticked `@` span is a reference candidate when its head is a kind or an anchor, and every other `@` span is silent `##at-span-candidate-rule`

The scanner records every backticked span that holds an `@` and no whitespace, backtick or
angle bracket, as written; it has no manifest, so it cannot tell a kind from an email address.
The resolver reads the head — the text before the first `@` — and decides: a known kind is
segmented and resolved; a declared anchor or a reserved anchor in that position is the old form
and is reported with the repair "prefix the kind"; anything else is not a reference and reports
nothing, so an email address or a git remote in backticks is silent unless the project declares
an anchor by that word. Every kind but `path`, `planned` and the harness kinds takes exactly three
non-empty segments; `path` and `planned` take an anchor and then everything after the second `@`;
a harness kind takes the arities of `design@model@harness-kinds-cited-without-anchor`.

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

### A reference is live wherever it is prose, fenced blocks included, for every kind `##a-reference-is-live-in-every-prose`

A sketch names what it names on purpose, and an illustration writes a placeholder in angle
brackets, which the tokenizer does not record. Reading a fenced slug reference as an illustration is
`rejected@model@fenced-reference-as-illustration`. A string literal bound to a name in Rust yields no
reference, per `design@model@grammars-not-prefixes`. Checking the literal parts of a placeholder
span is `rejected@model@placeholder-literal-part-check`.

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
  `design@model@plans-is-a-reserved-anchor` makes for `plans`. A reference to a harness kind still serves
  `goal@core@relocation-needs-only-goal-changes`, which binds a reference to what the manifest
  places: nothing a harness kind names moves with a manifest edit.
- **The rejected design "A register reference with no anchor"**,
  `rejected@model@register-reference-without-anchor`, lost on two reasons. Its first, that the tool cannot
  know which register is single-instance in a project, does not hold here: the tool fixes the four
  kinds and their layout, and no project declares one. Its second, one grammar with no special case
  in the resolver, is the cost of this form.
- **The kind is read first**, so a word that is both a kind and an anchor would read as the kind;
  no anchor may wear a kind's name, per `design@model@kind-names-refused-as-anchors`.
- **`instructions` names the document's role**, not one harness's file name, so a move of the root
  instructions to another file name changes no reference, per
  `issue@core@configuration-for-several-agent-providers`.

The nearest rival, an anchor the tool constructs for the agent configuration with three segments
everywhere, lost: every name for it stutters or names a provider, and it carries nothing. It is
`rejected@model@agent-configuration-anchor`. Serves `goal@knowledge-architect@every-target-project-can-adopt-it`:
one grammar, the same in every project, with a global namespace naming no anchor.

The design this decision did not keep: `rejected@model@section-kind`.

### Every level-two heading of a section home of the harness is a section, and owes a slug `##section-homes-carry-slugs`

A section home is a skill's `SKILL.md`, an agent's file, the primer and the root CLAUDE.md, under
the `claude` harness. The rule of `design@model@an-entry-is-a-heading-at-the-register-level` holds
there at level two: every level-two heading outside a fence owes a slug, which defines a section
of that home's skill, agent or document, and a slug anywhere else in it defines nothing, per
`design@model@a-slug-elsewhere-defines-nothing`. An id is
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
  checker. The shipped set is held to the rule where it is written, by a test of the checker over the
  shipped text, which also requires every reference of a harness kind in it to resolve within the
  set.
- **A project skill's or agent's slug at a level-two heading defines a section** of that skill or
  agent, whatever anchor holds its file.

The finding on an unslugged heading names the repair, and the setup skill states the requirement,
so an adopting project meets it in the documentation. Serves `goal@core@records-reach-their-reader`:
`show` on a section lists every text that cites it.
