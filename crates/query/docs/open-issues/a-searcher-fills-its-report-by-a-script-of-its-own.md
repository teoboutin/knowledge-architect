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

The report's line shape was then `- <reference> | grade: | relation: | reason:`, with a seed field
between the reference and the grade where the entry is or cites a seed. It is now
`- <reference> | verdict: | against: | action: | owner-question: | quote: | reason:`, after the
same seed field, per `design@query@worklist-writes-and-verifies-the-reports`, and still filled line
by line.

A further instance: the search at the design audit of the spec revising the rules for splitting a
project into Components, 19 agents. One wrote a script of its own, `grades.py`, and said so in its
report. Nine said that the harness refused an edit of their report; eight of them returned the filled
report as text, which the dispatching session wrote into the files by a script of its own.

Another: the search at the design audit of the slice that created the git Component, 19 agents.
Read in their reports: at least twelve rewrote `report.md` whole from a heredoc, and one of them said in
its report that this was a further instance of this entry; the others filled it by the edit tool.
The check `worklist --verify` passed on the second run, after two agents repaired a line each.

Further instances, from retrospective files received at the intake of 2026-11-07:

- the session that planned the development milestone, 2026-11-01: the searchers filled their
  reports by scripts of their own;
- the slice that created the `model` Component, 2026-11-04: most searchers wrote a script;
- the slice that put the `cli` Component on top, 2026-11-06: several of the 22 searchers said they
  wrote a fill script;
- a search in thaum, a project that installs the workflow, 2026-11-02: most of 14 searchers filled
  their report by a heredoc script from a list of grades they had decided first, and one script
  graded `unrelated` an entry its list had left out, which `--verify` then refused;
- another search in thaum, 2026-11-02: two searchers whose edit of their `report.md` the harness
  refused returned the report as text, and the session extracted the fenced report by a script
  before `--verify`.

The harness's edit tool replaces one exact string per call, so filling a report of 90 lines by it
costs 90 calls. Not established: whether another harness offers a cheaper edit.

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
