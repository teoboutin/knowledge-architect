# 2026-11-02-thaum-count-ceilings-slice-klarch-workflow

- **Received file**: 2026-11-02-thaum-count-ceilings-slice-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: knowledge-architect 0.9.0, the version the project pins.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@check@a-plain-hash-item-name-is-read-by-nothing`.

## W1: two homes say how a commit message names a plan item, and they disagree

- **a.** What it says: `skill@knowledge-architect-planning@plans-layout`: "From anywhere else, a
  commit message and a design head included, the plan document is cited whole, and an item is
  named in plain text with a hash sign". `primer@where-knowledge-goes`: "A finding is repaired in a
  form the checker judges, never by moving the pointer into plain text", and its rule on what a
  record of a deliberation names by its bare slug. The transcript reviewer, reading the primer, said
  twice that each such plain name of an item the tree holds owes its reference; the checker refused
  the reference in a commit message and its repair text asked for the plain form; the session kept
  the plain form; the decision-record reviewer cited the planning skill. Cost: a repair promised to
  the owner and not made, two items argued, one wrong "repaired" claim. Proposal: the primer's
  sentence on records of a deliberation gains the plan-item case as one clause; the planning
  skill's two sentences become pointers to it. +1 clause, −2 sentences.
- **b.** Whether it still applies: yes. The planning skill, lines 231 and 232 and line 346 of
  `path@agent-skills@content/skills/planning/SKILL.md`, and the primer, line 422 of
  `path@agent-skills@content/PRIMER.md` (line 412 at `v0.9.0`), read the same at `v0.9.0` and at
  HEAD. The primer names no plan item: a search of it for "hash sign" and for the item form finds
  nothing.
- **c.** Whether it is already recorded: the reconciliation is in the record, not in the primer.
  `design@agent-skills@plain-text-is-no-repair`: "A label beside a checked pointer is not such a
  form. A plan item named outside its plan as" a hash sign and its id "stands beside a citation of the
  whole plan, because an item reference is refused there, per `design@model@plan-item-scope`".
  `issue@check@a-plain-hash-item-name-is-read-by-nothing`, kind `design`, holds that no check reads
  that plain form; its re-entry is the next change to `design@check@retired-slug-reference-reported`
  or the owner's word, which this fix does not meet.
- **d.** Its kind: a defect of the text, wrong: two instructions a reader cannot both obey, the
  primer's general rule against the planning skill's exception and the checker's repair text,
  provable by reading.
- **e.** Its scope: installed text, the primer and the planning skill. The fix states in the primer
  what `design@agent-skills@plain-text-is-no-repair` already decides; it reverses no head and does
  not decide the open issue, which is whether the plain form is checked.
- **f.** A better fix: the nearest rival is the exception in the transcript reviewer's definition.
  It loses: every reviewer that reads commit messages loads the primer, and the decision-record
  reviewer, which also judges messages, cited the other home; one home where all of them read is the
  cause repaired. The planning skill keeps its sentence at the act, cut to one sentence with its
  pointer, as `primer@where-knowledge-goes` allows a one-sentence restatement.
- **g.** Its route: an edit of the installed text at its source.
- **h.** The proposal: one clause in the primer's paragraph on a finding's repair: a plan item named
  outside its plan is a label, a hash sign and its id beside a citation of the whole plan, not a
  pointer moved into plain text. The planning skill's two sentences each keep the form and point to
  that clause. Instructions: 1 added, 1 removed (the planning skill's landing sentence becomes a
  restatement with its pointer), net 0.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: a contradiction provable by reading; the
  clause restates `design@agent-skills@plain-text-is-no-repair` where every reviewer reads it.

## C1: saving a subagent's report whole means emitting it again by hand

Analysed with W1 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.

## C2: a deleted plan document's citations re-pointed by hand

Analysed with C2 of 2026-11-01-thaum-design-record-audit-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-01-thaum-design-record-audit-klarch-workflow.md`.

## The standing answers

- **a.** Items:
  - Installed file: no; the diff over the installed files is empty. An observation.
  - A default recorded as "approved" by an answer that did not name it: analysed with the
    attribution cluster of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.
  - A reviewer proposing that a plan document name a transcript file, because the document's older
    sources did, and the session following it: analysed here.
  - The two claims in messages: analysed with the claim-check cluster of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - No pointer written bare.
  - Hand tasks: C1 is cluster D (G3); C2 is cluster G (G4); the reword by a cherry-pick sequence is
    cluster H (G6).
- **b.** The instruction stood at 0.9.0 and stands at HEAD,
  `path@agent-skills@content/skills/planning/SKILL.md`, line 254: "the document names no file of
  the transcript directory". The rule the reviewer broke is `primer@verify-before-relying`,
  "Precedent is not authority".
- **d.** A lapse, by a reviewer and then by the session; a later reviewer caught it before the
  merge.
- **Default**: no change.
- **Outcome**: decided by the session: no change, a lapse. Ground: the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules`.

---
