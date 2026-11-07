---
kind: design
---
# An owner's interrupt with no `interruptedMessageId` is unlabelled at top level and read as `dispatch` in a subagent file

## Summary

`transcript` labels the owner's interrupt of a turn by its `interruptedMessageId` field, never by
its text. Since harness 2.1.316, interrupts carrying no such field appear: at top level they are
left unlabelled, and in a subagent file they are labelled `dispatch`, the dispatcher's text. A
transcript reviewer reading the owner's labels then misses the owner's stop, or reads it as the
dispatcher's words.

## Details

### What

`design@transcript@transcript-labels-in-code` decides the rule: a line is labelled "by its fields,
never by a substring of its text"; `owner-interrupt` is placed by `interruptedMessageId`; an
interrupt in a subagent file with no such field "reads as `dispatch`: no field tells it apart, and
the rule refuses a text test". It measured one such interrupt, and every other carrying the field,
over Claude Code 2.1.306 to 2.1.316 on 2026-10-31.

Instances seen, measured at the intake of 2026-11-07 on the owner's machine:

- top level: 2 lines of this repository, harness 2.1.316, of 2026-11-03 and 2026-11-04, whose text
  is "[Request interrupted by user]" and which carry no `interruptedMessageId`.
  `klarch transcript show 9686e08f` prints "L2037 unlabelled" and `klarch transcript show 868d26f8`
  prints "L1213 unlabelled" before that text. They are the "unlabelled lines: 2" of
  `klarch transcript list`'s header over this project; over every project the header counts 4, the
  other two being the pair of `issue@transcript@owner-shell-commands-are-unlabelled`;
- subagent files: 27 lines whose text starts "[Request interrupted" and which carry no
  `interruptedMessageId`, 26 of them on or after 2026-11-01, each labelled `dispatch`. Taken with the
  head's own selection:

```sh
find ~/.claude/projects -path '*subagents*' -name 'agent-*.jsonl' -exec jq -r 'select(.type=="user" and .interruptedMessageId == null) | .message.content | if type=="string" then . else ([.[] | .text? // empty] | join(" ")) end | select(startswith("[Request interrupted"))' {} + | sort | uniq -c
```

The header's unlabelled count was reported by retrospective files of four sessions of this
repository, 2026-11-04 to 2026-11-07, as 1, 2, 4 and 2; the 4 is over every project.

Suspected mechanism: the harness, from 2.1.316, writes some interrupts without the field, and the
head's rule then has no field to place them by. `not established`: which harness events write an
interrupt without the field.

`goal@transcript@a-session-reads-past-sessions` rules out "labelling a line the owner typed by a
phrase in its text"; an interrupt is no typed line, so the goal does not rule out a text test here,
and the head does.

### Why it matters

A transcript reviewer that reads the owner's labels misses the owner's stop, or reads it as the
dispatcher's text, against `goal@knowledge-architect@the-owner-s-word-is-verified`. Labelling by the
text reverses part of `design@transcript@transcript-labels-in-code`.

### What would close it

The re-entry point is the discussion of `issue@transcript@owner-shell-commands-are-unlabelled`,
held for both entries, since both ask whether a text test may place a line of the owner's act that
no field places: it admits a text test for such a line, rewriting the labels head, or keeps the
head and names the shapes in the transcript reviewer's text; with the head's measurement taken
again and a test over a fixture holding each shape.
