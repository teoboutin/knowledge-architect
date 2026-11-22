# When a default awaiting the owner enters a head, and what a durable record attributes

## Status and audience

This spec plans the work of one branch, which changes installed text and the
agent-skills design home. It leaves when its work lands, in a commit of its own after the review
before the merge. Where it and a design home disagree on a point that "What is already decided"
does not list as reversed or rewritten, the design home wins. Every name it uses is defined in it
or exists in the tree. Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice`
says what waits.

It was written for a session that did not see the discussion. The discussion is one session,
which begins at the owner's message "Right now, the primer says a default put to me “is built”.
The decision-recording skill says its head waits for my ruling." Its rounds, each named by the
opening words of the owner's message:

1. "Right now, the primer says a default put to me"
2. "For context, this waiting for the head is an artefact"
3. "Criteria "the review before the merge judges every head"
4. "approval-left-unrecorded: if my attribution is not recorded"
5. "Nothing more to context"
6. "Table approved, write the spec"

The threads and the arguments are in [deliberation](deliberation.md); the record search's
generated lines in [the discussion's search](search/discussion-1.txt); the transcript in
[the transcript](transcripts/1.txt).

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, "The work of a spec". Every edit of
installed text follows `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", read
whole before the first edit.

## Names

- **the fragment**: `path@agent-skills@fragments/head-ground.md`, included by
  `{{fragment:head-ground.md}}` in `path@agent-skills@content/PRIMER.md` (`primer@design-heads`),
  `path@agent-skills@content/skills/decision-recording/SKILL.md`,
  `path@agent-skills@content/skills/design/SKILL.md`,
  `path@agent-skills@content/agents/decision-record-reviewer.md` and
  `path@agent-skills@content/agents/design-record-auditor.md`.
- **the shape fragment**: `path@agent-skills@fragments/head-shape.md`.
- **the old form**: an attribution in a durable record that this work removes: "the owner approved
  it", an argument attributed ("the owner's proposal", "the owner's argument"), a choice attributed
  apart from the ranking it made.
- **an underivable premise**: a premise a session cannot derive from inside the project: a fact, the
  ranking of a cost (a fork's choice, a threshold set, a choice among the readings a source
  permits), a judgement of what is plausible or of expected use, and an act a goal reserves to the
  owner, such as a take-in under `goal@generic-tools-and-procedures-are-shipped`.
- **a durable record**: a design head, a `rejected` entry, a tripwire, an issue entry. A plan
  document's items and a commit message are records of the deliberation, not durable records, in
  this spec.

## What the work is

What exists today, each read at main `3c270857`:

- The fragment's table "what the owner brings" records an approval at a fork as "the session chose
  X over Y, which no goal ranks; the owner approved it", and a pick of the owner as "the owner chose
  X over Y, which no goal ranks". Its weak form opens "A fork's choice, the owner's or the
  session's". Its paragraph "A durable record attributes a decision to the owner; it does not quote
  the owner" lists a head, a commit message, a tripwire, a `rejected` entry, an issue entry and a
  plan document's items, and writes "the owner chose X over Y" or "on the owner's word". The
  same paragraph holds the exception that lets an issue entry keep the owner's words verbatim on
  the owner's request.
- `path@agent-skills@content/skills/decision-recording/SKILL.md`, `skill@knowledge-architect-decision-recording@when-recording-happens`:
  "A default awaiting the owner is built as written, per
  `skill@knowledge-architect-planning@working-a-slice`, and enters a design head once the owner rules
  on it. Until then it is no approved decision, and a head is read against the owner's approval
  before it is written". Its `skill@knowledge-architect-decision-recording@unargued-approval`: "Before a head is written from an approval that
  no argument for the decision, no rival and no cost stand behind".
- The shape fragment: "A head written from a decision the owner approved is read against that
  approval before it is written", and a conformance "that would relax or turn what the owner's
  words in it approved" goes to the owner as one proposal.
- `path@agent-skills@content/skills/planning/SKILL.md`, `skill@knowledge-architect-planning@plan-document-leaves`: "every default the
  document lists as awaiting the owner is ruled, since a design head records such a default only
  once the owner has ruled on it"; `skill@knowledge-architect-planning@acceptance-criteria`: "`primer@design-heads` records the
  owner's approval of it as the session's choice, and a value the owner sets as the owner's".
- `path@agent-skills@content/agents/transcript-reviewer.md`, `agent@knowledge-architect-transcript-reviewer@check-recorded-rulings` and `agent@knowledge-architect-transcript-reviewer@how-to-report`:
  "an argument attributed to the owner is the owner's", and the minor severity "for an argument
  attributed to the owner that is not the owner's".
- `path@agent-skills@content/skills/project-audit/SKILL.md`, the design-record axis's outcome table:
  "an approval cited as a head's ground, removed when the head stands on its argument, or recorded as
  the session's choice at a fork".
- `path@agent-skills@content/skills/goal-setting/SKILL.md`: a `%%` comment ending "which no goal
  ranks; the owner approved it".
- The design homes, tripwires and issues of this repository hold the old form. By
  `grep -rn "<phrase>" crates/*/docs tools/*/docs docs --include=*.md | grep -v docs/plans/` at main
  `3c270857`: "the owner approved it" 21 lines in 11 files, "the session chose" 20 in 14, "the
  owner chose" 40 in 29, "the owner's proposal" 25 in 13, "the owner's argument" 31 in 18, "the
  owner reported" or "the owner's report" 45 in 22. Part of these are underivable premises, which
  keep their attribution.

The deferral of a head was written on 2026-10-12, in the commit "The findings the retrospective
analyses ruled to handle now are handled", when the primer made the owner's words, quoted, a ground
of a head where the decision came from the owner. That ground no longer exists.

Outside the work:

- Which questions are the owner's, and which take a built default:
  `design@agent-skills@questions-go-to-their-authority` and `primer@met-outside-the-task` stay as they
  are (#what-takes-a-built-default, withdrawn).
- The conversion of the old form in heads the work does not touch: a change that touches a head
  converts it, per `design@agent-skills@existing-heads-on-touch`, and the next pass of the
  design-record audit, run when the owner chooses, converts the rest. Issue entries and tripwires
  are converted when a change touches them; no audit axis reads issue entries, the gap that
  `issue@agent-skills@audit-axes-beyond-the-design-record` records.

## What is already decided

Rests on, not argued again: `goal@the-owner-decides`, `goal@the-owner-s-word-is-verified`,
`goal@design-with-agents-as-counterparts`, `goal@design-is-recorded-with-its-arguments`,
`design@agent-skills@questions-go-to-their-authority`,
`design@agent-skills@owner-word-holds-as-far-as-its-premise`,
`design@agent-skills@answer-keeps-its-authority`, `design@agent-skills@maintenance-crate-ask`,
`design@agent-skills@design-home-is-built-intent`, `design@agent-skills@owner-rules-at-a-fork`,
`design@agent-skills@existing-heads-on-touch`, `design@agent-skills@extension-follows-the-ground`,
`design@agent-skills@milestone-is-a-directory`.

Rewritten, with every text the rewrite repairs and the step that repairs it:

| decision | texts it repairs | step |
| --- | --- | --- |
| `design@agent-skills@heads-stand-on-argument` | the fragment's table and "A head whose ground cites the owner's words" | 2, harvest |
| `design@agent-skills@fork-choice-in-weak-form` | the fragment's weak form; planning `skill@knowledge-architect-planning@acceptance-criteria` | 2, 3, harvest |
| `design@agent-skills@attribution-replaces-quotation` | the fragment's attribution paragraph; the transcript reviewer; the design-record auditor; the decision-record reviewer; goal-setting's `%%` comment | 1, 2, 3, harvest |
| the deferral in decision-recording `skill@knowledge-architect-decision-recording@when-recording-happens`, which no head states | decision-recording; the shape fragment's "read against that approval"; planning `skill@knowledge-architect-planning@plan-document-leaves`, its reason only | 2, 3 |

Touched without reversing or rewriting their decisions, each brought to the rules on heads before
the harvest:

- `design@agent-skills@unargued-approval-is-argued`
- `design@agent-skills@default-ruled-before-merge`
- `design@agent-skills@transcript-reviewer-agent`
- `design@agent-skills@misstated-ruling-rated-by-what-it-changes`
- `design@agent-skills@one-home-for-head-rules`
- `design@agent-skills@audit-outcomes`
- `design@agent-skills@tripwire-stands-on-the-standard`
- `design@agent-skills@instruction-record-is-minimal`

Entries of the record search whose outcome changed something:

- `a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`, an issue of
  agent-skills: the commit that adds this spec closes it, since the spec schedules its work, per
  `skill@knowledge-architect-planning@spec-or-milestone`.
- `design@agent-skills@extension-follows-the-ground`: it routes a new member by whether a fork's
  choice was the owner's, read from the head, so a fork's choice keeps its provenance, as the ranking
  it is (#approval-left-unrecorded).
- `design@agent-skills@default-ruled-before-merge`: planning's leaving check is the one check that
  catches a branch about to merge with a default unruled; the condition stays, its reason changes.
- `design@agent-skills@maintenance-crate-ask`: an act a goal reserves to the owner is an underivable
  premise.
- `design@agent-skills@answer-keeps-its-authority`: the owner's choice among the readings a source
  permits is a ranking, and stays attributed.
- `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`: its
  re-entry point is met; step 1 splits the exception for issue entries out of
  `design@agent-skills@attribution-replaces-quotation`.

The other verdicts: [the discussion's search](search/discussion-1.txt).

## Criteria

### The owner decides what no argument settles, and sees what the agents decided `##owner-decides`

Binding, from `goal@the-owner-decides`. Met by #review-timing, the merge waiting for every ruling,
and by #approval-left-unrecorded, which keeps the owner's rankings visible and the routing of a new
member unchanged.

### Nothing is built on a contradiction `##no-build-on-contradiction`

Binding, from `goal@the-owner-s-word-is-verified`. Met: the work changes nothing in what holds on a
contradiction (#what-takes-a-built-default, withdrawn).

### The owner meets each default before it reaches main `##default-met-before-main`

Binding, from `goal@design-with-agents-as-counterparts`. Met by #review-timing.

### Each head keeps its argument and its rivals `##argument-kept`

Binding, from `goal@design-is-recorded-with-its-arguments`. Met by #approval-left-unrecorded: no
argument or rival leaves a head, and an underivable premise keeps its provenance.

### A head lands in the branch of its code, whatever the rulings `##head-with-its-branch`

Weighed, stated in the discussion. Met by #head-written-with-the-code and #harvest-timing.

### The review before the merge judges each head in its text when the review runs `##review-judges-heads`

Weighed, stated in the discussion, in a weak form, since a repair after the review is judged as
`skill@knowledge-architect-review@what-review-leaves` judges repairs, not by a new review. Met by
#head-written-with-the-code and #review-timing.

## New names, in one place

None in code. New prose names in the fragment: "underivable premise" is not a term the work
introduces into installed text; the fragment states the test in words (step 2).

## Decided design

### #head-written-with-the-code

A head that records a default awaiting the owner is written at the harvest like any other head.
Its text does not depend on the ruling, since an approval is no ground. A ruling that picks another
shape is a repair after the harvest, landed and reviewed as
`skill@knowledge-architect-review@what-review-leaves` says.

- decision-recording `skill@knowledge-architect-decision-recording@when-recording-happens`: the paragraph "A default awaiting the owner is
  built as written … before it is written" leaves, replaced by one sentence: a default awaiting the
  owner is recorded at the harvest like any other decision, and a ruling that changes it is a repair.
- the shape fragment: "A head written from a decision the owner approved is read against that
  approval before it is written" becomes: a head is read against the position the owner was shown,
  in a plan document's thread or in the discussion of the in-change path, by its direction, as
  today. The rest of the paragraph stays.
- planning `skill@knowledge-architect-planning@plan-document-leaves`: the condition that every default is ruled stays; its reason
  becomes "since the branch merges only once the owner has ruled". The milestone's sentence on a
  default still unruled after its last slice stays.
- decision-recording `skill@knowledge-architect-decision-recording@unargued-approval`: its moment becomes "Before a head is written from a
  decision that no argument for it, no rival and no cost stand behind, such as a default approved in
  a batch". The rest stays.

The nearest rival, the deferral, lost because its reason is gone: a head no longer quotes the
owner's words as its ground.

### #harvest-timing

One harvest, at the end of the branch, as `skill@knowledge-architect-planning@working-a-slice`
point 6 says today. No text changes. The rival, each head written in the commit of the step that
builds its code, lost on the owner's weighing in round 4 (#harvest-timing in
[deliberation](deliberation.md)).

### #review-timing

The review before the merge runs before the owner's rulings, and the merge waits for them, per
`design@agent-skills@default-ruled-before-merge`. No text changes. The rival, the review after every
ruling, costs a second full review whenever the first gives findings, and the review gives defaults
of its own.

### #approval-left-unrecorded

A durable record states a choice by its argument, and attributes only an underivable premise, by
where it first came from, not by who approved it:

- an argument is recorded on its merits, never attributed;
- an approval leaves no mark in a durable record: it is kept in the deliberation, the plan
  document's Threads and `D<n>` list marked as ruled, and the commit message;
- a fork's choice is the ranking of a cost no goal ranks, and is written with that ranking's
  provenance: "X over Y: the owner ranked <cost> above <cost>, which no goal ranks", or "the session
  ranked". A session's ranking the owner approved stays the session's;
- a fact is "the owner reported" or "the session assumed"; a judgement of what is plausible or of
  expected use likewise; an act a goal reserves to the owner is "on the owner's word";
- the test the fragment gives: could a session derive this from inside the project, from its goals,
  its record, its code and its measurements, and the sources it follows? If yes, it is an argument
  and is not attributed. If no, it is attributed by its provenance.

Why provenance matters there and nowhere else: a defeated premise is routed by it, per
`primer@owner-word-premise` (the owner's gives way to its source and the owner is told; a ranking of
intent defeated is a contradiction put to the owner; the session's is corrected and listed under
"Decided"), and a new member is routed by it, per `design@agent-skills@extension-follows-the-ground`.
The weak form's reopening rule is unchanged.

The fragment's edits:

- the table's approval row: "no ground: the session's decision, kept in the deliberation; at a fork,
  the session's ranking, in the weak form below";
- the pick row: the weak form "X over Y: the owner ranked <cost> above <cost>, which no goal ranks";
- the weak form opens "A fork's choice names the rivals, the costs weighed, and whose ranking it was";
- the attribution paragraph: a durable record attributes no argument and no approval, only an
  underivable premise, by its provenance, never quoting the owner; a plan document's items and a
  commit message attribute the owner's rulings as today, for the transcript reviewer. The
  exception for issue entries moves to its own paragraph (step 1 gives it its own head).

The transcript reviewer: "an argument attributed to the owner is the owner's" becomes "a durable
record attributes no argument and no approval, and attributes each ranking and fact to its source";
its minor severity "for an argument attributed to the owner that is not the owner's" becomes "for an
argument or an approval attributed in a durable record". The design-record auditor, the
decision-record reviewer and the project-audit design-record outcome table take the same rule: an
approval or an argument attributed in a head is removed in the branch, applied in the branch.
Planning `skill@knowledge-architect-planning@acceptance-criteria`: "records the owner's approval of it as the session's choice, and a
value the owner sets as the owner's" becomes "records the value with the provenance of its ranking,
the session's where the owner approved it".

The nearest rival, a choice stated without provenance ("X over Y: <cost> weighed against <cost>"),
lost on `design@agent-skills@extension-follows-the-ground`: a later session would find who chose
only in the deliberation records, which is `rejected@agent-skills@member-routed-by-approval-scope`.

## Mapping tables

| what the owner brings | durable record today | durable record after |
| --- | --- | --- |
| a fact only the owner holds | "the owner reported" | unchanged |
| an argument | recorded on its merits, often "the owner's proposal" or "the owner's argument" | on its merits, no attribution |
| an approval of the session's default at a fork | "the session chose X over Y, which no goal ranks; the owner approved it" | "X over Y: the session ranked <cost> above <cost>, which no goal ranks" |
| a pick at a fork, a threshold set, a local cost accepted | "the owner chose X over Y, which no goal ranks" | "X over Y: the owner ranked <cost> above <cost>, which no goal ranks" |
| a choice among the readings a source permits | "the owner chose among the readings the source permits" | unchanged |
| an act a goal reserves to the owner | "on the owner's word" | unchanged |
| an intent that is a goal | a goal | unchanged |

## Losing alternatives

- The head deferred until the owner's ruling: lost to #head-written-with-the-code, its reason gone.
- The harvest along the work: lost to #harvest-timing B, on the owner's weighing.
- The review after every ruling: lost to #review-timing.
- The approval kept in heads: lost to #approval-left-unrecorded: it changes no later move.
- A choice stated without provenance: lost to #approval-left-unrecorded, on
  `design@agent-skills@extension-follows-the-ground`.
- A sweep of the old form in this branch: lost to conversion on touch and the next audit pass, since
  each line needs a judgement of whether its premise is derivable.
- Only a real fork takes a built default (#what-takes-a-built-default): withdrawn by the owner in
  round 3, since each refinement it needed, where a real fork ends and what counts as a lead in the
  record, opened a question with no good answer, so none was better than the current routing.

## Readings

The work reads no external specification.

## Premortem

1. **A head contradicts a later ruling.** The harvest writes the head; the owner picks the other
   shape after the review; the repair that rewrites the head is not judged on the decision-record
   axis. Stresses #head-written-with-the-code. Named as a cause for the harvest to judge as a
   tripwire: it fires when a head on main states a shape the owner's ruling rejected.
2. **The old form spreads from precedent.** About 150 to 180 lines in the old form invite copying.
   Stresses #approval-left-unrecorded. Named as a cause for the harvest to judge, with a short
   horizon; the design-record audit's standard and the review's decision-record axis carry the rule.
3. **An argument and a ranking are confused.** A session attributes an argument as the owner's
   ranking, or records a ranking unattributed, and a new member is routed wrong. Stresses
   #approval-left-unrecorded. Becomes a claim of step 2.

## Acceptance criteria

None: the work changes text, and no built code can apply a check on its decisions.

## Implementation sequence

1. **Conform the touched heads.** Split the exception for issue entries out of
   `design@agent-skills@attribution-replaces-quotation` into a head of its own, with its rival, an
   issue entry that never quotes the owner; re-point its citing sites (`cargo klarch show`). Apply
   every conformance the design-record auditor's verified drafts on the eight touched heads above
   find. Fails alone on: `cargo klarch check`.
2. **The fragments.** Edit the fragment and the shape fragment as "Decided design" says. Claim: at
   every text that includes the fragment, the test "could a session derive this from inside the
   project" is stated beside the attribution rule, and no sentence of an including text still
   attributes an approval or an argument; tested by reading each including text after
   `cargo klarch install-agent-skills`, not by a phrase search alone. Fails alone on: an including
   text that contradicts the fragment.
3. **The skills and the agents.** Edit decision-recording, planning, the transcript reviewer, the
   design-record auditor, the decision-record reviewer, project-audit's outcome table and
   goal-setting's `%%` comment, as "Decided design" says. Run `cargo klarch install-agent-skills`,
   then `cargo x gates`. Write the `Next release` entry of `path@knowledge-architect@CHANGELOG.md`,
   after reading `design@knowledge-architect@changelog-entries` and
   `design@knowledge-architect@versioning-policy` whole, then `cargo x changelog`.
4. **The harvest.** Rewrite `design@agent-skills@heads-stand-on-argument`,
   `design@agent-skills@fork-choice-in-weak-form` and
   `design@agent-skills@attribution-replaces-quotation` to the decided design, and the eight touched
   heads where step 1 did not already; convert the old form in every head the work touches; judge
   the losing alternatives as `rejected` entries by
   `skill@knowledge-architect-decision-recording@losing-alternatives`; judge premortem causes 1 and
   2 by the tripwire standard.

Then the review before the merge, and the deletion of this spec.

## Order rationale

- 1 before 2: the harvest's heads must already conform, per
  `skill@knowledge-architect-planning@cutting-steps-and-slices`.
- 2 before 3: the skills and agents restate the fragment's rule, so it is fixed first.
- 3 before 4: the heads record the text as built.

## Defaults awaiting the owner

None.

## Harvest

| thread | home |
| --- | --- |
| #head-written-with-the-code | `design@agent-skills@default-ruled-before-merge`, rewritten to state that the head is written at the harvest and a ruling that changes it is a repair |
| #harvest-timing | no head: `design@agent-skills@design-home-is-built-intent` already states it; the decision-recording tests judge whether the lost rival earns a `rejected` entry |
| #review-timing | no head: `design@agent-skills@default-ruled-before-merge` already states it |
| #approval-left-unrecorded | `design@agent-skills@heads-stand-on-argument`, `design@agent-skills@fork-choice-in-weak-form`, `design@agent-skills@attribution-replaces-quotation` |
| #what-takes-a-built-default | nothing: withdrawn |

## Later consequences

The next pass of the design-record audit converts the old form in the heads and the `rejected`
entries this work does not touch.
