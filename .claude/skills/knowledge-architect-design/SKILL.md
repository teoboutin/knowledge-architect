---
name: knowledge-architect-design
description: MUST use when a design discussion has an open solution space — requirements still being discovered, several defensible shapes, a technical choice with real trade-offs — when evaluating whether to keep or change an existing design or architecture, when a recurring bug trend suggests the design itself is the problem, when a choice about how something is built, or an owner's word that settles one, is met during another task and the decision creates a design head, contradicts a statement of one, or adds a member its argument does not cover, before starting a requested change whose design is not settled, to ground it and to find out whether it is bounded work that needs only one proposal, built on its default, or when a discussion is reverting to multiple-choice menus, minimal-solution defaults, or approval-seeking closers. Covers bounded work, threads and their states, criteria, decision authority and material findings, the loop from grounding to convergence and the premortem, the hand-off to planning, and verdicts on keeping or changing an incumbent design. Requires a frontier-tier model (Opus-class or stronger).
---

# Discussing design decisions

## Overview `##discussion-overview`

Design by open, argued exchange: both parties propose, push back with
arguments, and converge through rounds.
**The owner** is the person whose word closes the questions
`skill@knowledge-architect-design@decision-authority` gives the owner. A **decided thread** is one
closed `approved` or `argued`.

- a **thread** is one open question or proposal, the unit that carries state (below).
- an **argument** is what causes a thread's state to change.
  It has to be backed by facts, proven with examples, drawings, snippets of code, references...
- a **stall** is a move that repeats existing positions without a new
  argument or new evidence
- a **tripwire** is named future evidence that would flip a verdict; the event that would bring a
  deferral back is its trigger
- **open state** is what a project currently has outstanding:
  the entries of its issue registers and its tripwires homes.
  Differing from the decision record in that an item LEAVES it when it closes, or a tripwire
  by an exit of its lifecycle
- a **round** is one owner message and the reply to it.
- the **ledger** is the whole set of named threads with their states.
- a **delta** is the table of the threads whose state or note changed in one round
- a **bucket** is the group of threads sharing one state in a table.

Distinct from every one of these is the project's **decision record**:
the design homes of its Components and their `rejected` homes,
which the discussion grounds against and the harvest records into.
A metaphor that names a defined concept, such as a tripwire, is vocabulary
once defined, per `primer@working-prose`.

**This skill ends at convergence**, and, where the premortem runs, the premortem, its causes named,
and the owner's rulings on its acceptance criteria.

## Decision authority `##decision-authority`

The owner decides what `primer@question-authority` reserves to the
owner, a fork among it, and can overrule any decision of the
discussion. Their explicit word — an approval, a rejection, "stop, do
X" — ends the argument on that point at any time, converged or not.

- **Never assume the decision saw all its consequences.** A decision
  can be given before the implications, feasibility, and interactions
  of the chosen path were explored, especially one made to shorten a
  discussion. Verifying what the choice entails remains your job
  after the decision is given: check it against the other decided
  threads, the named criteria, and the system as you know it.
- **Material findings come back exactly once.** Material is one
  test, and it is answered by naming rather than by judging: the
  information arrived after the thread closed, AND you can name the
  part of the closure it defeats — a reason the ruling gave, a premise
  it rested on, or a criterion it claimed to meet. If you cannot name
  what it defeats, it is not material. Once is per FINDING, not
  per thread: a second finding on the same thread, genuinely different
  from the first, is a new finding and returns on its own; bringing
  the SAME finding back a second time is the stall. The common cases: a
  concrete failure the owner has not acknowledged, an infeasibility, a
  contradiction with a decided thread, a stronger alternative the
  discussion has never seen (whether an alternative is genuinely new
  is decided by the Reopening rule's record test). Present a finding
  at the top of the turn it appears in — before the delta, before the
  work: state it once with the discriminating fact, name the default
  ("unless you say otherwise, I build the decided shape"), and
  proceed unless the word comes. When the finding defeats a premise of a ruling the owner
  gave, or is a contradiction with another decided thread that no
  argument resolves, it is a contradiction, per
  `primer@owner-word-premise`: the part of the work that rests on it
  is held until the word comes, and the rest proceeds on the default.
  When the finding is that the DECIDED SHAPE CANNOT BE BUILT at all, there is nothing to proceed with: report it and stop,
  because a default would name work that does not exist. A finding
  that you cannot decide something yourself is not this case — it
  returns with a default like any
  other. And when part of the work the default would perform cannot
  be undone — data migrated or deleted, an interface others already
  consume published, a release shipped — do the reversible part and
  hold that part until the word comes. This tests the work in THIS
  TURN, not the decision's class: writing code that implements an
  irreversible plan is reversible, running the migration is not. Held
  work is named as held, with what it waits for; it is never silently
  completed and never silently dropped. And at the design audit of a
  slice or a spec, a load-bearing gap stops the work, as
  `skill@knowledge-architect-planning@working-a-slice` says, whatever
  the other cases would hold or let proceed. Presenting the finding reopens
  the thread, with the finding noted on its ledger line; it closes again
  as any thread does, on its argument or, at a fork, on the owner's
  word. Where it waits on the owner's word and no
  word has come by the next checkpoint — or the end of the current change, whichever comes
  first — the finding stays on the thread's ledger line, marked
  unacknowledged, and is carried into the plan document that
  `skill@knowledge-architect-planning` writes from the ledger, among its
  defaults awaiting the owner; on the in-change path below, into the
  commit message that carries the deliberation. For convergence, such a
  thread counts in its earlier closed state. The ledger does not outlive
  the discussion, and those do. It is not a fired tripwire. This is not
  approval-seeking: you are surfacing information
  that arrived after the closure. If the check surfaces nothing
  material, proceed — no confirmation round.
- **A premise you supplied is checked before the owner rules on it**, per
  `primer@owner-word-premise`: a question or a default put to the owner on an unchecked premise gets
  a ruling that holds only as far as that premise.
- **The weighing at a fork is theirs.** How much a risk matters, which
  trade-off wins between shapes that both survive the argument, what is
  good enough — that belongs to the owner, and
  re-arguing it after their informed ruling is not rigor. When you
  comply while still disagreeing, record the disagreement as a parked
  thread (a declared exception to closure by the owner's word — see Who moves what), and note the
  acknowledged consequence on the ledger, so that the plan document, or
  the commit message on the in-change path, carries it and the next session
  knows the decision was informed.

**Two paths, which differ in where the deliberation is kept.** Neither
bounds what is proposed, how many threads are opened, or how many rounds
the discussion takes: that depends on what grounding, investigation and
evidence bring, which nobody knows in advance.

- **The in-change path.** It is open when the decision lands in the
  change under way: its work, where that change began before the
  discussion opened, as an issue fix or a review repair does, or its
  record, whenever the discussion opened, for a decision that no work
  implements and that is not part of any spec, which
  `skill@knowledge-architect-decision-recording@when-recording-happens` records when it is made. No
  plan document is written. Its grounding applies `skill@knowledge-architect-design@what-earns-a-head`. The deliberation goes in the message of the
  commit that writes the decision's design entry, or, for a decision
  that earns none, of the commit that implements it, or, where neither
  exists, where `skill@knowledge-architect-review@what-review-leaves`
  puts a record carried by a message: every thread with its final state, the
  ground of each closure: the owner's ruling, attributed to the owner with what it
  reaches, as `skill@knowledge-architect-design@what-earns-a-head` says, the rivals that lost with
  their reasons, every premortem cause with its outcome, the owner's rulings on acceptance
  criteria, and the `may-bear` entries the grounding did not read, each with the searcher's
  reason, which the review before the merge reads. The decision is
  recorded at that landing like any other, under
  `skill@knowledge-architect-decision-recording`:
  `skill@knowledge-architect-decision-recording@reversal-check` decides the Component of a decision
  that reverses a recorded one, and `skill@knowledge-architect-design@what-earns-a-head`
  decides whether any other decision earns a design head at all. Apply both before proposing to the
  owner where a decision lands. Where the premortem runs, nothing of the decision is implemented
  or committed before it has run and the owner has ruled on its acceptance criteria, so the record
  that carries the deliberation carries those rulings.
- **The full path.** Otherwise, and whenever a discussion converges on
  work that no longer lands in the change under way (it needs a branch
  of its own, several pull requests, or nobody has started it): the
  hand-off to planning of loop step 8. A decision that is cheap to
  reverse and whose work comes later takes it too. A design session
  opened at a design audit under `skill@knowledge-architect-planning@working-a-slice`
  takes the full path, and that skill says which plan documents its
  converged design is written into.
  A choice among shapes stated in full, which
  `skill@knowledge-architect-planning@working-a-slice` puts to the owner in
  one message, ends at the owner's ruling, with no premortem.

On the in-change path, the premortem runs when reversal touches any of the four; the four never
decide which path is taken.
To decide, state what reversal touches, in nouns. The four: stored
data that would have to be migrated; an interface other code or other
people already consume; behavior users have adapted to, in the sense
Keep-or-change uses — a document that argues it, or observed use —
which is not the same as any user-visible string; or a decided thread
that would have to be REOPENED if this one changed, as opposed to one
that merely reads it.

## Bounded work `##bounded-work`

Whether a request is a design question or bounded work, a clear requirement whose main risk is
over-building, is known only after grounding. So a requested change whose design is not settled
starts here, at loop step 1, the search for standing entries included. After the grounding, the
work is **bounded** when all three hold:

- it reverses no recorded decision, as `skill@knowledge-architect-decision-recording@reversal-check` asks;
- every decision it makes fails the entry tests of `skill@knowledge-architect-design@what-earns-a-head`, so none earns a design head;
- no second defensible shape survives the nearest-rival test of loop step 3. State the strongest
  open reading of the problem beside the bounded one: the design question the bounded framing would
  suppress.

Then send **one message**: what the grounding found, the proposal, its nearest rival and the fact
that defeats it, the consequences the owner must know, and the default, which you then build: the
work proceeds on it unless the owner answers otherwise, a part that cannot be undone waiting for the
owner's word, per `primer@question-authority`.
A reply that brings an argument, a new shape or a doubt returns the work to the loop, on the
in-change path or the full path. The commit that implements bounded work carries the proposal and,
if one came, the owner's answer, attributed to the owner, as `skill@knowledge-architect-design@what-earns-a-head` says,
and the `may-bear` entries the grounding did not read, each with the searcher's reason, which the
review before the merge reads.

Misclassifying costs in both directions: a bounded problem does not need a discussion, and an open
one needs argument. Territory that looks technical is often not bounded (error handling can turn out
to be user-communication design).

## What earns a head `##what-earns-a-head`

Most implementation choices earn no head. A unit of work produces dozens of them, and a design home that
records dozens per unit of work stops being readable and stops being ranked.

**A decision earns an entry in a design home only if at least one of these holds:**

1. reversing it would change **an interface others consume**: a type or a signature that crosses
   the boundary of a separately built unit (a crate, a package, a library, a module others import),
   a file format, a document grammar, a command line;
2. **the same reason must be respected at more than one site, or at none.** A reason is an argument
   against a rival someone could plausibly propose; a name, a path or a value repeated for
   consistency is not one, since nothing argues for changing it and a stale copy is found by a
   search. At more than one site, the reason needs a home each site can cite, and a comment cannot
   be cited. At none: a decision about an absence ("we do not do X"), or a policy that no code and
   no text of the project states. For a decision about agent-facing text, a skill, an agent or a
   `CLAUDE.md`, each text that states the instruction is one site, a design head of the project
   that restates it included, other than the decision's own: a decision stated by one text
   has one site, and its reason lives in that text, beside the instruction; or
3. **its argument turns on the behaviour of something outside the project**: an external
   specification the project implements (a standard, a protocol, a rule set), or an external tool's
   behaviour, read in its documentation or measured; or
4. **the owner confirms that it records the owner's own intent**: a ruling the owner gave that an
   agent could judge superfluous and reverse as a small fix, or an argument the owner made and wants
   kept so as not to restate it. The agent does not judge this. When a decision would earn a head
   by this test alone, ask the owner, one question per decision, each under a label, `Q<n>`,
   numbered as `primer@labelled-items` says, several in one message, each quoting the owner's words
   the decision rests on: do they record the owner's intent, or were they an answer to the agent's
   proposal, or a hedged statement ("I think", "maybe"), which is a position to argue under
   `skill@knowledge-architect-design`? The head attributes the decision to the owner, with the
   owner's answer, as `primer@design-heads` says of attribution.
   With no answer, the decision earns no head by this test.

A choice that turns on what the specification means, or on how the tool behaves, is expensive to
get wrong and expensive to derive again, and it is visible: the argument quotes the specification
or the documentation, or names the measurement.

Otherwise it belongs in an **inline comment at the code it explains, plus the commit message**.
The comment is read by every session that touches the code, which a head is only where the session
finds it on the code's subject, and the commit
carries the argument. A reason that fits in one comment at one site, about that site's own code,
fails test 2.

**A head carries the intent it serves, its shape and the standing argument, not implementation.**
How the project is arranged to reach its goals belongs here, each head citing the goal it derives a
constraint from; what the project is for is its goals home's. How a particular function does its
work belongs in a comment at that function. The test: **if changing a piece of code would force a
change to the design home, it is design; if the design home would be unaffected, it is a comment.**

**The backstop. A decision that creates a head, contradicts a statement of one, its argument
included, or adds a member the head's argument does not cover, goes to the design skill before its
text is written**, unless it was argued there, under `skill@knowledge-architect-design`. This is the case of
a decision met during another task and settled there, by the owner's word or by the session's own
choice. The design skill's in-change path keeps the deliberation in the commit message, so the task
needs no plan document and no new session. An addition that answers another question than the
head's takes a head of its own, by the test of one decision per head of
`skill@knowledge-architect-decision-recording@head-shape`. A member the head's
argument covers is recorded directly, routed as the next paragraph says; where the owner approved
it, the approval is recorded in the commit, attributed to the owner, not in the head. A change that relocates or rewords
recorded decisions, a split of a head included, and adds or removes none, is not a decision: it
needs no design skill, and the routing and fidelity-of-relocation review axes judge that it adds or
removes none.

**A new member of a head is routed by the head's ground.** A member is what a head's rule admits
and its body does not yet name. Where the part of the head that admits it stands on its argument,
and the argument covers the member, it is recorded directly. Where that part's ground is the
owner's words and they state the rule, the member is within the ruling and is recorded directly,
the ground attributed to the owner as before. Where the owner's words name the members, it goes to the owner as one
proposal: one message stating the rule, the member and a default, and nothing of it is written
before the owner's word; it is no change of the decision, so it owes no reversal. A member the rule
does not admit, or that the argument does not cover, is a change of the decision, and goes to the
design skill, as the backstop says.

### What a head stands on

**The standing argument** is every premise whose failure would reopen the decision: the goal it
derives a constraint from and a decision of another Component it depends on, as references; the
measurement it rests on, with the command that takes it again, one a reader can run on the corpus
it names: a script that no tree holds is replaced by such a command, or kept in the message of the
commit that took it, which the head names by its subject; the fact that defeated its nearest
rival. A constraint from a goal binds outright, where one from a decision is a presumption, and the
reference is what tells a reader which. **The test: if this premise turned false, would the
decision have to be argued again? If yes, it is in the head.** If no, it is deliberation.

**A head stands on its argument, and cites the owner only for what came from the owner.** Most
decisions are proposed, argued over their costs and rivals, and approved by the owner. The head
records such a decision on that argument, with its costs and rivals in its body or in the
`rejected` entries it cites. An approval is not a ground, whatever its words ("approved", "agreed", "all defaults
approved", "accepted the cost"). It stays in the deliberation: the plan document or the commit
message. The owner's words are a ground, attributed to the owner, only where the decision came from
the owner: the owner proposed it, the owner chose where the argued rivals did not settle it, or it
rests on a premise only the owner can state, such as an intent, a plan or a weighing the owner
made. Test 4 is such a case. In a head that holds a part of each, the part that came from the owner says so, and
the rest stands on its argument.

**A durable record attributes a decision to the owner; it does not quote the owner.** A head, a
commit message, a tripwire, a `rejected` entry and an issue entry state what the owner decided and how far the
decision reaches, as "the owner chose X over Y" or "on the owner's word", never the owner's words
verbatim. The verbatim words stay where the deliberation is kept: a plan document's items, and a
committed file that leaves once its outcomes are carried out. An attribution keeps the scope and the
hedges of the owner's words, and is checked, like a quotation, against the question it answered.

**What the ground means to a later session.** A head that stands on its argument is reversed by a
better argument, which the owner approves where the reversal is a fork, per
`primer@question-authority`. A head whose ground is the owner's words is
reversed by the owner; a defeated premise of those words goes to the owner, as
`primer@owner-word-premise` says. An argument against it goes to the
owner as a question about their intent.

## Language `##discussion-language`

The discussion, and every text it produces, is working prose: it follows `primer@working-prose`,
read whole. Two rules are the discussion's own.

A qualitative claim may open a thread, never close one. As a stated goal or observation ("the app
should feel responsive") it is legitimate input, and the thread stays open until the claim is
converted into its checkable form. This governs arguments and never rulings: the owner's word
closes a thread whatever their reasons, and "it reads better to me" is a decision they are
entitled to make, not a claim for you to convert.

A one-sentence compression (see `skill@knowledge-architect-design@incumbent-evaluation`) names a mechanism in plain words. It lives as a
titled summary line next to a plain-prose restatement, never as the body prose of a document, a
commit message, or the discussion itself.

## Threads and states `##threads-and-states`

Track the discussion as named threads. This structure governs how the
exchange is conducted and displayed — never what may be proposed. New
threads, new arguments, and proposals that supersede settled points
enter freely at any time.

**Naming.** A thread gets a short content-named slug in the grammar of an
entry id, `[a-z0-9]+(-[a-z0-9]+)+`, since a decided thread whose
decision earns a design entry gives that entry its slug, or a slug and a
stated pair, per `skill@knowledge-architect-decision-recording@head-shape`. Name the thread by the decision it
would record if decided, not by the change it proposes: #retention-window,
not #shorten-retention. A slug is never changed during the discussion, since
every earlier delta carries it. Before using a slug, check that no entry of
the Component that will own the decision already holds it: a thread that
loses may become a `rejected` entry under its own name.
`cargo klarch id <slug>...` lists, for every candidate in one call, each
entity the project defines under it, and only a design or a `rejected`
entry of that Component refuses the slug. In prose, write it plain with a
`#` before it, as in #retention-window, never in backticks: a backticked
span may be read by the checker as a reference. In the plan document the
thread becomes an item, which the plan document cites by its checked form,
from inside its own plan only, per `skill@knowledge-architect-planning`;
the plain form stays there only in the harvest row's pair of a thread and
its entry slug, which `skill@knowledge-architect-decision-recording@head-shape` states. When the natural name is a
common prose word, choose a two-word slug so references stay unambiguous.

**Granularity.** Every proposal either party made is a thread. A
criterion is NOT a thread and never enters the ledger: threads are
judged against criteria, and a criterion that was itself a thread
would need criteria to judge it. A round that exchanged
proposals and produced no delta is a tracking failure, not a quiet
round.

**States.** Open: `new`, `in-discussion`, `presumed-settled`. Closed:
`approved`, `argued` (closed on its argument, one shape standing),
`ruled-out`, `parked` (deferred, with a named cause and a
re-entry point), `superseded` (absorbed by a named other thread),
`withdrawn` (retracted by its own proposer). The set is
closed on purpose: convergence is computed from it.

**Summaries and checkpoints.** When a round changed any state or any
note, end the turn with a delta: thread / state / one-line position /
note, per changed thread. Criteria go in a separate table above the
proposals — criterion / kind / satisfied by — because listed among
them a criterion gets read as a proposal and closed like one. A
criterion enters the delta when its kind or its satisfaction line
changed; it has no state to change.

The **note** column carries what a state cannot say: a relation to
another thread (absorbs #x, conflicts #y, serves #criterion), a
material finding pending on a closed thread, an assertion the
discussion has retracted (conflict with #y withdrawn: draw order
traced), and the conditions a closure came with. A retracted relation
is recorded as retracted, never deleted. Notes carry no state and
convergence ignores them; write one where it carries information, not
on every row.

A **checkpoint** is a display of the full table,
and it is ASSEMBLED, not recalled: sweep every prior delta and
collect every THREAD slug ever minted; each appears exactly once, in
its current bucket, closed buckets first. Criteria are assembled the
same way into their own table, where they have no bucket because they
carry no state: every criterion ever named, with its kind and its
current satisfaction line. A thread slug you cannot place is a
tracking failure to repair in that turn, not a row to drop. The checkpoint happens at two
moments: when proposing convergence, and before
the hand-off to planning on the full path. On the in-change path, a
ledger that holds a single thread skips it. The full
table is presented for contest ("contest any of these"), one batch
confirmation over the whole ledger — never a per-item ratification
questionnaire.

**Who moves what.** You may move threads freely between the open states of a thread.
The owner's explicit word moves a thread into a closed state — a word
given during the discussion ("drop the polling idea") counts;
no particular phrasing is required. Their word closes what it names:
a word that points at the ledger ("build it as the table says")
closes the table's contents. A word that requires interpretation — a
blanket positive ("sounds good", "all fine"), a sentence that may or
may not cover a thread —
closes nothing: mark the threads your reading would close as
`presumed-settled`, state the reading, and let their next word — or
the batch confirmation at the next checkpoint — promote or correct
it. A displayed checkpoint table changes this: a positive word given
against the table IS the batch confirmation and closes what the table
holds, because the table is what makes an otherwise vague word
specific. Do not solicit closures: a yes/no question inviting assent to one
option ("so we go with X?") is an approval question wherever it
appears; a legitimate closing question presents the argued fork and
asks for a ruling between positions. The checkpoint's batch
confirmation is not this pattern. When the ledger holds a single live
thread and a rival still stands, give even that confirmation the
argued-fork form.

When no rival stands — the alternatives are recorded `ruled-out`, and
raising one again would be a stall and inventing one would be padding
— there is no fork to present, and the closing move is not a question
at all: the thread closes `argued`, below. **A thread whose question
`primer@question-authority` gives the owner, a fork among them, closes on the
owner's word**, a fork put as the argued fork that section describes.

Closure by you, not the owner, exists in exactly three declared cases:

- `argued` — a thread with one shape standing, each rival out by a
  named fact or binding constraint. You close it on its argument, and a
  rival defeated so you close `ruled-out`, naming the fact. The turn
  lists both under "Decided", per `primer@decided-section`. A rival nothing defeats makes a fork, and
  the thread stays open.

- `withdrawn` — a proposal of your own, when your own argument or
  evidence defeats it and the owner has not adopted or built on it.
  The state is proposer-symmetric: a proposal the owner retracts
  themselves is recorded `withdrawn` too. `ruled-out` marks a
  proposal that lost an argument and carries why it lost into the
  record; `withdrawn` marks one its own proposer gave up on. When the
  withdrawal follows defeat by argument or evidence, record the
  defeating reason — the Reopening record test reads it.
- `parked` — your own overruled dissent after the owner's explicit
  ruling on the point (`skill@knowledge-architect-design@decision-authority`), and the observation
  recorded during a bug hunt for a later discussion (`skill@knowledge-architect-design@incumbent-evaluation`).
  Every parked thread names the evidence that would reopen it, as a
  cause, AND its re-entry point: the event or checkpoint at which it is
  re-proposed. A parked thread missing either one is open. A cause with
  no re-entry point names evidence nobody is scheduled to look for. The
  cause is named on the ledger, and judged and written as
  `skill@knowledge-architect-decision-recording@ruled-tripwires` says;
  the thread stays parked whatever the standard rules on its cause.
  `superseded` is not a fourth case:
  threads absorbed by a proposal close when that proposal closes, on the
  owner's word approving it by name or `argued`, and until then the
  absorbed threads keep their state. A displayed checkpoint showing the absorbing thread and the
  threads it absorbs satisfies that by-name requirement, for the reason
  the batch confirmation works there at all: the table is what makes an
  otherwise vague word specific. Away from a displayed table, the word
  has to name the proposal.

**Criteria.** A criterion is what proposals are judged against, so it
takes none of the states above: those are fates for a proposal — won,
lost, deferred, absorbed, retracted — and a criterion has
none of them. It is instead one of two kinds. A **binding** criterion
rules out any proposal that fails it. A **weighed** criterion makes
failing it a cost the owner rules on. Criteria may be stated in the
discussion or DERIVED from what the project has recorded: its goals, its
decisions, and the directives and contracts of its `CLAUDE.md` and
`README.md` files. A criterion derived
from a goal is binding: the goals are the owner's intent. A criterion
derived from a recorded decision is binding as a stated presumption until
the owner says otherwise, because a decision binds only as a presumption
that a better argument may rebut, per `primer@goals-bind`.

Each criterion carries a satisfaction line, and this is what the owner
reads at convergence: **met**, naming the decided threads that meet
it; **unmet-and-accepted**, which needs the owner's word naming that
criterion rather than a blanket confirmation; or **unmet**.
`unmet-and-accepted` is available to a WEIGHED criterion only. A
binding criterion that is unmet blocks convergence, and no word
waives it in place: the moves are to change the proposal, or for the
owner to demote the criterion to weighed — a change to the criterion,
recorded as one. A criterion derived from a goal is demoted only by
changing the goal, under `skill@knowledge-architect-goal-setting`. Were acceptance available to both
kinds, binding and weighed would differ only in how much ceremony the acceptance takes.

**`presumed-settled`** is a labeled claim about the owner: they are
continuing the discussion in a way that assumes this thread's
resolution. It is deliberately an open thread state — a presumption never
counts toward convergence. Each time the owner's arguments again
depend on the presumption, its evidence refreshes. A presumption the
owner's arguments have not depended on for two rounds is re-surfaced —
in the next delta, or as a standalone note when no recent round
produced one — rather than silently carried.

**Reopening — closed is not frozen.** This rule is the single owner
of the novelty test. The owner reopens a closed thread by their word
alone. You reopen one with a new argument or new evidence named, as a
material finding — the material-findings protocol (`skill@knowledge-architect-design@decision-authority`)
owns the mechanics.
The record arbitrates novelty: an alternative counts as
argued-and-lost only when a recorded reason covers it: a `ruled-out`
thread, or a `withdrawn` thread recorded with a defeating reason, of
this discussion's ledger or of a deliberation kept where
`skill@knowledge-architect-decision-recording@three-homes` says, or a
`rejected` entry. To
decline surfacing a finding, cite that record; if no recorded reason
covers the discriminating fact, the finding is new. A reworded
proposal that defeats or evades the recorded reason is new by
definition, and new evidence qualifies regardless of how the argument
previously went. Re-arguing a closed thread with nothing new is a stall. Late
reframing is welcome — a new proposal may supersede several settled
threads at once when it can be argued better than what it absorbs.

**Arguments are not tracked.** Only threads carry state. One argument bears on
several threads at once and displaces other arguments, so giving
arguments state would need a many-to-many relation that a delta row
cannot show — and the note column already carries the relations that
have to be visible. An argument that outgrows its thread is promoted
to a thread of its own, explicitly.

**No selection dialogs.** Within a design discussion, harness choice
dialogs (button pickers, option prompts) are not a discussion move:
they truncate the option space to a label, hide the reasoning that
belongs next to each option, and collect a click where the exchange
needs an argument. They stay legitimate for out-of-band practical
choices whose option space genuinely is closed and consequence-free.

## The loop `##discussion-loop`

1. **Ground first**: read `primer@verify-before-relying` again, whole,
   then the project's record, before proposing. The
   goals homes of the project's root and of every Component the question
   touches; their design homes; their `rejected` homes; their
   `README.md` files, which state their contracts. A search of the whole
   record runs beside that reading, with the question as the work and,
   as seeds, the decisions read so far that the question rests on or may
   change, never a goal: a goal reaches every cluster that cites it as
   context, and a goal seed forces every entry citing it to a `may-bear`
   grade at least.
   A goal the discussion sets through `skill@knowledge-architect-goal-setting`
   is no part of the search's work; only its consequences on the design
   are. The search is dispatched as the description of
   `agent@knowledge-architect-record-searcher` says. Then read whole
   `bears.md`, every entry the search grades `bears` with every reference
   to it, a goal there by its grades alone since its home is read above,
   and, from `may-bear.md`, the entries graded `may-bear` that a
   proposal touches, chosen from its table of contents; run
   `cargo klarch show <kind>@<anchor>@<id>` on each other entry the
   question bears on, and read it whole: never act on an entry from the
   reason the search gave. The first round states which of these were
   read and run, which entries the search graded `bears` or `may-bear`,
   which of them the question bears on, for each `may-bear` entry read
   whether it bears, and the `may-bear` entries not read, each with the
   searcher's reason. That list reaches the review that reads the work
   against the record, in the record each path keeps: the plan document's
   "What is already decided" on the full path, and the commit message on
   the in-change path and for bounded work. An entry graded `may change` goes to
   `skill@knowledge-architect-decision-recording@reversal-check`, and to the
   backstop of `skill@knowledge-architect-design@what-earns-a-head`, before a proposal that would change it,
   and a goal graded so to `skill@knowledge-architect-goal-setting`; one graded `may bring back` is
   proposed again only with a new argument, by the Reopening rule. When a
   thread opens on a subject the question did not cover, run the search
   again with the thread as the work: the first search graded the record
   against the question alone. Where the record is silent, read the code and its history; where it is empty, state that the record starts with this
   discussion. A record that
   carries a verdict but no decision — a review, an audit, an
   automated report, a prescribed change nobody ruled on — grounds
   the discussion without settling it: open a thread against its
   prescription and argue it like any proposal. A recorded decision is
   different; it carries its argument, and the owner's ruling where its
   ground is the owner's, and the Reopening rule governs it.

   Grounding also runs outward, under a narrow test. Propose a
   prior-art survey only when other projects have faced THE SAME
   question and shipped inspectable mechanisms answering it — pointer
   ownership in an immediate-mode UI, retry semantics, cache
   invalidation. Then name the class, the candidate sources, and the
   cost, and run it only on the owner's word. This is available at any
   point in the discussion, not only at the start, and it is not
   stall-breaking evidence: it needs no pre-commitment about which
   position it would flip, because a survey commonly replaces both
   positions rather than choosing between them.

   An adjacent literature is not prior art. When the answer turns on
   facts private to this project — its own data, its own contracts,
   its own two conflicting definitions — no outside mechanism answers
   the question, and surveying the general area spends a round on
   material that cannot discriminate. Nearly every problem has a
   general framing; having one is not the trigger. The test is
   concrete: name the problem class and at least one place you expect
   the answer to exist. Not knowing WHICH mechanism others shipped is
   the reason to survey; being unable to say the class has been
   answered outside this project is the reason not to.

2. **Name the criteria**: state early what the decision must
   achieve — the constraints and goals proposals will be judged
   against. Criteria are argued and refined like anything else.
3. **Round**: bring argued proposals, including your own new ideas.
   The proposal space is wider than the literal request — the request
   is where the discussion starts, not a limit on what may be
   proposed. Welcome moves include (not exhaustively): a novel
   alternative that supersedes the shapes under discussion; a
   solution that also covers an adjacent problem when bundling them
   is implementation-cheaper; an adjacent concern raised now because
   deciding it would shape this implementation.

   When presenting a proposal, do not assume the owner is an expert.
   You have to make sure the owner understands the implications of every proposal.
   In order to achieve that, add drawings, snippets of pseudocode,
   and any other descriptive content that would help the owner understand better.

   When genuinely distinct defensible shapes exist,
   present them as separate threads. To claim one shape dominates,
   name its nearest rival and the discriminating fact that defeats
   that rival — "no rival worth naming" is itself a claim, tested
   like any other; never pad with alternatives to reach a count.
   Factual questions are grounding: answer from the source, a
   measurement or the record what they answer, per
   `primer@question-authority`, and ask the owner, freely and early,
   the facts only the owner holds (their users, their plans, the scale
   they expect). Preference questions need
   an argued default attached, and earn their place only when the
   answer would change your proposal.

4. **Argue both ways**: the exchange is symmetric until a decision,
   and this step owns the rule: every proposal, whichever party made
   it, owes the same slots — why it should work, what it costs, what
   it enables or rules out later — and gets tested with the same
   rigor. The owner's proposal is a claim to test, not a directive to
   follow — only an explicit decision converts their statement into a
   directive (see `skill@knowledge-architect-design@decision-authority`). Fill its missing slots, test
   it against the criteria, and when a shape exists that meets a
   named criterion theirs fails, propose it against theirs even when
   theirs would work; filling and testing their proposal is your job,
   not a courtesy. Argued agreement is legitimate. Objecting to a
   proposal you have no argument against damages the discussion the
   same way agreeing with one you have not tested does. A factual claim
   doing closing work carries its provenance — measured (with the
   command that re-takes it), read in a named source, or assumed, as
   `primer@verify-before-relying` asks of every claim — and a thread may close over assumed
   claims only if each assumption is named as a cause on that
   closure, or, where only the built code can check it, as an acceptance
   criterion, as step 7 splits them. In the other direction, reverse when the opposing
   argument explains something your position cannot, or survives an
   objection your position fails — not merely because the owner
   insists. "Both have merits" with no synthesis proposal is a
   stall; a synthesis argued as its own proposal (what it takes from
   each side and why) is a legitimate move.
5. **Build discriminating evidence when stalled**: when positions
   rest on intuitions, a further round restates them and adds nothing
   either party could check; build evidence rather than trade
   intuitions, per `primer@verify-before-relying`. Build the
   discriminating artifact the project affords — a failing test, a
   throwaway prototype, a benchmark, a mockup. Evidence that the
   project's goals or design heads rule out is not built, whatever the
   stall; the fork goes to the owner as a tie. Before building, both
   parties state which outcome would flip their position; an artifact
   built without those pre-commitments cannot change either position,
   so it settles nothing. If the owner declines to pre-commit, build
   on their word and present the result as data, not as a verdict. A
   tie is a legitimate verdict: when the options differ by less than
   the cost of discriminating between them, say so and let the owner
   pick. Evidence gathered by several parallel investigations is
   reported as ONE round when the pieces bear on the same threads: the
   unit of the discussion is the round, not the arrival of a result.
   When one investigation returns something that
   invalidates what the others are still testing — an infeasibility, a
   defeated premise — report it when it arrives, before the other arms
   finish, naming what it invalidates and whether the rest are still
   worth completing. The threads here are open, so this is ordinary
   argument and NOT the closed-thread protocol, whose default and
   closure rules do not apply to a thread nobody has closed. This does not
   license shorter turns — the consolidated report carries the full
   depth of every piece.
6. **Converge**: convergence HOLDS when every PROPOSAL thread is
   closed, every criterion carries a satisfaction line, no binding
   criterion is unmet, and the discussion is quiescent — the round in
   which the last thread closed introduced no new thread, no
   reopening, no new argument. It
   is proposed before it holds, so a `presumed-settled` thread may be
   live when convergence is PROPOSED — putting that bucket in front of
   the owner is what the proposal is for — and never once it holds.
   Silence you produced is not quiescence: before proposing
   convergence, name the last new argument or probe you brought and
   what it produced.

   Convergence is PROPOSED, never requested: show the checkpoint
   table, with the `presumed-settled` bucket in view — the batch word
   promotes that bucket, and convergence holds when it
   lands in a round that introduces no new thread, reopening, or
   argument. Otherwise the new material opens as threads and
   convergence is re-proposed.

   When convergence can be proposed, prepare a premortem, where
   `skill@knowledge-architect-design@decision-authority` says it runs. See below.

7. **Premortem before convergence**: assume the design shipped and failed.
   Name the most plausible causes, each naming the decided thread it stresses.
   A premortem whose causes stress no decided thread is not finished.

   During this step, it is possible for a new MATERIAL finding to surface against
   a decided thread. In that case, convergence cannot hold, the invalidated thread reopens,
   and the discussion continues.

   If the design holds against the premortem, what survives is named as causes,
   except a cause that only the built code can check: that is an acceptance
   criterion, which `skill@knowledge-architect-planning` writes into the plan document,
   or which, on the in-change path, the change applies as a test.
   **A cause is named, not ruled.** It is judged by the standard below at the end of the work
   that implements the decision, on its wording then, as
   `skill@knowledge-architect-decision-recording@ruled-tripwires` says, and
   no cause is put to the owner.

   Ask the owner, for each acceptance criterion, whether it is applied,
   naming the decision its firing reopens. Put each under a label,
   `AC<n>`, numbered as `primer@labelled-items` says, so that the owner
   rules on each by its label. The criterion's item or, on the in-change
   path, the comment at its test, keeps that label in its text, so the
   ruling in the transcript is found from either. Some may be out of
   scope of the project, and the owner is the only judge of that. A
   criterion meets conditions 3 and 4 of the standard below: it names a
   built result under which it fails, and a bound in it carries its
   basis, per `skill@knowledge-architect-planning@acceptance-criteria`. The owner's word rules on whether a criterion is applied, not
   on the wording of its observable, which you may reword when it is
   written, on a review finding or at a later change, listing each change
   in the turn's "Decided" section, per `primer@decided-section`.

   A tripwire watches one premise of a recorded decision: a failure that would show the decision
   wrong and that nothing else in the project reports. Most causes a premortem names are not that,
   and are not written.

   **A cause becomes a tripwire only when all four conditions hold.**

   1. **The failure is silent.** Every procedure that meets it counts it as success: the tests and
      gates pass, no work is stopped by it, no review reports it as a defect of the decision, and no
      request names it. A failure that fails a test or a check, stops the work that meets it, or
      arrives as a request for what the decision refused reaches the decision without a tripwire.
      Where a script could compute the firing, write that check instead. A check of the work being
      built is that work's acceptance criterion; one that can be written only after another event,
      such as the end of a migration, is a `deferred` issue whose trigger is that event.
   2. **The evidence lands where tripwires are read.** It is in the tree, the history, an audit
      report, or a retrospective file the intake receives. Evidence that exists only in a session's
      transcript, or only in what the owner sees while driving a session, reaches no occasion that
      reads the tripwires.
   3. **The evidence is what the decision says must not happen.** A clause that counts what the
      decision instructs or endorses fires on the decision working.
   4. **A number has a basis.** A bound in the firing clause comes from a measurement or an argument
      the decision's record carries, its head, or its plan document before the head is written, and
      exceeds the noise of the instrument that takes it. Otherwise the clause names the event,
      specific enough that two readers agree whether it occurred: "it gets slow" names none.

   **The owner's word admits no tripwire alone.** A cause the owner proposes, or one derived from the
   owner's words, is judged by these conditions like any other, and a tripwire that fails them is not
   kept, whoever proposed it. A ruling of the owner that a tripwire restates, such as a bound or an
   exception, lives in a design head, which the tripwire cites.

   **A slip is no firing.** A clause on how sessions follow an instruction never fires on one instance
   that a review, a check or a retrospective reported under its own procedure: one slip is the review
   working. It fires on the same failure recurring in separate pieces of work, or on one instance that
   reached the main branch with no procedure reporting it. Where a clause fires on both a slip and
   something more, strike the slip.

   **A doubt keeps the cause, briefly.** A cause that no condition clearly excludes is recorded with a
   short horizon. A wrong keep costs reading until its horizon; a wrong drop loses the evidence for
   good, unless the owner would see the failure anyway, by using the project or by carrying work with
   its agents: then a wrong drop costs little, and a cause in doubt may be dropped. That weighs a
   doubt only; it never drops a cause the four conditions admit.

   The entry is a level-two heading in the tripwires home of the Component that owns the decision, its
   slug last:

   ```markdown
   ## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`

   **Fires when:** the event, or a count with its basis.
   **Response:** reopen the decision by its reference, or open an issue of a named kind.
   **Re-entry:** an occasion that holds the evidence.
   **Watches until:** the occasions after which the cause, unfired, no longer will.
   ```

   **Watches until** names the first occasions that exercise the decision, so that a command over the
   history counts them from the commit that wrote the line: commits on the main branch that change a
   named path or the guarded head's text, release tags, audit reports of an axis, plan documents
   deleted at their landing, or one event the tree shows, such as the first issue a consumer of an
   interface opens; never an occasion that only a session's transcript holds. Its length is the
   writer's judgement, short for a cause kept on a doubt, and no count is kept in the entry.
   `standing` is for a guarantee whose failure stays silent and permanent, with the reason in the
   entry.

8. **On the full path, hand off to `skill@knowledge-architect-planning`, in this
   session.** On the in-change path there is no hand-off: the work
   continues in the change, and the deltas are the draft of the commit
   message that carries the deliberation.
   The ledger, the criteria table, the arguments and the facts they
   depended on, the designs as they were presented to the owner, the
   premortem, its causes and the owner's rulings on its acceptance
   criteria live in this
   conversation and in the harness's transcript of it. The planning
   skill assembles them from the transcript into its plan documents,
   before the session ends, for the reason
   `skill@knowledge-architect-planning@when-planning-starts` gives, and
   owns which ones and their shape.
   Each round's delta is the draft that assembly reads.

The plan document is the full account of the
discussion, assembled from its transcript: every thread with its proposer
and final state, the arguments on each side as items, the owner's
rulings verbatim, the losing alternatives with their reasons, and the
causes the premortem named, each with its outcome once the work has judged it.
The decision record is the durable harvest of that account, written when
the work that implements each decision lands, under
`skill@knowledge-architect-decision-recording`: a decided thread becomes a
design entry when it earns one, and a shape that lost
to an argument earns a `rejected` entry only when that skill's
tests say so. Open state goes to the issue register and the tripwires
homes, under `skill@knowledge-architect-issue-tracking`. A decision that no
work implements and that is not part of any spec is recorded when it is made, per
`skill@knowledge-architect-decision-recording@when-recording-happens`.

## Keep-or-change (evaluating an incumbent design) `##incumbent-evaluation`

A recurring bug trend, a refactor proposal, a questioned pillar: run
the same exchange, plus:

- **Fresh discussion, trend framing.** Do not advocate a design
  direction during the bug hunt that raised it: while an incident is
  open, a redesign proposal is judged as scope creep, and the trend
  evidence is lost when the incident closes. At the trigger moment,
  record the instance and the suspicion
  as an observation, not a verdict: an issue entry of kind `design`,
  under `skill@knowledge-architect-issue-tracking`. Finish the fix; then argue the
  direction in a discussion whose entry point is the recorded trend,
  not the incident — a new session when the incident still dominates
  the context. Propose that discussion by name before the current
  work lands; it is the parked observation's re-entry point. The
  owner may still decide immediately — their authority is not
  suspended — but your recommendation waits for the fresh frame.
- **Sequence direction before cost, inside one decision.** Argue the
  direction on its merits first, so migration cost does not anchor
  the exchange: brought in early, migration cost is the only quantity
  on the table, and the direction gets decided by it before its merits
  are argued.
  But cost enters before the verdict is recorded: a direction chosen
  in ignorance of an unaffordable migration is not a decision — the
  cost will be discovered later and force the decision to be remade.
- **Verdicts must be falsifiable.** Every verdict on an incumbent design
  names the specific evidence that would flip it, as causes judged as
  step 7 says.
  "Each bug has a fix" is the unfalsifiable non-answer.
- **Compress before concluding.** The recurring cost is not
  diagnosed until one sentence names the mechanism producing it. The
  owner brings instances and a suspicion; writing that one-sentence
  mechanism statement is your job.
- **The losing side relocates, never deletes.** Record the losing
  alternatives in the plan document, or the commit message on the in-change
  path; which of them earn an entry is
  `skill@knowledge-architect-decision-recording@losing-alternatives`. Design
  the winner to absorb what was right in them.
- **Incumbent behavior counts as a constraint only on evidence.** A
  behavior is established by a document that argues it — a commit
  message that argues it counts, and in most projects it is the only
  home such an argument has, and so does a comment at the code that
  states its reason — OR by observed use: consumers,
  telemetry, bug reports about its absence. Behavior with neither is
  an implementation coincidence — question it, and raise it with the
  owner before deciding anything on its behalf. Do not treat
  "undocumented" as "disposable" in a project that documents nothing.

Verdict record — the account step 8 hands off on the full path, or
the commit message carries on the in-change path, specialized for a
verdict on an incumbent. Fill every slot. A decision that evaluates no
incumbent design hands off under step 8 alone: it has no recurring
failure to diagnose, so it has no Mechanism slot to fill, and this form
is not the shape for it.

- Verdict: keep | change (one line)
- Mechanism: the one-sentence statement of what produces the
  recurring failures
- Causes: the specific evidence that would flip this verdict, each with its outcome
- Losing alternatives: what they were, and where the winner absorbs them
- Recorded at: the plan document, or, for a verdict no work implements,
  such as keep, where the in-change path puts its deliberation; the design
  entry of the owning Component, when the verdict earns one, at the
  harvest or, on the in-change path, when the verdict is made
