# The core checker: design

Recorded intent for this tool: how it is built internally, and why. Present tense, each decision
carrying a slug anchor, cited from elsewhere with `core` as its component. The designs this
Component did not keep are `register@core@rejected`, each cited by the heads whose argument covers
it, and `skill@knowledge-architect-decision-recording@head-shape` owns the shape.

**What belongs here:** a decision that does **not** survive deleting this tool. How it works and
how the pieces inside divide the work belongs here.

What otherwise shapes it is the module documentation at the top of each file under
`path@core@src/`.

The heads are grouped by subject. A group heading carries no decision; each decision is one
level-three head below it.

## 1. The shape of a run

### Nothing about a project is compiled into the tool, and the manifest declares every list a check reads `##nothing-of-a-project-is-compiled-in`

Every list a check reads comes from the project's `knowledge-architect.toml`: the components, the
locations, the registers, the walk's exclusions. The manifest is also the marker that makes a
directory a project root. So one binary checks any project, and every mock project under
`path@core@tests/projects/`, with no special case in the code. A path that must not be
checked is declared there, in one place, with its reason beside it.

The constraint derives from `goal@knowledge-architect@any-project-can-adopt-it`. A list
compiled in for one project is a list every other project is checked against. It also keeps the
tests honest: a mock runs the same code path as a real tree, so a test over a mock tests the
code that checks the real tree.

Five things are compiled in, and none of them describes a project:

- **What the word _component_ means**: the documents a component carries and the five built-in
  component registers, per `design@core@components-carry-the-same-documents`, and the starting text
  `init` and `new-component` write in each document and home: a title and a line on what it
  holds, or that it holds no entry yet, with no entry, no reference and no link, so it passes every check in any project, and no
  goal, which is the owner's to state, per `design@agent-skills@goals-change-through-goal-setting`;
  the root CLAUDE.md is its title and the primer's import line, since what every root would say
  alike is the primer's, per `design@agent-skills@primer-limit`.
- **Where plan documents live**: the anchor `plans` at docs/plans/ of the root and its two
  registers, per `design@core@plans-dir-fixed`. It is the tool's convention for a kind of
  document every project has, not a fact about one project.
- **The directory of each Component of the tool's own source**, so that its string literals are
  read as data, per `design@core@checker-source-literals-are-data`.
- **Each crate's own directory and package name**, so that a binary built from another checkout
  is refused, per `design@core@a-foreign-build-is-refused`.
- **The agent harnesses it knows, and the installer's namespace in a project's agent
  configuration**, per `design@core@agents-table` and `design@core@owned-namespace-check`: what
  this tool writes, not what a project holds; and what a harness writes that the tool reads, the
  layout of Claude Code's transcripts and the values its reader knows, per
  `design@core@transcript-format-tolerated-and-reported`, which describe the harness, not a
  project. The files `init --rust` writes are of the same kind:
  the setup skill's snippets, per `design@agent-skills@rust-templates-one-home`, with the version
  and the checkout variable derived from the project's name filled in.

### The model is built once, and every check is a pure function over it `##model-then-checks`

A check never reads a file, spawns a process, or knows how the walk works. Anything a check cannot
fetch for itself is resolved by the caller and handed in: the generated files as committed, the
per-instance `register.toml` files, one listing of what exists and which of those paths are
directories, the files the walk does not cover, and git's answer for every path spelling a
reference in the run could ask about. An extension's checks read, besides these, what the extension's
own preparation read for the tree, per `design@core@an-extension-plugs-in-through-phased-hooks`.
The phases that precede the checks are pure over the same inputs, per
`design@core@phases-gate-the-report`, and an extension prepares only once the last phase is
reached.

Four things follow, and the rival in which each check reads the tree for itself pays for each of them.

**The single walk is a property of the design rather than of anyone's care.** One run reads each
live document once and parses it once. The core scans each line once; an extension scans the same
parse again, as often as its checks need, per `design@core@an-extension-builds-its-own-model`. That rival, as built in thaum, performed four tree
walks and 311 file reads over thaum's 104 documents per run, then spawned its citation checker twice more to
do it again.

**A check is tested against a model assembled in memory**, so most tests need no fixture on disk at
all, which keeps a planted-defect corpus cheap to build.

**No check writes to the tree; `check --fix` writes before the model its checks read is built**,
per `design@core@fix-before-the-checks`. Every generated index is rendered into a `String` and
compared. A check that regenerates by writing the file, comparing,
and writing the old bytes back mutates the tree it is checking, and a history replay over it
needs a discarding checkout between commits.

**A check whose subject is filesystem state is the one exception, and it says so where it
lives.** Whether a vendored corpus matches its pin, whether an archive has provenance, whether a
needed release is present: there is no model to hand such a check, so an extension reads that
state while it prepares a checkout, per `design@core@an-extension-plugs-in-through-phased-hooks`.
That is a different thing from a check over documents, which stays pure. A check over another
document than the model's, such as a changelog, is pure too: the extension hands it the text and
the releases it names.

### A finding is classified by the phase that produces it, the report stops at the first non-empty phase, and every check runs at the last `##phases-gate-the-report`

Findings are not independent. Some say what the model could not read or resolve, and every
finding computed from that model afterwards is then unreliable in both directions, missing and
false. A list that mixes the two advertises a completeness it does not have. So a run is four
phases, each of the first three building one input of the next:

| phase | produced by | what it reports |
| ----- | ----------- | --------------- |
| 1 | `Manifest::parse`, and each extension's resolution of its tables | a declaration the tool refused, which is then absent from the configuration |
| 2 | `check::tree`, over the core's declared paths, the paths each extension declares and every document's parse, and `check::agents`, over the installed agent configuration | a file the walk could not read or refused, a tracked-and-ignored file, an anchor or a register home that is not there, a declared path that does not exist, a home a walk row keeps out, an installed agent file missing, differing or unshipped, a root CLAUDE.md that does not import a shipped primer, a heading markdown reads and the checker does not, per `design@core@headings-open-with-hash-marks` |
| 3 | `Entities::build` | a slug or an entry id where none may sit, or defined twice; a level-two heading of a section home with no slug; a skill's or an agent's name a reference cannot spell, or that its frontmatter contradicts |
| 4 | every check, the core's and each extension's | everything computed over a complete model |

A finding is classified by **the place it is produced**, never by a label at its site, so a
finding added to a building step is gated because of where it is raised, and nothing in the
last phase can undermine anything, because no check reads another check's findings. The run
stops at the first phase that produced anything, prints those findings, and opens its summary
with the phase, the count and the phases not judged, so a stop is never mistaken for a pass;
a clean run names every check it performed. The whole report stops: the per-document form,
hiding only what an unread file could have defined, is three times the machinery for facts
that mostly have no per-file scope, and `register@core@tripwire` guards that choice.

**An extension prepares its tree when the last phase is reached**, so a run that stops earlier
resolves no release and fetches nothing, whatever it would have judged. **A writer refuses over an incomplete model**: `index`,
`new-component` and a writing command of an extension, such as thaum's `rules bump`, run the first
three phases before touching anything and exit 2 naming the
phase, since an index generated over such a model lists rows nobody asked for. `init` writes where
no model exists yet: its guarantee is that it judges every destination before its first write, per
`design@core@init-asks-no-confirmation`, and it writes the indexes, as `check --fix` does, only
once the three phases find nothing over what it wrote. `check --fix` writes
no generated file over such a model either; it prints the stopped report and exits 1, having
repaired before that gate only the installed files, whose bytes do not depend on the model, per
`design@core@fix-before-the-checks`. `commits` judges
no message's references against a commit whose tree stops early, and says so, since a message
judged against an incomplete entity table is judged against nothing. The scan for citations of the
range by SHA needs no entity table, so it still reads that message, per
`design@core@branch-shas-are-refused`.

**Nothing selects a subset of the checks.** The checks cross the phases — what `registers`
asserts sits in phases 2 and 4, what `references` asserts in 3 and 4 — so a selection could be
honoured only with a second bookkeeping saying which part of a check a stop withheld. The
selection, and what it bought, is `rejected@core@only-flag-check-subset`.

**One producer chain, and a rule for a second.** The manifest feeds the walk, the walk the
model, the model the table, and every check consumes the table. A future check whose findings
another check reads would be a second chain, and nothing prevents one: such a check is a phase,
not a check of the last one, and it goes before its consumers. A tripwire names the event.

The design this decision did not keep: `rejected@core@per-document-phase-gate`.

### A finding names a cause only where the repair depends on which cause holds `##finding-names-the-repair`

A finding's action names the repair. It names a cause only when several causes can produce the
finding and each needs a different repair; the finding then tells them apart by naming each case
with its repair, as a dangling reference does for an unknown kind, an unknown anchor and an
undefined id. Where one repair is correct for every cause, the finding names that repair and no
cause. A cause adds nothing the reader needs to act, and it can be wrong: a stale generated index
is left by a hand edit, by an upgrade of the checker that changes the generator's bytes, and by a
branch that changes the generator, and a finding that names one of them misleads the reader in the
other two cases. The test is checkable per finding: list the causes that can produce it, and ask
whether one repair is correct for all of them. The rival condition, that a finding names no cause
where its repair is a safe fix under `design@core@safe-fix-definition`, lost: the repair of a
generated file that fails in every earlier commit of a branch is a history edit, which is no safe
fix, and it is the same repair whatever left the file out of date. How a repair is
carried out safely is the project's rule for that operation, not the finding's text. This serves
`goal@knowledge-architect@adoption-is-easy`: the finding is what a consumer reads first.

### A domain's checks plug in as an extension compiled into the binary, and the core binary registers none `##an-extension-plugs-in-through-phased-hooks`

The core checks what every project carries: the walk, the parse, the entity table, references,
registers, generated indexes and commit messages. A subject that belongs to one project, such as
thaum's verification of rule quotes against a pinned corpus, is an **extension**: a set of checks, the
manifest tables they read, and the files they generate. Each of its checks is listed and counted
like a check of the core. This is what `goal@knowledge-architect@any-project-can-adopt-it`
requires: the core is published for other projects, and nothing of thaum's subject is compiled
into it. A binary registers its extensions at
compile time. The core's own binary registers none, and a project that needs an extension builds a
binary that registers it. There is no loading at run time, because Rust has no stable ABI a
plugin could be built against.

The core calls an extension at fixed points of the run, and each call sits in the phase its output
belongs to, per `design@core@phases-gate-the-report`:

| call | when | what the extension does |
| ---- | ---- | -------------------- |
| resolve | phase 1, before the walk | reads the tables it claims, and returns its complaints, the paths it declares and the files it generates; the core reports the complaints in phase 1, asserts in phase 2 that each declared path exists, and leaves the generated files out of the walk and reads them as committed |
| prepare | once phases 1 to 3 passed | reads what its checks need for one tree, and may fail the run as could-not-run |
| check | phase 4 | its checks, with the count lines and the not-run names its summary prints |
| judge a message | `commits` | the rules a commit message is judged by, against that commit's prepared tree |
| generate | `index`, and phase 4 | the contents of the files it generates |
| dump | `model` | its observation rows |

**An extension is configured once per manifest and prepared once per tree.** `commits` judges several
trees in one run, and a message is judged against what its own commit's tree holds, so the
prepared state belongs to the tree and not to the extension. A parent tree, assembled for its entity
table alone, prepares no extension.

**The core's summary prints its own count lines, then each extension's**, and the list of checks
performed names the core's checks, then each extension's.

### An extension is handed a snapshot, `Tree::Snapshot`, for a commit's tree and for the tree git's index would commit alike `##an-extension-reads-a-snapshot`

`extension::Tree` has two variants: `Checkout`, the working tree, which an extension may read on
disk; and `Snapshot(&Snapshot)`, a tree read from git objects only, which is one commit's under
`commits` and the one git's index would commit under `check --staged`. `Snapshot::revision()` says
which. Under `check --staged` an extension is prepared with `Purpose::Check`, under `commits` with
`Purpose::Commit`, and under `index --staged` with `Purpose::Index`. `Snapshot::object_id` answers
only for a path the snapshot holds.

The index is not a third kind of tree for an extension: it is read through the same git objects as
a commit, so a third variant would make every extension write an arm reading the same objects as
the second. That departs from what `design@core@ne-minimal` asks of a new tree, that every
extension say how it reads it, and `design@core@trait-defaults` records the cost: an extension
that maps a snapshot to its commit-mode handling runs, under `--staged`, only what it runs for a
commit. thaum's rules extension lists its `changes` and `corpus` checks as not run there, and an
extension that lists nothing skips them silently. An extension that wants more under `--staged`
reads `Snapshot::revision()`.

**An extension reads the tree through the core.** Over the checkout it may read the filesystem under
the root, because a subject such as a vendored corpus is filesystem state, per
`design@core@model-then-checks`. Over a snapshot it reads git objects only, through the same
batch reader the core assembles that tree with, which also names each blob so that an extension can
cache what it parsed from one blob across commits.

The design this decision did not keep: `rejected@core@third-tree-variant-for-index`.

### An extension scans the core's parse on its own, and the core's model carries nothing for it `##an-extension-builds-its-own-model`

The core's model holds what the walk read and the parse produced, and the observations the core's
checks read. It has no field that exists for an extension. An extension reads the parse through the core's
public API: each document's text, its prose regions and scopes, which spans are code, the Rust
names, the inert lines and the fences. It scans a second time and keys what it finds by document,
in the model it was given.

This is what keeps the core free of any one project's subject, which
`goal@knowledge-architect@any-project-can-adopt-it` requires. A model generic over an extension's
observation type would carry a type parameter into every check signature for the same data, and
a type-erased slot per document would hold it inside the core with no type checking; both are
`rejected@core@model-generic-over-extension` and `rejected@core@type-erased-document-slot`.

**The cost is the extension's own scans.** Each document is still read once and parsed once, per
`design@core@model-then-checks`, and an extension scans the parse as often as its checks
need: thaum's rules extension scans each document in each check that reads it. Over thaum's
tree the extension's scans cost 0.07 s per `check`: the wall time of thaum's declared command's
`check`, run three times with the scan in the core and three times with it in the extension, on
one machine, compared. A measurement that grows to a share
of the run comparable with the walk and the parse reopens caching the scan inside the extension.

**An extension cannot add a kind to the entity table.** A reference kind is the core's, so an extension's
subject cannot be cited as `<kind>@<anchor>@<id>`. Nothing needs that today.

### An extension claims the manifest tables it reads, and the core refuses a table nobody claims `##an-extension-claims-its-manifest-tables`

The core parses the tables it owns. Every other top-level table goes to the registered extension
that claims it. A table that no registered extension claims is a phase-1 finding, and so is a table a
registered extension claims that the manifest does not hold.

**Both refusals serve `design@core@the-regime-has-no-opt-out`.** An unclaimed table is a
declaration nothing reads, which a session would take for a regime in force. A missing claimed
table would let a manifest switch an extension off by leaving its table out. So the binary decides
which extensions run over a tree, and the manifest cannot remove one: the core binary run over a
manifest that declares thaum's `[rules]` table reports the table as unclaimed, rather than
skipping the regime in silence.

**A table keeps the extension's own name at the top level.** A namespace for extension tables is
`rejected@core@extension-table-namespace`.

### The core's commands are a library module, so a binary that registers extensions offers them unchanged `##the-core-cli-is-a-library-module`

The commands of the core, `check`, `show`, `issues`, `tripwires`, `index`, `model`, `anchors`,
`commits`, `install-agent-skills`, `init`, `new-component`, `worklist` and `transcript`, and the code that runs them, are a
module of the core library rather than of its binary. A binary that registers extensions flattens the core's command enum into its own and
adds the commands of its extensions beside them, so the core's commands keep one spelling, one set
of arguments and one exit-code contract in every binary. The core's binary is that module with no extension registered.
`init` creates the project the other commands locate, so every binary hands it to `cli::init`
before it locates one, as the core's main, the setup skill's maintenance crate and the crate
documentation's example of an extension binary do; `cli::run` refuses it inside a project, naming
`cli::init`, so a binary that skips the step says why instead of acting.

A binary crate cannot be depended on, so a command set held in the core's binary would be copied
into every binary that registers an extension, and the copies would drift. The helpers an
extension's own commands need from a run, such as assembling a complete working tree, are public
in the same module for the same reason.

### What a run fetches before any check is gathered once, by `Gathered`, and no caller outside the crate assembles `Inputs` `##inputs-are-gathered`

`Gathered`, in the core's command module, reads the
committed generated files, every `register.toml`, one survey, the two git batches and the shipped
agent files, and lends them as the `Inputs` a check of the working tree reads. `check`, the
commands that write files into the working tree (through `complete_working_tree`, which refuses
over an incomplete model), and an extension's tests all build on it. `check --staged` and
`index --staged` read a snapshot instead, and build their `Inputs` from it, per
`design@core@staged-tree-source`. So no caller outside the crate assembles
`Inputs`, which `design@core@ne-minimal` makes impossible for a literal. A new input is then a
change inside the core alone. The core's own mock-project tests still assemble `Inputs` by hand:
some of them hand a check an input that differs from the tree's, such as no committed file.

The design this decision did not keep: `rejected@core@inputs-struct-literal`.

### The public API is a facade of role modules, one per kind of use, and every other module is private `##api-facade`

The library has three kinds of consumer: a binary that registers an extension, the extension, and
the extension's tests. **Testing an extension over a mock project is part of what the library
publishes**, because a project that adopts the tool with a subject of its own, per
`goal@knowledge-architect@any-project-can-adopt-it`, has to test that subject as the core tests
its own. `design@core@a-foreign-build-is-refused` already requires every extension binary to test
the refusal in its own suite.

Every module except the role modules is private. The crate root re-exports the nouns every
consumer meets, and the role modules hold the rest, one per kind of use, as the table below lists
them. `cli` and
`extension` are both a role module and the code of that role; `document` and `testing` only
re-export:

| module | what it serves |
| --- | --- |
| `cli` | a binary's `main`: the commands, running one, finding the project, refusing a foreign build, gathering a run's inputs |
| `extension` | writing an extension: its two traits, what the core hands them, what they return |
| `document` | reading a document's parse, per `design@core@an-extension-builds-its-own-model` |
| `testing` | running the core over a mock project as the binary does |

- **The public surface is read in one file**, lib.rs, and the role modules beside it. It is
  not computed from the implementation's module tree, so moving an item between files changes no
  public path, and is no library break under `design@knowledge-architect@versioning-policy`.
- **A public signature keeps its types public.** `Inputs` is in `Prepared::check`'s signature and
  its fields expose `Entry` and `Outside`, so those are public although no consumer names them.
- **A `pub` item no consumer can reach is refused.** lib.rs carries `#![warn(unreachable_pub)]`,
  and the clippy gate runs with `-D warnings`, so an item is public only through the facade.
- **A value only the core produces is read, not built.** `Report`, `Snapshot` and `Outside`
  have public read-only methods and no public constructor. A value a consumer starts from, such
  as a `Manifest`, a `Model` or a `Finding`, keeps its public constructors.
- **Nothing becomes public for the core's own tests.** A test that needs a private item is a
  unit-test module inside the crate, as `path@core@src/mock_projects.rs` is.
- `Manifest::command` is public although thaum's extension does not call it: an extension's
  finding that names a repair command owes the project's declared command, per
  `design@core@declared-command`.

The designs this decision did not keep: `rejected@core@module-tree-as-public-api`,
`rejected@core@doc-hidden-test-items` and `rejected@core@testing-feature-for-test-items`.

### A public type is non-exhaustive only where no consumer can usefully match or build it `##ne-minimal`

`#[non_exhaustive]` sits on `cli::Command`, `extension::Inputs`, `extension::ExtensionReport`
and `extension::Resolution`. Every other public enum and struct is exhaustive on purpose.

| | consumer matches exhaustively | consumer uses `if let`, `==`, `matches!` or `_` |
| --- | --- | --- |
| enum without the attribute, variant added | compile error at the match | compiles, nothing changes |
| enum with the attribute, variant added | impossible: the attribute forced `_` | compiles, nothing changes |

- **Without the attribute, a new variant breaks only a consumer who chose an exhaustive match**,
  which is a consumer asking to be told. The attribute takes that choice from every consumer. The
  lint that would warn on a wildcard hiding a new variant, `non_exhaustive_omitted_patterns`, is
  unstable on the pinned toolchain.
- **For an enum the core hands an extension, the compile error is wanted.** A third
  `extension::Tree` must make every extension say how it reads it. A wildcard would read the
  working tree while the run judges another tree: a wrong verdict, with exit 0. The tree git's
  index would commit is no third kind, since it is read from the same git objects as a commit, per
  `design@core@an-extension-reads-a-snapshot`, which records the cost.
- **The attribute's gain is the library's:** a variant added to an enum without it is a breaking
  change under Rust's semver rules. Under 0.x that gain is nil, because major and minor both
  bump 0.MINOR, per `design@knowledge-architect@versioning-policy`. Every variant now foreseen
  comes with a change that already bumps 0.MINOR: a stricter or added check, a new command, a
  new run mode.
- So the attribute goes only where no consumer can usefully match or build a literal: a binary
  hands a `Command` to `cli::run` unmatched, `Inputs` comes from `Gathered`, and an extension
  fills the two reports from `Default`, which the attribute allows.
- `Generated`, `DumpRow` and `Library` stay constructible by literal. A new field there is a new
  obligation on the extension or the binary that builds the value, and the compile error says
  so.

After 1.0 the attribute decides minor against major for every added variant, so this decision is
re-examined before the project leaves 0.x. When it leaves is
`design@knowledge-architect@stays-at-zero-x`.

The design this decision did not keep: `rejected@core@non-exhaustive-on-growing-enums`.

### A hook added to `Extension` or `Prepared` has a default body only when doing nothing is a correct answer `##trait-defaults`

The two traits have no default method. A hook added later gets a default body only when an
extension that does not know the hook is still right to do nothing, such as a new dump. A hook
whose absence would make a verdict wrong is added without a default, and adding it is a breaking
change. The argument is the one of `design@core@ne-minimal`: suppose a pre-commit mode adds
`Prepared::check_staged` with a default that returns no findings. Every existing extension
compiles unchanged, its checks silently do not run in the new mode, and the run reports success.
`check --staged` adds no hook: it hands an extension a snapshot, as `commits` does, and what an
extension runs over one is its own, per `design@core@an-extension-reads-a-snapshot`, which records
where that falls short of this argument.

The design this decision did not keep: `rejected@core@no-op-hook-defaults`.

### The library and the binary are one crate, `knowledge-architect` `##single-crate`

The library, imported as `knowledge_architect`, and the binary are two targets of one package. The
library already depends on clap, because its commands are a library module per
`design@core@the-core-cli-is-a-library-module`, so a separate binary crate would save its users no
dependency and would add a second package name to publish and to pin. The foreign-build refusal of
`design@core@a-foreign-build-is-refused` lists this one crate for both, beside the crate of
installed text it links.

### The binary is named `klarch` `##binary-name`

The crate is `knowledge-architect`; the binary it installs is `klarch`, as the crate ripgrep
installs `rg`. The binary's name is typed at every run, so it is short; the crate's name is typed
once, in a dependency or an install, so it is descriptive.

The design this decision did not keep: `rejected@core@binary-named-after-package`.

### Every command's exit code says whether it ran before it says what it found `##exit-code-ladder`

| code | meaning |
| ---- | ---------------------------------------------------------------------- |
| 0    | the command ran, and its subject is in order                           |
| 1    | the command ran, and reports a negative answer about its subject       |
| 2    | the command could not run: bad arguments, no project, unreadable input |

This binds the core's commands in every binary that offers them, per
`design@core@the-core-cli-is-a-library-module`. `path@core@README.md` lists which
outcome of each command maps to which code.

**The third code is what makes the other two mean anything.** A project runs `check` and
`commits` as gates, in CI and in a session, and reads the exit code without reading the output.
Without code 2, a mistyped invocation or an unreadable input is indistinguishable from a tree
that fails its check, and a gate that treats both as a failure cannot say which one to repair.

clap supplies 2 for a parse failure, per `design@core@arguments-parse-through-clap`, so
the boundary costs nothing at the parse. What the ladder costs is that a command routes its own
unreadable-input errors to 2 rather than to the code it uses for a negative answer.

### Every command's arguments are declared through clap, and every binary tests its declaration `##arguments-parse-through-clap`

The core's commands are a clap `Subcommand` enum, `knowledge_architect::cli::Command`, and a binary
that registers extensions flattens it into its own derive, per
`design@core@the-core-cli-is-a-library-module`. So clap is part of the core's public
interface, and a binary built on the core parses through clap too.

These binaries run blind, as gates, so an unknown flag or an invalid combination has to be a
refusal rather than a no-op. A hand-written parser refuses only the combinations its author
thought of. With clap, an unknown argument is refused by default, and exclusivity is a
declaration (`conflicts_with`, `requires`, `ArgGroup`) that a test checks as a whole.

**Each binary tests its own declaration**, with clap's `Command::debug_assert`, and asserts that
an unknown argument is refused. A declaration is checked only by such a test: clap finds a
malformed one at the first invocation that reaches it, not at build time.

### A parse that cannot be trusted is reported, never silent `##a-failed-parse-is-loud`

A file the walk cannot read, a source the grammar cannot parse, and a source nested deeper than
the walk goes all produce a finding naming the file. Silence would remove every citation in it
from the walk while the run reported success, which is indistinguishable from a clean file,
against `goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all
of it.

**A file outside the walk is held to the same, for the question an extension asks of it.** An
extension may assert that no unwalked file says something, as thaum's rules extension does of a rule
number. Its bytes are handed over decoded lossily when they are not UTF-8: the question is
whether an ASCII token appears, and a lossy decoding keeps every ASCII byte, where a walked
document read lossily would be text nobody wrote. **A binary file is handed to no check**, by
git's own test, a NUL byte in its first 8000 bytes, in `check` and in `commits` alike: its bytes
are no prose anybody wrote a claim in, and a number in them, such as a PDF's page size, is data.
Every other state is a phase-2 finding naming the file, each with its own repair: a path git
lists and the working tree does not hold, a directory where git lists one entry, and a file whose
bytes cannot be had at all. A symlink and a gitlink are no file to read, and are reported as
links.

The rival, a read or a parse that fails in silence, fails in two ways. Under it, one byte of
Windows-1252, such as a pasted em dash, puts a document read strictly outside the walk and outside
the inverse assertion, so a fabricated quote in it is read by nothing. And unbounded recursion
aborts the whole run with no file named and no finding printed.

### Git supplies the walk, and the manifest declares only what git tracks `##git-supplies-the-walk`

The walked set is git's listing from the manifest's directory, `git ls-files --cached --others
--exclude-standard`, with `[walk] skip-dirs`, `skip-files` and `exclude` applied after it, and the
generated indexes and the installed agent files removed by construction. Nothing generated is declared: `target` and
`path@agent-config@worktrees/` are covered by the ignore rules, so no `[walk]` row names them. A path
reference whose target the ignore rules cover is exempt from assertion the same way, per
`design@core@ignored-targets-are-not-asserted`.

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
`check::tree` asserts each one exists. **A declared path is spelled the way git lists
it**, with no `.` segment, no `..` segment and no leading `/`: a `.` segment is dropped when
the manifest loads, so two spellings of one directory are one declaration, and a row holding
`..` or an absolute path is refused as a finding and acted on by nothing, because what it
names depends on where the manifest sits rather than on the tree.

**No ignore rule can take a tracked file out of the walk.** `--cached` is unaffected by the ignore
rules, so no ignore line, however written, can take a live document out of every check. What
leaves the walk does so by the manifest: a declared row, or an exclusion derived from a
declaration, as the generated indexes and the installed agent files are, per
`design@core@owned-namespace-check`. That is what a
hand-rolled matcher cannot promise, and it is the property the whole walk is
chosen for; the matcher and the three things it could not do are
`rejected@core@hand-rolled-gitignore-matcher`.

A tracked path the working tree does not hold — a deletion nobody has staged — stays
in the walk and is reported, because dropping it would take a live document out of every check on
the strength of a working-tree state.

**The summary block prints the walked-file count**, so two machines disagreeing about the walk is
visible in the output rather than inferred from a finding list. The tripwire is in
`register@core@tripwire`.

**This is the working tree's walk, the one plain `check` reads.** `check --staged` and `commits`
read a snapshot instead, from git objects alone, per `design@core@staged-tree-source` and
`design@core@a-commit-message-is-a-document`.

### Every git invocation runs with the per-user ignore file pointed at the null device `##per-user-ignore-file-pinned`

**The per-user ignore file is pinned away.** `core.excludesFile` lives in a developer's home and
is no part of any project, so every invocation runs with it pointed at the null device: a line
there would otherwise take an untracked live document out of every check on one clone and not on
another. What still decides the walk beside the tree's own ignore files is git's per-clone exclude
file, which git offers no way to pin, and that is what
`tripwire@core@walked-count-differs-between-machines` is left guarding, until the check of
`issue@core@the-walk-does-not-report-what-the-per-clone-exclude-file-removes` reports the files it
alone removes from the walk.

### A listed path is read as git's bytes, never decoded `##a-path-is-bytes`

**A path is bytes, not text.** The `-z` output is split on NUL and turned into paths byte for
byte. A name holding a byte no UTF-8 decoding accepts is legal here, and decoding it lossily
produces a path nothing on disk answers to — so the file leaves every check, and where nothing
reports the failed read the run stays green. That is the failure
`design@core@git-supplies-the-walk` exists against.

### Reading a commit's tree needs git 2.36 `##git-floor-for-commit-trees`

**Reading a commit's tree needs git 2.36.** `cat-file --batch -z` takes its requests
NUL-terminated, and a tracked filename may hold a newline: under the newline-terminated input
such a name is two requests, git answers both, and every later answer is paired with the wrong
path while the map still comes back full. An older git refuses the flag loudly, which is exit 2
with git's own reason.

### A git invocation a verdict depends on that fails is exit 2 with git's reason, never an empty walk `##failed-git-is-exit-two`

**No `git`, or no worktree, is exit 2 with the reason.** Never an empty walk: a project reported as
holding no document is a run that checked nothing and said so as a clean verdict, against
`goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of it.
**Every invocation a verdict depends on fails that way** —
the listing, the tracked-and-ignored listing and the `check-ignore` batch — carrying git's own
stderr, whatever the reason. The one invocation that degrades instead is `last_changed`, which
fills a listing's convenience column and moves no verdict: where git answers nothing the column
shows `-`.

### A symlink and a gitlink are read as no document, and each is a phase-2 finding naming it `##links-are-no-documents`

Git records a symlink as mode `120000` and a submodule's gitlink as `160000`, in the index and in
every commit's tree alike, so the listing classifies an entry by mode before either reader touches
a byte and `check` and `commits` agree by construction. Read through the filesystem a symlink
yields its target's bytes, read from a tree it yields the target's path as text, and a checkout on
a platform without symlinks holds that text as a plain file, so two readers of one tree would
judge two documents; a gitlink names a commit of another repository the listing never descends
into, so everything under it would be conformant by vacuum. A `skip-files` row keeps a symlink
and an `exclude` row a submodule, as declared silences. What a submodule is to the project that
holds it is an open question, `issue@core@a-submodule-is-a-project-of-its-own`.

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

The walk drops the file, no check reads it, and `check::tree` reports it
once with any line break escaped — `Finding` escapes a line break in the path it is located at
and in the statement it makes, so a reader taking the output by line meets one finding per line
whatever route a name took to reach it. A project that means to keep such a file names it in `skip-files` or in an ignore rule.

### A tracked file the ignore rules also cover is a phase-2 finding naming the file `##tracked-and-ignored-is-a-finding`

The two states contradict each other and the contradiction is otherwise silent: the walk reads the
file, and `git check-ignore` skips what the index holds so a reference to it is asserted too — the
reverse of what the ignore rule says. It is reported in phase 2, by `check::tree`, with the other findings
that judge what the tree holds against what the project declares. The run stops there, as for
every phase-2 finding, per `design@core@phases-gate-the-report`: the model holds a file the
project says is out of it, so a finding computed from that model is unreliable.

### Everything git answers goes through one module `##git-is-asked-through-one-module`

`path@core@src/git.rs` owns the listing, the tracked-and-ignored listing, the one `check-ignore` batch, the last-change dates, and
every read of history — `rev-list`, `rev-parse`, a commit's file list, its blobs and its message —
so the set of things this tool asks git is auditable in one read and a check keeps spawning
nothing.

### `check --staged` judges the tree git's index would commit, and plain `check` the working tree `##staged-tree-source`

`cargo klarch check` judges the working tree as git lists it, untracked files included, and prints
no `tree:` line. `cargo klarch check --staged` judges the tree
`git commit` would record now: HEAD's tree, or none before the first commit, overlaid with the
index's changes as `git diff --cached --raw` lists them, each blob read with the `cat-file` batch
`commits` reads a commit with. It is the only tree a session cannot
judge before the commit exists, and `commits` judges it only after; the motive is a partial commit,
where the working tree passes and what is committed does not.

- **The listing is not `ls-files -s`.** That lists an entry staged as intent-to-add, `git add -N`,
  at stage 0 with the empty blob, the same line as a staged empty file, and `git commit` records no
  such entry; `diff --cached` leaves it out. Measured on git 2.43.0. A path the listing does not
  hold is no path of the snapshot, a manifest staged as intent-to-add included.
- **The diff is git's, and three of its behaviours are pinned against**, each measured on git
  2.43.0 and each with a test: without `--relative` a project in a subdirectory of its repository
  meets repository-relative paths and its siblings' changes; under `--raw` only `--no-abbrev`
  gives whole blob ids; and `diff.ignoreSubmodules` or a `.gitmodules` entry's `ignore` key drops a
  staged gitlink unless `--ignore-submodules=none` overrides it. The full invocation is at
  `snapshot_entries` in `path@core@src/git.rs`.
- **An unmerged index is refused**, exit 2 naming each path once: a conflicted merge or rebase has
  no tree to commit. `git write-tree` is not used, since it writes objects into the repository and
  fails there too.
- **The summary block says which tree was walked.** `--staged` prints `tree: staged` after the
  `walk:` line, or `tree: staged (nothing staged: the tree of HEAD)` where the project's part of
  the index equals HEAD, so a run made before staging is not read as a verdict on the edits.
  Plain `check` prints no `tree:` line, so a `walk:` count CI compares with a local one is of the
  working tree on both sides.

The design this decision did not keep: `rejected@core@ls-files-staged-tree`.

### `check --staged` applies `check`'s rules, as on a clean checkout of the staged tree `##staged-check-semantics`

The staged installed files are compared with the running binary's shipped set, per
`design@core@owned-namespace-check`, and the files git both tracks and ignores are reported. The rival, `commits`'s rules over the
staged tree, lost because CI runs `check` on a checkout: a staged run that skips the installed
files passes a tree CI fails. Two reads stay the working tree's: the ignore rules a path reference
asks, as for `commits`, and the version pin, compared before any command per
`design@core@installed-binary-version-check`.

### `check` judges no given commit's tree; `commits` does `##check-judges-no-given-commit`

`commits <rev>~1..<rev>`, or `commits <rev>` for a root commit, judges a given commit's tree with
the tip checker, and a second judge with `check`'s rules would give one tree two verdicts, per
`design@core@one-question-one-command`.

### Every rule over a document is enforced, and no declaration exempts a document from one `##the-regime-has-no-opt-out`

The rules over documents are compiled in and the manifest declares nothing about them, so
conformance of a document means the same thing in every tree this tool checks. There is no flag, no
list and no severity: a rule either holds over a document or the run fails. The constraint derives
from `goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of
it, and no project can hold part of it back.

**A declared register adds obligations and removes none**, which is what keeps
`design@core@registers-are-declared` inside this head rather than an exception to it. Declaring one
gives a project a kind, a shape and a home to be checked against; it cannot loosen the eleven built
in, and it cannot exempt a document from anything. A manifest that declares no register is checked
exactly as it was.

**One declaration removes obligations, and they are the agent configuration's alone.** A project
that declares no agent harness, per `design@core@agents-table`, owes no CLAUDE.md and no installed
agent file: these are documents a component carries, not rules over the documents it has. Every
document the project carries is checked exactly as under the default.

**An exemption, where an extension offers one, names FILES rather than rules.** The core offers
none. A file named is one that is leaving the tree, so the exemption expires with its subject; a
rule held back would have applied to every file and expired with nothing. That asymmetry is the
whole of it — exempting a document that is about to be deleted costs the guarantee nothing, and
exempting a rule costs it everywhere at once.

## 2. What a tree carries: anchors and registers

### Every component carries the same documents and the home of every component register `##components-carry-the-same-documents`

The components are the manifest's `[project]` list, per
`design@core@nothing-of-a-project-is-compiled-in`: the project root, named by `project.name`, and
every other project-relative directory, named by the basename of its path. Nothing is discovered by filename, so a document in a
directory that is not a component is not that kind of document: nothing counts it and nothing
reports it. A component carries `README.md`, `CLAUDE.md` while the project serves the `claude`
agent harness (per `design@core@agents-table`), and the home of every `component`-scoped register
— `design`, `goal`, `tripwire`, `rejected` and `issue`, and any the manifest declares at that
scope. No document of the compiled list lies under `docs/`, where those homes are. The root also
holds the plans directory, which is not a component's document but the anchor `plans`, per
`design@core@plan-register`. A component that does not carry one it owes is a finding, raised by
`path@core@src/check/tree.rs`.

**Which components exist is declared, and what every component carries at least is compiled in.**
That is not an exception to `design@core@nothing-of-a-project-is-compiled-in`: the document list and
the five built-in component registers in `manifest.rs` belong to no project, they are what the word
_component_ means here. A project adds to that set, per
`design@core@a-component-carries-what-a-project-attaches`, and removes nothing from it but the
agent document, below: a project
free to declare its own set would be conformant with whatever
it declared, which is the same as being checked against nothing, against
`goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of it,
and no project can hold part of it back. The one declaration that changes
the set is the agent harness, and it removes only the agent document. The flaw this leaves, that a
developer's contracts lose their required home in a project without agents, is
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`.

**A directory that carries outstanding state without being a component is a location**, per
`design@core@anchors-are-components-and-locations`.

### The designs a project did not keep are entries of a built-in heading register, `rejected`, at level three `##rejected-register`

Every component carries the register `rejected`, its home `register@*@rejected` a file
`rejected.md` or a directory `rejected/` with a README under the component's docs/, per
`design@core@heading-register-two-shapes`. An entry is a level-three
heading that names the design not kept, its slug last, and its body opens with a marker, `live` or
`void`, then the reason it was not kept. It is cited `rejected@<anchor>@<id>`, so `show` prints it
whole with every text that cites it, and a plan item, a review finding, a commit message or a head
can name one design.

It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`, which asks for the
alternatives that lost and why, and `goal@core@records-reach-their-reader`, which asks that a
recorded entry reach whoever needs it through checked references: an entry with no slug can be
cited by nothing, and `show` prints an entry whole only when it is one. The home stays apart from
the design home, on the owner's argument: an agent reading a decision file partly, by grep, could
take a rejected design for what is built, and separating the two at the level of the files keeps a
grep hit on one from reading as the other. The nearest rival, the paragraphs of a compiled
document with `show` printing each one that cites a head, delivers only to a session that runs
`show` on the right head, and leaves nothing for an item, a finding or a message to cite. The name
is `rejected`, and the home is named after it, both on the owner's word: the owner proposed the
name for a shorter citation, finding a register of alternatives alone a bit odd, and judged the
new home's name to read better and to avoid a confusion point. Which designs earn an entry is
`design@agent-skills@losing-alternatives-filter`.

The designs this decision did not keep: `rejected@core@paragraphs-with-show-printing-them`,
`rejected@core@kind-named-rejected-alternative`, `rejected@core@home-keeping-the-old-file-name` and
`rejected@core@validity-marker-checked`.

### A tripwire guards no `rejected` entry `##tripwire-guards-no-rejected-entry`

The `guarding` column of `cargo klarch tripwires`, and its `--guarding` filter, read every
reference of a tripwire to an entry of a register but `issue`, `tripwire` and `rejected`. A
tripwire guards a decision: a `rejected` entry records a design not kept, which is no decision, so
a tripwire that names one names the candidate its response would reopen, not what it guards. The
rival, counting it like a declared register's entry, would list a design not kept as guarded.

### The checker does not check a tripwire's horizon line, for now `##tripwire-horizon-unchecked`

A tripwire's `**Watches until:**` line, which `design@agent-skills@tripwire-horizon` gives every
entry, is written and read by the sessions and the reviewers that the installed workflow sends, and
no check of the core reads it. Whether a horizon has passed is an event over the history the
checker does not judge, and a presence check would read a field of a heading entry, the syntax
`rejected@core@validity-marker-checked` refused for the `rejected` register. The owner chose to
keep the line out of the checker for now, judging a check not yet needed. The tripwire axis of the
project audit brings every entry to the line, per `design@agent-skills@tripwire-audit-axis`, and the
standing-state reviewer reads each entry written after it, per
`design@agent-skills@standing-state-review-before-every-merge`.

### A `rejected` entry that no design entry cites is a finding `##orphan-rejected-entry`

The last phase reports, at its definition site, every `rejected` entry that no entry of a `design`
register cites inside its own section, in any anchor. A citation from any other text, a goal, an
issue, a tripwire, another `rejected` entry, or a design home outside its entries, does not count.
The finding names the repair: cite it from the design head whose argument covers it, or from a head
of absence that records the decision not to build it; or, where what it lost to is an instruction no
head states, delete it and name the design and its reason beside that instruction, which
`skill@knowledge-architect-decision-recording@losing-alternatives` admits since no head cites it. It is raised over a tree in
`path@core@src/check/references.rs`, not over a commit message, which holds no design entry.

It serves `goal@core@declared-instructions-are-checked`: a head cites the designs its argument
covers, per `design@agent-skills@rejected-citation-direction`, and this check is what makes every
`rejected` entry reachable from the text a design session reads, the head, rather than only from
`show`. Only a citation from a design entry counts, on the owner's judgement; a citation counts by
the reference rule of `design@agent-skills@a-reference-claims-a-revisit`.

The designs this decision did not keep: `rejected@core@goal-citing-a-rejected-entry`,
`rejected@core@entry-reachable-by-citing-a-goal` and `rejected@core@no-orphan-check`.

### An anchor is a component or a location, and a location carries the registers it declares or the tool gives it `##anchors-are-components-and-locations`

Both carry registers. Every anchor is a named directory and a path anchor under
`design@core@every-path-names-its-anchor`, except the plan anchors the tool constructs, below: a
spec anchor is a file, and no plan anchor carries the `path` kind. What separates them is what they owe. A **component**
owes the compiled documents, every `component`-scoped register and every `opt-in` register the
project attaches to it, with its homes under `<component>/docs/`. A **location** owes the homes of the registers its `[locations.<name>]`
row declares and nothing else, with its homes directly under its own path, or at it for a register
declared there, per `design@core@home-at-the-carrier-path` — a location's directory is already
documentation, where a component's is source code.

**A location exists so that a directory outside every component can be cited.** Naming the
directory gives every entry in it a reference and makes relocating it one manifest edit, which
`goal@core@relocation-is-one-manifest-edit` asks and `design@core@every-path-names-its-anchor`
buys everywhere else. What a location is instead of
is `rejected@core@additional-trackers-file-list`.

**A location's own registers are the only ones it carries**, and a skill or an agent of the
project's own under one, such as this repository's under `agent-config`, is a section home of a
harness kind, which no anchor carries, per `design@core@harness-kinds`.

**Three kinds of location are constructed by the tool rather than declared**, and each is a location
in every respect but the one its layout fixes:

- **`plans`**, at docs/plans/ of the root, carrying `spec` and `milestone`, per
  `design@core@plan-register`. It is judged against the declared anchors like a location, first
  among the anchors of its depth, so a declaration that collides with it is the one refused and
  its complaint names a declaration the project can change. It owes a `README.md` beside its two
  homes.
- **One anchor per entry of a Directory register**: each directory of a Directory register's
  home holding a file `README.md`, named by its basename, per
  `design@core@declarable-directory-register`. Each register it carries has its home at the
  anchor's own path. A directory whose name `design@core@a-placed-name-reads-as-nothing-else`
  refuses is no anchor. A milestone is one: its `spec` register's slice specs sit beside the
  milestone document and are cited `spec@<milestone>@<slice>`, unique only inside the milestone,
  per `design@core@plan-document-kinds`; it carries the four item registers, per
  `design@core@plan-items-by-section`, and no `path` kind.
- **One anchor per spec**: each spec file of specs/, grouped or not, named by its id, per
  `design@core@spec-file-is-an-anchor`. It owns its own file, carries the four item registers,
  and carries no `path` kind. A spec whose name is refused, or that a milestone holds, is no
  anchor, and a directory is never a spec.

**The entry and spec anchors are read off the tree, so every builder of the anchors is handed the
tree's paths**: a check the survey's listing, a command git's listing, `commits` each commit's own
listing. An entry is a directory a session creates, not a declaration, so asking the manifest
alone would give a commit the working tree's entries and judge its message against anchors it
never held. `manifest::collides` does not judge them, because a register's home places them; the
name check is their collision check, and it is a phase-2 finding.

### No two anchors give one path two meanings: an anchor sits inside another's directory, never at its path or comparable with one of its register homes `##anchor-collisions-refused`

A location inside a component is the ordinary case, and the deepest anchor wins, so a document
under a location belongs to it and not to the component holding it. Four declarations take that rule away. Each is a complaint of the manifest's
resolution, and **the refused anchor is no anchor**: it owns nothing, carries nothing and is
asserted against nothing, per `design@core@a-wrong-declaration-is-a-finding`. Candidates are taken
shallowest first, a component before a location at equal depth, then components in declaration
order and locations in name order, so of two that collide the deeper or the later one in that
order is the one refused:

- **a component inside a location.** It nests one full register set inside a partial one and
  gives a document two candidate homes.
- **an anchor inside, or at, another anchor's register home.** Every file under it would be read
  as an entry or a subdocument of the outer register, and reported against that register and its
  anchor rather than against the declaration that caused it.
- **an anchor whose directory holds another anchor's register home** — a location at a
  component's `docs/`. Being deeper, it would own every document in that home, and each slug
  defined there would be misplaced against an anchor carrying no such register.
- **two anchors at one path.** Neither is deeper, so nothing decides which owns the documents
  under it.

The principle behind the four is that no two declarations give one path two meanings. No
compiled document lies under `docs/`, where the component registers keep their homes, so no
register's home can be a compiled document. Outside those shapes two anchors' homes cannot overlap: a home sits under its anchor's directory, so two
overlapping homes put one anchor's directory on the path to the other's home, which is one of the
shapes above.

### No anchor wears a kind's name `##kind-names-refused-as-anchors`

A reference's head is read as a kind before it is read as an anchor, so an anchor named `design`, `path` or `skill` would be shadowed there, and the old form
written with it would be misread. Every kind's name, the harness kinds' whatever the harness and
every declared register's included, is refused for a Component, a location, the project, a
plan and an entry of a directory register, per `design@core@a-placed-name-reads-as-nothing-else`
for the last two. The owner, on the question:
"My ruling on this is that all kind names should be refused for anything that can be an anchor
name (components, custom locations...). I'd rather make this decision early to avoid painful
migrations."

### A Component carries every register the workflow needs, and any register the project attaches to it `##a-component-carries-what-a-project-attaches`

Every Component carries the component registers, the built-in ones and those declared with
`scope = "component"`, which the workflow needs in every Component, per
`design@core@components-carry-the-same-documents`. An `opt-in` register names the Components that
carry it besides, in its own table, `components = [<component>, …]`, the root's name included, and
each keeps its home under its own `docs/`, as `<component>/docs/<dir>/`. A name that is no
Component, and a home that would give a directory of the Component two meanings, its `docs/` itself
with `dir = "."`, the root's plans directory, or the directory of another register it carries, is a
phase-1 finding, and that Component does not carry it.

**The ground is the owner's intent**: a Component is always required to carry the registers the
workflow needs to function, and a project may add any kind of register to any Component, the root
included. The floor holds for every project, one that installs no agent harness included, on the
owner's word that such a project is still supposed, in most cases, to have people working by the
workflow, without which the checker has little use. Which registers can be attached is the
session's argued limit within that intent: a register the project declares, since a built-in
component register is carried by every Component already and a plan register belongs to the plans
layout, per `design@core@plan-register`. A register attached adds obligations and removes none, per
`design@core@the-regime-has-no-opt-out`. Against a location carrying the register at its own path,
per `design@core@home-at-the-carrier-path`, attaching costs one key, invents no anchor name, and
cites what the register holds under the Component's name. The rival, a Component's register list
fixed by the word _component_, is `rejected@core@component-register-list-fixed`.

**The list is the register's**, beside its `scope`, because a Component is declared by its path in
`[project] components` and has no table of its own: a table per Component would declare the
Components twice.

### Plan documents are a structure the checker reads: the anchor `plans` carries a `spec` and a `milestone` register `##plan-register`

The plans directory holds the decided design of work that is not built yet, which no other home
holds, per `goal@knowledge-architect@design-is-recorded-with-its-arguments`. The checker reads it
as anchors and registers, so a citation of a plan is checked like any other, per
`goal@knowledge-architect@documentation-stays-consistent`: the tool constructs the anchor `plans`,
carrying two built-in registers, `spec` and `milestone`, that no project declares. `spec` is
also carried by each milestone anchor, for its slice specs, per
`design@core@anchors-are-components-and-locations`; no other anchor carries either. Inside each
plan, its threads, arguments, criteria and acceptance criteria are items, cited and checked like
any entry, per `design@core@plan-items-by-section`, so the record of a discussion is checked
before it is harvested. Plan documents read as free prose was the alternative, and it leaves
every citation of a plan unchecked and every plan document unlisted.

### The plans directory is docs/plans/ of the root, fixed by the tool `##plans-dir-fixed`

A component gets its register homes at paths the tool fixes, and a built-in register refuses a
`dir` key, so a declared plans directory would be the only declared path in that family. The tool
fixing it costs a project whose plans sit elsewhere one move, and a skill written against the fixed
path works in every project, per `goal@agent-skills@installed-text-works-anywhere`. A fixed path is the tool's convention, not a project compiled in, per
`design@core@nothing-of-a-project-is-compiled-in`.

### A project has one plans directory, in its root component `##plans-at-root`

Most work spans components, so a plan sits at the root whatever its subject: the milestone that
built the plans layout touched three, and the milestone that released version 0.1 touched every
component. Two premises are the owner's weighing: "there should never be that many planned tasks
open at once that splitting the directory would make sense", and "plans have a tendency to break
each other, when they are designed before the other is implemented", which one directory keeps in
one view. The reference grammar names the anchor, so a later split adds
anchors and rewrites no citation.

The design this decision did not keep: `rejected@core@plans-directory-per-component`.

### The plans directory holds its README and two homes, specs/ and milestones/, and nothing else `##plans-split-dirs`

A spec is one file under specs/, and a milestone one directory under milestones/, each in a
register of its own: one register with two kinds taken from the entry's shape would be a new kind
of register, where two registers reuse the File shape and add one entry shape. Anything else
directly under docs/plans/ is a phase-2 finding, so a plan document always sits in a home and is
always listed.

### A plan document is cited by its kind, `spec` or `milestone`, and a slice spec in its milestone's anchor `##plan-document-kinds`

A spec is `spec@plans@<id>` and a milestone `milestone@plans@<id>`. Each milestone directory is an
anchor carrying `spec` for its slice specs, `spec@<milestone>@<slice>`, so a slice's name need only
be unique inside its milestone, and two milestones may each have a slice called `the-harvest`. The
`milestone` register's entries are directories because each is an anchor, per
`design@core@milestone-entries-are-directories`. A `path` citation of a plan document is
refused, per `design@core@every-path-names-its-anchor`: with two names for one document,
`show` would miss the citations written the other way. A whole plan document may be cited from
anywhere; when it leaves, each citation dangles, and that is the revisit it asks for.

### A name the tree places as an anchor reads as no kind's name, no declared anchor's and no other placed anchor's `##a-placed-name-reads-as-nothing-else`

An entry's name, a milestone's among them, and a spec's id sit in the anchor position of a
reference, since each entry directory and each spec file is an anchor. So such a name that is a
kind's name, a component's name, a location's name or a reserved word is a phase-2 finding, and so
is one another placed anchor already holds; a name so refused is no anchor. Otherwise a reference
reads as the other anchor, and which one wins depends on the order the anchors were built in.
**The order is fixed**: the milestones first, then the specs, then the entries of the declared
Directory registers, by register name, each register's entries by path. Of a spec and a milestone
of one name, the milestone keeps it, and an entry of a declared register gives way to every plan,
since the plans layout is the tool's and every project has it. An entry's name is also an entity
id, since it is the id of an entry of its register, and is not `index`: beside an entry, `<id>.md`
is the refused single-file shape of what it carries, which for `index` is its home's own listing.

### A plan's items are the level-three headings of its item sections, and the section gives the kind `##plan-items-by-section`

A plan document's threads, arguments, criteria and acceptance criteria are items: a level-three
heading ending with its slug, `### <statement> ##<id>`, under the level-two section Threads,
Arguments, Criteria or Acceptance criteria, defines an item of the kind `thread`, `argument`,
`criterion` or `acceptance`, in the plan anchor that owns the document. A milestone's README and
its slice specs share one namespace, so an id defined in two of them is the duplicate finding.
Every level-three heading of those four sections owes a slug; any other level-three heading of a
plan document is section text, and a slug anywhere else in it defines nothing. Items need a model
of their own because a heading register has one home file per anchor and one level, where a plan
has one home and four kinds; an extension runs after the entity table is built, so items are the
core's, per `design@core@an-extension-plugs-in-through-phased-hooks`.

### An item is cited from inside its own plan only `##plan-item-scope`

An item citation, `<kind>@<plan>@<id>`, resolves only from a file inside the plan: the spec file, or a file of the milestone's directory. From anywhere
else, a commit message included, it is refused before its id is looked up, and the repair names
the document to cite whole: the spec, the milestone, or the slice spec that defines the item, with
the item named beside it as `#<id>`, the label `skill@knowledge-architect-planning@spec-contents`
gives an item outside its plan. A
whole plan may be cited from anywhere, so a dependency between plans is carried by the document,
and retiring a plan asks no other plan to be redesigned item by item. The premise is that a
whole-document citation carries every dependency between plans that matters while few plans are
open at once.

The design this decision did not keep: `rejected@core@cross-plan-item-citation`.

### Item registers are the plan's own register shape, `Section`, whose home is the plan's documents `##items-as-section-registers`

The four item registers have the shape `Section`: each names its level-two section, its entries
sit at level three, and its home is the plan anchor's own documents, which the File and Directory
homes that hold them already judge, so a `Section` register owes no home of its own in phase 2,
no index and no walk check. The kind is read from the level-two heading in force above an item,
out of the heading records the scanner makes. Four ordinary heading registers at one home would
each claim the same file, and only the first would define anything.

### The item registers are built in, and carried by the plan anchors alone `##item-registers-built-in`

Like `spec` and `milestone`, the item registers are the tool's, because their storage is the plan
documents, which the plans layout fixes. A declaration of one is refused, per
`design@core@registers-are-declared`.

### A spec file is an anchor of its own, so a spec's items are cited by the spec's name `##spec-file-is-an-anchor`

Each spec file of specs/ is an anchor, so its items are cited `<kind>@<spec>@<id>`, as a slice's
are cited by its milestone's name, in every project that writes specs. The plan anchor stands in
the anchor position, so an item citation keeps the three-part grammar, shorter than the root
anchor and a compound id by the root anchor's name and one separator. The anchor holds its own
file, which stays an entry of the `spec` register that holds it. Making every spec a directory was
the rival; it reopens the decision that a spec is one file.

### A plan document owes the sections of its kind, and a slice spec owes its own `##plan-document-sections-by-kind`

A spec of specs/ and a milestone's README owe the plan sections, in order, with Arguments right
after Threads, matched as the documents write them: Status and audience, How the work is done,
Names, What the work is, What is already decided, Criteria, Threads, Arguments, "New names, in one
place", Decided design, Mapping tables, Losing alternatives, Readings, Premortem, Acceptance
criteria, Implementation sequence, Order rationale, Defaults awaiting the owner, Harvest, Later
consequences. A slice spec owes Builds, Claims, Audit subjects, Fails alone on and Premises that
expire, in order; the plan sections a slice spec holds, for the design only its slice builds, are
not ordered against them. Fixtures is owed only where a Component drives its tests with authored
content, a condition no check can read, so it is not checked. One `spec` register owes two lists,
chosen by the anchor that holds the spec.
The two lists are the planning skill's, which owns the shape of a plan document, per
`design@agent-skills@design-hands-off-to-planning`; a change to them changes this head.

### Which registers exist beyond the built-in ones is the manifest's `##registers-are-declared`

`[registers.<name>]` declares a register's `scope`, `shape`, `dir`, `level`, `sections`,
`metadata.<key>.values`, for a Directory register `carries`, per
`design@core@declarable-directory-register`, and for an `opt-in` register the `components` that
carry it, per `design@core@a-component-carries-what-a-project-attaches`; `[locations.<name>]`
declares a directory and the registers it carries.
Eleven registers are compiled in — `design`, `goal`, `tripwire`, `rejected`, `issue`, carried
by every component; `spec` and `milestone`, carried by the anchor `plans`, `spec` also by each milestone
anchor, per `design@core@plan-register`; and the four item registers, carried by the plan
anchors, per `design@core@item-registers-built-in`. A declaration for one of the first five accepts `kinds` on `issue`
and nothing else, because their storage is what the word component means. A table for a plan
register or an item register is refused whole, and so is any of their names in a location's
`registers` list, and a
component register whose home at the root would be the plans directory: the plans layout is the
tool's, and a declaration that changes it changes nothing a project can rely on. A declared
register named `path`, `planned`, `register` or a harness kind is refused too: a reference whose
kind segment is that word never reaches the register. Setting `scope`, `shape`, `dir`, `level`,
`sections`, `metadata`, `carries` or `components` on a built-in is a finding, and the compiled
value stands.

**Declarations go in the manifest because its header promises that what is checked is said in one
place.** Its bulk is comments, which is what a declaration owes a reader.

### An `opt-in` register declared with `dir = "."` keeps its home at its carrier's own directory, and that carrier carries no other register `##home-at-the-carrier-path`

A location at `<path>` carrying such a register is that register's home, so a Directory register
places its entries directly under the location, as `<path>/<entry>/`, and a heading register's
subdocuments sit in the location's directory, behind its README. Any other register the location
named would keep its home inside that one, one path with two meanings, so the location carries the
first such register it names and the others leave its row, each a phase-1 finding, per
`design@core@anchor-collisions-refused`. At component scope it is refused: every component would
keep the home at its own `docs/`, among its other homes. A register at `.` has no single-file
shape, which would be `<path>.md`, outside the location: a file beside the location is one more
document of the anchor holding it.

**It serves a home directly at a directory outside every Component's `docs/`**, such as subjects
at `<path>/<subject>/` under the project's root. Its nearest rival is an ordinary `dir`, which puts
the home one segment under the location, at `<path>/<dir>/`, a directory level the records do not
need. A home under a Component's `docs/` is placed by attaching the register to the Component, per
`design@core@a-component-carries-what-a-project-attaches`, with no location to name. The owner chose
to keep this key beside that attachment rather than replace it: `rejected@core@carrier-dir-replaced-by-components`.

### A register instance's own options sit beside it, in `register.toml` `##instance-options-beside-the-instance`

Per-instance options go in `docs/<dir>/register.toml`. They cannot sit in
the manifest without a table keyed by anchor, and the shape that avoids the table — a single list
at the root — would put one component's groups in another's directory. Today the only such option
is `groups`. A key other than `groups` is a finding, a file with no `groups` is a finding, and a
`register.toml` beside a heading register is one too.

The design this decision did not keep: `rejected@core@instance-options-at-root`.

### A declaration that is wrong is a phase-1 finding, and absent from the configuration `##a-wrong-declaration-is-a-finding`

A declaration that is wrong is a finding rather than a load failure, and it is absent from the
configuration. A manifest that will not load reports nothing at all, and nothing at all is what
a session reads as conformance, against `goal@core@declared-instructions-are-checked`: a tree that
passes the check conforms to all of it. So `Manifest::parse` resolves the declaration into a
configuration and a list of complaints, each in the shape a finding takes, and every complaint
is the first phase of a run, per `design@core@phases-gate-the-report`. What a complaint is
about is not in the configuration: a refused register is no register, a refused anchor is no
anchor, a refused row is not in its list, so nothing acts on it and its consequences are never
reported as defects of the tree. An extension's tables follow the same rule, per
`design@core@an-extension-claims-its-manifest-tables`: its complaints are phase-1 findings, and a
run that holds one stops before the extension reads anything.

The design this decision did not keep: `rejected@core@refusing-a-wrong-declaration`.

### A heading register's home is `<dir>.md` or a `<dir>/` directory, never both `##heading-register-two-shapes`

The single file serves an anchor whose register fits one document. The directory serves one whose
register has outgrown it. In the directory shape, `README.md` is the head: an introduction, and a
bullet list of markdown links, one per subdocument, each target written relative to the README.
The entries and their slug anchors live in the subdocuments.

**Both shapes serve every heading register**, so `design`, `goal`, `tripwire` and `rejected` accept the same
two, and so does any heading register a project declares.

Five assertions carry the shape, split over three places. `check::tree` asserts exactly one
home per anchor and register and that a directory home carries its `README.md`;
`check::registers` asserts that the README links every markdown subdocument the walk covers — a gitignored scratch file owes nothing. That
each relative link resolves is the references family's assertion, made for every navigation file
alike per `design@core@links-are-navigation-rows`. The entity table of `design@core@one-entity-table`
accepts a slug definition only in the file home or in a subdocument, matched against the owning
anchor's own paths rather than against a filename suffix — a suffix match would accept a slug
in any file whose name ends in `design.md`, a plan document included.

**Exactly one home, because two give a decision two candidate places to land.** That is the
one-home failure, and the reader who finds one half acts on half the design.

**The README must link every subdocument, because a subdocument nobody links is a home nobody
finds.** A markdown link is the index's row because a reader can follow it where a renderer
shows the page, and a bullet list is its conventional shape — a table grows painful to edit as
soon as a row outgrows a short sentence. Both directions are asserted over the same links: an
existing subdocument no link names is reported, and a link that resolves to nothing is reported.
A fenced link is an illustration and discharges nothing, the stance a fenced definition takes
per `design@core@a-slug-elsewhere-defines-nothing`, and a link inside a code span is typography showing the
shape. **A row is any link shape `design@core@links-are-navigation-rows` reads**, a link
definition included, so a README written with reference-style links indexes its subdocuments
like one written with inline links.
A backticked path in the README stays what it is everywhere: a checked pointer in prose, not a
row of the index.

**The README defines no slugs.** It is the head and the index. A decision recorded there competes
with the subdocuments as a home, which is what the split exists to end.

### A file register is a directory of one file per entry, with a hand-written README and a generated index `##a-file-register-is-a-directory-of-entries`

Its home is `<dir>/`. Every `.md` file under it is an entry named by its basename, except
`README.md` and `index.md` at the instance's own top level; a subdirectory is a declared group
and nothing else, groups nest one level, and a file of another suffix is a finding. An entry
carries frontmatter with each declared metadata key, a level-one title, the declared level-two
sections in order, and — for the issue register — the level-three subsections its kind owes.

**One file per entry, because a tool that edits a tracker edits the whole file.** Sessions editing
a shared tracker with `sed` and a string replacement have replaced every occurrence in the file
more than once. One file per entry makes that failure unreachable across entries, gives each entry
its own history, and turns closing an entry into a dangling-reference work list: the file is
deleted, and every reference to it becomes a finding.

**The README is hand-written and the index is generated, and neither stands in for the other.**
The introduction is prose about the project and belongs in markdown, and the README is also what
keeps the directory in git, which an empty directory cannot be. The index is mandatory: with the
README already holding the directory open, the churn argument against generating one has no weight
left, and a listing nobody generates is one that silently stops listing. This head owns that the
index exists; what it holds is `design@core@a-file-register-index-is-rows`, and that it is generated
rather than written by hand is `design@core@generated-files-are-pure`.

The designs this decision did not keep: `rejected@core@index-introduction-in-manifest` and
`rejected@core@optional-index`.

### A file register's groups are subdirectories, declared beside the instance `##groups-are-subdirectories`

A subdirectory is visible to `ls` and to a listing without parsing anything, and the group is not
part of an entry's id, so regrouping is a `git mv` that breaks no reference. The declared list is what keeps the naming from
drifting, and it is checked in both directions. An entry may sit ungrouped at the top level, so a
two-entry instance owes no directory.

The design this decision did not keep: `rejected@core@theme-metadata-key`.

### An entry's frontmatter is a small subset, refused loudly outside it `##frontmatter-subset`

**The frontmatter subset is small and refused loudly outside itself**, per
`design@core@a-failed-parse-is-loud`: a block opened and closed by a line holding only `---`, at the
very top of the file, holding `key: value` lines with scalar values, no key written twice. A key
the register does not declare is a finding, so a typo in a key name cannot pass as an absent
optional. Frontmatter is required exactly when the register declares metadata, and refused when it
declares none — an undeclared block is metadata nothing checks.

**A blank line between the delimiters means there is no block**, which is what tells a block from
a document opening on a thematic break: a paragraph between two thematic breaks has blank lines
around it, and a block has none.

### An entry's frontmatter block is read as prose, outside the document's structure `##frontmatter-is-prose`

**The block is prose, entire, and its lines are marked as a fence.** That marking keeps a value
out of the document's structure — a heading, a slug definition, a navigation link — and buys
nothing else: a fence has never made its contents data here, per
`design@core@grammars-not-prefixes`. So a reference in a value is a reference, and an
extension that reads its own markers in prose reads them in a value too.

**The block is blanked byte for byte before the structure is read.** The closing `---` is a setext
underline for the `key: value` lines above it, so left in place a block would open the document
with a level-two heading nobody wrote. Each byte becomes a space rather than being removed, so
every offset the analysis returns is an offset into the file as written.

### The issue register's kind list is closed `##issue-kind-list-closed`

An unknown kind is a finding naming the list, and adding one is a reviewed manifest diff. The kind
decides which subsections the entry owes, so an unknown one would owe the wrong three in silence, which is what makes refusing it load-bearing
rather than tidy. The open set is `rejected@core@open-issue-kind-set`.

### A milestone entry is a directory, carrying the slice specs and the four item registers `##milestone-entries-are-directories`

`milestone` is the built-in Directory register, per `design@core@declarable-directory-register`,
carried by the anchor `plans`: each milestone is a directory of milestones/ holding its
`README.md`, the milestone document, and its entry carries `spec`, for its slice specs, and the
four item registers, per `design@core@plan-document-kinds`. What a plan adds on top of the shape is
the tool's and keyed on the register: its documents owe the sections of their kind, per
`design@core@plan-document-sections-by-kind`, its README defines items, its items are cited from
inside the plan only, per `design@core@plan-item-scope`, and it carries no `path` kind. The shape
exists for it because the owner asked to cite a milestone as `milestone@plans@<id>`, and a
milestone is a directory: a File entry is one file and could hold no slice specs. Citing the
milestone by a path to its README needs no new shape, and loses to that request.

### A project declares a Directory register, each entry an anchor carrying the register its `carries` names `##declarable-directory-register`

`shape = "directory"` with `carries = [<register>]`: its home holds a hand-written `README.md`,
a generated `index.md`, and one directory per entry, each holding a file `README.md`, which is the
entry's document and owes a level-one title. Nothing else sits in the home, and no entry holds a
`register.toml` or a subdirectory; each of these is a phase-2 finding, because the entries are
anchors read off the tree. An entry is defined at its `README.md`, and only while the walk reads
that file, as a File entry the walk leaves out defines nothing, and it is cited
`<register>@<parent>@<id>`, the parent being the anchor that carries the home.

**Each entry is an anchor carrying, at its own directory, the one register `carries` names**, an
`opt-in` register of the heading or file shape, one a location could carry: a component-scoped
register is carried by every component already, a plan register by the plan anchors alone, and a
Directory register carried by an entry would nest anchors inside anchors, which no case has asked
for. **One register, on the owner's ruling**: a second would keep its home at the same directory,
one path with two meanings, which `design@core@anchor-collisions-refused` refuses for every other
anchor, and a home of its own under the entry would put a directory level between a subject and
its files. How a carried home is shaped is `design@core@entry-anchor-carries`, and what names an
entry's files is `design@core@entry-anchor-path-kind`.

**It is declared because a project can hold records whose subjects are directories**, as a game's
design split by subject: one subject per directory, several files each, every entry cited
`<kind>@<subject>@<id>`, which `goal@knowledge-architect@any-project-can-adopt-it` asks the manifest
to express without anything of the project compiled in. Adding a subject is creating a directory,
not a manifest edit, and moving the home is one manifest edit, per
`goal@core@relocation-is-one-manifest-edit`. A location declared per subject costs a manifest row
per subject. The rival that makes each subdirectory of a location an anchor, with no Directory
register, is `rejected@core@subdirectory-anchors-location`.

### A register an entry carries keeps the directory shape alone, the entry's README its head `##entry-anchor-carries`

The home of what an entry carries is the entry's own directory, beside its `README.md`, which is
both the entry's document and the head of that home: it links every file of a carried heading
register and defines no slug, per `design@core@heading-register-two-shapes`. The single-file shape
would be `<id>.md` beside the entry directory, so a heading register at an entry would have two
homes, which that head refuses: `<id>.md` beside an entry is a phase-2 finding. A file register an
entry carries keeps its generated `index.md` in the entry, as a milestone's slice specs do.

### An entry of a declared Directory register carries `path`, and its directory and README are named by the entry's reference `##entry-anchor-path-kind`

A file inside an entry, other than its README, is no entry of any register: it is a subdocument of
the heading register the entry carries, so a `path` is its only name, `path@<entry>@<file>`. The
rival is the plan anchors' rule, which carries no `path` kind because every plan document has a
kind's name, per `design@core@plan-document-kinds`; a subject's file has none. The entry's directory
and its README are the entry itself, so a `path` or a `register` reference to either is a finding
whose repair names `<register>@<parent>@<id>`, per `design@core@path-to-a-home-is-reported` and
`design@core@register-home-reference`: one directory has one name, so `show` finds every citation.

### A file register's index is a banner, a count and one row per entry, and its bytes are the contract `##a-file-register-index-is-rows`

The generated `index.md` of a file-register instance, in order:

| line | content |
| --- | --- |
| 1 | `**Generated — do not edit.** \`<command> index\``, with the project's declared command, per `design@core@declared-command` |
| 2 | blank |
| 3 | `<n> entries`, over every entry of the instance, grouped or not, and `1 entry` where the instance holds one |
| then | the ungrouped entries' table, under no heading |
| then | per group, a blank line, `## <group>`, and that group's table |

A table's columns are the register's declared metadata keys, **ordered by name**, then `title`
holding the entry's level-one heading as a link to the file, relative to the index's own
directory. Rows sort by the first metadata key and then by id, or by id alone where the register
declares none. Groups sort by name, and the ungrouped entries come first.

**A `|` inside a cell is escaped as `\|`.** It is the one character of a title or of a metadata
value that would otherwise end the cell and shift every column right of it, turning a listing into
a table that says something else. Nothing else in a cell is rewritten.

**The bytes are a contract because `generated` compares bytes.** Any change here fails the gate on
every committed index at once, which is the cost of the comparison being exact; the gain is that a
listing cannot drift by a character without being named.

**Rows only: no summary and no date.** A row changes on create, delete, retitle, regroup and a
metadata change, and on nothing else, so an index is regenerated rarely. A summary column would
restale it on every wording edit, and a date would break
`design@core@generated-files-are-pure` outright. Both are `rejected@core@index-summary-column` and
`rejected@core@index-last-change-date`.

**A Directory register's index has the same bytes with one column before the title, `id`**: one
row per entry, its id, then its README's level-one title linking `<id>/README.md`, sorted by id. Its rows come from the
entries, which are anchors, so no file inside an entry is a row; the entry's own index lists those.

**The rows come from the walked entries, not from the instance's declarations.** A group is the
subdirectory an entry sits in, so an undeclared group still lists its entries and a declared group
holding none prints no heading; an entry whose id, title or metadata the shape check refuses is
still listed, under its basename and an empty cell. A listing that hid what the directory holds
because a declaration disagreed would be a listing that stops listing, and the declaration is
`check::registers`' to report.

The design this decision did not keep: `rejected@core@lines-flag-on-interpretation-index`.

### A generated file's content is a function of the walked tree `##generated-files-are-pure`

No generated file holds hand-written bytes: not a file-register `index.md`, and not a file an
extension generates. Each is what its generator returns over the model, which is what lets
`path@core@src/check/generated.rs` verify one by regenerating into a `String` and
comparing rather than by writing the file and reading it back.

**The purity is a property to preserve, not one to observe.** A generator that embedded a
timestamp, a hostname, or anything the walk does not see would break both consequences at once: the
check would report a file stale that nobody had changed, and `index` would rewrite on every run.

### `index` takes no dry-run flag, since writing a generated file destroys nothing `##index-takes-no-dry-run`

**Writing a generated file destroys nothing**, since its content is a function of the walked tree,
per `design@core@generated-files-are-pure`, so `cargo klarch index` takes no dry-run flag. Its one
flag, `--staged`, chooses the tree and the destination, per `design@core@index-staged-write`. The
dry run a write command normally owes exists because a write can lose something, and here it
cannot: the worst a run can do is replace a file with what the tree says that file is. It writes
only where the bytes differ, so running it to look moves not even an mtime, and it names each file
it rewrote.

**That last claim is held by two refusals, not by the paths being generated.** `fs::write` follows
a symlink and writes through it, so a generated path that is one would replace content this
command never produced; and a destination directory the manifest declares but the tree does not
have is a manifest defect rather than something to create. Both are checked over **every**
destination before any is written, so a run either refuses having written nothing or writes them
all. Remove either check and the paragraph above stops being true.

### The writer and the `generated` check take the generated files from one list `##one-list-of-generated-files`

**One list names the file-register indexes, and the writer and the gate both read it.**
`cargo klarch index` and the `generated` check take the same pairs of destination and expected
bytes from one function, so neither can generate one the other does not know about, nor disagree
about what is in it. Each is derived from the file-register instances, one per instance whose
directory is there. **An extension's generated files are read the same way**: the extension
names their paths before the walk and renders their bytes when prepared, and the writer and the
gate both take them from those two calls, per
`design@core@an-extension-plugs-in-through-phased-hooks`.

**An instance with no directory contributes no index**: generating into it would create a register home as a side effect of a listing, and the
missing home is what `check::tree` reports.

### One question has one command that answers it `##one-question-one-command`

Verifying is not `index`'s question. The `generated` check of `cargo klarch check` is the gate,
and it names the first line at which the committed file and the regenerated one disagree. A second
command answering the same question in its own format is what drifts, and there would be nothing to
say which of the two was right.

The design this decision did not keep: `rejected@core@index-diff-with-write-flag`.

### `anchors` lists every anchor of a project, computed as `check` computes them, with the home of each register it carries but a plan's `##anchors-listing-command`

`klarch anchors` prints every anchor, the constructed ones included, built by `Anchors::of` over the
manifest and the tree's listing as every check builds them: its name, its kind and its path, and,
under every anchor but a plan's, each register it carries with the home the tree holds,
the file, the directory, both, or `missing`. An entry's kind is the Directory register it is an
entry of. A milestone and a spec list no register: a plan
anchor's registers keep their homes inside the plan itself, its sections, or a milestone's own
directory for its slice specs. A home is judged held by the rule `check` judges it by, over the
same listing, so the two agree on what a project carries. It exits 1 when a home is missing, a negative answer
about the project, per `design@core@exit-code-ladder`. Its output is for a reader and is not a
contract, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

It answers a question no other command answers: `model` prints observations, and `show` one
register's homes. The rival, each project's root `CLAUDE.md` listing its anchors by hand, goes stale
when the manifest changes, and no check reports the stale list; the command is computed from what
it describes. The primer names it, so every installing project's sessions learn its anchors from
it.

### The record `klarch worklist` reads is every entry of every register but the plan registers, and its edges are the references written inside an entry `##record-is-every-register-entry`

The record holds every entry of `goal`, `design`, `rejected`, `issue`, `tripwire` and of every
register a project declares, in every anchor, a location's included. An edge is a reference written
inside an entry's own text, the section of a heading entry or the file of a file entry, to another
entry; a home's prose outside every entry yields none, and membership reads an edge either way. Plan
documents and their items are no part of it: they are deliberation, and leave when their work lands.
Agent-facing text is no part of it either: a decision stated by one text of it earns no head under
entry test 2 of `primer@design-heads`, its reader is the session that edits that text and reads it
whole, and the installed text cites no entry, so in an adopting project it would add nodes with no
edges. The command relies on the checker working as intended: the edges are the references the
checker resolves. It serves `goal@core@records-reach-their-reader`, a listing computed from the tree.

### Every entry of a unit register is in one unit, cut by the headings or the groups of its home, and a cluster is rooted at a unit, never at a goal `##cluster-rooted-at-a-subject`

A unit register is `design` or any register a project declares. Its entries are grouped into units:
a heading register whose entries sit at level 3 or deeper by the nearest heading above each entry
that is shallower than the entries, in a single file or in each subdocument of a directory home,
each such heading a unit of its own, so a design home's units are its level-two subjects and the
entries under its title; a heading register at level 2 by file; a file register by its group
subdirectories, per `design@core@groups-are-subdirectories`, and the top of its home; a Directory
register by the anchor that carries it. A cluster is a unit with the `rejected` entries its entries
cite, the issues and tripwires that cite them, and the goals of `design@core@goals-join-a-cluster-as-context`.

It serves `goal@agent-skills@installed-text-works-anywhere`, since every unit comes from what the
manifest declares and the tree holds. A subject groups what the work reads together: the owner
chose that grouping for design entries, since a subject's kept and rejected entries have to be read
together, and a standing entry joins the cluster of the heads it cites, since it is judged beside
them. The nearest rival, a cluster rooted at a goal, reached 342 of the 677 entries of this repository
at ed9dd60 and 208 of the 801 of thaum at its commit edd2cb5d, since 213 heads here cite no goal, and
the reference rule of `design@agent-skills@a-reference-claims-a-revisit` has a head name a goal only
where it derives a constraint from it; rooted at a subject, the clusters reached 663 and 706.
Followed transitively through design to design citations, a goal-rooted cluster held 466 to 480 of
the 677 entries, since 159 heads cite one another in one cycle. The counts were taken by scripts over
`klarch model`, which the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds. The design this decision did not keep:
`rejected@core@goal-rooted-clusters`.

### A goal joins a cluster as context, with every goal it refines, and brings in no issue or tripwire `##goals-join-a-cluster-as-context`

A cluster holds the goals its unit's entries cite, and every goal those cite in turn, which are the
goals they refine. A goal brings in nothing: an issue or a tripwire that cites only goals is no
member of a cluster through them. The rival, goals bringing in their citers, made every citer of a
root goal join every cluster whose heads cite that goal: the reads rose from 839 to 1323 for 663
entries, over this repository at ed9dd60. The design this decision did not keep:
`rejected@core@goals-pull-their-citers`.

### Every entry no cluster holds is in the residual group of its anchor `##residual-grouped-by-anchor`

One residual group per anchor that has any, packed with the clusters under the bound of
`design@core@clusters-packed-under-a-word-bound`, and named when it exceeds it. On the owner's
argument, the standing entries were already cut into groups before, and the entries of one anchor
share the anchor they apply to. A single residual group for the project grows with every entry no
head reaches: issues about code with no head, issues citing only goals, goals no head serves, and
tripwires guarding a single-site instruction, each a shape the reference rule admits, which made
14 entries here at ed9dd60 and 55 in thaum at edd2cb5d. The packing answers
`rejected@agent-skills@one-search-agent-per-anchor`, whose reason was that an anchor bounds nothing.
The design this decision did not keep: `rejected@core@one-residual-group`.

### No rule on the direction of a citation is checked `##no-citation-direction-rule`

Membership of a cluster reads an edge either way, so a rule that citations run from goals down to
issues would buy acyclicity alone, which serves a transitive membership that merges the clusters.
The design this decision did not keep: `rejected@core@citation-direction-checked`.

### `worklist --out` writes the agents' entries and the report skeletons into a directory outside the project, and `worklist --verify` judges the filled reports against the list it wrote `##worklist-writes-and-verifies-the-reports`

`klarch worklist --out <dir>` writes `agents.txt`, one line per agent and entry with its role and
its seed mark, and per agent `entries.md`, its entries whole, and `report.md`, one line per entry
to grade. It is a writer, so it refuses over an incomplete model, per
`design@core@phases-gate-the-report`, and it refuses a directory inside the project, whose files the
walk would read, a directory that holds anything, and a seed that names no entry, writing nothing.
`klarch worklist --verify <dir>` reads no model: it judges each report against `agents.txt`, and
reports an entry graded nowhere or twice, a reference no agent was given, a grade or a relation
outside its role's set, a missing reason, an entry that is or cites a seed graded `unrelated`, and
a report with no closing section on what was met outside the task. The grades and their relations
are `design@agent-skills@search-relevance-grade`'s.

It serves `goal@core@records-reach-their-reader` and `goal@core@declared-instructions-are-checked`:
an instruction to grade every entry once is kept by a command, not by a reader. The nearest rival,
each searcher deriving its group from the listings and writing its report by hand, listed one entry
under two lists in the discussion that produced the `rejected` register, and only the searcher's own
correction, appended to its report, showed it. The command's output is read by its own `--verify`
and the searcher, and is no contract with another reader, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`.

### The groups are packed into the fewest agents first-fit-decreasing gives, spread evenly, under a bound of 25,000 words, a group above it alone `##clusters-packed-under-a-word-bound`

A group's load is the words of its entries. A group above the bound goes to an agent alone, never
split, and `worklist` prints a notice naming it. The others are packed first-fit-decreasing, which
fixes the number of agents, then dealt again over that many agents, each to the least-loaded agent
it fits in; where one fits in none, the first-fit assignment stands. `--bound` sets another bound.
It serves `goal@knowledge-architect@agents-get-a-complete-workflow`: each agent's reading is bounded
whatever the project's size, and no agent is spent on a remnant.

The owner set the bound at 25,000 words, on the owner's judgement that loads of about 22,000 words
put not much of a strain on this repository's searchers, and holds raising it to 30,000 open for
later; no measurement of recall stands behind any value, and real sessions measure it, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`. Words rather than entries or
bytes: in the owner's estimate, words translate a bit better to an agent's tokens; an entry's mean
size ranged from 0.5 KB, a `rejected` entry, to 2.9 KB, an issue, over this repository at ed9dd60,
so a count of entries bounds a load poorly, while words and bytes differed by about 15% across
kinds. First-fit-decreasing uses at most 11/9 of the optimal number of agents plus 6/9, by the bound
György Dósa proved tight in 2007; over this repository at ed9dd60 it used the lower bound, 9 agents,
and over thaum at edd2cb5d it used the lower bound for the groups within the bound, 8 agents, beside
3 groups above it sent alone. The spreading keeps an agent from carrying the full bound while
another carries a remnant: first-fit alone filled this repository's agents to a median of 24,737
words of a 25,000 bound, as the scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds took it. Re-take the packing with `klarch worklist --out` and `--bound` over this repository; a run whose
agents sit at the bound reopens the spreading. Splitting a cluster to meet a bound would cut a
subject, which the grouping exists to keep whole. The designs this decision did not keep:
`rejected@core@packing-to-the-bound`, `rejected@core@bound-in-entries`,
`rejected@agent-skills@one-agent-reads-every-standing-entry`,
`rejected@agent-skills@design-listing-cut-in-groups-of-sixty` and
`rejected@agent-skills@subject-groups-bounded-at-sixty`.

### `index --staged` writes the staged tree's generated files into git's index, and no working-tree file `##index-staged-write`

`cargo klarch index --staged` generates each file from the tree git's index would commit, per
`design@core@staged-tree-source`, and sets the staged entry of each whose staged bytes differ, or
that the index lacks, leaving the working-tree file as it is. After it, `check --staged` passes on
what will be committed, and plain `check` judges the working tree as before. It serves a commit
of part of the working tree when an unstaged change touches the same register: the `index.md` that
commit needs then differs from what `index` writes, and without this command the repairs are a
hand edit of a generated file or moving the unstaged file away.

- It gates the staged tree on phases 1 to 3, per `design@core@phases-gate-the-report`, and refuses,
  having staged nothing, a destination the index holds as a symlink, a gitlink or a directory, asked
  of the index itself so that no walk row hides it, or whose directory the staged tree does not
  hold. A symlink's or a gitlink's mode would carry the generated text, and `--index-info` would
  replace the entries under a directory.
- It stages every entry in one locked write, so the index changes for all of them or for none: a
  held lock is exit 2 with the index as it was. How, and what a refused run can leave in the object
  store, is the doc comment of `stage_generated` in `path@core@src/git.rs`.

It writes into git, where `design@core@safe-fix-definition` refuses a fix that touches git. That
test is the one `--fix` applies before a fix, and `--fix` never applies this one, per
`design@core@fix-refusal-mixed-state`. What it writes is the staged entry of a generated file,
whose bytes the staged tree determines, so no bytes a writer meant are lost. The rival, writing
the staged rows into the working-tree file, lost: after it, plain `check` fails, since that file no
longer matches the working tree, and `check --staged` fails until the file is staged.

The design this decision did not keep: `rejected@core@index-staged-writes-working-file`.

### `check --fix` applies the safe fixes `design@core@fix-scope` admits, then runs the full check `##check-fix-flag`

`cargo klarch check --fix` applies the safe fixes `design@core@fix-scope` admits, lists each file it
wrote or removed, then runs the check, whose report and exit code are the run's. It makes an edit cycle
one command: updating the generated files is needed after nearly every edit of a register, and
without the option it takes `index` then `check`. The option came from the owner: "it's quite
common for tools similar as this one to provide a "quick fix" option, that performs the fixes it
can safely do automattically", as `cargo clippy --fix` and `eslint --fix` do, and the same words
lean "toward using a generic name for the cli option". Without `--fix`, `check`
is unchanged, and `index` and `install-agent-skills` stay as commands, each for one fix alone. The
option takes a generic name because a later safe fix needs no new option. The rival, a separate
`fix` command that writes and then runs the check, lost: with the option, the check stays one
command rather than two that end in one report. `commits` judges history and takes no `--fix`,
and the gates run `check` without it, so continuous integration judges the tree as committed. It
refuses a partial commit's mismatch, per `design@core@fix-refusal-mixed-state`, and `--staged`,
per `design@core@fix-with-staged`.

### `check --fix` refuses where a generated file it would write differs from the staged tree's, not wherever the tree holds staged and unstaged changes `##fix-refusal-mixed-state`

**Where the project's part of the index differs from HEAD, and a generated file `--fix` would
write differs from the one the tree git's index would commit needs, `--fix` writes no generated
file.** It names each such path, a path the staged tree does not need included, with the two
repairs: `index --staged` then `check --staged`, to commit the staged changes, and `index`,
to fix the working tree as a whole. `--fix` cannot lose unstaged content, since it writes only
generated and installed files; the hazard is a wrong commit, since the `index.md` it writes
reflects the working tree, and staged with a partial commit it makes a commit whose tree fails. A
file already current on disk is not written, so no refusal names it: the hazard is in what `--fix`
writes, and refusing a run that writes nothing would refuse one that `index`, the repair it names,
cannot clear. Where no staged tree is there to commit, an unmerged index, a staged tree holding no
manifest, as one where the manifest is written and not yet staged, or a staged tree stopped in
phases 1 to 3, nothing is compared and the fix runs. Any other failure to compute the staged
side refuses, naming it, such as an extension that cannot prepare over a snapshot: a comparison
skipped in silence lets the partial commit through, per `design@core@a-failed-parse-is-loud`. The
refusal is exit 2, or 1 where agent files were installed before it, since 2 promises an untouched
tree.

A refusal wherever the tree holds both staged and unstaged changes loses: it also fires where
the unstaged changes touch no input of a generated file and the fix is correct for both trees,
which blocks the edit-then-`--fix` loop for nothing. A printed note in place of the refusal loses: a
line in a long report is missed where an exit code is not.

### `--fix` with `--staged` is refused while parsing `##fix-with-staged`

The refusal is clap's `conflicts_with`, per `design@core@arguments-parse-through-clap`. `--fix`
repairs the working tree. Combining them would tie every fix to a write into git's index. The
owner's words, given with the ruling on `index --staged`, and hedged: "This is not definitive, but
I would not recommend to use this with --fix though. The reason  is that I'd like to provide more
types of quick fixes later, and this might not be compatible."

### A fix is safe when its bytes are determined by the tree and the pinned version, and it writes or removes only files of the installer's namespace or of the generated list `##safe-fix-definition`

This is the test every fix must pass before `--fix` applies it. The installer's namespace counts
whole, per `design@core@owned-namespace-check`: an unshipped file the check reports there is
removed, as the install removes it, whoever put it there. The rival, removing only the files the
install itself wrote, has nothing to tell them by: ownership is decided by a name, with no record
and no history, per that head. A fix that makes a choice, or touches
git or a hand-written file, would rewrite what a writer meant, so its repair stays the reader's.
The test is what keeps `--fix` safe to run after every edit, which serves
`goal@knowledge-architect@agents-work-without-drift`. `tripwire@core@fix-makes-a-choice` watches
it.

### `--fix` applies only the kinds of repair added to it on purpose, each passing the safe-fix test `##fix-scope`

`--fix` applies a repair only when it passes `design@core@safe-fix-definition`. Passing the test
makes a repair admissible, not owed: each kind of repair `--fix` applies is a member added on
purpose. The direction of the rule is the owner's, given against a title stating that `--fix`
repairs every finding whose repair passes the test: the owner put the rule the other way round,
`--fix` admitting a repair only when it passes the safe-fix test, since otherwise it becomes a
source of defects where the project may never have meant to cover some repairs.

`--fix` applies two kinds of repair today: an installed file missing, differing or no longer
shipped, and a stale or missing generated file. Every other repair touches git, such as staging a
deletion, or a hand-written file, such as the root CLAUDE.md's primer import, or is a choice. The
installed set is repaired exactly where the installed-file check reports it, judged from
git's listing after normalising line endings: a file git does not list, such as an ignored file in
the namespace, is never touched. The generated files alone, the rival, lost: repairing the installed set costs little, and it removes the
install-then-index sequence from an upgrade.

The design this decision did not keep: `rejected@core@fix-converting-old-entries`.

### A fix run repairs the installed files, gates the model, refuses a partial commit's mismatch, writes the generated files, then checks `##fix-before-the-checks`

The order of a `--fix` run: `--fix` with `--staged` is refused while parsing; a manifest holding a
refused declaration writes nothing; the installed files are repaired; phases 1 to 3 run over the
tree as the repairs left it, and a finding there stops the run, printed as `check` prints a stop,
with nothing more written; a partial commit's mismatch is refused, per
`design@core@fix-refusal-mixed-state`, after the installed repairs, whose bytes are the same for
both trees, and after the gate, whose model the comparison reads; the generated files are
written, from the list `index` reads, after every destination is checked; the full check runs. The
installed files' bytes do not depend on the model, so they are repaired before the gate, while a
generated file is never written over an incomplete model, per `design@core@phases-gate-the-report`.
An upgrade that removes a shipped file takes two runs: the deletion is unstaged, phase 2 reports
it, and staging it touches git. The rival, `--fix` treating the deletions its own install just made
as done, adds a special case to the phase gate. The cost of two runs is small by the owner's
weighing: the owner judged deleting or renaming a skill rare, and several invocations to handle it
not much of a problem. A failed
write exits 2 with nothing written, and 1 after any write, since 2 promises an untouched tree. The check stays the only verifier: `--fix` only writes
before it verifies, so the rejected design of an `index` that prints a diff and a `--write`
that applies it, a second command answering whether a file is current in its own format,
`rejected@core@index-diff-with-write-flag`, is not reopened.

## 3. Names and references

### Every citeable thing is an entity with a kind, an id, a definition site and, where its kind takes one, an anchor, held in one table built from the model, and every check that resolves a name reads that table `##one-entity-table`

**A kind is a register's name, or `path`, or `planned`, or `register`, or, under the `claude`
harness, a harness kind**: `skill`, `agent`, `primer` or `instructions`, which is no register and takes no anchor, per
`design@core@harness-kinds`. The eleven built-in registers give the kinds `design`,
`goal`, `tripwire`, `rejected`, `issue`, `spec`, `milestone`, `thread`, `argument`, `criterion` and
`acceptance`, and a project's own declarations give the rest, so the kind set is
data rather than a compiled enumeration. A heading register's entities are slugs defined in the
register's home under an anchor, per `design@core@an-entry-is-a-heading-at-the-register-level`; a file register's are the
files under its instance directory, one per entry, per
`design@core@a-file-register-is-a-directory-of-entries`. `path` is defined by the tree itself and is
resolved against the survey, under the same anchors and with the candidate rule and segmentation
the table's resolver applies to every kind, and `planned` with it, per
`design@core@planned-path-form`. `register` names a register's home and is resolved against the
anchors, under the same candidate rule and segmentation, per
`design@core@register-home-reference`. The
table lives in
`path@core@src/entity.rs`, is built once per run from the model, and is what
`check::references` resolves against.

**An anchor is a named directory carrying registers**, per
`design@core@anchors-are-components-and-locations`. It carries its register list and its home base
as data, so a component and a location are two constructors and not two resolvers. The deepest
anchor owns a document, as `design@core@an-entity-belongs-to-its-anchor` states.

**One table, because a notion of a name held per check drifts.** With one table there is one
resolver, one dangling check over every kind, one command that can print any entity given its
reference, and a one-pass rewrite of every reference into a link when the documents are
published. A check that resolves a name of its own is the shape this refuses: two resolvers
disagree the first time one of them is edited, and only the entities one of them knows can be
printed or rewritten.

### Each heading register declares the one heading level its entries sit at, and every heading at that level in its home carries a slug `##an-entry-is-a-heading-at-the-register-level`

**The level is part of the register.** `design` and `rejected` sit at level three, `goal` and
`tripwire` at level two, compiled in; a declared heading register states its level with the `level` key of its
`[registers.<name>]` table, from 2 to 6, and a file register takes none. Level one is the
document's title. With a declared level, "every heading at level N is an entry, and only those"
is a checked invariant, so the tool can tell a heading that is no entry from an entry whose slug
is missing. Without it, a tripwire written without a slug is defined nowhere, listed by nothing
and reported by nothing.

A heading at another level is section text and owes nothing: a design home groups its
level-three decisions under level-two subjects.

**The homes are the design, rejected, goals and tripwires homes of the owning anchor**, and the home of
any heading register the project declares, each in either shape of
`design@core@heading-register-two-shapes`: the single file, or a subdocument of the directory.
The directory's `README.md` is the head: it defines nothing and owes no slug, for every register
and not only design. A plan's items are the one register family read otherwise: their home is the
plan's own documents, a milestone's README included, and the section a heading sits under gives
its kind, per `design@core@items-as-section-registers`. Which anchor owns a definition is where its document sits, per
`design@core@an-entity-belongs-to-its-anchor`.

**The section homes of the harness keep the same rule at level two**, a skill's, an agent's, the
primer and the root CLAUDE.md, per `design@core@section-homes-carry-slugs`: their entries are
the harness kinds', not a register's, and a slug at a level-two heading there defines a section.

A heading at the register's level with no slug in its home is reported, naming the heading. An id
has two segments or more, per `design@core@two-segment-ids`.

Two definitions of one id in one register instance are a finding at each site, each naming the
other.

**Which homes an anchor must carry is `check::tree`'s question, not the table's.** The
table defines from every home shape of every heading register the anchor carries, and reports a
definition that sits where none may; whether the home is there at all, and in which shape, is
`check::tree`'s.

The design this decision did not keep: `rejected@core@slug-at-any-heading-or-cell`.

### Every id the checker defines has two segments or more `##two-segment-ids`

An id is `[a-z0-9]+(-[a-z0-9]+)+`: a register entry's id, a plan document's, a slice's, a plan
item's, a skill's or an agent's name and a section slug, at every site that judges one, the
shipped text's section placeholders included. A slug of one segment is reported where it is
defined, with the repair "rename it with a second segment", so a heading carrying one reads as an
entry with a wrong slug rather than as one with none. A plan document numbers its argument items
`arg-1`, `arg-2`, as `skill@knowledge-architect-planning@spec-contents` states. A register's name
and its directory keep the grammar of one segment or more: they are the kind and the home a
reference names, and no lint on ids reports one as an id.

**The rule is the owner's proposal.** Two segments keep a defined id from being an ordinary word,
so the lints on ids, `design@core@backticked-bare-id` and `design@core@plain-word-lint`, report a
pointer and not a word: in the census of the design discussion, the one-word ids of thaum, its
issues named after a feature area and its argument items, drew most of its plain hits and its
ordinary words in backticks. A rename of the id is a migration, which costs a consumer no capability, per
`design@knowledge-architect@capability-cost-before-migration`: a two-word id
that names the subject without a claim, such as `combat-coverage`, keeps the stability a feature's
own name had. The nearest rival, one-word ids exempt from the lints, leaves every mention of such an
id unchecked for good: `rejected@core@one-word-id-exception`. A second rival, plan items exempt
from the grammar since they are scoped to their plan, adds an exception justified by nothing but
that scope: `rejected@core@plan-items-exempt-from-the-grammar`. The census figures are those of
the design discussion, kept in its spec, `path@elsewhere@docs/plans/specs/defined-ids-outside-references.md`. They are re-taken over a
tree that still holds one-word ids, as a project's at its upgrade, by counting the plain words and
the backticked words equal to each, `git grep -nwF` on each id, since the lints read ids of two
segments only. This serves `goal@knowledge-architect@documentation-stays-consistent`.

### An entry's heading puts its statement before its slug `##statement-precedes-the-slug`

**The statement precedes the slug in the heading**, so a document outline lists the entries
rather than a set of identifiers, and an editor's outline view is the index.

### A slug is read wherever it sits in its heading `##a-slug-is-read-anywhere-in-its-heading`

**The slug may sit anywhere in the heading.** A pattern that requires text after the slug matches
no heading carrying nothing but the slug, and every reference to such an anchor is then reported
as dangling while the definition sits in the file.

### A slug anywhere but at a heading of its register's level in the register's home defines nothing, and is reported as a misplaced definition `##a-slug-elsewhere-defines-nothing`

A slug anywhere but at a heading of its register's level in the register's home defines nothing
and is reported as a misplaced definition: at a heading of another level, level one included, in a
table cell, at the head of a plain line, in the middle of a line, as a second slug on a definition
line, in a file that is no register home and no section home of the harness — a Rust comment
included — or in a directory home's README.

- **A table cell defines nothing.** A decision written as a row has no heading, so the outline does
  not list every decision, and one register would have two definition sites.
- **A slug at the head of a plain line is reported, not accepted.** It is neither a definition
  nor a reference, so every pointer at it dangles and the site itself is named. Accepting it
  beside the heading form would leave the two indistinguishable, and a project moving off the
  line-head form could not tell which of its slugs are left to move.
- **A mid-line slug is a pointer written in the definition form**, and recording it is what makes
  such a pointer visible: a census found such pointers in thaum's design homes, checked by
  nothing, in the message of 86adaf6.

**A fenced heading is an illustration, so a fenced slug neither defines nor is misplaced.** A
definition site is a heading, and the scanner already reads no heading inside a fence; the one
stance covers both. This is the one place a fence still suppresses anything in the grammar —
references are live in a fence for every kind, per `design@core@a-reference-is-live-in-every-prose`.

The design this decision did not keep: `rejected@core@table-cell-definition-site`.

### A heading is a line that opens with `#` marks, and a heading markdown reads in any other shape is reported `##headings-open-with-hash-marks`

The checker reads a heading where markdown reads an ATX heading at the start of a line: one to six
`#` marks, at most three spaces in, then a space or a tab, then text. A line indented four spaces
or more, or by a tab, opens no heading, so a slug on it is a mention and defines nothing; so does
a line whose marks are followed by any other character, a no-break space included. Every other
heading markdown reads is reported in phase 2, at its first line, in every markdown document of
the walk: a setext heading, a line underlined with `=` or `-`; a heading that follows a list
marker or a block-quote marker on its line; and a heading with no text. The repair is to write it
as a line that opens with `#` marks and holds text.

The opposite direction does not hold yet: a heading-shaped line inside an HTML block or a metadata
block that markdown does not render as a heading is still read as one, per
`issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`.

- **It serves `design@core@an-entry-is-a-heading-at-the-register-level`**, which rests on the
  checker reading headings as markdown does: an entry is a heading. A heading markdown shows and
  the checker does not read is a section no check sees. A slug on it defines nothing, and in a plan
  document the items under it are read under the section before it, which changes their kind.
- **Phase 2, because the model is incomplete**, per `design@core@phases-gate-the-report`. A
  finding of the entity table computed while such a heading stands is unreliable in both
  directions.
- **Reported, not read.** The scanner reads one line at a time. A setext heading's level is known
  only at its underline, which follows its last text line, and a heading inside a container starts
  after the container's markers. One shape per heading keeps the scanner line by line, and keeps
  every heading findable by a search for lines opening with `#`.
- **A Rust comment holds no section**, so a comment line underlined with dashes is not reported.

### An entity of a register belongs to its anchor and is unique inside one register instance, and a reference to it names all three: `` `<kind>@<anchor>@<id>` `` `##an-entity-belongs-to-its-anchor`

A definition is owned by where its document sits — the deepest anchor whose path holds it, and
the register whose home holds it — rather than by anything the line says, so moving a document
moves the entities in it. Two anchors may therefore each record an entity they call the same
word, and two registers of one anchor may too, which is what naming the kind and the anchor in a
reference buys. The id of a heading-register entity is called a slug.

**A harness kind names no anchor**, per `design@core@harness-kinds-cited-without-anchor`: its entities live
where the harness puts them, one namespace per kind, so an anchor would carry no information. The
refusal of a reference that names no anchor holds for every other kind.

The designs this decision did not keep: `rejected@core@project-wide-unique-slug`,
`rejected@core@implicit-own-component-reference`, `rejected@core@hash-reference-separator`,
`rejected@core@configurable-reference-separator` and
`rejected@core@register-reference-without-anchor`.

### A reference that resolves to nothing is reported, with the repair its failing segment needs `##an-unresolved-reference-is-reported`

**A reference resolves to nothing in five ways, and each is reported as the repair it needs.**
The kind position holds an anchor, the old form, and the repair names the kinds; the anchor is
not declared, and the repair lists the anchors; the anchor is declared and carries no register of
that kind, and the repair lists the anchors that do; an item of a plan is cited from outside that
plan, and the repair names the whole document, per `design@core@plan-item-scope`; the anchor
carries the register and does not define that id, and the reference is dangling. One finding for
all five would send a reader to check the wrong segment of the pointer most of the time. The third
way is reached by any anchor that does not carry the register: a location that does not declare
it, or a component naming an opt-in register, as the `spec` register only the anchor `plans`
carries. A kind resolved outside the table has ways of its own, each reported as the repair it
needs: `path` and `planned` against the tree, per `design@core@every-path-names-its-anchor` and
`design@core@planned-path-form`, and `register` against the anchors, per
`design@core@register-home-reference`.

**A reference that resolves to nothing is recorded and reported, never dropped.** The scanner
could require a resolvable shape and see nothing without one, which needs no finding and no
migration. It would also mean every pointer written in an older form stops being checked with
nothing saying so. A silent false negative passes a tree that does not conform, against
`goal@core@declared-instructions-are-checked`: a tree that passes the check conforms to all of it.
The candidate rule that bounds this is `design@core@at-span-candidate-rule`.

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

### Every path reference names its kind and its anchor, `` `path@<anchor>@<path>` ``, and the deepest anchor wins `##every-path-names-its-anchor`

A checked path reference is one backticked span in the one grammar of
`design@core@an-entity-belongs-to-its-anchor`, with `path` as its kind and the path under the named
anchor's directory as its id. The root is a component like any other, named by `project.name`.
The kind prefix costs five characters at every path reference and buys one grammar: the scanner
has one tokenizer, the old two-segment form `<anchor>@<path>` is reported as an anchor in kind
position by `design@core@at-span-candidate-rule`, and the unanchored lint is left with
one job. There is no unanchored form for a path of this tree: a backticked span of path characters
with two or more segments and no `@` is a finding naming the grammar when its first segment names a
file or a directory this tree's listing holds. One segment is a name rather
than a path, though the exact name of a skill or an agent is reported, per
`design@core@backticked-bare-id`, and a span holding a space, an angle bracket, or a colon anywhere but in a line
suffix is not path-shaped, which is what lets documentation of the syntax show a placeholder with no
carve-out.

**A span whose first segment names nothing here is another tool's notation, and is silent**: a
media type `application/json`, a unit `km/h`, a git ref `origin/main`, the fixture path of a test,
a path on another machine. The first segment is read from the root, from every anchor that is a
directory, and from the document's own directory, a `..` climbing from each; a leading slash is
read from the root alone. A spec is an anchor and a file, so it is no place a path is read from.
Only the first segment is asked, so a pointer whose first segment is here and whose rest is not is
still a finding, and the anchored form it is rewritten in reports it dangling. A commit message is
judged against two trees, per `design@core@a-commit-message-is-a-document`.

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
it is in the commit that rewrote this head, found with
`git log --grep='only where it names something of this project'`. Reporting every path-shaped span
is `rejected@core@every-path-shaped-span-reported`. This serves `goal@knowledge-architect@any-project-can-adopt-it`
and `goal@knowledge-architect@adoption-is-easy`.

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

**The path is plain**: `..`, a `.` segment and a leading `/` are refused. An upward path is
anchored at the wrong place by definition, and it is the shape that breaks when the referencing
file moves. The id is everything after the second `@`, so a path may itself hold an `@`.

**The deepest anchor wins, and inside means a proper descendant.** A reference whose target
sits inside another anchor is refused with the right anchor named. What this buys is the
same property `design@core@an-entity-belongs-to-its-anchor` buys for every entity: relocating an
anchor edits its one line of the project's `knowledge-architect.toml` and no document, which
`goal@core@relocation-is-one-manifest-edit` asks, and one fixed-string grep per anchor is an
anchor's complete inbound-reference list. An anchor's own directory is the one target with no
spelling under its own name, so it is named from an ancestor — a reference that names a
location, which a move is expected to break.

**A plan document is cited by its kind, never by its path**, per
`design@core@plan-document-kinds`. A
`path` citation of a spec, of a milestone directory or of a file inside one is refused from every
anchor, and the finding names the form that resolves: `spec@plans@<id>`, `milestone@plans@<id>`
or `spec@<milestone>@<slice>`. It is judged by where the target sits, before the deepest-anchor
rule and whether or not it exists, so a citation of a deleted plan gets the same repair. This
overrides the ancestor's spelling of an anchor's own directory for a milestone. A plan anchor, a
milestone or a spec, carries no `path` kind. One name per document is what keeps `show` complete: a citation written as a path
is one `show spec@…` would not list. The README of the plans directory, and the README and index
of each of its two homes, are not plan documents, and are cited `path@plans@<file>`.

**A fenced path reference is live**, as every reference is, per
`design@core@a-reference-is-live-in-every-prose`: a sketch names its paths on purpose, and an
illustration that needs a fake path writes the escape anchor or an angle-bracket placeholder.

The designs this decision did not keep: `rejected@core@bare-root-relative-path`,
`rejected@core@path-without-kind-prefix`, `rejected@core@at-prefix-root-relative-path`,
`rejected@core@retired-form-rs-suffix` and `rejected@core@ignored-first-segment-reported`.

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

### The escape for a path this tree does not hold, under `path`, and `*` for every component's own copy, under `path` and `register`, are reserved anchor words `##reserved-anchors`

**The escape anchor** — the word elsewhere in anchor position, `path@elsewhere@<path>` — marks a
path deliberately not resolvable here: a surveyed engine's layout, a deleted file a tracker
entry discusses, a hypothetical location. It is scanned and counted like any reference, so a
typo'd escape is still a registered pointer; it is exempt from the existence, kind and shape
assertions, because a foreign layout may spell anything. One assertion runs against it: a
target that DOES resolve here, beside any anchor, is a finding — without it, the escape is the
cheap way to silence the unanchored finding on a real path.

**A file of another project is written with that project's name as its first segment**, as
`path@elsewhere@<project>/<path>`, and the refusal's repair names that form. A foreign file often
shares its relative path with a file here, such as a dependency's src/lib.rs, and without the
prefix the assertion above refuses it. A repair that names only the local anchor leaves the writer
of a foreign pointer no checked form, and prose follows. The prefix is no cheap silence: it is false of
every file in this tree, so writing it before a local path is a deliberate misstatement, which costs
what removing the backticks costs, and that move is refused as a repair by
`design@agent-skills@plain-text-is-no-repair`.

**The generic anchor `*`**, `path@*@<path>`, marks each component's own copy of a path, as in
`path@*@README.md`. It is accepted when the path is a required document name
— a compiled document, or a file register's README or generated index — and otherwise when at
least one component carries the path with the claimed kind. The file-register set is derived from
the component-scoped registers rather than written out, so a project that declares one gets its
names in the set. A register's home is no `path` target, per
`design@core@path-to-a-home-is-reported`: `register@*@<register>` names every component's own home
of a component-scoped register, in whichever shape each holds, per
`design@core@register-home-reference`. A generic reference nothing resolves rots exactly like a
dangling one. The anchors the tool constructs hold no copy the generic form names: what they hold
is cited by its kind.

**Under any other kind the escape names nothing, and `*` names nothing but under `register`.** A
`design@*@<id>` is reported as an unknown anchor, and the finding says which kinds each word
serves. The `planned` kind refuses them
with a finding of its own, per `design@core@planned-path-form`. The manifest's resolution
refuses a declared anchor wearing either word, and it is no anchor: every pointer at it
would read as the reserved meaning.

The designs this decision did not keep: `rejected@core@at-prefix-escape` and
`rejected@core@component-placeholder-anchor`.

### A register's home is cited by its own kind, `register@<anchor>@<register>`, which names the home in whichever shape it has `##register-home-reference`

`register@<anchor>@<register>` names the home that the anchor keeps the register in. The check
resolves it against the anchors, which the table's module holds, and asks nothing of the tree,
since a missing home is the tree check's finding; `show` reads the tree's listing to print the
home it holds. No register's name becomes an entity id, so a register's name in backticks with a
hash sign before it stays silent under `design@core@retired-slug-reference-reported`.

- **The anchor** is a component, a declared location or the anchor `plans`, where it carries the
  register; `*` names every component's own home of a component-scoped register, per
  `design@core@reserved-anchors`. An entry or a spec anchor is no anchor of the form: an entry's
  homes are its own directory, which the entry's reference already names, as
  `milestone@plans@<id>` names a milestone's, and one document has one name, per
  `design@core@plan-document-kinds` and `design@core@declarable-directory-register`.
- **The id** is a register's name, built in or declared. An item register is no id of the form:
  its home is a plan's own documents, cited by the plan document's kind.
- **Each refusal names the form that resolves**, per `design@core@finding-names-the-repair`.
- **`show`** prints the home the tree holds, the file or the directory, with the register's shape,
  then every site that cites it, a citation of a component's home through `*` included.

**The kind exists because a home has two shapes**, per `design@core@heading-register-two-shapes`.
A `path` names one of them, so a home that outgrows one file dangles every `path` to it, against
`goal@core@relocation-is-one-manifest-edit`, and a text that must hold in every project names both
shapes, per `goal@agent-skills@installed-text-works-anywhere`. This reference names the register and lets the manifest and the tree say which shape it is.
The nearest rival is a `path` that resolves to either shape; it loses on
`design@core@trailing-slash-claims-directory`, since a path claims its target's kind, and a path
that matches either shape asserts no claim and puts a special case into path resolution.

**The kind's word is the owner's choice**: "I'd rather use register. home just reads weirdly." The
rival word, `home`, matched the documents' phrase "design home" better and cost the same refusal of
an anchor name. A kind's name is refused for every anchor, per
`design@core@kind-names-refused-as-anchors`, and for a declared register, per
`design@core@registers-are-declared`. The cost the form keeps: a reader outside the tool does not
see the file a reference names, which `issue@core@the-documents-do-not-render-as-a-site` would turn
into a link.

The designs this decision did not keep: `rejected@core@home-kind-word` and
`rejected@core@path-to-either-home-shape`.

### A `path` reference whose target is a register's home is a finding, whose repair names the `register` form `##path-to-a-home-is-reported`

A home target is a heading register's file, its directory, or that directory's `README.md`, which
is the home's head per `design@core@heading-register-two-shapes`; a file register's directory; a
Directory register's directory. A file register's `README.md` and generated `index.md`, a
subdocument and an entry are files inside a home, and stay `path` targets. **An entry's directory
and its `README.md` are no home target**: they are the entry, and a `path` to either is a finding
whose repair names the entry's reference, `<register>@<parent>@<id>`, per
`design@core@declarable-directory-register`; a milestone's, by `design@core@plan-document-kinds`.

- The finding is raised for a named anchor after the refusals of a refused shape and of a plan
  document cited by its path, on the target resolved to the anchor that keeps the home, and in
  place of the deeper-anchor finding: its repair names `register@<that anchor>@<register>`. Under
  `*`, it is raised before the required set and any kind claim are judged, and names
  `register@*@<register>`.
- The kind claim and the existence of the target are not judged: the manifest decides whether a
  path is a home, and either shape names the same register.
- A line number or a fragment is stripped before the test, and the repair says to drop it.
- A `planned` reference is not judged by it.

**Both forms legal** loses to this finding: a writer keeps writing the `path`, and a home that
changes shape then dangles every one of them, which `design@core@register-home-reference` exists to
prevent, against `goal@core@relocation-is-one-manifest-edit`. The finding repairs nothing by itself:
no fix of `check --fix` respells it.

The design this decision did not keep: `rejected@core@path-to-home-beside-register-form`.

### The word `plans` names the anchor the tool constructs at the plans directory, and no project may declare it `##plans-is-a-reserved-anchor`

The anchor is constructed per `design@core@plan-register`. The word is reserved like the two words
of `path`, so a citation of a plan reads `spec@plans@<id>` in every project, with no project name
in it: there is one plans directory, so the root anchor in every such citation would carry no
information. The manifest's resolution refuses a declared anchor named `plans`, and it is no
anchor: every pointer at it would read as the reserved meaning.

### A path a plan's work will create is cited `planned@<anchor>@<path>`, from the plans directory only, and reported once it exists `##planned-path-form`

A plan document names files and directories its work will create. The kind `planned` names one:
its anchor and its path follow every rule of a `path` reference, the named anchor, the deepest
anchor, the refused shapes and the trailing-slash claim, and its one assertion is the reverse of
`path`'s: the target does not exist. Once it exists, the finding names the `path` form and
nothing else, so the commit that creates the file converts the reference by its kind word, and
whether the plan still holds is that session's report to make. A target the ignore rules cover is
not asserted, per `design@core@ignored-targets-are-not-asserted`. `*` and the escape anchor are
refused: a planned path is in this tree, at one anchor.

**It is legal in the plans directory alone**, since a plan document is the one place that
describes unbuilt work, per `design@core@plan-register`. Elsewhere it is a finding that asks for
the plan that creates the file. An issue entry states what is missing rather than the shape of its
fix, and a hypothetical file there is `path@elsewhere@<path>`.

It is the checked form for a pointer that `design@agent-skills@plain-text-is-no-repair`
would otherwise send to plain text: the escape anchor is refused once the work creates the file.
The escape anchor as the planned form lost on the anchor: it names none, so the deepest-anchor rule goes unchecked until the file
exists and the conversion cannot be done by the kind word alone. `planned` is a reserved kind, and
the manifest refuses a register of that name, as it refuses `path`.

The design this decision did not keep: `rejected@core@planned-path-in-every-document`.

### A trailing slash claims a directory, and its absence claims a file `##trailing-slash-claims-directory`

The kind claim sits in the span itself — greppable, visible to the reader, checkable — rather
than inferred from what happens to exist. The claim is asserted as a fact in both directions, every
required document is asserted to be a file rather than a directory wearing its name. A register's
home is no `path` target, so its two shapes take no claim: the `register` kind names either, per
`design@core@path-to-a-home-is-reported`.

### A target the ignore rules cover is not asserted `##ignored-targets-are-not-asserted`

Resolution asks `git check-ignore` whether the ignore rules cover the target before asking
whether it is present. Covered means accepted with no existence or kind assertion: a generated
path cannot be asserted to exist, per `design@core@git-supplies-the-walk`, and deciding by the
RULES rather than by presence makes the verdict identical on a fresh clone and a built tree. A
verdict that depends on the checking machine's build state is a check nobody can trust twice.
The exemption is exactly as wide as git's own answer, deliberately, nested ignore files included.

**The question is asked with the reference's own kind claim.** A directory claim is spelled with
its trailing slash and a file claim without, because a `dir/` pattern matches a path git can tell
is a directory, and a target that does not exist yet is a directory only if the spelling says so.

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

The design this decision did not keep: `rejected@core@single-link-shape`.

## 4. What is prose

### The grammar for a file's kind decides which of its bytes are prose `##grammars-not-prefixes`

A markdown document is prose entire. A Rust file is prose only where its grammar says so — its
comments, and the string literals that are not bound to a name. Nothing is decided by a line's
prefix. The one file set whose literals are never prose is the checker's own source, per
`design@core@checker-source-literals-are-data`.

**A doc comment's content is markdown**, so the markdown analysis runs over a document and over
every Rust comment alike, and a heading, a fence or a blockquote means the same in both.

**Prefixes were the single cause of four defects in thaum's tree**, each found by a checker or a
review rather than by reasoning: a formatted return type opening a line with an angle bracket read as a
blockquote; a float literal read as a rule number; a test fixture's string literal read as live
content; and a rule quoted inside a block comment lost because the `*` continuation was left in
front of it.

**Three readings of "data" were too broad, and the measurement over thaum's tree, in the message
of the commit "rework how the knowledge tool parses documents and enforces rule citations", caught
each.** A fenced block is not data — a sketch in a design document comments
its rules on purpose, and reading fences as data lost 34 citations. A string literal is data only when BOUND to a name — an assertion's
message cites rules for a human to read on failure, and reading every literal as data lost 32
of them. A macro body is not an argument list, so a binding still applies inside one. **Reading
every literal as prose is a recorded rejected design**,
`rejected@core@every-literal-read-as-prose`, with the bound literals outside the checker that carry
a rule-shaped number by accident. The binding rule does not serve the checker's own
fixtures, which `design@core@checker-source-literals-are-data` answers.

The design this decision did not keep: `rejected@core@hand-rolled-indentation-scanner`.

### Rust source is parsed by one parser that keeps every comment and is released on its own cadence, `tree-sitter-rust` `##rust-parser-keeps-every-comment`

The core reads Rust source with `tree-sitter-rust`, through the `tree-sitter` crate, both declared in
`path@core@Cargo.toml`. Its syntax tree keeps every comment, doc comments and ordinary `//` and
`/* */` comments alike, with their byte ranges, so one pass tells which bytes of a Rust file are
prose, as `design@core@grammars-not-prefixes` requires, and `pulldown-cmark` reads those ranges as
it reads a markdown file.

The choice turns on two external tools' behaviour, entry test 3 of `primer@design-heads`. The
nearest rival, `syn`, drops ordinary comments while lexing, as the prototype that
`rejected@core@syn-rust-parser` records showed, so a reference in a `//` comment inside a function
body would need a second pass over the raw text: two readers that each answer half of the question.
Rust-analyzer's syntax crate keeps comments, but it is a crate of compiler internals released with
the rust-analyzer releases, whose interface changes from one release to the next; this cadence is an
assumption, which no measurement or documentation in this tree checks.
An upgrade of `tree-sitter-rust` that renames a node kind or a field the extractor asks for fails
the test `every_rust_extractor_node_kind_is_one_the_grammar_defines`, and a Rust file of the tree
that yields no prose region while it holds a comment, or no item scope while it holds a top-level
function, fails `every_tracked_rust_file_yields_prose`. An upgrade of `pulldown-cmark` that renames
an event fails to compile, since the extractor matches its events as Rust enum variants. The designs this decision did not keep: `rejected@core@syn-rust-parser` and
`rejected@core@rust-analyzer-syntax-crate`.

### A Rust name is collected where it is declared, never where it is used `##names-collected-where-declared`

**A name is the one thing prose cannot reach**, and the identifier form of a rule marker exists
for names. Names are collected where they are DECLARED. Collecting every occurrence would make a call
site and a `use` import citations owing the rule's whole body in the caller's scope, where there
is nowhere to put it.

### The checker's own source reads its string literals as data `##checker-source-literals-are-data`

Under the tool's own directories a string literal is never prose. Its comments are prose like
every other file's, and everywhere else in a tree a string literal is prose unless it is bound
to a name, per `design@core@grammars-not-prefixes`.

**The tool's unit tests are the one place in a tree whose fixtures are rule-shaped on purpose.**
A number, a marker, a slug, a path or a version pin is written so that a test can watch a check
react to it. Read as prose each one is a live claim, and hiding each behind a named constant to
be interpolated makes the tests harder to read than the code they test. Comments stay prose
because the test modules point at decisions and paths for real, and those pointers stay checked.

**The directories are compiled in, never declared.** A binary's source spans several
Components: the core's, and the Component of each extension the binary registers. Each library exports
the directory of the Component it belongs to, evaluated at build time, and the binary hands the
list of its libraries' directories to the walk. The core is one crate whose directory is its
Component, so the core library exports its own `CARGO_MANIFEST_DIR`, and the core's binary and
tests sit inside it. An extension whose library crate sits below its Component's directory exports
that Component's directory instead, or the binary and tests beside the library would sit outside
it: a binary sees only the libraries it depends on. A binary run
through a cargo alias over `cargo run` is built from the checkout on every invocation, so each
compiled path is the tree being checked, unless another checkout shares the target directory and
the build is not tied to its checkout, which `design@core@a-foreign-build-is-refused` refuses. Once the core is consumed as a published crate, its
directory is outside the tree and exempts nothing, which is correct: its source is then not part
of the tree. The root and the compiled path are canonicalised before
the prefix test, so a symlinked checkout does not defeat it; a symlink inside the tree is not
followed. A manifest row is not an option, because a row can be pointed at any directory, which
is the shape `design@core@the-regime-has-no-opt-out` exists to refuse: only the checker's own source
can ever be exempt, and it is exempt by construction. The row's entry is
`rejected@core@exempt-directory-manifest-row`.

**The failure that remains is loud.** The summary block names each of the checker's directories, relative to the root when it sits under it and
absolute otherwise, even when the compiled directory no longer exists, and prints the count of
Rust files it covered, so the state is visible in every run. A directory exempts files only when
it sits inside the tree being checked: a tree that sits inside it instead, such as a mock project under
`path@core@tests/projects/`, is a foreign project and every literal in it is prose.

**Whole source rather than test modules only.** The non-test source holds no literal that cites
a rule, a finding message interpolates the number it names, and a narrower rule would cost an
attribute lookup to protect a class with zero members.

The designs this decision did not keep: `rejected@core@fixture-marker-macro`,
`rejected@core@unit-tests-moved-under-tests`, `rejected@core@literals-as-data-in-every-crate` and
`rejected@core@no-string-literal-read`.

### A binary built from another checkout of the tool is refused before any command `##a-foreign-build-is-refused`

Two checkouts sharing one
target directory leave the last build's binary for both, and cargo does not rebuild it for the
other checkout, whose own package is still fresh; observed on thaum's repository, the other
checkout then runs that binary, its code and not only its compiled paths; the reproduction is in
the message of 4ebd9a2. Before any command,
the binary compares, over the project it locates or, for `init`, over PATH where PATH exists, where its own crate and each library it links were compiled with the tree it
is run over: a tree that holds the same package, by the name its `Cargo.toml` declares, at a
trailing run of the compiled directory but not at the compiled directory itself is a second
checkout, and the run exits 2 naming both directories and the `cargo clean` of every workspace
package the binary links, which rebuilds them from this one. The binary's own crate is listed
because it is rebuilt whenever anything it links is, so its compiled directory is always the last
build's; a worktree nested in the tree is a second checkout like any other. A tree holding no
copy is left alone, which covers a mock project under the tool's directory and a project
consuming a library as a published crate.

**The refusal detects, and a build tied to its checkout prevents.** A project whose every target
is tied to a variable valued at its checkout's root never runs another checkout's build, as
`design@knowledge-architect@a-build-is-tied-to-its-checkout` argues for this repository. The
refusal stays, for a binary built before the tie or outside the configuration that sets it, and
for a project that does not tie its builds. Its message names the tie beside the `cargo clean` that
recovers.

**Three limits.** Two come from reading only names and places. A second checkout that moved the
crate to another relative path is not seen, and runs, unless the build is tied to its checkout; the loud failure of
`design@core@checker-source-literals-are-data`, printed by `check`, still names it. A tree
holding an unrelated crate of the same name at the same relative path, one component long or
more, is taken for a copy, and refused; a binary run through an alias that builds it from the
checkout never meets one. The third comes from where the check lives: it is part of the binary it judges, so a binary
built from a checkout older than the refusal, or from one that changed it, runs unrefused.

**The refusal, not a correction of the paths.** Finding the directories at run time would exempt
the right files and leave the other checkout's code judging this tree. Every command is refused,
not only those that read literals, because every command runs that code. Each library a binary
links exports its crate directory and its package name, and the binary hands the list over, so an
extension's library is covered by listing it; this constrains every extension binary. The values
it reads are each crate's own directory and package name, beside the Component directories that
`design@core@checker-source-literals-are-data` compiles in.

**Every binary tests both halves over its own Components.** Over a checkout holding the tool,
the summary's checker-source line names each Component whose library the binary links and
counts the walked Rust files under them. And a tree holding one of the binary's crates at
another relative path is refused. The core's binary does this in
`path@core@tests/binary.rs`; an extension binary does it in its own suite, because only it
links its extension's library.

The design this decision did not keep: `rejected@core@run-time-checker-directory-discovery`.

### The manifest pins the checker's version, every binary that dispatches a command of the checker refuses to run over a project it does not satisfy, and a binary that dispatches none makes no call `##installed-binary-version-check`

Every manifest declares `[project] checker-version`: the exact version of the checker the project
runs, which is the version of the core library whatever binary links it, or one of the two values
of `design@core@checked-sentinel-values`. `Manifest::parse` reads it; an absent key, or a string
of none of the three forms, is a complaint opening with the key's name, and a value that is not a
string fails the parse. Each binary that dispatches a command of the checker calls
`cli::refuse_another_version` in its `main`, right after the build-origin refusal of
`design@core@a-foreign-build-is-refused`, before it dispatches any command but `init`, an
extension's own included: a version is accepted when it equals the core library's, and any refusal, the key's own
complaint included, is exit 2, per `design@core@exit-code-ladder`, naming the two versions and
which side is older. `init`, which every binary hands to `cli::init` before it locates a project,
calls no version refusal: no manifest exists to read a pin from, and the one it writes pins the
core library's version, per `design@core@init-command`. `cli::run` does not refuse, so a command
driven in-process needs a key but no confirmed one. A binary that links the library and dispatches no command of the checker, such as a
maintenance crate that uses it only to find the project's root through `MANIFEST_NAME`, does not
call it: nothing it runs reads the documents by the checker's rules, and where it runs the checker,
as a gate does, it starts a binary that makes the call, since the gates library depends on nothing
of the checker, per `design@gates@a-project-holds-its-gate-list`.

**Why the manifest, and why required.** Without a pin the binary reads, a binary older than the
skills a project installed reports each installed file as differing, with the repair "run
install-agent-skills", which writes the older text over the newer; under `harness = []` nothing is
compared at all; and two versions whose skill text is identical are not told apart. A pin in the
manifest covers all three; one more place to
edit when the pin moves cannot drift silently, since a binary differing from it refuses at its
first command. Optional, the key would leave unprotected the project that never wrote it, which is
the forgotten pin it exists for. This serves `goal@knowledge-architect@any-project-can-adopt-it`:
a project pins the version it uses, and moves when it chooses.

**The working tree only.** `commits` reads a historical tree's key, so a tree without it stops in
phase 1, and compares no historical value, so moving the pin fails no earlier commit. A branch that
introduces the key therefore opens with the commit that gives every manifest the key, per
`design@core@a-commit-message-is-a-document`.

**A binary released before the key** refuses a manifest carrying it at parse, as an unknown field,
since `[project]` denies unknown keys: it runs no command, so it cannot suggest a downgrade, but
its message does not name the pin.

### A checker version that is no version is accepted only where the running binary's build confirms it `##checked-sentinel-values`

Two values of `[project] checker-version` are not versions, for a project's own tests and for a
tree the checker is built from:

- `"fixture"` is accepted when the project lies strictly inside the core's compiled directory or
  one of the directories the binary passes as its own source: a mock project of a library the
  binary links. Strictly, because an extension whose crate is the project's root passes the root
  among its directories, and a library is no mock of itself.
- `"self"` is accepted when the core's compiled directory lies inside, or is, the project, and the
  project's git tracks that directory's `Cargo.toml`: a tree the checker is built from. The
  tracking tells it from a project whose `CARGO_HOME` is one of its own directories, where a
  registry build's directory also lies inside the root.

Both read the paths canonicalised, as `design@core@a-foreign-build-is-refused` does, and decide
from where the running binary was built, which names no project, per
`design@core@nothing-of-a-project-is-compiled-in`. Anywhere else each is refused, so no value of
the key switches a real project's pin off, the stance `design@core@the-regime-has-no-opt-out`
takes for rules over documents. A copy of a mock project outside its library carries the version.
The same values accepted wherever they are written are
`rejected@core@sentinels-accepted-everywhere`, which this decision did not keep.

## 5. Commit messages

### A commit message is a document under the regime, judged against the tree its commit carries `##a-commit-message-is-a-document`

A commit message is parsed as one markdown document — its subject line, its blank line and its
body — and every rule of the regime runs over it: every reference is judged against the entity
table of `design@core@one-entity-table`, and each registered extension judges the message
against the tree it prepared for that commit.

**A message is history, and it cannot be edited.** `main`'s history is never rewritten, so a
message's claims are fixed the moment it lands: a pointer that stops resolving in a document is
repaired where it stands, and the same pointer in a message can only be read wrong. That is why
the regime matters most on messages.

**A message is judged byte for byte, as the commit holds it.** `commits` reads the message
with `%B` and cleans nothing: git applied its own cleanup before the commit existed, and a
second pass would take bytes of a commit out of the regime — under `-m` a `#` line is committed
verbatim.

**One command judges messages, after the commit exists.** `commits <range>` judges every commit of
the range against its own tree, with its first parent. It is run over `HEAD~1..HEAD` after each
commit, or over the branch where `design@core@branch-shas-are-refused` is turned on, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`, so a finding in the newest
commit's message is repaired by `git commit --amend`, and over the branch by the gates and by CI
before any merge, which is where the guarantee lies: `main` is never rewritten, so what must not
happen is a message with a finding reaching it, and the range check before the merge excludes that.
No hook judges a draft before the commit exists. A hook repeats the range check's message half, not
its tree half, at every commit; it is per-clone state a session has to install and check; and it
resolves a message against HEAD, which under an amend, a reword or a squash is the commit being
replaced, so it refuses history edits the range check accepts.

**Everything a commit is judged against is read from that commit's tree.** The manifest, the
`[walk]` rules, the documents, the generated indexes, the per-instance options and every file an
extension reads all come through git objects, and the model is assembled in memory. Reading them from the
working tree instead would judge a message written a hundred commits ago against decisions that
did not exist then, and the finding list would be a list of things nobody could have known.

**Every commit of the range is judged by the tip checker, and a tree that fails is a finding.**
The tip checker is the binary built from the working tree, and it judges every commit's tree and
every commit's message. **Every commit of the range is judged alike, the last one included, and
wherever HEAD is.** A commit whose tree does not load, or carries findings, fails the run with
exit 1: its tree's findings are printed, each named by the commit and by the file inside it, and
its message is still judged wherever the tree reached the last phase, since its entity table is
then complete. Where the tree stopped earlier, the message is judged against nothing and the run
says so. No commit is set apart as the range's tip: a range that does not end at the checkout,
such as a pre-push hook's or an audit of old history, holds no commit that `check` has judged,
and treating its last commit specially would leave that commit's failure unreported as a finding.
The summary counts the range's commits, how many passed and how many failed. A commit fails when
any finding of the run belongs to it: its tree's, its message's, or a citation of the range by
SHA. Its line names each of those sources, with its count of findings, except a tree that does not
load, which is one finding. So the header agrees with the verdict line, and passed plus failed is
the number of commits. Separate counts of failed trees and
failed messages in the header lose to this: one commit can fail on both, so either such a count
overlaps the other, or one source is left out.

**A branch that makes the checker stricter orders its commits for it.** A new check, or a change
to the manifest format, makes every earlier commit of the same branch fail under the tip checker.
Such a branch puts that change in its first commit, with every fix the tree needs to pass it, or
is squashed to one commit before its review. Checker changes are rare and land on branches of
their own, so this costs a history edit on those branches alone; the alternative that avoids it,
building each commit's own checker, costs one release build per commit on every branch.

**A message's references resolve against its commit's tree or its first parent's, and its lints
stand where either tree holds what they name.** A commit that closes an issue deletes the entry
and names it in the message, and against its own tree alone every such message would dangle. A
reference finding therefore survives where both trees refuse the reference. The unanchored path
and the retired slug reference, per `design@core@every-path-names-its-anchor` and
`design@core@retired-slug-reference-reported`, fire where their span names something of the
project, so a finding of either stands where either tree holds what it names: a message naming,
by a bare path, the file its commit deletes still points at it. The parent's model is the previous commit's wherever
the walk followed the parent chain — a failed commit's tree still serves as the next commit's
parent, since its entities are read and not its verdict — so an extra model is built only at
the range's start and where the chain was not followed. **A tree whose manifest fails phase 1 is
the exception**: nothing of it is read past its manifest, it is judged no further, and it serves
as no parent, so the next commit's message resolves against its own tree alone. Such a commit
has already failed the range, and reading its whole tree for its successor's sake dominates the
time `commits` takes over a range of them. A parent tree is assembled for its
entity table and the facts a path reference and the two lints ask about, and nothing else: no extension is
prepared for it and no check runs over it, because nobody reads its verdict. The two arms are compared by the site each names — the line and the span — rather than by the words each writes,
because two trees can refuse one reference for different reasons and comparing the words whole
would let a reference that resolves in neither pass.

**A historical tree's `[project] checker-version` is read, not compared.** Its absence stops the
tree in phase 1; its value is never compared with the tip checker's version, so moving the pin
fails no earlier commit, per `design@core@installed-binary-version-check`.

**`check` reads no range of history.** A message is not a file of the tree, and a check whose
verdict moved with the branch's history would be a check nobody could reproduce from a checkout:
`git stash` alone would move it. `check --staged` reads HEAD's tree and the index, one tree and no
range, per `design@core@staged-tree-source`. The range is always explicit, and the gates pass the range from the
remote `main` to `HEAD`, so what is judged is the branch's own commits and never `main`'s.

**An extension leaves out, over a commit's tree, a check whose subject is not the tree.** A
check of filesystem state has no subject there, and one that would read every archived input at
every step of the range decides nothing about whether a message's references resolve, which is
the question the per-commit model exists to answer; `check` is what judges a checkout against
it.

**The ignore rules a per-commit run asks are the working tree's.** `git check-ignore` reads the
`.gitignore` files on disk and has no form that asks a historical tree. A commit whose ignore
rules differed from today's is therefore judged against today's, which can cost a path reference
asserted where that commit's own rules exempted it — a finding rather than a silence. Since a
failing tree fails the range, a branch that changes its ignore rules orders its commits for it
as a branch that makes the checker stricter does: the change first, or a squash.

The designs this decision did not keep: `rejected@core@check-refusing-a-pipe`,
`rejected@core@commit-subcommand` and `rejected@core@commit-msg-hook`.

### `commits` refuses a citation of a commit of its range by SHA, where the manifest turns it on `##branch-shas-are-refused`

Under `[commits] refuse-branch-shas = true`, `commits` reports every run of 7 to 64 lowercase hex
digits, bounded by bytes that are not ASCII letters, digits or `_`, that prefixes the SHA of a
commit of the range it judges: in each commit's message, and in the whole text of each document of
that commit's tree, a Rust file's code and literals included. A commit whose tree stops at phase 2
or 3 is scanned too, since the scan needs no entity table and no verdict. One whose tree stops at
phase 1 is not: its manifest did not resolve, so the option is not read from it, and no document of
it was read. The range a branch is judged over is
its own commits, so these are exactly the commits a merge that rebases gives new SHAs; a citation of
one would name nothing once the branch merges. A SHA on the main branch, or of another project, is
not in the range and passes, unless its first 7 digits collide with a SHA of the range. **A citation
is seen only when the cited commit is in the range judged**, so where the option is on, the check
after a commit is run over the branch, `<main>..HEAD`: over `HEAD~1..HEAD`, a citation of an earlier
commit of the branch passes, and fails first at the merge gate, after the push, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`. Only lowercase counts, since git
prints a SHA in lowercase, and `_` bounds a word as a letter or a digit does, since it joins the
parts of an identifier. A project whose branch commits keep their SHAs until they reach the main
branch has nothing to refuse; a rebase onto main before a fast-forward rewrites them as surely as a
rebase merge does, so the check is off unless the manifest turns it on. It serves
`goal@core@declared-instructions-are-checked`: a rule against such citations is kept by running
`commits`, not by a reviewer remembering it.

The check lives in `commits` because it already holds every input: the range's commits, each
message byte for byte, and each commit's tree assembled into a model. A gate of its own would
list the range and read the messages again. A SHA in a file the walk does not read is not seen,
which is the checker's scope everywhere else. Sixty-four digits is the length of a SHA in a
repository under SHA-256. A citation of a commit the branch rewrote before `commits` ran is not in
the range, and is not seen: no clone can tell reliably which commits a branch once held, since a
fresh clone, such as CI's, never fetched them.

## 6. The agent configuration

### The `[agents]` table declares which agent harnesses a project serves `##agents-table`

`[agents] harness` lists the agent harnesses the project serves. The tool knows one, `claude`, and
it is the default when the table is absent, because the tool is built for projects developed
mostly by agents. Under `claude`, every component carries a CLAUDE.md, and the agent files this
version ships must be installed, per `design@core@owned-namespace-check`. An empty list declares
no agent configuration: no CLAUDE.md is required, and nothing is installed or checked. A value the
tool does not know is refused in phase 1, and leaves the list.

Under `claude`, the four harness kinds exist, per `design@core@harness-kinds`; under an empty
list, they do not.

Under an empty list, the files of the installer's namespace are walked as the project's own
documents, like any others. A harness for another provider is a value added to the list, requiring
its own file and its own layout, which `issue@core@configuration-for-several-agent-providers`
records. This is what serves `goal@knowledge-architect@any-project-can-adopt-it` for a project
without agents.

The design this decision did not keep: `rejected@core@claude-md-in-every-component`.

### A project declares the command it runs the checker by `##declared-command`

`[project] command` names the command a project runs the checker by; when it is absent the command
is `klarch`, the binary's name. The checker prints it in its messages, in the header of every
generated index, and in the agent files it installs, which are templates holding the placeholder
`{{command}}`. No fixed name serves every project: a project with an extension runs its own
binary, as thaum runs its rules extension through a cargo alias, and the core's binary refuses a
manifest holding a table only an extension claims. An extension project therefore declares a
command that runs its own binary, preferably a cargo alias such as `cargo klarch`, and does not
install its binary under the plain name. A command that cannot be printed on one line in a code
span, empty or holding a line break or a backtick, is refused in phase 1, and the default stands.
This serves `goal@knowledge-architect@any-project-can-adopt-it`: every project, extended or not,
reads instructions that name the command it actually runs.

### The installed agent files are committed, and checked byte for byte in a namespace the installer owns `##owned-namespace-check`

`install-agent-skills` writes each file this version ships at its install path, rendered with the
project's command, and deletes every file of the installer's namespace that the version does not
ship. The namespace is the prefix `knowledge-architect-` among the skill directories, the agent
files and the saved workflow files of `.claude/`, and the directory .claude/knowledge-architect/. A project's own skill takes
a name of its own, so ownership is decided by a name, with no record and no history: a renamed
skill shows as the old file to remove and the new one missing; one install repairs both, and the
removal awaits staging like any deletion, which the check names until it is staged.

The installed files are committed, so a session started from a fresh clone, a web session or a
review in a new worktree has them without building anything, which serves
`goal@knowledge-architect@agents-get-a-complete-workflow`. A symbolic link on an owned path is
refused by the install before it touches anything, because following one would write or delete in
another directory. Under `claude`, phase 2 reports each
shipped file missing or whose bytes differ from its rendered template, each file of the namespace
the version does not ship, and a root CLAUDE.md that does not import the shipped primer with a line
holding exactly an at sign followed by .claude/knowledge-architect/PRIMER.md. These are facts about what the tree
holds, like a missing required document, so a run stops before references while one stands, per
`design@core@phases-gate-the-report`. A command that writes a generated file refuses while one
stands, as over any phase-2 finding;
`issue@core@installed-file-findings-belong-in-phase-four` records why the last phase would fit
them better. Only `check` compares: `commits` does not, because the running binary ships its own
version's text, and an older commit's installed set would fail against it with no repair a commit
already in history can take. The comparison is of bytes: the binary holds the text, so a
digest would add a dependency and nothing else. The installed files are outside the walk by
construction, as the generated indexes are: their prose is the shipped text's, judged where that
text is written, and its illustration paths would otherwise be reported in every installing
project.

**The installed copies are also read, for definitions only**, per
`design@core@installed-entities-from-the-tree`: the model holds them parsed, no check walks them
for references, and they raise no finding of the section rule.

The install edits nothing outside the namespace. In particular it does not edit the root
CLAUDE.md, which belongs to the project: it says the import line is missing, and the check reports
it with the line as its repair. A change a project needs to an installed skill belongs in a skill
of its own, routed from the project's CLAUDE.md.

The designs this decision did not keep: `rejected@core@session-start-hook-primer` and
`rejected@core@install-record-file`.

### Under the `claude` harness, skills, agents, the primer and the root CLAUDE.md are entities of kinds that are no register `##harness-kinds`

The four harness kinds, `skill`, `agent`, `primer` and `instructions`, exist under the `claude`
harness alone, per `design@core@agents-table`. They are kinds, not registers: a register is what
an anchor carries, and no anchor carries them and no project declares them, since the harness fixes
where each lives. So they sit beside `path`, `planned` and `register`, the kinds already no
register. The
rival, built-in registers of a new shape carried by no anchor, would make every reader of a
register handle a register with no carrier: the manifest's refusals, the index, the extension view.

| kind | an entity | named by |
| --- | --- | --- |
| `skill` | a directory directly under `.claude/skills/<name>/` holding `SKILL.md` | the directory, per `design@core@harness-entity-names` |
| `agent` | a Markdown file at any depth under the agents directory whose frontmatter sets `name` | the file, its `.md` removed once, per `design@core@harness-entity-names` |
| `primer` | the installed primer; its sections alone are entities | none |
| `instructions` | the root CLAUDE.md; its sections alone are entities | none |

The layout is the harness's, read in its documentation of skills,
https://code.claude.com/docs/en/skills.md, and of subagents,
https://code.claude.com/docs/en/sub-agents.md: a skill in a `<subdirectory>/.claude/skills/`
directory is loaded only when a session reads a file there, and is no entity here; the agents
directory is scanned "recursively"; a file there with no `name` is treated "as documentation kept
beside your agents", so it is no agent, and no README or index is owed there.

Under `harness = []` none of the four kinds exists, a span headed by one is silent, and installed
copies a project kept are its own documents, whose slugs are misplaced definitions. Serves
`goal@core@records-reach-their-reader`: a citation of a skill, an agent or a section is checked,
and `show` lists every one.

### Under the `claude` harness, the root CLAUDE.md is modelled and no scoped CLAUDE.md is `##root-instructions-alone-modelled`

The root CLAUDE.md alone is modelled, as the owner chose: "I'd model only root CLAUDE.md
sections." A scoped CLAUDE.md is not modelled: what such a file holds is the subject of
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`.

### Under the `claude` harness, a skill is named by its directory and an agent by its file, and a frontmatter `name`, where one is set, equals that name `##harness-entity-names`

"The directory name also invokes the skill", in the harness's documentation of skills, even where a
frontmatter `name` sets a second command. An agent's "identity comes only from the `name`
frontmatter field", in its documentation of subagents. A frontmatter `name` that differs from the
directory's or the file's name is a finding, so the name a reference cites and the name the harness
uses are one. The match is the owner's ruling, against the frontmatter `name` alone as an agent's
id: "I'm not sure our installed workflow would be able to work properly if not, and I'd rather make
the checks more strict to keep the workflow simpler than the other way around." Its cost: the
harness documents a skill whose `name` differs from its directory as a second command for it,
which is a finding here, so a project that wants a short alias renames the directory. The `name:`
line is read alone, as YAML reads it, so frontmatter the harness reads and the checker's own
frontmatter subset refuses, a list or a multi-line description, causes no finding.

### Under the `claude` harness, a skill or an agent whose name a reference cannot spell, or that the harness does not load, is a finding `##unusable-harness-name-is-a-finding`

A skill or an agent named outside the id grammar of `design@core@two-segment-ids` could not be
cited. The harness's documentation of skills says a skill folder is not named "`synced`, in any
capitalization", and that one named `anthropic-skills` or beginning `anthropic-skills:` does not
load. So a skill whose directory begins `anthropic-skills` is a finding, and one named `synced` is
one under the grammar, which refuses a name of one segment. The rival, recording the
documentation with no check, would pass a skill no session can use. An agent's `name` is, in the
documentation of subagents, "at most 256 characters", holds no `:` and does not start with `-`; the
id grammar is stricter, and stands as its own requirement.

### The installed skills, agents and primer are defined from the installed copies of the tree being judged `##installed-entities-from-the-tree`

The model holds the installed copies, parsed like a walked document and walked by nothing:
`Model::build` reads them off the disk, and `commits` hands in each commit's own blobs. Both read
them whatever the walk rows say, so a skip over the installer's namespace leaves the two verdicts
alike. A commit is therefore judged against the installed set it holds, so a branch that renames a
section in an upgrade and repairs its citations passes `commits` on every commit.

- **Not the binary's shipped set.** `commits` judges every commit of a range with the tip's binary,
  per `design@core@installed-binary-version-check`, so definitions from the shipped set would fail
  every earlier commit citing a section the tip renamed. That is why `design@core@owned-namespace-check`
  already compares no installed copy under `commits`.
- **An installed copy raises no finding**, per `design@core@section-homes-carry-slugs`.
- **This repository's shipped text defines nothing in its walk**: content/ writes each section slug
  as a build placeholder, per `design@agent-skills@content-mirrors-the-install-layout`, so the
  definitions are the installed copies', which this repository installs with each change to
  content/.

## 7. Creating a project and a Component

### `init` creates a project: its manifest, the installed set, and every document and home the check requires, ending as `check --fix` ends `##init-command`

`klarch init` writes the manifest, pinned to the version of the binary that runs it, listing each
`--component <DIR>`, DIR relative to PATH, as a Component, then every document the root Component,
each Component it lists and the plans directory owe, then ends as
`check --fix` does, through its code: it installs the shipped set, writes the indexes once phases 1
to 3 find nothing, and runs the check, whose verdict is the run's. Outside every git worktree, under
`--vcs none`, it installs the shipped set and stops there, saying what comes next: the indexes and
the check read git's listing, per `design@core@failed-git-is-exit-two`, and the install reads none.
No directory of audit reports or of retrospective analyses, and no roadmap, is written: each is
created when first needed. A project created from nothing is the target
`goal@knowledge-architect@any-project-can-adopt-it` puts first, and its creation is where a person
or an agent starts, per `goal@knowledge-architect@adoption-is-easy`: without the command, every
file `klarch init` lists as `wrote` but the installed ones and the manifest is written by hand
before the first check passes.

The rival, `check --fix` creating the missing documents, is out on
`design@core@safe-fix-definition`: a README is neither an installed file nor a generated one. The
list of what is written is not a second list: it is `Manifest::required_documents`, the register
homes `Anchors::of` gives, and the plans directory's README by the rule the check uses for it, so a
change of what a Component carries reaches `init` with no edit to it, per
`design@core@one-question-one-command`. What each document starts with is
`design@core@nothing-of-a-project-is-compiled-in`'s.

### `init` follows `cargo init` wherever no argument departs from it `##init-follows-cargo-init`

Rust is the language the project serves first, per
`goal@knowledge-architect@rust-projects-come-first`, and `cargo init` is the command its users
already type to start a project. The owner holds that following `cargo init`'s usage as far as
possible is probably best, so a user's habits carry over to `init`: a choice about
`init` that `cargo init` already makes is made the same way unless an argument departs from it. The
owner holds this argument as the one that may shape later changes of `init`. The
members today are the name and PATH of `design@core@init-name-and-path`, the absence of a prompt of
`design@core@init-asks-no-confirmation`, the repository and `--vcs` of
`design@core@init-runs-git-init`, and the `.gitignore` append and the refusal of an existing
package of `design@core@rust-scaffold`. The departures, each argued in the head that makes it: the
refusal of a directory holding files outside a worktree, and `--vcs` restricted to two values, in
`design@core@init-runs-git-init`; the refusal of a `Cargo.toml` above PATH, in
`design@core@rust-scaffold`.

### `init` takes a PATH, created when missing, and names the project after its basename, as `cargo init` does `##init-name-and-path`

`klarch init [PATH] [--name <NAME>]`: PATH is `.` by default, and is created with its parents when
it does not exist; the name is PATH's basename unless `--name` gives one. Per
`design@core@init-follows-cargo-init`, `cargo init` is the command a Rust user already types for
this act, and `git init` uses the same word for a repository: `cargo init d` creates `d/` when it
does not exist, measured with cargo 1.98.0. `bootstrap <name>`, the rival, named the same act in a
word neither of them types for it.

### `init` declares the project's command at creation when `--command` names one `##init-declares-the-command`

`--command <CMD>` writes `[project] command` into the manifest `init` creates, per
`design@core@declared-command`. It serves a project whose command is neither `klarch`, the default
where none is declared, nor the `cargo klarch` that `--rust` declares, such as a script the project
keeps in its own tree. The rival, no flag, leaves such a project to edit
the manifest by hand after its creation, and a command left undeclared is the default, which no
check reports, so the messages and the installed files name a command that project does not run.

### `init` refuses a project name that begins with the installer's prefix `##init-refuses-the-installer-s-prefix`

A name that begins with `knowledge-architect-` is refused, exit 2, before any write: a project's
own skills take its name and a hyphen as their prefix by default, per
`design@agent-skills@project-name-prefix`, and under that name the prefix is the installer's
namespace, whose files every install deletes. The name `knowledge-architect` itself is accepted,
since its prefix, `knowledge-architect-`, is the namespace only with the hyphen.

### `init` asks for no confirmation, and every refusal comes before its first write `##init-asks-no-confirmation`

`init` reads no input. It refuses, exit 2, having written nothing, on anything at the manifest's
path, a name the manifest's parse complains of, a destination that is not a regular file or lies
under one, the installed set's destinations included, two destinations that overlap, an `index.md`
already at a generated path, whose text the index writer would replace without this tool having
generated it, a PATH that lies under a file, is a dangling symlink or lies in a `.git`
directory, a PATH or a Component git ignores, whose files the check would read as missing with no
repair but the rule, and the refusals of git, of the project's name and of `--rust`; once every
file is written it prints the absolute root and one line per file. Each file is created only where nothing stands, so one that appears during the run is
kept as well. A
prompt guards against a loss, and a run that refuses before any write and overwrites no
hand-written file outside the installer's namespace loses nothing an install would not, which is
the argument of `design@core@index-takes-no-dry-run`: the worst a mistaken run does is add files,
and install the shipped set as `design@core@owned-namespace-check` lets every install do. The checker's binaries run blind, as gates, per
`design@core@arguments-parse-through-clap`, and an agent's shell has no terminal, so a prompt
either blocks or is answered by a flag every caller passes. `cargo init` asks nothing either,
which `design@core@init-follows-cargo-init` makes a ground:
`cargo init </dev/null` in cargo 1.98.0 prints no question and exits 0. Once files are written a
failure exits 1, not 2, which promises a run that touched nothing, as
`design@core@fix-refusal-mixed-state` keeps it for `check --fix`. The one exception is a stdout
closed early, which ends every command with 2, per `design@core@exit-code-ladder`; `check --fix`
completes such a run.

The design this decision did not keep: `rejected@core@init-asks-for-confirmation`.

### `init` keeps a hand-written document that exists, so it serves an existing project `##init-keeps-existing-files`

A README.md, a CLAUDE.md or a register home that exists, in either shape of its register, is kept
byte for byte and listed as `kept`; the check then reports what it lacks, such as the primer's
import line. The installed set is the install's, and the install writes and deletes there, per
`design@core@owned-namespace-check`: inside a worktree as `check --fix` repairs it, from git's
listing, and outside one as `install-agent-skills` does. On the owner's word, `init` serves an
existing project, one the owner would migrate being the owner's example, and refusing whenever a
destination exists would leave every such project with the hand procedure. A manifest at PATH still refuses the run:
that project exists, and `new-component` is what extends it.

### Outside every git worktree, `init` creates a repository in a new or empty directory, and refuses one holding files unless `--vcs` decides `##init-runs-git-init`

The rule over PATH and `--vcs <git|none>`:

| PATH | no `--vcs` | `--vcs git` | `--vcs none` |
| --- | --- | --- | --- |
| inside a worktree | no `git init` | `git init` at PATH, unless PATH holds `.git` | no `git init` |
| outside, new or empty | `git init` | `git init` | no `git init`; the agent files, no index, no check |
| outside, holding files | refused, exit 2 | `git init` | no `git init`; the agent files, no index, no check |

`--vcs` is `cargo init`'s flag, per `design@core@init-follows-cargo-init`, restricted to `git` and
`none` of the five values cargo 1.98.0 takes, `git`, `hg`, `pijul`, `fossil` and `none`: the
checker reads every tree through git, per `design@core@git-supplies-the-walk`, so a repository of
another system would leave a project the check cannot run in.

Git is asked about PATH before the first write, whatever `--vcs` says, since the check the run
ends with reads the tree through git: a git that cannot answer refuses the run, and so does a
failure other than "not a git repository", such as a repository refused as another user's, which
would otherwise read as outside every repository. A variable that points git at another
repository's files refuses it too, `GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE`, `GIT_COMMON_DIR`
or `GIT_OBJECT_DIRECTORY`, since git can then answer from them, and a git hook sets one: a
pre-commit hook gets `GIT_INDEX_FILE`, measured with git 2.43.0, per
`issue@core@git-environment-variables-redirect-the-walk`. Inside a worktree no repository is needed: a project in a subdirectory of a repository passes the
check, measured by copying a passing project into a subdirectory of another repository and running
`klarch check` there, exit 0. A repository `--vcs git` creates there is nested in the outer one,
whose check reports it in phase 2 until an ignore rule or `[walk] skip-dirs` names it, as that
finding says. Outside one, the checker cannot run without a repository, per
`design@core@failed-git-is-exit-two`, and `cargo init` creates one: cargo 1.98.0 creates `.git` and
`.gitignore` in a directory outside every repository and none in a subdirectory of one. A departure
from `cargo init`, per `design@core@init-follows-cargo-init`, is the refusal of a directory holding
files, chosen by the owner over following `cargo init` exactly: after `cargo init` in such a
directory, `git status --short` lists the files that were there as untracked beside the new ones, so the cleanup of a mistaken run is
not `git clean`, which deletes both. Whether PATH is inside a worktree is asked through
`path@core@src/git.rs`, per `design@core@git-is-asked-through-one-module`.

The designs this decision did not keep: `rejected@core@init-git-init-anywhere-outside-a-worktree`
and `rejected@core@init-refused-outside-a-repository-root`.

### `new-component` adds a Component behind the writer gate, writing what the check requires of it `##component-command`

`klarch new-component <DIR>` runs inside a located project and behind the writer gate, so over a
model phases 1 to 3 find incomplete it refuses, exit 2, writing nothing, per
`design@core@phases-gate-the-report`. DIR is relative to the working directory, inside the root,
and is written relative to the root with `/` separators. A DIR in a `.git` directory, which no walk
reads, or in the installer's namespace, whose files every install deletes, is refused, by
`new-component` and by `init --component` alike. It writes the Component's required
documents and the home of every register it carries, through the code `init` uses, and ends as
`init` ends. A register declared `opt-in` is not guessed: the user names the Component in its
`components`, and the check reports the home. What a Component owes beyond its files, its goals and
the decisions it inherits, is no command's to write: `issue@agent-skills@a-skill-for-creating-a-component`.

### The manifest is edited with a format-preserving editor, and parsed before it is written `##component-manifest-edit`

`new-component` appends DIR to `[project] components` with `toml_edit`, which keeps every byte it
does not change, comments included, then parses the edited text with `Manifest::parse` and refuses
with its first complaint, such as a DIR named like the project, which names two anchors, before
writing anything. A manifest written with `\r\n` line breaks keeps them. The manifest is
hand-written and carries the reasons beside its rows, and the
`toml` crate, which the core reads it with, holds no comment in a value it writes back: its `Table`
is a map of values, as its source under the cargo registry shows. Printing the line for the user to
add was the rival: the command could then not finish in one run.

### `init --rust` writes the setup skill's maintenance crate, workspace and aliases `##rust-scaffold`

`--rust` refuses a `Cargo.toml` at or above PATH, above PATH a departure argued here under
`design@core@init-follows-cargo-init`, since adding the maintenance crate to an
existing workspace or package is a design of its own, as `cargo init` refuses an existing package,
exit 101 in cargo 1.98.0, and cargo reads a `Cargo.toml` above PATH as
an enclosing workspace, which a `members` glob covering PATH would break with two workspace roots.
It refuses as well any of the files it writes that already exists: kept, an older cargo
configuration would leave the declared `cargo klarch` undefined. Otherwise it writes the snippets of
`design@agent-skills@rust-templates-one-home`: a virtual workspace, `xtask/` pinned to this version,
and the cargo aliases with the project's checkout variable; it lists `xtask` as a Component and
declares `cargo klarch` as the command unless `--command` gives one. It makes `.gitignore` ignore
`/target`, keeping every line of an existing one and appending under `# Added by klarch`, as
`design@core@init-follows-cargo-init` asks and `cargo init` appends `/target` under
`# Added by cargo`, measured with cargo 1.98.0 over a `.gitignore` holding `mine`; or it leaves the file as it is where `target/` is already ignored. In a
repository git decides, a negation or a leading space included; outside one, under `--vcs none`, a
line `target`, `target/`, `/target` or `/target/` does. This is the
shape of `design@agent-skills@xtask-pins-checker`, and it gives a new project the gates
`goal@knowledge-architect@setup-brings-quality-tools` asks for.

### `init --rust` writes no CI workflow `##init-writes-no-ci-workflow`

The setup skill's block for a CI workflow is the one snippet of
`design@agent-skills@rust-templates-one-home` that `init --rust` does not write, on the owner's
word. A workflow file assumes a host, and the setup skill's block stays the route to one.

### `init` takes no contribution from an extension `##init-has-no-extension-hook`

An extension binary that dispatches `cli::init` writes a manifest holding none of the extension's
tables, and its first check reports each, with its repair, per
`design@core@an-extension-claims-its-manifest-tables`. On the owner's word, a project with an
extension would not need to run `klarch init`, so a hook would serve no adoption the owner
expects.

The design this decision did not keep: `rejected@core@init-extension-hook`.

## 8. Session transcripts

### Claude Code's session transcripts are read by a command of the core, `transcript`, so every binary offers it `##transcript-command-in-core`

`klarch transcript` reads the transcripts the agent harness writes on disk, for the transcript
reviewer, the assembly of a plan document and any survey of past sessions. It is a variant of the
core's command enum, so every binary that flattens it offers the command, per
`design@core@the-core-cli-is-a-library-module`, and it runs inside a project, after the version
refusal of `design@core@installed-binary-version-check`, whatever the project's
`[agents] harness`: the transcripts are the harness's own, whether or not the project installs the
workflow. It reads Claude Code alone, on the owner's word that the owner uses no other harness and
that another waits for a consumer's request, should one ever come; a second harness is
`issue@core@configuration-for-several-agent-providers`. The reader is internal; its argument types
are public through `cli::Command`, per `design@core@api-facade`.

It serves `goal@knowledge-architect@agents-get-a-complete-workflow`: every session that consulted a
transcript wrote its own script, and `goal@knowledge-architect@structure-and-workflow-work-together`,
under which the tool computes the workflow's work lists. The nearest rival, a command of the
maintenance tool, is never published, so a project that adopts the workflow, such as thaum, would
have no copy; a second crate or binary would be a second package to publish and pin, which
`design@core@single-crate` argues against.

### `transcript` answers three questions with three verbs: which sessions, what one session holds, and which lines match `##transcript-query-verbs`

`list` prints one row per session of the project, oldest first, with its handle, its first and last
time, its git branches, its count of lines the owner typed, of compactions and of subagents, and its
first message from the owner; `--branch`, `--containing` and `--since` filter. `show` prints one
session's kept lines in file order, each with its line number and its label, from `--from` a text
the owner typed, with `--subagents`, `--tools` and `--full`. `search` prints each kept line whose
text matches a regular expression, with `--role`, `--tool` and `--tools`. Each prints a header
first, per `design@core@transcript-format-tolerated-and-reported`, and `--json` prints one object
per row or line instead. The output, text and JSON alike, is for a reader and no contract, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`. The exit codes are
`design@core@exit-code-ladder`'s: 1 when a verb finds nothing to print, or `show`'s argument names
no session or more than one, or `--from` matches no line; 2 when the transcript directory, a file,
a regular expression or a date cannot be read.

`list --branch` computes the sessions that worked on a branch from the `gitBranch` field every chain
line carries, which the review before a merge needs, and `list --containing` the session holding
the message where a piece of work begins. The nearest rival, one verb that prints a whole
extraction, leaves those two lists and every survey to a script each session writes again.

### A transcript file is cut into sessions where the `sessionId` changes and where a root that is no compaction opens, and a session carries the subagent files it dispatched `##transcript-session-split`

A session opens at the first chain line of a top-level `.jsonl` file, at a chain line whose
`sessionId` differs from the previous one's, and at a chain line whose `parentUuid` is null that is
no `compact_boundary` line. A compaction and a resumed process stay in their session. A subagent
file, under the parent file's directory `subagents/`, is attached to the session of its `sessionId`,
and where that id holds several, to the one holding the `tool_use` its `meta.json` names. A
session's handle is its `sessionId`'s first 8 characters, with `#2` and on for a later session of
the same id. A line whose `uuid` was already read in the session is printed once and counted.

The rule turns on how the harness writes, measured over every transcript of the machine it was
designed on, Claude Code 2.1.306 to 2.1.310, on 2026-10-25: a compaction writes a boundary with a
null parent and keeps its file and its `sessionId`, in 7 of 7; a resumed session appends to its file
under its `sessionId`, its first new line with a parent, in 3 of 3; `/clear` starts a new file under
a new `sessionId`, while the process keeps the earlier id in its paths, in the owner's test of it.
Re-take it with `klarch transcript list --all-projects`, whose rows give one session per file and
count the compactions, and with
`jq -r 'select(.sessionId) | .sessionId' <file> | sort -u` over a file. The nearest rival, one
session per file, agrees with every file measured; the rule is kept because the format is
internal, per `design@core@transcript-format-tolerated-and-reported`, and a file holding two
sessions would read as one with nothing reporting it.

### Every kept line of a transcript is labelled by its fields, never by a substring of its text, and the command holds that rule `##transcript-labels-in-code`

Each kept line takes the label of the first row of the mapping it meets, over `type`, `isMeta`,
`isCompactSummary`, `origin.kind`, `toolUseResult`, `attachment.type`, `attachment.origin.kind`,
`isSidechain` and the `system` subtypes. The owner's words are the lines labelled `owner`,
`owner-queued`, `owner-answer` (an answer to a question tool) and `owner-command` (a command such
as `/clear`, read from its `local_command` line's `commandRun`); `injected` carries the kind of what
the harness or another agent delivered; `agent` is the session's text; `dispatch` is a field-less
`user` line of a subagent file, the text its dispatcher or its workflow wrote; `tool` is a tool call
or its result; a line no row places is `unlabelled`, printed and counted. A top-level file none of
whose `user` lines carries a field, such as one of a session run with `claude -p`, is labelled by a
leading harness tag instead, and the header says so.

The rule is learned from a failure: a filter on text substrings once dropped one of the owner's
messages from a review, and an owner's message that opens with a pasted tag reads as injected text
to any tag test. In code it has one implementation, with a test over a fixture of each row; the
rival, the rule kept in the transcript reviewer's text with the command as a helper, would hold it
in two homes, and every session applying the text wrote its own script. The fields are the
harness's, measured as `design@core@transcript-session-split` says; a field-less `user` line of a
subagent file was the dispatcher's brief or a workflow's task in every subagent file measured, but
one owner's interrupt.

### The transcript reader tolerates every line it does not know, and every run reports each one `##transcript-format-tolerated-and-reported`

The reader knows, and marks as used or ignored on purpose, the top-level types, the `system`
subtypes, the `attachment.type` values, the content item types and the `origin.kind` values the
harness wrote over the window it was measured on. A value outside them is counted, never a failure:
every verb's header names each with its count and its first file and line, with the count of
`unlabelled` lines, each line that is no JSON, and the files the tag fallback labelled, and ends,
where any of the first three is non-zero, with the line asking for it to be reported as a finding
on the checker in the retrospective. It serves `goal@knowledge-architect@the-workflow-improves-through-real-use`
by that route, per `design@agent-skills@unrecognised-entries-reach-upstream`.

The harness documents the format as internal: "The entry format is internal to Claude Code and
changes between versions, so scripts that parse these files directly can break on any release", at
https://code.claude.com/docs/en/sessions. Over the window measured, each version added types or
keys and none removed or renamed one the labels use; re-take it with the header of
`klarch transcript list --all-projects`, which names any value outside the sets. A strict reader
would fail on each release that adds a type, `rejected@core@strict-transcript-reader`; a silent
tolerant one hides a renamed field, against `design@core@a-failed-parse-is-loud`.

### A project's transcripts are the files whose first line ran in the project's root or under it `##transcript-directory-resolution`

The transcript directory is `$CLAUDE_CONFIG_DIR/projects/`, or `~/.claude/projects/` where the
variable is unset. A top-level file belongs to the project when the `cwd` of its first chain line is
the project's root or a path under it, both canonicalised where they exist, so a session started in
a worktree under the root is the project's; `--all-projects` reads every file. The harness names
each project's directory by a rule of its own, "your working directory path with non-alphanumeric
characters replaced by `-`", truncated at 200 characters with an undocumented hash and overridden by
`CLAUDE_CODE_PROJECT_DIR_NAME`, on the same page of its documentation: the nearest rival, recomputing
that name from the root, reproduces an internal rule and misses a worktree's sessions, which sit in
another directory.
