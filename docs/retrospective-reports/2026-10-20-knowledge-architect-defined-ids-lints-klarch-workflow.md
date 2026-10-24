# 2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow

- **Received file**: 2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow.md, the
  workflow file of this repository's session that carried out the spec adding lints on ids, its
  reviews and its merge.
- **Version used**: built from its own source, main at `v0.7.0-77-g67d36d1`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`.
- **Standing entries that bear on it**:
  - for W1: `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`,
    whose trigger this file meets; `design@agent-skills@plan-document-leaves-at-landing`;
  - for the cluster under W3: `design@agent-skills@tripwire-wording-is-the-agents`,
    `design@agent-skills@premortem-tripwires-on-the-owners-word`,
    `design@agent-skills@premortem-tripwire-written-at-harvest`,
    `design@agent-skills@decisions-reported-to-the-owner`,
    `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`,
    `tripwire@agent-skills@ruling-lost-in-assembly`;
  - for C3: `design@core@checked-sentinel-values`, `design@core@owned-namespace-check`,
    `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`;
  - for C4: `issue@core@an-unbound-literal-is-read-up-to-its-first-escape`.

## W1. Real use of the deleted-plan gap, on three of its four cases

- **a. What it says.** Every brief of the review before the merge, and of the re-review, had to send
  its reviewer to the spec with `git show <commit>^:<path of the spec>`; a `filter-branch` reword of
  the harvest message between the two gave the deleting commit a new SHA, so the first briefs named
  a commit no longer on the branch. The standing-state reviewer called the extended brief the issue's
  repair applied by hand; the spec-conformity reviewer noted that the review's repairs were written
  into the heads and rewritten there, the spec being gone; `commits` refused the next review
  message, which cited the spec by its reference. The fix proposed: add this session to the issue as
  its evidence of real use, so that its structural repair can be designed.
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-planning@working-a-slice`, the same
  at `67d36d1` and on main but for unrelated edits: "**The harvest's review is the review before the
  merge**", and "**The slice's spec leaves** in the commit that completes its harvest". The commit
  that deleted the spec precedes four review commits on main.
- **c. Whether it is already recorded.** Yes, `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`,
  `deferred`, which says "no real session has shown any of them". Its trigger: "a review before a
  merge shows a reviewer reporting, or judging without, a spec the range deletes", or "an intake of a
  retrospective's file … reads a real session in which one of the four cases occurred … deciding a
  repair of the installed text from real use is that occasion's work". Both are met. Its two repairs,
  either decided under `skill@knowledge-architect-design`: "**The structure removed**", the spec
  deleted in a commit of its own after the review before the merge has read the tip that holds it,
  which closes all four cases; "**The briefs extended**", which closes cases 1 to 3. One fact the
  issue lacks: a brief that names the deleting commit by its SHA goes stale at the first history edit
  of the branch, which the review's own repairs often need.
- **d. Its kind.** A recorded gap whose trigger fired, now with real use.
- **e. Its scope.** `goal@knowledge-architect@agents-work-without-drift`,
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`; installed text, the planning and
  review skills and `primer@design-heads`. The structural repair changes
  `design@agent-skills@plan-document-leaves-at-landing`.
- **f. A better fix.** Rivals, as the issue names them; the stale SHA weighs against the briefs
  extended, whose reviewers find the spec through a commit the branch's own history edits move.
  The milestone case of W3 of 2026-10-23-rootpoint-editor-conveniences-milestone-klarch-workflow, a
  milestone kept after its last slice until its defaults are ruled, is the same deletion moment met
  from another side. The choice changes a head, so it is a design question.
- **g. Its route.** A decision, under `skill@knowledge-architect-design`.
- **h. The proposal.** Handle now: the issue's trigger fired; its repair is decided under
  `skill@knowledge-architect-design`, with the structure removed as the default for its closing
  of every case, and this session and the stale SHA added to the issue as its evidence.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W2. Where the command that re-takes a measurement lives when the instrument was a one-off script

This finding is analysed with P3 of 2026-10-20-knowledge-architect-defined-ids-lints, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-20-knowledge-architect-defined-ids-lints.md`.

## W3, with the findings on what of a ruled tripwire is the session's

This section analyses W3 of this file, W1 and W2 of
2026-10-21-thaum-reader-refusals-slice-klarch-workflow, W4 of
2026-10-23-rootpoint-editor-conveniences-milestone-klarch-workflow and W2 of
2026-10-23-thaum-driven-testing-setup-deviations-klarch-workflow: four sessions of three projects,
one question.

- **a. What they say.**
  - W3 here: a tripwire's premise sentence, written on the owner's ruling, dropped the owner's hedge.
    The session brought it back to the owner's words itself, did not put it to the owner, and did not
    list it under "Decided". Proposal: `primer@decided-section` says "a rewording of a tripwire's
    text, its premise included".
  - W1 of the reader-refusals file: the owner ruled a direct action as a tripwire's response; a
    reviewer flagged the shape, the session reworded it to "open a `todo` issue, closed by that
    action", and the transcript reviewer reported the rewording as a change of the owner's ruling,
    which went back to the owner. Proposal: a sentence in
    `skill@knowledge-architect-decision-recording@ruled-tripwires` that the premortem puts a response
    to the owner in the issue-tracking skill's shape. W2 of the same file: a firing clause restricted
    to one source of evidence, one instance, no proposal.
  - W4 of the editor-conveniences file: new behaviour added members to what a tripwire's decision
    covers; the standing-state reviewer proposed widening the firing clause as a "Decided" rewording,
    the decision-record reviewer judged it the owner's, and the transcript review caught the session
    widening it under the original ruling's line. Proposal: a change of what a ruled tripwire fires on
    goes to the owner.
  - W2 of the setup-deviations file: a slice spec's harvest row rewrote an existing tripwire with a
    second firing clause no ruling covered, of a shape the milestone's own premortem had dropped; the
    session followed the row and two reviewers reported it. Proposal: point 6 of
    `skill@knowledge-architect-planning@working-a-slice` sends a tripwire the row names to
    `skill@knowledge-architect-decision-recording@ruled-tripwires`, and an unruled firing clause to
    the owner.
- **b. Whether they still apply.** The texts are unchanged at `v0.8.0` and on main.
  `primer@decided-section`: "a rewording of a tripwire's clauses are such decisions".
  `skill@knowledge-architect-decision-recording@ruled-tripwires`: "**A tripwire is written only on
  the owner's word, and with the decision it guards**", and nothing on rewording one.
  `skill@knowledge-architect-issue-tracking@tripwire-entry`: "the response: reopen the decision by
  its reference, or open an issue of a named kind". Point 6 of working a slice harvests "the
  tripwires and the issues under `skill@knowledge-architect-issue-tracking`". The one installed
  sentence that draws the line is in the design skill's premortem step: "The owner's word rules on
  whether a cause is watched, not on the wording of the tripwire that watches it; … Its firing
  evidence, its response and its re-entry point may be reworded when it is written, or on a review
  finding, on your judgement, and each change is listed in the turn's "Decided" section … A tripwire
  that watches another cause is a new one, and needs the owner's word." The transcript reviewer's
  definition lists the session's own decisions and not a tripwire's rewording, and its scope rule,
  "with no obligation added that the owner did not rule", reads a reworded response that adds an
  issue as an added obligation.
- **c. Whether they are already recorded.** Yes, as the decision:
  `design@agent-skills@tripwire-wording-is-the-agents`, "The tripwire's firing evidence, response and
  re-entry point are reworded when it is written, or on a review finding, on the agent's judgement,
  and each change is listed in the turn's "Decided" section … A tripwire that watches another cause
  is a new one, and needs the owner's word … The rival, a change to a tripwire's firing evidence
  treated as a material finding, puts each such repair back to the owner." W4's proposal is that
  rival. `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` waits for the next change
  to the tripwire shape of the issue-tracking skill, which the proposal below does not make.
  `tripwire@agent-skills@ruling-lost-in-assembly` fires on a ruling missing or misstated in a plan
  document "where the misstatement is critical"; the harvest row's clause was caught before any
  work rested on it, so it did not fire.
- **d. Their kind.** Unclear, from one cause: the line the head draws, the cause watched against the
  wording, is delivered at the premortem only, and the four moments a tripwire is reworded later,
  the "Decided" section, the harvest, a review's repair and the transcript review, read texts that
  do not carry it. Three reviewers and three sessions read those texts two ways.
- **e. Their scope.** `goal@knowledge-architect@the-owner-decides`,
  `goal@agent-skills@no-contradiction-in-workflow`; installed text.
- **f. A better fix.** Rivals:
  - W4's proposal, every change of what fires to the owner, which reverses the head and its argument
    that the owner rules only on what the goals and the record leave open;
  - the four site proposals, each a rule written for one interaction;
  - the head's one sentence delivered, with its pointer, where a tripwire is reworded after the
    premortem: in `skill@knowledge-architect-decision-recording@ruled-tripwires`, which the harvest
    reaches through point 6 once point 6 points there for a ruled tripwire, and in the transcript
    reviewer's list of the session's own decisions; and the primer's "Decided" member read as "a
    rewording of a tripwire's text, its premise included, that keeps the cause it watches".

  The third closes all five: W3's premise rewording is the session's and goes under "Decided"; W1's
  reworded response is the session's, and the transcript reviewer reads it so; W4's widening is the
  session's where the new members are the same cause; W2 of the setup-deviations file is another
  cause, a new tripwire, so the owner's. A restatement of one sentence beside its pointer is what
  `primer@where-knowledge-goes` admits. Instructions added: 0; one sentence restated at two sites,
  and a pointer added to point 6.
- **g. Their route.** An installed text edit under `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent"; it delivers the head and changes no decision.
- **h. The proposal.** Handle now: the third rival; no change for W2 of the reader-refusals file, one
  instance; the tripwire `tripwire@agent-skills@ruling-lost-in-assembly` not fired.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W4. The losing-alternative tests misapplied once, repaired by a history edit

- **a.** The harvest named a rejected shape as failing test 3 where test 1 holds; the
  decision-record reviewer caught it, and the repair added a `rejected` entry and reworded the
  harvest message by `filter-branch`. A lapse, recorded for its cost.
- **b.** The instruction is unchanged, point 6 of working a slice; the entry is
  `rejected@core@plan-items-exempt-from-the-grammar`.
- **d.** A lapse, caught before the merge.
- **h.** No change.
- **Default**: no change, a lapse.
- **Outcome**: accepted.

## C1. No way to print the verdict line alone

This finding is analysed with P1 of 2026-10-20-knowledge-architect-defined-ids-lints, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-20-knowledge-architect-defined-ids-lints.md`.

## C2. Under the new lints, a pointer described in words is the free repair

- **a.** Twice, repairing 42 findings on this repository's text turned a pointer into a description
  in words, and lost the check; two axes caught both. Recorded for the next intake of an upgrade.
- **b.** `design@agent-skills@plain-text-is-no-repair` places "a description in words" outside its
  exception, unchanged.
- **c.** `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` covers pointers
  bare in the checker's syntax, not descriptions in words. The intake of the first upgrade to these
  lints in another project, 2026-10-20-thaum-pin-move-0-8-0-klarch-workflow, reports no such repair.
- **d.** An observation; the two cases were caught before the merge.
- **h.** No change.
- **Default**: no change, an observation.
- **Outcome**: accepted.

## C3. The stale-install finding names a released version under the `"self"` pin

- **a. What it says.** The finding read "this installed file differs from what version 0.7.0 ships
  for this project" under the `"self"` pin, where the text compared is what the current build ships.
  The fix proposed: under `"self"`, "what this build ships".
- **b. Whether it still applies.** Yes. `path@core@src/check/agents.rs` builds the message from the
  checker's own package version and never reads the pin; two sibling messages of the same function,
  for a missing and an unshipped file, use the same words. Under `"self"` on main it prints 0.8.0,
  while main is fourteen commits past the release and its shipped text differs from the release's.
  Read from the code: a `"self"` pin is refused outside this tree, per
  `design@core@checked-sentinel-values`, so a run in a copy cannot show it.
- **c. Whether it is already recorded.** No.
  `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text` quotes the same
  message about another defect.
- **d. Its kind.** Wrong, provable by reading: under `"self"` the message names a version whose text
  is not the text compared.
- **e. Its scope.** The checker; only this repository uses the `"self"` pin.
- **f. A better fix.** Rivals: the three messages say "what this build ships" under `"self"` and
  keep the version under a pinned one, the proposal applied to the siblings; no change, since only
  this repository meets it, at the cost of a message that misleads in the one tree the sentinel is
  for.
- **g. Its route.** A change to the Rust source, under `skill@klarch-development`.
- **h. The proposal.** Handle now: the three messages name the build under `"self"`.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C4. An unbound Rust string literal is read only up to its first escape

- **a, b, c.** Reproduced by the session, and recorded as
  `issue@core@an-unbound-literal-is-read-up-to-its-first-escape`; the code still takes the first
  content child only.
- **h.** No change: already recorded.
- **Default**: no change, already recorded.
- **Outcome**: accepted.

## The standing answers

- **a.** Rules acted on as if absent: the chained pipe, five times; "Decided" missing one rewording
  of a tripwire's premise; two hedges of the owner dropped, caught before the merge; a
  decision-record reviewer that ran `cargo klarch check` against its own definition.
- **b.** The reviewer's definition says "Do not run it", unchanged.
- **d.** The chained pipe is analysed with P1 of 2026-10-20-knowledge-architect-defined-ids-lints,
  the "Decided" lapse with W3 above, and the hedges with the attributions in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`.
  The reviewer's run is a subagent's lapse, one instance, which cost a run and changed nothing.
- **h.** No change for the reviewer's run.
- **Default**: no change, as h says.
- **Outcome**: accepted.
