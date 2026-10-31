---
kind: question
---
# Does a reversal keep the incumbent's Component when the successor decides another Component's interface?

## Summary

`skill@knowledge-architect-decision-recording@reversal-check` says "The Component is the
incumbent's", with "two homes for one question" as its reason. When the successor decides another
Component's interface and the incumbent leaves its home in the same change, no commit holds two
homes, and the rule puts the successor in the wrong Component.

## Details

### What

The rule is `path@agent-skills@fragments/reversal-check.md`, which several texts host.

Instance: W1 of the retrospective of the session that built `klarch worklist`, 2026-10-25. Two
incumbent heads of the skills Component decided how a session groups the record by hand; their
successor decides what a checker command writes, the checker's interface. The session recorded the
successor in the checker's design home and deleted both incumbents in the same change, against the
rule, and argued it in a commit message rather than putting the rule to the owner.

The proposal: the incumbent's Component holds while the reversal leaves the question there; a
successor that decides another Component's interface goes to that Component, and the incumbent
leaves its home in the same change. The rival: keep the rule and move the successor in a separate
change, which leaves it in the wrong Component for one commit.

Not established: whether the argument reaches every reversal. The rule asks every reversal to do
its work in the same change, and its reason in the fragment, two homes for one question, is given
for the order of the search, which comes before any commit; so "no commit holds two homes" may hold
of every reversal, and the question may be what the rule's reason is rather than this case alone.

### Why it matters

The rule's reason does not reach the case, so a session either follows it into a misfiled head or
acts against it, against `goal@agent-skills@no-contradiction-in-workflow`.
`issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`
is a neighbour on the audit's side.

### What would close it

A discussion under `skill@knowledge-architect-design` that rewrites the fragment's bullet, or
records why the rule holds in this case.
