---
kind: design
---
# No text says who judges where a new test over a project sits

## Summary

`design@testing@binary-tests-in-testing` places in this Component a test of the crates `klarch`
links that needs the shared means of running over a project. A goal of this Component held that a
review before a merge judges each such test's placement. No head and no installed text gives a
review that duty, and nobody has argued it.

## Details

### What

**The instance.** The statement, as the Component's goal held it in what showed the goal met: "and a
review before a merge that adds or moves such a test judges it against this goal." It bound the
review before a merge of a branch that adds or moves a test over a project: one of its reviewers
judges whether the test sits in the Component the line of
`design@testing@binary-tests-in-testing` gives it.

`path@testing@tests/dependency_rules.rs` holds part of the line mechanically: no crate of the
workspace names this one. Nothing holds the other part, that a test needing the shared means does
not sit in another crate with a copy of them.

**Suspected mechanism**: the statement was a clause of what showed a goal met, written when the
goal was ruled, and no review axis was given the duty.

**The re-entry point**: the discussion that reopens the parked thread on whether a review axis
judges work against the goals, which `milestone@plans@goals-at-the-root` records, or the first
review that finds such a test placed outside this Component.

### Why it matters

A test of `cli`, `check` or `model` that copies a mock project or builds `klarch` by its own means
passes every gate, against `design@knowledge-architect@mock-projects-in-one-crate`, and no
procedure names who reports it.

### What would close it

A decision, reached under `skill@knowledge-architect-design`, on who judges the placement: a review
axis that already reads a branch's tests, a test of `path@testing@tests/dependency_rules.rs` that
finds a copy of the shared means in another crate, or nobody, with the reason. Its nearest rival is
the present state, in which the head alone states the line.
