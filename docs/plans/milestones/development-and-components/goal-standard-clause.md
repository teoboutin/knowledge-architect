# Slice 0: the goal standard admits an outcome judged only in use, and the code-quality goal is reworded

Worked on the branch that adds this milestone's documents, before their reviews, on the owner's R7,
as the milestone document's "How the work is done" says.

## Builds

1. **The new clause of rule 6** in `path@agent-skills@fragments/goal-standard.md`, in its first
   reading, worded as #goal-standard-measure's decided design below, with its `%%` lines.
2. **The `intent` tag** in `agent@knowledge-architect-goal-reviewer@what-to-report`: a finding under the new clause, a goal judged only in use measured by a list of
   the properties its outcome is taken to consist of, is tagged `intent` as well as its rule, per
   #rule-6-finding-weight.
3. **The heads rewritten in place** under `skill@knowledge-architect-decision-recording`:
   `design@agent-skills@goal-standard-rules` and `design@agent-skills@goal-reviewer-agent`.
4. **The code-quality goal reworded and renamed** toward the slug `the-workflow-leads-to-better-code`,
   under `skill@knowledge-architect-goal-setting`: a draft in the conversation, the goal reviewer in
   draft mode reading the new fragment from disk, then the owner's ruling. Every reference to the old
   slug is re-pointed, this milestone's documents included.
5. **`issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`** updated or
   closed, per #rule-6-finding-weight, by `skill@knowledge-architect-issue-tracking`.
6. **The re-entry of `issue@agent-skills@heads-argued-from-another-harness`** becomes "the next
   design discussion that changes how fragments are delivered", per #harness-argued-heads-scope.
7. **`cargo klarch install-agent-skills`** run, the CHANGELOG.md entries written in `Next release`,
   and `cargo x changelog` run.

## Claims

- **The installed goal-setting skill and the installed goal reviewer each carry the new clause.**
  Test: after the install, read rule 6 in
  `path@agent-config@skills/knowledge-architect-goal-setting/SKILL.md` and in
  `path@agent-config@agents/knowledge-architect-goal-reviewer.md`. Shown to fail: the
  same reading on the tree before the edit finds no sentence on an outcome judged only in use.
- **The goal reviewer tags a finding under the new clause `intent`.** Test: read the report section
  of the installed agent. Shown to fail: the section before the edit tags every rule-6 finding
  `minor` only. No run of the reviewer is built to observe it, per
  `design@agent-skills@synthetic-evidence-not-built`; the goal reviewer's draft-mode run on the
  reworded goal, item 4, is the work's own use of it.
- **No reference to the old slug of the goal remains.** Test: `cargo klarch check` passes, and a
  search over the tree for the old slug returns only the lines that name the renamed entry by
  its old slug beside its new reference. Shown to fail: the
  check before the re-pointing reports every dangling reference.
- **The two heads state the clause and the tag.** Test: the decision-record axis of the review
  before the merge. Shown to fail: the heads before the edit say rule 6's findings are `minor` only.

## Audit subjects

- `path@agent-skills@fragments/goal-standard.md`, rule 6 and its `%%` lines; rule 2, "It has one
  reading", which the new clause must not weaken.
- `agent@knowledge-architect-goal-reviewer@the-goal-standard`, `agent@knowledge-architect-goal-reviewer@the-modes`
  and `agent@knowledge-architect-goal-reviewer@what-to-report`.
- `skill@knowledge-architect-goal-setting`, read whole before the draft, in particular
  `skill@knowledge-architect-goal-setting@goal-or-not`,
  `skill@knowledge-architect-goal-setting@drawing-out-intent` and
  `skill@knowledge-architect-goal-setting@goal-entry-shape`.
- `design@agent-skills@goal-standard-rules`, `design@agent-skills@goal-reviewer-agent`,
  `design@agent-skills@goal-entry-shape`, `design@agent-skills@goal-draft-shows-reach`,
  `design@agent-skills@goal-edit-ruled-alone`, `design@agent-skills@goals-axis-in-the-review-before-a-merge`.
- The goal and the texts citing it, as "What is already decided" lists them; the goal it refines,
  `goal@knowledge-architect@agents-get-a-complete-workflow`, and
  `goal@knowledge-architect@the-workflow-improves-through-real-use`, which the mechanism of the
  reworded goal leans on.
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: the goal reviewer cites rules by
  number in words.

## Fails alone on

- The goal reviewer's tag of a goal measured by a property list: missing, or applied to a rule-6
  finding outside the new clause.
- The reworded goal: a reading the owner did not mean, which the goal reviewer in merge mode or the
  owner reports.

## Premises that expire

- **The milestone's documents cite the goal by its old slug** until item 4 lands; item 4 re-points
  them. `cargo klarch check` is the guard.
- **Slice 2's code-shape reviewer is written against the reworded goal.** If the owner rules a
  wording that keeps the three properties as requirements of every edited unit, the decided design of
  #code-shape-axis in `spec@development-and-components@development-skill` is read against it at that
  slice's audit.

## Decided design

### The new clause of rule 6, first reading

Thread: `thread@development-and-components@goal-standard-measure`.

**Shape.** Rule 6 gains the clause of `argument@development-and-components@arg-45`, as the agent
worded it in the reply to R4:

> Where the outcome is judged only in use, the observable is what the project builds to pursue it
> and to learn from its use whether it is met: such as a test of a tool under the conditions of its
> use, a route by which its users' reports reach the project, or, for instructions to agents, a review
> that weighs them and a retrospective that asks how they fared. It is not a list of the properties
> the outcome is taken to consist of.

Its final wording is the slice's, within that sense, held to the agent-facing text standard; a
rewording is reported under "Decided". The rest of rule 6 stands: the measure stays in the tree, and
evidence from use shows the goal unmet in practice.

**Argument.** Rule 6 already allows the tree to show the mechanism that pursues an outcome and
corrects itself from use, but never says so, so a drafter reaches for the most specific observable,
a list of properties, which rule 2 then hardens (`argument@development-and-components@arg-31`). The
code-quality goal is that case (`argument@development-and-components@arg-27`). The clause adds to
rule 6 and reverses nothing, since it keeps the premise that a measure on use has limited
feasibility.

**Nearest rival and the fact that defeated it.** The second reading, the evidence from use itself as
the measure where it can be gathered, would reverse part of rule 6's premise; the owner chose the
first reading in R5: "I think that this is the better reading of both, because it has a systematic
method of checking it". An issue on the goal standard instead of a fix now lost because a draft
written under the old standard "gets pushed back toward the list" (`argument@development-and-components@arg-31`),
and the owner agreed "to fix in in this branch", R4.

### A finding under the new clause is also tagged intent

Thread: `thread@development-and-components@rule-6-finding-weight`.

**Shape.** In the goal reviewer's report, a finding under the new clause, a goal judged only in use
whose measure is a list of the properties its outcome is taken to consist of, carries the tag
`intent` beside its rule and its `minor` tag. Other rule-6 findings keep `minor` alone.

**Argument.** `design@agent-skills@goal-standard-rules` tags every rule-6 finding `minor`, so the
narrowing of the code-quality goal would have reached the owner as a minor note, which is the
mechanism `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect` records:
"a reach shown as a minor measure finding does not reach the owner as a question of intent"
(`argument@development-and-components@arg-60`).

**Nearest rival and the fact that defeated it.** Leaving the tag `minor` alone: it is the issue's
recorded mechanism, whose re-entry this work meets.

### The code-quality goal reworded, toward the slug the-workflow-leads-to-better-code

Thread: `thread@development-and-components@code-quality-goal-wording`.

**Shape.** The direction the owner approved: a vague intent, better code from the workflow in target
projects; its observable the mechanism under the new clause, the default shape the installed text
states, a review that weighs it, and the retrospective's question of
#retrospective-code-rules-question; the two-layer testability property kept as the most concrete;
the three properties a default shape the review weighs, not requirements of each edited unit; a
departure argued when the review reports it, not at every edited unit; the instructions growing from
real use (`argument@development-and-components@arg-18`). The slug moves toward
`the-workflow-leads-to-better-code`. The wording is drafted under goal-setting and is the owner's to
rule.

**Argument.** The departure clause leaves a one-line fix in a 300-line function two compliant moves,
a justification with no real argument or a refactor nobody asked for, both misaligned
(`argument@development-and-components@arg-18`); the owner's R2: "Rules too strict tend to make agents
take considerably misaligned actions when following them becomes impossible." The old slug "looks
like it applies to the project itself, not to the shipped skills" (R3).

**Nearest rival and the fact that defeated it.** Keeping the goal as written: it is the clause above.

### The re-entry of the harness issue narrows

Thread: `thread@development-and-components@harness-argued-heads-scope`.

**Shape.** `issue@agent-skills@heads-argued-from-another-harness` stays open; its re-entry becomes
"the next design discussion that changes how fragments are delivered".

**Argument.** The work adds a fragment and edits one, and changes no delivery of fragments; the owner
ruled Q4 in R6.

**Nearest rival and the fact that defeated it.** Arguing the issue's five heads again in this
milestone: the owner's R6, "there is no point handling it".

## Harvest

- **Heads:** `design@agent-skills@goal-standard-rules` and `design@agent-skills@goal-reviewer-agent`
  rewritten in place, under `skill@knowledge-architect-decision-recording`.
- **Goals:** the reworded goal in `register@knowledge-architect@goal`, on the owner's ruling.
- **Rejected:** the second reading of the new clause, if
  `design@agent-skills@losing-alternatives-filter` admits it, in `register@agent-skills@rejected`.
- **Issues:** `issue@agent-skills@a-goal-the-draft-review-read-was-reworded-again-for-a-defect`
  updated or closed; `issue@agent-skills@heads-argued-from-another-harness` re-entry changed.
- **Tripwires:** premortem cause 5 of the milestone document, judged by the tripwire standard.
