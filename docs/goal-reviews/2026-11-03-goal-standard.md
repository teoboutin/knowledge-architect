# Review of the whole goal set under the revised goal standard

The record of `skill@knowledge-architect-goal-setting@whole-set-review` over this repository's
goals, after the revision of the goal standard. No round has run yet. This file holds, until the
first round, what the review must read and do, which the owner ruled while the standard was
revised.

## Before the first round

What the rounds owe, beyond the procedure:

- **The branch merges only after rounds of this review.** The owner: "merge can only happen after we
  fix this under multiple rounds of the whole set review."
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
