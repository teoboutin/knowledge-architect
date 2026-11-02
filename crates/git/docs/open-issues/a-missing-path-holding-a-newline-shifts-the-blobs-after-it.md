---
kind: defect
---
# A requested path that the snapshot does not hold and whose name holds a newline shifts every blob after it to the wrong path

## Summary

`blob_bytes`, and `blobs` through it, in `path@git@src/git.rs`, answer a later path with another
path's bytes, and drop one, when a requested path holds a newline and the snapshot does not hold
it.

## Details

### What

Reproduced by the adversarial reviewer of the work that created the git Component, with git
2.53.0: a commit holding `x.md` with the bytes `XX` and `y.md` with `YY`, asked
`blob_bytes(root, Source::Commit("HEAD"), ["a\nb.md", "x.md", "y.md"])`, answers one entry,
`("y.md", "XX")`: `x.md` is lost and `y.md` carries `x.md`'s bytes.

Mechanism, read from `read_batch`: `cat-file --batch -z` takes its requests NUL-terminated, but
answers a miss with the line `<request> missing`, the request's own newline included. `read_batch`
cuts each answer's header at the first newline, so the second half of the missing request's name
is read as the next path's header, and every later answer is paired with the path after its own.
The test `a_newline_in_a_tracked_name_does_not_shift_the_answers_after_it` covers a newline in a
path the snapshot holds, whose answer header carries the object id rather than the request.

Ruled out: the checker as it stands. It asks only for paths its listing of the same snapshot
returned, so a request it makes is held, unless the snapshot changes between the listing and the
read, which `blobs` names as a race it tolerates.

### Why it matters

`design@git@git-floor-for-commit-trees` exists against exactly this, an answer paired with the
wrong path while the map comes back full; and the function's own contract says a path the
snapshot does not hold contributes no entry. A caller that asks for a path it did not list, as
`goal@git@a-pure-wrapper-over-git` admits, reads wrong bytes with no error.

### What would close it

`read_batch` reads a miss by the request it knows it sent, its length included, rather than by
the next newline; a unit test in `path@git@src/git.rs` asks for a missing path holding a newline
before two held paths and asserts each held path's own bytes, shown to fail against the current
`read_batch`.
