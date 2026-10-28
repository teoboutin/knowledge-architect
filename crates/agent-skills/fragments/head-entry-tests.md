%% The split of the rules on heads by act: `design@agent-skills@one-home-for-head-rules`.
Most implementation choices earn no head. A unit of work produces dozens of them, and a design home that
records dozens per unit of work stops being readable and stops being ranked.

**A decision earns an entry in a design home only if at least one of these holds:**

1. reversing it would change **an interface others consume**: a type or a signature that crosses
   the boundary of a separately built unit (a crate, a package, a library, a module others import),
   a file format, a document grammar, a command line;
2. **the same reason must be respected at more than one site, or at none.** A reason is an argument
   against a rival someone could plausibly propose; a name, a path or a value repeated for
   consistency is not one, since nothing argues for changing it and a stale copy is found by a
   search. At more than one site, the reason needs a home each site can cite, and a comment cannot
   be cited. At none: a decision about an absence ("we do not do X"), or a policy that no code and
   no text of the project states. For a decision about agent-facing text, a skill, an agent or a
   `CLAUDE.md`, each text that states the instruction is one site, a design head of the project
   that restates it included, other than the decision's own: a decision stated by one text
   has one site, and its reason lives in that text, beside the instruction; or
%% In this repository, the place beside an instruction of the installed text is a `%%` line:
%% `design@agent-skills@shipped-text-line-comments`.
3. **its argument turns on the behaviour of something outside the project**: an external
   specification the project implements (a standard, a protocol, a rule set), or an external tool's
   behaviour, read in its documentation or measured; or
%% The `Q` label: `design@agent-skills@ruled-items-labelled`.
4. **the owner confirms that it records the owner's own intent**: a ruling the owner gave that an
   agent could judge superfluous and reverse as a small fix, or an argument the owner made and wants
   kept so as not to restate it. The agent does not judge this. When a decision would earn a head
   by this test alone, ask the owner, one question per decision, each under a label, `Q<n>`,
   numbered as `primer@labelled-items` says, several in one message, each quoting the owner's words
   the decision rests on: do they record the owner's intent, or were they an answer to the agent's
   proposal, or a hedged statement ("I think", "maybe"), which is a position to argue under
   `skill@knowledge-architect-design`? The head attributes the decision to the owner, with the
   owner's answer, as `primer@design-heads` says of attribution.
   With no answer, the decision earns no head by this test.

A choice that turns on what the specification means, or on how the tool behaves, is expensive to
get wrong and expensive to derive again, and it is visible: the argument quotes the specification
or the documentation, or names the measurement.

Otherwise it belongs in an **inline comment at the code it explains, plus the commit message**.
The comment is read by every session that touches the code, which a head is only where the session
finds it on the code's subject, and the commit
carries the argument. A reason that fits in one comment at one site, about that site's own code,
fails test 2.

%% The purpose of the project is its goals home's, per the knowledge table of the primer; the
%% former sentence, "What the project is for … belongs here", sent the owner's statement of a goal
%% into a head in the session that built `klarch init`.
**A head carries the intent it serves, its shape and the standing argument, not implementation.**
How the project is arranged to reach its goals belongs here, each head citing the goal it derives a
constraint from; what the project is for is its goals home's. How a particular function does its
work belongs in a comment at that function. The test: **if changing a piece of code would force a
change to the design home, it is design; if the design home would be unaffected, it is a comment.**

**The backstop. A decision that creates a head, contradicts a statement of one, its argument
included, or adds a member the head's argument does not cover, goes to the design skill before its
text is written**, unless it was argued there, under `skill@knowledge-architect-design`. This is the case of
a decision met during another task and settled there, by the owner's word or by the session's own
choice. The design skill's in-change path keeps the deliberation in the commit message, so the task
needs no plan document and no new session. An addition that answers another question than the
head's takes a head of its own, by the test of one decision per head of
`skill@knowledge-architect-decision-recording@head-shape`. A member the head's
argument covers is recorded directly, routed as the next paragraph says; where the owner approved
it, the approval is recorded in the commit, attributed to the owner, not in the head. A change that relocates or rewords
recorded decisions, a split of a head included, and adds or removes none, is not a decision: it
needs no design skill, and the routing and fidelity-of-relocation review axes judge that it adds or
removes none.

**A new member of a head is routed by the head's ground.** A member is what a head's rule admits
and its body does not yet name. Where the part of the head that admits it stands on its argument,
and the argument covers the member, it is recorded directly. Where that part's ground is the
owner's words and they state the rule, the member is within the ruling and is recorded directly,
the ground attributed to the owner as before. Where the owner's words name the members, it goes to the owner as one
proposal: one message stating the rule, the member and a default, and nothing of it is written
before the owner's word; it is no change of the decision, so it owes no reversal. A member the rule
does not admit, or that the argument does not cover, is a change of the decision, and goes to the
design skill, as the backstop says.
