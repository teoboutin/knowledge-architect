---
kind: deferred
---
# A project cannot use an unreleased change to a shipped tool through a Cargo patch

## Summary

A target project that needs a change to a tool knowledge-architect ships before the next release has
no installed way to build against a branch of knowledge-architect, such as a `[patch.crates-io]`
naming that branch. It builds its improvement as a tool of its own meanwhile, per
`design@agent-skills@upstream-tool-loop`.

## Details

### What

A patched build of the checker carries the version of the branch it was built from. Every binary
refuses a project whose pin it does not satisfy, per `design@agent-skills@exact-pin`, and accepts a
binary only when its version equals the core library's, per
`design@core@installed-binary-version-check`: so a patched build runs only where the project's
`checker-version` names the branch's version, which no release of it may hold yet. Whether the
sentinel or a pre-release version would serve is not established.

### Why it matters

A consumer waiting for a release keeps a tool of its own that duplicates the shipped one, and the
improvement reaches the shipped tool only when the next release lands.

### Trigger

A consumer needs a change to a shipped tool before the next release, and building it as a tool of
its own is not enough: that need is the work of designing the co-development path.
