---
kind: defect
---
# `new_component` over a manifest loaded from a relative root refuses with a false reason

## Summary

`new_component`, given a manifest whose root is relative, refuses every directory as outside the project. `klarch` never
meets it, since `Manifest::find` is handed the working directory, which is absolute. A frontend calling the typed function can.

## Details

### What

Reported by an adversarial reviewer, not re-run by the session that opened this entry: a binary linking
`knowledge-architect-check`, with `Manifest::load` of the relative root `t2`, gets `/…/t2/comp1 is outside the project at t2`, and
nothing is written. `component_path` of `path@create@src/command.rs` compares the absolute candidate with the relative root.
The other typed commands gave the same output over a relative root as over an absolute one, in the same run.

### Why it matters

A refusal that names a false reason sends the reader to the wrong repair. It writes nothing, so nothing is lost. It bears on
`design@knowledge-architect@a-typed-command-prints-and-returns-its-code`, which offers these functions to a frontend.

### What would close it

`new_component` resolves the root before it compares, or states that it takes an absolute one, with a test.
