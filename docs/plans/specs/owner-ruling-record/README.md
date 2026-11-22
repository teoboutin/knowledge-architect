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
- **the entry-tests fragment**: `path@agent-skills@fragments/head-entry-tests.md`.
- **a durable record**: in this spec, a design head, a `rejected` entry, a tripwire or an issue
  entry. A plan document's items and a commit message are the **deliberation records**. Installed
  text names the four kinds of durable record in words, never by this term.
- **an underivable premise**: in this spec, a premise a session cannot derive from inside the
  project, from its goals, its record, its code and its measurements, and the sources it follows:
  a fact; the ranking of a cost, which a fork's choice, a threshold set and a local cost accepted
  are; a choice among the readings a source permits; a judgement of what is plausible or of
  expected use; an act a goal reserves to the owner, such as a take-in under
  `goal@generic-tools-and-procedures-are-shipped`. Installed text states the test in words, never
  by this term.
- **the old form**: in a durable record, "the owner approved it", an argument attributed to the
  owner ("the owner's proposal", "the owner's argument"), or a fork's choice attributed as a choice
  ("the owner chose X over Y", "the session chose X over Y") rather than as the ranking it made.
- **shape A, shape B** of #harvest-timing: A, each head written in the commit of the step that
  builds its code; B, one harvest at the end of the branch.

## What the work is

What exists today, each read at main `3c270857`:

- The fragment's table "what the owner brings" records an approval at a fork as "the session chose
  X over Y, which no goal ranks; the owner approved it" (line 23), and a pick of the owner as "the
  owner chose X over Y, which no goal ranks" (line 25). Its weak form opens "A fork's choice, the
  owner's or the session's" (line 30). Its paragraph "A head whose ground cites the owner's words is
  read by the same table" (lines 35-37). Its paragraph "A durable record attributes a decision to the
  owner; it does not quote the owner" (lines 39-48) lists a head, a commit message, a tripwire, a
  `rejected` entry, an issue entry and a plan document's items, writes "the owner chose X over Y"
  or "on the owner's word", and holds the exception that lets an issue entry keep the owner's words
  verbatim on the owner's request.
- The entry-tests fragment, lines 62-63: "where the owner approved it, the approval is recorded in
  the commit, as the session's decision, not in the head". It already agrees with the decided
  design.
- `path@agent-skills@content/skills/decision-recording/SKILL.md`,
  `skill@knowledge-architect-decision-recording@when-recording-happens`, lines 32-35: "A default
  awaiting the owner is built as written, per `skill@knowledge-architect-planning@working-a-slice`,
  and enters a design head once the owner rules on it. Until then it is no approved decision, and a
  head is read against the owner's approval before it is written". Its
  `skill@knowledge-architect-decision-recording@unargued-approval`, lines 51-52: "Before a head is
  written from an approval that no argument for the decision, no rival and no cost stand behind".
- The shape fragment, lines 81-82: "A head written from a decision the owner approved is read
  against that approval before it is written"; lines 102-105: a conformance "that would relax or
  turn what the owner's words in it approved" goes to the owner as one proposal.
- `path@agent-skills@content/skills/planning/SKILL.md`:
  `skill@knowledge-architect-planning@plan-document-leaves`, lines 857-858, "every default the
  document lists as awaiting the owner is ruled, since a design head records such a default only
  once the owner has ruled on it"; `skill@knowledge-architect-planning@acceptance-criteria`, lines
  541-542, "`primer@design-heads` records the owner's approval of it as the session's choice, and a
  value the owner sets as the owner's"; `skill@knowledge-architect-planning@cutting-steps-and-slices`,
  line 474, "An edit that would relax what the owner's words in the head approved".
- `path@agent-skills@content/agents/transcript-reviewer.md`:
  `agent@knowledge-architect-transcript-reviewer@check-recorded-rulings`, line 115, "an argument
  attributed to the owner is the owner's"; `agent@knowledge-architect-transcript-reviewer@how-to-report`,
  lines 127-128, the minor severity "for an argument attributed to the owner that is not the
  owner's".
- `path@agent-skills@content/skills/retrospective/SKILL.md`, lines 150-152: "an argument or a reading
  attributed to the owner that is not the owner's, caught by the transcript reviewer before the
  merge, is no finding".
- `path@agent-skills@content/agents/tripwire-auditor.md`, lines 46-48: "a durable record attributes
  a decision to the owner and never quotes the owner, keeping the scope and the hedges of the
  owner's words, per `primer@design-heads`".
- `path@agent-skills@content/agents/design-record-auditor.md`: line 49, "remove the owner's own
  words together with an approval cited as a ground"; lines 112-116, "where the head cites the
  owner, or rests on an approval, read the history"; line 119, "fidelity to what the owner
  approved, read by its direction".
- `path@agent-skills@content/agents/decision-record-reviewer.md`: line 112, "fidelity to what the
  owner approved, read by its direction"; line 115, "The owner's ground a head attributes is read in
  its source".
- `path@agent-skills@content/skills/project-audit/SKILL.md`, the design-record axis's outcome
  table: line 211, "an approval cited as a head's ground, removed when the head stands on its
  argument, or recorded as the session's choice at a fork"; line 212, "a change that relaxes what
  the owner's words approved".
- `%%` comments in the old form: `path@agent-skills@content/skills/goal-setting/SKILL.md` lines 58-61;
  `path@agent-skills@content/skills/component-creation/SKILL.md` lines 24, 44, 93, 116;
  `path@agent-skills@content/skills/setup/SKILL.md` line 260.
- The durable records of this repository hold the old form. Each count is
  `grep -rni -e "<pattern>" crates/*/docs tools/*/docs docs --include=*.md | grep -v docs/plans/`,
  lines then files, at main `3c270857`, counting a line once per pattern it matches:

  | pattern | lines | files |
  | --- | --- | --- |
  | `the owner approved it` | 21 | 11 |
  | `the session chose` | 20 | 14 |
  | `the owner chose` | 40 | 29 |
  | `the owner's proposal` | 25 | 13 |
  | `the owner's argument` | 31 | 18 |
  | `owner reported\|the owner's report` | 45 | 22 |

  The last pattern counts facts, which keep their attribution; the first five, 137 lines, are the
  candidates of the old form, some of them rankings that keep their provenance in the new form.

The deferral of a head was written on 2026-10-12, in the commit "The findings the retrospective
analyses ruled to handle now are handled", whose tree's primer read "The owner's words are a
ground, quoted, only where the decision came from the owner". The commit gives no reason for the
deferral; that this ground was its reason is the session's inference (`argument@owner-ruling-record@arg-6`).
No installed text holds that ground today.

Outside the work:

- Which questions are the owner's, and which take a built default:
  `design@agent-skills@questions-go-to-their-authority` and `primer@met-outside-the-task` stay as they
  are (#what-takes-a-built-default, withdrawn).
- The conversion of the old form in records the work does not touch. A head is converted when a
  change touches it, per `design@agent-skills@existing-heads-on-touch`, and the next pass of the
  design-record audit, run when the owner chooses, converts the heads and the `rejected` entries
  left. A tripwire is converted by the tripwire audit axis, which already rewords a ground "on the
  owner's word" to the entry's premise. An issue entry keeps the old form until it closes: no rule
  converts an issue entry a change touches, and no audit axis reads issue entries, the gap that
  `issue@agent-skills@audit-axes-beyond-the-design-record` records.

## What is already decided

Rests on, not argued again: `goal@the-owner-decides`, `goal@the-owner-s-word-is-verified`,
`goal@design-with-agents-as-counterparts`, `goal@design-is-recorded-with-its-arguments`,
`design@agent-skills@questions-go-to-their-authority`,
`design@agent-skills@owner-word-holds-as-far-as-its-premise`,
`design@agent-skills@answer-keeps-its-authority`, `design@agent-skills@maintenance-crate-ask`,
`design@agent-skills@design-home-is-built-intent`, `design@agent-skills@owner-rules-at-a-fork`,
`design@agent-skills@extension-follows-the-ground`, `design@agent-skills@milestone-is-a-directory`,
`design@agent-skills@one-decision-per-head`, `design@agent-skills@pre-harvest-conformance`,
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again`,
`design@agent-skills@default-ruled-before-merge`.

Rewritten, with every text the rewrite repairs and the step that repairs it. Each step also reads
every site `cargo klarch show <reference>` lists for the decision it rewrites, and repairs each
site whose statement the rewrite makes false.

| decision | texts it repairs | step |
| --- | --- | --- |
| `design@agent-skills@heads-stand-on-argument` | the fragment's table and its paragraph "A head whose ground cites the owner's words"; the design-record auditor, line 49 and lines 112-116; the decision-record reviewer, line 115; project-audit, line 211; the `%%` comments listed above | 2, 3, harvest |
| `design@agent-skills@fork-choice-in-weak-form` | the fragment's weak form; planning `skill@knowledge-architect-planning@acceptance-criteria` | 2, 3, harvest |
| `design@agent-skills@attribution-replaces-quotation` | the fragment's attribution paragraph; the tripwire auditor, lines 46-48; planning `skill@knowledge-architect-planning@spec-contents`, "The owner's words are attributed in the items" (lines 337-338) | 1, 2, 3, harvest |
| `design@agent-skills@transcript-reviewer-agent` | the transcript reviewer, lines 115 and 127-128; retrospective, lines 150-152 | 3, harvest |
| `design@agent-skills@misstated-ruling-rated-by-what-it-changes` | the transcript reviewer's severities, lines 127-128 | 3, harvest |
| `design@agent-skills@unargued-approval-is-argued` | decision-recording `skill@knowledge-architect-decision-recording@unargued-approval` | 3, harvest |
| `design@agent-skills@existing-heads-on-touch`, its second paragraph | the shape fragment, lines 102-105; planning, line 474; project-audit, line 212 | 2, 3, harvest |
| the deferral in decision-recording `skill@knowledge-architect-decision-recording@when-recording-happens`, which no head states | decision-recording, lines 32-35; the shape fragment, lines 81-82; planning `skill@knowledge-architect-planning@plan-document-leaves`, its reason only | 2, 3 |

Rewritten only where they restate the attribution rule, each sentence named by the step that
rewrites it, their decisions unchanged:

- `design@agent-skills@one-home-for-head-rules`: its naming of the fidelity rule "fidelity to what
  the owner approved, read by its direction".
- `design@agent-skills@audit-outcomes`: "An approval cited as a head's ground is removed in the
  branch where the head stands on its argument, or recorded as the session's choice at a fork, and
  the owner's pick at a fork cited as a ground is recorded in the weak form".
- `design@agent-skills@tripwire-stands-on-the-standard`: "unlike a head, which can record the
  owner's choice at a fork".
- `design@agent-skills@instruction-record-is-minimal`: "in a head, as an argument the owner made;
  and as the owner's choice at a fork in its weak form".

Created by the harvest, each a thread of this discussion: #head-written-with-the-code,
#harvest-timing, #review-timing; created by step 1: `owner-words-kept-in-an-issue-on-request`.
`cargo klarch id` reports each free at the commit that adds this spec.

Entries of the record search whose outcome changed something:

- `a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`, an issue of
  agent-skills: the commit that adds this spec closes it, since the spec schedules its work, per
  `skill@knowledge-architect-planning@spec-or-milestone`. Its live content moves here, for the head
  #head-written-with-the-code to carry as evidence of the deferral's cost: three lapses of the
  deferral in this repository, each found by a review. On 2026-11-01, an in-change decision's
  default was written into a head in the implementing commit, removed by the decision-record
  re-review, and written back after the ruling. On 2026-11-04, in the slice that built the `model`
  crate, the content of its default D4 was written into a head before the ruling. On 2026-11-13, in
  slice 1 of the milestone on split guidelines and plan directories, a head stated its default D2
  while D2 awaited the owner, and the head was not taken back before the ruling. The issue was
  opened from the owner's ruling at the retrospective intake of 2026-10-24 that the condition be
  changed.
- `design@agent-skills@extension-follows-the-ground`: it routes a new member by whether a fork's
  choice was the owner's, read from the head, so a fork's choice keeps its provenance, as the ranking
  it is (#approval-left-unrecorded).
- `design@agent-skills@default-ruled-before-merge`: planning's leaving check is the one check that
  catches a branch about to merge with a default unruled; the condition stays, its reason changes.
- `design@agent-skills@maintenance-crate-ask`: an act a goal reserves to the owner is an underivable
  premise.
- `design@agent-skills@answer-keeps-its-authority`: the owner's choice among the readings a source
  permits is an underivable premise, and keeps its form "the owner chose among the readings the
  source permits", as `primer@labelled-items` states it.
- `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`: its
  re-entry point is met. Step 1 splits the exception for issue entries out of
  `design@agent-skills@attribution-replaces-quotation`, and the harvest adds this branch to the
  issue's instances as one that took the split; the issue stays open, since its question, whether a
  touched head owes its older bundles, is not this work's.

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

No name in code. New design heads, in `path@agent-skills@docs/design/owner-and-intent.md`:

- a head whose slug is the thread's, #head-written-with-the-code, at the harvest;
- a head whose slug is the thread's, #harvest-timing, at the harvest;
- a head whose slug is the thread's, #review-timing, at the harvest;
- `owner-words-kept-in-an-issue-on-request`, `ruling-read-by-its-direction` and
  `session-removes-an-excluded-tripwire`, at step 1.

## Decided design

### #head-written-with-the-code

A head that records a default awaiting the owner is written at the harvest like any other head.
Its text does not depend on the ruling, since an approval is no ground. A ruling that picks another
shape is a repair after the harvest: it reverses or changes a head, so it is reviewed again, per
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.

- decision-recording `skill@knowledge-architect-decision-recording@when-recording-happens`: the
  paragraph "A default awaiting the owner is built as written … before it is written" becomes: "A
  default awaiting the owner is built as written, per
  `skill@knowledge-architect-planning@working-a-slice`, and recorded at the harvest like any other
  decision: a head's text does not depend on the owner's ruling, since an approval is no ground. A
  ruling that picks another shape is a repair, landed as
  `skill@knowledge-architect-review@what-review-leaves` says."
- the shape fragment, lines 81-82: "A head written from a decision the owner approved is read
  against that approval before it is written" becomes "A head is read against the position the
  owner was shown before it is written". The rest of that paragraph stays.
- planning `skill@knowledge-architect-planning@plan-document-leaves`: the condition that every
  default is ruled stays; its reason becomes "since the branch merges only once the owner has ruled
  on each". The milestone's sentence on a default still unruled after its last slice stays.

The head #head-written-with-the-code states this decision. Its nearest rival, the deferral, lost
because a head's text no longer depends on the owner's words. It passes entry test 2: the reason is
respected at decision-recording, at the shape fragment and at planning's leaving check.

### #harvest-timing

Shape B: one harvest, at the end of the branch, as `skill@knowledge-architect-planning@working-a-slice`
point 6 says today. No installed text changes. Shape A lost on the owner's ranking in round 4: the
owner ranked avoiding a decision amended at each step, and the better heads that record work done
at once gives, with the implementer kept to the code, above the review of the work seeing the heads,
which the spec already gives most of what it needs. No fact defeats either shape, and
`design@agent-skills@design-home-is-built-intent` admits both.

The head #harvest-timing states it in the weak form: "B over A: the owner ranked <those costs>,
which no goal ranks". It passes entry test 2: the rule is stated in planning's
`skill@knowledge-architect-planning@working-a-slice`, points 4 to 7, and in decision-recording's
`skill@knowledge-architect-decision-recording@when-recording-happens`, "the **harvest**, and it
happens in the change that lands the work".

### #review-timing

The review before the merge runs before the owner's rulings; the merge waits for them, per
`design@agent-skills@default-ruled-before-merge`; a ruling that changes the work is a repair. No
installed text changes: nothing gates the review on the rulings. The rival, the review after every
ruling, lost on two facts: the review gives defaults of its own, so waiting is circular, and a
second full review is owed whenever the first gives findings.

The head #review-timing states it. It passes entry test 2, as a decision about an absence: no
text gates the review on the rulings, and its rival is argued.

### #approval-left-unrecorded

A durable record states a choice by its argument, and attributes only an underivable premise, by
where it first came from, not by who approved it:

- an argument is recorded on its merits, never attributed;
- an approval leaves no mark in a durable record: it is kept in the deliberation records, the plan
  document's Threads and `D<n>` list marked as ruled, and the commit message;
- a fork's choice is the ranking of a cost no goal ranks, written with that ranking's provenance:
  "X over Y: the owner ranked <cost> above <cost>, which no goal ranks", or "the session ranked". A
  session's ranking the owner approved stays the session's. Where the record names no costs, a
  conversion writes "X over Y, which no goal ranks: the owner's ranking" (or "the session's"),
  never costs it has to invent; the history of the head, `git log -L`, and the plan document that
  carried the fork are read first;
- a fact is "the owner reported" or "the session assumed"; a judgement of what is plausible or of
  expected use likewise;
- a choice among the readings a source permits keeps "the owner chose among the readings the source
  permits";
- an act a goal reserves to the owner is "on the owner's word";
- the test the fragment gives, in words: could a session derive this from inside the project, from
  its goals, its record, its code and its measurements, and the sources it follows? If yes, it is an
  argument, and no one is named as its source. If no, it is attributed to where it came from.

Why provenance matters there and nowhere else: a defeated premise is routed by it, per
`primer@owner-word-premise` (the owner's gives way to its source and the owner is told; a ranking of
intent defeated is a contradiction put to the owner; the session's is corrected and listed under
"Decided"), and a new member is routed by it, per `design@agent-skills@extension-follows-the-ground`.
The weak form's reopening rule is unchanged.

The deliberation records keep who proposed and who ruled, as today:
`skill@knowledge-architect-planning@spec-contents` keeps its items attributing the owner's rulings
and arguments, and its sentence "as `primer@design-heads` says of a durable record" becomes "as
`primer@design-heads` says of the deliberation records".

The fragment's edits, at step 2:

- the table's approval row: "no ground: the session's decision, kept in the deliberation records; at
  a fork, the session's ranking, in the weak form below";
- the pick row: "the weak form: X over Y: the owner ranked <cost> above <cost>, which no goal ranks";
- a row is added after the fact row: "a judgement of what is plausible or of expected use | a
  premise, attributed as the owner's report and open to challenge"; the fact row, the reading row
  and the goal row stay;
- the weak form opens "A fork's choice names the rivals, the costs weighed, and whose ranking it
  was";
- the paragraph "A head whose ground cites the owner's words is read by the same table" stays, and
  gains: "a ground that cites the owner's argument is an argument, recorded on its merits";
- the attribution paragraph becomes two. The first: a design head, a `rejected` entry, a tripwire and
  an issue entry name no one as the source of an argument or of an approval; they attribute, to the
  owner or to the session, only what a session could not derive from inside the project (the
  derivability test, in words, and the kinds listed above), never quoting the owner; a plan
  document's items and a commit message attribute the owner's rulings and arguments, as
  `skill@knowledge-architect-planning@spec-contents` asks, never quoting the owner; an attribution
  keeps the scope and the hedges of the owner's words. The second: the exception for issue entries,
  as it stands, which step 1 gives its own head.

The shape fragment, lines 102-105, and planning, line 474: "would relax or turn what the owner's
words in it approved" becomes "would relax or turn a ranking or a report the head attributes to the
owner". Project-audit line 212 likewise. The second paragraph of
`design@agent-skills@existing-heads-on-touch` is rewritten with them at the harvest.

The transcript reviewer, at step 3:

- `agent@knowledge-architect-transcript-reviewer@check-recorded-rulings`: "an argument attributed to
  the owner is the owner's" becomes "an argument a deliberation record attributes to the owner is
  the owner's; a durable record names no one as the source of an argument or of an approval, and
  attributes each ranking and each fact to its source".
- `agent@knowledge-architect-transcript-reviewer@how-to-report`: the minor severity "for an argument
  attributed to the owner that is not the owner's" becomes "for an argument a deliberation record
  attributes to the owner that is not the owner's, and for an argument or an approval a durable
  record attributes to anyone"; the major severity for "a decision attributed to the owner that the
  owner did not make" stays, and covers a ranking attributed to the owner that the owner did not
  make.
- retrospective, lines 150-152, follows the minor severity: "an argument or a reading attributed to
  the owner that is not the owner's, or an argument or an approval a durable record attributes,
  caught by the transcript reviewer before the merge, is no finding".

The other agents and skills, at step 3:

- the design-record auditor, line 49: "remove the owner's own words together with an approval cited
  as a ground" becomes "remove a ranking or a fact the owner holds together with an approval cited
  as a ground"; lines 112-116: "rests on an approval" becomes "attributes a ranking or a fact to the
  owner"; line 119, and the decision-record reviewer, line 112: "fidelity to what the owner
  approved" becomes "fidelity to the position the owner was shown"; the decision-record reviewer,
  line 115, stays.
- the tripwire auditor, lines 46-48: "a durable record attributes a decision to the owner and never
  quotes the owner" becomes "a tripwire attributes only what a session could not derive from inside
  the project, as a head does, and never quotes the owner".
- project-audit, line 211: "an approval cited as a head's ground, removed when the head stands on its
  argument, or recorded as the session's choice at a fork" becomes "an approval or an argument
  attributed in a head, removed; a fork's choice recorded with the provenance of its ranking".
- planning `skill@knowledge-architect-planning@acceptance-criteria`: "records the owner's approval of
  it as the session's choice, and a value the owner sets as the owner's" becomes "records the value
  with the provenance of its ranking: the session's where the owner approved it, the owner's where
  the owner set it".
- decision-recording `skill@knowledge-architect-decision-recording@unargued-approval`: its moment
  becomes "Before a head is written from a decision that no argument for it, no rival and no cost
  stand behind, such as a default approved in a batch"; a tie the owner then breaks is recorded as
  the owner's ranking.
- each `%%` comment listed in "What the work is": converted to the new form.

The nearest rival, a choice stated without provenance ("X over Y: <cost> weighed against <cost>"),
lost on `design@agent-skills@extension-follows-the-ground`: a later session would find who chose
only in the deliberation records, which is `rejected@agent-skills@member-routed-by-approval-scope`.

## Mapping tables

| what the owner brings | durable record today | durable record after |
| --- | --- | --- |
| a fact only the owner holds | "the owner reported" | unchanged |
| a judgement of plausibility or expected use | no row | "the owner reported" or "the session assumed" |
| an argument | recorded on its merits, often "the owner's proposal" or "the owner's argument" | on its merits, no source named |
| an approval of the session's default at a fork | "the session chose X over Y, which no goal ranks; the owner approved it" | "X over Y: the session ranked <cost> above <cost>, which no goal ranks" |
| a pick at a fork, a threshold set, a local cost accepted | "the owner chose X over Y, which no goal ranks" | "X over Y: the owner ranked <cost> above <cost>, which no goal ranks" |
| a choice among the readings a source permits | "the owner chose among the readings the source permits" | unchanged |
| an act a goal reserves to the owner | "on the owner's word" | unchanged |
| an intent that is a goal | a goal | unchanged |

## Losing alternatives

- The head deferred until the owner's ruling: lost to #head-written-with-the-code, its ground gone.
- Shape A of #harvest-timing: lost on the owner's ranking.
- The review after every ruling: lost to #review-timing.
- The approval kept in heads: lost to #approval-left-unrecorded: it changes no later move.
- A choice stated without provenance: lost to #approval-left-unrecorded, on
  `design@agent-skills@extension-follows-the-ground`.
- A sweep of the old form in this branch: lost to conversion on touch and the next audit pass, since
  each of the 137 candidate lines needs a judgement of whether its premise is derivable.
- Only a real fork takes a built default (#what-takes-a-built-default): the owner considered
  dropping it in round 3, since each refinement it needed, where a real fork ends and what counts as
  a lead in the record, opened a question with no good answer, and approved its withdrawal with the
  checkpoint table in round 6.

## Readings

The work reads no external specification.

## Premortem

1. **A head contradicts a later ruling.** Stresses #head-written-with-the-code. Dropped: a ruling
   that changes a head is a repair that reverses or changes it, which
   `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` sends to review again, so the
   failure is not silent.
2. **The old form spreads from precedent.** The 137 candidate lines invite copying. Stresses
   #approval-left-unrecorded. Named as a cause for the harvest to judge by the tripwire standard,
   with a short horizon; the design-record audit's standard and the review's decision-record axis
   carry the rule.
3. **An argument and a ranking are confused.** A session names the owner as the source of an
   argument, or records a ranking with no source, and a new member is routed wrong. Stresses
   #approval-left-unrecorded. Becomes a claim of step 2.

## Acceptance criteria

None: the work changes text, and no built code can apply a check on its decisions.

## Implementation sequence

1. **Conform the touched heads.** Builds, from the drafts of the design-record auditor dispatched
   at the planning of this spec and their verification, each outcome in the commit that records
   them:
   - the head `owner-words-kept-in-an-issue-on-request`, split out of
     `design@agent-skills@attribution-replaces-quotation`, with its rival, an issue entry that never
     quotes the owner, written in the new form, and every citing site re-pointed, as
     `cargo klarch show` lists them;
   - `design@agent-skills@transcript-reviewer-agent` split: the reading of a ruling by its direction,
     wherever fidelity is judged, with its goal exception, moves to a head of its own,
     `ruling-read-by-its-direction`, the sentence on the observed case with no obligation added moved
     unchanged in the past tense; design-skill-and-heads.md:231 and owner-and-intent.md:245
     re-pointed;
   - `design@agent-skills@tripwire-stands-on-the-standard` split: the session's removal of a recorded
     tripwire that a condition of the standard excludes moves to a head of its own,
     `session-removes-an-excluded-tripwire`, titled "The session removes a recorded tripwire that a
     condition of the standard excludes, in an audit as elsewhere, but for a removal on the doubt's
     weight", the pair #removal-outside-the-exits → `session-removes-an-excluded-tripwire` in the
     commit; the rewording and the rescoping by the session, and the sentence on the owner naming the
     wording, stay in the head that keeps the slug;
   - `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: "made" becomes "makes";
   - `rejected@agent-skills@transcript-reviewer-reports-every-clause`: its verbatim quotation of the
     owner becomes an attribution that keeps the hedge;
   - each default D1 to D5 below, as written, unless the owner rules otherwise.

   `design@agent-skills@audit-outcomes` needs nothing: the verification refuted its draft, since its
   closing paragraph is an attribution that keeps the owner's hedges. Claims: each created head
   passes the rules on heads, judged alone by the auditor briefed to check, per
   `skill@knowledge-architect-planning@cutting-steps-and-slices`; the head it leaves no longer holds
   what moved. Fails alone on: a citing site that still names the moved part in the head it left.
2. **The fragments.** Builds: the edits of the fragment and the shape fragment that "Decided design"
   lists. Claims: at every text that includes the fragment, the derivability test is stated beside
   the attribution rule, and no sentence of an including text names anyone as the source of an
   argument or of an approval in a durable record, nor leaves a ranking without its source; tested
   by reading each including text whole after `cargo klarch install-agent-skills`, a phrase search
   being a lead only; the entry-tests fragment, lines 62-63, agrees. Fails alone on: an including
   text that contradicts the fragment.
3. **The skills and the agents.** Builds: every other edit "Decided design" lists. Run
   `cargo klarch install-agent-skills`, then `cargo x gates`. Write the `Next release` entry of
   `path@knowledge-architect@CHANGELOG.md`, after reading `design@knowledge-architect@changelog-entries`
   and `design@knowledge-architect@versioning-policy` whole, then `cargo x changelog`. Claims: each
   sentence "What the work is" quotes from these texts reads as "Decided design" says, or its
   change is named in the commit; no installed text states the deferral (a search for "enters a
   design head once" returns nothing). Fails alone on: an installed sentence that still states the
   old rule.
4. **The harvest.** Builds: the three heads created (#head-written-with-the-code,
   #harvest-timing, #review-timing); the rewrites the "Rewritten" table names; the old form
   converted in every head the work touches; the losing alternatives judged as `rejected` entries by
   `skill@knowledge-architect-decision-recording@losing-alternatives`; premortem cause 2 judged by
   the tripwire standard; this branch added to the instances of
   `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`. Claims:
   each harvest row is done; each head written passes the rules on heads. Fails alone on: the review
   before the merge, on its decision-record axis.

Then the review before the merge, and the deletion of this spec.

## Order rationale

- 1 before 2: the split moves the exception for issue entries out of the head whose paragraph step 2
  rewrites, so step 2 rewrites only the attribution rule.
- 2 before 3: the skills and agents restate the fragment's rule, so it is fixed first.
- 3 before 4: the heads record the text as built.

## Defaults awaiting the owner

Each is a finding of the design-record auditor on a head this work touches, verified by a second
auditor, whose outcome is the owner's, per `skill@knowledge-architect-planning@cutting-steps-and-slices`.
Step 1 builds each default as written.

- **D1, on `design@agent-skills@unargued-approval-is-argued` and
  `design@agent-skills@instruction-record-is-minimal`.** The first fails every entry test: the
  decision-recording skill's `skill@knowledge-architect-decision-recording@unargued-approval` alone
  states it, and nothing cites it. The second admits a decision "where it keeps the owner's intent
  on record", against "The owner's intent earns no head of its own"; its other case, keeping two
  parts of the workflow consistent, reads two ways against entry test 2: an instance of the test, or
  "a value repeated for consistency", which the test excludes. Default: remove
  `design@agent-skills@unargued-approval-is-argued`, its reason moved to a `%%` line beside the
  skill's instruction; retitle the second "A decision about the installed text earns a head only by
  an entry test or by keeping two parts of the workflow consistent, and a rewording earns none",
  reading the consistency case as an instance of entry test 2. The rival keeps the first head on the
  owner's request, recorded in the head-rules spec, that the provenance of design heads be recorded.
- **D2, on `design@agent-skills@default-ruled-before-merge`.** Its ground "which the owner does not
  want" cites the owner's intent against the rival in which the work waits for each answer. Default:
  ground it on `goal@agents-work-without-drift`, which serves it. The rival reading: the owner's
  words in the commit "A default put to the owner is built, and its branch merges once the owner has
  ruled on it" give a reason, not interrupting the work, so the choice is the owner's ranking at a
  fork, kept in the weak form as "the owner ranked".
- **D3, on `design@agent-skills@transcript-reviewer-agent`.** "The owner argued that the reviewer's
  weight belongs on …" drops the owner's "focus more", turns a wish into an argument, and widens
  "clearly sane deviations" to every small addition. The commit that wrote it chose "argued" so that
  the head records an argument rather than a ruling. Default: under the new rule, the sentence
  becomes the owner's ranking of where the reviewer's weight goes, with the hedge "more" and the
  scope "clearly sane deviations" kept.
- **D4, on `design@agent-skills@one-home-for-head-rules`.** "The centralization is the owner's
  proposal … avoided at all cost" cites an intent that recorded heads already serve, and the head
  tells how the design departed from the owner's proposal. Default: the attribution and the account
  of the departure leave; the head stands on the heads that serve it; its wording "no restatement
  longer than one sentence exists".
- **D5, on `design@agent-skills@tripwire-stands-on-the-standard`.** Its sentence "The owner ruled
  that a tripwire grounded on the owner's word gets no standing …" may be deliberation rather than a
  premise: the verifier found it plausible, not confirmed. Default: keep it, attributed as the
  owner's ranking, in the head that keeps the slug.

## Harvest

| thread | home |
| --- | --- |
| #head-written-with-the-code | the new head #head-written-with-the-code |
| #harvest-timing | the new head #harvest-timing, in the weak form with the owner's ranking |
| #review-timing | the new head #review-timing |
| #approval-left-unrecorded | `design@agent-skills@heads-stand-on-argument`, `design@agent-skills@fork-choice-in-weak-form`, `design@agent-skills@attribution-replaces-quotation`, and the other heads the "Rewritten" table names |
| #what-takes-a-built-default | nothing: withdrawn |

## Later consequences

The next pass of the design-record audit converts the old form in the heads and the `rejected`
entries this work does not touch. An issue entry keeps it until it closes.
