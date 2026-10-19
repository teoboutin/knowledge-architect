# 2026-10-16-knowledge-architect-anchors-test-closure

- **Received file**: 2026-10-16-knowledge-architect-anchors-test-closure.md, the project file of the
  retrospective whose workflow file is analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-16-knowledge-architect-anchors-test-closure-klarch-workflow.md`.
- **Version used**: built from source, main at `v0.7.0-14-g1754037f`, read at `a23c261`, as that
  analysis says.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**: none.

## P1. No way the Git rule allows to revert between two mutations in one scratch worktree

- **a. What it says.** `skill@klarch-development@discriminating-tests` gives the procedure for one
  mutation in a scratch worktree. `instructions@git-workflow` point 2 forbids, while uncommitted
  changes exist, any operation that "touches history or restores files", naming
  `checkout -- <path>`, with `git rebase --continue` as its one exception. The session ran eight mutations in one
  worktree and reverted each with `git checkout -- <path>`. No content could be lost: the work under
  check was the worktree's scratch commit. One worktree per mutation costs one full build each. The
  fix proposed: a second exception in point 2, for that checkout in such a worktree.
- **b. Whether it still applies.** Yes. `path@knowledge-architect@CLAUDE.md`, lines 193-196, and the
  mutation procedure of `path@agent-config@skills/klarch-development/SKILL.md`, lines
  79-115, are unchanged since the session; the procedure describes one mutation and says nothing of
  a second.
- **c. Whether it is already recorded.** No; no `rejected` entry of this repository covers it.
- **d. Its kind.** Missing: two instructions of the project leave no move that satisfies both at a
  build cost a session can bear. The rule's letter is wider than its home's reason.
- **e. Its scope.** The rule's home is `design@knowledge-architect@no-operation-loses-content`,
  whose hazard is content "which no commit holds" removed from the working tree, and which states
  "The one exception is `git rebase --continue`". In the scratch worktree the only uncommitted content
  is the mutation, whose removal is the purpose. A second exception changes the head's statement of
  one exception, so it is a change of the decision, argued on the in-change path of
  `skill@knowledge-architect-design`. The development skill's own reason for the worktree already
  expects a revert there: "Reverting a hand-made mutation in the live tree with a restore can lose
  uncommitted work".
- **f. A better fix.** Rivals:
  - the proposal, a second exception in the head and in point 2;
  - the rule narrowed to its reason, content no commit holds, which a scratch worktree's mutation is
    not meant to be: a rewording of the head that admits every case where the content removed is the
    revert's purpose, wider than the evidence;
  - the revert stated where it is delivered, in the development skill's procedure, with point 2
    unchanged, which leaves the two texts in contradiction;
  - a revert by `git apply -R` of the mutation's diff, which restores files as checkout does, so the
    rule forbids it too.

  The proposal is the narrowest that removes the contradiction, and the session's cost figure, a
  full build per mutation, is its evidence. Instructions added: 1 in point 2, restated from the head.
- **g. Its route.** A decision, on the in-change path of `skill@knowledge-architect-design`: the
  head and point 2 in one commit, and the development skill's procedure saying that several
  mutations in one worktree are each reverted that way.
- **h. The proposal.** Handle now, as g says.
- **Default**: handle now, on the in-change path.
- **Outcome**: Simple fix: the git rules should not apply when in a scratch worktree which is only meant for temporary testing and not for durable work. No need for a design session, unless you have a strong reason to push back on this.

## P2. The mutations of the second commit are not all quoted as `-` and `+` lines

- **a. What it says.** The repair commit records eight mutations in prose, against "**A recorded
  check that does not show the mutation applied establishes nothing.** It quotes the `-` and `+`
  lines `git diff` printed". The file proposes nothing: a lapse.
- **b.** The rule is at lines 130-132 of
  `path@agent-config@skills/klarch-development/SKILL.md`. The commit `3362e5a`,
  "[review] Repairs of the review before the merge: the `anchors` tests pin each property of a held
  home directory", records the eight mutations in prose, as the file says.
- **d. Its kind.** A lapse of a clear instruction.
- **h. The proposal.** No change.
- **Default**: no change, a lapse.
- **Outcome**: accepted.
