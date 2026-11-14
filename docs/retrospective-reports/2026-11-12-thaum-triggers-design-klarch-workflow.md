# 2026-11-12-thaum-triggers-design-klarch-workflow

- **Received file**: 2026-11-12-thaum-triggers-design-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: "knowledge-architect `=0.9.0`, pinned by the project."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, respect.
  - `design@agent-skills@undesigned-work-is-an-issue`: constrains, respect.
  - `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
  - `design@agent-skills@tripwire-stands-on-the-standard`: constrains, respect.
  - `design@check@tripwire-horizon-unchecked`: conflicts, check-reversal.
  - `design@check@a-file-register-is-a-directory-of-entries`: constrains, respect.
  - `rejected@check@validity-marker-checked`: constrains, respect.
  - `design@knowledge-architect@check-crate`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.
  - `design@knowledge-architect@cli-crate`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.
  - `design@query@cluster-rooted-at-a-subject`: conflicts, check-reversal.
  - `rejected@query@bound-in-entries`: brings-back, new-argument-needed.
  - `rejected@agent-skills@dispatch-size-hint`: constrains, respect.
  - `design@agent-skills@subject-split-standard`: constrains, respect.
  - `design@agent-skills@splits-planned-before-work`: constrains, respect.
  - `design@agent-skills@record-split-found-by-a-command`: constrains, respect.
  - `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`: brings-back, new-argument-needed.
  - `rejected@agent-skills@subject-groups-bounded-at-sixty`: brings-back, new-argument-needed.
  - `rejected@check@split-enforced-by-a-check`: constrains, respect.
  - `goal@projects-are-split-into-modular-units`: constrains, respect.

## worklist-cluster-too-loose: a work-list cluster held 62 loosely related entries

Members: 2026-11-12-thaum-triggers-design-klarch-workflow C1.

- **a. What it says.** "Reported by a record-search agent: one cluster held 62 entries over about
  2 470 lines", heads of one subject beside unrelated roadmap issues of the project, and "Its reading
  strained the agent". Proposed: "A cluster bound in entries, or a split by sub-subject". The file
  reports strain, and no entry missed.
- **b. Whether it still applies.** The cluster is drawn from the project's own record. At `v0.9.0`,
  the version the file pinned, the searcher's text reads "A cluster is what a piece of work reads
  together: a subject of a design home, or a group of a declared register, with the `rejected`
  entries its entries cite, the issues and tripwires that cite them, and the goals they serve"; at
  origin/main, `v0.10.0-59-g605d1538`, the same sentence without the goals, which goal agents now
  grade, and with every entry of the same Component or declared location, per
  `design@query@cluster-rooted-at-a-subject`. A large subject still makes a large
  cluster: the size is the project's subject.
- **c. Whether it is already recorded.** Yes, both proposals are recorded and lost:
  - a bound in entries is `rejected@query@bound-in-entries` ("a count of entries bounds an agent's
    load poorly; the bound of 60 standing entries rested on their mean size in thaum") and
    `rejected@agent-skills@design-listing-cut-in-groups-of-sixty` ("the owner believed that a grouping
    by quantity alone would most likely make an agent read unrelated entries"), each graded
    `brings-back`;
  - `rejected@agent-skills@subject-groups-bounded-at-sixty` names its own reopening: "this is the
    candidate if reports of large subjects come with missed entries". This report names no missed
    entry, so it is no new argument;
  - a split by sub-subject in the work list cuts a unit otherwise than by the headings of its home,
    against `design@query@cluster-rooted-at-a-subject` ("the owner chose that grouping for design
    entries, since a subject's kept and rejected entries have to be read together").

  The record's answer to a strained subject is that the searcher reports it ("A subject whose size
  strained your reading is an item met outside the task, with its cluster's name, as a sign that it
  needs splitting", in the description of `agent@knowledge-architect-record-searcher`, present at
  `v0.9.0` too), and that the project splits the subject under
  `design@agent-skills@subject-split-standard`, which this report's agent did.
- **d. Its kind.** An observation: the channel the record provides carried the strain to the
  session; the split is the project's own fix.

Default: no change.
Outcome: decided by the session: no change. Ground: both proposals bring back recorded rejected
designs with no new argument, the reopening condition of
`rejected@agent-skills@subject-groups-bounded-at-sixty` (missed entries) is not met, and the
reported strain reached the session by the channel the record gives it, the subject's split being
the project's under `design@agent-skills@subject-split-standard`.

## closing-todo-shifts-horizons: closing a `todo` entry at the plan's commit shifts every horizon naming its closing

Members: 2026-11-12-thaum-triggers-design-klarch-workflow W2, C2. One session, in the other project.

- **a. What it says.** W2: the plan's commit closed a `todo` issue, as `skill@knowledge-architect-planning@spec-or-milestone` says, "The plan document that schedules it closes that issue in the commit that adds the plan document". Six tripwire horizons named that issue's closing, written when the closing meant the work's landing. The session rewrote five to name the plan document's deletion and missed the sixth, whose "two releases after the issue closes" then counted from the plan's merge; three reviewers caught it before the merge. Proposal: a checker finding, at a commit that deletes an issue entry and adds a plan citing it, listing every tripwire whose `Watches until` names that entry (no instruction added). C2: no command lists the tripwires whose horizon names an entry; proposal: `cargo klarch tripwires --watching <ref>`, or a `check --staged` finding.
- **b. Whether it still applies.** The planning sentence stands at origin/main, `path@agent-skills@content/skills/planning/SKILL.md` line 88, as at `v0.9.0` (line 76). The command exists for one entry: `cargo klarch show <ref>` prints "every reference to it", the `Watches until` line included. Measured: `cargo klarch show spec@plans@path-quickfixes` lists line 69 of `register@check@tripwire`, the line "**Watches until:** the commit on the main branch that deletes `spec@plans@path-quickfixes`". And a deleted entry's references dangle: each is a finding of `cargo klarch check` at the commit that deletes the file.
- **c. Whether it is already recorded.** `design@check@a-file-register-is-a-directory-of-entries` (respect): closing an entry "turns closing an entry into a dangling-reference work list: the file is deleted, and every reference to it becomes a finding". `design@check@tripwire-horizon-unchecked` (conflicts, check-reversal, owner-question) keeps the horizon line out of the checks "for now" on the owner's choice; W2's checker finding would reverse it, and brings back the syntax gap of `rejected@check@validity-marker-checked` (respect). `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries` (touches) records the listing for a set of entries; one closed issue is one entry, which `show` already serves. `design@agent-skills@undesigned-work-is-an-issue` (respect) keeps the close at the plan's commit. `design@agent-skills@a-reference-claims-a-revisit` (respect): the inbound list is `show`'s to compute.
- **d. Its kind.** A lapse of the session: the texts to revisit were listed, by the dangling findings for a horizon citing the issue by reference, and by `show` before the deletion. The file does not say whether the sixth horizon cited the issue by reference or named it in words; in words it would have escaped every listing, and would itself depart from `primer@where-knowledge-goes`, "A reference is written wherever the text would have to be revisited if the entry it names changed" (an assumption either way). One slip, caught by three reviewers.
- **e. Its scope.** The installed text and the checker, `agent-skills` and `check`.
- **f. A better fix.** No change. A `--watching` filter lists what `show` lists for one entry; a `check --staged` finding reverses the owner's choice in `design@check@tripwire-horizon-unchecked` with no new argument beyond one slip a review caught.
- **g. Its route.** Nothing.

Default: no change.
Outcome: decided by the session: no change, a lapse. Ground: `cargo klarch show <ref>` lists every horizon citing an entry, and its deletion makes each a finding, per `design@check@a-file-register-is-a-directory-of-entries`; the proposed check would reverse `design@check@tripwire-horizon-unchecked` on one slip that its review caught.

## standing-2026-11-12-thaum-triggers-design-klarch-workflow: the standing answers

Members: the section "Standing questions" of 2026-11-12-thaum-triggers-design-klarch-workflow,
lines 97 to 116, of a project that installs the workflow, pinned at 0.9.0. It predates the
questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- the owner's words quoted verbatim in the first commit's message (lines 101 to 103):
  `owner-attribution-slips`;
- four of the 14 searchers giving a relation the role does not admit, each refused by `--verify`
  and repaired when sent back (lines 104 and 105): `searcher-role-relations-unstated`;
- C3 and C4, and the reviewers' reports saved with `cargo klarch transcript show --full` "since a
  report arrives as a message and the dispatcher would otherwise retype it" (lines 111 to 116):
  `named-repairs-applied-by-hand`, `unread-may-bear-list-fails-commits` and
  `no-command-saves-subagent-report`.

Analysed here:

- **A project skill's addition missed** (lines 106 to 109): the project's planning skill "requires
  naming each dropped lead of the closed `todo` entry with the condition it fails. Three leads were
  dropped silently. Four reviewers caught it". Kind: a lapse against the project's own skill. Its
  fix, if any, lies in that project's text, for its own intake; no installed text is blamed. The
  same act, an issue closed at the plan's commit losing content, is the subject
  `closed-issue-content-lost-at-plan` (W1 of
  2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow), which may
  count this as a consumer's instance of the content a closing loses.
- **Installed file changed**: no. **Backticks**: no. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: the one lapse is against the project's own
skill, outside the installed text, and the act it bears on is weighed under
`closed-issue-content-lost-at-plan`.

## searcher-role-relations-unstated: the report skeleton named no relation each role admits

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-thaum-split-oversized-modules-klarch-workflow.md`, in its section on `searcher-role-relations-unstated`, which gives the outcome.

## bears-file-too-large-to-read-whole: the grounding's "read whole `bears.md`" cannot be followed when the graded set exceeds one session's reading

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `bears-file-too-large-to-read-whole`, which gives the outcome.

## unread-may-bear-list-fails-commits: the list of verdicts with the searchers' reasons is a hand-built join that fails `commits`

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow.md`, in its section on `unread-may-bear-list-fails-commits`, which gives the outcome.

## named-repairs-applied-by-hand: the checker names each repair of a reference, and sessions apply them by scripts

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `named-repairs-applied-by-hand`, which gives the outcome.

## no-command-saves-subagent-report: the dispatcher saved each reviewer's report from the message, by hand or by a script over transcript files

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `no-command-saves-subagent-report`, which gives the outcome.

## owner-attribution-slips: the owner quoted, or given a ruling or a reason the owner did not give

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.md`, in its section on `owner-attribution-slips`, which gives the outcome.
