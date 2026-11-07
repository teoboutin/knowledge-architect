---
kind: todo
---
# A searcher's report skeleton names no relation its entry's role admits, and `--verify` names them only on a refusal

## Summary

`klarch worklist --out` writes each record searcher a `report.md` of one line per entry, to be
filled with a grade, a relation and a reason. The line names no relation, though the relations a
`bears` grade admits differ by the entry's role. Searchers write a relation the role does not
admit, `worklist --verify` refuses the line and only then names the admitted ones, and each such
searcher is sent its lines again.

## Details

### What

The skeleton line, written by `path@query@src/worklist.rs`, is
`- <reference> | grade: | relation: | reason:`, with a seed field between the reference and the
grade where the entry is or cites a seed, under the header "Grade each entry: `bears` with a
relation, `may-bear`, or `unrelated`." It names no relation.

The relations per role are in `agent@knowledge-architect-record-searcher@grade-each-entry`, a
table the searcher reads once before filling: `constrains` and `may change` for a goal, a design
entry or a declared register's entry, `may bring back` for a `rejected` entry, `touches`, `closes`,
`worsens`, `depends on` and `trigger met` for an issue, `fires` for a tripwire, and none for
`may-bear` and `unrelated`. `--verify`, in `judge` of the same file, refuses a line whose relation
the role does not admit with "`<relation>` is no relation for the role `<role>`: give one of" and
the list.

Instances, from retrospective files:

- the intake of seventeen retrospective files, 2026-10-31: 4 of 12 searchers wrote a relation
  their entry's role does not admit, on 10 lines; each was sent its lines again;
- the search at the grounding of the design discussion that revised the rules on Components,
  2026-11-03: 7 of 19 searchers; `--verify` refused 113 lines, 72 a `may-bear` line with a
  relation, 38 a relation in free text, 3 a relation on an issue. The session normalised the
  relation fields by a script of its own, keeping each grade, instead of sending the agents again;
- a search in thaum, a project that installs the workflow, 2026-11-02: 2 of 14 searchers gave a
  `rejected` entry the relation `may change`, which its role does not admit.

Counted over these three searches: 13 of 45 searchers wrote a refused relation.

Ruled out, at the intake of 2026-11-07:

- the dispatcher repairing a report refused on format alone, the rival a file proposed: a relation
  routes an act, the reversal check or a reopening, so choosing one is grading, which
  `design@agent-skills@one-record-searcher` gives the searcher; only dropping a relation from a
  `may-bear` line is mechanical;
- a further relation, `may change`, for a `rejected` entry the work edits, proposed by a
  retrospective of a session of thaum: the audit and the intake read such an entry whole either
  way, so the relation would route no act, which `design@agent-skills@search-relevance-grade`
  requires of a relation.

### Why it matters

The table is read before the filling and needed at each line, so a searcher grading its entries
applies it from memory, and each refused line costs another dispatch. Naming the admitted
relations on each line follows from the entry's role with no judgement, a task
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` gives the published checker.
`design@query@worklist-writes-and-verifies-the-reports` describes the skeleton the change rewrites.

### What would close it

Each skeleton line names the relations its entry's role admits, for instance
`relation (constrains, may change):`, with a test over a mock project that the line of each role
names its own list, and `--verify` still refusing a relation outside it.
`issue@query@a-searcher-fills-its-report-by-a-script-of-its-own` closes apart, on a form that needs
no script; this hint is owed whatever that form is.
