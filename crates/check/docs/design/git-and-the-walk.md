# Check design: Git and the walk

## Git and the walk

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
  `snapshot_entries` in `path@git@src/git.rs`.
- **An unmerged index is refused**, exit 2 naming each path once: a conflicted merge or rebase has
  no tree to commit. `git write-tree` is not used, since it writes objects into the repository and
  fails there too.
- **The summary block says which tree was walked.** `--staged` prints `tree: staged` after the
  `walk:` line, or `tree: staged (nothing staged: the tree of HEAD)` where the project's part of
  the index equals HEAD, so a run made before staging is not read as a verdict on the edits.
  Plain `check` prints no `tree:` line, so a `walk:` count CI compares with a local one is of the
  working tree on both sides.

The design this decision did not keep: `rejected@check@ls-files-staged-tree`.

### `check --staged` applies `check`'s rules, as on a clean checkout of the staged tree `##staged-check-semantics`

The staged installed files are compared with the running binary's shipped set, per
`design@check@installed-files-compared-by-bytes`, and the files git both tracks and ignores are reported. The rival, `commits`'s rules over the
staged tree, lost because CI runs `check` on a checkout: a staged run that skips the installed
files passes a tree CI fails. Two reads stay the working tree's: the ignore rules a path reference
asks, as for `commits`, and the version pin, compared before any command per
`design@check@installed-binary-version-check`.

### `check` judges no given commit's tree; `commits` does `##check-judges-no-given-commit`

`commits <rev>~1..<rev>`, or `commits <rev>` for a root commit, judges a given commit's tree with
the tip checker, and a second judge with `check`'s rules would give one tree two verdicts, per
`design@check@one-question-one-command`.
