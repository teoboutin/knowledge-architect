# Agent skills design: Reviews

## Reviews

### The standing-state review runs before every merge, and reads every tripwire and every deferred trigger `##standing-state-review-before-every-merge`

The installed review skill sends the standing-state reviewer before every merge to the
main branch, whatever the change. That reviewer is the standing re-entry point of every tripwire
home and of every deferred trigger, and asks of each tripwire whether it fired and whether its
horizon passed, per `design@agent-skills@tripwire-horizon`, as the installed issue-tracking skill states: it runs for every
change, where the search before the work runs only for work that goes through the design skill, a
plan or a retrospective intake, per
`design@agent-skills@record-searched-before-the-work`. A re-entry point that depends on whether a change looked
related to a tripwire is one that a change touching the guarded decision indirectly skips: the
reviewer reads every entry of every home, not the subset the diff seems to concern. This serves
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

The decision serves `goal@knowledge-architect@agents-work-without-drift`, which is met when "a
departure from the recorded design is caught by a check or a review before it merges". The owner
named the lack: "nothing seems to be reviewing a plan document against recorded goals and design."
The other reviewers read the document against itself, against the code, or against the discussion.
The design audit reads design homes, but not goals, and it is run by the session that implements the
work, which may be the document's author, and skipped for a spec worked in the session that
converged. Two acceptance criteria that
`goal@knowledge-architect@the-workflow-improves-through-real-use` rules out each passed a plan
document's three reviews in this repository. The deleted spec synthetic-evidence-and-plan-reviews
holds both, as its instances 1 and 2, and `git log --diff-filter=D --name-only -- docs/plans/`
finds it. The decision's nearest rival, the decision-record reviewer
extended to plan documents, judges the record a diff writes and holds a plan's items as not
decisions yet, so it would carry two questions with two yardsticks. The cold implementer, the other
rival, asks whether the document can be acted on, and a document that contradicts a goal can. The
reviewer depends on a record worded wide enough. The owner's words state where a miss then belongs:
"this is always dependent on the project being diligent in its goals and design records. Everything
in the workflow is, after all. Project owners still have to watch agent behavior and interrupt them
if they find them acting differently than their intent. Whether this is a published workflow defect
or an internal documentation defect is left to the retrospective skill to judge." Its cost is one
more dispatch at each of those moments; what it buys is a reader of the record that neither the
author nor the implementing session is, which the two acceptance criteria above show the other
reviews lacked.

### Before every merge, the transcript reviewer runs alone and last, and not with the other axes `##transcript-review-last-before-merge`

The installed review skill sends the transcript reviewer before every merge to the main branch
once, after every other axis has run and its repairs are committed, and not also in their batch, over the whole branch and the
transcripts of every session that worked on it. The findings of the other axes reach the session as
messages of its transcript, so only a reviewer reading it after their repairs can tell whether each
was acted on durably, per `goal@knowledge-architect@design-is-recorded-with-its-arguments`. A
transcript reviewer sent in their batch as well would read the transcript before the owner answers
the items the batch raises, and an answer of the owner given after its extraction reads to it as
missing, as `skill@knowledge-architect-review@review-axes` says: its findings would report as
missing what the last transcript review then finds answered. Its own repairs land as
`design@agent-skills@transcript-review-repairs-are-additional-commits` says.
An additional commit that adds, removes or reverses a design head is reviewed by the
decision-record axis at least, and the review ends with that review's repairs; any other is reviewed
by no axis again, so the review ends, per
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.

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
is reviewed after it, and the review ends with the repairs of that decision-record review. Any other repair is reviewed
by no axis again: a rewording, and an argument, a mention or a reference added inside an existing
head, included. One made before the last transcript review is still read by it, as every commit of
the branch is. Without the record review, a head added, removed or reversed in answer to a review
reaches the main branch with no review of its record, against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: the last transcript review judges
fidelity to the owner's rulings and not the record.

The trigger is a whole head, which the diff shows: a heading with a slug that appears or
disappears, or a reversal under `skill@knowledge-architect-decision-recording@reversal-check`. Each round of repairs then adds at most as
many re-reviews as it adds, removes or reverses heads. A trigger on any repair that "makes a
decision" lost: whether a rewording makes one is a judgement, and read wide it re-reviews repairs
that only reword a head. The rival that sends every axis again at every repair lost on the same
cost. A narrowing of an approved head that this trigger leaves out still goes to the owner, per
`skill@knowledge-architect-decision-recording@head-shape`, and the last transcript review checks it against the owner's rulings.

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
applicable to every diff that adds to or changes it, installed or a project's own, whatever
activity produced the diff. It judges each added or changed statement against
`skill@knowledge-architect-agent-configuration@admission-standard`, the standard every writer and
judge of that text applies, per `design@agent-skills@admission-rules-home`. No other axis judges
whether a statement is needed: the routing axis judges where it goes, and the reviewer agents leave
style aside. In the thaum session that wrote the statement the owner judged useless, five review axes passed
over it, by the message of thaum's commit that added it. The owner holds the review of a diff the cheaper place to catch such a statement,
against the cost of an audit.

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
