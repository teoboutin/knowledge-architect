---
kind: design
---
# No head argues what gate behaviour a project may keep in its own code, and when that copy goes

## Summary

A rule on a project's own copy of a behaviour of a gate command bound the project while a goal of
this Component held it: the project may write the behaviour itself while no released version ships
it, and the copy goes when the project moves to a version that does. No design head states the
rule, and nobody has argued it against a rival.

## Details

### What

**The instances.** The statements, as the Component's goal held them:

1. "A behaviour of a gate command that no released version of knowledge-architect ships may be
   written in the project's own code, and stay there until the project's own move to a released
   version that ships it: the copy goes at that move." It bound a target project: which gate logic
   it may hold beside its list, and until when.
2. "In knowledge-architect's own repository, which runs the library it builds, a behaviour is
   shipped once it merges into the main branch, and the copy goes at that merge." It bound this
   repository's `xtask`: a behaviour the library gains leaves `path@xtask@src/gates.rs` in the same
   merge.
3. "Admits, for instance: a project that keeps in its own code a behaviour of a gate command until
   its own move to a released version of knowledge-architect that ships it." It bound nothing
   beyond statement 1.
4. "Admits, for instance: a project that writes in its own code a behaviour knowledge-architect's
   owner has taken in, before a released version ships it." It bound nothing beyond statement 1.
5. "Rules out, for instance: a project that keeps its own copy of a behaviour after its move to a
   released version that ships it." It bound a target project after a move of its pin.

The instance in a project: thaum keeps its own runner, whose `--merge-ready` flag the library does
not hold, per `issue@gates@the-library-has-no-merge-verdict`.

A second wording of the rule existed, for every tool and procedure and not for the gates alone: "A
project's own copy of a part stays until a released version of knowledge-architect ships the part."
The two differ on the moment the copy goes: the release that ships the part, or the project's own
move to it.

**Suspected mechanism**: the rule was written as a goal, which bound outright, so no argument
against its rival was written.

**What was searched**: the setup skill's source and the primer's source for "own copy", "its copy"
and "ships the part" return nothing, so no installed text delivers the rule to a target project.
`design@gates@a-project-holds-its-gate-list` says a project "writes no gate logic beyond its list
and the call that runs it", with no exception for a behaviour the library does not ship yet.

**The re-entry point**: the discussion that builds the merge mode of
`issue@gates@the-library-has-no-merge-verdict`, at which thaum's own runner either moves to the
library or stays; or the first change that puts a gate behaviour in `path@xtask@src/` beside the
list.

### Why it matters

`design@gates@a-project-holds-its-gate-list` reads as refusing every gate logic of a project's own,
and thaum holds some today. A session that reviews such a copy, or that writes one in this
repository's `xtask`, finds no rule on whether it may exist and when it must go, and no argument
for either answer.

### What would close it

A discussion under `skill@knowledge-architect-design` argues the rule against its nearest rival, a
project that must drop every copy at its move and may write none before, and names the moment the
copy goes, in a target project and in this repository. A rule that stands on a fact enters
`design@gates@a-project-holds-its-gate-list` as its exception, or a head of its own where it loses
to a rival of its own; one that does not leaves, and that head stands as written.
