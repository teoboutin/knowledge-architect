---
kind: design
---
# A lighter pass of the record search for the Components a piece of work does not touch

## Summary

The record search reads the whole record for every piece of work, per
`design@agent-skills@search-reads-the-whole-record`. In a project of many Components that change
apart, most clusters belong to Components the work does not touch, and a lighter reading of those
could cut the agents one grounding costs. The owner parked the question until such a project shows
the cost.

## Details

### What

The leads, from the design discussion that decided the whole-record search:

- The owner's proposal: a searcher that reads only the entries' titles in the Components the work
  is not expected to touch, and reads deeper where a title raises a doubt. The owner then argued for
  reading the titles of design entries and every issue and tripwire whole.
- A title shows neither a tripwire's firing clause nor a deferred issue's trigger, so a title-only
  pass misses what the search of standing entries exists to find; a design head's title states its
  rule, per `design@agent-skills@title-states-the-rule`, so skimming heads by title is more
  defensible.
- Which Components are untouched can be computed: a cluster of an untouched Component with an edge
  into a cluster of a touched one is read whole.
- `klarch worklist` can take a Component filter later without changing the report format.
- The owner judged that filtering by the Components touched may make sense in a project with more
  Components, such as thaum, and expects to start a project of about 30 core Components, about 20
  of them nearly standalone and the others depending on those.

Measured at the decision, with `klarch worklist --out` at its default bound: 9 agents over this
repository, 11 over thaum.

Re-entry point: the first design discussion in the owner's planned project of about 30 Components,
or a retrospective that reports the search's cost.

### Why it matters

A grounding whose search costs more agents than sessions will pay for gets skipped, against
`goal@knowledge-architect@agents-work-without-drift`, which the search serves.

### What would close it

A design discussion under `skill@knowledge-architect-design`, at the re-entry point, that keeps the
whole-record search with the cost measured in that project, or adds a lighter pass with the rule
that decides which Components it reads lightly.
