---
kind: defect
---
# Public functions of the git library decode a path, so a path that is not plain UTF-8 or ASCII loses its answer

## Summary

On Unix the listings carry a path as git's bytes, per `design@git@a-path-is-bytes`, but `object_name`,
`ignore_query` and `last_changed` in `path@git@src/git.rs` decode one, and the path they answer
for no longer matches the path the listing gave. On the other systems the listings decode one too,
and a listed name that is not UTF-8 is lost.

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

### Why it matters

A path the listing gives is answered for under another name, with no error:
`design@git@a-path-is-bytes` holds of the listings on Unix only, and every question the checker's tools
ask git goes through these functions, per
`design@knowledge-architect@checker-asks-git-through-the-crate`. Outside Unix, the bytes of a
document under such a name reach no check, against `goal@documentation-stays-consistent`.

### What would close it

Each of the three builds or reads the path as bytes: `object_name` and `ignore_query` from the
path's bytes, and `last_changed` with `-z`; a unit test per function over a name holding a
non-ASCII byte, each shown to fail against its current body. Outside Unix, a listed name that is not
UTF-8 is an error, or reaches the caller as a name that git answers for.
