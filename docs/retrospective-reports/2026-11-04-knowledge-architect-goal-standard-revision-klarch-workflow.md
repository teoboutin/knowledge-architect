# 2026-11-04-knowledge-architect-goal-standard-revision-klarch-workflow

- **Received file**: 2026-11-04-knowledge-architect-goal-standard-revision-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-81-g59ac0c32`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@check@some-findings-in-a-goals-home-name-a-form-refused-there`.

## W1: the standing-state reviewer fetches tags into the repository every worktree shares (K4)

- **a.** What it says: the standing-state reviewer counts horizons, several in release tags; its
  repository held no tag newer than the clone's last fetch, so it ran `git fetch --tags` in its
  worktree, which writes tag refs into the repository every worktree shares. No file changed. The
  owner reports it is not the first occurrence. Proposal: one sentence in the reviewer's definition:
  count release tags from `git ls-remote --tags origin`, which writes nothing, and never fetch. +1.
  Rival: the brief names the tags; the definition wins as the one text every dispatch delivers.
- **b.** Whether it still applies. At `v0.9.0-81-g59ac0c32` and at HEAD,
  `path@agent-skills@content/agents/standing-state-reviewer.md`, lines 71 to 76, asks of every
  tripwire whether "its horizon has passed", counted "with `git log -S`" from the commit that wrote
  the line, and line 78 hosts the fragment `path@agent-skills@fragments/shallow-history.md`, which
  covers a shallow clone only: "report each horizon you could not count for that reason, and that
  `git fetch --unshallow` in the dispatcher's checkout makes it countable". Its `%%` comment gives the
  reason the delivered text does not: "The agent reports rather than fetches: a fetch writes to the
  repository, which the tripwire auditor's read-only rule forbids, and needs a reachable remote". No
  installed text names a tag missing locally; a grep of "fetch" and "ls-remote" over
  `path@agent-skills@content/` and `path@agent-skills@fragments/` finds only that fragment and an
  unrelated `cargo fetch`. Measured from git's documentation, `git fetch --help`, git 2.53.0: `--tags`
  fetches tags "in addition to whatever else would otherwise be fetched", so it also updates the
  remote-tracking branches, origin/main among them, which the dispatcher's ranges and every other
  reviewer's range name.
- **c.** Not recorded: no entry of `bears.md` or `may-bear.md` names a tag fetch, beyond the
  searcher's reason on `design@knowledge-architect@scratch-worktree-outside-git-rules`. That head
  puts a scratch worktree outside this repository's Git rules because "no operation in it can lose
  content and nothing in it reaches the main branch": a fetch from it writes refs the main checkout
  shares, so the head's premise does not cover it, and the head binds this repository's rules, not
  the installed agent. `design@agent-skills@tripwire-horizon` names release tags among the occasions.
- **d.** Kind: missing. The input the count needs, the tags the remote holds, no text supplies, and
  the reviewer's judgement chose a write; the reason against it stands only in a `%%` comment, which
  no agent reads.
- **e.** Scope: `goal@knowledge-architect@documentation-stays-consistent`, which the standing-state
  review serves, and the dispatcher's ranges. Owner: the installed text, the fragment that two agents
  host, `agent@knowledge-architect-standing-state-reviewer` and
  `agent@knowledge-architect-tripwire-auditor`. The fix reverses no decision: the fragment's comment
  already decides "reports rather than fetches", and a missing tag is a member its argument covers,
  since both cases are a count the local history cannot make and a fetch is the write it forbids.
- **f.** A better fix. The nearest rival is the file's proposal, a sentence in the reviewer's
  definition alone. It loses on two facts: the fragment is hosted by the tripwire auditor too, which
  counts the same horizons, so one edit of the fragment reaches both; and `git ls-remote` gives the
  remote's tags and their objects, while the count also needs each tagged commit in the local
  history, which a dispatcher's fetch supplies and the reviewer cannot. So the fix widens the
  fragment's existing report to a missing tag: the reviewer may list the remote's tags with `git
  ls-remote --tags origin`, which writes nothing, reports each horizon it could not count, names the
  fetch the dispatcher runs in its own checkout, and runs no fetch itself.
- **g.** Route: an edit of installed text whose design is settled, at its source, as
  `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", says, read whole.
- **h.** Proposal: rewrite `path@agent-skills@fragments/shallow-history.md` to: "In a shallow clone,
  where `git rev-parse --is-shallow-repository` prints `true`, a count over the history starts from
  the clone's boundary commit and comes out too low, with no error; a count of release tags sees
  only the tags the repository holds, and `git ls-remote --tags origin`, which writes nothing, lists
  the remote's. Report each horizon you could not count for either reason, and the command that makes
  it countable in the dispatcher's checkout, `git fetch --unshallow` or `git fetch --tags`. Run no
  fetch yourself: it writes refs that every worktree of the repository shares, the remote-tracking
  branch of the main branch among them." The `%%` comment gains the case and its mechanism, with git's
  documentation as the source. Instructions: +1 (run no fetch), one widened (report). Evidence: this
  session, and the owner's report that it recurred; the mechanism is git's documented behaviour.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now, the fragment widened. Ground: the fragment's own
  decision, report rather than fetch, covers a missing tag, per
  `design@agent-skills@extension-follows-the-ground`; the input is a provable gap no text supplies;
  `git fetch --help` shows that the write moves the remote-tracking branches every range names.

## W2 and W3, with P1 of 2026-11-04-knowledge-architect-goal-standard-revision and C2 of 2026-11-06-knowledge-architect-consumers-out-klarch-workflow: a batch drafted uncommitted in the tree

- **a.** A batch of more than three goals stays uncommitted in the goals homes for the whole review.
  W2: each round's six reviewers needed the drafts, which no commit held, so the session copied the
  modified files into each worktree by hand; every git operation ran beside uncommitted work, and
  one, `git rm -f`, lost an uncommitted section of the record (P1 of the project file; the owner
  ruled no project-text change, Q47). W3 and C2 of consumers-out: the committed record cannot name a
  drafted goal in a form both trees accept: a goal reference passes `check` over the tree and
  fails `commits` over the record's commit, a bare slug does the reverse; consumers-out wrote
  "#slug" bare, beside no issue entry, and folded two repairs. Blamed:
  `skill@knowledge-architect-goal-setting@whole-set-review` step 4, "Commit it at each round, apart
  from the drafts, which stay uncommitted until the owner rules on them", beside
  `skill@knowledge-architect-review@review-invariants`, a worktree "detached at that commit, never
  the live tree". Proposed (W2): commit each round's drafts on the branch, the ruling recorded by the
  commit that writes the ruled goals; 1 instruction changed. W3: closed by W2. C2: a form for a
  drafted goal, or the record's commit judged with the drafts absent.
- **b.** Still applies. At `59ac0c32` and at HEAD,
  `path@agent-skills@content/skills/goal-setting/SKILL.md`, line 115 to line 116: "A batch of more
  is written in the goals homes of the working tree, uncommitted"; line 213 to line 214 as quoted.
  Line 159 to line 160: "for a batch drafted in the tree, commit the goals as the tree holds them".
  The goal reviewer runs `cargo klarch show` and `cargo klarch anchors`
  (`path@agent-skills@content/agents/goal-reviewer.md`, line 21 to line 22), commands of the
  checker, which the review invariant gives a worktree at a commit. Recurrence: two files, at
  `v0.9.0-81` and `v0.9.0-160`.
- **c.** `design@agent-skills@goal-batch-ruled-in-the-tree` states the rule: "A larger batch is
  written in the goals homes of the working tree, uncommitted". Its argument is the owner's: a
  larger batch "is read more readily in the tree", ruled as the tree holds it; it gives no reason for
  "uncommitted". `git log -S` finds the word entering with the record at commit `803b9a45`, whose
  message argues the committed record (Q49) and nothing about the drafts' state. No rejected entry
  holds committed drafts (`grep -n -i 'uncommitted\|draft'` over
  `register@agent-skills@rejected` finds none on goals).
- **d.** Defect of the text: two installed instructions for one moment, the batch rule and the
  review's worktree rule, leave no move that satisfies both without a copy step the text does not
  give; the form failure follows from the record and the drafts living in two trees. In scope
  whatever the expectation sets say, per
  `skill@knowledge-architect-retrospective-intake@what-to-establish` item d.
- **e.** Installed text, `agent-skills`; threatens `goal@agent-skills@no-contradiction-in-workflow`.
  The fix contradicts a statement of `design@agent-skills@goal-batch-ruled-in-the-tree`
  ("uncommitted"), so it goes to `skill@knowledge-architect-design`, in-change path; it keeps the
  head's argued part (the owner reads and rules on the batch in the tree, by naming each goal
  dropped or reworded). No rejected design comes back.
- **f.** Nearest rival: keep the drafts uncommitted and let a draft-mode reviewer read the live
  tree, as it writes nothing. It removes the copy step, and keeps the tree dirty for hours, which
  every git rule of a project that forbids history edits beside uncommitted work then meets, and
  keeps the form failure of W3 and C2. Second rival: the drafts in the record file rather than in
  the goals homes; it loses the checker's goals-home check over a draft, per
  `design@check@goals-home-cites-upward-only`. What decides for committed drafts: they give the
  reviewers a commit, keep the tree clean between rounds, and make a goal reference resolve in
  the record's commit, so C2 needs no checker change. What guards an unruled goal in a commit of the
  branch: the merge mode reports "each goal the range adds, rewords or removes whose ruling by the
  owner no commit message of the range records", per
  `design@agent-skills@goals-axis-in-the-review-before-a-merge`.
- **g.** A decision on the in-change path of `skill@knowledge-architect-design`, then the edit of
  the installed text.
- **h.** Step 3: "A batch of more is written in the goals homes of the working tree, and each
  round's drafts are committed with the record". Whole-set step 4: "Commit it at each round, with
  the round's drafts." Step 7: "for a batch drafted in the tree, the commit that writes the ruled
  goals records the ruling". `design@agent-skills@goal-batch-ruled-in-the-tree` rewritten in place,
  its premortem naming an unruled draft reaching the main branch, which the goals axis reports; its
  committed-drafts part is now `design@agent-skills@goal-batch-drafts-committed-each-round`.
  Instructions: 0 added, 3 reworded. A `Next release` Workflow entry. C2 of consumers-out needs no
  checker form once this lands. P1's project side stays as the owner ruled.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now, on the in-change path. Ground: the head's
  "uncommitted" has no recorded argument, and the review invariant, the git rules and the reference
  forms each need the drafts at a commit; `goal@agent-skills@no-contradiction-in-workflow`.

## W4: the review of the whole goal set had two stop rules

Analysed with W1 of 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md`.

## C1: renaming an entry's slug has no command

Analysed with C1 of 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md`.

## C2: a finding that names its repair as an exact span has no `--fix`

Analysed with C2 of 2026-11-01-thaum-design-record-audit-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-01-thaum-design-record-audit-klarch-workflow.md`.

## C3: a goals home whose findings name refused forms

- **a.** What it says: four findings of the reference check name, in a goals home, a form the
  goals-home check refuses there; "Recorded in the project as an issue … No further proposal".
- **c.** Whether it is already recorded: yes, `issue@check@some-findings-in-a-goals-home-name-a-form-refused-there`.
- **Default**: no change.
- **Outcome**: decided by the session: no change, already recorded. Ground: the issue stands at HEAD.

The map lists a C4 of this file; the file holds C1 to C3 only under "Findings on the checker"
(its headings, by `grep -n '^#'`), so no section is owed for it.

## The standing answers

- **a.** Items:
  - Installed file: edited at its source, as the work's subject. An observation.
  - The two false claims in messages: analysed with the claim-check cluster of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - No bare span; a drafted goal named in words: W3 of this file, cluster N3 (G2), home W2 of this
    file.
  - Hand tasks: C1 is cluster E (G4); C2 is cluster G (G4); copying drafts into each worktree is W2
    of this file, cluster N3 (G2); the reword is cluster H (G6).
  - Rules on code: none. An observation.
- **Default**: no change.
- **Outcome**: decided by the session: no change for the observations analysed here. Ground: no
  text is blamed.

---
