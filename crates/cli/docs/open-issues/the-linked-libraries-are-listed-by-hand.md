---
kind: todo
---
# The libraries `klarch` links are listed by hand three times in its `main`, and a fourth time in a test

## Summary

`main` in `path@cli@src/main.rs` names each library of the workspace that `klarch` links in three
lists, `dirs`, `libraries` and `packages`, and the test of the foreign-build refusal in
`path@testing@tests/binary.rs` spells the same packages in the `cargo clean` line it expects. A
library the binary comes to link is joined to each by hand, and nothing reports one it is missing
from but that test, which reads one of the four.

## Details

### What

The four sites, each over the libraries the binary links:

- `dirs`: each library's `component_dir()`, whose fixtures the walk reads as data, per
  `design@check@checker-source-literals-are-data`;
- `libraries`: each library's `CRATE_DIR` and `PACKAGE`, the copies `refuse_a_foreign_build`
  looks for, per `design@check@a-foreign-build-is-refused`;
- `packages`: each library's `PACKAGE`, which the refusal prints as the `cargo clean -p` list;
- the test `a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused` of `path@testing@tests/binary.rs`,
  whose expected `cargo clean` line names every package of `packages` in order.

The slice that added `cargo-reader` to the workspace joined it to all four; the slice's design audit
found the three lists of `main` missing from the slice's own list of what a new crate joins, and
the review before its merge found the four sites listed by hand.

### Why it matters

A library missing from `libraries` is never looked for, so a second checkout holding it is not
refused, and the binary runs another checkout's code for that library, the failure
`design@check@a-foreign-build-is-refused` exists to prevent; no test reports it, since the test
reads only the printed `packages`. A library missing from `dirs` has the string literals of its
source read as prose. Each new library of the workspace that the binary links is an occasion for
the omission.

### What would close it

One source for the set, read by the three lists, such as a list of the linked libraries each
exporting its directory and package name in one value; or a test that compares the libraries named
in `main` with the workspace crates `knowledge-architect` depends on, as `cargo metadata` reports
them, through `cargo-reader`.
