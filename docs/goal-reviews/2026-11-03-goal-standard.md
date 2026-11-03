# Review of the whole goal set under the revised goal standard

The record of `skill@knowledge-architect-goal-setting@whole-set-review` over this repository's
goals, after the revision of the goal standard: what the review must read and do, which the owner
ruled while the standard was revised, then each round.

## Before the first round

What the rounds owe, beyond the procedure:

- **The branch merges only after rounds of this review.** The owner: "merge can only happen after we
  fix this under multiple rounds of the whole set review."
- **The rounds de-narrow the goals.** The owner: "The goal whole set review will then bring the goals
  into shape after the work, and de-narrow them when relevant."
- **Two goals whose meaning the mechanical edit changed**, by deleting a citation:
  - `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, whose sentence "Where the
    part is a procedure that the session's judgement already carries out well enough, it ships."
    lost " in the form `goal@agent-skills@only-needed-instructions` admits";
  - `goal@knowledge-architect@agents-get-a-complete-workflow`, which lost the sentence "What the step
    says, beyond what the session's judgement handles, follows
    `goal@agent-skills@only-needed-instructions`."
- **`goal@core@relocation-is-one-manifest-edit` changes shape, with no exception.** The owner: "the
  relocation goal should be updated, but not with an exception. Under our new constraints on goals,
  relocation can only happen if there is a change of purpose of the component that justifies it, or
  if it does not break the checker (a relocation with no intermediate component would never break
  the goals for example, and a rename is a mechanical edit in the goals no rewording needed)Under
  that, changing purpose (goals) should be the only manual work that needs to happen at a
  relocation, in addition  to a few mechanical edits tolerated. But the goal in itself should change
  shape, under the whole set review."
- **`goal@knowledge-architect@serves-one-kind-of-project` asks every ask to be stated in the setup
  skill and the README of each published crate.** The owner, on the citation check: "i think asks
  from checks do not need listing in setup skill, because the checks themselves report the ask to
  consumers already." That argument bears on the goal's measure.
- **The stop.** The fifth of the first round's count is the session's default under the new count,
  per `design@agent-skills@whole-set-rounds`. The owner: "we will observe the whole set review and
  decide after. I lean that blocks should probably required to reach 0. Practical use will tell us if
  that is achievable."
- **The goals the review before the merge flagged**, as material for the first round:
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` ("the project's maintenance binary"),
  `goal@agent-skills@one-skill-per-activity` ("sub-activity", whose sense the deleted citation
  carried, and "the primer's list of installed skills"), and the root goals that now restate
  agent-skills goals with no pointer ("No two instructions are ones a session cannot both obey";
  "What no instruction covers is left to the agent's judgement on purpose").
- **The model goals** for a goal judged in use, on the owner's report:
  `goal@knowledge-architect@the-workflow-leads-to-better-code` and
  `goal@knowledge-architect@projects-are-split-into-components`.
- **Entries the review reads**: `issue@agent-skills@goals-name-built-parts-until-the-whole-set-review`,
  which the commit writing the goals ruled last closes;
  `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`;
  `issue@mutate@a-mutation-of-several-files`, `design@mutate@one-file-per-run` and
  `issue@git@tools-run-git-past-the-wrapper`, which read "admits" or "can" as a requirement, a
  reclassification the owner accepted;
  `issue@core@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`,
  `issue@core@a-component-states-at-least-one-goal`,
  `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter` and
  `issue@core@configuration-for-several-agent-providers`, whose trigger is read again when
  `goal@knowledge-architect@serves-one-kind-of-project` is reworded.
- **The cost premise of `design@agent-skills@searcher-grades-goals`** is taken again once the goals
  carry their cases: `wc -c` over the goals homes against the design homes.
- **The setup skill's template goals**, in `path@agent-skills@content/skills/setup/SKILL.md`, carry
  no cases and name built parts; they are installed text, brought to the standard in the same
  branch.

## The owner's statements

The owner's statement that opened the revision of the goal standard, verbatim; every round's briefs
carry it through this record:

```text
I would like to rediscuss the goal standard and goal setting skill. We are splitting the project in smaller components, setting up new goals in the meantime, and I am observing various issues. I think there are a few exsiting open issues about the goal standard too. Here are my leads to discuss, in addition to any from the issues:
- goal changes are a rare occurence. they must be shaped so that standard work happen in their direction without any need to change them all the time. Otherwise, this is a major issue against the goal that agents must be able to work more autonomously.
- right now, the standard asks for "no ambiguity". I think this might be the cause of several symptoms. It requires giving a detailed scope of where it apply and what it asks for. Because of that, review passes tend to narrow goals to a very strict shape. This is not the intended shape I have in mind for goals that allow agent to work autonomously in the intended direction. I think that for that, goals should be more about describing far reached outcomes that the project, the owner, and the agents work toward, incrementally. It is important for goals to have no ambiguity, but not to the point of narrowing down. On the contrary, 
- The intent findings have a similar effect: the review passes tend to "narrow" goals to the shape that admits my intent, and nothing more. This ends up with drafts that rule out a lot of consequences I never ruled out. 
- Often, this also happens because the drafting agent tries to write a goal that the current tree meets. This is the wrong angle. If we are changing or adding goals, we are likely trying to shift the goals in order to drive changes in the project, its design and code. An new goal being unmet is the point, most of the time. The exception might be when goals are being reviewed and findings corrected. But even then, a reworded goal ending up unmet might be positive, because this is the kind of time where I have high chances of noticing if a goal was misworded and ended up having unforeseen consequences.
- I think that the goals of this project related to development and component creation have ended up in a good shape (more than the others). The key idea that was put into them is that meeting these goals is not observable in the tree "directly", instead only indirectly, through usage reports. Under that shape, meeting them is not an absolute criterion of the tree. Instead, it has two means of meeting them: effort is being done in the tree to meet it, and effort is being done to collect and analyze data that shows whether the goal is met or not in practice.
Some goals can be shown to be met only through things that can be observed in the tree and/or generated by the project. This is the case for a project that generates data, were correctness and quality of generated data can be tested and measured. For example, a scientific computing project, where results are tested absolutely against mathematical results. Overall, correctness related goals, or content related, can be met from the tree. But goals that are related to providing a public service/feature with effects on their usage environments (not just generate results, but also analyze or modify external data), cannot be shown to be met purely in tree, and I think this aspect is not clearly pointed out in the goal standard.
- Overall, I think that all goals should aim at being "vague", imprecise, and state outcomes that do not depend the implemented shape of the project. In particular root goals. Here in knowledge-architect for example, it is fine to define the checker as a command/tool provided by the project, without using it as a name, in goals. But it should not use it as "the checker crate", because the checker being its own create is an outcome of design choices and argumentation. The exception where this is fine would be in a subcomponent, which can state names of parent components or of itself. Because changing the shape of the project in this case would need to change what these goals state anyway. Maybe the condition is that the goal must stand even if every word in the content (docs, code...) that stands under it is replaced by other words, but keeping the same function.  
- Under this constraint, it becomes obvious that a goal must carry no citations of entries other than goals. Probably, it should also only cite goals that are in the same component or in a parent component (such as the goals it refines if any), never goals that are in a lower level component, because a lower level component can be replaced or split without changing the purpose of the containing component, or of the project.
- The proposal that I made and is recorded in an open issue, about goals avoiding to directly name things that exists in the project, also goes in that direction, in a more extreme way. It might be too extreme though. Sometimes, a goals probably needs to mention things that "exists" in the project, to say that they must exist or contain something. But in those case, I think it would be better that the goal is worded such that the existence of the item is defined from the goal itself. Then, everything does derive from the definition of the goal. I think that the main principle behind this, is that goals must keep making sense with the same wording, if the project was removed of all its design and implementation, left to an empty skeleton. Because goals are meant to be set before those, and they drive design and implementation.

Overall, I think that the goal standard, but also the way the reviewer work and report on issues, must be improved, so that they lead to goals that are more actionnable for agents, and do not end up deforming my intent. Because even with my reviewing, the narrowing effects I mentionned happen and are hard to catch. Once I have accepted a wording, the narrowing and what it rules out become "my word" for the agent, even if I did not noticed all of what it rules out, and the goal no longer state my intent. 
The narrowing is still an interesting property though. It truly surfaces whether a goal has wider effects than I could have intended. I mostly believe that the default effect for such findings should not be to propose a narrower wording, but instead to ask about the full list of what the goal admits that might not have been part of my intent, and whether that suits me.
Maybe this should go with a more precise grading of findings from the goal reviewer. Right now, the finding on overlap which does not change what an agent would do is the only graded (minor), and more variation in gravity of findings might help convergence.

Another point: the goal review in whole set mode was observed not to converge. I think this in itself is a smell that the standard has a problem. We need to find a standard that converges (or at least stops at a small number of findings). The non-convergence is in part due to corrections applied at previous rounds, which tend to have unforeseen effects when applied all together.  Though, I think that most raised findings in the later rounds were actually about intent. So I think that the non convergence might actually be because the reviews did not consider the goals "narrow" enough, and asked for more precisions. When applied to a single or low volume of goals, the reviews were converging more reliably.
But I believe that if we promote goals that are "more vague" as I propose, this should no longer happen, because less "potentially conflicting" instructions will be added, and the whole set review would have higher chances to converge.

Lastly, there is one point were I am not sure what the standard says (or if it is silent): what happens if a goal admits one shape, while another goal rules it out ? Is it a contradiction ? If yes, this might also be a cause of the narrowings I observed. I believe there is three layers, not two: goal requires, goal admits, goal rules out.
If one goal rules out somehting another requires, this is a contradiction.
If one goal rules out what another admits, this is normal. Admitting is not requiring, thus there is no conflict. The goal that rules out wins.
Also, maybe keeping one line of examples for each of Requires/Admits/Rules out in the written goals could help better transmit the intended readings.
```

## Sets

1. The owner's part: `goal@knowledge-architect@the-owner-decides`,
   `goal@knowledge-architect@the-owner-s-word-is-verified`,
   `goal@knowledge-architect@design-with-agents-as-counterparts`,
   `goal@knowledge-architect@owner-attention-on-what-is-theirs`.
2. Scope and adoption: `goal@knowledge-architect@serves-one-kind-of-project`,
   `goal@knowledge-architect@every-target-project-can-adopt-it`,
   `goal@knowledge-architect@adoption-is-easy`, `goal@knowledge-architect@setup-brings-quality-tools`,
   `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`,
   `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.
3. The workflow: the root's `goal@knowledge-architect@agents-work-without-drift`,
   `goal@knowledge-architect@agents-get-a-complete-workflow`,
   `goal@knowledge-architect@structure-and-workflow-work-together`,
   `goal@knowledge-architect@the-workflow-improves-through-real-use`,
   `goal@knowledge-architect@the-workflow-leads-to-better-code`,
   `goal@knowledge-architect@projects-are-split-into-components`, and every goal of
   `register@agent-skills@goal`.
4. The documents and the checker: `goal@knowledge-architect@design-is-recorded-with-its-arguments`,
   `goal@knowledge-architect@records-live-beside-the-code`,
   `goal@knowledge-architect@documentation-stays-consistent`, and every goal of
   `register@core@goal` and `register@git@goal`.
5. The tools: every goal of `register@gates@goal`, `register@mutate@goal` and `register@xtask@goal`.

## Round 1

Dispatched: one goal reviewer per set in draft mode and one in whole-set mode, over the goals as the
tree holds them at the commit that adds this section.

### Counts

| reviewer | findings | `blocks` | `asks` | `minor` | `settled` | `intent` | `contradiction` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| set 1, draft | 17 | 1 | 14 | 2 | 3 | 14 | 0 |
| set 2, draft | 17 | 5 | 8 | 4 | 8 | 8 | 1 |
| set 3, draft | 34 | 12 | 16 | 6 | 17 | 16 | 1 |
| set 4, draft | 26 | 10 | 9 | 7 | 16 | 9 | 1 |
| set 5, draft | 17 | 6 | 8 | 3 | 8 | 8 | 1 |
| whole set | 54 | 23 | 24 | 7 | 29 | 24 | 1 |
| all | 165 | 57 | 79 | 29 | 81 | 79 | 5 |

The first round's count for the stop, `blocks` and `asks` over all wording: 136. Many findings of
the set reviewers and the whole-set reviewer are on the same words of the same goal.

### Findings, by reviewer

The reports, each in substance, the session's condensation of the reports the reviewers returned:

#### set 1 (owner's part): 17 findings; rule1 3, rule2 7, rule3 1, rule4 2, rule6 4; settled 3, intent 14; blocks 1, asks 14, minor 2.
`goal@knowledge-architect@the-owner-decides`:
 1 R1 blocks settled: names built parts ("Decided" section, `rejected` entry, design head, tripwire) - restate what owner is informed of in own words.
 2 R1 asks intent: measure over "the installed text" - kind of interface intended? case: files written into project vs plugin. (applies to most goals; count once)
 3 R2 asks intent: does code count as durable text (iterator vs loop choice).
 4 R2 asks intent: "the record" excludes plan documents and goal-review records: is a fork ruled in a spec settled?
 5 R2 asks intent: domain "project that follows the workflow" vs serves-one-kind's "same in checker-alone project".
 6 R4 minor settled: reservations to knowledge-architect's owner are own intent (dupe in serves-one-kind and generic-tools).
 7 R6 asks intent: judged in use; measure lists installed-text properties; no learning route (retrospective has no question on misrouted questions/unreported decisions).
 8 R3 asks intent: rule-outs against de-narrow: "these and nothing else" rules out asking approval of large refactor; "never by a word that gives none" rules out overturning naming by bare preference.
 block: asks of future work: routing of ask changes/taking in a generic part not in installed text (grep lead); route for misrouted questions. Requires/Admits/Rules out cases; siblings.
`goal@knowledge-architect@the-owner-s-word-is-verified`: 1 R1 asks intent: premises "in commit messages that record a ruling" is mechanism (premise in goal-review record?); 2 R2 asks intent: "still binds the rest" two readings; 3 R4 minor: hold on contradiction is own intent; 4 R6 asks intent: in use, property list, no route. Cases: hold reaches every contradiction; premise check on "go on"; `goal@knowledge-architect@agents-work-without-drift` never-ranks-over list omits this goal.
`goal@knowledge-architect@design-with-agents-as-counterparts`: 1 R2 asks: "choice left open" wider than `goal@knowledge-architect@the-owner-decides`' fork (private fn name) -> merge needs ruling?; 2 R2 asks: no domain (checker-alone?); 3 R6 asks: in use, no unmet sentence, no route. Case: "every proposal of the owner that decides something" challenged even for plain instruction.
`goal@knowledge-architect@owner-attention-on-what-is-theirs`: 1 R2 asks: no domain; 2 R6 asks: property list, no route. Case: closed list rules out asking owner to read a retrospective file.
Met outside: `goal@knowledge-architect@agents-work-without-drift` "supervision measured by the-owner-decides" has no measure; its never-ranks list omits `goal@knowledge-architect@the-owner-s-word-is-verified`; "installed text" question once for all; serves-one-kind gives domain of three; grep: no installed instruction routes ask changes/generic part to k-a's owner (lead).

#### set 2 (scope and adoption): 17 findings; settled 8, intent 8, contradiction 1; blocks 5, asks 8, minor 4; stop count 13.
`goal@knowledge-architect@serves-one-kind-of-project`: F1 R1 blocks settled "the gates library" (l.384, 410) -> define by function. F2 R1 asks intent: kinds named (setup skill, README of each published crate/package, API docs, rendered site, published crate) across set - intended kinds? crate vs package. F3 (a) blocks contradiction: measure requires asks listed in setup skill vs owner's D3 statement. F4 R2 asks: "AI agents drive ... occasional edit" boundary (a third of commits by a person?). F5 R4 minor: four intents (definition+asks; merge conditions l.388-400; no-limiting-coverage l.413-420; exclusion l.427-432). F6 R6 minor: merge conditions/coverage/exclusions unmeasured. Block: future work per issue target-project-and-asks; cases owner may not intend: setup skill's no-Rust binary install (SKILL.md:71) ruled out; init --vcs none; publication word before merge for CI-after-merge publishing; quality tool taken in => ask; claude ask rules out other harnesses.
`goal@knowledge-architect@every-target-project-can-adopt-it`: F7 R2 asks: "tree they are built from aside" two readings (compile docs/goal-reviews path?). F8 R4 minor: three separate intents (this repo is a target project; one grammar; pin). F9 R6 asks: existing project adoption judged in use; measure property of text; no route. Cases: pin only by explicit edit rules out tracking latest; this repo bound by every goal.
`goal@knowledge-architect@adoption-is-easy`: F10 R1 asks: "the rendered site" depends on lower Component goal. F11 R6 asks: property list, no learning route. Cases: site must carry whole adoption; examples in every library API doc incl. git.
`goal@knowledge-architect@setup-brings-quality-tools`: F12 R2 asks: boundary of "quality tool" (is hand mutation a quality tool -> ask?). Case: lints taken in become binding ask.
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: F13 R5 asks: "it ships." vs `goal@agent-skills@only-needed-instructions`; rank or form in root's words (rule 7 forbids restoring citation). F14 R1 blocks: "beyond a restatement the primer admits". F15 R6 blocks: measure "or an issue entry records the work" rests on something that leaves -> "or its project's own copy stays". F16 R6 minor: third bullet ranges over this repository only. Case: reporting duty reaches every tool in projects owned by others.
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: F17 R1 blocks: "the project's maintenance binary" (`design@agent-skills@maintenance-crate-ask` decision) -> "a library function that a target project's own tooling calls". Case: two-step tasks.
Met outside: "the checker" used by root goals (l.24,49,211,378,411,443,457), defined by none; setup skill template goals.

#### set 3 (workflow): 34 findings; rule1 11 (10 blocks, 1 asks), rule2 11 (1 blocks, 10 asks), rule4 3, rule5 1, rule6 8 (5 asks, 3 minor); settled 17, intent 16, contradiction 1; blocks 12, asks 16, minor 6.
`goal@knowledge-architect@agents-work-without-drift`: 1 R1 blocks ("the checker", "the installed text" undefined at root; design heads, tripwires, issues) - counted once for set; 2 R6 asks (in use, no learning route; departure merges unasked); 3 R6 minor (supervision measured by `goal@knowledge-architect@the-owner-decides`). Cases: ranks first over every Component goal; "never ranks over what serves-one-kind states" covers its exclusions too.
`goal@knowledge-architect@agents-get-a-complete-workflow`: 4 R1 asks (root requires "a skill per activity" = lower Component's interface?); 5 R1 blocks ("project audit's method"); 6 R2 asks (no set for "no two instructions": installed vs project CLAUDE.md); 7 R4 minor (plainness/contradiction/judgement sentences restate agent-skills goals); 8 R2 asks (Rust idiom vs default shape: typed error per module); 9 R2 asks (deleted sentence: full test-design method vs what judgement doesn't handle); 10 R6 asks. Cases: "whatever the test of a generic part finds" rules out an idiom guide owner would take in; releasing skill installed everywhere; reviewing procedure relying on PR.
structure-and-workflow: 11 R1 blocks (knowledge table, design head, anchor, issues/tripwires, manifest); 12 R6 minor. Cases: a project's own register with no workflow step (glossary); checker features for checker-alone use.
`goal@knowledge-architect@the-workflow-leads-to-better-code`: 13 R1 blocks ("the installed retrospective skill") -> name the retrospective activity. Case: one-line fix gets a code review.
projects-are-split: 14 R1 blocks (manifest definition; retrospective skill). Case: criteria applied mid unrelated task.
`goal@knowledge-architect@the-workflow-improves-through-real-use`: 15 R2 blocks ("admits" used in another sense than the standard's layer -> replace verb); 16 R2 asks ("fails a goal ... defect provable by reading" for in-use goals); 17 R2 asks (mock-task exclusion vs census issue; agent tools dispatch); 18 R4 minor (owner sentence qualifies `goal@knowledge-architect@the-owner-decides`); 19 R6 minor (no measure that evidence changes text: intake). Cases: dry run of a saved workflow; replay of a recorded session.
`goal@agent-skills@one-skill-per-activity`: 20 R2 asks (sub-activity undefined; decision recording); 21 R1 blocks (primer's list); 22 R6 asks. Cases: loading a skill on own judgement; skills installed for unused activities.
`goal@agent-skills@no-contradiction-in-workflow`: 23 R1 blocks (saved workflows, primer); 24 R2 asks (body vs measure sets).
reviews-judge: 25 R1 blocks; 26 R2 asks (code under development skill); 27 R4 minor.
installed-text-every-project: 28 R1 blocks (%% comments, declared checker command, manifest layout); 29 R6 asks. Case: fallback for every instruction.
installed-text-any-environment: 30 R2 asks (harness's own text: relied on vs must not replace); 31 R6 asks. Case: owner's user-level CLAUDE.md can't replace an installed skill.
`goal@agent-skills@only-needed-instructions`: 32 R5 blocks contradiction (vs generic-tools "it ships."); 33 R1 blocks; 34 R2 asks ("act a goal's text states" for goals measured by tests: hook setup?).
Met outside: "admits"/"admission" other sense in serves-one-kind l.416, generic-tools l.494; goals homes' heads "A goal stays here while it is met" vs "a new goal is expected to be unmet"; agent-skills head no purpose (issue).

#### set 4 (documents and checker): 26 findings; rule1 11, rule2 3, rule4 1, rule6 9, rule7 1, contradiction 1; settled 16, intent 9, contradiction 1; blocks 10, asks 9, minor 7.
`goal@knowledge-architect@design-is-recorded-with-its-arguments`: R1 blocks (design head, rejected designs it cites, issue, tripwire) -> state kinds by content; R6 asks (in use, list measure, no unmet sentence); R2 asks (which no-head decisions need comment+commit reason: dozens per unit); R4 minor (outstanding tracking second intent). Cases: every trivial choice; a head of absence must name alternatives.
`goal@knowledge-architect@records-live-beside-the-code`: R1 blocks (knowledge table, project's own rows, retrospective's workflow file/project files, intake); R2 asks ("the project's own decisions": Decided-line decisions in repo?). Cases: PR-thread decisions copied; Decided lines copied.
`goal@knowledge-architect@documentation-stays-consistent`: R1 blocks ("compiled in or declared in the manifest", declaration switches off); R1 asks ("in the texts the checker reads" - saved workflows unchecked; direction?); R6 asks (reviews/audits in use); R6 minor (listing guarantee has no measure line).
core `goal@core@documents-stay-current`: R1 blocks (register declares shape, generated listing); R6 minor (no test named).
core `goal@core@document-rules-are-checked`: R1 blocks (core compiles in, manifest, walk, empty list of harnesses); R6 minor.
core `goal@core@relocation-is-one-manifest-edit`: contradiction (a) blocks vs owner's Q2 statement; R1 blocks (manifest, declared register's home, anchors, path references, basename); R7 blocks (refines `goal@knowledge-architect@documentation-stays-consistent` but parent states no cost - may fall).
core `goal@core@records-reach-their-reader`: R6 asks (in use, completeness measure); R1 asks (two listings = mechanism; filtered listing meets parent).
core `goal@core@documents-render-as-a-linked-site`: R1 blocks (preprocessing pass, mdbook; design argument belongs in head); R6 minor (hosted outside build). Not met yet (no book config).
core `goal@core@projects-add-their-own-checks`: R1 blocks ("project's checker binary", same subcommands); R6 minor (measure omits body). "no change to the core" admitted (own Component).
core `goal@core@checks-a-commit-from-its-hook`: R1 asks (commit forms list is mechanism; --fixup outside; owner ruled exclusion at 00ce06f7 before de-narrow direction); R6 minor (measure vs body sets).
git `goal@git@a-pure-wrapper-over-git`: R2 asks (fixture-building sequences / mutate init+apply: library's or tool's?).
Met outside: no root goal defines "the checker"; built-parts issue lists "no change to the core" which rule 1 admits; issue anchor-own-directory "What would close it" first option (exception) ruled out by owner's "not with an exception"; "every component states at least one goal" in no goal.

#### set 5 (tools): 17 findings; rule1 8, rule2 3, rule3 1, rule5 1, rule6 4; settled 8, intent 8, contradiction 1; blocks 6, asks 8, minor 3; stop count 14.
`goal@gates@gates-from-a-list`: G1 R1 blocks ("a project's binary", "a maintenance binary"); G2 R1 asks (distilled failures/complete logs are mechanisms?); G3 R2 asks ("a gate command": the one command vs a gate's command); G4 R5 blocks contradiction (taken-in-not-shipped interval: no own copy vs root "Until a part taken in ships, the project's own copy of it stays"; owner ruling af5cd491 only declined case); G5 R6 minor (no test).
mutate `goal@mutate@a-named-mutation-is-checked-safely`: M1 R1 blocks (`cargo x` command in measure); M2 R1 blocks (baseline/skip and build probe restate heads; causes are the outcome); M3 R1 asks ("kept where the crate's recovery reads it" presupposes automatic recovery); M4 R3 asks (several files required or admitted; 5656d472 vs 8be31725); M5 R6 asks (losing nothing judged in use; property list; no route). Case: "never chooses or generates" rules out a helper proposing mutations from a diff.
xtask `goal@xtask@one-command-runs-every-gate`: X1 R1 blocks ("the continuous integration runs the same command" - CI a design choice); X2 R1 asks (library defaults restated); X3 R2 asks ("on the working tree" vs root "on the branch's head"); X4 R6 minor.
xtask `goal@xtask@repeated-tasks-are-automated`: R1 R1 blocks (`klarch-` prefix); R2 R2 asks ("needs no judgement" whole vs per part); R3 R6 minor (measure narrower than body). Cases: merge predicate command runs gh pr merge; `skill@klarch-release` steps incl publish owed a command.
Met outside: built-parts issue's grep misses binary (gates), baseline/probe (mutate), CI (xtask); mutate issue/head hang on M4; gates/xtask heads no purpose (issue exists).

#### whole-set reviewer: 54 findings; settled 29, intent 24, contradiction 1; blocks 23, asks 24, minor 7. Rule7 citations all admitted. Conforming 8 goals.
Root: `goal@knowledge-architect@agents-work-without-drift` F1 R1 blocks (design heads, tripwires, issues), F2 R6 minor ("supervision measured by `goal@knowledge-architect@the-owner-decides`" - no such measure), F3 R6 asks (in use), F54 R2 asks (rank of an ask a quality tool brings). design-is-recorded F4 R1 blocks, F5 R6 asks. `goal@knowledge-architect@agents-get-a-complete-workflow` F6 R4 minor (instruction plainness/no-two-contradict/judgement paragraph dup of agent-skills goals; "particular to one project" paragraph), F7 R1 blocks ("project audit's method"), F8 R2 asks (Rust idiom vs default shape: typed error enum), F9 R6 asks; case: "how to test properly" required regardless of judgement after deletion. mechanical-tasks F10 R1 blocks (maintenance binary), F11 R1 asks (published crates: intended kind of interface? across 5 goals). structure-and-workflow F12 R1 blocks (knowledge table, issues/tripwires, manifest), F13 R6 minor. records-live F14 R1 blocks. `goal@knowledge-architect@documentation-stays-consistent` F15 R1 blocks, F16 R6 asks. `goal@knowledge-architect@the-owner-decides` F17 R1 blocks, F18 R2 asks (k-a owner items in other target projects: route upstream?), F19 R4 minor, F20 R7 minor ("the record" term defined here used by two other goals), F21 R6 asks. the-owner-s-word F22 R1 asks, F23 R6 asks. design-with-agents F24 R6 asks. owner-attention F25 R6 asks. serves-one-kind F26 R1 blocks (gates library), F27 R4 minor (six intents), F28 R6 asks (D3 vs measure). every-target F29 R1 blocks (manifest), F30 R4 minor. generic-tools F31 R5 blocks contradiction (vs `goal@agent-skills@only-needed-instructions`; cross-home), F32 R6 blocks (issue entry measure), F33 R1 blocks (primer). better-code conforming. projects-are-split F34 R1 blocks (manifest definition of Component). `goal@knowledge-architect@adoption-is-easy` F35 R1 asks (rendered site), F36 R6 asks. real-use conforming (case: dry run of a skill on scratch project ruled out). setup-brings conforming.
core: `goal@core@documents-stay-current` F37 R1 asks (core's vocabulary manifest/registers/anchors/listings: intended interface?). document-rules F38 R1 blocks. relocation F39 R1 blocks (reshape per owner). records-reach conforming. documents-render F40 R1 blocks (mdbook), F41 R2 asks ("hosted once released" - crates already published). projects-add F42 R1 asks (checker binary). checks-a-commit conforming (options left out: owner's).
agent-skills: one-skill F43 R1 blocks (primer's list), F44 R2 asks (sub-activity). no-contradiction conforming. reviews-judge F45 R1 blocks (register names). installed-text-every-project F46 R1 blocks (%% comments), F47 R1 asks (declared checker command). any-environment F48 R6 asks. only-needed F31.
git F49 R2 asks (can vs measure requires). gates conforming. mutate F50 R1 blocks (cargo x), F51 R1 asks (baseline/probe). xtask one-command F52 R1 blocks (CI); repeated F53 R1 blocks (klarch-).
Cross-home conflicts: only F31.
Met outside: record head stale ("No round has run yet"); checks-a-commit refinement line first.

### Questions put to the owner, and the owner's answers

Q8, the words the root goals use and do not define; Q9, the kinds of interface that are outcomes; Q10,
goals judged in use with no learning route; Q11, four contradictions, (a) shipping a procedure
judgement handles, (b) the interval between taking a part in and shipping it, (c) the owner's D3
against the scope goal's measure, (d) the relocation goal; Q12, the goals homes' heads; Q13, the
other findings, the reach of the owner's goals over a project that uses the checker alone among them.

The owner's answer, verbatim:

```text
Q8 agreed on this shape. It should probably be recommended in the skill itself: definitions in a separate paragraph of a goal home, not per goals, because they will often be shared among most goals . Unless the definition is the outcome of a specific goal, such as  the retrospective below. Actually, maybe each such definition should be a goal in itself. Components can have their owns. Whichever shape, they should stay general and few. The skeleton test still applies.

Q9 agreed on default

Q10 agreed, centralise in the real use goal.

Q1 a agreed 
b "taken in" replaced by in a released version of knowledge architect . This is when a consumer should consider following us. At its own bump, actually.
c agreed
D agreed

Q12. Are these heads necessary ? They are restatements of the goal setting skill, and they drift.

Q13: for checker only project: our target project use both the workflow and the checker. The checker only mode is kept as an exception to serve one kind of project, stated there. Nothing is done to cater to it. Consumers use it at their own risk.
```

"Q1" in the answer is Q11, answering its items (a) to (d).

### Round 1's repairs

Two drafters wrote the repairs into the tree, one for the root goals home and one for the six
Component goals homes, under one brief: every finding routed `settled` repaired as its implied
repair says, the owner's answers Q8 to Q13 applied, an `intent` finding's wording kept unless an
answer changes it, and "for instance" cases written for each layer of each goal. The session wrote
the root head's definitions paragraph, and trimmed each Component head to its purpose, per its
default on Q12.

Choices the drafters made, kept in the tree and put to the owner with the batch:

- `goal@core@relocation-is-one-manifest-edit` refines `goal@knowledge-architect@agents-work-without-drift`
  instead of `goal@knowledge-architect@documentation-stays-consistent`, answering the whole-set
  finding that the parent states no cost; the rival is to drop the refinement line.
- In `goal@knowledge-architect@serves-one-kind-of-project`, Q13's sentence follows the four
  conditions and is no fifth one, and the merge condition loses its qualifier "where the project
  uses the installed workflow".
- The false sentence "The supervision it saves is measured by" the owner-decides goal leaves
  `goal@knowledge-architect@agents-work-without-drift`, with no measure in its place.
- `goal@knowledge-architect@the-owner-decides` no longer defines "the record", which the root head
  now defines.
- The Q10 question is added to `goal@agent-skills@only-needed-instructions`, which no finding named,
  and to no goal whose measure is a test of a tool or a property provable by reading:
  `goal@agent-skills@no-contradiction-in-workflow`, `goal@core@records-reach-their-reader`, mutate,
  xtask and `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.
- Rule 1 repairs that no finding named: "issue and tripwire" in
  `goal@core@records-reach-their-reader`, "the project audit's method" in
  `goal@agent-skills@one-skill-per-activity`, and "plan documents" in
  `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`.
- `goal@mutate@a-named-mutation-is-checked-safely` states the causes a verdict tells apart in place
  of the baseline skip and the build probe.

Left for the owner, as the findings route them: the purposes of the agent-skills, gates, mutate and
xtask heads, drafted by the session; the never-ranks list of
`goal@knowledge-architect@agents-work-without-drift`, which omits
`goal@knowledge-architect@the-owner-s-word-is-verified`; every `intent` finding left as it was.

### Questions Q14 to Q17, put with round 1's batch

Each was put with its default, and the owner agreed to every default:

- Q14, the purposes of the trimmed heads of agent-skills, gates, mutate and xtask, drafted by the
  session as the tree holds them; default: those wordings.
- Q15, the parent of the relocation goal; default: it refines
  `goal@knowledge-architect@agents-work-without-drift`, the rival being to drop the refinement line.
- Q16, four drafting choices: (a) the sentence on the supervision `goal@knowledge-architect@agents-work-without-drift` saves,
  measured by no goal, removed with nothing in its place; (b) no retrospective question in
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`; (c) the retrospective question added to
  `goal@agent-skills@only-needed-instructions`; (d) the mutate verdict stating the cases it tells
  apart; default: keep all four.
- Q17, the never-ranks list of `goal@knowledge-architect@agents-work-without-drift`, which omits
  `goal@knowledge-architect@the-owner-s-word-is-verified`; default: add it. Round 1's repairs did not
  apply it; round 2's do.

The defaults of Q9 to Q11 are those the drafting brief applied, as round 1's repairs state: Q9, the
published Rust crates and their READMEs and a skill per activity kept as outcomes, the setup named
"the workflow's setup", continuous integration named by where the merge verdict is taken, the
rendered site removed from the root's goals; Q10, the retrospective's question stated once in
`goal@knowledge-architect@the-workflow-improves-through-real-use`, and a line citing it in the
measure of each goal judged in use whose measure lists properties of the text; Q11, the four
contradictions resolved as the owner's answer states.

### The owner's ruling on round 1's batch, and a goal added before round 2

The owner agreed to every default of Q14 to Q17, and found the wordings in the tree good. The
owner's words, verbatim:

```text
Agreed on those defaults, the current goals in the tree all look quite fine, I believe the wordings are quite good.
```

The owner then asked why the goal design of agent-skills held so many heads on how goals are set
up, most from this session, and whether they should stand on argument; the owner had read the
design subdocument on goals, which shares the basename of the goals home. The owner agreed to add a
goal stating the outcome the goal standard serves, so that a change to the standard that strains it
goes to the owner, and kept the goal design heads as they are, with no relocation in this branch:

```text
I agree with adding the goal about goal shapes.
The design of goals can probably stay this way overall, lets not bring more relocation/migration work in scope today.
```

The session drafted that goal, on the owner's intent as the owner holds it, in the agent-skills
goals home, refining
`goal@knowledge-architect@the-owner-decides`. It is new wording in round 2. The goal design heads
that argue the standard cite it in the commit that writes the ruled goals.

## Round 2

Dispatched: one goal reviewer per set in draft mode and one in whole-set mode. Each reads the goals
as round 1's repairs left them in the working tree, copied into a worktree detached at the commit
that adds this section. The wording round 1 read is the goals homes as that commit holds them,
which round 1's repairs have not touched in any commit. The new goal on the owner's intent belongs
to set 3, as a goal of the agent-skills goals home.

### Counts

| reviewer | findings | `blocks` | `asks` | `minor` | `settled` | `intent` | `contradiction` | `blocks` and `asks` on old wording |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| set 1, draft | 3 | 0 | 2 | 1 | 1 | 2 | 0 | 0 |
| set 2, draft | 8 | 0 | 3 | 5 | 5 | 3 | 0 | 1 |
| set 3, draft | 11 | 3 | 7 | 1 | 2 | 7 | 2 | 1 |
| set 4, draft | 13 | 3 | 5 | 5 | 8 | 5 | 0 | 2 |
| set 5, draft | 6 | 2 | 1 | 3 | 5 | 1 | 0 | 0 |
| whole set | 30 | 5 | 10 | 15 | 20 | 10 | 0 | 2 |
| all | 71 | 13 | 28 | 30 | 41 | 28 | 2 | 6 |

The count for the stop, `blocks` and `asks` on the wording round 1 read: 6, below a fifth of round
1's 136. Three `blocks` findings tagged `settled` stand on that wording, so the draft-mode stop of
step 4 is not met: "the agent definitions those skills dispatch" in
`goal@agent-skills@one-skill-per-activity`; "switches off the checks of the installed text" in
`goal@core@document-rules-are-checked`; the slug of `goal@core@relocation-is-one-manifest-edit`,
which names the manifest.

### Findings, by subject

Most of round 2's findings are on wording round 1's repairs wrote, and several reviewers found the
same words.

- The new goal on the owner's intent: it drops two outcomes of the owner's statements, goals that
  agents act on without changing them, drafted to be unmet, and a standard that converges (set 3,
  `blocks`, `contradiction`); "that standard" has no antecedent, so a narrowing change of the
  installed standard would meet it (set 3, `asks`); `goal@knowledge-architect@agents-work-without-drift`'s never-ranks list
  does not name it (set 3, whole set, `asks`); its sentence on what shows a goal met is a second
  intent (set 3, whole set, `minor`).
- `goal@knowledge-architect@the-owner-decides`: "that the owner could have made" names no set (set 1,
  whole set, `asks`); "at the end of the turn that made it" fixes the place of a report (set 1,
  `asks`); the half on knowledge-architect's owner is a second intent (set 1, whole set, `minor`).
- `goal@knowledge-architect@serves-one-kind-of-project`: Q13's sentence reads as a fifth condition,
  an ask, or a description, and the merge condition now owes a review to a project with no workflow
  (set 2, whole set, `asks`); "nothing is built for it" names no design (whole set, `asks`).
- The retrospective's question: the lines in each goal's measure need the installed retrospective to
  ask about goals another project does not hold, which
  `goal@agent-skills@installed-text-works-in-every-target-project` rules out (whole set, `asks`);
  "each goal judged in use" names no owner of the goals (set 3, `asks`); which goals are judged in use
  is unsettled, `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` and
  `goal@agent-skills@no-contradiction-in-workflow` among them (set 3, whole set, `asks`); a defect
  provable by reading against a goal judged in use (whole set, `asks`); whether a "for instance" case
  makes an instruction needed (set 3, `asks`).
- The interval of a project's own copy: "at its own bump" in `goal@gates@gates-from-a-list`, where
  the project's versioning policy gives "bump" another sense (set 5, `blocks`); whether a project that
  considers the shipped part and keeps its copy is admitted, whether the copy may stay past the
  release, and when this repository, which builds the checker it runs, ships a part (set 2, `asks`);
  a measure that a copy which stays meets forever (whole set, `minor`).
- Narrowings or new senses that repairs wrote: "the record" in the title of
  `goal@knowledge-architect@records-live-beside-the-code` against its list (set 4, `blocks`); "entry
  of the record" and "the shape the project declares" in `goal@core@documents-stay-current`, which
  leave goals and the built-in kinds out (set 4, whole set, `asks`); "everything outstanding" in
  `goal@core@records-reach-their-reader` (set 4, `asks`); "its record ... that bear on the work" in
  `goal@knowledge-architect@agents-work-without-drift` (whole set, `asks`); "the decisions the workflow records in an entry of
  its own" in `goal@knowledge-architect@design-is-recorded-with-its-arguments` (set 4, `asks`); the
  circular measure of `goal@knowledge-architect@structure-and-workflow-work-together` (set 3,
  `asks`); the xtask head's "not a target project" (set 5, whole set, `blocks`); the gates head's "no
  gate logic of its own" (set 5, `asks`).
- Built parts still named: the listings the checker generates (whole set, `blocks`; set 4, `asks`);
  a retrospective's two files (whole set, `blocks`); the three on old wording above.
- `goal@core@relocation-is-one-manifest-edit`: whether "happens on a change of purpose" restricts
  when a project relocates or bounds the cost promise (set 4, whole set, `asks`); "a few" unbounded in
  the measure (whole set, `minor`).
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: "owed a command" against "a command or
  a library function" (set 2, whole set, `asks`, old wording).
- `minor` findings on old wording that round 1 left unrepaired, though the record said every
  `settled` finding was repaired: measures narrower than their bodies in
  `goal@knowledge-architect@documentation-stays-consistent`, `goal@core@projects-add-their-own-checks`,
  `goal@core@checks-a-commit-from-its-hook`, `goal@core@documents-render-as-a-linked-site`,
  `goal@xtask@repeated-tasks-are-automated`, `goal@gates@gates-from-a-list`; second intents in
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@knowledge-architect@serves-one-kind-of-project`,
  `goal@knowledge-architect@every-target-project-can-adopt-it`,
  `goal@knowledge-architect@agents-get-a-complete-workflow`,
  `goal@knowledge-architect@the-workflow-improves-through-real-use` and
  `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`; the definition of a quality tool held in
  one goal, which three use (set 2, `minor`).

Met outside the goals: `design@core@agents-table`'s declaration of no agent harness serves a
project with no agent configuration, which the reworded scope goal may exclude (whole set); the
installed retrospective asks no question per goal yet (sets 1, 3, whole set); the root head uses
"the record" and "the workflow" in a second sense elsewhere, the head's own "record of design" and
the primer's "the workflow is the method" (sets 3, 4).
