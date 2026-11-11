---
kind: observation
---
# A test of the gates binary failed once in a full gates run, and passed in every run after

## Summary

`gate_children_never_inherit_a_caller_exported_bootstrap`, of `path@xtask@tests/gates_bin.rs`,
failed once inside `cargo x gates`, on a branch that changes no Rust source. It passed alone, three
times with its whole test file, and in the next `cargo x gates`. Whether it is a defect of the test
or of the gates is not established.

## Details

### What

On 2026-11-10, on the branch that added the milestone that moved the goals to the project's root, whose diff held
documents only, `cargo x gates` exited 1: the gates `fmt`, `check`, `commits` and `clippy` passed,
and `test` failed on this one test, at its assertion that the run of the gates over its fake
project succeeds:

```
thread 'gate_children_never_inherit_a_caller_exported_bootstrap' panicked at tools/xtask/tests/gates_bin.rs:145:5:
assertion failed: status.success()
```

The other 8 tests of the file passed in that run. Then, with nothing changed:
`cargo test -p xtask --test gates_bin gate_children_never_inherit` passed;
`cargo test -p xtask --test gates_bin` passed three times in a row; `cargo x gates` exited 0.

It does not reproduce on demand: one failure in six runs of the test. The failing run was the
first build of the test binary after a `git worktree remove` and a push, and ran beside the other
test binaries of the workspace; the passing runs were warm. The test removes its fake project only on success, so the failing run left it under the system's
temporary directory. Its `target/gates/` holds `clippy.log` and `test.log`, each one line of the
fake cargo, and no log of `fmt`, `check` or `commits`. So the first gates of that run wrote no log,
and the later ones ran. Not established: why. A lead, not checked: the test writes its fake
programs and runs them at once, while the other tests of the file do the same in other threads,
and a program just written cannot be run while another thread's child still holds it open for
writing; the failing run printed neither the run's output nor its error, which the test now
prints when it fails.

### Why it matters

`cargo x gates` is the one verdict a session trusts before a merge. A test that fails once in
several runs makes a red verdict cost a second run, and a session may take a real failure for this
one.

### What would close it

The test prints the run's status, its stdout and stderr, and every gate log of the fake project
when its first assertion fails. What remains: either a second failure read from them and its cause fixed, or, if none recurs over the
gates runs of the next three merged branches, this entry closed as not reproduced.
