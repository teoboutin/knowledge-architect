---
kind: todo
---
# A run of the tests of `testing` leaves directories in the temporary directory

## Summary

Some test helpers create a directory under the temporary directory and do not remove it, so the directory grows with each run.

## Details

### What

Measured by an adversarial reviewer, with `TMPDIR` set to a directory of its own: one `cargo test -p knowledge-architect`, when the tests of the binary sat in that package, left 90
directories, 1.8M in all, 78 named `klarch-*` and 12 `knowledge-*`. `Sandbox` of `path@testing@tests/binary.rs` removes its own directory
when dropped, so these come from other helpers, among them those named `klarch-transcript-*` and `knowledge-worklist-*-out`. Not
re-measured by the session that opened this entry. The tests this entry's slice added write under the temporary directory too, in
`with_a_work_list`.

Measured again by two reviewers, with `TMPDIR` set to a directory of their own: one `cargo test -p knowledge-architect-testing` leaves 96 top-level entries, 884 inodes, named `klarch-transcript-*`, `knowledge-*` and `refused-*`. On the machine this was measured on, `/tmp` is a tmpfs of 1,048,576 inodes, and held 68,118 `klarch-transcript-*` and 9,350 `knowledge-worklist-*` entries; at 1,048,465 inodes in use, builds failed with `No space left on device`, and a mutation run read such a failure as `caught`.

### Why it matters

The review that found it ran parallel builds on a `/tmp` with a quota, and some runs failed on disk quota and were repeated. A
machine that runs the suite often fills its temporary directory with no failure that names the cause.

### What would close it

Each helper removes what it creates when dropped, and a run with its own `TMPDIR` leaves none.
