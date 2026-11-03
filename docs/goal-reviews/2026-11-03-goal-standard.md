# Review of the whole goal set under the revised goal standard

The record of `skill@knowledge-architect-goal-setting@whole-set-review` over this repository's
goals, after the revision of the goal standard. No round has run yet. This file holds, until the
first round, what the review must read and do, which the owner ruled while the standard was
revised.

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
