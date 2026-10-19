---
kind: question
---
# The design-entry search runs only at a design discussion's grounding, not at the design audit of a slice or a spec, nor at a retrospective intake

## Summary

`agent@knowledge-architect-design-entry-searcher` is sent by the design skill's step 1 and nowhere
else. The design audit of a slice or a spec, and a retrospective intake, send the standing-entry
searcher alone, and read the design homes and the `rejected` homes in the session. Should they send
the design-entry search too?

## Details

### What

- `design@agent-skills@design-entry-search` places the search at the design skill's grounding, the
  occasion the owner named when proposing it, design sessions.
- `skill@knowledge-architect-planning@working-a-slice`, point 2, reads a slice's entry against the
  code and the design homes in the session, and sends the standing-entry searcher for the issues and
  the tripwires, per `design@agent-skills@standing-entries-searched-before-the-work`.
- `skill@knowledge-architect-retrospective-intake` checks a proposed fix against the `rejected`
  entries of the Component that owns the blamed text, in the session.
- An instance, at an intake: this repository's intake of thirteen received retrospective files,
  analysing C1 of the workflow file of the session that shipped the intake, a skill named in plain
  text before it existed, cited `design@core@planned-path-form` and the `rejected` entry it beat,
  and not the third exception of `design@agent-skills@plain-text-is-no-repair`, which decides how a
  record of a deliberation names what its work removes or will create, and records the owner's
  choice of the backticked bare slug over plain text. The owner recalled the head while ruling, as
  "plain words", and the session corrected the reading from the head. The intake's check c reads
  the `rejected` home of the owning Component and the standing entries; no step of it searches the
  design heads.
- Not established: whether the design audit of a slice or a spec misses design entries as the
  grounding of a discussion did. No instance is recorded there.

### Why it matters

An audit that misses a head a slice contradicts finds it only at the slice's review, after the code
exists, against `goal@knowledge-architect@agents-work-without-drift`. The design skill's grounding
gained the search on the owner's account of design entries missed in part many times; whether the
audits share that failure is unknown.

### What would close it

A recorded instance of an audit or an intake that missed a design entry or a `rejected` entry the
work bore on, then a change argued under `skill@knowledge-architect-design` that sends the search
there; or a decision, argued the same way, that the session's own reading suffices at those
occasions, with its reason.
