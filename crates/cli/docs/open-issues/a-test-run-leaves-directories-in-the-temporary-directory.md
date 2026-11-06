---
kind: todo
---
# A run of the tests of `cli` leaves directories in the temporary directory

## Summary

Some test helpers create a directory under the temporary directory and do not remove it, so the directory grows with each run.

## Details

### What

Measured by an adversarial reviewer, with `TMPDIR` set to a directory of its own: one `cargo test -p knowledge-architect` left 90
directories, 1.8M in all, 78 named `klarch-*` and 12 `knowledge-*`. `Sandbox` of `path@cli@tests/binary.rs` removes its own directory
when dropped, so these come from other helpers, among them those named `klarch-transcript-*` and `knowledge-worklist-*-out`. Not
re-measured by the session that opened this entry. The tests this entry's slice added write under the temporary directory too, in
`with_a_work_list`.

### Why it matters

The review that found it ran parallel builds on a `/tmp` with a quota, and some runs failed on disk quota and were repeated. A
machine that runs the suite often fills its temporary directory with no failure that names the cause.

### What would close it

Each helper removes what it creates when dropped, and a run with its own `TMPDIR` leaves none.
