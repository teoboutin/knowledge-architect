---
kind: design
---
# `references.rs` is past one read

## Summary

`path@check@src/check/references.rs` is more than a session reads whole in one pass: a Read of
every line was refused. The size signal for a split of
`design@agent-skills@module-split-guidelines` holds, and a second signal holds besides: two rules
threaded through the same arms. The first work that changed the file's code with the signal known
weighed it and took no split, so it is weighed again at the design or the planning of the next
work that changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

- **Size.** A Read of every line, with no size override, was refused at 66159 tokens, against a
  reader's limit of 25000, by the code-split scout dispatched at the design audit of slice 1 of
  `milestone@plans@per-crate-changelogs`, at the commit "[review] Repairs of the transcript review:
  D2 closed by the session on the owner's argument, and the owner's words restored where the record
  changed them". Most of the size is the inline tests: the code above `#[cfg(test)]` was 38381 of
  177432 bytes, a figure taken from bytes, not from a Read (`not established` that the code part
  alone fits one read).
- **A second reference.** The rule of `design@check@goals-home-cites-only-goals-and-root-homes` is
  threaded through every arm of `judge_part`, `id_lint` and `link`; slice 1 threads the rule of a
  citation-free file through the same arms, so a change to what a goals home admits and a change to
  what a citation-free level admits each touch those arms on their own.
- **The signals against.** Each rule changes the finding one arm writes for one observation, so a
  review of either rule reads that arm; and `judge_part` and `judge_ids` are also called by
  `path@check@src/commands/history.rs` over commit messages.

The instance: slice 1 of that milestone changed the arms and put the citation-free rule's own
logic in a module of its own, and its design audit decided no split.

### Why it matters

A review that reads the file in parts judges code it has not read whole, which is the failure the
size signal names in `design@agent-skills@module-split-guidelines`.
`goal@projects-are-split-into-modular-units` asks for units with one purpose.

### What would close it

A split of the module planned and committed before the next work that changes its code, as
`skill@knowledge-architect-planning@cutting-steps-and-slices` says of a split, such as the tests
moved out or the per-rule repairs gathered behind one function; or that work's argued decision that
no split pays, weighing the signals above.

### Suspected mechanism

Each rule over a span's finding was added to the arms that write the finding, and none weighed a
split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@check@src/check/references.rs`.
