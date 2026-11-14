# 2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow

- **Received file**: 2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, at `v0.9.0-451-gbc2c7036` when the work began; branch `release-0.10.0`; clean tree."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@query@no-command-counts-a-tripwire-s-horizon`: touches, respect.
  - `design@agent-skills@standing-state-review-before-every-merge`: constrains, respect.
  - `design@agent-skills@capabilities-not-structure`: constrains, respect.
  - `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies`: touches, respect.
  - `design@agent-skills@intent-through-goals-not-overrule`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `design@agent-skills@tripwire-stands-on-the-standard`: constrains, respect.
  - `design@agent-skills@tripwire-horizon`: conflicts, check-reversal.
  - `rejected@agent-skills@owner-rules-each-tripwire`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.

## release-tag-horizon-passes-late: a horizon counted in release tags passes only after the last review that could delete its tripwire

Members: 2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow W1. One session.

- **a. What it says.** Two tripwires watched "the next two release tags", and the release being prepared would make the second. The standing-state reviewer judged, by the text, that the horizon passes only once the tag stands, after the release branch merges, "so the deletion would need a branch of its own, with its own review". The owner ruled to delete the two tripwires in the release branch, one tag early. It blames the `Watches until` sentence of `skill@knowledge-architect-issue-tracking@tripwire-entry`, "release tags", and the exit "its horizon passes unfired: it is deleted". Proposal: name the occasion "releases, a release counted at the review before the merge of the branch it will tag" (a rewording); rival: the owner ruling each time, which loses because "every tripwire whose horizon counts release tags meets the same extra branch".
- **b. Whether it still applies.** The text is unchanged: `path@agent-skills@fragments/tripwire-standard.md` line 59, "release tags", and `path@agent-skills@fragments/tripwire-lifecycle.md` line 9, "its horizon passes unfired", the same at `bc2c7036` (`grep -c` on each phrase in its file gives 1). The premise that the deletion needs a branch of its own does not hold: `path@agent-skills@content/agents/standing-state-reviewer.md` line 75 has the reviewer "Read every entry in every one of those homes, not the subset whose subject the diff touches", asking of each whether its horizon has passed. The next branch to merge, whatever its work, reads the two tripwires after the tag and reports the passed horizon, and its session deletes them as a repair of its review, per `skill@knowledge-architect-review@what-review-leaves`.
- **c. Whether it is already recorded.** `design@agent-skills@tripwire-horizon` (conflicts, check-reversal) fixes the occasions as ones "counted by a command over the history from the commit that wrote the line"; a release not yet tagged is in no history, so the proposed occasion would break that property. `design@agent-skills@standing-state-review-before-every-merge` (respect) makes that reviewer "the standing re-entry point of every tripwire home", at every merge. `design@agent-skills@tripwire-stands-on-the-standard` (respect) admits a removal "on the owner's word", the exit the owner took. `rejected@agent-skills@owner-rules-each-tripwire` (respect) is not brought back by no change, since no change needs no ruling. `design@agent-skills@intent-through-goals-not-overrule` (respect): the owner's ruling weighed a cost for one release, and decides nothing beyond it.
- **d. Its kind.** No defect: the finding's cost rests on a premise the text does not hold. The text delays the deletion by one merge, at the cost of one more reading of two tripwires.
- **e. Its scope.** The installed text, `agent-skills`; the proposal would reword a member of `design@agent-skills@tripwire-horizon` against its countability property.
- **f. A better fix.** No change. The proposal's rival analysis assumed an extra branch; with the next merge's review carrying the deletion, the cost is one reading, and the occasion stays one a command counts, which `issue@query@no-command-counts-a-tripwire-s-horizon` would count.
- **g. Its route.** Nothing.

Default: no change.
Outcome: decided by the session: no change. Ground: the standing-state reviewer reads every tripwire at every merge, per `design@agent-skills@standing-state-review-before-every-merge`, so a horizon passed at a tag is reported and deleted at the next merge with no branch of its own; the proposed occasion, a release counted before its tag, is in no history, against `design@agent-skills@tripwire-horizon`.

## unfired-judged-over-branch-only: the standing-state review judges "unfired" over the branch, and a deletion at a horizon claims it over the whole watch

Members: 2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow W2. One session.

- **a. What it says.** The standing-state reviewer judged a tripwire unfired from the branch's range alone, while its firing clause reads every commit on the main branch since its line was written, some 550 commits. Before writing "neither fired" into the deleting commit, the session found the gap; a search of those messages gave 70 candidates, too many to judge, so it grounded "unfired" on the chain of reviews: each merge's standing-state review re-read the tripwire, and none opened its response. It blames the reviewer's question "has it fired and whether its horizon has passed" and the exit "the commit that deletes it names the occasions the decision held through". Proposal: the occasions held through are the merges whose standing-state review re-read the tripwire (a rewording). Rival: the reviewer judges firing over the whole watch at the horizon, which loses by repeating what each merge did.
- **b. Whether it still applies.** The text is unchanged: `path@agent-skills@content/agents/standing-state-reviewer.md` lines 75 and 76, "For each: has it fired, and has its horizon passed?", and line 84, "A diff that cannot have fired a tripwire is a normal outcome and is reported as one", which judges firing over the diff; `path@agent-skills@fragments/tripwire-lifecycle.md` line 9, the same at `bc2c7036`.
- **c. Whether it is already recorded.** `design@agent-skills@standing-state-review-before-every-merge` (respect) has the reviewer read every tripwire before every merge, so the ranges of the merges since a line was written cover the whole watch; the session's chain is that design. `design@agent-skills@tripwire-horizon` (conflicts, check-reversal) defines the occasions as those of the horizon, counted from the line; the proposal would make "the occasions the decision held through" name merges instead, a change of that head's exit.
- **d. Its kind.** No defect of the text: one reading follows from the reviewer reading its own range at every merge, and the session reached it before writing. No wrong act resulted.
- **e. Its scope.** The installed text, `agent-skills`.
- **f. A better fix.** No change. The proposal renames what the deleting commit lists, from the horizon's occasions to the merges, which changes `design@agent-skills@tripwire-horizon` on one instance where the text's derivation held. The report's own rival loses as it says.
- **g. Its route.** Nothing.

Default: no change.
Outcome: decided by the session: no change. Ground: each merge's standing-state review judges its own range and reads every tripwire, per `design@agent-skills@standing-state-review-before-every-merge`, so "unfired" over the whole watch is the chain of those readings, which the session derived; the proposed rewording would change the exit of `design@agent-skills@tripwire-horizon` with no act that differed.

## standing-2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow: the standing answers

Members: the section "Answers to the standing questions" of
2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow, lines 84 to 128. One answer takes a
section of its own below, the scripted reading of the shipped text's spans. Its counts are in the
lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`.

Carried by a subject, named only:

- the tripwire horizon count (line 100): `tripwire-horizon-counted-by-hand`;
- "neither fired" nearly written from a reviewer's branch-only judgement (lines 88 to 90): W2 of
  this file, `unfired-judged-over-branch-only`;
- "The project's changelog reviewer returned two coverage gaps, stated in its report" (line 109):
  `changelog-axis-range-too-long`, P2 of 2026-11-12-knowledge-architect-release-0-10-0.

Analysed here:

- **A slip of the review skill** (lines 91 to 95): the repair agent's brief "listed the reviewers'
  findings but not a reviewer's coverage notes. So two coverage gaps and three items a reviewer
  declined to call findings went without an outcome, against
  `skill@knowledge-architect-review@what-review-leaves`. The transcript reviewer caught all five, and
  each got its outcome before the merge." Kind: a lapse against a clear instruction, caught by its
  own procedure. One instance.
- **An agent told to read 48 commits and commit the record** found no file to change, returned the
  record as text, "as its brief allowed", and "refused, rightly, to write a reason the brief gave it
  that the record did not support" (lines 110 to 112). **Counts that differed by sorting**, 35 and
  20, "not in the verdict" (lines 115 to 117). **No record search ran**, "since the release decided
  no design" (lines 118 and 119). **The release's own checks scripted as one-line loops**, the
  project's release procedure (lines 101 to 104), the span
  reading among them taking the section below. **Installed file changed**: none. **Project skill
  addition missed**: none. **Tool set up**: none. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: one lapse against a clear instruction of the
review skill, caught by the transcript review it relies on; the rest are observations, and the
release procedure is this repository's own, `skill@klarch-release`.

## standing-2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow: the reading of the shipped text's spans, by a loop

Members: lines 101 to 104: "The release's own checks were scripted as one-line loops: the count of
changelog entries by class, the crates.io status of each crate, the comparison of each package
list, and the reading of the shipped text's spans."

- **Whether it is recorded.** `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`, a
  `todo` entry: "The release procedure, `skill@klarch-release`, checks the shipped text for
  references by hand; the check replaces that step." The release of 0.10.0 carried that step as a
  one-line loop: evidence that the step is mechanical, which the issue's closing condition, a check
  run by `cargo x gates`, assumes. Its searcher routed the answer to the issue.
- **Kind.** A task with no judgement, carried by a script, per the standing question on such tasks;
  the instance is the issue's, not a new one.

Default: add the instance to the issue's "What would close it", as below.

```
The release of 0.10.0 read the shipped text's spans by a one-line loop, among the release checks it
scripted (the standing answers of 2026-11-12-knowledge-architect-release-0-10-0-klarch-workflow).
```

Outcome: decided by the session: handle now, the instance added to
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. Ground: the entry holds the hand
step its check replaces, and a release that scripted the step shows it needs no judgement; the edit
changes no instruction.
