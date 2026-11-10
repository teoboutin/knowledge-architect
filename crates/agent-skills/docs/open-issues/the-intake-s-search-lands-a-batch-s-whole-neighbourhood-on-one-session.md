---
kind: design
---
# A retrospective intake handles many unrelated findings at once, so its grounding may still load one session beyond one reading

## Summary

A retrospective intake receives a batch of files, each holding findings on unrelated subjects, and
grounds them in one search of the whole record. The owner doubts that one verdict file per cluster
of findings, which `spec@plans@record-search-verdicts` builds, is enough, and believes the intake
may need a different procedure with more steps to partition its work. The doubt waits for the next
intake the owner runs.

## Details

### What

The instances, measured before the verdict shape, as the analyses of the commit "[intake] The
analyses of forty received retrospective files, three outcomes for the owner" record them: the
intake of seventeen files graded 200 entries `bears` and 164 `may-bear`; the intake of forty files 333
and 249, which nine checking subagents read in parts, as
`rejected@agent-skills@relevance-grades-read-by-the-session` also states; and an intake of thirteen
files, whose session read 64 standing entries whole, about 167 KB, as the head of the record
searcher stated before the verdict shape.

The owner's statement, in the design discussion of the record search: the search suits the design
skill better than the design audit and the intake; the intake may have a deeper problem, since its
nature has it handle many unrelated small problems, so it often ends up covering the whole design
record; one verdict file per subject of the work may not fully solve it, and a significantly
different procedure, with more steps to partition the work, might be needed; the scope of that
discussion stayed limited, to see how the situation improves at the next intake.

The suspected mechanism, in one sentence: an intake's work is the union of its findings'
neighbourhoods, so whatever the search's shape, everything the batch touches reaches the one session
that holds the batch.

Re-entry point: the retrospective of the first intake the owner runs after the work of
`spec@plans@record-search-verdicts` lands, or the owner's word.

### Why it matters

An intake whose session cannot carry its grounding judges outcomes on less than the record, against
`goal@knowledge-architect@agents-work-without-drift`, and spends the main session's context on reading
that `goal@agent-skills@session-orchestrates-the-work` hands to dispatched agents.

### What would close it

A design discussion under `skill@knowledge-architect-design`, at the re-entry point, that keeps the
intake's procedure with the measured load of that intake, or changes how the intake partitions its
work among agents.
