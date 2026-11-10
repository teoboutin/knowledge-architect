%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`.
The project is partitioned into Components, which its manifest declares. The project's root is a
Component too.

%% Point 1 and the rule of one Component in point 4 are `design@agent-skills@dependency-decision-home`;
%% point 3 is
%% `design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists`.
1. Does the decision shape the layout of the project: which Components exist, how they depend on
   each other, constraining more than one of them, or how several of them are shaped or packaged? One Component depending on another constrains
   both: the second gains a consumer of its interface and cannot depend on the first. Then it
   belongs in the root Component's design home.
2. Does it shape how every Component is implemented, or every Component of a kind: a design
   principle each of them follows? Then it belongs in the root Component's design home. A kind is a
   property a Component has whatever part of the project it belongs to, and can gain or lose
   without moving, such as being published or holding a binary.
3. Does it shape how the code of each of several Components is written, and not every one: a design
   principle they follow, neither stated for a kind nor about their shape, packaging or layout? Then
   it belongs in the design home of the lowest intermediate Component enclosing them all, a
   Component whose own files hold no Rust code, or in the root Component's where none encloses them
   all.
4. Does it shape how a single Component is implemented, or its own interface, a rule that it does
   not depend on another Component included? Then it belongs in that Component's design home.

If it is a statement about what the project is _for_ rather than about how it is built, it is a
goal, not a decision. Goals are a register of their own, in the goals home of the project's root,
each entry a level-two heading carrying a slug. A decision that leans on one names it as
`goal@<root>@<slug>`. What one Component is for is a decision, the statement of the head that
argues the Component's existence, which point 1 places.

If none of these fits, **ask the owner before writing it anywhere.**
