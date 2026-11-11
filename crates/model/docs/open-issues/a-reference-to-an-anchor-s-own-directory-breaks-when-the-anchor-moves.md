---
kind: question
---
# A reference to an anchor's own directory breaks when the anchor moves, where every other reference survives the move

## Summary

`design@check@deepest-anchor-wins` has a relocation of an anchor edit its one line of the manifest and no
document. The same head gives an anchor's own directory
no spelling under its own name, so it is named from an ancestor, as `path@<parent-anchor>@<dir>/`,
and `primer@where-knowledge-goes` tells every session to write it so. Such a reference names the
old location after a move, and must be repaired by hand.

## Details

### What

- The head says of that form: "a reference that names a location, which a move is expected to
  break". It is an exception the head states to its own property.
- Whether the exception is a defect depends on a bound no head argues: that a relocation must cost
  mechanical edits alone, per `issue@check@the-cost-of-a-relocation-is-bounded-by-no-argued-head`.
- `grep -rhoE 'path@knowledge-architect@(crates|tools)/[a-z-]+/' --include=*.md --include=*.rs .`
  finds no such reference in this repository's tree, so no move here has met it yet.

### Why it matters

A session moving an anchor meets a broken reference that the rest of the grammar spares it, and no goal ranks the
exception against the cost of a new spelling. It strains `design@check@deepest-anchor-wins`.

### What would close it

The discussion that `issue@check@the-cost-of-a-relocation-is-bounded-by-no-argued-head` names argues the bound.
Under a bound that admits no such exception, the checker gains a spelling of an anchor's own directory under its own
name, and the head and the primer's form change with it, under `skill@knowledge-architect-design`. Under one that
admits it, the head's sentence stands and this entry closes.
