---
kind: design
---
# A Rust item named in prose has no checked form

## Summary

The design homes, the scoped `CLAUDE.md` files and the code comments name Rust modules, types and
functions in plain backticks. To the checker that is plain text, so a move or a rename of the item
leaves the name stale with a passing check, where a `path` reference to the item's file would
dangle. No reference kind names a Rust item.

## Details

### What

`primer@checker-model` lists the reference kinds; none names a Rust item.
`design@model@names-collected-where-declared` collects a Rust name where it is declared, for a rule
marker, "A name is the one thing prose cannot reach"; it holds names, and no resolution of a module
path.

The instance seen: the slice that created the `model` Component, 2026-11-04, a split of one crate
into two. Its spec had a step to search for the module paths no check reads, such as
`check::tree`, since the move left them stale with a passing check. The search found most; one,
wrapped over a line break in a comment of the installed-text crate's build script, was found by two
reviewers. The session's rewrite of the paths produced three names of items no facade publishes,
which only the code-claims reviewer found. Each name was written bare, beside no issue entry, since
none recorded the missing form.

Suspected mechanism, in one sentence: the checker resolves a path and an entry but has no kind that
resolves a name against the Rust parse, so nothing it judges changes when an item moves.

The nearest rival to a new kind: a convention of naming a module by its file, as a `path`
reference, which dangles at a move today. Against it: a module path names an item a facade
publishes, which a file path does not, and the three wrong names the reviewer found were of that
kind. Both shapes are for the discussion.

### Why it matters

Renaming or removing an item does not give the list of what to repair, against
`goal@knowledge-architect@records-reach-their-reader`. `primer@where-knowledge-goes` asks a pointer no
checked form expresses to be written beside a reference to an issue entry that records the missing
form; this entry is that issue, and a site naming a Rust item in prose cites it.

### What would close it

The re-entry point: a discussion under `skill@knowledge-architect-design` that gives a reference kind
naming an item by its anchor and its path, resolved against the Rust parse `model` holds, or rules
that none is worth its cost; then the sites converted, or left beside this entry.
