# Agent skills design: The audit method

## The audit method

### Reading a whole project on one aspect is an activity with an installed skill of its own, and a review stays the reading of a diff `##audit-is-an-activity`

A project audit reads every entry or text of a corpus on one axis and brings what fails into line,
under `skill@knowledge-architect-project-audit`, which holds the method, the outcomes and one
section per axis; a repair of agent-facing text is judged by the standard of
`design@agent-skills@admission-rules-home`. Its artifact is a repaired record across the project,
where a review's is a verdict on one branch, so the two hold different standards: what a diff owes,
and what the whole record owes. The nearest rival, a whole-project mode of the review skill, would
hold both standards in one text. A skill per axis would restate the method in each, against
`goal@agent-skills@one-skill-per-activity`, while the method is generic and each axis gives only its
corpus, its dispatch and its outcomes, per `design@agent-skills@audit-method`. Every review reads a
diff, so without an audit a drift between two heads no branch touches is read by no step of the
workflow, against `goal@knowledge-architect@documentation-stays-consistent`. The goals are the
exception: their review of the whole set reads the whole project on one aspect and runs inside
goal-setting, since its methodology is goal-setting's and an activity is its methodology, per
`design@agent-skills@goal-set-review-in-goal-setting`; the argument above, two standards in one
text, does not reach one standard that serves a draft and a whole set.

### An audit's method holds for every axis, each axis giving its own dispatch `##audit-method`

The method of `skill@knowledge-architect-project-audit@audit-method` holds for every axis. An axis
gives what varies: its corpus and what is read beside it, its dispatch, its drafts, its outcomes,
and how its findings are classified as on old or new text. Its dispatch, its drafts and the reading
of them are its saved workflow's, per `design@agent-skills@audit-axis-runs-a-workflow`, and each
pass is one run of it, with one more on the application where the axis judges it, per
`design@agent-skills@one-run-pass`. Each agent writes its drafts in a
scratch directory of its own and edits nothing else, and nothing an agent proposes is applied before
a reading by the session confirms it, per `design@agent-skills@synthetic-evidence-not-built`. A
quotation or an attribution of the owner is confirmed against the question it answered, as well as
against its words.
The items that need the owner's word go on one list per pass, labelled `F<n>`, in the pass's report
file, per `design@agent-skills@committed-audit-report`. The edits are applied and their diff read
whole. How passes follow one another is
`design@agent-skills@audit-passes-repeat-while-the-count-falls`.

The reading guards against a quotation of the owner verbatim but taken from the answer to another
question, which the first audit of the design-record axis on this repository showed. The rival that
keeps the design-record axis's groups and calibration sample in the method, a later axis adjusting
them as an exception, would state a rule that one of its axes breaks whole: the agentic-workflow
axis shares no corpus out, per `design@agent-skills@workflow-axis-lenses`. The groups and the
sample are the design-record axis's dispatch, per `design@agent-skills@design-record-axis-dispatch`.
No point of the method serves an axis with no saved workflow: every axis has one, and a new axis is
designed with its own. The rival that keeps such points for a future axis keeps text no session
follows, since the fallback for a harness without the tool is the script's own steps.

### Fresh agents re-check each pass of an audit, and passes repeat while the violations left fall, until the owner stops them `##audit-passes-repeat-while-the-count-falls`

After a pass's edits, the axis's saved workflow runs again, given the kept list and the commit the
previous pass audited, and its findings start the next pass. Passes repeat while the violations
left fall from one pass to the next, until the owner judges what is left small enough. A violation
left is a finding of a re-check that a reading confirms, that is on no kept list and that no issue
entry records. Which of them the count reads is `design@agent-skills@convergence-count-scope`. A
pass where that count does not fall goes to the owner as a question, with the counts, and starts no
pass on its own. The stop rule is the owner's, given in place of a fixed threshold: "As long as we
observe a "convergence", and the amount of leftover violations is small enough, I think that is
good. If you ask an LLM to find a flaw or defect in something, it will nearly always find one IMO.
Especialy if that something is prose and not code."

The rival, a lighter method with no re-check, lost on real audits, whose commits
`git log --grep='design-record axis'` finds: in the first audit of the design-record axis on this
repository, the re-check of each pass found violations the pass had missed or made.

The design this decision did not keep: `rejected@agent-skills@audit-without-calibration-or-recheck`.

### The convergence count of an audit reads only the violations left on text the previous pass also read `##convergence-count-scope`

A violation left is on old text when the previous pass also read its words and no commit after the
one that pass audited changed them; a move, a reflow or a rename leaves text old. Any other is on
new text, the previous pass's repairs among it. The stop rule of
`design@agent-skills@audit-passes-repeat-while-the-count-falls` reads the count on old text. A
violation left on new text takes its outcome like any other, and decides nothing about convergence.
Each axis's section says how its findings are classified.

The count reads old text because each pass's repairs write new text, and the agents find defects in
new text at least as readily as in old. On the fifth pass of the agentic-workflow axis on this
repository, 30 of its 38 findings sat on words the branch that built the axis had written, 14 of
them by the change that made a pass one run, classified by `git blame -w -C` on each defect's line
at the commit audited, with the diffs read by hand where blame and the words disagreed. The plan
document of that work holds the classification, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it. A count over the whole corpus then
compares two corpora, and measures how much text the last pass wrote more than how much defect is
left. The nearest rival, the plain count with its fall judged over its context, was argued by hand
at each firing, with the same explanation each time. Dropping the count for the owner's judgement
alone drops the convergence signal the stop rule reads.

The design this decision did not keep: `rejected@agent-skills@plain-confirmed-findings-count`.

### A pass of an axis is one run of its saved workflow, from its preflight to its report, with one more run to judge its application where the axis has one, and the session reads each report after its run, never between its steps `##one-run-pass`

One run dispatches every agent of the pass and delivers one report, and the session reads nothing
between its steps. The saved workflow is their one home, per
`design@agent-skills@audit-axis-runs-a-workflow`. For the agentic-workflow axis the steps are the
preflight, the auditors' drafts, the verify step of `design@agent-skills@verifier-in-the-run`, the
dedup into the confirmed findings, the cluster and the merge into causes, the repair proposals and
their checks, per `design@agent-skills@root-cause-repair`, the cross-check, and the report. For the
design-record axis they are the preflight, a setup agent, the auditors' drafts, the calibration of
`design@agent-skills@calibration-in-the-run`, the verify step, the checks of
`design@agent-skills@created-heads-checked-in-the-run`, the cross-check, and the report. Every step
writes its files in the run's scratch directory, so the session can open any draft, grouping or
proposal the report cites. The run sets itself up inside it: the listings the agents read, and a
worktree of its own for each agent dispatched alongside others, as
`skill@knowledge-architect-review@review-invariants` asks; an agent that runs after another may work
in its worktree, as a verifier works in its auditor's. The preflight creates them where the script
names the agents from its own table; where the agents follow from what the preflight lists, as the
design-record axis's groups do, a setup agent creates their worktrees once the script has cut them.

The cross-check and the report are one agent each, two in all, on the owner's word. The cross-check
maps the findings that edit one
paragraph or one slug, and proposes each finding's outcome for the session to decide; in the
design-record axis it also lists each tripwire and issue an edit would leave false, read whole. The
report quotes what the session needs to discuss each finding with the owner and act: the refuted
drafts with their reasons, the run's counts, each finding or cause with its quoted evidence and
verdicts, its edit before and after, its check and a proposed outcome, the overlap map, a draft of
the owner list, and what the run did not decide.

After the run the session verifies the report, as the axis's section of the audit skill says, since
the owner list goes out under its responsibility: the report whole, every finding the report marks
as plausible or as bearing on a ruling of the owner or on a tripwire, against its draft, and a
sample of the refuted drafts, with what the axis adds: every repair proposal in full for the
agentic-workflow axis; the calibration's readings, every check and a sample of the conforming drafts
for the design-record axis. The samples are the session's to choose, on the owner's word that leaves
the sample to judgement. The owner list, the owner's word and the application follow. The run ends
at its report, and the application is the session's, per
`design@agent-skills@apply-stays-with-the-session`. The design-record axis then runs its workflow
once more on the application, in the mode `application`, per
`design@agent-skills@application-judged-by-diff`: the owner's ruling and the session's application
sit between the two runs, so no single run can hold both.

The shape is the owner's: the main power of a saved workflow is to organize several complex steps
into one complete result, such as a report, as ready as possible for the main session to discuss
with the owner and then act on, with the least burden on the main session; and verification by the
session is necessary, but not in the middle of the workflow's steps.
The nearest rival, the steps run as separate runs with a reading by the session between each, lost
on the pass of the agentic-workflow axis that ran so: the session's sort refuted 3 drafts, each a
refutation a verifier given the audit's earlier commit messages can make, its reading of 23 repair
proposals sent none back, by the session's own account, which no file records, and the readings
cost the session most of its context. The rival that has the session read every draft again after
the run lost on the same measurement, since the verifier makes the refutations that sort made. The
plan document of that work holds the measurement, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it.

The designs this decision did not keep: `rejected@agent-skills@three-runs-per-pass` and
`rejected@agent-skills@session-rereads-every-draft`.

### The session applies the repairs the owner approves, and no saved workflow does `##apply-stays-with-the-session`

A run of an axis's saved workflow ends at its report. The session applies the edits the owner
approves, itself or through agents dispatched on disjoint files, and reads the whole diff before
each commit, as `skill@knowledge-architect-project-audit@audit-method` says.

The nearest rival, a second saved workflow that applies the approved repairs, lost on the merges
the application needs. In the two applications that agents made on the agentic-workflow axis on
this repository, the repair of passes 2 and 3 and that of pass 4, the applying agents merged edits
that several repairs made to one paragraph, six times, three in each, by their reports, which no
file records. A workflow would make those merges against a tree that may have moved since the run
read it. Two applications are the evidence, so the decision is held loosely.

### An audit axis's steps live in a saved workflow of its own, their one home for any harness, and its section of the skill keeps what the session needs to verify the run's report `##audit-axis-runs-a-workflow`

An axis's saved workflow holds its dispatch, the prompts of its steps and their order, and covers
that axis alone. It ships in the installer's namespace, and the harness runs it by the name its
`meta` declares, per `design@agent-skills@saved-workflow-name-is-its-file-stem`. The axis's section
of the audit skill names the workflow and the `args` it takes, and keeps what the session needs to
verify a report and write the owner list: the axis's corpus, its rules, what the session's
verification reads, and its outcome table. The axis's agent states the standard its steps judge by,
and no dispatch. A session whose harness does not list the workflow dispatches its steps by hand,
in the script's order, with the script's prompts, read from the script. The agentic-workflow axis
runs `knowledge-architect-agentic-workflow-audit`, the design-record axis
`knowledge-architect-design-record-audit`, and the tripwire axis
`knowledge-architect-tripwire-audit`.

The shape is the owner's: the owner judged it the generic shape of every audit axis, each with its
own workflow producing a report ready to discuss and act on, which also keeps the project audit
skill concise. The delivery as a file of the harness's directory is the owner's too, as a file of
.claude/workflows/ named with the installer's prefix.

The argument turns on the behaviour of Claude Code's Workflow tool, read in its description and its
authoring reference and measured by probes, which the message of the commit "The agentic-workflow
axis's script ships as a saved workflow in the installer's namespace" records. The tool runs only
on an explicit opt-in, such as the
owner invoking a skill whose instructions tell the session to call it, or asking for a saved
workflow by name. A script reads no file and takes its inputs in `args`. A saved workflow runs by
the name its `meta` declares, only from a file directly under the workflows directory, and the
harness lists it with the skills. So the order of the steps and every agent's inputs are fixed by
the script and versioned with it, and the session copies nothing to run it; a script in a fenced
block, passed to the tool inline, would sit in a skill as text no session needs to read. The rival
that keeps the steps in prose beside the script, or in one markdown file per axis for a session
without the tool, keeps two homes for one moment, and one drifted: a repair prompt of the script
narrowed what the skill's prose said, "the homes merged into one", to "the section of the skill
that owns the moment". That the checker reads markdown and no JavaScript bears on references, and
the script cites none. A file per axis stays available for reading guidance that is not a step.
The rival of one workflow auditing every axis at once lost on the owner's
words: "it audits all axes of the project at once and produces a single report, which is too large
of a scope".

No goal binds the audit to one harness: the goals that name a harness speak of the harness that loads the installed text and of a manifest's list of harnesses, never of one harness; `goal@agent-skills@installed-text-works-in-every-target-project` names a project's paths, conventions and command, and
`goal@agent-skills@installed-text-works-in-any-environment` lets the installed text rely on the
harness's own mechanisms while no skill or instruction the environment adds takes the place of an
installed one. A second provider stays a design target, per
`issue@core@configuration-for-several-agent-providers`, and the dispatch by hand keeps it open. The
rival of a skill written for the Workflow tool alone fails in every other harness, for a gain the
dispatch by hand already gives. The owner's weighing of the dependency: "After all, I use only
Claude Code personally... I'm not really expecting anyone to start using this, outside of myself in
more projects than thaum." The cost of shipping them: each saved workflow is listed in every session
of every project that installs the workflow, one that never audits included.

The designs this decision did not keep: `rejected@agent-skills@manual-fallback-file-per-axis` and
`rejected@agent-skills@workflow-script-inline-in-skill`.

### Each audit pass commits a report file holding its evidence and the owner's rulings, on every axis, and the file leaves once its outcomes are carried out `##committed-audit-report`

The file is `docs/audit-reports/<YYYY-MM-DD>-<axis>-pass-<n>.md` at the project's root, the axis
named by its section's slug, per `skill@knowledge-architect-project-audit@record-of-an-audit`. The
session writes it after its reading of the pass: the run's report as its verification left it. The
owner list closes it, each item ending with a `Default` line
and an `Outcome` line, which the owner writes, or the session writes with the owner's words
verbatim. The file is committed with every `Outcome` line before any edit of the pass is applied.
The commit that carries out its last outcome, or a later commit of the same branch, deletes it, and
its message names it by its path. Each pass's commit message keeps the counts and one line per
outcome. The primer's knowledge table carries its row for every project, per
`design@agent-skills@knowledge-table-home`. A review of the audit's branch is told each report file
the branch added and deleted, with the commit that holds it, since a reviewer of the branch's tip
does not see a file the branch deleted.

The proposal is the owner's: "I think it might be a good thing for the audit report to be kept in
history, committed somewhere (maybe docs/audit-reports) and deleted once acted upon, just like retro
intake reports.", and so is its scope, every axis: "every axis for the report". A working file of
the session is lost with its scratch directory, and the evidence of a pass, its drafts and their
quoted texts, was needed after the pass, to classify its findings again as on old or new text. The
nearest rival, the commit messages alone, keeps the outcomes and loses the evidence, and a commit
that records rulings and changes no file is dropped by GitHub's rebase merge, as
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file` measured. History grows
by the report's size at each pass, and the tree by nothing once the file leaves.

The design this decision did not keep: `rejected@agent-skills@commit-messages-as-audit-record`.

### Every finding of an audit takes exactly one outcome of its axis's table, and none is left to judgement `##audit-outcomes`

Every finding takes exactly one outcome, from the generic table of
`skill@knowledge-architect-project-audit@finding-outcomes` as its axis's section fills it: applied
in the branch, put to the owner, sent upstream, an issue entry, or nothing. An edit that changes no
ruling of the owner is applied in the audit's branch: a rewording, a tense, a reference, a split
that moves no word of the owner, a rejected alternative repaired in place, a factual error, a
pointer whose target exists under another name, a drifted restatement brought back to its home, an
instruction read two ways reworded to the reading a design head or another instruction states. An
approval cited as a head's ground is removed in the branch where the head stands on its argument, or
recorded as the session's choice at a fork, and the owner's pick at a fork cited as a ground is
recorded in the weak form, per `design@agent-skills@fork-choice-in-weak-form`, since each repair
removes a false ground and relaxes nothing. A change that relaxes what the owner's words approved or
turns it from its direction, per `design@agent-skills@transcript-reviewer-agent`, the removal of an
entry, but for a tripwire the standard or its lifecycle removes, per
`design@agent-skills@tripwire-stands-on-the-standard`, an intent of the owner cited as a ground,
drafted as a goal, which side of a contradiction wins where no design head, goal or source settles
it, a verdict where a rule reads two ways and no record states the reading, a head
in the wrong Component and a conflict with a goal go to the owner, per
`goal@knowledge-architect@the-owner-decides` and
`design@agent-skills@questions-go-to-their-authority`; any finding no case names goes there too. In
the agentic-workflow axis, an instruction read two ways goes to the owner where nothing recorded
states the reading meant, and is applied where a design head or another instruction states it. A
predicted gap, per `design@agent-skills@behaviour-additions-need-real-use`, becomes an issue on the
project's own text and is sent upstream on the installed text. An unneeded statement, per
`design@agent-skills@statement-names-an-act`, takes the outcome of the table of
`skill@knowledge-architect-agent-configuration@admission-standard`: deleted in the branch where it
carries no instruction, moved to a `%%` comment of the installed source where only its maintainer
reads it, and, where it is an instruction no agent performs, a decision, which the audit's
outcome table routes: deleted where the record settles that its act is no longer wanted, and
otherwise put to the owner. A review applies the same table, so the two judge one statement alike,
and routes the decision by `primer@question-authority` to the same two outcomes. A provable gap is filled in the branch, from the fact it
names, per `design@agent-skills@behaviour-additions-need-real-use`. Work too large for the branch and a finding the
owner defers each become an issue. A finding outside the axis takes the first case of
`primer@met-outside-the-task` that applies. A finding that an issue entry already records, or that
is on the kept list, takes no action.

A finding on the installed text is never edited in a project that installs it, per
`skill@knowledge-architect-agent-configuration@installed-files-never-edited`. It is sent upstream,
in one file per audit, written when the audit stops from the report files of its passes, in the
shape of the file in which a retrospective sends the workflow's findings, so it reaches the project
that publishes the workflow by the same route, per `design@agent-skills@retrospective-destination`.
A finding of the interaction of the installed text and the project's own is split into its side on
each text, each side with one outcome, so a finding still takes exactly one. The installed text
holds no branch for the project that publishes the workflow, which edits the installed text at its
source by its own instructions.

No outcome leaves a confirmed finding to judgement.
`goal@knowledge-architect@agents-get-a-complete-workflow` asks that "Each installed instruction says
what it asks plainly enough to follow without guessing what it means", so an instruction that a
reading confirms is read two ways is repaired, even where every act it allows is acceptable. What no
instruction covers is left to judgement on purpose, and is no finding. The rival outcome, a finding
of two readings left to judgement where no text rules either act out, leaves such an instruction
standing against that goal. Its cost stands: each repair writes new text, which
`design@agent-skills@convergence-count-scope` counts apart and
`design@agent-skills@repair-proposals-audited` reads before the repair lands.

Most findings of the design-record axis are text edits, so they are applied at once: the owner
thinks the goal for that axis is usually immediate application, with little reason to delay since
those are mostly text edits, unless some design entries have major issues that bear on the
project's goals, for example.

### One adversarial verifier per auditor tries to refute that auditor's drafts inside the run `##verifier-in-the-run`

The verifier is the axis's own auditor under a verify brief, per
`design@agent-skills@audit-agent-per-axis`, in its auditor's worktree. It checks every quotation
against the tree, a quotation or an attribution of the owner against the question it answered, and
that what a draft describes exists. It marks known a draft whose finding an issue entry records, and
kept a draft on the kept list. It marks a finding that a ruling's premise has proved false as
bearing on that ruling, and leaves it to the session. Its verdict per draft is verified, refuted or
plausible, with a reason, and a refuted draft is kept with its reason, for the session's sample. It
reads the kept list, the issues listing and the record of the audit's earlier passes, their commit
messages and their report files, so it can make the refutations that rest on the audit's history.

In the agentic-workflow axis, each auditor's drafts go to its verifier as soon as the auditor
returns, and the verifier splits a finding of the interaction of the installed text and the
project's own into its two sides, per `design@agent-skills@workflow-auditor-agent`. In the
design-record axis, the verifiers start after the calibration, which needs every group's drafts,
and each judges again, under a reading the calibration proposes, its group's drafts that reading
bears on, per `design@agent-skills@calibration-in-the-run`. A design-record verifier judges only
the draft of record of a sample entry, its own group's; it reads a sample of the drafts that find
their entry conforming, at least one, and where one is wrong it reads again that auditor's other
drafts on the same rule; and on a re-check it classifies each finding as on old or new text.

One verifier per auditor reads the corpus once per auditor and sees that auditor's drafts together,
so the drafts that share a wrong draft's class are read by the same agent, against the same
corpus, in one verify step. The nearest rival, one verifier per draft, reads the whole corpus once
per draft: at the fourth pass of the agentic-workflow axis on this repository, 63 drafts against 16
auditors. The plan document of that work holds the measurement, and
`git log --diff-filter=D --name-only -- docs/plans/` finds it. Nothing a verifier passes is applied
before the session's verification, per `design@agent-skills@one-run-pass`.

### An audit runs on an occasion whose own work needs the record read under the rules, never on a period `##audit-triggers`

The owner's request is the main occasion. A version that changes the rules recorded content must
meet cites the section that brings a project's record to them in its Migration entry, an audit
axis's or the goal-setting skill's review of the whole goal set,
`skill@knowledge-architect-goal-setting@whole-set-review`, per
`design@knowledge-architect@changelog-entries`, and `skill@knowledge-architect-setup@moving-the-pin`
runs either in a branch of its own once the pin move has merged;
`skill@knowledge-architect-setup@existing-documentation` runs the design-record axis at the end of
the work that moves a project's documents into the homes. Each occasion is one whose own work needs
the record read under the rules. The rival, a periodic audit, has no evidence for a period, and
`design@agent-skills@behaviour-additions-need-real-use` asks for one.

### Each audit axis has an installed agent of its own, which carries the axis's standard `##audit-agent-per-axis`

The agent's definition reaches a subagent from its first token, so the axis's standard does not
depend on what a brief asks it to read, and a second axis adds an agent rather than a branch inside
one text. Its read-only rule is in its body, since its tools include a shell. The rival, one
generic auditor briefed to read the axis's section of the skill, puts the standard behind a read the
brief must ask for.

The design this decision did not keep: `rejected@agent-skills@one-generic-auditor-agent`.

### A project audit repairs the whole record at once, on each occasion an audit runs `##audit-repairs-the-whole-record`

Beside the repair of a head a change touches, `design@agent-skills@existing-heads-on-touch`, a
project audit brings every head and every rejected alternative to the rules at once, under
`design@agent-skills@audit-method`, a change that relaxes or turns the owner's words going to the
owner on one list, on the occasions of `design@agent-skills@audit-triggers`. Its premise is the
owner's: with the changes then made to the conditions and the shapes of decision records, the owner
predicted that projects migrating at the next pin would need to bring their design homes to them, a
need each later change of those rules renews. A repair on touch never reaches a head no change
touches. The rival, the repair on touch alone, leaves a project's record under the old rules after
its migration.

The design this decision did not keep: `rejected@agent-skills@head-repaired-only-when-touched`.
