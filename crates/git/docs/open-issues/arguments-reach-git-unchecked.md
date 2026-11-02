---
kind: observation
---
# A path or a revision a caller hands the git library can reach git as an option, and a directory reads as a blob

## Summary

Some functions of `path@git@src/git.rs` pass a caller's path or revision to git with no `--`
before it, and `blob_bytes` answers a directory with the tree object's bytes.

## Details

### What

Reproduced by the adversarial reviewer of the work that created the git Component, with git
2.53.0:

- `ignores(root, "--stdin")` answers `Ok(false)`: git reads its empty standard input and exits 1,
  whatever the ignore rules say of a path of that name;
- `commit_message(root, "--reverse")` answers `Ok` with HEAD's message;
- `rev_list(root, "--max-count=0")` is refused with git's usage, an `Err`, which is no defect;
- `blob_bytes(root, Source::Commit("HEAD"), ["d"])`, `d` a directory of the commit, answers an
  entry keyed `d` holding the tree object's raw bytes; whether `blobs` drops it as text that is not
  UTF-8 was not checked.

The checker hands these functions only paths from its own listings and revisions from its own
range, so it meets none of these cases. Not established: whether any is a defect rather than an
input no caller should make; the functions state no contract on such input.

### Why it matters

`goal@git@a-pure-wrapper-over-git` admits callers other than the checker, which a malformed or
hostile path or revision reaches; an answer that reads as a valid one, as the two `Ok` answers do,
gives such a caller no sign of it.

### What would close it

A ruling on the contract of each function for such input, written in its doc comment: an argument
separated by `--` where git takes one, a directory refused or skipped by `blob_bytes`; each with a
unit test.
