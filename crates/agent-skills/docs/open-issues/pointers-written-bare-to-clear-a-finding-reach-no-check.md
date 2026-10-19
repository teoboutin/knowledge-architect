---
kind: design
---
# Pointers written as bare plain text to clear a finding reach no check, and the rule against them is reopened

## Summary

`design@agent-skills@plain-text-is-no-repair` refuses plain text as the repair of a finding. Its
tripwire fired at its second instance, so the decision is reopened, with a mechanical check for
plain-text pointers among the candidates. Three instances are known; none of the three was shaped
like a reference, so the class A finding that `spec@plans@defined-ids-outside-references` plans
would have reported none of them.

## Details

### What

The instances:

- The first pass of the design-record axis on this repository: its report file had its references
  to the heads the pass removed turned into plain text in the checker's syntax, in the commit
  "[audit] Pass 1 of the design-record axis applied: its edits in the branch, F1 to F18 and F21
  carried out, two issues opened". The form the decision admits for such a file is the bare slug in
  backticks.
- A thaum session on knowledge-architect 0.7.0: a commit message named a directory its plan will
  create; backticked, `commits` reported it as a path naming no anchor, and the session wrote it
  bare, with no issue beside it. The checked form existed, `path@elsewhere@<path>`, which the
  finding's repair named "for a path outside this tree"; the repair line now says "for a path this
  tree does not hold", as `primer@where-knowledge-goes` does. Standing answer and C1 of that
  session's retrospective workflow file.
- This repository's session that shipped the retrospective intake: a spec and several commit
  messages named the skill the work would create in plain text, beside no issue, since a reference
  to a skill that does not exist is a finding. On the owner's word, a record of a deliberation now
  names a skill or an agent its work removes or will create by its bare name in backticks, as it
  names such an entry. C1 of that session's retrospective workflow file.

The suspected mechanism, in one sentence: a writer who meets a finding with no form it recognises
removes the backticks, and nothing reads text outside backticks unless it is reference-shaped.

Re-entry point: the harvest of `spec@plans@defined-ids-outside-references`, which reads
`design@agent-skills@plain-text-is-no-repair` again, or earlier on the owner's word.

The tripwire that watched for these instances left the tripwires home when it fired, per
`skill@knowledge-architect-issue-tracking@movement-instruction`; this entry carries its instances.

### Why it matters

A pointer written bare is read by no check, so it dangles unseen when its target moves, against
`goal@knowledge-architect@documentation-stays-consistent`. The two later instances each had a
repair in reach, a checked form or an admitted bare name, which the finding's message did not name
or no text gave.

### What would close it

A design discussion under `skill@knowledge-architect-design` that keeps
`design@agent-skills@plain-text-is-no-repair` with its reasons, or changes it: at least, whether a
check reports a path-shaped or a skill-name-shaped token outside backticks, beside class A, with a
census of its false positives over this tree and thaum's.
