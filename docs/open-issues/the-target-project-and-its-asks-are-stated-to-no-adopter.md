---
kind: todo
---
# The four conditions of a target project and what knowledge-architect asks of one are stated to no adopter

## Summary

`goal@knowledge-architect@serves-one-kind-of-project` defines a target project by four conditions
and lists what knowledge-architect asks of one: git with its `.git` at or above the project, agents
under the `claude` harness, every gate run through the gates library from one command, and one merge
invocation of that command in every target project. The setup skill states one ask, the
maintenance crate of every Rust target project, and no other; no published crate's README states the
conditions or the asks; and no design head names the command and its merge invocation.

## Details

### What

- The installed text states one ask, the maintenance crate of every Rust target project, per
  `design@agent-skills@maintenance-crate-ask`, in the setup skill, and none of the others; the goal
  requires it to state each one.
- The setup skill and the README of each published crate state neither the four conditions nor the
  other asks, nor that nothing is guaranteed outside the conditions or where an ask is not followed.
- The goal makes the name of the one command and its invocation for a merge a design decision of
  knowledge-architect. No head records it. The invocations differ today: this repository merges on
  `cargo x gates --require-rebased`, thaum on `cargo --locked x gates --merge-ready`, rootpoint on
  bare `cargo x gates`.

- Texts still offer what the asks rule out, or route a new ask otherwise:
  - the setup skill's template goals for the maintenance tool keep the wording this repository's own
    `goal@xtask@one-command-runs-every-gate` and `goal@xtask@repeated-tasks-are-automated` had before
    their rewording, though its comment says they follow those goals;
  - `init --vcs none`, per `design@core@init-runs-git-init`, writes a project that is no git
    repository, a state the git ask rules out unless it helps the project into conformance;
  - `primer@question-authority` sends the owner no new ask, which
    `goal@knowledge-architect@the-owner-decides` now sends whatever the argument settles.

### Why it matters

The goal is unmet while these are stated nowhere: an adopter cannot tell whether a project is served,
and the installed text, which `goal@agent-skills@installed-text-works-in-every-target-project` lets
rely on the asks only as the installed text states them, relies on none.

### What would close it

The installed text states each ask; the setup skill and every published crate's README state the
four conditions, the asks and the limit of the guarantee; a design head of knowledge-architect names
the command and its merge invocation, reached through `skill@knowledge-architect-design`; and each
text listed above is brought to the asks, a head among them through the same skill.
