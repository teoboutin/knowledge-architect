---
kind: todo
---
# The install's and the installed-file check's comparisons that misjudge a state, fixed on one branch

## Summary

A cluster of the issue record: three entries on how the install and the check of the installed files
judge what is on disk or staged, which one branch closes together. Each member's own entry holds its
observation, its reason and its closing work; this entry holds only that they go together.

## Details

### What

The members:

- `issue@check@install-agent-skills-exits-two-after-writing`
- `issue@check@a-staged-installed-finding-can-name-a-repair-that-does-nothing`
- `issue@check@the-self-pin-is-accepted-over-a-tracked-copy-of-the-checker`

They go on one branch because each sits in the install and the installed-file check,
`path@check@src/agents.rs` and `path@check@src/check/agents.rs`, or the pin refusal in
`path@check@src/commands/mod.rs`, with binary tests in `path@testing@tests/binary.rs`. Found by a
triage of the whole issue record on 2026-11-11, which read every entry against the tree and sized
it: each member was judged to have a clear path to closing, and to be more work than a fix of a few
dozen lines.

### Why it matters

Each member's cost of being left is its own entry's. Taken one per branch, each pays again the
grounding, the review and the merge that the others share, so the cluster is the unit the roadmap
orders.

### What would close it

One branch that closes every member. A member closed alone, or found to need a design discussion,
leaves this list in the commit that closes it or moves it; the entry closes with its last member.