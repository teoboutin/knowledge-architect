# 2026-11-25-knowledge-architect-tmp-cleanup

- **Received file**: 2026-11-25-knowledge-architect-tmp-cleanup.md, the project file of a retrospective of this repository, on the branch `tmp-cleanup`.
- **Version used**: "**Version used**: main at `147a95dd` (short) when the work began, the tree clean; knowledge-architect `v0.10.0-237-g147a95dd`."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@knowledge-architect@scratch-removed-by-a-guard`: constrains, respect.
  - `design@knowledge-architect@nothing-sweeps-a-killed-run`: constrains, respect.
  - `rejected@knowledge-architect@mod-tempdir-for-its-sweep`: constrains, respect.
  - `rejected@knowledge-architect@testdir-for-test-directories`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.

## test-tmp-leftovers: what the tests left in the temporary directory before the branch has no cleaning home

Members: 2026-11-25-knowledge-architect-tmp-cleanup P2.

- **a. What it says.** About 1.1 GB of copies made before the branch stay in `target/tmp`, and
  leftovers stay in `/tmp`. The branch stops the growth and removes nothing. The file proposes no
  change: the head the branch wrote argues it, and a removal by hand is one command.
- **b. Whether it still applies.** Yes, and as the record intends. On main,
  `design@knowledge-architect@nothing-sweeps-a-killed-run` (in `register@knowledge-architect@design`,
  its first paragraph): "The directories that the tests of a killed run made stay in the temporary
  directory, where their prefix finds them, and are removed by hand or by the machine. No test looks
  for an earlier run's directories, and no crate of this repository is made to remove them." The
  head's generalisation from one sample, which the workflow file's standing answer reports, was
  narrowed on main by fa479e80, "[knowledge-architect] The head on the sweep tells a toolchain's
  files from a test's and gives no ranking to the owner, and the head on `tempfile` states its
  property" (an ancestor of origin/main, `git merge-base --is-ancestor`).
- **c. Whether it is already recorded.** Yes: the head above (`constrains`, `respect`), with its
  two rivals, `rejected@knowledge-architect@mod-tempdir-for-its-sweep` and
  `rejected@knowledge-architect@testdir-for-test-directories`, and the guard of
  `design@knowledge-architect@scratch-removed-by-a-guard`. A cleaning home for the leftovers would
  be a sweep, which these entries rank against; the file argues none of them again. The act verdict
  of `goal@generic-tools-and-procedures-are-shipped` carries an owner question on the generic parts
  the sessions set up, the lint on the temporary directory among them; that tool is the workflow
  file's standing answer, in the cluster `tools-built-in-sessions`, not this finding.
- **d. Its kind.** An observation: the state the decision accepts, with its cost measured.
- **e. Its scope.** The tests of this repository, under `goal@the-workflow-leads-to-better-code`;
  the owner of any fix would be the root Component.
- **f. A better fix.** The nearest rival is a one-time removal by the branch of the intake. It
  changes nothing of the record and is a command on one machine, not a change of the project.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, an observation. Ground: `design@knowledge-architect@nothing-sweeps-a-killed-run` decides that a killed run's directories are removed by hand or by the machine, and the file argues none of its rivals again.

## workflow-item-routing-override: the rule that a workflow finding stays here sits away from the moment of routing

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-knowledge-architect-intake-forty-seven-files.md`, in its section on `workflow-item-routing-override`, which gives the outcome. Member here: P1.

## design-course-misread: the course of a design discussion is misread

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow.md`, in its section on `design-course-misread`, which gives the outcome. Member here: the section "The project's side of W1", which reports that the root `CLAUDE.md` names no rule of the project on bounded work.
