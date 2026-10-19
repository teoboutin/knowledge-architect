# 2026-10-15-knowledge-architect-primer-relocation-klarch-workflow

- **Received file**: 2026-10-15-knowledge-architect-primer-relocation-klarch-workflow.md, the
  workflow file of the retrospective of the session that moved the general root instructions into
  the primer: a design discussion, a spec, its four steps on one branch, the reviews, the merge.
- **Version used**: built from source, main at `v0.7.0-1-g7af8ca20`. That SHA resolves to no object
  in this repository; the count resolves, `git describe --tags 001de95` printing `v0.7.0-1-g001de95`,
  and the text was read there.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**, from one search over the 84 issues and 60 tripwires, run for
  the 13 files received together:
  - `a-spec-owes-no-claims-the-landing-commits-must-name`, W1;
  - `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways`, W4;
  - `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` and
    `tripwire@agent-skills@rarer-rule-in-the-model`, W3;
  - `spec@plans@defined-ids-outside-references`, `tripwire@core@candidate-rule-silence` and
    `tripwire@agent-skills@plain-text-pointer-found`, C1;
  - `issue@core@no-checked-form-cites-a-point-inside-a-section`, C2;
  - `issue@core@a-plain-hash-item-name-is-read-by-nothing`, C3;
  - `tripwire@agent-skills@attribution-loses-the-scope`, judged in the last section for every file
    received.

## W1, with P1 of 2026-10-15-knowledge-architect-primer-relocation. A spec's steps carry no claims

- **a. What it says.** A slice's entry owes a Claims section; a spec's implementation sequence owes
  only what each step builds and what it fails alone on. The implementing session wrote two claims
  for a command whose decided design stated six properties. An adversarial reviewer's thirteen
  mutations left eight non-equivalent survivors, one hiding a real defect. The fix proposed: the
  "Implementation sequence" row of `skill@knowledge-architect-planning@spec-contents` gives each
  step its Claims, pointing to the slice entry's definition. Its project side, P1, blames
  `skill@klarch-development@discriminating-tests`: "otherwise, a spec included, the session writes
  them".
- **b. Whether it still applies.** Yes, unchanged at both commits. In
  `path@agent-skills@content/skills/planning/SKILL.md`, the row reads "Concise: what each builds and
  what it fails alone on" (line 176); a slice's entry holds "**Claims**: each with the test that
  could refute it, and how the test is shown to fail against a wrong implementation" (line 288); and
  point 3 of `skill@knowledge-architect-planning@working-a-slice` already has the commits "name each
  claim of the spec with its test and its outcome" (line 431), so the skill presumes claims a spec is
  never asked to hold. The project's sentence is at `path@agent-config@skills/klarch-development/SKILL.md`,
  lines 63-66, unchanged. One sub-claim is stale: the eighth surviving mutation, said to survive on
  main, was pinned after the report by `cd09fad`, "[core] A test pins that `anchors` judges a file
  register's home by its directory, not by its README".
- **c. Whether it is already recorded.** Yes:
  `a-spec-owes-no-claims-the-landing-commits-must-name` is the same defect, and
  names this fix as one of its two closings; the other is the owner's ruling that the decided design
  is enough. It gates the fix on a reading of past specs, which was not done. No `rejected` entry
  covers the fix: `rejected@agent-skills@spec-plus-detailed-plan` is about steps written out with
  their code.
- **d. Its kind.** A defect of the text, missing, provable by reading: point 3 asks the commits to
  name claims that no section of a spec states.
- **e. Its scope.** It threatens `goal@knowledge-architect@agents-get-a-complete-workflow`. The
  owner of the fix is the installed text, edited here at its source. The record check found no head
  on the sections of a spec: a search of the agent-skills design home's titles for spec, claims and
  section returns none that states them. It reverses no decision.
- **f. A better fix.** The nearest rival is the issue's other closing, leaving the claims to the
  implementing session. This session is the real use that defeats it: the session wrote the claims
  it had in mind, and eight properties the decided design stated went unpinned. The reading of past
  specs the issue asked for is no longer needed to decide: it would count how often the gap occurred,
  and one occurrence with a hidden defect is the evidence the admission standard asks for. The shape
  already works in a spec: the first step of `spec@plans@defined-ids-outside-references` carries
  "Claims, each with its test". Instructions added: 0, one row reworded to name the section point 3
  already presumes. P1 then becomes a pointer: the development skill's sentence says a spec's step
  presents its claims as a slice's entry does.
- **g. Its route.** An edit of the installed text, under `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent", with `skill@knowledge-architect-agent-configuration`; and an edit of
  `skill@klarch-development` for P1.
- **h. The proposal.** Handle now: the spec's implementation sequence gives each step its Claims,
  defined as a slice entry's are; `skill@klarch-development@discriminating-tests` points to them for
  a spec as for a slice; the issue closes in that commit.
- **Default**: handle now, as h says, closing
  `a-spec-owes-no-claims-the-landing-commits-must-name`.
- **Outcome**: accepted.

## W2. The searcher's dispatch description does not state the order of its positions

- **a. What it says.** The description of `agent@knowledge-architect-standing-entry-searcher` says
  "each on a group of consecutive positions of the combined count", with no order. The session
  briefed its searchers with positions "in order" of the listings, which print issues by kind; the
  searchers used the order their definition fixes. The fix proposed: the description names the
  order.
- **b. Whether it still applies.** Yes. The description, line 3 of
  `path@agent-skills@content/agents/standing-entry-searcher.md`, is unchanged. The body fixes the
  order at lines 48-55, unchanged since its commit: "**The order is fixed, so that every search agent
  of one search takes the same slice**: the issue rows sorted by anchor, then by id; then the
  tripwire rows sorted by anchor, then by id, each sort in byte order".
- **c. Whether it is already recorded.** Not as such.
  `issue@core@searcher-reports-are-written-and-checked-by-hand` has a lead, a command that prints a
  group's entries "so the agent no longer derives the group or its order", which would make the
  order mechanical.
- **d. Its kind.** Unclear, narrowly: the dispatcher needs only position numbers, and the order is
  fixed where the agent reads it. The defect bites only when a brief restates an order of its own,
  as this one did. This intake's own brief gave positions only, and the three searchers agreed.
- **e. Its scope.** Installed text, no head.
- **f. A better fix.** Rivals: the description names the order (the proposal); the description says
  the positions follow the order the agent's definition fixes, a pointer of a few words; no change,
  since the searchers followed the definition. The pointer wins over the proposal: a restated sort
  key in the description is a second home of the order, which drifts. It wins over no change because
  the description is the only text the dispatcher is told to follow. Instructions added: 0.
- **g. Its route.** Installed text edit.
- **h. The proposal.** Handle now: the description says the positions are those of the order its
  definition fixes, and that a brief gives positions only.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W3. A tripwire about text can fire on the branch that writes the text

- **a. What it says.** The premortem's tripwire on the primer's model section,
  `tripwire@agent-skills@rarer-rule-in-the-model`, was met by a rule the same branch wrote and its
  own review found. Reviewers split between fired and a repair. The fix proposed: one sentence in
  `skill@knowledge-architect-issue-tracking@tripwire-entry`, a tripwire watches the record once its
  decision is on the main branch.
- **b. Whether it still applies.** Yes: neither the tripwire entry's section of
  `path@agent-skills@content/skills/issue-tracking/SKILL.md` nor
  `skill@knowledge-architect-decision-recording@ruled-tripwires` says from when a tripwire watches.
- **c. Whether it is already recorded.** In part. The owner ruled this tripwire's case, option (a)
  of Q2, as an exception in its own clause, `a23c261`, "[review] The owner's ruling on Q2: a rarer
  rule caught and removed on its own branch does not fire T2": the report's nearest rival.
  `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` holds the same family: two
  tripwires met by their letter on slips repaired before their merge, ruled not fired on that
  ground. Its re-entry is "the next change to the tripwire shape of the issue-tracking skill", which
  the proposal is.
- **d. Its kind.** Unclear: the shape is silent on the window a tripwire watches.
- **e. Its scope.** The tripwire shape is installed text, and the decision on it is the one the
  severity issue keeps open. The proposal changes the shape, so it opens that discussion.
- **f. A better fix.** One fix for the cluster: the severity issue's discussion decides the bound of
  a firing, of which "repaired before the merge" is one form; this finding, and the attribution
  tripwire's instances judged below, are its evidence. Writing the sentence now would decide that
  issue's question by one of its cases.
- **g. Its route.** The design discussion of the severity issue, under
  `skill@knowledge-architect-design`.
- **h. The proposal.** Handle now: add this case to
  `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` as an instance, and leave the
  sentence to its discussion.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W4. A premortem criterion listed as a default awaiting the owner reads two ways

- **a. What it says.** The report names its own record of the finding.
- **b. Whether it still applies.** Yes, both sentences unchanged at lines 337 and 365 of
  `path@agent-skills@content/skills/planning/SKILL.md`.
- **c. Whether it is already recorded.** Yes,
  `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways`, opened by the
  session's own branch.
- **d. to g.** Its closing is a design question on two heads, which the issue states.
- **h. The proposal.** No change: already recorded.
- **Default**: no change, already recorded.
- **Outcome**: accepted.

## C1, with C1 of 2026-10-18-thaum-chance-slice-merge-klarch-workflow. A reference-shaped token outside backticks is never reported

- **a. What it says.** This file: two `path` references written without backticks in commit
  messages passed `commits`; reviewers caught them. The other: one commit message listed 75 lines of
  the shape of a tripwire reference with no backticks, and `commits` passed it. Both propose a
  finding for a token with a known kind and the segments of a reference.
- **b. Whether it still applies.** Yes: no such check exists. A search of
  `path@core@src/` for an unbackticked-token lint finds none. One instance of this file is still on
  main: the message of `9bdf27a`, "[review] Repairs of the review before the merge: the model
  section drops a rarer rule, the moved text regains what it lost, and the record states its scope
  and hedges", holds such a token at its line 42, so the report's "reworded" is wrong for at least
  one of the two. Main's history is not rewritten, so it stays.
- **c. Whether it is already recorded.** Yes, and scheduled:
  `spec@plans@defined-ids-outside-references` plans exactly this as its class A, "a reference-shaped
  token outside any backtick span", in every prose region the checker reads, commit messages
  included, read against their first parent's tree too. `tripwire@core@candidate-rule-silence`
  excepts this shape for that reason. `tripwire@agent-skills@plain-text-pointer-found` is not met by
  these: no checked pointer was turned into plain text to clear a finding; the backticks were
  forgotten.
- **d. Its kind.** A blind spot of the checker, already planned.
- **e. to g.** Scope: `goal@core@declared-instructions-are-checked`. Route: the spec's work.
- **h. The proposal.** No change: the spec schedules it.
- **Default**: no change, scheduled by `spec@plans@defined-ids-outside-references`.
- **Outcome**: accepted.

## C2. No checked form cites a subsection

- **a. to c.** The planning skill names a primer subsection by its title in words, at line 210 of
  `path@agent-skills@content/skills/planning/SKILL.md`, beside the section's reference. Recorded:
  `issue@core@no-checked-form-cites-a-point-inside-a-section`, as the report says.
- **h. The proposal.** No change.
- **Default**: no change, already recorded.
- **Outcome**: accepted.

## C3. A plain hash name of an item inside its own plan is not reported

- **a. to c.** The spec's first draft named its own items in plain text with a hash sign; the
  routing reviewer found them. Recorded: `issue@core@a-plain-hash-item-name-is-read-by-nothing`,
  covering the inside case, which `spec@plans@defined-ids-outside-references` leaves to it. The
  report's count of 106 is not reproduced: a count over the spec at `4b29249` gives 130 such tokens,
  by a different method; the difference does not bear on the finding.
- **h. The proposal.** No change.
- **Default**: no change, already recorded.
- **Outcome**: accepted.

## The standing answers

- **a.** No installed file edited by hand. Three primer rules acted on as if absent: two plain-text
  pointers (C1); an attribution that dropped the owner's hedges, and a hedged proposal recorded as
  the owner's word, both caught by reviewers before the merge; an item dropped with a reason that
  did not say why it needed nothing, caught by the transcript reviewer.
- **b.** The quotations hold, in `path@agent-skills@content/PRIMER.md`. The hedged proposal was
  reattributed in `576f966`, "[review] Repairs of the last transcript review: the changelog names
  the anchor list a project can delete, an issue for the planning skill's two readings, and a
  proposal attributed as one".
- **d.** Lapses, each caught before the merge. The attribution slip is judged with the others in
  the next section.
- **h. The proposal.** No change.
- **Default**: no change, lapses caught by the reviews.
- **Outcome**: accepted.

## `tripwire@agent-skills@attribution-loses-the-scope`: whether it fired

Judged once for every file received, since four of them report the same slip.

- **The firing clause**: "the transcript reviewer reports a misstated ruling that traces to an
  attribution in a durable record, where the owner's words would have shown the scope".
- **The evidence**:
  - this file: a hedged proposal recorded as the owner's word in a design head, and an attribution
    without the owner's "tend to" and "often", caught by the transcript reviewer and a reviewer
    before the merge;
  - 2026-10-15-thaum-pin-move-0-7-0-klarch-workflow: nine of 28 attributions an auditor drafted
    dropped a hedge or a scope; the refuter caught them before application, two more were caught by
    the review. No transcript reviewer report, so the clause is not met there;
  - 2026-10-19-thaum-boundary-fuzzing-census-klarch-workflow: two hedges dropped at the harvest,
    the commit message claiming they were kept, found by two reviewers; the file does not say
    whether the transcript reviewer was one;
  - 2026-10-19-thaum-boundary-fuzzing-design-klarch-workflow: three attributions in harvest rows
    narrower or firmer than the owner's words, caught by the two transcript reviews before the merge,
    none load-bearing, as the file says.
- **The judgement proposed**: met by its letter, at least twice; not fired. Every instance was a
  slip caught and repaired before the merge, the case the retrospective skill counts as no finding,
  "A ruling of the owner recorded a little wider, narrower or firmer than given, on a detail that is
  not load-bearing, and caught by the transcript reviewer before the merge, is no finding", and the
  case `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound` records this same tripwire
  as met by its letter and ruled not fired. The rate is evidence for that issue: the attribution
  rule's premise holds at the reviews, not at the writing.
- **Default**: not fired; the instances are added to
  `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, with W3 above.
- **Outcome**: accepted.
