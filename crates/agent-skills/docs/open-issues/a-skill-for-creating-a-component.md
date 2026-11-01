---
kind: todo
---
# No installed skill says how a new Component is created

## Summary

The setup skill creates the Components a project starts with. A Component added later, a new
crate or a new service, owes the same documents and at least one goal, and no installed skill says
what creating one involves beyond the documents the check reports missing.

## Details

### What

A skill for creating a Component: when a directory earns being one, what it carries from its first
commit, how its goals are stated, and which decisions of the root Component it inherits.
`goal@knowledge-architect@projects-are-split-into-components` states the four properties a Component
that holds Rust code is led toward; how a Component maps to Cargo packages, and how the installed
text leads a session there, are left to a design discussion the owner asked for.

### Why it matters

A Component created without its goals or its design home's introduction passes the check once its
documents exist, empty, as `klarch new-component` writes them per
`design@core@component-command`, and a goal that is its responsibility is then stated nowhere, against
`design@agent-skills@goals-required`.

### What would close it

The skill, designed with the owner and installed, naming `new-component` as its first act, and the
setup skill pointing at it for a Component added after setup.
