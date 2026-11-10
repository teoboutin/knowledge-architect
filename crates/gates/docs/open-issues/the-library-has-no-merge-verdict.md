---
kind: todo
---
# The gates library has no single merge mode in the general form of thaum's merge-ready flag

## Summary

The library gives no verdict that says whether the gates let a branch merge, from one command with
the same invocation in every target project. No design head names that command and its merge
invocation yet, per
`issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`. The library holds a rebase gate,
run behind `--require-rebased`, and a `--locked` flag (`GatesArgs` and `rust_project` in
`path@gates@src/gates.rs`), and this repository's xtask runs that list unchanged. It holds no
clean-tree gate, and no single flag that runs every gate a merge owes. thaum keeps its own runner,
whose `--merge-ready` flag adds a rebase gate and a clean-tree gate and passes `--locked` to every
gate that resolves dependencies, per thaum's xtask README.

## Details

### What

A merge mode of the library's runner, in the general form of thaum's `--merge-ready`, which the owner
named as applying to any target project. The owner's plan is that thaum moves to the published
library, and that the library gains what thaum needs.

### Why it matters

While the library lacks the clean-tree gate and the one flag, a target project that wants them
writes merge logic of its own, as thaum does, and the one invocation every target project shares
cannot exist.

### What would close it

The library holds the merge mode, with tests; this repository's xtask and thaum use it; the design
head that names the invocation, per `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`,
cites it.
