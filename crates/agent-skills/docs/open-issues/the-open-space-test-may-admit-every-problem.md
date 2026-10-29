---
kind: question
---
# Does the design skill's open-space test leave bounded work almost unreachable?

## Summary

A review of designing-together, the plugin the design skill came from, predicted that "a second
defensible shape is nameable for nearly any request". If so, the third condition of the bounded
path, that no second defensible shape survives the nearest-rival test, would classify almost every
request as open, and the bounded path would almost never be taken.

## Details

### What

The prediction was recorded as a lead of the issue that asked for a skill for bounded problems,
with "No session has shown it". That issue closed when the bounded path was scheduled inside the
design skill, and said that the lead, untested by then, is carried to an issue of its own about the
design skill. The bounded path, `design@agent-skills@bounded-path-in-design`, is where it would show:
a session that grounds a requested change and names a second shape for work the owner sees as
bounded.

A count over the commit messages of two main branches, taken on 2026-10-29: in rootpoint, two
commits say their proposal was "put to the owner as bounded work", 8bdeced and d20eac7, both of
2026-10-26; in this repository, none does. The command is
`git log main --oneline -i --grep='as bounded work'` in each. In the same branches, 60 commits here
and 7 in rootpoint name the in-change path, by `git log main --oneline -i --grep='in-change path'`.
A zero does not tell "bounded work never occurred" from "a session built bounded work without
loading the design skill": the cloud sessions of this repository's pull requests 24 and 25 built
the owner's requests without loading it, as the spec of the branch that reworded the design
skill's trigger records.

### Why it matters

`design@agent-skills@bounded-path-in-design` exists so that bounded work costs one proposal, built on
its default. A classification that almost never finds work bounded sends it to a discussion each
time, the cost the path was decided to remove. It is the opposite failure to
`tripwire@agent-skills@a-shortcut-decision-earns-a-head`, which watches work wrongly classified as
bounded.

### What would close it

Real sessions after the bounded path lands: a count of requested changes the design skill grounded,
and how many it classified bounded, read at the retrospectives. A session where the owner says a
request was bounded and the agent opened a discussion on a second shape the owner judges
indefensible answers it yes; a run of bounded classifications the owner accepts answers it no.
