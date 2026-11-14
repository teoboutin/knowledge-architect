---
kind: todo
---
# `cargo x mutate run --replace` takes only a text that occurs once in the file

## Summary

`--replace` refuses a text that occurs more than once. A site whose text repeats in the file is
reached only by `--patch`, a patch the session writes by hand each time, though the patch follows
from the file and the occurrence's number with no judgement. An option naming the occurrence would
take the site.

## Details

### What

- The argument's doc comment in `path@mutate@src/lib.rs`: "The text to substitute. Refused unless
  it occurs exactly once."
- `plan_replace` in `path@mutate@src/harness.rs` refuses a count above one with
  `PlanError::NotUnique`, whose comment gives the reason: "Replacing all of them is a different
  mutation from the one the author described, and replacing the first is a coin toss." A named
  occurrence is neither: the author names the one site.
- `path@mutate@README.md`, "The two input forms", gives `--patch` the mutations that are not one
  substitution.

The instance: the slice on value checks while parsing in this repository, 2026-11-12, C1 of its
workflow file. Two declarations in one file carried the same text, `value_parser = since`, and each
had to be mutated apart. For each, the session wrote a patch by hand with `awk` and `diff -u` and
passed it with `--patch`, two extra steps per mutation. Ruled out by the file: a longer `--replace`
text spanning lines until it is unique, which failed where the two neighbourhoods differed only in a
field name three lines away.

The proposal: `--occurrence <n>` beside `--replace`, which picks the n-th occurrence and refuses an
n past the last. It keeps `design@mutate@one-command-owns-the-interval`, each apply and restore
inside the command, and `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`,
since the restore compares bytes; no head changes.

### Why it matters

A patch written by hand for a task with no judgement is the task
`goal@tasks-a-tool-does-better-are-the-tool-s` gives a tool, for a run
`design@mutate@mutation-command` already took in; met once, so its weight is low.

### What would close it

The option built under `skill@klarch-development`, with a test in `path@mutate@tests/modes.rs` of a
file holding the text twice, mutated at its second occurrence, and of an n past the last refused;
the comment at `PlanError::NotUnique`, the argument's doc comment and the README's "The two input
forms" reworded with it.
