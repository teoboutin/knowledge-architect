---
kind: todo
---
# A register home that `new-component` writes says "No entry yet." after its first entry lands, and nothing reports it

## Summary

`home_text` in `path@core@src/cli/create.rs` writes "No entry yet." into each register home of a
new Component. The sentence stays when the first entry lands, and no check reads it.

## Details

### What

Seen in the work that created the git Component: the git Component's issue home,
crates/git/docs/open-issues/README.md, still said "One file per outstanding item of git. No entry
yet." over four entries, until the review before its merge found it. The sentence is written for
every register `home_text` knows: goal, design, rejected, tripwire, issue, spec, milestone and any
other.

### Why it matters

A home's README is the directions a reader gets for that directory, per `primer@where-knowledge-goes`,
and a false sentence there is met by every session that opens it, against
`goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

`home_text` writes no sentence that turns false at the first entry, or the check reports a home
that says "No entry yet." over an entry; a test of `new-component` or of the check showing it.
