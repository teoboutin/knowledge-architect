# The installed workflow loads its skills on its own: the design skill's trigger, the primer's rule on loading skills, and the search after the first challenge

## Status and audience

This spec is the plan document of one branch, which builds what a design
discussion converged on. It leaves when its work lands. Where it and a design home disagree on a
point "What is already decided" does not list as reversed or rewritten, the design home wins. Every
name it uses is defined in it or exists in the tree. Where the owner's word is awaited,
`skill@knowledge-architect-planning@working-a-slice` says what waits.

The discussion spans one session. It begins at the owner's message opening "I think the design
skill is not triggering reliably without what's in my personal ~/.claude/CLAUDE.md". The goals the
discussion set are already written, in the branch's first commit, "[goals] The installed text works
in any environment and holds only the instructions it needs, and the owner designs with agents as
counterparts".

## How the work is done

By `skill@knowledge-architect-planning`, whose section `skill@knowledge-architect-planning@working-a-slice`
gives the work of a spec its points.

## Names

| name | what it names |
| --- | --- |
| the primer | `path@agent-skills@content/PRIMER.md`, installed as `path@agent-config@knowledge-architect/PRIMER.md` |
| the design skill | `path@agent-skills@content/skills/design/SKILL.md`, installed as `path@agent-config@skills/knowledge-architect-design/SKILL.md` |
| the description | the `description:` line of the design skill's frontmatter, which the harness lists to every session |
| the list | the primer's section `primer@installed-skills`, one bullet per installed skill with its moments |
| the list's design entry | the bullet of that list for `skill@knowledge-architect-design` |
| bounded work's paragraph | the first paragraph of `skill@knowledge-architect-design@bounded-work` |
| loop step 1 | the first numbered step of `skill@knowledge-architect-design@discussion-loop`, "Ground first" |
| the search | the record search: `cargo klarch worklist` and the agents of `agent@knowledge-architect-record-searcher` |
| the work statement | what the dispatching session gives each searcher as "the work" |
| the condition | the sentence of #settled-means-recorded, stated in Decided design |
| the opening paragraph | the paragraph of #skill-load-rule-in-primer and #installed-skill-takes-precedence that opens the list |
| a cloud session | a Claude Code session run in the cloud, where no user-level `CLAUDE.md`, setting or plugin of the owner's machine is present |

## What the work is

**What exists today at each site:**

- **The description** names six moments. The fifth is "before starting a requested change whose
  design is not settled, to ground it and to find out whether it is bounded work that needs only one
  proposal, built on its default".
- **The list's design entry** reads "a design question has an open solution space; a requested
  change whose design is not settled, to ground it and find out whether it is bounded work; …". The
  list has no sentence before its first bullet.
- **Bounded work's paragraph** reads "So a requested change whose design is not settled starts here,
  at loop step 1, the search for standing entries included."
- **Loop step 1** reads the goals, design and `rejected` homes and the READMEs, and says "A search of
  the whole record runs beside that reading, with the question as the work and, as seeds, the
  decisions read so far that the question rests on or may change, never a goal". No proposal is
  drafted before the search.
- **`path@agent-skills@README.md`** restates the design skill's moment: "grounds a design question
  or a requested change whose design is not settled".
- **This repository's root `CLAUDE.md`**, in `instructions@repository-skills`, says "Read the
  matching skill before doing that kind of work."

**What the discussion established about the failure.** These facts were read from the two cloud
sessions' transcript files, which the owner supplied, by a script over their JSON lines.

- Both sessions' harness listed every installed skill with its whole description. Neither held the
  superpowers plugin: 0 matches of "superpowers" in either file.
- PR 24's session invoked no skill through the harness's Skill tool. It read sections of four
  skills with `grep` and `sed`, and the review skill whole with `cat`. It wrote into a design home
  without reading the decision-recording skill, and grepped the design skill once, during its
  review.
- PR 25's session invoked four skills at their events. The design skill was never invoked. The
  decision-recording skill was invoked at the review repair, after the decision it records had been
  agreed, and before any head was written.
- On the owner's machine, the superpowers plugin's session-start hook and the owner's user-level
  `CLAUDE.md` gave every request to build something a load of the design skill. This is an
  assumption from the hook's text and the file's text; it was not measured in a session.

**Outside the work:**

- A full search before a fix. That question stays in
  `issue@agent-skills@a-fix-is-grounded-by-no-record-search`, whose trigger is the owner's future
  goal on development work. This is default D1.
- The owner's environment: uninstalling the superpowers plugin, and the user-level `CLAUDE.md`. They
  are the owner's files, outside the repository. The owner keeps the designing-together plugin.
- The bounded path's three conditions, and whether it is ever reached. That question stays in
  `issue@agent-skills@the-open-space-test-may-admit-every-problem`.
- How a tripwire on agent behaviour reaches this repository from a consumer's sessions. That stays in
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`.

## What is already decided

**The decisions the design rests on**, not argued again:

- `design@agent-skills@design-home-write-loads-recording`: a trigger that depends on a test the
  skill holds fires only in a session that already applies the test. #settled-means-recorded
  applies that argument to the design skill.
- `design@agent-skills@retro-trigger`: a moment named by an event can be followed by any agent.
- `design@agent-skills@no-external-handoff`: the installed text names no skill outside the set,
  which the opening paragraph respects.
- `design@agent-skills@primer-limit`: the primer holds what every session needs and no skill
  delivers at its moment. The opening paragraph is a new member, which its argument covers.
- `design@agent-skills@restatement-size-test`: the condition is restated in three texts, one
  sentence each.
- `design@agent-skills@behaviour-additions-need-real-use` and
  `design@agent-skills@synthetic-evidence-not-built`: the evidence is the two cloud sessions, and no
  run is built to test the new text.
- `design@agent-skills@in-change-path`: a requested change that is not bounded, and whose work has
  not started, takes the full path, which is why this spec exists.

**The decisions the work rewrites**, each with every text `cargo klarch show` lists as citing it:

- `design@agent-skills@bounded-path-in-design`. Its last sentence, "The skill's description reaches
  a session before it starts a requested change whose design is not settled", becomes the condition.
  Citing texts: `path@agent-skills@docs/design/record-search.md`, which cites it for the search's
  occasions, judged at step 5 and unchanged; `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`
  and `issue@agent-skills@the-open-space-test-may-admit-every-problem`, which cite the bounded path,
  not its trigger, judged at step 5 and unchanged; and three lines of
  `register@agent-skills@tripwire`, the tripwires
  `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` and
  `tripwire@agent-skills@a-shortcut-decision-earns-a-head`, read at the review before the merge by the standing-state axis.
- `design@agent-skills@record-searched-before-the-work`. Its first sentence, "with the question as
  the work", becomes the order of #search-after-first-challenge. Citing texts: two lines of
  `path@agent-skills@docs/design/project-configuration.md` and two of
  `path@agent-skills@docs/design/reviews.md`, which cite where the search runs, not its work
  statement, judged at step 5; `issue@agent-skills@a-fix-is-grounded-by-no-record-search` and
  `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`, unchanged in subject,
  judged at step 5; and two lines of `register@agent-skills@tripwire`, read by the
  standing-state axis.

**Each entry the search graded**, with its outcome. The search ran with the seeds
`design@agent-skills@bounded-path-in-design`, `design@agent-skills@new-or-reshaped-head-needs-design`
and `design@agent-skills@design-home-write-loads-recording`, and graded 59 entries `bears` and 55
`may-bear`. One goal it graded, on needed instructions, has since been renamed
`goal@agent-skills@only-needed-instructions`; it is listed under that name.

Graded `bears`:

| entry | outcome |
| --- | --- |
| `design@agent-skills@primer-limit` | its argument admits the opening paragraph; the harvest names it among the members |
| `design@agent-skills@code-change-reads-its-record` | unchanged: a fix keeps its reading of its code's record (D1) |
| `design@agent-skills@owner-word-holds-as-far-as-its-premise` | unchanged; the condition sends an owner's proposal to the skill that checks its premises |
| `design@agent-skills@one-home-for-head-rules` | unchanged: the backstop is not edited |
| `design@agent-skills@restatement-size-test` | bounds the three restatements of the condition to one sentence each |
| `design@agent-skills@context-cost-of-agent-text` | the opening paragraph is read by every session, which applies it at each of its moments |
| `design@agent-skills@no-routing-table` | unchanged: the opening paragraph maps no project skill to an installed one, it says how a skill that adds to one is loaded |
| `goal@agent-skills@only-needed-instructions` | met: each addition answers a failure read in a real session or a gap anyone can see by reading |
| `goal@agent-skills@installed-text-works-anywhere` | the goal the work meets |
| `goal@agent-skills@no-contradiction-in-workflow` | AC1 and AC2 check the restatements agree |
| `goal@knowledge-architect@adoption-is-easy` | served: the workflow needs no instruction outside it |
| `goal@knowledge-architect@agents-get-a-complete-workflow` | served: the design activity is reached from the installed text |
| `goal@knowledge-architect@agents-work-without-drift` | served: proposals are grounded before they are built |
| `goal@knowledge-architect@any-project-can-adopt-it` | served, through `goal@agent-skills@installed-text-works-anywhere` |
| `goal@knowledge-architect@the-owner-decides` | the condition keeps the owner's rulings with the owner and sends the owner's proposals to argument |
| `goal@knowledge-architect@the-owner-s-word-is-verified` | served: an owner's proposal is grounded and challenged |
| `issue@agent-skills@a-fix-is-grounded-by-no-record-search` | touched and left open, per D1 |
| `design@knowledge-architect@versioning-policy` | the change is a skill change: a patch |
| `design@knowledge-architect@changelog-entries` | a Workflow entry, written at step 4 |
| `design@knowledge-architect@the-branch-writes-its-changelog-entries` | the entry is written on this branch |
| `goal@knowledge-architect@design-is-recorded-with-its-arguments` | served |
| `goal@agent-skills@one-skill-per-activity` | served: the activity is carried out from its skill |
| `design@agent-skills@bounded-path-in-design` | rewritten at step 5 |
| `design@agent-skills@frontier-tier-only` | the description keeps its frontier-tier sentence |
| `design@agent-skills@new-or-reshaped-head-needs-design` | unchanged: its three delivering texts stay; the description keeps the backstop's moment |
| `design@agent-skills@one-decision-per-head` | the harvest cuts the heads by it |
| `design@agent-skills@existing-heads-on-touch` | the two rewritten heads are brought to the rules in the same change |
| `design@agent-skills@design-home-write-loads-recording` | rests on it, unchanged |
| `design@agent-skills@outcome-over-display` | met: the failure corrupted what was built and recorded, not a display |
| `rejected@agent-skills@every-decision-through-design-skill` | not brought back: the condition excludes plan steps, fixes that restore a recorded result, and changes that decide nothing |
| `rejected@agent-skills@separate-bounded-problem-skill` | not brought back; #grounding-split-from-discussion lost on another fact |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | the premortem's causes 2 and 3 are of its class; judged at the harvest |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` | more work goes through the skill, which narrows its gap; it stays open |
| `issue@agent-skills@the-open-space-test-may-admit-every-problem` | more work reaches the bounded path's classification; it stays open |
| `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit` | the mechanism of cause 2; unchanged |
| `design@knowledge-architect@workflow-findings-stay-here` | followed: the finding is repaired at the source |
| `goal@knowledge-architect@the-workflow-improves-through-real-use` | followed: the evidence is the two real sessions |
| `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` | the condition names a ruled outcome; a session finds it where it is kept, unchanged |
| `design@agent-skills@instruction-record-is-minimal` | the wording's reasons go in the commits; the harvest records the decisions |
| `design@agent-skills@behaviour-additions-need-real-use` | met by the two cloud sessions |
| `design@agent-skills@synthetic-evidence-not-built` | no run is built |
| `design@agent-skills@capabilities-not-structure` | the opening paragraph answers a default that was wrong in both sessions read |
| `design@agent-skills@statement-names-an-act` | the root `CLAUDE.md` sentence duplicating the opening paragraph is deleted at step 4 |
| `design@agent-skills@term-defined-at-first-load` | the opening paragraph coins no term |
| `design@agent-skills@admission-rules-home` | the edits are written under the agent-configuration skill |
| `design@core@section-homes-carry-slugs` | no section is added or renamed |
| `design@agent-skills@in-change-path` | the reason this is a spec |
| `design@agent-skills@record-searched-before-the-work` | rewritten at step 5 |
| `design@agent-skills@review-judges-additions` | the review owes the agent-facing-text axis |
| `design@agent-skills@head-ground-is-the-argument` | the harvest's heads stand on their arguments; the owner's proposals are attributed |
| `design@agent-skills@unargued-approval-is-argued` | every approved thread carries its argument |
| `design@agent-skills@owner-rules-at-a-fork` | unchanged; later consequence: it may cite `goal@knowledge-architect@design-with-agents-as-counterparts` |
| `design@agent-skills@derived-decision-proceeds-on-default` | unchanged: the bounded path still builds its default |
| `design@agent-skills@shared-text-fragment` | the condition is three one-sentence restatements, not a fragment: a fragment cannot sit in a frontmatter line |
| `design@agent-skills@shipped-text-cites-no-entry` | the new shipped text cites no entry |
| `design@agent-skills@no-external-handoff` | the opening paragraph names no outside skill |
| `design@agent-skills@workflow-axis-finding-standard` | the precedence gap is a provable gap of two listed skills; the load failure has real-use evidence |
| `design@agent-skills@root-cause-repair` | the repair is at its cause, the trigger's condition, and adds one paragraph only where no removal closes the cause |
| `design@agent-skills@repairs-read-against-the-record` | this section is that reading |

Graded `may-bear` and read whole:

| entry | outcome |
| --- | --- |
| `rejected@agent-skills@root-routing-table` | not brought back: the opening paragraph maps no project skill to an installed one |
| `tripwire@agent-skills@pointer-not-followed` | does not fire: no restatement becomes a pointer |
| `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` | same family as the PR 24 instance; unchanged |
| `tripwire@agent-skills@a-shortcut-decision-earns-a-head` | does not fire; more work reaches its premise |
| `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | does not fire: no deferred trigger was met by either cloud session |
| `tripwire@agent-skills@head-created-without-deliberation` | does not fire on PRs 24 and 25: each head created there records its rival and the owner's ruling |
| `rejected@core@session-start-hook-primer` | not brought back: no hook is proposed |
| `design@agent-skills@undesigned-work-is-an-issue` | an issue's leads state no result, so an issue does not settle a design under the condition |
| `design@agent-skills@retro-trigger` | rests on its argument |

Graded `may-bear` and not read, each with the searcher's reason:

| entry | the searcher's reason |
| --- | --- |
| `design@agent-skills@verification-in-primer` | a direction moving part of the design skill's grounding into the primer would meet "a skill keeps only the narrower rule its own moment needs" and "The design skill's grounding reads the section again"; no such direction is named yet |
| `design@agent-skills@questions-go-to-their-authority` | the candidate condition "an owner's proposal does not count as settled" and "a ruled outcome" decide what is settled and by whom, which this head routes ("what follows from the goals and the recorded decisions" is the session's); no conflict established until the condition is worded |
| `rejected@agent-skills@delegated-decision-state` | Element: a routing condition that lets the owner's word mark a change as settled, skipping the design skill; sentence: 'A word that hands judgement over is an approval like any other'. The candidate wording ('an owner's proposal does not count as settled') goes the other way; a rival exempting work on the owner's word would bring it back. |
| `rejected@agent-skills@directive-restated-everywhere` | kept because `tripwire@agent-skills@pointer-not-followed` may make it the candidate; restating the design skill's trigger in several texts would approach it; the observed miss is a skill not loaded, not a restatement turned pointer |
| `rejected@agent-skills@head-rules-whole-in-primer` | a direction putting the design skill's routing whole in the primer meets "The primer is read once at the start of a session and can dilute in a long one"; no such direction is fixed yet |
| `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched` | same mechanism as the work's defect, a session skipping a pointer it should follow ("applied from memory where the session skips it"); the issue's subject is pointers to standards, not skill loading, so no relation established. |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | a direction that puts the routing in the primer or the root CLAUDE.md could create a root restatement of an installed directive, which this issue says no step re-reads after an upgrade; it also records that the harness delivered a root CLAUDE.md older than disk; no bearing established unless the work adds such a restatement |
| `issue@agent-skills@patching-an-installed-skill` | the dependency the work removes is an instruction outside the installed text (the owner's user-level CLAUDE.md routing to the design skill); a project-level stopgap in a root CLAUDE.md would be an addition, not the supersession this issue is about, unless it contradicts an installed trigger; no bearing established |
| `issue@core@tooling-for-project-skills` | a reshaped description of the design skill meets the description rule ("begin each skill's description with MUST and its symptom") this issue would check for project skills; it names project skills only |
| `tripwire@agent-skills@fix-proposed-past-an-unread-head` | the observed cloud sessions built changes without the design skill's grounding; if a retrospective analysis reports one as a fix past an unread head of its code's record, it counts toward firing; not established from the brief |
| `tripwire@agent-skills@record-answered-questions-reach-the-owner` | routing owner proposals through the design skill can add questions put to the owner; its firing clause is a retrospective finding a question the record answered, which the routing does not itself meet. |
| `design@core@plan-register` | the candidate condition "the change's shape is not already stated by a plan document" relies on "The plans directory holds the decided design of work that is not built yet"; agreement, no conflict established |
| `design@core@plan-document-sections-by-kind` | a visible condition "the shape is stated by a plan document" may key on the sections "What is already decided" or "Decided design" this head fixes; no conflict established |
| `design@agent-skills@audit-axis-runs-a-workflow` | the work changes skill descriptions and a primer rule on loading skills; this head argues from Claude Code's behaviour that a tool runs only on an explicit opt-in such as a skill whose instructions tell the session to call it, and that the harness lists saved workflows with the skills; no relation established, since the work concerns the design skill, not a saved workflow |
| `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` | same class of gap as the work: an obligation stated in a skill and its description that the sessions doing the work do not read; a general primer rule on loading skills, a candidate direction, could close or reshape it; no relation established since its subject is the audit, not the design skill |
| `design@agent-skills@title-states-the-rule` | the work may rewrite the titles of the seed heads (e.g. "goes to the design skill only when ..."); a new trigger condition could leave a title wider or narrower than its argument; no title change is established yet |
| `design@agent-skills@extension-follows-the-ground` | cites the seed `design@agent-skills@new-or-reshaped-head-needs-design`; the work changes how a session reaches the design skill, not how a covered member is routed, so no element of the change meets its rule |
| `rejected@agent-skills@design-skill-for-weaker-models` | the work edits the design skill's description; if the cloud sessions ran a smaller model, a proposal to make the trigger work for any model could bring this back; the model used in those sessions is not established |
| `issue@agent-skills@a-lighter-pass-for-untouched-components` | routing more requested changes into the design skill multiplies whole-record searches; its "A grounding whose search costs more agents than sessions will pay for gets skipped" names a cause of skipping the design skill; no bearing established on its re-entry point (a ~30-Component project's retrospective) |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | "a proposal to add or cut workflow text ... is argued from intuition" without the counts; the work adds workflow text, but it argues from an observed failure rather than review cost |
| `tripwire@agent-skills@rule-title-wider-than-its-argument` | a rewrite of the seed heads' titles to state a new trigger could make a title wider than its argument; no rewrite is established yet |
| `design@agent-skills@a-head-is-owed-by-an-entry-test` | a routing trigger delivered by the primer and the description is stated at two sites, so test 2 likely admits a head; the bounded path's classification also uses these tests; no change to the tests is established |
| `design@knowledge-architect@committed-findings-analysis` | if the observation reached the session as a finding of a received or own retrospective file, its outcome is ruled and recorded in an analysis under docs/retrospective-reports/ ("A finding handled now that grows into a design discussion has its outcome rewritten ... to name the plan document or the issue"); the brief does not say how the observation arrived |
| `design@agent-skills@expectation-set-bounds-scope` | the work's finding (sessions without the owner's user-level CLAUDE.md skip the design skill) versus 'a finding that describes an owner's behaviour outside the set is not a gap': the design skill's set was not read here, so whether the finding falls inside it is not established |
| `rejected@agent-skills@admission-standard-in-primer` | a general primer rule about loading skills meets its kept doubt, 'a session that edits agent-facing text inside another task may not load the skill'; the work concerns the design skill, so a proposal to move the standard into the primer is not established |
| `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` | if the choice of trigger is grounded by a census of past session transcripts for whether the design skill loaded, the open question 'whether a census ... may support the choice of a shape' applies; no such census is in the work as stated |
| `tripwire@agent-skills@agent-text-written-without-the-standard` | its premise, that a session writing agent-facing text loads the agent-configuration skill from its trigger, rests on the description-based loading the work finds unreliable in cloud sessions; its firing clause (an unneeded addition made without that skill, in two pieces of work) is not met by the work |
| `tripwire@agent-skills@coined-term-without-an-entry` | a primer trigger that adds a coined term with no entry in `primer@coined-terms` is what it watches; it fires only when a review finds such a term in two pieces of work, which the work does not establish |
| `design@core@owned-namespace-check` | 'a session started from a fresh clone, a web session ... has them': the cloud sessions held the installed design skill and still did not load it; the work changes the installed text, not this delivery, so no bearing is established |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | the issue records the owner's finding that harness delivery of instruction files (scoped CLAUDE.md, AGENTS.md) is "very unreliable" and an intended edit of the agent-configuration skill's section on where text goes; a candidate that moves the routing rule into a file whose delivery that finding covers, or edits that section, would meet it; the candidates named (primer, skill description) are not such files, so no bearing is established |
| `design@knowledge-architect@binary-bundles-workflow` | the work removes a dependence on user-level plugins; this head's 'There is no plugin and no marketplace' delivers the installed text by the binary, which the work leaves as it is, so no bearing is established |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | the work edits and may cite 'the backstop' of primer@design-heads, a paragraph inside a level-two section; the issue says a citation of a point inside a section 'stays in words, and nothing checks it'; no bearing unless the new text cites such a point |
| `tripwire@agent-skills@ruling-lost-in-change` | Element: owner proposals met mid-task routed to the design skill, so more in-change commit messages carry rulings; firing clause: 'a ruling of the owner is found missing from, or misstated in, the commit message ... on the in-change path'. The work does not itself produce such an instance; no firing established. |
| `design@agent-skills@one-record-searcher` | Element: rewriting the design skill's description so a session loads it on its own; sentence: 'Its description carries the dispatch, since a dispatcher reads an agent's description and never its body', a premise about what the harness shows of installed text that the routing would rest on too. No conflict established. |
| `design@agent-skills@search-reads-the-whole-record` | Element: a wider trigger sends more work through the design skill's grounding, each dispatching a whole-record search; sentence: 'A lighter pass ... waits for a project whose whole record costs more agents than one grounding should'. The number of groundings, not the cost of one, would change; not established. |
| `design@agent-skills@tripwire-admission-test` | Element: a premortem cause of the routing, 'sessions still bypass the design skill'; sentence: condition 2, a cause becomes a tripwire only where 'its evidence lands where tripwires are read'. Bearing arises only at the work's premortem. |
| `design@agent-skills@premortem-as-watch-points` | the work may add a standing retrospective question on whether the session loaded the design skill before shaping a change; the head lists each question with the decision it watches ('the primer's delivery by an import line', 'whether a project skill's addition was missed'), and adding one would widen it, but the work does not yet propose one |
| `design@agent-skills@installed-fix-goes-upstream` | a general loading rule placed in the primer would rest on the premise this head states, 'the primer is the one text every session holds'; the head decides the upstream route, not routing to a skill, so no bearing is established |
| `design@agent-skills@standing-state-review-before-every-merge` | its argument rests on "the search before the work runs only for work that goes through the design skill" and "a trigger met by work that does not go through the design skill has no reader at all"; routing more work to the design skill changes that premise's scope, without contradicting the head's rule. |
| `design@agent-skills@unread-entries-reach-the-conformance-review` | its cost, "one more dispatch at the review before the merge of a branch whose grounding left entries unread outside a plan document", grows with every request newly routed to the design skill's bounded or in-change path (seed `design@agent-skills@bounded-path-in-design`); no conflict with the rule established. |
| `design@agent-skills@design-home-is-built-intent` | the candidate visible condition "the change's shape is not already stated by a plan document, a design head or a ruled outcome" leans on this head's split of built intent (design home) and unbuilt intent (plan document); no conflict established. |
| `design@agent-skills@divergence-routed-by-its-authority` | it is one of the installed texts' existing routes to the design skill ("the head's reversal goes to the design skill"); a general primer rule on loading skills could restate or reshape this route; no conflict established. |
| `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own` | both turn on what the owner's word settles (an approval vs a proposal) and on agent autonomy in design; a condition treating an owner's proposal as unsettled could bear on its open discussion, but the issue's subject is attribution, not routing. |
| `design@agent-skills@shipped-text-line-comments` | the reason for a reworded trigger on one instruction may go in a `%%` comment instead of a head ("a decision about one instruction need not take a head"); bears only on where the work's reason is kept. |
| `design@agent-skills@audit-net-growth` | a candidate (a general primer rule on loading skills plus a new trigger) adds instructions; the head's rule that repairs adding more instructions than they remove go to the owner with the net count, "as admission-standard says", is stated for an audit's set of repairs, and whether the admission standard carries it to a repair outside an audit is not established here |
| `rejected@agent-skills@one-repair-per-finding` | a repair that only adds a trigger line at the design skill's description, the site where the failure was seen, could be the "one repair per finding, at its site" shape; the entry is rejected for the audit's method, so whether it binds a single repair outside an audit is not established |

## Criteria

### The routing works from the installed text alone `##routing-from-installed-text`

Binding. Source: `goal@agent-skills@installed-text-works-anywhere`, as approved in round 3.
Satisfied by #settled-means-recorded, #skill-load-rule-in-primer and #installed-skill-takes-precedence.

### Only the needed instructions are added `##only-needed-instructions-held`

Binding. Source: `goal@agent-skills@only-needed-instructions`. Satisfied: each addition answers a
failure seen in a real session or a gap anyone can see by reading.

### A trigger names a condition visible before the load `##trigger-condition-visible`

Binding, as a presumption. Source: the argument of
`design@agent-skills@design-home-write-loads-recording`. Satisfied by #settled-means-recorded.

### The owner's proposals are grounded and challenged before they are built `##proposals-grounded-before-built`

Binding. Source: `goal@knowledge-architect@design-with-agents-as-counterparts`, approved in round 5.
Satisfied by #settled-means-recorded and #search-after-first-challenge.

### One record search per requested change `##one-search-per-requested-change`

Weighed. Source: the cost measured in round 1, about 1.05M subagent tokens for the search of that
session, summed from its 10 agents' completion reports. Unmet and accepted, on the owner's word in
round 4: "About the record search cost: I think this is right."

## Threads

### A design counts as settled only where a record the session can see states what the change must produce `##settled-means-recorded`

Proposed by the agent in round 1, as its default. Final state: approved. Moved by
`argument@design-skill-routing@arg-1`, `argument@design-skill-routing@arg-2`,
`argument@design-skill-routing@arg-3`, `argument@design-skill-routing@arg-5`,
`argument@design-skill-routing@arg-9`, `argument@design-skill-routing@arg-22`,
`argument@design-skill-routing@arg-33`, `argument@design-skill-routing@arg-34` and
`argument@design-skill-routing@arg-39`. Presumed settled from round 2, on the owner's "About
settled-means-recorded: this looks like a reasonnable solution." Closed in round 5 by the owner's
word against the displayed checkpoint table, which the design skill's loop step 6 makes the batch
word that promotes the presumed-settled bucket: "AC1 and AC2 both accepted, proceed with the spec
then implementation on the same branch". Shape: Decided design. Harvest: rewrites
`design@agent-skills@bounded-path-in-design`.

### Every requested change loads the design skill, with no condition `##design-load-unconditioned`

Proposed by the agent in round 1, as the rival of its default. Final state: ruled-out, on the
argument `argument@design-skill-routing@arg-6`, in round 1; the owner did not contest it. Shape:
Losing alternatives. Harvest: judged for a `rejected` entry.

### The primer's list opens with a rule: load the skill at its moment and follow it whole, a section found by a search not being the skill `##skill-load-rule-in-primer`

Proposed by the agent in round 1, as a rival: "A general rule in the primer of the superpowers kind
('load any skill that might apply')". Ruled out in round 1 by `argument@design-skill-routing@arg-7`;
reopened in round 2 as a material finding by `argument@design-skill-routing@arg-13`,
`argument@design-skill-routing@arg-14` and `argument@design-skill-routing@arg-15`; reshaped in round
3 by `argument@design-skill-routing@arg-22` and `argument@design-skill-routing@arg-23`; reworded in
round 4 by `argument@design-skill-routing@arg-30` and `argument@design-skill-routing@arg-35`. Final
state: approved. The owner's words: round 4, "skill-load-rule-in-primer: accepted", given on the
round 3 sentence; the round 4 rewording was closed by the round 5 word against the displayed table.
Shape: Decided design. Harvest: judged for a head of its own.

### An outside design-discussion skill displacing the design skill, answered by one installed sentence `##plugin-skill-displaces-design`

Proposed by the agent in round 1, with a default to park it. Moved by
`argument@design-skill-routing@arg-4`, `argument@design-skill-routing@arg-8`,
`argument@design-skill-routing@arg-12`, `argument@design-skill-routing@arg-19` and
`argument@design-skill-routing@arg-24`. Final state: superseded by #installed-skill-takes-precedence.
In round 2 it was marked "absorbed by the goal draft"; in round 3 #installed-skill-takes-precedence
absorbed it. The owner's words in round 2: "About plugin-skill-displaces-design: actually, my take on
this is that plugin are not truly adapted to agentic work, because they are not be made sufficiently
specific to a project, and installation is not very reliable." Shape: Losing alternatives. Harvest:
none of its own.

### The design skill split into a grounding skill and a discussion skill `##grounding-split-from-discussion`

Proposed by the owner in round 2. Moved by `argument@design-skill-routing@arg-10`,
`argument@design-skill-routing@arg-11`, `argument@design-skill-routing@arg-16`,
`argument@design-skill-routing@arg-17` and `argument@design-skill-routing@arg-18`. Final state:
withdrawn by the owner in round 3: "I agree on not splitting the design skill, you've convinced me
there is little value and more risks in doing that." Shape: Losing alternatives. Harvest: judged for
a `rejected` entry.

### For an activity the list covers, a skill from outside the installed set and the project never takes the place of the installed skill and the project's skills that add to it `##installed-skill-takes-precedence`

Proposed by the agent in round 3. Moved by `argument@design-skill-routing@arg-24`,
`argument@design-skill-routing@arg-30`, `argument@design-skill-routing@arg-35` and
`argument@design-skill-routing@arg-36`. Final state: approved. The owner's words in round 4:
"installed-skill-takes-precedence: for this one, be careful about the fact that consumer projects
can extend our installed skills with their own version, such as in ../games/thaum, where they must
load at the same time as our published ones (actually maybe we even do this internally, such as
klarch-retrospective-intake)". Presumed settled in the round 4 checkpoint, closed by the round 5
word against the displayed table. Shape: Decided design. Harvest: judged for a head of its own.

### The design skill's record search runs after the manual grounding and the first challenge, with the problem and the drafted shapes as the work `##search-after-first-challenge`

Proposed by the owner in round 3. Moved by `argument@design-skill-routing@arg-20`,
`argument@design-skill-routing@arg-25`, `argument@design-skill-routing@arg-26`,
`argument@design-skill-routing@arg-27` and `argument@design-skill-routing@arg-28`. Final state:
approved. The owner's words in round 4: "search-after-first-challenge looks right." Shape: Decided
design. Harvest: rewrites `design@agent-skills@record-searched-before-the-work`.

## Arguments

### The workflow must find its skills by itself `##arg-1`

Round 1, the owner. Threads: #settled-means-recorded, #skill-load-rule-in-primer. "I think the design
skill is not triggering reliably without what's in my personal ~/.claude/CLAUDE.md. This is a
problem, the workflow should not depend on something external like this, it should be able to self
sufficiently determine which skills to use." And: "I did not see the agent run the design grounding
search, which kinda surprised me, because I was expecting my proposals to go through the design
skill."

### The trigger asks the session to decide before the load the question the skill exists to answer `##arg-2`

Round 1, the agent. Threads: #settled-means-recorded, #design-load-unconditioned. "the design skill's
trigger makes the session decide, before it loads the skill, the one question the skill exists to
answer. The session's answer was wrong because the rule that would correct it is inside the skill."
It quotes `design@agent-skills@bounded-path-in-design`, "Whether a request is a design question or
bounded work … is known only after the grounding of the design skill's loop step 1", and the design
skill's step 4, "The owner's proposal is a claim to test, not a directive to follow".

### The record argues the same fault for other triggers `##arg-3`

Round 1, the agent. Threads: #settled-means-recorded, #skill-load-rule-in-primer. From
`design@agent-skills@design-home-write-loads-recording`: "A load conditioned on whether an edit
contradicts a head … fires only in a session that already applies those tests." From
`design@agent-skills@retro-trigger`: "A moment named by an event can be followed by any agent where
'a moment it judges right' could not." And: "The design skill's main trigger is the only one among
the installed skills that asks the session to judge, before loading, what the skill decides."

### On the owner's machine, a plugin's hook and the user-level file gave every build request a load `##arg-4`

Round 1, the agent, labelled an assumption. Threads: #skill-load-rule-in-primer,
#plugin-skill-displaces-design. "its session-start hook injects 'If you think there is even a 1%
chance a skill might apply … you MUST invoke' and ''Let's build X' → brainstorming first'. Your
`~/.claude/CLAUDE.md` then sends brainstorming to `knowledge-architect-design`. Together, the two
give every request to build something a trigger with no condition."

### The two PRs built the owner's proposals with no grounding recorded `##arg-5`

Round 1, the agent. Thread: #settled-means-recorded. "PR 24, `56b05c03`: adds an instruction to the
primer as a member 'covered by the argument' of a head. It records no grounding and no `may-bear`
list". "PR 25, `c92e26db`: puts the command in xtask rather than in the checker 'on the owner's
agreement', with no rival argued." "The cost moved to the review instead of disappearing."

### An unconditioned load searches a plan's step a second time `##arg-6`

Round 1, the agent. Thread: #design-load-unconditioned. "A plan's step is already searched at the
design audit of `skill@knowledge-architect-planning@working-a-slice`, so the rival pays a second
search of about 1M tokens for a design that is already recorded. You also set aside 'every fix
through the design skill's grounding', per `issue@agent-skills@a-fix-is-grounded-by-no-record-search`."

### The fault is local to one trigger `##arg-7`

Round 1, the agent; defeated in round 2 by #arg-13 and #arg-14. Thread: #skill-load-rule-in-primer.
"The fault is in one trigger's condition, and the other ten installed skills' triggers already name
events. The rule would add a load bias to every session, against
`design@agent-skills@capabilities-not-structure`, and would still leave 'is this design settled?' to
the session."

### An outside skill displacing the design skill is a predicted gap `##arg-8`

Round 1, the agent. Thread: #plugin-skill-displaces-design. "The installed text cannot name an
outside skill, per `design@agent-skills@no-external-handoff`." "This is a predicted gap, and
`design@agent-skills@behaviour-additions-need-real-use` asks for real use. Your machine is the only
instance, and the other half of the effect is masked there."

### One search per requested change costs about 1M subagent tokens, close to what is paid locally `##arg-9`

Round 1, the agent. Thread: #settled-means-recorded. "Each requested change now pays one design-skill
load and one record search, about 1M subagent tokens per search. 7 of the 11 transcripts under this
project's directory on this machine already loaded the skill, this session included, so locally the
cost is close to what you pay today."

### Grounding applies before almost any change, the discussion may not `##arg-10`

Round 2, the owner. Thread: #grounding-split-from-discussion. "split the design skill. Grounding
applies before pretty much any change (except the two cases you mentionned, the change is already
planned, or the change decides nothing). But the design discussion itself might not apply. I'm not
fully certain about that though. In particular, I'm not sure how much the grounding steps can change
whether the discussion is needed."

### The design skill's value to the owner `##arg-11`

Round 2, the owner. Thread: #grounding-split-from-discussion, and the goal
`goal@knowledge-architect@design-with-agents-as-counterparts`. "The main value of the design skill
right now, I believe, is that I propose changes, and the agent carries two very important tasks
before opening the discussion: 1. grounding in design and issues 2. challenging my proposal, looking
for failures, costs, and also benefits, with measurements to back these. Then, the discussion
happen, and the rounds often spark new ideas from either me or the agent. This is why both the full
and in change path carry discussion mostly the same way. The key value of the design skill lies in
that I am made aware of key decisions not just through a report, but through an open ended
argumented discussion, which helps me find new ideas of design that are better at solving the
problems at end and better for the long term. The fact that the agent on the other side carries all
the mechanical investigation and measures costs is what allows me to focus on the larger design, and
I think it has beem generating very powerful designs in the multiple projects were I used it."

### Plugins are not adapted to agentic work, and the shipped workflow must run independently `##arg-12`

Round 2, the owner. Threads: #plugin-skill-displaces-design, #skill-load-rule-in-primer. "plugin are
not truly adapted to agentic work, because they are not be made sufficiently specific to a project,
and installation is not very reliable. One proof of that is that me having the superpowers plugin
installed has been injecting prompts in all my sessions apparently, without me even knowing about
it, changing behavior in hard to control ways (even more with automatic updates)." "I think that the
workflow we ship should be built so that it can run fully independently, and that local plugins
never take away from it."

### The decision-recording skill was skipped too `##arg-13`

Round 2, the owner. Thread: #skill-load-rule-in-primer. "I now realize that the reliable loading of
skills I am observing on this machine is probably linked to superpowers, meaning the workflow is not
standalone right now (actually, it was not only the design skill that did not load on my cloud
session, the decision recording one had also been skipped once and was loaded late)".

### An event-shaped trigger was skipped too, which defeats the ground of the ruling-out `##arg-14`

Round 2, the agent, as a material finding. Thread: #skill-load-rule-in-primer. "I ruled it out on
this ground: 'the other installed skills' triggers name events, so the defect is local to one
trigger'. You report that the decision-recording skill was also skipped once in a cloud session and
loaded late. Its trigger is an event: 'before writing into a design home'. So that ground is
defeated, and the thread is open again."

### One plain directive sentence in always-loaded text did not make the skills load `##arg-15`

Round 2, the agent. Thread: #skill-load-rule-in-primer. "This repository's root `CLAUDE.md` already
says, in `instructions@repository-skills`: 'Read the matching skill before doing that kind of work.'
The cloud sessions had that sentence and still skipped skills."

### The split already exists as a sequence, and grounding decides whether to discuss `##arg-16`

Round 2, the agent. Thread: #grounding-split-from-discussion. "So the split you describe already
exists as a sequence. What a split into two skills changes is which text loads, and when." "So
grounding is exactly what decides it. How often it decides 'no discussion' has never been observed.
No commit on main in this repository or in rootpoint names the bounded path as taken." "That zero
cannot tell 'bounded work never occurs' apart from 'sessions built bounded work without loading the
skill'".

### The split saves about 5k words against about 1M tokens of search `##arg-17`

Round 2, the agent. Thread: #grounding-split-from-discussion. "The design skill holds 9196 words
(`wc -w` on the installed copy). On the bounded path, a session applies roughly the overview, bounded
work, what earns a head and step 1, which is about half. That is about 5k words left unused, against
about 1M subagent tokens for the record search that grounding runs anyway."

### A split puts the discussion behind a pointer `##arg-18`

Round 2, the agent. Thread: #grounding-split-from-discussion. "With a split, the skill that loads
every time is the grounding skill, and its normal ending is 'no discussion needed, build'. The
discussion becomes a second load behind a pointer." "The second skill would start mid-discussion,
with the proposal already challenged by a skill that does not hold the discussion's rules". What
would flip it: "retrospectives showing sessions load the skill and then misuse the discussion
machinery on work you judge bounded".

### The owner's statement on plugins states an outcome, so it is a goal `##arg-19`

Round 2, the agent. Thread: #plugin-skill-displaces-design. "'The workflow we ship should be built so
that it can run fully independently, and … local plugins never take away from it' states an outcome.
So it goes through `skill@knowledge-architect-goal-setting`, not a design head."

### The search should run after manual grounding and a first challenge `##arg-20`

Round 3, the owner. Thread: #search-after-first-challenge. "I think actually that the full search
should be dispatched after the agent has already done a bit of manual grounding through greps or
similar, and also challenged my proposal or look for better one. This way, it can present a more
complete proposal to subagents, and get a more precise answer from the exhaustive search. The point
is that this search is quite costly, so we want to maximize value from it."

### The owner's values may be a goal `##arg-21`

Round 3, the owner. The goal `goal@knowledge-architect@design-with-agents-as-counterparts`. "I'm
wondering whether the values I mentionned about the design skill should not be stated as goals. An
open ended, symmetric discussion with each participant challenging/refining the proposals, with the
agent also carrying tracking of thread, mechanical investigation and measurements, is the general
goal I had when first creating this project, because I believe it greatly enhances creativity on both
sides and improve resulting designs. I'm not sure how much it is a goal, or just a mean to an end
though."

### The cloud transcripts confirm the owner's observations `##arg-22`

Round 3, the agent, read from both session files by a script. Threads: #skill-load-rule-in-primer,
#settled-means-recorded. Both sessions listed the skills and held no superpowers plugin. PR 24: the
design skill "Never invoked. Grepped once, at line 239, during the review"; the decision-recording
skill "skipped"; "none invoked through the Skill tool. The session read parts of skills with `grep`
and `sed`". PR 25: the design skill "Never invoked"; the decision-recording skill "late: the xtask
placement was agreed at line 155; the skill was invoked at line 533, at the review". Limits:
"Thinking is redacted in all but 9 of 160 thinking blocks", and "This repository is a special case.
The PR 24 session was editing the primer".

### A section found by a search took the place of the skill `##arg-23`

Round 3, the agent. Thread: #skill-load-rule-in-primer. "PR 24 fails it with event-shaped triggers
too, so fixing the design skill's trigger alone does not meet the goal." "The failure was that a
section found by a search took the place of the skill." Limit: "the effect cannot be tested by a
built run, per `design@agent-skills@synthetic-evidence-not-built`."

### Two skills are listed for one activity, and nothing installed says which wins `##arg-24`

Round 3, the agent. Threads: #installed-skill-takes-precedence, #plugin-skill-displaces-design. "In
this session, the harness lists both `designing-together:designing-together` and
`knowledge-architect-design`, and their descriptions open with the same sentence. No installed text
says which one wins, and today only your `~/.claude/CLAUDE.md` decides. That is a gap anyone can see
by reading the two descriptions, so it needs no further evidence."

### The relation `may bring back` can only be graded against a shape `##arg-25`

Round 3, the agent. Thread: #search-after-first-challenge. "`may bring back` means 'a `rejected` entry
whose design the work could propose again'. Given a question alone, a searcher cannot tell whether a
rejected design will come back."

### The other two occasions of the search already send a shape `##arg-26`

Round 3, the agent. Thread: #search-after-first-challenge. "The retrospective intake sends the
received files, findings with their proposed fixes. The planning audit sends the spec. The design
skill is the only occasion that sends a bare question."

### This session's search carried candidate shapes, and the grades show it `##arg-27`

Round 3, the agent. Thread: #search-after-first-challenge. "My brief carried the candidate
directions. All 10 agents graded" the goal on needed instructions "as `bears`, each reason naming the
trigger with no condition. Agent 3 graded `rejected@agent-skills@every-decision-through-design-skill`
as `may bring back` against that same candidate. This is a reading of one real run, not a
comparison: no run without candidates exists, and none may be built." Cost: "The first round
reaches you later, after the manual grounding, the challenge and the search."

### Two searches double the cost `##arg-28`

Round 3, the agent. Thread: #search-after-first-challenge. "Rival, ruled out: two searches, one at the
start and one after the challenge. It doubles the cost, and the re-search for a thread on a new
subject already exists."

### The owner's values split into a goal and means `##arg-29`

Round 3, the agent. The goal `goal@knowledge-architect@design-with-agents-as-counterparts`. "A goal:
what you get from design work. You meet each key decision in an argued exchange, not only in a
report." "A means: threads, states, rounds, the ledger. These can change and the end stays the same.
They stay as heads, `design@agent-skills@structure-the-flow` and
`design@agent-skills@owner-rules-at-a-fork`."

### Project skills extend installed ones and must load with them `##arg-30`

Round 4, the owner. Threads: #installed-skill-takes-precedence, #skill-load-rule-in-primer. "be
careful about the fact that consumer projects can extend our installed skills with their own version,
such as in ../games/thaum, where they must load at the same time as our published ones (actually
maybe we even do this internally, such as klarch-retrospective-intake)".

### The goal on room to judge means only the necessary instructions `##arg-31`

Round 4, the owner. The goal `goal@agent-skills@only-needed-instructions`. "The intended goal is not
that the text does leave room to judge, but that we cannot cover every specific case, so we only
include instructions that are strictly necessary for an agent to follow the intended workflow, and
avoid bloating the isntructions to cover specific cases where judgement of the session following the
skills is \"judged\" sufficient."

### The agent proposes too, and every proposal goes through the owner `##arg-32`

Round 4, the owner. The goal `goal@knowledge-architect@design-with-agents-as-counterparts`. "It lacks
the aspect where the agent do emit their own proposals too, as well as refine mines, and that all of
these must go through me so that I have a chance to challenge them, throw in more arguments or facts,
or change the direction of the design under discussion."

### Development cannot work in the long term without grounding `##arg-33`

Round 4, the owner. Thread: #settled-means-recorded, and the criterion #one-search-per-requested-change.
"About the record search cost: I think this is right. Actually, I think that developing a project
following knowledge-architect can probably not work out at all in the long term if changes are ever
made without grounding in design records."

### The harness lists each description whole `##arg-34`

Round 4, the agent. Threads: #settled-means-recorded, #skill-load-rule-in-primer. "The harness lists
every description whole: the design skill's line is 1157 characters in the cloud listing and ends
with its last sentence. So a truncated description is not a cause."

### Three project skills extend installed ones today `##arg-35`

Round 4, the agent. Threads: #installed-skill-takes-precedence, #skill-load-rule-in-primer. "'is the
one loaded' would have shut out a project skill that adds to an installed one. Three skills do that
today: `thaum-planning` and `thaum-review` … and `klarch-retrospective-intake` here. The primer
already says 'A project's own skills add to these, and never replace them'. The precedence sentence
must keep that and exclude only skills from outside both sets."

### A user-level skill looks like a project skill in the listing `##arg-36`

Round 4, the agent. Thread: #installed-skill-takes-precedence. "in the harness listing, a plugin's
skill carries its plugin's name as a prefix (`designing-together:designing-together`), but a
user-level skill in `~/.claude/skills` looks like a project skill. A session tells them apart by
looking in the project's `path@agent-config@skills/`. I leave that to its judgement and add no instruction for
it."

### A default may be built before the owner rules, so the goal names the merge `##arg-37`

Round 4, the agent. The goal `goal@knowledge-architect@design-with-agents-as-counterparts`. "the
primer lets a default be built before your ruling, and the branch merges only once you have ruled
(`primer@question-authority`). So the draft says 'before the work that builds it merges', not
'before it is built', which would contradict that."

### The primer's section on room to judge keeps its slug `##arg-38`

Round 4, the agent. The goal `goal@agent-skills@only-needed-instructions`. "The primer's section
`primer@room-to-judge` keeps its slug: it already says 'it states the instructions the workflow
needs … and leaves the rest to the session', which matches your intent. Renaming an installed slug
would also cost every consumer a Migration entry."

### Every change is grounded at some depth, and fixes stay as they are `##arg-39`

Round 4, the agent. Threads: #settled-means-recorded, #design-load-unconditioned. A change requested
or proposed and not settled by a record gets the design skill's grounding with the full search; a
plan's step, the planning skill's design audit with the full search; a fix restoring what a head or
a failing test states, the reading of that code's record with no search; a change that decides
nothing, none. "Default: this spec leaves fixes as they are, and the issue stays parked." No word of
the owner answered this default: it is D1.

## New names, in one place

```text
the opening paragraph    crates/agent-skills/content/PRIMER.md, the section on the installed skills, before the first bullet
```

No other new name: the work changes text, and adds no section, slug, type or command.

## Decided design

### #settled-means-recorded

**Shape.** The condition, one sentence, restated in the description, the list's design entry and
bounded work's paragraph, each in the grammar of its host. Its content, which AC1 checks in all
three:

- a change the owner asks for or proposes goes to the design skill before it starts;
- except where a plan document, a ruled outcome, or a design head or a test the code fails already
  states what the change must produce;
- or where the change decides nothing: a typo, a stale pointer, a rewording;
- a request or a proposal of the owner does not by itself settle a design.

Illustration of the shape, for the description's clause, not authority:

```text
before starting a change the owner asks for or proposes, unless a plan document, a ruled outcome, or
a design head or test the code fails already states what the change must produce, or the change
decides nothing (a typo, a stale pointer, a rewording): a request or a proposal of the owner does
not by itself settle a design
```

**Argument.** Whether a design is settled is what the skill's grounding finds out, per
`design@agent-skills@bounded-path-in-design`. A trigger that asks it before the load fires only in a
session that already holds the skill's rule that an owner's proposal is a claim to test. Every
exception is a record a session can look at. Both cloud sessions built the owner's proposals without
the skill.

**Nearest rival** #design-load-unconditioned. **Fact that defeated it:** a plan's step is already
searched at the planning skill's design audit, so the rival pays a second search of about 1M tokens.

### #skill-load-rule-in-primer and #installed-skill-takes-precedence

**Shape.** One paragraph before the list's first bullet. Illustration, not authority:

```text
Before acting at a moment this list names, load its skill, with each skill of the project that
says it adds to it, and follow them whole: a section of one found by a search is not the skill. A
skill from outside the installed set and the project, which the environment lists for the same
activity, never takes their place.
```

**Argument, first sentence.** PR 24's session reached four skills by `grep` and `sed` and wrote into
a design home without the decision-recording skill; event-shaped triggers did not suffice. The
primer is the one text every session holds, per `design@agent-skills@primer-limit`. **Nearest
rival:** fixing the design skill's trigger alone. **Fact that defeated it:** the decision-recording
skill, whose trigger is an event, was skipped in PR 24 and late in PR 25.

**Argument, second sentence.** The goal `goal@agent-skills@installed-text-works-anywhere` binds that
no skill the environment adds replaces an installed one, and the harness lists the designing-together
plugin's skill beside the design skill, both descriptions opening with one sentence. A project's
skill that adds to an installed one is kept, as the primer's "A project's own skills add to these,
and never replace them" already says. **Nearest rival:** "its skill is the one loaded". **Fact that
defeated it:** it would shut out `thaum-planning`, `thaum-review` and `skill@klarch-retrospective-intake`,
which load together with an installed skill.

### #search-after-first-challenge

**Shape.** Loop step 1 orders the grounding: read the homes it names and grep the record; draft the
shapes, the owner's proposal and the agent's own, and challenge each on its failures, costs and
benefits, with measurements; then dispatch the search, with the problem and the drafted shapes as
the work and, as seeds, the decisions the shapes rest on or may change; then read its result and
write the first round. Nothing reaches the owner before the search's result is read.

**Argument.** The relation `may bring back` is graded only against a shape. The two other occasions
of the search already send one: the intake sends the received files, the planning audit sends the
spec. **Nearest rival:** two searches, before and after the challenge. **Fact that defeated it:** it
doubles a cost of about 1M tokens, and a thread on a new subject already gets its own search.

## Mapping tables

None: the work needs no total function.

## Losing alternatives

- **#design-load-unconditioned** lost to #settled-means-recorded: a plan's step is already searched
  at the planning skill's design audit, and the owner set aside a full search of every fix, per
  `issue@agent-skills@a-fix-is-grounded-by-no-record-search`.
- **#grounding-split-from-discussion**, withdrawn by the owner, lost to keeping one skill with
  #settled-means-recorded: the split saves about 5k words of the skill's 9196, against about 1M
  tokens of a search grounding runs anyway, and puts the discussion behind a second load, the failure
  under discussion. It reopens on retrospectives showing sessions load the skill and misuse the
  discussion on work the owner judges bounded.
- **#plugin-skill-displaces-design**, superseded, under #installed-skill-takes-precedence: the
  owner's statement became the goal `goal@agent-skills@installed-text-works-anywhere`, which makes
  the gap required work, and the gap is visible by reading two descriptions.
- **The two-searches rival** of #search-after-first-challenge, never minted as a thread: it doubles
  the cost, and the re-search for a thread on a new subject already exists.

## Readings

None: the work reads no external specification. The harness's behaviour it relies on, that every
installed skill is listed with its whole description, was measured in the two cloud sessions'
transcript files.

## Premortem

Each cause is named for the harvest to judge, as `skill@knowledge-architect-decision-recording@ruled-tripwires`
says.

1. **Stresses #settled-means-recorded.** A session reads "a design head already states what the
   change must produce" too widely, because some head sits near the subject, and builds the owner's
   proposal ungrounded. No test or review fails on it.
2. **Stresses #skill-load-rule-in-primer.** The opening paragraph is read at the session's start and
   not recalled at the moment it binds, the mechanism
   `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
   records for another primer rule.
3. **Stresses #installed-skill-takes-precedence.** In a project where the designing-together plugin
   is installed, a session still loads it for a design question. The evidence appears only in
   sessions, which the owner sees.
4. **Stresses #search-after-first-challenge.** A work statement holding only the shapes leaves an
   entry bearing on the problem outside them graded `unrelated`, found at the review.
   `tripwire@agent-skills@record-search-misses` already watches misses of the search.

The two acceptance criteria below, AC1 and AC2, were put to the owner after the premortem.

## Acceptance criteria

### AC1: the three texts that state the condition state the same exceptions `##condition-stated-alike`

Guards #settled-means-recorded. Judged at the review before the merge. Fires when the description,
the list's design entry and bounded work's paragraph do not all state the same exceptions: a plan
document, a ruled outcome, a head or a failing test that states the result, and a change that
decides nothing. Response: the decision that `design@agent-skills@bounded-path-in-design` records
reopens. The owner's ruling, round 5: "AC1 and AC2 both accepted".

### AC2: loop step 1 and its head state the same order `##search-order-stated-alike`

Guards #search-after-first-challenge. Judged at the review before the merge. Fires when loop step 1
of the installed design skill and `design@agent-skills@record-searched-before-the-work` do not both
state the order: the homes read and grepped, then the shapes drafted and challenged, then the search
with the problem and the shapes as the work, then the first round. Response: #search-after-first-challenge
reopens. The owner's ruling, round 5: "AC1 and AC2 both accepted".

## Implementation sequence

Each step edits the source under `path@agent-skills@content/`, written under
`skill@knowledge-architect-agent-configuration` as `path@agent-skills@CLAUDE.md` says, and runs
`cargo klarch install-agent-skills`. No step lands a mechanism, so the first step measures nothing.

1. **The primer.** The opening paragraph; the list's design entry states the condition. Claims: the
   paragraph is the list's first line of text; `grep -rn "design is not settled"` over
   `path@agent-skills@content/` no longer finds the primer. Fails alone on: the installed primer's
   list.
2. **The design skill's trigger.** The description and bounded work's paragraph state the condition.
   Claims: a reading of the three texts finds the four exceptions in each (AC1); the description keeps
   "Requires a frontier-tier model (Opus-class or stronger)", per
   `design@agent-skills@frontier-tier-only`. Fails alone on: the harness listing of the installed
   copy.
3. **Loop step 1.** The order of #search-after-first-challenge. Claim: a reading of the step finds
   the order of AC2, and its seeds name the decisions the shapes rest on or may change. Fails alone
   on: the installed step's text.
4. **The texts that restate the moved text.** `path@agent-skills@README.md` restates the condition;
   this repository's root `CLAUDE.md` loses the sentence "Read the matching skill before doing that
   kind of work.", a duplicate of the opening paragraph's first sentence, the primer's copy staying,
   per `design@agent-skills@statement-names-an-act`; the Workflow entry of CHANGELOG.md's `Next
   release` section, then `cargo x changelog`. Claim: `cargo x gates` passes.
5. **The harvest**, per its rows below, before the review before the merge.

## Order rationale

- 1 before 2: the list's entry and the description restate one condition, and the primer is the
  text every session reads first.
- 2 before 3: loop step 1 is the step the new trigger sends a session to.
- 3 before 4: the restatements follow what they restate.
- 4 before 5: the harvest records what the tree holds.

## Defaults awaiting the owner

### D1, on #settled-means-recorded: fixes stay as they are

A fix that restores what a head or a failing test states keeps the reading of its code's record,
with no full search; `issue@agent-skills@a-fix-is-grounded-by-no-record-search` stays parked. The
owner said in round 4 that development "can probably not work out at all in the long term if changes
are ever made without grounding in design records"; whether that reaches a full search before every
fix is the owner's to rule. Built as written until the ruling.

## Harvest

| what | where | when |
| --- | --- | --- |
| #settled-means-recorded | `design@agent-skills@bounded-path-in-design`, its delivery sentence rewritten | step 5 |
| #search-after-first-challenge | `design@agent-skills@record-searched-before-the-work`, its first sentence rewritten | step 5 |
| #skill-load-rule-in-primer | judged by the entry tests for a head of its own, `installed-skills-load-whole`, in `path@agent-skills@docs/design/project-configuration.md`; named among the members of `design@agent-skills@primer-limit` | step 5 |
| #installed-skill-takes-precedence | judged by the entry tests for a head of its own under the thread's slug, beside the previous one | step 5 |
| #design-load-unconditioned | judged for a `rejected` entry under its thread's slug, cited by the rewritten `design@agent-skills@bounded-path-in-design` | step 5 |
| #grounding-split-from-discussion | judged for a `rejected` entry under its thread's slug, cited by the rewritten `design@agent-skills@bounded-path-in-design` | step 5 |
| the premortem's four causes | judged as tripwires, per `skill@knowledge-architect-decision-recording@ruled-tripwires` | step 5 |
| AC1 and AC2 | reported in the landing commits | the review before the merge |

## Later consequences

- The owner uninstalls the superpowers plugin once this lands, as the owner asked, "We should fill
  the gaps before that though"; the owner's later sessions then show the installed workflow's real
  behaviour.
- `design@agent-skills@owner-rules-at-a-fork` grounds the discussion's form on "the owner's intent";
  the next change that touches it can cite `goal@knowledge-architect@design-with-agents-as-counterparts`.
- D1's question stays in `issue@agent-skills@a-fix-is-grounded-by-no-record-search` until the owner
  states the goal on development work.
