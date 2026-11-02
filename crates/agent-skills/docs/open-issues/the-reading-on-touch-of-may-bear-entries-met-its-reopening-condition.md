---
kind: design
---
# The reading on touch of `may-bear` entries met its reopening condition, at both plan reviews of one spec

## Summary

`design@agent-skills@may-bear-read-on-touch` has the grounding read whole only the `may-bear`
entries a proposal touches, and says: "A search whose `may-bear` entries left unread are found by
the review to bear reopens it." At both plan reviews of the goal standard's spec, deleted by "[plans] The goal-standard spec
leaves: its work has landed", its
design-conformance reviewer found entries the grounding had left unread that the work bears on.

## Details

### What

The grounding of the design discussion on goals graded 79 entries `may-bear` and read 10 of them
whole, listing the rest with the searcher's reasons.

- The first plan review found three of the unread entries bearing: a tripwire on the pin move's
  audits, a `rejected` entry on the transcript reviewer, and an issue on a built default awaiting
  the owner.
- The second found six more, after the owner widened the work to bring this repository's goals into
  conformance on the same branch: five goals and `design@agent-skills@goals-required`, each of
  whose searcher's reasons, written against the work as first drafted, no longer held.
- Each was read whole and its outcome written into the spec, as the reviews' records in the
  branch's commit messages list.

The head now says "reopens this decision", which settles the reading of "it".

The second case is answered: the design skill's search waits until the scope settles, per
`design@agent-skills@search-waits-until-the-scope-settles`, and a thread on a new subject after it
gets a search of its own, so a reason graded against a scope that widened before the search no
longer stands. The first case is not: the three entries the first review found were left unread
with no widened scope, the reading on touch itself missing them.

### Why it matters

`goal@knowledge-architect@agents-work-without-drift` asks that a departure from the record be
caught before it merges. Here the review caught each, so nothing merged unread, but each catch cost
a repair round of the plan document. A scope widened after the search makes its reasons stale, a
case the decision's argument, measured on one search over this repository, did not weigh.

### What would close it

A design discussion on the decision, under `skill@knowledge-architect-design`, opened on the owner's
word, that keeps the reading on touch with a stated answer to the first case, entries left unread
with no widened scope that a review finds bearing, or changes it; or the owner's word that one
spec's reviews do not reopen it.
