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

## An installed text works unchanged in any project that installs it `##installed-text-works-anywhere`

The installed skills, agents and primer name no project's paths or conventions, and write the
project's command as the project declares it. It is met while a project of any language and layout
can follow them without editing them. It refines `goal@knowledge-architect@any-project-can-adopt-it`.

## The installed text leaves agents room to judge `##installed-text-leaves-room-to-judge`

The installed skills and agents give agents capabilities and the reasons for each instruction, and
impose no structure the work does not need: no fixed sequence, count or mapping between a unit of
the work and a unit of its record where judgement would serve. It is met while no installed
instruction fixes such a mapping that another instruction needs to vary, and a review that finds
one is answered by removing structure rather than by adding a rule. It refines
`goal@knowledge-architect@agents-work-without-drift`.
