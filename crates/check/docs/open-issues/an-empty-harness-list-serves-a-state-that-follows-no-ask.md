---
kind: question
---
# An empty list of agent harnesses serves a project that uses the checker alone, which no longer follows an ask

## Summary

`design@check@agents-table` keeps an empty `[agents] harness` list, which declares no agent
configuration, and argues it as serving "a target project that uses the checker alone, which that
goal admits", with the checker's mock projects. The owner's ruling in the review of the whole goal
set made using both the workflow and the checker an ask of
`goal@knowledge-architect@serves-one-kind-of-project`. A project that uses the checker alone is then
a target project that does not follow an ask, and that goal builds nothing only to serve such a
state, other than what helps a project into conformance. The head's argument no longer stands as
written; whether the empty list stays, on the mock projects alone or as a step into conformance, is
open.

## Details

### What

The head's sentence "The empty list serves a target project that uses the checker alone, which that
goal admits" contradicts the goal as the owner ruled it. The empty list has two users: a project
that uses the checker alone, and the checker's mock projects, every one of which but `unsound`
declares `harness = []`, per `path@testing@CLAUDE.md`. Its other references are listed by
`cargo klarch show design@check@agents-table`.

Two other texts argue from the same premise, a target project that uses the checker alone and
declares no harness:

- `path@model@docs/design/anchors.md`, whose sentence on the flaw that the empty list leaves names "a
  target project that uses the checker alone and declares no harness";
- `issue@check@a-home-for-developer-contracts-outside-agent-configuration`, whose "Why it matters"
  says "a target project that uses the checker alone declares that empty list all the same".

### Why it matters

A design head that argues from a reach the goal no longer gives binds sessions on a false premise,
per `primer@intent-and-claims`: the goal prevails, and the head is either reversed or re-argued. It
strains `design@check@agents-table` and threatens
`goal@knowledge-architect@serves-one-kind-of-project`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that settles one of: the empty list
kept for the mock projects and as a step into conformance, the head re-argued on that ground; or
the list made non-empty in every manifest, the mocks given another way to skip the installed
text's checks, and the head reversed. The session put it outside the goal-standard branch, and the
owner approved.
