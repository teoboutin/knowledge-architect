---
kind: todo
---
# The standing-entry searcher's return has no list for an entry kept on a doubt

## Summary

`agent@knowledge-architect-standing-entry-searcher` returns two lists, "Bears on the work" and
"Read and judged unrelated", and asks for an entry it cannot rule out "with the doubt as its
reason". Its agents write that doubt each in their own words, so the session cannot tell, from the
return, which entries bear and which were kept on a doubt.

## Details

### What

- The return of `agent@knowledge-architect-standing-entry-searcher@what-to-return` holds two lists;
  its stance, "You lean to recall", asks for an entry kept on a doubt to be returned with the doubt
  as its reason.
- An instance, in the design discussion that produced the `rejected` register: three standing-entry
  searchers over 141 entries returned 41, and 12 of them carried a doubt in their reason, written
  as "Doubt: …" (9) or "included because I cannot rule it out" (3).
- `agent@knowledge-architect-design-entry-searcher` has a list of its own for such an entry, "May
  bear", per `design@agent-skills@search-relevance-grade`.

### Why it matters

The session reads every returned entry whole either way, but its first round states which entries
the question bears on, and an undefined doubt makes that statement depend on parsing free text in
each reason, against `goal@knowledge-architect@agents-get-a-complete-workflow`. The two searchers
now return in two shapes for one kind of judgement.

### What would close it

A change of the standing-entry searcher's return, argued under `skill@knowledge-architect-design`:
a third list for an entry kept on a doubt, as the design-entry searcher has, or a recorded decision
that the doubt stays in the reason, with the argument.
