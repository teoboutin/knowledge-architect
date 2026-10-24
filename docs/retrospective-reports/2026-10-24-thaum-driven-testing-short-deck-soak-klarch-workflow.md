# 2026-10-24-thaum-driven-testing-short-deck-soak-klarch-workflow

- **Received file**: 2026-10-24-thaum-driven-testing-short-deck-soak-klarch-workflow.md, the workflow
  file of a session in thaum that worked one milestone slice from its design audit, stopped on a
  load-bearing gap the owner ruled, to its merge, compacted once.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`; the decision-recording and planning skills, the
  primer and the checker's messages are identical at both.
- **Standing entries that bear on it**:
  - for W1: `design@agent-skills@verification-in-primer`;
  - for C1: `design@core@plan-item-scope`, `issue@core@a-plain-hash-item-name-is-read-by-nothing`,
    whose re-entry point is the next change to that head; `rejected@core@cross-plan-item-citation`,
    not revived, since the repair keeps the whole-plan citation; `design@agent-skills@plain-text-is-no-repair`,
    which admits the label; `tripwire@core@item-of-another-plan-named`, not met: the sites are a code
    comment and commit messages, not a plan's prose;
  - for C2 and C3: see the clusters they point to.

## W1. The reversal check's search for the behaviour's wording missed a phrase split by emphasis marks

- **a. What it says.** The session reversed a head and searched for the phrase that described the
  reversed behaviour; one document wrote it with one word in italics, so the search missed it, and
  that document kept describing the reversed design as current. Three reviewers found it. The fix
  proposed: the reversal check's sentence gains ", allowing for emphasis marks and line breaks inside
  the phrase".
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-decision-recording@reversal-check`:
  "Grep the behaviour's wording as well as the slug, since a pointer that describes the behaviour
  without naming it is the one a slug grep misses." `primer@verify-before-relying`: "so that they
  match across a line break, since wrapped prose splits a phrase over two lines". The line-break rule
  is stated in other words in the planning skill and the intake skill too; no installed text names
  inline markup.
- **c. Whether it is already recorded.** No. `design@agent-skills@verification-in-primer` keeps the
  verification rules in the primer, and a skill only the narrower rule its moment needs.
- **d. Its kind.** Missing, seen in real use: the same mechanism as a line break, a phrase split by
  something a search does not skip, which no text names.
- **e. Its scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`; installed text.
- **f. A better fix.** Rivals: the clause at the reversal check, a fourth wording of one rule; the
  primer's sentence widened to name emphasis marks beside the line break, which every phrase search
  reads, the reversal check included. The second keeps one home for the rule. Instructions added: 0, a
  rewording of how an existing instruction is met.
- **g. Its route.** An installed text edit of the primer.
- **h. The proposal.** Handle now: the primer's sentence says a search matches across a line break and
  across emphasis marks, since wrapped prose and inline markup split a phrase.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C1. The repair line for a plan item cited from outside its plan does not name the `#<id>` form

- **a. What it says.** Three times on one branch, the session cited an acceptance criterion of the
  milestone from outside it, in a code comment and in two commit messages. The checker's repair names
  only a citation of the whole plan, and the planning skill's form for the item, "#<id>", appears in no
  repair; the third time the session named the item in words. The fix proposed: the repair reads "cite
  the plan whole, as `milestone@plans@<milestone>`, and name the item in plain text as #<id>".
- **b. Whether it still applies.** Yes. `path@core@src/check/references.rs`: "cites an item of the plan
  … from outside it" with the repair "cite the plan whole, as `{form}`; an item is cited only from
  inside its own plan, so retiring the plan asks no other document to be redesigned". The planning
  skill: "naming it as #<id> beside a citation of the whole plan".
- **c. Whether it is already recorded.** `design@core@plan-item-scope`: "the repair names the document
  to cite whole". `issue@core@a-plain-hash-item-name-is-read-by-nothing` records that the prescribed
  form is one the checker never reads; its re-entry is the next change to that head, which this repair
  makes. `design@agent-skills@plain-text-is-no-repair` admits the form: "A plan item named outside its
  plan as #<id> stands beside a citation of the whole plan".
- **d. Its kind.** A lapse that recurred three times at the moment of the finding: the rule is in the
  planning skill and needed where the finding is read, so the repair line is its point of delivery.
- **e. Its scope.** `goal@knowledge-architect@adoption-is-easy`; the checker, and the head whose repair
  sentence gains the label.
- **f. A better fix.** Rivals: the proposal; no change, which leaves the lapse at its point of
  delivery. The head stands on its argument, and the label beside a whole-plan citation is the form
  its argument and the planning skill already give, so the head's body gains it directly. The change
  is the open issue's re-entry, so its question is read in the same branch.
- **g. Its route.** A change to the Rust source, under `skill@klarch-development`.
- **h. The proposal.** Handle now: the repair line as proposed, the head's repair sentence brought to
  it, and the open issue read at its re-entry.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C2. A commit message cannot be judged before the commit exists

This finding is analysed with W4 and C1 of 2026-10-22-rootpoint-live-control-slice-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-22-rootpoint-live-control-slice-klarch-workflow.md`.

## C3. The plain-word repair for a plan's name, a recurrence

This finding is analysed with C1 of 2026-10-20-thaum-pin-move-0-8-0-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-20-thaum-pin-move-0-8-0-klarch-workflow.md`.

## The standing answers

- **a.** Rules acted on as if absent: at the design audit, the session replaced a target an acceptance
  criterion names and reported it under "Decided", where it narrowed what the criterion judges and was
  a default for the owner, which a reviewer found and the owner approved; the audit's turn reported
  "Met outside the task: none" while its probe had met two recorded defects and a gap, listed under
  "Decided" only; five commit message claims false when written, four caught by reviewers, the fifth
  on the main branch and true after its commit; a push chained after `commits` with `;`.
- **b.** The rules stand: `primer@question-authority`, with point 2 of working a slice on a default
  awaiting the owner; `primer@met-outside-the-task`; `primer@verify-before-relying`. The `;` case is
  stated by this repository's `design@knowledge-architect@push-after-the-commits-check`; the installed
  primer names the pipe only.
- **d.** Lapses, caught before the merge. The claims are instances of the recurrence analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`;
  the chained push is weighed with P1 of 2026-10-20-knowledge-architect-defined-ids-lints, whose
  setting does not cover `;`. The narrowed criterion and the unreported items are single instances of
  clear instructions.
- **h.** No change for those two.
- **Default**: no change, as h says.
- **Outcome**: accepted.
