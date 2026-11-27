---
kind: question
---
# No head states which public items of the published crates a consumer may rely on

## Summary

Every published crate's public items are public in the Rust sense and on crates.io, but the owner
reported that only some of them are offered to consumers as an interface they may rely on: the
command line of `klarch`, the gates and the mutation libraries, and the framework an extension's
binary is built on. The other public crates, `git`, `cargo-reader` and `query` among them, are
built for this project itself. No head defines the line, nor how an item enters the offered
interface, when, or what entering it binds.

## Details

### What

- `design@knowledge-architect@api-facade` says which crates hold what an extension is written
  against (`check` and `cli`, `testing` for an extension's tests) and which hold what no extension
  is written against (`query`, `transcript`, `create`, `git`, `cargo-reader`), and gives every crate
  a facade; it does not say that the second group is no interface a consumer may rely on.
- `design@knowledge-architect@versioning-policy` classes "a breaking change to the library API" as
  major, without saying which library API.
- The owner reported, at the review before the merge of slice 4 of the per-crate changelogs, the
  list of the summary, said it may be incomplete, and asked for a later discussion that defines the
  line and the methods to decide what enters the offered interface, when and how: the opt-in is
  item by item, and binds once documented as more stable.
- The instance that raised it: `design@query@a-function-takes-a-configured-manifest` was argued
  from whether a program that links `query` and no check is served by
  `goal@documentation-has-a-structure-and-a-model`; the head now rests on the owner's report.

### Why it matters

The versioning policy classes changes by what a consumer could no longer do, per
`design@knowledge-architect@capability-cost-before-migration`, and a head that argues from what a
consumer relies on, as the instance does, has no recorded line to read. A release's review cannot
tell whether a change to `query`'s functions is a library break, which strains
`design@knowledge-architect@versioning-policy`.

### What would close it

A design discussion under `skill@knowledge-architect-design`, on the owner's word or when a change
is classed by whether a consumer relies on a public item: it defines the offered interface, how an
item enters it and what entering binds, in a head of `register@knowledge-architect@design` that
`design@knowledge-architect@api-facade` and `design@knowledge-architect@versioning-policy` cite.
