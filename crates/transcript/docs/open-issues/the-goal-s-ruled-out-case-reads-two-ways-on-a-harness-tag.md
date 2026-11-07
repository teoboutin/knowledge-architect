---
kind: todo
---
# The transcript goal's ruled-out case reads two ways on a line the harness writes around what the owner typed

## Summary

`goal@transcript@a-session-reads-past-sessions` rules out, as a case, "labelling a line the owner
typed by a phrase in its text". The `<bash-input>` line a `!` command writes holds what the owner
typed, inside a tag the harness writes, so the case reads two ways on labelling it by that tag. The
owner ruled the reading: the case rules out a test that reads what a line means in the discussion,
not a tag the harness writes. The goal's words do not say so.

## Details

### What

The two readings, met while closing the transcript reader's gaps on 2026-11-07:

- **Literal**: the `<bash-input>` line is the record of a line the owner typed, and its tag is a
  phrase in its text, so labelling it `owner-shell` by the tag is ruled out.
- **The owner's**: the case rules out a test on what a line says, its meaning in the discussion,
  such as the substring filter that once dropped one of the owner's messages from a review; a tag
  the harness writes around a command carries no meaning of the discussion.

`design@transcript@transcript-labels-in-code` builds on the owner's reading, and states it as the
owner's, since the goal's words carry the literal one as well.

### Why it matters

A later session reading the goal alone takes the literal reading, and finds the `owner-shell` row
of `design@transcript@transcript-labels-in-code` in conflict with the goal. Where a decision
conflicts with a goal, the goal prevails and the conflict goes to the owner, per
`primer@goals-bind`, so the owner would be asked again what the owner has already ruled.

### What would close it

The goal's ruled-out case reworded through `skill@knowledge-architect-goal-setting`, the owner
ruling the rewording, so that it states the reading the owner gave; and the sentence of
`design@transcript@transcript-labels-in-code` that attributes that reading to the owner then cites
the goal alone.
