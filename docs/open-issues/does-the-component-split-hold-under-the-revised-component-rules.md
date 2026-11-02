---
kind: question
---
# Does the plan of the Component split still hold now that the revised rules for splitting a project into Components are built?

## Summary

`milestone@plans@component-split` cited the spec that revised the rules for splitting a project
into Components, which left once its work landed. The milestone was written against the rules as
that spec planned them, so its slices are read again against the rules as they landed.

## Details

### What

The spec that left planned the revision of `skill@knowledge-architect-component-creation` and of
the code-shape fragment, `path@agent-skills@fragments/default-code-shape.md`, and the milestone
applies those rules to this repository. The landing departed from the spec at three points, each
argued in the commits that landed it:

- the module-directory exception holds only for a cycle that Cargo's rules force whatever the
  project decides, as the spec decided, and not for a publication a decision causes;
- the rule on a top-level binary crate and the rule that a command is parsed in the crate nearest
  the binaries among those every binary offering it depends on are two comment lines of the skill,
  with no head, since the skill is the one text that states either; the "lowest crate" of the
  milestone's #command-parsing-home reads "the crate nearest the binaries", the reading its own
  examples give;
- the integrated-test property conditions its first branch on an effect a consumer could see.

The milestone's citations of the spec were removed, and its documents re-pointed to say that
`spec@component-split@cli-on-top` writes the two heads of `register@agent-skills@design` once its
`cli` head is their second text.

### Why it matters

The milestone's slices cite what the spec harvested: `design@agent-skills@component-crate-mapping`,
`design@agent-skills@component-split-signals`, `design@agent-skills@shared-test-component`,
`design@agent-skills@integrated-test-through-public-api`, `design@agent-skills@default-code-shape`
and `design@agent-skills@test-economy`. A slice built on the spec's wording where the landing
departed from it would build a shape the installed rules no longer state.

### What would close it

Reading `milestone@plans@component-split` and its slice specs against those heads and the
installed component-creation skill, at the audit of the milestone's next slice, which reads this
issue; then each gap applied in place in the milestone's documents, or ruled as that audit sorts
it, and this entry deleted in the audit's commit.
