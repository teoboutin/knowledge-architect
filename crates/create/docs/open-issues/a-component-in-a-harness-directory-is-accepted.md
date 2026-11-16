---
kind: observation
---
# A Component in the directory the harness reads agents or skills from is accepted

## Summary

`new-component .claude/agents` creates a Component there, and `check` then passes, so the
Component's `README.md` and `CLAUDE.md` sit in the directory the `claude` harness reads agent
definitions from. Whether the harness then reads them as agents, and whether that is a defect, is
not established.

## Details

### What

Seen by the adversarial review of the branch that closed the mutation survivors of three suites, on
a project made with `init`: `new-component` refuses a DIR at or under an installed file, by its
overlap list in `refuse_overlaps` of `path@create@src/command.rs`, as in
`new-component .claude/knowledge-architect`, `new-component .claude/skills/knowledge-architect-review`
and `new-component .claude/agents/knowledge-architect-goal-reviewer.md/sub`. It accepts
`new-component .claude/agents`, and the same for the directories of skills and of saved workflows,
for `.claude` itself and for a new directory under the workflows' one, after which `check` passes.

Not established:

- whether the harness loads the Component's README or its CLAUDE.md, beside the installed agent
  definitions, as an agent definition, or ignores a file with no frontmatter;
- whether `design@model@unusable-harness-name-is-a-finding` should report such a file, and why the
  check passes over it.

It reproduces, with the `klarch` built from this repository on the owner's machine, in a scratch
directory: `klarch init p`, then, in `p`, `klarch new-component .claude/agents`, `git add -A` and
`klarch check`, which prints `PASSED: no findings`.

### Why it matters

A Component's documents read as an agent definition would put a project's README in the list of
agents a session can dispatch. `design@create@component-manifest-edit` parses the edited manifest
before writing it, and the overlap list refuses installed files, but nothing refuses a directory the
harness reads whole.

### What would close it

A run of the harness over a project with a Component at the agents' directory, which shows whether its
files are listed as agents. Where they are, a refusal of such a DIR in `new-component` and `init`,
with a test; where they are not, this entry is deleted with the observation.
