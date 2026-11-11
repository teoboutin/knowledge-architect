---
kind: design
---
# The labels head and the primer give two answers for a label that reaches a branch's commit messages

## Summary

`design@agent-skills@ruled-items-labelled` gives a label that reaches a committed document a prefix
of its own kind, and reserves `Q` for a label that stays in the conversation. `primer@labelled-items`
counts the commit messages of one branch as a record a label reaches, and admits `Q<n>` there. A
question put to the owner during a branch's work and recorded in its commit messages takes `Q` by
the primer and a prefix of its own kind by the head, which lists none for it.

## Details

### What

The instance: the branch that triaged the issue record put fourteen questions to the owner as
`Q1` to `Q14`, in the conversation, and its commit messages record each ruling by its label, "Q1 of
this branch's questions". The design-conformance reviewer of its review before the merge reported
the conflict: the head's table gives `Q` the committed document "none", and one of that branch's
commits argued against a rival on that very reading, while the primer's text admits the label.

- The head, in `register@agent-skills@design`: "A label that reaches a committed document, even one
  that leaves later, such as a plan document, takes a prefix of its own kind, distinct across the
  installed workflow. A label used only in the conversation reuses the common prefix `Q`."
- The primer, `primer@labelled-items`: an item "under a label, `Q<n>` or the prefix a skill gives
  it, takes the next number of its prefix in the record the label reaches", where that record is "a
  plan document, …; a file; or the commit messages of one branch, counting as one".
- `skill@knowledge-architect-review@what-review-leaves` puts several review items to the owner as
  `Q<n>` and has the record of the review name each item by its content, not by the label.

Suspected mechanism: the primer's list of records was written for the numbering, which needs every
record a label reaches, and the head's split of prefixes was written before the commit messages of a
branch counted as one.

The re-entry point: the next change to `design@agent-skills@ruled-items-labelled` or to
`primer@labelled-items`, or the next review that reports a `Q` label in a commit message.

### Why it matters

`design@agent-skills@no-two-installed-instructions-contradict` has each installed instruction say
what it requires plainly enough to follow. A session recording the owner's rulings in its commit
messages meets two answers, and two branches may label the same kind of question two ways, against
the reason the head gives, that the owner tells a kind of item by its label.

### What would close it

A design decision, under `skill@knowledge-architect-design`, on whether a question whose label
reaches only a branch's commit messages keeps `Q`, the head's table then giving `Q` that record, or
takes a prefix of its own, the primer's sentence and the skills that put such questions then naming
it.
