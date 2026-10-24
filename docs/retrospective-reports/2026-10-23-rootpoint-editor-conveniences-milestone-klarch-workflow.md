# 2026-10-23-rootpoint-editor-conveniences-milestone-klarch-workflow

- **Received file**: 2026-10-23-rootpoint-editor-conveniences-milestone-klarch-workflow.md, the
  workflow file of a session in rootpoint that implemented the six remaining slices of a milestone in
  one branch, at the owner's request, with defaults accumulating until the owner approved them all,
  compacted at least once.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`. The installed text this file blames is the
  same at both; the core's SHA scan moved in its file and kept its text.
- **Standing entries that bear on it**:
  - for W1: `design@agent-skills@head-ground-is-the-argument`,
    `design@agent-skills@unargued-approval-is-argued`, `design@agent-skills@owner-rules-at-a-fork`,
    `rejected@agent-skills@delegated-decision-state`, `tripwire@agent-skills@owner-intent-stripped`,
    not fired: the fork defaults kept "the owner chose";
  - for W2: `design@agent-skills@divergence-routed-by-its-authority`,
    `design@agent-skills@head-prevails-over-diverging-code`,
    `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`;
  - for W3: `design@agent-skills@plan-document-leaves-at-landing`,
    `design@agent-skills@milestone-is-a-directory`, `design@agent-skills@default-ruled-before-merge`,
    `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`,
    `tripwire@agent-skills@expectation-set-closes-a-contradiction`, not met: no outcome below closes a
    contradiction by an expectation set;
  - for W5: `design@agent-skills@document-vocabulary`,
    `issue@agent-skills@expectation-sets-for-the-installed-skills`;
  - for W6: `design@agent-skills@undesigned-work-is-an-issue`, `design@core@issue-kind-list-closed`;
  - for W8: `design@agent-skills@transcript-reviewer-agent`;
  - for C1: `design@core@branch-shas-are-refused`, `design@knowledge-architect@no-branch-sha-is-cited`,
    `issue@core@branch-sha-citations-are-judged-within-the-range-only`;
  - for C2: `design@core@git-supplies-the-walk`, `design@core@finding-names-the-repair`.

## W1. Is an owner's approval of a listed default a ground, or only an approval?

- **a. What it says.** The owner answered six defaults listed as `D<n>` with "all defaults approved".
  Some had been put as forks, where a rival survived every recorded argument; others as scope
  changes, whose rival lost to a recorded decision. The session first wrote "On the owner's word: the
  owner chose …" for all; the decision-record reviewer split them, "the owner chose" for the forks
  and an argued head for the scope changes. The primer's example phrase, "all defaults approved",
  made the first reading look forbidden. The fix proposed: after "An approval is not a ground", an
  approval of a default put as a fork is the owner's choice and a ground, and the plan document says
  of each default whether it was put as a fork.
- **b. Whether it still applies.** Yes. `primer@design-heads`: "An approval is not a ground, whatever
  its words ("approved", "agreed", "all defaults approved", "accepted the cost")", and the owner's
  words are a ground "only where the decision came from the owner: the owner proposed it, the owner
  chose where the argued rivals did not settle it, or it rests on a premise only the owner can
  state". The decision-recording skill already says, for an approval no argument stands behind:
  "When the search leaves the rivals equal, put the fork to the owner as a tie. The owner's choice is
  then the head's ground". The planning skill lists forks and scope changes in one list of defaults,
  with no field telling them apart.
- **c. Whether it is already recorded.** `design@agent-skills@unargued-approval-is-argued`: "Where the
  rivals come out equal, the fork goes to the owner as a tie, and the owner's choice is the ground".
  `rejected@agent-skills@delegated-decision-state`: "A word that hands judgement over is an approval
  like any other, as is approving every default on a quick read." The design skill's expectation set
  assumes that "The owner reads the summary tables and contests what is wrong in them".
- **d. Its kind.** Unclear: a batch word answers several defaults at once, and the primer's example
  list reads as covering a fork the owner chose by it.
- **e. Its scope.** `goal@knowledge-architect@the-owner-decides`; installed text, and the head on a
  head's ground, which a tripwire guards.
- **f. A better fix.** Rivals: the proposal's sentence; a pointer from the primer's example to the
  decision-recording sentence on a tie, which already answers it; leave it to judgement, on which the
  reviewers of this session disagreed. The second adds no rule: a default put as a fork is a tie
  already put, and its approval is the owner's choice. The marking of each default as a fork is a
  structure the reviewer did without, since the plan document's argument for each default shows it.
  The edit reads against `design@agent-skills@head-ground-is-the-argument`, so it goes through the
  design skill's in-change path.
- **g. Its route.** A decision, under the in-change path of `skill@knowledge-architect-design`, then
  an installed text edit of the primer.
- **h. The proposal.** Handle now: the primer's sentence says that the approval of a default put to
  the owner as a fork, where the rivals came out equal, is the owner's choice, per the tie rule of the
  decision-recording skill. No marking of defaults.
- **Default**: handle now, as h says.
- **Outcome**: I do not agree with that. I want such a case to not be attrributed to me. when i approve a default, i have not analyzed the proposal deeply. i do not want such a decision attributed to me long term. maybe there should be a path that lets an agent attribute to itself the decision, explicitly in the design head.

## W2. What a design head says while a built default narrows its rule and awaits the owner

- **a. What it says.** Three defaults narrowed rules that existing heads stated; the code built the
  narrower default, so the head and the code diverged until the ruling. The session tried three
  shapes and reviewers disagreed on which was right. The fix proposed: a built default that narrows an
  existing head leaves the head unchanged, and a `question` issue citing the default records the
  divergence until the ruling.
- **b. Whether it still applies.** The texts stand. `skill@knowledge-architect-decision-recording@when-recording-happens`:
  "A default awaiting the owner is built as written … and enters a design head once the owner rules
  on it." `primer@intent-and-claims`: a divergence "never closes by following the code. Work on that
  point that goes on meanwhile builds on the head." The file leaves out the planning skill's sentence
  that bears on it, in working a slice: "A scope change listed there does not start before its
  ruling", where a scope change is "An answer that widens or narrows a ruling of the owner". So a
  default that narrows a rule the owner ruled is not built before the ruling; one that narrows a head
  standing on its argument contradicts it, and the backstop sends it to the design skill.
- **c. Whether it is already recorded.** In part. `design@agent-skills@divergence-routed-by-its-authority`
  already gives the proposal's shape, "the code is the defect: the session fixes it or opens an issue,
  and the work builds on the head". `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`,
  a `question`, asks whether work on the point in divergence waits or goes on.
- **d. Its kind.** Unclear where the texts meet, with a sentence the session did not apply: which of
  the two cases a default is depends on the ground of the head it narrows, which the file does not
  say for the three.
- **f. A better fix.** Rivals: the proposal, which the divergence head already states; the planning
  sentence and the backstop, which already route each case; the open issue, whose question this case
  answers in real use. The record answers each case; the open question is the primer's paragraph.
- **g. Its route.** An edit of the issue entry.
- **h. The proposal.** Handle now: the open issue gains this case, with the two readings and the
  planning sentence that holds a scope change until its ruling.
- **Default**: handle now, as h says.
- **Outcome**: tbh, i want to change the condition of `skill@knowledge-architect-decision-recording@when-recording-happens`. i want agent to be more autonomous about design, defaults and records, and move more of the owner intent to goals. this should go to a design issue along with the above entry too .

## W3. A milestone whose last slice lands while defaults still await the owner

- **a. What it says.** The planning skill has the milestone document leave with the last slice, and
  every default it lists ruled before it is deleted; the last slice landed with six defaults unruled.
  The session kept the document, named in its harvest row the commit recording the rulings as the
  one that deletes it, and moved the last report on the acceptance criteria there. Reviewers accepted
  it. The fix proposed: in that case, the document leaves in the commit that records the last ruling.
- **b. Whether it still applies.** `skill@knowledge-architect-planning@plan-document-leaves`: "**A
  plan document is deleted in the commit that completes its last harvest** … the milestone document
  leaves with the last slice", and "every default the document lists as awaiting the owner is ruled,
  since a design head records such a default only once the owner has ruled on it". Read with the
  decision-recording skill, the last harvest is complete only once the rulings are recorded, so the
  first sentence already admits what the session did; "leaves with the last slice" reads the other
  way, here and in `design@agent-skills@milestone-is-a-directory`, "The README leaves with the last
  slice".
- **c. Whether it is already recorded.** `design@agent-skills@plan-document-leaves-at-landing`,
  deleted "in the commit that completes its last harvest", agrees with the session.
  `design@agent-skills@default-ruled-before-merge` keeps the slice from merging earlier. The moment of
  a plan document's deletion is also the subject of the deferred issue whose trigger W1 of
  2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow meets.
- **d. Its kind.** Unclear: one phrase, at two sites, contradicts the rule it abbreviates.
- **f. A better fix.** Rivals: the proposal's sentence for the case; the phrase brought to the rule
  at both sites, "in the commit that completes its last harvest, which records the ruling of every
  default it lists", which changes no head and adds no case. The deferred issue's design decision
  sets the same moment for a spec, so the phrase is reworded with that decision, in the same branch.
  Instructions added: 0.
- **g. Its route.** An installed text edit, with the head's body reworded, under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Handle now: the phrase reworded at both sites, with the decision that W1 of
  the defined-ids workflow file sends to the design skill.
- **Default**: handle now, as h says.
- **Outcome**: proposal accepted, and note this case in the design issue above.

## W4. Rewording a ruled tripwire's clause against widening it

This finding is analysed with W3 of 2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow,
in `path@knowledge-architect@docs/retrospective-reports/2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow.md`.

## W5. How "review as if before the merge" is scoped when every slice shares one branch

- **a. What it says.** With every slice in one branch, by the owner's direction, each slice's review
  before the merge read the whole branch again; the session weighted each brief on the newest slice's
  range and ran the mechanical checks over the whole range. The planning skill states no expectation
  set, so the file reports a gap. The fix proposed: one sentence for slices that share a branch.
- **b. Whether it still applies.** The planning skill defines a slice as "a part of a milestone that
  is one branch and one PR", and asks for "**One branch and one merge per slice**".
  `skill@knowledge-architect-review@review-axes`: "The review before a merge is the last review of a
  branch … the work it reviews is the whole branch." So on one shared branch only the last slice's
  review is the review before a merge, and the earlier ones are reviews of the work.
- **c. Whether it is already recorded.** `design@agent-skills@document-vocabulary` argues the word
  slice from "one branch and one merge per part".
  `issue@agent-skills@expectation-sets-for-the-installed-skills` records that the planning skill has
  no expectation set.
- **d. Its kind.** The owner's direction set aside one rule of the planning skill, and the review
  skill's own sentence decides what follows; no instruction is missing for a move the session could
  not make.
- **f. A better fix.** Rivals: the proposal's sentence, a rule for one direction of the owner's; an
  expectation set for the planning skill that names one branch per slice, the open issue's work; no
  change.
- **g. Its route.** An edit of the issue entry.
- **h. The proposal.** Handle now: `issue@agent-skills@expectation-sets-for-the-installed-skills`
  gains this case, an assumption of the planning skill that one slice is one branch.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W6. The kind of issue for a later consequence that waits on an event

- **a.** A consequence waiting on an owner's ruling took a `deferred` issue, against the letter of
  "a `todo` issue". The fix proposed: "a `todo` issue, or a `deferred` one where an event gates the
  work".
- **b.** `skill@knowledge-architect-planning@plan-document-leaves`: "or in a `todo` issue opened for
  it where that work has neither". The issue-tracking skill defines `todo` as work "no event gates",
  so the sentence contradicts it for gated work. The planning skill already has the proposed wording
  at another site: "It is a `todo` issue, or a `deferred` one if an event gates it".
- **d.** Wrong, provable by reading: the sentence names a kind the issue-tracking skill's definition
  excludes for this case.
- **f.** The skill's own wording at its other site, copied. Instructions added: 0.
- **g.** An installed text edit.
- **h. The proposal.** Handle now: the sentence takes the skill's other wording.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W7. Reviewers' reports saved as summaries

This finding is analysed with W4 of 2026-10-21-knowledge-architect-init-and-new-component-klarch-workflow,
in `path@knowledge-architect@docs/retrospective-reports/2026-10-21-knowledge-architect-init-and-new-component-klarch-workflow.md`.

## W8. The transcript extraction may drop some report bodies, observed by a reviewer and not verified

- **a.** The transcript reviewer reported that three injected reports of subagents extracted with
  empty bodies, though their text exists in the log, and named it a content shape the extraction may
  not name. The session did not check.
- **b.** `agent@knowledge-architect-transcript-reviewer`, its extraction section, keeps the text of
  `user` and `assistant` lines, a string or the `text` items of a list, and the text of
  `queued_command` attachments, and labels injected lines by their fields. Measured in this intake's
  session, on the logs of this machine: every line carrying a subagent's report in the logs of that
  project, 64 of them, is a `user` line whose content is a string, `isMeta` true and `origin.kind`
  `peer`; in this session's own log, the same, and a `queued_command` attachment whose `prompt` is a
  string. The extraction keeps all of them, with their text.
- **c.** Not recorded.
- **d.** An observation not reproduced: no report line of the measured shapes extracts empty. The
  measurement does not identify which session the file reports.
- **h.** No change.
- **Default**: no change, not reproduced.
- **Outcome**: accepted.

## C1. `cargo klarch commits` passes a SHA whose commit was amended away

- **a. What it says.** A commit message on the main branch cites the short SHA of a commit the same
  branch later amended; that SHA names a commit no branch reaches, and `commits` passed. The fix
  proposed: report a SHA in a message that names no commit reachable from the range's tip or from the
  main branch.
- **b. Whether it still applies.** Yes, measured by a checking agent: a commit amended, then a commit
  citing its old SHA, passes `commits`; a citation of the new SHA is refused. The scan, in
  `path@core@src/cli/history.rs`: "Only the range's commits are refused".
- **c. Whether it is already recorded.** Yes, as a decision. `design@core@branch-shas-are-refused`: "A
  citation of a commit the branch rewrote before `commits` ran is not in the range, and is not seen:
  no clone can tell reliably which commits a branch once held, since a fresh clone, such as CI's,
  never fetched them", and "A SHA on the main branch, or of another project, is not in the range and
  passes".
- **d. Its kind.** A blind spot the head states on purpose, with its reason.
- **f. A better fix.** Rivals: the proposal, which would refuse every SHA of another project and
  every hexadecimal word that names no commit here, in every document as in every message; a local
  warning for a word that resolves to a commit unreachable from the tip and from main, which works
  only while the object survives in the local store and is no gate in CI's clone, the head's reason;
  no change. The head's argument covers both rivals.
- **h. The proposal.** No change.
- **Default**: no change, as c says.
- **Outcome**: accepted.

## C2. A build directory inside a worktree that git does not ignore fails `cargo klarch check`

- **a.** Reviewers told to keep their build inside their worktree picked a directory the project
  does not ignore; the check failed on untracked output. The fix proposed: a hint for an untracked
  file under a build directory.
- **b.** The hint exists: the checker prints, after the findings, "N files above are untracked … the
  walk reads every file git does not ignore, tracked or not: commit a file that belongs to the
  project, and move out or ignore one that does not", measured by a checking agent with untracked
  build output, and present since before 0.3.0. The review skill already says where a subagent's
  build goes: "sets `CARGO_TARGET_DIR` to a directory inside its worktree that the project's ignore
  rules cover, such as its `target/`".
- **c.** `design@core@finding-names-the-repair`: "Where one repair is correct for every cause, the
  finding names that repair and no cause"; build output and a stray file take the same repair.
- **d.** Already delivered: a lapse of the briefs against the review skill.
- **h.** No change.
- **Default**: no change, as d says.
- **Outcome**: accepted.

## The standing answers

- **a.** Rules acted on as if absent, each caught before the merge: the owner quoted in a commit
  message; four false claims in commit messages; approvals first written as the owner's ground (W1);
  a plan's row recording a case "as the owner approved" that the approval never named, put to the
  owner as a default.
- **d.** Lapses. The claims and the quotation are instances of the recurrence and of the attributions
  analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`;
  the approvals are W1 above.
- The ruling is given where each is analysed.
