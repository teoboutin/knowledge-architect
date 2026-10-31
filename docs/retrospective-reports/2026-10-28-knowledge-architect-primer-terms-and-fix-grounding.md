# 2026-10-28-knowledge-architect-primer-terms-and-fix-grounding

- **Received file**: 2026-10-28-knowledge-architect-primer-terms-and-fix-grounding.md, the project
  file of this repository's session that added the primer's terms section, run in a cloud harness.
- **Version used**: built from its own source, main at `v0.8.0-116-gde6ed98d`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**: `design@knowledge-architect@git-flow`,
  `issue@knowledge-architect@ci-does-not-run-on-a-draft-and-no-record-says-why`,
  `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`.

## P1, with P1 of 2026-10-29-knowledge-architect-met-outside-upstream-route: the `## Git` procedure names `gh`, which a cloud session may not have

- **a.** Each step of `instructions@git-workflow` is a `gh` command. Two cloud sessions had no `gh`;
  each ran every step through the harness's own GitHub tools, the merge predicate included. One step
  failed in both: the merged branch was not deleted, since the merge tool deletes none and a
  `git push origin --delete` was refused by the container's proxy. The 10-28 file proposes no change;
  the 10-29 file proposes one clause in point 5: where the session cannot delete the branch, it tells
  the owner, who deletes it.
- **b.** Still applies: every step is still a `gh` command at HEAD (`CLAUDE.md:189-190`, `:273-276`,
  `:288-291`, `:307-308`). No text covers a branch the session cannot delete.
- **c.** No record has it as its subject; `design@knowledge-architect@git-flow` names no `gh` command.
- **d.** No defect established: both sessions met each step's intent by judgement, which the
  predicate's stated intent allows, per `primer@room-to-judge`. The deletion was refused by the
  environment.
- **f.** Rival: the 10-29 clause. A session that cannot take a step already reports it, as both did
  in their retrospectives, and the clause would add an instruction for a move the session has.
- **g.** Nothing.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: every step but the deletion was met by
  judgement, the deletion's failure is the environment's, and an instruction for a move the agent
  already makes is worth text only where its default behaviour is systematically wrong, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`.
