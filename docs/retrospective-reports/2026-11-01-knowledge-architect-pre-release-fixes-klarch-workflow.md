# 2026-11-01-knowledge-architect-pre-release-fixes-klarch-workflow

- **Received file**: 2026-11-01-knowledge-architect-pre-release-fixes-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.8.0-234-geb899523`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`, `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`.

## W1: draft-mode goal review has no stop rule

Analysed with W1 of 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md`.

## W2, with W4 of 2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow: the owner's word read as a goal to draft, an argument, or already stated by a head

- **a.** What it says. Both blame the table of `primer@design-heads`, "an intent an agent cannot
  derive, which states an outcome | a goal", beside "an argument, one that weighs a cost included |
  argued, and recorded on its merits".
  - W2: the owner ruled that a command be usable from a git hook. The session read the ruling as
    already stated by a head and drafted no goal; the decision-record reviewer found that the head
    defeated one rival the ruling defeats and not the other, so the ruling was the only ground
    against the second. The goal was then drafted, at five draft rounds and two more questions.
    Proposal: a test, "an owner's word is a goal to draft when a recorded decision rejects a rival
    on it and no goal or decision states it". +1.
  - W4 of the thaum file: the owner said fuzzing is launched on the side, outside development, to
    find what the test suite does not cover. A reviewer read it as an intent, since it states what
    fuzzing is for, and the session removed it from the head's ground; the owner said it is an
    argument for running fuzzing outside the gates, and it went back. Proposal: the row on intent
    says a statement that argues for a shape the decision has is an argument, whatever it says the
    work is for. 0, a rewording.
- **b.** Whether it still applies: yes. `path@agent-skills@fragments/head-ground.md`, lines 20 and
  21, read the same at `v0.8.0-234-geb899523`, at `v0.9.0` and at HEAD. The fragment is hosted by
  the primer, the decision-recording and design skills, and the decision-record reviewer and
  design-record auditor (`FRAGMENT_HOSTS`, `path@agent-skills@src/lib.rs`).
- **c.** Whether it is already recorded: the table is `design@agent-skills@heads-stand-on-argument`,
  which lists "an intent that states an outcome is a goal" and "an argument ... is argued and
  recorded on its merits", with no test between them. `rejected@agent-skills@owner-intent-entry-test`
  rejected an entry test on the owner's intent, keeping the doubt that "intents moved into goals
  could swell the goals homes with local preferences". No issue records the boundary.
- **d.** Its kind: a defect of the text, unclear: two misclassifications of the owner's words, in two
  projects, each settled only by a review or by the owner. Provable by reading: the row's words,
  "which states an outcome", fit a statement given as a reason for a shape, as the fuzzing sentence
  was, and "an agent cannot derive" gives no act to check whether the record already derives it.
- **e.** Its scope: installed text, the head-ground fragment. Served goal:
  `goal@knowledge-architect@the-owner-decides`, since a misread word sends the owner questions a
  goal draft does not need, or drops a ground the owner gave. The fix refines the head's list within
  its direction; it brings back no rejected design: it routes nothing to a head as intent, and its
  effect, fewer purposes stated as reasons drafted as goals, answers the doubt that keeps
  `rejected@agent-skills@owner-intent-entry-test`.
- **f.** A better fix: one rewording of the intent row for both findings, rather than W2's added test
  and W4's rewording apart: "an intent that no goal and no recorded decision already serves against
  every rival it is cited to defeat, and that states an outcome the project is to reach, not a reason
  for one shape over its rival". The first clause gives W2's check as the meaning of "an agent cannot
  derive" (read per rival, as the reviewer did); the second is W4's. Nearest rival: putting each
  doubtful reading to the owner, which costs a question per reading and is what both sessions
  avoided. The fact that decides: both readings are answered by the record or by the statement's role
  in the decision, which the session can establish, per `primer@question-authority`.
- **g.** Its route: a decision whose design the record does not settle wholly:
  `skill@knowledge-architect-design`, in-change path, since it adds a criterion to a member list of
  `design@agent-skills@heads-stand-on-argument`, then the edit of the fragment.
- **h.** The proposal: the rewording of the row, the head's list brought to it in the same change.
  Instructions: 0 added, 0 removed (a rewording of one table cell).
- **Default**: handle now, on the in-change path of the design skill.
- **Outcome**: decided by the session: handle now. Ground: a two-reading defect provable by reading,
  two instances; the rewording adds no instruction, reverses no head and brings back no rejected
  design.

## W3, with the standing item "D4 written into a head" of 2026-11-04-knowledge-architect-model-crate-klarch-workflow: a default awaiting the owner written into a head before the ruling

- **a.** What it says: `skill@knowledge-architect-decision-recording@when-recording-happens`: "A
  default awaiting the owner is built as written ... and enters a design head once the owner rules on
  it"; `skill@knowledge-architect-design@decision-authority`, the in-change path: "The deliberation
  goes in the message of the commit that writes the decision's design entry". An in-change decision
  held a part put to the owner as a default; the session wrote it into the head in the implementing
  commit; the decision-record re-review found it, and it was removed and written back after the
  ruling. The model crate session, a slice with no discussion, wrote D4's content into a head before
  the ruling; the decision-record reviewer caught it. Proposal (W3): the in-change path points to
  `skill@knowledge-architect-decision-recording@when-recording-happens` for a part awaiting the owner.
  +0.
- **b.** Whether it still applies: yes. `path@agent-skills@content/skills/decision-recording/SKILL.md`,
  lines 32 to 35, and `path@agent-skills@content/skills/design/SKILL.md`, line 154, read the same at
  `v0.8.0-234-geb899523` and at HEAD.
- **c.** Whether it is already recorded: `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`,
  kind `design`: the owner "wants that condition changed so that agents are more autonomous about
  design, defaults and records"; re-entry "a design discussion under `skill@knowledge-architect-design`,
  on the owner's word"; its instance was a rootpoint session's built default narrowing a head. These
  two are its second and third instances.
- **d.** Its kind: a lapse, twice. The condition is delivered at the act: `primer@installed-skills`
  loads `skill@knowledge-architect-decision-recording` "before writing into a design home", and the
  harvest of `skill@knowledge-architect-planning@working-a-slice`, point 6, loads it too. The two
  instances are at two points of delivery, the in-change path and a slice's harvest.
- **e.** Its scope: installed text. The condition is the subject of a design issue the owner wants
  to change; the fix the file proposes delivers the condition as it stands.
- **f.** A better fix: the nearest rival is the file's pointer in the in-change path. It loses: the
  model crate instance is not on the in-change path, so the pointer reaches one of the two; and the
  condition it would deliver again is the one the owner wants changed, so a discussion on the issue
  rewrites it. No change on the text, the issue gaining both instances as evidence of the cost of the
  condition (a head written, removed and written back, in one session), is the better fix.
- **g.** Its route: nothing on the text; an edit of the issue entry.
- **h.** The proposal: the issue gains both instances. Whether to open its design discussion now is
  the owner's word, per its re-entry point.
- **Default**: no change: the issue gains the instances.
- **Outcome**: decided by the session: no change; the issue gains the instances. Ground: the condition
  is delivered at both acts, so each instance is a lapse; the condition's future is
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`,
  which re-enters on the owner's word.

## W4: an issue's closing condition rested on the absence of an event over a period

- **a.** What it says: an entry's "What would close it" read "or this entry closed if no intake runs
  shallow by the next release"; the triage proposed closing it on that clause; the owner corrected
  it: with no intake run at all, the clause shows nothing. The tripwire standard counts occasions for
  a horizon, and nothing like it reaches an issue's closing condition. Proposal: the entry's shape
  says a closing condition is an event or a check, and a period with no event closes nothing unless
  its occasions are counted. +1.
- **b.** Whether it still applies: the instance is repaired: 9c887d03, on the same branch, rewrote
  the clause, "An occasion with no shallow intake closes nothing". The rule is unchanged:
  `path@agent-skills@content/skills/issue-tracking/SKILL.md`, line 102, at `v0.8.0-234-geb899523`
  and at HEAD: "**What would close it**: the measurement, experiment or work that retires the entry".
- **c.** Whether it is already recorded: no entry.
- **d.** Its kind: a lapse, once. The field's definition admits a measurement, an experiment or work;
  an absence over a period is none of the three, so the clause did not meet the field as written.
- **e.** Its scope: installed text, the issue-tracking skill. Reverses nothing.
- **f.** A better fix: the rival is the file's +1. It loses on one instance: a conformance rule is
  worth text only where the default behaviour is systematically wrong, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`, and the existing definition
  already excludes the clause.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: one instance; the field's definition,
  "the measurement, experiment or work", excludes a period with no event; the instance's entry was
  repaired in 9c887d03.

## W5: the review skill's "saves each report whole" not followed

Analysed with W1 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.

## C1: a partial commit of shipped-text sources fails the staged check in a project that builds its checker from source

- **a.** What it says: twice, a commit of part of the edited shipped-text sources failed
  `check --staged` and `index --staged` on installed copies of sources left unstaged, though
  `commits` passed the commits; the session folded unrelated text changes into one commit. It
  states that the defect is recorded.
- **c.** Already recorded: `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`,
  whose reproduction is this case, with one further instance, of the session of 2026-10-25. It does
  not hold this one. Its file at HEAD was last changed by the slice that renamed the core (the
  commit "[review] Repairs of the first axes of the review before the merge of the slice that renames
  the core"), so the defect stands.
- **Default**: no change, with the reason: recorded; the issue gains the instance.
- **Outcome**: decided by the session: no change to the code here; the issue gains this instance,
  with the cost it names, unrelated text changes folded into one commit. Ground: the entry records
  the defect and its closing shapes; an instance adds to its evidence, and this intake, which edits
  installed text, may meet it again.

## C3: no command moves a commit to the branch's front

Analysed with P2 of 2026-11-01-knowledge-architect-narrowed-goals, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals.md`.

## C2: no lighter record search for one in-change decision

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## The standing answers

- **a.** Items:
  - Installed file: sources edited and reinstalled. An observation.
  - The relayed count and the search hits: analysed with the claim-check cluster of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - The attribution in a commit message: analysed with the attribution cluster of
    2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow, in
    `path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.md`.
  - No pointer written bare.
  - Hand tasks: twelve searchers dispatched with identical briefs is the searcher's dispatch,
    cluster L (G1), home C2 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow;
    the report save is W5 of this file, cluster D (G3); moving a commit to the front and waiting on
    CI are cluster H (G6), with C3 of this file.
- **Default**: no change.
- **Outcome**: decided by the session: no change for the observation analysed here. Ground: no
  text is blamed.

---
