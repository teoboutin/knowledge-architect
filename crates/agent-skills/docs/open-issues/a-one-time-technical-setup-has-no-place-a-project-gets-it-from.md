---
kind: design
---
# A one-time technical setup that every target project would gain from has no worked-out place from which a project gets it

## Summary

A technical practice can be set up once in a project, such as a development dependency with a lint
that requires it, and would serve every target project. Nothing works out where a project gets one:
where an agent meets it at the moment its problem appears, and which practices are offered and which
are asked.

## Details

### What

The instance: this repository's tests hold their scratch directories in guards of the `tempfile`
crate, and a `disallowed-methods` row of `path@knowledge-architect@clippy.toml` refuses
`std::env::temp_dir`. The problem it answers, test runs that fill the temporary directory, was met
in this repository's sessions before the practice was found, and nothing in the installed text
names it.

The owner wants such solutions inherited by the owner's projects that use knowledge-architect, and
reported that the development skill does not fit a setup done once, and that the setup skill is not
where it would be read when it is needed.

What exists, each of which carries one practice to a project at another moment:

- the section `skill@knowledge-architect-setup@rust-project`, which asks lints, the documentation
  ones per `design@agent-skills@documentation-presence-is-linted`, and is read at a setup;
- the files `klarch init --rust` writes, per `design@create@rust-scaffold`, into a project
  with no `Cargo.toml` alone;
- the Migration entries of the changelog, per `design@knowledge-architect@changelog-entries`, read
  when a pin moves, for what a project must change.

What is missing: the place where an agent meets a practice when the problem it answers appears; and
which practices are offered and which asked, for which the rule of what becomes an ask is
`issue@knowledge-architect@no-head-argues-what-becomes-an-ask`. A lead:
`design@agent-skills@the-workflow-teaches-no-rust-idiom` keeps instructions on idiom out of the
installed text, and admits a mechanical gate on it once knowledge-architect's owner takes it in.

Suspected mechanism: the places that carry a practice are tied to an activity, a setup, a pin move or
a change of code, and the problem a one-time setup answers is met during any of them.

### Why it matters

`goal@generic-tools-and-procedures-are-shipped` ships the generic part of a tool once
knowledge-architect's owner takes it in. With no place to ship it from, each project meets the same
problem and solves it on its own, or not at all. The place may be an installed instruction, which
`goal@only-needed-instructions` weighs.

### What would close it

A design discussion under `skill@knowledge-architect-design` that argues the place, or argues that
no place is owed, and the first instance carried through what it decides.

### Re-entry point

The discussion of the next practice of this kind, a one-time technical setup that a session or the
owner proposes for more than one project.
