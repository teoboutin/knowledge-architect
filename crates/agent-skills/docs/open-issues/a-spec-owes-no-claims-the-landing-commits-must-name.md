---
kind: question
---
# The landing commits of a spec name each of its claims, and a spec owes no section that states them

## Summary

`skill@knowledge-architect-planning@working-a-slice` point 3 says the commits of a slice or of a
spec's work "name each claim of the spec with its test and its outcome". A slice's entry owes a
Claims section; the fixed sections of a spec, per `skill@knowledge-architect-planning@spec-contents`,
owe none. So for a spec, the claims the commits must name are stated nowhere the spec is required
to hold them, and the implementing session derives them from the decided design.

## Details

### What

Found by the cold-implementer review of `spec@plans@defined-ids-outside-references`, which had no
claims until its review repair added them to its first step. The two texts read:
`skill@knowledge-architect-planning@working-a-slice`, point 3, and the table of
`skill@knowledge-architect-planning@spec-contents`, whose sections include an implementation
sequence that says "what each builds and what it fails alone on" and no claims. Not established:
whether earlier specs of this repository stated claims, and whether their implementing sessions
named them.

### Why it matters

A claim a spec does not state is derived by the session that implements it, which did not see the
design discussion, against `goal@knowledge-architect@agents-get-a-complete-workflow`, which asks
that each instruction say what it asks plainly enough to follow.

### What would close it

A reading of the specs in this repository's history, `git log --diff-filter=D --name-only --
docs/plans/specs/`, for whether their landing commits named claims the spec did not state; then
either a line in the step of the implementation sequence of
`skill@knowledge-architect-planning@spec-contents` that states its claims, or the owner's ruling
that the decided design is enough.
