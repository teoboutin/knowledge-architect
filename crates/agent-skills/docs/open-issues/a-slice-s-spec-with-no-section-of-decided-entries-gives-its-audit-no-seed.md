---
kind: defect
---
# A slice's spec with no section "What is already decided" gives its design audit no seed

## Summary

The design audit of a slice runs the record search with `--seeds-from` the slice's spec, per
`design@agent-skills@seeds-name-decisions`. A slice's spec carries only the sections its slice
fills, per `skill@knowledge-architect-planning@spec-contents`, so it may hold no section "What is
already decided", and the search then has no seed.

## Details

### What

At the design audit of the first slice of the milestone that moved the goals to the project's root,
the slice's spec
held no such section: the milestone document holds the decided entries of both slices in one
table, each row naming its slice. The session seeded the search from the milestone document, the
shape `rejected@agent-skills@slice-audit-seeded-by-the-milestone` refuses, since an entry citing a
seed is never `unrelated` and the entries citing the other slice's decisions took lines they did
not need. The alternative the installed text left, the slice's spec, gave no seed at all.

At the design audit of the second slice of the same milestone, the slice's spec held no such
section either, and `--seeds-from` it gave no seed. The session took as seeds, after the search, the
8 entries the milestone document's table gives that slice: of the 53 entries that are or cite one
of them, 7 had been graded `unrelated`, and their agents graded them again. The design audit then
added the section to that slice's spec, citing the entries the slice rewrites.

### Why it matters

An audit with no seed lets the search grade as `unrelated` an entry that cites a decision the
slice reverses, and an audit seeded from the milestone document pays the cost the `rejected` entry
measured. It strains `design@agent-skills@seeds-name-decisions`.

### What would close it

The planning skill says where a slice's decided entries are listed so that its audit finds them:
in the slice's spec, or by a form of `--seeds-from` that takes the rows of the milestone
document's table that name the slice.
