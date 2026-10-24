---
kind: todo
---
# No check reports a lens of the agentic-workflow axis whose second agent found nothing the first did not

## Summary

`design@agent-skills@workflow-axis-lenses` dispatches some lenses twice by default, on the premise
that a second agent of one lens finds what the others missed. Which agent reported each confirmed
finding is in a pass's files, and no step reads it. A check after the dedup step of the axis's saved
workflow would report a lens whose agents beyond one added nothing.

## Details

### What

The premise is watched by `tripwire@agent-skills@duplicate-auditors-add-nothing`, which fires when, in one pass, for every lens dispatched more
than once, one of its agents alone reported every confirmed finding of that lens; two L2 agents
count as one lens dispatched twice only when they share a group of activities. The first pass of
the tripwire axis on this repository converted it to a check, on the tripwire standard's "Where a
script could compute the firing, write that check instead", by the owner's ruling on its owner
list; the entry stays until the check exists, its horizon the commit that closes this issue.

The missing work: in `path@agent-skills@content/workflows/agentic-workflow-audit.js`, after the
dedup step, a script groups the confirmed findings of each lens with more than one agent by the
agents whose drafts they merge, and the report's header gains a line naming each lens where one
agent holds them all. The confirmed-findings file the dedup step writes gives each finding one line
naming the draft kept, as `<auditor label>/<draft file name>`, and the ids merged into it, in a
line an agent writes with no format a script can rely on; the check needs that shape fixed. The
auditor label carries the lens and the agent, as `L2-d/draft-3-audit-after-the-pins-commit-two-readings.md`,
cited by the report of the sixth pass of the first agentic-workflow audit, shows.

The last evidence on the premise, recorded in the head: on the fifth pass of the axis on this
repository, each lens with more than one agent produced findings no other agent of that lens
reported. One pass of the axis, the sixth, has run since.

### Why it matters

Each duplicated lens costs a second agent's reading of the whole corpus. Until the check exists, a
pass where the second agents add nothing is seen only by the session's own count, which
`design@agent-skills@workflow-axis-lenses` rests on.

### What would close it

The confirmed-findings file given a shape that names each finding's drafts, and the line of the
report's header that the script above writes from it; the commit that writes it deletes
`tripwire@agent-skills@duplicate-auditors-add-nothing`.
