---
kind: design
---
# Pointers written as bare plain text to clear a finding reach no check, and the rule against them is reopened

## Summary

`design@agent-skills@plain-text-is-no-repair` refuses plain text as the repair of a finding. Its
tripwire fired at its second instance, so the decision is reopened, with a mechanical check for
plain-text pointers among the candidates. Three instances are known. The finding on a
reference-shaped token outside backticks, `design@core@reference-shape-outside-span`, reports the
first, references in the checker's syntax with their backticks removed, and neither of the other
two: a bare path, and the bare name of a skill still to be created, which no table defines.

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

Re-entry point: a further instance, a pointer written bare that no check reads, or earlier on the
owner's word.

The tripwire that watched for these instances left the tripwires home when it fired, per
`skill@knowledge-architect-issue-tracking@movement-instruction`; this entry carries its instances.

A fourth instance, which meets the re-entry point, and a form the checker lacks: a path at the root
of every installing project, named from a published Component's text. Two heads of
`register@agent-skills@design`, `design@agent-skills@committed-intake-analysis` and
`design@agent-skills@received-file-marked`, name docs/retrospective-reports/ at the project's root
in plain text; each now cites this entry beside the path. `path@*@<path>` names each Component's own
copy and resolves in no Component of an installing project before its first intake, and a path
anchored at this repository's root names this repository's directory only. The installed text names
the same directory and docs/audit-reports/ in plain text too, in the primer's knowledge table and
closing paragraph, the intake skill and the design-record auditor's definition; it cannot cite this
entry, per `design@agent-skills@shipped-text-cites-no-entry`, and a mention of a directory the reader
is not asked to follow is outside `design@agent-skills@plain-text-is-no-repair`. Two more sites:
`design@agent-skills@committed-audit-report` writes the audit directory as a span with a
placeholder, which the checker reads as no reference, and this repository's own intake skill,
`skill@klarch-retrospective-intake`, names the analyses' directory bare in its description, and
`issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` writes the installing
project's paths bare at five sites. A
candidate for the discussion: a reserved anchor for every installing project's root, which changes
`design@core@reserved-anchors` and `design@core@nothing-of-a-project-is-compiled-in`. Reported by the
retrospective of the intake of 2026-10-19, analysed at the intake of 2026-10-24.

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
