---
kind: defect
---
# Tests of the gates binary fail at random when a fake program they just wrote is busy

## Summary

`gate_children_never_inherit_a_caller_exported_bootstrap`, of `path@xtask@tests/gates_bin.rs`,
failed once inside `cargo x gates`, on a branch that changes no Rust source, and
`annotations_appear_under_actions_alone`, of the same file, failed once on another branch, its
first gate reporting "could not run: Text file busy (os error 26)". Each passed in every run after.
The second failure establishes the cause the first left as a lead: a fake program the test has
just written is run while another thread's child still holds it open for writing, so the kernel
refuses the run with ETXTBSY.

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
test binaries of the workspace; the passing runs were warm. The test removed its fake project only on
success then, so the failing run left it under the system's temporary directory. Its `target/gates/`
held `clippy.log` and `test.log`, each one line of the fake cargo, and no log of `fmt`, `check` or
`commits`. So the first gates of that run wrote no log, and the later ones ran. A failed run leaves
no directory now, per `design@knowledge-architect@scratch-removed-by-a-guard`, and what it can be
read from is the output and the logs the test prints. Not established: why. A lead, not checked: the test writes its fake
programs and runs them at once, while the other tests of the file do the same in other threads,
and a program just written cannot be run while another thread's child still holds it open for
writing; the failing run printed neither the run's output nor its error, which the test now
prints when it fails.

On 2026-11-14, on the branch of the last slice of the milestone on split guidelines and plan
directories, whose Rust change was one comment, `cargo x gates` failed its `test` gate on
`annotations_appear_under_actions_alone`, at `path@xtask@tests/gates_bin.rs` line 332, the output
the test now prints on failure showing its first gate:

```
fmt        could not run: Text file busy (os error 26)
```

It passed alone, `cargo test -p xtask --test gates_bin`, and in the next `cargo x gates`.

### Why it matters

`cargo x gates` is the one verdict a session trusts before a merge. A test that fails once in
several runs makes a red verdict cost a second run, and a session may take a real failure for this
one.

### What would close it

The tests of `path@xtask@tests/gates_bin.rs` write each fake program so that no thread's child holds
it open for writing when it runs, for example written under another name and renamed into place,
and no gates run fails on ETXTBSY over the gates runs of the next three merged branches.
