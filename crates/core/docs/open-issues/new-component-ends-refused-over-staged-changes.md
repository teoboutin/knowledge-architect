---
kind: question
---
# `new-component` ends with exit 1, writing no index, where the index holds staged changes

## Summary

In a committed project whose index differs from HEAD, `klarch new-component <DIR>` writes the
manifest's line and the Component's documents, then the refusal of `check --fix` stops it before
the indexes, exit 1, naming the new Component's `index.md` files. Should that refusal name a path
the staged tree does not need?

## Details

### What

Reproduction, with the checker built from this repository, in a scratch directory:

```sh
klarch init p && cd p && git add -A && git commit -qm p
echo more >> README.md && git add README.md
klarch new-component c     # exit 1: "these generated files would differ between the working
                           # tree and the staged tree: c/docs/open-issues/index.md"
klarch index               # writes it; `klarch check` then passes
```

`new-component` ends through `fix_then_check` in `path@core@src/cli/mod.rs`, whose comparison
names every generated path the working tree needs and the staged tree's model does not, as
`design@core@fix-refusal-mixed-state` decides: "It names each such path, a path the staged tree
does not need included". The staged tree's manifest lacks `c`, so each of its indexes is such a
path. The message names `index` as the repair, and the run is complete after it.

Not established: whether a partial commit can carry a path the staged tree does not need into a
failing tree, which is what including such a path in the refusal guards against.

### Why it matters

A staged change is the usual state of a project being worked on, so `new-component` meets the
refusal often, and its message speaks of a partial commit the user did not make. It strains
`design@core@component-command`, whose run ends as `init` ends, with the check.

### What would close it

The argument for naming a path the staged tree does not need, found in the history of
`design@core@fix-refusal-mixed-state` or derived again: if it holds, the outcome is a message of
`new-component` that says why; if not, the refusal leaves such paths out, with a test that runs
`new-component` over staged changes.
