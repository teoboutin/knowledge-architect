# Per-crate changelogs: the deliberation

## Threads

### One source `CHANGELOG.md` per published crate, holding only its entries `##changelog-per-component`

Proposed by the owner, R1. Final state: approved. Moved by `argument@per-crate-changelogs@arg-1`,
`argument@per-crate-changelogs@arg-12`, `argument@per-crate-changelogs@arg-6`. Its shape is the
milestone document's "One changelog per published crate"; slice 3 builds the model of the file and
slice 5 writes the files. Closed by the owner's agreement at R2, with the argument that a separate
changelog per published crate is the only shape that serves consuming projects, and confirmed with
the checkpoint table at R8.

### An entry that covers several crates: a root source file, withdrawn for one entry in each crate concerned `##cross-crate-entries`

Proposed by the session, R1. Final state: withdrawn by the session at R2, superseded by
`thread@per-crate-changelogs@changelog-per-component`. Moved by `argument@per-crate-changelogs@arg-5`,
`argument@per-crate-changelogs@arg-20`, `argument@per-crate-changelogs@arg-26`. Its replacement, one
entry in each crate whose consumers meet the change, is in the milestone document's "One changelog
per published crate" and the conversion's mapping. The owner agreed to the withdrawal at R3, adding
that every crate must be thought of as consumed on its own.

### The pin move reads the crates' own changelogs through `klarch changelog`; the assembled file exists once `##changelog-the-pin-reads`

Proposed by the session, R1, as two shapes, A and B, each shipping the assembled file in the binary
crate; reshaped at R2 to a single assembled file, each crate shipping its own, and the pin move
reading the registry sources; given its tool at R3. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-3`, `argument@per-crate-changelogs@arg-13`,
`argument@per-crate-changelogs@arg-18`, `argument@per-crate-changelogs@arg-25`,
`argument@per-crate-changelogs@arg-41`. Its shape is the milestone document's "What a project moving
its pin reads", with the command in slice 4. Closed by the owner's agreement at R4, after the owner's
condition at R3 that an agent reading the files by hand would make mistakes unless a tool assembles
them from the local registry; the preamble's URL and the requirement of `repository` were dropped
on the owner's word at R6; confirmed at R8.

### The assembled file's location is a path the manifest sets `##assembled-changelog-location`

Proposed by the session, R2, answering the owner's question at R2 whether the location is set by
path or by Component. Final state: approved. Moved by `argument@per-crate-changelogs@arg-19`. Its
shape is in slice 3's spec. The owner agreed to the shape of the round at R3 and confirmed the table
at R8.

### The assembled layout is version, then category, then package name in the manifest's order `##assembled-changelog-layout`

Proposed by the session, R1, against the owner's layout of R1, version then Component. Final
state: approved. Moved by `argument@per-crate-changelogs@arg-8`, `argument@per-crate-changelogs@arg-14`,
`argument@per-crate-changelogs@arg-21`, `argument@per-crate-changelogs@arg-27`. Its shape is in slice
3's spec. The owner agreed to the layout at R2, adding that the headings name the published package
names, and chose the manifest's order of the crates at R3; confirmed at R8.

### An entry goes to the crate whose public surface the consumer meets the change through `##entry-owning-component`

Proposed by the session, R1. Final state: approved. Moved by `argument@per-crate-changelogs@arg-4`,
`argument@per-crate-changelogs@arg-9`. Its shape is in slice 5's spec, with the conversion's mapping.
The owner agreed at R2 and confirmed at R8.

### Each entry is `- <category>, <class>: <text>`, the categories declared, optionally per Component `##changelog-entry-categories`

Proposed by the owner, R2: the subsections are to be customisable per project and per Component,
replaced by a tag in a crate's file that becomes a section once assembled. Shaped by the session at
R2. Final state: approved. Moved by `argument@per-crate-changelogs@arg-15`,
`argument@per-crate-changelogs@arg-22`, `argument@per-crate-changelogs@arg-44`. Its shape is in
slice 3's spec. The owner agreed to the shape at R3 and confirmed at R8.

### A version with no entry in a crate is omitted from that crate's file `##absent-version-section`

Proposed by the session, R1. Final state: argued. Moved by `argument@per-crate-changelogs@arg-10`,
closed by `argument@per-crate-changelogs@arg-24`: the bump check compares versions over every
crate's file together under the lockstep, so a heading with no entry adds nothing a check or a
reader uses. Its shape is in slice 3's spec.

### The parse and the rendering in `model`, the checks in `check`, the crates listed or else read from `publish` `##changelog-model-home`

Asked by the session, R1, as a question to the owner: `check` or a gate of `xtask`, and whether
adopting projects will use it. Answered by the owner, R2. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-7`, `argument@per-crate-changelogs@arg-16`,
`argument@per-crate-changelogs@arg-23`, `argument@per-crate-changelogs@arg-28`, refined by
`thread@per-crate-changelogs@changelog-opt-in`, `thread@per-crate-changelogs@changelog-grammar-compiled-in`
and `thread@per-crate-changelogs@changelog-findings-in-check`. Its shape is the milestone document's
"Where the changelog's model and its checks live", built in slice 3. Closed by the owner's
agreement at R3, with the owner's reading that the head that keeps the changelog out of the walk
was about its consequences, which the shape keeps; confirmed at R8.

### The changelog checks run only where the manifest declares `[changelog]` `##changelog-opt-in`

Proposed by the session, R6, from the record search. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-42`. Its shape is in slice 3's spec. The owner agreed at R7 and
confirmed at R8.

### The entry grammar and the classes are compiled in; the categories are declared `##changelog-grammar-compiled-in`

Proposed by the session, R6, from the record search. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-43`. Its shape is in slice 3's spec. The owner agreed at R7 and
confirmed at R8.

### Every changelog finding is a check of `check`; the parse raises no phase finding `##changelog-findings-in-check`

Proposed by the session, R4, from the code-split scout's report. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-37`. Its shape is in slice 3's spec. The owner agreed at R7, the
draft's contrary sentence on structural findings in `model` removed, and confirmed at R8.

### A released version follows the highest class of its entries, read in the tree alone `##changelog-bump-check`

Proposed by the session, R2, from the owner's example at R2 that the checker could verify version
bumps against each entry's class. Final state: approved. Moved by `argument@per-crate-changelogs@arg-16`,
`argument@per-crate-changelogs@arg-24`. Its shape is in slice 3's spec. The owner agreed at R3 and
confirmed at R8.

### The assembled file opens with a generated header, and `--fix` refuses to overwrite a file without it or to write while a line is unread `##assembled-file-not-overwritten`

Proposed by the session, R6, from the record search. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-48`. Its shape is in slice 3's spec. The owner approved it at R7
and confirmed at R8.

### The checker writes the category order into each crate's preamble `##category-order-in-preamble`

Proposed by the session, R6, from the record search. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-49`. Its shape is in slice 3's spec. The owner agreed at R7 and
confirmed at R8.

### `[walk.citation-free]` declares files at two levels, `all` and `entries` `##citation-free-paths`

Proposed by the owner, R3: a feature that declares files or paths the checker scans to ensure they
are free of citations. Shaped by the session at R3 as one level; given two levels at R6 as the
session's default for Q2, ruled by the owner at R7. Final state: approved. Moved by
`argument@per-crate-changelogs@arg-29`, `argument@per-crate-changelogs@arg-32`,
`argument@per-crate-changelogs@arg-33`, `argument@per-crate-changelogs@arg-44`,
`argument@per-crate-changelogs@arg-45`, `argument@per-crate-changelogs@arg-46`,
`argument@per-crate-changelogs@arg-56`, `argument@per-crate-changelogs@arg-58`. Its shape is in
slice 1's spec. The owner agreed at R4 and ruled Q2 at R7, approving the two levels with the reason
that installed text citing other installed text is useful; confirmed at R8.

### No changelog file holds a reference; the released spans are rewritten in words `##changelog-holds-no-citation`

Proposed by the owner, R3: the changelog faces the public, a reference to an entry serves its
reader in no useful way, and the citation system serves the internal documentation. Final state:
approved. Moved by `argument@per-crate-changelogs@arg-29`, `argument@per-crate-changelogs@arg-34`,
`argument@per-crate-changelogs@arg-45`, `argument@per-crate-changelogs@arg-58`. Its shape is in
slice 5's spec. The owner agreed at R4; at R7 the owner considered letting the changelogs cite
installed text and kept the strict reading, because a frozen released section cannot follow a
citation that changes; confirmed at R8.

### No check reads the history before the base of the branch it judges `##no-history-beyond-the-branch`

Proposed by the owner, R3: the general decision that a check reads no history beyond the branch
deserves its own head, since it shapes further design. Drafted by the session at R3. Final state:
approved. Moved by `argument@per-crate-changelogs@arg-17`, `argument@per-crate-changelogs@arg-30`,
`argument@per-crate-changelogs@arg-35`, `argument@per-crate-changelogs@arg-53`. Its shape is in
slice 1's spec, which writes the head. The owner agreed at R4 and confirmed at R8.

### A new Component `cargo` reads what the tools need of Cargo, in lockstep `##cargo-crate`

Proposed by the session, R4, from the code-split scout's report. Final state: argued, with its
versioning ruled by the owner at R7: the question of per-crate versions goes to a `todo` issue at
the top of the roadmap, to be discussed before the next release, since a new crate never argues
versioning. Moved by `argument@per-crate-changelogs@arg-38`, `argument@per-crate-changelogs@arg-55`,
`argument@per-crate-changelogs@arg-57`. Its shape is in slice 2's spec.

### `klarch changelog` assembles the changelogs of a consumer's dependencies from the registry sources `##registry-changelog-command`

Proposed by the session, R3, answering the owner's condition at R3 that a tool assemble the
changelogs from the local registry. Final state: argued. Moved by
`argument@per-crate-changelogs@arg-31`, `argument@per-crate-changelogs@arg-39`,
`argument@per-crate-changelogs@arg-41`, `argument@per-crate-changelogs@arg-50`,
`argument@per-crate-changelogs@arg-52`. Its shape is in slice 4's spec. Its closure rests on the
selection over the resolved graph, on the argument for its placement in `klarch`, and on the
owner's agreement to the pin move's shape at R4.

### A crate that holds a changelog and sets `include` lists its `CHANGELOG.md` there `##changelog-include-check`

Proposed by the session, R4, from the code-split scout's report: the assertion of the test in
`path@xtask@src/changelog.rs` needs a home once that module goes. Final state: argued. Moved by
`argument@per-crate-changelogs@arg-40`. Its shape is in slice 3's spec.

### A Migration entry names an audit axis by its section title, verbatim, and cites none `##axis-named-in-words`

Proposed by the session, R3, as a consequence of `thread@per-crate-changelogs@changelog-holds-no-citation`;
argued against `design@agent-skills@a-reference-claims-a-revisit` at R6; made verbatim at R7 from
premortem cause 4. Final state: argued. Moved by `argument@per-crate-changelogs@arg-34`,
`argument@per-crate-changelogs@arg-47`. Its shape is in slice 5's spec.

### A check that a released section equals its content at the version's tag `##released-content-check`

Proposed by the session, R1. Final state: ruled-out, by the owner at R2, on two arguments: no check
reads history beyond one branch, and a released text may be reworded without a change of meaning.
Moved by `argument@per-crate-changelogs@arg-11`, `argument@per-crate-changelogs@arg-17`,
`argument@per-crate-changelogs@arg-54`. Its `rejected` entry is written by slice 1, as its spec
says.

## Arguments

### The shared changelog grows without bound, in 11 copies, and floods the lower crates `##arg-1`

R1, the owner. Bears on `thread@per-crate-changelogs@changelog-per-component`. The owner reported
that one changelog shared by every crate and copied into each published crate grows without bound,
creates large files of no use, and fills the changelogs of the lower-level crates with entries
that are not theirs.

### The conversion is a change of structure only `##arg-2`

R1, the owner. Bears on `criterion@per-crate-changelogs@released-entries-keep-their-content` and
`thread@per-crate-changelogs@changelog-holds-no-citation`. The owner stated that every change the
conversion applies to the existing changelog is structural: no content removed, only moved or
restructured, with nothing lost to the reader.

### A consuming project reads exactly one file: the binary crate's `##arg-3`

R1, the session. Bears on `thread@per-crate-changelogs@changelog-the-pin-reads`. "Setup skill, pin
move step 2: 'read the file in the source cargo downloaded … `$CARGO_HOME/registry/src/<index>/knowledge-architect-<version>/CHANGELOG.md`'.
Package `knowledge-architect` is `cli`. If `cli` shipped only its own entries, a project moving its
pin would no longer see the `checks`, `manifest` or `agent-skills` Migration entries, which are most
of what it must act on."

### The current surfaces are not crates `##arg-4`

R1, the session. Bears on `thread@per-crate-changelogs@entry-owning-component`. "`agent-skills`,
`gates`, `mutate` and `cli` map to one crate each. `checks` is split: phases 1 to 3 are in `model`,
the checks are in `check`. `manifest` is read by `model`, and `library` covers several crates. So
about 100 released entries need a crate chosen one by one."

### Some entries belong to no single crate `##arg-5`

R1, the session. Bears on `thread@per-crate-changelogs@cross-crate-entries`. "Example: '`library`,
minor: every published crate compiles for Windows …' (Next release). The root `knowledge-architect`
Component is the obvious owner."

### The lockstep stays, and per-crate files are a precondition of per-crate versions `##arg-6`

R1, the session. Bears on `thread@per-crate-changelogs@changelog-per-component` and
`thread@per-crate-changelogs@cargo-crate`. "Every crate gets every version (`design@knowledge-architect@version-lockstep`),
so most crates have no entries for most versions. Per-crate files do not reopen lockstep. They are
a precondition for `issue@knowledge-architect@every-crate-takes-a-version-at-each-release`."

### A structural parse does not conflict with keeping the changelog out of the walk `##arg-7`

R1, the session. Bears on `thread@per-crate-changelogs@changelog-model-home`. "That head argues
against reading the changelog's spans as references, since released sections would then have to
follow grammar changes. A dedicated parser that reads sections, entries and classes, and never
resolves spans, has no such cost."

### The assembled file's one reader acts on a version's Migration block `##arg-8`

R1, the session. Bears on `thread@per-crate-changelogs@assembled-changelog-layout`. "The right one
groups every Migration entry of a version in one block, which is the block a pin move must act on
(step 2 lists those entries). My default is the right layout, because the assembled file's one
reader is the pin move. The cost is that the assembled file is a re-sort, not a join."

### Placing by the code's crate shows the reader the internal layering `##arg-9`

R1, the session. Bears on `thread@per-crate-changelogs@entry-owning-component`. "That puts a finding
of a check in `check`, even when phase 2 in `model` reports it, a manifest key in `model`, a command
in `cli`, and an API change in its crate. The rival is the crate whose code the change lands in. It
is simpler for the author, but it leaks the layering to the reader. Under either rule, the surface
tag on each line goes away (the heading carries it)."

### An empty version heading in nine files per release adds nothing `##arg-10`

R1, the session. Bears on `thread@per-crate-changelogs@absent-version-section`. "Default: omit it,
and let the model assert that every version the assembled file holds has at least one entry
somewhere. An empty heading in nine files per release adds nothing a reader uses."

### A released version's entries could be compared with the tree at its tag `##arg-11`

R1, the session. Bears on `thread@per-crate-changelogs@released-content-check`. "A released
version's entries, read as a set of (crate, subsection, class, text), equal that version's entries
in the tree at its tag. This conversion breaks the comparison for 0.1.0 to 0.10.0 … the check
starts at the first release after the conversion."

### Consuming projects need a changelog per published crate `##arg-12`

R2, the owner. Bears on `thread@per-crate-changelogs@changelog-per-component`. The owner argued
that, where the changelog workflow and its tools are offered to consuming projects, since most
projects publish and maintain changelogs, a separate changelog per published crate is the only
shape that makes sense in the end: the problem is not only size.

### The assembled file is read by people too, at the root, and must not be copied `##arg-13`

R2, the owner. Bears on `thread@per-crate-changelogs@changelog-the-pin-reads` and
`thread@per-crate-changelogs@assembled-changelog-location`. The owner was inclined to point the pin
move to the root changelog on GitHub, as the generic and natural place, which users look at
without the workflow pointing them to it; no shape may misdirect a user; the changelog serves
project owners deciding whether and when to move the pin, not only agents moving it; a consuming
project must be able to set the assembled file's location, by path or by Component; and the owner
refused keeping the assembled file both in `cli` and at the root, another large duplicated file
that grows without bound.

### The assembled headings name the published packages `##arg-14`

R2, the owner. Bears on `thread@per-crate-changelogs@assembled-changelog-layout`. The owner agreed
to the layout grouped by category, as more actionable, and asked that its headings use the
published crate names, with the prefix, rather than the Components' local names.

### The subsections are the project's, not the tool's `##arg-15`

R2, the owner. Bears on `thread@per-crate-changelogs@changelog-entry-categories`. The owner asked
whether the subsections would ship as enforced on consuming projects, judged that they need to be
customisable per project and perhaps per Component, found a Workflow subsection kept for one crate
odd, and proposed a tag in each crate's changelog that becomes a section once assembled.

### Consumers will use the model; the structure in `model`, the semantics in `check` `##arg-16`

R2, the owner. Bears on `thread@per-crate-changelogs@changelog-model-home` and
`thread@per-crate-changelogs@changelog-bump-check`. The owner expects consumers to use it. The model
is in `model`, with the structural checks there where constructing the model depends on them, and
the more semantic checks in the checker, such as a check that version bumps agree with each entry's
class. The crates that hold changelogs are declared in the manifest or detected from the `publish`
key of each Cargo.toml, and the manifest may hold a few more options. The owner reported that the
release procedure's generic parts would ship later, and that no consuming project needs this yet,
though one will within weeks to months.

### No check reads history beyond the branch; a released text may be reworded `##arg-17`

R2, the owner. Bears on `thread@per-crate-changelogs@released-content-check` and
`thread@per-crate-changelogs@no-history-beyond-the-branch`. The owner is against checks that read
git history past one branch; and a released text sometimes needs a change of wording without a
change of meaning, such as for a new constraint of the checker, which the rules fully allow: the rule
needs more flexibility than a mechanical check allows.

### The registry already holds every dependency's source, and GitHub may be out of reach `##arg-18`

R2, the session. Bears on `thread@per-crate-changelogs@changelog-the-pin-reads`. "`ls <home>/.cargo/registry/src/*/`
on this machine lists `knowledge-architect-0.10.0`, `knowledge-architect-agent-skills-0.10.0`,
`knowledge-architect-check-0.10.0`, `knowledge-architect-git-0.10.0` and the others." Against
GitHub: "The network may not reach it … The URL is not always known … It removes nothing that a
person can read today." Its cost: "the agent reads one file per crate, about twelve here, instead of
one."

### A path covers the root and a project with one crate; a Component covers neither `##arg-19`

R2, the session. Bears on `thread@per-crate-changelogs@assembled-changelog-location`. "The root is
not a published crate here, and in a project with a single crate the assembled file would be that
crate's own file. A path covers both cases, a component covers neither. Default:
`[changelog] assembled = \"CHANGELOG.md\"`. A project with one published crate omits it, and no file
is assembled."

### A root source file hides an entry from a lower crate's reader `##arg-20`

R2, the session. Bears on `thread@per-crate-changelogs@cross-crate-entries`. "A reader of
`knowledge-architect-gates` alone would never see an entry kept there, such as 'compiles for
Windows', even though it concerns gates. The replacement: such an entry is written in every crate
whose consumers meet it. … the project is named `knowledge-architect`, and so is the binary's
package, so a root heading would collide with the `cli` heading."

### The crates under a category: the manifest's order or the alphabet `##arg-21`

R2, the session. Bears on `thread@per-crate-changelogs@assembled-changelog-layout`. "My default is
the order the manifest declares the Components in. The rival is alphabetical by package name, which
needs no declaration but puts `knowledge-architect` (the binary) first only by chance."

### Tags keep the line grammar; headings per crate would multiply `##arg-22`

R2, the session. Bears on `thread@per-crate-changelogs@changelog-entry-categories`. "A category used
by a crate outside its list is a finding. The set is fully the project's choice, so a consumer is
not given ours. The test for each category … stays in the workflow's text, not in the checker. …
The classes stay patch, minor and major. They are Cargo's semver. … The rival was headings in each
crate's file. It loses because a crate with three entries would then carry three headings, and
Workflow would still exist in only one file."

### The head keeping the changelog out is rewritten, and the copy job goes `##arg-23`

R2, the session. Bears on `thread@per-crate-changelogs@changelog-model-home`. "The changelog files
enter the model, so `design@knowledge-architect@the-changelog-is-outside-the-walk` is reversed in part. The model reads their
sections, entries and classes. It never resolves a span as a reference … The assembled file becomes
a generated file. The check verifies it is current and `--fix` writes it, the same way as
`index.md`. `cargo x changelog` loses its copy job."

### The bump check reads the files in the tree alone, and settles the empty sections `##arg-24`

R2, the session. Bears on `thread@per-crate-changelogs@changelog-bump-check` and
`thread@per-crate-changelogs@absent-version-section`. "Per-crate versions: in each crate's file, the
class of a released section implies its version, compared with the previous section in the same
file. Lockstep (`version.workspace = true` in every crate): the comparison runs over all crates
together … Under 0.x, major and minor both bump the second number … A file with no classes, 0.1.0
here, is exempt. … a version with no entries is omitted from a crate's file, and the lockstep
comparison uses the versions all files hold together."

### Network independence is no argument; a tool is needed to read without mistakes `##arg-25`

R3, the owner. Bears on `thread@per-crate-changelogs@changelog-the-pin-reads` and
`thread@per-crate-changelogs@registry-changelog-command`. The owner argued that the pin move cannot
be free of the network, since fetching the crate and running a Claude model both need it, though a
strongly sandboxed agent could keep the argument; judged the shape right; and said that an agent
reading the files by hand may fail to move the pin without mistakes, unless a tool assembles from
the local cargo registry.

### Every crate can be consumed on its own; the assembled file is an overview `##arg-26`

R3, the owner. Bears on `thread@per-crate-changelogs@cross-crate-entries` and
`thread@per-crate-changelogs@changelog-per-component`. The owner agreed, adding that such things
must be thought from the point of view that every crate can be consumed independently, the
assembled changelog being there for the overview of the whole project and for convenience.

### The manifest's order is a choice with no new option `##arg-27`

R3, the owner. Bears on `thread@per-crate-changelogs@assembled-changelog-layout`. The owner chose the
manifest's order, as a choice that adds few option shapes.

### The head keeping the changelog out was about its consequences `##arg-28`

R3, the owner. Bears on `thread@per-crate-changelogs@changelog-model-home`. The owner stated that
`design@knowledge-architect@the-changelog-is-outside-the-walk` was not about keeping the changelog
outside the walk at any cost, but about the consequences that brings, which this shape preserves.

### The changelog faces the public, and the citation system serves internal documents `##arg-29`

R3, the owner. Bears on `thread@per-crate-changelogs@changelog-holds-no-citation` and
`thread@per-crate-changelogs@citation-free-paths`. The owner argued that the changelog is
public-facing documentation and should hold no reference to the project's entries, which serve its
readers in no useful way; that a rendered site with clickable references might change that, but a
changed decision would then leave older sections' citations dangling, while a repair into plain text
is a change of structure only; that the citation system mostly serves the internal developer
documentation, the design record; that the CRATES-IO.md pages are in the same case; and proposed
declaring files or paths the checker scans to ensure they hold no citation.

### Reading past history costs every future run without bound `##arg-30`

R3, the owner. Bears on `thread@per-crate-changelogs@no-history-beyond-the-branch` and
`thread@per-crate-changelogs@released-content-check`. The owner argued that a check reading past
history creates a cost that grows without bound for every future piece of work, which the project
has no way to control; that the owner does not want such ideas invited into the project; that
handling every unexpected case of a consuming project's history needs excessive design and code;
and that the value is low, since a review would flag a deviation even without being asked to look.

### `cargo metadata` gives every dependency's source, for any project `##arg-31`

R3, the session. Bears on `thread@per-crate-changelogs@registry-changelog-command`. "It runs `cargo
metadata` in the consuming project. That gives the path of each dependency's unpacked source, in
the registry or a path dependency, so the tool never needs to know the registry's directory layout.
… This works for any dependency whose changelogs follow the model, not only ours. That fits
`goal@generic-tools-and-procedures-are-shipped`. Default home: `query` … Pin move step 2 of the
installed setup skill becomes 'run it, and read its output'."

### The crates.io page's rule is a second member, unenforced today `##arg-32`

R3, the session. Bears on `thread@per-crate-changelogs@citation-free-paths`. "`design@knowledge-architect@crates-io-page-file`
says the page 'holds no backticked reference', but no check enforces it. `grep -rli crates-io
crates/*/src` finds no code, and the manifest has no row for it. So the rule exists today only as a
sentence in a design head, and that is a second member for your feature."

### A parsed span is a finding, resolved or not; a placeholder passes `##arg-33`

R3, the session. Bears on `thread@per-crate-changelogs@citation-free-paths`. "In a matched file, a
span that parses as a reference is a finding whether or not it resolves, and no span there is
resolved at all. A placeholder in angle brackets … is an illustration and passes … The assembled
file is generated from the checked sources, so it is not matched. … A future render site that wants
clickable links changes this one line of configuration, not the design."

### An audit trigger is named in words; an `Audit` category cannot hold it `##arg-34`

R3, the session. Bears on `thread@per-crate-changelogs@axis-named-in-words` and
`thread@per-crate-changelogs@changelog-holds-no-citation`. "`design@knowledge-architect@changelog-entries` and
`design@agent-skills@audit-triggers` make a Migration entry cite an audit axis's section 'as a
reference', and the installed setup skill looks for exactly that. Both change to an entry that
names the axis in words … The rival was a dedicated category such as `Audit`. It loses because a
single entry is both a Migration and an audit trigger, and a category can be only one."

### The head widens the `check` head to every check, and its rival earns a `rejected` entry `##arg-35`

R3, the session. Bears on `thread@per-crate-changelogs@no-history-beyond-the-branch`. "It sits beside
`design@check@check-reads-no-range-of-history`, which covers `check` alone, and widens it to every
check and to `commits`, whose range already starts at the base. `--require-rebased` asks only
whether `origin/main` is an ancestor of `HEAD` … so it is within the rule. Its rival,
#released-content-check, gets a `rejected` entry: the issue `a-mechanical-changelog-check` lists
that assertion as a thing to build, so someone could plausibly propose it again."

### A required `repository` was proposed `##arg-36`

R4, the owner. Bears on `thread@per-crate-changelogs@changelog-the-pin-reads`. The owner proposed
adding `repository` where the crates here lack it, and perhaps requiring it of consuming projects in
the crates that hold a changelog. The premise was the session's statement that the design needed
it, which `argument@per-crate-changelogs@arg-41` showed false; the owner withdrew it at R6.

### A finding of phases 1 to 3 would stop `id` and `worklist` `##arg-37`

R4, the session, from the code-split scout's report. Bears on
`thread@per-crate-changelogs@changelog-findings-in-check`. "`design@knowledge-architect@model-crate`
says: 'Phases 1 to 3 say whether the model can be trusted, not whether the content conforms.' … `id`
and `worklist` need phases 1 to 3 to pass. So one misspelt category would make `klarch id` refuse to
run. … This is your rule applied as you worded it, 'when constructing the model depends on it':
nothing in the model depends on a changelog's content."

### No Component reads Cargo, and the work needs it in two places `##arg-38`

R4, the session, from the code-split scout's report. Bears on `thread@per-crate-changelogs@cargo-crate`.
"The checks need it to tell which crates are published, their versions (with workspace
inheritance), `repository` and `include`. The registry command needs it to run `cargo metadata`. …
Its precedent is the `git` crate (`design@knowledge-architect@git-crate`). `model` and `query`
depend on it, and no cycle forms. The rival is a module of `model`. It loses because `model` would
gain a second reason to change, Cargo's formats, and the `cargo metadata` output would then be read
in a second Component as well."

### The command does not fit `query` as it is defined `##arg-39`

R4, the session, from the code-split scout's report. Bears on
`thread@per-crate-changelogs@registry-changelog-command`. "`design@knowledge-architect@query-crate`
gives `query` 'answering a question about the record'. A dependency's changelog is not this
project's record. Default: the command stays in `query`, and its purpose widens to the record of the
project or of one of its dependencies. With the Cargo formats isolated in `cargo`, and the parse and
rendering in `model`, the command itself is thin."

### The `include` assertion needs a home once the xtask test goes `##arg-40`

R4, the session. Bears on `thread@per-crate-changelogs@changelog-include-check`. "A crate that has a
changelog and sets `include` must list its `CHANGELOG.md` there. This becomes a check of `check`,
and replaces that assertion in the xtask test."

### Neither use of `repository` needs the field `##arg-41`

R5, the session. Bears on `thread@per-crate-changelogs@changelog-the-pin-reads` and
`thread@per-crate-changelogs@registry-changelog-command`. "Requiring `repository` came from two
proposals of mine. Neither one actually needs it. … For the preamble: the line can name the
assembled file in words … A project that wants a link sets the URL explicitly, as an optional
`[changelog]` key. … For the selection: the default can be the named package alone, with `--package`
adding others. … So my default is to drop the requirement entirely."

### Without an opt-in, every adopter with published crates fails its pin move `##arg-42`

R6, the session. Bears on `thread@per-crate-changelogs@changelog-opt-in`. "Without opt-in, every
adopter with published crates and its own changelog convention would fail at its next pin move.
That goes against `goal@every-target-project-can-adopt-it`, and it would be a new ask, which
`issue@knowledge-architect@no-head-argues-what-becomes-an-ask` says no head can argue yet. … Reading
`publish` is not discovery by filename, and it reports a new published crate that has no changelog,
which a hand list would miss."

### The classes are Cargo's semver, and the grammar is the tool's convention `##arg-43`

R6, the session. Bears on `thread@per-crate-changelogs@changelog-grammar-compiled-in`.
"`design@check@nothing-of-a-project-is-compiled-in` lists 'Five things are compiled in, and none of
them describes a project'. This adds a sixth member. Argument: the classes are Cargo's semver,
which `goal@serves-one-kind-of-project` fixes. The grammar is the tool's convention for a kind of
document, in the same way the plans layout is."

### Both declarations add obligations and remove none `##arg-44`

R6, the session. Bears on `thread@per-crate-changelogs@citation-free-paths` and
`thread@per-crate-changelogs@changelog-entry-categories`. "`design@check@the-regime-has-no-opt-out`
admits only declarations that add obligations, and both do. The id lints keep reading the
citation-free files."

### "Rewritten in words" never means the backticks removed `##arg-45`

R6, the session. Bears on `thread@per-crate-changelogs@citation-free-paths` and
`thread@per-crate-changelogs@changelog-holds-no-citation`. "In the conversion, and in the repair a
citation-free finding names, 'plain text' means a rewrite in words or an angle-bracket placeholder.
It never means the token with its backticks removed. `design@agent-skills@plain-text-is-no-repair`
forbids that, and `reference-shape-outside-span` and `plain-word-lint` would report it."

### The path lint stays in citation-free files, its repair a rewrite in words `##arg-46`

R6, the session. Bears on `thread@per-crate-changelogs@citation-free-paths`. "The unanchored-path
lint stays in force in citation-free files, and there its repair is a rewrite in words. Measured:
one instance today, `path@knowledge-architect@.cargo/config.toml` at CHANGELOG.md:101. The cost accepted: a changelog cannot
print an existing path of the tree in backticks."

### A reference in a crate's changelog could resolve nowhere it is read `##arg-47`

R6, the session. Bears on `thread@per-crate-changelogs@axis-named-in-words`.
"`design@agent-skills@a-reference-claims-a-revisit` says a reference is owed where the text would
need revisiting. The answer: the file is read in a consumer's registry, where no reference resolves,
and a released section could not follow a rename of the axis anyway. The `Next release` section is
covered by the release review. The rival was a structured trigger field in the entry, checked against
the shipped axes. It is specific to this repository, so a generic grammar would carry a field that
only klarch uses."

### The writer of the assembled file loses nothing and writes over no incomplete model `##arg-48`

R6, the session. Bears on `thread@per-crate-changelogs@assembled-file-not-overwritten`. "The
assembled file opens with a generated header, as an `index.md` does. `--fix` refuses to write over a
file at that path that lacks the header, which serves
`design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`. `--fix` also refuses to
write the assembled file while any changelog holds a line the parse could not read, per
`design@check@fix-before-the-checks`. The file is committed, sits outside the walk by construction,
and joins `design@model@one-list-of-generated-files`."

### A consumer holds no manifest of the publisher `##arg-49`

R6, the session. Bears on `thread@per-crate-changelogs@category-order-in-preamble`. "A consumer's
`klarch changelog` has no manifest of the publisher, since no crate ships `knowledge-architect.toml`.
So the category order the command renders by is written into each crate's preamble by the checker,
and kept current like the rest of the preamble … With this, each crate's file is self-contained."

### The closure of `knowledge-architect` misses `gates` and `mutate` `##arg-50`

R6, the session. Bears on `thread@per-crate-changelogs@registry-changelog-command`. "The closure of
`knowledge-architect` misses gates and mutate. Measured: `path@cli@Cargo.toml` depends on neither.
So the command takes every package in the consumer's resolved dependency graph whose `repository`
equals the named package's, falling back to the named package plus `--package`. It runs `cargo
metadata --format-version 1 --locked --offline`. Assumed, to be measured in the work: without those
flags it may write Cargo.lock or reach the network."

### `cargo x changelog` had one job `##arg-51`

R6, the session. Bears on `thread@per-crate-changelogs@changelog-model-home`. "Its copy job was its
only job. Repairs: `design@knowledge-architect@xtask-crate`, the root CLAUDE.md Component table,
`## Git` point 1, and `path@xtask@docs/open-issues/the-reword-the-fold-and-the-merge-predicate-have-no-command.md`."

### The pin move runs the binary it pinned, which links the model `##arg-52`

R6, the session. Bears on `thread@per-crate-changelogs@registry-changelog-command`.
"`issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head` says that
question is raised whenever a new tool is placed. The argument here: the pin move runs the binary
the project already pinned, and that binary links the changelog model. The general rule stays that
issue's question."

### The head admits asking git about the branch's base `##arg-53`

R6, the session. Bears on `thread@per-crate-changelogs@no-history-beyond-the-branch`. "Its wording
admits asking git about the branch's base, which `issue@git@a-shallow-clone-shortens-the-range-commits-judges`
needs."

### The `rejected` entry records both arguments `##arg-54`

R6, the session. Bears on `thread@per-crate-changelogs@released-content-check`. "The no-history
argument defeats the comparison with the tag. A rival that compares only within the branch is
defeated by the rewording argument alone."

### Q1's default: the `cargo` crate in lockstep, the issue's trigger moved `##arg-55`

R6, the session. Bears on `thread@per-crate-changelogs@cargo-crate`. "Default: `cargo` takes the
workspace version like every other crate. The issue stays deferred, with its trigger moved to the
next new crate … Why: this discussion has not argued versioning at all. The rival, opening
per-crate versioning now, would bring the exact pins, the release skill and the lockstep branch of
#changelog-bump-check into this work."

### Q2's default: two levels, since the installed copies may cite what every project holds `##arg-56`

R6, the session. Bears on `thread@per-crate-changelogs@citation-free-paths`. "Those copies may cite
what every project holds: skills, the primer, `path@*@<path>`, `register@*@<register>`. A rule that refuses every
reference would refuse those too. … The rival keeps the rule total, refusing every reference, and
leaves the issue open. That costs a second declaration later that does nearly the same thing."

### A new crate never argues versioning, so the issue is a `todo` `##arg-57`

R7, the owner. Bears on `thread@per-crate-changelogs@cargo-crate`. The owner argued that keeping the
same trigger keeps the issue deferred indefinitely, since adding a crate never argues versioning in
the first place, and ruled that it becomes a `todo` issue at the top of the roadmap, to be discussed
at least before the next release.

### Installed text citing installed text is useful; the changelogs stay strict `##arg-58`

R7, the owner. Bears on `thread@per-crate-changelogs@citation-free-paths` and
`thread@per-crate-changelogs@changelog-holds-no-citation`. The owner approved the two levels, since
shipped text can cite other installed text and that is useful; considered allowing the changelogs to
cite installed text as well; and kept the strict reading for them, since a frozen released section
cannot follow such a citation.

### A milestone of five slices keeps a check's failure apart from the conversion's `##arg-59`

After R8, the session; approved by the owner at R9. Bears on every thread, through the
implementation sequence. "Each slice is one branch, one pull request and one merge, because the
work crosses six Components and each part can fail on its own. … The rival, one spec on one branch,
would put about six Components and a 1892-line conversion into one review. A failure in the checks
could then not be told apart from a failure in the conversion."
