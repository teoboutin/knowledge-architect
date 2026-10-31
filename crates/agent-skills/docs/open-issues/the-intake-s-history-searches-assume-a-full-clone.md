---
kind: question
---
# The retrospective intake's searches of the history assume a full clone

## Summary

Check b of `skill@knowledge-architect-retrospective-intake@what-to-establish` dates a file whose
version line names no commit with `git log -S '<phrase>'`, finds a repairing commit the same way,
and reads a version by its `git describe` count. In a shallow clone, `git log -S` names the clone's
boundary commit where the commit sought is absent, with no error. Does the intake need the shallow
clone's rule that the horizon-counting agents carry?

## Details

### What

The two agents that count a tripwire's horizon host
`path@agent-skills@fragments/shallow-history.md`: in a shallow clone they report each horizon they
could not count. The intake's check b, in `path@agent-skills@content/skills/retrospective-intake/SKILL.md`,
runs the same searches and holds no such rule.

Measured during the intake of seventeen retrospective files of 2026-10-25 to 2026-10-31, by the
self-consistency reviewer of its branch: in a clone of depth 3, `git log -S` on a `Watches until`
line names the clone's boundary commit, where the full history names the two commits that wrote the
lines. A checking subagent of the same intake named check b as a further site of the full-history
assumption.

Not established: that an intake runs in a shallow clone. The intakes seen ran on the owner's
machine, in a full clone. `git describe` fails with an error in a shallow clone that holds no tag,
as the retrospective of 2026-10-28 reports for the version line, so only the `git log -S` searches
fail silently.

The fragment's wording does not fit the intake as it stands: it tells an agent that cannot write
to report, and the intake is a session that can run `git fetch --unshallow` itself.

### Why it matters

An intake that dates a finding from the boundary commit reads the blamed text at the wrong version,
and may judge a finding repaired or still standing wrongly, against
`goal@agent-skills@installed-text-works-anywhere`.

### What would close it

A retrospective file that reports an intake run in a shallow clone, or a session that met the
silent result, and then a sentence in check b with its own wording, through
`skill@knowledge-architect-agent-configuration`. An occasion with no shallow intake closes nothing:
the intakes seen run where no clone is shallow, so their count is no evidence that none will.
