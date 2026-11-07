---
kind: design
---
# A section of an intake's analysis that the session decides repeats its outcome as its default

## Summary

`skill@knowledge-architect-retrospective-intake@analysis-and-ruling` ends every section of an
analysis with a `Default` line and an `Outcome` line. Where the session decides the outcome, the
two lines state the same act, and in the two intakes measured almost every section is decided by
the session. Dropping the `Default` line there contradicts statements of two heads, so the change
needs a design discussion.

## Details

### What

The text, in `path@agent-skills@content/skills/retrospective-intake/SKILL.md`: "Every section ends
with a `Default` line and an `Outcome` line, whatever it proposes, no change included". The words
"whatever it proposes" leave one reading.

The instances:

- the intake of seventeen files, 2026-10-31: all 58 outcomes were the session's, so each `Default`
  line repeated its `Outcome` line. Its workflow file proposed the `Default` line only where the
  owner holds the outcome, −1 instruction;
- the intake of forty files, 2026-11-07: of the 132 `Outcome` lines of its 40 analyses, 129 open
  "decided by the session" and 3 "for the owner", at the commit "[intake] The owner's rulings on
  the three pending outcomes, and the thaum rule widened to every document", by `grep` over the
  analyses for lines opening with `- **Outcome**` and for that phrase.

The heads that state the shape:

- `design@agent-skills@intake-outcome-authority`: "The analysis keeps its shape, one section per
  finding with its evidence and its `Default` and `Outcome` lines, and the summary in the
  conversation says of each line who holds it, so that the owner sees every finding and its kind
  and can bring a fact, an argument or an intent to any of them before the merge";
- `design@agent-skills@committed-intake-analysis`: "Each finding's section ends with a default and
  its outcome, the session's decision or the owner's ruling".

The heads' reason, that the owner sees every finding and can add to any of them, is served by the
`Outcome` line alone, which the proposal keeps; so the heads' argument may not cover the `Default`
line on a decided section, which is what the discussion would establish. The nearest rival is no
change, one duplicated line per decided section, read by the owner before the merge. The fact that
decides between them is whether any act reads a decided section's `Default` line; the skill names
none: the work "proceeds on every default", and a decided outcome is the default it proceeds on.

The suspected mechanism, in one sentence: the shape was set for a section the owner rules, and the
sections the session decides, now nearly all, carry its default line with nothing left for it to
do.

Re-entry point: a design discussion under `skill@knowledge-architect-design`, in a branch whose
analyses are not being written in the shape it changes, on the owner's word or when work next
changes the shape of an analysis.

Source: W3 of the analysis 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow,
in the commit "[intake] The analyses of forty received retrospective files, three outcomes for the
owner". It was not reversed in that intake's branch, whose analyses were written in the current
shape: a shape changed while they were written would have left them in two shapes.

### Why it matters

A line no act reads fails the need test of
`skill@knowledge-architect-agent-configuration@admission-standard` at every decided section, and
the owner reads it before each merge of an intake. It strains
`design@agent-skills@intake-outcome-authority` and `design@agent-skills@committed-intake-analysis`.

### What would close it

A discussion under `skill@knowledge-architect-design` that keeps the `Default` line on every
section, with the act that reads it, or rewrites the two heads and the skill's sentence so that a
decided section ends with its `Outcome` line alone.
