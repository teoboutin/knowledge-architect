# Goals — agent skills

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@agent-skills@design`. A goal stays here while it is met, and leaves only when the
owner abandons it. This component is published, so its goals are encouraged to refine a goal of the
project as a whole, `register@knowledge-architect@goal`.

## Every activity of the workflow has one installed skill `##one-skill-per-activity`

An activity is the scope over which a complete set of procedures makes sense, a sub-activity
included, as `skill@knowledge-architect-agent-configuration@where-text-goes` uses the term. Two
tasks whose methodologies differ, their steps taken as a whole, are two activities, even where they
share part of a name, and are not bundled in one skill. Where the two tests disagree, the
methodologies decide.

The installed set holds one skill for each activity the workflow asks an agent to carry out in every
project that installs it, writing the code itself excepted, and the agent definitions those skills
dispatch. An agent carries out each activity from its skill, and reads another installed skill only
at a step that points to it, a moment of the primer's list of installed skills included. The two are
one intent, since a skill per activity serves only where the activity is carried out from it.

The activities include these, and the list grows with the workflow:

- setting up a project;
- setting its goals;
- discussing designs;
- planning;
- recording decisions;
- tracking open issues;
- reviewing;
- auditing a project on one aspect by the project audit's method;
- maintaining the agent configuration;
- the retrospective;
- the intake of a retrospective's findings.

It is met while the installed set holds a skill for each such activity and no skill for two of them,
and each skill:

- states every step its activity needs, leaving to judgement what judgement handles;
- points to another installed skill only at a step that needs it.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## No two installed instructions contradict `##no-contradiction-in-workflow`

The installed texts, the skills, the agent definitions, the saved workflows and the primer, give no
two instructions a session cannot both obey. A precedence an installed text states between the two
leaves the session the move the prevailing one asks, which counts as obeying both.

It is met while every pair of installed instructions that meet at one moment leaves a session a move
that obeys both, the move a precedence names counting as one. A pair found in use that leaves none
shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## A review or an audit judges a text by the criteria its writer is asked to meet `##reviews-judge-by-the-writer-s-criteria`

A review or an audit the installed text owes or offers judges a text written under a skill,
installed or the project's own, by the criteria that bind its writer at the time of the review, and
by no other. Those criteria are:

- the installed text;
- the project's root `CLAUDE.md`, its skills, and the scoped `CLAUDE.md` beside what is edited;
- the reasons recorded beside the code and the agent-facing text, and in the messages of the commits
  that made them;
- the goals, the design homes, the rejected designs, the tripwires and the issues;
- the external specifications that any of these make binding.

The criteria of the reviews and of the audits agree with each other.

It is met while each criterion that a review or an audit the installed text owes or offers applies
to such a text is one that binds its writer at that time.

## An installed text works unchanged in any project that installs it `##installed-text-works-anywhere`

The installed skills, agent definitions, saved workflows and primer name no path or convention
particular to one adopting project, and write the command by which the project runs the checker, as
its manifest declares it, or `klarch` where it declares none.

It is met while they name none, and write that command, so that a project of any language and of any
layout its manifest declares can follow them without editing them.

It refines `goal@knowledge-architect@any-project-can-adopt-it`.

## An installed text works in any environment that loads it `##installed-text-works-in-any-environment`

The installed text depends on no skill or instruction outside the installed set and the project; it
may rely on the harness's own mechanisms. In any environment that loads it:

- a session with no user-level instruction and no plugin reaches each installed skill at its moment;
- a skill or an instruction the environment adds, outside the installed set and the project, the
  harness's own included, never takes the place of an installed skill or instruction for an activity
  the set covers.

It is met while the installed text routes a session to each installed skill at its moment, and no
installed text lets a skill or an instruction the environment adds take the place of an installed
one for the same activity. A session routed otherwise because of its environment's own instructions
shows the goal unmet in practice.

## The installed text holds only the instructions the workflow needs, and leaves to the session's judgement the cases it handles well enough `##only-needed-instructions`

The installed text, its skills, agent definitions, saved workflows and primer, states what an agent
needs to carry out the workflow as intended, with the reason for each instruction.

It does not try to cover every case. A case the judgement of the session following it handles well
enough gets no instruction, since each instruction added for a specific case is read by every
session that loads the text, and can conflict with the instructions already there.

Nor does it impose structure the work does not need: a fixed sequence, count or mapping between a
unit of the work and a unit of its record that another instruction needs to vary.

It is met while each installed instruction names an act the workflow needs, and the installed text
imposes no fixed sequence, count or mapping between a unit of the work and a unit of its record that
another instruction needs to vary. A review that finds an instruction for a case judgement handles,
or a structure another instruction needs to vary, shows the goal unmet in practice; it is answered
by removing that instruction or that structure rather than by adding a rule.

It refines `goal@knowledge-architect@agents-work-without-drift`.
