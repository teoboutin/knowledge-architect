---
kind: todo
---
# `worklist.rs` is past one read and holds the work list's writer and its judge, which change apart

## Summary

`path@query@src/worklist.rs` holds both halves of the record search's command: the writer of the
work list, `--out`, and the judge of the reports with the outputs a passing check writes,
`--check` and `--verify`. It is 3118 lines, more than a session reads whole in one pass, and its two
halves change apart. Two signals for a split of `design@agent-skills@module-split-guidelines` hold,
and one signal against.

## Details

### What

The signals, counted at the review before the merge of the record search's saved workflow:

- **Size.** `wc -l crates/query/src/worklist.rs` gives 3118 lines. The Read tool reads 2000 lines
  by default, so a review that reads the file reads it in parts.
- **Two parts that change apart.** The writer, `write` and the packing it calls, and the judge,
  `verify`, `check`, `judge_reports`, `judge_report`, `outputs`, `closing_sections` and
  `write_all`. The commits "[query] A passing `worklist --verify` writes the closing sections of
  every report into `closing-sections.md`" and "[query] Each verdict file of `worklist --verify`
  holds its act verdicts whole, then its light verdicts as their lines" changed the judge alone.

The signal against: the writer and the judge read the same file formats, `agents.txt`,
`work.txt`, the snapshot and the report's line grammar, so a change to a format reads across both.
The spec of the record search's saved workflow, added by the commit "[plans] The spec of the record
search's saved workflow, with the closing sections and the verdict order a passing `worklist
--verify` writes", decided no split in that work, since it wrote into the judge alone; it weighed
the second signal, not the size.

### Why it matters

A review of a change to either half reads code outside its subject, or judges in parts a file it
cannot read whole, which `design@agent-skills@module-split-guidelines` names as the failure a split
prevents.

### What would close it

A split of the module planned and committed before the work that next changes the writer of
`worklist.rs` or a format both halves read, as `skill@knowledge-architect-planning@cutting-steps-and-slices`
says of a split; or that work's argued decision that no split pays, weighing both signals and the
one against.
