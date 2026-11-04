---
kind: defect
---
# Under `check --staged` a finding on an installed file can name a repair that does nothing

## Summary

The findings of the family `agents` name the install as their repair, and the install reads the working tree. Over
the index or a symlink the repair can do nothing, and one tree can get two verdicts.

## Details

### What

Reproduced by an adversarial reviewer:

- A stale copy in the index and a current working copy: `check --staged` says to run the install, which says the
  installed set is already the shipped one, and the verdict stays failed. The repair is `git add`.
- A deletion not staged, under `check --staged`: it says to run the install, which already removed the file.
- With `skip-dirs = [".claude"]`, an installed file replaced by a committed symlink to a copy holding the shipped
  bytes: `check` passes, `check --staged` fails, and the install refuses the link. A dangling link at an unshipped
  path is reported as a deletion awaiting staging.

Neighbour: `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, another case of the same
comparison.

### Why it matters

`design@check@finding-names-the-repair` asks a finding to name the repair its cause needs, and
`design@check@staged-check-semantics` asks `check --staged` to judge as a clean checkout would.

### What would close it

Each case above named with the repair it needs, and a symlink at an owned path judged alike by both readers, each
with a binary test.
