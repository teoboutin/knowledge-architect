%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`. Each rule's cases below were
%% read in the goal histories of rootpoint and thaum.
**A goal is proper when all of these hold.** The drafter writes to them, and the goal reviewer
judges by them.

%% Rule 1 reads goal-setting's outcome test against the owner's intent: an absolute list of things a
%% goal never names ruled out a kind of interface the owner intends to provide, such as a command
%% line, while an exact form, a command's name, is a decision. Cases: thaum's library goal stated "the
%% stream shape that meets this"; thaum's fuzzing goal stated "a lower-rate second arm" and two
%% mechanisms.
1. **It states an outcome the owner intends**: what the Component is for, and for whom. A sentence
   is a mechanism when, changed, the Component would still be for what the owner intends, and an
   outcome when the change would make it for something else. So the kind of interface the Component
   provides, a command line, a library API or a check, is an outcome where the owner intends to
   provide that kind, and the drafter asks the owner where the statements do not say. Its exact
   form, a command's name, a flag or a signature, is a decision: an interface others consume earns a
   design head. The argument that a design follows from a goal goes in that design's head, which
   cites it.
%% Cases: rootpoint's "every view it shows", a view of the presentation or anything the window
%% shows; thaum's "learning agent", a term the project reserved for an LLM playing through its
%% protocol.
2. **It has one reading.** Every term has one meaning in the project; a word the project already
   uses in another sense is defined or replaced. Every "every", "no", "all" and "not in scope" names
   the set it ranges over.
%% The rule allows the distillation goal-setting is for: a goal drawn from several statements of the
%% owner is wider than each of them. The defect it prevents is a generalisation the owner never saw
%% as one. Cases: rootpoint's "every feature the editor has", which ruled out presenter mode, saving
%% and the text protocol; "one human-readable text file", which ruled out assets beside it; "no
%% language is involved", which ruled out the text protocol.
3. **It rules out nothing the owner has not ruled out knowingly.** Where it generalises the owner's
   statements, the generalisation and the cases it rules out are shown to the owner before the
   ruling.
%% This is the head standard's rule on an exception, not the test "would reversing one part leave the
%% other standing", which splits a rule from its exception. Case: rootpoint's sentence on concurrent
%% editing, inside its goal on content kinds.
4. **It holds one intent.** Two statements are two goals when each means something without the
   other; a qualifier, a domain or a measure belongs to the goal it qualifies.
%% Cases: rootpoint's three goals whose excess reach conflicted with a sibling goal; thaum's
%% rewording of its fuzzing goal that dropped a qualifier and set it against its goal of rules
%% coverage before speed.
5. **It contradicts no other goal of the project.** No state of the project meets one goal only by
   failing another. Where two outcomes compete, a goal states which ranks first.
%% Case: a rootpoint measure naming the list of a milestone, which left when the milestone landed.
6. **What shows it met stays checkable**: an observable of the project as it stands, never a plan
   document or anything that leaves.
7. **A refinement neither widens nor contradicts the goal it refines.**
%% Case: rootpoint's goal on content kinds, whose sentence on concurrent editing named the design it
%% excluded, which another goal would have argued for, and whose part on staying small named none.
8. **A statement of what the Component is not for is a goal only when it names the design it
   excludes**, one that another goal would otherwise argue for. A property of smallness with no
   named design is no goal: it is the owner's scope control, exercised by what the owner asks for,
   or a quality argument a design head makes against its own rival.
