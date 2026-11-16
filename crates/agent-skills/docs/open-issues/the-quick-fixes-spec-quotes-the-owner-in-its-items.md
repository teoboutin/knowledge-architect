---
kind: question
---
# The quick-fixes spec quotes the owner in its items, which the attribution rule refuses

## Summary

`spec@plans@path-quickfixes` holds the owner's words verbatim in the items of its `deliberation.md`,
where `design@agent-skills@attribution-replaces-quotation` has a plan document's items attribute
them. Should the quotations be turned into attributions, or does the rule admit a plan document
that was moved into its directory with no other edit?

## Details

### What

The slice that made plan documents directories moved the spec into its directory with no other
edit, its words kept as written, the owner's verbatim words included: the owner proposed that
migration of `spec@plans@path-quickfixes`, minimal, and approved it, in the discussion of the milestone on split guidelines and plan directories,
whose documents left the tree in the commit "[plans] The milestone on split guidelines and plan directories leaves: its last slice landed, and every default is ruled".
Its `deliberation.md` quotes the owner in several items, the ruling on its thread
#fix-admission among them, and the spec has no committed transcript that keeps those words elsewhere.
The rewritten head states no exception for such a document, and this entry is the one record of the
exception: the thread that ruled it is printed by `git show <that commit>^:<path>`, its path
`path@elsewhere@docs/plans/milestones/split-guidelines-and-plan-directories/harvested/plan-directories/deliberation.md`.

### Why it matters

The tree holds a live plan document that the head
`design@agent-skills@attribution-replaces-quotation` refuses, with this entry alone recording why, so that once it closes while the spec stays nothing does, against
`goal@documentation-stays-consistent`; a session that reads the spec as precedent copies the
quotations into a new plan document.

### What would close it

One of: the spec leaves at its landing; its design audit, which
`issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` already asks for, turns
each quotation into an attribution and keeps the words in a committed file; or the head gains the
exception for a plan document migrated with no edit, argued under `skill@knowledge-architect-design`.
