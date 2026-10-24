# 2026-10-20-thaum-pin-move-0-8-0-klarch-workflow

- **Received file**: 2026-10-20-thaum-pin-move-0-8-0-klarch-workflow.md, the workflow file of a
  session in thaum that moved its pin of the checker from 0.7.0 to 0.8.0, carried out the migration
  the 0.8.0 changelog asks for, partly by subagents, and merged.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`. Between the two, only the setup skill changed
  in the installed text, and not its step 5; the 0.8.0 section of CHANGELOG.md is unchanged.
- **Standing entries that bear on it**:
  - for W1, W2 and W4: `design@knowledge-architect@changelog-section-fixed-at-release`, since each
    blames a released Migration entry;
  - for W1 and W6: `design@agent-skills@losing-shape-test`,
    `design@agent-skills@rejected-citation-direction`, `design@core@orphan-rejected-entry`,
    `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head`,
    `issue@agent-skills@patching-an-installed-skill`;
  - for W4: `design@agent-skills@statement-names-an-act`,
    `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`;
  - for W5: `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text`,
    `design@agent-skills@undesigned-work-is-an-issue`, `design@agent-skills@audit-triggers`,
    `tripwire@agent-skills@axis-not-run-at-a-pin-move`, not met: the file does not say an axis was
    not run;
  - for the cluster under C1: `design@core@plain-word-lint`,
    `tripwire@core@plain-word-neither-pointer-nor-subject`,
    `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`,
    `design@core@finding-names-the-repair`, `design@agent-skills@plain-text-is-no-repair`,
    `design@core@section-homes-carry-slugs`;
  - for C3: `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
    `goal@core@records-reach-their-reader`;
  - for the standing answers: `tripwire@agent-skills@member-beyond-the-argument`,
    `tripwire@agent-skills@head-created-without-deliberation`,
    `design@agent-skills@no-routing-table`, `design@agent-skills@goals-change-through-goal-setting`.

## W1, with W6. An entry that lost to something other than a design head has no stated home

- **a. What it says.** The 0.8.0 Migration entry asks that each `rejected` entry be cited from the
  design heads whose argument covers it. Five entries of one Component had lost to something that is
  not a head: instructions of the project's root CLAUDE.md, a project reviewer's sweep, a decision of
  another project. The session first cited four from heads whose argument did not cover them; a
  reviewer found two of those to be bundles the backstop should have sent to the design skill. The
  repair used the last paragraph of the decision-recording skill's section on designs not kept, which is written for heads that
  leave. W6 is the project side: that project gives a decision about its agent workflow no head. The
  fix proposed: one sentence, an entry whose old winner is an instruction leaves the register and
  the line beside that instruction names the design and its reason; one whose old winner is an
  absence takes a head of absence.
- **b. Whether it still applies.** Yes. The Migration entry, CHANGELOG.md's 0.8.0 section: "then
  cite each entry, `rejected@<anchor>@<slug>`, from the design heads whose argument covers it. A new
  check reports an entry that no design entry cites." That section is released, and "A released
  section's content never changes", per `design@knowledge-architect@changelog-section-fixed-at-release`.
  The installed text that states the rule now,
  `skill@knowledge-architect-decision-recording@losing-alternatives`: "**or when no design head cites
  it any longer and no head of absence is written for it**, as when the heads that cited it leave the
  design home for a line beside the instruction they state: that line then names the design and the
  reason it was not kept". The rule covers an entry no head ever cited; the example names only heads
  that leave. The orphan finding's repair, in `path@core@src/check/references.rs`, names two routes:
  "cite it from the design head whose argument covers it, or from a head of absence that records the
  decision not to build it".
- **c. Whether it is already recorded.** In part. `design@agent-skills@losing-shape-test` covers
  the absence. `design@core@orphan-rejected-entry` restates the two-route repair.
  `issue@agent-skills@patching-an-installed-skill` records that project's setting aside of entry test
  2 for its agent-workflow decisions, the cause of W6. No entry records the missing route.
  `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head` fires when "a review or an audit
  finds a `rejected` entry whose only citing heads decide a question other than the one the design it
  records answered"; the reviewer's finding on the first citations reads close to it, but what it
  found was a repair in progress, rewritten before the merge, not an entry whose citing heads stand.
- **d. Its kind.** Unclear: the rule admits the move, and its one example and the checker's repair
  line both name other moves, so a session reaches it only through a reviewer.
- **e. Its scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Installed
  text, and the checker's repair line; the head on the orphan check restates that line.
- **f. A better fix.** Rivals: one sentence in the Migration entry, which is released and fixed; one
  sentence in that section of the decision-recording skill, the proposal, which reads as a new instruction where
  the rule already admits the case; the example widened to the case and the repair line given its
  third route, which changes no rule. The deciding fact: the rule's own words, "no design head cites
  it any longer and no head of absence is written for it", already cover an entry whose winner was an
  instruction. Instructions added: 0.
- **g. Its route.** An installed text edit under `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent", and a change of the checker's repair line under
  `skill@klarch-development`, with `design@core@orphan-rejected-entry`'s restatement brought along.
- **h. The proposal.** Handle now: the example of the leave sentence names an entry whose winner is
  an instruction that no head states, the line beside that instruction naming the design and its
  reason; the orphan finding's repair names that route as its third. The tripwire did not fire.
- **Default**: handle now, as h says.
- **Outcome**: accepted. the root of the problem , though, is that theam does not record design  about the local agentic workflow, due to outdated reasons. I'm gonna change that soon, and things will work smoothly once again

## W2. Dropping the "lost to" clause removed the antecedent of later phrases

- **a. What it says.** The Migration entry says an entry's "lost to" clause is dropped; about a
  dozen entries then used "the winner" and like phrases with no antecedent. The repair named the head
  inside the reason, which the skill allows. The fix proposed: the Migration entry says so.
- **b. Whether it still applies.** The Migration entry is released and fixed. The installed text,
  `skill@knowledge-architect-decision-recording@losing-alternatives`: "It names no winner as such,
  and its reason may name a head where the reason needs one", which is the repair the session used.
- **c. Whether it is already recorded.** `design@agent-skills@rejected-citation-direction`: "an entry
  that names no winner drops it", on the owner's judgement that the framing of a winner and a loser
  is wrong; `rejected@agent-skills@entry-names-the-decision-it-lost-to`.
- **d. Its kind.** A defect of a one-time migration text, which cannot change, and whose act every
  project that moved to 0.8.0 has done. A new entry is written with no "lost to" clause, so the case
  does not recur.
- **h. The proposal.** No change: the blamed text is fixed, and the live text names the repair.
- **Default**: no change, as h says.
- **Outcome**: accepted.

## W3. "Where its argument meets that design" placed citations between two sentences of one argument

- **a. What it says.** Subagents inserted a sentence "The design not kept is …" after the sentence
  where a head's argument met the design; in about ten heads the next sentence opened "Then", "That",
  "This" and now read as a claim about the rejected design, in one case reversing the meaning. The fix
  proposed: a citation added to an existing head sits inside the sentence it qualifies or closes its
  paragraph.
- **b. Whether it still applies.** Yes, the same at the tag and on main:
  `skill@knowledge-architect-decision-recording@losing-alternatives`, "**each design head whose
  argument covers the design cites the entry**, where its argument meets that design, or in a closing
  sentence of its body." `design@agent-skills@rejected-citation-direction` says the same.
- **c. Whether it is already recorded.** As the decision, not as a defect.
- **d. Its kind.** Unclear: "where its argument meets that design" admits a separate sentence placed
  between two sentences of one argument, which changes the argument's meaning.
- **e. Its scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`; installed
  text, and the head that states the placement.
- **f. A better fix.** Rivals: the proposal; no change, since a reviewer reads a broken antecedent,
  which here cost two reviewers' findings over ten heads. The rewording keeps both placements the
  head names and rules out the third reading only; it adds and removes no decision, so the head's
  body is brought along by a rewording. Instructions added: 0.
- **g. Its route.** An installed text edit, with the head's body reworded in the same commit under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Handle now: "cites the entry inside the sentence where its argument meets
  that design, or in a closing sentence of its body", in the skill and in the head.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W4. A stricter rule of the project's root CLAUDE.md deleted as a copy of the primer's

- **a. What it says.** The Migration entry tells a root CLAUDE.md that restates the primer's rules
  to delete its copy; the session deleted two rules that were stricter than the primer's, and a
  reviewer restored them. The fix proposed: a statement that differs from the primer's is not a copy.
- **b. Whether it still applies.** The Migration sentence is released and fixed: "A project's root
  CLAUDE.md that restates any of these deletes its copy". The live installed text,
  `skill@knowledge-architect-agent-configuration@admission-standard`: "**A duplicate inside one text
  fails while its twin stays.** A text is read whole once it is loaded, so a second statement of the
  same act adds none". A stricter rule is not the same act, so the live text already excludes it.
- **c. Whether it is already recorded.** `design@agent-skills@statement-names-an-act`, the
  duplicate rule. `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, case 1, is about a
  root restatement that drifts, and this is a deletion that went too wide; it neither closes nor
  confirms that case.
- **d. Its kind.** A defect of the released Migration entry's wording, which is fixed; the live
  instruction is clear.
- **h. The proposal.** No change: the live need test states the act-based test the session needed.
- **Default**: no change, as h says.
- **Outcome**: none.

## W5. The audits a pin move owes after its merge have no home until they run

- **a. What it says.** Step 5 of moving the pin runs each owed audit in a branch of its own after
  the pin move merges; until then the obligation, and the items left to the audits, live only in the
  pin move's commit message. Two reviewers flagged it; the session opened one `todo` issue per owed
  audit. The fix proposed: step 5 opens them, in the root Component, closed by the audit's commit.
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-setup@moving-the-pin`, step 5, the
  same at the tag and on main: "For each axis that step 2 listed, run it under
  `skill@knowledge-architect-project-audit`, in a branch of its own started from the main branch once
  the pin move has merged into it, so that the audit's range holds the audit's work alone." No step
  opens an issue.
- **c. Whether it is already recorded.** In part.
  `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` is the same gap for the
  move of an adopting project's existing documents, with the same remedy, a `todo` issue carrying the
  closing audit. `design@agent-skills@undesigned-work-is-an-issue` is the ground the session used.
- **d. Its kind.** Missing: work known to be owed, held by no register between two branches, seen in
  real use and flagged by two reviewers.
- **e. Its scope.** `goal@knowledge-architect@any-project-can-adopt-it`; installed text, the setup
  skill.
- **f. A better fix.** Rivals: one `todo` issue per owed audit opened in the pin move's branch, the
  proposal; the audits run in the pin move's branch, which step 5's reason rules out; no change,
  which leaves the obligation in a commit message no search lists. One fix for both moves would let
  the other issue close too, if the existing-documents move takes the same sentence. Instructions
  added: 1.
- **g. Its route.** An installed text edit of the setup skill.
- **h. The proposal.** Handle now: step 5 opens, in the pin move's branch, one `todo` issue per owed
  axis in the root Component, which the audit's commit closes.
  `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` stays open for its own
  move.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C1, with C3 of 2026-10-20-thaum-retrospective-intake-klarch-workflow, C1 of 2026-10-24-thaum-driven-testing-driven-tests-klarch-workflow and C3 of 2026-10-24-thaum-driven-testing-short-deck-soak-klarch-workflow. The plain-word lint's repair branch does not fit what the slug names

- **a. What they say.** The plain-word finding's repair ends "where the word is meant as an ordinary
  term, the entry's slug names a subject: rename the entry to state its decision". Four sessions met
  it on slugs that state no decision or must name a subject:
  - an issue entry's id, here; the session reworded the owner's goal sentence instead, and the owner
    pointed out that renaming the issue was the simpler move;
  - a skill section's slug used as an ordinary phrase, three times, where the agent-configuration
    skill requires that slug to name the section's subject; this intake's own draft of this file
    met it twice, on the slug of a section of the decision-recording skill, and was reworded;
  - a slice's id in a commit subject, and a milestone's name in a commit message, where renaming a
    plan document is no repair.

  The fixes proposed: for an entry outside `design`, "rename the entry so its slug is not an ordinary
  word"; for a section slug, reword the prose; for a plan document's id, name it in words or write
  the reference.
- **b. Whether they still apply.** Yes. `path@core@src/check/references.rs`, the same at the tag and
  on main, appends the branch to every finding of the lint, whatever the kind: "; where the word is
  meant as an ordinary term, the entry's slug names a subject: rename the entry to state its
  decision". No test asserts the branch's text. Measured by the checking agents with the binary at
  the analysed commit: a project skill's section slug gives the branch unchanged; a slug written as
  `#<id>` gives no finding. `skill@knowledge-architect-agent-configuration@shaping-a-skill`: "the id
  naming the section's subject". The planning skill names a slice and an item outside its plan "in
  plain text with a hash sign, as #<id>".
- **c. Whether they are already recorded.** `design@core@plain-word-lint`: "Its repair names the
  reference, and adds that where the word is meant as an ordinary term, the entry's slug names a
  subject: the entry is renamed to state its decision, as `primer@design-heads` asks of a slug", and
  "**The rule is the owner's**", given with the owner's view that such a word "probably shows a slug
  wrongly chosen". That view, and `primer@design-heads`, are about a slug that should state a
  decision: a design head's. `tripwire@core@plain-word-neither-pointer-nor-subject` does not fire on
  any of the four: each word was a pointer written in words or a subject slug, and its re-entry, an
  intake reporting an upgrade, is this one. `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`
  records the same class of defect in two other findings.
- **d. Their kind.** Wrong: following the branch produced a correction of the owner's, a lint repair
  that edited a goal sentence outside goal-setting, and three repairs that went against the
  agent-configuration skill's rule on section slugs. Two installed texts leave no move satisfying
  both for a section slug.
- **e. Their scope.** `goal@knowledge-architect@adoption-is-easy`, for repair texts;
  `goal@agent-skills@no-contradiction-in-workflow`. The checker's message, and the head whose
  repair sentence it states, which rests on the owner's words.
- **f. A better fix.** Rivals: the branch given by the kind of what the slug names: for a design
  head, rename it to state its decision, as now; for any other entry, rename it so its slug is not an
  ordinary word, which is what the owner chose for the issue; for a section of a skill, an agent, the
  primer or the root CLAUDE.md, reword the prose; for a plan document or a plan item, write the
  reference, or name it as #<id>, the form the planning skill gives and the lint does not report. The
  proposal's "name it in words" for a plan id is a repair that offers an unchecked form, which
  `design@agent-skills@plain-text-is-no-repair` refuses, so the #<id> form replaces it. Rival: the
  branch dropped for every kind but design, the shortest message, which leaves the issue case without
  the repair the owner chose. One fix for the cluster, and it meets the class of the open issue.
  Instructions added: 0; a message changes.
- **g. Their route.** A change of the checker's message under `skill@klarch-development`. Since the
  head's repair sentence rests on the owner's words, the narrowing goes to the owner as one proposal,
  per `primer@design-heads`, "A new member of a head is routed by the head's ground": the owner's view
  that a plain word shows a slug wrongly chosen was given of decision slugs, so whether it holds for
  the other kinds is the owner's to say.
- **h. The proposal.** Handle now, on the owner's word: the repair branch by kind, as the first rival
  states it, and the head's repair sentence brought to it; the tripwire did not fire.
- **Default**: handle now, as h says.
- **Outcome**: now. we keep the same strong opinion: use slugs that have no reason to be used as ordinary words. this is the only way the checker can lint sreliably for wrong usage.

## C2. In a commit message, a renamed one-segment id is judged unevenly

- **a. What it says.** The branch renamed ids of one segment, as the migration requires. In a commit
  message, the old one-segment section slugs of the root CLAUDE.md, written bare in backticks, were
  reported against the first parent; the old slugs of a milestone, a spec, argument items and eight
  issue entries, written the same way, were not.
- **b. Whether it still applies.** The code explains it. `path@core@src/cli/history.rs` judges a
  message's id lints against the commit's table and the first parent's, both built by the tip's
  checker. In `path@core@src/entity.rs`, a one-segment heading slug stays defined with a finding,
  "It is still defined, so a reference to it resolves", for heading registers, plan items and the
  sections of skills, agents, the primer and the root CLAUDE.md; a one-segment issue file, milestone
  or spec is refused before it is defined; a plan item cited from a commit message resolves outside
  its plan. The primer's rule keys on whether "its tree does not hold it, nor, for a commit message,
  its first parent's"; for a one-segment issue file the parent tree holds the file and the tip's
  checker defines no entry for it, so "holds" and "defines" part for exactly these ids.
- **c. Whether it is already recorded.** No entry records it. `design@core@backticked-bare-id` and
  `design@core@two-segment-ids` are silent on ids the grammar refuses.
  `tripwire@core@candidate-rule-silence` does not apply: the slugs were not meant as references.
- **d. Its kind.** A blind spot that exists only for ids of one segment, which no tree defines as an
  entry under 0.8.0 once the migration has run.
- **f. A better fix.** Rivals: the proposal, both kinds alike and a Migration sentence, the latter on
  a released section; no change, since the case ends with the migration in each project, and every
  project that moved to 0.8.0 has run it.
- **h. The proposal.** No change: the condition ends with the migration.
- **Default**: no change, as h says.
- **Outcome**: accepted.

## C3. `show` lists where an entry is cited, not which entry cites it

- **a. What it says.** To find which head cites a `rejected` entry, the session and three reviewers
  wrote scripts mapping a line to its enclosing level-three heading. The fix proposed: `show` prints
  the enclosing entry's reference beside each location.
- **b. Whether it still applies.** Yes. `cargo klarch show` on a `rejected` entry prints
  "referenced at:" and one file and line per citation; `show --help` lists no option. The orphan
  check already maps each citation to the design entry whose section holds it, per
  `design@core@orphan-rejected-entry`.
- **c. Whether it is already recorded.** In substance:
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` says "`show <ref>` prints
  the entry, then `referenced at:` and one `<file>:<line>` per reference … a line number has to be
  mapped by hand to the heading above it", and its close names a listing of the issues and tripwires
  that cite a set of entries.
- **d. Its kind.** Missing: an option, seen in real use by a session and three reviewers.
- **e. Its scope.** `goal@core@records-reach-their-reader`; the checker.
- **f. A better fix.** Rivals: the enclosing entry printed by `show`, the proposal, which covers a
  design head, an issue and a tripwire alike; the open issue's listing, which serves a set of entries
  and the standing ones only. The first is narrower and serves the open issue's case as well.
- **g. Its route.** An issue only: Rust work not for this session.
- **h. The proposal.** Handle now: the open issue gains this case and the shape, `show` printing the
  enclosing entry's reference beside each location.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C4. The crate documentation does not list the new observation variants

- **a.** The crate-level documentation of the core lists the observations as ending "retired forms,
  bare names and links"; 0.8.0 added `UnquotedReference` and `PlainWord` to `document::Observation`.
- **b.** Still applies: `path@core@src/lib.rs` on main, the same at the tag; the enum's variants are
  in `path@core@src/scan.rs`.
- **c.** Not recorded.
- **d.** Wrong, provable by reading. Its fix changes no behaviour, no decision and no test outcome.
- **e.** `goal@core@projects-add-their-own-checks`, "a library API documented with its contract".
- **h. The proposal.** Handle now: the two variants named in the list, in a commit of its own.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## The standing answers

- **a. What it says.** Three rules acted on as if absent, each found by a reviewer before the merge:
  a commit message claim checked by nobody, false at 13 sites; sentences added to two heads so that
  they would cite `rejected` entries their argument did not cover, without the design skill; a lint
  repair that edited a goal sentence. And one project skill's addition missed: the project's pin-move
  procedure says a move that changes Rust source follows its development skill, and the session
  edited Rust doc comments without reading it.
- **b.** The rules stand: `primer@verify-before-relying`; the backstop of `primer@design-heads`;
  `design@agent-skills@goals-change-through-goal-setting`, "An agent never edits a goal outside it".
- **c.** The backstop lapse recurs in this intake: here; in the init workflow file, two rules added
  to a head under a relocation said to add no decision; in the driven-tests workflow file, a brief
  that added a member a head lists by case without reading the head.
  `tripwire@agent-skills@member-beyond-the-argument` fires when "the owner, or a review, names one
  member recorded directly, as covered by its head's argument, that the argument does not cover".
  Here a review named two such members, in that project's tree, before its merge.
  `tripwire@agent-skills@head-created-without-deliberation` fires on "a commit" with no deliberation
  in its message or a plan document; each case here was repaired before its merge, so no commit that
  reached a main branch is shown to lack it. The missed project skill is the watch point of
  `design@agent-skills@no-routing-table`, through `design@agent-skills@premortem-as-watch-points`: one
  instance against its premise that the harness lists every skill with its description.
- **d.** Lapses. The goal edit is caused by C1's repair branch, analysed above. On the tripwires:
  the letter of `tripwire@agent-skills@member-beyond-the-argument` is met, by a review, in a project
  that follows the installed text; the doubt is that each case was caught by the review the
  tripwire's re-entry names, before any merge, which is the guard working. The second tripwire is
  not met.
- **h. The proposal.** `tripwire@agent-skills@member-beyond-the-argument` fired: it becomes an issue
  that reopens `design@agent-skills@extension-follows-the-ground`, with the rival its response names,
  per `skill@knowledge-architect-issue-tracking`; its design is not this session's work.
  `tripwire@agent-skills@head-created-without-deliberation` not fired. No change for the missed
  project skill: one instance, against a head whose premise a second would test. The claims are
  instances of the recurrence analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`.
- **Default**: as h says. Whether the tripwire fired is the owner's to rule.
- **Outcome**: accepted as proposed.
