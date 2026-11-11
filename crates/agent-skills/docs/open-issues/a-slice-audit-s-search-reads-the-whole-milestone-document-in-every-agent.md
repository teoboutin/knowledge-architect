---
kind: defect
---
# A slice audit's record search gives every agent the whole milestone document, whose discussion record multiplies the agents

## Summary

The design audit of a slice runs the record search with `--work` the slice's spec and the
milestone document, per `skill@knowledge-architect-planning@working-a-slice`. The milestone
document holds every argument of its discussion verbatim, so its words fill most of each agent's
bound, and the search needs about three times the agents its entries alone would.

## Details

### What

At the design audit of the second slice of the milestone that moved the goals to the project's
root, the work held
26,222 words by `wc -w`: the milestone document 24,977 and the slice's spec 1,245. `cargo klarch
worklist` counts the work against each agent's bound of 40,000 words, so it left 13,778 words of
entries per agent, and the record went to 44 agents. The entries given, context included, were
577,404 words by `wc -w` over every agent's `entries.md`, for a record of 255,227 words over the
tracked register homes. With the slice's spec alone as the work, about 38,700 words per agent would
have held the same entries in about 15 agents.

Of the milestone document, the section Arguments held 11,767 words, What is already decided 5,100
and Threads 2,315, by `wc -w` over each section. The arguments are the record of the discussion,
which `skill@knowledge-architect-planning@spec-contents` has the milestone document keep, every
item more than one slice cites living there. So the document grows with its discussion, not with
the slice under audit, and every agent of every slice's audit reads all of it. Seven of the first
27 reports said the agent could not read the document whole, and read it in parts and by search.

### Why it matters

The search's cost is paid per agent, and the reading of each agent is the search's evidence. A work
that strains each agent's reading weakens every verdict, and multiplies the agents that pay for it.
It strains `design@agent-skills@search-reads-the-whole-record` and
`design@agent-skills@milestone-split-by-lifetime`, whose split keeps the milestone document small
only for what slices build, not for what the discussion recorded.

### What would close it

The planning skill gives a slice audit's search a work that holds what bears on the slice: the
slice's spec, and of the milestone document the sections a searcher grades against, without the
record of the discussion; or the planning skill keeps that record out of the milestone document,
where `rejected@agent-skills@milestone-holds-every-slice-design` measured a document read whole at
every slice's grounding as the cost to avoid.
