# Core design: Git and the walk

## Git and the walk

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

### Git answers about the repository the project's directory lies in, whatever the caller's environment names `##repository-variables-dropped`

Every invocation removes from its environment the variables that point git at another
repository's files, `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, `GIT_OBJECT_DIRECTORY` and
`GIT_ALTERNATE_OBJECT_DIRECTORIES`, the members of `git rev-parse --local-env-vars` that select a
repository, its work tree or its object store. It keeps the list's other members, the
configuration among them, and `GIT_INDEX_FILE`, per `design@core@index-file-variable-kept`. So git
finds the repository from the project's directory, as `design@core@git-supplies-the-walk` reads it,
and a run judges the project's own tree, which `goal@core@documents-stay-current` needs of a
verdict: with `GIT_DIR` naming another repository, the walk would be that repository's index.

Refusing to run with one of them set leaves the pre-commit hook of a linked worktree unable to
run the checker: git gives that hook a `GIT_DIR` naming the worktree's own git directory, measured
with git 2.53.0 by a pre-commit hook printing `env | grep '^GIT_'`, which the search from the
project's directory finds again. Honouring a repository
variable while refusing one whose git directory differs from the one the project's directory finds
gives this head's result wherever it runs, and refuses the runs where this head reads the project's
own repository. A project whose repository git finds only through these variables, with no `.git`
at or above its directory, is outside a worktree to the checker, exit 2 per
`design@core@failed-git-is-exit-two`.

### `GIT_INDEX_FILE` is the index every command reads, so a pre-commit hook judges the tree its commit records `##index-file-variable-kept`

Every git invocation keeps `GIT_INDEX_FILE`: it is the index `git commit` records, so
`check --staged`, which judges the tree the next commit records per
`design@core@staged-tree-source`, reads the index it names, and every other command uses the same
index, so a `check` run where the variable is set lists what `git ls-files` lists there. A
pre-commit hook gets it, measured with git 2.53.0 by a hook printing `env | grep '^GIT_'`:
`.git/index`, relative, for a commit of what is staged; the absolute path of `.git/index.lock` for
`commit -a`, and of a `.git/next-index-<pid>.lock` for `commit <paths>`, each holding the tree the
commit records while `.git/index` still holds what was staged before; in a linked worktree, the
absolute path of that worktree's index or its lock, beside a `GIT_DIR` naming the worktree's git
directory, which `design@core@repository-variables-dropped` removes and the search from the
project's directory finds again. A relative value resolves against the top of the work tree, not
git's working directory, measured with `GIT_INDEX_FILE=.git/index git ls-files` from a
subdirectory, so a project in a subdirectory of its repository reads the same index. Git 2.36, the
floor of `design@core@git-floor-for-commit-trees`, was not measured.

Dropping the variable with the others, `rejected@core@every-git-variable-dropped`, lost to that
measurement. Refusing to run with it set leaves a pre-commit hook unable to run the checker at all,
since git gives the hook the variable every time, against the owner's intent that a project can
run the check from its pre-commit hook. `init` alone refuses it: `init` belongs to no commit, and with the variable set git would
list and stage the project it writes through an index no commit of that project records.

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
`goal@core@document-rules-are-checked`: a tree that passes the checker conforms, in the texts the
checker reads, to every rule it holds. **Every invocation a verdict depends on fails that way** —
the listing, the tracked-and-ignored listing and the `check-ignore` batch — carrying git's own
stderr, whatever the reason. The one invocation that degrades instead is `last_changed`, which fills
a listing's convenience column and moves no verdict: where git answers nothing the column shows `-`.

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
