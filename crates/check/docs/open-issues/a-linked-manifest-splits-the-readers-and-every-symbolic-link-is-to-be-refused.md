---
kind: defect
---
# A manifest held as a symbolic link passes `check` and is refused by `check --staged` and `commits`; the direction is to refuse every symbolic link, early and with no row to silence it

## Summary

When knowledge-architect.toml at a project's root is a symbolic link to a manifest file, the
readers disagree. `check` reads the working tree through the filesystem, follows the link and
passes. `check --staged` and `commits` read a snapshot and refuse the tree, telling the project to
replace the link with the manifest file. The staged comparison of `check --fix` finds no manifest
file in the index and skips itself in silence.

The owner's direction for the fix: symbolic links are refused project-wide, early, before or at the
walk, under every condition, with no `skip-files` or `exclude` row to silence one. This is work for
a later branch, and it reverses part of `design@model@links-are-no-documents`.

## Details

### What

Three readers of one tree:

- **`check`** builds its `Manifest` with `Manifest::load` in `path@model@src/manifest.rs`, which
  reads the file at the root's path with `std::fs::read_to_string`, so the filesystem follows the
  link, and passes when the target is a valid manifest.
- **`check --staged` and `commits`**: `read_tree` in `path@check@src/commands/history.rs` looks the
  manifest up among the entries of kind `File`, and the snapshot lists the link as a `Symlink`. Since
  "[check] A manifest a snapshot holds as a link names its replacement by the file, not its
  staging", the refusal says "its tree holds knowledge-architect.toml as a symbolic link" and names
  replacing the link with the manifest file itself, staged in the index, or, for a commit, a history
  edit. `check --staged` exits 2.
- **`check --fix`**, its comparison of the staged tree's generated files: `staged_generated` in the
  same file returns `Ok(None)` when no listed entry is a `File` at the manifest's path, the branch
  meant for a staged tree that holds no manifest. A link is such a tree for that test, so the
  comparison is skipped and `check --fix` writes the working tree's generated files with no check
  that they match the staged tree. **Not established**: read from the code; no run reproduced it.

Reproduced through the binary for the first two readers, over a copy of the `dirhome` mock whose
manifest is a committed link to conf/knowledge-architect.toml, with
`skip-files = ["knowledge-architect.toml"]` so the walk keeps the link: `check` exits 0;
`check --staged` exits 2 with the refusal above. This is the reproduction the commit named above
records. That a `[walk]` row can keep the manifest itself out of the walk is
`issue@model@the-walk-may-exclude-a-file-the-model-requires`.

Ruled out: the refusal's text is not the defect. It names the one repair that makes the snapshot
readers accept the tree, per `design@check@finding-names-the-repair`. The defect is that the
readers disagree, and that the third skips its guard.

**The owner's direction**, given while this entry was written: every symbolic link in the project
is refused, before or at the walk, and no `skip-files` or `exclude` row silences one. The owner's
grounds: a project has little reason to hold one, the owner's projects must build on Windows in
every case, and the owner accepts that this becomes something every adopting project is asked to
do.

- **What it reverses**: the half of `design@model@links-are-no-documents` that keeps a symlink as a
  declared silence, a `skip-files` row. That reversal goes through `skill@knowledge-architect-design`
  when the work is done. A gitlink is outside the direction: its `exclude` row stays.
- **The cost to weigh there**: a project that keeps AGENTS.md or CLAUDE.md as a link to the other
  must replace it with a file to adopt the checker, against `goal@adoption-is-easy`.
- **What it fixes**: the disagreement above between `check`, `check --staged` and `commits`; the
  skipped comparison of `check --fix`; and the refusal of `commits` over a path under a link the
  tip's working tree holds, the symbolic-link cause of
  `issue@model@a-path-git-reads-as-a-pathspec-stops-the-run-or-misreads`, whose gitlink cause stays
  there.
- **What leaves with it**: repair texts this branch wrote become unreachable: the symbolic-link case
  of `read_tree`'s refusal of a manifest held as a link, its gitlink case staying, and the finding on
  a symlink at a generated path in `declarations` of `path@model@src/tree.rs`.
- The owner's reason that the owner's projects build on Windows may be a purpose of the project: it
  is drafted under `skill@knowledge-architect-goal-setting` when the work starts, if it is one.

### Why it matters

`design@model@links-are-no-documents` argues that `check` and `commits` agree by construction on a
link, since the listing classifies an entry by its mode before either reader touches a byte. Under
`check` the manifest is read by `Manifest::load` before the walk, which needs it, so that argument
does not reach it. A project can pass `check` on every commit and have `commits` refuse the whole
branch, against `goal@documentation-stays-consistent`. The skipped comparison of `check --fix` is
the silence `staged_generated`'s doc comment names as the failure it guards: a partial commit
recorded with the generated files of the wrong tree.

### What would close it

On a later branch: the reversal of the declared-silence half of
`design@model@links-are-no-documents` under `skill@knowledge-architect-design`, weighing the cost
above; the refusal of every symbolic link built before or at the walk, the manifest's included,
under `check`, `check --staged`, `check --fix` and `commits` alike; a test over a copy of `dirhome`
with a linked manifest, and one with a link a `skip-files` row names, asserting that every reader
refuses it with one repair; the unreachable repair texts removed; and, being a stricter check, a
Migration entry naming what a project holding a link must change.
