# 2026-10-25-knowledge-architect-tripwire-standard-klarch-workflow

- **Received file**: 2026-10-25-knowledge-architect-tripwire-standard-klarch-workflow.md, the
  workflow file of this repository's session that wrote the tripwire standard.
- **Version used**: built from its own source, main at `v0.8.0-32-g6da4781`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**:
  `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`,
  `design@agent-skills@plan-records-the-exchange`, `design@agent-skills@attribution-replaces-quotation`,
  `design@agent-skills@transcript-extraction-is-the-checker-s`.

## W1: the harvest copies the owner's verbatim words from a plan document into heads and commit messages

- **a.** A plan document's Threads section keeps the owner's words verbatim; the harvest copied them
  into five new heads, three touched heads and seven commit messages, which three reviewers caught.
  The proposal: one sentence at the harvest step, that the thread's verbatim words become an
  attribution in the head and in the harvest's commit message, with the primer's section as its
  pointer.
- **b.** The planning skill's Threads row is unchanged
  (`path@agent-skills@content/skills/planning/SKILL.md`, line 167). Delivery has improved since: the
  decision-recording skill hosts the fragment holding the attribution rule, by `25301b7e`, and the
  harvest step sends to that skill. The proposed sentence exists nowhere.
- **c.** Recorded: a further instance of
  `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`,
  whose closing outcomes include "a single rule for both, stated where a session writes either". The
  rival "plan documents attribute too" is weighed by `design@agent-skills@plan-records-the-exchange`.
- **d.** A defect of the text, already recorded as a design issue.
- **f.** The proposal is one of the shapes the issue's discussion weighs; this file's count, at a
  larger scale, is its evidence.
- **Default**: the issue gains the instance.
- **Outcome**: decided by the session: the issue gains the instance and the proposal as a shape.
  Ground: the issue is the subject's home, and its discussion has not run.

## W2: the slug of the decision-recording skill's section on a premortem's tripwires

Analysed with P1 of 2026-10-25-knowledge-architect-tripwire-standard, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-tripwire-standard.md`.

## C1: no command extracts a transcript for a reviewer

- **b.** Repaired: `klarch transcript` landed in `1e77f453`, and the reviewer's text names it since
  `790e7f01`.
- **c.** Recorded: `design@agent-skills@transcript-extraction-is-the-checker-s`.
- **Default**: no change, already repaired.
- **Outcome**: decided by the session: no change, already repaired. Ground: the commits above.

## The standing answers

- **a.** The claim-check lapses, in commit messages and in answers to the owner; a premise the
  session supplied, on tripwires holding no ruling of the owner, contradicted by a searcher's report
  in front of the session; a head of absence and a rule each wider than the owner's word; and the
  development skill's probe skipped, which is P2 of the project file.
- **b.** Each rule exists at `6da4781` and at HEAD, in `primer@verify-before-relying`,
  `primer@owner-word-premise` and `primer@design-heads`.
- **d.** Lapses, caught before the merge. The claim-check lapses are analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-design-home-split-klarch-workflow.md`,
  section "The standing answers".
- **Default**: no change beyond the claim-check instance.
- **Outcome**: decided by the session: no change beyond the claim-check instance. Ground: lapses of
  clear instructions.
