---
kind: question
---
# May the checker depend on every crate that does not depend on it, `query` and `render` among them?

## Summary

The owner's view, stated at the slice that renamed the core and left for a later session, is that `check` should be allowed to
depend on all or nearly all of the other crates, every crate but one that depends on it, such as `cli`: it reports where a
project breaks a rule, on how citations are done for example, and it may need `query` for that. It could also report that an
item cannot be rendered, or that rendering requires a further constraint on the model, better implemented from `render`. The
owner holds this last part as a lead only, and names a gate that runs `render` as a better rival there.

## Details

### What

Today `check` depends on `git`, `model` and `agent-skills`. `milestone@plans@component-split` plans `query` and `render` beside it, each
over `model`, and plans `create` and `cli` above it. `design@model@no-check-below-the-model` and acceptance criterion
#a1-no-check-below-the-tools of that milestone forbid the opposite direction alone, a tool below the checks depending on them, so the
owner's view contradicts neither. No recorded decision states which of these crates `check` may depend on. The crate that this would
exclude besides `cli` is `create`, which the milestone has depend on `check`.

### Why it matters

A lint that needs what `query` computes has no recorded answer on where it may get it, and a session could copy the computation into `check`
or read the absence of a dependency as a rule. The direction serves `goal@check@document-rules-are-checked`. It costs each consumer of
the checker the build of whatever it pulls in, mdbook's dependency tree if `render` holds it, which
`spec@component-split@integrated-tests` and the extension API's consumers meet.

### What would close it

The first lint or the first plan that wants a crate `check` does not yet depend on, or a design discussion the owner opens: a decision in
the root's design home, per `skill@knowledge-architect-decision-recording@owning-component`, that states which crates `check` may depend on,
with the render case judged against a gate that runs `render`.
