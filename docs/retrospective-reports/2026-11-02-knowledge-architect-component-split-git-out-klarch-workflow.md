# 2026-11-02-knowledge-architect-component-split-git-out-klarch-workflow

- **Received file**: 2026-11-02-knowledge-architect-component-split-git-out-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-59-g5c7bc807`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`, `issue@git@tools-run-git-past-the-wrapper`.

## W1: the design audit reads the assembled files whole, and has no outcome for an entry it could not read

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## C1: the audit's record, one line per graded entry with current references

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## The standing answer on a searcher's report filled by a heredoc

Analysed with C2 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## W2: the draft-mode review of goals loops

Analysed with W1 of 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md`.

## W3: the goal standard has no rule against naming what is built, or citing entries other than goals

- **a.** The owner ruled, drafting a goal, that a goal naming a part as built sets a means as an
  end, and that a goal cites no entry but goals. The reviewer reported neither until the ruling was
  in its brief. Blamed: `skill@knowledge-architect-goal-setting@goal-or-not` rule 1, which said only
  "Its exact form, a command's name, a flag or a signature, is a decision". Proposed: a sentence in
  rule 1, where the reviewer reads it.
- **b.** Repaired. At `5c7bc807` (`v0.9.0-59`), `path@agent-skills@fragments/goal-standard.md`, rule
  1 held no such sentence and rule 7 read "A refinement neither widens nor contradicts the goal it
  refines." Commit `c5131ca1` (`v0.9.0-86`) added both. At HEAD, the fragment's line 26: "it names
  no part whose existence comes from a design choice"; line 85: "and a goal cites only goals and
  registers of its own Component, or, in a location's goals home, of that location, or of a
  Component enclosing it". The fragment is hosted by the skill and by
  `agent@knowledge-architect-goal-reviewer@the-goal-standard`, and the reviewer's grade table has
  the rows "the goal names a part whose existence comes from a design choice" and "a citation rule 7
  refuses", both `blocks`. The checker reports any other reference in a goals home.
- **c.** Recorded: `design@agent-skills@goal-outcome-survives-empty-skeleton` (rule 1),
  `design@agent-skills@goal-standard-rules` (rule 7) and `design@check@goals-home-cites-upward-only`.
- **d.** Defect of the text, missing, repaired.
- **Default**: no change, already repaired.
- **Outcome**: decided by the session: no change, already repaired. Ground: commit `c5131ca1`, and
  the two rules as they stand in `path@agent-skills@fragments/goal-standard.md`.

## W4: the rules on Components

Analysed with P4 of 2026-11-02-knowledge-architect-component-split-git-out, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-split-git-out.md`.

## W5: what a review owes after the transcript review, when its repair makes another axis owed

Analysed with W2 of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.md`.

## C2: a register home's "No entry yet." goes stale

- **a.** What it says: `new-component` writes "No entry yet." into each home; a new Component's
  issue home kept it over four entries until a reviewer found it.
- **c.** Whether it is already recorded: yes, `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`,
  kind `todo`, which reports this instance and names its two closing shapes.
- **Default**: no change.
- **Outcome**: decided by the session: no change, already recorded with this instance. Ground: the
  issue's What is this file's case.

## C3: `fork-context-ref`

Analysed with C2 of 2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow.md`.

## The standing answers

- **a.** Items:
  - Installed file: no. An observation.
  - The search claim in a commit message and the defect's effect in an issue entry: analysed with
    the claim-check cluster of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - The owner quoted in two commit messages: analysed with the attribution cluster of
    2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.
  - The searcher's heredoc and the report fills: cluster L (G1).
  - No pointer written bare.
  - Hand tasks: C1 is cluster A (G1), home W1 of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow. The script listing each
    process start of git in library code, run as a claim's test, is analysed here.
  - Rules on Components: W4 of this file, analysed by G5.
- **b.** The measurement already has a recorded home:
  `issue@git@tools-run-git-past-the-wrapper` lists the code outside the git crate that starts git,
  "found with" a `grep -rln` on the spawn of git, and its closing condition is that each site moves
  onto the library or keeps a comment saying why it stays.
- **d.** An observation: the claim's evidence was a one-off script because the crate has no test of
  that claim, which is a matter of `git`'s code, not of the workflow.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: the claim belongs to `git`, whose issue
  already holds the command that measures it; no workflow step asks for the script.

---
