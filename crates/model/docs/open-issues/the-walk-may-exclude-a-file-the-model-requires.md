---
kind: todo
---
# A row of `[walk]` may keep out a file the model requires, and only one such case is reported

## Summary

The manifest's `[walk]` rows, `skip-dirs`, `skip-files` and `exclude`, may name any path. A row
that keeps out a file the model requires is reported in one case only: under the `claude` harness,
a root CLAUDE.md outside the walk, by `shipped` in `path@model@src/shipped.rs`, with the
repair "remove the row of [walk] that keeps it out". The owner's direction: the walk does not allow
excluding a file the model requires.

## Details

### What

- The one case reported today, in phase 2: "the root CLAUDE.md is outside the walk, so its import
  of the installed primer cannot be checked", raised in `shipped` when the root CLAUDE.md is
  present and the walk does not read it.
- The rule the owner gave: a row of `[walk]` that keeps out a file the model requires is refused.
  Under the `claude` harness the root CLAUDE.md is one such file. Which other files the model
  requires, such as a Component's required documents and its register homes, which
  `design@model@components-carry-the-same-documents` lists and `tree` asserts, is not
  established: whether a row can keep one of them out today, and what is reported then.
- Where the refusal belongs: a declaration that is wrong is a phase-1 finding, absent from the
  configuration, per `design@model@a-wrong-declaration-is-a-finding`, so a refused row would leave
  the walk; whether the refusal can be decided in phase 1, from the manifest and the harness alone,
  or needs the listing, is not established.

`milestone@plans@component-split` keeps today's case in phase 2, in `model`'s foundation, and moves
every other finding of `shipped` to the last phase; this work replaces that case.

### Why it matters

A file a row keeps out is read by no check, and a model built without it lacks what that file
defines, which every finding computed from it then misses, against
`goal@check@document-rules-are-checked`. One case reported and the rest silent is a rule nobody can
state from the tree.

### What would close it

A design discussion under `skill@knowledge-architect-design` that lists the files the model
requires and decides where a row naming one is refused; the refusal built, with a test over a mock
project for each kind of file; the case of `shipped` folded into it. A stricter check, so a
Migration entry names what a project that keeps such a file out must change.
