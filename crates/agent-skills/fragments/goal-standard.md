%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`. Each rule's cases below were
%% read in the goal histories of rootpoint, thaum and knowledge-architect; the rules as a whole are
%% argued in `design@agent-skills@goal-standard-rules`.
**A goal is proper when all of these hold.** The drafter writes to them, and the goal reviewer
judges by them.

%% The three layers keep an admitted case from being read as a requirement, which made a goal that
%% admits what another rules out read as a contradiction, and a reach finding read as a defect.
**A goal is read in three layers**: what it **requires**, what it **admits**, and what it **rules
out**. What it admits, it neither asks for nor forbids. A goal states a direction: what it asks
agents to move the project toward. It is drafted to move what the project builds, so a new or
reworded goal is expected to be unmet, and is never shaped so that the project as it stands meets
it.

%% Rule 1's skeleton test: a goal that names a part as built binds the part, so a redesign rewrites
%% goals only the owner may change. Rule 1 also reads goal-setting's outcome test against the
%% owner's intent: a kind of interface the owner intends to provide is an outcome, an exact form a
%% decision. Cases: thaum's library goal stated "the stream shape that meets this"; thaum's fuzzing
%% goal stated "a lower-rate second arm" and two mechanisms; knowledge-architect's goals named "the
%% rules the core compiles in" and "an mdbook build".
1. **It states an outcome the owner intends, which keeps its meaning in the project emptied of its
   design and its code.** Read the goal as if every design head, document and line of code under it
   were removed: each word still means what it meant. A goal may define what the project provides
   and then name it, as "the project provides a command that checks its documents"; it names no
   part whose existence comes from a design choice. A Component's goal may name its own Component
   and the Components that enclose it. The kind of interface the Component provides, a command line,
   a library API or a check, is an outcome where the owner intends to provide that kind, and the
   drafter asks the owner where the statements do not say. Its exact form, a command's name, a flag
   or a signature, is a decision: an interface others consume earns a design head. The argument
   that a design follows from a goal goes in that design's head, which cites it.
%% The rule judges ambiguity, not breadth: asking every word to name its set made each repair add a
%% scope, and the goals narrowed round after round. Cases: rootpoint's "every view it shows", a view
%% of the presentation or anything the window shows; thaum's "learning agent", a term the project
%% reserved for an LLM playing through its protocol.
2. **It has one reading.** Every term has one meaning in the project; a word the project already
   uses in another sense is defined or replaced. A goal reads two ways when two agents in the same
   situation, reading it, would move in different directions. A goal that admits many shapes in one
   direction reads one way: breadth is no defect. Every "every", "no", "all" and "not in scope" in
   what a goal requires or rules out names the set it ranges over.
%% The rule allows the distillation goal-setting is for: a goal drawn from several statements of the
%% owner is wider than each of them. The defect it prevents is a reach the owner never saw. Cases:
%% rootpoint's "every feature the editor has", which ruled out presenter mode, saving and the text
%% protocol; "one human-readable text file", which ruled out assets beside it; "no language is
%% involved", which ruled out the text protocol.
3. **It requires nothing, and rules out nothing, the owner has not required or ruled out
   knowingly.** Where it generalises the owner's statements, what the generalisation requires and
   rules out is shown to the owner before the ruling. What it admits needs no ruling.
%% This is the head standard's rule on an exception, not the test "would reversing one part leave the
%% other standing", which splits a rule from its exception. Case: rootpoint's sentence on concurrent
%% editing, inside its goal on content kinds.
4. **It holds one intent.** Two statements are two goals when each means something without the
   other; a qualifier, a domain or a measure belongs to the goal it qualifies. Two goals may share
   an aspect: a goal that touches what another states breaks no rule, and is a finding of rule 5
   only where the two cannot both be met.
%% Cases: rootpoint's three goals whose excess reach conflicted with a sibling goal; thaum's
%% rewording of its fuzzing goal that dropped a qualifier and set it against its goal of rules
%% coverage before speed.
5. **It contradicts no other goal of the project.** No state of the project meets one goal only by
   failing another: a goal that rules out what another requires contradicts it. A goal that rules
   out what another only admits does not, and the exclusion holds. Where two outcomes compete,
   including two that can both be met except under a condition the project or its use makes
   possible, a goal states which ranks first.
%% Case: a rootpoint measure naming the list of a milestone, which left when the milestone landed.
%% The measure is what the project builds, since a measure on use is out of reach in most projects,
%% where the project is not used on itself; evidence from use keeps the role of showing the goal
%% unmet in practice. The test of what the project does not control is KAOS's sort by who can
%% enforce a goal.
6. **What shows it met is in what the project builds**, never a plan document or anything that
   leaves. **Ask whether the outcome depends on anything the project does not control, such as its
   users, their data or their environment.** If not, what shows it met is a check or a test over
   what the project builds or produces. If so, the outcome is judged only in use, and what shows it
   met is what the project builds to pursue it and to learn from its use whether it is met, such as
   a test of a tool under the conditions of its use, a route by which its users' reports reach the
   project, or, for instructions to agents, a review that weighs them and a retrospective that asks
   how they fared. A list of the properties the outcome is taken to consist of is no such
   observable, since it narrows the outcome to what was foreseen before any use. Evidence from use,
   such as a review's finding, a retrospective's finding or what the owner observes in sessions, is
   not the measure: it shows the goal unmet in practice where what is built fails it, and leads to a
   change of what is built.
%% The citation rule is `design@core@goals-home-cites-upward-only`, which the checker enforces: a goal
%% tied to an entry agents move, or to a lower Component that can be split or replaced, changes when
%% they do.
7. **A refinement neither widens nor contradicts the goal it refines, and a goal cites only goals
   of its own Component or of one enclosing it.** A goals home, its goals and its head alike, cites
   no entry other than a goal, and no register's home other than one of its own Component or of one
   enclosing it; the checker reports any other reference there. A goal holds what it needs in its
   own words.
%% Case: rootpoint's goal on content kinds, whose sentence on concurrent editing named the design it
%% excluded, which another goal would have argued for, and whose part on staying small named none.
8. **A statement of what the Component is not for is a goal only when it names the design it
   excludes**, one that another goal would otherwise argue for, described in words that keep their
   meaning under rule 1. A property of smallness with no named design is no goal: it is the owner's
   scope control, exercised by what the owner asks for, or a quality argument a design head makes
   against its own rival.
