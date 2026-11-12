---
kind: deferred
---
# A dispatched agent writes its report once, at the end, where a file written as it works and annotated by its verifier could fill the gaps

## Summary

A dispatched agent's report is written in one pass, in its last messages, and the session that
dispatched it acts on that report after an independent verification, per
`design@agent-skills@at-least-one-independent-verification`. The owner proposed report files that
agents write and that a verifier can edit, which might reduce that gap; the session refined it
into a file written as the agent works and annotated by the verifier finding by finding, rather
than rewritten. The proposal waits for a real session whose returned
report could not be acted on without redoing the work.

## Details

### What

The owner's report: the weak point of dispatched work is less the judgement of the agent or of its
verifier than the completeness of the report, since it is often written in one pass at the end; a
session that verifies the report itself has a good chance to fill a gap in it, and a report file
an agent writes, which a verifier can edit, might narrow that gap. The owner has not read such
reports and cannot say for sure. The shape the session refined from it:

- the agent writes its report file as it works, not once at the end;
- a verifier annotates each finding in that file, rather than rewriting the report;
- the verifier's brief carries the original brief, so it can see what was asked and is not
  answered.

What exists: a reviewer already writes its report whole to a path its brief names, through the
shell, per `design@agent-skills@reviewer-writes-its-report` and
`design@agent-skills@report-written-through-the-shell`, since the harness can refuse a subagent's
`Write` of a file named `report.md`; an audit's report step returns its report as text only where
the harness refuses that write, per `design@agent-skills@refused-report-returned-as-text`. Any
report file written as the agent works meets the same refusal, so it is written through the shell
or with `Edit`.

How it differs from the two entries on reports:

- `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript` is about where
  a report reaches, the script of a workflow rather than the session's transcript; this entry is
  about when the report is written and who completes it.
- `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own` is about how a searcher fills
  the lines `klarch worklist --out` wrote for it, a mechanical task; this entry is about a report
  whose content the agent decides.

The shape does not bring back `rejected@agent-skills@ledger-written-during-discussion`: that entry
refused a file of the whole discussion's state, which an agent must update every round and whose
only corroborant is the conversation. A report file here is written by one agent for its own piece,
and the verifier's annotation corroborates it from outside the writing agent. That is less than the
entry's survey found in the agent systems it read, a state corroborated against something outside
the model, such as git: a verifier is another instance of the model.

### Why it matters

A report that omits what its agent established makes the session redo the work, or act on a
conclusion whose evidence it does not hold, against `goal@session-orchestrates-the-work`, under
which the session hands the work to agents and keeps its own context for the design. No real session
has shown such a report yet, so an instruction now would predict a behaviour, which
`design@agent-skills@behaviour-additions-need-real-use` refuses.

### Trigger

A retrospective's answer to its standing question on a dispatched agent that returned with a report
the session could not act on without redoing the work, naming such a report: the intake of that
answer is the occasion that designs the report file, from the brief and the report it names.
