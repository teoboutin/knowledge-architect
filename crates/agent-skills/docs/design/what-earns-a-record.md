# Agent skills design: What earns a record

## What earns a record

### A decision earns a design head only when an entry test passes `##a-head-is-owed-by-an-entry-test`

A decision earns a head in a design home only when one of the four entry tests of
`primer@design-heads` passes: reversing it would change an interface others consume, a type or
a signature crossing a separately built unit, a file format, a document grammar or a command line;
the same reason must be respected at more than one site, or at none, a name or a path repeated for
consistency being no reason; its argument turns on the behaviour of something outside the project,
an external specification or a tool's behaviour, documented or measured; or it records the owner's
own intent, a ruling an agent could reverse as a small fix or an argument the owner wants kept,
which the owner confirms when asked, since an approval of the agent's proposal or a hedged
statement is not intent and only the owner can tell. Every other
decision lives in a comment at its code, a `%%` line for the installed text, and in the commit that
argued it, where it binds as intent at that scale, per `design@agent-skills@local-intent-binds`.
For a decision about the installed text, `design@agent-skills@instruction-record-is-minimal` admits
one case the four tests do not: one that keeps two parts of the workflow consistent.

The tests keep the decisions recorded few, as long as no critical intent or argument is lost. Every
grounding reads the design homes whole, and a human overseer reviews them, so each head costs every
later session and every review. The owner stated the trade-off: more records, which over time bloat
the design homes and make them harder for human overseers to review and for agents to ground on, or
fewer, kept to the decisions that bear load, which leave to the agents' judgement what is not argued
in them. Every session that changes local code reads the comment at that code, where a head reaches
it through the reading that `design@agent-skills@code-change-reads-its-record` asks for or through a
citation in the comment. The decision serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: a later session can tell what it
may change and what a change costs from the head where one is owed, and from the comment where none
is. The nearest rival of keeping the recorded decisions few, a head for every decision discussed
with the owner, records a mechanism carried at one site by its comments, whose intent, on the
owner's word, local code comments carry easily.
Whether the heads cost a session more than they save is not measured:
`issue@agent-skills@the-retrospective-counts-no-review-cost`.

Test 2 is
where the checker's reach ends: a reason that several sites must respect needs a home each site can
cite, and a comment cannot be cited, while a policy no code and no text states, or an absence, has
none; for agent-facing text, each text that states the instruction is one site, a design head
that restates it included, other than the decision's own, which would otherwise earn itself, so a
reason stated by one text lives beside its instruction. A head that
restates an instruction is read and acted on as the skill is, so the reason must reach it too; an
audit that counted the skill alone moved a head to a `%%` line while a second head still stated its
rule, and the head was restored. The rival reading, a policy with no code of its own as the
case of installed text, fails: an audit counted two skills stating one decision as one site, and the
clause could be read to admit every decision about installed text. Test 1 covers
every interface others consume, not only a type or a signature: a head about a grammar, a file
format or a command binds every adopting project, and under a strict test 1 such a head enforced in
one module would pass no test. Test 3 counts a tool's behaviour measured as well as documented: a
measurement costs as much to take again as a documentation reading to derive again, and a
documented-only test 3 fails where the documentation is silent. Test 4 is the owner's: on the
owner's word, an answer of "yes" or "approved" to the agent's question carries no intent of the
owner and is no ground for a head, since a ruling that grounds one carries the owner's own intent;
on the owner's word, what the owner says in a discussion of the design skills should not all count
as the owner's intent, as the owner read a proposed option to make it; the owner
is asked rather than the agent judging, on the owner's suggestion, which the owner judged more
reliable; and the questions are numbered, at the owner's request, so that the owner answers
each without confusion. The agent's own judgement of the owner's intent, its rival, misread a
hedged proposal as a ruling twice in the session that made the test. Its other rival, any recorded
ruling of the owner, admits nearly every head harvested from a discussion, since the workflow asks
the owner to confirm most decisions. Within test 2, "the same statement at more than one site"
admits a path repeated for consistency, which on the owner's word has no reason to change, its
repetition being basic self-consistency. The wider test 2, "constrains work that has not been
built", admitted nearly every head under a wide reading and almost none under a narrow one, and the
design homes hold built intent, per `design@agent-skills@design-home-is-built-intent`.

The design this decision did not keep: `rejected@agent-skills@entry-test-for-unbuilt-work`.

### A design not kept earns an entry by the recording tests, not by having been rejected `##losing-alternatives-filter`

A design not kept earns an entry in the `rejected` home only if it passes the tests of the
installed decision-recording skill: it would change an interface others consume, as entry test 1
of `primer@design-heads` defines it, its reason rests on a reading of an external
specification the project implements, it was refuted by evidence that cost work, or a doubt
remains that the design kept meets every goal. The rest stay in the spec and the commit message.
`goal@knowledge-architect@design-is-recorded-with-its-arguments` asks that a later session need not
derive an argument again: an alternative refuted by reasoning is derived again in the one round its
proposal costs, and one refuted by evidence is not. Recording every losing thread of
every discussion would grow the `rejected` homes by proposals nobody would raise again, and a
reader proposing an alternative would have to search through them to find the few whose refutation
cannot be derived again in one round.

The design this decision did not keep: `rejected@agent-skills@every-losing-thread-recorded`.

### A thread is a candidate when a shape lost to an argument, whatever its state `##losing-shape-test`

A ruled-out thread, a thread withdrawn with a defeating reason, and a superseded thread whose
distinct shape lost are each judged by `design@agent-skills@losing-alternatives-filter`. A thread
withdrawn with no defeating reason, and a superseded thread whose shape the absorbing thread
carries whole, stay in the spec only. So does a shape that lost while its question stays open,
since the question is an issue. A shape rejected with nothing built in its place is judged by the
filter like any other, and the decision not to build it is a head of absence, "we do not do X",
which entry test 2 of `primer@design-heads` admits and which cites the entry: the owner came to judge
such a head the right solution where the design is not built. The state of a thread records how it
closed, not whether an argument defeated a shape, and a rule keyed on the state would drop a
superseded thread that lost on its merits.

### A design head cites the `rejected` entries its argument covers, and an entry names no design it lost to `##rejected-citation-direction`

A design head names each `rejected` entry its argument covers: inside the sentence where its
argument meets that design, or in a closing sentence of its body, never between two sentences of
one argument, where it would take the next sentence's antecedent. A `rejected` entry names no winner; `show` on it
lists every citation of it, with the file and the line of each. A design rejected with nothing built in its place is cited by a
head of absence, per `design@agent-skills@losing-shape-test`, and the checker reports an entry that
no design entry cites, per `design@core@orphan-rejected-entry`.

It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments` and
`goal@core@records-reach-their-reader`: the owner observed that rejected alternatives are missed in
design sessions and come back at the review of a plan document or after the work, and a design
session reads heads. The hand-written direction sits in the head, the text that session reads, and
the link back is `show`'s to compute, since an entry never lists what references it, per
`design@agent-skills@a-reference-claims-a-revisit`. The owner judged the framing of a winner and a
loser wrong, and an entry that names no winner drops it. The designs this decision did not keep:
`rejected@agent-skills@entry-names-the-decision-it-lost-to`,
`rejected@agent-skills@both-directions-written-by-hand` and
`rejected@agent-skills@both-directions-checked-for-symmetry`.

### A subject is split when `klarch worklist` names its cluster above the bound, or when its heads answer questions about parts that change apart, by a standard every writer and judge of a head hosts `##subject-split-standard`

The standard is the fragment `path@agent-skills@fragments/subject-split.md`: a level-two subject
holds the heads a piece of work reads together, one part of the Component; it is split when the
work list's notice names its cluster above the bound of `design@core@clusters-packed-under-a-word-bound`,
or when its heads answer questions about parts of the Component that change apart; a split moves
headings and decides nothing, so it is a fix in a commit of its own, per `primer@met-outside-the-task`.
Its hosts are `skill@knowledge-architect-decision-recording@current-design`, the writer's, and the
two judges of a head, `agent@knowledge-architect-decision-record-reviewer` and
`agent@knowledge-architect-design-record-auditor`, per `design@agent-skills@shared-text-fragment`;
each judge reports a subject the standard says to split.

The direction is the owner's: a standard of how to split, and a strong incentive to split early;
the owner chose the notice and the split as a fix of its own for the incentive, and asked that the
decision-recording skill, its reviewer and the design-record axis of the audit look for subjects to
split. A cluster above the bound is read whole by one agent, and thaum's state-model subject held
55,103 words at thaum's commit edd2cb5d, so a split keeps an agent's reading within what agents were
seen to read whole. The citations between heads give no split point, since most subjects are one
connected group of heads beside a few that cite nothing of theirs, measured over this repository at
ed9dd60 and over thaum by the scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds, so the second test
stays a judgement. On `goal@agent-skills@installed-text-leaves-room-to-judge`, the owner's reading is
that the goal covers a choice where either shape leaves the workflow working, and the split is what
keeps it working. The delivery to a judge by a pointer loses to the fragment by
`design@agent-skills@shared-text-fragment`. The design this decision did not keep:
`rejected@core@split-enforced-by-a-check`.

### A design thread's slug becomes its entry's slug `##thread-slug-is-entry-id`

A thread of a design discussion is named by a slug minted in the entry-id grammar and checked for
a collision with the design entries and the `rejected` entries of the Component before it is used,
since a thread that loses may become a `rejected` entry under its own name. In discussion prose it is written
plain, with a hash sign before it, never as a backticked span, which the checker reports as the
retired slug form once an entry carries that slug, per
`design@core@retired-slug-reference-reported`. When
the thread is approved, or closed `argued`, and its decision earns an entry, the entry's heading ends with the same
slug, except in the case below, so the spec, the commit messages and the design home name the decision with one identifier.
In the plan document the thread is an item, a level-three heading ending with the same slug under
the Threads section, cited from inside that document only, per `design@core@plan-items-by-section`.

A thread is named by the decision it would record if approved, not by the change it proposes. A
thread's slug is minted in the round it opens, and the approved shape can drift from the proposal,
so the rule has one exception: when the thread's slug misdescribes the decision as approved, the
entry takes a slug that names the decision, and the plan document's harvest row, or the commit
message on the in-change path, states the pair. A slug that misdescribes its decision misinforms
every reader of every citation, and the alignment rule of `skill@knowledge-architect-decision-recording@head-shape` orders a rename
in that case; without the exception, the two rules would give opposite orders, against
`goal@agent-skills@no-contradiction-in-workflow`. The rival, renaming the thread during the discussion
once it drifts, lost: every earlier delta carries the old slug, and the assembly that reads them
from the transcript, per `design@agent-skills@ledger-from-transcript`, would read one thread as
two.
