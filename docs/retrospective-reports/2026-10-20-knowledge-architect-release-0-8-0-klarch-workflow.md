# 2026-10-20-knowledge-architect-release-0-8-0-klarch-workflow

- **Received file**: 2026-10-20-knowledge-architect-release-0-8-0-klarch-workflow.md, the workflow
  file of this repository's session that released 0.8.0.
- **Version used**: built from its own source, main at `v0.7.0-84-g93558a0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`.
- **Standing entries that bear on it**: none beyond
  `design@core@checked-sentinel-values`, for the checker note.

## The installed files and the checker: no finding

- **a. What it says.** No finding on the installed files: the review skill's procedure for the
  record carried a late transcript review. No finding on the checker: a copy of a mock project
  needed its `checker-version` changed from the fixture sentinel to the version.
- **b. Whether it still applies.** The record procedure is
  `skill@knowledge-architect-review@what-review-leaves`, unchanged since `93558a0`: "A paragraph in
  the commit message records the review: the message of the last repair commit". The sentinel's
  refusal outside the tool's tree is `design@core@checked-sentinel-values`: "A copy of a mock
  project outside its library carries the version."
- **d. Its kind.** Observations: the text worked, and the checker behaved as its head says.
- **h. The proposal.** No change.
- **Default**: no change, observations.
- **Outcome**: accepted.

## The standing answers

- **a.** One rule of the primer acted on as if absent: a commit message carried a claim the session
  had not checked, taken from the changelog entry it repaired and from a reviewer's report. The
  session checked it after the commit and before the push, and narrowed the entry by a fixup folded
  into the repair commit.
- **b.** The rule is `primer@verify-before-relying`, "Verify a claim before writing it", unchanged.
  The narrowed entry is on main.
- **d.** A lapse, caught before the push. It is one more instance of the recurrence analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`,
  section "The standing answers", which proposes the outcome for the whole recurrence. The project
  skill's changelog axis was dispatched, as the answer says. The behavioural claim that no axis
  checked is P1 of 2026-10-20-knowledge-architect-release-0-8-0.
- The ruling is given where the recurrence is analysed.
