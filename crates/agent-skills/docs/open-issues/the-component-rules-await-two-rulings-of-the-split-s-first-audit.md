---
kind: todo
---
# The rules for splitting a project into Components still state two points the owner ruled otherwise at the Component split's first audit

## Summary

At the design audit of the slice that created the git Component, the owner ruled two points of the
installed rules for Components: an intermediate Component exists where several Components share design the
rest of the project does not, never for a count of Components; and the dependencies that span
several Components are the root design home's, while a rule that one crate depends on no other may
be that crate's. The installed text still states both points as before.

## Details

### What

The rulings are the threads #intermediate-by-shared-design and #dependency-layout-home of
`milestone@plans@component-split`, which keep the owner's words. What the installed text states
against them:

- `skill@knowledge-architect-component-creation@split-signals` says "Where many Components would
  sit beside one another, nested Components under an intermediate Component carry the layering".
  The owner ruled it removed, and the intermediate Component stated as: several Components that
  share design not true of the whole project are grouped under an intermediate Component, which
  holds the shared design and no code of its own; either the intermediate Component is no crate and
  each nested Component is one, which the owner recommends for isolation, or, where the cost of the
  crate count is too high, the intermediate Component is a crate and the nested Components are
  modules of it.
- `skill@knowledge-architect-component-creation@component-criteria` says "Which Components exist
  and how they depend on each other is a decision about the layout, which belongs in the root's
  design home", and point 1 of `skill@knowledge-architect-decision-recording@owning-component` says
  the same. The owner ruled that this holds of the dependencies that span several Components, and
  that a decision that one crate does not depend on another may belong to that crate.

Not established: how the second shape of the first ruling, Components as modules of an
intermediate crate for the cost of the crate count, sits beside
`design@agent-skills@component-crate-mapping`, whose exception admits a module directory only where
a crate would force a publication or a cycle Cargo's rules force, and beside
`goal@knowledge-architect@projects-are-split-into-components`, whose intermediate Component's own
files hold no Rust code.

### Why it matters

A session that applies the installed text acts against the owner's rulings: it groups Components by
their count, and it moves a crate's own rule against a dependency into the root's design home. It
strains `design@agent-skills@component-crate-mapping` and serves
`goal@knowledge-architect@projects-are-split-into-components`.

### What would close it

The installed text revised to the two rulings, the question above on the second shape settled
through `skill@knowledge-architect-design`, with the owner where it is a fork, and the heads of
`register@agent-skills@design` that the revision touches rewritten in the same change.
