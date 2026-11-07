# 2026-11-02-knowledge-architect-component-rules-revision

- **Received file**: 2026-11-02-knowledge-architect-component-rules-revision.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-51-gfef11af0`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-skills@the-agent-configuration-skill-names-no-place-for-a-local-uncommitted-instruction`.

## P1: the spec's harvest row and its decided design disagreed

- **a.** What it says: the spec's decided design said the module-directory exception applies "only to
  a cycle Cargo's rules force whatever the decisions"; its harvest row said it holds "where a crate
  of its own would force a publication or a cycle that no reversible decision causes". The session
  followed the row; the spec-conformity reviewer reported the departure; both were repaired to the
  decided design. No proposal on the project's text; whether plan reviewers should compare each
  harvest row with its decided design is left to the intake.
- **b.** Whether it still applies: the plan reviews ran the axis that reads it. The commits of the
  planning branch's reviews, 02346125 and 22b67f73, each list self-consistency among their axes. The
  axis is `skill@knowledge-architect-review@review-axes`: "does the result contradict itself. Two
  instructions a reader cannot both obey ... nearly all the time", and
  `skill@knowledge-architect-planning@plan-reviews` dispatches "every axis of its table whose
  condition holds". The exception itself is gone: 634b19e3 removed the module-directory Component.
- **c.** Whether it is already recorded: no entry.
- **d.** Its kind: no defect of the text: a miss of the self-consistency axis, twice, on a
  contradiction between two sections of one document, the axis's own question.
- **e.** Its scope: installed text (the reviews). Reverses nothing.
- **f.** A better fix: the rival is a readiness check, "every harvest row carries its decided design"
  in `skill@knowledge-architect-planning@plan-reviews`. It loses on one instance: the axis whose
  question it is already ran, and a second home for one check would be two homes for one moment.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: the self-consistency axis covers the case
  and ran, per 02346125 and 22b67f73; one instance of a miss.

## P2, with H1 of 2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow: the harness's cloud prompt said `gh` is absent, and the session believed it

- **a.** What it says: the session ran on the owner's machine through Remote Control, whose system
  prompt carried the block for cloud sessions, "You do NOT have access to the `gh` CLI". It skipped
  the draft pull request of `instructions@git-workflow` point 1 and told the owner it could not open
  one; `gh` was installed and authenticated. H1 names the harness defect, upstream issue 100759 of
  the anthropics/claude-code repository, and the workaround, a gitignored CLAUDE.local.md at the
  root. P2 classes the session's act as a lapse of `primer@verify-before-relying`. No proposal.
- **b.** Whether it still applies: the harness defect is outside this repository. The workaround is
  in place: the local file states that "A session here runs on the owner's own machine" and "Use
  `gh` for every GitHub operation", and this session's context holds it, so the harness loads it.
- **c.** Whether it is already recorded: `issue@agent-skills@the-agent-configuration-skill-names-no-place-for-a-local-uncommitted-instruction`,
  kind `todo`, holds this case as its own: the skill names no place for an instruction local to one
  machine; its closing is a design discussion or the owner's ruling. Still open.
- **d.** Its kind: P2, a lapse of `primer@verify-before-relying`, "Establish it from the
  implementation, or from a run you observe"; H1, an observation of a harness defect.
- **e.** Its scope: the harness, and for the local file, installed text, which the open issue holds.
- **g.** Its route: nothing beyond the issue.
- **h.** The proposal: no change; the issue already holds the case and its re-entry.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: already recorded in
  `issue@agent-skills@the-agent-configuration-skill-names-no-place-for-a-local-uncommitted-instruction`,
  whose case this is; the workaround is in place; the session's act is a lapse.
