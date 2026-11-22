---
kind: design
---
# Reviews of separate branches keep finding a head's title wider than its argument, so the rule that a title states the rule is reopened

## Summary

The tripwire that guarded `design@agent-skills@title-states-the-rule` on the premise that a title
moved to the rule stays within what its argument argues fired: the reviews of at least five
separate branches merged after it was written each found a title wider than its argument in a head
their branch wrote or rewrote. Its response is to reopen that decision, which is a design discussion
with the owner; this entry holds the firing and its evidence until that discussion runs.

## Details

### What

The tripwire, `rule-title-wider-than-its-argument`, was written in the commit "[audit] Pass 1 of
the tripwire axis applied: every tripwire kept has a horizon, and 20 leave" (2026-10-24). Its clause:
"the reviews of two separate branches each find a title wider than its argument in a head their
branch writes or rewrites, or an audit finds one on the main branch in a head written or rewritten
after the decision". Its response: "reopen `design@agent-skills@title-states-the-rule`; the title
found is repaired as the instance".

Instances on the main branch since that commit, each a review repair of a title its own branch
wrote or rewrote, each repaired in that branch:

- pull request 38, slice 3 of the component skill, the commit "[review] Repairs of the
  decision-record review of slice 3's repairs: the fragment rule kept to the owner's member, and a
  title brought to its argument" (2026-11-02): its F3, the title of
  `design@agent-skills@component-split-signals` naming the nested Components its body no longer
  argued; the head was written in that branch, by the commit "[agent-skills] The installed skill for
  creating or splitting a Component, its fragment in the design and development skills, and their
  record";
- pull request 39, slice 4, the commit "[review] Repairs of the decision-record review of slice 4's
  repairs: the issue's second case only narrowed, and the timing head's title kept to its argument"
  (2026-11-02): its F2, a title clause that the record search is the grounding's last part, "argued
  nowhere in the head"; the head was split in that branch, by the commit "[review] Repairs of the
  review before the merge of slice 4: the timing head split in two, its argument restored, and an
  issue kept open on its unanswered case";
- pull request 43, the commit "[review] Repairs of the review before the merge of the slice that
  moves git out of the core" (2026-11-02): `design@git@git-is-asked-through-one-module`, "its title
  was wider than its body", as its message states;
- pull request 57, the commit "[review] The repairs of the review before the merge of the goals'
  second slice" (2026-11-11): "the new title of `design@model@register-home-reference` wider than
  its body", as its message states;
- pull request 73, the commit "[review] Repairs of the second pass on the review's repairs: step
  3's scope restored, the git library's head titled to its argument, and the install's head kept to
  its decision" (2026-11-14): its F1, a head of the git Component "titled wider than its argument",
  retitled and renamed `design@git@the-library-brings-no-parse-of-the-model-into-a-tool-s-build`.
  That message counted F1 as "one finding of this branch's reviews, not a firing", reading the
  clause branch by branch; the clause counts separate branches, and the two first instances already
  met it.

The pull request numbers are given by `gh api repos/{owner}/{repo}/commits/<commit>/pulls` on each
commit. The first two instances are checked against the commit that wrote the head; the other three
rest on what their messages state, `not established` beyond that. No instance was found by an
audit: no pass of the design-record axis ran since the tripwire was written.

This is no slip of the rule on slips: the clause watches the same failure recurring in separate
pieces of work, which is what the instances show.

The suspected mechanism, in one sentence: a title moved from the members to the rule states the
rule in terms the session chooses, and those terms reach further than the argument the body
carries, a defect each review finds only in the heads its branch wrote.

Re-entry point: a design discussion under `skill@knowledge-architect-design` that reopens
`design@agent-skills@title-states-the-rule`, on the owner's word for when it runs.

### Why it matters

`design@agent-skills@title-states-the-rule` states that "a title states no more than its argument
argues", and the guard has not kept the defect out of the branches' first drafts: each instance
cost a review finding and a repair, and a title wider than its argument that a review misses
reaches the main branch as a rule its head does not argue, against
`goal@design-is-recorded-with-its-arguments`.

### What would close it

The discussion reopens `design@agent-skills@title-states-the-rule` and either changes how a title
is moved to its rule, recording the change in that head, or keeps the decision with the recurrence
recorded as its cost in the head; this entry is deleted in the commit that records the outcome.
