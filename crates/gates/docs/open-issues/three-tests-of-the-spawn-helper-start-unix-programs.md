---
kind: todo
---
# Three tests of the spawn helper start `sh` and `true`, which a Windows machine without Git's programs lacks

## Summary

Three unit tests of `path@gates@src/process.rs` start `sh` or `true` as the child. Where the path holds
neither, each fails to spawn. On GitHub's Windows runner Git for Windows provides them.

## Details

### What

`captures_both_streams_and_the_exit_status` and `an_added_env_reaches_the_child` start `sh -c`, and
`a_passing_child_reports_success` starts `true`. Run on Linux with a path that holds git alone, through the
built test binary of the package `knowledge-architect-gates`: these three fail, and no other of the module.
They passed on `windows-latest` in run 45652817841, whose runner has Git's programs on the path.

The tests of the hand mutation have the same shape, per
`issue@mutate@the-hand-mutation-fails-in-several-ways-on-windows`, and those of
`path@xtask@tests/gates_bin.rs` start a compiled program, which every system can.

### Why it matters

A test that depends on a program the machine may lack fails on a tree that is right, and the gates library is the
one that `design@knowledge-architect@platforms-served` has run on Windows.

### What would close it

The three children made by a program of the test, such as the test binary started again under a flag, with the
same claims and mutations.
