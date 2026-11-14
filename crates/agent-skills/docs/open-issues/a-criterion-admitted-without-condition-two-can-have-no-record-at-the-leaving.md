---
kind: design
---
# An acceptance criterion admitted without condition 2 can have its evidence in no record at the leaving

## Summary

An acceptance criterion is admitted on conditions 1, 3 and 4 of the tripwire standard, on the
premise that the landing's work reports on it, and is judged by all four at its plan's leaving. One
milestone's criterion watched the input files of a design audit's search, which no record the
workflow requires names, so at the leaving it failed condition 2 and was dropped. Each fix widens or
reverses a head.

## Details

### What

- `skill@knowledge-architect-planning@acceptance-criteria`: "An acceptance criterion is admitted
  only where it meets conditions 1, 3 and 4 of the standard below"; at the plan's leaving, a
  criterion that recurs is judged by all four.
- `design@agent-skills@criteria-test-silent-premises` rests on "Condition 2 does not apply: the
  landing's own work reports on a criterion".
- A design audit's record is its fit lines, per `design@agent-skills@design-audit-checks-fit` and
  `design@query@occasion-shaped-output`; `worklist --verify` skips every `work` line of `work.txt`
  when it writes `fit-lines.md`, in `path@query@src/worklist.rs`, so the fit lines name the plan
  document and not its work.
- Point 2 of working a slice fixes the audit's `--work`, "the milestone's `README.md` and the
  slice's `README.md`, and no other file", so a correct audit's work follows from the instruction,
  but a wrong one is in no record.

The instance: AC1 of the milestone on split guidelines and plan directories, guarding
`design@agent-skills@discussion-record-apart`, which fired on the `--work` files of a design
audit's search. The audits named their `--work` in their commit messages by the session's practice
alone. The commit "[plans] The milestone on split guidelines and plan directories leaves: its last
slice landed, and every default is ruled" kept it as a tripwire on a doubt, and the commit
"[review] Repairs of the standing-state read of the milestone's deletion: AC1 dropped as a cause,
and three issue entries state the present" dropped it: "No record a design audit owes names its
`--work`, so condition 2 [...] fails outright". Reported by W3 and C1 of the workflow file of the
slice that gave each Component's head its arguments, 2026-11-14, and W3 of its project file.

The suspected mechanism, in one sentence: a criterion is admitted on the premise that the landing's
work reports on it, and the work's records hold no observable for it.

The shapes:

- the fit lines name the work files, C1's proposal, preferred by the file; it widens
  `design@agent-skills@design-audit-checks-fit` and `design@query@occasion-shaped-output`;
- the design audit names its `--work` in its commit message, one instruction added; it changes what
  an audit's commit holds;
- condition 2 applied at admission to a criterion whose observable no record holds, which reverses
  the premise of `design@agent-skills@criteria-test-silent-premises`;
- no change, since the leaving's judgement dropped the criterion as the standard says.

The first two keep the guard of `design@agent-skills@discussion-record-apart`; the last two leave
that head with no guard of a wrong `--work`. `design@agent-skills@acceptance-criteria-in-the-document`
keeps the judgement at the leaving under every shape.

Re-entry point: the next acceptance criterion whose observable is an input of a command that no
committed record names, at its admission; the discussion under `skill@knowledge-architect-design`.

### Why it matters

It strains `design@agent-skills@criteria-test-silent-premises`: a criterion the planning admits can
be unjudgeable at the leaving, so the decision it guards loses its guard, as
`design@agent-skills@discussion-record-apart` did.

### What would close it

The discussion picks a shape and rewrites the heads it widens or reverses, with a test of
`worklist --verify` where the fit lines change; or it keeps the standard with this case stated in
`design@agent-skills@criteria-test-silent-premises`.
