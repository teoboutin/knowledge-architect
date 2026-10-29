# Goals — agent skills

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@agent-skills@design`. A goal stays here while it is met, and leaves only
when the owner abandons it. This component is published, so its goals are encouraged to refine a
goal of the project as a whole, `register@knowledge-architect@goal`.

## Every activity of the workflow has one installed skill `##one-skill-per-activity`

The installed set holds one skill for each activity the workflow asks an agent to carry out in
every project that installs it, and the agents those skills dispatch. The activities include
setting up a project and its goals, discussing designs, planning, recording decisions, tracking
open issues, reviewing, auditing a project, maintaining the agent configuration, the retrospective,
and the intake of a retrospective's findings; the list grows with the workflow. It is met while no
such activity is left without its skill, and an agent carries out each activity from its skill,
reading another skill only at a step that points to it. It refines
`goal@knowledge-architect@agents-get-a-complete-workflow`.

## Two installed texts never contradict `##no-contradiction-in-workflow`

The installed texts (the skills, the agent definitions, the saved workflows and the primer) give
no two instructions a session cannot both obey. A review or an audit judges a text by the criteria
the skill of its activity asks the writer to meet, and by no other, and the criteria of the reviews
and of the audits agree with each other. It is met while no review and no audit finds two installed
instructions a session cannot both obey, or a criterion of a review or an audit that the activity's
skill does not state. It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## An installed text works unchanged in any project that installs it, and in any environment that runs it `##installed-text-works-anywhere`

The installed skills, agents and primer name no project's paths or conventions, and write the
project's command as the project declares it. They depend on nothing the install does not write: a
session with no user-level instruction and no plugin reaches each installed skill at its moment, and
a skill or an instruction the environment adds outside the installed set never replaces an installed
one for an activity the set covers. It is met while a project of any language and layout can follow
them without editing them, and while a session is routed through the workflow the same way with or
without the environment's own instructions. It refines `goal@knowledge-architect@any-project-can-adopt-it`.

## The installed text holds only the instructions the workflow needs, and leaves to the session's judgement the cases it handles well enough `##only-needed-instructions`

The installed skills and agents state what an agent needs to carry out the workflow as intended,
with the reason for each instruction. They do not try to cover every case: a case the judgement of
the session following them handles well enough gets no instruction, since each instruction added
for a specific case is read by every session that loads the text and can conflict with the
instructions already there. Nor do they impose structure the work does not need: a fixed sequence,
count or mapping between a unit of the work and a unit of its record. It is met while each installed
instruction names an act the workflow needs, and a review that finds an instruction for a case
judgement handles, or a structure another instruction needs to vary, is answered by removing it
rather than by adding a rule. It refines `goal@knowledge-architect@agents-work-without-drift`.
