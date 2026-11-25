---
kind: defect
---
# Public functions of the git library decode a path, so a path that is not plain UTF-8 or ASCII, or that holds a backslash on Windows, loses its answer

## Summary

On Unix the listings carry a path as git's bytes, per `design@git@a-path-is-bytes`, but `object_name`,
`ignore_query` and `last_changed` in `path@git@src/git.rs` decode one, and the path they answer
for no longer matches the path the listing gave. On the other systems the listings decode one too,
and a listed name that is not UTF-8, or that holds `\` on Windows, is lost.

## Details

### What

`main_worktree_path` decodes too, with `String::from_utf8_lossy` over `--show-toplevel` and `--git-common-dir`, so `transcript`'s filter gets no main-tree root, or a wrong one, for a repository under such a path.

Reproduced by the adversarial reviewer of the work that created the git Component, with git
2.53.0, through a probe crate depending on the public API alone:

- `object_name` builds the request with `rel.display()`. For a tracked `re\xffad.md`, which
  `entries` lists byte for byte, it gives `":./re\u{fffd}ad.md"`, and `rev_parse` on it answers
  `None`, while `blob_bytes` on the same path answers its bytes. The checker's
  `Snapshot::object_id`, in `path@check@src/extension.rs`, is built this way, so it answers "no
  object id" for a path the snapshot holds.
- `ignore_query` builds the query with `to_string_lossy`, so the same name is asked as
  `re\u{fffd}ad.md`.
- `last_changed` reads `git log --name-only` without `-z`, so git quotes every name holding a byte
  outside ASCII: over `docs` holding `café.md` and `plain.md` it answers the key
  `"docs/caf\303\251.md"`, quotes included, so a lookup of the path docs/café.md finds no date though git
  has one.

Outside Unix, `as_path` decodes each name of a listing with `String::from_utf8_lossy`. An index can
hold a name that is not UTF-8, where it was committed on Linux: such a name is listed with a
replacement character, `path_bytes` names it back to git so, and `blob_bytes` answers no bytes for it.
Read in the code, not run.

On Windows a listed name that holds `\` as a character, such as `notes\b.md` committed on Linux, is
named back to git by `path_bytes` as `notes/b.md`, another file's name, so the bytes of that other file,
or none, are answered for it. The checker's walk refuses the name before it asks for the bytes, and
reports it where a document stands at `notes/b.md`, as the test
`a_refused_name_is_refused_where_a_document_stands_at_the_same_path` of `path@model@src/survey.rs` holds,
but for a name a `skip-files` row keeps; there, a row for `notes/b.md` keeps `notes\b.md` too, since a path
is compared by its segments. Read in the code, not run.

### Why it matters

A path the listing gives is answered for under another name, with no error:
the code departs from `design@git@a-path-is-bytes` in the listings outside Unix, and every question the checker's tools
ask git goes through these functions, per
`design@knowledge-architect@checker-asks-git-through-the-crate`. Outside Unix, the bytes of a
document under such a name reach no check, against `goal@documentation-stays-consistent`.

### What would close it

Each of the three builds or reads the path as bytes: `object_name` and `ignore_query` from the
path's bytes, and `last_changed` with `-z`; a unit test per function over a name holding a
non-ASCII byte, each shown to fail against its current body. Outside Unix, a listed name that is not
UTF-8, or that holds `\` on Windows, is an error, or reaches the caller as a name that git answers for.
