---
kind: design
---
# The head on how work reaches main bundles statements that each lose to a rival of their own, with no argument

## Summary

`design@knowledge-architect@git-flow` holds, as bullets, several statements that each answer
another question and lose to a nearest rival of their own, and carries no standing argument for
them and cites no goal. Splitting it in the branch that triaged the issue record moved out four
decisions; the decision-record review of that branch found the rest.

## Details

### What

The statements the head holds, each with the rival the reviewer named:

- all work happens on a branch, and a draft pull request is opened at its first commit; rival: work
  committed straight to main, or a pull request opened only when the work is ready;
- the branch is rebased on main before its review; rival: a rebase only at the merge;
- the branch is reviewed before any merge; rival: a merge without review;
- work whose intermediate trees cannot pass lands as one commit, squashed before review; rival: an
  appended fix;
- main's history is never rewritten; rival: a force push to main.

Its title, "merged up to date with main", states what
`design@knowledge-architect@main-is-guarded-by-a-ruleset` decides. Moved out already:
`design@knowledge-architect@ci-skips-a-draft`, `design@knowledge-architect@main-is-guarded-by-a-ruleset`
and `design@knowledge-architect@rebase-merge-alone`.

Suspected mechanism: the head grew one bullet per rule of the git workflow, each argued in the
commit that added it, and was never brought to one decision per head.

The re-entry point: the next pass of the design-record axis of a project audit over this
repository, which `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis` holds,
or the next change that edits the head.

### Why it matters

`primer@design-heads` asks a head for one decision and its standing argument. A session that weighs
a change to one of these rules finds no argument to defeat, and a reference to the head cannot say
which statement it depends on; the root `CLAUDE.md`'s `## Git` section takes the head as its home.

### What would close it

Each statement given its argument and, where it passes an entry test, a head of its own, or
recorded as no decision, through `skill@knowledge-architect-decision-recording`, the head's title
brought to what remains.
