---
kind: defect
---
# A shipped agent's frontmatter `name` is not checked against its file

## Summary

`design@core@harness-entity-names` makes a frontmatter `name` that differs from an agent's file a
finding, since the harness identifies an agent by that field. The finding is raised for a project's
own agents. An installed copy raises no finding of its own, per `design@core@owned-namespace-check`,
and no test of the shipped set compares the field with the file, so a shipped agent named wrongly
installs and passes.

## Details

### What

Measured on 2026-10-30 in this repository, in the step of `spec@plans@goal-standard-and-review` that
adds the goal reviewer agent: with the shipped agent's frontmatter `name` changed by one letter,
`cargo klarch install-agent-skills` then `cargo klarch check` exited 0. The mutation was reverted.

### Why it matters

An installed agent the harness registers under another name than its file's is dispatched by
neither: a reference `agent@<file name>` resolves, while the harness knows no agent of that type,
against `goal@agent-skills@installed-text-works-anywhere`.

### What would close it

A test over the shipped set, beside `each_fragment_has_its_hosts` in `path@agent-skills@src/lib.rs`
or in the core's tests of the shipped text, that compares each shipped agent's and skill's
frontmatter `name` with its install name, shown to fail on a renamed field.
