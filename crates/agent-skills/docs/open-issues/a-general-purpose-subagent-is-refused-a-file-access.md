---
kind: observation
---
# A general-purpose subagent is refused the read of a transcript, where an installed agent is not

## Summary

A general-purpose reviewer was refused the read of the session's transcript by the harness's
permission classifier, while `agent@knowledge-architect-transcript-reviewer` read the same file.
The same split for the write of a report file is reproduced and answered: the review skill states
that a general-purpose subagent returns its report as text, and the dispatcher saves it.

## Details

### What

The refusals were reported by the spec-conformity, self-consistency and fidelity-of-relocation
reviewers, and by the helpers the last of them dispatched. The routing, decision-record and
standing-state reviewers, each dispatched as its installed agent, wrote their report files, and so
did the general-purpose agents that applied the audit's repairs earlier in the same session. So the
refusal is not established as a property of every subagent: what decides it, the agent type, the
wording of the brief or the harness version, is not established. Not reproduced on purpose.

The same split recurred at the review before the merge of the branch that gave the design-record
axis its saved workflow. The spec-conformity, self-consistency and fidelity-of-relocation
reviewers, dispatched as general-purpose subagents, each reported the refusal. The routing,
decision-record and standing-state reviewers, each dispatched as its installed agent, wrote their
report files. In the same session, the general-purpose agents of that workflow's run wrote their
files, and so did the general-purpose agents that applied the pass. Two observations now pair the
refusal with a general-purpose reviewer, and none with an installed agent.

A third occurrence came from another project. The thaum project's retrospective of a roadmap
revision, on knowledge-architect 0.5.0, reports one general-purpose subagent refused a write to its
own scratch directory with "Subagents should return findings as text, not write report files",
returning its full table as text instead, while other subagents of the same session wrote to their
scratch directories. Three observations now pair the refusal with a general-purpose subagent, and
none with an installed agent. In the session that took that report in, three general-purpose
subagents wrote working files in their scratch directories without refusal; none was briefed to
write a report file, so the closing test below was not run.

A fourth occurrence, a read, in this repository's session that shipped the retrospective intake as
an installed skill: the fidelity-of-relocation reviewer of the spec, dispatched as a general-purpose
subagent, had its read of the session's transcript refused by the harness's permission classifier,
as handling personal data, while `agent@knowledge-architect-transcript-reviewer` read the same file.
The fidelity axis then judged its quotations against the text they were moved into only, and said
so. Reported as W3 of that session's retrospective workflow file. Four observations now pair a
refused file access with a general-purpose subagent, and none with an installed agent.

A fifth occurrence, the closing test of the write half: at the retrospective intake of
2026-10-24, seven general-purpose subagents, each briefed to write a report file in a scratch
directory of its own, were each refused with "Subagents should return findings as text, not write
report files", in the harness version current then. `skill@knowledge-architect-review@review-invariants`
now states that a general-purpose subagent returns its report as text, and every brief asks the
reviewer to end its report with a line asking the dispatcher to save it whole.

### Why it matters

`skill@knowledge-architect-review@review-invariants` has the dispatcher name a scratch directory for
each subagent, and the project-audit skill's drafts and reports live in files the session can open,
per `design@agent-skills@one-run-pass`, each agent writing in a scratch directory of its own, per
`design@agent-skills@audit-method`. A reviewer whose report comes back only as a message leaves the
session no file to read again, and a long report can be cut short in the message.
The dispatcher now saves each report in a file as it arrives, per
`skill@knowledge-architect-review@what-review-leaves`, so a refused write loses no report. A refused
read still leaves an axis judging without its source, as the fidelity axis above did.

### What would close it

One dispatch of a general-purpose subagent briefed to read a transcript file, in the harness
version then current: refused, the review skill names the agent types an axis that reads a
transcript is dispatched as; read, the entry closes as not reproduced.
