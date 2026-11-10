# Goals — agent skills

The agent skills are the text knowledge-architect installs in a target project for its agents: the
workflow that the root's goals name.

## Every activity of a target project whose generic part knowledge-architect's owner has taken in has one installed skill `##one-skill-per-activity`

An activity is the scope over which a complete set of procedures makes sense, a sub-activity
included. Two tasks whose methodologies differ, their steps taken as a whole, are two activities,
even where they share part of a name, and are not bundled in one skill. Where the two tests
disagree, the methodologies decide.

The installed set holds one skill for each activity of a target project whose generic part
knowledge-architect's owner has taken in, as
`goal@knowledge-architect@agents-get-a-complete-workflow` defines a generic part, the activities
listed below counting as taken in. The installed set, not each skill, holds every instruction each
activity needs, those a session gives another agent included. An agent carries out each activity
from its skill, and reads another installed skill only at a step that points to it, a moment at
which the installed text says to load that skill included. The two are one intent, since a skill per activity serves only where the
activity is carried out from it.

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
- auditing the whole project on one aspect;
- maintaining the agent configuration;
- the retrospective;
- the intake of a retrospective's findings.

Running continuous integration or pull requests is no activity this goal counts: its generic part
ships only as an offered shape, per `goal@knowledge-architect@agents-get-a-complete-workflow`.

It is met while the installed set holds a skill for each such activity and no skill for two of them,
holds every instruction a session gives another agent in each such activity, and each skill:

- states every step its activity needs, leaving to judgement what judgement handles;
- points to another installed skill only at a step that needs it.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: a session carries out a review from the installed review skill.
- Admits, for instance: a skill installed for an activity a project never carries out, such as
  releasing in a project that never releases.
- Admits, for instance: an installed agent definition that several skills dispatch, kept as one
  file of the installed set.
- Rules out, for instance: one skill that bundles planning and reviewing.
- Rules out, for instance: a session that loads another installed skill on its own judgement, at a
  step that points to none.

## No two installed instructions contradict `##no-contradiction-in-workflow`

The installed text, every part of it, gives no two instructions a session cannot both obey. A
precedence an installed text states between the two leaves the session the move the prevailing one
asks, which counts as obeying both.

It is met while every pair of installed instructions that meet at one moment leaves a session a move
that obeys both, the move a precedence names counting as one. A pair found in use that leaves none
shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: where one skill says to commit and another says not to commit while a
  check fails, the installed text leaves a session a move that obeys both.
- Admits, for instance: two instructions that would conflict if they met, which never meet at one
  moment.
- Rules out, for instance: two installed skills that, at one review, ask for opposite outcomes of
  the same finding, with no precedence between them.

## A review or an audit judges a text by the criteria its writer is asked to meet `##reviews-judge-by-the-writer-s-criteria`

A review or an audit the installed text owes or offers judges a text written under a skill,
installed or the project's own, by the criteria that bind its writer at the time of the review, and
by no other. Those criteria are:

- the installed text;
- the project's root `CLAUDE.md`, its skills, and the scoped `CLAUDE.md` beside what is edited;
- the goals and the record;
- the external specifications that any of these make binding;
- the plans the work answers, and the owner's statements and rulings it carries out, as far as each
  binds the writer.

The criteria of the reviews and of the audits agree with each other.

It is met while each criterion that a review or an audit the installed text owes or offers applies
to such a text is one that binds its writer at that time.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: a reviewer judges a commit message by the rules that bind its writer at
  the time of the review.
- Admits, for instance: a review that judges, by the same criteria, a text written outside any
  skill.
- Rules out, for instance: a reviewer judging a document by a style the reviewer prefers and no
  criterion states.

## A review judges whether what bears on the work reaches it and is followed `##reviews-judge-what-no-tool-can`

The review the installed text owes before a merge asks, of the work it reads, two questions of meaning: whether each recorded item that bears on the work is cited where it bears and a checked reference may stand,
and whether the work departs from the goals, the decisions and the reasons recorded beside its code and its
agent-facing text that bear on it, or brings back a design the record did not keep. The
checker and the commands that list the recorded items carry the mechanical part, and passing them answers neither.

It is met while the installed text owes that review, both questions named, before every merge into the main
branch, and asks in its retrospective how it fared.

It refines `goal@knowledge-architect@records-reach-their-reader` and
`goal@knowledge-architect@documentation-stays-consistent`.

- Requires, for instance: a reviewer reports code that departs from a recorded decision, every reference of
  it resolving.
- Admits, for instance: a reviewer that starts from a listing of what cites an entry.
- Rules out, for instance: a review passed because the check passed.

## An installed text works unchanged in every target project that installs it, one that holds none of knowledge-architect's goals and record included `##installed-text-works-in-every-target-project`

A target project other than knowledge-architect that installs the workflow holds, in its repository,
none of knowledge-architect's goals and record, its Components' included: none of the reasons
recorded beside its code and its agent-facing text, those in the source of the installed text
included, and none of its commit messages. It is not expected to hold many goals or decisions of its
own about its agent-facing text: knowledge-architect carries most of what concerns that text, in
the instructions the installed text gives, and the project adds what its own activities need. Where
it holds many, this goal holds all the same.

The installed text works unchanged in every target project that installs it, however the project
arranges its documents, as far as the project follows what knowledge-architect asks:

- it names no path or convention particular to one target project, and may rely on what
  knowledge-architect asks of every target project, as the installed text states it, per
  `goal@knowledge-architect@serves-one-kind-of-project`; it runs the checker the way the project
  says to run it, or the default way where the project says nothing;
- each of its instructions can be followed, and gives the result it intends, with what that
  project holds: the installed text, the project's own instructions, goals and record, its code and
  its measurements, the sources it follows, knowledge-architect's published documentation among
  them, and its owner;
- that holds where the project's own goals and record say nothing about an instruction's subject.

It is met while the installed text names no such path or convention, runs the checker that way, and
holds no instruction that needs any of knowledge-architect's goals and record to be followed or to
give the result it intends; while each instruction whose subject a project's own goals and record
can leave unsettled says what it does in that case, unless the session's judgement handles the case
well enough, as `goal@agent-skills@only-needed-instructions` puts it.

It refines `goal@knowledge-architect@every-target-project-can-adopt-it`.

- Requires, for instance: an installed skill that has a session run the checker runs it the way the
  project says to.
- Admits, for instance: an instruction that says nothing of the case where the project's record is
  silent, where the session's judgement handles that case well enough.
- Rules out, for instance: an installed instruction that names a directory only
  knowledge-architect's repository holds.

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
one for the same activity. A session
routed otherwise because of its configuration's own instructions shows the goal unmet in practice.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: a session with no user-level instruction loads the installed review skill
  before a merge.
- Admits, for instance: an installed skill that relies on the harness's own mechanism for
  dispatching an agent.
- Rules out, for instance: a plugin's design skill taking the place of the installed one because a
  user-level instruction names it.

## The installed text holds only the instructions the workflow needs, and leaves to the session's judgement the cases it handles well enough `##only-needed-instructions`

The installed text states what an agent needs to carry out the workflow as intended.

It does not try to cover every case. A case the judgement of the session following it handles well
enough gets no instruction, since each instruction added for a specific case is read by every
session that loads the text, and can conflict with the instructions already there.

Nor does an instruction impose structure the work does not need: a fixed sequence, a fixed count, or
a fixed mapping between a unit of the work and a unit of what the work produces, such as a commit or
a section of a document, that the work can be carried out as intended without, and that another
instruction needs to vary. The clause bears on what an agent decides by its judgement in carrying
out its work, such as the shapes it proposes in the design of a solution, where a fixed structure
narrows what is proposed. It does not bear on a command an agent runs, on that command's output, or
on a fixed sequence of commands to run, nor on a structure that a command or the checker carries
out, or that a mechanical check the work must pass refuses.

An instruction or a structure whose act the text of a goal of knowledge-architect states, in its
body or its measure, every Component's goals whose measure is the installed text included, is
needed, and judgement does not take its place. A category or an example that a goal names, such as
"a kind of reviewer" among the tools a target project sets up, makes no instruction needed.

It is met while each installed instruction names an act the workflow needs, and the installed text
imposes on what an agent decides by its judgement in carrying out its work no fixed sequence, fixed
count, or fixed mapping between a unit of the work and a unit of what the work produces, such as a
commit or a section of a document, that the work can be carried out as intended without and that
another instruction needs to vary, other than one that a command or the checker carries out, or that
a mechanical check the work must pass refuses. A review that finds an instruction for a case
judgement handles, or such a structure, other than one whose act a goal's text states, shows the
goal unmet in practice; it is answered by removing that instruction or that structure rather than by
adding a rule. The retrospective asks whether an installed instruction its session met covered a
case the session's judgement handles well enough, or imposed such a structure.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: an instruction whose act a goal's text states stays, even where sessions
  often do that act unprompted.
- Admits, for instance: an instruction for a case the session's judgement handles poorly.
- Admits, for instance: a fixed shape of a record search's report, which a command checks and
  refuses when it is not met.
- Admits, for instance: a fixed step of the design procedure that no other instruction needs to
  vary.
- Rules out, for instance: a fixed count of reviewers per review that a review can be carried out as
  intended without, and that another instruction needs to vary.

## A goal written under the installed text states the owner's intent as the owner holds it `##goals-state-the-owner-s-intent`

In a target project, a goal written or reworded under the installed text states the outcome the
owner intends, at the breadth the owner gave it. It is drafted to move what the project builds, so
agents work in its direction without changing it, and a new goal is expected to be unmet. It keeps
its meaning while the work moves toward it, whatever is designed or built to reach it. It requires
and rules out only what the owner has required or ruled out knowingly, and a review of it never
narrows or widens it past that without the owner's word. Every review of goals that runs in rounds,
of a batch of drafts or of the whole goal set, stops after a few rounds, at a small number of
findings. What shows it met is in what the project builds, or, where
its outcome depends on what the project does not control, in what the project builds to learn from
use whether it is met.

It is met while the installed text has each goal drafted to these properties and judged against
them, by a reviewer that reads the goal and the owner's statements before the owner rules, and puts
each finding on a goal's reach to the owner instead of narrowing the goal, and gives each review of
goals in rounds a stop on the count of its findings. A goal the owner finds narrower, wider or more
tied to a built part than the owner meant, after it was ruled, or a review of goals the owner
stopped by word because it did not converge, shows the goal unmet in practice.

It refines `goal@knowledge-architect@the-owner-decides`.

- Requires, for instance: a goal drafted from the owner's statement that the project serves "every
  user" keeps "every user", unless the owner rules a narrower set.
- Admits, for instance: a goal wider than each of the owner's statements it is drawn from, whose
  reach the owner was shown before ruling.
- Rules out, for instance: a goal reworded by a review to name the module that currently meets it.

## The main session of a piece of work does itself what needs it, and hands the rest to agents it dispatches `##session-orchestrates-the-work`

The main session is the session that the person supervising the work exchanges with. It does
itself what needs it:

- the design and the goals that it discusses with that person;
- the decisions on the design, but those a dispatched agent meets in its own part of the work, and
  the questions put to that person;
- the writing of the plan documents that record what was designed;
- the dispatch of the rest of the work, and acting on what comes back.

The rest of a piece of work is carried by agents the main session dispatches: the investigation,
the grounding in the record and in the code, the experiments and the measurements, the writing of
the record, the development, the updates of a plan document while its work is carried out, and the
review. The main session may also dispatch the check that another agent's conclusions hold.

The main session dispatches an agent for one coherent part of the work, and that agent carries the
whole of it: an agent that writes a change also writes its commits, and may make several in a row
toward the same change. What makes a coherent part, how much of the work one agent carries, and
whether a part is worth a dispatch, are left to the main session's judgement, by the size of the
work, the size of the record it reads and the scope of the action. A coherent part too small to
spare the main session's context, such as the reword of one commit message to repair a claim, is
done by the main session itself; several such repairs taken together can make one part worth a
dispatch.

Each such agent checks its conclusions before it reports them, as the main session checks its own
claims, no more and no less, and reports them with the evidence each one rests on, in a form that
names what it considered and can be checked without doing the work again. A stricter check is a
check against another standard than the one the main session applies to its own claims, or a chain
of checks, each checking the check before it. A check of another agent's conclusions against their
evidence is no chain: a chain begins only with a check of that check. A review judges the work, not
the check before it: it is no stricter check, it is a part of the work like any other, and how often
the work is reviewed is a way of dividing it, left to the main session's judgement where the
installed text sets no rule, such as a review before a merge. The check of a dispatched agent's
conclusions does not make the total cost of the work significantly higher than the same dispatched
work without that check.

It is met while the installed text keeps with the main session each part listed above as its own,
has it dispatch the rest at its judgement, and states for each dispatched part the report its agent
returns and how the agent checks its conclusions; and while the retrospective asks whether the main
session spent its context on work an agent could have carried, whether it acted on a conclusion not
checked as its own claims are, or on one that proved wrong, and whether the check of dispatched
conclusions raised the total cost significantly. A conclusion acted on after its check passed,
which then proves wrong, shows the goal unmet in practice where the form of the report did not let
what went wrong be found.

It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

- Requires, for instance: an agent that reads the project's record against the work returns, for
  each entry it judges, its conclusion with evidence that can be checked without reading the entry
  whole, and names every entry it was given and which of them it judged.
- Requires, for instance: where the main session judges a change worth a dispatch, its code is
  written by a dispatched agent and judged by a dispatched review, and the main session acts on the
  review's findings.
- Admits, for instance: the main session reading one known file, or rewording one commit message,
  itself.
- Admits, for instance: a dispatched developer making several commits in a row toward one change,
  each with its own message.
- Admits, for instance: a dispatched developer deciding a choice its change meets and reporting it,
  as the installed text says which choices return to the main session.
- Admits, for instance: a review dispatched after each step of a dispatched developer's change.
- Admits, for instance: an instruction that has a dispatched agent check another agent's
  conclusions against their evidence before the main session acts on them.
- Rules out, for instance: an instruction that has the main session read whole every entry a
  search returns, where it acts on few of them.
- Rules out, for instance: the main session acting on an agent's conclusion that was not checked as
  its own claims are.
- Rules out, for instance: an instruction that holds a dispatched agent's report to a stricter check
  than the main session's own claims.
- Rules out, for instance: the main session taking over a part of a dispatched agent's coherent part
  of the work, such as its commit, because that part alone is small, while that agent carries its
  part.
