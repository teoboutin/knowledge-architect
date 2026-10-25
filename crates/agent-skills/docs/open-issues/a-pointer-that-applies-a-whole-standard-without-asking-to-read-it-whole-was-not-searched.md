---
kind: todo
---
# A pointer that applies a whole standard, with no instruction to read it whole, was not searched for fragments

## Summary

The work of `spec@plans@standards-delivered-as-fragments` found the blocks of the installed text
read whole at several places by searching the lines that hold "whole" or "again" and the pointers
beside them. A pointer that applies a whole standard with no such word near it, written as "as X
says" or with another word such as "in full", was not found. The review before the merge of that
work found one, the decision-record reviewer's pointer to
`skill@knowledge-architect-decision-recording@three-homes`, "Read the last in full".

## Details

### What

The search, at the base of that work: every line of `path@agent-skills@content/` holding "whole",
`%%` lines excluded, and each pointer near it to a section of installed text, grouped by target and
read by hand. Its outcome is the spec's candidate table. Pointers that ask for no reading and
apply a whole section were outside it. A sweep would list every pointer of the installed text to
an installed section from a text other than its home, keep those whose act applies the section
whole, and judge each by the test of `design@agent-skills@shared-text-fragment`, that a host
applies the whole fragment.

### Why it matters

A block applied whole behind a pointer costs every session that applies it the lookup, and is
applied from memory where the session skips it, which
`design@agent-skills@context-cost-of-agent-text` counts as the cost a fragment removes, against
`goal@agent-skills@no-contradiction-in-workflow` where the memory is wrong.

### What would close it

The sweep done, each pointer it keeps judged: migrated to a fragment with its hosts in the hosts
test, or kept behind its pointer with the reason.
