# 2026-11-16-knowledge-architect-doc-gate-and-lints

- **Received file**: 2026-11-16-knowledge-architect-doc-gate-and-lints.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-147-gbb93531b` (commit bb93531b) when the work began, branch `doc-gate-and-lints`, tree clean."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@restatement-size-test`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@xtask@gates-list-primary-home`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.

## gate-list-restatement-drifted: the root `CLAUDE.md` restated the gate list over several sentences, and it drifted

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints P1.

- **a. What it says.** `instructions@verify-mechanically` said "It runs five gates in cost order:
  ..." beside "adds a sixth gate, `rebased`", a restatement of `design@xtask@gates-list-primary-home`
  longer than one sentence, and `path@xtask@README.md` said "the five gates". The doc gate made both
  false; the routing review found the restatement, and the branch converted it to a pointer with the
  CI command restated. Repaired in the session; no proposal.
- **b. Whether it still applies.** No. At `bb93531b`, line 107 of the root `CLAUDE.md` read "It runs
  five gates in cost order: `fmt`, `check` (`cargo klarch check`), `commits`", and line 114 "adds a
  sixth gate, `rebased`"; line 15 of `path@xtask@README.md` read "run the five gates". On main,
  `path@knowledge-architect@CLAUDE.md` line 107 reads "Which gates it runs is
  `design@xtask@gates-list-primary-home`", and a `grep` for "five gates" and "sixth gate" over both
  files returns nothing. The repair is 41972e22, "[gates] The recommended list gains a doc gate that
  denies rustdoc's warnings", on main, whose message says the root `CLAUDE.md`'s restatement "is
  converted to a pointer to that head with one sentence restated, the CI command".
- **c. Whether it is already recorded.** `design@agent-skills@restatement-size-test` (`constrains`)
  is the rule the conversion followed: "an existing longer restatement is converted when a change
  edits what it says". No issue.
- **d. Its kind.** A defect of the text, wrong at the file's version: a restatement that drifted from
  its home.
- **h. The proposal.** No change: already repaired.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground: 41972e22, on main, converted the root `CLAUDE.md`'s gate list to a pointer to `design@xtask@gates-list-primary-home` and corrected the xtask README.

## promise-to-owner-unrecorded, member P2

Members: 2026-11-16-knowledge-architect-doc-gate-and-lints P2.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`,
in its section on promise-to-owner-unrecorded, which gives the outcome.
