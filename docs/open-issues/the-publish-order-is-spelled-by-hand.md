---
kind: todo
---
# The order in which the release publishes the crates is spelled by hand in the release workflow and in a head

## Summary

The release workflow lists the published crates, in publish order, twice, and
`design@knowledge-architect@partial-publish-rerun` spells the same order in its body. Each list is
written by hand, so a crate added, renamed or given a new dependency leaves them stale until
someone notices.

## Details

### What

The three sites:

- `path@knowledge-architect@.github/workflows/release.yml`, the `publish` job's step `publish`: a
  shell `for` loop over `knowledge-architect-git knowledge-architect-agent-skills
  knowledge-architect-model knowledge-architect-check knowledge-architect-testing
  knowledge-architect-query knowledge-architect-transcript knowledge-architect-create
  knowledge-architect-gates knowledge-architect-mutate knowledge-architect`, which publishes each
  crate and, inside the same loop, skips a version crates.io already holds;
- the same file, the `verify` job's step "the tag is the version of every published crate": a
  second `for` loop over the same eleven names, in the same order;
- `design@knowledge-architect@partial-publish-rerun`, whose body spells which crate goes before
  which.

The drift of hand-written crate lists is recorded in retrospective files of four sessions of this
repository: the release of 0.9.0, 2026-11-01, where the release skill's package list missed
agent-skills' fragments directory and its dry-run step named two expected warnings where three
appeared; the slice that created the `git` Component, 2026-11-02, where the skill counted the
crates; and the last two slices of the Component split, 2026-11-06 and 2026-11-07, where the list of
what a new crate owes missed the release skill's paragraph on a failed publish. The release
skill's own lists can be derived with no code: the exact pins by the grep its step 1 gives, and the
published set as `path@xtask@src/changelog.rs` computes it, every directory under crates/ that holds
a Cargo manifest. The workflow's two loops need a source the workflow runs, which is code, so they
are this entry's.

Measured at the intake of 2026-11-07, with `cargo metadata --no-deps`: every normal dependency of
each crate of the loops comes before it in their order, and every workspace package but `xtask` is
in them, so no list is false today.

Constraints on a fix: `design@knowledge-architect@partial-publish-rerun` publishes each crate before
the crates that depend on it and skips a version crates.io holds;
`design@knowledge-architect@the-changelog-ships-in-every-crate` makes every published crate carry
the changelog copy. Ruled out: a test comparing the lists, which keeps every copy.

### Why it matters

A list that drifted publishes a crate before one it depends on, and the release stops part-way,
against `goal@agents-work-without-drift`; four sessions met a hand-written
crate list that had drifted.

### What would close it

One source of the publish order that the release workflow runs, such as a `cargo x` command that
prints the crates under crates/ in dependency order, with a test over this workspace; the two loops
reading it, and the head's body pointing to it instead of spelling the order.
