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

A lead the owner named bears on the second question: a review axis that reads the work against the
project's goals at a merge or at a plan document, with the collection there of evidence of whether
the goals are met. The owner left it open, judged that it can never be met mechanically, and named
it, beside an audit axis on whether the goals are met, as the subject of the discussion that takes
it up again; the audit lead is `issue@agent-skills@audit-axes-beyond-the-design-record`, which
holds what that discussion must respect. Its cause: the goals carry no measure and fewer details,
per `design@agent-skills@what-shows-a-goal-met-is-design`, so grounding in the goals and the record
search may hold work to them less well than before. What a project builds to collect evidence of a
goal met is a design decision that no target project has taken:
`issue@agent-skills@a-target-project-collects-no-feedback-from-use`. Today the design-conformance
reviewer reads a plan document against the goals, per
`design@agent-skills@plan-read-against-the-record`, and at a merge only under the condition of
`design@agent-skills@verdicts-reach-the-conformance-review`; the goals axis before a merge reads
the goals a branch changes, not the work against them, per
`design@agent-skills@goals-axis-in-the-review-before-a-merge`.

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
`skill@knowledge-architect-review@review-axes`, or the design discussion, held on the owner's word,
on a review of work against the project's goals and its record at a merge or at a plan document.
The evidence that reopens the lead on the goals sooner: a retrospective's or a review's finding of
merged work that moved against a goal with no procedure reporting it.

### Why it matters

A departure from the record whose references all resolve passes each gate, so only a review
catches it, and no installed text has a review look for it, against
`goal@agents-work-without-drift`.
`goal@documentation-stays-consistent` gives reviews and audits the judging of
whether the code departs from what the documents state, and has the checker judge no meaning. An
item that bears on the work and is not cited where it bears does not reach the session working
there, against `goal@records-reach-their-reader`, and no check reports a
citation that is missing.

### What would close it

A design discussion under `skill@knowledge-architect-design` that argues, from a target project's
side as `path@agent-skills@CLAUDE.md` says, whether the review before a merge asks each question
and on which axis of its table, whether a review reads the work against the goals and collects
evidence of them at a merge or a plan document, what a session reads before a reversal and before a change that is
not to code, and whether the retrospective asks how the review fared. Then a head for each
decision that earns one, and the installed text edited to it, or the recorded decision that a
question stays unasked, with its reason.
