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
"[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds, since `klarch` refuses thaum's manifest, which holds the tables of
thaum's own extension.

Re-entry point: a change to how the record search clusters, packs or writes the record, whose work
weighs the search's cost; the first piece of work in the owner's planned project of about 30
Components; or the owner's word. The discussion that packed the search by search unit and gave each
entry a checked verdict met this point and kept the whole-record search, the session's default, which the owner approved:
its packing read the whole record of this repository within the bound, and a project of about 30
Components remains unmeasured.

An intake of a retrospective file that reports the search's cost is no re-entry point, since an
intake's own work does not include this discussion. The cost measured so far: W4 of the
retrospective of the intake deciding by authority, 2026-10-31: its `bears.md` of 80 entries, about 35,000 words, was
the session's largest read. The intake of seventeen retrospective files of 2026-10-25 to 2026-10-31,
at main's commit "[plans] The spec of the retrospective intake deciding by authority leaves: its
work has landed", ran the search over 12 agents; its `bears.md` held 200 entries in
83,788 words and its `may-bear.md` 164 entries in 57,415 words, by `wc -w`, more than that session
could read whole beside its own work.

The intake of forty retrospective files, 2026-11-07, analysed at main's commit "[plans] The
milestone that split the Components leaves, with the spec of its last slice: their work and their
harvest have landed", measured the cost at every occasion of the search from 2026-10-31 on, by
the files' reports and by `wc -w` where words are given:

- its own search: `bears.md` 333 entries in 133,010 words, `may-bear.md` 249 entries in 78,434;
- two design groundings: 136 entries graded `bears`, 61,473 words; and, for one small in-change
  decision, twelve searchers and 179 entries graded `bears`, about 72,000 words;
- the design audits of seven slices: 172 `bears` (about 68,000 words) and 148 `may-bear` (about
  60,000); 888 KB and 810 KB for two slices of one milestone; 292 entries; 347 entries, about 1 MB;
  13,589 lines, 131,549 words; 425 entries, about 1.1 MB; 166 and 199 entries, 1.07 MB, about
  146,000 words;
- the agents: the work lists of two slice audits held 21 and 22 searchers, above the 20 the harness
  started at once, by the files' report, the rest sent as slots freed.

Those sizes were the reading the grades `bears` and `may-bear` owed the session, which the verdict
report of `design@agent-skills@verdict-report` replaces, the session acting on each verdict as
`design@agent-skills@session-acts-on-the-verdicts` says; the cost left for this entry is the agents a
search takes, which `klarch worklist --out` prints. The analysis is W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow,
in the commit "[intake] The analyses of forty received retrospective files, three outcomes for the
owner".

### Why it matters

A grounding whose search costs more agents than sessions will pay for gets skipped, against
`goal@knowledge-architect@agents-work-without-drift`, which the search serves.

### What would close it

A design discussion under `skill@knowledge-architect-design`, at the re-entry point, that keeps the
whole-record search with the cost measured in that project, or adds a lighter pass with the rule
that decides which Components it reads lightly.
