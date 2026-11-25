---
kind: design
---
# A touched head is defined by what a plan's harvest changes, not by what its steps change

## Summary

The planning skill lists, for a step before the harvest to conform, the heads a plan's harvest
touches. In a project whose product is text, a step of the work writes into existing heads, which
the definition leaves out, so the review before the merge reads them with their older defects, the
failure the definition's own reason names. One session kept such heads in its list by the reason.
Widening the definition contradicts two heads.

## Details

### What

The definition, `skill@knowledge-architect-planning@cutting-steps-and-slices`: "A head the harvest
touches is one whose text it changes without reversing or rewriting its decision", with its reason,
"so that the harvest writes into heads that already conform, and the review before the merge judges
the harvest's change rather than a head's older defects". Two heads state the list by the harvest:
`design@agent-skills@pre-harvest-conformance`, "the heads its harvest will touch without reversing
or rewriting their decision", and `design@agent-skills@scoped-auditor-on-touched-heads`, "Once a
plan document lists the heads its harvest will touch". A third is wider:
`design@agent-skills@existing-heads-on-touch` brings "a head that a change touches" to the rules,
in planned work by a step before the harvest, which covers a step's touch.

The instance: the slice that gave each Component's head its arguments in this repository,
2026-11-14, W2 of its workflow file, and the first note for the intake of the workflow file of the
slice that moved the conformance check before the harvest. A step of the work brought twelve heads
to a new rule, and the harvest wrote one new head and the changelog. Read literally, the twelve
heads were outside the list; the session read the definition by its reason and kept them in, and
the slice's spec lists twelve root heads its harvest "touches without reversing them".
The tripwire that watched whether a plan's list misses heads did not fire, the heads being in the
list, and it left at its horizon.

A second instance: the spec of the branch that added the doc gate to the recommended list of the
gates library and the documentation lints to this repository. Its step 3 brought two heads of the
gates Component to the rules because its harvest would write the doc gate into them, and listed them
as heads its harvest touches; its step 4 rewrote `design@agent-skills@splits-planned-before-work`,
which the spec listed apart, as a head the work rewrites. That branch, too, read the definition by
its reason. Its review before the merge found the re-entry point below met, and the owner chose to
keep the issue open with its re-entry point as written, rather than hold the discussion before that
branch merged.

A third instance: `spec@plans@gates-on-windows-and-macos`, whose step 8 brings to the rules, before its
harvest writes into them, the heads that the design-record audit found to fail, and splits two bundles. It
lists the heads its harvest touches by the definition and reads the step's heads by its reason, so every head
whose text the step changes is one its harvest rewrites or touches. The spec's search met the
re-entry point, and the entry stays open.

The suspected mechanism, in one sentence: the definition names the harvest, while a project whose
product is text writes heads in its steps.

The shapes:

- "a head the plan's work or its harvest touches", the file's rewording;
- a narrower rewording that counts only a step's edit of an existing head's text;
- no change, since the session read the definition by its reason and the reason sits beside it.

What would rank them is not settled by the record: in a slice whose work is the record itself, the
step that writes the heads can be the conforming step, and whether a step that brings heads to a new
rule must first be preceded by a conformance of their older defects, or is that conformance, no
head answers. Any widening keeps conformance out of the harvest, per
`rejected@agent-skills@conformance-in-the-harvest`.

Re-entry point: the planning of the next plan document whose steps write into existing heads, at
the moment it writes its list; the discussion under `skill@knowledge-architect-design`.

### Why it matters

It strains `design@agent-skills@pre-harvest-conformance` and
`design@agent-skills@scoped-auditor-on-touched-heads`: a session that reads the definition by its
words leaves out the heads a step writes, and the review before the merge judges their older
defects with the branch's change, the cost those heads exist to remove.

### What would close it

The discussion picks a shape and rewrites the planning skill's definition and the two heads to it,
or keeps the definition with a sentence on a step's touch stated in its head.
