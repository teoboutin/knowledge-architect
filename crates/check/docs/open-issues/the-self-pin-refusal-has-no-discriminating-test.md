---
kind: todo
---
# The refusal of the `"self"` pin has no test that fails when its question to git is removed

## Summary

A guard of the checker's binary survives the removal of its logic, found by an adversarial reviewer
with `cargo x mutate run` over the suite of the workspace. It predates the slice that found it.

## Details

### What

- `refuse_another_version` of `path@check@src/commands/mod.rs`: the closure that asks git whether the project
  tracks the compiled directory's `Cargo.toml`, `.is_ok_and(|out| !out.is_empty())`, mutated to `.is_ok()`,
  survives. The unit test of `design@check@checked-sentinel-values` hands in a `tracked` closure of its own, so
  the real closure is exercised on the tracked side only.

The `"self"` case is harder to reach from a binary test than the closing section suggests:
`is_its_own_build` in `path@check@src/commands/mod.rs` calls the `tracked` closure only when the
compiled-in directory of `check`, `crate::component_dir()`, lies inside the project, since it asks
`strip_prefix` of the project's root first. A sandbox outside this checkout, where the binary tests
build their projects, never reaches the closure. Such a test needs a fixture placed so that the
compiled-in directory lies inside it, or the closure tested through
`refuse_another_version_built_at`, which takes the directory and the closure as arguments.

### Why it matters

`design@check@a-foreign-build-is-refused` and `design@check@checked-sentinel-values` state both guards, and no
test would report either one lost. A stale copy of the installed text, or a registry's copy of the checker
inside a project, would then pass, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A binary test for each: a stale `agent-skills` crate refused as a foreign build, and a project holding an
untracked copy of the crate refused under `"self"`, each shown to fail by the mutation above.
