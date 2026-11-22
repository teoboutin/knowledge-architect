# A head's title must not mislead, and a tripwire fires on a defect a later procedure finds on main

## Status and audience

This spec plans the work of one branch, which changes installed text, the agent-skills design home
and its tripwires. It leaves when its work lands, in a commit of its own after the review before the
merge. Where it and a design home disagree on a point that "What is already decided" does not list
as reversed or rewritten, the design home wins. Every name it uses is defined in it or exists in the
tree. Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says
what waits.

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
whole before the first edit.

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
- **a later procedure**: an audit, or the review of a later branch that reads the text without
  having written it.

## What the work is

What exists today, each read at main `28f369b5`:

- The shape fragment, lines 44-47: test 1, with its example; lines 49-55: test 2, ending with test
  3, "A title states no more than its argument argues."; lines 88-91: test 4, "The slug and the
  title stay aligned with the full scope of the decision … A slug or a title that misdescribes its
  decision misinforms every reader, or undermines the decision it names, so rename it even when that
  means rewriting every reference in the project."
- The reversal fragment, lines 33-34: "The slug and the title stay aligned with the full scope of
  the decision, per `skill@knowledge-architect-decision-recording@head-shape`."
- The standard fragment, lines 33-37: "A clause on how sessions follow an instruction never fires on
  one instance that a review, a check or a retrospective reported under its own procedure: one slip
  is the review working. It fires on the same failure recurring in separate pieces of work, or on
  one instance that reached the main branch with no procedure reporting it. Where a clause fires on
  both a slip and something more, strike the slip."
- `design@agent-skills@title-states-the-rule` argues tests 2 and 3. `design@agent-skills@tripwire-admission-test`
  states the rule on slips, and its sentence "That a recurrence fires is the session's argument,
  since a recurrence is how such a premise shows false."
- Three tripwires of `register@agent-skills@tripwire` have a clause that counts what reviews report
  on their own branches:
  - `tripwire@agent-skills@a-shortcut-decision-earns-a-head`: "a decision-record review finds, in a
    commit made on the bounded path, a decision that passes an entry test …, at instances in two
    separate pieces of work";
  - `tripwire@agent-skills@split-rule-from-exception`: "or the reviews of two separate branches
    each find an exception split from its rule in a head their branch writes";
  - `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head`: "or the reviews of two separate
    branches each find one that their branch added".

  `tripwire@agent-skills@record-check-misses-a-head` counts reviews' findings too, but of a miss of
  an earlier check, a procedure failing; it is outside the work.
- `titles-wider-than-their-argument-recur-across-branches`, an issue of agent-skills, held the
  firing of the deleted tripwire `rule-title-wider-than-its-argument`: from 2026-10-24, the reviews of
  five merged branches, pull requests 38, 39, 43, 57 and 73, each found and repaired a title wider
  than its argument in a head the branch wrote or rewrote; two are checked against the commit that
  wrote the head, three rest on what their review messages state. No audit of the design record ran
  since, so a title a review missed is not known. The commit that adds this spec closes the issue,
  since the spec schedules its work, per `skill@knowledge-architect-planning@spec-or-milestone`;
  this paragraph carries its live content.
- Title lengths, measured at main `28f369b5` by a script over every level-three heading carrying a
  slug in the design homes, test mock projects excluded, the slug removed:
  612 heads; median 17 words, 90th percentile 27, longest 53; body median 199 words; median
  title/body ratio 9 %. The same script at earlier commits: median 14 before "The harvest of the
  head-rules work" (2026-10-09), 16 at 2026-10-24.

Outside the work:

- `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps` stays
  open: this spec lists the heads its steps rewrite apart from those its harvest touches, as its
  default asks, the owner having confirmed the table that carried it.
- `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: its trigger is not met, since no
  decision of this spec turns on review cost.
- Heads whose titles the new test would judge differently: converted when a change touches them,
  per `design@agent-skills@existing-heads-on-touch`, or by the next design-record audit pass.

## What is already decided

Rests on, not argued again: `goal@design-is-recorded-with-its-arguments`,
`goal@agents-work-without-drift`, `goal@only-needed-instructions`,
`design@agent-skills@reviews-judge-by-the-writer-s-criteria`,
`design@agent-skills@extension-follows-the-ground`, `design@agent-skills@one-decision-per-head`,
`design@agent-skills@existing-heads-on-touch`, `design@agent-skills@tripwire-stands-on-the-standard`,
`design@agent-skills@tripwire-horizon`.

Rewritten by the steps, with what they repair:

| decision | texts it repairs | step |
| --- | --- | --- |
| `design@agent-skills@title-states-the-rule` | the shape fragment's tests 1, 3 and 4; the reversal fragment, lines 33-34 | 1, harvest |
| `design@agent-skills@tripwire-admission-test`, its rule on slips | the standard fragment, lines 33-37; the three tripwires above | 1, 2, harvest |

Touched by the harvest without reversing or rewriting their decisions: none known; each step reads
every site `cargo klarch show` lists for the two decisions above.

Entries of the record search whose outcome changed something:

- `design@agent-skills@title-states-the-rule`: a title that leaves out a condition misleads where
  the condition narrows what the title admits (#title-misleads-bar).
- `design@agent-skills@reviews-judge-by-the-writer-s-criteria`: test 1 becomes advice that binds no
  one, so no writer's criterion goes unjudged by a review (#title-misleads-bar, shape A).
- `design@agent-skills@tripwire-admission-test`: its sentence "That a recurrence fires is the
  session's argument" is reversed on the owner's argument (#review-finding-is-no-firing); "one slip
  is no firing" stays.
- `titles-wider-than-their-argument-recur-across-branches`: closed by the commit that adds this
  spec.
- `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps` and
  `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: as "Outside the work" says.

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

### Review findings on titles cost less `##fewer-title-findings`

Weighed, stated in the discussion. Met by #title-misleads-bar: a narrower or vaguer title is no
finding.

## New names, in one place

The tripwire `misleading-title-found-on-main`, in `register@agent-skills@tripwire`, at the harvest.
No name in code.

## Decided design

### #title-misleads-bar

Tests 3 and 4 are replaced by one test, and test 1 stops binding. The shape fragment's new text, to
be written in step 1:

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
- the reversal fragment, lines 33-34: "The slug and the title do not mislead about the decision,
  per `skill@knowledge-architect-decision-recording@head-shape`."

The decision was the owner's proposal in round 1: a title need not be perfectly precise as long as
it does not mislead. Its argument: tests 1 and 4 together push a title toward restating its body,
as each gap a review finds adds a clause (in the branch of "The spec on when a default awaiting the
owner enters a head", a head's title reached 51 words that way), while the defect that costs a
reader, acting on a rule the body does not decide, is the one test 3 caught. The nearest rival,
keeping the four tests, keeps that pressure. Test 1 binding the writer alone was rival B; it lost on
`design@agent-skills@reviews-judge-by-the-writer-s-criteria`, under which a review judges by the
writer's criteria, so a binding writer's criterion is a review's.

### #title-length-bound

No word bound on a title. The title states the core of the decision, in the text above. The rival,
a word bound, has no argument that places a value, and a set closed on purpose lists its members in
its title.

### #review-finding-is-no-firing

The standard fragment's rule on slips, lines 33-37, becomes:

"**A finding a procedure reports is no firing.** A clause on how sessions follow an instruction
fires only on a defect that a later procedure finds in text already on the main branch: an audit,
or the review of a later branch that reads that text without having written it. A finding that a
review, a check or a retrospective reports on the work it reviews fires nothing, however often it
recurs: it is that procedure working, and the defects such procedures miss are what the clause can
watch. Where a clause fires on both such a finding and something more, strike the finding."

The three tripwires lose their clause that counts reviews' findings on their own branches, each
keeping its clause on a defect found on the main branch, worded as a later procedure finds it:

- `tripwire@agent-skills@a-shortcut-decision-earns-a-head`: "**Fires when:** an audit, or the review
  of a branch that did not make it, finds on the main branch a decision taken on the bounded path
  that passes an entry test of `design@agent-skills@a-head-is-owed-by-an-entry-test`."
- `tripwire@agent-skills@split-rule-from-exception`: "**Fires when:** a commit merges back two heads
  split under the decision; or an audit, or the review of a branch that did not write the head,
  finds on the main branch an exception split from its rule."
- `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head`: its first clause kept, the clause
  "or the reviews of two separate branches each find one that their branch added" removed.

The argument is the owner's, in rounds 2 and 3: a finding a review reports on its own branch shows
the review working, and counting such findings fires on the procedure working, which condition 3 of
the standard excludes; a lapse no review caught is the failure, and what makes it observable is a
later procedure reading the same text on main. It reverses the sentence of
`design@agent-skills@tripwire-admission-test` "That a recurrence fires is the session's argument,
since a recurrence is how such a premise shows false"; "one slip is no firing" stays. The nearest
rival, recurrence kept as a firing, measures review cost, which is not silent: the owner sees each
review and its findings, which condition 1 excludes. A count of reviews that find nothing, as a sign
of blind reviews, lost: it cannot tell clean work from a blind review, and a defect a later
procedure finds on main shows a blind review.

### #title-tripwire-verdict

The decision of `design@agent-skills@title-states-the-rule` has no defect: every known instance was
reported by the review of its branch. The tripwire written at the harvest, in
`register@agent-skills@tripwire`:

```markdown
## Guarding `design@agent-skills@title-states-the-rule`: a title held to not misleading stays within what its head decides `##misleading-title-found-on-main`

The decision rests on the premise that the test of a misleading title, which replaces the tests of
the argument's reach and of the decision's full scope, keeps a title from stating a rule its body
does not argue.

**Fires when:** an audit, or the review of a branch that did not write or retitle the head, finds on
the main branch a title that misleads, in a head written or retitled after the decision.
**Response:** reopen `design@agent-skills@title-states-the-rule`; the title found is repaired as the
instance.
**Re-entry:** each pass of the design-record axis of a project audit, and the review of a branch
that touches a head written or retitled after the decision.
**Watches until:** the second report of the design-record axis in `path@knowledge-architect@docs/audit-reports/`
after the decision.
```

The issue's second exit, the decision kept with the recurrence recorded as its cost, is what the
harvest writes into the head: the recurrence of titles wider than their argument, each caught by its
branch's review, is a cost the review carries.

## Mapping tables

| title test today | after |
| --- | --- |
| 1, false of the nearest rival | advice, binds no one |
| 2, the rule, not the list | unchanged |
| 3, no more than its argument argues | absorbed in "must not mislead" |
| 4, aligned with the full scope | replaced by "do not mislead about the decision" |

## Losing alternatives

- A check at writing time against titles wider than their argument: a failure the review already
  reports.
- Recurrence across branches kept as a firing: lost to #review-finding-is-no-firing.
- A count of reviews that find nothing: lost to #review-finding-is-no-firing.
- A word bound on titles: lost to #title-length-bound.
- No tripwire on the reshaped title decision: the next audit's findings would reopen nothing.
- Test 1 kept as a binding criterion reported only where the title reads as the rival's decision
  (shape B): lost on `design@agent-skills@reviews-judge-by-the-writer-s-criteria`.

## Readings

The work reads no external specification.

## Premortem

1. **Reviewers read "misleads" too narrowly**, and misleading titles reach main. Stresses
   #title-misleads-bar. Watched by the tripwire of #title-tripwire-verdict.
2. **Tripwires on how instructions are followed stop firing, audits being rare.** Stresses
   #review-finding-is-no-firing. Named as a cause for the harvest to judge by the standard; a
   horizon counted in audit passes shows an audit not run as a horizon not reached.
3. **Titles keep growing anyway.** Stresses #title-length-bound. Dropped: the owner sees titles in
   every review, so the failure is not silent, condition 1 of the standard.

## Acceptance criteria

None: the work changes text, and no built code can apply a check on its decisions.

## Implementation sequence

1. **The fragments.** Builds: the new text of the shape fragment, the reversal fragment and the
   standard fragment, as "Decided design" gives it; `cargo klarch install-agent-skills`; the
   `Next release` entry of `path@knowledge-architect@CHANGELOG.md`, after reading
   `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`
   whole, then `cargo x changelog`; `cargo x gates`. Claims: no installed text still states tests 3
   or 4, or the recurrence branch of the rule on slips (a search for "full scope of the decision",
   "no more than its argument argues" and "recurring in separate pieces of work" over
   `path@agent-skills@content/`, `path@agent-skills@fragments/` and the installed copies under the
   project's `.claude` directory returns nothing, a
   lead read with each including text read whole); every text that includes a changed fragment
   reads consistently with it. Fails alone on: an including text that still applies a removed test.
2. **The tripwires.** Builds: the three tripwires' new clauses, as "Decided design" gives them.
   Claims: each tripwire still guards its premise, judged by `agent@knowledge-architect-tripwire-auditor`
   briefed on an application. Fails alone on: a tripwire whose remaining clause no longer matches
   its premise.
3. **The harvest.** Builds: `design@agent-skills@title-states-the-rule` rewritten to the decided
   design, with the recurrence as its cost; `design@agent-skills@tripwire-admission-test` rewritten,
   its sentence on recurrence reversed; the tripwire `misleading-title-found-on-main`; the losing
   alternatives judged by `skill@knowledge-architect-decision-recording@losing-alternatives`;
   premortem cause 2 judged by the standard. Claims: each head written passes the rules on heads,
   judged alone by `agent@knowledge-architect-design-record-auditor` briefed to check. Fails alone
   on: the review before the merge, on its decision-record axis.

Then the review before the merge, and the deletion of this spec.

## Order rationale

- 1 before 2: the tripwires are reworded under the rule on slips as step 1 writes it.
- 2 before 3: the heads record the text and the tripwires as built.

## Defaults awaiting the owner

None.

## Harvest

| thread | home |
| --- | --- |
| #title-misleads-bar | `design@agent-skills@title-states-the-rule` |
| #title-length-bound | `design@agent-skills@title-states-the-rule` |
| #review-finding-is-no-firing | `design@agent-skills@tripwire-admission-test` |
| #title-tripwire-verdict | the tripwire `misleading-title-found-on-main`, and the cost paragraph of `design@agent-skills@title-states-the-rule` |

## Later consequences

The next pass of the design-record axis judges every title by the new test. Titles that test 4
grew by a clause may then be shortened, each as a change of its head's wording.
