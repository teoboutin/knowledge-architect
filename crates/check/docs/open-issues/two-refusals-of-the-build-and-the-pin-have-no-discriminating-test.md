---
kind: todo
---
# Two refusals of a foreign build and of the `"self"` pin have no test that fails when they are removed

## Summary

Two guards of the checker's binary survive the removal of their logic, each found by an adversarial reviewer
with `cargo x mutate run` over the suite of the workspace. Both predate the slice that found them.

## Details

### What

- `refuse_another_version` of `path@check@src/commands/mod.rs`: the closure that asks git whether the project
  tracks the compiled directory's `Cargo.toml`, `.is_ok_and(|out| !out.is_empty())`, mutated to `.is_ok()`,
  survives. The unit test of `design@check@checked-sentinel-values` hands in a `tracked` closure of its own, so
  the real closure is exercised on the tracked side only.
- `main` of `path@cli@src/main.rs`: the entry of `knowledge_architect_agent_skills` in the libraries the
  foreign-build refusal compares can be removed and the suite passes. `path@cli@tests/binary.rs` holds a test
  for each of the three other libraries.

### Why it matters

`design@check@a-foreign-build-is-refused` and `design@check@checked-sentinel-values` state both guards, and no
test would report either one lost. A stale copy of the installed text, or a registry's copy of the checker
inside a project, would then pass, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A binary test for each: a stale `agent-skills` crate refused as a foreign build, and a project holding an
untracked copy of the crate refused under `"self"`, each shown to fail by the mutation above.
