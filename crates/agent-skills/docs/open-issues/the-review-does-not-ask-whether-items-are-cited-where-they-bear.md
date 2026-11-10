---
kind: design
---
# The installed review names neither of two questions of meaning that a passing check leaves open

## Summary

The review the installed text owes before a merge names neither of two questions: whether each
recorded item that bears on the work is cited where it bears, and whether the work departs from
what bears on it. No design head decides that the review asks them, nor which axis carries each,
and no plan document schedules the work.

## Details

### What

The statements, none of which a design head argues, each with what it bound:

- "The review the installed text owes before a merge asks, of the work it reads, two questions of
  meaning: whether each recorded item that bears on the work is cited where it bears and a checked
  reference may stand, and whether the work departs from the goals, the decisions and the reasons
  recorded beside its code and its agent-facing text that bear on it, or brings back a design the
  record did not keep." It bound the installed review skill to name both questions.
- "It is met while the installed text owes that review, both questions named, before every merge
  into the main branch, and asks in its retrospective how it fared." It bound the review skill to
  owe that review at every merge, and the retrospective skill to ask how the review fared.
- "the installed text, which requires a session to read what cites a decision before it reverses
  it, and what is outstanding and the evidence that would flip a decision before a change they
  bear on". It bound the installed text to require three readings. Two have a holder:
  `design@agent-skills@code-change-reads-its-record` holds the reading before a change to code,
  and `path@agent-skills@fragments/reversal-check.md` has a session search for the statement it
  replaces before a reversal. Two have none: the listing of what cites a decision, read before it
  is reversed, since that fragment names a search and no such listing, and the reading of what is
  outstanding before a change that is not to code.

Instances seen: none is recorded of a departure that passed a review before a merge. What is
measured is the absence in the text: `git grep -n 'cited where' -- crates/agent-skills/content/skills/review/`
returns nothing, and `grep -rn -i fared crates/agent-skills/content` returns nothing, so the
retrospective holds no question on how the review fared. The design-conformance axis reads a plan
document against the record, and at a merge only under a condition, and the code review axes judge
shape and claims.

The suspected mechanism, in one sentence: the two questions were stated as what the review asks
before any axis was argued to carry them, so the review's table of axes was never read against
them.

Re-entry point: the next design discussion on the axes of
`skill@knowledge-architect-review@review-axes`, or on a review of work against the project's goals
and its record at a merge or at a plan document.

### Why it matters

A departure from the record whose references all resolve passes each gate, so only a review
catches it, and no installed text has a review look for it, against
`goal@knowledge-architect@agents-work-without-drift`.
`goal@knowledge-architect@documentation-stays-consistent` gives reviews and audits the judging of
whether the code departs from what the documents state, and has the checker judge no meaning. An
item that bears on the work and is not cited where it bears does not reach the session working
there, against `goal@knowledge-architect@records-reach-their-reader`, and no check reports a
citation that is missing.

### What would close it

A design discussion under `skill@knowledge-architect-design` that argues, from a target project's
side as `path@agent-skills@CLAUDE.md` says, whether the review before a merge asks each question
and on which axis of its table, what a session reads before a reversal and before a change that is
not to code, and whether the retrospective asks how the review fared. Then a head for each
decision that earns one, and the installed text edited to it, or the recorded decision that a
question stays unasked, with its reason.
