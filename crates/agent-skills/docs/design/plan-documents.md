# Agent skills design: Plan documents

## Plan documents

### The plan documents' words follow common developer usage, one word to one meaning `##document-vocabulary`

A plan document is a spec, a milestone document or the spec of a slice, kept in the plans
directory, docs/plans/ at the project's root, whose path the checker fixes, per
`design@core@plans-dir-fixed`. The directory is named plans rather than planned: in common English,
planned work is intended or scheduled work, designed or not, which is the roadmap's content. A spec is the plan document of work done in one branch and one PR. A milestone is work across
several PRs with design sessions between them; its plan documents are its milestone document and
one spec per slice. A slice is a part of a milestone that is one branch and one PR, with its own
spec. A step is one item of an implementation sequence, in a spec or in a slice, and no unit of
the history, per `design@agent-skills@step-is-no-commit`. The word "plan" alone never names a document: it would name the directory, a
document and a kind of document at once. The words follow common usage among developers, which the
owner made binding: "the vocabulary must be conform with common usage, especially in the developing
community". A milestone groups the work toward one goal, as GitLab's milestones do, and a
spec says what will be built and how before the code exists, in the sense engineering teams give
the word. "Design doc", the closest common term, lost because "design" already names the durable
register. One word for both a milestone's part and an item of a spec's sequence lets a rule for
the one, one branch and one merge per part, contradict the other, a spec of several steps on one
branch. So the milestone's part takes its own word, slice, on the owner's proposal, a part of the
larger whole, and "step" keeps the general sense it has in common usage and in the design skill's
loop.

### A spec's step is no unit of the history: how many commits it takes is the implementing session's to judge `##step-is-no-commit`

Naming a spec's
items by a unit of the history, one step to one commit, lost: it fixes the plan onto the
history, against `goal@agent-skills@only-needed-instructions`, and a commit the work
needs for another reason, such as a fix met outside the task, would then contradict the plan. The
owner ruled it out: "since it designates something 'real', it rigidifies the workflow and
'hardcodes' that one step equals one commit. This is likely to cause conflicts with other workflow
instructions, for example fixing with a commit a 'met outside this task' item".

### A layer of planned work has one plan document `##spec-and-milestone`

The work of one PR has one plan document, its spec. A milestone has a milestone document and one
spec per slice, which hold the spec's sections between them, split as
`design@agent-skills@milestone-split-by-lifetime` says. The two layers are the owner's proposal: "I
would decompose this into two layers: plans for short work, and plan for multi session work". A
spec plus a separate detailed implementation plan lost: both carry the same decisions and the
second drifts from the first, against `goal@knowledge-architect@documentation-stays-consistent`.
The one exception is a detailed plan, if one is ever written for a less capable implementer: it covers a bounded
amount of work and opens by saying it rests on assumptions, per
`design@agent-skills@no-untested-snippet-is-authority`.

The design this decision did not keep: `rejected@agent-skills@spec-plus-detailed-plan`.

### No untested code snippet in a plan document is authority `##no-untested-snippet-is-authority`

A plan document is detailed about the design and concise about the implementation sequence, and no
untested code snippet in it is presented as authority: a snippet is labelled as an illustration of
a shape. The rule is the owner's: "a detailed design/spec document, with a concise implementation
sequencing plan, is much better than a very detailed plan", from what the owner saw of detailed
plans: "an implementer that is only given the resulting plan often takes it as absolute truth, and
does anything to make the code snippets work, at all costs (sometimes breaking everything else
instead). They also often keep comments in snippets (that are meant as guidelines for the
implementer) verbatim in the real project", and "it must keep a disclaimer that content is based on
assumption and might not be perfect". The rival, a plan document whose snippets read as
instructions, is not prevented by one document per layer, since a spec with no separate plan can
still carry them. Asked, under entry test 4, whether these words record the owner's intent, the
owner answered: "this one is quite important and I want to give it a higher ranking. It could
apply elsewhere in the future too."

The design this decision did not keep: `rejected@agent-skills@spec-plus-detailed-plan`.

### A milestone is a directory, its head a README, each slice a spec `##milestone-is-a-directory`

A milestone's plan documents are one directory under milestones/ in the plans directory, which the
checker makes an anchor named by its basename, per `design@core@plan-document-kinds`: the milestone
document is its `README.md`, a generated `index.md` lists the slices, and each slice's spec is a file
beside it, cited `spec@<milestone>@<slice>`. The head is a README because the checker
resolves a relative link only in a `README.md` or an `index.md`, per
`design@core@links-are-navigation-rows`, so the head can link each slice's spec as a navigation row.
A slice's spec is a spec, so it leaves when its slice lands, per
`design@agent-skills@plan-document-leaves-at-landing`: its decisions are then in the design homes. The
README leaves once its last slice has landed and every default it lists is ruled, per
`design@agent-skills@default-ruled-before-merge`: in the commit that deletes that slice's spec, or in
the commit that records the last ruling.

### A milestone's design is split across its documents by lifetime, from the session that converged `##milestone-split-by-lifetime`

The design of a milestone is split across its documents from the start, by lifetime: a slice's spec holds what
only that slice builds, and the README holds what crosses slices or outlives one, every item cited
from more than one slice's document included. A README holding every slice's design is read whole
at every slice's grounding and grows with every slice: the README of the structured-plans milestone
held 1,624 lines when it left, counted with `wc -l` on `git show` of its deleting commit's parent.
The slice specs exist from the session that converged, so the audit of a slice applies its findings
in place, in whichever document holds the shape, and a new slice found at an audit gets a spec of its
own, as a scope change the owner rules on. The audit's commit message lists each finding with its
gap, its answer and the decision it follows from, so an edit in place loses no record of what the
audit found.

The design this decision did not keep: `rejected@agent-skills@milestone-holds-every-slice-design`.

### A plan document leaves when its work lands, in a commit of its own after the review before the merge `##plan-document-leaves-at-landing`

A plan document is deleted once its last harvest is complete, in a commit of its own after the
review before the merge has read the tip that holds it, and that commit's message cites it by its
kind, which resolves against the commit's parent. A plan document kept after its work lands is a
second home for every decision it carried, and it starts drifting at the first later reversal.
Walked by the checker, its references break at every reversal, and someone repairs a document about
finished work; left out of the walk, it is unchecked text that a grep finds with no marker that it
is stale. Either breaks `goal@knowledge-architect@documentation-stays-consistent`. Leaving the
choice to each project lost too: the installed planning skill would have nothing to say where a
document's work ends, against `goal@knowledge-architect@agents-get-a-complete-workflow`.

The deletion comes after the review before the merge, since four acts of that review need the
document: the transcript reviewer's search of the tip for an argument's outcome, the standing-state
reviewer's reading of the criteria the landing judges, the spec-conformity and
fidelity-of-relocation axes, and `git log -G` on a thread-to-slug pair written into the harvest row,
which a row edited in the deleting commit leaves in no tree: re-taken by the reproduction the
issue entry closed by the commit "[agent-skills] A plan document leaves in a commit of its own, after
the review before the merge has read it" held, in its parent's tree, where `git log -G` on the pair
prints nothing. A
branch that deleted its spec with its
harvest showed the first three in real use: every brief had to name the deleting commit's parent,
and a history edit of the branch then gave that commit a new SHA, so the briefs named a commit the
branch no longer held. Extending the briefs to read the document from history was the rival: it
closes neither the pair's case nor the stale SHA. The design this decision did not keep:
`rejected@agent-skills@plan-document-deleted-with-its-harvest`.

### A discussion whose decision lands in the change under way keeps its deliberation in that change's commit message `##in-change-path`

The design skill has two paths, which differ only in where the deliberation is kept until the
harvest. The in-change path is open when the decision lands in the change under way: its work,
where that change began before the discussion opened, as an issue fix or a review repair does, or
its record, whenever the discussion opened, for a decision recorded when it is made, per
`design@agent-skills@design-home-is-built-intent`. It writes no plan document. The message of the
commit that writes the decision's design entry, or that implements a decision that earns none,
carries what a plan document would: every thread with its final state, the ground of each
closure, the owner's ruling attributed per `design@agent-skills@attribution-replaces-quotation` or
the argument of a thread closed `argued`, the rivals that lost with their reasons, every premortem
cause with its outcome, and the acceptance criteria. The
premortem runs when reversal touches any of the four things the skill names: stored data, a
consumed interface, behaviour users have adapted to, and a decided thread that would reopen. The four never open the in-change path: a decision that is cheap to
reverse and whose work comes later takes the full path, per
`design@agent-skills@design-hands-off-to-planning`, as does any discussion that converges on work
that does not land in the change under way. A requested change that is not bounded, and whose work
has not started when its discussion opens, takes the full path too, even when one pull request
builds it. Decided and unbuilt intent then has one home, the plan document, per
`design@agent-skills@design-home-is-built-intent`.

A plan document exists to carry a decision from convergence to a landing that comes later, and to
be reviewed before its work is built. A decision met during a change begun before the discussion
lands in work already under way, so neither applies, and requiring a spec would make every
decision met during a task cost a plan document, which is what
`design@agent-skills@new-or-reshaped-head-needs-design` must not cost to be followed. A requested
change whose work has not started has all of that work ahead of it, and its plan can be reviewed
before any of it is built. The move of an adopting project's existing documents starts as such a
change, and it is planned work under the planning skill, per
`design@agent-skills@adopting-existing-docs`. The cost is a spec, its plan reviews, its harvest and
its deletion for every requested change that is not bounded, including one the converging session
builds in one pull request, which strains the "fewer review rounds" of
`goal@knowledge-architect@agents-get-a-complete-workflow`. Widening the existing short path, rather
than adding a path for one decision met during a task, is the owner's proposal, on the owner's
argument, given as the owner's opinion, that a discussion that starts with one decision should be
bounded in neither threads nor rounds, since its size depends on what grounding, investigation and
experiment bring, which nobody knows in advance. The separate path bounded the discussion to one thread, against
`design@agent-skills@structure-the-flow`. The cost is a long commit message when a discussion
inside a task grows to several threads; its length is not bounded, since a bound on it would bound
the discussion.

### The planning skill writes the plan document, and the design skill writes none `##design-hands-off-to-planning`

The planning skill starts where a design discussion has converged on the full path, and writes the
spec or the milestone. A discussion on the in-change path has no plan document,
per `design@agent-skills@in-change-path`. The design skill ends at convergence and the premortem,
its causes named and its acceptance criteria written, and writes no plan document of its
own. Its decisions are
recorded when their work lands, per `design@agent-skills@design-home-is-built-intent`. One skill owns the document's shape: two skills describing the sections of one document
would drift apart, against `goal@knowledge-architect@agents-get-a-complete-workflow`.

### A plan document is written in the session where its discussion converged `##plan-written-in-the-converging-session`

The planning skill writes the spec or the milestone in the session where the design discussion
converged. The session
matters because the discussion's records are that session's: its conversation, and the transcript
the harness keeps of it, which that session finds with `klarch transcript list`, per
`design@agent-skills@transcript-found-by-listing`. A document written from memory in a
later session is written from a summary, and a summary loses the losing arguments and the
conditions of each closure. Where the harness keeps no transcript, the conversation is the only
record, and compaction can remove it.

### The planning skill assembles the plan document from the discussion's transcript `##ledger-from-transcript`

The planning skill assembles the document through a subagent that reads every transcript file the
discussion spans, the harness's log of the session on disk. The design skill's per-round delta is
the draft it reads, and the deltas are corroborated as written, since the owner read and answered
each. Where the harness keeps no transcript, it assembles from the conversation. The premise is
that the transcript keeps the records from before a compaction, which a summary of the
conversation loses. Every user and assistant record before a compaction boundary stays in the file;
`jq -c 'select(.type=="system" and .subtype=="compact_boundary") | input_line_number' <log>` finds
the boundaries of any log, and a count of the records before each re-takes it; the column
`compactions` of `klarch transcript list --all-projects` counts them per session. Over every
transcript of the machine the transcript command was designed on, on 2026-10-25, the lines before
the boundary stayed in the file in 7 of 7 compactions. The extraction can be
wrong, so the transcript reviewer checks every assembled document against the same files. The
rival, a draft ledger the agent writes to a file every round with the least effort, answers the
cost of a write per round and not the other two reasons of the rejected alternative it repeats: a
stale ledger stated with confidence is worse than none, and a file the agent writes is corroborated
only by the conversation, which compaction removes.

The design this decision did not keep: `rejected@agent-skills@ledger-written-during-discussion`.

### Acceptance criteria live in the plan document of the work that judges them `##acceptance-criteria-in-the-document`

An acceptance criterion, a check on a recorded decision that only the work's built code can apply,
is written in its own section of the plan document of that work. Each names the decision it guards,
the step or slice that judges it and the observable that fires it; its firing reopens that decision,
per `design@agent-skills@criterion-ruled-at-reopening`. Every landing reports on
the criteria it judges. When the document leaves, a criterion that recurs is judged as a cause by
the standard, and written as a tripwire where it passes, per
`design@agent-skills@tripwire-recording-authority`; any other is deleted. A separate file
of criteria would hold statements about the same work with the same lifetime, drift from the
document, against `goal@knowledge-architect@documentation-stays-consistent`, and stay behind when
the document leaves. The plan document is written in the session that converged, per
`design@agent-skills@plan-written-in-the-converging-session`, so the criteria and the document
are born together.

### The owner gives no ruling on an acceptance criterion, and a firing opens a design discussion with the owner on the decision it guards `##criterion-ruled-at-reopening`

No acceptance criterion is put to the owner, wherever it is proposed: at the premortem, at the
assembly, by a review or at an audit. The standard of `design@agent-skills@criteria-test-silent-premises`
ranks a criterion, as `design@agent-skills@tripwire-admission-test` ranks a tripwire, so no fork
remains, and the question is the session's, per `primer@question-authority`; the argument that
ended the owner's ruling on each tripwire, `design@agent-skills@tripwire-recording-authority`,
applies. A value a criterion's basis leaves open is a fork among values: the session lists it among
the plan document's defaults awaiting the owner, so the owner meets it before the merge, per
`goal@knowledge-architect@design-with-agents-as-counterparts`.

A criterion that fires leaves the plan document as a reopened decision: its firing opens a design
discussion with the owner on the decision it guards, under `skill@knowledge-architect-design`. In
that discussion each question goes to its authority, per
`design@agent-skills@questions-go-to-their-authority`: the measurement says whether the decision
failed, the argument settles the shape that replaces it where it can, and the owner rules its forks,
per `goal@knowledge-architect@the-owner-decides`. The part of the work that rests on the guarded
decision waits for the discussion, and the rest goes on.

The ruling a firing would have answered is the owner's at the discussion it opens, where the
owner's word bears on whether a criterion's decision stands, besides a value a criterion's basis
leaves open; the reopening is one stopping point where a ruling at the premortem and another at
the firing were two. The cost: a criterion that costs real work enters a plan on the standard
alone, and the owner meets that cost by reading the plan document.
The design this decision did not keep: `rejected@agent-skills@owner-rules-each-criterion`.

### An acceptance criterion is admitted only where its failure is silent, its firing shows the decision wrong, and a number in it has a basis `##criteria-test-silent-premises`

A criterion meets conditions 1, 3 and 4 of the tripwire standard, the fragment
`path@agent-skills@fragments/tripwire-standard.md`: no test, gate or review reports its failure;
its firing is what the decision says must not happen, a failure of the decision and not of the
work; and a number in it carries its basis. The planning skill's section on acceptance criteria and
step 7 of the design skill state the three conditions where criteria are written, and condition 1
of the standard routes a check of the work being built the same way. Condition 2 does not apply:
the landing's own work reports on a criterion.

The census of the 61 acceptance criteria of the 39 plan documents deleted from docs/plans/ before
741f655e, read by the agent of the design discussion that made this decision, found about 49
completeness checks, which a review already reports, about 5 that failed the silence test, and 3
that only a measurement of the built result in use could take: a criterion bound to pass, or one a
review reports, misdirects the attention a criterion should hold, and an instruction that asks for
it is one `goal@agent-skills@only-needed-instructions` removes. The corpus is listed by
`git log 741f655e --diff-filter=D --name-only -- docs/plans/`, each document read at its deleting
commit's parent; the per-criterion verdicts were the agent's judgement, kept in no record, so only
their order of magnitude is claimed. The same agent counted about ten past firings, of which one tested a decision. The
owner reported that criteria are too often of low quality, carrying numerical bounds that are not
relevant, or are in practice certain to pass. The design this
decision did not keep: `rejected@agent-skills@criteria-as-they-stand`, and removing criteria,
`rejected@agent-skills@criteria-removed`.

### A check whose firing would show the work unfinished is a claim of the step that builds it `##completeness-check-is-a-step-claim`

A cause that only the built result can check, and whose firing would show the work unfinished
rather than the decision wrong, is written as a claim of the step that builds it, with its test,
which the spec-conformity review judges; on the in-change path, the change applies the test. It
keeps the check the owner did not want lost, and leaves `design@agent-skills@criteria-test-silent-premises`
its criteria, whose firing reopens a decision. The rival, dropping such checks with the criteria
that held them, loses a completeness check no review was told to look for, against
`goal@knowledge-architect@agents-work-without-drift`, under which a departure from the recorded
design is caught by a check or a review before it merges. Step 7 of the design
skill, the planning skill's section on criteria and condition 1 of the tripwire standard route it.

### A plan document is committed before its reviews, which read that commit `##plan-reviewed-as-a-commit`

The planning skill has a plan document's reviewers read its commit, not the working tree. The review
skill names what a reviewer reads as a commit range and gives each reviewer that runs the tests,
the checker or a program the project builds a worktree detached at the commit under review; a document still uncommitted has neither, so the two
skills could not both be obeyed. A repair lands after the review, per
`design@agent-skills@review-repair-appended-or-folded`.

### A review repair is appended, and folded only where appending leaves an earlier commit failing `##review-repair-appended-or-folded`

The review skill has a repair land as a new commit after the work it repairs, which edits no
history. Where a project requires every commit of a branch to pass checks the repair changes, an
appended repair leaves the earlier commits failing, and the project's rule and this one leave no
legal move; the repair is then folded into the earliest commit it repairs, with a clean tree,
confirmed to have lost no content. The record of the review says what was folded, so the landing
history still says what the review found. The rule exists to avoid history edits, not to keep a
repair apart from what it repairs, which is the owner's reason for it.

### A plan document's landing is not tied to its work's, except before work that changes what the per-commit gate checks `##plan-landing-is-not-tied-to-its-work`

A plan document, a spec or a milestone document, is merged to the main branch in a pull request of
its own or with its work, whatever the time of its work. An owner
plans a feature that will not be built yet so that the work done meanwhile does not drift from it,
which needs the document where every session reads it, on the main branch; that serves
`goal@knowledge-architect@agents-work-without-drift`. What "spec" and "milestone" measure is the
work, one pull request or several, not where the document lands. The document leaves when its work
lands, per `design@agent-skills@plan-document-leaves-at-landing`.

The owner's intent: "It regularly happens that I will prepare a spec/design for a feature that I
won't be implementing right now, just to ensure that the intermediate tasks I will perform do not
get misaligned too much with that planned feature. Anyway, what I mean is that writing plan
document should not be constrained with when its implementation work happens." Asked whether it
records the owner's intent, the owner answered: "Q1 yes".

The one exception: a plan document lands before any work that changes what the project's
per-commit gate checks. The per-commit gate judges each commit of a branch, its tree and its
message, with the checker at the branch's tip, per `design@core@a-commit-message-is-a-document`.
On one branch, that gate as the work's first commit changes it would judge the
commit that added the document, whose tree predates the change, and refuse it. A change that only a
check of the working tree sees, such as installed text that the checker compares on the working
tree alone, leaves the per-commit gate unchanged and is outside this exception. Keeping both on one branch would force
the fix the owner called absurd: the plan document committed after the work it plans. So that work
begins on a branch of its own, after the document is merged. This holds for a spec as for a milestone document, whose slice
that changes what the per-commit gate checks is the one this exception meets.

### Undesigned work is an issue `##undesigned-work-is-an-issue`

Work that is known but not designed is a `todo` or `deferred` issue in the owning anchor, with its
leads in the entry. The plan document that schedules it closes the issue in the commit that adds
the document. `goal@knowledge-architect@structure-and-workflow-work-together` asks for one place for
what is open: the issue register holds the work, and the roadmap only orders it, per
`design@agent-skills@roadmap-orders-issues`.

### The roadmap holds only the order of known work, and every row is a checked reference `##roadmap-orders-issues`

A row of the roadmap cites an issue entry or a whole plan document, in the order the owner wants the
work done, and an unordered section may follow. The work stays in the issue register and the plans
directory, so what is open is still listed in one place, per
`goal@knowledge-architect@structure-and-workflow-work-together`; the roadmap adds only the order. A
row dangles when its issue closes or its plan document leaves, and the check reports it, so the
order cannot drift from the work unnoticed. The commit that adds a plan document rewrites the row
of the issue it closes, and the commit that deletes one removes its row. The need is observed in
real use: thaum kept a file of its next milestones as an exception to the installed skill. A
roadmap register of its own, holding undesigned work, lost: two registers would each hold known,
undesigned work, which needs a routing rule and hides one of them from the issue listing.

The design this decision did not keep: `rejected@agent-skills@no-roadmap`.

### The roadmap's order is the owner's `##roadmap-order-is-the-owners`

A row of the roadmap is added or moved on the owner's word, since ordering known work is a fork: no
goal or decision ranks one piece of it over another, so the order is the owner's, per
`goal@knowledge-architect@the-owner-decides`. The rival, any session ordering the work, would make
the order a session's choice where no argument settles it. The two edits that keep a row pointing at the same work, rewriting
the row of an issue a new plan document closes and removing the row of a plan document that
leaves, need no word, per `design@agent-skills@roadmap-orders-issues`.

### No record of landed work is kept `##no-progress-record`

No installed skill asks for a file recording what has landed. History lists every plan document
that left, and each landing commit says where its results live; releases are in the changelog. A
progress file would be a third document about the same work, with a lifetime of its own, against
the one place for what is open of `goal@knowledge-architect@structure-and-workflow-work-together`.

### A plan document records the whole discussion `##plan-records-the-exchange`

A plan document assembled from a discussion records every thread with its proposer and round, its
final state, the arguments on each side, the owner's rulings verbatim with their round, and its
relations. The record is the owner's proposal: "I'd like to have the initial spec aim to be contain
a precise record of the design work and exchange that happened, with all threads written, their
final states, and the arguments, with the possibility of cross referencing all these items, bounded
to the spec document or the milestone directory." It applies to a spec, and to a milestone document
with its slice specs, which share the record by the rule of
`design@agent-skills@milestone-split-by-lifetime`; the owner extended it to milestones: "Milestones are
created through the design skill too after all." `design@agent-skills@standing-argument-in-head`
names the plan document as the home of the deliberation while it exists, and
`goal@knowledge-architect@the-owner-decides` is served only where the rulings are recorded as the
owner made them. On the in-change path, the commit message carries the same record, for the same
reason, per `design@agent-skills@in-change-path`. The rival, a plan document recording each thread's
final state and resolution, leaves the rulings and the arguments to memory. The cost is a longer plan
document to write and to read.

### An argument is an item, without a state `##arguments-as-items`

Each argument of the discussion is an item of the plan document, under its Arguments section, so a
ruling, a closure or a premortem cause can cite what decided it, and the standing argument a
harvest writes is a selection of named arguments rather than a rewrite. It carries no state: a
state would need a relation between many arguments and many threads, which the design skill
declines to track, and an identifier needs none.

### A leaving plan's citations are removed, and each citing plan gets a question `##retiring-plan-opens-issue`

A whole plan document may be cited from another plan, per `design@core@plan-item-scope`. When it
leaves, the session that meets the dangling citation, the one deleting it or the one rebasing the
citing plan onto the deletion, removes the citation and opens a `question` issue on the citing
plan: does it still hold now that the leaving plan is built, accounting for deviations or other
unplanned happenings? The issue cites
the citing plan, so it cannot outlive it, and its `Why it matters` cites what the leaving plan
harvested, since that plan no longer exists. It is a `question` rather than a `todo` because the
reading may find nothing to change. For a milestone, its next slice's audit reads it. A milestone's
own documents are one plan, per `design@core@plan-item-scope`, so its milestone document and its
sibling slice specs re-point a citation of a leaving slice with no question: that audit already
reads the milestone against what landed. The retiring
session revisits nothing itself: a revisit at that moment would redesign the citing plan at a time
chosen by another plan's landing, rather than at its own audit, where the owner rules on it as
`goal@knowledge-architect@the-owner-decides` asks.

The decision is recorded here because two installed skills apply it, the planning skill where a
plan document leaves and the issue-tracking skill among the issue kinds, and both act on the
dangling citation the checker reports: the same reason at two sites, and a procedure the installed
text and the checker must keep consistent, per `design@agent-skills@instruction-record-is-minimal`.
The argument is the owner's, against citing a plan's items from another plan: it "forces design
work at a moment that might not be the best".
