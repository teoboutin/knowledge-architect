# The command line of knowledge-architect

The binary `klarch` and its library, the package `knowledge-architect`: the tools of knowledge-architect as one
program, those whose result needs the project's build or tests apart, parsed here, and the API a binary that registers an extension is written against. A project that runs them
names this package alone of those that make up the program. What the checks hold a project's documents to is
`path@check@README.md`, which also states what the tool needs of git, 2.36 or newer, and how `check` and `commits`
behave. A command's logic is the crate that holds it: `check`, `query`, `transcript` or `create`.

**`klarch`, and a binary per set of extensions.** `klarch` runs every command below with no extension
registered. A binary that registers extensions runs the same commands unchanged, and its extensions'
own commands beside them, per `design@cli@the-commands-are-a-library-module`. Every such binary hands
`init` to `cli::init` before it locates a project, since `init` is what creates one. Over a manifest
holding a table no extension of the binary claims, the binary reports that table in phase 1, rather
than skip in silence what the table configures, per
`design@check@an-extension-claims-its-manifest-tables`. The commands below are written
`cargo klarch <command>`, the command this repository declares: a cargo alias over the binary it runs,
built from the checkout. A project declares its own command with `[project] command`, and `klarch` is the
command when it declares none. The checker prints the declared command in its messages, in the header of
every generated index, and in the agent files it installs, per `design@check@declared-command`.

**The library is for a binary that registers an extension.** Its public API, and how to write and
test an extension with it, is described at <https://docs.rs/knowledge-architect>, for each
published version. The text is the crate documentation of `path@cli@src/lib.rs`.

## Who it serves

knowledge-architect serves one kind of project, a target project, which holds all of these, per
`goal@knowledge-architect@serves-one-kind-of-project`:

- the code it builds and ships is Rust, built by Cargo, apart from the glue its targets need;
- AI agents drive its development: its code, its documents, its entries and its reviews;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect.

It asks more of a target project: that it uses both the workflow and the checker, git with its `.git`
at or above the project, agents under the
`claude` harness, every gate run through the gates library from one command, and one merge invocation
of that command. Nothing is guaranteed outside these conditions, or where an ask is not followed.

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
                                       create a project, overwriting no hand-written file
cargo klarch new-component <DIR>       add a Component: its manifest line, its documents and homes
cargo klarch worklist --out <dir> [--seed <ref>]…
                                       the work list of a search of the whole record, one directory per agent
cargo klarch worklist --verify <dir>   check the reports the agents filled, and assemble the graded entries
cargo klarch transcript list [--branch b] [--containing t] [--since d] [--all-projects]
                                       the sessions of Claude Code's transcripts that ran in the project
cargo klarch transcript show <session> [--from t] [--subagents] [--tools] [--full] [--all-projects]
                                       one session's lines, each labelled by what the harness recorded
cargo klarch transcript search <regex> [--role r]… [--tool n] [--tools] [--branch b] [--since d]
                         [--subagents] [--all-projects]
                                       every line whose text matches
cargo klarch --version                 the version of the checker that runs
```

## Exit codes

Three, per `design@cli@exit-code-ladder`, and the third is what makes the other two mean anything.

| code | meaning | where it comes from |
| ---- | ------- | ------------------- |
| `0` | the command ran and its subject is in order | `--version`, from any directory; `check` with no findings; `show` on a reference that resolves; `id` with every argument an id nothing defines; `issues` and `tripwires` with at least one row; `index` having written every destination, or found each already current; `index --staged` having staged every generated file whose staged bytes differed, or found each already current; every `model` run; `worklist --out` having written the work list, notices included; `worklist --verify` on complete reports; `transcript` having printed at least one row or line; `anchors` with every home it prints present; `commits` with no finding, an empty range included; `install-agent-skills` having written and removed what it had to, or found nothing to do; `init` and `new-component` having written, with the final check passing, and `init` under `--vcs none` outside a worktree, where no check runs |
| `1` | the command ran and reports a negative answer | `check` with findings, `--fix` included; `check --fix` when a write failed, a destination was refused, a partial commit's mismatch was refused, or the final check could not run, after another file was already written; `show` on a reference that resolves to nothing; `id` with an argument some entity defines, or one that is no id; `issues` or `tripwires` with no row; `anchors` with a home missing; `index` when a write failed after another destination was already rewritten; `commits` with a finding against a judged message or a failing tree; `worklist --verify` with a finding on a report; `transcript list` or `search` finding no session or no line, `transcript show` on a session it cannot name, with a `--from` text no line the owner typed holds, or printing no line; `init` and `new-component` having written, when the final check finds anything, cannot run, or a write failed |
| `2` | the command could not run, `anchors` included | an unknown or invalid argument, `--fix` with `--staged` among them, a `show` argument that is not reference-shaped, a binary built from another checkout of the tool, which names the `cargo clean` that rebuilds it, a binary of another version than `[project] checker-version` pins, or a pin it does not confirm, or a manifest with no pin, `index` or `check --fix` refusing a destination — a symlink, or a directory that is not there — having written nothing, `check --fix` refusing a partial commit's mismatch having installed nothing, `check --staged` or `index --staged` over an index that holds an unmerged path or a staged tree that cannot be read, `index --staged` refused by its gate, a destination or git's index lock, having staged nothing, `commits` on a range that does not resolve, no project above the working directory, no `git` on the path or a project outside a worktree, `install-agent-skills` over a manifest holding a refused declaration, over a symbolic link on an owned path, or when a write or a removal failed, naming the path, a stdout closed before the output was written — as `\| head` does — which ends the run silently, an input that cannot be read, an input an extension prepares that it cannot resolve, `init` refusing, having written nothing: anything at the manifest's path, a refused name, a DIR `--component` names in a `.git` directory or in the installer's namespace, a destination that is not a regular file or lies under one, two destinations that overlap, an `index.md` already at a generated path, a non-empty directory outside every git repository with no `--vcs`, a PATH git ignores, in a `.git` directory or under a file, `GIT_INDEX_FILE` set, a git that cannot answer, a `Cargo.toml` at or above PATH or a file `--rust` writes already there, a `.gitignore` it cannot read; `new-component` refusing, having written nothing: an incomplete model, a DIR outside the project, in a `.git` directory or in the installer's namespace, or one the manifest refuses, a destination it cannot write; `init` dispatched inside a project by a binary that skips `cli::init`; `worklist --out` refusing, having written nothing: an incomplete model, a directory inside the project or not empty, a seed that names no entry; or a write that failed, naming the path, the files written before it left in place; `worklist --verify` over a file it cannot read or parse, or on passing reports whose snapshot is missing or lacks an entry, writing no assembly; `id` over an incomplete model; `transcript` with no transcript directory, over a file it cannot read, or with a regular expression or a `--since` date that does not parse |

**A caller scripting against a run reads the exit code; a person reads the last line.** Arguments
are refused before the project is located, so `--help` answers from anywhere and a mistyped
invocation costs no walk.

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
the manifest declares, per `design@query@tripwire-guards-no-rejected-entry`. For both, the
text keeps the rows whose id or title contains it, case-insensitively, and **no row is exit 1**.

**The first positional argument is an anchor when something declares that name**, and text
otherwise, which is what lets one positional list mean both. A search for a word that is also an
anchor name is written with the anchor before it.

**The last-change column is git's**, taken in one `git log` for every issue instance at once. An
entry with no commit shows `uncommitted`; where git cannot answer the column shows `-`.
A date never enters a generated file, per `design@check@generated-files-are-pure` — it is printed
here and nowhere else.

## `index`

Regenerates every generated file in place, from one walk: each file a registered extension
generates, and one `index.md` per file-register instance whose directory is there. It takes no
dry-run flag and **writes only where the bytes differ**, naming each file it rewrote:

```
$ cargo klarch index
docs/open-issues/index.md                rewritten
crates/check/docs/open-issues/index.md    already current
```

**A file-register index is a fixed shape, and its bytes are a contract**, per
`design@model@a-file-register-index-is-rows`:

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
The two halves are `design@check@index-takes-no-dry-run` and `design@check@one-question-one-command`.

**`index --staged` writes into git's index instead, for a commit of part of the working tree**,
per `design@check@index-staged-write`.
It generates each file from the tree git's index would commit, which is HEAD's tree with the
staged changes, and sets the staged entry of each one whose bytes differ. No working-tree file
changes: after it, `check --staged` passes on what will be committed, and the working tree keeps
the rows of the working tree for a later commit. Each generated file whose working copy is out of
date for the working tree is named, with `index`, which rewrites it.

```
$ cargo klarch index --staged
docs/plans/specs/index.md                already current
docs/open-issues/index.md                staged
docs/open-issues/index.md                the working copy is out of date for the working tree: `cargo klarch index` rewrites it
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
byte, and fails in its last phase on a missing, differing or unshipped file, so a project that moves
to a new version runs the install with the move. A reference to what a missing file defined is
reported beside it, and the install repairs both. The commands that write a generated file do not
wait for it. The decisions are `design@check@installed-files-committed` and `design@check@installed-files-compared-by-bytes`.

```toml
[agents]
harness = ["claude"]   # the default when the table is absent
# harness = []         # no agent configuration: no install, no check, no CLAUDE.md required
```

With `harness = []` no component owes a CLAUDE.md, and nothing is installed or checked, per
`design@check@agents-table`.

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
as the setup skill says. Like `cargo init`, per `design@create@init-follows-cargo-init`, it takes a PATH, `.` by default, created when missing,
names the project after its basename unless `--name` is given, and asks nothing:

```sh
klarch init my-project              # the manifest, the agent files, every document and home
klarch init . --component crates/check --component tools/xtask
klarch init my-project --rust       # also the maintenance crate, its workspace and aliases
```

- **It overwrites no hand-written file outside the installer's namespace**, per
  `design@create@init-keeps-existing-files`. An
  existing README.md, CLAUDE.md or register home is kept and listed as `kept`; the check then
  reports what it lacks, such as the primer's import. The agent files are the install's, and it
  writes them inside a worktree as `check --fix` repairs them, and outside one as
  `install-agent-skills` does. Every refusal comes before its first write, exit 2,
  each listed in the table of exit codes above, per `design@create@init-asks-no-confirmation`. Each
  file is created only if nothing is there, so one that appears during the run is kept too. A manifest above PATH does not refuse
  it: a project nested in another is legal, and the run notes that the outer project walks it
  unless its `[walk] exclude` names it. A run stopped after its first write, such as by a stdout
  closed early, which exits 2 as every command does, is completed by `check --fix`.
- **Git, as `cargo init` does, with one guard**, per `design@create@init-runs-git-init`. Inside a
  git worktree no repository is created unless `--vcs git` asks for one, which the outer
  project's check then reports as a nested repository until an ignore rule or `[walk] skip-dirs`
  names it; outside every worktree a new or empty directory gets one. A directory there that already holds
  files is refused unless `--vcs git` or `--vcs none` decides, since an accidental run would leave
  its files untracked beside the project's, where `git clean` deletes both. Git is asked about
  PATH before anything is written, so a git that cannot answer, such as one refusing a repository
  owned by another user, refuses the run, and so does `GIT_INDEX_FILE` set, through which git
  would stage the new project into an index no commit of it records.
- **It ends as `check --fix` does**: it installs the agent files, writes the indexes once the first
  three phases find nothing, and runs the check, whose verdict is its exit code. Under `--vcs none`
  outside a worktree it installs the agent files and stops before the indexes, and says so.
- **The documents hold no entry.** Each opens with a title and a line on what it holds, or that
  it holds no entry yet. No goal is written: the goals are stated with the owner. The root CLAUDE.md is its title and the
  primer's import line.
- **`--rust`** writes the blocks the setup skill shows: a virtual workspace, the maintenance crate
  `xtask/`, pinned to this version, and the cargo aliases, declares `cargo klarch` as the command,
  and makes `.gitignore` ignore `/target`, appending to an existing one unless git already ignores
  it there. It writes no CI workflow, per `design@create@init-writes-no-ci-workflow`. Its refusals
  and their reasons are `design@create@rust-scaffold`.

**`new-component <DIR>` adds a Component to an existing project**: it appends DIR to
`[project] components`, keeping every other byte of the manifest, comments included, per
`design@create@component-manifest-edit`, then writes
the Component's documents and register homes, and ends as `init` does. A manifest written with
`\r\n` line breaks keeps them. It runs behind the gate of
the first three phases, so over a tree they find incomplete it refuses, writing nothing. A
register declared `opt-in` is not guessed: name the Component in its `components`, and the check
reports the home. Where git's index holds staged changes over a staged tree the refusal of
`check --fix` compares, the run stops before the indexes of the new Component's registers, as that
refusal does over a partial commit, exit 1, and says so: `index` writes them.

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
rule for each shape of register is the design head `design@query@cluster-rooted-at-a-subject`. An entry no
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

It prints one line per agent with its load and its clusters, each with its own load, and a **notice** for each cluster or
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
bearing grade line, identical lines merged; then its show text, fenced, or for a goal its grade lines and its site alone. It
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
  after the date, in UTC. In the text `--containing` and `--from` take, a space matches any
  whitespace of the owner's line but a line break, a non-breaking space included.
- **`show <session>`** prints the session's lines in order, each with its line number and its label;
  `<session>` is a handle or a prefix of a `sessionId` that names one session. `--from` starts at a
  text the owner typed, `--subagents` adds each subagent's lines, `--tools` the tool calls and their
  results, and `--full` an injected or a tool text whole, which otherwise prints its first 5 lines
  beyond 40. A subagent's lines are numbered with its id, as `<id> L<n>`.
- **`search <regex>`** prints each line whose text matches, as the line of the text where the match
  starts, with `--role` keeping one role, `--tool <name>` one tool's calls and their results,
  `--tools` every tool line, and `--subagents` the subagents' lines too.

**Every line is labelled by what the harness recorded of it, never by a substring of its text.** The owner's words are the lines
labelled `owner`, `owner-queued` (typed while the session was busy), `owner-answer` (an answer to a
question tool) and `owner-command` (a command such as `/clear`); `owner-interrupt` is the owner's
interrupt of a turn and `owner-shell` a shell command the owner ran with the `!` prefix; `injected`
lines are what the harness or another agent delivered, with their kind, the command's output and a
fork's inherited context included; `agent` is the session's own text, `dispatch` the text a subagent
received, `tool` a tool call or its result. A message line no field places is labelled by its text
only where the harness writes that text whole, as an interrupt's, or writes a tag first, as a shell
command's; any other is `dispatch` in a subagent file and `unlabelled` elsewhere, the message lines
of a transcript with no field at all, such as one of a session run with `claude -p`, included.

**The format is the harness's own, which it documents as internal and liable to change between
versions.** The command never fails on what it does not know: its header names, before the rows,
every line type and every value it does not recognise, with its count and its first file and line,
every line it could not label, and every line that is no JSON. Each is a gap of this command, to
report as a finding on the checker in the retrospective.

**The harness deletes transcripts after 30 days by default**: the setting `cleanupPeriodDays` of
Claude Code's own settings keeps them longer.

Every verb takes `--json`, which prints one JSON object per row or line instead, the header first. The output is for a
reader and is no contract, text and JSON alike.

## What to respect

**An item is public only through the facade**: `path@cli@src/lib.rs`, the module `cli` and the re-exports of
the checker's library, per `design@knowledge-architect@api-facade`. Read `register@cli@design` before changing how the
command line works.
