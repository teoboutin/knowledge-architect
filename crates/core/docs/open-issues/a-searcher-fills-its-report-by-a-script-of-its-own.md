---
kind: todo
---
# A record searcher fills its report by a script or a whole rewrite of its own

## Summary

`klarch worklist --out` writes each searcher a `report.md` with one line per entry, each to be
filled with a grade, a relation and a reason. No searcher of one search edited the lines one by
one: each wrote a script, or rewrote the whole file from a heredoc. Filling the report is a
mechanical task the installed workflow asks of every searcher, and no command does it.

## Details

### What

The instance: the search of the whole record at the grounding of the design discussion that wrote
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, ten agents, each given 76 to 96 entries
to grade. Read in the reports the agents returned:

- five agents wrote a script that reads a table of grades they wrote first, a heredoc of tab-separated
  lines or a Python dictionary, and writes each grade into its line of `report.md`;
- five agents rewrote `report.md` whole from a heredoc holding every line;
- two of the five scripts failed on their first run and were repaired: one stopped on an entry its
  table lacked and wrote nothing, and one matched only the lines that carry a seed mark, so it left
  the other lines blank until a second run.

The report's line shape is `- <reference> | grade: | relation: | reason:`, with a seed field
between the reference and the grade where the entry is or cites a seed, per
`design@core@worklist-writes-and-verifies-the-reports`. The harness's edit tool replaces one exact
string per call, so filling a report of 90 lines by it costs 90 calls. Not established: whether
another harness offers a cheaper edit.

### Why it matters

Each searcher builds the same tool again, and a script that misses a line leaves it blank, which
`worklist --verify` reports, at the cost of another dispatch. It is a task of the kind
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` gives the published checker.

### What would close it

A form in which a searcher records its grades without writing a script, read by
`worklist --verify` as the reports are today, with its shape argued under
`skill@knowledge-architect-design`: for instance a file of one line per graded entry that
`--verify` reads in place of the filled skeleton. Or the owner's ruling that the filling stays by
hand, recorded in the commit that deletes this entry.
