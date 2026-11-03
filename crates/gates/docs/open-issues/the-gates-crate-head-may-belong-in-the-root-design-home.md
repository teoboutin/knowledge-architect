---
kind: question
---
# Does the head deciding that the gates runner is a crate of its own belong in the root's design home?

## Summary

`design@gates@gates-crate` sits in the gates Component's design home. Part of it decides which
Components exist: the runner is a published library of its own, and a `gates` command of the
checker's binary lost. Point 1 of `skill@knowledge-architect-decision-recording@owning-component`
sends a decision on which Components exist to the root's design home, before the revision of the
rules on Components and after it.

## Details

### What

The head states two things: the runner is the package knowledge-architect-gates, a published
library that a project's maintenance crate calls; and how the library is used, a project's binary
handing it the root, the gate list and the flags. The first answers which Components exist, and its
rival, a command of the checker's binary, is a layout of the project. The second is the gates
Component's own interface.

The root's design home already holds the decisions of this kind for other Components:
`design@knowledge-architect@git-crate` and `design@knowledge-architect@mutation-command-host`, each a
crate of its own and the crates that depend on it.

Not established: whether the head is one decision or two, by
`design@agent-skills@one-decision-per-head`, and so whether the move takes the whole head or the
first part only.

### Why it matters

A session that reads the root's design home for the layout of the project does not find the gates
runner's place in it, against `design@agent-skills@dependency-decision-home`, which keeps the
decisions on which Components exist and how they depend on each other in one place.

### What would close it

The head judged under the rule on the owning Component: the part that decides which Components
exist moved to the root's design home, its references re-pointed, or the head kept where it is with
the reason, in a design-record audit or the next change that touches it.
