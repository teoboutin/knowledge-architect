# Agent skills design: How the installed text changes

## How the installed text changes

The measurements and incidents these entries cite were taken in the designing-together repository,
whose record is docs/decisions.md on its branch next. A figure is taken again from that record with
`git show next:docs/decisions.md`, run in that repository.

### The record of the installed text keeps intent and coherence, not wording `##instruction-record-is-minimal`

A decision about the installed text is recorded only where it keeps the owner's intent on record, as
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks, or keeps two parts of the
workflow consistent, as `goal@agent-skills@no-contradiction-in-workflow` asks; or where another entry test
admits it, such as a reason several installed texts must respect. A rewording, and the rationale for how one instruction
is phrased, is not recorded. Such proposals can be made without bound, unlike a technical decision,
which costs implementation work. The commit message carries a rewording's argument.

The owner's intent is on record in a goal, drafted under `skill@knowledge-architect-goal-setting`,
and the owner's choice at a fork in its weak form, per `design@agent-skills@fork-choice-in-weak-form`;
an approval of the agent's proposal is neither.
Two parts of the workflow are installed texts, or an installed text and the checker it must agree
with, as the planning skill's sections agree with what the core checks; a decision of this
repository alone is not one, since the installed text names none of this repository's conventions.
Installed texts alone would leave a skill free to disagree with what the checker enforces, which an
adopting project meets as a check that fails on a document the skill told it to write. A reason that passes no test goes in a
`%%` comment at the instruction it explains, per `design@agent-skills@shipped-text-line-comments`.

### An instruction about how an agent behaves is added to agent-facing text only on evidence from real use `##behaviour-additions-need-real-use`

A defect provable by reading is repaired on reading, and an instruction about how an agent behaves
is added only on evidence from real use. Real use originates such an addition: a behaviour seen in
a real session, produced unprompted or asked for by the owner mid-session, with the owner naming
what the session would have lacked without it, and a one-sentence statement of the mechanism that
produced the need. An unprompted behaviour shows the gap, not the wording that fills it. A finding
of a review or of an audit originates an edit only where its defect is provable by reading: a
contradiction, a broken trigger, a factual error, an instruction whose words allow two acts, a
provable gap, divergent verdicts that a review standard and an audit standard give over one text
of the project, or a statement that fails the need test of `design@agent-skills@statement-names-an-act`. A
provable gap predicts no behaviour: it is an input that an act needs, that no text
supplies and that judgement cannot derive, since it is a fact fixed elsewhere, such as a format the
checker enforces, a name or a path. So a reading admits its fill, and the fill, which adds an
instruction, goes to the owner, per `design@agent-skills@audit-outcomes`. The rule is scoped to an
addition about how an agent behaves because its argument is that a predicted behaviour is
unreliable evidence, and a fact that no text gives is no prediction. A finding that predicts a
behaviour is parked as an issue, which states what a real session would have to show. Published
literature and synthetic scenarios originate no edit, a written example of divergent verdicts
among them, and no run is built to observe how an agent
follows the workflow's instructions, per `design@agent-skills@synthetic-evidence-not-built`. The rule derives from
`goal@knowledge-architect@the-workflow-improves-through-real-use`, which is stated for the whole
workflow, so it covers every installed skill and every installed agent, a reviewer agent growing
the same way, one predicted check at a time, and a project's own agent-facing text alike: neither
part of its argument, that a predicted behaviour is unreliable evidence and that rules grow one per
interaction, depends on who ships the text, and a project's own text exempt would let its writer
admit what the audit refuses. The rule is stated in
`skill@knowledge-architect-agent-configuration@admission-standard`, per
`design@agent-skills@admission-rules-home`. The evidence behind it comes from one skill, and the skills and agents forked from thaum have not
been edited under it, so a text that needs a different standard argues its exception. Two exceptions are argued in heads of their own,
`design@agent-skills@staged-check-before-each-commit`, and `design@agent-skills@installed-skill-takes-precedence`,
whose ground is a goal on the owner's ruling.

### An installed skill's text carries no size budget `##no-size-budget`

The skill text carries no size budget: an edit is judged on whether it changes behaviour, and two
real sessions held the design skill's full ledger discipline at several hundred lines without
drift.

### No run is built to observe how an agent follows the workflow's instructions, and a run is admitted only when its verdict reproduces `##synthetic-evidence-not-built`

A run built to produce evidence about the workflow is admitted only when its verdict reproduces:
the same run, repeated, gives the same verdict, at a cost low enough to repeat it. The verdict may
read text an agent produced, as long as it reproduces. What an agent decides in following the
workflow's instructions does not meet this test: its verdict varies between runs, and it would mean
something only under a statistical design. So no run is built to observe it, for any of four
purposes: to originate an edit, to choose between shapes, to accept a piece of work, or to check
that a text runs as written. An agent's decision is presumed not to reproduce, since showing that
one does would take the repeated runs this rule refuses.

- **Admitted:** the harness's mechanics, such as a skill loading when invoked by name, a hook
  firing, a tool being granted, or a hook's injected text reaching the context; one dispatch of a
  new agent to see that its tools are granted is one. Also the installer's and the checker's
  behaviour, tested over mock projects.
- **Refused:** whether an agent invokes a skill from its description, whether a reviewer finds a
  given entry, whether an agent follows a step, and the judgement of a new agent's report against
  its instructions.
- **Always admitted as evidence:** a finding from a retrospective, and a misbehaviour observed in
  real use. A finding that comes from a synthetic run anyway is used only for the part a reading
  confirms.

The rule derives from `goal@knowledge-architect@the-workflow-improves-through-real-use`, which
states it for the whole workflow. The owner's argument: synthetic evidence is too unreliable, since
reading it soundly would need at least a statistical analysis, which costs too much, and a proper
mock task needs a design and a test of its own, so it cannot be created on demand. One run of an agent is a sample of one. The
evidence the rule gives up is supplied otherwise: retrospectives of real work reach this repository,
and this repository installs its own skills, so its own sessions use each change before a release
reaches a consumer. The cost accepted is that a defect in a new text is found at its first real use,
not before its merge. The cost is small: the defect a run would catch first is a text's mechanics,
such as a new agent's tools or report format, and this repository's own next review is that text's
first real dispatch, where such a failure shows at once and costs one re-dispatch. The owner judged
that loss small, if it is the only one the session could name, since most of what this repository
does is already well proven to work. Two instances in this repository grounded the rule. A spec's
acceptance criterion, a replay of a search agent over another project's history, was ruled out by
the owner at the step that ran it, on the owner's experience that such testing methods are not very reliable, and drive the
workflow toward wrong directions more often than toward good ones. A slice's criterion, a subagent
classifying decisions written for the check, was dropped at the slice's design audit. The boundary
is a property of the verdict, not a list of subjects: the owner preferred that the head name a wider
category matching the owner's intent, since a narrow wording reduces the intent behind it to
something lesser.
The nearest rival, a list of exempt subjects naming harness mechanics, leaves out the installer and
the checker.

### An installed instruction adds a capability and the judgement to use it, not a rule or a structure for one interaction `##capabilities-not-structure`

No wording makes a methodology self-enforcing: a rule set edited toward leaving no gap grows one
rule per observed interaction, and each new rule creates surface against the rules already there.
In one revision of designing-together, 34 edits produced 10 interaction defects, nearly all between
rules that scripted one exchange. So a gap is weighed by its kind. A capability gap is a move the
agent does not have, such as a way to investigate, to discriminate between positions, or to record
what the next session needs; filling it is worth text. A conformance gap is a move the agent has and
might not make; filling it is worth text only where the default behaviour is systematically wrong,
not occasionally absent, as the design skill's rules against agreeing without testing
are. When both readings fit, the capability form is written: a tool and the judgement to use it,
not a script for one interaction.

A structure an instruction imposes on the work has the same cost as a rule added for one gap: a
fixed sequence, count or mapping between a unit of the work and a unit of its record meets another
instruction that needs that unit to vary, and the session is left with two instructions it cannot
both obey, and less room to work on its own. So the installed text imposes no structure the work
does not need, per `goal@agent-skills@only-needed-instructions`, which gives this
argument a goal's weight, and a contradiction a structure causes is answered by removing structure
rather than by adding a rule. An instance: a plan whose steps are its commits meets the primer's
instruction that a fix met outside the task takes a commit of its own, which is why a spec's steps
are no unit of the history, per `design@agent-skills@step-is-no-commit`.

### An installed skill's scope is bounded by a stated expectation set, which the retrospective carries `##expectation-set-bounds-scope`

An installed skill may state what it expects of the owner: the assumptions about the owner's
behaviour and the project's shape that the skill is built on, each naming what degrades when it
does not hold. The set is the scope test for every proposed edit of the skill: a finding that
describes an owner's behaviour outside the set is not a gap, and no instruction is written for it.
The set scopes the owner's behaviour only. A finding that two installed instructions leave no move
satisfying both is always in scope, because `goal@agent-skills@no-contradiction-in-workflow` is met only
while no two installed instructions contradict.

The sets live in the retrospective skill, which applies them, and two other installed texts read
them by a pointer to its section: the intake, at its check of a finding's kind, and the admission
standard of the agent-configuration skill, which every judge of agent-facing text reads. A file of
the sets' own would add a pointer from the retrospective and remove none, since each other text
would point to it as it points to the retrospective's section. All run in the installing project,
which holds the installed files and not this component's README, so a set they apply has to ship. A
set inside the skill it bounds would be read on every use of that skill, and an agent reading it
would check the owner against a list. The README restates each set for the owner, who is the one it
describes. It works with `design@agent-skills@behaviour-additions-need-real-use`: that entry says what evidence
admits an addition, and this one says which gaps are worth admitting. In the designing-together
repository, a rule for an abandoned discussion was argued, approved, written and reverted in one
session, and four further review findings about the user's behaviour were queued behind it on the
same reasoning. Only the design skill and the setup skill state a set so far:
`issue@agent-skills@expectation-sets-for-the-installed-skills`.

The design this decision did not keep: `rejected@agent-skills@separate-expectation-sets-file`.

### Every statement of agent-facing text names an act an agent does differently because of it, or it is not written `##statement-names-an-act`

Every statement of agent-facing text, installed or a project's own, passes the need test of
`skill@knowledge-architect-agent-configuration@admission-standard`: name the act of an agent
following the workflow that would differ without it. A statement that names none is not written,
and an existing one leaves by that section's outcome table: deleted where it carries no
instruction, moved to a `%%` comment of the installed source where only the source's maintainer
reads it, and put to the owner where it is an instruction no agent performs. A reason passes when
it states a mechanism or a failure prevented; a
second reason that names no other mechanism, a duplicate inside one text, a lead, an emphasis and a
remark about the text fail, since a loaded text is read whole and they add no act. A statement
about state outside the project is suspect, and is written only where an act needs it.

The question the test asks is the owner's proposal, made on a statement an agent had written into a project's root
`CLAUDE.md` about another project's issue, which the owner judged useless to an agent and liable to
go stale with no work in the project. Whose act counts is the owner's ruling: an agent that
maintains the text counts, a statement for the maintainer alone is at most one sentence, and in
the installed text it is a `%%` line, per `design@agent-skills@shipped-text-line-comments`, since
no installing project maintains the installed text. Where the standard left it open, the owner chose
to count the primer and the root `CLAUDE.md` that imports it as one text, since the two are loaded
together, and ruled that the primer's copy is the one that stays. The test's reach, stripping what
is not fully necessary at the price of some variation between its readers, is the owner's
weighing: a statement one reader passes is met by a later pass.

The nearest rival, a narrower test under which a second reason, a duplicate inside one text, a
lead and an emphasis pass, loses on that weighing: it leaves whole classes of statements no act
needs in place. A content rule naming each kind of unneeded statement, such as "no state of another
project" beside "current reality only", grows one clause per case observed, the mechanism
`design@agent-skills@capabilities-not-structure` records, and would refuse a statement an act needs,
such as a tool's behaviour a command relies on. A duplicate fails only while its twin stays, and its
finding names the copy that stays, the one in the section whose subject it is, or the primer's
across the primer and the root `CLAUDE.md`: two passes that each delete the copy they flagged would
otherwise delete both.

### A coined term is defined in the text of the activity that coins it, and the primer restates in one sentence with its pointer each one it uses, and only those, and separates the senses of each word used in more than one `##term-defined-at-first-load`

A term the installed text gives a meaning of its own is defined once, in the text of the activity
that coins it. `primer@coined-terms`, the primer's last section, holds one sentence with a pointer to
that home for each coined term the primer uses and does not define in itself, and one sentence that
separates the senses of each word the workflow uses in more than one, such as "workflow" and
"kind", the primer being the home of that separation. A term used only inside the texts of other
activities takes no entry: a restatement of it elsewhere is governed by
`primer@where-knowledge-goes`, as any restatement is, per `design@agent-skills@restatement-size-test`.
The rule is stated for the maintainer of the installed text alone, in a `%%` line above the
section's heading, since no installing project edits the primer, per
`design@agent-skills@statement-names-an-act`.

A term defined where the act that uses it is delivered reaches every reader of that act, the split
`design@agent-skills@one-home-for-head-rules` makes for the rules on heads: the primer reaches every
session and every subagent, so a term it uses is defined there, and a term one activity uses is met
only by the sessions that load that activity's texts. The section serves
`goal@agent-skills@no-contradiction-in-workflow`: an entry whose term another text defines restates
that home in one sentence and points to it, so it says nothing its home does not. A term the primer
uses and does not define is a defect provable by reading, and so is a word used in more than one
sense, which lets an instruction that uses it allow two acts while no other text sets its senses
side by side; so the section needs no evidence of real use; a rule sending every text to point to a term's home at its first use would be an instruction
to writers with none, and a second home for restatement across texts. The nearest rivals lost to a
count: a glossary in the primer of every term the workflow coins,
`rejected@agent-skills@primer-glossary`, and the section holding every term two or more installed
texts use, `rejected@agent-skills@terms-used-by-two-installed-texts`. At the commit "[plans] The
spec of the primer's terms section and of the record a session reads before a fix", 39 coined terms
not defined in the primer were used by two or more of the 27 installed texts, 19 of them not used
by the primer, each of which either rival would add to every session's load as a second definition; the spec as
the commit "[review] Repairs of the spec's review: the terms rule awaits the owner twice, and the
subsection holds in any project and routes a contradicting fix to the design skill" left it lists
them, and the count is taken again, per term, by counting the files that
`grep -liw -- '<term>' knowledge-architect/PRIMER.md skills/knowledge-architect-*/SKILL.md agents/knowledge-architect-*.md workflows/knowledge-architect-*.js`
lists, run in the agent configuration's directory.

### Every rule on what enters agent-facing text and how a change to it is shaped lives in one section of the agent-configuration skill `##admission-rules-home`

`skill@knowledge-architect-agent-configuration@admission-standard` holds the need test of
`design@agent-skills@statement-names-an-act`, the evidence an addition needs, the lapse rule, the kind of a gap,
the shape of a repair with its instruction count and net growth, the record check, and the outcomes
of a need-test failure. Every actor that writes or judges agent-facing text applies it at the moment it judges: a
session writing such text, a skill that moves text into it, the review axis on agent-facing text,
the agentic-workflow axis's auditors and repair steps, a retrospective's proposals, and an analysis
of received findings. A session writing the text, a skill's move included, reaches it through the
agent-configuration skill's trigger. The section is four fragments, the need test with the outcomes
of its failure, evidence and kind, the shape of a repair, and the record check, per
`design@agent-skills@shared-text-fragment`, with the rule on a text's size cost after the shape of
a repair, per `design@agent-skills@context-cost-of-agent-text`, and each judging actor's text hosts those it applies
whole, so the standard reaches it at its act; the repair steps of the agentic-workflow axis's
saved workflow, general-purpose agents, read it by a pointer, since a prompt of the workflow can
host no fragment. None restates more than one sentence of it, per
`design@agent-skills@restatement-size-test`.

The centralization is the owner's proposal, made so that the audit, the reviewers, an intake of
received findings and the installed skills cannot produce an addition that another of them would
refuse or reshape, the agreement `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` asks of
the reviews and the audits. The home in the agent-configuration skill is the owner's proposal too,
and it stands on the argument below. Maintaining the configuration is the activity whose skill
governs what enters it, per `goal@agent-skills@one-skill-per-activity` and the knowledge table's row
"how to perform an activity → the owning skill"; the audit, the reviews and an intake judge that
text and do not own its standard. Where the section lives does not decide when it is read: each
actor's copy of a fragment is read at that actor's own moment.

The nearest rival, the standard kept in the audit skill with the agent-configuration skill pointing
to it, loses on that row; under it, the writer of a project's configuration, the most frequent producer,
reads another subset of the rules than the audit applies. An installed agent holding the standard loses on rule 4 of
`skill@knowledge-architect-agent-configuration@where-text-goes`: a session writing configuration
reads the standard in the main session, which loads a skill and reads an agent's definition only by
opening its file.

The design this decision did not keep: `rejected@agent-skills@admission-standard-in-primer`.
