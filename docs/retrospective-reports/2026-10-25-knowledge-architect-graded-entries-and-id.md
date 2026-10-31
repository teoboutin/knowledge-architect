# 2026-10-25-knowledge-architect-graded-entries-and-id

- **Received file**: 2026-10-25-knowledge-architect-graded-entries-and-id.md, the project file of
  this repository's session that reduced the grounding's reading and built `klarch id`.
- **Version used**: built from its own source, main at `v0.8.0-100-gb985e14`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**: `design@knowledge-architect@no-operation-loses-content`,
  `design@knowledge-architect@scratch-worktree-outside-git-rules`,
  `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`.

## P1: a temporary edit of the live tree reverted with a restore

- **a.** To take a reference copy, the session added a line to a test, staged the file, ran it, and
  reverted with a restore from the staged copy while the tree held other uncommitted work. Nothing
  was lost.
- **b.** The rule is unchanged: `CLAUDE.md:212-218` at HEAD names `checkout -- <path>` among the
  operations refused while uncommitted changes exist.
  `design@knowledge-architect@scratch-worktree-outside-git-rules` exempts only a scratch worktree,
  and records that the owner chose it over an exception for this very revert.
- **d.** A lapse of a clear instruction, once.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: a lapse is not repaired by adding text,
  per `skill@knowledge-architect-retrospective-intake@admission-rules`, and the rejected exception
  stays rejected.

## P2, with C1 of 2026-10-25-knowledge-architect-graded-entries-and-id-klarch-workflow: `check --staged` compares the staged installed copies with the working tree's shipped text

- **a.** With installed-text sources edited and installed in the working tree, a commit of the Rust
  changes alone failed `check --staged` on seven installed files: the binary embeds the shipped text
  of the tree it was built from. The workflow file proposes that the finding's message say so, so a
  partial commit stages the sources and their installed copies together.
- **b.** Still applies: the message in `path@core@src/check/agents.rs` is unchanged since
  `b985e14`.
- **c.** Recorded: a further instance of
  `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, opened before
  the session. Its closing options do not include rewording the message.
- **d.** A defect of the checker, already recorded.
- **f.** The message is the smaller repair, and needs no change to what `--staged` judges; the
  issue's own options change the comparison, which is a design question.
- **g.** An issue already open.
- **h.** Add the instance and the message as a closing shape to the issue.
- **Default**: the issue gains the instance and the shape.
- **Outcome**: decided by the session: the issue gains the instance and the shape. Ground: the
  instance is the issue's subject, and the shape is a repair its discussion has not weighed.
