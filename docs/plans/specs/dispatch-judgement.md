# The primer states how a session divides the work between itself and the agents it dispatches, as a judgement, and one check by another session is sufficient verification of a claim the work acts on

## Status and audience

This spec is the plan of the work that answers
`the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, an issue of agent-skills that the commit adding this spec closes,. It adds
to the primer a section on dividing work between a session and the agents it dispatches, a pointer
to it where implementation starts, three standing questions of the retrospective, and the design heads
that argue them, and it opens two `deferred` issues for what the discussion parked.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- No default awaits the owner. Where a later finding puts one to the owner, it waits as
  `skill@knowledge-architect-planning@working-a-slice` says.
- The spec and its work go on one branch: the work's first commit changes nothing the per-commit
  gate checks, so the spec need not merge first, per
  `skill@knowledge-architect-planning@plan-reviews`.
- It was assembled from one transcript. The discussion begins at the owner's message that opens "I
  want to discuss the goal and issue of having the main session focus on design", and ends at the
  owner's message "Good, I have nothing more to add or contest." It holds 8 owner messages, called
  rounds 1 to 8 below; round 7 was sent in two parts, the first interrupted. The agent's reply to
  round N carries the heading "Round N" for N from 1 to 7, the reply to round 4 coming after the
  record search. Find the session by that opening message, not by a file name.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, in the form its last paragraph gives the work
of a spec. The design audit of point 2 is skipped while the work starts in the session where the
discussion converged and no other commit has landed on main since this spec was written.

## Names

- **the primer**: the source `path@agent-skills@content/PRIMER.md`, installed under .claude by the install.
- **the planning skill**, **the retrospective skill**: the sources
  `path@agent-skills@content/skills/planning/SKILL.md` and
  `path@agent-skills@content/skills/retrospective/SKILL.md`.
- **a carrier**: the session or an agent that carries a piece of work.
- **a piece**: what a brief gives one agent, or what the session keeps for itself.
- **a check**: the verification of a claim by a session other than the one that wrote it, against
  the evidence its report carries.
- **the harness's default**: the Agent tool description of Claude Code, read in this session: "Do
  the work yourself when it is a handful of tool calls … When in doubt, don't spawn."
- **install**: `cargo klarch install-agent-skills`, which writes the installed copies under .claude.

## What the work is

Today:

- `goal@session-orchestrates-the-work` has the main session orchestrate agents that carry the work;
  it says neither when a dispatch happens nor for which kind or size of work.
- The primer mentions subagents only in `primer@met-outside-the-task` and `primer@decided-section`:
  a subagent reports those sections, and the dispatcher carries them. No text states when a dispatch
  is worth it.
- The installed skills dispatch absolutely at named steps: the record search, the review axes before
  a merge and at a development checkpoint, the plan reviews, the goal reviewer, the audits and their
  saved workflows, and the transcript extraction at a plan's assembly.
- The installed text checks a returned claim in several places, each with its own words: the review
  skill's dispatcher "Verify each against the tree before relaying or acting on it", the planning
  skill's "Check each finding against the tree, or against the transcript", the record search's
  verdicts "checked as it checks its own claims", per
  `design@agent-skills@session-acts-on-the-verdicts`, and the audit's verifiers with the session's
  verification after them, per `design@agent-skills@one-run-pass` and
  `design@agent-skills@verifier-in-the-run`.
- The retrospective skill's standing questions ask nothing about dispatch.
- The owner's real-use report (round 2): the main session nearly never dispatched where it would
  have helped, and spent context the design discussion needed; in the session that drafted the goal,
  the leads were applied, and the session ended with spare context and the work carried well; since,
  dispatch happens occasionally, not reliably.
- Measured in Claude Code, this session, one run each: a general-purpose subagent holds `Agent` and
  can dispatch a nested agent, and holds no `Workflow`; the primer reaches a subagent of a subagent,
  which quoted the first sentence of `primer@room-to-judge` verbatim without reading a file.

Outside the work:

- how parallel work is partitioned among agents: a shared tree or a worktree per agent, who commits,
  how the parts are merged. A later design discussion, behind the `deferred` issue this work opens,
  on the owner's word (round 3);
- report files that an agent writes as it works and a verifier annotates: a `deferred` issue this
  work opens, since no real session has shown the gap;
- the owner's user-level instructions, outside the project, which the owner aligns with this
  section after the work (round 2);
- the cost a review adds, which `issue@agent-skills@the-retrospective-counts-no-review-cost` holds.

## What is already decided

The decisions the design rests on and does not argue again:

- `goal@only-needed-instructions`, `goal@session-orchestrates-the-work`,
  `goal@the-workflow-improves-through-real-use`.
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@statement-names-an-act`,
  `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@primer-limit`,
  `design@agent-skills@verification-in-primer`, `design@agent-skills@a-scratch-directory-per-subagent`,
  `design@agent-skills@step-is-no-commit`, `design@agent-skills@plan-written-in-the-converging-session`,
  `design@agent-skills@restatement-size-test`, `design@agent-skills@premortem-as-watch-points`.

The recorded decisions the work rewrites, each with the texts that `cargo klarch show` lists as
referencing it, judged at the harvest (step 3):

- `design@agent-skills@session-acts-on-the-verdicts`: its rule on checking a verdict becomes an
  instance of the new check head, which it cites instead of stating the rule. Its other clauses stand.
- `design@agent-skills@primer-limit`: gains the new section as a member.
- `design@agent-skills@premortem-as-watch-points`: its list of the standing questions gains the three
  new questions, each naming what it watches.

The record search ran on the draft of this spec, 18 agents, every report passing
`cargo klarch worklist --verify`. Each verdict other than `unrelated` and `constrains` with action
`none`, by its outcome:

| entry | verdict, action | outcome |
| --- | --- | --- |
| `design@agent-skills@verification-in-primer` | conflicts, check-reversal | answered: the check paragraph says one check stands for the session's own verification, which resolves the two acts "Re-verify a conclusion before acting on it" gives on a verifier's report; the head is not reversed |
| `design@agent-skills@session-acts-on-the-verdicts` | conflicts, check-reversal | rewritten at the harvest to cite the check head |
| `design@agent-skills@one-run-pass`, `design@agent-skills@verifier-in-the-run`, `design@agent-skills@workflow-axis-lenses`, `design@agent-skills@repair-proposals-audited`, `design@agent-skills@verdicts-reach-the-conformance-review` | conflicts, check-reversal | answered: the check is a minimum, "one check is enough", which forbids no second check a skill asks for; none is reversed |
| `rejected@agent-skills@repair-check-left-to-session`, `rejected@agent-skills@repair-defects-left-to-next-pass` | brings-back, new-argument-needed | answered: the minimum drops no check step, so neither comes back |
| `design@agent-skills@premortem-as-watch-points` | conflicts, check-reversal | the questions widen its list within its argument: each names what it watches; the compaction question is dropped, since a session sees an agent's brief and report, not its context |
| `the-installed-text-has-the-main-session-carry-work-it-could-dispatch` | closes, close-with-the-work | closed by the commit that adds this spec: each departure is built or refused on its argument |
| `rejected@agent-skills@head-rules-whole-in-primer` | constrains, respect | the primer "can dilute in a long session": answered by the pointer sentence at point 3 of the planning skill's procedure, where implementation starts |
| `design@agent-skills@a-goal-reading-is-reported` | constrains, respect | the reading of `goal@session-orchestrates-the-work` is named under "Decided" (`thread@dispatch-judgement@dispatch-judgement-in-primer`) |
| `design@agent-skills@one-decision-per-head`, `rejected@agent-skills@head-split-by-independent-reversal` | constrains, respect | the harvest writes one head per decision; the exception of `thread@dispatch-judgement@absolute-dispatch-conditions` stays in the judgement's head |
| `design@agent-skills@extension-follows-the-ground`, `design@agent-skills@new-or-reshaped-head-needs-design` | constrains, respect | the widening of the check rule was argued in this discussion, rounds 4 to 7 |
| `design@agent-skills@no-two-installed-instructions-contradict` | constrains, respect | the minimum form leaves every named check chain standing |
| `issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness` | touches, respect | the judgement head states the section as a weighing beside the harness's default, not a precedence over it, so it does not depend on that issue |
| `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies` | touches, respect | the judgement head states where a fixed dispatch is admitted; the issue stays open |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` | touches | the two parked threads take `deferred` issues with a trigger |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | touches | the cost question is left to it |
| `issue@knowledge-architect@the-retrospective-asks-for-no-tool-a-session-set-up` | touches | it plans a question in the same section; the two are written to fit |
| `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`, `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand` | touches | cited by the partition issue |
| `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`, `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own` | touches | cited by the report-file issue |
| `issue@agent-skills@agent-skills-heads-owing-an-argument-on-one-branch` | touches, respect | through its member on the harness, answered as above |
| `design@knowledge-architect@changelog-entries`, `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@the-branch-writes-its-changelog-entries` | constrains, respect | the branch writes a Workflow entry in `Next release`, a patch |
| `design@query@worklist-bound-is-40000-words` | constrains, respect | the owner's earlier report of reliable agents up to about 200,000 to 250,000 tokens stays the search's bound's ground; no number goes in the primer |
| `rejected@query@one-search-agent-per-anchor` | constrains, respect | its measured start of about 36,000 tokens per agent is the ground of "a fresh agent first reloads its instructions" |
| `design@model@section-homes-carry-slugs`, `design@model@installed-entities-from-the-tree` | constrains, respect | the section's heading carries `{{slug:dispatching-work}}` in the source |
| `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@shipped-text-line-comments`, `design@agent-skills@installed-text-works-with-what-an-installing-project-holds` | constrains, respect | the reasons go in `%%` lines; the shipped text cites no entry |
| `design@check@orphan-rejected-entry` | constrains, respect | each `rejected` entry the harvest writes is cited by its head |
| `rejected@mutate@worktree-holds-the-mutation`, `design@knowledge-architect@no-operation-loses-content`, `design@knowledge-architect@scratch-worktree-outside-git-rules`, `design@knowledge-architect@git-flow` | constrains, respect | carried into the partition issue |
| `rejected@agent-skills@ledger-written-during-discussion`, `design@agent-skills@refused-report-returned-as-text` | constrains, respect | answered in the report-file issue |

The 188 lines of `generated-lines.md` that the search wrote are carried in the message of the commit
that adds this spec.

## Criteria

### The main session orchestrates the agents that carry the work, and the goal fixes no when, kind or size `##orchestration-goal`

Binding, from `goal@session-orchestrates-the-work`. Met by `thread@dispatch-judgement@dispatch-judgement-in-primer`
and `thread@dispatch-judgement@dispatched-piece-carried-whole`.

### No instruction for a case judgement handles well enough `##only-needed-instructions-goal`

Binding, from `goal@only-needed-instructions`. Met: the section states a judgement and no
threshold, and each further paragraph repairs a reading that allows two acts or rests on the owner's
real-use report.

### A capability and the judgement to use it, no structure the work does not need `##no-imposed-structure`

Binding, from `design@agent-skills@capabilities-not-structure`. Met by
`thread@dispatch-judgement@absolute-dispatch-conditions`; the division of a piece is left to its
brief.

### A behaviour addition rests on real-use evidence, or repairs a defect provable by reading `##real-use-evidence`

Binding, from `design@agent-skills@behaviour-additions-need-real-use`. Met: the owner's report
(round 2) for the judgement; a reading that allows two acts for the carried-whole and the check
paragraphs (round 5).

### Every statement names an act it changes `##every-statement-an-act`

Binding, from `design@agent-skills@statement-names-an-act`. Met: each sentence changes a dispatch, a
carry, a check or an answer.

### Checking dispatched work does not raise the total cost much `##check-cost-weighed`

Weighed, a lead of the source issue. Met: one check is enough, and no rule asks for a second.

## Threads

### The primer states how a session divides the work between itself and the agents it dispatches, built on two relations between pieces, as a judgement `##dispatch-judgement-in-primer`

Proposed by the owner, round 1: guidelines in the primer, judgement decides, no absolute condition
of size or kind (`argument@dispatch-judgement@arg-1`, `argument@dispatch-judgement@arg-2`). The
session sharpened it: the evidence rule (`argument@dispatch-judgement@arg-8`), met by the owner's
report (`argument@dispatch-judgement@arg-15`, `argument@dispatch-judgement@arg-18`,
`argument@dispatch-judgement@arg-25`); the harness's default as the mechanism
(`argument@dispatch-judgement@arg-9`, `argument@dispatch-judgement@arg-19`); the owner's three
conditions (`argument@dispatch-judgement@arg-21`, `argument@dispatch-judgement@arg-22`) worked into
(A and B) or C (`argument@dispatch-judgement@arg-27`); the plan document as the brief
(`argument@dispatch-judgement@arg-28`); the asymmetry of the two contexts
(`argument@dispatch-judgement@arg-29`); the measured reload (`argument@dispatch-judgement@arg-30`,
`argument@dispatch-judgement@arg-47`); dilution answered by a pointer
(`argument@dispatch-judgement@arg-45`); a weighing, not a precedence
(`argument@dispatch-judgement@arg-46`); scratch directories (`argument@dispatch-judgement@arg-48`);
the goal's reading (`argument@dispatch-judgement@arg-56`); one carrier for verifying and repairing
(`argument@dispatch-judgement@arg-68`, `argument@dispatch-judgement@arg-73`); and, on the owner's
point that pieces relate to each other too (`argument@dispatch-judgement@arg-77`), the rebuild on two
relations (`argument@dispatch-judgement@arg-79`, `argument@dispatch-judgement@arg-82`). Its rival, no
text, lost to the owner's report and the harness's default; guidance in each skill lost to
`argument@dispatch-judgement@arg-7`. Final state: approved. The owner, round 8: "Good, I have
nothing more to add or contest." Shape: "Decided design", the section. Harvest: the head
`work-divided-by-judgement` in the agent-skills design home.

### An installed text fixes a dispatch only for a step that needs a separate point of view, whose output a check verifies, or at an occasion most work meets `##absolute-dispatch-conditions`

Proposed by the owner, round 1 (`argument@dispatch-judgement@arg-3`,
`argument@dispatch-judgement@arg-4`), sharpened by the session into a test on the step's property
(`argument@dispatch-judgement@arg-13`). Every existing absolute dispatch fits a ground, and the rule
lives in a head, not in the admission standard, since it binds what the installed text states
(`argument@dispatch-judgement@arg-49`). Final state: argued, round 4; confirmed in the owner's word
of round 8. Shape: the section's first paragraph. Harvest: inside the head `work-divided-by-judgement`,
the exception staying with its rule.

### A dispatched piece is what its brief gives the agent, and the agent carries all of it while it is dispatched `##dispatched-piece-carried-whole`

Proposed by the session, round 1, from the issue's lead and the owner's user-level instructions
(`argument@dispatch-judgement@arg-10`). The owner, round 3, feared it forbade the thaum process
(`argument@dispatch-judgement@arg-35`); the session reworded it so the brief defines the piece
(`argument@dispatch-judgement@arg-37`). Kept on reading, not on real use: the section's own Carry
bullet, read at the size of one step, produces the takeover (`argument@dispatch-judgement@arg-58`,
`argument@dispatch-judgement@arg-62`). The premortem added that what comes back unfinished is a new
piece (`argument@dispatch-judgement@arg-64`), and the rebuild made "writing a change and its
commit" the example of one carrier (`argument@dispatch-judgement@arg-80`). Final state: approved,
round 8. Shape: the section's carried-whole paragraph. Harvest: none in a design home, since one
text states it; its reason in a `%%` line at the paragraph.

### The design skill gets no text of its own on dispatching `##design-investigation-dispatch`

Proposed by the session, round 1, as a capability sentence in the design skill's loop. Superseded by
`thread@dispatch-judgement@dispatch-judgement-in-primer`, whose dispatch bullet covers a search for
another shape (`argument@dispatch-judgement@arg-34`). Final state: superseded, round 4. Harvest:
none.

### The development skill gets no edit: a dispatched developer loads it and sends its own checkpoint reviews `##development-dispatch`

Proposed by the session, round 1 (`argument@dispatch-judgement@arg-11`), on the measured nested
dispatch and the primer's list of installed skills (`argument@dispatch-judgement@arg-52`). Final
state: argued, round 4. Harvest: none.

### One check, by a session other than the claim's author, against the evidence its report carries, is sufficient verification of a claim the work acts on `##dispatch-report-standard`

Proposed by the session, round 1, as a sentence on the dispatcher's side
(`argument@dispatch-judgement@arg-12`); the owner gave its principle, round 2: equal trust, one
check is enough (`argument@dispatch-judgement@arg-23`). The record search found the wording "a
check is not checked again" in conflict with the named check chains
(`argument@dispatch-judgement@arg-42`, `argument@dispatch-judgement@arg-43`). The owner, round 5,
asked for a definition instead of a workaround (`argument@dispatch-judgement@arg-57`); the session
found the record holds the principle as a minimum (`argument@dispatch-judgement@arg-59`,
`argument@dispatch-judgement@arg-60`, `argument@dispatch-judgement@arg-61`). The owner, round 6,
made it a general principle with self-verification's lapses as its ground
(`argument@dispatch-judgement@arg-66`, `argument@dispatch-judgement@arg-67`), measured in this
session (`argument@dispatch-judgement@arg-70`). The boundary on a claim nothing acts on was put to
the owner as a fork (`argument@dispatch-judgement@arg-72`); the owner, round 7: "2. Agreed. This
matters most when the claims is acted on." and "Both cases are pretty much the same here actually."
(`argument@dispatch-judgement@arg-76`, `argument@dispatch-judgement@arg-78`). Final state: approved,
round 8. Shape: the section's check paragraph. Harvest: the head `one-check-by-another-session`;
`design@agent-skills@session-acts-on-the-verdicts` rewritten to cite it.

### No token bound goes in the primer; a piece too large is asked about in the retrospective `##dispatch-size-hint`

Proposed by the owner, round 2: at most a hint of a bound, maybe 300k tokens
(`argument@dispatch-judgement@arg-17`). The session argued against a number
(`argument@dispatch-judgement@arg-31`), and the record holds the owner's earlier figure as the
search's ground, not a dispatch guideline (`argument@dispatch-judgement@arg-50`). Final state:
superseded by `thread@dispatch-judgement@retrospective-dispatch-questions`, round 4. Harvest: a
`rejected` entry if `skill@knowledge-architect-decision-recording` admits it.

### The retrospective asks whether the session carried work an agent could have carried, whether an agent came back unfinished or with an unusable report, and whether a returned conclusion proved wrong or was checked twice `##retrospective-dispatch-questions`

Proposed by the session, round 1 (`argument@dispatch-judgement@arg-14`); reshaped after the search,
each question naming what it watches, answerable from the session's side, and met by most sessions
(`argument@dispatch-judgement@arg-51`). Final state: approved, round 8. Shape: "Decided design",
the questions. Harvest: the three questions in `design@agent-skills@premortem-as-watch-points`.

### Report files written as an agent works, annotated by a verifier, wait for evidence `##dispatch-report-file`

Proposed by the owner, round 2 (`argument@dispatch-judgement@arg-24`); the session found it predicts
a behaviour (`argument@dispatch-judgement@arg-33`). Final state: parked. Cause: a real session whose
returned report could not be acted on without redoing the work. Re-entry: the trigger of the
`deferred` issue `dispatched-agents-write-their-report-as-they-work`, a retrospective answer to the
second dispatch question naming such a report (`argument@dispatch-judgement@arg-53`).

### How parallel work is partitioned among agents is decided at design and planning, in a later discussion `##parallel-work-partition`

Proposed by the owner, round 3 (`argument@dispatch-judgement@arg-35`,
`argument@dispatch-judgement@arg-36`). The owner, round 3: "We can probably keep the guidelines such
as in the draft for now, and keep that idea for a later design discussion." Final state: parked.
Cause: a piece of work whose plan wants parallel agents. Re-entry: the trigger of the `deferred`
issue `how-parallel-work-is-partitioned-among-agents`, which carries the facts of
`argument@dispatch-judgement@arg-38`, `argument@dispatch-judgement@arg-39` and
`argument@dispatch-judgement@arg-54`.

### The owner's user-level instructions are aligned with the section after the work `##user-level-alignment`

Proposed by the owner, round 2 (`argument@dispatch-judgement@arg-20`). Final state: approved, round
8. Outside the project: no harvest. Done after the work, in the owner's dotfiles, under the owner's
own instructions for them.

## Arguments

### Absolute conditions on when or what to dispatch would fail or grow without bound, size being the most varied measure `##arg-1`

Round 1, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "we should not give absolute conditions for when to dispatch subagents, or even for which kind of work"; "these tasks can have very varied shapes, so absolute instructions would either fail, or need to become very large to handle every case (especially for determining whether it is worth dispatching or not by size of work, which is the most varied metric that can be faced)".

### Guidelines in the primer reach every agent, and judgement decides `##arg-2`

Round 1, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "it is better to provide guidelines, in the primer where they reach all agents, and to rely on agent judgement for deciding."

### A mandated step an agent does better, or a review that needs another point of view, is the exception on what to dispatch `##arg-3`

Round 1, owner. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "The exception to "which tasks to dispatch" are steps that the workflow mandates where an agent can do the work better (and in such cases we try to provide a shipped subagent definition, such as record search), or reviews, where a subagent is clearly better, because it can take a different point of view."

### A mandated step, or an occasion most work meets, is the exception on when to dispatch `##arg-4`

Round 1, owner. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "The exception to "when to dispatch" has the same shape: a step the workflow mandates (such as record search, plan writing...), or situations and tasks which can be anticipated, because they happen for most work, if not all (such as a merge of a branch)"

### A case that judgement handles gets no instruction `##arg-5`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: `goal@only-needed-instructions`: "A case that judgement handles well enough gets no instruction, whichever goal the instruction would serve."

### A size threshold is a rule per interaction, which the structure head's measurement argues against `##arg-6`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: `design@agent-skills@capabilities-not-structure`: "Its measurement is the argument against size thresholds: in one revision of designing-together, 34 edits produced 10 interaction defects, nearly all of them between rules that scripted one exchange. A size threshold for dispatching is that kind of rule. Every activity would meet it, and each would need its own carve-out."

### No skill loads at the moment a dispatch is judged, so the primer is the home `##arg-7`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: `design@agent-skills@primer-limit`: "A session meets the question of whether to dispatch in every activity, and no skill loads at that moment. So by its own test, the primer is the home."

### A behaviour addition needs real-use evidence, and a guideline that changes no act is not written `##arg-8`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@design-investigation-dispatch`. Key words: "The evidence rule applies to this addition."; "A guideline for a case where sessions already dispatch changes no act. It fails `design@agent-skills@statement-names-an-act`, so it is not written."; "A guideline for implementation or design-time investigation needs your observation".

### The harness's default leans against dispatching, and no installed text weighs on the other side `##arg-9`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "*the harness's default tells a session to do the work itself when in doubt, and no installed text weighs on the other side, so a session carries a coherent change itself.*"; ""Do the work yourself when it is a handful of tool calls … When in doubt, don't spawn." Under test 3 of what earns a head, that external fact earns the decision a head."

### One coherent change is never split between two authors `##arg-10`

Round 1, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "Read together, both say *do not split one coherent change between two authors*: either the main session carries it whole, or one agent carries it whole, commits included."; "If it said "keep authorship", the session would read it as "keep the work"."

### A dispatched developer can send its own checkpoint reviews; a saved workflow stays with the main session `##arg-11`

Round 1, session. Bears on `thread@dispatch-judgement@development-dispatch`. Key words: "A dispatched developer can itself send the checkpoint review axes that `skill@knowledge-architect-development` names. So dispatching the implementation takes no reviews away from it."; "A saved workflow, such as an audit axis's run, stays with the main session."; "the guidelines cover `Agent` dispatches only."

### The primer binds a subagent's own verification; the dispatcher's side is unstated `##arg-12`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "The primer's `primer@verify-before-relying` already binds every subagent"; "What the primer does not state is the dispatcher's side: *a conclusion that comes back is a claim, checked like one's own before acting on it.*"

### An absolute dispatch is admitted only on a property of the step that holds whatever the task's size `##arg-13`

Round 1, session. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "**an installed text states an absolute dispatch only where its ground is a property of the step that holds whatever the task's size.** The two such properties are a fresh point of view (reviews, the goal reviewer) and an output whose shape a check verifies (the record search, the audits). Any other "always dispatch X" is a size claim in disguise"; "Without this test, the two classes are a list. A later writer would extend the list by analogy, and the same threshold creep would come back."

### The retrospective questions produce the evidence that corrects judgement-based guidelines `##arg-14`

Round 1, session. Bears on `thread@dispatch-judgement@retrospective-dispatch-questions`. Key words: "these are what produces the real-use evidence that judgement-based guidelines are later corrected by"

### The main session nearly never dispatched, and spent context the design needed `##arg-15`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "I nearly never saw the main session dispatch subagents for tasks where I believe it could have been useful, especially for saving context that was not well spent and should probably have been kept for design discussions."

### A session with a large context costs more usage than one with a small context `##arg-16`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "There is also the cost of a session whose context gets high, which costs more usage quota than a session with nearly empty context AFAIK. This was discussed at the last redesign of the record search."

### Numbers do not belong in guidelines, at most a hint of a token bound per agent `##arg-17`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-size-hint`. Key words: "I'm not sure we should include numbers in guidelines though. At most, a hint that "if dispatched agents regularly exceed <bound> tokens, they might not be sufficiently fine grained" with bound at maybe 300k."

### In the goal-drafting session the leads were applied and worked; since, dispatch is occasional `##arg-18`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "In the session that drafted the orchestration goal and wrote the issue, the session applied my leads, and from my observation it went very well, with the main session ending the task with enough spare context"; "In sessions since that, I have seen that happen occasionally, I guess due to sessions reading the orchestration goal, but definitely not reliable."

### The harness keeps dispatching opt in because early users want to watch the work `##arg-19`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the claude code harness tries to discourage dispatch by default, keeping that feature opt in, because users might not like it (especially at the beginning, where trusting agents is harder, and early users prefer keeping a close watch on agent work)."

### The owner's user-level instructions are more likely the flawed text, and are to be aligned `##arg-20`

Round 2, owner. Bears on `thread@dispatch-judgement@user-level-alignment`. Key words: "I would like to bring my root CLAUDE.md to align with the guidelines we settle on in this session. The current text was written without a deeper discussion and brainstorming, so it is more likely to be the flawed one, at least on the wording aspect."

### Three conditions make a dispatch clearly worth it `##arg-21`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the current session's full context is not useful to the task (a brief can carry what the agent will need)"; "the process of the work is not useful to the current session, only the result and/or report is"; "A fresh point of view is needed."; "(I am unsure if they should be "or" or "and")".

### Implementation sits where the conditions are in doubt `##arg-22`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The cases were there is doubt about these is when it is harder to decide, and this includes implementation I believe, where the current context is actually useful, and the process itself can be useful, but the cost of doing it itself might be too big for the session."

### A verifier's check weighs as much as the main session's, and one check is enough `##arg-23`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "there is no reason to put more trust in the main session than in a subagent"; "one round of verifier is enough"; "One verifier verifying the claims of a review is just as much weight, and just as trustable, as the main session verifying the claims of a review. The point is that one verification was done."

### The completeness of a subagent's report, not its judgement, is the weak point `##arg-24`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-report-file`. Key words: "The hard point this leaves is not on trusting the judgement of the verifier or of the initial reviewer, but more on trusting the completeness of the subagent reports. Because those are often written one shot in the last messages"; "Having the main session carry the verification as the advantage that it has high chances to fill any gap in the report. I think that having file reports that agents write and  a verifier can edit might help reduce this gap."

### The owner's report meets the evidence rule `##arg-25`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "Your report meets `design@agent-skills@behaviour-additions-need-real-use`"; "**What it lacked:** context left for the design discussion."; "Your reason for the default (users who want to watch the work closely) is your belief, recorded as such."

### Each turn costs more as the context grows `##arg-26`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the API resends the whole context on every turn, and cached tokens are billed per token at a reduced rate. So each turn of a session costs more as its context grows. This is my reading of the pricing structure. I have not checked it this session."

### A fresh view implies a briefable task; briefable with only the result needed is the clear case `##arg-27`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**C implies A, and makes it a requirement.**"; "**A and B together** is the clear case"; "**A without B:** … Carry it."; "**B without A, or either one partly true,** is your doubt zone, implementation among it."; "**(A and B) or C** is the clear dispatch. Everything else is weighed."

### For implementation, the plan document is the brief `##arg-28`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@development-dispatch`. Key words: "**For implementation, the plan document is the brief.**"; "The process that still matters to the main session comes back through the agent's "Decided" and "Met outside the task" sections".

### The main session's context is worth more than an agent's, since no brief rebuilds it `##arg-29`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**The two contexts are not worth the same.** The main session's context holds the exchange with you, and no brief rebuilds it. An agent's context is rebuilt by sending it again. The harness's generic "when in doubt, don't spawn" ignores this asymmetry."

### A fresh agent pays a fixed reload `##arg-30`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**a fresh agent pays a fixed reload.**"; "a dispatch whose brief costs about as much as the work is not worth it."

### A token bound in the primer measures the wrong figure, varies by model, and serves the next session `##arg-31`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-size-hint`, `thread@dispatch-judgement@retrospective-dispatch-questions`. Key words: "The one figure a session sees per agent, `subagent_tokens`, has an unestablished meaning"; "Context windows differ by model. Under 300k, a 200k-window agent never trips the bound."; "The act the hint changes is cutting pieces finer *next time*. That is a lesson from a finished session, so its occasion is the retrospective, not the moment of dispatch."

### The record already checks search verdicts as the session's own claims, which ends the chain `##arg-32`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Your point, that one check carries the same weight whoever makes it and a check is not checked again, already stands in the record for the search. Its verdicts are checked as the session "checks its own claims, no more and no less". The primer would generalise that, and it ends the chain of verifiers."

### Report files written as an agent works, annotated by a verifier, predict a behaviour without evidence `##arg-33`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-report-file`. Key words: "Reviewers already write their reports to files"; "dispatchers saved condensed copies, or none, in 7 of 12 sessions"; "a verifier that **annotates** each finding rather than rewriting it"; "you report not having read such reports, so this predicts a behaviour without evidence"; "the verifier's brief carries the original brief, so it can see what was asked and is not answered."

### The design skill needs no text, since the primer's dispatch bullet covers a search for another shape `##arg-34`

Round 2, session. Bears on `thread@dispatch-judgement@design-investigation-dispatch`. Key words: "superseded by #dispatch-judgement-in-primer: "a search for another shape" is in the dispatch bullet, so the design skill needs no text"

### Parallel work is another reason to dispatch, and the carried-whole rule must not forbid the thaum process `##arg-35`

Round 3, owner. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "dispatching agent has another condition where it can be beneficial, in addition to the three I mentionned before: parallelizing work."; "Here, more than context, the goal is to also save time."; "In particular, **A dispatched piece is carried whole by its agent** no longer necessarily holds. As in the thaum split example."; "We should just make sure that our guidelines do not rule out a process such as wht happened in thaum with parallel subagents."

### How parallel work is partitioned is decided at design and planning, not during a task `##arg-36`

Round 3, owner. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "such kind of procedures are significant decisions about how the work is carried, and those are hard to come with "on the move", while carrying a task."; "maybe taking these kind of decisions about how the work is carried could be left to the design and planning steps."

### The brief, not a rule about commits, defines what belongs to a dispatched piece `##arg-37`

Round 3, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "The lead behind the clause is narrower than its wording. It says the session must not *interrupt* a dispatched agent to take over a task that belongs to the dispatched work, such as its commit, just because that task is small. What makes the commit "part of the dispatched work" is the brief, not a rule about commits."

### Under a partition between agents, which session judges the commit count is ambiguous `##arg-38`

Round 3, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "`design@agent-skills@step-is-no-commit` gives "how many commits it takes" to "the implementing session". Under a plan that partitions the work between agents, it is ambiguous which session that is."

### A shared tree is safe only while partitions touch disjoint files `##arg-39`

Round 3, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "That is safe only while their partitions touch disjoint files. A worktree per agent removes the condition, at the cost of a merge."

### Parallelism gets no sentence for lack of evidence, and nothing in the draft forbids it `##arg-40`

Round 3, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "It does not show a session that failed to parallelize when it should have. So a sentence would predict a behaviour"; "the bullets say when to dispatch, not "only when""; "Thaum's split also met the dispatch bullet on its own terms: a mechanical change, briefable, where only the result matters."

### A deferral needs a home a later session reads `##arg-41`

Round 3, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "The deferral needs a home that a later session reads, because a thread parked in this conversation dies with it."

### Checked once, as written, collides with the check chains the installed text builds `##arg-42`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Read as written, "a conclusion that comes back is checked once … a check is not checked again" reaches every check chain the installed text already builds on purpose"; "it would also bring back two `rejected` designs of the audit"; "scope the sentence to dispatches that the session judges, and leave every dispatch an installed skill names under its own head's argument. Those chains have measured reasons, such as verifiers that wave findings through". (Proposed first by searcher agent-4, L311.)

### A session acting on another's check has not observed the run, so the report must carry the evidence `##arg-43`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "A session that acts on a verifier agent's check has not observed the run."; "Its condition is that **the report carries the evidence**"; "Extending that head to every returned conclusion changes it, so the change is argued here and recorded as a change".

### Two paragraphs lack real-use evidence and would be deferred `##arg-44`

Round 4, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@dispatch-report-standard`. Key words: "two parts of the draft lack real-use evidence"; "**Default if you have no instance:** both parts leave the primer and become `deferred` issues". (Q1, withdrawn in round 5.)

### The primer dilutes in a long session, answered by one pointer at the moment implementation starts `##arg-45`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "*Dilution.* `rejected@agent-skills@head-rules-whole-in-primer` records that the primer "can dilute in a long session". The case you saw fail, implementation, starts at a known moment"; "**one sentence there, with its pointer to the primer section**". (Raised by searcher agent-3, L251.)

### The section is a weighing beside the harness's default, not a precedence over it `##arg-46`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The head states the section as a weighing beside the harness's default, not a precedence over it."

### The reload is measured at about 36,000 tokens `##arg-47`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "A fresh agent's fixed start is about 36,000 tokens"; "The head cites it as the ground of "carry when the brief costs as much as the work"."

### Agents dispatched together need distinct scratch directories, shown in real use `##arg-48`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "*Scratch directories.* `design@agent-skills@a-scratch-directory-per-subagent` states a general rule, but only the review skill and the searcher deliver it."; "Its evidence is real use: parallel searchers overwrote each other's files."

### Every existing absolute dispatch fits a ground, and the rule lives in a head `##arg-49`

Round 4, session. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "Every existing absolute dispatch still fits"; "if it bound a project's own skills, it would belong in the admission standard; if it binds only this repository's maintainers, in a head and a `%%` line. My default is **the head only**."

### The 250k figure grounds the search's bound, not a dispatch guideline `##arg-50`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-size-hint`. Key words: "The 250k is an argument the search's bound stands on, not a guideline a dispatcher can apply. So no number goes in the primer".

### A standing question must name what it watches, be answerable from the session, and concern most sessions `##arg-51`

Round 4, session. Bears on `thread@dispatch-judgement@retrospective-dispatch-questions`. Key words: "Adding questions widens it, and its argument covers each question only if the question names what it watches."; "So my compaction question is unanswerable as written."; "every project's retrospective answers each standing question, so a question must concern what most sessions meet."

### A dispatched developer loads the development skill through the primer and reads its own code's record `##arg-52`

Round 4, session. Bears on `thread@dispatch-judgement@development-dispatch`. Key words: "A dispatched developer loads `skill@knowledge-architect-development` through the primer's list of installed skills, not through its brief, as `design@agent-skills@audit-agent-per-axis` requires. It reads its own code's record, per `design@agent-skills@code-change-reads-its-record`."

### A parked thread takes a deferred issue, since only a trigger is read by a review `##arg-53`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-report-file`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "Both are opened as `deferred` issues with a trigger, since only a trigger is read by a review."

### The partition issue carries the facts the search found `##arg-54`

Round 4, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "the cold build of a fresh worktree"; "parallel target directories that filled a shared tmpfs"; "the content-loss risk of a shared tree"; "that a durable per-agent worktree is not a scratch worktree"; "that `skill@knowledge-architect-review@review-invariants` gives a worktree only to a subagent that judges a commit." (From searchers agent-2, agent-13, agent-15, agent-6.)

### The source issue closes, a changelog entry is owed, and the harvest splits the heads `##arg-55`

Round 4, session. Bears on no thread: bookkeeping. Key words: "The source issue closes with the work: its clause accepts "refused on its argument"."; "The branch writes a Workflow entry in the changelog, classed as a patch."; "The harvest splits the decision into one head per decision, per `design@agent-skills@one-decision-per-head`. A part stated by one text alone earns no head and keeps its reason in a `%%` line."

### The goal is read as settled by judgement with named exceptions `##arg-56`

Round 4, session, in its "Decided" section. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the goal leaves when, and for what size of work, a dispatch happens to design. I read it as settled by judgement with named exceptions. The rival reading, a threshold, is defeated by `design@agent-skills@capabilities-not-structure`."

### A check is another agent than the reporter verifying the claim, as applied until now `##arg-57`

Round 5, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "I don't think we should work around this in our additions. Instead, we should make it clear what a "check" means. To me, what was meant by a check, is another agent than the one reporting an information verifies the claim. This can be the main session, or a verifier. For review repairs, this can be a dispatched repairer, tasked with both verifying findings, and applying repairs"; "I think that this is pretty much the general principle that had been applied until now, no ?"

### A defect provable by reading is admitted without real-use evidence `##arg-58`

Round 5, owner. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Those obviously did not happen, because the instructions that could trigger them did not exist yet. A defect that is provable by reading the instructions is admitted without real use evidence."; "If we defer those, we have to make sure the text is not making those happen."

### The ceiling wording caused the collisions, and the definition removes the exemption `##arg-59`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "The clause that did the harm was "that check is not checked again". It reads as a ceiling, so it collided with every skill that asks for more"; "Your definition removes the need for an exemption."

### The record holds the principle as a minimum `##arg-60`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**Review:** … That is one check, by an agent other than the reporter."; "`--check` and `--verify` are commands that check the report's form and quotations. They are no check in your sense."; "That sample checks the verifier itself, and three tripwires guard it."; "So the record holds your principle as a minimum. Where it goes beyond one check, a skill states its reason, and no skill builds an open-ended chain. The rule therefore needs a minimum, not a maximum".

### Each clause of the check paragraph closes a two-act reading `##arg-61`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**"One check is enough", not "a check is not checked again".** It ends the chain, because no rule asks for a second check. It forbids nothing"; "**"It stands for the session's own verification".** It answers `design@agent-skills@verification-in-primer`."; "**"A command that checks a report's form is no such check".** Without it, a session could take `worklist --verify` as the one check and skip reading the verdicts."

### The Carry bullet, read at the size of one step, produces the takeover `##arg-62`

Round 5, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "The draft's own **Carry** bullet says to carry a piece "when the brief would cost about as much as the work". Applied to the commit step of a dispatched change, taken alone, the bullet says carry it."; "So our addition, read at the size of one step, produces the takeover. The paragraph fixes the unit at the brief's piece. It is admitted on reading."

### Re-verify before acting gives two acts on a verifier's report `##arg-63`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**"Checked once".** This is the two-act reading of "re-verify before acting" shown above. It is admitted on reading too."

### The premortem's causes and outcomes `##arg-64`

Round 5, session, at the premortem. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@dispatch-report-standard`, `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@parallel-work-partition`. Key words: Cause 1 "sessions still carry the implementation themselves: the primer dilutes, and the harness default wins" (watched by retrospective question 1); cause 2 "sessions dispatch lookups, and each pays about 36,000 tokens to reload" ("dropped: only you see usage, which fails condition 2"); cause 3 "a verifier refutes a true finding, and one check lets it pass unseen"; cause 4 "an agent returns its piece unfinished, and "the session does not take over a part of it" is read as forbidding the session to finish it" ("**changed in the design**: one clause, "what an agent returns unfinished is a new piece, for the session to judge""); cause 5 "agents dispatched together edit one tree and lose each other's uncommitted work".

### No acceptance criterion, since the work has no built code to observe `##arg-65`

Round 5, session. Bears on no thread: acceptance criteria. Key words: "**Acceptance criteria: none.** The work is text, and nothing in it is built code whose observation could show a decision wrong."

### The principle becomes a general head, a definition of sufficient verification `##arg-66`

Round 6, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "The principle I gave should be made a general design principle, with individual instances citing it. The bound on checks is a minimum, a definition of what is "sufficient verification"."

### Self-verification lapses often, hence a separate verification `##arg-67`

Round 6, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "An agent is still asked to verify its own claims before writing them, by the primer instructions, whether it is the main session or not. But this instruction is not sufficiently reliable, because it is the kind of instruction were a lapse is very common. Hence, we ask for a separate verification from a session that did not write the claims."

### Verification may be dispatched, but where its process serves the repair, one carrier takes both `##arg-68`

Round 6, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Dispatching for the verification is fully admitable, mostly because in most cases the verification process is not useful (such as code defect findings). However, for some cases, such as review findings, it is likely that the verification process is actually useful for the repair."; "either the session carries both verification and repairs, or it dispatches an agent that does both verification and repairs, if the scope is too large (which reports what it acted on or not and why, and this report gets a verification of its claims too)."

### The check principle earns a head by test 2 `##arg-69`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "It earns a head by test 2: the same reason is respected at several sites, namely the review's check of findings, the record search's verdicts, the audit's verifiers, the plan reviews, and now the primer section."

### The lapse rate of self-verification is measured in this session `##arg-70`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Every one of the 18 reported that its first `--check` failed"; "Two limits on this evidence: the failures are about exact quotation, not about the truth of a claim, and an agent may have relied on the cheap check on purpose rather than lapsed."; "dispatchers saved condensed copies, or none, in 7 of 12 sessions against an explicit instruction"; the transcript reviewer "exists to catch that expected rate".
 Corrected at assembly: 16 reports state that the first check failed; for agent-4 and agent-8 a first failure is inferred from their re-runs.
### The chain ends: each claim gets one check, and the main session's claims are checked at the merge `##arg-71`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**The chain still ends.** Each claim gets one check by a session other than its author. A repairer's report holds *new* claims"; "That is not a check of a check. The main session's own claims are checked by the review before the merge."

### The minimum binds a claim the work acts on; a refutation may be sampled `##arg-72`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "`design@agent-skills@one-run-pass` has the session read "a sample of the refuted drafts", not each one."; "**A wrong confirmation** produces a wrong edit, which lands."; "**A wrong refutation** loses a finding, which nothing applies."; "**the minimum binds a claim the work acts on. A claim whose effect is that nothing is done, such as a refuted finding, may be sampled where a skill says so, with a watch on what is lost.**"; put to the owner as a fork.

### Verifying and repairing go to one carrier, as a bullet of the judgement `##arg-73`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "Applied to review findings, that sends verification to one agent and the repair to another, so the repairer reads everything again. The **Carry** bullet covers only the session, not two agents."; the **Keep together** bullet; "The bullet is admitted on reading as a two-act repair".

### The refinements fall within subjects already searched `##arg-74`

Round 6, session. Bears on no thread: process. Key words: "These are refinements of #dispatch-report-standard and #dispatch-judgement-in-primer, and both subjects were searched."; "So no new search is owed."

### Lapses are normal, and the separate check catches what remains `##arg-75`

Round 7, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "In practice, this instruction has a lot of lapses, and this is totally normal to me"; "Overall I think it helps reduce the amount of wrong claims, and the separate verification takes care of what's left."

### A repairer's reason for not acting may go unchecked, and both reasons are alike `##arg-76`

Round 7, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "This matters most when the claims is acted on. Though this might leave a loophole in review findings, where a repaired that did not apply a repair to a claim, might not have its reason checked."; "If the reason is deciding the finding is real but not worth acting on, maybe that deserves verification. Or maybe not. Both cases are pretty much the same here actually."

### The guidelines considered pieces against the main session only, not against each other `##arg-77`

Round 7, owner, in the second part of the message. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The problem was that our draft guidelines were only considering dispatched tasks relative to the main session, but not dispatched tasks relative to each other. Maybe we should cover this case a bit more largely ?"

### A refutation keeps its reproduction, and a not-worth decision reaches the owner under Decided `##arg-78`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**"The finding is wrong."** This is a refutation."; "**"Real, but not worth acting on."** This is a decision, one the owner could make. The primer's `primer@decided-section` makes a subagent report it under "Decided""; "It is not unseen. It is merely unchecked by a third session, and that is all the boundary allows."

### Two relations between pieces govern any pair: one carrier, or separate carriers `##arg-79`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The check principle is the same kind of question, about two pieces that must *not* share a carrier"; "**One carrier** when they reason over the same details"; "**Separate carriers** when one judges the other, as a review or a check judges what it reads; or when they are independent and the split saves context."; "The session-or-agent question then becomes the special case of the first piece the session holds."

### Writing a change and its commit is the example of one carrier `##arg-80`

Round 7, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "**"Writing a change and its commit"** is now the example of one carrier. That makes the "carried whole" rule follow from the first bullet. The "carried whole" paragraph stays, to fix the unit at the brief's piece."

### Independence is a condition for a split, not time a reason `##arg-81`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "**Independence is stated as a condition for a split, not time as a reason.** Your thaum case meets it."

### A relation, not a list of cases, judges a pairing nobody anticipated `##arg-82`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**The section is built on a relation, not a list of cases.** So a pairing nobody anticipated, such as an investigation and the fix it leads to, is judged by the same two bullets with no new text. That is `goal@only-needed-instructions`."

## New names, in one place

```text
dispatching-work                                 the primer section's slug, content/PRIMER.md
work-divided-by-judgement                        a design head, the agent-skills design home
one-check-by-another-session                     a design head, the agent-skills design home
dispatched-agents-write-their-report-as-they-work   a deferred issue, crates/agent-skills/docs/open-issues/
how-parallel-work-is-partitioned-among-agents    a deferred issue, crates/agent-skills/docs/open-issues/
dispatching.md                                   a subdocument of the agent-skills design home, holding the two heads
```

## Decided design

### `thread@dispatch-judgement@dispatch-judgement-in-primer`, with `thread@dispatch-judgement@absolute-dispatch-conditions`, `thread@dispatch-judgement@dispatched-piece-carried-whole` and `thread@dispatch-judgement@dispatch-report-standard`

A new primer section, placed after `primer@decided-section`. An illustration of its shape, its
wording the step's own; the `%%` lines carry the reasons:

```markdown
## Dispatching work {{slug:dispatching-work}}

**A session that can dispatch agents judges how the work is divided between itself and the agents
it dispatches.** No instruction fixes a size or a kind of work for this, except the dispatches an
installed skill names, each for a step that needs a point of view apart from the session's, an
output a check verifies, or an occasion most work meets.

- **One carrier**, the session or one agent, takes two pieces that reason over the same details, so
  that none reads again what another read: verifying findings and repairing them, or writing a
  change and its commit.
- **Separate carriers** take two pieces when one judges the other, as a review or a check judges
  what it reads; or when they are independent and splitting them saves context.
- **The session dispatches** a piece when a brief can carry what it needs of the session's context
  and the session needs only its result, not the reading and the trials behind it, as an
  investigation, a survey or a mechanical change.
- **The session carries** a piece when it will reason later over the details the piece reads, or
  when the brief would cost about as much as the work: a fresh agent first reloads its
  instructions.
- **Between the two**, as for implementing a design, it weighs what the brief leaves out against
  the context carrying the piece spends. The main session's context holds the exchange with the
  owner, which no brief rebuilds, while an agent's is rebuilt by dispatching it again; and every
  later turn costs more as the context grows. A plan document carries most of what an implementer
  needs.

**A dispatched piece is what its brief gives the agent, and the agent carries all of it**: the
session does not take over a part of it while it is dispatched, such as its commit because that
part alone is small. What an agent returns unfinished is a new piece. An agent reports what it acted
on and what it did not, with why. Agents dispatched together each get a scratch directory of their
own, as `skill@knowledge-architect-review@review-invariants` says.

**A conclusion that comes back is checked before the session acts on it**: an agent other than the
one that reported it, the session or one it dispatches, verifies its claims against the evidence
the report carries; a command that checks a report's form is no such check. One check is enough:
it stands for the session's own verification of that conclusion, per `primer@verify-before-relying`.
```

The argument, by part:

- **A judgement, not a threshold.** Size varies without bound, and a threshold is a rule per
  interaction, against `design@agent-skills@capabilities-not-structure`, whose measurement is 34
  edits producing 10 interaction defects in one revision of designing-together. A relation between
  pieces, not a list of cases, judges a pairing nobody anticipated. The nearest rival, no text and
  `primer@room-to-judge` alone, lost to the owner's real-use report and to the harness's default,
  which leans against dispatching; the section is a weighing beside that default, not a precedence
  over it.
- **The primer as its home.** A session meets the question in every activity, and no skill loads at
  that moment, per `design@agent-skills@primer-limit`. The primer "can dilute in a long session",
  per `rejected@agent-skills@head-rules-whole-in-primer`: the case the owner saw fail,
  implementation, starts at point 3 of the planning skill's procedure, which gets one sentence and
  its pointer to the section, per `design@agent-skills@restatement-size-test`. The rival, guidance
  in each activity's skill, restates the judgement in each and reaches only the sessions that load
  it.
- **The absolute dispatches.** An installed text fixes a dispatch only on a property of the step that
  holds whatever the task's size: a separate point of view (the reviews, the goal reviewer), a
  checked output (the record search through `worklist --verify`, the audits through their
  verifiers, the transcript extraction through the transcript reviewer), or an occasion most work
  meets (a merge, a development checkpoint). Any other "always dispatch X" is a size claim in
  disguise. The rule binds what the installed text states, so it lives in a head and its `%%`
  lines, not in the admission standard.
- **Carried whole.** The Carry bullet, read at the size of one step, says the session carries a
  commit alone; the harness's "Do the work yourself when it is a handful of tool calls" says the
  same. The paragraph fixes the unit at the brief's piece, which admits the thaum process, where the
  briefs said not to commit and the session committed the parts after.
- **The check.** The primer asks every session to verify its own claims before writing them, and
  the instruction lapses often: of the 18 record searchers of this discussion, each told to quote
  verbatim, 16 reported that their first `worklist --check` failed on their quotations, and agent-4
  and agent-8 re-ran theirs after repairs, a first failure inferred; dispatchers saved condensed
  copies of reports, or none, in 7 of 12 sessions, per `design@agent-skills@reviewer-writes-its-report`.
  So a claim the work acts on gets one check by a session other than its author. One check is a
  minimum, not a ceiling: it forbids no second check a skill asks for, such as the audit's sample of
  a verifier's refutations. A claim whose effect is that nothing is done, a refuted finding, may be
  sampled where a skill says so; a finding judged real and not worth acting on is a decision, which
  `primer@decided-section` brings to the owner. Each claim gets its one check, and the main
  session's own are checked by the review before the merge, so no chain is open.

### `thread@dispatch-judgement@retrospective-dispatch-questions`

Three standing questions, after the existing ones in the retrospective skill. An illustration of
their shape:

```markdown
- Did this session carry work an agent could have carried, spending context its design or its
  exchange with the owner needed? For each: the work, and what it cost.
- Did a dispatched agent return without finishing its piece, or with a report the session could not
  act on without redoing the work? For each: the brief, and what was missing.
- Did this session act on a returned conclusion that proved wrong, or check one conclusion more than
  once? For each: the conclusion, and who checked it.
```

Each watches a decision whose failure shows in real sessions first, per
`design@agent-skills@premortem-as-watch-points`: the first `work-divided-by-judgement`, the second
the division of pieces and the evidence `dispatched-agents-write-their-report-as-they-work` waits
for, the third `one-check-by-another-session`. A session sees an agent's brief and report, not its
context, so no question asks about an agent's context.

### The pointer at implementation's start

One sentence at point 3 of `skill@knowledge-architect-planning@working-a-slice`, an illustration:
"Who carries the implementation, the session or agents it dispatches, is judged as the
primer's section on dispatching work says."

## Mapping tables

The work needs none.

## Losing alternatives

- **No text, relying on `primer@room-to-judge`**, lost to `thread@dispatch-judgement@dispatch-judgement-in-primer`:
  the owner's real-use report shows the judgement failing, and the harness's default leans against
  dispatching.
- **Guidance in each activity's skill**, lost to the same thread: the judgement is met in every
  activity, and no skill loads at that moment.
- **Absolute thresholds of size or kind**, lost to the same thread: size varies without bound, per
  `design@agent-skills@capabilities-not-structure`.
- **A token bound in the primer**, from `thread@dispatch-judgement@dispatch-size-hint`, lost to
  `thread@dispatch-judgement@retrospective-dispatch-questions`: the figure a session sees per agent
  has an unestablished meaning, context windows differ by model, and the lesson serves the next
  session.
- **"Carried whole, commits included, never split between two authors"**, the first wording of
  `thread@dispatch-judgement@dispatched-piece-carried-whole`, lost to its rewording: it forbade the
  thaum process.
- **"A check is not checked again", scoped to dispatches no skill names**, the wording of rounds 2
  and 4 of `thread@dispatch-judgement@dispatch-report-standard`, lost to the minimum: as a ceiling
  it collided with six heads and needed an exemption.
- **Every refutation checked**, lost to the boundary of `thread@dispatch-judgement@dispatch-report-standard`:
  a wrong refutation loses a finding nothing applies, which the audit's next pass and
  `tripwire@agent-skills@verifier-refutes-a-true-finding` watch, where a wrong confirmation lands an
  edit.
- **A capability sentence in the design skill**, `thread@dispatch-judgement@design-investigation-dispatch`,
  superseded: the section's dispatch bullet covers it.
- **A compaction question in the retrospective**, lost: a session cannot observe an agent's context,
  per `design@agent-skills@premortem-as-watch-points`.

## Readings

The work reads no external specification. It reads the behaviour of an external tool, Claude Code:

- its Agent tool description, "Do the work yourself when it is a handful of tool calls … When in
  doubt, don't spawn.", read in this session's tool list; recorded in `work-divided-by-judgement`;
- a general-purpose subagent holds `Agent` and no `Workflow`, and the primer reaches a nested agent:
  measured once each in this session; recorded in `work-divided-by-judgement`, with the probe that
  takes each again: one general-purpose agent asked to list its tools and to dispatch one nested
  agent that quotes a sentence of the primer from its initial context.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| sessions still carry the implementation themselves: the primer dilutes, and the harness's default wins | `thread@dispatch-judgement@dispatch-judgement-in-primer` | converted: the pointer sentence at point 3 of the planning skill's procedure; and watched by the first retrospective question |
| sessions dispatch lookups, each paying about 36,000 tokens to reload | `thread@dispatch-judgement@dispatch-judgement-in-primer` | dropped: only the owner sees usage, which fails condition 2 of the standard, and the owner would see the failure |
| a verifier refutes a true finding, and one check lets it pass unseen | `thread@dispatch-judgement@dispatch-report-standard` | in the audit, guarded by `tripwire@agent-skills@verifier-refutes-a-true-finding`; elsewhere, named as a cause for the harvest to judge, watched by the third retrospective question |
| an agent returns its piece unfinished, and "does not take over a part of it" is read as forbidding the session to finish it | `thread@dispatch-judgement@dispatched-piece-carried-whole` | converted into a clause: "What an agent returns unfinished is a new piece." |
| agents dispatched together edit one tree and lose each other's uncommitted work | `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@parallel-work-partition` | carried into the issue `how-parallel-work-is-partitioned-among-agents` |

## Acceptance criteria

None: the work is text, and nothing in it is built code whose observation could show a decision
wrong.

## Implementation sequence

### Step 1: the primer section and the pointer

Builds the section `dispatching-work` in the primer, after `primer@decided-section`, with its `%%`
lines; the pointer sentence at point 3 of `skill@knowledge-architect-planning@working-a-slice`; the
install; a line for the coined term "carrier" in `primer@coined-terms` if the section uses it as a
term other installed texts use, per `design@agent-skills@term-defined-at-first-load`.

Claims:

- every absolute dispatch the installed text states fits one of the three grounds: tested by reading
  every installed skill and agent for a fixed dispatch, a phrase search being a lead only;
- no installed instruction and the section leave a session without a move that obeys both, the
  audit's verify-then-sample and the review's "Verify each against the tree" included: tested by
  reading each check site the section "What the work is" lists against the check paragraph;
- `cargo klarch check` passes on the installed copies.

### Step 2: the retrospective questions

Builds the three standing questions in the retrospective skill, after the existing ones, with a
`%%` line naming what each watches; the install.

Claims: each question is answerable from what a session holds, its own transcript, the briefs it
sent and the reports it received: tested by reading each against
`design@agent-skills@premortem-as-watch-points`.

### Step 3: the harvest

Per the row below.

## Order rationale

- Step 1 before step 2: the questions watch the decisions the section states.
- Step 2 before step 3: the harvest records the questions as members of the watch head.

## Defaults awaiting the owner

None.

## Harvest

| what | where | when |
| --- | --- | --- |
| the head `work-divided-by-judgement` (pair: `thread@dispatch-judgement@dispatch-judgement-in-primer` with `thread@dispatch-judgement@absolute-dispatch-conditions` → `work-divided-by-judgement`), test 3 on the harness's default, test 2 on the absolute dispatches at several sites | a new subdocument, `planned@agent-skills@docs/design/dispatching.md`, linked from the README of `register@agent-skills@design` | step 3 |
| the head `one-check-by-another-session` (pair: `thread@dispatch-judgement@dispatch-report-standard` → `one-check-by-another-session`), test 2: the review, the planning skill's findings, the record search, the audit and the primer check claims | the same subdocument | step 3 |
| `design@agent-skills@session-acts-on-the-verdicts` rewritten to cite `one-check-by-another-session`; `design@agent-skills@primer-limit` gains the section as a member; `design@agent-skills@premortem-as-watch-points` gains the three questions | their homes | step 3 |
| the losing alternatives, each judged by `skill@knowledge-architect-decision-recording` for a `rejected` entry | `register@agent-skills@rejected` | step 3 |
| the premortem's third cause, judged by the tripwire standard | `register@agent-skills@tripwire`, or dropped with its reason | step 3 |
| the issue `dispatched-agents-write-their-report-as-they-work`, kind `deferred`, trigger: a retrospective answer to the second dispatch question naming a report the session could not act on; it cites `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript` and `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`, and answers `rejected@agent-skills@ledger-written-during-discussion` and `design@agent-skills@refused-report-returned-as-text` | `register@agent-skills@issue` | step 3 |
| the issue `how-parallel-work-is-partitioned-among-agents`, kind `deferred`, trigger: a plan document whose work wants parallel agents; it carries the cold build of a fresh worktree (`rejected@mutate@worktree-holds-the-mutation`), parallel target directories filling a shared tmpfs (`issue@mutate@an-environment-failure-reads-as-caught`), the content-loss risk of a shared tree (`design@knowledge-architect@no-operation-loses-content`), that a durable per-agent worktree is not a scratch one (`design@knowledge-architect@scratch-worktree-outside-git-rules`), that `skill@knowledge-architect-review@review-invariants` gives a worktree only to a subagent that judges a commit, the ambiguity `design@agent-skills@step-is-no-commit` meets under a partition, and `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`; it cites `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session` and `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand` | `register@agent-skills@issue` | step 3 |
| a Workflow entry, a patch, in `Next release`, then `cargo x changelog` | CHANGELOG.md | step 3 |

## Later consequences

- The later discussion on partitioning parallel work starts from
  `how-parallel-work-is-partitioned-among-agents`.
- The owner aligns the user-level instructions on subagents with the section, outside the project.
