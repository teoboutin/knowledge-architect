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
- **The core's relocation goal changes shape, with no exception.** The owner: "the
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
- **Entries the review reads**: `goals-name-built-parts-until-the-whole-set-review`,
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
core the core's relocation goal: contradiction (a) blocks vs owner's Q2 statement; R1 blocks (manifest, declared register's home, anchors, path references, basename); R7 blocks (refines `goal@knowledge-architect@documentation-stays-consistent` but parent states no cost - may fall).
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

- the core's relocation goal refines `goal@knowledge-architect@agents-work-without-drift`
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
`goal@core@document-rules-are-checked`; the slug of the core's relocation goal,
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
- the core's relocation goal: whether "happens on a change of purpose" restricts
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

### Questions put to the owner after round 2, and the owner's answer

Each with its default:

- Q18, the new goal on the owner's intent: add that a goal is drafted to move what the project
  builds, so agents work in its direction without changing it, a new goal expected to be unmet, and
  that a review of a goal set stops after a few rounds at a small number of findings; "these
  properties" in place of "that standard"; the goal joins the never-ranks list of
  `goal@knowledge-architect@agents-work-without-drift`; its sentence on what shows a goal met stays.
- Q19, the per-goal retrospective lines: removed, and stated once in
  `goal@knowledge-architect@the-workflow-improves-through-real-use`: the retrospective collects the
  cases where the installed text failed the session, and knowledge-architect's intake of them judges
  each against its goals judged in use.
- Q20, Q13 in `goal@knowledge-architect@serves-one-kind-of-project`: using both the workflow and the
  checker becomes an ask, so a project that uses the checker alone is a target project that does not
  follow an ask; the merge condition stays; an issue is opened on `design@core@agents-table`'s
  declaration of no agent harness.
- Q21, `goal@knowledge-architect@the-owner-decides`: (a) "that the owner could have made" reads as a
  choice the owner could have ruled, an implementation choice a commit message records excluded; (b)
  a decision is reported where the owner reads it, the three places as cases.
- Q22, the interval of a project's own copy: (a) a project that considers the shipped part at its own
  move and keeps its copy is admitted, "found held again" being a copy written after that move; (b)
  the gates measure ends the admitted copy at the project's own move; (c) for this repository, which
  runs the checker it builds, a part ships when it merges into main.
- Q23, the core's relocation goal: "happens on a change of purpose, or where it
  breaks no check" restricts when a project relocates; the slug is renamed to
  "relocation-needs-only-goal-changes", every reference repaired.
- Q24, the narrowings round 1's repairs wrote, each de-narrowed: `goal@core@documents-stay-current`
  ranges over the entries of its goals and its record and the shape each kind requires;
  `goal@core@records-reach-their-reader` over everything the project records as outstanding;
  `goal@knowledge-architect@records-live-beside-the-code` over the whole record;
  `goal@knowledge-architect@agents-work-without-drift` reads the parts of the goals and record that
  bear on the work; `goal@knowledge-architect@design-is-recorded-with-its-arguments` gives an entry
  of its own to each decision whose argument a later session would otherwise derive again;
  `goal@knowledge-architect@structure-and-workflow-work-together` declares to the checker each home
  whose entries other records cite.
- Q25, `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: every such task is owed a command
  or a library function.
- Q26, the `minor` findings that would split a goal: left as they are.

The owner's answer, verbatim:

```text
I agree on all the proposed defaults here. Apply it, then start round 3, so we see what we can reach.
```

### Round 2's repairs

Two drafters wrote the repairs into the tree under one brief: every default of Q18 to Q26, Q17's
default, and every round-2 finding routed `settled`, but the rule-4 `minor` findings, which Q26
leaves. The session renamed the core's relocation goal's slug to
"relocation-needs-only-goal-changes" in every file that cites it, changed the root head's "what it
keeps beside its goals" to "what it keeps other than its goals", and opened an issue in the core on
the empty list of agent harnesses, per Q20; both ride with the goals' commit.

Choices the drafters made, put to the owner with the batch:

- the never-ranks list of `goal@knowledge-architect@agents-work-without-drift` names the new
  agent-skills goal in words, since a root goal cites only root goals;
- Q19's measure is in two parts: the installed retrospective collects the cases, and
  knowledge-architect's own instructions for taking them in judge each against its goals judged in
  use;
- `goal@knowledge-architect@serves-one-kind-of-project`'s measure gains three items, for the merge
  conditions, what is built only for a state that does not follow an ask, and the rule against
  limits;
- `goal@core@documents-stay-current` defines a generated text, which the checker writes into the
  project's tree from its documents, rather than dropping it; `goal@agent-skills@one-skill-per-activity`
  states the agent definitions by outcome, as the instructions a session gives another agent;
- the relocation goal counts the mechanical edits in "breaks no check";
- the new goal on the owner's intent has its measure gain the stop of a review of a goal set.

## Round 3

Dispatched: one goal reviewer per set in draft mode and one in whole-set mode, as in round 2, over
the goals as round 2's repairs left them in the working tree, copied into a worktree detached at the
commit that adds this section. The wording round 2 read is the snapshot the drafters took before
their repairs, outside the tree.

### Counts

| reviewer | findings | `blocks` | `asks` | `minor` | `settled` | `intent` | `contradiction` | `blocks` and `asks` on old wording |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| set 1, draft | 3 | 0 | 2 | 1 | 1 | 2 | 0 | 0 |
| set 2, draft | 6 | 0 | 2 | 4 | 4 | 2 | 0 | 1 |
| set 3, draft | 8 | 0 | 7 | 1 | 1 | 7 | 0 | 0 |
| set 4, draft | 11 | 1 | 6 | 4 | 5 | 6 | 0 | 1 |
| set 5, draft | 5 | 1 | 3 | 1 | 2 | 3 | 0 | 1 |
| whole set | 11 | 1 | 5 | 5 | 6 | 5 | 0 | 0 |
| all | 44 | 3 | 25 | 16 | 19 | 25 | 0 | 3 |

Over the three rounds, all findings went 165, 71, 44; `blocks` 57, 13, 3; the count for the stop
136, 6, 3. One `blocks` finding tagged `settled` stands on the wording round 2 read: "the gates", in
the gates goals home's head, names the Component and the merge checks both. Most of round 3's
findings are `asks` on wording round 2's repairs wrote.

### Findings, by subject

- The root head's "the record ... what it keeps other than its goals: ...": the list reads as
  examples or as the whole set, which decides whether a plan document or a retrospective's pending
  findings are record (sets 1, 3, 4, whole set, `asks`).
- `goal@knowledge-architect@the-owner-decides`: whether the durable text that records an
  implementation choice is itself its report (set 1, `asks`); the measure covers the first of the two
  sentences only (set 1, `minor`).
- The never-ranks item naming the agent-skills goal on the owner's intent in words: it reads against
  the case that rules out a Component goal ranked above the drift goal, and against the owner's
  statement that a goal never cites a lower Component's goal (sets 1, 3, whole set, `asks`).
- `goal@knowledge-architect@serves-one-kind-of-project`'s new measure: "every condition above"
  against the four conditions; the exception for a state helped into conformance reaching a project
  that is not a target project, and "or repaired" dropped; the four uses of the argument from current
  need that the body keeps (set 2, whole set, `minor`).
- `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: whether a fix to a kept copy
  after the move is a copy "written" then (set 2, `asks`); no measure item for considering at the
  move (set 2, `minor`).
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: whether a task the installed text
  requires is taken in by that alone (set 2, `asks`, old wording).
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: two readings of "would otherwise
  have to derive again", and "any other decision" reaching every implementation choice, with a
  reason in two places (set 4, `asks`).
- `goal@knowledge-architect@records-live-beside-the-code`: whether a retrospective's pending
  project-file findings are outstanding, so record (whole set, `asks`); its measure over the project's
  own homes ranges over what an adopter chooses (set 4, `minor`).
- `goal@knowledge-architect@structure-and-workflow-work-together`: which documents count as "other
  records" citing a home (set 3, `asks`).
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: whether "failed the session"
  means the session's task or what a goal judged in use intends (set 3, `asks`).
- `goal@agent-skills@one-skill-per-activity`: whether the instructions a session gives another agent
  sit in the installed set or in each skill (set 3, whole set, `asks`); the measure does not carry
  them (set 3, `minor`).
- The agent-skills goal on the owner's intent: whether "a review of a goal set" covers a draft-mode
  batch (set 3, `asks`); "a few rounds" as a property of the text, though convergence is judged in use
  (set 3, whole set, `asks`); the stop is a second intent, placed there by an approved default (whole
  set, `minor`).
- `goal@core@documents-stay-current`: "the shape its kind requires", where "kind" also names an
  issue's `kind` field (set 4, whole set, `blocks`); whether a pointer to a section or a directory is
  "a file" (set 4, `asks`, old wording).
- The core's relocation goal: whose change of purpose permits a move, the owner's words saying "of
  the component that justifies it" (set 4, `asks`); its cases read as the checker's obligations where
  its body restricts the project's moves (set 4, `asks`); bringing an existing project's documents
  into conformance at adoption, which the root's adoption goal needs and the "only" rules out (whole
  set, `asks`, a conflict between two homes); no measure for the restriction, and two intents (set 4,
  whole set, `minor`).
- `goal@core@documents-render-as-a-linked-site`: the hosting has no measure (set 4, whole set,
  `minor`); `goal@core@checks-a-commit-from-its-hook`: the measure owes `git commit -a` with paths,
  which git refuses (set 4, `minor`).
- `goal@gates@gates-from-a-list`: the head and the title rule out writing a taken-in behaviour before
  a release ships it, which the body admits; this repository never moves to a released version;
  "until" against "considers following" (set 5, `asks`); "the gates" in the head (set 5, `blocks`,
  old wording).
- `goal@xtask@one-command-runs-every-gate`: the measure ranges over the whole merge verdict, the body
  over the gates' part (set 5, `minor`).

Met outside the goals: the record holds no list of questions still open (whole set); round 2's
finding on a defect provable by reading against a goal judged in use has no outcome (whole set); the
quotation in `milestone@plans@component-split`, and an argument of its plan, rewritten by the slug
rename (set 4), repaired by keeping the old slug with the new reference beside it; the issue on an anchor's own directory quotes
the old relocation measure (set 4).

### Questions put to the owner after round 3, and the owner's answer

Each with its default, put with the `settled` repairs the session makes and the default to stop the
rounds after them:

- Q27, the root head's definition of "the record": the list closes the set.
- Q28, `goal@knowledge-architect@the-owner-decides`: the durable text that records a decision is its
  report.
- Q29, the never-ranks item naming the agent-skills goal in words: dropped; the bullet reads "the
  owner's authority, exchange and attention that [the three goals] give, as the goals that refine
  them state it".
- Q30, the core's relocation goal: (a) the change of purpose is the moved part's, and justifies the
  move; (b) bringing an existing project's documents into conformance at adoption is no relocation
  under it; (c) a measure item, the installed text has a session relocate only on those occasions.
- Q31, `goal@knowledge-architect@design-is-recorded-with-its-arguments`: an entry of its own where a
  comment at one site cannot carry the reason; any other decision whose reason the code does not show
  has it beside the code and in the commit message.
- Q32, `goal@agent-skills@one-skill-per-activity`: the installed set, not each skill, holds the
  instructions a session gives another agent.
- Q33, the agent-skills goal on the owner's intent: (a) "a review of a goal set" covers every review
  of goals in rounds; (b) the measure, a stop on the count of findings, a review the owner stopped by
  word for not converging showing the goal unmet; (c) the stop stays in this goal.
- Q34, `goal@knowledge-architect@the-workflow-improves-through-real-use`: "failed what it is for in
  the session".
- Q35, the copy interval: (a) a fix to a kept copy is maintenance, admitted; (b) in
  `goal@gates@gates-from-a-list`, writing a taken-in behaviour before a release ships it is admitted,
  the copy goes at the project's move, and at the merge in this repository.
- Q36, `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: taking in is the owner's own act.
- Q37, `goal@core@documents-stay-current`: a reference names a goal, an entry of the record, a file,
  a directory or a section of a text.
- Q38, `goal@knowledge-architect@structure-and-workflow-work-together`: any document of the project
  counts as a citing record.

The owner's answer, verbatim:

```text
Agreed on the default, and to stop after this edit.
In conclusion, what would you propose for the stop rule ? Can you remind me the current one ?
```

The rounds stop on the owner's word after round 3's repairs, which no round reviews.

### The stop rule, after observing this review

The owner asked what the session would propose for the stop rule, and was reminded of the two rules
in force: draft mode stopping when a round from the second on reports no `blocks` finding tagged
`settled` on old wording; the whole-set review stopping on the owner's word or when its count of
`blocks` and `asks` findings, over old wording after the first round, falls to a fifth of the first
round's. The session proposed one rule for both modes: stop when a round from the second on reports
no `blocks` finding on old wording, whatever its route, or on the owner's word; put the stop to the
owner when that count does not fall from the previous round's; the fifth dropped, since it fired at
round 2 with three `blocks` findings standing. The owner's answer, verbatim:

```text
Agreed on this proposal for the stop rule.
```

## The ruling

The rounds stopped on the owner's word after round 3's repairs. Every goal is put to the owner as the
tree holds it. Each boundary block below is the whole-set reviewer's of round 3, over the wording
round 3 read; round 3's repairs change a block where Q27 to Q38 change its goal.

### Questions still open

- Whether a retrospective's pending findings on the project's own instructions are "what is
  outstanding", and so record (round 3, whole set, `asks`); Q27 settles the list, not this case.
- The `intent` findings each round kept as worded, which the rounds' sections name, and the rule-4
  `minor` findings Q26 left.

### Boundary blocks

#### docs/goals.md, `goal@knowledge-architect@agents-work-without-drift`

- Asks of future work: catching departures before a merge.
- Requires: a change that contradicts a recorded decision is caught by the checker or by a review.
- Admits: a reversal through the workflow's procedure.
- Rules out: a Component goal ranked above it.
- Cases the owner may not intend: it ranks above `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` and above every Component goal not in the never-ranks list, for instance the mutate and gates goals.
- Siblings: ranks over `goal@knowledge-architect@the-owner-s-word-is-verified`, `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@design-with-agents-as-counterparts` and `goal@knowledge-architect@owner-attention-on-what-is-theirs` (exempted), and over serves-one-kind's asks (exempted). Refined by core `goal@core@relocation-needs-only-goal-changes` and `goal@core@projects-add-their-own-checks`.

#### docs/goals.md, `goal@knowledge-architect@design-is-recorded-with-its-arguments`

- Asks of future work: an entry of its own for each decision whose argument would otherwise be derived again.
- Requires: a recorded decision names its losing alternative.
- Admits: a decision and its designs not kept in separate documents.
- Rules out: a conclusion recorded without its argument.
- Cases the owner may not intend: "any other decision has its reason in the message of the commit that made it, and beside the code or the agent-facing text it shapes" reaches every trivial implementation choice, such as a loop rather than an iterator, and asks a comment beside the code for each one.
- Siblings: `goal@knowledge-architect@structure-and-workflow-work-together` names the same set by other words, "each decision the workflow records in an entry of its own". The two descriptions differ but do not conflict. `goal@knowledge-architect@records-live-beside-the-code`.

#### docs/goals.md, `goal@knowledge-architect@agents-get-a-complete-workflow`

- Requires: the development skill says how to make a test discriminate.
- Admits: the release skill installed in a project that never releases.
- Rules out: Rust-idiom instructions, and an installed instruction outside an offered shape that assumes a pull request.
- Cases the owner may not intend: agent-skills `goal@agent-skills@one-skill-per-activity` counts two more activities as taken in, "maintaining the agent configuration" and "the intake of a retrospective's findings". The root list does not name them. This is no widening, since the root list "grows with the workflow".
- Siblings: refined by `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, `goal@knowledge-architect@the-workflow-leads-to-better-code`, `goal@knowledge-architect@projects-are-split-into-components` and most agent-skills goals.

#### docs/goals.md, `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`

- Requires: a command that folds a fixup with its no-loss check.
- Admits: a task offered both as a command and as a library function.
- Rules out: each session writing its own reword script.
- Cases the owner may not intend: every multi-step deterministic task taken in is owed a command or a function, and the merge predicate is among them.
- Siblings: xtask `goal@xtask@repeated-tasks-are-automated` (agrees), `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

#### docs/goals.md, `goal@knowledge-architect@structure-and-workflow-work-together`

- Requires: the checker lists what a change to a decision touches.
- Admits: a project's own kind of record.
- Rules out: a kind of record written with no home.
- Siblings: refined by core `goal@core@records-reach-their-reader`.
  - The parent's second list is "which outstanding items … a change must read again". The core's is "everything the project records as outstanding": wider, with the filtered listing only admitted (round 1's question).
  - The parent's set of decisions differs in words from that of `goal@knowledge-architect@design-is-recorded-with-its-arguments`.

#### docs/goals.md, `goal@knowledge-architect@records-live-beside-the-code`

- Requires: an outstanding defect kept in a file of the repository.
- Admits: a pull request's discussion kept on the hosting service, with its decisions recorded in the repository.
- Rules out: a decision held only in an external tracker.
- Siblings: the head's definition of "the record"; `goal@knowledge-architect@the-workflow-improves-through-real-use`, which sends the workflow findings to knowledge-architect's owner (agrees).

#### docs/goals.md, `goal@knowledge-architect@documentation-stays-consistent`

- Requires: the check reports a dangling reference.
- Admits: a rule the project declares.
- Rules out: a merge with no review.
- Siblings: refined by core `goal@core@documents-stay-current`, `goal@core@document-rules-are-checked` and `goal@core@checks-a-commit-from-its-hook`.

#### docs/goals.md, `goal@knowledge-architect@the-owner-decides`

- Requires: the order of known work goes to the owner.
- Admits: a merge reserved to the owner; a report in the record, in a commit message, or at the end of the turn.
- Rules out: a question to the owner that a measurement answers.
- Cases the owner may not intend: "these and nothing else" (round 1's question, held).
- Siblings: refined by agent-skills `goal@agent-skills@goals-state-the-owner-s-intent`. Cited by serves-one-kind and generic-tools for asks and for taking in a part.

#### docs/goals.md, `goal@knowledge-architect@the-owner-s-word-is-verified`

- Requires: an agent says a premise is false before it builds on it.
- Admits: a premise checked by a measurement.
- Rules out: work built on an unresolved contradiction.
- Siblings: exempted in the never-ranks list of `goal@knowledge-architect@agents-work-without-drift` (Q17 applied); a merge condition of serves-one-kind.

#### docs/goals.md, `goal@knowledge-architect@design-with-agents-as-counterparts`

- Requires: the agent challenges the owner's design before it builds it.
- Admits: a bare pick.
- Rules out: an open choice reaching main before the owner meets it.
- Siblings: a merge condition of serves-one-kind.

#### docs/goals.md, `goal@knowledge-architect@owner-attention-on-what-is-theirs`

- Requires: measurements taken before a proposal is put to the owner.
- Admits: the owner reading a transcript by choice.
- Rules out: a proposal whose consequences the owner must work out.

#### docs/goals.md, `goal@knowledge-architect@serves-one-kind-of-project`

- Asks of future work: the setup and the READMEs state the four conditions and the asks; nothing built for a state that follows no ask.
- Requires: the setup states linear history.
- Admits: JavaScript glue for a web target.
- Rules out: a checker mode for another language.
- Cases the owner may not intend: `design@core@agents-table`'s `harness = []`, which every core mock project but `unsound` uses, may count as built only for the checker-alone state (the issue opened per Q20).
- Siblings: refined by gates `goal@gates@gates-from-a-list`; cited by `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@every-target-project-can-adopt-it`, `goal@knowledge-architect@setup-brings-quality-tools` and agent-skills `goal@agent-skills@installed-text-works-in-every-target-project`.

#### docs/goals.md, `goal@knowledge-architect@every-target-project-can-adopt-it`

- Requires: one reference form in every project.
- Admits: conformance reached over several branches.
- Rules out: a pin that moves with no edit.
- Siblings: core `goal@core@relocation-needs-only-goal-changes` (F7 below); core `goal@core@document-rules-are-checked` admits "save what a declaration switches off" after "the reference grammar". That admission meets this goal's "the checker enforces one reference grammar in every project", and the exclusion holds.

#### docs/goals.md, `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`

- Requires: a gate one project built and the owner took in ships in a released version.
- Admits: a copy kept until the project's move; a copy kept after the project considered the shipped part (Q22a).
- Rules out: a skill of this repository that restates a shipped procedure.
- Cases the owner may not intend: "which runs the checker it builds" is given as the reason why a part ships at merge, also for parts that are not the checker (a skill, the gates library).
- Siblings: gates `goal@gates@gates-from-a-list` rules out a copy kept past the move, which this goal only admits. That is no conflict under rule 5, and the gates exclusion holds, per Q22b. Also serves-one-kind (F4).

#### docs/goals.md, `goal@knowledge-architect@the-workflow-leads-to-better-code`

- Requires: a code review against the default shape before a merge.
- Admits: a departure with its reason stated.
- Rules out: a fixed quality definition.

#### docs/goals.md, `goal@knowledge-architect@projects-are-split-into-components`

- Requires: the criteria applied when new work is placed.
- Admits: a single-purpose project kept as one Component.
- Rules out: a split rule a Component fails alone.
- Siblings: core `goal@core@relocation-needs-only-goal-changes`. A move that the placement criteria call for, with no change of goals, is ruled out there unless it breaks no check.

#### docs/goals.md, `goal@knowledge-architect@adoption-is-easy`

- Requires: each README states with an example what an adopter does.
- Admits: an adopter who reads the source by choice.
- Rules out: an adoption step stated only in source.

#### docs/goals.md, `goal@knowledge-architect@the-workflow-improves-through-real-use`

- Asks of future work: this repository's own intake instructions judge each case against the goals judged in use.
- Requires: a retrospective records an installed instruction that failed the session.
- Admits: an instruction added on a defect found by reading.
- Rules out: a mock project.
- Siblings: the learning route that Q19 centralised for every goal judged in use; `goal@knowledge-architect@records-live-beside-the-code`.

#### docs/goals.md, `goal@knowledge-architect@setup-brings-quality-tools`

- Requires: the setup gives the commit-message check.
- Admits: a project's own extra tool.
- Rules out: a project that designs its own gate runner.
- Siblings: refined by gates.

#### crates/core/docs/goals.md, `goal@core@documents-stay-current`

- Requires: a merging commit that deletes an entry still cited fails.
- Admits: a dangling pointer in an unread text.
- Rules out: a hand-edited generated listing.

#### crates/core/docs/goals.md, `goal@core@document-rules-are-checked`

- Requires: an entry missing a required part fails.
- Admits: a path outside the check.
- Rules out: a rule held back with no declaration.
- Siblings: `goal@knowledge-architect@every-target-project-can-adopt-it` (see that goal).

#### crates/core/docs/goals.md, `goal@core@relocation-needs-only-goal-changes`

- Requires: a moved Component passes after its declaration changes.
- Admits: a rename by one search and replace. "a few mechanical edits" is not bounded, and the admits case reads it as few operations, not few sites.
- Rules out: a move for tidiness that needs references rewritten by judgement.
- Siblings: refines `goal@knowledge-architect@agents-work-without-drift` (Q15); `goal@knowledge-architect@projects-are-split-into-components`.

#### crates/core/docs/goals.md, `goal@core@records-reach-their-reader`

- Requires: the listing for a decision names a code comment that cites it.
- Admits: a filtered listing.
- Rules out: a list of outstanding items kept by hand.

#### crates/core/docs/goals.md, `goal@core@documents-render-as-a-linked-site`

- Requires: a reference rendered as a link.
- Admits: any rendering tool.
- Rules out: a resolved reference rendered as plain text.
- Not met yet.

#### crates/core/docs/goals.md, `goal@core@projects-add-their-own-checks`

- Requires: a project's naming check in the same report as the built-in ones.
- Admits: an extension one project uses.
- Rules out: an extension that needs a change to the core.

#### crates/core/docs/goals.md, `goal@core@checks-a-commit-from-its-hook`

- Requires: `git commit -a -m` in a linked worktree, on a failing tree, is refused.
- Admits: `--amend` also judged.
- Rules out: a conforming covered commit refused.
- Cases the owner may not intend: options left out of coverage (the owner's earlier ruling).

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@one-skill-per-activity`

- Requires: a review carried out from the review skill.
- Admits: a skill for an activity the project never carries out.
- Rules out: a bundled skill; a skill loaded with no pointer.
- "sub-activity" stays undefined (round 1's question, held).

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@no-contradiction-in-workflow`

- Admits: conflicting instructions that never meet.
- Rules out: opposite outcomes of one finding with no precedence.

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`

- Requires: a commit message judged by the rules binding its writer.
- Admits: a text written outside any skill.
- Rules out: a reviewer's own preference.

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@installed-text-works-in-every-target-project`

- Requires: the checker run the way the project says.
- Admits: silence where judgement handles the case.
- Rules out: a path only this repository holds.
- Siblings: agrees with Q19's split; the retrospective in another project needs none of knowledge-architect's goals.

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@installed-text-works-in-any-environment`

- Requires: the review skill reached with no user-level instruction.
- Admits: the harness's dispatch mechanism.
- Rules out: a plugin's skill taking the installed skill's place.

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@only-needed-instructions`

- Requires: an instruction a goal states stays.
- Admits: an instruction for a case judgement handles poorly.
- Rules out: a fixed count of reviewers.
- Siblings: removing the per-goal retrospective lines from the measures removes acts that this goal counts as needed.

#### crates/agent-skills/docs/goals.md, `goal@agent-skills@goals-state-the-owner-s-intent`

- Requires: "every user" kept.
- Admits: a wider goal shown to the owner.
- Rules out: a goal reworded to name the module that meets it.
- Siblings: refines `goal@knowledge-architect@the-owner-decides`; exempted by `goal@knowledge-architect@agents-work-without-drift` (F1).

#### crates/gates/docs/goals.md, `goal@gates@gates-from-a-list`

- Requires: no project code for logs or distilled failures.
- Admits: a behaviour kept until the project's move.
- Rules out: a project's own runner, and a copy kept past the move. The second is an exclusion over what generic-tools admits; it holds, per Q22b.

#### crates/mutate/docs/goals.md, `goal@mutate@a-named-mutation-is-checked-safely`

- Requires: uncommitted edits restored byte for byte.
- Admits: any test command.
- Rules out: the crate proposing a mutation from the diff.
- Cases the owner may not intend: several files are required by the measure (round 1's M4, held).

#### tools/xtask/docs/goals.md, `goal@xtask@one-command-runs-every-gate`

- Requires: the merge check runs this command.
- Admits: one gate run by hand.
- Rules out: another list of gates.

#### tools/xtask/docs/goals.md, `goal@xtask@repeated-tasks-are-automated`

- Requires: the reword no-loss check is a command.
- Admits: a command for a one-off task.
- Rules out: the merge-predicate checks typed one by one.

#### crates/git/docs/goals.md, `goal@git@a-pure-wrapper-over-git`

- Requires: folding a fixup as one operation.
- Admits: a tool that runs git itself.
- Rules out: a library function that chooses which commits to fold.

### The owner's ruling

The owner ruled on every goal as the tree holds it, the open question's default with it. The
owner's words, verbatim:

```text
agreed on current goals, proceed
```
