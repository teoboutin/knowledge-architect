---
kind: todo
---
# Several behaviours around the installed files have no test that fails when they change

## Summary

An adversarial reviewer's mutations of code around the installed files survived the whole suite. None was written by
the branch that found them, save that it moved the family they surround.

## Details

### What

Each run with `cargo x mutate run` and a build probe, verdict `SURVIVED`:

- `repairs` of `path@check@src/agents.rs`: `Outside::Binary` dropped from the unshipped match, so `--fix` keeps a
  binary stray; `if !current` made `if true`, so an ignored current copy is rewritten on every run;
  `Some(Outside::Directory) => {}` made a write; a `Missing` copy no longer restored.
- `read_tree` of `path@check@src/commands/history.rs`: the read of every installed file under `Source::Index` narrowed to
  markdown.
- `install_agent_skills` of `path@check@src/commands/mod.rs`: the import check made `if false`.
- `finish` of `path@check@src/commands/create.rs`: its `fixed: wrote` line removed.
- `from_listing` of `path@model@src/survey.rs`: a link at an owned path left out of `installed`.

### Why it matters

`design@check@fix-scope` and `design@check@staged-check-semantics` state these behaviours, and no test would report one
lost, against `skill@knowledge-architect-development@discriminating-tests`.

### What would close it

A test per behaviour, shown to fail by the mutation above, or the behaviour judged unneeded and removed.
