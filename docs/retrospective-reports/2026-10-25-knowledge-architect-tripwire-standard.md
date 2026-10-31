# 2026-10-25-knowledge-architect-tripwire-standard

- **Received file**: 2026-10-25-knowledge-architect-tripwire-standard.md, the project file of this
  repository's session that wrote the tripwire standard.
- **Version used**: built from its own source, main at `v0.8.0-32-g6da4781`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**:
  `design@knowledge-architect@changelog-section-fixed-at-release`,
  `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument`,
  `design@core@section-homes-carry-slugs`, `design@agent-skills@shipped-text-cites-no-entry`,
  `design@agent-skills@shared-text-fragment`, `design@agent-skills@one-home-for-head-rules`.

## P1, with W2 of 2026-10-25-knowledge-architect-tripwire-standard-klarch-workflow: a slug cannot follow its section's subject

- **a.** The decision-recording skill's section `skill@knowledge-architect-decision-recording@ruled-tripwires` was retitled once the owner no
  longer rules on tripwires; its slug stayed, since a released changelog section cites it. The
  workflow file proposes an installed exception: a slug a released changelog cites stays, and the
  title carries the subject.
- **b.** Still applies: the slug rule is
  `path@agent-skills@content/skills/agent-configuration/SKILL.md`, line 105, unchanged; the section is
  "Recording a premortem's tripwires" at `path@agent-skills@content/skills/decision-recording/SKILL.md`, line 97;
  the 0.6.0 section of the changelog cites the slug.
- **c.** No issue. `design@core@section-homes-carry-slugs`: a rename of an installed slug dangles a
  consuming project's citations of it.
  `design@knowledge-architect@changelog-section-fixed-at-release` leaves a released section's
  reference rewritten in another form to the owner, case by case.
- **d.** An interaction between the installed rule and this repository's own changelog rule.
- **e.** The proposed exception is this repository's convention, written into the installed text
  every project reads, against `design@agent-skills@shipped-text-cites-no-entry`: the shipped text
  "names no Component and no convention of this repository".
- **f.** Rival: rename the slug, with a Migration entry for consuming projects and the released
  reference put to the owner. It weighs one misnamed slug against an interface change. The nearest
  rival to the proposal is no change; it wins, since a slug is an interface in every project, and a
  title that names the subject already tells a reader what the section holds.
- **g.** Nothing.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: the exception would put one repository's
  changelog convention into the installed text, and the slug's stability is the cost
  `design@core@section-homes-carry-slugs` already accepts for every installed slug.

## P2: a widened test detector recorded as a regression guard

- **a.** A review found that a test's detector missed `pub(crate) fn`; the session widened it and
  recorded it as "a regression guard widened", with no probe. The proposal: a test widened to new
  inputs owes the discriminating check for the inputs it adds.
- **b.** The text is unchanged: `path@agent-config@skills/klarch-development/SKILL.md`, lines 139-140, "A claim
  already settled by a probe beforehand enters as a regression guard". The event is in `87cbbdb5`.
- **c.** No record.
- **d.** A lapse: the widened part was no claim "already settled by a probe", so the text did not
  admit it as a guard.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: the instruction's words exclude the case,
  and a lapse is not repaired by adding text, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`.

## P3: the brief that reads a fragment in every host has no list of the hosts

- **a.** `path@agent-skills@CLAUDE.md` asks a review of a change to a fragment to name each host's
  installed copy in the brief. No text names the hosts; the session copied them from the literal of
  the test `each_fragment_has_its_hosts`. The proposal: the crate's `CLAUDE.md` names the test, or its
  failure message calls itself the list.
- **b.** Still applies: `path@agent-skills@CLAUDE.md`, lines 54-56, names no list. The design home has since
  named it: `design@agent-skills@one-home-for-head-rules` says the hosts test of the crate's
  `path@agent-skills@src/lib.rs` "holds the lists". `FRAGMENT_HOSTS` in `path@agent-skills@src/lib.rs` now lists 21
  fragments, where the report weighed two.
- **c.** No issue.
- **d.** Missing: a provable gap, an input the brief needs, fixed in the code.
- **e.** This repository's own text, `path@agent-skills@CLAUDE.md`.
- **f.** Rival: a command printing each fragment's hosts. Its premise, a list of two, is gone, but a
  pointer to the constant still serves the brief with no code. The failure message reaches only a
  session whose test fails.
- **g.** A text edit of the project's agent configuration,
  `skill@knowledge-architect-agent-configuration`. 0 instructions added: a pointer.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: the instruction needs an input that only
  the code holds, and a pointer to it fills the gap, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`.

## The notes for the intake

- **a.** The standing questions are in the workflow file. A general-purpose subagent was refused a
  write of its report.
- **b.** The issue named for that refusal is closed, by `67aa3a94`; the review skill already says a
  general-purpose subagent returns its report as text.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: already handled.
