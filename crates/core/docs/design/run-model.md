# Core design: The model, the phases and the findings

## The model, the phases and the findings

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
