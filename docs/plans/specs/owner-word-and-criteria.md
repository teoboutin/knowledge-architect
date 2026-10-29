# The owner's word and the agents' autonomy: acceptance criteria that test a decision, heads that stand on argument, no overrule, and a transcript reviewer that reads a ruling by its direction

## Status and audience

This spec is the plan document of one branch. It leaves when its work lands. Where it and a design
home disagree on a point "What is already decided" does not list as reversed or rewritten, the
design home wins. Every name it uses is defined in it or exists in the tree. Where the owner's word
is awaited, `skill@knowledge-architect-planning@working-a-slice` says what waits.

The discussion spans one session, the one that also built the branch merged as "The installed
workflow loads its skills on its own". It begins at the owner's message opening "About AC2:
Actually, I believe there might be a problem in how an acceptance criterion firing is handled".
The two goals it reworded are written in the branch's first commit, "[goals] The owner changes a
decision through a goal, a fact or an argument, and meets in an exchange what the goals and the
record leave open". After the plan reviews, the same session put six defaults to the owner, D1 to
D6, and refined `thread@owner-word-and-criteria@heads-stand-on-argument` on the owner's argument, in
rounds 12 to 14; and, after a second plan review, D7 and D8, in rounds 15 and 16. "Defaults
awaiting the owner" records them.

The work changes the Component `agent-skills`, its installed text and its design record; the goals
it rests on are the project root's. A marker `L<n>` is a line of the session's transcript file, the
numbering `cargo klarch transcript show <session> --full` prints, `<session>` being the session that
`cargo klarch transcript list --containing "About AC2: Actually, I believe there might be a problem"`
lists.

## How the work is done

By `skill@knowledge-architect-planning`, whose section `skill@knowledge-architect-planning@working-a-slice`
gives the work of a spec its points.

## Names

| name | what it names |
| --- | --- |
| the design skill | `path@agent-skills@content/skills/design/SKILL.md` |
| the planning skill | `path@agent-skills@content/skills/planning/SKILL.md` |
| the decision-recording skill | `path@agent-skills@content/skills/decision-recording/SKILL.md` |
| the transcript reviewer | `path@agent-skills@content/agents/transcript-reviewer.md` |
| the expectation set | the design skill's list in `skill@knowledge-architect-retrospective@expectation-sets`, in `path@agent-skills@content/skills/retrospective/SKILL.md` |
| the head rules | the fragments `path@agent-skills@fragments/head-entry-tests.md` and `path@agent-skills@fragments/head-ground.md`, which five texts host: the primer, the design skill, the decision-recording skill, the decision-record reviewer and the design-record auditor |
| the tripwire standard | the fragment `path@agent-skills@fragments/tripwire-standard.md` |
| a step claim | a claim of a step of a plan document, with the test that could refute it, per `skill@knowledge-architect-planning@cutting-steps-and-slices` |
| the residual fork | a choice the argument leaves open, which no goal ranks: a true fork, a threshold the owner sets, a local cost the owner accepts with no argument that weighs it |
| the weak form | a head's record of a choice made at a fork, "the owner chose X over Y, which no goal ranks", which an argument or a fact the fork's record does not hold, and that ranks the shapes, reopens without the owner |
| D1 to D8 | the defaults the plan reviews raised, in "Defaults awaiting the owner" |
| the counterparts goal | `goal@knowledge-architect@design-with-agents-as-counterparts` |
| AC2, AC4, Q3, option C | labels of items the owner ruled on, quoted from the transcript: AC2 an acceptance criterion of the earlier branch, AC4 the convergence criterion of the work that made the agentic-workflow audit axis, Q3 a default of the earlier branch, option C the owner's first option of round 6 |
| the earlier branch | the branch merged as "The installed workflow loads its skills on its own" |
| the overrule | the owner's power, in the goal before this branch, to "overrule any of them, at any time and for any reason" |

## What the work is

**What exists today at each site.** The sites were found by these searches over
`path@agent-skills@content/`, `path@agent-skills@fragments/` and `path@agent-skills@README.md`,
run at the commit that adds this spec:

```sh
grep -rn -i -E 'acceptance criteri|AC<n>|AC[0-9]' crates/agent-skills/content crates/agent-skills/fragments
grep -rn -i 'overrul' crates/agent-skills/content crates/agent-skills/fragments crates/agent-skills/README.md
grep -rn -i -E "head's ground|weighing the owner made|ruling on intent|chose where the argued|fidelity to what the owner approved|widens it, narrows it|no obligation added|wider, narrower|owner's words name the members" crates/agent-skills/content crates/agent-skills/fragments
```

Acceptance criteria:

- **The design skill**:
  - its overview (lines 38-39) ends at "the owner's rulings on its acceptance criteria";
  - the in-change path's record (lines 140-152) carries "the owner's rulings on acceptance
    criteria", and nothing is committed "before it has run and the owner has ruled on its
    acceptance criteria";
  - the `%%` comment above it (lines 122-128) argues "Nothing is committed before the premortem's
    rulings";
  - loop step 7 (lines 620-640) routes "a cause that only the built code can check" to an acceptance
    criterion, and asks the owner, for each one, whether it is applied, under a label `AC<n>`.
- **The planning skill**, at lines 3, 17-18, 39, 46, 174-178, 217-220, 300-306, 312, 331-344,
  345-346 ("A number in a criterion is a threshold the owner sets"), 352-361 (the firing and leaving
  rules), 451, 521, 563-564 and 584. Lines 451, 521 and 563 change only where they name the owner's
  ruling or the response:
  - its terms and the `skill@knowledge-architect-planning@spec-contents` rows (Premortem, Acceptance
    criteria, Defaults awaiting the owner);
  - its section `skill@knowledge-architect-planning@acceptance-criteria` ("An acceptance criterion
    stands on the owner's word", a criterion awaiting the owner, the standard's conditions 3 and 4
    alone);
  - its plan reviews, `skill@knowledge-architect-planning@working-a-slice`, and its leaving rules
    on criteria.
- **The tripwire standard**, condition 1 (line 13): "A check of the work being built is that work's
  acceptance criterion".
- **The primer**, `primer@labelled-items` (line 70): "as an acceptance criterion's into the comment
  at its test".
- **The decision-recording skill**, line 105, on a criterion at the harvest.
- **The standing-state reviewer**, lines 90-94: "a check after the build lives in the acceptance
  criteria section".
- **The design-conformance reviewer** reads acceptance criteria (lines 3, 17, 58, 74). Its reading is
  unchanged.

The overrule:

- **The design skill**:
  - its decision authority (lines 43-46): the owner "can overrule any decision of the discussion",
    and an explicit word, "an approval, a rejection, 'stop, do X'", ends the argument at any time;
  - "The weighing at a fork is theirs" (lines 107-115), and the `parked` state (line 348), "your own
    overruled dissent after the owner's explicit ruling";
  - "Who moves what" (lines 303-320): the owner's explicit word closes a thread;
  - loop step 4 (line 543): "only an explicit decision converts their statement into a directive";
  - `skill@knowledge-architect-design@threads-and-states` (line 377): a criterion derived from a
    recorded decision binds "until the owner says otherwise",
    `a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`.
- **The decision-recording skill**, line 111: "No cause is put to the owner, who overrules by
  reading the work".
- **The project-audit skill**, line 395: the owner "overrules any of them".
- **The expectation set**:
  - "The owner says why when rejecting something";
  - "'Stop, build X' and 'park this' both end the argument at once".
- **The README** of agent-skills:
  - line 95, the `parked` state's "own overruled dissent";
  - lines 208-210, the restatement of the expectation set, "**You end a discussion rather than
    dropping it.** "Stop, build X" and "park this, I want to think" are both handled and both end
    the argument immediately", per `design@agent-skills@expectation-set-bounds-scope`.

The ground of a head, and how the owner's words are read:

- **The head rules**, the fragments `path@agent-skills@fragments/head-entry-tests.md` and
  `path@agent-skills@fragments/head-ground.md`, hosted by five texts: the primer, the design
  skill, the decision-recording skill, the decision-record reviewer and the design-record auditor.
  - Entry test 4: "the owner confirms that it records the owner's own intent".
  - The ground (head-ground.md lines 16-17): the owner's words are a ground where "the owner
    proposed it, the owner chose where the argued rivals did not settle it, or it rests on a premise
    only the owner can state".
  - "What the ground means to a later session": "A head whose ground is the owner's words is
    reversed by the owner".
  - Member routing (head-entry-tests.md lines 64 and 69-74): an approval of a member is "recorded
    in the commit, attributed to the owner", and a member the owner's words name goes to the owner.
- **The fragment** `path@agent-skills@fragments/head-shape.md`, lines 66-70: a clause that "widens
  it, narrows it, or drops part of it" goes to the owner. It is hosted by the decision-recording
  skill, the decision-record reviewer and the design-record auditor.
- **The reviewers and the auditor**:
  - the decision-record reviewer (line 109) and the design-record auditor (line 118) judge "fidelity
    to what the owner approved";
  - the transcript reviewer's description (line 3), its section
    `agent@knowledge-architect-transcript-reviewer@check-recorded-rulings` (lines 91-110), with a
    ruling's scope "not wider, not narrower, and with no obligation added that the owner did not
    rule" at 104-106, and its severities in `agent@knowledge-architect-transcript-reviewer@how-to-report` (lines 111-125).
- **The planning skill**, lines 526-528: the transcript review checks that "no ruling of the owner is
  recorded wider, narrower or in another state".
- **The retrospective skill**, lines 103-106: "A ruling of the owner recorded a little wider,
  narrower or firmer than given".
- **The decision-recording skill**, `skill@knowledge-architect-decision-recording@unargued-approval`
  (lines 50-56): "The owner's choice is then the head's ground".
- **The design skill**:
  - line 140, the in-change record's "the owner's ruling, attributed to the owner";
  - line 199, the bounded path's "the owner's answer, attributed to the owner".
- **The primer**:
  - `primer@question-authority` (line 31), "a fact only the owner holds, such as an intent, a plan
    or a weighing the owner made";
  - `primer@owner-word-premise` (line 58), "A ruling on intent";
  - `primer@intent-and-claims` (line 94), a reversal goes to the owner "where reversing it is a
    fork or the head's ground is the owner's".

**Further sites, found by the second plan review**, each with its target in "Decided design":

- the design skill: loop step 4's routing of an assumption to a criterion (lines 553-556), loop
  step 8's rulings on criteria (lines 650-653), and the bounded path's "unless the owner answers
  otherwise" (lines 194-195);
- the expectation set's "the owner's answer stops or redirects it" (line 177), and the README's
  lines 12, 141 and 173, and its whole restatement of the set (lines 176-215, "You say why when you
  reject something" at 190-194);
- the project-audit skill's outcome table (lines 201-202), "a widening of what the owner's words
  approved";
- head-shape.md lines 79-82, "would widen what the owner's words in it approved";
- `primer@intent-and-claims` line 90, "its argument or the owner's ground undefeated";
- "test 4 included" in the decision-record reviewer (line 106) and the design-record auditor
  (line 116);
- the tripwire standard's six hosts: the design (line 644), planning (365), decision-recording
  (120) and issue-tracking (194) skills, the decision-record reviewer (146) and the tripwire
  auditor (71).

The design skill's lines 372 and 383-384, a weighed criterion's cost "the owner rules on" and
`unmet-and-accepted`, stay: a cost the owner accepts is the owner's choice at a fork, recorded in
its weak form by the table.

**Outside the work:**

- **Re-grounding every existing head that cites the owner's words** is left to the design-record axis
  of `skill@knowledge-architect-project-audit`. Its next pass does it, and the changelog's
  Migration entry sends every consumer to it. This branch re-grounds only the heads the harvest
  rows name.
- **What a proper goal is** is left to the owner's next design session.
- **The checker's plan-document grammar** is unchanged: the `acceptance` item kind and the sections
  stay.
- **No text is added on challenging the owner's proposals.** The instruction exists in loop step 4,
  and one lapse is no recurrence, per `argument@owner-word-and-criteria@arg-47`.
- **Whether agents merge** stays each project's choice, per
  `design@agent-skills@questions-go-to-their-authority`. The thread's shape names "the merge" among
  what stays the owner's, and the work does not carry that clause.

## What is already decided

**The decisions the design rests on:**

- `design@agent-skills@tripwire-admission-test`: its conditions 1, 3 and 4 become the standard a
  criterion meets.
- `design@agent-skills@questions-go-to-their-authority`: what the owner answers is unchanged, but
  for the list of facts the work rewrites; how the answer is recorded changes. Whether agents merge
  stays each project's.
- `design@agent-skills@behaviour-additions-need-real-use`: the evidence is the history of fired
  criteria and the owner's reports in this session.
- `design@agent-skills@default-ruled-before-merge`: a question put to the owner with a built
  default, which the merge waits on, is the exchange of
  `goal@knowledge-architect@design-with-agents-as-counterparts`, per D3.
- `design@agent-skills@goal-entry-shape`: a goal never states a mechanism. So the owner's intent
  that states an outcome lands in a goal, and a preference about a mechanism is argued or is a
  fork's choice, per D5.

**The decisions the work reverses or rewrites.** Each is judged at step 6, and the texts that
`cargo klarch show` lists as citing it are read there:

| decision | what changes | by |
| --- | --- | --- |
| `design@agent-skills@acceptance-criteria-on-the-owners-word` | reversed: no ruling at the premortem | `thread@owner-word-and-criteria@criterion-ruled-at-reopening` |
| `design@agent-skills@acceptance-criterion-quality` | rewritten: conditions 1, 3 and 4 | `thread@owner-word-and-criteria@criteria-test-silent-premises` |
| `design@agent-skills@acceptance-criteria-in-the-document` | its firing response rewritten: a firing opens a discussion with the owner | `thread@owner-word-and-criteria@criterion-ruled-at-reopening` |
| `design@agent-skills@design-hands-off-to-planning`, `design@agent-skills@in-change-path` | no rulings on acceptance criteria | `thread@owner-word-and-criteria@criterion-ruled-at-reopening` |
| `design@agent-skills@ruled-items-labelled` | its `AC` row leaves, and test 4's questions leave its `Q` row | `thread@owner-word-and-criteria@criterion-ruled-at-reopening` |
| `design@agent-skills@head-ground-is-the-argument` | rewritten: the table, and a fork's choice recorded in its weak form | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | test 4 sends an intent to a goal | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@unargued-approval-is-argued` | the tie's choice is a fork's choice, in its weak form | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@extension-follows-the-ground` | a member's route by the ground, under the new ground rule | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@divergence-routed-by-its-authority` | a reversal goes to the owner at a fork; a fork's recorded choice yields to a new argument that ranks the shapes | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@structure-the-flow` | re-grounded on its argument and two goals, no longer on the owner's ruling | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@owner-rules-at-a-fork` | the owner's word closes a fork, and contests any closure by a goal, a fact or an argument; no overrule | `thread@owner-word-and-criteria@intent-through-goals-not-overrule` |
| `design@agent-skills@derived-decision-proceeds-on-default`, `design@agent-skills@tripwire-recording-authority`, `design@agent-skills@tripwire-stands-on-the-standard`, `design@agent-skills@tripwire-audit-axis`, `design@agent-skills@decisions-reported-to-the-owner` | "overrules" becomes "contests by a goal, a fact or an argument" | `thread@owner-word-and-criteria@intent-through-goals-not-overrule` |
| `design@agent-skills@transcript-reviewer-agent`, `design@agent-skills@misstated-ruling-rated-by-what-it-changes`, `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`, `design@agent-skills@one-home-for-head-rules` | a ruling is read by its direction, in every text that judges fidelity to an approval | `thread@owner-word-and-criteria@ruling-read-by-its-direction` |
| `design@agent-skills@questions-go-to-their-authority` | its list "a fact only the owner holds, such as an intent, a plan or a weighing" | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@bounded-path-in-design` | the owner's answer is recorded by what it brings, and a redirect comes with a goal, a fact or an argument | `thread@owner-word-and-criteria@intent-through-goals-not-overrule` |
| `design@agent-skills@audit-outcomes` | a widening of the owner's words is read by its direction | `thread@owner-word-and-criteria@ruling-read-by-its-direction` |
| `design@agent-skills@existing-heads-on-touch`, `design@agent-skills@audit-repairs-the-whole-record` | a change that relaxes or turns the owner's words goes to the owner, where a widening did | `thread@owner-word-and-criteria@ruling-read-by-its-direction` |
| `design@agent-skills@new-or-reshaped-head-needs-design` | an approval of a member is the session's decision | `thread@owner-word-and-criteria@heads-stand-on-argument` |
| `design@agent-skills@answer-keeps-its-authority`, `design@agent-skills@attribution-replaces-quotation` | their premise follows the reworded goal: a choice the owner made is the owner's, a decision the owner approved stands on its argument | `thread@owner-word-and-criteria@heads-stand-on-argument` |

**The texts that cite each decision the work reverses or rewrites**, by `cargo klarch show`, this spec
excluded. Each is read at the step that edits it, or at step 6:

| decision | citing texts |
| --- | --- |
| `design@agent-skills@acceptance-criteria-on-the-owners-word` | `path@agent-skills@content/skills/design/SKILL.md`, `path@agent-skills@content/skills/planning/SKILL.md`, `path@agent-skills@docs/open-issues/a-premortem-criterion-listed-as-a-default-reads-two-ways.md` |
| `design@agent-skills@acceptance-criterion-quality` | `path@agent-skills@content/skills/planning/SKILL.md` |
| `design@agent-skills@acceptance-criteria-in-the-document` | `path@agent-skills@content/skills/planning/SKILL.md` |
| `design@agent-skills@design-hands-off-to-planning` | `path@agent-skills@docs/design/plan-documents.md`, `path@core@docs/design/plan-documents.md` |
| `design@agent-skills@in-change-path` | `path@agent-skills@docs/design/plan-documents.md`, `path@agent-skills@docs/open-issues/a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see.md`, `register@agent-skills@tripwire` |
| `design@agent-skills@ruled-items-labelled` | `path@agent-skills@content/PRIMER.md`, `path@agent-skills@content/skills/design/SKILL.md`, `path@agent-skills@content/skills/planning/SKILL.md`, `path@agent-skills@content/skills/project-audit/SKILL.md`, `path@agent-skills@content/skills/retrospective/SKILL.md`, `path@agent-skills@content/skills/review/SKILL.md`, `path@agent-skills@content/skills/setup/SKILL.md`, `path@agent-skills@docs/design/plan-documents.md`, `path@agent-skills@docs/design/project-configuration.md`, `path@agent-skills@fragments/head-entry-tests.md` |
| `design@agent-skills@head-ground-is-the-argument` | `path@agent-skills@docs/design/owner-and-intent.md`, `path@agent-skills@docs/design/premortem-and-tripwires.md`, `path@agent-skills@docs/open-issues/an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own.md`, `path@core@docs/open-issues/a-kind-name-refusal-quotes-a-future-migration-reason.md` |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | `path@agent-skills@docs/design/design-skill-and-heads.md`, `path@agent-skills@docs/design/how-installed-text-changes.md`, `path@agent-skills@docs/design/owner-and-intent.md`, `path@agent-skills@docs/design/project-configuration.md`, `path@agent-skills@docs/design/shipped-set.md`, `path@agent-skills@docs/open-issues/audit-axes-beyond-the-design-record.md`, `path@agent-skills@docs/open-issues/the-material-finding-duty-has-no-head.md`, `register@agent-skills@tripwire` |
| `design@agent-skills@unargued-approval-is-argued` | `path@agent-skills@docs/open-issues/an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own.md` |
| `design@agent-skills@extension-follows-the-ground` | `path@agent-skills@docs/design/design-skill-and-heads.md`, `path@agent-skills@docs/open-issues/a-member-recorded-directly-lies-beyond-its-head-s-argument.md` |
| `design@agent-skills@divergence-routed-by-its-authority` | `path@agent-skills@docs/open-issues/the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on.md` |
| `design@agent-skills@structure-the-flow` | `path@agent-skills@docs/design/plan-documents.md` |
| `design@agent-skills@owner-rules-at-a-fork` | `path@agent-skills@docs/design/design-skill-and-heads.md`, `path@agent-skills@docs/design/premortem-and-tripwires.md`, `path@agent-skills@docs/open-issues/an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own.md` |
| `design@agent-skills@derived-decision-proceeds-on-default` | `path@agent-skills@docs/design/design-skill-and-heads.md`, `register@agent-skills@tripwire` |
| `design@agent-skills@tripwire-recording-authority` | `path@agent-skills@content/skills/decision-recording/SKILL.md`, `path@agent-skills@docs/design/plan-documents.md`, `path@agent-skills@docs/design/premortem-and-tripwires.md`, `path@agent-skills@docs/design/project-configuration.md` |
| `design@agent-skills@tripwire-stands-on-the-standard` | `path@agent-skills@docs/design/audit-method.md`, `path@agent-skills@docs/design/audit-record-axes.md`, `path@agent-skills@docs/design/project-configuration.md`, `path@agent-skills@fragments/tripwire-lifecycle.md` |
| `design@agent-skills@tripwire-audit-axis` | `path@agent-skills@docs/design/audit-record-axes.md`, `path@agent-skills@docs/open-issues/audit-axes-beyond-the-design-record.md`, `path@core@docs/design/registers.md` |
| `design@agent-skills@decisions-reported-to-the-owner` | `path@agent-skills@docs/design/owner-and-intent.md`, `path@agent-skills@docs/design/premortem-and-tripwires.md`, `path@agent-skills@docs/design/project-configuration.md` |
| `design@agent-skills@transcript-reviewer-agent` | `path@agent-skills@docs/design/owner-and-intent.md` |
| `design@agent-skills@misstated-ruling-rated-by-what-it-changes` | `register@agent-skills@tripwire` |
| `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` | `path@agent-skills@docs/design/reviews.md` |
| `design@agent-skills@one-home-for-head-rules` | `path@agent-skills@docs/design/design-skill-and-heads.md`, `path@agent-skills@docs/design/how-installed-text-changes.md`, `path@agent-skills@docs/design/project-configuration.md`, `path@agent-skills@docs/design/shipped-set.md`, `path@agent-skills@fragments/head-entry-tests.md`, `path@agent-skills@fragments/head-ground.md`, `path@agent-skills@fragments/head-shape.md` |
| `design@agent-skills@questions-go-to-their-authority` | `path@agent-skills@docs/design/audit-method.md`, `path@agent-skills@docs/design/owner-and-intent.md`, `path@agent-skills@docs/design/project-configuration.md`, the issue `a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`, closed at step 2, `path@agent-skills@docs/open-issues/a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component.md`, `path@agent-skills@docs/open-issues/patching-an-installed-skill.md`, `register@agent-skills@rejected`, `register@agent-skills@tripwire` |
| `design@agent-skills@bounded-path-in-design` | `path@agent-skills@docs/design/design-skill-and-heads.md`, `path@agent-skills@docs/design/record-search.md`, `path@agent-skills@docs/open-issues/an-issue-s-re-entry-point-is-read-by-no-review.md`, `path@agent-skills@docs/open-issues/the-open-space-test-may-admit-every-problem.md`, `register@agent-skills@tripwire` |
| `design@agent-skills@audit-outcomes` | `path@agent-skills@docs/design/audit-record-axes.md`, `path@agent-skills@docs/design/audit-workflow-axis.md`, `path@agent-skills@docs/design/how-installed-text-changes.md`, `path@agent-skills@docs/design/retrospective.md`, `path@agent-skills@docs/open-issues/a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component.md`, `path@agent-skills@docs/open-issues/an-upstream-finding-stays-in-every-later-old-text-count.md`, `register@agent-skills@rejected`, `register@knowledge-architect@design` |

`design@agent-skills@expectation-set-bounds-scope` is unchanged: the README restates the expectation
set, as the head says, and step 2 edits that restatement with the set.

Two mentions of the overrule stay as they are:

- `design@agent-skills@goals-bind` (project-configuration.md line 186), "an unreviewed implication
  of an approval overrule a goal", uses the word in another sense.
- The title of `rejected@agent-skills@every-thread-closed-by-argument` describes the rejected design
  as it was proposed.

Tripwires and issues whose clauses depend on a rewritten head:

- `tripwire@agent-skills@ruling-lost-in-change`: its firing clause requires a misstatement
  "critical under" `design@agent-skills@misstated-ruling-rated-by-what-it-changes`. It is reread
  at step 6 against the rewritten head.
- `tripwire@agent-skills@record-answered-questions-reach-the-owner`: its premise cites the same head
  for the owner's observation. It is reread at step 6 against the rewritten head.
- `tripwire@agent-skills@silent-read-too-widely`: the work gives condition 1 a new use, routing a
  check to a criterion or a step claim. At step 6 its firing clause is widened to count a premise
  check routed to a step claim under condition 1, the reverse failure premortem cause 5 names.
- `tripwire@agent-skills@a-shortcut-decision-earns-a-head`: its clause "passes an entry test" is
  reread at step 6 against the three tests that remain.
- `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own`
  closes. Its first case, an approval of a default put as a fork, is answered by D7. Its second case, that
  `skill@knowledge-architect-decision-recording@when-recording-happens` writes a default awaiting
  the owner into a head once the owner rules, stays as it is: the owner's ruling decides when the
  head is written, not what it stands on.
- `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` closes, since no
  criterion awaits the owner.
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` stays
  open. The census of `argument@owner-word-and-criteria@arg-12` is the agent's own reading of the
  deleted plan documents, made in the discussion with the owner, not a run built to produce
  evidence. The decision against `thread@owner-word-and-criteria@criteria-as-they-stand` also rests
  on the firing history, `argument@owner-word-and-criteria@arg-4`, and on the owner's reports.
- `a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`
  closes at step 2, which rewrites the sentence the way its "What would close it" says.

**Each entry the discussion's two searches graded**, with its outcome. The first search, on
acceptance criteria, was seeded with the three heads on criteria and
`design@agent-skills@tripwire-recording-authority`. The second, on the owner's word, was seeded with:

- `design@agent-skills@head-ground-is-the-argument`
- `design@agent-skills@default-ruled-before-merge`
- `design@agent-skills@derived-decision-proceeds-on-default`
- `design@agent-skills@owner-rules-at-a-fork`
- `design@agent-skills@transcript-reviewer-agent`
- `design@agent-skills@misstated-ruling-rated-by-what-it-changes`
- `design@agent-skills@new-or-reshaped-head-needs-design`

An entry graded by both searches is listed once, under its higher grade. One searcher of the
second search graded its cluster on git and the walk from a keyword scan.

**The searchers' reasons name shapes as they stood when each search ran**, and those shapes are
given here under this spec's thread names. The first search:

- S1 is `thread@owner-word-and-criteria@criterion-firing-repairs-first`;
- S2 is `thread@owner-word-and-criteria@criteria-test-silent-premises`;
- S3 is `thread@owner-word-and-criteria@criteria-removed`;
- S4 is `thread@owner-word-and-criteria@criteria-as-they-stand`.

The second search:

- A is `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open` as first drafted,
  which sent a decision met outside a discussion through the bounded path;
- B is `thread@owner-word-and-criteria@every-head-through-discussion`;
- C is the owner's option C, absorbed;
- D is `thread@owner-word-and-criteria@approved-default-stays-the-sessions`, superseded by
  `thread@owner-word-and-criteria@heads-stand-on-argument`;
- E is `thread@owner-word-and-criteria@ruling-read-by-its-direction`.

Its two problems are also named by number:

- problem (1) is the conflict of the counterparts goal with the owner's wish for autonomy,
  `argument@owner-word-and-criteria@arg-17` and `argument@owner-word-and-criteria@arg-18`;
- problem (2) is an approved default attributed to the owner,
  `argument@owner-word-and-criteria@arg-19`.

Graded `bears`, read whole:

| entry | relation | how it bears, as graded | outcome |
| --- | --- | --- | --- |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | constrains | each rewritten or new head (S2's premise-only criteria, its step-claim rule, the premortem routing) must pass an entry test; a part that passes none goes to a %% line | test 4 rewritten at step 3; the head at step 6; each new or rewritten head of the harvest passes an entry test |
| `design@agent-skills@a-reference-claims-a-revisit` | constrains | Shape A rewords `goal@knowledge-architect@design-with-agents-as-counterparts` and D changes how heads are grounded; this head makes every head citing the goal, and every issue straining it, a text to revisit at the reword (show on the goal lists them), and its table's 'a design head names the goal it derives from' governs the heads A and D rewrite. | unchanged: a constraint the steps meet |
| `design@agent-skills@acceptance-criteria-in-the-document` | may change, constrains | seed; defines a criterion as a check only the built code can apply, with "the response", landing reports and the criterion-to-tripwire move at the document's leaving; S1 changes the response, S2 narrows the definition, S3 removes it | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@acceptance-criteria-on-the-owners-word` | may change, constrains | seed; the owner rules "on the decision its firing reopens", which S1 contradicts (a firing is answered by a repair first); S2 narrows what the owner rules on, S3 removes the ruling | reversed at step 6 |
| `design@agent-skills@acceptance-criterion-quality` | may change, constrains | seed; "a criterion bound to pass checks what the design instructs, which condition 3 removes": S2 moves completeness checks out of criteria into step claims, S3 removes the hosted standard's criterion conditions | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@admission-rules-home` | constrains | every edit S1-S3 make to the planning, design and review skills is written and judged under `skill@knowledge-architect-agent-configuration@admission-standard`, which this head makes the one home. | unchanged: a constraint the steps meet |
| `design@agent-skills@answer-keeps-its-authority` | constrains, may change | States 'a decision the owner made is recorded as the owner's and every other decision on its own ground'; shape D adds a case of the same kind (an approved default recorded as the session's decision) and could widen this head or need a sibling. | unchanged: the table's row "a reading of a source" applies it |
| `design@agent-skills@attribution-replaces-quotation` | constrains, may change | an attribution "keeps the scope and the hedges of the words" and is checked by the transcript reviewer; E's lighter reading of the owner's word and D's rule on what is attributed to the owner must respect or change that check. | unchanged: a fork's choice is still attributed, never quoted |
| `design@agent-skills@audit-outcomes` | constrains, may change | it routes "An approval cited as a head's ground is removed in the branch where the head stands on its argument" and sends "A widening of what the owner's words approved" to the owner, citing `goal@knowledge-architect@the-owner-decides`; D widens what counts as an approval rather than a ground, and E's line between a refinement within a ruling's direction and a relaxation must agree with this head's "edit that changes no ruling of the owner" versus "widening". | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@behaviour-additions-need-real-use` | constrains | S1 and S2 add instructions on how an agent answers a fired criterion and routes a premortem cause (step 7); such an addition is admitted only on evidence from real use, which the work offers as this repository's history of fired criteria and the owner's observation. | unchanged: a constraint the steps meet |
| `design@agent-skills@bounded-path-in-design` | may change, constrains | shape A sends a decision that earns a head down the bounded path (one message, rival, default, built), while this head admits work as bounded only when "every decision it makes fails the entry tests"; its "the owner's answer, attributed" is what D reshapes | its clause on the owner's answer rewritten at step 6 |
| `design@agent-skills@calibration-in-the-run` | may change | an instance of problem (2): "The rerun is the owner's choice: 'With a rerun, as you proposed.'" records an approval of the session's proposal as the owner's choice, which D would recast as the session's decision on its argument | unchanged: no step edits an audit workflow |
| `design@agent-skills@capabilities-not-structure` | constrains | the installed text imposes no fixed mapping between a unit of the work and a unit of its record, and a contradiction is answered by removing structure; S2's move of completeness checks into step claims and S3's removal are weighed by it, and S1's repair-first rule against 'a rule for one interaction'. | unchanged: a constraint the steps meet |
| `design@agent-skills@decisions-reported-to-the-owner` | constrains, may change | Cites the seed `design@agent-skills@tripwire-recording-authority` and lists "a premortem cause recorded as a tripwire or dropped" among the Decided items; S2 and S3 reroute premortem causes (to tripwires, claims or nothing) and S1 lets the session answer a firing by a repair, so the list of decisions the Decided section owes may gain or lose a member. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@default-ruled-before-merge` | constrains | a criterion awaiting the owner is listed as a default awaiting the owner (issue `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` names this head as strained); any shape that keeps owner-ruled criteria must state how they meet "merges once the owner has ruled on it" | unchanged; the work rests on it, per D3 |
| `design@agent-skills@derived-decision-proceeds-on-default` | may change | A sends every head-earning decision met outside a discussion through the bounded path's one message with its default, built, which widens this head's "bounded work or settled by the record" scope to decisions that hold a fork. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@design-hands-off-to-planning` | may change | the design skill "ends at convergence, the premortem and the owner's rulings on its acceptance criteria"; S3 removes and S2 narrows those rulings | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@design-home-write-loads-recording` | constrains | the work writes into the agent-skills design home and edits the design, planning and review skill texts whose behaviour the seed heads describe, so the decision-recording skill loads first | unchanged: a constraint the steps meet |
| `design@agent-skills@divergence-routed-by-its-authority` | constrains | S1 and S2 decide what a firing of a criterion routes to: "Where the head stands ... the code is the defect: the session fixes it" and "Where the session finds the head's argument defeated, the head's reversal goes to the design skill" is the routing rule the redesigned firing must agree with or argue against | rewritten at step 6; its primer line at step 3 |
| `design@agent-skills@existing-heads-on-touch` | constrains | the work touches the four seed heads, which must be brought to the rules on heads in that change, with a widening of the owner's words (`design@agent-skills@acceptance-criteria-on-the-owners-word`) going to the owner as one proposal | unchanged: a constraint the steps meet |
| `design@agent-skills@extension-follows-the-ground` | constrains, may change | its argument "the owner approves an argued decision on its argument, so a member the argument covers is within what was approved" is D's premise, and its routing by the head's ground (argument vs owner's words) changes when D moves approved defaults from owner ground to argument | rewritten at step 6; the fragment's member routing at step 3 |
| `design@agent-skills@goal-entry-shape` | constrains | Shape A rewords a goal so that a decision outside a discussion 'goes through the design skill's bounded path, one message with its rival and default, built'; this head says a goal 'never states a mechanism', so the reword must state an outcome and leave the bounded path to a decision. | unchanged; the work rests on it, per D5 |
| `design@agent-skills@goals-bind` | constrains | Problem (1) is a goal (`goal@knowledge-architect@design-with-agents-as-counterparts`) conflicting with recorded decisions; this head says the goal prevails and the conflict goes to the owner, and its rival sentence ('an unreviewed implication of an approval') is the argument shape D extends. | unchanged: a constraint the steps meet |
| `design@agent-skills@goals-change-through-goal-setting` | constrains | Shape A rewords `goal@knowledge-architect@design-with-agents-as-counterparts` and the problem is a decision conflicting with a goal; this head sends both through the goal-setting skill and the owner: 'An agent never edits a goal outside it'. | unchanged: a constraint the steps meet |
| `design@agent-skills@head-ground-is-the-argument` | may change | D narrows "chose where the argued rivals did not settle it" to a choice other than the session's default or with a reason of the owner's own; an approved default becomes the session's decision; the issue on approved defaults names this head. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@in-change-path` | may change | the in-change commit message carries "every premortem cause with its outcome, and the rulings on acceptance criteria"; S2 and S3 change what a premortem cause routes to and whether criteria exist | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@instruction-record-is-minimal` | constrains | S1-S3 change installed text of the planning, design and review skills; this head admits a head for that change only where it keeps the owner's intent or two installed texts consistent (planning criteria vs the spec-conformity review), a rewording's argument staying in the commit. | unchanged: a constraint the steps meet |
| `design@agent-skills@losing-alternatives-filter` | constrains | the shapes that lose among S1-S4, and the current design if reversed, earn a rejected entry only by the recording tests | unchanged: a constraint the steps meet |
| `design@agent-skills@losing-shape-test` | constrains | decides which of the losing drafted shapes (e.g. S3, S4) are candidates for a rejected entry, and that S3 rejected with nothing built would take a head of absence | unchanged: a constraint the steps meet |
| `design@agent-skills@met-outside-the-task-outcomes` | may change | Row 1 sends a fork met outside the task to the owner 'with a default the work proceeds on'; shape A routes a head-earning decision met outside a discussion through the bounded path with a built default, which would change or restate this routing. | unchanged: row 1's routing by authority is the one the work keeps |
| `design@agent-skills@misstated-ruling-rated-by-what-it-changes` | may change | E's problem is that a stricter refinement of an acceptance criterion was rated major; this head rates "any other misstated ruling" major and "wider, narrower or firmer" minor, so E changes what counts as misstated at all and so its rating cut. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@new-or-reshaped-head-needs-design` | constrains | S1-S3 contradict or narrow the seed heads on acceptance criteria; a decision that contradicts a head goes through the design skill, and the rewrite of the seeds is routed by this rule | unchanged: a constraint the steps meet |
| `design@agent-skills@one-decision-per-head` | constrains | S2 bundles three statements (criteria kept for untestable premises; completeness checks become step claims judged by spec-conformity; premortem routing in step 7) that may lose to different nearest rivals and so need separate heads | unchanged: a constraint the steps meet |
| `design@agent-skills@one-home-for-head-rules` | constrains | Shape D rewrites what a head stands on, the fragment head-ground.md this head homes; the fragments also hold 'fidelity to what the owner approved', which shape E's leniency on refinements bears on. | its mention of fidelity to an approval rewritten at step 6; the fragments stay the one home |
| `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` | may change | its last sentence, "A narrowing of an approved head ... still goes to the owner ... and the last transcript review checks it against the owner's rulings", is what D (an approved default is the session's decision, so its narrowing need not go to the owner) and E (only a relaxation is a finding) would change. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@owner-rules-at-a-fork` | constrains | whether each criterion (AC) still goes to the owner, under S2, S3 or the seed `design@agent-skills@acceptance-criteria-on-the-owners-word`, must meet "A fork ... closes on the owner's word" and "A cost alone makes no fork": the same ground on which `design@agent-skills@tripwire-recording-authority` took premortem causes from the owner | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@owner-word-holds-as-far-as-its-premise` | constrains | S2 keeps a criterion only for "a premise of a decision that only the built result can test", whose firing shows the decision wrong; where that decision is the owner's ruling, this head's rule that a ruling whose premise proves false goes back to the owner with the dependent work held governs the firing. | unchanged: the owner still answers an intent, which lands in a goal; the table's fact row applies it |
| `design@agent-skills@plan-document-leaves-at-landing` | may change | its argument for deleting the document after the review names "the standing-state reviewer's reading of the criteria the landing judges" as one of four acts; S2 narrows and S3 removes the criteria that act reads | unchanged in its rule; the planning skill's leaving rules on criteria edited at step 1 |
| `design@agent-skills@plan-read-against-the-record` | may change, constrains | The design-conformance reviewer reports 'an acceptance criterion whose observable the record rules out'. S3 removes that object, and S2 narrows it to premises only the built result can test. The head's argument also rests on two acceptance criteria that passed three plan reviews. | unchanged |
| `design@agent-skills@plan-records-the-exchange` | constrains, may change | records "the owner's rulings verbatim with their round" and says `goal@knowledge-architect@the-owner-decides` "is served only where the rulings are recorded as the owner made them"; shape D changes what is recorded as the owner's ruling (an approved default becomes the session's decision), and shape E governs how the transcript reviewer reads those records | unchanged: a plan document keeps the owner's words verbatim |
| `design@agent-skills@primer-limit` | constrains | Shape D rewrites the head-ground rule the primer hosts in `primer@design-heads`, and shape A may add primer text on decisions met outside a discussion; every primer statement must pass this head's content test (every session needs it, no skill delivers it). | unchanged: a constraint the steps meet |
| `design@agent-skills@questions-go-to-their-authority` | constrains | S1 decides who answers a fired criterion (a repair by the session, or a reopening that may reach the owner); the routing of a question by its authority, and "a cost alone makes no fork", bound which firings go to the owner. | its list rewritten at step 6; the work rests on the rest of it |
| `design@agent-skills@rejected-citation-direction` | constrains | any rejected entry the work adds for a losing shape is cited by the head whose argument covers it, and names no winner | unchanged: a constraint the steps meet |
| `design@agent-skills@ruled-items-labelled` | may change | its table gives the prefix `AC` to "acceptance criteria put to the owner", reaching "a plan document's Premortem section and its defaults"; S3 removes that row, S2 narrows which criteria reach the owner and changes what the Premortem section holds | its `AC` row removed at step 6; the labels at step 1 |
| `design@agent-skills@settled-means-recorded` | constrains, may change | it serves the goal A rewords ("met while no proposal of the owner is built before it has been grounded and challenged"), and states "a ruling of the owner on a question the session put is no proposal", which D's approved-default rule meets | unchanged: the goal's clause on grounding the owner's proposals stays |
| `design@agent-skills@shared-text-fragment` | may change | "The host lists of every fragment but the four of the admission standard are the owner's: the owner approved each list, ... so a host added to one goes to the owner" grounds a decision on the owner's approval, the attribution D would make the session's; D also edits the rules on design heads, one of these fragments. | unchanged: the fragments are edited in place |
| `design@agent-skills@statement-names-an-act` | constrains | the owner's question whether criteria are useful at all is the need test applied to the criterion instructions; each statement S1-S3 add or keep must name an act that differs, and S3's removal leaves by that test. | unchanged: a constraint the steps meet |
| `design@agent-skills@structure-the-flow` | constrains | on the owner's ground it forbids any bound on threads or rounds and says a path "changes where the deliberation is kept, never how large the discussion may grow"; B and C (which decisions earn a discussion) and A's bounded path must respect it | re-grounded at step 6, per D5 |
| `design@agent-skills@synthetic-evidence-not-built` | constrains | its two grounding instances are acceptance criteria (a spec's replay of a search agent, a slice's subagent classification) ruled out, and it refuses a run 'to accept a piece of work'; any criterion S1/S2 keep, and the evidence the discussion builds over the history, must not be such a run. | unchanged: a constraint the steps meet |
| `design@agent-skills@term-defined-at-first-load` | constrains | S2 narrows and S3 removes what 'acceptance criterion' means; the primer uses the term (`primer@labelled-items`) and the item kind `acceptance` (`primer@checker-model`), so this head governs where the redefined term is defined and whether the primer needs an entry. | unchanged: a constraint the steps meet |
| `design@agent-skills@thread-slug-is-entry-id` | constrains | the drafted shapes S1-S4 are threads of the discussion; their slugs become entry slugs and must not collide with the seed heads' slugs | unchanged: a constraint the steps meet |
| `design@agent-skills@title-states-the-rule` | constrains | any rewritten seed head (e.g. criteria kept only for a premise only the built result can test, S2) must title the rule it argues, not the instance, and no wider than its argument | unchanged: a constraint the steps meet |
| `design@agent-skills@transcript-reviewer-agent` | constrains, may change | E rewrites its second check, "no ruling of the owner is recorded in a state or a scope the owner did not give", to report only a reversal, a relaxed check/claim/criterion, or a decision wrongly attributed to the owner; its existing "Detail added inside a ruling ... are not findings" is the base E widens to stricter refinements. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@tripwire-admission-test` | constrains | S3 makes premortem causes tripwires and S2 routes a cause to a criterion only when its firing shows the decision wrong: condition 1 ("25 were found and acted on anyway, through ... a review doing its own job") and condition 3 decide which former criteria may become tripwires, and its measurement is the model for counting fired criteria answered by a repair | unchanged: a constraint the steps meet |
| `design@agent-skills@tripwire-recording-authority` | may change | seed; S2 changes the premortem's routing (design skill loop step 7) so a cause goes to a criterion only when its firing shows the decision wrong, and S3 sends causes now routed to criteria to tripwires, claims or nothing, which widens what this head's harvest judges | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@tripwire-stands-on-the-standard` | constrains | cites seed `design@agent-skills@tripwire-recording-authority`; its argument "the owner's word alone admits no tripwire and keeps none" and "its only value is the fact it delivers" is the rival argument against seed `design@agent-skills@acceptance-criteria-on-the-owners-word`, and S3's causes made tripwires stand on it | rewritten at step 6, per the table of decisions the work rewrites |
| `design@agent-skills@unargued-approval-is-argued` | may change | "Where the rivals come out equal, the fork goes to the owner as a tie, and the owner's choice is the ground": D makes the owner's choice of the session's default no ground; named by the approved-default issue. | rewritten at step 6, per the table of decisions the work rewrites |
| `design@core@anchors-are-components-and-locations` | may change | it states that a spec and a milestone each carry 'the four item registers' (thread, argument, criterion, acceptance); S3 removes acceptance criteria and with them, possibly, the `acceptance` item register, which would change this sentence. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@items-as-section-registers` | may change | It defines 'The four item registers' of shape `Section`. S3 removes the `acceptance` register, leaving three. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@milestone-entries-are-directories` | may change | It says a milestone carries 'the four item registers'. S3 makes them three. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@one-entity-table` | may change | it lists the built-in kinds "`criterion` and `acceptance`"; S3 removing acceptance criteria would remove the `acceptance` item kind (per `design@core@plan-items-by-section`, outside this cluster) | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@plan-document-sections-by-kind` | may change | It lists the owed plan sections 'Premortem, Acceptance criteria, Implementation sequence', and a slice spec's 'Claims'. S3 removes Acceptance criteria. S2 moves completeness checks into claims of the steps, and the head says 'a change to them changes this head'. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@plan-items-by-section` | may change | It defines the section 'Acceptance criteria' and the item kind `acceptance`. S3 removes them, and S2 may rename or narrow the section, so the checker's item kinds change. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@plan-register` | may change | It states that a plan's 'threads, arguments, criteria and acceptance criteria are items'. S3 removes acceptance criteria, and that sentence changes with it. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@core@registers-are-declared` | may change | It lists "the four item registers, carried by the plan anchors" among the eleven compiled-in registers, `criterion` and `acceptance` among them per the primer; S3 removes acceptance criteria and S2 narrows them, which could remove or reshape a compiled-in item register this head enumerates. | unchanged: the checker's grammar and the `acceptance` item kind stay |
| `design@knowledge-architect@capability-cost-before-migration` | constrains | S3 argues removing acceptance criteria; its cost is first what a consumer could no longer do (record a built-result test of a decision's premise in a plan), the migration of existing plan documents ranking lowest | unchanged: a constraint the steps meet |
| `design@knowledge-architect@changelog-entries` | constrains | a change to the installed skills that a person watching sessions would observe (a criterion no longer written, a fired criterion answered by a repair, no owner question on firing) is a Workflow entry; a removed plan item kind is a Migration entry for consuming projects' plan documents | unchanged: a constraint the steps meet |
| `design@knowledge-architect@versioning-policy` | constrains | S2/S3 change the installed skills (patch) and S3 may remove the `criterion`/`acceptance` plan item kinds the checker reads, a change to a document grammar or a looser check; the table classes the bump | unchanged: a constraint the steps meet |
| `goal@agent-skills@no-contradiction-in-workflow` | constrains | The goal asks that "a review judges a text by the criteria the skill of its activity asks the writer to meet"; S2 moves a completeness check into a step's claim judged by the spec-conformity review, and S1/S3 change what the design and planning skills ask, which the review skill must then match. | unchanged: a constraint the steps meet |
| `goal@agent-skills@only-needed-instructions` | constrains | The owner asks whether acceptance criteria are a distraction; the goal answers an instruction or structure the work does not need by removing it, which S2 and S3 serve and S4 must argue against. | unchanged: a constraint the steps meet |
| `goal@knowledge-architect@agents-get-a-complete-workflow` | constrains | The work changes the design, planning and review parts of the workflow; the goal asks for fewer review rounds and fewer decisions reversed by accident, which a firing handled as a reopening rather than a repair works against. | unchanged: a constraint the steps meet |
| `goal@knowledge-architect@agents-work-without-drift` | constrains | The goal is met while "a departure from the recorded design is caught by a check or a review before it merges"; acceptance criteria are one such check, so S2/S3 narrowing or removing them must keep that catch through the reviews. | unchanged: a constraint the steps meet |
| `goal@knowledge-architect@design-is-recorded-with-its-arguments` | constrains | the reversal or narrowing of the seed heads is recorded with its argument and the shapes that lost (S1-S4) | unchanged: a constraint the steps meet |
| `goal@knowledge-architect@design-with-agents-as-counterparts` | may change, constrains | problem (1)'s goal; A rewords its clause "no decision that earns a head reaches the main branch before the owner has met it in that exchange", B keeps it by full discussion | reworded and written in the branch's first commit |
| `goal@knowledge-architect@structure-and-workflow-work-together` | - | the plan documents carry `criterion` and `acceptance` item kinds the checker knows (`primer@checker-model`); S2/S3 could narrow or remove the use of them, and the goal says neither structure nor workflow is designed without the other; whether a checker change follows is not established | unchanged: a constraint the steps meet |
| `goal@knowledge-architect@the-owner-decides` | constrains | S1 changes whether a fired criterion reopens a decision, which may reach the owner, or is repaired by the session; the goal reserves to the owner only what the goals and the record leave open and keeps the owner informed of the rest. | reworded and written in the branch's first commit |
| `goal@knowledge-architect@the-owner-s-word-is-verified` | constrains | Shape E sets how strictly a reviewer reads the owner's word against the record; this goal binds how the owner's word is taken and checked in every activity. | unchanged: a constraint the steps meet |
| `goal@knowledge-architect@the-workflow-improves-through-real-use` | - | The goal forbids a mock task built "to accept a piece of work"; S2 keeps criteria for premises only the built result can test, which for an agent-behaviour premise could ask for such a run, but the drafted shapes say nothing of how a criterion is tested, so no bearing is established. | unchanged: a constraint the steps meet |
| `a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise` | touches | Who may lift a criterion derived from a decision, the owner or the session: the work's question of owner's say versus agent autonomy, and the joined discussion on acceptance criteria, decide the same thing. | closed at step 2 |
| `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` | touches | it reopens `design@agent-skills@extension-follows-the-ground`, whose premise (owner approves on the argument) D rests on, and A changes how a decision met outside a discussion is routed, the act this issue records going wrong | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` | touches, closes | cites seed `design@agent-skills@acceptance-criteria-on-the-owners-word`; its subject is how a premortem criterion awaiting the owner is judged; S3 would close it and S2 changes which criteria reach the owner | closed at step 6 |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | touches | S3 turns premortem causes into tripwires, and S2 changes the premortem's routing (loop step 7); this issue records premortem causes of agent behaviour whose evidence lands only in sessions, and how they were dropped or rerouted | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own` | closes, touches | the work is the discussion this issue's "What would close it" asks for; D answers who an approved default is attributed to, and A the autonomy on defaults. | closed at step 6 |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | - | D and seed design@agent-skills@head-ground-is-the-argument decide when an approval is the owner's ground; this issue records an audit axis 'a head citing an approval of the owner as its ground' and an approved threshold written as 'the session's proposal, presumed approved', which D's rule would govern; no relation to the issue's open axes is established. | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s` | touches, depends on | It records AC4, the owner-proposed convergence criterion of the work that made `design@agent-skills@root-cause-repair`, the very count the work cites, and the owner's ruling that it is "a recurring acceptance criterion of the work that creates or changes an audit" rather than a tripwire; S2/S3 change what such a recurring criterion is and where it lives, so the issue's framing and close depend on the answer. | stays open: AC4's kind is the one the standard keeps, so the issue's framing holds |
| `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them` | touches, trigger met | one of its instances is "a batch-approved default attributed to the owner in a head", problem 2 of the work; shape D changes attribution of approved defaults and E the transcript reviewer that catches these slips; its re-entry is a design discussion on how the owner's words are kept | stays open: its trigger, work that changes how a plan document keeps the owner's words, is not met; the work changes how an attribution is judged |
| `issue@agent-skills@the-material-finding-duty-has-no-head` | touches | The design skill's material finding is one that defeats "a criterion it claimed to meet" and comes back to the owner once; S1 and S2 decide when a fired acceptance criterion reopens its decision, which is that duty's case, and a head recording either would meet this issue. | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@the-open-space-test-may-admit-every-problem` | touches | C (which decisions earn a discussion) and A (more decisions on the bounded path) change the classification this issue questions, the bounded path's "no second defensible shape" condition | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default` | touches | A and the reworded `design@agent-skills@default-ruled-before-merge` change how a default is built and put to the owner, the routing whose hold this issue questions; the owner's ruling recorded there (a question with no sensible default holds its part) constrains A. | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on` | touches | the approved-default issue records that its case of a built default narrowing a head answers this issue's question in real use; A/D's rule on built defaults that change a head touches which work goes on. | stays open: step 3 edits one clause of the paragraph, not the two readings the issue records |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | touches | the work's evidence (most fired criteria answered by repairs found by the review axes) and the owner's question whether criteria cost more than they catch are review-cost claims this issue says have no counts; a cut of workflow text is argued from intuition without them | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` | touches | its subject is the workflow's model of the owner's authority, in goal@knowledge-architect@the-owner-decides and `primer@question-authority`, and how a record says who ruled; the work rewords the owner's goals (A) and the rule of attributing a decision to the owner (D), which a decision on who rules inherits. | stays open: the work neither meets its closing condition nor changes its subject |
| `issue@agent-skills@two-texts-define-a-term-in-their-own-words` | touches | the design skill defines "the owner" as "the person whose word closes the questions" its `skill@knowledge-architect-design@decision-authority` section gives the owner; shapes A and D edit that section's allocation, the subject of the two diverging definitions | stays open: step 2 keeps the design skill's definition of the owner |
| `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` | touches | its question, whether the head stands on the owner's quoted ruling or on its own argument, is read through seed design@agent-skills@head-ground-is-the-argument, whose rule on when the owner's words are a ground shape D rewrites (the owner chose other than the agent's proposal here) | read at step 6 against the rewritten head on the ground |
| `rejected@agent-skills@delegated-decision-state` | may bring back | The owner's wish for more autonomy on design and defaults could propose a grant to agents within a boundary; this entry rejected that, and says 'approving every default on a quick read' is an approval like any other, the case shape D treats. | not brought back: an approval lands by the rule of `primer@question-authority`, no grant within a boundary |
| `rejected@agent-skills@every-decision-through-design-skill` | may bring back | B, every decision that earns a head through a full discussion, is a narrower form of this rejected design ("every decision that earns durable text goes through the design skill") | not brought back: B is ruled out |
| `rejected@agent-skills@every-thread-closed-by-argument` | may bring back | A lets a decision met outside a discussion close on the session's default with the owner only answering if they wish, and D records approved defaults as the session's; extended to forks, that is "every design thread closing on its argument, the owner's word only overruling". | not brought back: a fork still closes on the owner's word; its title stays |
| `rejected@agent-skills@owner-rules-each-tripwire` | may bring back | S3 sends causes now ruled by the owner as criteria (AC) to tripwires; keeping the owner's ruling on each converted cause, or on each criterion S2 keeps, brings back "the owner ruled on each cause put under a label", against the measured cost recorded here | not brought back: no cause or criterion is ruled at the premortem |

Graded `may-bear`, not read, with the searcher's reason:

| entry | relation | the searcher's reason | outcome |
| --- | --- | --- | --- |
| `design@agent-skills@audit-axis-runs-a-workflow` | - | D; the head says "The delivery as a file of the harness's directory is the owner's too", while `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` records that the session proposed the delivery and "The owner approved the delivery on that statement": under D that part would stand on its argument as the session's decision; not established whether the owner chose other than the session's default. | not read |
| `design@agent-skills@audit-method` | - | D changes which decisions are attributed to the owner; the head's "A quotation or an attribution of the owner is confirmed against the question it answered" is the audit's check of attributions, which would then judge by D's rule; no conflict established, the sentence states no rule of attribution. | not read |
| `design@agent-skills@audit-passes-repeat-while-the-count-falls` | - | The work cites the agentic-workflow axis's per-pass count as the one fired criterion that led to a redesign; this head's stop rule ("a pass where that count does not fall goes to the owner as a question") is the first half of AC4 named in the issue on severe counts; S2/S3 could re-home that check, but no shape is shown to change the head. | not read |
| `design@agent-skills@code-change-reads-its-record` | - | Cites seed `design@agent-skills@new-or-reshaped-head-needs-design`: it says a fix contradicting a head is 'sent to the design skill by the backstop'; shape A routes such a decision through the design skill's bounded path, which still lies inside the design skill, so no contradiction is established. | not read |
| `design@agent-skills@content-is-in-the-walk` | - | "the extension is the session's argument, which the owner's choice of content/ in the walk followed" is an attribution of a choice following the session's argument, the case D rules on; no change to the head established. | not read |
| `design@agent-skills@convergence-count-scope` | - | This head is the redesign the work names as the outcome of the fired count criterion (plain count replaced by the old-text count); the work uses it as evidence that a firing can show a decision wrong, which S2 keeps, and changes no sentence of it. | not read |
| `design@agent-skills@design-home-is-built-intent` | - | a criterion is judged on the built result while its decision is still in the plan document; the sentence "each decision moves into the design home at the landing of the work that builds it" bears on whether a firing before landing reopens a decided-but-unbuilt decision (S1, S2), but no shape changes where the decision lives | not read |
| `design@agent-skills@design-record-axis` | - | the auditor reads "the history behind each citation of the owner" in every head; D changes which approvals may be cited as the owner's ground, so the rule the axis applies shifts; the axis's own shape is not changed | not read |
| `design@agent-skills@expectation-set-bounds-scope` | - | the work edits the design skill (step 7), whose expectation set is the scope test of every proposed edit; the owner's observation concerns how firings are handled, not the owner's behaviour, so no bearing is established. | read at the plan review: the README restates the expectation set, which step 2 edits |
| `design@agent-skills@goal-lifecycle` | - | Shape A rewords `goal@knowledge-architect@design-with-agents-as-counterparts`; this head's 'A goal leaves only when the owner abandons it' could be met if the reword drops the goal's present claim (the owner meets every head-earning decision) rather than rewording it, which is not established. | not read |
| `design@agent-skills@head-prevails-over-diverging-code` | - | S1's rule, a fired criterion answered by a repair of the work rather than by reopening the decision, parallels "the divergence closes when the code meets the head or when the head is reversed"; whether a firing is such a divergence between a decision and the built work is not established, since the decision is still in a plan document | not read |
| `design@agent-skills@ledger-from-transcript` | - | shape E changes what the transcript reviewer reports; this head gives that reviewer the check of "every assembled document against the same files"; E narrows the findings, not the check this head assigns, so no bearing is established | not read |
| `design@agent-skills@milestone-is-a-directory` | - | cites the seed `design@agent-skills@default-ruled-before-merge`: "The README leaves once its last slice has landed and every default it lists is ruled"; shapes A and D change what a default put to the owner is and whether an approved default stays the owner's ruling, which could change when a default counts as ruled; no change to the leaving rule itself is proposed | not read |
| `design@agent-skills@one-run-pass` | - | D makes an approved default the session's decision; the head grounds "The cross-check and the report are one agent each, two in all, on the owner's word" and the session's choice of samples "on the owner's word"; whether these were the owner's own choice or an approved default of the session is not shown. | not read |
| `design@agent-skills@outcome-over-display` | - | the owner asks whether criteria are a distraction; the entry says a finding is worth an instruction only if it corrupts the outcome (what gets recorded or built); a criterion's firing handled as reopening rather than repair may be such an outcome corruption, or removing the instruction may follow from it; no bearing on the shapes' text established | not read |
| `design@agent-skills@plain-text-is-no-repair` | - | acceptance criteria are plan items, and the head admits a plan item named outside its plan as #<id> beside a citation of the plan; S3 removes the `acceptance` item kind, which the head's sentence on plan items does not depend on | not read |
| `design@agent-skills@record-searched-before-the-work` | - | S2 changes the premortem's routing (design skill loop step 7), and S3 sends every premortem cause to a tripwire, a claim or nothing. This head says a premortem cause whose firing condition the planned code meets 'enters a tripwires home only at the harvest, if the standard admits it', and the slice audit lists it. That sentence may need rewording if routing changes; no conflict is established. | not read |
| `design@agent-skills@repairs-read-against-the-record` | - | A sends a head-earning decision met outside a discussion through the bounded path, built, with the owner answering after; this head says "A repair that reverses a head is a decision, recorded under decision-recording" and is read "before the owner rules on it", arguing from `goal@knowledge-architect@the-owner-decides` that the owner must know before ruling; whether A's no-stop path applies to audit repairs that reverse a head is not established | not read |
| `design@agent-skills@review-repair-appended-or-folded` | - | S1 answers a fired criterion by a repair of the work; the entry rules how a review repair lands (appended, folded only where needed); no bearing established unless the repair is made as a review repair | not read |
| `design@agent-skills@roadmap-order-is-the-owners` | - | argues "ordering known work is a fork ... so the order is the owner's, per the-owner-decides"; shape D redefines when the owner's choice at a fork is the owner's ground (only where the owner chose other than the default or gave a reason), which could reach a roadmap row added on the owner's approval of a session's proposed order; no bearing established since this head is about who rules, not how a ruling is attributed | not read |
| `design@agent-skills@root-cause-repair` | - | the work's one firing that led to a redesign (the count per pass of the agentic-workflow axis) is this head's argument: 'its third pass found about 25 findings ... the count did not fall, the repairs move to a method by cause'; S2 keeps criteria only for such a premise and S3 turns them into tripwires or claims, which could change how the evidence in this head is described; no change to the head's decision is established | not read |
| `design@agent-skills@standing-argument-in-head` | - | S2 keeps a criterion only "for a premise of a decision that only the built result can test"; the head defines the premises as "every premise whose failure would reopen it ... the measurement it rests on, with the command that takes it again", so S2's criterion may become part of the standing argument, but no relation is established | not read |
| `design@agent-skills@standing-state-review-before-every-merge` | - | S3 turns premortem causes into tripwires, and S2 sends fewer causes to acceptance criteria; this head makes the standing-state reviewer read every tripwire before every merge, so more tripwires add to that reading. Whether that load changes the head is not established. | not read |
| `design@agent-skills@step-is-no-commit` | - | S2 makes a completeness check "a claim of the step that builds it"; the entry fixes that a step is no unit of the history, so a claim judged per step could be read as judged per commit; no bearing established, S2 names no commit | not read |
| `design@agent-skills@tripwire-horizon` | - | under S3 a cause now a criterion becomes a tripwire carrying "**Watches until:**"; a criterion judged at landing has an implicit horizon, which this head's exits may need to name, not established | not read |
| `design@agent-skills@unrecognised-entries-reach-upstream` | - | (D) makes an owner's choice at a fork the owner's ground only where the owner chose other than the session's default or gave a reason; this head says "The route is the owner's choice between two shapes neither of which failed a constraint"; whether that choice was the session's default is not shown here, so whether (D) would re-attribute this head is not established | not read |
| `design@agent-skills@verification-in-primer` | - | S2 turns a completeness check into "a claim of the step that builds it"; the primer's rule that "a checkable claim is checked before it is written" may be what such a claim relies on, but the work's "claim" is a plan step's claim judged by the spec-conformity review, so no bearing is established. | not read |
| `design@agent-skills@verifier-in-the-run` | - | D and E; the verifier "checks ... a quotation or an attribution of the owner against the question it answered" and marks a finding that a ruling's premise proved false; a changed attribution rule changes what it checks against, with no conflict in the sentence itself. | not read |
| `design@agent-skills@workflow-axis-lenses` | - | E rewrites the transcript reviewer's standard on what a misstated ruling is, and D the rule on owner attributions; the head's L8 "reads each pair of a review standard and an audit standard that judge one subject" for divergent verdicts, and the design-record and tripwire axes judge owner attributions; no change to the lens is drafted, so the doubt is only that E/D create a pair L8 would report | not read |
| `design@core@declarable-directory-register` | - | D; "One register, on the owner's ruling", followed by the argument; if the ruling approved the session's default, D would ground it on that argument instead; not shown. | not read |
| `design@core@harness-entity-names` | - | "The match is the owner's ruling, against the frontmatter name alone" with the owner's reason; under shape D it stays the owner's ground only if it departed from the session's default or carries the owner's own reason, which it seems to ("I'd rather make the checks more strict"); no change established | not read |
| `design@core@home-at-the-carrier-path` | - | D; "The owner chose to keep this key beside that attachment rather than replace it", which `rejected@core@carrier-dir-replaced-by-components` says was "made on the session's recommendation": the case D would record as the session's decision on its argument. | not read |
| `design@core@init-runs-git-init` | - | D; the departure "chosen by the owner over following `cargo init` exactly" is a fork choice attributed to the owner; under D it is the owner's ground only if the owner chose other than the session's default or gave a reason; the body gives the `git clean` argument, its origin not shown. | not read |
| `design@core@init-writes-no-ci-workflow` | - | D; "on the owner's word" grounds the exclusion of the CI snippet; whether that word approved a session's default is not shown. | not read |
| `design@core@item-registers-built-in` | - | S3 removes the built-in `acceptance` item register. The head says 'the item registers are the tool's'. That sentence stays true with three registers, but removing a built-in register changes the document grammar consumers rely on, an interface. | not read |
| `design@core@kind-names-refused-as-anchors` | - | it quotes the owner's ruling as its ground; the issue citing seed `design@agent-skills@head-ground-is-the-argument` asks which ground it stands on, and shape D's test (the owner chose other than the session's default) would answer that; the owner ruled against the agent's proposed issue entry, so D seems to keep the owner's ground; not established | not read |
| `design@core@orphan-rejected-entry` | - | D; "Only a citation from a design entry counts, on the owner's judgement" attributes a choice to the owner; whether it was the session's default approved is not shown. | not read |
| `design@core@root-instructions-alone-modelled` | - | its ground is "as the owner chose", with the owner's words; shape D makes an owner's choice a ground only where the owner chose other than the session's default or gave a reason of their own, and whether this choice was the session's default is not shown in the entry | not read |
| `design@core@tripwire-horizon-unchecked` | - | D; "The owner chose to keep the line out of the checker for now, judging a check not yet needed" is a fork choice attributed to the owner; D keeps it the owner's only where the owner chose other than the session's default or gave a reason of their own; the stated judgement may be such a reason, not established. | not read |
| `design@knowledge-architect@committed-findings-analysis` | - | Its 'Each finding gets one of three outcomes, ruled by the owner' and 'on the owner's word' for a rewritten outcome are owner rulings on session-proposed outcomes; D (an approved default is the session's decision) and A's autonomy could reach how such a ruling is attributed, but the work names only decisions that earn a head, so no bearing is established. | not read |
| `design@knowledge-architect@no-future-breaking-cost-argument` | - | S4 (keep the current design) could be argued on the cost of changing the plan grammar later; this head states such a cost 'argues against nothing'; no such argument is in the drafted shapes yet | not read |
| `goal@agent-skills@one-skill-per-activity` | - | S2 moves a check between planning (criteria), the design skill (step 7) and the review (spec-conformity); the goal's 'reading another skill only at a step that points to it' could be met by the new routing, with no conflict established. | not read |
| `goal@knowledge-architect@any-project-can-adopt-it` | - | S3 removes the acceptance item kinds that adopting projects' plan documents hold; the goal's 'A project pins the version it uses, and moves to a new one when it chooses' bears on the migration; no conflict established since a plan document leaves at landing | not read |
| `goal@knowledge-architect@documentation-stays-consistent` | - | S2 moves completeness checks to the spec-conformity review; the goal says "The reviews of the workflow keep it consistent", which concerns the record and the code rather than plan acceptance, so no bearing is established. | not read |
| `issue@agent-skills@a-fix-is-grounded-by-no-record-search` | - | Shape A sends a head-earning decision met outside a discussion, such as in a fix, through the design skill's bounded path; this issue's question is whether a fix gets the design skill's grounding, and its trigger is a goal on development work, which the work does not state; no bearing established. | not read |
| `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component` | - | shape A re-draws which decisions go to the owner and which the session settles; this issue argues an audit sends the owner a question the record answers, against goal@knowledge-architect@agents-work-without-drift; the work's rewording of the goals could settle its routing, but the issue's subject is one audit outcome, which the work does not name. | not read |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | - | an assumption about how the owner answers a session's default (shape A's "which the owner may answer") is the kind of item an expectation set of the design or planning skill would state with the owner; no bearing established on the drafted shapes | not read |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | - | S2 recasts a completeness check as a claim of the step judged by the spec-conformity review; the entry states 'The acceptance criterion of the work that shipped the retrospective intake searched by hand for all three' kinds of reference, an instance of such a check. It records the instance only, so no bearing is established | not read |
| `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head` | - | if the `skill@knowledge-architect-planning@acceptance-criteria` seed heads earned their place as a document grammar (plan item kind `acceptance`, entry test 1), every losing shape (S1-S4) would pass test 1 of the filter, which this issue says reads two ways; whether the seeds stand on test 1 is not established | not read |
| `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` | - | if the discussion classifies this repository's fired criteria by subagents to choose between S1-S4, that census is the case this question leaves open ('to choose between shapes'); no such census is named in the work. | not read |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing` | - | its first instance is a wrong slug written as an acceptance criterion's name in a harvest message; S3 removes acceptance items and S2 narrows them, which changes how often such a name is written but establishes no change to the issue's subject, the unchecked plain #<id> | not read |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | - | S2 changes the premortem's routing in 'design skill loop step 7'; the entry records that 'loop step' numbers across the design skill cite `skill@knowledge-architect-design@discussion-loop` in words, unchecked. Only a renumbering or a new citation of that step would touch it, which no shape states | not read |
| `issue@core@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history` | - | the issue counts 'the next criterion id' among the numbers a plan must not reuse; S3 removing acceptance criteria would remove that prefix from what its command lists; the issue's closure is not changed otherwise | not read |
| `rejected@agent-skills@head-cites-every-owner-ruling` | - | D moves away from it; the doubt is a rival of D that keeps the current fork rule, where an approval at a fork is cited as the owner's ground, a partial form of this rejected design; not drafted. | read at the second plan review: not brought back, per "Decided design" |
| `rejected@agent-skills@member-routed-by-approval-scope` | - | D changes what an owner's approval grounds; this rejected design routes a member by the scope of the owner's approval; D does not propose routing by approval scope, so no bringing back established | not read |
| `rejected@agent-skills@owner-words-quoted-in-head` | - | its doubt "this is the candidate if a misstated attribution recurs" bears on E's lighter transcript reading, which could let misstated attributions pass; E keeps "attributes to the owner a decision the owner did not make" as a finding, so no bearing established. | not read |
| `rejected@agent-skills@plain-confirmed-findings-count` | - | It records that "each firing of the plain count, at passes 1 to 3 and at 5, was explained by the same churn, argued by hand", which is the evidence the work cites for a firing that led to a redesign; the work proposes no return of the plain count. | not read |
| `rejected@agent-skills@plan-document-deleted-with-its-harvest` | - | one of its four reasons is "the standing-state reviewer's criteria"; S3 removes that reason, but the three others stand, so no return of the design is established | not read |
| `rejected@agent-skills@separate-bounded-problem-skill` | - | kept on the doubt "whether the path lets a decision that earns a head skip its discussion"; A sends such decisions down the bounded path, which feeds that doubt, but A proposes no separate skill | not read |
| `rejected@agent-skills@transcript-reviewer-reports-every-clause` | - | cited by seed `design@agent-skills@transcript-reviewer-agent`; E moves further from this design (fewer reports), so it does not bring it back, but E's rewrite of the standard must keep it rejected and its "detail or a better wording inside a ruling needs no report" is E's precedent. | not read |
| `rejected@agent-skills@trigger-judges-settled-design` | - | C, a criterion to tell which decisions earn a discussion, could be written as a session's judgement before the skill load, which this entry rejected; if C runs after the grounding, no bearing | not read |
| `rejected@core@carrier-dir-replaced-by-components` | - | D; "It loses on the owner's choice to keep both, made on the session's recommendation" is an approved default recorded as the owner's choice, the attribution D would replace with the session's argument; the entry's design is not brought back by the work. | not read |
| `rejected@core@entry-reachable-by-citing-a-goal` | - | D; "The owner judged that only a design entry should count" attributes the deciding judgement to the owner; whether it was the session's default is not shown. | not read |
| `rejected@core@init-git-init-anywhere-outside-a-worktree` | - | D; "The owner chose a refusal there over following `cargo init`" is the same fork-choice attribution as `design@core@init-runs-git-init`; not established whether it was the session's default. | not read |
| `tripwire@agent-skills@a-shortcut-decision-earns-a-head` | - | it guards the premise that bounded work holds no decision passing an entry test; shape A sends head-earning decisions down the bounded path by design, which removes that premise and does what its response asks (reopen `design@agent-skills@bounded-path-in-design` on its three conditions); its firing clause, a review finding such a decision in two pieces of work, is met by no element of the work itself, so no firing is established | read at the second plan review: reread at step 6 |
| `tripwire@agent-skills@agent-text-written-without-the-standard` | - | the work edits agent-facing text (planning, design, review skills); the tripwire fires on an unneeded addition made in a session that did not load the agent-configuration skill, which this work would meet only by skipping that skill. | not read |
| `tripwire@agent-skills@coined-term-without-an-entry` | - | S2 may coin or redefine a term the primer uses (an acceptance criterion as a decision's premise test, a step's claim); the tripwire fires when such a term the work adds to the primer has no `primer@coined-terms` entry; no such addition is drafted. | not read |
| `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | - | Shape A moves decisions met during other work into the design skill's bounded path; the tripwire's premise is that "a deferred trigger met by work that bypasses the skill costs little", so A changes which work bypasses the skill; its firing clause, a second recorded instance found at the standing-state review, is not met by the change itself. | not read |
| `tripwire@agent-skills@owner-proposal-built-ungrounded` | - | guards `design@agent-skills@settled-means-recorded`, which serves the goal clause A rewords; A as drafted keeps "no proposal of the owner built before grounded", so no firing or change established | not read |
| `tripwire@agent-skills@record-answered-questions-reach-the-owner` | - | its decision, `design@agent-skills@questions-go-to-their-authority`, governs whether criteria go to the owner; the firing clause needs two retrospectives finding a question the record answered put to the owner, which the redesign does not itself meet, so no firing is established | read at the plan review: reread at step 6 against the rewritten head |
| `tripwire@agent-skills@ruling-lost-in-change` | - | cites the seed `design@agent-skills@misstated-ruling-rated-by-what-it-changes`; its firing clause requires a misstatement "critical under" that seed, which shape E narrows (a refinement within the ruling's direction is no finding), and it guards `design@agent-skills@in-change-path`, which shape A widens to decisions met outside a discussion; the work changes what the clause depends on but would not itself find a ruling lost or misstated in an in-change commit message, so the clause is not met | read at the plan review: reread at step 6 against the rewritten head |
| `tripwire@agent-skills@silent-read-too-widely` | - | S3 drops or converts causes now criteria; its firing clause, a silent failure whose cause a commit or plan lists as dropped under condition 1, could be met by a cause dropped in the conversion, but the work itself drops no cause yet | read at the second plan review: its clause widened at step 6 |
| `tripwire@agent-skills@split-rule-from-exception` | - | S1 is a rule with an exception (a firing is repaired; it reopens the decision only where no repair meets both); the tripwire fires on an exception split from its rule in a head a branch writes; no split is drafted, so no firing is established | not read |
| `tripwire@core@init-cleanup-beyond-the-listed-files` | - | D; its premise names "the refusal of a directory holding files outside every worktree that the owner chose over following `cargo init` exactly", an attribution D could re-ground; its firing clause is not met by the work. | not read |

## Criteria

The transcript gives these criteria no ids. The ids below are minted at assembly. The three
criteria were listed once, in the round 5 reply (L2420), and covered only the `skill@knowledge-architect-planning@acceptance-criteria`
threads. The later checkpoints (round 7 reply L2758, round 10 reply L2808) list no criteria. The threads of
rounds 6 to 10 were argued from the goal `goal@knowledge-architect@design-with-agents-as-counterparts`
(`argument@owner-word-and-criteria@arg-17`, `argument@owner-word-and-criteria@arg-22`,
`argument@owner-word-and-criteria@arg-32`), which no criterion lists; it binds them as a goal.

### Only needed instructions `##only-needed-instructions`

- **Kind:** binding.
- **Source:** the goal `goal@agent-skills@only-needed-instructions`.
- **Satisfaction (L2420):** "#criteria-test-silent-premises: about 49 of 61 past criteria duplicated a test or a review".

### Departures are caught before the merge `##departures-caught-before-merge`

- **Kind:** binding.
- **Source:** the goal `goal@knowledge-architect@agents-work-without-drift`.
- **Satisfaction (L2420):** "completeness checks become step claims, judged by the spec-conformity review".

### The owner rules on what is the owner's `##owner-rules-on-own`

- **Kind:** binding.
- **Source:** the goal `goal@knowledge-architect@the-owner-decides`. This discussion rewords that goal (see "The owner's goal rulings").
- **Satisfaction (L2420):** "#criterion-ruled-at-reopening: a firing opens a design discussion with you".

## Threads

### An acceptance criterion is admitted only if it passes the tripwire standard's conditions 1, 3 and 4; a check whose firing shows the work unfinished becomes a claim of the step that builds it `##criteria-test-silent-premises`

- **Proposer and round:** the agent, round 1 (reply L2068), answering the owner's `argument@owner-word-and-criteria@arg-1` and `argument@owner-word-and-criteria@arg-2`.
- **Final state:** approved, round 2.
- **Shape (L2068):** "silent: no test, gate or review axis would report the failure; what the decision forbids: firing would show the decision wrong, not the work unfinished; a basis for any number." Condition 2 does not apply, "since the landing reads it". The checker's grammar and the `acceptance` item kind stay (L2420).
- **Arguments:** `argument@owner-word-and-criteria@arg-1`, `argument@owner-word-and-criteria@arg-2`, `argument@owner-word-and-criteria@arg-3`, `argument@owner-word-and-criteria@arg-4`, `argument@owner-word-and-criteria@arg-5`, `argument@owner-word-and-criteria@arg-6`, `argument@owner-word-and-criteria@arg-10`, `argument@owner-word-and-criteria@arg-12`, `argument@owner-word-and-criteria@arg-16`.
- **Closed by the owner, round 2 (L2070):** "criteria-test-silent-premises accepted. I was worried about fully losing the criteria that show unfinished work, but keeping them as claims for the steps that build them is the right solution."
- **Confirmed, round 11 (L2811):** "table approved, you can proceed with the spec then implementation."

### Keep criteria as they are, and answer a firing with a repair first `##criterion-firing-repairs-first`

- **Proposer and round:** the agent minted it as a rival in round 1 (L2068). Its idea comes from the owner's round 1 message (`argument@owner-word-and-criteria@arg-1`).
- **Final state:** ruled out, by argument, in the round 2 reply (L2116, "Decided"). Shown ruled-out in every later checkpoint; the owner approved the table in round 11.
- **Arguments:** `argument@owner-word-and-criteria@arg-1`, `argument@owner-word-and-criteria@arg-4`, `argument@owner-word-and-criteria@arg-7`, `argument@owner-word-and-criteria@arg-14`.
- **Closed by argument (L2116):** "#criterion-firing-repairs-first is ruled out. Ground: under the approved shape a completeness failure is a step claim, so a criterion's firing is by definition a failure of the decision."

### Remove acceptance criteria `##criteria-removed`

- **Proposer and round:** the agent minted it as a rival in round 1 (L2068), from the owner's doubt in round 1 (`argument@owner-word-and-criteria@arg-2`: "Iḿ wondering how much acceptance criteria are even useful").
- **Final state:** ruled out, by argument, in the round 2 reply (L2116, "Decided"). The owner approved the table in round 11.
- **Arguments:** `argument@owner-word-and-criteria@arg-2`, `argument@owner-word-and-criteria@arg-3`, `argument@owner-word-and-criteria@arg-8`, `argument@owner-word-and-criteria@arg-12`, `argument@owner-word-and-criteria@arg-15`.
- **Closed by argument (L2116):** "#criteria-removed is ruled out. Ground: it loses the 3 measurements no other check takes, and your ruling on AC4."

### Keep the current design of acceptance criteria `##criteria-as-they-stand`

- **Proposer and round:** the agent, round 1 (L2068), as the incumbent.
- **Final state:** ruled out, round 2 reply (L2116 delta: "about 49 of 61 duplicate a test or a review"). The owner approved the table in round 11.
- **Arguments:** `argument@owner-word-and-criteria@arg-4`, `argument@owner-word-and-criteria@arg-6`, `argument@owner-word-and-criteria@arg-12`.
- **Closed by argument:** the census (`argument@owner-word-and-criteria@arg-12`) and the firing history (`argument@owner-word-and-criteria@arg-4`). It is not in a "Decided" list; its ruled-out state appears in the delta tables only.

### The owner gives no ruling on acceptance criteria at the premortem, and rules in the design discussion that a firing opens `##criterion-ruled-at-reopening`

- **Proposer and round:** the owner, round 2 (L2070). The agent minted the slug in its round 2 reply (L2116). The agent's own round 1 default was the opposite: "you keep ruling on each one" (`argument@owner-word-and-criteria@arg-9`).
- **Final state:** presumed-settled from round 2 to round 10; approved in round 11, when the owner approved the table that listed it.
- **Arguments:** `argument@owner-word-and-criteria@arg-9`, `argument@owner-word-and-criteria@arg-11`, `argument@owner-word-and-criteria@arg-13`.
- **The owner's words, round 2 (L2070):** "About ruling on AC : with these conditions for keeping one, I think my ruling is less useful. But the reopening would then show a true failure of the design that I approved, so maybe my word is useful on what I would consider a reopening of the design. However, if the reaction is reopening under a discussion with me, then I can rule through the discussion at reopening, whether I consider the design to have failed or not, which I think would probably be more convenient to have agents work autonomously, by having one less stopping point. Due to that, overalll I lean for no ruling at premortem, since expected reopening is a design discussion with me." The same message adds: "Since they become rare, I agree the ruling costs little."
- **Closed, round 11 (L2811):** "table approved, you can proceed with the spec then implementation." The round 10 reply had said: "a word against it closes the two presumed threads".

### The goal's clause is narrowed to what the goals and the record leave open; outside a discussion the agent decides what they settle and reports it `##exchange-for-what-the-record-leaves-open`

- **Proposer and round:** the agent, round 6 reply (L2751). It absorbs the owner's first option of round 6, "criteria about whether a decision does earn a discussion or not", which the agent called "your option C" (L2751, "Decided": "Your option C is absorbed into #exchange-for-what-the-record-leaves-open").
- **Final state:** approved, round 7.
- **Shape:** the goal `goal@knowledge-architect@design-with-agents-as-counterparts` reworded as drafted (see "The owner's goal rulings").
- **Arguments:** `argument@owner-word-and-criteria@arg-17`, `argument@owner-word-and-criteria@arg-18`, `argument@owner-word-and-criteria@arg-22`, `argument@owner-word-and-criteria@arg-23`.
- **Closed by the owner, round 7 (L2754):** "Point 1 agreed on exchange-for-what-the-record-leaves-open and the goal rewording"

### Every decision that earns a head goes through a full design discussion `##every-head-through-discussion`

- **Proposer and round:** the owner, round 6 (L2429): "Or I can lower my expectation for this, and instead accept that decisions must go through discussion, thus lowering autonomy but being more aligned with my goals." The agent minted it as "your option B" in its round 6 reply.
- **Final state:** ruled out, by argument, round 7 reply (L2758, "Decided").
- **Arguments:** `argument@owner-word-and-criteria@arg-18`, `argument@owner-word-and-criteria@arg-24`, `argument@owner-word-and-criteria@arg-32`.
- **Closed by argument (L2758):** "#every-head-through-discussion is ruled out. Ground: the approved goal rewording draws the line at what the record settles, which this rival would ignore."

### An approved default stands on the session's argument, not on the owner's word `##approved-default-stays-the-sessions`

- **Proposer and round:** the agent, round 6 reply (L2751), from the owner's round 6 point (`argument@owner-word-and-criteria@arg-19`).
- **Final state:** superseded, round 7 reply (L2758), by `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Shape as proposed (L2751):** "your words become a ground only where you chose against the default, gave a reason of your own, or the session put a true tie with no default."
- **Arguments:** `argument@owner-word-and-criteria@arg-19`, `argument@owner-word-and-criteria@arg-21`, `argument@owner-word-and-criteria@arg-25`, `argument@owner-word-and-criteria@arg-26`, `argument@owner-word-and-criteria@arg-29`.
- **The owner's words, round 7 (L2754):** "approved-default-stays-the-sessions: The rule is going in the right direction. However, the case "gave a reason of [my] own" should probably not stand as a direct ruling. If I put a new argument that was not considered yet, this is supposed to be challenged and discussed, per design-with-agents-as-counterparts."
- **Closed by argument (L2758, "Decided"):** "#approved-default-stays-the-sessions is superseded by #heads-stand-on-argument, which carries its whole shape."

### The transcript reviewer reports a ruling as misstated only on a reversal, a relaxation, a false attribution or a turned direction `##ruling-read-by-its-direction`

- **Proposer and round:** the agent, round 6 reply (L2751), from the owner's round 6 point 2 (`argument@owner-word-and-criteria@arg-20`).
- **Final state:** approved, round 7.
- **Shape (L2751):** the reviewer reports a ruling as misstated only where the record "reverses it; relaxes what you approved, so a check, a claim, a criterion or the scope of work gets weaker; attributes to you a decision you did not make; turns its direction." "A refinement within its direction is no finding, whether stricter, more detailed or reworded. "Approved", "accepted" and "agreed" are read as approving the idea, not the wording."
- **Arguments:** `argument@owner-word-and-criteria@arg-20`, `argument@owner-word-and-criteria@arg-27`, `argument@owner-word-and-criteria@arg-28`.
- **Closed by the owner, round 7 (L2754):** "ruling-read-by-its-direction accepted."

### A design head stands on its argument; the owner's facts are premises, the owner's intent is a goal, and only a choice the argument leaves open is recorded as the owner's `##heads-stand-on-argument`

- **Proposer and round:** the owner, round 7 (L2754), as a refinement of `thread@owner-word-and-criteria@approved-default-stays-the-sessions`. The agent minted the slug in its round 7 reply (L2758) and added the residual-fork default.
- **States:** presumed-settled (round 7 reply); in-discussion (round 8 reply and round 9 reply); presumed-settled (round 10 reply); approved, round 11.
- **Final table (round 9 reply, L2795):**

  | what you bring | where it lands |
  | --- | --- |
  | a fact only you hold, a doubt included | a premise, attributed as your report, open to challenge |
  | an intent an agent cannot derive | a goal, which the head cites |
  | an argument, one that weighs a cost included | argued, recorded on its merits |
  | an approval of the session's default or threshold | no ground: the session's decision on its argument |
  | a reading of a source | the source |
  | a choice the argument leaves open: a true fork, a threshold you set, a local cost you accept | after asking whether a goal states it: "the owner chose X over Y, which no goal ranks" |

  "The overrule row is gone." The round 8 reply's row "a choice against the argument: an overrule or a veto" left in round 9, on `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Arguments:** `argument@owner-word-and-criteria@arg-21`, `argument@owner-word-and-criteria@arg-26`, `argument@owner-word-and-criteria@arg-29`, `argument@owner-word-and-criteria@arg-30`, `argument@owner-word-and-criteria@arg-31`, `argument@owner-word-and-criteria@arg-33`, `argument@owner-word-and-criteria@arg-34`, `argument@owner-word-and-criteria@arg-35`, `argument@owner-word-and-criteria@arg-36`, `argument@owner-word-and-criteria@arg-37`, `argument@owner-word-and-criteria@arg-38`, `argument@owner-word-and-criteria@arg-39`, `argument@owner-word-and-criteria@arg-42`, `argument@owner-word-and-criteria@arg-43`.
- **The owner's words:**
  - round 7 (L2754): "This is why I believe nothing or nearly nothing under design heads should actually stand on my rulings. Because going through challenge and discussion means the final decision is likely to have been selected from argument, and this actually significantly restrict cases that would need to stand on my word. What stand on my word is either facts I hold (recorded in arguments), or a wider intent that an agent cannot derive on its own (such as wanting compatibility with a platform, or needing a flexible implementation to facilitate later features), which is probably something that belongs in goals and not design."
  - round 8 (L2766): "heads-stand-on-argument: I think we are converging. Have you properly challenged my refinement though ? I'm actually a bit surprised that you did not find more to it than the residual case of the true fork (I agree with the proposed handling btw). In particular, is the table of "what you bring" sufficiently exhaustive ? I have no idea for a missing row, but I want to be sure we are not missing a case."
  - round 9 (L2773), on thresholds and costs: "About the two cases for the fork row: agreed on set threshold and accepting a cost, whem set by me they must be marked as such. A threshold set (or proposed and accepted by me) by an agent should not end up attributed to me though."
- **Closed, round 11 (L2811):** "table approved, you can proceed with the spec then implementation."
- **Refined after the plan review, rounds 12 to 14:** the last row records a fork's choice in its weak form, with the guard against reversals back and forth, on the owner's argument (`argument@owner-word-and-criteria@arg-48` to `argument@owner-word-and-criteria@arg-52`). The owner's words, round 14: "Agreed on this shape, with the guard against back and forths". See D5. After the second plan review, an approval at a fork is recorded as the session's choice, per D7, and a re-pick at an unchanged fork needs a goal, a fact or an argument, per D8; the owner's words, round 16: "agreed on the defaults".

### The owner changes a decision through a goal, a fact or an argument, never by an overrule that gives none `##intent-through-goals-not-overrule`

- **Proposer and round:** the owner, round 9 (L2773). The agent minted the slug in its round 9 reply (L2795), after challenging it (`argument@owner-word-and-criteria@arg-44`).
- **Final state:** approved, round 10.
- **Shape (L2795):** the goal `goal@knowledge-architect@the-owner-decides` reworded as drafted (see "The owner's goal rulings"); the design skill's expectation set amended in three places (see "Other rulings"); "The design skill's decision authority loses "can overrule any decision""; two lines that say the owner "overrule[s] by reading the work" become "contest", in the decision-recording and project-audit skills; "Twelve mentions in six files of the design homes and goals get the same treatment." What stays the owner's and is not an overrule: "authorising an action that can't be undone, such as a release or a deletion, and the merge."
- **Arguments:** `argument@owner-word-and-criteria@arg-34`, `argument@owner-word-and-criteria@arg-36`, `argument@owner-word-and-criteria@arg-39`, `argument@owner-word-and-criteria@arg-40`, `argument@owner-word-and-criteria@arg-41`, `argument@owner-word-and-criteria@arg-44`.
- **The owner's words, round 9 (L2773):** "I think, instead, that it would be better to remove this path of overuling a decision completely, from goal@knowledge-architect@the-owner-decides itself , and have me be able to "overrule" only through goals or arguments."
- **Closed by the owner, round 10 (L2798):** "intent-through-goals-not-overrule accepted with goal wording"

## Arguments

### A firing usually means the work is unfinished, so reopening is the wrong reaction `##arg-1`

- **Round:** 1. **Given by:** the owner (L1800).
- **Threads:** `thread@owner-word-and-criteria@criterion-firing-repairs-first`, `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Key words:** "The current instruction is to treat it as reopening the design and decisions it bears on, if I'm not mistaken. I believe this is a weird reaction. Most of the time, an acceptance criterion firing means the work is just not properly finished and actually needs repairs to be completed, isn'that the main point that would make sense ?"

### Acceptance criteria may misdirect attention from what matters more `##arg-2`

- **Round:** 1. **Given by:** the owner (L1800).
- **Threads:** `thread@owner-word-and-criteria@criteria-removed`, `thread@owner-word-and-criteria@criteria-as-they-stand`, `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Key words:** "TBH, Iḿ wondering how much acceptance criteria are even useful. They give a generic baseline to judge the final result of the work, but they are not really the full extent of what must be judged, so I'm even wondering wether they could act as a misdirection, a distraction diverting attention from more important points"

### The owner ruled AC4 a recurring acceptance criterion that tests a decision `##arg-3`

- **Round:** 1. **Given by:** the agent (L1921, L2068).
- **Threads:** `thread@owner-word-and-criteria@criteria-removed`, `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Key words (L1921):** "you once ruled that the audit's count check (AC4) is "a recurring acceptance criterion of the work that creates or changes an audit", not a tripwire. That is a criterion that tests a decision, not the work". L2068 cites `issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s`.

### Of about ten past firings, one tested a decision; the others showed unfinished work `##arg-4`

- **Round:** 1. **Given by:** the agent (L2068).
- **Threads:** `thread@owner-word-and-criteria@criteria-test-silent-premises`, `thread@owner-word-and-criteria@criteria-as-they-stand`, `thread@owner-word-and-criteria@criterion-firing-repairs-first`.
- **Key words:** the firing history table over seven pieces of work (head rules AC2; `check --staged` AC1; tripwire standard AC1; fragments AC1 and AC2; primer terms AC1 and AC2; agentic-workflow audit AC7 and AC4; this branch AC2). "So one firing in about ten tested a decision. That one was a measurement, not something a review reads off the text, and its firing reshaped how audits repair findings. The others were unfinished work. At least five of them were found by review axes doing their normal job." Labelled by the agent (L2068, "Decided"): "The firing classification above is my reading of the commit messages, not a measurement."

### The design skill's step 7 sends every check only the built code can take to a criterion, whatever its firing would show `##arg-5`

- **Round:** 1. **Given by:** the agent (L2068).
- **Threads:** `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Key words:** "The planning skill defines a criterion as "a bet about a decision the work is built on, where a claim is a bet about the work itself: firing a criterion reopens the decision, not the step". The distinction is right. But the design skill's step 7 routes every premortem cause that "only the built code can check" to an acceptance criterion, whether its firing would show the decision wrong or only the work unfinished. My AC1 and AC2 in this branch were exactly that: checks that three texts say the same thing are completeness checks dressed as bets on a decision."

### The tripwire standard already admits only silent checks; the criterion standard does not `##arg-6`

- **Round:** 1. **Given by:** the agent (L2068).
- **Threads:** `thread@owner-word-and-criteria@criteria-test-silent-premises`, `thread@owner-word-and-criteria@criteria-as-they-stand`.
- **Key words:** "In this branch, three review axes spent findings on AC2's literal wording. The record already argued the same point for tripwires, `design@agent-skills@tripwire-admission-test`: of 49 past tripwire firings, 25 "were found and acted on anyway, through a test, a gate, a review doing its own job". That finding introduced condition 1, silence, for tripwires. The criterion standard, `design@agent-skills@acceptance-criterion-quality`, explicitly does not apply condition 1: "Conditions 1 and 2 do not apply." That is the gap."

### Repairing first fixes the reaction but keeps the duplicate criteria `##arg-7`

- **Round:** 1. **Given by:** the agent (L2068).
- **Threads:** `thread@owner-word-and-criteria@criterion-firing-repairs-first`.
- **Key words:** "This fixes the reaction you noticed. It keeps every criterion that duplicates a review, and so keeps the distraction. It also blurs criterion and claim further, against `goal@agent-skills@only-needed-instructions`."

### Removing criteria loses the AC4 kind and changes a grammar consumers use `##arg-8`

- **Round:** 1. **Given by:** the agent (L2068).
- **Threads:** `thread@owner-word-and-criteria@criteria-removed`.
- **Key words:** "This loses the AC4 kind, a measured premise of a decision judged at the landing. … It would also remove the `acceptance` item kind from the plan-document grammar the checker reads, a change consumers would see in their own plan documents."

### With a standard that ranks criteria, the argument that ended the owner's ruling on each tripwire applies to criteria `##arg-9`

- **Round:** 1. **Given by:** the agent (L2068).
- **Threads:** `thread@owner-word-and-criteria@criterion-ruled-at-reopening`.
- **Key words:** "`design@agent-skills@acceptance-criteria-on-the-owners-word` stands on your view that "acceptance criteria are a point where the owner's ruling is useful". With a standard that ranks criteria, the same argument that ended your ruling on each tripwire (`design@agent-skills@tripwire-recording-authority`) would apply. Under my proposal criteria become rare, so your ruling would cost little. **Default:** you keep ruling on each one."

### Checks of unfinished work are kept as claims of the steps that build them `##arg-10`

- **Round:** 2. **Given by:** the owner (L2070).
- **Threads:** `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Key words:** "I was worried about fully losing the criteria that show unfinished work, but keeping them as claims for the steps that build them is the right solution."

### A ruling inside the reopening discussion removes one stopping point `##arg-11`

- **Round:** 2. **Given by:** the owner (L2070).
- **Threads:** `thread@owner-word-and-criteria@criterion-ruled-at-reopening`.
- **Key words:** "if the reaction is reopening under a discussion with me, then I can rule through the discussion at reopening, whether I consider the design to have failed or not, which I think would probably be more convenient to have agents work autonomously, by having one less stopping point."

### Census: exactly 3 of the past 61 criteria would have been kept `##arg-12`

- **Round:** 2. **Given by:** the agent (L2095, L2116).
- **Threads:** `thread@owner-word-and-criteria@criteria-test-silent-premises`, `thread@owner-word-and-criteria@criteria-as-they-stand`, `thread@owner-word-and-criteria@criteria-removed`.
- **Key words:** "Thirty-two past plan documents held 61 acceptance criteria." "**Exactly 3 of the past 61 criteria would have been kept.**" The three: lints-cost-little (the reference-lints spec: "no test or gate measures runtime"), passes-fall (AC4 of the agentic-workflow axis: "the one that fired and led to a redesign"), passes-converge (the design-record audit: "the same measurement, on another axis"). Of the other 58: "One was withdrawn"; "About 49 become step claims"; "About 5 fail the silence test"; "The rest are completeness checks". "What survives is a measurement of the built result in use, which nothing else in the workflow takes. The per-criterion verdicts are my judgement, not a mechanical count." The list was kept in the session's scratch directory, numbered 1 to 61, the kept ones 22, 39 and 42.
- **Corpus, as measured at the plan review:** every plan document deleted under docs/plans/ in the history of `741f655e`, the commit the census ran on, read at its deleting commit's parent, its "Acceptance criteria" section split at its level-three headings: `git log 741f655e --diff-filter=D --name-only -- docs/plans/`, the generated `index.md` files excluded. It lists 39 documents, of which 32 hold the section and 24 hold at least one criterion, 61 in all; the same command at a later head counts the documents deleted since. 741f655e reached the main branch in the rebase merge whose head is 4a6f7dc2; the head before that merge, 3cbf5ee2, lists the same 39 documents. The three kept: `lints-cost-little` of the spec defined-ids-outside-references, which the quotation above calls "the reference-lints spec"; `passes-fall` of the spec of the agentic-workflow axis; `passes-converge` of the spec of the design-record audit. The other verdicts are the agent's judgement, as it said, and only their order of magnitude is claimed.

### The new conditions are the standard whose absence grounded the owner's ruling on each criterion `##arg-13`

- **Round:** 2. **Given by:** the agent (L2116).
- **Threads:** `thread@owner-word-and-criteria@criterion-ruled-at-reopening`.
- **Key words:** "The case for your ruling, in `design@agent-skills@acceptance-criteria-on-the-owners-word`, was that "no standard ranks a criterion's application". The new conditions are that standard." "**The loss:** with no ruling at the premortem, a criterion that would cost real work, as the audit passes did, enters the plan on the standard alone. Its cost becomes visible to you when you read the plan, not as a question put to you."

### Under the approved shape, a firing is by definition a failure of the decision `##arg-14`

- **Round:** 2. **Given by:** the agent (L2116).
- **Threads:** `thread@owner-word-and-criteria@criterion-firing-repairs-first`.
- **Key words:** "absorbed: under the new rule a repair is a failed claim's fix, and a firing is a real reopening". "Ground: under the approved shape a completeness failure is a step claim, so a criterion's firing is by definition a failure of the decision."

### Removing criteria loses the 3 measurements no other check takes `##arg-15`

- **Round:** 2. **Given by:** the agent (L2116).
- **Threads:** `thread@owner-word-and-criteria@criteria-removed`.
- **Key words:** "loses the 3 measurements above, and you ruled AC4 a recurring criterion".

### This work needs no acceptance criterion of its own `##arg-16`

- **Round:** 5 (reply L2420), repeated in the round 7 and round 10 replies. **Given by:** the agent.
- **Threads:** `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Key words (L2420):** "**Acceptance criteria for this work: none.** Under its own rule, every check it owes, such as "step 7 and the planning skill state the same routing", is a completeness check, so each becomes a step claim." (L2808): "Every check this work owes compares texts it writes, so each is a step claim."

### The new counterparts goal conflicts with the owner's recorded objectives and with proceeding on a default `##arg-17`

- **Round:** 6. **Given by:** the owner (L2429).
- **Threads:** `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`, `thread@owner-word-and-criteria@every-head-through-discussion`.
- **Key words:** "the goal  design-with-agents-as-counterparts we just added kinda conflicts with my objectives as recorded in an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own, on the points that all design heads must have gone through a discussion. I think it might also conflict with the current rules about an agent being able to proceed on a decision with a default, and awaiting my ruling only before merge."

### The owner wants more agent autonomy and still a say in design through discussion `##arg-18`

- **Round:** 6. **Given by:** the owner (L2429).
- **Threads:** `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`, `thread@owner-word-and-criteria@every-head-through-discussion`.
- **Key words:** "It is a fact that I want agents to have more autonomy, especially after a spec has already been written from a design discussion. However, I still want to have a say in design through discussion. This has to be brough into a self consistent shape. Maybe there could be criteria about whether a decision does earn a discussion or not. But it would probably have high chances of misfiring. Or I can lower my expectation for this, and instead accept that decisions must go through discussion, thus lowering autonomy but being more aligned with my goals."

### An accepted default is counted as the owner's ruling, though it stands on its argument `##arg-19`

- **Round:** 6. **Given by:** the owner (L2429).
- **Threads:** `thread@owner-word-and-criteria@approved-default-stays-the-sessions`, `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "I think we should fix the problem that me accepting a default counts as my ruling and the decisions gets attributed to me, when a default actually does stands on its argument (otherwise it probably would not have been the default from the agent). If every such decision starts going through a design discussion, at least it would stop being counted as my ruling I suppose."

### The transcript reviewer is too strict; approval is of the idea and direction, not the wording `##arg-20`

- **Round:** 6. **Given by:** the owner (L2429).
- **Threads:** `thread@owner-word-and-criteria@ruling-read-by-its-direction`.
- **Key words:** "I think that the transcript reviewer tends to be a bit too strict about what counts as a deviation from my word. When I "accept" or "agree" with something (or even approve, actually), it is rarely about the exact wording, instead about the idea and direction. Any refinement that the agent judges relevant later is likely acceptable, unless it is about relaxing a claim or AC to get half finished work to pass, or similar. Common sense has to apply here. I think the addition of the transcript reviewer, a while ago, has been a great idea, but it also has its downsides, and I think it is partially the root cause of having design decisions defaults from agents ending up being attributed to me through my acceptation."

### About 140 phrases across 376 heads attribute something to the owner `##arg-21`

- **Round:** 6. **Given by:** the agent (L2519, repeated L2758).
- **Threads:** `thread@owner-word-and-criteria@approved-default-stays-the-sessions`, `thread@owner-word-and-criteria@heads-stand-on-argument`, `thread@owner-word-and-criteria@ruling-read-by-its-direction`.
- **Key words:** "The attributions are common: about 140 phrases across the design homes' 376 heads attribute something to you ("on the owner's word", "the owner's ruling", and so on). The commit messages don't record misstated-ruling counts in a form I can count reliably, so I won't claim a figure for the reviewer."
- **Measured at the plan review:** no command was kept for the count of about 140, and a re-count with one phrase set gave 158. The figure is an order of magnitude, not a measurement.

### The conflict sits in one clause of the goal; the record already draws the line `##arg-22`

- **Round:** 6. **Given by:** the agent (L2751).
- **Threads:** `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`.
- **Key words:** "**The conflict is real, and it sits in one clause.** The goal you approved says it is "met while … no decision that earns a head reaches the main branch before the owner has met it in that exchange". The record lets a session decide some head-earning decisions alone, by design" (citing `design@agent-skills@derived-decision-proceeds-on-default`, `design@agent-skills@extension-follows-the-ground`, `goal@knowledge-architect@agents-work-without-drift`). "So the line already exists. The goal's clause is just wider than it."

### No new criterion: the record already asks whether it settles a decision `##arg-23`

- **Round:** 6. **Given by:** the agent (L2751).
- **Threads:** `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`.
- **Key words:** "This is your option C, with no new criterion. The criterion is the one the record already applies: does the record settle it? Your worry about misfiring is fair. It is the risk `tripwire@agent-skills@owner-proposal-built-ungrounded` and the backstop's review already watch, and it adds no new judgement call." "Decided": "the criterion it would add already exists as "the record settles it", which `primer@question-authority` applies in every session."

### Every head through discussion costs one stop per head-earning decision `##arg-24`

- **Round:** 6. **Given by:** the agent (L2751).
- **Threads:** `thread@owner-word-and-criteria@every-head-through-discussion`.
- **Key words:** "It is consistent and aligns with your goal as written. It costs one stopping point per head-earning default, review repair or harvest decision. For scale: this branch alone wrote 4 new heads, 3 of them in review repairs, after the discussion. It is also a narrower form of `rejected@agent-skills@every-decision-through-design-skill`. You'd pay autonomy for decisions whose argument the record already carries."
- **Measured at the plan review:** the branch merged as "The installed workflow loads its skills on its own" wrote 3 new heads, 1 of them in a review repair, not 4 and 3. The argument's point, one stop per head-earning decision, does not depend on the count.

### The record contradicts itself on whether an approval is a ground `##arg-25`

- **Round:** 6. **Given by:** the agent (L2751).
- **Threads:** `thread@owner-word-and-criteria@approved-default-stays-the-sessions`.
- **Key words:** "`design@agent-skills@head-ground-is-the-argument` holds that "An approval is not a ground, whatever its words". The other half contradicts it: the primer makes your words the ground where "the owner chose where the argued rivals did not settle it", and the decision-recording skill makes your choice at a tie the ground. A fork put to you *with a default* is where your quick yes becomes your ruling, which is the case the issue records."

### Existing heads are re-grounded by the design-record audit, through a Migration entry `##arg-26`

- **Round:** 6 (L2751), repeated round 7 (L2758). **Given by:** the agent.
- **Threads:** `thread@owner-word-and-criteria@approved-default-stays-the-sessions`, `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words (L2751):** "existing heads attribute decisions to you that were approved defaults. The searchers named about a dozen in the core alone, and `rejected@core@carrier-dir-replaced-by-components` even says "made on the session's recommendation". They'd be re-grounded by the design-record audit axis, through a Migration entry citing that axis, not in this work. This closes `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own`." (L2758): "That is a separate audit pass, not this branch."

### The reviewer's own head says a sane deviation is no finding; its text says otherwise `##arg-27`

- **Round:** 6. **Given by:** the agent (L2751).
- **Threads:** `thread@owner-word-and-criteria@ruling-read-by-its-direction`.
- **Key words:** "The head `design@agent-skills@transcript-reviewer-agent` already says, on your word: "a small addition, even a clearly sane deviation, is no finding". The agent's text says a ruling's scope is "not wider, not narrower, and with no obligation added that the owner did not rule". That last clause turned today's AC2, a stricter check I added, into a major finding."

### The owner's root-cause reading is plausible but uncounted `##arg-28`

- **Round:** 6. **Given by:** the agent (L2751).
- **Threads:** `thread@owner-word-and-criteria@ruling-read-by-its-direction`, `thread@owner-word-and-criteria@approved-default-stays-the-sessions`.
- **Key words:** "a reviewer that flags every gap between your words and the record pushes sessions to record your approvals faithfully as rulings, which is how they become your ground. That's plausible, but I have no count that shows it. I'd fix both, since each stands on its own argument."

### A new argument of the owner is challenged, not taken as a ruling `##arg-29`

- **Round:** 7. **Given by:** the owner (L2754).
- **Threads:** `thread@owner-word-and-criteria@approved-default-stays-the-sessions`, `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "the case "gave a reason of [my] own" should probably not stand as a direct ruling. If I put a new argument that was not considered yet, this is supposed to be challenged and discussed, per design-with-agents-as-counterparts."

### Nearly nothing in a head stands on the owner's word; facts are premises and wider intent belongs in goals `##arg-30`

- **Round:** 7. **Given by:** the owner (L2754).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`, `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "This is why I believe nothing or nearly nothing under design heads should actually stand on my rulings. … What stand on my word is either facts I hold (recorded in arguments), or a wider intent that an agent cannot derive on its own (such as wanting compatibility with a platform, or needing a flexible implementation to facilitate later features), which is probably something that belongs in goals and not design."

### What the owner brings lands as a premise or a goal; a true fork is the one residual case `##arg-31`

- **Round:** 7. **Given by:** the agent (L2758).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "**One residual case is left: a true fork.** Two shapes survive the argument, no fact defeats either, no goal ranks them, and you pick. You said "nearly nothing", so my default is: the discussion first asks whether a goal states the weighing behind your pick; only where none does, the head records "the owner chose X over Y, which no goal ranks", the one place your word stays a ground." Changes named: the primer's section on heads; "Entry test 4 … sends that intent to a goal, not to a head"; `design@agent-skills@head-ground-is-the-argument` rewritten; the transcript reviewer's attribution check shrinks.

### The approved goal rewording draws the line `thread@owner-word-and-criteria@every-head-through-discussion` would ignore `##arg-32`

- **Round:** 7. **Given by:** the agent (L2758).
- **Threads:** `thread@owner-word-and-criteria@every-head-through-discussion`.
- **Key words:** "Ground: the approved goal rewording draws the line at what the record settles, which this rival would ignore." Checkpoint row: "one stop per head-earning decision for decisions the record already settles; the goal now draws the line".

### The owner asks whether the refinement was challenged and the table is exhaustive `##arg-33`

- **Round:** 8. **Given by:** the owner (L2766).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "Have you properly challenged my refinement though ? I'm actually a bit surprised that you did not find more to it than the residual case of the true fork … In particular, is the table of "what you bring" sufficiently exhaustive ?"

### A missing row: an overrule or a veto against the argument `##arg-34`

- **Round:** 8. **Given by:** the agent (L2770).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`, `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "You can overrule any decision, at any time, for any reason (`goal@knowledge-architect@the-owner-decides`). When you overrule an argued closure, or veto a proposal the argument favours, the decision does not stand on argument, because the argument points the other way. … Recording it "on its argument" would make the record false, and recording it nowhere would lose why X lost." Proposal: ask for the reason; with none, record "the owner chose Y over X against the argument".

### A threshold and an accepted cost are forks, not argument `##arg-35`

- **Round:** 8. **Given by:** the agent (L2770).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "**A threshold you set**, such as the 10% bound on the lints' cost, or "40 lines" for a crates.io page. A number picked inside a range the argument doesn't narrow is a fork among values." "**A cost you accept**, such as this branch's "one record search per change", unmet and accepted. That is a weighing. It is a goal when it is general … and a fork choice when it is local."

### The laundering risk: the owner's preference recorded as argument `##arg-36`

- **Round:** 8. **Given by:** the agent (L2770).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`, `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "Today, a head grounded on your words can be reversed only by you. A head grounded on argument can be reversed by any better argument … In a discussion, the agent drifts toward your view, and the final argument is shaped to fit what you wanted. The head then reads as pure argument. A later session finds a better argument and reverses it, against your actual intent, which was never recorded." "**Mitigation:** at convergence, for every thread whose closure your word moved, the discussion asks whether it rests on a fact or an intent not yet recorded."

### Moving intent into goals means more goal drafts for the owner to rule on `##arg-37`

- **Round:** 8. **Given by:** the agent (L2770).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "Intent moved into goals means more goal drafts for you to rule on. Under the goal-setting skill you rule on each goal's wording, so each one is a stop. In practice, these are the moments where you already state intent, and the ruling replaces a head attribution rather than adding to it."

### No further case was found among the record's attributions and this session's rulings `##arg-38`

- **Round:** 8. **Given by:** the agent (L2770).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "I found no further case. I checked these sources: the record's existing attributions: the searchers' examples, the 140 phrases, the kinds in `primer@question-authority`; your rulings in this session: goals, defaults, D3, AC2, thresholds. Every one lands in a row above."

### Asked for a reason, an overrule always yields a fact, an intent or an unstated argument `##arg-39`

- **Round:** 9. **Given by:** the owner (L2773).
- **Threads:** `thread@owner-word-and-criteria@intent-through-goals-not-overrule`, `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "As you concluded, if the discussion asks me for the reason to overrule, it is probably pretty much guaranteed that there is either a fact I hold, a goal/intent I have, or an argument I have not yet stated. Or the other case is an intent/goal that is already recorded but the session failed to properly account for it. I don't think leaving the overule path open is actually needed."

### The overrule conflicts with the counterparts goal and raises the laundering risk `##arg-40`

- **Round:** 9. **Given by:** the owner (L2773).
- **Threads:** `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "TBH, I find that the goal of being able to overrule any decision from goal@knowledge-architect@the-owner-decides is pretty much useless, and probably problematic for the workflow overall, as it actually conflicts with the design-with-agents-as-counterparts we added, by reducing the ability of an agent to challenge my arguments (and increases the risk you mentionned of having arguments that favor my intent, without recording as my intent)."

### The owner never used the overrule path, by intent `##arg-41`

- **Round:** 9. **Given by:** the owner (L2773).
- **Threads:** `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "Also, I think I have never used this overrule path, because my intent is to have design decisions that are grounded on arguments or intent, I think this is simply always better for the long term health of a project."

### A threshold an agent sets or proposes is not attributed to the owner `##arg-42`

- **Round:** 9. **Given by:** the owner (L2773).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "agreed on set threshold and accepting a cost, whem set by me they must be marked as such. A threshold set (or proposed and accepted by me) by an agent should not end up attributed to me though."

### A cost is often weighed down by an argument; goal rulings are an investment that pays off in autonomy `##arg-43`

- **Round:** 9. **Given by:** the owner (L2773).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "Also, about costs, there are at least two outcomes: either simply accept, but often I have arguments that reduce the weight of a cost. For example here, you mentionned me having to rule on more goal wordings being a cost. However, this is actually not that much of a cost IMO, because my complete intent has limits, often a few sufficiently generic goals will be sufficient to carry most if not all of the intent and ground most decisions. So setting up proper goals becomes an initial investment in the project which requires more rulings from me, but over the long term it pays off by increasing the autonimy of agents, because they already know my long term intent."

### Three tests of removing the overrule; it holds `##arg-44`

- **Round:** 9. **Given by:** the agent (L2795).
- **Threads:** `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "**You disagree but can't yet say why.** A doubt is a fact you hold … It enters as a premise, and the discussion builds evidence that discriminates, per loop step 5. Nothing requires an overrule." "**You want to stop and move on.** With one shape standing on its argument, it is built. With two shapes standing, your pick is the fork row. "Stop" ends a discussion. It doesn't have to decide against the argument." "**A goal gives the wrong answer in one case.** You change the goal, under the goal-setting skill." "What stays yours is not an overrule: authorising an action that can't be undone, such as a release or a deletion, and the merge. Your observation that you never used the path, plus the laundering risk, decides it."

### No precise definition of a proper goal is added `##arg-45`

- **Round:** 9. **Given by:** the agent (L2795).
- **Threads:** `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Key words:** "What "proper" means stays deliberately loose. The working instrument is the residual fork question: a fork recorded as "the owner chose X over Y, which no goal ranks" is a candidate goal, especially when the same weighing recurs." "Decided": "the goal-setting skill already holds a goal's shape, and the residual fork question is the instrument; a stricter definition would be a rule for one interaction, against `goal@agent-skills@only-needed-instructions`."

### Having to remind the agent to challenge may show the design skill is not explicit enough `##arg-46`

- **Round:** 10. **Given by:** the owner (L2798).
- **Threads:** none of the minted threads; the question of the design skill's challenge instruction.
- **Key words:** "I've had to remind you to properly challenge my proposals, during this discussion. I think this is a tell that the design skill is not sufficiently explicit about this requirement. At the round where I reminded you of this, you had taken my proposal nearly as is. This might have been because of the scope of the-owner-decides, which was impacting how you weight my proposals and arguments, though, so maybe nothing more needs to be done."

### The challenge instruction exists; one lapse is no recurrence `##arg-47`

- **Round:** 10. **Given by:** the agent (L2808).
- **Threads:** none of the minted threads; the same question as `argument@owner-word-and-criteria@arg-46`.
- **Key words:** "**The instruction exists and is plain.** Loop step 4: "The owner's proposal is a claim to test, not a directive to follow … Fill its missing slots, test it against the criteria, and when a shape exists that meets a named criterion theirs fails, propose it against theirs even when theirs would work." So this was a lapse, not a gap." "**One instance is no recurrence.**" "Under "the owner can overrule any decision", testing your proposal hard looks pointless, and that clause is now gone." "**Default:** no text change; your observation is recorded in the spec's deliberation."

### A head recorded as the owner's ruling stands on an argument the owner gave `##arg-48`

- **Round:** 12. **Given by:** the owner.
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "I do not think this is a ruling, it stands on the argument that too much structure will hinder the ability and freedom of the agent to carry investigations of various shapes, or to freely challenge and refine proposals. This is the kind of proble;s I wanted to fix by preventing defaults to be considered as rulings." Said of `design@agent-skills@structure-the-flow`.

### No argument against an unbounded discussion survives; the head can stand on two goals `##arg-49`

- **Round:** 12. **Given by:** the agent.
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "The strongest case for a bound is cost … But a bound doesn't buy that. You can already end a discussion at any point, since "Stop" ends it, and bounded work already gets a single proposal." The head can rest on `goal@agent-skills@only-needed-instructions` and `goal@knowledge-architect@design-with-agents-as-counterparts`, and on the owner's argument as an argument.

### An intent about a mechanism is always settled by an argument, or is a fork recorded in a weak form `##arg-50`

- **Round:** 13. **Given by:** the owner.
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "in this case were intent is about a mechanism, as you worded it, is actually always decided by arguments (or it is a real fork, and then either there is a missing  goal that clears it, or a free choice with no strong ruling, the decision stated in a weak form so that a new argument found later can reopen it freely)".

### A weak record risks reversals back and forth; a choice reopens only on an argument the fork's record does not hold `##arg-51`

- **Round:** 13. **Given by:** the agent.
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "If any argument can reopen a free choice, two sessions could flip it back and forth on arguments of equal weight." "The guard: a choice reopens only on an argument or a fact that the record of the fork does not already hold, and that ranks the shapes." "That guard is the planning skill's definition of a material finding, applied here, so it adds no new rule."

### Keeping a fork's choice strong protects only an unstated weighing, which belongs in a goal `##arg-52`

- **Round:** 13. **Given by:** the agent (L3230).
- **Threads:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Key words:** "I see no stronger argument in the other direction. Keeping a fork choice strong would protect a weighing you made but didn't state, and that case is exactly what the convergence question sends to a goal." The convergence question was dropped at the second plan review; a weighing the owner states lands by the table, as an argument or as a goal.

## New names, in one place

```text
the residual fork        the head rules' fragment head-ground.md, and `thread@owner-word-and-criteria@heads-stand-on-argument`
the weak form            the same fragment and head
completeness-check-is-a-step-claim   the slug of the head the harvest writes for the routing half of the thread on admitting a criterion
```

The harvest may write the design heads and `rejected` entries its rows name, under the threads'
slugs. The work adds no section, type or command.

## Decided design

### `thread@owner-word-and-criteria@criteria-test-silent-premises`

This thread holds two decisions, and the harvest gives each a head of its own.

- **Admission.** An acceptance criterion is admitted only where it meets the tripwire standard's
  conditions 1, 3 and 4:
  - its failure is silent: no test, gate or review reports it;
  - its firing is what the decision says must not happen, a failure of the decision and not of the
    work;
  - a number in it has a basis.

  The nearest rival is `thread@owner-word-and-criteria@criteria-as-they-stand`. Of the past 61
  criteria, about 49 were completeness checks, which a step claim and the spec-conformity review
  cover, and about 5 failed the silence test, by `argument@owner-word-and-criteria@arg-12`.
- **Routing a completeness check.** A cause that only the built result can check, and whose firing
  would show the work unfinished, is a claim of the step that builds it. The claim carries its test,
  and the spec-conformity review judges it. The nearest rival is dropping such checks, which
  `argument@owner-word-and-criteria@arg-10` defeats: the owner's worry was losing them. The harvest
  writes this head under the new slug `completeness-check-is-a-step-claim`.

**Where both decisions are stated.** Each of these texts states both, or points to their statement:

- loop step 7 of the design skill, and loop step 4's routing of an assumption that only the built
  code can check (lines 553-556);
- the planning skill's section on criteria;
- the tripwire standard's condition 1, a fragment with six hosts: the design, planning,
  decision-recording and issue-tracking skills, the decision-record reviewer and the tripwire
  auditor;
- the standing-state reviewer's lines 90-94, which become: a check that can be applied only once
  unbuilt work is built lives in the plan document of the work that builds it, as an acceptance
  criterion where it tests a decision, and otherwise as a claim of the step that builds it.

The decision-recording skill's line 105, "a recurring acceptance criterion", stays: such a
criterion is the kind the standard keeps.

The planning skill's sentence "A number in a criterion is a threshold the owner sets" (lines
345-346) becomes: a number in a criterion carries its basis. Where the basis leaves a range of
values, the choice among them is a fork among values, per `argument@owner-word-and-criteria@arg-35`:
the session lists its value under the defaults awaiting the owner, so the owner meets it before the
merge, as the counterparts goal requires. The owner's approval of it is recorded as the session's
choice, per D7, and a value the owner sets as the owner's, both in the weak form.

### `thread@owner-word-and-criteria@criterion-ruled-at-reopening`

**The owner gives no ruling on a criterion, at the premortem or later.** This holds wherever a
criterion is proposed, the assembly, a review and an audit included. So:

- the planning skill's criterion "awaiting the owner" (lines 337-344) leaves;
- the `AC` label leaves `design@agent-skills@ruled-items-labelled`;
- `primer@labelled-items`'s example "as an acceptance criterion's into the comment at its test" is
  dropped, leaving the rule with no example;
- the in-change path's record carries no rulings on criteria;
- the design skill's `%%` comment at lines 122-128, and the in-change rule at lines 150-151, keep
  that nothing is committed before the premortem has run, since a cause can still reopen a thread.
  They lose the rulings.
- Loop step 8 (lines 650-653) names the premortem and its causes, with no rulings.

**A firing opens a design discussion with the owner** on the decision the criterion guards, per
D4. Each question that discussion meets goes to its authority:

- whether the design failed is answered by the measurement;
- the shape that replaces it is settled by the argument, where an argument settles it;
- the owner rules its forks.

**The work's state after a firing:**

- The criterion leaves the plan document as a reopened decision. The planning skill's other
  outcome, "an issue entry" (line 352), leaves.
- Each criterion's field "the response" (lines 312 and 564) becomes the decision its firing
  reopens.
- The part of the work that rests on the guarded decision waits for the discussion's outcome.
- The rest of the work goes on.

On the in-change path, the criterion's test is in the change. A firing there opens the same
discussion before the change merges.

**Ground.** Its ground is the standard that ranks criteria now, per
`argument@owner-word-and-criteria@arg-13`, as `design@agent-skills@tripwire-recording-authority`
argued for tripwires.

**The loss, named in round 2.** A criterion that costs real work enters a plan on the standard
alone, and the owner meets its cost by reading the plan document.

### `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`

The goal `goal@knowledge-architect@design-with-agents-as-counterparts` is reworded and written. The
installed text already draws the line the goal states:

- a decision closed on its argument reaches the owner in a round's delta and at the checkpoint;
- a decision met outside a discussion that the record settles is the session's, reported under
  "Decided";
- a decision the record leaves open goes to the owner.

A question put to the owner with a built default, which the merge waits on, is that exchange, per
D3 and `design@agent-skills@default-ruled-before-merge`. No text changes for this thread beyond the
goal.

### `thread@owner-word-and-criteria@intent-through-goals-not-overrule`

The goal `goal@knowledge-architect@the-owner-decides` is reworded and written.

**The design skill's decision authority loses the overrule.**

- The owner decides what `primer@question-authority` reserves to the owner, a fork among it.
- The owner contests any closure by a goal, a fact or an argument.
- "Stop" ends a discussion and decides nothing against an argument.
- "The weighing at a fork is theirs" keeps its rule, with one added clause: an argument or a fact
  that the fork's record does not hold, and that ranks the shapes, is no re-arguing.
- The `parked` state becomes the agent's own dissent after the owner's ruling at a fork.
- Loop step 4's "only an explicit decision converts their statement into a directive" becomes:
  only the owner's word at a fork, or an argument that survives the test, does.

**"Who moves what" follows.**

- The owner's word closes a fork.
- On a thread with one shape standing, the owner's word confirms the closure on its argument, or
  contests it with a goal, a fact or an argument.
- Its example "drop the polling idea" stays: it closes a fork, or a thread when it comes with a
  reason that survives the challenge.
- The rest of the paragraph, on vague words and the checkpoint, is unchanged.

**A re-pick at a fork, per D8.**

- The owner's word reopens a closed thread at any time. The expectation set's item "A closed thread
  is not frozen" stays, since a reopening opens the exchange the counterparts goal gives the owner,
  and decides nothing.
- A new pick at the same fork needs what any change needs: a goal, a fact or an argument. A new
  weighing the owner states counts.

**A bounded default.** The design skill's "the work proceeds on it unless the owner answers
otherwise" (lines 194-195) becomes: the owner stops it, or redirects it with a goal, a fact or an
argument. The same applies to the expectation set's line 177, and to the README's lines 12, 141 and 173,
which say "answers otherwise" or "stops or redirects it". `primer@met-outside-the-task`'s row 1
(line 228), "a default the work proceeds on unless the owner answers otherwise", stays: it covers
only the questions that are the owner's, where the owner's answer decides.

**A criterion derived from a recorded decision** binds until the decision is reversed: by the
session, where the decision's argument is defeated and no fork stands, or by the owner at a fork.
Under the new ground rule, an owner's ground is a fork's choice, so this is the rewrite that
`a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`
asks for, and closes it.

**The expectation set** in `skill@knowledge-architect-retrospective@expectation-sets` changes in
three places, as the owner approved:

- "The owner says why when rejecting something" becomes: the owner changes a decision through a
  goal, a fact or an argument, and gives no word that decides against an argument without one.
- "'Stop, build X' and 'park this' both end the argument at once" becomes: "Stop" ends a
  discussion; "Build X" decides only at a fork, or with a reason that survives the challenge.
- A new item: the owner states the project's long-term intent as goals, investing early.

Its restatement in `path@agent-skills@README.md`, lines 176-215, follows in the same three places:
lines 190-194 ("You say why when you reject something"), lines 208-210, and a new item.

**Other skills.** The decision-recording and project-audit skills say "contests by a goal, a fact
or an argument" where they now say "overrules".

**The merge.** The thread's shape names the merge among what stays the owner's. That clause is not
carried, since whether agents merge is each project's choice, per
`design@agent-skills@questions-go-to-their-authority`.

### `thread@owner-word-and-criteria@heads-stand-on-argument`

A design head stands on its argument. What the owner brings lands by its kind:

| what the owner brings | where it lands |
| --- | --- |
| a fact only the owner holds, a doubt included | a premise, attributed as the owner's report, open to challenge; it gives way to a measurement, per `primer@owner-word-premise` |
| an intent an agent cannot derive, which states an outcome | a goal, which the head cites |
| an argument, one that weighs a cost included | argued, recorded on its merits |
| an approval of the session's default, threshold or weighing of a cost, at a fork included | no ground: the session's decision, or at a fork the session's choice in its weak form, "the session chose X over Y, which no goal ranks; the owner approved it" |
| a reading of a source | the source |
| a choice the argument leaves open, made by the owner: a pick at a true fork against the session's default or with none offered, a threshold the owner sets, a local cost the owner accepts | first, whether a missing goal ranks the shapes, which goes to `skill@knowledge-architect-goal-setting`; otherwise the head records "the owner chose X over Y, which no goal ranks", in its weak form |

An approval at a fork is recorded as the session's choice, per D7. This answers the first case of
the closing issue, `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own`.
It brings back no part of `rejected@agent-skills@head-cites-every-owner-ruling`: an approval cites
no ruling of the owner as a ground, and the owner's own pick is cited only at a true fork, in its
weak form.

**The consequences of the table:**

- **A preference of the owner about a mechanism is no case of its own.** It is an argument or a
  choice at a fork, per D5.
- **Entry test 4 stops being an entry test**, which leaves three:
  - a decision that would earn a head only as the owner's intent is drafted as a goal under
    `skill@knowledge-architect-goal-setting`, not written as a head;
  - an argument the owner made earns a head by tests 1 to 3, like any other;
  - test 4's questions to the owner leave the `Q` row of
    `design@agent-skills@ruled-items-labelled`;
  - "test 4 included" in the decision-record reviewer (line 106) and the design-record auditor
    (line 116) goes.
- **What the ground means to a later session:**
  - A head that records a fork's choice in its weak form, the owner's or the session's, reopens,
    without the owner, on an argument or a fact that ranks the shapes. Once one ranks them, it is no
    longer a fork.
  - The reopening goes through `skill@knowledge-architect-design`, and is reported to the owner
    under "Decided".
  - The guard against reversals back and forth: an argument or a fact that the record of the fork
    already holds reopens nothing. This is the planning skill's test of a material finding, applied
    to a fork's choice. So the head of a fork's choice names the rivals and the costs that were
    weighed.
  - A goal still changes only through the owner.
- **A head written before this rule** is read by the same table until the design-record audit
  re-grounds it:
  - a ground that cites the owner's pick at a fork is a fork's choice in its weak form;
  - an approval cited as a ground is no ground;
  - an intent is a goal to draft.

  So the fragment needs no rule for the time in between.
- **A new member of a head**, under head-entry-tests.md lines 63-64 and 69-74:
  - It is recorded directly where the head's argument covers it.
  - Where the part that admits it is a fork's choice whose words name its members, it goes to the
    owner as one proposal, as now.
  - An approval of a member is recorded in the commit, as the session's decision.
- **The owner's word at other sites is recorded by what it brings**, per the table:
  - the design skill's line 140 (the in-change record) and line 199 (the bounded path's answer);
  - `primer@intent-and-claims` lines 90 and 94, where a reversal goes to the owner at a fork only.
- **`primer@question-authority`'s "a fact only the owner holds, such as an intent, a plan or a
  weighing the owner made"** (line 31) becomes "a fact only the owner holds, such as a plan or what
  the owner observed". The same section already gives the owner "a goal to set or change" and a
  fork, where an intent and a weighing land. `design@agent-skills@questions-go-to-their-authority`,
  which states the same list, is rewritten with it. `primer@owner-word-premise`'s "a ruling on
  intent" stays: the owner still answers an intent, and the table lands it in a goal.

**An attribution keeps the scope and the hedges of the owner's words.** This holds in
`skill@knowledge-architect-decision-recording` lines 143-145, head-ground.md lines 26-27 and the
tripwire auditor's lines 46-48. Dropping a hedge records a firmer ruling than the owner gave, which
is a false attribution. The direction reading of
`thread@owner-word-and-criteria@ruling-read-by-its-direction` governs the work built from a ruling,
not how the ruling is attributed, so these texts stay.

**The nearest rival** to the table: the owner's words as a ground wherever the decision came from
the owner, and a fork's choice reversed only by the owner. It lets a quick approval stand as the
owner's ruling, the first case of the closing issue. It also keeps a choice no argument made from
yielding to the first argument that ranks it. Its one benefit, protecting a weighing the owner made
and did not state, is defeated by `argument@owner-word-and-criteria@arg-52`.

### `thread@owner-word-and-criteria@ruling-read-by-its-direction`

**When a ruling is misstated.** A ruling of the owner is reported as misstated only where the record
does one of these:

- reverses it;
- relaxes what the owner approved: a check, a claim, a criterion or the scope of the work;
- attributes to the owner a decision the owner did not make;
- turns its direction.

**What is no finding.** A refinement within the ruling's direction is no finding, whether it is
stricter, more detailed or reworded. "Approved", "accepted" and "agreed" approve the idea, not the
wording.

**Severities in the transcript reviewer**, in its section `agent@knowledge-architect-transcript-reviewer@how-to-report` (lines 111-125):

- A reversal, and a relaxation that changes what is built or a load-bearing decision, are critical.
- A false attribution of a decision and a turned direction are major.
- A relaxation on a detail that is not load-bearing is minor.
- So are an argument and a reading attributed to the owner that are not the owner's, which change
  no decision.
- The existing rules stay: "neither critical nor minor is major", and a minor misstated ruling
  still goes to the owner.

**Where the reading applies.** It is how the owner's words are read, so it applies wherever fidelity
to an approval is judged:

- the transcript reviewer: its description (line 3), and its section
  `agent@knowledge-architect-transcript-reviewer@check-recorded-rulings` (lines 91-110), whose scope
  bullet is at 104-106;
- the fragment `path@agent-skills@fragments/head-shape.md`, lines 66-70 and 79-82, and its three
  hosts: the decision-recording skill, the decision-record reviewer and the design-record auditor;
- "fidelity to what the owner approved", in the decision-record reviewer (line 109) and the
  design-record auditor (line 118), which becomes "fidelity to what the owner approved, read by its
  direction";
- the project-audit skill's outcome table (lines 201-202), "a widening of what the owner's words
  approved";
- the planning skill's transcript review (lines 526-528), and its two rules that "widens or narrows
  a ruling of the owner, or adds an obligation to one" is a scope change (lines 416-418 and 543-548).
  Both become: a clause that relaxes a ruling, or turns its direction, is a scope change;
- the retrospective's defect kind (lines 103-106), which becomes: a ruling recorded within its
  direction, stricter, more detailed or reworded, is no finding.

`goal@agent-skills@no-contradiction-in-workflow` requires the criteria of the reviews and the audits
to agree. `design@agent-skills@audit-outcomes`, whose "a widening of what the owner's words approved
… goes to the owner" states the audit's rule, is rewritten with it.

**Nearest rival:** the scope check as it stands, "no obligation added". It made a stricter
acceptance criterion a major finding on the previous branch, the one merged as "The installed
workflow loads its skills on its own".

### The owner's goal rulings

#### `goal@knowledge-architect@design-with-agents-as-counterparts`, reworded

Draft as last shown, round 6 reply (L2751), written in the branch's first commit:

```markdown
## The owner designs with agents as counterparts, and spends their attention on the design itself `##design-with-agents-as-counterparts`

The owner and the agent both propose, and each challenges and refines the other's proposals on
their failures, their costs and their benefits, grounded in the record and backed by the
measurements the agent takes. A design discussion is an open, argued exchange where the owner can
challenge any proposal, the agent's own and its refinements of the owner's included, add arguments
or facts, or change the direction of the design; a report alone is not that exchange. Outside a
discussion, the agent decides what the goals and the record settle, and the owner learns of it in
the report and can open the exchange on it. The agent carries the investigation, the measurements
and the tracking of what is open, so the owner's attention goes to the design. It is met while no
proposal of the owner is built before it has been grounded and challenged, and no decision the
goals and the record leave open reaches the main branch before the owner has met it in that
exchange.
```

Approved, round 7 (L2754): "Point 1 agreed on exchange-for-what-the-record-leaves-open and the goal rewording"

#### `goal@knowledge-architect@the-owner-decides`, reworded

Draft as last shown, round 9 reply (L2795), written in the branch's first commit. "The slug stays":

```markdown
## The owner decides what the goals and the record leave open, and is informed of every other decision `##the-owner-decides`

The owner states the project's intent in its goals and brings the facts only the owner holds.
Agents decide what the argument from the goals, the record and the sources the project follows
settles, and record each such decision on that ground. What that leaves open is the owner's: a
choice between shapes that all survive the argument, which no goal ranks, a contradiction no
argument resolves, a goal to set or change, a fact only the owner holds, and an action that cannot
be undone, such as a publication. The owner changes a decision through a goal, a fact or an
argument, which the agents weigh like any other, and never by a word that gives none. A choice the
owner made is recorded as the owner's; a decision the owner approved stands on its argument. It is
met while the owner rules only on what is theirs, and learns of every other decision from the
record and the reports.
```

Approved, round 10 (L2798): "intent-through-goals-not-overrule accepted with goal wording"

### Other rulings and statements of the owner

#### Where the work on acceptance criteria goes (round 3)

L2119: "For AC2, the design stands as repaired
Agreed to merge this work then take the AC work to another branch"

The first line ruled on the earlier branch's AC2. In round 5 the owner accepted the agent's
default Q3 (L2254: "all defaults accepted here"). Under that default, AC2 as the owner accepted it did not
fire, and the ruling "stands as repaired" "then answers nothing and leaves the record" (L2252). The
agent later proposed one spec on one branch for all the threads (L2751: "I'd put it in one spec on
one branch"; L2808: "I start the spec on a new branch"), and the owner approved in round 11.

#### The earlier branch (rounds 4 and 5)

- Round 4 (L2202): "can you give me the exact wording/content of head-created-without-deliberation ?"
- Round 5 (L2254): "all defaults accepted here", on the agent's defaults Q2 to Q6 of L2252.

#### No retrospective (round 6)

L2429: "No retrospective this session, skip it."

#### The owner's autonomy wish (round 6)

L2429: "It is a fact that I want agents to have more autonomy, especially after a spec has already been written from a design discussion. However, I still want to have a say in design through discussion. This has to be brough into a self consistent shape."

#### On the transcript reviewer (round 6)

L2429: "I think the addition of the transcript reviewer, a while ago, has been a great idea, but it also has its downsides, and I think it is partially the root cause of having design decisions defaults from agents ending up being attributed to me through my acceptation."

#### On the ruling cost of rare criteria (round 2)

L2070: "Also, based on these new conditions, how many AC would have been kept from past specs exactly ? Since they become rare, I agree the ruling costs little."

#### The residual fork handling (round 8)

L2766: "I'm actually a bit surprised that you did not find more to it than the residual case of the true fork (I agree with the proposed handling btw)."

#### The expectation-set change (rounds 9 and 10)

The owner's proposal, round 9 (L2773): "This would become part of the expecttion set of the agentic workflow, that the project owner transmits intent via goals, and does not try to overrule decisions with no argument or a deeper justification." And: "BTW, having the project owner set up goals properly should also be in the expectation set I believe. Though I am not certain how what constitute a "proper goal" could be defined precisely."

The change as the agent drafted it, round 9 reply (L2795), for the design skill's expectation set in `skill@knowledge-architect-retrospective@expectation-sets`:

- "**"The owner says why when rejecting something"** widens to: *the owner changes a decision through a goal, a fact or an argument, and gives no word that decides against an argument without one.*"
- "**"'Stop, build X' and 'park this' both end the argument at once"** narrows. "Stop" ends the discussion. "Build X" decides only at a fork, or with a reason."
- "**A new item:** *the owner states the project's long-term intent as goals, investing early.* What "proper" means stays deliberately loose."

Accepted, round 10 (L2798): "The expectation set change is accepted."

#### The next design session, on proper goals (round 10)

L2798: "About what constitutes a proper goal: the goal setting skill already has a few leads to that, but I am already planning to have my next design session specifically on this subject, so no need to provide a more thorough one today."

#### The lapse in challenging, and its conclusion (rounds 10 and 11)

Round 10 (L2798): "One last thing I want to add: I've had to remind you to properly challenge my proposals, during this discussion. I think this is a tell that the design skill is not sufficiently explicit about this requirement. At the round where I reminded you of this, you had taken my proposal nearly as is. This might have been because of the scope of the-owner-decides, which was impacting how you weight my proposals and arguments, though, so maybe nothing more needs to be done."

The agent's default (L2808): "no text change; your observation is recorded in the spec's deliberation."

Round 11 (L2811): "I agree with your conclusion on the lapse. My proposals had been properly challenged and refined in all other orunds, I agree with that."

#### Convergence (round 11)

L2811: "table approved, you can proceed with the spec then implementation."

#### The plan review's defaults (rounds 12 to 14)

The agent put six defaults to the owner after the plan reviews, D1 to D6 of "Defaults awaiting the
owner". The owner's words:

- Round 12, on D5: "What is this specific case ? Is it not resolved if this design heads gets grounded in the goal we added about design discussions ? I do not think this is a ruling, it stands on the argument that too much structure will hinder the ability and freedom of the agent to carry investigations of various shapes, or to freely challenge and refine proposals. This is the kind of proble;s I wanted to fix by preventing defaults to be considered as rulings. This one is recorded as a ruling, but do you have a better argument that goes in the opposite direction ?"
- Round 13: "I agree on the other defaults. But the cause of D5 is not closed. I do not think your default stands, because I believe that in this case were intent is about a mechanism, as you worded it, is actually always decided by arguments (or it is a real fork, and then either there is a missing  goal that clears it, or a free choice with no strong ruling, the decision stated in a weak form so that a new argument found later can reopen it freely)"
- Round 14: "Agreed on this shape, with the guard against back and forths"
- Round 16, on D7 and D8 after the second plan review: "agreed on the defaults"

## Mapping tables

None: the work needs no total function.

## Losing alternatives

### `thread@owner-word-and-criteria@criterion-firing-repairs-first`

- **Lost to:** `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Deciding fact:** under the approved shape a completeness failure is a step claim, so a criterion's firing is a failure of the decision (`argument@owner-word-and-criteria@arg-14`). It also kept every criterion that duplicates a review (`argument@owner-word-and-criteria@arg-7`).

### `thread@owner-word-and-criteria@criteria-removed`

- **Lost to:** `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Deciding fact:** it loses the 3 past measurements no other check takes (`argument@owner-word-and-criteria@arg-12`, `argument@owner-word-and-criteria@arg-15`), and the owner's ruling that AC4 is a recurring acceptance criterion (`argument@owner-word-and-criteria@arg-3`).

### `thread@owner-word-and-criteria@criteria-as-they-stand`

- **Lost to:** `thread@owner-word-and-criteria@criteria-test-silent-premises`.
- **Deciding fact:** about 49 of the 61 past criteria duplicate a test or a review (`argument@owner-word-and-criteria@arg-12`); about one firing in ten tested a decision (`argument@owner-word-and-criteria@arg-4`).

### The owner keeps ruling on each criterion (the agent's round 1 default, never minted as a thread)

- **Lost to:** `thread@owner-word-and-criteria@criterion-ruled-at-reopening`.
- **Deciding fact:** the new conditions are the ranking standard whose absence grounded the ruling (`argument@owner-word-and-criteria@arg-13`), and a ruling at the reopening removes one stopping point (`argument@owner-word-and-criteria@arg-11`).

### `thread@owner-word-and-criteria@every-head-through-discussion`

- **Lost to:** `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`.
- **Deciding fact:** the approved goal rewording draws the line at what the record settles, which this rival ignores (`argument@owner-word-and-criteria@arg-32`); it costs one stop per head-earning decision (`argument@owner-word-and-criteria@arg-24`).

### The owner's first option of round 6, criteria about whether a decision earns a discussion (the agent's "option C", never minted)

- **Absorbed into:** `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`.
- **Deciding fact:** the criterion it would add already exists as "the record settles it", which `primer@question-authority` applies in every session (`argument@owner-word-and-criteria@arg-23`).

### `thread@owner-word-and-criteria@approved-default-stays-the-sessions` (superseded)

- **Absorbed by:** `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **Deciding fact:** its clause "gave a reason of your own" made a new argument of the owner a ground, where the counterparts goal has it challenged (`argument@owner-word-and-criteria@arg-29`, `argument@owner-word-and-criteria@arg-30`).

### The overrule or veto row of `thread@owner-word-and-criteria@heads-stand-on-argument` (withdrawn, round 9)

- **Lost to:** `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.
- **Deciding fact:** asked for a reason, an overrule yields a fact, an intent or an unstated argument (`argument@owner-word-and-criteria@arg-39`); the owner never used the path (`argument@owner-word-and-criteria@arg-41`); and the laundering risk (`argument@owner-word-and-criteria@arg-36`, `argument@owner-word-and-criteria@arg-40`). The agent's three tests found no case that needs it (`argument@owner-word-and-criteria@arg-44`).

### A fork's choice reversed only by the owner (the agent's D5 default, withdrawn)

- **Lost to:** `thread@owner-word-and-criteria@heads-stand-on-argument`, refined in rounds 12 to 14.
- **Deciding fact:** a preference about a mechanism is always settled by an argument or is a fork, and a fork's choice has no argument a later argument would have to defeat (`argument@owner-word-and-criteria@arg-50`); the guard keeps the weak form from reversals back and forth (`argument@owner-word-and-criteria@arg-51`).

### The owner rules every decision a firing reopens (the spec's first wording of `thread@owner-word-and-criteria@criterion-ruled-at-reopening`)

- **Lost to:** D4.
- **Deciding fact:** the reworded `goal@knowledge-architect@the-owner-decides` keeps the owner's rulings to what the goals and the record leave open, and whether a design failed is answered by a measurement.

### The convergence question (the agent's mitigation of premortem cause 6, withdrawn at the second plan review)

- **Withdrawn by:** the agent, on `design@agent-skills@behaviour-additions-need-real-use`.
- **Deciding fact:** it is an instruction about how an agent behaves, argued from a predicted failure, with no instance seen in real use; the head admits no addition on a prediction. Cause 6 stays named for the harvest to judge as a tripwire.

### An approval at a fork recorded as the owner's choice (D2's second half as first ruled)

- **Lost to:** D7.
- **Deciding fact:** the owner reported, in the first case of the closing issue on approved defaults, that an approval of a default is given without analysing it deeply and is not to be attributed to the owner long term; D2's premise, supplied by the agent, missed that report.

## Readings

None: the work reads no external specification.

## Premortem

The seven causes as last listed, in the round 10 reply (L2808), with the thread each stresses as
given where each was first named (L2420, L2758, L2770, L2795). Verdict of each: named as a cause for
the harvest to judge.

| # | cause (L2808) | thread it stresses | first named |
| --- | --- | --- | --- |
| 1 | "Fork weighings pushed into goals swell the goals homes." | `thread@owner-word-and-criteria@heads-stand-on-argument` | round 7 reply |
| 2 | "A fact you reported is recorded unattributed." | `thread@owner-word-and-criteria@heads-stand-on-argument` | round 7 reply |
| 3 | "A relaxation framed as a refinement passes the transcript reviewer." | `thread@owner-word-and-criteria@ruling-read-by-its-direction` | round 7 reply |
| 4 | "A session classes an open decision as settled by the record." | `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open` | round 7 reply |
| 5 | "Completeness checks are still written as criteria, or premise checks are demoted to claims." | `thread@owner-word-and-criteria@criteria-test-silent-premises` | round 5 reply, as two causes |
| 6 | "Your preference shapes an argument and isn't recorded as yours." | `thread@owner-word-and-criteria@heads-stand-on-argument` | round 8 reply |
| 7 | "A short word of yours is read as deciding against an argument." | `thread@owner-word-and-criteria@intent-through-goals-not-overrule` | round 9 reply |

Fuller wording where first named:

- Cause 1 (L2758): "fork weighings pushed into goals swell the goals homes with local preferences, and goals stop reading as what the project is for."
- Cause 2 (L2758): "a fact you reported is recorded as an unattributed premise, and a later session cannot tell it was yours to check with you."
- Cause 3 (L2758): "a relaxation written as "a refinement" passes the reviewer. For example, a claim's test weakened "for precision"."
- Cause 4 (L2758): "a session classes a decision the record leaves open as settled by it, and it reaches main with no exchange. `tripwire@agent-skills@owner-proposal-built-ungrounded` watches part of this."
- Cause 5 (L2420): "a premortem still writes completeness checks as criteria, because "only the built code can check it" reads like a criterion. Firings then keep showing unfinished work." And: "the reverse. A true premise check, like AC4's falling counts, is written as a step claim. When it fails, it is "repaired" instead of reopening the decision, and a wrong decision survives."
- Cause 6 (L2770): "your preference shaped the final argument and isn't recorded as yours, so a later session reverses it on a better argument."
- Cause 7 (L2795): "a session reads a short word of yours, such as "no, do Y", as decided, because the old habit still holds on one side or the other. Then a decision against the argument enters the record unmarked. The batch confirmation at a checkpoint and the reworded expectation set are where it surfaces."

**A cause named once and not in the last list.** The round 5 reply (L2420) named a third cause,
stressing `thread@owner-word-and-criteria@criterion-ruled-at-reopening`: "a criterion that costs real work, as the audit passes did,
enters a plan unseen, and you meet its cost only when reading the spec." The round 7 reply's
premortem and every later one omit it, with no reason given. The round 2 reply (L2116) had named it
as "the loss" of that thread. Verdict: named as a cause for the harvest to judge, like the seven.

**A cause named after the plan review**, round 13, stressing
`thread@owner-word-and-criteria@heads-stand-on-argument`: a fork's choice in its weak form is
reversed back and forth by sessions on arguments of equal weight. Verdict: converted into a named
clause of the design, the guard that a reopening needs an argument or a fact the fork's record does
not hold (`argument@owner-word-and-criteria@arg-51`).

**No acceptance criterion was proposed.** Reason (L2808): "Every check this work owes compares texts
it writes, so each is a step claim." Earlier (L2420): "Under its own rule, every check it owes, such
as "step 7 and the planning skill state the same routing", is a completeness check, so each becomes
a step claim." The owner approved the table and the premortem in round 11.

## Acceptance criteria

None. Every check this work owes compares texts it writes, so each is a step claim, under its own
rule, `thread@owner-word-and-criteria@criteria-test-silent-premises`.

## Implementation sequence

**How every step is done:**

- Steps 1 to 4, and the `%%` lines step 6 touches, edit the installed source under
  `path@agent-skills@content/` and `path@agent-skills@fragments/`.
  - Each edit is written under `skill@knowledge-architect-agent-configuration`, as
    `path@agent-skills@CLAUDE.md` says.
  - Each step then runs `cargo klarch install-agent-skills`.
  - A review of a fragment change names each host's installed copy.
- Each claim is a command over the source, `path@agent-skills@content/` and
  `path@agent-skills@fragments/`, unless it names other paths.
  - Each is run on the tree before the step's edit, and shown to fail there, except the claims of
    steps 5 and 6, which state that the gates still pass.
  - Where a phrase is split over a line break, the claim searches a word of it on its own, or a
    phrase that sits on one line.
- No step lands a mechanism.
- The line numbers are those of "What the work is" and "Decided design", at the commit that adds
  this spec.

1. **Acceptance criteria.**
   - **Sites:** every site "Decided design" names for
     `thread@owner-word-and-criteria@criteria-test-silent-premises` and
     `thread@owner-word-and-criteria@criterion-ruled-at-reopening`, in the design skill, the
     planning skill, the tripwire standard, the primer and the standing-state reviewer.
   - **Claims:**
     - `grep -rn -E 'AC<n>|AC[0-9]' crates/agent-skills/content crates/agent-skills/fragments` finds
       nothing; today it finds planning 174 and 336 and design 632.
     - `grep -rn -i -E 'threshold the owner sets|awaiting the owner, its label|an issue entry or a reopened' crates/agent-skills/content`
       finds nothing.
     - `grep -n -i 'comment at its test' crates/agent-skills/content/PRIMER.md` finds nothing.
     - `grep -rn -i -E 'ruled on its acceptance|rulings on its acceptance|rulings on acceptance$' crates/agent-skills/content/skills/design/SKILL.md`
       finds nothing.
     - `grep -rln 'conditions 1, 3 and 4' crates/agent-skills/content crates/agent-skills/fragments`
       lists the planning skill and the tripwire standard.
     - `grep -rln -i 'claim of the step' crates/agent-skills/content crates/agent-skills/fragments`
       lists the design skill, the planning skill, the tripwire standard and the standing-state
       reviewer.
   - **Fails alone on:** the two skills' sections on criteria.
2. **The overrule.**
   - **Sites:** every site "Decided design" names for
     `thread@owner-word-and-criteria@intent-through-goals-not-overrule`:
     - the design skill's decision authority, "The weighing at a fork is theirs", the `parked`
       state, "Who moves what", loop step 4, the bounded path's lines 194-195 and the sentence on
       a criterion derived from a decision;
     - the expectation set, with its line 177;
     - the README's lines 12, 95, 141, 173 and 176-215;
     - the decision-recording skill's line 111 and the project-audit skill's line 395.
   - **Claims:**
     - `grep -rn -i overrul crates/agent-skills/content crates/agent-skills/fragments crates/agent-skills/README.md`
       finds nothing.
     - `grep -n 'says otherwise' crates/agent-skills/content/skills/design/SKILL.md` finds nothing.
     - `grep -rn -i -E 'answers otherwise|stops or redirects|stop or redirect' crates/agent-skills/content/skills crates/agent-skills/README.md`
       finds no line without "goal, a fact or an argument" beside it.
     - `grep -n -i 'goal, a fact or an argument'` over the retrospective skill and the README finds
       each.
     - `grep -n -i 'investing early'` over the same two files finds each.
     - `grep -rn 'park this' crates/agent-skills/content/skills/retrospective/SKILL.md crates/agent-skills/README.md`
       finds nothing.
   - **Fails alone on:** the design skill's decision authority.
3. **Heads stand on argument.**
   - **Sites:** every site "Decided design" names for
     `thread@owner-word-and-criteria@heads-stand-on-argument`:
     - the fragments head-entry-tests.md (test 4 removed, the member routing) and head-ground.md
       (the table, the weak form and its guard, the reading of an older head, and what the ground
       means to a later session);
     - `skill@knowledge-architect-decision-recording@unargued-approval`;
     - the design skill's lines 140 and 199;
     - the primer's lines 31, 90 and 94;
     - "test 4 included" in the decision-record reviewer and the design-record auditor.
   - **Claims:**
     - `grep -rn -E 'the owner chose where the argued rivals|owner.s choice is then the head.s ground|reversed by the owner|test 4 included|such as an intent, a plan' crates/agent-skills/content crates/agent-skills/fragments`
       finds nothing.
     - `grep -n -i 'which no goal ranks' crates/agent-skills/fragments/head-ground.md` finds the
       owner's and the session's forms.
     - `grep -n -i 'already holds reopens nothing' crates/agent-skills/fragments/head-ground.md`
       finds the guard.
   - **Fails alone on:** the primer's section on heads.
4. **A ruling read by its direction.**
   - **Sites:** every site "Decided design" names for
     `thread@owner-word-and-criteria@ruling-read-by-its-direction`:
     - the transcript reviewer;
     - head-shape.md;
     - the decision-record reviewer's line 109 and the design-record auditor's line 118;
     - the project-audit skill's lines 201-202;
     - the planning skill's lines 416-418, 526-528 and 543-548;
     - the retrospective skill's lines 103-106.
   - **Claims:**
     - `grep -rn -E 'no obligation added|wider, narrower|widens it, narrows it|widens or narrows|or adds an obligation|a widening of what the owner|widen what the owner' crates/agent-skills/content crates/agent-skills/fragments`
       finds nothing.
     - `grep -rln -i 'read by its direction\|within its direction' crates/agent-skills/content crates/agent-skills/fragments`
       lists the transcript reviewer, head-shape.md, the decision-record reviewer, the
       design-record auditor, the project-audit skill, the planning skill and the retrospective
       skill.
   - **Fails alone on:** the transcript reviewer's text.
5. **Changelog.**
   - **Content:**
     - Workflow entries for steps 1 to 4 in the `Next release` section;
     - a Migration entry citing `skill@knowledge-architect-project-audit@design-record-axis`, which
       re-grounds a head that cites the owner's words;
     - then `cargo x changelog`.
   - **Claim:** `cargo test --workspace` passes. Its test `every_published_crate_carries_the_root_changelog`
     fails while a crate's copy of the changelog differs.
   - **Fails alone on:** the changelog copies.
6. **The harvest**, per its rows.
   - **Procedure:**
     - It writes the design homes, the `rejected` homes and the tripwires.
     - A head it renames to its thread's slug is cited by `%%` lines of the installed source: the
       planning skill's and the design skill's comments on acceptance criteria, and the head-ground
       fragment's comment. `grep -rn -E 'acceptance-criteria-on-the-owners-word|acceptance-criterion-quality|head-ground-is-the-argument' crates/agent-skills/content crates/agent-skills/fragments`
       lists them. Those lines are edited to the new slug in the same commit, followed by
       `cargo klarch install-agent-skills`.
     - The citers of the reworded `goal@knowledge-architect@the-owner-decides` that no row names
       are read for a clause that still argues from what the goal no longer states: the overrule,
       "a decision the owner made is recorded as the owner's", or the list "such as an intent, a
       plan or a weighing". `design@knowledge-architect@stays-at-zero-x`'s "the weighing is the
       owner's" stays: a weighing at a fork is the owner's.
     - `issue@core@no-checked-form-cites-a-point-inside-a-section` gives "test 4 included" as an
       example; the example is replaced by a point that still exists.
   - **Claim:** `cargo klarch check` passes. It reports a reference to a renamed or removed head.
   - **Fails alone on:** the design homes.

## Order rationale

- 1 before 2: the criteria rule is the narrower change, and step 2 rewrites the authority text step
  1 leaves standing.
- 2 before 3: the head rules follow the goal the overrule's removal states.
- 3 before 4: the reviewer reads the attributions step 3 shapes.
- 4 before 5, 5 before 6: the changelog and the harvest record what the tree holds.

## Defaults awaiting the owner

The first plan review raised six defaults, put to the owner in one message, round 12; the second
raised two more, round 15. All are ruled.

- **D1, the first commit's attribution of the owner's wish for autonomy.** The message is reworded
  to keep the owner's scope: "especially once a spec written from a design discussion holds it".
  Ruled, round 13 ("I agree on the other defaults"). Bears on
  `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`.
- **D2, an approval that matches two rows of the table.**
  - A threshold or a cost-weighing the agent proposed, which the owner approved, stays the
    session's.
  - At a true fork, the owner's pick, the session's default included, is the owner's choice. This
    half is superseded by D7.

  Ruled, round 13. Bears on `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **D3, the exchange.** A question put to the owner with a built default, which the merge waits on,
  is the exchange the counterparts goal names. Ruled, round 13. Bears on
  `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open`.
- **D4, who rules after a firing.** A firing always opens a discussion with the owner. In it:
  - the measurement says whether the design failed;
  - the argument settles the replacement where it can;
  - the owner rules its forks.

  Ruled, round 13. Bears on `thread@owner-word-and-criteria@criterion-ruled-at-reopening`.
- **D5, the owner's intent about a mechanism.** The agent's default was a landing in the fork row.
  It was withdrawn on the owner's argument, rounds 12 and 13, that such an intent is always settled
  by an argument or is a fork, and that a fork's choice is recorded in a weak form a later argument
  can reopen. The owner agreed the refined shape and its guard in round 14 (see
  `argument@owner-word-and-criteria@arg-48` to
  `argument@owner-word-and-criteria@arg-52`). Bears on
  `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **D6, the goal and the head rule.** The reworded `goal@knowledge-architect@the-owner-decides`
  states "a decision the owner approved stands on its argument" as an outcome, and keeps it. The
  rule's home is the head the harvest rewrites from `design@agent-skills@head-ground-is-the-argument`,
  which cites the goal. The default as put said "The fragment cites the goal"; the session moved
  the pointer to the head, since installed text cites no entry of the project, per
  `primer@where-knowledge-goes`. Ruled, round 13. Bears on
  `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **D7, an approval of the session's default at a true fork.** It is recorded as the session's
  choice in its weak form, "the session chose X over Y, which no goal ranks; the owner approved
  it". Only the owner's own pick, against the session's default or with none offered, is recorded
  as the owner's choice. This supersedes D2's second half, whose premise, supplied by the agent,
  missed the first case of the issue the work closes. Ruled, round 16 ("agreed on the defaults").
  Bears on `thread@owner-word-and-criteria@heads-stand-on-argument`.
- **D8, a re-pick at an unchanged fork.** A reopening opens the exchange and decides nothing. A new
  pick at the same fork needs a goal, a fact or an argument, a new weighing the owner states
  included. Ruled, round 16. Bears on
  `thread@owner-word-and-criteria@intent-through-goals-not-overrule`.

## Harvest

| what | where | when |
| --- | --- | --- |
| `thread@owner-word-and-criteria@criteria-test-silent-premises`, admission | `design@agent-skills@acceptance-criterion-quality` rewritten under the thread's slug, its title stating conditions 1, 3 and 4 | step 6 |
| `thread@owner-word-and-criteria@criteria-test-silent-premises`, routing | a new head under the slug `completeness-check-is-a-step-claim` | step 6 |
| `thread@owner-word-and-criteria@criterion-ruled-at-reopening` | `design@agent-skills@acceptance-criteria-on-the-owners-word` reversed into a head under the thread's slug, with D4's routing; the reversed decision judged for a `rejected` entry; `design@agent-skills@acceptance-criteria-in-the-document`, `design@agent-skills@design-hands-off-to-planning`, `design@agent-skills@in-change-path` and `design@agent-skills@ruled-items-labelled` rewritten | step 6 |
| `thread@owner-word-and-criteria@heads-stand-on-argument` | `design@agent-skills@head-ground-is-the-argument` rewritten under the thread's slug, with the table, D7, the weak form and its guard; `design@agent-skills@a-head-is-owed-by-an-entry-test` with three tests; `design@agent-skills@unargued-approval-is-argued`, `design@agent-skills@extension-follows-the-ground`, `design@agent-skills@divergence-routed-by-its-authority`, `design@agent-skills@questions-go-to-their-authority`, `design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@answer-keeps-its-authority` and `design@agent-skills@attribution-replaces-quotation` rewritten; `design@agent-skills@structure-the-flow` re-grounded on its argument, `goal@agent-skills@only-needed-instructions` and `goal@knowledge-architect@design-with-agents-as-counterparts`; `tripwire@agent-skills@a-shortcut-decision-earns-a-head` reread | step 6 |
| `thread@owner-word-and-criteria@intent-through-goals-not-overrule` | `design@agent-skills@owner-rules-at-a-fork` with D8, `design@agent-skills@derived-decision-proceeds-on-default`, `design@agent-skills@tripwire-recording-authority`, `design@agent-skills@tripwire-stands-on-the-standard`, `design@agent-skills@tripwire-audit-axis`, `design@agent-skills@decisions-reported-to-the-owner` and `design@agent-skills@bounded-path-in-design` rewritten; a head under the thread's slug judged by the entry tests | step 6 |
| `thread@owner-word-and-criteria@ruling-read-by-its-direction` | `design@agent-skills@transcript-reviewer-agent`, `design@agent-skills@misstated-ruling-rated-by-what-it-changes`, `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`, `design@agent-skills@one-home-for-head-rules`, `design@agent-skills@audit-outcomes`, `design@agent-skills@existing-heads-on-touch` and `design@agent-skills@audit-repairs-the-whole-record` rewritten; `tripwire@agent-skills@ruling-lost-in-change` and `tripwire@agent-skills@record-answered-questions-reach-the-owner` reread against them | step 6 |
| `thread@owner-word-and-criteria@criteria-test-silent-premises`, its use of condition 1 | `tripwire@agent-skills@silent-read-too-widely`, its firing clause widened | step 6 |
| `thread@owner-word-and-criteria@exchange-for-what-the-record-leaves-open` | the goal, written; no head | done |
| the losing alternatives | judged for `rejected` entries by the tests of `skill@knowledge-architect-decision-recording@losing-alternatives` | step 6 |
| the three issues that close | `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own` and `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` deleted at step 6, `a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise` at step 2 | steps 2 and 6 |
| the premortem's causes | judged as tripwires, cause 6 included | step 6 |

## Later consequences

- The design-record audit's next pass re-grounds the heads that cite the owner's words, in every
  Component; the searchers of the second search named several in the core.
- The owner's next design session is on what a proper goal is.
