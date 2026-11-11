# Rejected designs — model

The designs the Component model did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@model@design` whose argument covers it. An entry stays `live` while its reason
holds, and is marked `void` when it no longer does.

### A planned path legal in every document `##planned-path-in-every-document`

`live`. An issue entry could then name the file its fix would create. It lost because an issue
states what is missing rather than the shape of its fix, and a hypothetical file there is served by
the escape anchor. It is recorded because a doubt remains:
`tripwire@model@planned-path-outside-plans` watches for a file named as to be created outside the
plans directory, and reopens the scope with this alternative as the candidate.

### A hand-rolled gitignore matcher reading the root file alone, with the walk pruning what it covered `##hand-rolled-gitignore-matcher`

`live`. It needed no `git` on the path and no process per run, and its supported subset was chosen
against thaum's own file. It loses on three counts, each measured against the tree rather than
argued: a tracked live document left the walk AND the inverse assertion when one root ignore line
matched its bare name at any depth, reproduced with a one-line `.gitignore` over a copy of
`path@testing@tests/projects/minimal/` and recorded as a defect before the reversal; a nested
`.gitignore` was not honoured at all, so a project needing one had to declare the path in the
manifest; and every pattern the matcher could not honour — a negation, `**`, `?`, a character class
— had to be refused by name, because dropping an ignore rule makes the walk read more than it should
and dropping a negation makes it read less, and both are silent. Git answers all three by
construction, and the walked-file count over thaum's tree was identical under the two.

### A slug unique across the whole project, with the component named for the reader only `##project-wide-unique-slug`

`live`. It keeps one meaning per word everywhere and needs no lookup to resolve a reference. It
loses because it makes every component's vocabulary global: two components cannot each decide
something they call the same word, and the second one to want the word has to take a worse one.

### A reference with no component read as one inside its own component `##implicit-own-component-reference`

`live`. It would leave a pointer inside a component as short as it was before components existed,
and qualify only the crossings. It loses on what a reference has to carry by itself: the same text
would name different decisions depending on which file it sits in, so moving a document between
components would silently retarget every unqualified reference in it.

### A fenced reference read as an illustration, the stance slug references took before the `@` grammar `##fenced-reference-as-illustration`

`live`. It let a document explaining the convention hold an example without inventing a slug. It
loses on consistency: path references were already live in a fence, so one kind had two stances, and
a sketch in a design document names its decisions as deliberately as it names its paths. The
illustration is a placeholder in angle brackets, which the tokenizer does not record.

### `#` as the reference separator, the incumbent of `` `<component>#<slug>` `` `##hash-reference-separator`

`live`. `design@model@an-entity-belongs-to-its-anchor` fixes the separator `@`. It costs no migration
of the slug references. It loses on collision: `#` is a Rust attribute opener and a markdown heading
marker, both of which sit inside code spans in thaum's tree, and `:` — the other candidate — is a
Rust path separator; `@` meets almost nothing in Rust and only email addresses in prose, which never
sit in backticks here. It entered the tree with the first migration to qualified references,
carrying no argument of its own.

### A configurable reference separator `##configurable-reference-separator`

`live`. `design@model@an-entity-belongs-to-its-anchor` fixes the separator `@`. A project could pick
the character its prose collides with least. It loses because every instruction and every skill
would be parameterised on it, and `@` already meets almost nothing in Rust and only email addresses
in prose, which never sit in backticks here.

### A register reference with no anchor, `<kind>@<id>`, for a register a project declares or carries in every Component `##register-reference-without-anchor`

`live`. It is shorter for the common case. It loses, for a register a project declares or carries
in every Component, because the extracted tool cannot know which such register is single-instance in
a given project, and one three-part grammar serves every kind without a special case in the resolver
or in the instructions. Its first reason does not hold for a kind whose namespace the harness fixes,
nor for a register the tool gives to the root alone, whose one carrier no declaration moves; the second stands as the cost
of a form with no anchor wherever one is written.

### `*` read under `register` as every anchor that carries the register, so that the goals home with `*` as its anchor names the root's `##star-names-every-carrier-of-a-register`

`live`. It keeps the form the shipped text wrote and the released changelog's line resolving, and
adds no reserved word. It loses at the owner's fork to the form with no anchor, which no goal
ranks. The costs weighed at that fork: `*` would read as every Component under `path` and as every
carrier under `register`, two readings of one word, and a goal would still be cited with an anchor
that each project spells with its own name.

### A reserved anchor word for the project's root, so that `register@<word>@goal` and `goal@<word>@<id>` resolve in every project `##reserved-word-for-the-root`

`live`. It names the root in every project and would give a root path a checked form too. It loses
at the owner's fork to the form with no anchor, and on a measured cost besides: `is_anchor_word` of
`path@model@src/entity.rs` covers every reserved word, so every backticked span of that word
before an `@`, as a login `root@<host>`, would be a finding in every project; and a Component or a
project named by the word would be refused. The loss covers register references only: a reserved
word for a root path stays open in
`issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`.

### The plan registers cited with no anchor, a milestone by its kind and its id alone, and the home of `spec` or `milestone` by the register alone `##plan-register-references-have-no-anchor`

`live`. It extends the form with no anchor to the registers of the anchor `plans`. It loses at the
owner's fork: the owner chose to keep the plan registers' anchor over dropping it for `milestone`
alone and over `spec@<id>` beside `spec@<milestone>@<slice>`, which no goal ranks. The costs weighed
at that fork: `spec` has two carriers, `plans` and each milestone, so a spec cited by its id alone
would read the anchor from the segment count within one kind, the special case the form with no
anchor refuses.

### The anchored goal form kept resolving beside the form with no anchor, as a second spelling `##anchored-goal-form-as-a-second-spelling`

`live`. It keeps every goal reference written with an anchor resolving. It loses on shape: two
spellings of one entry put two shapes on one meaning, the reason
`rejected@check@at-prefix-root-relative-path` gives.

### An anchor the tool constructs for the agent configuration, so that a skill, an agent and their sections keep three segments, as `skill@<cfg>@<name>` `##agent-configuration-anchor`

`live`. It keeps one arity for every kind. It loses because the anchor carries no information, one
namespace per harness, and every word for it stutters, as `skill@skills@<name>`, or names one
provider, which a skill served to two harnesses would cite two ways.

### A kind of its own for a section, as `section@<skill>@<slug>` `##section-kind`

`live`. One kind would keep one meaning. It loses because a section's reference would no longer say
whether the section belongs to a skill or to an agent, which the owner judged important, while the
third segment of `skill@<name>@<slug>` narrows within one kind as a path's last segment does.

### Keeping the `@` prefix as the escape `##at-prefix-escape`

`live`. It costs no migration. It loses on the census taken at the design session: the tree held 75
`@`-prefixed spans carrying three meanings — about 65 meant every component's own copy, a handful
meant a path outside the tree, three were Java annotations that were never path syntax — and the
scanner registered none of them, so a typo'd escape was invisible by construction. One mute marker
for three meanings is the confusion the anchor words dissolve.

### `syn` as the Rust parser `##syn-rust-parser`

`live`. Prototyped in thaum, where the tool was first built: it parses all 68 of its Rust files
without error, but doc comments survive only as
attributes and ordinary comments are discarded during lexing, so a rule cited in a `//` comment
inside a function body needs a second raw-text pass. One parser that answers every question beats
two that each answer half.

### Rust-analyzer's syntax crate `##rust-analyzer-syntax-crate`

`live`. Pure Rust, lossless and it keeps comments, but it is published per nightly, so adopting it
means pinning a crate of compiler internals that churns weekly.

### A hand-rolled indentation scanner, leaning on the formatter gate to normalise indentation `##hand-rolled-indentation-scanner`

`live`. Zero dependencies, and rejected because a mis-scope would be silent, which is the failure
class this tool exists to prevent. The line-prefix scanner it would have resembled was the single
cause of the four defects in thaum's tree that `design@model@grammars-not-prefixes` lists.

### `--lines` on thaum's interpretation index, a temporary copy carrying each citation's line numbers `##lines-flag-on-interpretation-index`

`live`. Refuted by a census over the file's whole history rather than by argument:
`git log -p -- docs/rules/interpretations/index.md` matches no line-numbered citing row, so in every
commit the index has ever had, the flag's output was never one of them. Every generated index gates
on data that does not move when unrelated prose shifts a line — the interpretation index carries one
row per entry, with no line and no citing list at all, and the rule index names its citing files at
file level — so per-line data stood against the argument for the file it would have sat in. What it
would have bought is answered instead by filtering `cargo klarch model` on the rule number.

### Every string literal outside the checker read as prose, bound to a name or not `##every-literal-read-as-prose`

`live`. The binding rule of `design@model@grammars-not-prefixes` decides against it. Proposed to
remove the three special cases the binding rule costs the extractor, on a measurement that found no
bound literal outside the checker carrying rule-shaped content. The measurement was wrong: built,
the checker reported six in three crates, an address, formatted figures a display test expects and a
line of tool output a parser is fed, and the class recurs wherever a test displays a three-digit
float.

### `additional-trackers` kept as a file list `##additional-trackers-file-list`

`live`. It named each tracker outside every component directly, needed no second kind of anchor, and
every check that read it worked. It loses because a file in that list had no anchor: nothing in the
project could cite what was open there, and the register thaum's rules directory holds could not be
named at all. The whole of this tool is about citing, so a recorded thing with no name is the one
shape it may not have.

### A summary column in a file register's index `##index-summary-column`

`live`. It would let a reader take in what an instance holds without opening a file, which is the
whole point of a listing. The summary loses on churn: it changes on every wording edit of every
entry, so an index that today is regenerated on create, delete, retitle, regroup and a metadata
change would go stale on each one and fail the gate. It is answered by a command instead,
`show <ref>` for the body.

### A last-change date in a file register's index `##index-last-change-date`

`live`. It would let a reader take in what an instance holds without opening a file, which is the
whole point of a listing. The date loses outright to `design@check@generated-files-are-pure`: the
last change is git's, not the walked tree's, so a generator reading it would rewrite the file on a
rebase and report a file stale that nobody had touched. It is answered by a command instead,
`issues` for the dates.

### Refusing to load a manifest whose register declaration is wrong `##refusing-a-wrong-declaration`

`live`. A declaration the tool cannot act on is a could-not-run by `design@cli@exit-code-ladder`,
and exiting 2 is what a manifest the grammar cannot parse does, a key of `[project]` it does not
know included. It loses for the case where the tool CAN act: a bad `dir` on a built-in has a
compiled default to fall back on, so refusing the whole run would report nothing at all about the
tree — and nothing at all is what a session reads as conformance. An unknown key keeps the refusal
because there is no default to fall back on.

### A slug defined at the end of any level-two or level-three heading, or in a table cell, with no level declared by the register `##slug-at-any-heading-or-cell`

`live`. It needed no manifest key, it let a register keep entries at either level, and the table
cell served decision tables written before the heading form. It loses because it cannot tell a
heading that is no entry from an entry whose slug is missing: a tripwire written without a slug is
defined nowhere, listed by no `tripwires` run and reported by no check, and a reviewer reading the
listing as the list of what to re-read never opens it. With one declared level per register, every
heading at that level is an entry and one without a slug is a finding. A table row has no heading,
so a decision defined in a cell is missing from the document outline, which is the index the heading
form exists to give.

### Table cells kept as a second definition site, behind a per-register manifest key `##table-cell-definition-site`

`live`. It would have kept decision tables readable as tables, and left the declared level to govern
headings alone. It loses because one register would then have two definition sites, and the outline
would again fail to list every entry of a register that opts in. The one home that held cell
definitions converts to headings without losing any content.

### A plans directory in every component `##plans-directory-per-component`

`live`. It lets a large project keep each component's plans beside it. It loses on the premises `design@model@plans-at-root` states: most work spans components, so its plans sit at the root anyway; no project should hold
enough open plans for a split to help; and plans designed before another is built break each other,
which separate directories hide from each other. A further cost, which the owner did not argue:
every component would owe a plans home that holds nothing.

### Citing an item of another plan `##cross-plan-item-citation`

`live`. It lets a plan name the exact thread of another plan it depends on. It loses because
whole-document citations already carry the dependency while few plans are open at once, and because
item citations across plans would force a design revisit of every citing plan, item by item, at the
moment a plan is retired, a moment the owner judged might not be the best for it.

### Checking the literal parts of a placeholder span `##placeholder-literal-part-check`

`live`. A span such as one whose kind is literal and whose anchor and id are placeholders would be
recorded, and its kind, its arity and a literal anchor checked. A census found no defect among the
placeholder spans of the tracked Markdown and Rust files: a `grep -ohE` of backticked spans holding
an `@` and an angle bracket over `git ls-files '*.md' '*.rs'`, each split on `@` and its kind, its
segment count and a literal anchor tested; the spans such a check would report were deliberate
illustrations of an invalid form and format strings of the checker's source. A defect found by that
census reopens this. The cost was the escape an illustration needs, and the edit of a bundled head.
A placeholder name fixed per kind lost too: the names carry meaning, as a milestone placeholder in a
slice spec's anchor does.

### `<component>` in place of the generic anchor `*` `##component-placeholder-anchor`

`live`. It reads to some writers as clearer than `*`. A span holding an angle bracket is not
recorded, so alone it makes the pointer silent; recorded as a reserved word, it is a second spelling
of what `*` means, and a typo of it is another placeholder, silent where a typo of `*` is an unknown
anchor. The owner kept `*` while still finding it "a bit confusing", which is the doubt this entry
keeps, against `goal@adoption-is-easy`.

### `home` as the word of the kind naming a register's home `##home-kind-word`

`live`. It matched the documents' phrase "design home", and cost the same refusal of an anchor name.
The owner chose `register`.

### A `path` that resolves to either shape of a heading register's home `##path-to-either-home-shape`

`live`. A path claims its target's kind by its trailing slash, per
`design@model@trailing-slash-claims-directory`; one that matches either shape asserts no claim and
puts a special case into path resolution.

### A `path` reference to a register's home accepted beside the `register` form `##path-to-home-beside-register-form`

`live`. Under the generic anchor it accepted every shape of a component register's home, whether or
not any component used that shape yet, so a text that must hold in every project could name both
shapes and pass. The `register` form serves that need with one span. Kept legal, the path form keeps
being written, and a home that changes shape dangles every one of them.

### Every subdirectory of a location an anchor, with no Directory register `##subdirectory-anchors-location`

`live`. A flag on a location would have made each of its subdirectories an anchor. It loses on two
facts: `milestone` needs entries that are directories, to be cited `milestone@plans@<id>`, so it
keeps the Directory shape, and the tool would hold two mechanisms placing anchors off the tree for
one shape; and it gives a subject no reference of its own and no generated listing of the subjects.

### A Component's register list fixed by the word _component_ `##component-register-list-fixed`

`live`. Every Component carried the component registers and nothing else, so one mechanism, the
location, placed every other register, and a Component's register list stayed a fact about the word
_component_. It loses on the owner's intent, that a project may add any kind of register to any
Component, the root included. Under it, a register's home at `<component>/docs/<dir>/` needs a location carrying
the register at its own path, a manifest row and an anchor name invented for each placement, where
attaching costs one key.

### A Component's attachment in place of `dir = "."` `##carrier-dir-replaced-by-components`

`live`. Attaching a register to a Component places its home under that Component's `docs/`, and
the key `dir = "."` with its rule, that a location carrying such a register carries no other, would
have left the manifest. It loses on the owner's choice to keep both, made on the session's
recommendation, whose argument is that only a location at its own path places a home directly at a
directory outside every Component's `docs/`, with no directory level between the location and the
entries.

### One-word ids exempt from the lints on ids `##one-word-id-exception`

`live`. A defined id of one segment would be silent wherever it is written alone or as a word, so
the lints would report no ordinary word. It loses on the census of the design discussion, kept in
its spec, `path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`: in thaum,
661 of the 713 plain hits fell on its one-word ids, its argument items and its issues named after a
feature area, such as an issue on combat cited 68 times, and under the exemption no mention of such
an issue would ever be checked. A rename to a second segment costs no capability.

### Plan argument items exempt from the two-segment grammar `##plan-items-exempt-from-the-grammar`

`live`. An item of a plan document would keep a one-segment id, numbered `a1`, `a2`, since an item
is cited from inside its own plan only, so no lint outside it would meet the word. It loses because
it adds an exception to the grammar every project's documents follow, justified by nothing but the
item's scope, while numbering the items `arg-1`, `arg-2` costs no capability.

### The phase-1 contribution as a second trait `##contribution-as-a-trait`

`live`. Phase 1 would call an extension's code, so a test of it would need an extension, and a tool
with no extension would name a trait it implements nowhere. It would change an interface others consume.

### The words of a cause in the lints of the checks `##cause-words-in-the-lints`

`live`. A tool that prints a refusal would then have to link the checks or write the words again. It
would change an interface others consume.

### The check-ignore batch of the working tree gathered with the checks `##ignore-batch-in-the-checks`

`live`. A consumer of the resolver with no check would read each target the ignore rules cover as
absent. It would change an interface others consume. A run over a tree git holds does ask in `check`,
since no gathering of the working tree reads that tree.
