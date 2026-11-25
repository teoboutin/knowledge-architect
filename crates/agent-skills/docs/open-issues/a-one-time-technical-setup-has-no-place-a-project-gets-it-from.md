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
crate, per `design@knowledge-architect@scratch-removed-by-a-guard`, and a `disallowed-methods` row of
`path@knowledge-architect@clippy.toml` refuses `std::env::temp_dir`. The problem it answers, test runs that fill the temporary directory, was met
in this repository's sessions before the practice was found, and nothing in the installed text
names it.

The owner reported a hedged wish: that this project kind of delivers such generalisable solutions, so
that the owner's projects that use knowledge-architect inherit them. The owner reported also that no
place for such information is properly worked out right now; that the setup skill is maybe one, but
is not really where the owner expects it to be read when needed; and that an open issue could maybe
keep the question for now.

An installed skill serves one activity, per `design@agent-skills@one-skill-per-activity`. A setup done
once belongs to no change of code, the activity of the development skill; it is an act of the setup,
whose skill an agent reads at the adoption and at each move of the pin, where the problem the setup
answers is not met. So the setup skill stays open as a place, and what it lacks is the moment.

What exists, each of which carries one practice to a project at another moment. The recorded precedent
of a one-time setting is `design@agent-skills@harness-shell-fails-on-a-pipe`, which uses two of them:
the setup skill writes the setting at a setup, and a Migration entry tells existing projects to add
it. It is carried to a project whether or not that project has met the problem, which is what an
instance of this entry could do too; it does not answer how a session finds the practice when the
problem appears, since neither is read then.

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

A later instance: the work of `spec@plans@gates-on-windows-and-macos` carries two practices to a project, a gate that
runs clippy for another system and the alias `cargo x` in the release profile. A project meets the gate in
`path@gates@README.md`, in the documentation of `cross_clippy` and in the changelog, so it finds it where it
looks for a gate, and not when code for another system first fails; it meets the alias in the snippet that
`klarch init --rust` writes, in the setup skill and in a Migration entry. The work weighed a sentence on the
gate in the setup skill and wrote none, on `goal@only-needed-instructions`.

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
