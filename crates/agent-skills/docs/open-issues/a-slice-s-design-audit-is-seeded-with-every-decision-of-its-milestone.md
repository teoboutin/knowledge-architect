---
kind: design
---
# A slice's design audit is seeded with every decision of its milestone, and each forces the entries citing it to `may-bear`

## Summary

The design audit of a milestone's slice seeds the record search with every decision the milestone
document lists, whatever slice is audited. An entry that cites a seed is graded `may-bear` at
least, so the later slices of a long milestone grade many entries `may-bear` only for citing a
decision another slice built. The mechanism is the one `rejected@agent-skills@goal-seeds` records
for goals, and no entry records it for a milestone's other slices.

## Details

### What

The text, `skill@knowledge-architect-planning@working-a-slice`, in
`path@agent-skills@content/skills/planning/SKILL.md`: the slice's spec and the milestone document
are the work, "and, as seeds, the decisions the milestone document lists under "What is already
decided", those the design rests on and those it reverses or rewrites, and the decisions the
slice's spec cites". `klarch worklist` takes seeds by `--seed` alone, one per flag.

The heads that decide it: `design@agent-skills@seeds-name-decisions`, "the design audit of a slice
or a spec with the decisions its plan documents cite", and
`design@agent-skills@record-searched-before-the-work`, "with the slice's spec and the milestone
document as the work and the decisions they cite as seeds". The second's argument against a goal
seed, that a seed many entries cite forces each of them to a `may-bear` grade at least, is the
mechanism seen here for the decisions of other slices.

The instances:

- the last slice of the five-slice development milestone, 2026-11-02: 51 seeds, among them
  the core library's API, the anchors and the mutation crate; one searcher graded 11 entries
  `may-bear` "only because each is a seed or cites one", and `may-bear.md` reached 155 entries,
  which the audit's whole reading had to cover
  (`issue@agent-skills@the-search-s-graded-entries-exceed-what-one-session-reads-whole`). Its
  workflow file proposes seeding a slice's audit with the decisions the slice's spec cites and
  those the milestone lists for that slice, 0 instructions added and one changed;
- the slice of the integrated tests, 2026-11-07: 54 seeds extracted by `grep` from the milestone's
  "What is already decided" and the slice spec, `design` and `rejected` entries both, each passed as
  `--seed`. Its file proposes `worklist --seeds-from <plan document>`; the seed list of each audit
  of the development milestone was extracted the same way, with `awk` and `grep`, and
  resolved with `show`;
- a slice of thaum, 2026-11-02: about 110 of 125 `may-bear` entries graded so for citing a seed,
  with no element of the work meeting them.

The rivals: keep the whole list, the heads' choice, at the cost of the `may-bear` entries above;
fold this into `issue@agent-skills@the-search-s-graded-entries-exceed-what-one-session-reads-whole`,
since the seeds feed the size, which loses because the question differs, what is seeded rather than
how it is read, under another head. `--seeds-from` follows the seed rule, whichever it is, so it is
the rule's mechanical half; it must take decisions alone, never a goal a plan document cites, or it
brings back `rejected@agent-skills@goal-seeds`. Which references count as decisions, design entries
only or `rejected` entries too, is part of the rule.

The suspected mechanism, in one sentence: a milestone's list of decisions grows with each slice,
and each decision as a seed lifts every entry that cites it to `may-bear`, whether or not the
audited slice touches it.

Re-entry point: a design discussion under `skill@knowledge-architect-design`, on the owner's word
or when work next changes how a slice's audit is seeded; then the planning text, and the option in
`query` and `cli`.

Source: W2 of the analysis 2026-11-02-knowledge-architect-development-and-components-klarch-workflow,
with C4 of 2026-11-07-knowledge-architect-integrated-tests-klarch-workflow, in the commit "[intake]
The analyses of forty received retrospective files, three outcomes for the owner", cluster A2.

### Why it matters

Entries graded for a seed alone enlarge the reading the audit owes with entries that bear on
nothing in the slice, against `goal@knowledge-architect@agents-work-without-drift`, which the
search serves; the scripted extraction of seeds is a mechanical task with no command, against
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`. It strains
`design@agent-skills@seeds-name-decisions` and `design@agent-skills@record-searched-before-the-work`.

### What would close it

A discussion under `skill@knowledge-architect-design` that keeps the milestone's whole list as
seeds, with the cost measured above, or narrows a slice's seeds to the decisions that slice rests
on, rewriting the two heads; and, for either rule, a `worklist` option that reads the seeds from
the plan documents, decisions only.
