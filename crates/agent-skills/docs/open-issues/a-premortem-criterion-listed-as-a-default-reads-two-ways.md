---
kind: defect
---
# An acceptance criterion from the premortem, listed among the defaults awaiting the owner, is both judged and not judged before the ruling

## Summary

The planning skill gives two acts for one case: an acceptance criterion the premortem proposed,
whose ruling the plan document lists among the defaults awaiting the owner. One sentence says a
default awaiting the owner is built as written; another says a criterion awaiting the owner is not
judged before the ruling. A session working the spec cannot tell whether to build and judge the
criterion's test, or to leave it until the owner rules.

## Details

### What

Two sentences of `skill@knowledge-architect-planning`, read in the installed text of this
repository at the branch that moved the checker's model into the primer:

- `skill@knowledge-architect-planning@acceptance-criteria`: "A criterion first proposed after the
  premortem, at the assembly, by a review or at an audit, is written as an item of the acceptance
  criteria section marked as awaiting the owner, [...] It is not judged before the ruling, and the
  work goes on without it".
- `skill@knowledge-architect-planning@working-a-slice`, "Where the owner's word is awaited": "A
  default the plan documents list as awaiting the owner is built as they write it until the owner
  rules on it".

The first sentence scopes itself to a criterion proposed after the premortem. A criterion the
premortem proposed is put to the owner at the premortem, per
`skill@knowledge-architect-design@discussion-loop` step 7, and when the owner has not ruled by the
assembly, the plan document lists its ruling as a default awaiting the owner, as the spec of that
branch did under its label D2. Neither sentence says which governs that criterion: the first
excludes it by its scope and then states the rule a reader applies by analogy; the second
covers it as a listed default.

Reported by the cold-implementer and the self-consistency reviewers of that spec, each reading
"Awaiting the owner, D2 [...] Judged at step 2" as two readings. Not established: whether a
session has acted on either reading, since the owner ruled D2 before the step that judges the
criterion.

### Why it matters

An instruction whose words allow two acts is a defect provable by reading, per
`skill@knowledge-architect-agent-configuration@admission-standard`. Built and judged, the criterion
fires or passes on a decision the owner may still decline; left unjudged, the step that should
judge it lands without its check, and the landing's report has nothing to say on it. It strains
`design@agent-skills@acceptance-criteria-on-the-owners-word` and
`design@agent-skills@default-ruled-before-merge`.

### What would close it

One sentence in `skill@knowledge-architect-planning@acceptance-criteria` that states which rule a
premortem's criterion awaiting the owner follows, the other sentence pointing to it, argued under
`skill@knowledge-architect-design` where it changes either head named above.
