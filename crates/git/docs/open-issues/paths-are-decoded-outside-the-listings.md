---
kind: defect
---
# Three public functions of the git library decode a path, so a path that is not plain UTF-8 or ASCII loses its answer

## Summary

The listings carry a path as git's bytes, per `design@git@a-path-is-bytes`, but `object_name`,
`ignore_query` and `last_changed` in `path@git@src/git.rs` decode one, and the path they answer
for no longer matches the path the listing gave.

## Details

### What

Reproduced by the adversarial reviewer of the work of `spec@component-split@git-out`, with git
2.53.0, through a probe crate depending on the public API alone:

- `object_name` builds the request with `rel.display()`. For a tracked `re\xffad.md`, which
  `entries` lists byte for byte, it gives `":./re\u{fffd}ad.md"`, and `rev_parse` on it answers
  `None`, while `blob_bytes` on the same path answers its bytes. The checker's
  `Snapshot::object_id`, in `path@core@src/extension.rs`, is built this way, so it answers "no
  object id" for a path the snapshot holds.
- `ignore_query` builds the query with `to_string_lossy`, so the same name is asked as
  `re\u{fffd}ad.md`.
- `last_changed` reads `git log --name-only` without `-z`, so git quotes every name holding a byte
  outside ASCII: over `docs` holding `café.md` and `plain.md` it answers the key
  `"docs/caf\303\251.md"`, quotes included, so a lookup of the path docs/café.md finds no date though git
  has one.

### Why it matters

A path the listing gives is answered for under another name, with no error:
`design@git@a-path-is-bytes` holds of the listings only, and `goal@git@a-pure-wrapper-over-git`
asks the library to let a tool read what a repository holds.

### What would close it

Each of the three builds or reads the path as bytes: `object_name` and `ignore_query` from the
path's bytes, and `last_changed` with `-z`; a unit test per function over a name holding a
non-ASCII byte, each shown to fail against its current body.
