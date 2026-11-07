# 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow

- **Received file**: 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-11-g3d61549a`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@check@cross-project-references`, `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`.

## W1, with W1 of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow, W1 of 2026-11-01-knowledge-architect-pre-release-fixes-klarch-workflow, W2 of 2026-11-02-knowledge-architect-component-split-git-out-klarch-workflow and W4 of 2026-11-04-knowledge-architect-goal-standard-revision-klarch-workflow: the stop rule of goal rounds

- **a.** Four files report that a goal review in rounds did not stop. Three blame
  `skill@knowledge-architect-goal-setting@drawing-out-intent` step 4, "Repair each `settled`
  finding, and dispatch it again on the repaired batch, so that the version shown is a version
  reviewed": one batch ran five draft rounds with 8, 10, 6, 6 and 6 findings (development milestone
  plan); one goal ran five rounds (pre-release fixes); one goal ran five rounds and the owner was
  twice shown a version no review had read (git out). The fourth blames
  `skill@knowledge-architect-goal-setting@whole-set-review` step 6, "a round's count falls to a
  fifth of the first round's": the not-minor counts were 79, 46, 49, 50, 53 and 37, and the stop
  never fired (narrowed goals). The fixes proposed: a stop for draft mode, by pointer to the
  whole-set stop (pre-release fixes); a stop on findings on text the round did not see before, or on
  two rounds whose count does not fall, the last findings going to the owner (narrowed goals); none
  beyond the then open issue (git out). W4 of goal-standard revision reports the repair made on its
  own branch: one stop for both modes, no `blocks` finding on old wording, a no-progress stop to the
  owner.
- **b.** Repaired. At `v0.9.0`, line 102, the draft mode had no stop ("and dispatch it again on the
  repaired batch"), and line 174 held the fifth; `eb899523` (pre-release fixes), `3d61549a` and
  `5c7bc807` hold the same line. Commit `c5131ca1` (`v0.9.0-86`) gave the draft mode a stop on
  `settled` `blocks` findings on old wording; commit `a6c65b2a` (`v0.9.0-105`) made it one stop for
  both reviews and moved the fifth to `rejected@agent-skills@goal-review-stop-at-a-fifth`. At HEAD,
  `path@agent-skills@content/skills/goal-setting/SKILL.md`, line 131: "Stop when a round, from the
  second on, reports no `blocks` finding on old wording, whatever its route, or when the owner says
  so." Line 132: "From the third round on, where a round's `blocks` findings on old wording are no
  fewer than the previous round's, put the stop to the owner instead of running another round."
  Line 135: "Where the owner stops the rounds after a round's repairs, the owner's stop prevails:
  tell the owner that those repairs are unreviewed, and the goals axis of the review before the
  merge [...] reads them." Line 216, the whole-set step 6, stops "by the rule of step 4". Each
  member's proposal is met: the stop on old wording is narrowed goals' first form, the no-progress
  stop its second, and the unreviewed versions shown to the owner (git out) are now named to the
  owner and read by the merge mode.
- **c.** Recorded and closed: `design@agent-skills@goal-review-stops-on-old-wording` states the rule
  and cites the five-round cases; `rejected@agent-skills@goal-review-stop-at-a-fifth` and
  `rejected@agent-skills@goal-review-stop-over-all-wording` hold the two rivals. The issue the three
  earlier files name, `a-goal-the-draft-review-read-was-reworded-again-for-a-defect`, was closed by
  commit `32a5d365` (`v0.9.0-87`). Its text at `32a5d365^` lists the development-milestone and the
  git-out batches as its third and fourth instances; the pre-release-fixes batch and the
  narrowed-goals counts are not among them.
- **d.** Defect of the text (missing in draft mode, wrong in the whole-set mode), repaired.
- **e.** Installed text, `agent-skills`; serves `goal@agent-skills@goals-state-the-owner-s-intent`,
  "Every review of goals that runs in rounds [...] stops after a few rounds". No reversal.
- **h.** No change. The files after the repair, at `v0.9.0-109` and later, read the text as it
  stands (`git diff 3f6074c8 HEAD` on the skill changes two `%%` lines only). They report on the
  repaired stop in C2 and C3 below, which are new findings, not this one.
- **Default**: no change, already repaired.
- **Outcome**: decided by the session: no change, already repaired. Ground: commits `c5131ca1` and
  `a6c65b2a`, and `design@agent-skills@goal-review-stops-on-old-wording`, which states every part
  the members propose.

## W2: a plan document that schedules an issue did not close it

- **a.** What it says: `skill@knowledge-architect-planning@spec-or-milestone`: "The plan document
  that schedules it closes that issue in the commit that adds the plan document". The milestone
  scheduled four issues and narrowed a fifth, and its commit closed none; four plan reviews missed
  it, the standing-state axis found it. The delivery point is the section on choosing a spec or a
  milestone, the act is at the assembly. A lapse; no repair unless it recurs.
- **b.** Whether it still applies: the instruction stands,
  `path@agent-skills@content/skills/planning/SKILL.md`, line 76, at `3d61549a` and at HEAD. The
  instance was repaired before the merge, in ae41d55a ("the issues the milestone schedules closed").
- **c.** Whether it is already recorded: no entry. A search of the received files for "closes that
  issue" and "schedules it closes" finds only this file: no recurrence.
- **d.** Its kind: a lapse, once.
- **e.** Its scope: installed text, the planning skill. Reverses nothing.
- **f.** A better fix: moving the sentence to the assembly is the rival; the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules` admits a move only on a
  recurrence at one point of delivery, and there is none.
- **g.** Its route: nothing.
- **h.** The proposal: no change; the delivery mismatch is noted for a recurrence.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: one instance of a lapse of a clear
  instruction; the standing-state review caught it, as it is meant to.

## W3: a slice's harvest left two of its row's items undone

- **a.** What it says: `skill@knowledge-architect-planning@working-a-slice`, point 6, the harvest
  "per the harvest row of the slice's spec", and "an item of the row the tests exclude is named in
  the harvest's commit". The slice's commit wrote the heads and neither judged a premortem cause by
  the tripwire standard nor a losing alternative by the filter. Four review axes reported it. A
  lapse; no proposal.
- **b.** Whether it still applies: the instruction stands, line 468 of
  `path@agent-skills@content/skills/planning/SKILL.md`, at `3d61549a` and at HEAD. The instance was
  repaired in ae41d55a ("slice 0's harvest completed").
- **d.** Its kind: a lapse, once.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: a lapse of a clear instruction, caught by
  the review before the merge.

## W4: the whole read of the search's `bears` entries

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## C2: the rows of "What is already decided" generated by a script

Analysed with C3 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## The standing answer on the searchers' report fill

Analysed with C2 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## C1, with C1 of narrowed-goals, C1 of check-renamed, C1 of goal-standard-revision, C1 of the model crate's file and the item "moving heads" of consumers-out: no command renames an entry or a Component, or moves entries between Components

- **a.** What it says, per file:
  - C1 here: an issue entry and a goal renamed by `git mv`, a `sed` over the files a grep found,
    and a script. Proposal: a command that renames an entry's slug and rewrites every reference
    the entity table holds, leaving a record of a deliberation to the session; "not established
    whether it is worth the command: two uses in one session".
  - C1 of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow: two goals renamed, one
    removed, re-pointing over 26 files by `sed`. Proposal: a `rename` command for a heading or a
    file entry, listing the commit messages it cannot rewrite.
  - C1 of 2026-11-04-knowledge-architect-check-renamed-klarch-workflow: a Component renamed and its
    directory moved by hand over several scripts; the check found every dangling reference, but not
    33 lines of bare paths nor the prose names, whose first regular expression "rewrote too much,
    and it broke one code line". Proposal: a `rename-component` command that moves the directory,
    rewrites the manifest and every reference, and lists the bare mentions.
  - C1 of 2026-11-04-knowledge-architect-goal-standard-revision-klarch-workflow: one goal slug
    renamed over 11 files by a script, which also rewrote a quotation and a plan argument the session
    then repaired by hand. Proposal: a rename command leaving quotations in a record of a
    deliberation to the session.
  - C1 of 2026-11-04-knowledge-architect-model-crate-klarch-workflow: a split moved 62 heads,
    34 `rejected` entries, 3 tripwires and 10 issues, re-pointed over 95 files, by a script.
    Proposal: a command taking a list of entries and a target Component, which moves each byte for
    byte with what follows it and re-points every reference.
  - The standing answer of 2026-11-06-knowledge-architect-consumers-out-klarch-workflow: moving
    heads, `rejected` entries and issues into a new Component, a script of about 100 lines.
- **b.** Whether it still applies: yes. `klarch --help` at HEAD lists no command that renames or
  moves anything. The installed text requires the tasks: `path@agent-skills@content/skills/decision-recording/SKILL.md`,
  line 92, "renaming one means rewriting every reference in the same change. Grep before you
  rename."; `path@agent-skills@content/skills/component-creation/SKILL.md`, lines 175 to 180, point 4
  of the creation procedure, "move the record that is now the new Component's … each is
  re-pointed"; `path@agent-skills@content/skills/goal-setting/SKILL.md`, line 178, "When one is
  removed, every reference to it dangles". The same text stood at each file's version.
- **c.** Whether it is already recorded: no issue, head or `rejected` entry records a rename or a
  move command; the record search graded no such entry. What the record holds on the shape:
  - `design@agent-skills@a-past-sentence-is-rewritten`: a sentence about the past whose reference
    dangles is rewritten to the present, and "A verbatim quotation of the owner that names a renamed
    entry is left as it is, with a reference to the current entry beside it". So a command may not
    rewrite a quotation or a record of a deliberation's bare slug, which the proposals already say.
  - `design@model@an-entity-belongs-to-its-anchor`: "moving a document moves the entities in it",
    so a move rewrites the anchor segment of each reference, no more.
  - `goal@check@relocation-needs-only-goal-changes` admits "a Component renamed, its old name
    replaced wherever it is cited by one search and replace". Its measure is the check passing; the
    bare paths and prose names that check-renamed met are outside what the check judges, so no
    strain of that goal is shown (an assumption on the 33 lines, which this check did not re-count).
  - `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`: one form a
    move breaks beyond a mechanical edit; a move command meets it.
- **d.** Its kind: missing, a task the installed text requires carried by script, in six files of
  five sessions (one session has two uses).
- **e.** Its scope: `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` covers "a task whose
  result follows, with no judgement, from the tree … and what the session gives it", and owes it a
  command "once knowledge-architect's owner takes its generic part in", which "is an act of
  knowledge-architect's owner of its own" (`register@knowledge-architect@goal`, line 183). The
  references follow with no judgement; the quotations, the bare slugs of a record of a deliberation
  and the prose names do not, so the command's generic part is the references and the listing of
  the rest. Owner of the fix: the checker's crates, which Component to be decided (it reads the
  entity table of `model` and writes documents, as `create` and `check --fix` do). No decision is
  reversed.
- **f.** A better fix: the nearest rival is the present procedure, the check's list of dangling
  references repaired by the session's own script. The deciding facts: goal-standard-revision's
  script rewrote a quotation and a plan argument, and check-renamed's broke a code line; the
  checker already holds both what to rewrite (the entity table) and what not to (the quotation rule
  of `design@check@quotation-read-by-no-id-lint`, the bare-slug rule). A second rival, a report of
  bare paths in `check` alone, finds and rewrites nothing.
- **g.** Its route: an issue, since the work is a new command, and taking it in is the owner's.
- **h.** The proposal: open one issue for the three tasks, at the root, since the Component that
  owns the command is undecided: anchor `knowledge-architect`, kind `todo`, id
  `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`.
  - What: renaming an entry's slug, renaming a Component and moving entries between Components are
    done by the session's own scripts, over the check's list of dangling references, in six
    retrospective files of 2026-11-01 to 2026-11-06.
  - Why it matters: the installed text requires each task, and its references follow from the
    entity table, per `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`; the scripts
    rewrote a quotation, a plan argument and a code line they should not have.
  - What would close it: the owner takes the tasks in or declines them; if taken in, a design
    discussion places the command, which rewrites every reference the entity table holds, moves the
    entries with the `rejected` entries their citing heads cover and the tripwires guarding them,
    and lists each quotation, bare slug, bare path and commit message it leaves to the session,
    with a test over a mock project.
- **Default**: open the issue; the owner is told that taking the tasks in is the owner's act.
- **Outcome**: decided by the session: open `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component` at
  `knowledge-architect`, kind `todo`. Ground: no record holds the need, and the goal names the
  taking-in as the owner's, so the issue holds the need until the owner rules; it contradicts no
  head and brings back no `rejected` entry.

## The standing answers

- **a.** Items:
  - Installed file: no. An observation.
  - Both lapses of `primer@verify-before-relying`, the count in an issue entry and the defect
    diagnosed to the owner without `cargo klarch issues` or `show`: analysed with the claim-check
    cluster of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - Pointers: another project's design entries named in words in the plan documents. A description
    in words is outside `design@agent-skills@plain-text-is-no-repair`, which says so ("a description
    in words, is outside the exception"), and the missing cross-project form is recorded as
    `issue@check@cross-project-references`. Not an instance of the pointers cluster.
  - Hand tasks: C1 is cluster E (G4), home C1 of this file; C2 is cluster B (G1), home C3 of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow; the searchers' fill
    scripts are cluster L (G1), home C2 of that file.
- **Default**: no change.
- **Outcome**: decided by the session: no change for the item analysed here, the words naming
  another project's entries. Ground: `design@agent-skills@plain-text-is-no-repair` puts a
  description in words outside its exception.

---
