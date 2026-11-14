# 2026-11-14-knowledge-architect-planned-splits-slice

- **Received file**: 2026-11-14-knowledge-architect-planned-splits-slice.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-21-ga9ed6e3f` (`a9ed6e3f`) when the work began, branch `planned-splits`, no uncommitted changes."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@agent-skills@heads-argued-from-another-harness`: touches, respect.
  - `design@agent-skills@shared-text-fragment`: constrains, respect.
  - `design@agent-skills@host-lists-are-the-session-s`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@agent-skills@goal-standard-rules`: constrains, respect.
  - `design@agent-skills@extension-follows-the-ground`: constrains, respect.
  - `rejected@agent-skills@member-routed-by-approval-scope`: constrains, respect.
  - `design@agent-skills@admission-rules-home`: constrains, respect.
  - `design@agent-skills@heads-stand-on-argument`: constrains, respect.
  - `design@agent-skills@unargued-approval-is-argued`: constrains, respect.
  - `rejected@agent-skills@head-cites-every-owner-ruling`: constrains, respect.
  - `design@agent-skills@component-skill-delivery`: constrains, respect.
  - `design@agent-skills@module-split-guidelines`: constrains, respect.
  - `design@agent-skills@code-split-scout`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@the-owner-decides`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `design@agent-skills@met-outside-the-task-outcomes`: constrains, respect.
  - `design@agent-skills@overlay-by-separate-skills`: constrains, respect.
  - `issue@agent-skills@patching-an-installed-skill`: touches, respect.
  - `design@agent-skills@installed-fix-goes-upstream`: constrains, respect.
  - `issue@agent-skills@a-retrospective-that-runs-may-miss-an-item-kept-for-it`: touches, none.
  - `goal@records-reach-their-reader`: constrains, respect.

## fragment-host-list-undecided: no rule said how a fragment's host list is decided

Members: 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow W5;
2026-11-14-knowledge-architect-planned-splits-slice P3.

- **a. What it says.** W5: a question put a new agent as a host of two fragments, and the owner
  agreed; the searchers then found that each fragment held text the agent applies none of, against
  the rule that a host applies the whole fragment. Two more questions followed, and a third on who
  decides host lists. P3: `design@agent-skills@shared-text-fragment` held that the host lists of
  every fragment but the four of the admission standard "are the owner's: the owner approved each
  list, … so a host added to one goes to the owner", an approval being no ground under
  `primer@design-heads`. Both report the repair done on the branch.
- **b. Whether it still applies.** No. `design@agent-skills@host-lists-are-the-session-s`, in
  `path@agent-skills@docs/design/shipped-set.md` near line 88, holds: "The session decides which
  texts host a fragment, by the rule of `design@agent-skills@shared-text-fragment` that a text is a
  host only where its act applies the whole fragment, and reports each change of a list under the
  turn's "Decided" section". 15350653, on main, "[review] Repairs of
  the review before the merge of the slice on splits planned before the work: [...] the host lists
  the session's, [...]", removed the sentence that reserved the lists to the owner and recorded the
  thread; the head entered in 0a1080de, on main, "[review] Repairs of the decision-record review of
  the repairs: the record side and the host lists in heads of their own" (`git log -S` on the slug,
  then `git show` of each).
- **c. Whether it is already recorded.** Yes, as that head, with
  `design@agent-skills@shared-text-fragment` (`constrains`): "A text is a host only where its act
  applies the whole fragment". `rejected@agent-skills@member-routed-by-approval-scope` and
  `design@agent-skills@unargued-approval-is-argued` (`constrains`) bound it, and the repair keeps
  both.
- **d. Its kind.** W5: missing, for a maintainer of the installed text, since only this repository
  edits the fragments. P3: a defect of this repository's record, wrong. Both repaired.
- **h. The proposal.** No change: already repaired.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground:
`design@agent-skills@host-lists-are-the-session-s`, added by 0a1080de on main after 15350653
removed the owner's reservation, has the session decide each list by the host rule.

## repository-departure-buried: a subagent routed an installed-text fix to the workflow file, missing this repository's departure

Members: 2026-11-14-knowledge-architect-planned-splits-slice P2 (its interaction with W3 of
2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow).

- **a. What it says.** "The implementer met a false sentence in an installed skill and reported it
  "goes to the workflow file of the retrospective; I did not edit it", per
  `primer@met-outside-the-task`." The root's departure, in `instructions@repository-skills`: "**So
  is an item met outside the task whose fix lies in the installed text or in the checker**: here
  both are this project's own, so it takes the rows of `primer@met-outside-the-task`, never the
  workflow's file". "The session fixed it on the branch. The departure is one bolded sentence
  inside a long paragraph about findings of the retrospective and the audit." Proposal: "lift the
  departure out of its paragraph in" the section "into a sentence of its own at the head of the
  section, so a subagent that reads the section's opening finds it. Adds 0 instructions: a move."
  The false sentence was W3 of the workflow file, the planning skill's "This workflow installs
  none", reworded on that branch.
- **b. Whether it still applies.** The paragraph is the same at a9ed6e3f, the file's version, and
  at origin/main: `path@knowledge-architect@CLAUDE.md`, section "Skills", lines 170 to 181, the
  departure the second bolded sentence, lines 172 to 175. The primer's sentence the implementer
  followed, `path@agent-skills@content/PRIMER.md`, line 237: "**An item whose fix lies in the
  installed text or in the checker** ... goes to the workflow's file of
  `skill@knowledge-architect-retrospective`".
- **c. Whether it is already recorded.** The departure is
  `design@knowledge-architect@workflow-findings-stay-here`. Its documentation as a supersession of
  installed text is a subject of `issue@agent-skills@patching-an-installed-skill`, read whole,
  verdict `touches`: the issue lists this departure among "The three supersessions this
  repository holds today", and its closing condition has "this repository's root CLAUDE.md
  documents its three in that shape", the shape the issue's discussion decides. The verdicts on
  `design@agent-skills@met-outside-the-task-outcomes`, `design@agent-skills@overlay-by-separate-skills`,
  `design@agent-skills@installed-fix-goes-upstream` and `goal@records-reach-their-reader` are
  `respect`. `issue@agent-skills@a-retrospective-that-runs-may-miss-an-item-kept-for-it` is
  `touches`, action none: the item was fixed on its branch, neither kept nor missed.
- **d. Its kind.** A lapse of a clear instruction, once, by a dispatched agent. The primer's route
  made the agent report the item rather than edit installed text, and the dispatching session
  routed it by the departure, as `primer@met-outside-the-task` has a dispatcher do: nothing was
  lost.
- **e. Its scope.** `goal@records-reach-their-reader`; the owner of the text is the root
  `CLAUDE.md`. The move changes how a supersession is documented, which
  `issue@agent-skills@patching-an-installed-skill` decides.
- **f. A better fix.** Rival: the proposed move, now. It loses on two facts: one instance is no
  recurrence, and the lapse rule moves a delivery only on recurrence; and the shape in which the
  root documents its supersessions, inline or in a table, is the open question of
  `issue@agent-skills@patching-an-installed-skill`, so a move now would set that shape before its
  discussion. Second rival: no record at all; it loses, since the instance is the first evidence
  of a supersession missed where it is documented, which that discussion weighs.
- **g. Its route.** An instance of a recorded issue.
- **h. The proposal.** No change to the root now; the patching issue gains the bullet below, under
  "What bears on it".

Default: the patching issue gains the bullet below; no change to the root.
Outcome: decided by the session: an instance added to
`issue@agent-skills@patching-an-installed-skill`. Ground: the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules` needs a recurrence to move a
delivery; the shape of a supersession's documentation is that issue's discussion, per its
closing condition.

Text to add to the issue's "What bears on it", as a paragraph after its first, since the section
holds paragraphs, not bullets:

```
A supersession missed where it is documented: in this repository's slice on splits planned before
the work, 2026-11-14, a dispatched implementer met a fix in the installed text and reported it for
the workflow's file, by `primer@met-outside-the-task`, missing the root's supersession of that
route, which `instructions@repository-skills` states as its paragraph's second bolded sentence;
the dispatching session routed it by the supersession. The retrospective proposed lifting the
sentence to the head of the section. One instance; the shape decided here settles where it goes.
```

## restore-run-with-dirty-tree: a file restored from a commit while the tree held uncommitted changes

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-retrospective-standing-questions.md`, in its section on `restore-run-with-dirty-tree`, which gives the outcome.
