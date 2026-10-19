---
kind: todo
---
# A searcher's report is written by hand, and nothing checks that it covers its group

## Summary

The searchers a design discussion dispatches return a report that must list every entry of their
group exactly once, and each agent writes that list by hand from its own reading. No tool gives the
agent a skeleton holding the group's entries to fill, and no tool checks a filled report against
the group. The owner wants such tooling, for every searcher: the standing-entry searcher, and the
design-entry searcher, `agent@knowledge-architect-design-entry-searcher`.

## Details

### What

- `agent@knowledge-architect-standing-entry-searcher` reads one group of consecutive positions of
  the combined rows of `cargo klarch issues` and `cargo klarch tripwires`, and its section
  `agent@knowledge-architect-standing-entry-searcher@what-to-return` requires "Every entry of the
  group appears in exactly one of the two lists, and the second list is complete." The agent derives
  its group itself, by the sort its section
  `agent@knowledge-architect-standing-entry-searcher@take-your-group` states, and writes both lists
  by hand.
- The design-entry searcher, per `design@agent-skills@design-entry-search`, reads a group of a Component's design
  entries and `rejected` entries, and returns three lists, `Bears on the work`, `May bear` and `Read
  and judged unrelated`, under the same rule: every entry of the group in exactly one list. The
  owner ruled, in that discussion, that every unrelated entry is listed rather than counted.
- Nothing mechanical produces the group's entries for the agent, and nothing reads a report back.
  The session that dispatched the searchers checks coverage by reading, or not at all.
- An instance, in the design discussion that produced the `rejected` register: of three standing-entry
  searchers over 141 entries, the one on positions 1 to 47 listed
  `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry` under "Bears on the
  work" and again at the end of "Read and judged unrelated", then appended a correction to its own
  report. The same three reports gave 12 of their 41 returned entries an ad-hoc doubt grade in their
  reason, each worded differently.

The owner's direction: later, a mechanical tool that verifies a searcher's report covered the whole
of what it was given, and possibly a report skeleton the agent only fills.

### Why it matters

`design@agent-skills@standing-entry-search-agent` rests on the return listing every entry read, so
that the dispatching session can see what was judged unrelated. A report that drops an entry, or
lists one twice, reads the same as a complete one, and the entry dropped is the one a design session
then misses: the failure `design@agent-skills@design-entry-search` exists to reduce. The rule "exactly one
list" is an instruction no check enforces, against `goal@core@declared-instructions-are-checked`.

### What would close it

A design, under `skill@knowledge-architect-design`, of tooling for both searchers, then the work it
decides. The leads, from the owner's direction:

- a command that prints a group's entries as a skeleton of the report, each entry a line the agent
  moves into one list and gives a reason, so the agent no longer derives the group or its order;
- a command that reads a filled report and reports an entry of the group that appears in no list
  or in two, and a reference that is no entry of the group, with an exit code a session can act on;
- for the design-entry searcher, the listing of a Component's design entries and `rejected`
  entries, which no command prints today: `cargo klarch model` prints every heading and slug
  definition, not entries.

Not established: whether the tool belongs to the checker's commands or to the installed agents'
text, and whether a skeleton changes how an agent judges an entry.
