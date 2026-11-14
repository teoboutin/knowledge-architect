# 2026-11-08-knowledge-architect-quick-fixes

- **Received file**: 2026-11-08-knowledge-architect-quick-fixes.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, at main `v0.9.0-214-g5b88b090`; the project's main commit when the work began, `5b88b090`; the branch `quick-fixes-create-git-model`; no uncommitted changes in the tree."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@restatement-size-test`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: touches, respect.
  - `design@knowledge-architect@no-operation-loses-content`: conflicts, check-reversal.
  - `goal@only-needed-instructions`: constrains, respect.

## index-only-op-scope-unclear: whether an operation on git's index alone is barred while the tree holds work

Members: 2026-11-08-knowledge-architect-quick-fixes P1.

- **a. What it says.** The session had staged, for its first commit, the changes of four later commits, and unstaged them with `git reset -q -- <paths>` while the tree held uncommitted work. It blames the root `CLAUDE.md`, `instructions@git-workflow` point 2: "**While uncommitted changes exist in the tree, run no operation that touches history, restores files, or removes a file that holds uncommitted content.** Examples (non-exhaustive): `reset --hard`, `checkout -- <path>`, `restore`, ...". `reset` and `restore` are on the list, and the list is non-exhaustive, so the rule reads two ways for an operation on the index alone. The session judged it outside the hazard. Its proposal: the bullet says that an operation on the index alone, such as `git reset -- <path>` or `git restore --staged <path>`, "unstages and loses nothing, and is allowed", in `design@knowledge-architect@no-operation-loses-content` first, the root following.
- **b. Whether it still applies.** At the version, main `5b88b090`, `git show 5b88b090:CLAUDE.md` holds the sentence at line 218. On origin/main, `v0.10.0-59-g605d1538`, `path@knowledge-architect@CLAUDE.md` holds it unchanged in point 2, with the hazard bullet at line 219: "The hazard is **content loss**, in two forms: an operation that removes uncommitted content from the working tree, and a history edit that drops a commit no other reference holds." The head, line 973 of `register@knowledge-architect@design`, says the same: "Content is lost in two forms: an operation that removes uncommitted content from the working tree, which no commit holds, and a history edit ...". Neither names git's index. It applies.
- **c. Whether it is already recorded.** No issue holds it. The search graded `design@knowledge-architect@no-operation-loses-content` `conflicts` / `check-reversal` (agent-15): the proposal's sentence "is false where a path's staged content differs from both HEAD and the working tree". I measured that claim, with git 2.53.0, in a scratch repository: a file committed as `a`, staged as `staged`, then written as `worktree` (`git status --short` prints `MM f`); `git reset -q -- f` leaves `M f`, the working tree `worktree`, and the staged content only as a "dangling blob" that `git fsck --lost-found` lists. So the proposal as worded is false in that case: an index-only operation can drop content no commit and no file holds. `design@agent-skills@restatement-size-test` (`constrains`, respect) bounds the root's copy to one sentence with its pointer. `goal@only-needed-instructions` (`constrains`, respect): "A case that judgement handles well enough gets no instruction".
- **d. Its kind.** A defect of the text, unclear: the bold rule's "restores files" and its example `restore` admit two acts for `git reset -- <path>`, and the hazard sentence, in the root and in the head, names the working tree alone, though staged content is content too. Provable by reading, and by the measurement above.
- **e. Its scope.** It serves `goal@only-needed-instructions` and the head's own rule, "No operation can lose content, committed or not". The owner of the fix: the root design home, `register@knowledge-architect@design`, and the root `CLAUDE.md`'s restatement. It reverses nothing: staged content is a member that the head's first sentence covers, so naming git's index in its forms sentence is recorded directly, per `primer@design-heads`, under `skill@knowledge-architect-decision-recording` at application.
- **f. A better fix.** The report's sentence, "loses nothing, and is allowed", is false where the staged content differs from the working tree's, as measured. The nearest rival is no change: the session judged right in its case, since its working tree held every staged byte. The fact that decides: the rule's words read two ways, and the reading the report proposes would lose content in the case measured, so a session that took it would lose work; a rewording that names where content lives settles both readings and adds no instruction. The repair is at the cause, the forms sentence of the head and its restatement, not a new permission.
- **g. Its route.** A text edit of this repository's design home and of its root `CLAUDE.md`, under `skill@knowledge-architect-decision-recording` for the head and `skill@knowledge-architect-agent-configuration` for the root.
- **h. The proposal.** In `register@knowledge-architect@design`, head the section "no-operation-loses-content", replace "Content is lost in two forms: an operation that removes uncommitted content from the working tree, which no commit holds, and" by the first block. In `CLAUDE.md`, point 2, replace the hazard bullet by the second block. Instructions: 0 added, 0 removed; the root's bullet stays one sentence, the head's forms sentence restated with the check that shows the case, `git diff -- <path>`, and its pointer is the one point 2 already carries, so `design@agent-skills@restatement-size-test` is kept. Nearest rival: no change, defeated as item f says.

```text
Content is lost in two forms: an operation after which uncommitted content is held by neither the
working tree, git's index nor a commit, as `git reset -- <path>` does to staged content that differs
from the path's working-tree content, and
```

```text
- The hazard is **content loss**, in two forms: an operation after which uncommitted content is
  held by neither the working tree, git's index nor a commit, as `git reset -- <path>` does to
  staged content that differs from the working tree's, which `git diff -- <path>` shows, and a
  history edit that drops a commit no other reference holds.
```

Default: handle now, the two rewordings above.
Outcome: decided by the session: handle now. Ground: the rule's words admit two acts for an operation on the index alone (item d); the report's own sentence is false in the case measured with git 2.53.0 (item c); naming git's index is a member that "No operation can lose content, committed or not" covers, per `primer@design-heads`, so no reversal; it adds no instruction, per `goal@only-needed-instructions`.
