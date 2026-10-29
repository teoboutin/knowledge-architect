%% The split of the rules on heads by act: `design@agent-skills@one-home-for-head-rules`.
**The deliberation is not copied into the head.** Where it is kept, and how a reader finds it,
is `skill@knowledge-architect-decision-recording@three-homes`.

A head is written **as if the design had always been so**. Present tense, no dates, no "formerly", no
account of the change. If you find yourself writing "we used to…", that sentence belongs in the
commit, and so does an opening that motivates a decision by describing the state before it.

Never list what cites a head: `{{command}} show design@<component>@<slug>` computes it.

%% Until a reference can name an entry of another project, `issue@core@cross-project-references`.
A decision that relies on the checker of this workflow states that it relies on the checker
working as intended. It cannot reference the checker's own decisions: a reference resolves only
against the project that holds it.

Give the decision a **slug anchor**: a short hyphenated name in backticks, prefixed with `##`. It
goes at the **end of a level-three heading**, the level the design register declares, so that the
outline reads as decisions under level-two subjects. **Every level-three heading in a design home
is an entry** and carries a slug: one without is a finding, so a heading that is section text sits
at level two or four. Nowhere else: a slug at another heading level, in a table cell, at the head
of a plain line, in the middle of a line or in a file that is not the design home defines nothing.
Outside a fenced block, `{{command}} check` reports it as a misplaced definition; a reference to it
resolves to nothing.
The slug is an id, unique in the design home.

**When the decision was a thread of a design discussion, its slug is the thread's name**, unless
that name misdescribes the decision as approved, as a name for the change it proposed does. The
discussion minted it in the same grammar and checked it for a collision with the Component's
entries before using it, for that reason. When the name misdescribes the decision, the entry takes
a slug that names the decision, by the alignment rule below, and the text that keeps the
deliberation states the pair, `#<thread> → <entry slug>`: the plan document's harvest row, or the
commit message on the in-change path. `git log --grep` on either name then finds a commit message
that states the pair, and `git log -G` a plan document's diff that does.

**A list item is not a definition site either**, so a decision written as one bullet among several
carries no anchor and cannot be cited or found by `git log -G`. This is a constraint on the
document rather than a gap in the checker: **a decision worth a slug is worth its own level-three
heading**. When a section of bulleted arguments produces one, break it out of the list.

The statement comes first and the slug last, with no bold and no em dash between them, so that a
document outline reads as a list of decisions rather than a list of identifiers. The body follows as
ordinary prose.

**A title states a decision only while it is false of the nearest rival it beat.** A title that the
losing alternative would make true names a subject, not a decision: "The configuration is read
once, at start-up" is false of a configuration read again on every request, and "The configuration
is read with care" is true of nearly any rival.

**A title states the rule that decided, not the list of what it admits today.** When a decision
admits members (fixes, kinds, verbs, sections, consumers), or picks a mechanism to meet a
requirement, the title states the property that admits a member, in the argument's own terms, and
the body names the members built as what the rule admits today. A count or a list in a title goes
stale at the next member, and every later member then reads as a change of the decision. A set
closed on purpose keeps its list in the title, with the sentence that argues the closure in the
body. A title states no more than its argument argues.

**A head holds one decision.** Two statements are one decision when they answer one question and
lose to the same nearest rival. A rule's exception, its parameter and its delivery belong to its
head, since none means anything without the rule, unless one loses to a nearest rival of its own. Two statements that lose to different nearest
rivals, or whose arguments share no premise, are two decisions. Each takes a head of its own when
it passes an entry test, and a comment at its code when it passes none. A title joined by "and"
over two decisions shows a bundle. Rewording the title to state both does not make them one. A
member of a set the title's rule admits is no decision, so a member missing from a title is no
finding.

**A head written from a decision the owner approved is read against that approval before it is
written.** The title and the body state the position as the owner saw it, in a plan document's
thread or in the discussion of the in-change path. The approval is read by its direction: a clause
that relaxes it, drops part of it or turns it can still be false of the nearest rival, and is still
not what was approved: it goes to the owner, as a change of the decision. A refinement within its
direction, stricter, more detailed or reworded, is the head's to state.

**The slug and the title stay aligned with the full scope of the decision**: the slug is often the
only part a reader sees, in a citing document or in code, and the title is what a document outline
shows. A slug or a title that misdescribes its decision misinforms every reader, or undermines the
decision it names, so rename it even when that means rewriting every reference in the project.

**A head that a change touches is brought to the rules on design heads in that change**: its title
to the rule it argues, each decision it bundles to a head of its own, its ground to its argument. Re-pointing a
reference in a head, to a renamed entry or a migrated form, does not touch it; adding or removing
one does. Where bringing it
to the rule would relax or turn what the owner's words in it approved, as a title moved from the
members the owner named to a rule that admits others, the change goes to the owner as one proposal,
as a new member does.
