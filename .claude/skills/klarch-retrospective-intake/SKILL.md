---
name: klarch-retrospective-intake
description: MUST use, together with knowledge-architect-retrospective-intake, which it adds to, when the owner hands this session a file of findings on the workflow to act on — a `-klarch-workflow.md` file, written by a retrospective or by a project audit, or a `-workflow.md` file written before that name, from this repository or from a project that uses the workflow — or this repository's own retrospective project file, or asks what to do with the findings of such a file; and while an outcome of an analysis of such a file in docs/retrospective-reports/ is carried out. Covers what this repository adds as the workflow's upstream: the files it takes, another project's file read for grounding, the commit an analysis is checked against, the routes to the installed text's source and to the Rust source, the outcomes, and what becomes of the received files once their analyses leave. Not for running a retrospective or an audit, nor for the work an outcome rules, which its route's skill covers.
---

# Retrospective intake, as the workflow's upstream

Scope: what this repository adds to `skill@knowledge-architect-retrospective-intake`, as the
repository that receives the workflow files of every project and edits the installed text at its
source. Read the installed intake whole and follow it, with the additions below. Three of them set
an instruction of the installed intake aside: a workflow file's findings are resolved here, and a
fix in the installed text or in the checker is handled here rather than sent upstream, both this
repository's departure, `design@knowledge-architect@workflow-findings-stay-here`, which the
root `CLAUDE.md` states in `instructions@repository-skills`; and this repository's own received
files are deleted rather than marked, on the owner's word, as the last section says.

## What it takes `##what-it-takes`

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

The main commit you analyse against is written as `git describe --tags origin/main` gives it,
which adds to the commit the release tag it follows. A W or C finding is read against the installed text's source under
`path@agent-skills@content/` and the checker's source: first at the version the file's version
line names, as a tag of this repository, then at the analysed commit. The commit a workflow file
of another project names is that project's, and is not read here.

## Routes and outcomes `##routes-and-outcomes`

Here, two routes of the installed intake's route g read:

- a fix in the installed text: edited at its source under `path@agent-skills@content/`, as
  `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says, read whole;
- a fix in the checker, or any change to the Rust source: `skill@klarch-development`.

For a workflow file and for this repository's own project file, the outcomes are three: handle now,
open an issue, and no change with the reason. "Sent upstream" does not apply: this repository is
the upstream.

The committed analysis of these files is `design@knowledge-architect@committed-findings-analysis`.

## The analysis of a received file `##received-analysis`

**A standing answer of a workflow file gets a section of its own**, ending with its `Default` and
`Outcome` lines, as a finding's does.

**Name nothing of the other project in detail**: none of its paths, none of its design heads, none
of its internals, which tell a reader of this repository nothing. Say it in words where the finding
needs it, as "the project's design home". The directory's README,
`path@knowledge-architect@docs/retrospective-reports/README.md`, says what it holds.

## What becomes of the received files `##received-files-after`

Once the analyses leave, every received file the intake analysed is dealt with, in the place the
owner keeps it:

- **A file written by a session of this repository**, its workflow file or its project file, is
  deleted: the analysis, which history keeps, holds every finding and its ruling.
- **A workflow file from another project** is marked, as the installed intake marks a project file:
  a line at its head saying that it was analysed and every outcome carried out, and that
  `git log -- docs/retrospective-reports/<stem>.md` in this repository finds the analysis.
- **Another project's own file, read for grounding**, is neither marked nor deleted: its findings
  are that project's, for its own intake.
