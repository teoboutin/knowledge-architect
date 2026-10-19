# 2026-10-19-thaum-boundary-fuzzing-design-klarch-workflow

- **Received file**: 2026-10-19-thaum-boundary-fuzzing-design-klarch-workflow.md, the workflow file
  of a design discussion in thaum on the full path, nine owner rounds, a milestone document and five
  slice specs, the reviews, one branch merged.
- **Version used**: the published checker, 0.7.0, read at the tag `v0.7.0`.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**:
  - for W2: `tripwire@agent-skills@design-conformance-findings-needless`, not met: the re-reading
    found four items that were needed;
  - for W3: `tripwire@agent-skills@design-entry-search-misses`, not met, since the search did not
    exist at 0.7.0; `tripwire@agent-skills@search-missed-before-the-work`, not met, since the missed
    entry was a design head, not a standing entry;
  - for C1: `tripwire@agent-skills@plain-text-pointer-found`, judged in
    `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow.md`;
  - for C2: `spec@plans@path-quickfixes`;
  - for C3: `the-retired-slug-finding-names-a-design-repair-for-every-register`, now `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`, a
    neighbour;
  - for C4: `rejected@core@check-refusing-a-pipe`;
  - for the standing answers: `tripwire@agent-skills@attribution-loses-the-scope`, judged in
    `path@knowledge-architect@docs/retrospective-reports/2026-10-15-knowledge-architect-primer-relocation-klarch-workflow.md`.

## W1. An acceptance criterion's observable reaches the owner unchecked against the code

- **a. What it says.** Step 7 of `skill@knowledge-architect-design@discussion-loop` asks the owner
  whether each acceptance criterion is applied. The owner ruled one applied whose observable named an
  enum private to its crate, and another that named a profiler the project's record rules out.
  Reviewers found both; their observables were reworded. The fix proposed: one sentence in step 7,
  the observable is checked against the code and the record before it is put to the owner.
- **b. Whether it still applies.** In part. Step 7 is unchanged,
  `path@agent-skills@content/skills/design/SKILL.md` lines 587-594. Two texts weaken the claim. The
  owner's word does not rule on the observable's wording: "The owner's word rules on whether a cause
  is watched, not on the wording … the same holds for an acceptance criterion's observable", lines
  612-614. And after 0.7.0 the primer asks the check generally: "A checkable claim is checked
  before it is written down" and "The same checks apply to an answer given to the owner in the
  conversation", in `primer@verify-before-relying`, added by `658c44a`, "[agent-skills] The primer
  states the checker's model, the verification rules and the working-prose rules, and the skills
  point to them". At 0.7.0 the primer held only the check before relying on a claim.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** At 0.7.0, missing; on main, a case of the primer's rule on a claim written or
  given to the owner.
- **f. A better fix.** Rivals: the proposal, a sentence in step 7; a pointer from step 7 to the
  primer's subsection; no change. No change wins: the primer, read by every session, now covers it,
  and a sentence for one kind of claim is a narrow rule where a general one decides.
- **h. The proposal.** No change.
- **Default**: no change, covered on main by `primer@verify-before-relying`.
- **Outcome**: accepted.

## W2. Which review a decided shape changed by the owner during review owes

- **a. What it says.** `skill@knowledge-architect-planning@plan-reviews`: "It is read again after a
  design session converges on a change to its decided design". The owner changed a decided shape
  after one question, one measurement and one ruling, with no design session. The text does not say
  whether that counts, nor which axes. The re-reading the session sent found four items. The fix
  proposed: "after its decided design changes, by a converged design session or by a ruling of the
  owner, on the axes whose subject the change touches, and then the transcript axis".
- **b. Whether it still applies.** Yes, `path@agent-skills@content/skills/planning/SKILL.md` lines
  486-488, unchanged.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Unclear: the trigger names one way a decided design changes, and the case is
  another way to the same change.
- **f. A better fix.** The trigger's half is a rewording: the plan document is read again after its
  decided design changes, whoever changes it. The axes' half restates what the review skill's table
  and invariants already decide for any review, which axes and the transcript axis last; restating
  it here makes a second home. Instructions added: 0.
- **g. Its route.** Installed text edit.
- **h. The proposal.** Handle now: "It is read again after its decided design changes, by a design
  session that converges or by a ruling of the owner, as at a load-bearing gap of a design audit."
- **Default**: handle now, the trigger's half only.
- **Outcome**: accepted.

## W3. The standing-entry search does not reach design heads

- **a. What it says.** A touched Component's design home was read by its headings only; a head
  there was found only by the cold-implementer review. No proposal; a searcher for design heads
  "would close it" if the pattern recurs.
- **b. Whether it still applies.** Repaired after 0.7.0: `4612058`, "[agent-skills] A design head
  cites the `rejected` entries its argument covers, and a design discussion sends a design-entry
  searcher at its grounding", sends `agent@knowledge-architect-design-entry-searcher` at the
  grounding. It ships in the next release.
- **d. Its kind.** An observation, repaired; with W1 of
  2026-10-19-knowledge-architect-rejected-register-klarch-workflow, two sessions of two projects
  met the same failure.
- **h. The proposal.** No change.
- **Default**: no change, repaired by `4612058`.
- **Outcome**: accepted.

## C1. No checked form names, in a commit message, a path the work will create

- **a. What it says.** A commit message described a directory the plan will create. Backticked, it
  was reported as a path naming no anchor; the session wrote it bare, with no issue beside it. The
  fix proposed: accept the planned form in a message whose commit holds a plan document with the
  same planned path, or report the bare form with a suggestion to reword.
- **b. Whether it still applies.** In part. The planned form is refused outside the plans directory,
  per `design@core@planned-path-form`. But a checked form existed: `primer@where-knowledge-goes`
  names "`path@elsewhere@<path>` for a path the tree does not hold", and `commits` accepts it for a
  path its commit's tree does not hold. The finding's repair, in `path@core@src/check/references.rs`
  lines 225-229, names the same form "for a path outside this tree", which does not say it serves a
  path the work will create inside the tree.
- **c. Whether it is already recorded.** The proposal's first form lost:
  `rejected@core@planned-path-in-every-document`, on the ground that a hypothetical file is served by
  the escape anchor.
- **d. Its kind.** A lapse, the escape anchor being available; helped by a repair line whose wording
  differs from the primer's.
- **f. A better fix.** Rivals: the proposal, which reopens a design that lost; the repair line
  reworded to the primer's "for a path the tree does not hold"; no change. The rewording removes the
  difference that made the available form look inapplicable, at the cost of one string and the tests
  that pin it.
- **g. Its route.** A change to the Rust source, under `skill@klarch-development`.
- **h. The proposal.** Handle now: the repair line reads "`path@elsewhere@<path>` for a path this
  tree does not hold".
- **Default**: handle now, as h says.
- **Outcome**: See my other ruling. Something not yet created in plain words. Or for this case, maybe the checker lints against plain word paths ? Then, proposed solution is right.

## C2. A glob in backticks is reported as a path naming no anchor

- **a. What it says.** A backticked glob over a directory was reported as shaped like a path and
  naming no anchor; no form fits a glob. The fix proposed: a span whose last component is a star is
  not a path, or the repair line says what to do.
- **b. Whether it still applies.** Yes: the path shape of `path@core@src/scan.rs` accepts a star,
  and the shapes the lint exempts, a space, an angle bracket, a colon, a single segment, do not
  include a glob. A placeholder in angle brackets is silent, but the finding does not say so.
- **c. Whether it is already recorded.** The second form: `spec@plans@path-quickfixes`, on hold,
  adds "or a placeholder for a pattern" to this finding's repair. The first form is recorded
  nowhere.
- **d. Its kind.** A message that does not name the repair a writer needs.
- **f. A better fix.** The spec's repair wording, against an exemption for a trailing star, which
  would silence a path whose last segment is a literal star and no glob. The spec carries it.
- **h. The proposal.** No change: the spec carries it.
- **Default**: no change, scheduled by `spec@plans@path-quickfixes`.
- **Outcome**: accepted.

## C3. A heading anchor written in prose gets a repair that does not name the reference form

- **a. What it says.** In a plan document, a slug with two hash signs, meant to point at a section of
  the project's root instructions, was reported as written outside the item sections, with the
  repair "move it there, or delete it". The intended form was an `instructions` reference, which the
  finding does not name though the slug matched a section.
- **b. Whether it still applies.** Yes: `path@core@src/entity.rs` lines 1255-1262, unchanged,
  "an item of a plan is defined at the end of a level-3 heading under one of its sections …; move
  it there, or delete it", with no lookup against the other kinds' slugs.
- **c. Whether it is already recorded.** A neighbour:
  `the-retired-slug-finding-names-a-design-repair-for-every-register`, now `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`, asks the same of the
  retired single-hash form, a repair naming the kind the id resolves to. This finding, on a
  misplaced definition, is not in it.
- **d. Its kind.** A finding whose repair does not name the one the writer needs, against
  `design@core@finding-names-the-repair`.
- **f. A better fix.** Rivals: widen the neighbour issue to the misplaced-definition findings, which
  share the question; a new issue; no change. Widening keeps one design for one question.
- **h. The proposal.** Handle now: widen
  `the-retired-slug-finding-names-a-design-repair-for-every-register`, now `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`, to a misplaced
  definition whose slug another kind defines.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C4. `commits` gives no protection against a verdict read through a pipe

- **a. What it says.** The session chained a push on a `commits` verdict read through a pipe; the
  check failed and the push ran. The fix proposed: an option under which `check` and `commits`
  refuse a pipe on stdout outside CI.
- **b. Whether it still applies.** The behaviour is unchanged. The primer at 0.7.0 did not say it;
  since `658c44a`, `primer@checker-model` says "Never chain a command on a verdict that went through
  a pipe".
- **c. Whether it is already recorded.** Yes, as a design that lost:
  `rejected@core@check-refusing-a-pipe`, built and reviewed, "it cannot stop that chain: a process
  inside a pipeline does not set the pipeline's status".
- **h. The proposal.** No change.
- **Default**: no change: the proposal lost, per `rejected@core@check-refusing-a-pipe`, and the
  primer now delivers the rule.
- **Outcome**: accepted.

## The standing answers

- **a.** Three primer rules acted on as if absent, each caught before the merge: four defaults
  already labelled D1 to D4 put to the owner as Q1 to Q4, against `primer@labelled-items`; three
  attributions narrower or firmer than the owner's words; a closed entry named by its bare slug while
  the parent commit held it. One path written bare for lack of a form, C1.
- **b.** The quoted rules hold at the tag.
- **d.** Lapses. The attributions are judged in the first file's last section; the bare path in the
  tripwire section named above.
- **Default**: no change, lapses caught before the merge.
- **Outcome**: accepted.
