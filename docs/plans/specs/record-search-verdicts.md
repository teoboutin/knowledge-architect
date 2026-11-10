# The record search reports checkable verdicts, packed by search unit, and reads the draft of the plan

## Status and audience

This spec is the plan of work done on one branch and one pull request, on the owner's word: it
changes the record search, `klarch worklist`, and the installed texts that dispatch the search and
act on its output. It leaves when its work lands. Where it and a design home disagree on a point the
section "What is already decided" does not list as reversed or rewritten, the design home wins.
Every name it uses is defined in it or exists in the code. Where the owner's word is awaited, as its
section "Defaults awaiting the owner" lists, `skill@knowledge-architect-planning@working-a-slice`
says what waits.

**The requirements of this spec are its sections "Decided design", "Mapping tables" and
"Implementation sequence".** The other sections record the discussion and the search that grounded
it; a sentence there binds the implementer only where one of those three sections states it.

It is assembled from one session's transcript, the session whose discussion begins with the owner's
message "I want to discuss the design record search procedure and the design audit step." A label
`L<n>` in an argument names line `n` of that session's extraction by
`cargo klarch transcript show <session> --from "I want to discuss the design record search procedure and the design audit step."`,
the session found by `cargo klarch transcript list --containing` with the same words.
The owner's rounds 2 to 10 hold a side task, the drafting of
`goal@agent-skills@session-orchestrates-the-work`, committed on this spec's branch before it; "F2",
"F3" and the other `F<n>` labels in the arguments name findings of that goal's reviews, which no file
keeps: what the owner ruled on each is in the owner's quoted answer.

## How the work is done

`skill@knowledge-architect-planning`, as the work of a spec.

## Names

- **the search**: the record search: `klarch worklist --out <dir>`, one agent per agent directory it
  writes, then `klarch worklist --verify <dir>`. Its code is `path@query@src/graph.rs` (`graph`,
  which builds the clusters and the residual groups, and `pack`), `path@query@src/worklist.rs`
  (`worklist`, `write`, `render`, `snapshot`, `relations`, `verify`, `assemble`, `parse_fields`,
  `judge`) and `path@cli@src/cli/mod.rs` (`WorklistArgs`, `worklist_mode`,
  `DEFAULT_WORKLIST_BOUND`).
- **the searcher**: `agent@knowledge-architect-record-searcher`, source
  `path@agent-skills@content/agents/record-searcher.md`.
- **a search unit**: a Component, or a location the manifest declares; the unit of packing.
- **a cluster**: a unit of a unit register, a design subject or a group of a declared register, with
  what `design@query@cluster-rooted-at-a-subject` joins to it; that head's own word for a design
  subject is "unit".
- **a context entry**: a head or a `rejected` entry that an entry an agent grades cites, and that the
  agent does not grade itself, given to that agent whole and read only.
- **a title line**: a goal's heading statement without its slug.
- **a cluster agent, a goal agent**: an agent directory `--out` writes, holding the entries of
  clusters, or the goals of search units.
- **a subject of the work**: a part of the work the dispatching session names, by an id in the grammar
  of an entry id: a thread at the design grounding, a cluster of findings at a retrospective intake,
  `slice-audit` or `spec-audit` at a design audit, `whole-work` where the brief names none.
- **a verdict**: one line of a searcher's report on one entry, shaped as "Decided design" says.
- **a verdict file**: `verdicts-<subject>.md`, what `--verify` writes for one subject of the work.
- **the gaps file, the fit lines**: `gaps.md` and `fit-lines.md`, what `--verify` writes at a design
  audit.
- **the draft**: the draft of the document that will carry a design's deliberation, the search's work
  at the design grounding.
- **the main session**: as `goal@agent-skills@session-orchestrates-the-work` defines it.
- **thaum**: another project of the owner that installs the workflow, at
  `path@elsewhere@../games/thaum`, whose manifest declares a location `rules`.

## What the work is

What exists today at each site:

- `graph` in `path@query@src/graph.rs` builds a cluster per unit of a unit register, with the
  `rejected` entries its entries cite, the standing entries that cite them, and the goals they cite
  and every goal those refine (`design@query@cluster-rooted-at-a-subject`,
  `goals-join-a-cluster-as-context` (`design@query@goal-context-once`)), whatever anchor each member sits in; the entries
  no cluster holds form a residual group per anchor (`residual-grouped-by-anchor` (`design@query@residual-grouped-by-search-unit`));
  `pack` packs the groups first-fit decreasing and spreads them under 25,000 words, a group above
  the bound alone (`design@query@clusters-packed-under-a-word-bound`).
- The searcher grades each entry `bears` with a relation, `may-bear`, or `unrelated`; the relations
  are `constrains`, `may change`, `may bring back`, `touches`, `closes`, `worsens`, `depends on`,
  `trigger met` and `fires`, by role (`relations`); `--verify` checks the reports against `agents.txt`
  and assembles `bears.md` and `may-bear.md` (`graded-entries-assembled` (`design@query@occasion-shaped-output`)); `shown.md`
  holds each entry's text followed by its inbound list (`snapshot`).
- The design skill's loop step 1 runs the search with the problem and the drafted shapes as the work,
  "at the latest before any thread closes", reads `bears.md` whole and `may-bear.md` on touch
  (`may-bear-read-on-touch` (`design@agent-skills@session-acts-on-the-verdicts`)), and runs it again for a thread on a subject the
  first work did not cover. The design audit of a slice seeds it with the decisions the milestone
  document lists and those the slice's spec cites, reads both files whole, and gives one line to each
  entry graded `bears` or `may-bear` in its commit message. The intake reads every issue and tripwire
  graded `bears` or `may-bear` whole, and searches again for a fix on a subject the files did not
  raise.

The measurements the design rests on are in the arguments, each with its instrument; the scripts
the session ran are in the message of the commit that revised this spec after its first review.

Outside the work:

- a lighter pass for untouched Components, `issue@agent-skills@a-lighter-pass-for-untouched-components`,
  which stays open on the owner's planned project of about 30 Components;
- a partitioned procedure of the intake, parked as
  `thread@record-search-verdicts@intake-partitioned-procedure`;
- where the judgement of a dispatch is stated, in
  `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`;
- the rewording of `goal@agent-skills@only-needed-instructions`, which the owner asked for in round
  14: drafted under the goal-setting skill on this spec's branch before it merges, on the owner's
  word.

## What is already decided

Rested on, not argued again: `goal@agent-skills@session-orchestrates-the-work`,
`goal@knowledge-architect@agents-work-without-drift`,
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
`goal@query@the-record-is-consulted-as-the-workflow-requires`,
`design@query@record-is-every-register-entry`, `design@agent-skills@synthetic-evidence-not-built`,
`design@agent-skills@ledger-from-transcript`.

Reversed or rewritten, each at step 5 (the harvest) unless named:

- `query`: `design@query@cluster-rooted-at-a-subject`, `goals-join-a-cluster-as-context` (`design@query@goal-context-once`),
  `residual-grouped-by-anchor` (`design@query@residual-grouped-by-search-unit`), `design@query@clusters-packed-under-a-word-bound`,
  `design@query@worklist-writes-and-verifies-the-reports`, `graded-entries-assembled` (`design@query@occasion-shaped-output`).
  The rewritten heads keep citing `rejected@query@goal-rooted-clusters`,
  `rejected@query@bound-in-entries`, `rejected@query@gather-reads-the-model`,
  `rejected@query@goals-pull-their-citers`, `rejected@query@one-residual-group` and
  `rejected@query@packing-to-the-bound`.
- `agent-skills`: `design@agent-skills@one-record-searcher`, `design@agent-skills@searcher-grades-goals`,
  `search-relevance-grade` (`design@agent-skills@verdict-report`), `may-bear-read-on-touch` (`design@agent-skills@session-acts-on-the-verdicts`),
  `unread-entries-reach-the-conformance-review` (`design@agent-skills@verdicts-reach-the-conformance-review`),
  `design@agent-skills@seeds-name-decisions`, `search-waits-until-the-scope-settles` (`design@agent-skills@search-runs-once-the-shapes-settle`, `design@agent-skills@search-reads-the-draft`),
  `design@agent-skills@record-searched-before-the-work`, `design@agent-skills@search-reads-the-whole-record`,
  `design@agent-skills@design-hands-off-to-planning`, `design@agent-skills@subject-split-standard` (with
  `path@agent-skills@fragments/subject-split.md`), and the sentence of
  `design@agent-skills@no-thread-closes-before-the-search` on the cost of a reading after the owner's
  rulings.
- the root: `design@knowledge-architect@query-crate`, for a second use of `git` in `query`.
- reopened with a new argument, not built: `rejected@query@one-search-agent-per-anchor`, which
  stays, its reason rewritten to what still holds.
- a reason defeated: `rejected@agent-skills@conformance-reader-at-the-checkpoint`, its reason
  rewritten with the two heads that cite its cost.
- reworded: `tripwire@agent-skills@owner-proposal-built-ungrounded` (its "entries it left unread")
  and `tripwire@agent-skills@record-search-misses` (its `bears`, `may-bear` and named doubt).

The texts that cite each reversed or rewritten head, by `cargo klarch show`, with the number of
citations, outside this spec. Each is read at the harvest, except the installed texts under
`path@agent-skills@content/` and the code comments, which step 4 and step 2 rewrite:

- `design@query@cluster-rooted-at-a-subject`: `path@agent-skills@content/skills/retrospective-intake/SKILL.md` (1), `path@agent-skills@docs/design/record-search.md` (2), `register@agent-skills@rejected` (1), `path@cli@README.md` (1).
- `goals-join-a-cluster-as-context` (`design@query@goal-context-once`): `path@agent-skills@docs/design/record-search.md` (2), `register@query@design` (1).
- `residual-grouped-by-anchor` (`design@query@residual-grouped-by-search-unit`): `register@agent-skills@rejected` (1).
- `design@query@clusters-packed-under-a-word-bound`: `path@agent-skills@docs/design/record-search.md` (1), `path@agent-skills@docs/design/what-earns-a-record.md` (1), `register@agent-skills@rejected` (2), `register@agent-skills@tripwire` (1), `register@query@design` (1).
- `design@query@worklist-writes-and-verifies-the-reports`: `path@agent-skills@docs/design/record-search.md` (2), `register@agent-skills@rejected` (1), `path@query@docs/open-issues/a-searcher-fills-its-report-by-a-script-of-its-own.md` (1), `path@query@docs/open-issues/no-command-lists-the-standing-entries-citing-a-set-of-entries.md` (1), `the-report-skeleton-names-no-relation-its-entry-s-role-admits` (1), `path@query@src/worklist.rs` (1).
- `graded-entries-assembled` (`design@query@occasion-shaped-output`): `path@agent-skills@docs/design/record-search.md` (1), `the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion` (2), `register@query@design` (2), `the-graded-entries-repeat-a-shared-entry-s-grade-per-agent` (2), `path@query@src/worklist.rs` (1).
- `design@agent-skills@one-record-searcher`: `path@agent-skills@content/skills/retrospective-intake/SKILL.md` (1), `path@agent-skills@docs/design/audit-method.md` (1), `register@agent-skills@tripwire` (2), `the-report-skeleton-names-no-relation-its-entry-s-role-admits` (1).
- `design@agent-skills@searcher-grades-goals`: `path@agent-skills@content/skills/design/SKILL.md` (1), `path@agent-skills@docs/design/record-search.md` (1), `path@agent-skills@docs/open-issues/a-spec-worked-without-its-design-audit-missed-a-critical-gap.md` (1), `the-root-goals-home-is-larger-than-its-design-home` (2), `register@query@design` (1).
- `search-relevance-grade` (`design@agent-skills@verdict-report`): `path@agent-skills@content/skills/retrospective-intake/SKILL.md` (1), `path@agent-skills@docs/design/record-search.md` (2), `register@agent-skills@tripwire` (1), `register@query@design` (1), `the-report-skeleton-names-no-relation-its-entry-s-role-admits` (1).
- `may-bear-read-on-touch` (`design@agent-skills@session-acts-on-the-verdicts`): `path@agent-skills@docs/design/record-search.md` (1), `path@agent-skills@docs/design/reviews.md` (1), `path@agent-skills@docs/open-issues/the-installed-text-has-the-main-session-carry-work-it-could-dispatch.md` (2), `the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition` (1), `the-search-s-graded-entries-exceed-what-one-session-reads-whole` (5), `register@agent-skills@rejected` (1).
- `unread-entries-reach-the-conformance-review` (`design@agent-skills@verdicts-reach-the-conformance-review`): `path@agent-skills@docs/design/record-search.md` (1), `the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion` (1).
- `design@agent-skills@seeds-name-decisions`: `path@agent-skills@content/skills/design/SKILL.md` (1), `path@agent-skills@docs/design/record-search.md` (1), `a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone` (2).
- `search-waits-until-the-scope-settles` (`design@agent-skills@search-runs-once-the-shapes-settle`, `design@agent-skills@search-reads-the-draft`): `path@agent-skills@content/skills/design/SKILL.md` (1), `path@agent-skills@docs/design/design-skill-and-heads.md` (1), `path@agent-skills@docs/design/record-search.md` (1), `the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition` (1).
- `design@agent-skills@record-searched-before-the-work`: `path@agent-skills@docs/design/project-configuration.md` (1), `path@agent-skills@docs/design/record-search.md` (1), `path@agent-skills@docs/design/reviews.md` (2), `path@agent-skills@docs/open-issues/a-pending-ruled-file-is-reached-by-no-installed-search.md` (2), `a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone` (2), `register@agent-skills@tripwire` (2).
- `design@agent-skills@search-reads-the-whole-record`: `path@agent-skills@docs/open-issues/a-lighter-pass-for-untouched-components.md` (1), `the-search-s-graded-entries-exceed-what-one-session-reads-whole` (1).
- `design@agent-skills@design-hands-off-to-planning`: `path@agent-skills@docs/design/plan-documents.md` (1), `path@check@docs/design/plan-documents.md` (1).
- `design@knowledge-architect@query-crate`: `register@query@design` (1), `path@knowledge-architect@docs/open-issues/the-checker-may-depend-on-every-crate-below-it.md` (1).
- `design@agent-skills@subject-split-standard`: `path@agent-skills@content/skills/component-creation/SKILL.md` (1), `path@agent-skills@docs/design/audit-record-axes.md` (1).
- `design@agent-skills@no-thread-closes-before-the-search`: `path@agent-skills@content/skills/design/SKILL.md` (2), `path@agent-skills@docs/design/owner-and-intent.md` (1), `path@agent-skills@docs/design/record-search.md` (2).

Closed by the work: `the-search-s-graded-entries-exceed-what-one-session-reads-whole`,
`a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`,
`the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion`,
`the-root-goals-home-is-larger-than-its-design-home`,
`the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`,
`the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`,
`the-report-skeleton-names-no-relation-its-entry-s-role-admits`.

Opened by the work: `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`,
which holds the parked intake thread.

The discussion ran two searches in the verdict shape, before it was built: the first with a work
statement as its work, whose findings `argument@record-search-verdicts@arg-38` answers; the second
with the draft of this spec. Each entry the second graded other than `unrelated` follows, with its
verdict, its action, and its outcome: the answer the discussion gave, or "respected by the decided
design", or "stays open". The searchers' reasons are not reproduced: they were written for the
session, and were not checked line by line.

- `design@agent-skills@verification-in-primer`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@code-change-reads-its-record`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@questions-go-to-their-authority`: conflicts, respect. Answered: `owner-question` takes every question `primer@question-authority` reserves to the owner.
- `design@agent-skills@owner-word-holds-as-far-as-its-premise`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@met-outside-the-task-outcomes`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@decisions-reported-to-the-owner`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`: closes, close-with-the-work. Closed by the work.
- `design@agent-skills@losing-alternatives-filter`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@losing-shape-test`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@rejected-citation-direction`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@subject-split-standard`: constrains, respect. Rewritten at the harvest with `path@agent-skills@fragments/subject-split.md`, which step 4 edits: a cluster holds the standing entries of its own search unit, and a split notice weighs a cluster's own entries.
- `design@agent-skills@thread-slug-is-entry-id`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@one-agent-reads-every-standing-entry`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@query@one-search-agent-per-anchor`: brings-back, new-argument-needed. Reopened with a new argument, per `thread@record-search-verdicts@component-packing`, and not built: small search units share agents. The entry stays, its reason rewritten at the harvest to what still holds: one agent per small anchor spends more on starting agents than on reading.
- `rejected@agent-skills@subject-groups-bounded-at-sixty`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@audit-method`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@refused-report-returned-as-text`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@run-counts-computed-by-the-workflow`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: touches, none. Stays open; the harvest rewrites its sentence on residual groups, which the work removes.
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: touches, respect. Respected: the generated lines carry no quotation, only references in checked forms.
- `the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion`: closes, close-with-the-work. Closed by the work.
- `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@milestone-split-by-lifetime`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@in-change-path`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@design-hands-off-to-planning`: conflicts, check-reversal. Rewritten at the harvest: the planning skill starts, on the full path, once the shapes settle, writes the draft the search reads, and assembles the plan from the transcript after convergence; on the in-change path the design skill drafts the commit message, as it holds that deliberation today.
- `design@agent-skills@ledger-from-transcript`: conflicts, check-reversal. Answered: the draft is the search's work only; the spec is assembled from the transcript after convergence.
- `design@agent-skills@completeness-check-is-a-step-claim`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@plan-records-the-exchange`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@premortem-as-watch-points`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@tripwire-recording-authority`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@standing-state-review-before-every-merge`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@plan-read-against-the-record`: conflicts, respect. Answered: the design-conformance review still reads the plan against the goals, design and rejected homes, and reads the verdict file beside them.
- `unread-entries-reach-the-conformance-review` (`design@agent-skills@verdicts-reach-the-conformance-review`): conflicts, check-reversal. Rewritten at the harvest: the reviewer reads the verdict files beside the homes, and at the merge of an in-change or bounded branch the axis is owed wherever the commit message's generated lines hold a verdict other than `constrains` and `unrelated`.
- `design@agent-skills@a-scratch-directory-per-subagent`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`: touches, none. Stays open; the condition of the conformance axis it names changes at step 4, from a list of unread entries to generated verdict lines.
- `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: touches, none. Stays open; `trigger-met` now covers a design issue's re-entry point at the search's occasions, which answers its instance at those occasions only.
- `design@agent-skills@bounded-path-in-design`: constrains, respect. Respected: the draft's form follows the path the session expects; the classification after the search still decides the path, per the decided design.
- `design@agent-skills@frontier-tier-only`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@extension-follows-the-ground`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@a-lighter-pass-for-untouched-components`: trigger-met, rule-before-work. Re-entry point met. The discussion keeps the whole-record search, on the owner's agreement, and leaves the issue open on the owner's planned project of about 30 Components; the harvest rewrites its re-entry point, which names an issue this work closes.
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: constrains, none. Respected by the decided design; no requirement beyond it.
- `tripwire@agent-skills@owner-proposal-built-ungrounded`: conflicts, check-reversal. Reworded at the harvest: its firing clause names the entries left unread, a record the verdict shape removes; it names the generated per-entry lines instead.
- `design@agent-skills@ruled-items-labelled`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@standing-argument-in-head`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@heads-stand-on-argument`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@fork-choice-in-weak-form`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@owner-rules-at-a-fork`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@default-ruled-before-merge`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `tripwire@agent-skills@owner-fact-recorded-unattributed`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`: touches, none. Stays open; no requirement on this work.
- `design@agent-skills@one-record-searcher`: conflicts, check-reversal. Rewritten at the harvest, as listed above.
- `design@agent-skills@search-apart-from-the-standing-state-review`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@search-reads-the-whole-record`: constrains, respect. Rewritten at the harvest: every search unit is read, residual groups no longer exist.
- `design@agent-skills@searcher-grades-goals`: conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `search-relevance-grade` (`design@agent-skills@verdict-report`): conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `may-bear-read-on-touch` (`design@agent-skills@session-acts-on-the-verdicts`): conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `search-waits-until-the-scope-settles` (`design@agent-skills@search-runs-once-the-shapes-settle`, `design@agent-skills@search-reads-the-draft`): conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `design@agent-skills@no-thread-closes-before-the-search`: constrains, respect. Rested on; its sentence on the cost of a reading after the owner's rulings is rewritten at the harvest with the reason of `rejected@agent-skills@conformance-reader-at-the-checkpoint`.
- `design@agent-skills@seeds-name-decisions`: conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `design@agent-skills@record-searched-before-the-work`: conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `rejected@agent-skills@conformance-reader-at-the-checkpoint`: conflicts, check-reversal. Its reason, rulings already given, is defeated by `design@agent-skills@no-thread-closes-before-the-search`; the harvest rewrites its reason and the two heads citing its cost.
- `rejected@agent-skills@delta-search-at-convergence`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design-grounding-reads-every-may-bear-entry-whole`: constrains, none. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@goal-seeds`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `intake-reads-every-may-bear-entry-whole`: constrains, none. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@search-after-the-first-challenge`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@search-before-and-after-the-challenge`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@search-work-is-the-question`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@searches-split-by-kind`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: constrains, none. Respected by the decided design; no requirement beyond it.
- `a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`: closes, close-with-the-work. Closed by the work.
- `issue@agent-skills@a-spec-worked-without-its-design-audit-missed-a-critical-gap`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`: touches, none. Stays open; the harvest removes its first bullet, the record-search departure this work repairs.
- `the-root-goals-home-is-larger-than-its-design-home`: closes, close-with-the-work. Closed by the work.
- `the-search-s-graded-entries-exceed-what-one-session-reads-whole`: closes, close-with-the-work. Closed by the work.
- `tripwire@agent-skills@record-search-misses`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@goals-change-through-goal-setting`: constrains, respect. Answered: a `conflicts` verdict on a goal routes to the goal-setting skill, never to `check-reversal`.
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: conflicts, respect. Answered: `owner-question` carries no rule of its own for a goal read two ways; it takes what `primer@question-authority` reserves to the owner.
- `design@agent-skills@goal-edit-ruled-alone`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@ledger-written-during-discussion`: constrains, respect. Respected: the draft is written once, when the shapes settle, read at once by the search it is written for, extended only for a later thread's own search, and never the source of the plan, per the decided design.
- `design@agent-skills@instruction-record-is-minimal`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@synthetic-evidence-not-built`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@capabilities-not-structure`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@statement-names-an-act`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@term-defined-at-first-load`: constrains, respect. Answered: the packing unit is named a search unit, and a unit of the work a subject of the work.
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: depends-on, none. Answered for this work by the owner's ruling, recorded in `design@agent-skills@synthetic-evidence-not-built`: the searches of this discussion are real use. The issue stays open on its census case.
- `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@a-past-sentence-is-rewritten`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@agent-skills@plain-text-is-no-repair`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@agent-skills@checked-commit-form-by-subject`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: trigger-met, rule-before-work. Re-entry point met, not taken: its question is where the claim check is delivered at a commit, which this work does not change; the generated lines only remove one class of hand-written claims.
- `issue@check@the-self-pin-refusal-has-no-discriminating-test`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@check@a-commit-message-is-a-document`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@branch-shas-are-refused`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@orphan-rejected-entry`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@goals-home-cites-upward-only`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@check@no-citation-direction-rule`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@check@a-home-for-developer-contracts-outside-agent-configuration`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@check@no-checked-form-cites-a-point-inside-a-section`: touches, respect. Respected: step 4 checks by hand every loop-step citation its edits renumber.
- `issue@check@install-agent-skills-exits-two-after-writing`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@check@plan-document-sections-by-kind`: constrains, respect. Respected: this spec holds the sections Arguments and New names, in one place.
- `design@check@retired-slug-reference-reported`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@reference-shape-outside-span`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@backticked-bare-id`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@plain-word-lint`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@quotation-read-by-no-id-lint`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@check@no-command-applies-the-repair-a-message-finding-names`: touches, none. Stays open; generated lines in checked forms remove one occasion of it.
- `design@check@every-path-names-its-anchor`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@check@declared-command`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@check@a-plain-hash-item-name-is-read-by-nothing`: touches, none. Respected: the generated lines cite a plan whole and name no item; `against` holds subject ids only inside the scratch verdict files.
- `issue@check@the-tests-read-the-machine-s-git-configuration`: worsens, respect. Respected: step 3's tests that commit use `Sandbox::git_command`; the remaining exposure is the issue's own.
- `rejected@check@split-enforced-by-a-check`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@checker-asks-git-through-the-crate`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@model-crate`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@git-crate`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@query-crate`: conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `design@knowledge-architect@cli-crate`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@mock-projects-in-one-crate`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@the-checker-may-depend-on-every-crate-below-it`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@the-mechanical-tasks-goal-omits-what-a-helper-does-better`: constrains, none. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@versioning-policy`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@changelog-entries`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: touches, none. Stays open; the work's outputs stay read by the session and the searcher only, as today.
- `design@knowledge-architect@no-branch-sha-is-cited`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`: constrains, none. Respected by the decided design; no requirement beyond it.
- `design@model@model-facade`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@plan-document-kinds`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@plan-item-scope`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@declarable-directory-register`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@a-reference-is-live-in-every-prose`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@anchors-are-components-and-locations`: conflicts, respect. Answered: a search unit is each Component and each location the manifest declares; a constructed location belongs to the search unit holding its home, per `thread@record-search-verdicts@component-packing`.
- `design@model@anchor-collisions-refused`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@one-entity-table`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@model@an-entity-belongs-to-its-anchor`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`: closes, close-with-the-work. Closed by the work.
- `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, none. Stays open; the harvest rewrites its sentence on `worklist`'s output.
- `the-report-skeleton-names-no-relation-its-entry-s-role-admits`: touches, none. Closed by the work: the skeleton line names the verdicts its entry's register admits, per the decided design.
- `design@query@query-depends-on-no-check`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@query@record-is-every-register-entry`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@query@cluster-rooted-at-a-subject`: conflicts, check-reversal. Answered: a standing entry that cites a head of another search unit gets that head's full text, counted in the bound, so it is judged beside the heads it cites; the head's word for a design subject keeps its sense, and the packing unit is named a search unit.
- `goals-join-a-cluster-as-context` (`design@query@goal-context-once`): conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `residual-grouped-by-anchor` (`design@query@residual-grouped-by-search-unit`): conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `design@query@worklist-writes-and-verifies-the-reports`: conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `graded-entries-assembled` (`design@query@occasion-shaped-output`): conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `design@query@clusters-packed-under-a-word-bound`: conflicts, check-reversal. Listed as reversed or rewritten; judged at the harvest.
- `rejected@query@bound-in-entries`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@query@gather-reads-the-model`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@query@goals-pull-their-citers`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@query@one-residual-group`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `rejected@query@packing-to-the-bound`: constrains, respect. Respected: the spreading over the agent count is kept for the search units below half the bound.
- `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`: touches, none. Stays open: the report stays a skeleton filled line by line.
- `issue@query@no-command-lists-the-entries-two-commits-differ-by`: touches, none. Stays open; the harvest rewrites its sentence on the only command reading a commit's tree, which `--since` changes.
- `design@git@git-is-asked-through-one-module`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@git@failed-git-is-an-error`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@cli@command-parsing-home`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@cli@arguments-parse-through-clap`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `design@cli@exit-code-ladder`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation`: touches, none. Stays open; step 1 adds a test of the default bound, which the issue names as missing, and changes the README's figure with it.
- `issue@cli@some-value-checks-are-made-after-parsing`: touches, none. Respected: `--since` and `--seeds-from` name things in the project, so the tool checks them, as the issue's own line says.
- `issue@testing@a-test-run-leaves-directories-in-the-temporary-directory`: worsens, respect. Stays open; the tests of steps 1 to 3 use the existing helpers and add instances of it.
- `design@testing@binary-tests-in-testing`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@no-contradiction-in-workflow`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@only-needed-instructions`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@agents-work-without-drift`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@design-with-agents-as-counterparts`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@documentation-stays-consistent`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@owner-attention-on-what-is-theirs`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@projects-are-split-into-components`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@serves-one-kind-of-project`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@the-owner-decides`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@the-owner-s-word-is-verified`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@installed-text-works-in-every-target-project`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@one-skill-per-activity`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@every-target-project-can-adopt-it`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@testing@a-consumer-test-uses-what-a-consumer-can`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@knowledge-architect@records-reach-their-reader`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@cli@one-home-for-the-command-line`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@git@a-pure-wrapper-over-git`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@model@a-tool-reads-the-documents-with-no-check`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@query@agents-query-the-project`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@query@the-record-is-consulted-as-the-workflow-requires`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@testing@program-tests-over-whole-projects-have-one-home`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@cli@a-refused-invocation-does-no-work`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@reviews-judge-what-no-tool-can`: constrains, respect. Respected by the decided design; no requirement beyond it.
- `goal@agent-skills@session-orchestrates-the-work`: conflicts, check-reversal. Answered: every `unrelated` line carries a one-line reason, so every conclusion carries evidence.

## Criteria

### Every entry the work bears on is judged against it before the work `##c-bearing-reaches-a-judge`

Binding, from `goal@knowledge-architect@agents-work-without-drift`, whose measure admits that the
reading is done "by a step the installed text owes an agent or a reviewer". Met by
`thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@component-packing` and
`thread@record-search-verdicts@search-reads-the-draft`.

### A conclusion acted on is checked as the main session checks its own claims `##c-claims-verified`

Binding, from `primer@verify-before-relying` and `goal@agent-skills@session-orchestrates-the-work`.
Met by `thread@record-search-verdicts@verdict-report`: the command checks each quotation against the
entry's own text, and the searcher checks its conclusions.

### What reaches the main session grows with what changes the work `##c-session-load-bounded`

Weighed when named in round 1; binding once `goal@agent-skills@session-orchestrates-the-work` was
written, per `argument@record-search-verdicts@arg-31`. Met by
`thread@record-search-verdicts@verdict-report` and `thread@record-search-verdicts@occasion-shaped-output`.

### The agent count grows with distinct entries, not with duplication `##c-agents-scale-with-record`

Weighed, from the owner's second flaw in round 1. Met by
`thread@record-search-verdicts@goal-context-once` and `thread@record-search-verdicts@component-packing`.

### Mechanical tasks are done by a command `##c-mechanical-by-command`

Binding, from `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`. Met by
`thread@record-search-verdicts@verdict-report` (the quotation check, the verdict files, the
generated lines) and `thread@record-search-verdicts@slice-audit-checks-fit` (`--since`,
`--seeds-from`).

### It works in a project of one Component and in a project with a large location `##c-any-project`

Binding, from `goal@agent-skills@installed-text-works-in-every-target-project`. Met by
`thread@record-search-verdicts@component-packing`: a project of one Component is one search unit,
split by its clusters above the bound; a declared location, such as thaum's `rules`, is a search
unit of its own; an intermediate Component's goals reach the agents of the Components it encloses.

## Threads

### The main session orchestrates the work, a goal `##orchestrating-session`

Proposed by the owner, round 1; framed as a goal by the session, round 1
(`argument@record-search-verdicts@arg-15`); the owner, round 2: "agreed this is a goal. I think your
draft is fairly correct with my intent. The reviews will judge any problem with it. I think we should
draft it as a goal in this session, so that it can shape the direction of the discussion more
reliably." Drafted and ruled in rounds 2 to 9 under the goal-setting skill, and committed on this
spec's branch as `goal@agent-skills@session-orchestrates-the-work`, before this spec. State:
`approved`. Section: none, since the goal is written. Harvest: none.

### Goals reach the cluster agents as titles, and goal agents grade them `##goal-context-once`

Proposed by the session, round 1 (`argument@record-search-verdicts@arg-9`), on the measurement of
`argument@record-search-verdicts@arg-7`. The owner, round 2: "goal-context-once: I agree with this."
Revised in round 13 from one goal searcher to goal agents packed under the bound; the owner, round
14: "goal-context-once: agreed to pack under the same bound, by component. This is clearly needed if
goal size exceeds the bound". The ground of the revision is the bound, not
`goal@agent-skills@only-needed-instructions`, which the owner showed misapplied there
(`argument@record-search-verdicts@arg-40`, `argument@record-search-verdicts@arg-42`). State:
`approved`, by the owner's word against the checkpoint, round 20: "agreed on the table". Section:
"Decided design". Harvest: `goals-join-a-cluster-as-context` (`design@query@goal-context-once`),
`design@agent-skills@searcher-grades-goals`.

### The bound is 40,000 words, counting everything an agent reads `##bound-raise`

Proposed by the owner, round 2 (`argument@record-search-verdicts@arg-16`): "I would also consider
raising the word cap, say to 30k." The owner reported that agents stay highly reliable to about
200,000 to 250,000 tokens and very reliable after that, and that token usage ramps up more noticeably
around 250,000 tokens, so the measure to weigh is cost. The session proposed 50,000
(`argument@record-search-verdicts@arg-21`). The owner, round 3: "about bound-raise: I think we can go
with a middle ground of 40k first. Because we are also demanding more detailed results of searchers,
I think we can raise incrementally, rather too much at once. This should leave plenty more ground."
State: `approved`, round 20. Section: "Decided design". Harvest:
`design@query@clusters-packed-under-a-word-bound`, `design@query@worklist-bound-is-40000-words`.

### The packing unit is a search unit, a Component or a declared location, alone above half the bound `##component-packing`

Proposed by the owner, round 1 (`argument@record-search-verdicts@arg-5`); argued by the session
(`argument@record-search-verdicts@arg-10`, `argument@record-search-verdicts@arg-22`). The owner, round
2: "component-packing agreed. In particular, an agent that ends up judging an issue of a component of
which it saw no design, would be de major defect IMO." The owner, round 3, narrowing it: "I think most
issues are actually fine to judge without the design. The hard case is one issue that cites a design
head." (`argument@record-search-verdicts@arg-26`), and proposing one agent per Component, split when
too large. The owner, round 12 (`argument@record-search-verdicts@arg-33`): "if a component exceeds
half of the bound, always alone (and split if it  exceeds the bound). Only components below half of
the bound are packed together. Large components are likely to be more critical to the project, so
splitting attention is not optimal IMO. And their entries probably have lots of interactions
together. This is a bit arbitrary though, I have no measurement to back that idea." The owner, round
14 (`argument@record-search-verdicts@arg-41`): "I think each location should be treated as one small
component (or not necessarily small) for that." The second search added declared locations only, and
context entries in full text (`argument@record-search-verdicts@arg-50`,
`argument@record-search-verdicts@arg-51`). The owner chose the half-bound rule over packing small
units into the room of large ones, which no goal ranks, at a cost of one agent over this repository
(`argument@record-search-verdicts@arg-34`). State: `approved`, round 20. Section: "Decided design".
Harvest: `design@query@clusters-packed-under-a-word-bound`, `residual-grouped-by-anchor` (`design@query@residual-grouped-by-search-unit`),
`design@query@cluster-rooted-at-a-subject`, `design@query@packed-by-search-unit`,
`design@query@constructed-anchor-in-its-home-s-unit`, `design@query@shared-entry-in-its-most-tied-cluster`,
`design@query@context-entries-given-whole`, `design@query@search-unit-above-half-the-bound-alone`,
`design@agent-skills@search-reads-the-whole-record`,
`design@agent-skills@subject-split-standard`; the reason of
`rejected@query@one-search-agent-per-anchor`.

### The searcher returns a verdict per entry, its quotation checked by the command `##verdict-report`

Proposed by the owner as a direction, round 1 (`argument@record-search-verdicts@arg-4`); shaped by the
session (`argument@record-search-verdicts@arg-11`, `argument@record-search-verdicts@arg-12`). The
owner, round 2: "verdict-report: I think this is a good shape." Reopened by the session in round 15 on
a conditional verdict consumed as unconditional (`argument@record-search-verdicts@arg-45`), with an
`if:` field; the owner argued parity of context instead (`argument@record-search-verdicts@arg-46`), and
the session withdrew the field for `owner-question` (`argument@record-search-verdicts@arg-47`). The
owner, round 17: "verdict-report agreed." Members from the second search
(`argument@record-search-verdicts@arg-50`) and from this spec's first review. State: `approved`,
round 20. Section: "Decided design". Harvest: `search-relevance-grade` (`design@agent-skills@verdict-report`,
`design@agent-skills@verdict-states-a-conclusion`, `design@agent-skills@every-verdict-quotes-its-entry`,
`design@agent-skills@a-seed-and-its-citers-are-never-unrelated`),
`may-bear-read-on-touch` (`design@agent-skills@session-acts-on-the-verdicts`), `design@agent-skills@one-record-searcher`,
`unread-entries-reach-the-conformance-review` (`design@agent-skills@verdicts-reach-the-conformance-review`),
`design@query@worklist-writes-and-verifies-the-reports`, `graded-entries-assembled` (`design@query@occasion-shaped-output`).

### One verdict file per subject of the work, the dispatch over them left to the main session `##occasion-shaped-output`

Proposed by the session, round 1 (`argument@record-search-verdicts@arg-13`). The owner, round 2: "I
am not convinced this fully solves the problem. For the intake, I believe a significantly different
procedure, with more steps to partition the work, might be neeeded. But today's scope shuld stay
relatively limited, so we can go with that at first". Reopened by the session in round 13 on a reading
of `goal@agent-skills@only-needed-instructions` (`argument@record-search-verdicts@arg-37`), which the
owner contested in round 14 (`argument@record-search-verdicts@arg-40`); the session read the goal
whole and withdrew its revision (`argument@record-search-verdicts@arg-42`). State: `argued`, round 14:
the owner's earlier word on the first shape stands, minus the sentence sending each cluster of
findings to one subagent, which `goal@agent-skills@session-orchestrates-the-work` rules out. Section:
"Decided design". Harvest: `graded-entries-assembled` (`design@query@occasion-shaped-output`).

### The design audit of a slice checks that the spec still fits `##slice-audit-checks-fit`

Proposed by the owner, round 2 (`argument@record-search-verdicts@arg-19`): "The goal is to verify that
the spec fits in the curent design and code (and fix if it does not), not to redo the work the design
session did." Shaped by the session (`argument@record-search-verdicts@arg-23`). The owner, round 3:
"slice-audit-checks-fit: agreed". Members from both searches and this spec's first review. State:
`approved`, round 20. Section: "Decided design". Harvest: `design@agent-skills@seeds-name-decisions`,
`design@agent-skills@record-searched-before-the-work`, `design@agent-skills@design-audit-checks-fit`,
`design@query@since-marks-changed-entries`, `design@query@seeds-from-the-plan-s-decided-section`.

### The search's work at the design grounding is the draft of the document that carries the deliberation `##search-reads-the-draft`

Proposed by the owner, round 17 (`argument@record-search-verdicts@arg-48`): "Maybe the best shape would
be to have a plan document already written". Argued by the session
(`argument@record-search-verdicts@arg-49`). The owner, round 18: "agreed, draft the plan and run the
search on it". Answered by the second search (`argument@record-search-verdicts@arg-50`): the draft is
never the plan document. The owner, round 19, on the searches of this discussion: "For the question,
I would count that as real use." State: `approved`, round 20. Section: "Decided design". Harvest:
`search-waits-until-the-scope-settles` (`design@agent-skills@search-runs-once-the-shapes-settle`, `design@agent-skills@search-reads-the-draft`),
`design@agent-skills@record-searched-before-the-work`, `design@agent-skills@design-hands-off-to-planning`,
and the reason of `rejected@agent-skills@conformance-reader-at-the-checkpoint`.

### The extracted transcript as the search's work `##search-work-carries-the-deliberation`

Proposed by the owner, round 16 (`argument@record-search-verdicts@arg-46`); costed by the session
(`argument@record-search-verdicts@arg-47`). The owner, round 17: "the cost does seem a bit too high
here. I am not convinced this is worth it." State: `superseded` by
`thread@record-search-verdicts@search-reads-the-draft`. Section: "Losing alternatives".

### A lighter pass for untouched Components is not taken `##lighter-pass-untouched`

Raised by the session in round 1 as its reading of the owner's fourth idea
(`argument@record-search-verdicts@arg-14`), and not proposed. Its issue's re-entry point was met by
the first search (`argument@record-search-verdicts@arg-38`); the owner, round 14, of the session's
default to keep the whole-record search and leave the issue open: "Agreed on the other items". State:
`approved`. Section: "What the work is", outside the work. Harvest: the re-entry point of
`issue@agent-skills@a-lighter-pass-for-untouched-components`.

### A partitioned procedure of the intake `##intake-partitioned-procedure`

Raised by the owner, round 1 (`argument@record-search-verdicts@arg-3`) and round 2
(`argument@record-search-verdicts@arg-18`); parked by the session in round 2
(`argument@record-search-verdicts@arg-24`). State: `parked`. Cause: the next intake the owner runs
still loads its session beyond one reading. Re-entry point: the retrospective of that intake. Its
durable home is `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`,
which this work opens.

### Where the judgement of a dispatch is stated `##dispatch-judgement-home`

Raised by the owner, round 8 (`argument@record-search-verdicts@arg-30`). State: `parked`, outside this
work. Cause and re-entry point: the work that closes
`issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, which holds
the owner's leads.

## Arguments

### Too many entries graded bearing `##arg-1`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@occasion-shaped-output`
- "the search is often leading to far too many entries that are rated bear. Nearly all of the record is often rated bear or may bear. This results in loads that are far too much for one session."

### More searchers because goals grew `##arg-2`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@component-packing`
- "since the goals have started becoming fuller, especially with the component split that led to more goals overall, much more searchers now need to be dispatched."

### The search fits the design skill less than the audit and the intake `##arg-3`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@intake-partitioned-procedure`
- "I think that the design search is relatively well suited for the design skill, especially now that it runs later. But I think it is less adapted to design audit, and especially to the intake."
- "The intake may have a deeper problem in itself, mostly because its nature has it handle lots of unrelated small problems, hence it often ends up covering the full design record."

### Richer verdicts than bears or may-bear `##arg-4`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@verdict-report`
- "I think the search would be more valuable if the results were more detailed than bear or maybear. Since we are dispactching all of these subagents, we could ask them to give stronger opinions and conclucsions on what actions must be taken related to the read entries."

### Pack by Component, give only enclosing goals `##arg-5`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@lighter-pass-untouched`
- "packing entries in a way that reduce the count of spanned component for each searcher, and give only the goals of components that are in parents of the entries a searcher got."

### The main session as orchestrator `##arg-6`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@orchestrating-session`, `thread@record-search-verdicts@verdict-report`
- "I would like to move the installed workfolw toward something were the main session acts more like an orchestrator, dispatching subagents to work or analyze or review, then collecting results and reports and acting one them (with double checking)."
- "we need to move away from the curretn shape that requires the main session to read the full bear entries. This probably goes with giving the searcher's words more weights."

### Measurement of the 25-agent work list `##arg-7`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@component-packing`
- Source: `cargo klarch worklist --out` over main at 350d4d59, a script over the agents' `entries.md`; `wc -w` on goals homes.
- "`cargo klarch worklist --out` over main now gives **25 agents**." "total words the 25 searchers read | 497,444"; "of which goal entries | 181,458 (36%)"; "all 25 root goals are in all 25 agents"; "Components spanned per agent | 2 to 7"; "goals homes, `wc -w` | 17,886 words; the root's alone 8,544".
- "**About a third of every searcher's load is the same goals, copied 25 times.** That comes from `goals-join-a-cluster-as-context` (`design@query@goal-context-once`)"
- "about 85,000 words of issues and `rejected` entries sit in more than one cluster."

### The whole reading stands on the owner's earlier ask only `##arg-8`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@orchestrating-session`
- "The goal does not require the session itself to read. Its text says the bearing parts "are read against the work by a step the installed text owes **an agent or a reviewer**". So the whole reading stands on your earlier ask alone, and your message today changes it."
- "`primer@verify-before-relying` treats a report as a lead, never as evidence. So "giving the searcher's words more weight" has to mean words the session can check cheaply, not words it trusts."

### Goal titles plus one goal searcher `##arg-9`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@goal-context-once`
- "Cluster searchers get each goal as its title line only, about 20 to 40 words. A title states the goal. One extra **goal searcher** reads every goals home whole (17,886 words, one agent under the bound) and grades every goal against the work."
- Estimate: "232,000 words of distinct entries plus shared duplicates, at 25,000 per agent, gives about 13 cluster agents plus one."
- Rival: "your idea of giving each searcher only the goals of the Components its entries sit in. It still copies the root's 8,544 words into every agent, because every Component goal refines a root goal."

### Packing by Component gains coherence `##arg-10`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@component-packing`
- "The gain is coherence more than words. A searcher holding one Component's whole design home grades each head against its neighbours, so a contradiction or a "may change" between two heads of the same Component is seen by one reader."
- Cost: "a cluster with edges into two Components, such as an issue citing heads in `check` and `model`, still goes to one agent."

### The quoted verdict with a closed action `##arg-11`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@verdict-report`
- "**`quote` is verbatim, and `klarch worklist --verify` checks it** against the snapshot `shown.md` it already holds. A paraphrase cannot pass as a quotation."
- "**The session reads whole only the entries whose action changes the work**: a conflict, a design brought back, a firing, a closing, a dependency."
- Measured precedent: "about 30 of 202 entries changed a proposal (commit "[harvest] The graded entries' assembly, `klarch id`, …")."

### May-bear removed `##arg-12`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@verdict-report`
- "**`may-bear` goes away as a grade.** A doubt becomes a `conflicts` verdict whose `against` names the element of the work."
- Record cited: "80 of the 137 entries graded `may-bear` alone gave as reason that no bearing was established" (`rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`).
- Cost: "The searcher's judgement becomes load-bearing for entries graded `constrains` and `unrelated`." "the five entries missed in `tripwire@agent-skills@record-search-misses` were all graded so."
- Rival: "Rival 1 of the open issue: fresh readers over parts, each writing one line per entry. It is the same trust, with a second set of agents and no quote check."

### Output partitioned by the unit of the work `##arg-13`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@slice-audit-checks-fit`
- "Its work is N unrelated findings, and today the union of their neighbourhoods lands on one session. Recorded instances: 333 `bears` plus 249 `may-bear` entries for the forty-file intake."
- "seed with the slice's decisions only, read from the plan documents by a `worklist --seeds-from <plan>` option. Recorded instance: "about 110 of 125 `may-bear` entries graded so for citing a seed" in thaum."

### The lighter pass not proposed `##arg-14`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@lighter-pass-untouched`
- "Taking out the goal duplication roughly halves the agent count without dropping any entry from a full reading, so the case for a lighter pass is weaker than when it was parked."

### Orchestration is an outcome, so a goal `##arg-15`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@orchestrating-session`
- "This states an outcome, not a shape. Per `primer@design-heads`, it lands as a **goal**, drafted under `skill@knowledge-architect-goal-setting`. Once written, it ranks shapes that no goal ranks today."

### Raise the bound; reliability and cost `##arg-16`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@goal-context-once`
- "I would also consider raising the word cap, say to 30k. My observation is that searchers right now go up to around 130k to 150k tokens."
- "So the main metric to consider is actually cost, which is split between flat upfront costs and agent need to start [...] versus increasing costs of each round, which are reduced due to prompt caching."
- "token usage starts to ramp up more noticeably around 250k tokens. So I think we can aim for more content per searcher, and more detailed conclusions, as long as these searchers end their tasks around 250k tokens."

### An issue judged without its Component's design is a defect `##arg-17`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@component-packing`
- "an agent that ends up judging an issue of a component of which it saw no design, would be de major defect IMO. The searcher would need to reason on very little info to judge, or to go read the related design. I think this was a major miss in the current design."

### The intake needs more than per-unit files `##arg-18`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@intake-partitioned-procedure`
- "For the intake, I believe a significantly different procedure, with more steps to partition the work, might be neeeded. But today's scope shuld stay relatively limited, so we can go with that at first, and see how the situation improved the next time I run an intake."

### The slice audit verifies fit, not redesign `##arg-19`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@slice-audit-checks-fit`
- "the audit is much less needed when the spec was recently designed. It is important when work has landed between the design and the satrt of the work. I think it shoudl keep being done before every slice, but the important point is that it should not put a lot of weight on the main session"
- "it should not put a lot of weight on the main session, especially when the findings are low and easy to fix, or even require basicall no action. The goal is to verify that the spec fits in the curent design and code (and fix if it does not), not to redo the work the design session did."

### Draft the goal in this session `##arg-20`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@orchestrating-session`
- "I think we should draft it as a goal in this session, so that it can shape the direction of the discussion more reliably."

### Token fit over 40 transcripts; bound of 50,000 proposed `##arg-21`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@bound-raise`
- Source: token usage of the 40 most recent `agent@knowledge-architect-record-searcher` transcripts, all from 2026-11-07; "Peak context" is input plus cache plus output on the last turn.
- "The fit is about **36,000 tokens of fixed start plus 3.3 tokens per word read**. These runs peak lower than the 130k to 150k you saw."
- "**I propose a bound of 50,000 words**, which predicts about 200k tokens plus the verdict output: under 250k, with margin. Your 30,000 is the conservative rival. It predicts about 135k tokens and keeps the agent count about 1.7 times higher."
- "With goals as titles (`thread@record-search-verdicts@goal-context-once`), 50,000 words gives about 6 to 8 agents over this repository, against 25 today."

### 100 of 341 issue placements without their design `##arg-22`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@component-packing`
- Source: the 25-agent work list.
- "of 341 placements of an issue or a tripwire, **100 sit in an agent that holds no design entry of their own Component**. That's 99 issues and 1 tripwire. The largest counts are the root (22), `check` (19) and `model` (15)."
- "**an agent that holds any entry of a Component holds that Component's whole design home** [...] Applied directly, this conflicts with the bound for `agent-skills` (115,616 words of entries)."

### The audit's narrower question and shape `##arg-23`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@slice-audit-checks-fit`
- "The audit's question is narrower than the design's: **does the spec still fit the record and the code as they stand now?**"
- "the whole record is still read, every cluster by some searcher, so this does not bring back `rejected@agent-skills@delta-search-at-convergence`. [...] Here `--since` is a mark that draws attention, not a filter."
- "An audit with no gap costs it one count line, so the case you described, a spec designed recently, is cheap without any rule about when to skip."
- "**I recommend the generated lines**: the reason stands, and once a command writes the lines, their cost is message length only." Recorded count: "365 lines and 45 checker findings for one slice".

### The intake procedure parked `##arg-24`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@intake-partitioned-procedure`, `thread@record-search-verdicts@occasion-shaped-output`
- "The fuller intake procedure is parked as `thread@record-search-verdicts@intake-partitioned-procedure`. Its cause: the next intake you run still loads its session beyond one reading. Its re-entry: that intake's retrospective."

### 40k as a middle step `##arg-25`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@bound-raise`
- "I think we can go with a middle ground of 40k first. Because we are also demanding more detailed results of searchers, I think we can raise incrementally, rather too much at once."

### One agent per Component as the stronger rival `##arg-26`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@component-packing`
- "with the solution you are proposing, we might as well just drop the packing, and simply dispatch one agent per component. I think most issues are actually fine to judge without the design. The hard case is one issue that cites a design head. But I believe that case is already packed with others actually ?"
- "Maybe with a split of the component once volume goes to high, since one component can actually end up very big even under the split rules. Such as agent skills, which has a single subject, but can end up packing lots of design"

### This session's search need not be read whole `##arg-27`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@verdict-report`
- "For the current session, I think we can try to apply a different procedure. Such as giving more isntructions to the search agents, maybe even trying out the verdic shape you proposed, even if we have to bypass the checked verification. I don't think it is worth it to have you read the whole bear entries."

### F3, the new word replaces the earlier ask `##arg-28`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@orchestrating-session`
- "F3: agreed, my new word replaces the previous one. I've had a change of mind of what I expect of the main session, the goal we are drafting carries my new intent."
- "F2 design only"

### Trust in the main session equals trust in an agent `##arg-29`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@orchestrating-session`, `thread@record-search-verdicts@verdict-report`
- "my take is that the main session is just as reliable as any dispatched agent, so there is no need to give more trust to a subagent. [...] So the reauirements for the main session to verify the subagent work should not  be stronger (or weaker) than the requirements of the main session to verify its own claims."

### How to judge a dispatch `##arg-30`

Round 8, the owner, at L412.

- Threads: `thread@record-search-verdicts@dispatch-judgement-home`
- "Is the next coherent piece of work large enough that it would spend too much of this sessions context and make finishing the complete piece of work at hand (the slice) difficult ? Is there a need to investigate a subject were only the conclusions matter, or write a large code probe ? Is there a need for a fresh point of view ?"
- "Maybe we should add it to the primer, that would probably make the goal met in good part."

### The new goal makes the load criterion binding; reading only what it reasons over `##arg-31`

Round 11, the session, at L611.

- Threads: `thread@record-search-verdicts@orchestrating-session`, `thread@record-search-verdicts@verdict-report`
- "The weighed criterion #c-session-load-bounded becomes **binding**"
- "So the session reads an entry whole only where it must itself reason over the entry, for instance to reverse it. Reading an entry just because the work changes because of it is no longer required."

### An issue citing a head is packed with it; the Component as unit `##arg-32`

Round 11, the session, at L611.

- Threads: `thread@record-search-verdicts@component-packing`
- "**Yes.** `design@query@cluster-rooted-at-a-subject` puts an issue in the cluster of every head it cites. So the case you called hard is covered today, but only by the cited head, not by the rest of its Component's design."
- "This answers the reason behind `rejected@query@one-search-agent-per-anchor`, "an anchor bounds nothing", because the split above the bound restores the bound."
- Measurement (session's script over today's tree, bound 30,000): "9 agents"; "`agent-skills` in 4 agents, `check` in 2"; "read-only context heads per agent from 7 to 76"; "the largest agent at 56,000 words, context included."
- "With a fixed start of about 36,000 tokens per agent, eight agents for Components of 464 to 4,800 words each would spend more on starting than on reading."

### Half-bound rule `##arg-33`

Round 12, the owner, at L613.

- Threads: `thread@record-search-verdicts@component-packing`
- "if a component exceeds half of the bound, always alone (and split if it  exceeds the bound). Only components below half of the bound are packed together. Large components are likely to be more critical to the project, so splitting attention is not optimal IMO. And their entries probably have lots of interactions together."
- "This is a bit arbitrary though, I have no measurement to back that idea."

### Half the bound is the one point that does only what is intended; cost measured `##arg-34`

Round 12, the session, at L621.

- Threads: `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@bound-raise`
- Source: a script over the entries of the first, 25-agent work list, packing whole Components at 40,000 words, run in the session and not kept.
- "Two Components that each exceed half the bound can never share an agent anyway, since together they exceed the bound. So at exactly half, your rule adds a single constraint: no small Component joins a large one."
- Measurement over today's tree at 40,000 words: "agent-skills | 115,191"; "check | 37,185"; "model | 26,742"; "knowledge-architect (root) | 19,838"; "the 10 others | 463 to 7,571 each, 31,511 in all"; "**Your rule: 7 agents.**" against "Small Components packed into the free room of large ones: 6 agents."
- "Nothing ranks the two shapes beyond that, so this is a fork, and the record will say you chose it over the rival at a cost of one agent per search here."
- "**One refinement of mine: the read-only context heads count toward the bound.** In the prototype, `check` alone carried 39 context heads from other Components."

### Searchers' returns during the first search `##arg-35`

Round 13, the session, at L707 to L806.

- Threads: `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@occasion-shaped-output`
- L707: "`--since` would add a second use of `git` in `query`, which `design@knowledge-architect@query-crate` words as if it had only one."
- L724: "`thread@record-search-verdicts@component-packing` says nothing about where a location's entries go, or those of a declared register's entry anchors."
- L751: "**`conflicts`** on `rejected@agent-skills@one-agent-reads-every-standing-entry`: the single goal searcher's load grows with the goals homes."
- L765: "Under `design@agent-skills@synthetic-evidence-not-built`, a run built to observe how agents follow an instruction may not choose a shape. [...] I'll use only its mechanics: agent counts, words per agent, peak tokens, and whether the quotation check works."
- L780: "**`--since` should mark changed entries only, not code.** Marking code would read beyond the record that `design@query@record-is-every-register-entry` defines."
- L806: "Of 54 goals it graded 25 `constrains`, 3 `conflicts` and 26 `unrelated`."

### Mechanics and token fit of the first search `##arg-36`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@verdict-report`
- Source: the searchers' reports, the session's quotation-check script, the searchers' transcripts.
- "7 cluster searchers plus 1 goal searcher, against 25 under today's work list." "911 entries, each graded by exactly one searcher." "Verdicts: 794 `unrelated`, 74 `constrains`, 18 `conflicts`, 16 `touches`, 5 `closes`, 2 `worsens`, 1 `brings-back`, 1 `depends-on`." "0 findings over 911 lines."
- "That fits about **68,000 tokens of fixed start plus 3.4 per word**. [...] At 40,000 words the peak is about 200,000, under your 250,000. A step to 50,000 would predict about 240,000, near the limit, so 40,000 holds and the next raise should be small."

### A goal rules out the per-unit mapping (later withdrawn) `##arg-37`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@goal-context-once`
- "`goal@agent-skills@only-needed-instructions` refuses "a fixed sequence, count or mapping between a unit of the work and a unit of its record that another instruction needs to vary". One verdict file per finding cluster or per thread is such a mapping."
- "**3. `thread@record-search-verdicts@goal-context-once`: "one goal searcher" is a fixed count.**" "**Revised:** goals are packed under the same bound, by Component, in as many goal searchers as that takes."

### Locations, conformance replacement, trigger-met, routed actions, audit fixes, lighter pass `##arg-38`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@lighter-pass-untouched`
- "**Rule:** an anchor that is no Component counts in the nearest Component that encloses its path."
- "Once that list is gone, the reviewer reads the verdict file whole and judges the work against every verdict that is not `unrelated`."
- "**`trigger-met`** extends to the re-entry point of a `design` issue, not only a deferred trigger."
- "[the action then named after the reversal check, since renamed `check-reversal`] goes to the reversal check, which reads what cites the decision. That keeps `goal@knowledge-architect@records-reach-their-reader`, whose "requires a session to read what cites a decision before it reverses it" names no main session."
- "`--seeds-from` takes the spec's design entries and declared-register entries, no goal and no `rejected` entry."
- "The cost argument is gone here: 8 agents read the whole record at about 200,000 tokens each. But a project of about 30 Components hasn't been measured."

### Premortem causes `##arg-39`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@bound-raise`
- "a searcher grades `unrelated` an entry that bears, and no reader reads `unrelated` entries"; "a quotation is verbatim but not the sentence that decides, so the check passes on a wrong verdict"; "a cluster searcher misjudges a head's constraint from goal titles alone"; "a Component split above the bound separates two subjects that interact"; "in a project with longer entries, 40,000 words exceeds 250,000 tokens"

### `goal@agent-skills@only-needed-instructions` misread `##arg-40`

Round 14, the owner, at L848.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@goal-context-once`
- "the refusal you mentionned is about what "the work does not need". If we judge the work needs it, that is allowed."
- "this clause was intended to apply mostly to the design procedure, were creativity is important, and structure could hurt it. But the semi-mechanical part of a record search should not be affected."
- "The last reason for this clause is that imposing rigid structure everywhere would create lots of instructions from which deviation would happen often. If the structure can be enforced by a mechanical check, that clause should not apply, because deviation will be corrected instantly."
- "It should not apply to a mechanical procedure. It applies to agentic instructions."

### Each location its own unit `##arg-41`

Round 14, the owner, at L848.

- Threads: `thread@record-search-verdicts@component-packing`
- "I think each location should be treated as one small component (or not necessarily small) for that. If you look at thaum (../games/thaum), it contains a rule interpretation location, zhich is quite big, and probably needs to get its own searcher(s)."
- "goal-context-once: agreed to pack under the same bound, by component. This is clearly needed if goal size exceeds the bound"

### The goal read whole; first shape wins minus one sentence `##arg-42`

Round 14, the session, at L873.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@goal-context-once`
- "Its measure is "each installed instruction". So it constrains instructions in the installed text, and a structure that an instruction would need to vary."
- "The number of goal searchers is computed by the command from the bound. That is no instruction, and no instruction needs to vary it."
- "**So the first shape wins, minus that one sentence.** `--verify` writes one verdict file per unit the brief names: per thread, per finding cluster, or the slice."

### Locations as units; thaum measured `##arg-43`

Round 14, the session, at L873.

- Threads: `thread@record-search-verdicts@component-packing`
- Source: thaum's manifest; `wc -w` over `path@elsewhere@docs/rules`.
- "The packing unit becomes **every anchor that carries registers: a Component or a location**, each treated alike."
- "`rules`, at `path@elsewhere@docs/rules`, carrying `issue`, `tripwire` and `interpretation`. Its Markdown holds 83,376 words by `wc -w`. That is an upper bound on its entries"

### Can the searcher be trusted; context or model? `##arg-44`

Round 15, the owner, at L876.

- Threads: `thread@record-search-verdicts@verdict-report`
- "Right now, it looks like we cannot trust the record searcher as much as I thought. But maybe the difference lies in what instructions the searcher has in context. For example, you held the design skill in context. [...] Or maybe it is about the model. Which one did it use ?"

### A conditional verdict consumed as unconditional `##arg-45`

Round 15, the session, at L900.

- Threads: `thread@record-search-verdicts@verdict-report`
- Source: the `model` field of the searchers' transcripts; grep over the reports.
- "**Mostly mine, not the searchers'.** Within their conditions, the searchers were largely right"
- "**I dropped the conditions.** Three "under the reading that…" verdicts became one unconditional sentence" "**I moved the objection.**" "**I ruled without reading the entry.**"
- "**Model:** all eight searchers ran `claude-opus-5-5`, the same model as this session, by the `model` field of their transcripts."
- "By grep, 8 of the 18 `conflicts` verdicts from the cluster searchers mention a reading in their reason, and all 3 from the goal searcher do."
- "That premise holds only if the line between the two cases is visible, and a reason in prose doesn't make it visible." Proposal: "a conditional verdict as a field of its own" with action `resolve-reading` (withdrawn in round 16).
- "A shape worded vaguely produces conditional verdicts."

### Give searchers the design context; parity rather than conditional verdicts `##arg-46`

Round 16, the owner, at L910.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@search-work-carries-the-deliberation`
- "the main design session holds detailed info on the design, which the searchers do not. Maybe feeding them with the extracted transcript of the design discussion could help, if the size is reasonnable"
- "I think the best shape would be to give the dispatched agents a level of reading that is on par with what the main session holds, so that they can reach conclusion that are similar to the ones of the main session, without needing a conditional. The only advantage the main session keeps is that it can ask me for a goal reading, or for a fork."

### Transcript cost; parity; `owner-question` `##arg-47`

Round 16, the session, at L939.

- Threads: `thread@record-search-verdicts@search-work-carries-the-deliberation`, `thread@record-search-verdicts@verdict-report`
- Source: `klarch transcript show` over this session.
- "owner | 3,087"; "session | 15,908"; "injected (skills, reminders, hand-backs) | 4,946"; "about 65,000 tokens per searcher at the 3.4 tokens per word measured in today's run."
- "40,000 words of entries plus 19,000 of transcript predicts about 270,000 tokens at peak, over your 250,000." "this repository needs about 12 agents instead of 8."
- "**A raw transcript holds superseded positions.** [...] So the brief gives **the latest delta or checkpoint as the authority on each thread's state**"
- "So I withdraw the `if` field." "**What remains is a question for you**, as an `owner-question` field"
- "So point 1 changes the design skill's grounding alone."

### Transcript cost too high; plan before the search? `##arg-48`

Round 17, the owner, at L942.

- Threads: `thread@record-search-verdicts@search-work-carries-the-deliberation`, `thread@record-search-verdicts@search-reads-the-draft`
- "the cost does seem a bit too high here. I am not convinced this is worth it. And you are right that the transcript holds information that is not necessarily useful."
- "Maybe the best shape would be to have a plan document already written, but this is a shape we rejected long ago because we wanted to have a search that runs before the plan writing. But maybe it is truly more valuable to have that search run as part of the plan review."
- "The search before the plan is starting to look like it has more flaws, for now, to me."

### The search reads the draft `##arg-49`

Round 17, the session, at L971.

- Threads: `thread@record-search-verdicts@search-reads-the-draft`, `thread@record-search-verdicts@search-work-carries-the-deliberation`
- "**the search reads the draft of the document that will carry the design**, written when convergence is near, and still before any thread closes."
- "**That reason no longer holds.** `design@agent-skills@no-thread-closes-before-the-search` came after it" (against `rejected@agent-skills@conformance-reader-at-the-checkpoint`).
- "a work statement written for the search (today) | about 1,500 words, written only for the search | it compresses: it said "one extra goal searcher" where the discussion meant a count the bound decides, and three verdicts turned on that wording" (this argument from the prototype run was removed in round 18, then reinstated by the owner's ruling in round 19, per L1282).
- "written anyway, since the main session writes the plan under `goal@agent-skills@session-orchestrates-the-work`"
- "**A draft written before the search can need revising after it.** It's a document, so a revision costs an edit, never rework of code."

### Returns of the search on the draft `##arg-50`

Round 18, the session, at L1075 to L1187.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@search-reads-the-draft`, `thread@record-search-verdicts@lighter-pass-untouched`
- L1075: "**The bound** is 37,400 words of entries per agent, leaving room for the draft's 2,508 words inside your 40,000."
- L1101: "the real brief must apply the rule to decisions only, never to goals" (`rejected@agent-skills@goal-seeds`).
- L1129: "So "each Component and each location" would make every one of those its own unit, against the draft's own rule. The thread will say "each location the manifest declares""
- L1140: "The verdict file is added to that reading, not put in its place." "today the list of unread entries is what triggers the conformance axis at the merge of an in-change or bounded branch. Once the list goes, that trigger needs a replacement."
- L1158: "The field takes every question that section reserves to you." "the draft's figures, such as "100 of 341" and "about 200,000 tokens", must not go into the heads as numbers."
- L1176: "A wrong `unrelated`, which is the premortem's first cause, then leaves nothing to check [...] The fix: every `unrelated` line carries a one-line reason." "about 800 `unrelated` lines per search here. At about 15 words a line that is about 1,500 words per agent."
- L1187: "So the draft must not "become the spec", as my Status section says. The draft is the search's work only. The spec is still assembled from the transcript afterwards." "the design skill writes no plan document. So the draft is written under the planning skill's sections"

### Result of the search on the draft `##arg-51`

Round 18, the session, at L1270.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@search-reads-the-draft`
- Source: the reports, the session's quotation-check script, the transcripts, the session's packing script.
- "912 entries, each graded once, and 54 goals." "Peaks ran from 139,098 to 210,556 tokens, all under your 250,000."
- "The packing unit becomes a **search unit**, and a unit of the work a **subject of the work**."
- "a standing entry that cites a head of another search unit gets that head's **full text**, counted in the bound. [...] This costs **12 agents instead of 9** over this repository, by my script."
- "the evenly spread packing of small search units stays, or `rejected@query@packing-to-the-bound` would come back."
- "A `conflicts` verdict on a goal goes to goal-setting, never to the reversal check."
- "`--seeds-from` reads the slice's own spec, not the milestone document."
- "The thread now rests on two arguments: the draft is written anyway, and a separate work statement would be a second copy of the design, which `primer@where-knowledge-goes` refuses."

### The prototype run is real use `##arg-52`

Round 19, the owner, at L1272.

- Threads: `thread@record-search-verdicts@search-reads-the-draft`, `thread@record-search-verdicts@bound-raise`
- "For the question, I would count that as real use."

### Scope of the real-use ruling; spec or milestone `##arg-53`

Round 19, the session, at L1282.

- Threads: `thread@record-search-verdicts@search-reads-the-draft`
- "a record search run in a real design discussion, under a planned instruction's shape, counts as real use. [...] Your word was about our search, not a census."
- "The prototype argument then comes back into `thread@record-search-verdicts@search-reads-the-draft`: the first search's compressed work statement is evidence again."
- "**D1, spec or milestone** [...] **Default: one spec on one branch.** Between the `query` steps and the text step, the installed text would name files the command no longer writes."

## New names, in one place

```text
crates/cli/src/cli/mod.rs     DEFAULT_WORKLIST_BOUND     existing; 25000 becomes 40000
                              WorklistArgs               existing; gains since, seeds_from, work, subjects
                              --since <commit>           new option of worklist --out
                              --seeds-from <spec>        new option of worklist --out
                              --work <path>              new option of worklist --out, repeatable
                              --subject <id>             new option of worklist --out, repeatable
crates/query/src/lib.rs       WorklistMode               existing; Write gains the same fields
crates/query/src/graph.rs     graph, pack                existing; search units, context entries
crates/query/src/worklist.rs  relations                  existing; becomes the verdicts per register
                              parse_fields, judge        existing; read and judge the verdict line
crates/git/src/git.rs         paths_changed_since        new: the paths a commit and the work tree differ by
agent directory               entries.md                 gains "Context entries" and "Goal titles"
                              report.md                  verdict lines, then "## Met outside the task", "## Decided"
work list directory           agents.txt                 gains the changed-since mark
                              work.txt                   new: the work's paths, the subjects, the audit's plan and commit
                              verdicts-<subject>.md      new, one per subject; replaces bears.md, may-bear.md
                              gaps.md, fit-lines.md      new, at a design audit
                              generated-lines.md         new: the per-entry lines a commit message owes
report fields                 verdict, against, action,  new, replacing grade and relation
                              owner-question, quote
verdict and action words      as "Mapping tables" lists them
```

## Decided design

### Search units, and what an agent holds

Every Component and every location the manifest declares is a search unit. An entry belongs to the
search unit that holds it; a location the tool constructs, a plan or the entry anchor of a directory
register, belongs to the search unit that holds its home. Clusters are built inside a search unit, by
the rule of `design@query@cluster-rooted-at-a-subject` over every kind of unit `unit_key` gives, from
the unit's own entries: a standing entry of the search unit joins the cluster of each head of that
unit it cites. An entry a cluster's entry cites that the agent holding the cluster does not grade, a
head or a `rejected` entry, of another search unit or of another agent of a split one, is a context
entry of that agent: given whole, read only, not graded there, and named by no verdict line. The goals
reach a cluster agent as their title lines, the heading's statement without its slug: every goal of the
root, of each Component enclosing a search unit the agent holds, and of those search units. An entry no
cluster holds is packed with its search unit, as the unit's remainder; no residual group per anchor
exists. The session still reads every goals home at the design grounding, so both readings of
`design@agent-skills@searcher-grades-goals` stay.

The argument: an issue that cites a head is judged beside that head, which
`design@query@cluster-rooted-at-a-subject` requires, and the owner narrowed the defect of today's
packing to that case (`argument@record-search-verdicts@arg-26`). Nearest rival: context entries by title
only, which judges a citing issue without its head's text; it cost 3 agents fewer over this repository
(`argument@record-search-verdicts@arg-51`). A report names every entry its agent was given to judge,
which is how the owner read "names every entry it was given" in
`goal@agent-skills@session-orchestrates-the-work`.

### An agent's load, and the packing alone above half the bound

The bound left for entries is the bound minus the words of the work. A group's load, the input of
`pack`, is the words of its graded entries, of its context entries and of its title lines; a context
entry or a title line two groups of one agent share is counted in each, so an agent's true load is at
most the sum. A search unit's load is the load of one group holding all of it. A search unit whose
load exceeds half the bound left gets agents of its own: one where its load fits, else its clusters and
its remainder spread over as many agents as the bound needs, first-fit decreasing then spread, as `pack`
spreads today. Search units whose load is at most half the bound left are packed together whole, by the
same rule. Before packing, a standing entry of a split search unit that cites heads of several of its
clusters is assigned to the cluster it cites most, the first such cluster in the unit's order on a tie,
so every entry is graded in exactly one agent. A cluster whose own entries exceed the bound alone goes
to an agent alone, with the notice of today, which weighs the cluster's own entries only. Goals are
packed into goal agents by the same rules, by search unit, each goal agent reading its goals whole with
the prose of each goals home above its first goal.

The argument for the half bound: two search units each above half the bound can never share an agent,
so at half the rule only keeps a small unit out of a large one's agent; the owner chose it, on the
owner's judgement that a large Component's entries probably interact, which no measurement backs
(`argument@record-search-verdicts@arg-33`, `argument@record-search-verdicts@arg-34`). Nearest rival:
small units packed into the room of large ones, one agent fewer over this repository. Reopened, not
built: `rejected@query@one-search-agent-per-anchor`, whose reason, "an anchor bounds nothing",
the split above the bound answers; small units still share agents, since one agent each would spend
more on starting agents than on reading (`argument@record-search-verdicts@arg-32`).

### The bound is 40,000 words

`DEFAULT_WORKLIST_BOUND` is 40000, `--bound` setting another. The owner reported that token usage ramps
up more noticeably around 250,000 tokens of context, and that agents stay reliable past it; the bound
keeps a searcher under that point. Both searches of the discussion, real use on the owner's word,
measured every searcher's peak under 250,000 tokens at this bound
(`argument@record-search-verdicts@arg-36`, `argument@record-search-verdicts@arg-51`). Nearest rivals:
30,000, the owner's first figure, which keeps more agents for the same record; 50,000, the session's,
which the measured fit puts near 240,000 tokens with the verdict output added; the owner chose to raise
by steps. A later raise rests on the peaks of real searches, which the retrospective's question on cost,
per `goal@agent-skills@session-orchestrates-the-work`, reaches.

### The work and the subjects of the work

`worklist --out` takes `--work <path>` for each file of the work (the draft, a spec and its milestone
document, the received files of an intake, a proposed fix) and `--subject <id>` for each subject of the
work, an id in the grammar of an entry id, which `cli` checks by its form while parsing. Without
`--subject`, the one subject is `whole-work`; at a design audit, `slice-audit` or `spec-audit`. `--out`
writes `work.txt`, listing the work's paths, the subjects, and at an audit the spec's path and the
`--since` commit's subject; each agent's `report.md` skeleton names the subjects. `--verify` refuses an
`against` value that `work.txt` does not list, and reads from `work.txt` whether the run is an audit:
it is one where `work.txt` holds a spec's path. The intake's second search, for a proposed fix on a
subject its files did not raise, passes the fix as its `--work`.

### The verdict line

Each line of a report is, in this order, with ` | ` between the fields:

```text
- <reference> | verdict: <verdict> | against: <subject ids, comma-separated, or -> | action: <action> | owner-question: <text, or empty> | quote: <text> | reason: <text>
```

- `quote` runs to the last ` | reason: ` of the line, and `reason` to its end. `--verify` refuses a
  line whose fields are missing or out of this order, an `owner-question` holding ` | quote: `, and a
  `quote` holding ` | reason: `.
- The skeleton line of each entry gives, after `verdict:`, the verdicts its register admits, which the
  searcher replaces by one; the seed marks stay on the line as today.
- The verdicts a register admits, and the actions a verdict admits, are the tables of "Mapping
  tables". `--verify` refuses a verdict or an action outside them.
- The quotation is owed on every line, `unrelated` included: at least eight whitespace-separated
  words, copied verbatim, markdown marks included, from the entry's own body, never from its
  `<reference>  <site>` first line nor from its inbound list. `--verify` compares after collapsing every
  run of whitespace to one space, and refuses a quotation it does not find. For an `unrelated` entry,
  the quotation is the sentence that shows the entry's subject, which the reason contrasts with the
  work; so every conclusion carries evidence checkable without reading the entry whole, as the owner
  read `goal@agent-skills@session-orchestrates-the-work`.
- Every line has a reason, one line: what in the work meets what in the entry, or, for `unrelated`,
  why nothing does. `respect` is given where the quoted sentence limits a shape of the work, `none`
  where nothing in the work is limited by it.
- An entry that is a seed, or cites one, is never `unrelated`: where nothing in the work meets it, it is
  `constrains`, action `none`, `against: -`, its reason naming the seed. `worklist` keeps taking a goal
  as a seed, which the intake's seeds need, per `design@agent-skills@seeds-name-decisions`; the design
  grounding and the design audit pass no goal as a seed, per `rejected@agent-skills@goal-seeds`.
- The searcher reads whole a goal or an entry before a verdict rests on it, running `show` for one it
  holds by title. It resolves every doubt the work and the record answer, and its verdict states its
  conclusion, not a condition. What `primer@question-authority` reserves to the owner goes in
  `owner-question`: the readings, and what each changes in the work.
- The report closes with `## Decided`, the searcher's decisions the owner could have made, then
  `## Met outside the task`, in the order `primer@decided-section` gives; `--verify` refuses a report
  missing either. The main session carries both into its own sections.

The argument: the size of the reading the grades owed
(`the-search-s-graded-entries-exceed-what-one-session-reads-whole`), and the goal's
check parity: the searcher checks its conclusions as the main session checks its own claims, and the
command checks what a command can. Nearest rival: the conditional `if:` field, withdrawn for parity of
context (`argument@record-search-verdicts@arg-46`, `argument@record-search-verdicts@arg-47`). The bound
of eight words is a default awaiting the owner, D2.

### What `--verify` writes

`--out` writes `shown.md` with each entry's own text and its inbound list as two separate fenced
blocks, so `--verify`, which reads no model, compares a quotation with the first alone. When every
report passes, `--verify` writes:

- one verdict file per subject, `verdicts-<subject>.md`: every verdict but `unrelated` whose `against`
  names the subject, with its entry's text;
- outside an audit, `generated-lines.md`: one line per entry given a verdict other than `unrelated`,
  its reference, its verdict and its action, with no quotation and no reason, in forms
  `cargo klarch commits` judges, for the commit message the occasion owes;
- at an audit, `gaps.md` and `fit-lines.md`, as "The design audit checks fit" says, in place of
  `generated-lines.md`.

It removes each of these files left by a previous run before judging, as it removes `bears.md` today.
The shapes of `work.txt`, the files' lines and the changed-since mark are the implementer's, since these
files are read by `--verify`, the searcher and the session only, per
`design@query@worklist-writes-and-verifies-the-reports`; the implementer records them in the code's
comments. The main session divides the work among agents over the verdict files at its own judgement,
per `goal@agent-skills@session-orchestrates-the-work`, and acts on the verdicts, checked as its own
claims are, with no re-reading by rule. The argument for files per subject: a session that dispatches
one agent per subject hands each its own file. Nearest rival: one file with a filter option, withdrawn,
its ground a misreading of `goal@agent-skills@only-needed-instructions`
(`argument@record-search-verdicts@arg-42`).

### The design audit checks fit

At the design audit of a slice or a spec, the work is the slice's spec and the milestone document, or
the spec, passed by `--work`, with the subject `slice-audit` or `spec-audit`. `--seeds-from <spec>`
takes the path of the spec, and adds as seeds the design entries and the declared-register entries its
section "What is already decided" cites; for a slice, its own spec, not the milestone document. It
combines with `--seed`, and is refused where the path is no plan document. `--since <commit>` marks,
in `agents.txt` and on each entry's `===` line in `entries.md`, the entries whose file changed between
that commit and the work tree, an untracked or renamed file included, through `paths_changed_since` in
`git`, which fails with git's reason where git fails, per `design@git@failed-git-is-an-error`: a mark,
not a filter, and no mark on code. The audit passes the commit that last changed its spec.

At an audit, `--verify` writes `gaps.md`, every verdict other than `constrains` and `unrelated` with its
entry's text, and `fit-lines.md`, the line of each entry given a verdict other than `unrelated`, the
spec cited whole and the commit named by its subject. A `constrains` verdict is a fit; every other
verdict but `unrelated` is a gap, which the main session sorts into the planning skill's four sorts of
a gap, as it does today. The main session reads the gaps file. The argument: the owner's aim that the
audit verify fit before every slice and weigh little on the main session
(`argument@record-search-verdicts@arg-19`), and the lines an audit's message owed by hand, which
`the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion` counts.
Nearest rival: the audit's whole reading of both files today, lost on that count.

### The search reads the draft

At the design grounding, the search runs once the shapes settle and before any thread closes, with the
draft as its work, passed by `--work`, and the threads as its subjects. The draft's form follows the
path the session expects; the classification after the search still decides the path, as
`design@agent-skills@bounded-path-in-design` says. On the full path, the planning skill starts there:
it writes the draft, in the session's scratch space, with the sections of a spec that carry the design
(what the work is, what is already decided, the threads as they stand, the decided design as proposed,
the losing alternatives), under less strict constraints than a plan document, since the checker reads
no scratch file. On the in-change path, the design skill writes the draft of the commit message, as it
holds that deliberation today. The findings revise the draft; the premortem and the convergence follow;
on the full path the plan document is then assembled from the transcript, per
`design@agent-skills@ledger-from-transcript`, never from the draft. A thread that opens after the search
on a subject its work did not cover gets a search of its own, with the draft extended by that thread as
the work. The draft is no ledger in the sense of `rejected@agent-skills@ledger-written-during-discussion`:
it is written once, read at once by the search it is written for, extended only for a later thread's
own search, and never read as the discussion's state or used to assemble the plan.

On the in-change path and for bounded work, the commit message carries the generated lines, and the
design-conformance axis is owed at the merge of a branch whose commit message carries generated lines
with a verdict other than `constrains` and `unrelated`. The design-conformance review reads the plan
against the goals, design and rejected homes, as `design@agent-skills@plan-read-against-the-record`
says, and reads the verdict files beside them.

The argument: the draft is written anyway; a work statement written apart from it is a second copy of
the design, which `primer@where-knowledge-goes` refuses, and the first search's work statement compressed
the design (`argument@record-search-verdicts@arg-49`), evidence admitted as real use on the owner's
word; the transcript would cost more agents, an estimate of about 12 instead of 8 over this repository
from the token fit (`argument@record-search-verdicts@arg-47`). The reason of
`rejected@agent-skills@conformance-reader-at-the-checkpoint`, rulings already given, is defeated by
`design@agent-skills@no-thread-closes-before-the-search`, which holds every closure until the search
has run.

## Mapping tables

The verdicts each register admits:

| register | verdicts |
| --- | --- |
| `goal` | `constrains`, `conflicts`, `unrelated` |
| `design`, and a register the project declares | `constrains`, `conflicts`, `unrelated` |
| `rejected` | `brings-back`, `constrains`, `unrelated` |
| `issue` | `closes`, `touches`, `worsens`, `depends-on`, `trigger-met`, `constrains`, `unrelated` |
| `tripwire` | `fires`, `constrains`, `unrelated` |

`trigger-met` is the verdict of an issue whose deferred trigger, or whose re-entry point if it is of kind
`design`, the work meets. `constrains` on an issue or a tripwire is admitted without a further check:
it is the verdict of a seed's citer that nothing in the work meets.

The actions each verdict admits, and the procedure each action sends to:

| verdict | actions |
| --- | --- |
| `constrains` | `respect`, `none` |
| `conflicts` on a goal | `goal-setting` |
| `conflicts` on another entry | `check-reversal` |
| `brings-back` | `new-argument-needed` |
| `closes` | `close-with-the-work` |
| `touches`, `worsens` | `respect`, `none` |
| `depends-on`, `fires`, `trigger-met` | `rule-before-work` |
| `unrelated` | `none` |

| action | procedure |
| --- | --- |
| `respect` | the work keeps within the quoted sentence |
| `none` | nothing |
| `check-reversal` | `skill@knowledge-architect-decision-recording@reversal-check`, and the backstop of `primer@design-heads` |
| `goal-setting` | `skill@knowledge-architect-goal-setting` |
| `new-argument-needed` | the Reopening rule of `skill@knowledge-architect-design@threads-and-states` |
| `close-with-the-work` | the work's commit closes the issue, per `skill@knowledge-architect-issue-tracking` |
| `rule-before-work` | ruled before the work is designed further or built: a firing by `skill@knowledge-architect-issue-tracking@movement-instruction`; a dependency, a trigger or a re-entry point in the discussion, by `skill@knowledge-architect-design@discussion-loop` |

`against: -` is admitted on `unrelated`, and on `constrains` with action `none`.

Today's grades and relations, into verdicts:

| today | after |
| --- | --- |
| `bears`, `constrains` | `constrains` |
| `bears`, `may change` | `conflicts` |
| `bears`, `may bring back` | `brings-back` |
| `bears`, `touches`, `closes`, `worsens`, `fires` | the verdict of the same word |
| `bears`, `depends on`, `trigger met` | `depends-on`, `trigger-met` |
| `may-bear` | resolved by the searcher, or a verdict above with an `owner-question` |
| `unrelated` | `unrelated`, with a quotation and a reason |

## Losing alternatives

- Small search units packed into the room of large ones: lost to the half-bound rule, the owner's
  choice at a fork no goal ranks, at a cost of one agent over this repository
  (`argument@record-search-verdicts@arg-34`).
- Context entries by title only: lost to `design@query@cluster-rooted-at-a-subject`; it cost 3 agents
  fewer (`argument@record-search-verdicts@arg-51`).
- One goal searcher: lost to goals homes that can exceed the bound
  (`argument@record-search-verdicts@arg-41`).
- One verdict file with a filter option: withdrawn, its ground a misreading
  (`argument@record-search-verdicts@arg-42`).
- A sentence sending each cluster of findings to one subagent: ruled out by
  `goal@agent-skills@session-orchestrates-the-work`.
- The conditional `if:` field with a `resolve-reading` action: withdrawn by its proposer for parity of
  context (`argument@record-search-verdicts@arg-47`).
- The transcript as the search's work: superseded by
  `thread@record-search-verdicts@search-reads-the-draft`, on its cost
  (`argument@record-search-verdicts@arg-47`).
- A work statement written for the search: lost to the draft, a second copy of the design.
- `--since` marking code: lost to `design@query@record-is-every-register-entry`.
- Gap classes of the search's own: withdrawn at this spec's first review, since the planning skill
  already sorts a gap into four sorts.
- A quotation carried into the generated lines: withdrawn at the first review, since a commit message
  reads a quotation's references as live and the quotation's own marks can end it early.
- `--seed goal@…` refused by the command: withdrawn at the second review, since the intake's seeds name
  goals.
- An `unrelated` line with a reason and no quotation: lost to the owner's reading of
  `goal@agent-skills@session-orchestrates-the-work`, evidence checkable without reading the entry whole.

## Readings

None: the work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| a searcher gives `unrelated` to an entry that bears, and no reader reads `unrelated` entries | `thread@record-search-verdicts@verdict-report` | survives as a named clause: every `unrelated` line carries a quotation and a reason; `tripwire@agent-skills@record-search-misses` watches the misses |
| a quotation is verbatim but not the sentence that decides, so the check passes a wrong verdict | `thread@record-search-verdicts@verdict-report` | named as a cause for the harvest to judge as a tripwire |
| a cluster searcher misjudges a head's constraint from goal titles alone | `thread@record-search-verdicts@goal-context-once` | converted into a named clause: the searcher reads a goal whole before a verdict rests on it |
| a split above the bound separates two subjects that interact | `thread@record-search-verdicts@component-packing` | named as a cause for the harvest to judge |
| in a project with longer entries, 40,000 words exceeds 250,000 tokens, where the cost ramps up | `thread@record-search-verdicts@bound-raise` | named as a cause for the harvest to judge; its evidence is in transcripts |
| a draft written before the search anchors the discussion, so a finding is answered by amending rather than reconsidering | `thread@record-search-verdicts@search-reads-the-draft` | named as a cause for the harvest to judge |

## Acceptance criteria

None. The one proposed, on searchers' peaks at the bound, named no real work to judge it, so judging it
would have been a run built to accept the work; the owner ruled it dropped.

## Implementation sequence

1. **`query`, `cli`: search units and the load.** Builds, in `graph` and `pack`: search units, clusters
   inside them, context entries, goal titles and goal agents, the remainder of a unit, the assignment of
   a split unit's citing entry, the load and the half-bound rule; in `cli`: `--work`, `--subject` checked
   by its form, `work.txt`, `DEFAULT_WORKLIST_BOUND` at 40000. `worklist_mode` refuses the new options
   with `--verify`, at both its calls in `run`, beside the clap `conflicts_with` on each; the tests that
   build `WorklistArgs` field by field (`path@cli@src/cli/mod.rs`) gain the new fields. The doc comments
   that state today's packing change with it: the module doc and the `Group` doc in
   `path@query@src/graph.rs`, and the `Command::Worklist` and `--verify` docs of `cli`; so do the tests
   asserting residual groups (`every_entry_is_in_a_cluster_or_its_anchor_s_residual` in `graph.rs`, and
   two in `path@testing@tests/binary.rs`). Claims, each tested over a mock project in
   `path@testing@tests/binary.rs` beside `worklist_project`, which gains a directory register, a search
   unit above half the bound, a cross-cluster citing entry and a cross-unit citing issue, and each shown
   to fail by `cargo x mutate` on the site it names: every entry is graded in exactly one agent
   (`graph`); no agent's summed load exceeds the bound left unless it holds one cluster alone (`pack`);
   a search unit above half the bound shares no agent (`pack`); a constructed location is packed with
   its home's search unit (`graph`); an issue citing a head another agent holds has it as a context
   entry (`graph`); a cluster agent holds the title lines of the root's and its units' goals, and a goal
   agent every goal of its units (`graph`); the words of `--work` reduce the bound left (`pack`); the
   default bound is 40000 (`DEFAULT_WORKLIST_BOUND`). Fails alone on: the agent count and the loads.
2. **`query`: the verdict report.** Builds the verdict line, the tables of "Mapping tables" in
   `relations`, its parsing in `parse_fields`, the parsing of `agents.txt` in `verify` for the new
   mark, the two fenced blocks of `shown.md` in `snapshot`, the quotation check, the refusals, the
   skeleton in `render`, the verdict files and `generated-lines.md` in `assemble`, and their removal
   before a run. The comments of `assemble`, of `relations`, of the `BEARS_FILE` and `MAY_BEAR_FILE`
   constants and the skeleton text of `render` change with it, and the seed refusal message of `write`
   no longer invites a goal at the occasions that pass none. Claims, each tested over mock reports and
   shown to fail by `cargo x mutate` on the site it names: `--verify` refuses a quotation absent from the
   entry's own body, one taken from its first line or its inbound list, and a line with no quotation
   (`judge`); a verdict outside its register's set and an action outside its verdict's (`judge`); a
   missing reason, a field out of order, an `against` that `work.txt` does not list, and a report
   missing `## Decided` (`parse_fields`, `verify`); a seed or a seed's citer graded `unrelated`
   (`judge`); `verdicts-<subject>.md` holds exactly the verdicts naming that subject, and
   `generated-lines.md` exactly the entries given a verdict other than `unrelated` (`assemble`); a
   failed run leaves none of the outputs of a previous run (`verify`). Fails alone on: what `--verify`
   refuses and writes.
3. **`git`, `query`, `cli`: the audit's options.** Builds `paths_changed_since` in `git`, exported in
   the `pub use` list of `path@git@src/lib.rs`, `--since` and `--seeds-from` in `worklist`, the audit
   mode of `--verify`, `gaps.md` and `fit-lines.md`. Claims, tested over mock projects with histories
   built through `Sandbox::git_command`, and shown to fail by `cargo x mutate` on the site each names:
   `--since` marks exactly the entries of the files changed since the commit, an untracked one included
   (`paths_changed_since`, `write`); a failing git call stops the run with git's reason
   (`paths_changed_since`); `--seeds-from` takes only the design and declared-register entries of the
   spec's "What is already decided" (`write`); `gaps.md` holds every verdict but `constrains` and
   `unrelated`, and `fit-lines.md` cites the spec whole (`assemble`). Fails alone on: the marks, the
   seeds and the audit's outputs.
4. **`agent-skills` and the user-facing text.** Rewrites, in `path@agent-skills@content/`: the searcher
   agent; the design-conformance reviewer agent, which reads the `may-bear` entries not read; the
   design skill's loop step 1, its in-change path and its bounded work; the planning skill's "When this
   skill starts", the row "What is already decided" of `skill@knowledge-architect-planning@spec-contents`,
   its design audit, and the bullet on the conformance reviewer of
   `skill@knowledge-architect-planning@plan-reviews`; the retrospective intake's grounding and its
   analysis section; the review skill's sentence on what the conformance reviewer reads and its axes
   table on that axis's condition; the primer's moment for loading the planning skill; the `%%` lines
   citing a rewritten head; and `path@agent-skills@fragments/subject-split.md`. Also, in
   `path@cli@README.md`, the `worklist` section, the usage block and the exit-code rows that name the
   assembly. Then `klarch install-agent-skills`. Claims: no installed text names `bears.md`,
   `may-bear.md` or a `may-bear` grade, by `grep` over `.claude` and the source; each loop-step citation
   the edits renumber is checked by hand (`issue@check@no-checked-form-cites-a-point-inside-a-section`);
   the installed copies equal the source, by `cargo klarch check`. Fails alone on: the installed copies.
5. **The harvest**, as the section "Harvest" says.

## Order rationale

- 1 before 2: the report is judged over the agents the packing writes.
- 2 before 3: the audit's outputs are written from the verdicts.
- 3 before 4: the text names what the command writes.
- 4 before 5: the harvest records what landed.
- Steps 1 and 3 each touch more than one Component: the options of `worklist` are one interface, which
  `cli` declares and `query` carries out, and `git` gains only the one function step 3 calls, so neither
  step can fail on its `cli` or `git` part alone.

## Defaults awaiting the owner

### D1, one spec on one branch, not a milestone

Ruled by the owner: one spec. Between steps 2 and 4 the installed text would name files the command no
longer writes, so the work lands on one branch; the rival, a milestone of a `query` slice and a text
slice, would need `--verify` to keep writing `bears.md` and `may-bear.md` until the second slice.

### D2, a quotation of at least eight words

Bears on `thread@record-search-verdicts@verdict-report`. The bound has no measurement: shorter
quotations match more than one sentence of an entry, and longer ones cost output; eight is the session's
default.

## Harvest

- **Heads**: each listed as reversed or rewritten in "What is already decided", rewritten in place under
  `skill@knowledge-architect-decision-recording`, each measurement as its claim, its command and its
  direction, with no magnitude, as the root `CLAUDE.md` asks; every text the citing list names read and
  repaired where the rewrite leaves it false, `path@testing@tests/binary.rs` included.
- **`rejected` entries**: `rejected@query@one-search-agent-per-anchor` stays, its reason rewritten
  to what still holds; the losing alternatives above judged by the recording tests, those refuted by a
  measurement as candidates (small units in large ones' room; context entries by title; the transcript
  as the work); each new entry cited from a head.
- **Tripwires**: the two listed as reworded; the premortem's causes judged by the standard.
- **Issues**: the seven listed as closed, deleted; the re-entry point of
  `issue@agent-skills@a-lighter-pass-for-untouched-components`, the sentence on residual groups of
  `issue@agent-skills@audit-axes-beyond-the-design-record`, the first bullet of
  `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, and the
  sentences on `worklist` of `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`
  and `issue@query@no-command-lists-the-entries-two-commits-differ-by`, rewritten;
  `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`, which
  holds the parked intake thread, its citation of this spec rewritten at the spec's deletion.
- **The goal**: the rewording of `goal@agent-skills@only-needed-instructions`, ruled by the owner, written
  on this branch before it merges, with the texts that restate its clause:
  `path@agent-skills@fragments/admission-repair-shape.md`, and the heads of
  `path@agent-skills@docs/design/audit-workflow-axis.md` that argue from it.
- **CHANGELOG.md**, by `design@knowledge-architect@changelog-entries`: Workflow (the verdict report, the
  draft searched, the audit's fit), New features (`--since`, `--seeds-from`, `--work`, `--subject`), and
  the library change of `WorklistMode` and `WorklistArgs`, public types with public fields, whose class
  `design@knowledge-architect@versioning-policy` gives.

## Later consequences

- The dispatch judgement, in
  `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`.
- The intake's partitioned procedure, in
  `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`.

