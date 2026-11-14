# 2026-11-12-knowledge-architect-retrospective-standing-questions

- **Received file**: 2026-11-12-knowledge-architect-retrospective-standing-questions.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from source: main at `v0.9.0-384-g5ce466db` (`5ce466db`), branch `retrospective-standing-questions`, tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@version-line-names-the-main-commit`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@knowledge-architect@no-operation-loses-content`: conflicts, check-reversal.
  - `design@knowledge-architect@scratch-worktree-outside-git-rules`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.

## restore-run-with-dirty-tree: a file restored from a commit while the tree held uncommitted changes

Members: 2026-11-12-knowledge-architect-retrospective-standing-questions P1,
2026-11-13-knowledge-architect-plan-directories-slice-1 P1, 2026-11-14-knowledge-architect-planned-splits-slice P1.

- **a. What it says.** Three sessions report a restore run while other files held uncommitted
  work, to drop an edit of the agent's own:
  - retrospective-standing-questions P1: the session ran `git checkout -- crates/agent-skills/docs/tripwires.md`;
    "The file held only that edit, so nothing was lost";
  - plan-directories-slice-1 P1: an implementing agent ran `git checkout -- <file>` in its own
    worktree, "to drop an edit of its own to a mock file. Only that edit was lost";
  - planned-splits-slice P1: a repair agent ran `git checkout origin/main -- <file>`; "the restored
    file held none, so nothing was lost"; "The rule was in the brief's list of files to read whole".

  Each file calls it a lapse. Only planned-splits proposes a fix: a brief sending an agent into a
  dirty tree "could say that a file is restored from a commit only once the tree is clean", which
  "Adds 1 instruction to a brief, none to a text".
- **b. Whether it still applies.** The rule is unchanged from the three versions (v0.9.0-384,
  v0.10.0-5, v0.10.0-21) to origin/main: `path@knowledge-architect@CLAUDE.md`, `## Git` point 2,
  line 221, "**While uncommitted changes exist in the tree, run no operation that touches history,
  restores files, or removes a file that holds uncommitted content.**", with `checkout -- <path>`
  among its examples.
- **c. Whether it is already recorded.** No entry records these lapses. The verdict on
  `design@knowledge-architect@no-operation-loses-content` is `conflicts`, `check-reversal`, but
  against two other subjects, `index-only-op-scope-unclear` and `fold-autosquash-conflict-no-move`;
  of this one it says "the three restore lapses meet the rule as it stands". The verdict on
  `design@knowledge-architect@scratch-worktree-outside-git-rules` is `respect`: the implementer's
  worktree held work, so it is outside that exception and the rule bound it. The verdicts on
  `design@agent-skills@intake-shares-the-admission-standard`,
  `design@agent-skills@behaviour-additions-need-real-use` and `goal@only-needed-instructions` are
  `respect`.
- **d. Its kind.** Lapses of a clear instruction, three times, two of them by dispatched agents.
  No unintended content was lost: each restored file held no uncommitted content but the edit the
  agent meant to drop, and planned-splits' held none.
- **e. Its scope.** `design@knowledge-architect@no-operation-loses-content`, whose rule a fix must
  keep. Nothing is reversed.
- **f. A better fix.** Rival: the planned-splits proposal, a sentence in a brief. It is no change
  to any text of the project: what a brief holds is the dispatching session's judgement. Second
  rival: narrow the rule to admit a restore of a file that holds only the edit to drop. It loses:
  the act then rests on the agent's belief about what the file holds, the belief the rule exists
  not to trust, and the head's argument is not defeated by three cases where the belief was right.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, three lapses with no content lost. Ground: the lapse
rule of `skill@knowledge-architect-retrospective-intake@admission-rules`;
`design@knowledge-architect@no-operation-loses-content` stands, its argument undefeated.

## overlong-line-in-retrospective-skill: a line of about 160 columns merged in the retrospective skill's source

Members: 2026-11-12-knowledge-architect-retrospective-standing-questions P2.

- **a. What it says.** A line of about 160 columns was merged in `path@agent-skills@content/skills/retrospective/SKILL.md`, "a repair folded into an earlier commit counted as one, and the times the owner corrected work...", and in its installed copy; the report classes it a lapse, says no instruction states a wrap width, and proposes a rewrap in a commit of its own, layout only.
- **b. Whether it still applies.** The line was first written by de8e8da9, "[agent-skills] The retrospective asks for the tools a session set up, the record search's misses and a merged branch's review cost, and reports a needless instruction", and took its present 155 columns in bd37bb85, "[review] Repairs of the transcript review: ... and the long lines rewrapped" (`git log -S` on the phrase gives bd37bb85; `git log -L139,139` on the source gives bd37bb85, 019ed7a8, de8e8da9), on main as `v0.9.0-387-gbd37bb85`: the rewrap the report describes. On origin/main it stands at line 139 of the source and at line 120 of the installed copy. It is not alone: `awk 'length > 110'` over the source finds 12 lines above 110 columns, 11 outside the frontmatter, and `git ls-files 'crates/agent-skills/content/*.md' | xargs awk 'length > 100 && FNR>4 {n++} END {print n}'` counts 452 lines above 100 columns over the 31 Markdown files of the content, frontmatter lines aside. `grep -n -i -E "wrap|columns|line length"` over the root `CLAUDE.md`, `path@agent-skills@CLAUDE.md` and `skill@klarch-development` finds no width rule.
- **c. Whether it is already recorded.** The only verdict other than the seeds is `design@agent-skills@version-line-names-the-main-commit` (`constrains`, respect), which governs how item b is read. No issue, head or rejected design states a width.
- **d. Its kind.** No defect and no lapse: a lapse needs an instruction that exists and was not followed, and none states a width; the line is one of 452 above 100 columns in the content.
- **e. Its scope.** None of the goals: a line's width changes no act of an agent and no check.
- **f. A better fix.** The rival is the report's rewrap. The fact that decides: with no width rule, one rewrapped line among 452 makes no file conform to anything, and the commit costs a review of a diff no act needs.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, no rule is broken. Ground: no instruction states a wrap width (search in item b), so the finding is no lapse; 452 content lines exceed 100 columns, so one rewrap conforms to nothing.
