# Agent skills design: The record search

## The record search

### One installed agent reads whole the clusters `klarch worklist` gives it, grades every entry of every kind against the work, and fills the report `klarch worklist --verify` judges `##one-record-searcher`

`agent@knowledge-architect-record-searcher` takes the work, the seeds, the decisions and goals the
dispatching skill names, and one agent directory that `klarch worklist --out` wrote, per
`design@core@worklist-writes-and-verifies-the-reports`. It reads its entries whole, leaning to
recall, and edits its report in place, or returns it as text where the harness refuses the edit. Its
description carries the dispatch, since a dispatcher reads an agent's description and never its
body: the work list written with the seeds, one agent per directory in parallel, the check of every
report, a report that fails it sent back, then the reading of the graded entries that
`design@agent-skills@search-relevance-grade` gives.

One agent grades every kind of entry, since a cluster holds a head beside the `rejected` entries it
cites and the issues and tripwires that cite it, per `design@core@cluster-rooted-at-a-subject`: an
agent per kind would read half of every cluster. The search runs in subagents so that the main
session spends its context on the entries the work bears on, per
`goal@knowledge-architect@agents-get-a-complete-workflow`. The reading holds at a retrospective
intake of many files too, at its cost: an intake of thirteen files read 64 standing entries whole,
about 167 KB. The design this decision did not keep: `rejected@agent-skills@searches-split-by-kind`.

### The record search is an agent apart from the standing-state reviewer `##search-apart-from-the-standing-state-review`

The record searcher grades by relevance on purpose, and the standing-state reviewer reads every
tripwire and every deferred trigger by rule, per
`design@agent-skills@standing-state-review-before-every-merge`. The rival, the reviewer in a second
mode, puts both standards in one text, which lets a session apply the wrong one.

### A search reads the whole record, every cluster and every residual group, whatever the work `##search-reads-the-whole-record`

The search gives no cluster a lighter reading because the work seems not to touch it. The owner
believed the whole record the safer reading, judged a choice by a cluster's
title bad practice here, and believed the bearing is found only from the search's results. The rival, clusters chosen
by the work, needs the judgement the work list removes, and rests on every pointer being a
reference, which `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` shows
it is not. A lighter pass for the Components a piece of work does not touch waits for a project
whose whole record costs more agents than one grounding should, per
`issue@agent-skills@a-lighter-pass-for-untouched-components`. Its cost is the packing of
`design@core@clusters-packed-under-a-word-bound`, re-taken with `klarch worklist --out` over the
tree.

### The searcher grades the goals its clusters reach, beside the session's own reading of every goals home `##searcher-grades-goals`

A goal reaches a cluster as context, per `design@core@goals-join-a-cluster-as-context`, and one that
no cluster reaches is in its anchor's residual group, so the searcher grades every goal, as it
grades a design entry; the session still reads every goals home itself, at the design skill's
grounding and at the intake's. The decision is the owner's: the owner saw design sessions miss a
goal twice, although the goals homes are small enough to be read whole, and ruled that both
readings stay. How a session misses a goal it is told to read is not established; the owner did not
know. The nearest rival, the session's reading alone, is the reading those sessions had. The cost is
small while a goal is small beside the entries of the clusters it joins; re-take it with `wc -c`
over the goals homes against the design homes, and goals homes that approach the size of their
design homes reopen the cost.

### The searcher grades each entry by a grade and a relation of its role, a `may-bear` grade names its doubt, every entry appears once in its report, and the session reads whole the `bears` entries and the `may-bear` entries its proposals touch `##search-relevance-grade`

The searcher grades each entry `bears`, with a relation of its role; `may-bear`, an entry where an
element the change makes might conflict with a sentence of the entry, both named in its reason; or
`unrelated`, an entry that only shares the work's subject, or that the work could lead someone to
change later, included. The relations: `constrains` and `may change` for a goal, a design entry or
an entry of a declared register; `may bring back` for a `rejected` entry; `touches`, `closes`,
`worsens`, `depends on` and `trigger met` for an issue; `fires` for a tripwire. An entry that is a
seed, or cites one, is never `unrelated`. Every entry of the agent appears once in its report, and
`klarch worklist --verify` checks it, then assembles the graded entries into `bears.md` and
`may-bear.md`, per `design@core@worklist-writes-and-verifies-the-reports`. The design skill routes
each grade to an act: the session reads `bears.md` whole and, from `may-bear.md`, the entries a
proposal touches, chosen by their reasons, and lists the others as not read, each with the
searcher's reason, where the review that reads the work against the record reads them, per
`design@agent-skills@plan-read-against-the-record`; it sends a `may change` entry to the reversal
check and a `may change` goal to the goal-setting skill, since a goal changes only there, and
proposes a `may bring back` entry again only with a new argument. A goal is in the assembly as its
grades alone, since the session reads every goals home whole, per
`design@agent-skills@searcher-grades-goals`.

The session reads whole what it acts on, never a summary of it, per
`goal@knowledge-architect@the-owner-decides`; the owner asked that it read the reported issues and
tripwires themselves rather than rely on the subagent's summary. At the design grounding the owner
ruled that a `may-bear` issue or tripwire is read whole only where a proposal touches it, as any
other `may-bear` entry, once the list of those not read reaches a reviewer that reads them. A grade
is kept only where it changes what the session does: standing-entry searchers graded a doubt
anyway, unasked, in 12 of the 41 entries they returned, each wording it differently, and a numeric
scale would route no act. The owner asked for a ruling per entry on how closely it relates, unsure
whether yes or no would suffice, and asked that every entry read be listed, which is what the check
of a report's coverage reads.

The named doubt and the reading on touch stand on one measurement, of the search that grounded the
discussion that made them, over this repository: 202 entries graded `bears` or `may-bear`, 576 KB of
`show` output read in 12 reads, of which about 30 changed a proposal, 6 of them `may-bear`, by the
session's own judgement; and 80 of the 137 entries graded `may-bear` alone gave as reason that no
bearing was established or that the entry shared the subject. Re-take it over the reports of a
search's work list, counting the `may-bear` reasons that name no element of the work. A search whose
`may-bear` entries mostly change a proposal reopens the reading on touch. The designs this decision
did not keep: `rejected@agent-skills@design-grounding-reads-every-may-bear-entry-whole` and
`rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`.

At a retrospective intake, a batch of files, the session reads whole every issue and tripwire graded
`bears` or `may-bear`, and a goal, a design entry or a `rejected` entry graded `may-bear` only where
a proposal touches its subject; it lists the others with the searcher's reason as not read, beside
every `bears` entry and every entry a proposal rests on or would change, which it reads whole. A
batch's search returns the union of many files' neighbourhoods, mostly graded `may-bear` with no
bearing established: the two searches of one intake of nine files returned 169 distinct entries,
and the session read whole about 35, those its proposals rested on; this repository's intake of 24
files did the same. Both read whole what their proposals rested on, so no outcome rested on an
unread entry. The design it does not keep:
`rejected@agent-skills@intake-reads-every-may-bear-entry-whole`.

### The record search takes as seeds the decisions the work rests on or may change, never a goal, and a goal a discussion sets is no part of its work `##seeds-name-decisions`

The design skill's grounding seeds the search with the decisions read so far that the question rests
on or may change, and the design audit of a slice or a spec with the decisions its plan documents
cite; neither seeds a goal. A goal the discussion sets through the goal-setting skill is no part of
the search's work: only its consequences on the design are. An entry that is or cites a seed is
never graded `unrelated`, per `design@agent-skills@search-relevance-grade`, so a seed that many
entries cite forces each of them to a doubt the session must read. A goal is such a seed: it reaches
every cluster that cites it as context already, per `design@core@goals-join-a-cluster-as-context`,
and the session reads every goals home whole. In the search that grounded the discussion that made
this decision, 29 of the 137 entries graded `may-bear` alone carried the mark of one goal seed, and
the work's goal widened the search to what the goal could lead to later; re-take it with a search's
`agents.txt`, counting the entries marked as citing a goal seed. The intake's seeds, the design
heads and goals its received findings name, are the findings' own citations, and stay. The design
this decision did not keep: `rejected@agent-skills@goal-seeds`.

### The record search runs where design is discussed, before the work: at a design discussion's grounding and for each later thread on a new subject, at a retrospective intake's grounding and for each fix on a new subject, and at the design audit of a slice or a spec `##record-searched-before-the-work`

The installed design skill dispatches the search at its loop step 1, with the question as the work
and seeds as `design@agent-skills@seeds-name-decisions` says, for a design question and for bounded work alike,
per `design@agent-skills@bounded-path-in-design`, and again for a thread that opens on a subject the
first work did not cover, with the thread as the work: the first search read the whole record but
graded it against the first work, so an entry that bears only on the new subject may stand graded
`unrelated`. The plan-document review of `design@agent-skills@plan-read-against-the-record` stays
the backstop. The installed planning skill dispatches it at the design audit of a milestone slice,
point 2 of `skill@knowledge-architect-planning@working-a-slice`, with the slice's spec and the
milestone document as the work and the decisions they cite as seeds, and at the design
audit of a spec where one runs, with the spec as the work. The audit lists as a gap every standing
entry the slice's code bears on: a tripwire whose firing condition, or a deferred trigger, the
planned code meets, and an issue of any kind the slice's code touches, closes, makes worse or
depends on. A cause of the milestone document's premortem whose firing condition the planned code
meets is listed with them: it enters a tripwires home only at the harvest, if the standard admits
it, so the search cannot return it, and the audit reads it in the milestone document. The slice's
grounding, point 1 of `skill@knowledge-architect-planning@working-a-slice`, reads no issues and no
tripwires, since the audit's search covers every anchor. The retrospective intake sends it at its
grounding, with the received files as the work, and again for a proposed fix on a subject the files
did not raise, with the fix as the work, before the proposal goes to the owner. Other work that
does not go through the design skill sends no search: the standing-state review reads its deferred
triggers, per `design@agent-skills@standing-state-review-before-every-merge`, which this search adds
to and never replaces. A search before every other piece of work stays parked: its one host would be
the primer, which holds only what every session needs, per `design@agent-skills@primer-limit`, the
evidence for it is one instance, and a line sending every session to search before any work is a
conformance rule. In thaum, a move of the pinned checker met the trigger of a deferred issue that
named that move, and the session read it at no step.

The occasions are those where design is discussed, so that an entry the work bears on costs a
ruling before the work rather than rework after it, per
`goal@knowledge-architect@agents-work-without-drift`. A search at grounding puts the record in view
before the first proposal, so before any ruling, where a reading at the checkpoint comes after the
owner has ruled most threads, as `rejected@agent-skills@conformance-reader-at-the-checkpoint`
records. The evidence is real use, per `design@agent-skills@behaviour-additions-need-real-use`:

- for the design entries, the owner reported them missed in part many times, and in one case the
  owner approved a proposal contradicting a head the session had not read, caught only at the
  plan-document review; and an intake of thirteen received files analysed a finding about naming a
  skill not yet created and missed the third exception of
  `design@agent-skills@plain-text-is-no-repair`, which decides that case, and the owner recalled the
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
the cluster of `design@core@cluster-rooted-at-a-subject`, under which a standing entry is judged
beside the heads it cites, so the audit's reading of the design entries is no instruction about how
an agent behaves, the kind `design@agent-skills@behaviour-additions-need-real-use` admits only on a
miss seen in real use. The misses of a search are in its judgement, not in when it runs: two pieces
of work on one day met, only at their review before the merge, five standing entries the search
before the work had not returned, `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`,
`issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways`,
`issue@core@a-plain-hash-item-name-is-read-by-nothing`, `tripwire@agent-skills@ruling-lost-in-assembly`
and the tripwire on attributions that the commit "[tripwires] Three tripwires leave, their horizons
passed unfired" removed, and each was recorded days before its
search ran, dated with `git log -S"##<slug>"` and `--diff-filter=A`. The work list and its check take
the assembly of the groups from the agents, and `tripwire@agent-skills@record-search-misses` watches
what judgement still misses. The designs this decision did not keep:
`rejected@agent-skills@conformance-reader-at-the-checkpoint` and
`rejected@agent-skills@delta-search-at-convergence`.
