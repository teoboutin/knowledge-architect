---
kind: todo
---
# The installed text has the main session carry work it could dispatch, and says nothing of how a dispatch is judged

## Summary

`goal@agent-skills@session-orchestrates-the-work` has the main session keep the design, the goals,
the decisions on the design, the questions put to the person supervising the work, the writing of
the plan documents and the dispatch, and hand the rest to agents it dispatches, at its judgement.
The installed text as it stands has the main session carry the development itself, and gives no guidance on when a dispatch is worth it; the retrospective
asks none of the goal's questions. No plan document schedules the work.

## Details

### What

Where the installed text departs from the goal, as it stands at the commit that wrote the goal:

- the development: `skill@knowledge-architect-development` dispatches the review axes, and has the
  session that grounds the change write its code; no text dispatches the implementation of a
  coherent part of a change to an agent that also makes its commits;
- the investigation at the start of a design discussion: `skill@knowledge-architect-design` loop
  step 1 has the session read the homes and grep the record itself, and loop step 5 builds
  discriminating evidence with no word on dispatching it, nor on dispatching agents that look for
  another shape from a fresh point of view;
- the judgement of a dispatch: neither the primer nor any skill states when a part of the work is
  worth a dispatch. The owner's leads for it are under "Leads from the owner" below;
- the report of a dispatched agent: the goal asks that each report name what it considered and be
  checkable without doing the work again, and that the agent check its conclusions as the main
  session checks its own claims; no installed text states this for every dispatch;
- the retrospective: `skill@knowledge-architect-retrospective` asks none of the goal's questions,
  whether the main session spent its context on work an agent could have carried, whether it acted
  on a conclusion not checked as its own claims are or on one that proved wrong, and whether the
  check of dispatched conclusions raised the total cost significantly.

### Leads from the owner

What the owner said, while the goal was drafted, about when to dispatch and what for. These are
leads for the design of the repairs, not rulings: each is argued again where it is built, and the
owner's hedges are kept.

When a dispatch is worth it, which the owner placed in design rather than in the goal:

- the instructions bind where they speak: where they say to dispatch, the session dispatches, where
  they say not to, it does not, and only where they are silent does it judge;
- the owner's example conditions for that judgement: whether the next coherent part of the work is
  large enough to spend so much of the session's context that finishing the whole piece of work at
  hand, such as the slice, becomes hard; whether a subject needs investigating where only the
  conclusions matter, or a large code probe needs writing; whether a fresh point of view is needed.
  The owner believes these are close to the examples of their user-level instructions for agents,
  `path@elsewhere@~/.claude/CLAUDE.md`, section "Using subagents", which lists as likely worth
  delegating an investigation of a bug or a question, a survey of prior art, a review of the
  session's own work by a fresh agent rather than a fork, and mechanical work that is repetitive and
  parallelizable, and as usually not worth it a lookup whose target is known, work whose
  intermediate detail the session needs later, and the edits of a single coherent change;
- the owner suggested the primer as the home of this judgement, which would probably meet the goal
  in good part, and reported already seeing sessions dispatch in most of these cases. Where it did
  not apply yet: the implementation, and the investigation during design, where the owner believes
  that dispatching a few agents to try things out, or to take a fresh point of view in search of
  another solution, could improve the results.

What a dispatch carries:

- the unit of a dispatch is one coherent piece of work, and the agent carries all of it: the agent
  that writes code writes its commit too, and may make several commits in a row toward the same
  piece of work. The main session does not interrupt a dispatched agent to take over a task that is
  part of the dispatched work, such as its commit, because that task alone is small;
- what makes a coherent piece, and how much of the work one agent carries, are the session's
  judgement, by the size of the work and of the record it reads and by the scope of the action, not
  by size alone. A commit message alone is nearly never worth a dispatch, since it goes with a
  change written in the project. The reword of one earlier commit message to repair a claim is one
  coherent change that the main session can carry, unless it is bundled with other repairs made at
  the same time, which together make one larger piece a dispatch can handle;
- writing a plan document is probably the main session's work, since it knows best what was
  designed; acting on the plan is a dispatched agent's;
- checking the work is a dispatched agent's, and even checking that reported findings are real can
  be dispatched;
- a dispatched developer meeting a choice that earns a design head follows the procedure the
  installed text already gives, carried by that agent instead of the main session; the owner sees
  no reason to refuse or change it, and would argue for that design;
- how often the work is reviewed is a way of dividing the work, not a stricter check, and stays the
  session's judgement wherever no clearly good absolute criterion applies, such as a review at least
  at the merge. Reviews are part of the normal work.

Why the owner wants it, which bears on how each repair is argued:

- the main session focuses on design, decisions and following the goals, while each dispatched
  agent starts fresh and reads the instructions related to its task as it needs them, which the
  owner expects to improve its efficiency and its following of instructions;
- the owner holds the main session as reliable as any dispatched agent, its only added value being
  that it exchanges with the owner and reports to the owner. So the main session's check of a
  dispatched agent's work is neither stronger nor weaker than its check of its own claims, and above
  all these checks should not raise the total cost significantly, though the owner finds that hard
  to measure;
- for the record search of the design discussion held beside the goal: the owner judged it not
  worth that session reading whole the entries the search grades, and proposed giving its searchers
  more instructions and trying the verdict shape the discussion proposes, even without the checked
  verification.

### Why it matters

The goal is unmet while these stand, against `goal@agent-skills@session-orchestrates-the-work` and
the efficiency `goal@knowledge-architect@agents-get-a-complete-workflow` asks.

### What would close it

Each departure above repaired in the installed text, through `skill@knowledge-architect-design`
where a repair decides a design and `skill@knowledge-architect-agent-configuration` for the text:
a dispatch of implementation in the development
skill; the dispatch judgement stated where the design discussion puts it; the report standard of a
dispatched agent; and the retrospective's questions. The entry closes when the goal's "It is met
while" clause holds of the installed text.
