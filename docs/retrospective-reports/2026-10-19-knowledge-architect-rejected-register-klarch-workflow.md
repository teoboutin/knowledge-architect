# 2026-10-19-knowledge-architect-rejected-register-klarch-workflow

- **Received file**: 2026-10-19-knowledge-architect-rejected-register-klarch-workflow.md, the
  workflow file of the session that made the rejected designs a register: a design discussion, a
  spec merged on its own, then its work on a second branch, both merged.
- **Version used**: built from source, `v0.7.0-62-g6e2b300`, main after both merges. Its W1 quotes
  the design skill as it stood before the session's own change.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**:
  - for W1: `tripwire@agent-skills@design-entry-search-misses`, not met, since the search did not
    exist during the miss; `issue@agent-skills@the-design-entry-search-runs-only-at-a-discussion-s-grounding`,
    adjacent;
  - for W2: `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, which shows a touch's
    cost on heads with many citing sites;
  - for C1: `issue@core@searcher-reports-are-written-and-checked-by-hand` and
    `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt`;
  - for W3, with W1 of 2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow:
    none.

## W1. The design skill's grounding asks for whole design homes, and a session does not read them whole

- **a. What it says.** The session read the heads it found by grep, not the design homes whole, and
  missed a live rejected design and two heads; the spec's design-conformance review found the miss
  after the owner had ruled. The remedy, a design-entry searcher at the grounding, was built by the
  same discussion and ships. The file proposes nothing beyond it.
- **b. Whether it still applies.** Repaired by the session's own branch: `4612058`,
  "[agent-skills] A design head cites the `rejected` entries its argument covers, and a design
  discussion sends a design-entry searcher at its grounding". Step 1 of
  `skill@knowledge-architect-design@discussion-loop` now sends two searches beside the session's
  reading, `path@agent-skills@content/skills/design/SKILL.md` lines 420-430.
- **c. Whether it is already recorded.** The tripwire that watches the new search,
  `tripwire@agent-skills@design-entry-search-misses`, fires only where the search ran, so not here.
- **d. Its kind.** An observation, repaired.
- **h. The proposal.** No change.
- **Default**: no change, repaired by `4612058`.
- **Outcome**: accepted.

## W2. Whether adding a citation to a head touches it

- **a. What it says.** The primer says "Re-pointing a reference in a head does not touch it". The
  conversion added 116 citation sentences to existing heads; the session read a citation moved from
  an entry into the head as a re-pointing, and recorded that reading in the spec. A review later
  asked whether two heads touched by new citations counted as touched. The fix proposed: "nor does
  adding a citation of an entry its argument already covers".
- **b. Whether it still applies.** The primer is unchanged, `path@agent-skills@content/PRIMER.md`
  lines 590-592, and silent on an added reference. The decision is not:
  `design@agent-skills@existing-heads-on-touch` states "**Re-pointing a reference in a head does not
  touch it.** … Adding a reference, or rewording a sentence, is more than re-pointing."
- **c. Whether it is already recorded.** Yes, by that head, against the proposal. Its argument for
  the exemption is that "a change that re-points references across the record, as a renamed entry
  or a migrated reference form does, would otherwise owe the repair of every head it passes through,
  which is a project audit's scope, not that change's". A conversion that adds citations across the
  record meets that argument too, which the head's last sentence does not weigh.
- **d. Its kind.** Two parts: the primer does not deliver a sentence of the decision it states, a
  restatement that dropped part of its home; and the session's reading reverses that sentence. The
  proposal is a reversal of part of the head.
- **e. Its scope.** A reversal goes through `skill@knowledge-architect-decision-recording@reversal-check`
  and `skill@knowledge-architect-design`. The head's argument gives a case for it, so it is not
  settled by citing the head.
- **f. A better fix.** Rivals: the primer delivers the head's sentence as it stands, adding a
  reference is a touch, which leaves a record-wide conversion owing every head it passes; the
  proposal, which reverses the sentence for a citation the head's argument already covers; a
  reversal scoped to a change that adds citations across the record, the case the exemption's
  argument names. The fact that decides is a weighing of a conversion's cost against the repair the
  touch rule brings, which no record states. That is a design question.
- **g. Its route.** A design discussion; until then an issue.
- **h. The proposal.** Open an issue: a `design` issue in agent-skills, stating the divergence
  between the primer and `design@agent-skills@existing-heads-on-touch` and the three rivals.
- **Default**: open a `design` issue in agent-skills.
- **Outcome**: No need for an issue. Rephrase the primer to something like: "re wording a reference does not touch it. Adding or removing one does". The existing sentence was about rewording I believe, the case where the slug of the pointed head changed, and not any other case. 

## W3, with W1 of 2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow. The plan document that schedules an issue closes it in the commit that adds it

- **a. What they say.** This file: the spec cited the issue it scheduled by its reference, in three
  places, so deleting the issue in the spec's commit would dangle them; it closed the issue in the
  next commit. The fix proposed: the plan names the issue by its bare slug in backticks. The other
  file: the issue was a `question`; the session read the rule as covering a `todo` only, and the
  issue stayed open for 11 commits; the standing-state reviewer read it the other way. The fix
  proposed: "closes the issue, of whatever kind, whose work it schedules".
- **b. Whether it still applies.** The sentence is unchanged,
  `path@agent-skills@content/skills/planning/SKILL.md` lines 73-77: "**Work that is known but not
  designed is not a plan document.** It is a `todo` issue, or a `deferred` one if an event gates it
  … The plan document that schedules it closes that issue in the commit that adds the plan document".
  The bare-slug form exists: `primer@where-knowledge-goes` lets a record of a deliberation, a plan
  document included, name by its bare slug in backticks an entry its work removes.
- **c. Whether it is already recorded.** The scope of the sentence is
  `design@agent-skills@undesigned-work-is-an-issue`: "Work that is known but not designed is a
  `todo` or `deferred` issue … The plan document that schedules it closes the issue". No issue or
  `rejected` entry covers either finding.
- **d. Its kind.**
  - The kind of issue: not unclear. "That issue" has one antecedent, a `todo` or a `deferred` one,
    and the head says the same. The session's reading was the text's. A `question` closes as its
    own "What would close it" says. Widening the rule to every kind is a change of the head.
  - The naming: a lapse, of a form the primer provides; one instance, which cost one commit.
- **f. A better fix.** For the naming, a clause in the planning sentence pointing to the bare-slug
  rule, against no change; one instance with a one-commit cost is no recurrence, per the lapse rule
  of `skill@knowledge-architect-agent-configuration@admission-standard`. For the kind, no change:
  the widening would move a question's closing from its own condition to a plan's commit, against
  `design@agent-skills@undesigned-work-is-an-issue`, with no evidence that the question form fails.
- **h. The proposal.** No change for both.
- **Default**: no change: the rule's scope is a `todo` or `deferred` issue, as its head says; the
  bare-slug form exists in the primer.
- **Outcome**: accepted.

## W4. The review skill's warning about a stale agent definition does not reach a branch that edits the agent

- **a. What it says.** The routing reviewer dispatched before the merge ran on its definition from
  the session's start and missed a row the branch had added; it noticed by reading the source. The
  file calls it a lapse and proposes nothing.
- **b.** The warning, lines 90-96 of `path@agent-skills@content/skills/review/SKILL.md`, names "an
  edit or … an install" of the agent definition, so it reaches this case; the title's "does not
  reach" is not what the text says.
- **d. Its kind.** A lapse.
- **h. The proposal.** No change.
- **Default**: no change, a lapse of a clear instruction.
- **Outcome**: accepted.

## C1. No command lists a Component's design entries and `rejected` entries for a searcher

- **a. to c.** Recorded, as the file says: `issue@core@searcher-reports-are-written-and-checked-by-hand`,
  with a lead for the listing, and `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt`.
- **h. The proposal.** No change.
- **Default**: no change, already recorded.
- **Outcome**: accepted.

## C2. A placeholder span becomes a malformed reference when its head becomes a kind

- **a. What it says.** A span with an ellipsis for its segments, written before `rejected` was a
  kind, was reported as malformed once it was one. The finding was right. No proposal.
- **b.** The behaviour is the candidate rule's, in `path@core@src/entity.rs`: a span whose head is
  a kind and whose segments are short is malformed. The checked placeholder is angle brackets, per
  `primer@where-knowledge-goes`; an ellipsis is not one.
- **d. Its kind.** An observation; the checker behaves as designed.
- **h. The proposal.** No change.
- **Default**: no change, an observation.
- **Outcome**: accepted.

## The standing answers

- **a.** Three primer rules acted on as if absent, each caught before the merge: unchecked claims
  in three commit messages, the owner's words quoted in a head and a message, and a head describing
  the state before the change.
- **b.** The rules hold on main, `path@agent-skills@content/PRIMER.md` lines 153, 494-497 and 511.
- **d.** Lapses.
- **Default**: no change, lapses caught before the merge.
- **Outcome**: accepted.
