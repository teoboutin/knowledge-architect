---
kind: defect
---
# The check of a searcher's report refuses a quotation cut before a word's punctuation, with a message that does not say so

## Summary

`klarch worklist --verify` compares a quotation with the entry's body as whole words separated by
whitespace, the punctuation attached to a word being part of it. A quotation that stops at
"decision" where the body reads "decision," is refused. The message says the quotation is not in
the entry's own text and to copy it from the body, which the agent did. In one search, 11 of 17
reports failed their first check, most of the findings being of this kind.

## Details

### What

The test: `judge` in `path@query@src/worklist.rs` requires the quotation's words to be a run of
the body's words. Its unit test `judge_owes_a_quotation_of_the_body_and_a_reason_on_every_line`
holds the case on purpose: a quotation that ends inside the word `project.` is refused.

The message, for every such failure: "the quotation is not in the entry's own text: copy it
verbatim from the entry's body, never from its first line or from the sites that cite it".

Reproduction: in a report of a work list, give an entry a quotation of eight or more words copied
from one line of its body, ending on a word that the body follows with a comma, the comma left
out. `cargo klarch worklist --verify <dir>` reports the line with the message above.

The instance: the first record search of the discussion planned by
`milestone@plans@goals-at-the-root`, on 2026-11-10. The check reported 625 findings over 11 of the
17 reports. Two of those reports held lines never graded. For the nine others, the agents sent to
repair them each reported the same cause: the quotation ended on a word cut from its punctuation,
or began so. Each repaired its report by extending the quotation by one character. Not counted:
how many of the 625 findings were of this kind, the check's output giving one message for every
cause.

Ruled out: that the quotations crossed a line break of the body. The check accepts any whitespace
between two words, as its unit test shows.

### Why it matters

An agent that copied its quotation from the body is told to copy it from the body. It cannot tell
what to change, and a second dispatch of the agent is spent on one character per line. It strains
`design@query@worklist-writes-and-verifies-the-reports`: the message names a repair the agent has
already made.

### What would close it

One of two changes, with a test of it: the message names the first word of the quotation that
matches no whole word of the body, with the body's word it is part of; or the comparison ignores
the punctuation at the two ends of the quotation.
