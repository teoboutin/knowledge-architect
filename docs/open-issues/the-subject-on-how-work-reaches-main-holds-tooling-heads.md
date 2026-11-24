---
kind: todo
---
# The design home's subject "How work reaches main" holds heads on how the code builds and is tested, which change apart from its branch and merge heads

## Summary

The level-two subject "4. How work reaches main" of `register@knowledge-architect@design` holds six
heads that answer how the workspace builds, lints and tests, beside those that answer how a branch, its
review and its merge go. A session that searches the subject for one reads the other. The heads move to
a subject of their own in a commit that moves headings and decides nothing.

## Details

### What

The second test of `skill@knowledge-architect-decision-recording@current-design` splits a subject whose
"heads answer questions about parts of the Component that change apart". Such heads, in the order of
the subject:

- `design@knowledge-architect@toolchain-is-pinned`;
- `design@knowledge-architect@doc-lints-table`;
- `design@knowledge-architect@scratch-removed-by-a-guard`;
- `design@knowledge-architect@tempfile-for-tests-alone`;
- `design@knowledge-architect@nothing-sweeps-a-killed-run`;
- `design@knowledge-architect@a-build-is-tied-to-its-checkout`.

The others, from `design@knowledge-architect@git-flow` to
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`, answer how a branch, its
review and its merge go. Not established: whether a notice of `cargo klarch worklist --out <dir>` names
the cluster, or whether one of these heads belongs in another subject that exists, such as
"2. Naming, facades and what a package ships".

### Why it matters

The record search reads a subject's heads together, as one cluster that one agent reads whole, so a
piece of work on the toolchain or on a test's directory loads the branch and merge heads with it, and
the reverse.

### What would close it

A level-two subject for how the workspace builds and is tested, in a commit of its own that moves the
headings, with `git diff --color-moved` showing moved lines alone; the subjects' numbers and any text
that names them by number brought to it; `cargo klarch check` passing.
