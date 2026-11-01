---
kind: todo
---
# The gates library gives no verdict on whether the gates let a branch merge

## Summary

`goal@gates@gates-from-a-list` lists among what comes with the library the verdict that says whether
the gates let a branch merge, and `goal@knowledge-architect@serves-one-kind-of-project` asks that one
command give it with the same invocation in every target project. The library offers no such mode:
this repository's xtask adds a rebase check behind `--require-rebased`, and thaum keeps its own runner
with a `--merge-ready` flag, which adds a rebase gate and a clean-tree gate and passes `--locked` to
every gate that resolves dependencies.

## Details

### What

A merge mode of the library's runner, in the general form of thaum's `--merge-ready`, which the owner
named as applying to any target project. The owner's plan is that thaum moves to the published
library, and that the library gains what thaum needs.

### Why it matters

While the library lacks the mode, every target project writes merge logic of its own, and the one
invocation every target project shares cannot exist.

### What would close it

The library holds the merge mode, with tests; this repository's xtask and thaum use it; the design
head that names the invocation, per `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`,
cites it.
