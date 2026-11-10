---
kind: design
---
# No head argues that a project can run the checker from its git pre-commit hook

## Summary

`design@git@index-file-variable-kept` holds how a pre-commit hook reads the index its commit records. No head argues
the capability that mechanism serves: that a project can judge a commit's tree from the hook, before the commit
exists.

## Details

### What

The statement nobody has argued, with what it bound:

- "A project can run the checker from its git pre-commit hook, the hook git runs before it records a commit, to judge
  the tree that commit records, before the commit exists." It bound the checker to run under the variables git gives
  a hook. `design@git@index-file-variable-kept` rules its rival out by it: refusing to run with `GIT_INDEX_FILE` set
  "leaves a pre-commit hook unable to run the checker at all". `design@git@repository-variables-dropped` takes from
  it the project a hook covers.

The instances seen: those two heads. `design@check@staged-tree-source` gives the motive of `check --staged`, a
partial commit, and names no hook. `path@check@README.md` tells a user that `check --staged` runs as a pre-commit
hook.

The suspected mechanism: the capability was held as a statement no argument had to defend, so the head on the hook's
index argued the mechanism alone.

### Why it matters

A change that would stop the checker running under a hook's variables is ruled out by `design@git@index-file-variable-kept`
on a capability no head states, so a session cannot weigh that change against it. It strains that head.

### What would close it

A design discussion under `skill@knowledge-architect-design`, raised when a change bears on how the checker runs
under a hook, or on the owner's word. It argues the capability and its reason against its rival, a checker that
judges a commit only once it exists, through `commits`. Its outcome is a statement of
`design@git@index-file-variable-kept`, where that head's argument covers it, or a head of its own.
