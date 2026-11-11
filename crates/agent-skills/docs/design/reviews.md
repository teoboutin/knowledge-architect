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
`goal@documentation-stays-consistent`.

A deferred issue's trigger answers the question a tripwire's firing clause answers, what will make
someone do this, and the issue-tracking skill holds both to one test. So the reviewer reads every
deferred trigger against the change, as it reads every tripwire, and reports a trigger the change
meets as a finding, whose repair is the work the issue names or the owner's ruling. Without it, a
trigger met by work that does not go through the design skill has no reader at all.

### A fresh reviewer reads every plan document against the goals, and the design heads and the rejected alternatives of the Components it touches `##plan-read-against-the-record`

At every moment the installed planning skill sends a plan document's reviewers, it sends the
design-conformance reviewer beside the cold implementer, the code-claims reviewer, which also reads
a branch's diff before its merge, per `design@agent-skills@code-claims-scope`, and the transcript
reviewer. It reads the document against the goals, which the root's goals home holds, and against
the design homes and the rejected alternatives of the project's root and of every Component the
document touches. It reports a contradiction of a goal always, for the owner; a contradiction or a
widening of a head that the document's "What is already decided" does not list as reversed or
rewritten; a rejected alternative brought back without a recorded reopening; and an acceptance
criterion whose observable the record rules out. The second rule keeps it from reporting each
widening the document makes on purpose. It runs only commands that read the project.

`goal@agents-work-without-drift` has what agents build stay within the design
the project has recorded, a design not kept coming back only by a change that reopens it on
purpose. A plan document states what its work will build, so a reading of it against the record
before that work starts is where a departure is caught at the lowest cost. The owner named the
lack: no review read a plan document against the recorded goals and design. The other reviewers
read the document against itself, against the code, or against the discussion. The design audit
reads design homes, but not goals, and it is run by the session that implements the work, which
may be the document's author, and skipped for a spec worked in the session that converged. Two
acceptance criteria that `design@agent-skills@synthetic-evidence-not-built` rules out each passed a
plan document's three reviews in this repository. The deleted spec
synthetic-evidence-and-plan-reviews holds both, as its instances 1 and 2, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it. The decision's nearest rival, the
decision-record reviewer extended to plan documents, judges the record a diff writes and holds a
plan's items as not decisions yet, so it would carry two questions with two yardsticks. The cold
implementer, the other rival, asks whether the document can be acted on, and a document that
contradicts a goal can. The reviewer depends on a record worded wide enough. The owner stated where
a miss then belongs: every part of the workflow depends on a project keeping its goals and design
records with diligence, an owner still watches what the agents do and interrupts them where they
act against the owner's intent, and whether a miss is a defect of the published workflow or of the
project's own documentation is the retrospective's to judge. Its cost is one more dispatch at each
of those moments; what it buys is a reader of the record that neither the author nor the
implementing session is, which the two acceptance criteria above show the other reviews lacked.

### The design-conformance reviewer reads the verdicts of the work's record search beside the record, and is owed at a merge whose commit message carries a verdict other than `constrains` and `unrelated` `##verdicts-reach-the-conformance-review`

On the full path, the design-conformance reviewer reads the verdict files of the discussion's record
search, in the directory its brief names, beside the goals, design and rejected homes, per
`design@agent-skills@plan-read-against-the-record`, and reports a verdict the plan document leaves
unanswered. On the in-change path and for bounded work, the commit message carries the generated
lines of the grounding's search; at the review before the merge of a branch whose message carries
one with a verdict other than `constrains` and `unrelated`, the axis is dispatched on that message
and the branch's diff, runs `show` on each entry such a line names, reports a verdict the work leaves
unanswered, and reads as declared departures the heads the diff rewrites and those the message names
as reversed.

The session acts on each verdict without reading the entry by rule, per
`design@agent-skills@session-acts-on-the-verdicts`, and this reviewer already reads the record
against the work, so on the full path the verdict files add reading and no dispatch. The rival, the
verdicts recorded and read by no reviewer, leaves a verdict the session misjudged to surface only
after the work, the rework the search before the work exists to prevent, per
`design@agent-skills@record-searched-before-the-work`. A `constrains` verdict asks the work only to
keep within the sentence it quotes, and an `unrelated` one asks nothing, so a branch whose lines hold
no other verdict owes no dispatch for them. The cost is one more dispatch at the review before the
merge of a branch whose grounding's search gave a verdict that changes the work, outside a plan
document.

### Before every merge, the transcript reviewer runs alone and last, and not with the other axes `##transcript-review-last-before-merge`

The installed review skill sends the transcript reviewer before every merge to the main branch once,
after every other axis has run and its repairs are committed, and not also in their batch, over the
whole branch and the transcripts of every session that worked on it. The findings of the other axes
reach the session as messages of its transcript, so only a reviewer reading it after their repairs
can tell whether each was acted on durably, per
`goal@design-is-recorded-with-its-arguments`. A transcript reviewer sent in
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
`goal@design-is-recorded-with-its-arguments`: the last transcript review judges
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
which `goal@agents-get-a-complete-workflow` does not allow of a step every
design discussion runs.

The rival that keeps every agent from writing files lost: the searcher's own task is to fill its
report, and searchers told to write nothing wrote files all the same, so the instruction could not
be obeyed together with the task.

### Every installed agent holds `Write`, and its body, never its tool list, keeps it from editing the project tree `##agents-write-in-scratch`

Every installed agent's tools include `Write`, and the record searcher's `Edit` as well. Each
agent's body states that it edits nothing in the project tree and writes only in the scratch
directory its brief names, beside its rule that it runs only commands that read the project, where
it has one; the adversarial reviewer's mutations go in its own worktree alone. The reason a reviewer or an auditor edits nothing is that one that mutates the tree
corrupts what the other axes read, and a tool list cannot hold it: every installed agent holds
`Bash`, which writes anywhere. The nearest rival, a tool list without `Write`, prevents no write the
shell allows, and leaves an agent without the tool its own text asks it to use, as the design-record
auditor's instruction to write files in its scratch directory stood beside a tool list with no
`Write`. The owner argued that a restriction that prevents no action only surprises the agent it
applies to.

### A reviewer writes its report whole to the path its brief names, and the dispatcher builds the record of the review from the files `##reviewer-writes-its-report`

A reviewer the installed review skill dispatches, an installed agent or a general-purpose one,
writes its report to the path its brief names in its scratch directory, distinct for each reviewer
dispatched together; the goal reviewer does so wherever its brief names a path, as the review
skill's briefs do. The dispatcher builds the record of the review from the files and restates no
report; where a write was refused, the text the reviewer returned is the only copy, and the
dispatcher writes it into the file before acting on it.

A dispatcher that saved each report as it arrived, from the message alone, saved condensed copies,
or none, against the instruction to save it whole, in seven of the twelve sessions the deleted issue
on it recorded, which `git log --diff-filter=D --name-only -- crates/agent-skills/docs/open-issues/`
lists. Two other rivals that issue named lost. A script of the dispatcher over the harness's files,
the task output files, a saved workflow's journal or the subagents' transcripts, failed in two of
the five sessions that used it, two reports coming out empty in one, and a file giving the wrong
text twice in the other,
and a saved workflow's completion notice cut the joined reports short. A verb of the checker's
`transcript` command that writes a subagent's last message to a file, as
`design@agent-skills@transcript-extraction-is-the-checker-s` would place it, reads that message
back from a transcript format the harness owns, a step per report after the reviewer has gone;
the reviewer already holds its report and a tool that writes it.

### A subagent writes a report file with a shell redirect, since the harness refuses its file tool on a report file by its name `##report-written-through-the-shell`

A reviewer, a record searcher filling its `report.md`, and the report step of an audit's saved
workflow write their report file with a shell redirect, `cat > <path> <<'EOF'`, or, for the
searcher, by `Edit` of the skeleton, never by `Write`, and fall back to returning the report as text
where the harness refuses the write. An auditor's drafts are no report file of a name the
harness refuses, and an auditor writes them with a shell redirect only where the file tool is
refused.

The argument turns on the harness's behaviour. Claude Code refuses a subagent's `Write` of a new
`report.md`, of a new `findings.md` and over an existing `report.md`, with "Subagents should return
findings as text, not write report files. Include this content in your final response instead.",
and lets the same subagent `Write` a `review.md`, a `scratch.md` or a `report.txt`, `Edit` an
existing `report.md`, and write a `report-bash.md` with a shell redirect; the record searchers and
the reviewers of the sessions that built this wrote their `report.md` with a shell redirect, none
refused. One general-purpose agent sent to make each of these calls in a scratch directory takes the
measurement again; it was taken in the
auto permission mode, and the default mode is not established. Two rivals lost. A file name the
harness does not refuse depends on a pattern no documentation states, which a release of the
harness can change with no signal. A skeleton the dispatcher creates and the reviewer fills by
`Edit` costs a dispatcher step per reviewer; it and the shell both write past the refusal's intent,
and the owner chose the shell, which no goal ranks. No head argues the reliance on a mechanism of
the harness against its own refusal, per
`issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness`.

### A reviewer that writes its report file also returns the report whole as text `##reviewer-returns-its-report-whole`

The report file is the copy the dispatcher builds the record from, and the text returned is the copy
the session's transcript holds. The transcript reviewer finds the other axes' findings as messages
of that transcript, per `design@agent-skills@transcript-review-last-before-merge`, and its
extraction drops a tool result, such as the dispatcher's reading of a file, unless asked for it. The
nearest rival, a reviewer that returns only the path, keeps its findings out of the transcript the
last review reads.

### A review or an audit judges a text by the criteria that bind its writer at the time of the review, and by no other `##reviews-judge-by-the-writer-s-criteria`

A review or an audit the installed text owes or offers judges a text by the criteria that bind its
writer at the time of the review. Today those are the installed text; the project's root
`CLAUDE.md`, its skills, and the scoped `CLAUDE.md` beside what is edited; the goals and the record;
the external specifications that any of these make binding; and the plans the work answers, with
the owner's statements and rulings it carries out, as far as each binds the writer. A text written
outside any skill is judged by the same criteria. Every review and every audit takes its criteria
from the writer's, so the criteria of the reviews and of the audits agree with each other; for
agent-facing text they are one standard, per `design@agent-skills@admission-rules-home`.

`goal@agents-get-a-complete-workflow` has an agent carry each activity out with
the workflow and what its project adds, needing no instruction from outside what knowledge-architect
provides and what its project adds, so a criterion a reviewer brings from outside them is one the
writer could not have met. A writer bound by one
criterion and a reviewer that applies another are also two instructions that meet on one text with
no move that obeys both, which `design@agent-skills@no-two-installed-instructions-contradict`
excludes. The nearest rival is a reviewer with a standard of its own, stricter than the writer's or
the reviewer's preference, such as a style no criterion states. A finding it raises has no repair
that the writer's instructions give, so the next round raises it again. One was observed: a check
of scope that reported a clause added within the owner's ruling made a stricter acceptance
criterion a major finding, per `design@agent-skills@transcript-reviewer-agent`. Divergent verdicts
that a review standard and an audit standard give over one text are a defect provable by reading,
per `design@agent-skills@behaviour-additions-need-real-use`.

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
all of them, since each judges by the criteria that bind the writer, so that the criteria of the
reviews and of the audits agree, per `design@agent-skills@reviews-judge-by-the-writer-s-criteria`.

It is an agent, not a line in a skill, because its standard is fixed. Its extraction is the
checker's, per `design@agent-skills@transcript-extraction-is-the-checker-s`. It serves
`goal@the-owner-decides`, whose forks are the owner's, so a ruling misrecorded
decides in the owner's place. The durability check serves
`goal@design-is-recorded-with-its-arguments`. The check of rulings has shown its
worth: in the review of the change that installed the first skills, a reviewer briefed with it was
the one of six to find a decision recorded narrower than the owner's approval. The milestone
document of the v0.1 extraction holds that count, in its section on session B of step 5, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it; the message of the commit "[review]
Six reviewers read PR 1: the hand-off rule, the shipped text and the record are corrected" records
the review's six axes and its finding.

The design this decision did not keep:
`rejected@agent-skills@transcript-reviewer-reports-every-clause`.

### A session is extracted with the checker's `transcript show`, and no installed text holds an extraction rule of its own `##transcript-extraction-is-the-checker-s`

The transcript reviewer extracts each session with `klarch transcript show --from --full`, and the
subagent that assembles a plan document extracts as the reviewer's section says; the command labels
every line by what the harness recorded of it, per `design@transcript@transcript-labels-in-code`, so the installed texts state
what the labels mean and no rule for which lines to keep or whose they are. A rule held in the
reviewer's text, with the command as a helper, is applied by a script each session writes again,
and stands in two homes beside the command's, against `design@check@one-question-one-command`; a
fragment the build copies would still leave each session to write the script. It serves
`goal@agents-get-a-complete-workflow`.

### A transcript is found by the transcript command's listing, never by a session identifier `##transcript-found-by-listing`

The review skill, the planning skill and the transcript reviewer name a session by the opening words
of the message where its work begins, and find it with `klarch transcript list`: `--branch` lists
the sessions that worked on a branch, `--containing` the session holding those words in a message
the owner typed. No installed text finds a transcript by a session identifier. After `/clear`, the
process keeps the earlier identifier in its paths while the harness writes a new file, as
`design@transcript@transcript-session-split` measures, so a session that derives its transcript from the
identifier it sees reads the session before the clear. The same reason holds at the three texts,
which is why it has this home. The listing is computed from the transcripts, per
`design@transcript@transcript-query-verbs`.

### A misstated ruling is critical only when it reverses the ruling, or relaxes it in what is built or a load-bearing decision `##misstated-ruling-rated-by-what-it-changes`

A misstated ruling is critical only when it is reversed, or relaxed or put in a state that changes
what is built or a load-bearing decision. A decision attributed to the owner that the owner did not
make, and a ruling turned from its direction, are major. A ruling recorded a little relaxed, or
firmer than its hedges, on a detail that is not load-bearing, and an argument or a reading
attributed to the owner that is not the owner's, are minor and still go to the owner; a minor ruling
is no finding of the retrospective once caught before the merge; a refinement within the ruling's
direction is none at all, per `design@agent-skills@transcript-reviewer-agent`, except in a goal: a goal recorded stricter than what the owner ruled out knowingly is major: it rules out work the owner did not rule out, a change of the ruling's direction short of a reversal, and it reaches no work before the owner's next ruling on that goal shows it. The cut is what the misstatement would build: a reversal, or a
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

A row of `skill@knowledge-architect-review@review-axes` sends `agent@knowledge-architect-goal-reviewer` in its merge mode at a branch whose diff adds, rewords or removes a goal; an edit to a goals home's head alone changes no goal, so it owes none. It reads every goal the range adds or changes against the goal standard's rules that need no
statement of the owner, and against every goal of the tree, and reports each goal edit whose ruling
by the owner no commit message of the range records. A goal edit can be made inside another
activity: in thaum, a goal reworded as a plan's default reached the goals home inside a batch of
other defaults. The rival, no row, leaves such an edit to be read by no step before it merges,
against `goal@the-owner-decides`. Per `design@agent-skills@goal-reviewer-agent`,
the agent is the same one goal-setting dispatches.

### The shape of the code a branch writes is judged against the default shape by an installed reviewer, before the branch merges `##code-shape-axis`

`agent@knowledge-architect-code-shape-reviewer` judges each unit of code a branch writes or edits
against the default shape of `design@agent-skills@default-code-shape`, and each test it adds by
`design@agent-skills@test-economy`, in the review before the merge, so every branch's code is
weighed against the shape before it merges.
`goal@the-workflow-leads-to-better-code` has what the workflow says about code
and tests aim at code of better quality, and the default shape is what it says about the shape of
code. The nearest rival, the shape applied by the session that writes the code and weighed by no
review, applies it only as far as that session did: a departure the writer did not see, or saw and
did not argue, merges with no finding. A general quality reviewer has no standard to judge against, and a
general-purpose reviewer briefed with the standard judges unevenly, per
`rejected@agent-skills@one-generic-auditor-agent`, so the axis is an installed agent, per
`design@agent-skills@additions-review-agent`.

### Where a project implements an external specification, an installed reviewer works out what it requires before reading the code `##spec-first-axis`

`agent@knowledge-architect-spec-first-reviewer` finds what the specification requires for a
change's claims, reads the departures from it the project states, and only then compares the code,
at development checkpoints where the change touches how the project implements it. A reviewer that
reads the code first reads the specification through it. It judges the code against the
specification as the project follows it, per
`design@agent-skills@reviews-judge-by-the-writer-s-criteria`.
A project with no specification never dispatches it, so it costs such a project nothing. It is the
generic form of thaum's rules-first axis. A general-purpose reviewer briefed with the procedure would
judge unevenly, per `rejected@agent-skills@one-generic-auditor-agent`, so it is an installed agent.

### Adversarial reviewers are scoped to one subsystem, family of claims or boundary each, pick their tests, and run the wider suite before reporting a survivor `##adversarial-scoping`

`agent@knowledge-architect-adversarial-reviewer` chooses its own mutations within the scope its
brief names, runs the tests that bear on each, and runs the package's wider suite before it reports
a mutation as surviving; several may be dispatched, their scopes overlapping, a gap the dispatcher's
to justify. It has no cap on its running time. The owner reported, from thaum, that scoping reduced
the time a reviewer takes enough to fix its running time, with no cap that might stop it finding
more; whether scoped reviewers still run unbounded in real use is what a retrospective would report.
The rival, a cap on a reviewer's time, would stop the reviewer at an arbitrary point of its attack.

### An axis for code whose late finding would unbuild work runs at each development checkpoint, and the others once per branch, before the transcript reviewer `##review-cadence`

The adversarial and spec-first axes run at each development checkpoint, since a defect they find
late, in the code's behaviour, would unbuild work built on it. The code-shape and code-claims axes
judge what can be reworded or reshaped without unbuilding, and run once per branch, in the
review before the merge and before the transcript reviewer, per
`design@agent-skills@transcript-review-last-before-merge`. Every axis at every checkpoint would
multiply the dispatches, against the efficiency `goal@agents-get-a-complete-workflow`
asks of the workflow; the instrument that would show whether two more per branch are too many is
`issue@agent-skills@the-retrospective-counts-no-review-cost`.

### A code axis dispatched once per branch is a row of the review skill's table, and one dispatched at checkpoints a row of the development skill's `##code-axes-rows-home`

A diff of any activity can write code or a statement about code, a plan's harvest or an issue's
closure among them, and the review before the merge reaches each, so the code-shape and code-claims
rows sit in `skill@knowledge-architect-review@review-axes`, for the reason
`design@agent-skills@review-judges-additions` gives for the agent-facing-text row. The adversarial
and spec-first axes are dispatched at moments only the development activity names, so they are rows
of `skill@knowledge-architect-development@code-review-axes`, which the review skill's sentence on an
activity's added axes reaches. Each row has one home.

### The code-claims reviewer reads every statement a diff writes about code, and the completeness of a changed public interface's documentation `##code-claims-scope`

`agent@knowledge-architect-code-claims-reviewer` reads, beside a plan document's statements, every
statement about code a branch's diff writes, a doc comment, a README, a design head, an issue entry,
a commit message, against the tree, except the changelog's entries, which the release's review
reads, per `design@knowledge-architect@changelog-reviewed-at-the-release`. Where the diff changes a
public interface others consume, it asks of each public item the diff adds or changes whether its
documentation states what a caller must respect. The same method reads both, at no extra dispatch,
and a thaum case where an improvised reviewer for this question produced the review's most
consequential findings showed the gap; that the documentation exists is the lints' of
`design@agent-skills@documentation-presence-is-linted`.

### The review owes no security axis: the adversarial axis attacks accepted inputs, and `unsafe` code is a lint gate's `##no-security-axis`

No installed agent reviews security as an axis of its own. The adversarial reviewer already tries to
reach a panic, a wrong result or a lost input from an accepted input, and code marked `unsafe` is a
lint gate's to refuse, not a reviewer's to judge. The setup asks every crate of a target project
for a lint on `unsafe` code, in `skill@knowledge-architect-setup@rust-project`, so the clippy gate
refuses an `unsafe` block nobody argued.
