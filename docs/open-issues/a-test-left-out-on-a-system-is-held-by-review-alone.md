---
kind: design
---
# A test left out on a system is held to the two kinds that the head on platform skips admits by a review alone, and one reached main

## Summary

`design@knowledge-architect@platform-skips` admits two kinds of test left out on a system:
development workflow on Windows, and a behaviour a system lacks by nature. Nothing but a reviewer
reads each mark against them, and a test of product behaviour left out on Windows for neither
reason reached main with slice 3 of the per-crate changelogs. The tripwire that watched this fired,
and its response asks for this decision to be reopened on a check that lists each mark with its kind.

## Details

### What

- The instance: the test `a_symlinked_cargo_toml_is_read_alike_in_the_working_tree_and_the_index`
  of `path@testing@tests/changelog_checks.rs` carried `#[cfg(unix)]` and made its link with
  `std::os::unix::fs::symlink`, with no reason beside the mark and no reference to the head, while
  the same package's `link_file` in `path@testing@tests/links/mod.rs` makes a link on every system.
  It landed with the commit "[model, check, testing, knowledge-architect] ..." of slice 3, passed
  every gate and that slice's review, and was found by the standing-state axis of the review before
  the merge of slice 4, which repaired the test in the commit "[model, query, cli, check,
  agent-skills, testing] Slice 4's second review: ...".
- The tripwire `a-product-test-left-out-on-a-system` of the root's tripwires home fired on it: its
  firing clause, "the review of a branch that did not add the mark, finds on main a test of product
  behaviour that a system leaves out ... which neither of the head's two kinds admits", holds.
- The lead the tripwire named: `git grep -nwE 'unix|windows|target_os|target_family|target_vendor' -- '*.rs'`,
  a condition in `cfg`, `cfg_attr` or `cfg!` naming a system.

Suspected mechanism: a mark that leaves a test out is one attribute line, which a review reading
the test's claim and its code passes over as setup.

### Why it matters

A test left out on a system for convenience leaves a product behaviour untested where it is
shipped, which `design@knowledge-architect@platform-skips` exists to prevent and
`design@knowledge-architect@platforms-served` makes a cost on every system it serves.

### What would close it

A design discussion under `skill@knowledge-architect-design` that reopens
`design@knowledge-architect@platform-skips` on the response the tripwire named: a check, such as a
gate or a test, that lists each mark leaving a test out on a system with the kind it claims, in
place of the review alone; or an argument that the review suffices, recorded in the head.

### Re-entry point

The design or the planning of the next work that adds a mark leaving a test out on a system, or the
owner's word.
