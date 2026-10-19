---
name: klarch-retrospective-intake
description: MUST use, together with knowledge-architect-retrospective-intake, which it adds to, when the owner hands this session a file of findings on the workflow to act on — a `-klarch-workflow.md` file, written by a retrospective or by a project audit, or a `-workflow.md` file written before that name, from this repository or from a project that uses the workflow — or this repository's own retrospective project file, or asks what to do with the findings of such a file; and while an outcome of an analysis of such a file in docs/retrospective-reports/ is carried out. Covers what this repository adds as the workflow's upstream: the files it takes, another project's file read for grounding, the commit an analysis is checked against, the routes to the installed text's source and to the Rust source, and the outcomes. Not for running a retrospective or an audit, nor for the work an outcome rules, which its route's skill covers.
---

# Retrospective intake, as the workflow's upstream

Scope: what this repository adds to `skill@knowledge-architect-retrospective-intake`, as the
repository that receives the workflow files of every project and edits the installed text at its
source. Read the installed intake whole and follow it, with the additions below. Where an addition
differs from it, the addition is this repository's departure,
`design@knowledge-architect@workflow-findings-stay-here`, which the root `CLAUDE.md` states in
`instructions@repository-skills`.

## What it takes `##what-it-takes`

Beside the project file of the installed intake:

- **A workflow file**, the second file of `skill@knowledge-architect-retrospective@two-files`,
  whatever project wrote it, and whether a retrospective or the agentic-workflow axis of a project
  audit wrote it. It is named `<date>-<project>-<subject>-klarch-workflow.md`, or, before that
  name, `<date>-<project>-workflow.md` or `<date>-<project>-<subject>-workflow.md`. It holds the
  findings on the installed files (W), and, from a retrospective, the findings on the checker (C)
  and the answers to the standing questions. Here its findings are resolved, not only read.
- **This repository's own project file**, `<date>-knowledge-architect-<subject>.md`, or
  `<date>-knowledge-architect.md` before that name, which the installed intake takes too.
- **For grounding only: another project's file** of the same retrospective, when it is present. It
  can explain a workflow finding: what the project's text said, what the session did. Its own
  findings are that project's, and are fixed in its text. **Nothing of it enters the analysis
  beyond what the workflow file already holds**: the workflow file is written to be publishable,
  the project's file is not, and the analysis is committed to a public repository.

The findings of this repository's two files are handled here, never as an issue on GitHub, per
`design@knowledge-architect@workflow-findings-stay-here`.

## The commit an analysis is checked against `##analysed-commit`

The main commit you analyse against is `git describe --tags origin/main`, named in the analysis's
head. The version a received file used is its own version line's. A W or C finding is read against
the installed text's source under `path@agent-skills@content/` and the checker's source at that
commit; a finding against an older version of the checker may already be repaired.

## Routes and outcomes `##routes-and-outcomes`

Route g of the installed intake gains two routes:

- a fix in the installed text: edited at its source under `path@agent-skills@content/`, as
  `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says, read whole;
- a change to the Rust source: `skill@klarch-development`.

For a workflow file and for this repository's own project file, the outcomes are three: handle now,
open an issue, and no change with the reason. "Sent upstream" does not apply: this repository is
the upstream, and a fix in the installed text is handled now at its source or opened as an issue.

The committed analysis of these files is `design@knowledge-architect@committed-findings-analysis`.

## The analysis of a received file `##received-analysis`

**Name nothing of the other project in detail**: none of its paths, none of its design heads, none
of its internals, which tell a reader of this repository nothing. Say it in words where the finding
needs it, as "the project's design home". The directory's README,
`path@knowledge-architect@docs/retrospective-reports/README.md`, says what it holds.
