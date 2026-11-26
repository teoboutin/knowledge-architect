# Tripwires — agent skills

Evidence that would reopen a decision of `register@agent-skills@design`. A tripwire leaves this
file by an exit of `skill@knowledge-architect-issue-tracking@movement-instruction`. What is outstanding is `register@agent-skills@issue`.

## Guarding `design@agent-skills@bounded-path-in-design`: a decision taken on the bounded path earns a head `##a-shortcut-decision-earns-a-head`

The decision rests on the premise that a session classifies work as bounded only when every
decision it makes fails the entry tests, and does not use the path to skip a discussion.

**Fires when:** a later procedure, such as an audit, the review of a branch that did not make it,
or a retrospective file the intake receives, finds on the main branch a decision taken on the
bounded path that passes an entry test of `design@agent-skills@a-head-is-owed-by-an-entry-test`.
**Response:** reopen `design@agent-skills@bounded-path-in-design` on its three conditions.
**Re-entry:** the decision-record review of a branch whose diff touches code that a commit made on
the bounded path by another branch introduced, which reads that commit's message; the intake of a
retrospective file, which reads each tripwire whose firing evidence the file carries; and each pass
of a project audit, whose report marks a finding that bears on a tripwire.
**Watches until:** three commits on the main branch whose message says its proposal was put to the
owner as bounded work, found by `git log -i --grep='as bounded work'` and confirmed by reading each,
and for each of them a later commit on the main branch that changes a file it changed and that the
first release tag containing it does not contain, counted from the commit that writes this line: a
release tag is set on the main branch after a merge, so a commit it does not contain came from
another branch. No instruction asks a bounded-path commit for that phrase, so the search can miss
one worded otherwise: the watch then runs longer than three such commits, and no firing is lost.

## Guarding `design@agent-skills@one-decision-per-head`: a split that separates a rule from its own exception `##split-rule-from-exception`

The decision rests on the premise that the nearest-rival test keeps a rule's exception with the
rule.

**Fires when:** a commit on the main branch merges back two heads that another branch split under
the decision, on the ground that they were one decision when split, such as a rule and its own
exception; or an audit, or the review of a branch that did not write the head, finds on the main
branch an exception split from its rule.
**Response:** reopen `design@agent-skills@one-decision-per-head`, with the reversal test among the
candidates.
**Re-entry:** each pass of the design-record axis of a project audit, the decision-record review of
a change that touches a head it did not write, and the standing-state review, for a commit that
merges back two heads that another branch split, on that ground.
**Watches until:** 20 commits on the main branch that add a level-three heading to a design home,
and one pass of the design-record axis. A commit whose
added level-three heading lines are all lines it also removes, a relocation, is not counted.

## Guarding `design@agent-skills@title-misleads-bar`: a title held to not misleading stays within what its head decides `##misleading-title-found-on-main`

The decision rests on the premise that the test of a misleading title, which replaces the tests of
the argument's reach and of the decision's full scope, keeps a title from stating a rule its body
does not argue. The suspected mechanism of the earlier recurrence: a title moved from the members
to the rule states the rule in terms the session chooses, and those terms reach further than the
argument the body carries. That recurrence is a title wider than its argument, found and repaired
by the reviews of five branches merged from 2026-10-24, which the issue that the commit "[plans]
The spec on head titles that must not mislead and on tripwires that fire on a defect found on main"
closed recorded, read at the parent of that commit under `register@agent-skills@issue`.

**Fires when:** an audit, or the review of a branch that did not write or retitle the head, finds on
the main branch a title that misleads, in a head written or retitled after the decision.
**Response:** reopen `design@agent-skills@title-misleads-bar`; the title found is repaired as the
instance.
**Re-entry:** each pass of the design-record axis of a project audit, and the review of a branch
that touches a head written or retitled after the decision.
**Watches until:** the second report of the design-record axis in
`path@knowledge-architect@docs/audit-reports/` after the decision.

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

## Guarding `design@agent-skills@rejected-citation-direction`: an entry cited only by heads that do not decide its question `##rejected-entry-cited-by-the-wrong-head`

The decision rests on the premise that the head citing a `rejected` entry is the one whose argument
covers it, so that a session reading that head meets the design.

**Fires when:** an audit, or the review of a branch that did not add the citation, finds on the
main branch a `rejected` entry whose only citing heads decide a question other than the one the
design it records answered.
**Response:** reopen `design@agent-skills@rejected-citation-direction` on how the citing head is
chosen.
**Re-entry:** the design-record axis of `skill@knowledge-architect-project-audit`, and the
decision-record review of a branch that touches a head citing a `rejected` entry it did not
add.
**Watches until:** 15 commits on the main branch that add a level-three heading to a `rejected`
home, and one pass of the design-record axis.

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
condition 1, or lists the check that would have caught it as a claim of the step that builds it
rather than an acceptance criterion.
**Response:** reopen `design@agent-skills@tripwire-admission-test`, on how condition 1 is worded.
**Re-entry:** the standing-state review: the issue and the commit are in the tree and the history.
**Watches until:** the later of two occasions, each counted from b1e171d7, "[review] Repairs of
the review of the decision that a tripwire stands on the standard", the commit that added the second
occasion to this line and counted both from itself: the fifth plan document deleted at its landing
on the main branch, and the second report file of the tripwire axis added under docs/audit-reports/
on the main branch.

## Guarding `design@agent-skills@audit-net-growth`: an intake's repairs, held for no word of the owner, add instructions that interact `##intake-repairs-interact-unheld`

The decision holds a set of repairs for the owner's word on its growth in a project audit only, on
the premise that an intake's findings, bounded by real use, do not grow the text into rules that
interact, as the audit's unbounded passes did. Kept on a doubt.

**Fires when:** a pass of the agentic-workflow axis of an audit reports two instructions that leave
no move satisfying both, or a narrow rule where judgement decides, one of which an intake's repair
added, found in the audit's report file of that pass.
**Response:** reopen `design@agent-skills@audit-net-growth` on whether an intake's repairs wait for
the owner's word on their growth.
**Re-entry:** the session's verification of each pass of the agentic-workflow axis.
**Watches until:** two audit reports of the agentic-workflow axis, counted from the commit that
writes this line.

## Guarding `design@agent-skills@default-code-shape` and `design@agent-skills@module-split-guidelines`: a shape finding or a signal for a module split leads a session to split code that gains nothing from the split `##shape-findings-fragment-code`

The decisions rest on the premise that the fragmentation clause of the default shape, and the
signals against a split of the module guidelines, hold a session back from splitting code to answer
a finding of one reason to change or a signal of
`agent@knowledge-architect-code-split-scout`. Each procedure counts such a split as success: the
tests pass, a move-only step is built as planned, the review reports nothing left, and only a reader
of the result sees the cost.

**Fires when:** an analysis of a retrospective workflow file received under `path@knowledge-architect@docs/retrospective-reports/`
reports, in answer to the standing question on rules about code and splitting, a split that bought
nothing: one made to satisfy the default shape, a function with one
caller, no test of its own and a name that says no more than its body; or a module split planned on
a signal of the module guidelines whose parts are read against one reference and changed together
in every later change that touched them.
**Response:** reopen `design@agent-skills@default-code-shape` on how the fragmentation clause is
worded and delivered, or `design@agent-skills@module-split-guidelines` on its signals against a
split, with the instance found.
**Re-entry:** the intake of a retrospective workflow file, `skill@klarch-retrospective-intake`, and
the standing-state review.
**Watches until:** three commits on the main branch that add analyses of received retrospective
workflow files, counted for both decisions from the commit that writes this line.

## Guarding `design@agent-skills@what-shows-a-goal-met-is-design`: no installed instruction that a goal's measure used to name is removed as unneeded and then found missing `##a-measure-protected-instruction-is-removed-as-unneeded`

A goal states no measure, so no goal names the act of an installed instruction any longer, and the
need test alone keeps a needed instruction in the installed text. An audit or a review that
removes such an instruction as unneeded passes every procedure: the removal is a repair, and the
gap shows only in a later session.

**Fires when:** an analysis of a retrospective workflow file received under
`path@knowledge-architect@docs/retrospective-reports/`, or an audit report of the agentic-workflow
axis, reports a session that lacked an instruction which an earlier audit or review had removed on
the need test, and whose act a measure of a goal stated in a goals home as the commit `cb8d5a22`
holds it, the root's or a Component's, which
`git grep -n 'It is met while' cb8d5a22 -- ':(glob)**/docs/goals.md'` lists.
**Response:** reopen `design@agent-skills@what-shows-a-goal-met-is-design` on what protects a needed
instruction once no goal names its act, with the instance.
**Re-entry:** the intake of a retrospective workflow file, `skill@klarch-retrospective-intake`, and
the standing-state review.
**Watches until:** two audit reports of the agentic-workflow axis added under
`path@knowledge-architect@docs/audit-reports/` on the main branch, counted from the commit that
writes this line.

## Guarding `design@agent-skills@one-session-cost-weighed-before-removal`: an instruction removed or narrowed on one session's needless finding is needed by other sessions `##needless-removal-found-missing`

A needless finding reports one task at one moment. An intake that removes or narrows the
instruction on it passes every procedure: the removal is a repair, and the sessions that needed the
instruction at other moments show the gap only later.

**Fires when:** an analysis of a retrospective workflow file received under
`path@knowledge-architect@docs/retrospective-reports/`, or an audit report of the agentic-workflow
axis, reports a session that lacked an instruction which an earlier intake removed or narrowed on a
needless finding.
**Response:** reopen `design@agent-skills@one-session-cost-weighed-before-removal` on how a cost one
session met is weighed before a removal, with the instance.
**Re-entry:** the intake of a retrospective workflow file, `skill@klarch-retrospective-intake`, and
the standing-state review.
**Watches until:** three commits on the main branch that add an analysis under
`path@knowledge-architect@docs/retrospective-reports/`, counted from the commit that writes this
line.

## Guarding `design@agent-skills@harvested-slice-kept`: a later slice builds on an item of a harvested slice whose decision an earlier landing reversed `##harvested-item-built-after-its-reversal`

The decision rests on the premise that a harvested slice's spec serves the later slices of its
milestone without misleading them: a later slice that cites one of its items reads the design homes,
which win over it where an earlier landing of the milestone reversed what the item states.

**Fires when:** an issue, an audit report or a retrospective file reports work merged on the main
branch that follows a decision stated by an item of a harvested slice's spec, where the design
homes held, at that merge, a later decision that reversed it.
**Response:** reopen `design@agent-skills@harvested-slice-kept`, on whether a harvested slice's
items stay citable from inside its milestone.
**Re-entry:** the standing-state review: the issue, the report or the analysis is in the tree.
**Watches until:** the second milestone whose directory leaves the main branch holding a slice under
`harvested/`, counted from the commit that writes this line.
