# Agent skills design: The configuration a project holds

## The configuration a project holds

### The setup sets `pipefail` in the shell of the `claude` harness, beside the primer's rule against chaining on a piped verdict `##harness-shell-fails-on-a-pipe`

`skill@knowledge-architect-setup@harness-shell` writes `"env": {"SHELLOPTS": "pipefail"}` into the
file `settings.json` of a project's checked-in `.claude` directory, for the `claude` harness, and a
project that moves its pin adds it, per the Migration entry of the version that ships it. Every
shell the harness starts then fails a pipeline when any command of it fails, so a command chained
after a piped verdict does not run over a failing check. The primer's rule against that chain stays,
for `;`, for any other shell, and because a filter drops lines in any shell, per
`goal@agent-skills@installed-text-works-in-any-environment`: the option is bash's, and the setting
the `claude` harness's. The setup delivers the setting, as
`goal@knowledge-architect@setup-brings-quality-tools` asks of the tools a project gets at its
adoption.

The rule alone did not hold: sessions chained a push or a commit on a piped verdict in five of nine
sessions of one adopting project, three times pushing a commit that failed the gate, and five times
in one session of this repository, each time to keep only the verdict line. A lapse that recurs at
one moment is repaired where it is delivered, per
`skill@knowledge-architect-agent-configuration@admission-standard`, and the option delivers it at
every command. Measured: with `SHELLOPTS=pipefail` in the environment a shell starts with,
`false | tail -1 && echo ran` prints nothing and exits 1, a passing check piped to `tail` still runs
its chained command, and this repository's gates pass with the option inherited, re-taken with
`env SHELLOPTS=pipefail cargo x gates`. Its costs fail safe: a producer cut short by `head` exits
141, and a `grep` that matches nothing fails its pipeline, so a command chained after either does not
run. Set as a prefix of one command, `SHELLOPTS=pipefail bash -c …`, the variable is refused as
read-only and the command runs without the option, so the setting goes in the environment the
harness gives its shells.

The owner ruled that the setup sets it, over a recommendation, and that existing projects are told
by a Migration entry. The rivals: an option of `check` and `commits` that prints the verdict alone,
`rejected@core@verdict-only-output`, which removes the motive for a session that uses it and stops
no pipe; the checker refusing a pipe,
`rejected@core@check-refusing-a-pipe`, which a shell without the option makes powerless and the
option makes needless; the install writing the setting, which edits a file outside the installer's
namespace, against `design@core@owned-namespace-check`.

### The workflow's rows of the knowledge table ship in the primer; the project's own rows sit in its root CLAUDE.md `##knowledge-table-home`

The knowledge table maps each kind of statement to its one home. Its rows for the workflow's own
records (a goal, a decision, a losing alternative, an issue, a tripwire, a contract, a plan
document) ship in the installed primer, and move with the version. A project's own rows, such as
its changelog or a register it declares, sit in its root `CLAUDE.md`, which the project
owns. A table written whole into each project at setup would keep the old routing after an upgrade
that changes it, against `goal@knowledge-architect@agents-get-a-complete-workflow`. A table shipped
whole would leave a project no place for its own rows, against
`design@agent-skills@overlay-by-separate-skills`. A project row may refine a primer row with what is
the project's own, such as a README that is also its package's page; it never contradicts one, and a row
that only repeats one is removed.

### The primer is the part of every adopting project's root CLAUDE.md that is the same in every project: what every session needs and no installed skill delivers `##primer-limit`

The root `CLAUDE.md` imports the primer, which the checker requires of every project serving the
`claude` harness, so the primer reaches every session of every installing project, every subagent
included. The owner set how it can be read: as an extension of every adopting project's root
`CLAUDE.md`. Its content test follows from that, and is the bound, not a count of lines: it holds
what every session needs and no installed skill delivers at the moment it is needed. A statement
that the setup skill would have every root `CLAUDE.md` write alike belongs in the primer, the
import line aside, which the primer cannot carry. What one project's root alone would say does not
go in it, and neither does a procedure a skill delivers when it loads. On the owner's proposal,
the agent-configuration skill states the framing where a root `CLAUDE.md` is written, in
`skill@knowledge-architect-agent-configuration@where-text-goes`: the root never restates an
instruction of the primer. The workflow targets
frontier-tier models, which the design skill's work requires, per
`design@agent-skills@frontier-tier-only`, so a size limit would protect a reader the workflow does
not serve.

Its members today, beside the knowledge table and the list of installed skills, each with the head
that argues it where one does: the rule on when to write a reference,
`design@agent-skills@a-reference-claims-a-revisit`; the goals binding and decisions binding as a
presumption, `design@agent-skills@goals-bind`; the routing of a question by its authority,
`design@agent-skills@questions-go-to-their-authority`, since every session meets questions and a
skill that routes them reaches only the sessions that load it; the owner's word bound by its premise,
`design@agent-skills@owner-word-holds-as-far-as-its-premise`; the labels and an answer's authority,
`design@agent-skills@ruled-items-labelled` and `design@agent-skills@answer-keeps-its-authority`; the
room to judge, which states `goal@agent-skills@only-needed-instructions`, since every
session meets a situation no instruction covers; built intent
and a reason at the code, `design@agent-skills@design-home-is-built-intent` and
`design@agent-skills@local-intent-binds`; the verification rules,
`design@agent-skills@verification-in-primer`; `primer@working-prose`, whose reasons stand beside
them in the primer; the outcomes of something met outside the task,
`design@agent-skills@met-outside-the-task-outcomes`; the "Decided" section,
`design@agent-skills@decisions-reported-to-the-owner`; the checker's model and its CLI facts,
`design@agent-skills@checker-model-in-primer`, the staged check before each commit among them,
`design@agent-skills@staged-check-before-each-commit`; every commit judged at the branch's tip,
`design@agent-skills@branch-commits-judged-at-tip`; the rules on design heads every session applies, what earns a head and what a head stands on,
`design@agent-skills@one-home-for-head-rules`; how a size cost of agent-facing text is
weighed, `design@agent-skills@context-cost-of-agent-text`, since every session weighs where a text
goes; the reading of the record of code before changing it,
`design@agent-skills@code-change-reads-its-record`; the paragraph that opens the list of installed
skills, on loading each skill whole at its moment, whose second sentence is
`design@agent-skills@installed-skill-takes-precedence`; and the terms the primer uses, each restated with
its pointer, `design@agent-skills@term-defined-at-first-load`.

### The primer states the checker's model a reader needs, and the rarer checker rules reach a session through their findings `##checker-model-in-primer`

`primer@checker-model` states the manifest, the anchors, the documents every Component carries,
the registers and how an entry is defined, every reference kind and what it names, how an entry is
read, and how the checker is run and its verdict read. Every session reads references before it
writes any, and a finding arrives only after something is written, so the model is needed from a
session's first read, which no skill delivers: the setup skill, which states the layout, loads only
when a project adopts the workflow or moves its pin.

The nearest rival, the binary printing the model and the primer pointing to the command, loses on
delivery: the harness puts the primer in every session's context, and a command reaches a session
only if it thinks of running it. Drift between the primer and the checker is bounded by
`design@knowledge-architect@version-lockstep`, and by the core's test
`the_primer_s_model_section_names_every_kind_and_reserved_word`, which fails when a reference kind,
a built-in register or a reserved anchor word is missing from the section.

The section states no rarer rule: the candidate rule, the retired forms, ignored targets, the path
to anchor, a register's home cited by its path, a plan item's scope, a slug outside a definition
site. Each reaches a session through its finding, which names its repair, on the owner's rulings in
three passes of the first agentic-workflow audit: the rarer checker rules are delivered by their
finding messages rather than by the installed text, and an unstated checker rule is a finding only
where its finding message does not name its repair. One rule stands in the section on the owner's
word, which asked for a short explanation in the primer: `check --fix`'s refusal and the two repairs
it names, which sessions of an adopting project routed around in real use. The owner accepted its
placement in this section, over that ruling. The rulings are recorded in
`issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`. A rule a writer of
every `path` reference needs stays in `primer@where-knowledge-goes`.

The design this decision did not keep: `rejected@agent-skills@checker-model-printed-by-binary`.

### The primer states the rules for verifying a claim before relying on it or writing it, and a skill keeps only the narrower rule its own moment needs `##verification-in-primer`

`primer@verify-before-relying` holds them: a claim about how anything behaves is established from
the implementation or a run, the whole path is followed, one case is no convention, the evidence
is reported with the conclusion, a conclusion is re-verified before acting, a transfer names its
property; a checkable claim is checked before it is written, a commit message's claims are listed
and checked before the commit, and the same holds for an answer to the owner; what is already
recorded is checked before diagnosing; the record of code is read before changing it, per
`design@agent-skills@code-change-reads-its-record`; precedent is not authority. The design skill's grounding
reads the section again, and its steps and the planning skill point to it. Each skill's and agent's narrower rule,
such as a reviewer's "establish the state of the tree yourself", stays where its moment needs it.

The evidence of real use that `design@agent-skills@behaviour-additions-need-real-use` asks is the
owner's: the owner reports that these rules, built over work with agents in several projects,
prevented many difficulties there, and names the mechanism: agents tend to believe that things will
work out easily, and that the project and its tools have intuitive, known behaviour, which is often
not the case. Every session meets claims, so the rules pass
`design@agent-skills@primer-limit`. The nearest rival, the primer pointing to each skill's narrower
rule, loses on the five directives no installed text stated in any form: following the whole path,
a transfer's property, a commit message's claims checked by its writer, precedent, and confidence
uncorrelated with correctness.

### Before proposing a change to code, a session reads the record of that code, wherever its task came from `##code-change-reads-its-record`

`primer@verify-before-relying`, in its subsection "Read the record of code before changing it",
sends a session about to propose a fix or any other change to code to what the project decided
about that code: the entries its comments cite, the entries that cite its file, which
`klarch show path@<anchor>@<file>` lists for a file of any language, and the heads on its subject
in the design home of the Component that holds it, with the entries on it in its `rejected` home. A
fix that would contradict a head, or bring back a design the `rejected` home records, is a
decision, sent to the design skill by the backstop of `primer@design-heads`, per
`design@agent-skills@new-or-reshaped-head-needs-design`.

It serves `goal@knowledge-architect@agents-work-without-drift`. The evidence of real use that
`design@agent-skills@behaviour-additions-need-real-use` asks is the owner's, given from memory: in
thaum and in rootpoint, where the owner met the problem mostly during bug investigations, a session
investigating a bug was not really informed of where to look for known design, and looked for the
issues but mostly skimmed them, which mattered little to the investigation and started to matter
once it proposed a fix. The mechanism: the only texts such a
session meets that say what to read, the primer's "Check whether it is already known" and
`skill@knowledge-architect-issue-tracking@read-before-diagnosing`, name the issue and tripwire
listings and no design home as a place to read, while the design skill's grounding, which names
them, reaches only work that goes through it. Every session that changes code meets the moment and no skill delivers
the reading outside the design path, so `design@agent-skills@primer-limit` admits it. It is no
search of the whole record: `design@agent-skills@record-searched-before-the-work` keeps that search
for the work that goes through the design skill, a plan or an intake.

The nearest rival, the same sentence in the issue-tracking skill's section on diagnosing, loads at
the diagnosis of a new problem and reaches no change that is not a bug. Sending every fix through
the design skill's grounding dispatches the record search, beyond the minimal fix the owner said
would do for now. Whether a fix dispatches the search is decided by
`milestone@plans@development-and-components`, in its thread on the grounding of a fix, which its
slice 2 builds: a fix runs no search, and the development skill's grounding reads the record of the
code it changes.

### The goals are the only statements assumed to come from the owner, and a recorded decision binds as a presumption `##goals-bind`

`primer@goals-bind` states it, and the design skill derives its criteria from it: a criterion from
a goal binds outright, one from a decision binds as a presumption. A recorded decision was reviewed,
but its review can miss a detail or an implication, more often as the volume of agentic work grows.
When a decision conflicts with a goal, the likely cause is that the owner missed the conflict: the
goal prevails, and the conflict goes to the owner, per `goal@knowledge-architect@the-owner-decides`.
The rival, every recorded decision binding as the owner's word, would let an unreviewed implication
of an approval overrule a goal.

### Every commit of a branch is judged by the checker at the branch's tip, so a change that sets what that checker requires goes in the branch's first commit `##branch-commits-judged-at-tip`

`primer@branch-commits` states it, where a project's gates run the commits check over a branch: a
change that sets what the tip's checker requires of every tree goes in the branch's first commit,
and a repair that would leave an earlier commit failing is folded into it. The planning skill meets
one case, a plan document merged before work that changes what the per-commit gate checks, per
`design@agent-skills@plan-landing-is-not-tied-to-its-work`, and the review skill another, a fold,
per `design@agent-skills@review-repair-appended-or-folded`. Every session commits, and each skill
meets only one case, so the rule is the primer's. The rival, judging each commit by the checker of
its own tree, would let a branch that adds the manifest in its last commit pass with every earlier
commit unjudged.

### A question is answered by its authority, and the owner answers only what no authority settles `##questions-go-to-their-authority`

`primer@question-authority` routes each question to what answers it: what a specification the
project implements says, to its text; what the code or a tool does, to a measurement; what follows
from the goals and the recorded decisions, to the argument from them. The session answers those
itself, cites the source and tells the owner. The owner answers a goal to set or change, a fact
only the owner holds, such as a plan or what the owner observed, a contradiction no argument
resolves, an action that cannot be undone, and a fork: two or more shapes
that meet every binding constraint with no fact defeating either. A cost alone makes no fork, since
a shape that fails a goal or loses to a fact is out whatever it would save. The section prevails
over an installed instruction that sends to the owner a question it gives the session, as a goal
prevails over a decision. Which further actions a project reserves to its owner is the project's
to state.

It serves `goal@knowledge-architect@the-owner-decides`, which gives the owner what the argument
leaves open and counts a question the record answered, put to the owner, as the goal unmet. A workflow whose only authority is the owner
sends every question to the owner, and records every answer as a ruling. In thaum, which implements a published rules text, at least three of
the eleven questions one session put to the owner were answered by the rules, and in this
repository the owner reported many defaults put back to the owner that, the owner believes, the
recorded design heads should answer, and which appear to have been proposed without reading those
heads; the owner thinks the workflow sends such a question to the owner before it looks at the
heads. The nearest rival, a sentence that gives an external specification alone a rank above
the owner's word, loses on that second instance: the mechanism runs in a project that implements
no specification. A second rival, sending to the owner every choice that weighs a cost, sends every
choice, since nearly every choice has a cost or can be made to have one. The rank sentence holds because an edit of a skill can bring
back a routing to the owner, and with it no session meets two instructions it cannot both obey,
as `goal@agent-skills@no-contradiction-in-workflow` asks. The
owner thought that a list keeping only goals, contradictions and irreversible actions with the
owner left too large a scope to agents. The fork is the fusion of that list with one that kept scope and weighing with the
owner too, and keeps with the owner only the weighing no argument settles. A merge is no action that
cannot be undone, since a revert commit undoes it, and whether agents merge is left to each project,
as the owner directed. A fact only the owner holds is among the kinds because a premise of a head
can be one, per `design@agent-skills@heads-stand-on-argument`, and a ruling on intent is defined by
these kinds: an intent is a goal to set, and a weighing the owner makes is the owner's at a fork.

The design this decision did not keep: `rejected@agent-skills@delegated-decision-state`.

### An answer of the owner on a reading or a fact is recorded with its source, never as the owner's ruling `##answer-keeps-its-authority`

`primer@labelled-items` states it beside the label rule of
`design@agent-skills@ruled-items-labelled`: an answer of the owner on a question that a source, a
measurement or the record answers is recorded as a reading or a fact, with its source. Where the
source leaves the point open and the owner chose, the record says the owner chose among the
readings the source permits.

It serves `goal@knowledge-architect@the-owner-decides`, which sends to the owner only what the
argument leaves open, so every other decision is recorded on its own ground. An answer recorded as a ruling
weighs on every later session as the owner's intent: the owner observed that questioning such
answers is discouraged, or that they come back to the owner even when the answer is obvious and the
owner may not hold it. The nearest rival, leaving
the recording rules as they are since fewer such questions reach the owner under
`design@agent-skills@questions-go-to-their-authority`, loses because a question still reaches the
owner by mistake: in thaum, one went to the owner through a definition the session had not looked
up, and its answer became "the owner's ruling" in every later text.

### A word of the owner holds only as far as its premise, and an answer on a reading gives way to its source `##owner-word-holds-as-far-as-its-premise`

`primer@owner-word-premise` holds three rules. A ruling does not rule on the case where its
premise is false, and a premise the session supplied is the session's to check before the owner
rules on it. An answer of the owner on a reading or a fact that the source or a measurement
contradicts gives way to the source at once, the owner told, and a part of the work that cannot be
undone waits. A ruling on intent, a word on a question `primer@question-authority` reserves to the
owner per `design@agent-skills@questions-go-to-their-authority`, whose premise proves false is a contradiction: the corrected premise goes to the owner, and
the part of the work that rests on it waits, the rest going on. Whether a project follows a
specification is the project's goal or decision, and a departure from one is a scope decision the
project states, never inferred from an answer to a reading question.

The three rules sit in one section because the second and the third are the first's two cases,
split by the authority of the question, per `tripwire@agent-skills@split-rule-from-exception`. The
hold derives from `goal@knowledge-architect@the-owner-s-word-is-verified`. The source rule's nearest
rival, putting the owner's answer back to the owner with the source's reading as the default, is
what thaum built first: it still asked, offered an option the specification forbids, and two of
thaum's reviewers found it contradicted the specification's authority. The design skill holds the
same duty for a closed thread, its material finding, and reaches only a session that has loaded it,
where a word of the owner can arrive in any session, which is the content test of
`design@agent-skills@primer-limit`. So the directive sits in the primer: it arrived twice outside a discussion,
in a review repair and at an entry test on the owner's intent, `rejected@agent-skills@owner-intent-entry-test`,
the second on a premise the session supplied, that the decision failed the first three tests, when
the same change had made the second pass. A word acted on past its premise records a decision the
owner did not make, against `goal@knowledge-architect@the-owner-decides`.

The design this decision did not keep: `rejected@agent-skills@owner-reading-answer-put-back`.

### Something met outside the task takes the first outcome of the primer's table that applies, routed by its authority, unless its fix lies in the installed text or in the checker, and the turn lists every item `##met-outside-the-task-outcomes`

`primer@met-outside-the-task` gives four outcomes, the first that applies. An item that bears on
the work is routed by `primer@question-authority`, per
`design@agent-skills@questions-go-to-their-authority`: what a source, a measurement or the record
answers, the session decides and reports; a fork, a goal or a fact only the owner holds goes to the
owner with a default the work proceeds on, a goal standing as it is meanwhile; a contradiction, or an action that cannot be undone, holds the part of the work
that rests on it, as a contradiction does wherever it is met, per
`goal@knowledge-architect@the-owner-s-word-is-verified`, and as `primer@owner-word-premise` and the
design skill's material finding say. A fix checkable from the diff alone lands in a commit of its
own; an item whose `Why it matters` and `What would close it` can be written is opened as an
issue; any other is named, with why it is dropped. An item whose fix lies in the installed text or
in the checker goes to the retrospective's workflow file instead of a fix or an issue entry, but the
entry beside a pointer no checked form expresses, per
`design@agent-skills@installed-fix-goes-upstream`. A turn that met anything ends with a section
listing every item and its outcome, and a subagent that edits nothing reports each item to the
session that dispatched it, which routes it.

A one-line mention inside a long report is easy to miss: the owner saw an agent find a small issue,
judge it too small for an issue entry, not fix it, and report it in the conversation in one line,
so every outcome leaves a record or a listed line. A fix under the second outcome needs no word of
the owner's: the owner accepts such an autonomous fix as long as it passes the test the session
wrote. The directive sits
in the primer since every session meets something outside its task.

The design this decision did not keep: `rejected@agent-skills@finding-reported-not-recorded`.

### A turn that decided what the owner could have decided ends with a Decided section `##decisions-reported-to-the-owner`

`primer@decided-section`: a turn in which the session decided something the owner could have
decided ends with a section titled "Decided", one line per decision with its ground, before "Met
outside the task" where both are owed. A design thread closed on its argument, a default the record
answered, a source applied over an answer of the owner, a premortem cause recorded as a tripwire or
dropped, per `design@agent-skills@tripwire-recording-authority`, and a tripwire's wording or scope
changed or a tripwire removed, per `design@agent-skills@tripwire-stands-on-the-standard`,
are such decisions; an implementation choice a commit message records is not. A subagent reports its
decisions under the same title, and the dispatching session carries them into its own.

It serves `goal@knowledge-architect@the-owner-decides`, under which the owner is informed of every
decision that earns durable text or a line of the "Decided" section, and can contest any of them
with a goal, a fact or an argument, and `design@agent-skills@questions-go-to-their-authority` gives
the session many of those decisions. The nearest rival, the commit messages and the pull request,
loses because a decision made and reverted within one session reaches no commit, while the owner
reads the turn. An implementation choice is left out because a list of every choice would hide the
few the owner could have made. The directive sits in the primer since a session of any activity
makes such decisions.

### Each rule on design heads has one home, a fragment hosted by every text whose act applies it, and the primer hosts those every session applies `##one-home-for-head-rules`

What earns a head, what a head stands on, and what else a head holds and how it is shaped are
fragments, `path@agent-skills@fragments/head-entry-tests.md`,
`path@agent-skills@fragments/head-ground.md` and `path@agent-skills@fragments/head-shape.md`, per
`design@agent-skills@shared-text-fragment`. The primer hosts the first two in `primer@design-heads`:
every session applies them, to tell during any task whether a decision needs a head, goes to the
design skill or belongs in a comment, and to read a head at every grounding. The third is held by
the decision-recording skill's section `skill@knowledge-architect-decision-recording@head-shape`,
beside the first two, and by the texts that judge a head; the hosts test of
`path@agent-skills@src/lib.rs` holds the lists. The decision-recording skill keeps the procedures
of writing a head. Every other text points to the rules, or restates one sentence of them with its
pointer, per `design@agent-skills@restatement-size-test`. The fragments are also the home of four
rules on heads that have no head of their own: present tense, the alignment of the slug and the
title, a decision worth a slug has its own level-three heading, and fidelity to what the owner
approved, read by its direction, per `design@agent-skills@transcript-reviewer-agent`.

The centralization is the owner's proposal: the rules on what a design head records and how are
kept consistent with each other and applicable, partial restatements of them are avoided at all
cost, and they are read whole where they are needed. The owner's proposal put them in a single
location; the split by act below keeps one home per rule instead, one file the build copies into
each host, so no copy is written by hand. This serves `goal@agent-skills@no-contradiction-in-workflow`.
The rival, restating the rules wherever they are delivered, gives partial copies that drift: an
inventory of the installed text at commit bc22d4d, re-taken by a fresh read-only subagent listing
each statement of a rule on heads outside its home at that commit, finds about ten texts restating
parts of the decision-recording skill, with nine inconsistencies between them and it.

The split by act delivers each rule right before the act it shapes: the primer is loaded once at
the start of a session and can dilute in a long one, a pointer costs a lookup the session may skip,
and the shape rules are applied only by a writer or a judge of a head.
`issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
records that mechanism in real use for another rule of the primer. What stays in the primer stands
on the owner's arguments that the rules are useful to interpret recorded design and to identify in
advance what kind of decision may need to enter a design home, and that nearly every piece of work
adds or rewrites recorded design. Reading a head loads no skill, and every subagent holds the
primer in its initial context, which a probe measures in one harness, re-taken by dispatching a
subagent of each type, with no file read and no tool, and asking it to quote a sentence of the
primer from its initial context; a subagent that cannot reopens this decision. The cost is the
primer's share of the rules, the two fragments it hosts, in every session of every installing
project, re-taken with `wc -w` over them, which `design@agent-skills@primer-limit` admits for what
every session applies and no skill delivers at the moment it is needed.

The rival that keeps the whole section in the primer, read again before recording,
`rejected@agent-skills@head-rules-whole-in-primer`, delivers the rules at the act only through a
reread the session may skip. The rival that makes the whole section one fragment hosted by the
primer and by every writer and judge, `rejected@agent-skills@head-rules-one-fragment`, keeps in
every session's primer the shape rules no act there applies, measured with `wc -w` over
head-shape.md but its `%%` lines, the cost `design@agent-skills@context-cost-of-agent-text` counts.
The rules in the decision-recording skill alone, `rejected@agent-skills@head-rules-in-decision-recording`,
leave the parts every session applies unread at a grounding.

### A directive is restated only where the restatement is no longer than a pointer to it `##restatement-size-test`

A directive is restated at its point of delivery only when the restatement is no longer than a
pointer to it: a path, a file name, a command, a value, or one sentence, a sentence carrying its
pointer beside it. A longer directive is delivered by a pointer to its home, read whole at that
moment, and more than one sentence of it is never restated. In the shipped workflow, a standard, or
a similar large block of coherent instructions, read whole at several places is instead a fragment
that the build copies into every text that applies it, per `design@agent-skills@shared-text-fragment`:
no copy can drift from its one file, so the primer counts it no restatement. For installed text, the home a pointer names is
installed text, since installed text cites no entry of the project, per
`design@agent-skills@shipped-text-cites-no-entry`. The rule governs new text and text a change
touches: an existing longer restatement is converted when a change edits what it says, and no sweep
converts them all; re-pointing a reference in it does not count as an edit, since it changes no
directive. A sweep would convert at once every long restatement a project holds, each of them a
delivery choice of its owner, where a conversion at an edit costs one read of a text the change
already reads. The rule is the primer's `primer@where-knowledge-goes`.

The test is the owner's proposal: a restatement significantly bigger than a pointer is replaced by
the pointer. Its reason is the owner's too: restating a directive wherever it is delivered is meant
to keep unrelated text out of a session's load, a path or a file name costs no more than its
pointer, and a skill cannot be restated inside every other one. One sentence is the checkable form of that size, since a pointer with its instruction to
read the home whole is itself about one sentence. A partial copy drifts, and its reader takes it for
the whole: the rival, a directive restated wherever it is delivered, gives the partial restatements that
`design@agent-skills@one-home-for-head-rules` measures. That rival's own reason, that a reader who cannot reach a statement at the moment
of acting is not served by a pointer, stays answered for a short directive, which is still restated,
and for a standard by the fragment, whose copy is written by the build and not by hand.

The design this decision did not keep: `rejected@agent-skills@directive-restated-everywhere`.

### The size cost of agent-facing text is counted in the context of the sessions that load it, never in the text's length `##context-cost-of-agent-text`

The rule is `path@agent-skills@fragments/context-cost.md`: a statement a session applies to its act
costs that session nothing when the text it loads delivers it; a pointer the session must follow
costs more, the reference resolved, the location found and the target read, in context and in time,
and the risk that the session acts from memory instead; a size cost exists only for a session that
loads a statement its act does not apply. The primer hosts it in `primer@where-knowledge-goes`,
beside the restatement rule, and every host of the shape of a repair of
`skill@knowledge-architect-agent-configuration@admission-standard` hosts it after that fragment.

The rule is the owner's proposal: the owner thinks sessions of this project tend to count as a cost
the length of a text a session already had to read, and asked for a rule on how a size cost is
weighed. Its wording and its two locations stand on their
arguments, which the owner approved. The primer reaches the sessions that weigh where a text goes
without loading the agent-configuration skill: both misattributions of the session that wrote the
rule were made with that skill unloaded, one mapping the installed text, one discussing a design,
which defeats the rival that keeps the rule in the admission standard alone. The rival that keeps
it in this repository's root `CLAUDE.md` loses because every project that installs the workflow
weighs where its text goes, in its audits, retrospectives and repairs. The rule agrees with
`design@agent-skills@restatement-size-test`, whose restatement keeps unrelated text out of a
session's load, and `design@agent-skills@primer-limit` admits it, since every session weighs where
a text goes. Its cost is the rule's length in every session, re-taken with `wc -w` over the
fragment's lines but its `%%` line.

### The primer names the plans directory the checker fixes `##plans-directory-in-primer`

The plans directory is docs/plans/ in every project, per `design@core@plans-dir-fixed`, so the
primer's row for unbuilt work names it, and the installed skills name it in plain text. A project's
own row for it would only repeat the primer's, and is removed, per
`design@agent-skills@knowledge-table-home`. A row each project declared lost: once the checker
fixes the path, a declared one could name no other.

### A project skill reaches a session through its own description and through the skill of the activity it serves, with no table mapping it to an installed skill `##no-routing-table`

A project's root `CLAUDE.md` carries no table of which project skill adds to which installed one.
Two mechanisms deliver a project skill at its moment: the harness lists every skill with
its description every session, and the description begins with MUST and names its symptom, per
`skill@knowledge-architect-agent-configuration@shaping-a-skill`; and an axis a project adds to a
review is owed through the skill of an activity whose work the review reads, per
`skill@knowledge-architect-review@review-axes`. A table restates both, and its "adds to" is a fixed
mapping between a project skill and an installed one, which each new kind of link between them
must vary, against `design@agent-skills@capabilities-not-structure`. The fact that defeated it: in
this repository's first agentic-workflow audit, no row of the table delivered anything the
description or the dispatching skill did not, and every finding on the routing side turned on
whether a row was owed. The paragraph that opens the primer's list of installed skills maps no
project skill to an installed one, so it is no table. After an install that changes the installed files, each project skill and
agent is read against the installed text that changed, not against a row.

The design this decision did not keep: `rejected@agent-skills@root-routing-table`.

### A project adds to the workflow through skills of its own, never by editing an installed one `##overlay-by-separate-skills`

An installed skill is complete on its own. A project adds its conventions through its own skills,
agents and root `CLAUDE.md`, under its own names. An installed file is compared byte for byte with
the pinned version and overwritten by the install, per `design@core@owned-namespace-check`, so an
edit to it fails the check and is lost at the next install; a project skill with the same base
name as an installed one would load beside it, not replace it. Where a project skill would have to
contradict an installed one, the installed text is wrong for that project, and the owner reports
it. A project that needs to change one instruction has no means yet:
`issue@agent-skills@patching-an-installed-skill`.

### For an activity the installed set covers, its installed skill loads with the project's skills that add to it, and no skill from outside both takes their place `##installed-skill-takes-precedence`

The paragraph that opens the primer's list of installed skills says that a skill from outside the
installed set and the project, which the environment lists for the same activity, never takes the
place of the installed skill and of the project's skills that add to it. A project's own skill loads
beside the installed one, per `design@agent-skills@overlay-by-separate-skills`. The instruction names
no outside skill, per `design@agent-skills@no-external-handoff`.

The ground is `goal@agent-skills@installed-text-works-in-any-environment`: no skill or instruction
the environment adds, the harness's own included, takes the place of an installed one for an
activity the set covers. The harness lists a plugin's skill beside the installed ones: both cloud
sessions read for `design@agent-skills@settled-means-recorded` listed a plugin's design-discussion
skill beside `skill@knowledge-architect-design`, the two descriptions opening with one sentence.
Which listed skill a session loads is a behaviour, and no session read showed a plugin's skill
loaded in place of an installed one; the owner ruled that the goal grounds the instruction where
that evidence of real use falls short of what
`design@agent-skills@behaviour-additions-need-real-use` asks, an exception that head names. The
nearest rival, the installed skill as the one skill loaded, shuts out a project skill whose
description says it loads together with an installed one.

### A project's own files in the installer's directories carry its name as a prefix `##project-name-prefix`

A project skill, agent or saved workflow is named `<project>-<activity>`, the directory or file
name equal to the frontmatter `name`, or to the `name` of a saved workflow's `meta`, so it is told
apart from an installed one and from another project's. A colon,
as in `<project>:<activity>`, is not used: a subagent's name cannot hold one, and a skill so named
cannot be told apart from a plugin's.

### Setting up stops at a conformant structure, and the move of existing documents is planned work `##adopting-existing-docs`

For a project that already has documentation, the setup skill takes an inventory of it,
proposes a destination for each document, places a document a row of the knowledge table fits,
per `design@agent-skills@questions-go-to-their-authority`, takes the owner's rulings on the others,
and opens one `todo` issue for the move. The move itself is planned work of its own, written under
the planning skill. A migration is a decision with arguments of its own, such as which recorded decisions still hold, and it fits
in one session only for a small project. The issue keeps the old documents beside the new homes
listed as outstanding work, per `design@agent-skills@undesigned-work-is-an-issue`.

### A project pins the checker exactly, and its manifest declares the pin `##exact-pin`

A project depends on one exact version of the checker, so moving it is an explicit edit, and the
installed skills move with it. **The manifest declares the
same pin**, in `[project] checker-version`, and every binary that dispatches a command of the
checker refuses to run over a project whose pin it does not satisfy, per `design@core@installed-binary-version-check`, so an install or a build
left behind by a move of the pin is refused at its first command rather than run. `klarch init`
writes the key, per `design@core@init-command`, or the setup skill does by hand, and the setup
skill moves it with the pin.

### A project runs its pinned checker from inside the project, never from a machine-wide install `##pinned-binary-runs-inside-the-project`

A dependency builds no executable for the project, so a Rust project runs the pinned version
through its maintenance crate, per `design@agent-skills@xtask-pins-checker`, which every Rust target
project carries, per `design@agent-skills@maintenance-crate-ask`; a Rust project that is one package
with no workspace gains a workspace for that crate. A project with no Rust installs the binary into
a directory of its own with `cargo install --locked --root`. A machine-wide install would give two
projects on one machine one version, which is what bundling the workflow into the checker avoids,
per `design@knowledge-architect@binary-bundles-workflow`. A project with an extension runs its own
binary under a name of its own, per `design@core@declared-command`. The binary that runs
`klarch init` is the one exception: it runs before any pin exists, from whatever install its user
keeps, and writes its own version as the pin, per `design@core@init-command`.

### The setup skill recommends one gates command, a command of the maintenance crate run by the published gates library `##gates-convention`

One command runs every check a project owes before a merge, runs them all when one fails, and exits
non-zero when any fails, so a verdict is one exit code and nothing is read through a pipe. In a Rust
project it is a command of the maintenance crate, per `design@agent-skills@xtask-pins-checker`, run
by the published gates library, per `design@gates@gates-crate`; every Rust target project has that
crate, per `design@agent-skills@maintenance-crate-ask`, so every adopting project runs the gates
refined in this repository and in thaum. The gates a project owes are its own list, per
`design@gates@a-project-holds-its-gate-list`.

### The primer asks for `check --staged` after staging and before each commit `##staged-check-before-each-commit`

`primer@checker-model` tells a session to run `check --staged` after staging and before each
commit, and to repair a failing one before the commit, beside the gates command, which judges the
branch before a merge. The command is the core's, `design@core@staged-tree-source`. The reason: the
staged tree is the one the commit records, and a file left unstaged, or a change staged in part,
passes a check of the working tree and fails the commit's own, which the gates meet only before the
merge. The primer states it because every adopting project's root `CLAUDE.md` would state it alike,
per `design@agent-skills@primer-limit`; the setup skill asking each root to state it lost on that
test.

`design@agent-skills@behaviour-additions-need-real-use` asks of an addition a behaviour seen in real use, or
asked for by the owner with the lack named. The instruction has no named lack of its own,
so it is an exception to that rule, argued on delivery: without it, no shipped text tells a session to check
before a commit, so a command that judges the commit's tree is used only where a project's own
instructions already ask for it. Its rival, dropping the instruction until a real session shows
the lack, keeps the rule and leaves the command undelivered. Its cost is an instruction no observed
session asked for.

### The skills name the staged forms of the checker's commands for a commit of part of the working tree `##staged-forms-for-a-partial-commit`

The issue-tracking and planning skills name `index --staged` and `check --staged` for a commit of
part of the working tree. Both commands are the core's, `design@core@staged-tree-source` and
`design@core@index-staged-write`. The staged tree is the one the commit records, and a change
staged in part passes a check of the working tree and fails the commit's own.
`design@agent-skills@behaviour-additions-need-real-use` asks of an addition a behaviour seen in real use, or
asked for by the owner with the lack named. The owner asked for the delivery, "identify shipped
workflow changes needed to deliver this new feature", and, on the refusal of `--fix` for a partial
commit, named what a session does instead: "the recommended action would be to run check without
--fix and restricted to staged area, and fix findings manually (fixing indexes might need index to
accept running on staged changes)". The rival, the skills naming only the working-tree forms, leaves
that action undelivered.

### A Rust project's maintenance crate pins the checker and the libraries, and runs the gates and the project's other repeated tasks `##xtask-pins-checker`

The maintenance crate of a Rust project depends on the checker, the gates library and the mutation
library, all pinned exactly, at the version `[project] checker-version` declares, per
`design@core@installed-binary-version-check`, and serves two cargo aliases: `cargo x` for its own
commands, the gates and the hand mutation among them, and `cargo klarch` for the checker's commands,
which it carries under a command of its own through the core's library, per
`design@core@the-core-cli-is-a-library-module`. The gates run through the published library, per
`design@gates@gates-crate`, and the checker they run is the one this crate pins; the hand mutation
runs through its own, per `design@knowledge-architect@mutation-command-host`. One crate does what a
crate per tool would otherwise do, and gives the project's other repeated tasks a place, per
`goal@knowledge-architect@setup-brings-quality-tools`. The cost accepted: building the gates builds
the checker, which the check gate needs anyway, and the mutation library.

### Every Rust target project has a maintenance crate whose commands run as `cargo x <command>` `##maintenance-crate-ask`

knowledge-architect asks every Rust target project to carry the maintenance crate of
`design@agent-skills@xtask-pins-checker`, so that a command of the installed text, such as
`cargo x gates` or `cargo x mutate`, names one invocation in every project. The ask is on the
owner's word; adding an ask is the owner's, per `goal@knowledge-architect@serves-one-kind-of-project`,
which also makes how an ask is met, the command's name, a design decision of knowledge-architect.
The setup skill states it, and offers a Rust project no local install of the checker, which would
leave it with no maintenance crate and a gates command of its own:
`rejected@agent-skills@rust-project-on-a-local-install`. A project with a Cargo workspace or
package of its own writes the crate by hand from the setup skill's blocks, since `klarch init --rust`
refuses one, per `design@core@rust-scaffold`.

### The setup skill proposes a short CRATES-IO.md for a crate the project publishes `##setup-default-crates-io-page`

For a crate the project publishes, the setup skill proposes to the owner a repository-facing
README.md and a separate, short CRATES-IO.md as its crates.io page, named by `readme`: the split
`design@knowledge-architect@crates-io-page-file` makes for this repository's own crates, for the
same reason. It serves `goal@knowledge-architect@adoption-is-easy`.

The owner's intent, on the owner's own proposal: "Maybe I'd also make it the default in the set-up
skill: for published Rust crates, keep the README.md as a repo facing document, and use a separate
CRATES-IO.md for what crates.io readers see." Asked whether it records the owner's intent, the
owner answered: "Q5: yes".

### Every Component states at least one goal `##goals-required`

The setup skill does not finish a Component without at least one goal, stated with the owner.
A Component with no goal gives its design nothing of its own to be judged against, and a goal that
is a Component's responsibility left unstated is one nobody is responsible for, against
`goal@knowledge-architect@agents-work-without-drift`. Nothing checks it mechanically yet:
`issue@core@a-component-states-at-least-one-goal`.

### The installed skills do not mention a project whose name is the installer's namespace `##installed-text-omits-the-prefix-collision`

The installed skills name a project's prefix rule, per `design@agent-skills@project-name-prefix`,
and say nothing of a project whose name is the installer's namespace. Outside this repository the
collision is unlikely, and a rule for it in the shipped text would be read by every installing
project. The owner: "The collision risk outside of this exact project is ridiculously small. This
would only add noise to every project that uses this workflow".
