# A head's title must not mislead, and a tripwire fires on a defect a later procedure finds on main

## Status and audience

This spec plans the work of one branch, which changes installed text, one message of the checker,
the design homes of agent-skills and check, and the agent-skills tripwires. It leaves when its work
lands, in a commit of its own after the review before the merge. Where it and a design home
disagree on a point that "What is already decided" does not list as reversed or rewritten, the
design home wins. Every name it uses is defined in it or exists in the tree. Where the owner's word
is awaited, `skill@knowledge-architect-planning@working-a-slice` says what waits.

It was written for a session that did not see the discussion. The discussion is one session, which
begins at the owner's message "can you start the design discussion on the fired tripwire ?". Its
rounds, each named by the opening words of the owner's message:

1. "can you start the design discussion on the fired tripwire"
2. "you say "Every instance was caught by the review of its own branch"
3. "review-finding-is-no-firing looks like the right direction"
4. "agreed on this table"

The threads and the arguments are in [deliberation](deliberation.md); the record search's
generated lines in [the discussion's search](search/discussion-1.txt); the transcript in
[the transcript](transcripts/1.txt).

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, "The work of a spec". Every edit of
installed text follows `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", read
whole before the first edit. The Rust change of step 2 follows
`skill@knowledge-architect-development` and `skill@klarch-development`, both loaded before it.

## Names

- **the shape fragment**: `path@agent-skills@fragments/head-shape.md`, included in
  decision-recording, the decision-record reviewer and the design-record auditor.
- **the standard fragment**: `path@agent-skills@fragments/tripwire-standard.md`, included in
  decision-recording, planning, the design skill, issue-tracking, the decision-record reviewer and
  the tripwire auditor.
- **the reversal fragment**: `path@agent-skills@fragments/reversal-check.md`.
- **the four title tests**, as the shape fragment states them today: (1) "A title states a decision
  only while it is false of the nearest rival it beat"; (2) "A title states the rule that decided,
  not the list of what it admits today"; (3) "A title states no more than its argument argues";
  (4) "The slug and the title stay aligned with the full scope of the decision".
- **the rule on slips**: the standard fragment's paragraph "A slip is no firing", lines 33-37.
- **a later procedure**: a procedure that reads the work after the procedures of the work itself
  have passed it, such as an audit, the review of a later branch that reads the text without having
  written it, or a retrospective file the intake receives. The property decides, not the list.
- **the plain-word repair**: the repair text of the function `id_finding` in
  `path@check@src/check/references.rs`, with its comment, and the tests of that file's `tests`
  module that assert it.
- **the title script**: the Python script of the commit that repairs this spec after its plan
  reviews, named by its subject, "[plans] Repairs of the plan reviews of the spec on head titles
  that must not mislead"; its message holds it whole, since no tree does. It is run from the
  repository's root as `python3 -I <script> <commit> .`: every `###` heading carrying a slug in a
  file of a design home, `register@*@design`, a directory home's README excluded, the mock projects
  under `path@testing@tests/projects/` excluded, the slug removed.

## What the work is

What exists today, each read at main `28f369b5`:

- The shape fragment, lines 44-47: test 1, with its example; lines 49-55: test 2, ending with test
  3, "A title states no more than its argument argues."; lines 88-91: test 4, "The slug and the
  title stay aligned with the full scope of the decision … A slug or a title that misdescribes its
  decision misinforms every reader, or undermines the decision it names, so rename it even when that
  means rewriting every reference in the project."
- The shape fragment also leans on tests 1 and 4 outside them: lines 26-30, the thread-slug
  paragraph, "unless that name misdescribes the decision as approved … When the name misdescribes
  the decision, the entry takes a slug that names the decision, by the alignment rule below"; lines
  83-85, the reading by direction, "a clause that relaxes it, drops part of it or turns it can
  still be false of the nearest rival, and is still not the position the owner was shown".
- The reversal fragment, lines 33-34: "The slug and the title stay aligned with the full scope of
  the decision, per `skill@knowledge-architect-decision-recording@head-shape`."
- The standard fragment, lines 33-37: "A clause on how sessions follow an instruction never fires on
  one instance that a review, a check or a retrospective reported under its own procedure: one slip
  is the review working. It fires on the same failure recurring in separate pieces of work, or on
  one instance that reached the main branch with no procedure reporting it. Where a clause fires on
  both a slip and something more, strike the slip."
- Hand-written texts that name the tests or the rule on slips by their words:
  `path@agent-skills@content/agents/decision-record-reviewer.md` line 111 and
  `path@agent-skills@content/agents/design-record-auditor.md` line 119, "the slug, its level and
  its alignment"; `path@agent-skills@content/agents/design-record-auditor.md` line 51, "move a title
  to a rule wider than the head's argument"; `path@agent-skills@content/agents/tripwire-auditor.md`
  line 54, "strike a clause that is not a slip, or keep a slip beside the clause it is struck from",
  and line 107, "then the slip"; `path@agent-skills@content/skills/project-audit/SKILL.md` line 396,
  "a slip struck".
- `design@agent-skills@title-states-the-rule` argues tests 2 and 3. `design@agent-skills@tripwire-admission-test`
  states the rule on slips, and its sentence "That a recurrence fires is the session's argument,
  since a recurrence is how such a premise shows false." Its last paragraph, lines 117-121 of
  `path@agent-skills@docs/design/premortem-and-tripwires.md`, ties the owner's ranking to the rule:
  "a refinement admitting a dropped useful firing was taken only where it admitted none of the 25
  found anyway, which is the rule on slips' second case".
- `design@check@plain-word-lint` and the plain-word repair ask that a design entry whose slug is a
  plain word be renamed "so that its slug states its decision", "as
  `skill@knowledge-architect-decision-recording@head-shape` asks of a slug".
- Every tripwire of every tripwires home, read whole, by what its firing clause counts. Twelve
  homes hold none: those of the root, cli, create, gates, git, mutate, query, testing, transcript
  and xtask; check holds three and model two.

  | tripwire | what its clause counts | under the new rule |
  | --- | --- | --- |
  | `tripwire@agent-skills@a-shortcut-decision-earns-a-head` | a decision-record review's findings on its own branch, recurring, or one found on main | in the rule: its recurrence clause struck, step 3 |
  | `tripwire@agent-skills@split-rule-from-exception` | the reviews of two branches each finding one in a head their branch writes, or an audit on main | in the rule: its recurrence clause struck, step 3 |
  | `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head` | an audit or a later review on main, or the reviews of two branches each finding one their branch added | in the rule: its recurrence clause struck, step 3 |
  | `tripwire@agent-skills@silent-read-too-widely` | an issue opened on a silent failure whose cause a commit dropped | in the rule, and conforms: the issue is a later procedure |
  | `tripwire@agent-skills@intake-repairs-interact-unheld` | an audit report of the agentic-workflow axis | in the rule, and conforms |
  | `tripwire@agent-skills@a-measure-protected-instruction-is-removed-as-unneeded` | an analysis of a received retrospective file, or an audit report | in the rule, and conforms |
  | `tripwire@agent-skills@needless-removal-found-missing` | the same | in the rule, and conforms |
  | `tripwire@agent-skills@harvested-item-built-after-its-reversal` | an issue, an audit report or a retrospective file on work merged on main | in the rule, and conforms |
  | `tripwire@agent-skills@record-check-misses-a-head` | the reviews of two audits' branches finding a repair the check of its proposal missed | outside: an earlier procedure's misses |
  | `tripwire@agent-skills@verifier-refutes-a-true-finding`, `tripwire@agent-skills@verifiers-wave-findings-through`, `tripwire@agent-skills@conforming-draft-wrong` | the session's verification finding a verifier wrong | outside: an earlier procedure's misses |
  | `tripwire@agent-skills@calibration-reading-rejected`, `tripwire@agent-skills@rule-misread-across-groups` | the session's verification finding the calibration wrong or short | outside: an earlier procedure's misses |
  | `tripwire@agent-skills@overlap-missed` | the application meeting a collision the cross-check missed | outside: an earlier procedure's misses |
  | `tripwire@agent-skills@audit-drafts-mostly-noise` | drafts the reading of a pass rejects | outside: the decision's outcome in use, the reading a pass spends |
  | `tripwire@agent-skills@duplicate-auditors-add-nothing` | what a second agent of a lens adds | outside: the decision's outcome in use |
  | `tripwire@agent-skills@shape-findings-fragment-code`, `tripwire@agent-skills@logic-tests-relabelled` | received retrospective files reporting the decision's result in two pieces of work | outside: the decision's outcome in real use, as the owner ruled; a received file is also a later procedure |
  | `tripwire@check@phases-gate-the-report-two`, `tripwire@check@private-item-needed`, `tripwire@check@fix-makes-a-choice` | an event: a flag added, a consumer's need, a fix added | outside: no clause on how sessions follow an instruction |
  | `tripwire@model@candidate-rule-silence` | a span the check left silent | outside: an earlier procedure's misses |
  | `tripwire@model@walked-count-differs-between-machines` | a count measured on two machines | outside: a measurement |

- `titles-wider-than-their-argument-recur-across-branches`, an issue of agent-skills, held the
  firing of the deleted tripwire `rule-title-wider-than-its-argument`: from 2026-10-24, the reviews of
  five merged branches, pull requests 38, 39, 43, 57 and 73, each found and repaired a title wider
  than its argument in a head the branch wrote or rewrote; two are checked against the commit that
  wrote the head, three rest on what their review messages state. No pass of the design-record
  axis ran since 2026-10-24, so a title a review missed is not known. The commit that adds this spec
  closes the issue, since the spec schedules its work, per
  `skill@knowledge-architect-planning@spec-or-milestone`. This paragraph carries its instances, and
  the new tripwire's premise carries its suspected mechanism; the rest, the commit subject of each
  instance and its "What would close it", is read with
  `git show 28f369b5:crates/agent-skills/docs/open-issues/titles-wider-than-their-argument-recur-across-branches.md`.
- Title lengths, measured at main `28f369b5` by the title script: 613 heads; median 17 words, 90th
  percentile 27, longest 53; body median 202 words; median title/body ratio 8.9 %; by title length,
  under 10 words 52 heads, 10-19 323, 20-29 200, 30-39 29, 40 or more 9. The longest median of a
  Component is query's, 21 over 26 heads; agent-skills' is 20 over 295. The same script at
  `68841c55~1`, before "The harvest of the head-rules work" (2026-10-09): median 14 over 196 heads;
  at `2ad7d3d9` (2026-10-24): median 16 over 341.
- In the branch of "The spec on when a default enters a head", a head's title reached 51 words, the
  transcript-reviewer head added by c811787e, "[review] Repairs of the review before the merge of
  the spec on when a default enters a head …": `git show c811787e --format= -- '*docs/design*'`,
  its added `###` lines counted with the slug removed.

Outside the work:

- `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps` stays
  open: this spec lists the heads its steps rewrite apart from those its harvest touches. The issue
  holds no default; the default is the session's, from the discussion, and whether the owner's
  "agreed on this table" covered it is D1, which the owner ruled: the default.
- `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: its trigger is not met, since no
  decision of this spec turns on review cost.
- Heads whose titles the new test would judge differently: converted when a change touches them,
  per `design@agent-skills@existing-heads-on-touch`, or by the next design-record audit pass, which
  `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis` schedules, step 5.
- The released sections of `path@knowledge-architect@CHANGELOG.md` and their copies state tests 1
  and 4: a released section's content never changes, per
  `design@knowledge-architect@changelog-section-fixed-at-release`.

## What is already decided

Rests on, not argued again: `goal@design-is-recorded-with-its-arguments`,
`goal@agents-work-without-drift`, `goal@only-needed-instructions`,
`goal@the-workflow-improves-through-real-use`,
`design@agent-skills@reviews-judge-by-the-writer-s-criteria`,
`design@agent-skills@extension-follows-the-ground`, `design@agent-skills@one-decision-per-head`,
`design@agent-skills@existing-heads-on-touch`, `design@agent-skills@tripwire-stands-on-the-standard`,
`design@agent-skills@tripwire-horizon`, `design@agent-skills@tripwire-audit-axis`.

Rewritten by the steps, with every text the change repairs and the step that judges or updates it:

| decision | texts it repairs | step |
| --- | --- | --- |
| `design@agent-skills@title-states-the-rule`: keeps test 2; test 3, which it argues today, leaves it for #title-misleads-bar, with tests 1 and 4, which no head argues | the shape fragment, lines 44-47, 55 and 88-91 (tests 1, 3, 4), lines 26-30 ("misdescribes", "the alignment rule below") and lines 83-85 ("false of the nearest rival"); the reversal fragment, lines 33-34; decision-record-reviewer.md line 111 and design-record-auditor.md line 119 ("its alignment"); design-record-auditor.md line 51 ("a rule wider than the head's argument", which the new test keeps a failure: read, reworded only if it reads as another test) | 1 |
| the same | the head itself, split; `design@agent-skills@one-home-for-head-rules`, whose sentence "three rules on heads that have no head of their own: present tense, the alignment of the slug and the title, …" turns false once #title-misleads-bar has a head; the `%%` comments of the shape fragment, which name the new heads | 5 |
| `design@agent-skills@thread-slug-is-entry-id`: its exception's trigger, "misdescribes", becomes "misleads", by its own argument: the exception exists so that the thread-slug rule and the rename order of the slug rule never give opposite orders, and the rename order now fires on a slug that misleads | the head's second paragraph, lines 171-178 of `path@agent-skills@docs/design/what-earns-a-record.md`; the shape fragment, lines 26-30 (step 1) | 1, 5 |
| `design@check@plain-word-lint`: a design entry whose slug is a plain word is renamed so that its slug is not an ordinary word, as every other entry is, since a slug that is an ordinary word collides with the prose that uses the word; no longer because the shape fragment asks a slug to state its decision | the head, lines 136-138 of `path@check@docs/design/reference-lints.md`; the plain-word repair: its comment and its repair texts, "rename the entry so that its slug states its decision" and the mixed arm's "so that its slug states its decision", become "so that its slug is not an ordinary word", and the tests that assert them; whether the arms merge is the implementer's | 2, 5 |
| `design@agent-skills@tripwire-admission-test`, its rule on slips: the recurrence branch reversed, the rule moved to #review-finding-is-no-firing | the standard fragment, lines 33-37, and its `%%` comment, line 1 (step 5); tripwire-auditor.md lines 54 and 107 and project-audit SKILL.md line 396, which keep the word "slip" since the new text keeps it as a term (read, no change expected); the three tripwires above; the head's first paragraph (lines 89-92) and its sentence on recurrence; its last paragraph, lines 117-121, per D3; `design@agent-skills@tripwire-stands-on-the-standard`, lines 59-61, "is a case of the rule on slips, which the standard states as the owner's", re-pointed to the new head (step 5) after step 4 conforms it | 1, 3, 4, 5 |
| the same | the `Next release` Migration entry of `path@knowledge-architect@CHANGELOG.md`: the standard changes what a recorded tripwire must meet, so it names the tripwire axis, `skill@knowledge-architect-project-audit@tripwire-axis`, per `design@agent-skills@tripwire-audit-axis`; the title test changes what a head must meet, so it names the design-record axis | 1 |

The other sites `cargo klarch show` lists for the two agent-skills decisions were read at main
`28f369b5`, and need no change, since each cites test 2 or a condition of the standard, which the
work keeps: for `design@agent-skills@title-states-the-rule`, design-skill-and-heads.md line 215,
rejected.md line 133, the issues `issue@agent-skills@a-lighter-pass-for-untouched-components` line 25,
`issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it` line 88 and
`issue@agent-skills@audit-axes-beyond-the-design-record` line 116, and `spec@plans@path-quickfixes`; for
`design@agent-skills@tripwire-admission-test`, audit-record-axes.md lines 38 and 48,
plan-documents.md line 385, premortem-and-tripwires.md lines 10, 39, 43 and 155, tripwires.md lines
176 and 185, and the issue
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` line 97.
Each step that edits a text reads the sites `cargo klarch show` then lists for it again.

Touched by the harvest without reversing or rewriting their decisions. One
`agent@knowledge-architect-design-record-auditor`, briefed as a drafter at the spec's commit,
judged them, a single group with no sample; the session verified each draft against the head's
text and its history:

- `design@agent-skills@one-home-for-head-rules`: conforms.
- `design@agent-skills@tripwire-stands-on-the-standard`: fails what a head stands on, twice: "with
  the owner's approval" cites an approval as a ground, which 2ad7d3d9 records as the approval of the
  session's defaults; and "which the standard states as the owner's" attributes to the standard
  what only `design@agent-skills@tripwire-admission-test` states. Step 4 removes the first and
  turns the second into a reference to that head; neither changes a word of the owner.
- `design@agent-skills@application-judged-by-diff`: fails the title test, since its title admits an
  entry whose only change is a re-pointed reference, which the body leaves out; and what a head
  stands on, since its thaum counts name no source. Step 4 retitles it, its slug kept, and names
  the source: the analysis of thaum's workflow file on its pin move and design-record audit,
  `path@elsewhere@docs/retrospective-reports/2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow.md`,
  in the commit d18f3e6a, "The analyses of seven retrospective files, with the owner's ruling on
  each". The design-record auditor that checked the retitle found that the head bundles three
  threads, so step 4 split it into
  `design@agent-skills@application-judged-by-diff`,
  `design@agent-skills@application-judge-reads-tripwires` and
  `design@agent-skills@application-judge-in-workflow`, the first citing the file's W1 and the second
  its W3.

What the harvest then changes in each, step 5:

- `design@agent-skills@one-home-for-head-rules`: its list of the rules with no head loses "the
  alignment of the slug and the title".
- `design@agent-skills@tripwire-stands-on-the-standard`: its sentence on the rule on slips is
  re-pointed to the new head.
- `design@agent-skills@application-judged-by-diff`: its evidence, "found 19 failures the run's
  checks could not see, among them a retitle that made a title true of a recorded rival", narrows:
  that retitle is no failure under #title-misleads-bar unless it widened the title, so the
  sentence names the rules the count was taken under, those of knowledge-architect 0.6.0, and keeps
  the example only where the source shows the retitle misled,
  `git show 8f37ffc5^:docs/retrospective-reports/2026-10-12-thaum-pin-move-and-design-record-audit-klarch-workflow.md`.

Entries of the record search whose outcome changed something:

- `design@agent-skills@title-states-the-rule`: a title that leaves out a condition misleads where
  the condition narrows what the title admits (#title-misleads-bar).
- `design@agent-skills@reviews-judge-by-the-writer-s-criteria`: test 1 becomes advice that binds no
  one, so no writer's criterion goes unjudged by a review (#title-misleads-bar, shape A).
- `design@agent-skills@tripwire-admission-test`: its sentence "That a recurrence fires is the
  session's argument" is reversed (#review-finding-is-no-firing); the owner's "one slip is no
  firing" stays, in the head and in the fragment's text.
- `titles-wider-than-their-argument-recur-across-branches`: closed by the commit that adds this
  spec.
- `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps` and
  `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: as "Outside the work" says.

The plan reviews added, against verdicts the search gave and the document had left unanswered:

- `goal@the-workflow-improves-through-real-use` and
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: a
  retrospective file the intake receives is a later procedure, so a clause on the installed text
  keeps the one channel by which a consumer's real use reaches this repository
  (#review-finding-is-no-firing).
- `design@agent-skills@session-removes-an-excluded-tripwire`: every tripwire of every home is read
  under the new rule, in the table above.
- `design@agent-skills@application-judged-by-diff` and `design@agent-skills@tripwire-audit-axis`:
  as the tables above say.

The other verdicts: [the discussion's search](search/discussion-1.txt).

## Criteria

### A title states no rule its body does not argue `##no-unargued-title`

Binding, from `goal@design-is-recorded-with-its-arguments`. Met by #title-misleads-bar: a title
wider than its argument misleads, and stays a finding.

### A session acting on titles alone does not act against the body `##titles-do-not-cause-drift`

Binding, from `goal@agents-work-without-drift`. Met by #title-misleads-bar.

### No title rule where judgement does well enough `##fewer-title-rules`

Binding, from `goal@only-needed-instructions`. Met by #title-misleads-bar and #review-finding-is-no-firing.

### Titles stay far below a restatement of their body `##titles-stay-short`

Weighed, stated in the discussion. Met by #title-length-bound.

## New names, in one place

Each new name is written here by its thread's name. The slug each entry takes is in the Harvest
table, which departs from the thread's slug where that slug misleads about the decision as
approved.

```
#title-misleads-bar               crates/agent-skills/docs/design/design-skill-and-heads.md, design heads, at the harvest
#title-length-bound               crates/agent-skills/docs/design/design-skill-and-heads.md, a design head, at the harvest
#review-finding-is-no-firing      crates/agent-skills/docs/design/premortem-and-tripwires.md, a design head, at the harvest
#title-tripwire-verdict           crates/agent-skills/docs/tripwires.md, a tripwire, at the harvest
```

No name in code: step 2 changes a message and a comment.

## Decided design

### #title-misleads-bar

A title must not mislead, and the slug must not either; test 1 stops binding. One test, stated for
the title and again for the slug, replaces tests 3 and 4. The shape fragment's new text, to be
written in step 1:

- lines 44-47, test 1, become: "**A title false of the nearest rival it beat tells the reader most.**
  A title that the losing alternative would make true names a subject, not a decision: …" (the
  example kept), ending "This binds no one: a title true of its rival is vague, and vagueness alone
  does not mislead."
- line 55, "A title states no more than its argument argues.", becomes a paragraph of its own:
  "**A title must not mislead.** A title misleads when a session acting on the title alone would do
  what the body excludes: add a member the body does not admit, or take a scope its argument does
  not argue. A title that leaves something out misleads only where what it leaves out narrows what
  the title admits, as an exception or a limit does. A title states the core of the decision, and
  the conditions that do not narrow it are in the body."
- lines 88-91, test 4, become: "**The slug and the title do not mislead about the decision**: the
  slug is often the only part a reader sees, in a citing document or in code, and the title is what
  a document outline shows. A slug or a title that misleads misinforms every reader, or undermines
  the decision it names, so rename it even when that means rewriting every reference in the
  project."
- lines 26-30: "misdescribes" becomes "misleads about", twice, and "by the alignment rule below"
  becomes "by the rule below on a slug that misleads".
- lines 83-85: "can still be false of the nearest rival" becomes "can still meet every title test".
- the reversal fragment, lines 33-34: "The slug and the title do not mislead about the decision,
  per `skill@knowledge-architect-decision-recording@head-shape`."

The bar is the owner's proposal in round 1: a title need not be perfectly precise as long as it
does not mislead. The definition of "mislead", the test replacing tests 3 and 4, and test 1 as
advice that binds no one are the session's, which the owner approved with the checkpoint table of
round 4. The argument: tests 1 and 4 together push a title toward restating its body, as each gap
a review finds adds a clause (the 51-word title of "What the work is"), while the defect that costs
a reader, acting on a rule the body does not decide, is the one test 3 caught. The nearest rival,
keeping the four tests, keeps that pressure.

Test 1 has two rivals. Binding the writer alone, the session's default of round 2, lost on
`design@agent-skills@reviews-judge-by-the-writer-s-criteria`: a review judges by the writer's
criteria, so a criterion binding the writer binds the review. Shape B, test 1 kept as a binding
criterion of the writer and of the review, reported only where the title reads as the rival's
decision, lost in the weak form: shape A over shape B, the session ranked one title rule fewer and
titles kept short above what B's reviews would give, titles pushed to be false of their rival, a
ranking no goal makes; the owner approved it with the checkpoint table of round 4.

### #title-length-bound

No word bound on a title. The title states the core of the decision, and the conditions that do not
narrow it are in the body, in the text of #title-misleads-bar. The rival, a word bound, has no
argument that places a value, and a set closed on purpose lists its members in its title. The
measurement, "What the work is", shows titles at a median of 8.9 % of their body.

### #review-finding-is-no-firing

The standard fragment's rule on slips, lines 33-37, becomes:

"**A slip is no firing.** A slip is a failure to follow an instruction that the review of the work
itself, or a check run on that work, reports. A clause on how sessions follow an instruction never
fires on a slip, however often slips recur in separate pieces of work, since each slip is that
procedure working, and the failures it misses are what the clause can watch. The clause fires only on a failure that passed the procedures of its own work and
that a later procedure reports, such as an audit, the review of a later branch that reads the text
without having written it, or a retrospective file the intake receives. A clause that watches
whether an earlier procedure catches what it exists to catch, or what a decision gives in real use,
is outside this rule. Where a clause fires on both a slip and something more, strike the slip."

The rule is stated by its property, a procedure later than the work, and its list gives examples.
A retrospective file the intake receives is one, since a consumer's real use reaches this
repository only that way, per `goal@the-workflow-improves-through-real-use`. The word "slip" stays a
term, so the texts that apply it keep their words. The table of "What the work is" gives each
tripwire's outcome under the rule.

The three tripwires in the rule lose their recurrence clause. Their lines, as built, the lines of
`tripwire@agent-skills@a-shortcut-decision-earns-a-head` and the merge-back clause of
`tripwire@agent-skills@split-rule-from-exception` as the tripwire auditor's judgement of step 3 and
the review before the merge gave them:

- `tripwire@agent-skills@a-shortcut-decision-earns-a-head`:
  - "**Fires when:** a later procedure, such as an audit, the review of a branch that did not make
    it, or a retrospective file the intake receives, finds on the main branch a decision taken on
    the bounded path that passes an entry test of
    `design@agent-skills@a-head-is-owed-by-an-entry-test`."
  - "**Re-entry:** the decision-record review of a branch whose diff touches code that a commit made
    on the bounded path by another branch introduced, which reads that commit's message; the intake
    of a retrospective file, which reads each tripwire whose firing evidence the file carries; and
    each pass of a project audit, whose report marks a finding that bears on a tripwire."
  - "**Watches until:** three commits on the main branch whose message says its proposal was put to
    the owner as bounded work, found by `git log -i --grep='as bounded work'` and confirmed by
    reading each, and for each of them a later commit on the main branch that changes a file it
    changed and that the first release tag containing it does not contain, counted from the commit
    that writes this line", with the reason that a commit no later tag contains came from another
    branch.
- `tripwire@agent-skills@split-rule-from-exception`:
  - "**Fires when:** a commit on the main branch merges back two heads that another branch split
    under the decision, on the ground that they were one decision when split, such as a rule and
    its own exception; or an audit, or the review of a branch that did not write the head, finds on
    the main branch an exception split from its rule."
  - "**Re-entry:** each pass of the design-record axis of a project audit, the decision-record
    review of a change that touches a head it did not write, and the standing-state review, for a
    commit that merges back two heads that another branch split, on that ground."
  - "**Watches until:**" kept: "20 commits on the main branch that add a level-three heading to a
    design home, and one pass of the design-record axis", with its sentence on a relocation.
- `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head`:
  - "**Fires when:** an audit, or the review of a branch that did not add the citation, finds on the
    main branch a `rejected` entry whose only citing heads decide a question other than the one the
    design it records answered."
  - "**Re-entry:** the design-record axis of `skill@knowledge-architect-project-audit`, and the
    decision-record review of a branch that touches a head citing a `rejected` entry it did not
    add."
  - "**Watches until:**" kept: "15 commits on the main branch that add a level-three heading to a
    `rejected` home, and one pass of the design-record axis."

The argument: the owner's, in rounds 2 and 3: a finding a review reports on its own branch shows the
review working; counting such findings suffers from survivor bias, since a lapse no review caught
is not known; what makes such a lapse observable is a later procedure reading the same text on
main. The session's reading against the standard, from round 2: counting such findings fires on
the procedure working, which condition 3 excludes. It reverses the sentence of
`design@agent-skills@tripwire-admission-test` "That a recurrence fires is the session's argument,
since a recurrence is how such a premise shows false"; the owner's "one slip is no firing" stays.
The nearest rival, recurrence kept as a firing, measures review cost, which is not silent: the
reviews report those findings, which condition 1 excludes. A count of reviews that find nothing, as
a sign of blind reviews, lost: it cannot tell clean work from a blind review, and a defect a later
procedure finds on main shows a blind review.

The harvest writes the rule as a head of its own, `design@agent-skills@slip-is-no-firing`, in
`path@agent-skills@docs/design/premortem-and-tripwires.md`, since its nearest rival, the recurrence
branch, is not the nearest rival of the four conditions, per
`design@agent-skills@one-decision-per-head`; `design@agent-skills@tripwire-admission-test` keeps the
four conditions, and cites it and `design@agent-skills@doubt-keeps-the-cause`, the doubt, which the
harvest splits out on the same test.

### #title-tripwire-verdict

The decision of `design@agent-skills@title-states-the-rule` has no defect: every known instance was
reported by the review of its branch. The issue closes, and a tripwire watches a misleading title
found on main in its place, written at the harvest in `register@agent-skills@tripwire`. The block
writes `<title-misleads-bar>` where the harvest writes the reference to the new head of
#title-misleads-bar:

```markdown
## Guarding `<title-misleads-bar>`: a title held to not misleading stays within what its head decides `##misleading-title-found-on-main`

The decision rests on the premise that the test of a misleading title, which replaces the tests of
the argument's reach and of the decision's full scope, keeps a title from stating a rule its body
does not argue. The suspected mechanism of the earlier recurrence: a title moved from the members
to the rule states the rule in terms the session chooses, and those terms reach further than the
argument the body carries.

**Fires when:** an audit, or the review of a branch that did not write or retitle the head, finds on
the main branch a title that misleads, in a head written or retitled after the decision.
**Response:** reopen `<title-misleads-bar>`; the title found is repaired as the instance.
**Re-entry:** each pass of the design-record axis of a project audit, and the review of a branch
that touches a head written or retitled after the decision.
**Watches until:** the second report of the design-record axis in `path@knowledge-architect@docs/audit-reports/`
after the decision.
```

## Mapping tables

| title test today | after |
| --- | --- |
| 1, false of the nearest rival | advice, binds no one (#title-misleads-bar) |
| 2, the rule, not the list | unchanged (`design@agent-skills@title-states-the-rule`) |
| 3, no more than its argument argues | absorbed in "must not mislead" (#title-misleads-bar) |
| 4, aligned with the full scope | replaced by "do not mislead about the decision" (#title-misleads-bar) |

## Losing alternatives

- A check at writing time against titles wider than their argument: lost to
  #title-tripwire-verdict, since it is one more instruction aimed at a failure the review already
  reports.
- Recurrence across branches kept as a firing: lost to #review-finding-is-no-firing, since review
  findings are not silent.
- A count of reviews that find nothing: lost to #review-finding-is-no-firing, since it cannot tell
  clean work from a blind review.
- A word bound on titles: lost to #title-length-bound, since no argument places a value.
- No tripwire on the reshaped title decision: lost to #title-tripwire-verdict, since the next
  audit's findings would then reopen nothing.
- Test 1 binding the writer alone: lost to #title-misleads-bar on
  `design@agent-skills@reviews-judge-by-the-writer-s-criteria`.
- Test 1 kept as a binding criterion reported only where the title reads as the rival's decision
  (shape B): lost to #title-misleads-bar on the session's ranking, in the weak form its section
  states.

## Readings

The work reads no external specification.

## Premortem

1. **Reviewers read "misleads" too narrowly**, and misleading titles reach main. Stresses
   #title-misleads-bar. Watched by the tripwire of #title-tripwire-verdict.
2. **Tripwires on how instructions are followed stop firing, audits being rare.** Stresses
   #review-finding-is-no-firing. Each of the three tripwires step 3 edits, and the new one, can fire
   only on a later procedure. The horizons of `tripwire@agent-skills@split-rule-from-exception`,
   `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head` and the new one count a pass or a
   report of the design-record axis, so an audit not run shows as a horizon not reached rather than
   as a pass; that of `tripwire@agent-skills@a-shortcut-decision-earns-a-head` counts later commits
   that change a file a bounded-path commit changed. Named as a cause for the harvest to judge by
   the standard.
3. **Titles keep growing anyway.** Stresses #title-length-bound. No condition clearly excludes it:
   under #title-length-bound no review reports a long title, so condition 1 holds. Dropped under the
   standard's doubt paragraph: the owner sees titles while carrying work with the project's agents,
   in every review, so a wrong drop costs little.

## Acceptance criteria

None: the work changes text and one message, and no built code can apply a check on its decisions.

## Implementation sequence

1. **The fragments and the hand-written texts.** Builds: the new text of the shape fragment, the
   reversal fragment and the standard fragment, as "Decided design" gives it; the sites the table of "What is
   already decided" gives to step 1; `cargo klarch install-agent-skills`; the `Next release` entries
   of `path@knowledge-architect@CHANGELOG.md`, the Migration entry among them, after reading
   `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`
   whole, then `cargo x changelog`; `cargo x gates`. Claims: no installed text still states tests 3
   or 4, the recurrence branch of the rule on slips, or a slug rule on "misdescribes": a search,
   matched across a line break, for "full scope of the decision", "no more than its argument
   argues", "recurring in separate pieces of work", "misdescrib", "alignment" and "aligned" over
   `path@agent-skills@content/`, `path@agent-skills@fragments/` and the installed copies,
   `path@agent-config@skills/`, `path@agent-config@agents/` and
   `path@agent-config@knowledge-architect/`, the worktrees under `.claude` excluded, returns no hit that applies a removed test, a lead read with each including text read
   whole; every text that includes a changed fragment reads consistently with it. Fails alone on:
   an including text that still applies a removed test.
2. **The plain-word repair.** Builds: the plain-word repair's comment and texts, and their tests,
   as "What is already decided" gives them; its `Next release` entry; `cargo x gates`. Claims:
   each test that asserts a repair text is shown to fail with `cargo x mutate` on the old text; no
   repair text of `id_finding` says "states its decision". Fails alone on: a test that does not
   discriminate the new text.
3. **The tripwires.** Builds: the three tripwires' new lines, as "Decided design" gives them.
   Claims: each tripwire still guards its premise, judged by
   `agent@knowledge-architect-tripwire-auditor` briefed on an application. Its brief names, in
   place of an audit pass's commits and report file, the commit of step 3 as the application and
   its parent as the commit audited, the three entries it edits, and "Decided design"
   #review-finding-is-no-firing as the ruling the edit carries out. Fails alone on: a tripwire whose
   remaining clause no longer matches its premise.
4. **The touched heads conformed.** Builds: the edits of
   `design@agent-skills@tripwire-stands-on-the-standard` and
   `design@agent-skills@application-judged-by-diff` that "What is already decided" gives, per
   `skill@knowledge-architect-planning@cutting-steps-and-slices`. Claims: the retitle passes the
   rules on heads, judged alone by `agent@knowledge-architect-design-record-auditor` briefed to
   check, before the step's commit. Fails alone on: a touched head that still fails a rule.
5. **The harvest.** Builds: the heads of #title-misleads-bar
   and #title-length-bound in `path@agent-skills@docs/design/design-skill-and-heads.md`, and that of
   #review-finding-is-no-firing in `path@agent-skills@docs/design/premortem-and-tripwires.md`, each
   under the slug the Harvest table gives;
   `design@agent-skills@title-states-the-rule` keeping test 2, its slug kept, its title given the
   exception of a set closed on purpose;
   `design@agent-skills@tripwire-admission-test` without the rule on slips, its last paragraph per
   D3; the other heads the table of "What is already decided" gives to step 5; the `%%` comments of the shape and
   standard fragments naming the new heads; the tripwire `tripwire@agent-skills@misleading-title-found-on-main`; the
   losing alternatives judged by `skill@knowledge-architect-decision-recording@losing-alternatives`;
   premortem cause 2 judged by the standard; a paragraph in
   `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis` that its pass also
   judges every title by #title-misleads-bar, so that a title grown by test 4 may be shortened, each
   as a change of its head's wording. Claims: each head written passes the rules on heads, judged
   alone by `agent@knowledge-architect-design-record-auditor` briefed to check. Fails alone on: the
   review before the merge, on its decision-record axis.

Then the review before the merge, and the deletion of this spec.

## Order rationale

- 1 before 2: the plain-word repair gives its reason once the slug rule it no longer cites is
  written.
- 2 before 3: the tripwires change no code, and step 2's gates pass on a tree without them.
- 3 before 4: the touched heads are conformed after the text they cite is built.
- 4 before 5: the harvest writes into heads that already conform.

## Defaults awaiting the owner

### D1: the touched-head issue stays open

Bears on the work's scope, `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`.
The session put it to the owner in the reply to round 3, as item 3, outside the two tables the owner
answered with "agreed on this table". **Default**, built: the issue stays open, and this spec lists
the heads its steps rewrite apart from those its harvest touches. **Rival**: hold that discussion
inside this spec, which would also rewrite the planning skill's definition and two more heads.
Ruled: the owner accepted the default.

### D2: the owner's hedges of round 3 restored

Bears on #review-finding-is-no-firing. The attribution of round 3 in
`argument@title-rules@arg-17` had dropped "I guess" and "probably"; it now keeps them, as
`primer@design-heads` asks of an attribution. **Default**: the attribution as `argument@title-rules@arg-17` now words it.
**Rival**: the owner rewords it. Ruled: the owner accepted the default.

### D3: the owner's ranking of the standard and the new rule on slips

Bears on #review-finding-is-no-firing. The last paragraph of
`design@agent-skills@tripwire-admission-test` says a refinement admitting a dropped useful firing
"was taken only where it admitted none of the 25 found anyway, which is the rule on slips' second
case". That case, one instance reaching main with no procedure reporting it, was built to admit one
useful firing, thaum's commit-message tripwire, a single message on the main branch no review had
reported, per the message of 71f26fd5 and D3 of the deleted spec `tripwire-standard`
(`git show 79c92392^:docs/plans/specs/tripwire-standard.md`). Whether the new rule still admits
that firing, and none of the 25, cannot be told: the head says "the judgements per firing were not
kept", and the spec names the firings without saying who found them. The new rule also stops
admitting a useful firing the old recurrence branch admitted, `member-beyond-the-argument`, whose
instances reviews repaired as lapses (the argument numbered 21 of that spec). **Default**, built at step 5: the
sentence is rewritten to state the ranking and the case it admitted, now stated as a failure a later
procedure reports, and says that whether the new case admits any of the 25 is not known, since
the judgements per firing were not kept; the dropped `member-beyond-the-argument` firing is within
the ranking, which put the value of the tripwires admitted above keeping every past useful one.
**Rival**: the work re-judges the 49 firings, from their commits in this repository and in thaum,
under the new case before the harvest. Ruled: the owner accepted the default.

### D4: a received retrospective file counts as a later procedure

Bears on #review-finding-is-no-firing. The position the owner agreed to in round 4 fires a clause
only on a defect that a later audit, or the review of a branch that did not write the text, finds
on main. The plan-review repair of this spec, the session's, added a retrospective file the intake
receives as a later procedure, which widens that "only". It rests on
`goal@the-workflow-improves-through-real-use`: a consumer's real use reaches this repository only
through such files, and `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`
asks for them to be kept as evidence. **Default**, built: keep it, in
`design@agent-skills@slip-is-no-firing` and the standard fragment. **Rival**: remove it; the two
tripwires that fire on retrospective analyses, `tripwire@agent-skills@shape-findings-fragment-code`
and `tripwire@agent-skills@logic-tests-relabelled`, then lose that clause. The review before the
merge put it to the owner; the branch merges once the owner rules.

### D5: a clause that watches what a decision gives in real use is outside the rule

Bears on #review-finding-is-no-firing. The same plan-review repair added that a clause watching
what a decision gives in real use stays outside the rule and keeps counting, which narrows the
rule's scope. Such a clause watches whether the decision is wrong, not a session's lapse; without
the exception the rule would strike the clauses of `tripwire@agent-skills@audit-drafts-mostly-noise`,
`tripwire@agent-skills@duplicate-auditors-add-nothing` and the two tripwires of D4. **Default**,
built: keep it, in `design@agent-skills@slip-is-no-firing`, its title and the standard fragment.
**Rival**: remove it, and judge those four tripwires again under the rule. The review before the
merge put it to the owner; the branch merges once the owner rules.

## Harvest

| thread | home |
| --- | --- |
| #title-misleads-bar | #title-misleads-bar → `design@agent-skills@title-misleads-bar` and `design@agent-skills@nearest-rival-title-is-advice`, two heads in `register@agent-skills@design`, design-skill-and-heads.md, since test 1 as advice loses to a nearest rival of its own |
| #title-length-bound | #title-length-bound → `design@agent-skills@no-title-word-bound`, a head of absence in `register@agent-skills@design`, design-skill-and-heads.md, since the thread's slug names the rival |
| #review-finding-is-no-firing | #review-finding-is-no-firing → `design@agent-skills@slip-is-no-firing`, a new head in `register@agent-skills@design`, premortem-and-tripwires.md, cited by `design@agent-skills@tripwire-admission-test`, since the thread's slug misleads about a later branch's review finding, which fires |
| #title-tripwire-verdict | the tripwire `tripwire@agent-skills@misleading-title-found-on-main` |

## Later consequences

The next pass of the design-record axis judges every title by #title-misleads-bar. Titles that test
4 grew by a clause may then be shortened, each as a change of its head's wording; step 5 writes this
into `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`.
