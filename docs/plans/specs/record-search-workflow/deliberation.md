# The threads and arguments of the spec on the record search's saved workflow

## Threads

### The search runs one saved workflow for every occasion, its flags in `args` `##search-runs-a-saved-workflow`

Proposed by the session in the reply to round 1, on the owner's request of round 1 that the search
become a workflow and that the session do as little as possible itself. Final state: argued.
Arguments: `argument@record-search-workflow@arg-1`, `argument@record-search-workflow@arg-3`,
`argument@record-search-workflow@arg-4`, `argument@record-search-workflow@arg-5`,
`argument@record-search-workflow@arg-27`. Shape: Decided design, "The search runs one saved
workflow, for every occasion". Harvest: `design@agent-skills@one-record-searcher` rewritten, and a
head in the agent-skills design home where the entry tests admit one. Closed in the reply to round
3, on the argument that the rewording rival still leaves 22 to 36 hand dispatches; the owner
approved the design as displayed, round 4.

### An agent at low effort runs `worklist --out` and returns the agent directories `##workflow-runs-the-worklist`

Proposed by the owner in round 1, as a question whether the workflow could call the command that
generates the work list itself; shaped by the session in the reply to round 1. Final state: argued.
Arguments: `argument@record-search-workflow@arg-1`, `argument@record-search-workflow@arg-2`,
`argument@record-search-workflow@arg-6`, `argument@record-search-workflow@arg-7`. Shape: Decided
design, "An agent of the workflow runs `worklist --out`, and the workflow returns its notices".
Harvest: the head of the workflow. Closed in the reply to round 3 on its argument; extended by
#worklist-notices-returned.

### The worklist step returns the notices `--out` prints, and the workflow returns them `##worklist-notices-returned`

Proposed by the session in the reply to round 3, from the experiment's refutation of the verdict on
`design@agent-skills@design-audit-checks-fit`. Final state: argued. Arguments:
`argument@record-search-workflow@arg-20`, `argument@record-search-workflow@arg-24`. Shape: Decided
design, "An agent of the workflow runs `worklist --out`, and the workflow returns its notices".
Harvest: the head of the workflow. Closed in the reply to round 3, on the fact that `--out` prints
its notices on stdout only. A thread on a subject the search covered, so it owed no search of its
own.

### The workflow writes the text reports, runs `--verify` and sends each failing report back, for at most `maxRounds` rounds `##report-loop-in-the-workflow`

Proposed by the session in the reply to round 1. Final state: argued, with a clause from the
premortem. Arguments: `argument@record-search-workflow@arg-8`, `argument@record-search-workflow@arg-9`,
`argument@record-search-workflow@arg-27`, `argument@record-search-workflow@arg-35`. Shape: Decided
design, "The workflow writes the text reports, runs `--verify`, and sends a failing report back".
Harvest: the head of the workflow. Closed in the reply to round 3 on its argument; the value 3 of
`maxRounds` is D1, put as open to the owner's word in the reply to round 3, which the owner approved
with the design, round 4. The premortem of the reply to round 4 added the clause that each text
report is written by an agent of its own.

### A passing `--verify` writes every report's closing sections into one file `##closing-sections-file`

Proposed by the session in the reply to round 1, as shape 1 of
`no-command-gathers-the-closing-sections-of-the-searchers-reports`. Final state:
argued. Arguments: `argument@record-search-workflow@arg-10`, `argument@record-search-workflow@arg-26`.
Shape: Decided design, "A passing `--verify` writes the closing sections of every report into one
file". Harvest: `design@query@occasion-shaped-output` widened, and
`design@agent-skills@session-acts-on-the-verdicts` rewritten for where the session reads them.
Closed in the reply to round 3, shape 1 over shape 3 on the fact that in a workflow the searchers'
returns never reach the session.

### Each verdict file holds its act verdicts whole first, then its light verdicts as lines with their quotation and reason `##verdicts-split-by-action`

Proposed by the session in the reply to round 2, as a third shape against case A and case B of
#verdicts-verified-in-the-workflow. Approved by the owner in round 3, in its first shape: act
verdicts with their entries whole, light verdicts as compact lines without their reason. Revised in
the reply to round 3, on a material finding of the experiment: three load-bearing verdicts lay in
the reasons the compact lines dropped; the files stay per subject, and each light line keeps its
quotation and its reason. Final state: argued, revised; the owner approved the design as displayed,
round 4. Arguments: `argument@record-search-workflow@arg-15`, `argument@record-search-workflow@arg-16`,
`argument@record-search-workflow@arg-18`, `argument@record-search-workflow@arg-20`,
`argument@record-search-workflow@arg-21`, `argument@record-search-workflow@arg-22`,
`argument@record-search-workflow@arg-29`. Shape: Decided design, "Each verdict file holds its act
verdicts whole first, then its light verdicts as lines with their quotation and reason". Harvest:
`design@agent-skills@session-acts-on-the-verdicts` rewritten, `design@query@occasion-shaped-output`
widened.

### A cross-check stage inside the workflow refutes the verdicts before the session reads them `##verdicts-verified-in-the-workflow`

Proposed by the session in the reply to round 1. Weakened in the reply to round 2, after the
owner's worry of round 2 that the main session may have more context to judge. Run as the
experiment in the reply to round 3, on the owner's word of round 3. Final state: ruled-out, in the
reply to round 3, on `design@agent-skills@work-divided-by-judgement`: an output verified by a
session other than its author is no property on which an instruction fixes a dispatch. The session
may still hand a subject's verdict file to an agent it dispatches, at its judgement. Arguments:
`argument@record-search-workflow@arg-11`, `argument@record-search-workflow@arg-14`,
`argument@record-search-workflow@arg-17`, `argument@record-search-workflow@arg-18`,
`argument@record-search-workflow@arg-19`, `argument@record-search-workflow@arg-23`,
`argument@record-search-workflow@arg-29`. Shape: Losing alternatives. Harvest: the agent-skills
`rejected` home, where the tests for a losing alternative admit it.

### The session edits nothing while the workflow runs `##search-reads-a-fixed-tree`

Proposed by the session in the reply to round 1. Final state: argued. Arguments:
`argument@record-search-workflow@arg-12`. Shape: Decided design, "The session edits nothing while
the workflow runs". Harvest: the head of the workflow. Closed in the reply to round 3 on its
argument.

### The dispatch moves out of the searcher's description into the script `##dispatch-text-home`

Proposed by the session in the reply to round 1. Final state: argued; presumed-settled in the reply
to round 3 while it waited for Q1, closed by the owner's ruling of Q1, no, round 4. Arguments:
`argument@record-search-workflow@arg-13`, `argument@record-search-workflow@arg-30`. Shape: Decided
design, "The dispatch lives in the script, and the searcher's description keeps what the session
needs". Harvest: `design@agent-skills@one-record-searcher` rewritten and
`design@agent-skills@a-scratch-directory-per-subagent`'s site. Its line for a harness without the
workflow is superseded by #fallback-dispatch-line.

### The by-hand line stays, argued on a measured fact of `claude`, and points to the script `##fallback-dispatch-line`

Proposed by the session in the reply to round 3, from the experiment's refutation of the verdicts on
`goal@serves-one-kind-of-project` and `issue@agent-skills@heads-argued-from-another-harness`.
Final state: argued, with a clause from the premortem. Arguments:
`argument@record-search-workflow@arg-20`, `argument@record-search-workflow@arg-25`,
`argument@record-search-workflow@arg-36`. Shape: Decided design, "The dispatch lives in the script,
and the searcher's description keeps what the session needs". Harvest: the head of the workflow.
The premortem of the reply to round 4 added the clause that the line restates no step.

### The flag builder is a pure function, tested under `node` `##search-workflow-logic-tested`

Proposed by the session in the draft of the reply to round 3, against the criterion the reply to
round 1 left open. Final state: argued. Arguments: `argument@record-search-workflow@arg-31`,
`argument@record-search-workflow@arg-35`. Shape: Decided design, "The flag builder is a pure
function, tested under `node` beside the parse check". Harvest: the doc of the parse check's module
and the commit of step 3. The premortem's cause 2 made its test a claim of step 3.

### No split of `worklist.rs` in this work `##worklist-module-split`

Proposed by the session in the reply to round 3, from the code-split scout's report. Final state:
argued. Arguments: `argument@record-search-workflow@arg-32`. Shape: Decided design, "No split of
`worklist.rs` or of `binary.rs` in this work". Harvest: the commit of step 1.

### No split of `binary.rs` in this work `##binary-tests-split`

Proposed by the session in the reply to round 3, from the code-split scout's report. Final state:
argued. Arguments: `argument@record-search-workflow@arg-33`. Shape: Decided design, "No split of
`worklist.rs` or of `binary.rs` in this work". Harvest: the commit of step 1.

### The logic test lives in the parse check's module, its doc widened `##workflow-logic-test-home`

Proposed by the session in the reply to round 3, from the code-split scout's report. Final state:
argued. Arguments: `argument@record-search-workflow@arg-34`. Shape: Decided design, "The flag
builder is a pure function, tested under `node` beside the parse check". Harvest: the module's doc,
in step 3.

## Arguments

### The workflow could run the command that writes the work list, and the session should do as little as possible itself `##arg-1`

Round 1, the owner. Bears on #search-runs-a-saved-workflow and #workflow-runs-the-worklist. The
owner proposed that the search be a dynamic workflow, asked whether the workflow could call the
command that generates the work list itself, and stated the aim of minimising what the session does.

### A script cannot run a command, so an agent of it runs `--out` `##arg-2`

Round 1, the session. Bears on #workflow-runs-the-worklist. "Per the authoring reference, a
workflow script has "No filesystem or Node.js API access": it only spawns agents. So the first step
is a small agent that runs `klarch worklist --out` and returns the list of `agent-<n>` directories
through a schema. The three audit workflows already do this".

### The session's part shrinks to three acts `##arg-3`

Round 1, the session. Bears on #search-runs-a-saved-workflow. "Choose the inputs: the work files,
the subjects, the seeds. These are judgements only the session can make." "Start the workflow."
"Act on the verdicts it returns. These acts route decisions, so per
`design@agent-skills@session-acts-on-the-verdicts` they stay with the session."

### The occasions differ only in their flags `##arg-4`

Round 1, the session. Bears on #search-runs-a-saved-workflow. "The four occasions the searcher's
description names (a design grounding, a retrospective intake, the design audit of a slice or of a
spec) differ only in their flags. `--seeds-from` switches the outputs to `gaps.md` and
`fit-lines.md`, so one script covers all of them."

### A rewording still leaves the hand dispatches `##arg-5`

Round 1, the session. Bears on #search-runs-a-saved-workflow. "you took in the generic part of the
36-agent workflow, and a rewording still leaves 24 to 36 hand dispatches and checks per search."

### The session running `--out` saves nothing `##arg-6`

Round 1, the session. Bears on #workflow-runs-the-worklist. "That costs the session one command and
one decision about flags, and the session must already know those flags to fill `args`. So the
rival saves nothing and adds a step."

### A refusal of `--out` is the workflow's result `##arg-7`

Round 1, the session. Bears on #workflow-runs-the-worklist. "a refusal by `--out`, such as a seed
that names no entry or a work file it cannot read, comes back as the workflow's result, and the
session repairs its args and starts the workflow again."

### No step repairs a report `##arg-8`

Round 1, the session. Bears on #report-loop-in-the-workflow. "It never repairs a report itself,
because the dispatcher may not repair one, per `design@agent-skills@one-record-searcher`."

### Whether a searcher in a workflow meets the refusal of `Write` is a probe of the harness `##arg-9`

Round 1, the session. Bears on #report-loop-in-the-workflow. "that a searcher run inside a workflow
meets the same refusal of `Write` as one dispatched by hand. That probe is the harness's mechanics,
which the root `CLAUDE.md` admits." Answered in the run of round 3, `argument@record-search-workflow@arg-27`.

### A command writes the closing sections better than an agent copying them `##arg-10`

Round 1, the session. Bears on #closing-sections-file. "A command does this better than an agent
copying text (`goal@tasks-a-tool-does-better-are-the-tool-s`)." The rival "needs no Rust change,
but it is the kind of copy that missed sixteen reports before."

### A cross-check stage would take the verification out of the session `##arg-11`

Round 1, the session. Bears on #verdicts-verified-in-the-workflow.
`design@agent-skills@session-acts-on-the-verdicts` "requires an independent check of each verdict
other than `unrelated` before the session acts on it, and allows the session to hand that check to
an agent." The cost: "about one more agent per subject, and the verifier must read the work files
too." The rival, the check kept with the session: "cheaper in agents, but it is exactly the reading
you want out of the session."

### A workflow runs in the background, and a reading worktree fails on uncommitted work `##arg-12`

Round 1, the session. Bears on #search-reads-a-fixed-tree. "The searcher's body says "You run in
the session's tree, which nobody edits while you search". A workflow runs in the background, so the
session could edit meanwhile." The rival "fails on uncommitted work: the draft of the work, and
often the entries it changes, are not committed when the search runs."

### The dispatch moves to the script, as the audit axes' did `##arg-13`

Round 1, the session. Bears on #dispatch-text-home. "The dispatch moves out of the searcher's
description into the script, the same move the audit axes made." It "rewrites part of
`design@agent-skills@one-record-searcher` ("its description carries the dispatch")", and "Each
skill names the workflow, which is what counts as the opt-in the Workflow tool requires."

### The main session may have more context to judge the verdicts `##arg-14`

Round 2, the owner. Bears on #verdicts-verified-in-the-workflow. The owner was worried by the
cross-check in the workflow, since the main session might have more context to judge the verdicts,
asked what would change in each case, and proposed an experiment in this discussion's own search.

### A search returns two populations of verdicts `##arg-15`

Round 2, the session. Bears on #verdicts-split-by-action. Counted over the verdict lines of the
commit messages of four past searches: 177, 116, 125 and 19 verdicts `constrains` or `touches` with
`respect` or `none`, and 11, 9, 6 and 1 that need an act. "So a search returns two very different
populations".

### To act on a verdict the session reads its entry whole anyway `##arg-16`

Round 2, the session. Bears on #verdicts-split-by-action. "To act on a `check-reversal`, the session
must read the entry whole and reason over it anyway. Checking the verdict costs almost nothing extra
during that reading. This is also where the session's context matters most: whether the design
really reverses a head depends on the shapes as argued in the conversation."

### A cross-checker sees what the searcher saw `##arg-17`

Round 2, the session. Bears on #verdicts-verified-in-the-workflow. "the verifier sees exactly what
the searcher saw, the draft and the record. It is a separate session, but it has the same
information, so it cannot catch a verdict that is wrong only against the conversation." "a
downgrade that escapes reaches the design-conformance reviewer at the earliest, at the plan's review
or at the merge (`design@agent-skills@verdicts-reach-the-conformance-review`). By then the work may
exist and the fix is rework."

### A cross-check leaves the session reading the light lines all the same `##arg-18`

Round 2, the session. Bears on #verdicts-verified-in-the-workflow and #verdicts-split-by-action. "it
must still know every `respect` constraint to keep the shapes within it. So it still reads the light
lines and only drops its own checking of them."

### What the experiment can show `##arg-19`

Round 2, the session. Bears on #verdicts-verified-in-the-workflow. Under
`design@agent-skills@synthetic-evidence-not-built`, the run is real use; "comparing the
cross-checker's results with mine" is not evidence for choosing, since "One run is a sample of one,
and agreement between two agents chooses no shape"; "a concrete miss confirmed by reading" counts;
"a miss would be evidence. No miss proves nothing."

### Three load-bearing verdicts lay in the reasons the compact lines dropped `##arg-20`

Round 3, the session, after the run: a material finding against the shape the owner approved in
round 3. Bears on #verdicts-split-by-action, #worklist-notices-returned, #fallback-dispatch-line and
#verdicts-verified-in-the-workflow. "Three load-bearing items got past me that way. The
cross-checkers found all three, and each one is in the searcher's own `reason` field, which the
compact lines leave out": `design@agent-skills@design-audit-checks-fit`, whose notices `--out`
prints to stdout only; `design@agent-skills@work-divided-by-judgement`; and
`goal@serves-one-kind-of-project`. "The defect is in the format I proposed, not in the session
doing the reading."

### The light lines with their reasons are a fraction of the verdict files `##arg-21`

Round 3, the session. Bears on #verdicts-split-by-action. "With reasons, the light lines are 95 KB
for 178 entries. The verdict files are 1.5 MB, since one entry appears under each of its subjects."

### A subject handed to an agent is that subject's file `##arg-22`

Round 3, the session. Bears on #verdicts-split-by-action. The files stay per subject, on the
argument of `design@query@occasion-shaped-output`, "a session that hands one subject of the work to
an agent hands it that subject's file".

### A verification by another session is no ground for a fixed dispatch `##arg-23`

Round 3, the session. Bears on #verdicts-verified-in-the-workflow.
`design@agent-skills@work-divided-by-judgement` says "An output verified by a session other than
its author is no such property". "So no fixed cross-check stage can stand on verification." The
workflow's other fixed steps stand on "an output whose form a command checks".

### `--out` prints its notices on stdout only `##arg-24`

Round 3, the session. Bears on #worklist-notices-returned. "A design audit reads the notices that
`worklist --out` prints. Those notices go to stdout only (`outln!`, in
`path@query@src/worklist.rs`, lines 435 to 471). The draft's worklist step returned only the directories,
so the notices would be lost."

### A search run inside an agent cannot run the workflow `##arg-25`

Round 3, the session. Bears on #fallback-dispatch-line. "a general-purpose subagent holds no
`Workflow` tool (measured in `design@agent-skills@work-divided-by-judgement`), so a search run
inside an agent dispatches by hand". The line is argued on that fact, "not on another harness. This
answers `goal@serves-one-kind-of-project`."

### In a workflow the searchers' returns never reach the session `##arg-26`

Round 3, the session. Bears on #closing-sections-file. "In a workflow, the searchers' returns never
reach the session. Reading the closing sections by hand (shape 3 of that issue) would therefore mean
22 file reads per search."

### The run's mechanics `##arg-27`

Round 3, the session. Bears on #search-runs-a-saved-workflow and #report-loop-in-the-workflow. "22
searchers and 36 agents in all, in 8.5 minutes, 4.49M subagent tokens." "`--verify` passed on its
first round. No report came back as text: the 22 searchers in the workflow all wrote their own
files."

### The searchers' peaks stay under the bound, and the head's re-take path is stale for a workflow `##arg-28`

Round 3, the session. Bears on no thread; its outcome is the rewording of
`design@query@worklist-bound-is-40000-words` in the harvest. "The searchers' peak context was 64,251
to 170,216 tokens, below the 250,000" of that head, taken with its `jq` command; "a workflow's
agents sit under `subagents/workflows/wf_<id>/`, not `subagents/`."

### The experiment's result `##arg-29`

Round 3, the session. Bears on #verdicts-verified-in-the-workflow and #verdicts-split-by-action.
"The cross-checkers refuted 17 of 430 verdicts": 3 load-bearing, 1 the owner question Q1, "About
10 narrowed an `against` list or an action without changing the work", and "I found nothing the
cross-checkers missed through the conversation's context." "This is one run, so it cannot rank
sessions against cross-checkers. What it does establish, confirmed by reading: a cross-check would
have saved my misses, but so would the reasons the searchers had already written."

### This work moves the dispatch, and the reopening would change what a search reads `##arg-30`

Round 3, the session, the default of Q1. Bears on #dispatch-text-home. "In a related case, your Q6
kept this reopening out of the milestone on split guidelines. The two works also change apart: this
one moves the dispatch, the reopening would change what a search reads." And: "with a workflow, a
second search after a new thread is a single call with the extended draft." The owner ruled Q1 no,
round 4.

### A dropped seed is a silent failure `##arg-31`

Round 3, the session, in the draft and its delta. Bears on #search-workflow-logic-tested. The flag
builder is "the script's one piece of logic whose failure is silent", since "a dropped seed weakens
the search and nothing reports it", and it is a pure function, tested under `node`.

### The writer and the judge of `worklist.rs` share their formats, and the work writes into the judge alone `##arg-32`

Round 3, the code-split scout's report, taken by the session. Bears on #worklist-module-split. The
file holds a writer and a judge that some commits change apart; but "The writer and the judge read
the same file formats together", and "This work touches only the judge".

### The tests of `binary.rs` share one build of the binary `##arg-33`

Round 3, the code-split scout's report, taken by the session. Bears on #binary-tests-split. "the
tests share `klarch()`, the one build of the binary, and the sandbox helpers."

### One owner per external effect `##arg-34`

Round 3, the code-split scout's report, taken by the session. Bears on #workflow-logic-test-home.
"this module owns running `node` and the CI requirement on it (`checked`, `required`, `CI_VAR`).
`path@xtask@src/gates.rs`, lines 100 to 101, reads this file by `include_str!` to check the spelling of
`CI_VAR`. A second module running `node` would either share these items or carry a copy of the CI
requirement that the xtask test does not read."

### One verify agent's prompt would hold every text report `##arg-35`

Round 4, the session, the premortem's cause 1, with cause 2 on the flag builder. Bears on
#report-loop-in-the-workflow and #search-workflow-logic-tested. "Many searchers return their
reports as text. The draft passes all of them through one verify agent's prompt, about 20 KB each,
so that prompt grows past what one agent handles well"; the outcome: "each text report is written
by a low-effort agent of its own, before `--verify`".

### The by-hand line could drift from the script `##arg-36`

Round 4, the session, the premortem's cause 6. Bears on #fallback-dispatch-line. "The by-hand line
drifts from the script"; the outcome: "the line points to the script and restates no step".
