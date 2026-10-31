# 2026-10-28-knowledge-architect-primer-terms-and-fix-grounding-klarch-workflow

- **Received file**: 2026-10-28-knowledge-architect-primer-terms-and-fix-grounding-klarch-workflow.md,
  the workflow file of this repository's session that added the primer's terms section, run in a
  cloud harness with a shallow clone.
- **Version used**: built from its own source, main at `v0.8.0-116-gde6ed98d`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**: `design@agent-skills@version-line-names-the-main-commit`,
  `design@agent-skills@tripwire-horizon`, `goal@agent-skills@installed-text-works-anywhere`.

## W1, with W1 of 2026-10-29-knowledge-architect-met-outside-upstream-route-klarch-workflow: a count over history in a shallow clone

- **a.** The retrospective's version line is taken with `git describe --tags`, which failed in a
  shallow clone; the session ran `git fetch --unshallow` and took it. In the 10-29 session, also a
  shallow clone, the standing-state reviewer found that the commits its horizon counts start from
  were absent; it cloned the repository again and found a horizon passing, which the shallow history
  would have hidden. The 10-28 file proposes one sentence after the version-line bullet; the 10-29
  file names the review invariants, which give each reviewer a worktree of the shared history.
- **b.** Still applies: `path@agent-skills@content/skills/retrospective/SKILL.md`, lines 35-38,
  `path@agent-skills@content/agents/standing-state-reviewer.md`, lines 69-72 and
  `path@agent-skills@content/skills/review/SKILL.md`, line 113 are unchanged, and no installed text names
  a shallow clone. The same count by `git log -S` is in
  `path@agent-skills@content/agents/tripwire-auditor.md`, line 102, which the saved audit workflows
  dispatch with worktrees of their own.
- **c.** No record. `design@agent-skills@tripwire-horizon` counts occasions by a command over the
  history, and states nothing on how complete the history is.
- **d.** Missing, with a default behaviour that is systematically wrong in one case: in a shallow
  clone, a horizon count finds no commit to start from and reports nothing, silently. The version
  line fails loudly, and the session repaired it by judgement.
- **e.** The installed text: the two agents that count horizons. The fix serves
  `goal@agent-skills@installed-text-works-anywhere`.
- **f.** Rival: one sentence in the review invariants, for the dispatcher. It misses the tripwire
  auditors the saved audit workflows dispatch. Rival: a sentence after the version-line bullet. The
  failure there is loud and the session recovered by judgement. Rival: the primer. Every session
  would load it for an occasion few meet, per `design@agent-skills@primer-limit`.
- **g.** The installed text at its source: one clause in each agent's horizon count, to run
  `git rev-parse --is-shallow-repository` and, where it prints `true`, `git fetch --unshallow` before
  counting. +2 instructions, 0 removed.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: a count that a shallow clone empties
  without a sign is a default systematically wrong, which the evidence of one real session shows, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`; the loud failure needs no text.

## C1: `klarch transcript` does not recognise three record types

Analysed with C1 of 2026-10-25-knowledge-architect-design-home-split-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-design-home-split-klarch-workflow.md`.

## The standing answers

- **a.** Two turns ended with no "Decided" section; six bare slugs were written, each caught by
  `check` or `commits`; claims in commit messages failed; three attributions were wider than the
  owner's rulings.
- **b.** The rules are unchanged. The slugs were caught by the checker, so they meet no entry; the
  wider attributions, caught by the transcript reviewer, are no finding per
  `skill@knowledge-architect-retrospective@what-it-examines`.
- **d.** Lapses. The claim-check lapses are analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-design-home-split-klarch-workflow.md`,
  section "The standing answers".
- **Default**: no change beyond the claim-check instance.
- **Outcome**: decided by the session: no change beyond the claim-check instance. Ground: lapses of
  clear instructions, the checker catching the slugs as designed.
