# Core design: Creating a project and a Component

## Creating a project and a Component

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
