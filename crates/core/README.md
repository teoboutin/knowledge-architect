# The core checker

How a project's knowledge is held: its documents, the references between them and the registers
they carry. This Component is the generic core. A subject that belongs to one project is an
extension of it, per `design@core@an-extension-plugs-in-through-phased-hooks`.

**The core's binary, and a binary per set of extensions.** The core's own binary, `klarch` in the
package `knowledge-architect`, runs every command below with no extension registered. A binary
that registers extensions runs the same commands unchanged, and its extensions' own commands beside them, per
`design@core@the-core-cli-is-a-library-module`. Every such binary hands `init` to `cli::init`
before it locates a project, since `init` is what creates one. Over a manifest holding a table no
extension of the binary claims, the binary reports that table in phase 1, rather than skip in
silence what the table configures, per `design@core@an-extension-claims-its-manifest-tables`.
The commands below are written `cargo klarch <command>`, the command this repository declares: a
cargo alias over the binary it runs, built from the checkout. A project declares its own command
with `[project] command`, and `klarch` is the command when it declares none. The checker prints
the declared command in its messages, in the header of every generated index, and in the agent
files it installs, per `design@core@declared-command`.

**The library is for a binary that registers an extension.** Its public API, and how to write and
test an extension with it, is described at <https://docs.rs/knowledge-architect>, for each
published version. The text is the crate documentation of `path@core@src/lib.rs`.

**It needs `git` 2.36 or newer on the path, and a project inside a git worktree.** What the tool reads is what
`git ls-files` reports from the project root, so every pattern git honours decides the walk,
nested `.gitignore` files included, and a file git tracks is read whatever the ignore rules say.
A file whose name holds a line break, or a name Windows cannot create, is read by nothing and
reported once with the reason, any line break escaped: name it in `[walk] skip-files` or in an
ignore rule to keep it.
Neither failure is silent: no binary and no worktree are both exit 2 naming the reason, never an
empty walk. `commits` reads a commit's tree through `cat-file --batch -z`, which is where the
version floor comes from. `check --staged` reads the tree git's index would commit the same way:
HEAD's tree with the staged changes, a file added with `git add -N` left out, as `git commit`
leaves it out. The decisions are `design@core@git-supplies-the-walk`,
`design@core@failed-git-is-exit-two`, `design@core@git-floor-for-commit-trees` and
`design@core@staged-tree-source`.

```sh
cargo klarch check                     every check, over one walk, in four phases
cargo klarch check --fix               apply the admitted safe fixes, list each, then every check
cargo klarch check --staged            every check, over the tree git's index would commit
cargo klarch show <kind>@<anchor>@<id> one recorded entry, and every reference to it
cargo klarch id <id>…                  whether each id is free, or every entity defined under it
cargo klarch issues [anchor] [--kind k] [--group g] [text …]
                                       every issue entry, one row each
cargo klarch tripwires [anchor] [--guarding <ref>] [text …]
                                       every tripwire entry, and what each guards
cargo klarch index                     regenerate every generated index in place
cargo klarch index --staged            stage the generated files the tree git's index would commit needs
cargo klarch model                     every observation the walk produced
cargo klarch anchors                   every anchor, its kind, its path and each register's home
cargo klarch commits <range>           judge every commit in the range, message and tree, against its own tree
cargo klarch install-agent-skills      write the agent files this version ships, remove the ones it does not
klarch init [PATH] [--name n] [--component DIR]… [--rust] [--vcs git|none] [--command c]
                                       create a project, overwriting no file that exists
cargo klarch new-component <DIR>       add a Component: its manifest line, its documents and homes
cargo klarch worklist --out <dir> [--seed <ref>]…
                                       the work list of a search of the whole record, one directory per agent
cargo klarch worklist --verify <dir>   check the reports the agents filled, and assemble the graded entries
cargo klarch transcript list [--branch b] [--containing t] [--since d] [--all-projects]
                                       the sessions of Claude Code's transcripts that ran in the project
cargo klarch transcript show <session> [--from t] [--subagents] [--tools] [--full] [--all-projects]
                                       one session's lines, each labelled by its fields
cargo klarch transcript search <regex> [--role r]… [--tool n] [--tools] [--branch b] [--since d]
                         [--subagents] [--all-projects]
                                       every line whose text matches
cargo klarch --version                 the version of the checker that runs
```

## Exit codes

Three, per `design@core@exit-code-ladder`, and the third is what makes the other two mean anything.

| code | meaning | where it comes from |
| ---- | ------- | ------------------- |
| `0` | the command ran and its subject is in order | `--version`, from any directory; `check` with no findings; `show` on a reference that resolves; `id` with every argument an id nothing defines; `issues` and `tripwires` with at least one row; `index` having written every destination, or found each already current; `index --staged` having staged every generated file whose staged bytes differed, or found each already current; every `model` run; `worklist --out` having written the work list, notices included; `worklist --verify` on complete reports; `transcript` having printed at least one row or line; `anchors` with every home it prints present; `commits` with no finding, an empty range included; `install-agent-skills` having written and removed what it had to, or found nothing to do; `init` and `new-component` having written, with the final check passing, and `init` under `--vcs none` outside a worktree, where no check runs |
| `1` | the command ran and reports a negative answer | `check` with findings, `--fix` included; `check --fix` when a write failed, a destination was refused, a partial commit's mismatch was refused, or the final check could not run, after another file was already written; `show` on a reference that resolves to nothing; `id` with an argument some entity defines, or one that is no id; `issues` or `tripwires` with no row; `anchors` with a home missing; `index` when a write failed after another destination was already rewritten; `commits` with a finding against a judged message or a failing tree; `worklist --verify` with a finding on a report; `transcript list` or `search` finding no session or no line, `transcript show` on a session it cannot name, with a `--from` text no line the owner typed holds, or printing no line; `init` and `new-component` having written, when the final check finds anything, cannot run, or a write failed |
| `2` | the command could not run, `anchors` included | an unknown or invalid argument, `--fix` with `--staged` among them, a `show` argument that is not reference-shaped, a binary built from another checkout of the tool, which names the `cargo clean` that rebuilds it, a binary of another version than `[project] checker-version` pins, or a pin it does not confirm, or a manifest with no pin, `index` or `check --fix` refusing a destination — a symlink, or a directory that is not there — having written nothing, `check --fix` refusing a partial commit's mismatch having installed nothing, `check --staged` or `index --staged` over an index that holds an unmerged path or a staged tree that cannot be read, `index --staged` refused by its gate, a destination or git's index lock, having staged nothing, `commits` on a range that does not resolve, no project above the working directory, no `git` on the path or a project outside a worktree, `install-agent-skills` over a manifest holding a refused declaration, over a symbolic link on an owned path, or when a write or a removal failed, naming the path, a stdout closed before the output was written — as `\| head` does — which ends the run silently, an input that cannot be read, an input an extension prepares that it cannot resolve, `init` refusing, having written nothing: anything at the manifest's path, a refused name, a DIR `--component` names in a `.git` directory or in the installer's namespace, a destination that is not a regular file or lies under one, two destinations that overlap, an `index.md` already at a generated path, a non-empty directory outside every git repository with no `--vcs`, a PATH git ignores, in a `.git` directory or under a file, a variable such as `GIT_DIR` or `GIT_INDEX_FILE` that points git at another repository's files, a git that cannot answer, a `Cargo.toml` at or above PATH or a file `--rust` writes already there, a `.gitignore` it cannot read; `new-component` refusing, having written nothing: an incomplete model, a DIR outside the project, in a `.git` directory or in the installer's namespace, or one the manifest refuses, a destination it cannot write; `init` dispatched inside a project by a binary that skips `cli::init`; `worklist --out` refusing, having written nothing: an incomplete model, a directory inside the project or not empty, a seed that names no entry; or a write that failed, naming the path, the files written before it left in place; `worklist --verify` over a file it cannot read or parse, or on passing reports whose snapshot is missing or lacks an entry, writing no assembly; `id` over an incomplete model; `transcript` with no transcript directory, over a file it cannot read, or with a regular expression or a `--since` date that does not parse |

**A caller scripting against a run reads the exit code; a person reads the last line.** Arguments
are refused before the project is located, so `--help` answers from anywhere and a mistyped
invocation costs no walk.

## `check`

**A run prints the summary first, its findings under it, and its verdict on the last line** —
`PASSED: no findings`, or `FAILED: n findings above`. The verdict is derived from the finding list
rather than tracked beside it, so it cannot disagree with the exit code. **The order is a
contract**: the summary block prints on a failing run as well as a passing one, so a reader taking
the tail of the output has to reach the verdict rather than the counts. When a finding sits in a
file git does not track, one note naming each such file follows the findings, above the verdict:
the walk reads untracked files, and a scratch file left in the tree fails the run like any other.

**The `checker source:` line names the binary's own source directories**, compiled into it, and
counts the walked files under them whose string literals are read as data rather than prose, per
`design@core@checker-source-literals-are-data`. A directory inside the tree is printed relative to
the root, and one outside it as an absolute path. A binary installed with `cargo install` names its
source in cargo's registry, outside the tree, so its count is 0 and nothing of the project is read
differently. A maintenance crate that runs the checker names its own directory, as the setup
skill's `main` does, so the literals of that crate's Rust files are data, and its comments stay
prose.

**A run is four phases, and it stops at the first that finds anything.** Phase 1 resolves the
manifest: a declaration the tool refuses is reported and acted on by nothing. Phase 2 reads the
tree against what the manifest declares: a file the walk could not read, a name it refuses, an
anchor or a register home that is not there, a home a walk row keeps out, a declared path that
does not exist, a file git both tracks and ignores, a symlink or a submodule, a heading markdown
reads and the checker does not, and, under the `claude` agent
harness, an installed agent file missing, differing or unshipped, a deletion of one not staged, or
a root CLAUDE.md that does not import a shipped primer. Phase 3 builds the entity table: a slug or an entry id where none may sit, or
defined twice. Each of these says the model is incomplete, and a finding computed from the model
afterwards would be unreliable in both directions, so the run prints that phase's findings, says
which phases were not judged, and exits 1. Phase 4 is every check, over the complete model: the
core's `generated`, `registers` and `references`, each a module under
`path@core@src/check/`, then each registered extension's, per
`design@core@an-extension-plugs-in-through-phased-hooks`. `references` judges
every `` `<kind>@<anchor>@<id>` `` reference against the entity table — a register kind against the
entries its home defines, the `path` kind against the tree, the `planned` kind against its absence
from the tree, the `register` kind against the anchors that carry the register, and, under the `claude` harness, a skill, an agent, the primer and the root
CLAUDE.md, cited with no anchor as `skill@<name>@<slug>` — and reports the retired slug
reference form and the unanchored path shape where either names something of this project, and
a defined id written outside its reference form: a reference-shaped token outside backticks, per
`design@core@reference-shape-outside-span`, an id written in backticks alone, per
`design@core@backticked-bare-id`, and an id written as a plain word, per
`design@core@plain-word-lint`. An id several entries define gets one finding listing each
reference, and a quotation, between double quotes or on a blockquote line, is read by none of the
three, per `design@core@quotation-read-by-no-id-lint`.
The directory of an anchor other than the root has no spelling under its own name, so a reference
names it from the anchor above it, as `path@<parent-anchor>@<dir>/`, per
`design@core@every-path-names-its-anchor`. An illustration of a form writes its id as a placeholder
in angle brackets, as `path@*@<path>`, per `design@core@a-reference-is-live-in-every-prose`: a span
with an empty id is malformed.
`registers` judges the shape of what each anchor carries: a file
register's README, index, groups and entry shapes, and a directory home's links. A check the tree
gives no input to is printed as not run rather than counted. There is no way to select a subset:
the checks cross the phases, and a run over a passing tree costs under a second. The argument is
`design@core@phases-gate-the-report`.

`index`, `index --staged` over the staged tree, `check --fix`, `new-component` and a writing
command of an extension run the first three phases too, and
write no generated file while one of them holds anything; `init`, which writes where no project
exists yet, checks every destination before its first write instead, and writes the indexes only
once the three phases find nothing. `index` and an extension's command refuse
with exit 2; `check --fix` prints the stopped report and exits 1, having installed agent files
before that gate when they needed it, since their bytes do not depend on the model: an index generated over an incomplete model lists rows nobody asked for.

**`check --fix` applies the safe fixes `design@core@fix-scope` admits, then runs the check**, per
`design@core@check-fix-flag`. A fix is safe, per `design@core@safe-fix-definition`, when its
bytes are determined by the tree and the pinned version, and it writes or removes only files of
the installer's namespace or of the generated list. Two pass: the agent files the check reports missing,
differing or no longer shipped, judged from git's listing as the check judges them, so a file git
does not list, such as an ignored swap file, is never removed; and every stale or missing generated
file. Each file written or removed is listed on a line `fixed: wrote <path> (<kind>)` or
`fixed: removed <path> (installed)`, before the report. Every other finding's repair is a choice,
or touches git or a hand-written file, and stays the reader's.

- The order, per `design@core@fix-before-the-checks`: a manifest holding a refused declaration
  writes nothing; then the agent files; then
  the first three phases, over the tree as the install left it, which stop the run and write nothing
  more; then the refusal of a partial commit's mismatch, below; then the generated files; then the
  full check, whose report and exit code are the run's. With nothing to fix, the output is a plain
  `check`'s.
- **A partial commit's mismatch is refused**, per `design@core@fix-refusal-mixed-state`. Where the
  index differs from HEAD, and a generated
  file `--fix` would write differs from the one the tree git's index would commit needs, `--fix`
  writes no generated file and names each such file, with the two repairs: `index --staged`, then
  `check --staged`, to commit the staged changes; `index`, to fix the working tree as a whole.
  Staged with a partial commit, the files `--fix` writes would make a commit whose tree fails. A
  file already current on disk is not written, so it is not refused, and after `index` the run
  writes nothing and refuses nothing. It exits 2, or 1 when agent files were already installed.
  Where no staged tree is there to commit, an unmerged index, a staged tree holding no manifest or
  one stopped in phases 1 to 3, `--fix` compares nothing and runs; any other failure to compute the staged side,
  such as an extension that cannot prepare over a snapshot, refuses the same way, naming it.
- **`--fix` with `--staged` is refused** while parsing, exit 2: `--fix` repairs the working tree.
  For the staged tree, run `index --staged`, then `check --staged`.
- **An upgrade that removes a shipped file takes two runs.** The install deletes the file, git
  still lists it, and phase 2 reports the deletion as not staged, so the generated files wait. Run
  `check --fix`, stage the deletion with `git add`, then run `check --fix` again: no fix touches
  git.
- A write that fails exits 2 when nothing was written yet, and 1 after any write, as `index` does.
- `--fix` belongs to `check` alone. `commits` judges history and takes no `--fix`, and the gates
  run `check` without it, so continuous integration judges the tree as committed.

## `show`, `issues` and `tripwires`

The three commands that print what is recorded, given a reference or an anchor. All three read the
entity table, so what they print is what `check` resolves against.

**`show <kind>@<anchor>@<id>`** prints one entry and then every reference to it, as `file:line`:

- a file register's entry is printed **whole**, frontmatter included, and a directory register's
  entry as its `README.md`, whole;
- a heading register's entry is printed as **its section** — the heading through to the next
  heading at the same level or shallower;
- `path@<anchor>@<path>` prints the walked document's text, or says the path is outside the walk;
  its inbound list includes the `planned` citations of the same path.
- `planned@<anchor>@<path>` says whether the tree's listing holds the target yet, and lists the
  plans that cite it, with or without the trailing slash; one the check refuses resolves to nothing.
- `register@<anchor>@<register>` prints the home the tree holds, the file or the directory, with
  the register's shape, for each Component under `*`; one the check refuses resolves to nothing.

**The two failures are different questions**: an argument that is not reference-shaped is exit 2
with the grammar named, and a reference the grammar accepts that names nothing is exit 1.

**`issues [anchor] [--kind k] [--group g] [text …]`** prints one row per issue entry — kind,
anchor, id, title, last change — sorted by kind then id. **`tripwires [anchor] [--guarding <ref>]
[text …]`** prints one row per tripwire entry — anchor, id, title, and every reference the
entry carries to what it guards — sorted by anchor then id. What a tripwire guards is an entry of
any register but `issue`, `tripwire` and `rejected`: a decision, a goal, or an entry of a register
the manifest declares, per `design@core@tripwire-guards-no-rejected-entry`. For both, the
text keeps the rows whose id or title contains it, case-insensitively, and **no row is exit 1**.

**The first positional argument is an anchor when something declares that name**, and text
otherwise, which is what lets one positional list mean both. A search for a word that is also an
anchor name is written with the anchor before it.

**The last-change column is git's**, taken in one `git log` for every issue instance at once. An
entry with no commit shows `uncommitted`; where git cannot answer the column shows `-`.
A date never enters a generated file, per `design@core@generated-files-are-pure` — it is printed
here and nowhere else.

## `index`

Regenerates every generated file in place, from one walk: each file a registered extension
generates, and one `index.md` per file-register instance whose directory is there. It takes no
dry-run flag and **writes only where the bytes differ**, naming each file it rewrote:

```
$ cargo klarch index
docs/open-issues/index.md                rewritten
crates/core/docs/open-issues/index.md    already current
```

**A file-register index is a fixed shape, and its bytes are a contract**, per
`design@core@a-file-register-index-is-rows`:

```markdown
**Generated — do not edit.** `<command> index`

2 entries

| kind | title |
| --- | --- |
| defect | [A thing that is broken](a-thing-that-is-broken.md) |

## a-group

| kind | title |
| --- | --- |
| todo | [Work left undone](a-group/work-left-undone.md) |
```

The columns are the register's declared metadata keys ordered by name, then the title as a link
relative to the index's own directory. Ungrouped entries come first, under no heading; rows sort by
the first metadata key and then by id. No summary and no date: a row changes on create, delete,
retitle, regroup and a metadata change, and on nothing else.

**Every generated file is outside the walk by construction** — the tool derives the set, each file
an extension generates and one index per file-register instance, from the manifest, so no `[walk] skip-files` row names one
and none can be created inside the walk.

Running it to look therefore costs nothing, not even an mtime. **Whether a generated file is
current is not this command's question** — that is `cargo klarch check`, whose `generated` check
is a gate and names the first line at which the committed file and the regenerated one disagree.
The two halves are `design@core@index-takes-no-dry-run` and `design@core@one-question-one-command`.

**`index --staged` writes into git's index instead, for a commit of part of the working tree**,
per `design@core@index-staged-write`.
It generates each file from the tree git's index would commit, which is HEAD's tree with the
staged changes, and sets the staged entry of each one whose bytes differ. No working-tree file
changes: after it, `check --staged` passes on what will be committed, and the working tree keeps
the rows of the working tree for a later commit.

```
$ cargo klarch index --staged
docs/plans/specs/index.md                already current
docs/open-issues/index.md                staged
```

It refuses with exit 2, having staged nothing, when phases 1 to 3 of the staged tree find
anything, when a generated path is staged as a symlink, a gitlink or a directory, or its directory
is not in the staged tree, when the index holds an unmerged path, and when git cannot take its index lock.

## `install-agent-skills`

Under the `claude` agent harness, the default, the checker installs the agent skills, subagent
definitions and a primer, and this command writes them into the project's `.claude/` directory,
each rendered with the project's declared command. It removes every file of its own namespace the
version does not ship: a skill directory, an agent file or a saved workflow file under
.claude/workflows/ whose name starts with `knowledge-architect-`, and the directory
.claude/knowledge-architect/. A project's own skill
takes a name of its own, and the command never touches it. It never edits the root CLAUDE.md
either: when the primer is shipped and the root CLAUDE.md does not import it, it says so, and the
line to add is one holding exactly an at sign followed by .claude/knowledge-architect/PRIMER.md.

Commit what it writes. `check` compares each installed file with what the version ships, byte for
byte, and stops in phase 2 on a missing, differing or unshipped file, so a project that moves to a
new version runs the install with the move. The decision is `design@core@owned-namespace-check`.

```toml
[agents]
harness = ["claude"]   # the default when the table is absent
# harness = []         # no agent configuration: no install, no check, no CLAUDE.md required
```

With `harness = []` no component owes a CLAUDE.md, and nothing is installed or checked, per
`design@core@agents-table`.

**Adopting the workflow starts with `init`**, below, which writes the manifest and runs the
install. By hand, a project first holds the smallest manifest the install accepts, at its root,
`<version>` being the version of the checker the project runs, since the skill that sets a project
up, `skill@knowledge-architect-setup`, is one of the files the install writes:

```toml
[project]
name = "<project name>"
checker-version = "<version>"
components = []

[walk]
skip-dirs = []
skip-files = []
exclude = []
```

Then `klarch install-agent-skills` writes the skills, the agents, the saved workflows and the primer, and an agent
session follows the installed setup skill from there. `klarch check` lists every document the
project still owes, each with its repair.

## `init` and `new-component`

**`init` creates a project, before any agent session**, so the session starts with the workflow
installed. It runs from a binary its user installed, such as
`cargo install --locked knowledge-architect`; the project then runs the version that binary pinned,
as the setup skill says. Like `cargo init`, per `design@core@init-follows-cargo-init`, it takes a PATH, `.` by default, created when missing,
names the project after its basename unless `--name` is given, and asks nothing:

```sh
klarch init my-project              # the manifest, the agent files, every document and home
klarch init . --component crates/core --component tools/xtask
klarch init my-project --rust       # also the maintenance crate, its workspace and aliases
```

- **It overwrites no hand-written file outside the installer's namespace**, per
  `design@core@init-keeps-existing-files`. An
  existing README.md, CLAUDE.md or register home is kept and listed as `kept`; the check then
  reports what it lacks, such as the primer's import. The agent files are the install's, and it
  writes them inside a worktree as `check --fix` repairs them, and outside one as
  `install-agent-skills` does. Every refusal comes before its first write, exit 2,
  each listed in the table of exit codes above, per `design@core@init-asks-no-confirmation`. Each
  file is created only if nothing is there, so one that appears during the run is kept too. A manifest above PATH does not refuse
  it: a project nested in another is legal, and the run notes that the outer project walks it
  unless its `[walk] exclude` names it. A run stopped after its first write, such as by a stdout
  closed early, which exits 2 as every command does, is completed by `check --fix`.
- **Git, as `cargo init` does, with one guard**, per `design@core@init-runs-git-init`. Inside a
  git worktree no repository is created unless `--vcs git` asks for one, which the outer
  project's check then reports as a nested repository until an ignore rule or `[walk] skip-dirs`
  names it; outside every worktree a new or empty directory gets one. A directory there that already holds
  files is refused unless `--vcs git` or `--vcs none` decides, since an accidental run would leave
  its files untracked beside the project's, where `git clean` deletes both. Git is asked about
  PATH before anything is written, so a git that cannot answer, such as one refusing a repository
  owned by another user, refuses the run, and so does a variable that points git at another
  repository's files, such as the `GIT_INDEX_FILE` a git hook gets, per
  `issue@core@git-environment-variables-redirect-the-walk`.
- **It ends as `check --fix` does**: it installs the agent files, writes the indexes once the first
  three phases find nothing, and runs the check, whose verdict is its exit code. Under `--vcs none`
  outside a worktree it installs the agent files and stops before the indexes, and says so.
- **The documents hold no entry.** Each opens with a title and a line on what it holds, or that
  it holds no entry yet. No goal is written: the goals are stated with the owner. The root CLAUDE.md is its title and the
  primer's import line.
- **`--rust`** writes the blocks the setup skill shows: a virtual workspace, the maintenance crate
  `xtask/`, pinned to this version, and the cargo aliases, declares `cargo klarch` as the command,
  and makes `.gitignore` ignore `/target`, appending to an existing one unless git already ignores
  it there. It writes no CI workflow, per `design@core@init-writes-no-ci-workflow`. Its refusals
  and their reasons are `design@core@rust-scaffold`.

**`new-component <DIR>` adds a Component to an existing project**: it appends DIR to
`[project] components`, keeping every other byte of the manifest, comments included, per
`design@core@component-manifest-edit`, then writes
the Component's documents and register homes, and ends as `init` does. A manifest written with
`\r\n` line breaks keeps them. It runs behind the gate of
the first three phases, so over a tree they find incomplete it refuses, writing nothing. A
register declared `opt-in` is not guessed: name the Component in its `components`, and the check
reports the home.

## `model`

Every observation the walk and the scanner produced, one per line, as `file`, `line`, `kind`,
`value`, tab-separated on stdout; the document and observation counts go to stderr, so redirecting
stdout gives a file that is only observations.

## `anchors`

Every anchor of the project, as `check` builds them: the Components and the locations the
manifest declares, then `plans`, each entry of a Directory register, a milestone included, and
each spec. One line per anchor gives its name, its kind and its path; an entry's kind is the
register it is an entry of. Under every anchor but a plan's, one line per register it carries
gives the home the tree holds: the file, the directory, both where both exist, or `missing`, which
`check` reports with its repair. A plan's items live in the plan's own documents, so a milestone
and a spec list no register. The output is for a reader, not for a script.

## `worklist`

The work list of a search of the whole record, for the record searcher the workflow installs.
Every entry of every register but the plan registers is grouped into **clusters**: each is rooted
at a unit of `design` or of a declared register, cut by the headings or the groups of its home, as
a design home is cut at its level-two subjects, and holds the `rejected` entries its entries cite,
the issues and tripwires that cite them, and the goals they cite with every goal those refine. The
rule for each shape of register is the design head `design@core@cluster-rooted-at-a-subject`. An entry no
cluster holds is in its anchor's **residual group**. The groups are packed into the fewest agents,
the load spread evenly, under a bound in words.

**`worklist --out <dir> [--bound <words>] [--seed <ref>]…`** writes into `<dir>`, a new or empty
directory outside the project:

- `agents.txt`: one line per agent and entry, `<agent> <role> <reference>`, with `seed` or
  `cites-seed` after an entry that is or cites a seed;
- per agent, `agent-<n>/entries.md`, every entry it is given, whole, cluster by cluster, and
  `agent-<n>/report.md`, the skeleton it fills, one line per entry;
- `shown.md`, the **snapshot**: every entry of the record as `show` prints it, its inbound list
  included, each after a line `=== <reference>` and in a fence; no agent is given it.

It prints one line per agent with its load and clusters, and a **notice** for each cluster or
residual group above the bound, 25,000 words by default, which goes to an agent alone: a subject
named so is split into subjects read apart. It refuses, writing nothing, a tree that stops before
the last phase of `check`, a directory inside the project or not empty, and a seed that names no
entry.

**`worklist --verify <dir>`** reads `agents.txt` and every report, and reports each entry graded
nowhere or twice, a reference no agent was given, a grade other than `bears`, `may-bear` or
`unrelated`, a `bears` with no relation or one outside its role's set, a relation on another grade,
a missing reason, an entry that is or cites a seed graded `unrelated`, and a report with no
section `## Met outside the task`. It reads no model, so it judges the reports against what the
agents were given.

When every report passes, it writes the **assembly** from the snapshot: `bears.md`, every entry
whose highest grade over every report is `bears`, and `may-bear.md`, every entry whose highest grade
is `may-bear`, after a table of contents with each entry's reasons. Above each entry, every agent's
bearing grade line, identical lines merged; then its show text, or a goal's first line alone. It
removes a previous assembly first, so a run that fails or stops leaves none. A snapshot that is
missing or lacks an entry the list gives, as an older `--out` wrote, writes no assembly and exits 2.

## `id`

**`id <id>…`** answers, for each argument in order: `free` where nothing defines it; every entity
the project defines under it, one per line, as the reference `show` takes, of every kind and every
anchor; or `not an id` where it is outside the grammar `[a-z0-9]+(-[a-z0-9]+)+`. Which collision
matters is the reader's rule: a design thread's slug, for instance, collides only with a design or a
`rejected` entry of the Component that will own it. It refuses an incomplete model, as a writer
does, since an entity an unread file defines would read as free.

## `transcript`

The sessions of Claude Code's transcripts that ran in the project, for the transcript reviewer, the
assembly of a plan document and any survey of past sessions. It reads only the harness Claude Code
writes, under `$CLAUDE_CONFIG_DIR/projects/`, or `~/.claude/projects/` where the variable is unset.
A session belongs to the project when the `cwd` of its first chain line is the project's root or a path
under it, as a worktree under the root is; `--all-projects` reads every project's.

- **A session** is a run of a transcript file cut where its `sessionId` changes or where a new root
  opens, with the subagent files it dispatched. A compaction and a resumed session stay in their
  session; `/clear` starts a new one. Its handle is its `sessionId`'s first 8 characters, with `#2`
  and on for a later session of the same id.
- **`list`** prints one row per session, oldest first: its handle, its first and last time, its git
  branches, its count of lines the owner typed, of compactions and of subagents, and its first
  message from the owner. `--branch` keeps the sessions that worked on a branch, `--containing` the
  sessions holding a text the owner typed, and `--since <YYYY-MM-DD>` those whose last line is on or
  after the date, in UTC.
- **`show <session>`** prints the session's lines in order, each with its line number and its label;
  `<session>` is a handle or a prefix of a `sessionId` that names one session. `--from` starts at a
  text the owner typed, `--subagents` adds each subagent's lines, `--tools` the tool calls and their
  results, and `--full` an injected or a tool text whole, which otherwise prints its first 5 lines
  beyond 40. A subagent's lines are numbered with its id, as `<id> L<n>`.
- **`search <regex>`** prints each line whose text matches, as the line of the text where the match
  starts, with `--role` keeping one role, `--tool <name>` one tool's calls and their results,
  `--tools` every tool line, and `--subagents` the subagents' lines too.

**Every line is labelled by its fields, never by its text.** The owner's words are the lines
labelled `owner`, `owner-queued` (typed while the session was busy), `owner-answer` (an answer to a
question tool) and `owner-command` (a command such as `/clear`); `injected` lines are what the
harness or another agent delivered, with their kind; `agent` is the session's own text, `dispatch`
the text a subagent received, `tool` a tool call or its result. A transcript with no field at all,
such as one of a session run with `claude -p`, is labelled by its text, and the header says so.

**The format is the harness's own, which it documents as internal and liable to change between
versions.** The command never fails on what it does not know: its header names, before the rows,
every line type and every value it does not recognise, with its count and its first file and line,
every line it could not label, and every line that is no JSON. Each is a gap of this command, to
report as a finding on the checker in the retrospective.

**The harness deletes transcripts after 30 days by default**: the setting `cleanupPeriodDays` of
Claude Code's own settings keeps them longer.

Every verb takes `--json`, which prints one JSON object per row or line instead, the header first. The output is for a
reader and is no contract, text and JSON alike.

## Registers

**Eleven registers are compiled in.** `design`, `goal`, `tripwire`, `rejected` and `issue` are
what the word component means to this tool. `rejected` holds the designs a Component did not keep,
per `design@core@rejected-register`, each cited by the design entries whose argument covers it, and an entry no design entry cites is a
finding, per `design@core@orphan-rejected-entry`. `spec` and `milestone` are the plan documents, and `thread`,
`argument`, `criterion` and `acceptance` their items, described under "Plan documents" below. A project declares further ones in its
`knowledge-architect.toml`. The first two tables are from thaum, whose `rules` location carries
an `interpretation` register:

```toml
[locations.rules]              # a directory carrying a subset of the registers
path = "docs/rules"
registers = ["issue", "tripwire", "interpretation"]

[registers.interpretation]
scope = "opt-in"               # `component` (every component carries it) or `opt-in` (those naming it)
shape = "file"                 # `file` (one file per entry), `heading` (slugs in a home) or `directory`
dir = "interpretations"        # the basename of the home; defaults to the register's name
sections = ["Rules", "Reading", "Consequences"]

[registers.interpretation.metadata.status]
values = ["settled", "interpretation", "ambiguous", "cr-gap"]

[registers.note]
scope = "opt-in"
shape = "heading"
dir = "decisions"
level = 2                      # a heading register only: the heading level its entries sit at
```

The next tables are a game project's: its design split by subject, one directory per subject under
game-design/, each entry cited `game-design@<subject>@<id>`:

```toml
[locations.gdd]
path = "game-design"
registers = ["subject"]

[registers.subject]
scope = "opt-in"
shape = "directory"            # one directory per entry, each an anchor of its own
dir = "."                      # the home is the carrier's own directory: game-design/<subject>/
carries = ["game-design"]      # what each entry carries, at the entry's own directory

[registers.game-design]
scope = "opt-in"
shape = "heading"
level = 3
```

A **heading register** keeps its entries as slugged headings, in `<dir>.md` or in `<dir>/` behind
a `README.md` that links every subdocument. It declares `level`, from 2 to 6: every heading at that
level in its home is an entry and carries a slug, per
`design@core@an-entry-is-a-heading-at-the-register-level`, and a slug at any other level defines
nothing, per `design@core@a-slug-elsewhere-defines-nothing`. The built-in levels are 3 for
`design` and `rejected`, and 2 for `goal` and `tripwire`. A heading is a line that opens with `#` marks, at most
three spaces in, followed by a space or a tab and text. A setext heading, a heading after a list or
block-quote marker on its line, and a heading with no text are reported in any markdown document,
per `design@core@headings-open-with-hash-marks`.

A **file register** takes no `level`. It keeps one file per entry under `<dir>/`, beside a
hand-written `README.md`, a generated `index.md` and an optional `register.toml` declaring the
group subdirectories. An entry opens with frontmatter carrying each declared metadata key, then a
level-one title, then the declared sections.

A **directory register** keeps one directory per entry under `<dir>/`, beside a hand-written
`README.md` and a generated `index.md`. Each entry directory holds a `README.md`, the entry's
document, which owes a level-one title, beside the files of what it carries, and holds no
subdirectory and no `register.toml`. Each entry is an anchor named by its basename, cited as a
whole `<register>@<parent>@<id>`, the parent being the anchor that carries the home, and it carries
the one register `carries` names, an `opt-in` heading or file register, with its home at the
entry's own directory: a carried heading register's README is the entry's README, which links
every file and defines no slug. A `register` or a `path` reference to an entry's directory or to
its README is a finding naming the entry's reference. An entry's name is refused when it is a
kind's, a Component's, a location's or a reserved word, or when a milestone, a spec or an earlier
entry holds it: the milestones first, then the specs, then the declared registers by name, each
register's entries by path. The argument is `design@core@declarable-directory-register`.

**`dir = "."` puts a register's home at its carrier's own directory**, for an `opt-in` register
only: the location carrying it carries no other register, since every other home would sit inside
that one, per `design@core@home-at-the-carrier-path`.

**`components = [...]` attaches an `opt-in` register to the Components it names**, the root's name
included, each keeping its home at `<component>/docs/<dir>/`, per
`design@core@a-component-carries-what-a-project-attaches`. The game design above could sit at
`<root>/docs/game-design/` with no location: `dir = "game-design"` and
`components = ["<root>"]`, the root being named by `[project] name`, in place of the location and
`dir = "."`, each subject then cited `subject@<root>@<subject>`. A name that is no Component is a finding, and so is a home at the
Component's `docs/` itself, at the root's plans directory, or at the directory of another register
the Component carries.

`[registers.issue]` accepts `kinds` and nothing else; the design, goal, tripwire and rejected
registers accept nothing, `level`, `carries` and `components` included, and a table for a plan register or an
item register is refused whole. The
arguments are `design@core@registers-are-declared` and
`design@core@a-file-register-is-a-directory-of-entries`.

## Plan documents

**Every project carries a plans directory, docs/plans/ at its root**, and the tool constructs an
anchor named `plans` there. No manifest row declares it, and a declared anchor named `plans` is
refused. It holds three things and nothing else:

| path | what it is |
| --- | --- |
| docs/plans/README.md | hand-written: what the directory holds |
| docs/plans/specs/ | the `spec` register: one file `<id>.md` per spec, beside a hand-written `README.md` and a generated `index.md`, as any file register |
| docs/plans/milestones/ | the `milestone` register: one directory `<id>/` per milestone, beside a hand-written `README.md` and a generated `index.md` |

**Each milestone directory is an anchor of its own**, named by its basename: `milestone` is the
built-in directory register, whose entries carry `spec` and the four item registers. Its `README.md` is the
milestone document, its `index.md` is generated and lists its slices, and every other `.md` file in
it is the spec of one slice. A directory under milestones/ with no `README.md` is a finding. **Each
spec file is an anchor too**, named by its id, and stays an entry of the `spec` register of
`plans`.

**A plan document holds fixed sections.** A spec of specs/ and a milestone's README owe, in order:
Status and audience, How the work is done, Names, What the work is, What is already decided,
Criteria, Threads, Arguments, "New names, in one place", Decided design, Mapping tables, Losing
alternatives, Readings, Premortem, Acceptance criteria, Implementation sequence, Order rationale,
Defaults awaiting the owner, Harvest, Later consequences. A slice spec owes Builds, Claims, Audit
subjects, Fails alone on, Premises that expire.

**A plan's items are headings of four of those sections.** A level-three heading ending with its
slug, `### <statement> ##<id>`, under Threads, Arguments, Criteria or Acceptance criteria, defines
an item of the kind `thread`, `argument`, `criterion` or `acceptance`, in the plan's anchor: the
spec, or the milestone, whose README and slice specs share one namespace. Every level-three heading
of those sections carries a slug; a slug anywhere else in a plan defines nothing. An item is cited
`<kind>@<plan>@<id>`, from inside its own plan only: a citation from outside it, a commit message
included, is refused, and the finding names the whole-document form.

**A plan document is cited by its kind, never by its path**:

```text
spec@plans@<id>               a spec, docs/plans/specs/<id>.md
milestone@plans@<id>          a milestone, docs/plans/milestones/<id>/
spec@<milestone>@<slice>      a slice spec, docs/plans/milestones/<milestone>/<slice>.md
```

A `path` citation of a plan document is refused, and the finding names the form above. The
plans directory's README, and the README and index of each of its two homes, are cited by path,
as in `path@plans@README.md`.

**A path a plan's work will create is cited `planned@<anchor>@<path>`**, from a document of the
plans directory only. Its anchor and path follow the rules of a `path` reference, and the target
must not exist: once it does, the finding asks for `path@<anchor>@<path>`, which the change that
creates the file writes. `*` and `elsewhere` are refused for it, and a register named `planned` is
refused like one named `path`.

**A register's home is cited `register@<anchor>@<register>`**, as `register@core@design` or
`register@*@issue`, and names the home in whichever shape the tree holds, so a home that outgrows
one file breaks no reference. The anchor is a component, a declared location or `plans`, or `*`
for a component-scoped register. A `path` reference to a home, a heading register's file, its
directory or that directory's `README.md`, a file register's directory, or a directory register's
directory, is a finding whose repair names this form; a `path` reference to an entry's directory or
its `README.md` names the entry's own reference. A register named `register` is refused like one named `path`.

**A plan's name reads as nothing else.** A milestone's name or a spec's id that is the name of a
component, of a location or of `plans`, `elsewhere` or `*`, and one name used under both homes,
are findings. So is a milestone name outside the id grammar, `[a-z0-9]+(-[a-z0-9]+)+`, per
`design@core@two-segment-ids`. An entry of
a declared directory register gives way to every plan of the same name.

A tree that breaks this layout stops the run at phase 2, except a home's missing `README.md` or
`index.md`, which is a finding of the last phase as in every file register. A misplaced or
duplicated item stops it at phase 3, and a missing section is a finding of the last phase. The arguments are
`design@core@plan-register`, `design@core@plans-split-dirs`, `design@core@plan-document-kinds`,
`design@core@a-placed-name-reads-as-nothing-else`, `design@core@spec-file-is-an-anchor`,
`design@core@plan-document-sections-by-kind`, `design@core@plan-items-by-section` and
`design@core@plan-item-scope`.

## Commit messages

**A commit message is a document under the regime.** It is parsed as one markdown document —
subject line, blank line, body — and every rule runs over it: every
`` `<kind>@<anchor>@<id>` `` reference resolves, and each registered extension judges it against
its commit's tree. The argument is
`design@core@a-commit-message-is-a-document`.

```sh
cargo klarch commits HEAD~1..HEAD        # the commit just made, where branch SHAs are not refused
cargo klarch commits origin/main..HEAD   # the branch's own commits
```

`commits` reads everything from each commit's own tree through git objects — the manifest, the
documents, the generated indexes, every file an extension reads — so a message is judged against the tree it
was written against, byte for byte and cleaned of nothing. **A commit whose tree does not load or
carries findings fails the run**, the range's last commit included, with its tree's findings
named by the commit and the file, and its message is still judged where its tree reached the last
phase. The summary block counts the commits, how many passed and how many failed, and a commit
fails on any finding of its tree, of its message or of a citation by SHA:

```
commits in origin/main..HEAD: 2 commits, 1 passed, 1 failed
  <sha> passed
  <sha> failed: its tree fails 1 finding(s); its message carries 2 finding(s)
```

The checker that judges every commit is the one built from the working tree, so a branch
that makes it stricter puts that change in its first commit or is squashed before review. A
message's references resolve against its commit's tree **or its first parent's**, which is what
lets a commit that closes an issue name it; a parent whose manifest fails phase 1 is not read, and
serves as no parent. No hook judges a message before the commit exists:
the range is run after each commit, and a finding in the newest commit is repaired by an amend.

**A project whose merges rewrite SHAs refuses citations of its branch's own commits.** Under
`[commits] refuse-branch-shas = true`, `commits` reports every run of 7 to 64 lowercase hex digits
that prefixes the SHA of a commit of the range, in a message or in a document of a commit's tree: a
rebase merge gives those commits new SHAs, and the citation would then name nothing. Name such a
commit by its subject. A SHA already on the main branch may be cited. The option is off when absent,
and is read from each commit's own manifest. A citation is seen only when the cited commit is in the
range judged, so with the option on, run the check after a commit over the branch, `<base>..HEAD`:
over `HEAD~1..HEAD`, a citation of an earlier commit of the branch passes, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`. The argument is
`design@core@branch-shas-are-refused`.

`check` reads no range of history, and the range is always explicit. `cargo x gates` runs
`commits origin/main..HEAD` as a gate.

**The range need not end at the checkout.** Any range `git rev-list` resolves is judged, commit by
commit, and where HEAD sits changes nothing. The uses a range serves:

| use | range |
| --- | --- |
| the commit just made, before it is amended or built on | `HEAD~1..HEAD`, or `<base>..HEAD` under `refuse-branch-shas` |
| a branch's own commits, before review and merge | `<base>..HEAD` |
| CI over a pull request, which checks out the branch's head | `<base>..HEAD` |
| a pre-push hook: git gives it, per pushed ref, the local and the remote sha, and with `git push origin other-branch` the local sha is not HEAD | `<remote sha>..<local sha>` |
| a branch that is not checked out, reviewed locally | `<base>..<branch>` |
| an audit of old history, such as the commits made before a manifest migration | `<first>^..<last>` |

`<base>` is the branch the work merges into, as the remote holds it.

Two things are still read from the working tree, whatever the range: the binary that judges, and
git's ignore rules, which `git check-ignore` answers for the files on disk only.

## What to respect

**Nothing about a project is compiled into the tool.** Every list a check reads comes from the
project's `knowledge-architect.toml`, which is both the manifest and the marker that makes a
directory a project root. So one binary checks any project and every mock project under
`path@core@tests/projects/` with no special case. A path that should not be checked says so
there, in one place, with a reason beside it. What is compiled in describes the tool, not a tree:
the directory of each Component a binary's libraries belong to, so that the string literals of
the tool's own source are read as data. The decision is
`design@core@nothing-of-a-project-is-compiled-in`.

Read `register@core@design` before changing how it works.
