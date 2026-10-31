# Rejected designs — core

The designs the Component core did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@core@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### A planned path legal in every document `##planned-path-in-every-document`

`live`. An issue entry could then name the file its fix would create. It lost because an issue
states what is missing rather than the shape of its fix, and a hypothetical file there is served by
the escape anchor. It is recorded because a doubt remains:
`tripwire@core@planned-path-outside-plans` watches for a file named as to be created outside the
plans directory, and reopens the scope with this alternative as the candidate.

### A hand-rolled gitignore matcher reading the root file alone, with the walk pruning what it covered `##hand-rolled-gitignore-matcher`

`live`. It needed no `git` on the path and no process per run, and its supported subset was chosen
against thaum's own file. It loses on three counts, each measured against the tree rather than
argued: a tracked live document left the walk AND the inverse assertion when one root ignore line
matched its bare name at any depth, reproduced with a one-line `.gitignore` over a copy of
`path@core@tests/projects/minimal/` and recorded as a defect before the reversal; a nested
`.gitignore` was not honoured at all, so a project needing one had to declare the path in the
manifest; and every pattern the matcher could not honour — a negation, `**`, `?`, a character class
— had to be refused by name, because dropping an ignore rule makes the walk read more than it should
and dropping a negation makes it read less, and both are silent. Git answers all three by
construction, and the walked-file count over thaum's tree was identical under the two.

### `check` and `commits` refusing a pipe on stdout `##check-refusing-a-pipe`

`live`. It was meant to stop `cargo klarch check | tail -n 10 && git commit`, which commits over a
failing tree because a shell without `pipefail` takes the exit status of `tail`. It was built and reviewed, and it
loses because it cannot stop that chain in a shell without `pipefail`, where a process inside a
pipeline does not set the pipeline's status; under the option, which the setup sets in the `claude`
harness's shell per `design@agent-skills@harness-shell-fails-on-a-pipe`, the chain stops whatever
the checker prints, and a refusal adds nothing. Reproduced with the refusal in place,
`bash -c 'cargo klarch check | tail -n 3 && echo commit'` printed the refusal, then `commit`, and
exited 0. The chained command still ran, now over a tree the checker had not judged at all. Two
further costs were measured. The test was `is_fifo` on stdout, and ksh93 builds its pipelines from
sockets, so `klarch check | tail` under ksh93 was not refused. And every caller that captures the
output had to give the child a regular file: `cargo x gates` and the test helpers of thaum's two
checker binaries; the runner's side, now the gates library's, is its own entry,
`rejected@gates@streams-through-one-file`. The merge is guarded either way by the range check of `design@core@a-commit-message-is-a-document`.

### Every git variable of the caller's environment removed, `GIT_INDEX_FILE` included `##every-git-variable-dropped`

`live`. Git would then always find the repository and its index from the project's directory. It
lost to a measurement with git 2.53.0, a pre-commit hook printing `env | grep '^GIT_'`: under
`commit -a` and `commit <paths>`, `GIT_INDEX_FILE` names a lock file holding the tree the commit
records, while `.git/index` still holds what was staged before, so `check --staged` run from the
hook would judge another tree than the commit's.

### A `commit` subcommand that runs the checks and then `git commit` `##commit-subcommand`

`live`. It would judge the tree and the message at the moment of committing. It loses on three
counts. It has to offer `git commit`'s interface: `-a`, `-m` and `-F`, the editor, pathspecs,
`--amend`, `-p`. A session can still call `git commit` directly, so it guards nothing a session does
not opt into. And judging a message before the commit exists repeats the reason the `commit-msg`
hook below lost: under an amend or a reword, HEAD is the commit being replaced.

### A `commit-msg` hook judging each message before the commit exists `##commit-msg-hook`

`live`. It repaired a message by an edit of the draft rather than by an amend, and cost 0.62 s per
commit against 1.6 s for `commits HEAD~1..HEAD` after it; the figures and the two refusals are in
the message of the change that removed the hook. It loses on three counts. The range check runs in
the gates and in CI before every merge, so the hook guarded nothing the merge does not. It judged a
message against HEAD as the parent, which under an amend or a reword is the commit being replaced: a
rebase rewording two messages that `commits` accepted was refused by it twice and needed
`--no-verify`. And it was per-clone configuration every clone had to install and every session had
to check.

### Selecting a subset of the checks with `--only`, one family per check `##only-flag-check-subset`

`live`. It let a reviewer run the checks of its own axis and let a run that read no rule text
resolve no release. It loses to the phases: the checks cross them, so a selection is honoured only
with a second bookkeeping saying which part of a check a stop withheld; every consumer was a
reviewer on a tree the gate already passes, where the selected and the full run print one verdict;
and resolving releases at the last phase gives every early stop the no-network property the
selection gave one run. What it bought was measured: a full run over thaum's tree against a
references-only one, with the GNU time command, and the figures are in the message of the change
that removed the selection. A full run of `cargo klarch check` over thaum's tree growing past a few
seconds is what would reopen it.

### A per-document phase gate, hiding only what an unread file could have defined `##per-document-phase-gate`

`live`. It would let a citation finding in one file print while another file is unreadable, at the
cost of tracing which entities the unread file could have defined and hiding only the findings that
name them, roughly three times the machinery of the whole-report stop. It lost on that cost, and
because the tree-level facts of phase 2 — a home that is not there, a refused declaration — have no
per-file scope to trace. It is the form the tripwire on the whole-report stop reopens with, which is
why it is here.

### A slug unique across the whole project, with the component named for the reader only `##project-wide-unique-slug`

`live`. It keeps one meaning per word everywhere and needs no lookup to resolve a reference. It
loses because it makes every component's vocabulary global: two components cannot each decide
something they call the same word, and the second one to want the word has to take a worse one.

### A reference with no component read as one inside its own component `##implicit-own-component-reference`

`live`. It would leave a pointer inside a component as short as it was before components existed,
and qualify only the crossings. It loses on what a reference has to carry by itself: the same text
would name different decisions depending on which file it sits in, so moving a document between
components would silently retarget every unqualified reference in it.

### A bare root-relative path form, with a targeted check on the collision cases `##bare-root-relative-path`

`live`. It keeps prose short and costs no migration. It loses on what was counted: the required
document set gives every component the same names, so a bare `docs/`-headed reference from inside a
component silently names the project's file — twelve candidate sites in one component alone, nine of
them wrong, counted by hand when the defect was recorded. A targeted check patrols the collision;
retiring the form makes it unrepresentable, and an anchored reference is what a fixed-string grep
can find.

### A fenced reference read as an illustration, the stance slug references took before the `@` grammar `##fenced-reference-as-illustration`

`live`. It let a document explaining the convention hold an example without inventing a slug. It
loses on consistency: path references were already live in a fence, so one kind had two stances, and
a sketch in a design document names its decisions as deliberately as it names its paths. The
illustration is a placeholder in angle brackets, which the tokenizer does not record.

### `#` as the reference separator, the incumbent of `` `<component>#<slug>` `` `##hash-reference-separator`

`live`. `design@core@an-entity-belongs-to-its-anchor` fixes the separator `@`. It costs no migration
of the slug references. It loses on collision: `#` is a Rust attribute opener and a markdown heading
marker, both of which sit inside code spans in thaum's tree, and `:` — the other candidate — is a
Rust path separator; `@` meets almost nothing in Rust and only email addresses in prose, which never
sit in backticks here. It entered the tree with the first migration to qualified references,
carrying no argument of its own.

### A configurable reference separator `##configurable-reference-separator`

`live`. `design@core@an-entity-belongs-to-its-anchor` fixes the separator `@`. A project could pick
the character its prose collides with least. It loses because every instruction and every skill
would be parameterised on it, and `@` already meets almost nothing in Rust and only email addresses
in prose, which never sit in backticks here.

### Paths without the kind prefix, keeping `` `<anchor>@<path>` `` beside `` `<kind>@<anchor>@<id>` `` `##path-without-kind-prefix`

`live`. It costs no migration of the path references. It loses on what it leaves in the scanner: two
grammars, told apart by segment count, and an unsupported-shape lint that keeps its heuristic
instead of becoming "unknown kind". Five characters at every path reference bought one tokenizer and
one candidate rule.

### A register reference with no anchor, `<kind>@<id>` for a register a project has only one instance of `##register-reference-without-anchor`

`live`. It is shorter for the common case. It loses because the extracted tool cannot know which
register is single-instance in a given project, and one three-part grammar serves every kind without
a special case in the resolver or in the instructions. Its first reason does not hold for a kind
whose namespace the harness fixes, which `design@core@harness-kinds-cited-without-anchor` cites with
no anchor; the second stands as the cost that head accepts.

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

### The `@` prefix reused as the root-relative spelling `##at-prefix-root-relative-path`

`live`. It is one character where the project's name is several. It loses on shape: the root is a
component and already has a spelling under the one grammar, so a second one puts two shapes on one
meaning. Its second reason, that it spends the prefix the census of the entry "Keeping the `@`
prefix as the escape" shows is needed for the generic and escape meanings, no longer holds:
`design@core@reserved-anchors` spells those meanings with the anchor words `elsewhere` and `*`, so
the prefix carries none of them.

### Widening the retired form's suffix set by one entry, `.rs` `##retired-form-rs-suffix`

`live`. It was the repair a defect proposed: relocating thaum's nine game-driving suites moved every
backticked Rust path across five of its knowledge documents, and the checker reported none of them,
because no suffix in the set covered them; the sweep had to be grep-driven. It loses because `design@core@every-path-names-its-anchor` retires the form that carried the set: a path reference names its kind and its anchor and is
checked whatever its suffix, so no set is left to widen and no suffix can be missing from one.

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
cause of the four defects in thaum's tree that `design@core@grammars-not-prefixes` lists.

### `--lines` on thaum's interpretation index, a temporary copy carrying each citation's line numbers `##lines-flag-on-interpretation-index`

`live`. Refuted by a census over the file's whole history rather than by argument:
`git log -p -- docs/rules/interpretations/index.md` matches no line-numbered citing row, so in every
commit the index has ever had, the flag's output was never one of them. Every generated index gates
on data that does not move when unrelated prose shifts a line — the interpretation index carries one
row per entry, with no line and no citing list at all, and the rule index names its citing files at
file level — so per-line data stood against the argument for the file it would have sat in. What it
would have bought is answered instead by filtering `cargo klarch model` on the rule number.

### `cargo klarch index` prints the diff it would apply, and `--write` applies it `##index-diff-with-write-flag`

`live`. This is the `cargo fmt` / `cargo fmt --check` shape, and it lost to a prior-art survey
rather than to reasoning. `fmt`, `gofmt` and `prettier` each build the check into the generator, and
the discriminator is that **none of them had a separate verifier to build it into anything else**.
Here the `generated` check of `cargo klarch check` is a gate that already names the first differing
line, so the split those tools could not make is already made, and a second command answering the
same question in its own format would leave nothing to say which of the two was right.

### Every string literal outside the checker read as prose, bound to a name or not `##every-literal-read-as-prose`

`live`. The binding rule of `design@core@grammars-not-prefixes` decides against it. Proposed to
remove the three special cases the binding rule costs the extractor, on a measurement that found no
bound literal outside the checker carrying rule-shaped content. The measurement was wrong: built,
the checker reported six in three crates, an address, formatted figures a display test expects and a
line of tool output a parser is fed, and the class recurs wherever a test displays a three-digit
float.

### A fixture marker, a macro whose token tree the checker reads as data `##fixture-marker-macro`

`live`. It frees every fixture and needs a convention at each one, which the rule of `design@core@checker-source-literals-are-data`, by the directory a file sits in, does not. It is the recorded fallback should a literal outside the tool
ever need rule-shaped bytes as data where no binding can hold it.

### Moving the tool's unit tests under `path@core@tests/` and excluding the directory `##unit-tests-moved-under-tests`

`live`. Integration tests reach only public items. Measured: 31 files carry a test module, 349
tests, most over private functions, so the move makes those public or drops the tests.

### Literals as data in every `cfg(test)` module of every crate `##literals-as-data-in-every-crate`

`live`. Zero members outside the tool today, and the owner's ruling is that a rule cited in another
crate's unit test stays checked.

### Reading no string literal anywhere `##no-string-literal-read`

`live`. Measured: 17 assertion messages in the tests of thaum's `thaum-testing` crate cite a rule
behind a marker and are checked today. Each would become text nothing reads.

### The binary finding the checker's directories at run time, from the tree or its manifest, in place of refusing a binary built elsewhere `##run-time-checker-directory-discovery`

`live`. Reproduced on thaum's repository: a binary built from a second checkout into the first one's
target directory, run in the first, printed a verdict string changed only in the second, so the
other checkout's code runs and not only its compiled paths; correct paths would still judge the tree
with the wrong code. The exemption half of the argument is the entry "A manifest row naming the
exempt directory".

### A manifest row naming the exempt directory `##exempt-directory-manifest-row`

`live`. A row can be pointed at any directory, and the tree declaring it decides what conformance
means, which `design@core@the-regime-has-no-opt-out` refuses. The compiled path can name only the
checker's own source.

### `additional-trackers` kept as a file list `##additional-trackers-file-list`

`live`. It named each tracker outside every component directly, needed no second kind of anchor, and
every check that read it worked. It loses because a file in that list had no anchor: nothing in the
project could cite what was open there, and the register thaum's rules directory holds could not be
named at all. The whole of this tool is about citing, so a recorded thing with no name is the one
shape it may not have.

### A `theme` metadata key in place of group subdirectories `##theme-metadata-key`

`live`. It keeps every entry in one flat directory and regroups by editing one line. It loses on two
counts: a subdirectory is visible to `ls` and to a listing without parsing any file, and a group
that is not part of an entry's id makes regrouping a `git mv` that breaks no reference — which a
metadata key would also have to promise, and could not, without a second rule saying the key is not
part of the id.

### The index introduction as a string in the manifest `##index-introduction-in-manifest`

`live`. It would make the whole of a generated index generated, with nothing hand-written beside it.
It loses because the introduction is prose about the project and belongs in markdown, and because a
hand-written README is also what keeps the instance directory in git: an empty directory is not a
thing git tracks, so a register with no entries would have no home at all.

### An optional index, generated only where a project asks for one `##optional-index`

`live`. It was argued on churn: a generated file in every instance is a file that goes stale and
fails the gate. It loses once the README holds the directory open, because the rows change only on
create, delete, retitle, regroup and a metadata change — not on a wording edit — so the churn the
argument feared does not happen. An optional index is also an opt-out, which
`design@core@the-regime-has-no-opt-out` refuses.

### A summary column in a file register's index `##index-summary-column`

`live`. It would let a reader take in what an instance holds without opening a file, which is the
whole point of a listing. The summary loses on churn: it changes on every wording edit of every
entry, so an index that today is regenerated on create, delete, retitle, regroup and a metadata
change would go stale on each one and fail the gate. It is answered by a command instead,
`show <ref>` for the body.

### A last-change date in a file register's index `##index-last-change-date`

`live`. It would let a reader take in what an instance holds without opening a file, which is the
whole point of a listing. The date loses outright to `design@core@generated-files-are-pure`: the
last change is git's, not the walked tree's, so a generator reading it would rewrite the file on a
rebase and report a file stale that nobody had touched. It is answered by a command instead,
`issues` for the dates.

### Per-instance register options declared at the root, in the project's own instance `##instance-options-at-root`

`live`. It puts every option in one place beside the manifest, with no second configuration file to
find. It loses on where the file would sit: the root component's own issue directory would carry the
groups of every other component's issue instance, which is the one-home failure written into the
layout.

### One link shape, the plain inline form with a spaceless target, so that the scanner stays one pattern `##single-link-shape`

`live`. A row written in another shape was not silent: an angle-bracketed target was read with its
brackets and an existing file was reported as missing, which a test in a navigation README showed. A
directory home's README written with link definitions would have been reported as linking none of
its subdocuments. Both are wrong findings, where `design@core@links-are-navigation-rows` had judged the gap to be a silence.

### Refusing to load a manifest whose register declaration is wrong `##refusing-a-wrong-declaration`

`live`. A declaration the tool cannot act on is a could-not-run by `design@core@exit-code-ladder`,
and exiting 2 is what a manifest the grammar cannot parse does, a key of `[project]` it does not
know included. It loses for the case where the tool CAN act: a bad `dir` on a built-in has a
compiled default to fall back on, so refusing the whole run would report nothing at all about the
tree — and nothing at all is what a session reads as conformance. An unknown key keeps the refusal
because there is no default to fall back on.

### An open set of issue kinds, the label read off each entry as written `##open-issue-kind-set`

`live`. It was the incumbent: the listing derived an entry's label from the tag its own title
carried rather than matching a list, because an entry written with a kind nobody anticipated is
intended, and matching against a list made such an entry fall through to a label that is also a real
kind. It loses once the kind decides which subsections an entry owes: an unanticipated kind then
owes the wrong three, silently, and no reader can tell. A closed list makes an unknown kind a
finding naming the list and an addition a reviewed manifest diff; the failure case the open set was
written against — a wrong label — cannot occur once an unknown kind is refused.

### A model generic over the extension's observation type, `Model<E>` with the extension's observations in each document's list `##model-generic-over-extension`

`live`. It scans each document once, and the extension's observations are typed and sit beside the
core's. It loses because the type parameter reaches every type and every check signature that
touches a document, to carry data the core never reads: at the split, 18 of the 28 Rust files of the
core's library and binary named `Model`, `Document`, `Observation` or `Located` (`git grep -lwE`).
Two extensions would also need a hand-written composition of their two types.

### A type-erased slot per document, a map from extension name to a boxed value that each extension downcasts `##type-erased-document-slot`

`live`. It keeps one model and needs no type parameter. It loses because the extension keeps the
same data outside the core with its type checked at compile time, so the slot adds a downcast that
can fail at run time and gains nothing.

### A namespace for an extension's tables, `[extensions.<name>]` `##extension-table-namespace`

`live`. It shows in the manifest itself which tables are an extension's. It loses because the
refusal of an unclaimed table already reports a table nobody owns, and because it changes the
manifest format of every project and every mock for no check that reads the difference.

### The sentinels `"fixture"` and `"self"` accepted wherever a manifest writes them `##sentinels-accepted-everywhere`

`live`. It made a fixture manifest and this repository's own pin read the same in every copy, needed
no build to confirm anything, and worked wherever an extension keeps its mock projects. It loses
because a value open to every project is an opt-out: an agent stuck on the refusal could write
`"fixture"` in a real manifest, and the pin the key exists for would be silently off. Recorded
because doubt remains on `design@core@checked-sentinel-values`: its review found two layouts that confirmed a sentinel on a
real project, a root that is an extension's crate and a registry inside the tree, and closed each by
its own rule; a third would argue for this one again.

### Every path-shaped span reported as an unanchored pointer, whatever its first segment names `##every-path-shaped-span-reported`

`live`. It let no backticked pointer pass unregistered, a pointer written after its target's
directory left the tree included, and it needed no read of the tree. It loses on a census of a
project new to the tool, in the commit that rewrote `design@core@every-path-names-its-anchor`: nearly all its findings there
were another tool's notation (fixture paths of tests, paths on the user's machine, example paths in
a changelog), each to be rewritten or anchored at `elsewhere` before the project passed, and the one
real pointer, a stale one, is reported under `design@core@every-path-names-its-anchor` too.

### The ignore rules asked whether an unanchored path's first segment is ignored, and an ignored one reported `##ignored-first-segment-reported`

`live`. It kept a pointer into a build output a finding. It loses on three reproductions made by the
review of the change that proposed it: `git check-ignore` refuses a spelling through a tracked
symlink, so one backticked span through a symlink made `check` and `commits` exit 2 instead of
giving a verdict; a whitelist `.gitignore`, `/*` then its exceptions, covers every root name, so
`application/json` and `origin/main` were findings again; and the batch grows with the spans times
the places each is read from, which on a synthetic tree of 60 components and 20 000 distinct spans
made the run take about nine seconds, against half a second when the listing alone answers.

### The retired slug reference reported whatever it names `##retired-slug-reference-always-reported`

`live`. It reported a copied form whose entry has left the tree and whose word is no anchor, which `design@core@retired-slug-reference-reported` leaves silent. It loses because the form was this grammar's alone: in a project that
never used it, every match is that project's own notation, such as an issue number `#123` or a
preprocessor directive `#include`, and in the census of the commit "Report an unanchored path or a retired slug form only where it
names something of this project", its one finding was an issue number.

### The interpretation entry number kept as a retired form permanently, a bare `R` followed by digits reported wherever prose holds it `##permanent-interpretation-number-form`

`live`. It was kept because a retired form must stay visible or the migration is unfinishable, and
it was read in every prose region, Rust comments included. It loses because the migration is
finished, the tree holds no such span, and the number is thaum's own register's: kept, the lint
would have had to move into thaum's extension when the core was split from the rules half. The
commit history holds the form in 65 of the 500 messages before that split, counted with the lint's
own pattern; the slug reference, which stays a finding wherever its id is still an entry, is in 64
of them, so exposure in history does not separate the two.

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

### CLAUDE.md required in every component, whatever the project declares `##claude-md-in-every-component`

`live`. Its argument was that what a component carries is compiled in, because a project free to
declare its own set is checked against nothing, and CLAUDE.md is where a component's contracts for a
developer are routed, so requiring it guarantees them a home. It loses for the agent document alone:
the declaration that removes it removes nothing else, so every other document is still compiled in
and checked. Doubt remains on the second half of the argument, and
`issue@core@a-home-for-developer-contracts-outside-agent-configuration` records it.

### The implementation's module tree as the public API, with only its visibility narrowed `##module-tree-as-public-api`

`live`. A consumer would learn the internal layout: thaum's rules extension at thaum's e98e296 named
13 of the 16 modules public at 465ff35: the distinct names `git grep -o "documentation::[a-z_]*"`
prints over its tools/rules-corpus, the function `component_dir` aside. Moving an item between files
would be a library break, and the public surface would be computed from the whole module tree rather
than read in one file.

### `#[non_exhaustive]` on every public enum expected to grow `##non-exhaustive-on-growing-enums`

`live`. It assumed the attribute prevents breakage. It relabels a break that hits only the consumers
who opted into an exhaustive match, and removes that option from every consumer. Under 0.x it buys
no version number, because major and minor both bump 0.MINOR.

### A struct literal of `Inputs` in each caller, accepting a break per new input `##inputs-struct-literal`

`live`. `design@core@inputs-are-gathered` gathers the inputs instead. The checker surface of v0.1
added two inputs and broke all five literals in thaum's rules extension; each later input would do
the same.

### The items only the core's own tests use kept public under `#[doc(hidden)]` `##doc-hidden-test-items`

`live`. It publishes a second contract that nothing states, beside the one the facade states.

### The items only the core's own tests use gated behind a `testing` feature `##testing-feature-for-test-items`

`live`. A unit-test module inside the crate reaches them with no feature and no second surface, and
the crate declares no feature, so this one would exist for the core's own tests alone.

### A default body that does nothing on every hook added to the extension traits `##no-op-hook-defaults`

`live`. A judging hook defaulted to nothing turns a compile error into a check that silently does
not run, with exit 0.

### The primer delivered by a session-start hook that prints it `##session-start-hook-primer`

`live`. `design@core@owned-namespace-check` delivers the primer instead, by an import line in the
root CLAUDE.md. A hook needs the installer to edit the user's settings file, which belongs to the
user, and it needs a shell. It is kept because a doubt remains: the import is verified for the main
session and a subagent of the `claude` harness, and another context or harness might not follow it.

### The binary named knowledge-architect, after its package `##binary-named-after-package`

`live`. The binary's name is the default of the declared command, which a project types at every
run, and the owner wanted it short to type. Renaming the binary is a change to the command-line
surface every installing project scripts against, so the alternative will be raised again whenever
the name is questioned.

### An install record file listing what the installer wrote `##install-record-file`

`live`. The checker would read a committed file of its own format to know which installed files are
its own. It is one more committed file, which can itself be edited, while the prefix makes ownership
readable from a path alone with no record and no history.

### A plans directory in every component `##plans-directory-per-component`

`live`. It lets a large project keep each component's plans beside it. It loses on the premises `design@core@plans-at-root` states: most work spans components, so its plans sit at the root anyway; no project should hold
enough open plans for a split to help; and plans designed before another is built break each other,
which separate directories hide from each other. A further cost, which the owner did not argue:
every component would owe a plans home that holds nothing.

### Citing an item of another plan `##cross-plan-item-citation`

`live`. It lets a plan name the exact thread of another plan it depends on. It loses because
whole-document citations already carry the dependency while few plans are open at once, and because
item citations across plans would force a design revisit of every citing plan, item by item, at the
moment a plan is retired, a moment the owner judged might not be the best for it.

### The staged tree as `git ls-files -s` lists it `##ls-files-staged-tree`

`live`. It lists an entry staged with `git add -N` at stage 0 with the empty blob, the line a staged
empty file also gives, and `git commit` records no such entry. Measured on git 2.43.0: `ls-files -s`
listed both, `git diff --cached` and `git write-tree` the empty file alone.

### A third `extension::Tree` variant for the index `##third-tree-variant-for-index`

`live`. The index is read through the same git objects as a commit, so the third arm reads what the
second reads. It is what `design@core@ne-minimal` would ask of a new tree, and `design@core@an-extension-reads-a-snapshot` records the cost of not taking it: an extension's commit-mode handling runs under `check --staged` without
the extension deciding anything.

### `index --staged` writing the staged rows into the working-tree file `##index-staged-writes-working-file`

`live`. After it, plain `check` fails, since the working-tree file no longer matches the working
tree, and `check --staged` fails until the file is staged.

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
keeps, against `goal@knowledge-architect@adoption-is-easy`.

### `home` as the word of the kind naming a register's home `##home-kind-word`

`live`. It matched the documents' phrase "design home", and cost the same refusal of an anchor name.
The owner chose `register`.

### A `path` that resolves to either shape of a heading register's home `##path-to-either-home-shape`

`live`. A path claims its target's kind by its trailing slash, per
`design@core@trailing-slash-claims-directory`; one that matches either shape asserts no claim and
puts a special case into path resolution.

### A `path` reference to a register's home accepted beside the `register` form `##path-to-home-beside-register-form`

`live`. Under the generic anchor it accepted every shape of a component register's home, whether or
not any component used that shape yet, so a text that must hold in every project could name both
shapes and pass. The `register` form serves that need with one span. Kept legal, the path form keeps
being written, and a home that changes shape dangles every one of them.

### The paragraphs of a compiled document, with `show` printing each one that cites a head `##paragraphs-with-show-printing-them`

`live`. It needed no migration of any project. It delivers a rejected design only to a session that
runs `show` on the right head, and the case that prompted the register is a session that did not
read the right head; it also leaves nothing for a plan item, a review finding or a commit message to
cite, since a paragraph has no slug.

### The kind named `rejected-alternative` `##kind-named-rejected-alternative`

`live`. It names what most entries are. The span is 12 characters longer, `-alternative`, in a
reference that never crosses a line break, the slug already names the design, and the owner found
a register of alternatives alone a bit odd.

### The home keeping the file name of the rejected-alternatives document `##home-keeping-the-old-file-name`

`live`. It would leave the file a reader already knows. Every pointer to the file changed anyway,
since a `path` span to a register's home is a finding whose repair is the `register` form, so it
saves no edit, and it leaves a home named otherwise than its kind; the owner judged the new name to
read better and to avoid a confusion point.

### A validity marker the checker reads `##validity-marker-checked`

`live`. It would make `void` mechanical. A heading register has no field for it, per
`design@core@registers-are-declared`, so it needs a syntax of its own, and when the register was
built no entry of this repository was marked `void`, so it would protect none.

### A `--fix` that converts the paragraphs of a rejected-alternatives document into `rejected` entries `##fix-converting-old-entries`

`live`. It would make the migration one command. Converting an entry chooses its slug and its
citing heads, and a fix never makes a choice, per `design@core@fix-scope`.

### A goal citing a `rejected` entry to make it reachable `##goal-citing-a-rejected-entry`

`live`. It would keep reachable an entry whose design lost to a goal alone. A goal does not change
when a design it did not keep goes `void`, so the citation fails the revisit test of the installed
reference rule and is decoration, and a goal is written only on the owner's word.

### A `rejected` entry made reachable by its own citation of a goal `##entry-reachable-by-citing-a-goal`

`live`. It keeps reachable an entry whose design lost to a goal alone, and its citation passes the
revisit test. The owner judged that only a design entry
should count, and that a head of absence is the right home where nothing is built; it is the
candidate if heads of absence prove to be written only to satisfy the check.

### No check of uncited `rejected` entries `##no-orphan-check`

`live`. It would leave the citations to review. An entry that no head cites is the case the
register exists to end: a design session reads the heads, and an uncited entry reaches it only
through `show`.

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

### A listing in `show` of the plain words that are an entry's id, instead of a finding `##plain-word-mention`

`live`. `cargo klarch show` would list where an entry's id is written as a plain word, and no
check would report it. Its premise was the noise of the first census, 713 plain hits in thaum and
none meant as a pointer in the sample. The second census defeated it in thaum, where it sorted the
hits: every plain hit that was no pointer came from a slug naming a subject, which the rename repairs, and the rest were pointers
written in words, which a listing leaves unchecked. Both censuses are kept in the spec of the
design discussion, `path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`. It is the first candidate where the owner rules
a finding of the plain-word lint to be on a word that is neither a pointer nor a subject slug.

### Plan argument items exempt from the two-segment grammar `##plan-items-exempt-from-the-grammar`

`live`. An item of a plan document would keep a one-segment id, numbered `a1`, `a2`, since an item
is cited from inside its own plan only, so no lint outside it would meet the word. It loses because
it adds an exception to the grammar every project's documents follow, justified by nothing but the
item's scope, while numbering the items `arg-1`, `arg-2` costs no capability.

### No finding on a backticked bare id, only a listing `##bare-id-listing-only`

`live`. A backticked span that is exactly a defined id would be listed by `show` and reported by
nothing. It loses on the census of the design discussion, kept in its spec,
`path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`: with ids of two segments and plan items
read inside their plan, the bare ids it found were pointers, one here and about 11 in thaum, which a
listing leaves silent until a reviewer reads them, against 3 in thaum that were not.

### A confirmation prompt before `init` writes `##init-asks-for-confirmation`

`live`. It guards against no loss: `init` refuses before its first write and overwrites no
hand-written file outside the installer's namespace, which every install rewrites. It blocks an agent, whose shell has no terminal, or is answered by a `--yes`
every caller passes. `cargo init` asks nothing, measured with cargo 1.98.0.

### `init` running `git init` in any directory outside a worktree, as `cargo init` does `##init-git-init-anywhere-outside-a-worktree`

`live`. In a directory that already holds files, the files there become untracked beside the
project's, as `git status --short` after `cargo init` shows, so `git clean`, the obvious cleanup of
a mistaken run, deletes them too. The owner chose a refusal there over following `cargo init`.

### `init` refused wherever PATH is outside a git repository, or not its root `##init-refused-outside-a-repository-root`

`live`. A project in a subdirectory of a repository passes the check, so a refusal there refuses a
legal layout, and a new or empty directory outside every repository loses nothing to `git init`.
There is still some doubt that the kept rule spares every mistaken run a messy cleanup: the
tripwire `tripwire@core@init-cleanup-beyond-the-listed-files` watches it.

### A hook through which an extension contributes its manifest tables to `init` `##init-extension-hook`

`live`. It would add a hook to the extension interface for a case the check already reports with
its repair, and a project with an extension would not need to run `init`, on the owner's word.

### An option of `check` and `commits` that prints the verdict alone `##verdict-only-output`

`live`. It was proposed so that a session that wants only the verdict line has no reason to pipe
the checker's output and chain on the pipe. It removes that motive for a session that uses it, and
stops no pipe: a session that pipes anyway still chains on the filter's status in a shell without
`pipefail`. The shell option stops the chain for every command a session runs.

### Clusters rooted at a goal, holding the heads that serve it `##goal-rooted-clusters`

`live`. Rooted at the heads citing a goal directly, at a goal no other goal refines, or at any goal
of its chain, they reached at most 342 of this repository's 677 entries at ed9dd60 and 208 of the
801 of thaum at its commit edd2cb5d,
since most heads cite another head rather than a goal; followed through the citations between heads
they merged into one cluster of 466 to 480 entries. The scripts the message of the commit
"[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds took the counts. It is kept here because a doubt remains: a goal is the owner's own
grouping, and a project whose heads all cite their goals would reach every entry.

### Goals bringing into a cluster the issues and tripwires that cite them `##goals-pull-their-citers`

`live`. A goal of the root is cited by heads of many subjects, so each of its citers joined every
such cluster: the reads rose from 839 to 1323 for 663 entries over this repository at ed9dd60, counted with the
scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds.

### The record search's agents filled to the bound by first-fit packing `##packing-to-the-bound`

`live`. First-fit-decreasing alone fills each agent to the bound, a median of 24,737 words at a
bound of 25,000 over this repository at ed9dd60, so one agent carries the full bound while another
carries a remnant.

### A rule that citations run one way, from goals down to issues, checked by the checker `##citation-direction-checked`

`live`. Clustering reads an edge either way, so the rule buys only acyclicity, which serves a
transitive membership that merges the clusters. It would fail 287 edges of this repository at ed9dd60 and
591 of thaum's at edd2cb5d, a design head citing its `rejected` entries and a goal refining another among them, as
counted with the scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds.

### A check failing the tree on a subject whose cluster is above the bound `##split-enforced-by-a-check`

`live`. The bound rests on the loads agents read with no strain reported, not on a measurement of
recall, so a gate on it would fail a tree on a number with no basis; the notice of `klarch worklist`
and a split as a fix of its own carry the incentive instead.

### A bound on the number of entries one search agent reads `##bound-in-entries`

`live`. An entry's mean size ranged from 0.5 KB, a `rejected` entry, to 2.9 KB, an issue, over this
repository at ed9dd60, so a count of entries bounds an agent's load poorly; the bound of 60 standing
entries rested on their mean size in thaum, 1.6 KB, and on no measurement of recall.

### One residual group for the whole project `##one-residual-group`

`live`. The entries no cluster holds are of shapes the reference rule admits, issues about code with
no head among them, so the group grows with the project: 14 entries here at ed9dd60, 55 in thaum at
its commit edd2cb5d, counted with the scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds.

### A transcript reader that refuses a line type or a value it does not know `##strict-transcript-reader`

`live`. Over every transcript of the machine the reader was designed on, Claude Code 2.1.306 to
2.1.310, each version added a line type, an attachment type or a key with the feature it shipped,
and a parser that refused the unknown would have failed on each; a third-party parser,
claude-code-log, printed a warning on every line of the new types, its issue 116 reporting that
"Claude Code 2.1.x emits a few new top-level JSONL types" it did not recognise. The window's types are re-taken with the command of
`design@core@transcript-format-tolerated-and-reported`. The harness documents the
format as internal and liable to change between versions.

### An assembly of the graded entries that reads the model again when `worklist --verify` runs `##gather-reads-the-model`

`live`. It would have changed the command line, by `--verify` reading a model or by a third mode of
`worklist`. Reading a model in `--verify` lets a tree that gained a finding of the first three
phases since `--out` block a check of reports that no longer depend on the tree; a third mode adds a
command step to every grounding. Its one gain, an inbound list current to the minute, is lost only
for the minutes between `--out` and `--verify`.

### A slug test of the design and `rejected` entries of one named anchor `##slug-test-of-one-anchor`

`live`. It would have changed the command line `id` takes. The anchor that will own a design
thread's decision is often not settled when the thread is named, so the test would need the
answer it is asked for; listing every kind and anchor answers it whatever the owner is.
