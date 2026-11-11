---
kind: todo
---
# No reviewer writes its own report file, so the dispatcher saves each report by hand

## Summary

`skill@knowledge-architect-review@what-review-leaves` asks the dispatcher to save each reviewer's
report whole. No installed reviewer agent is told to write its report to a file, so the report
arrives as a message and the dispatcher writes it out, token by token. The tools allow it: the
reviewers' tools are `Read, Grep, Glob, Bash`, the same as the auditor agents', which write their
drafts through the shell to the scratch directory their brief names.

## Details

### What

Instance: the retrospective of the session that reduced the grounding's reading, 2026-10-25:
sixteen reports arrived as messages, and the session saved condensed copies rather than whole ones,
against the instruction, since writing each whole meant restating it.

A further instance: the review before the merge of the intake of seventeen retrospective files of
2026-10-25 to 2026-10-31. Eight reports arrived as messages, each asking to be saved whole; the
dispatcher saved condensed copies of the first seven, and the transcript reviewer reported it.

Ten further sessions, in the retrospective files of the intake of forty files, seven of this
repository and three of thaum, eleven files with the two of the 0.9.0 release, which report one
session; with the two above, twelve sessions since 2026-10-25. Saving one report whole cost 3,000
to 5,000 tokens of the dispatcher's output, by the file of the slice that built the `model` crate.
Five sessions did not follow the instruction: four condensed copies (the transcript-command
issues' review, eight reports condensed; the pre-release fixes, condensed lists labelled as such;
the 0.9.0 release, one report saved as a summary, then whole; a slice of thaum, one-line summaries
of about 30 reports, after which three items under "Met outside the task" had no outcome until the
transcript reviewer found them) and one no save (a slice of the development milestone built
its repair record from the reports in context). The other five followed it by a script over the
harness's files: the task output files, a saved workflow's journal, the agents' output files, or
the subagents' transcripts, 12 reports in 6 runs in one thaum session. The scripts failed twice:
two reports came out empty from the task output files, noticed only after the agents had gone (the
`model` crate); and the output file gave the wrong text twice, found when a later step read it (the
slice that put `cli` on top). No item reached the main branch lost, by each file's account: the
transcript reviewer caught the missing outcomes.

The saved-workflow case: where a saved workflow dispatches the reviewers, no report arrives on its
own, and in one thaum session the completion notice cut the joined results at "truncated 51800
chars", so the reports were read from the workflow's journal. The instruction "as it arrives" has
nothing to act on there.

Facts for the choice of shape: every installed reviewer agent but the adversarial one says "You do
not use `Write` or `Edit`", and none names a report file, while the three auditor agents write
their drafts through the shell with the same tools; so the first shape needs no new tool for an
installed reviewer. A general-purpose axis (spec conformity, self-consistency, fidelity of
relocation) is the case `skill@knowledge-architect-review@review-invariants` says the harness
refuses a write for, a premise not measured at that intake. `klarch transcript --help` lists
`list`, `show` and `search`, and no verb that writes a subagent's last message;
`design@agent-skills@transcript-extraction-is-the-checker-s` puts such an extraction in the
checker's `transcript` command, which supports the second shape. The proposals the files make: a
reviewer writing its report to the scratch directory its brief names and returning it too (the
`model` crate); a `transcript` verb writing a subagent's final report to a file, the save
instruction naming it (two thaum sessions, one also for a searcher's report the harness refused to
write); the save reworded to "before acting on any, from wherever the harness keeps it" (one thaum
session), which the analysis did not take, since the shape this entry's discussion chooses
rewrites the same sentence.

The analysis is W1 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in the
commit "[intake] The analyses of forty received retrospective files, three outcomes for the owner",
cluster D.

### Why it matters

A condensed copy loses what the record of the review needs, and the save follows from the report
with no judgement, per `goal@tasks-a-tool-does-better-are-the-tool-s`.

### What would close it

A shape that saves a report with no restatement: a reviewer that writes its own report file in a
scratch directory the brief names, which the review invariants already give each subagent, while
it keeps from editing the tree; or a command that writes a subagent's last message from the
harness's transcript. Chosen under `skill@knowledge-architect-design`.
