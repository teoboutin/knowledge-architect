# Check design: Extensions and the library API

## Extensions and the library API

### A domain's checks plug in as an extension compiled into the binary, and `klarch` registers none `##an-extension-plugs-in-through-phased-hooks`

The checker, `check` over `model`, checks what every project carries: the walk, the parse, the
entity table, references, registers, generated indexes and commit messages. A subject that belongs to one project, such as
thaum's verification of rule quotes against a pinned corpus, is an **extension**: a set of checks, the
manifest tables they read, and the files they generate. Each of its checks is listed and counted
like a check of `check`. This is what `goal@knowledge-architect@every-target-project-can-adopt-it`
requires: `check` is published for other projects, and nothing of thaum's subject is compiled
into it. A binary registers its extensions at
compile time. `klarch` registers none, and a project that needs an extension builds a
binary that registers it. There is no loading at run time, because Rust has no stable ABI a
plugin could be built against.

`check` calls an extension at fixed points of the run, and each call sits in the phase its output
belongs to, per `design@check@phases-gate-the-report`:

| call | when | what the extension does |
| ---- | ---- | -------------------- |
| resolve | phase 1, before the walk | reads the tables it claims, and returns its complaints, the paths it declares and the files it generates; `check` hands them to `model` as data, per `design@model@contribution-is-data`, which reports the complaints in phase 1, asserts in phase 2 that each declared path exists, and leaves the generated files out of the walk; `check` reads them as committed |
| prepare | once phases 1 to 3 passed | reads what its checks need for one tree, and may fail the run as could-not-run |
| check | phase 4 | its checks, with the count lines and the not-run names its summary prints |
| judge a message | `commits` | the rules a commit message is judged by, against that commit's prepared tree |
| generate | `index`, and phase 4 | the contents of the files it generates |
| dump | `model` | its observation rows |

**An extension is configured once per manifest and prepared once per tree.** `commits` judges several
trees in one run, and a message is judged against what its own commit's tree holds, so the
prepared state belongs to the tree and not to the extension. A parent tree, assembled for its entity
table alone, prepares no extension.

**`check`'s summary prints its own count lines, then each extension's**, and the list of checks
performed names `check`'s checks, then each extension's.

### An extension is handed a snapshot, `Tree::Snapshot`, for a commit's tree and for the tree git's index would commit alike `##an-extension-reads-a-snapshot`

`extension::Tree` has two variants: `Checkout`, the working tree, which an extension may read on
disk; and `Snapshot(&Snapshot)`, a tree read from git objects only, which is one commit's under
`commits` and the one git's index would commit under `check --staged`. `Snapshot::revision()` says
which. Under `check --staged` an extension is prepared with `Purpose::Check`, under `commits` with
`Purpose::Commit`, and under `index --staged` with `Purpose::Index`. `Snapshot::object_id` answers
only for a path the snapshot holds.

The index is not a third kind of tree for an extension: it is read through the same git objects as
a commit, so a third variant would make every extension write an arm reading the same objects as
the second. That departs from what `design@check@ne-minimal` asks of a new tree, that every
extension say how it reads it, and `design@check@trait-defaults` records the cost: an extension
that maps a snapshot to its commit-mode handling runs, under `--staged`, only what it runs for a
commit. thaum's rules extension lists its `changes` and `corpus` checks as not run there, and an
extension that lists nothing skips them silently. An extension that wants more under `--staged`
reads `Snapshot::revision()`.

**An extension reads the tree through `check`.** Over the checkout it may read the filesystem under
the root, because a subject such as a vendored corpus is filesystem state, per
`design@check@model-then-checks`. Over a snapshot it reads git objects only, through the same
batch reader `check` assembles that tree with, which also names each blob so that an extension can
cache what it parsed from one blob across commits.

The design this decision did not keep: `rejected@check@third-tree-variant-for-index`.

### An extension scans `check`'s parse on its own, and `check`'s model carries nothing for it `##an-extension-builds-its-own-model`

`check`'s model holds what the walk read and the parse produced, and the observations `check`'s
checks read. It has no field that exists for an extension. An extension reads the parse through `check`'s
public API: each document's text, its prose regions and scopes, which spans are code, the Rust
names, the inert lines and the fences. It scans a second time and keys what it finds by document,
in the model it was given.

This is what keeps `check` free of any one project's subject, which
`goal@knowledge-architect@every-target-project-can-adopt-it` requires. A model generic over an extension's
observation type would carry a type parameter into every check signature for the same data, and
a type-erased slot per document would hold it inside `check` with no type checking; both are
`rejected@check@model-generic-over-extension` and `rejected@check@type-erased-document-slot`.

**The cost is the extension's own scans.** Each document is still read once and parsed once, per
`design@check@model-then-checks`, and an extension scans the parse as often as its checks
need: thaum's rules extension scans each document in each check that reads it. Over thaum's
tree the extension's scans cost 0.07 s per `check`: the wall time of thaum's declared command's
`check`, run three times with the scan in `check` and three times with it in the extension, on
one machine, compared. A measurement that grows to a share
of the run comparable with the walk and the parse reopens caching the scan inside the extension.

**An extension cannot add a kind to the entity table.** A reference kind is `check`'s, so an extension's
subject cannot be cited as `<kind>@<anchor>@<id>`. Nothing needs that today.

### An extension claims the manifest tables it reads, and `model` refuses a table nobody claims `##an-extension-claims-its-manifest-tables`

`model` parses the tables `check` owns. Every other top-level table goes to the registered extension
that claims it. A table that no registered extension claims is a phase-1 finding, and so is a table a
registered extension claims that the manifest does not hold.

**Both refusals serve `design@check@the-regime-has-no-opt-out`.** An unclaimed table is a
declaration nothing reads, which a session would take for a regime in force. A missing claimed
table would let a manifest switch an extension off by leaving its table out. So the binary decides
which extensions run over a tree, and the manifest cannot remove one: `klarch` run over a
manifest that declares thaum's `[rules]` table reports the table as unclaimed, rather than
skipping the regime in silence.

**A table keeps the extension's own name at the top level.** A namespace for extension tables is
`rejected@check@extension-table-namespace`.

### A typed command prints as it works and returns its exit code, rather than data for its caller to print `##a-typed-command-prints-and-returns-its-code`

Each command's function in `commands` takes the domain's types, prints, and returns `Result<ExitCode, String>`. The session chose
this over a function that returns data for `cli` to print and map to a code, which no goal ranks; the owner approved it.
Printing stays where the data is defined. The rival meets a frontend that is no command line whole, and costs a rewrite of
the printing sites, some inside a write loop, with the order of what `check --fix` lists at risk. The cost of the choice:
such a frontend names no clap type, and receives stdout, and a process that ends on a closed stdout. A command can return
data later, one at a time, which `issue@check@a-typed-command-prints-and-exits-rather-than-returning-data` records on the
owner's word.

### `check`'s role modules publish what a consumer of the library needs, and nothing for `check`'s own tests `##the-facade-publishes-what-a-consumer-needs`

The library has four kinds of consumer: a binary that registers an extension, the extension, the
extension's tests, and a frontend that is no command line, which alone names `commands`, through this package. **Testing an extension over a mock project is part of what the library
publishes**, because a project that adopts the tool with a subject of its own, per
`goal@knowledge-architect@every-target-project-can-adopt-it`, has to test that subject as `check` tests
its own. `design@check@a-foreign-build-is-refused` already requires every extension binary to test
the refusal in its own suite. Its facade is shaped as `design@knowledge-architect@api-facade` says.

`cli` keeps each consumer's path by re-exporting `extension`, `document` and `testing` from its own facade, per
`design@knowledge-architect@the-binary-crate-keeps-the-package-name`, beside its module `cli`, the role a binary's `main` meets.
`commands` and `extension` are both a role module and the code of that role; `document` and `testing` only re-export:

| module | what it serves |
| --- | --- |
| `commands` | a frontend, the command line among them: each command's logic behind a typed function, refusing a foreign build or another version, gathering a run's inputs |
| `extension` | writing an extension: its two traits, what `check` hands them, what they return |
| `document` | reading a document's parse, per `design@check@an-extension-builds-its-own-model` |
| `testing` | running `check` over a mock project as the binary does |

- **What `model` publishes is `model`'s facade, per `design@model@model-facade`**, and this facade names
  of it what an extension meets, in its root and its role modules, and neither the entity table nor the
  anchors. A method a caller in this crate needs of `Model` or `Manifest` is public
  on these re-exports too, since a re-export carries a type's methods.
- `Inputs` is in `Prepared::check`'s signature and its fields expose `Entry` and `Outside`, so those are public although no
  consumer names them.
- **A value only `check` produces is read, not built.** `Report` and `Snapshot` have public read-only
  methods and no public constructor; `Outside`'s is `model`'s, whose variants a reader matches. A value a consumer starts from, such
  as a `Manifest`, a `Model` or a `Finding`, keeps its public constructors.
- **Nothing becomes public for `check`'s own tests.** A test that needs a private item is a
  unit-test module inside the crate, as `path@check@src/mock_projects.rs` is.
- `Manifest::command` is public although thaum's extension does not call it: an extension's
  finding that names a repair command owes the project's declared command, per
  `design@check@declared-command`.

The designs this decision did not keep: `rejected@check@doc-hidden-test-items` and
`rejected@check@testing-feature-for-test-items`.

### What a run fetches before any check is gathered once, and no caller outside the crate assembles `Inputs` `##inputs-are-gathered`

Two gatherings, one in each of `model` and `check`: `Gathered` of `model` reads what phases 1 to 3 and the resolver read,
per `design@model@tree-inputs-gathered-here`. `Gathered`, in `check`'s command module, holds it, adds
the committed generated files, every `register.toml` and the shipped text, which the last phase alone reads, and lends
them all as the `Inputs` a check of the working tree reads. `check`, the
commands that write files into the working tree (through `complete_working_tree`, which refuses
over an incomplete model), and an extension's tests all build on it. `check --staged` and
`index --staged` read a snapshot instead, and build their `Inputs` from it, per
`design@check@staged-tree-source`. So no caller outside the crate assembles
`Inputs`, which `design@check@ne-minimal` makes impossible for a literal. A new input is then a
change inside the gathering that reads it. `check`'s own mock-project tests still assemble `Inputs` by hand:
some of them hand a check an input that differs from the tree's, such as no committed file.

The design this decision did not keep: `rejected@check@inputs-struct-literal`.

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
- **For an enum `check` hands an extension, the compile error is wanted.** A third
  `extension::Tree` must make every extension say how it reads it. A wildcard would read the
  working tree while the run judges another tree: a wrong verdict, with exit 0. The tree git's
  index would commit is no third kind, since it is read from the same git objects as a commit, per
  `design@check@an-extension-reads-a-snapshot`, which records the cost.
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

The design this decision did not keep: `rejected@check@non-exhaustive-on-growing-enums`.

### A hook added to `Extension` or `Prepared` has a default body only when doing nothing is a correct answer `##trait-defaults`

The two traits have no default method. A hook added later gets a default body only when an
extension that does not know the hook is still right to do nothing, such as a new dump. A hook
whose absence would make a verdict wrong is added without a default, and adding it is a breaking
change. The argument is the one of `design@check@ne-minimal`: suppose a pre-commit mode adds
`Prepared::check_staged` with a default that returns no findings. Every existing extension
compiles unchanged, its checks silently do not run in the new mode, and the run reports success.
`check --staged` adds no hook: it hands an extension a snapshot, as `commits` does, and what an
extension runs over one is its own, per `design@check@an-extension-reads-a-snapshot`, which records
where that falls short of this argument.

The design this decision did not keep: `rejected@check@no-op-hook-defaults`.

