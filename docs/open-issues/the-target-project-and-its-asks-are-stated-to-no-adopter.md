---
kind: todo
---
# The four conditions of a target project and what knowledge-architect asks of one are stated to no adopter

## Summary

`goal@knowledge-architect@serves-one-kind-of-project` defines a target project by four conditions,
says that knowledge-architect asks more of one, and names two asks: the project is a git
repository, and its agents run under the `claude` harness. Two further asks are stated by no goal
and argued by no head, which `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` holds:
every gate run through the gates library from one command, and one merge invocation of that command
in every target project. The setup skill states one ask, the
maintenance crate of every Rust target project, and no other; only the READMEs of `model`, `check`, `cli`, `query`, `transcript`, `create` and `testing` among the published crates state the
conditions and the asks; and no design head names the command and its merge invocation.

## Details

### What

- The installed text states one ask, the maintenance crate of every Rust target project, per
  `design@agent-skills@maintenance-crate-ask`, in the setup skill, and none of the others.
- The setup skill and the READMEs of the published crates other than `model`, `check`, `cli`, `query`, `transcript`, `create` and `testing` state neither the four conditions nor the
  other asks, nor that nothing is guaranteed outside the conditions or where an ask is not followed.
- The name of the one command and its invocation for a merge are design decisions of
  knowledge-architect. No head records the invocation for a merge. The invocations differ today: this repository merges on
  `cargo x gates --require-rebased`, thaum on `cargo --locked x gates --merge-ready`, rootpoint on
  bare `cargo x gates`.

- Texts still offer what the asks rule out, or route a new ask otherwise:
  - the setup skill's case of a project with no Rust, which installs the checker locally, serves no
    target project, since a target project's shipped code is Rust;
  - `init --vcs none`, per `design@create@init-runs-git-init`, writes a project that is no git
    repository, a state that does not follow the git ask and whose working the goal does not guarantee;
  - `primer@question-authority` sends the owner no new ask, which
    `goal@knowledge-architect@the-owner-decides` now sends whatever the argument settles.

### Why it matters

`goal@knowledge-architect@serves-one-kind-of-project` guarantees knowledge-architect's working only
as far as a project follows what it asks, and `goal@knowledge-architect@adoption-is-easy` has an
adopter find what to do in the documentation. While the conditions and the asks are stated nowhere
an adopter reads, an adopter cannot tell whether a project is served, nor which ask an installed
instruction relies on when it fails in a project that does not follow it.

### What would close it

The installed text states each ask; the setup skill and every published crate's README state the
four conditions, the asks and the limit of the guarantee; a design head of knowledge-architect names
the command and its merge invocation, reached through `skill@knowledge-architect-design`; and each
text listed above is brought to the asks, a head among them through the same skill.
