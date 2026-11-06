---
kind: question
---
# A new crate's first publish by hand needs the versions it pins on crates.io first

## Summary

`skill@klarch-release` has the owner publish a new crate's first version after the merge and before the tag, and the workflow then uploads the rest. A new crate pins the crates below it exactly, and those versions reach crates.io only when the workflow runs.

## Details

### What

Assumption, not run: `cargo publish` refuses a dependency version the registry does not hold. No first publish of a crate with a pinned dependency has happened; `model`, `check`, `query`, `transcript` and `create` are the first. The order that seems to work is the one `design@knowledge-architect@partial-publish-rerun` allows: the job stops at the new crate, the owner publishes it by hand, and the job is re-run.

### Why it matters

`design@knowledge-architect@publish-credential` says an existing crate rejects a publish with an API token, so the crates below cannot go first by hand; the next release takes this path.

### What would close it

A dry run or the release itself shows the order, and the step of `skill@klarch-release` states it.
