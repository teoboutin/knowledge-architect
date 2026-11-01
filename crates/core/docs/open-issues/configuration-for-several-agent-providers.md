---
kind: deferred
---
# The agent configuration serves one provider's harness

## Summary

`[agents] harness` declares the agent harnesses a project serves, and the checker knows one,
`claude`: it requires a CLAUDE.md in every component and installs into the `.claude/` layout.
Projects often keep their agent instructions in a generic AGENTS.md, with CLAUDE.md, GEMINI.md and
others as symlinks to it or as files importing it. The checker has no harness for either.

## Details

### What

Harness values for other providers, each requiring its own file and installing into its own
layout, and AGENTS.md as the generic file; and, for `klarch transcript`, a reader of each other
harness's session transcripts. The mechanism exists; what each value requires is open.

### Why it matters

It extends `design@core@agents-table`, whose list holds one value, and a second harness's
transcripts strain `design@core@transcript-reads-claude-code-alone`.
`goal@knowledge-architect@serves-one-kind-of-project` asks every target project to run its agents
under `claude`, and excludes a mode of the checker for another harness, so no target project needs
this work while that ask stands. The entry keeps what a second harness would need.

### Trigger

The owner changes the ask of `goal@knowledge-architect@serves-one-kind-of-project` that every target
project's agents run under `claude`: that change is already deciding which harnesses
knowledge-architect serves.
