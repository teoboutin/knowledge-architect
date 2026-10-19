---
kind: design
---
# An intake of many files at once reads every standing entry the search returns whole, and the cost grows with the batch

## Summary

`skill@knowledge-architect-retrospective-intake@ground-before-judging` runs one search of the
standing entries for every file received together, and asks the session to read every entry
returned whole. The searchers lean to recall, and a batch names many subjects, so the session's
reading grows with the batch. The owner judged it a problem to handle one day, not now.

## Details

### What

- The instance: this repository's intake of thirteen received retrospective files. Three
  standing-entry searchers over the 84 issues and 60 tripwires returned 64 entries, about 167 KB of
  text, which the session read whole, as step 3 asked at that intake: "Read every entry returned
  whole with `cargo klarch show <ref>`, never from its reason line." Step 3 now asks the same of
  every entry the design-entry search grades `bears` or `may-bear`.
- The reading paid for itself in part: it found the tripwire judgements two of the owner's rulings
  rest on, and three issues whose closing conditions the files met. Not established: how many of
  the 64 entries bore on no outcome.
- The searcher's description asks the same reading of every dispatching session, per
  `design@agent-skills@standing-entry-search-agent`.
- The intake also sends the design-entry search at its grounding, per
  `design@agent-skills@design-entry-search`, whose `bears` and `may-bear` entries the session reads
  whole too, so the reading this entry measures grows with it. The owner accepted that cost as
  unmet when the search was sent there.

The suspected mechanism, in one sentence: a search whose work is the union of many files returns
the union of their neighbourhoods, and the rule that the session reads each entry whole does not
scale with that union.

Re-entry point: the next change to the intake's grounding or to the standing-entry searcher's
return, or earlier on the owner's word.

### Why it matters

The session's context is what an intake spends on reading instead of judging, against
`goal@knowledge-architect@agents-get-a-complete-workflow`. Reading from the reason line alone is
what the rule refuses, since a reason line can be wrong.

### What would close it

A design discussion under `skill@knowledge-architect-design` on how a batch intake reads its
returned entries: the session reads whole only the entries a proposal rests on, the subagents of
checks a to c reading the rest; the batch split into several searches; or the rule kept, with the
measured cost, recorded with its reason.
