# Goals — agent skills

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@agent-skills@design`. A goal stays here while it is met, and leaves only when the
owner abandons it. This component is published, so its goals are encouraged to refine a goal of the
project as a whole, `register@knowledge-architect@goal`.

## Every activity of a target project whose generic part knowledge-architect's owner has taken in has one installed skill `##one-skill-per-activity`

An activity is the scope over which a complete set of procedures makes sense, a sub-activity
included, as `skill@knowledge-architect-agent-configuration@where-text-goes` uses the term. Two
tasks whose methodologies differ, their steps taken as a whole, are two activities, even where they
share part of a name, and are not bundled in one skill. Where the two tests disagree, the
methodologies decide.

The installed set holds one skill for each activity of a target project whose generic part
knowledge-architect's owner has taken in, as
`goal@knowledge-architect@agents-get-a-complete-workflow` defines a generic part, the activities
listed below counting as taken in, and the agent definitions those skills dispatch. An agent carries
out each activity from its skill, and reads another installed skill only at a step that points to
it, a moment of the primer's list of installed skills included. The two are one intent, since a
skill per activity serves only where the activity is carried out from it.

The activities include these, and the list grows with the workflow:

- setting up a project;
- setting its goals;
- discussing designs;
- planning;
- recording decisions;
- tracking open issues;
- developing;
- working on a branch under a linear history;
- releasing;
- reviewing;
- auditing a project on one aspect by the project audit's method;
- maintaining the agent configuration;
- the retrospective;
- the intake of a retrospective's findings.

Running continuous integration or pull requests is no activity this goal counts: its generic part
ships only as an offered shape, per `goal@knowledge-architect@agents-get-a-complete-workflow`.

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
- the external specifications that any of these make binding;
- the plan documents the work answers, and the owner's statements and rulings it carries out, as far
  as each binds the writer.

The criteria of the reviews and of the audits agree with each other.

It is met while each criterion that a review or an audit the installed text owes or offers applies
to such a text is one that binds its writer at that time.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## An installed text works unchanged in every target project that installs it, one that holds none of knowledge-architect's goals and record included `##installed-text-works-in-every-target-project`

A target project other than knowledge-architect that installs the workflow holds, in its repository,
none of knowledge-architect's goals and record: none of the goals, design heads, rejected designs,
tripwires or issues of its Components, none of the reasons recorded beside its code and its
agent-facing text, the `%%` comments of the installed text's source included, and none of its commit
messages. It is not expected to hold many goals or design heads of its own about its agent-facing
text: knowledge-architect carries most of what concerns that text, in the instructions the installed
text gives, and the project adds what its own activities need. Where it holds many, this goal holds
all the same.

The installed skills, agent definitions, saved workflows and primer work unchanged in every target
project that installs them, of any layout its manifest declares, as far as the project follows what
knowledge-architect asks:

- they name no path or convention particular to one target project, and may rely on what
  knowledge-architect asks of every target project, as the installed text states it, per
  `goal@knowledge-architect@serves-one-kind-of-project`; they write the command the project declares
  for running the checker, or the checker's default where it declares none;
- each of their instructions can be followed, and gives the result it intends, with what that
  project holds: the installed text, the project's own instructions, goals and record, its code and
  its measurements, the sources it follows, knowledge-architect's published documentation among
  them, and its owner;
- that holds where the project's own goals and record say nothing about an instruction's subject.

It is met while the installed text names no such path or convention, writes that command, and holds
no instruction that needs any of knowledge-architect's goals and record to be followed or to give
the result it intends; and while each instruction whose subject a project's own goals and record
can leave unsettled says what it does in that case, unless the session's judgement handles the case
well enough, as `goal@agent-skills@only-needed-instructions` puts it.

It refines `goal@knowledge-architect@every-target-project-can-adopt-it`.

## An installed text works in any configuration of the `claude` harness that loads it `##installed-text-works-in-any-environment`

The installed text depends on no skill or instruction outside the installed set and the project; it
may rely on the harness's own mechanisms. In any configuration of the `claude` harness that loads
it, user-level instructions and plugins included:

- a session with no user-level instruction and no plugin reaches each installed skill at its moment;
- a skill or an instruction the configuration adds, outside the installed set and the project, the
  harness's own included, never takes the place of an installed skill or instruction for an activity
  the set covers, unless an installed instruction itself defers to it.

It is met while the installed text routes a session to each installed skill at its moment, and no
installed text lets a skill or an instruction the configuration adds take the place of an installed
one for the same activity. A session routed otherwise because of its configuration's own
instructions shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## The installed text holds only the instructions the workflow needs, and leaves to the session's judgement the cases it handles well enough `##only-needed-instructions`

The installed text, its skills, agent definitions, saved workflows and primer, states what an agent
needs to carry out the workflow as intended.

It does not try to cover every case. A case the judgement of the session following it handles well
enough gets no instruction, since each instruction added for a specific case is read by every
session that loads the text, and can conflict with the instructions already there.

Nor does it impose structure the work does not need: a fixed sequence, count or mapping between a
unit of the work and a unit of its record that another instruction needs to vary.

An instruction or a structure whose act the text of a goal of knowledge-architect states, in its
body or its measure, every Component's goals whose measure is the installed text included, is
needed, and judgement does not take its place. A category or an example that a goal names, such as
"a kind of reviewer" among the tools a target project sets up, makes no instruction needed.

It is met while each installed instruction names an act the workflow needs, and the installed text
imposes no fixed sequence, count or mapping between a unit of the work and a unit of its record that
another instruction needs to vary. A review that finds an instruction for a case judgement handles,
or a structure another instruction needs to vary, other than one whose act a goal's text states,
shows the goal unmet in practice; it is answered by removing that instruction or that structure
rather than by adding a rule.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.
