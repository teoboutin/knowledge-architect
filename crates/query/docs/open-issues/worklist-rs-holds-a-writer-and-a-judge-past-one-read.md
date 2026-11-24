---
kind: design
---
# `worklist.rs` is past one read and holds the work list's writer and its judge, which change apart

## Summary

`path@query@src/worklist.rs` holds both halves of the record search's command: the writer of the
work list, `--out`, and the judge of the reports with the outputs a passing check writes,
`--check` and `--verify`. A session cannot read it whole in one pass, and its two halves change
apart. Two signals for a split of `design@agent-skills@module-split-guidelines` hold, and one signal
against. No work has yet needed the split, so it is weighed again at the design or the planning of
the next work that changes the file's code, per `design@agent-skills@split-issue-reenters-at-planning`.

## Details

### What

The signals:

- **Size.** A Read of the file with no range returned lines 1 to 1263 of 3123, with the message
  "showing lines 1-1263 of 3123 total (52538 tokens, cap 25000)", measured on the branch that wrote
  `design@agent-skills@split-not-needed-is-an-issue`; so a review that reads the file reads it in
  parts.
- **Two parts that change apart**, counted at the review before the merge of the record search's
  saved workflow. The writer, `write` and the packing it calls, and the judge, `verify`, `check`,
  `judge_reports`, `judge_report`, `outputs`, `closing_sections` and `write_all`. The commits
  "[query] A passing `worklist --verify` writes the closing sections of every report into
  `closing-sections.md`" and "[query] Each verdict file of `worklist --verify` holds its act
  verdicts whole, then its light verdicts as their lines" changed the judge alone.

The signal against: the writer and the judge read the same file formats, `agents.txt`,
`work.txt`, the snapshot and the report's line grammar, so a change to a format reads across both.

The instances: the spec of the record search's saved workflow, added by the commit "[plans] The spec
of the record search's saved workflow, with the closing sections and the verdict order a passing
`worklist --verify` writes", decided no split in that work, since it wrote into the judge alone; it
weighed the second signal, not the size. The branch that wrote
`design@agent-skills@split-not-needed-is-an-issue` added one `# Errors` section to the file and
weighed no split, since a doc comment is no change of its code.

A later instance: the commit "[tests] The tests outside `testing` hold each scratch directory in a
guard of `tempfile`, which removes it when the test ends" changed the file's test code, and its
branch weighed a split in its design discussion. The work needs none: it changes how a test makes a
directory, one pattern at each site, and moves no subject, so a split would not shorten what its
review reads. The weighing passes to the next work that changes the file's code after it.

### Why it matters

A review of a change to either half reads code outside its subject, or judges in parts a file it
cannot read whole, which `design@agent-skills@module-split-guidelines` names as the failure a split
prevents.

### What would close it

A split of the module planned and committed before the work that next changes the writer of
`worklist.rs` or a format both halves read, as `skill@knowledge-architect-planning@cutting-steps-and-slices`
says of a split; or that work's argued decision that no split pays, weighing both signals and the
one against.

### Suspected mechanism

The judge grew in the module of the writer, by works that wrote into one half and weighed no split.

### Re-entry point

The design discussion or the planning of the next work that changes the code of
`path@query@src/worklist.rs`.
