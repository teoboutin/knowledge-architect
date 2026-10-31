---
kind: question
---
# A reference to an anchor's own directory breaks when the anchor moves, which the relocation goal says no move does

## Summary

`goal@core@relocation-is-one-manifest-edit` is met while moving a Component or a location "leaves
no reference to repair". `design@core@every-path-names-its-anchor` gives an anchor's own directory
no spelling under its own name, so it is named from an ancestor, as `path@<parent-anchor>@<dir>/`,
and `primer@where-knowledge-goes` tells every session to write it so. Such a reference names the
old location after a move, and must be repaired.

## Details

### What

- The head says of that form: "a reference that names a location, which a move is expected to
  break". The goal admits no such exception.
- Found by the goal reviewer's whole-set reading of this repository's goals on the branch that
  brought them to the goal standard. The session's default there was to leave the goal's text
  unchanged and treat the primer's form as the defect.
- `grep -rhoE 'path@knowledge-architect@(crates|tools)/[a-z-]+/' --include=*.md --include=*.rs .`
  finds no such reference in this repository's tree, so no move here has met it yet.

### Why it matters

A recorded decision conflicts with a goal, so per `primer@goals-bind` the goal prevails and the
conflict goes to the owner. Until it is answered, a session moving an anchor meets a broken
reference that the goal says cannot exist, and a session judging the head against the goal finds
the head in defect.

### What would close it

The owner's answer, under `skill@knowledge-architect-goal-setting`, to one of:

- the goal admits the exception the head states, a reference naming an anchor's own directory being
  a reference to a location;
- the checker gains a spelling of an anchor's own directory under its own name, and the head and the
  primer's form change with it, under `skill@knowledge-architect-design`.
