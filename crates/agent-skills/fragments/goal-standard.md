%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`. Each rule's cases below were
%% read in the goal histories of rootpoint, thaum and knowledge-architect; the rules as a whole are
%% argued in `design@agent-skills@goal-standard-rules`, which names the head of each rule that has one.
**A goal is proper when all of these hold.** The drafter writes to them, and the goal reviewer
judges by them.

%% The three layers keep an admitted case from being read as a requirement, which made a goal that
%% admits what another rules out read as a contradiction, and a reach finding read as a defect, per
%% `design@agent-skills@requires-admits-rules-out`.
**A goal is read in three layers**: what it **requires**, what it **admits**, and what it **rules
out**. What it admits, it neither asks for nor forbids. A goal states a direction: what it asks
agents to move the project toward. It is drafted to move what the project builds, so a new or
reworded goal is expected to be unmet, and is never shaped so that the project as it stands meets
it.

%% `design@agent-skills@a-goal-is-a-purpose-the-project-exists-for`. Cases: of knowledge-architect's
%% 54 goals, 33 were goals of one Component, several of which stated how the Component fits in the
%% split, and 24 stated that they refined another goal, 10 of them the widest goal of the root. The
%% scope's condition was rule 8 of the standard before the kinds. Its case: rootpoint's goal on
%% content kinds, whose sentence on concurrent editing named the design it excluded, which another
%% goal would have argued for, and whose part on staying small named none.
1. **It is a purpose, a scope or a ranking of the project as a whole.**
   - A **purpose** states what the project exists to provide, and for whom. The test: were it
     removed, some part of what the project provides, or is to provide, would have no reason left
     to exist.
   - A **scope** bounds whom or what the purposes serve, and names the design it excludes, one that
     a purpose would otherwise argue for, described in words that keep their meaning under rule 2.
     A property of smallness with no named design is no goal: it is the owner's scope control,
     exercised by what the owner asks for, or a quality argument a design head makes against its
     own rival.
   - A **ranking** says which of two purposes comes first where they compete. It holds across
     decisions, never for one.

   Any other statement is design, whoever makes it: it is a decision, argued against its rivals,
   and what the owner brings to it lands as `primer@design-heads` says. A statement about one part
   of the project is design too, since the part exists by a design choice: a Component's purpose is
   the statement of the design head that argues its existence.
%% `design@agent-skills@goal-outcome-survives-empty-skeleton`. Cases: thaum's library goal stated
%% "the stream shape that meets this"; thaum's fuzzing goal stated "a lower-rate second arm" and two
%% mechanisms; knowledge-architect's goals named "the rules the core compiles in" and "an mdbook
%% build".
2. **It states an outcome the owner intends, which keeps its meaning in the project emptied of its
   design and its code.** Read the goal as if every design head, document and line of code under it
   were removed: each word still means what it meant. A sentence is a mechanism when, changed, the
   project would still be for what the owner intends, and an outcome when the change would make it
   for something else. A goal may define what the project provides
   and then name it, as "the project provides a command that checks its documents"; it names no
   part whose existence comes from a design choice, a Component included. A kind of interface, a
   command line, a library API or a check, is an outcome only where it passes the purpose test of
   rule 1, and the drafter asks the owner where the statements do not say. Its exact form, a
   command's name, a flag or a signature, is a decision: an interface others consume earns a design
   head. The argument that a design follows from a goal goes in that design's head, which cites it.
%% `design@agent-skills@goal-one-meaning-any-breadth`. Cases: rootpoint's "every view it shows", a
%% view of the presentation or anything the window shows; thaum's "learning agent", a term the
%% project reserved for an LLM playing through its protocol. Case of a question closed in a goal:
%% knowledge-architect's goal on a session that orchestrates its work, whose body ruled that a part
%% too small to spare the session's context is done by the session itself, a question of how much
%% argued against no rival.
3. **It states one direction.** Every term has one meaning in the project; a word the project
   already uses in another sense is defined or replaced. A goal reads two ways when two agents in
   the same situation, reading it, would move in different directions. A goal that admits many
   shapes in one direction reads one way: breadth is no defect. Neither is a question it leaves
   open of when, how much or by what means: a design head closes such a question, on its argument
   against a rival, and no wording of the goal does.
%% The rule allows the distillation goal-setting is for: a goal drawn from several statements of the
%% owner is wider than each of them. The defect it prevents is a reach the owner never saw. Cases:
%% rootpoint's "every feature the editor has", which ruled out presenter mode, saving and the text
%% protocol; "one human-readable text file", which ruled out assets beside it; "no language is
%% involved", which ruled out the text protocol.
4. **It requires nothing, and rules out nothing, the owner has not required or ruled out
   knowingly.** Where it generalises the owner's statements, what the generalisation requires and
   rules out is shown to the owner before the ruling. What it admits needs no ruling.
%% This is the head standard's rule on an exception, not the test "would reversing one part leave the
%% other standing", which splits a rule from its exception, per
%% `design@agent-skills@goal-standard-rules`. Case: rootpoint's sentence on concurrent editing,
%% inside its goal on content kinds.
5. **It holds one intent.** Two statements are two goals when each means something without the
   other; a qualifier or a domain belongs to the goal it qualifies. So a scope is a goal of its own
   only where it means something without any one purpose, and is otherwise a qualifier of the
   purpose it bounds. Two goals may share an aspect: a goal that touches what another states breaks
   no rule, and is a finding of rule 6 only where the two cannot both be met.
%% Cases: rootpoint's three goals whose excess reach conflicted with a sibling goal; thaum's
%% rewording of its fuzzing goal that dropped a qualifier and set it against its goal of rules
%% coverage before speed.
6. **It contradicts no other goal of the project.** No state of the project meets one goal only by
   failing another: a goal that rules out what another requires contradicts it. A goal that rules
   out what another only admits does not, and the exclusion holds. Where two purposes compete,
   including two that can both be met except under a condition the project or its use makes
   possible, a ranking states which comes first.
%% `design@agent-skills@what-shows-a-goal-met-is-design`. Cases: a rootpoint measure naming the list
%% of a milestone, which left when the milestone landed; knowledge-architect's goal on the quality
%% of the code the workflow leads to, whose measure listed three properties and bound every unit a
%% change edits to state why it departed from them; its goal on a session that orchestrates its
%% work, whose measure named the questions of a retrospective.
7. **It states no measure.** What would show the goal met, what the project builds to pursue it,
   and how the project learns from its use whether it is met, are design decisions, each in a head
   that cites the goal.
%% `design@agent-skills@no-goal-refines-another`. The citation rule is
%% `design@check@goals-home-cites-upward-only`, which the checker enforces.
8. **It refines no other goal, and cites only goals and registers of its own Component, or, in a
   location's goals home, of that location, or of a Component enclosing it.** A statement grounded
   on another goal is a design head, which cites that goal; a goal whose intent holds without the
   other stands alone. The citation rule holds for the home's head too, and the checker reports any
   other reference there. A goal holds what it needs in its own words.
