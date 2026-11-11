# Agent skills design: The record search

## The record search

### One installed agent reads whole the entries `klarch worklist` gives one agent directory, gives each a verdict against the work, and fills the report `klarch worklist --verify` judges `##one-record-searcher`

`agent@knowledge-architect-record-searcher` takes the path of `work.txt` and one agent directory that
`klarch worklist --out` wrote, per `design@query@worklist-writes-and-verifies-the-reports`. It reads
every file of the work whole, then its entries whole, leaning to recall, with their context entries
and its goal title lines, edits its report in place and runs `klarch worklist --check` on it,
repairing each finding, before it returns, per `design@query@worklist-check-judges-one-report`, or returns it as text where the harness refuses every
write of it. The same agent grades the clusters of a cluster agent and the goals of a goal agent,
per `design@query@goal-context-once`. Its description carries the dispatch, since a dispatcher reads
an agent's description and never its body: the work list written with the work, the subjects and the
seeds, one agent per directory in parallel, the check of every report by `--verify`, a report that
fails it sent back, then the acts on the verdict files that `design@agent-skills@session-acts-on-the-verdicts`
gives.

A report that fails the check goes back to its agent and is never repaired by the dispatcher, since
choosing a verdict or an action routes an act, which is the grading this agent holds.

One agent grades every kind of entry, since a cluster holds a head beside the `rejected` entries it
cites and the issues and tripwires that cite it, per `design@query@cluster-rooted-at-a-subject`: an
agent per kind would read half of every cluster. The design this decision did not keep:
`rejected@agent-skills@searches-split-by-kind`. The search runs in subagents so that the main
session keeps its context for what needs it, the design, the decisions and the questions to the
owner, and hands the reading to the agents it dispatches, per
`goal@session-orchestrates-the-work`.

### The record search is an agent apart from the standing-state reviewer `##search-apart-from-the-standing-state-review`

The record searcher judges by relevance on purpose, and the standing-state reviewer reads every
tripwire and every deferred trigger by rule, per
`design@agent-skills@standing-state-review-before-every-merge`. The rival, the reviewer in a second
mode, puts both standards in one text, which lets a session apply the wrong one.


### A search reads the whole record, every search unit, whatever the work `##search-reads-the-whole-record`

The search gives no cluster, remainder or goals file a lighter reading because the work seems not
to touch it. The owner believed the whole record the safer reading, judged a choice by a cluster's
title bad practice here, and believed the bearing is found only from the search's results. The rival, clusters chosen
by the work, needs the judgement the work list removes, and rests on every pointer being a
reference, which `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` shows
it is not. A lighter pass for the Components a piece of work does not touch waits for a project
whose whole record costs more agents than one grounding should, per
`issue@agent-skills@a-lighter-pass-for-untouched-components`. Its cost is the packing of
`design@query@clusters-packed-under-a-word-bound`, re-taken with `klarch worklist --out` over the
tree.

### Goal agents grade every goal, beside the session's own reading of the goals its grounding names `##searcher-grades-goals`

Every goal is graded once, by a goal agent that reads it whole, per `design@query@goal-context-once`,
and each verdict names the subjects of the work it is against, so a goal that constrains a thread is
named with that thread. A cluster agent holds the goals that bind its entries by their title lines,
and reads a goal whole before a verdict rests on it. The session still reads the goals itself: at
the design skill's grounding, the root's goals home, and at the intake, the goal each finding serves or threatens. The decision is the owner's: the owner saw
design sessions miss a goal twice, although the goals homes were small enough to be read whole, and
ruled that both readings stay. How a session misses a goal it is told to read is not established;
the owner did not know. The nearest rival, the session's reading alone, is the reading those
sessions had. The cost is the goal agents' reading, each goal once, and a title line per goal in
each cluster agent it binds; `klarch worklist --out` re-takes it, printing each agent's load, and
title lines that make a large part of a cluster agent's load reopen the cost.

### The searcher gives each entry one verdict its register admits and an action its verdict admits, each verdict routing an act `##verdict-report`

The verdicts a register admits:

| register | verdicts |
| --- | --- |
| `goal`, `design`, a register the project declares | `constrains`, `conflicts`, `unrelated` |
| `rejected` | `brings-back`, `constrains`, `unrelated` |
| `issue` | `closes`, `touches`, `worsens`, `depends-on`, `trigger-met`, `constrains`, `unrelated` |
| `tripwire` | `fires`, `constrains`, `unrelated` |

The actions a verdict admits: `respect` or `none` for `constrains`, `touches` and `worsens`;
`goal-setting` for `conflicts` on a goal, since a goal changes only there, and `check-reversal` on
any other entry; `new-argument-needed` for `brings-back`; `close-with-the-work` for `closes`;
`rule-before-work` for `depends-on`, `trigger-met` and `fires`; `none` for `unrelated`.
`trigger-met` is an issue whose deferred trigger, or whose re-entry point if it is of kind `design`,
the work meets. `against` names the subjects of the work the verdict is against, or `-` on
`unrelated`, and on `constrains` with the action `none`. Each line carries a quotation, per
`design@agent-skills@every-verdict-quotes-its-entry`, and a reason of one line. An entry that is or
cites a seed is never `unrelated`, per `design@agent-skills@a-seed-and-its-citers-are-never-unrelated`. The
report closes with `## Decided` and `## Met outside the task`.
`klarch worklist --verify` checks what a command can, per
`design@query@worklist-writes-and-verifies-the-reports`; the session acts on each verdict by its
action, per `design@agent-skills@session-acts-on-the-verdicts`.

It serves `goal@session-orchestrates-the-work`, under which the session acts on
what its agents bring back. It can do so without doing the work again only on a report that names
what the agent considered and carries the evidence each conclusion rests on: the searcher checks
its conclusions as the session checks its own claims, and the command checks the quotation, the
sets and the coverage, so that a verdict carries its evidence and the session need not read the
entry to act on it. A verdict exists only where it routes an act, the action it
admits: standing-entry searchers graded a doubt anyway, unasked, in 12 of the
41 entries they returned, each wording it differently, and a numeric scale would route no act. Every
entry an agent is given takes a line, `unrelated` included, so that the coverage check tells an entry
left without a verdict, the form that names what the agent considered. The nearest
rival, grades of how closely an entry bears whose entries the session reads whole, is
`rejected@agent-skills@relevance-grades-read-by-the-session`. What a search still misses, its checks
holding nothing of its judgement, is the question of `issue@agent-skills@a-record-search-misses-what-is-added-after-it`.

### A seed and an entry that cites one are never `unrelated`, so each takes a generated line `##a-seed-and-its-citers-are-never-unrelated`

An entry that is a seed, or cites one, is never given `unrelated`: where nothing in the work meets
it, it is `constrains`, action `none`, `against: -`, its reason naming the seed, and
`klarch worklist --verify` refuses `unrelated` on it, per
`design@query@worklist-writes-and-verifies-the-reports`. So its line enters the lines
`klarch worklist --verify` writes, per `design@query@occasion-shaped-output`, while an `unrelated`
verdict enters none:

- at the design grounding, `generated-lines.md`, which the commit message carries on the in-change
  path and for bounded work, and whose entries the plan document's "What is already decided" gives
  an outcome on the full path, per `design@agent-skills@session-acts-on-the-verdicts`;
- at the design audit, `fit-lines.md`, which the audit's commit message carries, per
  `design@agent-skills@design-audit-checks-fit`;
- at an intake, `generated-lines.md`, which the session reads; a line `against: -` names no cluster
  and no finding, so no analysis carries it.

It serves `goal@session-orchestrates-the-work`, under which the session acts on
what a dispatched agent brings back, which it can do without doing the work again only where the
report names what the agent considered. A seed is an entry the session named as one the work rests on or may change, per
`design@agent-skills@seeds-name-decisions`, so an `unrelated` verdict on it would contradict the
session's own naming with no line to show it: as `constrains`, action `none`, the line tells the
session that the work meets nothing in the seed, and the session keeps the seed or drops it. An
entry that cites a seed stands on that decision, so a judgement that the work meets nothing in it is
a claim the occasion's record shows only as such a line. The nearest rival, a seed and its citers
graded like any other entry, leaves that judgement in the verdict lines of the searcher's report
alone, which the session does not read, since it reads the verdict files and the generated lines.
Its cost is a line per citer of every seed: the design grounding and the design audit seed no goal,
which many entries cite, per `design@agent-skills@seeds-name-decisions`, while the intake seeds the
goals and the design heads its findings name.

### A verdict states the searcher's conclusion, never a condition, and what only the owner answers goes in its `owner-question` `##verdict-states-a-conclusion`

The searcher reads whole a goal or an entry before a verdict rests on it, resolves every doubt the
work and the record answer, and states its conclusion; what `primer@question-authority` reserves to
the owner goes in the line's `owner-question`, with the readings and what each changes in the work.

It serves `goal@session-orchestrates-the-work`, under which the session acts on a
dispatched conclusion. A doubt that names nothing gives the session nothing to act by,
`rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`. A conditional verdict, a field naming the
reading it rests on, loses to parity, as its proposer agreed: the owner argued that an agent given a
reading on par with the main session's reaches conclusions similar to its own with no condition, the
main session's one advantage being that it can ask the owner, which `owner-question` carries.

### Every verdict line quotes the entry's own text, an `unrelated` line included `##every-verdict-quotes-its-entry`

Every line, `unrelated` included, quotes the entry's own text verbatim, at least the number of words
`QUOTE_WORDS` sets in `path@query@src/worklist.rs` or the whole of a shorter text: for an `unrelated`
entry, the sentence that shows its subject, which the reason contrasts with the work.
`klarch worklist --verify` checks the quotation, per `design@query@worklist-writes-and-verifies-the-reports`.

It serves `goal@session-orchestrates-the-work`, under which the session acts on what
a dispatched agent brings back, which it can do without doing the work again only where each
conclusion carries the evidence it rests on: an `unrelated` verdict is such a conclusion, and its
quotation is the evidence a command checks. The nearest rival, an `unrelated` line with a reason
alone, is a paraphrase no command checks. What the quotation does
not hold of the searcher's judgement is a question of `issue@agent-skills@a-record-search-misses-what-is-added-after-it`.

### The session acts on each verdict by its action, checked as it checks its own claims, and reads an entry whole only where it reasons over it `##session-acts-on-the-verdicts`

Once `klarch worklist --verify` passes, the session reads the verdict files, or hands them to agents
it dispatches, at its judgement; acts on each verdict other than `unrelated` by its action, checking
it as it checks its own claims, per `primer@verify-before-relying`; reads whole, with `show`, an
entry it must reason over itself, such as one a proposal would reverse; puts each `owner-question`
to the owner as `primer@question-authority` says; and carries each report's `## Decided` and
`## Met outside the task` into its own sections. The searcher's description states these acts, and
the act each action names at an occasion is stated by the skill that dispatches the search there.
Where the occasion's record goes:

- at the design grounding on the full path, the plan document's "What is already decided" gives each
  entry with a verdict other than `unrelated` its verdict, its action and its outcome: the answer the
  discussion gave, that the design respects it, or that it stays open;
- on the in-change path and for bounded work, the commit message carries the lines of
  `generated-lines.md`;
- at a design audit, the session reads `gaps.md` and sorts each gap, and the audit's commit message
  carries `fit-lines.md`;
- at an intake, the analysis that holds a cluster of findings carries the generated line of each
  entry given a verdict against it.

It serves `goal@session-orchestrates-the-work`: the main session keeps the design and
the decisions and hands the reading to agents, and checks a dispatched conclusion as it checks its
own claims, no more and no less. A verdict carries its quotation, which the command checks, so the evidence an act rests on
is in the line; the session reads an entry whole where it must reason over the entry itself, since
there the line does not carry what it needs. The nearest rival, the session reading whole the
entries graded as bearing, lost to the size of that reading:
`rejected@agent-skills@relevance-grades-read-by-the-session`. A verdict the session misjudges has a
reader at the plan document's review and at the merge, per
`design@agent-skills@verdicts-reach-the-conformance-review`.

### The design skill's record search runs once the shapes settle `##search-runs-once-the-shapes-settle`

Loop step 1 of the installed design skill grounds a discussion: the session re-reads
`primer@verify-before-relying` whole, reads the homes the step names and greps the record, drafts
the shapes, the owner's proposal if one was made and its own, and challenges each, before the first
round. The search of the whole record runs once the shapes settle, a moment the session judges with
no count of rounds, within the bound `design@agent-skills@no-thread-closes-before-the-search` sets,
and the grounding is complete only once it has run; each round before it states the grounding it
rests on and that the search has not run. The classification after the search still decides the
path, per `design@agent-skills@bounded-path-in-design`. The findings revise the draft that is the
search's work, per `design@agent-skills@search-reads-the-draft`; the premortem and the convergence
follow. No thread closes before the search, per `design@agent-skills@no-thread-closes-before-the-search`. A thread that opens after the search on a subject its work did not cover
gets a search of its own, with the draft extended by that thread as the work and the thread as its
subject, per `design@agent-skills@record-searched-before-the-work`. It stays the discussion's one
search of the whole record, never a pass over what changed since the first rounds, per
`rejected@agent-skills@delta-search-at-convergence`, a premise of the cost argument below: a change
that splits it into several searches reads that argument again.

The argument: the search's work is a draft of the shapes, per
`design@agent-skills@search-reads-the-draft`, so it is most precise once the shapes stop moving. In the discussion
that designed the development skill and the component skill, the owner reported that the scope
shifted and expanded at least twice before the search; the search then graded the record against the
seventeen shapes drafted by then, and a search at the first challenge would have needed one more
search for each new subject. The owner argued, as a practice that may become the usual one, that a
search after the first rounds allows a freer discussion of the design, which the record then
confronts. The cost, proposals that reach the owner before the search, strains
`goal@design-with-agents-as-counterparts`, under which the owner meets each
proposal with its grounding established; each round's statement of its
grounding, and the bound on closures, keep it to proposals no ruling rests on yet.

The rivals: the search at the first challenge,
`rejected@agent-skills@search-after-the-first-challenge`, whose verdicts go stale when the scope
widens after it; a search before the challenge and another after it,
`rejected@agent-skills@search-before-and-after-the-challenge`; and a design-conformance reviewer
dispatched at the checkpoint, `rejected@agent-skills@conformance-reader-at-the-checkpoint`.

### The design skill's record search reads, as its work, the draft of the document that will carry the deliberation `##search-reads-the-draft`

The work of the search of `design@agent-skills@search-runs-once-the-shapes-settle` is the draft of
the document that will carry the deliberation, in the form of the path the session expects: on the
full path the draft of the spec, which the planning skill writes, per
`design@agent-skills@design-hands-off-to-planning`; on the in-change path and for bounded work the
draft of the commit message, which the design skill writes. Each thread is a subject of the work. The
draft is never read as the discussion's state, and the plan document is never assembled from it, per
`design@agent-skills@ledger-from-transcript`.

The argument: a searcher gives `brings-back` against a shape, and a question alone gives it none,
`rejected@agent-skills@search-work-is-the-question`. The draft is written anyway, the plan
document's or the commit message's, so a work statement written apart for the search is a second
copy of the design, which `primer@where-knowledge-goes` refuses:
`rejected@agent-skills@work-statement-written-for-the-search`. The discussion's transcript as the
work costs more than the reading it adds:
`rejected@agent-skills@search-work-carries-the-deliberation`.

### No thread of a design discussion closes before its record search has run `##no-thread-closes-before-the-search`

The installed design skill closes no thread, in any closed state and by the owner's word or the
session's, before the search of loop step 1 has run: a word of the owner that would close one leaves
it `presumed-settled`, and a closure on the argument waits. Once the search has run, each thread held
only for it closes on the word already given, with no second question, and convergence can be
proposed. An entry the search returns on a held thread is routed as `primer@owner-word-premise` and
`primer@question-authority` route it, with no rule of the design skill's own: a word on a reading or
a fact gives way to its source, a ruling on intent whose premise the entry defeats is a
contradiction put to the owner, a pick at a fork changes only in the weak form, where the entry ranks
the shapes, and an entry that defeats nothing lets the thread close on the word given.

The argument: a reading of the record that comes after the owner has ruled reads rulings already
given, as in the discussion that first decided a search of the design entries at grounding, where the
owner ruled 7 of 8 threads one by one before any checkpoint table was shown; and a ruling given
before the record was read rests on premises nobody checked against it.
In the discussion that designed the development skill and the component skill,
threads closed before the search, six or seven by its two counts; none was defeated, but the owner
approved one while restating premises a recorded issue already held. Holding every closure until the
search keeps that cost out, since no ruling precedes the record. It serves
`goal@the-owner-s-word-is-verified`, since an early word of the owner is checked
against the record before anything rests on it, and `goal@design-with-agents-as-counterparts`,
since every proposal is grounded in the record and challenged before it is built. Its nearest rival
is the owner's plain proposal, a search after a few rounds, during which, on the session's reading,
threads close, which carries that cost.

### The design grounding and the design audit seed the record search with the decisions the work rests on or may change, never a goal `##seeds-name-decisions`

The design skill's grounding seeds the search with the decisions read so far that the problem and
the drafted shapes rest on or may change. The design audit of a slice or a spec seeds it with
`--seeds-from` its spec, `design@query@seeds-from-the-plan-s-decided-section`: the design entries
and the declared-register entries the spec's section "What is already decided" cites, for a slice
its own spec and not the milestone document. Neither seeds a goal. An entry that is or
cites a seed is never `unrelated`, per `design@agent-skills@a-seed-and-its-citers-are-never-unrelated`, so each entry citing a
seed takes a line among the generated lines and the outcomes its occasion records. A goal is such a
seed, cited by many entries: it is graded by a goal agent and reaches every cluster agent it binds by
its title line already, per `design@query@goal-context-once`, and the session reads the root's goals
home, which its grounding names. In the search that grounded the discussion that made this decision, 29 of the
137 entries graded `may-bear` alone carried the mark of one goal seed, as the message of the commit
"[harvest] The graded entries' assembly, `klarch id`, the narrower grounding and the retrospective's
new question recorded" states; the lines a goal seed forces under the verdicts are re-taken over a
search's work list run with one, by
`grep -h 'cites seed:' <dir>/agent-*/report.md | grep -c 'cites seed: [^|]*goal@'`. A milestone's
whole list of decisions as seeds did the same at a slice's audit, since the slice rests on the
decisions its own spec names, not on those another slice built:
`rejected@agent-skills@slice-audit-seeded-by-the-milestone`. The intake's seeds, the design
heads and goals its received findings name, are the findings' own citations, and stay, so `worklist`
takes a goal as a seed. The design this decision did not keep: `rejected@agent-skills@goal-seeds`.

### The record search runs where design is discussed, before the work `##record-searched-before-the-work`

The installed design skill dispatches the search at its loop step 1, once the shapes settle and
before any thread closes, as `design@agent-skills@search-runs-once-the-shapes-settle` and
`design@agent-skills@no-thread-closes-before-the-search` order it, with
the draft as the work, the threads as its subjects and seeds as `design@agent-skills@seeds-name-decisions` says, for a design question and for bounded work alike,
per `design@agent-skills@bounded-path-in-design`, and again for a thread that opens on a subject the
first work did not cover, with the draft extended by that thread as the work: the first search read
the whole record but judged it against the first draft, so an entry that bears only on the new
subject may stand `unrelated`. The plan-document review of `design@agent-skills@plan-read-against-the-record` stays
the backstop. The installed planning skill dispatches it at the design audit of a milestone slice,
point 2 of `skill@knowledge-architect-planning@working-a-slice`, with the slice's spec and the
milestone document as the work, `slice-audit` as its subject, the seeds of its spec, and `--since`
the commit that last changed the spec, and at the design audit of a spec where one runs, with the
spec as the work and `spec-audit` as its subject, and acts on its verdicts as
`design@agent-skills@design-audit-checks-fit` says. The audit lists as a gap every standing
entry the slice's code bears on: a tripwire whose firing condition, or a deferred trigger, the
planned code meets, and an issue of any kind the slice's code touches, closes, makes worse or
depends on. A cause of the milestone document's premortem whose firing condition the planned code
meets is listed with them: it enters a tripwires home only at the harvest, if the standard admits
it, so the search cannot return it, and the audit reads it in the milestone document. The slice's
grounding, point 1 of `skill@knowledge-architect-planning@working-a-slice`, reads no issues and no
tripwires, since the audit's search covers every anchor. The retrospective intake sends it at its
grounding, with the received files as the work and each cluster of findings, or lone finding, as a
subject, and again for a proposed fix on a subject the files
did not raise, with the fix as the work, before its outcome is decided or put to the owner. Other work that
does not go through the design skill sends no search: the standing-state review reads its deferred
triggers, per `design@agent-skills@standing-state-review-before-every-merge`, which this search adds
to and never replaces. A search of the whole record before every other piece of work stays parked: its one host would be
the primer, which holds only what every session needs, per `design@agent-skills@primer-limit`, the
evidence for it is one instance, and a line sending every session to search before any work is a
conformance rule. In thaum, a move of the pinned checker met the trigger of a deferred issue that
named that move, and the session read it at no step. A session about to change code reads the record of that code instead, a
reading narrower than the search, which the owner's instances in thaum and rootpoint admitted, per
`design@agent-skills@code-change-reads-its-record`.

The occasions are those where design is discussed, so that an entry the work bears on costs a
ruling before the work rather than rework after it, per
`goal@agents-work-without-drift`. A search at grounding puts the record in view
before any ruling, since no thread closes before it runs, and reads the draft that the shapes have
settled into, which a reviewer dispatched at the checkpoint would read again,
`rejected@agent-skills@conformance-reader-at-the-checkpoint`. The evidence is real use, per `design@agent-skills@behaviour-additions-need-real-use`:

- for the design entries, the owner reported them missed in part many times, and in one case the
  owner approved a proposal contradicting a head the session had not read, caught only at the
  plan-document review; and an intake of thirteen received files analysed a finding about naming a
  skill not yet created and missed the exception of
  `design@agent-skills@plain-text-is-no-repair` for an entry or a skill a record of a deliberation
  is still to create, which decides that case, and the owner recalled the
  head while ruling;
- for the standing entries, in thaum the standing-state reviewer found, after the work, issues the
  changes bore on and tripwires that had fired, especially in the slices of large milestones, and
  the repairs and decisions this forced could have been taken before the design and the
  implementation. The owner's account is that the reviewer too often finds, after the work, issues
  the changes bear on or tripwires that fired and bear on the design, especially on large work
  within big milestones in thaum. The audit reads every issue kind because those late findings
  included issues the changes touched, not only fired tripwires. The slices of thaum whose reviews
  found such entries had their audits before the installed planning skill told an audit to read the
  tripwires and the deferred issues, and the reviews of the two slices audited after it found no
  entry missed. The owner judged the work before 0.2.0 not that different from the work just before
  the change, so the evidence stands; a stream of audits that read the standing entries and miss
  none would weaken it.

The nearest rival, the session's own reading of the record, is the reading those sessions had, and
the searcher is its capability form, per `design@agent-skills@capabilities-not-structure`. The
occasions are the owner's: the search runs at least at a design discussion's grounding and at the
audits of a plan's steps, the main moments design is discussed, and should miss nothing.

At the design audit the search reads every kind of entry, as at every other occasion: its unit is
the cluster of `design@query@cluster-rooted-at-a-subject`, under which a standing entry is judged
beside the heads it cites, so the audit's reading of the design entries is no instruction about how
an agent behaves, the kind `design@agent-skills@behaviour-additions-need-real-use` admits only on a
miss seen in real use. The misses of a search are in its judgement, not in when it runs: two pieces
of work on one day met, only at their review before the merge, five standing entries the search
before the work had not returned, `issue@model@a-span-with-an-empty-head-is-malformed-against-its-head`,
the issue on a premortem criterion listed among a plan document's defaults, closed when the owner's
rulings on criteria left the workflow,
`issue@check@a-plain-hash-item-name-is-read-by-nothing`, the tripwire on a ruling lost at assembly that
the commit "[plans] The spec of the graded entries' assembly and of `klarch id` leaves: its work has landed, and every default is ruled" removed, and the tripwire on attributions that the commit "[tripwires] Three
tripwires leave, their horizons passed unfired" removed, and each was recorded days before its
search ran, dated with `git log -S"##<slug>"` and `--diff-filter=A`. The work list and its check take
the grouping of the entries and the checks a report can be given from the agents, and `issue@agent-skills@a-record-search-misses-what-is-added-after-it` holds
what judgement still misses. The designs this decision did not keep:
`rejected@agent-skills@conformance-reader-at-the-checkpoint` and
`rejected@agent-skills@delta-search-at-convergence`.

### A design audit checks that its spec still fits the record and the code, a `constrains` verdict a fit and every other verdict but `unrelated` a gap `##design-audit-checks-fit`

The design audit of a milestone slice, and of a spec where one runs, asks whether the spec still
fits the record and the code as they stand, and fixes the spec where it does not; it does not design
the work again. The search's check writes `gaps.md`, every verdict other than `constrains` and
`unrelated` with its entry's text, and `fit-lines.md`, the generated lines under the spec's citation.
A `constrains` verdict is a fit, which the audit records and does not read again; every other verdict
but `unrelated` is a gap, which the session reads in `gaps.md` and sorts into the planning skill's
sorts, applying in place the gaps the spec's decisions already answer; `fit-lines.md` is the audit's
record, in its commit message.

It serves `goal@session-orchestrates-the-work`: the audit runs before every slice, point
2 of `skill@knowledge-architect-planning@working-a-slice`, so its cost recurs at each slice, and the
main session spends its context on the gaps, the part of the audit that may need a decision, while
the reading of the record is the searchers'. A fit needs no act, since a `constrains` verdict asks
the work only to keep within the sentence it quotes, which the spec already decided. The nearest
rival, the audit's whole reading of the verdicts and of the entries they name, re-reads at every slice
what the spec's design session already weighed, the cost
`rejected@agent-skills@relevance-grades-read-by-the-session` records. The lines the audit's message
owes per entry are written by the command, per `design@query@occasion-shaped-output`.
