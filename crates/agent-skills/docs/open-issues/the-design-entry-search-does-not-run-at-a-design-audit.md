---
kind: question
---
# The design-entry search does not run at the design audit of a slice or a spec

## Summary

`agent@knowledge-architect-design-entry-searcher` is sent at a design discussion's grounding and at
a retrospective intake's, per `design@agent-skills@design-entry-search`. The design audit of a
milestone slice or of a spec sends the standing-entry searcher alone, and reads the design homes
and the `rejected` homes in the session. Should it send the design-entry search too?

## Details

### What

- `design@agent-skills@standing-entries-searched-before-the-work` says the audits of a slice or a
  spec send the standing-entry search alone.
- `skill@knowledge-architect-planning@working-a-slice`, point 2, reads a slice's entry against the
  code and the design homes in the session.
- The plan document was read against the goals, the design heads and the `rejected` entries by
  `agent@knowledge-architect-design-conformance-reviewer` before the audit, per
  `design@agent-skills@plan-read-against-the-record`, so what an audit's reading must catch is a head
  that changed or entered after that review, or a decision the slice's entry makes that the plan
  document did not.
- No instance is recorded of an audit that missed a design entry or a `rejected` entry the work bore
  on. The intake's case had one, and the intake now sends the search.

### Why it matters

An audit that misses a head a slice contradicts finds it only at the slice's review, after the code
exists, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A recorded instance of a design audit that missed a design entry or a `rejected` entry the work bore
on, then a change argued under `skill@knowledge-architect-design` that sends the search there; or a
decision, argued the same way, that the audit's own reading suffices, with its reason.
