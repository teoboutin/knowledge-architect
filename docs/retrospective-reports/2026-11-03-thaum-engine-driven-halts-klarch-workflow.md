# 2026-11-03-thaum-engine-driven-halts-klarch-workflow

- **Received file**: 2026-11-03-thaum-engine-driven-halts-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: knowledge-architect 0.9.0, the version the project pins.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: none beyond those its sections cite.

## W1: a harvest row mapping several threads to one head, unchecked before the merge

Analysed with W2 of 2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow.md`.

## W2 and C2: the searchers' reasons are not in checked forms, and the "not read" lines are built by a script

Analysed with C3 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## W3: saving each review report "as it arrives" when the reviewers run in one saved workflow

Analysed with W1 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.

## C1: `transcript list --branch` finds no session for work done in a worktree

- **a.** What it says: the whole branch was worked on from a worktree; the session's branch fields
  read the main checkout's branch and another branch; `--branch <branch>` printed its header alone,
  exit 1, and the reviewer fell back to `--containing`. The review skill tells the dispatcher to use
  `--branch`.
- **b.** Whether it still applies: yes. `design@transcript@transcript-query-verbs`: "`list --branch`
  computes the sessions that worked on a branch from the `gitBranch` field every chain line
  carries"; `path@transcript@src/reader.rs`, line 649, reads that field, and no commit since
  `v0.9.0` changed it (`git log -S gitBranch v0.9.0..HEAD` returns nothing). Measured over this
  machine's transcripts, the top-level lines whose `cwd` lies in a linked worktree mostly carry
  the main checkout's branch: in this repository, 22 lines in one worktree carry the branch the main
  checkout held, and in another project's agent worktree 47 lines carry `main` and 23 the worktree's
  branch. So the field names the branch of the session's start, not always the branch worked on.
  That reading of the harness is an inference from these counts, not from its documentation.
- **c.** Whether it is already recorded: no. `design@transcript@transcript-directory-resolution`
  makes a worktree's sessions the project's, which is a different question.
- **d.** Its kind: wrong: the head's premise, that the field names the branch a session worked on,
  fails for work in a worktree.
- **e.** Its scope: `goal@transcript@a-session-reads-past-sessions`; the review skill's
  `--branch` sentence, `path@agent-skills@content/skills/review/SKILL.md`, line 73. Owner
  `transcript`. A fix changes what the head says `--branch` computes, so it goes to
  `skill@knowledge-architect-design`.
- **f.** A better fix: the rivals are `--branch` also matching a line whose `cwd` is a worktree that
  has the branch checked out, which git can no longer tell once the worktree is removed; or the
  installed text naming `--containing` as the route where the work ran in a worktree. What decides
  is not established here: it needs a measurement of which harness events write the worktree's
  branch, which is the discussion's.
- **g.** Its route: an issue.
- **h.** The proposal: open an issue in `transcript`, kind `defect`, id
  `list-branch-misses-a-session-that-worked-in-a-worktree`.
  - What: a session that worked on a branch from a linked worktree carries, in most of its lines,
    the main checkout's branch, so `list --branch` misses it; the counts above.
  - Why it matters: the review skill finds the sessions of a branch with `--branch`, so the
    transcript review can miss a session, against `goal@transcript@a-session-reads-past-sessions`.
  - What would close it: a discussion that measures the field over worktree sessions and either
    widens what `--branch` matches or has the review skill name `--containing` for that case,
    rewriting `design@transcript@transcript-query-verbs`, with a test over a fixture.
- **Default**: open the issue.
- **Outcome**: decided by the session: open `list-branch-misses-a-session-that-worked-in-a-worktree`
  in `transcript`, kind `defect`. Ground: the measurement above contradicts the head's premise; no
  record holds it.

## The standing answers

- **a.** Items:
  - Installed file: no. An observation.
  - The wrong count and quotation in the spec's first draft, and the repair message: analysed with
    the claim-check cluster of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - One decision per head: W1 of this file, analysed by G5 with W2 of
    2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow.
  - Two brief additions of the project's own review skill missed: analysed here.
  - No pointer written bare.
  - Hand tasks: C2 is cluster B (G1); W3 is cluster D (G3); W2, the rewritten references in the
    pasted reasons, is cluster B (G1).
- **e.** The brief additions are the other project's own skill: out of this repository's scope,
  which `skill@klarch-retrospective-intake@what-it-takes` leaves to that project ("Its own findings
  are that project's, and are fixed in its text"). The file itself calls them slips.
- **Default**: no change.
- **Outcome**: decided by the session: no change, out of scope. Ground: a lapse on the other
  project's own text is that project's.

---
