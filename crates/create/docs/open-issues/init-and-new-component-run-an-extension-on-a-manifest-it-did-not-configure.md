---
kind: defect
---
# `init` and `new-component` run an extension over a manifest no `resolve` configured

## Summary

`init` and `new_component` of `path@create@src/command.rs` parse the manifest they wrote or edited, and run `fix_then_check` over it,
without calling `extension::configure`. An extension's `prepare` then runs with no `resolve` before it, and a file it
generates is walked. It predates the slice that moved the binary to the top, whose adversarial review found it.

## Details

### What

Reproduced with a binary built as the crate documentation of `path@cli@src/lib.rs` shows, registering one extension that
generates `listing.md` and claims no table:

- `extbin init b` prints `fixed: wrote listing.md (regenerated)`, then reports `listing.md  the generated file is missing`
  beside a note that it is untracked, `FAILED: 1 finding above`, exit 1. After `git add -A` in `b`, `extbin check` prints
  `PASSED: no findings`, exit 0.
- An extension that claims `[claimed]` gets the same `listing.md` finding from `init`, and the phase-1 finding on the
  missing table only from the next `check`.
- An extension whose `prepare` asserts that its `resolve` ran panics, exit 101, with the project's files already written:
  a code outside `design@cli@exit-code-ladder`.
- `extbin new-component nc` in a committed project: `listing.md` is written with `18 documents`, since the model that wrote
  it walked `listing.md` itself, which a configured manifest leaves out. `extbin check` then reports it out of date, and
  `extbin check --fix` rewrites it with `17 documents`.

The call of `configure` in `check`'s commands is in `path@check@src/commands/history.rs`; `path@create@src/command.rs` holds none. A project with no
extension meets none of this, `klarch` among them.

### Why it matters

An extension project's `init` exits 1 over a project that passes, or panics, against the doc of `Extension::resolve`, "Called
once per manifest, before the walk", and against `design@create@init-has-no-extension-hook`, which keeps no hook but
does not exempt the hooks a run calls. `goal@check@projects-add-their-own-checks` is threatened where an extension's project
is created this way.

### What would close it

`init` and `new-component` configure the manifest they re-parse, before the walk that `fix_then_check` runs, with a test over
an extension that generates a file, in both commands. `spec@component-split@consumers-out` moves this code to `create`, so
that slice's audit reads this entry.
