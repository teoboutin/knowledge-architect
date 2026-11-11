# Design — git

How the `git` crate wraps git, and why. Each level-three heading is one
decision with its standing argument.

## Asking git

### Every git invocation of the library goes through one module `##git-is-asked-through-one-module`

`path@git@src/git.rs`, the one module of the `git` crate, owns the listing, the
tracked-and-ignored listing, the one `check-ignore` batch, the last-change dates, the paths a commit
and the work tree differ by, and every read of history: `rev-list`, `rev-parse`, the commit a
revision names, a commit's file list, its blobs and its message. Every one of them
is an `Invocation`, run by `Invocation::run`, so the set of things the library asks git is
auditable in one read, and the environment `run` sets reaches every invocation. Why git is wrapped
by a crate of its own is `design@knowledge-architect@git-crate`. That the checker's library asks git
through this crate alone is the checker's decision,
`design@knowledge-architect@checker-asks-git-through-the-crate`.

### Every git invocation runs with the per-user ignore file pointed at the null device `##per-user-ignore-file-pinned`

**The per-user ignore file is pinned away.** `core.excludesFile` lives in a developer's home and
is no part of any project, so every invocation runs with it pointed at the null device: a line
there would otherwise take an untracked live document out of every check on one clone and not on
another. What still decides the walk beside the tree's own ignore files is git's per-clone exclude
file, which git offers no way to pin, and that is what
`tripwire@model@walked-count-differs-between-machines` is left guarding, until the check of
`issue@model@the-walk-does-not-report-what-the-per-clone-exclude-file-removes` reports the files it
alone removes from the walk.

### Git answers about the repository the project's directory lies in, whatever the caller's environment names `##repository-variables-dropped`

Every invocation removes from its environment the variables that point git at another
repository's files, `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, `GIT_OBJECT_DIRECTORY` and
`GIT_ALTERNATE_OBJECT_DIRECTORIES`, the members of `git rev-parse --local-env-vars` that select a
repository, its work tree or its object store. It keeps the list's other members, the
configuration among them, and `GIT_INDEX_FILE`, per `design@git@index-file-variable-kept`. So git
finds the repository from the project's directory, as `design@model@git-supplies-the-walk` reads it,
and a run judges the project's own tree, which `goal@documentation-stays-consistent` needs of a
verdict: with `GIT_DIR` naming another repository, the walk would be that repository's index.

Refusing to run with one of them set leaves the pre-commit hook of a linked worktree unable to
run the checker: git gives that hook a `GIT_DIR` naming the worktree's own git directory, measured
with git 2.53.0 by a pre-commit hook printing `env | grep '^GIT_'`, which the search from the
project's directory finds again. Honouring a repository
variable while refusing one whose git directory differs from the one the project's directory finds
gives this head's result wherever it runs, and refuses the runs where this head reads the project's
own repository.

Honouring the variables where the search from the project's directory finds no repository loses on
two grounds. It lets a stray `GIT_DIR` in a shell outside every repository point the walk at
another repository. And the one project it keeps checkable is a project whose repository git finds
only through these variables, with no `.git` at or above its directory.
`goal@serves-one-kind-of-project` asks that a target project be a git
repository, which this head reads as a `.git` at or above the project's directory, where git's own
search finds it. Such a project is outside a worktree to the checker here, an error per
`design@git@failed-git-is-an-error`, and so is a pre-commit hook's run in it.

### `GIT_INDEX_FILE` is the index every command reads, so a pre-commit hook judges the tree its commit records `##index-file-variable-kept`

Every git invocation keeps `GIT_INDEX_FILE`: it is the index `git commit` records, so
`check --staged`, which judges the tree the next commit records per
`design@check@staged-tree-source`, reads the index it names, and every other command uses the same
index, so a `check` run where the variable is set lists what `git ls-files` lists there. A
pre-commit hook gets it, measured with git 2.53.0 by a hook printing `env | grep '^GIT_'`:
`.git/index`, relative, for a commit of what is staged; the absolute path of `.git/index.lock` for
`commit -a`, and of a `.git/next-index-<pid>.lock` for `commit <paths>`, each holding the tree the
commit records while `.git/index` still holds what was staged before; in a linked worktree, the
absolute path of that worktree's index or its lock, beside a `GIT_DIR` naming the worktree's git
directory, which `design@git@repository-variables-dropped` removes and the search from the
project's directory finds again. A relative value resolves against the top of the work tree, not
git's working directory, measured with `GIT_INDEX_FILE=.git/index git ls-files` from a
subdirectory, so a project in a subdirectory of its repository reads the same index. A plain
commit that concludes a resolved merge gets `.git/index`, as a commit of what is staged does. And
`index --staged`, run from the hook of `commit -a`, stages into the lock file the variable names,
and the commit records what it staged, measured with git 2.53.0 by a hook running
`index --staged` then `check --staged` over an entry staged without its index. Git 2.36, the floor
of `design@git@git-floor-for-commit-trees`, was not measured.

Dropping the variable with the others, `rejected@git@every-git-variable-dropped`, lost to that
measurement. Refusing to run with it set leaves a pre-commit hook unable to run the checker at all,
since git gives the hook the variable every time, and that hook is where git runs a command on the
tree `design@check@staged-tree-source` has `check --staged` judge, before the commit exists. That a
project can run the checker from that hook is argued by no head, which
`issue@check@no-head-argues-that-the-checker-runs-from-a-pre-commit-hook` holds.
`init` alone refuses it: `init` belongs to no commit, and with the variable set git would
list and stage the project it writes through an index no commit of that project records.

### A listed path is read as git's bytes, never decoded `##a-path-is-bytes`

**A path is bytes, not text.** The `-z` output is split on NUL and turned into paths byte for
byte. A name holding a byte no UTF-8 decoding accepts is legal here, and decoding it lossily
produces a path nothing on disk answers to — so the file leaves every check, and where nothing
reports the failed read the run stays green. That is the failure
`design@model@git-supplies-the-walk` exists against.

### Reading a commit's tree needs git 2.36 `##git-floor-for-commit-trees`

**Reading a commit's tree needs git 2.36.** `cat-file --batch -z` takes its requests
NUL-terminated, and a tracked filename may hold a newline: under the newline-terminated input
such a name is two requests, git answers both, and every later answer is paired with the wrong
path while the map still comes back full. An older git refuses the flag loudly, which is an error
with git's own reason.

### A git invocation a verdict depends on that fails is an error carrying git's reason, never an empty answer `##failed-git-is-an-error`

**No `git`, or no worktree, is an error carrying the reason.** Never an empty walk: a project
reported as holding no document is a run that checked nothing and said so as a clean verdict,
against `goal@documentation-stays-consistent`: a tree that passes the checker conforms, in the
texts the checker reads, to every rule it holds. **Every invocation a verdict depends on fails that
way**: the listing, the tracked-and-ignored listing and the `check-ignore` batch, each carrying
git's own stderr, whatever the reason. The checker's commands give that error the code 2, a
command that could not run, per `design@cli@exit-code-ladder`. The two invocations that degrade
instead are `last_changed`, which fills a listing's convenience column and moves no verdict: where
git answers nothing the column shows `-`; and `main_worktree_path`, which adds a root to what `transcript` lists.

## What the library offers

### An operation stated in git's own terms is a function of the library, however many git commands it takes, and a step that decides for a tool's matter is the tool's, however many tools share it `##an-operation-in-git-s-terms-is-the-library-s`

The library models operations on a repository, each stated in git's own terms: the paths, objects,
commits, refs and index it reads or writes. Such an operation is one function of the library
however many git commands it takes, as `snapshot_entries` reads the tree git's index would commit
through several git commands; rewording a commit's message, or folding a fixup commit into the commit it
fixes, would be one too. A step that decides for the matter a tool works on, such as which
commits to fold or what a check of the result must find, is the tool's, composed from those
operations, however many tools share it: the library holds no function that chooses which commits
a tool folds, and none that reads a project's documents.

The crate's interface changes for reasons of its own, a git question that another tool needs and the
checker does not among them, per `design@knowledge-architect@git-crate`; so what admits a function
is what it says of a repository, never which tool calls it. The nearest rival, an operation that
enters the library once a second tool needs it, loses to this: under a count of tools the same
operation changes its home when a tool is added, with no change to the operation, so the design and
the code move after the fact.
