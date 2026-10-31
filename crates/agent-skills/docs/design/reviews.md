# Agent skills design: Reviews

## Reviews

### The standing-state review runs before every merge, and reads every tripwire and every deferred trigger `##standing-state-review-before-every-merge`

The installed review skill sends the standing-state reviewer before every merge to the main branch,
whatever the change. That reviewer is the standing re-entry point of every tripwire home and of
every deferred trigger, and asks of each tripwire whether it fired and whether its horizon passed,
per `design@agent-skills@tripwire-horizon`, as the installed issue-tracking skill states: it runs
for every change, where the search before the work runs only for work that goes through the design
skill, a plan or a retrospective intake, per `design@agent-skills@record-searched-before-the-work`.
A re-entry point that depends on whether a change looked related to a tripwire is one that a change
touching the guarded decision indirectly skips: the reviewer reads every entry of every home, not
the subset the diff seems to concern. This serves
`goal@knowledge-architect@documentation-stays-consistent`.

A deferred issue's trigger answers the question a tripwire's firing clause answers, what will make
someone do this, and the issue-tracking skill holds both to one test. So the reviewer reads every
deferred trigger against the change, as it reads every tripwire, and reports a trigger the change
meets as a finding, whose repair is the work the issue names or the owner's ruling. Without it, a
trigger met by work that does not go through the design skill has no reader at all.

### A fresh reviewer reads every plan document against the goals, the design heads and the rejected alternatives of the Components it touches `##plan-read-against-the-record`

At every moment the installed planning skill sends a plan document's reviewers, it sends the
design-conformance reviewer beside the cold implementer, the code-claims reviewer and the
transcript reviewer. It reads the document against the goals homes, the design homes and the
rejected alternatives of the project's root and of every Component the document touches. It
reports a contradiction of a goal always, for the owner; a contradiction or a widening of a head
that the document's "What is already decided" does not list as reversed or rewritten; a rejected
alternative brought back without a recorded reopening; and an acceptance criterion whose observable
the record rules out. The second rule keeps it from reporting each widening the document makes on
purpose. It runs only commands that read.

The decision serves `goal@knowledge-architect@agents-work-without-drift`, which is met while "a
departure from that recorded design, or a rejected design brought back without that procedure, is
caught before it merges". The owner named the lack: "nothing seems to be reviewing a plan document
against recorded goals and design." The other reviewers read the document against itself, against
the code, or against the discussion. The design audit reads design homes, but not goals, and it is
run by the session that implements the work, which may be the document's author, and skipped for a
spec worked in the session that converged. Two acceptance criteria that
`goal@knowledge-architect@the-workflow-improves-through-real-use` rules out each passed a plan
document's three reviews in this repository. The deleted spec synthetic-evidence-and-plan-reviews
holds both, as its instances 1 and 2, and `git log --diff-filter=D --name-only -- docs/plans/` finds
it. The decision's nearest rival, the decision-record reviewer extended to plan documents, judges
the record a diff writes and holds a plan's items as not decisions yet, so it would carry two
questions with two yardsticks. The cold implementer, the other rival, asks whether the document can
be acted on, and a document that contradicts a goal can. The reviewer depends on a record worded
wide enough. The owner's words state where a miss then belongs: "this is always dependent on the
project being diligent in its goals and design records. Everything in the workflow is, after all.
Project owners still have to watch agent behavior and interrupt them if they find them acting
differently than their intent. Whether this is a published workflow defect or an internal
documentation defect is left to the retrospective skill to judge." Its cost is one more dispatch at
each of those moments; what it buys is a reader of the record that neither the author nor the
implementing session is, which the two acceptance criteria above show the other reviews lacked.

### The `may-bear` entries a design grounding leaves unread reach the design-conformance reviewer, which reads them whole `##unread-entries-reach-the-conformance-review`

The list of `may-bear` entries the session left unread, per
`design@agent-skills@may-bear-read-on-touch`, is kept where the work's deliberation is: in the plan
document's "What is already decided", or in the commit message on the in-change path and for
bounded work. The design-conformance reviewer reads whole every entry the list names, issues and
tripwires included, and reports one the work meets. At the review before the merge of a branch whose
commit message carries such a list, it is dispatched on that message and the branch's diff, and
reads as declared departures the heads the diff rewrites and those the message names as reversed.

The reading on touch leaves those entries to a reader, and this reviewer already reads the record
against the work, per `design@agent-skills@plan-read-against-the-record`, so on the full path the
list adds reading and no dispatch. The rival, the list recorded and read by no reviewer, leaves an
entry the session misjudged to surface only after the work, the rework the search before the work
exists to prevent, per `design@agent-skills@record-searched-before-the-work`. The cost is one more
dispatch at the review before the merge of a branch whose grounding left entries unread outside a
plan document.

### Before every merge, the transcript reviewer runs alone and last, and not with the other axes `##transcript-review-last-before-merge`

The installed review skill sends the transcript reviewer before every merge to the main branch once,
after every other axis has run and its repairs are committed, and not also in their batch, over the
whole branch and the transcripts of every session that worked on it. The findings of the other axes
reach the session as messages of its transcript, so only a reviewer reading it after their repairs
can tell whether each was acted on durably, per
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. A transcript reviewer sent in
their batch as well would read the transcript before the owner answers the items the batch raises,
and an answer of the owner given after its extraction reads to it as missing, as
`skill@knowledge-architect-review@review-axes` says: its findings would report as missing what the
last transcript review then finds answered. Its own repairs land as
`design@agent-skills@transcript-review-repairs-are-additional-commits` says. An additional commit
that adds, removes or reverses a design head is reviewed by the decision-record axis at least, and
the review ends with that review's repairs; any other is reviewed by no axis again, so the review
ends, per `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.

### The last transcript review's repairs land as additional commits, as many as their kinds need `##transcript-review-repairs-are-additional-commits`

The repairs of the last transcript review before a merge land as additional commits, as many as
their kinds need: a repair that the primer routes to a commit of its own stays one, and nothing asks
the dispatcher to merge them into a single commit. The additional commits are the owner's proposal:
"I'd just replace "as a last commit" by "as an additional commit". This is simpler, it prevents any
possible conflict, without exceptions and no need to check a condition."
A repair that would leave an earlier commit failing is folded, per
`design@agent-skills@review-repair-appended-or-folded`.

### A review repair that adds, removes or reverses a design head is reviewed again by the decision-record axis, and no other repair is `##only-a-head-changing-repair-is-reviewed-again`

The installed review skill sends the decision-record axis at every repair commit that adds, removes
or reverses a design head, whether it answers the transcript reviewer or any other axis. A repair
made before the last transcript review is reviewed before it; a repair of the last transcript review
is reviewed after it, and the review ends with the repairs of that decision-record review. Any other
repair is reviewed by no axis again: a rewording, and an argument, a mention or a reference added
inside an existing head, included. One made before the last transcript review is still read by it,
as every commit of the branch is. Without the record review, a head added, removed or reversed in
answer to a review reaches the main branch with no review of its record, against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: the last transcript review judges
fidelity to the owner's rulings and not the record.

The trigger is a whole head, which the diff shows: a heading with a slug that appears or disappears,
or a reversal under `skill@knowledge-architect-decision-recording@reversal-check`. Each round of
repairs then adds at most as many re-reviews as it adds, removes or reverses heads. A trigger on any
repair that "makes a decision" lost: whether a rewording makes one is a judgement, and read wide it
re-reviews repairs that only reword a head. The rival that sends every axis again at every repair
lost on the same cost. A relaxation of an approved head, or a turn of its direction, that this
trigger leaves out still goes to the owner, per
`skill@knowledge-architect-decision-recording@head-shape`, and the last transcript review checks it
against the owner's rulings.

### Subagents dispatched together each get a scratch directory of their own `##a-scratch-directory-per-subagent`

A dispatcher that sends several subagents together names in each brief a scratch directory distinct
from the others', as it names a distinct worktree for a reviewer that builds, and an agent that
writes working files writes them there only. The installed review skill states it among the
invariants of a dispatch, and the record searcher's description, which carries its dispatch rule,
makes each agent directory that `klarch worklist` writes its searcher's scratch directory.
Subagents sharing one scratch directory overwrite one another's files: parallel searchers in one
session did, and one repeated its work, and reviewers in another could not tell whether files of the
same names had been overwritten. A searcher that does not notice fills another agent's report,
which `goal@knowledge-architect@agents-get-a-complete-workflow` does not allow of a step every
design discussion runs.

The rival that keeps every agent from writing files lost: the searcher's own task is to fill its
report, and searchers told to write nothing wrote files all the same, so the instruction could not
be obeyed together with the task.

### Every review whose diff adds to or changes agent-facing text owes an axis that judges each statement against the standard on that text `##review-judges-additions`

A row of `skill@knowledge-architect-review@review-axes` holds the axis on agent-facing text,
applicable to every diff that adds to or changes it, installed or a project's own, whatever activity
produced the diff. It judges each added or changed statement against
`skill@knowledge-architect-agent-configuration@admission-standard`, the standard every writer and
judge of that text applies, per `design@agent-skills@admission-rules-home`. No other axis judges
whether a statement is needed: the routing axis judges where it goes, and the reviewer agents leave
style aside. In the thaum session that wrote the statement the owner judged useless, five review
axes passed over it, by the message of thaum's commit that added it. The owner holds the review of a
diff the cheaper place to catch such a statement, against the cost of an audit.

The row sits in the review skill's table rather than among the axes the
agent-configuration skill adds to the reviews of its own activity: agent-facing text is also
edited inside other activities, such as an issue's closure or a plan's harvest, whose reviews an
axis of that skill would not reach. Text no branch touches is read by the need-test lens of the
agentic-workflow axis, per `design@agent-skills@workflow-axis-lenses`.

### The review axis on agent-facing text is carried by an installed agent, which holds the standard `##additions-review-agent`

`agent@knowledge-architect-agent-text-reviewer` holds the axis's procedure, naming the act of each
statement and then judging its evidence and its kind, and its report. For the standard itself it
hosts every fragment of `skill@knowledge-architect-agent-configuration@admission-standard`, per
`design@agent-skills@admission-rules-home`; a branch that changes one has the reviewer read its
definition from disk, as `skill@knowledge-architect-review@review-invariants` asks of every changed
definition. The nearest rival, a general-purpose subagent briefed with the axis's question, has each
dispatching session write the procedure into its brief anew, and a procedure rewritten per brief
drifts; every other review axis with a fixed standard has an agent of its own.

### A transcript reviewer checks that what a work's sessions established has a durable outcome, and that no ruling of the owner is recorded reversed, relaxed, turned from its direction or falsely attributed `##transcript-reviewer-agent`

The installed agent `agent@knowledge-architect-transcript-reviewer` reads the transcripts of the
sessions that produced a piece of work, named as a commit range, and checks two things. First, that
each decision of the owner, each argument that decided something, each finding of a reviewer and
each thing met outside the task has a durable outcome: recorded in its home, acted on by a repair or
an issue, or judged to need nothing with the reason written where the owner reads it. What lives
only in a conversation is lost when the session ends or is compacted. Second, that no ruling of the
owner is recorded reversed, relaxed, a check, a claim, a criterion or the scope of the work made
weaker, turned from its direction, or attributed to the owner where the owner did not decide. A
refinement within the ruling's direction, stricter, more detailed or reworded, is no finding, and
"approved", "accepted" and "agreed" approve the idea, not the wording. An attribution still keeps
the owner's scope and hedges, per `design@agent-skills@attribution-replaces-quotation`: a hedge
dropped records a firmer ruling than the owner gave, minor on a detail that is not load-bearing and
major otherwise, per `design@agent-skills@misstated-ruling-rated-by-what-it-changes`. The owner
argued that the reviewer's weight belongs on whether something from the transcript was forgotten or
not handled durably, and that a small addition, even a clearly sane deviation, is no finding. A
ruling is a word of the owner on a question `primer@question-authority` reserves to the owner; an
answer on a reading or a fact is checked against its source, per
`design@agent-skills@answer-keeps-its-authority`. The reviewer assumes no shape of the work: a plan
document is one home among the others.

The reading by direction is the owner's report: an acceptance or an agreement of the owner is rarely
about the exact wording, but about the idea and the direction, so a refinement the agent later
judges relevant is likely acceptable, unless it relaxes a claim or a criterion so that unfinished
work passes. A check of scope that also reported a clause added within the ruling, "no obligation
added", made a stricter acceptance criterion a major finding. The same reading applies wherever
fidelity to an approval is judged, in the rules on heads, the record reviewers and auditors, the
plan reviews and the retrospective, with one exception: a goal's wording that rules out what the
owner has not ruled out knowingly is a finding, since a goal binds outright and rules work out, so a
wording stricter than the ruling is no refinement there; the transcript reviewer, the planning
skill's plan reviews and the retrospective each state it, and the case is new against
`rejected@agent-skills@transcript-reviewer-reports-every-clause` on that fact. The reading holds in
all of them, since `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` asks that the criteria
of the reviews and of the audits agree.

It is an agent, not a line in a skill, because its standard is fixed. Its extraction is the
checker's, per `design@agent-skills@transcript-extraction-is-the-checker-s`. It serves
`goal@knowledge-architect@the-owner-decides`, whose forks are the owner's, so a ruling misrecorded
decides in the owner's place. The durability check serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. The check of rulings has shown its
worth: in the review of the change that installed the first skills, a reviewer briefed with it was
the one of six to find a decision recorded narrower than the owner's approval. The milestone
document of the v0.1 extraction holds that count, in its section on session B of step 5, and `git
log --diff-filter=D --name-only -- docs/plans/` finds it; the message of the commit "[review] Six
reviewers read PR 1: the hand-off rule, the shipped text and the record are corrected" records the
review's six axes and its finding.

The design this decision did not keep:
`rejected@agent-skills@transcript-reviewer-reports-every-clause`.

### A session is extracted with the checker's `transcript show`, and no installed text holds an extraction rule of its own `##transcript-extraction-is-the-checker-s`

The transcript reviewer extracts each session with `klarch transcript show --from --full`, and the
subagent that assembles a plan document extracts as the reviewer's section says; the command labels
every line by its fields, per `design@core@transcript-labels-in-code`, so the installed texts state
what the labels mean and no rule for which lines to keep or whose they are. A rule held in the
reviewer's text, with the command as a helper, is applied by a script each session writes again,
and stands in two homes beside the command's, against `design@core@one-question-one-command`; a
fragment the build copies would still leave each session to write the script. It serves
`goal@knowledge-architect@agents-get-a-complete-workflow`.

### A transcript is found by the transcript command's listing, never by a session identifier `##transcript-found-by-listing`

The review skill, the planning skill and the transcript reviewer name a session by the opening words
of the message where its work begins, and find it with `klarch transcript list`: `--branch` lists
the sessions that worked on a branch, `--containing` the session holding those words in a message
the owner typed. No installed text finds a transcript by a session identifier. After `/clear`, the
process keeps the earlier identifier in its paths while the harness writes a new file, as
`design@core@transcript-session-split` measures, so a session that derives its transcript from the
identifier it sees reads the session before the clear. The same reason holds at the three texts,
which is why it has this home. The listing is computed from the transcripts, per
`design@core@transcript-query-verbs`.

### A misstated ruling is critical only when it reverses the ruling, or relaxes it in what is built or a load-bearing decision `##misstated-ruling-rated-by-what-it-changes`

A misstated ruling is critical only when it is reversed, or relaxed or put in a state that changes
what is built or a load-bearing decision. A decision attributed to the owner that the owner did not
make, and a ruling turned from its direction, are major. A ruling recorded a little relaxed, or
firmer than its hedges, on a detail that is not load-bearing, and an argument or a reading
attributed to the owner that is not the owner's, are minor and still go to the owner; a minor ruling
is no finding of the retrospective once caught before the merge; a refinement within the ruling's
direction is none at all, per `design@agent-skills@transcript-reviewer-agent`, except in a goal: a
goal recorded stricter than what the owner ruled out knowingly changes what is built, since a goal
rules work out, so it is major. The cut is what the misstatement would build: a reversal, or a
change of what is built or of a decision others rest on, produces wrong work if it merges, where a
slip on a detail that bears on nothing built produces none, and is repaired once the owner sees it.
That such a slip is no critical defect is the owner's weighing, from the owner's observation of real
sessions: each slip the owner saw was on a small detail the owner judged not load-bearing, the
transcript reviewer catches them reliably in the owner's view, the owner does not believe any
instruction would do better since agents are trained to take a human's input seriously, and a
reviewer flagging such a slip once in a while, for a decision that is not among the most critical
and with only a small deviation from the owner's word, short of a reversal, is no critical defect.
The rival, every misstated ruling rated critical, made a reviewer's report of a slip read as a
critical defect, which the owner judged it is not; the slip is still repaired, and it is no defect
of the workflow.

### The review before a merge owes the goals axis, the goal reviewer in its merge mode, when the diff adds, rewords or removes a goal `##goals-axis-in-the-review-before-a-merge`

A row of `skill@knowledge-architect-review@review-axes` sends
`agent@knowledge-architect-goal-reviewer` in its merge mode at a branch whose diff changes a goals
home. It reads every goal the range adds or changes against the goal standard's rules that need no
statement of the owner, and against every goal of the tree, and reports each goal edit whose ruling
by the owner no commit message of the range records. A goal edit can be made inside another
activity: in thaum, a goal reworded as a plan's default reached the goals home inside a batch of
other defaults. The rival, no row, leaves such an edit to be read by no step before it merges,
against `goal@knowledge-architect@the-owner-decides`. Per `design@agent-skills@goal-reviewer-agent`,
the agent is the same one goal-setting dispatches.
