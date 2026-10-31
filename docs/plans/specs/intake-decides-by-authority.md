# The retrospective intake decides by authority, and the owner rules only on the closed list

## Status and audience

This spec is the plan of the work that changes who decides the outcome of a finding at a
retrospective intake. Today the installed intake, `skill@knowledge-architect-retrospective-intake`,
states "This skill decides nothing", asks the owner's ruling on every finding, and commits its
analysis only once every `Outcome` line holds the owner's words. After the work:

- the session decides every outcome that the goals, the record, the code or a source settles, and
  the owner rules only on what `goal@knowledge-architect@the-owner-decides` gives the owner;
- the analysis keeps its shape, one section per finding with its evidence and its `Default` and
  `Outcome` lines, and each line of the summary in the conversation says who holds the outcome;
- the analysis is committed as soon as it is written, the work proceeds on every default, and the
  branch merges once the owner has ruled every item that is the owner's;
- the three owner gates of the admission standard's shared fragments leave them, and the project
  audit keeps them in its own text;
- a handled retrospective file is marked, and moved where the owner's instructions name a place.

The spec is written for a session that did not witness the discussion. It leaves the repository in
a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined under Names or New names, or exists in the tree.
- No default awaits the owner: the section "Defaults awaiting the owner" is empty. Where one is
  added later, `skill@knowledge-architect-planning@working-a-slice` says what waits.
- **The work and the spec are one branch**, the branch named after `spec@plans@intake-decides-by-authority`, on the owner's word,
  round 4. The work starts in the session where the discussion converged, so the design audit of
  `skill@knowledge-architect-planning@working-a-slice` point 2 does not run unless other commits
  land on the main branch before the work starts.
- It was assembled from one transcript, found by the owner's message that opens "Over the last
  few merged branches, work has been done to shift most of the owner rulings toward goal records".
  The discussion holds 4 owner messages, called rounds 1 to 4 below, and 3 agent replies, one after
  each of rounds 1 to 3, with the headings "Round 1", "Round 2" and "Round 3"; each is called here
  "the reply to round <n>". The discussion ends at the owner's message "agreed on the table.
  Proceed, with the work carried as one spec and one branch".

## How the work is done

`skill@knowledge-architect-planning`, its section on the work of a spec,
`skill@knowledge-architect-planning@working-a-slice`. Every edit of the installed text follows
`path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", read whole, and
`skill@knowledge-architect-agent-configuration`.

## Names

| shorthand | what it names |
| --- | --- |
| the intake | `skill@knowledge-architect-retrospective-intake`, source `path@agent-skills@content/skills/retrospective-intake/SKILL.md` |
| this repository's intake | `skill@klarch-retrospective-intake` |
| the analysis | one file of docs/retrospective-reports/ at a project's root, written by the intake for one received file |
| the closed list | the five kinds `goal@knowledge-architect@the-owner-decides` sends to the owner: a fork, a contradiction no argument resolves, a goal to set or change, a fact only the owner holds, an action that cannot be undone |
| an owner item | a section of the analysis whose outcome the closed list gives the owner |
| the three gates | the three sentences of the admission standard's fragments that send to the owner a growth of instructions, a provable gap's fill, and an instruction whose act no agent performs |
| the repair fragment | `path@agent-skills@fragments/admission-repair-shape.md` |
| the evidence fragment | `path@agent-skills@fragments/admission-evidence.md` |
| the need-test fragment | `path@agent-skills@fragments/admission-need-test.md` |
| the audit | `skill@knowledge-architect-project-audit`, source `path@agent-skills@content/skills/project-audit/SKILL.md` |
| H3 | the shape of `thread@intake-decides-by-authority@handled-file-archive`: mark always, move where the owner's instructions name a place, never delete |

## What the work is

What exists today, read at the main commit `6f75bc98`:

- **The intake** says, in its scope, "**This skill decides nothing.** It establishes the facts
  about each finding, proposes an action with its argument, and the owner rules." Its `%%` comment
  above the sentence reads "The sentence carries no ground on the owner's ruling that it needs
  none." Its section `skill@knowledge-architect-retrospective-intake@analysis-and-ruling` asks the
  owner to rule by writing each `Outcome` line, and, "Before committing, list every section whose
  `Outcome` line is still pending, and ask the owner for those." Its outcome "sent upstream" asks
  the owner whether the workflow file was sent, and treats every edit of it as the owner's. Its
  section `skill@knowledge-architect-retrospective-intake@after-the-ruling` says "Each section's
  `Outcome` line holds the owner's choice and the owner's words, verbatim. The analysis is
  committed … once every `Outcome` line holds the owner's ruling, and before any outcome is carried
  out", then marks each received file "a line at its head", leaving it where it lies.
- **The intake's description** says "offering the owner an outcome per finding, … and the life of
  the analysis after the ruling", and its item h names "the default outcome".
- **The installed retrospective**, `path@agent-skills@content/skills/retrospective/SKILL.md`, says
  the intake "puts each finding to the owner with its evidence and a default outcome".
- **The primer's knowledge table**, `path@agent-skills@content/PRIMER.md`, has the row "the
  analysis of a retrospective's project file, and the owner's ruling on each of its findings".
- **The three gates**:
  - the repair fragment: "When the repairs applied together add more instructions than they
    remove, the owner is told so first, with the net figure, and rules on that growth before any
    of them is applied." Its hosts: the intake, `skill@knowledge-architect-agent-configuration`,
    `agent@knowledge-architect-agent-text-reviewer`, the audit and
    `skill@knowledge-architect-retrospective`;
  - the evidence fragment: a provable gap's "fill adds an instruction, so it goes to the owner".
    Its hosts: the intake, `skill@knowledge-architect-agent-configuration` and
    `agent@knowledge-architect-agent-text-reviewer`;
  - the need-test fragment, the row "it is an instruction whose act no agent performs": "put to
    the owner, since deleting it changes what agents are told: on an audit's owner list, as a
    default awaiting the owner in a plan document, or as a question in the turn elsewhere". Its
    hosts: the intake, `skill@knowledge-architect-agent-configuration`,
    `agent@knowledge-architect-agent-text-reviewer` and `agent@knowledge-architect-workflow-auditor`.
- **The audit's own text** lists a provable gap and an instruction no agent performs on its owner
  list, in the outcome table of its agentic-workflow axis. On growth it says only "The owner list
  states the net growth of the pass's repairs in the project first, where they add more
  instructions than they remove, as `skill@knowledge-architect-project-audit@repair-shape` says":
  the hold itself is the fragment's sentence.
- **This repository**: `design@knowledge-architect@committed-findings-analysis` already states that
  the session decides what the argument settles; this repository's intake says "Read the installed
  intake whole and follow it", so a session follows the installed "decides nothing". The root
  `CLAUDE.md` row for the analysis says "and the owner's ruling on each", and
  `path@knowledge-architect@docs/retrospective-reports/README.md` says the analysis "records the
  owner's ruling on it".

**Outside the work:**

- the rest of the audit's owner list, which sends the owner the removal of an entry, a head in the
  wrong Component and "any finding no case names": the audit's own work, recorded by
  `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` and
  `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`.
  The audit stays consistent after this work, since it keeps its own gates;
- when a built default awaiting the owner enters a head,
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`;
- the real-use gate, "the owner naming what the session would have lacked without it", which is a
  fact only the owner holds and stays the owner's.

## What is already decided

**The decisions the design rests on**, not argued again:
`design@agent-skills@questions-go-to-their-authority`,
`design@agent-skills@derived-decision-proceeds-on-default`,
`design@agent-skills@default-ruled-before-merge`, `design@agent-skills@owner-rules-at-a-fork`,
`design@agent-skills@tripwire-recording-authority`, `design@agent-skills@criterion-ruled-at-reopening`,
`design@agent-skills@heads-stand-on-argument`, `design@agent-skills@answer-keeps-its-authority`,
`design@agent-skills@attribution-replaces-quotation`, `design@agent-skills@retrospective-destination`,
`design@agent-skills@project-file-goes-to-an-intake`, `design@agent-skills@shared-text-fragment`,
`design@agent-skills@admission-rules-home`, `design@agent-skills@shipped-text-cites-no-entry`,
`design@agent-skills@capabilities-not-structure`, `design@agent-skills@goals-change-through-goal-setting`,
`design@agent-skills@goal-edit-ruled-alone`, `design@knowledge-architect@workflow-findings-stay-here`,
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`, and
`rejected@agent-skills@delegated-decision-state`.

**The decisions the work rewrites**, with every text `cargo klarch show` lists as citing each, and
where each is judged or updated:

| decision | what changes | citing texts, and who updates them |
| --- | --- | --- |
| `design@agent-skills@intake-skill-home` | "puts each finding to the owner" becomes the session deciding by authority | the intake's `%%` comment and the retrospective skill: step 2 |
| `design@agent-skills@committed-intake-analysis` | committed at once; the owner's ruling only on owner items; the branch merges after them | the intake's `%%` comment, `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`, which keeps its pointer beside the plain path, and `design@knowledge-architect@committed-findings-analysis`: steps 2 and 3 |
| `design@agent-skills@installed-fix-goes-upstream` | "The action, an edit of the workflow file or of the issue that carries it, is the owner's" narrows to a publication | the primer, the intake, `issue@agent-skills@an-item-kept-for-the-retrospective-is-lost-when-no-retrospective-follows`, which needs no edit: step 2 |
| `design@agent-skills@intake-shares-the-admission-standard` | "and the owner's ruling on every finding" leaves | the intake's `%%` comment: step 2 |
| `design@agent-skills@received-file-marked` | H3 | `issue@agent-skills@a-handled-retrospective-file-is-marked-where-it-lies`, closed: step 2 |
| `design@agent-skills@audit-net-growth` | growth counted and reported by every actor; the hold stays the audit's | the audit's design file: step 1 |
| `design@agent-skills@behaviour-additions-need-real-use` | a provable gap's fill goes to the owner in an audit only | its citing heads and issues need no edit, since none restates the owner gate: step 1 checks each |
| `design@agent-skills@statement-names-an-act` | an instruction no agent performs goes to the owner in an audit; elsewhere it is routed by authority | its citing heads: step 1 checks each |
| `design@agent-skills@record-searched-before-the-work` | "before the proposal goes to the owner" becomes "before the outcome is decided or put to the owner" | step 2 |
| `design@agent-skills@derived-decision-proceeds-on-default` | its list of deliveries gains the intake | step 2 |
| `design@knowledge-architect@committed-findings-analysis` | "The file is the record of the rulings" becomes the record of the outcomes and their grounds | the root `CLAUDE.md` row, the README of docs/retrospective-reports/, this repository's intake: step 3 |

**The entries the search graded**, 80 `bears` and 73 `may-bear`, `cargo klarch worklist --verify`
passing. Each `bears` entry and its outcome:

- Goals, all criteria of this spec: `goal@knowledge-architect@the-owner-decides`,
  `goal@knowledge-architect@design-with-agents-as-counterparts`,
  `goal@knowledge-architect@owner-attention-on-what-is-theirs`,
  `goal@knowledge-architect@agents-get-a-complete-workflow`,
  `goal@knowledge-architect@agents-work-without-drift`,
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`,
  `goal@knowledge-architect@the-owner-s-word-is-verified`,
  `goal@knowledge-architect@the-workflow-improves-through-real-use`,
  `goal@knowledge-architect@any-project-can-adopt-it`, `goal@agent-skills@no-contradiction-in-workflow`,
  `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@only-needed-instructions`,
  `goal@agent-skills@installed-text-works-anywhere`,
  `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`.
- The decisions listed above, rested on or rewritten.
- `design@agent-skills@knowledge-table-home`: the primer's row changes in step 2, and the root
  `CLAUDE.md` row, which refines it, in step 3.
- `design@agent-skills@goals-bind`, `design@agent-skills@owner-word-holds-as-far-as-its-premise`,
  `design@agent-skills@met-outside-the-task-outcomes`, `design@agent-skills@decisions-reported-to-the-owner`,
  `design@agent-skills@fork-choice-in-weak-form`, `design@agent-skills@unargued-approval-is-argued`,
  `design@agent-skills@intent-through-goals-not-overrule`, `design@agent-skills@transcript-reviewer-agent`,
  `design@agent-skills@ruled-items-labelled`, `design@agent-skills@new-or-reshaped-head-needs-design`,
  `design@agent-skills@one-decision-per-head`, `design@agent-skills@existing-heads-on-touch`,
  `design@agent-skills@instruction-record-is-minimal`, `design@agent-skills@synthetic-evidence-not-built`,
  `design@agent-skills@expectation-set-bounds-scope`, `design@agent-skills@review-judges-additions`,
  `design@agent-skills@restatement-size-test`, `design@agent-skills@overlay-by-separate-skills`,
  `design@agent-skills@shipped-text-line-comments`, `design@agent-skills@tripwire-stands-on-the-standard`,
  `design@agent-skills@committed-audit-report`, `design@agent-skills@repairs-read-against-the-record`:
  each constrains the text the steps write, and none is changed. The record check stays before
  every decision of an outcome, so `design@agent-skills@repairs-read-against-the-record` holds; its
  wording "before the owner rules on it" covers the intake only where the owner rules, and the
  intake's text states the check before the session decides.
- `design@agent-skills@settled-means-recorded`: judged, no change. Its exception "an outcome the
  owner ruled in an audit, an intake or a review" serves a change the owner asks for; an outcome the
  session decides is no request of the owner, so the design skill's trigger does not fire on it.
- `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@changelog-entries`,
  `design@knowledge-architect@the-branch-writes-its-changelog-entries`: step 3 writes a Workflow
  entry, surface agent-skills, class patch, since a question put to the owner changes.
- `rejected@agent-skills@unbounded-repair-growth`: the intake's growth stops being held; the
  rejected entry stays true of the audit, whose hold stays, per `thread@intake-decides-by-authority@admission-gates-in-intake`.
- `rejected@agent-skills@owner-rules-each-tripwire`: not brought back; a finding on a tripwire is
  the session's under `design@agent-skills@tripwire-stands-on-the-standard`.
- Issues: `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` stays open for the
  audit, its intake items and its three gates marked done, and its attribution of the owner's
  ruling reworded (step 3);
  `issue@agent-skills@a-handled-retrospective-file-is-marked-where-it-lies` closes (step 2);
  `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` stays, its premise that
  an analysis is committed before any outcome reworded in step 3;
  `issue@agent-skills@patching-an-installed-skill` stays: this repository's intake still sets aside
  the same two instructions;
  `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`
  stays, outside the work;
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`
  stays, outside the work;
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` stays: the
  intake's search still reads every tripwire against a received file;
  `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`
  stays: the analysis still keeps the owner's words verbatim on owner items;
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` stays.
- `design@agent-skills@shared-text-fragment`: no host is added to a fragment, so no host list goes
  to the owner.

The `may-bear` entries read, with their outcome:

- `tripwire@agent-skills@record-answered-questions-reach-the-owner`: not fired, on the owner's
  fact, round 2, "Q1 there was no retrospective about this".
- `design@agent-skills@criterion-ruled-at-reopening`: a precedent of the shape; no change.
- `tripwire@agent-skills@expectation-set-closes-a-contradiction`: its exposure widens, since the
  session now closes such a finding; no change, its re-entry still holds.

The 70 `may-bear` entries not read, each with the searcher's reason, as the search gave them:

- `design@agent-skills@primer-limit`, `design@agent-skills@verification-in-primer`,
  `design@agent-skills@code-change-reads-its-record`, `design@agent-skills@context-cost-of-agent-text`,
  `design@agent-skills@installed-skill-takes-precedence`, `design@agent-skills@adopting-existing-docs`,
  `design@agent-skills@staged-check-before-each-commit`,
  `design@agent-skills@staged-forms-for-a-partial-commit`: each cites a seed; none is changed by a
  drafted shape.
- `rejected@agent-skills@directive-restated-everywhere`: the closed list restated in each text;
  the work points to `primer@question-authority` instead.
- `rejected@agent-skills@finding-reported-not-recorded`: the analysis records every decision.
- `rejected@agent-skills@owner-reading-answer-put-back`: the work stops putting such questions back.
- `issue@agent-skills@a-fix-is-grounded-by-no-record-search`,
  `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`,
  `issue@agent-skills@the-material-finding-duty-has-no-head`: no change established.
- `tripwire@agent-skills@departure-overridden-as-a-reading`,
  `tripwire@agent-skills@fix-proposed-past-an-unread-head`,
  `tripwire@agent-skills@open-fork-classed-as-settled`,
  `tripwire@agent-skills@piped-chain-runs-under-pipefail`,
  `tripwire@agent-skills@pipefail-status-misread`, `tripwire@agent-skills@axis-not-run-at-a-pin-move`,
  `tripwire@core@checker-source-literals-are-data-whole-source-scope`,
  `tripwire@core@init-cleanup-beyond-the-listed-files`: each re-enters at an intake or counts
  analyses; an analysis per received file stays.
- `tripwire@agent-skills@record-search-misses`: its clause counts "an outcome the owner ruled in an
  intake"; under the work most outcomes are the session's.
- `goal@knowledge-architect@structure-and-workflow-work-together`,
  `goal@knowledge-architect@records-live-beside-the-code`: the analysis keeps its home.
- `design@agent-skills@audit-method`, `design@agent-skills@audit-passes-repeat-while-the-count-falls`,
  `design@agent-skills@one-run-pass`, `design@agent-skills@apply-stays-with-the-session`,
  `design@agent-skills@audit-triggers`, `design@agent-skills@design-record-axis`,
  `design@agent-skills@tripwire-audit-axis`, `design@agent-skills@workflow-axis-finding-standard`,
  `design@agent-skills@workflow-auditor-agent`, `design@agent-skills@root-cause-repair`:
  the audit's model; the audit is unchanged in behaviour.
- `issue@agent-skills@an-upstream-finding-stays-in-every-later-old-text-count`,
  `issue@agent-skills@audit-axes-beyond-the-design-record`,
  `issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s`,
  `rejected@agent-skills@wrong-component-outcome-removed`: the audit's own entries.
- `design@agent-skills@title-states-the-rule`, `design@agent-skills@extension-follows-the-ground`,
  `design@agent-skills@outcome-over-display`, `rejected@agent-skills@design-load-unconditioned`,
  `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`,
  `tripwire@agent-skills@owner-proposal-built-ungrounded`,
  `issue@agent-skills@a-reviewer-s-running-time-is-unbounded`,
  `issue@agent-skills@expectation-sets-for-the-installed-skills`,
  `tripwire@agent-skills@agent-text-written-without-the-standard`: the session now decides where
  the owner did; no change established.
- `design@agent-skills@roadmap-order-is-the-owners`: an outcome that orders work stays the owner's.
- `rejected@agent-skills@owner-rules-each-criterion`, `design@agent-skills@divergence-routed-by-its-authority`,
  `rejected@agent-skills@every-thread-closed-by-argument`,
  `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default`: the same
  routing elsewhere.
- `design@agent-skills@may-bear-read-on-touch`: the analysis still lists the entries not read.
- `rejected@agent-skills@repair-check-left-to-session`: the record check stays before the decision.
- `design@agent-skills@intake-takes-the-project-file`,
  `rejected@agent-skills@retrospective-finding-straight-to-issue`,
  `issue@agent-skills@an-item-kept-for-the-retrospective-is-lost-when-no-retrospective-follows`,
  `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`: the analysis stays before any change.
- `design@knowledge-architect@publish-after-merge`: a publication stays the owner's.
- `design@agent-skills@goal-lifecycle`, `design@agent-skills@goal-set-review-in-goal-setting`,
  `design@agent-skills@whole-set-rounds`, `design@agent-skills@goal-batch-ruled-in-the-tree`: a goal
  stays the owner's.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the review checks the new text.
- `design@agent-skills@tripwire-admission-test`: the intake still reads every tripwire.
- `design@knowledge-architect@no-branch-sha-is-cited`,
  `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`,
  `issue@agent-skills@the-audit-assumes-every-project-merges-by-rebase`: the rulings land by an edit
  of the analysis, a commit that changes a file.

The design-conformance review reads each entry not read whole.

## Criteria

### The owner rules only on the closed list `##owner-rules-the-closed-list`

Binding, from `goal@knowledge-architect@the-owner-decides`: "A question put to the owner for a
ruling that the goals, the record, the code and its measurements, or the sources answered … shows
the goal unmet." Met by `thread@intake-decides-by-authority@intake-outcome-authority`, `thread@intake-decides-by-authority@admission-gates-in-intake` and
`thread@intake-decides-by-authority@upstream-edit-authority`.

### No two instructions a session cannot both obey `##no-contradiction`

Binding, from `goal@agent-skills@no-contradiction-in-workflow`. Today "This skill decides nothing"
stands against `primer@question-authority`. Met by `thread@intake-decides-by-authority@intake-outcome-authority`, and by
`thread@intake-decides-by-authority@admission-gates-in-intake`, under which the audit's hold and the fragment's report can both be
obeyed.

### What the argument leaves open reaches the owner before the main branch `##before-the-merge`

Binding, from `goal@knowledge-architect@design-with-agents-as-counterparts`. Met by
`thread@intake-decides-by-authority@intake-ruling-timing`.

### Structure the work does not need is removed `##no-needed-structure`

Binding, from `goal@agent-skills@only-needed-instructions`: a ruling owed on every finding is a
fixed mapping. Met by `thread@intake-decides-by-authority@intake-outcome-authority` and `thread@intake-decides-by-authority@intake-ruling-timing`.

### The installed text names nothing of one project `##installed-text-anywhere`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met by `thread@intake-decides-by-authority@handled-file-archive`,
whose H3 names no directory.

### Nothing leaves the machine before the owner has read it `##nothing-leaves-unread`

Binding, from `design@agent-skills@retrospective-destination`. Met by `thread@intake-decides-by-authority@upstream-edit-authority`.

### The session decides by authority, never by a grant `##no-grant`

Binding, from `rejected@agent-skills@delegated-decision-state`. Met by `thread@intake-decides-by-authority@intake-outcome-authority`.

### The analysis keeps its shape, so the owner sees every finding and its kind `##report-keeps-its-shape`

Binding, stated by the owner in round 2: "the reports keep the shape as right now, so that I can
get informed easily about the list and kind of findings that have been reported, and I can complete
with informations or intent I hold". Met by `thread@intake-decides-by-authority@intake-outcome-authority`.

### The owner's substantive interventions keep a moment `##interventions-keep-a-moment`

Weighed, from the measurement of `argument@intake-decides-by-authority@arg-3`. Met: the decided lines are in the committed analysis,
readable before the merge, per `thread@intake-decides-by-authority@intake-ruling-timing`.

## Threads

### The session decides each outcome the goals, the record, the code or a source settles, and the owner rules on the closed list `##intake-outcome-authority`

Proposed by the session in the reply to round 1, from the owner's report in round 1. State:
`argued`, one shape standing; its rivals ruled out by `argument@intake-decides-by-authority@arg-5` and `argument@intake-decides-by-authority@arg-6`. Arguments:
`argument@intake-decides-by-authority@arg-1`, `argument@intake-decides-by-authority@arg-3`, `argument@intake-decides-by-authority@arg-4`, `argument@intake-decides-by-authority@arg-7`, `argument@intake-decides-by-authority@arg-8`, `argument@intake-decides-by-authority@arg-15`, `argument@intake-decides-by-authority@arg-25`. Shape: "Decided
design", its first subsection. Harvest: `design@agent-skills@intake-skill-home` and
`design@agent-skills@committed-intake-analysis`. The owner, round 2: "intake-outcome-authority
agreed on that shape. The part that is important I believe, is that the reports keep the shape as
right now, so that I can get informed easily about the list and kind of findings that have been
reported, and I can complete with informations or intent I hold." An approval, recorded as the
session's decision, with the owner's added criterion `criterion@intake-decides-by-authority@report-keeps-its-shape`.

### "Handle now" when the finding's work lands in the intake's branch, "an issue" when it needs a branch of its own `##intake-now-or-issue`

Proposed by the session in the reply to round 1; reshaped in the reply to round 2 on the owner's
context. State: `argued`. Arguments: `argument@intake-decides-by-authority@arg-9`, `argument@intake-decides-by-authority@arg-16`, `argument@intake-decides-by-authority@arg-26`. Shape: "Decided design".
Harvest: the intake's text; `design@agent-skills@intake-skill-home`. The owner, round 2:
"intake-now-or-issue: this is probably the correct shape. One detail of context: an intake happens
on its own branch. The "fits on this branch" condition might end up feeling weird due to that. But
I think that should work ."

### The analysis is committed at once, the work proceeds on defaults, and the branch merges once every owner item is ruled `##intake-ruling-timing`

Proposed by the session in the reply to round 1. State: `argued`. Argument: `argument@intake-decides-by-authority@arg-10`. Shape:
"Decided design". Harvest: `design@agent-skills@committed-intake-analysis`. The owner, round 2:
"intake-ruling-timing: agreed on shape B".

### Adding a finding to an unsent workflow file is the session's; editing or opening an issue is the owner's `##upstream-edit-authority`

Proposed by the session in the reply to round 1. State: `argued`. Argument: `argument@intake-decides-by-authority@arg-11`. Shape:
"Decided design". Harvest: `design@agent-skills@installed-fix-goes-upstream`. The owner, round 2:
"upstream-edit-authority agreed".

### The three gates leave the shared fragments; growth is counted and reported; the audit keeps its gates in its own text `##admission-gates-in-intake`

Proposed by the session in the reply to round 1 as a deferral; reopened by the owner's arguments
in round 2, reshaped by the owner's argument in round 3. State: `argued`. Arguments: `argument@intake-decides-by-authority@arg-12`,
`argument@intake-decides-by-authority@arg-17`, `argument@intake-decides-by-authority@arg-18`, `argument@intake-decides-by-authority@arg-19`, `argument@intake-decides-by-authority@arg-21`, `argument@intake-decides-by-authority@arg-24`, `argument@intake-decides-by-authority@arg-27`, `argument@intake-decides-by-authority@arg-30`, `argument@intake-decides-by-authority@arg-31`.
Shape: "Decided design". Harvest: `design@agent-skills@audit-net-growth`,
`design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@statement-names-an-act`,
`design@agent-skills@intake-shares-the-admission-standard`. The owner, round 2: "About net growth,
I think that only informing me of it and recording it is fine. It should not hold the change. This
has to be handled with our design I believe, or what we are changing will simply not be able to
function. The provable gap fill does not need for my word either. The other rules, from
intake-outcome-authority, are supposed to be sufficient to determine outcomes and authority. I do
not think those admission gates should add to them." Round 3: "about the net growth rule and the
admission gates: I think that a difference must be applied between how retro intake is handled,
compared to the agent text audit. The agent text audit has little bounds of what findings it can
reports, while the retrospective is bounded by what sessions actually end up doing, and backed by
real use. I don't think the retro intake can lead to unbounded growth the same way the audit can."
The routing of "an instruction no agent performs" is the session's reading of round 2, shown to
the owner as `presumed-settled` at the checkpoint and confirmed by the owner's word on the table,
round 4.

### The work carries the intake activity whole, with the three gates in every host, and the audit's own list waits for its branch `##intake-change-scope`

Proposed by the session in the reply to round 1. State: `argued`, once the audit kept its gates
and no fork remained (`argument@intake-decides-by-authority@arg-32`). Arguments: `argument@intake-decides-by-authority@arg-14`, `argument@intake-decides-by-authority@arg-20`, `argument@intake-decides-by-authority@arg-23`, `argument@intake-decides-by-authority@arg-28`.
Shape: the implementation sequence. Harvest: none; the scope leaves with the spec. The owner, round
2: "About my ruling "each text … in a change of its own", I do not remember ruling that. I think
those words overstate my intent. I simply wanted to get those in line separately from the session
that opened the issue, not necessarily one separate change per text. To make sense, I think we need
to apply these change to "one whole activity" at once. Or an activity would end up being in a
halfway updated state and probably malfunction." Round 4: "agreed on the table. Proceed, with the
work carried as one spec and one branch".

### A handled file is marked always, moved where the owner's instructions name a place, and never deleted `##handled-file-archive`

Proposed by the session in the reply to round 2, as H3. State: `argued`; H1 and H2 ruled out.
Arguments: `argument@intake-decides-by-authority@arg-13`, `argument@intake-decides-by-authority@arg-29`. Shape: "Decided design". Harvest:
`design@agent-skills@received-file-marked`. The owner, round 2: "handled-file-archive agreed to
discuss here"; round 3: "handled-file-archive agreed on H3".

## Arguments

### The intake asks the owner's ruling on items the record settles `##arg-1`

Round 1, the owner: "an agent reported to me that the intake skill asks for my rulings on every
item, even when the record and the docs would allow an agent side decision of the action to take".
Bears on `thread@intake-decides-by-authority@intake-outcome-authority`.

### The issue exists under a title naming no skill `##arg-2`

The reply to round 1, the session: "It is `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`,
opened on the goal-standard branch." Bears on `thread@intake-decides-by-authority@intake-outcome-authority`.

### About 5 rulings in 6 at past intakes approved the default `##arg-3`

The reply to round 1, the session. Every analysis deleted from docs/retrospective-reports/, 60
files, read with `git show <commit>^:<path>` after
`git log --diff-filter=D --name-only -- docs/retrospective-reports/`, its `Outcome` lines extracted
with `grep -i -E '^\s*(- )?\**Outcome\**:?'`: 368 lines, 305 beginning with an approval word, 63
others, about 25 of them bringing an argument, a fact, an intent or a new shape. The grep is a
heuristic, so the figures are approximate. "about 5 rulings in 6 recorded nothing the analysis did
not already hold." Bears on `thread@intake-decides-by-authority@intake-outcome-authority` and `criterion@intake-decides-by-authority@interventions-keep-a-moment`.

### The rule every other activity follows `##arg-4`

The reply to round 1, the session: "Replace "This skill decides nothing" and the ruling owed on
every finding with the rule every other activity already follows." Bears on
`thread@intake-decides-by-authority@intake-outcome-authority`.

### A batch confirmation fails the goal and is a grant `##arg-5`

The reply to round 1, the session: "It is ruled out by the binding goal … A batch word is also an
approval, which `rejected@agent-skills@delegated-decision-state` refuses as a ground." Rules out
shape C of `thread@intake-decides-by-authority@intake-outcome-authority`.

### Today's text leaves two instructions a session cannot both obey `##arg-6`

The reply to round 1, the session: "It is ruled out by `goal@agent-skills@no-contradiction-in-workflow`.
The agent that reported to you followed the skill, not the primer, which is the measured failure of
this rival." Bears on `thread@intake-decides-by-authority@intake-outcome-authority`.

### Precedents of the same routing `##arg-7`

The reply to round 1, the session: `design@agent-skills@tripwire-recording-authority`, "you had
ruled on 66 causes and dropped 23, and the session now decides by the standard";
`design@agent-skills@criterion-ruled-at-reopening`; `design@agent-skills@derived-decision-proceeds-on-default`.
Bears on `thread@intake-decides-by-authority@intake-outcome-authority`.

### The real-use gate is a fact only the owner holds `##arg-8`

The reply to round 1, the session: an added instruction about behaviour needs "the owner naming
what the session would have lacked without it"; the retrospective records no such statement, a
search of the installed retrospective for "would have lacked" finding nothing. Bears on
`thread@intake-decides-by-authority@intake-outcome-authority`.

### A rule ranks "now or later", as in the review and the audit `##arg-9`

The reply to round 1, the session: "the session decides it by the rule that the review, the audit
and `primer@met-outside-the-task` already use"; "It loses because a rule ranks it, and the review
already applies that rule without you." Bears on `thread@intake-decides-by-authority@intake-now-or-issue`.

### The wait interrupts the work and makes a default no default `##arg-10`

The reply to round 1, the session: "This is `design@agent-skills@default-ruled-before-merge`, which
is your timing ("the work goes on with no interruption"), applied to the intake." Bears on
`thread@intake-decides-by-authority@intake-ruling-timing`.

### A local edit can be undone; a publication cannot `##arg-11`

The reply to round 1, the session: "Adding a finding to a workflow file not yet sent is a local,
reversible edit, so the session decides it."; "An edit of a GitHub issue, or a new issue, is a
publication, so it stays yours." Bears on `thread@intake-decides-by-authority@upstream-edit-authority`.

### The growth gate holds the work against the primer's timing `##arg-12`

The reply to round 1, the session: "The growth gate also contradicts the primer's timing: it holds
the work, where a fork goes to you with a default the work proceeds on." Bears on
`thread@intake-decides-by-authority@admission-gates-in-intake`.

### The re-entry of the handled-file issue is met `##arg-13`

The reply to round 1, the session: the issue gives its re-entry as "the next change to the intake's
section on the life of an analysis", which `thread@intake-decides-by-authority@intake-ruling-timing` rewrites. Bears on
`thread@intake-decides-by-authority@handled-file-archive`.

### One branch per group of texts describing one activity `##arg-14`

The reply to round 1, the session: "My reading of "each text … in a change of its own" is one
branch per group of texts that describe one activity." Bears on `thread@intake-decides-by-authority@intake-change-scope`.

### The report keeps its shape `##arg-15`

Round 2, the owner: "the reports keep the shape as right now, so that I can get informed easily
about the list and kind of findings that have been reported, and I can complete with informations
or intent I hold". Bears on `thread@intake-decides-by-authority@intake-outcome-authority`.

### An intake runs on its own branch `##arg-16`

Round 2, the owner: "One detail of context: an intake happens on its own branch. The "fits on this
branch" condition might end up feeling weird due to that." Bears on `thread@intake-decides-by-authority@intake-now-or-issue`.

### Growth is reported and recorded, never held `##arg-17`

Round 2, the owner: "only informing me of it and recording it is fine. It should not hold the
change. This has to be handled with our design I believe, or what we are changing will simply not
be able to function." Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`.

### A provable gap's fill needs no word of the owner `##arg-18`

Round 2, the owner: "The provable gap fill does not need for my word either." Bears on
`thread@intake-decides-by-authority@admission-gates-in-intake`.

### The admission gates add nothing to the authority rules `##arg-19`

Round 2, the owner: "The other rules, from intake-outcome-authority, are supposed to be sufficient
to determine outcomes and authority. I do not think those admission gates should add to them."
Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`.

### One whole activity at once, or it malfunctions `##arg-20`

Round 2, the owner, correcting the attribution of "each text … in a change of its own": "I simply
wanted to get those in line separately from the session that opened the issue, not necessarily one
separate change per text. To make sense, I think we need to apply these change to "one whole
activity" at once." Bears on `thread@intake-decides-by-authority@intake-change-scope` and `thread@intake-decides-by-authority@admission-gates-in-intake`.

### The gates are in the intake skill `##arg-21`

Round 2, the owner: "But admission-gates-in-intake is about the intake skill, right ? You proposed
to defer it if I understood correctly". Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`.

### No retrospective reported the case `##arg-22`

Round 2, the owner: "Q1 there was no retrospective about this". A fact:
`tripwire@agent-skills@record-answered-questions-reach-the-owner` has not fired.

### The issue overstates the owner's ruling `##arg-23`

The reply to round 2, the session: "the issue's sentence records a stricter ruling than you gave.
This branch rewords it to your intent". Bears on `thread@intake-decides-by-authority@intake-change-scope`.

### The record cannot tell which remedy stopped the growth `##arg-24`

The reply to round 2, the session, a material finding on `rejected@agent-skills@unbounded-repair-growth`:
the repair by cause ("adds an instruction only where none") and the hold ("rules on that growth")
both first appear on 2026-10-10, and `design@agent-skills@review-judges-additions` on 2026-10-13,
each found with `git log -S'<phrase>' --reverse`. Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`; closed by
`argument@intake-decides-by-authority@arg-31`.

### Each line of the summary says who holds the outcome `##arg-25`

The reply to round 2, the session: "The one change is that each line in the conversation states who
holds the outcome"; "You see every finding and its kind, and you can add a fact or an intent to any
line, decided ones included, before the merge." Bears on `thread@intake-decides-by-authority@intake-outcome-authority`.

### "Fits the branch" is the wrong test on an intake's own branch `##arg-26`

The reply to round 2, the session: "**handle now:** the finding's work lands in the intake's
branch, with its design settled, or settled there through the design skill's in-change path";
"**issue:** it needs a branch of its own". Bears on `thread@intake-decides-by-authority@intake-now-or-issue`.

### The deferral would leave the intake half updated `##arg-27`

The reply to round 2, the session: "Your argument and your correction of the ruling make that
deferral wrong: it would leave the intake half updated."; "The gates are shared text, so changing
them reaches every host at once". Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`.

### The rest of the audit is no contradiction `##arg-28`

The reply to round 2, the session: "**Rival:** the whole audit activity in this branch too. It
loses on size, since the audit's own list touches three axes. It is not a contradiction, so your
rule does not demand it." Bears on `thread@intake-decides-by-authority@intake-change-scope`.

### H2 invents a path; H3 asks no question `##arg-29`

The reply to round 2, the session: H2 loses since "`goal@knowledge-architect@any-project-can-adopt-it`
admits no path particular to one setup"; "Your instruction already exists at user level, so H3
adds no question at any intake"; H1 loses "on the issue's evidence that a marked file left among
pending ones can be handed to an intake again". Bears on `thread@intake-decides-by-authority@handled-file-archive`.

### Intake findings are bounded by real use; audit findings are not `##arg-30`

Round 3, the owner: "The agent text audit has little bounds of what findings it can reports, while
the retrospective is bounded by what sessions actually end up doing, and backed by real use. I
don't think the retro intake can lead to unbounded growth the same way the audit can." Bears on
`thread@intake-decides-by-authority@admission-gates-in-intake`.

### The rejected entry's evidence is the audit's `##arg-31`

The reply to round 3, the session: "`rejected@agent-skills@unbounded-repair-growth` has its
evidence in audit passes 1 to 3, so that evidence does not carry over to the intake."; "the
rejected entry stays true of the audit and says nothing about the intake." Bears on
`thread@intake-decides-by-authority@admission-gates-in-intake`.

### The audit keeps its behaviour, so no fork remains in the scope `##arg-32`

The reply to round 3, the session: "the audit activity is not touched in behaviour, so
`thread@intake-decides-by-authority@intake-change-scope` has no fork left." Bears on `thread@intake-decides-by-authority@intake-change-scope`.

## New names, in one place

```text
(no new type, function or command)
the line of the summary in the conversation: "decided: <outcome> (<ground>)" or "for you: <kind of the closed list>, default <outcome>"   content/skills/retrospective-intake/SKILL.md
```

## Decided design

### Who decides an outcome, and what the analysis records

The intake's scope sentence "**This skill decides nothing.**" leaves, with its `%%` comment. In
its place: the session decides each outcome as `primer@question-authority` routes it, a pointer of
one sentence, per `design@agent-skills@restatement-size-test`; an owner item is one of the closed
list's kinds. The intake names the cases it meets most, as examples of the primer's kinds, not as
a second list: a goal to set or change, which goes to `skill@knowledge-architect-goal-setting`; the
real-use gate, a fact only the owner holds; two fixes that both survive with no goal ranking them;
a publication, under `thread@intake-decides-by-authority@upstream-edit-authority`.

Each section keeps every bullet of `skill@knowledge-architect-retrospective-intake@what-to-establish`
and its `Default` line. Its `Outcome` line holds one of:

- `decided by the session: <outcome>. Ground: <the argument, with its references>`;
- `for the owner (<kind>)`, pending, then the owner's choice and words verbatim once given.

An illustration of the shape, not authority:

```markdown
- **Default:** handle now: reword the sentence to the reading `design@<anchor>@<id>` states.
- **Outcome:** decided by the session: handle now. Ground: one fix stands; its rival, no change,
  leaves two readings against `goal@<anchor>@<id>`.
```

The summary in the conversation keeps one line per section, and each says who holds the outcome,
"decided" or "for you", so the owner sees every finding and its kind and can add a fact or an intent
to any line, decided ones included, before the merge. A question tool is still not the way to
collect a ruling. The proposal to rule by writing in the file stays, for owner items.

Rival, ruled out: a batch confirmation of every default (`argument@intake-decides-by-authority@arg-5`). Rival, ruled out: today's text
with the primer prevailing (`argument@intake-decides-by-authority@arg-6`).

### Handle now, or an issue

"Handle now": the finding's work lands in the intake's branch, its design settled by the record, or
settled there through the design skill's in-change path. "Open an issue": the work needs a branch
of its own, the design skill's full path. A finding handled now that grows into a full path has its
`Outcome` rewritten to name the plan document or the issue that carries it, as the intake already
says. The session decides which. Rival, ruled out: each choice put to the owner as a fork; a rule
ranks it, as in the review and the audit (`argument@intake-decides-by-authority@arg-9`). A row of the roadmap stays the owner's, per
`design@agent-skills@roadmap-order-is-the-owners`.

### When the analysis is committed, and when the branch merges

The analysis is committed as soon as it is written, owner items pending. The work proceeds on every
default, decided outcomes and owner items alike, except a part that rests on a goal change, a
contradiction or an action that cannot be undone, which waits for the owner's word. The owner's
rulings land as edits of the analysis, each a commit that changes a file. Before the merge of the
branch that holds the analysis, the session lists every `Outcome` line still pending and asks the
owner for those; the branch merges once none is pending. The analysis still leaves with the commit
that carries out its last outcome, or a later commit of the same branch. Rival, ruled out: today's
wait for every ruling before the commit (`argument@intake-decides-by-authority@arg-10`).

### Who edits the workflow file

Adding a finding to the retrospective's workflow file not yet sent is the session's: the file stays
on the machine and the owner reads it whole before it leaves, per
`design@agent-skills@retrospective-destination`. Editing the issue that carries it, posting a
comment, or opening a new issue is a publication, and the owner's. The intake no longer asks the
owner whether the file was sent where the session can tell: an issue on the workflow's repository
found with `gh` shows it was. Rival, ruled out: both edits the owner's (`argument@intake-decides-by-authority@arg-11`).

### The three gates

- The repair fragment: each repair counts the instructions it adds and removes; where the
  repairs applied together add more than they remove, the net figure is told to the owner and
  recorded, in the analysis, the report or the commit message that applies them. It holds no
  repair.
- The evidence fragment: a provable gap is admitted on reading; its fill takes the outcome its
  authority gives, per `primer@question-authority`.
- The need-test fragment, the row of an instruction no agent performs: routed by
  `primer@question-authority`: deleted where nothing recorded ties it to a goal or to a ruling of
  the owner; otherwise a contradiction or a fork, which goes to the owner.
- The audit's own text gains the hold the fragment loses: in the agentic-workflow axis, the owner
  rules on the net growth of a pass's repairs before any of them is applied. Its owner list keeps a
  provable gap and an instruction no agent performs, as it already states. The argument is the
  owner's, `argument@intake-decides-by-authority@arg-30`: an audit's lenses read the whole corpus and report findings with little
  bound, where an intake's findings come from what real sessions did.
- `rejected@agent-skills@unbounded-repair-growth` stays, true of the audit; the head
  `design@agent-skills@audit-net-growth` names it for the audit's hold.

### What becomes of a handled file

The intake marks every received project file it carried out, as today. It then moves the file
where the owner's instructions name a place for handled files, and leaves it where it lies where
none does. It never deletes one. This repository's intake points to the same rule for every file it
marks. Rivals, ruled out: H1, the mark alone (`argument@intake-decides-by-authority@arg-29`); H2, a fixed archive directory beside the
pending one, which invents a path no installed text may name.

## Mapping tables

None: the work maps no domain onto another.

## Losing alternatives

- **A batch confirmation of every default**, shape C, lost to `thread@intake-decides-by-authority@intake-outcome-authority` on
  `argument@intake-decides-by-authority@arg-5`. Covered by `rejected@agent-skills@delegated-decision-state`.
- **Today's text, the primer prevailing in silence**, lost to `thread@intake-decides-by-authority@intake-outcome-authority` on
  `argument@intake-decides-by-authority@arg-6`.
- **Each "now or later" choice put to the owner as a fork**, lost to `thread@intake-decides-by-authority@intake-now-or-issue` on
  `argument@intake-decides-by-authority@arg-9`.
- **The analysis committed only once every ruling is in it**, lost to `thread@intake-decides-by-authority@intake-ruling-timing` on
  `argument@intake-decides-by-authority@arg-10`.
- **Every edit of the workflow file the owner's**, lost to `thread@intake-decides-by-authority@upstream-edit-authority` on `argument@intake-decides-by-authority@arg-11`.
- **The three gates deferred to their own change**, withdrawn by the session on the owner's
  arguments, `argument@intake-decides-by-authority@arg-19`, `argument@intake-decides-by-authority@arg-20` and `argument@intake-decides-by-authority@arg-27`.
- **The three gates removed from the audit as well**, lost to `thread@intake-decides-by-authority@admission-gates-in-intake` on
  `argument@intake-decides-by-authority@arg-30`.
- **The whole audit activity in this branch**, lost to `thread@intake-decides-by-authority@intake-change-scope` on `argument@intake-decides-by-authority@arg-28` and
  `argument@intake-decides-by-authority@arg-32`.
- **H1, the mark alone**, and **H2, a fixed archive directory**, lost to `thread@intake-decides-by-authority@handled-file-archive` on
  `argument@intake-decides-by-authority@arg-29`.

Which of these earns a `rejected` entry is judged at the harvest, by
`skill@knowledge-architect-decision-recording`.

## Readings

None: the work reads no external specification.

## Premortem

| # | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| 1 | a session classes a fork as settled and decides it, so a choice that was the owner's never reaches the owner | `thread@intake-decides-by-authority@intake-outcome-authority` | named as a cause for the harvest to judge; `tripwire@agent-skills@open-fork-classed-as-settled` already guards that premise, and the harvest folds it there or drops it |
| 2 | an analysis committed at once merges with an owner item still pending | `thread@intake-decides-by-authority@intake-ruling-timing` | a claim of step 2, with its check |
| 3 | intake repairs, no longer held, add rules that a later agentic-workflow audit pass finds interacting | `thread@intake-decides-by-authority@admission-gates-in-intake` | named as a cause for the harvest to judge as a tripwire, with a short horizon |
| 4 | the owner stops reading the decided lines, and the substantive interventions disappear | `criterion@intake-decides-by-authority@report-keeps-its-shape` | named as a cause; the provisional judgement drops it, since its evidence is only what the owner would have written, which the owner sees |
| 5 | a session takes a retrospective's narrative for the owner's naming of what a session lacked, and admits an addition about behaviour on it | `thread@intake-decides-by-authority@intake-outcome-authority` | named as a cause; the provisional judgement drops it, since the agent-text review axis judges the evidence of every added statement |

## Acceptance criteria

None: no cause meets conditions 1, 3 and 4 of the standard on code only the built work can check.

## Implementation sequence

### Step 1: the three gates leave the shared fragments, and the audit keeps them in its own text

**Builds**: the edits of the three fragments of "Decided design", "The three gates"; the hold on
growth written into the audit's agentic-workflow axis, beside its sentence on the owner list's net
growth; the heads `design@agent-skills@audit-net-growth`,
`design@agent-skills@behaviour-additions-need-real-use` and `design@agent-skills@statement-names-an-act`
rewritten in place, each brought to the rules on heads, per
`design@agent-skills@existing-heads-on-touch`.

**Claims**:

- No shared fragment sends a growth, a provable gap's fill or an unperformed instruction to the
  owner. Test: reading each of the three fragments whole; a search for "rules on that growth",
  "goes to the owner" and "put to the owner" over `path@agent-skills@fragments/` returns no gate.
  Shown to fail on the main commit, where it returns the three sentences.
- The audit's text states the hold on growth and the two owner-list cases on its own. Test: reading
  the agentic-workflow axis's section of the audit and its outcome table.
- The fragments' host lists are unchanged. Test: `each_fragment_has_its_hosts` of
  `path@agent-skills@src/lib.rs` passes unchanged.
- `cargo x gates` passes.

**Fails alone on**: a reader of the audit who finds no hold on growth, or a host of a fragment that
still sends a gate to the owner.

### Step 2: the intake decides by authority, and its analysis keeps its shape

**Builds**: the intake's text, every section the "Decided design" subsections on who decides, now
or later, timing, the workflow file and a handled file change: its scope, its description, its item
h, `skill@knowledge-architect-retrospective-intake@analysis-and-ruling` and
`skill@knowledge-architect-retrospective-intake@after-the-ruling`, with their `%%` comments; the
installed retrospective's sentence on the intake; the primer's row for the analysis, which becomes
"the analysis of a retrospective's project file, the outcome of each finding and the owner's ruling
on each owner item"; the heads `design@agent-skills@intake-skill-home`,
`design@agent-skills@committed-intake-analysis`, `design@agent-skills@installed-fix-goes-upstream`,
`design@agent-skills@intake-shares-the-admission-standard`, `design@agent-skills@received-file-marked`,
`design@agent-skills@record-searched-before-the-work` and
`design@agent-skills@derived-decision-proceeds-on-default` rewritten in place; and the closure of
`issue@agent-skills@a-handled-retrospective-file-is-marked-where-it-lies`.

**Claims**:

- No installed text says the intake decides nothing or puts each finding to the owner. Test: a
  search for "decides nothing" and "puts each finding to the owner" over
  `path@agent-skills@content/`, matched across a line break. Shown to fail on the main commit.
- The intake asks, before the merge of the branch that holds an analysis, for every `Outcome` line
  still pending, and the branch merges only once none is. This is the check of premortem cause 2.
  Test: reading `skill@knowledge-architect-retrospective-intake@after-the-ruling`.
- The analysis keeps one section per finding with its bullets, its `Default` and its `Outcome`
  lines, and the summary's line per section says who holds the outcome. Test: reading
  `skill@knowledge-architect-retrospective-intake@analysis-and-ruling`.
- The installed text names no directory for handled files. Test: reading
  `skill@knowledge-architect-retrospective-intake@after-the-ruling`.
- `cargo x gates` passes, and `cargo klarch install-agent-skills` writes the new text into
  `.claude/`.

**Fails alone on**: a session following the intake that is still sent to the owner for a finding
the record settles, or that merges with an owner item pending.

### Step 3: this repository's intake and records follow, and the changelog says what changed

**Builds**: `design@knowledge-architect@committed-findings-analysis` rewritten in place; this
repository's intake, its section on what becomes of received files pointing to the installed rule
on a handled file; the root `CLAUDE.md` row for the analysis; the README of
`path@knowledge-architect@docs/retrospective-reports/`; the rewording of
`issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`, its intake items and its three
gates marked done and its attribution of the owner's ruling brought to the owner's words of round
2; the premise of `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`
reworded; a CHANGELOG.md entry under `Next release`, Workflow, surface agent-skills, patch, then
`cargo x changelog`.

**Claims**:

- No text of this repository says the analysis records the owner's ruling on every finding. Test: a
  search for "owner's ruling on each" and "records the owner's ruling" over the root `CLAUDE.md`,
  `path@knowledge-architect@docs/retrospective-reports/README.md`, `register@knowledge-architect@design`
  and this repository's intake.
- `cargo x gates` passes.

**Fails alone on**: a reader of this repository's records who finds the old ruling on every
finding.

## Order rationale

Step 1 before step 2: the intake hosts the three fragments, and its text written after them
describes a standard that is already in place. Step 2 before step 3: this repository's texts follow
the installed intake, and restate nothing of it.

## Defaults awaiting the owner

None.

## Harvest

| what | where | when |
| --- | --- | --- |
| the decisions of the threads | rewritten in place in the heads listed in "What is already decided", in the steps that land them, per `design@agent-skills@design-home-is-built-intent` | steps 1 to 3 |
| the losing alternatives | judged by the tests of `skill@knowledge-architect-decision-recording@losing-alternatives`; each that earns one, a `rejected` entry of agent-skills cited by its head | the last commit of step 3 |
| premortem causes 1, 3, 4, 5 | judged by the tripwire standard, as `skill@knowledge-architect-decision-recording@ruled-tripwires` says, on their wording then | the last commit of step 3 |
| the issues | `issue@agent-skills@a-handled-retrospective-file-is-marked-where-it-lies` closed; `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` and `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` reworded | steps 2 and 3 |

## Later consequences

- The audit's own owner list, the removal of an entry, a head in the wrong Component and "any finding
  no case names", is brought to the closed list in the audit's own work, which
  `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` keeps.
- When a built default awaiting the owner enters a head stays
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`.
