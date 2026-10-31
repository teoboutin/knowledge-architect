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

Measured at the decision, at the default bound: 9 agents over this repository at ed9dd60, with
`klarch worklist --out`; over thaum at its commit edd2cb5d, 8 agents for the clusters within the
bound and 3 above it sent alone, with the scripts the message of the commit
"[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds, since the core binary refuses thaum's manifest, which holds the tables of
thaum's own extension.

Re-entry point: this repository's intake of a retrospective file from the owner's planned project
of about 30 Components, or of any retrospective file that reports the search's cost.

The re-entry point is met by W4 of the retrospective of the intake deciding by authority,
2026-10-31, which reports the search's cost: its `bears.md` of 80 entries, about 35,000 words, was
the session's largest read. The intake of seventeen retrospective files of 2026-10-25 to 2026-10-31,
at main `v0.8.0-224-g7e3784aa`, ran the search over 12 agents; its `bears.md` held 200 entries in
83,788 words and its `may-bear.md` 164 entries in 57,415 words, by `wc -w`, more than that session
could read whole beside its own work. The discussion this entry asks for is left to a branch of its
own; `issue@core@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent` holds the part of the
cost that repeated grade lines make.

### Why it matters

A grounding whose search costs more agents than sessions will pay for gets skipped, against
`goal@knowledge-architect@agents-work-without-drift`, which the search serves.

### What would close it

A design discussion under `skill@knowledge-architect-design`, at the re-entry point, that keeps the
whole-record search with the cost measured in that project, or adds a lighter pass with the rule
that decides which Components it reads lightly.
