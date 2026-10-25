# Rejected designs — agent-skills

The designs the Component agent-skills did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@agent-skills@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### Recording every losing thread of a design discussion `##every-losing-thread-recorded`

`live`. The rejected alternatives would grow by every thread of every discussion, including
proposals nobody would raise again, and a reader would search through them to find the few whose
refutation cannot be derived again in one round. It is kept here because a doubt remains: the owner
judges the filter "still a little imperfect", and
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks for the alternatives that lost
and why, which the filter records only in part.

### A spec plus a separate detailed implementation plan, each step written out with its code `##spec-plus-detailed-plan`

`live`. Both documents carry the same decisions and the second drifts from the first. Its refutation
also rests on the owner's account of real sessions, quoted in
`design@agent-skills@no-untested-snippet-is-authority`: implementers force such a plan's untested
snippets into the code at any cost and copy their comments verbatim. That observation cannot be
derived again in one discussion round, and workflows that write such plans are in common use, so the
alternative will be proposed again.

### Reporting a finding met outside the task without recording it ("finish the task, then say what you found") `##finding-reported-not-recorded`

`live`. The owner observed, over real sessions, that an agent left a small defect neither fixed nor
recorded, as a one-line mention inside a long report, where it is easy to miss. That observation
cannot be derived again in one discussion round, and the rule it defeats is the common one.

### Every design thread closing on its argument, the owner's word only overruling `##every-thread-closed-by-argument`

`live`. The owner chose to keep the discussion an exchange in which the agent does not make every
choice, judging that the owner's rough designs most of the time, though not always, end up the
better direction, and that the exchange drives a design's quality; the fork rule keeps that exchange
where two shapes survive. A doubt remains that keeping forks with the owner's word reaches
`goal@knowledge-architect@agents-work-without-drift`, and this is the candidate if forks put to the
owner prove to be what keeps the owner answering what the record could settle.

### Quoting the owner's words verbatim in a head whose ground is the owner's `##owner-words-quoted-in-head`

`live`. Two sessions bent such a quotation to get it past the checker, one by turning spans of
reference kinds still to come into placeholders, in the agent-configuration-entities milestone, one
by writing a reference form bare beside no issue entry, in the session that wrote
`spec@plans@path-quickfixes`; an intake of three retrospective files found both. A doubt remains that an
attribution keeps the scope a quotation showed, which
`tripwire@agent-skills@attribution-loses-the-scope` watches, and this is the candidate if it fires.

### Putting an owner's answer on a reading back to the owner, with the source's reading as the default, when the source contradicts it `##owner-reading-answer-put-back`

`live`. thaum, a project that implements a published rules text, built this first, as a local
addition to the premise procedure. Two of its reviewers found that it still contradicted the rules'
authority: it put to the owner a question the text answers, and the question offered, as one option,
a behaviour the text forbids. That review evidence cost a review pass to obtain.

### Writing the design skill so that weaker models can follow it `##design-skill-for-weaker-models`

`live`. Rejected as a design goal: the workflow uses weaker models as implementers conducted by a
frontier model, not as the owner's counterpart in a discussion. The scripted comparison that entry
cites, which cannot be derived again in one discussion round, showed a smaller model reproducing the
format without the discipline. Simplifying for a cheaper model is a change that will be proposed
again.

### A delegated state, under which the owner hands a decision to the agent within a stated boundary, or a note recording that judgement was handed over `##delegated-decision-state`

`live`. A word that hands judgement over is an approval like any other, as is approving every
default on a quick read. A session decides what the record and its sources settle by the rule of
`primer@question-authority`, which every session applies, per
`design@agent-skills@questions-go-to-their-authority`, never by a grant, and `goal@knowledge-architect@the-owner-decides` keeps an action that cannot be undone with the owner. The state was decided and reversed in one revision of
designing-together, after five of that revision's eight blocking review findings fell on it: inside
a grant, a correction whose own action could not be undone ran with no word from the owner.

### Writing the discussion's ledger to a file during the discussion, so it survives compaction or a new session `##ledger-written-during-discussion`

`live`. An artifact an agent must update every round is one it forgets to update, and a stale ledger
stated with confidence is worse than none. Written with the least effort, as a draft, it answers the
first reason only. A survey of agent systems that keep state files found two properties a thread
ledger lacks: the state is corroborated against something outside the model, such as git, and every
system that lets a model overwrite its state caps it hard. A thread ledger's only corroborant is the
conversation, which is what compaction removes. In the runs of the designing-together skill, thread
states were correct without a file.

### No roadmap, with known undesigned work listed only as issues `##no-roadmap`

`live`. It held that a roadmap file would be a second schedule beside the issue register, and two
schedules drift; it accepted as its cost that the order of future work had no home. Two facts
defeated it. thaum, the first project to use the workflow, kept a file of its next milestones as an
exception to the installed skill: a need for order observed in real use, which cannot be derived
again in one discussion round. And a roadmap whose rows are only checked references is no second
schedule: the work stays in the issue register, and a row dangles, reported by the check, when its
work closes.

### A separate installed file holding the expectation sets, read only by those who judge findings `##separate-expectation-sets-file`

`live`. It would keep the sets out of every skill's text and out of the retrospective, at the cost
of a change to the install layout. The texts that read the sets, the retrospective, the intake and
the admission standard of the agent-configuration skill, would all reach them by a pointer to the
file, where today the two others point to the retrospective's section: the file adds one pointer and
removes none. A pointer to the section loads the section alone, so no reader of the sets loads the
rest of the retrospective through it, the admission standard included. It wins if a reader is shown
to need the sets where the retrospective's section cannot be reached, as a text read outside the
installed set would.

### A project's kept retrospective finding turned straight into an issue entry, with no analysis `##retrospective-finding-straight-to-issue`

`live`. The first use of this repository's intake, on five received files holding 40 findings, found
4 already repaired, 3 already recorded, 2 resting on a false premise, since a checked form already
existed, 2 whose report credited a repair the history places earlier, and 3 whose proposed fix would
have reversed a recorded head: 14 that the direct path would have recorded as open work or applied
against the record. Those files held mostly findings on the workflow; the rate on a project's own
file is not measured. The figures are those of the issue the intake's shipping closed, read with
`git log --diff-filter=D -- crates/agent-skills/docs/open-issues/a-received-retrospective-file-has-no-installed-analysis.md`.

### A transcript reviewer that reports every clause added inside a ruling, as the agent's addition for the owner to contest `##transcript-reviewer-reports-every-clause`

`live`. It protected the owner's rulings down to their wording, at the cost of a list of additions
in every review. The owner, who had meant the reviewer to check durability from the start, judged
from its reports in real sessions that it was "focusing more on the wrong side, the small deviations
from what I approved", and ruled that a detail or a better wording inside a ruling needs no report.

### A milestone document holding the design of every slice, with slice specs holding only their entry `##milestone-holds-every-slice-design`

`live`. It kept the whole design in one place to read. It lost to a measurement: the first milestone
written that way, structured-plans, had a README of 1,624 lines when it left, read whole at the
grounding of every slice. That cannot be derived again in one discussion round.

### Every decision that earns durable text goes through the design skill `##every-decision-through-design-skill`

`live`. It sends a member a head's argument covers, and a relocation that adds or removes no
decision, to a full discussion whose outcome the head already records; the owner ruled such a change
does not need one, and every new member through the design skill cost a reversal for an ordinary
extension, per `design@agent-skills@title-states-the-rule`. It is kept here because a doubt remains:
`tripwire@agent-skills@head-created-without-deliberation` watches whether a session adds to a head a
member its argument does not cover, with no deliberation, and this alternative is a candidate if it
does.

### One agent reading every issue and tripwire of the project for a piece of work `##one-agent-reads-every-standing-entry`

`live`. One agent's load grows linearly with the project: the standing entries of thaum at its
commit ef21314 hold 318,947 bytes. One agent reads every entry whole only while its context holds
them, and cannot judge so many entries reliably. It is kept here because a doubt remains: no
measurement shows that agents given groups of 60 entries judge better, and one agent sees every
entry beside every other, where a group sees only its own.

### One search agent per anchor `##one-search-agent-per-anchor`

`live`. An anchor bounds nothing: in thaum at its commit ef21314, one Component holds 51.9% of the
bytes of the standing entries, measured with `git ls-files` over the issue entry files and the
tripwires homes and `wc -c`, summed by the anchor that holds each file.

### A separate installed skill for bounded problems `##separate-bounded-problem-skill`

`live`. Whether work is bounded is known only after the design skill's grounding, so a separate
skill would repeat that grounding to classify at all. The owner had planned one, then proposed the
path inside the design skill instead, on the argument above: "IMO, there is no way to determine
whether a task is "bounded work that does not change the project's design" without going through the
grounding steps of the design skill." It is kept here because a doubt remains:
`tripwire@agent-skills@a-shortcut-decision-earns-a-head` watches whether the path lets a decision
that earns a head skip its discussion, and this alternative is a candidate if it fires.

### A second entry test that admits a decision because it constrains work not yet built `##entry-test-for-unbuilt-work`

`live`. Read against the 178 heads the design homes held at commit ca83444, which replaced it, by
four subagents, it admitted nearly every head under a wide reading, "governs future work", and
almost none under a narrow one, two of them reporting on their own that it does not discriminate.
Designed work that is not built has its home in a plan document, per
`design@agent-skills@design-home-is-built-intent`, so the test's narrow reading had nothing left to
admit but a policy, which the test that replaced it admits as a decision that no code and no text of
the project states.

### A manifest declaration in the published checker that reads only the `%%` lines of a path `##published-checker-reads-percent-lines`

`live`. It would have checked the comments while content/ stayed out of the walk, at the cost of a
key in the manifest format that every consumer's checker reads, built for this repository's use of
the workflow for its own text. The owner said of the installed files: "I don't want to cater too
much to this use case in the installed files"; applying it to the checker is the session's argument.
Taking content/ into the walk checks the comments with no new format. The reason is about a
declaration serving this repository alone: a declaration of paths that must cite no entry, which
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` considers, would serve any project
that ships text, and is not covered by it.

### A checked form for a commit, resolved by its subject `##checked-commit-form-by-subject`

`live`. A commit of the branch is named by its subject, and nothing checks that a commit with that
subject exists. A checked form would read the history on every run, a cost that grows with the age
of the project, to check names that git itself does not make permanent. A check bounded to the range
`commits` judges fails too: a later branch's documents and messages cite, by subject, commits that
are no longer in its range, so the check must either load every commit or issue no finding on them,
and it then only confirms citations that are already correct. Its value is low, and a subject
written in prose escapes no check: it names history, not the tree.

### The rules on design heads in one section of the decision-recording skill `##head-rules-in-decision-recording`

`live`. A session reads a head at every grounding and every plan, and nothing loads the skill at
that moment, while every subagent holds the primer in its initial context, which a probe of two
subagent types measured in one harness. It is kept here because the probe covers one harness, and
another agent provider may not load the primer.

### A directive restated wherever it has to be delivered `##directive-restated-everywhere`

`live`. An inventory of the installed text at commit bc22d4d found about ten texts restating parts
of the decision-recording skill, with nine inconsistencies between them and it; it is re-taken by a
fresh read-only subagent listing each statement of a rule on heads outside its home at that commit.
It is kept here because a doubt remains: `tripwire@agent-skills@pointer-not-followed` watches
whether a session misses a directive whose restatement became a pointer, and this rule is the
candidate if it does.

### Splitting a head where reversing one part leaves the other standing `##head-split-by-independent-reversal`

`live`. Four audits of the design heads, run apart, found that the test splits a rule from its own
exception, which always passes it, and that parts losing to different nearest rivals discriminate
better.

### A head citing every ruling of the owner as its ground `##head-cites-every-owner-ruling`

`live`. An audit of the heads added or rewritten in the week entry test 4 was added, up to commit
bc22d4d, found about two in five of their citations of the owner to be an approval of the agent's
position, often a batch word, which made an approved default read as the owner's intent. It is kept
here because a doubt remains: `tripwire@agent-skills@owner-intent-stripped` watches whether the
correction removes the owner's own words, and this alternative is the candidate if it does.

### A new member routed by the scope of the owner's approval, found in the history of each head `##member-routed-by-approval-scope`

`live`. It needs the word that approved a head found over the commits and plan documents that wrote
it, where the head's ground is read from the head itself. It is kept here because a doubt remains:
`issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` reopens the routing
of a new member, with this alternative among the candidates.

### A head's title stating the instance built, the members or the mechanism that exist `##title-states-built-instance`

`live`. An audit of this repository's design heads at commit bc22d4d found the instance stated as
the rule in 27 of its 189 heads, and its history holds rewrites of such heads, with their slugs and
tripwires renamed, that a member the argument already admitted forced; the count is re-taken by a
fresh read-only audit of the heads at that commit.

### An audit method with no calibration and no re-check `##audit-without-calibration-or-recheck`

`live`. The re-check is `design@agent-skills@audit-passes-repeat-while-the-count-falls`, and the
calibration `design@agent-skills@design-record-axis-dispatch`. On the first audit of the
design-record axis on this repository, the re-check of each pass found violations the pass had
missed or made, and two agents read one shared sample entry two ways in two of the samples; a method
that trusted one reading per entry would have left both. The audit's commits, found with
`git log --grep='design-record axis'`, carry the counts.

### The repair of a head only when a change touches it `##head-repaired-only-when-touched`

`live`. A repair on touch never reaches a head no change touches, so a project's record stays under
the old rules after its migration. It costs the owner one proposal per head, at the moment a session
already edits it, where a project audit puts every item of its owner list to the owner at once: the
first audit on this repository put 72 items to the owner over three passes. It is kept because a
doubt remains on that cost to the owner's reading time, which
`goal@knowledge-architect@the-owner-decides` spends, and it is the candidate if a project's owner
finds an audit's lists too long to read.

### One generic auditor agent, briefed to read the axis's section of the skill `##one-generic-auditor-agent`

`live`. It puts the axis's standard behind a read the brief must ask for. The head-rules sweep that
preceded the audit skill ran that way, and the discussion that chose the agent recorded its result,
in its argument a16 of the deleted spec design-record-audit: "Today's sweep drafts came out uneven
where the brief was the only standard."

### A routing table in the project's root `CLAUDE.md`, one row per installed skill or agent a project skill or agent adds to `##root-routing-table`

`live`. Every project skill is already listed with its description each session, and a review's
added axes are owed through the dispatching activity's skill, so no row delivered anything those two
did not. Its "adds to" had to be widened for each new kind of link, and the first agentic-workflow
audit of this repository confirmed four findings that turned only on whether a row was owed.

### The agentic-workflow axis's corpus shared out among agents of one task, as the design-record axis shares out its entries `##workflow-corpus-shared-out`

`live`. The workflow functions as a whole, and the axis exists mainly to find two instructions in
different files that no single move satisfies; a contradiction across two files sent to different
agents is found by neither. It is kept here because a doubt remains: the corpus grows with every
skill and every repair, and may outgrow one agent's reading. `design@agent-skills@workflow-axis-corpus`
reopens how the corpus is shared out when an auditor's return says it did not read every file
whole, and this alternative is a candidate then.

### One workflow auditor agent per lens `##workflow-auditor-per-lens`

`live`. Each agent would repeat the axis's shared standard, the corpus, the finding standard, the
tags and the draft's shape, which is about half of any one of them, and the owner did not want
"publishing too many agents and polluting the directory". The concern it answers, one agent holding
every lens drafting mostly under lenses other than its brief's, was counted over the drafts of the
first agentic-workflow audit of this repository: more than a third of one agent's drafts named
another lens at passes 1 to 3 and 6, passes 4 and 5 not counted. After the first, the owner kept one
agent: "multiple agents reporting the same findings is a stronger proof of a problem." That count
measures what the workflow auditor is told to do, since a finding of another lens met on the way is
drafted under that lens, so no tripwire watches it.

### One repair per finding, at its site `##one-repair-per-finding`

`live`. It lost to a measurement on the first agentic-workflow audit of this repository: passes 1
and 2 repaired at their sites, 63 findings applied and 57 ruled by the owner, and the auditors'
returns of pass 3 put about 25 of its findings in the interactions of the rules those repairs added,
by the message of the pass-3 commit, "Pass 3 of the agentic-workflow axis stops the run: the count
did not fall, the repairs move to a method by cause".
`design@agent-skills@capabilities-not-structure` records the same effect from an earlier run.

### No bound on how much a pass's repairs add `##unbounded-repair-growth`

`live`. Passes 1 and 2 of the first agentic-workflow audit of this repository turned most of their
findings into added rules, which the auditors of pass 3 then found interacting, about 25 findings by
their returns, per the pass-3 commit named in the entry on one repair per finding,
`rejected@agent-skills@one-repair-per-finding`.

### Three runs of the audit's saved workflow per pass, the session reading the drafts, the groupings and the proposals between them `##three-runs-per-pass`

`live`. It lost to a measurement over the files of pass 4 of the first agentic-workflow audit of
this repository, `wc -l` over each set: the sort read 63 drafts, 3,738 lines, confirmed 36 findings
and refuted 3, each refutation one a verifier given the audit's commit messages can make; the merge
read two groupings, 1,074 lines, and made two choices of boundary; the confirmation read 23
proposals, 4,030 lines, and sent none back, by the session's own account, which no file records; the
diff read before the commit, 1,290 lines, changed three wrapped lines and one claim. The readings
changed little and took the bulk of the session's context.

### The session reading every draft again after the run `##session-rereads-every-draft`

`live`. In the sort of pass 4 of the first agentic-workflow audit of this repository, the session
refuted 3 of 63 drafts: 2 mechanically, a kept item and a pair of texts one move satisfies, and 1 on
the history of the audit, a pointer an earlier ruling had judged. An adversarial verifier given the
kept list and the audit's commit messages makes all three. It is kept here because a doubt remains:
`tripwire@agent-skills@verifier-refutes-a-true-finding` watches whether a verifier refutes a true
finding the session's sample of refuted drafts does not meet, and this alternative is the candidate
if it does.

### One markdown file per audit axis holding the steps of the manual fallback `##manual-fallback-file-per-axis`

`live`. It is a second home for the steps the saved workflow already holds, and pass 4 of the first
agentic-workflow audit of this repository confirmed such a drift between the skill's prose and the
script: the script's repair prompt had narrowed "the homes merged into one" to "the section of the
skill that owns the moment", which F81 of the pass-4 commit, "Pass 4 of the agentic-workflow axis,
run through the saved workflow: 36 findings confirmed, 23 causes repaired", repaired.

### The audit's workflow script kept in a fenced block of the skill, passed to the Workflow tool inline `##workflow-script-inline-in-skill`

`live`. Probes of Claude Code's Workflow tool, three files under the harness's workflows directory
after a reload, recorded in the message of the commit "The agentic-workflow axis's script ships as a
saved workflow in the installer's namespace", showed a saved workflow run by the name its `meta`
declares, its `args` passed through, and a file one directory deeper not found. A saved workflow is
run by name with nothing to copy, where an inline block lives in the skill as text no session needs
to read.

### The plain count of confirmed findings, its fall judged pass by pass over its context `##plain-confirmed-findings-count`

`live`. It lost to a measurement: each of pass 5's 38 findings of the first agentic-workflow audit
of this repository was classified by the commit that last wrote the words carrying its defect, with
`git blame -w -C` and a check that the words are absent at the earlier commit. 8 sat on text the
main branch held before the audit's branch, 16 on text the repairs of passes 1 to 4 and the commits
of the method by cause wrote, and 14 on text the method's one-run rewrite wrote. The counts of two
passes are taken over two different corpora, and each firing of the plain count, at passes 1 to 3
and at 5, was explained by the same churn, argued by hand.

### Fewer auditors on the two-readings lens, or a narrower scope for it `##narrower-two-readings-lens`

`live`. At pass 5 of the first agentic-workflow audit of this repository, the verifiers and the
session confirmed 37 of 38 findings, each lens with more than one agent produced findings no other
agent of that lens reported, and 14 of the 15 findings of the two-readings class were confirmed, the
session rejecting one, while 9 of the 15 sat on text the audit's own branch had just written.

### Defects in the text a repair writes, left for the next pass to find `##repair-defects-left-to-next-pass`

`live`. By the classification of pass 5 of the first agentic-workflow audit of this repository,
given in the entry on the plain count of confirmed findings,
`rejected@agent-skills@plain-confirmed-findings-count`, 16 of its 38 findings sat on text the
repairs of the earlier passes and the method's commits had written, and each such finding cost a
whole pass to surface.

### A confirmed finding of two readings left to judgement where no text rules either act out `##two-readings-left-to-judgement`

`live`. It lost to `goal@knowledge-architect@agents-get-a-complete-workflow`: "Each instruction says
what it asks plainly enough to follow without guessing what it means". An instruction a reading
confirms is read two ways is covered by an instruction, so the goal leaves it to no judgement. Its
own rivals, such a finding dropped at admission and the session leaving it alone, fall with it. It
is kept here because a doubt remains on the cost: every repair is new text, and new text is where
the next pass finds defects, as `design@agent-skills@capabilities-not-structure` records. Applied to
pass 5 of the first agentic-workflow audit of this repository, the test it rested on, whether
another instruction, a design head or a goal rules one of the two acts out, would have left 6 of the
14 two-readings findings the session kept to judgement. It is the candidate if the owner restates
the goal, under `skill@knowledge-architect-goal-setting`, so that an instruction whose readings all
lead to acceptable acts meets it.

### The check of a repair against the design heads, the goals and the rejected alternatives left to the session at the application `##repair-check-left-to-session`

`live`. The application comes after the owner's ruling on the owner list, so the owner would rule on
a repair without knowing it reverses a head. It is kept here because a doubt remains:
`tripwire@agent-skills@record-check-misses-a-head` watches whether the check, whose search for the
heads that describe a behaviour is a judgement, misses one in the reviews of two separate audits'
branches, and where the check runs is then reopened.

### The commit messages alone as the record of an audit's passes `##commit-messages-as-audit-record`

`live`. A message holds the outcomes, not the evidence: the classification of pass 5's findings in
the first agentic-workflow audit of this repository needed that run's confirmed-findings file and
drafts, which only a scratch directory held. And a record carried by a message alone is lost when
its commit changes no file, which the rebase merge drops, as
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file` records from a probe
repository.

### Divergent verdicts between a review and an audit as a finding class with no lens of its own `##divergent-verdicts-without-lens`

`live`. Over the drafts of passes 1 to 4 of the first agentic-workflow audit of this repository, the
eight whose titles pair a review with an audit are about which axes review an audit's branch and
when the transcript review runs; none compares what the two count as a finding. A deliberate
comparison, made on the owner's request before that audit, had found the design-record auditor and
the decision-record reviewer judging rejected alternatives by two restatements of one rule, which
the commit "The decision-record reviewer and the design-record auditor judge the record by the same
rules" aligned.

### The session settling each calibration reading of the design-record axis, and reading again the drafts it bears on `##session-settles-calibration`

`live`. It puts a reading across every group back into the session, against
`design@agent-skills@one-run-pass`. It is kept here because a doubt remains:
`tripwire@agent-skills@calibration-reading-rejected` watches whether the session keeps rejecting the
readings an agent proposes, and this alternative is the candidate if it does.

### The agentic-workflow axis's cluster, merge and repair steps reused in the design-record axis's run `##workflow-repair-steps-reused`

`live`. An edit of a head adds no instruction, so two edits interact only where they touch one
paragraph or one slug, which the cross-check maps. It is kept here because a doubt remains:
`tripwire@agent-skills@rule-misread-across-groups` watches whether one rule is misread across groups
beyond the calibration sample, and this alternative is a candidate if it is.

### The outcome "a head in the wrong Component" removed from the design-record axis, instead of the auditor judging it `##wrong-component-outcome-removed`

`live`. It leaves a misfiled head unread until a change touches it. It is kept here because a doubt
remains that the auditors' judgements of a head's Component are mostly right. Each such finding goes
to the owner list, per `design@agent-skills@audit-outcomes`, so the owner rules every one and
sees whether they are mostly rejected, and this alternative is the candidate if they are.

### The standard on agent-facing text held in the primer `##admission-standard-in-primer`

`live`. Writing agent-facing text is not every session's work, and the agent-configuration skill's
trigger delivers the standard when it is, per `design@agent-skills@primer-limit`; the session that
wrote the statement its owner judged useless had loaded that skill. It is kept here because a doubt
remains: a session that edits agent-facing text inside another task may not load the skill, which
`tripwire@agent-skills@agent-text-written-without-the-standard` watches.

### The checker's model printed by the binary, the primer pointing to the command `##checker-model-printed-by-binary`

`live`. A command reaches a session only if it thinks of running it, while the harness puts the
primer in every session's context. It would have added a command line, an interface others consume.

### A `rejected` entry naming the decision it lost to `##entry-names-the-decision-it-lost-to`

`live`. Each entry would name its winner, and `show` on a head would list its losers as a file and a
line. The head is what a design session reads, so the link
belongs there; written in both directions it is an inbound list kept by hand, which goes stale at
the next citation; and the owner judged the framing of a winner and a loser wrong.

### Both directions of the link between a head and its `rejected` entries written by hand `##both-directions-written-by-hand`

`live`. Each side would name the other. An entry never lists what references it, since a list kept
by hand is stale at the next reference written elsewhere, per
`design@agent-skills@a-reference-claims-a-revisit`; the entry's side is `show`'s to compute.

### Both directions written, with a check that they agree `##both-directions-checked-for-symmetry`

`live`. It would keep both directions and catch their drift. The check needs to tell a citation of
a winner from any other reference in an entry's body, and a heading register has no field for it,
so it needs a syntax of its own that the head-only direction does not.

### The design-conformance reviewer dispatched on a discussion's checkpoint `##conformance-reader-at-the-checkpoint`

`live`. It would read the proposals as they stand at convergence, with an existing agent. In the
discussion that decided the design-entry search, the owner ruled 7 of 8 threads one by one before
any checkpoint table was shown, so a reader there reads rulings already given. A doubt remains
that a search at grounding misses a thread opened later, which
`tripwire@agent-skills@design-entry-search-misses` watches; this is the candidate if it fires.

### A listing command of design entries, cut into groups of 60 rows `##design-listing-cut-in-groups-of-sixty`

`live`. It would give even groups, as the standing-entry search has. It loses on the owner's
argument that `design@agent-skills@design-entry-search-groups` records, and it needs a command that
grouping by Component does not.

### Groups of whole subjects bounded at 60 design entries, a larger subject cut into consecutive heads `##subject-groups-bounded-at-sixty`

`live`. It applied to design entries the bound of 60 that
`design@agent-skills@standing-entry-search-groups` argues for standing entries, measured on their
size in thaum. The owner ruled otherwise for design entries, as
`design@agent-skills@design-entry-search-groups` records. A
doubt remains whether one agent reads a large subject reliably; this is the candidate if reports of
large subjects come with missed entries.

### A plan document deleted in the commit that completes its last harvest `##plan-document-deleted-with-its-harvest`

`live`. The review before the merge reads the tip, and four of its acts need the document there:
the transcript reviewer's search, the standing-state reviewer's criteria, the spec-conformity and
fidelity-of-relocation axes, and `git log -G` on a pair the harvest row states, which a row edited
in the deleting commit leaves in no tree. A branch that deleted its spec this way had every brief
name the deleting commit's parent, and a history edit then left those briefs naming a commit the
branch no longer held.

### A retrospective intake of many files reading whole every entry the design-entry search grades `may-bear` `##intake-reads-every-may-bear-entry-whole`

`live`. A batch's search returns the union of its files' neighbourhoods, mostly graded `may-bear`
with no bearing established: one intake of nine files returned 169 distinct entries, about 400 KB
of `show` output, and its session read about 35 whole, the ones its proposals rested on; a later
intake of 24 files did the same. Reading every one whole spends the session's context on reading
instead of judging, and neither session could.

### The owner rules on each premortem cause, whether it is recorded as a tripwire `##owner-rules-each-tripwire`

`live`. Whether a risk is worth watching was held a fork no goal or decision ranks, the cost of
watching against the risk, so the owner ruled on each cause put under a label. The tripwire
standard ranks a cause by named conditions, so no fork remains, and the measured cost of the
ruling stood: the owner dropped 23 of 66 ruled causes of this repository, and about 45 of the 64
tripwires it then held watched a failure caught anyway, a subagent's reading of each entry; the
owner also spent rulings on the wording and scope of tripwires.
