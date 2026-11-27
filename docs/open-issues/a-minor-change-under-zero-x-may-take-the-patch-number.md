---
kind: question
---
# A minor change under 0.x may take the patch number, as the Cargo book admits, where this project and its bump check take the second number

## Summary

Under `0.y.z`, this repository's versioning policy and the changelog's bump check give a minor
change the second number, `0.(y+1).0`. The Cargo book also admits the third: releases under
"0.y.z" can treat a change of `y` as major and a change of `z` as minor. The owner reported having
chosen the second number believing it the usual practice, and asked for a discussion of whether to
follow the book's reading here and in the check.

## Details

### What

- `design@knowledge-architect@versioning-policy`, its bullet "Under 0.x, Cargo has two positions":
  "while at 0.x, major and minor both bump 0.MINOR, and patch bumps 0.x.PATCH".
- `design@check@changelog-bump-check` applies the same reading to every project that declares
  `[changelog]`: a release whose highest class is minor must follow its predecessor `0.y.z` as
  `0.(y+1).0`, and `0.y.(z+1)` with a minor entry is a finding.
- The Cargo book's semver chapter, as `path@check@docs/grounding/cargo-semver.md` quotes it:
  releases starting with "0.y.z" "can treat changes in "y" as a major release, and "z" as a minor
  release". Under that reading a minor change gives `0.y.(z+1)`.
- The owner reported, at the ruling on D14 of `milestone@plans@per-crate-changelogs`, having
  chosen the second number for this project believing it standard practice, without knowing that
  the usual 0.x practice may put a minor change in the patch number; the owner kept the built
  reading for now and asked for this discussion.

The question: does this project's versioning policy, the bump check, or both, take the book's
reading, admit both readings, or let a project choose one?

### Why it matters

It strains `design@check@changelog-bump-check`: an adopter that numbers its 0.x releases as the
book admits gets a finding on every minor release once it declares `[changelog]`, which bears on
`goal@every-target-project-can-adopt-it`. The check is opt-in, so no adopter meets it until it
declares the table. It also bears on `design@knowledge-architect@versioning-policy`, whose reading
the check takes; a change there changes how this project numbers its own releases.

### What would close it

A design discussion under `skill@knowledge-architect-design`, reading the Cargo book's semver
chapter and the resolver's reading of a `0.y` requirement, with the shapes so far:

1. keep the second number for a minor change, here and in the check;
2. the book's reading, a minor change in the third number, here and in the check;
3. the check admits both `0.y.(z+1)` and `0.(y+1).0` for a minor change, and this project keeps its
   own policy;
4. a key of `[changelog]` names the reading a project follows.

The discussion picks one; the outcome rewrites `design@check@changelog-bump-check`, and
`design@knowledge-architect@versioning-policy` where the project's own numbering changes, or
records why both stay.

Re-entry point: a design discussion the owner opens on it, or the next change to the bump check's
rule, whichever comes first.
