---
kind: todo
---
# A project cannot change one instruction of an installed skill

## Summary

An installed skill is compared byte for byte with the version the project pins, so a project
cannot edit it. A project adds to it through a skill of its own, and the installed
agent-configuration skill says such an addition "never contradicts" the installed text. A project
whose need is to change or remove one instruction of an installed skill has no sanctioned means.
This repository already sets installed instructions aside in its root CLAUDE.md, in three cases, so the need
is met here, not only in an adopting project.

## Details

### What

The owner's leads, from the owner list of the first run of the agentic-workflow axis (F26): a
project is allowed to supersede installed text, but this is not recommended. A supersession is
documented in the root CLAUDE.md, either by holding the superseding instruction or by a table that
maps every superseding instruction, and each says what is superseded, where the superseding
instruction is, and why it is needed, concisely. For instance, a project's own `<project>-review`
skill could hold instructions that supersede some of `skill@knowledge-architect-review`. The
owner: "This practice should be allowed, but not recommended."

The three supersessions this repository holds today, each ruled by the owner:

- the project skill prefix `klarch-`, against the installed rule that the prefix is the project's
  name and a hyphen, per `instructions@repository-skills`;
- a retrospective's workflow file handled here rather than filed as an issue on GitHub, per
  `design@knowledge-architect@workflow-findings-stay-here`; with it, `skill@klarch-retrospective-intake`
  sets aside two instructions of `skill@knowledge-architect-retrospective-intake`: a workflow
  file's findings are resolved rather than read for grounding only, and the outcome "sent
  upstream" does not apply; and an item met outside the task whose fix lies in the installed text
  or in the checker takes the rows of `primer@met-outside-the-task` rather than going to the
  workflow file;
- the agentic-workflow axis's findings on the installed text edited at their source, with no
  upstream file, in `instructions@repository-skills`.

The need is met in an adopting project too. thaum, on knowledge-architect 0.6.0, set two installed
instructions aside on its owner's rulings, each stated in its root agent instructions: the premise
procedure of `primer@owner-word-premise` for a reading its specification's text answers, and entry
test 2 of `primer@design-heads` for decisions about the content of its agent workflow. It first
wrote the first as an addition to the primer, as the agent-configuration skill allows; two of its
reviewers showed the addition still contradicted the primer, and its owner then ruled the installed
text wrong for the project on that point. The first override answers a gap the primer has since
closed, under `design@agent-skills@questions-go-to-their-authority`; the second stands as that
project's own scope. The owner judges its reason outdated, and plans to have that project record its
design decisions about its agent workflow again, a change the owner expects to settle the case
of `rejected` entries that lost to its instructions.

An earlier lead, kept: a patch stored as a diff in the project, applied by the install and checked
by the check, so that an upgrade either applies it again or reports that it no longer applies.

### Why it matters

`design@agent-skills@overlay-by-separate-skills` lets a project add, never contradict. A project
whose need is a contradiction is left two moves: report it to this repository, or declare
`harness = []` and lose the installed workflow, the checks of `design@check@installed-files-compared-by-bytes`
and the overlay rule with it. That threatens `goal@every-target-project-can-adopt-it`.
This repository's own three supersessions stand against the agent-configuration skill's "never
contradicts it" as written, which the first run of the agentic-workflow axis reported. The
retrospective asks about the need at every session, per
`design@agent-skills@premortem-as-watch-points`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that decides the shape from the
owner's leads, and its work landed: the agent-configuration skill and
`design@agent-skills@overlay-by-separate-skills` say how a project supersedes installed text and
where it documents each supersession, and this repository's root CLAUDE.md documents its three in
that shape.

### What bears on it

A superseding instruction is agent-facing text, so the shape decided writes it under
`skill@knowledge-architect-agent-configuration@admission-standard`, per
`design@agent-skills@admission-rules-home`.

A supersession missed where it is documented: in this repository's slice on splits planned before
the work, 2026-11-14, a dispatched implementer met a fix in the installed text and reported it for
the workflow's file, by `primer@met-outside-the-task`, missing the root's supersession of that
route, which `instructions@repository-skills` states as its paragraph's second bolded sentence;
the dispatching session routed it by the supersession. The retrospective proposed lifting the
sentence to the head of the section.

Further instances, from the retrospective files received at the intake of twenty files: the
session on committed transcripts, 2026-11-16, recorded two such items as kept for the workflow's
file, in a turn's closing section and in a review record, which the routing reviewer caught (P1 of
2026-11-16-knowledge-architect-committed-transcripts); the session on the owner's rulings and title
rules, 2026-11-22, kept a record-search miss for its retrospective, which the last transcript
review caught (P1 of 2026-11-22-knowledge-architect-owner-rulings-and-title-rules); the session on
the test temporary directories, 2026-11-25, sent one to the workflow's file, which the transcript
review caught (P1 of 2026-11-25-knowledge-architect-tmp-cleanup). Four sessions in eleven days, each
routing by the primer's paragraph at a turn's closing section while the supersession sits in the
root's section on skills. The leads they propose: a row of the root's knowledge table; the sentence
moved to a section of the root named for the moment; no change, the sentence being in context. The
shape decided here settles where it goes.
