---
kind: todo
---
# Arms of the resolver and of the lints beside it have mutations no test catches

## Summary

An adversarial review of the crate split found mutations of the resolver and the lints that survive both packages' suites; each predates the split, whose differential showed no change in behaviour.

## Details

### What

Each mutation below survived `cargo test -p knowledge-architect-model -p knowledge-architect-check -p knowledge-architect-query -p knowledge-architect`.

- `without_suffix`: the second strip of a fragment dropped, so `docs/design.md#x:12` keeps a suffix.
- `resolved_here`: `if !bare.starts_with('/')` made always true, against "a leading `/` from the root alone".
- `path`, the escape arm: `suffix` always false, so the sentence on dropping a line number goes.
- `register`, `carriers`: `!a.is_plan()` dropped from its filter.
- `table`: an undefined item taking the home repair rather than the section repair.
- `anchored_target`: the entry test taking the path with its suffix on, so a suffix on an entry's path stops naming the entry.
- `ignore_queries`: the generic form asking about the first anchor alone, and the link arm's `refused` test dropped.
- `link` of `path@check@src/check/references.rs`: the finding showing the link without its fragment.
- `judge_part`: a harness finding's line shifted, `planned` no longer skipped in a goals home, and the document's own directory left out of the search that decides whether to offer `planned`.
- `show`: the two texts of a cause swapped.
- The words of about two dozen causes in `Cause::texts`.

A differential of the binaries built before and after the split, over a generated project, wrote 83565 findings and was byte-identical but for the checker-source line, so these are gaps in the suite and not defects of the move.

### Why it matters

`design@check@finding-names-the-repair` and `design@model@an-unresolved-reference-is-reported` state what these arms do, and a change to one fails no test. Whether the checker's own tests should pin a cause's wording is part of this: a rewording is no change to what a check reports.

### What would close it

A test for each logic arm above, each with its recorded mutation, and a decision on whether the words are pinned.
