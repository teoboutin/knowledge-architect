# Agent skills design: Goals

## Goals

### A goal stays while it is met, and leaves only on the owner's word `##goal-lifecycle`

A goal is met or unmet, and stays in its goals home while it is met. The rule and its reason are the
owner's, in the root goals home as the owner adopted it: "A goal removed when it is achieved stops
being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word." A goal leaves only when the owner abandons it, per
`goal@knowledge-architect@the-owner-decides`.

### A goal entry states an outcome, and what would show it met, in a body with no bound on its length `##goal-entry-shape`

A goal is a heading stating an outcome as a sentence, with its slug, at the level the core gives the
goal register, per `design@core@an-entry-is-a-heading-at-the-register-level`, and a body saying what
it means and what would show it is met. What it may state is the goal standard's, per
`design@agent-skills@goal-standard-rules`; its first rule carries the outcome test. The body
has no bound on its length, and names the members of a set in a list where it has several.

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

The goal standard is `path@agent-skills@fragments/goal-standard.md`. Its rules: a goal states an
outcome the owner intends; it has one reading; it rules out nothing the owner has not ruled out
knowingly; it holds one intent; it contradicts no other goal; what shows it met is in what the
project builds; a refinement neither widens nor contradicts the goal it refines; and a statement of
what a Component is not for is a goal only when it names the design it excludes. Each rule's case,
read in the goal histories of rootpoint and thaum, is a `%%` comment beside it, except rule 7's, of
which no case was observed: it follows from the rule of no contradiction. The drafter writes to it
and the reviewer judges by it: goal-setting hosts it in
`skill@knowledge-architect-goal-setting@goal-or-not`, and the goal reviewer in
`agent@knowledge-architect-goal-reviewer@the-goal-standard`, per
`design@agent-skills@shared-text-fragment`. The primer hosts none of it, since only a drafter and a
judge of a goal apply it.

The goals bind outright and are the only statements assumed to come from the owner, per
`goal@knowledge-architect@the-owner-decides`, so a goal that reads wider than the owner meant rules
out work every later agent applies at its widest reading. In rootpoint's goal history, 9 changes
over 7 commits, which `git log --follow -p -- docs/goals.md` in rootpoint lists, the drafting step
caught no defect: reviews or the session caught 4, the owner 3, a design discussion 2; in 3 of its 4
cases of excess reach the excess was a conflict with a sibling goal. The mechanism is a phrase the
owner bounded by its context turned into a universal with no qualifier, read by the owner for intent
and not for what it rules out. The deliberation that read those histories is the spec of the work
that built the standard, which `git log --diff-filter=D --name-only -- docs/plans/` finds.

Rule 1 reads the outcome test against the owner's intent: an owner who intends a command line makes
a Component without one for something else, so a kind of interface the owner intends to provide is
an outcome, and the owner argued that such an interface is stated as a goal; renaming a command
leaves the Component for what the owner intends, so its exact form is a decision, which an interface
others consume makes a head. Rule 3 allows the distillation goal-setting is for, a goal wider than
each statement it is drawn from: the defect it prevents is a generalisation the owner never saw as
one, so the rival "no wider than the owner's statement" forbids what the skill exists to do. Rule 6
measures a goal on what the project builds, and evidence from use shows it unmet in practice, on the
owner's argument that in most projects a measure on use has limited feasibility. Rule 8 admits an
exclusion
only where another goal would otherwise argue for the design it names: a property of smallness with
no named design is the owner's scope control, exercised by what the owner asks for, or a quality
argument a head makes against its own rival. Rule 4 asks only that one goal not conflate two
intents, and two goals may share an aspect; a finding of rule 4 or of rule 6 is tagged `minor` and
counted apart, as the owner proposed; for rule 4, on the owner's argument that it does not change
what an agent does in the project, and that no overlap at all between goals is probably out of
reach. Rule 4's test is the head standard's rule on
an exception, not the test of reversing one part alone, which splits a rule from its exception, per
`rejected@agent-skills@head-split-by-independent-reversal`.



The nearest rival, the standard written in goal-setting alone with the reviewer briefed to read it,
puts the reviewer's standard behind a read the brief must ask for, which
`rejected@agent-skills@one-generic-auditor-agent` records as uneven.

### An agent that meets a goal reading two ways in use gets no instruction of its own `##no-instruction-for-a-goal-read-two-ways`

No installed text tells an agent what to do when a goal it applies reads two ways. A conflict
between a proposal and a goal already goes to the owner, per `primer@goals-bind`, which a case of
rootpoint's goal history showed working, and the goal reviewer reads a goal's readings before the
owner rules. The rival, an instruction for an agent applying a goal that reads two ways, adds a rule
for a case the routing already decides, against `goal@agent-skills@only-needed-instructions`.

### A goal draft shows each goal with its boundary block, and the owner's statements go to the reviewer, not into the display `##goal-draft-shows-reach`

Goal-setting shows the owner each drafted goal with its boundary block: for each universal or
exclusion, at least one case it rules out and one it admits, and each sibling goal it touches with
the relation, a conflict named as such; and each question of intent the reviewer raised, with the
case on which the readings differ. A contradiction is put at the top of the draft. The block never
reaches the goals home. The owner's statements are given to the reviewer whole and are not shown
beside each goal, since an intent spread over several statements is carried by the distilled goal,
and a short quotation beside it carries little of it and can misstate it.

The block turns a goal's reach into cases the owner judges by intent, since a prose goal is easy to
approve without seeing what it rules out, and the owner reported approving rootpoint's goals in a
rush, trusting proposals that read well. Whether a case was meant is a question of intent, so the
rival, the repaired draft shown alone with the reviewer as the only reader of reach, leaves the
reviewer a question it cannot rule on. It serves `goal@knowledge-architect@the-owner-decides` and
`goal@knowledge-architect@owner-attention-on-what-is-theirs`.

### A fresh goal reviewer reads every version of a goal draft before the owner rules, in a draft, a whole-set and a merge mode `##goal-reviewer-agent`

`agent@knowledge-architect-goal-reviewer` is read-only and hosts the goal standard. In draft mode,
goal-setting dispatches it before every ruling of the owner, on the whole batch of new and reworded
goals, every relevant statement of the owner verbatim whether or not the drafter drew on it, and
every goals home, and again on each repaired or reworded version, so the version shown is a version
reviewed. In whole-set mode it reads every goals home and gives every goal's boundary block
material, on which the owner judges rule 3. In merge mode it is the goals axis of the review before
a merge, per `design@agent-skills@goals-axis-in-the-review-before-a-merge`. Its findings are tagged
`settled`, which the drafter repairs; `intent`, a question only the owner answers; or
`contradiction`: a statement of the owner that conflicts with a goal or with another statement, or a
part of the statements a draft drops or turns around without showing it, which the drafter never
resolves alone. A finding of rule 4 or 6 is also tagged `minor`, per
`design@agent-skills@goal-standard-rules`. Each mode's own checks fire only on the input its brief
carries, so one fragment serves three modes.

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
`docs/goal-reviews/<YYYY-MM-DD>-<subject>.md`, holds each round's counts, outcomes and defaults, the
owner's answers verbatim and the boundary blocks; it is committed each round, every brief names it,
and the commit that writes the goals the owner ruled last deletes it. The rounds stop on the owner's
word, or once a round's count of findings that are not `minor`, every reviewer together and under
one brief, falls to a fifth of the first round's; the round that meets the stop writes no repair,
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
tenth only at their last; the owner approved it. It serves
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
