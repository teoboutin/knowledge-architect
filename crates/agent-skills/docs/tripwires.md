# Tripwires — agent skills

Evidence that would reopen a decision of `register@agent-skills@design`. A tripwire leaves this
file by an exit of `skill@knowledge-architect-issue-tracking@movement-instruction`. What is outstanding is `register@agent-skills@issue`.

## Guarding `design@agent-skills@ledger-from-transcript`: a ruling lost or misstated at assembly `##ruling-lost-in-assembly`

The decision rests on the premise that assembly from the transcript, checked by the transcript
reviewer, records the owner's rulings as the owner made them, and an answer of the owner on a
reading or a fact with its source, per `design@agent-skills@answer-keeps-its-authority`.

**Fires when:** a ruling of the owner is found missing from, or misstated in, a plan document that
reached the main branch with no review reporting it, or in the plan documents of two separate
pieces of work, where the misstatement is critical under
`design@agent-skills@misstated-ruling-rated-by-what-it-changes`: reversed, or changing what is
built or a load-bearing decision.
**Response:** open a `defect` naming the ruling and the document, and reopen
`design@agent-skills@ledger-from-transcript`, with the rejected design "Writing the
discussion's ledger to a file during the discussion", `rejected@agent-skills@ledger-written-during-discussion`,
among the candidates.
**Re-entry:** the retrospective of the session that assembled the plan document: it reads that
session's own record against the committed document.
**Watches until:** 5 plan documents deleted at their landing.
**Evidence when written:** in the commit that added the structured-plans milestone document, its
transcript reviewers found three rulings recorded wider than the owner made them. The author wrote
each of the three, and the extraction none, and each was repaired before that commit. That is
before the commit, so it did not fire; it is why the guard exists. Two later instances, in the
discussion that produced the milestone load-bearing-records and in an earlier spec, recorded a
hedged word of the owner firmer than given, on details that were not load-bearing; the owner ruled
that such a slip is no critical defect, still repaired, and under the firing condition above
neither fires.

## Guarding `design@agent-skills@expectation-set-bounds-scope`: a contradiction between two installed instructions is closed as outside a skill's scope `##expectation-set-closes-a-contradiction`

The decision rests on the premise that an expectation set scopes the owner's behaviour only, so
that no finding of two installed instructions leaving no move satisfying both is closed by citing
one.

**Fires when:** a finding that two installed instructions leave no move satisfying both is closed
by citing an expectation set, in two separate analyses or reviews, or in a record that reaches the
main branch with no review reporting it.
**Response:** open a `defect` naming the finding and the set cited, and reopen
`design@agent-skills@expectation-set-bounds-scope` if its boundary sentence admits the reading.
**Re-entry:** the standing-state review of the change that closed the finding.
**Watches until:** three commits on the main branch that add analyses of received retrospective
files under docs/retrospective-reports/, counted from the commit that writes this line.

## Guarding `design@agent-skills@new-or-reshaped-head-needs-design`: a head is created, contradicted or given a member its argument does not cover with no deliberation recorded `##head-created-without-deliberation`

The decision rests on the premise that the design skill's description and the backstop, which the
primer hosts and the decision-recording skill, loaded before any write into a design home per
`design@agent-skills@design-home-write-loads-recording`, hosts again, reach a session before it
writes a decision that creates a head, contradicts one, or adds a member its argument does not
cover.

**Fires when:** a commit on the main branch creates a design head, contradicts a statement of one,
or adds to it a member its argument does not cover, and neither its message nor a plan document
carries the deliberation: no thread, no rival, and no record of the owner's ruling where the owner
ruled; or reviews of two separate branches each report such a commit of their branch. A relocation
or rewording that adds or removes no decision does not count. The instances that
`issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` records do not
count: they are that issue's.
**Response:** open a `defect` naming the head and the commit, and reopen
`design@agent-skills@new-or-reshaped-head-needs-design` on where its trigger is delivered; for a
member beyond the argument, with the rival that sends every decision earning durable text through
the design skill among the candidates.
**Re-entry:** the standing-state review before every merge: the commits and their messages are in
the branch's history.
**Watches until:** 20 commits on the main branch that add a level-three heading to a design home.

## Guarding `design@agent-skills@in-change-path`: a ruling lost or misstated in an in-change commit message `##ruling-lost-in-change`

The decision rests on the premise that a commit message records the owner's rulings as the owner
made them, as a plan document does, with no review of a plan document before the work.

**Fires when:** a ruling of the owner is found missing from, or misstated in, the commit message
that carries the deliberation of a discussion on the in-change path, after the branch's transcript
review, and the misstatement is critical under
`design@agent-skills@misstated-ruling-rated-by-what-it-changes`.
**Response:** open a `defect` naming the ruling and the commit, and reopen
`design@agent-skills@in-change-path` on its entry condition: whether a discussion with more than
one approved thread takes the full path.
**Re-entry:** the retrospective of each session that wrote such a commit message: it reads the
session's own record against the message.
**Watches until:** 10 commits on the main branch whose message names the in-change path and that
add a heading to a design home.

## Guarding `design@agent-skills@standing-entries-searched-before-the-work`: a deferred trigger met by work that bypasses the design skill and found only at the review `##deferred-trigger-met-by-undesigned-work`

The decision sends no search before work that goes through neither the design skill nor a
retrospective intake; bounded work that goes through the skill is searched at the grounding, per `design@agent-skills@bounded-path-in-design`. It rests
on the premise that a deferred trigger met by work that bypasses the skill costs little when the
review is the first to read it.

**Fires when:** a second instance is recorded of a deferred trigger met by work that did not go
through the design skill, and found only at the standing-state review. The first was in thaum: a move
of the pinned checker met the trigger of a deferred issue that named that move.
**Response:** open a `design` issue naming both instances, and propose a search before such work,
with its host.
**Re-entry:** the standing-state review before every merge, which reads every deferred trigger
against the change.
**Watches until:** three commits on the main branch that add analyses of received retrospective
files under docs/retrospective-reports/, counted from the commit that writes this line.

## Guarding `skill@knowledge-architect-planning@working-a-slice`: a critical gap an audit would have found, in a spec's work that skipped it as the rule allows `##skipped-spec-audit-would-have-found-a-gap`

The instruction that runs a spec's design audit only when a fresh session starts its work or the
main branch has moved, argued in the `%%` line beside it, rests on the premise that a spec
implemented by the session that wrote it, on a main branch with no commit since the spec other
than its own, has nothing load-bearing for an audit to find. The tripwire guards against a critical failure of
the workflow, not a small mistake that is not load-bearing.

**Fires when:** a review finds, in the work of a spec that skipped its design audit as the rule
allows, a critical gap an audit would have listed: a shape the tree refutes, a name the code does
not have, or a standing entry the work bears on that the grounding did not meet. A gap is critical
when the work's result, or a decision it rests on, is wrong without its repair.
**Response:** open a `defect` naming the spec and the change, and reopen the condition for the
audit in `skill@knowledge-architect-planning@working-a-slice`, with a design audit for the work of
every spec, as for every slice, the first candidate.
**Re-entry:** the standing-state review before every merge: the spec, its commit and the main
branch's history since are in the branch's range and its base.
**Watches until:** 6 specs deleted at their landing.

## Guarding `design@agent-skills@bounded-path-in-design`: a decision taken on the bounded path earns a head `##a-shortcut-decision-earns-a-head`

The decision rests on the premise that a session classifies work as bounded only when every
decision it makes fails the entry tests, and does not use the path to skip a discussion.

**Fires when:** a decision-record review finds, in a commit made on the bounded path, a decision
that passes an entry test of `design@agent-skills@a-head-is-owed-by-an-entry-test`, at instances in
two separate pieces of work; or such a decision is found once on the main branch with no review
having reported it.
**Response:** reopen `design@agent-skills@bounded-path-in-design` on its three conditions.
**Re-entry:** the decision-record review before every merge: the commit carries the proposal and,
if one came, the owner's answer.
**Watches until:** three commits on the main branch whose message names the bounded path, counted
from the commit that writes this line.

## Guarding `design@agent-skills@plan-read-against-the-record`: the design-conformance reviewer reports a widening the plan document lists as decided `##design-conformance-findings-needless`

The decision rests on the premise that the reviewer reports departures from the record, and not the
widenings a plan document makes on purpose, so that its findings are worth the owner's reading.

**Fires when:** in the reviews of two separate plan documents, the design-conformance reviewer
reports as a finding a widening or a reversal of a head that the document's "What is already
decided" lists as reversed or rewritten.
**Response:** open a `design` issue naming the two reviews and their findings, and reopen
`design@agent-skills@plan-read-against-the-record` on the reviewer's scope.
**Re-entry:** the standing-state review before every merge, which reads the outcome each review
commit records for each finding; and the retrospective of the session where the second of those
reviews ran, which reads that session's record.
**Watches until:** three plan documents deleted at their landing on the main branch, counted from
the commit that writes this line.

## Guarding `design@agent-skills@head-ground-is-the-argument`: the owner's own ground is removed from a head `##owner-intent-stripped`

The decision rests on the premise that the repair of an approval cited as a ground does not remove
the owner's own ground with it.

**Fires when:** a commit on the main branch has removed, or reworded as an argument, the owner's
ground in a design head, quoted or attributed, where a plan document or a commit message shows that
the decision, or that part, came from the owner, and no review reported it before the merge; or
reviews of two separate branches each report such a removal. A quotation converted to an
attribution that states the same ground, per `design@agent-skills@attribution-replaces-quotation`,
does not fire it.
**Response:** reopen `design@agent-skills@head-ground-is-the-argument` on how a ground is told
apart, with the rival that cites every ruling of the owner among the candidates; the removed ground
is restored as the repair of the instance.
**Re-entry:** the standing-state review before every merge, and the design-record axis of
`skill@knowledge-architect-project-audit`.
**Watches until:** 20 commits on the main branch that change a design home's heads' text, counted
as commits that add or remove a level-three heading of a design home.

## Guarding `design@agent-skills@title-states-the-rule`: a rule title wider than its argument `##rule-title-wider-than-its-argument`

The decision rests on the premise that a title moved to the rule stays within what its argument
argues.

**Fires when:** the reviews of two separate branches each find a title wider than its argument in a
head their branch writes or rewrites, or an audit finds one on the main branch in a head written or
rewritten after the decision.
**Response:** reopen `design@agent-skills@title-states-the-rule`; the title found is repaired as the
instance.
**Re-entry:** the decision-record review of every change that writes a head.
**Watches until:** 20 commits on the main branch that add a level-three heading to a design home,
and one pass of the design-record axis.

## Guarding `design@agent-skills@one-decision-per-head`: a split that separates a rule from its own exception `##split-rule-from-exception`

The decision rests on the premise that the nearest-rival test keeps a rule's exception with the
rule.

**Fires when:** a commit merges back two heads split under the decision; or the reviews of two
separate branches each find an exception split from its rule in a head their branch writes, or an
audit finds one on the main branch.
**Response:** reopen `design@agent-skills@one-decision-per-head`, with the reversal test among the
candidates.
**Re-entry:** the decision-record review of every change that writes a head.
**Watches until:** 20 commits on the main branch that add a level-three heading to a design home,
and one pass of the design-record axis.

## Guarding `design@agent-skills@restatement-size-test`: a directive missed because its restatement became a pointer `##pointer-not-followed`

The decision rests on the premise that a session follows a pointer to a directive's home at the
moment it needs the directive.

**Fires when:** reviews or retrospectives of two separate pieces of work each find a directive
broken where its restatement had been replaced by a pointer under the decision, or one such break
reaches the main branch with no review or retrospective reporting it.
**Response:** reopen `design@agent-skills@restatement-size-test`, with the rule it replaced, a
directive restated wherever it is delivered, among the candidates.
**Re-entry:** the retrospective of each session, and the review of every change.
**Watches until:** 12 retrospective analyses, each a file added under docs/retrospective-reports/.

## Guarding `design@agent-skills@audit-triggers`: a pin moves across a version whose changelog cites an axis, and the axis is not run `##axis-not-run-at-a-pin-move`

The decision rests on the premise that the move of a pin reads the Migration entries of the
versions crossed and runs the axis they cite. It watches agent behaviour in a consumer project,
which this project cannot see, per
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`.

**Fires when:** in two separate pin moves, received retrospective files show a pin moved across
such a version with the axis not run. An axis that the owner chose to replace or to scope at the
move is not counted, since the owner's request is the main occasion of an audit, per
`design@agent-skills@audit-triggers`.
**Response:** reopen `design@agent-skills@audit-triggers`.
**Re-entry:** the retrospective of a session that moved a pin.
**Watches until:** three commits on the main branch that add analyses of received retrospective
files under docs/retrospective-reports/, counted from the commit that writes this line.

## Guarding `design@agent-skills@workflow-axis-finding-standard`: auditors report predicted gaps as provable ones across passes `##audit-drafts-mostly-noise`

The decision rests on the premise that the finding standard keeps auditors from reporting predicted
gaps as provable ones, and L8's auditors from reporting two standards that differ in wording and
give one verdict, so the reading of a pass is not spent rejecting drafts.

**Fires when:** in two passes of the agentic-workflow axis, drafts fail the reading, such as a
predicted gap reported as a provable one or, from L8, two standards that differ in wording and give
one verdict. A draft the verifier refutes or the session rejects fails it; a draft marked kept or
known is not counted.
**Response:** reopen `design@agent-skills@workflow-axis-finding-standard` and the standard of
`agent@knowledge-architect-workflow-auditor`.
**Re-entry:** the session's verification of each pass's report.
**Watches until:** two report files of the agentic-workflow axis added under docs/audit-reports/ on
the main branch, counted from the commit that writes this line.

## Guarding `design@agent-skills@verifier-in-the-run`: a verifier refutes a true finding `##verifier-refutes-a-true-finding`

The decision rests on the premise that the adversarial verifier refutes only drafts a reading
refutes, since the session reads only a sample of the refuted drafts, chosen by its judgement.

**Fires when:** in two passes, of any axis, the session's sample of refuted drafts holds one that a
reading confirms.
**Response:** reopen `design@agent-skills@verifier-in-the-run`.
**Re-entry:** the session's verification of every report.
**Watches until:** 3 passes of any audit axis, each counted by its report file.

## Guarding `design@agent-skills@verifier-in-the-run`: the verifiers wave findings through `##verifiers-wave-findings-through`

The decision rests on the premise that the verifiers refute what a reading refutes, so that the
session's verification after the run is not a second sort.

**Fires when:** in two passes, of any axis, the session's verification rejects a finding a verifier
confirmed, on a refutation the verifier's brief gives it the record to make.
**Response:** reopen `design@agent-skills@verifier-in-the-run`.
**Re-entry:** the session's verification of every report.
**Watches until:** 3 passes of any audit axis, each counted by its report file.

## Guarding `design@agent-skills@repairs-read-against-the-record`: a repair reverses a head or brings back a rejected alternative its check did not name `##record-check-misses-a-head`

The decision rests on the premise that the check of a repair proposal finds the heads its edits
would reverse, although its search for the heads that describe a behaviour is a judgement.

**Fires when:** the reviews of two separate audits' branches each find a repair that contradicts a
design head, or brings back a rejected alternative, which the check of its proposal did not name;
one such miss, which the head's argument expects that review to find, does not fire it.
**Response:** reopen `design@agent-skills@repairs-read-against-the-record` on where the record
check runs.
**Re-entry:** the review of each audit's branch.
**Watches until:** 3 passes of the agentic-workflow axis, each counted by its report file.

## Guarding `design@agent-skills@calibration-in-the-run`: the session rejects the calibration's readings `##calibration-reading-rejected`

The decision rests on the premise that the calibration agent proposes readings the session
confirms, so that the session's confirmation is a check rather than a second calibration, and a
wrong reading is not applied in every group.

**Fires when:** in one audit, the session's verification rejects a reading the calibration agent
proposed, in two passes.
**Response:** reopen `design@agent-skills@calibration-in-the-run`, with the session settling the
reading among the candidates.
**Re-entry:** the session's verification of each report of the design-record axis.
**Watches until:** two report files of the design-record axis added under docs/audit-reports/ on
the main branch, counted from the commit that writes this line.

## Guarding `design@agent-skills@verifier-in-the-run`: a design-record verifier passes a wrong conforming draft `##conforming-draft-wrong`

The decision rests on the premise that a design-record verifier's sample of the drafts that find
their entry conforming is enough, so a violation a drafter missed does not wait a whole pass for
the re-check.

**Fires when:** in two passes, the session's own sample of conforming drafts holds one that a
reading shows failing a rule, and that the report does not list among the drafts whose verdict it
doubts.
**Response:** reopen `design@agent-skills@verifier-in-the-run`, on how much of the conforming drafts
a design-record verifier reads.
**Re-entry:** the session's verification of each report of the design-record axis.
**Watches until:** two report files of the design-record axis added under docs/audit-reports/ on
the main branch, counted from the commit that writes this line.
**Evidence when written:** on the first pass of the design-record axis through its saved workflow
on this repository, the session's sample held a wrong conforming draft that the drafter, the
verifier and the cross-check had each flagged as doubtful; the clause leaves such a draft out,
since a draft the report lists as doubtful is one the session reads.

## Guarding `design@agent-skills@one-run-pass`: the design-record cross-check misses an overlap `##overlap-missed`

The decision rests on the premise that the cross-check maps every pair of findings that edit one
paragraph or one slug, so the application meets no collision the report did not show.

**Fires when:** in the applications of two passes of the design-record axis, the session meets two
findings that edit one paragraph or one slug and that the cross-check's overlap map does not list;
or two such findings, both applied, reach the main branch with no review reporting the collision.
**Response:** reopen the design-record axis's cross-check, in `design@agent-skills@one-run-pass`.
**Re-entry:** the application of each pass of the design-record axis.
**Watches until:** the application of 3 passes of the design-record axis, each counted by the
deletion of its report file at its last outcome.

## Guarding `design@agent-skills@root-cause-repair` and `design@agent-skills@design-record-axis-dispatch`: one rule misread across groups outside the sample `##rule-misread-across-groups`

The decisions rest on the premise that the design-record axis's findings need no grouping by cause,
since the calibration sample catches a rule the auditors read apart.

**Fires when:** in two passes of the design-record axis, the session traces findings of more than
one group to one misreading of one rule that the calibration did not catch.
**Response:** reopen the absence of a cause step for the design-record axis, in
`design@agent-skills@root-cause-repair`, and how the sample is chosen, in
`design@agent-skills@design-record-axis-dispatch`.
**Re-entry:** the session's verification of each report of the design-record axis.
**Watches until:** 3 passes of the design-record axis, each counted by its report file.

## Guarding `design@agent-skills@questions-go-to-their-authority`: questions the record answers still reach the owner `##record-answered-questions-reach-the-owner`

The decision rests on the premise that routing a question by its authority, stated in the primer
every session reads and delivered where a skill writes a default, per
`design@agent-skills@derived-decision-proceeds-on-default`, overcomes the weight agents give to the
owner's word, which `design@agent-skills@misstated-ruling-rated-by-what-it-changes` records the owner
observing in agents.

**Fires when:** the retrospectives of two separate pieces of work each find a question put to the
owner that a goal, a design head or a source's text answered.
**Response:** reopen `design@agent-skills@questions-go-to-their-authority` and
`design@agent-skills@derived-decision-proceeds-on-default` on how the routing is delivered, for
example at the points where a skill puts a finding to the owner rather than a default.
**Re-entry:** the retrospective of each session, with the limit
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` names.
**Watches until:** 15 retrospective analyses, each a file added under docs/retrospective-reports/.

## Guarding `design@agent-skills@owner-word-holds-as-far-as-its-premise`: a deliberate departure from a specification is overridden as a reading `##departure-overridden-as-a-reading`

The decision rests on the premise that a project states its departures from a specification it
follows, so that a session can tell an owner's answer on a reading from a scope decision.

**Fires when:** a session applies a source's text over an answer the owner gave as a deliberate
departure from that source, in two separate pieces of work, or once in a record that reaches the
main branch with no review reporting it.
**Response:** reopen `design@agent-skills@owner-word-holds-as-far-as-its-premise` on how a
departure is recorded, so that a session can tell it from a reading.
**Re-entry:** the retrospective of each session, and the standing-state review before every merge,
with the limit `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`
names.
**Watches until:** three commits on the main branch that add analyses of received retrospective
files under docs/retrospective-reports/, counted from the commit that writes this line.

## Guarding `design@agent-skills@attribution-replaces-quotation`: an attribution loses the scope the owner's words had `##attribution-loses-the-scope`

The decision rests on the premise that an attribution states what the owner decided and how far it
reaches as well as the owner's words did.

**Fires when:** an attribution in a durable record misstates a ruling of the owner where the
owner's words would have shown its scope, and the misstatement is critical under
`design@agent-skills@misstated-ruling-rated-by-what-it-changes`, in two separate pieces of work, or
once on the main branch with no review having reported it.
**Response:** reopen `design@agent-skills@attribution-replaces-quotation` on where verbatim words
are owed, with the rejected design that quotes the owner in a head whose ground is the owner's,
`rejected@agent-skills@owner-words-quoted-in-head`, among the candidates.
**Re-entry:** the transcript review of each change, which reads the owner's words in the transcript
against the record.
**Watches until:** ten commits on the main branch whose subject opens with `[review]`, counted from
the commit that writes this line.

## Guarding `design@agent-skills@admission-rules-home`: an unneeded addition is made in a session that did not load the agent-configuration skill `##agent-text-written-without-the-standard`

The decision rests on the premise that a session writing agent-facing text loads the
agent-configuration skill, whose trigger delivers the standard at that moment, so that a home
outside the primer reaches every writer.

**Fires when:** an addition to agent-facing text that fails the need test of
`design@agent-skills@statement-names-an-act` or the real-use rule of
`design@agent-skills@behaviour-additions-need-real-use`, made in a session that did not load the
agent-configuration skill, is reported in two separate pieces of work, or reaches the main branch
with no review reporting it.
**Response:** reopen `design@agent-skills@admission-rules-home` on its rival the primer, a
one-sentence pointer in the primer to the standard.
**Re-entry:** the retrospective of that session, and the outcome of the review's finding.
**Watches until:** three commits on the main branch that add analyses of received retrospective
files under docs/retrospective-reports/, counted from the commit that writes this line.

## Guarding `design@agent-skills@additions-review-agent`: the agent-facing-text axis reports findings that need nothing `##agent-text-findings-need-nothing`

The decision rests on the premise that the agent reports statements that fail the need test, and
not its hard cases, so that a session spends its verification on findings that hold.

**Fires when:** in the reviews of two separate pieces of work, the axis on agent-facing text reports
findings and every one of them is judged to need nothing, by the session or by the owner.
**Response:** reopen the need test's readings in
`skill@knowledge-architect-agent-configuration@admission-standard`, which the agent and the
need-test lens both apply, and `design@agent-skills@statement-names-an-act` with them.
**Re-entry:** the outcome step of each review, `skill@knowledge-architect-review@what-review-leaves`.
**Watches until:** ten commits on the main branch that change a file under
`path@agent-skills@content/`, counted from the commit that writes this line.

## Guarding `design@agent-skills@checker-model-in-primer`: a rarer checker rule enters the model section `##rarer-rule-in-the-model`

The decision rests on the premise that the model section holds what a reader needs, and that a rule
a writer needs only for an uncommon shape reaches a session through its finding.

**Fires when:** a review or an audit finds a rule in `primer@checker-model` that a finding message
already delivers with its repair, other than one the head keeps there, the explanation of
`check --fix`'s refusal. A rule that a review of the branch adding it finds, and that the branch
removes before its merge, does not fire it, on the owner's ruling for this entry, which the rule on
slips states for every tripwire.
**Response:** reopen `design@agent-skills@checker-model-in-primer` with the owner, on the scope of
the owner's ruling that the rarer checker rules are delivered by their findings; the rule found is
removed from the section as the repair of the instance.
**Re-entry:** that review or audit.
**Watches until:** 10 commits on the main branch that change `path@agent-skills@content/PRIMER.md`.

## Guarding `design@agent-skills@rejected-citation-direction`: an entry cited only by heads that do not decide its question `##rejected-entry-cited-by-the-wrong-head`

The decision rests on the premise that the head citing a `rejected` entry is the one whose argument
covers it, so that a session reading that head meets the design.

**Fires when:** an audit, or the review of a branch that did not add the citation, finds on the
main branch a `rejected` entry whose only citing heads decide a question other than the one the
design it records answered; or the reviews of two separate branches each find one that their
branch added.
**Response:** reopen `design@agent-skills@rejected-citation-direction` on how the citing head is
chosen.
**Re-entry:** the decision-record review of every branch that adds a `rejected` entry, and the
design-record axis of `skill@knowledge-architect-project-audit`.
**Watches until:** 15 commits on the main branch that add a level-three heading to a `rejected`
home, and one pass of the design-record axis.

## Guarding `design@agent-skills@design-entry-search`: a design entry missed though the search ran `##design-entry-search-misses`

The decision rests on the premise that a search at grounding, sent again for a later thread, puts
before the session every design entry and `rejected` entry its proposals bear on.

**Fires when:** in two separate discussions or intakes where the design-entry search ran, a head
contradicted, or a `rejected` entry brought back without a recorded reopening, is found that the
search did not return: by the design-conformance review of a plan document, or by an outcome the
owner ruled in the intake. Or once, on the main branch, with no review having reported it.
**Response:** open a `design` issue naming the review and the discussion or the intake, and reopen
`design@agent-skills@design-entry-search`, asking whether the miss came from a thread or a subject
that opened after the search.
**Re-entry:** the reviews of every plan document, each retrospective, and the review of the branch
that carries out an intake's outcomes.
**Watches until:** three plan documents deleted at their landing, and three commits that add
analyses of received retrospective files under docs/retrospective-reports/, on the main branch,
counted from the commit that writes this line.

## Guarding `design@agent-skills@searcher-grades-goals`: a goal is missed where the searcher graded the goals `##goal-missed-though-graded`

The decision rests on the premise that the searcher's grades of the goals are a second reading
beside the session's own, which the session keeps, and not a summary it reads instead.

**Fires when:** a goal is missed in a design discussion or a retrospective intake where the
design-entry searcher graded that goal's Component's goals, in two separate pieces of work, or once
in a record that reaches the main branch with no review reporting it.
**Response:** reopen `design@agent-skills@searcher-grades-goals` on how goals reach the session.
**Re-entry:** the retrospective of that session, and the review of the change.
**Watches until:** three plan documents deleted at their landing, and three commits that add
analyses of received retrospective files under docs/retrospective-reports/, on the main branch,
counted from the commit that writes this line.

## Guarding `design@agent-skills@harness-shell-fails-on-a-pipe`: a chained command runs over a failing piped check in a project with the option `##piped-chain-runs-under-pipefail`

The decision rests on the premise that the option reaches every shell the harness starts.

**Fires when:** a retrospective reports a command chained on a piped verdict that ran over a failing
check, in a project whose harness settings set the option.
**Response:** reopen `design@agent-skills@harness-shell-fails-on-a-pipe` on how the option reaches
the shell.
**Re-entry:** each retrospective intake.
**Watches until:** 10 retrospective analyses, each a file added under docs/retrospective-reports/.

## Guarding `design@agent-skills@harness-shell-fails-on-a-pipe`: a session misreads a status the option produced `##pipefail-status-misread`

The decision rests on the premise that the statuses the option adds, 141 after `head` and 1 after a
`grep` that matched nothing, fail safe.

**Fires when:** a retrospective reports a session that read such a status as a failure of the
command before the pipe, and acted on it.
**Response:** reopen `design@agent-skills@harness-shell-fails-on-a-pipe` on what the setup tells a
session of the statuses.
**Re-entry:** each retrospective intake.
**Watches until:** 8 retrospective analyses, each a file added under docs/retrospective-reports/.

## Guarding `design@agent-skills@workflow-axis-lenses`: in a pass, one agent of each duplicated lens alone reports every confirmed finding of that lens `##duplicate-auditors-add-nothing`

The decision rests on the premise that a second agent on one lens finds what the others of that lens
missed, so its cost buys coverage.

**Fires when:** in one pass, for every lens dispatched more than once, one of its agents alone
reported every confirmed finding of that lens. Two L2 agents count as one lens dispatched twice
only when they share a group of activities.
**Response:** reopen `design@agent-skills@workflow-axis-lenses` on the duplicate dispatch.
**Re-entry:** the session's verification of each pass's report counts it.
**Watches until:** the commit that closes
`issue@agent-skills@no-check-reports-a-duplicated-workflow-lens-whose-other-agents-found-nothing`,
whose check then carries it, and which deletes this entry.

## Guarding `design@agent-skills@tripwire-admission-test`: sessions read "silent" too widely, and drop a cause or remove a tripwire whose failure was silent `##silent-read-too-widely`

The decision rests on the premise that a session can tell, at recording or later, a failure every
procedure would count as success from one a test, a check, a review or a request would reach.

**Fires when:** an issue is opened on a silent failure of a decision, and a commit message or a plan
document lists the cause of that failure as dropped, or the tripwire watching it as removed, under
condition 1.
**Response:** reopen `design@agent-skills@tripwire-admission-test`, on how condition 1 is worded.
**Re-entry:** the standing-state review: the issue and the commit are in the tree and the history.
**Watches until:** the fifth plan document deleted at its landing on the main branch, and the second
report file of the tripwire axis added under docs/audit-reports/ on the main branch, both counted
from the commit that writes this line.

