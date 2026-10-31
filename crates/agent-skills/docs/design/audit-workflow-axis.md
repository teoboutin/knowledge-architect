# Agent skills design: The agentic-workflow axis

## The agentic-workflow axis

### The agentic-workflow axis reads every text the harness delivers to a session as an instruction, and neither the scoped CLAUDE.md files nor the registers are in its corpus `##workflow-axis-corpus`

The corpus is the project's root `CLAUDE.md` and the primer it imports, and every other text the
harness delivers to a session as an instruction from the project's agent configuration directory,
installed or the project's own. `agent@knowledge-architect-workflow-auditor@the-corpus` defines it,
and the axis's section and the saved workflow point there. The definition names no provider's
directory, so it holds for a second harness, per
`issue@core@configuration-for-several-agent-providers`. The registers are records, not
instructions: an auditor reads the issues listing so that a finding already recorded is marked with
its reference rather than reported again. An auditor may run read-only commands to check what the
corpus claims about the checker or the harness.

The corpus is the owner's: the root `CLAUDE.md` and the content of the agent configuration
directory. So is the exclusion of the scoped `CLAUDE.md` files next to the code, which the owner judged more like developer documentation than instructions to follow. `issue@core@a-home-for-developer-contracts-outside-agent-configuration`
points the same way. The cost is that no audit reads a scoped `CLAUDE.md`: the need test of
`design@agent-skills@statement-names-an-act` reaches one through its writer and the review axis on agent-facing
text only. Every auditor
reads the whole corpus, per `design@agent-skills@workflow-axis-lenses`, which rests on one agent
being able to read it whole: `git ls-files CLAUDE.md .claude | grep -v open-issues | xargs wc -w`
measures it, and an auditor's return that says it did not read every file whole reopens how the
corpus is shared out.

### Every auditor of the agentic-workflow axis reads the whole corpus through one lens, each lens dispatched at least once, with no calibration sample `##workflow-axis-lenses`

A lens is one task: what an auditor looks for. The lenses built are L1 contradiction, L2 activity
walk, L3 two readings, L4 provable gap, L5 restatement, L6 local against installed, L7 checker rules,
L8 verdict alignment and L9 need test, and `agent@knowledge-architect-workflow-auditor@the-lenses` is the one
home of what each looks for. Each lens is dispatched at least once, and the count per lens is the
session's, passed to the run as `counts`. The saved workflow's lens table holds each lens's default
count and L2's groups of activities, so the session writes no entry per agent. An L2 walker takes
one group of activities, and two walkers are duplicates only when they share one. A finding two
agents report is merged. There is no calibration sample: the agents give no verdict per entry, the
duplicates of a lens add coverage, a verifier and the session read every finding, and the re-check
reads again.

The shape is the owner's: the owner judged that the workflow functions as a whole, that none of
its files can be judged alone, and that a main aim is finding contradictions across files, and
preferred agents with varied tasks, each given the whole corpus, duplicates included where there is
room, since a duplicate may find more. The nearest rival, the corpus shared out among
agents of one task as the design-record axis shares its own, loses on that argument: a
contradiction across two files sent to different agents is found by neither. Cutting a lens's count
or narrowing its scope lost on the fifth pass of the axis on this repository: the verifiers and the
session confirmed 37 of its 38 findings, each lens with more than one agent produced findings no
other agent of that lens reported, and 14 of the 15 findings of two readings, L3's class, were
confirmed, the session rejecting one, while 9 of the 15 sat on text written for the axis itself. The plan document of that work holds the
measurement, and `git log --diff-filter=D --name-only -- docs/plans/` finds it. The count per lens
rather than a total budget is the owner's preference, stated with a hedge, so that the script
rather than the session's judgement dispatches the agents. The table lives in the script, since the agents' entries written by
hand were a run's largest input and the source of its slips, which the owner judged error
prone. A fixed count would be a structure another instruction needs to vary, against
`goal@agent-skills@only-needed-instructions`.

Two lenses read beside the corpus. L7 starts from the checker's user documentation, its README for
the version the project uses and the help of each command, and reports a rule the corpus states
wrongly, and a rule on what an agent writes that no installed text states, when its finding message
does not name its repair. It judges the installed text alone, since a project text restating the
checker's rules would hide a gap the installed text leaves every adopting project. The owner raised
it, judging that agents cannot work efficiently without knowing what the checker refuses and allows
them to write.
Its rival, the README given to L4, loses: L4 starts from a requirement the corpus states, and a rule
no text mentions is invisible from there. A rule whose message names its repair costs one check and
one repair when it is broken, while a rule that shapes writing from the start, such as the
reference grammar, must be known before writing. L8 reads each pair of a review standard and an
audit standard that judge one subject, side by side, for a text of the project that one reports as
a finding and the other must not, the class of divergent verdicts. The owner proposed it, holding it quite important that the reviews and the audit axis agree on what they count as
findings. Two standards that no single session obeys together are no contradiction, so no other
lens drafts such a pair, and none of the drafts of the axis's first four passes compared what two
standards count as a finding. The rival,
the class with no lens of its own, loses on that.

L5 reads each restatement of a directive against its home, since the checker cannot compare
prose. The restatements are in this axis on the owner's word, given when they were proposed for the
design-record axis: the owner judged that they read another thing than that axis does, and placed
them, as the owner's opinion, in the agentic-workflow axis.

L9 reads each statement of the corpus against the need test of `design@agent-skills@statement-names-an-act`, so
that the audit and the review axis on agent-facing text judge one statement by one standard, per
`design@agent-skills@admission-rules-home`. A lens of its own, rather than a widened L5, is the owner's choice; L5 judges a restatement
against its home, and L9 whether a statement is needed at all.
Its two auditors by default rest on the premise above, that a second agent of one lens finds what
the others missed.

The designs this decision did not keep: `rejected@agent-skills@workflow-corpus-shared-out`,
`rejected@agent-skills@narrower-two-readings-lens` and
`rejected@agent-skills@divergent-verdicts-without-lens`.

### The agentic-workflow axis admits a finding only when a reading confirms it, and a gap that only predicts what an agent would do leads to no edit `##workflow-axis-finding-standard`

A finding is admitted in a class whose evidence a reading checks: a contradiction, two instructions
quoted and the moment both apply; a broken trigger, the instruction and the search that shows its
target absent; a factual error, the claim and the command that shows it false; two readings, the
instruction and the two acts its words allow; a provable gap, a requirement and an input to it that
no text supplies and judgement cannot derive, since it is a fact fixed elsewhere, such as a format
the checker enforces, a name or a path, not a choice an agent could make; divergent verdicts, one text of the project and the sentence of each standard that judges it; a
restatement or a pointer its home does not allow; and an unneeded statement, the statement and the
act tried, per `design@agent-skills@statement-names-an-act`. A predicted gap, "an agent might misjudge X", leads to no edit:
on the project's own text it is an issue that states what a real session would have to show, and
on the installed text it is sent upstream, per `design@agent-skills@audit-outcomes`. A case no instruction covers, where judgement can
decide, is no finding, per `primer@room-to-judge`. A finding about how the owner works is outside
the axis: the retrospective judges it against an expectation set.
`agent@knowledge-architect-workflow-auditor@what-a-finding-is` is the one home of the classes.

The standard derives from `design@agent-skills@behaviour-additions-need-real-use`: a defect provable by
reading is repaired on reading, and an instruction about how an agent behaves is added only on
evidence from real use. A divergent verdict needs a text of the project, since a written example is
a synthetic scenario. The owner set the axis's aim: most of its work is finding what cannot be followed, and the gaps
where judgement is not enough, those gaps following the principle that some things are left to the
agent's judgement, since no text covers every case, and that instructions are best kept to what the
workflow needs to function as intended. The class of an unneeded statement is in the axis because
the owner ruled that all work that judges or edits agent-facing text follows one standard, per
`design@agent-skills@admission-rules-home`.
The nearest rival, every gap a finding to fill, fails
`goal@agent-skills@only-needed-instructions`.

A confirmed finding of two readings is repaired even where both acts it allows are acceptable, as
`design@agent-skills@audit-outcomes` says. The rival that left it to judgement where no text rules
either act out would have left 6 of the 14 findings of two readings the session kept at the fifth
pass, by a reading of each against the corpus, the design homes and the goals at the commit that
pass audited. The spec of that work,
`path@elsewhere@docs/plans/specs/agentic-workflow-axis.md`, deleted at its landing, holds the
measurement, and `git log --diff-filter=D --name-only -- docs/plans/` finds it. The findings of a
reading are worth their repairs: of the first pass, the owner observed that several findings were likely the
source of a misbehaviour the owner had seen.

The design this decision did not keep: `rejected@agent-skills@two-readings-left-to-judgement`.

### One installed agent carries the agentic-workflow axis's standard, its brief choosing its lens, or its role as a verifier or a checker `##workflow-auditor-agent`

`agent@knowledge-architect-workflow-auditor` holds the standard every auditor of the axis follows:
it edits nothing and writes only in its scratch directory, it copies every quotation from the file
and lists only commands it ran, it reads the corpus whole, and it holds the finding classes, the
tags, one draft per finding and its return. Its definition has one section per lens, and the brief
names which is its task; a finding of another lens met on the way is drafted too, under the
finding's lens. A verify brief makes it the verifier of another auditor's drafts, per
`design@agent-skills@verifier-in-the-run`, and a check brief makes it the checker of one repair
proposal, per `design@agent-skills@repair-proposals-audited`. So the standard reaches every agent
that judges a finding of the axis from its definition's first token, per
`design@agent-skills@audit-agent-per-axis`. The steps whose agents judge no finding against the
standard, the dedup, the cluster, the merge, the repairs, the cross-check and the report, run
general-purpose agents whose prompts are in the saved workflow. No agent of the run decides an
outcome: the cross-check proposes one, and the session decides it.

The nearest rival, one agent per lens, would repeat the standard shared by every lens, which is most
of the definition, by `wc -l` over the definition and over its section on the lenses. The owner
weighed the count of published agents: "I was worried about publishing too many agents and
polluting the directory. I think this shape works." The rival of the verify brief, a verifier agent
of its own, would carry the axis's standard outside the axis's agent, the shape
`design@agent-skills@audit-agent-per-axis` rejects in one generic auditor.

The design this decision did not keep: `rejected@agent-skills@workflow-auditor-per-lens`.

### A verifier of the agentic-workflow axis marks a finding that carries a tripwire's firing evidence, after reading the tripwire whole, and leaves the firing to the session `##tripwires-read-by-the-audit`

The run's preflight saves the project's tripwires listing, and each verify brief names it. A
verifier reads whole, with the checker's `show`, each tripwire whose guarded decision or subject the
draft's finding touches, and marks a finding that carries its firing evidence, naming the tripwire.
The cross-check and the report list the marked findings beside those bearing on a ruling, and the
session judges whether each tripwire fired; a fired tripwire moves as
`skill@knowledge-architect-issue-tracking@movement-instruction` says. The corpus excludes the
registers, so without the mark no step of the axis reads a tripwire, and a tripwire whose evidence
the audit holds has no reader. The nearest rival, a search of the standing entries at each pass,
looks for the entries one piece of work bears on; an audit's work is the whole workflow, so the
search would return most of the register of the installed text, while a verifier holds one finding
at a time, the narrow question a tripwire's firing needs.

### A finding of agent-facing text is repaired at the cause that produced it, not at its site `##root-cause-repair`

A cause is one of four kinds: two homes for one moment, instructions about one moment or one kind of
act stated in more than one text; a structure the work did not need, a fixed sequence, count or
mapping that another instruction needs to vary; a restatement that drifted from its home; and a rule
written for one interaction, a narrow rule where judgement already decides. A finding that fits none
is a cause of its own. Each kind has its repair: the homes merged into one and the others pointing
to it, the structure removed, the restatement brought back to a pointer, the narrow rule deleted and
the case left to judgement. A repair adds an instruction only where none of these closes the cause,
and says why. One repair is proposed per cause, never one per finding, and each finding takes the
outcome of its cause's repair. Outside the project that publishes the workflow, a cause whose
findings are on both sides, the installed text and the project's own, is repaired in two parts: its
findings on the installed text are sent upstream, per `design@agent-skills@audit-outcomes`, and
those on the project's own text take the outcome of its repair's edits on that text.
`skill@knowledge-architect-agent-configuration@admission-standard` is the one home of the standard,
per `design@agent-skills@admission-rules-home`, and the audit and each text that names it read it
there.

In the agentic-workflow axis a second phase of agents delivers it inside the run: cluster agents,
two by default, group the confirmed findings by cause independently; a merge agent writes one
causes file from their groupings and states each choice it made where they differ; and repair
agents, one per cause by default, propose its repair, which
`design@agent-skills@repair-proposals-audited` and `design@agent-skills@repairs-read-against-the-record`
check before the session reads it. The counts are the session's, passed to the run. These agents
judge no finding against the axis's standard, so they are general-purpose agents whose prompts are
in the saved workflow, not briefs of the workflow auditor, since an agent's definition is the copy
the harness registered when the session started. The rival, the workflow auditor gaining two more
briefs, one to cluster and one to repair, loses on that.

The design-record axis's run has no cause step: an edit of a head adds no instruction, so two
edits interact only where they touch one paragraph or one slug, which its cross-check maps, per
`design@agent-skills@one-run-pass`. The rival, the cluster and repair steps of the
agentic-workflow axis, would group by cause findings the owner list already groups by kind.

The direction is the owner's. The owner judged that instructions added for corner cases are always more likely to interact
incompatibly by their number alone, as `design@agent-skills@capabilities-not-structure`
records, that repairing contradictions one at a time at each site is not viable, and that a more
thorough method of repair might be needed. One home per moment is the owner's too: the owner asked to try to centralize the instructions about one moment or one kind of act, as the rules on design
heads were. So is the second phase: the owner asked that the axis dispatch a second set of agents to
classify the findings and propose their repairs. `design@agent-skills@capabilities-not-structure`
states the principle the repair applies: a conflict a structure causes is answered by removing
structure rather than by adding a rule. The nearest rival, one repair per finding at its site, lost
on the axis's first audit on this repository: the repairs of its first two passes turned most findings
into added rules, and its third pass found about 25 findings in the interactions of those rules, by
the auditors' returns, which the message of the commit "[audit] Pass 3 of the agentic-workflow axis
stops the run: the count did not fall, the repairs move to a method by cause" records. The rival of
the second phase, the session grouping the findings alone, lost on the same audit: the session's
sort dropped a finding and left a ruling of the owner off the next pass's kept list.

The designs this decision did not keep: `rejected@agent-skills@one-repair-per-finding` and
`rejected@agent-skills@workflow-repair-steps-reused`.

### A set of repairs that adds more instructions than it removes is reported to the owner with its net count, and an audit's pass waits for the owner's word on that growth `##audit-net-growth`

An instruction is a sentence that tells an agent to do, or not to do, an act at a moment; a
rewording that makes an existing instruction's intent clearer adds none, whatever its length. Each
repair proposal counts the instructions it adds and removes. When the repairs to be applied together
add more than they remove, the owner is told the net figure, and the record that applies them states
it, as `skill@knowledge-architect-agent-configuration@admission-standard` says. In a project audit,
the owner rules on that growth before any repair of a pass is applied, as
`skill@knowledge-architect-project-audit@agentic-workflow-axis` says, and only there: an audit's
lenses read the whole corpus, so the findings of a pass have no bound, where the findings of a
piece of work, a retrospective intake's or a review's, are bounded by real use or by the work's
diff. The bound is the owner's argument, and the growth of a sum of instructions is a cost no goal
ranks, since `goal@agent-skills@only-needed-instructions` judges each instruction. The standard
holds the count and the report, and the audit's own text the hold, since a fragment names none of
its hosts, per `design@agent-skills@admission-rules-home`. Each pass of the agentic-workflow axis also gives the corpus's word count before and after in its
commit message, for monitoring.

The metric is the owner's: "I don't think word count is the correct metric here ... It should
probably apply to "more instructions" rather than more words. Though monitoring net word count
change produced by this process is still interesting, and should at least be done and recorded in
commit messages of audit repairs." A bound on words loses on the owner's argument: "Repairing a
sentence to make intent clearer may need adding words locally, and this should not need my ruling.
The net growth is hard to account for before the fact, too." An instruction is countable from a
proposal before the edit. No bound at all loses on the axis's first passes, whose repairs turned
most findings into added rules that the next pass found interacting, per
`design@agent-skills@root-cause-repair`.

The design this decision did not keep: `rejected@agent-skills@unbounded-repair-growth`.

### A step of the run reads each repair proposal's new text for contradictions and two readings before the session reads it `##repair-proposals-audited`

In the agentic-workflow axis's run a check step follows the repair proposals: one workflow auditor
per proposal, under its check brief, in a worktree of its own, each starting when its proposal is
written. It reads the proposal's text after its edits, in the corpus, through L1 and L3, for a
contradiction or two readings that the new text creates with the rest, and writes its check beside
the proposal. The cross-check and the report read the checks, and the report gives each beside its
proposal. The run does not revise a proposal: the session's verification corrects it, or sends it
back to a repair agent of its cause briefed with the script's repair prompt and with the check, so
a repair is checked before it lands.

Each pass's repairs write new text, and the next pass finds defects in it, per
`design@agent-skills@convergence-count-scope`: of the fifth pass's 38 findings, 16 sat on text that the
repairs of the earlier passes and the commits of the method by cause wrote. The nearest rival, those
defects left for the next pass to find, costs a full pass to surface each. The rival of the session
reading every proposal's new text through L1 and L3 moves those readings into the session, against
the burden `design@agent-skills@one-run-pass` keeps from it. The step adds one agent per proposal
to a pass.

The design this decision did not keep: `rejected@agent-skills@repair-defects-left-to-next-pass`.

### Each repair of agent-facing text is read against the goals, the design heads and the rejected alternatives it touches before the owner rules on it `##repairs-read-against-the-record`

The record check reads every goals home, the design heads that the checker's `show` and a search of
the design homes return for the behaviour the edited text describes, the rejected alternatives
of each Component that owns an edited text, and the scoped `CLAUDE.md` files and code comments that
`skill@knowledge-architect-decision-recording@reversal-check` searches. It names each head the repair would reverse, each rejected
alternative it brings back and each goal it strains, with its quotation. A repair that reverses a
head is a decision, recorded under `skill@knowledge-architect-decision-recording`.
`skill@knowledge-architect-agent-configuration@admission-standard` states it, for every actor that
writes or judges agent-facing text, per `design@agent-skills@admission-rules-home`. In the agentic-workflow axis's run the check of
`design@agent-skills@repair-proposals-audited` makes it, reading those homes beside the corpus,
which they stay outside, and every repair prompt names the nearest rival to its repair and the fact
that decides between them.

The corpus excludes the registers, so without the check no step of the run reads a design head, a
goal or a rejected alternative, and a repair can bring back an alternative that already lost. This
repository's intake of received findings read the same record for its fixes, and the owner asked
whether the audit could learn from what the intake did more thoroughly. The nearest rival, the check
left to the session at the application, when the edit is written, comes after the owner's ruling, so the owner would rule on a repair without knowing that it reverses
a head, against `goal@knowledge-architect@the-owner-decides`. The search by behaviour is a
judgement, and a head it misses is found at the review of the audit's branch, which reads each
repair against the design heads.

The design this decision did not keep: `rejected@agent-skills@repair-check-left-to-session`.
