# Agent skills design: Goals

## Goals

### A goal stays while it is met, and leaves only on the owner's word `##goal-lifecycle`

A goal is met or unmet, and stays in its goals home while it is met. The rule and its reason are the
owner's, in the root goals home as the owner adopted it: "A goal removed when it is achieved stops
being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word." A goal leaves only when the owner abandons it, per
`goal@knowledge-architect@the-owner-decides`.

### A goal entry states an outcome, what would show it met, and "for instance" cases of each layer it has, in a body with no bound on its length `##goal-entry-shape`

A goal is a heading stating an outcome as a sentence, with its slug, at the level the core gives the
goal register, per `design@core@an-entry-is-a-heading-at-the-register-level`, and a body saying what
it means, what would show it is met, and, for each layer it has, what it requires, what it admits and
what it rules out, one or more "for instance" cases. What it may state is the goal standard's, per
`design@agent-skills@goal-standard-rules`; its first rule carries the outcome test. The body
has no bound on its length, and names the members of a set in a list where it has several.

The cases are those of the boundary block the owner's ruling kept, per
`design@agent-skills@goal-draft-shows-reach`. The block leaves with the ruling, so a later agent
reads the goal alone; the owner proposed that example lines per layer written into the goal could
carry the reading the owner ruled on better. A case shows a reading and binds nothing, since a case taken as the
measure narrows the outcome to what was foreseen, the defect rule 6 names. Their count is one or more
for each layer the goal has, not one per layer, since
`goal@agent-skills@only-needed-instructions` refuses a fixed count that another instruction needs
to vary, and a goal that rules nothing out has no case to show there.

A session that argues from a goal applies the goal whole, so the words it reads cost that session
nothing, by the primer's rule on the size cost of agent-facing text; a goal pressed into few
sentences carries its sets and its measure in long ones, and long sentences read two ways, against
the goal standard's rule of one reading. The owner made this argument: the standard needs more than
four sentences to state what matters, the bound probably makes one reading harder to keep, and the
owner was fairly certain that a list makes some goals more readable. The measurement
behind it was taken on the drafts of this repository's 31 goals written under a bound of two to four
sentences, as the review record added by the commit "[agent-skills] A goal's body has no sentence
bound, and the whole-set review runs in rounds the owner answers" holds them: 9 goals broke the
bound, and 51 of the 129 sentences of the 31 goals ran over 40 words. The script that takes it again
is in the message of the commit "[agent-skills] Repairs of the review before the merge of the
goal-standard branch". The nearest rival, a soft bound such as "as short as the standard allows",
gives a reviewer nothing to apply, and keeps the pressure that produced the long sentences. Its
rival with a fixed bound is `rejected@agent-skills@goal-body-in-two-to-four-sentences`.

### Goals change only through the goal-setting skill `##goals-change-through-goal-setting`

The goal-setting skill runs at setup for every Component, when the owner states, rewords or abandons
a purpose, when a decision conflicts with a goal and the primer's rule sends the conflict to the
owner, and for the review of the whole goal set, on the owner's request or at a pin move across a
version whose Migration entry names it, per `design@agent-skills@goal-set-review-in-goal-setting`.
An agent never edits a goal outside it, since a goal binds outright where a decision binds as a
presumption, per `goal@knowledge-architect@the-owner-decides`.

### A proper goal meets the rules of the goal standard, written once as a fragment that goal-setting and the goal reviewer host `##goal-standard-rules`

The goal standard is `path@agent-skills@fragments/goal-standard.md`. It reads a goal in three
layers, what it requires, what it admits and what it rules out, and as a direction agents move the
project toward; a new or reworded goal is expected to be unmet. Its rules: a goal states an outcome
the owner intends that keeps its meaning in the project emptied of its design and its code; it has
one reading, which judges ambiguity and never breadth; it requires and rules out nothing the owner
has not required or ruled out knowingly; it holds one intent; it contradicts no other goal, a
contradiction being one goal ruling out what another requires; what shows it met is in what the
project builds, sorted by whether the outcome depends on what the project does not control; a
refinement neither widens nor contradicts the goal it refines, and a goals home cites only goals
and registers of its own or an enclosing Component, per `design@core@goals-home-cites-upward-only`;
and a statement of what a Component is not for is a goal only when it names the design it excludes.
Each rule's case, read in the goal histories of rootpoint, thaum and this repository, is a `%%`
comment beside it, except rule 7's first half, of which no case was observed: it follows from the
rule of no contradiction. The drafter writes to it and the reviewer judges by it: goal-setting hosts
it in `skill@knowledge-architect-goal-setting@goal-or-not`, and the goal reviewer in
`agent@knowledge-architect-goal-reviewer@the-goal-standard`, per
`design@agent-skills@shared-text-fragment`. The primer hosts none of it, since only a drafter and a
judge of a goal apply it. The findings from other fields it leans on are
`path@agent-skills@docs/grounding/goal-standard.md`.

The goals bind outright and are the only statements assumed to come from the owner, per
`goal@knowledge-architect@the-owner-decides`, and agents work toward them over a long time, per
`goal@knowledge-architect@agents-work-without-drift`. So a goal must not rule out what the owner
never ruled out, since every later agent applies it at its widest reading, and must not change as
the design under it changes, since only the owner changes it. In rootpoint's goal history, 9
changes over 7 commits, which `git log --follow -p -- docs/goals.md` in rootpoint lists, the
drafting step caught no defect: reviews or the session caught 4, the owner 3, a design discussion 2;
in 3 of its 4 cases of excess reach the excess was a conflict with a sibling goal.

A standard whose every rule is repaired by adding words narrows the goals it reviews: asking each
universal to name its set adds a scope, a rule against an unknowing exclusion adds an exception,
and a measure in the tree pulls toward the most specific observable. A narrowing repair rules out
cases the owner never ruled out, which the next round reports, so the rounds do not converge, as the
91 then 88 findings of `design@agent-skills@whole-set-rounds` showed. The owner argued the rules
that answer it, and the session shaped them:

- **Rule 1's skeleton test.** A goal that names a part as built binds the part, so a redesign
  rewrites goals only the owner may change; the owner argued that a goal keeps its sense in a
  project emptied of its design and its implementation, since goals are set before both. The
  sources the grounding document reads place stability in a goal's level, a requirement being one
  way of achieving a goal. A Component's goal may name its own and enclosing Components, since
  changing the shape there changes what the goal states anyway. Rule 1 also reads the outcome
  test against the owner's intent: an owner who intends a command line makes a Component without
  one for something else, so a kind of interface the owner intends to provide is an outcome, and
  its exact form is a decision, which an interface others consume makes a head.
- **Rule 2 judges ambiguity, never breadth**, on the owner's argument that a goal has no ambiguity
  without being narrowed: a goal reads two ways when two agents in one situation would move in
  different directions, and a wide direction admits many shapes in one reading.
- **Rules 3 and 5 read three layers**, on the owner's argument that admitting is not requiring: a
  goal that rules out what another only admits leaves a state that meets both, which rule 5's
  sentence "No state of the project meets one goal only by failing another" already held. Rule 3
  allows the distillation goal-setting is for, a goal wider than each statement it is drawn from:
  the defect it prevents is a reach the owner never saw, so the rival "no wider than the owner's
  statement" forbids what the skill exists to do.
- **A goal is drafted to move the tree**, on the owner's argument that a new goal being unmet is the
  point: a drafter that shapes a goal so the tree meets it states the project as it is.
- **Rule 6's test**, whether the outcome depends on what the project does not control, sorts an
  outcome measured by a check of what the project builds from one judged only in use, which the
  owner's goals measured through reports from use exemplify. For an outcome judged only in use,
  what the project builds is the mechanism that pursues it and learns from its use whether it is
  met, never a list of the properties the outcome is taken to consist of: a drafter held to one
  reading turns such an outcome into the most specific observable it can find, and the list then
  binds every agent to what was foreseen before any use. The measure is what the project builds, on
  the owner's argument that in most projects a measure on use has limited feasibility. The rival,
  evidence from use itself as the measure where it can be gathered,
  `rejected@agent-skills@goal-measured-by-evidence-from-use`, lost on that premise, and the owner
  chose the mechanism as the reading that gives a systematic check. A measure kept apart from the
  goal and revised by agents, `rejected@agent-skills@goal-measure-revised-apart`, would move what
  shows a goal met out of the owner's statements.
- **Rule 8** admits an exclusion only where another goal would otherwise argue for the design it
  names: a property of smallness with no named design is the owner's scope control, exercised by
  what the owner asks for, or a quality argument a head makes against its own rival.
- **Rule 4** asks only that one goal not conflate two intents, and two goals may share an aspect. Its
  test is the head standard's rule on an exception, not the test of reversing one part alone, which
  splits a rule from its exception, per `rejected@agent-skills@head-split-by-independent-reversal`.

The nearest rival to hosting, the standard written in goal-setting alone with the reviewer briefed
to read it, puts the reviewer's standard behind a read the brief must ask for, which
`rejected@agent-skills@one-generic-auditor-agent` records as uneven.

### An agent that meets a goal reading two ways in use gets no instruction of its own `##no-instruction-for-a-goal-read-two-ways`

No installed text tells an agent what to do when a goal it applies reads two ways. A conflict
between a proposal and a goal already goes to the owner, per `primer@goals-bind`, which a case of
rootpoint's goal history showed working, and the goal reviewer reads a goal's readings before the
owner rules. The rival, an instruction for an agent applying a goal that reads two ways, adds a rule
for a case the routing already decides, against `goal@agent-skills@only-needed-instructions`.

### A goal draft shows each goal with its boundary block, its reach by layer, and the owner's statements go to the reviewer, not into the display `##goal-draft-shows-reach`

Goal-setting shows the owner each drafted goal with its boundary block: what the goal asks of
future work; for each layer it has, requires, admits and rules out, at least one case, and the cases
it reaches that the owner may not intend; each sibling goal it touches with the relation, a conflict
named as such; and each question of intent the reviewer raised, with the case on which the readings
differ. A contradiction is put at the top of the draft. The block is not written to the goals home;
the cases of it the owner's ruling keeps are, per `design@agent-skills@goal-entry-shape`. The owner's
statements are given to the reviewer whole and are not shown beside each goal, since an intent
spread over several statements is carried by the distilled goal, and a short quotation beside it
carries little of it and can misstate it.

The block turns a goal's reach into cases the owner judges by intent, since a prose goal is easy to
approve without seeing what it rules out, and the owner reported approving rootpoint's goals in a
rush, trusting proposals that read well. The owner argued that once a wording is approved, what it
rules out becomes the owner's word for every later agent, noticed or not, and that a goal's reach is
worth showing because it surfaces effects wider than the owner intended. So the reach is shown by
layer and put to the owner, and the drafter keeps the wording by default: a narrower wording is
drafted only on the owner's word, with what it rules out, since a narrowing the owner did not see
rules out cases the owner never ruled out. Whether a case was meant is a question of intent, so the
rival, the repaired draft shown alone with the reviewer as the only reader of reach, leaves the
reviewer a question it cannot rule on. It serves `goal@knowledge-architect@the-owner-decides` and
`goal@knowledge-architect@owner-attention-on-what-is-theirs`.

### A fresh goal reviewer reads every version of a goal draft before the owner rules, in a draft, a whole-set and a merge mode, and grades each finding by what it changes in an agent's act `##goal-reviewer-agent`

`agent@knowledge-architect-goal-reviewer` is read-only and hosts the goal standard. In draft mode,
goal-setting dispatches it before every ruling of the owner, on the whole batch of new and reworded
goals, every relevant statement of the owner verbatim whether or not the drafter drew on it, and
every goals home, and again on each repaired or reworded version, so the version shown is a version
reviewed. In whole-set mode it reads every goals home and gives every goal's boundary block
material, on which the owner judges rule 3. In merge mode it is the goals axis of the review before
a merge, per `design@agent-skills@goals-axis-in-the-review-before-a-merge`.

Each finding carries a route, `settled`, which the drafter repairs, `intent`, a question only the
owner answers, or `contradiction`: a statement of the owner that conflicts with a goal or with
another statement, a part of the statements a draft drops or turns around without showing it, or a
goal ruling out what another requires, which the drafter never resolves alone. Each also carries a
grade by what it changes in an agent's act, from a table in the agent's definition that covers every
finding of the standard of `design@agent-skills@goal-standard-rules`: `blocks`, a finding that
changes what agents do against the standard or against the owner's statements; `asks`, a reach in
what a goal requires or rules out, or a ranking, which the owner rules on; `minor`, an overlap or the
wording of a measure, which changes no agent's act. The owner argued that, with `minor` as the only grade,
more grades of gravity might help the reviews converge. A case a goal only admits, and
a goal not met yet, are no finding, and a finding on reach lists the cases and never proposes a
narrower wording, per `design@agent-skills@goal-draft-shows-reach`.

The draft-mode review stops, in goal-setting, when a round from the second on reports no `blocks`
finding tagged `settled` on old wording, the words the previous round read and no repair changed
since, and the findings of that round go to the owner unrepaired. The count reads old wording
because each round's repairs write new wording, in which the next round finds defects at least as
readily, per `design@agent-skills@convergence-count-scope`; the owner reported that the non-convergence came in part
from the corrections of earlier rounds, which had unforeseen effects applied together, and that a review of one goal or a few
converged more reliably. The rival, a stop on no `blocks` finding over all wording,
`rejected@agent-skills@goal-review-stop-over-all-wording`, is held open by the wording each round
writes. A stop counting the `settled` findings alone leaves the owner's questions out of the count,
since no round answers them. Each mode's own checks fire only on the input its brief carries, so one
fragment serves three modes.

It is fresh, never a fork, because the drafter is anchored on its own reading: in rootpoint and
thaum a draft's excess reach went unnoticed by the drafter and the owner alike, which is the nearest
rival, the drafter checking its own draft, refuted. It reads every statement of the owner, not only
those the drafter drew on, so a statement the drafter left out still reaches the check of a part
dropped. The owner reported, unsure, that a project's root goals refining no other goal stay in the
tens and that one agent reads at least 100 goals when it does only that; no rule splits a larger set
before evidence asks for one: the review of the whole goal set splits a set into clusters for each
reviewer's focus, per `design@agent-skills@whole-set-rounds`, not for one agent's reach. It serves
`goal@knowledge-architect@the-owner-decides`.

### A goal's wording proposed in another activity is ruled as its own item, never inside a batch approval of other items `##goal-edit-ruled-alone`

A goal's wording that a plan's default, a review's repair, an audit's item or a design discussion
proposes is drafted under `skill@knowledge-architect-goal-setting@drawing-out-intent` as its own
item, with its boundary block, and the owner's word on a batch of other items never covers it. The
planning skill's `skill@knowledge-architect-planning@plan-reviews`, the review skill's
`skill@knowledge-architect-review@what-review-leaves` and the design skill's
`skill@knowledge-architect-design@threads-and-states` each restate it in one sentence with the
pointer. In thaum, a goal reworded as a plan's default was approved in one word with the plan's
other defaults and set the goal against another; a review caught it only after the ruling. So the
rival, the review row alone, catches a batched goal edit only after the approval it rides on. It
serves `goal@knowledge-architect@the-owner-decides`.

### The review of the whole goal set runs inside goal-setting, by the goal reviewer, not as an axis of the project audit `##goal-set-review-in-goal-setting`

`skill@knowledge-architect-goal-setting@whole-set-review` reviews every goal of a project, on the
owner's request and at a pin move across a version whose Migration entry names it. It follows
goal-setting's methodology, a fresh reviewer and goals ruled by the owner, since every goal edit is
the owner's, so an audit's split between edits applied in its branch and items put to the owner has
nothing to split. The owner argued that an activity is its methodology, not the scope of what it
reads: two activities that follow different methodologies are two, and are not bundled in one skill,
per `goal@agent-skills@one-skill-per-activity`. The review shares means with the audit, rounds of
fresh reviewers, a committed record and a stop on a count, per
`design@agent-skills@whole-set-rounds`; its methodology stays goal-setting's because no edit lands
from the review itself: the owner rules on every goal before it is written, where an audit applies
its settled repairs in its own branch. So that goal rules out the rival, a fourth axis of the
project audit with its saved workflow, `rejected@agent-skills@goal-set-review-as-an-audit-axis`.

### The review of the whole goal set runs in rounds the owner answers, over a record committed in docs/goal-reviews/, and stops on the owner's word or at a fifth of the first round's count `##whole-set-rounds`

Each round dispatches together one goal reviewer per set of goals that touch each other, in draft
mode, and one in whole-set mode; the sets give each reviewer a cluster to read against every rule,
which is focus, not a bound on what one agent reads; the session writes the repairs and its defaults
in the goals homes of the working tree, and puts every question of intent and every contradiction to
the owner in one message, the next round waiting for the answer. The record, one file
`docs/goal-reviews/<YYYY-MM-DD>-<subject>.md`, holds each round's counts per reviewer, rule,
grade, route and old or new wording, outcomes and defaults, the
owner's answers verbatim and the boundary blocks; it is committed each round, every brief names it,
and the commit that writes the goals the owner ruled last deletes it. The rounds stop on the owner's
word, or once a round's count of `blocks` and `asks` findings, every reviewer together and under one
brief, over all wording in the first round and over old wording in each later one, falls to a fifth
of the first round's, the grades of `design@agent-skills@goal-reviewer-agent`; the round that meets
the stop writes no repair,
and its findings go to the owner beside the goals, so the goals the owner reads are the goals a
round reviewed. The owner then rules on the goals as the tree holds them, per
`design@agent-skills@goal-batch-ruled-in-the-tree`. The primer's knowledge table carries the
record's row for every project.

The owner proposed the procedure's six steps from what this repository's own review of its 31
goals showed. The session added the whole-set reviewer in every round, the boundary blocks in the
record, the open questions named in every brief from the first round on, and the rule that the
round meeting the stop writes no repair. The record of that review is in the history of its branch,
`git log --diff-filter=D --name-only -- docs/goal-reviews/`. Rounds the session ran on its own
defaults, with no answer of the owner between them, did not converge, 91 then 88 findings, and set
defaults against the owner's approved words, which only a later round caught. The counts fell only
once the briefs named the questions already open. The whole-set reviewer, run beside the set
reviewers, alone found two conflicts between goals of different homes. The record is committed
because the owner's words stay verbatim only in a deliberation record, and a session's scratch
space, the nearest rival, is lost to the next session. It is not the ledger
`rejected@agent-skills@ledger-written-during-discussion` rejects: a stale ledger of a discussion is
corroborated by the conversation alone, while each round's reviewers read this record against the
tree and report an open question it misstates, and the owner answers it each round. The set
reviewers found most of the findings, and the whole-set reviewer the conflicts across homes. The
session chose a fifth over a tenth, which no goal ranks, since the rounds of that review reached a
tenth only at their last; the owner approved it. The count reads old wording after the first
round, since each round's repairs write the wording the next round finds defects in, per
`design@agent-skills@convergence-count-scope`; the rival over all wording is
`rejected@agent-skills@goal-review-stop-over-all-wording`. It serves
`goal@knowledge-architect@the-owner-decides` and
`goal@knowledge-architect@owner-attention-on-what-is-theirs`.

### A goal-setting batch of more than three goals is drafted and ruled in the tree, three or fewer in the conversation `##goal-batch-ruled-in-the-tree`

A batch of three goals or fewer is drafted in the message and ruled by slug. A larger batch is
written in the goals homes of the working tree, uncommitted, its source marks, its boundary blocks
and the reviewer's findings in a record of `path@knowledge-architect@docs/goal-reviews/`, and the
owner rules
on the batch as the tree holds it, naming each goal dropped or reworded; the goals are committed as
the tree holds them. The owner argued that a larger batch is read more readily in the tree, and
found ruling by slug inconvenient for the 31 goals of this repository's review; the session
generalised it, ruling on each goal by slug costing the owner more than it checks for any larger
batch; the owner's word on the batch is
still an explicit word, so no goal is accepted by silence. The threshold is the session's reading of
the owner's bound of two or three goals, at its upper end. The rival, a tree draft still ruled slug
by slug, keeps the cost the
owner named and adds nothing that a goal named on the ruling does not give. It serves
`goal@knowledge-architect@owner-attention-on-what-is-theirs`, and is stated in
`skill@knowledge-architect-goal-setting@drawing-out-intent`,
`skill@knowledge-architect-goal-setting@whole-set-review` and the primer's knowledge table.
