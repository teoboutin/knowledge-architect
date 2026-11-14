---
kind: design
---
# After `index --staged` and the commit, the working tree's generated file is stale

## Summary

`index --staged` writes the generated files the staged tree needs into git's index and leaves the
working-tree files as they are. After the commit, a working-tree generated file still holds the old
rows, and the next plain `check` fails until `index` runs. Two sessions met it; each recovered with
one run. Every fix reverses part of `design@check@index-staged-write`.

## Details

### What

- `design@check@index-staged-write` "sets the staged entry of each whose staged bytes differ, or
  that the index lacks, leaving the working-tree file as it is", and "plain `check` judges the
  working tree as before".
- The primer, `primer@checker-model`, names the two repairs of a refused `check --fix`:
  "`cargo klarch index --staged` then `cargo klarch check --staged`, to commit the staged part, or
  `cargo klarch index`, to take the working tree whole, staged after it." Neither leaves the
  working tree current after a partial commit.

Instances:

- thaum, on 0.9.0, 2026-11-11, C4 of the workflow file of its split of oversized modules: a session
  retitled an entry, ran `index --staged`, committed, and left the working tree's index holding the
  old title; a reviewer found the stale file;
- this repository, 2026-11-12, C3 of the workflow file of the slice on the dispatch judgement: a
  spec deleted with `git rm`; `index --staged` and `check --staged` passed and the commit was made;
  the working tree's copy of `path@plans@specs/index.md` still held the deleted spec's row, so
  `cargo klarch check` failed with "the generated file is out of date" until a plain
  `cargo klarch index` ran.

The suspected mechanism, in one sentence: the head leaves the working-tree file as it was, so after
the commit it holds the rows of the tree before the staged change.

The shapes:

1. `index --staged` prints each working-tree generated file that now differs from the staged one,
   with "run `cargo klarch index`", thaum's proposal; `design@check@one-question-one-command`
   defeats it: whether a generated file is current is `check`'s question, and `check` already
   answers it, with the repair `index`;
2. `index --staged` also writes the working tree's copy where that copy equals HEAD's, the dispatch
   judgement's proposal; it brings back `rejected@check@index-staged-writes-working-file`, whose
   reason it answers only in part: where an unstaged change undoes the staged one for that
   register, the copy equal to HEAD's is current for the working tree, and the staged rows written
   there make plain `check` fail, the case the reason names;
3. `index --staged` also writes each working-tree generated file from the working tree, as plain
   `index` does, beside the staged entry from the staged tree; no recorded reason covers it, since a
   file generated from the working tree passes plain `check` by construction, but it reverses the
   head's title, "and no working-tree file";
4. no change: the next `check` names `index`, at the cost of one run, per
   `design@check@finding-names-the-repair`.

Any change keeps `design@check@fix-refusal-mixed-state`, both repairs the refusal names staying
correct, and takes the generated files from the one list of
`design@model@one-list-of-generated-files`.

Re-entry point: a design discussion under `skill@knowledge-architect-design` when a change next
touches `index --staged`, or earlier on the owner's word.

### Why it matters

It strains `design@check@index-staged-write`: the repair the primer names for a partial commit
leaves the working tree failing the next `check`, a stale generated file that a reviewer, not the
session, found in one instance, against `goal@documentation-stays-consistent`.

### What would close it

The discussion picks a shape, rewriting `design@check@index-staged-write` for shapes 2 or 3, built
under `skill@klarch-development` with a test over a mock project in which a staged deletion leaves
the working-tree index current after the commit; or it rules that the next `check` is the repair,
and this entry is deleted with that reason.
