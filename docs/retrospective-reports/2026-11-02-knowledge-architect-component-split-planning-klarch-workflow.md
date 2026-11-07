# 2026-11-02-knowledge-architect-component-split-planning-klarch-workflow

- **Received file**: 2026-11-02-knowledge-architect-component-split-planning-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-45-gb88cfa70`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`.

## W1: a gap the workflow's own plan schedules

Analysed with P1 of 2026-11-02-knowledge-architect-component-split-planning, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-split-planning.md`.

## W2: one discussion that converges on two plan documents

- **a.** What it says: `skill@knowledge-architect-planning@spec-contents` says a plan document
  "records the whole discussion", and arguments are numbered in one sequence across a milestone's
  documents. One discussion produced a spec and a milestone, which cannot cite each other's items.
  The session gave each document the arguments its threads use; 36 arguments sit in both, and a
  reviewer asked whether that is a duplicate. Routing 121 arguments took a script. Proposal: reword
  "records the whole discussion" to "records the threads it carries, with every argument they use".
  0 instructions.
- **b.** Whether it still applies: yes. `path@agent-skills@content/skills/planning/SKILL.md`, line
  242, "A plan document records the whole discussion", at `v0.9.0-45-gb88cfa70` and at HEAD.
- **c.** Whether it is already recorded: `design@agent-skills@plan-records-the-exchange`, "A plan
  document records the whole discussion", from the owner's proposal of a record "bounded to the spec
  document or the milestone directory", and `design@agent-skills@milestone-split-by-lifetime`, which
  splits a milestone's record across its documents. Neither names one discussion that converges on
  two plan documents. Items cannot be cited across plans, per `design@model@plan-item-scope`. No
  issue.
- **d.** Its kind: a defect of the text, unclear: for two plan documents, "the whole discussion" in
  each, or the threads each carries. Provable by reading.
- **e.** Its scope: installed text, the planning skill, and the head's title. The rewording is
  within the head's argument: the owner's proposal bounds the cross-referencing to one document, so
  each document needs every argument its threads use, and nothing more.
- **f.** A better fix: the file's rewording loses one property of the present text, that every
  thread of the discussion is recorded somewhere. A rewording that keeps it: "The plan documents of
  one discussion record the whole of it, each thread in the document that carries it, with every
  argument that thread uses." The file's rival, one document holding every argument and the other
  citing it whole, loses: an item cannot be cited from another plan, so the second document would
  point at no argument.
- **g.** Its route: an edit of the installed text, with the head brought to it under
  `skill@knowledge-architect-decision-recording`, as a member within its argument.
- **h.** The proposal: the rewording above, in the planning skill's assembly paragraph and in the
  statement of `design@agent-skills@plan-records-the-exchange`. Instructions: 0 added, 0 removed.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: a two-reading defect provable by reading;
  the rewording is within the argument of `design@agent-skills@plan-records-the-exchange` and keeps
  every thread recorded.

## C1: no command lists every text that cites a set of entries

- **a.** What it says: the planning skill asks, for each decision the work reverses or rewrites,
  "every text that `cargo klarch show` lists as referencing it"; the session ran `show` on 19 heads
  in a shell loop and grouped the files by hand. "The workflow's repository already holds an issue."
  Proposal: build that command.
- **c.** Whether it is already recorded: yes, `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
  kind `todo`, whose first work list is "every text citing an entry whose head changed", with two
  instances; this one, 19 heads, is not listed.
- **Default**: no change; the issue gains the instance.
- **Outcome**: decided by the session: no change; `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`
  gains this instance. Ground: the issue holds the need and its open shape.

## C3: `fork-context-ref`

Analysed with C2 of 2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow.md`.

## C4: nested quotes in a quoted reason

- **a.** What it says: may-bear reasons quoted in a plan document held double quotes inside double
  quotes, and the checker reported references inside the quotation; the session replaced the inner
  quotes with single ones. "unknown whether it is a defect or correct behaviour". What would
  establish it: a mock document with a nested quotation holding a reference.
- **b.** Whether it still applies, established: over a mock project made by `klarch init`, a line
  holding a double-quoted text with a double-quoted part inside it, the inner part holding a goal's
  slug as a plain word, gives "is written as a plain word, and names" the goal; the same line with
  single inner quotes gives nothing. This is the behaviour `design@check@quotation-read-by-no-id-lint`
  defines: "A quotation is text between `"` and `"` … within one paragraph". The inner opening mark
  closes the outer quotation, so the inner text is outside every quotation.
- **d.** Its kind: an observation, now established as the decided behaviour.
- **Default**: no change.
- **Outcome**: decided by the session: no change, the behaviour the head decides. Ground: the
  reproduction above; the single-quote repair the session used is the one the head admits.

## C2: no check that a plan document gives an outcome to every graded entry

Analysed with C3 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## The standing answers

- **a.** Items:
  - Installed file: no; the changes it found needed are W1 of this file, analysed by G5.
  - The five unchecked claims in plan documents and commit messages: analysed with the claim-check
    cluster of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow; the act
    attributed to the owner and the misquoted phrase also with the attribution cluster of
    2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.
  - A closed issue named by bare slug in backticks: the form `primer@where-knowledge-goes` admits
    for an entry the tree no longer holds. An observation.
  - Hand tasks: C1 is analysed by G4 in this file; C2 is cluster B (G1); W2 is analysed by G5 in
    this file.
  - Rules on Components: the six points of W1, analysed by G5. The two dev-dependency cycles in the
    first layouts, which reviewers caught, are a lapse the file names as such, analysed here.
- **d.** A lapse of rules the file calls "present and adequate"; caught before the merge.
- **Default**: no change.
- **Outcome**: decided by the session: no change for the items analysed here, a lapse and an
  observation. Ground: the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules`.

---
