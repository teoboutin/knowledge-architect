---
kind: defect
---
# Commit bc22d4d, cited as the corpus of a measurement in six places, names no object in this repository

## Summary

Six places of the agent-skills design record cite commit `bc22d4d` as the state at which a
measurement was taken, an inventory of the installed text or an audit of the design heads, and the
figure is re-taken at that commit. `git cat-file -t bc22d4d` fails: no object of this repository starts with
those digits, so no reader can re-take the measurement.

## Details

### What

The citations, found by `grep -rn bc22d4d crates/agent-skills/docs`:
`rejected@agent-skills@head-cites-every-owner-ruling` and two other entries of
`register@agent-skills@rejected`, two heads of `path@agent-skills@docs/design/design-skill-and-heads.md`,
and a head of `path@agent-skills@docs/design/project-configuration.md`. The commit likely predates a
rewrite of history, such as a rebase merge that gave the branch's commits new SHAs; that is an
assumption, not established. `git log --all --format=%h` lists 1029 commits, none starting with
`bc22d4`.

### Why it matters

A measurement whose corpus cannot be found cannot be re-taken, against
`goal@knowledge-architect@design-is-recorded-with-its-arguments`, and each head that rests on the
figure cites a premise no later session can check.

### What would close it

Each citation names the corpus by a commit that exists on the main branch, found by the commit's
subject or by the date and the work it describes, or by the subject of the commit that took the
measurement, and the figure is labelled as unreproducible where no such commit is found.
