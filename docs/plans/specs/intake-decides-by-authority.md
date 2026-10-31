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
- the admission standard's shared fragments state the standard and route nothing to the owner;
  the project audit keeps, in its own text, its hold on the net growth of a pass's repairs;
- a handled retrospective file is marked, and moved where the owner's instructions name a place.

The spec is written for a session that did not witness the discussion. It leaves the repository in
a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined under Names or New names, or exists in the tree.
- No default awaits the owner: `D1`, under "Defaults awaiting the owner", is ruled.
- **The work and the spec are one branch**, the branch named after `spec@plans@intake-decides-by-authority`, on the owner's word,
  round 4. The design audit of `skill@knowledge-architect-planning@working-a-slice` point 2 runs
  when the work does not start in the session where the discussion converged, or when commits
  other than this branch's have landed on the main branch since the spec was written. A session
  that finds no commit of step 1 on the branch and did not hold the discussion runs it; any
  session checks the second condition with `git fetch origin main` then
  `git log 6f75bc98..origin/main`.
- It was assembled from one transcript, found by the owner's message that opens "Over the last
  few merged branches, work has been done to shift most of the owner rulings toward goal records".
  The discussion holds 4 owner messages, called rounds 1 to 4 below, and 3 agent replies, one after
  each of rounds 1 to 3, with the headings "Round 1", "Round 2" and "Round 3"; each is called here
  "the reply to round <n>". The discussion ends at the owner's message "agreed on the table.
  Proceed, with the work carried as one spec and one branch". During the reviews of this spec, the
  owner sent three more messages: round 5, which opens "About the fragments shared between intake
  and audits"; round 6, which opens "D1: I think (a) looks like the right direction"; and round 7,
  "agreed on (a′), proceed".

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
| shape B | the name the discussion gave to `thread@intake-decides-by-authority@intake-ruling-timing`'s shape: the analysis committed at once, the work proceeding on defaults, the branch merging once every owner item is ruled |
| Q1 | the question of the reply to round 1: whether `tripwire@agent-skills@record-answered-questions-reach-the-owner` had fired, that is, whether a retrospective reported a question put to the owner that the record answered |
| H3 | the shape of `thread@intake-decides-by-authority@handled-file-archive`: mark always, move where the owner's instructions name a place, never delete |

## What the work is

What exists today, read at the main commit `6f75bc98`:

- **The intake** says, in its scope, "**This skill decides nothing.** It establishes the facts
  about each finding, proposes an action with its argument, and the owner rules." Its `%%` comment
  above the sentence reads "The sentence carries no ground on the owner's ruling that it needs
  none." Its section `skill@knowledge-architect-retrospective-intake@analysis-and-ruling` asks the
  owner to rule by writing each `Outcome` line, and, "Before committing, list every section whose
  `Outcome` line is still pending, and ask the owner for those." Its outcome "sent upstream" asks
  the owner whether the workflow file was sent; the head `design@agent-skills@installed-fix-goes-upstream`
  says "The action, an edit of the workflow file or of the issue that carries it, is the owner's".
  For a standing entry the file may meet, it says "Whether it fired, and its response, are the
  owner's to rule; the analysis proposes the judgement and its evidence". Its point 3 of
  `skill@knowledge-architect-retrospective-intake@ground-before-judging` searches again for a fix
  "before the proposal goes to the owner", and its route g of
  `skill@knowledge-architect-retrospective-intake@what-to-establish` reads "an issue only, when the
  work is not to be done in this session". Its
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
- **`design@agent-skills@audit-outcomes`** says an unneeded statement "takes the outcome of the table
  of `skill@knowledge-architect-agent-configuration@admission-standard`: … put to the owner where
  it is an instruction no agent performs; a review applies the same table, so the two judge one
  statement alike", and sends a provable gap to the owner.
- **This repository**: `design@knowledge-architect@committed-findings-analysis` already states that
  the session decides what the argument settles; this repository's intake says "Read the installed
  intake whole and follow it", so a session follows the installed "decides nothing". The root
  `CLAUDE.md` row for the analysis says "and the owner's ruling on each", and
  `path@knowledge-architect@docs/retrospective-reports/README.md` says the analysis "records the
  owner's ruling on it".

**Outside the work:**

- the rest of the audit's owner list, which sends the owner the removal of an entry, a head in the
  wrong Component, "any finding no row names", a provable gap's fill and an instruction no agent
  performs, and the primer's row for docs/audit-reports/, "the owner's ruling on each of its
  items": the audit's own work, recorded by
  `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` and
  `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`.
  The audit's behaviour does not change in this work: the fragments keep its gates, scoped to it;
- when a built default awaiting the owner enters a head,
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`;
- the real-use gate, "the owner naming what the session would have lacked without it", which is a
  fact only the owner holds and stays the owner's.

## What is already decided

**The decisions the design rests on**, not argued again:
`design@agent-skills@questions-go-to-their-authority`,
`design@agent-skills@default-ruled-before-merge`, `design@agent-skills@owner-rules-at-a-fork`,
`design@agent-skills@tripwire-recording-authority`, `design@agent-skills@criterion-ruled-at-reopening`,
`design@agent-skills@heads-stand-on-argument`, `design@agent-skills@answer-keeps-its-authority`,
`design@agent-skills@attribution-replaces-quotation`, `design@agent-skills@retrospective-destination`,
`design@agent-skills@project-file-goes-to-an-intake`, `design@agent-skills@shared-text-fragment`,
`design@agent-skills@shipped-text-cites-no-entry`,
`design@agent-skills@capabilities-not-structure`, `design@agent-skills@goals-change-through-goal-setting`,
`design@agent-skills@goal-edit-ruled-alone`, `design@knowledge-architect@workflow-findings-stay-here`,
`design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`, and
`rejected@agent-skills@delegated-decision-state`.

**The decisions the work rewrites**, with every text `cargo klarch show` lists as citing each, and
where each is judged or updated:

| decision | what changes | citing texts `cargo klarch show` lists, and who updates them |
| --- | --- | --- |
| `design@agent-skills@intake-skill-home` | "puts each finding to the owner" becomes the session deciding by authority | the retrospective skill's sentence and `%%` comment, the intake's `%%` comment: step 2; `design@agent-skills@project-file-goes-to-an-intake`: checked in step 2, no change, since it says only that the intake analyses each finding |
| `design@agent-skills@committed-intake-analysis` | committed at once; the owner's ruling only on owner items; the branch merges after them | the intake's `%%` comment: step 2; `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`, which keeps its pointer beside the plain path: no change; `design@knowledge-architect@committed-findings-analysis`: step 3 |
| `design@agent-skills@installed-fix-goes-upstream` | "The action, an edit of the workflow file or of the issue that carries it, is the owner's" narrows to a publication | the primer's paragraph on an item met outside the task and the intake: step 2, the primer checked, no change, since it names the route and not who edits; `design@agent-skills@met-outside-the-task-outcomes`, `design@agent-skills@intake-takes-the-project-file`, `design@knowledge-architect@workflow-findings-stay-here` and `issue@agent-skills@an-item-kept-for-the-retrospective-is-lost-when-no-retrospective-follows`: checked in step 2, no change, since none says who edits |
| `design@agent-skills@intake-shares-the-admission-standard` | "and the owner's ruling on every finding" leaves; "The worth of a fix and its shape do not differ" gains the one difference, the hold on growth that the audit keeps, on `argument@intake-decides-by-authority@arg-30` | the intake's `%%` comment: step 2 |
| `design@agent-skills@received-file-marked` | H3 | `a-handled-retrospective-file-is-marked-where-it-lies`, closed in step 3, since this repository's intake cites it until then; `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: no change |
| `design@agent-skills@admission-rules-home` | the fragments hold the standard; who decides an outcome is each activity's text, per `primer@question-authority` | the four fragments' `%%` lines: no change; the heads `design@agent-skills@audit-method`, the agentic-workflow axis's heads, `design@agent-skills@context-cost-of-agent-text`, `design@agent-skills@intake-shares-the-admission-standard`, the reviews' heads and `design@agent-skills@shared-text-fragment`, `issue@agent-skills@patching-an-installed-skill`, `issue@agent-skills@the-agentic-workflow-audit-s-repair-agents-read-the-admission-standard-by-a-pointer` and the two tripwires on the standard: checked in step 1, each listed by `cargo klarch show design@agent-skills@admission-rules-home` |
| `design@agent-skills@audit-net-growth` | every actor counts the growth and tells the owner the net figure; the hold is the audit's own text, on `argument@intake-decides-by-authority@arg-30` | none cites it: step 1 |
| `design@agent-skills@behaviour-additions-need-real-use` | a provable gap's fill takes the outcome its authority gives, in every activity, the audit included: the fact the gap names determines it | `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: step 3; `design@agent-skills@audit-outcomes`: step 1; the other citers, the agent-skills heads on the audit, the configuration, the record search and the retrospective, two issues and `tripwire@agent-skills@agent-text-written-without-the-standard`: checked in step 1, each kept unless it restates the gate |
| `design@agent-skills@statement-names-an-act` | an instruction no agent performs is routed by authority in every activity: deleted where the record settles that its act is no longer wanted, otherwise put to the owner as a fact only the owner holds | `design@agent-skills@audit-outcomes`: step 1; the other citers, heads of the audit and of how installed text changes, `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project` and the tripwire beside it: checked in step 1 |
| `design@agent-skills@audit-outcomes` | a provable gap leaves its owner-list cases; an instruction no agent performs stays on the owner list on the ground of a fact only the owner holds, unless the record settles it; "a review applies the same table, so the two judge one statement alike" stays true | its citers: checked in step 1; `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: step 3 |
| `design@agent-skills@record-searched-before-the-work` | "before the proposal goes to the owner" becomes "before the outcome is decided or put to the owner" | `tripwire@agent-skills@record-search-misses`, whose clause "an outcome the owner ruled in an intake" becomes "an outcome of an intake": step 2; its other citers, heads of the configuration and the reviews, two issues and the tripwire on the record search's occasions: checked in step 2 |
| `design@agent-skills@derived-decision-proceeds-on-default` | its list of deliveries gains the intake | `design@agent-skills@settled-means-recorded`'s design file and the tripwires that guard it, `tripwire@agent-skills@record-answered-questions-reach-the-owner` among them: checked in step 2, no change, since each guards the routing the list delivers |
| `design@knowledge-architect@committed-findings-analysis` | "The file is the record of the rulings" becomes the record of the outcomes and their grounds | the root `CLAUDE.md` row, the README of docs/retrospective-reports/, this repository's intake, `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` and `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: step 3 |

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
  audit, its intake items marked done, its audit items kept, and its attribution of the owner's
  ruling reworded (step 3);
  `a-handled-retrospective-file-is-marked-where-it-lies` closes (step 3);
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
- `design@agent-skills@received-file-marked`: rewritten by the work, as the table above says.
- `tripwire@agent-skills@record-search-misses`: its clause reworded in step 2, as the table above
  says.
- `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`: rested on; the
  owner's rulings land as edits of the analysis.
- `issue@agent-skills@an-item-kept-for-the-retrospective-is-lost-when-no-retrospective-follows`:
  checked in step 2, as the table above says.
- `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default`: read after
  the second design-conformance review of this spec found D1 a new instance of its subject, a
  contradiction first answered with a built default; the instance is recorded in it in step 3.

The 66 `may-bear` entries not read, each with the searcher's reason, as the search gave them:

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
  the same routing elsewhere.
- `design@agent-skills@may-bear-read-on-touch`: the analysis still lists the entries not read.
- `rejected@agent-skills@repair-check-left-to-session`: the record check stays before the decision.
- `design@agent-skills@intake-takes-the-project-file`,
  `rejected@agent-skills@retrospective-finding-straight-to-issue`,
  `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`: the analysis stays before any change.
- `design@knowledge-architect@publish-after-merge`: a publication stays the owner's.
- `design@agent-skills@goal-lifecycle`, `design@agent-skills@goal-set-review-in-goal-setting`,
  `design@agent-skills@whole-set-rounds`, `design@agent-skills@goal-batch-ruled-in-the-tree`: a goal
  stays the owner's.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the review checks the new text.
- `design@agent-skills@tripwire-admission-test`: the intake still reads every tripwire.
- `design@knowledge-architect@no-branch-sha-is-cited`,
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

### The installed text works in a project that holds none of knowledge-architect's record `##installed-text-anywhere`

Binding, from `goal@agent-skills@installed-text-works-anywhere`, reworded on this branch on the
owner's word. Met by `thread@intake-decides-by-authority@handled-file-archive`, whose H3 names no
directory, and by `thread@intake-decides-by-authority@admission-gates-in-intake`, whose shape (a′)
routes by what a consumer holds. Step 2 also brings the intake's item e, a finding's scope taken
"from the goals homes", to a consumer: a finding on the installed text has its scope in the
installed text, since the consumer holds no goal of knowledge-architect.

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

### Every actor counts the growth of its repairs and tells the owner, only a project audit holds them for the owner's word, and a gap's fill and an unperformed instruction go by authority `##admission-gates-in-intake`

Proposed by the session in the reply to round 1 as a deferral; reopened by the owner's arguments
in round 2, reshaped by the owner's argument in round 3. State: `argued`. Arguments: `argument@intake-decides-by-authority@arg-12`,
`argument@intake-decides-by-authority@arg-17`, `argument@intake-decides-by-authority@arg-18`, `argument@intake-decides-by-authority@arg-19`, `argument@intake-decides-by-authority@arg-21`, `argument@intake-decides-by-authority@arg-24`,
`argument@intake-decides-by-authority@arg-27`, `argument@intake-decides-by-authority@arg-30`, `argument@intake-decides-by-authority@arg-31`. Shape: "Decided design", "The three
gates", as `thread@intake-decides-by-authority@fragment-holds-no-routing` places the text. Harvest:
`design@agent-skills@audit-net-growth`, `design@agent-skills@behaviour-additions-need-real-use`,
`design@agent-skills@statement-names-an-act`, `design@agent-skills@intake-shares-the-admission-standard`.
The owner, round 2: "About net growth,
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
round 4. The gates leave every actor but the audit, `skill@knowledge-architect-agent-configuration`
and the agent-text review included, by the session's reading of `argument@intake-decides-by-authority@arg-30`: a session writing
text and a review judge findings bounded by the work or its diff, as an intake's are by real use,
where an audit's lenses read the whole corpus. The design-conformance review of this spec found
that the audit's gates, kept by the owner's argument of round 3, are not all among the closed list's
kinds: `D1`, ruled by the owner in rounds 6 and 7 on shape (a′), which also corrects the reading of
the third gate: an instruction no agent performs is deleted where the record settles that its act
is no longer wanted, and otherwise goes to the owner, a fact only the owner holds
(`argument@intake-decides-by-authority@arg-35`, `argument@intake-decides-by-authority@arg-36`, `argument@intake-decides-by-authority@arg-37`). The owner, round 6: "D1: I think (a)
looks like the right direction. However, one thing I want to question: have you thought about it
from the perspective of a consumer project using the intake or the audit ? They would not have the
same goals recorded. More generally, a consumer project is not expected to have much goals
pertaining to agent instructions, since this project is supposed to be the one handling most of it
and they carry only local additions related to their precise local activities." Round 7: "agreed on
(a′), proceed".

### A shared fragment states the standard and no routing to the owner, and each activity's own skill states who decides its outcomes `##fragment-holds-no-routing`

Proposed by the owner in round 5, as two shapes; the session's choice between them in this spec's
review round. State: `argued`. Arguments: `argument@intake-decides-by-authority@arg-33`, `argument@intake-decides-by-authority@arg-34`. Shape: "Decided
design", "The three gates". Harvest: `design@agent-skills@admission-rules-home`,
`design@agent-skills@audit-net-growth`. The owner, round 5: "About the fragments shared between
intake and audits: Maybe we should stop using fragments for these. The fragments should not need to
be made aware of where they are inserted, or the whole mechanic loses its purpose. Either that, or a
truly shared part is extracted, and the rest belong to its own skill". The thread supersedes the
shape the reply to round 3 gave the fragments, scoping each gate to the audit inside the fragment.

### The work carries the intake activity whole and the three fragments with their heads, and the audit's own list waits for its branch `##intake-change-scope`

Proposed by the session in the reply to round 1. State: `argued`, once the audit kept its behaviour
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
ruled on 66 causes and dropped 23, and the session now decides by the standard", the figures of
`design@agent-skills@tripwire-recording-authority`'s body;
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

### Growth at the intake is reported and recorded, not held `##arg-17`

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

### A fragment unaware of its hosts `##arg-33`

Round 5, the owner: "The fragments should not need to be made aware of where they are inserted, or
the whole mechanic loses its purpose. Either that, or a truly shared part is extracted, and the
rest belong to its own skill". Bears on `thread@intake-decides-by-authority@fragment-holds-no-routing`.

### The shared part is the standard; the routing is each activity's `##arg-34`

This spec's review round, the session: `design@agent-skills@shared-text-fragment` already says "A
text is a host only where its act applies the whole fragment"; a fragment whose owner routing
differs by host breaks that, so the routing leaves it. Stopping the fragments, the owner's first
shape, loses on that head's own argument: a pointer costs a session the decision to fetch it, and a
hand copy drifts. Bears on `thread@intake-decides-by-authority@fragment-holds-no-routing`.

### A provable gap's fill is one instruction, which the goal judges `##arg-35`

The second design-conformance review of this spec: `goal@agent-skills@only-needed-instructions`
judges each instruction, so it ranks a single fill, and only the sum of a pass's instructions is a
cost no goal ranks. Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`.

### A consumer project holds none of this project's goals `##arg-36`

Round 6, the owner: "have you thought about it from the perspective of a consumer project using the
intake or the audit ? They would not have the same goals recorded." Bears on
`thread@intake-decides-by-authority@admission-gates-in-intake`.

### In a consumer, what settles an outcome is the installed text and the consumer's record `##arg-37`

The reply to round 6, the session: the authority a consumer's session cites is
`primer@question-authority` and the admission standard, never this project's goals. The sum of a
pass's instructions is ranked by no installed rule, so the audit's hold stays; a gap's fill is
determined by the fact the gap names; and whether an instruction's act is still wanted is the intent
of whoever wrote it, which a consumer seldom records, so the reading "deleted where nothing recorded
ties it" had the default backwards. Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`.

## New names, in one place

```text
(no new type, function or command)
the Outcome line of a decided section:
  - **Outcome:** decided by the session: <outcome>. Ground: <argument, with references>
the Outcome line of an owner item, pending, then ruled:
  - **Outcome:** for the owner (<kind of the closed list>): pending
  - **Outcome:** for the owner (<kind of the closed list>): <choice>. The owner: "<words, verbatim>"
the line of the summary in the conversation:
  <ids>  <proposal in a few words>  decided: <outcome> | for you: <kind>, default <outcome>
                                                    content/skills/retrospective-intake/SKILL.md
```

## Decided design

### Who decides an outcome, and what the analysis records

The intake's scope sentence "**This skill decides nothing.**" leaves, with its `%%` comment. In
its place: the session decides each outcome as `primer@question-authority` routes it, a pointer of
one sentence, per `design@agent-skills@restatement-size-test`; an owner item is one of the closed
list's kinds. The intake names the cases it meets most, as examples of the primer's kinds, not as
a second list: a goal to set or change, which goes to `skill@knowledge-architect-goal-setting`; the
real-use gate, a fact only the owner holds; two fixes that both survive with no goal ranking them;
a publication, under `thread@intake-decides-by-authority@upstream-edit-authority`. A standing entry
the file may meet, a tripwire that may have fired, a deferred trigger, a design issue's re-entry,
is judged by the session too: whether it fired, and its response, are what the tripwire's standard
and its entry settle, per `design@agent-skills@tripwire-stands-on-the-standard`.

Each section keeps every bullet of `skill@knowledge-architect-retrospective-intake@what-to-establish`
and its `Default` line. Its `Outcome` line holds one of:

- `decided by the session: <outcome>. Ground: <the argument, with its references>`;
- `for the owner (<kind>): pending`; once the owner rules, `pending` is replaced by the owner's
  choice and words verbatim, the prefix kept. A line ending in `: pending` is the marker the check
  before the merge searches for.

An illustration of the shape, not authority:

```markdown
- **Default:** handle now: reword the sentence to the reading `design@<anchor>@<id>` states.
- **Outcome:** decided by the session: handle now. Ground: one fix stands; its rival, no change,
  leaves two readings against `goal@<anchor>@<id>`.
```

The summary in the conversation keeps one line per section, and each says who holds the outcome,
"decided" or "for you", so the owner sees every finding and its kind and can add a fact or an intent
to any line, decided ones included, before the merge. The owner writes such an addition in the
section, or gives it in the conversation, where the session copies it into the section verbatim.
The session then weighs it like any other goal, fact or argument: it decides the outcome again, and
rewrites the `Outcome` line with the new ground, or makes the section an owner item where the
addition opens a question of the closed list. The message of the commit that writes the analysis
lists each outcome the session decided, one line each, and the message of a later commit lists
each outcome it decides or changes. A section that points to a cluster analysed in another file
has neither line, as today: the outcome is given where the cluster is analysed, once. Item h of
`skill@knowledge-architect-retrospective-intake@what-to-establish` becomes "The proposal: the action,
its argument, its default outcome, and who holds it, the session or the owner, with the kind of
the closed list"; the scope's comment "the owner's ruling on each" becomes "the outcome of each";
the outcome "handle now" is handled "once decided" where it said "after the ruling".
A question tool is still not the way to collect a ruling. The proposal to rule by writing in the
file stays, for owner items.

Rival, ruled out: a batch confirmation of every default (`argument@intake-decides-by-authority@arg-5`). Rival, ruled out: today's text
with the primer prevailing (`argument@intake-decides-by-authority@arg-6`).

### Handle now, or an issue

"Handle now": the finding's work lands in the intake's branch, its design settled by the record, or
settled there through the design skill's in-change path. "Open an issue": the work needs a branch
of its own, the design skill's full path. A finding handled now that grows into a full path has its
`Outcome` rewritten, by the session, to name the plan document or the issue that carries it; the
intake's "on the owner's word" on that rewriting leaves. The session decides which. Route g of
`skill@knowledge-architect-retrospective-intake@what-to-establish`, "an issue only, when the work is
not to be done in this session", becomes "an issue only, when the work needs a branch of its own". Rival, ruled out: each choice put to the owner as a fork; a rule
ranks it, as in the review and the audit (`argument@intake-decides-by-authority@arg-9`). A row of the roadmap stays the owner's, per
`design@agent-skills@roadmap-order-is-the-owners`.

### When the analysis is committed, and when the branch merges

The analysis is committed as soon as it is written, owner items pending. The work proceeds on every
default, decided outcomes and owner items alike, except a part that rests on a goal change, a
contradiction or an action that cannot be undone, which waits for the owner's word. The owner's
rulings land as edits of the analysis, each a commit that changes a file. An owner item's outcome
is carried out only after its ruling, so the analysis cannot leave while one is pending. Before the merge of the
branch that holds the analysis, the session lists every `Outcome` line still pending and asks the
owner for those; the branch merges once none is pending. The analysis still leaves with the commit
that carries out its last outcome, or a later commit of the same branch. Rival, ruled out: today's
wait for every ruling before the commit (`argument@intake-decides-by-authority@arg-10`).

### Who edits the workflow file

The intake asks the owner whether the retrospective's workflow file was sent, and how, as today:
it is a fact only the owner holds, since the retrospective opens its issue with a free title. A
section whose fix lies in the installed text or in the checker then takes one of two outcomes:

- the file is not sent yet: `decided by the session: sent upstream`, and the session adds the
  finding to the file. The file stays on the machine and the owner reads it whole before it leaves,
  per `design@agent-skills@retrospective-destination`;
- the file is sent: `for the owner (an action that cannot be undone): pending`. The session writes
  the edit of the issue, or the new issue, and runs `gh` only on the owner's word.

Rival, ruled out: both edits the owner's (`argument@intake-decides-by-authority@arg-11`).

### The three gates

The fragments keep the shared standard, and no routing to the owner, per
`thread@intake-decides-by-authority@fragment-holds-no-routing`:

- The repair fragment: each repair counts the instructions it adds and removes, and the net figure
  of the repairs applied together is told to the owner and recorded with them. The sentence "and
  rules on that growth before any of them is applied" leaves it.
- The evidence fragment: a provable gap is admitted on reading, and its fill adds an instruction.
  "so it goes to the owner" leaves it.
- The need-test fragment, the row of an instruction no agent performs: its outcome is "a decision,
  since deleting it changes what agents are told", with no route named; the clause "put to the
  owner … on an audit's owner list, as a default awaiting the owner in a plan document, or as a
  question in the turn elsewhere" leaves it.

Who decides, by `primer@question-authority`, the same in every project, on `argument@intake-decides-by-authority@arg-37`:

- the net growth of a set of repairs: a weighing of the sum that no installed rule ranks. Only the
  audit holds its repairs for it: its agentic-workflow axis gains the sentence that the owner rules
  on that growth before any repair of the pass is applied, beside its line on the owner list's net
  growth. Every other activity tells the owner and proceeds, on `argument@intake-decides-by-authority@arg-30`;
- a provable gap's fill: the session's, since the fact the gap names determines it. The audit's
  agentic-workflow outcome table drops "a provable gap, since filling it adds an instruction" from
  its owner list, and gains it under "applied in the branch";
- an instruction no agent performs: deleted by the session where the record settles that its act is
  no longer wanted, such as a head reversed or a subject removed; otherwise put to the owner, as a
  fact only the owner holds, since whether the act is still wanted is the intent of whoever wrote
  it. The audit's owner-list row keeps the case, worded on that ground.
- `agent@knowledge-architect-workflow-auditor` drafts "the outcome the standard's table gives it" for
  an unneeded statement; its line becomes "the outcome the standard's table and the audit's
  outcome table give it".
- Each installed instruction can be followed with what a consumer holds, per
  `goal@agent-skills@installed-text-works-anywhere`: its reason is written into the installed text
  only where following it needs that reason.
- `rejected@agent-skills@unbounded-repair-growth` stays, true of the audit; the head
  `design@agent-skills@audit-net-growth` names it for the audit's hold, on `argument@intake-decides-by-authority@arg-30`.

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
- **Fragments stopped for these rules**, the owner's first shape of round 5, lost to
  `thread@intake-decides-by-authority@fragment-holds-no-routing` on `argument@intake-decides-by-authority@arg-34`.
- **Each gate scoped to the audit inside the fragment**, the shape of the reply to round 3,
  superseded by `thread@intake-decides-by-authority@fragment-holds-no-routing` on
  `argument@intake-decides-by-authority@arg-33`.
- **The audit keeping all three gates**, shape (b) of `D1`, lost to shape (a′) on
  `argument@intake-decides-by-authority@arg-35` and the owner's ruling, round 7.
- **An instruction no agent performs deleted wherever nothing recorded ties it**, the reading
  confirmed in round 4, superseded on `argument@intake-decides-by-authority@arg-37`.
- **A sent workflow file found by a search on its stem**, a repair of the first review round,
  withdrawn by the session: the retrospective opens its issue with a free title, so the search has
  nothing reliable to match.
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
| 2 | an analysis committed at once merges with an owner item still pending | `thread@intake-decides-by-authority@intake-ruling-timing` | a claim of step 2, with its check. The ground of the reply to round 3: a script can find a pending `Outcome` line in docs/retrospective-reports/ at the merge, so the failure is not silent and is no tripwire |
| 3 | intake repairs, no longer held, add rules that a later agentic-workflow audit pass finds interacting | `thread@intake-decides-by-authority@admission-gates-in-intake` | named as a cause for the harvest to judge as a tripwire. The ground of the reply to round 3: the event is named, its evidence lands in an audit report, and it is what the decision says must not happen; proposed horizon, the next two passes of that axis |
| 4 | the owner stops reading the decided lines, and the substantive interventions disappear | `criterion@intake-decides-by-authority@report-keeps-its-shape` | named as a cause; the provisional judgement drops it, since its evidence is only what the owner would have written, which the owner sees |
| 5 | a session takes a retrospective's narrative for the owner's naming of what a session lacked, and admits an addition about behaviour on it | `thread@intake-decides-by-authority@intake-outcome-authority` | named as a cause; the provisional judgement drops it, since the agent-text review axis judges the evidence of every added statement |

## Acceptance criteria

None: no cause meets conditions 1, 3 and 4 of the standard on code only the built work can check.

## Implementation sequence

### Step 1: the fragments keep the standard, and each activity states who decides

**Builds**: the edits of the three fragments of "Decided design", "The three gates"; in
`skill@knowledge-architect-project-audit@agentic-workflow-axis`, the hold on growth beside its
sentence on the owner list's net growth, that sentence's pointer to
`skill@knowledge-architect-project-audit@repair-shape` kept for the count, the provable gap moved
from the owner list to "applied in the branch" in its outcome table, and the row of an instruction
no agent performs worded on the ground of a fact only the owner holds; the line of
`agent@knowledge-architect-workflow-auditor` on an unneeded statement's outcome; the heads `design@agent-skills@audit-net-growth`,
`design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@statement-names-an-act`,
`design@agent-skills@audit-outcomes` and `design@agent-skills@admission-rules-home` rewritten in
place, each brought to the rules on heads, per `design@agent-skills@existing-heads-on-touch`;
`design@agent-skills@admission-rules-home` gains that the standard is the fragments', and who
decides an outcome is each activity's, per `primer@question-authority` and the activity's text.

**Claims**:

- No fragment of `path@agent-skills@fragments/` names an activity, or routes to the owner a growth,
  a provable gap's fill or an instruction no agent performs. Test: reading the three fragments
  whole, and a search over `path@agent-skills@fragments/` for "growth", "provable gap" and "no
  agent performs", matched across a line break, each hit read. Shown to fail on the main commit,
  where its hits in the repair, evidence and need-test fragments name the owner.
- The audit's own text states the hold on growth, applies a gap's fill in the branch, and keeps an
  unperformed instruction on its owner list on the ground of a fact only the owner holds. Test:
  reading `skill@knowledge-architect-project-audit@agentic-workflow-axis` and its outcome table.
- Each citer the table of "What is already decided" marks "checked in step 1" restates no gate the
  step removes, or is reworded. Test: reading each, listed in the step's commit message with its
  outcome.
- The fragments' host lists are unchanged. Test: `each_fragment_has_its_hosts` of
  `path@agent-skills@src/lib.rs` passes unchanged.
- `cargo x gates` passes.

**Fails alone on**: a reader of the audit who finds no hold on growth, or a fragment that names an
activity or the owner for one of the three outcomes.

### Step 2: the intake decides by authority, and its analysis keeps its shape

**Builds**: the intake's text, every passage the "Decided design" subsections change, the slugs of
its sections kept: its scope paragraph, with "the owner's ruling on each", and the sentence "This
skill decides nothing" with its `%%` comment; its description; point 3 of
`skill@knowledge-architect-retrospective-intake@ground-before-judging`; route g and item h of
`skill@knowledge-architect-retrospective-intake@what-to-establish`;
`skill@knowledge-architect-retrospective-intake@analysis-and-ruling`, its heading's words, its
sentence on a standing entry, its sentence on a section that points to a cluster, its outcomes
"handle now" and "sent upstream", and its ruling paragraph; and
`skill@knowledge-architect-retrospective-intake@after-the-ruling`; with their `%%` comments. Also:
the installed retrospective's sentence on the intake; the primer's row for the analysis, which
becomes "the analysis of a retrospective's project file, the outcome of each finding and the
owner's ruling on each owner item"; the heads `design@agent-skills@intake-skill-home`,
`design@agent-skills@committed-intake-analysis`, `design@agent-skills@installed-fix-goes-upstream`,
`design@agent-skills@intake-shares-the-admission-standard`, `design@agent-skills@received-file-marked`,
`design@agent-skills@record-searched-before-the-work` and
`design@agent-skills@derived-decision-proceeds-on-default` rewritten in place; and the clause of
`tripwire@agent-skills@record-search-misses`.

**Claims**:

- No installed text says the intake decides nothing or puts each finding to the owner. Test: a
  search for "This skill decides nothing" in the intake, and for "puts each finding to the owner"
  and "owner's ruling on each of its findings" over `path@agent-skills@content/`, matched across a
  line break. Shown to fail on the main commit. The phrase "decides nothing" alone also matches
  the primer, the audit, the design skill and the retrospective, which say it of other things.
- The intake asks, before the merge of the branch that holds an analysis, for every `Outcome` line
  still pending, and the branch merges only once none is: premortem cause 2. The intake names the
  check, `grep -rn -E 'Outcome.*for the owner \(.*\): pending' docs/retrospective-reports/`.
  Test: reading `skill@knowledge-architect-retrospective-intake@after-the-ruling`, and running that
  command over two files written in a scratch directory, one holding a pending line in the form of
  "New names" and one holding only ruled and decided lines: one hit, then none.
- The analysis keeps one section per finding with its bullets, its `Default` and its `Outcome`
  lines, and the summary's line per section says who holds the outcome. Test: reading
  `skill@knowledge-architect-retrospective-intake@analysis-and-ruling`.
- The installed text names no directory for handled files. Test: reading
  `skill@knowledge-architect-retrospective-intake@after-the-ruling`.
- Each citer the table of "What is already decided" marks "checked in step 2" is read, and listed
  in the step's commit message with its outcome.
- `cargo x gates` passes, and `cargo klarch install-agent-skills` writes the new text into
  `.claude/`.

**Fails alone on**: a session following the intake that is still sent to the owner for a finding
the record settles, or that merges with an owner item pending.

### Step 3: this repository's intake and records follow, and the changelog says what changed

**Builds**: `design@knowledge-architect@committed-findings-analysis` rewritten in place; this
repository's intake, its section on what becomes of received files pointing to the installed rule
on a handled file, and its citation of the closing issue removed; the closure of
`a-handled-retrospective-file-is-marked-where-it-lies`; the root `CLAUDE.md`
row for the analysis; the README of `path@knowledge-architect@docs/retrospective-reports/`; the
rewording of `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: its intake items,
its provable-gap items and its knowledge-table row for the analysis removed from its list, since
they are fixed, its other audit items kept, and its attribution of the owner's ruling brought to the
owner's intent of round 2, attributed and not quoted, per `primer@design-heads`; the instance of
`D1` recorded in `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default`; the
premise of `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` reworded; a
CHANGELOG.md entry under `Next release`, Workflow, surface agent-skills, patch, then
`cargo x changelog`.

**Claims**:

- No text of this repository says the analysis records the owner's ruling on every finding. Test:
  reading the root `CLAUDE.md` row, `path@knowledge-architect@docs/retrospective-reports/README.md`,
  `design@knowledge-architect@committed-findings-analysis` and this repository's intake; a search
  for "the owner's ruling on each" over them is a lead only, since the new primer row keeps "the
  owner's ruling on each owner item".
- `cargo x gates` passes.

**Fails alone on**: a reader of this repository's records who finds the old ruling on every
finding.

## Order rationale

Step 1 before step 2: the intake hosts the three fragments, and its text written after them
describes a standard that is already in place. Step 2 before step 3: this repository's texts follow
the installed intake, and restate nothing of it; the issue it cites closes once its citation is
removed, in step 3.

## Defaults awaiting the owner

### D1: the audit's gates, and the closed list — ruled

Bears on `thread@intake-decides-by-authority@admission-gates-in-intake`. The design-conformance review of this spec found
that the audit keeps sending to the owner a growth of instructions, a provable gap's fill and an
instruction no agent performs, while `goal@knowledge-architect@the-owner-decides` sends the owner
"these and nothing else", and the owner's word of round 3 kept the gates in the audit: a decision
in conflict with a goal goes to the owner, per `primer@goals-bind`. The second reading found the
first default wrong on a fill, `argument@intake-decides-by-authority@arg-35`, and the part resting on it was held.

Shape (a′), on `argument@intake-decides-by-authority@arg-37`: the audit keeps only the hold on net growth, a weighing of the sum;
a gap's fill and an instruction no agent performs go by authority in every activity, the latter
reaching the owner unless the record settles it. The owner, round 6: "D1: I think (a) looks like
the right direction." Round 7: "agreed on (a′), proceed". Shape (a′) is the session's, approved by
the owner.

## Harvest

| what | where | when |
| --- | --- | --- |
| the decisions of the threads | rewritten in place in the heads listed in "What is already decided", in the steps that land them, per `design@agent-skills@design-home-is-built-intent` | steps 1 to 3 |
| the losing alternatives, `thread@intake-decides-by-authority@fragment-holds-no-routing`'s first shape included | judged by the tests of `skill@knowledge-architect-decision-recording@losing-alternatives`; each that earns one, a `rejected` entry of agent-skills cited by its head | the last commit of step 3 |
| premortem causes 1, 3, 4, 5 | judged by the tripwire standard, as `skill@knowledge-architect-decision-recording@ruled-tripwires` says, on their wording then | the last commit of step 3 |
| the issues | `a-handled-retrospective-file-is-marked-where-it-lies` closed; `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` and `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` reworded | step 3 |

## Later consequences

- The audit's own owner list, the removal of an entry, a head in the wrong Component and "any finding
  no case names", is brought to the closed list in the audit's own work, which
  `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list` keeps.
- When a built default awaiting the owner enters a head stays
  `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`.
